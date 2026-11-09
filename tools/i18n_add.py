#!/usr/bin/env python3
"""Add messages to every interface language at once.

Reads JSON lines on stdin, one message per line:

    {"id": "radio-foo", "en": "Foo", "fr": "Foo", "es": "Foo", "de": "Foo", "nl": "Foo", "it": "Foo", "pt-BR": "Foo", "ru": "Foo", "tr": "Foo", "ja": "Foo", "zh-CN": "Foo",
     "note": "optional comment for translators"}

and appends it to crates/sdroxide-ui/i18n/{en-US,fr,es,de,nl,it,pt-BR,ru,tr,ja,zh-CN}/sdroxide.ftl under a
lock, so several people (or agents) can add messages at the same time. An id
that already exists is reported and skipped. `{ $name }` placeables are kept as
they are; a value with a newline is written as a Fluent multi-line value.

    tools/i18n_add.py --check ID...   prints the English text of existing ids
"""
import fcntl, json, os, re, sys

ROOT = os.path.join(os.path.dirname(os.path.abspath(__file__)), "..", "crates", "sdroxide-ui", "i18n")
LANGS = {"en": "en-US", "fr": "fr", "es": "es", "de": "de", "nl": "nl", "it": "it", "pt-BR": "pt-BR", "ru": "ru", "tr": "tr", "ja": "ja", "zh-CN": "zh-CN"}


def fmt(id_, text, note=None):
    out = []
    if note:
        out += ["# " + l for l in note.split("\n")]
    lines = text.split("\n")
    first = lines[0]
    # A value may not start with a character Fluent reads as syntax.
    if first[:1] in "[*.}" or first != first.lstrip():
        first = "{ \"" + first[: len(first) - len(first.lstrip())] + "\" }" + first.lstrip() if first != first.lstrip() else "{ \"" + first[0] + "\" }" + first[1:]
    if len(lines) == 1:
        out.append(f"{id_} = {first}")
    else:
        out.append(f"{id_} =")
        out.append("    " + first)
        out += ["    " + l for l in lines[1:]]
    return "\n".join(out) + "\n"


def existing(lang):
    p = os.path.join(ROOT, LANGS[lang], "sdroxide.ftl")
    return set(re.findall(r"^([a-z0-9-]+) =", open(p, encoding="utf8").read(), re.M))


def main():
    msgs = [json.loads(l) for l in sys.stdin if l.strip()]
    lock = open("/tmp/sdroxide-i18n-add.lock", "w")
    fcntl.flock(lock, fcntl.LOCK_EX)
    have = existing("en")
    added = skipped = 0
    for m in msgs:
        if m["id"] in have:
            print("exists:", m["id"])
            skipped += 1
            continue
        if not re.fullmatch(r"[a-z][a-z0-9-]*", m["id"]):
            print("bad id:", m["id"])
            continue
        for lang in LANGS:
            text = m.get(lang) or (m["en"] if lang != "en" else None)
            if text is None:
                raise SystemExit("missing en for " + m["id"])
            with open(os.path.join(ROOT, LANGS[lang], "sdroxide.ftl"), "a", encoding="utf8") as f:
                f.write("\n" + fmt(m["id"], text, m.get("note") if lang == "en" else None))
        have.add(m["id"])
        added += 1
    print(f"added {added}, skipped {skipped}")


if __name__ == "__main__":
    if len(sys.argv) > 1 and sys.argv[1] == "--check":
        p = open(os.path.join(ROOT, "en-US", "sdroxide.ftl"), encoding="utf8").read()
        for i in sys.argv[2:]:
            m = re.search(r"^%s =(.*(?:\n    .*)*)" % re.escape(i), p, re.M)
            print(i, "=", m.group(1).strip() if m else "(missing)")
    else:
        main()

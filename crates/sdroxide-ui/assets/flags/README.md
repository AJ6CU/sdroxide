# Packaged country flags

The PNGs here are built by `tools/gen_flags.py` from the public-domain flag
collection at <https://github.com/fonttools/region-flags> (branch `gh-pages`,
`png/`), whose images were taken from Wikimedia Commons and checked to be in
the public domain or otherwise exempt from copyright. See that project's
`COPYING` for the per-flag provenance.

They are scaled to fit 48×32 pixels and reduced to a 64-colour palette, which
is invisible at the dozen points a decode row draws them at and keeps the whole
set — every DXCC entity, 265 distinct flags — small enough to compile into the
browser client as well as the native binary.

Which entity flies which flag is decided in
`crates/sdroxide-types/src/entity_flags.rs`; this directory only holds the
images that table names. Do not add or edit files here by hand — change that
table and re-run the generator:

    ./tools/gen_flags.py --src /path/to/region-flags/png

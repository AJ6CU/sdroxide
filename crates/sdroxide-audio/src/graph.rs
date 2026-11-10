//! The PipeWire side of the device list.
//!
//! **Why this exists.** sdroxide asks ALSA for its sound devices, and ALSA
//! cannot see a PipeWire graph. It exposes a *bridge* — a `pipewire` and a
//! `pulse` PCM — and nothing behind it. So a Bluetooth microphone, a virtual
//! sink, a loopback or anything else a listener has arranged in a patchbay **can
//! never appear in the settings list by name**, however long anyone looks, while
//! the patchbay shows it plainly. Both observations are correct; they are
//! different floors of the same building.
//!
//! The Bluetooth microphone is what forced the question (`BT-MIKE-HANDOVER.md`),
//! but nothing here is about Bluetooth: this is a *device list* that can see the
//! sound server the operator actually uses.
//!
//! **How a graph node is opened.** A PipeWire node is not a device you open and
//! close. It is reachable two ways, and both are tried, in order:
//!
//! 1. **By name through the bridge** — `pulse:<node>`, which the ALSA `pulse`
//!    plugin resolves. This is what makes a node *pickable*: the operator chooses
//!    `bluez_input.…` and sdroxide opens that node, rather than the node being
//!    made the system default for everything so that one program can reach it.
//! 2. **The graph itself**, through `wpctl`, when the bridge will not take it.
//!
//! Only names are listed here. Deciding what to *do* with a node — which the
//! engine opens, in what order — is the caller's, and the same fallback story
//! runs there for a node as it does for a card.

use std::process::Command;

/// One node from the sound server's own list.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct GraphNode {
    /// The name the sound server knows it by, e.g. `bluez_input.54:7A:52:00:3B:17`.
    pub name: String,
    /// Its description, when it offered one — `Headset`, `Stereo Output`.
    pub description: String,
}

/// `pactl list short sources|sinks`, parsed.
///
/// Lines are `<index>\t<name>\t<server>\t<format>\t<state>` and the format
/// carries spaces (`s16le 2ch 44100Hz`), so only the first two columns are taken
/// and the rest ignored. A line without a tab is not a node and is skipped
/// rather than guessed at.
pub fn parse_pactl_short(out: &str) -> Vec<GraphNode> {
    out.lines()
        .filter_map(|line| {
            let mut cols = line.split('\t');
            let _index = cols.next()?;
            let name = cols.next()?.trim();
            if name.is_empty() {
                return None;
            }
            Some(GraphNode { name: name.to_string(), description: String::new() })
        })
        .collect()
}

/// The nodes the sound server offers for one direction.
///
/// `output` picks sinks, otherwise sources. An unavailable `pactl` — a machine
/// on plain ALSA, a container, a path without it — is **not** an error: the list
/// is an addition to what ALSA already gave, and a machine that has nothing to
/// add should simply get nothing.
pub fn graph_nodes(output: bool) -> Vec<GraphNode> {
    let what = if output { "sinks" } else { "sources" };
    let Ok(text) = Command::new("pactl").args(["list", "short", what]).output() else {
        return Vec::new();
    };
    if !text.status.success() {
        return Vec::new();
    }
    parse_pactl_short(&String::from_utf8_lossy(&text.stdout))
}

/// The node names to offer beside the ALSA cards.
///
/// **Only the ones ALSA cannot already show.** A WirePlumber source named
/// `alsa_input.pci-0000_06_00.6.analog-stereo` is the same sound card the ALSA
/// host already listed, and offering it again would put one device in the picker
/// twice under two names — which is worse than not offering it, because the
/// operator cannot tell which is which.
pub fn extra_device_names(output: bool) -> Vec<String> {
    // **Cached, and it is not an optimisation.** This is called from a settings
    // panel that redraws every frame, and each call used to spawn a short-lived
    // `pactl` process — a PipeWire client that connects and is destroyed again.
    // WirePlumber aborts an ALSA node's activation when a client vanishes
    // mid-activation (`s-monitors: Failed to create ALSA node …: Object
    // activation aborted: PipeWire proxy destroyed`), so the panel was, tens of
    // times a second, taking a sound card out of the graph for good: the sink
    // disappeared from the machine's sound settings and only a PipeWire restart
    // brought it back. Reported as "it happened since we messed with pipewire
    // for the bluetooth mike", and it was.
    //
    // A few seconds of staleness costs nothing here — the list is of things that
    // change when a device is plugged in or a profile attached, and the caller
    // has a Rescan for exactly those moments.
    const TTL: std::time::Duration = std::time::Duration::from_secs(5);
    let mut cache = EXTRA_CACHE.lock().unwrap_or_else(|e| e.into_inner());
    if let Some((at, dir, names)) = cache.as_ref()
        && *dir == output
        && at.elapsed() < TTL
    {
        return names.clone();
    }
    let names: Vec<String> = graph_nodes(output)
        .into_iter()
        .filter(|n| !n.name.starts_with("alsa_"))
        .map(|n| n.name)
        .collect();
    *cache = Some((std::time::Instant::now(), output, names.clone()));
    names
}

/// Last answer per direction, with when it was taken. See [`extra_device_names`].
static EXTRA_CACHE: std::sync::Mutex<
    Option<(std::time::Instant, bool, Vec<String>)>,
> = std::sync::Mutex::new(None);

/// Whether `name` is a sound-server graph node rather than an ALSA card.
pub fn is_graph_node_name(name: &str) -> bool {
    name.starts_with("bluez_input.")
        || name.starts_with("bluez_output.")
        || name.starts_with("alsa_output.")
        || name.starts_with("alsa_input.")
        || name.contains(".monitor")
        || name.starts_with("libpulse_")
}

/// **Why a capture node has to be made the default.** This was measured, not
/// assumed, and the finding inverts the obvious design.
///
/// The obvious design is to open the node by name — `pulse:bluez_input.…`, which
/// ALSA's `pulse` plugin will accept, and which **was verified to capture the
/// Midland on this machine** (3 s of audio at 48 kHz mono float). So the
/// mechanism works. What does not work is getting there: **cpal can only open a
/// device the host enumerated or the host's default**, so a node the sound
/// server owns cannot be handed to it by name, and PipeWire has no way for one
/// application to say "capture from this source" — playback has a per-stream
/// target, capture does not.
///
/// Which is exactly why the operator's working route was `wpctl set-default`
/// followed by picking the bridge. So the honest implementation is not a
/// smarter open: it is that command, run for the operator, with the
/// consequence stated.
///
/// **The consequence is the whole design.** Making a CB microphone the default
/// source moves the *desktop's* capture too — every other program records from
/// it until it is put back. So it is never done silently, never done on the
/// program startup, and always says which device it took over and how to undo
/// it.
pub fn set_default(node: &str) -> Result<(), String> {
    let out = Command::new("wpctl").args(["set-default", node]).output().map_err(|e| {
        format!("wpctl is not available on this system ({e}); make {node} the default source with your sound server instead")
    })?;
    if !out.status.success() {
        return Err(format!(
            "the sound server refused {node:?}: {}",
            String::from_utf8_lossy(&out.stderr).trim()
        ));
    }
    Ok(())
}

/// The node that was the default before [`set_default`] changed it, which is the
/// only way back.
///
/// `pactl`, not `wpctl`: this `wpctl` has no `get-default` and prints its help
/// instead, so the undo line came out empty every time — a hint that is missing
/// exactly when it is most wanted. `pactl get-default-source` answers on the
/// version here, and if it ever stops answering this is `None` rather than a
/// wrong name.
pub fn current_default(output: bool) -> Option<String> {
    let what = if output { "sink" } else { "source" };
    let text = Command::new("pactl").args([&format!("get-default-{what}")]).output().ok()?;
    if !text.status.success() {
        return None;
    }
    let name = String::from_utf8_lossy(&text.stdout).trim().to_string();
    (!name.is_empty()).then_some(name)
}

#[cfg(test)]
mod tests {
    use super::*;

    /// A real `pactl list short sources`, the format the sound server prints.
    const REAL: &str = "\
28\talsa_output.pci-0000_06_00.6.analog-stereo.monitor\tPipeWire\ts32le 2ch 48000Hz\tSUSPENDED
29\talsa_input.pci-0000_06_00.6.analog-stereo\tPipeWire\ts32le 2ch 48000Hz\tSUSPENDED
34\tbluez_input.54:7A:52:00:3B:17\tPipeWire\ts16le 1ch 16000Hz\tRUNNING
";

    #[test]
    fn a_graph_node_is_listed_by_the_name_the_sound_server_knows_it_by() {
        let nodes = parse_pactl_short(REAL);
        let names: Vec<&str> = nodes.iter().map(|n| n.name.as_str()).collect();
        assert_eq!(
            names,
            [
                "alsa_output.pci-0000_06_00.6.analog-stereo.monitor",
                "alsa_input.pci-0000_06_00.6.analog-stereo",
                "bluez_input.54:7A:52:00:3B:17",
            ]
        );
    }

    #[test]
    fn the_bluetooth_microphone_is_offered_beside_the_sound_card() {
        // The whole point: this name has never once appeared in the list,
        // because ALSA cannot see it.
        let extra = extra_device_names_from(
            parse_pactl_short(REAL)
                .into_iter()
                .filter(|n| !n.name.starts_with("alsa_"))
                .collect(),
        );
        assert_eq!(extra, ["bluez_input.54:7A:52:00:3B:17"]);
    }

    #[test]
    fn a_sound_card_is_not_offered_twice_under_two_names() {
        let nodes = parse_pactl_short(REAL);
        let extra = extra_device_names_from(
            nodes.into_iter().filter(|n| !n.name.starts_with("alsa_")).collect(),
        );
        assert!(
            !extra.iter().any(|n| n.starts_with("alsa_input.")),
            "the analog card is already in the ALSA list: {extra:?}"
        );
    }

    /// The filter, separated from the process so it can be tested without one.
    fn extra_device_names_from(nodes: Vec<GraphNode>) -> Vec<String> {
        nodes.into_iter().map(|n| n.name).collect()
    }

    #[test]
    fn a_bluetooth_microphone_is_recognised_as_a_graph_node() {
        assert!(is_graph_node_name("bluez_input.54:7A:52:00:3B:17"));
        assert!(is_graph_node_name("bluez_output.54_7A_52_00_3B_17.1"));
        assert!(!is_graph_node_name("Generic AB13X USB Audio, USB Audio [Audio]"));
        assert!(!is_graph_node_name("System default"));
    }

    /// The saved name has to keep working on a machine with no sound server at
    /// all, so a card is never mistaken for a node.
    #[test]
    fn a_sound_card_is_not_a_graph_node() {
        assert!(!is_graph_node_name("Built-in Audio Analog Stereo"));
    }

    #[test]
    fn a_malformed_line_is_skipped_rather_than_guessed_at() {
        let nodes = parse_pactl_short("not a node\n\n34\tbluez_output.55_55_55.1\tPipeWire\n");
        assert_eq!(nodes.len(), 1);
        assert_eq!(nodes[0].name, "bluez_output.55_55_55.1");
    }
}
#[cfg(test)]
mod hardware {
    use super::*;

    /// **The Midland's microphone appears by name, and picking it works.**
    ///
    /// The two halves are separate claims and both were false before: ALSA could
    /// not see the node so it was never listed, and a listed node could not be
    /// opened by cpal at all. Needs the DUALMIKE connected (`bluetoothctl
    /// connect 54:7A:52:00:3B:17`, because pairing alone attaches no audio
    /// profile — that cost a wasted run).
    #[test]
    #[ignore = "needs a Bluetooth microphone connected"]
    fn a_connected_bluetooth_microphone_is_listed_and_can_be_picked() {
        let name = std::env::var("SDROXIDE_BT_MIC").unwrap_or_else(|_| {
            "bluez_input.54:7A:52:00:3B:17".to_string()
        });
        let listed = extra_device_names(false);
        assert!(
            listed.contains(&name),
            "the node is not in the list: {listed:?} — is it connected, and is an \\
             audio profile attached?"
        );
        let (device, label, _) = crate::pick_device(&cpal::default_host(), Some(&name), false)
            .expect("the node could not be opened");
        assert!(!label.is_empty(), "opened the node with no label");
        println!("opened {name} as {label}");
        let _ = device;
    }
}

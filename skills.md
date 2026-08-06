# Agent guidance — signal-router

Read `ARCHITECTURE.md` before editing.

This repository owns only Router’s ordinary cross-boundary vocabulary. Keep
daemon actors, storage, routing policy, sockets, and cryptography in `router`;
keep owner channel-policy orders in `meta-signal-router`.

`ethos/interface.ethos` is the sole schema authority. Generated Rust identities
must remain encoded. Regenerate with
`SIGNAL_ROUTER_UPDATE_INTERFACE_ARTIFACTS=1 cargo build`; do not hand-edit the
generated projection.

Every root variant needs exhaustive bound-frame, rkyv, and Dotos witnesses.
Wire enums stay closed. Missing entities receive positive typed outcomes, not
an open `Unknown` escape hatch.

The repository is under fast development and constantly breaking.

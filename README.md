# signal-router

Ordinary Signal contract for Persona router-owned observations and relations.

The first use is introspection: `introspect` can ask the router for a typed
summary, message trace, or channel state without opening `router.sema`. The
contract also carries the encrypted authenticated peer session handshake, the
router-to-router forwarding relation, actor registration, and the router-owned
bootstrap and daemon configuration records.

`ethos/signal.ethos` is the schema authority. `build.rs` actualizes it through
`ethos-zero` and asserts the checked-in projection in `src/generated/signal.rs`
equals a fresh generation. `examples/canonical.datom` carries one Datom line per
contract head, every one of them written by the codec that reads it back.

One request is one Signal frame carrying the rkyv archive of `Query`; one reply
is one frame of `Response`. There is no envelope. The portable frame comes from
`signal` and is re-exported here, so a router frame is the same Rust type as
every other contract's frame.

See `ARCHITECTURE.md`.

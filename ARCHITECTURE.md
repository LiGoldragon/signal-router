# signal-persona-router - architecture

*Signal contract for Persona router-owned observations and relations.*

## 0. Intent

`persona-router` needs a contract home for router state that is visible outside
the router. The first pressure is `persona-introspect`: it needs to ask the
router what happened to a message without reading `router.redb` directly.

This crate exists because router observations do not belong in
`signal-persona-introspect` and do not belong in `signal-persona-message`.
The former asks and wraps; the latter owns message ingress.

## 1. Owned surface

- `RouterRequest`
- `RouterReply`
- Router summary queries and replies.
- Router message trace queries and replies.
- Router channel state queries and replies.

Current request variants are read-shaped observation queries. They all declare
their root verb in the `signal_channel!` declaration, which generates
`RouterRequest::signal_verb()` and `RouterRequest::into_request()`.
All current variants map to `Match`:

```text
Summary      -> Match
MessageTrace -> Match
ChannelState -> Match
```

No router observation request may be wrapped as `Assert`; write-shaped router
state changes belong in a separate request variant with its own root-verb
witness.

## 2. Constraints

| Constraint | Witness |
|---|---|
| Router observations have a router-owned contract home. | This crate exists; central introspection contract does not define router rows. |
| Every request/reply travels as a Signal frame. | `tests/round_trip.rs` length-prefixed frame tests. |
| Router observation queries use the `Match` root. | `RouterRequest::signal_verb()` plus round-trip tests assert `SignalVerb::Match`. |
| Message ingress remains in `signal-persona-message`. | This crate imports `MessageSlot` but does not redefine message submission records. |
| Runtime code stays out of the contract. | Source scan: no Kameo, Tokio, socket, or redb code. |

## 3. Prototype status

Scaffold exists. The next implementation step is wiring `persona-router` to
answer these requests from its own state actors.

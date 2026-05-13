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

## 2. Constraints

| Constraint | Witness |
|---|---|
| Router observations have a router-owned contract home. | This crate exists; central introspection contract does not define router rows. |
| Every request/reply travels as a Signal frame. | `tests/round_trip.rs` length-prefixed frame tests. |
| Message ingress remains in `signal-persona-message`. | This crate imports `MessageSlot` but does not redefine message submission records. |
| Runtime code stays out of the contract. | Source scan: no Kameo, Tokio, socket, or redb code. |

## 3. Prototype status

Scaffold exists. The next implementation step is wiring `persona-router` to
answer these requests from its own state actors.

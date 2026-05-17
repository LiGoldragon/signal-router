# signal-persona-router — architecture

*Signal contract for Persona router-owned observations and relations.*

## 0 · TL;DR

`signal-persona-router` is the typed contract for the router's
component-owned wire vocabulary. It carries the observation channel
`persona-introspect` uses to ask the router what happened to a message,
a channel, or an engine. It also carries the manager-written router
bootstrap vocabulary consumed by `persona-router` at daemon startup.

There is one `signal_channel!` invocation in `src/lib.rs` declaring the
`Router` observation channel. Bootstrap is not a live request/reply
channel; it is a typed startup document projected as line-oriented NOTA
records for the current manager-to-router handoff.

Closed enums on the wire; positive names for "entity not in store"
cases; one reply variant per concrete observation shape. Slot-lookup
miss is a distinct `MessageTraceMissing` reply variant, not a sentinel
status inside `RouterMessageTrace`. Channel absence is the positive
`RouterChannelStatus::Missing`, not a polling-shape `Unknown`.

## 1 · Channel

| Side | Component |
|---|---|
| Request side | `persona-introspect` (today); other observation clients later. |
| Reply side | `persona-router` |

The router answers observation queries. The crate carries no
streaming subscription today: all current variants are one-shot
`Match` reads.

## 2 · Owned surface

- `RouterRequest` / `RouterReply` (closed enums).
- `RouterBootstrapDocument` / `RouterBootstrapOperation`.
- Bootstrap operation records:
  - `RegisterActor`
  - `GrantDirectMessage`
  - `InstallStructuralChannels`
- Bootstrap actor endpoint records:
  - `ActorId`
  - `Actor`
  - `EndpointTransport`
  - `EndpointKind`
- `RouterSummaryQuery` / `RouterSummary`.
- `RouterMessageTraceQuery` and the **two-variant reply split**:
  - `RouterReply::MessageTrace(RouterMessageTrace)` — slot present;
    `status` is a closed `RouterDeliveryStatus`.
  - `RouterReply::MessageTraceMissing(RouterMessageTraceMissing)` —
    slot not in store. The split keeps the inner status enum closed.
- `RouterChannelStateQuery` / `RouterChannelState` /
  `RouterChannelStatus`. The "slot not in store" case is the positive
  `Missing` variant.
- `RouterObservationUnimplemented` + closed
  `RouterObservationUnimplementedReason`.
- `signal_channel!`-generated `RouterRequest::signal_verb()` and
  `RouterRequest::into_request()` for verb-witness round trips.

## 3 · Closed-enum integrity

Wire enums in this crate are closed; no `Unknown` placeholder
smuggles polling-shape uncertainty across the boundary. The closed
shapes:

```text
RouterDeliveryStatus
  | Accepted
  | Routed
  | Delivered
  | Deferred
  | Failed

RouterChannelStatus
  | Installed
  | Missing            -- positive name for "no slot in store"
  | Disabled

RouterObservationUnimplementedReason
  | NotInPrototypeScope
  | RouterStoreUnavailable
  | MessageTraceUnavailable
```

`Missing` is a domain answer, not a polling sentinel. It says "we
looked; nothing is bound to this channel id." A consumer that sees
`Missing` does not retry the same query expecting a different answer;
it acts on the closed observation. The same shape applies to
`MessageTraceMissing` reply variant at the reply level — slot
presence/absence pivots at the reply variant, not by sentinel inside a
present reply.

## 4 · Signal root verbs

Every `RouterRequest` variant declares its root verb in the
`signal_channel!` declaration. `signal-core` generates
`RouterRequest::signal_verb()` and `RouterRequest::into_request()`
from that declaration. All current variants are read-shaped:

```text
Summary       -> Match
MessageTrace  -> Match
ChannelState  -> Match
```

No router observation request may be wrapped as `Assert`; write-shaped
router state changes belong in a separate request variant with its own
root-verb witness once they earn a contract surface.

## 5 · Constraints

| Constraint | Witness |
|---|---|
| Router observations have a router-owned contract home. | This crate exists; central introspection contract does not define router rows. |
| Every request/reply travels as a Signal frame. | `tests/round_trip.rs` length-prefixed frame tests per variant. |
| Manager-written router bootstrap uses router-owned typed vocabulary, not duplicated private records in `persona`. | `RouterBootstrapDocument` and `RouterBootstrapOperation` live in this crate; `bootstrap_document_owns_line_vocabulary_for_manager_and_router` round-trips the line projection. |
| Router observation queries use the `Match` root. | `RouterRequest::signal_verb()` plus `router_request_variants_declare_match_as_signal_root_verb`. |
| Message ingress remains in `signal-persona-message`. | This crate imports `MessageSlot` but does not redefine message submission records. |
| Runtime code stays out of the contract. | Source scan: no Kameo, Tokio, socket, or redb code. |
| Wire enums contain no `Unknown` variant. | `tests/round_trip.rs::router_status_enums_are_closed_no_unknown_variants` exhaustively matches every `RouterDeliveryStatus` and `RouterChannelStatus` variant. Adding an `Unknown` variant breaks the match. |
| Any record name containing the word `Unknown` represents a positive "entity not in our state" rejection, not a polling-shape escape hatch. | This crate has no such records today; reply absence pivots at the reply variant (`MessageTraceMissing`) and channel absence at the positive `RouterChannelStatus::Missing`. |
| Slot lookup miss travels as the typed `MessageTraceMissing` reply variant, not a sentinel inside `RouterMessageTrace.status`. | `router_message_trace_missing_reply_round_trips_through_length_prefixed_frame`. |
| Every `signal_channel!` request variant has a typed `signal_verb()` mapping. | `router_request_variants_declare_match_as_signal_root_verb` asserts the mapping for every variant. |
| Round-trip witnesses cover every variant in rkyv. | `tests/round_trip.rs` exercises every request and reply variant through `Frame::encode_length_prefixed` / `decode_length_prefixed`. |
| Round-trip witnesses cover every variant in NOTA. | `examples/canonical.nota` holds one canonical text example per request/reply variant; round-trip tests parse and re-emit each. |
| Bootstrap line records round-trip through NOTA using the contract crate. | `bootstrap_register_actor_operation_round_trips_through_nota_line`, `bootstrap_direct_message_grant_operation_round_trips_through_nota_line`, and `bootstrap_document_owns_line_vocabulary_for_manager_and_router`. |
| No stringly-typed dispatch (`match s.as_str()`) for closed-set states. | All status/scope/reason fields are typed closed enums. |
| Contract crate dependencies use a named API reference (branch or tag), not a raw revision pin. | `Cargo.toml` review: `signal-core`, `signal-persona-auth`, `signal-persona-message`, `nota-codec` are declared `git = "..."` with a named-branch shape; raw `rev = "..."` pins are not used. |

## 6 · NOTA codec quirk on `signal_channel!` payload heads

The `signal_channel!` macro emits a request variant's NOTA head as
the **payload's record head**, not the Rust variant name. For
example, `RouterRequest::Summary(RouterSummaryQuery { .. })` encodes
as `(RouterSummaryQuery (...))`, not `(Summary (...))`. Tests and
canonical examples carry the payload heads. The same shape applies
to reply variants: `RouterReply::MessageTraceMissing(RouterMessageTraceMissing { .. })`
encodes as `(RouterMessageTraceMissing (...))`.

## 7 · Versioning

`signal_core::Frame` carries the protocol version. Schema-level
changes are breaking; coordinate `persona-router` and observation
consumers (`persona-introspect`) on the upgrade.

This crate depends on `signal-core` via a named-branch reference, not
a raw revision pin. The destination is a stable `signal-core` API
branch/bookmark once that lane is declared.

## 8 · Non-ownership

- No router daemon — that is `persona-router`.
- No introspection daemon — that is `persona-introspect`.
- No router redb table layout — `persona-router` owns it.
- No subscription accounting — there is no subscription today.
- No transport (UDS path, reconnect, timeouts).

## 9 · Code map

```text
src/
└── lib.rs                — payloads + signal_channel! invocation
examples/
└── canonical.nota         — one canonical example per request/reply variant
tests/
└── round_trip.rs          — per-variant frame round trips + NOTA witnesses
                             + closed-enum + verb-mapping witnesses
                             + bootstrap line-projection witness
                             + canonical examples parser
```

## See also

- `signal-core/src/channel.rs` — the macro
- `signal-persona-message/ARCHITECTURE.md` — companion crate that
  carries message ingress records this crate imports.
- `signal-persona-introspect/ARCHITECTURE.md` — the central
  introspection envelope that wraps router observations.

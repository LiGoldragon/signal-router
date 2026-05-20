# signal-persona-router — architecture

*Signal contract for Persona router-owned observations and relations.*

## 0 · TL;DR

`signal-persona-router` is the typed contract for the router's
component-owned wire vocabulary. It carries the observation channel
`persona-introspect` uses to ask the router what happened to a message,
a channel, or an engine. It also carries the manager-written router
bootstrap vocabulary consumed by `persona-router` at daemon startup.

Owner-only channel policy orders are not part of this ordinary
observation contract. Grants, extensions, revocations, and
adjudication denials live in `owner-signal-persona-router`, the
router's policy signal.

## MUST IMPLEMENT — three-layer migration

This contract is migrating to the three-layer model affirmed
2026-05-20 per
`primary/reports/designer/246-v4-bundled-fix-deep-design-with-examples.md`
and `primary/reports/designer/248-three-layer-changes-for-operators.md`.

**Layer 1 — Contract Operations on the wire (this crate).** Drop the
SignalVerb wrappers entirely. The current shape — three siblings
`Match Summary(RouterSummaryQuery)`, `Match MessageTrace(RouterMessageTraceQuery)`,
`Match ChannelState(RouterChannelStateQuery)` — collapses to one
contract-local verb root. Use `Observe` as the read verb (router
domain is observation of routed work), with a closed payload enum
naming the observation kind (`Summary`, `MessageTrace`, `ChannelState`).
Alternatively `Query` if the receiver context reads more naturally as
querying router state. Drop the `Router*` prefix from `RouterRequest`,
`RouterReply`, `RouterSummaryQuery`, `RouterDeliveryStatus`, etc. —
the crate namespace already supplies "router." The bootstrap
vocabulary (`RouterBootstrapDocument` / `RouterBootstrapOperation`
and the `RegisterActor` / `GrantDirectMessage` /
`InstallStructuralChannels` operations) is not a live request/reply
channel; those records are fine as typed data records.

**Mandatory `Tap`/`Untap` for persona components.** Persona-router
is a persona component, so its observable surface is standardized.
Add a mandatory `observable { … }` block; the macro injects
`Tap(ObserverFilter)` / `Untap(RouterObserverSubscriptionToken)`
verbs for the standardized observer hook that `persona-introspect`
subscribes to.

**Layer 2 — Component Commands (persona-router daemon).** The router
daemon owns its typed Command enum (e.g.
`RouterCommand::ReadRouterSummary`,
`RouterCommand::ReadMessageTrace`,
`RouterCommand::ReadChannelState`,
`RouterCommand::RegisterActor`,
`RouterCommand::InstallStructuralChannel`) plus a `CommandExecutor`
that knows the router's tables.

**Layer 3 — Sema classification (signal-sema).** Each Component
Command projects to a payloadless `SemaOperation` class via
`ToSemaOperation`. Cross-component observers filter by class.

**Frame layer.** The dependency on `signal-core` shifts to
`signal-frame`.

References:
- `primary/reports/designer/246-v4-bundled-fix-deep-design-with-examples.md`
- `primary/reports/designer/248-three-layer-changes-for-operators.md`
- `primary/skills/component-triad.md` §"Verbs come in three layers"
- `primary/skills/contract-repo.md` §"Public contracts use contract-local operation verbs"

**Note to remover:** when the refactor lands, remove this section and
add a `## Migration history — three-layer model (2026-05-XX)`
paragraph noting the shape change.

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
- Contract-local verbs declared in the `signal_channel!` invocation;
  Sema classification (Layer 3) is daemon-side projection only.

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

## 4 · Sema-class projections (Layer 3)

Each contract-local operation's daemon-side Component Command
projects to a payloadless Sema class label for observation. All
current operations are read-shaped:

```text
Observe (Summary kind)        -> Match
Observe (MessageTrace kind)   -> Match
Observe (ChannelState kind)   -> Match
Tap (mandatory)               -> Subscribe
Untap (mandatory)             -> Retract
```

The wire form carries the contract-local verb only; the Sema class
label is computed at observation publish time inside the daemon.

Write-shaped router state changes belong on the authority surface that
matches who may call them. Owner-only channel policy changes live in
`owner-signal-persona-router`; peer-callable router writes, once they
earn a contract surface, belong in this ordinary contract. Their
Component Commands project to `Assert` / `Mutate` / `Retract` as
appropriate.

## 5 · Constraints

| Constraint | Witness |
|---|---|
| Router observations have a router-owned contract home. | This crate exists; central introspection contract does not define router rows. |
| Every request/reply travels as a Signal frame. | `tests/round_trip.rs` length-prefixed frame tests per variant. |
| Manager-written router bootstrap uses router-owned typed vocabulary, not duplicated private records in `persona`. | `RouterBootstrapDocument` and `RouterBootstrapOperation` live in this crate; `bootstrap_document_owns_line_vocabulary_for_manager_and_router` round-trips the line projection. |
| Router observation queries are contract-local verbs in verb form; their daemon-side Component Commands project to Sema `Match`. | Daemon-side `ToSemaOperation` impl is the witness; round-trip tests assert each variant's NOTA head. |
| Message ingress remains in `signal-persona-message`. | This crate imports `MessageSlot` but does not redefine message submission records. |
| Owner-only router channel policy orders remain out of this ordinary observation contract. | `owner-signal-persona-router` owns `Grant`, `Extend`, `Revoke`, and `Deny`; this crate does not define those operations. |
| Runtime code stays out of the contract. | Source scan: no Kameo, Tokio, socket, or redb code. |
| Wire enums contain no `Unknown` variant. | `tests/round_trip.rs::router_status_enums_are_closed_no_unknown_variants` exhaustively matches every `RouterDeliveryStatus` and `RouterChannelStatus` variant. Adding an `Unknown` variant breaks the match. |
| Any record name containing the word `Unknown` represents a positive "entity not in our state" rejection, not a polling-shape escape hatch. | This crate has no such records today; reply absence pivots at the reply variant (`MessageTraceMissing`) and channel absence at the positive `RouterChannelStatus::Missing`. |
| Slot lookup miss travels as the typed `MessageTraceMissing` reply variant, not a sentinel inside `RouterMessageTrace.status`. | `router_message_trace_missing_reply_round_trips_through_length_prefixed_frame`. |
| Each variant's NOTA head matches the contract-local verb declared in `signal_channel!`. | Generated by the macro; round-trip tests assert each variant's head. |
| Round-trip witnesses cover every variant in rkyv. | `tests/round_trip.rs` exercises every request and reply variant through `Frame::encode_length_prefixed` / `decode_length_prefixed`. |
| Round-trip witnesses cover every variant in NOTA. | `examples/canonical.nota` holds one canonical text example per request/reply variant; round-trip tests parse and re-emit each. |
| Bootstrap line records round-trip through NOTA using the contract crate. | `bootstrap_register_actor_operation_round_trips_through_nota_line`, `bootstrap_direct_message_grant_operation_round_trips_through_nota_line`, and `bootstrap_document_owns_line_vocabulary_for_manager_and_router`. |
| No stringly-typed dispatch (`match s.as_str()`) for closed-set states. | All status/scope/reason fields are typed closed enums. |
| Contract crate dependencies use a named API reference (branch or tag), not a raw revision pin. | `Cargo.toml` review: `signal-frame`, `signal-persona-auth`, `signal-persona-message`, `nota-codec` are declared `git = "..."` with a named-branch shape; raw `rev = "..."` pins are not used. |

## 6 · NOTA codec quirk on `signal_channel!` payload heads

The `signal_channel!` macro emits a request variant's NOTA head as
the **payload's record head**, not the Rust variant name. For
example, `RouterRequest::Summary(RouterSummaryQuery { .. })` encodes
as `(RouterSummaryQuery (...))`, not `(Summary (...))`. Tests and
canonical examples carry the payload heads. The same shape applies
to reply variants: `RouterReply::MessageTraceMissing(RouterMessageTraceMissing { .. })`
encodes as `(RouterMessageTraceMissing (...))`.

## 7 · Versioning

`signal_frame::Frame` carries the protocol version. Schema-level
changes are breaking; coordinate `persona-router` and observation
consumers (`persona-introspect`) on the upgrade.

This crate depends on `signal-frame` via a named-branch reference, not
a raw revision pin. The destination is a stable `signal-frame` API
branch/bookmark once that lane is declared.

## 8 · Non-ownership

- No router daemon — that is `persona-router`.
- No introspection daemon — that is `persona-introspect`.
- No router redb table layout — `persona-router` owns it.
- No subscription accounting — there is no subscription today.
- No transport (UDS path, reconnect, timeouts).
- No owner-only channel policy orders; those live in
  `owner-signal-persona-router`.

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

- `owner-signal-persona-router/ARCHITECTURE.md` — owner-only router
  channel policy orders.
- `signal-frame/macros/src/validate.rs` — the macro
- `~/primary/skills/component-triad.md` §"Verbs come in three layers".
- `signal-persona-message/ARCHITECTURE.md` — companion crate that
  carries message ingress records this crate imports.
- `signal-persona-introspect/ARCHITECTURE.md` — the central
  introspection envelope that wraps router observations.

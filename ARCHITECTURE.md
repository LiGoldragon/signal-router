# signal-router — architecture

*Signal contract for Persona router-owned observations and relations.*

## 0 · TL;DR

`signal-router` is the typed contract for the router's
component-owned wire vocabulary. It carries the observation channel
`introspect` uses to ask the router what happened to a message,
a channel, or an engine. It also carries the manager-written router
bootstrap vocabulary consumed by `router` at daemon startup.

Meta channel-policy orders are not part of this ordinary
observation contract. Grants, extensions, revocations, and
adjudication denials live in `meta-signal-router`, the
router's policy signal, called by Orchestrate.
Mind decides at the cognitive level and orders Orchestrate first; it
does not call Router's meta signal directly.

## Wire operation heads

The public wire carries only bare contract-local operation heads. The three
router observation reads are:
`Summary`, `MessageTrace`, and `ChannelState`. Durable read/write
classification is daemon-side only.

This schema-derived crate depends on `signal-frame` for length-prefixed
rkyv framing. It still owns only wire vocabulary, NOTA codecs, and
bootstrap records; it does not own daemon actors, store tables, sockets,
or routing policy.

`schema/lib.schema` declares the contract roots and payload records.
`schema-rust-next` generates `Input`, `Output`, short-header routes,
frame aliases, and codecs into `src/schema/lib.rs`; `src/lib.rs`
re-exports the generated surface and keeps only small contract-owned
helpers for bootstrap line projection and rkyv configuration archives.
Bootstrap is not a live request/reply channel; it is a typed startup
document projected as line-oriented NOTA records for the current
manager-to-router handoff.

Closed enums on the wire; positive names for "entity not in store"
cases; one reply variant per concrete observation shape. Slot-lookup
miss is a distinct `MessageTraceMissing` reply variant, not a sentinel
status inside `RouterMessageTrace`. Channel absence is the positive
`RouterChannelStatus::Missing`, not a polling-shape `Unknown`.

## 1 · Channel

| Side | Component |
|---|---|
| Request side | `introspect` (today); other observation clients later. |
| Reply side | `router` |

The router answers observation queries. The crate carries no
streaming subscription today: all current variants are one-shot
observation reads.

## 2 · Owned surface

- `Input` / `Output` (closed generated wire enums).
- `RouterBootstrapDocument` / `RouterBootstrapOperation`.
- Bootstrap operation records:
  - `RegisterActor`
  - `GrantDirectMessage`
  - `InstallStructuralChannels`
- Bootstrap actor endpoint records:
  - `ActorIdentifier`
  - `Actor`
  - `EndpointTransport`
  - `EndpointKind`
- `RouterSummaryQuery` / `RouterSummary`.
- `RouterMessageTraceQuery` and the **two-variant reply split**:
  - `Output::MessageTrace(RouterMessageTrace)` — slot present;
    `status` is a closed `RouterDeliveryStatus`.
  - `Output::MessageTraceMissing(RouterMessageTraceMissing)` —
    slot not in store. The split keeps the inner status enum closed.
- `RouterChannelStateQuery` / `RouterChannelState` /
  `RouterChannelStatus`. The "slot not in store" case is the positive
  `Missing` variant.
- `RouterObservationUnimplemented` + closed
  `RouterObservationUnimplementedReason`.
- Contract-local verbs declared as root variants in `schema/lib.schema`;
  durable read/write classification is daemon-side only.

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

## 4 · Daemon Lowering Boundary

Each contract-local operation lowers inside `router` into a daemon-owned Nexus
command and any SEMA reads or writes needed to answer it. All current live
request variants are observation reads. The public wire carries only the
contract-local operation head; it never carries `Assert`, `Mutate`, `Retract`,
`Match`, `Subscribe`, or `Validate`, and this crate has no `signal-sema`
dependency.

Write-shaped router state changes belong on the authority surface that
matches who may call them. Meta channel-policy changes live in
`meta-signal-router` and are issued by Orchestrate;
peer-callable router writes, once they earn a contract surface, belong
in this ordinary contract. Their database effects still remain daemon-owned
lowering, not public operation roots.

## 5 · Constraints

| Constraint | Witness |
|---|---|
| Router observations have a router-owned contract home. | This crate exists; central introspection contract does not define router rows. |
| Every request/reply travels as a Signal frame. | `tests/round_trip.rs` length-prefixed frame tests per variant. |
| Manager-written router bootstrap uses router-owned typed vocabulary, not duplicated private records in `persona`. | `RouterBootstrapDocument` and `RouterBootstrapOperation` live in this crate; `bootstrap_document_owns_line_vocabulary_for_manager_and_router` round-trips the line projection. |
| Router observation queries are contract-local operation heads, never universal database-action class roots. | `router_request_heads_are_contract_local_operations` and `router_contract_has_no_sema_classification_dependency_or_roots`. |
| Message ingress remains outside this contract. | This crate carries only the router-facing `MessageSlot` scalar needed to observe a routed message; message submission records remain outside `signal-router`. |
| Meta router channel policy orders remain out of this ordinary observation contract. | `meta-signal-router` owns `Grant`, `Extend`, `Revoke`, and `Deny`; Orchestrate calls that meta contract; this crate does not define those operations. |
| Runtime code stays out of the contract. | Source scan: no Kameo, Tokio, socket, or storage code. |
| Wire enums contain no `Unknown` variant. | `tests/round_trip.rs::router_status_enums_are_closed_no_unknown_variants` exhaustively matches every `RouterDeliveryStatus` and `RouterChannelStatus` variant. Adding an `Unknown` variant breaks the match. |
| Any record name containing the word `Unknown` represents a positive "entity not in our state" rejection, not a polling-shape escape hatch. | This crate has no such records today; reply absence pivots at the reply variant (`MessageTraceMissing`) and channel absence at the positive `RouterChannelStatus::Missing`. |
| Slot lookup miss travels as the typed `MessageTraceMissing` reply variant, not a sentinel inside `RouterMessageTrace.status`. | `router_message_trace_missing_reply_round_trips_through_length_prefixed_frame`. |
| Each variant's NOTA head matches the contract-local verb declared in `schema/lib.schema`. | Generated by `schema-rust-next`; round-trip tests assert each variant's head. |
| Round-trip witnesses cover every variant in rkyv. | `tests/round_trip.rs` exercises every request and reply variant through `Frame::encode_length_prefixed` / `decode_length_prefixed`. |
| Round-trip witnesses cover every variant in NOTA. | `examples/canonical.nota` holds one canonical text example per request/reply variant; round-trip tests parse and re-emit each. |
| Bootstrap line records round-trip through NOTA using the contract crate. | `bootstrap_register_actor_operation_round_trips_through_nota_line`, `bootstrap_direct_message_grant_operation_round_trips_through_nota_line`, and `bootstrap_document_owns_line_vocabulary_for_manager_and_router`. |
| No stringly-typed dispatch (`match s.as_str()`) for closed-set states. | All status/scope/reason fields are typed closed enums. |
| Contract crate dependencies use a named API reference (branch or tag), not a raw revision pin. | `Cargo.toml` review: `nota-next`, `signal-frame`, and `schema-rust-next` are declared `git = "..."` with a named-branch shape; raw `rev = "..."` pins are not used. |

## 6 · NOTA codec shape on schema operation heads

The schema-derived codec emits a root variant's NOTA head as the operation
head. For example, `Input::Summary(RouterSummaryQuery)` encodes as
`(Summary prototype)`, while struct payload roots keep their record body,
such as `(MessageTrace (prototype 7))`. Tests and canonical examples carry
the operation heads. The same shape applies to reply variants:
`Output::MessageTraceMissing(RouterMessageTraceMissing { .. })` encodes as
`(MessageTraceMissing (prototype 99))`.

## 7 · Versioning

`signal_frame::Frame` carries the protocol version. Schema-level
changes are breaking; coordinate `router` and observation
consumers (`introspect`) on the upgrade.

This crate depends on `signal-frame` via a named-branch reference, not
a raw revision pin. The destination is a stable `signal-frame` API
branch/bookmark once that lane is declared.

## 8 · Non-ownership

- No router daemon — that is `router`.
- No introspection daemon — that is `introspect`.
- No router sema-engine table layout — `router` owns it.
- No subscription accounting — there is no subscription today.
- No transport (UDS path, reconnect, timeouts).
- No meta channel-policy orders; those live in `meta-signal-router`.

## 9 · Code map

```text
src/
├── lib.rs                — generated surface re-export + small contract helpers
└── schema/
    ├── mod.rs
    └── lib.rs            — checked-in schema-rust-next generated wire contract
schema/
└── lib.schema            — authored wire roots and payload records
examples/
└── canonical.nota         — one canonical example per request/reply variant
tests/
└── round_trip.rs          — per-variant frame round trips + NOTA witnesses
                             + closed-enum + operation-head witnesses
                             + bootstrap line-projection witness
                             + canonical examples parser
```

## See also

- `meta-signal-router/ARCHITECTURE.md` — meta router
  channel policy orders.
- `~/primary/skills/component-triad.md`.
- `signal-message/ARCHITECTURE.md` — companion crate that carries message
  ingress records outside this observation contract.
- `signal-introspect/ARCHITECTURE.md` — the central
  introspection envelope that wraps router observations.

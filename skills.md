# skills — signal-router

*Per-repo agent guide for the router-owned Signal observation
contract.*

## Checkpoint — read before editing

Before changing code in this repo, read:

- `~/primary/skills/contract-repo.md`
- `~/primary/skills/architecture-editor.md`
- `~/primary/skills/architectural-truth-tests.md`
- `~/primary/skills/subscription-lifecycle.md` (when adding any
  subscription / event / retract variant)
- `~/primary/skills/nix-discipline.md`
- this repo's `ARCHITECTURE.md`
- `router/ARCHITECTURE.md`
- `introspect/ARCHITECTURE.md` when touching observation
  records.

## What this repo is for

`signal-router` is the typed contract `introspect`
(and any future observation consumer) uses to ask the router what
happened to a message, a channel, or an engine — without opening the
router's sema-engine store directly.

The crate carries the router's observation request/reply vocabulary
and nothing else. It is the contract for **router-owned facts that
need to cross a wire** — not the place for router daemon code,
storage tables, or runtime actors.

## What this repo owns

- Router-owned Signal vocabulary: generated `Input`, generated `Output`,
  and the typed query / observation records they carry.
- Router observation records used by `introspect`.
- The two-variant reply split for slot-lookup miss
  (`MessageTraceMissing`) keeping `RouterDeliveryStatus` closed.
- Future router operational relations when they are extracted from
  consumers.

## What this repo does not own

- Router daemon code, actor logic, state reducers, or storage tables.
- Meta-only channel policy orders; `meta-signal-router`
  owns grants, extensions, revocations, and adjudication denials,
  called by Orchestrate.
- Message ingress records owned by `signal-message`.
- Introspection query envelopes owned by `signal-introspect`.

## Load-bearing invariants

These are the rules that make this contract crate useful. Break one
and downstream code breaks silently.

- **Wire enums are closed.** No `Unknown` variant. Slot-lookup miss
  pivots at the **reply variant** (`MessageTraceMissing`), not at a
  sentinel inside a present reply. Channel absence is the positive
  `RouterChannelStatus::Missing`. Any future "entity not in our
  state" answer is a new positive variant, not a polling-shape
  placeholder.
- **Every request variant declares a contract-local operation head.**
  `schema/lib.schema` is the source of truth; tests assert the exact
  heads.
- **No runtime code.** No Kameo, Tokio, socket, storage, or daemon
  glue in this crate. The contract is the typed vocabulary; the
  runtime is `router`.
- **Round trips cover every variant.** rkyv length-prefixed frame
  round trips in `tests/round_trip.rs`; canonical NOTA examples in
  `examples/canonical.nota` with a parser test.
- **Pin upstream contracts via a named API reference.** Cargo deps
  to `signal-frame` and `schema-rust` use `git = "..."`
  with a named branch/bookmark, never raw
  `rev = "..."`.

## Editing patterns

### Adding a new observation request

1. Write the canonical NOTA example for the request and the expected
   reply in `examples/canonical.nota` first. Per
   `~/primary/skills/contract-repo.md` §"Examples-first round-trip
   discipline", the example is the falsifiable spec.
2. Declare the payload struct and root variant in `schema/lib.schema`.
3. Regenerate `src/schema/lib.rs` with
   `SIGNAL_ROUTER_UPDATE_SCHEMA_ARTIFACTS=1 cargo build`.
4. Add the rkyv round-trip test in `tests/round_trip.rs`.
5. Add the NOTA round-trip witness for the new variant in the
   canonical-examples test.
6. Update `ARCHITECTURE.md` §"Owned surface" and the messages table.

### Modeling "entity not in store"

Two options, both closed:

- **Reply-variant split** — distinct reply variant for the absence
  case (the `MessageTraceMissing` shape). Use this when the inner
  payload differs structurally between present and absent.
- **Positive enum variant** — name the absence positively (the
  `RouterChannelStatus::Missing` shape). Use this when the carrier
  record's shape doesn't change.

Never add `Unknown` to encode either. That's an open-world escape
hatch; the workspace forbids it on wire enums.

### Adding a subscription variant

This crate is non-streaming today. If a subscription lands:

1. Read `~/primary/skills/subscription-lifecycle.md` end-to-end.
2. Declare the semantic stream in the schema surface once the schema
   stream grammar is available for this contract, with both a
   request-side typed close operation and a reply-side close
   acknowledgement.
3. Keep `Subscribe` and `Retract` out of ordinary public request roots unless
   the current contract discipline explicitly ratifies the stream grammar for
   that relation.
4. Witness the full subscribe → event → retract → ack → end
   lifecycle in `tests/round_trip.rs`.

## NOTA codec shape

The schema-derived codec emits a root variant's NOTA head as the operation
head. For example, `Input::Summary(RouterSummaryQuery)` encodes as
`(Summary [prototype])`. Canonical examples and round-trip tests use the
operation heads.

## See also

- this workspace's `skills/contract-repo.md` — contract-repo
  discipline.
- this workspace's `skills/subscription-lifecycle.md` — canonical
  subscription FSM.
- this workspace's `skills/architectural-truth-tests.md` — witness
  discipline for the constraints in `ARCHITECTURE.md`.
- this workspace's `ESSENCE.md` §"Perfect specificity at
  boundaries" — the rule the closed-enum discipline implements.

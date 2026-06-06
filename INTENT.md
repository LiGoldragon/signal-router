# INTENT — signal-router

*The ordinary peer-callable wire contract for Persona's router. Defines
the typed observation channel that `introspect` (and future observation
clients) use to ask the router what happened to a message, a channel, or
an engine, plus the manager-written router bootstrap vocabulary.
Companion to `ARCHITECTURE.md` and `Cargo.toml`. Maintenance:
`primary/skills/repo-intent.md`.*

## Repo-scope only

This file carries only the intent that is FOR this `signal-router`
contract. Workspace-shape intent stays in the primary workspace
`primary/INTENT.md`. Component daemon intent stays in `router/INTENT.md`.
Meta channel-policy intent stays in
`meta-signal-router/INTENT.md`.

## Why this repo exists

`signal-router` is the **ordinary peer-callable wire contract** for the
`router` daemon. It exists so `introspect` can ask the router for a
typed summary, message trace, or channel state without opening
`router.redb`. It also carries the manager-written router bootstrap
vocabulary the daemon consumes at startup. Meta channel-policy
orders — grants, extensions, revocations, adjudication denials — stay in
`meta-signal-router`, called by Orchestrate; runtime
actors, sockets, storage, and routing logic live in `router`.

## The channel shape

The router observation channel carries:

- **Requests:** `Observe`/`Query` reads over a closed payload enum
  naming the observation kind (`Summary`, `MessageTrace`,
  `ChannelState`), plus the mandatory `Tap`/`Untap` standardized
  observer hook persona components carry.
- **Replies:** one reply variant per concrete observation shape
  (`Summary`, `MessageTrace`, `ChannelState`), with absence pivoting at
  the reply variant (`MessageTraceMissing`) and at the positive
  `RouterChannelStatus::Missing`, plus `Unimplemented` for
  skeleton-honest unbuilt behavior.
- **Bootstrap vocabulary:** `RouterBootstrapDocument` /
  `RouterBootstrapOperation` (`RegisterActor`, `GrantDirectMessage`,
  `InstallStructuralChannels`) as typed startup data records, not a live
  request/reply channel.

The wire vocabulary is contract-local — the daemon lowers these public
operations into component-local commands; Sema classification happens at
observation publish time, not on the wire.

## Channels are closed, boundaries are named

- Wire enums are closed. No `Unknown` escape hatch.
- "Entity not in our state" is a positive answer — `MessageTraceMissing`
  reply variant and `RouterChannelStatus::Missing` — not a polling-shape
  sentinel a consumer retries against.
- Request payloads do not mint router-owned identity, timestamps, or
  sequence numbers; `router` mints those at the daemon.
- No stringly-typed dispatch. Delivery status, channel status, and
  reason fields are typed closed enums.

## Wire vocabulary discipline

Per `primary/skills/contract-repo.md` §"Public contracts use
contract-local operation verbs":

- Operation roots are domain verbs in verb form (`Observe` / `Query`),
  not the Sema class word `Match`. The six Sema classification words must
  not appear as request roots on this wire.
- Reply success variants name the concrete observation shape returned.
- Payload record names drop the redundant `Router*` prefix where the
  crate namespace already supplies "router."

## Three-layer model

Layer 1 (this crate): contract observation operations on the wire.
Layer 2 (daemon): component-local `RouterCommand` records the daemon
executes (`ReadRouterSummary`, `ReadMessageTrace`, `ReadChannelState`,
`RegisterActor`, `InstallStructuralChannel`).
Layer 3 (observation): payloadless Sema class labels (`Match`,
`Subscribe`, `Retract`) computed daemon-side for cross-component
introspection.

The contract names the public action at the boundary; the daemon decides
what internal work and Sema class label each action maps to. Sema
classification never appears on the wire.

## Constraints

- This crate carries only typed wire vocabulary, NOTA codecs, and
  round-trip witnesses.
- No runtime code: no actors, no tokio, no socket binding, no redb, no
  routing or adjudication policy logic.
- Contract types derive NOTA in this crate. Clients do not carry shadow
  types that re-derive the text surface.
- Every operation and reply variant round-trips through both rkyv frames
  and NOTA text; witnesses live in `tests/round_trip.rs` and
  `examples/canonical.nota`.
- Manager-written router bootstrap uses router-owned typed vocabulary,
  not duplicated private records in `persona`.
- Wire dependency pins use named branches or tags, not raw revision
  hashes.

## Non-ownership

This crate does not own:

- `router` daemon runtime, actors, or component lifecycle;
- `router.redb` or any storage tables, channel state, or delivery logs;
- socket binding, transport, reconnect, or version handshake policy;
- meta channel-policy orders (those live in `meta-signal-router`);
- message ingress records (those live in `signal-message`);
- NOTA projection policy or surface (CLI formatting, audit wrapping,
  introspection-envelope composition).

## See also

- `ARCHITECTURE.md` — detailed channel shape, per-operation vocabulary,
  closed-enum discipline, and the three-layer migration in progress.
- `../router/INTENT.md` — daemon-side intent (schema-driven planes,
  actors, state).
- `../meta-signal-router/INTENT.md` — meta router policy contract.
- `primary/skills/contract-repo.md` — contract repo discipline and
  naming rules.
- `primary/skills/component-triad.md` — repo triad structure and wire
  layers.

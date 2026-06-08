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
`router.sema`. It also carries the manager-written router bootstrap
vocabulary the daemon consumes at startup. `schema/lib.schema` is the
authored source of the contract; `src/schema/lib.rs` is the checked-in
generated Rust surface that publishes `Input`, `Output`, frames, typed
payload records, and codecs. Meta channel-policy orders — grants,
extensions, revocations, adjudication denials — stay in
`meta-signal-router`, called by Orchestrate; runtime actors, sockets,
storage, and routing logic live in `router`.

## The channel shape

The router observation channel carries:

- **Requests:** `Summary`, `MessageTrace`, and `ChannelState` read
  operations, each carrying a typed `*Query` payload. Future
  streaming uses the standardized observer hook persona components
  carry.
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
operations into component-local Nexus commands and SEMA reads or writes.
Database-action classification never crosses this public wire.

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

- Operation roots are contract-local operation heads, not the Sema class word
  `Match`. The six database-action classification words must not appear as
  request roots on this wire.
- Reply success variants name the concrete observation shape returned.
- Payload record names drop the redundant `Router*` prefix where the
  crate namespace already supplies "router."

## Daemon lowering boundary

The contract names the public action at the boundary. The daemon decides what
internal work, durable read, durable write, effect, rejection, or reply each
action becomes. Public contracts do not mirror `Assert`, `Mutate`, `Retract`,
`Match`, `Subscribe`, or `Validate`, and this crate does not depend on
`signal-sema`.

## Constraints

- This crate carries only typed wire vocabulary, generated codecs, and
  round-trip witnesses.
- No runtime code: no actors, no tokio, no socket binding, no storage, no
  routing or adjudication policy logic.
- Contract types derive their NOTA text surface in this crate when the
  `nota-text` feature is enabled. Clients do not carry shadow types that
  re-derive that human-edge surface.
- Every operation and reply variant round-trips through rkyv frames by
  default and through NOTA text under `nota-text`; witnesses live in
  `tests/round_trip.rs` and `examples/canonical.nota`.
- Manager-written router bootstrap uses router-owned typed vocabulary,
  not duplicated private records in `persona`.
- Wire dependency pins use named branches or tags, not raw revision
  hashes.

## Non-ownership

This crate does not own:

- `router` daemon runtime, actors, or component lifecycle;
- `router.sema` or any storage tables, channel state, or delivery logs;
- socket binding, transport, reconnect, or version handshake policy;
- meta channel-policy orders (those live in `meta-signal-router`);
- message ingress records (those live in `signal-message`);
- NOTA projection policy or surface (CLI formatting, audit wrapping,
  introspection-envelope composition).

## See also

- `ARCHITECTURE.md` — detailed channel shape, per-operation vocabulary,
  closed-enum discipline, and the daemon lowering boundary.
- `../router/INTENT.md` — daemon-side intent (schema-driven planes,
  actors, state).
- `../meta-signal-router/INTENT.md` — meta router policy contract.
- `primary/skills/contract-repo.md` — contract repo discipline and
  naming rules.
- `primary/skills/component-triad.md` — repo triad structure and wire
  layers.

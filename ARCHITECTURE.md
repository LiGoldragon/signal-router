# signal-router — architecture

*Signal contract for Persona router-owned observations and relations.*

## 0 · TL;DR

`signal-router` is the typed contract for the router's
component-owned wire vocabulary. It carries the observation channel
`introspect` uses to ask the router what happened to a message,
a channel, or an engine. It also carries the manager-written router
bootstrap vocabulary consumed by `router` at daemon startup, and the
standardized router-to-router forwarding protocol. That protocol is a
router-owned envelope around payload-blind contract objects: Router reads
routing/authentication metadata and forwards opaque rkyv octets without
decoding the inner contract.

Meta channel-policy orders are not part of this ordinary
observation contract. Grants, extensions, revocations, and
adjudication denials live in `meta-signal-router`, the
router's policy signal, called by Orchestrate.
Mind decides at the cognitive level and orders Orchestrate first; it
does not call Router's meta signal directly.

## Wire operation heads

The public wire carries only bare contract-local operation heads. The three
router observation reads are `Summary`, `MessageTrace`, and `ChannelState`. The
router-to-router forwarding relation adds `ForwardMessage` and
`SubmitRoutedObjects`, with the reply pairs `ForwardAccepted` /
`ForwardRefused` and `RoutedObjectsAccepted` / `RoutedObjectsRefused`. The peer
session handshake adds `SessionClientHello`, `SessionClientProof`, and
`SessionData`; actor registration adds `RegisterActor`. Durable read/write
classification is daemon-side only.

## The stack

`ethos/signal.ethos` is the one schema authority. `build.rs` actualizes it
through `ethos-zero` and asserts the checked-in Rust projection in
`src/generated/signal.rs` equals a fresh generation, so the committed code is
the authored schema and nothing else. `src/lib.rs` re-exports that projection
and adds the frame surface: `Signal<T>`, `Signalizable`, `ByteViewable`,
`Restorable`.

The crate depends on `rkyv` for the archive and, under the `datom` feature, on
`protos` and `datom-codec` for the text edge. It carries no envelope crate, no
build-time bootstrap codegen, and no contract-to-contract dependency.

**One request is one frame.** A `Query` is the rkyv archive of one request; a
`Response` is the rkyv archive of one reply. The contract carries no exchange
identifier, no lane, no epoch, no route code and no short header: the `Query`
and `Response` heads are the discrimination, and the connection is the
correlation. The byte layer — a four-byte big-endian length prefix — belongs to
the transport, not to this contract.

**Bootstrap is a document, not a channel.** `RouterBootstrapDocument` is a
vector of `RouterBootstrapOperation`; the manager writes it as Datom text and
`router` actualizes it at startup. It is not a request/reply surface.

Closed enums on the wire; positive names for "entity not in store" cases; one
reply variant per concrete observation shape. Slot-lookup miss is a distinct
`MessageTraceMissing` reply variant, not a sentinel status inside
`RouterMessageTrace`. Channel absence is the positive
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

- `Query` / `Response` (closed wire enums).
- `RouterBootstrapDocument` / `RouterBootstrapOperation`.
- Bootstrap operation records:
  - `RegisterActor` — now carries `home (Optional RemoteRouterIdentity)`:
    `None` ⇒ a local actor (harness-registry delivery); `Some(peer)` ⇒ the
    actor lives behind that remote router, so the local router records it in
    the remote-route table. This is how a router learns a recipient's host —
    the production source for remote-route resolution.
  - `GrantDirectMessage`
  - `InstallStructuralChannels`
  - `RegisterRemoteRouter` — deploy-time peer manifest line
    (`RemoteRouterIdentity` → `TailnetAddress`).
- Bootstrap actor endpoint records:
  - `ActorIdentifier`
  - `Actor`
  - `EndpointTransport`
  - `EndpointKind` (now including `RemoteRouter`: for that kind the
    transport `target` is a `TailnetAddress` literal and `auxiliary` a
    `RemoteRouterIdentity` — one address model, not a parallel one).
- Router-to-router forwarding surface:
  - Addressing nouns `TailnetAddress`, `RemoteRouterIdentity`,
    `HostName`, plus the self-contained `TimestampNanos` and
    `ReplayNonce`.
  - `ForwardMessage(RouterForwardRequest)` request, carrying a
    self-contained `ForwardedMessagePayload`, a `RouterPeerAttestation`,
    the first-class `ForwardMarker` loop guard, a `ReplayNonce`, and a
    `TimestampNanos`.
  - `RoutedContractObject` — contract name, operation name, declared byte
    size, and opaque contract payload octets. This is how a mirror
    `NotifyObject` or later component-owned object rides inside the router
    protocol while remaining owned by its own contract.
  - Reply pair `ForwardAccepted(RouterForwardAccepted)` (the minted
    delivery slot) / `ForwardRefused(RouterForwardRefused)` with closed
    `RouterForwardRefusalReason`.
  - Closed `SignatureScheme` mirroring criome's scheme set.
- `RouterDaemonConfiguration` extended with `tailnet_listen_address`
  (Optional), `router_identity`, and `criome_socket_path` (Optional).
- `RouterSummaryQuery` / `RouterSummary`.
- `RouterMessageTraceQuery` and the **two-variant reply split**:
  - `Response::MessageTrace(RouterMessageTrace)` — slot present;
    `status` is a closed `RouterDeliveryStatus`.
  - `Response::MessageTraceMissing(RouterMessageTraceMissing)` —
    slot not in store. The split keeps the inner status enum closed.
- `RouterChannelStateQuery` / `RouterChannelState` /
  `RouterChannelStatus`. The "slot not in store" case is the positive
  `Missing` variant.
- `RouterObservationUnimplemented` + closed
  `RouterObservationUnimplementedReason`.
- Contract-local verbs declared as query and reply heads in `ethos/signal.ethos`;
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
  | ForwardedRemote    -- the message was handed to a peer router

RouterChannelStatus
  | Installed
  | Missing            -- positive name for "no slot in store"
  | Disabled

RouterObservationUnimplementedReason
  | NotInPrototypeScope
  | RouterStoreUnavailable
  | MessageTraceUnavailable

ForwardMarker
  | Origin             -- an originating submission, may resolve remotely
  | Forwarded          -- already arrived via a forward; never re-resolved

RouterForwardRefusalReason
  | UnknownPeer
  | AttestationInvalid
  | ReplayDetected
  | ClockSkew
  | RecipientUnknown
  | ChannelUnauthorized
  | AlreadyForwarded

SignatureScheme
  | Bls12_381MinPk
  | Bls12_381MinSig
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

## 4a · Router-to-router forwarding

This contract carries the router↔router forwarding relation — the wire
half of "networking through the router" (Spirit `wckt`, comms
architecture; Spirit `ermr`, cross-system trust root). Milestone 1 is the
contract only; the daemon's tailnet ingress, outbound peer client, remote
registry, attestation verification, and replay window are separate
milestones in `router`.

**Self-contained attestation (the decision).** A networked router cannot
rely on the kernel's `SO_PEERCRED` local vouching, which dies at the TCP
hop. Instead `RouterForwardRequest` carries a `RouterPeerAttestation` —
signer, scheme, public key, signature, content digest, the router's own
forward issue time, replay nonce, and the criome attestation issue time
(the timestamp criome server-stamped into the BLS-signed attestation, so
the receiver can reconstruct criome's exact signed preimage) — that
**mirrors what criome produces without depending on `signal-criome`.** The crate holds a self-contained-vocabulary policy and
carries no contract→contract dependency; the daemon projects this record
to/from criome's `Attestation` at the boundary and delegates all signing
and verification to its local criome daemon. The router never holds keys
or verifies signatures itself (`wckt`: tailnet encrypts the bytes, BLS
authenticates the identity — two separate concerns). The closed
`SignatureScheme` mirrors criome's scheme set for the same reason.

**Self-contained payload (the dependency decision).** `signal-router` carries
no contract→contract dependency at all: it declares its own `Signal<T>` frame
surface and imports no sibling contract. Rather than import `signal-message`'s
stamped submission (which would break self-containment and the
"buildable in isolation" milestone-1 constraint), the forwarded message
travels as a self-contained `ForwardedMessagePayload` (from/to actor,
body, attachments). The daemon projects it into its stamped-submission
ledger entry on receipt.

**First-class loop guard.** `ForwardMarker` is a first-class field, not a
risk footnote. The inbound handler sets it deterministically: an `Origin`
submission may resolve to a remote route, but a `Forwarded` message is
delivered-local-or-parked only and must never be re-resolved remotely
(refused `AlreadyForwarded` if it would be). The guard keys on the marker,
independent of the criome-derived origin identity.

**Addressing.** `TailnetAddress` is the dialed IPv6 literal + port;
`RemoteRouterIdentity` is the peer's stable criome `PrincipalName`.
Addresses re-home, identity does not: peers are routed by identity and
dialed by current address. `RegisterRemoteRouter` is the deploy-time peer
manifest of *which peers exist* (bootstrap-as-config, not runtime
discovery); `RegisterActor.home` is the deploy-time source of *which
recipient lives behind which peer* — the input to remote-route resolution.

**Config.** `tailnet_listen_address` is `Optional` — `Some` ⇒ the daemon
binds a TCP forwarding tier; `None` ⇒ a single-host router stays
local-only. `router_identity` is this router's own stable identity;
`criome_socket_path` (`Optional`) is the local criome daemon for
attestation.

## 5 · Constraints

| Constraint | Witness |
|---|---|
| Router observations have a router-owned contract home. | This crate exists; the central introspection contract does not define router rows. |
| The committed Rust projection is the authored schema and nothing else. | `build.rs` asserts `src/generated/signal.rs` equals a fresh `ethos-zero` generation of `ethos/signal.ethos`. |
| Every request and reply travels as one rkyv Signal frame. | `tests/contract.rs::queries_round_trip_through_received_bytes` and `responses_round_trip_through_received_bytes` restore each head from fresh peer bytes. |
| Every contract head has a canonical Datom text. | `tests/contract.rs::every_canonical_datom_line_actualizes_into_a_contract_head` actualizes every line of `examples/canonical.datom`. |
| A bare head stays a bare head. | `tests/contract.rs::bare_heads_cross_the_wire_as_bare_heads` carries `RouterObservationScope` and `RouterObservationUnimplementedReason` tags over the wire. |
| Peer-supplied Datom text is bounded before it is read. | `tests/contract.rs::peer_text_beyond_the_extent_is_refused`; the reader budget is one mebibyte of extent and 256 levels of descent. |
| Router forwarding carries a contract-owned object without knowing the inner contract. | `RoutedContractObject` carries a contract name, an operation name, a declared size, and opaque octets; the forwarding round trips leave the octets unchanged. |
| Manager-written router bootstrap uses router-owned typed vocabulary. | `RouterBootstrapDocument` and `RouterBootstrapOperation` live in this crate and round-trip as Datom text. |
| Meta router channel policy orders stay out of this ordinary contract. | `meta-signal-router` owns `Grant`, `Extend`, `Revoke`, and `Deny`. |
| Runtime code stays out of the contract. | No Kameo, Tokio, socket, or storage code; no build-time codegen beyond the ethos assertion. |

## 6 · Layout

```text
ethos/
└── signal.ethos          — the schema authority
src/
├── lib.rs                — re-export plus the Signal frame surface
└── generated/signal.rs   — the checked-in Ethos Zero projection
examples/
└── canonical.datom       — one Datom line per contract head
tests/
└── contract.rs           — frame round trips, Datom round trips, canonical lines
```

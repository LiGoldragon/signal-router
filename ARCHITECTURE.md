# signal-router — architecture

## Center

This Interface is the vocabulary of Router’s ordinary relation. It says what a
Router can be asked, what it can answer, how actors and peer routes are named,
and how opaque contract-owned objects cross a Router boundary. It owns no
daemon mechanics.

Ethos is the readable authority. Humans, agents, harnesses, and GUIs see the
meaningful names there and in Dotos; Rust sees encoded identities only.

## Relations

The request root contains nine closed operations:

- observation: `Summary`, `MessageTrace`, and `ChannelState`;
- forwarding: `ForwardMessage` and `SubmitRoutedObjects`;
- peer session: `SessionClientHello`, `SessionClientProof`, and `SessionData`;
- local runtime registration: `RegisterActor`.

The reply root contains fifteen closed outcomes: the observation answers and
typed absence, forward and routed-object acceptance/refusal, peer-session
handshake/data outcomes, actor registration outcomes, and an explicitly typed
unimplemented observation answer.

The Rust coordinates of both roots and every payload are encoded. Route enums
retain readable operation names because routing behavior is producer-owned,
not a copied structural naming surface.

## Owned vocabulary

Observation vocabulary keeps absence closed and positive. A missing message
trace is a distinct reply; channel absence is `Missing`, not `Unknown`.

Bootstrap vocabulary names actors, endpoints, direct-message grants,
structural-channel installation, peer routers, and the ordered bootstrap
document. Runtime actor registration accepts only local actors; remote homes
belong to bootstrap route discovery.

Forwarding is payload-blind. `RoutedContractObject` carries contract name,
operation, declared size, and opaque octets. Router authenticates and routes
the envelope without decoding the owned object. `ForwardMarker` prevents a
received forward from being re-resolved onto another remote route.

Peer forwarding carries a self-contained Criome-rooted attestation and a
three-message ephemeral-key handshake followed by encrypted session data.
Router never owns signing keys or signature verification; it projects the
attestation to its local Criome relation at the daemon boundary.

Configuration names working, owner-meta, supervision, store, bootstrap, peer,
and Criome reachability. `WirePath` remains contract-local because it types
both socket and ordinary file paths; it is not a duplicate of the narrower
shared `StandardSocket`. The unused local `HostName` spelling was removed.

## Authority and projection

`ethos/interface.ethos` is the only schema source. It is a strict
`Interface.{1 0 0}` with empty bootstrap role sections; the encoded
`RouterRequest` and `RouterReply` declarations are seated into ordinary Signal
behavior by the producer-owned Rust layer.

`src/bootstrap_manifest.rs` carries explicit authority identity, revision,
grammar identities, fixed vocabulary, declarations, variants, and canonical
ordering. `build.rs` verifies the authorized transaction, checks the generated
Rust projection, and publishes the owned Ethos directory through Cargo
metadata.

`src/schema/lib/generated.rs` is structural projection only.
`src/schema/lib/behavior.rs` owns structural wire conversion, Dotos, rkyv,
route seating, and contract binding 7 revision 2. It creates no readable type
aliases.

## Dependency topology

The ordinary Router Interface depends at runtime only on `signal-frame` and
optional Dotos. It deliberately does not import component contracts. No
`signal-standard` identity is semantically present: the one overlapping
spelling on the retired source was unused and was deleted. `meta-signal-router`
depends on this producer and imports only the ordinary types it actually uses.

## Constraints

- Wire enums are closed; no catch-all or `Unknown` state.
- Every request and reply variant has a Dotos, rkyv, and bound-frame witness.
- Routed objects remain opaque to Router.
- A forwarded message is never forwarded remotely again.
- Runtime policy, actors, storage, sockets, and cryptography remain in
  `router`.
- Meta channel-policy orders remain in `meta-signal-router`.
- All dependency revisions are exact and producer-owned sources match Cargo
  metadata.
- No second schema language, copied readable aliases, or retired emitter
  machinery may reappear.

## Consumers

`router` lowers the encoded ordinary roots into its actor and storage planes.
`introspect` consumes observation replies. Orchestrate uses actor-registration
and bootstrap vocabulary. The meta producer imports Router configuration and
policy payloads from this exact producer head.

## Evolution

New relations extend the Ethos Interface with explicitly minted seats and
exhaustive witnesses. Schema decisions must not assume Rust, LLVM, or the
current operating system as a permanent substrate.

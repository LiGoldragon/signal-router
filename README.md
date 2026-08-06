# signal-router

The ordinary Router Interface: observation, actor registration, bootstrap,
payload-blind routed-object forwarding, and authenticated peer-session
vocabulary.

`ethos/interface.ethos` is the sole schema authority. The strict role-free
`Interface.{1 0 0}` is verified against explicit producer-owned identity
seats, then projected to encoded-only Rust. The producer-owned behavior layer
supplies Dotos, rkyv, and the bound Signal frame while those slices remain
outside the bootstrap language.

The crate is self-contained apart from `signal-frame`; the Router daemon owns
routing, authentication policy, storage, sockets, and actors.

Regenerate the checked projection with:

```sh
SIGNAL_ROUTER_UPDATE_INTERFACE_ARTIFACTS=1 cargo build
```

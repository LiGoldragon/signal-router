//! Ordinary Signal contract for the `router` surface.
//!
//! The contract carries the peer-callable router observations (summary,
//! message trace, channel state), the encrypted authenticated peer session
//! handshake, cross-host message forwarding and routed contract objects,
//! actor registration, and the router-owned bootstrap and daemon
//! configuration records. Routing itself, storage, sockets, and policy live
//! in `router`; meta policy orders live in `meta-signal-router`.
//!
//! `ethos/signal.ethos` is the schema authority; `build.rs` checks the
//! checked-in Rust projection in `src/generated/signal.rs` against a fresh
//! generation.
//!
//! # The wire
//!
//! One request is one [`Signal`] frame carrying the rkyv archive of [`Query`];
//! one reply is one frame of [`Response`]. The contract carries no envelope,
//! no exchange identifier, no lane, no epoch, and no route code: the `Query`
//! and `Response` heads are the discrimination, and the connection is the
//! correlation.
//!
//! The portable rkyv frame and its three kinds come from `signal` and are
//! re-exported here, so a router frame is the same Rust type as every other
//! contract's frame.

pub mod generated;
pub use generated::signal::*;

pub use signal::{ByteViewable, Restorable, Signal, Signalizable};

/// The authored Ethos source of this contract.
pub const ETHOS: &str = include_str!("../ethos/signal.ethos");
/// The Rust projection generated from [`ETHOS`].
pub const ETHOS_RUST: &str = include_str!("generated/signal.rs");

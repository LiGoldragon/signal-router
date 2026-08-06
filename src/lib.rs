//! Authority-verified ordinary Router Interface.
//!
//! `ethos/interface.ethos` is the canonical textual projection. Its checked
//! Rust projection carries encoded structural identities; role seating,
//! structural behavior, and the Signal frame boundary remain producer-owned
//! until the language expresses those slices directly.

pub mod bootstrap_manifest;
pub mod schema;

pub use schema::lib::*;

pub const ROUTER_INTERFACE_SOURCE: &str = include_str!("../ethos/interface.ethos");
pub const ROUTER_INTERFACE_RUST: &str = include_str!("schema/lib/generated.rs");

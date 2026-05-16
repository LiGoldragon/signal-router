//! Signal contract for Persona router observations and relations.
//!
//! This crate gives `persona-router` a typed contract home. It begins
//! with observation records needed by `persona-introspect`; operational
//! router relations can land here as they are extracted.

use nota_codec::{NotaEnum, NotaRecord, NotaTransparent};
use rkyv::{Archive, Deserialize as RkyvDeserialize, Serialize as RkyvSerialize};
use signal_core::signal_channel;
use signal_persona::{SocketMode, WirePath};
use signal_persona_auth::{ChannelId, EngineId, OwnerIdentity};
use signal_persona_message::MessageSlot;

#[derive(
    Archive, RkyvSerialize, RkyvDeserialize, NotaTransparent, Debug, Clone, PartialEq, Eq, Hash,
)]
pub struct RouterObservationId(String);

impl RouterObservationId {
    pub fn new(value: impl Into<String>) -> Self {
        Self(value.into())
    }

    pub fn as_str(&self) -> &str {
        self.0.as_str()
    }
}

#[derive(
    Archive, RkyvSerialize, RkyvDeserialize, NotaEnum, Debug, Clone, Copy, PartialEq, Eq, Hash,
)]
pub enum RouterObservationScope {
    Summary,
    MessageTrace,
    ChannelState,
}

#[derive(Archive, RkyvSerialize, RkyvDeserialize, NotaRecord, Debug, Clone, PartialEq, Eq)]
pub struct RouterSummaryQuery {
    pub engine: EngineId,
}

#[derive(Archive, RkyvSerialize, RkyvDeserialize, NotaRecord, Debug, Clone, PartialEq, Eq)]
pub struct RouterMessageTraceQuery {
    pub engine: EngineId,
    pub message_slot: MessageSlot,
}

#[derive(Archive, RkyvSerialize, RkyvDeserialize, NotaRecord, Debug, Clone, PartialEq, Eq)]
pub struct RouterChannelStateQuery {
    pub engine: EngineId,
    pub channel: ChannelId,
}

#[derive(Archive, RkyvSerialize, RkyvDeserialize, NotaRecord, Debug, Clone, PartialEq, Eq)]
pub struct RouterSummary {
    pub engine: EngineId,
    pub accepted_messages: u64,
    pub routed_messages: u64,
    pub deferred_messages: u64,
    pub failed_messages: u64,
}

/// Observation of a known message slot. The slot was found in the router's
/// store; `status` is the closed-enum delivery state derived from its trace.
/// When the slot itself is not in the store, the reply is
/// `RouterReply::MessageTraceMissing(RouterMessageTraceMissing)` — *not* this
/// record with a placeholder status. The closed-enum rule applies: a
/// `Unknown` variant would conflate "slot not observed" with "slot observed,
/// state unrepresentable," which are two different facts.
#[derive(Archive, RkyvSerialize, RkyvDeserialize, NotaRecord, Debug, Clone, PartialEq, Eq)]
pub struct RouterMessageTrace {
    pub engine: EngineId,
    pub message_slot: MessageSlot,
    pub status: RouterDeliveryStatus,
}

/// Slot lookup failed: the message slot is not present in the router's
/// store. Distinct from `RouterMessageTrace` so callers pattern-match on
/// presence vs absence without inspecting a sentinel status variant.
#[derive(Archive, RkyvSerialize, RkyvDeserialize, NotaRecord, Debug, Clone, PartialEq, Eq)]
pub struct RouterMessageTraceMissing {
    pub engine: EngineId,
    pub message_slot: MessageSlot,
}

#[derive(
    Archive, RkyvSerialize, RkyvDeserialize, NotaEnum, Debug, Clone, Copy, PartialEq, Eq, Hash,
)]
pub enum RouterDeliveryStatus {
    Accepted,
    Routed,
    Delivered,
    Deferred,
    Failed,
}

#[derive(Archive, RkyvSerialize, RkyvDeserialize, NotaRecord, Debug, Clone, PartialEq, Eq)]
pub struct RouterChannelState {
    pub engine: EngineId,
    pub channel: ChannelId,
    pub status: RouterChannelStatus,
}

#[derive(
    Archive, RkyvSerialize, RkyvDeserialize, NotaEnum, Debug, Clone, Copy, PartialEq, Eq, Hash,
)]
pub enum RouterChannelStatus {
    Installed,
    Missing,
    Disabled,
}

#[derive(Archive, RkyvSerialize, RkyvDeserialize, NotaRecord, Debug, Clone, PartialEq, Eq)]
pub struct RouterObservationUnimplemented {
    pub scope: RouterObservationScope,
    pub reason: RouterObservationUnimplementedReason,
}

#[derive(
    Archive, RkyvSerialize, RkyvDeserialize, NotaEnum, Debug, Clone, Copy, PartialEq, Eq, Hash,
)]
pub enum RouterObservationUnimplementedReason {
    NotInPrototypeScope,
    RouterStoreUnavailable,
    MessageTraceUnavailable,
}

signal_channel! {
    channel Router {
        request RouterRequest {
            Match Summary(RouterSummaryQuery),
            Match MessageTrace(RouterMessageTraceQuery),
            Match ChannelState(RouterChannelStateQuery),
        }

        reply RouterReply {
            Summary(RouterSummary),
            MessageTrace(RouterMessageTrace),
            MessageTraceMissing(RouterMessageTraceMissing),
            ChannelState(RouterChannelState),
            Unimplemented(RouterObservationUnimplemented),
        }
    }
}

// ─── Daemon configuration ──────────────────────────────────
//
// Typed startup configuration for `persona-router-daemon`. The
// persona manager writes one of these (NOTA or rkyv) to a state-dir
// path and passes that path as argv. The daemon decodes through
// `nota_config::ConfigurationSource::from_argv()?.decode()?` and
// runs with the resulting record. No environment variables on the
// production launch path.

/// Startup configuration for `persona-router-daemon`.
///
/// Replaces the previous `--socket`, `--store`, `--bootstrap`,
/// `PERSONA_SOCKET_MODE`, `PERSONA_SUPERVISION_SOCKET_PATH`, and
/// `PERSONA_SUPERVISION_SOCKET_MODE` argv/environment-variable
/// surface.
#[derive(Archive, RkyvSerialize, RkyvDeserialize, NotaRecord, Debug, Clone, PartialEq, Eq)]
pub struct RouterDaemonConfiguration {
    /// Where the daemon binds its router Unix socket.
    pub router_socket_path: WirePath,
    /// chmod applied to the router socket after bind.
    pub router_socket_mode: SocketMode,
    /// Where the daemon binds its supervision Unix socket.
    pub supervision_socket_path: WirePath,
    /// chmod applied to the supervision socket after bind.
    pub supervision_socket_mode: SocketMode,
    /// Path to the router daemon's redb store file.
    pub store_path: WirePath,
    /// Optional bootstrap-record path the daemon applies at startup.
    pub bootstrap_path: Option<WirePath>,
    /// The engine owner identity passed to the router daemon.
    pub owner_identity: OwnerIdentity,
}

nota_config::impl_rkyv_configuration!(RouterDaemonConfiguration);

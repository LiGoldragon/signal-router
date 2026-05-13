//! Signal contract for Persona router observations and relations.
//!
//! This crate gives `persona-router` a typed contract home. It begins
//! with observation records needed by `persona-introspect`; operational
//! router relations can land here as they are extracted.

use nota_codec::{NotaEnum, NotaRecord, NotaTransparent};
use rkyv::{Archive, Deserialize as RkyvDeserialize, Serialize as RkyvSerialize};
use signal_core::signal_channel;
use signal_persona_auth::{ChannelId, EngineId};
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

#[derive(Archive, RkyvSerialize, RkyvDeserialize, NotaRecord, Debug, Clone, PartialEq, Eq)]
pub struct RouterMessageTrace {
    pub engine: EngineId,
    pub message_slot: MessageSlot,
    pub status: RouterDeliveryStatus,
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
    Unknown,
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
    Unknown,
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
    request RouterRequest {
        Summary(RouterSummaryQuery),
        MessageTrace(RouterMessageTraceQuery),
        ChannelState(RouterChannelStateQuery),
    }

    reply RouterReply {
        Summary(RouterSummary),
        MessageTrace(RouterMessageTrace),
        ChannelState(RouterChannelState),
        Unimplemented(RouterObservationUnimplemented),
    }
}

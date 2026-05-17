//! Signal contract for Persona router observations and relations.
//!
//! This crate gives `persona-router` a typed contract home. It begins
//! with observation records needed by `persona-introspect`; operational
//! router relations can land here as they are extracted.

use nota_codec::{
    Decoder, Encoder, NotaDecode, NotaEncode, NotaEnum, NotaRecord, NotaSum, NotaTransparent,
};
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
    Archive, RkyvSerialize, RkyvDeserialize, NotaTransparent, Debug, Clone, PartialEq, Eq, Hash,
)]
pub struct ActorId(String);

impl ActorId {
    pub fn new(value: impl Into<String>) -> Self {
        Self(value.into())
    }

    pub fn as_str(&self) -> &str {
        self.0.as_str()
    }
}

#[derive(Archive, RkyvSerialize, RkyvDeserialize, NotaRecord, Debug, Clone, PartialEq, Eq)]
pub struct Actor {
    pub name: ActorId,
    pub process: u32,
    pub endpoint: Option<EndpointTransport>,
}

impl Actor {
    pub fn new(name: ActorId, process: u32, endpoint: Option<EndpointTransport>) -> Self {
        Self {
            name,
            process,
            endpoint,
        }
    }
}

#[derive(Archive, RkyvSerialize, RkyvDeserialize, NotaRecord, Debug, Clone, PartialEq, Eq)]
pub struct EndpointTransport {
    pub kind: EndpointKind,
    pub target: String,
    pub auxiliary: Option<String>,
}

impl EndpointTransport {
    pub fn new(kind: EndpointKind, target: impl Into<String>, auxiliary: Option<String>) -> Self {
        Self {
            kind,
            target: target.into(),
            auxiliary,
        }
    }
}

#[derive(Archive, RkyvSerialize, RkyvDeserialize, NotaEnum, Debug, Clone, Copy, PartialEq, Eq)]
pub enum EndpointKind {
    Human,
    HarnessSocket,
    PtySocket,
}

#[derive(Archive, RkyvSerialize, RkyvDeserialize, NotaRecord, Debug, Clone, PartialEq, Eq)]
pub struct RegisterActor {
    pub actor: Actor,
}

impl RegisterActor {
    pub fn new(actor: Actor) -> Self {
        Self { actor }
    }
}

#[derive(Archive, RkyvSerialize, RkyvDeserialize, NotaRecord, Debug, Clone, PartialEq, Eq)]
pub struct GrantDirectMessage {
    pub from: ActorId,
    pub to: ActorId,
}

impl GrantDirectMessage {
    pub fn new(from: ActorId, to: ActorId) -> Self {
        Self { from, to }
    }
}

#[derive(Archive, RkyvSerialize, RkyvDeserialize, NotaRecord, Debug, Clone, PartialEq, Eq)]
pub struct InstallStructuralChannels {
    pub requester: ActorId,
}

impl InstallStructuralChannels {
    pub fn new(requester: ActorId) -> Self {
        Self { requester }
    }
}

#[derive(Archive, RkyvSerialize, RkyvDeserialize, NotaSum, Debug, Clone, PartialEq, Eq)]
pub enum RouterBootstrapOperation {
    RegisterActor(RegisterActor),
    GrantDirectMessage(GrantDirectMessage),
    InstallStructuralChannels(InstallStructuralChannels),
}

impl RouterBootstrapOperation {
    pub fn from_nota(text: &str) -> nota_codec::Result<Self> {
        let mut decoder = Decoder::new(text);
        let operation = Self::decode(&mut decoder)?;
        Self::expect_end(&mut decoder)?;
        Ok(operation)
    }

    pub fn to_nota(&self) -> nota_codec::Result<String> {
        let mut encoder = Encoder::new();
        self.encode(&mut encoder)?;
        Ok(encoder.into_string())
    }

    fn expect_end(decoder: &mut Decoder<'_>) -> nota_codec::Result<()> {
        if let Some(token) = decoder.peek_token()? {
            return Err(nota_codec::Error::UnexpectedToken {
                expected: "end of input",
                got: token,
            });
        }
        Ok(())
    }
}

#[derive(Archive, RkyvSerialize, RkyvDeserialize, NotaRecord, Debug, Clone, PartialEq, Eq)]
pub struct RouterBootstrapDocument {
    pub operations: Vec<RouterBootstrapOperation>,
}

impl RouterBootstrapDocument {
    pub fn new(operations: Vec<RouterBootstrapOperation>) -> Self {
        Self { operations }
    }

    pub fn operations(&self) -> &[RouterBootstrapOperation] {
        self.operations.as_slice()
    }

    pub fn into_operations(self) -> Vec<RouterBootstrapOperation> {
        self.operations
    }

    pub fn from_nota_lines(text: &str) -> nota_codec::Result<Self> {
        let mut operations = Vec::new();
        for line in text.lines().map(str::trim).filter(|line| !line.is_empty()) {
            operations.push(RouterBootstrapOperation::from_nota(line)?);
        }
        Ok(Self::new(operations))
    }

    pub fn to_nota_lines(&self) -> nota_codec::Result<String> {
        let mut text = String::new();
        for operation in &self.operations {
            text.push_str(operation.to_nota()?.as_str());
            text.push('\n');
        }
        Ok(text)
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

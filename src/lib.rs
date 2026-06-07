//! Signal contract for Persona router observations and relations.
//!
//! This crate gives `router` a typed contract home. It begins
//! with observation records needed by `introspect`; operational
//! router relations can land here as they are extracted.

use nota_next::{Block, Delimiter, NotaBlock, NotaDecode, NotaDecodeError, NotaEncode, NotaSource};
use rkyv::{Archive, Deserialize as RkyvDeserialize, Serialize as RkyvSerialize};
use signal_engine_management::{SocketMode, WirePath};
use signal_frame::signal_channel;
use signal_message::MessageSlot;
use signal_persona_origin::{ChannelIdentifier, EngineIdentifier, OwnerIdentity};

#[derive(
    Archive,
    RkyvSerialize,
    RkyvDeserialize,
    NotaEncode,
    NotaDecode,
    Debug,
    Clone,
    PartialEq,
    Eq,
    Hash,
)]
pub struct ObservationIdentifier(String);

impl ObservationIdentifier {
    pub fn new(value: impl Into<String>) -> Self {
        Self(value.into())
    }

    pub fn as_str(&self) -> &str {
        self.0.as_str()
    }
}

#[derive(
    Archive,
    RkyvSerialize,
    RkyvDeserialize,
    NotaEncode,
    NotaDecode,
    Debug,
    Clone,
    PartialEq,
    Eq,
    Hash,
)]
pub struct ActorIdentifier(String);

impl ActorIdentifier {
    pub fn new(value: impl Into<String>) -> Self {
        Self(value.into())
    }

    pub fn as_str(&self) -> &str {
        self.0.as_str()
    }
}

#[derive(Archive, RkyvSerialize, RkyvDeserialize, Debug, Clone, PartialEq, Eq)]
pub struct Actor {
    pub name: ActorIdentifier,
    pub process: u32,
    pub endpoint: Option<EndpointTransport>,
}

impl NotaDecode for Actor {
    fn from_nota_block(block: &Block) -> Result<Self, NotaDecodeError> {
        let fields = NotaBlock::new(block).expect_children(Delimiter::Parenthesis, "Actor", 3)?;
        let process = NotaBlock::new(&fields[1]).parse_integer()?;
        let process = u32::try_from(process).map_err(|_| NotaDecodeError::InvalidInteger {
            value: process.to_string(),
        })?;
        Ok(Self {
            name: ActorIdentifier::from_nota_block(&fields[0])?,
            process,
            endpoint: Option::<EndpointTransport>::from_nota_block(&fields[2])?,
        })
    }
}

impl NotaEncode for Actor {
    fn to_nota(&self) -> String {
        Delimiter::Parenthesis.wrap([
            self.name.to_nota(),
            self.process.to_string(),
            self.endpoint.to_nota(),
        ])
    }
}

impl Actor {
    pub fn new(name: ActorIdentifier, process: u32, endpoint: Option<EndpointTransport>) -> Self {
        Self {
            name,
            process,
            endpoint,
        }
    }
}

#[derive(
    Archive, RkyvSerialize, RkyvDeserialize, NotaEncode, NotaDecode, Debug, Clone, PartialEq, Eq,
)]
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

#[derive(
    Archive,
    RkyvSerialize,
    RkyvDeserialize,
    NotaEncode,
    NotaDecode,
    Debug,
    Clone,
    Copy,
    PartialEq,
    Eq,
)]
pub enum EndpointKind {
    Human,
    HarnessSocket,
    PtySocket,
}

#[derive(
    Archive, RkyvSerialize, RkyvDeserialize, NotaEncode, NotaDecode, Debug, Clone, PartialEq, Eq,
)]
pub struct RegisterActor {
    pub actor: Actor,
}

impl RegisterActor {
    pub fn new(actor: Actor) -> Self {
        Self { actor }
    }
}

#[derive(
    Archive, RkyvSerialize, RkyvDeserialize, NotaEncode, NotaDecode, Debug, Clone, PartialEq, Eq,
)]
pub struct GrantDirectMessage {
    pub from: ActorIdentifier,
    pub to: ActorIdentifier,
}

impl GrantDirectMessage {
    pub fn new(from: ActorIdentifier, to: ActorIdentifier) -> Self {
        Self { from, to }
    }
}

#[derive(
    Archive, RkyvSerialize, RkyvDeserialize, NotaEncode, NotaDecode, Debug, Clone, PartialEq, Eq,
)]
pub struct InstallStructuralChannels {
    pub requester: ActorIdentifier,
}

impl InstallStructuralChannels {
    pub fn new(requester: ActorIdentifier) -> Self {
        Self { requester }
    }
}

#[derive(
    Archive, RkyvSerialize, RkyvDeserialize, NotaEncode, NotaDecode, Debug, Clone, PartialEq, Eq,
)]
pub enum RouterBootstrapOperation {
    RegisterActor(RegisterActor),
    GrantDirectMessage(GrantDirectMessage),
    InstallStructuralChannels(InstallStructuralChannels),
}

impl RouterBootstrapOperation {
    pub fn from_nota(text: &str) -> Result<Self, NotaDecodeError> {
        NotaSource::new(text).parse()
    }

    pub fn to_nota(&self) -> String {
        NotaEncode::to_nota(self)
    }
}

#[derive(
    Archive, RkyvSerialize, RkyvDeserialize, NotaEncode, NotaDecode, Debug, Clone, PartialEq, Eq,
)]
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

    pub fn from_nota_lines(text: &str) -> Result<Self, NotaDecodeError> {
        let mut operations = Vec::new();
        for line in text.lines().map(str::trim).filter(|line| !line.is_empty()) {
            operations.push(RouterBootstrapOperation::from_nota(line)?);
        }
        Ok(Self::new(operations))
    }

    pub fn to_nota_lines(&self) -> String {
        let mut text = String::new();
        for operation in &self.operations {
            text.push_str(operation.to_nota().as_str());
            text.push('\n');
        }
        text
    }
}

#[derive(
    Archive,
    RkyvSerialize,
    RkyvDeserialize,
    NotaEncode,
    NotaDecode,
    Debug,
    Clone,
    Copy,
    PartialEq,
    Eq,
    Hash,
)]
pub enum RouterObservationScope {
    Summary,
    MessageTrace,
    ChannelState,
}

#[derive(
    Archive, RkyvSerialize, RkyvDeserialize, NotaEncode, NotaDecode, Debug, Clone, PartialEq, Eq,
)]
pub struct RouterSummaryQuery {
    pub engine: EngineIdentifier,
}

#[derive(
    Archive, RkyvSerialize, RkyvDeserialize, NotaEncode, NotaDecode, Debug, Clone, PartialEq, Eq,
)]
pub struct RouterMessageTraceQuery {
    pub engine: EngineIdentifier,
    pub message_slot: MessageSlot,
}

#[derive(
    Archive, RkyvSerialize, RkyvDeserialize, NotaEncode, NotaDecode, Debug, Clone, PartialEq, Eq,
)]
pub struct RouterChannelStateQuery {
    pub engine: EngineIdentifier,
    pub channel: ChannelIdentifier,
}

#[derive(
    Archive, RkyvSerialize, RkyvDeserialize, NotaEncode, NotaDecode, Debug, Clone, PartialEq, Eq,
)]
pub struct RouterSummary {
    pub engine: EngineIdentifier,
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
#[derive(
    Archive, RkyvSerialize, RkyvDeserialize, NotaEncode, NotaDecode, Debug, Clone, PartialEq, Eq,
)]
pub struct RouterMessageTrace {
    pub engine: EngineIdentifier,
    pub message_slot: MessageSlot,
    pub status: RouterDeliveryStatus,
}

/// Slot lookup failed: the message slot is not present in the router's
/// store. Distinct from `RouterMessageTrace` so callers pattern-match on
/// presence vs absence without inspecting a sentinel status variant.
#[derive(
    Archive, RkyvSerialize, RkyvDeserialize, NotaEncode, NotaDecode, Debug, Clone, PartialEq, Eq,
)]
pub struct RouterMessageTraceMissing {
    pub engine: EngineIdentifier,
    pub message_slot: MessageSlot,
}

#[derive(
    Archive,
    RkyvSerialize,
    RkyvDeserialize,
    NotaEncode,
    NotaDecode,
    Debug,
    Clone,
    Copy,
    PartialEq,
    Eq,
    Hash,
)]
pub enum RouterDeliveryStatus {
    Accepted,
    Routed,
    Delivered,
    Deferred,
    Failed,
}

#[derive(
    Archive, RkyvSerialize, RkyvDeserialize, NotaEncode, NotaDecode, Debug, Clone, PartialEq, Eq,
)]
pub struct RouterChannelState {
    pub engine: EngineIdentifier,
    pub channel: ChannelIdentifier,
    pub status: RouterChannelStatus,
}

#[derive(
    Archive,
    RkyvSerialize,
    RkyvDeserialize,
    NotaEncode,
    NotaDecode,
    Debug,
    Clone,
    Copy,
    PartialEq,
    Eq,
    Hash,
)]
pub enum RouterChannelStatus {
    Installed,
    Missing,
    Disabled,
}

#[derive(
    Archive, RkyvSerialize, RkyvDeserialize, NotaEncode, NotaDecode, Debug, Clone, PartialEq, Eq,
)]
pub struct RouterObservationUnimplemented {
    pub scope: RouterObservationScope,
    pub reason: RouterObservationUnimplementedReason,
}

#[derive(
    Archive,
    RkyvSerialize,
    RkyvDeserialize,
    NotaEncode,
    NotaDecode,
    Debug,
    Clone,
    Copy,
    PartialEq,
    Eq,
    Hash,
)]
pub enum RouterObservationUnimplementedReason {
    NotInPrototypeScope,
    RouterStoreUnavailable,
    MessageTraceUnavailable,
}

signal_channel! {
    channel Router {
        operation Summary(RouterSummaryQuery),
        operation MessageTrace(RouterMessageTraceQuery),
        operation ChannelState(RouterChannelStateQuery),
    }
    reply RouterReply {
        Summary(RouterSummary),
        MessageTrace(RouterMessageTrace),
        MessageTraceMissing(RouterMessageTraceMissing),
        ChannelState(RouterChannelState),
        Unimplemented(RouterObservationUnimplemented),
    }
}

pub type RouterRequest = Operation;
pub type RouterFrame = Frame;
pub type RouterFrameBody = FrameBody;
pub type RouterRequestBuilder = RequestBuilder;

// ─── Daemon configuration ──────────────────────────────────
//
// Typed startup configuration for `router-daemon`. Human tooling may
// author this record through NOTA, but the live daemon consumes a
// signal-encoded rkyv archive path. The daemon does not parse NOTA.

/// Startup configuration for `router-daemon`.
///
/// Replaces the previous `--socket`, `--store`, `--bootstrap`,
/// `PERSONA_SOCKET_MODE`, `PERSONA_SUPERVISION_SOCKET_PATH`, and
/// `PERSONA_SUPERVISION_SOCKET_MODE` argv/environment-variable
/// surface. The ordinary router socket and meta-policy socket are
/// separate fields so launch configuration exposes both triad tiers.
#[derive(
    Archive, RkyvSerialize, RkyvDeserialize, NotaEncode, NotaDecode, Debug, Clone, PartialEq, Eq,
)]
pub struct RouterDaemonConfiguration {
    /// Where the daemon binds its router Unix socket.
    pub router_socket_path: WirePath,
    /// chmod applied to the router socket after bind.
    pub router_socket_mode: SocketMode,
    /// Where the daemon binds its meta-policy Unix socket.
    pub meta_router_socket_path: WirePath,
    /// chmod applied to the meta-policy socket after bind.
    pub meta_router_socket_mode: SocketMode,
    /// Where the daemon binds its supervision Unix socket.
    pub supervision_socket_path: WirePath,
    /// chmod applied to the supervision socket after bind.
    pub supervision_socket_mode: SocketMode,
    /// Path to the router daemon's sema-engine store file.
    pub store_path: WirePath,
    /// Optional bootstrap-record path the daemon applies at startup.
    pub bootstrap_path: Option<WirePath>,
    /// The engine owner identity passed to the router daemon.
    pub owner_identity: OwnerIdentity,
}

impl RouterDaemonConfiguration {
    pub fn from_rkyv_bytes(bytes: &[u8]) -> Result<Self, RouterDaemonConfigurationArchiveError> {
        rkyv::from_bytes::<Self, rkyv::rancor::Error>(bytes)
            .map_err(|_| RouterDaemonConfigurationArchiveError::Decode)
    }

    pub fn to_rkyv_bytes(&self) -> Result<Vec<u8>, RouterDaemonConfigurationArchiveError> {
        rkyv::to_bytes::<rkyv::rancor::Error>(self)
            .map(|bytes| bytes.to_vec())
            .map_err(|_| RouterDaemonConfigurationArchiveError::Encode)
    }
}

#[derive(Debug, thiserror::Error)]
pub enum RouterDaemonConfigurationArchiveError {
    #[error("failed to encode router daemon configuration archive")]
    Encode,

    #[error("failed to decode router daemon configuration archive")]
    Decode,
}

#![allow(dead_code, non_camel_case_types, non_snake_case)]
#[rustfmt::skip]
pub type ObservationIdentifier = String;
#[rustfmt::skip]
pub type ActorIdentifier = String;
#[rustfmt::skip]
pub type EngineIdentifier = String;
#[rustfmt::skip]
pub type ChannelIdentifier = String;
#[rustfmt::skip]
pub type MessageSlot = i64;
#[rustfmt::skip]
pub type WirePath = String;
#[rustfmt::skip]
pub type SocketMode = i64;
#[rustfmt::skip]
pub type UnixUserIdentifier = i64;
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq)]
#[cfg_attr(
    feature = "datom",
    derive(datom_codec::Datomizable, datom_codec::Compositional)
)]
pub enum OwnerIdentity {
    UnixUser(UnixUserIdentifier),
}
#[rustfmt::skip]
pub type TailnetAddress = String;
#[rustfmt::skip]
pub type CriomeHostId = String;
#[rustfmt::skip]
pub type HostName = String;
#[rustfmt::skip]
pub type TimestampNanos = i64;
#[rustfmt::skip]
pub type ReplayNonce = String;
#[rustfmt::skip]
pub type ContractName = String;
#[rustfmt::skip]
pub type ContractOperation = String;
#[rustfmt::skip]
pub type ContractPayloadSize = i64;
#[rustfmt::skip]
pub type Engine = EngineIdentifier;
#[rustfmt::skip]
pub type Channel = ChannelIdentifier;
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq)]
#[cfg_attr(
    feature = "datom",
    derive(datom_codec::Datomizable, datom_codec::Compositional)
)]
pub enum EndpointKind {
    Human,
    HarnessSocket,
    PtySocket,
    RemoteRouter,
    ComponentSocket,
}
#[rustfmt::skip]
pub type Target = String;
#[rustfmt::skip]
pub type Auxiliary = std::option::Option<String>;
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq)]
#[cfg_attr(
    feature = "datom",
    derive(datom_codec::Datomizable, datom_codec::Compositional)
)]
pub struct EndpointTransport {
    pub endpoint_kind: EndpointKind,
    pub target: Target,
    pub auxiliary: Auxiliary,
}
#[rustfmt::skip]
pub type Name = ActorIdentifier;
#[rustfmt::skip]
pub type Process = i64;
#[rustfmt::skip]
pub type Endpoint = std::option::Option<EndpointTransport>;
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq)]
#[cfg_attr(
    feature = "datom",
    derive(datom_codec::Datomizable, datom_codec::Compositional)
)]
pub struct Actor {
    pub name: Name,
    pub process: Process,
    pub endpoint: Endpoint,
}
#[rustfmt::skip]
pub type Home = std::option::Option<CriomeHostId>;
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq)]
#[cfg_attr(
    feature = "datom",
    derive(datom_codec::Datomizable, datom_codec::Compositional)
)]
pub struct RegisterActor {
    pub actor: Actor,
    pub home: Home,
}
#[rustfmt::skip]
pub type RegisteredActor = ActorIdentifier;
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq)]
#[cfg_attr(
    feature = "datom",
    derive(datom_codec::Datomizable, datom_codec::Compositional)
)]
pub enum ActorRegistrationDisposition {
    Registered,
    EndpointUpdated,
}
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq)]
#[cfg_attr(
    feature = "datom",
    derive(datom_codec::Datomizable, datom_codec::Compositional)
)]
pub struct ActorRegistered {
    pub registered_actor: RegisteredActor,
    pub actor_registration_disposition: ActorRegistrationDisposition,
}
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq)]
#[cfg_attr(
    feature = "datom",
    derive(datom_codec::Datomizable, datom_codec::Compositional)
)]
pub enum ActorRegistrationRefusalReason {
    ProcessIdentifierOutOfRange,
    RemoteRouterEndpointNotLocal,
}
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq)]
#[cfg_attr(
    feature = "datom",
    derive(datom_codec::Datomizable, datom_codec::Compositional)
)]
pub struct ActorRegistrationRefused {
    pub registered_actor: RegisteredActor,
    pub actor_registration_refusal_reason: ActorRegistrationRefusalReason,
}
#[rustfmt::skip]
pub type SourceActor = ActorIdentifier;
#[rustfmt::skip]
pub type DestinationActor = ActorIdentifier;
#[rustfmt::skip]
pub type Requester = ActorIdentifier;
#[rustfmt::skip]
pub type Identity = CriomeHostId;
#[rustfmt::skip]
pub type Address = TailnetAddress;
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq)]
#[cfg_attr(
    feature = "datom",
    derive(datom_codec::Datomizable, datom_codec::Compositional)
)]
pub struct GrantDirectMessage {
    pub source_actor: SourceActor,
    pub destination_actor: DestinationActor,
}
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq)]
#[cfg_attr(
    feature = "datom",
    derive(datom_codec::Datomizable, datom_codec::Compositional)
)]
pub struct InstallStructuralChannels {
    pub requester: Requester,
}
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq)]
#[cfg_attr(
    feature = "datom",
    derive(datom_codec::Datomizable, datom_codec::Compositional)
)]
pub struct RegisterRemoteRouter {
    pub identity: Identity,
    pub address: Address,
}
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq)]
#[cfg_attr(
    feature = "datom",
    derive(datom_codec::Datomizable, datom_codec::Compositional)
)]
pub enum RouterBootstrapOperation {
    RegisterActor(RegisterActor),
    GrantDirectMessage(GrantDirectMessage),
    InstallStructuralChannels(InstallStructuralChannels),
    RegisterRemoteRouter(RegisterRemoteRouter),
}
#[rustfmt::skip]
pub type RouterBootstrapDocument = std::vec::Vec<RouterBootstrapOperation>;
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq)]
#[cfg_attr(
    feature = "datom",
    derive(datom_codec::Datomizable, datom_codec::Compositional)
)]
pub enum RouterObservationScope {
    Summary,
    MessageTrace,
    ChannelState,
}
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq)]
#[cfg_attr(
    feature = "datom",
    derive(datom_codec::Datomizable, datom_codec::Compositional)
)]
pub struct RouterSummaryQuery {
    pub engine: Engine,
}
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq)]
#[cfg_attr(
    feature = "datom",
    derive(datom_codec::Datomizable, datom_codec::Compositional)
)]
pub struct RouterMessageTraceQuery {
    pub engine: Engine,
    pub message_slot: MessageSlot,
}
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq)]
#[cfg_attr(
    feature = "datom",
    derive(datom_codec::Datomizable, datom_codec::Compositional)
)]
pub struct RouterChannelStateQuery {
    pub engine: Engine,
    pub channel: Channel,
}
#[rustfmt::skip]
pub type AcceptedMessages = i64;
#[rustfmt::skip]
pub type RoutedMessages = i64;
#[rustfmt::skip]
pub type DeferredMessages = i64;
#[rustfmt::skip]
pub type FailedMessages = i64;
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq)]
#[cfg_attr(
    feature = "datom",
    derive(datom_codec::Datomizable, datom_codec::Compositional)
)]
pub struct RouterSummary {
    pub engine: Engine,
    pub accepted_messages: AcceptedMessages,
    pub routed_messages: RoutedMessages,
    pub deferred_messages: DeferredMessages,
    pub failed_messages: FailedMessages,
}
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq)]
#[cfg_attr(
    feature = "datom",
    derive(datom_codec::Datomizable, datom_codec::Compositional)
)]
pub enum RouterDeliveryStatus {
    Accepted,
    Routed,
    Delivered,
    Deferred,
    Failed,
    ForwardedRemote,
}
#[rustfmt::skip]
pub type DeliveryStatus = RouterDeliveryStatus;
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq)]
#[cfg_attr(
    feature = "datom",
    derive(datom_codec::Datomizable, datom_codec::Compositional)
)]
pub struct RouterMessageTrace {
    pub engine: Engine,
    pub message_slot: MessageSlot,
    pub delivery_status: DeliveryStatus,
}
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq)]
#[cfg_attr(
    feature = "datom",
    derive(datom_codec::Datomizable, datom_codec::Compositional)
)]
pub struct RouterMessageTraceMissing {
    pub engine: Engine,
    pub message_slot: MessageSlot,
}
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq)]
#[cfg_attr(
    feature = "datom",
    derive(datom_codec::Datomizable, datom_codec::Compositional)
)]
pub enum RouterChannelStatus {
    Installed,
    Missing,
    Disabled,
}
#[rustfmt::skip]
pub type ChannelStatus = RouterChannelStatus;
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq)]
#[cfg_attr(
    feature = "datom",
    derive(datom_codec::Datomizable, datom_codec::Compositional)
)]
pub struct RouterChannelState {
    pub engine: Engine,
    pub channel: Channel,
    pub channel_status: ChannelStatus,
}
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq)]
#[cfg_attr(
    feature = "datom",
    derive(datom_codec::Datomizable, datom_codec::Compositional)
)]
pub enum RouterObservationUnimplementedReason {
    NotInPrototypeScope,
    RouterStoreUnavailable,
    MessageTraceUnavailable,
}
#[rustfmt::skip]
pub type ObservationScope = RouterObservationScope;
#[rustfmt::skip]
pub type ObservationReason = RouterObservationUnimplementedReason;
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq)]
#[cfg_attr(
    feature = "datom",
    derive(datom_codec::Datomizable, datom_codec::Compositional)
)]
pub struct RouterObservationUnimplemented {
    pub observation_scope: ObservationScope,
    pub observation_reason: ObservationReason,
}
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq)]
#[cfg_attr(
    feature = "datom",
    derive(datom_codec::Datomizable, datom_codec::Compositional)
)]
pub enum SignatureScheme {
    Bls12_381MinPk,
    Bls12_381MinSig,
}
#[rustfmt::skip]
pub type Signer = CriomeHostId;
#[rustfmt::skip]
pub type Scheme = SignatureScheme;
#[rustfmt::skip]
pub type PublicKey = String;
#[rustfmt::skip]
pub type Signature = String;
#[rustfmt::skip]
pub type ContentDigest = String;
#[rustfmt::skip]
pub type IssuedAt = TimestampNanos;
#[rustfmt::skip]
pub type Nonce = ReplayNonce;
#[rustfmt::skip]
pub type AttestationIssuedAt = TimestampNanos;
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq)]
#[cfg_attr(
    feature = "datom",
    derive(datom_codec::Datomizable, datom_codec::Compositional)
)]
pub struct RouterPeerAttestation {
    pub signer: Signer,
    pub scheme: Scheme,
    pub public_key: PublicKey,
    pub signature: Signature,
    pub content_digest: ContentDigest,
    pub issued_at: IssuedAt,
    pub nonce: Nonce,
    pub attestation_issued_at: AttestationIssuedAt,
}
#[rustfmt::skip]
pub type Octet = i64;
#[rustfmt::skip]
pub type PayloadOctets = std::vec::Vec<Octet>;
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq)]
#[cfg_attr(
    feature = "datom",
    derive(datom_codec::Datomizable, datom_codec::Compositional)
)]
pub struct RoutedContractObject {
    pub contract_name: ContractName,
    pub contract_operation: ContractOperation,
    pub contract_payload_size: ContractPayloadSize,
    pub payload_octets: PayloadOctets,
}
#[rustfmt::skip]
pub type Body = String;
#[rustfmt::skip]
pub type Attachment = String;
#[rustfmt::skip]
pub type Attachments = std::vec::Vec<Attachment>;
#[rustfmt::skip]
pub type RoutedObjects = std::vec::Vec<RoutedContractObject>;
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq)]
#[cfg_attr(
    feature = "datom",
    derive(datom_codec::Datomizable, datom_codec::Compositional)
)]
pub struct ForwardedMessagePayload {
    pub source_actor: SourceActor,
    pub destination_actor: DestinationActor,
    pub body: Body,
    pub attachments: Attachments,
    pub routed_objects: RoutedObjects,
}
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq)]
#[cfg_attr(
    feature = "datom",
    derive(datom_codec::Datomizable, datom_codec::Compositional)
)]
pub enum ForwardMarker {
    Origin,
    Forwarded,
}
#[rustfmt::skip]
pub type Submission = ForwardedMessagePayload;
#[rustfmt::skip]
pub type Attestation = RouterPeerAttestation;
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq)]
#[cfg_attr(
    feature = "datom",
    derive(datom_codec::Datomizable, datom_codec::Compositional)
)]
pub struct RouterForwardRequest {
    pub submission: Submission,
    pub attestation: Attestation,
    pub forward_marker: ForwardMarker,
    pub nonce: Nonce,
    pub issued_at: IssuedAt,
}
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq)]
#[cfg_attr(
    feature = "datom",
    derive(datom_codec::Datomizable, datom_codec::Compositional)
)]
pub struct RouterForwardAccepted {
    pub message_slot: MessageSlot,
}
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq)]
#[cfg_attr(
    feature = "datom",
    derive(datom_codec::Datomizable, datom_codec::Compositional)
)]
pub enum RouterForwardRefusalReason {
    UnknownPeer,
    AttestationInvalid,
    ReplayDetected,
    ClockSkew,
    RecipientUnknown,
    ChannelUnauthorized,
    AlreadyForwarded,
    MirrorDisabled,
    SessionRequired,
}
#[rustfmt::skip]
pub type ForwardRefusalReason = RouterForwardRefusalReason;
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq)]
#[cfg_attr(
    feature = "datom",
    derive(datom_codec::Datomizable, datom_codec::Compositional)
)]
pub struct RouterForwardRefused {
    pub forward_refusal_reason: ForwardRefusalReason,
}
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq)]
#[cfg_attr(
    feature = "datom",
    derive(datom_codec::Datomizable, datom_codec::Compositional)
)]
pub struct RouterRoutedObjectsAccepted {
    pub message_slot: MessageSlot,
}
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq)]
#[cfg_attr(
    feature = "datom",
    derive(datom_codec::Datomizable, datom_codec::Compositional)
)]
pub struct RouterRoutedObjectsRefused {
    pub forward_refusal_reason: ForwardRefusalReason,
}
#[rustfmt::skip]
pub type SessionChallenge = String;
#[rustfmt::skip]
pub type EphemeralPublicKey = String;
#[rustfmt::skip]
pub type ProofSigner = CriomeHostId;
#[rustfmt::skip]
pub type ProofScheme = SignatureScheme;
#[rustfmt::skip]
pub type ProofPublicKey = String;
#[rustfmt::skip]
pub type ProofSignature = String;
#[rustfmt::skip]
pub type ProofDigest = String;
#[rustfmt::skip]
pub type ProofChallengeNonce = ReplayNonce;
#[rustfmt::skip]
pub type ProofAttestationIssuedAt = TimestampNanos;
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq)]
#[cfg_attr(
    feature = "datom",
    derive(datom_codec::Datomizable, datom_codec::Compositional)
)]
pub struct RouterIdentityProof {
    pub proof_signer: ProofSigner,
    pub proof_scheme: ProofScheme,
    pub proof_public_key: ProofPublicKey,
    pub proof_signature: ProofSignature,
    pub proof_digest: ProofDigest,
    pub proof_challenge_nonce: ProofChallengeNonce,
    pub proof_attestation_issued_at: ProofAttestationIssuedAt,
}
#[rustfmt::skip]
pub type InitiatorChallenge = SessionChallenge;
#[rustfmt::skip]
pub type InitiatorEphemeralKey = EphemeralPublicKey;
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq)]
#[cfg_attr(
    feature = "datom",
    derive(datom_codec::Datomizable, datom_codec::Compositional)
)]
pub struct RouterSessionClientHello {
    pub initiator_challenge: InitiatorChallenge,
    pub initiator_ephemeral_key: InitiatorEphemeralKey,
}
#[rustfmt::skip]
pub type ResponderChallenge = SessionChallenge;
#[rustfmt::skip]
pub type ResponderEphemeralKey = EphemeralPublicKey;
#[rustfmt::skip]
pub type ResponderProof = RouterIdentityProof;
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq)]
#[cfg_attr(
    feature = "datom",
    derive(datom_codec::Datomizable, datom_codec::Compositional)
)]
pub struct RouterSessionServerHello {
    pub responder_challenge: ResponderChallenge,
    pub responder_ephemeral_key: ResponderEphemeralKey,
    pub responder_proof: ResponderProof,
}
#[rustfmt::skip]
pub type InitiatorProof = RouterIdentityProof;
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq)]
#[cfg_attr(
    feature = "datom",
    derive(datom_codec::Datomizable, datom_codec::Compositional)
)]
pub struct RouterSessionClientProof {
    pub initiator_proof: InitiatorProof,
}
#[rustfmt::skip]
pub type KeyConfirmation = std::vec::Vec<Octet>;
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq)]
#[cfg_attr(
    feature = "datom",
    derive(datom_codec::Datomizable, datom_codec::Compositional)
)]
pub struct RouterSessionAccepted {
    pub key_confirmation: KeyConfirmation,
}
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq)]
#[cfg_attr(
    feature = "datom",
    derive(datom_codec::Datomizable, datom_codec::Compositional)
)]
pub enum SessionRefusalReason {
    IdentityProofInvalid,
    ChallengeMismatch,
    HandshakeMalformed,
    SessionCipherFailure,
}
#[rustfmt::skip]
pub type RefusalReason = SessionRefusalReason;
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq)]
#[cfg_attr(
    feature = "datom",
    derive(datom_codec::Datomizable, datom_codec::Compositional)
)]
pub struct RouterSessionRefused {
    pub refusal_reason: RefusalReason,
}
#[rustfmt::skip]
pub type SealedOctets = std::vec::Vec<Octet>;
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq)]
#[cfg_attr(
    feature = "datom",
    derive(datom_codec::Datomizable, datom_codec::Compositional)
)]
pub struct RouterSessionData {
    pub sealed_octets: SealedOctets,
}
#[rustfmt::skip]
pub type RouterSocketPath = WirePath;
#[rustfmt::skip]
pub type RouterSocketMode = SocketMode;
#[rustfmt::skip]
pub type MetaRouterSocketPath = WirePath;
#[rustfmt::skip]
pub type MetaRouterSocketMode = SocketMode;
#[rustfmt::skip]
pub type SupervisionSocketPath = WirePath;
#[rustfmt::skip]
pub type SupervisionSocketMode = SocketMode;
#[rustfmt::skip]
pub type StorePath = WirePath;
#[rustfmt::skip]
pub type BootstrapPath = std::option::Option<WirePath>;
#[rustfmt::skip]
pub type TailnetListenAddress = std::option::Option<TailnetAddress>;
#[rustfmt::skip]
pub type RouterIdentity = CriomeHostId;
#[rustfmt::skip]
pub type CriomeSocketPath = std::option::Option<WirePath>;
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq)]
#[cfg_attr(
    feature = "datom",
    derive(datom_codec::Datomizable, datom_codec::Compositional)
)]
pub struct RouterDaemonConfiguration {
    pub router_socket_path: RouterSocketPath,
    pub router_socket_mode: RouterSocketMode,
    pub meta_router_socket_path: MetaRouterSocketPath,
    pub meta_router_socket_mode: MetaRouterSocketMode,
    pub supervision_socket_path: SupervisionSocketPath,
    pub supervision_socket_mode: SupervisionSocketMode,
    pub store_path: StorePath,
    pub bootstrap_path: BootstrapPath,
    pub owner_identity: OwnerIdentity,
    pub tailnet_listen_address: TailnetListenAddress,
    pub router_identity: RouterIdentity,
    pub criome_socket_path: CriomeSocketPath,
}
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq)]
#[cfg_attr(
    feature = "datom",
    derive(datom_codec::Datomizable, datom_codec::Compositional)
)]
pub enum Query {
    Summary(RouterSummaryQuery),
    MessageTrace(RouterMessageTraceQuery),
    ChannelState(RouterChannelStateQuery),
    ForwardMessage(RouterForwardRequest),
    SubmitRoutedObjects(ForwardedMessagePayload),
    SessionClientHello(RouterSessionClientHello),
    SessionClientProof(RouterSessionClientProof),
    SessionData(RouterSessionData),
    RegisterActor(Actor),
}
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq)]
#[cfg_attr(
    feature = "datom",
    derive(datom_codec::Datomizable, datom_codec::Compositional)
)]
pub enum Response {
    Summary(RouterSummary),
    MessageTrace(RouterMessageTrace),
    MessageTraceMissing(RouterMessageTraceMissing),
    ChannelState(RouterChannelState),
    ForwardAccepted(RouterForwardAccepted),
    ForwardRefused(RouterForwardRefused),
    Unimplemented(RouterObservationUnimplemented),
    RoutedObjectsAccepted(RouterRoutedObjectsAccepted),
    RoutedObjectsRefused(RouterRoutedObjectsRefused),
    SessionServerHello(RouterSessionServerHello),
    SessionAccepted(RouterSessionAccepted),
    SessionRefused(RouterSessionRefused),
    SessionData(RouterSessionData),
    ActorRegistered(ActorRegistered),
    ActorRegistrationRefused(ActorRegistrationRefused),
}

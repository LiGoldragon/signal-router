//! Schema-derived Signal contract for the ordinary `router` surface.
//!
//! This crate exposes the peer-callable router observation contract and
//! router-owned bootstrap/configuration records. Runtime routing, Nexus/SEMA
//! lowering, storage, sockets, and policy logic live in `router`; meta policy
//! orders live in `meta-signal-router`.

#[rustfmt::skip]
#[allow(clippy::large_enum_variant, dead_code, private_interfaces)]
pub mod schema;

pub use schema::lib::*;

impl RouterBootstrapOperation {
    #[cfg(feature = "nota-text")]
    pub fn from_nota(text: &str) -> Result<Self, NotaDecodeError> {
        NotaSource::new(text).parse()
    }
}

impl RouterBootstrapDocument {
    pub fn from_operations(operations: Vec<RouterBootstrapOperation>) -> Self {
        Self::new(Operations::new(operations))
    }

    pub fn operations(&self) -> &[RouterBootstrapOperation] {
        self.payload().payload().as_slice()
    }

    pub fn into_operations(self) -> Vec<RouterBootstrapOperation> {
        self.into_payload().into_payload()
    }

    #[cfg(feature = "nota-text")]
    pub fn from_nota_lines(text: &str) -> Result<Self, NotaDecodeError> {
        let mut operations = Vec::new();
        for line in text.lines().map(str::trim).filter(|line| !line.is_empty()) {
            operations.push(RouterBootstrapOperation::from_nota(line)?);
        }
        Ok(Self::from_operations(operations))
    }

    #[cfg(feature = "nota-text")]
    pub fn to_nota_lines(&self) -> String {
        let mut text = String::new();
        for operation in self.operations() {
            text.push_str(operation.to_nota().as_str());
            text.push('\n');
        }
        text
    }
}

impl EndpointTransport {
    pub fn new(kind: EndpointKind, target: String, auxiliary: Option<String>) -> Self {
        Self {
            kind,
            target,
            auxiliary: Auxiliary::new(auxiliary),
        }
    }

    pub fn auxiliary(&self) -> Option<&String> {
        self.auxiliary.payload().as_ref()
    }
}

impl Actor {
    pub fn new(
        name: ActorIdentifier,
        process: Integer,
        endpoint: Option<EndpointTransport>,
    ) -> Self {
        Self {
            name,
            process,
            endpoint: Endpoint::new(endpoint),
        }
    }

    pub fn endpoint(&self) -> Option<&EndpointTransport> {
        self.endpoint.payload().as_ref()
    }
}

impl RegisterActor {
    pub fn new(actor: Actor, home: Option<RemoteRouterIdentity>) -> Self {
        Self {
            actor,
            home: Home::new(home),
        }
    }

    pub fn home(&self) -> Option<&RemoteRouterIdentity> {
        self.home.payload().as_ref()
    }
}

impl RoutedContractObject {
    pub fn new(
        contract_name: ContractName,
        contract_operation: ContractOperation,
        contract_payload_size: ContractPayloadSize,
        payload_octets: Vec<Integer>,
    ) -> Self {
        Self {
            contract_name,
            contract_operation,
            contract_payload_size,
            payload_octets: PayloadOctets::new(payload_octets),
        }
    }

    pub fn payload_octets(&self) -> &[Integer] {
        self.payload_octets.payload().as_slice()
    }
}

impl ForwardedMessagePayload {
    pub fn new(
        from: ActorIdentifier,
        to: ActorIdentifier,
        body: String,
        attachments: Vec<String>,
        routed_objects: Vec<RoutedContractObject>,
    ) -> Self {
        Self {
            from,
            to,
            body,
            attachments: Attachments::new(attachments),
            routed_objects: RoutedObjects::new(routed_objects),
        }
    }

    pub fn attachments(&self) -> &[String] {
        self.attachments.payload().as_slice()
    }

    pub fn routed_objects(&self) -> &[RoutedContractObject] {
        self.routed_objects.payload().as_slice()
    }

    pub fn push_routed_object(&mut self, routed_object: RoutedContractObject) {
        let mut routed_objects = self.routed_objects.clone().into_payload();
        routed_objects.push(routed_object);
        self.routed_objects = RoutedObjects::new(routed_objects);
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct RouterDaemonConfigurationParts {
    pub router_socket_path: WirePath,
    pub router_socket_mode: SocketMode,
    pub meta_router_socket_path: WirePath,
    pub meta_router_socket_mode: SocketMode,
    pub supervision_socket_path: WirePath,
    pub supervision_socket_mode: SocketMode,
    pub store_path: WirePath,
    pub bootstrap_path: Option<WirePath>,
    pub owner_identity: OwnerIdentity,
    pub tailnet_listen_address: Option<TailnetAddress>,
    pub router_identity: RemoteRouterIdentity,
    pub criome_socket_path: Option<WirePath>,
}

impl From<RouterDaemonConfigurationParts> for RouterDaemonConfiguration {
    fn from(parts: RouterDaemonConfigurationParts) -> Self {
        Self {
            router_socket_path: parts.router_socket_path,
            router_socket_mode: parts.router_socket_mode,
            meta_router_socket_path: parts.meta_router_socket_path,
            meta_router_socket_mode: parts.meta_router_socket_mode,
            supervision_socket_path: parts.supervision_socket_path,
            supervision_socket_mode: parts.supervision_socket_mode,
            store_path: parts.store_path,
            bootstrap_path: BootstrapPath::new(parts.bootstrap_path),
            owner_identity: parts.owner_identity,
            tailnet_listen_address: TailnetListenAddress::new(parts.tailnet_listen_address),
            router_identity: parts.router_identity,
            criome_socket_path: CriomeSocketPath::new(parts.criome_socket_path),
        }
    }
}

impl RouterDaemonConfiguration {
    pub fn bootstrap_path(&self) -> Option<&WirePath> {
        self.bootstrap_path.payload().as_ref()
    }

    pub fn tailnet_listen_address(&self) -> Option<&TailnetAddress> {
        self.tailnet_listen_address.payload().as_ref()
    }

    pub fn criome_socket_path(&self) -> Option<&WirePath> {
        self.criome_socket_path.payload().as_ref()
    }

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

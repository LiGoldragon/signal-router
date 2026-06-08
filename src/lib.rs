//! Schema-derived Signal contract for the ordinary `router` surface.
//!
//! This crate exposes the peer-callable router observation contract and
//! router-owned bootstrap/configuration records. Runtime routing, Nexus/SEMA
//! lowering, storage, sockets, and policy logic live in `router`; meta policy
//! orders live in `meta-signal-router`.

#[rustfmt::skip]
pub mod schema;

pub use schema::lib::*;

impl RouterBootstrapOperation {
    #[cfg(feature = "nota-text")]
    pub fn from_nota(text: &str) -> Result<Self, NotaDecodeError> {
        NotaSource::new(text).parse()
    }
}

impl RouterBootstrapDocument {
    pub fn operations(&self) -> &[RouterBootstrapOperation] {
        self.payload().as_slice()
    }

    pub fn into_operations(self) -> Vec<RouterBootstrapOperation> {
        self.into_payload()
    }

    #[cfg(feature = "nota-text")]
    pub fn from_nota_lines(text: &str) -> Result<Self, NotaDecodeError> {
        let mut operations = Vec::new();
        for line in text.lines().map(str::trim).filter(|line| !line.is_empty()) {
            operations.push(RouterBootstrapOperation::from_nota(line)?);
        }
        Ok(Self::new(operations))
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

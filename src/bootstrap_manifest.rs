//! Explicit producer-owned bootstrap authority state for the ordinary Router Interface.
//!
//! Every identity and canonical-order value below is an already-minted opaque
//! seat. None is derived from source spelling, position, or content.

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct AuthoritySeat {
    pub spelling: &'static str,
    pub local: u16,
    pub canonical: u64,
}

impl AuthoritySeat {
    pub const fn new(spelling: &'static str, local: u16, canonical: u64) -> Self {
        Self {
            spelling,
            local,
            canonical,
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct DeclarationSeat {
    pub owner_local: Option<u16>,
    pub spelling: &'static str,
    pub local: u16,
    pub canonical: u64,
}

impl DeclarationSeat {
    pub const fn new(
        owner_local: Option<u16>,
        spelling: &'static str,
        local: u16,
        canonical: u64,
    ) -> Self {
        Self {
            owner_local,
            spelling,
            local,
            canonical,
        }
    }
}

pub const AUTHORITY_IDENTITY: [u8; 32] = [
    108, 213, 31, 242, 147, 87, 32, 23, 51, 137, 217, 110, 222, 192, 60, 167, 226, 160, 65, 51,
    105, 105, 26, 195, 1, 231, 217, 247, 41, 248, 178, 54,
];
pub const AUTHORITY_REVISION: u64 = 1;
pub const GRAMMAR_DOCUMENT_LOCAL: u16 = 29365;
pub const GRAMMAR_SYNTAX_LOCAL: u16 = 44072;

pub const INTERFACE_SEAT: AuthoritySeat =
    AuthoritySeat::new("Interface", 52373, 0xbf67db0d6e547001);
pub const NEXUS_SEAT: AuthoritySeat = AuthoritySeat::new("Nexus", 19014, 0x0307db30686f702b);
pub const SEMA_SEAT: AuthoritySeat = AuthoritySeat::new("Sema", 62054, 0x0f1e298c704167f1);
pub const INPUT_SEAT: AuthoritySeat = AuthoritySeat::new("Input", 18547, 0xa701797429ec243d);
pub const OUTPUT_SEAT: AuthoritySeat = AuthoritySeat::new("Output", 56984, 0x6e0fb3caedcaa9ef);
pub const REFUSAL_SEAT: AuthoritySeat = AuthoritySeat::new("Refusal", 25009, 0x16b3c6b53f7bfae9);
pub const STRING_SEAT: AuthoritySeat = AuthoritySeat::new("String", 10716, 0xece4b7fab29a280e);
pub const INTEGER_SEAT: AuthoritySeat = AuthoritySeat::new("Integer", 54980, 0x4ec30fcd1939bfe8);
pub const BOOLEAN_SEAT: AuthoritySeat = AuthoritySeat::new("Boolean", 17604, 0xc761a5c6a49a63be);
pub const UNIT_SEAT: AuthoritySeat = AuthoritySeat::new("Unit", 33805, 0xf4601f58a1f28535);
pub const VECTOR_SEAT: AuthoritySeat = AuthoritySeat::new("Vector", 2787, 0x961b5c344acdd970);
pub const OPTION_SEAT: AuthoritySeat = AuthoritySeat::new("Option", 4009, 0x9c36e58019e97e3d);
pub const MAP_SEAT: AuthoritySeat = AuthoritySeat::new("Map", 19518, 0xc11c147eeaf50cf7);
pub const RESULT_SEAT: AuthoritySeat = AuthoritySeat::new("Result", 43639, 0xda43c0937238dfe3);
pub const STREAM_SEAT: AuthoritySeat = AuthoritySeat::new("Stream", 57364, 0x6e72dc5c45ba1e1f);
pub const STREAMIDENTITY_SEAT: AuthoritySeat =
    AuthoritySeat::new("StreamIdentity", 47505, 0xc0d73d84c332e127);

pub const RUST_VOCABULARY_LOCALS: [u16; 10] = [
    7535, 55482, 40807, 13244, 49320, 32447, 16204, 27145, 17596, 62882,
];

pub const DECLARATION_SEATS: &[DeclarationSeat] = &[
    DeclarationSeat::new(None, "ObservationIdentifier", 34492, 0x5b52183c1fce1d05),
    DeclarationSeat::new(None, "ActorIdentifier", 8741, 0xa8b24919d9d0b71e),
    DeclarationSeat::new(None, "EngineIdentifier", 52861, 0x1fdf80c997c0b943),
    DeclarationSeat::new(None, "ChannelIdentifier", 30071, 0x51e5ef9b6fadc4bf),
    DeclarationSeat::new(None, "MessageSlot", 41030, 0x62f7dfac32d1825e),
    DeclarationSeat::new(None, "WirePath", 13500, 0xb2bc25d82372e931),
    DeclarationSeat::new(None, "SocketMode", 64749, 0x8dec447c5892cbfa),
    DeclarationSeat::new(None, "UnixUserIdentifier", 65176, 0xe1d08d92bc8e15d4),
    DeclarationSeat::new(None, "OwnerIdentity", 3986, 0xbda8bcc71a5d6fb3),
    DeclarationSeat::new(Some(3986), "UnixUser", 4349, 0x28dc968f3c88e503),
    DeclarationSeat::new(None, "TailnetAddress", 32403, 0x8edaed89606946ce),
    DeclarationSeat::new(None, "CriomeHostId", 10701, 0xe2d5987086749b0b),
    DeclarationSeat::new(None, "TimestampNanos", 15140, 0x04d4b8f5f2f06ccf),
    DeclarationSeat::new(None, "ReplayNonce", 1637, 0x70a7fad5300eb672),
    DeclarationSeat::new(None, "ContractName", 52327, 0x14051da4be869ea3),
    DeclarationSeat::new(None, "ContractOperation", 31344, 0x0e86f8cb49140b63),
    DeclarationSeat::new(None, "ContractPayloadSize", 11426, 0x99632a5dbca621a2),
    DeclarationSeat::new(None, "EndpointKind", 48929, 0xa40b94d15cce86d7),
    DeclarationSeat::new(Some(48929), "Human", 40233, 0x7906cd31c9e370c2),
    DeclarationSeat::new(Some(48929), "HarnessSocket", 8524, 0x9351a199085e0c87),
    DeclarationSeat::new(Some(48929), "PtySocket", 58338, 0xb7bb4975a73d9151),
    DeclarationSeat::new(Some(48929), "RemoteRouter", 42868, 0x78bd467239fc2d73),
    DeclarationSeat::new(Some(48929), "ComponentSocket", 28690, 0xaf5029f880543bb5),
    DeclarationSeat::new(None, "Kind", 45756, 0x39e7d1ce860e771a),
    DeclarationSeat::new(None, "Target", 39164, 0x050a3dcf069dc004),
    DeclarationSeat::new(None, "EndpointTransport", 2863, 0xc5e0d2f2ead1ce10),
    DeclarationSeat::new(None, "Name", 14694, 0xef2784841b9f48ea),
    DeclarationSeat::new(None, "Process", 33188, 0xd90ad7ff78269126),
    DeclarationSeat::new(None, "Actor", 25173, 0xa9d3a6f0779c8959),
    DeclarationSeat::new(None, "RegisterActor", 13021, 0x84c32f80585b5ef2),
    DeclarationSeat::new(None, "RegisteredActor", 23818, 0x8c433c4e030c8139),
    DeclarationSeat::new(
        None,
        "ActorRegistrationDisposition",
        45743,
        0x5bc1a9fad1ceee9d,
    ),
    DeclarationSeat::new(Some(45743), "Registered", 6404, 0xd00d9dd0e572969a),
    DeclarationSeat::new(Some(45743), "EndpointUpdated", 20739, 0xfd7f2f82b357d6d4),
    DeclarationSeat::new(None, "ActorRegistered", 8570, 0x68b6faa77de26d1b),
    DeclarationSeat::new(
        None,
        "ActorRegistrationRefusalReason",
        26554,
        0xb61b702d87480831,
    ),
    DeclarationSeat::new(
        Some(26554),
        "ProcessIdentifierOutOfRange",
        25427,
        0x25305fc4cb790643,
    ),
    DeclarationSeat::new(
        Some(26554),
        "RemoteRouterEndpointNotLocal",
        26218,
        0x17ed4e28931dacd2,
    ),
    DeclarationSeat::new(None, "ActorRegistrationRefused", 54608, 0xef27f64a5e025f12),
    DeclarationSeat::new(None, "SourceActor", 33065, 0x014e22de647c6d87),
    DeclarationSeat::new(None, "DestinationActor", 32880, 0xd534dd11c2e44539),
    DeclarationSeat::new(None, "Requester", 46173, 0x8ead8ad29c76d319),
    DeclarationSeat::new(None, "Identity", 61297, 0x3ebda03fb4e6c48b),
    DeclarationSeat::new(None, "Address", 54419, 0xb455f1235b3568d6),
    DeclarationSeat::new(None, "GrantDirectMessage", 13425, 0xd402d378617e6689),
    DeclarationSeat::new(None, "InstallStructuralChannels", 11768, 0xda24483ff8a71d03),
    DeclarationSeat::new(None, "RegisterRemoteRouter", 36972, 0xb3fc6192b60136a8),
    DeclarationSeat::new(None, "RouterBootstrapOperation", 9845, 0x898afae661dfb68f),
    DeclarationSeat::new(Some(9845), "RegisterActor", 14849, 0x9c229a9f89d7313e),
    DeclarationSeat::new(Some(9845), "GrantDirectMessage", 29483, 0xcf782ae82f39e6b6),
    DeclarationSeat::new(
        Some(9845),
        "InstallStructuralChannels",
        20386,
        0xd54f2a2c76f447b3,
    ),
    DeclarationSeat::new(
        Some(9845),
        "RegisterRemoteRouter",
        61310,
        0xf58be82ac31d8188,
    ),
    DeclarationSeat::new(None, "RouterBootstrapDocument", 36025, 0x18ad8c0db9977cbc),
    DeclarationSeat::new(None, "RouterObservationScope", 2510, 0x63a0a4198aa015cc),
    DeclarationSeat::new(Some(2510), "Summary", 10619, 0xe7de0ce4e40a266b),
    DeclarationSeat::new(Some(2510), "MessageTrace", 56815, 0x696a803632280ba4),
    DeclarationSeat::new(Some(2510), "ChannelState", 25923, 0x886139bbf9affe5e),
    DeclarationSeat::new(None, "Engine", 16301, 0x8c4ed0a2ba5d5b70),
    DeclarationSeat::new(None, "Channel", 57530, 0x704ec979e3f9e403),
    DeclarationSeat::new(None, "RouterSummaryQuery", 9961, 0xd87cfc8bf3aa894b),
    DeclarationSeat::new(None, "RouterMessageTraceQuery", 43298, 0x8dd87e7df65d4f97),
    DeclarationSeat::new(None, "RouterChannelStateQuery", 46198, 0x39b00b6f6bb696aa),
    DeclarationSeat::new(None, "AcceptedMessages", 22495, 0x3e67eb43745af9b8),
    DeclarationSeat::new(None, "RoutedMessages", 57725, 0xb038ac18fcf8f5dd),
    DeclarationSeat::new(None, "DeferredMessages", 13438, 0x4e7defc80f7c1519),
    DeclarationSeat::new(None, "FailedMessages", 54945, 0xe9df70b71a2c6e3e),
    DeclarationSeat::new(None, "RouterSummary", 17891, 0xe10446d4daf50faf),
    DeclarationSeat::new(None, "RouterDeliveryStatus", 6389, 0x578c81c37f0fd747),
    DeclarationSeat::new(Some(6389), "Accepted", 41837, 0x6f6ce5b31b03b02e),
    DeclarationSeat::new(Some(6389), "Routed", 55446, 0xcdaf1b15f3e444be),
    DeclarationSeat::new(Some(6389), "Delivered", 36449, 0x73aff54519f14540),
    DeclarationSeat::new(Some(6389), "Deferred", 13386, 0x5503f02e7e74ed22),
    DeclarationSeat::new(Some(6389), "Failed", 58293, 0xda5cced8aaeb40c5),
    DeclarationSeat::new(Some(6389), "ForwardedRemote", 61819, 0x203c26a19f10b50f),
    DeclarationSeat::new(None, "DeliveryStatus", 15237, 0x03c47d987f566e91),
    DeclarationSeat::new(None, "RouterMessageTrace", 6069, 0xaa4c73f0b0eebb4a),
    DeclarationSeat::new(None, "RouterMessageTraceMissing", 9613, 0x762c995b485853d1),
    DeclarationSeat::new(None, "RouterChannelStatus", 20624, 0x23ea1a2ceb21ca0a),
    DeclarationSeat::new(Some(20624), "Installed", 24992, 0x099596903259b8da),
    DeclarationSeat::new(Some(20624), "Missing", 7367, 0x94651196dfe6fd38),
    DeclarationSeat::new(Some(20624), "Disabled", 55840, 0xd7f25c1071fe7c1c),
    DeclarationSeat::new(None, "ChannelStatus", 22415, 0xf95dadb20f3cd1aa),
    DeclarationSeat::new(None, "RouterChannelState", 41192, 0x4557490aae21a87c),
    DeclarationSeat::new(
        None,
        "RouterObservationUnimplementedReason",
        15510,
        0x15811b1b6e2424de,
    ),
    DeclarationSeat::new(Some(15510), "NotInPrototypeScope", 2431, 0x7652e9b2b27b6515),
    DeclarationSeat::new(
        Some(15510),
        "RouterStoreUnavailable",
        49708,
        0xaf6b5ac5373a6462,
    ),
    DeclarationSeat::new(
        Some(15510),
        "MessageTraceUnavailable",
        53647,
        0x59becd11787daecb,
    ),
    DeclarationSeat::new(None, "ObservationScope", 65497, 0x2f9528c79f86887b),
    DeclarationSeat::new(None, "ObservationReason", 45641, 0x59d23768be1cbc99),
    DeclarationSeat::new(
        None,
        "RouterObservationUnimplemented",
        34818,
        0x5aa96150b777bbaf,
    ),
    DeclarationSeat::new(None, "SignatureScheme", 3829, 0x9eeed8f736abf94d),
    DeclarationSeat::new(Some(3829), "Bls12_381MinPk", 43557, 0xb1622b1fa28e2e19),
    DeclarationSeat::new(Some(3829), "Bls12_381MinSig", 42291, 0xa95b6bca6f2f7382),
    DeclarationSeat::new(None, "Signer", 2360, 0x96e7233c3e35a562),
    DeclarationSeat::new(None, "Scheme", 46098, 0x57ebffe3beb041c2),
    DeclarationSeat::new(None, "PublicKey", 31637, 0xfa3a846b73378ae5),
    DeclarationSeat::new(None, "Signature", 46272, 0xf63b5e8b8ca3732c),
    DeclarationSeat::new(None, "ContentDigest", 58113, 0xf5cf99f220232f8e),
    DeclarationSeat::new(None, "IssuedAt", 58090, 0xee3032013c853bb0),
    DeclarationSeat::new(None, "Nonce", 57367, 0x8223f9e3494533fc),
    DeclarationSeat::new(None, "AttestationIssuedAt", 23255, 0xb553f238a6b6c63d),
    DeclarationSeat::new(None, "RouterPeerAttestation", 37359, 0x58c57e1243b7e1e5),
    DeclarationSeat::new(None, "RoutedContractObject", 57498, 0x613ff42aa04195a2),
    DeclarationSeat::new(None, "Body", 42740, 0xb3702b7d8b27f755),
    DeclarationSeat::new(None, "ForwardedMessagePayload", 9938, 0x19a09fa101a283eb),
    DeclarationSeat::new(None, "ForwardMarker", 5468, 0xf1ef5a38532e023f),
    DeclarationSeat::new(Some(5468), "Origin", 29917, 0x4f799f1feedb6676),
    DeclarationSeat::new(Some(5468), "Forwarded", 19615, 0x9ad31d98fb7fa311),
    DeclarationSeat::new(None, "Submission", 38288, 0x0ca9017a633eb3af),
    DeclarationSeat::new(None, "Attestation", 1169, 0xdd4783bd7a5c7403),
    DeclarationSeat::new(None, "Forwarded", 34129, 0xa65f77b29945c78a),
    DeclarationSeat::new(None, "RouterForwardRequest", 19688, 0x22eeedcd18dd821e),
    DeclarationSeat::new(None, "RouterForwardAccepted", 15161, 0x3f1c9703080adc6d),
    DeclarationSeat::new(
        None,
        "RouterForwardRefusalReason",
        14896,
        0xb08a38d8b9fa692a,
    ),
    DeclarationSeat::new(Some(14896), "UnknownPeer", 39982, 0xb12200a2d506077a),
    DeclarationSeat::new(Some(14896), "AttestationInvalid", 4120, 0x99b48bcb4edfb7c8),
    DeclarationSeat::new(Some(14896), "ReplayDetected", 43802, 0xa1f056b73f8ff466),
    DeclarationSeat::new(Some(14896), "ClockSkew", 39615, 0xdbdf9ce1adb7283d),
    DeclarationSeat::new(Some(14896), "RecipientUnknown", 41237, 0x9209c8b25922c28f),
    DeclarationSeat::new(
        Some(14896),
        "ChannelUnauthorized",
        44917,
        0xb4523f8950414204,
    ),
    DeclarationSeat::new(Some(14896), "AlreadyForwarded", 11179, 0x0308ed7db5559c84),
    DeclarationSeat::new(Some(14896), "MirrorDisabled", 26527, 0x62ab6c355a0cfdba),
    DeclarationSeat::new(Some(14896), "SessionRequired", 64102, 0x113debfe2db5a1df),
    DeclarationSeat::new(None, "ForwardRefusalReason", 8962, 0x0849e8224707d6ce),
    DeclarationSeat::new(None, "RouterForwardRefused", 24390, 0xbe3327b1019f0205),
    DeclarationSeat::new(
        None,
        "RouterRoutedObjectsRefused",
        23085,
        0x59e45c05d0c491b3,
    ),
    DeclarationSeat::new(
        None,
        "RouterRoutedObjectsAccepted",
        15976,
        0x60f7c15c4e205686,
    ),
    DeclarationSeat::new(None, "SessionChallenge", 54463, 0x5708e618d2d8b7f9),
    DeclarationSeat::new(None, "EphemeralPublicKey", 62189, 0xbef8072545a56426),
    DeclarationSeat::new(None, "ProofSigner", 58784, 0x6c476af4a11fbeec),
    DeclarationSeat::new(None, "ProofScheme", 33164, 0xc7b9703f4663ae5e),
    DeclarationSeat::new(None, "ProofPublicKey", 16452, 0x9f6c130e541acebe),
    DeclarationSeat::new(None, "ProofSignature", 52908, 0xa57fa2d08d1bef2a),
    DeclarationSeat::new(None, "ProofDigest", 50100, 0x69bdafe4be30c535),
    DeclarationSeat::new(None, "ProofChallengeNonce", 58018, 0x683bc2d80fe5c749),
    DeclarationSeat::new(None, "ProofAttestationIssuedAt", 29237, 0xa7f86d40088d604b),
    DeclarationSeat::new(None, "RouterIdentityProof", 13113, 0x03c6093e0dc5470b),
    DeclarationSeat::new(None, "InitiatorChallenge", 13794, 0xfe2ecb7275a117f0),
    DeclarationSeat::new(None, "InitiatorEphemeralKey", 49038, 0x0fb0b21eb8d333db),
    DeclarationSeat::new(None, "RouterSessionClientHello", 35585, 0x8310d9fe31ea66d4),
    DeclarationSeat::new(None, "ResponderChallenge", 8622, 0xb670d17bc6da0085),
    DeclarationSeat::new(None, "ResponderEphemeralKey", 34809, 0x4aa9b96669fa5ecc),
    DeclarationSeat::new(None, "ResponderProof", 46283, 0xadbfd2240587df48),
    DeclarationSeat::new(None, "RouterSessionServerHello", 1853, 0xf2da7750067e66dc),
    DeclarationSeat::new(None, "InitiatorProof", 21911, 0xd3c3f164545df7d5),
    DeclarationSeat::new(None, "RouterSessionClientProof", 28558, 0xbd9564ed7d0b0c18),
    DeclarationSeat::new(None, "RouterSessionAccepted", 42924, 0x422fd67740183bdc),
    DeclarationSeat::new(None, "SessionRefusalReason", 32313, 0xade7cc628d02cc6c),
    DeclarationSeat::new(
        Some(32313),
        "IdentityProofInvalid",
        51581,
        0x1ce61869fd4307ea,
    ),
    DeclarationSeat::new(Some(32313), "ChallengeMismatch", 39981, 0x6c6d09c6f79e8f29),
    DeclarationSeat::new(Some(32313), "HandshakeMalformed", 41224, 0x955ce6c21693a302),
    DeclarationSeat::new(
        Some(32313),
        "SessionCipherFailure",
        48282,
        0x01127977d7c4a908,
    ),
    DeclarationSeat::new(None, "RefusalReason", 29111, 0x54866aed8bbabf26),
    DeclarationSeat::new(None, "RouterSessionRefused", 12824, 0x5adae8324e340200),
    DeclarationSeat::new(None, "RouterSessionData", 10752, 0xa28846f0cf2e5c39),
    DeclarationSeat::new(None, "RouterSocketPath", 61509, 0xb22b80687db5e310),
    DeclarationSeat::new(None, "RouterSocketMode", 52756, 0x898c788dec20d000),
    DeclarationSeat::new(None, "MetaRouterSocketPath", 48227, 0xb578b42bac5df1b5),
    DeclarationSeat::new(None, "MetaRouterSocketMode", 5556, 0x0277ed58c764242d),
    DeclarationSeat::new(None, "SupervisionSocketPath", 57540, 0x68aba53ad8464781),
    DeclarationSeat::new(None, "SupervisionSocketMode", 38672, 0x185c1b4afeb19339),
    DeclarationSeat::new(None, "StorePath", 56146, 0x45587c419fab768a),
    DeclarationSeat::new(None, "RouterIdentity", 50143, 0x62ca72a2b77942a8),
    DeclarationSeat::new(None, "RouterDaemonConfiguration", 46751, 0x7954963c28b93dc7),
    DeclarationSeat::new(None, "RouterRequest", 45409, 0x8ad2bf9651935462),
    DeclarationSeat::new(Some(45409), "Summary", 7457, 0x484d6d00d898105d),
    DeclarationSeat::new(Some(45409), "MessageTrace", 37757, 0xdf0fe46dbff5884c),
    DeclarationSeat::new(Some(45409), "ChannelState", 59288, 0x1701c1898b114b40),
    DeclarationSeat::new(Some(45409), "ForwardMessage", 58039, 0x444583c1e7afdf98),
    DeclarationSeat::new(
        Some(45409),
        "SubmitRoutedObjects",
        61216,
        0x397def9b89a41b7e,
    ),
    DeclarationSeat::new(Some(45409), "SessionClientHello", 5383, 0xad31e44444f060f2),
    DeclarationSeat::new(Some(45409), "SessionClientProof", 15459, 0x7438d61ace9afde7),
    DeclarationSeat::new(Some(45409), "SessionData", 58079, 0xc4ea1ad2e136bbf0),
    DeclarationSeat::new(Some(45409), "RegisterActor", 36573, 0xcf5d3d155b9c1fb6),
    DeclarationSeat::new(None, "RouterReply", 40496, 0xd4354261626d0a6e),
    DeclarationSeat::new(Some(40496), "Summary", 57194, 0x58ab0dbf9d928061),
    DeclarationSeat::new(Some(40496), "MessageTrace", 10235, 0x21ffc9c19ff8f2e4),
    DeclarationSeat::new(
        Some(40496),
        "MessageTraceMissing",
        56729,
        0x863e542f9e5b0ca4,
    ),
    DeclarationSeat::new(Some(40496), "ChannelState", 41767, 0x1731bfc77271549d),
    DeclarationSeat::new(Some(40496), "ForwardAccepted", 27511, 0xf44194c9d56ff94a),
    DeclarationSeat::new(Some(40496), "ForwardRefused", 11489, 0x7c9b4e4fa8f1ea1b),
    DeclarationSeat::new(Some(40496), "Unimplemented", 23583, 0xc339a3230271b825),
    DeclarationSeat::new(
        Some(40496),
        "RoutedObjectsAccepted",
        54706,
        0x96073d42f095771b,
    ),
    DeclarationSeat::new(
        Some(40496),
        "RoutedObjectsRefused",
        4159,
        0x47a3a18de7699013,
    ),
    DeclarationSeat::new(Some(40496), "SessionServerHello", 27358, 0xd9e256b39315243b),
    DeclarationSeat::new(Some(40496), "SessionAccepted", 56159, 0xc5cb2c44d67e8e9f),
    DeclarationSeat::new(Some(40496), "SessionRefused", 9873, 0x96708803d20a59b7),
    DeclarationSeat::new(Some(40496), "SessionData", 11977, 0x2b57af8b29657df7),
    DeclarationSeat::new(Some(40496), "ActorRegistered", 62964, 0x99c2fdeb337ce03a),
    DeclarationSeat::new(
        Some(40496),
        "ActorRegistrationRefused",
        31201,
        0x621c8d7dcc8aa301,
    ),
];

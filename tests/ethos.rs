//! Structural proof that Router's shipped Interface is admitted before its
//! current Rust binding is considered.

use core_ethos::bootstrap::{
    BootstrapCatalog, BootstrapGrammarIdentities, BootstrapNamingAuthority,
    BootstrapNamingAuthorityRequest, BootstrapPriorIdentities, BootstrapPriorVocabulary,
    BootstrapReader, BootstrapVersionPolicy, CanonicalIdentityOrder, EthosKind, EthosVersion,
    IdentitySchema, IdentitySchemaCatalog, InterfaceRole, NomosSchema, SchemaRole,
    TextualMetadataRecord, TextualMetadataSnapshot, TextualProjectionAddress,
};
use name_table::LocalEncodedId;
use signal_router::ROUTER_INTERFACE_SOURCE;
use signal_sema_translator::{VocabularyEncodedId, VocabularyRoot};

const MODULE_PATH: &[&str] = &["signal_router", "router"];

fn identity(local: u16) -> VocabularyEncodedId {
    VocabularyEncodedId::new(VocabularyRoot::Universal, vec![LocalEncodedId::new(local)])
        .expect("bootstrap identity is nonempty")
}

#[derive(Clone, Debug)]
struct NoNamingAuthority;

impl BootstrapNamingAuthority for NoNamingAuthority {
    type Proof = ();
    type Receipt = ();

    fn authorize(
        &self,
        _request: BootstrapNamingAuthorityRequest<'_>,
        _proof: &Self::Proof,
    ) -> Option<Self::Receipt> {
        None
    }

    fn verify_receipt(
        &self,
        _request: BootstrapNamingAuthorityRequest<'_>,
        _receipt: &Self::Receipt,
    ) -> bool {
        false
    }
}

struct PriorSeat {
    spelling: &'static str,
    local: u16,
    canonical: u64,
    roles: Vec<SchemaRole>,
}

fn prior_seats() -> Vec<PriorSeat> {
    vec![
        PriorSeat {
            spelling: "Interface",
            local: 2339,
            canonical: 0x69de7fd166248bf9,
            roles: vec![SchemaRole::FileKind(EthosKind::Interface)],
        },
        PriorSeat {
            spelling: "Nexus",
            local: 3235,
            canonical: 0x331959aa1e843299,
            roles: vec![SchemaRole::FileKind(EthosKind::Nexus)],
        },
        PriorSeat {
            spelling: "Sema",
            local: 29501,
            canonical: 0xe2a4350c99ae55dd,
            roles: vec![SchemaRole::FileKind(EthosKind::Sema)],
        },
        PriorSeat {
            spelling: "Input",
            local: 8732,
            canonical: 0x8894b5ef0707c2a2,
            roles: vec![SchemaRole::InterfaceRole(InterfaceRole::Input)],
        },
        PriorSeat {
            spelling: "Output",
            local: 24382,
            canonical: 0xcb74b215d6f48caf,
            roles: vec![SchemaRole::InterfaceRole(InterfaceRole::Output)],
        },
        PriorSeat {
            spelling: "Refusal",
            local: 41050,
            canonical: 0x5845b33dbf737f51,
            roles: vec![SchemaRole::InterfaceRole(InterfaceRole::Refusal)],
        },
        PriorSeat {
            spelling: "String",
            local: 22764,
            canonical: 0x40799e630ab7d6fe,
            roles: vec![SchemaRole::Nominal { persistent: true }],
        },
        PriorSeat {
            spelling: "Integer",
            local: 37512,
            canonical: 0x4d7a0baceb981840,
            roles: vec![SchemaRole::Nominal { persistent: true }],
        },
        PriorSeat {
            spelling: "Boolean",
            local: 5827,
            canonical: 0xa8b18a6031e7376e,
            roles: vec![SchemaRole::Nominal { persistent: true }],
        },
        PriorSeat {
            spelling: "Unit",
            local: 57783,
            canonical: 0x99dee585f7a5ed28,
            roles: vec![SchemaRole::Nominal { persistent: true }],
        },
        PriorSeat {
            spelling: "Vector",
            local: 54192,
            canonical: 0x72811aca319e44ad,
            roles: vec![SchemaRole::Shape { arity: 1 }],
        },
        PriorSeat {
            spelling: "Option",
            local: 53590,
            canonical: 0xfca9637601fb9618,
            roles: vec![SchemaRole::Shape { arity: 1 }],
        },
        PriorSeat {
            spelling: "Map",
            local: 27798,
            canonical: 0x552f219cb39f22ba,
            roles: vec![SchemaRole::Shape { arity: 2 }],
        },
        PriorSeat {
            spelling: "Result",
            local: 28566,
            canonical: 0xf297d74464de032a,
            roles: vec![SchemaRole::Shape { arity: 2 }],
        },
        PriorSeat {
            spelling: "Stream",
            local: 35711,
            canonical: 0x6d525d7c5ec015fa,
            roles: vec![
                SchemaRole::Shape { arity: 1 },
                SchemaRole::Nomos(NomosSchema::StreamInitiation { arity: 2 }),
            ],
        },
        PriorSeat {
            spelling: "StreamIdentity",
            local: 49810,
            canonical: 0x6de2d2bfea0bd258,
            roles: vec![SchemaRole::Shape { arity: 1 }],
        },
    ]
}

fn catalog() -> BootstrapCatalog {
    let seats = prior_seats();
    let metadata = TextualMetadataSnapshot::new(
        seats
            .iter()
            .map(|seat| TextualMetadataRecord {
                address: TextualProjectionAddress {
                    module_path: MODULE_PATH.iter().map(|part| (*part).to_owned()).collect(),
                    lexical_owner: None,
                    visible_name: seat.spelling.to_owned(),
                },
                encoded_name: identity(seat.local),
            })
            .collect(),
    )
    .expect("prior spellings are unique");
    let schemas = IdentitySchemaCatalog::new(
        seats
            .iter()
            .map(|seat| {
                IdentitySchema::new(identity(seat.local), seat.roles.clone())
                    .expect("prior role is admitted")
            })
            .collect(),
    )
    .expect("prior identities are unique");
    let priors = BootstrapPriorVocabulary::new(
        BootstrapPriorIdentities {
            interface_kind: identity(2339),
            nexus_kind: identity(3235),
            sema_kind: identity(29501),
            input_role: identity(8732),
            output_role: identity(24382),
            refusal_role: identity(41050),
            string_type: identity(22764),
            integer_type: identity(37512),
            boolean_type: identity(5827),
            unit_type: identity(57783),
            vector_shape: identity(54192),
            option_shape: identity(53590),
            map_shape: identity(27798),
            result_shape: identity(28566),
            stream_nomos: identity(35711),
            stream_shape: identity(35711),
            stream_identity_shape: identity(49810),
        },
        &schemas,
        &metadata,
    )
    .expect("prior relationships are complete");
    let canonical_order = CanonicalIdentityOrder::new(
        seats
            .iter()
            .map(|seat| (identity(seat.local), seat.canonical.to_be_bytes().to_vec())),
    )
    .expect("prior canonical values are unique");
    BootstrapCatalog::new(
        MODULE_PATH.iter().map(|part| (*part).to_owned()).collect(),
        metadata,
        schemas,
        priors,
        BootstrapVersionPolicy::exact(EthosVersion::new(1, 0, 0)),
        canonical_order,
    )
    .expect("bootstrap catalog is complete")
}

#[test]
fn router_interface_is_admitted_by_the_strict_ethos_reader() {
    let reader = BootstrapReader::build(
        BootstrapGrammarIdentities {
            document: identity(38507),
            syntax: identity(44879),
        },
        catalog(),
        NoNamingAuthority,
    )
    .expect("strict Ethos reader builds");
    let plan = reader
        .plan(ROUTER_INTERFACE_SOURCE)
        .expect("router Interface is structurally valid");
    let names = plan
        .declarations()
        .iter()
        .map(|declaration| declaration.spelling())
        .collect::<Vec<_>>();
    for required in ["Actor", "RegisterActor", "Request", "Reply"] {
        assert!(names.contains(&required), "missing {required}");
    }
}

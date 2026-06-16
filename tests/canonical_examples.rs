#![cfg(feature = "nota-text")]

//! Canonical examples round-trip witness.
//!
//! Parses `examples/canonical.nota` end-to-end, decoding each record
//! as an `Input` or `Output` and asserting the re-encoded
//! text equals the canonical form. Adding a new variant requires
//! adding both a canonical-text example and the matching expected
//! value here; the witness is what keeps the examples file aligned
//! with the typed surface.

use nota_next::{NotaEncode, NotaSource};
use signal_router::{
    ForwardMarker, ForwardedMessagePayload, Input, Output, OwnerIdentity, RegisterRemoteRouter,
    RouterBootstrapOperation, RouterChannelState, RouterChannelStateQuery, RouterChannelStatus,
    RouterDaemonConfiguration, RouterDeliveryStatus, RouterForwardRefusalReason,
    RouterForwardRequest, RouterMessageTrace, RouterMessageTraceMissing, RouterMessageTraceQuery,
    RouterObservationScope, RouterObservationUnimplemented, RouterObservationUnimplementedReason,
    RouterPeerAttestation, RouterSummary, RouterSummaryQuery, SignatureScheme,
};

const CANONICAL: &str = include_str!("../examples/canonical.nota");

fn engine() -> String {
    String::from("prototype")
}

fn channel() -> String {
    String::from("internal-message-router")
}

fn forward_request() -> RouterForwardRequest {
    RouterForwardRequest {
        submission: ForwardedMessagePayload {
            from: String::from("ouranos-mind").into(),
            to: String::from("prometheus-responder").into(),
            body: String::from("hello over the tailnet"),
            attachments: vec![String::from("digest-001")],
        },
        attestation: RouterPeerAttestation {
            signer: String::from("prometheus-router").into(),
            scheme: SignatureScheme::Bls12_381MinPk,
            public_key: String::from("bls-pk-abc"),
            signature: String::from("bls-sig-def"),
            content_digest: String::from("blake3-0011"),
            issued_at: 1_726_000_000_000_000_000u64.into(),
            nonce: String::from("nonce-7f3a").into(),
        },
        forwarded: ForwardMarker::Origin,
        nonce: String::from("nonce-7f3a").into(),
        issued_at: 1_726_000_000_000_000_000u64.into(),
    }
}

#[test]
fn canonical_request_examples_round_trip() {
    let expected: Vec<(Input, &str)> = vec![
        (
            Input::Summary(RouterSummaryQuery::new(engine().into())),
            "(Summary prototype)",
        ),
        (
            Input::MessageTrace(RouterMessageTraceQuery {
                engine: engine().into(),
                message_slot: 7.into(),
            }),
            "(MessageTrace (prototype 7))",
        ),
        (
            Input::ChannelState(RouterChannelStateQuery {
                engine: engine().into(),
                channel: channel().into(),
            }),
            "(ChannelState (prototype internal-message-router))",
        ),
        (
            Input::ForwardMessage(forward_request()),
            "(ForwardMessage ((ouranos-mind prometheus-responder [hello over the tailnet] [digest-001]) (prometheus-router Bls12_381MinPk bls-pk-abc bls-sig-def blake3-0011 1726000000000000000 nonce-7f3a) Origin nonce-7f3a 1726000000000000000))",
        ),
    ];

    for (value, canonical_text) in expected {
        let text = value.to_nota();
        assert_eq!(text, canonical_text, "encode for {value:?}");

        let decoded = NotaSource::new(canonical_text)
            .parse::<Input>()
            .expect("decode");
        assert_eq!(decoded, value, "decode for {canonical_text}");

        assert!(
            CANONICAL.contains(canonical_text),
            "examples/canonical.nota missing line: {canonical_text}",
        );
    }
}

#[test]
fn canonical_reply_examples_round_trip() {
    let expected: Vec<(Output, &str)> = vec![
        (
            Output::Summary(RouterSummary {
                engine: engine().into(),
                accepted_messages: 1,
                routed_messages: 1,
                deferred_messages: 0,
                failed_messages: 0,
            }),
            "(Summary (prototype 1 1 0 0))",
        ),
        (
            Output::MessageTrace(RouterMessageTrace {
                engine: engine().into(),
                message_slot: 7.into(),
                status: RouterDeliveryStatus::Routed,
            }),
            "(MessageTrace (prototype 7 Routed))",
        ),
        (
            Output::MessageTraceMissing(RouterMessageTraceMissing {
                engine: engine().into(),
                message_slot: 99.into(),
            }),
            "(MessageTraceMissing (prototype 99))",
        ),
        (
            Output::ChannelState(RouterChannelState {
                engine: engine().into(),
                channel: channel().into(),
                status: RouterChannelStatus::Installed,
            }),
            "(ChannelState (prototype internal-message-router Installed))",
        ),
        (Output::forward_accepted(7.into()), "(ForwardAccepted 7)"),
        (
            Output::forward_refused(RouterForwardRefusalReason::UnknownPeer),
            "(ForwardRefused UnknownPeer)",
        ),
        (
            Output::forward_refused(RouterForwardRefusalReason::AttestationInvalid),
            "(ForwardRefused AttestationInvalid)",
        ),
        (
            Output::forward_refused(RouterForwardRefusalReason::ReplayDetected),
            "(ForwardRefused ReplayDetected)",
        ),
        (
            Output::forward_refused(RouterForwardRefusalReason::ClockSkew),
            "(ForwardRefused ClockSkew)",
        ),
        (
            Output::forward_refused(RouterForwardRefusalReason::RecipientUnknown),
            "(ForwardRefused RecipientUnknown)",
        ),
        (
            Output::forward_refused(RouterForwardRefusalReason::ChannelUnauthorized),
            "(ForwardRefused ChannelUnauthorized)",
        ),
        (
            Output::forward_refused(RouterForwardRefusalReason::AlreadyForwarded),
            "(ForwardRefused AlreadyForwarded)",
        ),
        (
            Output::Unimplemented(RouterObservationUnimplemented {
                scope: RouterObservationScope::Summary,
                reason: RouterObservationUnimplementedReason::NotInPrototypeScope,
            }),
            "(Unimplemented (Summary NotInPrototypeScope))",
        ),
    ];

    for (value, canonical_text) in expected {
        let text = value.to_nota();
        assert_eq!(text, canonical_text, "encode for {value:?}");

        let decoded = NotaSource::new(canonical_text)
            .parse::<Output>()
            .expect("decode");
        assert_eq!(decoded, value, "decode for {canonical_text}");

        assert!(
            CANONICAL.contains(canonical_text),
            "examples/canonical.nota missing line: {canonical_text}",
        );
    }
}

#[test]
fn canonical_register_remote_router_bootstrap_example_round_trips() {
    let operation = RouterBootstrapOperation::RegisterRemoteRouter(RegisterRemoteRouter {
        identity: String::from("prometheus-router").into(),
        address: String::from("[201:abcd::2]:9930").into(),
    });
    let canonical_text = "(RegisterRemoteRouter (prometheus-router [|[201:abcd::2]:9930|]))";

    let text = operation.to_nota();
    assert_eq!(text, canonical_text, "encode for {operation:?}");

    let decoded = NotaSource::new(canonical_text)
        .parse::<RouterBootstrapOperation>()
        .expect("decode");
    assert_eq!(decoded, operation, "decode for {canonical_text}");

    assert!(
        CANONICAL.contains(canonical_text),
        "examples/canonical.nota missing line: {canonical_text}",
    );
}

#[test]
fn canonical_extended_daemon_configuration_example_round_trips() {
    let configuration = RouterDaemonConfiguration {
        router_socket_path: String::from("/run/persona/X/router.sock").into(),
        router_socket_mode: 0o600.into(),
        meta_router_socket_path: String::from("/run/persona/X/router-meta.sock").into(),
        meta_router_socket_mode: 0o600.into(),
        supervision_socket_path: String::from("/run/persona/X/router-supervision.sock").into(),
        supervision_socket_mode: 0o600.into(),
        store_path: String::from("/var/lib/persona/X/router.sema").into(),
        bootstrap_path: Some(String::from("/var/lib/persona/X/router-bootstrap.nota").into()),
        owner_identity: OwnerIdentity::UnixUser(1000.into()),
        tailnet_listen_address: Some(String::from("[200:1234::1]:9930").into()),
        router_identity: String::from("ouranos-router").into(),
        criome_socket_path: Some(String::from("/run/persona/X/criome.sock").into()),
    };
    let canonical_text = "(/run/persona/X/router.sock 384 /run/persona/X/router-meta.sock 384 /run/persona/X/router-supervision.sock 384 /var/lib/persona/X/router.sema (Some /var/lib/persona/X/router-bootstrap.nota) (UnixUser 1000) (Some [|[200:1234::1]:9930|]) ouranos-router (Some /run/persona/X/criome.sock))";

    let text = configuration.to_nota();
    assert_eq!(text, canonical_text, "encode for {configuration:?}");

    let decoded = NotaSource::new(canonical_text)
        .parse::<RouterDaemonConfiguration>()
        .expect("decode");
    assert_eq!(decoded, configuration, "decode for {canonical_text}");

    assert!(
        CANONICAL.contains(canonical_text),
        "examples/canonical.nota missing the extended router daemon configuration example",
    );
}

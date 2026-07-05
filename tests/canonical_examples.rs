#![cfg(feature = "nota-text")]

//! Canonical examples round-trip witness.
//!
//! Parses `examples/canonical.nota` end-to-end, decoding each record
//! as an `Input` or `Output` and asserting the re-encoded
//! text equals the canonical form. Adding a new variant requires
//! adding both a canonical-text example and the matching expected
//! value here; the witness is what keeps the examples file aligned
//! with the typed surface.

use nota::{NotaEncode, NotaSource};
use signal_router::{
    AttestationIssuedAt, Channel, ChannelIdentifier, ContentDigest, CriomeHostId, Engine,
    EngineIdentifier, ForwardMarker, ForwardedMessagePayload, Input, IssuedAt, Nonce, Output,
    OwnerIdentity, PublicKey, RegisterRemoteRouter, ReplayNonce, RoutedContractObject,
    RouterBootstrapOperation, RouterChannelState, RouterChannelStateQuery, RouterChannelStatus,
    RouterDaemonConfiguration, RouterDaemonConfigurationParts, RouterDeliveryStatus,
    RouterForwardRefusalReason, RouterForwardRequest, RouterMessageTrace,
    RouterMessageTraceMissing, RouterMessageTraceQuery, RouterObservationScope,
    RouterObservationUnimplemented, RouterObservationUnimplementedReason, RouterPeerAttestation,
    RouterSummary, RouterSummaryQuery, Signature, SignatureScheme, TailnetAddress, TimestampNanos,
};

const CANONICAL: &str = include_str!("../examples/canonical.nota");

fn engine() -> String {
    String::from("prototype")
}

fn engine_ref() -> Engine {
    EngineIdentifier::new(engine()).into()
}

fn channel() -> String {
    String::from("internal-message-router")
}

fn channel_ref() -> Channel {
    ChannelIdentifier::new(channel()).into()
}

fn forward_request() -> RouterForwardRequest {
    RouterForwardRequest {
        submission: ForwardedMessagePayload::new(
            String::from("ouranos-mind").into(),
            String::from("prometheus-responder").into(),
            String::from("hello over the tailnet"),
            vec![String::from("digest-001")],
            Vec::new(),
        )
        .into(),
        attestation: RouterPeerAttestation {
            signer: CriomeHostId::new("prometheus-router").into(),
            scheme: SignatureScheme::Bls12_381MinPk.into(),
            public_key: PublicKey::new("bls-pk-abc"),
            signature: Signature::new("bls-sig-def"),
            content_digest: ContentDigest::new("blake3-0011"),
            issued_at: IssuedAt::new(TimestampNanos::new(1_726_000_000_000_000_000)),
            nonce: Nonce::new(ReplayNonce::new("nonce-7f3a")),
            attestation_issued_at: AttestationIssuedAt::new(TimestampNanos::new(
                1_726_000_000_000_000_500,
            )),
        }
        .into(),
        forwarded: ForwardMarker::Origin.into(),
        nonce: ReplayNonce::new("nonce-7f3a").into(),
        issued_at: TimestampNanos::new(1_726_000_000_000_000_000).into(),
    }
}

fn routed_object_submission() -> ForwardedMessagePayload {
    ForwardedMessagePayload::new(
        String::from("spirit").into(),
        String::from("spirit-peer").into(),
        String::from("mirror-append"),
        Vec::new(),
        vec![RoutedContractObject::new(
            String::from("signal-mirror").into(),
            String::from("NotifyObject").into(),
            3.into(),
            vec![1, 2, 3],
        )],
    )
}

#[test]
fn canonical_request_examples_round_trip() {
    let expected: Vec<(Input, &str)> = vec![
        (
            Input::Summary(RouterSummaryQuery::new(engine_ref())),
            "(Summary prototype)",
        ),
        (
            Input::MessageTrace(RouterMessageTraceQuery {
                engine: engine_ref(),
                message_slot: 7.into(),
            }),
            "(MessageTrace (prototype 7))",
        ),
        (
            Input::ChannelState(RouterChannelStateQuery {
                engine: engine_ref(),
                channel: channel_ref(),
            }),
            "(ChannelState (prototype internal-message-router))",
        ),
        (
            Input::ForwardMessage(forward_request()),
            "(ForwardMessage ((ouranos-mind prometheus-responder [hello over the tailnet] [digest-001] []) (prometheus-router Bls12_381MinPk bls-pk-abc bls-sig-def blake3-0011 1726000000000000000 nonce-7f3a 1726000000000000500) Origin nonce-7f3a 1726000000000000000))",
        ),
        (
            Input::SubmitRoutedObjects(routed_object_submission()),
            "(SubmitRoutedObjects (spirit spirit-peer mirror-append [] [(signal-mirror NotifyObject 3 [1 2 3])]))",
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
                engine: engine_ref(),
                accepted_messages: 1.into(),
                routed_messages: 1.into(),
                deferred_messages: 0.into(),
                failed_messages: 0.into(),
            }),
            "(Summary (prototype 1 1 0 0))",
        ),
        (
            Output::MessageTrace(RouterMessageTrace {
                engine: engine_ref(),
                message_slot: 7.into(),
                delivery_status: RouterDeliveryStatus::Routed.into(),
            }),
            "(MessageTrace (prototype 7 Routed))",
        ),
        (
            Output::MessageTraceMissing(RouterMessageTraceMissing {
                engine: engine_ref(),
                message_slot: 99.into(),
            }),
            "(MessageTraceMissing (prototype 99))",
        ),
        (
            Output::ChannelState(RouterChannelState {
                engine: engine_ref(),
                channel: channel_ref(),
                channel_status: RouterChannelStatus::Installed.into(),
            }),
            "(ChannelState (prototype internal-message-router Installed))",
        ),
        (Output::forward_accepted(7.into()), "(ForwardAccepted 7)"),
        (
            Output::forward_refused(RouterForwardRefusalReason::UnknownPeer.into()),
            "(ForwardRefused UnknownPeer)",
        ),
        (
            Output::forward_refused(RouterForwardRefusalReason::AttestationInvalid.into()),
            "(ForwardRefused AttestationInvalid)",
        ),
        (
            Output::forward_refused(RouterForwardRefusalReason::ReplayDetected.into()),
            "(ForwardRefused ReplayDetected)",
        ),
        (
            Output::forward_refused(RouterForwardRefusalReason::ClockSkew.into()),
            "(ForwardRefused ClockSkew)",
        ),
        (
            Output::forward_refused(RouterForwardRefusalReason::RecipientUnknown.into()),
            "(ForwardRefused RecipientUnknown)",
        ),
        (
            Output::forward_refused(RouterForwardRefusalReason::ChannelUnauthorized.into()),
            "(ForwardRefused ChannelUnauthorized)",
        ),
        (
            Output::forward_refused(RouterForwardRefusalReason::AlreadyForwarded.into()),
            "(ForwardRefused AlreadyForwarded)",
        ),
        (
            Output::forward_refused(RouterForwardRefusalReason::MirrorDisabled.into()),
            "(ForwardRefused MirrorDisabled)",
        ),
        (
            Output::forward_refused(RouterForwardRefusalReason::SessionRequired.into()),
            "(ForwardRefused SessionRequired)",
        ),
        (
            Output::Unimplemented(RouterObservationUnimplemented {
                observation_scope: RouterObservationScope::Summary.into(),
                observation_reason: RouterObservationUnimplementedReason::NotInPrototypeScope
                    .into(),
            }),
            "(Unimplemented (Summary NotInPrototypeScope))",
        ),
        (
            Output::routed_objects_accepted(11.into()),
            "(RoutedObjectsAccepted 11)",
        ),
        (
            Output::routed_objects_refused(RouterForwardRefusalReason::MirrorDisabled.into()),
            "(RoutedObjectsRefused MirrorDisabled)",
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
        identity: CriomeHostId::new("prometheus-router").into(),
        address: TailnetAddress::new("[201:abcd::2]:9930").into(),
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
    let configuration = RouterDaemonConfiguration::from(RouterDaemonConfigurationParts {
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
    });
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

#[cfg(feature = "nota-text")]
use nota::{NotaDecode, NotaEncode, NotaSource};
use signal_frame::{
    ExchangeIdentifier, ExchangeLane, LaneSequence, NonEmpty, Reply, RequestPayload, SessionEpoch,
    SignalOperationHeads, SubReply,
};
use signal_router::{
    Actor as RuntimeActor, ActorIdentifier as RuntimeActorIdentifier, ActorRegistered,
    ActorRegistrationDisposition, ActorRegistrationRefusalReason, ActorRegistrationRefused,
    EndpointKind as RuntimeEndpointKind, EndpointTransport as RuntimeEndpointTransport,
};
#[cfg(feature = "nota-text")]
use signal_router::{
    Actor, ActorIdentifier, DestinationActor, EndpointKind, EndpointTransport, GrantDirectMessage,
    RegisterActor, RegisterRemoteRouter, RouterBootstrapDocument, RouterBootstrapOperation,
    RouterObservationScope, SourceActor, TailnetAddress,
};
use signal_router::{
    AttestationIssuedAt, Channel, ChannelIdentifier, ContentDigest, CriomeHostId, Engine,
    EngineIdentifier, ForwardMarker, ForwardedMessagePayload, Frame, FrameBody, Input, IssuedAt,
    Nonce, Output, OwnerIdentity, PublicKey, ReplayNonce, RoutedContractObject, RouterChannelState,
    RouterChannelStateQuery, RouterChannelStatus, RouterDaemonConfiguration,
    RouterDaemonConfigurationParts, RouterDeliveryStatus, RouterForwardRefusalReason,
    RouterForwardRequest, RouterMessageTrace, RouterMessageTraceMissing, RouterMessageTraceQuery,
    RouterPeerAttestation, RouterSummary, RouterSummaryQuery, Signature, SignatureScheme,
    TimestampNanos,
};
use signal_router::{
    EphemeralPublicKey, RouterIdentityProof, RouterSessionAccepted, RouterSessionClientHello,
    RouterSessionClientProof, RouterSessionData, RouterSessionRefused, RouterSessionServerHello,
    SessionChallenge, SessionRefusalReason,
};

fn exchange() -> ExchangeIdentifier {
    ExchangeIdentifier::new(
        SessionEpoch::new(1),
        ExchangeLane::Connector,
        LaneSequence::first(),
    )
}

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

#[cfg(feature = "nota-text")]
fn actor(name: &str) -> String {
    String::from(name)
}

#[cfg(feature = "nota-text")]
fn actor_ref(name: &str) -> ActorIdentifier {
    ActorIdentifier::new(actor(name))
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

fn mirror_forward_request() -> RouterForwardRequest {
    let mut request = forward_request();
    let mut submission = request.submission.into_payload();
    submission.push_routed_object(mirror_object());
    request.submission = submission.into();
    request
}

fn mirror_object() -> RoutedContractObject {
    let payload = vec![
        0x91, 0x26, 0xec, 0xcb, 0xb5, 0x00, 0x00, 0x00, b's', b'p', b'i', b'r', b'i', b't', 0x00,
        0x00, 0x07, 0x42,
    ];
    RoutedContractObject::new(
        String::from("signal-mirror").into(),
        String::from("NotifyObject").into(),
        u64::try_from(payload.len())
            .expect("payload size fits")
            .into(),
        payload.into_iter().map(u64::from).collect(),
    )
}

fn routed_object_submission() -> ForwardedMessagePayload {
    ForwardedMessagePayload::new(
        String::from("spirit").into(),
        String::from("spirit-peer").into(),
        String::from("mirror-append"),
        Vec::new(),
        vec![mirror_object()],
    )
}

fn round_trip_request(request: Input) {
    let frame = Frame::new(FrameBody::Request {
        exchange: exchange(),
        request: request.clone().into_request(),
    });

    let bytes = frame.encode_length_prefixed().expect("encode");
    let decoded = Frame::decode_length_prefixed(&bytes).expect("decode");

    match decoded.into_body() {
        FrameBody::Request {
            request: decoded_request,
            ..
        } => {
            assert_eq!(decoded_request.payloads().head(), &request);
        }
        other => panic!("expected router request, got {other:?}"),
    }
}

fn round_trip_reply(reply: Output) -> Output {
    let frame = Frame::new(FrameBody::Reply {
        exchange: exchange(),
        reply: Reply::committed(NonEmpty::single(SubReply::Ok(reply))),
    });

    let bytes = frame.encode_length_prefixed().expect("encode");
    let decoded = Frame::decode_length_prefixed(&bytes).expect("decode");

    match decoded.into_body() {
        FrameBody::Reply { reply, .. } => match reply {
            Reply::Accepted { per_operation, .. } => match per_operation.into_head() {
                SubReply::Ok(payload) => payload,
                other => panic!("expected accepted reply payload, got {other:?}"),
            },
            other => panic!("expected accepted reply, got {other:?}"),
        },
        other => panic!("expected reply, got {other:?}"),
    }
}

#[cfg(feature = "nota-text")]
fn round_trip_nota<Value>(value: Value, expected: &str)
where
    Value: NotaEncode + NotaDecode + PartialEq + std::fmt::Debug,
{
    let text = value.to_nota();
    assert_eq!(text, expected);
    let recovered = NotaSource::new(&text).parse::<Value>().expect("decode");
    assert_eq!(recovered, value);
}

#[test]
fn router_summary_query_round_trips_through_length_prefixed_frame() {
    round_trip_request(Input::Summary(RouterSummaryQuery::new(engine_ref())));
}

#[test]
fn router_message_trace_query_round_trips_through_length_prefixed_frame() {
    round_trip_request(Input::MessageTrace(RouterMessageTraceQuery {
        engine: engine_ref(),
        message_slot: 7.into(),
    }));
}

#[test]
fn router_channel_state_query_round_trips_through_length_prefixed_frame() {
    round_trip_request(Input::ChannelState(RouterChannelStateQuery {
        engine: engine_ref(),
        channel: channel_ref(),
    }));
}

#[test]
fn router_forward_request_round_trips_through_length_prefixed_frame() {
    round_trip_request(Input::ForwardMessage(forward_request()));
}

#[test]
fn router_forward_request_carries_contract_object_octets_without_decoding_them() {
    let request = Input::ForwardMessage(mirror_forward_request());
    let frame = Frame::new(FrameBody::Request {
        exchange: exchange(),
        request: request.into_request(),
    });
    let bytes = frame.encode_length_prefixed().expect("encode router frame");
    let decoded = Frame::decode_length_prefixed(&bytes).expect("decode router frame");

    let FrameBody::Request { request, .. } = decoded.into_body() else {
        panic!("expected router request");
    };
    let Input::ForwardMessage(forward) = request.payloads().head() else {
        panic!("expected forward request");
    };
    let object = forward
        .submission
        .payload()
        .routed_objects()
        .first()
        .expect("forward carries routed object");

    assert_eq!(object.contract_name.payload().as_str(), "signal-mirror");
    assert_eq!(object.contract_operation.payload().as_str(), "NotifyObject");
    assert_eq!(
        usize::try_from(*object.contract_payload_size.payload()).expect("payload size fits"),
        object.payload_octets().len()
    );
    assert_eq!(
        object.payload_octets(),
        vec![
            0x91, 0x26, 0xec, 0xcb, 0xb5, 0, 0, 0, b's', b'p', b'i', b'r', b'i', b't', 0, 0, 7,
            0x42,
        ]
        .into_iter()
        .map(u64::from)
        .collect::<Vec<_>>()
        .as_slice()
    );
}

#[cfg(feature = "nota-text")]
#[test]
fn router_forward_request_round_trips_through_nota_text() {
    round_trip_nota(
        Input::ForwardMessage(forward_request()),
        "(ForwardMessage ((ouranos-mind prometheus-responder [hello over the tailnet] [digest-001] []) (prometheus-router Bls12_381MinPk bls-pk-abc bls-sig-def blake3-0011 1726000000000000000 nonce-7f3a 1726000000000000500) Origin nonce-7f3a 1726000000000000000))",
    );
}

#[test]
fn router_request_heads_are_contract_local_operations() {
    assert_eq!(
        <Input as SignalOperationHeads>::HEADS,
        &[
            "Summary",
            "MessageTrace",
            "ChannelState",
            "ForwardMessage",
            "SubmitRoutedObjects",
            "SessionClientHello",
            "SessionClientProof",
            "SessionData",
            "RegisterActor"
        ]
    );
}

#[test]
fn router_contract_has_no_sema_classification_dependency_or_roots() {
    let manifest = include_str!("../Cargo.toml");
    assert!(
        !manifest.contains("signal-sema"),
        "ordinary signal contracts must not depend on signal-sema for public wire vocabulary"
    );

    let heads = <Input as SignalOperationHeads>::HEADS;
    for forbidden in [
        "Assert",
        "Mutate",
        "Retract",
        "Match",
        "Subscribe",
        "Validate",
    ] {
        assert!(
            !heads.contains(&forbidden),
            "Sema classification root {forbidden} must not appear on the public router wire"
        );
    }
}

fn runtime_register_actor_input() -> Input {
    Input::RegisterActor(RuntimeActor::new(
        RuntimeActorIdentifier::new("orchestrate"),
        4242,
        Some(RuntimeEndpointTransport::new(
            RuntimeEndpointKind::ComponentSocket,
            String::from("/run/persona/X/orchestrate.sock"),
            None,
        )),
    ))
}

#[test]
fn runtime_register_actor_request_round_trips_through_length_prefixed_frame() {
    round_trip_request(runtime_register_actor_input());
}

#[test]
fn runtime_actor_registered_reply_round_trips_through_length_prefixed_frame() {
    for disposition in [
        ActorRegistrationDisposition::Registered,
        ActorRegistrationDisposition::EndpointUpdated,
    ] {
        let reply = Output::ActorRegistered(ActorRegistered::new(
            RuntimeActorIdentifier::new("orchestrate"),
            disposition,
        ));
        assert_eq!(round_trip_reply(reply.clone()), reply);
    }
}

#[test]
fn runtime_actor_registration_refused_reply_round_trips_through_length_prefixed_frame() {
    for reason in [
        ActorRegistrationRefusalReason::ProcessIdentifierOutOfRange,
        ActorRegistrationRefusalReason::RemoteRouterEndpointNotLocal,
    ] {
        let reply = Output::ActorRegistrationRefused(ActorRegistrationRefused::new(
            RuntimeActorIdentifier::new("orchestrate"),
            reason,
        ));
        assert_eq!(round_trip_reply(reply.clone()), reply);
    }
}

#[test]
fn router_summary_reply_round_trips_through_length_prefixed_frame() {
    let reply = Output::Summary(RouterSummary {
        engine: engine_ref(),
        accepted_messages: 1.into(),
        routed_messages: 1.into(),
        deferred_messages: 0.into(),
        failed_messages: 0.into(),
    });
    assert_eq!(round_trip_reply(reply.clone()), reply);
}

#[test]
fn router_message_trace_reply_round_trips_through_length_prefixed_frame() {
    let reply = Output::MessageTrace(RouterMessageTrace {
        engine: engine_ref(),
        message_slot: 7.into(),
        delivery_status: RouterDeliveryStatus::Routed.into(),
    });
    assert_eq!(round_trip_reply(reply.clone()), reply);
}

#[test]
fn router_channel_state_reply_round_trips_through_length_prefixed_frame() {
    let reply = Output::ChannelState(RouterChannelState {
        engine: engine_ref(),
        channel: channel_ref(),
        channel_status: RouterChannelStatus::Installed.into(),
    });
    assert_eq!(round_trip_reply(reply.clone()), reply);
}

#[test]
fn router_message_trace_missing_reply_round_trips_through_length_prefixed_frame() {
    let reply = Output::MessageTraceMissing(RouterMessageTraceMissing {
        engine: engine_ref(),
        message_slot: 99.into(),
    });
    assert_eq!(round_trip_reply(reply.clone()), reply);
}

#[test]
fn router_forward_accepted_reply_round_trips_through_length_prefixed_frame() {
    let reply = Output::forward_accepted(7.into());
    assert_eq!(round_trip_reply(reply.clone()), reply);
}

#[test]
fn router_origin_submission_round_trips_through_length_prefixed_frame() {
    round_trip_request(Input::SubmitRoutedObjects(routed_object_submission()));
}

#[test]
fn routed_objects_accepted_reply_round_trips_through_length_prefixed_frame() {
    let reply = Output::routed_objects_accepted(11.into());
    assert_eq!(round_trip_reply(reply.clone()), reply);
}

#[test]
fn routed_objects_refused_reply_round_trips_through_length_prefixed_frame() {
    let reply = Output::routed_objects_refused(RouterForwardRefusalReason::MirrorDisabled.into());
    assert_eq!(round_trip_reply(reply.clone()), reply);
}

#[cfg(feature = "nota-text")]
#[test]
fn router_forward_accepted_reply_round_trips_through_nota_text() {
    round_trip_nota(Output::forward_accepted(7.into()), "(ForwardAccepted 7)");
}

#[test]
fn router_forward_refused_reply_round_trips_through_length_prefixed_frame_for_every_reason() {
    for reason in [
        RouterForwardRefusalReason::UnknownPeer,
        RouterForwardRefusalReason::AttestationInvalid,
        RouterForwardRefusalReason::ReplayDetected,
        RouterForwardRefusalReason::ClockSkew,
        RouterForwardRefusalReason::RecipientUnknown,
        RouterForwardRefusalReason::ChannelUnauthorized,
        RouterForwardRefusalReason::AlreadyForwarded,
        RouterForwardRefusalReason::MirrorDisabled,
        RouterForwardRefusalReason::SessionRequired,
    ] {
        let reply = Output::forward_refused(reason.into());
        assert_eq!(round_trip_reply(reply.clone()), reply);
    }
}

#[test]
fn router_forward_refusal_reason_is_closed_and_exhaustive() {
    for reason in [
        RouterForwardRefusalReason::UnknownPeer,
        RouterForwardRefusalReason::AttestationInvalid,
        RouterForwardRefusalReason::ReplayDetected,
        RouterForwardRefusalReason::ClockSkew,
        RouterForwardRefusalReason::RecipientUnknown,
        RouterForwardRefusalReason::ChannelUnauthorized,
        RouterForwardRefusalReason::AlreadyForwarded,
        RouterForwardRefusalReason::MirrorDisabled,
        RouterForwardRefusalReason::SessionRequired,
    ] {
        let observed = match reason {
            RouterForwardRefusalReason::UnknownPeer => "unknown-peer",
            RouterForwardRefusalReason::AttestationInvalid => "attestation-invalid",
            RouterForwardRefusalReason::ReplayDetected => "replay-detected",
            RouterForwardRefusalReason::ClockSkew => "clock-skew",
            RouterForwardRefusalReason::RecipientUnknown => "recipient-unknown",
            RouterForwardRefusalReason::ChannelUnauthorized => "channel-unauthorized",
            RouterForwardRefusalReason::AlreadyForwarded => "already-forwarded",
            RouterForwardRefusalReason::MirrorDisabled => "mirror-disabled",
            RouterForwardRefusalReason::SessionRequired => "session-required",
        };
        assert!(!observed.is_empty());
    }
}

#[test]
fn forward_marker_is_closed_origin_or_forwarded() {
    for marker in [ForwardMarker::Origin, ForwardMarker::Forwarded] {
        let observed = match marker {
            ForwardMarker::Origin => "origin",
            ForwardMarker::Forwarded => "forwarded",
        };
        assert!(!observed.is_empty());
    }
}

#[test]
fn router_status_enums_are_closed_no_unknown_variants() {
    for status in [
        RouterDeliveryStatus::Accepted,
        RouterDeliveryStatus::Routed,
        RouterDeliveryStatus::Delivered,
        RouterDeliveryStatus::Deferred,
        RouterDeliveryStatus::Failed,
        RouterDeliveryStatus::ForwardedRemote,
    ] {
        let observed = match status {
            RouterDeliveryStatus::Accepted => "accepted",
            RouterDeliveryStatus::Routed => "routed",
            RouterDeliveryStatus::Delivered => "delivered",
            RouterDeliveryStatus::Deferred => "deferred",
            RouterDeliveryStatus::Failed => "failed",
            RouterDeliveryStatus::ForwardedRemote => "forwarded-remote",
        };
        assert!(!observed.is_empty());
    }
    for status in [
        RouterChannelStatus::Installed,
        RouterChannelStatus::Missing,
        RouterChannelStatus::Disabled,
    ] {
        let observed = match status {
            RouterChannelStatus::Installed => "installed",
            RouterChannelStatus::Missing => "missing",
            RouterChannelStatus::Disabled => "disabled",
        };
        assert!(!observed.is_empty());
    }
}

#[cfg(feature = "nota-text")]
#[test]
fn router_daemon_configuration_round_trips_through_nota_text() {
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

    let text = configuration.to_nota();
    let recovered = NotaSource::new(&text)
        .parse::<RouterDaemonConfiguration>()
        .expect("decode configuration");

    assert_eq!(recovered, configuration);
    assert!(text.contains("/run/persona/X/router.sock"));
    assert!(text.contains("[|[200:1234::1]:9930|]"));
}

#[test]
fn router_daemon_configuration_round_trips_through_rkyv() {
    let configuration = RouterDaemonConfiguration::from(RouterDaemonConfigurationParts {
        router_socket_path: String::from("/run/persona/X/router.sock").into(),
        router_socket_mode: 0o600.into(),
        meta_router_socket_path: String::from("/run/persona/X/router-meta.sock").into(),
        meta_router_socket_mode: 0o600.into(),
        supervision_socket_path: String::from("/run/persona/X/router-supervision.sock").into(),
        supervision_socket_mode: 0o600.into(),
        store_path: String::from("/var/lib/persona/X/router.sema").into(),
        bootstrap_path: None,
        owner_identity: OwnerIdentity::UnixUser(1000.into()),
        tailnet_listen_address: None,
        router_identity: String::from("ouranos-router").into(),
        criome_socket_path: None,
    });

    let bytes = configuration.to_rkyv_bytes().expect("archive");
    let recovered = RouterDaemonConfiguration::from_rkyv_bytes(&bytes).expect("decode rkyv");
    assert_eq!(recovered, configuration);
}

#[test]
fn single_host_router_configuration_has_no_tailnet_listen_address() {
    let configuration = RouterDaemonConfiguration::from(RouterDaemonConfigurationParts {
        router_socket_path: String::from("/run/persona/X/router.sock").into(),
        router_socket_mode: 0o600.into(),
        meta_router_socket_path: String::from("/run/persona/X/router-meta.sock").into(),
        meta_router_socket_mode: 0o600.into(),
        supervision_socket_path: String::from("/run/persona/X/router-supervision.sock").into(),
        supervision_socket_mode: 0o600.into(),
        store_path: String::from("/var/lib/persona/X/router.sema").into(),
        bootstrap_path: None,
        owner_identity: OwnerIdentity::UnixUser(1000.into()),
        tailnet_listen_address: None,
        router_identity: String::from("solo-router").into(),
        criome_socket_path: None,
    });

    assert!(
        configuration.tailnet_listen_address().is_none(),
        "an absent tailnet listen address keeps the router local-only with no TCP forwarding tier"
    );

    let bytes = configuration.to_rkyv_bytes().expect("archive");
    let recovered = RouterDaemonConfiguration::from_rkyv_bytes(&bytes).expect("decode rkyv");
    assert_eq!(recovered, configuration);
}

#[cfg(feature = "nota-text")]
fn endpoint_transport(path: &str) -> EndpointTransport {
    EndpointTransport::new(EndpointKind::HarnessSocket, String::from(path), None)
}

#[cfg(feature = "nota-text")]
#[test]
fn bootstrap_register_actor_operation_round_trips_through_nota_line() {
    let operation = RouterBootstrapOperation::RegisterActor(RegisterActor::new(
        Actor::new(
            actor("responder").into(),
            42,
            Some(endpoint_transport("/tmp/responder.harness.sock")),
        ),
        None,
    ));

    let text = operation.to_nota();
    assert_eq!(
        text,
        "(RegisterActor ((responder 42 (Some (HarnessSocket /tmp/responder.harness.sock None))) None))"
    );
    assert_eq!(
        RouterBootstrapOperation::from_nota(&text).expect("decode bootstrap operation"),
        operation
    );
}

#[cfg(feature = "nota-text")]
#[test]
fn bootstrap_register_actor_with_remote_home_round_trips_through_nota_line() {
    // A remotely-homed actor: home names the peer router it lives behind, so
    // the local router records it in the remote-route table rather than the
    // harness registry. This is how a router learns the recipient's host.
    let operation = RouterBootstrapOperation::RegisterActor(RegisterActor::new(
        Actor::new(
            actor("responder").into(),
            42,
            Some(endpoint_transport("/tmp/responder.harness.sock")),
        ),
        Some(String::from("prometheus-router").into()),
    ));

    let text = operation.to_nota();
    assert_eq!(
        text,
        "(RegisterActor ((responder 42 (Some (HarnessSocket /tmp/responder.harness.sock None))) (Some prometheus-router)))"
    );
    assert_eq!(
        RouterBootstrapOperation::from_nota(&text).expect("decode bootstrap operation"),
        operation
    );
}

#[cfg(feature = "nota-text")]
#[test]
fn bootstrap_direct_message_grant_operation_round_trips_through_nota_line() {
    let operation = RouterBootstrapOperation::GrantDirectMessage(GrantDirectMessage {
        source_actor: SourceActor::new(actor_ref("owner")),
        destination_actor: DestinationActor::new(actor_ref("initiator")),
    });

    let text = operation.to_nota();
    assert_eq!(text, "(GrantDirectMessage (owner initiator))");
    assert_eq!(
        RouterBootstrapOperation::from_nota(&text).expect("decode bootstrap operation"),
        operation
    );
}

#[cfg(feature = "nota-text")]
#[test]
fn bootstrap_register_remote_router_operation_round_trips_through_nota_line() {
    let operation = RouterBootstrapOperation::RegisterRemoteRouter(RegisterRemoteRouter {
        identity: CriomeHostId::new("prometheus-router").into(),
        address: TailnetAddress::new("[201:abcd::2]:9930").into(),
    });

    let text = operation.to_nota();
    assert_eq!(
        text,
        "(RegisterRemoteRouter (prometheus-router [|[201:abcd::2]:9930|]))"
    );
    assert_eq!(
        RouterBootstrapOperation::from_nota(&text).expect("decode bootstrap operation"),
        operation
    );
}

#[cfg(feature = "nota-text")]
#[test]
fn bootstrap_document_owns_line_vocabulary_for_manager_and_router() {
    let document = RouterBootstrapDocument::from_operations(vec![
        RouterBootstrapOperation::RegisterActor(RegisterActor::new(
            Actor::new(
                actor("initiator").into(),
                0,
                Some(endpoint_transport(
                    "/run/persona/engine/harness/initiator.sock",
                )),
            ),
            None,
        )),
        RouterBootstrapOperation::GrantDirectMessage(GrantDirectMessage {
            source_actor: SourceActor::new(actor_ref("initiator")),
            destination_actor: DestinationActor::new(actor_ref("responder")),
        }),
    ]);

    let text = document.to_nota_lines();
    let recovered =
        RouterBootstrapDocument::from_nota_lines(&text).expect("decode bootstrap document");

    assert_eq!(recovered, document);
    assert_eq!(recovered.operations().len(), 2);
}

#[cfg(feature = "nota-text")]
#[test]
fn router_observation_operation_kind_round_trips_through_nota_text() {
    round_trip_nota(RouterObservationScope::MessageTrace, "MessageTrace");
}

// ─── Encrypted authenticated peer session (primary-nbmq.6) ──────────────

fn identity_proof(signer: &str, challenge: &str) -> RouterIdentityProof {
    RouterIdentityProof::new(
        CriomeHostId::new(signer),
        SignatureScheme::Bls12_381MinPk,
        format!("public-key-{signer}"),
        format!("signature-{signer}"),
        format!("digest-{signer}-{challenge}"),
        ReplayNonce::new(challenge),
        TimestampNanos::new(4242),
    )
}

#[test]
fn session_client_hello_round_trips_through_length_prefixed_frame() {
    let hello = RouterSessionClientHello::new(
        SessionChallenge::new("challenge-initiator"),
        EphemeralPublicKey::new("ephemeral-initiator"),
    );
    round_trip_request(Input::session_client_hello(hello));
}

#[test]
fn session_server_hello_round_trips_carrying_the_responder_proof() {
    let hello = RouterSessionServerHello::new(
        SessionChallenge::new("challenge-responder"),
        EphemeralPublicKey::new("ephemeral-responder"),
        identity_proof("router-b", "challenge-initiator"),
    );
    let recovered = round_trip_reply(Output::session_server_hello(hello.clone()));
    assert_eq!(recovered, Output::SessionServerHello(hello));
}

#[test]
fn session_client_proof_round_trips_carrying_the_initiator_proof() {
    let proof = identity_proof("router-a", "challenge-responder");
    let payload = RouterSessionClientProof::new(proof.clone().into());
    round_trip_request(Input::SessionClientProof(payload.clone()));
    assert_eq!(payload.identity_proof(), &proof);
}

#[test]
fn session_data_carries_opaque_sealed_octets_without_decoding_them() {
    let sealed = vec![9_u64, 8, 7, 6, 5, 4, 3, 2, 1, 0];
    let data = RouterSessionData::from_octets(sealed.clone());
    round_trip_request(Input::SessionData(RouterSessionData::from_octets(
        sealed.clone(),
    )));
    assert_eq!(data.sealed_octets(), sealed.as_slice());
}

#[test]
fn session_accepted_carries_the_key_confirmation_octets() {
    let confirmation = vec![1_u64, 2, 3, 4];
    let accepted = RouterSessionAccepted::from_confirmation(confirmation.clone());
    let recovered = round_trip_reply(Output::SessionAccepted(accepted.clone()));
    assert_eq!(recovered, Output::SessionAccepted(accepted.clone()));
    assert_eq!(accepted.key_confirmation(), confirmation.as_slice());
}

#[test]
fn session_refused_round_trips_for_every_reason() {
    for reason in [
        SessionRefusalReason::IdentityProofInvalid,
        SessionRefusalReason::ChallengeMismatch,
        SessionRefusalReason::HandshakeMalformed,
        SessionRefusalReason::SessionCipherFailure,
    ] {
        let refused = RouterSessionRefused::new(reason.into());
        let recovered = round_trip_reply(Output::SessionRefused(refused.clone()));
        assert_eq!(recovered, Output::SessionRefused(refused.clone()));
        assert_eq!(refused.reason(), reason);
    }
}

#[test]
fn session_refusal_reason_is_closed_and_exhaustive() {
    for reason in [
        SessionRefusalReason::IdentityProofInvalid,
        SessionRefusalReason::ChallengeMismatch,
        SessionRefusalReason::HandshakeMalformed,
        SessionRefusalReason::SessionCipherFailure,
    ] {
        let observed = match reason {
            SessionRefusalReason::IdentityProofInvalid => "identity-proof-invalid",
            SessionRefusalReason::ChallengeMismatch => "challenge-mismatch",
            SessionRefusalReason::HandshakeMalformed => "handshake-malformed",
            SessionRefusalReason::SessionCipherFailure => "session-cipher-failure",
        };
        assert!(!observed.is_empty());
    }
}

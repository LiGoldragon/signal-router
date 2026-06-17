#[cfg(feature = "nota-text")]
use nota_next::{NotaDecode, NotaEncode, NotaSource};
use signal_frame::{
    ExchangeIdentifier, ExchangeLane, LaneSequence, NonEmpty, Reply, RequestPayload, SessionEpoch,
    SignalOperationHeads, SubReply,
};
#[cfg(feature = "nota-text")]
use signal_router::{
    Actor, EndpointKind, EndpointTransport, GrantDirectMessage, RegisterActor,
    RegisterRemoteRouter, RouterBootstrapDocument, RouterBootstrapOperation,
    RouterObservationScope,
};
use signal_router::{
    ForwardMarker, ForwardedMessagePayload, Frame, FrameBody, Input, Output, OwnerIdentity,
    RoutedContractObject, RouterChannelState, RouterChannelStateQuery, RouterChannelStatus,
    RouterDaemonConfiguration, RouterDeliveryStatus, RouterForwardRefusalReason,
    RouterForwardRequest, RouterMessageTrace, RouterMessageTraceMissing, RouterMessageTraceQuery,
    RouterPeerAttestation, RouterSummary, RouterSummaryQuery, SignatureScheme,
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

fn channel() -> String {
    String::from("internal-message-router")
}

#[cfg(feature = "nota-text")]
fn actor(name: &str) -> String {
    String::from(name)
}

fn forward_request() -> RouterForwardRequest {
    RouterForwardRequest {
        submission: ForwardedMessagePayload {
            from: String::from("ouranos-mind").into(),
            to: String::from("prometheus-responder").into(),
            body: String::from("hello over the tailnet"),
            attachments: vec![String::from("digest-001")],
            routed_objects: Vec::new(),
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

fn mirror_forward_request() -> RouterForwardRequest {
    let mut request = forward_request();
    request.submission.routed_objects.push(mirror_object());
    request
}

fn mirror_object() -> RoutedContractObject {
    let payload = vec![
        0x91, 0x26, 0xec, 0xcb, 0xb5, 0x00, 0x00, 0x00, b's', b'p', b'i', b'r', b'i', b't', 0x00,
        0x00, 0x07, 0x42,
    ];
    RoutedContractObject {
        contract: String::from("signal-mirror").into(),
        operation: String::from("NotifyObject").into(),
        payload_size: u64::try_from(payload.len())
            .expect("payload size fits")
            .into(),
        payload_octets: payload.into_iter().map(u64::from).collect(),
    }
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
    round_trip_request(Input::Summary(RouterSummaryQuery::new(engine().into())));
}

#[test]
fn router_message_trace_query_round_trips_through_length_prefixed_frame() {
    round_trip_request(Input::MessageTrace(RouterMessageTraceQuery {
        engine: engine().into(),
        message_slot: 7.into(),
    }));
}

#[test]
fn router_channel_state_query_round_trips_through_length_prefixed_frame() {
    round_trip_request(Input::ChannelState(RouterChannelStateQuery {
        engine: engine().into(),
        channel: channel().into(),
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
        .routed_objects
        .first()
        .expect("forward carries routed object");

    assert_eq!(object.contract.payload().as_str(), "signal-mirror");
    assert_eq!(object.operation.payload().as_str(), "NotifyObject");
    assert_eq!(
        usize::try_from(*object.payload_size.payload()).expect("payload size fits"),
        object.payload_octets.len()
    );
    assert_eq!(
        object.payload_octets,
        vec![
            0x91, 0x26, 0xec, 0xcb, 0xb5, 0, 0, 0, b's', b'p', b'i', b'r', b'i', b't', 0, 0, 7,
            0x42,
        ]
        .into_iter()
        .map(u64::from)
        .collect::<Vec<_>>()
    );
}

#[cfg(feature = "nota-text")]
#[test]
fn router_forward_request_round_trips_through_nota_text() {
    round_trip_nota(
        Input::ForwardMessage(forward_request()),
        "(ForwardMessage ((ouranos-mind prometheus-responder [hello over the tailnet] [digest-001] []) (prometheus-router Bls12_381MinPk bls-pk-abc bls-sig-def blake3-0011 1726000000000000000 nonce-7f3a) Origin nonce-7f3a 1726000000000000000))",
    );
}

#[test]
fn router_request_heads_are_contract_local_operations() {
    assert_eq!(
        <Input as SignalOperationHeads>::HEADS,
        &["Summary", "MessageTrace", "ChannelState", "ForwardMessage"]
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

#[test]
fn router_summary_reply_round_trips_through_length_prefixed_frame() {
    let reply = Output::Summary(RouterSummary {
        engine: engine().into(),
        accepted_messages: 1,
        routed_messages: 1,
        deferred_messages: 0,
        failed_messages: 0,
    });
    assert_eq!(round_trip_reply(reply.clone()), reply);
}

#[test]
fn router_message_trace_reply_round_trips_through_length_prefixed_frame() {
    let reply = Output::MessageTrace(RouterMessageTrace {
        engine: engine().into(),
        message_slot: 7.into(),
        status: RouterDeliveryStatus::Routed,
    });
    assert_eq!(round_trip_reply(reply.clone()), reply);
}

#[test]
fn router_channel_state_reply_round_trips_through_length_prefixed_frame() {
    let reply = Output::ChannelState(RouterChannelState {
        engine: engine().into(),
        channel: channel().into(),
        status: RouterChannelStatus::Installed,
    });
    assert_eq!(round_trip_reply(reply.clone()), reply);
}

#[test]
fn router_message_trace_missing_reply_round_trips_through_length_prefixed_frame() {
    let reply = Output::MessageTraceMissing(RouterMessageTraceMissing {
        engine: engine().into(),
        message_slot: 99.into(),
    });
    assert_eq!(round_trip_reply(reply.clone()), reply);
}

#[test]
fn router_forward_accepted_reply_round_trips_through_length_prefixed_frame() {
    let reply = Output::forward_accepted(7.into());
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
    ] {
        let reply = Output::forward_refused(reason);
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
    ] {
        let observed = match reason {
            RouterForwardRefusalReason::UnknownPeer => "unknown-peer",
            RouterForwardRefusalReason::AttestationInvalid => "attestation-invalid",
            RouterForwardRefusalReason::ReplayDetected => "replay-detected",
            RouterForwardRefusalReason::ClockSkew => "clock-skew",
            RouterForwardRefusalReason::RecipientUnknown => "recipient-unknown",
            RouterForwardRefusalReason::ChannelUnauthorized => "channel-unauthorized",
            RouterForwardRefusalReason::AlreadyForwarded => "already-forwarded",
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
    let configuration = RouterDaemonConfiguration {
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
    };

    let bytes = configuration.to_rkyv_bytes().expect("archive");
    let recovered = RouterDaemonConfiguration::from_rkyv_bytes(&bytes).expect("decode rkyv");
    assert_eq!(recovered, configuration);
}

#[test]
fn single_host_router_configuration_has_no_tailnet_listen_address() {
    let configuration = RouterDaemonConfiguration {
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
    };

    assert!(
        configuration.tailnet_listen_address.is_none(),
        "an absent tailnet listen address keeps the router local-only with no TCP forwarding tier"
    );

    let bytes = configuration.to_rkyv_bytes().expect("archive");
    let recovered = RouterDaemonConfiguration::from_rkyv_bytes(&bytes).expect("decode rkyv");
    assert_eq!(recovered, configuration);
}

#[cfg(feature = "nota-text")]
fn endpoint_transport(path: &str) -> EndpointTransport {
    EndpointTransport {
        kind: EndpointKind::HarnessSocket,
        target: String::from(path),
        auxiliary: None,
    }
}

#[cfg(feature = "nota-text")]
#[test]
fn bootstrap_register_actor_operation_round_trips_through_nota_line() {
    let operation = RouterBootstrapOperation::RegisterActor(RegisterActor {
        actor: Actor {
            name: actor("responder").into(),
            process: 42,
            endpoint: Some(endpoint_transport("/tmp/responder.harness.sock")),
        },
        home: None,
    });

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
    let operation = RouterBootstrapOperation::RegisterActor(RegisterActor {
        actor: Actor {
            name: actor("responder").into(),
            process: 42,
            endpoint: Some(endpoint_transport("/tmp/responder.harness.sock")),
        },
        home: Some(String::from("prometheus-router").into()),
    });

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
        from: actor("owner").into(),
        to: actor("initiator").into(),
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
        identity: String::from("prometheus-router").into(),
        address: String::from("[201:abcd::2]:9930").into(),
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
    let document = RouterBootstrapDocument::new(vec![
        RouterBootstrapOperation::RegisterActor(RegisterActor {
            actor: Actor {
                name: actor("initiator").into(),
                process: 0,
                endpoint: Some(endpoint_transport(
                    "/run/persona/engine/harness/initiator.sock",
                )),
            },
            home: None,
        }),
        RouterBootstrapOperation::GrantDirectMessage(GrantDirectMessage {
            from: actor("initiator").into(),
            to: actor("responder").into(),
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

use signal_core::{
    ExchangeIdentifier, ExchangeLane, LaneSequence, NonEmpty, Reply, RequestPayload, SessionEpoch,
    SignalVerb, SubReply,
};
use signal_message::MessageSlot;
use signal_persona_origin::{ChannelIdentifier, EngineIdentifier};
use signal_router::{
    Actor, ActorIdentifier, EndpointKind, EndpointTransport, GrantDirectMessage, RegisterActor,
    RouterBootstrapDocument, RouterBootstrapOperation, RouterChannelState, RouterChannelStateQuery,
    RouterChannelStatus, RouterDeliveryStatus, RouterFrame as Frame, RouterFrameBody as FrameBody,
    RouterMessageTrace, RouterMessageTraceMissing, RouterMessageTraceQuery, RouterReply,
    RouterRequest, RouterSummary, RouterSummaryQuery,
};

fn exchange() -> ExchangeIdentifier {
    ExchangeIdentifier::new(
        SessionEpoch::new(1),
        ExchangeLane::Connector,
        LaneSequence::first(),
    )
}

fn round_trip_request(request: RouterRequest) {
    let expected_verb = request.signal_verb();
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
            let operation = decoded_request.operations().head();
            assert_eq!(operation.verb, expected_verb);
            assert_eq!(operation.verb, SignalVerb::Match);
            assert_eq!(operation.payload, request);
        }
        other => panic!("expected Match request, got {other:?}"),
    }
}

fn round_trip_reply(reply: RouterReply) -> RouterReply {
    let frame = Frame::new(FrameBody::Reply {
        exchange: exchange(),
        reply: Reply::completed(NonEmpty::single(SubReply::Ok {
            verb: SignalVerb::Match,
            payload: reply,
        })),
    });

    let bytes = frame.encode_length_prefixed().expect("encode");
    let decoded = Frame::decode_length_prefixed(&bytes).expect("decode");

    match decoded.into_body() {
        FrameBody::Reply { reply, .. } => match reply {
            Reply::Accepted { per_operation, .. } => match per_operation.into_head() {
                SubReply::Ok { payload, .. } => payload,
                other => panic!("expected accepted reply payload, got {other:?}"),
            },
            other => panic!("expected accepted reply, got {other:?}"),
        },
        other => panic!("expected reply, got {other:?}"),
    }
}

#[test]
fn router_summary_query_round_trips_through_length_prefixed_frame() {
    let request = RouterRequest::Summary(RouterSummaryQuery {
        engine: EngineIdentifier::new("prototype"),
    });
    round_trip_request(request);
}

#[test]
fn router_message_trace_query_round_trips_through_length_prefixed_frame() {
    let request = RouterRequest::MessageTrace(RouterMessageTraceQuery {
        engine: EngineIdentifier::new("prototype"),
        message_slot: MessageSlot::new(7),
    });
    round_trip_request(request);
}

#[test]
fn router_channel_state_query_round_trips_through_length_prefixed_frame() {
    let request = RouterRequest::ChannelState(RouterChannelStateQuery {
        engine: EngineIdentifier::new("prototype"),
        channel: ChannelIdentifier::new("internal-message-router"),
    });
    round_trip_request(request);
}

#[test]
fn router_request_variants_declare_match_as_signal_root_verb() {
    let requests = [
        RouterRequest::Summary(RouterSummaryQuery {
            engine: EngineIdentifier::new("prototype"),
        }),
        RouterRequest::MessageTrace(RouterMessageTraceQuery {
            engine: EngineIdentifier::new("prototype"),
            message_slot: MessageSlot::new(7),
        }),
        RouterRequest::ChannelState(RouterChannelStateQuery {
            engine: EngineIdentifier::new("prototype"),
            channel: ChannelIdentifier::new("internal-message-router"),
        }),
    ];

    for request in requests {
        assert_eq!(request.signal_verb(), SignalVerb::Match);
    }
}

#[test]
fn router_summary_reply_round_trips_through_length_prefixed_frame() {
    let reply = RouterReply::Summary(RouterSummary {
        engine: EngineIdentifier::new("prototype"),
        accepted_messages: 1,
        routed_messages: 1,
        deferred_messages: 0,
        failed_messages: 0,
    });
    assert_eq!(round_trip_reply(reply.clone()), reply);
}

#[test]
fn router_message_trace_reply_round_trips_through_length_prefixed_frame() {
    let reply = RouterReply::MessageTrace(RouterMessageTrace {
        engine: EngineIdentifier::new("prototype"),
        message_slot: MessageSlot::new(7),
        status: RouterDeliveryStatus::Routed,
    });
    assert_eq!(round_trip_reply(reply.clone()), reply);
}

#[test]
fn router_channel_state_reply_round_trips_through_length_prefixed_frame() {
    let reply = RouterReply::ChannelState(RouterChannelState {
        engine: EngineIdentifier::new("prototype"),
        channel: ChannelIdentifier::new("internal-message-router"),
        status: RouterChannelStatus::Installed,
    });
    assert_eq!(round_trip_reply(reply.clone()), reply);
}

#[test]
fn router_message_trace_missing_reply_round_trips_through_length_prefixed_frame() {
    let reply = RouterReply::MessageTraceMissing(RouterMessageTraceMissing {
        engine: EngineIdentifier::new("prototype"),
        message_slot: MessageSlot::new(99),
    });
    assert_eq!(round_trip_reply(reply.clone()), reply);
}

#[test]
fn router_status_enums_are_closed_no_unknown_variants() {
    // Witness for the closed-enum integrity rule: callers may exhaustively
    // match every `RouterDeliveryStatus` and `RouterChannelStatus` variant.
    // Adding an `Unknown` (or any forward-compat placeholder) would smuggle
    // a polling-shape escape hatch back into the wire enum; this match must
    // continue to enumerate only positively-named, store-derivable states.
    for status in [
        RouterDeliveryStatus::Accepted,
        RouterDeliveryStatus::Routed,
        RouterDeliveryStatus::Delivered,
        RouterDeliveryStatus::Deferred,
        RouterDeliveryStatus::Failed,
    ] {
        let observed = match status {
            RouterDeliveryStatus::Accepted => "accepted",
            RouterDeliveryStatus::Routed => "routed",
            RouterDeliveryStatus::Delivered => "delivered",
            RouterDeliveryStatus::Deferred => "deferred",
            RouterDeliveryStatus::Failed => "failed",
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

#[test]
fn router_daemon_configuration_round_trips_through_nota_text() {
    use nota_codec::{Decoder, Encoder, NotaDecode, NotaEncode};
    use signal_persona::{SocketMode, WirePath};
    use signal_persona_origin::{OwnerIdentity, UnixUserIdentifier};
    use signal_router::RouterDaemonConfiguration;

    let configuration = RouterDaemonConfiguration {
        router_socket_path: WirePath::new("/run/persona/X/router.sock"),
        router_socket_mode: SocketMode::new(0o600),
        supervision_socket_path: WirePath::new("/run/persona/X/router-supervision.sock"),
        supervision_socket_mode: SocketMode::new(0o600),
        store_path: WirePath::new("/var/lib/persona/X/router.redb"),
        bootstrap_path: Some(WirePath::new("/var/lib/persona/X/router-bootstrap.nota")),
        owner_identity: OwnerIdentity::UnixUser(UnixUserIdentifier::new(1000)),
    };

    let mut encoder = Encoder::new();
    configuration
        .encode(&mut encoder)
        .expect("encode configuration");
    let text = encoder.into_string();
    let mut decoder = Decoder::new(&text);
    let recovered = RouterDaemonConfiguration::decode(&mut decoder).expect("decode configuration");

    assert_eq!(recovered, configuration);
}

#[test]
fn router_daemon_configuration_round_trips_through_rkyv() {
    use nota_config::ConfigurationRecord;
    use signal_persona::{SocketMode, WirePath};
    use signal_persona_origin::{OwnerIdentity, UnixUserIdentifier};
    use signal_router::RouterDaemonConfiguration;

    let configuration = RouterDaemonConfiguration {
        router_socket_path: WirePath::new("/run/persona/X/router.sock"),
        router_socket_mode: SocketMode::new(0o600),
        supervision_socket_path: WirePath::new("/run/persona/X/router-supervision.sock"),
        supervision_socket_mode: SocketMode::new(0o600),
        store_path: WirePath::new("/var/lib/persona/X/router.redb"),
        bootstrap_path: None,
        owner_identity: OwnerIdentity::UnixUser(UnixUserIdentifier::new(1000)),
    };

    let bytes = rkyv::to_bytes::<rkyv::rancor::Error>(&configuration).expect("archive");
    let recovered = RouterDaemonConfiguration::from_rkyv_bytes(&bytes).expect("decode rkyv");
    assert_eq!(recovered, configuration);
}

#[test]
fn bootstrap_register_actor_operation_round_trips_through_nota_line() {
    let operation = RouterBootstrapOperation::RegisterActor(RegisterActor::new(Actor::new(
        ActorIdentifier::new("responder"),
        42,
        Some(EndpointTransport::new(
            EndpointKind::HarnessSocket,
            "/tmp/responder.harness.sock",
            None,
        )),
    )));

    let text = operation.to_nota().expect("encode bootstrap operation");
    assert_eq!(
        text,
        "(RegisterActor ((responder 42 (Some (HarnessSocket [/tmp/responder.harness.sock] None)))))"
    );
    assert_eq!(
        RouterBootstrapOperation::from_nota(&text).expect("decode bootstrap operation"),
        operation
    );
}

#[test]
fn bootstrap_direct_message_grant_operation_round_trips_through_nota_line() {
    let operation = RouterBootstrapOperation::GrantDirectMessage(GrantDirectMessage::new(
        ActorIdentifier::new("owner"),
        ActorIdentifier::new("initiator"),
    ));

    let text = operation.to_nota().expect("encode bootstrap operation");
    assert_eq!(text, "(GrantDirectMessage (owner initiator))");
    assert_eq!(
        RouterBootstrapOperation::from_nota(&text).expect("decode bootstrap operation"),
        operation
    );
}

#[test]
fn bootstrap_document_owns_line_vocabulary_for_manager_and_router() {
    let document = RouterBootstrapDocument::new(vec![
        RouterBootstrapOperation::RegisterActor(RegisterActor::new(Actor::new(
            ActorIdentifier::new("initiator"),
            0,
            Some(EndpointTransport::new(
                EndpointKind::HarnessSocket,
                "/run/persona/engine/harness/initiator.sock",
                None,
            )),
        ))),
        RouterBootstrapOperation::GrantDirectMessage(GrantDirectMessage::new(
            ActorIdentifier::new("initiator"),
            ActorIdentifier::new("responder"),
        )),
    ]);

    let text = document.to_nota_lines().expect("encode bootstrap document");
    let recovered =
        RouterBootstrapDocument::from_nota_lines(&text).expect("decode bootstrap document");

    assert_eq!(recovered, document);
    assert_eq!(recovered.operations().len(), 2);
}

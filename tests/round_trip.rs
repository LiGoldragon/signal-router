use signal_core::{
    ExchangeIdentifier, ExchangeLane, LaneSequence, NonEmpty, Reply, RequestPayload, SessionEpoch,
    SignalVerb, SubReply,
};
use signal_persona_auth::{ChannelId, EngineId};
use signal_persona_message::MessageSlot;
use signal_persona_router::{
    RouterChannelState, RouterChannelStateQuery, RouterChannelStatus, RouterDeliveryStatus,
    RouterFrame as Frame, RouterFrameBody as FrameBody, RouterMessageTrace,
    RouterMessageTraceMissing, RouterMessageTraceQuery, RouterReply, RouterRequest, RouterSummary,
    RouterSummaryQuery,
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
        engine: EngineId::new("prototype"),
    });
    round_trip_request(request);
}

#[test]
fn router_message_trace_query_round_trips_through_length_prefixed_frame() {
    let request = RouterRequest::MessageTrace(RouterMessageTraceQuery {
        engine: EngineId::new("prototype"),
        message_slot: MessageSlot::new(7),
    });
    round_trip_request(request);
}

#[test]
fn router_channel_state_query_round_trips_through_length_prefixed_frame() {
    let request = RouterRequest::ChannelState(RouterChannelStateQuery {
        engine: EngineId::new("prototype"),
        channel: ChannelId::new("internal-message-router"),
    });
    round_trip_request(request);
}

#[test]
fn router_request_variants_declare_match_as_signal_root_verb() {
    let requests = [
        RouterRequest::Summary(RouterSummaryQuery {
            engine: EngineId::new("prototype"),
        }),
        RouterRequest::MessageTrace(RouterMessageTraceQuery {
            engine: EngineId::new("prototype"),
            message_slot: MessageSlot::new(7),
        }),
        RouterRequest::ChannelState(RouterChannelStateQuery {
            engine: EngineId::new("prototype"),
            channel: ChannelId::new("internal-message-router"),
        }),
    ];

    for request in requests {
        assert_eq!(request.signal_verb(), SignalVerb::Match);
    }
}

#[test]
fn router_summary_reply_round_trips_through_length_prefixed_frame() {
    let reply = RouterReply::Summary(RouterSummary {
        engine: EngineId::new("prototype"),
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
        engine: EngineId::new("prototype"),
        message_slot: MessageSlot::new(7),
        status: RouterDeliveryStatus::Routed,
    });
    assert_eq!(round_trip_reply(reply.clone()), reply);
}

#[test]
fn router_channel_state_reply_round_trips_through_length_prefixed_frame() {
    let reply = RouterReply::ChannelState(RouterChannelState {
        engine: EngineId::new("prototype"),
        channel: ChannelId::new("internal-message-router"),
        status: RouterChannelStatus::Installed,
    });
    assert_eq!(round_trip_reply(reply.clone()), reply);
}

#[test]
fn router_message_trace_missing_reply_round_trips_through_length_prefixed_frame() {
    let reply = RouterReply::MessageTraceMissing(RouterMessageTraceMissing {
        engine: EngineId::new("prototype"),
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
    use signal_persona_auth::{OwnerIdentity, UnixUserId};
    use signal_persona_router::RouterDaemonConfiguration;

    let configuration = RouterDaemonConfiguration {
        router_socket_path: WirePath::new("/run/persona/X/router.sock"),
        router_socket_mode: SocketMode::new(0o600),
        supervision_socket_path: WirePath::new("/run/persona/X/router-supervision.sock"),
        supervision_socket_mode: SocketMode::new(0o600),
        store_path: WirePath::new("/var/lib/persona/X/router.redb"),
        bootstrap_path: Some(WirePath::new("/var/lib/persona/X/router-bootstrap.nota")),
        owner_identity: OwnerIdentity::UnixUser(UnixUserId::new(1000)),
    };

    let mut encoder = Encoder::new();
    configuration.encode(&mut encoder).expect("encode configuration");
    let text = encoder.into_string();
    let mut decoder = Decoder::new(&text);
    let recovered = RouterDaemonConfiguration::decode(&mut decoder).expect("decode configuration");

    assert_eq!(recovered, configuration);
}

#[test]
fn router_daemon_configuration_round_trips_through_rkyv() {
    use nota_config::ConfigurationRecord;
    use signal_persona::{SocketMode, WirePath};
    use signal_persona_auth::{OwnerIdentity, UnixUserId};
    use signal_persona_router::RouterDaemonConfiguration;

    let configuration = RouterDaemonConfiguration {
        router_socket_path: WirePath::new("/run/persona/X/router.sock"),
        router_socket_mode: SocketMode::new(0o600),
        supervision_socket_path: WirePath::new("/run/persona/X/router-supervision.sock"),
        supervision_socket_mode: SocketMode::new(0o600),
        store_path: WirePath::new("/var/lib/persona/X/router.redb"),
        bootstrap_path: None,
        owner_identity: OwnerIdentity::UnixUser(UnixUserId::new(1000)),
    };

    let bytes = rkyv::to_bytes::<rkyv::rancor::Error>(&configuration).expect("archive");
    let recovered = RouterDaemonConfiguration::from_rkyv_bytes(&bytes).expect("decode rkyv");
    assert_eq!(recovered, configuration);
}

#[cfg(feature = "nota-text")]
use nota_next::{NotaDecode, NotaEncode, NotaSource};
use signal_frame::{
    ExchangeIdentifier, ExchangeLane, LaneSequence, NonEmpty, Reply, RequestPayload, SessionEpoch,
    SignalOperationHeads, SubReply,
};
#[cfg(feature = "nota-text")]
use signal_router::{
    Actor, EndpointKind, EndpointTransport, GrantDirectMessage, RegisterActor,
    RouterBootstrapDocument, RouterBootstrapOperation, RouterObservationScope,
};
use signal_router::{
    Frame, FrameBody, Input, Output, OwnerIdentity, RouterChannelState, RouterChannelStateQuery,
    RouterChannelStatus, RouterDaemonConfiguration, RouterDeliveryStatus, RouterMessageTrace,
    RouterMessageTraceMissing, RouterMessageTraceQuery, RouterSummary, RouterSummaryQuery,
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
    round_trip_request(Input::Summary(RouterSummaryQuery::new(engine())));
}

#[test]
fn router_message_trace_query_round_trips_through_length_prefixed_frame() {
    round_trip_request(Input::MessageTrace(RouterMessageTraceQuery {
        engine: engine(),
        message_slot: 7,
    }));
}

#[test]
fn router_channel_state_query_round_trips_through_length_prefixed_frame() {
    round_trip_request(Input::ChannelState(RouterChannelStateQuery {
        engine: engine(),
        channel: channel(),
    }));
}

#[test]
fn router_request_heads_are_contract_local_operations() {
    assert_eq!(
        <Input as SignalOperationHeads>::HEADS,
        &["Summary", "MessageTrace", "ChannelState"]
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
        engine: engine(),
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
        engine: engine(),
        message_slot: 7,
        status: RouterDeliveryStatus::Routed,
    });
    assert_eq!(round_trip_reply(reply.clone()), reply);
}

#[test]
fn router_channel_state_reply_round_trips_through_length_prefixed_frame() {
    let reply = Output::ChannelState(RouterChannelState {
        engine: engine(),
        channel: channel(),
        status: RouterChannelStatus::Installed,
    });
    assert_eq!(round_trip_reply(reply.clone()), reply);
}

#[test]
fn router_message_trace_missing_reply_round_trips_through_length_prefixed_frame() {
    let reply = Output::MessageTraceMissing(RouterMessageTraceMissing {
        engine: engine(),
        message_slot: 99,
    });
    assert_eq!(round_trip_reply(reply.clone()), reply);
}

#[test]
fn router_status_enums_are_closed_no_unknown_variants() {
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

#[cfg(feature = "nota-text")]
#[test]
fn router_daemon_configuration_round_trips_through_nota_text() {
    let configuration = RouterDaemonConfiguration {
        router_socket_path: String::from("/run/persona/X/router.sock"),
        router_socket_mode: 0o600,
        meta_router_socket_path: String::from("/run/persona/X/router-meta.sock"),
        meta_router_socket_mode: 0o600,
        supervision_socket_path: String::from("/run/persona/X/router-supervision.sock"),
        supervision_socket_mode: 0o600,
        store_path: String::from("/var/lib/persona/X/router.sema"),
        bootstrap_path: Some(String::from("/var/lib/persona/X/router-bootstrap.nota")),
        owner_identity: OwnerIdentity::UnixUser(1000),
    };

    let text = configuration.to_nota();
    let recovered = NotaSource::new(&text)
        .parse::<RouterDaemonConfiguration>()
        .expect("decode configuration");

    assert_eq!(recovered, configuration);
    assert!(text.contains("[/run/persona/X/router.sock]"));
}

#[test]
fn router_daemon_configuration_round_trips_through_rkyv() {
    let configuration = RouterDaemonConfiguration {
        router_socket_path: String::from("/run/persona/X/router.sock"),
        router_socket_mode: 0o600,
        meta_router_socket_path: String::from("/run/persona/X/router-meta.sock"),
        meta_router_socket_mode: 0o600,
        supervision_socket_path: String::from("/run/persona/X/router-supervision.sock"),
        supervision_socket_mode: 0o600,
        store_path: String::from("/var/lib/persona/X/router.sema"),
        bootstrap_path: None,
        owner_identity: OwnerIdentity::UnixUser(1000),
    };

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
    let operation = RouterBootstrapOperation::RegisterActor(RegisterActor::new(Actor {
        name: actor("responder"),
        process: 42,
        endpoint: Some(endpoint_transport("/tmp/responder.harness.sock")),
    }));

    let text = operation.to_nota();
    assert_eq!(
        text,
        "(RegisterActor ([responder] 42 (Some (HarnessSocket [/tmp/responder.harness.sock] None))))"
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
        from: actor("owner"),
        to: actor("initiator"),
    });

    let text = operation.to_nota();
    assert_eq!(text, "(GrantDirectMessage ([owner] [initiator]))");
    assert_eq!(
        RouterBootstrapOperation::from_nota(&text).expect("decode bootstrap operation"),
        operation
    );
}

#[cfg(feature = "nota-text")]
#[test]
fn bootstrap_document_owns_line_vocabulary_for_manager_and_router() {
    let document = RouterBootstrapDocument::new(vec![
        RouterBootstrapOperation::RegisterActor(RegisterActor::new(Actor {
            name: actor("initiator"),
            process: 0,
            endpoint: Some(endpoint_transport(
                "/run/persona/engine/harness/initiator.sock",
            )),
        })),
        RouterBootstrapOperation::GrantDirectMessage(GrantDirectMessage {
            from: actor("initiator"),
            to: actor("responder"),
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

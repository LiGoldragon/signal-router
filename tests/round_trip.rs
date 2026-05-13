use signal_core::{FrameBody, Request, SemaVerb};
use signal_persona_auth::{ChannelId, EngineId};
use signal_persona_message::MessageSlot;
use signal_persona_router::{
    Frame, RouterChannelState, RouterChannelStateQuery, RouterChannelStatus, RouterDeliveryStatus,
    RouterMessageTrace, RouterMessageTraceQuery, RouterReply, RouterRequest, RouterSummary,
    RouterSummaryQuery,
};

#[test]
fn router_summary_query_round_trips_through_length_prefixed_frame() {
    let request = RouterRequest::Summary(RouterSummaryQuery {
        engine: EngineId::new("prototype"),
    });
    let frame = Frame::new(FrameBody::Request(Request::assert(request.clone())));

    let bytes = frame.encode_length_prefixed().expect("encode");
    let decoded = Frame::decode_length_prefixed(&bytes).expect("decode");

    match decoded.into_body() {
        FrameBody::Request(Request::Operation { verb, payload }) => {
            assert_eq!(verb, SemaVerb::Assert);
            assert_eq!(payload, request);
        }
        other => panic!("expected Assert request, got {other:?}"),
    }
}
#[test]
fn router_message_trace_query_round_trips_through_length_prefixed_frame() {
    let request = RouterRequest::MessageTrace(RouterMessageTraceQuery {
        engine: EngineId::new("prototype"),
        message_slot: MessageSlot::new(7),
    });
    let frame = Frame::new(FrameBody::Request(Request::assert(request.clone())));

    let bytes = frame.encode_length_prefixed().expect("encode");
    let decoded = Frame::decode_length_prefixed(&bytes).expect("decode");

    match decoded.into_body() {
        FrameBody::Request(Request::Operation { payload, .. }) => {
            assert_eq!(payload, request);
        }
        other => panic!("expected request, got {other:?}"),
    }
}

#[test]
fn router_channel_state_query_round_trips_through_length_prefixed_frame() {
    let request = RouterRequest::ChannelState(RouterChannelStateQuery {
        engine: EngineId::new("prototype"),
        channel: ChannelId::new("internal-message-router"),
    });
    let frame = Frame::new(FrameBody::Request(Request::assert(request.clone())));

    let bytes = frame.encode_length_prefixed().expect("encode");
    let decoded = Frame::decode_length_prefixed(&bytes).expect("decode");

    match decoded.into_body() {
        FrameBody::Request(Request::Operation { payload, .. }) => {
            assert_eq!(payload, request);
        }
        other => panic!("expected request, got {other:?}"),
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
    let frame = Frame::new(FrameBody::Reply(signal_core::Reply::operation(
        reply.clone(),
    )));

    let bytes = frame.encode_length_prefixed().expect("encode");
    let decoded = Frame::decode_length_prefixed(&bytes).expect("decode");

    match decoded.into_body() {
        FrameBody::Reply(signal_core::Reply::Operation(decoded_reply)) => {
            assert_eq!(decoded_reply, reply);
        }
        other => panic!("expected reply, got {other:?}"),
    }
}

#[test]
fn router_message_trace_reply_round_trips_through_length_prefixed_frame() {
    let reply = RouterReply::MessageTrace(RouterMessageTrace {
        engine: EngineId::new("prototype"),
        message_slot: MessageSlot::new(7),
        status: RouterDeliveryStatus::Routed,
    });
    let frame = Frame::new(FrameBody::Reply(signal_core::Reply::operation(
        reply.clone(),
    )));

    let bytes = frame.encode_length_prefixed().expect("encode");
    let decoded = Frame::decode_length_prefixed(&bytes).expect("decode");

    match decoded.into_body() {
        FrameBody::Reply(signal_core::Reply::Operation(decoded_reply)) => {
            assert_eq!(decoded_reply, reply);
        }
        other => panic!("expected reply, got {other:?}"),
    }
}

#[test]
fn router_channel_state_reply_round_trips_through_length_prefixed_frame() {
    let reply = RouterReply::ChannelState(RouterChannelState {
        engine: EngineId::new("prototype"),
        channel: ChannelId::new("internal-message-router"),
        status: RouterChannelStatus::Installed,
    });
    let frame = Frame::new(FrameBody::Reply(signal_core::Reply::operation(
        reply.clone(),
    )));

    let bytes = frame.encode_length_prefixed().expect("encode");
    let decoded = Frame::decode_length_prefixed(&bytes).expect("decode");

    match decoded.into_body() {
        FrameBody::Reply(signal_core::Reply::Operation(decoded_reply)) => {
            assert_eq!(decoded_reply, reply);
        }
        other => panic!("expected reply, got {other:?}"),
    }
}

//! Canonical examples round-trip witness.
//!
//! Parses `examples/canonical.nota` end-to-end, decoding each record
//! as a `RouterRequest` or `RouterReply` and asserting the re-encoded
//! text equals the canonical form. Adding a new variant requires
//! adding both a canonical-text example and the matching expected
//! value here; the witness is what keeps the examples file aligned
//! with the typed surface.

use nota_codec::{Decoder, Encoder, NotaDecode, NotaEncode};
use signal_message::MessageSlot;
use signal_persona_origin::{ChannelIdentifier, EngineIdentifier};
use signal_persona_router::{
    RouterChannelState, RouterChannelStateQuery, RouterChannelStatus, RouterDeliveryStatus,
    RouterMessageTrace, RouterMessageTraceMissing, RouterMessageTraceQuery, RouterObservationScope,
    RouterObservationUnimplemented, RouterObservationUnimplementedReason, RouterReply,
    RouterRequest, RouterSummary, RouterSummaryQuery,
};

const CANONICAL: &str = include_str!("../examples/canonical.nota");

fn engine() -> EngineIdentifier {
    EngineIdentifier::new("prototype")
}

fn channel() -> ChannelIdentifier {
    ChannelIdentifier::new("internal-message-router")
}

#[test]
fn canonical_request_examples_round_trip() {
    let expected: Vec<(RouterRequest, &str)> = vec![
        (
            RouterRequest::Summary(RouterSummaryQuery { engine: engine() }),
            "(Summary (prototype))",
        ),
        (
            RouterRequest::MessageTrace(RouterMessageTraceQuery {
                engine: engine(),
                message_slot: MessageSlot::new(7),
            }),
            "(MessageTrace (prototype 7))",
        ),
        (
            RouterRequest::ChannelState(RouterChannelStateQuery {
                engine: engine(),
                channel: channel(),
            }),
            "(ChannelState (prototype internal-message-router))",
        ),
    ];

    for (value, canonical_text) in expected {
        let mut encoder = Encoder::new();
        value.encode(&mut encoder).expect("encode");
        let text = encoder.into_string();
        assert_eq!(text, canonical_text, "encode for {value:?}");

        let mut decoder = Decoder::new(canonical_text);
        let decoded = RouterRequest::decode(&mut decoder).expect("decode");
        assert_eq!(decoded, value, "decode for {canonical_text}");

        assert!(
            CANONICAL.contains(canonical_text),
            "examples/canonical.nota missing line: {canonical_text}",
        );
    }
}

#[test]
fn canonical_reply_examples_round_trip() {
    let expected: Vec<(RouterReply, &str)> = vec![
        (
            RouterReply::Summary(RouterSummary {
                engine: engine(),
                accepted_messages: 1,
                routed_messages: 1,
                deferred_messages: 0,
                failed_messages: 0,
            }),
            "(Summary (prototype 1 1 0 0))",
        ),
        (
            RouterReply::MessageTrace(RouterMessageTrace {
                engine: engine(),
                message_slot: MessageSlot::new(7),
                status: RouterDeliveryStatus::Routed,
            }),
            "(MessageTrace (prototype 7 Routed))",
        ),
        (
            RouterReply::MessageTraceMissing(RouterMessageTraceMissing {
                engine: engine(),
                message_slot: MessageSlot::new(99),
            }),
            "(MessageTraceMissing (prototype 99))",
        ),
        (
            RouterReply::ChannelState(RouterChannelState {
                engine: engine(),
                channel: channel(),
                status: RouterChannelStatus::Installed,
            }),
            "(ChannelState (prototype internal-message-router Installed))",
        ),
        (
            RouterReply::Unimplemented(RouterObservationUnimplemented {
                scope: RouterObservationScope::Summary,
                reason: RouterObservationUnimplementedReason::NotInPrototypeScope,
            }),
            "(Unimplemented (Summary NotInPrototypeScope))",
        ),
    ];

    for (value, canonical_text) in expected {
        let mut encoder = Encoder::new();
        value.encode(&mut encoder).expect("encode");
        let text = encoder.into_string();
        assert_eq!(text, canonical_text, "encode for {value:?}");

        let mut decoder = Decoder::new(canonical_text);
        let decoded = RouterReply::decode(&mut decoder).expect("decode");
        assert_eq!(decoded, value, "decode for {canonical_text}");

        assert!(
            CANONICAL.contains(canonical_text),
            "examples/canonical.nota missing line: {canonical_text}",
        );
    }
}

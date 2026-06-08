#![cfg(feature = "nota-text")]

//! Canonical examples round-trip witness.
//!
//! Parses `examples/canonical.nota` end-to-end, decoding each record
//! as an `Input` or `Output` and asserting the re-encoded
//! text equals the canonical form. Adding a new variant requires
//! adding both a canonical-text example and the matching expected
//! value here; the witness is what keeps the examples file aligned
//! with the typed surface.

use nota_next::NotaSource;
use signal_router::{
    Input, Output, RouterChannelState, RouterChannelStateQuery, RouterChannelStatus,
    RouterDeliveryStatus, RouterMessageTrace, RouterMessageTraceMissing, RouterMessageTraceQuery,
    RouterObservationScope, RouterObservationUnimplemented, RouterObservationUnimplementedReason,
    RouterSummary, RouterSummaryQuery,
};

const CANONICAL: &str = include_str!("../examples/canonical.nota");

fn engine() -> String {
    String::from("prototype")
}

fn channel() -> String {
    String::from("internal-message-router")
}

#[test]
fn canonical_request_examples_round_trip() {
    let expected: Vec<(Input, &str)> = vec![
        (
            Input::Summary(RouterSummaryQuery::new(engine())),
            "(Summary [prototype])",
        ),
        (
            Input::MessageTrace(RouterMessageTraceQuery {
                engine: engine(),
                message_slot: 7,
            }),
            "(MessageTrace ([prototype] 7))",
        ),
        (
            Input::ChannelState(RouterChannelStateQuery {
                engine: engine(),
                channel: channel(),
            }),
            "(ChannelState ([prototype] [internal-message-router]))",
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
                engine: engine(),
                accepted_messages: 1,
                routed_messages: 1,
                deferred_messages: 0,
                failed_messages: 0,
            }),
            "(Summary ([prototype] 1 1 0 0))",
        ),
        (
            Output::MessageTrace(RouterMessageTrace {
                engine: engine(),
                message_slot: 7,
                status: RouterDeliveryStatus::Routed,
            }),
            "(MessageTrace ([prototype] 7 Routed))",
        ),
        (
            Output::MessageTraceMissing(RouterMessageTraceMissing {
                engine: engine(),
                message_slot: 99,
            }),
            "(MessageTraceMissing ([prototype] 99))",
        ),
        (
            Output::ChannelState(RouterChannelState {
                engine: engine(),
                channel: channel(),
                status: RouterChannelStatus::Installed,
            }),
            "(ChannelState ([prototype] [internal-message-router] Installed))",
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

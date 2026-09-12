use signal_router::{
    Actor, ActorRegistered, ActorRegistrationDisposition, ActorRegistrationRefusalReason,
    ActorRegistrationRefused, ByteViewable, EndpointKind, EndpointTransport, ForwardMarker,
    ForwardedMessagePayload, GrantDirectMessage, InstallStructuralChannels, OwnerIdentity, Query,
    RegisterActor, RegisterRemoteRouter, Response, Restorable, RoutedContractObject,
    RouterBootstrapDocument, RouterBootstrapOperation, RouterChannelState, RouterChannelStateQuery,
    RouterChannelStatus, RouterDaemonConfiguration, RouterDeliveryStatus, RouterForwardAccepted,
    RouterForwardRefusalReason, RouterForwardRefused, RouterForwardRequest, RouterIdentityProof,
    RouterMessageTrace, RouterMessageTraceMissing, RouterMessageTraceQuery, RouterObservationScope,
    RouterObservationUnimplemented, RouterObservationUnimplementedReason, RouterPeerAttestation,
    RouterRoutedObjectsAccepted, RouterRoutedObjectsRefused, RouterSessionAccepted,
    RouterSessionClientHello, RouterSessionClientProof, RouterSessionData, RouterSessionRefused,
    RouterSessionServerHello, RouterSummary, RouterSummaryQuery, SessionRefusalReason, Signal,
    Signalizable, SignatureScheme,
};

fn attestation() -> RouterPeerAttestation {
    RouterPeerAttestation {
        signer: String::from("goldragon"),
        scheme: SignatureScheme::Bls12_381MinPk,
        public_key: String::from("public-key"),
        signature: String::from("signature"),
        content_digest: String::from("content-digest"),
        issued_at: 1_757_600_000_000_000_000,
        nonce: String::from("nonce-1"),
        attestation_issued_at: 1_757_600_000_000_000_001,
    }
}

fn identity_proof() -> RouterIdentityProof {
    RouterIdentityProof {
        proof_signer: String::from("goldragon"),
        proof_scheme: SignatureScheme::Bls12_381MinSig,
        proof_public_key: String::from("public-key"),
        proof_signature: String::from("signature"),
        proof_digest: String::from("digest"),
        proof_challenge_nonce: String::from("nonce-2"),
        proof_attestation_issued_at: 1_757_600_000_000_000_002,
    }
}

fn forwarded_payload() -> ForwardedMessagePayload {
    ForwardedMessagePayload {
        source_actor: String::from("li"),
        destination_actor: String::from("claude"),
        body: String::from("rebase onto main, please"),
        attachments: vec![String::from("attachment-1")],
        routed_objects: vec![RoutedContractObject {
            contract_name: String::from("signal-router"),
            contract_operation: String::from("ForwardMessage"),
            contract_payload_size: 3,
            payload_octets: vec![1, 2, 3],
        }],
    }
}

fn actor() -> Actor {
    Actor {
        name: String::from("claude"),
        process: 4242,
        endpoint: Some(EndpointTransport {
            endpoint_kind: EndpointKind::HarnessSocket,
            target: String::from("/run/persona/harness.sock"),
            auxiliary: None,
        }),
    }
}

fn queries() -> Vec<Query> {
    vec![
        Query::Summary(RouterSummaryQuery {
            engine: String::from("engine-1"),
        }),
        Query::MessageTrace(RouterMessageTraceQuery {
            engine: String::from("engine-1"),
            message_slot: 7,
        }),
        Query::ChannelState(RouterChannelStateQuery {
            engine: String::from("engine-1"),
            channel: String::from("channel-aab"),
        }),
        Query::ForwardMessage(RouterForwardRequest {
            submission: forwarded_payload(),
            attestation: attestation(),
            forward_marker: ForwardMarker::Forwarded,
            nonce: String::from("nonce-3"),
            issued_at: 1_757_600_000_000_000_003,
        }),
        Query::SubmitRoutedObjects(forwarded_payload()),
        Query::SessionClientHello(RouterSessionClientHello {
            initiator_challenge: String::from("challenge-1"),
            initiator_ephemeral_key: String::from("ephemeral-1"),
        }),
        Query::SessionClientProof(RouterSessionClientProof {
            initiator_proof: identity_proof(),
        }),
        Query::SessionData(RouterSessionData {
            sealed_octets: vec![9, 8, 7],
        }),
        Query::RegisterActor(actor()),
    ]
}

fn responses() -> Vec<Response> {
    vec![
        Response::Summary(RouterSummary {
            engine: String::from("engine-1"),
            accepted_messages: 10,
            routed_messages: 9,
            deferred_messages: 1,
            failed_messages: 0,
        }),
        Response::MessageTrace(RouterMessageTrace {
            engine: String::from("engine-1"),
            message_slot: 7,
            delivery_status: RouterDeliveryStatus::Delivered,
        }),
        Response::MessageTraceMissing(RouterMessageTraceMissing {
            engine: String::from("engine-1"),
            message_slot: 8,
        }),
        Response::ChannelState(RouterChannelState {
            engine: String::from("engine-1"),
            channel: String::from("channel-aab"),
            channel_status: RouterChannelStatus::Installed,
        }),
        Response::ForwardAccepted(RouterForwardAccepted { message_slot: 7 }),
        Response::ForwardRefused(RouterForwardRefused {
            forward_refusal_reason: RouterForwardRefusalReason::ReplayDetected,
        }),
        Response::Unimplemented(RouterObservationUnimplemented {
            observation_scope: RouterObservationScope::MessageTrace,
            observation_reason: RouterObservationUnimplementedReason::MessageTraceUnavailable,
        }),
        Response::RoutedObjectsAccepted(RouterRoutedObjectsAccepted { message_slot: 11 }),
        Response::RoutedObjectsRefused(RouterRoutedObjectsRefused {
            forward_refusal_reason: RouterForwardRefusalReason::ChannelUnauthorized,
        }),
        Response::SessionServerHello(RouterSessionServerHello {
            responder_challenge: String::from("challenge-2"),
            responder_ephemeral_key: String::from("ephemeral-2"),
            responder_proof: identity_proof(),
        }),
        Response::SessionAccepted(RouterSessionAccepted {
            key_confirmation: vec![4, 5, 6],
        }),
        Response::SessionRefused(RouterSessionRefused {
            refusal_reason: SessionRefusalReason::ChallengeMismatch,
        }),
        Response::SessionData(RouterSessionData {
            sealed_octets: vec![9, 8, 7],
        }),
        Response::ActorRegistered(ActorRegistered {
            registered_actor: String::from("claude"),
            actor_registration_disposition: ActorRegistrationDisposition::EndpointUpdated,
        }),
        Response::ActorRegistrationRefused(ActorRegistrationRefused {
            registered_actor: String::from("claude"),
            actor_registration_refusal_reason:
                ActorRegistrationRefusalReason::RemoteRouterEndpointNotLocal,
        }),
    ]
}

fn configuration() -> RouterDaemonConfiguration {
    RouterDaemonConfiguration {
        router_socket_path: String::from("/run/router/router.sock"),
        router_socket_mode: 0o600,
        meta_router_socket_path: String::from("/run/router/meta-router.sock"),
        meta_router_socket_mode: 0o600,
        supervision_socket_path: String::from("/run/router/supervision.sock"),
        supervision_socket_mode: 0o600,
        store_path: String::from("/var/lib/router/store"),
        bootstrap_path: Some(String::from("/var/lib/router/bootstrap.datom")),
        owner_identity: OwnerIdentity::UnixUser(1000),
        tailnet_listen_address: Some(String::from("100.64.0.1")),
        router_identity: String::from("goldragon"),
        criome_socket_path: None,
    }
}

fn bootstrap_document() -> RouterBootstrapDocument {
    vec![
        RouterBootstrapOperation::RegisterActor(RegisterActor {
            actor: actor(),
            home: Some(String::from("goldragon")),
        }),
        RouterBootstrapOperation::GrantDirectMessage(GrantDirectMessage {
            source_actor: String::from("li"),
            destination_actor: String::from("claude"),
        }),
        RouterBootstrapOperation::InstallStructuralChannels(InstallStructuralChannels {
            requester: String::from("li"),
        }),
        RouterBootstrapOperation::RegisterRemoteRouter(RegisterRemoteRouter {
            identity: String::from("hexis"),
            address: String::from("100.64.0.2"),
        }),
    ]
}

#[test]
fn queries_round_trip_through_received_bytes() {
    for query in queries() {
        let received =
            Signal::<Query>::from(query.signalize().expect("query archives").bytes().to_vec());
        assert_eq!(received.restore().expect("query restores"), query);
    }
}

#[test]
fn responses_round_trip_through_received_bytes() {
    for response in responses() {
        let received = Signal::<Response>::from(
            response
                .signalize()
                .expect("response archives")
                .bytes()
                .to_vec(),
        );
        assert_eq!(received.restore().expect("response restores"), response);
    }
}

#[test]
fn the_daemon_configuration_round_trips_through_received_bytes() {
    let configuration = configuration();
    let received = Signal::<RouterDaemonConfiguration>::from(
        configuration
            .signalize()
            .expect("configuration archives")
            .bytes()
            .to_vec(),
    );
    assert_eq!(
        received.restore().expect("configuration restores"),
        configuration
    );
}

#[test]
fn a_bootstrap_document_round_trips_through_received_bytes() {
    let document = bootstrap_document();
    let received = Signal::<RouterBootstrapDocument>::from(
        document
            .signalize()
            .expect("document archives")
            .bytes()
            .to_vec(),
    );
    assert_eq!(received.restore().expect("document restores"), document);
}

#[test]
fn malformed_peer_bytes_are_rejected() {
    assert!(Signal::<Query>::from(vec![0xff, 0, 1]).restore().is_err());
}

/// The bare heads a router speaks must stay bare. Ethos Zero gives a variant
/// whose head spells a declared type that type as its payload, so a tag that
/// collided with a type name would silently start carrying one. Carrying
/// these over the wire is the witness that they do not.
#[test]
fn bare_heads_cross_the_wire_as_bare_heads() {
    let response = Response::Unimplemented(RouterObservationUnimplemented {
        observation_scope: RouterObservationScope::Summary,
        observation_reason: RouterObservationUnimplementedReason::NotInPrototypeScope,
    });
    let received = Signal::<Response>::from(
        response
            .signalize()
            .expect("response archives")
            .bytes()
            .to_vec(),
    );
    assert_eq!(received.restore().expect("response restores"), response);
}

#[cfg(feature = "datom")]
mod datom {
    use super::*;
    use datom_codec::{Actualizing, Budget, Datomizable, Potential};
    use protos::{Protosizable, ReaderBudget, Textualizable};

    /// The ceiling a router applies to peer-supplied Datom text: one mebibyte
    /// of extent and two hundred fifty-six levels of descent. The wire type
    /// constrains neither, so the reader must.
    const PEER_TEXT_EXTENT: usize = 1024 * 1024;
    const PEER_TEXT_DEPTH: i64 = 256;

    fn budget() -> Budget {
        Budget {
            remaining: i64::try_from(PEER_TEXT_EXTENT).expect("the extent fits an i64"),
            reader: ReaderBudget {
                remaining: PEER_TEXT_EXTENT,
            },
            depth: 0,
            maximum_depth: PEER_TEXT_DEPTH,
        }
    }

    fn textualize<T: Datomizable<Output = datom_codec::Datom>>(value: T) -> String {
        value.datomize(vec![]).protosize().textualize()
    }

    #[test]
    fn queries_round_trip_as_datom_text() {
        for query in queries() {
            let text = textualize(query.clone());
            let restored = Potential::<Query>::from(text)
                .actualize(&mut budget())
                .expect("query actualizes");
            assert_eq!(restored, query);
        }
    }

    #[test]
    fn responses_round_trip_as_datom_text() {
        for response in responses() {
            let text = textualize(response.clone());
            let restored = Potential::<Response>::from(text)
                .actualize(&mut budget())
                .expect("response actualizes");
            assert_eq!(restored, response);
        }
    }

    #[test]
    fn the_daemon_configuration_round_trips_as_datom_text() {
        let configuration = configuration();
        let text = textualize(configuration.clone());
        let restored = Potential::<RouterDaemonConfiguration>::from(text)
            .actualize(&mut budget())
            .expect("configuration actualizes");
        assert_eq!(restored, configuration);
    }

    #[test]
    fn a_bootstrap_document_round_trips_as_datom_text() {
        let document = bootstrap_document();
        let text = textualize(document.clone());
        let restored = Potential::<RouterBootstrapDocument>::from(text)
            .actualize(&mut budget())
            .expect("document actualizes");
        assert_eq!(restored, document);
    }

    #[test]
    fn every_canonical_datom_line_actualizes_into_a_contract_head() {
        let canonical = include_str!("../examples/canonical.datom");
        let mut lines = 0;
        for line in canonical.lines() {
            let line = line.trim();
            if line.is_empty() || line.starts_with(';') {
                continue;
            }
            lines += 1;
            let actualized = Potential::<Query>::from(line.to_owned())
                .actualize(&mut budget())
                .is_ok()
                || Potential::<Response>::from(line.to_owned())
                    .actualize(&mut budget())
                    .is_ok()
                || Potential::<RouterDaemonConfiguration>::from(line.to_owned())
                    .actualize(&mut budget())
                    .is_ok()
                || Potential::<RouterBootstrapDocument>::from(line.to_owned())
                    .actualize(&mut budget())
                    .is_ok();
            assert!(actualized, "canonical line is no contract head: {line}");
        }
        assert_eq!(
            lines, 26,
            "canonical file should carry twenty-six contract heads"
        );
    }

    #[test]
    fn peer_text_beyond_the_extent_is_refused() {
        let mut exhausted = Budget {
            remaining: 4,
            reader: ReaderBudget { remaining: 4 },
            depth: 0,
            maximum_depth: PEER_TEXT_DEPTH,
        };
        let text = textualize(Query::ForwardMessage(RouterForwardRequest {
            submission: forwarded_payload(),
            attestation: attestation(),
            forward_marker: ForwardMarker::Origin,
            nonce: String::from("nonce-4"),
            issued_at: 1,
        }));
        assert!(
            Potential::<Query>::from(text)
                .actualize(&mut exhausted)
                .is_err()
        );
    }
}

use signal_router::*;

fn requests() -> Vec<(z2VZGC, &'static str)> {
    vec![
        (
            z2VZGC::z2VdPV(z2VZVo {
                field_0: z2VQcL::new(z2VbUg::new("fixture".to_owned())),
                field_1: z2VcsB::new(z2VUhk::new("fixture".to_owned())),
            }),
            "ChannelState",
        ),
        (
            z2VZGC::z2Vdxj(z2VNid {
                field_0: z2VVbN::new(z2VNMz::new("fixture".to_owned())),
                field_1: z2VVYB::new(z2VNMz::new("fixture".to_owned())),
                field_2: z2VYUB::new("fixture".to_owned()),
                field_3: Vec::new(),
                field_4: Vec::new(),
            }),
            "SubmitRoutedObjects",
        ),
        (
            z2VZGC::z2Vd1x(z2VRcj {
                field_0: z2VX9R::new(z2VNid {
                    field_0: z2VVbN::new(z2VNMz::new("fixture".to_owned())),
                    field_1: z2VVYB::new(z2VNMz::new("fixture".to_owned())),
                    field_2: z2VYUB::new("fixture".to_owned()),
                    field_3: Vec::new(),
                    field_4: Vec::new(),
                }),
                field_1: z2VL7S::new(z2VWsQ {
                    field_0: z2VLTy::new(z2VNwn::new("fixture".to_owned())),
                    field_1: z2VZU5::new(z2VLuJ::z2VYLS),
                    field_2: z2VVAk::new("fixture".to_owned()),
                    field_3: z2VZX5::new("fixture".to_owned()),
                    field_4: z2Vd3E::new("fixture".to_owned()),
                    field_5: z2Vd2q::new(z2VQGK::new(7)),
                    field_6: z2VcpN::new(z2VLFW::new("fixture".to_owned())),
                    field_7: z2VSgE::new(z2VQGK::new(7)),
                }),
                field_2: z2VVui::new(z2VMPZ::z2VUf6),
                field_3: z2VcpN::new(z2VLFW::new("fixture".to_owned())),
                field_4: z2Vd2q::new(z2VQGK::new(7)),
            }),
            "ForwardMessage",
        ),
        (
            z2VZGC::z2VMyr(z2VNj2 {
                field_0: z2VQcL::new(z2VbUg::new("fixture".to_owned())),
            }),
            "Summary",
        ),
        (
            z2VZGC::z2VQMp(z2VUFf {
                field_0: z2VSH4::new(z2VPfN {
                    field_0: z2VdEo::new(z2VNwn::new("fixture".to_owned())),
                    field_1: z2VVd5::new(z2VLuJ::z2VYLS),
                    field_2: z2VQew::new("fixture".to_owned()),
                    field_3: z2VbVV::new("fixture".to_owned()),
                    field_4: z2Vaf5::new("fixture".to_owned()),
                    field_5: z2Vd1b::new(z2VLFW::new("fixture".to_owned())),
                    field_6: z2VUTN::new(z2VQGK::new(7)),
                }),
            }),
            "SessionClientProof",
        ),
        (
            z2VZGC::z2VMN6(z2VWLp {
                field_0: z2VPs7::new(z2VbxJ::new("fixture".to_owned())),
                field_1: z2VaLm::new(z2VeFW::new("fixture".to_owned())),
            }),
            "SessionClientHello",
        ),
        (
            z2VZGC::z2Vd2e(z2VNxf {
                field_0: Vec::new(),
            }),
            "SessionData",
        ),
        (
            z2VZGC::z2VWdr(z2VTFJ {
                field_0: z2VQ8d::new(z2VNMz::new("fixture".to_owned())),
                field_1: z2VVdV::new(7),
                field_2: None,
            }),
            "RegisterActor",
        ),
        (
            z2VZGC::z2VWzG(z2VYdo {
                field_0: z2VQcL::new(z2VbUg::new("fixture".to_owned())),
                field_1: z2VXxh::new(7),
            }),
            "MessageTrace",
        ),
    ]
}

fn replies() -> Vec<(z2VXoV, &'static str)> {
    vec![
        (
            z2VXoV::z2VYBQ(z2VY1V {
                field_0: z2VQcL::new(z2VbUg::new("fixture".to_owned())),
                field_1: z2VcsB::new(z2VUhk::new("fixture".to_owned())),
                field_2: z2VSRk::new(z2VRts::z2VTCB),
            }),
            "ChannelState",
        ),
        (
            z2VXoV::z2VNok(z2VMZv {
                field_0: z2VQcL::new(z2VbUg::new("fixture".to_owned())),
                field_1: z2VXxh::new(7),
                field_2: z2VQHz::new(z2VMfS::z2Ve98),
            }),
            "MessageTrace",
        ),
        (
            z2VXoV::z2VPKn(z2VNxf {
                field_0: Vec::new(),
            }),
            "SessionData",
        ),
        (
            z2VXoV::z2VLzz(z2VSdJ::new(z2VNRo::new(z2VQC7::z2VP62))),
            "RoutedObjectsRefused",
        ),
        (
            z2VXoV::z2VcmP(z2VR5k {
                field_0: z2VQcL::new(z2VbUg::new("fixture".to_owned())),
                field_1: z2VST8::new(7),
                field_2: z2VcvY::new(7),
                field_3: z2VPky::new(7),
                field_4: z2Vc6c::new(7),
            }),
            "Summary",
        ),
        (
            z2VXoV::z2VV3E(z2Vbzo {
                field_0: z2VSqw::new(z2VNMz::new("fixture".to_owned())),
                field_1: z2VTf7::z2VTZK,
            }),
            "ActorRegistrationRefused",
        ),
        (
            z2VXoV::z2VPBN(z2VT1o::new(z2VNRo::new(z2VQC7::z2VP62))),
            "ForwardRefused",
        ),
        (
            z2VXoV::z2VcdN(z2VNd2 {
                field_0: z2VQcL::new(z2VbUg::new("fixture".to_owned())),
                field_1: z2VXxh::new(7),
            }),
            "MessageTraceMissing",
        ),
        (
            z2VXoV::z2Vc2V(z2VQWj {
                field_0: z2VXxh::new(7),
            }),
            "RoutedObjectsAccepted",
        ),
        (
            z2VXoV::z2VNhW(z2VPaP {
                field_0: z2VURC::new(z2VVNQ::z2Va7j),
            }),
            "SessionRefused",
        ),
        (
            z2VXoV::z2VeUs(z2VNK3 {
                field_0: z2VSqw::new(z2VNMz::new("fixture".to_owned())),
                field_1: z2VZMx::z2VMfh,
            }),
            "ActorRegistered",
        ),
        (
            z2VXoV::z2VSmt(z2VW7b {
                field_0: z2VfEY::new(z2VLWZ::z2Vcer),
                field_1: z2VZLC::new(z2VQNh::z2VbiE),
            }),
            "Unimplemented",
        ),
        (
            z2VXoV::z2VcTY(z2VYXM {
                field_0: Vec::new(),
            }),
            "SessionAccepted",
        ),
        (
            z2VXoV::z2VTty(z2VLKE {
                field_0: z2VNKw::new(z2VbxJ::new("fixture".to_owned())),
                field_1: z2VW7S::new(z2VeFW::new("fixture".to_owned())),
                field_2: z2VZXG::new(z2VPfN {
                    field_0: z2VdEo::new(z2VNwn::new("fixture".to_owned())),
                    field_1: z2VVd5::new(z2VLuJ::z2VYLS),
                    field_2: z2VQew::new("fixture".to_owned()),
                    field_3: z2VbVV::new("fixture".to_owned()),
                    field_4: z2Vaf5::new("fixture".to_owned()),
                    field_5: z2Vd1b::new(z2VLFW::new("fixture".to_owned())),
                    field_6: z2VUTN::new(z2VQGK::new(7)),
                }),
            }),
            "SessionServerHello",
        ),
        (
            z2VXoV::z2VTwc(z2VQGg {
                field_0: z2VXxh::new(7),
            }),
            "ForwardAccepted",
        ),
    ]
}

#[test]
fn every_request_round_trips_through_the_bound_frame() {
    for (request, _head) in requests() {
        let expected = request.clone();
        let exchange = signal_frame::ExchangeIdentifier::new(
            signal_frame::SessionEpoch::new(31),
            signal_frame::ExchangeLane::Connector,
            signal_frame::LaneSequence::first(),
        );
        let encoded = request
            .encode_request_frame(exchange)
            .expect("request frame encodes");
        let (decoded_exchange, decoded) =
            ContractMarker::decode_single_request(&encoded).expect("request frame decodes");
        assert_eq!(decoded_exchange, exchange);
        assert_eq!(decoded, expected);
    }
}

#[test]
fn every_reply_has_bound_frame_and_rkyv_behavior() {
    for (reply, _head) in replies() {
        let expected = reply.clone();
        let exchange = signal_frame::ExchangeIdentifier::new(
            signal_frame::SessionEpoch::new(37),
            signal_frame::ExchangeLane::Connector,
            signal_frame::LaneSequence::first(),
        );
        let encoded = reply
            .clone()
            .encode_reply_frame(exchange)
            .expect("reply frame encodes");
        ContractMarker::decode_frame(&encoded).expect("reply frame decodes");

        let archive = rkyv::to_bytes::<rkyv::rancor::Error>(&reply).expect("reply archives");
        let recovered =
            rkyv::from_bytes::<z2VXoV, rkyv::rancor::Error>(&archive).expect("reply recovers");
        assert_eq!(recovered, expected);
    }
}

#[cfg(feature = "dotos-text")]
#[test]
fn every_root_round_trips_through_dotos_with_visible_heads() {
    use dotos::{DotosEncode, DotosSource};

    for (request, head) in requests() {
        let text = request.to_dotos();
        assert!(text.starts_with(&format!("{head}.")), "{text}");
        let recovered = DotosSource::new(&text)
            .parse::<z2VZGC>()
            .expect("request Dotos decodes");
        assert_eq!(recovered, request);
    }
    for (reply, head) in replies() {
        let text = reply.to_dotos();
        assert!(text.starts_with(&format!("{head}.")), "{text}");
        let recovered = DotosSource::new(&text)
            .parse::<z2VXoV>()
            .expect("reply Dotos decodes");
        assert_eq!(recovered, reply);
    }
}

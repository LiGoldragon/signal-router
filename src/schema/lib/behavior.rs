// Handwritten operational behavior for the authority-verified ordinary Router Interface.
//
// The strict bootstrap projection owns every structural type below. This file
// owns only behavior the current bootstrap language cannot yet express:
// structural runtime traits, the ordinary Input/Output role seating, and the
// allocated Signal frame boundary.

use rkyv::{
    Archive, Deserialize as RkyvDeserialize, Serialize as RkyvSerialize,
    rancor::Source as _,
};

#[derive(Archive, RkyvSerialize, RkyvDeserialize, Debug, Clone, PartialEq, Eq)]
#[rkyv(serialize_bounds(
    __S: rkyv::ser::Writer + rkyv::ser::Allocator,
    __S::Error: rkyv::rancor::Source,
))]
#[rkyv(deserialize_bounds(__D::Error: rkyv::rancor::Source))]
#[rkyv(bytecheck(bounds(__C: rkyv::validation::ArchiveContext)))]
#[doc(hidden)]
pub enum WireValue {
    Text(std::string::String), Integer(u64), Boolean(bool),
    Sequence(#[rkyv(omit_bounds)] Vec<WireValue>),
    Absent, Present(#[rkyv(omit_bounds)] Box<WireValue>),
    Product(#[rkyv(omit_bounds)] Vec<WireValue>),
    Variant { ordinal: u16, #[rkyv(omit_bounds)] fields: Vec<WireValue> },
}
#[derive(Debug, thiserror::Error)]
#[error("structural wire value does not match the authority-verified Interface")]
#[doc(hidden)]
pub struct WireShapeError;

/// Current-stage structural behavior shared by Interfaces that import these
/// producer-owned types.
#[doc(hidden)]
pub trait WireShape: Sized {
    fn to_wire(&self) -> WireValue;
    fn from_wire(value: WireValue) -> Result<Self, WireShapeError>;
}

impl WireShape for std::string::String {
    fn to_wire(&self) -> WireValue { WireValue::Text(self.clone()) }
    fn from_wire(value: WireValue) -> Result<Self, WireShapeError> { match value { WireValue::Text(value) => Ok(value), _ => Err(WireShapeError) } }
}
impl WireShape for u64 {
    fn to_wire(&self) -> WireValue { WireValue::Integer(*self) }
    fn from_wire(value: WireValue) -> Result<Self, WireShapeError> { match value { WireValue::Integer(value) => Ok(value), _ => Err(WireShapeError) } }
}
impl WireShape for bool {
    fn to_wire(&self) -> WireValue { WireValue::Boolean(*self) }
    fn from_wire(value: WireValue) -> Result<Self, WireShapeError> { match value { WireValue::Boolean(value) => Ok(value), _ => Err(WireShapeError) } }
}
impl<Value: WireShape> WireShape for Vec<Value> {
    fn to_wire(&self) -> WireValue { WireValue::Sequence(self.iter().map(WireShape::to_wire).collect()) }
    fn from_wire(value: WireValue) -> Result<Self, WireShapeError> {
        let WireValue::Sequence(values) = value else { return Err(WireShapeError) };
        values.into_iter().map(Value::from_wire).collect()
    }
}
impl<Value: WireShape> WireShape for Option<Value> {
    fn to_wire(&self) -> WireValue { match self { Some(value) => WireValue::Present(Box::new(value.to_wire())), None => WireValue::Absent } }
    fn from_wire(value: WireValue) -> Result<Self, WireShapeError> {
        match value { WireValue::Present(value) => Ok(Some(Value::from_wire(*value)?)), WireValue::Absent => Ok(None), _ => Err(WireShapeError) }
    }
}
fn one_field(mut fields: Vec<WireValue>) -> Result<WireValue, WireShapeError> {
    if fields.len() != 1 { return Err(WireShapeError); }
    Ok(fields.pop().expect("one field checked"))
}

macro_rules! wire_traits {
    ($name:ident) => {
        impl Clone for $name { fn clone(&self) -> Self { Self::from_wire(self.to_wire()).expect("a projected value revalidates") } }
        impl std::fmt::Debug for $name { fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result { self.to_wire().fmt(formatter) } }
        impl PartialEq for $name { fn eq(&self, other: &Self) -> bool { self.to_wire() == other.to_wire() } }
        impl Eq for $name {}
    };
}
macro_rules! wire_external_newtype {
    ($name:ident, $inner:ty) => {
        impl WireShape for $name {
            fn to_wire(&self) -> WireValue { self.payload().to_wire() }
            fn from_wire(value: WireValue) -> Result<Self, WireShapeError> { Ok(Self::new(<$inner as WireShape>::from_wire(value)?)) }
        }
        wire_traits!($name);
        #[cfg(feature = "dotos-text")]
        impl dotos::DotosEncode for $name {
            fn to_dotos(&self) -> std::string::String {
                dotos::DotosEncode::to_dotos(self.payload())
            }
        }
        #[cfg(feature = "dotos-text")]
        impl dotos::DotosDecode for $name {
            fn from_dotos_block(block: &dotos::Block) -> Result<Self, dotos::DotosDecodeError> {
                <$inner as dotos::DotosDecode>::from_dotos_block(block).map(Self::new)
            }
        }
    };
}
macro_rules! wire_newtype {
    ($name:ident, $inner:ty) => {
        impl $name {
            pub fn new(payload: $inner) -> Self { Self(payload) }
            pub fn payload(&self) -> &$inner { &self.0 }
            pub fn into_payload(self) -> $inner { self.0 }
        }
        impl WireShape for $name {
            fn to_wire(&self) -> WireValue { self.0.to_wire() }
            fn from_wire(value: WireValue) -> Result<Self, WireShapeError> { Ok(Self(<$inner as WireShape>::from_wire(value)?)) }
        }
        wire_traits!($name);
        #[cfg(feature = "dotos-text")]
        impl dotos::DotosEncode for $name {
            fn to_dotos(&self) -> std::string::String {
                dotos::DotosEncode::to_dotos(&self.0)
            }
        }
        #[cfg(feature = "dotos-text")]
        impl dotos::DotosDecode for $name {
            fn from_dotos_block(block: &dotos::Block) -> Result<Self, dotos::DotosDecodeError> {
                <$inner as dotos::DotosDecode>::from_dotos_block(block).map(Self)
            }
        }
    };
}
macro_rules! wire_struct {
    ($name:ident { $($field:ident: $field_type:ty),* $(,)? }) => {
        impl WireShape for $name {
            fn to_wire(&self) -> WireValue { WireValue::Product(vec![$(self.$field.to_wire()),*]) }
            fn from_wire(value: WireValue) -> Result<Self, WireShapeError> {
                let WireValue::Product(fields) = value else { return Err(WireShapeError) };
                let mut fields = fields.into_iter();
                let result = Self { $($field: <$field_type as WireShape>::from_wire(fields.next().ok_or(WireShapeError)?)?),* };
                if fields.next().is_some() { return Err(WireShapeError); }
                Ok(result)
            }
        }
        wire_traits!($name);
        #[cfg(feature = "dotos-text")]
        impl dotos::DotosEncode for $name {
            fn to_dotos(&self) -> std::string::String {
                dotos::Delimiter::Parenthesis.wrap([
                    $(dotos::DotosEncode::to_dotos(&self.$field)),*
                ])
            }
        }
        #[cfg(feature = "dotos-text")]
        impl dotos::DotosDecode for $name {
            fn from_dotos_block(block: &dotos::Block) -> Result<Self, dotos::DotosDecodeError> {
                let body = dotos::DotosBody::from_delimited(
                    block,
                    dotos::Delimiter::Parenthesis,
                    stringify!($name),
                )?;
                let expected = 0usize $(+ {
                    let _ = stringify!($field);
                    1usize
                })*;
                #[allow(unused_mut, unused_variables)]
                let mut fields = body.expect_fields(stringify!($name), expected)?.iter();
                Ok(Self {
                    $($field: <$field_type as dotos::DotosDecode>::from_dotos_block(
                        fields.next().expect("field count checked"),
                    )?),*
                })
            }
        }
    };
}
macro_rules! wire_enum {
    ($name:ident {
        unit { $($unit_ordinal:literal => $unit:ident : $unit_visible:literal),* $(,)? }
        unary { $($unary_ordinal:literal => $unary:ident($payload:ty) : $unary_visible:literal),* $(,)? }
    }) => {
        impl WireShape for $name {
            fn to_wire(&self) -> WireValue {
                match self {
                    $(Self::$unit => WireValue::Variant { ordinal: $unit_ordinal, fields: Vec::new() },)*
                    $(Self::$unary(payload) => WireValue::Variant { ordinal: $unary_ordinal, fields: vec![payload.to_wire()] },)*
                }
            }
            fn from_wire(value: WireValue) -> Result<Self, WireShapeError> {
                let WireValue::Variant { ordinal, fields } = value else { return Err(WireShapeError) };
                match ordinal {
                    $($unit_ordinal if fields.is_empty() => Ok(Self::$unit),)*
                    $($unary_ordinal => Ok(Self::$unary(<$payload as WireShape>::from_wire(one_field(fields)?)?)),)*
                    _ => Err(WireShapeError),
                }
            }
        }
        wire_traits!($name);
        #[cfg(feature = "dotos-text")]
        impl dotos::DotosEncode for $name {
            fn to_dotos(&self) -> std::string::String {
                match self {
                    $(Self::$unit => $unit_visible.to_owned(),)*
                    $(Self::$unary(payload) => format!(
                        "{}.{}",
                        $unary_visible,
                        dotos::DotosEncode::to_dotos(payload),
                    ),)*
                }
            }
        }
        #[cfg(feature = "dotos-text")]
        impl dotos::DotosDecode for $name {
            fn from_dotos_block(block: &dotos::Block) -> Result<Self, dotos::DotosDecodeError> {
                if let Some(variant) = block.demote_to_string() {
                    return match variant {
                        $($unit_visible => Ok(Self::$unit),)*
                        _ => Err(dotos::DotosDecodeError::UnknownVariant {
                            enum_name: stringify!($name),
                            variant: variant.to_owned(),
                        }),
                    };
                }
                let (head, payload) = block.as_application().ok_or(
                    dotos::DotosDecodeError::ExpectedAtom { type_name: stringify!($name) },
                )?;
                let _ = &payload;
                let variant = head.demote_to_string().ok_or(
                    dotos::DotosDecodeError::ExpectedAtom { type_name: stringify!($name) },
                )?;
                match variant {
                    $($unary_visible => Ok(Self::$unary(
                        <$payload as dotos::DotosDecode>::from_dotos_block(payload)?,
                    )),)*
                    _ => Err(dotos::DotosDecodeError::UnknownVariant {
                        enum_name: stringify!($name),
                        variant: variant.to_owned(),
                    }),
                }
            }
        }
    };
}
wire_newtype!(z2VVbN, z2VNMz);
wire_newtype!(z2VMR5, z2Vf1e);
wire_newtype!(z2VQHz, z2VMfS);
wire_struct!(z2VPfN { field_0: z2VdEo, field_1: z2VVd5, field_2: z2VQew, field_3: z2VbVV, field_4: z2Vaf5, field_5: z2Vd1b, field_6: z2VUTN });
wire_external_newtype!(z2VQGK, u64);
wire_external_newtype!(z2VXQX, std::string::String);
wire_newtype!(z2VNRo, z2VQC7);
wire_newtype!(z2VX9R, z2VNid);
wire_external_newtype!(z2VV5h, std::string::String);
wire_newtype!(z2VaLm, z2VeFW);
wire_external_newtype!(z2VbKU, std::string::String);
wire_enum!(z2VQNh { unit { 0 => z2VbiE : "MessageTraceUnavailable", 1 => z2VLVC : "NotInPrototypeScope", 2 => z2VaYK : "RouterStoreUnavailable" } unary {  } });
wire_newtype!(z2VXG3, z2Vf1e);
wire_struct!(z2VWUQ { field_0: Vec< z2VNh2> });
wire_struct!(z2VNid { field_0: z2VVbN, field_1: z2VVYB, field_2: z2VYUB, field_3: Vec< std::string::String>, field_4: Vec< z2Vcrd> });
wire_external_newtype!(z2VbUg, std::string::String);
wire_struct!(z2VRcj { field_0: z2VX9R, field_1: z2VL7S, field_2: z2VVui, field_3: z2VcpN, field_4: z2Vd2q });
wire_enum!(z2VRts { unit { 0 => z2VTCB : "Installed", 1 => z2VMxJ : "Missing", 2 => z2VcN3 : "Disabled" } unary {  } });
wire_newtype!(z2VfEY, z2VLWZ);
wire_struct!(z2VZVo { field_0: z2VQcL, field_1: z2VcsB });
wire_newtype!(z2VZNB, z2VaJt);
wire_external_newtype!(z2VST8, u64);
wire_newtype!(z2Vdz8, z2VNwn);
wire_struct!(z2VQGg { field_0: z2VXxh });
wire_struct!(z2VYXM { field_0: Vec< u64> });
wire_struct!(z2VY1V { field_0: z2VQcL, field_1: z2VcsB, field_2: z2VSRk });
wire_newtype!(z2VcTK, z2VPn3);
wire_newtype!(z2VW7S, z2VeFW);
wire_external_newtype!(z2VPky, u64);
wire_external_newtype!(z2VUhk, std::string::String);
wire_newtype!(z2VURC, z2VVNQ);
wire_external_newtype!(z2VbxJ, std::string::String);
wire_enum!(z2VMfS { unit { 0 => z2Ve98 : "ForwardedRemote", 1 => z2VPk5 : "Deferred", 2 => z2VYCc : "Accepted", 3 => z2VWbi : "Delivered", 4 => z2VcFF : "Routed", 5 => z2Vd6L : "Failed" } unary {  } });
wire_newtype!(z2VZU5, z2VLuJ);
wire_struct!(z2VWsQ { field_0: z2VLTy, field_1: z2VZU5, field_2: z2VVAk, field_3: z2VZX5, field_4: z2Vd3E, field_5: z2Vd2q, field_6: z2VcpN, field_7: z2VSgE });
wire_newtype!(z2VZLC, z2VQNh);
wire_newtype!(z2VSdJ, z2VNRo);
wire_struct!(z2VW7b { field_0: z2VfEY, field_1: z2VZLC });
wire_struct!(z2VPaP { field_0: z2VURC });
wire_external_newtype!(z2VW1y, std::string::String);
wire_enum!(z2VZMx { unit { 0 => z2VMfh : "Registered", 1 => z2VRvr : "EndpointUpdated" } unary {  } });
wire_struct!(z2VQWj { field_0: z2VXxh });
wire_struct!(z2Vcrd { field_0: z2VbKU, field_1: z2VV5h, field_2: z2VPAH, field_3: Vec< u64> });
wire_newtype!(z2Vafp, z2VNwn);
wire_external_newtype!(z2VXxh, u64);
wire_enum!(z2VLWZ { unit { 0 => z2Vcer : "MessageTrace", 1 => z2VTUE : "ChannelState", 2 => z2VNvN : "Summary" } unary {  } });
wire_newtype!(z2Vd1b, z2VLFW);
wire_newtype!(z2VcsM, z2VPn3);
wire_struct!(z2VNK3 { field_0: z2VSqw, field_1: z2VZMx });
wire_external_newtype!(z2Vaf5, std::string::String);
wire_newtype!(z2VdEo, z2VNwn);
wire_newtype!(z2VcsB, z2VUhk);
wire_external_newtype!(z2VLFW, std::string::String);
wire_struct!(z2VNd2 { field_0: z2VQcL, field_1: z2VXxh });
wire_struct!(z2VZfL { field_0: z2Ve3n, field_1: z2VbSs, field_2: z2Va6n, field_3: z2VMR5, field_4: z2VcsM, field_5: z2VXG3, field_6: z2VcTK, field_7: Option< z2VPn3>, field_8: z2VLx1, field_9: Option< z2VVPx>, field_10: z2Vafp, field_11: Option< z2VPn3> });
wire_newtype!(z2VcpN, z2VLFW);
wire_struct!(z2VWLp { field_0: z2VPs7, field_1: z2VaLm });
wire_struct!(z2VPdn { field_0: z2VTFJ, field_1: Option< z2VNwn> });
wire_enum!(z2VNh2 { unit {  } unary { 0 => z2VQBJ(z2VPdn) : "RegisterActor", 1 => z2VUXc(z2VPkk) : "GrantDirectMessage", 2 => z2VRpm(z2VPGB) : "InstallStructuralChannels", 3 => z2VdzM(z2VWkj) : "RegisterRemoteRouter" } });
wire_newtype!(z2VbSs, z2Vf1e);
wire_enum!(z2VZGC { unit {  } unary { 0 => z2VdPV(z2VZVo) : "ChannelState", 1 => z2Vdxj(z2VNid) : "SubmitRoutedObjects", 2 => z2Vd1x(z2VRcj) : "ForwardMessage", 3 => z2VMyr(z2VNj2) : "Summary", 4 => z2VQMp(z2VUFf) : "SessionClientProof", 5 => z2VMN6(z2VWLp) : "SessionClientHello", 6 => z2Vd2e(z2VNxf) : "SessionData", 7 => z2VWdr(z2VTFJ) : "RegisterActor", 8 => z2VWzG(z2VYdo) : "MessageTrace" } });
wire_newtype!(z2VSqw, z2VNMz);
wire_newtype!(z2VQcL, z2VbUg);
wire_struct!(z2VYdo { field_0: z2VQcL, field_1: z2VXxh });
wire_external_newtype!(z2Vf1e, u64);
wire_newtype!(z2VZVN, z2VNMz);
wire_external_newtype!(z2VVPx, std::string::String);
wire_newtype!(z2VLTy, z2VNwn);
wire_external_newtype!(z2VPAH, u64);
wire_enum!(z2VLuJ { unit { 0 => z2VYLS : "Bls12_381MinSig", 1 => z2VYiG : "Bls12_381MinPk" } unary {  } });
wire_external_newtype!(z2VQew, std::string::String);
wire_struct!(z2VNxf { field_0: Vec< u64> });
wire_enum!(z2VaJt { unit { 0 => z2VYWP : "RemoteRouter", 1 => z2VXix : "Human", 2 => z2VNJF : "HarnessSocket", 3 => z2VUHw : "ComponentSocket", 4 => z2Vd77 : "PtySocket" } unary {  } });
wire_external_newtype!(z2VbVV, std::string::String);
wire_newtype!(z2VVui, z2VMPZ);
wire_newtype!(z2VUTN, z2VQGK);
wire_external_newtype!(z2VNMz, std::string::String);
wire_struct!(z2VTFJ { field_0: z2VQ8d, field_1: z2VVdV, field_2: Option< z2VLce> });
wire_struct!(z2VMZv { field_0: z2VQcL, field_1: z2VXxh, field_2: z2VQHz });
wire_newtype!(z2VZXG, z2VPfN);
wire_enum!(z2VVNQ { unit { 0 => z2Va7j : "SessionCipherFailure", 1 => z2Vb6c : "IdentityProofInvalid", 2 => z2VXec : "ChallengeMismatch", 3 => z2VY23 : "HandshakeMalformed" } unary {  } });
wire_external_newtype!(z2VcvY, u64);
wire_enum!(z2VQC7 { unit { 0 => z2VP62 : "AlreadyForwarded", 1 => z2VepV : "SessionRequired", 2 => z2VTee : "MirrorDisabled", 3 => z2VY2G : "RecipientUnknown", 4 => z2VLzK : "AttestationInvalid", 5 => z2VYnV : "ReplayDetected", 6 => z2VXed : "UnknownPeer", 7 => z2VZ7i : "ChannelUnauthorized", 8 => z2VXYJ : "ClockSkew" } unary {  } });
wire_newtype!(z2Ve3n, z2VPn3);
wire_external_newtype!(z2VPn3, std::string::String);
wire_external_newtype!(z2VYUB, std::string::String);
wire_struct!(z2VWkj { field_0: z2Vdz8, field_1: z2VbwY });
wire_newtype!(z2VbwY, z2VVPx);
wire_newtype!(z2VSgE, z2VQGK);
wire_newtype!(z2Va6n, z2VPn3);
wire_enum!(z2VTf7 { unit { 0 => z2VTZK : "RemoteRouterEndpointNotLocal", 1 => z2VTKg : "ProcessIdentifierOutOfRange" } unary {  } });
wire_newtype!(z2VNKw, z2VbxJ);
wire_struct!(z2VUFf { field_0: z2VSH4 });
wire_enum!(z2VLx1 { unit {  } unary { 0 => z2VM4G(z2Vf91) : "UnixUser" } });
wire_newtype!(z2VT1o, z2VNRo);
wire_external_newtype!(z2VeFW, std::string::String);
wire_struct!(z2VLce { field_0: z2VZNB, field_1: z2VXQX, field_2: Option< std::string::String> });
wire_newtype!(z2VVd5, z2VLuJ);
wire_newtype!(z2VSH4, z2VPfN);
wire_struct!(z2VPkk { field_0: z2VVbN, field_1: z2VVYB });
wire_enum!(z2VXoV { unit {  } unary { 0 => z2VYBQ(z2VY1V) : "ChannelState", 1 => z2VNok(z2VMZv) : "MessageTrace", 2 => z2VPKn(z2VNxf) : "SessionData", 3 => z2VLzz(z2VSdJ) : "RoutedObjectsRefused", 4 => z2VcmP(z2VR5k) : "Summary", 5 => z2VV3E(z2Vbzo) : "ActorRegistrationRefused", 6 => z2VPBN(z2VT1o) : "ForwardRefused", 7 => z2VcdN(z2VNd2) : "MessageTraceMissing", 8 => z2Vc2V(z2VQWj) : "RoutedObjectsAccepted", 9 => z2VNhW(z2VPaP) : "SessionRefused", 10 => z2VeUs(z2VNK3) : "ActorRegistered", 11 => z2VSmt(z2VW7b) : "Unimplemented", 12 => z2VcTY(z2VYXM) : "SessionAccepted", 13 => z2VTty(z2VLKE) : "SessionServerHello", 14 => z2VTwc(z2VQGg) : "ForwardAccepted" } });
wire_newtype!(z2VVYB, z2VNMz);
wire_struct!(z2VNj2 { field_0: z2VQcL });
wire_external_newtype!(z2VVdV, u64);
wire_struct!(z2VPGB { field_0: z2VZVN });
wire_newtype!(z2VL7S, z2VWsQ);
wire_struct!(z2VR5k { field_0: z2VQcL, field_1: z2VST8, field_2: z2VcvY, field_3: z2VPky, field_4: z2Vc6c });
wire_external_newtype!(z2Vf91, u64);
wire_external_newtype!(z2VNwn, std::string::String);
wire_external_newtype!(z2Vc6c, u64);
wire_newtype!(z2Vd2q, z2VQGK);
wire_newtype!(z2VQ8d, z2VNMz);
wire_struct!(z2Vbzo { field_0: z2VSqw, field_1: z2VTf7 });
wire_enum!(z2VMPZ { unit { 0 => z2VUf6 : "Origin", 1 => z2VRbU : "Forwarded" } unary {  } });
wire_struct!(z2VLKE { field_0: z2VNKw, field_1: z2VW7S, field_2: z2VZXG });
wire_external_newtype!(z2Vd3E, std::string::String);
wire_external_newtype!(z2VZX5, std::string::String);
wire_newtype!(z2VSRk, z2VRts);
wire_external_newtype!(z2VVAk, std::string::String);
wire_newtype!(z2VPs7, z2VbxJ);

macro_rules! archive_root {
    ($root:ident) => {
        impl Archive for $root {
            type Archived = <WireValue as Archive>::Archived;
            type Resolver = <WireValue as Archive>::Resolver;
            fn resolve(&self, resolver: Self::Resolver, out: rkyv::Place<Self::Archived>) {
                self.to_wire().resolve(resolver, out);
            }
        }
        impl<Serializer> RkyvSerialize<Serializer> for $root
        where
            Serializer: rkyv::rancor::Fallible + ?Sized,
            WireValue: RkyvSerialize<Serializer>,
        {
            fn serialize(
                &self,
                serializer: &mut Serializer,
            ) -> Result<Self::Resolver, Serializer::Error> {
                self.to_wire().serialize(serializer)
            }
        }
        impl<Deserializer> RkyvDeserialize<$root, Deserializer> for ArchivedWireValue
        where
            Deserializer: rkyv::rancor::Fallible + ?Sized,
            Deserializer::Error: rkyv::rancor::Source,
            ArchivedWireValue: RkyvDeserialize<WireValue, Deserializer>,
        {
            fn deserialize(
                &self,
                deserializer: &mut Deserializer,
            ) -> Result<$root, Deserializer::Error> {
                let wire = <ArchivedWireValue as RkyvDeserialize<
                    WireValue,
                    Deserializer,
                >>::deserialize(self, deserializer)?;
                <$root as WireShape>::from_wire(wire).map_err(Deserializer::Error::new)
            }
        }
    };
}
archive_root!(z2VVbN);
archive_root!(z2VMR5);
archive_root!(z2VQHz);
archive_root!(z2VPfN);
archive_root!(z2VQGK);
archive_root!(z2VXQX);
archive_root!(z2VNRo);
archive_root!(z2VX9R);
archive_root!(z2VV5h);
archive_root!(z2VaLm);
archive_root!(z2VbKU);
archive_root!(z2VQNh);
archive_root!(z2VXG3);
archive_root!(z2VWUQ);
archive_root!(z2VNid);
archive_root!(z2VbUg);
archive_root!(z2VRcj);
archive_root!(z2VRts);
archive_root!(z2VfEY);
archive_root!(z2VZVo);
archive_root!(z2VZNB);
archive_root!(z2VST8);
archive_root!(z2Vdz8);
archive_root!(z2VQGg);
archive_root!(z2VYXM);
archive_root!(z2VY1V);
archive_root!(z2VcTK);
archive_root!(z2VW7S);
archive_root!(z2VPky);
archive_root!(z2VUhk);
archive_root!(z2VURC);
archive_root!(z2VbxJ);
archive_root!(z2VMfS);
archive_root!(z2VZU5);
archive_root!(z2VWsQ);
archive_root!(z2VZLC);
archive_root!(z2VSdJ);
archive_root!(z2VW7b);
archive_root!(z2VPaP);
archive_root!(z2VW1y);
archive_root!(z2VZMx);
archive_root!(z2VQWj);
archive_root!(z2Vcrd);
archive_root!(z2Vafp);
archive_root!(z2VXxh);
archive_root!(z2VLWZ);
archive_root!(z2Vd1b);
archive_root!(z2VcsM);
archive_root!(z2VNK3);
archive_root!(z2Vaf5);
archive_root!(z2VdEo);
archive_root!(z2VcsB);
archive_root!(z2VLFW);
archive_root!(z2VNd2);
archive_root!(z2VZfL);
archive_root!(z2VcpN);
archive_root!(z2VWLp);
archive_root!(z2VPdn);
archive_root!(z2VNh2);
archive_root!(z2VbSs);
archive_root!(z2VZGC);
archive_root!(z2VSqw);
archive_root!(z2VQcL);
archive_root!(z2VYdo);
archive_root!(z2Vf1e);
archive_root!(z2VZVN);
archive_root!(z2VVPx);
archive_root!(z2VLTy);
archive_root!(z2VPAH);
archive_root!(z2VLuJ);
archive_root!(z2VQew);
archive_root!(z2VNxf);
archive_root!(z2VaJt);
archive_root!(z2VbVV);
archive_root!(z2VVui);
archive_root!(z2VUTN);
archive_root!(z2VNMz);
archive_root!(z2VTFJ);
archive_root!(z2VMZv);
archive_root!(z2VZXG);
archive_root!(z2VVNQ);
archive_root!(z2VcvY);
archive_root!(z2VQC7);
archive_root!(z2Ve3n);
archive_root!(z2VPn3);
archive_root!(z2VYUB);
archive_root!(z2VWkj);
archive_root!(z2VbwY);
archive_root!(z2VSgE);
archive_root!(z2Va6n);
archive_root!(z2VTf7);
archive_root!(z2VNKw);
archive_root!(z2VUFf);
archive_root!(z2VLx1);
archive_root!(z2VT1o);
archive_root!(z2VeFW);
archive_root!(z2VLce);
archive_root!(z2VVd5);
archive_root!(z2VSH4);
archive_root!(z2VPkk);
archive_root!(z2VXoV);
archive_root!(z2VVYB);
archive_root!(z2VNj2);
archive_root!(z2VVdV);
archive_root!(z2VPGB);
archive_root!(z2VL7S);
archive_root!(z2VR5k);
archive_root!(z2Vf91);
archive_root!(z2VNwn);
archive_root!(z2Vc6c);
archive_root!(z2Vd2q);
archive_root!(z2VQ8d);
archive_root!(z2Vbzo);
archive_root!(z2VMPZ);
archive_root!(z2VLKE);
archive_root!(z2Vd3E);
archive_root!(z2VZX5);
archive_root!(z2VSRk);
archive_root!(z2VVAk);
archive_root!(z2VPs7);


pub enum ContractMarker {}

impl signal_frame::WireContract for ContractMarker {
    const BINDING: signal_frame::ContractBinding = signal_frame::ContractBinding::new(
        match signal_frame::ContractId::try_new(7) {
            Ok(value) => value,
            Err(_) => panic!("contract ID is allocated"),
        },
        match signal_frame::WireRevision::try_new(2) {
            Ok(value) => value,
            Err(_) => panic!("wire revision is allocated"),
        },
    );
}

#[derive(Archive, RkyvSerialize, RkyvDeserialize, Clone, Copy, Debug, PartialEq, Eq)]
pub enum EngineRefusalReason {
    Rejected,
    Unavailable,
}

#[derive(Archive, RkyvSerialize, RkyvDeserialize, Clone, Debug, PartialEq, Eq)]
pub struct EngineRefusal {
    pub reason: EngineRefusalReason,
    pub detail: std::string::String,
}

impl EngineRefusal {
    pub fn rejected(detail: std::string::String) -> Self {
        Self { reason: EngineRefusalReason::Rejected, detail }
    }

    pub fn unavailable(detail: std::string::String) -> Self {
        Self { reason: EngineRefusalReason::Unavailable, detail }
    }
}

#[derive(Clone, Debug, PartialEq, Eq, thiserror::Error)]
pub enum SignalFrameError {
    #[error("failed to encode bound signal frame")]
    FrameEncode,
    #[error("failed to decode bound signal frame")]
    ArchiveDecode,
    #[error("unexpected signal frame body")]
    UnexpectedFrameBody,
    #[error("expected one request operation, found {found}")]
    OperationCount { found: usize },
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum InputRoute {
    ChannelState,
    SubmitRoutedObjects,
    ForwardMessage,
    Summary,
    SessionClientProof,
    SessionClientHello,
    SessionData,
    RegisterActor,
    MessageTrace,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum OutputRoute {
    ChannelState,
    MessageTrace,
    SessionData,
    RoutedObjectsRefused,
    Summary,
    ActorRegistrationRefused,
    ForwardRefused,
    MessageTraceMissing,
    RoutedObjectsAccepted,
    SessionRefused,
    ActorRegistered,
    Unimplemented,
    SessionAccepted,
    SessionServerHello,
    ForwardAccepted,
}

impl z2VZGC {
    pub fn route(&self) -> InputRoute {
        match self {
            Self::z2VdPV(_) => InputRoute::ChannelState,
            Self::z2Vdxj(_) => InputRoute::SubmitRoutedObjects,
            Self::z2Vd1x(_) => InputRoute::ForwardMessage,
            Self::z2VMyr(_) => InputRoute::Summary,
            Self::z2VQMp(_) => InputRoute::SessionClientProof,
            Self::z2VMN6(_) => InputRoute::SessionClientHello,
            Self::z2Vd2e(_) => InputRoute::SessionData,
            Self::z2VWdr(_) => InputRoute::RegisterActor,
            Self::z2VWzG(_) => InputRoute::MessageTrace,
        }
    }

    pub fn wire_route(&self) -> signal_frame::WireRoute {
        signal_frame::WireRoute::new(
            signal_frame::RootCode::new(0),
            signal_frame::VariantCode::new(self.route() as u8),
        )
    }

    pub fn into_frame(self, exchange: signal_frame::ExchangeIdentifier) -> Frame {
        let route = self.wire_route();
        Frame::new(
            route,
            FrameBody::Request {
                exchange,
                request: signal_frame::Request::from_payload(self),
            },
        )
    }

    pub fn encode_request_frame(
        self,
        exchange: signal_frame::ExchangeIdentifier,
    ) -> Result<Vec<u8>, SignalFrameError> {
        self.into_frame(exchange)
            .encode()
            .map_err(|_| SignalFrameError::FrameEncode)
    }
}

impl z2VXoV {
    pub fn route(&self) -> OutputRoute {
        match self {
            Self::z2VYBQ(_) => OutputRoute::ChannelState,
            Self::z2VNok(_) => OutputRoute::MessageTrace,
            Self::z2VPKn(_) => OutputRoute::SessionData,
            Self::z2VLzz(_) => OutputRoute::RoutedObjectsRefused,
            Self::z2VcmP(_) => OutputRoute::Summary,
            Self::z2VV3E(_) => OutputRoute::ActorRegistrationRefused,
            Self::z2VPBN(_) => OutputRoute::ForwardRefused,
            Self::z2VcdN(_) => OutputRoute::MessageTraceMissing,
            Self::z2Vc2V(_) => OutputRoute::RoutedObjectsAccepted,
            Self::z2VNhW(_) => OutputRoute::SessionRefused,
            Self::z2VeUs(_) => OutputRoute::ActorRegistered,
            Self::z2VSmt(_) => OutputRoute::Unimplemented,
            Self::z2VcTY(_) => OutputRoute::SessionAccepted,
            Self::z2VTty(_) => OutputRoute::SessionServerHello,
            Self::z2VTwc(_) => OutputRoute::ForwardAccepted,
        }
    }

    pub fn wire_route(&self) -> signal_frame::WireRoute {
        signal_frame::WireRoute::new(
            signal_frame::RootCode::new(1),
            signal_frame::VariantCode::new(self.route() as u8),
        )
    }

    pub fn into_reply_frame(self, exchange: signal_frame::ExchangeIdentifier) -> Frame {
        let route = self.wire_route();
        let reply = signal_frame::Reply::committed(
            signal_frame::NonEmpty::single(signal_frame::SubReply::Ok(self)),
        );
        Frame::new(route, FrameBody::Reply { exchange, reply })
    }

    pub fn encode_reply_frame(
        self,
        exchange: signal_frame::ExchangeIdentifier,
    ) -> Result<Vec<u8>, SignalFrameError> {
        self.into_reply_frame(exchange)
            .encode()
            .map_err(|_| SignalFrameError::FrameEncode)
    }
}

impl signal_frame::RequestPayload for z2VZGC {}

impl signal_frame::SignalOperationHeads for z2VZGC {
    const HEADS: &'static [&'static str] = &["ChannelState", "SubmitRoutedObjects", "ForwardMessage", "Summary", "SessionClientProof", "SessionClientHello", "SessionData", "RegisterActor", "MessageTrace"];
}

impl signal_frame::LogVariant for z2VZGC {
    fn log_variant(&self) -> u64 {
        let route = self.wire_route();
        u64::from(route.root().value()) | (u64::from(route.variant().value()) << 8)
    }
}

pub type Frame = signal_frame::BoundExchangeFrame<ContractMarker, z2VZGC, z2VXoV>;
pub type FrameBody = signal_frame::ExchangeFrameBody<z2VZGC, z2VXoV>;
pub type Request = signal_frame::Request<z2VZGC>;
pub type ReplyEnvelope = signal_frame::Reply<z2VXoV>;
pub type RequestBuilder = signal_frame::RequestBuilder<z2VZGC>;

impl ContractMarker {
    pub fn decode_frame(bytes: &[u8]) -> Result<Frame, SignalFrameError> {
        Frame::decode(bytes).map_err(|_| SignalFrameError::ArchiveDecode)
    }

    pub fn decode_single_request(
        bytes: &[u8],
    ) -> Result<(signal_frame::ExchangeIdentifier, z2VZGC), SignalFrameError> {
        match Self::decode_frame(bytes)?.into_body() {
            FrameBody::Request { exchange, request } => {
                let found = request.payloads().len();
                if found != 1 {
                    return Err(SignalFrameError::OperationCount { found });
                }
                Ok((exchange, request.payloads.into_head()))
            }
            _ => Err(SignalFrameError::UnexpectedFrameBody),
        }
    }
}

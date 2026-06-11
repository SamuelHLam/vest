#![allow(unused_imports)]
#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(non_camel_case_types)]
use vest_lib::properties::*;
use vest_lib::errors::*;
use vest_lib::regular::*;
use vest_lib::regular::sequence::{Pair, Preceded, Terminated, DepCombinator};
use vest_lib::regular::variant::{Dispatch, Opt, OptThen, Choice};
use vest_lib::regular::repetition::{Repeat, RepeatN};
use vest_lib::regular::modifier::{
    Refined, Mapped, FixedLen, Length, RuntimeValue, AndThen, CondEq, Mapper,
};
use vest_lib::regular::tag::Tag;
use vest_lib::regular::bytes::{Fixed, Variable, Tail};
use vest_lib::regular::success::Success;
use vest_lib::regular::fail::Fail;
use vest_lib::regular::end::End;
use vest_lib::regular::uints::*;
use vest_lib::buf_traits::{VestInput, VestOutput};
#[repr(u64)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CertificateType {
    X509 = 0,
    RawPublicKey = 2,
    Unknown(u64),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ServerCertTypeServerExtension {
    pub server_certificate_type: CertificateType,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Opaque1Ff<'a> {
    pub l: u8,
    pub data: &'a [u8],
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Opaque1FfOwned {
    pub l: u8,
    pub data: Vec<u8>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Opaque1Ffff<'a> {
    pub l: u16,
    pub data: &'a [u8],
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Opaque1FfffOwned {
    pub l: u16,
    pub data: Vec<u8>,
}

#[repr(u64)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum NameType {
    HostName = 0,
    Unknown(u64),
}

pub type HostName<'a> = Opaque1Ffff<'a>;
pub type HostNameOwned = Opaque1FfffOwned;
pub type UnknownName<'a> = Opaque1Ffff<'a>;
pub type UnknownNameOwned = Opaque1FfffOwned;
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ServerNameName<'a> {
    HostName(HostName<'a>),
    __default(UnknownName<'a>),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ServerNameNameOwned {
    HostName(HostNameOwned),
    __default(UnknownNameOwned),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ServerName<'a> {
    pub name_type: NameType,
    pub name: ServerNameName<'a>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ServerNameOwned {
    pub name_type: NameType,
    pub name: ServerNameNameOwned,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ServerNameList<'a> {
    pub l: u16,
    pub list: Vec<ServerName<'a>>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ServerNameListOwned {
    pub l: u16,
    pub list: Vec<ServerNameOwned>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Opaque0Ffff<'a> {
    pub l: u16,
    pub data: &'a [u8],
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Opaque0FfffOwned {
    pub l: u16,
    pub data: Vec<u8>,
}

pub type OcspExtensions<'a> = Opaque0Ffff<'a>;
pub type OcspExtensionsOwned = Opaque0FfffOwned;
pub type ResponderId<'a> = Opaque1Ffff<'a>;
pub type ResponderIdOwned = Opaque1FfffOwned;
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ResponderIdList<'a> {
    pub l: u16,
    pub list: Vec<ResponderId<'a>>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ResponderIdListOwned {
    pub l: u16,
    pub list: Vec<ResponderIdOwned>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct OscpStatusRequest<'a> {
    pub responder_id_list: ResponderIdList<'a>,
    pub extensions: OcspExtensions<'a>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct OscpStatusRequestOwned {
    pub responder_id_list: ResponderIdListOwned,
    pub extensions: OcspExtensionsOwned,
}

pub type SrtpProtectionProfile<'a> = &'a [u8];
pub type SrtpProtectionProfileOwned = Vec<u8>;
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SrtpProtectionProfiles<'a> {
    pub l: u16,
    pub list: Vec<SrtpProtectionProfile<'a>>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SrtpProtectionProfilesOwned {
    pub l: u16,
    pub list: Vec<SrtpProtectionProfileOwned>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Opaque0Ff<'a> {
    pub l: u8,
    pub data: &'a [u8],
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Opaque0FfOwned {
    pub l: u8,
    pub data: Vec<u8>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct UseSrtpData<'a> {
    pub profiles: SrtpProtectionProfiles<'a>,
    pub srtp_mki: Opaque0Ff<'a>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct UseSrtpDataOwned {
    pub profiles: SrtpProtectionProfilesOwned,
    pub srtp_mki: Opaque0FfOwned,
}

#[repr(u64)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MaxFragmentLength {
    Pow2_9 = 1,
    Pow2_10 = 2,
    Pow2_11 = 3,
    Pow2_12 = 4,
    Unknown(u64),
}

#[repr(u64)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SignatureScheme {
    RSA_PKCS1_MD5 = 257,
    RSA_PKCS1_SHA1 = 513,
    ECDSA_MD5 = 259,
    ECDSA_SHA1 = 515,
    RSA_PKCS1_SHA256 = 1025,
    RSA_PKCS1_SHA384 = 1281,
    RSA_PKCS1_SHA512 = 1537,
    ECDSA_SECP256R1_SHA256 = 1027,
    ECDSA_SECP384R1_SHA384 = 1283,
    ECDSA_SECP521R1_SHA512 = 1539,
    RSA_PSS_RSAE_SHA256 = 2052,
    RSA_PSS_RSAE_SHA384 = 2053,
    RSA_PSS_RSAE_SHA512 = 2054,
    ED25519 = 2055,
    ED448 = 2056,
    RSA_PSS_PSS_SHA256 = 2057,
    RSA_PSS_PSS_SHA384 = 2058,
    RSA_PSS_PSS_SHA512 = 2059,
    Unknown(u64),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ZeroByte {
    pub zero: u8,
}

pub type SerializedSct<'a> = Opaque1Ffff<'a>;
pub type SerializedSctOwned = Opaque1FfffOwned;
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PaddingExtension {
    pub l: u16,
    pub padding: Vec<ZeroByte>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SignatureSchemeList {
    pub l: u16,
    pub list: Vec<SignatureScheme>,
}

#[repr(u64)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ExtensionType {
    ServerName = 0,
    MaxFragmentLength = 1,
    StatusRequest = 5,
    SupportedGroups = 10,
    ECPointFormats = 11,
    SignatureAlgorithms = 13,
    UseSRTP = 14,
    Heartbeat = 15,
    ApplicationLayerProtocolNegotiation = 16,
    SignedCertificateTimeStamp = 18,
    ClientCertificateType = 19,
    ServerCertificateType = 20,
    Padding = 21,
    EncryptThenMac = 22,
    ExtendedMasterSecret = 23,
    SessionTicket = 35,
    PreSharedKey = 41,
    EarlyData = 42,
    SupportedVersions = 43,
    Cookie = 44,
    PskKeyExchangeModes = 45,
    CertificateAuthorities = 47,
    OidFilters = 48,
    PostHandshakeAuth = 49,
    SignatureAlgorithmsCert = 50,
    KeyShare = 51,
    Dummy = 65535,
    Unknown(u64),
}

pub type ProtocolName<'a> = Opaque1Ff<'a>;
pub type ProtocolNameOwned = Opaque1FfOwned;
#[repr(u64)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ProtocolVersion {
    SSLv3 = 768,
    TLSv1_0 = 769,
    TLSv1_1 = 770,
    TLSv1_2 = 771,
    TLSv1_3 = 772,
    Unknown(u64),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ServerCertTypeClientExtension {
    pub l: u8,
    pub list: Vec<CertificateType>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SignedCertificateTimestampList<'a> {
    pub l: u16,
    pub list: Vec<SerializedSct<'a>>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SignedCertificateTimestampListOwned {
    pub l: u16,
    pub list: Vec<SerializedSctOwned>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ClientCertTypeServerExtension {
    pub client_certificate_type: CertificateType,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Opaque2Ffff<'a> {
    pub l: u16,
    pub data: &'a [u8],
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Opaque2FfffOwned {
    pub l: u16,
    pub data: Vec<u8>,
}

#[repr(u64)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum NamedGroup {
    Sect163k1 = 1,
    Sect163r1 = 2,
    Sect163r2 = 3,
    Sect193r1 = 4,
    Sect193r2 = 5,
    Sect233k1 = 6,
    Sect233r1 = 7,
    Sect239k1 = 8,
    Sect283k1 = 9,
    Sect283r1 = 10,
    Sect409k1 = 11,
    Sect409r1 = 12,
    Sect571k1 = 13,
    Sect571r1 = 14,
    Secp160k1 = 15,
    Secp160r1 = 16,
    Secp160r2 = 17,
    Secp192k1 = 18,
    Secp192r1 = 19,
    Secp224k1 = 20,
    Secp224r1 = 21,
    Secp256k1 = 22,
    Secp256r1 = 23,
    Secp384r1 = 24,
    Secp521r1 = 25,
    X25519 = 29,
    X448 = 30,
    Ffdhe2048 = 256,
    Ffdhe3072 = 257,
    Ffdhe4096 = 258,
    Ffdhe6144 = 259,
    Ffdhe8192 = 260,
    Unknown(u64),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct NamedGroupList {
    pub l: u16,
    pub list: Vec<NamedGroup>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Opaque0Ffffff<'a> {
    pub l: u24,
    pub data: &'a [u8],
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Opaque0FfffffOwned {
    pub l: u24,
    pub data: Vec<u8>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ClientCertTypeClientExtension {
    pub l: u8,
    pub list: Vec<CertificateType>,
}

#[repr(u64)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum HeartbeatMode {
    PeerAllowedToSend = 1,
    PeerNotAllowedToSend = 2,
    Unknown(u64),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct HeartbeatExtension {
    pub mode: HeartbeatMode,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Opaque1Ffffff<'a> {
    pub l: u24,
    pub data: &'a [u8],
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Opaque1FfffffOwned {
    pub l: u24,
    pub data: Vec<u8>,
}

pub type OcspResponse<'a> = Opaque1Ffffff<'a>;
pub type OcspResponseOwned = Opaque1FfffffOwned;
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CertificateStatus<'a> {
    pub status_type: u8,
    pub response: OcspResponse<'a>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CertificateStatusOwned {
    pub status_type: u8,
    pub response: OcspResponseOwned,
}

#[repr(u64)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum EcPointFormat {
    Uncompressed = 0,
    AnsiX962CompressedPrime = 1,
    AnsiX962CompressedChar2 = 2,
    Unknown(u64),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CertificateStatusRequest<'a> {
    pub status_type: u8,
    pub request: OscpStatusRequest<'a>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CertificateStatusRequestOwned {
    pub status_type: u8,
    pub request: OscpStatusRequestOwned,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct EcPointFormatList {
    pub l: u8,
    pub list: Vec<EcPointFormat>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ProtocolNameList<'a> {
    pub l: u16,
    pub list: Vec<ProtocolName<'a>>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ProtocolNameListOwned {
    pub l: u16,
    pub list: Vec<ProtocolNameOwned>,
}

impl From<u8> for CertificateType {
    fn from(src: u8) -> Self {
        match src as i128 {
            0 => Self::X509,
            2 => Self::RawPublicKey,
            _ => unreachable!("validated by combinator"),
        }
    }
}

impl From<CertificateType> for u8 {
    fn from(v: CertificateType) -> Self {
        u8::from(v)
    }
}

pub struct CertificateTypeMapper;
impl Mapper for CertificateTypeMapper {
    type Src<'p> = u8;
    type Dst<'p> = CertificateType;
    type SrcBorrow<'s> = u8;
    type DstBorrow<'s> = CertificateType;
    type SrcOwned = u8;
    type DstOwned = CertificateType;
}

impl From<CertificateType> for ServerCertTypeServerExtension {
    fn from(src: CertificateType) -> Self {
        Self {
            server_certificate_type: src,
        }
    }
}

impl<'s> From<ServerCertTypeServerExtension> for CertificateType {
    fn from(v: ServerCertTypeServerExtension) -> Self {
        v.server_certificate_type
    }
}

pub struct ServerCertTypeServerExtensionMapper;
impl Mapper for ServerCertTypeServerExtensionMapper {
    type Src<'p> = CertificateType;
    type Dst<'p> = ServerCertTypeServerExtension;
    type SrcBorrow<'s> = CertificateType;
    type DstBorrow<'s> = ServerCertTypeServerExtension;
    type SrcOwned = CertificateType;
    type DstOwned = ServerCertTypeServerExtension;
}

impl<'a> From<(u8, &'a [u8])> for Opaque1Ff<'a> {
    fn from(src: (u8, &'a [u8])) -> Self {
        Self { l: src.0, data: src.1 }
    }
}

impl<'s, 'a: 's> From<&'s Opaque1Ff<'a>> for (u8, &'s [u8]) {
    fn from(v: &'s Opaque1Ff<'a>) -> Self {
        (v.l, v.data)
    }
}

impl From<(u8, Vec<u8>)> for Opaque1FfOwned {
    fn from(src: (u8, Vec<u8>)) -> Self {
        Self { l: src.0, data: src.1 }
    }
}

impl From<Opaque1FfOwned> for (u8, Vec<u8>) {
    fn from(v: Opaque1FfOwned) -> Self {
        (v.l, v.data)
    }
}

pub struct Opaque1FfMapper;
impl Mapper for Opaque1FfMapper {
    type Src<'p> = (u8, &'p [u8]);
    type Dst<'p> = Opaque1Ff<'p>;
    type SrcBorrow<'s> = (u8, &'s [u8]);
    type DstBorrow<'s> = &'s Opaque1Ff<'s>;
    type SrcOwned = (u8, Vec<u8>);
    type DstOwned = Opaque1FfOwned;
}

impl<'a> From<(u16, &'a [u8])> for Opaque1Ffff<'a> {
    fn from(src: (u16, &'a [u8])) -> Self {
        Self { l: src.0, data: src.1 }
    }
}

impl<'s, 'a: 's> From<&'s Opaque1Ffff<'a>> for (u16, &'s [u8]) {
    fn from(v: &'s Opaque1Ffff<'a>) -> Self {
        (v.l, v.data)
    }
}

impl From<(u16, Vec<u8>)> for Opaque1FfffOwned {
    fn from(src: (u16, Vec<u8>)) -> Self {
        Self { l: src.0, data: src.1 }
    }
}

impl From<Opaque1FfffOwned> for (u16, Vec<u8>) {
    fn from(v: Opaque1FfffOwned) -> Self {
        (v.l, v.data)
    }
}

pub struct Opaque1FfffMapper;
impl Mapper for Opaque1FfffMapper {
    type Src<'p> = (u16, &'p [u8]);
    type Dst<'p> = Opaque1Ffff<'p>;
    type SrcBorrow<'s> = (u16, &'s [u8]);
    type DstBorrow<'s> = &'s Opaque1Ffff<'s>;
    type SrcOwned = (u16, Vec<u8>);
    type DstOwned = Opaque1FfffOwned;
}

impl From<u8> for NameType {
    fn from(src: u8) -> Self {
        match src as i128 {
            0 => Self::HostName,
            _ => unreachable!("validated by combinator"),
        }
    }
}

impl From<NameType> for u8 {
    fn from(v: NameType) -> Self {
        u8::from(v)
    }
}

pub struct NameTypeMapper;
impl Mapper for NameTypeMapper {
    type Src<'p> = u8;
    type Dst<'p> = NameType;
    type SrcBorrow<'s> = u8;
    type DstBorrow<'s> = NameType;
    type SrcOwned = u8;
    type DstOwned = NameType;
}

impl<'a> From<(NameType, ServerNameName<'a>)> for ServerName<'a> {
    fn from(src: (NameType, ServerNameName<'a>)) -> Self {
        Self {
            name_type: src.0,
            name: src.1,
        }
    }
}

impl<'s, 'a: 's> From<&'s ServerName<'a>> for (NameType, &'s ServerNameName<'s>) {
    fn from(v: &'s ServerName<'a>) -> Self {
        (v.name_type, &v.name)
    }
}

impl From<(NameType, ServerNameNameOwned)> for ServerNameOwned {
    fn from(src: (NameType, ServerNameNameOwned)) -> Self {
        Self {
            name_type: src.0,
            name: src.1,
        }
    }
}

impl From<ServerNameOwned> for (NameType, ServerNameNameOwned) {
    fn from(v: ServerNameOwned) -> Self {
        (v.name_type, v.name)
    }
}

pub struct ServerNameMapper;
impl Mapper for ServerNameMapper {
    type Src<'p> = (NameType, ServerNameName<'p>);
    type Dst<'p> = ServerName<'p>;
    type SrcBorrow<'s> = (NameType, &'s ServerNameName<'s>);
    type DstBorrow<'s> = &'s ServerName<'s>;
    type SrcOwned = (NameType, ServerNameNameOwned);
    type DstOwned = ServerNameOwned;
}

impl<'a> From<(u16, Vec<ServerName<'a>>)> for ServerNameList<'a> {
    fn from(src: (u16, Vec<ServerName<'a>>)) -> Self {
        Self { l: src.0, list: src.1 }
    }
}

impl<'s, 'a: 's> From<&'s ServerNameList<'a>> for (u16, &'s [&'s ServerName<'s>]) {
    fn from(v: &'s ServerNameList<'a>) -> Self {
        (v.l, v.list.as_slice())
    }
}

impl From<(u16, Vec<ServerNameOwned>)> for ServerNameListOwned {
    fn from(src: (u16, Vec<ServerNameOwned>)) -> Self {
        Self { l: src.0, list: src.1 }
    }
}

impl From<ServerNameListOwned> for (u16, Vec<ServerNameOwned>) {
    fn from(v: ServerNameListOwned) -> Self {
        (v.l, v.list)
    }
}

pub struct ServerNameListMapper;
impl Mapper for ServerNameListMapper {
    type Src<'p> = (u16, Vec<ServerName<'p>>);
    type Dst<'p> = ServerNameList<'p>;
    type SrcBorrow<'s> = (u16, &'s [&'s ServerName<'s>]);
    type DstBorrow<'s> = &'s ServerNameList<'s>;
    type SrcOwned = (u16, Vec<ServerNameOwned>);
    type DstOwned = ServerNameListOwned;
}

impl<'a> From<(u16, &'a [u8])> for Opaque0Ffff<'a> {
    fn from(src: (u16, &'a [u8])) -> Self {
        Self { l: src.0, data: src.1 }
    }
}

impl<'s, 'a: 's> From<&'s Opaque0Ffff<'a>> for (u16, &'s [u8]) {
    fn from(v: &'s Opaque0Ffff<'a>) -> Self {
        (v.l, v.data)
    }
}

impl From<(u16, Vec<u8>)> for Opaque0FfffOwned {
    fn from(src: (u16, Vec<u8>)) -> Self {
        Self { l: src.0, data: src.1 }
    }
}

impl From<Opaque0FfffOwned> for (u16, Vec<u8>) {
    fn from(v: Opaque0FfffOwned) -> Self {
        (v.l, v.data)
    }
}

pub struct Opaque0FfffMapper;
impl Mapper for Opaque0FfffMapper {
    type Src<'p> = (u16, &'p [u8]);
    type Dst<'p> = Opaque0Ffff<'p>;
    type SrcBorrow<'s> = (u16, &'s [u8]);
    type DstBorrow<'s> = &'s Opaque0Ffff<'s>;
    type SrcOwned = (u16, Vec<u8>);
    type DstOwned = Opaque0FfffOwned;
}

impl<'a> From<(u16, Vec<ResponderId<'a>>)> for ResponderIdList<'a> {
    fn from(src: (u16, Vec<ResponderId<'a>>)) -> Self {
        Self { l: src.0, list: src.1 }
    }
}

impl<'s, 'a: 's> From<&'s ResponderIdList<'a>> for (u16, &'s [&'s ResponderId<'s>]) {
    fn from(v: &'s ResponderIdList<'a>) -> Self {
        (v.l, v.list.as_slice())
    }
}

impl From<(u16, Vec<ResponderIdOwned>)> for ResponderIdListOwned {
    fn from(src: (u16, Vec<ResponderIdOwned>)) -> Self {
        Self { l: src.0, list: src.1 }
    }
}

impl From<ResponderIdListOwned> for (u16, Vec<ResponderIdOwned>) {
    fn from(v: ResponderIdListOwned) -> Self {
        (v.l, v.list)
    }
}

pub struct ResponderIdListMapper;
impl Mapper for ResponderIdListMapper {
    type Src<'p> = (u16, Vec<ResponderId<'p>>);
    type Dst<'p> = ResponderIdList<'p>;
    type SrcBorrow<'s> = (u16, &'s [&'s ResponderId<'s>]);
    type DstBorrow<'s> = &'s ResponderIdList<'s>;
    type SrcOwned = (u16, Vec<ResponderIdOwned>);
    type DstOwned = ResponderIdListOwned;
}

impl<'a> From<(ResponderIdList<'a>, OcspExtensions<'a>)> for OscpStatusRequest<'a> {
    fn from(src: (ResponderIdList<'a>, OcspExtensions<'a>)) -> Self {
        Self {
            responder_id_list: src.0,
            extensions: src.1,
        }
    }
}

impl<'s, 'a: 's> From<&'s OscpStatusRequest<'a>>
for (&'s ResponderIdList<'s>, &'s OcspExtensions<'s>) {
    fn from(v: &'s OscpStatusRequest<'a>) -> Self {
        (&v.responder_id_list, &v.extensions)
    }
}

impl From<(ResponderIdListOwned, OcspExtensionsOwned)> for OscpStatusRequestOwned {
    fn from(src: (ResponderIdListOwned, OcspExtensionsOwned)) -> Self {
        Self {
            responder_id_list: src.0,
            extensions: src.1,
        }
    }
}

impl From<OscpStatusRequestOwned> for (ResponderIdListOwned, OcspExtensionsOwned) {
    fn from(v: OscpStatusRequestOwned) -> Self {
        (v.responder_id_list, v.extensions)
    }
}

pub struct OscpStatusRequestMapper;
impl Mapper for OscpStatusRequestMapper {
    type Src<'p> = (ResponderIdList<'p>, OcspExtensions<'p>);
    type Dst<'p> = OscpStatusRequest<'p>;
    type SrcBorrow<'s> = (&'s ResponderIdList<'s>, &'s OcspExtensions<'s>);
    type DstBorrow<'s> = &'s OscpStatusRequest<'s>;
    type SrcOwned = (ResponderIdListOwned, OcspExtensionsOwned);
    type DstOwned = OscpStatusRequestOwned;
}

impl<'a> From<(u16, Vec<SrtpProtectionProfile<'a>>)> for SrtpProtectionProfiles<'a> {
    fn from(src: (u16, Vec<SrtpProtectionProfile<'a>>)) -> Self {
        Self { l: src.0, list: src.1 }
    }
}

impl<'s, 'a: 's> From<&'s SrtpProtectionProfiles<'a>>
for (u16, &'s [SrtpProtectionProfile<'s>]) {
    fn from(v: &'s SrtpProtectionProfiles<'a>) -> Self {
        (v.l, v.list.as_slice())
    }
}

impl From<(u16, Vec<SrtpProtectionProfileOwned>)> for SrtpProtectionProfilesOwned {
    fn from(src: (u16, Vec<SrtpProtectionProfileOwned>)) -> Self {
        Self { l: src.0, list: src.1 }
    }
}

impl From<SrtpProtectionProfilesOwned> for (u16, Vec<SrtpProtectionProfileOwned>) {
    fn from(v: SrtpProtectionProfilesOwned) -> Self {
        (v.l, v.list)
    }
}

pub struct SrtpProtectionProfilesMapper;
impl Mapper for SrtpProtectionProfilesMapper {
    type Src<'p> = (u16, Vec<SrtpProtectionProfile<'p>>);
    type Dst<'p> = SrtpProtectionProfiles<'p>;
    type SrcBorrow<'s> = (u16, &'s [SrtpProtectionProfile<'s>]);
    type DstBorrow<'s> = &'s SrtpProtectionProfiles<'s>;
    type SrcOwned = (u16, Vec<SrtpProtectionProfileOwned>);
    type DstOwned = SrtpProtectionProfilesOwned;
}

impl<'a> From<(u8, &'a [u8])> for Opaque0Ff<'a> {
    fn from(src: (u8, &'a [u8])) -> Self {
        Self { l: src.0, data: src.1 }
    }
}

impl<'s, 'a: 's> From<&'s Opaque0Ff<'a>> for (u8, &'s [u8]) {
    fn from(v: &'s Opaque0Ff<'a>) -> Self {
        (v.l, v.data)
    }
}

impl From<(u8, Vec<u8>)> for Opaque0FfOwned {
    fn from(src: (u8, Vec<u8>)) -> Self {
        Self { l: src.0, data: src.1 }
    }
}

impl From<Opaque0FfOwned> for (u8, Vec<u8>) {
    fn from(v: Opaque0FfOwned) -> Self {
        (v.l, v.data)
    }
}

pub struct Opaque0FfMapper;
impl Mapper for Opaque0FfMapper {
    type Src<'p> = (u8, &'p [u8]);
    type Dst<'p> = Opaque0Ff<'p>;
    type SrcBorrow<'s> = (u8, &'s [u8]);
    type DstBorrow<'s> = &'s Opaque0Ff<'s>;
    type SrcOwned = (u8, Vec<u8>);
    type DstOwned = Opaque0FfOwned;
}

impl<'a> From<(SrtpProtectionProfiles<'a>, Opaque0Ff<'a>)> for UseSrtpData<'a> {
    fn from(src: (SrtpProtectionProfiles<'a>, Opaque0Ff<'a>)) -> Self {
        Self {
            profiles: src.0,
            srtp_mki: src.1,
        }
    }
}

impl<'s, 'a: 's> From<&'s UseSrtpData<'a>>
for (&'s SrtpProtectionProfiles<'s>, &'s Opaque0Ff<'s>) {
    fn from(v: &'s UseSrtpData<'a>) -> Self {
        (&v.profiles, &v.srtp_mki)
    }
}

impl From<(SrtpProtectionProfilesOwned, Opaque0FfOwned)> for UseSrtpDataOwned {
    fn from(src: (SrtpProtectionProfilesOwned, Opaque0FfOwned)) -> Self {
        Self {
            profiles: src.0,
            srtp_mki: src.1,
        }
    }
}

impl From<UseSrtpDataOwned> for (SrtpProtectionProfilesOwned, Opaque0FfOwned) {
    fn from(v: UseSrtpDataOwned) -> Self {
        (v.profiles, v.srtp_mki)
    }
}

pub struct UseSrtpDataMapper;
impl Mapper for UseSrtpDataMapper {
    type Src<'p> = (SrtpProtectionProfiles<'p>, Opaque0Ff<'p>);
    type Dst<'p> = UseSrtpData<'p>;
    type SrcBorrow<'s> = (&'s SrtpProtectionProfiles<'s>, &'s Opaque0Ff<'s>);
    type DstBorrow<'s> = &'s UseSrtpData<'s>;
    type SrcOwned = (SrtpProtectionProfilesOwned, Opaque0FfOwned);
    type DstOwned = UseSrtpDataOwned;
}

impl From<u8> for MaxFragmentLength {
    fn from(src: u8) -> Self {
        match src as i128 {
            1 => Self::Pow2_9,
            2 => Self::Pow2_10,
            3 => Self::Pow2_11,
            4 => Self::Pow2_12,
            _ => unreachable!("validated by combinator"),
        }
    }
}

impl From<MaxFragmentLength> for u8 {
    fn from(v: MaxFragmentLength) -> Self {
        u8::from(v)
    }
}

pub struct MaxFragmentLengthMapper;
impl Mapper for MaxFragmentLengthMapper {
    type Src<'p> = u8;
    type Dst<'p> = MaxFragmentLength;
    type SrcBorrow<'s> = u8;
    type DstBorrow<'s> = MaxFragmentLength;
    type SrcOwned = u8;
    type DstOwned = MaxFragmentLength;
}

impl From<u16> for SignatureScheme {
    fn from(src: u16) -> Self {
        match src as i128 {
            257 => Self::RSA_PKCS1_MD5,
            513 => Self::RSA_PKCS1_SHA1,
            259 => Self::ECDSA_MD5,
            515 => Self::ECDSA_SHA1,
            1025 => Self::RSA_PKCS1_SHA256,
            1281 => Self::RSA_PKCS1_SHA384,
            1537 => Self::RSA_PKCS1_SHA512,
            1027 => Self::ECDSA_SECP256R1_SHA256,
            1283 => Self::ECDSA_SECP384R1_SHA384,
            1539 => Self::ECDSA_SECP521R1_SHA512,
            2052 => Self::RSA_PSS_RSAE_SHA256,
            2053 => Self::RSA_PSS_RSAE_SHA384,
            2054 => Self::RSA_PSS_RSAE_SHA512,
            2055 => Self::ED25519,
            2056 => Self::ED448,
            2057 => Self::RSA_PSS_PSS_SHA256,
            2058 => Self::RSA_PSS_PSS_SHA384,
            2059 => Self::RSA_PSS_PSS_SHA512,
            _ => unreachable!("validated by combinator"),
        }
    }
}

impl From<SignatureScheme> for u16 {
    fn from(v: SignatureScheme) -> Self {
        u16::from(v)
    }
}

pub struct SignatureSchemeMapper;
impl Mapper for SignatureSchemeMapper {
    type Src<'p> = u16;
    type Dst<'p> = SignatureScheme;
    type SrcBorrow<'s> = u16;
    type DstBorrow<'s> = SignatureScheme;
    type SrcOwned = u16;
    type DstOwned = SignatureScheme;
}

impl From<()> for ZeroByte {
    fn from(src: ()) -> Self {
        Self { zero: ZERO_BYTEZERO_CONST }
    }
}

impl<'s> From<ZeroByte> for () {
    fn from(v: ZeroByte) -> Self {
        ()
    }
}

pub struct ZeroByteMapper;
impl Mapper for ZeroByteMapper {
    type Src<'p> = ();
    type Dst<'p> = ZeroByte;
    type SrcBorrow<'s> = ();
    type DstBorrow<'s> = ZeroByte;
    type SrcOwned = ();
    type DstOwned = ZeroByte;
}

impl From<(u16, Vec<ZeroByte>)> for PaddingExtension {
    fn from(src: (u16, Vec<ZeroByte>)) -> Self {
        Self { l: src.0, padding: src.1 }
    }
}

impl<'s> From<&'s PaddingExtension> for (u16, &'s [ZeroByte]) {
    fn from(v: &'s PaddingExtension) -> Self {
        (v.l, v.padding.as_slice())
    }
}

impl<'a> From<PaddingExtension> for (u16, Vec<ZeroByte>) {
    fn from(v: PaddingExtension) -> Self {
        (v.l, v.padding)
    }
}

pub struct PaddingExtensionMapper;
impl Mapper for PaddingExtensionMapper {
    type Src<'p> = (u16, Vec<ZeroByte>);
    type Dst<'p> = PaddingExtension;
    type SrcBorrow<'s> = (u16, &'s [ZeroByte]);
    type DstBorrow<'s> = &'s PaddingExtension;
    type SrcOwned = (u16, Vec<ZeroByte>);
    type DstOwned = PaddingExtension;
}

impl From<(u16, Vec<SignatureScheme>)> for SignatureSchemeList {
    fn from(src: (u16, Vec<SignatureScheme>)) -> Self {
        Self { l: src.0, list: src.1 }
    }
}

impl<'s> From<&'s SignatureSchemeList> for (u16, &'s [SignatureScheme]) {
    fn from(v: &'s SignatureSchemeList) -> Self {
        (v.l, v.list.as_slice())
    }
}

impl<'a> From<SignatureSchemeList> for (u16, Vec<SignatureScheme>) {
    fn from(v: SignatureSchemeList) -> Self {
        (v.l, v.list)
    }
}

pub struct SignatureSchemeListMapper;
impl Mapper for SignatureSchemeListMapper {
    type Src<'p> = (u16, Vec<SignatureScheme>);
    type Dst<'p> = SignatureSchemeList;
    type SrcBorrow<'s> = (u16, &'s [SignatureScheme]);
    type DstBorrow<'s> = &'s SignatureSchemeList;
    type SrcOwned = (u16, Vec<SignatureScheme>);
    type DstOwned = SignatureSchemeList;
}

impl From<u16> for ExtensionType {
    fn from(src: u16) -> Self {
        match src as i128 {
            0 => Self::ServerName,
            1 => Self::MaxFragmentLength,
            5 => Self::StatusRequest,
            10 => Self::SupportedGroups,
            11 => Self::ECPointFormats,
            13 => Self::SignatureAlgorithms,
            14 => Self::UseSRTP,
            15 => Self::Heartbeat,
            16 => Self::ApplicationLayerProtocolNegotiation,
            18 => Self::SignedCertificateTimeStamp,
            19 => Self::ClientCertificateType,
            20 => Self::ServerCertificateType,
            21 => Self::Padding,
            22 => Self::EncryptThenMac,
            23 => Self::ExtendedMasterSecret,
            35 => Self::SessionTicket,
            41 => Self::PreSharedKey,
            42 => Self::EarlyData,
            43 => Self::SupportedVersions,
            44 => Self::Cookie,
            45 => Self::PskKeyExchangeModes,
            47 => Self::CertificateAuthorities,
            48 => Self::OidFilters,
            49 => Self::PostHandshakeAuth,
            50 => Self::SignatureAlgorithmsCert,
            51 => Self::KeyShare,
            65535 => Self::Dummy,
            _ => unreachable!("validated by combinator"),
        }
    }
}

impl From<ExtensionType> for u16 {
    fn from(v: ExtensionType) -> Self {
        u16::from(v)
    }
}

pub struct ExtensionTypeMapper;
impl Mapper for ExtensionTypeMapper {
    type Src<'p> = u16;
    type Dst<'p> = ExtensionType;
    type SrcBorrow<'s> = u16;
    type DstBorrow<'s> = ExtensionType;
    type SrcOwned = u16;
    type DstOwned = ExtensionType;
}

impl From<u16> for ProtocolVersion {
    fn from(src: u16) -> Self {
        match src as i128 {
            768 => Self::SSLv3,
            769 => Self::TLSv1_0,
            770 => Self::TLSv1_1,
            771 => Self::TLSv1_2,
            772 => Self::TLSv1_3,
            _ => unreachable!("validated by combinator"),
        }
    }
}

impl From<ProtocolVersion> for u16 {
    fn from(v: ProtocolVersion) -> Self {
        u16::from(v)
    }
}

pub struct ProtocolVersionMapper;
impl Mapper for ProtocolVersionMapper {
    type Src<'p> = u16;
    type Dst<'p> = ProtocolVersion;
    type SrcBorrow<'s> = u16;
    type DstBorrow<'s> = ProtocolVersion;
    type SrcOwned = u16;
    type DstOwned = ProtocolVersion;
}

impl From<(u8, Vec<CertificateType>)> for ServerCertTypeClientExtension {
    fn from(src: (u8, Vec<CertificateType>)) -> Self {
        Self { l: src.0, list: src.1 }
    }
}

impl<'s> From<&'s ServerCertTypeClientExtension> for (u8, &'s [CertificateType]) {
    fn from(v: &'s ServerCertTypeClientExtension) -> Self {
        (v.l, v.list.as_slice())
    }
}

impl<'a> From<ServerCertTypeClientExtension> for (u8, Vec<CertificateType>) {
    fn from(v: ServerCertTypeClientExtension) -> Self {
        (v.l, v.list)
    }
}

pub struct ServerCertTypeClientExtensionMapper;
impl Mapper for ServerCertTypeClientExtensionMapper {
    type Src<'p> = (u8, Vec<CertificateType>);
    type Dst<'p> = ServerCertTypeClientExtension;
    type SrcBorrow<'s> = (u8, &'s [CertificateType]);
    type DstBorrow<'s> = &'s ServerCertTypeClientExtension;
    type SrcOwned = (u8, Vec<CertificateType>);
    type DstOwned = ServerCertTypeClientExtension;
}

impl<'a> From<(u16, Vec<SerializedSct<'a>>)> for SignedCertificateTimestampList<'a> {
    fn from(src: (u16, Vec<SerializedSct<'a>>)) -> Self {
        Self { l: src.0, list: src.1 }
    }
}

impl<'s, 'a: 's> From<&'s SignedCertificateTimestampList<'a>>
for (u16, &'s [&'s SerializedSct<'s>]) {
    fn from(v: &'s SignedCertificateTimestampList<'a>) -> Self {
        (v.l, v.list.as_slice())
    }
}

impl From<(u16, Vec<SerializedSctOwned>)> for SignedCertificateTimestampListOwned {
    fn from(src: (u16, Vec<SerializedSctOwned>)) -> Self {
        Self { l: src.0, list: src.1 }
    }
}

impl From<SignedCertificateTimestampListOwned> for (u16, Vec<SerializedSctOwned>) {
    fn from(v: SignedCertificateTimestampListOwned) -> Self {
        (v.l, v.list)
    }
}

pub struct SignedCertificateTimestampListMapper;
impl Mapper for SignedCertificateTimestampListMapper {
    type Src<'p> = (u16, Vec<SerializedSct<'p>>);
    type Dst<'p> = SignedCertificateTimestampList<'p>;
    type SrcBorrow<'s> = (u16, &'s [&'s SerializedSct<'s>]);
    type DstBorrow<'s> = &'s SignedCertificateTimestampList<'s>;
    type SrcOwned = (u16, Vec<SerializedSctOwned>);
    type DstOwned = SignedCertificateTimestampListOwned;
}

impl From<CertificateType> for ClientCertTypeServerExtension {
    fn from(src: CertificateType) -> Self {
        Self {
            client_certificate_type: src,
        }
    }
}

impl<'s> From<ClientCertTypeServerExtension> for CertificateType {
    fn from(v: ClientCertTypeServerExtension) -> Self {
        v.client_certificate_type
    }
}

pub struct ClientCertTypeServerExtensionMapper;
impl Mapper for ClientCertTypeServerExtensionMapper {
    type Src<'p> = CertificateType;
    type Dst<'p> = ClientCertTypeServerExtension;
    type SrcBorrow<'s> = CertificateType;
    type DstBorrow<'s> = ClientCertTypeServerExtension;
    type SrcOwned = CertificateType;
    type DstOwned = ClientCertTypeServerExtension;
}

impl<'a> From<(u16, &'a [u8])> for Opaque2Ffff<'a> {
    fn from(src: (u16, &'a [u8])) -> Self {
        Self { l: src.0, data: src.1 }
    }
}

impl<'s, 'a: 's> From<&'s Opaque2Ffff<'a>> for (u16, &'s [u8]) {
    fn from(v: &'s Opaque2Ffff<'a>) -> Self {
        (v.l, v.data)
    }
}

impl From<(u16, Vec<u8>)> for Opaque2FfffOwned {
    fn from(src: (u16, Vec<u8>)) -> Self {
        Self { l: src.0, data: src.1 }
    }
}

impl From<Opaque2FfffOwned> for (u16, Vec<u8>) {
    fn from(v: Opaque2FfffOwned) -> Self {
        (v.l, v.data)
    }
}

pub struct Opaque2FfffMapper;
impl Mapper for Opaque2FfffMapper {
    type Src<'p> = (u16, &'p [u8]);
    type Dst<'p> = Opaque2Ffff<'p>;
    type SrcBorrow<'s> = (u16, &'s [u8]);
    type DstBorrow<'s> = &'s Opaque2Ffff<'s>;
    type SrcOwned = (u16, Vec<u8>);
    type DstOwned = Opaque2FfffOwned;
}

impl From<u16> for NamedGroup {
    fn from(src: u16) -> Self {
        match src as i128 {
            1 => Self::Sect163k1,
            2 => Self::Sect163r1,
            3 => Self::Sect163r2,
            4 => Self::Sect193r1,
            5 => Self::Sect193r2,
            6 => Self::Sect233k1,
            7 => Self::Sect233r1,
            8 => Self::Sect239k1,
            9 => Self::Sect283k1,
            10 => Self::Sect283r1,
            11 => Self::Sect409k1,
            12 => Self::Sect409r1,
            13 => Self::Sect571k1,
            14 => Self::Sect571r1,
            15 => Self::Secp160k1,
            16 => Self::Secp160r1,
            17 => Self::Secp160r2,
            18 => Self::Secp192k1,
            19 => Self::Secp192r1,
            20 => Self::Secp224k1,
            21 => Self::Secp224r1,
            22 => Self::Secp256k1,
            23 => Self::Secp256r1,
            24 => Self::Secp384r1,
            25 => Self::Secp521r1,
            29 => Self::X25519,
            30 => Self::X448,
            256 => Self::Ffdhe2048,
            257 => Self::Ffdhe3072,
            258 => Self::Ffdhe4096,
            259 => Self::Ffdhe6144,
            260 => Self::Ffdhe8192,
            _ => unreachable!("validated by combinator"),
        }
    }
}

impl From<NamedGroup> for u16 {
    fn from(v: NamedGroup) -> Self {
        u16::from(v)
    }
}

pub struct NamedGroupMapper;
impl Mapper for NamedGroupMapper {
    type Src<'p> = u16;
    type Dst<'p> = NamedGroup;
    type SrcBorrow<'s> = u16;
    type DstBorrow<'s> = NamedGroup;
    type SrcOwned = u16;
    type DstOwned = NamedGroup;
}

impl From<(u16, Vec<NamedGroup>)> for NamedGroupList {
    fn from(src: (u16, Vec<NamedGroup>)) -> Self {
        Self { l: src.0, list: src.1 }
    }
}

impl<'s> From<&'s NamedGroupList> for (u16, &'s [NamedGroup]) {
    fn from(v: &'s NamedGroupList) -> Self {
        (v.l, v.list.as_slice())
    }
}

impl<'a> From<NamedGroupList> for (u16, Vec<NamedGroup>) {
    fn from(v: NamedGroupList) -> Self {
        (v.l, v.list)
    }
}

pub struct NamedGroupListMapper;
impl Mapper for NamedGroupListMapper {
    type Src<'p> = (u16, Vec<NamedGroup>);
    type Dst<'p> = NamedGroupList;
    type SrcBorrow<'s> = (u16, &'s [NamedGroup]);
    type DstBorrow<'s> = &'s NamedGroupList;
    type SrcOwned = (u16, Vec<NamedGroup>);
    type DstOwned = NamedGroupList;
}

impl<'a> From<(u24, &'a [u8])> for Opaque0Ffffff<'a> {
    fn from(src: (u24, &'a [u8])) -> Self {
        Self { l: src.0, data: src.1 }
    }
}

impl<'s, 'a: 's> From<&'s Opaque0Ffffff<'a>> for (u24, &'s [u8]) {
    fn from(v: &'s Opaque0Ffffff<'a>) -> Self {
        (v.l, v.data)
    }
}

impl From<(u24, Vec<u8>)> for Opaque0FfffffOwned {
    fn from(src: (u24, Vec<u8>)) -> Self {
        Self { l: src.0, data: src.1 }
    }
}

impl From<Opaque0FfffffOwned> for (u24, Vec<u8>) {
    fn from(v: Opaque0FfffffOwned) -> Self {
        (v.l, v.data)
    }
}

pub struct Opaque0FfffffMapper;
impl Mapper for Opaque0FfffffMapper {
    type Src<'p> = (u24, &'p [u8]);
    type Dst<'p> = Opaque0Ffffff<'p>;
    type SrcBorrow<'s> = (u24, &'s [u8]);
    type DstBorrow<'s> = &'s Opaque0Ffffff<'s>;
    type SrcOwned = (u24, Vec<u8>);
    type DstOwned = Opaque0FfffffOwned;
}

impl From<(u8, Vec<CertificateType>)> for ClientCertTypeClientExtension {
    fn from(src: (u8, Vec<CertificateType>)) -> Self {
        Self { l: src.0, list: src.1 }
    }
}

impl<'s> From<&'s ClientCertTypeClientExtension> for (u8, &'s [CertificateType]) {
    fn from(v: &'s ClientCertTypeClientExtension) -> Self {
        (v.l, v.list.as_slice())
    }
}

impl<'a> From<ClientCertTypeClientExtension> for (u8, Vec<CertificateType>) {
    fn from(v: ClientCertTypeClientExtension) -> Self {
        (v.l, v.list)
    }
}

pub struct ClientCertTypeClientExtensionMapper;
impl Mapper for ClientCertTypeClientExtensionMapper {
    type Src<'p> = (u8, Vec<CertificateType>);
    type Dst<'p> = ClientCertTypeClientExtension;
    type SrcBorrow<'s> = (u8, &'s [CertificateType]);
    type DstBorrow<'s> = &'s ClientCertTypeClientExtension;
    type SrcOwned = (u8, Vec<CertificateType>);
    type DstOwned = ClientCertTypeClientExtension;
}

impl From<u8> for HeartbeatMode {
    fn from(src: u8) -> Self {
        match src as i128 {
            1 => Self::PeerAllowedToSend,
            2 => Self::PeerNotAllowedToSend,
            _ => unreachable!("validated by combinator"),
        }
    }
}

impl From<HeartbeatMode> for u8 {
    fn from(v: HeartbeatMode) -> Self {
        u8::from(v)
    }
}

pub struct HeartbeatModeMapper;
impl Mapper for HeartbeatModeMapper {
    type Src<'p> = u8;
    type Dst<'p> = HeartbeatMode;
    type SrcBorrow<'s> = u8;
    type DstBorrow<'s> = HeartbeatMode;
    type SrcOwned = u8;
    type DstOwned = HeartbeatMode;
}

impl From<HeartbeatMode> for HeartbeatExtension {
    fn from(src: HeartbeatMode) -> Self {
        Self { mode: src }
    }
}

impl<'s> From<HeartbeatExtension> for HeartbeatMode {
    fn from(v: HeartbeatExtension) -> Self {
        v.mode
    }
}

pub struct HeartbeatExtensionMapper;
impl Mapper for HeartbeatExtensionMapper {
    type Src<'p> = HeartbeatMode;
    type Dst<'p> = HeartbeatExtension;
    type SrcBorrow<'s> = HeartbeatMode;
    type DstBorrow<'s> = HeartbeatExtension;
    type SrcOwned = HeartbeatMode;
    type DstOwned = HeartbeatExtension;
}

impl<'a> From<(u24, &'a [u8])> for Opaque1Ffffff<'a> {
    fn from(src: (u24, &'a [u8])) -> Self {
        Self { l: src.0, data: src.1 }
    }
}

impl<'s, 'a: 's> From<&'s Opaque1Ffffff<'a>> for (u24, &'s [u8]) {
    fn from(v: &'s Opaque1Ffffff<'a>) -> Self {
        (v.l, v.data)
    }
}

impl From<(u24, Vec<u8>)> for Opaque1FfffffOwned {
    fn from(src: (u24, Vec<u8>)) -> Self {
        Self { l: src.0, data: src.1 }
    }
}

impl From<Opaque1FfffffOwned> for (u24, Vec<u8>) {
    fn from(v: Opaque1FfffffOwned) -> Self {
        (v.l, v.data)
    }
}

pub struct Opaque1FfffffMapper;
impl Mapper for Opaque1FfffffMapper {
    type Src<'p> = (u24, &'p [u8]);
    type Dst<'p> = Opaque1Ffffff<'p>;
    type SrcBorrow<'s> = (u24, &'s [u8]);
    type DstBorrow<'s> = &'s Opaque1Ffffff<'s>;
    type SrcOwned = (u24, Vec<u8>);
    type DstOwned = Opaque1FfffffOwned;
}

impl<'a> From<((), OcspResponse<'a>)> for CertificateStatus<'a> {
    fn from(src: ((), OcspResponse<'a>)) -> Self {
        Self {
            status_type: CERTIFICATE_STATUSSTATUS_TYPE_CONST,
            response: src.1,
        }
    }
}

impl<'s, 'a: 's> From<&'s CertificateStatus<'a>> for ((), &'s OcspResponse<'s>) {
    fn from(v: &'s CertificateStatus<'a>) -> Self {
        ((), &v.response)
    }
}

impl From<((), OcspResponseOwned)> for CertificateStatusOwned {
    fn from(src: ((), OcspResponseOwned)) -> Self {
        Self {
            status_type: CERTIFICATE_STATUSSTATUS_TYPE_CONST,
            response: src.1,
        }
    }
}

impl From<CertificateStatusOwned> for ((), OcspResponseOwned) {
    fn from(v: CertificateStatusOwned) -> Self {
        ((), v.response)
    }
}

pub struct CertificateStatusMapper;
impl Mapper for CertificateStatusMapper {
    type Src<'p> = ((), OcspResponse<'p>);
    type Dst<'p> = CertificateStatus<'p>;
    type SrcBorrow<'s> = ((), &'s OcspResponse<'s>);
    type DstBorrow<'s> = &'s CertificateStatus<'s>;
    type SrcOwned = ((), OcspResponseOwned);
    type DstOwned = CertificateStatusOwned;
}

impl From<u8> for EcPointFormat {
    fn from(src: u8) -> Self {
        match src as i128 {
            0 => Self::Uncompressed,
            1 => Self::AnsiX962CompressedPrime,
            2 => Self::AnsiX962CompressedChar2,
            _ => unreachable!("validated by combinator"),
        }
    }
}

impl From<EcPointFormat> for u8 {
    fn from(v: EcPointFormat) -> Self {
        u8::from(v)
    }
}

pub struct EcPointFormatMapper;
impl Mapper for EcPointFormatMapper {
    type Src<'p> = u8;
    type Dst<'p> = EcPointFormat;
    type SrcBorrow<'s> = u8;
    type DstBorrow<'s> = EcPointFormat;
    type SrcOwned = u8;
    type DstOwned = EcPointFormat;
}

impl<'a> From<((), OscpStatusRequest<'a>)> for CertificateStatusRequest<'a> {
    fn from(src: ((), OscpStatusRequest<'a>)) -> Self {
        Self {
            status_type: CERTIFICATE_STATUS_REQUESTSTATUS_TYPE_CONST,
            request: src.1,
        }
    }
}

impl<'s, 'a: 's> From<&'s CertificateStatusRequest<'a>>
for ((), &'s OscpStatusRequest<'s>) {
    fn from(v: &'s CertificateStatusRequest<'a>) -> Self {
        ((), &v.request)
    }
}

impl From<((), OscpStatusRequestOwned)> for CertificateStatusRequestOwned {
    fn from(src: ((), OscpStatusRequestOwned)) -> Self {
        Self {
            status_type: CERTIFICATE_STATUS_REQUESTSTATUS_TYPE_CONST,
            request: src.1,
        }
    }
}

impl From<CertificateStatusRequestOwned> for ((), OscpStatusRequestOwned) {
    fn from(v: CertificateStatusRequestOwned) -> Self {
        ((), v.request)
    }
}

pub struct CertificateStatusRequestMapper;
impl Mapper for CertificateStatusRequestMapper {
    type Src<'p> = ((), OscpStatusRequest<'p>);
    type Dst<'p> = CertificateStatusRequest<'p>;
    type SrcBorrow<'s> = ((), &'s OscpStatusRequest<'s>);
    type DstBorrow<'s> = &'s CertificateStatusRequest<'s>;
    type SrcOwned = ((), OscpStatusRequestOwned);
    type DstOwned = CertificateStatusRequestOwned;
}

impl From<(u8, Vec<EcPointFormat>)> for EcPointFormatList {
    fn from(src: (u8, Vec<EcPointFormat>)) -> Self {
        Self { l: src.0, list: src.1 }
    }
}

impl<'s> From<&'s EcPointFormatList> for (u8, &'s [EcPointFormat]) {
    fn from(v: &'s EcPointFormatList) -> Self {
        (v.l, v.list.as_slice())
    }
}

impl<'a> From<EcPointFormatList> for (u8, Vec<EcPointFormat>) {
    fn from(v: EcPointFormatList) -> Self {
        (v.l, v.list)
    }
}

pub struct EcPointFormatListMapper;
impl Mapper for EcPointFormatListMapper {
    type Src<'p> = (u8, Vec<EcPointFormat>);
    type Dst<'p> = EcPointFormatList;
    type SrcBorrow<'s> = (u8, &'s [EcPointFormat]);
    type DstBorrow<'s> = &'s EcPointFormatList;
    type SrcOwned = (u8, Vec<EcPointFormat>);
    type DstOwned = EcPointFormatList;
}

impl<'a> From<(u16, Vec<ProtocolName<'a>>)> for ProtocolNameList<'a> {
    fn from(src: (u16, Vec<ProtocolName<'a>>)) -> Self {
        Self { l: src.0, list: src.1 }
    }
}

impl<'s, 'a: 's> From<&'s ProtocolNameList<'a>> for (u16, &'s [&'s ProtocolName<'s>]) {
    fn from(v: &'s ProtocolNameList<'a>) -> Self {
        (v.l, v.list.as_slice())
    }
}

impl From<(u16, Vec<ProtocolNameOwned>)> for ProtocolNameListOwned {
    fn from(src: (u16, Vec<ProtocolNameOwned>)) -> Self {
        Self { l: src.0, list: src.1 }
    }
}

impl From<ProtocolNameListOwned> for (u16, Vec<ProtocolNameOwned>) {
    fn from(v: ProtocolNameListOwned) -> Self {
        (v.l, v.list)
    }
}

pub struct ProtocolNameListMapper;
impl Mapper for ProtocolNameListMapper {
    type Src<'p> = (u16, Vec<ProtocolName<'p>>);
    type Dst<'p> = ProtocolNameList<'p>;
    type SrcBorrow<'s> = (u16, &'s [&'s ProtocolName<'s>]);
    type DstBorrow<'s> = &'s ProtocolNameList<'s>;
    type SrcOwned = (u16, Vec<ProtocolNameOwned>);
    type DstOwned = ProtocolNameListOwned;
}

pub const ZERO_BYTEZERO_CONST: u8 = 0;
pub const CERTIFICATE_STATUSSTATUS_TYPE_CONST: u8 = 1;
pub const CERTIFICATE_STATUS_REQUESTSTATUS_TYPE_CONST: u8 = 1;
///Type alias for certificate_type combinator
pub type CertificateTypeCombinatorAlias = Mapped<
    Refined<U8, fn(u8) -> bool>,
    CertificateTypeMapper,
>;
///Wrapper struct for certificate_type combinator
pub struct CertificateTypeCombinator<C = CertificateTypeCombinatorAlias>(pub C);
///Type alias for server_cert_type_server_extension combinator
pub type ServerCertTypeServerExtensionCombinatorAlias = Mapped<
    CertificateTypeCombinator,
    ServerCertTypeServerExtensionMapper,
>;
///Wrapper struct for server_cert_type_server_extension combinator
pub struct ServerCertTypeServerExtensionCombinator<
    C = ServerCertTypeServerExtensionCombinatorAlias,
>(
    pub C,
);
///Type alias for opaque_1_ff combinator
pub type Opaque1FfCombinatorAlias = Mapped<
    Pair<Refined<U8, fn(u8) -> bool>, Opaque1FfDep>,
    Opaque1FfMapper,
>;
///Wrapper struct for opaque_1_ff combinator
pub struct Opaque1FfCombinator<C = Opaque1FfCombinatorAlias>(pub C);
///Type alias for opaque_1_ffff combinator
pub type Opaque1FfffCombinatorAlias = Mapped<
    Pair<Refined<U16Be, fn(u16) -> bool>, Opaque1FfffDep>,
    Opaque1FfffMapper,
>;
///Wrapper struct for opaque_1_ffff combinator
pub struct Opaque1FfffCombinator<C = Opaque1FfffCombinatorAlias>(pub C);
///Type alias for name_type combinator
pub type NameTypeCombinatorAlias = Mapped<Refined<U8, fn(u8) -> bool>, NameTypeMapper>;
///Wrapper struct for name_type combinator
pub struct NameTypeCombinator<C = NameTypeCombinatorAlias>(pub C);
///Type alias for host_name combinator
pub type HostNameCombinatorAlias = Opaque1FfffCombinator;
///Wrapper struct for host_name combinator
pub struct HostNameCombinator<C = HostNameCombinatorAlias>(pub C);
///Type alias for unknown_name combinator
pub type UnknownNameCombinatorAlias = Opaque1FfffCombinator;
///Wrapper struct for unknown_name combinator
pub struct UnknownNameCombinator<C = UnknownNameCombinatorAlias>(pub C);
///Type alias for server_name_name combinator
pub type ServerNameNameCombinatorAlias<'x> = Dispatch<
    'x,
    NameType,
    ServerNameNameDispatchCase,
    1,
>;
///Wrapper struct for server_name_name combinator
pub struct ServerNameNameCombinator<C = ServerNameNameCombinatorAlias<'static>>(pub C);
///Type alias for server_name combinator
pub type ServerNameCombinatorAlias = Mapped<
    Pair<NameTypeCombinator, ServerNameDep>,
    ServerNameMapper,
>;
///Wrapper struct for server_name combinator
pub struct ServerNameCombinator<C = ServerNameCombinatorAlias>(pub C);
///Type alias for server_name_list combinator
pub type ServerNameListCombinatorAlias = Mapped<
    Pair<Refined<U16Be, fn(u16) -> bool>, ServerNameListDep>,
    ServerNameListMapper,
>;
///Wrapper struct for server_name_list combinator
pub struct ServerNameListCombinator<C = ServerNameListCombinatorAlias>(pub C);
///Type alias for opaque_0_ffff combinator
pub type Opaque0FfffCombinatorAlias = Mapped<
    Pair<U16Be, Opaque0FfffDep>,
    Opaque0FfffMapper,
>;
///Wrapper struct for opaque_0_ffff combinator
pub struct Opaque0FfffCombinator<C = Opaque0FfffCombinatorAlias>(pub C);
///Type alias for ocsp_extensions combinator
pub type OcspExtensionsCombinatorAlias = Opaque0FfffCombinator;
///Wrapper struct for ocsp_extensions combinator
pub struct OcspExtensionsCombinator<C = OcspExtensionsCombinatorAlias>(pub C);
///Type alias for responder_id combinator
pub type ResponderIdCombinatorAlias = Opaque1FfffCombinator;
///Wrapper struct for responder_id combinator
pub struct ResponderIdCombinator<C = ResponderIdCombinatorAlias>(pub C);
///Type alias for responder_id_list combinator
pub type ResponderIdListCombinatorAlias = Mapped<
    Pair<U16Be, ResponderIdListDep>,
    ResponderIdListMapper,
>;
///Wrapper struct for responder_id_list combinator
pub struct ResponderIdListCombinator<C = ResponderIdListCombinatorAlias>(pub C);
///Type alias for oscp_status_request combinator
pub type OscpStatusRequestCombinatorAlias = Mapped<
    (ResponderIdListCombinator, OcspExtensionsCombinator),
    OscpStatusRequestMapper,
>;
///Wrapper struct for oscp_status_request combinator
pub struct OscpStatusRequestCombinator<C = OscpStatusRequestCombinatorAlias>(pub C);
///Type alias for srtp_protection_profile combinator
pub type SrtpProtectionProfileCombinatorAlias = Fixed<2>;
///Wrapper struct for srtp_protection_profile combinator
pub struct SrtpProtectionProfileCombinator<C = SrtpProtectionProfileCombinatorAlias>(
    pub C,
);
///Type alias for srtp_protection_profiles combinator
pub type SrtpProtectionProfilesCombinatorAlias = Mapped<
    Pair<Refined<U16Be, fn(u16) -> bool>, SrtpProtectionProfilesDep>,
    SrtpProtectionProfilesMapper,
>;
///Wrapper struct for srtp_protection_profiles combinator
pub struct SrtpProtectionProfilesCombinator<C = SrtpProtectionProfilesCombinatorAlias>(
    pub C,
);
///Type alias for opaque_0_ff combinator
pub type Opaque0FfCombinatorAlias = Mapped<Pair<U8, Opaque0FfDep>, Opaque0FfMapper>;
///Wrapper struct for opaque_0_ff combinator
pub struct Opaque0FfCombinator<C = Opaque0FfCombinatorAlias>(pub C);
///Type alias for use_srtp_data combinator
pub type UseSrtpDataCombinatorAlias = Mapped<
    (SrtpProtectionProfilesCombinator, Opaque0FfCombinator),
    UseSrtpDataMapper,
>;
///Wrapper struct for use_srtp_data combinator
pub struct UseSrtpDataCombinator<C = UseSrtpDataCombinatorAlias>(pub C);
///Type alias for max_fragment_length combinator
pub type MaxFragmentLengthCombinatorAlias = Mapped<
    Refined<U8, fn(u8) -> bool>,
    MaxFragmentLengthMapper,
>;
///Wrapper struct for max_fragment_length combinator
pub struct MaxFragmentLengthCombinator<C = MaxFragmentLengthCombinatorAlias>(pub C);
///Type alias for signature_scheme combinator
pub type SignatureSchemeCombinatorAlias = Mapped<
    Refined<U16Be, fn(u16) -> bool>,
    SignatureSchemeMapper,
>;
///Wrapper struct for signature_scheme combinator
pub struct SignatureSchemeCombinator<C = SignatureSchemeCombinatorAlias>(pub C);
///Type alias for zero_byte combinator
pub type ZeroByteCombinatorAlias = Mapped<Tag<U8, u8>, ZeroByteMapper>;
///Wrapper struct for zero_byte combinator
pub struct ZeroByteCombinator<C = ZeroByteCombinatorAlias>(pub C);
///Type alias for serialized_sct combinator
pub type SerializedSctCombinatorAlias = Opaque1FfffCombinator;
///Wrapper struct for serialized_sct combinator
pub struct SerializedSctCombinator<C = SerializedSctCombinatorAlias>(pub C);
///Type alias for padding_extension combinator
pub type PaddingExtensionCombinatorAlias = Mapped<
    Pair<U16Be, PaddingExtensionDep>,
    PaddingExtensionMapper,
>;
///Wrapper struct for padding_extension combinator
pub struct PaddingExtensionCombinator<C = PaddingExtensionCombinatorAlias>(pub C);
///Type alias for signature_scheme_list combinator
pub type SignatureSchemeListCombinatorAlias = Mapped<
    Pair<Refined<U16Be, fn(u16) -> bool>, SignatureSchemeListDep>,
    SignatureSchemeListMapper,
>;
///Wrapper struct for signature_scheme_list combinator
pub struct SignatureSchemeListCombinator<C = SignatureSchemeListCombinatorAlias>(pub C);
///Type alias for extension_type combinator
pub type ExtensionTypeCombinatorAlias = Mapped<
    Refined<U16Be, fn(u16) -> bool>,
    ExtensionTypeMapper,
>;
///Wrapper struct for extension_type combinator
pub struct ExtensionTypeCombinator<C = ExtensionTypeCombinatorAlias>(pub C);
///Type alias for protocol_name combinator
pub type ProtocolNameCombinatorAlias = Opaque1FfCombinator;
///Wrapper struct for protocol_name combinator
pub struct ProtocolNameCombinator<C = ProtocolNameCombinatorAlias>(pub C);
///Type alias for protocol_version combinator
pub type ProtocolVersionCombinatorAlias = Mapped<
    Refined<U16Be, fn(u16) -> bool>,
    ProtocolVersionMapper,
>;
///Wrapper struct for protocol_version combinator
pub struct ProtocolVersionCombinator<C = ProtocolVersionCombinatorAlias>(pub C);
///Type alias for server_cert_type_client_extension combinator
pub type ServerCertTypeClientExtensionCombinatorAlias = Mapped<
    Pair<Refined<U8, fn(u8) -> bool>, ServerCertTypeClientExtensionDep>,
    ServerCertTypeClientExtensionMapper,
>;
///Wrapper struct for server_cert_type_client_extension combinator
pub struct ServerCertTypeClientExtensionCombinator<
    C = ServerCertTypeClientExtensionCombinatorAlias,
>(
    pub C,
);
///Type alias for signed_certificate_timestamp_list combinator
pub type SignedCertificateTimestampListCombinatorAlias = Mapped<
    Pair<Refined<U16Be, fn(u16) -> bool>, SignedCertificateTimestampListDep>,
    SignedCertificateTimestampListMapper,
>;
///Wrapper struct for signed_certificate_timestamp_list combinator
pub struct SignedCertificateTimestampListCombinator<
    C = SignedCertificateTimestampListCombinatorAlias,
>(
    pub C,
);
///Type alias for client_cert_type_server_extension combinator
pub type ClientCertTypeServerExtensionCombinatorAlias = Mapped<
    CertificateTypeCombinator,
    ClientCertTypeServerExtensionMapper,
>;
///Wrapper struct for client_cert_type_server_extension combinator
pub struct ClientCertTypeServerExtensionCombinator<
    C = ClientCertTypeServerExtensionCombinatorAlias,
>(
    pub C,
);
///Type alias for opaque_2_ffff combinator
pub type Opaque2FfffCombinatorAlias = Mapped<
    Pair<Refined<U16Be, fn(u16) -> bool>, Opaque2FfffDep>,
    Opaque2FfffMapper,
>;
///Wrapper struct for opaque_2_ffff combinator
pub struct Opaque2FfffCombinator<C = Opaque2FfffCombinatorAlias>(pub C);
///Type alias for named_group combinator
pub type NamedGroupCombinatorAlias = Mapped<
    Refined<U16Be, fn(u16) -> bool>,
    NamedGroupMapper,
>;
///Wrapper struct for named_group combinator
pub struct NamedGroupCombinator<C = NamedGroupCombinatorAlias>(pub C);
///Type alias for named_group_list combinator
pub type NamedGroupListCombinatorAlias = Mapped<
    Pair<Refined<U16Be, fn(u16) -> bool>, NamedGroupListDep>,
    NamedGroupListMapper,
>;
///Wrapper struct for named_group_list combinator
pub struct NamedGroupListCombinator<C = NamedGroupListCombinatorAlias>(pub C);
///Type alias for opaque_0_ffffff combinator
pub type Opaque0FfffffCombinatorAlias = Mapped<
    Pair<U24Be, Opaque0FfffffDep>,
    Opaque0FfffffMapper,
>;
///Wrapper struct for opaque_0_ffffff combinator
pub struct Opaque0FfffffCombinator<C = Opaque0FfffffCombinatorAlias>(pub C);
///Type alias for client_cert_type_client_extension combinator
pub type ClientCertTypeClientExtensionCombinatorAlias = Mapped<
    Pair<Refined<U8, fn(u8) -> bool>, ClientCertTypeClientExtensionDep>,
    ClientCertTypeClientExtensionMapper,
>;
///Wrapper struct for client_cert_type_client_extension combinator
pub struct ClientCertTypeClientExtensionCombinator<
    C = ClientCertTypeClientExtensionCombinatorAlias,
>(
    pub C,
);
///Type alias for heartbeat_mode combinator
pub type HeartbeatModeCombinatorAlias = Mapped<
    Refined<U8, fn(u8) -> bool>,
    HeartbeatModeMapper,
>;
///Wrapper struct for heartbeat_mode combinator
pub struct HeartbeatModeCombinator<C = HeartbeatModeCombinatorAlias>(pub C);
///Type alias for heartbeat_extension combinator
pub type HeartbeatExtensionCombinatorAlias = Mapped<
    HeartbeatModeCombinator,
    HeartbeatExtensionMapper,
>;
///Wrapper struct for heartbeat_extension combinator
pub struct HeartbeatExtensionCombinator<C = HeartbeatExtensionCombinatorAlias>(pub C);
///Type alias for opaque_1_ffffff combinator
pub type Opaque1FfffffCombinatorAlias = Mapped<
    Pair<Refined<U24Be, fn(u24) -> bool>, Opaque1FfffffDep>,
    Opaque1FfffffMapper,
>;
///Wrapper struct for opaque_1_ffffff combinator
pub struct Opaque1FfffffCombinator<C = Opaque1FfffffCombinatorAlias>(pub C);
///Type alias for ocsp_response combinator
pub type OcspResponseCombinatorAlias = Opaque1FfffffCombinator;
///Wrapper struct for ocsp_response combinator
pub struct OcspResponseCombinator<C = OcspResponseCombinatorAlias>(pub C);
///Type alias for certificate_status combinator
pub type CertificateStatusCombinatorAlias = Mapped<
    (Tag<U8, u8>, OcspResponseCombinator),
    CertificateStatusMapper,
>;
///Wrapper struct for certificate_status combinator
pub struct CertificateStatusCombinator<C = CertificateStatusCombinatorAlias>(pub C);
///Type alias for ec_point_format combinator
pub type EcPointFormatCombinatorAlias = Mapped<
    Refined<U8, fn(u8) -> bool>,
    EcPointFormatMapper,
>;
///Wrapper struct for ec_point_format combinator
pub struct EcPointFormatCombinator<C = EcPointFormatCombinatorAlias>(pub C);
///Type alias for certificate_status_request combinator
pub type CertificateStatusRequestCombinatorAlias = Mapped<
    (Tag<U8, u8>, OscpStatusRequestCombinator),
    CertificateStatusRequestMapper,
>;
///Wrapper struct for certificate_status_request combinator
pub struct CertificateStatusRequestCombinator<
    C = CertificateStatusRequestCombinatorAlias,
>(
    pub C,
);
///Type alias for ec_point_format_list combinator
pub type EcPointFormatListCombinatorAlias = Mapped<
    Pair<Refined<U8, fn(u8) -> bool>, EcPointFormatListDep>,
    EcPointFormatListMapper,
>;
///Wrapper struct for ec_point_format_list combinator
pub struct EcPointFormatListCombinator<C = EcPointFormatListCombinatorAlias>(pub C);
///Type alias for protocol_name_list combinator
pub type ProtocolNameListCombinatorAlias = Mapped<
    Pair<Refined<U16Be, fn(u16) -> bool>, ProtocolNameListDep>,
    ProtocolNameListMapper,
>;
///Wrapper struct for protocol_name_list combinator
pub struct ProtocolNameListCombinator<C = ProtocolNameListCombinatorAlias>(pub C);
///Constructor for certificate_type combinator
pub fn certificate_type() -> CertificateTypeCombinator {
    CertificateTypeCombinator(
        Mapped::new(
            Refined {
                inner: U8,
                predicate: |v: u8| (v as i128 == 0 || v as i128 == 2),
            },
            CertificateTypeMapper,
        ),
    )
}

///Constructor for server_cert_type_server_extension combinator
pub fn server_cert_type_server_extension() -> ServerCertTypeServerExtensionCombinator {
    ServerCertTypeServerExtensionCombinator(
        Mapped::new(certificate_type(), ServerCertTypeServerExtensionMapper),
    )
}

///Constructor for opaque_1_ff combinator
pub fn opaque_1_ff() -> Opaque1FfCombinator {
    Opaque1FfCombinator(
        Mapped::new(
            Pair::new(
                Refined {
                    inner: U8,
                    predicate: |v: u8| (v as i128 >= 1 && v as i128 <= 255),
                },
                Opaque1FfDep {},
            ),
            Opaque1FfMapper,
        ),
    )
}

///Constructor for opaque_1_ffff combinator
pub fn opaque_1_ffff() -> Opaque1FfffCombinator {
    Opaque1FfffCombinator(
        Mapped::new(
            Pair::new(
                Refined {
                    inner: U16Be,
                    predicate: |v: u16| (v as i128 >= 1 && v as i128 <= 65535),
                },
                Opaque1FfffDep {},
            ),
            Opaque1FfffMapper,
        ),
    )
}

///Constructor for name_type combinator
pub fn name_type() -> NameTypeCombinator {
    NameTypeCombinator(
        Mapped::new(
            Refined {
                inner: U8,
                predicate: |v: u8| (v as i128 == 0),
            },
            NameTypeMapper,
        ),
    )
}

///Constructor for host_name combinator
pub fn host_name() -> HostNameCombinator {
    HostNameCombinator(opaque_1_ffff())
}

///Constructor for unknown_name combinator
pub fn unknown_name() -> UnknownNameCombinator {
    UnknownNameCombinator(opaque_1_ffff())
}

///Constructor for server_name_name combinator
pub fn server_name_name<'a, NameTypeArg>(
    name_type: NameTypeArg,
) -> ServerNameNameCombinator<ServerNameNameCombinatorAlias<'a>>
where
    NameTypeArg: RuntimeValParam<'a, NameType>,
{
    ServerNameNameCombinator(
        Dispatch::new(
            name_type.into_runtime_value(),
            [(NameType::HostName, ServerNameNameDispatchCase::V1(host_name()))],
            Some(ServerNameNameDispatchCase::V2(unknown_name())),
        ),
    )
}

///Constructor for server_name combinator
pub fn server_name() -> ServerNameCombinator {
    ServerNameCombinator(
        Mapped::new(Pair::new(name_type(), ServerNameDep {}), ServerNameMapper),
    )
}

///Constructor for server_name_list combinator
pub fn server_name_list() -> ServerNameListCombinator {
    ServerNameListCombinator(
        Mapped::new(
            Pair::new(
                Refined {
                    inner: U16Be,
                    predicate: |v: u16| v as i128 >= 1 && v as i128 <= 65535,
                },
                ServerNameListDep {},
            ),
            ServerNameListMapper,
        ),
    )
}

///Constructor for opaque_0_ffff combinator
pub fn opaque_0_ffff() -> Opaque0FfffCombinator {
    Opaque0FfffCombinator(
        Mapped::new(Pair::new(U16Be, Opaque0FfffDep {}), Opaque0FfffMapper),
    )
}

///Constructor for ocsp_extensions combinator
pub fn ocsp_extensions() -> OcspExtensionsCombinator {
    OcspExtensionsCombinator(opaque_0_ffff())
}

///Constructor for responder_id combinator
pub fn responder_id() -> ResponderIdCombinator {
    ResponderIdCombinator(opaque_1_ffff())
}

///Constructor for responder_id_list combinator
pub fn responder_id_list() -> ResponderIdListCombinator {
    ResponderIdListCombinator(
        Mapped::new(Pair::new(U16Be, ResponderIdListDep {}), ResponderIdListMapper),
    )
}

///Constructor for oscp_status_request combinator
pub fn oscp_status_request() -> OscpStatusRequestCombinator {
    OscpStatusRequestCombinator(
        Mapped::new((responder_id_list(), ocsp_extensions()), OscpStatusRequestMapper),
    )
}

///Constructor for srtp_protection_profile combinator
pub fn srtp_protection_profile() -> SrtpProtectionProfileCombinator {
    SrtpProtectionProfileCombinator(Fixed::<2>)
}

///Constructor for srtp_protection_profiles combinator
pub fn srtp_protection_profiles() -> SrtpProtectionProfilesCombinator {
    SrtpProtectionProfilesCombinator(
        Mapped::new(
            Pair::new(
                Refined {
                    inner: U16Be,
                    predicate: |v: u16| v as i128 >= 2 && v as i128 <= 65535,
                },
                SrtpProtectionProfilesDep {},
            ),
            SrtpProtectionProfilesMapper,
        ),
    )
}

///Constructor for opaque_0_ff combinator
pub fn opaque_0_ff() -> Opaque0FfCombinator {
    Opaque0FfCombinator(Mapped::new(Pair::new(U8, Opaque0FfDep {}), Opaque0FfMapper))
}

///Constructor for use_srtp_data combinator
pub fn use_srtp_data() -> UseSrtpDataCombinator {
    UseSrtpDataCombinator(
        Mapped::new((srtp_protection_profiles(), opaque_0_ff()), UseSrtpDataMapper),
    )
}

///Constructor for max_fragment_length combinator
pub fn max_fragment_length() -> MaxFragmentLengthCombinator {
    MaxFragmentLengthCombinator(
        Mapped::new(
            Refined {
                inner: U8,
                predicate: |v: u8| {
                    (v as i128 == 1 || v as i128 == 2 || v as i128 == 3
                        || v as i128 == 4)
                },
            },
            MaxFragmentLengthMapper,
        ),
    )
}

///Constructor for signature_scheme combinator
pub fn signature_scheme() -> SignatureSchemeCombinator {
    SignatureSchemeCombinator(
        Mapped::new(
            Refined {
                inner: U16Be,
                predicate: |v: u16| {
                    (v as i128 == 257 || v as i128 == 513 || v as i128 == 259
                        || v as i128 == 515 || v as i128 == 1025 || v as i128 == 1281
                        || v as i128 == 1537 || v as i128 == 1027 || v as i128 == 1283
                        || v as i128 == 1539 || v as i128 == 2052 || v as i128 == 2053
                        || v as i128 == 2054 || v as i128 == 2055 || v as i128 == 2056
                        || v as i128 == 2057 || v as i128 == 2058 || v as i128 == 2059)
                },
            },
            SignatureSchemeMapper,
        ),
    )
}

///Constructor for zero_byte combinator
pub fn zero_byte() -> ZeroByteCombinator {
    ZeroByteCombinator(Mapped::new(Tag::new(U8, ZERO_BYTEZERO_CONST), ZeroByteMapper))
}

///Constructor for serialized_sct combinator
pub fn serialized_sct() -> SerializedSctCombinator {
    SerializedSctCombinator(opaque_1_ffff())
}

///Constructor for padding_extension combinator
pub fn padding_extension(ext_len: u16) -> PaddingExtensionCombinator {
    PaddingExtensionCombinator(
        Mapped::new(Pair::new(U16Be, PaddingExtensionDep {}), PaddingExtensionMapper),
    )
}

///Constructor for signature_scheme_list combinator
pub fn signature_scheme_list() -> SignatureSchemeListCombinator {
    SignatureSchemeListCombinator(
        Mapped::new(
            Pair::new(
                Refined {
                    inner: U16Be,
                    predicate: |v: u16| v as i128 >= 2 && v as i128 <= 65534,
                },
                SignatureSchemeListDep {},
            ),
            SignatureSchemeListMapper,
        ),
    )
}

///Constructor for extension_type combinator
pub fn extension_type() -> ExtensionTypeCombinator {
    ExtensionTypeCombinator(
        Mapped::new(
            Refined {
                inner: U16Be,
                predicate: |v: u16| {
                    (v as i128 == 0 || v as i128 == 1 || v as i128 == 5
                        || v as i128 == 10 || v as i128 == 11 || v as i128 == 13
                        || v as i128 == 14 || v as i128 == 15 || v as i128 == 16
                        || v as i128 == 18 || v as i128 == 19 || v as i128 == 20
                        || v as i128 == 21 || v as i128 == 22 || v as i128 == 23
                        || v as i128 == 35 || v as i128 == 41 || v as i128 == 42
                        || v as i128 == 43 || v as i128 == 44 || v as i128 == 45
                        || v as i128 == 47 || v as i128 == 48 || v as i128 == 49
                        || v as i128 == 50 || v as i128 == 51 || v as i128 == 65535)
                },
            },
            ExtensionTypeMapper,
        ),
    )
}

///Constructor for protocol_name combinator
pub fn protocol_name() -> ProtocolNameCombinator {
    ProtocolNameCombinator(opaque_1_ff())
}

///Constructor for protocol_version combinator
pub fn protocol_version() -> ProtocolVersionCombinator {
    ProtocolVersionCombinator(
        Mapped::new(
            Refined {
                inner: U16Be,
                predicate: |v: u16| {
                    (v as i128 == 768 || v as i128 == 769 || v as i128 == 770
                        || v as i128 == 771 || v as i128 == 772)
                },
            },
            ProtocolVersionMapper,
        ),
    )
}

///Constructor for server_cert_type_client_extension combinator
pub fn server_cert_type_client_extension() -> ServerCertTypeClientExtensionCombinator {
    ServerCertTypeClientExtensionCombinator(
        Mapped::new(
            Pair::new(
                Refined {
                    inner: U8,
                    predicate: |v: u8| v as i128 >= 1 && v as i128 <= 255,
                },
                ServerCertTypeClientExtensionDep {
                },
            ),
            ServerCertTypeClientExtensionMapper,
        ),
    )
}

///Constructor for signed_certificate_timestamp_list combinator
pub fn signed_certificate_timestamp_list() -> SignedCertificateTimestampListCombinator {
    SignedCertificateTimestampListCombinator(
        Mapped::new(
            Pair::new(
                Refined {
                    inner: U16Be,
                    predicate: |v: u16| v as i128 >= 1 && v as i128 <= 65535,
                },
                SignedCertificateTimestampListDep {
                },
            ),
            SignedCertificateTimestampListMapper,
        ),
    )
}

///Constructor for client_cert_type_server_extension combinator
pub fn client_cert_type_server_extension() -> ClientCertTypeServerExtensionCombinator {
    ClientCertTypeServerExtensionCombinator(
        Mapped::new(certificate_type(), ClientCertTypeServerExtensionMapper),
    )
}

///Constructor for opaque_2_ffff combinator
pub fn opaque_2_ffff() -> Opaque2FfffCombinator {
    Opaque2FfffCombinator(
        Mapped::new(
            Pair::new(
                Refined {
                    inner: U16Be,
                    predicate: |v: u16| (v as i128 >= 2 && v as i128 <= 65535),
                },
                Opaque2FfffDep {},
            ),
            Opaque2FfffMapper,
        ),
    )
}

///Constructor for named_group combinator
pub fn named_group() -> NamedGroupCombinator {
    NamedGroupCombinator(
        Mapped::new(
            Refined {
                inner: U16Be,
                predicate: |v: u16| {
                    (v as i128 == 1 || v as i128 == 2 || v as i128 == 3 || v as i128 == 4
                        || v as i128 == 5 || v as i128 == 6 || v as i128 == 7
                        || v as i128 == 8 || v as i128 == 9 || v as i128 == 10
                        || v as i128 == 11 || v as i128 == 12 || v as i128 == 13
                        || v as i128 == 14 || v as i128 == 15 || v as i128 == 16
                        || v as i128 == 17 || v as i128 == 18 || v as i128 == 19
                        || v as i128 == 20 || v as i128 == 21 || v as i128 == 22
                        || v as i128 == 23 || v as i128 == 24 || v as i128 == 25
                        || v as i128 == 29 || v as i128 == 30 || v as i128 == 256
                        || v as i128 == 257 || v as i128 == 258 || v as i128 == 259
                        || v as i128 == 260)
                },
            },
            NamedGroupMapper,
        ),
    )
}

///Constructor for named_group_list combinator
pub fn named_group_list() -> NamedGroupListCombinator {
    NamedGroupListCombinator(
        Mapped::new(
            Pair::new(
                Refined {
                    inner: U16Be,
                    predicate: |v: u16| v as i128 >= 2 && v as i128 <= 65535,
                },
                NamedGroupListDep {},
            ),
            NamedGroupListMapper,
        ),
    )
}

///Constructor for opaque_0_ffffff combinator
pub fn opaque_0_ffffff() -> Opaque0FfffffCombinator {
    Opaque0FfffffCombinator(
        Mapped::new(Pair::new(U24Be, Opaque0FfffffDep {}), Opaque0FfffffMapper),
    )
}

///Constructor for client_cert_type_client_extension combinator
pub fn client_cert_type_client_extension() -> ClientCertTypeClientExtensionCombinator {
    ClientCertTypeClientExtensionCombinator(
        Mapped::new(
            Pair::new(
                Refined {
                    inner: U8,
                    predicate: |v: u8| v as i128 >= 1 && v as i128 <= 255,
                },
                ClientCertTypeClientExtensionDep {
                },
            ),
            ClientCertTypeClientExtensionMapper,
        ),
    )
}

///Constructor for heartbeat_mode combinator
pub fn heartbeat_mode() -> HeartbeatModeCombinator {
    HeartbeatModeCombinator(
        Mapped::new(
            Refined {
                inner: U8,
                predicate: |v: u8| (v as i128 == 1 || v as i128 == 2),
            },
            HeartbeatModeMapper,
        ),
    )
}

///Constructor for heartbeat_extension combinator
pub fn heartbeat_extension() -> HeartbeatExtensionCombinator {
    HeartbeatExtensionCombinator(Mapped::new(heartbeat_mode(), HeartbeatExtensionMapper))
}

///Constructor for opaque_1_ffffff combinator
pub fn opaque_1_ffffff() -> Opaque1FfffffCombinator {
    Opaque1FfffffCombinator(
        Mapped::new(
            Pair::new(
                Refined {
                    inner: U24Be,
                    predicate: |v: u24| {
                        (v.as_u32() as i128 >= 1 && v.as_u32() as i128 <= 16777215)
                    },
                },
                Opaque1FfffffDep {},
            ),
            Opaque1FfffffMapper,
        ),
    )
}

///Constructor for ocsp_response combinator
pub fn ocsp_response() -> OcspResponseCombinator {
    OcspResponseCombinator(opaque_1_ffffff())
}

///Constructor for certificate_status combinator
pub fn certificate_status() -> CertificateStatusCombinator {
    CertificateStatusCombinator(
        Mapped::new(
            (Tag::new(U8, CERTIFICATE_STATUSSTATUS_TYPE_CONST), ocsp_response()),
            CertificateStatusMapper,
        ),
    )
}

///Constructor for ec_point_format combinator
pub fn ec_point_format() -> EcPointFormatCombinator {
    EcPointFormatCombinator(
        Mapped::new(
            Refined {
                inner: U8,
                predicate: |v: u8| (v as i128 == 0 || v as i128 == 1 || v as i128 == 2),
            },
            EcPointFormatMapper,
        ),
    )
}

///Constructor for certificate_status_request combinator
pub fn certificate_status_request() -> CertificateStatusRequestCombinator {
    CertificateStatusRequestCombinator(
        Mapped::new(
            (
                Tag::new(U8, CERTIFICATE_STATUS_REQUESTSTATUS_TYPE_CONST),
                oscp_status_request(),
            ),
            CertificateStatusRequestMapper,
        ),
    )
}

///Constructor for ec_point_format_list combinator
pub fn ec_point_format_list() -> EcPointFormatListCombinator {
    EcPointFormatListCombinator(
        Mapped::new(
            Pair::new(
                Refined {
                    inner: U8,
                    predicate: |v: u8| v as i128 >= 1 && v as i128 <= 255,
                },
                EcPointFormatListDep {},
            ),
            EcPointFormatListMapper,
        ),
    )
}

///Constructor for protocol_name_list combinator
pub fn protocol_name_list() -> ProtocolNameListCombinator {
    ProtocolNameListCombinator(
        Mapped::new(
            Pair::new(
                Refined {
                    inner: U16Be,
                    predicate: |v: u16| v as i128 >= 2 && v as i128 <= 65535,
                },
                ProtocolNameListDep {},
            ),
            ProtocolNameListMapper,
        ),
    )
}

#[derive(Clone, Copy)]
pub struct Opaque1FfDep {}
impl DepCombinator<Refined<U8, fn(u8) -> bool>, [u8], Vec<u8>> for Opaque1FfDep {
    type Out = Variable;
    type OutGen<'g> = Variable;
    fn dep_snd<'s>(&self, fst: u8) -> Self::Out {
        let fst: u8 = fst;
        let l = fst;
        Variable(l as usize)
    }
    fn dep_snd_gen<'g>(&self, fst: &'g mut u8) -> Self::OutGen<'g> {
        let fst: &'g mut u8 = fst;
        let l = fst;
        Variable(*l as usize)
    }
}

#[derive(Clone, Copy)]
pub struct Opaque1FfffDep {}
impl DepCombinator<Refined<U16Be, fn(u16) -> bool>, [u8], Vec<u8>> for Opaque1FfffDep {
    type Out = Variable;
    type OutGen<'g> = Variable;
    fn dep_snd<'s>(&self, fst: u16) -> Self::Out {
        let fst: u16 = fst;
        let l = fst;
        Variable(l as usize)
    }
    fn dep_snd_gen<'g>(&self, fst: &'g mut u16) -> Self::OutGen<'g> {
        let fst: &'g mut u16 = fst;
        let l = fst;
        Variable(*l as usize)
    }
}

pub enum ServerNameNameDispatchCase<
    C0 = HostNameCombinator,
    C1 = UnknownNameCombinator,
> {
    V1(C0),
    V2(C1),
}

impl<C0, C1> Combinator<[u8], Vec<u8>> for ServerNameNameDispatchCase<C0, C1>
where
    for<'p, 's> C0: Combinator<
        [u8],
        Vec<u8>,
        Type<'p> = HostName<'p>,
        SType<'s> = &'s HostName<'s>,
        GType = HostNameOwned,
    >,
    for<'p, 's> C1: Combinator<
        [u8],
        Vec<u8>,
        Type<'p> = UnknownName<'p>,
        SType<'s> = &'s UnknownName<'s>,
        GType = UnknownNameOwned,
    >,
{
    type Type<'p> = ServerNameName<'p>;
    type SType<'s> = &'s ServerNameName<'s>;
    type GType = ServerNameNameOwned;
    fn length<'s>(&self, v: Self::SType<'s>) -> usize
    where
        [u8]: 's,
    {
        match (self, v) {
            (ServerNameNameDispatchCase::V1(inner), ServerNameName::HostName(v0)) => {
                inner.length(&(*v0))
            }
            _ => panic!("dispatch branch combinator does not match value"),
        }
    }
    fn parse<'p>(&self, s: &'p [u8]) -> Result<(usize, Self::Type<'p>), ParseError>
    where
        [u8]: 'p,
    {
        match self {
            ServerNameNameDispatchCase::V1(inner) => {
                let (n, v) = inner.parse(s)?;
                Ok((n, ServerNameName::HostName(v)))
            }
            ServerNameNameDispatchCase::V2(inner) => {
                let (n, v) = inner.parse(s)?;
                Ok((n, ServerNameName::__default(v)))
            }
        }
    }
    fn serialize<'s>(
        &self,
        v: Self::SType<'s>,
        data: &mut Vec<u8>,
        pos: usize,
    ) -> Result<usize, SerializeError>
    where
        [u8]: 's,
    {
        match (self, v) {
            (ServerNameNameDispatchCase::V1(inner), ServerNameName::HostName(v0)) => {
                inner.serialize(&(*v0), data, pos)
            }
            _ => {
                Err(
                    SerializeError::Other(
                        "dispatch branch combinator does not match value".into(),
                    ),
                )
            }
        }
    }
    fn serialize_gen(
        &self,
        v: Self::GType,
        data: &mut Vec<u8>,
        pos: usize,
    ) -> Result<usize, SerializeError> {
        match (self, v) {
            (
                ServerNameNameDispatchCase::V1(inner),
                ServerNameNameOwned::HostName(v0),
            ) => inner.serialize_gen(v0, data, pos),
            _ => {
                Err(
                    SerializeError::Other(
                        "dispatch branch combinator does not match value".into(),
                    ),
                )
            }
        }
    }
    fn generate(&mut self, g: &mut GenSt) -> GResult<Self::GType, GenerateError> {
        match self {
            ServerNameNameDispatchCase::V1(inner) => {
                let (n, v) = inner.generate(g)?;
                Ok((n, ServerNameNameOwned::HostName(v)))
            }
            ServerNameNameDispatchCase::V2(inner) => {
                let (n, v) = inner.generate(g)?;
                Ok((n, ServerNameNameOwned::__default(v)))
            }
        }
    }
    fn well_formed<'s>(&self, v: Self::SType<'s>) -> bool
    where
        [u8]: 's,
    {
        match (self, v) {
            (ServerNameNameDispatchCase::V1(inner), ServerNameName::HostName(v0)) => {
                inner.well_formed(&(*v0))
            }
            _ => false,
        }
    }
}

#[derive(Clone, Copy)]
pub struct ServerNameDep {}
impl DepCombinator<NameTypeCombinator, [u8], Vec<u8>> for ServerNameDep {
    type Out = ServerNameNameCombinator;
    type OutGen<'g> = ServerNameNameCombinator<ServerNameNameCombinatorAlias<'g>>;
    fn dep_snd<'s>(&self, fst: NameType) -> Self::Out {
        let fst: NameType = fst;
        let name_type = fst;
        server_name_name(name_type)
    }
    fn dep_snd_gen<'g>(&self, fst: &'g mut NameType) -> Self::OutGen<'g> {
        let fst: &'g mut NameType = fst;
        let name_type = fst;
        server_name_name(name_type)
    }
}

#[derive(Clone, Copy)]
pub struct ServerNameListDep {}
impl DepCombinator<Refined<U16Be, fn(u16) -> bool>, [u8], Vec<u8>>
for ServerNameListDep {
    type Out = FixedLen<'static, Repeat<ServerNameCombinator>>;
    type OutGen<'g> = FixedLen<'g, Repeat<ServerNameCombinator>>;
    fn dep_snd<'s>(&self, fst: u16) -> Self::Out {
        let fst: u16 = fst;
        let l = fst;
        FixedLen(Length::from_value(l as usize), Repeat::new(server_name()))
    }
    fn dep_snd_gen<'g>(&self, fst: &'g mut u16) -> Self::OutGen<'g> {
        let fst: &'g mut u16 = fst;
        let l = fst;
        FixedLen(Length::from_u16_mut(l), Repeat::new(server_name()))
    }
}

#[derive(Clone, Copy)]
pub struct Opaque0FfffDep {}
impl DepCombinator<U16Be, [u8], Vec<u8>> for Opaque0FfffDep {
    type Out = Variable;
    type OutGen<'g> = Variable;
    fn dep_snd<'s>(&self, fst: u16) -> Self::Out {
        let fst: u16 = fst;
        let l = fst;
        Variable(l as usize)
    }
    fn dep_snd_gen<'g>(&self, fst: &'g mut u16) -> Self::OutGen<'g> {
        let fst: &'g mut u16 = fst;
        let l = fst;
        Variable(*l as usize)
    }
}

#[derive(Clone, Copy)]
pub struct ResponderIdListDep {}
impl DepCombinator<U16Be, [u8], Vec<u8>> for ResponderIdListDep {
    type Out = FixedLen<'static, Repeat<ResponderIdCombinator>>;
    type OutGen<'g> = FixedLen<'g, Repeat<ResponderIdCombinator>>;
    fn dep_snd<'s>(&self, fst: u16) -> Self::Out {
        let fst: u16 = fst;
        let l = fst;
        FixedLen(Length::from_value(l as usize), Repeat::new(responder_id()))
    }
    fn dep_snd_gen<'g>(&self, fst: &'g mut u16) -> Self::OutGen<'g> {
        let fst: &'g mut u16 = fst;
        let l = fst;
        FixedLen(Length::from_u16_mut(l), Repeat::new(responder_id()))
    }
}

#[derive(Clone, Copy)]
pub struct SrtpProtectionProfilesDep {}
impl DepCombinator<Refined<U16Be, fn(u16) -> bool>, [u8], Vec<u8>>
for SrtpProtectionProfilesDep {
    type Out = FixedLen<'static, Repeat<SrtpProtectionProfileCombinator>>;
    type OutGen<'g> = FixedLen<'g, Repeat<SrtpProtectionProfileCombinator>>;
    fn dep_snd<'s>(&self, fst: u16) -> Self::Out {
        let fst: u16 = fst;
        let l = fst;
        FixedLen(Length::from_value(l as usize), Repeat::new(srtp_protection_profile()))
    }
    fn dep_snd_gen<'g>(&self, fst: &'g mut u16) -> Self::OutGen<'g> {
        let fst: &'g mut u16 = fst;
        let l = fst;
        FixedLen(Length::from_u16_mut(l), Repeat::new(srtp_protection_profile()))
    }
}

#[derive(Clone, Copy)]
pub struct Opaque0FfDep {}
impl DepCombinator<U8, [u8], Vec<u8>> for Opaque0FfDep {
    type Out = Variable;
    type OutGen<'g> = Variable;
    fn dep_snd<'s>(&self, fst: u8) -> Self::Out {
        let fst: u8 = fst;
        let l = fst;
        Variable(l as usize)
    }
    fn dep_snd_gen<'g>(&self, fst: &'g mut u8) -> Self::OutGen<'g> {
        let fst: &'g mut u8 = fst;
        let l = fst;
        Variable(*l as usize)
    }
}

#[derive(Clone, Copy)]
pub struct PaddingExtensionDep {}
impl DepCombinator<U16Be, [u8], Vec<u8>> for PaddingExtensionDep {
    type Out = FixedLen<'static, Repeat<ZeroByteCombinator>>;
    type OutGen<'g> = FixedLen<'g, Repeat<ZeroByteCombinator>>;
    fn dep_snd<'s>(&self, fst: u16) -> Self::Out {
        let fst: u16 = fst;
        let l = fst;
        FixedLen(Length::from_value(l as usize), Repeat::new(zero_byte()))
    }
    fn dep_snd_gen<'g>(&self, fst: &'g mut u16) -> Self::OutGen<'g> {
        let fst: &'g mut u16 = fst;
        let l = fst;
        FixedLen(Length::from_u16_mut(l), Repeat::new(zero_byte()))
    }
}

#[derive(Clone, Copy)]
pub struct SignatureSchemeListDep {}
impl DepCombinator<Refined<U16Be, fn(u16) -> bool>, [u8], Vec<u8>>
for SignatureSchemeListDep {
    type Out = FixedLen<'static, Repeat<SignatureSchemeCombinator>>;
    type OutGen<'g> = FixedLen<'g, Repeat<SignatureSchemeCombinator>>;
    fn dep_snd<'s>(&self, fst: u16) -> Self::Out {
        let fst: u16 = fst;
        let l = fst;
        FixedLen(Length::from_value(l as usize), Repeat::new(signature_scheme()))
    }
    fn dep_snd_gen<'g>(&self, fst: &'g mut u16) -> Self::OutGen<'g> {
        let fst: &'g mut u16 = fst;
        let l = fst;
        FixedLen(Length::from_u16_mut(l), Repeat::new(signature_scheme()))
    }
}

#[derive(Clone, Copy)]
pub struct ServerCertTypeClientExtensionDep {}
impl DepCombinator<Refined<U8, fn(u8) -> bool>, [u8], Vec<u8>>
for ServerCertTypeClientExtensionDep {
    type Out = FixedLen<'static, Repeat<CertificateTypeCombinator>>;
    type OutGen<'g> = FixedLen<'g, Repeat<CertificateTypeCombinator>>;
    fn dep_snd<'s>(&self, fst: u8) -> Self::Out {
        let fst: u8 = fst;
        let l = fst;
        FixedLen(Length::from_value(l as usize), Repeat::new(certificate_type()))
    }
    fn dep_snd_gen<'g>(&self, fst: &'g mut u8) -> Self::OutGen<'g> {
        let fst: &'g mut u8 = fst;
        let l = fst;
        FixedLen(Length::from_u8_mut(l), Repeat::new(certificate_type()))
    }
}

#[derive(Clone, Copy)]
pub struct SignedCertificateTimestampListDep {}
impl DepCombinator<Refined<U16Be, fn(u16) -> bool>, [u8], Vec<u8>>
for SignedCertificateTimestampListDep {
    type Out = FixedLen<'static, Repeat<SerializedSctCombinator>>;
    type OutGen<'g> = FixedLen<'g, Repeat<SerializedSctCombinator>>;
    fn dep_snd<'s>(&self, fst: u16) -> Self::Out {
        let fst: u16 = fst;
        let l = fst;
        FixedLen(Length::from_value(l as usize), Repeat::new(serialized_sct()))
    }
    fn dep_snd_gen<'g>(&self, fst: &'g mut u16) -> Self::OutGen<'g> {
        let fst: &'g mut u16 = fst;
        let l = fst;
        FixedLen(Length::from_u16_mut(l), Repeat::new(serialized_sct()))
    }
}

#[derive(Clone, Copy)]
pub struct Opaque2FfffDep {}
impl DepCombinator<Refined<U16Be, fn(u16) -> bool>, [u8], Vec<u8>> for Opaque2FfffDep {
    type Out = Variable;
    type OutGen<'g> = Variable;
    fn dep_snd<'s>(&self, fst: u16) -> Self::Out {
        let fst: u16 = fst;
        let l = fst;
        Variable(l as usize)
    }
    fn dep_snd_gen<'g>(&self, fst: &'g mut u16) -> Self::OutGen<'g> {
        let fst: &'g mut u16 = fst;
        let l = fst;
        Variable(*l as usize)
    }
}

#[derive(Clone, Copy)]
pub struct NamedGroupListDep {}
impl DepCombinator<Refined<U16Be, fn(u16) -> bool>, [u8], Vec<u8>>
for NamedGroupListDep {
    type Out = FixedLen<'static, Repeat<NamedGroupCombinator>>;
    type OutGen<'g> = FixedLen<'g, Repeat<NamedGroupCombinator>>;
    fn dep_snd<'s>(&self, fst: u16) -> Self::Out {
        let fst: u16 = fst;
        let l = fst;
        FixedLen(Length::from_value(l as usize), Repeat::new(named_group()))
    }
    fn dep_snd_gen<'g>(&self, fst: &'g mut u16) -> Self::OutGen<'g> {
        let fst: &'g mut u16 = fst;
        let l = fst;
        FixedLen(Length::from_u16_mut(l), Repeat::new(named_group()))
    }
}

#[derive(Clone, Copy)]
pub struct Opaque0FfffffDep {}
impl DepCombinator<U24Be, [u8], Vec<u8>> for Opaque0FfffffDep {
    type Out = Variable;
    type OutGen<'g> = Variable;
    fn dep_snd<'s>(&self, fst: u24) -> Self::Out {
        let fst: u24 = fst;
        let l = fst;
        Variable((l).as_u32() as usize)
    }
    fn dep_snd_gen<'g>(&self, fst: &'g mut u24) -> Self::OutGen<'g> {
        let fst: &'g mut u24 = fst;
        let l = fst;
        Variable((*l).as_u32() as usize)
    }
}

#[derive(Clone, Copy)]
pub struct ClientCertTypeClientExtensionDep {}
impl DepCombinator<Refined<U8, fn(u8) -> bool>, [u8], Vec<u8>>
for ClientCertTypeClientExtensionDep {
    type Out = FixedLen<'static, Repeat<CertificateTypeCombinator>>;
    type OutGen<'g> = FixedLen<'g, Repeat<CertificateTypeCombinator>>;
    fn dep_snd<'s>(&self, fst: u8) -> Self::Out {
        let fst: u8 = fst;
        let l = fst;
        FixedLen(Length::from_value(l as usize), Repeat::new(certificate_type()))
    }
    fn dep_snd_gen<'g>(&self, fst: &'g mut u8) -> Self::OutGen<'g> {
        let fst: &'g mut u8 = fst;
        let l = fst;
        FixedLen(Length::from_u8_mut(l), Repeat::new(certificate_type()))
    }
}

#[derive(Clone, Copy)]
pub struct Opaque1FfffffDep {}
impl DepCombinator<Refined<U24Be, fn(u24) -> bool>, [u8], Vec<u8>> for Opaque1FfffffDep {
    type Out = Variable;
    type OutGen<'g> = Variable;
    fn dep_snd<'s>(&self, fst: u24) -> Self::Out {
        let fst: u24 = fst;
        let l = fst;
        Variable((l).as_u32() as usize)
    }
    fn dep_snd_gen<'g>(&self, fst: &'g mut u24) -> Self::OutGen<'g> {
        let fst: &'g mut u24 = fst;
        let l = fst;
        Variable((*l).as_u32() as usize)
    }
}

#[derive(Clone, Copy)]
pub struct EcPointFormatListDep {}
impl DepCombinator<Refined<U8, fn(u8) -> bool>, [u8], Vec<u8>> for EcPointFormatListDep {
    type Out = FixedLen<'static, Repeat<EcPointFormatCombinator>>;
    type OutGen<'g> = FixedLen<'g, Repeat<EcPointFormatCombinator>>;
    fn dep_snd<'s>(&self, fst: u8) -> Self::Out {
        let fst: u8 = fst;
        let l = fst;
        FixedLen(Length::from_value(l as usize), Repeat::new(ec_point_format()))
    }
    fn dep_snd_gen<'g>(&self, fst: &'g mut u8) -> Self::OutGen<'g> {
        let fst: &'g mut u8 = fst;
        let l = fst;
        FixedLen(Length::from_u8_mut(l), Repeat::new(ec_point_format()))
    }
}

#[derive(Clone, Copy)]
pub struct ProtocolNameListDep {}
impl DepCombinator<Refined<U16Be, fn(u16) -> bool>, [u8], Vec<u8>>
for ProtocolNameListDep {
    type Out = FixedLen<'static, Repeat<ProtocolNameCombinator>>;
    type OutGen<'g> = FixedLen<'g, Repeat<ProtocolNameCombinator>>;
    fn dep_snd<'s>(&self, fst: u16) -> Self::Out {
        let fst: u16 = fst;
        let l = fst;
        FixedLen(Length::from_value(l as usize), Repeat::new(protocol_name()))
    }
    fn dep_snd_gen<'g>(&self, fst: &'g mut u16) -> Self::OutGen<'g> {
        let fst: &'g mut u16 = fst;
        let l = fst;
        FixedLen(Length::from_u16_mut(l), Repeat::new(protocol_name()))
    }
}

///Parse function for certificate_type combinator
pub fn parse_certificate_type<'p>(
    input: &'p [u8],
) -> Result<(usize, CertificateType), ParseError> {
    let combinator = certificate_type();
    combinator.parse(input)
}

///Serialize function for certificate_type combinator
pub fn serialize_certificate_type<'s>(
    v: CertificateType,
    data: &mut Vec<u8>,
    pos: usize,
) -> Result<usize, SerializeError> {
    let combinator = certificate_type();
    combinator.serialize(v, data, pos)
}

///Serialize function for certificate_type combinator (owned version)
pub fn serialize_gen_certificate_type(
    v: CertificateType,
    data: &mut Vec<u8>,
    pos: usize,
) -> Result<usize, SerializeError> {
    let combinator = certificate_type();
    combinator.serialize_gen(v, data, pos)
}

///Length function for certificate_type combinator
pub fn certificate_type_len<'s>(v: CertificateType) -> usize {
    let combinator = certificate_type();
    combinator.length(v)
}

///Generate function for certificate_type combinator
pub fn generate_certificate_type(
    g: &mut GenSt,
) -> GResult<CertificateType, GenerateError> {
    let mut combinator = certificate_type();
    combinator.generate(g)
}

///Parse function for server_cert_type_server_extension combinator
pub fn parse_server_cert_type_server_extension<'p>(
    input: &'p [u8],
) -> Result<(usize, ServerCertTypeServerExtension), ParseError> {
    let combinator = server_cert_type_server_extension();
    combinator.parse(input)
}

///Serialize function for server_cert_type_server_extension combinator
pub fn serialize_server_cert_type_server_extension<'s>(
    v: ServerCertTypeServerExtension,
    data: &mut Vec<u8>,
    pos: usize,
) -> Result<usize, SerializeError> {
    let combinator = server_cert_type_server_extension();
    combinator.serialize(v, data, pos)
}

///Serialize function for server_cert_type_server_extension combinator (owned version)
pub fn serialize_gen_server_cert_type_server_extension(
    v: ServerCertTypeServerExtension,
    data: &mut Vec<u8>,
    pos: usize,
) -> Result<usize, SerializeError> {
    let combinator = server_cert_type_server_extension();
    combinator.serialize_gen(v, data, pos)
}

///Length function for server_cert_type_server_extension combinator
pub fn server_cert_type_server_extension_len<'s>(
    v: ServerCertTypeServerExtension,
) -> usize {
    let combinator = server_cert_type_server_extension();
    combinator.length(v)
}

///Generate function for server_cert_type_server_extension combinator
pub fn generate_server_cert_type_server_extension(
    g: &mut GenSt,
) -> GResult<ServerCertTypeServerExtension, GenerateError> {
    let mut combinator = server_cert_type_server_extension();
    combinator.generate(g)
}

///Parse function for opaque_1_ff combinator
pub fn parse_opaque_1_ff<'p>(
    input: &'p [u8],
) -> Result<(usize, Opaque1Ff<'p>), ParseError> {
    let combinator = opaque_1_ff();
    combinator.parse(input)
}

///Serialize function for opaque_1_ff combinator
pub fn serialize_opaque_1_ff<'s>(
    v: &'s Opaque1Ff<'s>,
    data: &mut Vec<u8>,
    pos: usize,
) -> Result<usize, SerializeError> {
    let combinator = opaque_1_ff();
    combinator.serialize(v, data, pos)
}

///Serialize function for opaque_1_ff combinator (owned version)
pub fn serialize_gen_opaque_1_ff(
    v: Opaque1FfOwned,
    data: &mut Vec<u8>,
    pos: usize,
) -> Result<usize, SerializeError> {
    let combinator = opaque_1_ff();
    combinator.serialize_gen(v, data, pos)
}

///Length function for opaque_1_ff combinator
pub fn opaque_1_ff_len<'s>(v: &'s Opaque1Ff<'s>) -> usize {
    let combinator = opaque_1_ff();
    combinator.length(v)
}

///Generate function for opaque_1_ff combinator
pub fn generate_opaque_1_ff(g: &mut GenSt) -> GResult<Opaque1FfOwned, GenerateError> {
    let mut combinator = opaque_1_ff();
    combinator.generate(g)
}

///Parse function for opaque_1_ffff combinator
pub fn parse_opaque_1_ffff<'p>(
    input: &'p [u8],
) -> Result<(usize, Opaque1Ffff<'p>), ParseError> {
    let combinator = opaque_1_ffff();
    combinator.parse(input)
}

///Serialize function for opaque_1_ffff combinator
pub fn serialize_opaque_1_ffff<'s>(
    v: &'s Opaque1Ffff<'s>,
    data: &mut Vec<u8>,
    pos: usize,
) -> Result<usize, SerializeError> {
    let combinator = opaque_1_ffff();
    combinator.serialize(v, data, pos)
}

///Serialize function for opaque_1_ffff combinator (owned version)
pub fn serialize_gen_opaque_1_ffff(
    v: Opaque1FfffOwned,
    data: &mut Vec<u8>,
    pos: usize,
) -> Result<usize, SerializeError> {
    let combinator = opaque_1_ffff();
    combinator.serialize_gen(v, data, pos)
}

///Length function for opaque_1_ffff combinator
pub fn opaque_1_ffff_len<'s>(v: &'s Opaque1Ffff<'s>) -> usize {
    let combinator = opaque_1_ffff();
    combinator.length(v)
}

///Generate function for opaque_1_ffff combinator
pub fn generate_opaque_1_ffff(
    g: &mut GenSt,
) -> GResult<Opaque1FfffOwned, GenerateError> {
    let mut combinator = opaque_1_ffff();
    combinator.generate(g)
}

///Parse function for name_type combinator
pub fn parse_name_type<'p>(input: &'p [u8]) -> Result<(usize, NameType), ParseError> {
    let combinator = name_type();
    combinator.parse(input)
}

///Serialize function for name_type combinator
pub fn serialize_name_type<'s>(
    v: NameType,
    data: &mut Vec<u8>,
    pos: usize,
) -> Result<usize, SerializeError> {
    let combinator = name_type();
    combinator.serialize(v, data, pos)
}

///Serialize function for name_type combinator (owned version)
pub fn serialize_gen_name_type(
    v: NameType,
    data: &mut Vec<u8>,
    pos: usize,
) -> Result<usize, SerializeError> {
    let combinator = name_type();
    combinator.serialize_gen(v, data, pos)
}

///Length function for name_type combinator
pub fn name_type_len<'s>(v: NameType) -> usize {
    let combinator = name_type();
    combinator.length(v)
}

///Generate function for name_type combinator
pub fn generate_name_type(g: &mut GenSt) -> GResult<NameType, GenerateError> {
    let mut combinator = name_type();
    combinator.generate(g)
}

///Parse function for host_name combinator
pub fn parse_host_name<'p>(
    input: &'p [u8],
) -> Result<(usize, HostName<'p>), ParseError> {
    let combinator = host_name();
    combinator.parse(input)
}

///Serialize function for host_name combinator
pub fn serialize_host_name<'s>(
    v: &'s Opaque1Ffff<'s>,
    data: &mut Vec<u8>,
    pos: usize,
) -> Result<usize, SerializeError> {
    let combinator = host_name();
    combinator.serialize(v, data, pos)
}

///Serialize function for host_name combinator (owned version)
pub fn serialize_gen_host_name(
    v: HostNameOwned,
    data: &mut Vec<u8>,
    pos: usize,
) -> Result<usize, SerializeError> {
    let combinator = host_name();
    combinator.serialize_gen(v, data, pos)
}

///Length function for host_name combinator
pub fn host_name_len<'s>(v: &'s Opaque1Ffff<'s>) -> usize {
    let combinator = host_name();
    combinator.length(v)
}

///Generate function for host_name combinator
pub fn generate_host_name(g: &mut GenSt) -> GResult<HostNameOwned, GenerateError> {
    let mut combinator = host_name();
    combinator.generate(g)
}

///Parse function for unknown_name combinator
pub fn parse_unknown_name<'p>(
    input: &'p [u8],
) -> Result<(usize, UnknownName<'p>), ParseError> {
    let combinator = unknown_name();
    combinator.parse(input)
}

///Serialize function for unknown_name combinator
pub fn serialize_unknown_name<'s>(
    v: &'s Opaque1Ffff<'s>,
    data: &mut Vec<u8>,
    pos: usize,
) -> Result<usize, SerializeError> {
    let combinator = unknown_name();
    combinator.serialize(v, data, pos)
}

///Serialize function for unknown_name combinator (owned version)
pub fn serialize_gen_unknown_name(
    v: UnknownNameOwned,
    data: &mut Vec<u8>,
    pos: usize,
) -> Result<usize, SerializeError> {
    let combinator = unknown_name();
    combinator.serialize_gen(v, data, pos)
}

///Length function for unknown_name combinator
pub fn unknown_name_len<'s>(v: &'s Opaque1Ffff<'s>) -> usize {
    let combinator = unknown_name();
    combinator.length(v)
}

///Generate function for unknown_name combinator
pub fn generate_unknown_name(g: &mut GenSt) -> GResult<UnknownNameOwned, GenerateError> {
    let mut combinator = unknown_name();
    combinator.generate(g)
}

///Parse function for server_name_name combinator
pub fn parse_server_name_name<'p>(
    input: &'p [u8],
    name_type: NameType,
) -> Result<(usize, ServerNameName<'p>), ParseError> {
    let combinator = server_name_name(name_type);
    combinator.parse(input)
}

///Serialize function for server_name_name combinator
pub fn serialize_server_name_name<'s>(
    v: &'s ServerNameName<'s>,
    data: &mut Vec<u8>,
    pos: usize,
    name_type: NameType,
) -> Result<usize, SerializeError> {
    let combinator = server_name_name(name_type);
    combinator.serialize(v, data, pos)
}

///Serialize function for server_name_name combinator (owned version)
pub fn serialize_gen_server_name_name(
    v: ServerNameNameOwned,
    data: &mut Vec<u8>,
    pos: usize,
    name_type: NameType,
) -> Result<usize, SerializeError> {
    let combinator = server_name_name(name_type);
    combinator.serialize_gen(v, data, pos)
}

///Length function for server_name_name combinator
pub fn server_name_name_len<'s>(
    v: &'s ServerNameName<'s>,
    name_type: NameType,
) -> usize {
    let combinator = server_name_name(name_type);
    combinator.length(v)
}

///Generate function for server_name_name combinator
pub fn generate_server_name_name<'g>(
    g: &mut GenSt,
    name_type: &'g mut NameType,
) -> GResult<ServerNameNameOwned, GenerateError> {
    let mut combinator = server_name_name(name_type);
    combinator.generate(g)
}

///Parse function for server_name combinator
pub fn parse_server_name<'p>(
    input: &'p [u8],
) -> Result<(usize, ServerName<'p>), ParseError> {
    let combinator = server_name();
    combinator.parse(input)
}

///Serialize function for server_name combinator
pub fn serialize_server_name<'s>(
    v: &'s ServerName<'s>,
    data: &mut Vec<u8>,
    pos: usize,
) -> Result<usize, SerializeError> {
    let combinator = server_name();
    combinator.serialize(v, data, pos)
}

///Serialize function for server_name combinator (owned version)
pub fn serialize_gen_server_name(
    v: ServerNameOwned,
    data: &mut Vec<u8>,
    pos: usize,
) -> Result<usize, SerializeError> {
    let combinator = server_name();
    combinator.serialize_gen(v, data, pos)
}

///Length function for server_name combinator
pub fn server_name_len<'s>(v: &'s ServerName<'s>) -> usize {
    let combinator = server_name();
    combinator.length(v)
}

///Generate function for server_name combinator
pub fn generate_server_name(g: &mut GenSt) -> GResult<ServerNameOwned, GenerateError> {
    let mut combinator = server_name();
    combinator.generate(g)
}

///Parse function for server_name_list combinator
pub fn parse_server_name_list<'p>(
    input: &'p [u8],
) -> Result<(usize, ServerNameList<'p>), ParseError> {
    let combinator = server_name_list();
    combinator.parse(input)
}

///Serialize function for server_name_list combinator
pub fn serialize_server_name_list<'s>(
    v: &'s ServerNameList<'s>,
    data: &mut Vec<u8>,
    pos: usize,
) -> Result<usize, SerializeError> {
    let combinator = server_name_list();
    combinator.serialize(v, data, pos)
}

///Serialize function for server_name_list combinator (owned version)
pub fn serialize_gen_server_name_list(
    v: ServerNameListOwned,
    data: &mut Vec<u8>,
    pos: usize,
) -> Result<usize, SerializeError> {
    let combinator = server_name_list();
    combinator.serialize_gen(v, data, pos)
}

///Length function for server_name_list combinator
pub fn server_name_list_len<'s>(v: &'s ServerNameList<'s>) -> usize {
    let combinator = server_name_list();
    combinator.length(v)
}

///Generate function for server_name_list combinator
pub fn generate_server_name_list(
    g: &mut GenSt,
) -> GResult<ServerNameListOwned, GenerateError> {
    let mut combinator = server_name_list();
    combinator.generate(g)
}

///Parse function for opaque_0_ffff combinator
pub fn parse_opaque_0_ffff<'p>(
    input: &'p [u8],
) -> Result<(usize, Opaque0Ffff<'p>), ParseError> {
    let combinator = opaque_0_ffff();
    combinator.parse(input)
}

///Serialize function for opaque_0_ffff combinator
pub fn serialize_opaque_0_ffff<'s>(
    v: &'s Opaque0Ffff<'s>,
    data: &mut Vec<u8>,
    pos: usize,
) -> Result<usize, SerializeError> {
    let combinator = opaque_0_ffff();
    combinator.serialize(v, data, pos)
}

///Serialize function for opaque_0_ffff combinator (owned version)
pub fn serialize_gen_opaque_0_ffff(
    v: Opaque0FfffOwned,
    data: &mut Vec<u8>,
    pos: usize,
) -> Result<usize, SerializeError> {
    let combinator = opaque_0_ffff();
    combinator.serialize_gen(v, data, pos)
}

///Length function for opaque_0_ffff combinator
pub fn opaque_0_ffff_len<'s>(v: &'s Opaque0Ffff<'s>) -> usize {
    let combinator = opaque_0_ffff();
    combinator.length(v)
}

///Generate function for opaque_0_ffff combinator
pub fn generate_opaque_0_ffff(
    g: &mut GenSt,
) -> GResult<Opaque0FfffOwned, GenerateError> {
    let mut combinator = opaque_0_ffff();
    combinator.generate(g)
}

///Parse function for ocsp_extensions combinator
pub fn parse_ocsp_extensions<'p>(
    input: &'p [u8],
) -> Result<(usize, OcspExtensions<'p>), ParseError> {
    let combinator = ocsp_extensions();
    combinator.parse(input)
}

///Serialize function for ocsp_extensions combinator
pub fn serialize_ocsp_extensions<'s>(
    v: &'s Opaque0Ffff<'s>,
    data: &mut Vec<u8>,
    pos: usize,
) -> Result<usize, SerializeError> {
    let combinator = ocsp_extensions();
    combinator.serialize(v, data, pos)
}

///Serialize function for ocsp_extensions combinator (owned version)
pub fn serialize_gen_ocsp_extensions(
    v: OcspExtensionsOwned,
    data: &mut Vec<u8>,
    pos: usize,
) -> Result<usize, SerializeError> {
    let combinator = ocsp_extensions();
    combinator.serialize_gen(v, data, pos)
}

///Length function for ocsp_extensions combinator
pub fn ocsp_extensions_len<'s>(v: &'s Opaque0Ffff<'s>) -> usize {
    let combinator = ocsp_extensions();
    combinator.length(v)
}

///Generate function for ocsp_extensions combinator
pub fn generate_ocsp_extensions(
    g: &mut GenSt,
) -> GResult<OcspExtensionsOwned, GenerateError> {
    let mut combinator = ocsp_extensions();
    combinator.generate(g)
}

///Parse function for responder_id combinator
pub fn parse_responder_id<'p>(
    input: &'p [u8],
) -> Result<(usize, ResponderId<'p>), ParseError> {
    let combinator = responder_id();
    combinator.parse(input)
}

///Serialize function for responder_id combinator
pub fn serialize_responder_id<'s>(
    v: &'s Opaque1Ffff<'s>,
    data: &mut Vec<u8>,
    pos: usize,
) -> Result<usize, SerializeError> {
    let combinator = responder_id();
    combinator.serialize(v, data, pos)
}

///Serialize function for responder_id combinator (owned version)
pub fn serialize_gen_responder_id(
    v: ResponderIdOwned,
    data: &mut Vec<u8>,
    pos: usize,
) -> Result<usize, SerializeError> {
    let combinator = responder_id();
    combinator.serialize_gen(v, data, pos)
}

///Length function for responder_id combinator
pub fn responder_id_len<'s>(v: &'s Opaque1Ffff<'s>) -> usize {
    let combinator = responder_id();
    combinator.length(v)
}

///Generate function for responder_id combinator
pub fn generate_responder_id(g: &mut GenSt) -> GResult<ResponderIdOwned, GenerateError> {
    let mut combinator = responder_id();
    combinator.generate(g)
}

///Parse function for responder_id_list combinator
pub fn parse_responder_id_list<'p>(
    input: &'p [u8],
) -> Result<(usize, ResponderIdList<'p>), ParseError> {
    let combinator = responder_id_list();
    combinator.parse(input)
}

///Serialize function for responder_id_list combinator
pub fn serialize_responder_id_list<'s>(
    v: &'s ResponderIdList<'s>,
    data: &mut Vec<u8>,
    pos: usize,
) -> Result<usize, SerializeError> {
    let combinator = responder_id_list();
    combinator.serialize(v, data, pos)
}

///Serialize function for responder_id_list combinator (owned version)
pub fn serialize_gen_responder_id_list(
    v: ResponderIdListOwned,
    data: &mut Vec<u8>,
    pos: usize,
) -> Result<usize, SerializeError> {
    let combinator = responder_id_list();
    combinator.serialize_gen(v, data, pos)
}

///Length function for responder_id_list combinator
pub fn responder_id_list_len<'s>(v: &'s ResponderIdList<'s>) -> usize {
    let combinator = responder_id_list();
    combinator.length(v)
}

///Generate function for responder_id_list combinator
pub fn generate_responder_id_list(
    g: &mut GenSt,
) -> GResult<ResponderIdListOwned, GenerateError> {
    let mut combinator = responder_id_list();
    combinator.generate(g)
}

///Parse function for oscp_status_request combinator
pub fn parse_oscp_status_request<'p>(
    input: &'p [u8],
) -> Result<(usize, OscpStatusRequest<'p>), ParseError> {
    let combinator = oscp_status_request();
    combinator.parse(input)
}

///Serialize function for oscp_status_request combinator
pub fn serialize_oscp_status_request<'s>(
    v: &'s OscpStatusRequest<'s>,
    data: &mut Vec<u8>,
    pos: usize,
) -> Result<usize, SerializeError> {
    let combinator = oscp_status_request();
    combinator.serialize(v, data, pos)
}

///Serialize function for oscp_status_request combinator (owned version)
pub fn serialize_gen_oscp_status_request(
    v: OscpStatusRequestOwned,
    data: &mut Vec<u8>,
    pos: usize,
) -> Result<usize, SerializeError> {
    let combinator = oscp_status_request();
    combinator.serialize_gen(v, data, pos)
}

///Length function for oscp_status_request combinator
pub fn oscp_status_request_len<'s>(v: &'s OscpStatusRequest<'s>) -> usize {
    let combinator = oscp_status_request();
    combinator.length(v)
}

///Generate function for oscp_status_request combinator
pub fn generate_oscp_status_request(
    g: &mut GenSt,
) -> GResult<OscpStatusRequestOwned, GenerateError> {
    let mut combinator = oscp_status_request();
    combinator.generate(g)
}

///Parse function for srtp_protection_profile combinator
pub fn parse_srtp_protection_profile<'p>(
    input: &'p [u8],
) -> Result<(usize, SrtpProtectionProfile<'p>), ParseError> {
    let combinator = srtp_protection_profile();
    combinator.parse(input)
}

///Serialize function for srtp_protection_profile combinator
pub fn serialize_srtp_protection_profile<'s>(
    v: &'s [u8],
    data: &mut Vec<u8>,
    pos: usize,
) -> Result<usize, SerializeError> {
    let combinator = srtp_protection_profile();
    combinator.serialize(v, data, pos)
}

///Serialize function for srtp_protection_profile combinator (owned version)
pub fn serialize_gen_srtp_protection_profile(
    v: SrtpProtectionProfileOwned,
    data: &mut Vec<u8>,
    pos: usize,
) -> Result<usize, SerializeError> {
    let combinator = srtp_protection_profile();
    combinator.serialize_gen(v, data, pos)
}

///Length function for srtp_protection_profile combinator
pub fn srtp_protection_profile_len<'s>(v: &'s [u8]) -> usize {
    let combinator = srtp_protection_profile();
    combinator.length(v)
}

///Generate function for srtp_protection_profile combinator
pub fn generate_srtp_protection_profile(
    g: &mut GenSt,
) -> GResult<SrtpProtectionProfileOwned, GenerateError> {
    let mut combinator = srtp_protection_profile();
    combinator.generate(g)
}

///Parse function for srtp_protection_profiles combinator
pub fn parse_srtp_protection_profiles<'p>(
    input: &'p [u8],
) -> Result<(usize, SrtpProtectionProfiles<'p>), ParseError> {
    let combinator = srtp_protection_profiles();
    combinator.parse(input)
}

///Serialize function for srtp_protection_profiles combinator
pub fn serialize_srtp_protection_profiles<'s>(
    v: &'s SrtpProtectionProfiles<'s>,
    data: &mut Vec<u8>,
    pos: usize,
) -> Result<usize, SerializeError> {
    let combinator = srtp_protection_profiles();
    combinator.serialize(v, data, pos)
}

///Serialize function for srtp_protection_profiles combinator (owned version)
pub fn serialize_gen_srtp_protection_profiles(
    v: SrtpProtectionProfilesOwned,
    data: &mut Vec<u8>,
    pos: usize,
) -> Result<usize, SerializeError> {
    let combinator = srtp_protection_profiles();
    combinator.serialize_gen(v, data, pos)
}

///Length function for srtp_protection_profiles combinator
pub fn srtp_protection_profiles_len<'s>(v: &'s SrtpProtectionProfiles<'s>) -> usize {
    let combinator = srtp_protection_profiles();
    combinator.length(v)
}

///Generate function for srtp_protection_profiles combinator
pub fn generate_srtp_protection_profiles(
    g: &mut GenSt,
) -> GResult<SrtpProtectionProfilesOwned, GenerateError> {
    let mut combinator = srtp_protection_profiles();
    combinator.generate(g)
}

///Parse function for opaque_0_ff combinator
pub fn parse_opaque_0_ff<'p>(
    input: &'p [u8],
) -> Result<(usize, Opaque0Ff<'p>), ParseError> {
    let combinator = opaque_0_ff();
    combinator.parse(input)
}

///Serialize function for opaque_0_ff combinator
pub fn serialize_opaque_0_ff<'s>(
    v: &'s Opaque0Ff<'s>,
    data: &mut Vec<u8>,
    pos: usize,
) -> Result<usize, SerializeError> {
    let combinator = opaque_0_ff();
    combinator.serialize(v, data, pos)
}

///Serialize function for opaque_0_ff combinator (owned version)
pub fn serialize_gen_opaque_0_ff(
    v: Opaque0FfOwned,
    data: &mut Vec<u8>,
    pos: usize,
) -> Result<usize, SerializeError> {
    let combinator = opaque_0_ff();
    combinator.serialize_gen(v, data, pos)
}

///Length function for opaque_0_ff combinator
pub fn opaque_0_ff_len<'s>(v: &'s Opaque0Ff<'s>) -> usize {
    let combinator = opaque_0_ff();
    combinator.length(v)
}

///Generate function for opaque_0_ff combinator
pub fn generate_opaque_0_ff(g: &mut GenSt) -> GResult<Opaque0FfOwned, GenerateError> {
    let mut combinator = opaque_0_ff();
    combinator.generate(g)
}

///Parse function for use_srtp_data combinator
pub fn parse_use_srtp_data<'p>(
    input: &'p [u8],
) -> Result<(usize, UseSrtpData<'p>), ParseError> {
    let combinator = use_srtp_data();
    combinator.parse(input)
}

///Serialize function for use_srtp_data combinator
pub fn serialize_use_srtp_data<'s>(
    v: &'s UseSrtpData<'s>,
    data: &mut Vec<u8>,
    pos: usize,
) -> Result<usize, SerializeError> {
    let combinator = use_srtp_data();
    combinator.serialize(v, data, pos)
}

///Serialize function for use_srtp_data combinator (owned version)
pub fn serialize_gen_use_srtp_data(
    v: UseSrtpDataOwned,
    data: &mut Vec<u8>,
    pos: usize,
) -> Result<usize, SerializeError> {
    let combinator = use_srtp_data();
    combinator.serialize_gen(v, data, pos)
}

///Length function for use_srtp_data combinator
pub fn use_srtp_data_len<'s>(v: &'s UseSrtpData<'s>) -> usize {
    let combinator = use_srtp_data();
    combinator.length(v)
}

///Generate function for use_srtp_data combinator
pub fn generate_use_srtp_data(
    g: &mut GenSt,
) -> GResult<UseSrtpDataOwned, GenerateError> {
    let mut combinator = use_srtp_data();
    combinator.generate(g)
}

///Parse function for max_fragment_length combinator
pub fn parse_max_fragment_length<'p>(
    input: &'p [u8],
) -> Result<(usize, MaxFragmentLength), ParseError> {
    let combinator = max_fragment_length();
    combinator.parse(input)
}

///Serialize function for max_fragment_length combinator
pub fn serialize_max_fragment_length<'s>(
    v: MaxFragmentLength,
    data: &mut Vec<u8>,
    pos: usize,
) -> Result<usize, SerializeError> {
    let combinator = max_fragment_length();
    combinator.serialize(v, data, pos)
}

///Serialize function for max_fragment_length combinator (owned version)
pub fn serialize_gen_max_fragment_length(
    v: MaxFragmentLength,
    data: &mut Vec<u8>,
    pos: usize,
) -> Result<usize, SerializeError> {
    let combinator = max_fragment_length();
    combinator.serialize_gen(v, data, pos)
}

///Length function for max_fragment_length combinator
pub fn max_fragment_length_len<'s>(v: MaxFragmentLength) -> usize {
    let combinator = max_fragment_length();
    combinator.length(v)
}

///Generate function for max_fragment_length combinator
pub fn generate_max_fragment_length(
    g: &mut GenSt,
) -> GResult<MaxFragmentLength, GenerateError> {
    let mut combinator = max_fragment_length();
    combinator.generate(g)
}

///Parse function for signature_scheme combinator
pub fn parse_signature_scheme<'p>(
    input: &'p [u8],
) -> Result<(usize, SignatureScheme), ParseError> {
    let combinator = signature_scheme();
    combinator.parse(input)
}

///Serialize function for signature_scheme combinator
pub fn serialize_signature_scheme<'s>(
    v: SignatureScheme,
    data: &mut Vec<u8>,
    pos: usize,
) -> Result<usize, SerializeError> {
    let combinator = signature_scheme();
    combinator.serialize(v, data, pos)
}

///Serialize function for signature_scheme combinator (owned version)
pub fn serialize_gen_signature_scheme(
    v: SignatureScheme,
    data: &mut Vec<u8>,
    pos: usize,
) -> Result<usize, SerializeError> {
    let combinator = signature_scheme();
    combinator.serialize_gen(v, data, pos)
}

///Length function for signature_scheme combinator
pub fn signature_scheme_len<'s>(v: SignatureScheme) -> usize {
    let combinator = signature_scheme();
    combinator.length(v)
}

///Generate function for signature_scheme combinator
pub fn generate_signature_scheme(
    g: &mut GenSt,
) -> GResult<SignatureScheme, GenerateError> {
    let mut combinator = signature_scheme();
    combinator.generate(g)
}

///Parse function for zero_byte combinator
pub fn parse_zero_byte<'p>(input: &'p [u8]) -> Result<(usize, ZeroByte), ParseError> {
    let combinator = zero_byte();
    combinator.parse(input)
}

///Serialize function for zero_byte combinator
pub fn serialize_zero_byte<'s>(
    v: ZeroByte,
    data: &mut Vec<u8>,
    pos: usize,
) -> Result<usize, SerializeError> {
    let combinator = zero_byte();
    combinator.serialize(v, data, pos)
}

///Serialize function for zero_byte combinator (owned version)
pub fn serialize_gen_zero_byte(
    v: ZeroByte,
    data: &mut Vec<u8>,
    pos: usize,
) -> Result<usize, SerializeError> {
    let combinator = zero_byte();
    combinator.serialize_gen(v, data, pos)
}

///Length function for zero_byte combinator
pub fn zero_byte_len<'s>(v: ZeroByte) -> usize {
    let combinator = zero_byte();
    combinator.length(v)
}

///Generate function for zero_byte combinator
pub fn generate_zero_byte(g: &mut GenSt) -> GResult<ZeroByte, GenerateError> {
    let mut combinator = zero_byte();
    combinator.generate(g)
}

///Parse function for serialized_sct combinator
pub fn parse_serialized_sct<'p>(
    input: &'p [u8],
) -> Result<(usize, SerializedSct<'p>), ParseError> {
    let combinator = serialized_sct();
    combinator.parse(input)
}

///Serialize function for serialized_sct combinator
pub fn serialize_serialized_sct<'s>(
    v: &'s Opaque1Ffff<'s>,
    data: &mut Vec<u8>,
    pos: usize,
) -> Result<usize, SerializeError> {
    let combinator = serialized_sct();
    combinator.serialize(v, data, pos)
}

///Serialize function for serialized_sct combinator (owned version)
pub fn serialize_gen_serialized_sct(
    v: SerializedSctOwned,
    data: &mut Vec<u8>,
    pos: usize,
) -> Result<usize, SerializeError> {
    let combinator = serialized_sct();
    combinator.serialize_gen(v, data, pos)
}

///Length function for serialized_sct combinator
pub fn serialized_sct_len<'s>(v: &'s Opaque1Ffff<'s>) -> usize {
    let combinator = serialized_sct();
    combinator.length(v)
}

///Generate function for serialized_sct combinator
pub fn generate_serialized_sct(
    g: &mut GenSt,
) -> GResult<SerializedSctOwned, GenerateError> {
    let mut combinator = serialized_sct();
    combinator.generate(g)
}

///Parse function for padding_extension combinator
pub fn parse_padding_extension<'p>(
    input: &'p [u8],
    ext_len: u16,
) -> Result<(usize, PaddingExtension), ParseError> {
    let combinator = padding_extension(ext_len);
    combinator.parse(input)
}

///Serialize function for padding_extension combinator
pub fn serialize_padding_extension<'s>(
    v: &'s PaddingExtension,
    data: &mut Vec<u8>,
    pos: usize,
    ext_len: u16,
) -> Result<usize, SerializeError> {
    let combinator = padding_extension(ext_len);
    combinator.serialize(v, data, pos)
}

///Serialize function for padding_extension combinator (owned version)
pub fn serialize_gen_padding_extension(
    v: PaddingExtension,
    data: &mut Vec<u8>,
    pos: usize,
    ext_len: u16,
) -> Result<usize, SerializeError> {
    let combinator = padding_extension(ext_len);
    combinator.serialize_gen(v, data, pos)
}

///Length function for padding_extension combinator
pub fn padding_extension_len<'s>(v: &'s PaddingExtension, ext_len: u16) -> usize {
    let combinator = padding_extension(ext_len);
    combinator.length(v)
}

///Generate function for padding_extension combinator
pub fn generate_padding_extension(
    g: &mut GenSt,
    ext_len: u16,
) -> GResult<PaddingExtension, GenerateError> {
    let mut combinator = padding_extension(ext_len);
    combinator.generate(g)
}

///Parse function for signature_scheme_list combinator
pub fn parse_signature_scheme_list<'p>(
    input: &'p [u8],
) -> Result<(usize, SignatureSchemeList), ParseError> {
    let combinator = signature_scheme_list();
    combinator.parse(input)
}

///Serialize function for signature_scheme_list combinator
pub fn serialize_signature_scheme_list<'s>(
    v: &'s SignatureSchemeList,
    data: &mut Vec<u8>,
    pos: usize,
) -> Result<usize, SerializeError> {
    let combinator = signature_scheme_list();
    combinator.serialize(v, data, pos)
}

///Serialize function for signature_scheme_list combinator (owned version)
pub fn serialize_gen_signature_scheme_list(
    v: SignatureSchemeList,
    data: &mut Vec<u8>,
    pos: usize,
) -> Result<usize, SerializeError> {
    let combinator = signature_scheme_list();
    combinator.serialize_gen(v, data, pos)
}

///Length function for signature_scheme_list combinator
pub fn signature_scheme_list_len<'s>(v: &'s SignatureSchemeList) -> usize {
    let combinator = signature_scheme_list();
    combinator.length(v)
}

///Generate function for signature_scheme_list combinator
pub fn generate_signature_scheme_list(
    g: &mut GenSt,
) -> GResult<SignatureSchemeList, GenerateError> {
    let mut combinator = signature_scheme_list();
    combinator.generate(g)
}

///Parse function for extension_type combinator
pub fn parse_extension_type<'p>(
    input: &'p [u8],
) -> Result<(usize, ExtensionType), ParseError> {
    let combinator = extension_type();
    combinator.parse(input)
}

///Serialize function for extension_type combinator
pub fn serialize_extension_type<'s>(
    v: ExtensionType,
    data: &mut Vec<u8>,
    pos: usize,
) -> Result<usize, SerializeError> {
    let combinator = extension_type();
    combinator.serialize(v, data, pos)
}

///Serialize function for extension_type combinator (owned version)
pub fn serialize_gen_extension_type(
    v: ExtensionType,
    data: &mut Vec<u8>,
    pos: usize,
) -> Result<usize, SerializeError> {
    let combinator = extension_type();
    combinator.serialize_gen(v, data, pos)
}

///Length function for extension_type combinator
pub fn extension_type_len<'s>(v: ExtensionType) -> usize {
    let combinator = extension_type();
    combinator.length(v)
}

///Generate function for extension_type combinator
pub fn generate_extension_type(g: &mut GenSt) -> GResult<ExtensionType, GenerateError> {
    let mut combinator = extension_type();
    combinator.generate(g)
}

///Parse function for protocol_name combinator
pub fn parse_protocol_name<'p>(
    input: &'p [u8],
) -> Result<(usize, ProtocolName<'p>), ParseError> {
    let combinator = protocol_name();
    combinator.parse(input)
}

///Serialize function for protocol_name combinator
pub fn serialize_protocol_name<'s>(
    v: &'s Opaque1Ff<'s>,
    data: &mut Vec<u8>,
    pos: usize,
) -> Result<usize, SerializeError> {
    let combinator = protocol_name();
    combinator.serialize(v, data, pos)
}

///Serialize function for protocol_name combinator (owned version)
pub fn serialize_gen_protocol_name(
    v: ProtocolNameOwned,
    data: &mut Vec<u8>,
    pos: usize,
) -> Result<usize, SerializeError> {
    let combinator = protocol_name();
    combinator.serialize_gen(v, data, pos)
}

///Length function for protocol_name combinator
pub fn protocol_name_len<'s>(v: &'s Opaque1Ff<'s>) -> usize {
    let combinator = protocol_name();
    combinator.length(v)
}

///Generate function for protocol_name combinator
pub fn generate_protocol_name(
    g: &mut GenSt,
) -> GResult<ProtocolNameOwned, GenerateError> {
    let mut combinator = protocol_name();
    combinator.generate(g)
}

///Parse function for protocol_version combinator
pub fn parse_protocol_version<'p>(
    input: &'p [u8],
) -> Result<(usize, ProtocolVersion), ParseError> {
    let combinator = protocol_version();
    combinator.parse(input)
}

///Serialize function for protocol_version combinator
pub fn serialize_protocol_version<'s>(
    v: ProtocolVersion,
    data: &mut Vec<u8>,
    pos: usize,
) -> Result<usize, SerializeError> {
    let combinator = protocol_version();
    combinator.serialize(v, data, pos)
}

///Serialize function for protocol_version combinator (owned version)
pub fn serialize_gen_protocol_version(
    v: ProtocolVersion,
    data: &mut Vec<u8>,
    pos: usize,
) -> Result<usize, SerializeError> {
    let combinator = protocol_version();
    combinator.serialize_gen(v, data, pos)
}

///Length function for protocol_version combinator
pub fn protocol_version_len<'s>(v: ProtocolVersion) -> usize {
    let combinator = protocol_version();
    combinator.length(v)
}

///Generate function for protocol_version combinator
pub fn generate_protocol_version(
    g: &mut GenSt,
) -> GResult<ProtocolVersion, GenerateError> {
    let mut combinator = protocol_version();
    combinator.generate(g)
}

///Parse function for server_cert_type_client_extension combinator
pub fn parse_server_cert_type_client_extension<'p>(
    input: &'p [u8],
) -> Result<(usize, ServerCertTypeClientExtension), ParseError> {
    let combinator = server_cert_type_client_extension();
    combinator.parse(input)
}

///Serialize function for server_cert_type_client_extension combinator
pub fn serialize_server_cert_type_client_extension<'s>(
    v: &'s ServerCertTypeClientExtension,
    data: &mut Vec<u8>,
    pos: usize,
) -> Result<usize, SerializeError> {
    let combinator = server_cert_type_client_extension();
    combinator.serialize(v, data, pos)
}

///Serialize function for server_cert_type_client_extension combinator (owned version)
pub fn serialize_gen_server_cert_type_client_extension(
    v: ServerCertTypeClientExtension,
    data: &mut Vec<u8>,
    pos: usize,
) -> Result<usize, SerializeError> {
    let combinator = server_cert_type_client_extension();
    combinator.serialize_gen(v, data, pos)
}

///Length function for server_cert_type_client_extension combinator
pub fn server_cert_type_client_extension_len<'s>(
    v: &'s ServerCertTypeClientExtension,
) -> usize {
    let combinator = server_cert_type_client_extension();
    combinator.length(v)
}

///Generate function for server_cert_type_client_extension combinator
pub fn generate_server_cert_type_client_extension(
    g: &mut GenSt,
) -> GResult<ServerCertTypeClientExtension, GenerateError> {
    let mut combinator = server_cert_type_client_extension();
    combinator.generate(g)
}

///Parse function for signed_certificate_timestamp_list combinator
pub fn parse_signed_certificate_timestamp_list<'p>(
    input: &'p [u8],
) -> Result<(usize, SignedCertificateTimestampList<'p>), ParseError> {
    let combinator = signed_certificate_timestamp_list();
    combinator.parse(input)
}

///Serialize function for signed_certificate_timestamp_list combinator
pub fn serialize_signed_certificate_timestamp_list<'s>(
    v: &'s SignedCertificateTimestampList<'s>,
    data: &mut Vec<u8>,
    pos: usize,
) -> Result<usize, SerializeError> {
    let combinator = signed_certificate_timestamp_list();
    combinator.serialize(v, data, pos)
}

///Serialize function for signed_certificate_timestamp_list combinator (owned version)
pub fn serialize_gen_signed_certificate_timestamp_list(
    v: SignedCertificateTimestampListOwned,
    data: &mut Vec<u8>,
    pos: usize,
) -> Result<usize, SerializeError> {
    let combinator = signed_certificate_timestamp_list();
    combinator.serialize_gen(v, data, pos)
}

///Length function for signed_certificate_timestamp_list combinator
pub fn signed_certificate_timestamp_list_len<'s>(
    v: &'s SignedCertificateTimestampList<'s>,
) -> usize {
    let combinator = signed_certificate_timestamp_list();
    combinator.length(v)
}

///Generate function for signed_certificate_timestamp_list combinator
pub fn generate_signed_certificate_timestamp_list(
    g: &mut GenSt,
) -> GResult<SignedCertificateTimestampListOwned, GenerateError> {
    let mut combinator = signed_certificate_timestamp_list();
    combinator.generate(g)
}

///Parse function for client_cert_type_server_extension combinator
pub fn parse_client_cert_type_server_extension<'p>(
    input: &'p [u8],
) -> Result<(usize, ClientCertTypeServerExtension), ParseError> {
    let combinator = client_cert_type_server_extension();
    combinator.parse(input)
}

///Serialize function for client_cert_type_server_extension combinator
pub fn serialize_client_cert_type_server_extension<'s>(
    v: ClientCertTypeServerExtension,
    data: &mut Vec<u8>,
    pos: usize,
) -> Result<usize, SerializeError> {
    let combinator = client_cert_type_server_extension();
    combinator.serialize(v, data, pos)
}

///Serialize function for client_cert_type_server_extension combinator (owned version)
pub fn serialize_gen_client_cert_type_server_extension(
    v: ClientCertTypeServerExtension,
    data: &mut Vec<u8>,
    pos: usize,
) -> Result<usize, SerializeError> {
    let combinator = client_cert_type_server_extension();
    combinator.serialize_gen(v, data, pos)
}

///Length function for client_cert_type_server_extension combinator
pub fn client_cert_type_server_extension_len<'s>(
    v: ClientCertTypeServerExtension,
) -> usize {
    let combinator = client_cert_type_server_extension();
    combinator.length(v)
}

///Generate function for client_cert_type_server_extension combinator
pub fn generate_client_cert_type_server_extension(
    g: &mut GenSt,
) -> GResult<ClientCertTypeServerExtension, GenerateError> {
    let mut combinator = client_cert_type_server_extension();
    combinator.generate(g)
}

///Parse function for opaque_2_ffff combinator
pub fn parse_opaque_2_ffff<'p>(
    input: &'p [u8],
) -> Result<(usize, Opaque2Ffff<'p>), ParseError> {
    let combinator = opaque_2_ffff();
    combinator.parse(input)
}

///Serialize function for opaque_2_ffff combinator
pub fn serialize_opaque_2_ffff<'s>(
    v: &'s Opaque2Ffff<'s>,
    data: &mut Vec<u8>,
    pos: usize,
) -> Result<usize, SerializeError> {
    let combinator = opaque_2_ffff();
    combinator.serialize(v, data, pos)
}

///Serialize function for opaque_2_ffff combinator (owned version)
pub fn serialize_gen_opaque_2_ffff(
    v: Opaque2FfffOwned,
    data: &mut Vec<u8>,
    pos: usize,
) -> Result<usize, SerializeError> {
    let combinator = opaque_2_ffff();
    combinator.serialize_gen(v, data, pos)
}

///Length function for opaque_2_ffff combinator
pub fn opaque_2_ffff_len<'s>(v: &'s Opaque2Ffff<'s>) -> usize {
    let combinator = opaque_2_ffff();
    combinator.length(v)
}

///Generate function for opaque_2_ffff combinator
pub fn generate_opaque_2_ffff(
    g: &mut GenSt,
) -> GResult<Opaque2FfffOwned, GenerateError> {
    let mut combinator = opaque_2_ffff();
    combinator.generate(g)
}

///Parse function for named_group combinator
pub fn parse_named_group<'p>(
    input: &'p [u8],
) -> Result<(usize, NamedGroup), ParseError> {
    let combinator = named_group();
    combinator.parse(input)
}

///Serialize function for named_group combinator
pub fn serialize_named_group<'s>(
    v: NamedGroup,
    data: &mut Vec<u8>,
    pos: usize,
) -> Result<usize, SerializeError> {
    let combinator = named_group();
    combinator.serialize(v, data, pos)
}

///Serialize function for named_group combinator (owned version)
pub fn serialize_gen_named_group(
    v: NamedGroup,
    data: &mut Vec<u8>,
    pos: usize,
) -> Result<usize, SerializeError> {
    let combinator = named_group();
    combinator.serialize_gen(v, data, pos)
}

///Length function for named_group combinator
pub fn named_group_len<'s>(v: NamedGroup) -> usize {
    let combinator = named_group();
    combinator.length(v)
}

///Generate function for named_group combinator
pub fn generate_named_group(g: &mut GenSt) -> GResult<NamedGroup, GenerateError> {
    let mut combinator = named_group();
    combinator.generate(g)
}

///Parse function for named_group_list combinator
pub fn parse_named_group_list<'p>(
    input: &'p [u8],
) -> Result<(usize, NamedGroupList), ParseError> {
    let combinator = named_group_list();
    combinator.parse(input)
}

///Serialize function for named_group_list combinator
pub fn serialize_named_group_list<'s>(
    v: &'s NamedGroupList,
    data: &mut Vec<u8>,
    pos: usize,
) -> Result<usize, SerializeError> {
    let combinator = named_group_list();
    combinator.serialize(v, data, pos)
}

///Serialize function for named_group_list combinator (owned version)
pub fn serialize_gen_named_group_list(
    v: NamedGroupList,
    data: &mut Vec<u8>,
    pos: usize,
) -> Result<usize, SerializeError> {
    let combinator = named_group_list();
    combinator.serialize_gen(v, data, pos)
}

///Length function for named_group_list combinator
pub fn named_group_list_len<'s>(v: &'s NamedGroupList) -> usize {
    let combinator = named_group_list();
    combinator.length(v)
}

///Generate function for named_group_list combinator
pub fn generate_named_group_list(
    g: &mut GenSt,
) -> GResult<NamedGroupList, GenerateError> {
    let mut combinator = named_group_list();
    combinator.generate(g)
}

///Parse function for opaque_0_ffffff combinator
pub fn parse_opaque_0_ffffff<'p>(
    input: &'p [u8],
) -> Result<(usize, Opaque0Ffffff<'p>), ParseError> {
    let combinator = opaque_0_ffffff();
    combinator.parse(input)
}

///Serialize function for opaque_0_ffffff combinator
pub fn serialize_opaque_0_ffffff<'s>(
    v: &'s Opaque0Ffffff<'s>,
    data: &mut Vec<u8>,
    pos: usize,
) -> Result<usize, SerializeError> {
    let combinator = opaque_0_ffffff();
    combinator.serialize(v, data, pos)
}

///Serialize function for opaque_0_ffffff combinator (owned version)
pub fn serialize_gen_opaque_0_ffffff(
    v: Opaque0FfffffOwned,
    data: &mut Vec<u8>,
    pos: usize,
) -> Result<usize, SerializeError> {
    let combinator = opaque_0_ffffff();
    combinator.serialize_gen(v, data, pos)
}

///Length function for opaque_0_ffffff combinator
pub fn opaque_0_ffffff_len<'s>(v: &'s Opaque0Ffffff<'s>) -> usize {
    let combinator = opaque_0_ffffff();
    combinator.length(v)
}

///Generate function for opaque_0_ffffff combinator
pub fn generate_opaque_0_ffffff(
    g: &mut GenSt,
) -> GResult<Opaque0FfffffOwned, GenerateError> {
    let mut combinator = opaque_0_ffffff();
    combinator.generate(g)
}

///Parse function for client_cert_type_client_extension combinator
pub fn parse_client_cert_type_client_extension<'p>(
    input: &'p [u8],
) -> Result<(usize, ClientCertTypeClientExtension), ParseError> {
    let combinator = client_cert_type_client_extension();
    combinator.parse(input)
}

///Serialize function for client_cert_type_client_extension combinator
pub fn serialize_client_cert_type_client_extension<'s>(
    v: &'s ClientCertTypeClientExtension,
    data: &mut Vec<u8>,
    pos: usize,
) -> Result<usize, SerializeError> {
    let combinator = client_cert_type_client_extension();
    combinator.serialize(v, data, pos)
}

///Serialize function for client_cert_type_client_extension combinator (owned version)
pub fn serialize_gen_client_cert_type_client_extension(
    v: ClientCertTypeClientExtension,
    data: &mut Vec<u8>,
    pos: usize,
) -> Result<usize, SerializeError> {
    let combinator = client_cert_type_client_extension();
    combinator.serialize_gen(v, data, pos)
}

///Length function for client_cert_type_client_extension combinator
pub fn client_cert_type_client_extension_len<'s>(
    v: &'s ClientCertTypeClientExtension,
) -> usize {
    let combinator = client_cert_type_client_extension();
    combinator.length(v)
}

///Generate function for client_cert_type_client_extension combinator
pub fn generate_client_cert_type_client_extension(
    g: &mut GenSt,
) -> GResult<ClientCertTypeClientExtension, GenerateError> {
    let mut combinator = client_cert_type_client_extension();
    combinator.generate(g)
}

///Parse function for heartbeat_mode combinator
pub fn parse_heartbeat_mode<'p>(
    input: &'p [u8],
) -> Result<(usize, HeartbeatMode), ParseError> {
    let combinator = heartbeat_mode();
    combinator.parse(input)
}

///Serialize function for heartbeat_mode combinator
pub fn serialize_heartbeat_mode<'s>(
    v: HeartbeatMode,
    data: &mut Vec<u8>,
    pos: usize,
) -> Result<usize, SerializeError> {
    let combinator = heartbeat_mode();
    combinator.serialize(v, data, pos)
}

///Serialize function for heartbeat_mode combinator (owned version)
pub fn serialize_gen_heartbeat_mode(
    v: HeartbeatMode,
    data: &mut Vec<u8>,
    pos: usize,
) -> Result<usize, SerializeError> {
    let combinator = heartbeat_mode();
    combinator.serialize_gen(v, data, pos)
}

///Length function for heartbeat_mode combinator
pub fn heartbeat_mode_len<'s>(v: HeartbeatMode) -> usize {
    let combinator = heartbeat_mode();
    combinator.length(v)
}

///Generate function for heartbeat_mode combinator
pub fn generate_heartbeat_mode(g: &mut GenSt) -> GResult<HeartbeatMode, GenerateError> {
    let mut combinator = heartbeat_mode();
    combinator.generate(g)
}

///Parse function for heartbeat_extension combinator
pub fn parse_heartbeat_extension<'p>(
    input: &'p [u8],
) -> Result<(usize, HeartbeatExtension), ParseError> {
    let combinator = heartbeat_extension();
    combinator.parse(input)
}

///Serialize function for heartbeat_extension combinator
pub fn serialize_heartbeat_extension<'s>(
    v: HeartbeatExtension,
    data: &mut Vec<u8>,
    pos: usize,
) -> Result<usize, SerializeError> {
    let combinator = heartbeat_extension();
    combinator.serialize(v, data, pos)
}

///Serialize function for heartbeat_extension combinator (owned version)
pub fn serialize_gen_heartbeat_extension(
    v: HeartbeatExtension,
    data: &mut Vec<u8>,
    pos: usize,
) -> Result<usize, SerializeError> {
    let combinator = heartbeat_extension();
    combinator.serialize_gen(v, data, pos)
}

///Length function for heartbeat_extension combinator
pub fn heartbeat_extension_len<'s>(v: HeartbeatExtension) -> usize {
    let combinator = heartbeat_extension();
    combinator.length(v)
}

///Generate function for heartbeat_extension combinator
pub fn generate_heartbeat_extension(
    g: &mut GenSt,
) -> GResult<HeartbeatExtension, GenerateError> {
    let mut combinator = heartbeat_extension();
    combinator.generate(g)
}

///Parse function for opaque_1_ffffff combinator
pub fn parse_opaque_1_ffffff<'p>(
    input: &'p [u8],
) -> Result<(usize, Opaque1Ffffff<'p>), ParseError> {
    let combinator = opaque_1_ffffff();
    combinator.parse(input)
}

///Serialize function for opaque_1_ffffff combinator
pub fn serialize_opaque_1_ffffff<'s>(
    v: &'s Opaque1Ffffff<'s>,
    data: &mut Vec<u8>,
    pos: usize,
) -> Result<usize, SerializeError> {
    let combinator = opaque_1_ffffff();
    combinator.serialize(v, data, pos)
}

///Serialize function for opaque_1_ffffff combinator (owned version)
pub fn serialize_gen_opaque_1_ffffff(
    v: Opaque1FfffffOwned,
    data: &mut Vec<u8>,
    pos: usize,
) -> Result<usize, SerializeError> {
    let combinator = opaque_1_ffffff();
    combinator.serialize_gen(v, data, pos)
}

///Length function for opaque_1_ffffff combinator
pub fn opaque_1_ffffff_len<'s>(v: &'s Opaque1Ffffff<'s>) -> usize {
    let combinator = opaque_1_ffffff();
    combinator.length(v)
}

///Generate function for opaque_1_ffffff combinator
pub fn generate_opaque_1_ffffff(
    g: &mut GenSt,
) -> GResult<Opaque1FfffffOwned, GenerateError> {
    let mut combinator = opaque_1_ffffff();
    combinator.generate(g)
}

///Parse function for ocsp_response combinator
pub fn parse_ocsp_response<'p>(
    input: &'p [u8],
) -> Result<(usize, OcspResponse<'p>), ParseError> {
    let combinator = ocsp_response();
    combinator.parse(input)
}

///Serialize function for ocsp_response combinator
pub fn serialize_ocsp_response<'s>(
    v: &'s Opaque1Ffffff<'s>,
    data: &mut Vec<u8>,
    pos: usize,
) -> Result<usize, SerializeError> {
    let combinator = ocsp_response();
    combinator.serialize(v, data, pos)
}

///Serialize function for ocsp_response combinator (owned version)
pub fn serialize_gen_ocsp_response(
    v: OcspResponseOwned,
    data: &mut Vec<u8>,
    pos: usize,
) -> Result<usize, SerializeError> {
    let combinator = ocsp_response();
    combinator.serialize_gen(v, data, pos)
}

///Length function for ocsp_response combinator
pub fn ocsp_response_len<'s>(v: &'s Opaque1Ffffff<'s>) -> usize {
    let combinator = ocsp_response();
    combinator.length(v)
}

///Generate function for ocsp_response combinator
pub fn generate_ocsp_response(
    g: &mut GenSt,
) -> GResult<OcspResponseOwned, GenerateError> {
    let mut combinator = ocsp_response();
    combinator.generate(g)
}

///Parse function for certificate_status combinator
pub fn parse_certificate_status<'p>(
    input: &'p [u8],
) -> Result<(usize, CertificateStatus<'p>), ParseError> {
    let combinator = certificate_status();
    combinator.parse(input)
}

///Serialize function for certificate_status combinator
pub fn serialize_certificate_status<'s>(
    v: &'s CertificateStatus<'s>,
    data: &mut Vec<u8>,
    pos: usize,
) -> Result<usize, SerializeError> {
    let combinator = certificate_status();
    combinator.serialize(v, data, pos)
}

///Serialize function for certificate_status combinator (owned version)
pub fn serialize_gen_certificate_status(
    v: CertificateStatusOwned,
    data: &mut Vec<u8>,
    pos: usize,
) -> Result<usize, SerializeError> {
    let combinator = certificate_status();
    combinator.serialize_gen(v, data, pos)
}

///Length function for certificate_status combinator
pub fn certificate_status_len<'s>(v: &'s CertificateStatus<'s>) -> usize {
    let combinator = certificate_status();
    combinator.length(v)
}

///Generate function for certificate_status combinator
pub fn generate_certificate_status(
    g: &mut GenSt,
) -> GResult<CertificateStatusOwned, GenerateError> {
    let mut combinator = certificate_status();
    combinator.generate(g)
}

///Parse function for ec_point_format combinator
pub fn parse_ec_point_format<'p>(
    input: &'p [u8],
) -> Result<(usize, EcPointFormat), ParseError> {
    let combinator = ec_point_format();
    combinator.parse(input)
}

///Serialize function for ec_point_format combinator
pub fn serialize_ec_point_format<'s>(
    v: EcPointFormat,
    data: &mut Vec<u8>,
    pos: usize,
) -> Result<usize, SerializeError> {
    let combinator = ec_point_format();
    combinator.serialize(v, data, pos)
}

///Serialize function for ec_point_format combinator (owned version)
pub fn serialize_gen_ec_point_format(
    v: EcPointFormat,
    data: &mut Vec<u8>,
    pos: usize,
) -> Result<usize, SerializeError> {
    let combinator = ec_point_format();
    combinator.serialize_gen(v, data, pos)
}

///Length function for ec_point_format combinator
pub fn ec_point_format_len<'s>(v: EcPointFormat) -> usize {
    let combinator = ec_point_format();
    combinator.length(v)
}

///Generate function for ec_point_format combinator
pub fn generate_ec_point_format(g: &mut GenSt) -> GResult<EcPointFormat, GenerateError> {
    let mut combinator = ec_point_format();
    combinator.generate(g)
}

///Parse function for certificate_status_request combinator
pub fn parse_certificate_status_request<'p>(
    input: &'p [u8],
) -> Result<(usize, CertificateStatusRequest<'p>), ParseError> {
    let combinator = certificate_status_request();
    combinator.parse(input)
}

///Serialize function for certificate_status_request combinator
pub fn serialize_certificate_status_request<'s>(
    v: &'s CertificateStatusRequest<'s>,
    data: &mut Vec<u8>,
    pos: usize,
) -> Result<usize, SerializeError> {
    let combinator = certificate_status_request();
    combinator.serialize(v, data, pos)
}

///Serialize function for certificate_status_request combinator (owned version)
pub fn serialize_gen_certificate_status_request(
    v: CertificateStatusRequestOwned,
    data: &mut Vec<u8>,
    pos: usize,
) -> Result<usize, SerializeError> {
    let combinator = certificate_status_request();
    combinator.serialize_gen(v, data, pos)
}

///Length function for certificate_status_request combinator
pub fn certificate_status_request_len<'s>(v: &'s CertificateStatusRequest<'s>) -> usize {
    let combinator = certificate_status_request();
    combinator.length(v)
}

///Generate function for certificate_status_request combinator
pub fn generate_certificate_status_request(
    g: &mut GenSt,
) -> GResult<CertificateStatusRequestOwned, GenerateError> {
    let mut combinator = certificate_status_request();
    combinator.generate(g)
}

///Parse function for ec_point_format_list combinator
pub fn parse_ec_point_format_list<'p>(
    input: &'p [u8],
) -> Result<(usize, EcPointFormatList), ParseError> {
    let combinator = ec_point_format_list();
    combinator.parse(input)
}

///Serialize function for ec_point_format_list combinator
pub fn serialize_ec_point_format_list<'s>(
    v: &'s EcPointFormatList,
    data: &mut Vec<u8>,
    pos: usize,
) -> Result<usize, SerializeError> {
    let combinator = ec_point_format_list();
    combinator.serialize(v, data, pos)
}

///Serialize function for ec_point_format_list combinator (owned version)
pub fn serialize_gen_ec_point_format_list(
    v: EcPointFormatList,
    data: &mut Vec<u8>,
    pos: usize,
) -> Result<usize, SerializeError> {
    let combinator = ec_point_format_list();
    combinator.serialize_gen(v, data, pos)
}

///Length function for ec_point_format_list combinator
pub fn ec_point_format_list_len<'s>(v: &'s EcPointFormatList) -> usize {
    let combinator = ec_point_format_list();
    combinator.length(v)
}

///Generate function for ec_point_format_list combinator
pub fn generate_ec_point_format_list(
    g: &mut GenSt,
) -> GResult<EcPointFormatList, GenerateError> {
    let mut combinator = ec_point_format_list();
    combinator.generate(g)
}

///Parse function for protocol_name_list combinator
pub fn parse_protocol_name_list<'p>(
    input: &'p [u8],
) -> Result<(usize, ProtocolNameList<'p>), ParseError> {
    let combinator = protocol_name_list();
    combinator.parse(input)
}

///Serialize function for protocol_name_list combinator
pub fn serialize_protocol_name_list<'s>(
    v: &'s ProtocolNameList<'s>,
    data: &mut Vec<u8>,
    pos: usize,
) -> Result<usize, SerializeError> {
    let combinator = protocol_name_list();
    combinator.serialize(v, data, pos)
}

///Serialize function for protocol_name_list combinator (owned version)
pub fn serialize_gen_protocol_name_list(
    v: ProtocolNameListOwned,
    data: &mut Vec<u8>,
    pos: usize,
) -> Result<usize, SerializeError> {
    let combinator = protocol_name_list();
    combinator.serialize_gen(v, data, pos)
}

///Length function for protocol_name_list combinator
pub fn protocol_name_list_len<'s>(v: &'s ProtocolNameList<'s>) -> usize {
    let combinator = protocol_name_list();
    combinator.length(v)
}

///Generate function for protocol_name_list combinator
pub fn generate_protocol_name_list(
    g: &mut GenSt,
) -> GResult<ProtocolNameListOwned, GenerateError> {
    let mut combinator = protocol_name_list();
    combinator.generate(g)
}

impl<C> Combinator<[u8], Vec<u8>> for CertificateTypeCombinator<C>
where
    C: Combinator<[u8], Vec<u8>>,
{
    type Type<'p> = C::Type<'p>;
    type SType<'s> = C::SType<'s>;
    type GType = C::GType;
    fn length<'s>(&self, v: Self::SType<'s>) -> usize
    where
        [u8]: 's,
    {
        self.0.length(v)
    }
    fn parse<'p>(&self, s: &'p [u8]) -> Result<(usize, Self::Type<'p>), ParseError>
    where
        [u8]: 'p,
    {
        self.0.parse(s)
    }
    fn serialize<'s>(
        &self,
        v: Self::SType<'s>,
        data: &mut Vec<u8>,
        pos: usize,
    ) -> Result<usize, SerializeError>
    where
        [u8]: 's,
    {
        self.0.serialize(v, data, pos)
    }
    fn serialize_gen(
        &self,
        v: Self::GType,
        data: &mut Vec<u8>,
        pos: usize,
    ) -> Result<usize, SerializeError> {
        self.0.serialize_gen(v, data, pos)
    }
    fn generate(&mut self, g: &mut GenSt) -> GResult<Self::GType, GenerateError> {
        self.0.generate(g)
    }
    fn well_formed<'s>(&self, v: Self::SType<'s>) -> bool
    where
        [u8]: 's,
    {
        self.0.well_formed(v)
    }
}

impl<C> Combinator<[u8], Vec<u8>> for ServerCertTypeServerExtensionCombinator<C>
where
    C: Combinator<[u8], Vec<u8>>,
{
    type Type<'p> = C::Type<'p>;
    type SType<'s> = C::SType<'s>;
    type GType = C::GType;
    fn length<'s>(&self, v: Self::SType<'s>) -> usize
    where
        [u8]: 's,
    {
        self.0.length(v)
    }
    fn parse<'p>(&self, s: &'p [u8]) -> Result<(usize, Self::Type<'p>), ParseError>
    where
        [u8]: 'p,
    {
        self.0.parse(s)
    }
    fn serialize<'s>(
        &self,
        v: Self::SType<'s>,
        data: &mut Vec<u8>,
        pos: usize,
    ) -> Result<usize, SerializeError>
    where
        [u8]: 's,
    {
        self.0.serialize(v, data, pos)
    }
    fn serialize_gen(
        &self,
        v: Self::GType,
        data: &mut Vec<u8>,
        pos: usize,
    ) -> Result<usize, SerializeError> {
        self.0.serialize_gen(v, data, pos)
    }
    fn generate(&mut self, g: &mut GenSt) -> GResult<Self::GType, GenerateError> {
        self.0.generate(g)
    }
    fn well_formed<'s>(&self, v: Self::SType<'s>) -> bool
    where
        [u8]: 's,
    {
        self.0.well_formed(v)
    }
}

impl<C> Combinator<[u8], Vec<u8>> for Opaque1FfCombinator<C>
where
    C: Combinator<[u8], Vec<u8>>,
{
    type Type<'p> = C::Type<'p>;
    type SType<'s> = C::SType<'s>;
    type GType = C::GType;
    fn length<'s>(&self, v: Self::SType<'s>) -> usize
    where
        [u8]: 's,
    {
        self.0.length(v)
    }
    fn parse<'p>(&self, s: &'p [u8]) -> Result<(usize, Self::Type<'p>), ParseError>
    where
        [u8]: 'p,
    {
        self.0.parse(s)
    }
    fn serialize<'s>(
        &self,
        v: Self::SType<'s>,
        data: &mut Vec<u8>,
        pos: usize,
    ) -> Result<usize, SerializeError>
    where
        [u8]: 's,
    {
        self.0.serialize(v, data, pos)
    }
    fn serialize_gen(
        &self,
        v: Self::GType,
        data: &mut Vec<u8>,
        pos: usize,
    ) -> Result<usize, SerializeError> {
        self.0.serialize_gen(v, data, pos)
    }
    fn generate(&mut self, g: &mut GenSt) -> GResult<Self::GType, GenerateError> {
        self.0.generate(g)
    }
    fn well_formed<'s>(&self, v: Self::SType<'s>) -> bool
    where
        [u8]: 's,
    {
        self.0.well_formed(v)
    }
}

impl<C> Combinator<[u8], Vec<u8>> for Opaque1FfffCombinator<C>
where
    C: Combinator<[u8], Vec<u8>>,
{
    type Type<'p> = C::Type<'p>;
    type SType<'s> = C::SType<'s>;
    type GType = C::GType;
    fn length<'s>(&self, v: Self::SType<'s>) -> usize
    where
        [u8]: 's,
    {
        self.0.length(v)
    }
    fn parse<'p>(&self, s: &'p [u8]) -> Result<(usize, Self::Type<'p>), ParseError>
    where
        [u8]: 'p,
    {
        self.0.parse(s)
    }
    fn serialize<'s>(
        &self,
        v: Self::SType<'s>,
        data: &mut Vec<u8>,
        pos: usize,
    ) -> Result<usize, SerializeError>
    where
        [u8]: 's,
    {
        self.0.serialize(v, data, pos)
    }
    fn serialize_gen(
        &self,
        v: Self::GType,
        data: &mut Vec<u8>,
        pos: usize,
    ) -> Result<usize, SerializeError> {
        self.0.serialize_gen(v, data, pos)
    }
    fn generate(&mut self, g: &mut GenSt) -> GResult<Self::GType, GenerateError> {
        self.0.generate(g)
    }
    fn well_formed<'s>(&self, v: Self::SType<'s>) -> bool
    where
        [u8]: 's,
    {
        self.0.well_formed(v)
    }
}

impl<C> Combinator<[u8], Vec<u8>> for NameTypeCombinator<C>
where
    C: Combinator<[u8], Vec<u8>>,
{
    type Type<'p> = C::Type<'p>;
    type SType<'s> = C::SType<'s>;
    type GType = C::GType;
    fn length<'s>(&self, v: Self::SType<'s>) -> usize
    where
        [u8]: 's,
    {
        self.0.length(v)
    }
    fn parse<'p>(&self, s: &'p [u8]) -> Result<(usize, Self::Type<'p>), ParseError>
    where
        [u8]: 'p,
    {
        self.0.parse(s)
    }
    fn serialize<'s>(
        &self,
        v: Self::SType<'s>,
        data: &mut Vec<u8>,
        pos: usize,
    ) -> Result<usize, SerializeError>
    where
        [u8]: 's,
    {
        self.0.serialize(v, data, pos)
    }
    fn serialize_gen(
        &self,
        v: Self::GType,
        data: &mut Vec<u8>,
        pos: usize,
    ) -> Result<usize, SerializeError> {
        self.0.serialize_gen(v, data, pos)
    }
    fn generate(&mut self, g: &mut GenSt) -> GResult<Self::GType, GenerateError> {
        self.0.generate(g)
    }
    fn well_formed<'s>(&self, v: Self::SType<'s>) -> bool
    where
        [u8]: 's,
    {
        self.0.well_formed(v)
    }
}

impl<C> Combinator<[u8], Vec<u8>> for HostNameCombinator<C>
where
    C: Combinator<[u8], Vec<u8>>,
{
    type Type<'p> = C::Type<'p>;
    type SType<'s> = C::SType<'s>;
    type GType = C::GType;
    fn length<'s>(&self, v: Self::SType<'s>) -> usize
    where
        [u8]: 's,
    {
        self.0.length(v)
    }
    fn parse<'p>(&self, s: &'p [u8]) -> Result<(usize, Self::Type<'p>), ParseError>
    where
        [u8]: 'p,
    {
        self.0.parse(s)
    }
    fn serialize<'s>(
        &self,
        v: Self::SType<'s>,
        data: &mut Vec<u8>,
        pos: usize,
    ) -> Result<usize, SerializeError>
    where
        [u8]: 's,
    {
        self.0.serialize(v, data, pos)
    }
    fn serialize_gen(
        &self,
        v: Self::GType,
        data: &mut Vec<u8>,
        pos: usize,
    ) -> Result<usize, SerializeError> {
        self.0.serialize_gen(v, data, pos)
    }
    fn generate(&mut self, g: &mut GenSt) -> GResult<Self::GType, GenerateError> {
        self.0.generate(g)
    }
    fn well_formed<'s>(&self, v: Self::SType<'s>) -> bool
    where
        [u8]: 's,
    {
        self.0.well_formed(v)
    }
}

impl<C> Combinator<[u8], Vec<u8>> for UnknownNameCombinator<C>
where
    C: Combinator<[u8], Vec<u8>>,
{
    type Type<'p> = C::Type<'p>;
    type SType<'s> = C::SType<'s>;
    type GType = C::GType;
    fn length<'s>(&self, v: Self::SType<'s>) -> usize
    where
        [u8]: 's,
    {
        self.0.length(v)
    }
    fn parse<'p>(&self, s: &'p [u8]) -> Result<(usize, Self::Type<'p>), ParseError>
    where
        [u8]: 'p,
    {
        self.0.parse(s)
    }
    fn serialize<'s>(
        &self,
        v: Self::SType<'s>,
        data: &mut Vec<u8>,
        pos: usize,
    ) -> Result<usize, SerializeError>
    where
        [u8]: 's,
    {
        self.0.serialize(v, data, pos)
    }
    fn serialize_gen(
        &self,
        v: Self::GType,
        data: &mut Vec<u8>,
        pos: usize,
    ) -> Result<usize, SerializeError> {
        self.0.serialize_gen(v, data, pos)
    }
    fn generate(&mut self, g: &mut GenSt) -> GResult<Self::GType, GenerateError> {
        self.0.generate(g)
    }
    fn well_formed<'s>(&self, v: Self::SType<'s>) -> bool
    where
        [u8]: 's,
    {
        self.0.well_formed(v)
    }
}

impl<C> Combinator<[u8], Vec<u8>> for ServerNameNameCombinator<C>
where
    C: Combinator<[u8], Vec<u8>>,
{
    type Type<'p> = C::Type<'p>;
    type SType<'s> = C::SType<'s>;
    type GType = C::GType;
    fn length<'s>(&self, v: Self::SType<'s>) -> usize
    where
        [u8]: 's,
    {
        self.0.length(v)
    }
    fn parse<'p>(&self, s: &'p [u8]) -> Result<(usize, Self::Type<'p>), ParseError>
    where
        [u8]: 'p,
    {
        self.0.parse(s)
    }
    fn serialize<'s>(
        &self,
        v: Self::SType<'s>,
        data: &mut Vec<u8>,
        pos: usize,
    ) -> Result<usize, SerializeError>
    where
        [u8]: 's,
    {
        self.0.serialize(v, data, pos)
    }
    fn serialize_gen(
        &self,
        v: Self::GType,
        data: &mut Vec<u8>,
        pos: usize,
    ) -> Result<usize, SerializeError> {
        self.0.serialize_gen(v, data, pos)
    }
    fn generate(&mut self, g: &mut GenSt) -> GResult<Self::GType, GenerateError> {
        self.0.generate(g)
    }
    fn well_formed<'s>(&self, v: Self::SType<'s>) -> bool
    where
        [u8]: 's,
    {
        self.0.well_formed(v)
    }
}

impl<C> Combinator<[u8], Vec<u8>> for ServerNameCombinator<C>
where
    C: Combinator<[u8], Vec<u8>>,
{
    type Type<'p> = C::Type<'p>;
    type SType<'s> = C::SType<'s>;
    type GType = C::GType;
    fn length<'s>(&self, v: Self::SType<'s>) -> usize
    where
        [u8]: 's,
    {
        self.0.length(v)
    }
    fn parse<'p>(&self, s: &'p [u8]) -> Result<(usize, Self::Type<'p>), ParseError>
    where
        [u8]: 'p,
    {
        self.0.parse(s)
    }
    fn serialize<'s>(
        &self,
        v: Self::SType<'s>,
        data: &mut Vec<u8>,
        pos: usize,
    ) -> Result<usize, SerializeError>
    where
        [u8]: 's,
    {
        self.0.serialize(v, data, pos)
    }
    fn serialize_gen(
        &self,
        v: Self::GType,
        data: &mut Vec<u8>,
        pos: usize,
    ) -> Result<usize, SerializeError> {
        self.0.serialize_gen(v, data, pos)
    }
    fn generate(&mut self, g: &mut GenSt) -> GResult<Self::GType, GenerateError> {
        self.0.generate(g)
    }
    fn well_formed<'s>(&self, v: Self::SType<'s>) -> bool
    where
        [u8]: 's,
    {
        self.0.well_formed(v)
    }
}

impl<C> Combinator<[u8], Vec<u8>> for ServerNameListCombinator<C>
where
    C: Combinator<[u8], Vec<u8>>,
{
    type Type<'p> = C::Type<'p>;
    type SType<'s> = C::SType<'s>;
    type GType = C::GType;
    fn length<'s>(&self, v: Self::SType<'s>) -> usize
    where
        [u8]: 's,
    {
        self.0.length(v)
    }
    fn parse<'p>(&self, s: &'p [u8]) -> Result<(usize, Self::Type<'p>), ParseError>
    where
        [u8]: 'p,
    {
        self.0.parse(s)
    }
    fn serialize<'s>(
        &self,
        v: Self::SType<'s>,
        data: &mut Vec<u8>,
        pos: usize,
    ) -> Result<usize, SerializeError>
    where
        [u8]: 's,
    {
        self.0.serialize(v, data, pos)
    }
    fn serialize_gen(
        &self,
        v: Self::GType,
        data: &mut Vec<u8>,
        pos: usize,
    ) -> Result<usize, SerializeError> {
        self.0.serialize_gen(v, data, pos)
    }
    fn generate(&mut self, g: &mut GenSt) -> GResult<Self::GType, GenerateError> {
        self.0.generate(g)
    }
    fn well_formed<'s>(&self, v: Self::SType<'s>) -> bool
    where
        [u8]: 's,
    {
        self.0.well_formed(v)
    }
}

impl<C> Combinator<[u8], Vec<u8>> for Opaque0FfffCombinator<C>
where
    C: Combinator<[u8], Vec<u8>>,
{
    type Type<'p> = C::Type<'p>;
    type SType<'s> = C::SType<'s>;
    type GType = C::GType;
    fn length<'s>(&self, v: Self::SType<'s>) -> usize
    where
        [u8]: 's,
    {
        self.0.length(v)
    }
    fn parse<'p>(&self, s: &'p [u8]) -> Result<(usize, Self::Type<'p>), ParseError>
    where
        [u8]: 'p,
    {
        self.0.parse(s)
    }
    fn serialize<'s>(
        &self,
        v: Self::SType<'s>,
        data: &mut Vec<u8>,
        pos: usize,
    ) -> Result<usize, SerializeError>
    where
        [u8]: 's,
    {
        self.0.serialize(v, data, pos)
    }
    fn serialize_gen(
        &self,
        v: Self::GType,
        data: &mut Vec<u8>,
        pos: usize,
    ) -> Result<usize, SerializeError> {
        self.0.serialize_gen(v, data, pos)
    }
    fn generate(&mut self, g: &mut GenSt) -> GResult<Self::GType, GenerateError> {
        self.0.generate(g)
    }
    fn well_formed<'s>(&self, v: Self::SType<'s>) -> bool
    where
        [u8]: 's,
    {
        self.0.well_formed(v)
    }
}

impl<C> Combinator<[u8], Vec<u8>> for OcspExtensionsCombinator<C>
where
    C: Combinator<[u8], Vec<u8>>,
{
    type Type<'p> = C::Type<'p>;
    type SType<'s> = C::SType<'s>;
    type GType = C::GType;
    fn length<'s>(&self, v: Self::SType<'s>) -> usize
    where
        [u8]: 's,
    {
        self.0.length(v)
    }
    fn parse<'p>(&self, s: &'p [u8]) -> Result<(usize, Self::Type<'p>), ParseError>
    where
        [u8]: 'p,
    {
        self.0.parse(s)
    }
    fn serialize<'s>(
        &self,
        v: Self::SType<'s>,
        data: &mut Vec<u8>,
        pos: usize,
    ) -> Result<usize, SerializeError>
    where
        [u8]: 's,
    {
        self.0.serialize(v, data, pos)
    }
    fn serialize_gen(
        &self,
        v: Self::GType,
        data: &mut Vec<u8>,
        pos: usize,
    ) -> Result<usize, SerializeError> {
        self.0.serialize_gen(v, data, pos)
    }
    fn generate(&mut self, g: &mut GenSt) -> GResult<Self::GType, GenerateError> {
        self.0.generate(g)
    }
    fn well_formed<'s>(&self, v: Self::SType<'s>) -> bool
    where
        [u8]: 's,
    {
        self.0.well_formed(v)
    }
}

impl<C> Combinator<[u8], Vec<u8>> for ResponderIdCombinator<C>
where
    C: Combinator<[u8], Vec<u8>>,
{
    type Type<'p> = C::Type<'p>;
    type SType<'s> = C::SType<'s>;
    type GType = C::GType;
    fn length<'s>(&self, v: Self::SType<'s>) -> usize
    where
        [u8]: 's,
    {
        self.0.length(v)
    }
    fn parse<'p>(&self, s: &'p [u8]) -> Result<(usize, Self::Type<'p>), ParseError>
    where
        [u8]: 'p,
    {
        self.0.parse(s)
    }
    fn serialize<'s>(
        &self,
        v: Self::SType<'s>,
        data: &mut Vec<u8>,
        pos: usize,
    ) -> Result<usize, SerializeError>
    where
        [u8]: 's,
    {
        self.0.serialize(v, data, pos)
    }
    fn serialize_gen(
        &self,
        v: Self::GType,
        data: &mut Vec<u8>,
        pos: usize,
    ) -> Result<usize, SerializeError> {
        self.0.serialize_gen(v, data, pos)
    }
    fn generate(&mut self, g: &mut GenSt) -> GResult<Self::GType, GenerateError> {
        self.0.generate(g)
    }
    fn well_formed<'s>(&self, v: Self::SType<'s>) -> bool
    where
        [u8]: 's,
    {
        self.0.well_formed(v)
    }
}

impl<C> Combinator<[u8], Vec<u8>> for ResponderIdListCombinator<C>
where
    C: Combinator<[u8], Vec<u8>>,
{
    type Type<'p> = C::Type<'p>;
    type SType<'s> = C::SType<'s>;
    type GType = C::GType;
    fn length<'s>(&self, v: Self::SType<'s>) -> usize
    where
        [u8]: 's,
    {
        self.0.length(v)
    }
    fn parse<'p>(&self, s: &'p [u8]) -> Result<(usize, Self::Type<'p>), ParseError>
    where
        [u8]: 'p,
    {
        self.0.parse(s)
    }
    fn serialize<'s>(
        &self,
        v: Self::SType<'s>,
        data: &mut Vec<u8>,
        pos: usize,
    ) -> Result<usize, SerializeError>
    where
        [u8]: 's,
    {
        self.0.serialize(v, data, pos)
    }
    fn serialize_gen(
        &self,
        v: Self::GType,
        data: &mut Vec<u8>,
        pos: usize,
    ) -> Result<usize, SerializeError> {
        self.0.serialize_gen(v, data, pos)
    }
    fn generate(&mut self, g: &mut GenSt) -> GResult<Self::GType, GenerateError> {
        self.0.generate(g)
    }
    fn well_formed<'s>(&self, v: Self::SType<'s>) -> bool
    where
        [u8]: 's,
    {
        self.0.well_formed(v)
    }
}

impl<C> Combinator<[u8], Vec<u8>> for OscpStatusRequestCombinator<C>
where
    C: Combinator<[u8], Vec<u8>>,
{
    type Type<'p> = C::Type<'p>;
    type SType<'s> = C::SType<'s>;
    type GType = C::GType;
    fn length<'s>(&self, v: Self::SType<'s>) -> usize
    where
        [u8]: 's,
    {
        self.0.length(v)
    }
    fn parse<'p>(&self, s: &'p [u8]) -> Result<(usize, Self::Type<'p>), ParseError>
    where
        [u8]: 'p,
    {
        self.0.parse(s)
    }
    fn serialize<'s>(
        &self,
        v: Self::SType<'s>,
        data: &mut Vec<u8>,
        pos: usize,
    ) -> Result<usize, SerializeError>
    where
        [u8]: 's,
    {
        self.0.serialize(v, data, pos)
    }
    fn serialize_gen(
        &self,
        v: Self::GType,
        data: &mut Vec<u8>,
        pos: usize,
    ) -> Result<usize, SerializeError> {
        self.0.serialize_gen(v, data, pos)
    }
    fn generate(&mut self, g: &mut GenSt) -> GResult<Self::GType, GenerateError> {
        self.0.generate(g)
    }
    fn well_formed<'s>(&self, v: Self::SType<'s>) -> bool
    where
        [u8]: 's,
    {
        self.0.well_formed(v)
    }
}

impl<C> Combinator<[u8], Vec<u8>> for SrtpProtectionProfileCombinator<C>
where
    C: Combinator<[u8], Vec<u8>>,
{
    type Type<'p> = C::Type<'p>;
    type SType<'s> = C::SType<'s>;
    type GType = C::GType;
    fn length<'s>(&self, v: Self::SType<'s>) -> usize
    where
        [u8]: 's,
    {
        self.0.length(v)
    }
    fn parse<'p>(&self, s: &'p [u8]) -> Result<(usize, Self::Type<'p>), ParseError>
    where
        [u8]: 'p,
    {
        self.0.parse(s)
    }
    fn serialize<'s>(
        &self,
        v: Self::SType<'s>,
        data: &mut Vec<u8>,
        pos: usize,
    ) -> Result<usize, SerializeError>
    where
        [u8]: 's,
    {
        self.0.serialize(v, data, pos)
    }
    fn serialize_gen(
        &self,
        v: Self::GType,
        data: &mut Vec<u8>,
        pos: usize,
    ) -> Result<usize, SerializeError> {
        self.0.serialize_gen(v, data, pos)
    }
    fn generate(&mut self, g: &mut GenSt) -> GResult<Self::GType, GenerateError> {
        self.0.generate(g)
    }
    fn well_formed<'s>(&self, v: Self::SType<'s>) -> bool
    where
        [u8]: 's,
    {
        self.0.well_formed(v)
    }
}

impl<C> Combinator<[u8], Vec<u8>> for SrtpProtectionProfilesCombinator<C>
where
    C: Combinator<[u8], Vec<u8>>,
{
    type Type<'p> = C::Type<'p>;
    type SType<'s> = C::SType<'s>;
    type GType = C::GType;
    fn length<'s>(&self, v: Self::SType<'s>) -> usize
    where
        [u8]: 's,
    {
        self.0.length(v)
    }
    fn parse<'p>(&self, s: &'p [u8]) -> Result<(usize, Self::Type<'p>), ParseError>
    where
        [u8]: 'p,
    {
        self.0.parse(s)
    }
    fn serialize<'s>(
        &self,
        v: Self::SType<'s>,
        data: &mut Vec<u8>,
        pos: usize,
    ) -> Result<usize, SerializeError>
    where
        [u8]: 's,
    {
        self.0.serialize(v, data, pos)
    }
    fn serialize_gen(
        &self,
        v: Self::GType,
        data: &mut Vec<u8>,
        pos: usize,
    ) -> Result<usize, SerializeError> {
        self.0.serialize_gen(v, data, pos)
    }
    fn generate(&mut self, g: &mut GenSt) -> GResult<Self::GType, GenerateError> {
        self.0.generate(g)
    }
    fn well_formed<'s>(&self, v: Self::SType<'s>) -> bool
    where
        [u8]: 's,
    {
        self.0.well_formed(v)
    }
}

impl<C> Combinator<[u8], Vec<u8>> for Opaque0FfCombinator<C>
where
    C: Combinator<[u8], Vec<u8>>,
{
    type Type<'p> = C::Type<'p>;
    type SType<'s> = C::SType<'s>;
    type GType = C::GType;
    fn length<'s>(&self, v: Self::SType<'s>) -> usize
    where
        [u8]: 's,
    {
        self.0.length(v)
    }
    fn parse<'p>(&self, s: &'p [u8]) -> Result<(usize, Self::Type<'p>), ParseError>
    where
        [u8]: 'p,
    {
        self.0.parse(s)
    }
    fn serialize<'s>(
        &self,
        v: Self::SType<'s>,
        data: &mut Vec<u8>,
        pos: usize,
    ) -> Result<usize, SerializeError>
    where
        [u8]: 's,
    {
        self.0.serialize(v, data, pos)
    }
    fn serialize_gen(
        &self,
        v: Self::GType,
        data: &mut Vec<u8>,
        pos: usize,
    ) -> Result<usize, SerializeError> {
        self.0.serialize_gen(v, data, pos)
    }
    fn generate(&mut self, g: &mut GenSt) -> GResult<Self::GType, GenerateError> {
        self.0.generate(g)
    }
    fn well_formed<'s>(&self, v: Self::SType<'s>) -> bool
    where
        [u8]: 's,
    {
        self.0.well_formed(v)
    }
}

impl<C> Combinator<[u8], Vec<u8>> for UseSrtpDataCombinator<C>
where
    C: Combinator<[u8], Vec<u8>>,
{
    type Type<'p> = C::Type<'p>;
    type SType<'s> = C::SType<'s>;
    type GType = C::GType;
    fn length<'s>(&self, v: Self::SType<'s>) -> usize
    where
        [u8]: 's,
    {
        self.0.length(v)
    }
    fn parse<'p>(&self, s: &'p [u8]) -> Result<(usize, Self::Type<'p>), ParseError>
    where
        [u8]: 'p,
    {
        self.0.parse(s)
    }
    fn serialize<'s>(
        &self,
        v: Self::SType<'s>,
        data: &mut Vec<u8>,
        pos: usize,
    ) -> Result<usize, SerializeError>
    where
        [u8]: 's,
    {
        self.0.serialize(v, data, pos)
    }
    fn serialize_gen(
        &self,
        v: Self::GType,
        data: &mut Vec<u8>,
        pos: usize,
    ) -> Result<usize, SerializeError> {
        self.0.serialize_gen(v, data, pos)
    }
    fn generate(&mut self, g: &mut GenSt) -> GResult<Self::GType, GenerateError> {
        self.0.generate(g)
    }
    fn well_formed<'s>(&self, v: Self::SType<'s>) -> bool
    where
        [u8]: 's,
    {
        self.0.well_formed(v)
    }
}

impl<C> Combinator<[u8], Vec<u8>> for MaxFragmentLengthCombinator<C>
where
    C: Combinator<[u8], Vec<u8>>,
{
    type Type<'p> = C::Type<'p>;
    type SType<'s> = C::SType<'s>;
    type GType = C::GType;
    fn length<'s>(&self, v: Self::SType<'s>) -> usize
    where
        [u8]: 's,
    {
        self.0.length(v)
    }
    fn parse<'p>(&self, s: &'p [u8]) -> Result<(usize, Self::Type<'p>), ParseError>
    where
        [u8]: 'p,
    {
        self.0.parse(s)
    }
    fn serialize<'s>(
        &self,
        v: Self::SType<'s>,
        data: &mut Vec<u8>,
        pos: usize,
    ) -> Result<usize, SerializeError>
    where
        [u8]: 's,
    {
        self.0.serialize(v, data, pos)
    }
    fn serialize_gen(
        &self,
        v: Self::GType,
        data: &mut Vec<u8>,
        pos: usize,
    ) -> Result<usize, SerializeError> {
        self.0.serialize_gen(v, data, pos)
    }
    fn generate(&mut self, g: &mut GenSt) -> GResult<Self::GType, GenerateError> {
        self.0.generate(g)
    }
    fn well_formed<'s>(&self, v: Self::SType<'s>) -> bool
    where
        [u8]: 's,
    {
        self.0.well_formed(v)
    }
}

impl<C> Combinator<[u8], Vec<u8>> for SignatureSchemeCombinator<C>
where
    C: Combinator<[u8], Vec<u8>>,
{
    type Type<'p> = C::Type<'p>;
    type SType<'s> = C::SType<'s>;
    type GType = C::GType;
    fn length<'s>(&self, v: Self::SType<'s>) -> usize
    where
        [u8]: 's,
    {
        self.0.length(v)
    }
    fn parse<'p>(&self, s: &'p [u8]) -> Result<(usize, Self::Type<'p>), ParseError>
    where
        [u8]: 'p,
    {
        self.0.parse(s)
    }
    fn serialize<'s>(
        &self,
        v: Self::SType<'s>,
        data: &mut Vec<u8>,
        pos: usize,
    ) -> Result<usize, SerializeError>
    where
        [u8]: 's,
    {
        self.0.serialize(v, data, pos)
    }
    fn serialize_gen(
        &self,
        v: Self::GType,
        data: &mut Vec<u8>,
        pos: usize,
    ) -> Result<usize, SerializeError> {
        self.0.serialize_gen(v, data, pos)
    }
    fn generate(&mut self, g: &mut GenSt) -> GResult<Self::GType, GenerateError> {
        self.0.generate(g)
    }
    fn well_formed<'s>(&self, v: Self::SType<'s>) -> bool
    where
        [u8]: 's,
    {
        self.0.well_formed(v)
    }
}

impl<C> Combinator<[u8], Vec<u8>> for ZeroByteCombinator<C>
where
    C: Combinator<[u8], Vec<u8>>,
{
    type Type<'p> = C::Type<'p>;
    type SType<'s> = C::SType<'s>;
    type GType = C::GType;
    fn length<'s>(&self, v: Self::SType<'s>) -> usize
    where
        [u8]: 's,
    {
        self.0.length(v)
    }
    fn parse<'p>(&self, s: &'p [u8]) -> Result<(usize, Self::Type<'p>), ParseError>
    where
        [u8]: 'p,
    {
        self.0.parse(s)
    }
    fn serialize<'s>(
        &self,
        v: Self::SType<'s>,
        data: &mut Vec<u8>,
        pos: usize,
    ) -> Result<usize, SerializeError>
    where
        [u8]: 's,
    {
        self.0.serialize(v, data, pos)
    }
    fn serialize_gen(
        &self,
        v: Self::GType,
        data: &mut Vec<u8>,
        pos: usize,
    ) -> Result<usize, SerializeError> {
        self.0.serialize_gen(v, data, pos)
    }
    fn generate(&mut self, g: &mut GenSt) -> GResult<Self::GType, GenerateError> {
        self.0.generate(g)
    }
    fn well_formed<'s>(&self, v: Self::SType<'s>) -> bool
    where
        [u8]: 's,
    {
        self.0.well_formed(v)
    }
}

impl<C> Combinator<[u8], Vec<u8>> for SerializedSctCombinator<C>
where
    C: Combinator<[u8], Vec<u8>>,
{
    type Type<'p> = C::Type<'p>;
    type SType<'s> = C::SType<'s>;
    type GType = C::GType;
    fn length<'s>(&self, v: Self::SType<'s>) -> usize
    where
        [u8]: 's,
    {
        self.0.length(v)
    }
    fn parse<'p>(&self, s: &'p [u8]) -> Result<(usize, Self::Type<'p>), ParseError>
    where
        [u8]: 'p,
    {
        self.0.parse(s)
    }
    fn serialize<'s>(
        &self,
        v: Self::SType<'s>,
        data: &mut Vec<u8>,
        pos: usize,
    ) -> Result<usize, SerializeError>
    where
        [u8]: 's,
    {
        self.0.serialize(v, data, pos)
    }
    fn serialize_gen(
        &self,
        v: Self::GType,
        data: &mut Vec<u8>,
        pos: usize,
    ) -> Result<usize, SerializeError> {
        self.0.serialize_gen(v, data, pos)
    }
    fn generate(&mut self, g: &mut GenSt) -> GResult<Self::GType, GenerateError> {
        self.0.generate(g)
    }
    fn well_formed<'s>(&self, v: Self::SType<'s>) -> bool
    where
        [u8]: 's,
    {
        self.0.well_formed(v)
    }
}

impl<C> Combinator<[u8], Vec<u8>> for PaddingExtensionCombinator<C>
where
    C: Combinator<[u8], Vec<u8>>,
{
    type Type<'p> = C::Type<'p>;
    type SType<'s> = C::SType<'s>;
    type GType = C::GType;
    fn length<'s>(&self, v: Self::SType<'s>) -> usize
    where
        [u8]: 's,
    {
        self.0.length(v)
    }
    fn parse<'p>(&self, s: &'p [u8]) -> Result<(usize, Self::Type<'p>), ParseError>
    where
        [u8]: 'p,
    {
        self.0.parse(s)
    }
    fn serialize<'s>(
        &self,
        v: Self::SType<'s>,
        data: &mut Vec<u8>,
        pos: usize,
    ) -> Result<usize, SerializeError>
    where
        [u8]: 's,
    {
        self.0.serialize(v, data, pos)
    }
    fn serialize_gen(
        &self,
        v: Self::GType,
        data: &mut Vec<u8>,
        pos: usize,
    ) -> Result<usize, SerializeError> {
        self.0.serialize_gen(v, data, pos)
    }
    fn generate(&mut self, g: &mut GenSt) -> GResult<Self::GType, GenerateError> {
        self.0.generate(g)
    }
    fn well_formed<'s>(&self, v: Self::SType<'s>) -> bool
    where
        [u8]: 's,
    {
        self.0.well_formed(v)
    }
}

impl<C> Combinator<[u8], Vec<u8>> for SignatureSchemeListCombinator<C>
where
    C: Combinator<[u8], Vec<u8>>,
{
    type Type<'p> = C::Type<'p>;
    type SType<'s> = C::SType<'s>;
    type GType = C::GType;
    fn length<'s>(&self, v: Self::SType<'s>) -> usize
    where
        [u8]: 's,
    {
        self.0.length(v)
    }
    fn parse<'p>(&self, s: &'p [u8]) -> Result<(usize, Self::Type<'p>), ParseError>
    where
        [u8]: 'p,
    {
        self.0.parse(s)
    }
    fn serialize<'s>(
        &self,
        v: Self::SType<'s>,
        data: &mut Vec<u8>,
        pos: usize,
    ) -> Result<usize, SerializeError>
    where
        [u8]: 's,
    {
        self.0.serialize(v, data, pos)
    }
    fn serialize_gen(
        &self,
        v: Self::GType,
        data: &mut Vec<u8>,
        pos: usize,
    ) -> Result<usize, SerializeError> {
        self.0.serialize_gen(v, data, pos)
    }
    fn generate(&mut self, g: &mut GenSt) -> GResult<Self::GType, GenerateError> {
        self.0.generate(g)
    }
    fn well_formed<'s>(&self, v: Self::SType<'s>) -> bool
    where
        [u8]: 's,
    {
        self.0.well_formed(v)
    }
}

impl<C> Combinator<[u8], Vec<u8>> for ExtensionTypeCombinator<C>
where
    C: Combinator<[u8], Vec<u8>>,
{
    type Type<'p> = C::Type<'p>;
    type SType<'s> = C::SType<'s>;
    type GType = C::GType;
    fn length<'s>(&self, v: Self::SType<'s>) -> usize
    where
        [u8]: 's,
    {
        self.0.length(v)
    }
    fn parse<'p>(&self, s: &'p [u8]) -> Result<(usize, Self::Type<'p>), ParseError>
    where
        [u8]: 'p,
    {
        self.0.parse(s)
    }
    fn serialize<'s>(
        &self,
        v: Self::SType<'s>,
        data: &mut Vec<u8>,
        pos: usize,
    ) -> Result<usize, SerializeError>
    where
        [u8]: 's,
    {
        self.0.serialize(v, data, pos)
    }
    fn serialize_gen(
        &self,
        v: Self::GType,
        data: &mut Vec<u8>,
        pos: usize,
    ) -> Result<usize, SerializeError> {
        self.0.serialize_gen(v, data, pos)
    }
    fn generate(&mut self, g: &mut GenSt) -> GResult<Self::GType, GenerateError> {
        self.0.generate(g)
    }
    fn well_formed<'s>(&self, v: Self::SType<'s>) -> bool
    where
        [u8]: 's,
    {
        self.0.well_formed(v)
    }
}

impl<C> Combinator<[u8], Vec<u8>> for ProtocolNameCombinator<C>
where
    C: Combinator<[u8], Vec<u8>>,
{
    type Type<'p> = C::Type<'p>;
    type SType<'s> = C::SType<'s>;
    type GType = C::GType;
    fn length<'s>(&self, v: Self::SType<'s>) -> usize
    where
        [u8]: 's,
    {
        self.0.length(v)
    }
    fn parse<'p>(&self, s: &'p [u8]) -> Result<(usize, Self::Type<'p>), ParseError>
    where
        [u8]: 'p,
    {
        self.0.parse(s)
    }
    fn serialize<'s>(
        &self,
        v: Self::SType<'s>,
        data: &mut Vec<u8>,
        pos: usize,
    ) -> Result<usize, SerializeError>
    where
        [u8]: 's,
    {
        self.0.serialize(v, data, pos)
    }
    fn serialize_gen(
        &self,
        v: Self::GType,
        data: &mut Vec<u8>,
        pos: usize,
    ) -> Result<usize, SerializeError> {
        self.0.serialize_gen(v, data, pos)
    }
    fn generate(&mut self, g: &mut GenSt) -> GResult<Self::GType, GenerateError> {
        self.0.generate(g)
    }
    fn well_formed<'s>(&self, v: Self::SType<'s>) -> bool
    where
        [u8]: 's,
    {
        self.0.well_formed(v)
    }
}

impl<C> Combinator<[u8], Vec<u8>> for ProtocolVersionCombinator<C>
where
    C: Combinator<[u8], Vec<u8>>,
{
    type Type<'p> = C::Type<'p>;
    type SType<'s> = C::SType<'s>;
    type GType = C::GType;
    fn length<'s>(&self, v: Self::SType<'s>) -> usize
    where
        [u8]: 's,
    {
        self.0.length(v)
    }
    fn parse<'p>(&self, s: &'p [u8]) -> Result<(usize, Self::Type<'p>), ParseError>
    where
        [u8]: 'p,
    {
        self.0.parse(s)
    }
    fn serialize<'s>(
        &self,
        v: Self::SType<'s>,
        data: &mut Vec<u8>,
        pos: usize,
    ) -> Result<usize, SerializeError>
    where
        [u8]: 's,
    {
        self.0.serialize(v, data, pos)
    }
    fn serialize_gen(
        &self,
        v: Self::GType,
        data: &mut Vec<u8>,
        pos: usize,
    ) -> Result<usize, SerializeError> {
        self.0.serialize_gen(v, data, pos)
    }
    fn generate(&mut self, g: &mut GenSt) -> GResult<Self::GType, GenerateError> {
        self.0.generate(g)
    }
    fn well_formed<'s>(&self, v: Self::SType<'s>) -> bool
    where
        [u8]: 's,
    {
        self.0.well_formed(v)
    }
}

impl<C> Combinator<[u8], Vec<u8>> for ServerCertTypeClientExtensionCombinator<C>
where
    C: Combinator<[u8], Vec<u8>>,
{
    type Type<'p> = C::Type<'p>;
    type SType<'s> = C::SType<'s>;
    type GType = C::GType;
    fn length<'s>(&self, v: Self::SType<'s>) -> usize
    where
        [u8]: 's,
    {
        self.0.length(v)
    }
    fn parse<'p>(&self, s: &'p [u8]) -> Result<(usize, Self::Type<'p>), ParseError>
    where
        [u8]: 'p,
    {
        self.0.parse(s)
    }
    fn serialize<'s>(
        &self,
        v: Self::SType<'s>,
        data: &mut Vec<u8>,
        pos: usize,
    ) -> Result<usize, SerializeError>
    where
        [u8]: 's,
    {
        self.0.serialize(v, data, pos)
    }
    fn serialize_gen(
        &self,
        v: Self::GType,
        data: &mut Vec<u8>,
        pos: usize,
    ) -> Result<usize, SerializeError> {
        self.0.serialize_gen(v, data, pos)
    }
    fn generate(&mut self, g: &mut GenSt) -> GResult<Self::GType, GenerateError> {
        self.0.generate(g)
    }
    fn well_formed<'s>(&self, v: Self::SType<'s>) -> bool
    where
        [u8]: 's,
    {
        self.0.well_formed(v)
    }
}

impl<C> Combinator<[u8], Vec<u8>> for SignedCertificateTimestampListCombinator<C>
where
    C: Combinator<[u8], Vec<u8>>,
{
    type Type<'p> = C::Type<'p>;
    type SType<'s> = C::SType<'s>;
    type GType = C::GType;
    fn length<'s>(&self, v: Self::SType<'s>) -> usize
    where
        [u8]: 's,
    {
        self.0.length(v)
    }
    fn parse<'p>(&self, s: &'p [u8]) -> Result<(usize, Self::Type<'p>), ParseError>
    where
        [u8]: 'p,
    {
        self.0.parse(s)
    }
    fn serialize<'s>(
        &self,
        v: Self::SType<'s>,
        data: &mut Vec<u8>,
        pos: usize,
    ) -> Result<usize, SerializeError>
    where
        [u8]: 's,
    {
        self.0.serialize(v, data, pos)
    }
    fn serialize_gen(
        &self,
        v: Self::GType,
        data: &mut Vec<u8>,
        pos: usize,
    ) -> Result<usize, SerializeError> {
        self.0.serialize_gen(v, data, pos)
    }
    fn generate(&mut self, g: &mut GenSt) -> GResult<Self::GType, GenerateError> {
        self.0.generate(g)
    }
    fn well_formed<'s>(&self, v: Self::SType<'s>) -> bool
    where
        [u8]: 's,
    {
        self.0.well_formed(v)
    }
}

impl<C> Combinator<[u8], Vec<u8>> for ClientCertTypeServerExtensionCombinator<C>
where
    C: Combinator<[u8], Vec<u8>>,
{
    type Type<'p> = C::Type<'p>;
    type SType<'s> = C::SType<'s>;
    type GType = C::GType;
    fn length<'s>(&self, v: Self::SType<'s>) -> usize
    where
        [u8]: 's,
    {
        self.0.length(v)
    }
    fn parse<'p>(&self, s: &'p [u8]) -> Result<(usize, Self::Type<'p>), ParseError>
    where
        [u8]: 'p,
    {
        self.0.parse(s)
    }
    fn serialize<'s>(
        &self,
        v: Self::SType<'s>,
        data: &mut Vec<u8>,
        pos: usize,
    ) -> Result<usize, SerializeError>
    where
        [u8]: 's,
    {
        self.0.serialize(v, data, pos)
    }
    fn serialize_gen(
        &self,
        v: Self::GType,
        data: &mut Vec<u8>,
        pos: usize,
    ) -> Result<usize, SerializeError> {
        self.0.serialize_gen(v, data, pos)
    }
    fn generate(&mut self, g: &mut GenSt) -> GResult<Self::GType, GenerateError> {
        self.0.generate(g)
    }
    fn well_formed<'s>(&self, v: Self::SType<'s>) -> bool
    where
        [u8]: 's,
    {
        self.0.well_formed(v)
    }
}

impl<C> Combinator<[u8], Vec<u8>> for Opaque2FfffCombinator<C>
where
    C: Combinator<[u8], Vec<u8>>,
{
    type Type<'p> = C::Type<'p>;
    type SType<'s> = C::SType<'s>;
    type GType = C::GType;
    fn length<'s>(&self, v: Self::SType<'s>) -> usize
    where
        [u8]: 's,
    {
        self.0.length(v)
    }
    fn parse<'p>(&self, s: &'p [u8]) -> Result<(usize, Self::Type<'p>), ParseError>
    where
        [u8]: 'p,
    {
        self.0.parse(s)
    }
    fn serialize<'s>(
        &self,
        v: Self::SType<'s>,
        data: &mut Vec<u8>,
        pos: usize,
    ) -> Result<usize, SerializeError>
    where
        [u8]: 's,
    {
        self.0.serialize(v, data, pos)
    }
    fn serialize_gen(
        &self,
        v: Self::GType,
        data: &mut Vec<u8>,
        pos: usize,
    ) -> Result<usize, SerializeError> {
        self.0.serialize_gen(v, data, pos)
    }
    fn generate(&mut self, g: &mut GenSt) -> GResult<Self::GType, GenerateError> {
        self.0.generate(g)
    }
    fn well_formed<'s>(&self, v: Self::SType<'s>) -> bool
    where
        [u8]: 's,
    {
        self.0.well_formed(v)
    }
}

impl<C> Combinator<[u8], Vec<u8>> for NamedGroupCombinator<C>
where
    C: Combinator<[u8], Vec<u8>>,
{
    type Type<'p> = C::Type<'p>;
    type SType<'s> = C::SType<'s>;
    type GType = C::GType;
    fn length<'s>(&self, v: Self::SType<'s>) -> usize
    where
        [u8]: 's,
    {
        self.0.length(v)
    }
    fn parse<'p>(&self, s: &'p [u8]) -> Result<(usize, Self::Type<'p>), ParseError>
    where
        [u8]: 'p,
    {
        self.0.parse(s)
    }
    fn serialize<'s>(
        &self,
        v: Self::SType<'s>,
        data: &mut Vec<u8>,
        pos: usize,
    ) -> Result<usize, SerializeError>
    where
        [u8]: 's,
    {
        self.0.serialize(v, data, pos)
    }
    fn serialize_gen(
        &self,
        v: Self::GType,
        data: &mut Vec<u8>,
        pos: usize,
    ) -> Result<usize, SerializeError> {
        self.0.serialize_gen(v, data, pos)
    }
    fn generate(&mut self, g: &mut GenSt) -> GResult<Self::GType, GenerateError> {
        self.0.generate(g)
    }
    fn well_formed<'s>(&self, v: Self::SType<'s>) -> bool
    where
        [u8]: 's,
    {
        self.0.well_formed(v)
    }
}

impl<C> Combinator<[u8], Vec<u8>> for NamedGroupListCombinator<C>
where
    C: Combinator<[u8], Vec<u8>>,
{
    type Type<'p> = C::Type<'p>;
    type SType<'s> = C::SType<'s>;
    type GType = C::GType;
    fn length<'s>(&self, v: Self::SType<'s>) -> usize
    where
        [u8]: 's,
    {
        self.0.length(v)
    }
    fn parse<'p>(&self, s: &'p [u8]) -> Result<(usize, Self::Type<'p>), ParseError>
    where
        [u8]: 'p,
    {
        self.0.parse(s)
    }
    fn serialize<'s>(
        &self,
        v: Self::SType<'s>,
        data: &mut Vec<u8>,
        pos: usize,
    ) -> Result<usize, SerializeError>
    where
        [u8]: 's,
    {
        self.0.serialize(v, data, pos)
    }
    fn serialize_gen(
        &self,
        v: Self::GType,
        data: &mut Vec<u8>,
        pos: usize,
    ) -> Result<usize, SerializeError> {
        self.0.serialize_gen(v, data, pos)
    }
    fn generate(&mut self, g: &mut GenSt) -> GResult<Self::GType, GenerateError> {
        self.0.generate(g)
    }
    fn well_formed<'s>(&self, v: Self::SType<'s>) -> bool
    where
        [u8]: 's,
    {
        self.0.well_formed(v)
    }
}

impl<C> Combinator<[u8], Vec<u8>> for Opaque0FfffffCombinator<C>
where
    C: Combinator<[u8], Vec<u8>>,
{
    type Type<'p> = C::Type<'p>;
    type SType<'s> = C::SType<'s>;
    type GType = C::GType;
    fn length<'s>(&self, v: Self::SType<'s>) -> usize
    where
        [u8]: 's,
    {
        self.0.length(v)
    }
    fn parse<'p>(&self, s: &'p [u8]) -> Result<(usize, Self::Type<'p>), ParseError>
    where
        [u8]: 'p,
    {
        self.0.parse(s)
    }
    fn serialize<'s>(
        &self,
        v: Self::SType<'s>,
        data: &mut Vec<u8>,
        pos: usize,
    ) -> Result<usize, SerializeError>
    where
        [u8]: 's,
    {
        self.0.serialize(v, data, pos)
    }
    fn serialize_gen(
        &self,
        v: Self::GType,
        data: &mut Vec<u8>,
        pos: usize,
    ) -> Result<usize, SerializeError> {
        self.0.serialize_gen(v, data, pos)
    }
    fn generate(&mut self, g: &mut GenSt) -> GResult<Self::GType, GenerateError> {
        self.0.generate(g)
    }
    fn well_formed<'s>(&self, v: Self::SType<'s>) -> bool
    where
        [u8]: 's,
    {
        self.0.well_formed(v)
    }
}

impl<C> Combinator<[u8], Vec<u8>> for ClientCertTypeClientExtensionCombinator<C>
where
    C: Combinator<[u8], Vec<u8>>,
{
    type Type<'p> = C::Type<'p>;
    type SType<'s> = C::SType<'s>;
    type GType = C::GType;
    fn length<'s>(&self, v: Self::SType<'s>) -> usize
    where
        [u8]: 's,
    {
        self.0.length(v)
    }
    fn parse<'p>(&self, s: &'p [u8]) -> Result<(usize, Self::Type<'p>), ParseError>
    where
        [u8]: 'p,
    {
        self.0.parse(s)
    }
    fn serialize<'s>(
        &self,
        v: Self::SType<'s>,
        data: &mut Vec<u8>,
        pos: usize,
    ) -> Result<usize, SerializeError>
    where
        [u8]: 's,
    {
        self.0.serialize(v, data, pos)
    }
    fn serialize_gen(
        &self,
        v: Self::GType,
        data: &mut Vec<u8>,
        pos: usize,
    ) -> Result<usize, SerializeError> {
        self.0.serialize_gen(v, data, pos)
    }
    fn generate(&mut self, g: &mut GenSt) -> GResult<Self::GType, GenerateError> {
        self.0.generate(g)
    }
    fn well_formed<'s>(&self, v: Self::SType<'s>) -> bool
    where
        [u8]: 's,
    {
        self.0.well_formed(v)
    }
}

impl<C> Combinator<[u8], Vec<u8>> for HeartbeatModeCombinator<C>
where
    C: Combinator<[u8], Vec<u8>>,
{
    type Type<'p> = C::Type<'p>;
    type SType<'s> = C::SType<'s>;
    type GType = C::GType;
    fn length<'s>(&self, v: Self::SType<'s>) -> usize
    where
        [u8]: 's,
    {
        self.0.length(v)
    }
    fn parse<'p>(&self, s: &'p [u8]) -> Result<(usize, Self::Type<'p>), ParseError>
    where
        [u8]: 'p,
    {
        self.0.parse(s)
    }
    fn serialize<'s>(
        &self,
        v: Self::SType<'s>,
        data: &mut Vec<u8>,
        pos: usize,
    ) -> Result<usize, SerializeError>
    where
        [u8]: 's,
    {
        self.0.serialize(v, data, pos)
    }
    fn serialize_gen(
        &self,
        v: Self::GType,
        data: &mut Vec<u8>,
        pos: usize,
    ) -> Result<usize, SerializeError> {
        self.0.serialize_gen(v, data, pos)
    }
    fn generate(&mut self, g: &mut GenSt) -> GResult<Self::GType, GenerateError> {
        self.0.generate(g)
    }
    fn well_formed<'s>(&self, v: Self::SType<'s>) -> bool
    where
        [u8]: 's,
    {
        self.0.well_formed(v)
    }
}

impl<C> Combinator<[u8], Vec<u8>> for HeartbeatExtensionCombinator<C>
where
    C: Combinator<[u8], Vec<u8>>,
{
    type Type<'p> = C::Type<'p>;
    type SType<'s> = C::SType<'s>;
    type GType = C::GType;
    fn length<'s>(&self, v: Self::SType<'s>) -> usize
    where
        [u8]: 's,
    {
        self.0.length(v)
    }
    fn parse<'p>(&self, s: &'p [u8]) -> Result<(usize, Self::Type<'p>), ParseError>
    where
        [u8]: 'p,
    {
        self.0.parse(s)
    }
    fn serialize<'s>(
        &self,
        v: Self::SType<'s>,
        data: &mut Vec<u8>,
        pos: usize,
    ) -> Result<usize, SerializeError>
    where
        [u8]: 's,
    {
        self.0.serialize(v, data, pos)
    }
    fn serialize_gen(
        &self,
        v: Self::GType,
        data: &mut Vec<u8>,
        pos: usize,
    ) -> Result<usize, SerializeError> {
        self.0.serialize_gen(v, data, pos)
    }
    fn generate(&mut self, g: &mut GenSt) -> GResult<Self::GType, GenerateError> {
        self.0.generate(g)
    }
    fn well_formed<'s>(&self, v: Self::SType<'s>) -> bool
    where
        [u8]: 's,
    {
        self.0.well_formed(v)
    }
}

impl<C> Combinator<[u8], Vec<u8>> for Opaque1FfffffCombinator<C>
where
    C: Combinator<[u8], Vec<u8>>,
{
    type Type<'p> = C::Type<'p>;
    type SType<'s> = C::SType<'s>;
    type GType = C::GType;
    fn length<'s>(&self, v: Self::SType<'s>) -> usize
    where
        [u8]: 's,
    {
        self.0.length(v)
    }
    fn parse<'p>(&self, s: &'p [u8]) -> Result<(usize, Self::Type<'p>), ParseError>
    where
        [u8]: 'p,
    {
        self.0.parse(s)
    }
    fn serialize<'s>(
        &self,
        v: Self::SType<'s>,
        data: &mut Vec<u8>,
        pos: usize,
    ) -> Result<usize, SerializeError>
    where
        [u8]: 's,
    {
        self.0.serialize(v, data, pos)
    }
    fn serialize_gen(
        &self,
        v: Self::GType,
        data: &mut Vec<u8>,
        pos: usize,
    ) -> Result<usize, SerializeError> {
        self.0.serialize_gen(v, data, pos)
    }
    fn generate(&mut self, g: &mut GenSt) -> GResult<Self::GType, GenerateError> {
        self.0.generate(g)
    }
    fn well_formed<'s>(&self, v: Self::SType<'s>) -> bool
    where
        [u8]: 's,
    {
        self.0.well_formed(v)
    }
}

impl<C> Combinator<[u8], Vec<u8>> for OcspResponseCombinator<C>
where
    C: Combinator<[u8], Vec<u8>>,
{
    type Type<'p> = C::Type<'p>;
    type SType<'s> = C::SType<'s>;
    type GType = C::GType;
    fn length<'s>(&self, v: Self::SType<'s>) -> usize
    where
        [u8]: 's,
    {
        self.0.length(v)
    }
    fn parse<'p>(&self, s: &'p [u8]) -> Result<(usize, Self::Type<'p>), ParseError>
    where
        [u8]: 'p,
    {
        self.0.parse(s)
    }
    fn serialize<'s>(
        &self,
        v: Self::SType<'s>,
        data: &mut Vec<u8>,
        pos: usize,
    ) -> Result<usize, SerializeError>
    where
        [u8]: 's,
    {
        self.0.serialize(v, data, pos)
    }
    fn serialize_gen(
        &self,
        v: Self::GType,
        data: &mut Vec<u8>,
        pos: usize,
    ) -> Result<usize, SerializeError> {
        self.0.serialize_gen(v, data, pos)
    }
    fn generate(&mut self, g: &mut GenSt) -> GResult<Self::GType, GenerateError> {
        self.0.generate(g)
    }
    fn well_formed<'s>(&self, v: Self::SType<'s>) -> bool
    where
        [u8]: 's,
    {
        self.0.well_formed(v)
    }
}

impl<C> Combinator<[u8], Vec<u8>> for CertificateStatusCombinator<C>
where
    C: Combinator<[u8], Vec<u8>>,
{
    type Type<'p> = C::Type<'p>;
    type SType<'s> = C::SType<'s>;
    type GType = C::GType;
    fn length<'s>(&self, v: Self::SType<'s>) -> usize
    where
        [u8]: 's,
    {
        self.0.length(v)
    }
    fn parse<'p>(&self, s: &'p [u8]) -> Result<(usize, Self::Type<'p>), ParseError>
    where
        [u8]: 'p,
    {
        self.0.parse(s)
    }
    fn serialize<'s>(
        &self,
        v: Self::SType<'s>,
        data: &mut Vec<u8>,
        pos: usize,
    ) -> Result<usize, SerializeError>
    where
        [u8]: 's,
    {
        self.0.serialize(v, data, pos)
    }
    fn serialize_gen(
        &self,
        v: Self::GType,
        data: &mut Vec<u8>,
        pos: usize,
    ) -> Result<usize, SerializeError> {
        self.0.serialize_gen(v, data, pos)
    }
    fn generate(&mut self, g: &mut GenSt) -> GResult<Self::GType, GenerateError> {
        self.0.generate(g)
    }
    fn well_formed<'s>(&self, v: Self::SType<'s>) -> bool
    where
        [u8]: 's,
    {
        self.0.well_formed(v)
    }
}

impl<C> Combinator<[u8], Vec<u8>> for EcPointFormatCombinator<C>
where
    C: Combinator<[u8], Vec<u8>>,
{
    type Type<'p> = C::Type<'p>;
    type SType<'s> = C::SType<'s>;
    type GType = C::GType;
    fn length<'s>(&self, v: Self::SType<'s>) -> usize
    where
        [u8]: 's,
    {
        self.0.length(v)
    }
    fn parse<'p>(&self, s: &'p [u8]) -> Result<(usize, Self::Type<'p>), ParseError>
    where
        [u8]: 'p,
    {
        self.0.parse(s)
    }
    fn serialize<'s>(
        &self,
        v: Self::SType<'s>,
        data: &mut Vec<u8>,
        pos: usize,
    ) -> Result<usize, SerializeError>
    where
        [u8]: 's,
    {
        self.0.serialize(v, data, pos)
    }
    fn serialize_gen(
        &self,
        v: Self::GType,
        data: &mut Vec<u8>,
        pos: usize,
    ) -> Result<usize, SerializeError> {
        self.0.serialize_gen(v, data, pos)
    }
    fn generate(&mut self, g: &mut GenSt) -> GResult<Self::GType, GenerateError> {
        self.0.generate(g)
    }
    fn well_formed<'s>(&self, v: Self::SType<'s>) -> bool
    where
        [u8]: 's,
    {
        self.0.well_formed(v)
    }
}

impl<C> Combinator<[u8], Vec<u8>> for CertificateStatusRequestCombinator<C>
where
    C: Combinator<[u8], Vec<u8>>,
{
    type Type<'p> = C::Type<'p>;
    type SType<'s> = C::SType<'s>;
    type GType = C::GType;
    fn length<'s>(&self, v: Self::SType<'s>) -> usize
    where
        [u8]: 's,
    {
        self.0.length(v)
    }
    fn parse<'p>(&self, s: &'p [u8]) -> Result<(usize, Self::Type<'p>), ParseError>
    where
        [u8]: 'p,
    {
        self.0.parse(s)
    }
    fn serialize<'s>(
        &self,
        v: Self::SType<'s>,
        data: &mut Vec<u8>,
        pos: usize,
    ) -> Result<usize, SerializeError>
    where
        [u8]: 's,
    {
        self.0.serialize(v, data, pos)
    }
    fn serialize_gen(
        &self,
        v: Self::GType,
        data: &mut Vec<u8>,
        pos: usize,
    ) -> Result<usize, SerializeError> {
        self.0.serialize_gen(v, data, pos)
    }
    fn generate(&mut self, g: &mut GenSt) -> GResult<Self::GType, GenerateError> {
        self.0.generate(g)
    }
    fn well_formed<'s>(&self, v: Self::SType<'s>) -> bool
    where
        [u8]: 's,
    {
        self.0.well_formed(v)
    }
}

impl<C> Combinator<[u8], Vec<u8>> for EcPointFormatListCombinator<C>
where
    C: Combinator<[u8], Vec<u8>>,
{
    type Type<'p> = C::Type<'p>;
    type SType<'s> = C::SType<'s>;
    type GType = C::GType;
    fn length<'s>(&self, v: Self::SType<'s>) -> usize
    where
        [u8]: 's,
    {
        self.0.length(v)
    }
    fn parse<'p>(&self, s: &'p [u8]) -> Result<(usize, Self::Type<'p>), ParseError>
    where
        [u8]: 'p,
    {
        self.0.parse(s)
    }
    fn serialize<'s>(
        &self,
        v: Self::SType<'s>,
        data: &mut Vec<u8>,
        pos: usize,
    ) -> Result<usize, SerializeError>
    where
        [u8]: 's,
    {
        self.0.serialize(v, data, pos)
    }
    fn serialize_gen(
        &self,
        v: Self::GType,
        data: &mut Vec<u8>,
        pos: usize,
    ) -> Result<usize, SerializeError> {
        self.0.serialize_gen(v, data, pos)
    }
    fn generate(&mut self, g: &mut GenSt) -> GResult<Self::GType, GenerateError> {
        self.0.generate(g)
    }
    fn well_formed<'s>(&self, v: Self::SType<'s>) -> bool
    where
        [u8]: 's,
    {
        self.0.well_formed(v)
    }
}

impl<C> Combinator<[u8], Vec<u8>> for ProtocolNameListCombinator<C>
where
    C: Combinator<[u8], Vec<u8>>,
{
    type Type<'p> = C::Type<'p>;
    type SType<'s> = C::SType<'s>;
    type GType = C::GType;
    fn length<'s>(&self, v: Self::SType<'s>) -> usize
    where
        [u8]: 's,
    {
        self.0.length(v)
    }
    fn parse<'p>(&self, s: &'p [u8]) -> Result<(usize, Self::Type<'p>), ParseError>
    where
        [u8]: 'p,
    {
        self.0.parse(s)
    }
    fn serialize<'s>(
        &self,
        v: Self::SType<'s>,
        data: &mut Vec<u8>,
        pos: usize,
    ) -> Result<usize, SerializeError>
    where
        [u8]: 's,
    {
        self.0.serialize(v, data, pos)
    }
    fn serialize_gen(
        &self,
        v: Self::GType,
        data: &mut Vec<u8>,
        pos: usize,
    ) -> Result<usize, SerializeError> {
        self.0.serialize_gen(v, data, pos)
    }
    fn generate(&mut self, g: &mut GenSt) -> GResult<Self::GType, GenerateError> {
        self.0.generate(g)
    }
    fn well_formed<'s>(&self, v: Self::SType<'s>) -> bool
    where
        [u8]: 's,
    {
        self.0.well_formed(v)
    }
}

pub trait RuntimeValParam<'a, T: Copy> {
    fn into_runtime_value(self) -> RuntimeValue<'a, T>;
}

pub trait LengthParam<'a, T: Copy> {
    fn into_length(self) -> Length<'a>;
}

impl<T: Copy> RuntimeValParam<'static, T> for T {
    fn into_runtime_value(self) -> RuntimeValue<'static, T> {
        RuntimeValue::from_value(self)
    }
}

impl<'a, T: Copy> RuntimeValParam<'a, T> for &'a mut T {
    fn into_runtime_value(self) -> RuntimeValue<'a, T> {
        RuntimeValue::from_mut(self)
    }
}

impl LengthParam<'static, u8> for u8 {
    fn into_length(self) -> Length<'static> {
        Length::from_value(self as usize)
    }
}

impl LengthParam<'static, u16> for u16 {
    fn into_length(self) -> Length<'static> {
        Length::from_value(self as usize)
    }
}

impl LengthParam<'static, u24> for u24 {
    fn into_length(self) -> Length<'static> {
        Length::from_value(self.as_u32() as usize)
    }
}

impl LengthParam<'static, u32> for u32 {
    fn into_length(self) -> Length<'static> {
        Length::from_value(self as usize)
    }
}

impl LengthParam<'static, u64> for u64 {
    fn into_length(self) -> Length<'static> {
        Length::from_value(self as usize)
    }
}

impl<'a> LengthParam<'a, u8> for &'a mut u8 {
    fn into_length(self) -> Length<'a> {
        Length::from_u8_mut(self)
    }
}

impl<'a> LengthParam<'a, u16> for &'a mut u16 {
    fn into_length(self) -> Length<'a> {
        Length::from_u16_mut(self)
    }
}

impl<'a> LengthParam<'a, u24> for &'a mut u24 {
    fn into_length(self) -> Length<'a> {
        Length::from_value(self.as_u32() as usize)
    }
}

impl<'a> LengthParam<'a, u32> for &'a mut u32 {
    fn into_length(self) -> Length<'a> {
        Length::from_u32_mut(self)
    }
}

impl<'a> LengthParam<'a, u64> for &'a mut u64 {
    fn into_length(self) -> Length<'a> {
        Length::from_u64_mut(self)
    }
}

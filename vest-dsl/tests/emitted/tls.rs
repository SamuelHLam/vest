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
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AlertLevel {
    Warning = 1,
    Fatal = 2,
}

pub type Empty<'a> = &'a [u8];
pub type EmptyOwned = Vec<u8>;
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
pub type ExtensionType = u16;
pub type SignatureScheme = u16;
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SignatureSchemeList {
    pub l: u16,
    pub list: Vec<SignatureScheme>,
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

pub type DistinguishedName<'a> = Opaque1Ffff<'a>;
pub type DistinguishedNameOwned = Opaque1FfffOwned;
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CertificateAuthoritiesExtension<'a> {
    pub l: u16,
    pub list: Vec<DistinguishedName<'a>>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CertificateAuthoritiesExtensionOwned {
    pub l: u16,
    pub list: Vec<DistinguishedNameOwned>,
}

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

pub type SerializedSct<'a> = Opaque1Ffff<'a>;
pub type SerializedSctOwned = Opaque1FfffOwned;
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
pub struct OidFilter<'a> {
    pub certificate_extension_oid: Opaque1Ff<'a>,
    pub certificate_extension_values: Opaque0Ffff<'a>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct OidFilterOwned {
    pub certificate_extension_oid: Opaque1FfOwned,
    pub certificate_extension_values: Opaque0FfffOwned,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct OidFilterExtension<'a> {
    pub l: u16,
    pub list: Vec<OidFilter<'a>>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct OidFilterExtensionOwned {
    pub l: u16,
    pub list: Vec<OidFilterOwned>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum CertificateRequestExtensionExtensionData<'a> {
    SignatureAlgorithms(SignatureSchemeList),
    CertificateAuthorities(CertificateAuthoritiesExtension<'a>),
    SignatureAlgorithmsCert(SignatureSchemeList),
    StatusRequest(CertificateStatusRequest<'a>),
    SignedCertificateTimeStamp(SignedCertificateTimestampList<'a>),
    OidFilters(OidFilterExtension<'a>),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum CertificateRequestExtensionExtensionDataOwned {
    SignatureAlgorithms(SignatureSchemeList),
    CertificateAuthorities(CertificateAuthoritiesExtensionOwned),
    SignatureAlgorithmsCert(SignatureSchemeList),
    StatusRequest(CertificateStatusRequestOwned),
    SignedCertificateTimeStamp(SignedCertificateTimestampListOwned),
    OidFilters(OidFilterExtensionOwned),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CertificateRequestExtension<'a> {
    pub extension_type: ExtensionType,
    pub ext_len: u16,
    pub extension_data: CertificateRequestExtensionExtensionData<'a>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CertificateRequestExtensionOwned {
    pub extension_type: ExtensionType,
    pub ext_len: u16,
    pub extension_data: CertificateRequestExtensionExtensionDataOwned,
}

pub type NameType = u8;
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ShOrHrr<'a> {
    pub legacy_version: u16,
    pub random: &'a [u8],
    pub payload: &'a [u8],
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ShOrHrrOwned {
    pub legacy_version: u16,
    pub random: Vec<u8>,
    pub payload: Vec<u8>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum HandshakeType {
    ClientHello = 1,
    ServerHello = 2,
    NewSessionTicket = 4,
    EndOfEarlyData = 5,
    EncryptedExtensions = 8,
    Certificate = 11,
    CertificateRequest = 13,
    CertificateVerify = 15,
    Finished = 20,
    KeyUpdate = 24,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SessionId<'a> {
    pub l: u8,
    pub id: &'a [u8],
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SessionIdOwned {
    pub l: u8,
    pub id: Vec<u8>,
}

pub type CipherSuite = u16;
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CipherSuiteList {
    pub l: u16,
    pub list: Vec<CipherSuite>,
}

pub type HostName<'a> = Opaque1Ffff<'a>;
pub type HostNameOwned = Opaque1FfffOwned;
pub type UnknownName<'a> = Opaque1Ffff<'a>;
pub type UnknownNameOwned = Opaque1FfffOwned;
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ServerNameName<'a> {
    HostName(HostName<'a>),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ServerNameNameOwned {
    HostName(HostNameOwned),
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

pub type NamedGroup = u16;
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct NamedGroupList {
    pub l: u16,
    pub list: Vec<NamedGroup>,
}

pub type ProtocolName<'a> = Opaque1Ff<'a>;
pub type ProtocolNameOwned = Opaque1FfOwned;
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

pub type ProtocolVersion = u16;
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SupportedVersionsClient {
    pub l: u8,
    pub list: Vec<ProtocolVersion>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct KeyShareEntry<'a> {
    pub group: NamedGroup,
    pub l: u16,
    pub key_exchange: &'a [u8],
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct KeyShareEntryOwned {
    pub group: NamedGroup,
    pub l: u16,
    pub key_exchange: Vec<u8>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct KeyShareClientHello<'a> {
    pub l: u16,
    pub list: Vec<KeyShareEntry<'a>>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct KeyShareClientHelloOwned {
    pub l: u16,
    pub list: Vec<KeyShareEntryOwned>,
}

pub type PskKeyExchangeMode = u8;
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PskKeyExchangeModes {
    pub l: u8,
    pub list: Vec<PskKeyExchangeMode>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PskIdentity<'a> {
    pub identity: Opaque1Ffff<'a>,
    pub obfuscated_ticket_age: u32,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PskIdentityOwned {
    pub identity: Opaque1FfffOwned,
    pub obfuscated_ticket_age: u32,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PskIdentities<'a> {
    pub l: u16,
    pub list: Vec<PskIdentity<'a>>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PskIdentitiesOwned {
    pub l: u16,
    pub list: Vec<PskIdentityOwned>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PskBinderEntry<'a> {
    pub l: u8,
    pub entries: &'a [u8],
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PskBinderEntryOwned {
    pub l: u8,
    pub entries: Vec<u8>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PskBinderEntries<'a> {
    pub l: u16,
    pub list: Vec<PskBinderEntry<'a>>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PskBinderEntriesOwned {
    pub l: u16,
    pub list: Vec<PskBinderEntryOwned>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct OfferedPsks<'a> {
    pub identities: PskIdentities<'a>,
    pub binders: PskBinderEntries<'a>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct OfferedPsksOwned {
    pub identities: PskIdentitiesOwned,
    pub binders: PskBinderEntriesOwned,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PreSharedKeyClientExtension<'a> {
    pub offers: OfferedPsks<'a>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PreSharedKeyClientExtensionOwned {
    pub offers: OfferedPsksOwned,
}

pub type MaxFragmentLength = u8;
pub type HeartbeatMode = u8;
pub type CertificateType = u8;
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ClientCertTypeClientExtension {
    pub l: u8,
    pub list: Vec<CertificateType>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ServerCertTypeClientExtension {
    pub l: u8,
    pub list: Vec<CertificateType>,
}

pub type Cookie<'a> = Opaque1Ffff<'a>;
pub type CookieOwned = Opaque1FfffOwned;
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ClientHelloExtensionRest<'a> {
    MaxFragmentLength(MaxFragmentLength),
    Heartbeat(HeartbeatMode),
    SignedCertificateTimeStamp(SignedCertificateTimestampList<'a>),
    ClientCertificateType(ClientCertTypeClientExtension),
    ServerCertificateType(ServerCertTypeClientExtension),
    Padding(&'a [u8]),
    Cookie(Cookie<'a>),
    CertificateAuthorities(CertificateAuthoritiesExtension<'a>),
    OidFilters(OidFilterExtension<'a>),
    SignatureAlgorithmsCert(SignatureSchemeList),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ClientHelloExtensionRestOwned {
    MaxFragmentLength(MaxFragmentLength),
    Heartbeat(HeartbeatMode),
    SignedCertificateTimeStamp(SignedCertificateTimestampListOwned),
    ClientCertificateType(ClientCertTypeClientExtension),
    ServerCertificateType(ServerCertTypeClientExtension),
    Padding(Vec<u8>),
    Cookie(CookieOwned),
    CertificateAuthorities(CertificateAuthoritiesExtensionOwned),
    OidFilters(OidFilterExtensionOwned),
    SignatureAlgorithmsCert(SignatureSchemeList),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ClientHelloExtensionExtensionData<'a> {
    ServerName(ServerNameList<'a>),
    SignatureAlgorithms(SignatureSchemeList),
    SupportedGroups(NamedGroupList),
    StatusRequest(CertificateStatusRequest<'a>),
    ApplicationLayerProtocolNegotiation(ProtocolNameList<'a>),
    SupportedVersions(SupportedVersionsClient),
    KeyShare(KeyShareClientHello<'a>),
    PskKeyExchangeModes(PskKeyExchangeModes),
    PreSharedKey(PreSharedKeyClientExtension<'a>),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ClientHelloExtensionExtensionDataOwned {
    ServerName(ServerNameListOwned),
    SignatureAlgorithms(SignatureSchemeList),
    SupportedGroups(NamedGroupList),
    StatusRequest(CertificateStatusRequestOwned),
    ApplicationLayerProtocolNegotiation(ProtocolNameListOwned),
    SupportedVersions(SupportedVersionsClient),
    KeyShare(KeyShareClientHelloOwned),
    PskKeyExchangeModes(PskKeyExchangeModes),
    PreSharedKey(PreSharedKeyClientExtensionOwned),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ClientHelloExtension<'a> {
    pub extension_type: ExtensionType,
    pub ext_len: u16,
    pub extension_data: ClientHelloExtensionExtensionData<'a>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ClientHelloExtensionOwned {
    pub extension_type: ExtensionType,
    pub ext_len: u16,
    pub extension_data: ClientHelloExtensionExtensionDataOwned,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ClientExtensions<'a> {
    pub l: u16,
    pub list: Vec<ClientHelloExtension<'a>>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ClientExtensionsOwned {
    pub l: u16,
    pub list: Vec<ClientHelloExtensionOwned>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ClientHello<'a> {
    pub legacy_version: u16,
    pub random: &'a [u8],
    pub legacy_session_id: SessionId<'a>,
    pub cipher_suites: CipherSuiteList,
    pub legacy_compression_methods: Opaque1Ff<'a>,
    pub extensions: ClientExtensions<'a>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ClientHelloOwned {
    pub legacy_version: u16,
    pub random: Vec<u8>,
    pub legacy_session_id: SessionIdOwned,
    pub cipher_suites: CipherSuiteList,
    pub legacy_compression_methods: Opaque1FfOwned,
    pub extensions: ClientExtensionsOwned,
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

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct EarlyDataIndicationNewSessionTicket {
    pub max_early_data_size: u32,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum NewSessionTicketExtensionExtensionData {
    EarlyData(EarlyDataIndicationNewSessionTicket),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct NewSessionTicketExtension {
    pub extension_type: ExtensionType,
    pub ext_len: u16,
    pub extension_data: NewSessionTicketExtensionExtensionData,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct NewSessionTicketExtensions {
    pub l: u16,
    pub list: Vec<NewSessionTicketExtension>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct NewSessionTicket<'a> {
    pub ticket_lifetime: u32,
    pub ticket_age_add: u32,
    pub ticket_nonce: Opaque0Ff<'a>,
    pub ticket: Opaque1Ffff<'a>,
    pub extensions: NewSessionTicketExtensions,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct NewSessionTicketOwned {
    pub ticket_lifetime: u32,
    pub ticket_age_add: u32,
    pub ticket_nonce: Opaque0FfOwned,
    pub ticket: Opaque1FfffOwned,
    pub extensions: NewSessionTicketExtensions,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum EncryptedExtensionExtensionData<'a> {
    ServerName(Empty<'a>),
    MaxFragmentLength(MaxFragmentLength),
    SupportedGroups(NamedGroupList),
    Heartbeat(HeartbeatMode),
    ApplicationLayerProtocolNegotiation(ProtocolNameList<'a>),
    ClientCertificateType(ClientCertTypeClientExtension),
    ServerCertificateType(ServerCertTypeClientExtension),
    EarlyData(Empty<'a>),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum EncryptedExtensionExtensionDataOwned {
    ServerName(EmptyOwned),
    MaxFragmentLength(MaxFragmentLength),
    SupportedGroups(NamedGroupList),
    Heartbeat(HeartbeatMode),
    ApplicationLayerProtocolNegotiation(ProtocolNameListOwned),
    ClientCertificateType(ClientCertTypeClientExtension),
    ServerCertificateType(ServerCertTypeClientExtension),
    EarlyData(EmptyOwned),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct EncryptedExtension<'a> {
    pub extension_type: ExtensionType,
    pub ext_len: u16,
    pub extension_data: EncryptedExtensionExtensionData<'a>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct EncryptedExtensionOwned {
    pub extension_type: ExtensionType,
    pub ext_len: u16,
    pub extension_data: EncryptedExtensionExtensionDataOwned,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct EncryptedExtensions<'a> {
    pub l: u16,
    pub list: Vec<EncryptedExtension<'a>>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct EncryptedExtensionsOwned {
    pub l: u16,
    pub list: Vec<EncryptedExtensionOwned>,
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

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum CertificateExtensionExtensionData<'a> {
    StatusRequest(CertificateStatus<'a>),
    SignedCertificateTimeStamp(SignedCertificateTimestampList<'a>),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum CertificateExtensionExtensionDataOwned {
    StatusRequest(CertificateStatusOwned),
    SignedCertificateTimeStamp(SignedCertificateTimestampListOwned),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CertificateExtension<'a> {
    pub extension_type: ExtensionType,
    pub ext_len: u16,
    pub extension_data: CertificateExtensionExtensionData<'a>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CertificateExtensionOwned {
    pub extension_type: ExtensionType,
    pub ext_len: u16,
    pub extension_data: CertificateExtensionExtensionDataOwned,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CertificateExtensions<'a> {
    pub l: u16,
    pub list: Vec<CertificateExtension<'a>>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CertificateExtensionsOwned {
    pub l: u16,
    pub list: Vec<CertificateExtensionOwned>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CertificateEntryOpaque<'a> {
    pub cert_data: Opaque1Ffffff<'a>,
    pub extensions: CertificateExtensions<'a>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CertificateEntryOpaqueOwned {
    pub cert_data: Opaque1FfffffOwned,
    pub extensions: CertificateExtensionsOwned,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CertificateList<'a> {
    pub l: u24,
    pub list: Vec<CertificateEntryOpaque<'a>>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CertificateListOwned {
    pub l: u24,
    pub list: Vec<CertificateEntryOpaqueOwned>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Certificate<'a> {
    pub certificate_request_context: Opaque0Ff<'a>,
    pub certificate_list: CertificateList<'a>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CertificateOwned {
    pub certificate_request_context: Opaque0FfOwned,
    pub certificate_list: CertificateListOwned,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CertificateRequestExtensions<'a> {
    pub l: u16,
    pub list: Vec<CertificateRequestExtension<'a>>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CertificateRequestExtensionsOwned {
    pub l: u16,
    pub list: Vec<CertificateRequestExtensionOwned>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CertificateRequest<'a> {
    pub certificate_request_context: Opaque0Ff<'a>,
    pub extensions: CertificateRequestExtensions<'a>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CertificateRequestOwned {
    pub certificate_request_context: Opaque0FfOwned,
    pub extensions: CertificateRequestExtensionsOwned,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CertificateVerify<'a> {
    pub algorithm: SignatureScheme,
    pub signature: Opaque0Ffff<'a>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CertificateVerifyOwned {
    pub algorithm: SignatureScheme,
    pub signature: Opaque0FfffOwned,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Finished<'a> {
    Hash12(&'a [u8]),
    Hash20(&'a [u8]),
    Sha256(&'a [u8]),
    Sha384(&'a [u8]),
    Sha512(&'a [u8]),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum FinishedOwned {
    Hash12(Vec<u8>),
    Hash20(Vec<u8>),
    Sha256(Vec<u8>),
    Sha384(Vec<u8>),
    Sha512(Vec<u8>),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum KeyUpdateRequest {
    UpdateNotRequested = 0,
    UpdateRequested = 1,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct KeyUpdate {
    pub request_update: KeyUpdateRequest,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum HandshakeMsg<'a> {
    ClientHello(ClientHello<'a>),
    ServerHello(ShOrHrr<'a>),
    NewSessionTicket(NewSessionTicket<'a>),
    EndOfEarlyData(Empty<'a>),
    EncryptedExtensions(EncryptedExtensions<'a>),
    Certificate(Certificate<'a>),
    CertificateRequest(CertificateRequest<'a>),
    CertificateVerify(CertificateVerify<'a>),
    Finished(Finished<'a>),
    KeyUpdate(KeyUpdate),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum HandshakeMsgOwned {
    ClientHello(ClientHelloOwned),
    ServerHello(ShOrHrrOwned),
    NewSessionTicket(NewSessionTicketOwned),
    EndOfEarlyData(EmptyOwned),
    EncryptedExtensions(EncryptedExtensionsOwned),
    Certificate(CertificateOwned),
    CertificateRequest(CertificateRequestOwned),
    CertificateVerify(CertificateVerifyOwned),
    Finished(FinishedOwned),
    KeyUpdate(KeyUpdate),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Handshake<'a> {
    pub msg_type: HandshakeType,
    pub length: u24,
    pub msg: HandshakeMsg<'a>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct HandshakeOwned {
    pub msg_type: HandshakeType,
    pub length: u24,
    pub msg: HandshakeMsgOwned,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ZeroByte {
    pub zero: u8,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PaddingExtension {
    pub l: u16,
    pub padding: Vec<ZeroByte>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Extension<'a> {
    pub extension_type: ExtensionType,
    pub extension_data: Opaque0Ffff<'a>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ExtensionOwned {
    pub extension_type: ExtensionType,
    pub extension_data: Opaque0FfffOwned,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ClientCertTypeServerExtension {
    pub client_certificate_type: CertificateType,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ContentType {
    Invalid = 0,
    ChangeCipherSpec = 20,
    Alert = 21,
    Handshake = 22,
    ApplicationData = 23,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TlsPlaintext<'a> {
    pub content_type: ContentType,
    pub legacy_record_version: ProtocolVersion,
    pub fragment: Opaque0Ffff<'a>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TlsPlaintextOwned {
    pub content_type: ContentType,
    pub legacy_record_version: ProtocolVersion,
    pub fragment: Opaque0FfffOwned,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AlertDescription {
    CloseNotify = 0,
    UnexpectedMessage = 10,
    BadRecordMac = 20,
    RecordOverflow = 22,
    HandshakeFailure = 40,
    BadCertificate = 42,
    UnsupportedCertificate = 43,
    CertificateRevoked = 44,
    CertificateExpired = 45,
    CertificateUnknown = 46,
    IllegalParameter = 47,
    UnknownCA = 48,
    AccessDenied = 49,
    DecodeError = 50,
    DecryptError = 51,
    ProtocolVersion = 70,
    InsufficientSecurity = 71,
    InternalError = 80,
    InappropriateFallback = 86,
    UserCanceled = 90,
    MissingExtension = 109,
    UnsupportedExtension = 110,
    UnrecognizedName = 112,
    BadCertificateStatusResponse = 113,
    UnknownPSKIdentity = 115,
    CertificateRequired = 116,
    NoApplicationProtocol = 120,
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
pub struct UseSrtpData<'a> {
    pub profiles: SrtpProtectionProfiles<'a>,
    pub srtp_mki: Opaque0Ff<'a>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct UseSrtpDataOwned {
    pub profiles: SrtpProtectionProfilesOwned,
    pub srtp_mki: Opaque0FfOwned,
}

pub type SupportedVersionsServer = ProtocolVersion;
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum HelloRetryExtensionExtensionData<'a> {
    SupportedVersions(SupportedVersionsServer),
    Cookie(Cookie<'a>),
    KeyShare(NamedGroup),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum HelloRetryExtensionExtensionDataOwned {
    SupportedVersions(SupportedVersionsServer),
    Cookie(CookieOwned),
    KeyShare(NamedGroup),
}

pub type FinishedOpaque<'a> = &'a [u8];
pub type FinishedOpaqueOwned = Vec<u8>;
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct PreSharedKeyServerExtension {
    pub selected_identity: u16,
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

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct HelloRetryExtension<'a> {
    pub extension_type: ExtensionType,
    pub ext_len: u16,
    pub extension_data: HelloRetryExtensionExtensionData<'a>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct HelloRetryExtensionOwned {
    pub extension_type: ExtensionType,
    pub ext_len: u16,
    pub extension_data: HelloRetryExtensionExtensionDataOwned,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Alert {
    pub level: AlertLevel,
    pub description: AlertDescription,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ServerCertTypeServerExtension {
    pub server_certificate_type: CertificateType,
}

pub type UnknownExtension<'a> = Opaque0Ffff<'a>;
pub type UnknownExtensionOwned = Opaque0FfffOwned;
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct HelloRetryExtensions<'a> {
    pub l: u16,
    pub list: Vec<HelloRetryExtension<'a>>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct HelloRetryExtensionsOwned {
    pub l: u16,
    pub list: Vec<HelloRetryExtensionOwned>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum SeverHelloExtensionExtensionData<'a> {
    PreSharedKey(PreSharedKeyServerExtension),
    SupportedVersions(SupportedVersionsServer),
    KeyShare(KeyShareEntry<'a>),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum SeverHelloExtensionExtensionDataOwned {
    PreSharedKey(PreSharedKeyServerExtension),
    SupportedVersions(SupportedVersionsServer),
    KeyShare(KeyShareEntryOwned),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SeverHelloExtension<'a> {
    pub extension_type: ExtensionType,
    pub ext_len: u16,
    pub extension_data: SeverHelloExtensionExtensionData<'a>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SeverHelloExtensionOwned {
    pub extension_type: ExtensionType,
    pub ext_len: u16,
    pub extension_data: SeverHelloExtensionExtensionDataOwned,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ServerExtensions<'a> {
    pub l: u16,
    pub list: Vec<SeverHelloExtension<'a>>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ServerExtensionsOwned {
    pub l: u16,
    pub list: Vec<SeverHelloExtensionOwned>,
}

pub type DigestSize = u24;
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct HeartbeatExtension {
    pub mode: HeartbeatMode,
}

pub type EcPointFormat = u8;
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ServerHello<'a> {
    pub legacy_session_id_echo: SessionId<'a>,
    pub cipher_suite: CipherSuite,
    pub legacy_compression_method: u8,
    pub extensions: ServerExtensions<'a>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ServerHelloOwned {
    pub legacy_session_id_echo: SessionIdOwned,
    pub cipher_suite: CipherSuite,
    pub legacy_compression_method: u8,
    pub extensions: ServerExtensionsOwned,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct EcPointFormatList {
    pub l: u8,
    pub list: Vec<EcPointFormat>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct HelloRetryRequest<'a> {
    pub legacy_session_id_echo: SessionId<'a>,
    pub cipher_suite: CipherSuite,
    pub legacy_compression_method: u8,
    pub extensions: HelloRetryExtensions<'a>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct HelloRetryRequestOwned {
    pub legacy_session_id_echo: SessionIdOwned,
    pub cipher_suite: CipherSuite,
    pub legacy_compression_method: u8,
    pub extensions: HelloRetryExtensionsOwned,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum CertificateEntryData<'a> {
    X509(Opaque1Ffffff<'a>),
    RawPublicKey(Opaque1Ffffff<'a>),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum CertificateEntryDataOwned {
    X509(Opaque1FfffffOwned),
    RawPublicKey(Opaque1FfffffOwned),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CertificateEntry<'a> {
    pub data: CertificateEntryData<'a>,
    pub extensions: CertificateExtensions<'a>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CertificateEntryOwned {
    pub data: CertificateEntryDataOwned,
    pub extensions: CertificateExtensionsOwned,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TlsCiphertext<'a> {
    pub opaque_type: ContentType,
    pub version: ProtocolVersion,
    pub encrypted_record: Opaque0Ffff<'a>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TlsCiphertextOwned {
    pub opaque_type: ContentType,
    pub version: ProtocolVersion,
    pub encrypted_record: Opaque0FfffOwned,
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

impl From<u8> for AlertLevel {
    fn from(src: u8) -> Self {
        match src as i128 {
            1 => Self::Warning,
            2 => Self::Fatal,
            _ => unreachable!("validated by combinator"),
        }
    }
}

impl From<AlertLevel> for u8 {
    fn from(v: AlertLevel) -> Self {
        v as u8
    }
}

pub struct AlertLevelMapper;
impl Mapper for AlertLevelMapper {
    type Src<'p> = u8;
    type Dst<'p> = AlertLevel;
    type SrcBorrow<'s> = u8;
    type DstBorrow<'s> = AlertLevel;
    type SrcOwned = u8;
    type DstOwned = AlertLevel;
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

impl<'a> From<(u16, Vec<DistinguishedName<'a>>)>
for CertificateAuthoritiesExtension<'a> {
    fn from(src: (u16, Vec<DistinguishedName<'a>>)) -> Self {
        Self { l: src.0, list: src.1 }
    }
}

impl<'s, 'a: 's> From<&'s CertificateAuthoritiesExtension<'a>>
for (u16, &'s [&'s DistinguishedName<'s>]) {
    fn from(v: &'s CertificateAuthoritiesExtension<'a>) -> Self {
        (v.l, v.list.as_slice())
    }
}

impl From<(u16, Vec<DistinguishedNameOwned>)> for CertificateAuthoritiesExtensionOwned {
    fn from(src: (u16, Vec<DistinguishedNameOwned>)) -> Self {
        Self { l: src.0, list: src.1 }
    }
}

impl From<CertificateAuthoritiesExtensionOwned> for (u16, Vec<DistinguishedNameOwned>) {
    fn from(v: CertificateAuthoritiesExtensionOwned) -> Self {
        (v.l, v.list)
    }
}

pub struct CertificateAuthoritiesExtensionMapper;
impl Mapper for CertificateAuthoritiesExtensionMapper {
    type Src<'p> = (u16, Vec<DistinguishedName<'p>>);
    type Dst<'p> = CertificateAuthoritiesExtension<'p>;
    type SrcBorrow<'s> = (u16, &'s [&'s DistinguishedName<'s>]);
    type DstBorrow<'s> = &'s CertificateAuthoritiesExtension<'s>;
    type SrcOwned = (u16, Vec<DistinguishedNameOwned>);
    type DstOwned = CertificateAuthoritiesExtensionOwned;
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
        (CERTIFICATE_STATUS_REQUESTSTATUS_TYPE_CONST, v.request)
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

impl<'a> From<(Opaque1Ff<'a>, Opaque0Ffff<'a>)> for OidFilter<'a> {
    fn from(src: (Opaque1Ff<'a>, Opaque0Ffff<'a>)) -> Self {
        Self {
            certificate_extension_oid: src.0,
            certificate_extension_values: src.1,
        }
    }
}

impl<'s, 'a: 's> From<&'s OidFilter<'a>> for (&'s Opaque1Ff<'s>, &'s Opaque0Ffff<'s>) {
    fn from(v: &'s OidFilter<'a>) -> Self {
        (&v.certificate_extension_oid, &v.certificate_extension_values)
    }
}

impl From<(Opaque1FfOwned, Opaque0FfffOwned)> for OidFilterOwned {
    fn from(src: (Opaque1FfOwned, Opaque0FfffOwned)) -> Self {
        Self {
            certificate_extension_oid: src.0,
            certificate_extension_values: src.1,
        }
    }
}

impl From<OidFilterOwned> for (Opaque1FfOwned, Opaque0FfffOwned) {
    fn from(v: OidFilterOwned) -> Self {
        (v.certificate_extension_oid, v.certificate_extension_values)
    }
}

pub struct OidFilterMapper;
impl Mapper for OidFilterMapper {
    type Src<'p> = (Opaque1Ff<'p>, Opaque0Ffff<'p>);
    type Dst<'p> = OidFilter<'p>;
    type SrcBorrow<'s> = (&'s Opaque1Ff<'s>, &'s Opaque0Ffff<'s>);
    type DstBorrow<'s> = &'s OidFilter<'s>;
    type SrcOwned = (Opaque1FfOwned, Opaque0FfffOwned);
    type DstOwned = OidFilterOwned;
}

impl<'a> From<(u16, Vec<OidFilter<'a>>)> for OidFilterExtension<'a> {
    fn from(src: (u16, Vec<OidFilter<'a>>)) -> Self {
        Self { l: src.0, list: src.1 }
    }
}

impl<'s, 'a: 's> From<&'s OidFilterExtension<'a>> for (u16, &'s [&'s OidFilter<'s>]) {
    fn from(v: &'s OidFilterExtension<'a>) -> Self {
        (v.l, v.list.as_slice())
    }
}

impl From<(u16, Vec<OidFilterOwned>)> for OidFilterExtensionOwned {
    fn from(src: (u16, Vec<OidFilterOwned>)) -> Self {
        Self { l: src.0, list: src.1 }
    }
}

impl From<OidFilterExtensionOwned> for (u16, Vec<OidFilterOwned>) {
    fn from(v: OidFilterExtensionOwned) -> Self {
        (v.l, v.list)
    }
}

pub struct OidFilterExtensionMapper;
impl Mapper for OidFilterExtensionMapper {
    type Src<'p> = (u16, Vec<OidFilter<'p>>);
    type Dst<'p> = OidFilterExtension<'p>;
    type SrcBorrow<'s> = (u16, &'s [&'s OidFilter<'s>]);
    type DstBorrow<'s> = &'s OidFilterExtension<'s>;
    type SrcOwned = (u16, Vec<OidFilterOwned>);
    type DstOwned = OidFilterExtensionOwned;
}

impl<'a> From<((ExtensionType, u16), CertificateRequestExtensionExtensionData<'a>)>
for CertificateRequestExtension<'a> {
    fn from(
        src: ((ExtensionType, u16), CertificateRequestExtensionExtensionData<'a>),
    ) -> Self {
        Self {
            extension_type: src.0.0,
            ext_len: src.0.1,
            extension_data: src.1,
        }
    }
}

impl<'s, 'a: 's> From<&'s CertificateRequestExtension<'a>>
for ((ExtensionType, u16), &'s CertificateRequestExtensionExtensionData<'s>) {
    fn from(v: &'s CertificateRequestExtension<'a>) -> Self {
        ((v.extension_type, v.ext_len), &v.extension_data)
    }
}

impl From<((ExtensionType, u16), CertificateRequestExtensionExtensionDataOwned)>
for CertificateRequestExtensionOwned {
    fn from(
        src: ((ExtensionType, u16), CertificateRequestExtensionExtensionDataOwned),
    ) -> Self {
        Self {
            extension_type: src.0.0,
            ext_len: src.0.1,
            extension_data: src.1,
        }
    }
}

impl From<CertificateRequestExtensionOwned>
for ((ExtensionType, u16), CertificateRequestExtensionExtensionDataOwned) {
    fn from(v: CertificateRequestExtensionOwned) -> Self {
        ((v.extension_type, v.ext_len), v.extension_data)
    }
}

pub struct CertificateRequestExtensionMapper;
impl Mapper for CertificateRequestExtensionMapper {
    type Src<'p> = ((ExtensionType, u16), CertificateRequestExtensionExtensionData<'p>);
    type Dst<'p> = CertificateRequestExtension<'p>;
    type SrcBorrow<'s> = (
        (ExtensionType, u16),
        &'s CertificateRequestExtensionExtensionData<'s>,
    );
    type DstBorrow<'s> = &'s CertificateRequestExtension<'s>;
    type SrcOwned = (
        (ExtensionType, u16),
        CertificateRequestExtensionExtensionDataOwned,
    );
    type DstOwned = CertificateRequestExtensionOwned;
}

impl<'a> From<((), (&'a [u8], &'a [u8]))> for ShOrHrr<'a> {
    fn from(src: ((), (&'a [u8], &'a [u8]))) -> Self {
        Self {
            legacy_version: SH_OR_HRRLEGACY_VERSION_CONST,
            random: src.1.0,
            payload: src.1.1,
        }
    }
}

impl<'s, 'a: 's> From<&'s ShOrHrr<'a>> for ((), (&'s [u8], &'s [u8])) {
    fn from(v: &'s ShOrHrr<'a>) -> Self {
        ((), (v.random, v.payload))
    }
}

impl From<((), (Vec<u8>, Vec<u8>))> for ShOrHrrOwned {
    fn from(src: ((), (Vec<u8>, Vec<u8>))) -> Self {
        Self {
            legacy_version: SH_OR_HRRLEGACY_VERSION_CONST,
            random: src.1.0,
            payload: src.1.1,
        }
    }
}

impl From<ShOrHrrOwned> for ((), (Vec<u8>, Vec<u8>)) {
    fn from(v: ShOrHrrOwned) -> Self {
        (SH_OR_HRRLEGACY_VERSION_CONST, (v.random, v.payload))
    }
}

pub struct ShOrHrrMapper;
impl Mapper for ShOrHrrMapper {
    type Src<'p> = ((), (&'p [u8], &'p [u8]));
    type Dst<'p> = ShOrHrr<'p>;
    type SrcBorrow<'s> = ((), (&'s [u8], &'s [u8]));
    type DstBorrow<'s> = &'s ShOrHrr<'s>;
    type SrcOwned = ((), (Vec<u8>, Vec<u8>));
    type DstOwned = ShOrHrrOwned;
}

impl From<u8> for HandshakeType {
    fn from(src: u8) -> Self {
        match src as i128 {
            1 => Self::ClientHello,
            2 => Self::ServerHello,
            4 => Self::NewSessionTicket,
            5 => Self::EndOfEarlyData,
            8 => Self::EncryptedExtensions,
            11 => Self::Certificate,
            13 => Self::CertificateRequest,
            15 => Self::CertificateVerify,
            20 => Self::Finished,
            24 => Self::KeyUpdate,
            _ => unreachable!("validated by combinator"),
        }
    }
}

impl From<HandshakeType> for u8 {
    fn from(v: HandshakeType) -> Self {
        v as u8
    }
}

pub struct HandshakeTypeMapper;
impl Mapper for HandshakeTypeMapper {
    type Src<'p> = u8;
    type Dst<'p> = HandshakeType;
    type SrcBorrow<'s> = u8;
    type DstBorrow<'s> = HandshakeType;
    type SrcOwned = u8;
    type DstOwned = HandshakeType;
}

impl<'a> From<(u8, &'a [u8])> for SessionId<'a> {
    fn from(src: (u8, &'a [u8])) -> Self {
        Self { l: src.0, id: src.1 }
    }
}

impl<'s, 'a: 's> From<&'s SessionId<'a>> for (u8, &'s [u8]) {
    fn from(v: &'s SessionId<'a>) -> Self {
        (v.l, v.id)
    }
}

impl From<(u8, Vec<u8>)> for SessionIdOwned {
    fn from(src: (u8, Vec<u8>)) -> Self {
        Self { l: src.0, id: src.1 }
    }
}

impl From<SessionIdOwned> for (u8, Vec<u8>) {
    fn from(v: SessionIdOwned) -> Self {
        (v.l, v.id)
    }
}

pub struct SessionIdMapper;
impl Mapper for SessionIdMapper {
    type Src<'p> = (u8, &'p [u8]);
    type Dst<'p> = SessionId<'p>;
    type SrcBorrow<'s> = (u8, &'s [u8]);
    type DstBorrow<'s> = &'s SessionId<'s>;
    type SrcOwned = (u8, Vec<u8>);
    type DstOwned = SessionIdOwned;
}

impl From<(u16, Vec<CipherSuite>)> for CipherSuiteList {
    fn from(src: (u16, Vec<CipherSuite>)) -> Self {
        Self { l: src.0, list: src.1 }
    }
}

impl<'s> From<&'s CipherSuiteList> for (u16, &'s [CipherSuite]) {
    fn from(v: &'s CipherSuiteList) -> Self {
        (v.l, v.list.as_slice())
    }
}

impl<'a> From<CipherSuiteList> for (u16, Vec<CipherSuite>) {
    fn from(v: CipherSuiteList) -> Self {
        (v.l, v.list)
    }
}

pub struct CipherSuiteListMapper;
impl Mapper for CipherSuiteListMapper {
    type Src<'p> = (u16, Vec<CipherSuite>);
    type Dst<'p> = CipherSuiteList;
    type SrcBorrow<'s> = (u16, &'s [CipherSuite]);
    type DstBorrow<'s> = &'s CipherSuiteList;
    type SrcOwned = (u16, Vec<CipherSuite>);
    type DstOwned = CipherSuiteList;
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

impl From<(u8, Vec<ProtocolVersion>)> for SupportedVersionsClient {
    fn from(src: (u8, Vec<ProtocolVersion>)) -> Self {
        Self { l: src.0, list: src.1 }
    }
}

impl<'s> From<&'s SupportedVersionsClient> for (u8, &'s [ProtocolVersion]) {
    fn from(v: &'s SupportedVersionsClient) -> Self {
        (v.l, v.list.as_slice())
    }
}

impl<'a> From<SupportedVersionsClient> for (u8, Vec<ProtocolVersion>) {
    fn from(v: SupportedVersionsClient) -> Self {
        (v.l, v.list)
    }
}

pub struct SupportedVersionsClientMapper;
impl Mapper for SupportedVersionsClientMapper {
    type Src<'p> = (u8, Vec<ProtocolVersion>);
    type Dst<'p> = SupportedVersionsClient;
    type SrcBorrow<'s> = (u8, &'s [ProtocolVersion]);
    type DstBorrow<'s> = &'s SupportedVersionsClient;
    type SrcOwned = (u8, Vec<ProtocolVersion>);
    type DstOwned = SupportedVersionsClient;
}

impl<'a> From<((NamedGroup, u16), &'a [u8])> for KeyShareEntry<'a> {
    fn from(src: ((NamedGroup, u16), &'a [u8])) -> Self {
        Self {
            group: src.0.0,
            l: src.0.1,
            key_exchange: src.1,
        }
    }
}

impl<'s, 'a: 's> From<&'s KeyShareEntry<'a>> for ((NamedGroup, u16), &'s [u8]) {
    fn from(v: &'s KeyShareEntry<'a>) -> Self {
        ((v.group, v.l), v.key_exchange)
    }
}

impl From<((NamedGroup, u16), Vec<u8>)> for KeyShareEntryOwned {
    fn from(src: ((NamedGroup, u16), Vec<u8>)) -> Self {
        Self {
            group: src.0.0,
            l: src.0.1,
            key_exchange: src.1,
        }
    }
}

impl From<KeyShareEntryOwned> for ((NamedGroup, u16), Vec<u8>) {
    fn from(v: KeyShareEntryOwned) -> Self {
        ((v.group, v.l), v.key_exchange)
    }
}

pub struct KeyShareEntryMapper;
impl Mapper for KeyShareEntryMapper {
    type Src<'p> = ((NamedGroup, u16), &'p [u8]);
    type Dst<'p> = KeyShareEntry<'p>;
    type SrcBorrow<'s> = ((NamedGroup, u16), &'s [u8]);
    type DstBorrow<'s> = &'s KeyShareEntry<'s>;
    type SrcOwned = ((NamedGroup, u16), Vec<u8>);
    type DstOwned = KeyShareEntryOwned;
}

impl<'a> From<(u16, Vec<KeyShareEntry<'a>>)> for KeyShareClientHello<'a> {
    fn from(src: (u16, Vec<KeyShareEntry<'a>>)) -> Self {
        Self { l: src.0, list: src.1 }
    }
}

impl<'s, 'a: 's> From<&'s KeyShareClientHello<'a>>
for (u16, &'s [&'s KeyShareEntry<'s>]) {
    fn from(v: &'s KeyShareClientHello<'a>) -> Self {
        (v.l, v.list.as_slice())
    }
}

impl From<(u16, Vec<KeyShareEntryOwned>)> for KeyShareClientHelloOwned {
    fn from(src: (u16, Vec<KeyShareEntryOwned>)) -> Self {
        Self { l: src.0, list: src.1 }
    }
}

impl From<KeyShareClientHelloOwned> for (u16, Vec<KeyShareEntryOwned>) {
    fn from(v: KeyShareClientHelloOwned) -> Self {
        (v.l, v.list)
    }
}

pub struct KeyShareClientHelloMapper;
impl Mapper for KeyShareClientHelloMapper {
    type Src<'p> = (u16, Vec<KeyShareEntry<'p>>);
    type Dst<'p> = KeyShareClientHello<'p>;
    type SrcBorrow<'s> = (u16, &'s [&'s KeyShareEntry<'s>]);
    type DstBorrow<'s> = &'s KeyShareClientHello<'s>;
    type SrcOwned = (u16, Vec<KeyShareEntryOwned>);
    type DstOwned = KeyShareClientHelloOwned;
}

impl From<(u8, Vec<PskKeyExchangeMode>)> for PskKeyExchangeModes {
    fn from(src: (u8, Vec<PskKeyExchangeMode>)) -> Self {
        Self { l: src.0, list: src.1 }
    }
}

impl<'s> From<&'s PskKeyExchangeModes> for (u8, &'s [PskKeyExchangeMode]) {
    fn from(v: &'s PskKeyExchangeModes) -> Self {
        (v.l, v.list.as_slice())
    }
}

impl<'a> From<PskKeyExchangeModes> for (u8, Vec<PskKeyExchangeMode>) {
    fn from(v: PskKeyExchangeModes) -> Self {
        (v.l, v.list)
    }
}

pub struct PskKeyExchangeModesMapper;
impl Mapper for PskKeyExchangeModesMapper {
    type Src<'p> = (u8, Vec<PskKeyExchangeMode>);
    type Dst<'p> = PskKeyExchangeModes;
    type SrcBorrow<'s> = (u8, &'s [PskKeyExchangeMode]);
    type DstBorrow<'s> = &'s PskKeyExchangeModes;
    type SrcOwned = (u8, Vec<PskKeyExchangeMode>);
    type DstOwned = PskKeyExchangeModes;
}

impl<'a> From<(Opaque1Ffff<'a>, u32)> for PskIdentity<'a> {
    fn from(src: (Opaque1Ffff<'a>, u32)) -> Self {
        Self {
            identity: src.0,
            obfuscated_ticket_age: src.1,
        }
    }
}

impl<'s, 'a: 's> From<&'s PskIdentity<'a>> for (&'s Opaque1Ffff<'s>, u32) {
    fn from(v: &'s PskIdentity<'a>) -> Self {
        (&v.identity, v.obfuscated_ticket_age)
    }
}

impl From<(Opaque1FfffOwned, u32)> for PskIdentityOwned {
    fn from(src: (Opaque1FfffOwned, u32)) -> Self {
        Self {
            identity: src.0,
            obfuscated_ticket_age: src.1,
        }
    }
}

impl From<PskIdentityOwned> for (Opaque1FfffOwned, u32) {
    fn from(v: PskIdentityOwned) -> Self {
        (v.identity, v.obfuscated_ticket_age)
    }
}

pub struct PskIdentityMapper;
impl Mapper for PskIdentityMapper {
    type Src<'p> = (Opaque1Ffff<'p>, u32);
    type Dst<'p> = PskIdentity<'p>;
    type SrcBorrow<'s> = (&'s Opaque1Ffff<'s>, u32);
    type DstBorrow<'s> = &'s PskIdentity<'s>;
    type SrcOwned = (Opaque1FfffOwned, u32);
    type DstOwned = PskIdentityOwned;
}

impl<'a> From<(u16, Vec<PskIdentity<'a>>)> for PskIdentities<'a> {
    fn from(src: (u16, Vec<PskIdentity<'a>>)) -> Self {
        Self { l: src.0, list: src.1 }
    }
}

impl<'s, 'a: 's> From<&'s PskIdentities<'a>> for (u16, &'s [&'s PskIdentity<'s>]) {
    fn from(v: &'s PskIdentities<'a>) -> Self {
        (v.l, v.list.as_slice())
    }
}

impl From<(u16, Vec<PskIdentityOwned>)> for PskIdentitiesOwned {
    fn from(src: (u16, Vec<PskIdentityOwned>)) -> Self {
        Self { l: src.0, list: src.1 }
    }
}

impl From<PskIdentitiesOwned> for (u16, Vec<PskIdentityOwned>) {
    fn from(v: PskIdentitiesOwned) -> Self {
        (v.l, v.list)
    }
}

pub struct PskIdentitiesMapper;
impl Mapper for PskIdentitiesMapper {
    type Src<'p> = (u16, Vec<PskIdentity<'p>>);
    type Dst<'p> = PskIdentities<'p>;
    type SrcBorrow<'s> = (u16, &'s [&'s PskIdentity<'s>]);
    type DstBorrow<'s> = &'s PskIdentities<'s>;
    type SrcOwned = (u16, Vec<PskIdentityOwned>);
    type DstOwned = PskIdentitiesOwned;
}

impl<'a> From<(u8, &'a [u8])> for PskBinderEntry<'a> {
    fn from(src: (u8, &'a [u8])) -> Self {
        Self { l: src.0, entries: src.1 }
    }
}

impl<'s, 'a: 's> From<&'s PskBinderEntry<'a>> for (u8, &'s [u8]) {
    fn from(v: &'s PskBinderEntry<'a>) -> Self {
        (v.l, v.entries)
    }
}

impl From<(u8, Vec<u8>)> for PskBinderEntryOwned {
    fn from(src: (u8, Vec<u8>)) -> Self {
        Self { l: src.0, entries: src.1 }
    }
}

impl From<PskBinderEntryOwned> for (u8, Vec<u8>) {
    fn from(v: PskBinderEntryOwned) -> Self {
        (v.l, v.entries)
    }
}

pub struct PskBinderEntryMapper;
impl Mapper for PskBinderEntryMapper {
    type Src<'p> = (u8, &'p [u8]);
    type Dst<'p> = PskBinderEntry<'p>;
    type SrcBorrow<'s> = (u8, &'s [u8]);
    type DstBorrow<'s> = &'s PskBinderEntry<'s>;
    type SrcOwned = (u8, Vec<u8>);
    type DstOwned = PskBinderEntryOwned;
}

impl<'a> From<(u16, Vec<PskBinderEntry<'a>>)> for PskBinderEntries<'a> {
    fn from(src: (u16, Vec<PskBinderEntry<'a>>)) -> Self {
        Self { l: src.0, list: src.1 }
    }
}

impl<'s, 'a: 's> From<&'s PskBinderEntries<'a>> for (u16, &'s [&'s PskBinderEntry<'s>]) {
    fn from(v: &'s PskBinderEntries<'a>) -> Self {
        (v.l, v.list.as_slice())
    }
}

impl From<(u16, Vec<PskBinderEntryOwned>)> for PskBinderEntriesOwned {
    fn from(src: (u16, Vec<PskBinderEntryOwned>)) -> Self {
        Self { l: src.0, list: src.1 }
    }
}

impl From<PskBinderEntriesOwned> for (u16, Vec<PskBinderEntryOwned>) {
    fn from(v: PskBinderEntriesOwned) -> Self {
        (v.l, v.list)
    }
}

pub struct PskBinderEntriesMapper;
impl Mapper for PskBinderEntriesMapper {
    type Src<'p> = (u16, Vec<PskBinderEntry<'p>>);
    type Dst<'p> = PskBinderEntries<'p>;
    type SrcBorrow<'s> = (u16, &'s [&'s PskBinderEntry<'s>]);
    type DstBorrow<'s> = &'s PskBinderEntries<'s>;
    type SrcOwned = (u16, Vec<PskBinderEntryOwned>);
    type DstOwned = PskBinderEntriesOwned;
}

impl<'a> From<(PskIdentities<'a>, PskBinderEntries<'a>)> for OfferedPsks<'a> {
    fn from(src: (PskIdentities<'a>, PskBinderEntries<'a>)) -> Self {
        Self {
            identities: src.0,
            binders: src.1,
        }
    }
}

impl<'s, 'a: 's> From<&'s OfferedPsks<'a>>
for (&'s PskIdentities<'s>, &'s PskBinderEntries<'s>) {
    fn from(v: &'s OfferedPsks<'a>) -> Self {
        (&v.identities, &v.binders)
    }
}

impl From<(PskIdentitiesOwned, PskBinderEntriesOwned)> for OfferedPsksOwned {
    fn from(src: (PskIdentitiesOwned, PskBinderEntriesOwned)) -> Self {
        Self {
            identities: src.0,
            binders: src.1,
        }
    }
}

impl From<OfferedPsksOwned> for (PskIdentitiesOwned, PskBinderEntriesOwned) {
    fn from(v: OfferedPsksOwned) -> Self {
        (v.identities, v.binders)
    }
}

pub struct OfferedPsksMapper;
impl Mapper for OfferedPsksMapper {
    type Src<'p> = (PskIdentities<'p>, PskBinderEntries<'p>);
    type Dst<'p> = OfferedPsks<'p>;
    type SrcBorrow<'s> = (&'s PskIdentities<'s>, &'s PskBinderEntries<'s>);
    type DstBorrow<'s> = &'s OfferedPsks<'s>;
    type SrcOwned = (PskIdentitiesOwned, PskBinderEntriesOwned);
    type DstOwned = OfferedPsksOwned;
}

impl<'a> From<OfferedPsks<'a>> for PreSharedKeyClientExtension<'a> {
    fn from(src: OfferedPsks<'a>) -> Self {
        Self { offers: src }
    }
}

impl<'s, 'a: 's> From<&'s PreSharedKeyClientExtension<'a>> for &'s OfferedPsks<'s> {
    fn from(v: &'s PreSharedKeyClientExtension<'a>) -> Self {
        &v.offers
    }
}

impl From<OfferedPsksOwned> for PreSharedKeyClientExtensionOwned {
    fn from(src: OfferedPsksOwned) -> Self {
        Self { offers: src }
    }
}

impl From<PreSharedKeyClientExtensionOwned> for OfferedPsksOwned {
    fn from(v: PreSharedKeyClientExtensionOwned) -> Self {
        v.offers
    }
}

pub struct PreSharedKeyClientExtensionMapper;
impl Mapper for PreSharedKeyClientExtensionMapper {
    type Src<'p> = OfferedPsks<'p>;
    type Dst<'p> = PreSharedKeyClientExtension<'p>;
    type SrcBorrow<'s> = &'s OfferedPsks<'s>;
    type DstBorrow<'s> = &'s PreSharedKeyClientExtension<'s>;
    type SrcOwned = OfferedPsksOwned;
    type DstOwned = PreSharedKeyClientExtensionOwned;
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

impl<'a> From<((ExtensionType, u16), ClientHelloExtensionExtensionData<'a>)>
for ClientHelloExtension<'a> {
    fn from(src: ((ExtensionType, u16), ClientHelloExtensionExtensionData<'a>)) -> Self {
        Self {
            extension_type: src.0.0,
            ext_len: src.0.1,
            extension_data: src.1,
        }
    }
}

impl<'s, 'a: 's> From<&'s ClientHelloExtension<'a>>
for ((ExtensionType, u16), &'s ClientHelloExtensionExtensionData<'s>) {
    fn from(v: &'s ClientHelloExtension<'a>) -> Self {
        ((v.extension_type, v.ext_len), &v.extension_data)
    }
}

impl From<((ExtensionType, u16), ClientHelloExtensionExtensionDataOwned)>
for ClientHelloExtensionOwned {
    fn from(
        src: ((ExtensionType, u16), ClientHelloExtensionExtensionDataOwned),
    ) -> Self {
        Self {
            extension_type: src.0.0,
            ext_len: src.0.1,
            extension_data: src.1,
        }
    }
}

impl From<ClientHelloExtensionOwned>
for ((ExtensionType, u16), ClientHelloExtensionExtensionDataOwned) {
    fn from(v: ClientHelloExtensionOwned) -> Self {
        ((v.extension_type, v.ext_len), v.extension_data)
    }
}

pub struct ClientHelloExtensionMapper;
impl Mapper for ClientHelloExtensionMapper {
    type Src<'p> = ((ExtensionType, u16), ClientHelloExtensionExtensionData<'p>);
    type Dst<'p> = ClientHelloExtension<'p>;
    type SrcBorrow<'s> = (
        (ExtensionType, u16),
        &'s ClientHelloExtensionExtensionData<'s>,
    );
    type DstBorrow<'s> = &'s ClientHelloExtension<'s>;
    type SrcOwned = ((ExtensionType, u16), ClientHelloExtensionExtensionDataOwned);
    type DstOwned = ClientHelloExtensionOwned;
}

impl<'a> From<(u16, Vec<ClientHelloExtension<'a>>)> for ClientExtensions<'a> {
    fn from(src: (u16, Vec<ClientHelloExtension<'a>>)) -> Self {
        Self { l: src.0, list: src.1 }
    }
}

impl<'s, 'a: 's> From<&'s ClientExtensions<'a>>
for (u16, &'s [&'s ClientHelloExtension<'s>]) {
    fn from(v: &'s ClientExtensions<'a>) -> Self {
        (v.l, v.list.as_slice())
    }
}

impl From<(u16, Vec<ClientHelloExtensionOwned>)> for ClientExtensionsOwned {
    fn from(src: (u16, Vec<ClientHelloExtensionOwned>)) -> Self {
        Self { l: src.0, list: src.1 }
    }
}

impl From<ClientExtensionsOwned> for (u16, Vec<ClientHelloExtensionOwned>) {
    fn from(v: ClientExtensionsOwned) -> Self {
        (v.l, v.list)
    }
}

pub struct ClientExtensionsMapper;
impl Mapper for ClientExtensionsMapper {
    type Src<'p> = (u16, Vec<ClientHelloExtension<'p>>);
    type Dst<'p> = ClientExtensions<'p>;
    type SrcBorrow<'s> = (u16, &'s [&'s ClientHelloExtension<'s>]);
    type DstBorrow<'s> = &'s ClientExtensions<'s>;
    type SrcOwned = (u16, Vec<ClientHelloExtensionOwned>);
    type DstOwned = ClientExtensionsOwned;
}

impl<
    'a,
> From<
    (
        (),
        (
            &'a [u8],
            (SessionId<'a>, (CipherSuiteList, (Opaque1Ff<'a>, ClientExtensions<'a>))),
        ),
    ),
> for ClientHello<'a> {
    fn from(
        src: (
            (),
            (
                &'a [u8],
                (SessionId<'a>, (CipherSuiteList, (Opaque1Ff<'a>, ClientExtensions<'a>))),
            ),
        ),
    ) -> Self {
        Self {
            legacy_version: CLIENT_HELLOLEGACY_VERSION_CONST,
            random: src.1.0,
            legacy_session_id: src.1.1.0,
            cipher_suites: src.1.1.1.0,
            legacy_compression_methods: src.1.1.1.1.0,
            extensions: src.1.1.1.1.1,
        }
    }
}

impl<'s, 'a: 's> From<&'s ClientHello<'a>>
for (
    (),
    (
        &'s [u8],
        (
            &'s SessionId<'s>,
            (&'s CipherSuiteList, (&'s Opaque1Ff<'s>, &'s ClientExtensions<'s>)),
        ),
    ),
) {
    fn from(v: &'s ClientHello<'a>) -> Self {
        (
            (),
            (
                v.random,
                (
                    &v.legacy_session_id,
                    (&v.cipher_suites, (&v.legacy_compression_methods, &v.extensions)),
                ),
            ),
        )
    }
}

impl From<
    (
        (),
        (
            Vec<u8>,
            (SessionIdOwned, (CipherSuiteList, (Opaque1FfOwned, ClientExtensionsOwned))),
        ),
    ),
> for ClientHelloOwned {
    fn from(
        src: (
            (),
            (
                Vec<u8>,
                (
                    SessionIdOwned,
                    (CipherSuiteList, (Opaque1FfOwned, ClientExtensionsOwned)),
                ),
            ),
        ),
    ) -> Self {
        Self {
            legacy_version: CLIENT_HELLOLEGACY_VERSION_CONST,
            random: src.1.0,
            legacy_session_id: src.1.1.0,
            cipher_suites: src.1.1.1.0,
            legacy_compression_methods: src.1.1.1.1.0,
            extensions: src.1.1.1.1.1,
        }
    }
}

impl From<ClientHelloOwned>
for (
    (),
    (
        Vec<u8>,
        (SessionIdOwned, (CipherSuiteList, (Opaque1FfOwned, ClientExtensionsOwned))),
    ),
) {
    fn from(v: ClientHelloOwned) -> Self {
        (
            CLIENT_HELLOLEGACY_VERSION_CONST,
            (
                v.random,
                (
                    v.legacy_session_id,
                    (v.cipher_suites, (v.legacy_compression_methods, v.extensions)),
                ),
            ),
        )
    }
}

pub struct ClientHelloMapper;
impl Mapper for ClientHelloMapper {
    type Src<'p> = (
        (),
        (
            &'p [u8],
            (SessionId<'p>, (CipherSuiteList, (Opaque1Ff<'p>, ClientExtensions<'p>))),
        ),
    );
    type Dst<'p> = ClientHello<'p>;
    type SrcBorrow<'s> = (
        (),
        (
            &'s [u8],
            (
                &'s SessionId<'s>,
                (&'s CipherSuiteList, (&'s Opaque1Ff<'s>, &'s ClientExtensions<'s>)),
            ),
        ),
    );
    type DstBorrow<'s> = &'s ClientHello<'s>;
    type SrcOwned = (
        (),
        (
            Vec<u8>,
            (SessionIdOwned, (CipherSuiteList, (Opaque1FfOwned, ClientExtensionsOwned))),
        ),
    );
    type DstOwned = ClientHelloOwned;
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

impl From<u32> for EarlyDataIndicationNewSessionTicket {
    fn from(src: u32) -> Self {
        Self { max_early_data_size: src }
    }
}

impl<'s> From<EarlyDataIndicationNewSessionTicket> for u32 {
    fn from(v: EarlyDataIndicationNewSessionTicket) -> Self {
        v.max_early_data_size
    }
}

pub struct EarlyDataIndicationNewSessionTicketMapper;
impl Mapper for EarlyDataIndicationNewSessionTicketMapper {
    type Src<'p> = u32;
    type Dst<'p> = EarlyDataIndicationNewSessionTicket;
    type SrcBorrow<'s> = u32;
    type DstBorrow<'s> = EarlyDataIndicationNewSessionTicket;
    type SrcOwned = u32;
    type DstOwned = EarlyDataIndicationNewSessionTicket;
}

impl From<((ExtensionType, u16), NewSessionTicketExtensionExtensionData)>
for NewSessionTicketExtension {
    fn from(
        src: ((ExtensionType, u16), NewSessionTicketExtensionExtensionData),
    ) -> Self {
        Self {
            extension_type: src.0.0,
            ext_len: src.0.1,
            extension_data: src.1,
        }
    }
}

impl<'s> From<&'s NewSessionTicketExtension>
for ((ExtensionType, u16), &'s NewSessionTicketExtensionExtensionData) {
    fn from(v: &'s NewSessionTicketExtension) -> Self {
        ((v.extension_type, v.ext_len), &v.extension_data)
    }
}

impl<'a> From<NewSessionTicketExtension>
for ((ExtensionType, u16), NewSessionTicketExtensionExtensionData) {
    fn from(v: NewSessionTicketExtension) -> Self {
        ((v.extension_type, v.ext_len), v.extension_data)
    }
}

pub struct NewSessionTicketExtensionMapper;
impl Mapper for NewSessionTicketExtensionMapper {
    type Src<'p> = ((ExtensionType, u16), NewSessionTicketExtensionExtensionData);
    type Dst<'p> = NewSessionTicketExtension;
    type SrcBorrow<'s> = (
        (ExtensionType, u16),
        &'s NewSessionTicketExtensionExtensionData,
    );
    type DstBorrow<'s> = &'s NewSessionTicketExtension;
    type SrcOwned = ((ExtensionType, u16), NewSessionTicketExtensionExtensionData);
    type DstOwned = NewSessionTicketExtension;
}

impl From<(u16, Vec<NewSessionTicketExtension>)> for NewSessionTicketExtensions {
    fn from(src: (u16, Vec<NewSessionTicketExtension>)) -> Self {
        Self { l: src.0, list: src.1 }
    }
}

impl<'s> From<&'s NewSessionTicketExtensions>
for (u16, &'s [&'s NewSessionTicketExtension]) {
    fn from(v: &'s NewSessionTicketExtensions) -> Self {
        (v.l, v.list.as_slice())
    }
}

impl<'a> From<NewSessionTicketExtensions> for (u16, Vec<NewSessionTicketExtension>) {
    fn from(v: NewSessionTicketExtensions) -> Self {
        (v.l, v.list)
    }
}

pub struct NewSessionTicketExtensionsMapper;
impl Mapper for NewSessionTicketExtensionsMapper {
    type Src<'p> = (u16, Vec<NewSessionTicketExtension>);
    type Dst<'p> = NewSessionTicketExtensions;
    type SrcBorrow<'s> = (u16, &'s [&'s NewSessionTicketExtension]);
    type DstBorrow<'s> = &'s NewSessionTicketExtensions;
    type SrcOwned = (u16, Vec<NewSessionTicketExtension>);
    type DstOwned = NewSessionTicketExtensions;
}

impl<
    'a,
> From<(u32, (u32, (Opaque0Ff<'a>, (Opaque1Ffff<'a>, NewSessionTicketExtensions))))>
for NewSessionTicket<'a> {
    fn from(
        src: (u32, (u32, (Opaque0Ff<'a>, (Opaque1Ffff<'a>, NewSessionTicketExtensions)))),
    ) -> Self {
        Self {
            ticket_lifetime: src.0,
            ticket_age_add: src.1.0,
            ticket_nonce: src.1.1.0,
            ticket: src.1.1.1.0,
            extensions: src.1.1.1.1,
        }
    }
}

impl<'s, 'a: 's> From<&'s NewSessionTicket<'a>>
for (
    u32,
    (u32, (&'s Opaque0Ff<'s>, (&'s Opaque1Ffff<'s>, &'s NewSessionTicketExtensions))),
) {
    fn from(v: &'s NewSessionTicket<'a>) -> Self {
        (
            v.ticket_lifetime,
            (v.ticket_age_add, (&v.ticket_nonce, (&v.ticket, &v.extensions))),
        )
    }
}

impl From<(u32, (u32, (Opaque0FfOwned, (Opaque1FfffOwned, NewSessionTicketExtensions))))>
for NewSessionTicketOwned {
    fn from(
        src: (
            u32,
            (u32, (Opaque0FfOwned, (Opaque1FfffOwned, NewSessionTicketExtensions))),
        ),
    ) -> Self {
        Self {
            ticket_lifetime: src.0,
            ticket_age_add: src.1.0,
            ticket_nonce: src.1.1.0,
            ticket: src.1.1.1.0,
            extensions: src.1.1.1.1,
        }
    }
}

impl From<NewSessionTicketOwned>
for (u32, (u32, (Opaque0FfOwned, (Opaque1FfffOwned, NewSessionTicketExtensions)))) {
    fn from(v: NewSessionTicketOwned) -> Self {
        (
            v.ticket_lifetime,
            (v.ticket_age_add, (v.ticket_nonce, (v.ticket, v.extensions))),
        )
    }
}

pub struct NewSessionTicketMapper;
impl Mapper for NewSessionTicketMapper {
    type Src<'p> = (
        u32,
        (u32, (Opaque0Ff<'p>, (Opaque1Ffff<'p>, NewSessionTicketExtensions))),
    );
    type Dst<'p> = NewSessionTicket<'p>;
    type SrcBorrow<'s> = (
        u32,
        (u32, (&'s Opaque0Ff<'s>, (&'s Opaque1Ffff<'s>, &'s NewSessionTicketExtensions))),
    );
    type DstBorrow<'s> = &'s NewSessionTicket<'s>;
    type SrcOwned = (
        u32,
        (u32, (Opaque0FfOwned, (Opaque1FfffOwned, NewSessionTicketExtensions))),
    );
    type DstOwned = NewSessionTicketOwned;
}

impl<'a> From<((ExtensionType, u16), EncryptedExtensionExtensionData<'a>)>
for EncryptedExtension<'a> {
    fn from(src: ((ExtensionType, u16), EncryptedExtensionExtensionData<'a>)) -> Self {
        Self {
            extension_type: src.0.0,
            ext_len: src.0.1,
            extension_data: src.1,
        }
    }
}

impl<'s, 'a: 's> From<&'s EncryptedExtension<'a>>
for ((ExtensionType, u16), &'s EncryptedExtensionExtensionData<'s>) {
    fn from(v: &'s EncryptedExtension<'a>) -> Self {
        ((v.extension_type, v.ext_len), &v.extension_data)
    }
}

impl From<((ExtensionType, u16), EncryptedExtensionExtensionDataOwned)>
for EncryptedExtensionOwned {
    fn from(src: ((ExtensionType, u16), EncryptedExtensionExtensionDataOwned)) -> Self {
        Self {
            extension_type: src.0.0,
            ext_len: src.0.1,
            extension_data: src.1,
        }
    }
}

impl From<EncryptedExtensionOwned>
for ((ExtensionType, u16), EncryptedExtensionExtensionDataOwned) {
    fn from(v: EncryptedExtensionOwned) -> Self {
        ((v.extension_type, v.ext_len), v.extension_data)
    }
}

pub struct EncryptedExtensionMapper;
impl Mapper for EncryptedExtensionMapper {
    type Src<'p> = ((ExtensionType, u16), EncryptedExtensionExtensionData<'p>);
    type Dst<'p> = EncryptedExtension<'p>;
    type SrcBorrow<'s> = ((ExtensionType, u16), &'s EncryptedExtensionExtensionData<'s>);
    type DstBorrow<'s> = &'s EncryptedExtension<'s>;
    type SrcOwned = ((ExtensionType, u16), EncryptedExtensionExtensionDataOwned);
    type DstOwned = EncryptedExtensionOwned;
}

impl<'a> From<(u16, Vec<EncryptedExtension<'a>>)> for EncryptedExtensions<'a> {
    fn from(src: (u16, Vec<EncryptedExtension<'a>>)) -> Self {
        Self { l: src.0, list: src.1 }
    }
}

impl<'s, 'a: 's> From<&'s EncryptedExtensions<'a>>
for (u16, &'s [&'s EncryptedExtension<'s>]) {
    fn from(v: &'s EncryptedExtensions<'a>) -> Self {
        (v.l, v.list.as_slice())
    }
}

impl From<(u16, Vec<EncryptedExtensionOwned>)> for EncryptedExtensionsOwned {
    fn from(src: (u16, Vec<EncryptedExtensionOwned>)) -> Self {
        Self { l: src.0, list: src.1 }
    }
}

impl From<EncryptedExtensionsOwned> for (u16, Vec<EncryptedExtensionOwned>) {
    fn from(v: EncryptedExtensionsOwned) -> Self {
        (v.l, v.list)
    }
}

pub struct EncryptedExtensionsMapper;
impl Mapper for EncryptedExtensionsMapper {
    type Src<'p> = (u16, Vec<EncryptedExtension<'p>>);
    type Dst<'p> = EncryptedExtensions<'p>;
    type SrcBorrow<'s> = (u16, &'s [&'s EncryptedExtension<'s>]);
    type DstBorrow<'s> = &'s EncryptedExtensions<'s>;
    type SrcOwned = (u16, Vec<EncryptedExtensionOwned>);
    type DstOwned = EncryptedExtensionsOwned;
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
        (CERTIFICATE_STATUSSTATUS_TYPE_CONST, v.response)
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

impl<'a> From<((ExtensionType, u16), CertificateExtensionExtensionData<'a>)>
for CertificateExtension<'a> {
    fn from(src: ((ExtensionType, u16), CertificateExtensionExtensionData<'a>)) -> Self {
        Self {
            extension_type: src.0.0,
            ext_len: src.0.1,
            extension_data: src.1,
        }
    }
}

impl<'s, 'a: 's> From<&'s CertificateExtension<'a>>
for ((ExtensionType, u16), &'s CertificateExtensionExtensionData<'s>) {
    fn from(v: &'s CertificateExtension<'a>) -> Self {
        ((v.extension_type, v.ext_len), &v.extension_data)
    }
}

impl From<((ExtensionType, u16), CertificateExtensionExtensionDataOwned)>
for CertificateExtensionOwned {
    fn from(
        src: ((ExtensionType, u16), CertificateExtensionExtensionDataOwned),
    ) -> Self {
        Self {
            extension_type: src.0.0,
            ext_len: src.0.1,
            extension_data: src.1,
        }
    }
}

impl From<CertificateExtensionOwned>
for ((ExtensionType, u16), CertificateExtensionExtensionDataOwned) {
    fn from(v: CertificateExtensionOwned) -> Self {
        ((v.extension_type, v.ext_len), v.extension_data)
    }
}

pub struct CertificateExtensionMapper;
impl Mapper for CertificateExtensionMapper {
    type Src<'p> = ((ExtensionType, u16), CertificateExtensionExtensionData<'p>);
    type Dst<'p> = CertificateExtension<'p>;
    type SrcBorrow<'s> = (
        (ExtensionType, u16),
        &'s CertificateExtensionExtensionData<'s>,
    );
    type DstBorrow<'s> = &'s CertificateExtension<'s>;
    type SrcOwned = ((ExtensionType, u16), CertificateExtensionExtensionDataOwned);
    type DstOwned = CertificateExtensionOwned;
}

impl<'a> From<(u16, Vec<CertificateExtension<'a>>)> for CertificateExtensions<'a> {
    fn from(src: (u16, Vec<CertificateExtension<'a>>)) -> Self {
        Self { l: src.0, list: src.1 }
    }
}

impl<'s, 'a: 's> From<&'s CertificateExtensions<'a>>
for (u16, &'s [&'s CertificateExtension<'s>]) {
    fn from(v: &'s CertificateExtensions<'a>) -> Self {
        (v.l, v.list.as_slice())
    }
}

impl From<(u16, Vec<CertificateExtensionOwned>)> for CertificateExtensionsOwned {
    fn from(src: (u16, Vec<CertificateExtensionOwned>)) -> Self {
        Self { l: src.0, list: src.1 }
    }
}

impl From<CertificateExtensionsOwned> for (u16, Vec<CertificateExtensionOwned>) {
    fn from(v: CertificateExtensionsOwned) -> Self {
        (v.l, v.list)
    }
}

pub struct CertificateExtensionsMapper;
impl Mapper for CertificateExtensionsMapper {
    type Src<'p> = (u16, Vec<CertificateExtension<'p>>);
    type Dst<'p> = CertificateExtensions<'p>;
    type SrcBorrow<'s> = (u16, &'s [&'s CertificateExtension<'s>]);
    type DstBorrow<'s> = &'s CertificateExtensions<'s>;
    type SrcOwned = (u16, Vec<CertificateExtensionOwned>);
    type DstOwned = CertificateExtensionsOwned;
}

impl<'a> From<(Opaque1Ffffff<'a>, CertificateExtensions<'a>)>
for CertificateEntryOpaque<'a> {
    fn from(src: (Opaque1Ffffff<'a>, CertificateExtensions<'a>)) -> Self {
        Self {
            cert_data: src.0,
            extensions: src.1,
        }
    }
}

impl<'s, 'a: 's> From<&'s CertificateEntryOpaque<'a>>
for (&'s Opaque1Ffffff<'s>, &'s CertificateExtensions<'s>) {
    fn from(v: &'s CertificateEntryOpaque<'a>) -> Self {
        (&v.cert_data, &v.extensions)
    }
}

impl From<(Opaque1FfffffOwned, CertificateExtensionsOwned)>
for CertificateEntryOpaqueOwned {
    fn from(src: (Opaque1FfffffOwned, CertificateExtensionsOwned)) -> Self {
        Self {
            cert_data: src.0,
            extensions: src.1,
        }
    }
}

impl From<CertificateEntryOpaqueOwned>
for (Opaque1FfffffOwned, CertificateExtensionsOwned) {
    fn from(v: CertificateEntryOpaqueOwned) -> Self {
        (v.cert_data, v.extensions)
    }
}

pub struct CertificateEntryOpaqueMapper;
impl Mapper for CertificateEntryOpaqueMapper {
    type Src<'p> = (Opaque1Ffffff<'p>, CertificateExtensions<'p>);
    type Dst<'p> = CertificateEntryOpaque<'p>;
    type SrcBorrow<'s> = (&'s Opaque1Ffffff<'s>, &'s CertificateExtensions<'s>);
    type DstBorrow<'s> = &'s CertificateEntryOpaque<'s>;
    type SrcOwned = (Opaque1FfffffOwned, CertificateExtensionsOwned);
    type DstOwned = CertificateEntryOpaqueOwned;
}

impl<'a> From<(u24, Vec<CertificateEntryOpaque<'a>>)> for CertificateList<'a> {
    fn from(src: (u24, Vec<CertificateEntryOpaque<'a>>)) -> Self {
        Self { l: src.0, list: src.1 }
    }
}

impl<'s, 'a: 's> From<&'s CertificateList<'a>>
for (u24, &'s [&'s CertificateEntryOpaque<'s>]) {
    fn from(v: &'s CertificateList<'a>) -> Self {
        (v.l, v.list.as_slice())
    }
}

impl From<(u24, Vec<CertificateEntryOpaqueOwned>)> for CertificateListOwned {
    fn from(src: (u24, Vec<CertificateEntryOpaqueOwned>)) -> Self {
        Self { l: src.0, list: src.1 }
    }
}

impl From<CertificateListOwned> for (u24, Vec<CertificateEntryOpaqueOwned>) {
    fn from(v: CertificateListOwned) -> Self {
        (v.l, v.list)
    }
}

pub struct CertificateListMapper;
impl Mapper for CertificateListMapper {
    type Src<'p> = (u24, Vec<CertificateEntryOpaque<'p>>);
    type Dst<'p> = CertificateList<'p>;
    type SrcBorrow<'s> = (u24, &'s [&'s CertificateEntryOpaque<'s>]);
    type DstBorrow<'s> = &'s CertificateList<'s>;
    type SrcOwned = (u24, Vec<CertificateEntryOpaqueOwned>);
    type DstOwned = CertificateListOwned;
}

impl<'a> From<(Opaque0Ff<'a>, CertificateList<'a>)> for Certificate<'a> {
    fn from(src: (Opaque0Ff<'a>, CertificateList<'a>)) -> Self {
        Self {
            certificate_request_context: src.0,
            certificate_list: src.1,
        }
    }
}

impl<'s, 'a: 's> From<&'s Certificate<'a>>
for (&'s Opaque0Ff<'s>, &'s CertificateList<'s>) {
    fn from(v: &'s Certificate<'a>) -> Self {
        (&v.certificate_request_context, &v.certificate_list)
    }
}

impl From<(Opaque0FfOwned, CertificateListOwned)> for CertificateOwned {
    fn from(src: (Opaque0FfOwned, CertificateListOwned)) -> Self {
        Self {
            certificate_request_context: src.0,
            certificate_list: src.1,
        }
    }
}

impl From<CertificateOwned> for (Opaque0FfOwned, CertificateListOwned) {
    fn from(v: CertificateOwned) -> Self {
        (v.certificate_request_context, v.certificate_list)
    }
}

pub struct CertificateMapper;
impl Mapper for CertificateMapper {
    type Src<'p> = (Opaque0Ff<'p>, CertificateList<'p>);
    type Dst<'p> = Certificate<'p>;
    type SrcBorrow<'s> = (&'s Opaque0Ff<'s>, &'s CertificateList<'s>);
    type DstBorrow<'s> = &'s Certificate<'s>;
    type SrcOwned = (Opaque0FfOwned, CertificateListOwned);
    type DstOwned = CertificateOwned;
}

impl<'a> From<(u16, Vec<CertificateRequestExtension<'a>>)>
for CertificateRequestExtensions<'a> {
    fn from(src: (u16, Vec<CertificateRequestExtension<'a>>)) -> Self {
        Self { l: src.0, list: src.1 }
    }
}

impl<'s, 'a: 's> From<&'s CertificateRequestExtensions<'a>>
for (u16, &'s [&'s CertificateRequestExtension<'s>]) {
    fn from(v: &'s CertificateRequestExtensions<'a>) -> Self {
        (v.l, v.list.as_slice())
    }
}

impl From<(u16, Vec<CertificateRequestExtensionOwned>)>
for CertificateRequestExtensionsOwned {
    fn from(src: (u16, Vec<CertificateRequestExtensionOwned>)) -> Self {
        Self { l: src.0, list: src.1 }
    }
}

impl From<CertificateRequestExtensionsOwned>
for (u16, Vec<CertificateRequestExtensionOwned>) {
    fn from(v: CertificateRequestExtensionsOwned) -> Self {
        (v.l, v.list)
    }
}

pub struct CertificateRequestExtensionsMapper;
impl Mapper for CertificateRequestExtensionsMapper {
    type Src<'p> = (u16, Vec<CertificateRequestExtension<'p>>);
    type Dst<'p> = CertificateRequestExtensions<'p>;
    type SrcBorrow<'s> = (u16, &'s [&'s CertificateRequestExtension<'s>]);
    type DstBorrow<'s> = &'s CertificateRequestExtensions<'s>;
    type SrcOwned = (u16, Vec<CertificateRequestExtensionOwned>);
    type DstOwned = CertificateRequestExtensionsOwned;
}

impl<'a> From<(Opaque0Ff<'a>, CertificateRequestExtensions<'a>)>
for CertificateRequest<'a> {
    fn from(src: (Opaque0Ff<'a>, CertificateRequestExtensions<'a>)) -> Self {
        Self {
            certificate_request_context: src.0,
            extensions: src.1,
        }
    }
}

impl<'s, 'a: 's> From<&'s CertificateRequest<'a>>
for (&'s Opaque0Ff<'s>, &'s CertificateRequestExtensions<'s>) {
    fn from(v: &'s CertificateRequest<'a>) -> Self {
        (&v.certificate_request_context, &v.extensions)
    }
}

impl From<(Opaque0FfOwned, CertificateRequestExtensionsOwned)>
for CertificateRequestOwned {
    fn from(src: (Opaque0FfOwned, CertificateRequestExtensionsOwned)) -> Self {
        Self {
            certificate_request_context: src.0,
            extensions: src.1,
        }
    }
}

impl From<CertificateRequestOwned>
for (Opaque0FfOwned, CertificateRequestExtensionsOwned) {
    fn from(v: CertificateRequestOwned) -> Self {
        (v.certificate_request_context, v.extensions)
    }
}

pub struct CertificateRequestMapper;
impl Mapper for CertificateRequestMapper {
    type Src<'p> = (Opaque0Ff<'p>, CertificateRequestExtensions<'p>);
    type Dst<'p> = CertificateRequest<'p>;
    type SrcBorrow<'s> = (&'s Opaque0Ff<'s>, &'s CertificateRequestExtensions<'s>);
    type DstBorrow<'s> = &'s CertificateRequest<'s>;
    type SrcOwned = (Opaque0FfOwned, CertificateRequestExtensionsOwned);
    type DstOwned = CertificateRequestOwned;
}

impl<'a> From<(SignatureScheme, Opaque0Ffff<'a>)> for CertificateVerify<'a> {
    fn from(src: (SignatureScheme, Opaque0Ffff<'a>)) -> Self {
        Self {
            algorithm: src.0,
            signature: src.1,
        }
    }
}

impl<'s, 'a: 's> From<&'s CertificateVerify<'a>>
for (SignatureScheme, &'s Opaque0Ffff<'s>) {
    fn from(v: &'s CertificateVerify<'a>) -> Self {
        (v.algorithm, &v.signature)
    }
}

impl From<(SignatureScheme, Opaque0FfffOwned)> for CertificateVerifyOwned {
    fn from(src: (SignatureScheme, Opaque0FfffOwned)) -> Self {
        Self {
            algorithm: src.0,
            signature: src.1,
        }
    }
}

impl From<CertificateVerifyOwned> for (SignatureScheme, Opaque0FfffOwned) {
    fn from(v: CertificateVerifyOwned) -> Self {
        (v.algorithm, v.signature)
    }
}

pub struct CertificateVerifyMapper;
impl Mapper for CertificateVerifyMapper {
    type Src<'p> = (SignatureScheme, Opaque0Ffff<'p>);
    type Dst<'p> = CertificateVerify<'p>;
    type SrcBorrow<'s> = (SignatureScheme, &'s Opaque0Ffff<'s>);
    type DstBorrow<'s> = &'s CertificateVerify<'s>;
    type SrcOwned = (SignatureScheme, Opaque0FfffOwned);
    type DstOwned = CertificateVerifyOwned;
}

impl From<u8> for KeyUpdateRequest {
    fn from(src: u8) -> Self {
        match src as i128 {
            0 => Self::UpdateNotRequested,
            1 => Self::UpdateRequested,
            _ => unreachable!("validated by combinator"),
        }
    }
}

impl From<KeyUpdateRequest> for u8 {
    fn from(v: KeyUpdateRequest) -> Self {
        v as u8
    }
}

pub struct KeyUpdateRequestMapper;
impl Mapper for KeyUpdateRequestMapper {
    type Src<'p> = u8;
    type Dst<'p> = KeyUpdateRequest;
    type SrcBorrow<'s> = u8;
    type DstBorrow<'s> = KeyUpdateRequest;
    type SrcOwned = u8;
    type DstOwned = KeyUpdateRequest;
}

impl From<KeyUpdateRequest> for KeyUpdate {
    fn from(src: KeyUpdateRequest) -> Self {
        Self { request_update: src }
    }
}

impl<'s> From<KeyUpdate> for KeyUpdateRequest {
    fn from(v: KeyUpdate) -> Self {
        v.request_update
    }
}

pub struct KeyUpdateMapper;
impl Mapper for KeyUpdateMapper {
    type Src<'p> = KeyUpdateRequest;
    type Dst<'p> = KeyUpdate;
    type SrcBorrow<'s> = KeyUpdateRequest;
    type DstBorrow<'s> = KeyUpdate;
    type SrcOwned = KeyUpdateRequest;
    type DstOwned = KeyUpdate;
}

impl<'a> From<((HandshakeType, u24), HandshakeMsg<'a>)> for Handshake<'a> {
    fn from(src: ((HandshakeType, u24), HandshakeMsg<'a>)) -> Self {
        Self {
            msg_type: src.0.0,
            length: src.0.1,
            msg: src.1,
        }
    }
}

impl<'s, 'a: 's> From<&'s Handshake<'a>>
for ((HandshakeType, u24), &'s HandshakeMsg<'s>) {
    fn from(v: &'s Handshake<'a>) -> Self {
        ((v.msg_type, v.length), &v.msg)
    }
}

impl From<((HandshakeType, u24), HandshakeMsgOwned)> for HandshakeOwned {
    fn from(src: ((HandshakeType, u24), HandshakeMsgOwned)) -> Self {
        Self {
            msg_type: src.0.0,
            length: src.0.1,
            msg: src.1,
        }
    }
}

impl From<HandshakeOwned> for ((HandshakeType, u24), HandshakeMsgOwned) {
    fn from(v: HandshakeOwned) -> Self {
        ((v.msg_type, v.length), v.msg)
    }
}

pub struct HandshakeMapper;
impl Mapper for HandshakeMapper {
    type Src<'p> = ((HandshakeType, u24), HandshakeMsg<'p>);
    type Dst<'p> = Handshake<'p>;
    type SrcBorrow<'s> = ((HandshakeType, u24), &'s HandshakeMsg<'s>);
    type DstBorrow<'s> = &'s Handshake<'s>;
    type SrcOwned = ((HandshakeType, u24), HandshakeMsgOwned);
    type DstOwned = HandshakeOwned;
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

impl<'a> From<(ExtensionType, Opaque0Ffff<'a>)> for Extension<'a> {
    fn from(src: (ExtensionType, Opaque0Ffff<'a>)) -> Self {
        Self {
            extension_type: src.0,
            extension_data: src.1,
        }
    }
}

impl<'s, 'a: 's> From<&'s Extension<'a>> for (ExtensionType, &'s Opaque0Ffff<'s>) {
    fn from(v: &'s Extension<'a>) -> Self {
        (v.extension_type, &v.extension_data)
    }
}

impl From<(ExtensionType, Opaque0FfffOwned)> for ExtensionOwned {
    fn from(src: (ExtensionType, Opaque0FfffOwned)) -> Self {
        Self {
            extension_type: src.0,
            extension_data: src.1,
        }
    }
}

impl From<ExtensionOwned> for (ExtensionType, Opaque0FfffOwned) {
    fn from(v: ExtensionOwned) -> Self {
        (v.extension_type, v.extension_data)
    }
}

pub struct ExtensionMapper;
impl Mapper for ExtensionMapper {
    type Src<'p> = (ExtensionType, Opaque0Ffff<'p>);
    type Dst<'p> = Extension<'p>;
    type SrcBorrow<'s> = (ExtensionType, &'s Opaque0Ffff<'s>);
    type DstBorrow<'s> = &'s Extension<'s>;
    type SrcOwned = (ExtensionType, Opaque0FfffOwned);
    type DstOwned = ExtensionOwned;
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

impl From<u8> for ContentType {
    fn from(src: u8) -> Self {
        match src as i128 {
            0 => Self::Invalid,
            20 => Self::ChangeCipherSpec,
            21 => Self::Alert,
            22 => Self::Handshake,
            23 => Self::ApplicationData,
            _ => unreachable!("validated by combinator"),
        }
    }
}

impl From<ContentType> for u8 {
    fn from(v: ContentType) -> Self {
        v as u8
    }
}

pub struct ContentTypeMapper;
impl Mapper for ContentTypeMapper {
    type Src<'p> = u8;
    type Dst<'p> = ContentType;
    type SrcBorrow<'s> = u8;
    type DstBorrow<'s> = ContentType;
    type SrcOwned = u8;
    type DstOwned = ContentType;
}

impl<'a> From<(ContentType, (ProtocolVersion, Opaque0Ffff<'a>))> for TlsPlaintext<'a> {
    fn from(src: (ContentType, (ProtocolVersion, Opaque0Ffff<'a>))) -> Self {
        Self {
            content_type: src.0,
            legacy_record_version: src.1.0,
            fragment: src.1.1,
        }
    }
}

impl<'s, 'a: 's> From<&'s TlsPlaintext<'a>>
for (ContentType, (ProtocolVersion, &'s Opaque0Ffff<'s>)) {
    fn from(v: &'s TlsPlaintext<'a>) -> Self {
        (v.content_type, (v.legacy_record_version, &v.fragment))
    }
}

impl From<(ContentType, (ProtocolVersion, Opaque0FfffOwned))> for TlsPlaintextOwned {
    fn from(src: (ContentType, (ProtocolVersion, Opaque0FfffOwned))) -> Self {
        Self {
            content_type: src.0,
            legacy_record_version: src.1.0,
            fragment: src.1.1,
        }
    }
}

impl From<TlsPlaintextOwned> for (ContentType, (ProtocolVersion, Opaque0FfffOwned)) {
    fn from(v: TlsPlaintextOwned) -> Self {
        (v.content_type, (v.legacy_record_version, v.fragment))
    }
}

pub struct TlsPlaintextMapper;
impl Mapper for TlsPlaintextMapper {
    type Src<'p> = (ContentType, (ProtocolVersion, Opaque0Ffff<'p>));
    type Dst<'p> = TlsPlaintext<'p>;
    type SrcBorrow<'s> = (ContentType, (ProtocolVersion, &'s Opaque0Ffff<'s>));
    type DstBorrow<'s> = &'s TlsPlaintext<'s>;
    type SrcOwned = (ContentType, (ProtocolVersion, Opaque0FfffOwned));
    type DstOwned = TlsPlaintextOwned;
}

impl From<u8> for AlertDescription {
    fn from(src: u8) -> Self {
        match src as i128 {
            0 => Self::CloseNotify,
            10 => Self::UnexpectedMessage,
            20 => Self::BadRecordMac,
            22 => Self::RecordOverflow,
            40 => Self::HandshakeFailure,
            42 => Self::BadCertificate,
            43 => Self::UnsupportedCertificate,
            44 => Self::CertificateRevoked,
            45 => Self::CertificateExpired,
            46 => Self::CertificateUnknown,
            47 => Self::IllegalParameter,
            48 => Self::UnknownCA,
            49 => Self::AccessDenied,
            50 => Self::DecodeError,
            51 => Self::DecryptError,
            70 => Self::ProtocolVersion,
            71 => Self::InsufficientSecurity,
            80 => Self::InternalError,
            86 => Self::InappropriateFallback,
            90 => Self::UserCanceled,
            109 => Self::MissingExtension,
            110 => Self::UnsupportedExtension,
            112 => Self::UnrecognizedName,
            113 => Self::BadCertificateStatusResponse,
            115 => Self::UnknownPSKIdentity,
            116 => Self::CertificateRequired,
            120 => Self::NoApplicationProtocol,
            _ => unreachable!("validated by combinator"),
        }
    }
}

impl From<AlertDescription> for u8 {
    fn from(v: AlertDescription) -> Self {
        v as u8
    }
}

pub struct AlertDescriptionMapper;
impl Mapper for AlertDescriptionMapper {
    type Src<'p> = u8;
    type Dst<'p> = AlertDescription;
    type SrcBorrow<'s> = u8;
    type DstBorrow<'s> = AlertDescription;
    type SrcOwned = u8;
    type DstOwned = AlertDescription;
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

impl From<u16> for PreSharedKeyServerExtension {
    fn from(src: u16) -> Self {
        Self { selected_identity: src }
    }
}

impl<'s> From<PreSharedKeyServerExtension> for u16 {
    fn from(v: PreSharedKeyServerExtension) -> Self {
        v.selected_identity
    }
}

pub struct PreSharedKeyServerExtensionMapper;
impl Mapper for PreSharedKeyServerExtensionMapper {
    type Src<'p> = u16;
    type Dst<'p> = PreSharedKeyServerExtension;
    type SrcBorrow<'s> = u16;
    type DstBorrow<'s> = PreSharedKeyServerExtension;
    type SrcOwned = u16;
    type DstOwned = PreSharedKeyServerExtension;
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

impl<'a> From<((ExtensionType, u16), HelloRetryExtensionExtensionData<'a>)>
for HelloRetryExtension<'a> {
    fn from(src: ((ExtensionType, u16), HelloRetryExtensionExtensionData<'a>)) -> Self {
        Self {
            extension_type: src.0.0,
            ext_len: src.0.1,
            extension_data: src.1,
        }
    }
}

impl<'s, 'a: 's> From<&'s HelloRetryExtension<'a>>
for ((ExtensionType, u16), &'s HelloRetryExtensionExtensionData<'s>) {
    fn from(v: &'s HelloRetryExtension<'a>) -> Self {
        ((v.extension_type, v.ext_len), &v.extension_data)
    }
}

impl From<((ExtensionType, u16), HelloRetryExtensionExtensionDataOwned)>
for HelloRetryExtensionOwned {
    fn from(src: ((ExtensionType, u16), HelloRetryExtensionExtensionDataOwned)) -> Self {
        Self {
            extension_type: src.0.0,
            ext_len: src.0.1,
            extension_data: src.1,
        }
    }
}

impl From<HelloRetryExtensionOwned>
for ((ExtensionType, u16), HelloRetryExtensionExtensionDataOwned) {
    fn from(v: HelloRetryExtensionOwned) -> Self {
        ((v.extension_type, v.ext_len), v.extension_data)
    }
}

pub struct HelloRetryExtensionMapper;
impl Mapper for HelloRetryExtensionMapper {
    type Src<'p> = ((ExtensionType, u16), HelloRetryExtensionExtensionData<'p>);
    type Dst<'p> = HelloRetryExtension<'p>;
    type SrcBorrow<'s> = (
        (ExtensionType, u16),
        &'s HelloRetryExtensionExtensionData<'s>,
    );
    type DstBorrow<'s> = &'s HelloRetryExtension<'s>;
    type SrcOwned = ((ExtensionType, u16), HelloRetryExtensionExtensionDataOwned);
    type DstOwned = HelloRetryExtensionOwned;
}

impl From<(AlertLevel, AlertDescription)> for Alert {
    fn from(src: (AlertLevel, AlertDescription)) -> Self {
        Self {
            level: src.0,
            description: src.1,
        }
    }
}

impl<'s> From<Alert> for (AlertLevel, AlertDescription) {
    fn from(v: Alert) -> Self {
        (v.level, v.description)
    }
}

pub struct AlertMapper;
impl Mapper for AlertMapper {
    type Src<'p> = (AlertLevel, AlertDescription);
    type Dst<'p> = Alert;
    type SrcBorrow<'s> = (AlertLevel, AlertDescription);
    type DstBorrow<'s> = Alert;
    type SrcOwned = (AlertLevel, AlertDescription);
    type DstOwned = Alert;
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

impl<'a> From<(u16, Vec<HelloRetryExtension<'a>>)> for HelloRetryExtensions<'a> {
    fn from(src: (u16, Vec<HelloRetryExtension<'a>>)) -> Self {
        Self { l: src.0, list: src.1 }
    }
}

impl<'s, 'a: 's> From<&'s HelloRetryExtensions<'a>>
for (u16, &'s [&'s HelloRetryExtension<'s>]) {
    fn from(v: &'s HelloRetryExtensions<'a>) -> Self {
        (v.l, v.list.as_slice())
    }
}

impl From<(u16, Vec<HelloRetryExtensionOwned>)> for HelloRetryExtensionsOwned {
    fn from(src: (u16, Vec<HelloRetryExtensionOwned>)) -> Self {
        Self { l: src.0, list: src.1 }
    }
}

impl From<HelloRetryExtensionsOwned> for (u16, Vec<HelloRetryExtensionOwned>) {
    fn from(v: HelloRetryExtensionsOwned) -> Self {
        (v.l, v.list)
    }
}

pub struct HelloRetryExtensionsMapper;
impl Mapper for HelloRetryExtensionsMapper {
    type Src<'p> = (u16, Vec<HelloRetryExtension<'p>>);
    type Dst<'p> = HelloRetryExtensions<'p>;
    type SrcBorrow<'s> = (u16, &'s [&'s HelloRetryExtension<'s>]);
    type DstBorrow<'s> = &'s HelloRetryExtensions<'s>;
    type SrcOwned = (u16, Vec<HelloRetryExtensionOwned>);
    type DstOwned = HelloRetryExtensionsOwned;
}

impl<'a> From<((ExtensionType, u16), SeverHelloExtensionExtensionData<'a>)>
for SeverHelloExtension<'a> {
    fn from(src: ((ExtensionType, u16), SeverHelloExtensionExtensionData<'a>)) -> Self {
        Self {
            extension_type: src.0.0,
            ext_len: src.0.1,
            extension_data: src.1,
        }
    }
}

impl<'s, 'a: 's> From<&'s SeverHelloExtension<'a>>
for ((ExtensionType, u16), &'s SeverHelloExtensionExtensionData<'s>) {
    fn from(v: &'s SeverHelloExtension<'a>) -> Self {
        ((v.extension_type, v.ext_len), &v.extension_data)
    }
}

impl From<((ExtensionType, u16), SeverHelloExtensionExtensionDataOwned)>
for SeverHelloExtensionOwned {
    fn from(src: ((ExtensionType, u16), SeverHelloExtensionExtensionDataOwned)) -> Self {
        Self {
            extension_type: src.0.0,
            ext_len: src.0.1,
            extension_data: src.1,
        }
    }
}

impl From<SeverHelloExtensionOwned>
for ((ExtensionType, u16), SeverHelloExtensionExtensionDataOwned) {
    fn from(v: SeverHelloExtensionOwned) -> Self {
        ((v.extension_type, v.ext_len), v.extension_data)
    }
}

pub struct SeverHelloExtensionMapper;
impl Mapper for SeverHelloExtensionMapper {
    type Src<'p> = ((ExtensionType, u16), SeverHelloExtensionExtensionData<'p>);
    type Dst<'p> = SeverHelloExtension<'p>;
    type SrcBorrow<'s> = (
        (ExtensionType, u16),
        &'s SeverHelloExtensionExtensionData<'s>,
    );
    type DstBorrow<'s> = &'s SeverHelloExtension<'s>;
    type SrcOwned = ((ExtensionType, u16), SeverHelloExtensionExtensionDataOwned);
    type DstOwned = SeverHelloExtensionOwned;
}

impl<'a> From<(u16, Vec<SeverHelloExtension<'a>>)> for ServerExtensions<'a> {
    fn from(src: (u16, Vec<SeverHelloExtension<'a>>)) -> Self {
        Self { l: src.0, list: src.1 }
    }
}

impl<'s, 'a: 's> From<&'s ServerExtensions<'a>>
for (u16, &'s [&'s SeverHelloExtension<'s>]) {
    fn from(v: &'s ServerExtensions<'a>) -> Self {
        (v.l, v.list.as_slice())
    }
}

impl From<(u16, Vec<SeverHelloExtensionOwned>)> for ServerExtensionsOwned {
    fn from(src: (u16, Vec<SeverHelloExtensionOwned>)) -> Self {
        Self { l: src.0, list: src.1 }
    }
}

impl From<ServerExtensionsOwned> for (u16, Vec<SeverHelloExtensionOwned>) {
    fn from(v: ServerExtensionsOwned) -> Self {
        (v.l, v.list)
    }
}

pub struct ServerExtensionsMapper;
impl Mapper for ServerExtensionsMapper {
    type Src<'p> = (u16, Vec<SeverHelloExtension<'p>>);
    type Dst<'p> = ServerExtensions<'p>;
    type SrcBorrow<'s> = (u16, &'s [&'s SeverHelloExtension<'s>]);
    type DstBorrow<'s> = &'s ServerExtensions<'s>;
    type SrcOwned = (u16, Vec<SeverHelloExtensionOwned>);
    type DstOwned = ServerExtensionsOwned;
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

impl<'a> From<(SessionId<'a>, (CipherSuite, ((), ServerExtensions<'a>)))>
for ServerHello<'a> {
    fn from(src: (SessionId<'a>, (CipherSuite, ((), ServerExtensions<'a>)))) -> Self {
        Self {
            legacy_session_id_echo: src.0,
            cipher_suite: src.1.0,
            legacy_compression_method: SERVER_HELLOLEGACY_COMPRESSION_METHOD_CONST,
            extensions: src.1.1.1,
        }
    }
}

impl<'s, 'a: 's> From<&'s ServerHello<'a>>
for (&'s SessionId<'s>, (CipherSuite, ((), &'s ServerExtensions<'s>))) {
    fn from(v: &'s ServerHello<'a>) -> Self {
        (&v.legacy_session_id_echo, (v.cipher_suite, ((), &v.extensions)))
    }
}

impl From<(SessionIdOwned, (CipherSuite, ((), ServerExtensionsOwned)))>
for ServerHelloOwned {
    fn from(src: (SessionIdOwned, (CipherSuite, ((), ServerExtensionsOwned)))) -> Self {
        Self {
            legacy_session_id_echo: src.0,
            cipher_suite: src.1.0,
            legacy_compression_method: SERVER_HELLOLEGACY_COMPRESSION_METHOD_CONST,
            extensions: src.1.1.1,
        }
    }
}

impl From<ServerHelloOwned>
for (SessionIdOwned, (CipherSuite, ((), ServerExtensionsOwned))) {
    fn from(v: ServerHelloOwned) -> Self {
        (
            v.legacy_session_id_echo,
            (v.cipher_suite, (SERVER_HELLOLEGACY_COMPRESSION_METHOD_CONST, v.extensions)),
        )
    }
}

pub struct ServerHelloMapper;
impl Mapper for ServerHelloMapper {
    type Src<'p> = (SessionId<'p>, (CipherSuite, ((), ServerExtensions<'p>)));
    type Dst<'p> = ServerHello<'p>;
    type SrcBorrow<'s> = (
        &'s SessionId<'s>,
        (CipherSuite, ((), &'s ServerExtensions<'s>)),
    );
    type DstBorrow<'s> = &'s ServerHello<'s>;
    type SrcOwned = (SessionIdOwned, (CipherSuite, ((), ServerExtensionsOwned)));
    type DstOwned = ServerHelloOwned;
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

impl<'a> From<(SessionId<'a>, (CipherSuite, ((), HelloRetryExtensions<'a>)))>
for HelloRetryRequest<'a> {
    fn from(
        src: (SessionId<'a>, (CipherSuite, ((), HelloRetryExtensions<'a>))),
    ) -> Self {
        Self {
            legacy_session_id_echo: src.0,
            cipher_suite: src.1.0,
            legacy_compression_method: HELLO_RETRY_REQUESTLEGACY_COMPRESSION_METHOD_CONST,
            extensions: src.1.1.1,
        }
    }
}

impl<'s, 'a: 's> From<&'s HelloRetryRequest<'a>>
for (&'s SessionId<'s>, (CipherSuite, ((), &'s HelloRetryExtensions<'s>))) {
    fn from(v: &'s HelloRetryRequest<'a>) -> Self {
        (&v.legacy_session_id_echo, (v.cipher_suite, ((), &v.extensions)))
    }
}

impl From<(SessionIdOwned, (CipherSuite, ((), HelloRetryExtensionsOwned)))>
for HelloRetryRequestOwned {
    fn from(
        src: (SessionIdOwned, (CipherSuite, ((), HelloRetryExtensionsOwned))),
    ) -> Self {
        Self {
            legacy_session_id_echo: src.0,
            cipher_suite: src.1.0,
            legacy_compression_method: HELLO_RETRY_REQUESTLEGACY_COMPRESSION_METHOD_CONST,
            extensions: src.1.1.1,
        }
    }
}

impl From<HelloRetryRequestOwned>
for (SessionIdOwned, (CipherSuite, ((), HelloRetryExtensionsOwned))) {
    fn from(v: HelloRetryRequestOwned) -> Self {
        (
            v.legacy_session_id_echo,
            (
                v.cipher_suite,
                (HELLO_RETRY_REQUESTLEGACY_COMPRESSION_METHOD_CONST, v.extensions),
            ),
        )
    }
}

pub struct HelloRetryRequestMapper;
impl Mapper for HelloRetryRequestMapper {
    type Src<'p> = (SessionId<'p>, (CipherSuite, ((), HelloRetryExtensions<'p>)));
    type Dst<'p> = HelloRetryRequest<'p>;
    type SrcBorrow<'s> = (
        &'s SessionId<'s>,
        (CipherSuite, ((), &'s HelloRetryExtensions<'s>)),
    );
    type DstBorrow<'s> = &'s HelloRetryRequest<'s>;
    type SrcOwned = (SessionIdOwned, (CipherSuite, ((), HelloRetryExtensionsOwned)));
    type DstOwned = HelloRetryRequestOwned;
}

impl<'a> From<(CertificateEntryData<'a>, CertificateExtensions<'a>)>
for CertificateEntry<'a> {
    fn from(src: (CertificateEntryData<'a>, CertificateExtensions<'a>)) -> Self {
        Self {
            data: src.0,
            extensions: src.1,
        }
    }
}

impl<'s, 'a: 's> From<&'s CertificateEntry<'a>>
for (&'s CertificateEntryData<'s>, &'s CertificateExtensions<'s>) {
    fn from(v: &'s CertificateEntry<'a>) -> Self {
        (&v.data, &v.extensions)
    }
}

impl From<(CertificateEntryDataOwned, CertificateExtensionsOwned)>
for CertificateEntryOwned {
    fn from(src: (CertificateEntryDataOwned, CertificateExtensionsOwned)) -> Self {
        Self {
            data: src.0,
            extensions: src.1,
        }
    }
}

impl From<CertificateEntryOwned>
for (CertificateEntryDataOwned, CertificateExtensionsOwned) {
    fn from(v: CertificateEntryOwned) -> Self {
        (v.data, v.extensions)
    }
}

pub struct CertificateEntryMapper;
impl Mapper for CertificateEntryMapper {
    type Src<'p> = (CertificateEntryData<'p>, CertificateExtensions<'p>);
    type Dst<'p> = CertificateEntry<'p>;
    type SrcBorrow<'s> = (&'s CertificateEntryData<'s>, &'s CertificateExtensions<'s>);
    type DstBorrow<'s> = &'s CertificateEntry<'s>;
    type SrcOwned = (CertificateEntryDataOwned, CertificateExtensionsOwned);
    type DstOwned = CertificateEntryOwned;
}

impl<'a> From<(ContentType, (ProtocolVersion, Opaque0Ffff<'a>))> for TlsCiphertext<'a> {
    fn from(src: (ContentType, (ProtocolVersion, Opaque0Ffff<'a>))) -> Self {
        Self {
            opaque_type: src.0,
            version: src.1.0,
            encrypted_record: src.1.1,
        }
    }
}

impl<'s, 'a: 's> From<&'s TlsCiphertext<'a>>
for (ContentType, (ProtocolVersion, &'s Opaque0Ffff<'s>)) {
    fn from(v: &'s TlsCiphertext<'a>) -> Self {
        (v.opaque_type, (v.version, &v.encrypted_record))
    }
}

impl From<(ContentType, (ProtocolVersion, Opaque0FfffOwned))> for TlsCiphertextOwned {
    fn from(src: (ContentType, (ProtocolVersion, Opaque0FfffOwned))) -> Self {
        Self {
            opaque_type: src.0,
            version: src.1.0,
            encrypted_record: src.1.1,
        }
    }
}

impl From<TlsCiphertextOwned> for (ContentType, (ProtocolVersion, Opaque0FfffOwned)) {
    fn from(v: TlsCiphertextOwned) -> Self {
        (v.opaque_type, (v.version, v.encrypted_record))
    }
}

pub struct TlsCiphertextMapper;
impl Mapper for TlsCiphertextMapper {
    type Src<'p> = (ContentType, (ProtocolVersion, Opaque0Ffff<'p>));
    type Dst<'p> = TlsCiphertext<'p>;
    type SrcBorrow<'s> = (ContentType, (ProtocolVersion, &'s Opaque0Ffff<'s>));
    type DstBorrow<'s> = &'s TlsCiphertext<'s>;
    type SrcOwned = (ContentType, (ProtocolVersion, Opaque0FfffOwned));
    type DstOwned = TlsCiphertextOwned;
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

pub const CERTIFICATE_STATUS_REQUESTSTATUS_TYPE_CONST: u8 = 1;
pub const SH_OR_HRRLEGACY_VERSION_CONST: u16 = 771;
pub const CLIENT_HELLOLEGACY_VERSION_CONST: u16 = 771;
pub const CERTIFICATE_STATUSSTATUS_TYPE_CONST: u8 = 1;
pub const ZERO_BYTEZERO_CONST: u8 = 0;
pub const SERVER_HELLOLEGACY_COMPRESSION_METHOD_CONST: u8 = 0;
pub const HELLO_RETRY_REQUESTLEGACY_COMPRESSION_METHOD_CONST: u8 = 0;
///Type alias for alert_level combinator
pub type AlertLevelCombinatorAlias = Mapped<
    Refined<U8, fn(u8) -> bool>,
    AlertLevelMapper,
>;
///Wrapper struct for alert_level combinator
pub struct AlertLevelCombinator<C = AlertLevelCombinatorAlias>(pub C);
///Type alias for empty combinator
pub type EmptyCombinatorAlias = Fixed<0>;
///Wrapper struct for empty combinator
pub struct EmptyCombinator<C = EmptyCombinatorAlias>(pub C);
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
///Type alias for extension_type combinator
pub type ExtensionTypeCombinatorAlias = U16Be;
///Wrapper struct for extension_type combinator
pub struct ExtensionTypeCombinator<C = ExtensionTypeCombinatorAlias>(pub C);
///Type alias for signature_scheme combinator
pub type SignatureSchemeCombinatorAlias = U16Be;
///Wrapper struct for signature_scheme combinator
pub struct SignatureSchemeCombinator<C = SignatureSchemeCombinatorAlias>(pub C);
///Type alias for signature_scheme_list combinator
pub type SignatureSchemeListCombinatorAlias = Mapped<
    Pair<Refined<U16Be, fn(u16) -> bool>, SignatureSchemeListDep>,
    SignatureSchemeListMapper,
>;
///Wrapper struct for signature_scheme_list combinator
pub struct SignatureSchemeListCombinator<C = SignatureSchemeListCombinatorAlias>(pub C);
///Type alias for opaque_1_ffff combinator
pub type Opaque1FfffCombinatorAlias = Mapped<
    Pair<Refined<U16Be, fn(u16) -> bool>, Opaque1FfffDep>,
    Opaque1FfffMapper,
>;
///Wrapper struct for opaque_1_ffff combinator
pub struct Opaque1FfffCombinator<C = Opaque1FfffCombinatorAlias>(pub C);
///Type alias for distinguished_name combinator
pub type DistinguishedNameCombinatorAlias = Opaque1FfffCombinator;
///Wrapper struct for distinguished_name combinator
pub struct DistinguishedNameCombinator<C = DistinguishedNameCombinatorAlias>(pub C);
///Type alias for certificate_authorities_extension combinator
pub type CertificateAuthoritiesExtensionCombinatorAlias = Mapped<
    Pair<Refined<U16Be, fn(u16) -> bool>, CertificateAuthoritiesExtensionDep>,
    CertificateAuthoritiesExtensionMapper,
>;
///Wrapper struct for certificate_authorities_extension combinator
pub struct CertificateAuthoritiesExtensionCombinator<
    C = CertificateAuthoritiesExtensionCombinatorAlias,
>(
    pub C,
);
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
///Type alias for serialized_sct combinator
pub type SerializedSctCombinatorAlias = Opaque1FfffCombinator;
///Wrapper struct for serialized_sct combinator
pub struct SerializedSctCombinator<C = SerializedSctCombinatorAlias>(pub C);
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
///Type alias for opaque_1_ff combinator
pub type Opaque1FfCombinatorAlias = Mapped<
    Pair<Refined<U8, fn(u8) -> bool>, Opaque1FfDep>,
    Opaque1FfMapper,
>;
///Wrapper struct for opaque_1_ff combinator
pub struct Opaque1FfCombinator<C = Opaque1FfCombinatorAlias>(pub C);
///Type alias for oid_filter combinator
pub type OidFilterCombinatorAlias = Mapped<
    (Opaque1FfCombinator, Opaque0FfffCombinator),
    OidFilterMapper,
>;
///Wrapper struct for oid_filter combinator
pub struct OidFilterCombinator<C = OidFilterCombinatorAlias>(pub C);
///Type alias for oid_filter_extension combinator
pub type OidFilterExtensionCombinatorAlias = Mapped<
    Pair<U16Be, OidFilterExtensionDep>,
    OidFilterExtensionMapper,
>;
///Wrapper struct for oid_filter_extension combinator
pub struct OidFilterExtensionCombinator<C = OidFilterExtensionCombinatorAlias>(pub C);
///Type alias for certificate_request_extension_extension_data combinator
pub type CertificateRequestExtensionExtensionDataCombinatorAlias<'x> = FixedLen<
    'x,
    Dispatch<'x, u16, CertificateRequestExtensionExtensionDataDispatchCase0, 6>,
>;
///Wrapper struct for certificate_request_extension_extension_data combinator
pub struct CertificateRequestExtensionExtensionDataCombinator<
    C = CertificateRequestExtensionExtensionDataCombinatorAlias<'static>,
>(
    pub C,
);
///Type alias for certificate_request_extension combinator
pub type CertificateRequestExtensionCombinatorAlias = Mapped<
    Pair<(ExtensionTypeCombinator, U16Be), CertificateRequestExtensionDep>,
    CertificateRequestExtensionMapper,
>;
///Wrapper struct for certificate_request_extension combinator
pub struct CertificateRequestExtensionCombinator<
    C = CertificateRequestExtensionCombinatorAlias,
>(
    pub C,
);
///Type alias for name_type combinator
pub type NameTypeCombinatorAlias = U8;
///Wrapper struct for name_type combinator
pub struct NameTypeCombinator<C = NameTypeCombinatorAlias>(pub C);
///Type alias for sh_or_hrr combinator
pub type ShOrHrrCombinatorAlias = Mapped<
    (Tag<U16Be, u16>, Pair<Fixed<32>, ShOrHrrDep1>),
    ShOrHrrMapper,
>;
///Wrapper struct for sh_or_hrr combinator
pub struct ShOrHrrCombinator<C = ShOrHrrCombinatorAlias>(pub C);
///Type alias for handshake_type combinator
pub type HandshakeTypeCombinatorAlias = Mapped<
    Refined<U8, fn(u8) -> bool>,
    HandshakeTypeMapper,
>;
///Wrapper struct for handshake_type combinator
pub struct HandshakeTypeCombinator<C = HandshakeTypeCombinatorAlias>(pub C);
///Type alias for session_id combinator
pub type SessionIdCombinatorAlias = Mapped<
    Pair<Refined<U8, fn(u8) -> bool>, SessionIdDep>,
    SessionIdMapper,
>;
///Wrapper struct for session_id combinator
pub struct SessionIdCombinator<C = SessionIdCombinatorAlias>(pub C);
///Type alias for cipher_suite combinator
pub type CipherSuiteCombinatorAlias = U16Be;
///Wrapper struct for cipher_suite combinator
pub struct CipherSuiteCombinator<C = CipherSuiteCombinatorAlias>(pub C);
///Type alias for cipher_suite_list combinator
pub type CipherSuiteListCombinatorAlias = Mapped<
    Pair<Refined<U16Be, fn(u16) -> bool>, CipherSuiteListDep>,
    CipherSuiteListMapper,
>;
///Wrapper struct for cipher_suite_list combinator
pub struct CipherSuiteListCombinator<C = CipherSuiteListCombinatorAlias>(pub C);
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
    u8,
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
///Type alias for named_group combinator
pub type NamedGroupCombinatorAlias = U16Be;
///Wrapper struct for named_group combinator
pub struct NamedGroupCombinator<C = NamedGroupCombinatorAlias>(pub C);
///Type alias for named_group_list combinator
pub type NamedGroupListCombinatorAlias = Mapped<
    Pair<Refined<U16Be, fn(u16) -> bool>, NamedGroupListDep>,
    NamedGroupListMapper,
>;
///Wrapper struct for named_group_list combinator
pub struct NamedGroupListCombinator<C = NamedGroupListCombinatorAlias>(pub C);
///Type alias for protocol_name combinator
pub type ProtocolNameCombinatorAlias = Opaque1FfCombinator;
///Wrapper struct for protocol_name combinator
pub struct ProtocolNameCombinator<C = ProtocolNameCombinatorAlias>(pub C);
///Type alias for protocol_name_list combinator
pub type ProtocolNameListCombinatorAlias = Mapped<
    Pair<Refined<U16Be, fn(u16) -> bool>, ProtocolNameListDep>,
    ProtocolNameListMapper,
>;
///Wrapper struct for protocol_name_list combinator
pub struct ProtocolNameListCombinator<C = ProtocolNameListCombinatorAlias>(pub C);
///Type alias for protocol_version combinator
pub type ProtocolVersionCombinatorAlias = U16Be;
///Wrapper struct for protocol_version combinator
pub struct ProtocolVersionCombinator<C = ProtocolVersionCombinatorAlias>(pub C);
///Type alias for supported_versions_client combinator
pub type SupportedVersionsClientCombinatorAlias = Mapped<
    Pair<Refined<U8, fn(u8) -> bool>, SupportedVersionsClientDep>,
    SupportedVersionsClientMapper,
>;
///Wrapper struct for supported_versions_client combinator
pub struct SupportedVersionsClientCombinator<C = SupportedVersionsClientCombinatorAlias>(
    pub C,
);
///Type alias for key_share_entry combinator
pub type KeyShareEntryCombinatorAlias = Mapped<
    Pair<(NamedGroupCombinator, Refined<U16Be, fn(u16) -> bool>), KeyShareEntryDep>,
    KeyShareEntryMapper,
>;
///Wrapper struct for key_share_entry combinator
pub struct KeyShareEntryCombinator<C = KeyShareEntryCombinatorAlias>(pub C);
///Type alias for key_share_client_hello combinator
pub type KeyShareClientHelloCombinatorAlias = Mapped<
    Pair<U16Be, KeyShareClientHelloDep>,
    KeyShareClientHelloMapper,
>;
///Wrapper struct for key_share_client_hello combinator
pub struct KeyShareClientHelloCombinator<C = KeyShareClientHelloCombinatorAlias>(pub C);
///Type alias for psk_key_exchange_mode combinator
pub type PskKeyExchangeModeCombinatorAlias = U8;
///Wrapper struct for psk_key_exchange_mode combinator
pub struct PskKeyExchangeModeCombinator<C = PskKeyExchangeModeCombinatorAlias>(pub C);
///Type alias for psk_key_exchange_modes combinator
pub type PskKeyExchangeModesCombinatorAlias = Mapped<
    Pair<Refined<U8, fn(u8) -> bool>, PskKeyExchangeModesDep>,
    PskKeyExchangeModesMapper,
>;
///Wrapper struct for psk_key_exchange_modes combinator
pub struct PskKeyExchangeModesCombinator<C = PskKeyExchangeModesCombinatorAlias>(pub C);
///Type alias for psk_identity combinator
pub type PskIdentityCombinatorAlias = Mapped<
    (Opaque1FfffCombinator, U32Be),
    PskIdentityMapper,
>;
///Wrapper struct for psk_identity combinator
pub struct PskIdentityCombinator<C = PskIdentityCombinatorAlias>(pub C);
///Type alias for psk_identities combinator
pub type PskIdentitiesCombinatorAlias = Mapped<
    Pair<Refined<U16Be, fn(u16) -> bool>, PskIdentitiesDep>,
    PskIdentitiesMapper,
>;
///Wrapper struct for psk_identities combinator
pub struct PskIdentitiesCombinator<C = PskIdentitiesCombinatorAlias>(pub C);
///Type alias for psk_binder_entry combinator
pub type PskBinderEntryCombinatorAlias = Mapped<
    Pair<Refined<U8, fn(u8) -> bool>, PskBinderEntryDep>,
    PskBinderEntryMapper,
>;
///Wrapper struct for psk_binder_entry combinator
pub struct PskBinderEntryCombinator<C = PskBinderEntryCombinatorAlias>(pub C);
///Type alias for psk_binder_entries combinator
pub type PskBinderEntriesCombinatorAlias = Mapped<
    Pair<Refined<U16Be, fn(u16) -> bool>, PskBinderEntriesDep>,
    PskBinderEntriesMapper,
>;
///Wrapper struct for psk_binder_entries combinator
pub struct PskBinderEntriesCombinator<C = PskBinderEntriesCombinatorAlias>(pub C);
///Type alias for offered_psks combinator
pub type OfferedPsksCombinatorAlias = Mapped<
    (PskIdentitiesCombinator, PskBinderEntriesCombinator),
    OfferedPsksMapper,
>;
///Wrapper struct for offered_psks combinator
pub struct OfferedPsksCombinator<C = OfferedPsksCombinatorAlias>(pub C);
///Type alias for pre_shared_key_client_extension combinator
pub type PreSharedKeyClientExtensionCombinatorAlias = Mapped<
    OfferedPsksCombinator,
    PreSharedKeyClientExtensionMapper,
>;
///Wrapper struct for pre_shared_key_client_extension combinator
pub struct PreSharedKeyClientExtensionCombinator<
    C = PreSharedKeyClientExtensionCombinatorAlias,
>(
    pub C,
);
///Type alias for max_fragment_length combinator
pub type MaxFragmentLengthCombinatorAlias = U8;
///Wrapper struct for max_fragment_length combinator
pub struct MaxFragmentLengthCombinator<C = MaxFragmentLengthCombinatorAlias>(pub C);
///Type alias for heartbeat_mode combinator
pub type HeartbeatModeCombinatorAlias = U8;
///Wrapper struct for heartbeat_mode combinator
pub struct HeartbeatModeCombinator<C = HeartbeatModeCombinatorAlias>(pub C);
///Type alias for certificate_type combinator
pub type CertificateTypeCombinatorAlias = U8;
///Wrapper struct for certificate_type combinator
pub struct CertificateTypeCombinator<C = CertificateTypeCombinatorAlias>(pub C);
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
///Type alias for cookie combinator
pub type CookieCombinatorAlias = Opaque1FfffCombinator;
///Wrapper struct for cookie combinator
pub struct CookieCombinator<C = CookieCombinatorAlias>(pub C);
///Type alias for client_hello_extension_rest combinator
pub type ClientHelloExtensionRestCombinatorAlias<'x> = Dispatch<
    'x,
    u16,
    ClientHelloExtensionRestDispatchCase,
    10,
>;
///Wrapper struct for client_hello_extension_rest combinator
pub struct ClientHelloExtensionRestCombinator<
    C = ClientHelloExtensionRestCombinatorAlias<'static>,
>(
    pub C,
);
///Type alias for client_hello_extension_extension_data combinator
pub type ClientHelloExtensionExtensionDataCombinatorAlias<'x> = FixedLen<
    'x,
    Dispatch<'x, u16, ClientHelloExtensionExtensionDataDispatchCase0, 9>,
>;
///Wrapper struct for client_hello_extension_extension_data combinator
pub struct ClientHelloExtensionExtensionDataCombinator<
    C = ClientHelloExtensionExtensionDataCombinatorAlias<'static>,
>(
    pub C,
);
///Type alias for client_hello_extension combinator
pub type ClientHelloExtensionCombinatorAlias = Mapped<
    Pair<(ExtensionTypeCombinator, U16Be), ClientHelloExtensionDep>,
    ClientHelloExtensionMapper,
>;
///Wrapper struct for client_hello_extension combinator
pub struct ClientHelloExtensionCombinator<C = ClientHelloExtensionCombinatorAlias>(
    pub C,
);
///Type alias for client_extensions combinator
pub type ClientExtensionsCombinatorAlias = Mapped<
    Pair<Refined<U16Be, fn(u16) -> bool>, ClientExtensionsDep>,
    ClientExtensionsMapper,
>;
///Wrapper struct for client_extensions combinator
pub struct ClientExtensionsCombinator<C = ClientExtensionsCombinatorAlias>(pub C);
///Type alias for client_hello combinator
pub type ClientHelloCombinatorAlias = Mapped<
    (
        Tag<U16Be, u16>,
        (
            Fixed<32>,
            (
                SessionIdCombinator,
                (
                    CipherSuiteListCombinator,
                    (Opaque1FfCombinator, ClientExtensionsCombinator),
                ),
            ),
        ),
    ),
    ClientHelloMapper,
>;
///Wrapper struct for client_hello combinator
pub struct ClientHelloCombinator<C = ClientHelloCombinatorAlias>(pub C);
///Type alias for opaque_0_ff combinator
pub type Opaque0FfCombinatorAlias = Mapped<Pair<U8, Opaque0FfDep>, Opaque0FfMapper>;
///Wrapper struct for opaque_0_ff combinator
pub struct Opaque0FfCombinator<C = Opaque0FfCombinatorAlias>(pub C);
///Type alias for early_data_indication_new_session_ticket combinator
pub type EarlyDataIndicationNewSessionTicketCombinatorAlias = Mapped<
    U32Be,
    EarlyDataIndicationNewSessionTicketMapper,
>;
///Wrapper struct for early_data_indication_new_session_ticket combinator
pub struct EarlyDataIndicationNewSessionTicketCombinator<
    C = EarlyDataIndicationNewSessionTicketCombinatorAlias,
>(
    pub C,
);
///Type alias for new_session_ticket_extension_extension_data combinator
pub type NewSessionTicketExtensionExtensionDataCombinatorAlias<'x> = FixedLen<
    'x,
    Dispatch<'x, u16, NewSessionTicketExtensionExtensionDataDispatchCase0, 1>,
>;
///Wrapper struct for new_session_ticket_extension_extension_data combinator
pub struct NewSessionTicketExtensionExtensionDataCombinator<
    C = NewSessionTicketExtensionExtensionDataCombinatorAlias<'static>,
>(
    pub C,
);
///Type alias for new_session_ticket_extension combinator
pub type NewSessionTicketExtensionCombinatorAlias = Mapped<
    Pair<(ExtensionTypeCombinator, U16Be), NewSessionTicketExtensionDep>,
    NewSessionTicketExtensionMapper,
>;
///Wrapper struct for new_session_ticket_extension combinator
pub struct NewSessionTicketExtensionCombinator<
    C = NewSessionTicketExtensionCombinatorAlias,
>(
    pub C,
);
///Type alias for new_session_ticket_extensions combinator
pub type NewSessionTicketExtensionsCombinatorAlias = Mapped<
    Pair<Refined<U16Be, fn(u16) -> bool>, NewSessionTicketExtensionsDep>,
    NewSessionTicketExtensionsMapper,
>;
///Wrapper struct for new_session_ticket_extensions combinator
pub struct NewSessionTicketExtensionsCombinator<
    C = NewSessionTicketExtensionsCombinatorAlias,
>(
    pub C,
);
///Type alias for new_session_ticket combinator
pub type NewSessionTicketCombinatorAlias = Mapped<
    (
        U32Be,
        (
            U32Be,
            (
                Opaque0FfCombinator,
                (Opaque1FfffCombinator, NewSessionTicketExtensionsCombinator),
            ),
        ),
    ),
    NewSessionTicketMapper,
>;
///Wrapper struct for new_session_ticket combinator
pub struct NewSessionTicketCombinator<C = NewSessionTicketCombinatorAlias>(pub C);
///Type alias for encrypted_extension_extension_data combinator
pub type EncryptedExtensionExtensionDataCombinatorAlias<'x> = FixedLen<
    'x,
    Dispatch<'x, u16, EncryptedExtensionExtensionDataDispatchCase0, 8>,
>;
///Wrapper struct for encrypted_extension_extension_data combinator
pub struct EncryptedExtensionExtensionDataCombinator<
    C = EncryptedExtensionExtensionDataCombinatorAlias<'static>,
>(
    pub C,
);
///Type alias for encrypted_extension combinator
pub type EncryptedExtensionCombinatorAlias = Mapped<
    Pair<(ExtensionTypeCombinator, U16Be), EncryptedExtensionDep>,
    EncryptedExtensionMapper,
>;
///Wrapper struct for encrypted_extension combinator
pub struct EncryptedExtensionCombinator<C = EncryptedExtensionCombinatorAlias>(pub C);
///Type alias for encrypted_extensions combinator
pub type EncryptedExtensionsCombinatorAlias = Mapped<
    Pair<U16Be, EncryptedExtensionsDep>,
    EncryptedExtensionsMapper,
>;
///Wrapper struct for encrypted_extensions combinator
pub struct EncryptedExtensionsCombinator<C = EncryptedExtensionsCombinatorAlias>(pub C);
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
///Type alias for certificate_extension_extension_data combinator
pub type CertificateExtensionExtensionDataCombinatorAlias<'x> = FixedLen<
    'x,
    Dispatch<'x, u16, CertificateExtensionExtensionDataDispatchCase0, 2>,
>;
///Wrapper struct for certificate_extension_extension_data combinator
pub struct CertificateExtensionExtensionDataCombinator<
    C = CertificateExtensionExtensionDataCombinatorAlias<'static>,
>(
    pub C,
);
///Type alias for certificate_extension combinator
pub type CertificateExtensionCombinatorAlias = Mapped<
    Pair<(ExtensionTypeCombinator, U16Be), CertificateExtensionDep>,
    CertificateExtensionMapper,
>;
///Wrapper struct for certificate_extension combinator
pub struct CertificateExtensionCombinator<C = CertificateExtensionCombinatorAlias>(
    pub C,
);
///Type alias for certificate_extensions combinator
pub type CertificateExtensionsCombinatorAlias = Mapped<
    Pair<U16Be, CertificateExtensionsDep>,
    CertificateExtensionsMapper,
>;
///Wrapper struct for certificate_extensions combinator
pub struct CertificateExtensionsCombinator<C = CertificateExtensionsCombinatorAlias>(
    pub C,
);
///Type alias for certificate_entry_opaque combinator
pub type CertificateEntryOpaqueCombinatorAlias = Mapped<
    (Opaque1FfffffCombinator, CertificateExtensionsCombinator),
    CertificateEntryOpaqueMapper,
>;
///Wrapper struct for certificate_entry_opaque combinator
pub struct CertificateEntryOpaqueCombinator<C = CertificateEntryOpaqueCombinatorAlias>(
    pub C,
);
///Type alias for certificate_list combinator
pub type CertificateListCombinatorAlias = Mapped<
    Pair<U24Be, CertificateListDep>,
    CertificateListMapper,
>;
///Wrapper struct for certificate_list combinator
pub struct CertificateListCombinator<C = CertificateListCombinatorAlias>(pub C);
///Type alias for certificate combinator
pub type CertificateCombinatorAlias = Mapped<
    (Opaque0FfCombinator, CertificateListCombinator),
    CertificateMapper,
>;
///Wrapper struct for certificate combinator
pub struct CertificateCombinator<C = CertificateCombinatorAlias>(pub C);
///Type alias for certificate_request_extensions combinator
pub type CertificateRequestExtensionsCombinatorAlias = Mapped<
    Pair<Refined<U16Be, fn(u16) -> bool>, CertificateRequestExtensionsDep>,
    CertificateRequestExtensionsMapper,
>;
///Wrapper struct for certificate_request_extensions combinator
pub struct CertificateRequestExtensionsCombinator<
    C = CertificateRequestExtensionsCombinatorAlias,
>(
    pub C,
);
///Type alias for certificate_request combinator
pub type CertificateRequestCombinatorAlias = Mapped<
    (Opaque0FfCombinator, CertificateRequestExtensionsCombinator),
    CertificateRequestMapper,
>;
///Wrapper struct for certificate_request combinator
pub struct CertificateRequestCombinator<C = CertificateRequestCombinatorAlias>(pub C);
///Type alias for certificate_verify combinator
pub type CertificateVerifyCombinatorAlias = Mapped<
    (SignatureSchemeCombinator, Opaque0FfffCombinator),
    CertificateVerifyMapper,
>;
///Wrapper struct for certificate_verify combinator
pub struct CertificateVerifyCombinator<C = CertificateVerifyCombinatorAlias>(pub C);
///Type alias for finished combinator
pub type FinishedCombinatorAlias<'x> = Dispatch<'x, u24, FinishedDispatchCase, 5>;
///Wrapper struct for finished combinator
pub struct FinishedCombinator<C = FinishedCombinatorAlias<'static>>(pub C);
///Type alias for key_update_request combinator
pub type KeyUpdateRequestCombinatorAlias = Mapped<
    Refined<U8, fn(u8) -> bool>,
    KeyUpdateRequestMapper,
>;
///Wrapper struct for key_update_request combinator
pub struct KeyUpdateRequestCombinator<C = KeyUpdateRequestCombinatorAlias>(pub C);
///Type alias for key_update combinator
pub type KeyUpdateCombinatorAlias = Mapped<KeyUpdateRequestCombinator, KeyUpdateMapper>;
///Wrapper struct for key_update combinator
pub struct KeyUpdateCombinator<C = KeyUpdateCombinatorAlias>(pub C);
///Type alias for handshake_msg combinator
pub type HandshakeMsgCombinatorAlias<'x> = FixedLen<
    'x,
    Dispatch<'x, HandshakeType, HandshakeMsgDispatchCase0, 10>,
>;
///Wrapper struct for handshake_msg combinator
pub struct HandshakeMsgCombinator<C = HandshakeMsgCombinatorAlias<'static>>(pub C);
///Type alias for handshake combinator
pub type HandshakeCombinatorAlias = Mapped<
    Pair<(HandshakeTypeCombinator, U24Be), HandshakeDep>,
    HandshakeMapper,
>;
///Wrapper struct for handshake combinator
pub struct HandshakeCombinator<C = HandshakeCombinatorAlias>(pub C);
///Type alias for zero_byte combinator
pub type ZeroByteCombinatorAlias = Mapped<Tag<U8, u8>, ZeroByteMapper>;
///Wrapper struct for zero_byte combinator
pub struct ZeroByteCombinator<C = ZeroByteCombinatorAlias>(pub C);
///Type alias for padding_extension combinator
pub type PaddingExtensionCombinatorAlias = Mapped<
    Pair<U16Be, PaddingExtensionDep>,
    PaddingExtensionMapper,
>;
///Wrapper struct for padding_extension combinator
pub struct PaddingExtensionCombinator<C = PaddingExtensionCombinatorAlias>(pub C);
///Type alias for extension combinator
pub type ExtensionCombinatorAlias = Mapped<
    (ExtensionTypeCombinator, Opaque0FfffCombinator),
    ExtensionMapper,
>;
///Wrapper struct for extension combinator
pub struct ExtensionCombinator<C = ExtensionCombinatorAlias>(pub C);
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
///Type alias for content_type combinator
pub type ContentTypeCombinatorAlias = Mapped<
    Refined<U8, fn(u8) -> bool>,
    ContentTypeMapper,
>;
///Wrapper struct for content_type combinator
pub struct ContentTypeCombinator<C = ContentTypeCombinatorAlias>(pub C);
///Type alias for tls_plaintext combinator
pub type TlsPlaintextCombinatorAlias = Mapped<
    (ContentTypeCombinator, (ProtocolVersionCombinator, Opaque0FfffCombinator)),
    TlsPlaintextMapper,
>;
///Wrapper struct for tls_plaintext combinator
pub struct TlsPlaintextCombinator<C = TlsPlaintextCombinatorAlias>(pub C);
///Type alias for alert_description combinator
pub type AlertDescriptionCombinatorAlias = Mapped<
    Refined<U8, fn(u8) -> bool>,
    AlertDescriptionMapper,
>;
///Wrapper struct for alert_description combinator
pub struct AlertDescriptionCombinator<C = AlertDescriptionCombinatorAlias>(pub C);
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
///Type alias for use_srtp_data combinator
pub type UseSrtpDataCombinatorAlias = Mapped<
    (SrtpProtectionProfilesCombinator, Opaque0FfCombinator),
    UseSrtpDataMapper,
>;
///Wrapper struct for use_srtp_data combinator
pub struct UseSrtpDataCombinator<C = UseSrtpDataCombinatorAlias>(pub C);
///Type alias for supported_versions_server combinator
pub type SupportedVersionsServerCombinatorAlias = ProtocolVersionCombinator;
///Wrapper struct for supported_versions_server combinator
pub struct SupportedVersionsServerCombinator<C = SupportedVersionsServerCombinatorAlias>(
    pub C,
);
///Type alias for hello_retry_extension_extension_data combinator
pub type HelloRetryExtensionExtensionDataCombinatorAlias<'x> = FixedLen<
    'x,
    Dispatch<'x, u16, HelloRetryExtensionExtensionDataDispatchCase0, 3>,
>;
///Wrapper struct for hello_retry_extension_extension_data combinator
pub struct HelloRetryExtensionExtensionDataCombinator<
    C = HelloRetryExtensionExtensionDataCombinatorAlias<'static>,
>(
    pub C,
);
///Type alias for finished_opaque combinator
pub type FinishedOpaqueCombinatorAlias = Variable;
///Wrapper struct for finished_opaque combinator
pub struct FinishedOpaqueCombinator<C = FinishedOpaqueCombinatorAlias>(pub C);
///Type alias for pre_shared_key_server_extension combinator
pub type PreSharedKeyServerExtensionCombinatorAlias = Mapped<
    U16Be,
    PreSharedKeyServerExtensionMapper,
>;
///Wrapper struct for pre_shared_key_server_extension combinator
pub struct PreSharedKeyServerExtensionCombinator<
    C = PreSharedKeyServerExtensionCombinatorAlias,
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
///Type alias for hello_retry_extension combinator
pub type HelloRetryExtensionCombinatorAlias = Mapped<
    Pair<(ExtensionTypeCombinator, U16Be), HelloRetryExtensionDep>,
    HelloRetryExtensionMapper,
>;
///Wrapper struct for hello_retry_extension combinator
pub struct HelloRetryExtensionCombinator<C = HelloRetryExtensionCombinatorAlias>(pub C);
///Type alias for alert combinator
pub type AlertCombinatorAlias = Mapped<
    (AlertLevelCombinator, AlertDescriptionCombinator),
    AlertMapper,
>;
///Wrapper struct for alert combinator
pub struct AlertCombinator<C = AlertCombinatorAlias>(pub C);
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
///Type alias for unknown_extension combinator
pub type UnknownExtensionCombinatorAlias = Opaque0FfffCombinator;
///Wrapper struct for unknown_extension combinator
pub struct UnknownExtensionCombinator<C = UnknownExtensionCombinatorAlias>(pub C);
///Type alias for hello_retry_extensions combinator
pub type HelloRetryExtensionsCombinatorAlias = Mapped<
    Pair<Refined<U16Be, fn(u16) -> bool>, HelloRetryExtensionsDep>,
    HelloRetryExtensionsMapper,
>;
///Wrapper struct for hello_retry_extensions combinator
pub struct HelloRetryExtensionsCombinator<C = HelloRetryExtensionsCombinatorAlias>(
    pub C,
);
///Type alias for sever_hello_extension_extension_data combinator
pub type SeverHelloExtensionExtensionDataCombinatorAlias<'x> = FixedLen<
    'x,
    Dispatch<'x, u16, SeverHelloExtensionExtensionDataDispatchCase0, 3>,
>;
///Wrapper struct for sever_hello_extension_extension_data combinator
pub struct SeverHelloExtensionExtensionDataCombinator<
    C = SeverHelloExtensionExtensionDataCombinatorAlias<'static>,
>(
    pub C,
);
///Type alias for sever_hello_extension combinator
pub type SeverHelloExtensionCombinatorAlias = Mapped<
    Pair<(ExtensionTypeCombinator, U16Be), SeverHelloExtensionDep>,
    SeverHelloExtensionMapper,
>;
///Wrapper struct for sever_hello_extension combinator
pub struct SeverHelloExtensionCombinator<C = SeverHelloExtensionCombinatorAlias>(pub C);
///Type alias for server_extensions combinator
pub type ServerExtensionsCombinatorAlias = Mapped<
    Pair<Refined<U16Be, fn(u16) -> bool>, ServerExtensionsDep>,
    ServerExtensionsMapper,
>;
///Wrapper struct for server_extensions combinator
pub struct ServerExtensionsCombinator<C = ServerExtensionsCombinatorAlias>(pub C);
///Type alias for digest_size combinator
pub type DigestSizeCombinatorAlias = U24Be;
///Wrapper struct for digest_size combinator
pub struct DigestSizeCombinator<C = DigestSizeCombinatorAlias>(pub C);
///Type alias for heartbeat_extension combinator
pub type HeartbeatExtensionCombinatorAlias = Mapped<
    HeartbeatModeCombinator,
    HeartbeatExtensionMapper,
>;
///Wrapper struct for heartbeat_extension combinator
pub struct HeartbeatExtensionCombinator<C = HeartbeatExtensionCombinatorAlias>(pub C);
///Type alias for ec_point_format combinator
pub type EcPointFormatCombinatorAlias = U8;
///Wrapper struct for ec_point_format combinator
pub struct EcPointFormatCombinator<C = EcPointFormatCombinatorAlias>(pub C);
///Type alias for server_hello combinator
pub type ServerHelloCombinatorAlias = Mapped<
    (
        SessionIdCombinator,
        (CipherSuiteCombinator, (Tag<U8, u8>, ServerExtensionsCombinator)),
    ),
    ServerHelloMapper,
>;
///Wrapper struct for server_hello combinator
pub struct ServerHelloCombinator<C = ServerHelloCombinatorAlias>(pub C);
///Type alias for ec_point_format_list combinator
pub type EcPointFormatListCombinatorAlias = Mapped<
    Pair<Refined<U8, fn(u8) -> bool>, EcPointFormatListDep>,
    EcPointFormatListMapper,
>;
///Wrapper struct for ec_point_format_list combinator
pub struct EcPointFormatListCombinator<C = EcPointFormatListCombinatorAlias>(pub C);
///Type alias for hello_retry_request combinator
pub type HelloRetryRequestCombinatorAlias = Mapped<
    (
        SessionIdCombinator,
        (CipherSuiteCombinator, (Tag<U8, u8>, HelloRetryExtensionsCombinator)),
    ),
    HelloRetryRequestMapper,
>;
///Wrapper struct for hello_retry_request combinator
pub struct HelloRetryRequestCombinator<C = HelloRetryRequestCombinatorAlias>(pub C);
///Type alias for certificate_entry_data combinator
pub type CertificateEntryDataCombinatorAlias<'x> = Dispatch<
    'x,
    u8,
    CertificateEntryDataDispatchCase,
    2,
>;
///Wrapper struct for certificate_entry_data combinator
pub struct CertificateEntryDataCombinator<
    C = CertificateEntryDataCombinatorAlias<'static>,
>(
    pub C,
);
///Type alias for certificate_entry combinator
pub type CertificateEntryCombinatorAlias<'x> = Mapped<
    (CertificateEntryDataCombinator, CertificateExtensionsCombinator),
    CertificateEntryMapper,
>;
///Wrapper struct for certificate_entry combinator
pub struct CertificateEntryCombinator<C = CertificateEntryCombinatorAlias<'static>>(
    pub C,
);
///Type alias for tls_ciphertext combinator
pub type TlsCiphertextCombinatorAlias = Mapped<
    (ContentTypeCombinator, (ProtocolVersionCombinator, Opaque0FfffCombinator)),
    TlsCiphertextMapper,
>;
///Wrapper struct for tls_ciphertext combinator
pub struct TlsCiphertextCombinator<C = TlsCiphertextCombinatorAlias>(pub C);
///Type alias for opaque_0_ffffff combinator
pub type Opaque0FfffffCombinatorAlias = Mapped<
    Pair<U24Be, Opaque0FfffffDep>,
    Opaque0FfffffMapper,
>;
///Wrapper struct for opaque_0_ffffff combinator
pub struct Opaque0FfffffCombinator<C = Opaque0FfffffCombinatorAlias>(pub C);
///Constructor for alert_level combinator
pub fn alert_level() -> AlertLevelCombinator {
    AlertLevelCombinator(
        Mapped::new(
            Refined {
                inner: U8,
                predicate: |v: u8| (v as i128 == 1 || v as i128 == 2),
            },
            AlertLevelMapper,
        ),
    )
}

///Constructor for empty combinator
pub fn empty() -> EmptyCombinator {
    EmptyCombinator(Fixed::<0>)
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

///Constructor for extension_type combinator
pub fn extension_type() -> ExtensionTypeCombinator {
    ExtensionTypeCombinator(U16Be)
}

///Constructor for signature_scheme combinator
pub fn signature_scheme() -> SignatureSchemeCombinator {
    SignatureSchemeCombinator(U16Be)
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

///Constructor for distinguished_name combinator
pub fn distinguished_name() -> DistinguishedNameCombinator {
    DistinguishedNameCombinator(opaque_1_ffff())
}

///Constructor for certificate_authorities_extension combinator
pub fn certificate_authorities_extension() -> CertificateAuthoritiesExtensionCombinator {
    CertificateAuthoritiesExtensionCombinator(
        Mapped::new(
            Pair::new(
                Refined {
                    inner: U16Be,
                    predicate: |v: u16| v as i128 >= 3 && v as i128 <= 65535,
                },
                CertificateAuthoritiesExtensionDep {
                },
            ),
            CertificateAuthoritiesExtensionMapper,
        ),
    )
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

///Constructor for serialized_sct combinator
pub fn serialized_sct() -> SerializedSctCombinator {
    SerializedSctCombinator(opaque_1_ffff())
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

///Constructor for oid_filter combinator
pub fn oid_filter() -> OidFilterCombinator {
    OidFilterCombinator(Mapped::new((opaque_1_ff(), opaque_0_ffff()), OidFilterMapper))
}

///Constructor for oid_filter_extension combinator
pub fn oid_filter_extension() -> OidFilterExtensionCombinator {
    OidFilterExtensionCombinator(
        Mapped::new(Pair::new(U16Be, OidFilterExtensionDep {}), OidFilterExtensionMapper),
    )
}

///Constructor for certificate_request_extension_extension_data combinator
pub fn certificate_request_extension_extension_data<'a, ExtLenArg, ExtensionTypeArg>(
    ext_len: ExtLenArg,
    extension_type: ExtensionTypeArg,
) -> CertificateRequestExtensionExtensionDataCombinator<
    CertificateRequestExtensionExtensionDataCombinatorAlias<'a>,
>
where
    ExtLenArg: LengthParam<'a, u16>,
    ExtensionTypeArg: RuntimeValParam<'a, u16>,
{
    CertificateRequestExtensionExtensionDataCombinator(
        FixedLen(
            ext_len.into_length(),
            Dispatch::new(
                extension_type.into_runtime_value(),
                [
                    (
                        ExtensionType::SignatureAlgorithms,
                        CertificateRequestExtensionExtensionDataDispatchCase0::V1(
                            signature_scheme_list(),
                        ),
                    ),
                    (
                        ExtensionType::CertificateAuthorities,
                        CertificateRequestExtensionExtensionDataDispatchCase0::V2(
                            certificate_authorities_extension(),
                        ),
                    ),
                    (
                        ExtensionType::SignatureAlgorithmsCert,
                        CertificateRequestExtensionExtensionDataDispatchCase0::V3(
                            signature_scheme_list(),
                        ),
                    ),
                    (
                        ExtensionType::StatusRequest,
                        CertificateRequestExtensionExtensionDataDispatchCase0::V4(
                            certificate_status_request(),
                        ),
                    ),
                    (
                        ExtensionType::SignedCertificateTimeStamp,
                        CertificateRequestExtensionExtensionDataDispatchCase0::V5(
                            signed_certificate_timestamp_list(),
                        ),
                    ),
                    (
                        ExtensionType::OidFilters,
                        CertificateRequestExtensionExtensionDataDispatchCase0::V6(
                            oid_filter_extension(),
                        ),
                    ),
                ],
                Some(Variable(ext_len.value() as usize)),
            ),
        ),
    )
}

///Constructor for certificate_request_extension combinator
pub fn certificate_request_extension() -> CertificateRequestExtensionCombinator {
    CertificateRequestExtensionCombinator(
        Mapped::new(
            Pair::new((extension_type(), U16Be), CertificateRequestExtensionDep {}),
            CertificateRequestExtensionMapper,
        ),
    )
}

///Constructor for name_type combinator
pub fn name_type() -> NameTypeCombinator {
    NameTypeCombinator(U8)
}

///Constructor for sh_or_hrr combinator
pub fn sh_or_hrr() -> ShOrHrrCombinator {
    ShOrHrrCombinator(
        Mapped::new(
            (
                Tag::new(U16Be, SH_OR_HRRLEGACY_VERSION_CONST),
                Pair::new(Fixed::<32>, ShOrHrrDep1 {}),
            ),
            ShOrHrrMapper,
        ),
    )
}

///Constructor for handshake_type combinator
pub fn handshake_type() -> HandshakeTypeCombinator {
    HandshakeTypeCombinator(
        Mapped::new(
            Refined {
                inner: U8,
                predicate: |v: u8| {
                    (v as i128 == 1 || v as i128 == 2 || v as i128 == 4 || v as i128 == 5
                        || v as i128 == 8 || v as i128 == 11 || v as i128 == 13
                        || v as i128 == 15 || v as i128 == 20 || v as i128 == 24)
                },
            },
            HandshakeTypeMapper,
        ),
    )
}

///Constructor for session_id combinator
pub fn session_id() -> SessionIdCombinator {
    SessionIdCombinator(
        Mapped::new(
            Pair::new(
                Refined {
                    inner: U8,
                    predicate: |v: u8| (v as i128 >= 0 && v as i128 <= 32),
                },
                SessionIdDep {},
            ),
            SessionIdMapper,
        ),
    )
}

///Constructor for cipher_suite combinator
pub fn cipher_suite() -> CipherSuiteCombinator {
    CipherSuiteCombinator(U16Be)
}

///Constructor for cipher_suite_list combinator
pub fn cipher_suite_list() -> CipherSuiteListCombinator {
    CipherSuiteListCombinator(
        Mapped::new(
            Pair::new(
                Refined {
                    inner: U16Be,
                    predicate: |v: u16| v as i128 >= 2 && v as i128 <= 65534,
                },
                CipherSuiteListDep {},
            ),
            CipherSuiteListMapper,
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
    NameTypeArg: RuntimeValParam<'a, u8>,
{
    ServerNameNameCombinator(
        Dispatch::new(
            name_type.into_runtime_value(),
            [(NameType::HostName, ServerNameNameDispatchCase::V1(host_name()))],
            Some(unknown_name()),
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

///Constructor for named_group combinator
pub fn named_group() -> NamedGroupCombinator {
    NamedGroupCombinator(U16Be)
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

///Constructor for protocol_name combinator
pub fn protocol_name() -> ProtocolNameCombinator {
    ProtocolNameCombinator(opaque_1_ff())
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

///Constructor for protocol_version combinator
pub fn protocol_version() -> ProtocolVersionCombinator {
    ProtocolVersionCombinator(U16Be)
}

///Constructor for supported_versions_client combinator
pub fn supported_versions_client() -> SupportedVersionsClientCombinator {
    SupportedVersionsClientCombinator(
        Mapped::new(
            Pair::new(
                Refined {
                    inner: U8,
                    predicate: |v: u8| v as i128 >= 2 && v as i128 <= 254,
                },
                SupportedVersionsClientDep {},
            ),
            SupportedVersionsClientMapper,
        ),
    )
}

///Constructor for key_share_entry combinator
pub fn key_share_entry() -> KeyShareEntryCombinator {
    KeyShareEntryCombinator(
        Mapped::new(
            Pair::new(
                (
                    named_group(),
                    Refined {
                        inner: U16Be,
                        predicate: |v: u16| v as i128 >= 1 && v as i128 <= 65535,
                    },
                ),
                KeyShareEntryDep {},
            ),
            KeyShareEntryMapper,
        ),
    )
}

///Constructor for key_share_client_hello combinator
pub fn key_share_client_hello() -> KeyShareClientHelloCombinator {
    KeyShareClientHelloCombinator(
        Mapped::new(
            Pair::new(U16Be, KeyShareClientHelloDep {}),
            KeyShareClientHelloMapper,
        ),
    )
}

///Constructor for psk_key_exchange_mode combinator
pub fn psk_key_exchange_mode() -> PskKeyExchangeModeCombinator {
    PskKeyExchangeModeCombinator(U8)
}

///Constructor for psk_key_exchange_modes combinator
pub fn psk_key_exchange_modes() -> PskKeyExchangeModesCombinator {
    PskKeyExchangeModesCombinator(
        Mapped::new(
            Pair::new(
                Refined {
                    inner: U8,
                    predicate: |v: u8| v as i128 >= 1 && v as i128 <= 255,
                },
                PskKeyExchangeModesDep {},
            ),
            PskKeyExchangeModesMapper,
        ),
    )
}

///Constructor for psk_identity combinator
pub fn psk_identity() -> PskIdentityCombinator {
    PskIdentityCombinator(Mapped::new((opaque_1_ffff(), U32Be), PskIdentityMapper))
}

///Constructor for psk_identities combinator
pub fn psk_identities() -> PskIdentitiesCombinator {
    PskIdentitiesCombinator(
        Mapped::new(
            Pair::new(
                Refined {
                    inner: U16Be,
                    predicate: |v: u16| v as i128 >= 7 && v as i128 <= 65535,
                },
                PskIdentitiesDep {},
            ),
            PskIdentitiesMapper,
        ),
    )
}

///Constructor for psk_binder_entry combinator
pub fn psk_binder_entry() -> PskBinderEntryCombinator {
    PskBinderEntryCombinator(
        Mapped::new(
            Pair::new(
                Refined {
                    inner: U8,
                    predicate: |v: u8| (v as i128 >= 32 && v as i128 <= 255),
                },
                PskBinderEntryDep {},
            ),
            PskBinderEntryMapper,
        ),
    )
}

///Constructor for psk_binder_entries combinator
pub fn psk_binder_entries() -> PskBinderEntriesCombinator {
    PskBinderEntriesCombinator(
        Mapped::new(
            Pair::new(
                Refined {
                    inner: U16Be,
                    predicate: |v: u16| v as i128 >= 33 && v as i128 <= 65535,
                },
                PskBinderEntriesDep {},
            ),
            PskBinderEntriesMapper,
        ),
    )
}

///Constructor for offered_psks combinator
pub fn offered_psks() -> OfferedPsksCombinator {
    OfferedPsksCombinator(
        Mapped::new((psk_identities(), psk_binder_entries()), OfferedPsksMapper),
    )
}

///Constructor for pre_shared_key_client_extension combinator
pub fn pre_shared_key_client_extension() -> PreSharedKeyClientExtensionCombinator {
    PreSharedKeyClientExtensionCombinator(
        Mapped::new(offered_psks(), PreSharedKeyClientExtensionMapper),
    )
}

///Constructor for max_fragment_length combinator
pub fn max_fragment_length() -> MaxFragmentLengthCombinator {
    MaxFragmentLengthCombinator(U8)
}

///Constructor for heartbeat_mode combinator
pub fn heartbeat_mode() -> HeartbeatModeCombinator {
    HeartbeatModeCombinator(U8)
}

///Constructor for certificate_type combinator
pub fn certificate_type() -> CertificateTypeCombinator {
    CertificateTypeCombinator(U8)
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

///Constructor for cookie combinator
pub fn cookie() -> CookieCombinator {
    CookieCombinator(opaque_1_ffff())
}

///Constructor for client_hello_extension_rest combinator
pub fn client_hello_extension_rest<'a, ExtensionTypeArg>(
    ext_len: u16,
    extension_type: ExtensionTypeArg,
) -> ClientHelloExtensionRestCombinator<ClientHelloExtensionRestCombinatorAlias<'a>>
where
    ExtensionTypeArg: RuntimeValParam<'a, u16>,
{
    ClientHelloExtensionRestCombinator(
        Dispatch::new(
            extension_type.into_runtime_value(),
            [
                (
                    ExtensionType::MaxFragmentLength,
                    ClientHelloExtensionRestDispatchCase::V1(max_fragment_length()),
                ),
                (
                    ExtensionType::Heartbeat,
                    ClientHelloExtensionRestDispatchCase::V2(heartbeat_mode()),
                ),
                (
                    ExtensionType::SignedCertificateTimeStamp,
                    ClientHelloExtensionRestDispatchCase::V3(
                        signed_certificate_timestamp_list(),
                    ),
                ),
                (
                    ExtensionType::ClientCertificateType,
                    ClientHelloExtensionRestDispatchCase::V4(
                        client_cert_type_client_extension(),
                    ),
                ),
                (
                    ExtensionType::ServerCertificateType,
                    ClientHelloExtensionRestDispatchCase::V5(
                        server_cert_type_client_extension(),
                    ),
                ),
                (
                    ExtensionType::Padding,
                    ClientHelloExtensionRestDispatchCase::V6(Variable(ext_len as usize)),
                ),
                (
                    ExtensionType::Cookie,
                    ClientHelloExtensionRestDispatchCase::V7(cookie()),
                ),
                (
                    ExtensionType::CertificateAuthorities,
                    ClientHelloExtensionRestDispatchCase::V8(
                        certificate_authorities_extension(),
                    ),
                ),
                (
                    ExtensionType::OidFilters,
                    ClientHelloExtensionRestDispatchCase::V9(oid_filter_extension()),
                ),
                (
                    ExtensionType::SignatureAlgorithmsCert,
                    ClientHelloExtensionRestDispatchCase::V10(signature_scheme_list()),
                ),
            ],
            Some(Variable(ext_len as usize)),
        ),
    )
}

///Constructor for client_hello_extension_extension_data combinator
pub fn client_hello_extension_extension_data<'a, ExtLenArg, ExtensionTypeArg>(
    ext_len: ExtLenArg,
    extension_type: ExtensionTypeArg,
) -> ClientHelloExtensionExtensionDataCombinator<
    ClientHelloExtensionExtensionDataCombinatorAlias<'a>,
>
where
    ExtLenArg: LengthParam<'a, u16>,
    ExtensionTypeArg: RuntimeValParam<'a, u16>,
{
    ClientHelloExtensionExtensionDataCombinator(
        FixedLen(
            ext_len.into_length(),
            Dispatch::new(
                extension_type.into_runtime_value(),
                [
                    (
                        ExtensionType::ServerName,
                        ClientHelloExtensionExtensionDataDispatchCase0::V1(
                            server_name_list(),
                        ),
                    ),
                    (
                        ExtensionType::SignatureAlgorithms,
                        ClientHelloExtensionExtensionDataDispatchCase0::V2(
                            signature_scheme_list(),
                        ),
                    ),
                    (
                        ExtensionType::SupportedGroups,
                        ClientHelloExtensionExtensionDataDispatchCase0::V3(
                            named_group_list(),
                        ),
                    ),
                    (
                        ExtensionType::StatusRequest,
                        ClientHelloExtensionExtensionDataDispatchCase0::V4(
                            certificate_status_request(),
                        ),
                    ),
                    (
                        ExtensionType::ApplicationLayerProtocolNegotiation,
                        ClientHelloExtensionExtensionDataDispatchCase0::V5(
                            protocol_name_list(),
                        ),
                    ),
                    (
                        ExtensionType::SupportedVersions,
                        ClientHelloExtensionExtensionDataDispatchCase0::V6(
                            supported_versions_client(),
                        ),
                    ),
                    (
                        ExtensionType::KeyShare,
                        ClientHelloExtensionExtensionDataDispatchCase0::V7(
                            key_share_client_hello(),
                        ),
                    ),
                    (
                        ExtensionType::PskKeyExchangeModes,
                        ClientHelloExtensionExtensionDataDispatchCase0::V8(
                            psk_key_exchange_modes(),
                        ),
                    ),
                    (
                        ExtensionType::PreSharedKey,
                        ClientHelloExtensionExtensionDataDispatchCase0::V9(
                            pre_shared_key_client_extension(),
                        ),
                    ),
                ],
                Some(client_hello_extension_rest(ext_len.value(), extension_type)),
            ),
        ),
    )
}

///Constructor for client_hello_extension combinator
pub fn client_hello_extension() -> ClientHelloExtensionCombinator {
    ClientHelloExtensionCombinator(
        Mapped::new(
            Pair::new((extension_type(), U16Be), ClientHelloExtensionDep {}),
            ClientHelloExtensionMapper,
        ),
    )
}

///Constructor for client_extensions combinator
pub fn client_extensions() -> ClientExtensionsCombinator {
    ClientExtensionsCombinator(
        Mapped::new(
            Pair::new(
                Refined {
                    inner: U16Be,
                    predicate: |v: u16| v as i128 >= 8 && v as i128 <= 65535,
                },
                ClientExtensionsDep {},
            ),
            ClientExtensionsMapper,
        ),
    )
}

///Constructor for client_hello combinator
pub fn client_hello() -> ClientHelloCombinator {
    ClientHelloCombinator(
        Mapped::new(
            (
                Tag::new(U16Be, CLIENT_HELLOLEGACY_VERSION_CONST),
                (
                    Fixed::<32>,
                    (
                        session_id(),
                        (cipher_suite_list(), (opaque_1_ff(), client_extensions())),
                    ),
                ),
            ),
            ClientHelloMapper,
        ),
    )
}

///Constructor for opaque_0_ff combinator
pub fn opaque_0_ff() -> Opaque0FfCombinator {
    Opaque0FfCombinator(Mapped::new(Pair::new(U8, Opaque0FfDep {}), Opaque0FfMapper))
}

///Constructor for early_data_indication_new_session_ticket combinator
pub fn early_data_indication_new_session_ticket() -> EarlyDataIndicationNewSessionTicketCombinator {
    EarlyDataIndicationNewSessionTicketCombinator(
        Mapped::new(U32Be, EarlyDataIndicationNewSessionTicketMapper),
    )
}

///Constructor for new_session_ticket_extension_extension_data combinator
pub fn new_session_ticket_extension_extension_data<'a, ExtLenArg, ExtensionTypeArg>(
    ext_len: ExtLenArg,
    extension_type: ExtensionTypeArg,
) -> NewSessionTicketExtensionExtensionDataCombinator<
    NewSessionTicketExtensionExtensionDataCombinatorAlias<'a>,
>
where
    ExtLenArg: LengthParam<'a, u16>,
    ExtensionTypeArg: RuntimeValParam<'a, u16>,
{
    NewSessionTicketExtensionExtensionDataCombinator(
        FixedLen(
            ext_len.into_length(),
            Dispatch::new(
                extension_type.into_runtime_value(),
                [
                    (
                        ExtensionType::EarlyData,
                        NewSessionTicketExtensionExtensionDataDispatchCase0::V1(
                            early_data_indication_new_session_ticket(),
                        ),
                    ),
                ],
                Some(Variable(ext_len.value() as usize)),
            ),
        ),
    )
}

///Constructor for new_session_ticket_extension combinator
pub fn new_session_ticket_extension() -> NewSessionTicketExtensionCombinator {
    NewSessionTicketExtensionCombinator(
        Mapped::new(
            Pair::new((extension_type(), U16Be), NewSessionTicketExtensionDep {}),
            NewSessionTicketExtensionMapper,
        ),
    )
}

///Constructor for new_session_ticket_extensions combinator
pub fn new_session_ticket_extensions() -> NewSessionTicketExtensionsCombinator {
    NewSessionTicketExtensionsCombinator(
        Mapped::new(
            Pair::new(
                Refined {
                    inner: U16Be,
                    predicate: |v: u16| v as i128 >= 0 && v as i128 <= 65534,
                },
                NewSessionTicketExtensionsDep {},
            ),
            NewSessionTicketExtensionsMapper,
        ),
    )
}

///Constructor for new_session_ticket combinator
pub fn new_session_ticket() -> NewSessionTicketCombinator {
    NewSessionTicketCombinator(
        Mapped::new(
            (
                U32Be,
                (
                    U32Be,
                    (opaque_0_ff(), (opaque_1_ffff(), new_session_ticket_extensions())),
                ),
            ),
            NewSessionTicketMapper,
        ),
    )
}

///Constructor for encrypted_extension_extension_data combinator
pub fn encrypted_extension_extension_data<'a, ExtLenArg, ExtensionTypeArg>(
    ext_len: ExtLenArg,
    extension_type: ExtensionTypeArg,
) -> EncryptedExtensionExtensionDataCombinator<
    EncryptedExtensionExtensionDataCombinatorAlias<'a>,
>
where
    ExtLenArg: LengthParam<'a, u16>,
    ExtensionTypeArg: RuntimeValParam<'a, u16>,
{
    EncryptedExtensionExtensionDataCombinator(
        FixedLen(
            ext_len.into_length(),
            Dispatch::new(
                extension_type.into_runtime_value(),
                [
                    (
                        ExtensionType::ServerName,
                        EncryptedExtensionExtensionDataDispatchCase0::V1(empty()),
                    ),
                    (
                        ExtensionType::MaxFragmentLength,
                        EncryptedExtensionExtensionDataDispatchCase0::V2(
                            max_fragment_length(),
                        ),
                    ),
                    (
                        ExtensionType::SupportedGroups,
                        EncryptedExtensionExtensionDataDispatchCase0::V3(
                            named_group_list(),
                        ),
                    ),
                    (
                        ExtensionType::Heartbeat,
                        EncryptedExtensionExtensionDataDispatchCase0::V4(
                            heartbeat_mode(),
                        ),
                    ),
                    (
                        ExtensionType::ApplicationLayerProtocolNegotiation,
                        EncryptedExtensionExtensionDataDispatchCase0::V5(
                            protocol_name_list(),
                        ),
                    ),
                    (
                        ExtensionType::ClientCertificateType,
                        EncryptedExtensionExtensionDataDispatchCase0::V6(
                            client_cert_type_client_extension(),
                        ),
                    ),
                    (
                        ExtensionType::ServerCertificateType,
                        EncryptedExtensionExtensionDataDispatchCase0::V7(
                            server_cert_type_client_extension(),
                        ),
                    ),
                    (
                        ExtensionType::EarlyData,
                        EncryptedExtensionExtensionDataDispatchCase0::V8(empty()),
                    ),
                ],
                Some(Variable(ext_len.value() as usize)),
            ),
        ),
    )
}

///Constructor for encrypted_extension combinator
pub fn encrypted_extension() -> EncryptedExtensionCombinator {
    EncryptedExtensionCombinator(
        Mapped::new(
            Pair::new((extension_type(), U16Be), EncryptedExtensionDep {}),
            EncryptedExtensionMapper,
        ),
    )
}

///Constructor for encrypted_extensions combinator
pub fn encrypted_extensions() -> EncryptedExtensionsCombinator {
    EncryptedExtensionsCombinator(
        Mapped::new(
            Pair::new(U16Be, EncryptedExtensionsDep {}),
            EncryptedExtensionsMapper,
        ),
    )
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

///Constructor for certificate_extension_extension_data combinator
pub fn certificate_extension_extension_data<'a, ExtLenArg, ExtensionTypeArg>(
    ext_len: ExtLenArg,
    extension_type: ExtensionTypeArg,
) -> CertificateExtensionExtensionDataCombinator<
    CertificateExtensionExtensionDataCombinatorAlias<'a>,
>
where
    ExtLenArg: LengthParam<'a, u16>,
    ExtensionTypeArg: RuntimeValParam<'a, u16>,
{
    CertificateExtensionExtensionDataCombinator(
        FixedLen(
            ext_len.into_length(),
            Dispatch::new(
                extension_type.into_runtime_value(),
                [
                    (
                        ExtensionType::StatusRequest,
                        CertificateExtensionExtensionDataDispatchCase0::V1(
                            certificate_status(),
                        ),
                    ),
                    (
                        ExtensionType::SignedCertificateTimeStamp,
                        CertificateExtensionExtensionDataDispatchCase0::V2(
                            signed_certificate_timestamp_list(),
                        ),
                    ),
                ],
                Some(Variable(ext_len.value() as usize)),
            ),
        ),
    )
}

///Constructor for certificate_extension combinator
pub fn certificate_extension() -> CertificateExtensionCombinator {
    CertificateExtensionCombinator(
        Mapped::new(
            Pair::new((extension_type(), U16Be), CertificateExtensionDep {}),
            CertificateExtensionMapper,
        ),
    )
}

///Constructor for certificate_extensions combinator
pub fn certificate_extensions() -> CertificateExtensionsCombinator {
    CertificateExtensionsCombinator(
        Mapped::new(
            Pair::new(U16Be, CertificateExtensionsDep {}),
            CertificateExtensionsMapper,
        ),
    )
}

///Constructor for certificate_entry_opaque combinator
pub fn certificate_entry_opaque() -> CertificateEntryOpaqueCombinator {
    CertificateEntryOpaqueCombinator(
        Mapped::new(
            (opaque_1_ffffff(), certificate_extensions()),
            CertificateEntryOpaqueMapper,
        ),
    )
}

///Constructor for certificate_list combinator
pub fn certificate_list() -> CertificateListCombinator {
    CertificateListCombinator(
        Mapped::new(Pair::new(U24Be, CertificateListDep {}), CertificateListMapper),
    )
}

///Constructor for certificate combinator
pub fn certificate() -> CertificateCombinator {
    CertificateCombinator(
        Mapped::new((opaque_0_ff(), certificate_list()), CertificateMapper),
    )
}

///Constructor for certificate_request_extensions combinator
pub fn certificate_request_extensions() -> CertificateRequestExtensionsCombinator {
    CertificateRequestExtensionsCombinator(
        Mapped::new(
            Pair::new(
                Refined {
                    inner: U16Be,
                    predicate: |v: u16| v as i128 >= 2 && v as i128 <= 65535,
                },
                CertificateRequestExtensionsDep {},
            ),
            CertificateRequestExtensionsMapper,
        ),
    )
}

///Constructor for certificate_request combinator
pub fn certificate_request() -> CertificateRequestCombinator {
    CertificateRequestCombinator(
        Mapped::new(
            (opaque_0_ff(), certificate_request_extensions()),
            CertificateRequestMapper,
        ),
    )
}

///Constructor for certificate_verify combinator
pub fn certificate_verify() -> CertificateVerifyCombinator {
    CertificateVerifyCombinator(
        Mapped::new((signature_scheme(), opaque_0_ffff()), CertificateVerifyMapper),
    )
}

///Constructor for finished combinator
pub fn finished<'a, SizeArg>(
    size: SizeArg,
) -> FinishedCombinator<FinishedCombinatorAlias<'a>>
where
    SizeArg: RuntimeValParam<'a, u24>,
{
    FinishedCombinator(
        Dispatch::new(
            size.into_runtime_value(),
            [
                (DigestSize::Hash12, FinishedDispatchCase::V1(Fixed::<12>)),
                (DigestSize::Hash20, FinishedDispatchCase::V2(Fixed::<20>)),
                (DigestSize::Sha256, FinishedDispatchCase::V3(Fixed::<32>)),
                (DigestSize::Sha384, FinishedDispatchCase::V4(Fixed::<48>)),
                (DigestSize::Sha512, FinishedDispatchCase::V5(Fixed::<64>)),
            ],
            Some(Variable(size.value().as_u32() as usize)),
        ),
    )
}

///Constructor for key_update_request combinator
pub fn key_update_request() -> KeyUpdateRequestCombinator {
    KeyUpdateRequestCombinator(
        Mapped::new(
            Refined {
                inner: U8,
                predicate: |v: u8| (v as i128 == 0 || v as i128 == 1),
            },
            KeyUpdateRequestMapper,
        ),
    )
}

///Constructor for key_update combinator
pub fn key_update() -> KeyUpdateCombinator {
    KeyUpdateCombinator(Mapped::new(key_update_request(), KeyUpdateMapper))
}

///Constructor for handshake_msg combinator
pub fn handshake_msg<'a, LengthArg, MsgTypeArg>(
    length: LengthArg,
    msg_type: MsgTypeArg,
) -> HandshakeMsgCombinator<HandshakeMsgCombinatorAlias<'a>>
where
    LengthArg: LengthParam<'a, u24>,
    MsgTypeArg: RuntimeValParam<'a, HandshakeType>,
{
    HandshakeMsgCombinator(
        FixedLen(
            length.into_length(),
            Dispatch::new(
                msg_type.into_runtime_value(),
                [
                    (
                        HandshakeType::ClientHello,
                        HandshakeMsgDispatchCase0::V1(client_hello()),
                    ),
                    (
                        HandshakeType::ServerHello,
                        HandshakeMsgDispatchCase0::V2(sh_or_hrr()),
                    ),
                    (
                        HandshakeType::NewSessionTicket,
                        HandshakeMsgDispatchCase0::V3(new_session_ticket()),
                    ),
                    (
                        HandshakeType::EndOfEarlyData,
                        HandshakeMsgDispatchCase0::V4(empty()),
                    ),
                    (
                        HandshakeType::EncryptedExtensions,
                        HandshakeMsgDispatchCase0::V5(encrypted_extensions()),
                    ),
                    (
                        HandshakeType::Certificate,
                        HandshakeMsgDispatchCase0::V6(certificate()),
                    ),
                    (
                        HandshakeType::CertificateRequest,
                        HandshakeMsgDispatchCase0::V7(certificate_request()),
                    ),
                    (
                        HandshakeType::CertificateVerify,
                        HandshakeMsgDispatchCase0::V8(certificate_verify()),
                    ),
                    (
                        HandshakeType::Finished,
                        HandshakeMsgDispatchCase0::V9(finished(length)),
                    ),
                    (
                        HandshakeType::KeyUpdate,
                        HandshakeMsgDispatchCase0::V10(key_update()),
                    ),
                ],
                None,
            ),
        ),
    )
}

///Constructor for handshake combinator
pub fn handshake() -> HandshakeCombinator {
    HandshakeCombinator(
        Mapped::new(
            Pair::new((handshake_type(), U24Be), HandshakeDep {}),
            HandshakeMapper,
        ),
    )
}

///Constructor for zero_byte combinator
pub fn zero_byte() -> ZeroByteCombinator {
    ZeroByteCombinator(Mapped::new(Tag::new(U8, ZERO_BYTEZERO_CONST), ZeroByteMapper))
}

///Constructor for padding_extension combinator
pub fn padding_extension(ext_len: u16) -> PaddingExtensionCombinator {
    PaddingExtensionCombinator(
        Mapped::new(Pair::new(U16Be, PaddingExtensionDep {}), PaddingExtensionMapper),
    )
}

///Constructor for extension combinator
pub fn extension() -> ExtensionCombinator {
    ExtensionCombinator(
        Mapped::new((extension_type(), opaque_0_ffff()), ExtensionMapper),
    )
}

///Constructor for client_cert_type_server_extension combinator
pub fn client_cert_type_server_extension() -> ClientCertTypeServerExtensionCombinator {
    ClientCertTypeServerExtensionCombinator(
        Mapped::new(certificate_type(), ClientCertTypeServerExtensionMapper),
    )
}

///Constructor for content_type combinator
pub fn content_type() -> ContentTypeCombinator {
    ContentTypeCombinator(
        Mapped::new(
            Refined {
                inner: U8,
                predicate: |v: u8| {
                    (v as i128 == 0 || v as i128 == 20 || v as i128 == 21
                        || v as i128 == 22 || v as i128 == 23)
                },
            },
            ContentTypeMapper,
        ),
    )
}

///Constructor for tls_plaintext combinator
pub fn tls_plaintext() -> TlsPlaintextCombinator {
    TlsPlaintextCombinator(
        Mapped::new(
            (content_type(), (protocol_version(), opaque_0_ffff())),
            TlsPlaintextMapper,
        ),
    )
}

///Constructor for alert_description combinator
pub fn alert_description() -> AlertDescriptionCombinator {
    AlertDescriptionCombinator(
        Mapped::new(
            Refined {
                inner: U8,
                predicate: |v: u8| {
                    (v as i128 == 0 || v as i128 == 10 || v as i128 == 20
                        || v as i128 == 22 || v as i128 == 40 || v as i128 == 42
                        || v as i128 == 43 || v as i128 == 44 || v as i128 == 45
                        || v as i128 == 46 || v as i128 == 47 || v as i128 == 48
                        || v as i128 == 49 || v as i128 == 50 || v as i128 == 51
                        || v as i128 == 70 || v as i128 == 71 || v as i128 == 80
                        || v as i128 == 86 || v as i128 == 90 || v as i128 == 109
                        || v as i128 == 110 || v as i128 == 112 || v as i128 == 113
                        || v as i128 == 115 || v as i128 == 116 || v as i128 == 120)
                },
            },
            AlertDescriptionMapper,
        ),
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

///Constructor for use_srtp_data combinator
pub fn use_srtp_data() -> UseSrtpDataCombinator {
    UseSrtpDataCombinator(
        Mapped::new((srtp_protection_profiles(), opaque_0_ff()), UseSrtpDataMapper),
    )
}

///Constructor for supported_versions_server combinator
pub fn supported_versions_server() -> SupportedVersionsServerCombinator {
    SupportedVersionsServerCombinator(protocol_version())
}

///Constructor for hello_retry_extension_extension_data combinator
pub fn hello_retry_extension_extension_data<'a, ExtLenArg, ExtensionTypeArg>(
    ext_len: ExtLenArg,
    extension_type: ExtensionTypeArg,
) -> HelloRetryExtensionExtensionDataCombinator<
    HelloRetryExtensionExtensionDataCombinatorAlias<'a>,
>
where
    ExtLenArg: LengthParam<'a, u16>,
    ExtensionTypeArg: RuntimeValParam<'a, u16>,
{
    HelloRetryExtensionExtensionDataCombinator(
        FixedLen(
            ext_len.into_length(),
            Dispatch::new(
                extension_type.into_runtime_value(),
                [
                    (
                        ExtensionType::SupportedVersions,
                        HelloRetryExtensionExtensionDataDispatchCase0::V1(
                            supported_versions_server(),
                        ),
                    ),
                    (
                        ExtensionType::Cookie,
                        HelloRetryExtensionExtensionDataDispatchCase0::V2(cookie()),
                    ),
                    (
                        ExtensionType::KeyShare,
                        HelloRetryExtensionExtensionDataDispatchCase0::V3(named_group()),
                    ),
                ],
                Some(Variable(ext_len.value() as usize)),
            ),
        ),
    )
}

///Constructor for finished_opaque combinator
pub fn finished_opaque(digest_size: u24) -> FinishedOpaqueCombinator {
    FinishedOpaqueCombinator(Variable(digest_size.as_u32() as usize))
}

///Constructor for pre_shared_key_server_extension combinator
pub fn pre_shared_key_server_extension() -> PreSharedKeyServerExtensionCombinator {
    PreSharedKeyServerExtensionCombinator(
        Mapped::new(U16Be, PreSharedKeyServerExtensionMapper),
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

///Constructor for hello_retry_extension combinator
pub fn hello_retry_extension() -> HelloRetryExtensionCombinator {
    HelloRetryExtensionCombinator(
        Mapped::new(
            Pair::new((extension_type(), U16Be), HelloRetryExtensionDep {}),
            HelloRetryExtensionMapper,
        ),
    )
}

///Constructor for alert combinator
pub fn alert() -> AlertCombinator {
    AlertCombinator(Mapped::new((alert_level(), alert_description()), AlertMapper))
}

///Constructor for server_cert_type_server_extension combinator
pub fn server_cert_type_server_extension() -> ServerCertTypeServerExtensionCombinator {
    ServerCertTypeServerExtensionCombinator(
        Mapped::new(certificate_type(), ServerCertTypeServerExtensionMapper),
    )
}

///Constructor for unknown_extension combinator
pub fn unknown_extension() -> UnknownExtensionCombinator {
    UnknownExtensionCombinator(opaque_0_ffff())
}

///Constructor for hello_retry_extensions combinator
pub fn hello_retry_extensions() -> HelloRetryExtensionsCombinator {
    HelloRetryExtensionsCombinator(
        Mapped::new(
            Pair::new(
                Refined {
                    inner: U16Be,
                    predicate: |v: u16| v as i128 >= 6 && v as i128 <= 65535,
                },
                HelloRetryExtensionsDep {},
            ),
            HelloRetryExtensionsMapper,
        ),
    )
}

///Constructor for sever_hello_extension_extension_data combinator
pub fn sever_hello_extension_extension_data<'a, ExtLenArg, ExtensionTypeArg>(
    ext_len: ExtLenArg,
    extension_type: ExtensionTypeArg,
) -> SeverHelloExtensionExtensionDataCombinator<
    SeverHelloExtensionExtensionDataCombinatorAlias<'a>,
>
where
    ExtLenArg: LengthParam<'a, u16>,
    ExtensionTypeArg: RuntimeValParam<'a, u16>,
{
    SeverHelloExtensionExtensionDataCombinator(
        FixedLen(
            ext_len.into_length(),
            Dispatch::new(
                extension_type.into_runtime_value(),
                [
                    (
                        ExtensionType::PreSharedKey,
                        SeverHelloExtensionExtensionDataDispatchCase0::V1(
                            pre_shared_key_server_extension(),
                        ),
                    ),
                    (
                        ExtensionType::SupportedVersions,
                        SeverHelloExtensionExtensionDataDispatchCase0::V2(
                            supported_versions_server(),
                        ),
                    ),
                    (
                        ExtensionType::KeyShare,
                        SeverHelloExtensionExtensionDataDispatchCase0::V3(
                            key_share_entry(),
                        ),
                    ),
                ],
                Some(Variable(ext_len.value() as usize)),
            ),
        ),
    )
}

///Constructor for sever_hello_extension combinator
pub fn sever_hello_extension() -> SeverHelloExtensionCombinator {
    SeverHelloExtensionCombinator(
        Mapped::new(
            Pair::new((extension_type(), U16Be), SeverHelloExtensionDep {}),
            SeverHelloExtensionMapper,
        ),
    )
}

///Constructor for server_extensions combinator
pub fn server_extensions() -> ServerExtensionsCombinator {
    ServerExtensionsCombinator(
        Mapped::new(
            Pair::new(
                Refined {
                    inner: U16Be,
                    predicate: |v: u16| v as i128 >= 6 && v as i128 <= 65535,
                },
                ServerExtensionsDep {},
            ),
            ServerExtensionsMapper,
        ),
    )
}

///Constructor for digest_size combinator
pub fn digest_size() -> DigestSizeCombinator {
    DigestSizeCombinator(U24Be)
}

///Constructor for heartbeat_extension combinator
pub fn heartbeat_extension() -> HeartbeatExtensionCombinator {
    HeartbeatExtensionCombinator(Mapped::new(heartbeat_mode(), HeartbeatExtensionMapper))
}

///Constructor for ec_point_format combinator
pub fn ec_point_format() -> EcPointFormatCombinator {
    EcPointFormatCombinator(U8)
}

///Constructor for server_hello combinator
pub fn server_hello() -> ServerHelloCombinator {
    ServerHelloCombinator(
        Mapped::new(
            (
                session_id(),
                (
                    cipher_suite(),
                    (
                        Tag::new(U8, SERVER_HELLOLEGACY_COMPRESSION_METHOD_CONST),
                        server_extensions(),
                    ),
                ),
            ),
            ServerHelloMapper,
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

///Constructor for hello_retry_request combinator
pub fn hello_retry_request() -> HelloRetryRequestCombinator {
    HelloRetryRequestCombinator(
        Mapped::new(
            (
                session_id(),
                (
                    cipher_suite(),
                    (
                        Tag::new(U8, HELLO_RETRY_REQUESTLEGACY_COMPRESSION_METHOD_CONST),
                        hello_retry_extensions(),
                    ),
                ),
            ),
            HelloRetryRequestMapper,
        ),
    )
}

///Constructor for certificate_entry_data combinator
pub fn certificate_entry_data<'a, CertTypeArg>(
    cert_type: CertTypeArg,
) -> CertificateEntryDataCombinator<CertificateEntryDataCombinatorAlias<'a>>
where
    CertTypeArg: RuntimeValParam<'a, u8>,
{
    CertificateEntryDataCombinator(
        Dispatch::new(
            cert_type.into_runtime_value(),
            [
                (
                    CertificateType::X509,
                    CertificateEntryDataDispatchCase::V1(opaque_1_ffffff()),
                ),
                (
                    CertificateType::RawPublicKey,
                    CertificateEntryDataDispatchCase::V2(opaque_1_ffffff()),
                ),
            ],
            None,
        ),
    )
}

///Constructor for certificate_entry combinator
pub fn certificate_entry<'a, CertTypeArg>(
    cert_type: CertTypeArg,
) -> CertificateEntryCombinator<CertificateEntryCombinatorAlias<'a>>
where
    CertTypeArg: RuntimeValParam<'a, u8>,
{
    CertificateEntryCombinator(
        Mapped::new(
            (certificate_entry_data(cert_type), certificate_extensions()),
            CertificateEntryMapper,
        ),
    )
}

///Constructor for tls_ciphertext combinator
pub fn tls_ciphertext() -> TlsCiphertextCombinator {
    TlsCiphertextCombinator(
        Mapped::new(
            (content_type(), (protocol_version(), opaque_0_ffff())),
            TlsCiphertextMapper,
        ),
    )
}

///Constructor for opaque_0_ffffff combinator
pub fn opaque_0_ffffff() -> Opaque0FfffffCombinator {
    Opaque0FfffffCombinator(
        Mapped::new(Pair::new(U24Be, Opaque0FfffffDep {}), Opaque0FfffffMapper),
    )
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

#[derive(Clone, Copy)]
pub struct CertificateAuthoritiesExtensionDep {}
impl DepCombinator<Refined<U16Be, fn(u16) -> bool>, [u8], Vec<u8>>
for CertificateAuthoritiesExtensionDep {
    type Out = FixedLen<'static, Repeat<DistinguishedNameCombinator>>;
    type OutGen<'g> = FixedLen<'g, Repeat<DistinguishedNameCombinator>>;
    fn dep_snd<'s>(&self, fst: u16) -> Self::Out {
        let fst: u16 = fst;
        let l = fst;
        FixedLen(Length::from_value(l as usize), Repeat::new(distinguished_name()))
    }
    fn dep_snd_gen<'g>(&self, fst: &'g mut u16) -> Self::OutGen<'g> {
        let fst: &'g mut u16 = fst;
        let l = fst;
        FixedLen(Length::from_u16_mut(l), Repeat::new(distinguished_name()))
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
pub struct OidFilterExtensionDep {}
impl DepCombinator<U16Be, [u8], Vec<u8>> for OidFilterExtensionDep {
    type Out = FixedLen<'static, Repeat<OidFilterCombinator>>;
    type OutGen<'g> = FixedLen<'g, Repeat<OidFilterCombinator>>;
    fn dep_snd<'s>(&self, fst: u16) -> Self::Out {
        let fst: u16 = fst;
        let l = fst;
        FixedLen(Length::from_value(l as usize), Repeat::new(oid_filter()))
    }
    fn dep_snd_gen<'g>(&self, fst: &'g mut u16) -> Self::OutGen<'g> {
        let fst: &'g mut u16 = fst;
        let l = fst;
        FixedLen(Length::from_u16_mut(l), Repeat::new(oid_filter()))
    }
}

pub enum CertificateRequestExtensionExtensionDataDispatchCase0<
    C0 = SignatureSchemeListCombinator,
    C1 = CertificateAuthoritiesExtensionCombinator,
    C2 = SignatureSchemeListCombinator,
    C3 = CertificateStatusRequestCombinator,
    C4 = SignedCertificateTimestampListCombinator,
    C5 = OidFilterExtensionCombinator,
> {
    V1(C0),
    V2(C1),
    V3(C2),
    V4(C3),
    V5(C4),
    V6(C5),
}

impl<C0, C1, C2, C3, C4, C5> Combinator<[u8], Vec<u8>>
for CertificateRequestExtensionExtensionDataDispatchCase0<C0, C1, C2, C3, C4, C5>
where
    for<'p, 's> C0: Combinator<
        [u8],
        Vec<u8>,
        Type<'p> = SignatureSchemeList,
        SType<'s> = &'s SignatureSchemeList,
        GType = SignatureSchemeList,
    >,
    for<'p, 's> C1: Combinator<
        [u8],
        Vec<u8>,
        Type<'p> = CertificateAuthoritiesExtension<'p>,
        SType<'s> = &'s CertificateAuthoritiesExtension<'s>,
        GType = CertificateAuthoritiesExtensionOwned,
    >,
    for<'p, 's> C2: Combinator<
        [u8],
        Vec<u8>,
        Type<'p> = SignatureSchemeList,
        SType<'s> = &'s SignatureSchemeList,
        GType = SignatureSchemeList,
    >,
    for<'p, 's> C3: Combinator<
        [u8],
        Vec<u8>,
        Type<'p> = CertificateStatusRequest<'p>,
        SType<'s> = &'s CertificateStatusRequest<'s>,
        GType = CertificateStatusRequestOwned,
    >,
    for<'p, 's> C4: Combinator<
        [u8],
        Vec<u8>,
        Type<'p> = SignedCertificateTimestampList<'p>,
        SType<'s> = &'s SignedCertificateTimestampList<'s>,
        GType = SignedCertificateTimestampListOwned,
    >,
    for<'p, 's> C5: Combinator<
        [u8],
        Vec<u8>,
        Type<'p> = OidFilterExtension<'p>,
        SType<'s> = &'s OidFilterExtension<'s>,
        GType = OidFilterExtensionOwned,
    >,
{
    type Type<'p> = CertificateRequestExtensionExtensionData<'p>;
    type SType<'s> = &'s CertificateRequestExtensionExtensionData<'s>;
    type GType = CertificateRequestExtensionExtensionDataOwned;
    fn length<'s>(&self, v: Self::SType<'s>) -> usize
    where
        [u8]: 's,
    {
        match (self, v) {
            (
                CertificateRequestExtensionExtensionDataDispatchCase0::V1(inner),
                CertificateRequestExtensionExtensionData::SignatureAlgorithms(v0),
            ) => inner.length(&(*v0)),
            (
                CertificateRequestExtensionExtensionDataDispatchCase0::V2(inner),
                CertificateRequestExtensionExtensionData::CertificateAuthorities(v1),
            ) => inner.length(&(*v1)),
            (
                CertificateRequestExtensionExtensionDataDispatchCase0::V3(inner),
                CertificateRequestExtensionExtensionData::SignatureAlgorithmsCert(v2),
            ) => inner.length(&(*v2)),
            (
                CertificateRequestExtensionExtensionDataDispatchCase0::V4(inner),
                CertificateRequestExtensionExtensionData::StatusRequest(v3),
            ) => inner.length(&(*v3)),
            (
                CertificateRequestExtensionExtensionDataDispatchCase0::V5(inner),
                CertificateRequestExtensionExtensionData::SignedCertificateTimeStamp(v4),
            ) => inner.length(&(*v4)),
            (
                CertificateRequestExtensionExtensionDataDispatchCase0::V6(inner),
                CertificateRequestExtensionExtensionData::OidFilters(v5),
            ) => inner.length(&(*v5)),
            _ => panic!("dispatch branch combinator does not match value"),
        }
    }
    fn parse<'p>(&self, s: &'p [u8]) -> Result<(usize, Self::Type<'p>), ParseError>
    where
        [u8]: 'p,
    {
        match self {
            CertificateRequestExtensionExtensionDataDispatchCase0::V1(inner) => {
                let (n, v) = inner.parse(s)?;
                Ok((n, CertificateRequestExtensionExtensionData::SignatureAlgorithms(v)))
            }
            CertificateRequestExtensionExtensionDataDispatchCase0::V2(inner) => {
                let (n, v) = inner.parse(s)?;
                Ok((
                    n,
                    CertificateRequestExtensionExtensionData::CertificateAuthorities(v),
                ))
            }
            CertificateRequestExtensionExtensionDataDispatchCase0::V3(inner) => {
                let (n, v) = inner.parse(s)?;
                Ok((
                    n,
                    CertificateRequestExtensionExtensionData::SignatureAlgorithmsCert(v),
                ))
            }
            CertificateRequestExtensionExtensionDataDispatchCase0::V4(inner) => {
                let (n, v) = inner.parse(s)?;
                Ok((n, CertificateRequestExtensionExtensionData::StatusRequest(v)))
            }
            CertificateRequestExtensionExtensionDataDispatchCase0::V5(inner) => {
                let (n, v) = inner.parse(s)?;
                Ok((
                    n,
                    CertificateRequestExtensionExtensionData::SignedCertificateTimeStamp(
                        v,
                    ),
                ))
            }
            CertificateRequestExtensionExtensionDataDispatchCase0::V6(inner) => {
                let (n, v) = inner.parse(s)?;
                Ok((n, CertificateRequestExtensionExtensionData::OidFilters(v)))
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
            (
                CertificateRequestExtensionExtensionDataDispatchCase0::V1(inner),
                CertificateRequestExtensionExtensionData::SignatureAlgorithms(v0),
            ) => inner.serialize(&(*v0), data, pos),
            (
                CertificateRequestExtensionExtensionDataDispatchCase0::V2(inner),
                CertificateRequestExtensionExtensionData::CertificateAuthorities(v1),
            ) => inner.serialize(&(*v1), data, pos),
            (
                CertificateRequestExtensionExtensionDataDispatchCase0::V3(inner),
                CertificateRequestExtensionExtensionData::SignatureAlgorithmsCert(v2),
            ) => inner.serialize(&(*v2), data, pos),
            (
                CertificateRequestExtensionExtensionDataDispatchCase0::V4(inner),
                CertificateRequestExtensionExtensionData::StatusRequest(v3),
            ) => inner.serialize(&(*v3), data, pos),
            (
                CertificateRequestExtensionExtensionDataDispatchCase0::V5(inner),
                CertificateRequestExtensionExtensionData::SignedCertificateTimeStamp(v4),
            ) => inner.serialize(&(*v4), data, pos),
            (
                CertificateRequestExtensionExtensionDataDispatchCase0::V6(inner),
                CertificateRequestExtensionExtensionData::OidFilters(v5),
            ) => inner.serialize(&(*v5), data, pos),
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
                CertificateRequestExtensionExtensionDataDispatchCase0::V1(inner),
                CertificateRequestExtensionExtensionDataOwned::SignatureAlgorithms(v0),
            ) => inner.serialize_gen(v0, data, pos),
            (
                CertificateRequestExtensionExtensionDataDispatchCase0::V2(inner),
                CertificateRequestExtensionExtensionDataOwned::CertificateAuthorities(v1),
            ) => inner.serialize_gen(v1, data, pos),
            (
                CertificateRequestExtensionExtensionDataDispatchCase0::V3(inner),
                CertificateRequestExtensionExtensionDataOwned::SignatureAlgorithmsCert(
                    v2,
                ),
            ) => inner.serialize_gen(v2, data, pos),
            (
                CertificateRequestExtensionExtensionDataDispatchCase0::V4(inner),
                CertificateRequestExtensionExtensionDataOwned::StatusRequest(v3),
            ) => inner.serialize_gen(v3, data, pos),
            (
                CertificateRequestExtensionExtensionDataDispatchCase0::V5(inner),
                CertificateRequestExtensionExtensionDataOwned::SignedCertificateTimeStamp(
                    v4,
                ),
            ) => inner.serialize_gen(v4, data, pos),
            (
                CertificateRequestExtensionExtensionDataDispatchCase0::V6(inner),
                CertificateRequestExtensionExtensionDataOwned::OidFilters(v5),
            ) => inner.serialize_gen(v5, data, pos),
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
            CertificateRequestExtensionExtensionDataDispatchCase0::V1(inner) => {
                let (n, v) = inner.generate(g)?;
                Ok((
                    n,
                    CertificateRequestExtensionExtensionDataOwned::SignatureAlgorithms(v),
                ))
            }
            CertificateRequestExtensionExtensionDataDispatchCase0::V2(inner) => {
                let (n, v) = inner.generate(g)?;
                Ok((
                    n,
                    CertificateRequestExtensionExtensionDataOwned::CertificateAuthorities(
                        v,
                    ),
                ))
            }
            CertificateRequestExtensionExtensionDataDispatchCase0::V3(inner) => {
                let (n, v) = inner.generate(g)?;
                Ok((
                    n,
                    CertificateRequestExtensionExtensionDataOwned::SignatureAlgorithmsCert(
                        v,
                    ),
                ))
            }
            CertificateRequestExtensionExtensionDataDispatchCase0::V4(inner) => {
                let (n, v) = inner.generate(g)?;
                Ok((n, CertificateRequestExtensionExtensionDataOwned::StatusRequest(v)))
            }
            CertificateRequestExtensionExtensionDataDispatchCase0::V5(inner) => {
                let (n, v) = inner.generate(g)?;
                Ok((
                    n,
                    CertificateRequestExtensionExtensionDataOwned::SignedCertificateTimeStamp(
                        v,
                    ),
                ))
            }
            CertificateRequestExtensionExtensionDataDispatchCase0::V6(inner) => {
                let (n, v) = inner.generate(g)?;
                Ok((n, CertificateRequestExtensionExtensionDataOwned::OidFilters(v)))
            }
        }
    }
    fn well_formed<'s>(&self, v: Self::SType<'s>) -> bool
    where
        [u8]: 's,
    {
        match (self, v) {
            (
                CertificateRequestExtensionExtensionDataDispatchCase0::V1(inner),
                CertificateRequestExtensionExtensionData::SignatureAlgorithms(v0),
            ) => inner.well_formed(&(*v0)),
            (
                CertificateRequestExtensionExtensionDataDispatchCase0::V2(inner),
                CertificateRequestExtensionExtensionData::CertificateAuthorities(v1),
            ) => inner.well_formed(&(*v1)),
            (
                CertificateRequestExtensionExtensionDataDispatchCase0::V3(inner),
                CertificateRequestExtensionExtensionData::SignatureAlgorithmsCert(v2),
            ) => inner.well_formed(&(*v2)),
            (
                CertificateRequestExtensionExtensionDataDispatchCase0::V4(inner),
                CertificateRequestExtensionExtensionData::StatusRequest(v3),
            ) => inner.well_formed(&(*v3)),
            (
                CertificateRequestExtensionExtensionDataDispatchCase0::V5(inner),
                CertificateRequestExtensionExtensionData::SignedCertificateTimeStamp(v4),
            ) => inner.well_formed(&(*v4)),
            (
                CertificateRequestExtensionExtensionDataDispatchCase0::V6(inner),
                CertificateRequestExtensionExtensionData::OidFilters(v5),
            ) => inner.well_formed(&(*v5)),
            _ => false,
        }
    }
}

#[derive(Clone, Copy)]
pub struct CertificateRequestExtensionDep {}
impl DepCombinator<(ExtensionTypeCombinator, U16Be), [u8], Vec<u8>>
for CertificateRequestExtensionDep {
    type Out = CertificateRequestExtensionExtensionDataCombinator;
    type OutGen<'g> = CertificateRequestExtensionExtensionDataCombinator<
        CertificateRequestExtensionExtensionDataCombinatorAlias<'g>,
    >;
    fn dep_snd<'s>(&self, fst: (ExtensionType, u16)) -> Self::Out {
        let fst: (ExtensionType, u16) = fst;
        let (extension_type, ext_len) = fst;
        certificate_request_extension_extension_data(ext_len, extension_type)
    }
    fn dep_snd_gen<'g>(&self, fst: &'g mut (ExtensionType, u16)) -> Self::OutGen<'g> {
        let fst: &'g mut (ExtensionType, u16) = fst;
        let (extension_type, ext_len) = fst;
        certificate_request_extension_extension_data(ext_len, extension_type)
    }
}

#[derive(Clone, Copy)]
pub struct ShOrHrrDep1 {}
impl DepCombinator<Fixed<32>, [u8], Vec<u8>> for ShOrHrrDep1 {
    type Out = Tail;
    type OutGen<'g> = Tail;
    fn dep_snd<'s>(&self, fst: &'s [u8]) -> Self::Out {
        let fst: &'s [u8] = fst;
        let random = fst;
        Tail
    }
    fn dep_snd_gen<'g>(&self, fst: &'g mut Vec<u8>) -> Self::OutGen<'g> {
        let fst: &'g mut Vec<u8> = fst;
        let random = fst;
        Tail
    }
}

#[derive(Clone, Copy)]
pub struct SessionIdDep {}
impl DepCombinator<Refined<U8, fn(u8) -> bool>, [u8], Vec<u8>> for SessionIdDep {
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
pub struct CipherSuiteListDep {}
impl DepCombinator<Refined<U16Be, fn(u16) -> bool>, [u8], Vec<u8>>
for CipherSuiteListDep {
    type Out = FixedLen<'static, Repeat<CipherSuiteCombinator>>;
    type OutGen<'g> = FixedLen<'g, Repeat<CipherSuiteCombinator>>;
    fn dep_snd<'s>(&self, fst: u16) -> Self::Out {
        let fst: u16 = fst;
        let l = fst;
        FixedLen(Length::from_value(l as usize), Repeat::new(cipher_suite()))
    }
    fn dep_snd_gen<'g>(&self, fst: &'g mut u16) -> Self::OutGen<'g> {
        let fst: &'g mut u16 = fst;
        let l = fst;
        FixedLen(Length::from_u16_mut(l), Repeat::new(cipher_suite()))
    }
}

pub enum ServerNameNameDispatchCase<C0 = HostNameCombinator> {
    V1(C0),
}

impl<C0> Combinator<[u8], Vec<u8>> for ServerNameNameDispatchCase<C0>
where
    for<'p, 's> C0: Combinator<
        [u8],
        Vec<u8>,
        Type<'p> = HostName<'p>,
        SType<'s> = &'s HostName<'s>,
        GType = HostNameOwned,
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

#[derive(Clone, Copy)]
pub struct SupportedVersionsClientDep {}
impl DepCombinator<Refined<U8, fn(u8) -> bool>, [u8], Vec<u8>>
for SupportedVersionsClientDep {
    type Out = FixedLen<'static, Repeat<ProtocolVersionCombinator>>;
    type OutGen<'g> = FixedLen<'g, Repeat<ProtocolVersionCombinator>>;
    fn dep_snd<'s>(&self, fst: u8) -> Self::Out {
        let fst: u8 = fst;
        let l = fst;
        FixedLen(Length::from_value(l as usize), Repeat::new(protocol_version()))
    }
    fn dep_snd_gen<'g>(&self, fst: &'g mut u8) -> Self::OutGen<'g> {
        let fst: &'g mut u8 = fst;
        let l = fst;
        FixedLen(Length::from_u8_mut(l), Repeat::new(protocol_version()))
    }
}

#[derive(Clone, Copy)]
pub struct KeyShareEntryDep {}
impl DepCombinator<
    (NamedGroupCombinator, Refined<U16Be, fn(u16) -> bool>),
    [u8],
    Vec<u8>,
> for KeyShareEntryDep {
    type Out = Variable;
    type OutGen<'g> = Variable;
    fn dep_snd<'s>(&self, fst: (NamedGroup, u16)) -> Self::Out {
        let fst: (NamedGroup, u16) = fst;
        let (group, l) = fst;
        Variable(l as usize)
    }
    fn dep_snd_gen<'g>(&self, fst: &'g mut (NamedGroup, u16)) -> Self::OutGen<'g> {
        let fst: &'g mut (NamedGroup, u16) = fst;
        let (group, l) = fst;
        Variable(*l as usize)
    }
}

#[derive(Clone, Copy)]
pub struct KeyShareClientHelloDep {}
impl DepCombinator<U16Be, [u8], Vec<u8>> for KeyShareClientHelloDep {
    type Out = FixedLen<'static, Repeat<KeyShareEntryCombinator>>;
    type OutGen<'g> = FixedLen<'g, Repeat<KeyShareEntryCombinator>>;
    fn dep_snd<'s>(&self, fst: u16) -> Self::Out {
        let fst: u16 = fst;
        let l = fst;
        FixedLen(Length::from_value(l as usize), Repeat::new(key_share_entry()))
    }
    fn dep_snd_gen<'g>(&self, fst: &'g mut u16) -> Self::OutGen<'g> {
        let fst: &'g mut u16 = fst;
        let l = fst;
        FixedLen(Length::from_u16_mut(l), Repeat::new(key_share_entry()))
    }
}

#[derive(Clone, Copy)]
pub struct PskKeyExchangeModesDep {}
impl DepCombinator<Refined<U8, fn(u8) -> bool>, [u8], Vec<u8>>
for PskKeyExchangeModesDep {
    type Out = FixedLen<'static, Repeat<PskKeyExchangeModeCombinator>>;
    type OutGen<'g> = FixedLen<'g, Repeat<PskKeyExchangeModeCombinator>>;
    fn dep_snd<'s>(&self, fst: u8) -> Self::Out {
        let fst: u8 = fst;
        let l = fst;
        FixedLen(Length::from_value(l as usize), Repeat::new(psk_key_exchange_mode()))
    }
    fn dep_snd_gen<'g>(&self, fst: &'g mut u8) -> Self::OutGen<'g> {
        let fst: &'g mut u8 = fst;
        let l = fst;
        FixedLen(Length::from_u8_mut(l), Repeat::new(psk_key_exchange_mode()))
    }
}

#[derive(Clone, Copy)]
pub struct PskIdentitiesDep {}
impl DepCombinator<Refined<U16Be, fn(u16) -> bool>, [u8], Vec<u8>> for PskIdentitiesDep {
    type Out = FixedLen<'static, Repeat<PskIdentityCombinator>>;
    type OutGen<'g> = FixedLen<'g, Repeat<PskIdentityCombinator>>;
    fn dep_snd<'s>(&self, fst: u16) -> Self::Out {
        let fst: u16 = fst;
        let l = fst;
        FixedLen(Length::from_value(l as usize), Repeat::new(psk_identity()))
    }
    fn dep_snd_gen<'g>(&self, fst: &'g mut u16) -> Self::OutGen<'g> {
        let fst: &'g mut u16 = fst;
        let l = fst;
        FixedLen(Length::from_u16_mut(l), Repeat::new(psk_identity()))
    }
}

#[derive(Clone, Copy)]
pub struct PskBinderEntryDep {}
impl DepCombinator<Refined<U8, fn(u8) -> bool>, [u8], Vec<u8>> for PskBinderEntryDep {
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
pub struct PskBinderEntriesDep {}
impl DepCombinator<Refined<U16Be, fn(u16) -> bool>, [u8], Vec<u8>>
for PskBinderEntriesDep {
    type Out = FixedLen<'static, Repeat<PskBinderEntryCombinator>>;
    type OutGen<'g> = FixedLen<'g, Repeat<PskBinderEntryCombinator>>;
    fn dep_snd<'s>(&self, fst: u16) -> Self::Out {
        let fst: u16 = fst;
        let l = fst;
        FixedLen(Length::from_value(l as usize), Repeat::new(psk_binder_entry()))
    }
    fn dep_snd_gen<'g>(&self, fst: &'g mut u16) -> Self::OutGen<'g> {
        let fst: &'g mut u16 = fst;
        let l = fst;
        FixedLen(Length::from_u16_mut(l), Repeat::new(psk_binder_entry()))
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

pub enum ClientHelloExtensionRestDispatchCase<
    C0 = MaxFragmentLengthCombinator,
    C1 = HeartbeatModeCombinator,
    C2 = SignedCertificateTimestampListCombinator,
    C3 = ClientCertTypeClientExtensionCombinator,
    C4 = ServerCertTypeClientExtensionCombinator,
    C5 = Variable,
    C6 = CookieCombinator,
    C7 = CertificateAuthoritiesExtensionCombinator,
    C8 = OidFilterExtensionCombinator,
    C9 = SignatureSchemeListCombinator,
> {
    V1(C0),
    V2(C1),
    V3(C2),
    V4(C3),
    V5(C4),
    V6(C5),
    V7(C6),
    V8(C7),
    V9(C8),
    V10(C9),
}

impl<C0, C1, C2, C3, C4, C5, C6, C7, C8, C9> Combinator<[u8], Vec<u8>>
for ClientHelloExtensionRestDispatchCase<C0, C1, C2, C3, C4, C5, C6, C7, C8, C9>
where
    for<'p, 's> C0: Combinator<
        [u8],
        Vec<u8>,
        Type<'p> = MaxFragmentLength,
        SType<'s> = MaxFragmentLength,
        GType = MaxFragmentLength,
    >,
    for<'p, 's> C1: Combinator<
        [u8],
        Vec<u8>,
        Type<'p> = HeartbeatMode,
        SType<'s> = HeartbeatMode,
        GType = HeartbeatMode,
    >,
    for<'p, 's> C2: Combinator<
        [u8],
        Vec<u8>,
        Type<'p> = SignedCertificateTimestampList<'p>,
        SType<'s> = &'s SignedCertificateTimestampList<'s>,
        GType = SignedCertificateTimestampListOwned,
    >,
    for<'p, 's> C3: Combinator<
        [u8],
        Vec<u8>,
        Type<'p> = ClientCertTypeClientExtension,
        SType<'s> = &'s ClientCertTypeClientExtension,
        GType = ClientCertTypeClientExtension,
    >,
    for<'p, 's> C4: Combinator<
        [u8],
        Vec<u8>,
        Type<'p> = ServerCertTypeClientExtension,
        SType<'s> = &'s ServerCertTypeClientExtension,
        GType = ServerCertTypeClientExtension,
    >,
    for<'p, 's> C5: Combinator<
        [u8],
        Vec<u8>,
        Type<'p> = &'p [u8],
        SType<'s> = &'s [u8],
        GType = Vec<u8>,
    >,
    for<'p, 's> C6: Combinator<
        [u8],
        Vec<u8>,
        Type<'p> = Cookie<'p>,
        SType<'s> = &'s Cookie<'s>,
        GType = CookieOwned,
    >,
    for<'p, 's> C7: Combinator<
        [u8],
        Vec<u8>,
        Type<'p> = CertificateAuthoritiesExtension<'p>,
        SType<'s> = &'s CertificateAuthoritiesExtension<'s>,
        GType = CertificateAuthoritiesExtensionOwned,
    >,
    for<'p, 's> C8: Combinator<
        [u8],
        Vec<u8>,
        Type<'p> = OidFilterExtension<'p>,
        SType<'s> = &'s OidFilterExtension<'s>,
        GType = OidFilterExtensionOwned,
    >,
    for<'p, 's> C9: Combinator<
        [u8],
        Vec<u8>,
        Type<'p> = SignatureSchemeList,
        SType<'s> = &'s SignatureSchemeList,
        GType = SignatureSchemeList,
    >,
{
    type Type<'p> = ClientHelloExtensionRest<'p>;
    type SType<'s> = &'s ClientHelloExtensionRest<'s>;
    type GType = ClientHelloExtensionRestOwned;
    fn length<'s>(&self, v: Self::SType<'s>) -> usize
    where
        [u8]: 's,
    {
        match (self, v) {
            (
                ClientHelloExtensionRestDispatchCase::V1(inner),
                ClientHelloExtensionRest::MaxFragmentLength(v0),
            ) => inner.length((*v0)),
            (
                ClientHelloExtensionRestDispatchCase::V2(inner),
                ClientHelloExtensionRest::Heartbeat(v1),
            ) => inner.length((*v1)),
            (
                ClientHelloExtensionRestDispatchCase::V3(inner),
                ClientHelloExtensionRest::SignedCertificateTimeStamp(v2),
            ) => inner.length(&(*v2)),
            (
                ClientHelloExtensionRestDispatchCase::V4(inner),
                ClientHelloExtensionRest::ClientCertificateType(v3),
            ) => inner.length(&(*v3)),
            (
                ClientHelloExtensionRestDispatchCase::V5(inner),
                ClientHelloExtensionRest::ServerCertificateType(v4),
            ) => inner.length(&(*v4)),
            (
                ClientHelloExtensionRestDispatchCase::V6(inner),
                ClientHelloExtensionRest::Padding(v5),
            ) => inner.length((*v5)),
            (
                ClientHelloExtensionRestDispatchCase::V7(inner),
                ClientHelloExtensionRest::Cookie(v6),
            ) => inner.length(&(*v6)),
            (
                ClientHelloExtensionRestDispatchCase::V8(inner),
                ClientHelloExtensionRest::CertificateAuthorities(v7),
            ) => inner.length(&(*v7)),
            (
                ClientHelloExtensionRestDispatchCase::V9(inner),
                ClientHelloExtensionRest::OidFilters(v8),
            ) => inner.length(&(*v8)),
            (
                ClientHelloExtensionRestDispatchCase::V10(inner),
                ClientHelloExtensionRest::SignatureAlgorithmsCert(v9),
            ) => inner.length(&(*v9)),
            _ => panic!("dispatch branch combinator does not match value"),
        }
    }
    fn parse<'p>(&self, s: &'p [u8]) -> Result<(usize, Self::Type<'p>), ParseError>
    where
        [u8]: 'p,
    {
        match self {
            ClientHelloExtensionRestDispatchCase::V1(inner) => {
                let (n, v) = inner.parse(s)?;
                Ok((n, ClientHelloExtensionRest::MaxFragmentLength(v)))
            }
            ClientHelloExtensionRestDispatchCase::V2(inner) => {
                let (n, v) = inner.parse(s)?;
                Ok((n, ClientHelloExtensionRest::Heartbeat(v)))
            }
            ClientHelloExtensionRestDispatchCase::V3(inner) => {
                let (n, v) = inner.parse(s)?;
                Ok((n, ClientHelloExtensionRest::SignedCertificateTimeStamp(v)))
            }
            ClientHelloExtensionRestDispatchCase::V4(inner) => {
                let (n, v) = inner.parse(s)?;
                Ok((n, ClientHelloExtensionRest::ClientCertificateType(v)))
            }
            ClientHelloExtensionRestDispatchCase::V5(inner) => {
                let (n, v) = inner.parse(s)?;
                Ok((n, ClientHelloExtensionRest::ServerCertificateType(v)))
            }
            ClientHelloExtensionRestDispatchCase::V6(inner) => {
                let (n, v) = inner.parse(s)?;
                Ok((n, ClientHelloExtensionRest::Padding(v)))
            }
            ClientHelloExtensionRestDispatchCase::V7(inner) => {
                let (n, v) = inner.parse(s)?;
                Ok((n, ClientHelloExtensionRest::Cookie(v)))
            }
            ClientHelloExtensionRestDispatchCase::V8(inner) => {
                let (n, v) = inner.parse(s)?;
                Ok((n, ClientHelloExtensionRest::CertificateAuthorities(v)))
            }
            ClientHelloExtensionRestDispatchCase::V9(inner) => {
                let (n, v) = inner.parse(s)?;
                Ok((n, ClientHelloExtensionRest::OidFilters(v)))
            }
            ClientHelloExtensionRestDispatchCase::V10(inner) => {
                let (n, v) = inner.parse(s)?;
                Ok((n, ClientHelloExtensionRest::SignatureAlgorithmsCert(v)))
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
            (
                ClientHelloExtensionRestDispatchCase::V1(inner),
                ClientHelloExtensionRest::MaxFragmentLength(v0),
            ) => inner.serialize((*v0), data, pos),
            (
                ClientHelloExtensionRestDispatchCase::V2(inner),
                ClientHelloExtensionRest::Heartbeat(v1),
            ) => inner.serialize((*v1), data, pos),
            (
                ClientHelloExtensionRestDispatchCase::V3(inner),
                ClientHelloExtensionRest::SignedCertificateTimeStamp(v2),
            ) => inner.serialize(&(*v2), data, pos),
            (
                ClientHelloExtensionRestDispatchCase::V4(inner),
                ClientHelloExtensionRest::ClientCertificateType(v3),
            ) => inner.serialize(&(*v3), data, pos),
            (
                ClientHelloExtensionRestDispatchCase::V5(inner),
                ClientHelloExtensionRest::ServerCertificateType(v4),
            ) => inner.serialize(&(*v4), data, pos),
            (
                ClientHelloExtensionRestDispatchCase::V6(inner),
                ClientHelloExtensionRest::Padding(v5),
            ) => inner.serialize((*v5), data, pos),
            (
                ClientHelloExtensionRestDispatchCase::V7(inner),
                ClientHelloExtensionRest::Cookie(v6),
            ) => inner.serialize(&(*v6), data, pos),
            (
                ClientHelloExtensionRestDispatchCase::V8(inner),
                ClientHelloExtensionRest::CertificateAuthorities(v7),
            ) => inner.serialize(&(*v7), data, pos),
            (
                ClientHelloExtensionRestDispatchCase::V9(inner),
                ClientHelloExtensionRest::OidFilters(v8),
            ) => inner.serialize(&(*v8), data, pos),
            (
                ClientHelloExtensionRestDispatchCase::V10(inner),
                ClientHelloExtensionRest::SignatureAlgorithmsCert(v9),
            ) => inner.serialize(&(*v9), data, pos),
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
                ClientHelloExtensionRestDispatchCase::V1(inner),
                ClientHelloExtensionRestOwned::MaxFragmentLength(v0),
            ) => inner.serialize_gen(v0, data, pos),
            (
                ClientHelloExtensionRestDispatchCase::V2(inner),
                ClientHelloExtensionRestOwned::Heartbeat(v1),
            ) => inner.serialize_gen(v1, data, pos),
            (
                ClientHelloExtensionRestDispatchCase::V3(inner),
                ClientHelloExtensionRestOwned::SignedCertificateTimeStamp(v2),
            ) => inner.serialize_gen(v2, data, pos),
            (
                ClientHelloExtensionRestDispatchCase::V4(inner),
                ClientHelloExtensionRestOwned::ClientCertificateType(v3),
            ) => inner.serialize_gen(v3, data, pos),
            (
                ClientHelloExtensionRestDispatchCase::V5(inner),
                ClientHelloExtensionRestOwned::ServerCertificateType(v4),
            ) => inner.serialize_gen(v4, data, pos),
            (
                ClientHelloExtensionRestDispatchCase::V6(inner),
                ClientHelloExtensionRestOwned::Padding(v5),
            ) => inner.serialize_gen(v5, data, pos),
            (
                ClientHelloExtensionRestDispatchCase::V7(inner),
                ClientHelloExtensionRestOwned::Cookie(v6),
            ) => inner.serialize_gen(v6, data, pos),
            (
                ClientHelloExtensionRestDispatchCase::V8(inner),
                ClientHelloExtensionRestOwned::CertificateAuthorities(v7),
            ) => inner.serialize_gen(v7, data, pos),
            (
                ClientHelloExtensionRestDispatchCase::V9(inner),
                ClientHelloExtensionRestOwned::OidFilters(v8),
            ) => inner.serialize_gen(v8, data, pos),
            (
                ClientHelloExtensionRestDispatchCase::V10(inner),
                ClientHelloExtensionRestOwned::SignatureAlgorithmsCert(v9),
            ) => inner.serialize_gen(v9, data, pos),
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
            ClientHelloExtensionRestDispatchCase::V1(inner) => {
                let (n, v) = inner.generate(g)?;
                Ok((n, ClientHelloExtensionRestOwned::MaxFragmentLength(v)))
            }
            ClientHelloExtensionRestDispatchCase::V2(inner) => {
                let (n, v) = inner.generate(g)?;
                Ok((n, ClientHelloExtensionRestOwned::Heartbeat(v)))
            }
            ClientHelloExtensionRestDispatchCase::V3(inner) => {
                let (n, v) = inner.generate(g)?;
                Ok((n, ClientHelloExtensionRestOwned::SignedCertificateTimeStamp(v)))
            }
            ClientHelloExtensionRestDispatchCase::V4(inner) => {
                let (n, v) = inner.generate(g)?;
                Ok((n, ClientHelloExtensionRestOwned::ClientCertificateType(v)))
            }
            ClientHelloExtensionRestDispatchCase::V5(inner) => {
                let (n, v) = inner.generate(g)?;
                Ok((n, ClientHelloExtensionRestOwned::ServerCertificateType(v)))
            }
            ClientHelloExtensionRestDispatchCase::V6(inner) => {
                let (n, v) = inner.generate(g)?;
                Ok((n, ClientHelloExtensionRestOwned::Padding(v)))
            }
            ClientHelloExtensionRestDispatchCase::V7(inner) => {
                let (n, v) = inner.generate(g)?;
                Ok((n, ClientHelloExtensionRestOwned::Cookie(v)))
            }
            ClientHelloExtensionRestDispatchCase::V8(inner) => {
                let (n, v) = inner.generate(g)?;
                Ok((n, ClientHelloExtensionRestOwned::CertificateAuthorities(v)))
            }
            ClientHelloExtensionRestDispatchCase::V9(inner) => {
                let (n, v) = inner.generate(g)?;
                Ok((n, ClientHelloExtensionRestOwned::OidFilters(v)))
            }
            ClientHelloExtensionRestDispatchCase::V10(inner) => {
                let (n, v) = inner.generate(g)?;
                Ok((n, ClientHelloExtensionRestOwned::SignatureAlgorithmsCert(v)))
            }
        }
    }
    fn well_formed<'s>(&self, v: Self::SType<'s>) -> bool
    where
        [u8]: 's,
    {
        match (self, v) {
            (
                ClientHelloExtensionRestDispatchCase::V1(inner),
                ClientHelloExtensionRest::MaxFragmentLength(v0),
            ) => inner.well_formed((*v0)),
            (
                ClientHelloExtensionRestDispatchCase::V2(inner),
                ClientHelloExtensionRest::Heartbeat(v1),
            ) => inner.well_formed((*v1)),
            (
                ClientHelloExtensionRestDispatchCase::V3(inner),
                ClientHelloExtensionRest::SignedCertificateTimeStamp(v2),
            ) => inner.well_formed(&(*v2)),
            (
                ClientHelloExtensionRestDispatchCase::V4(inner),
                ClientHelloExtensionRest::ClientCertificateType(v3),
            ) => inner.well_formed(&(*v3)),
            (
                ClientHelloExtensionRestDispatchCase::V5(inner),
                ClientHelloExtensionRest::ServerCertificateType(v4),
            ) => inner.well_formed(&(*v4)),
            (
                ClientHelloExtensionRestDispatchCase::V6(inner),
                ClientHelloExtensionRest::Padding(v5),
            ) => inner.well_formed((*v5)),
            (
                ClientHelloExtensionRestDispatchCase::V7(inner),
                ClientHelloExtensionRest::Cookie(v6),
            ) => inner.well_formed(&(*v6)),
            (
                ClientHelloExtensionRestDispatchCase::V8(inner),
                ClientHelloExtensionRest::CertificateAuthorities(v7),
            ) => inner.well_formed(&(*v7)),
            (
                ClientHelloExtensionRestDispatchCase::V9(inner),
                ClientHelloExtensionRest::OidFilters(v8),
            ) => inner.well_formed(&(*v8)),
            (
                ClientHelloExtensionRestDispatchCase::V10(inner),
                ClientHelloExtensionRest::SignatureAlgorithmsCert(v9),
            ) => inner.well_formed(&(*v9)),
            _ => false,
        }
    }
}

pub enum ClientHelloExtensionExtensionDataDispatchCase0<
    C0 = ServerNameListCombinator,
    C1 = SignatureSchemeListCombinator,
    C2 = NamedGroupListCombinator,
    C3 = CertificateStatusRequestCombinator,
    C4 = ProtocolNameListCombinator,
    C5 = SupportedVersionsClientCombinator,
    C6 = KeyShareClientHelloCombinator,
    C7 = PskKeyExchangeModesCombinator,
    C8 = PreSharedKeyClientExtensionCombinator,
> {
    V1(C0),
    V2(C1),
    V3(C2),
    V4(C3),
    V5(C4),
    V6(C5),
    V7(C6),
    V8(C7),
    V9(C8),
}

impl<C0, C1, C2, C3, C4, C5, C6, C7, C8> Combinator<[u8], Vec<u8>>
for ClientHelloExtensionExtensionDataDispatchCase0<C0, C1, C2, C3, C4, C5, C6, C7, C8>
where
    for<'p, 's> C0: Combinator<
        [u8],
        Vec<u8>,
        Type<'p> = ServerNameList<'p>,
        SType<'s> = &'s ServerNameList<'s>,
        GType = ServerNameListOwned,
    >,
    for<'p, 's> C1: Combinator<
        [u8],
        Vec<u8>,
        Type<'p> = SignatureSchemeList,
        SType<'s> = &'s SignatureSchemeList,
        GType = SignatureSchemeList,
    >,
    for<'p, 's> C2: Combinator<
        [u8],
        Vec<u8>,
        Type<'p> = NamedGroupList,
        SType<'s> = &'s NamedGroupList,
        GType = NamedGroupList,
    >,
    for<'p, 's> C3: Combinator<
        [u8],
        Vec<u8>,
        Type<'p> = CertificateStatusRequest<'p>,
        SType<'s> = &'s CertificateStatusRequest<'s>,
        GType = CertificateStatusRequestOwned,
    >,
    for<'p, 's> C4: Combinator<
        [u8],
        Vec<u8>,
        Type<'p> = ProtocolNameList<'p>,
        SType<'s> = &'s ProtocolNameList<'s>,
        GType = ProtocolNameListOwned,
    >,
    for<'p, 's> C5: Combinator<
        [u8],
        Vec<u8>,
        Type<'p> = SupportedVersionsClient,
        SType<'s> = &'s SupportedVersionsClient,
        GType = SupportedVersionsClient,
    >,
    for<'p, 's> C6: Combinator<
        [u8],
        Vec<u8>,
        Type<'p> = KeyShareClientHello<'p>,
        SType<'s> = &'s KeyShareClientHello<'s>,
        GType = KeyShareClientHelloOwned,
    >,
    for<'p, 's> C7: Combinator<
        [u8],
        Vec<u8>,
        Type<'p> = PskKeyExchangeModes,
        SType<'s> = &'s PskKeyExchangeModes,
        GType = PskKeyExchangeModes,
    >,
    for<'p, 's> C8: Combinator<
        [u8],
        Vec<u8>,
        Type<'p> = PreSharedKeyClientExtension<'p>,
        SType<'s> = &'s PreSharedKeyClientExtension<'s>,
        GType = PreSharedKeyClientExtensionOwned,
    >,
{
    type Type<'p> = ClientHelloExtensionExtensionData<'p>;
    type SType<'s> = &'s ClientHelloExtensionExtensionData<'s>;
    type GType = ClientHelloExtensionExtensionDataOwned;
    fn length<'s>(&self, v: Self::SType<'s>) -> usize
    where
        [u8]: 's,
    {
        match (self, v) {
            (
                ClientHelloExtensionExtensionDataDispatchCase0::V1(inner),
                ClientHelloExtensionExtensionData::ServerName(v0),
            ) => inner.length(&(*v0)),
            (
                ClientHelloExtensionExtensionDataDispatchCase0::V2(inner),
                ClientHelloExtensionExtensionData::SignatureAlgorithms(v1),
            ) => inner.length(&(*v1)),
            (
                ClientHelloExtensionExtensionDataDispatchCase0::V3(inner),
                ClientHelloExtensionExtensionData::SupportedGroups(v2),
            ) => inner.length(&(*v2)),
            (
                ClientHelloExtensionExtensionDataDispatchCase0::V4(inner),
                ClientHelloExtensionExtensionData::StatusRequest(v3),
            ) => inner.length(&(*v3)),
            (
                ClientHelloExtensionExtensionDataDispatchCase0::V5(inner),
                ClientHelloExtensionExtensionData::ApplicationLayerProtocolNegotiation(
                    v4,
                ),
            ) => inner.length(&(*v4)),
            (
                ClientHelloExtensionExtensionDataDispatchCase0::V6(inner),
                ClientHelloExtensionExtensionData::SupportedVersions(v5),
            ) => inner.length(&(*v5)),
            (
                ClientHelloExtensionExtensionDataDispatchCase0::V7(inner),
                ClientHelloExtensionExtensionData::KeyShare(v6),
            ) => inner.length(&(*v6)),
            (
                ClientHelloExtensionExtensionDataDispatchCase0::V8(inner),
                ClientHelloExtensionExtensionData::PskKeyExchangeModes(v7),
            ) => inner.length(&(*v7)),
            (
                ClientHelloExtensionExtensionDataDispatchCase0::V9(inner),
                ClientHelloExtensionExtensionData::PreSharedKey(v8),
            ) => inner.length(&(*v8)),
            _ => panic!("dispatch branch combinator does not match value"),
        }
    }
    fn parse<'p>(&self, s: &'p [u8]) -> Result<(usize, Self::Type<'p>), ParseError>
    where
        [u8]: 'p,
    {
        match self {
            ClientHelloExtensionExtensionDataDispatchCase0::V1(inner) => {
                let (n, v) = inner.parse(s)?;
                Ok((n, ClientHelloExtensionExtensionData::ServerName(v)))
            }
            ClientHelloExtensionExtensionDataDispatchCase0::V2(inner) => {
                let (n, v) = inner.parse(s)?;
                Ok((n, ClientHelloExtensionExtensionData::SignatureAlgorithms(v)))
            }
            ClientHelloExtensionExtensionDataDispatchCase0::V3(inner) => {
                let (n, v) = inner.parse(s)?;
                Ok((n, ClientHelloExtensionExtensionData::SupportedGroups(v)))
            }
            ClientHelloExtensionExtensionDataDispatchCase0::V4(inner) => {
                let (n, v) = inner.parse(s)?;
                Ok((n, ClientHelloExtensionExtensionData::StatusRequest(v)))
            }
            ClientHelloExtensionExtensionDataDispatchCase0::V5(inner) => {
                let (n, v) = inner.parse(s)?;
                Ok((
                    n,
                    ClientHelloExtensionExtensionData::ApplicationLayerProtocolNegotiation(
                        v,
                    ),
                ))
            }
            ClientHelloExtensionExtensionDataDispatchCase0::V6(inner) => {
                let (n, v) = inner.parse(s)?;
                Ok((n, ClientHelloExtensionExtensionData::SupportedVersions(v)))
            }
            ClientHelloExtensionExtensionDataDispatchCase0::V7(inner) => {
                let (n, v) = inner.parse(s)?;
                Ok((n, ClientHelloExtensionExtensionData::KeyShare(v)))
            }
            ClientHelloExtensionExtensionDataDispatchCase0::V8(inner) => {
                let (n, v) = inner.parse(s)?;
                Ok((n, ClientHelloExtensionExtensionData::PskKeyExchangeModes(v)))
            }
            ClientHelloExtensionExtensionDataDispatchCase0::V9(inner) => {
                let (n, v) = inner.parse(s)?;
                Ok((n, ClientHelloExtensionExtensionData::PreSharedKey(v)))
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
            (
                ClientHelloExtensionExtensionDataDispatchCase0::V1(inner),
                ClientHelloExtensionExtensionData::ServerName(v0),
            ) => inner.serialize(&(*v0), data, pos),
            (
                ClientHelloExtensionExtensionDataDispatchCase0::V2(inner),
                ClientHelloExtensionExtensionData::SignatureAlgorithms(v1),
            ) => inner.serialize(&(*v1), data, pos),
            (
                ClientHelloExtensionExtensionDataDispatchCase0::V3(inner),
                ClientHelloExtensionExtensionData::SupportedGroups(v2),
            ) => inner.serialize(&(*v2), data, pos),
            (
                ClientHelloExtensionExtensionDataDispatchCase0::V4(inner),
                ClientHelloExtensionExtensionData::StatusRequest(v3),
            ) => inner.serialize(&(*v3), data, pos),
            (
                ClientHelloExtensionExtensionDataDispatchCase0::V5(inner),
                ClientHelloExtensionExtensionData::ApplicationLayerProtocolNegotiation(
                    v4,
                ),
            ) => inner.serialize(&(*v4), data, pos),
            (
                ClientHelloExtensionExtensionDataDispatchCase0::V6(inner),
                ClientHelloExtensionExtensionData::SupportedVersions(v5),
            ) => inner.serialize(&(*v5), data, pos),
            (
                ClientHelloExtensionExtensionDataDispatchCase0::V7(inner),
                ClientHelloExtensionExtensionData::KeyShare(v6),
            ) => inner.serialize(&(*v6), data, pos),
            (
                ClientHelloExtensionExtensionDataDispatchCase0::V8(inner),
                ClientHelloExtensionExtensionData::PskKeyExchangeModes(v7),
            ) => inner.serialize(&(*v7), data, pos),
            (
                ClientHelloExtensionExtensionDataDispatchCase0::V9(inner),
                ClientHelloExtensionExtensionData::PreSharedKey(v8),
            ) => inner.serialize(&(*v8), data, pos),
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
                ClientHelloExtensionExtensionDataDispatchCase0::V1(inner),
                ClientHelloExtensionExtensionDataOwned::ServerName(v0),
            ) => inner.serialize_gen(v0, data, pos),
            (
                ClientHelloExtensionExtensionDataDispatchCase0::V2(inner),
                ClientHelloExtensionExtensionDataOwned::SignatureAlgorithms(v1),
            ) => inner.serialize_gen(v1, data, pos),
            (
                ClientHelloExtensionExtensionDataDispatchCase0::V3(inner),
                ClientHelloExtensionExtensionDataOwned::SupportedGroups(v2),
            ) => inner.serialize_gen(v2, data, pos),
            (
                ClientHelloExtensionExtensionDataDispatchCase0::V4(inner),
                ClientHelloExtensionExtensionDataOwned::StatusRequest(v3),
            ) => inner.serialize_gen(v3, data, pos),
            (
                ClientHelloExtensionExtensionDataDispatchCase0::V5(inner),
                ClientHelloExtensionExtensionDataOwned::ApplicationLayerProtocolNegotiation(
                    v4,
                ),
            ) => inner.serialize_gen(v4, data, pos),
            (
                ClientHelloExtensionExtensionDataDispatchCase0::V6(inner),
                ClientHelloExtensionExtensionDataOwned::SupportedVersions(v5),
            ) => inner.serialize_gen(v5, data, pos),
            (
                ClientHelloExtensionExtensionDataDispatchCase0::V7(inner),
                ClientHelloExtensionExtensionDataOwned::KeyShare(v6),
            ) => inner.serialize_gen(v6, data, pos),
            (
                ClientHelloExtensionExtensionDataDispatchCase0::V8(inner),
                ClientHelloExtensionExtensionDataOwned::PskKeyExchangeModes(v7),
            ) => inner.serialize_gen(v7, data, pos),
            (
                ClientHelloExtensionExtensionDataDispatchCase0::V9(inner),
                ClientHelloExtensionExtensionDataOwned::PreSharedKey(v8),
            ) => inner.serialize_gen(v8, data, pos),
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
            ClientHelloExtensionExtensionDataDispatchCase0::V1(inner) => {
                let (n, v) = inner.generate(g)?;
                Ok((n, ClientHelloExtensionExtensionDataOwned::ServerName(v)))
            }
            ClientHelloExtensionExtensionDataDispatchCase0::V2(inner) => {
                let (n, v) = inner.generate(g)?;
                Ok((n, ClientHelloExtensionExtensionDataOwned::SignatureAlgorithms(v)))
            }
            ClientHelloExtensionExtensionDataDispatchCase0::V3(inner) => {
                let (n, v) = inner.generate(g)?;
                Ok((n, ClientHelloExtensionExtensionDataOwned::SupportedGroups(v)))
            }
            ClientHelloExtensionExtensionDataDispatchCase0::V4(inner) => {
                let (n, v) = inner.generate(g)?;
                Ok((n, ClientHelloExtensionExtensionDataOwned::StatusRequest(v)))
            }
            ClientHelloExtensionExtensionDataDispatchCase0::V5(inner) => {
                let (n, v) = inner.generate(g)?;
                Ok((
                    n,
                    ClientHelloExtensionExtensionDataOwned::ApplicationLayerProtocolNegotiation(
                        v,
                    ),
                ))
            }
            ClientHelloExtensionExtensionDataDispatchCase0::V6(inner) => {
                let (n, v) = inner.generate(g)?;
                Ok((n, ClientHelloExtensionExtensionDataOwned::SupportedVersions(v)))
            }
            ClientHelloExtensionExtensionDataDispatchCase0::V7(inner) => {
                let (n, v) = inner.generate(g)?;
                Ok((n, ClientHelloExtensionExtensionDataOwned::KeyShare(v)))
            }
            ClientHelloExtensionExtensionDataDispatchCase0::V8(inner) => {
                let (n, v) = inner.generate(g)?;
                Ok((n, ClientHelloExtensionExtensionDataOwned::PskKeyExchangeModes(v)))
            }
            ClientHelloExtensionExtensionDataDispatchCase0::V9(inner) => {
                let (n, v) = inner.generate(g)?;
                Ok((n, ClientHelloExtensionExtensionDataOwned::PreSharedKey(v)))
            }
        }
    }
    fn well_formed<'s>(&self, v: Self::SType<'s>) -> bool
    where
        [u8]: 's,
    {
        match (self, v) {
            (
                ClientHelloExtensionExtensionDataDispatchCase0::V1(inner),
                ClientHelloExtensionExtensionData::ServerName(v0),
            ) => inner.well_formed(&(*v0)),
            (
                ClientHelloExtensionExtensionDataDispatchCase0::V2(inner),
                ClientHelloExtensionExtensionData::SignatureAlgorithms(v1),
            ) => inner.well_formed(&(*v1)),
            (
                ClientHelloExtensionExtensionDataDispatchCase0::V3(inner),
                ClientHelloExtensionExtensionData::SupportedGroups(v2),
            ) => inner.well_formed(&(*v2)),
            (
                ClientHelloExtensionExtensionDataDispatchCase0::V4(inner),
                ClientHelloExtensionExtensionData::StatusRequest(v3),
            ) => inner.well_formed(&(*v3)),
            (
                ClientHelloExtensionExtensionDataDispatchCase0::V5(inner),
                ClientHelloExtensionExtensionData::ApplicationLayerProtocolNegotiation(
                    v4,
                ),
            ) => inner.well_formed(&(*v4)),
            (
                ClientHelloExtensionExtensionDataDispatchCase0::V6(inner),
                ClientHelloExtensionExtensionData::SupportedVersions(v5),
            ) => inner.well_formed(&(*v5)),
            (
                ClientHelloExtensionExtensionDataDispatchCase0::V7(inner),
                ClientHelloExtensionExtensionData::KeyShare(v6),
            ) => inner.well_formed(&(*v6)),
            (
                ClientHelloExtensionExtensionDataDispatchCase0::V8(inner),
                ClientHelloExtensionExtensionData::PskKeyExchangeModes(v7),
            ) => inner.well_formed(&(*v7)),
            (
                ClientHelloExtensionExtensionDataDispatchCase0::V9(inner),
                ClientHelloExtensionExtensionData::PreSharedKey(v8),
            ) => inner.well_formed(&(*v8)),
            _ => false,
        }
    }
}

#[derive(Clone, Copy)]
pub struct ClientHelloExtensionDep {}
impl DepCombinator<(ExtensionTypeCombinator, U16Be), [u8], Vec<u8>>
for ClientHelloExtensionDep {
    type Out = ClientHelloExtensionExtensionDataCombinator;
    type OutGen<'g> = ClientHelloExtensionExtensionDataCombinator<
        ClientHelloExtensionExtensionDataCombinatorAlias<'g>,
    >;
    fn dep_snd<'s>(&self, fst: (ExtensionType, u16)) -> Self::Out {
        let fst: (ExtensionType, u16) = fst;
        let (extension_type, ext_len) = fst;
        client_hello_extension_extension_data(ext_len, extension_type)
    }
    fn dep_snd_gen<'g>(&self, fst: &'g mut (ExtensionType, u16)) -> Self::OutGen<'g> {
        let fst: &'g mut (ExtensionType, u16) = fst;
        let (extension_type, ext_len) = fst;
        client_hello_extension_extension_data(ext_len, extension_type)
    }
}

#[derive(Clone, Copy)]
pub struct ClientExtensionsDep {}
impl DepCombinator<Refined<U16Be, fn(u16) -> bool>, [u8], Vec<u8>>
for ClientExtensionsDep {
    type Out = FixedLen<'static, Repeat<ClientHelloExtensionCombinator>>;
    type OutGen<'g> = FixedLen<'g, Repeat<ClientHelloExtensionCombinator>>;
    fn dep_snd<'s>(&self, fst: u16) -> Self::Out {
        let fst: u16 = fst;
        let l = fst;
        FixedLen(Length::from_value(l as usize), Repeat::new(client_hello_extension()))
    }
    fn dep_snd_gen<'g>(&self, fst: &'g mut u16) -> Self::OutGen<'g> {
        let fst: &'g mut u16 = fst;
        let l = fst;
        FixedLen(Length::from_u16_mut(l), Repeat::new(client_hello_extension()))
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

pub enum NewSessionTicketExtensionExtensionDataDispatchCase0<
    C0 = EarlyDataIndicationNewSessionTicketCombinator,
> {
    V1(C0),
}

impl<C0> Combinator<[u8], Vec<u8>>
for NewSessionTicketExtensionExtensionDataDispatchCase0<C0>
where
    for<'p, 's> C0: Combinator<
        [u8],
        Vec<u8>,
        Type<'p> = EarlyDataIndicationNewSessionTicket,
        SType<'s> = EarlyDataIndicationNewSessionTicket,
        GType = EarlyDataIndicationNewSessionTicket,
    >,
{
    type Type<'p> = NewSessionTicketExtensionExtensionData;
    type SType<'s> = &'s NewSessionTicketExtensionExtensionData;
    type GType = NewSessionTicketExtensionExtensionData;
    fn length<'s>(&self, v: Self::SType<'s>) -> usize
    where
        [u8]: 's,
    {
        match (self, v) {
            (
                NewSessionTicketExtensionExtensionDataDispatchCase0::V1(inner),
                NewSessionTicketExtensionExtensionData::EarlyData(v0),
            ) => inner.length((*v0)),
            _ => panic!("dispatch branch combinator does not match value"),
        }
    }
    fn parse<'p>(&self, s: &'p [u8]) -> Result<(usize, Self::Type<'p>), ParseError>
    where
        [u8]: 'p,
    {
        match self {
            NewSessionTicketExtensionExtensionDataDispatchCase0::V1(inner) => {
                let (n, v) = inner.parse(s)?;
                Ok((n, NewSessionTicketExtensionExtensionData::EarlyData(v)))
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
            (
                NewSessionTicketExtensionExtensionDataDispatchCase0::V1(inner),
                NewSessionTicketExtensionExtensionData::EarlyData(v0),
            ) => inner.serialize((*v0), data, pos),
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
                NewSessionTicketExtensionExtensionDataDispatchCase0::V1(inner),
                NewSessionTicketExtensionExtensionDataOwned::EarlyData(v0),
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
            NewSessionTicketExtensionExtensionDataDispatchCase0::V1(inner) => {
                let (n, v) = inner.generate(g)?;
                Ok((n, NewSessionTicketExtensionExtensionData::EarlyData(v)))
            }
        }
    }
    fn well_formed<'s>(&self, v: Self::SType<'s>) -> bool
    where
        [u8]: 's,
    {
        match (self, v) {
            (
                NewSessionTicketExtensionExtensionDataDispatchCase0::V1(inner),
                NewSessionTicketExtensionExtensionData::EarlyData(v0),
            ) => inner.well_formed((*v0)),
            _ => false,
        }
    }
}

#[derive(Clone, Copy)]
pub struct NewSessionTicketExtensionDep {}
impl DepCombinator<(ExtensionTypeCombinator, U16Be), [u8], Vec<u8>>
for NewSessionTicketExtensionDep {
    type Out = NewSessionTicketExtensionExtensionDataCombinator;
    type OutGen<'g> = NewSessionTicketExtensionExtensionDataCombinator<
        NewSessionTicketExtensionExtensionDataCombinatorAlias<'g>,
    >;
    fn dep_snd<'s>(&self, fst: (ExtensionType, u16)) -> Self::Out {
        let fst: (ExtensionType, u16) = fst;
        let (extension_type, ext_len) = fst;
        new_session_ticket_extension_extension_data(ext_len, extension_type)
    }
    fn dep_snd_gen<'g>(&self, fst: &'g mut (ExtensionType, u16)) -> Self::OutGen<'g> {
        let fst: &'g mut (ExtensionType, u16) = fst;
        let (extension_type, ext_len) = fst;
        new_session_ticket_extension_extension_data(ext_len, extension_type)
    }
}

#[derive(Clone, Copy)]
pub struct NewSessionTicketExtensionsDep {}
impl DepCombinator<Refined<U16Be, fn(u16) -> bool>, [u8], Vec<u8>>
for NewSessionTicketExtensionsDep {
    type Out = FixedLen<'static, Repeat<NewSessionTicketExtensionCombinator>>;
    type OutGen<'g> = FixedLen<'g, Repeat<NewSessionTicketExtensionCombinator>>;
    fn dep_snd<'s>(&self, fst: u16) -> Self::Out {
        let fst: u16 = fst;
        let l = fst;
        FixedLen(
            Length::from_value(l as usize),
            Repeat::new(new_session_ticket_extension()),
        )
    }
    fn dep_snd_gen<'g>(&self, fst: &'g mut u16) -> Self::OutGen<'g> {
        let fst: &'g mut u16 = fst;
        let l = fst;
        FixedLen(Length::from_u16_mut(l), Repeat::new(new_session_ticket_extension()))
    }
}

pub enum EncryptedExtensionExtensionDataDispatchCase0<
    C0 = EmptyCombinator,
    C1 = MaxFragmentLengthCombinator,
    C2 = NamedGroupListCombinator,
    C3 = HeartbeatModeCombinator,
    C4 = ProtocolNameListCombinator,
    C5 = ClientCertTypeClientExtensionCombinator,
    C6 = ServerCertTypeClientExtensionCombinator,
    C7 = EmptyCombinator,
> {
    V1(C0),
    V2(C1),
    V3(C2),
    V4(C3),
    V5(C4),
    V6(C5),
    V7(C6),
    V8(C7),
}

impl<C0, C1, C2, C3, C4, C5, C6, C7> Combinator<[u8], Vec<u8>>
for EncryptedExtensionExtensionDataDispatchCase0<C0, C1, C2, C3, C4, C5, C6, C7>
where
    for<'p, 's> C0: Combinator<
        [u8],
        Vec<u8>,
        Type<'p> = Empty<'p>,
        SType<'s> = Empty<'s>,
        GType = EmptyOwned,
    >,
    for<'p, 's> C1: Combinator<
        [u8],
        Vec<u8>,
        Type<'p> = MaxFragmentLength,
        SType<'s> = MaxFragmentLength,
        GType = MaxFragmentLength,
    >,
    for<'p, 's> C2: Combinator<
        [u8],
        Vec<u8>,
        Type<'p> = NamedGroupList,
        SType<'s> = &'s NamedGroupList,
        GType = NamedGroupList,
    >,
    for<'p, 's> C3: Combinator<
        [u8],
        Vec<u8>,
        Type<'p> = HeartbeatMode,
        SType<'s> = HeartbeatMode,
        GType = HeartbeatMode,
    >,
    for<'p, 's> C4: Combinator<
        [u8],
        Vec<u8>,
        Type<'p> = ProtocolNameList<'p>,
        SType<'s> = &'s ProtocolNameList<'s>,
        GType = ProtocolNameListOwned,
    >,
    for<'p, 's> C5: Combinator<
        [u8],
        Vec<u8>,
        Type<'p> = ClientCertTypeClientExtension,
        SType<'s> = &'s ClientCertTypeClientExtension,
        GType = ClientCertTypeClientExtension,
    >,
    for<'p, 's> C6: Combinator<
        [u8],
        Vec<u8>,
        Type<'p> = ServerCertTypeClientExtension,
        SType<'s> = &'s ServerCertTypeClientExtension,
        GType = ServerCertTypeClientExtension,
    >,
    for<'p, 's> C7: Combinator<
        [u8],
        Vec<u8>,
        Type<'p> = Empty<'p>,
        SType<'s> = Empty<'s>,
        GType = EmptyOwned,
    >,
{
    type Type<'p> = EncryptedExtensionExtensionData<'p>;
    type SType<'s> = &'s EncryptedExtensionExtensionData<'s>;
    type GType = EncryptedExtensionExtensionDataOwned;
    fn length<'s>(&self, v: Self::SType<'s>) -> usize
    where
        [u8]: 's,
    {
        match (self, v) {
            (
                EncryptedExtensionExtensionDataDispatchCase0::V1(inner),
                EncryptedExtensionExtensionData::ServerName(v0),
            ) => inner.length((*v0)),
            (
                EncryptedExtensionExtensionDataDispatchCase0::V2(inner),
                EncryptedExtensionExtensionData::MaxFragmentLength(v1),
            ) => inner.length((*v1)),
            (
                EncryptedExtensionExtensionDataDispatchCase0::V3(inner),
                EncryptedExtensionExtensionData::SupportedGroups(v2),
            ) => inner.length(&(*v2)),
            (
                EncryptedExtensionExtensionDataDispatchCase0::V4(inner),
                EncryptedExtensionExtensionData::Heartbeat(v3),
            ) => inner.length((*v3)),
            (
                EncryptedExtensionExtensionDataDispatchCase0::V5(inner),
                EncryptedExtensionExtensionData::ApplicationLayerProtocolNegotiation(v4),
            ) => inner.length(&(*v4)),
            (
                EncryptedExtensionExtensionDataDispatchCase0::V6(inner),
                EncryptedExtensionExtensionData::ClientCertificateType(v5),
            ) => inner.length(&(*v5)),
            (
                EncryptedExtensionExtensionDataDispatchCase0::V7(inner),
                EncryptedExtensionExtensionData::ServerCertificateType(v6),
            ) => inner.length(&(*v6)),
            (
                EncryptedExtensionExtensionDataDispatchCase0::V8(inner),
                EncryptedExtensionExtensionData::EarlyData(v7),
            ) => inner.length((*v7)),
            _ => panic!("dispatch branch combinator does not match value"),
        }
    }
    fn parse<'p>(&self, s: &'p [u8]) -> Result<(usize, Self::Type<'p>), ParseError>
    where
        [u8]: 'p,
    {
        match self {
            EncryptedExtensionExtensionDataDispatchCase0::V1(inner) => {
                let (n, v) = inner.parse(s)?;
                Ok((n, EncryptedExtensionExtensionData::ServerName(v)))
            }
            EncryptedExtensionExtensionDataDispatchCase0::V2(inner) => {
                let (n, v) = inner.parse(s)?;
                Ok((n, EncryptedExtensionExtensionData::MaxFragmentLength(v)))
            }
            EncryptedExtensionExtensionDataDispatchCase0::V3(inner) => {
                let (n, v) = inner.parse(s)?;
                Ok((n, EncryptedExtensionExtensionData::SupportedGroups(v)))
            }
            EncryptedExtensionExtensionDataDispatchCase0::V4(inner) => {
                let (n, v) = inner.parse(s)?;
                Ok((n, EncryptedExtensionExtensionData::Heartbeat(v)))
            }
            EncryptedExtensionExtensionDataDispatchCase0::V5(inner) => {
                let (n, v) = inner.parse(s)?;
                Ok((
                    n,
                    EncryptedExtensionExtensionData::ApplicationLayerProtocolNegotiation(
                        v,
                    ),
                ))
            }
            EncryptedExtensionExtensionDataDispatchCase0::V6(inner) => {
                let (n, v) = inner.parse(s)?;
                Ok((n, EncryptedExtensionExtensionData::ClientCertificateType(v)))
            }
            EncryptedExtensionExtensionDataDispatchCase0::V7(inner) => {
                let (n, v) = inner.parse(s)?;
                Ok((n, EncryptedExtensionExtensionData::ServerCertificateType(v)))
            }
            EncryptedExtensionExtensionDataDispatchCase0::V8(inner) => {
                let (n, v) = inner.parse(s)?;
                Ok((n, EncryptedExtensionExtensionData::EarlyData(v)))
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
            (
                EncryptedExtensionExtensionDataDispatchCase0::V1(inner),
                EncryptedExtensionExtensionData::ServerName(v0),
            ) => inner.serialize((*v0), data, pos),
            (
                EncryptedExtensionExtensionDataDispatchCase0::V2(inner),
                EncryptedExtensionExtensionData::MaxFragmentLength(v1),
            ) => inner.serialize((*v1), data, pos),
            (
                EncryptedExtensionExtensionDataDispatchCase0::V3(inner),
                EncryptedExtensionExtensionData::SupportedGroups(v2),
            ) => inner.serialize(&(*v2), data, pos),
            (
                EncryptedExtensionExtensionDataDispatchCase0::V4(inner),
                EncryptedExtensionExtensionData::Heartbeat(v3),
            ) => inner.serialize((*v3), data, pos),
            (
                EncryptedExtensionExtensionDataDispatchCase0::V5(inner),
                EncryptedExtensionExtensionData::ApplicationLayerProtocolNegotiation(v4),
            ) => inner.serialize(&(*v4), data, pos),
            (
                EncryptedExtensionExtensionDataDispatchCase0::V6(inner),
                EncryptedExtensionExtensionData::ClientCertificateType(v5),
            ) => inner.serialize(&(*v5), data, pos),
            (
                EncryptedExtensionExtensionDataDispatchCase0::V7(inner),
                EncryptedExtensionExtensionData::ServerCertificateType(v6),
            ) => inner.serialize(&(*v6), data, pos),
            (
                EncryptedExtensionExtensionDataDispatchCase0::V8(inner),
                EncryptedExtensionExtensionData::EarlyData(v7),
            ) => inner.serialize((*v7), data, pos),
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
                EncryptedExtensionExtensionDataDispatchCase0::V1(inner),
                EncryptedExtensionExtensionDataOwned::ServerName(v0),
            ) => inner.serialize_gen(v0, data, pos),
            (
                EncryptedExtensionExtensionDataDispatchCase0::V2(inner),
                EncryptedExtensionExtensionDataOwned::MaxFragmentLength(v1),
            ) => inner.serialize_gen(v1, data, pos),
            (
                EncryptedExtensionExtensionDataDispatchCase0::V3(inner),
                EncryptedExtensionExtensionDataOwned::SupportedGroups(v2),
            ) => inner.serialize_gen(v2, data, pos),
            (
                EncryptedExtensionExtensionDataDispatchCase0::V4(inner),
                EncryptedExtensionExtensionDataOwned::Heartbeat(v3),
            ) => inner.serialize_gen(v3, data, pos),
            (
                EncryptedExtensionExtensionDataDispatchCase0::V5(inner),
                EncryptedExtensionExtensionDataOwned::ApplicationLayerProtocolNegotiation(
                    v4,
                ),
            ) => inner.serialize_gen(v4, data, pos),
            (
                EncryptedExtensionExtensionDataDispatchCase0::V6(inner),
                EncryptedExtensionExtensionDataOwned::ClientCertificateType(v5),
            ) => inner.serialize_gen(v5, data, pos),
            (
                EncryptedExtensionExtensionDataDispatchCase0::V7(inner),
                EncryptedExtensionExtensionDataOwned::ServerCertificateType(v6),
            ) => inner.serialize_gen(v6, data, pos),
            (
                EncryptedExtensionExtensionDataDispatchCase0::V8(inner),
                EncryptedExtensionExtensionDataOwned::EarlyData(v7),
            ) => inner.serialize_gen(v7, data, pos),
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
            EncryptedExtensionExtensionDataDispatchCase0::V1(inner) => {
                let (n, v) = inner.generate(g)?;
                Ok((n, EncryptedExtensionExtensionDataOwned::ServerName(v)))
            }
            EncryptedExtensionExtensionDataDispatchCase0::V2(inner) => {
                let (n, v) = inner.generate(g)?;
                Ok((n, EncryptedExtensionExtensionDataOwned::MaxFragmentLength(v)))
            }
            EncryptedExtensionExtensionDataDispatchCase0::V3(inner) => {
                let (n, v) = inner.generate(g)?;
                Ok((n, EncryptedExtensionExtensionDataOwned::SupportedGroups(v)))
            }
            EncryptedExtensionExtensionDataDispatchCase0::V4(inner) => {
                let (n, v) = inner.generate(g)?;
                Ok((n, EncryptedExtensionExtensionDataOwned::Heartbeat(v)))
            }
            EncryptedExtensionExtensionDataDispatchCase0::V5(inner) => {
                let (n, v) = inner.generate(g)?;
                Ok((
                    n,
                    EncryptedExtensionExtensionDataOwned::ApplicationLayerProtocolNegotiation(
                        v,
                    ),
                ))
            }
            EncryptedExtensionExtensionDataDispatchCase0::V6(inner) => {
                let (n, v) = inner.generate(g)?;
                Ok((n, EncryptedExtensionExtensionDataOwned::ClientCertificateType(v)))
            }
            EncryptedExtensionExtensionDataDispatchCase0::V7(inner) => {
                let (n, v) = inner.generate(g)?;
                Ok((n, EncryptedExtensionExtensionDataOwned::ServerCertificateType(v)))
            }
            EncryptedExtensionExtensionDataDispatchCase0::V8(inner) => {
                let (n, v) = inner.generate(g)?;
                Ok((n, EncryptedExtensionExtensionDataOwned::EarlyData(v)))
            }
        }
    }
    fn well_formed<'s>(&self, v: Self::SType<'s>) -> bool
    where
        [u8]: 's,
    {
        match (self, v) {
            (
                EncryptedExtensionExtensionDataDispatchCase0::V1(inner),
                EncryptedExtensionExtensionData::ServerName(v0),
            ) => inner.well_formed((*v0)),
            (
                EncryptedExtensionExtensionDataDispatchCase0::V2(inner),
                EncryptedExtensionExtensionData::MaxFragmentLength(v1),
            ) => inner.well_formed((*v1)),
            (
                EncryptedExtensionExtensionDataDispatchCase0::V3(inner),
                EncryptedExtensionExtensionData::SupportedGroups(v2),
            ) => inner.well_formed(&(*v2)),
            (
                EncryptedExtensionExtensionDataDispatchCase0::V4(inner),
                EncryptedExtensionExtensionData::Heartbeat(v3),
            ) => inner.well_formed((*v3)),
            (
                EncryptedExtensionExtensionDataDispatchCase0::V5(inner),
                EncryptedExtensionExtensionData::ApplicationLayerProtocolNegotiation(v4),
            ) => inner.well_formed(&(*v4)),
            (
                EncryptedExtensionExtensionDataDispatchCase0::V6(inner),
                EncryptedExtensionExtensionData::ClientCertificateType(v5),
            ) => inner.well_formed(&(*v5)),
            (
                EncryptedExtensionExtensionDataDispatchCase0::V7(inner),
                EncryptedExtensionExtensionData::ServerCertificateType(v6),
            ) => inner.well_formed(&(*v6)),
            (
                EncryptedExtensionExtensionDataDispatchCase0::V8(inner),
                EncryptedExtensionExtensionData::EarlyData(v7),
            ) => inner.well_formed((*v7)),
            _ => false,
        }
    }
}

#[derive(Clone, Copy)]
pub struct EncryptedExtensionDep {}
impl DepCombinator<(ExtensionTypeCombinator, U16Be), [u8], Vec<u8>>
for EncryptedExtensionDep {
    type Out = EncryptedExtensionExtensionDataCombinator;
    type OutGen<'g> = EncryptedExtensionExtensionDataCombinator<
        EncryptedExtensionExtensionDataCombinatorAlias<'g>,
    >;
    fn dep_snd<'s>(&self, fst: (ExtensionType, u16)) -> Self::Out {
        let fst: (ExtensionType, u16) = fst;
        let (extension_type, ext_len) = fst;
        encrypted_extension_extension_data(ext_len, extension_type)
    }
    fn dep_snd_gen<'g>(&self, fst: &'g mut (ExtensionType, u16)) -> Self::OutGen<'g> {
        let fst: &'g mut (ExtensionType, u16) = fst;
        let (extension_type, ext_len) = fst;
        encrypted_extension_extension_data(ext_len, extension_type)
    }
}

#[derive(Clone, Copy)]
pub struct EncryptedExtensionsDep {}
impl DepCombinator<U16Be, [u8], Vec<u8>> for EncryptedExtensionsDep {
    type Out = FixedLen<'static, Repeat<EncryptedExtensionCombinator>>;
    type OutGen<'g> = FixedLen<'g, Repeat<EncryptedExtensionCombinator>>;
    fn dep_snd<'s>(&self, fst: u16) -> Self::Out {
        let fst: u16 = fst;
        let l = fst;
        FixedLen(Length::from_value(l as usize), Repeat::new(encrypted_extension()))
    }
    fn dep_snd_gen<'g>(&self, fst: &'g mut u16) -> Self::OutGen<'g> {
        let fst: &'g mut u16 = fst;
        let l = fst;
        FixedLen(Length::from_u16_mut(l), Repeat::new(encrypted_extension()))
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
        Variable(l.as_u32() as usize)
    }
    fn dep_snd_gen<'g>(&self, fst: &'g mut u24) -> Self::OutGen<'g> {
        let fst: &'g mut u24 = fst;
        let l = fst;
        Variable(*l.as_u32() as usize)
    }
}

pub enum CertificateExtensionExtensionDataDispatchCase0<
    C0 = CertificateStatusCombinator,
    C1 = SignedCertificateTimestampListCombinator,
> {
    V1(C0),
    V2(C1),
}

impl<C0, C1> Combinator<[u8], Vec<u8>>
for CertificateExtensionExtensionDataDispatchCase0<C0, C1>
where
    for<'p, 's> C0: Combinator<
        [u8],
        Vec<u8>,
        Type<'p> = CertificateStatus<'p>,
        SType<'s> = &'s CertificateStatus<'s>,
        GType = CertificateStatusOwned,
    >,
    for<'p, 's> C1: Combinator<
        [u8],
        Vec<u8>,
        Type<'p> = SignedCertificateTimestampList<'p>,
        SType<'s> = &'s SignedCertificateTimestampList<'s>,
        GType = SignedCertificateTimestampListOwned,
    >,
{
    type Type<'p> = CertificateExtensionExtensionData<'p>;
    type SType<'s> = &'s CertificateExtensionExtensionData<'s>;
    type GType = CertificateExtensionExtensionDataOwned;
    fn length<'s>(&self, v: Self::SType<'s>) -> usize
    where
        [u8]: 's,
    {
        match (self, v) {
            (
                CertificateExtensionExtensionDataDispatchCase0::V1(inner),
                CertificateExtensionExtensionData::StatusRequest(v0),
            ) => inner.length(&(*v0)),
            (
                CertificateExtensionExtensionDataDispatchCase0::V2(inner),
                CertificateExtensionExtensionData::SignedCertificateTimeStamp(v1),
            ) => inner.length(&(*v1)),
            _ => panic!("dispatch branch combinator does not match value"),
        }
    }
    fn parse<'p>(&self, s: &'p [u8]) -> Result<(usize, Self::Type<'p>), ParseError>
    where
        [u8]: 'p,
    {
        match self {
            CertificateExtensionExtensionDataDispatchCase0::V1(inner) => {
                let (n, v) = inner.parse(s)?;
                Ok((n, CertificateExtensionExtensionData::StatusRequest(v)))
            }
            CertificateExtensionExtensionDataDispatchCase0::V2(inner) => {
                let (n, v) = inner.parse(s)?;
                Ok((n, CertificateExtensionExtensionData::SignedCertificateTimeStamp(v)))
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
            (
                CertificateExtensionExtensionDataDispatchCase0::V1(inner),
                CertificateExtensionExtensionData::StatusRequest(v0),
            ) => inner.serialize(&(*v0), data, pos),
            (
                CertificateExtensionExtensionDataDispatchCase0::V2(inner),
                CertificateExtensionExtensionData::SignedCertificateTimeStamp(v1),
            ) => inner.serialize(&(*v1), data, pos),
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
                CertificateExtensionExtensionDataDispatchCase0::V1(inner),
                CertificateExtensionExtensionDataOwned::StatusRequest(v0),
            ) => inner.serialize_gen(v0, data, pos),
            (
                CertificateExtensionExtensionDataDispatchCase0::V2(inner),
                CertificateExtensionExtensionDataOwned::SignedCertificateTimeStamp(v1),
            ) => inner.serialize_gen(v1, data, pos),
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
            CertificateExtensionExtensionDataDispatchCase0::V1(inner) => {
                let (n, v) = inner.generate(g)?;
                Ok((n, CertificateExtensionExtensionDataOwned::StatusRequest(v)))
            }
            CertificateExtensionExtensionDataDispatchCase0::V2(inner) => {
                let (n, v) = inner.generate(g)?;
                Ok((
                    n,
                    CertificateExtensionExtensionDataOwned::SignedCertificateTimeStamp(v),
                ))
            }
        }
    }
    fn well_formed<'s>(&self, v: Self::SType<'s>) -> bool
    where
        [u8]: 's,
    {
        match (self, v) {
            (
                CertificateExtensionExtensionDataDispatchCase0::V1(inner),
                CertificateExtensionExtensionData::StatusRequest(v0),
            ) => inner.well_formed(&(*v0)),
            (
                CertificateExtensionExtensionDataDispatchCase0::V2(inner),
                CertificateExtensionExtensionData::SignedCertificateTimeStamp(v1),
            ) => inner.well_formed(&(*v1)),
            _ => false,
        }
    }
}

#[derive(Clone, Copy)]
pub struct CertificateExtensionDep {}
impl DepCombinator<(ExtensionTypeCombinator, U16Be), [u8], Vec<u8>>
for CertificateExtensionDep {
    type Out = CertificateExtensionExtensionDataCombinator;
    type OutGen<'g> = CertificateExtensionExtensionDataCombinator<
        CertificateExtensionExtensionDataCombinatorAlias<'g>,
    >;
    fn dep_snd<'s>(&self, fst: (ExtensionType, u16)) -> Self::Out {
        let fst: (ExtensionType, u16) = fst;
        let (extension_type, ext_len) = fst;
        certificate_extension_extension_data(ext_len, extension_type)
    }
    fn dep_snd_gen<'g>(&self, fst: &'g mut (ExtensionType, u16)) -> Self::OutGen<'g> {
        let fst: &'g mut (ExtensionType, u16) = fst;
        let (extension_type, ext_len) = fst;
        certificate_extension_extension_data(ext_len, extension_type)
    }
}

#[derive(Clone, Copy)]
pub struct CertificateExtensionsDep {}
impl DepCombinator<U16Be, [u8], Vec<u8>> for CertificateExtensionsDep {
    type Out = FixedLen<'static, Repeat<CertificateExtensionCombinator>>;
    type OutGen<'g> = FixedLen<'g, Repeat<CertificateExtensionCombinator>>;
    fn dep_snd<'s>(&self, fst: u16) -> Self::Out {
        let fst: u16 = fst;
        let l = fst;
        FixedLen(Length::from_value(l as usize), Repeat::new(certificate_extension()))
    }
    fn dep_snd_gen<'g>(&self, fst: &'g mut u16) -> Self::OutGen<'g> {
        let fst: &'g mut u16 = fst;
        let l = fst;
        FixedLen(Length::from_u16_mut(l), Repeat::new(certificate_extension()))
    }
}

#[derive(Clone, Copy)]
pub struct CertificateListDep {}
impl DepCombinator<U24Be, [u8], Vec<u8>> for CertificateListDep {
    type Out = FixedLen<'static, Repeat<CertificateEntryOpaqueCombinator>>;
    type OutGen<'g> = FixedLen<'g, Repeat<CertificateEntryOpaqueCombinator>>;
    fn dep_snd<'s>(&self, fst: u24) -> Self::Out {
        let fst: u24 = fst;
        let l = fst;
        FixedLen(
            Length::from_value(l.as_u32() as usize),
            Repeat::new(certificate_entry_opaque()),
        )
    }
    fn dep_snd_gen<'g>(&self, fst: &'g mut u24) -> Self::OutGen<'g> {
        let fst: &'g mut u24 = fst;
        let l = fst;
        FixedLen(
            Length::from_value(*l as usize),
            Repeat::new(certificate_entry_opaque()),
        )
    }
}

#[derive(Clone, Copy)]
pub struct CertificateRequestExtensionsDep {}
impl DepCombinator<Refined<U16Be, fn(u16) -> bool>, [u8], Vec<u8>>
for CertificateRequestExtensionsDep {
    type Out = FixedLen<'static, Repeat<CertificateRequestExtensionCombinator>>;
    type OutGen<'g> = FixedLen<'g, Repeat<CertificateRequestExtensionCombinator>>;
    fn dep_snd<'s>(&self, fst: u16) -> Self::Out {
        let fst: u16 = fst;
        let l = fst;
        FixedLen(
            Length::from_value(l as usize),
            Repeat::new(certificate_request_extension()),
        )
    }
    fn dep_snd_gen<'g>(&self, fst: &'g mut u16) -> Self::OutGen<'g> {
        let fst: &'g mut u16 = fst;
        let l = fst;
        FixedLen(Length::from_u16_mut(l), Repeat::new(certificate_request_extension()))
    }
}

pub enum FinishedDispatchCase<
    C0 = Fixed<12>,
    C1 = Fixed<20>,
    C2 = Fixed<32>,
    C3 = Fixed<48>,
    C4 = Fixed<64>,
> {
    V1(C0),
    V2(C1),
    V3(C2),
    V4(C3),
    V5(C4),
}

impl<C0, C1, C2, C3, C4> Combinator<[u8], Vec<u8>>
for FinishedDispatchCase<C0, C1, C2, C3, C4>
where
    for<'p, 's> C0: Combinator<
        [u8],
        Vec<u8>,
        Type<'p> = &'p [u8],
        SType<'s> = &'s [u8],
        GType = Vec<u8>,
    >,
    for<'p, 's> C1: Combinator<
        [u8],
        Vec<u8>,
        Type<'p> = &'p [u8],
        SType<'s> = &'s [u8],
        GType = Vec<u8>,
    >,
    for<'p, 's> C2: Combinator<
        [u8],
        Vec<u8>,
        Type<'p> = &'p [u8],
        SType<'s> = &'s [u8],
        GType = Vec<u8>,
    >,
    for<'p, 's> C3: Combinator<
        [u8],
        Vec<u8>,
        Type<'p> = &'p [u8],
        SType<'s> = &'s [u8],
        GType = Vec<u8>,
    >,
    for<'p, 's> C4: Combinator<
        [u8],
        Vec<u8>,
        Type<'p> = &'p [u8],
        SType<'s> = &'s [u8],
        GType = Vec<u8>,
    >,
{
    type Type<'p> = Finished<'p>;
    type SType<'s> = &'s Finished<'s>;
    type GType = FinishedOwned;
    fn length<'s>(&self, v: Self::SType<'s>) -> usize
    where
        [u8]: 's,
    {
        match (self, v) {
            (FinishedDispatchCase::V1(inner), Finished::Hash12(v0)) => {
                inner.length((*v0))
            }
            (FinishedDispatchCase::V2(inner), Finished::Hash20(v1)) => {
                inner.length((*v1))
            }
            (FinishedDispatchCase::V3(inner), Finished::Sha256(v2)) => {
                inner.length((*v2))
            }
            (FinishedDispatchCase::V4(inner), Finished::Sha384(v3)) => {
                inner.length((*v3))
            }
            (FinishedDispatchCase::V5(inner), Finished::Sha512(v4)) => {
                inner.length((*v4))
            }
            _ => panic!("dispatch branch combinator does not match value"),
        }
    }
    fn parse<'p>(&self, s: &'p [u8]) -> Result<(usize, Self::Type<'p>), ParseError>
    where
        [u8]: 'p,
    {
        match self {
            FinishedDispatchCase::V1(inner) => {
                let (n, v) = inner.parse(s)?;
                Ok((n, Finished::Hash12(v)))
            }
            FinishedDispatchCase::V2(inner) => {
                let (n, v) = inner.parse(s)?;
                Ok((n, Finished::Hash20(v)))
            }
            FinishedDispatchCase::V3(inner) => {
                let (n, v) = inner.parse(s)?;
                Ok((n, Finished::Sha256(v)))
            }
            FinishedDispatchCase::V4(inner) => {
                let (n, v) = inner.parse(s)?;
                Ok((n, Finished::Sha384(v)))
            }
            FinishedDispatchCase::V5(inner) => {
                let (n, v) = inner.parse(s)?;
                Ok((n, Finished::Sha512(v)))
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
            (FinishedDispatchCase::V1(inner), Finished::Hash12(v0)) => {
                inner.serialize((*v0), data, pos)
            }
            (FinishedDispatchCase::V2(inner), Finished::Hash20(v1)) => {
                inner.serialize((*v1), data, pos)
            }
            (FinishedDispatchCase::V3(inner), Finished::Sha256(v2)) => {
                inner.serialize((*v2), data, pos)
            }
            (FinishedDispatchCase::V4(inner), Finished::Sha384(v3)) => {
                inner.serialize((*v3), data, pos)
            }
            (FinishedDispatchCase::V5(inner), Finished::Sha512(v4)) => {
                inner.serialize((*v4), data, pos)
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
            (FinishedDispatchCase::V1(inner), FinishedOwned::Hash12(v0)) => {
                inner.serialize_gen(v0, data, pos)
            }
            (FinishedDispatchCase::V2(inner), FinishedOwned::Hash20(v1)) => {
                inner.serialize_gen(v1, data, pos)
            }
            (FinishedDispatchCase::V3(inner), FinishedOwned::Sha256(v2)) => {
                inner.serialize_gen(v2, data, pos)
            }
            (FinishedDispatchCase::V4(inner), FinishedOwned::Sha384(v3)) => {
                inner.serialize_gen(v3, data, pos)
            }
            (FinishedDispatchCase::V5(inner), FinishedOwned::Sha512(v4)) => {
                inner.serialize_gen(v4, data, pos)
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
    fn generate(&mut self, g: &mut GenSt) -> GResult<Self::GType, GenerateError> {
        match self {
            FinishedDispatchCase::V1(inner) => {
                let (n, v) = inner.generate(g)?;
                Ok((n, FinishedOwned::Hash12(v)))
            }
            FinishedDispatchCase::V2(inner) => {
                let (n, v) = inner.generate(g)?;
                Ok((n, FinishedOwned::Hash20(v)))
            }
            FinishedDispatchCase::V3(inner) => {
                let (n, v) = inner.generate(g)?;
                Ok((n, FinishedOwned::Sha256(v)))
            }
            FinishedDispatchCase::V4(inner) => {
                let (n, v) = inner.generate(g)?;
                Ok((n, FinishedOwned::Sha384(v)))
            }
            FinishedDispatchCase::V5(inner) => {
                let (n, v) = inner.generate(g)?;
                Ok((n, FinishedOwned::Sha512(v)))
            }
        }
    }
    fn well_formed<'s>(&self, v: Self::SType<'s>) -> bool
    where
        [u8]: 's,
    {
        match (self, v) {
            (FinishedDispatchCase::V1(inner), Finished::Hash12(v0)) => {
                inner.well_formed((*v0))
            }
            (FinishedDispatchCase::V2(inner), Finished::Hash20(v1)) => {
                inner.well_formed((*v1))
            }
            (FinishedDispatchCase::V3(inner), Finished::Sha256(v2)) => {
                inner.well_formed((*v2))
            }
            (FinishedDispatchCase::V4(inner), Finished::Sha384(v3)) => {
                inner.well_formed((*v3))
            }
            (FinishedDispatchCase::V5(inner), Finished::Sha512(v4)) => {
                inner.well_formed((*v4))
            }
            _ => false,
        }
    }
}

pub enum HandshakeMsgDispatchCase0<
    C0 = ClientHelloCombinator,
    C1 = ShOrHrrCombinator,
    C2 = NewSessionTicketCombinator,
    C3 = EmptyCombinator,
    C4 = EncryptedExtensionsCombinator,
    C5 = CertificateCombinator,
    C6 = CertificateRequestCombinator,
    C7 = CertificateVerifyCombinator,
    C8 = FinishedCombinator,
    C9 = KeyUpdateCombinator,
> {
    V1(C0),
    V2(C1),
    V3(C2),
    V4(C3),
    V5(C4),
    V6(C5),
    V7(C6),
    V8(C7),
    V9(C8),
    V10(C9),
}

impl<C0, C1, C2, C3, C4, C5, C6, C7, C8, C9> Combinator<[u8], Vec<u8>>
for HandshakeMsgDispatchCase0<C0, C1, C2, C3, C4, C5, C6, C7, C8, C9>
where
    for<'p, 's> C0: Combinator<
        [u8],
        Vec<u8>,
        Type<'p> = ClientHello<'p>,
        SType<'s> = &'s ClientHello<'s>,
        GType = ClientHelloOwned,
    >,
    for<'p, 's> C1: Combinator<
        [u8],
        Vec<u8>,
        Type<'p> = ShOrHrr<'p>,
        SType<'s> = &'s ShOrHrr<'s>,
        GType = ShOrHrrOwned,
    >,
    for<'p, 's> C2: Combinator<
        [u8],
        Vec<u8>,
        Type<'p> = NewSessionTicket<'p>,
        SType<'s> = &'s NewSessionTicket<'s>,
        GType = NewSessionTicketOwned,
    >,
    for<'p, 's> C3: Combinator<
        [u8],
        Vec<u8>,
        Type<'p> = Empty<'p>,
        SType<'s> = Empty<'s>,
        GType = EmptyOwned,
    >,
    for<'p, 's> C4: Combinator<
        [u8],
        Vec<u8>,
        Type<'p> = EncryptedExtensions<'p>,
        SType<'s> = &'s EncryptedExtensions<'s>,
        GType = EncryptedExtensionsOwned,
    >,
    for<'p, 's> C5: Combinator<
        [u8],
        Vec<u8>,
        Type<'p> = Certificate<'p>,
        SType<'s> = &'s Certificate<'s>,
        GType = CertificateOwned,
    >,
    for<'p, 's> C6: Combinator<
        [u8],
        Vec<u8>,
        Type<'p> = CertificateRequest<'p>,
        SType<'s> = &'s CertificateRequest<'s>,
        GType = CertificateRequestOwned,
    >,
    for<'p, 's> C7: Combinator<
        [u8],
        Vec<u8>,
        Type<'p> = CertificateVerify<'p>,
        SType<'s> = &'s CertificateVerify<'s>,
        GType = CertificateVerifyOwned,
    >,
    for<'p, 's> C8: Combinator<
        [u8],
        Vec<u8>,
        Type<'p> = Finished<'p>,
        SType<'s> = &'s Finished<'s>,
        GType = FinishedOwned,
    >,
    for<'p, 's> C9: Combinator<
        [u8],
        Vec<u8>,
        Type<'p> = KeyUpdate,
        SType<'s> = KeyUpdate,
        GType = KeyUpdate,
    >,
{
    type Type<'p> = HandshakeMsg<'p>;
    type SType<'s> = &'s HandshakeMsg<'s>;
    type GType = HandshakeMsgOwned;
    fn length<'s>(&self, v: Self::SType<'s>) -> usize
    where
        [u8]: 's,
    {
        match (self, v) {
            (HandshakeMsgDispatchCase0::V1(inner), HandshakeMsg::ClientHello(v0)) => {
                inner.length(&(*v0))
            }
            (HandshakeMsgDispatchCase0::V2(inner), HandshakeMsg::ServerHello(v1)) => {
                inner.length(&(*v1))
            }
            (
                HandshakeMsgDispatchCase0::V3(inner),
                HandshakeMsg::NewSessionTicket(v2),
            ) => inner.length(&(*v2)),
            (HandshakeMsgDispatchCase0::V4(inner), HandshakeMsg::EndOfEarlyData(v3)) => {
                inner.length((*v3))
            }
            (
                HandshakeMsgDispatchCase0::V5(inner),
                HandshakeMsg::EncryptedExtensions(v4),
            ) => inner.length(&(*v4)),
            (HandshakeMsgDispatchCase0::V6(inner), HandshakeMsg::Certificate(v5)) => {
                inner.length(&(*v5))
            }
            (
                HandshakeMsgDispatchCase0::V7(inner),
                HandshakeMsg::CertificateRequest(v6),
            ) => inner.length(&(*v6)),
            (
                HandshakeMsgDispatchCase0::V8(inner),
                HandshakeMsg::CertificateVerify(v7),
            ) => inner.length(&(*v7)),
            (HandshakeMsgDispatchCase0::V9(inner), HandshakeMsg::Finished(v8)) => {
                inner.length(&(*v8))
            }
            (HandshakeMsgDispatchCase0::V10(inner), HandshakeMsg::KeyUpdate(v9)) => {
                inner.length((*v9))
            }
            _ => panic!("dispatch branch combinator does not match value"),
        }
    }
    fn parse<'p>(&self, s: &'p [u8]) -> Result<(usize, Self::Type<'p>), ParseError>
    where
        [u8]: 'p,
    {
        match self {
            HandshakeMsgDispatchCase0::V1(inner) => {
                let (n, v) = inner.parse(s)?;
                Ok((n, HandshakeMsg::ClientHello(v)))
            }
            HandshakeMsgDispatchCase0::V2(inner) => {
                let (n, v) = inner.parse(s)?;
                Ok((n, HandshakeMsg::ServerHello(v)))
            }
            HandshakeMsgDispatchCase0::V3(inner) => {
                let (n, v) = inner.parse(s)?;
                Ok((n, HandshakeMsg::NewSessionTicket(v)))
            }
            HandshakeMsgDispatchCase0::V4(inner) => {
                let (n, v) = inner.parse(s)?;
                Ok((n, HandshakeMsg::EndOfEarlyData(v)))
            }
            HandshakeMsgDispatchCase0::V5(inner) => {
                let (n, v) = inner.parse(s)?;
                Ok((n, HandshakeMsg::EncryptedExtensions(v)))
            }
            HandshakeMsgDispatchCase0::V6(inner) => {
                let (n, v) = inner.parse(s)?;
                Ok((n, HandshakeMsg::Certificate(v)))
            }
            HandshakeMsgDispatchCase0::V7(inner) => {
                let (n, v) = inner.parse(s)?;
                Ok((n, HandshakeMsg::CertificateRequest(v)))
            }
            HandshakeMsgDispatchCase0::V8(inner) => {
                let (n, v) = inner.parse(s)?;
                Ok((n, HandshakeMsg::CertificateVerify(v)))
            }
            HandshakeMsgDispatchCase0::V9(inner) => {
                let (n, v) = inner.parse(s)?;
                Ok((n, HandshakeMsg::Finished(v)))
            }
            HandshakeMsgDispatchCase0::V10(inner) => {
                let (n, v) = inner.parse(s)?;
                Ok((n, HandshakeMsg::KeyUpdate(v)))
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
            (HandshakeMsgDispatchCase0::V1(inner), HandshakeMsg::ClientHello(v0)) => {
                inner.serialize(&(*v0), data, pos)
            }
            (HandshakeMsgDispatchCase0::V2(inner), HandshakeMsg::ServerHello(v1)) => {
                inner.serialize(&(*v1), data, pos)
            }
            (
                HandshakeMsgDispatchCase0::V3(inner),
                HandshakeMsg::NewSessionTicket(v2),
            ) => inner.serialize(&(*v2), data, pos),
            (HandshakeMsgDispatchCase0::V4(inner), HandshakeMsg::EndOfEarlyData(v3)) => {
                inner.serialize((*v3), data, pos)
            }
            (
                HandshakeMsgDispatchCase0::V5(inner),
                HandshakeMsg::EncryptedExtensions(v4),
            ) => inner.serialize(&(*v4), data, pos),
            (HandshakeMsgDispatchCase0::V6(inner), HandshakeMsg::Certificate(v5)) => {
                inner.serialize(&(*v5), data, pos)
            }
            (
                HandshakeMsgDispatchCase0::V7(inner),
                HandshakeMsg::CertificateRequest(v6),
            ) => inner.serialize(&(*v6), data, pos),
            (
                HandshakeMsgDispatchCase0::V8(inner),
                HandshakeMsg::CertificateVerify(v7),
            ) => inner.serialize(&(*v7), data, pos),
            (HandshakeMsgDispatchCase0::V9(inner), HandshakeMsg::Finished(v8)) => {
                inner.serialize(&(*v8), data, pos)
            }
            (HandshakeMsgDispatchCase0::V10(inner), HandshakeMsg::KeyUpdate(v9)) => {
                inner.serialize((*v9), data, pos)
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
                HandshakeMsgDispatchCase0::V1(inner),
                HandshakeMsgOwned::ClientHello(v0),
            ) => inner.serialize_gen(v0, data, pos),
            (
                HandshakeMsgDispatchCase0::V2(inner),
                HandshakeMsgOwned::ServerHello(v1),
            ) => inner.serialize_gen(v1, data, pos),
            (
                HandshakeMsgDispatchCase0::V3(inner),
                HandshakeMsgOwned::NewSessionTicket(v2),
            ) => inner.serialize_gen(v2, data, pos),
            (
                HandshakeMsgDispatchCase0::V4(inner),
                HandshakeMsgOwned::EndOfEarlyData(v3),
            ) => inner.serialize_gen(v3, data, pos),
            (
                HandshakeMsgDispatchCase0::V5(inner),
                HandshakeMsgOwned::EncryptedExtensions(v4),
            ) => inner.serialize_gen(v4, data, pos),
            (
                HandshakeMsgDispatchCase0::V6(inner),
                HandshakeMsgOwned::Certificate(v5),
            ) => inner.serialize_gen(v5, data, pos),
            (
                HandshakeMsgDispatchCase0::V7(inner),
                HandshakeMsgOwned::CertificateRequest(v6),
            ) => inner.serialize_gen(v6, data, pos),
            (
                HandshakeMsgDispatchCase0::V8(inner),
                HandshakeMsgOwned::CertificateVerify(v7),
            ) => inner.serialize_gen(v7, data, pos),
            (HandshakeMsgDispatchCase0::V9(inner), HandshakeMsgOwned::Finished(v8)) => {
                inner.serialize_gen(v8, data, pos)
            }
            (HandshakeMsgDispatchCase0::V10(inner), HandshakeMsgOwned::KeyUpdate(v9)) => {
                inner.serialize_gen(v9, data, pos)
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
    fn generate(&mut self, g: &mut GenSt) -> GResult<Self::GType, GenerateError> {
        match self {
            HandshakeMsgDispatchCase0::V1(inner) => {
                let (n, v) = inner.generate(g)?;
                Ok((n, HandshakeMsgOwned::ClientHello(v)))
            }
            HandshakeMsgDispatchCase0::V2(inner) => {
                let (n, v) = inner.generate(g)?;
                Ok((n, HandshakeMsgOwned::ServerHello(v)))
            }
            HandshakeMsgDispatchCase0::V3(inner) => {
                let (n, v) = inner.generate(g)?;
                Ok((n, HandshakeMsgOwned::NewSessionTicket(v)))
            }
            HandshakeMsgDispatchCase0::V4(inner) => {
                let (n, v) = inner.generate(g)?;
                Ok((n, HandshakeMsgOwned::EndOfEarlyData(v)))
            }
            HandshakeMsgDispatchCase0::V5(inner) => {
                let (n, v) = inner.generate(g)?;
                Ok((n, HandshakeMsgOwned::EncryptedExtensions(v)))
            }
            HandshakeMsgDispatchCase0::V6(inner) => {
                let (n, v) = inner.generate(g)?;
                Ok((n, HandshakeMsgOwned::Certificate(v)))
            }
            HandshakeMsgDispatchCase0::V7(inner) => {
                let (n, v) = inner.generate(g)?;
                Ok((n, HandshakeMsgOwned::CertificateRequest(v)))
            }
            HandshakeMsgDispatchCase0::V8(inner) => {
                let (n, v) = inner.generate(g)?;
                Ok((n, HandshakeMsgOwned::CertificateVerify(v)))
            }
            HandshakeMsgDispatchCase0::V9(inner) => {
                let (n, v) = inner.generate(g)?;
                Ok((n, HandshakeMsgOwned::Finished(v)))
            }
            HandshakeMsgDispatchCase0::V10(inner) => {
                let (n, v) = inner.generate(g)?;
                Ok((n, HandshakeMsgOwned::KeyUpdate(v)))
            }
        }
    }
    fn well_formed<'s>(&self, v: Self::SType<'s>) -> bool
    where
        [u8]: 's,
    {
        match (self, v) {
            (HandshakeMsgDispatchCase0::V1(inner), HandshakeMsg::ClientHello(v0)) => {
                inner.well_formed(&(*v0))
            }
            (HandshakeMsgDispatchCase0::V2(inner), HandshakeMsg::ServerHello(v1)) => {
                inner.well_formed(&(*v1))
            }
            (
                HandshakeMsgDispatchCase0::V3(inner),
                HandshakeMsg::NewSessionTicket(v2),
            ) => inner.well_formed(&(*v2)),
            (HandshakeMsgDispatchCase0::V4(inner), HandshakeMsg::EndOfEarlyData(v3)) => {
                inner.well_formed((*v3))
            }
            (
                HandshakeMsgDispatchCase0::V5(inner),
                HandshakeMsg::EncryptedExtensions(v4),
            ) => inner.well_formed(&(*v4)),
            (HandshakeMsgDispatchCase0::V6(inner), HandshakeMsg::Certificate(v5)) => {
                inner.well_formed(&(*v5))
            }
            (
                HandshakeMsgDispatchCase0::V7(inner),
                HandshakeMsg::CertificateRequest(v6),
            ) => inner.well_formed(&(*v6)),
            (
                HandshakeMsgDispatchCase0::V8(inner),
                HandshakeMsg::CertificateVerify(v7),
            ) => inner.well_formed(&(*v7)),
            (HandshakeMsgDispatchCase0::V9(inner), HandshakeMsg::Finished(v8)) => {
                inner.well_formed(&(*v8))
            }
            (HandshakeMsgDispatchCase0::V10(inner), HandshakeMsg::KeyUpdate(v9)) => {
                inner.well_formed((*v9))
            }
            _ => false,
        }
    }
}

#[derive(Clone, Copy)]
pub struct HandshakeDep {}
impl DepCombinator<(HandshakeTypeCombinator, U24Be), [u8], Vec<u8>> for HandshakeDep {
    type Out = HandshakeMsgCombinator;
    type OutGen<'g> = HandshakeMsgCombinator<HandshakeMsgCombinatorAlias<'g>>;
    fn dep_snd<'s>(&self, fst: (HandshakeType, u24)) -> Self::Out {
        let fst: (HandshakeType, u24) = fst;
        let (msg_type, length) = fst;
        handshake_msg(length, msg_type)
    }
    fn dep_snd_gen<'g>(&self, fst: &'g mut (HandshakeType, u24)) -> Self::OutGen<'g> {
        let fst: &'g mut (HandshakeType, u24) = fst;
        let (msg_type, length) = fst;
        handshake_msg(length, msg_type)
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

pub enum HelloRetryExtensionExtensionDataDispatchCase0<
    C0 = SupportedVersionsServerCombinator,
    C1 = CookieCombinator,
    C2 = NamedGroupCombinator,
> {
    V1(C0),
    V2(C1),
    V3(C2),
}

impl<C0, C1, C2> Combinator<[u8], Vec<u8>>
for HelloRetryExtensionExtensionDataDispatchCase0<C0, C1, C2>
where
    for<'p, 's> C0: Combinator<
        [u8],
        Vec<u8>,
        Type<'p> = SupportedVersionsServer,
        SType<'s> = SupportedVersionsServer,
        GType = SupportedVersionsServer,
    >,
    for<'p, 's> C1: Combinator<
        [u8],
        Vec<u8>,
        Type<'p> = Cookie<'p>,
        SType<'s> = &'s Cookie<'s>,
        GType = CookieOwned,
    >,
    for<'p, 's> C2: Combinator<
        [u8],
        Vec<u8>,
        Type<'p> = NamedGroup,
        SType<'s> = NamedGroup,
        GType = NamedGroup,
    >,
{
    type Type<'p> = HelloRetryExtensionExtensionData<'p>;
    type SType<'s> = &'s HelloRetryExtensionExtensionData<'s>;
    type GType = HelloRetryExtensionExtensionDataOwned;
    fn length<'s>(&self, v: Self::SType<'s>) -> usize
    where
        [u8]: 's,
    {
        match (self, v) {
            (
                HelloRetryExtensionExtensionDataDispatchCase0::V1(inner),
                HelloRetryExtensionExtensionData::SupportedVersions(v0),
            ) => inner.length((*v0)),
            (
                HelloRetryExtensionExtensionDataDispatchCase0::V2(inner),
                HelloRetryExtensionExtensionData::Cookie(v1),
            ) => inner.length(&(*v1)),
            (
                HelloRetryExtensionExtensionDataDispatchCase0::V3(inner),
                HelloRetryExtensionExtensionData::KeyShare(v2),
            ) => inner.length((*v2)),
            _ => panic!("dispatch branch combinator does not match value"),
        }
    }
    fn parse<'p>(&self, s: &'p [u8]) -> Result<(usize, Self::Type<'p>), ParseError>
    where
        [u8]: 'p,
    {
        match self {
            HelloRetryExtensionExtensionDataDispatchCase0::V1(inner) => {
                let (n, v) = inner.parse(s)?;
                Ok((n, HelloRetryExtensionExtensionData::SupportedVersions(v)))
            }
            HelloRetryExtensionExtensionDataDispatchCase0::V2(inner) => {
                let (n, v) = inner.parse(s)?;
                Ok((n, HelloRetryExtensionExtensionData::Cookie(v)))
            }
            HelloRetryExtensionExtensionDataDispatchCase0::V3(inner) => {
                let (n, v) = inner.parse(s)?;
                Ok((n, HelloRetryExtensionExtensionData::KeyShare(v)))
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
            (
                HelloRetryExtensionExtensionDataDispatchCase0::V1(inner),
                HelloRetryExtensionExtensionData::SupportedVersions(v0),
            ) => inner.serialize((*v0), data, pos),
            (
                HelloRetryExtensionExtensionDataDispatchCase0::V2(inner),
                HelloRetryExtensionExtensionData::Cookie(v1),
            ) => inner.serialize(&(*v1), data, pos),
            (
                HelloRetryExtensionExtensionDataDispatchCase0::V3(inner),
                HelloRetryExtensionExtensionData::KeyShare(v2),
            ) => inner.serialize((*v2), data, pos),
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
                HelloRetryExtensionExtensionDataDispatchCase0::V1(inner),
                HelloRetryExtensionExtensionDataOwned::SupportedVersions(v0),
            ) => inner.serialize_gen(v0, data, pos),
            (
                HelloRetryExtensionExtensionDataDispatchCase0::V2(inner),
                HelloRetryExtensionExtensionDataOwned::Cookie(v1),
            ) => inner.serialize_gen(v1, data, pos),
            (
                HelloRetryExtensionExtensionDataDispatchCase0::V3(inner),
                HelloRetryExtensionExtensionDataOwned::KeyShare(v2),
            ) => inner.serialize_gen(v2, data, pos),
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
            HelloRetryExtensionExtensionDataDispatchCase0::V1(inner) => {
                let (n, v) = inner.generate(g)?;
                Ok((n, HelloRetryExtensionExtensionDataOwned::SupportedVersions(v)))
            }
            HelloRetryExtensionExtensionDataDispatchCase0::V2(inner) => {
                let (n, v) = inner.generate(g)?;
                Ok((n, HelloRetryExtensionExtensionDataOwned::Cookie(v)))
            }
            HelloRetryExtensionExtensionDataDispatchCase0::V3(inner) => {
                let (n, v) = inner.generate(g)?;
                Ok((n, HelloRetryExtensionExtensionDataOwned::KeyShare(v)))
            }
        }
    }
    fn well_formed<'s>(&self, v: Self::SType<'s>) -> bool
    where
        [u8]: 's,
    {
        match (self, v) {
            (
                HelloRetryExtensionExtensionDataDispatchCase0::V1(inner),
                HelloRetryExtensionExtensionData::SupportedVersions(v0),
            ) => inner.well_formed((*v0)),
            (
                HelloRetryExtensionExtensionDataDispatchCase0::V2(inner),
                HelloRetryExtensionExtensionData::Cookie(v1),
            ) => inner.well_formed(&(*v1)),
            (
                HelloRetryExtensionExtensionDataDispatchCase0::V3(inner),
                HelloRetryExtensionExtensionData::KeyShare(v2),
            ) => inner.well_formed((*v2)),
            _ => false,
        }
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
pub struct HelloRetryExtensionDep {}
impl DepCombinator<(ExtensionTypeCombinator, U16Be), [u8], Vec<u8>>
for HelloRetryExtensionDep {
    type Out = HelloRetryExtensionExtensionDataCombinator;
    type OutGen<'g> = HelloRetryExtensionExtensionDataCombinator<
        HelloRetryExtensionExtensionDataCombinatorAlias<'g>,
    >;
    fn dep_snd<'s>(&self, fst: (ExtensionType, u16)) -> Self::Out {
        let fst: (ExtensionType, u16) = fst;
        let (extension_type, ext_len) = fst;
        hello_retry_extension_extension_data(ext_len, extension_type)
    }
    fn dep_snd_gen<'g>(&self, fst: &'g mut (ExtensionType, u16)) -> Self::OutGen<'g> {
        let fst: &'g mut (ExtensionType, u16) = fst;
        let (extension_type, ext_len) = fst;
        hello_retry_extension_extension_data(ext_len, extension_type)
    }
}

#[derive(Clone, Copy)]
pub struct HelloRetryExtensionsDep {}
impl DepCombinator<Refined<U16Be, fn(u16) -> bool>, [u8], Vec<u8>>
for HelloRetryExtensionsDep {
    type Out = FixedLen<'static, Repeat<HelloRetryExtensionCombinator>>;
    type OutGen<'g> = FixedLen<'g, Repeat<HelloRetryExtensionCombinator>>;
    fn dep_snd<'s>(&self, fst: u16) -> Self::Out {
        let fst: u16 = fst;
        let l = fst;
        FixedLen(Length::from_value(l as usize), Repeat::new(hello_retry_extension()))
    }
    fn dep_snd_gen<'g>(&self, fst: &'g mut u16) -> Self::OutGen<'g> {
        let fst: &'g mut u16 = fst;
        let l = fst;
        FixedLen(Length::from_u16_mut(l), Repeat::new(hello_retry_extension()))
    }
}

pub enum SeverHelloExtensionExtensionDataDispatchCase0<
    C0 = PreSharedKeyServerExtensionCombinator,
    C1 = SupportedVersionsServerCombinator,
    C2 = KeyShareEntryCombinator,
> {
    V1(C0),
    V2(C1),
    V3(C2),
}

impl<C0, C1, C2> Combinator<[u8], Vec<u8>>
for SeverHelloExtensionExtensionDataDispatchCase0<C0, C1, C2>
where
    for<'p, 's> C0: Combinator<
        [u8],
        Vec<u8>,
        Type<'p> = PreSharedKeyServerExtension,
        SType<'s> = PreSharedKeyServerExtension,
        GType = PreSharedKeyServerExtension,
    >,
    for<'p, 's> C1: Combinator<
        [u8],
        Vec<u8>,
        Type<'p> = SupportedVersionsServer,
        SType<'s> = SupportedVersionsServer,
        GType = SupportedVersionsServer,
    >,
    for<'p, 's> C2: Combinator<
        [u8],
        Vec<u8>,
        Type<'p> = KeyShareEntry<'p>,
        SType<'s> = &'s KeyShareEntry<'s>,
        GType = KeyShareEntryOwned,
    >,
{
    type Type<'p> = SeverHelloExtensionExtensionData<'p>;
    type SType<'s> = &'s SeverHelloExtensionExtensionData<'s>;
    type GType = SeverHelloExtensionExtensionDataOwned;
    fn length<'s>(&self, v: Self::SType<'s>) -> usize
    where
        [u8]: 's,
    {
        match (self, v) {
            (
                SeverHelloExtensionExtensionDataDispatchCase0::V1(inner),
                SeverHelloExtensionExtensionData::PreSharedKey(v0),
            ) => inner.length((*v0)),
            (
                SeverHelloExtensionExtensionDataDispatchCase0::V2(inner),
                SeverHelloExtensionExtensionData::SupportedVersions(v1),
            ) => inner.length((*v1)),
            (
                SeverHelloExtensionExtensionDataDispatchCase0::V3(inner),
                SeverHelloExtensionExtensionData::KeyShare(v2),
            ) => inner.length(&(*v2)),
            _ => panic!("dispatch branch combinator does not match value"),
        }
    }
    fn parse<'p>(&self, s: &'p [u8]) -> Result<(usize, Self::Type<'p>), ParseError>
    where
        [u8]: 'p,
    {
        match self {
            SeverHelloExtensionExtensionDataDispatchCase0::V1(inner) => {
                let (n, v) = inner.parse(s)?;
                Ok((n, SeverHelloExtensionExtensionData::PreSharedKey(v)))
            }
            SeverHelloExtensionExtensionDataDispatchCase0::V2(inner) => {
                let (n, v) = inner.parse(s)?;
                Ok((n, SeverHelloExtensionExtensionData::SupportedVersions(v)))
            }
            SeverHelloExtensionExtensionDataDispatchCase0::V3(inner) => {
                let (n, v) = inner.parse(s)?;
                Ok((n, SeverHelloExtensionExtensionData::KeyShare(v)))
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
            (
                SeverHelloExtensionExtensionDataDispatchCase0::V1(inner),
                SeverHelloExtensionExtensionData::PreSharedKey(v0),
            ) => inner.serialize((*v0), data, pos),
            (
                SeverHelloExtensionExtensionDataDispatchCase0::V2(inner),
                SeverHelloExtensionExtensionData::SupportedVersions(v1),
            ) => inner.serialize((*v1), data, pos),
            (
                SeverHelloExtensionExtensionDataDispatchCase0::V3(inner),
                SeverHelloExtensionExtensionData::KeyShare(v2),
            ) => inner.serialize(&(*v2), data, pos),
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
                SeverHelloExtensionExtensionDataDispatchCase0::V1(inner),
                SeverHelloExtensionExtensionDataOwned::PreSharedKey(v0),
            ) => inner.serialize_gen(v0, data, pos),
            (
                SeverHelloExtensionExtensionDataDispatchCase0::V2(inner),
                SeverHelloExtensionExtensionDataOwned::SupportedVersions(v1),
            ) => inner.serialize_gen(v1, data, pos),
            (
                SeverHelloExtensionExtensionDataDispatchCase0::V3(inner),
                SeverHelloExtensionExtensionDataOwned::KeyShare(v2),
            ) => inner.serialize_gen(v2, data, pos),
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
            SeverHelloExtensionExtensionDataDispatchCase0::V1(inner) => {
                let (n, v) = inner.generate(g)?;
                Ok((n, SeverHelloExtensionExtensionDataOwned::PreSharedKey(v)))
            }
            SeverHelloExtensionExtensionDataDispatchCase0::V2(inner) => {
                let (n, v) = inner.generate(g)?;
                Ok((n, SeverHelloExtensionExtensionDataOwned::SupportedVersions(v)))
            }
            SeverHelloExtensionExtensionDataDispatchCase0::V3(inner) => {
                let (n, v) = inner.generate(g)?;
                Ok((n, SeverHelloExtensionExtensionDataOwned::KeyShare(v)))
            }
        }
    }
    fn well_formed<'s>(&self, v: Self::SType<'s>) -> bool
    where
        [u8]: 's,
    {
        match (self, v) {
            (
                SeverHelloExtensionExtensionDataDispatchCase0::V1(inner),
                SeverHelloExtensionExtensionData::PreSharedKey(v0),
            ) => inner.well_formed((*v0)),
            (
                SeverHelloExtensionExtensionDataDispatchCase0::V2(inner),
                SeverHelloExtensionExtensionData::SupportedVersions(v1),
            ) => inner.well_formed((*v1)),
            (
                SeverHelloExtensionExtensionDataDispatchCase0::V3(inner),
                SeverHelloExtensionExtensionData::KeyShare(v2),
            ) => inner.well_formed(&(*v2)),
            _ => false,
        }
    }
}

#[derive(Clone, Copy)]
pub struct SeverHelloExtensionDep {}
impl DepCombinator<(ExtensionTypeCombinator, U16Be), [u8], Vec<u8>>
for SeverHelloExtensionDep {
    type Out = SeverHelloExtensionExtensionDataCombinator;
    type OutGen<'g> = SeverHelloExtensionExtensionDataCombinator<
        SeverHelloExtensionExtensionDataCombinatorAlias<'g>,
    >;
    fn dep_snd<'s>(&self, fst: (ExtensionType, u16)) -> Self::Out {
        let fst: (ExtensionType, u16) = fst;
        let (extension_type, ext_len) = fst;
        sever_hello_extension_extension_data(ext_len, extension_type)
    }
    fn dep_snd_gen<'g>(&self, fst: &'g mut (ExtensionType, u16)) -> Self::OutGen<'g> {
        let fst: &'g mut (ExtensionType, u16) = fst;
        let (extension_type, ext_len) = fst;
        sever_hello_extension_extension_data(ext_len, extension_type)
    }
}

#[derive(Clone, Copy)]
pub struct ServerExtensionsDep {}
impl DepCombinator<Refined<U16Be, fn(u16) -> bool>, [u8], Vec<u8>>
for ServerExtensionsDep {
    type Out = FixedLen<'static, Repeat<SeverHelloExtensionCombinator>>;
    type OutGen<'g> = FixedLen<'g, Repeat<SeverHelloExtensionCombinator>>;
    fn dep_snd<'s>(&self, fst: u16) -> Self::Out {
        let fst: u16 = fst;
        let l = fst;
        FixedLen(Length::from_value(l as usize), Repeat::new(sever_hello_extension()))
    }
    fn dep_snd_gen<'g>(&self, fst: &'g mut u16) -> Self::OutGen<'g> {
        let fst: &'g mut u16 = fst;
        let l = fst;
        FixedLen(Length::from_u16_mut(l), Repeat::new(sever_hello_extension()))
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

pub enum CertificateEntryDataDispatchCase<
    C0 = Opaque1FfffffCombinator,
    C1 = Opaque1FfffffCombinator,
> {
    V1(C0),
    V2(C1),
}

impl<C0, C1> Combinator<[u8], Vec<u8>> for CertificateEntryDataDispatchCase<C0, C1>
where
    for<'p, 's> C0: Combinator<
        [u8],
        Vec<u8>,
        Type<'p> = Opaque1Ffffff<'p>,
        SType<'s> = &'s Opaque1Ffffff<'s>,
        GType = Opaque1FfffffOwned,
    >,
    for<'p, 's> C1: Combinator<
        [u8],
        Vec<u8>,
        Type<'p> = Opaque1Ffffff<'p>,
        SType<'s> = &'s Opaque1Ffffff<'s>,
        GType = Opaque1FfffffOwned,
    >,
{
    type Type<'p> = CertificateEntryData<'p>;
    type SType<'s> = &'s CertificateEntryData<'s>;
    type GType = CertificateEntryDataOwned;
    fn length<'s>(&self, v: Self::SType<'s>) -> usize
    where
        [u8]: 's,
    {
        match (self, v) {
            (
                CertificateEntryDataDispatchCase::V1(inner),
                CertificateEntryData::X509(v0),
            ) => inner.length(&(*v0)),
            (
                CertificateEntryDataDispatchCase::V2(inner),
                CertificateEntryData::RawPublicKey(v1),
            ) => inner.length(&(*v1)),
            _ => panic!("dispatch branch combinator does not match value"),
        }
    }
    fn parse<'p>(&self, s: &'p [u8]) -> Result<(usize, Self::Type<'p>), ParseError>
    where
        [u8]: 'p,
    {
        match self {
            CertificateEntryDataDispatchCase::V1(inner) => {
                let (n, v) = inner.parse(s)?;
                Ok((n, CertificateEntryData::X509(v)))
            }
            CertificateEntryDataDispatchCase::V2(inner) => {
                let (n, v) = inner.parse(s)?;
                Ok((n, CertificateEntryData::RawPublicKey(v)))
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
            (
                CertificateEntryDataDispatchCase::V1(inner),
                CertificateEntryData::X509(v0),
            ) => inner.serialize(&(*v0), data, pos),
            (
                CertificateEntryDataDispatchCase::V2(inner),
                CertificateEntryData::RawPublicKey(v1),
            ) => inner.serialize(&(*v1), data, pos),
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
                CertificateEntryDataDispatchCase::V1(inner),
                CertificateEntryDataOwned::X509(v0),
            ) => inner.serialize_gen(v0, data, pos),
            (
                CertificateEntryDataDispatchCase::V2(inner),
                CertificateEntryDataOwned::RawPublicKey(v1),
            ) => inner.serialize_gen(v1, data, pos),
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
            CertificateEntryDataDispatchCase::V1(inner) => {
                let (n, v) = inner.generate(g)?;
                Ok((n, CertificateEntryDataOwned::X509(v)))
            }
            CertificateEntryDataDispatchCase::V2(inner) => {
                let (n, v) = inner.generate(g)?;
                Ok((n, CertificateEntryDataOwned::RawPublicKey(v)))
            }
        }
    }
    fn well_formed<'s>(&self, v: Self::SType<'s>) -> bool
    where
        [u8]: 's,
    {
        match (self, v) {
            (
                CertificateEntryDataDispatchCase::V1(inner),
                CertificateEntryData::X509(v0),
            ) => inner.well_formed(&(*v0)),
            (
                CertificateEntryDataDispatchCase::V2(inner),
                CertificateEntryData::RawPublicKey(v1),
            ) => inner.well_formed(&(*v1)),
            _ => false,
        }
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
        Variable(l.as_u32() as usize)
    }
    fn dep_snd_gen<'g>(&self, fst: &'g mut u24) -> Self::OutGen<'g> {
        let fst: &'g mut u24 = fst;
        let l = fst;
        Variable(*l.as_u32() as usize)
    }
}

///Parse function for alert_level combinator
pub fn parse_alert_level<'p>(
    input: &'p [u8],
) -> Result<(usize, AlertLevel), ParseError> {
    let combinator = alert_level();
    combinator.parse(input)
}

///Serialize function for alert_level combinator
pub fn serialize_alert_level<'s>(
    v: AlertLevel,
    data: &mut Vec<u8>,
    pos: usize,
) -> Result<usize, SerializeError> {
    let combinator = alert_level();
    combinator.serialize(v, data, pos)
}

///Serialize function for alert_level combinator (owned version)
pub fn serialize_gen_alert_level(
    v: AlertLevel,
    data: &mut Vec<u8>,
    pos: usize,
) -> Result<usize, SerializeError> {
    let combinator = alert_level();
    combinator.serialize_gen(v, data, pos)
}

///Length function for alert_level combinator
pub fn alert_level_len<'s>(v: AlertLevel) -> usize {
    let combinator = alert_level();
    combinator.length(v)
}

///Generate function for alert_level combinator
pub fn generate_alert_level(g: &mut GenSt) -> GResult<AlertLevel, GenerateError> {
    let mut combinator = alert_level();
    combinator.generate(g)
}

///Parse function for empty combinator
pub fn parse_empty<'p>(input: &'p [u8]) -> Result<(usize, Empty<'p>), ParseError> {
    let combinator = empty();
    combinator.parse(input)
}

///Serialize function for empty combinator
pub fn serialize_empty<'s>(
    v: &'s [u8],
    data: &mut Vec<u8>,
    pos: usize,
) -> Result<usize, SerializeError> {
    let combinator = empty();
    combinator.serialize(v, data, pos)
}

///Serialize function for empty combinator (owned version)
pub fn serialize_gen_empty(
    v: EmptyOwned,
    data: &mut Vec<u8>,
    pos: usize,
) -> Result<usize, SerializeError> {
    let combinator = empty();
    combinator.serialize_gen(v, data, pos)
}

///Length function for empty combinator
pub fn empty_len<'s>(v: &'s [u8]) -> usize {
    let combinator = empty();
    combinator.length(v)
}

///Generate function for empty combinator
pub fn generate_empty(g: &mut GenSt) -> GResult<EmptyOwned, GenerateError> {
    let mut combinator = empty();
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

///Parse function for extension_type combinator
pub fn parse_extension_type<'p>(
    input: &'p [u8],
) -> Result<(usize, ExtensionType), ParseError> {
    let combinator = extension_type();
    combinator.parse(input)
}

///Serialize function for extension_type combinator
pub fn serialize_extension_type<'s>(
    v: u16,
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
pub fn extension_type_len<'s>(v: u16) -> usize {
    let combinator = extension_type();
    combinator.length(v)
}

///Generate function for extension_type combinator
pub fn generate_extension_type(g: &mut GenSt) -> GResult<ExtensionType, GenerateError> {
    let mut combinator = extension_type();
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
    v: u16,
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
pub fn signature_scheme_len<'s>(v: u16) -> usize {
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

///Parse function for distinguished_name combinator
pub fn parse_distinguished_name<'p>(
    input: &'p [u8],
) -> Result<(usize, DistinguishedName<'p>), ParseError> {
    let combinator = distinguished_name();
    combinator.parse(input)
}

///Serialize function for distinguished_name combinator
pub fn serialize_distinguished_name<'s>(
    v: &'s Opaque1Ffff<'s>,
    data: &mut Vec<u8>,
    pos: usize,
) -> Result<usize, SerializeError> {
    let combinator = distinguished_name();
    combinator.serialize(v, data, pos)
}

///Serialize function for distinguished_name combinator (owned version)
pub fn serialize_gen_distinguished_name(
    v: DistinguishedNameOwned,
    data: &mut Vec<u8>,
    pos: usize,
) -> Result<usize, SerializeError> {
    let combinator = distinguished_name();
    combinator.serialize_gen(v, data, pos)
}

///Length function for distinguished_name combinator
pub fn distinguished_name_len<'s>(v: &'s Opaque1Ffff<'s>) -> usize {
    let combinator = distinguished_name();
    combinator.length(v)
}

///Generate function for distinguished_name combinator
pub fn generate_distinguished_name(
    g: &mut GenSt,
) -> GResult<DistinguishedNameOwned, GenerateError> {
    let mut combinator = distinguished_name();
    combinator.generate(g)
}

///Parse function for certificate_authorities_extension combinator
pub fn parse_certificate_authorities_extension<'p>(
    input: &'p [u8],
) -> Result<(usize, CertificateAuthoritiesExtension<'p>), ParseError> {
    let combinator = certificate_authorities_extension();
    combinator.parse(input)
}

///Serialize function for certificate_authorities_extension combinator
pub fn serialize_certificate_authorities_extension<'s>(
    v: &'s CertificateAuthoritiesExtension<'s>,
    data: &mut Vec<u8>,
    pos: usize,
) -> Result<usize, SerializeError> {
    let combinator = certificate_authorities_extension();
    combinator.serialize(v, data, pos)
}

///Serialize function for certificate_authorities_extension combinator (owned version)
pub fn serialize_gen_certificate_authorities_extension(
    v: CertificateAuthoritiesExtensionOwned,
    data: &mut Vec<u8>,
    pos: usize,
) -> Result<usize, SerializeError> {
    let combinator = certificate_authorities_extension();
    combinator.serialize_gen(v, data, pos)
}

///Length function for certificate_authorities_extension combinator
pub fn certificate_authorities_extension_len<'s>(
    v: &'s CertificateAuthoritiesExtension<'s>,
) -> usize {
    let combinator = certificate_authorities_extension();
    combinator.length(v)
}

///Generate function for certificate_authorities_extension combinator
pub fn generate_certificate_authorities_extension(
    g: &mut GenSt,
) -> GResult<CertificateAuthoritiesExtensionOwned, GenerateError> {
    let mut combinator = certificate_authorities_extension();
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

///Parse function for oid_filter combinator
pub fn parse_oid_filter<'p>(
    input: &'p [u8],
) -> Result<(usize, OidFilter<'p>), ParseError> {
    let combinator = oid_filter();
    combinator.parse(input)
}

///Serialize function for oid_filter combinator
pub fn serialize_oid_filter<'s>(
    v: &'s OidFilter<'s>,
    data: &mut Vec<u8>,
    pos: usize,
) -> Result<usize, SerializeError> {
    let combinator = oid_filter();
    combinator.serialize(v, data, pos)
}

///Serialize function for oid_filter combinator (owned version)
pub fn serialize_gen_oid_filter(
    v: OidFilterOwned,
    data: &mut Vec<u8>,
    pos: usize,
) -> Result<usize, SerializeError> {
    let combinator = oid_filter();
    combinator.serialize_gen(v, data, pos)
}

///Length function for oid_filter combinator
pub fn oid_filter_len<'s>(v: &'s OidFilter<'s>) -> usize {
    let combinator = oid_filter();
    combinator.length(v)
}

///Generate function for oid_filter combinator
pub fn generate_oid_filter(g: &mut GenSt) -> GResult<OidFilterOwned, GenerateError> {
    let mut combinator = oid_filter();
    combinator.generate(g)
}

///Parse function for oid_filter_extension combinator
pub fn parse_oid_filter_extension<'p>(
    input: &'p [u8],
) -> Result<(usize, OidFilterExtension<'p>), ParseError> {
    let combinator = oid_filter_extension();
    combinator.parse(input)
}

///Serialize function for oid_filter_extension combinator
pub fn serialize_oid_filter_extension<'s>(
    v: &'s OidFilterExtension<'s>,
    data: &mut Vec<u8>,
    pos: usize,
) -> Result<usize, SerializeError> {
    let combinator = oid_filter_extension();
    combinator.serialize(v, data, pos)
}

///Serialize function for oid_filter_extension combinator (owned version)
pub fn serialize_gen_oid_filter_extension(
    v: OidFilterExtensionOwned,
    data: &mut Vec<u8>,
    pos: usize,
) -> Result<usize, SerializeError> {
    let combinator = oid_filter_extension();
    combinator.serialize_gen(v, data, pos)
}

///Length function for oid_filter_extension combinator
pub fn oid_filter_extension_len<'s>(v: &'s OidFilterExtension<'s>) -> usize {
    let combinator = oid_filter_extension();
    combinator.length(v)
}

///Generate function for oid_filter_extension combinator
pub fn generate_oid_filter_extension(
    g: &mut GenSt,
) -> GResult<OidFilterExtensionOwned, GenerateError> {
    let mut combinator = oid_filter_extension();
    combinator.generate(g)
}

///Parse function for certificate_request_extension_extension_data combinator
pub fn parse_certificate_request_extension_extension_data<'p>(
    input: &'p [u8],
    ext_len: u16,
    extension_type: u16,
) -> Result<(usize, CertificateRequestExtensionExtensionData<'p>), ParseError> {
    let combinator = certificate_request_extension_extension_data(
        ext_len,
        extension_type,
    );
    combinator.parse(input)
}

///Serialize function for certificate_request_extension_extension_data combinator
pub fn serialize_certificate_request_extension_extension_data<'s>(
    v: &'s CertificateRequestExtensionExtensionData<'s>,
    data: &mut Vec<u8>,
    pos: usize,
    ext_len: u16,
    extension_type: u16,
) -> Result<usize, SerializeError> {
    let combinator = certificate_request_extension_extension_data(
        ext_len,
        extension_type,
    );
    combinator.serialize(v, data, pos)
}

///Serialize function for certificate_request_extension_extension_data combinator (owned version)
pub fn serialize_gen_certificate_request_extension_extension_data(
    v: CertificateRequestExtensionExtensionDataOwned,
    data: &mut Vec<u8>,
    pos: usize,
    ext_len: u16,
    extension_type: u16,
) -> Result<usize, SerializeError> {
    let combinator = certificate_request_extension_extension_data(
        ext_len,
        extension_type,
    );
    combinator.serialize_gen(v, data, pos)
}

///Length function for certificate_request_extension_extension_data combinator
pub fn certificate_request_extension_extension_data_len<'s>(
    v: &'s CertificateRequestExtensionExtensionData<'s>,
    ext_len: u16,
    extension_type: u16,
) -> usize {
    let combinator = certificate_request_extension_extension_data(
        ext_len,
        extension_type,
    );
    combinator.length(v)
}

///Generate function for certificate_request_extension_extension_data combinator
pub fn generate_certificate_request_extension_extension_data<'g>(
    g: &mut GenSt,
    ext_len: &'g mut u16,
    extension_type: &'g mut u16,
) -> GResult<CertificateRequestExtensionExtensionDataOwned, GenerateError> {
    let mut combinator = certificate_request_extension_extension_data(
        ext_len,
        extension_type,
    );
    combinator.generate(g)
}

///Parse function for certificate_request_extension combinator
pub fn parse_certificate_request_extension<'p>(
    input: &'p [u8],
) -> Result<(usize, CertificateRequestExtension<'p>), ParseError> {
    let combinator = certificate_request_extension();
    combinator.parse(input)
}

///Serialize function for certificate_request_extension combinator
pub fn serialize_certificate_request_extension<'s>(
    v: &'s CertificateRequestExtension<'s>,
    data: &mut Vec<u8>,
    pos: usize,
) -> Result<usize, SerializeError> {
    let combinator = certificate_request_extension();
    combinator.serialize(v, data, pos)
}

///Serialize function for certificate_request_extension combinator (owned version)
pub fn serialize_gen_certificate_request_extension(
    v: CertificateRequestExtensionOwned,
    data: &mut Vec<u8>,
    pos: usize,
) -> Result<usize, SerializeError> {
    let combinator = certificate_request_extension();
    combinator.serialize_gen(v, data, pos)
}

///Length function for certificate_request_extension combinator
pub fn certificate_request_extension_len<'s>(
    v: &'s CertificateRequestExtension<'s>,
) -> usize {
    let combinator = certificate_request_extension();
    combinator.length(v)
}

///Generate function for certificate_request_extension combinator
pub fn generate_certificate_request_extension(
    g: &mut GenSt,
) -> GResult<CertificateRequestExtensionOwned, GenerateError> {
    let mut combinator = certificate_request_extension();
    combinator.generate(g)
}

///Parse function for name_type combinator
pub fn parse_name_type<'p>(input: &'p [u8]) -> Result<(usize, NameType), ParseError> {
    let combinator = name_type();
    combinator.parse(input)
}

///Serialize function for name_type combinator
pub fn serialize_name_type<'s>(
    v: u8,
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
pub fn name_type_len<'s>(v: u8) -> usize {
    let combinator = name_type();
    combinator.length(v)
}

///Generate function for name_type combinator
pub fn generate_name_type(g: &mut GenSt) -> GResult<NameType, GenerateError> {
    let mut combinator = name_type();
    combinator.generate(g)
}

///Parse function for sh_or_hrr combinator
pub fn parse_sh_or_hrr<'p>(input: &'p [u8]) -> Result<(usize, ShOrHrr<'p>), ParseError> {
    let combinator = sh_or_hrr();
    combinator.parse(input)
}

///Serialize function for sh_or_hrr combinator
pub fn serialize_sh_or_hrr<'s>(
    v: &'s ShOrHrr<'s>,
    data: &mut Vec<u8>,
    pos: usize,
) -> Result<usize, SerializeError> {
    let combinator = sh_or_hrr();
    combinator.serialize(v, data, pos)
}

///Serialize function for sh_or_hrr combinator (owned version)
pub fn serialize_gen_sh_or_hrr(
    v: ShOrHrrOwned,
    data: &mut Vec<u8>,
    pos: usize,
) -> Result<usize, SerializeError> {
    let combinator = sh_or_hrr();
    combinator.serialize_gen(v, data, pos)
}

///Length function for sh_or_hrr combinator
pub fn sh_or_hrr_len<'s>(v: &'s ShOrHrr<'s>) -> usize {
    let combinator = sh_or_hrr();
    combinator.length(v)
}

///Generate function for sh_or_hrr combinator
pub fn generate_sh_or_hrr(g: &mut GenSt) -> GResult<ShOrHrrOwned, GenerateError> {
    let mut combinator = sh_or_hrr();
    combinator.generate(g)
}

///Parse function for handshake_type combinator
pub fn parse_handshake_type<'p>(
    input: &'p [u8],
) -> Result<(usize, HandshakeType), ParseError> {
    let combinator = handshake_type();
    combinator.parse(input)
}

///Serialize function for handshake_type combinator
pub fn serialize_handshake_type<'s>(
    v: HandshakeType,
    data: &mut Vec<u8>,
    pos: usize,
) -> Result<usize, SerializeError> {
    let combinator = handshake_type();
    combinator.serialize(v, data, pos)
}

///Serialize function for handshake_type combinator (owned version)
pub fn serialize_gen_handshake_type(
    v: HandshakeType,
    data: &mut Vec<u8>,
    pos: usize,
) -> Result<usize, SerializeError> {
    let combinator = handshake_type();
    combinator.serialize_gen(v, data, pos)
}

///Length function for handshake_type combinator
pub fn handshake_type_len<'s>(v: HandshakeType) -> usize {
    let combinator = handshake_type();
    combinator.length(v)
}

///Generate function for handshake_type combinator
pub fn generate_handshake_type(g: &mut GenSt) -> GResult<HandshakeType, GenerateError> {
    let mut combinator = handshake_type();
    combinator.generate(g)
}

///Parse function for session_id combinator
pub fn parse_session_id<'p>(
    input: &'p [u8],
) -> Result<(usize, SessionId<'p>), ParseError> {
    let combinator = session_id();
    combinator.parse(input)
}

///Serialize function for session_id combinator
pub fn serialize_session_id<'s>(
    v: &'s SessionId<'s>,
    data: &mut Vec<u8>,
    pos: usize,
) -> Result<usize, SerializeError> {
    let combinator = session_id();
    combinator.serialize(v, data, pos)
}

///Serialize function for session_id combinator (owned version)
pub fn serialize_gen_session_id(
    v: SessionIdOwned,
    data: &mut Vec<u8>,
    pos: usize,
) -> Result<usize, SerializeError> {
    let combinator = session_id();
    combinator.serialize_gen(v, data, pos)
}

///Length function for session_id combinator
pub fn session_id_len<'s>(v: &'s SessionId<'s>) -> usize {
    let combinator = session_id();
    combinator.length(v)
}

///Generate function for session_id combinator
pub fn generate_session_id(g: &mut GenSt) -> GResult<SessionIdOwned, GenerateError> {
    let mut combinator = session_id();
    combinator.generate(g)
}

///Parse function for cipher_suite combinator
pub fn parse_cipher_suite<'p>(
    input: &'p [u8],
) -> Result<(usize, CipherSuite), ParseError> {
    let combinator = cipher_suite();
    combinator.parse(input)
}

///Serialize function for cipher_suite combinator
pub fn serialize_cipher_suite<'s>(
    v: u16,
    data: &mut Vec<u8>,
    pos: usize,
) -> Result<usize, SerializeError> {
    let combinator = cipher_suite();
    combinator.serialize(v, data, pos)
}

///Serialize function for cipher_suite combinator (owned version)
pub fn serialize_gen_cipher_suite(
    v: CipherSuite,
    data: &mut Vec<u8>,
    pos: usize,
) -> Result<usize, SerializeError> {
    let combinator = cipher_suite();
    combinator.serialize_gen(v, data, pos)
}

///Length function for cipher_suite combinator
pub fn cipher_suite_len<'s>(v: u16) -> usize {
    let combinator = cipher_suite();
    combinator.length(v)
}

///Generate function for cipher_suite combinator
pub fn generate_cipher_suite(g: &mut GenSt) -> GResult<CipherSuite, GenerateError> {
    let mut combinator = cipher_suite();
    combinator.generate(g)
}

///Parse function for cipher_suite_list combinator
pub fn parse_cipher_suite_list<'p>(
    input: &'p [u8],
) -> Result<(usize, CipherSuiteList), ParseError> {
    let combinator = cipher_suite_list();
    combinator.parse(input)
}

///Serialize function for cipher_suite_list combinator
pub fn serialize_cipher_suite_list<'s>(
    v: &'s CipherSuiteList,
    data: &mut Vec<u8>,
    pos: usize,
) -> Result<usize, SerializeError> {
    let combinator = cipher_suite_list();
    combinator.serialize(v, data, pos)
}

///Serialize function for cipher_suite_list combinator (owned version)
pub fn serialize_gen_cipher_suite_list(
    v: CipherSuiteList,
    data: &mut Vec<u8>,
    pos: usize,
) -> Result<usize, SerializeError> {
    let combinator = cipher_suite_list();
    combinator.serialize_gen(v, data, pos)
}

///Length function for cipher_suite_list combinator
pub fn cipher_suite_list_len<'s>(v: &'s CipherSuiteList) -> usize {
    let combinator = cipher_suite_list();
    combinator.length(v)
}

///Generate function for cipher_suite_list combinator
pub fn generate_cipher_suite_list(
    g: &mut GenSt,
) -> GResult<CipherSuiteList, GenerateError> {
    let mut combinator = cipher_suite_list();
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
    name_type: u8,
) -> Result<(usize, ServerNameName<'p>), ParseError> {
    let combinator = server_name_name(name_type);
    combinator.parse(input)
}

///Serialize function for server_name_name combinator
pub fn serialize_server_name_name<'s>(
    v: &'s ServerNameName<'s>,
    data: &mut Vec<u8>,
    pos: usize,
    name_type: u8,
) -> Result<usize, SerializeError> {
    let combinator = server_name_name(name_type);
    combinator.serialize(v, data, pos)
}

///Serialize function for server_name_name combinator (owned version)
pub fn serialize_gen_server_name_name(
    v: ServerNameNameOwned,
    data: &mut Vec<u8>,
    pos: usize,
    name_type: u8,
) -> Result<usize, SerializeError> {
    let combinator = server_name_name(name_type);
    combinator.serialize_gen(v, data, pos)
}

///Length function for server_name_name combinator
pub fn server_name_name_len<'s>(v: &'s ServerNameName<'s>, name_type: u8) -> usize {
    let combinator = server_name_name(name_type);
    combinator.length(v)
}

///Generate function for server_name_name combinator
pub fn generate_server_name_name<'g>(
    g: &mut GenSt,
    name_type: &'g mut u8,
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

///Parse function for named_group combinator
pub fn parse_named_group<'p>(
    input: &'p [u8],
) -> Result<(usize, NamedGroup), ParseError> {
    let combinator = named_group();
    combinator.parse(input)
}

///Serialize function for named_group combinator
pub fn serialize_named_group<'s>(
    v: u16,
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
pub fn named_group_len<'s>(v: u16) -> usize {
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

///Parse function for protocol_version combinator
pub fn parse_protocol_version<'p>(
    input: &'p [u8],
) -> Result<(usize, ProtocolVersion), ParseError> {
    let combinator = protocol_version();
    combinator.parse(input)
}

///Serialize function for protocol_version combinator
pub fn serialize_protocol_version<'s>(
    v: u16,
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
pub fn protocol_version_len<'s>(v: u16) -> usize {
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

///Parse function for supported_versions_client combinator
pub fn parse_supported_versions_client<'p>(
    input: &'p [u8],
) -> Result<(usize, SupportedVersionsClient), ParseError> {
    let combinator = supported_versions_client();
    combinator.parse(input)
}

///Serialize function for supported_versions_client combinator
pub fn serialize_supported_versions_client<'s>(
    v: &'s SupportedVersionsClient,
    data: &mut Vec<u8>,
    pos: usize,
) -> Result<usize, SerializeError> {
    let combinator = supported_versions_client();
    combinator.serialize(v, data, pos)
}

///Serialize function for supported_versions_client combinator (owned version)
pub fn serialize_gen_supported_versions_client(
    v: SupportedVersionsClient,
    data: &mut Vec<u8>,
    pos: usize,
) -> Result<usize, SerializeError> {
    let combinator = supported_versions_client();
    combinator.serialize_gen(v, data, pos)
}

///Length function for supported_versions_client combinator
pub fn supported_versions_client_len<'s>(v: &'s SupportedVersionsClient) -> usize {
    let combinator = supported_versions_client();
    combinator.length(v)
}

///Generate function for supported_versions_client combinator
pub fn generate_supported_versions_client(
    g: &mut GenSt,
) -> GResult<SupportedVersionsClient, GenerateError> {
    let mut combinator = supported_versions_client();
    combinator.generate(g)
}

///Parse function for key_share_entry combinator
pub fn parse_key_share_entry<'p>(
    input: &'p [u8],
) -> Result<(usize, KeyShareEntry<'p>), ParseError> {
    let combinator = key_share_entry();
    combinator.parse(input)
}

///Serialize function for key_share_entry combinator
pub fn serialize_key_share_entry<'s>(
    v: &'s KeyShareEntry<'s>,
    data: &mut Vec<u8>,
    pos: usize,
) -> Result<usize, SerializeError> {
    let combinator = key_share_entry();
    combinator.serialize(v, data, pos)
}

///Serialize function for key_share_entry combinator (owned version)
pub fn serialize_gen_key_share_entry(
    v: KeyShareEntryOwned,
    data: &mut Vec<u8>,
    pos: usize,
) -> Result<usize, SerializeError> {
    let combinator = key_share_entry();
    combinator.serialize_gen(v, data, pos)
}

///Length function for key_share_entry combinator
pub fn key_share_entry_len<'s>(v: &'s KeyShareEntry<'s>) -> usize {
    let combinator = key_share_entry();
    combinator.length(v)
}

///Generate function for key_share_entry combinator
pub fn generate_key_share_entry(
    g: &mut GenSt,
) -> GResult<KeyShareEntryOwned, GenerateError> {
    let mut combinator = key_share_entry();
    combinator.generate(g)
}

///Parse function for key_share_client_hello combinator
pub fn parse_key_share_client_hello<'p>(
    input: &'p [u8],
) -> Result<(usize, KeyShareClientHello<'p>), ParseError> {
    let combinator = key_share_client_hello();
    combinator.parse(input)
}

///Serialize function for key_share_client_hello combinator
pub fn serialize_key_share_client_hello<'s>(
    v: &'s KeyShareClientHello<'s>,
    data: &mut Vec<u8>,
    pos: usize,
) -> Result<usize, SerializeError> {
    let combinator = key_share_client_hello();
    combinator.serialize(v, data, pos)
}

///Serialize function for key_share_client_hello combinator (owned version)
pub fn serialize_gen_key_share_client_hello(
    v: KeyShareClientHelloOwned,
    data: &mut Vec<u8>,
    pos: usize,
) -> Result<usize, SerializeError> {
    let combinator = key_share_client_hello();
    combinator.serialize_gen(v, data, pos)
}

///Length function for key_share_client_hello combinator
pub fn key_share_client_hello_len<'s>(v: &'s KeyShareClientHello<'s>) -> usize {
    let combinator = key_share_client_hello();
    combinator.length(v)
}

///Generate function for key_share_client_hello combinator
pub fn generate_key_share_client_hello(
    g: &mut GenSt,
) -> GResult<KeyShareClientHelloOwned, GenerateError> {
    let mut combinator = key_share_client_hello();
    combinator.generate(g)
}

///Parse function for psk_key_exchange_mode combinator
pub fn parse_psk_key_exchange_mode<'p>(
    input: &'p [u8],
) -> Result<(usize, PskKeyExchangeMode), ParseError> {
    let combinator = psk_key_exchange_mode();
    combinator.parse(input)
}

///Serialize function for psk_key_exchange_mode combinator
pub fn serialize_psk_key_exchange_mode<'s>(
    v: u8,
    data: &mut Vec<u8>,
    pos: usize,
) -> Result<usize, SerializeError> {
    let combinator = psk_key_exchange_mode();
    combinator.serialize(v, data, pos)
}

///Serialize function for psk_key_exchange_mode combinator (owned version)
pub fn serialize_gen_psk_key_exchange_mode(
    v: PskKeyExchangeMode,
    data: &mut Vec<u8>,
    pos: usize,
) -> Result<usize, SerializeError> {
    let combinator = psk_key_exchange_mode();
    combinator.serialize_gen(v, data, pos)
}

///Length function for psk_key_exchange_mode combinator
pub fn psk_key_exchange_mode_len<'s>(v: u8) -> usize {
    let combinator = psk_key_exchange_mode();
    combinator.length(v)
}

///Generate function for psk_key_exchange_mode combinator
pub fn generate_psk_key_exchange_mode(
    g: &mut GenSt,
) -> GResult<PskKeyExchangeMode, GenerateError> {
    let mut combinator = psk_key_exchange_mode();
    combinator.generate(g)
}

///Parse function for psk_key_exchange_modes combinator
pub fn parse_psk_key_exchange_modes<'p>(
    input: &'p [u8],
) -> Result<(usize, PskKeyExchangeModes), ParseError> {
    let combinator = psk_key_exchange_modes();
    combinator.parse(input)
}

///Serialize function for psk_key_exchange_modes combinator
pub fn serialize_psk_key_exchange_modes<'s>(
    v: &'s PskKeyExchangeModes,
    data: &mut Vec<u8>,
    pos: usize,
) -> Result<usize, SerializeError> {
    let combinator = psk_key_exchange_modes();
    combinator.serialize(v, data, pos)
}

///Serialize function for psk_key_exchange_modes combinator (owned version)
pub fn serialize_gen_psk_key_exchange_modes(
    v: PskKeyExchangeModes,
    data: &mut Vec<u8>,
    pos: usize,
) -> Result<usize, SerializeError> {
    let combinator = psk_key_exchange_modes();
    combinator.serialize_gen(v, data, pos)
}

///Length function for psk_key_exchange_modes combinator
pub fn psk_key_exchange_modes_len<'s>(v: &'s PskKeyExchangeModes) -> usize {
    let combinator = psk_key_exchange_modes();
    combinator.length(v)
}

///Generate function for psk_key_exchange_modes combinator
pub fn generate_psk_key_exchange_modes(
    g: &mut GenSt,
) -> GResult<PskKeyExchangeModes, GenerateError> {
    let mut combinator = psk_key_exchange_modes();
    combinator.generate(g)
}

///Parse function for psk_identity combinator
pub fn parse_psk_identity<'p>(
    input: &'p [u8],
) -> Result<(usize, PskIdentity<'p>), ParseError> {
    let combinator = psk_identity();
    combinator.parse(input)
}

///Serialize function for psk_identity combinator
pub fn serialize_psk_identity<'s>(
    v: &'s PskIdentity<'s>,
    data: &mut Vec<u8>,
    pos: usize,
) -> Result<usize, SerializeError> {
    let combinator = psk_identity();
    combinator.serialize(v, data, pos)
}

///Serialize function for psk_identity combinator (owned version)
pub fn serialize_gen_psk_identity(
    v: PskIdentityOwned,
    data: &mut Vec<u8>,
    pos: usize,
) -> Result<usize, SerializeError> {
    let combinator = psk_identity();
    combinator.serialize_gen(v, data, pos)
}

///Length function for psk_identity combinator
pub fn psk_identity_len<'s>(v: &'s PskIdentity<'s>) -> usize {
    let combinator = psk_identity();
    combinator.length(v)
}

///Generate function for psk_identity combinator
pub fn generate_psk_identity(g: &mut GenSt) -> GResult<PskIdentityOwned, GenerateError> {
    let mut combinator = psk_identity();
    combinator.generate(g)
}

///Parse function for psk_identities combinator
pub fn parse_psk_identities<'p>(
    input: &'p [u8],
) -> Result<(usize, PskIdentities<'p>), ParseError> {
    let combinator = psk_identities();
    combinator.parse(input)
}

///Serialize function for psk_identities combinator
pub fn serialize_psk_identities<'s>(
    v: &'s PskIdentities<'s>,
    data: &mut Vec<u8>,
    pos: usize,
) -> Result<usize, SerializeError> {
    let combinator = psk_identities();
    combinator.serialize(v, data, pos)
}

///Serialize function for psk_identities combinator (owned version)
pub fn serialize_gen_psk_identities(
    v: PskIdentitiesOwned,
    data: &mut Vec<u8>,
    pos: usize,
) -> Result<usize, SerializeError> {
    let combinator = psk_identities();
    combinator.serialize_gen(v, data, pos)
}

///Length function for psk_identities combinator
pub fn psk_identities_len<'s>(v: &'s PskIdentities<'s>) -> usize {
    let combinator = psk_identities();
    combinator.length(v)
}

///Generate function for psk_identities combinator
pub fn generate_psk_identities(
    g: &mut GenSt,
) -> GResult<PskIdentitiesOwned, GenerateError> {
    let mut combinator = psk_identities();
    combinator.generate(g)
}

///Parse function for psk_binder_entry combinator
pub fn parse_psk_binder_entry<'p>(
    input: &'p [u8],
) -> Result<(usize, PskBinderEntry<'p>), ParseError> {
    let combinator = psk_binder_entry();
    combinator.parse(input)
}

///Serialize function for psk_binder_entry combinator
pub fn serialize_psk_binder_entry<'s>(
    v: &'s PskBinderEntry<'s>,
    data: &mut Vec<u8>,
    pos: usize,
) -> Result<usize, SerializeError> {
    let combinator = psk_binder_entry();
    combinator.serialize(v, data, pos)
}

///Serialize function for psk_binder_entry combinator (owned version)
pub fn serialize_gen_psk_binder_entry(
    v: PskBinderEntryOwned,
    data: &mut Vec<u8>,
    pos: usize,
) -> Result<usize, SerializeError> {
    let combinator = psk_binder_entry();
    combinator.serialize_gen(v, data, pos)
}

///Length function for psk_binder_entry combinator
pub fn psk_binder_entry_len<'s>(v: &'s PskBinderEntry<'s>) -> usize {
    let combinator = psk_binder_entry();
    combinator.length(v)
}

///Generate function for psk_binder_entry combinator
pub fn generate_psk_binder_entry(
    g: &mut GenSt,
) -> GResult<PskBinderEntryOwned, GenerateError> {
    let mut combinator = psk_binder_entry();
    combinator.generate(g)
}

///Parse function for psk_binder_entries combinator
pub fn parse_psk_binder_entries<'p>(
    input: &'p [u8],
) -> Result<(usize, PskBinderEntries<'p>), ParseError> {
    let combinator = psk_binder_entries();
    combinator.parse(input)
}

///Serialize function for psk_binder_entries combinator
pub fn serialize_psk_binder_entries<'s>(
    v: &'s PskBinderEntries<'s>,
    data: &mut Vec<u8>,
    pos: usize,
) -> Result<usize, SerializeError> {
    let combinator = psk_binder_entries();
    combinator.serialize(v, data, pos)
}

///Serialize function for psk_binder_entries combinator (owned version)
pub fn serialize_gen_psk_binder_entries(
    v: PskBinderEntriesOwned,
    data: &mut Vec<u8>,
    pos: usize,
) -> Result<usize, SerializeError> {
    let combinator = psk_binder_entries();
    combinator.serialize_gen(v, data, pos)
}

///Length function for psk_binder_entries combinator
pub fn psk_binder_entries_len<'s>(v: &'s PskBinderEntries<'s>) -> usize {
    let combinator = psk_binder_entries();
    combinator.length(v)
}

///Generate function for psk_binder_entries combinator
pub fn generate_psk_binder_entries(
    g: &mut GenSt,
) -> GResult<PskBinderEntriesOwned, GenerateError> {
    let mut combinator = psk_binder_entries();
    combinator.generate(g)
}

///Parse function for offered_psks combinator
pub fn parse_offered_psks<'p>(
    input: &'p [u8],
) -> Result<(usize, OfferedPsks<'p>), ParseError> {
    let combinator = offered_psks();
    combinator.parse(input)
}

///Serialize function for offered_psks combinator
pub fn serialize_offered_psks<'s>(
    v: &'s OfferedPsks<'s>,
    data: &mut Vec<u8>,
    pos: usize,
) -> Result<usize, SerializeError> {
    let combinator = offered_psks();
    combinator.serialize(v, data, pos)
}

///Serialize function for offered_psks combinator (owned version)
pub fn serialize_gen_offered_psks(
    v: OfferedPsksOwned,
    data: &mut Vec<u8>,
    pos: usize,
) -> Result<usize, SerializeError> {
    let combinator = offered_psks();
    combinator.serialize_gen(v, data, pos)
}

///Length function for offered_psks combinator
pub fn offered_psks_len<'s>(v: &'s OfferedPsks<'s>) -> usize {
    let combinator = offered_psks();
    combinator.length(v)
}

///Generate function for offered_psks combinator
pub fn generate_offered_psks(g: &mut GenSt) -> GResult<OfferedPsksOwned, GenerateError> {
    let mut combinator = offered_psks();
    combinator.generate(g)
}

///Parse function for pre_shared_key_client_extension combinator
pub fn parse_pre_shared_key_client_extension<'p>(
    input: &'p [u8],
) -> Result<(usize, PreSharedKeyClientExtension<'p>), ParseError> {
    let combinator = pre_shared_key_client_extension();
    combinator.parse(input)
}

///Serialize function for pre_shared_key_client_extension combinator
pub fn serialize_pre_shared_key_client_extension<'s>(
    v: &'s PreSharedKeyClientExtension<'s>,
    data: &mut Vec<u8>,
    pos: usize,
) -> Result<usize, SerializeError> {
    let combinator = pre_shared_key_client_extension();
    combinator.serialize(v, data, pos)
}

///Serialize function for pre_shared_key_client_extension combinator (owned version)
pub fn serialize_gen_pre_shared_key_client_extension(
    v: PreSharedKeyClientExtensionOwned,
    data: &mut Vec<u8>,
    pos: usize,
) -> Result<usize, SerializeError> {
    let combinator = pre_shared_key_client_extension();
    combinator.serialize_gen(v, data, pos)
}

///Length function for pre_shared_key_client_extension combinator
pub fn pre_shared_key_client_extension_len<'s>(
    v: &'s PreSharedKeyClientExtension<'s>,
) -> usize {
    let combinator = pre_shared_key_client_extension();
    combinator.length(v)
}

///Generate function for pre_shared_key_client_extension combinator
pub fn generate_pre_shared_key_client_extension(
    g: &mut GenSt,
) -> GResult<PreSharedKeyClientExtensionOwned, GenerateError> {
    let mut combinator = pre_shared_key_client_extension();
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
    v: u8,
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
pub fn max_fragment_length_len<'s>(v: u8) -> usize {
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

///Parse function for heartbeat_mode combinator
pub fn parse_heartbeat_mode<'p>(
    input: &'p [u8],
) -> Result<(usize, HeartbeatMode), ParseError> {
    let combinator = heartbeat_mode();
    combinator.parse(input)
}

///Serialize function for heartbeat_mode combinator
pub fn serialize_heartbeat_mode<'s>(
    v: u8,
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
pub fn heartbeat_mode_len<'s>(v: u8) -> usize {
    let combinator = heartbeat_mode();
    combinator.length(v)
}

///Generate function for heartbeat_mode combinator
pub fn generate_heartbeat_mode(g: &mut GenSt) -> GResult<HeartbeatMode, GenerateError> {
    let mut combinator = heartbeat_mode();
    combinator.generate(g)
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
    v: u8,
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
pub fn certificate_type_len<'s>(v: u8) -> usize {
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

///Parse function for cookie combinator
pub fn parse_cookie<'p>(input: &'p [u8]) -> Result<(usize, Cookie<'p>), ParseError> {
    let combinator = cookie();
    combinator.parse(input)
}

///Serialize function for cookie combinator
pub fn serialize_cookie<'s>(
    v: &'s Opaque1Ffff<'s>,
    data: &mut Vec<u8>,
    pos: usize,
) -> Result<usize, SerializeError> {
    let combinator = cookie();
    combinator.serialize(v, data, pos)
}

///Serialize function for cookie combinator (owned version)
pub fn serialize_gen_cookie(
    v: CookieOwned,
    data: &mut Vec<u8>,
    pos: usize,
) -> Result<usize, SerializeError> {
    let combinator = cookie();
    combinator.serialize_gen(v, data, pos)
}

///Length function for cookie combinator
pub fn cookie_len<'s>(v: &'s Opaque1Ffff<'s>) -> usize {
    let combinator = cookie();
    combinator.length(v)
}

///Generate function for cookie combinator
pub fn generate_cookie(g: &mut GenSt) -> GResult<CookieOwned, GenerateError> {
    let mut combinator = cookie();
    combinator.generate(g)
}

///Parse function for client_hello_extension_rest combinator
pub fn parse_client_hello_extension_rest<'p>(
    input: &'p [u8],
    ext_len: u16,
    extension_type: u16,
) -> Result<(usize, ClientHelloExtensionRest<'p>), ParseError> {
    let combinator = client_hello_extension_rest(ext_len, extension_type);
    combinator.parse(input)
}

///Serialize function for client_hello_extension_rest combinator
pub fn serialize_client_hello_extension_rest<'s>(
    v: &'s ClientHelloExtensionRest<'s>,
    data: &mut Vec<u8>,
    pos: usize,
    ext_len: u16,
    extension_type: u16,
) -> Result<usize, SerializeError> {
    let combinator = client_hello_extension_rest(ext_len, extension_type);
    combinator.serialize(v, data, pos)
}

///Serialize function for client_hello_extension_rest combinator (owned version)
pub fn serialize_gen_client_hello_extension_rest(
    v: ClientHelloExtensionRestOwned,
    data: &mut Vec<u8>,
    pos: usize,
    ext_len: u16,
    extension_type: u16,
) -> Result<usize, SerializeError> {
    let combinator = client_hello_extension_rest(ext_len, extension_type);
    combinator.serialize_gen(v, data, pos)
}

///Length function for client_hello_extension_rest combinator
pub fn client_hello_extension_rest_len<'s>(
    v: &'s ClientHelloExtensionRest<'s>,
    ext_len: u16,
    extension_type: u16,
) -> usize {
    let combinator = client_hello_extension_rest(ext_len, extension_type);
    combinator.length(v)
}

///Generate function for client_hello_extension_rest combinator
pub fn generate_client_hello_extension_rest<'g>(
    g: &mut GenSt,
    ext_len: u16,
    extension_type: &'g mut u16,
) -> GResult<ClientHelloExtensionRestOwned, GenerateError> {
    let mut combinator = client_hello_extension_rest(ext_len, extension_type);
    combinator.generate(g)
}

///Parse function for client_hello_extension_extension_data combinator
pub fn parse_client_hello_extension_extension_data<'p>(
    input: &'p [u8],
    ext_len: u16,
    extension_type: u16,
) -> Result<(usize, ClientHelloExtensionExtensionData<'p>), ParseError> {
    let combinator = client_hello_extension_extension_data(ext_len, extension_type);
    combinator.parse(input)
}

///Serialize function for client_hello_extension_extension_data combinator
pub fn serialize_client_hello_extension_extension_data<'s>(
    v: &'s ClientHelloExtensionExtensionData<'s>,
    data: &mut Vec<u8>,
    pos: usize,
    ext_len: u16,
    extension_type: u16,
) -> Result<usize, SerializeError> {
    let combinator = client_hello_extension_extension_data(ext_len, extension_type);
    combinator.serialize(v, data, pos)
}

///Serialize function for client_hello_extension_extension_data combinator (owned version)
pub fn serialize_gen_client_hello_extension_extension_data(
    v: ClientHelloExtensionExtensionDataOwned,
    data: &mut Vec<u8>,
    pos: usize,
    ext_len: u16,
    extension_type: u16,
) -> Result<usize, SerializeError> {
    let combinator = client_hello_extension_extension_data(ext_len, extension_type);
    combinator.serialize_gen(v, data, pos)
}

///Length function for client_hello_extension_extension_data combinator
pub fn client_hello_extension_extension_data_len<'s>(
    v: &'s ClientHelloExtensionExtensionData<'s>,
    ext_len: u16,
    extension_type: u16,
) -> usize {
    let combinator = client_hello_extension_extension_data(ext_len, extension_type);
    combinator.length(v)
}

///Generate function for client_hello_extension_extension_data combinator
pub fn generate_client_hello_extension_extension_data<'g>(
    g: &mut GenSt,
    ext_len: &'g mut u16,
    extension_type: &'g mut u16,
) -> GResult<ClientHelloExtensionExtensionDataOwned, GenerateError> {
    let mut combinator = client_hello_extension_extension_data(ext_len, extension_type);
    combinator.generate(g)
}

///Parse function for client_hello_extension combinator
pub fn parse_client_hello_extension<'p>(
    input: &'p [u8],
) -> Result<(usize, ClientHelloExtension<'p>), ParseError> {
    let combinator = client_hello_extension();
    combinator.parse(input)
}

///Serialize function for client_hello_extension combinator
pub fn serialize_client_hello_extension<'s>(
    v: &'s ClientHelloExtension<'s>,
    data: &mut Vec<u8>,
    pos: usize,
) -> Result<usize, SerializeError> {
    let combinator = client_hello_extension();
    combinator.serialize(v, data, pos)
}

///Serialize function for client_hello_extension combinator (owned version)
pub fn serialize_gen_client_hello_extension(
    v: ClientHelloExtensionOwned,
    data: &mut Vec<u8>,
    pos: usize,
) -> Result<usize, SerializeError> {
    let combinator = client_hello_extension();
    combinator.serialize_gen(v, data, pos)
}

///Length function for client_hello_extension combinator
pub fn client_hello_extension_len<'s>(v: &'s ClientHelloExtension<'s>) -> usize {
    let combinator = client_hello_extension();
    combinator.length(v)
}

///Generate function for client_hello_extension combinator
pub fn generate_client_hello_extension(
    g: &mut GenSt,
) -> GResult<ClientHelloExtensionOwned, GenerateError> {
    let mut combinator = client_hello_extension();
    combinator.generate(g)
}

///Parse function for client_extensions combinator
pub fn parse_client_extensions<'p>(
    input: &'p [u8],
) -> Result<(usize, ClientExtensions<'p>), ParseError> {
    let combinator = client_extensions();
    combinator.parse(input)
}

///Serialize function for client_extensions combinator
pub fn serialize_client_extensions<'s>(
    v: &'s ClientExtensions<'s>,
    data: &mut Vec<u8>,
    pos: usize,
) -> Result<usize, SerializeError> {
    let combinator = client_extensions();
    combinator.serialize(v, data, pos)
}

///Serialize function for client_extensions combinator (owned version)
pub fn serialize_gen_client_extensions(
    v: ClientExtensionsOwned,
    data: &mut Vec<u8>,
    pos: usize,
) -> Result<usize, SerializeError> {
    let combinator = client_extensions();
    combinator.serialize_gen(v, data, pos)
}

///Length function for client_extensions combinator
pub fn client_extensions_len<'s>(v: &'s ClientExtensions<'s>) -> usize {
    let combinator = client_extensions();
    combinator.length(v)
}

///Generate function for client_extensions combinator
pub fn generate_client_extensions(
    g: &mut GenSt,
) -> GResult<ClientExtensionsOwned, GenerateError> {
    let mut combinator = client_extensions();
    combinator.generate(g)
}

///Parse function for client_hello combinator
pub fn parse_client_hello<'p>(
    input: &'p [u8],
) -> Result<(usize, ClientHello<'p>), ParseError> {
    let combinator = client_hello();
    combinator.parse(input)
}

///Serialize function for client_hello combinator
pub fn serialize_client_hello<'s>(
    v: &'s ClientHello<'s>,
    data: &mut Vec<u8>,
    pos: usize,
) -> Result<usize, SerializeError> {
    let combinator = client_hello();
    combinator.serialize(v, data, pos)
}

///Serialize function for client_hello combinator (owned version)
pub fn serialize_gen_client_hello(
    v: ClientHelloOwned,
    data: &mut Vec<u8>,
    pos: usize,
) -> Result<usize, SerializeError> {
    let combinator = client_hello();
    combinator.serialize_gen(v, data, pos)
}

///Length function for client_hello combinator
pub fn client_hello_len<'s>(v: &'s ClientHello<'s>) -> usize {
    let combinator = client_hello();
    combinator.length(v)
}

///Generate function for client_hello combinator
pub fn generate_client_hello(g: &mut GenSt) -> GResult<ClientHelloOwned, GenerateError> {
    let mut combinator = client_hello();
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

///Parse function for early_data_indication_new_session_ticket combinator
pub fn parse_early_data_indication_new_session_ticket<'p>(
    input: &'p [u8],
) -> Result<(usize, EarlyDataIndicationNewSessionTicket), ParseError> {
    let combinator = early_data_indication_new_session_ticket();
    combinator.parse(input)
}

///Serialize function for early_data_indication_new_session_ticket combinator
pub fn serialize_early_data_indication_new_session_ticket<'s>(
    v: EarlyDataIndicationNewSessionTicket,
    data: &mut Vec<u8>,
    pos: usize,
) -> Result<usize, SerializeError> {
    let combinator = early_data_indication_new_session_ticket();
    combinator.serialize(v, data, pos)
}

///Serialize function for early_data_indication_new_session_ticket combinator (owned version)
pub fn serialize_gen_early_data_indication_new_session_ticket(
    v: EarlyDataIndicationNewSessionTicket,
    data: &mut Vec<u8>,
    pos: usize,
) -> Result<usize, SerializeError> {
    let combinator = early_data_indication_new_session_ticket();
    combinator.serialize_gen(v, data, pos)
}

///Length function for early_data_indication_new_session_ticket combinator
pub fn early_data_indication_new_session_ticket_len<'s>(
    v: EarlyDataIndicationNewSessionTicket,
) -> usize {
    let combinator = early_data_indication_new_session_ticket();
    combinator.length(v)
}

///Generate function for early_data_indication_new_session_ticket combinator
pub fn generate_early_data_indication_new_session_ticket(
    g: &mut GenSt,
) -> GResult<EarlyDataIndicationNewSessionTicket, GenerateError> {
    let mut combinator = early_data_indication_new_session_ticket();
    combinator.generate(g)
}

///Parse function for new_session_ticket_extension_extension_data combinator
pub fn parse_new_session_ticket_extension_extension_data<'p>(
    input: &'p [u8],
    ext_len: u16,
    extension_type: u16,
) -> Result<(usize, NewSessionTicketExtensionExtensionData), ParseError> {
    let combinator = new_session_ticket_extension_extension_data(
        ext_len,
        extension_type,
    );
    combinator.parse(input)
}

///Serialize function for new_session_ticket_extension_extension_data combinator
pub fn serialize_new_session_ticket_extension_extension_data<'s>(
    v: &'s NewSessionTicketExtensionExtensionData,
    data: &mut Vec<u8>,
    pos: usize,
    ext_len: u16,
    extension_type: u16,
) -> Result<usize, SerializeError> {
    let combinator = new_session_ticket_extension_extension_data(
        ext_len,
        extension_type,
    );
    combinator.serialize(v, data, pos)
}

///Serialize function for new_session_ticket_extension_extension_data combinator (owned version)
pub fn serialize_gen_new_session_ticket_extension_extension_data(
    v: NewSessionTicketExtensionExtensionData,
    data: &mut Vec<u8>,
    pos: usize,
    ext_len: u16,
    extension_type: u16,
) -> Result<usize, SerializeError> {
    let combinator = new_session_ticket_extension_extension_data(
        ext_len,
        extension_type,
    );
    combinator.serialize_gen(v, data, pos)
}

///Length function for new_session_ticket_extension_extension_data combinator
pub fn new_session_ticket_extension_extension_data_len<'s>(
    v: &'s NewSessionTicketExtensionExtensionData,
    ext_len: u16,
    extension_type: u16,
) -> usize {
    let combinator = new_session_ticket_extension_extension_data(
        ext_len,
        extension_type,
    );
    combinator.length(v)
}

///Generate function for new_session_ticket_extension_extension_data combinator
pub fn generate_new_session_ticket_extension_extension_data<'g>(
    g: &mut GenSt,
    ext_len: &'g mut u16,
    extension_type: &'g mut u16,
) -> GResult<NewSessionTicketExtensionExtensionData, GenerateError> {
    let mut combinator = new_session_ticket_extension_extension_data(
        ext_len,
        extension_type,
    );
    combinator.generate(g)
}

///Parse function for new_session_ticket_extension combinator
pub fn parse_new_session_ticket_extension<'p>(
    input: &'p [u8],
) -> Result<(usize, NewSessionTicketExtension), ParseError> {
    let combinator = new_session_ticket_extension();
    combinator.parse(input)
}

///Serialize function for new_session_ticket_extension combinator
pub fn serialize_new_session_ticket_extension<'s>(
    v: &'s NewSessionTicketExtension,
    data: &mut Vec<u8>,
    pos: usize,
) -> Result<usize, SerializeError> {
    let combinator = new_session_ticket_extension();
    combinator.serialize(v, data, pos)
}

///Serialize function for new_session_ticket_extension combinator (owned version)
pub fn serialize_gen_new_session_ticket_extension(
    v: NewSessionTicketExtension,
    data: &mut Vec<u8>,
    pos: usize,
) -> Result<usize, SerializeError> {
    let combinator = new_session_ticket_extension();
    combinator.serialize_gen(v, data, pos)
}

///Length function for new_session_ticket_extension combinator
pub fn new_session_ticket_extension_len<'s>(v: &'s NewSessionTicketExtension) -> usize {
    let combinator = new_session_ticket_extension();
    combinator.length(v)
}

///Generate function for new_session_ticket_extension combinator
pub fn generate_new_session_ticket_extension(
    g: &mut GenSt,
) -> GResult<NewSessionTicketExtension, GenerateError> {
    let mut combinator = new_session_ticket_extension();
    combinator.generate(g)
}

///Parse function for new_session_ticket_extensions combinator
pub fn parse_new_session_ticket_extensions<'p>(
    input: &'p [u8],
) -> Result<(usize, NewSessionTicketExtensions), ParseError> {
    let combinator = new_session_ticket_extensions();
    combinator.parse(input)
}

///Serialize function for new_session_ticket_extensions combinator
pub fn serialize_new_session_ticket_extensions<'s>(
    v: &'s NewSessionTicketExtensions,
    data: &mut Vec<u8>,
    pos: usize,
) -> Result<usize, SerializeError> {
    let combinator = new_session_ticket_extensions();
    combinator.serialize(v, data, pos)
}

///Serialize function for new_session_ticket_extensions combinator (owned version)
pub fn serialize_gen_new_session_ticket_extensions(
    v: NewSessionTicketExtensions,
    data: &mut Vec<u8>,
    pos: usize,
) -> Result<usize, SerializeError> {
    let combinator = new_session_ticket_extensions();
    combinator.serialize_gen(v, data, pos)
}

///Length function for new_session_ticket_extensions combinator
pub fn new_session_ticket_extensions_len<'s>(
    v: &'s NewSessionTicketExtensions,
) -> usize {
    let combinator = new_session_ticket_extensions();
    combinator.length(v)
}

///Generate function for new_session_ticket_extensions combinator
pub fn generate_new_session_ticket_extensions(
    g: &mut GenSt,
) -> GResult<NewSessionTicketExtensions, GenerateError> {
    let mut combinator = new_session_ticket_extensions();
    combinator.generate(g)
}

///Parse function for new_session_ticket combinator
pub fn parse_new_session_ticket<'p>(
    input: &'p [u8],
) -> Result<(usize, NewSessionTicket<'p>), ParseError> {
    let combinator = new_session_ticket();
    combinator.parse(input)
}

///Serialize function for new_session_ticket combinator
pub fn serialize_new_session_ticket<'s>(
    v: &'s NewSessionTicket<'s>,
    data: &mut Vec<u8>,
    pos: usize,
) -> Result<usize, SerializeError> {
    let combinator = new_session_ticket();
    combinator.serialize(v, data, pos)
}

///Serialize function for new_session_ticket combinator (owned version)
pub fn serialize_gen_new_session_ticket(
    v: NewSessionTicketOwned,
    data: &mut Vec<u8>,
    pos: usize,
) -> Result<usize, SerializeError> {
    let combinator = new_session_ticket();
    combinator.serialize_gen(v, data, pos)
}

///Length function for new_session_ticket combinator
pub fn new_session_ticket_len<'s>(v: &'s NewSessionTicket<'s>) -> usize {
    let combinator = new_session_ticket();
    combinator.length(v)
}

///Generate function for new_session_ticket combinator
pub fn generate_new_session_ticket(
    g: &mut GenSt,
) -> GResult<NewSessionTicketOwned, GenerateError> {
    let mut combinator = new_session_ticket();
    combinator.generate(g)
}

///Parse function for encrypted_extension_extension_data combinator
pub fn parse_encrypted_extension_extension_data<'p>(
    input: &'p [u8],
    ext_len: u16,
    extension_type: u16,
) -> Result<(usize, EncryptedExtensionExtensionData<'p>), ParseError> {
    let combinator = encrypted_extension_extension_data(ext_len, extension_type);
    combinator.parse(input)
}

///Serialize function for encrypted_extension_extension_data combinator
pub fn serialize_encrypted_extension_extension_data<'s>(
    v: &'s EncryptedExtensionExtensionData<'s>,
    data: &mut Vec<u8>,
    pos: usize,
    ext_len: u16,
    extension_type: u16,
) -> Result<usize, SerializeError> {
    let combinator = encrypted_extension_extension_data(ext_len, extension_type);
    combinator.serialize(v, data, pos)
}

///Serialize function for encrypted_extension_extension_data combinator (owned version)
pub fn serialize_gen_encrypted_extension_extension_data(
    v: EncryptedExtensionExtensionDataOwned,
    data: &mut Vec<u8>,
    pos: usize,
    ext_len: u16,
    extension_type: u16,
) -> Result<usize, SerializeError> {
    let combinator = encrypted_extension_extension_data(ext_len, extension_type);
    combinator.serialize_gen(v, data, pos)
}

///Length function for encrypted_extension_extension_data combinator
pub fn encrypted_extension_extension_data_len<'s>(
    v: &'s EncryptedExtensionExtensionData<'s>,
    ext_len: u16,
    extension_type: u16,
) -> usize {
    let combinator = encrypted_extension_extension_data(ext_len, extension_type);
    combinator.length(v)
}

///Generate function for encrypted_extension_extension_data combinator
pub fn generate_encrypted_extension_extension_data<'g>(
    g: &mut GenSt,
    ext_len: &'g mut u16,
    extension_type: &'g mut u16,
) -> GResult<EncryptedExtensionExtensionDataOwned, GenerateError> {
    let mut combinator = encrypted_extension_extension_data(ext_len, extension_type);
    combinator.generate(g)
}

///Parse function for encrypted_extension combinator
pub fn parse_encrypted_extension<'p>(
    input: &'p [u8],
) -> Result<(usize, EncryptedExtension<'p>), ParseError> {
    let combinator = encrypted_extension();
    combinator.parse(input)
}

///Serialize function for encrypted_extension combinator
pub fn serialize_encrypted_extension<'s>(
    v: &'s EncryptedExtension<'s>,
    data: &mut Vec<u8>,
    pos: usize,
) -> Result<usize, SerializeError> {
    let combinator = encrypted_extension();
    combinator.serialize(v, data, pos)
}

///Serialize function for encrypted_extension combinator (owned version)
pub fn serialize_gen_encrypted_extension(
    v: EncryptedExtensionOwned,
    data: &mut Vec<u8>,
    pos: usize,
) -> Result<usize, SerializeError> {
    let combinator = encrypted_extension();
    combinator.serialize_gen(v, data, pos)
}

///Length function for encrypted_extension combinator
pub fn encrypted_extension_len<'s>(v: &'s EncryptedExtension<'s>) -> usize {
    let combinator = encrypted_extension();
    combinator.length(v)
}

///Generate function for encrypted_extension combinator
pub fn generate_encrypted_extension(
    g: &mut GenSt,
) -> GResult<EncryptedExtensionOwned, GenerateError> {
    let mut combinator = encrypted_extension();
    combinator.generate(g)
}

///Parse function for encrypted_extensions combinator
pub fn parse_encrypted_extensions<'p>(
    input: &'p [u8],
) -> Result<(usize, EncryptedExtensions<'p>), ParseError> {
    let combinator = encrypted_extensions();
    combinator.parse(input)
}

///Serialize function for encrypted_extensions combinator
pub fn serialize_encrypted_extensions<'s>(
    v: &'s EncryptedExtensions<'s>,
    data: &mut Vec<u8>,
    pos: usize,
) -> Result<usize, SerializeError> {
    let combinator = encrypted_extensions();
    combinator.serialize(v, data, pos)
}

///Serialize function for encrypted_extensions combinator (owned version)
pub fn serialize_gen_encrypted_extensions(
    v: EncryptedExtensionsOwned,
    data: &mut Vec<u8>,
    pos: usize,
) -> Result<usize, SerializeError> {
    let combinator = encrypted_extensions();
    combinator.serialize_gen(v, data, pos)
}

///Length function for encrypted_extensions combinator
pub fn encrypted_extensions_len<'s>(v: &'s EncryptedExtensions<'s>) -> usize {
    let combinator = encrypted_extensions();
    combinator.length(v)
}

///Generate function for encrypted_extensions combinator
pub fn generate_encrypted_extensions(
    g: &mut GenSt,
) -> GResult<EncryptedExtensionsOwned, GenerateError> {
    let mut combinator = encrypted_extensions();
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

///Parse function for certificate_extension_extension_data combinator
pub fn parse_certificate_extension_extension_data<'p>(
    input: &'p [u8],
    ext_len: u16,
    extension_type: u16,
) -> Result<(usize, CertificateExtensionExtensionData<'p>), ParseError> {
    let combinator = certificate_extension_extension_data(ext_len, extension_type);
    combinator.parse(input)
}

///Serialize function for certificate_extension_extension_data combinator
pub fn serialize_certificate_extension_extension_data<'s>(
    v: &'s CertificateExtensionExtensionData<'s>,
    data: &mut Vec<u8>,
    pos: usize,
    ext_len: u16,
    extension_type: u16,
) -> Result<usize, SerializeError> {
    let combinator = certificate_extension_extension_data(ext_len, extension_type);
    combinator.serialize(v, data, pos)
}

///Serialize function for certificate_extension_extension_data combinator (owned version)
pub fn serialize_gen_certificate_extension_extension_data(
    v: CertificateExtensionExtensionDataOwned,
    data: &mut Vec<u8>,
    pos: usize,
    ext_len: u16,
    extension_type: u16,
) -> Result<usize, SerializeError> {
    let combinator = certificate_extension_extension_data(ext_len, extension_type);
    combinator.serialize_gen(v, data, pos)
}

///Length function for certificate_extension_extension_data combinator
pub fn certificate_extension_extension_data_len<'s>(
    v: &'s CertificateExtensionExtensionData<'s>,
    ext_len: u16,
    extension_type: u16,
) -> usize {
    let combinator = certificate_extension_extension_data(ext_len, extension_type);
    combinator.length(v)
}

///Generate function for certificate_extension_extension_data combinator
pub fn generate_certificate_extension_extension_data<'g>(
    g: &mut GenSt,
    ext_len: &'g mut u16,
    extension_type: &'g mut u16,
) -> GResult<CertificateExtensionExtensionDataOwned, GenerateError> {
    let mut combinator = certificate_extension_extension_data(ext_len, extension_type);
    combinator.generate(g)
}

///Parse function for certificate_extension combinator
pub fn parse_certificate_extension<'p>(
    input: &'p [u8],
) -> Result<(usize, CertificateExtension<'p>), ParseError> {
    let combinator = certificate_extension();
    combinator.parse(input)
}

///Serialize function for certificate_extension combinator
pub fn serialize_certificate_extension<'s>(
    v: &'s CertificateExtension<'s>,
    data: &mut Vec<u8>,
    pos: usize,
) -> Result<usize, SerializeError> {
    let combinator = certificate_extension();
    combinator.serialize(v, data, pos)
}

///Serialize function for certificate_extension combinator (owned version)
pub fn serialize_gen_certificate_extension(
    v: CertificateExtensionOwned,
    data: &mut Vec<u8>,
    pos: usize,
) -> Result<usize, SerializeError> {
    let combinator = certificate_extension();
    combinator.serialize_gen(v, data, pos)
}

///Length function for certificate_extension combinator
pub fn certificate_extension_len<'s>(v: &'s CertificateExtension<'s>) -> usize {
    let combinator = certificate_extension();
    combinator.length(v)
}

///Generate function for certificate_extension combinator
pub fn generate_certificate_extension(
    g: &mut GenSt,
) -> GResult<CertificateExtensionOwned, GenerateError> {
    let mut combinator = certificate_extension();
    combinator.generate(g)
}

///Parse function for certificate_extensions combinator
pub fn parse_certificate_extensions<'p>(
    input: &'p [u8],
) -> Result<(usize, CertificateExtensions<'p>), ParseError> {
    let combinator = certificate_extensions();
    combinator.parse(input)
}

///Serialize function for certificate_extensions combinator
pub fn serialize_certificate_extensions<'s>(
    v: &'s CertificateExtensions<'s>,
    data: &mut Vec<u8>,
    pos: usize,
) -> Result<usize, SerializeError> {
    let combinator = certificate_extensions();
    combinator.serialize(v, data, pos)
}

///Serialize function for certificate_extensions combinator (owned version)
pub fn serialize_gen_certificate_extensions(
    v: CertificateExtensionsOwned,
    data: &mut Vec<u8>,
    pos: usize,
) -> Result<usize, SerializeError> {
    let combinator = certificate_extensions();
    combinator.serialize_gen(v, data, pos)
}

///Length function for certificate_extensions combinator
pub fn certificate_extensions_len<'s>(v: &'s CertificateExtensions<'s>) -> usize {
    let combinator = certificate_extensions();
    combinator.length(v)
}

///Generate function for certificate_extensions combinator
pub fn generate_certificate_extensions(
    g: &mut GenSt,
) -> GResult<CertificateExtensionsOwned, GenerateError> {
    let mut combinator = certificate_extensions();
    combinator.generate(g)
}

///Parse function for certificate_entry_opaque combinator
pub fn parse_certificate_entry_opaque<'p>(
    input: &'p [u8],
) -> Result<(usize, CertificateEntryOpaque<'p>), ParseError> {
    let combinator = certificate_entry_opaque();
    combinator.parse(input)
}

///Serialize function for certificate_entry_opaque combinator
pub fn serialize_certificate_entry_opaque<'s>(
    v: &'s CertificateEntryOpaque<'s>,
    data: &mut Vec<u8>,
    pos: usize,
) -> Result<usize, SerializeError> {
    let combinator = certificate_entry_opaque();
    combinator.serialize(v, data, pos)
}

///Serialize function for certificate_entry_opaque combinator (owned version)
pub fn serialize_gen_certificate_entry_opaque(
    v: CertificateEntryOpaqueOwned,
    data: &mut Vec<u8>,
    pos: usize,
) -> Result<usize, SerializeError> {
    let combinator = certificate_entry_opaque();
    combinator.serialize_gen(v, data, pos)
}

///Length function for certificate_entry_opaque combinator
pub fn certificate_entry_opaque_len<'s>(v: &'s CertificateEntryOpaque<'s>) -> usize {
    let combinator = certificate_entry_opaque();
    combinator.length(v)
}

///Generate function for certificate_entry_opaque combinator
pub fn generate_certificate_entry_opaque(
    g: &mut GenSt,
) -> GResult<CertificateEntryOpaqueOwned, GenerateError> {
    let mut combinator = certificate_entry_opaque();
    combinator.generate(g)
}

///Parse function for certificate_list combinator
pub fn parse_certificate_list<'p>(
    input: &'p [u8],
) -> Result<(usize, CertificateList<'p>), ParseError> {
    let combinator = certificate_list();
    combinator.parse(input)
}

///Serialize function for certificate_list combinator
pub fn serialize_certificate_list<'s>(
    v: &'s CertificateList<'s>,
    data: &mut Vec<u8>,
    pos: usize,
) -> Result<usize, SerializeError> {
    let combinator = certificate_list();
    combinator.serialize(v, data, pos)
}

///Serialize function for certificate_list combinator (owned version)
pub fn serialize_gen_certificate_list(
    v: CertificateListOwned,
    data: &mut Vec<u8>,
    pos: usize,
) -> Result<usize, SerializeError> {
    let combinator = certificate_list();
    combinator.serialize_gen(v, data, pos)
}

///Length function for certificate_list combinator
pub fn certificate_list_len<'s>(v: &'s CertificateList<'s>) -> usize {
    let combinator = certificate_list();
    combinator.length(v)
}

///Generate function for certificate_list combinator
pub fn generate_certificate_list(
    g: &mut GenSt,
) -> GResult<CertificateListOwned, GenerateError> {
    let mut combinator = certificate_list();
    combinator.generate(g)
}

///Parse function for certificate combinator
pub fn parse_certificate<'p>(
    input: &'p [u8],
) -> Result<(usize, Certificate<'p>), ParseError> {
    let combinator = certificate();
    combinator.parse(input)
}

///Serialize function for certificate combinator
pub fn serialize_certificate<'s>(
    v: &'s Certificate<'s>,
    data: &mut Vec<u8>,
    pos: usize,
) -> Result<usize, SerializeError> {
    let combinator = certificate();
    combinator.serialize(v, data, pos)
}

///Serialize function for certificate combinator (owned version)
pub fn serialize_gen_certificate(
    v: CertificateOwned,
    data: &mut Vec<u8>,
    pos: usize,
) -> Result<usize, SerializeError> {
    let combinator = certificate();
    combinator.serialize_gen(v, data, pos)
}

///Length function for certificate combinator
pub fn certificate_len<'s>(v: &'s Certificate<'s>) -> usize {
    let combinator = certificate();
    combinator.length(v)
}

///Generate function for certificate combinator
pub fn generate_certificate(g: &mut GenSt) -> GResult<CertificateOwned, GenerateError> {
    let mut combinator = certificate();
    combinator.generate(g)
}

///Parse function for certificate_request_extensions combinator
pub fn parse_certificate_request_extensions<'p>(
    input: &'p [u8],
) -> Result<(usize, CertificateRequestExtensions<'p>), ParseError> {
    let combinator = certificate_request_extensions();
    combinator.parse(input)
}

///Serialize function for certificate_request_extensions combinator
pub fn serialize_certificate_request_extensions<'s>(
    v: &'s CertificateRequestExtensions<'s>,
    data: &mut Vec<u8>,
    pos: usize,
) -> Result<usize, SerializeError> {
    let combinator = certificate_request_extensions();
    combinator.serialize(v, data, pos)
}

///Serialize function for certificate_request_extensions combinator (owned version)
pub fn serialize_gen_certificate_request_extensions(
    v: CertificateRequestExtensionsOwned,
    data: &mut Vec<u8>,
    pos: usize,
) -> Result<usize, SerializeError> {
    let combinator = certificate_request_extensions();
    combinator.serialize_gen(v, data, pos)
}

///Length function for certificate_request_extensions combinator
pub fn certificate_request_extensions_len<'s>(
    v: &'s CertificateRequestExtensions<'s>,
) -> usize {
    let combinator = certificate_request_extensions();
    combinator.length(v)
}

///Generate function for certificate_request_extensions combinator
pub fn generate_certificate_request_extensions(
    g: &mut GenSt,
) -> GResult<CertificateRequestExtensionsOwned, GenerateError> {
    let mut combinator = certificate_request_extensions();
    combinator.generate(g)
}

///Parse function for certificate_request combinator
pub fn parse_certificate_request<'p>(
    input: &'p [u8],
) -> Result<(usize, CertificateRequest<'p>), ParseError> {
    let combinator = certificate_request();
    combinator.parse(input)
}

///Serialize function for certificate_request combinator
pub fn serialize_certificate_request<'s>(
    v: &'s CertificateRequest<'s>,
    data: &mut Vec<u8>,
    pos: usize,
) -> Result<usize, SerializeError> {
    let combinator = certificate_request();
    combinator.serialize(v, data, pos)
}

///Serialize function for certificate_request combinator (owned version)
pub fn serialize_gen_certificate_request(
    v: CertificateRequestOwned,
    data: &mut Vec<u8>,
    pos: usize,
) -> Result<usize, SerializeError> {
    let combinator = certificate_request();
    combinator.serialize_gen(v, data, pos)
}

///Length function for certificate_request combinator
pub fn certificate_request_len<'s>(v: &'s CertificateRequest<'s>) -> usize {
    let combinator = certificate_request();
    combinator.length(v)
}

///Generate function for certificate_request combinator
pub fn generate_certificate_request(
    g: &mut GenSt,
) -> GResult<CertificateRequestOwned, GenerateError> {
    let mut combinator = certificate_request();
    combinator.generate(g)
}

///Parse function for certificate_verify combinator
pub fn parse_certificate_verify<'p>(
    input: &'p [u8],
) -> Result<(usize, CertificateVerify<'p>), ParseError> {
    let combinator = certificate_verify();
    combinator.parse(input)
}

///Serialize function for certificate_verify combinator
pub fn serialize_certificate_verify<'s>(
    v: &'s CertificateVerify<'s>,
    data: &mut Vec<u8>,
    pos: usize,
) -> Result<usize, SerializeError> {
    let combinator = certificate_verify();
    combinator.serialize(v, data, pos)
}

///Serialize function for certificate_verify combinator (owned version)
pub fn serialize_gen_certificate_verify(
    v: CertificateVerifyOwned,
    data: &mut Vec<u8>,
    pos: usize,
) -> Result<usize, SerializeError> {
    let combinator = certificate_verify();
    combinator.serialize_gen(v, data, pos)
}

///Length function for certificate_verify combinator
pub fn certificate_verify_len<'s>(v: &'s CertificateVerify<'s>) -> usize {
    let combinator = certificate_verify();
    combinator.length(v)
}

///Generate function for certificate_verify combinator
pub fn generate_certificate_verify(
    g: &mut GenSt,
) -> GResult<CertificateVerifyOwned, GenerateError> {
    let mut combinator = certificate_verify();
    combinator.generate(g)
}

///Parse function for finished combinator
pub fn parse_finished<'p>(
    input: &'p [u8],
    size: u24,
) -> Result<(usize, Finished<'p>), ParseError> {
    let combinator = finished(size);
    combinator.parse(input)
}

///Serialize function for finished combinator
pub fn serialize_finished<'s>(
    v: &'s Finished<'s>,
    data: &mut Vec<u8>,
    pos: usize,
    size: u24,
) -> Result<usize, SerializeError> {
    let combinator = finished(size);
    combinator.serialize(v, data, pos)
}

///Serialize function for finished combinator (owned version)
pub fn serialize_gen_finished(
    v: FinishedOwned,
    data: &mut Vec<u8>,
    pos: usize,
    size: u24,
) -> Result<usize, SerializeError> {
    let combinator = finished(size);
    combinator.serialize_gen(v, data, pos)
}

///Length function for finished combinator
pub fn finished_len<'s>(v: &'s Finished<'s>, size: u24) -> usize {
    let combinator = finished(size);
    combinator.length(v)
}

///Generate function for finished combinator
pub fn generate_finished<'g>(
    g: &mut GenSt,
    size: &'g mut u24,
) -> GResult<FinishedOwned, GenerateError> {
    let mut combinator = finished(size);
    combinator.generate(g)
}

///Parse function for key_update_request combinator
pub fn parse_key_update_request<'p>(
    input: &'p [u8],
) -> Result<(usize, KeyUpdateRequest), ParseError> {
    let combinator = key_update_request();
    combinator.parse(input)
}

///Serialize function for key_update_request combinator
pub fn serialize_key_update_request<'s>(
    v: KeyUpdateRequest,
    data: &mut Vec<u8>,
    pos: usize,
) -> Result<usize, SerializeError> {
    let combinator = key_update_request();
    combinator.serialize(v, data, pos)
}

///Serialize function for key_update_request combinator (owned version)
pub fn serialize_gen_key_update_request(
    v: KeyUpdateRequest,
    data: &mut Vec<u8>,
    pos: usize,
) -> Result<usize, SerializeError> {
    let combinator = key_update_request();
    combinator.serialize_gen(v, data, pos)
}

///Length function for key_update_request combinator
pub fn key_update_request_len<'s>(v: KeyUpdateRequest) -> usize {
    let combinator = key_update_request();
    combinator.length(v)
}

///Generate function for key_update_request combinator
pub fn generate_key_update_request(
    g: &mut GenSt,
) -> GResult<KeyUpdateRequest, GenerateError> {
    let mut combinator = key_update_request();
    combinator.generate(g)
}

///Parse function for key_update combinator
pub fn parse_key_update<'p>(input: &'p [u8]) -> Result<(usize, KeyUpdate), ParseError> {
    let combinator = key_update();
    combinator.parse(input)
}

///Serialize function for key_update combinator
pub fn serialize_key_update<'s>(
    v: KeyUpdate,
    data: &mut Vec<u8>,
    pos: usize,
) -> Result<usize, SerializeError> {
    let combinator = key_update();
    combinator.serialize(v, data, pos)
}

///Serialize function for key_update combinator (owned version)
pub fn serialize_gen_key_update(
    v: KeyUpdate,
    data: &mut Vec<u8>,
    pos: usize,
) -> Result<usize, SerializeError> {
    let combinator = key_update();
    combinator.serialize_gen(v, data, pos)
}

///Length function for key_update combinator
pub fn key_update_len<'s>(v: KeyUpdate) -> usize {
    let combinator = key_update();
    combinator.length(v)
}

///Generate function for key_update combinator
pub fn generate_key_update(g: &mut GenSt) -> GResult<KeyUpdate, GenerateError> {
    let mut combinator = key_update();
    combinator.generate(g)
}

///Parse function for handshake_msg combinator
pub fn parse_handshake_msg<'p>(
    input: &'p [u8],
    length: u24,
    msg_type: HandshakeType,
) -> Result<(usize, HandshakeMsg<'p>), ParseError> {
    let combinator = handshake_msg(length, msg_type);
    combinator.parse(input)
}

///Serialize function for handshake_msg combinator
pub fn serialize_handshake_msg<'s>(
    v: &'s HandshakeMsg<'s>,
    data: &mut Vec<u8>,
    pos: usize,
    length: u24,
    msg_type: HandshakeType,
) -> Result<usize, SerializeError> {
    let combinator = handshake_msg(length, msg_type);
    combinator.serialize(v, data, pos)
}

///Serialize function for handshake_msg combinator (owned version)
pub fn serialize_gen_handshake_msg(
    v: HandshakeMsgOwned,
    data: &mut Vec<u8>,
    pos: usize,
    length: u24,
    msg_type: HandshakeType,
) -> Result<usize, SerializeError> {
    let combinator = handshake_msg(length, msg_type);
    combinator.serialize_gen(v, data, pos)
}

///Length function for handshake_msg combinator
pub fn handshake_msg_len<'s>(
    v: &'s HandshakeMsg<'s>,
    length: u24,
    msg_type: HandshakeType,
) -> usize {
    let combinator = handshake_msg(length, msg_type);
    combinator.length(v)
}

///Generate function for handshake_msg combinator
pub fn generate_handshake_msg<'g>(
    g: &mut GenSt,
    length: &'g mut u24,
    msg_type: &'g mut HandshakeType,
) -> GResult<HandshakeMsgOwned, GenerateError> {
    let mut combinator = handshake_msg(length, msg_type);
    combinator.generate(g)
}

///Parse function for handshake combinator
pub fn parse_handshake<'p>(
    input: &'p [u8],
) -> Result<(usize, Handshake<'p>), ParseError> {
    let combinator = handshake();
    combinator.parse(input)
}

///Serialize function for handshake combinator
pub fn serialize_handshake<'s>(
    v: &'s Handshake<'s>,
    data: &mut Vec<u8>,
    pos: usize,
) -> Result<usize, SerializeError> {
    let combinator = handshake();
    combinator.serialize(v, data, pos)
}

///Serialize function for handshake combinator (owned version)
pub fn serialize_gen_handshake(
    v: HandshakeOwned,
    data: &mut Vec<u8>,
    pos: usize,
) -> Result<usize, SerializeError> {
    let combinator = handshake();
    combinator.serialize_gen(v, data, pos)
}

///Length function for handshake combinator
pub fn handshake_len<'s>(v: &'s Handshake<'s>) -> usize {
    let combinator = handshake();
    combinator.length(v)
}

///Generate function for handshake combinator
pub fn generate_handshake(g: &mut GenSt) -> GResult<HandshakeOwned, GenerateError> {
    let mut combinator = handshake();
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

///Parse function for extension combinator
pub fn parse_extension<'p>(
    input: &'p [u8],
) -> Result<(usize, Extension<'p>), ParseError> {
    let combinator = extension();
    combinator.parse(input)
}

///Serialize function for extension combinator
pub fn serialize_extension<'s>(
    v: &'s Extension<'s>,
    data: &mut Vec<u8>,
    pos: usize,
) -> Result<usize, SerializeError> {
    let combinator = extension();
    combinator.serialize(v, data, pos)
}

///Serialize function for extension combinator (owned version)
pub fn serialize_gen_extension(
    v: ExtensionOwned,
    data: &mut Vec<u8>,
    pos: usize,
) -> Result<usize, SerializeError> {
    let combinator = extension();
    combinator.serialize_gen(v, data, pos)
}

///Length function for extension combinator
pub fn extension_len<'s>(v: &'s Extension<'s>) -> usize {
    let combinator = extension();
    combinator.length(v)
}

///Generate function for extension combinator
pub fn generate_extension(g: &mut GenSt) -> GResult<ExtensionOwned, GenerateError> {
    let mut combinator = extension();
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

///Parse function for content_type combinator
pub fn parse_content_type<'p>(
    input: &'p [u8],
) -> Result<(usize, ContentType), ParseError> {
    let combinator = content_type();
    combinator.parse(input)
}

///Serialize function for content_type combinator
pub fn serialize_content_type<'s>(
    v: ContentType,
    data: &mut Vec<u8>,
    pos: usize,
) -> Result<usize, SerializeError> {
    let combinator = content_type();
    combinator.serialize(v, data, pos)
}

///Serialize function for content_type combinator (owned version)
pub fn serialize_gen_content_type(
    v: ContentType,
    data: &mut Vec<u8>,
    pos: usize,
) -> Result<usize, SerializeError> {
    let combinator = content_type();
    combinator.serialize_gen(v, data, pos)
}

///Length function for content_type combinator
pub fn content_type_len<'s>(v: ContentType) -> usize {
    let combinator = content_type();
    combinator.length(v)
}

///Generate function for content_type combinator
pub fn generate_content_type(g: &mut GenSt) -> GResult<ContentType, GenerateError> {
    let mut combinator = content_type();
    combinator.generate(g)
}

///Parse function for tls_plaintext combinator
pub fn parse_tls_plaintext<'p>(
    input: &'p [u8],
) -> Result<(usize, TlsPlaintext<'p>), ParseError> {
    let combinator = tls_plaintext();
    combinator.parse(input)
}

///Serialize function for tls_plaintext combinator
pub fn serialize_tls_plaintext<'s>(
    v: &'s TlsPlaintext<'s>,
    data: &mut Vec<u8>,
    pos: usize,
) -> Result<usize, SerializeError> {
    let combinator = tls_plaintext();
    combinator.serialize(v, data, pos)
}

///Serialize function for tls_plaintext combinator (owned version)
pub fn serialize_gen_tls_plaintext(
    v: TlsPlaintextOwned,
    data: &mut Vec<u8>,
    pos: usize,
) -> Result<usize, SerializeError> {
    let combinator = tls_plaintext();
    combinator.serialize_gen(v, data, pos)
}

///Length function for tls_plaintext combinator
pub fn tls_plaintext_len<'s>(v: &'s TlsPlaintext<'s>) -> usize {
    let combinator = tls_plaintext();
    combinator.length(v)
}

///Generate function for tls_plaintext combinator
pub fn generate_tls_plaintext(
    g: &mut GenSt,
) -> GResult<TlsPlaintextOwned, GenerateError> {
    let mut combinator = tls_plaintext();
    combinator.generate(g)
}

///Parse function for alert_description combinator
pub fn parse_alert_description<'p>(
    input: &'p [u8],
) -> Result<(usize, AlertDescription), ParseError> {
    let combinator = alert_description();
    combinator.parse(input)
}

///Serialize function for alert_description combinator
pub fn serialize_alert_description<'s>(
    v: AlertDescription,
    data: &mut Vec<u8>,
    pos: usize,
) -> Result<usize, SerializeError> {
    let combinator = alert_description();
    combinator.serialize(v, data, pos)
}

///Serialize function for alert_description combinator (owned version)
pub fn serialize_gen_alert_description(
    v: AlertDescription,
    data: &mut Vec<u8>,
    pos: usize,
) -> Result<usize, SerializeError> {
    let combinator = alert_description();
    combinator.serialize_gen(v, data, pos)
}

///Length function for alert_description combinator
pub fn alert_description_len<'s>(v: AlertDescription) -> usize {
    let combinator = alert_description();
    combinator.length(v)
}

///Generate function for alert_description combinator
pub fn generate_alert_description(
    g: &mut GenSt,
) -> GResult<AlertDescription, GenerateError> {
    let mut combinator = alert_description();
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

///Parse function for supported_versions_server combinator
pub fn parse_supported_versions_server<'p>(
    input: &'p [u8],
) -> Result<(usize, SupportedVersionsServer), ParseError> {
    let combinator = supported_versions_server();
    combinator.parse(input)
}

///Serialize function for supported_versions_server combinator
pub fn serialize_supported_versions_server<'s>(
    v: ProtocolVersion,
    data: &mut Vec<u8>,
    pos: usize,
) -> Result<usize, SerializeError> {
    let combinator = supported_versions_server();
    combinator.serialize(v, data, pos)
}

///Serialize function for supported_versions_server combinator (owned version)
pub fn serialize_gen_supported_versions_server(
    v: SupportedVersionsServer,
    data: &mut Vec<u8>,
    pos: usize,
) -> Result<usize, SerializeError> {
    let combinator = supported_versions_server();
    combinator.serialize_gen(v, data, pos)
}

///Length function for supported_versions_server combinator
pub fn supported_versions_server_len<'s>(v: ProtocolVersion) -> usize {
    let combinator = supported_versions_server();
    combinator.length(v)
}

///Generate function for supported_versions_server combinator
pub fn generate_supported_versions_server(
    g: &mut GenSt,
) -> GResult<SupportedVersionsServer, GenerateError> {
    let mut combinator = supported_versions_server();
    combinator.generate(g)
}

///Parse function for hello_retry_extension_extension_data combinator
pub fn parse_hello_retry_extension_extension_data<'p>(
    input: &'p [u8],
    ext_len: u16,
    extension_type: u16,
) -> Result<(usize, HelloRetryExtensionExtensionData<'p>), ParseError> {
    let combinator = hello_retry_extension_extension_data(ext_len, extension_type);
    combinator.parse(input)
}

///Serialize function for hello_retry_extension_extension_data combinator
pub fn serialize_hello_retry_extension_extension_data<'s>(
    v: &'s HelloRetryExtensionExtensionData<'s>,
    data: &mut Vec<u8>,
    pos: usize,
    ext_len: u16,
    extension_type: u16,
) -> Result<usize, SerializeError> {
    let combinator = hello_retry_extension_extension_data(ext_len, extension_type);
    combinator.serialize(v, data, pos)
}

///Serialize function for hello_retry_extension_extension_data combinator (owned version)
pub fn serialize_gen_hello_retry_extension_extension_data(
    v: HelloRetryExtensionExtensionDataOwned,
    data: &mut Vec<u8>,
    pos: usize,
    ext_len: u16,
    extension_type: u16,
) -> Result<usize, SerializeError> {
    let combinator = hello_retry_extension_extension_data(ext_len, extension_type);
    combinator.serialize_gen(v, data, pos)
}

///Length function for hello_retry_extension_extension_data combinator
pub fn hello_retry_extension_extension_data_len<'s>(
    v: &'s HelloRetryExtensionExtensionData<'s>,
    ext_len: u16,
    extension_type: u16,
) -> usize {
    let combinator = hello_retry_extension_extension_data(ext_len, extension_type);
    combinator.length(v)
}

///Generate function for hello_retry_extension_extension_data combinator
pub fn generate_hello_retry_extension_extension_data<'g>(
    g: &mut GenSt,
    ext_len: &'g mut u16,
    extension_type: &'g mut u16,
) -> GResult<HelloRetryExtensionExtensionDataOwned, GenerateError> {
    let mut combinator = hello_retry_extension_extension_data(ext_len, extension_type);
    combinator.generate(g)
}

///Parse function for finished_opaque combinator
pub fn parse_finished_opaque<'p>(
    input: &'p [u8],
    digest_size: u24,
) -> Result<(usize, FinishedOpaque<'p>), ParseError> {
    let combinator = finished_opaque(digest_size);
    combinator.parse(input)
}

///Serialize function for finished_opaque combinator
pub fn serialize_finished_opaque<'s>(
    v: &'s [u8],
    data: &mut Vec<u8>,
    pos: usize,
    digest_size: u24,
) -> Result<usize, SerializeError> {
    let combinator = finished_opaque(digest_size);
    combinator.serialize(v, data, pos)
}

///Serialize function for finished_opaque combinator (owned version)
pub fn serialize_gen_finished_opaque(
    v: FinishedOpaqueOwned,
    data: &mut Vec<u8>,
    pos: usize,
    digest_size: u24,
) -> Result<usize, SerializeError> {
    let combinator = finished_opaque(digest_size);
    combinator.serialize_gen(v, data, pos)
}

///Length function for finished_opaque combinator
pub fn finished_opaque_len<'s>(v: &'s [u8], digest_size: u24) -> usize {
    let combinator = finished_opaque(digest_size);
    combinator.length(v)
}

///Generate function for finished_opaque combinator
pub fn generate_finished_opaque(
    g: &mut GenSt,
    digest_size: u24,
) -> GResult<FinishedOpaqueOwned, GenerateError> {
    let mut combinator = finished_opaque(digest_size);
    combinator.generate(g)
}

///Parse function for pre_shared_key_server_extension combinator
pub fn parse_pre_shared_key_server_extension<'p>(
    input: &'p [u8],
) -> Result<(usize, PreSharedKeyServerExtension), ParseError> {
    let combinator = pre_shared_key_server_extension();
    combinator.parse(input)
}

///Serialize function for pre_shared_key_server_extension combinator
pub fn serialize_pre_shared_key_server_extension<'s>(
    v: PreSharedKeyServerExtension,
    data: &mut Vec<u8>,
    pos: usize,
) -> Result<usize, SerializeError> {
    let combinator = pre_shared_key_server_extension();
    combinator.serialize(v, data, pos)
}

///Serialize function for pre_shared_key_server_extension combinator (owned version)
pub fn serialize_gen_pre_shared_key_server_extension(
    v: PreSharedKeyServerExtension,
    data: &mut Vec<u8>,
    pos: usize,
) -> Result<usize, SerializeError> {
    let combinator = pre_shared_key_server_extension();
    combinator.serialize_gen(v, data, pos)
}

///Length function for pre_shared_key_server_extension combinator
pub fn pre_shared_key_server_extension_len<'s>(v: PreSharedKeyServerExtension) -> usize {
    let combinator = pre_shared_key_server_extension();
    combinator.length(v)
}

///Generate function for pre_shared_key_server_extension combinator
pub fn generate_pre_shared_key_server_extension(
    g: &mut GenSt,
) -> GResult<PreSharedKeyServerExtension, GenerateError> {
    let mut combinator = pre_shared_key_server_extension();
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

///Parse function for hello_retry_extension combinator
pub fn parse_hello_retry_extension<'p>(
    input: &'p [u8],
) -> Result<(usize, HelloRetryExtension<'p>), ParseError> {
    let combinator = hello_retry_extension();
    combinator.parse(input)
}

///Serialize function for hello_retry_extension combinator
pub fn serialize_hello_retry_extension<'s>(
    v: &'s HelloRetryExtension<'s>,
    data: &mut Vec<u8>,
    pos: usize,
) -> Result<usize, SerializeError> {
    let combinator = hello_retry_extension();
    combinator.serialize(v, data, pos)
}

///Serialize function for hello_retry_extension combinator (owned version)
pub fn serialize_gen_hello_retry_extension(
    v: HelloRetryExtensionOwned,
    data: &mut Vec<u8>,
    pos: usize,
) -> Result<usize, SerializeError> {
    let combinator = hello_retry_extension();
    combinator.serialize_gen(v, data, pos)
}

///Length function for hello_retry_extension combinator
pub fn hello_retry_extension_len<'s>(v: &'s HelloRetryExtension<'s>) -> usize {
    let combinator = hello_retry_extension();
    combinator.length(v)
}

///Generate function for hello_retry_extension combinator
pub fn generate_hello_retry_extension(
    g: &mut GenSt,
) -> GResult<HelloRetryExtensionOwned, GenerateError> {
    let mut combinator = hello_retry_extension();
    combinator.generate(g)
}

///Parse function for alert combinator
pub fn parse_alert<'p>(input: &'p [u8]) -> Result<(usize, Alert), ParseError> {
    let combinator = alert();
    combinator.parse(input)
}

///Serialize function for alert combinator
pub fn serialize_alert<'s>(
    v: Alert,
    data: &mut Vec<u8>,
    pos: usize,
) -> Result<usize, SerializeError> {
    let combinator = alert();
    combinator.serialize(v, data, pos)
}

///Serialize function for alert combinator (owned version)
pub fn serialize_gen_alert(
    v: Alert,
    data: &mut Vec<u8>,
    pos: usize,
) -> Result<usize, SerializeError> {
    let combinator = alert();
    combinator.serialize_gen(v, data, pos)
}

///Length function for alert combinator
pub fn alert_len<'s>(v: Alert) -> usize {
    let combinator = alert();
    combinator.length(v)
}

///Generate function for alert combinator
pub fn generate_alert(g: &mut GenSt) -> GResult<Alert, GenerateError> {
    let mut combinator = alert();
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

///Parse function for unknown_extension combinator
pub fn parse_unknown_extension<'p>(
    input: &'p [u8],
) -> Result<(usize, UnknownExtension<'p>), ParseError> {
    let combinator = unknown_extension();
    combinator.parse(input)
}

///Serialize function for unknown_extension combinator
pub fn serialize_unknown_extension<'s>(
    v: &'s Opaque0Ffff<'s>,
    data: &mut Vec<u8>,
    pos: usize,
) -> Result<usize, SerializeError> {
    let combinator = unknown_extension();
    combinator.serialize(v, data, pos)
}

///Serialize function for unknown_extension combinator (owned version)
pub fn serialize_gen_unknown_extension(
    v: UnknownExtensionOwned,
    data: &mut Vec<u8>,
    pos: usize,
) -> Result<usize, SerializeError> {
    let combinator = unknown_extension();
    combinator.serialize_gen(v, data, pos)
}

///Length function for unknown_extension combinator
pub fn unknown_extension_len<'s>(v: &'s Opaque0Ffff<'s>) -> usize {
    let combinator = unknown_extension();
    combinator.length(v)
}

///Generate function for unknown_extension combinator
pub fn generate_unknown_extension(
    g: &mut GenSt,
) -> GResult<UnknownExtensionOwned, GenerateError> {
    let mut combinator = unknown_extension();
    combinator.generate(g)
}

///Parse function for hello_retry_extensions combinator
pub fn parse_hello_retry_extensions<'p>(
    input: &'p [u8],
) -> Result<(usize, HelloRetryExtensions<'p>), ParseError> {
    let combinator = hello_retry_extensions();
    combinator.parse(input)
}

///Serialize function for hello_retry_extensions combinator
pub fn serialize_hello_retry_extensions<'s>(
    v: &'s HelloRetryExtensions<'s>,
    data: &mut Vec<u8>,
    pos: usize,
) -> Result<usize, SerializeError> {
    let combinator = hello_retry_extensions();
    combinator.serialize(v, data, pos)
}

///Serialize function for hello_retry_extensions combinator (owned version)
pub fn serialize_gen_hello_retry_extensions(
    v: HelloRetryExtensionsOwned,
    data: &mut Vec<u8>,
    pos: usize,
) -> Result<usize, SerializeError> {
    let combinator = hello_retry_extensions();
    combinator.serialize_gen(v, data, pos)
}

///Length function for hello_retry_extensions combinator
pub fn hello_retry_extensions_len<'s>(v: &'s HelloRetryExtensions<'s>) -> usize {
    let combinator = hello_retry_extensions();
    combinator.length(v)
}

///Generate function for hello_retry_extensions combinator
pub fn generate_hello_retry_extensions(
    g: &mut GenSt,
) -> GResult<HelloRetryExtensionsOwned, GenerateError> {
    let mut combinator = hello_retry_extensions();
    combinator.generate(g)
}

///Parse function for sever_hello_extension_extension_data combinator
pub fn parse_sever_hello_extension_extension_data<'p>(
    input: &'p [u8],
    ext_len: u16,
    extension_type: u16,
) -> Result<(usize, SeverHelloExtensionExtensionData<'p>), ParseError> {
    let combinator = sever_hello_extension_extension_data(ext_len, extension_type);
    combinator.parse(input)
}

///Serialize function for sever_hello_extension_extension_data combinator
pub fn serialize_sever_hello_extension_extension_data<'s>(
    v: &'s SeverHelloExtensionExtensionData<'s>,
    data: &mut Vec<u8>,
    pos: usize,
    ext_len: u16,
    extension_type: u16,
) -> Result<usize, SerializeError> {
    let combinator = sever_hello_extension_extension_data(ext_len, extension_type);
    combinator.serialize(v, data, pos)
}

///Serialize function for sever_hello_extension_extension_data combinator (owned version)
pub fn serialize_gen_sever_hello_extension_extension_data(
    v: SeverHelloExtensionExtensionDataOwned,
    data: &mut Vec<u8>,
    pos: usize,
    ext_len: u16,
    extension_type: u16,
) -> Result<usize, SerializeError> {
    let combinator = sever_hello_extension_extension_data(ext_len, extension_type);
    combinator.serialize_gen(v, data, pos)
}

///Length function for sever_hello_extension_extension_data combinator
pub fn sever_hello_extension_extension_data_len<'s>(
    v: &'s SeverHelloExtensionExtensionData<'s>,
    ext_len: u16,
    extension_type: u16,
) -> usize {
    let combinator = sever_hello_extension_extension_data(ext_len, extension_type);
    combinator.length(v)
}

///Generate function for sever_hello_extension_extension_data combinator
pub fn generate_sever_hello_extension_extension_data<'g>(
    g: &mut GenSt,
    ext_len: &'g mut u16,
    extension_type: &'g mut u16,
) -> GResult<SeverHelloExtensionExtensionDataOwned, GenerateError> {
    let mut combinator = sever_hello_extension_extension_data(ext_len, extension_type);
    combinator.generate(g)
}

///Parse function for sever_hello_extension combinator
pub fn parse_sever_hello_extension<'p>(
    input: &'p [u8],
) -> Result<(usize, SeverHelloExtension<'p>), ParseError> {
    let combinator = sever_hello_extension();
    combinator.parse(input)
}

///Serialize function for sever_hello_extension combinator
pub fn serialize_sever_hello_extension<'s>(
    v: &'s SeverHelloExtension<'s>,
    data: &mut Vec<u8>,
    pos: usize,
) -> Result<usize, SerializeError> {
    let combinator = sever_hello_extension();
    combinator.serialize(v, data, pos)
}

///Serialize function for sever_hello_extension combinator (owned version)
pub fn serialize_gen_sever_hello_extension(
    v: SeverHelloExtensionOwned,
    data: &mut Vec<u8>,
    pos: usize,
) -> Result<usize, SerializeError> {
    let combinator = sever_hello_extension();
    combinator.serialize_gen(v, data, pos)
}

///Length function for sever_hello_extension combinator
pub fn sever_hello_extension_len<'s>(v: &'s SeverHelloExtension<'s>) -> usize {
    let combinator = sever_hello_extension();
    combinator.length(v)
}

///Generate function for sever_hello_extension combinator
pub fn generate_sever_hello_extension(
    g: &mut GenSt,
) -> GResult<SeverHelloExtensionOwned, GenerateError> {
    let mut combinator = sever_hello_extension();
    combinator.generate(g)
}

///Parse function for server_extensions combinator
pub fn parse_server_extensions<'p>(
    input: &'p [u8],
) -> Result<(usize, ServerExtensions<'p>), ParseError> {
    let combinator = server_extensions();
    combinator.parse(input)
}

///Serialize function for server_extensions combinator
pub fn serialize_server_extensions<'s>(
    v: &'s ServerExtensions<'s>,
    data: &mut Vec<u8>,
    pos: usize,
) -> Result<usize, SerializeError> {
    let combinator = server_extensions();
    combinator.serialize(v, data, pos)
}

///Serialize function for server_extensions combinator (owned version)
pub fn serialize_gen_server_extensions(
    v: ServerExtensionsOwned,
    data: &mut Vec<u8>,
    pos: usize,
) -> Result<usize, SerializeError> {
    let combinator = server_extensions();
    combinator.serialize_gen(v, data, pos)
}

///Length function for server_extensions combinator
pub fn server_extensions_len<'s>(v: &'s ServerExtensions<'s>) -> usize {
    let combinator = server_extensions();
    combinator.length(v)
}

///Generate function for server_extensions combinator
pub fn generate_server_extensions(
    g: &mut GenSt,
) -> GResult<ServerExtensionsOwned, GenerateError> {
    let mut combinator = server_extensions();
    combinator.generate(g)
}

///Parse function for digest_size combinator
pub fn parse_digest_size<'p>(
    input: &'p [u8],
) -> Result<(usize, DigestSize), ParseError> {
    let combinator = digest_size();
    combinator.parse(input)
}

///Serialize function for digest_size combinator
pub fn serialize_digest_size<'s>(
    v: u24,
    data: &mut Vec<u8>,
    pos: usize,
) -> Result<usize, SerializeError> {
    let combinator = digest_size();
    combinator.serialize(v, data, pos)
}

///Serialize function for digest_size combinator (owned version)
pub fn serialize_gen_digest_size(
    v: DigestSize,
    data: &mut Vec<u8>,
    pos: usize,
) -> Result<usize, SerializeError> {
    let combinator = digest_size();
    combinator.serialize_gen(v, data, pos)
}

///Length function for digest_size combinator
pub fn digest_size_len<'s>(v: u24) -> usize {
    let combinator = digest_size();
    combinator.length(v)
}

///Generate function for digest_size combinator
pub fn generate_digest_size(g: &mut GenSt) -> GResult<DigestSize, GenerateError> {
    let mut combinator = digest_size();
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

///Parse function for ec_point_format combinator
pub fn parse_ec_point_format<'p>(
    input: &'p [u8],
) -> Result<(usize, EcPointFormat), ParseError> {
    let combinator = ec_point_format();
    combinator.parse(input)
}

///Serialize function for ec_point_format combinator
pub fn serialize_ec_point_format<'s>(
    v: u8,
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
pub fn ec_point_format_len<'s>(v: u8) -> usize {
    let combinator = ec_point_format();
    combinator.length(v)
}

///Generate function for ec_point_format combinator
pub fn generate_ec_point_format(g: &mut GenSt) -> GResult<EcPointFormat, GenerateError> {
    let mut combinator = ec_point_format();
    combinator.generate(g)
}

///Parse function for server_hello combinator
pub fn parse_server_hello<'p>(
    input: &'p [u8],
) -> Result<(usize, ServerHello<'p>), ParseError> {
    let combinator = server_hello();
    combinator.parse(input)
}

///Serialize function for server_hello combinator
pub fn serialize_server_hello<'s>(
    v: &'s ServerHello<'s>,
    data: &mut Vec<u8>,
    pos: usize,
) -> Result<usize, SerializeError> {
    let combinator = server_hello();
    combinator.serialize(v, data, pos)
}

///Serialize function for server_hello combinator (owned version)
pub fn serialize_gen_server_hello(
    v: ServerHelloOwned,
    data: &mut Vec<u8>,
    pos: usize,
) -> Result<usize, SerializeError> {
    let combinator = server_hello();
    combinator.serialize_gen(v, data, pos)
}

///Length function for server_hello combinator
pub fn server_hello_len<'s>(v: &'s ServerHello<'s>) -> usize {
    let combinator = server_hello();
    combinator.length(v)
}

///Generate function for server_hello combinator
pub fn generate_server_hello(g: &mut GenSt) -> GResult<ServerHelloOwned, GenerateError> {
    let mut combinator = server_hello();
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

///Parse function for hello_retry_request combinator
pub fn parse_hello_retry_request<'p>(
    input: &'p [u8],
) -> Result<(usize, HelloRetryRequest<'p>), ParseError> {
    let combinator = hello_retry_request();
    combinator.parse(input)
}

///Serialize function for hello_retry_request combinator
pub fn serialize_hello_retry_request<'s>(
    v: &'s HelloRetryRequest<'s>,
    data: &mut Vec<u8>,
    pos: usize,
) -> Result<usize, SerializeError> {
    let combinator = hello_retry_request();
    combinator.serialize(v, data, pos)
}

///Serialize function for hello_retry_request combinator (owned version)
pub fn serialize_gen_hello_retry_request(
    v: HelloRetryRequestOwned,
    data: &mut Vec<u8>,
    pos: usize,
) -> Result<usize, SerializeError> {
    let combinator = hello_retry_request();
    combinator.serialize_gen(v, data, pos)
}

///Length function for hello_retry_request combinator
pub fn hello_retry_request_len<'s>(v: &'s HelloRetryRequest<'s>) -> usize {
    let combinator = hello_retry_request();
    combinator.length(v)
}

///Generate function for hello_retry_request combinator
pub fn generate_hello_retry_request(
    g: &mut GenSt,
) -> GResult<HelloRetryRequestOwned, GenerateError> {
    let mut combinator = hello_retry_request();
    combinator.generate(g)
}

///Parse function for certificate_entry_data combinator
pub fn parse_certificate_entry_data<'p>(
    input: &'p [u8],
    cert_type: u8,
) -> Result<(usize, CertificateEntryData<'p>), ParseError> {
    let combinator = certificate_entry_data(cert_type);
    combinator.parse(input)
}

///Serialize function for certificate_entry_data combinator
pub fn serialize_certificate_entry_data<'s>(
    v: &'s CertificateEntryData<'s>,
    data: &mut Vec<u8>,
    pos: usize,
    cert_type: u8,
) -> Result<usize, SerializeError> {
    let combinator = certificate_entry_data(cert_type);
    combinator.serialize(v, data, pos)
}

///Serialize function for certificate_entry_data combinator (owned version)
pub fn serialize_gen_certificate_entry_data(
    v: CertificateEntryDataOwned,
    data: &mut Vec<u8>,
    pos: usize,
    cert_type: u8,
) -> Result<usize, SerializeError> {
    let combinator = certificate_entry_data(cert_type);
    combinator.serialize_gen(v, data, pos)
}

///Length function for certificate_entry_data combinator
pub fn certificate_entry_data_len<'s>(
    v: &'s CertificateEntryData<'s>,
    cert_type: u8,
) -> usize {
    let combinator = certificate_entry_data(cert_type);
    combinator.length(v)
}

///Generate function for certificate_entry_data combinator
pub fn generate_certificate_entry_data<'g>(
    g: &mut GenSt,
    cert_type: &'g mut u8,
) -> GResult<CertificateEntryDataOwned, GenerateError> {
    let mut combinator = certificate_entry_data(cert_type);
    combinator.generate(g)
}

///Parse function for certificate_entry combinator
pub fn parse_certificate_entry<'p>(
    input: &'p [u8],
    cert_type: u8,
) -> Result<(usize, CertificateEntry<'p>), ParseError> {
    let combinator = certificate_entry(cert_type);
    combinator.parse(input)
}

///Serialize function for certificate_entry combinator
pub fn serialize_certificate_entry<'s>(
    v: &'s CertificateEntry<'s>,
    data: &mut Vec<u8>,
    pos: usize,
    cert_type: u8,
) -> Result<usize, SerializeError> {
    let combinator = certificate_entry(cert_type);
    combinator.serialize(v, data, pos)
}

///Serialize function for certificate_entry combinator (owned version)
pub fn serialize_gen_certificate_entry(
    v: CertificateEntryOwned,
    data: &mut Vec<u8>,
    pos: usize,
    cert_type: u8,
) -> Result<usize, SerializeError> {
    let combinator = certificate_entry(cert_type);
    combinator.serialize_gen(v, data, pos)
}

///Length function for certificate_entry combinator
pub fn certificate_entry_len<'s>(v: &'s CertificateEntry<'s>, cert_type: u8) -> usize {
    let combinator = certificate_entry(cert_type);
    combinator.length(v)
}

///Generate function for certificate_entry combinator
pub fn generate_certificate_entry<'g>(
    g: &mut GenSt,
    cert_type: &'g mut u8,
) -> GResult<CertificateEntryOwned, GenerateError> {
    let mut combinator = certificate_entry(cert_type);
    combinator.generate(g)
}

///Parse function for tls_ciphertext combinator
pub fn parse_tls_ciphertext<'p>(
    input: &'p [u8],
) -> Result<(usize, TlsCiphertext<'p>), ParseError> {
    let combinator = tls_ciphertext();
    combinator.parse(input)
}

///Serialize function for tls_ciphertext combinator
pub fn serialize_tls_ciphertext<'s>(
    v: &'s TlsCiphertext<'s>,
    data: &mut Vec<u8>,
    pos: usize,
) -> Result<usize, SerializeError> {
    let combinator = tls_ciphertext();
    combinator.serialize(v, data, pos)
}

///Serialize function for tls_ciphertext combinator (owned version)
pub fn serialize_gen_tls_ciphertext(
    v: TlsCiphertextOwned,
    data: &mut Vec<u8>,
    pos: usize,
) -> Result<usize, SerializeError> {
    let combinator = tls_ciphertext();
    combinator.serialize_gen(v, data, pos)
}

///Length function for tls_ciphertext combinator
pub fn tls_ciphertext_len<'s>(v: &'s TlsCiphertext<'s>) -> usize {
    let combinator = tls_ciphertext();
    combinator.length(v)
}

///Generate function for tls_ciphertext combinator
pub fn generate_tls_ciphertext(
    g: &mut GenSt,
) -> GResult<TlsCiphertextOwned, GenerateError> {
    let mut combinator = tls_ciphertext();
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

impl<C> Combinator<[u8], Vec<u8>> for AlertLevelCombinator<C>
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

impl<C> Combinator<[u8], Vec<u8>> for EmptyCombinator<C>
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

impl<C> Combinator<[u8], Vec<u8>> for DistinguishedNameCombinator<C>
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

impl<C> Combinator<[u8], Vec<u8>> for CertificateAuthoritiesExtensionCombinator<C>
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

impl<C> Combinator<[u8], Vec<u8>> for OidFilterCombinator<C>
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

impl<C> Combinator<[u8], Vec<u8>> for OidFilterExtensionCombinator<C>
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

impl<C> Combinator<[u8], Vec<u8>>
for CertificateRequestExtensionExtensionDataCombinator<C>
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

impl<C> Combinator<[u8], Vec<u8>> for CertificateRequestExtensionCombinator<C>
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

impl<C> Combinator<[u8], Vec<u8>> for ShOrHrrCombinator<C>
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

impl<C> Combinator<[u8], Vec<u8>> for HandshakeTypeCombinator<C>
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

impl<C> Combinator<[u8], Vec<u8>> for SessionIdCombinator<C>
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

impl<C> Combinator<[u8], Vec<u8>> for CipherSuiteCombinator<C>
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

impl<C> Combinator<[u8], Vec<u8>> for CipherSuiteListCombinator<C>
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

impl<C> Combinator<[u8], Vec<u8>> for SupportedVersionsClientCombinator<C>
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

impl<C> Combinator<[u8], Vec<u8>> for KeyShareEntryCombinator<C>
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

impl<C> Combinator<[u8], Vec<u8>> for KeyShareClientHelloCombinator<C>
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

impl<C> Combinator<[u8], Vec<u8>> for PskKeyExchangeModeCombinator<C>
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

impl<C> Combinator<[u8], Vec<u8>> for PskKeyExchangeModesCombinator<C>
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

impl<C> Combinator<[u8], Vec<u8>> for PskIdentityCombinator<C>
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

impl<C> Combinator<[u8], Vec<u8>> for PskIdentitiesCombinator<C>
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

impl<C> Combinator<[u8], Vec<u8>> for PskBinderEntryCombinator<C>
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

impl<C> Combinator<[u8], Vec<u8>> for PskBinderEntriesCombinator<C>
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

impl<C> Combinator<[u8], Vec<u8>> for OfferedPsksCombinator<C>
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

impl<C> Combinator<[u8], Vec<u8>> for PreSharedKeyClientExtensionCombinator<C>
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

impl<C> Combinator<[u8], Vec<u8>> for CookieCombinator<C>
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

impl<C> Combinator<[u8], Vec<u8>> for ClientHelloExtensionRestCombinator<C>
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

impl<C> Combinator<[u8], Vec<u8>> for ClientHelloExtensionExtensionDataCombinator<C>
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

impl<C> Combinator<[u8], Vec<u8>> for ClientHelloExtensionCombinator<C>
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

impl<C> Combinator<[u8], Vec<u8>> for ClientExtensionsCombinator<C>
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

impl<C> Combinator<[u8], Vec<u8>> for ClientHelloCombinator<C>
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

impl<C> Combinator<[u8], Vec<u8>> for EarlyDataIndicationNewSessionTicketCombinator<C>
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

impl<C> Combinator<[u8], Vec<u8>> for NewSessionTicketExtensionExtensionDataCombinator<C>
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

impl<C> Combinator<[u8], Vec<u8>> for NewSessionTicketExtensionCombinator<C>
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

impl<C> Combinator<[u8], Vec<u8>> for NewSessionTicketExtensionsCombinator<C>
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

impl<C> Combinator<[u8], Vec<u8>> for NewSessionTicketCombinator<C>
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

impl<C> Combinator<[u8], Vec<u8>> for EncryptedExtensionExtensionDataCombinator<C>
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

impl<C> Combinator<[u8], Vec<u8>> for EncryptedExtensionCombinator<C>
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

impl<C> Combinator<[u8], Vec<u8>> for EncryptedExtensionsCombinator<C>
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

impl<C> Combinator<[u8], Vec<u8>> for CertificateExtensionExtensionDataCombinator<C>
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

impl<C> Combinator<[u8], Vec<u8>> for CertificateExtensionCombinator<C>
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

impl<C> Combinator<[u8], Vec<u8>> for CertificateExtensionsCombinator<C>
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

impl<C> Combinator<[u8], Vec<u8>> for CertificateEntryOpaqueCombinator<C>
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

impl<C> Combinator<[u8], Vec<u8>> for CertificateListCombinator<C>
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

impl<C> Combinator<[u8], Vec<u8>> for CertificateCombinator<C>
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

impl<C> Combinator<[u8], Vec<u8>> for CertificateRequestExtensionsCombinator<C>
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

impl<C> Combinator<[u8], Vec<u8>> for CertificateRequestCombinator<C>
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

impl<C> Combinator<[u8], Vec<u8>> for CertificateVerifyCombinator<C>
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

impl<C> Combinator<[u8], Vec<u8>> for FinishedCombinator<C>
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

impl<C> Combinator<[u8], Vec<u8>> for KeyUpdateRequestCombinator<C>
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

impl<C> Combinator<[u8], Vec<u8>> for KeyUpdateCombinator<C>
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

impl<C> Combinator<[u8], Vec<u8>> for HandshakeMsgCombinator<C>
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

impl<C> Combinator<[u8], Vec<u8>> for HandshakeCombinator<C>
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

impl<C> Combinator<[u8], Vec<u8>> for ExtensionCombinator<C>
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

impl<C> Combinator<[u8], Vec<u8>> for ContentTypeCombinator<C>
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

impl<C> Combinator<[u8], Vec<u8>> for TlsPlaintextCombinator<C>
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

impl<C> Combinator<[u8], Vec<u8>> for AlertDescriptionCombinator<C>
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

impl<C> Combinator<[u8], Vec<u8>> for SupportedVersionsServerCombinator<C>
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

impl<C> Combinator<[u8], Vec<u8>> for HelloRetryExtensionExtensionDataCombinator<C>
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

impl<C> Combinator<[u8], Vec<u8>> for FinishedOpaqueCombinator<C>
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

impl<C> Combinator<[u8], Vec<u8>> for PreSharedKeyServerExtensionCombinator<C>
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

impl<C> Combinator<[u8], Vec<u8>> for HelloRetryExtensionCombinator<C>
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

impl<C> Combinator<[u8], Vec<u8>> for AlertCombinator<C>
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

impl<C> Combinator<[u8], Vec<u8>> for UnknownExtensionCombinator<C>
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

impl<C> Combinator<[u8], Vec<u8>> for HelloRetryExtensionsCombinator<C>
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

impl<C> Combinator<[u8], Vec<u8>> for SeverHelloExtensionExtensionDataCombinator<C>
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

impl<C> Combinator<[u8], Vec<u8>> for SeverHelloExtensionCombinator<C>
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

impl<C> Combinator<[u8], Vec<u8>> for ServerExtensionsCombinator<C>
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

impl<C> Combinator<[u8], Vec<u8>> for DigestSizeCombinator<C>
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

impl<C> Combinator<[u8], Vec<u8>> for ServerHelloCombinator<C>
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

impl<C> Combinator<[u8], Vec<u8>> for HelloRetryRequestCombinator<C>
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

impl<C> Combinator<[u8], Vec<u8>> for CertificateEntryDataCombinator<C>
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

impl<C> Combinator<[u8], Vec<u8>> for CertificateEntryCombinator<C>
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

impl<C> Combinator<[u8], Vec<u8>> for TlsCiphertextCombinator<C>
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

#![allow(non_camel_case_types)]

#[repr(C)]
#[derive(Debug, Eq, PartialEq)]
pub enum BlockType {
    InterfaceDescription = 1,
    Packet = 2,
    SimplePacket = 3,
    NameResolution = 4,
    InterfaceStatistics = 5,
    EnhancedPacket = 6,
    IrigTimestamp = 7,
    Arinc429_AFDX = 8,
    SectionHeader = 0x0A0D0D0A,
}

#[repr(C)]
/// Link Type
// Taken from https://www.winpcap.org/ntar/draft/PCAP-DumpFileFormat.html#appendixLinkTypes
pub enum LinkType {
    NULL = 0,
    ETHERNET = 1,
    EXP_ETHERNET = 2,
    AX25 = 3,
    PRONET = 4,
    CHAOS = 5,
    TOKEN_RING = 6,
    ARCNET = 7,
    SLIP = 8,
    PPP = 9,
    FDDI = 10,
    PPP_HDLC = 50,
    PPP_ETHER = 51,
    SYMANTEC_FIREWALL = 52,
    ATM_RFC1483 = 100,
    RAW = 101,
    SLIP_BSDOS = 102,
    PPP_BSDOS = 103,
    C_HDLC = 104,
    IEEE802_11 = 105,
    ATM_CLIP = 106,
    FRELAY = 107,
    LOOP = 108,
    ENC = 109,
    LANE8023 = 110,
    HIPPI = 111,
    HDLC = 112,
    LINUX_SLL = 113,
    LTALK = 114,
    ECONET = 115,
    IPFILTER = 116,
    PFLOG = 117,
    CISCO_IOS = 118,
    PRISM_HEADER = 119,
    AIRONET_HEADER = 120,
    HHDLC = 121,
    IP_OVER_FC = 122,
    SUNATM = 123,
    RIO = 124,
    PCI_EXP = 125,
    AURORA = 126,
    IEEE802_11_RADIO = 127,
    TZSP = 128,
    ARCNET_LINUX = 129,
    JUNIPER_MLPPP = 130,
    JUNIPER_MLFR = 131,
    JUNIPER_ES = 132,
    JUNIPER_GGSN = 133,
    JUNIPER_MFR = 134,
    JUNIPER_ATM2 = 135,
    JUNIPER_SERVICES = 136,
    JUNIPER_ATM1 = 137,
    APPLE_IP_OVER_IEEE1394 = 138,
    MTP2_WITH_PHDR = 139,
    MTP2 = 140,
    MTP3 = 141,
    SCCP = 142,
    DOCSIS = 143,
    LINUX_IRDA = 144,
    IBM_SP = 145,
    IBM_SN = 146,
}

#[repr(C)]
pub enum LinkTypeOptions {
    EndOfOpt = 0,
    Comment = 1,
    Name = 2,
    Description = 3,
    Ipv4Addr = 4,
    Ipv6Addr = 5,
    MacAddr = 6,
    EuiAddr = 7,
    Speed = 8,
    TsResol = 9,
    Tzone = 10,
    Filter = 11,
    OS = 12,
    Fcslen = 13,
    TsOffset = 14,
}

#[repr(C)]
pub enum EnhancedPacketOptions {
    EndOfOpt = 0,
    Comment = 1,
    Flags = 2,
    Hash = 3,
    DropCount = 4,
}

#[repr(C)]
pub enum InterfaceStatisticsOptions {
    EndOfOpt = 0,
    Comment = 1,
    StartTime = 2,
    EndTime = 3,
    IfRecv = 4,
    IfDrop = 5,
    FilterAccept = 6,
    OSDrop = 7,
    UsrDeliv = 8,
}

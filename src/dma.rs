use vcell::VolatileCell;
#[doc = r" Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - DMA Status Registers"]
    pub status: STATUS,
    #[doc = "0x04 - DMA Configuration Register"]
    pub config: CONFIG,
    #[doc = "0x08 - Channel Control Data Base Pointer Register"]
    pub ctrlbase: CTRLBASE,
    #[doc = "0x0c - Channel Alternate Control Data Base Pointer Register"]
    pub altctrlbase: ALTCTRLBASE,
    #[doc = "0x10 - Channel Wait on Request Status Register"]
    pub chwaitstatus: CHWAITSTATUS,
    #[doc = "0x14 - Channel Software Request Register"]
    pub chswreq: CHSWREQ,
    #[doc = "0x18 - Channel Useburst Set Register"]
    pub chusebursts: CHUSEBURSTS,
    #[doc = "0x1c - Channel Useburst Clear Register"]
    pub chuseburstc: CHUSEBURSTC,
    #[doc = "0x20 - Channel Request Mask Set Register"]
    pub chreqmasks: CHREQMASKS,
    #[doc = "0x24 - Channel Request Mask Clear Register"]
    pub chreqmaskc: CHREQMASKC,
    #[doc = "0x28 - Channel Enable Set Register"]
    pub chens: CHENS,
    #[doc = "0x2c - Channel Enable Clear Register"]
    pub chenc: CHENC,
    #[doc = "0x30 - Channel Alternate Set Register"]
    pub chalts: CHALTS,
    #[doc = "0x34 - Channel Alternate Clear Register"]
    pub chaltc: CHALTC,
    #[doc = "0x38 - Channel Priority Set Register"]
    pub chpris: CHPRIS,
    #[doc = "0x3c - Channel Priority Clear Register"]
    pub chpric: CHPRIC,
    _reserved0: [u8; 12usize],
    #[doc = "0x4c - Bus Error Clear Register"]
    pub errorc: ERRORC,
    _reserved1: [u8; 3520usize],
    #[doc = "0xe10 - Channel Request Status"]
    pub chreqstatus: CHREQSTATUS,
    _reserved2: [u8; 4usize],
    #[doc = "0xe18 - Channel Single Request Status"]
    pub chsreqstatus: CHSREQSTATUS,
    _reserved3: [u8; 484usize],
    #[doc = "0x1000 - Interrupt Flag Register"]
    pub if_: IF,
    #[doc = "0x1004 - Interrupt Flag Set Register"]
    pub ifs: IFS,
    #[doc = "0x1008 - Interrupt Flag Clear Register"]
    pub ifc: IFC,
    #[doc = "0x100c - Interrupt Enable register"]
    pub ien: IEN,
    #[doc = "0x1010 - DMA Control Register"]
    pub ctrl: CTRL,
    #[doc = "0x1014 - DMA Retain Descriptor State"]
    pub rds: RDS,
    _reserved4: [u8; 8usize],
    #[doc = "0x1020 - Channel 0 Loop Register"]
    pub loop0: LOOP0,
    #[doc = "0x1024 - Channel 1 Loop Register"]
    pub loop1: LOOP1,
    _reserved5: [u8; 56usize],
    #[doc = "0x1060 - Channel 0 Rectangle Register"]
    pub rect0: RECT0,
    _reserved6: [u8; 156usize],
    #[doc = "0x1100 - Channel Control Register"]
    pub ch0_ctrl: CH0_CTRL,
    #[doc = "0x1104 - Channel Control Register"]
    pub ch1_ctrl: CH1_CTRL,
    #[doc = "0x1108 - Channel Control Register"]
    pub ch2_ctrl: CH2_CTRL,
    #[doc = "0x110c - Channel Control Register"]
    pub ch3_ctrl: CH3_CTRL,
    #[doc = "0x1110 - Channel Control Register"]
    pub ch4_ctrl: CH4_CTRL,
    #[doc = "0x1114 - Channel Control Register"]
    pub ch5_ctrl: CH5_CTRL,
    #[doc = "0x1118 - Channel Control Register"]
    pub ch6_ctrl: CH6_CTRL,
    #[doc = "0x111c - Channel Control Register"]
    pub ch7_ctrl: CH7_CTRL,
    #[doc = "0x1120 - Channel Control Register"]
    pub ch8_ctrl: CH8_CTRL,
    #[doc = "0x1124 - Channel Control Register"]
    pub ch9_ctrl: CH9_CTRL,
    #[doc = "0x1128 - Channel Control Register"]
    pub ch10_ctrl: CH10_CTRL,
    #[doc = "0x112c - Channel Control Register"]
    pub ch11_ctrl: CH11_CTRL,
}
#[doc = "DMA Status Registers"]
pub struct STATUS {
    register: VolatileCell<u32>,
}
#[doc = "DMA Status Registers"]
pub mod status;
#[doc = "DMA Configuration Register"]
pub struct CONFIG {
    register: VolatileCell<u32>,
}
#[doc = "DMA Configuration Register"]
pub mod config;
#[doc = "Channel Control Data Base Pointer Register"]
pub struct CTRLBASE {
    register: VolatileCell<u32>,
}
#[doc = "Channel Control Data Base Pointer Register"]
pub mod ctrlbase;
#[doc = "Channel Alternate Control Data Base Pointer Register"]
pub struct ALTCTRLBASE {
    register: VolatileCell<u32>,
}
#[doc = "Channel Alternate Control Data Base Pointer Register"]
pub mod altctrlbase;
#[doc = "Channel Wait on Request Status Register"]
pub struct CHWAITSTATUS {
    register: VolatileCell<u32>,
}
#[doc = "Channel Wait on Request Status Register"]
pub mod chwaitstatus;
#[doc = "Channel Software Request Register"]
pub struct CHSWREQ {
    register: VolatileCell<u32>,
}
#[doc = "Channel Software Request Register"]
pub mod chswreq;
#[doc = "Channel Useburst Set Register"]
pub struct CHUSEBURSTS {
    register: VolatileCell<u32>,
}
#[doc = "Channel Useburst Set Register"]
pub mod chusebursts;
#[doc = "Channel Useburst Clear Register"]
pub struct CHUSEBURSTC {
    register: VolatileCell<u32>,
}
#[doc = "Channel Useburst Clear Register"]
pub mod chuseburstc;
#[doc = "Channel Request Mask Set Register"]
pub struct CHREQMASKS {
    register: VolatileCell<u32>,
}
#[doc = "Channel Request Mask Set Register"]
pub mod chreqmasks;
#[doc = "Channel Request Mask Clear Register"]
pub struct CHREQMASKC {
    register: VolatileCell<u32>,
}
#[doc = "Channel Request Mask Clear Register"]
pub mod chreqmaskc;
#[doc = "Channel Enable Set Register"]
pub struct CHENS {
    register: VolatileCell<u32>,
}
#[doc = "Channel Enable Set Register"]
pub mod chens;
#[doc = "Channel Enable Clear Register"]
pub struct CHENC {
    register: VolatileCell<u32>,
}
#[doc = "Channel Enable Clear Register"]
pub mod chenc;
#[doc = "Channel Alternate Set Register"]
pub struct CHALTS {
    register: VolatileCell<u32>,
}
#[doc = "Channel Alternate Set Register"]
pub mod chalts;
#[doc = "Channel Alternate Clear Register"]
pub struct CHALTC {
    register: VolatileCell<u32>,
}
#[doc = "Channel Alternate Clear Register"]
pub mod chaltc;
#[doc = "Channel Priority Set Register"]
pub struct CHPRIS {
    register: VolatileCell<u32>,
}
#[doc = "Channel Priority Set Register"]
pub mod chpris;
#[doc = "Channel Priority Clear Register"]
pub struct CHPRIC {
    register: VolatileCell<u32>,
}
#[doc = "Channel Priority Clear Register"]
pub mod chpric;
#[doc = "Bus Error Clear Register"]
pub struct ERRORC {
    register: VolatileCell<u32>,
}
#[doc = "Bus Error Clear Register"]
pub mod errorc;
#[doc = "Channel Request Status"]
pub struct CHREQSTATUS {
    register: VolatileCell<u32>,
}
#[doc = "Channel Request Status"]
pub mod chreqstatus;
#[doc = "Channel Single Request Status"]
pub struct CHSREQSTATUS {
    register: VolatileCell<u32>,
}
#[doc = "Channel Single Request Status"]
pub mod chsreqstatus;
#[doc = "Interrupt Flag Register"]
pub struct IF {
    register: VolatileCell<u32>,
}
#[doc = "Interrupt Flag Register"]
pub mod if_;
#[doc = "Interrupt Flag Set Register"]
pub struct IFS {
    register: VolatileCell<u32>,
}
#[doc = "Interrupt Flag Set Register"]
pub mod ifs;
#[doc = "Interrupt Flag Clear Register"]
pub struct IFC {
    register: VolatileCell<u32>,
}
#[doc = "Interrupt Flag Clear Register"]
pub mod ifc;
#[doc = "Interrupt Enable register"]
pub struct IEN {
    register: VolatileCell<u32>,
}
#[doc = "Interrupt Enable register"]
pub mod ien;
#[doc = "DMA Control Register"]
pub struct CTRL {
    register: VolatileCell<u32>,
}
#[doc = "DMA Control Register"]
pub mod ctrl;
#[doc = "DMA Retain Descriptor State"]
pub struct RDS {
    register: VolatileCell<u32>,
}
#[doc = "DMA Retain Descriptor State"]
pub mod rds;
#[doc = "Channel 0 Loop Register"]
pub struct LOOP0 {
    register: VolatileCell<u32>,
}
#[doc = "Channel 0 Loop Register"]
pub mod loop0;
#[doc = "Channel 1 Loop Register"]
pub struct LOOP1 {
    register: VolatileCell<u32>,
}
#[doc = "Channel 1 Loop Register"]
pub mod loop1;
#[doc = "Channel 0 Rectangle Register"]
pub struct RECT0 {
    register: VolatileCell<u32>,
}
#[doc = "Channel 0 Rectangle Register"]
pub mod rect0;
#[doc = "Channel Control Register"]
pub struct CH0_CTRL {
    register: VolatileCell<u32>,
}
#[doc = "Channel Control Register"]
pub mod ch0_ctrl;
#[doc = "Channel Control Register"]
pub struct CH1_CTRL {
    register: VolatileCell<u32>,
}
#[doc = "Channel Control Register"]
pub mod ch1_ctrl;
#[doc = "Channel Control Register"]
pub struct CH2_CTRL {
    register: VolatileCell<u32>,
}
#[doc = "Channel Control Register"]
pub mod ch2_ctrl;
#[doc = "Channel Control Register"]
pub struct CH3_CTRL {
    register: VolatileCell<u32>,
}
#[doc = "Channel Control Register"]
pub mod ch3_ctrl;
#[doc = "Channel Control Register"]
pub struct CH4_CTRL {
    register: VolatileCell<u32>,
}
#[doc = "Channel Control Register"]
pub mod ch4_ctrl;
#[doc = "Channel Control Register"]
pub struct CH5_CTRL {
    register: VolatileCell<u32>,
}
#[doc = "Channel Control Register"]
pub mod ch5_ctrl;
#[doc = "Channel Control Register"]
pub struct CH6_CTRL {
    register: VolatileCell<u32>,
}
#[doc = "Channel Control Register"]
pub mod ch6_ctrl;
#[doc = "Channel Control Register"]
pub struct CH7_CTRL {
    register: VolatileCell<u32>,
}
#[doc = "Channel Control Register"]
pub mod ch7_ctrl;
#[doc = "Channel Control Register"]
pub struct CH8_CTRL {
    register: VolatileCell<u32>,
}
#[doc = "Channel Control Register"]
pub mod ch8_ctrl;
#[doc = "Channel Control Register"]
pub struct CH9_CTRL {
    register: VolatileCell<u32>,
}
#[doc = "Channel Control Register"]
pub mod ch9_ctrl;
#[doc = "Channel Control Register"]
pub struct CH10_CTRL {
    register: VolatileCell<u32>,
}
#[doc = "Channel Control Register"]
pub mod ch10_ctrl;
#[doc = "Channel Control Register"]
pub struct CH11_CTRL {
    register: VolatileCell<u32>,
}
#[doc = "Channel Control Register"]
pub mod ch11_ctrl;

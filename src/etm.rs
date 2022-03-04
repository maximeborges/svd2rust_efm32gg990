use vcell::VolatileCell;
#[doc = r" Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Main Control Register"]
    pub etmcr: ETMCR,
    #[doc = "0x04 - Configuration Code Register"]
    pub etmccr: ETMCCR,
    #[doc = "0x08 - ETM Trigger Event Register"]
    pub etmtrigger: ETMTRIGGER,
    _reserved0: [u8; 4usize],
    #[doc = "0x10 - ETM Status Register"]
    pub etmsr: ETMSR,
    #[doc = "0x14 - ETM System Configuration Register"]
    pub etmscr: ETMSCR,
    _reserved1: [u8; 8usize],
    #[doc = "0x20 - ETM TraceEnable Event Register"]
    pub etmteevr: ETMTEEVR,
    #[doc = "0x24 - ETM Trace control Register"]
    pub etmtecr1: ETMTECR1,
    _reserved2: [u8; 4usize],
    #[doc = "0x2c - ETM Fifo Full Level Register"]
    pub etmfflr: ETMFFLR,
    _reserved3: [u8; 272usize],
    #[doc = "0x140 - Counter Reload Value"]
    pub etmcntrldvr1: ETMCNTRLDVR1,
    _reserved4: [u8; 156usize],
    #[doc = "0x1e0 - Synchronisation Frequency Register"]
    pub etmsyncfr: ETMSYNCFR,
    #[doc = "0x1e4 - ID Register"]
    pub etmidr: ETMIDR,
    #[doc = "0x1e8 - Configuration Code Extension Register"]
    pub etmccer: ETMCCER,
    _reserved5: [u8; 4usize],
    #[doc = "0x1f0 - TraceEnable Start/Stop EmbeddedICE Control Register"]
    pub etmtesseicr: ETMTESSEICR,
    _reserved6: [u8; 4usize],
    #[doc = "0x1f8 - Timestamp Event Register"]
    pub etmtsevr: ETMTSEVR,
    _reserved7: [u8; 4usize],
    #[doc = "0x200 - CoreSight Trace ID Register"]
    pub etmtraceidr: ETMTRACEIDR,
    _reserved8: [u8; 4usize],
    #[doc = "0x208 - ETM ID Register 2"]
    pub etmidr2: ETMIDR2,
    _reserved9: [u8; 264usize],
    #[doc = "0x314 - Device Power-down Status Register"]
    pub etmpdsr: ETMPDSR,
    _reserved10: [u8; 3016usize],
    #[doc = "0xee0 - Integration Test Miscellaneous Inputs Register"]
    pub etmiscin: ETMISCIN,
    _reserved11: [u8; 4usize],
    #[doc = "0xee8 - Integration Test Trigger Out Register"]
    pub ittrigout: ITTRIGOUT,
    _reserved12: [u8; 4usize],
    #[doc = "0xef0 - ETM Integration Test ATB Control 2 Register"]
    pub etmitatbctr2: ETMITATBCTR2,
    _reserved13: [u8; 4usize],
    #[doc = "0xef8 - ETM Integration Test ATB Control 0 Register"]
    pub etmitatbctr0: ETMITATBCTR0,
    _reserved14: [u8; 4usize],
    #[doc = "0xf00 - ETM Integration Control Register"]
    pub etmitctrl: ETMITCTRL,
    _reserved15: [u8; 156usize],
    #[doc = "0xfa0 - ETM Claim Tag Set Register"]
    pub etmclaimset: ETMCLAIMSET,
    #[doc = "0xfa4 - ETM Claim Tag Clear Register"]
    pub etmclaimclr: ETMCLAIMCLR,
    _reserved16: [u8; 8usize],
    #[doc = "0xfb0 - ETM Lock Access Register"]
    pub etmlar: ETMLAR,
    #[doc = "0xfb4 - Lock Status Register"]
    pub etmlsr: ETMLSR,
    #[doc = "0xfb8 - ETM Authentication Status Register"]
    pub etmauthstatus: ETMAUTHSTATUS,
    _reserved17: [u8; 16usize],
    #[doc = "0xfcc - CoreSight Device Type Register"]
    pub etmdevtype: ETMDEVTYPE,
    #[doc = "0xfd0 - Peripheral ID4 Register"]
    pub etmpidr4: ETMPIDR4,
    #[doc = "0xfd4 - Peripheral ID5 Register"]
    pub etmpidr5: ETMPIDR5,
    #[doc = "0xfd8 - Peripheral ID6 Register"]
    pub etmpidr6: ETMPIDR6,
    #[doc = "0xfdc - Peripheral ID7 Register"]
    pub etmpidr7: ETMPIDR7,
    #[doc = "0xfe0 - Peripheral ID0 Register"]
    pub etmpidr0: ETMPIDR0,
    #[doc = "0xfe4 - Peripheral ID1 Register"]
    pub etmpidr1: ETMPIDR1,
    #[doc = "0xfe8 - Peripheral ID2 Register"]
    pub etmpidr2: ETMPIDR2,
    #[doc = "0xfec - Peripheral ID3 Register"]
    pub etmpidr3: ETMPIDR3,
    #[doc = "0xff0 - Component ID0 Register"]
    pub etmcidr0: ETMCIDR0,
    #[doc = "0xff4 - Component ID1 Register"]
    pub etmcidr1: ETMCIDR1,
    #[doc = "0xff8 - Component ID2 Register"]
    pub etmcidr2: ETMCIDR2,
    #[doc = "0xffc - Component ID3 Register"]
    pub etmcidr3: ETMCIDR3,
}
#[doc = "Main Control Register"]
pub struct ETMCR {
    register: VolatileCell<u32>,
}
#[doc = "Main Control Register"]
pub mod etmcr;
#[doc = "Configuration Code Register"]
pub struct ETMCCR {
    register: VolatileCell<u32>,
}
#[doc = "Configuration Code Register"]
pub mod etmccr;
#[doc = "ETM Trigger Event Register"]
pub struct ETMTRIGGER {
    register: VolatileCell<u32>,
}
#[doc = "ETM Trigger Event Register"]
pub mod etmtrigger;
#[doc = "ETM Status Register"]
pub struct ETMSR {
    register: VolatileCell<u32>,
}
#[doc = "ETM Status Register"]
pub mod etmsr;
#[doc = "ETM System Configuration Register"]
pub struct ETMSCR {
    register: VolatileCell<u32>,
}
#[doc = "ETM System Configuration Register"]
pub mod etmscr;
#[doc = "ETM TraceEnable Event Register"]
pub struct ETMTEEVR {
    register: VolatileCell<u32>,
}
#[doc = "ETM TraceEnable Event Register"]
pub mod etmteevr;
#[doc = "ETM Trace control Register"]
pub struct ETMTECR1 {
    register: VolatileCell<u32>,
}
#[doc = "ETM Trace control Register"]
pub mod etmtecr1;
#[doc = "ETM Fifo Full Level Register"]
pub struct ETMFFLR {
    register: VolatileCell<u32>,
}
#[doc = "ETM Fifo Full Level Register"]
pub mod etmfflr;
#[doc = "Counter Reload Value"]
pub struct ETMCNTRLDVR1 {
    register: VolatileCell<u32>,
}
#[doc = "Counter Reload Value"]
pub mod etmcntrldvr1;
#[doc = "Synchronisation Frequency Register"]
pub struct ETMSYNCFR {
    register: VolatileCell<u32>,
}
#[doc = "Synchronisation Frequency Register"]
pub mod etmsyncfr;
#[doc = "ID Register"]
pub struct ETMIDR {
    register: VolatileCell<u32>,
}
#[doc = "ID Register"]
pub mod etmidr;
#[doc = "Configuration Code Extension Register"]
pub struct ETMCCER {
    register: VolatileCell<u32>,
}
#[doc = "Configuration Code Extension Register"]
pub mod etmccer;
#[doc = "TraceEnable Start/Stop EmbeddedICE Control Register"]
pub struct ETMTESSEICR {
    register: VolatileCell<u32>,
}
#[doc = "TraceEnable Start/Stop EmbeddedICE Control Register"]
pub mod etmtesseicr;
#[doc = "Timestamp Event Register"]
pub struct ETMTSEVR {
    register: VolatileCell<u32>,
}
#[doc = "Timestamp Event Register"]
pub mod etmtsevr;
#[doc = "CoreSight Trace ID Register"]
pub struct ETMTRACEIDR {
    register: VolatileCell<u32>,
}
#[doc = "CoreSight Trace ID Register"]
pub mod etmtraceidr;
#[doc = "ETM ID Register 2"]
pub struct ETMIDR2 {
    register: VolatileCell<u32>,
}
#[doc = "ETM ID Register 2"]
pub mod etmidr2;
#[doc = "Device Power-down Status Register"]
pub struct ETMPDSR {
    register: VolatileCell<u32>,
}
#[doc = "Device Power-down Status Register"]
pub mod etmpdsr;
#[doc = "Integration Test Miscellaneous Inputs Register"]
pub struct ETMISCIN {
    register: VolatileCell<u32>,
}
#[doc = "Integration Test Miscellaneous Inputs Register"]
pub mod etmiscin;
#[doc = "Integration Test Trigger Out Register"]
pub struct ITTRIGOUT {
    register: VolatileCell<u32>,
}
#[doc = "Integration Test Trigger Out Register"]
pub mod ittrigout;
#[doc = "ETM Integration Test ATB Control 2 Register"]
pub struct ETMITATBCTR2 {
    register: VolatileCell<u32>,
}
#[doc = "ETM Integration Test ATB Control 2 Register"]
pub mod etmitatbctr2;
#[doc = "ETM Integration Test ATB Control 0 Register"]
pub struct ETMITATBCTR0 {
    register: VolatileCell<u32>,
}
#[doc = "ETM Integration Test ATB Control 0 Register"]
pub mod etmitatbctr0;
#[doc = "ETM Integration Control Register"]
pub struct ETMITCTRL {
    register: VolatileCell<u32>,
}
#[doc = "ETM Integration Control Register"]
pub mod etmitctrl;
#[doc = "ETM Claim Tag Set Register"]
pub struct ETMCLAIMSET {
    register: VolatileCell<u32>,
}
#[doc = "ETM Claim Tag Set Register"]
pub mod etmclaimset;
#[doc = "ETM Claim Tag Clear Register"]
pub struct ETMCLAIMCLR {
    register: VolatileCell<u32>,
}
#[doc = "ETM Claim Tag Clear Register"]
pub mod etmclaimclr;
#[doc = "ETM Lock Access Register"]
pub struct ETMLAR {
    register: VolatileCell<u32>,
}
#[doc = "ETM Lock Access Register"]
pub mod etmlar;
#[doc = "Lock Status Register"]
pub struct ETMLSR {
    register: VolatileCell<u32>,
}
#[doc = "Lock Status Register"]
pub mod etmlsr;
#[doc = "ETM Authentication Status Register"]
pub struct ETMAUTHSTATUS {
    register: VolatileCell<u32>,
}
#[doc = "ETM Authentication Status Register"]
pub mod etmauthstatus;
#[doc = "CoreSight Device Type Register"]
pub struct ETMDEVTYPE {
    register: VolatileCell<u32>,
}
#[doc = "CoreSight Device Type Register"]
pub mod etmdevtype;
#[doc = "Peripheral ID4 Register"]
pub struct ETMPIDR4 {
    register: VolatileCell<u32>,
}
#[doc = "Peripheral ID4 Register"]
pub mod etmpidr4;
#[doc = "Peripheral ID5 Register"]
pub struct ETMPIDR5 {
    register: VolatileCell<u32>,
}
#[doc = "Peripheral ID5 Register"]
pub mod etmpidr5;
#[doc = "Peripheral ID6 Register"]
pub struct ETMPIDR6 {
    register: VolatileCell<u32>,
}
#[doc = "Peripheral ID6 Register"]
pub mod etmpidr6;
#[doc = "Peripheral ID7 Register"]
pub struct ETMPIDR7 {
    register: VolatileCell<u32>,
}
#[doc = "Peripheral ID7 Register"]
pub mod etmpidr7;
#[doc = "Peripheral ID0 Register"]
pub struct ETMPIDR0 {
    register: VolatileCell<u32>,
}
#[doc = "Peripheral ID0 Register"]
pub mod etmpidr0;
#[doc = "Peripheral ID1 Register"]
pub struct ETMPIDR1 {
    register: VolatileCell<u32>,
}
#[doc = "Peripheral ID1 Register"]
pub mod etmpidr1;
#[doc = "Peripheral ID2 Register"]
pub struct ETMPIDR2 {
    register: VolatileCell<u32>,
}
#[doc = "Peripheral ID2 Register"]
pub mod etmpidr2;
#[doc = "Peripheral ID3 Register"]
pub struct ETMPIDR3 {
    register: VolatileCell<u32>,
}
#[doc = "Peripheral ID3 Register"]
pub mod etmpidr3;
#[doc = "Component ID0 Register"]
pub struct ETMCIDR0 {
    register: VolatileCell<u32>,
}
#[doc = "Component ID0 Register"]
pub mod etmcidr0;
#[doc = "Component ID1 Register"]
pub struct ETMCIDR1 {
    register: VolatileCell<u32>,
}
#[doc = "Component ID1 Register"]
pub mod etmcidr1;
#[doc = "Component ID2 Register"]
pub struct ETMCIDR2 {
    register: VolatileCell<u32>,
}
#[doc = "Component ID2 Register"]
pub mod etmcidr2;
#[doc = "Component ID3 Register"]
pub struct ETMCIDR3 {
    register: VolatileCell<u32>,
}
#[doc = "Component ID3 Register"]
pub mod etmcidr3;

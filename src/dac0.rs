use vcell::VolatileCell;
#[doc = r" Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Control Register"]
    pub ctrl: CTRL,
    #[doc = "0x04 - Status Register"]
    pub status: STATUS,
    #[doc = "0x08 - Channel 0 Control Register"]
    pub ch0ctrl: CH0CTRL,
    #[doc = "0x0c - Channel 1 Control Register"]
    pub ch1ctrl: CH1CTRL,
    #[doc = "0x10 - Interrupt Enable Register"]
    pub ien: IEN,
    #[doc = "0x14 - Interrupt Flag Register"]
    pub if_: IF,
    #[doc = "0x18 - Interrupt Flag Set Register"]
    pub ifs: IFS,
    #[doc = "0x1c - Interrupt Flag Clear Register"]
    pub ifc: IFC,
    #[doc = "0x20 - Channel 0 Data Register"]
    pub ch0data: CH0DATA,
    #[doc = "0x24 - Channel 1 Data Register"]
    pub ch1data: CH1DATA,
    #[doc = "0x28 - Combined Data Register"]
    pub combdata: COMBDATA,
    #[doc = "0x2c - Calibration Register"]
    pub cal: CAL,
    #[doc = "0x30 - Bias Programming Register"]
    pub biasprog: BIASPROG,
    _reserved0: [u8; 32usize],
    #[doc = "0x54 - Operational Amplifier Control Register"]
    pub opactrl: OPACTRL,
    #[doc = "0x58 - Operational Amplifier Offset Register"]
    pub opaoffset: OPAOFFSET,
    #[doc = "0x5c - Operational Amplifier Mux Configuration Register"]
    pub opa0mux: OPA0MUX,
    #[doc = "0x60 - Operational Amplifier Mux Configuration Register"]
    pub opa1mux: OPA1MUX,
    #[doc = "0x64 - Operational Amplifier Mux Configuration Register"]
    pub opa2mux: OPA2MUX,
}
#[doc = "Control Register"]
pub struct CTRL {
    register: VolatileCell<u32>,
}
#[doc = "Control Register"]
pub mod ctrl;
#[doc = "Status Register"]
pub struct STATUS {
    register: VolatileCell<u32>,
}
#[doc = "Status Register"]
pub mod status;
#[doc = "Channel 0 Control Register"]
pub struct CH0CTRL {
    register: VolatileCell<u32>,
}
#[doc = "Channel 0 Control Register"]
pub mod ch0ctrl;
#[doc = "Channel 1 Control Register"]
pub struct CH1CTRL {
    register: VolatileCell<u32>,
}
#[doc = "Channel 1 Control Register"]
pub mod ch1ctrl;
#[doc = "Interrupt Enable Register"]
pub struct IEN {
    register: VolatileCell<u32>,
}
#[doc = "Interrupt Enable Register"]
pub mod ien;
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
#[doc = "Channel 0 Data Register"]
pub struct CH0DATA {
    register: VolatileCell<u32>,
}
#[doc = "Channel 0 Data Register"]
pub mod ch0data;
#[doc = "Channel 1 Data Register"]
pub struct CH1DATA {
    register: VolatileCell<u32>,
}
#[doc = "Channel 1 Data Register"]
pub mod ch1data;
#[doc = "Combined Data Register"]
pub struct COMBDATA {
    register: VolatileCell<u32>,
}
#[doc = "Combined Data Register"]
pub mod combdata;
#[doc = "Calibration Register"]
pub struct CAL {
    register: VolatileCell<u32>,
}
#[doc = "Calibration Register"]
pub mod cal;
#[doc = "Bias Programming Register"]
pub struct BIASPROG {
    register: VolatileCell<u32>,
}
#[doc = "Bias Programming Register"]
pub mod biasprog;
#[doc = "Operational Amplifier Control Register"]
pub struct OPACTRL {
    register: VolatileCell<u32>,
}
#[doc = "Operational Amplifier Control Register"]
pub mod opactrl;
#[doc = "Operational Amplifier Offset Register"]
pub struct OPAOFFSET {
    register: VolatileCell<u32>,
}
#[doc = "Operational Amplifier Offset Register"]
pub mod opaoffset;
#[doc = "Operational Amplifier Mux Configuration Register"]
pub struct OPA0MUX {
    register: VolatileCell<u32>,
}
#[doc = "Operational Amplifier Mux Configuration Register"]
pub mod opa0mux;
#[doc = "Operational Amplifier Mux Configuration Register"]
pub struct OPA1MUX {
    register: VolatileCell<u32>,
}
#[doc = "Operational Amplifier Mux Configuration Register"]
pub mod opa1mux;
#[doc = "Operational Amplifier Mux Configuration Register"]
pub struct OPA2MUX {
    register: VolatileCell<u32>,
}
#[doc = "Operational Amplifier Mux Configuration Register"]
pub mod opa2mux;

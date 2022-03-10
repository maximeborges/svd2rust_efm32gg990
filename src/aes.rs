use vcell::VolatileCell;
#[doc = r" Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Control Register"]
    pub ctrl: CTRL,
    #[doc = "0x04 - Command Register"]
    pub cmd: CMD,
    #[doc = "0x08 - Status Register"]
    pub status: STATUS,
    #[doc = "0x0c - Interrupt Enable Register"]
    pub ien: IEN,
    #[doc = "0x10 - Interrupt Flag Register"]
    pub if_: IF,
    #[doc = "0x14 - Interrupt Flag Set Register"]
    pub ifs: IFS,
    #[doc = "0x18 - Interrupt Flag Clear Register"]
    pub ifc: IFC,
    #[doc = "0x1c - DATA Register"]
    pub data: DATA,
    #[doc = "0x20 - XORDATA Register"]
    pub xordata: XORDATA,
    _reserved0: [u8; 12usize],
    #[doc = "0x30 - KEY Low Register"]
    pub keyla: KEYLA,
    #[doc = "0x34 - KEY Low Register"]
    pub keylb: KEYLB,
    #[doc = "0x38 - KEY Low Register"]
    pub keylc: KEYLC,
    #[doc = "0x3c - KEY Low Register"]
    pub keyld: KEYLD,
    #[doc = "0x40 - KEY High Register"]
    pub keyha: KEYHA,
    #[doc = "0x44 - KEY High Register"]
    pub keyhb: KEYHB,
    #[doc = "0x48 - KEY High Register"]
    pub keyhc: KEYHC,
    #[doc = "0x4c - KEY High Register"]
    pub keyhd: KEYHD,
}
#[doc = "Control Register"]
pub struct CTRL {
    register: VolatileCell<u32>,
}
#[doc = "Control Register"]
pub mod ctrl;
#[doc = "Command Register"]
pub struct CMD {
    register: VolatileCell<u32>,
}
#[doc = "Command Register"]
pub mod cmd;
#[doc = "Status Register"]
pub struct STATUS {
    register: VolatileCell<u32>,
}
#[doc = "Status Register"]
pub mod status;
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
#[doc = "DATA Register"]
pub struct DATA {
    register: VolatileCell<u32>,
}
#[doc = "DATA Register"]
pub mod data;
#[doc = "XORDATA Register"]
pub struct XORDATA {
    register: VolatileCell<u32>,
}
#[doc = "XORDATA Register"]
pub mod xordata;
#[doc = "KEY Low Register"]
pub struct KEYLA {
    register: VolatileCell<u32>,
}
#[doc = "KEY Low Register"]
pub mod keyla;
#[doc = "KEY Low Register"]
pub struct KEYLB {
    register: VolatileCell<u32>,
}
#[doc = "KEY Low Register"]
pub mod keylb;
#[doc = "KEY Low Register"]
pub struct KEYLC {
    register: VolatileCell<u32>,
}
#[doc = "KEY Low Register"]
pub mod keylc;
#[doc = "KEY Low Register"]
pub struct KEYLD {
    register: VolatileCell<u32>,
}
#[doc = "KEY Low Register"]
pub mod keyld;
#[doc = "KEY High Register"]
pub struct KEYHA {
    register: VolatileCell<u32>,
}
#[doc = "KEY High Register"]
pub mod keyha;
#[doc = "KEY High Register"]
pub struct KEYHB {
    register: VolatileCell<u32>,
}
#[doc = "KEY High Register"]
pub mod keyhb;
#[doc = "KEY High Register"]
pub struct KEYHC {
    register: VolatileCell<u32>,
}
#[doc = "KEY High Register"]
pub mod keyhc;
#[doc = "KEY High Register"]
pub struct KEYHD {
    register: VolatileCell<u32>,
}
#[doc = "KEY High Register"]
pub mod keyhd;

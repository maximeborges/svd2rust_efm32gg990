use vcell::VolatileCell;
#[doc = r" Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Control Register"]
    pub ctrl: CTRL,
    #[doc = "0x04 - Memory Control Register"]
    pub memctrl: MEMCTRL,
    #[doc = "0x08 - Configuration Lock Register"]
    pub lock: LOCK,
    _reserved0: [u8; 24usize],
    #[doc = "0x24 - Auxiliary Control Register"]
    pub auxctrl: AUXCTRL,
    _reserved1: [u8; 4usize],
    #[doc = "0x2c - Energy mode 4 configuration register"]
    pub em4conf: EM4CONF,
    #[doc = "0x30 - Backup Power configuration register"]
    pub buctrl: BUCTRL,
    #[doc = "0x34 - Power connection configuration register"]
    pub pwrconf: PWRCONF,
    #[doc = "0x38 - Backup mode inactive configuration register"]
    pub buinact: BUINACT,
    #[doc = "0x3c - Backup mode active configuration register"]
    pub buact: BUACT,
    #[doc = "0x40 - Status register"]
    pub status: STATUS,
    #[doc = "0x44 - I/O Routing Register"]
    pub route: ROUTE,
    #[doc = "0x48 - Interrupt Flag Register"]
    pub if_: IF,
    #[doc = "0x4c - Interrupt Flag Set Register"]
    pub ifs: IFS,
    #[doc = "0x50 - Interrupt Flag Clear Register"]
    pub ifc: IFC,
    #[doc = "0x54 - Interrupt Enable Register"]
    pub ien: IEN,
    #[doc = "0x58 - BU_VIN Backup BOD calibration"]
    pub bubodbuvincal: BUBODBUVINCAL,
    #[doc = "0x5c - Unregulated power Backup BOD calibration"]
    pub bubodunregcal: BUBODUNREGCAL,
}
#[doc = "Control Register"]
pub struct CTRL {
    register: VolatileCell<u32>,
}
#[doc = "Control Register"]
pub mod ctrl;
#[doc = "Memory Control Register"]
pub struct MEMCTRL {
    register: VolatileCell<u32>,
}
#[doc = "Memory Control Register"]
pub mod memctrl;
#[doc = "Configuration Lock Register"]
pub struct LOCK {
    register: VolatileCell<u32>,
}
#[doc = "Configuration Lock Register"]
pub mod lock;
#[doc = "Auxiliary Control Register"]
pub struct AUXCTRL {
    register: VolatileCell<u32>,
}
#[doc = "Auxiliary Control Register"]
pub mod auxctrl;
#[doc = "Energy mode 4 configuration register"]
pub struct EM4CONF {
    register: VolatileCell<u32>,
}
#[doc = "Energy mode 4 configuration register"]
pub mod em4conf;
#[doc = "Backup Power configuration register"]
pub struct BUCTRL {
    register: VolatileCell<u32>,
}
#[doc = "Backup Power configuration register"]
pub mod buctrl;
#[doc = "Power connection configuration register"]
pub struct PWRCONF {
    register: VolatileCell<u32>,
}
#[doc = "Power connection configuration register"]
pub mod pwrconf;
#[doc = "Backup mode inactive configuration register"]
pub struct BUINACT {
    register: VolatileCell<u32>,
}
#[doc = "Backup mode inactive configuration register"]
pub mod buinact;
#[doc = "Backup mode active configuration register"]
pub struct BUACT {
    register: VolatileCell<u32>,
}
#[doc = "Backup mode active configuration register"]
pub mod buact;
#[doc = "Status register"]
pub struct STATUS {
    register: VolatileCell<u32>,
}
#[doc = "Status register"]
pub mod status;
#[doc = "I/O Routing Register"]
pub struct ROUTE {
    register: VolatileCell<u32>,
}
#[doc = "I/O Routing Register"]
pub mod route;
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
#[doc = "Interrupt Enable Register"]
pub struct IEN {
    register: VolatileCell<u32>,
}
#[doc = "Interrupt Enable Register"]
pub mod ien;
#[doc = "BU_VIN Backup BOD calibration"]
pub struct BUBODBUVINCAL {
    register: VolatileCell<u32>,
}
#[doc = "BU_VIN Backup BOD calibration"]
pub mod bubodbuvincal;
#[doc = "Unregulated power Backup BOD calibration"]
pub struct BUBODUNREGCAL {
    register: VolatileCell<u32>,
}
#[doc = "Unregulated power Backup BOD calibration"]
pub mod bubodunregcal;

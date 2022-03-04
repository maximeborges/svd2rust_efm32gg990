use vcell::VolatileCell;
#[doc = r" Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - CMU Control Register"]
    pub ctrl: CTRL,
    #[doc = "0x04 - High Frequency Core Clock Division Register"]
    pub hfcoreclkdiv: HFCORECLKDIV,
    #[doc = "0x08 - High Frequency Peripheral Clock Division Register"]
    pub hfperclkdiv: HFPERCLKDIV,
    #[doc = "0x0c - HFRCO Control Register"]
    pub hfrcoctrl: HFRCOCTRL,
    #[doc = "0x10 - LFRCO Control Register"]
    pub lfrcoctrl: LFRCOCTRL,
    #[doc = "0x14 - AUXHFRCO Control Register"]
    pub auxhfrcoctrl: AUXHFRCOCTRL,
    #[doc = "0x18 - Calibration Control Register"]
    pub calctrl: CALCTRL,
    #[doc = "0x1c - Calibration Counter Register"]
    pub calcnt: CALCNT,
    #[doc = "0x20 - Oscillator Enable/Disable Command Register"]
    pub oscencmd: OSCENCMD,
    #[doc = "0x24 - Command Register"]
    pub cmd: CMD,
    #[doc = "0x28 - Low Frequency Clock Select Register"]
    pub lfclksel: LFCLKSEL,
    #[doc = "0x2c - Status Register"]
    pub status: STATUS,
    #[doc = "0x30 - Interrupt Flag Register"]
    pub if_: IF,
    #[doc = "0x34 - Interrupt Flag Set Register"]
    pub ifs: IFS,
    #[doc = "0x38 - Interrupt Flag Clear Register"]
    pub ifc: IFC,
    #[doc = "0x3c - Interrupt Enable Register"]
    pub ien: IEN,
    #[doc = "0x40 - High Frequency Core Clock Enable Register 0"]
    pub hfcoreclken0: HFCORECLKEN0,
    #[doc = "0x44 - High Frequency Peripheral Clock Enable Register 0"]
    pub hfperclken0: HFPERCLKEN0,
    _reserved0: [u8; 8usize],
    #[doc = "0x50 - Synchronization Busy Register"]
    pub syncbusy: SYNCBUSY,
    #[doc = "0x54 - Freeze Register"]
    pub freeze: FREEZE,
    #[doc = "0x58 - Low Frequency A Clock Enable Register 0 (Async Reg)"]
    pub lfaclken0: LFACLKEN0,
    _reserved1: [u8; 4usize],
    #[doc = "0x60 - Low Frequency B Clock Enable Register 0 (Async Reg)"]
    pub lfbclken0: LFBCLKEN0,
    _reserved2: [u8; 4usize],
    #[doc = "0x68 - Low Frequency A Prescaler Register 0 (Async Reg)"]
    pub lfapresc0: LFAPRESC0,
    _reserved3: [u8; 4usize],
    #[doc = "0x70 - Low Frequency B Prescaler Register 0 (Async Reg)"]
    pub lfbpresc0: LFBPRESC0,
    _reserved4: [u8; 4usize],
    #[doc = "0x78 - PCNT Control Register"]
    pub pcntctrl: PCNTCTRL,
    #[doc = "0x7c - LCD Control Register"]
    pub lcdctrl: LCDCTRL,
    #[doc = "0x80 - I/O Routing Register"]
    pub route: ROUTE,
    #[doc = "0x84 - Configuration Lock Register"]
    pub lock: LOCK,
}
#[doc = "CMU Control Register"]
pub struct CTRL {
    register: VolatileCell<u32>,
}
#[doc = "CMU Control Register"]
pub mod ctrl;
#[doc = "High Frequency Core Clock Division Register"]
pub struct HFCORECLKDIV {
    register: VolatileCell<u32>,
}
#[doc = "High Frequency Core Clock Division Register"]
pub mod hfcoreclkdiv;
#[doc = "High Frequency Peripheral Clock Division Register"]
pub struct HFPERCLKDIV {
    register: VolatileCell<u32>,
}
#[doc = "High Frequency Peripheral Clock Division Register"]
pub mod hfperclkdiv;
#[doc = "HFRCO Control Register"]
pub struct HFRCOCTRL {
    register: VolatileCell<u32>,
}
#[doc = "HFRCO Control Register"]
pub mod hfrcoctrl;
#[doc = "LFRCO Control Register"]
pub struct LFRCOCTRL {
    register: VolatileCell<u32>,
}
#[doc = "LFRCO Control Register"]
pub mod lfrcoctrl;
#[doc = "AUXHFRCO Control Register"]
pub struct AUXHFRCOCTRL {
    register: VolatileCell<u32>,
}
#[doc = "AUXHFRCO Control Register"]
pub mod auxhfrcoctrl;
#[doc = "Calibration Control Register"]
pub struct CALCTRL {
    register: VolatileCell<u32>,
}
#[doc = "Calibration Control Register"]
pub mod calctrl;
#[doc = "Calibration Counter Register"]
pub struct CALCNT {
    register: VolatileCell<u32>,
}
#[doc = "Calibration Counter Register"]
pub mod calcnt;
#[doc = "Oscillator Enable/Disable Command Register"]
pub struct OSCENCMD {
    register: VolatileCell<u32>,
}
#[doc = "Oscillator Enable/Disable Command Register"]
pub mod oscencmd;
#[doc = "Command Register"]
pub struct CMD {
    register: VolatileCell<u32>,
}
#[doc = "Command Register"]
pub mod cmd;
#[doc = "Low Frequency Clock Select Register"]
pub struct LFCLKSEL {
    register: VolatileCell<u32>,
}
#[doc = "Low Frequency Clock Select Register"]
pub mod lfclksel;
#[doc = "Status Register"]
pub struct STATUS {
    register: VolatileCell<u32>,
}
#[doc = "Status Register"]
pub mod status;
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
#[doc = "High Frequency Core Clock Enable Register 0"]
pub struct HFCORECLKEN0 {
    register: VolatileCell<u32>,
}
#[doc = "High Frequency Core Clock Enable Register 0"]
pub mod hfcoreclken0;
#[doc = "High Frequency Peripheral Clock Enable Register 0"]
pub struct HFPERCLKEN0 {
    register: VolatileCell<u32>,
}
#[doc = "High Frequency Peripheral Clock Enable Register 0"]
pub mod hfperclken0;
#[doc = "Synchronization Busy Register"]
pub struct SYNCBUSY {
    register: VolatileCell<u32>,
}
#[doc = "Synchronization Busy Register"]
pub mod syncbusy;
#[doc = "Freeze Register"]
pub struct FREEZE {
    register: VolatileCell<u32>,
}
#[doc = "Freeze Register"]
pub mod freeze;
#[doc = "Low Frequency A Clock Enable Register 0 (Async Reg)"]
pub struct LFACLKEN0 {
    register: VolatileCell<u32>,
}
#[doc = "Low Frequency A Clock Enable Register 0 (Async Reg)"]
pub mod lfaclken0;
#[doc = "Low Frequency B Clock Enable Register 0 (Async Reg)"]
pub struct LFBCLKEN0 {
    register: VolatileCell<u32>,
}
#[doc = "Low Frequency B Clock Enable Register 0 (Async Reg)"]
pub mod lfbclken0;
#[doc = "Low Frequency A Prescaler Register 0 (Async Reg)"]
pub struct LFAPRESC0 {
    register: VolatileCell<u32>,
}
#[doc = "Low Frequency A Prescaler Register 0 (Async Reg)"]
pub mod lfapresc0;
#[doc = "Low Frequency B Prescaler Register 0 (Async Reg)"]
pub struct LFBPRESC0 {
    register: VolatileCell<u32>,
}
#[doc = "Low Frequency B Prescaler Register 0 (Async Reg)"]
pub mod lfbpresc0;
#[doc = "PCNT Control Register"]
pub struct PCNTCTRL {
    register: VolatileCell<u32>,
}
#[doc = "PCNT Control Register"]
pub mod pcntctrl;
#[doc = "LCD Control Register"]
pub struct LCDCTRL {
    register: VolatileCell<u32>,
}
#[doc = "LCD Control Register"]
pub mod lcdctrl;
#[doc = "I/O Routing Register"]
pub struct ROUTE {
    register: VolatileCell<u32>,
}
#[doc = "I/O Routing Register"]
pub mod route;
#[doc = "Configuration Lock Register"]
pub struct LOCK {
    register: VolatileCell<u32>,
}
#[doc = "Configuration Lock Register"]
pub mod lock;

use vcell::VolatileCell;
#[doc = r" Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Software Pulse Register"]
    pub swpulse: SWPULSE,
    #[doc = "0x04 - Software Level Register"]
    pub swlevel: SWLEVEL,
    #[doc = "0x08 - I/O Routing Register"]
    pub route: ROUTE,
    _reserved0: [u8; 4usize],
    #[doc = "0x10 - Channel Control Register"]
    pub ch0_ctrl: CH0_CTRL,
    #[doc = "0x14 - Channel Control Register"]
    pub ch1_ctrl: CH1_CTRL,
    #[doc = "0x18 - Channel Control Register"]
    pub ch2_ctrl: CH2_CTRL,
    #[doc = "0x1c - Channel Control Register"]
    pub ch3_ctrl: CH3_CTRL,
    #[doc = "0x20 - Channel Control Register"]
    pub ch4_ctrl: CH4_CTRL,
    #[doc = "0x24 - Channel Control Register"]
    pub ch5_ctrl: CH5_CTRL,
    #[doc = "0x28 - Channel Control Register"]
    pub ch6_ctrl: CH6_CTRL,
    #[doc = "0x2c - Channel Control Register"]
    pub ch7_ctrl: CH7_CTRL,
    #[doc = "0x30 - Channel Control Register"]
    pub ch8_ctrl: CH8_CTRL,
    #[doc = "0x34 - Channel Control Register"]
    pub ch9_ctrl: CH9_CTRL,
    #[doc = "0x38 - Channel Control Register"]
    pub ch10_ctrl: CH10_CTRL,
    #[doc = "0x3c - Channel Control Register"]
    pub ch11_ctrl: CH11_CTRL,
}
#[doc = "Software Pulse Register"]
pub struct SWPULSE {
    register: VolatileCell<u32>,
}
#[doc = "Software Pulse Register"]
pub mod swpulse;
#[doc = "Software Level Register"]
pub struct SWLEVEL {
    register: VolatileCell<u32>,
}
#[doc = "Software Level Register"]
pub mod swlevel;
#[doc = "I/O Routing Register"]
pub struct ROUTE {
    register: VolatileCell<u32>,
}
#[doc = "I/O Routing Register"]
pub mod route;
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

use vcell::VolatileCell;
#[doc = r" Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Control Register"]
    pub ctrl: CTRL,
    #[doc = "0x04 - Display Control Register"]
    pub dispctrl: DISPCTRL,
    #[doc = "0x08 - Segment Enable Register"]
    pub segen: SEGEN,
    #[doc = "0x0c - Blink and Animation Control Register"]
    pub bactrl: BACTRL,
    #[doc = "0x10 - Status Register"]
    pub status: STATUS,
    #[doc = "0x14 - Animation Register A"]
    pub arega: AREGA,
    #[doc = "0x18 - Animation Register B"]
    pub aregb: AREGB,
    #[doc = "0x1c - Interrupt Flag Register"]
    pub if_: IF,
    #[doc = "0x20 - Interrupt Flag Set Register"]
    pub ifs: IFS,
    #[doc = "0x24 - Interrupt Flag Clear Register"]
    pub ifc: IFC,
    #[doc = "0x28 - Interrupt Enable Register"]
    pub ien: IEN,
    _reserved0: [u8; 20usize],
    #[doc = "0x40 - Segment Data Low Register 0"]
    pub segd0l: SEGD0L,
    #[doc = "0x44 - Segment Data Low Register 1"]
    pub segd1l: SEGD1L,
    #[doc = "0x48 - Segment Data Low Register 2"]
    pub segd2l: SEGD2L,
    #[doc = "0x4c - Segment Data Low Register 3"]
    pub segd3l: SEGD3L,
    #[doc = "0x50 - Segment Data High Register 0"]
    pub segd0h: SEGD0H,
    #[doc = "0x54 - Segment Data High Register 1"]
    pub segd1h: SEGD1H,
    #[doc = "0x58 - Segment Data High Register 2"]
    pub segd2h: SEGD2H,
    #[doc = "0x5c - Segment Data High Register 3"]
    pub segd3h: SEGD3H,
    #[doc = "0x60 - Freeze Register"]
    pub freeze: FREEZE,
    #[doc = "0x64 - Synchronization Busy Register"]
    pub syncbusy: SYNCBUSY,
    _reserved1: [u8; 76usize],
    #[doc = "0xb4 - Segment Data High Register 4"]
    pub segd4h: SEGD4H,
    #[doc = "0xb8 - Segment Data High Register 5"]
    pub segd5h: SEGD5H,
    #[doc = "0xbc - Segment Data High Register 6"]
    pub segd6h: SEGD6H,
    #[doc = "0xc0 - Segment Data High Register 7"]
    pub segd7h: SEGD7H,
    _reserved2: [u8; 8usize],
    #[doc = "0xcc - Segment Data Low Register 4"]
    pub segd4l: SEGD4L,
    #[doc = "0xd0 - Segment Data Low Register 5"]
    pub segd5l: SEGD5L,
    #[doc = "0xd4 - Segment Data Low Register 6"]
    pub segd6l: SEGD6L,
    #[doc = "0xd8 - Segment Data Low Register 7"]
    pub segd7l: SEGD7L,
}
#[doc = "Control Register"]
pub struct CTRL {
    register: VolatileCell<u32>,
}
#[doc = "Control Register"]
pub mod ctrl;
#[doc = "Display Control Register"]
pub struct DISPCTRL {
    register: VolatileCell<u32>,
}
#[doc = "Display Control Register"]
pub mod dispctrl;
#[doc = "Segment Enable Register"]
pub struct SEGEN {
    register: VolatileCell<u32>,
}
#[doc = "Segment Enable Register"]
pub mod segen;
#[doc = "Blink and Animation Control Register"]
pub struct BACTRL {
    register: VolatileCell<u32>,
}
#[doc = "Blink and Animation Control Register"]
pub mod bactrl;
#[doc = "Status Register"]
pub struct STATUS {
    register: VolatileCell<u32>,
}
#[doc = "Status Register"]
pub mod status;
#[doc = "Animation Register A"]
pub struct AREGA {
    register: VolatileCell<u32>,
}
#[doc = "Animation Register A"]
pub mod arega;
#[doc = "Animation Register B"]
pub struct AREGB {
    register: VolatileCell<u32>,
}
#[doc = "Animation Register B"]
pub mod aregb;
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
#[doc = "Segment Data Low Register 0"]
pub struct SEGD0L {
    register: VolatileCell<u32>,
}
#[doc = "Segment Data Low Register 0"]
pub mod segd0l;
#[doc = "Segment Data Low Register 1"]
pub struct SEGD1L {
    register: VolatileCell<u32>,
}
#[doc = "Segment Data Low Register 1"]
pub mod segd1l;
#[doc = "Segment Data Low Register 2"]
pub struct SEGD2L {
    register: VolatileCell<u32>,
}
#[doc = "Segment Data Low Register 2"]
pub mod segd2l;
#[doc = "Segment Data Low Register 3"]
pub struct SEGD3L {
    register: VolatileCell<u32>,
}
#[doc = "Segment Data Low Register 3"]
pub mod segd3l;
#[doc = "Segment Data High Register 0"]
pub struct SEGD0H {
    register: VolatileCell<u32>,
}
#[doc = "Segment Data High Register 0"]
pub mod segd0h;
#[doc = "Segment Data High Register 1"]
pub struct SEGD1H {
    register: VolatileCell<u32>,
}
#[doc = "Segment Data High Register 1"]
pub mod segd1h;
#[doc = "Segment Data High Register 2"]
pub struct SEGD2H {
    register: VolatileCell<u32>,
}
#[doc = "Segment Data High Register 2"]
pub mod segd2h;
#[doc = "Segment Data High Register 3"]
pub struct SEGD3H {
    register: VolatileCell<u32>,
}
#[doc = "Segment Data High Register 3"]
pub mod segd3h;
#[doc = "Freeze Register"]
pub struct FREEZE {
    register: VolatileCell<u32>,
}
#[doc = "Freeze Register"]
pub mod freeze;
#[doc = "Synchronization Busy Register"]
pub struct SYNCBUSY {
    register: VolatileCell<u32>,
}
#[doc = "Synchronization Busy Register"]
pub mod syncbusy;
#[doc = "Segment Data High Register 4"]
pub struct SEGD4H {
    register: VolatileCell<u32>,
}
#[doc = "Segment Data High Register 4"]
pub mod segd4h;
#[doc = "Segment Data High Register 5"]
pub struct SEGD5H {
    register: VolatileCell<u32>,
}
#[doc = "Segment Data High Register 5"]
pub mod segd5h;
#[doc = "Segment Data High Register 6"]
pub struct SEGD6H {
    register: VolatileCell<u32>,
}
#[doc = "Segment Data High Register 6"]
pub mod segd6h;
#[doc = "Segment Data High Register 7"]
pub struct SEGD7H {
    register: VolatileCell<u32>,
}
#[doc = "Segment Data High Register 7"]
pub mod segd7h;
#[doc = "Segment Data Low Register 4"]
pub struct SEGD4L {
    register: VolatileCell<u32>,
}
#[doc = "Segment Data Low Register 4"]
pub mod segd4l;
#[doc = "Segment Data Low Register 5"]
pub struct SEGD5L {
    register: VolatileCell<u32>,
}
#[doc = "Segment Data Low Register 5"]
pub mod segd5l;
#[doc = "Segment Data Low Register 6"]
pub struct SEGD6L {
    register: VolatileCell<u32>,
}
#[doc = "Segment Data Low Register 6"]
pub mod segd6l;
#[doc = "Segment Data Low Register 7"]
pub struct SEGD7L {
    register: VolatileCell<u32>,
}
#[doc = "Segment Data Low Register 7"]
pub mod segd7l;

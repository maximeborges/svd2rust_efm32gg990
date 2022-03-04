use vcell::VolatileCell;
#[doc = r" Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Control Register"]
    pub ctrl: CTRL,
    #[doc = "0x04 - Address Timing Register"]
    pub addrtiming: ADDRTIMING,
    #[doc = "0x08 - Read Timing Register"]
    pub rdtiming: RDTIMING,
    #[doc = "0x0c - Write Timing Register"]
    pub wrtiming: WRTIMING,
    #[doc = "0x10 - Polarity Register"]
    pub polarity: POLARITY,
    #[doc = "0x14 - I/O Routing Register"]
    pub route: ROUTE,
    #[doc = "0x18 - Address Timing Register 1"]
    pub addrtiming1: ADDRTIMING1,
    #[doc = "0x1c - Read Timing Register 1"]
    pub rdtiming1: RDTIMING1,
    #[doc = "0x20 - Write Timing Register 1"]
    pub wrtiming1: WRTIMING1,
    #[doc = "0x24 - Polarity Register 1"]
    pub polarity1: POLARITY1,
    #[doc = "0x28 - Address Timing Register 2"]
    pub addrtiming2: ADDRTIMING2,
    #[doc = "0x2c - Read Timing Register 2"]
    pub rdtiming2: RDTIMING2,
    #[doc = "0x30 - Write Timing Register 2"]
    pub wrtiming2: WRTIMING2,
    #[doc = "0x34 - Polarity Register 2"]
    pub polarity2: POLARITY2,
    #[doc = "0x38 - Address Timing Register 3"]
    pub addrtiming3: ADDRTIMING3,
    #[doc = "0x3c - Read Timing Register 3"]
    pub rdtiming3: RDTIMING3,
    #[doc = "0x40 - Write Timing Register 3"]
    pub wrtiming3: WRTIMING3,
    #[doc = "0x44 - Polarity Register 3"]
    pub polarity3: POLARITY3,
    #[doc = "0x48 - Page Control Register"]
    pub pagectrl: PAGECTRL,
    #[doc = "0x4c - NAND Control Register"]
    pub nandctrl: NANDCTRL,
    #[doc = "0x50 - Command Register"]
    pub cmd: CMD,
    #[doc = "0x54 - Status Register"]
    pub status: STATUS,
    #[doc = "0x58 - ECC Parity register"]
    pub eccparity: ECCPARITY,
    #[doc = "0x5c - TFT Control Register"]
    pub tftctrl: TFTCTRL,
    #[doc = "0x60 - TFT Status Register"]
    pub tftstatus: TFTSTATUS,
    #[doc = "0x64 - TFT Frame Base Register"]
    pub tftframebase: TFTFRAMEBASE,
    #[doc = "0x68 - TFT Stride Register"]
    pub tftstride: TFTSTRIDE,
    #[doc = "0x6c - TFT Size Register"]
    pub tftsize: TFTSIZE,
    #[doc = "0x70 - TFT Horizontal Porch Register"]
    pub tfthporch: TFTHPORCH,
    #[doc = "0x74 - TFT Vertical Porch Register"]
    pub tftvporch: TFTVPORCH,
    #[doc = "0x78 - TFT Timing Register"]
    pub tfttiming: TFTTIMING,
    #[doc = "0x7c - TFT Polarity Register"]
    pub tftpolarity: TFTPOLARITY,
    #[doc = "0x80 - TFT Direct Drive Data Register"]
    pub tftdd: TFTDD,
    #[doc = "0x84 - TFT Alpha Blending Register"]
    pub tftalpha: TFTALPHA,
    #[doc = "0x88 - TFT Pixel 0 Register"]
    pub tftpixel0: TFTPIXEL0,
    #[doc = "0x8c - TFT Pixel 1 Register"]
    pub tftpixel1: TFTPIXEL1,
    #[doc = "0x90 - TFT Alpha Blending Result Pixel Register"]
    pub tftpixel: TFTPIXEL,
    #[doc = "0x94 - TFT Masking Register"]
    pub tftmask: TFTMASK,
    #[doc = "0x98 - Interrupt Flag Register"]
    pub if_: IF,
    #[doc = "0x9c - Interrupt Flag Set Register"]
    pub ifs: IFS,
    #[doc = "0xa0 - Interrupt Flag Clear Register"]
    pub ifc: IFC,
    #[doc = "0xa4 - Interrupt Enable Register"]
    pub ien: IEN,
}
#[doc = "Control Register"]
pub struct CTRL {
    register: VolatileCell<u32>,
}
#[doc = "Control Register"]
pub mod ctrl;
#[doc = "Address Timing Register"]
pub struct ADDRTIMING {
    register: VolatileCell<u32>,
}
#[doc = "Address Timing Register"]
pub mod addrtiming;
#[doc = "Read Timing Register"]
pub struct RDTIMING {
    register: VolatileCell<u32>,
}
#[doc = "Read Timing Register"]
pub mod rdtiming;
#[doc = "Write Timing Register"]
pub struct WRTIMING {
    register: VolatileCell<u32>,
}
#[doc = "Write Timing Register"]
pub mod wrtiming;
#[doc = "Polarity Register"]
pub struct POLARITY {
    register: VolatileCell<u32>,
}
#[doc = "Polarity Register"]
pub mod polarity;
#[doc = "I/O Routing Register"]
pub struct ROUTE {
    register: VolatileCell<u32>,
}
#[doc = "I/O Routing Register"]
pub mod route;
#[doc = "Address Timing Register 1"]
pub struct ADDRTIMING1 {
    register: VolatileCell<u32>,
}
#[doc = "Address Timing Register 1"]
pub mod addrtiming1;
#[doc = "Read Timing Register 1"]
pub struct RDTIMING1 {
    register: VolatileCell<u32>,
}
#[doc = "Read Timing Register 1"]
pub mod rdtiming1;
#[doc = "Write Timing Register 1"]
pub struct WRTIMING1 {
    register: VolatileCell<u32>,
}
#[doc = "Write Timing Register 1"]
pub mod wrtiming1;
#[doc = "Polarity Register 1"]
pub struct POLARITY1 {
    register: VolatileCell<u32>,
}
#[doc = "Polarity Register 1"]
pub mod polarity1;
#[doc = "Address Timing Register 2"]
pub struct ADDRTIMING2 {
    register: VolatileCell<u32>,
}
#[doc = "Address Timing Register 2"]
pub mod addrtiming2;
#[doc = "Read Timing Register 2"]
pub struct RDTIMING2 {
    register: VolatileCell<u32>,
}
#[doc = "Read Timing Register 2"]
pub mod rdtiming2;
#[doc = "Write Timing Register 2"]
pub struct WRTIMING2 {
    register: VolatileCell<u32>,
}
#[doc = "Write Timing Register 2"]
pub mod wrtiming2;
#[doc = "Polarity Register 2"]
pub struct POLARITY2 {
    register: VolatileCell<u32>,
}
#[doc = "Polarity Register 2"]
pub mod polarity2;
#[doc = "Address Timing Register 3"]
pub struct ADDRTIMING3 {
    register: VolatileCell<u32>,
}
#[doc = "Address Timing Register 3"]
pub mod addrtiming3;
#[doc = "Read Timing Register 3"]
pub struct RDTIMING3 {
    register: VolatileCell<u32>,
}
#[doc = "Read Timing Register 3"]
pub mod rdtiming3;
#[doc = "Write Timing Register 3"]
pub struct WRTIMING3 {
    register: VolatileCell<u32>,
}
#[doc = "Write Timing Register 3"]
pub mod wrtiming3;
#[doc = "Polarity Register 3"]
pub struct POLARITY3 {
    register: VolatileCell<u32>,
}
#[doc = "Polarity Register 3"]
pub mod polarity3;
#[doc = "Page Control Register"]
pub struct PAGECTRL {
    register: VolatileCell<u32>,
}
#[doc = "Page Control Register"]
pub mod pagectrl;
#[doc = "NAND Control Register"]
pub struct NANDCTRL {
    register: VolatileCell<u32>,
}
#[doc = "NAND Control Register"]
pub mod nandctrl;
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
#[doc = "ECC Parity register"]
pub struct ECCPARITY {
    register: VolatileCell<u32>,
}
#[doc = "ECC Parity register"]
pub mod eccparity;
#[doc = "TFT Control Register"]
pub struct TFTCTRL {
    register: VolatileCell<u32>,
}
#[doc = "TFT Control Register"]
pub mod tftctrl;
#[doc = "TFT Status Register"]
pub struct TFTSTATUS {
    register: VolatileCell<u32>,
}
#[doc = "TFT Status Register"]
pub mod tftstatus;
#[doc = "TFT Frame Base Register"]
pub struct TFTFRAMEBASE {
    register: VolatileCell<u32>,
}
#[doc = "TFT Frame Base Register"]
pub mod tftframebase;
#[doc = "TFT Stride Register"]
pub struct TFTSTRIDE {
    register: VolatileCell<u32>,
}
#[doc = "TFT Stride Register"]
pub mod tftstride;
#[doc = "TFT Size Register"]
pub struct TFTSIZE {
    register: VolatileCell<u32>,
}
#[doc = "TFT Size Register"]
pub mod tftsize;
#[doc = "TFT Horizontal Porch Register"]
pub struct TFTHPORCH {
    register: VolatileCell<u32>,
}
#[doc = "TFT Horizontal Porch Register"]
pub mod tfthporch;
#[doc = "TFT Vertical Porch Register"]
pub struct TFTVPORCH {
    register: VolatileCell<u32>,
}
#[doc = "TFT Vertical Porch Register"]
pub mod tftvporch;
#[doc = "TFT Timing Register"]
pub struct TFTTIMING {
    register: VolatileCell<u32>,
}
#[doc = "TFT Timing Register"]
pub mod tfttiming;
#[doc = "TFT Polarity Register"]
pub struct TFTPOLARITY {
    register: VolatileCell<u32>,
}
#[doc = "TFT Polarity Register"]
pub mod tftpolarity;
#[doc = "TFT Direct Drive Data Register"]
pub struct TFTDD {
    register: VolatileCell<u32>,
}
#[doc = "TFT Direct Drive Data Register"]
pub mod tftdd;
#[doc = "TFT Alpha Blending Register"]
pub struct TFTALPHA {
    register: VolatileCell<u32>,
}
#[doc = "TFT Alpha Blending Register"]
pub mod tftalpha;
#[doc = "TFT Pixel 0 Register"]
pub struct TFTPIXEL0 {
    register: VolatileCell<u32>,
}
#[doc = "TFT Pixel 0 Register"]
pub mod tftpixel0;
#[doc = "TFT Pixel 1 Register"]
pub struct TFTPIXEL1 {
    register: VolatileCell<u32>,
}
#[doc = "TFT Pixel 1 Register"]
pub mod tftpixel1;
#[doc = "TFT Alpha Blending Result Pixel Register"]
pub struct TFTPIXEL {
    register: VolatileCell<u32>,
}
#[doc = "TFT Alpha Blending Result Pixel Register"]
pub mod tftpixel;
#[doc = "TFT Masking Register"]
pub struct TFTMASK {
    register: VolatileCell<u32>,
}
#[doc = "TFT Masking Register"]
pub mod tftmask;
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

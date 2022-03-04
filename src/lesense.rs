use vcell::VolatileCell;
#[doc = r" Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Control Register"]
    pub ctrl: CTRL,
    #[doc = "0x04 - Timing Control Register"]
    pub timctrl: TIMCTRL,
    #[doc = "0x08 - Peripheral Control Register"]
    pub perctrl: PERCTRL,
    #[doc = "0x0c - Decoder control Register"]
    pub decctrl: DECCTRL,
    #[doc = "0x10 - Bias Control Register"]
    pub biasctrl: BIASCTRL,
    #[doc = "0x14 - Command Register"]
    pub cmd: CMD,
    #[doc = "0x18 - Channel enable Register"]
    pub chen: CHEN,
    #[doc = "0x1c - Scan result register"]
    pub scanres: SCANRES,
    #[doc = "0x20 - Status Register"]
    pub status: STATUS,
    #[doc = "0x24 - Result buffer pointers"]
    pub ptr: PTR,
    #[doc = "0x28 - Result buffer data register"]
    pub bufdata: BUFDATA,
    #[doc = "0x2c - Current channel index"]
    pub curch: CURCH,
    #[doc = "0x30 - Current decoder state"]
    pub decstate: DECSTATE,
    #[doc = "0x34 - Decoder input register"]
    pub sensorstate: SENSORSTATE,
    #[doc = "0x38 - GPIO Idle phase configuration"]
    pub idleconf: IDLECONF,
    #[doc = "0x3c - Alternative excite pin configuration"]
    pub altexconf: ALTEXCONF,
    #[doc = "0x40 - Interrupt Flag Register"]
    pub if_: IF,
    #[doc = "0x44 - Interrupt Flag Clear Register"]
    pub ifc: IFC,
    #[doc = "0x48 - Interrupt Flag Set Register"]
    pub ifs: IFS,
    #[doc = "0x4c - Interrupt Enable Register"]
    pub ien: IEN,
    #[doc = "0x50 - Synchronization Busy Register"]
    pub syncbusy: SYNCBUSY,
    #[doc = "0x54 - I/O Routing Register"]
    pub route: ROUTE,
    #[doc = "0x58 - LESENSE RAM power-down register"]
    pub powerdown: POWERDOWN,
    _reserved0: [u8; 420usize],
    #[doc = "0x200 - State transition configuration A"]
    pub st0_tconfa: ST0_TCONFA,
    #[doc = "0x204 - State transition configuration B"]
    pub st0_tconfb: ST0_TCONFB,
    #[doc = "0x208 - State transition configuration A"]
    pub st1_tconfa: ST1_TCONFA,
    #[doc = "0x20c - State transition configuration B"]
    pub st1_tconfb: ST1_TCONFB,
    #[doc = "0x210 - State transition configuration A"]
    pub st2_tconfa: ST2_TCONFA,
    #[doc = "0x214 - State transition configuration B"]
    pub st2_tconfb: ST2_TCONFB,
    #[doc = "0x218 - State transition configuration A"]
    pub st3_tconfa: ST3_TCONFA,
    #[doc = "0x21c - State transition configuration B"]
    pub st3_tconfb: ST3_TCONFB,
    #[doc = "0x220 - State transition configuration A"]
    pub st4_tconfa: ST4_TCONFA,
    #[doc = "0x224 - State transition configuration B"]
    pub st4_tconfb: ST4_TCONFB,
    #[doc = "0x228 - State transition configuration A"]
    pub st5_tconfa: ST5_TCONFA,
    #[doc = "0x22c - State transition configuration B"]
    pub st5_tconfb: ST5_TCONFB,
    #[doc = "0x230 - State transition configuration A"]
    pub st6_tconfa: ST6_TCONFA,
    #[doc = "0x234 - State transition configuration B"]
    pub st6_tconfb: ST6_TCONFB,
    #[doc = "0x238 - State transition configuration A"]
    pub st7_tconfa: ST7_TCONFA,
    #[doc = "0x23c - State transition configuration B"]
    pub st7_tconfb: ST7_TCONFB,
    #[doc = "0x240 - State transition configuration A"]
    pub st8_tconfa: ST8_TCONFA,
    #[doc = "0x244 - State transition configuration B"]
    pub st8_tconfb: ST8_TCONFB,
    #[doc = "0x248 - State transition configuration A"]
    pub st9_tconfa: ST9_TCONFA,
    #[doc = "0x24c - State transition configuration B"]
    pub st9_tconfb: ST9_TCONFB,
    #[doc = "0x250 - State transition configuration A"]
    pub st10_tconfa: ST10_TCONFA,
    #[doc = "0x254 - State transition configuration B"]
    pub st10_tconfb: ST10_TCONFB,
    #[doc = "0x258 - State transition configuration A"]
    pub st11_tconfa: ST11_TCONFA,
    #[doc = "0x25c - State transition configuration B"]
    pub st11_tconfb: ST11_TCONFB,
    #[doc = "0x260 - State transition configuration A"]
    pub st12_tconfa: ST12_TCONFA,
    #[doc = "0x264 - State transition configuration B"]
    pub st12_tconfb: ST12_TCONFB,
    #[doc = "0x268 - State transition configuration A"]
    pub st13_tconfa: ST13_TCONFA,
    #[doc = "0x26c - State transition configuration B"]
    pub st13_tconfb: ST13_TCONFB,
    #[doc = "0x270 - State transition configuration A"]
    pub st14_tconfa: ST14_TCONFA,
    #[doc = "0x274 - State transition configuration B"]
    pub st14_tconfb: ST14_TCONFB,
    #[doc = "0x278 - State transition configuration A"]
    pub st15_tconfa: ST15_TCONFA,
    #[doc = "0x27c - State transition configuration B"]
    pub st15_tconfb: ST15_TCONFB,
    #[doc = "0x280 - Scan results"]
    pub buf0_data: BUF0_DATA,
    #[doc = "0x284 - Scan results"]
    pub buf1_data: BUF1_DATA,
    #[doc = "0x288 - Scan results"]
    pub buf2_data: BUF2_DATA,
    #[doc = "0x28c - Scan results"]
    pub buf3_data: BUF3_DATA,
    #[doc = "0x290 - Scan results"]
    pub buf4_data: BUF4_DATA,
    #[doc = "0x294 - Scan results"]
    pub buf5_data: BUF5_DATA,
    #[doc = "0x298 - Scan results"]
    pub buf6_data: BUF6_DATA,
    #[doc = "0x29c - Scan results"]
    pub buf7_data: BUF7_DATA,
    #[doc = "0x2a0 - Scan results"]
    pub buf8_data: BUF8_DATA,
    #[doc = "0x2a4 - Scan results"]
    pub buf9_data: BUF9_DATA,
    #[doc = "0x2a8 - Scan results"]
    pub buf10_data: BUF10_DATA,
    #[doc = "0x2ac - Scan results"]
    pub buf11_data: BUF11_DATA,
    #[doc = "0x2b0 - Scan results"]
    pub buf12_data: BUF12_DATA,
    #[doc = "0x2b4 - Scan results"]
    pub buf13_data: BUF13_DATA,
    #[doc = "0x2b8 - Scan results"]
    pub buf14_data: BUF14_DATA,
    #[doc = "0x2bc - Scan results"]
    pub buf15_data: BUF15_DATA,
    #[doc = "0x2c0 - Scan configuration"]
    pub ch0_timing: CH0_TIMING,
    #[doc = "0x2c4 - Scan configuration"]
    pub ch0_interact: CH0_INTERACT,
    #[doc = "0x2c8 - Scan configuration"]
    pub ch0_eval: CH0_EVAL,
    _reserved1: [u8; 4usize],
    #[doc = "0x2d0 - Scan configuration"]
    pub ch1_timing: CH1_TIMING,
    #[doc = "0x2d4 - Scan configuration"]
    pub ch1_interact: CH1_INTERACT,
    #[doc = "0x2d8 - Scan configuration"]
    pub ch1_eval: CH1_EVAL,
    _reserved2: [u8; 4usize],
    #[doc = "0x2e0 - Scan configuration"]
    pub ch2_timing: CH2_TIMING,
    #[doc = "0x2e4 - Scan configuration"]
    pub ch2_interact: CH2_INTERACT,
    #[doc = "0x2e8 - Scan configuration"]
    pub ch2_eval: CH2_EVAL,
    _reserved3: [u8; 4usize],
    #[doc = "0x2f0 - Scan configuration"]
    pub ch3_timing: CH3_TIMING,
    #[doc = "0x2f4 - Scan configuration"]
    pub ch3_interact: CH3_INTERACT,
    #[doc = "0x2f8 - Scan configuration"]
    pub ch3_eval: CH3_EVAL,
    _reserved4: [u8; 4usize],
    #[doc = "0x300 - Scan configuration"]
    pub ch4_timing: CH4_TIMING,
    #[doc = "0x304 - Scan configuration"]
    pub ch4_interact: CH4_INTERACT,
    #[doc = "0x308 - Scan configuration"]
    pub ch4_eval: CH4_EVAL,
    _reserved5: [u8; 4usize],
    #[doc = "0x310 - Scan configuration"]
    pub ch5_timing: CH5_TIMING,
    #[doc = "0x314 - Scan configuration"]
    pub ch5_interact: CH5_INTERACT,
    #[doc = "0x318 - Scan configuration"]
    pub ch5_eval: CH5_EVAL,
    _reserved6: [u8; 4usize],
    #[doc = "0x320 - Scan configuration"]
    pub ch6_timing: CH6_TIMING,
    #[doc = "0x324 - Scan configuration"]
    pub ch6_interact: CH6_INTERACT,
    #[doc = "0x328 - Scan configuration"]
    pub ch6_eval: CH6_EVAL,
    _reserved7: [u8; 4usize],
    #[doc = "0x330 - Scan configuration"]
    pub ch7_timing: CH7_TIMING,
    #[doc = "0x334 - Scan configuration"]
    pub ch7_interact: CH7_INTERACT,
    #[doc = "0x338 - Scan configuration"]
    pub ch7_eval: CH7_EVAL,
    _reserved8: [u8; 4usize],
    #[doc = "0x340 - Scan configuration"]
    pub ch8_timing: CH8_TIMING,
    #[doc = "0x344 - Scan configuration"]
    pub ch8_interact: CH8_INTERACT,
    #[doc = "0x348 - Scan configuration"]
    pub ch8_eval: CH8_EVAL,
    _reserved9: [u8; 4usize],
    #[doc = "0x350 - Scan configuration"]
    pub ch9_timing: CH9_TIMING,
    #[doc = "0x354 - Scan configuration"]
    pub ch9_interact: CH9_INTERACT,
    #[doc = "0x358 - Scan configuration"]
    pub ch9_eval: CH9_EVAL,
    _reserved10: [u8; 4usize],
    #[doc = "0x360 - Scan configuration"]
    pub ch10_timing: CH10_TIMING,
    #[doc = "0x364 - Scan configuration"]
    pub ch10_interact: CH10_INTERACT,
    #[doc = "0x368 - Scan configuration"]
    pub ch10_eval: CH10_EVAL,
    _reserved11: [u8; 4usize],
    #[doc = "0x370 - Scan configuration"]
    pub ch11_timing: CH11_TIMING,
    #[doc = "0x374 - Scan configuration"]
    pub ch11_interact: CH11_INTERACT,
    #[doc = "0x378 - Scan configuration"]
    pub ch11_eval: CH11_EVAL,
    _reserved12: [u8; 4usize],
    #[doc = "0x380 - Scan configuration"]
    pub ch12_timing: CH12_TIMING,
    #[doc = "0x384 - Scan configuration"]
    pub ch12_interact: CH12_INTERACT,
    #[doc = "0x388 - Scan configuration"]
    pub ch12_eval: CH12_EVAL,
    _reserved13: [u8; 4usize],
    #[doc = "0x390 - Scan configuration"]
    pub ch13_timing: CH13_TIMING,
    #[doc = "0x394 - Scan configuration"]
    pub ch13_interact: CH13_INTERACT,
    #[doc = "0x398 - Scan configuration"]
    pub ch13_eval: CH13_EVAL,
    _reserved14: [u8; 4usize],
    #[doc = "0x3a0 - Scan configuration"]
    pub ch14_timing: CH14_TIMING,
    #[doc = "0x3a4 - Scan configuration"]
    pub ch14_interact: CH14_INTERACT,
    #[doc = "0x3a8 - Scan configuration"]
    pub ch14_eval: CH14_EVAL,
    _reserved15: [u8; 4usize],
    #[doc = "0x3b0 - Scan configuration"]
    pub ch15_timing: CH15_TIMING,
    #[doc = "0x3b4 - Scan configuration"]
    pub ch15_interact: CH15_INTERACT,
    #[doc = "0x3b8 - Scan configuration"]
    pub ch15_eval: CH15_EVAL,
}
#[doc = "Control Register"]
pub struct CTRL {
    register: VolatileCell<u32>,
}
#[doc = "Control Register"]
pub mod ctrl;
#[doc = "Timing Control Register"]
pub struct TIMCTRL {
    register: VolatileCell<u32>,
}
#[doc = "Timing Control Register"]
pub mod timctrl;
#[doc = "Peripheral Control Register"]
pub struct PERCTRL {
    register: VolatileCell<u32>,
}
#[doc = "Peripheral Control Register"]
pub mod perctrl;
#[doc = "Decoder control Register"]
pub struct DECCTRL {
    register: VolatileCell<u32>,
}
#[doc = "Decoder control Register"]
pub mod decctrl;
#[doc = "Bias Control Register"]
pub struct BIASCTRL {
    register: VolatileCell<u32>,
}
#[doc = "Bias Control Register"]
pub mod biasctrl;
#[doc = "Command Register"]
pub struct CMD {
    register: VolatileCell<u32>,
}
#[doc = "Command Register"]
pub mod cmd;
#[doc = "Channel enable Register"]
pub struct CHEN {
    register: VolatileCell<u32>,
}
#[doc = "Channel enable Register"]
pub mod chen;
#[doc = "Scan result register"]
pub struct SCANRES {
    register: VolatileCell<u32>,
}
#[doc = "Scan result register"]
pub mod scanres;
#[doc = "Status Register"]
pub struct STATUS {
    register: VolatileCell<u32>,
}
#[doc = "Status Register"]
pub mod status;
#[doc = "Result buffer pointers"]
pub struct PTR {
    register: VolatileCell<u32>,
}
#[doc = "Result buffer pointers"]
pub mod ptr;
#[doc = "Result buffer data register"]
pub struct BUFDATA {
    register: VolatileCell<u32>,
}
#[doc = "Result buffer data register"]
pub mod bufdata;
#[doc = "Current channel index"]
pub struct CURCH {
    register: VolatileCell<u32>,
}
#[doc = "Current channel index"]
pub mod curch;
#[doc = "Current decoder state"]
pub struct DECSTATE {
    register: VolatileCell<u32>,
}
#[doc = "Current decoder state"]
pub mod decstate;
#[doc = "Decoder input register"]
pub struct SENSORSTATE {
    register: VolatileCell<u32>,
}
#[doc = "Decoder input register"]
pub mod sensorstate;
#[doc = "GPIO Idle phase configuration"]
pub struct IDLECONF {
    register: VolatileCell<u32>,
}
#[doc = "GPIO Idle phase configuration"]
pub mod idleconf;
#[doc = "Alternative excite pin configuration"]
pub struct ALTEXCONF {
    register: VolatileCell<u32>,
}
#[doc = "Alternative excite pin configuration"]
pub mod altexconf;
#[doc = "Interrupt Flag Register"]
pub struct IF {
    register: VolatileCell<u32>,
}
#[doc = "Interrupt Flag Register"]
pub mod if_;
#[doc = "Interrupt Flag Clear Register"]
pub struct IFC {
    register: VolatileCell<u32>,
}
#[doc = "Interrupt Flag Clear Register"]
pub mod ifc;
#[doc = "Interrupt Flag Set Register"]
pub struct IFS {
    register: VolatileCell<u32>,
}
#[doc = "Interrupt Flag Set Register"]
pub mod ifs;
#[doc = "Interrupt Enable Register"]
pub struct IEN {
    register: VolatileCell<u32>,
}
#[doc = "Interrupt Enable Register"]
pub mod ien;
#[doc = "Synchronization Busy Register"]
pub struct SYNCBUSY {
    register: VolatileCell<u32>,
}
#[doc = "Synchronization Busy Register"]
pub mod syncbusy;
#[doc = "I/O Routing Register"]
pub struct ROUTE {
    register: VolatileCell<u32>,
}
#[doc = "I/O Routing Register"]
pub mod route;
#[doc = "LESENSE RAM power-down register"]
pub struct POWERDOWN {
    register: VolatileCell<u32>,
}
#[doc = "LESENSE RAM power-down register"]
pub mod powerdown;
#[doc = "State transition configuration A"]
pub struct ST0_TCONFA {
    register: VolatileCell<u32>,
}
#[doc = "State transition configuration A"]
pub mod st0_tconfa;
#[doc = "State transition configuration B"]
pub struct ST0_TCONFB {
    register: VolatileCell<u32>,
}
#[doc = "State transition configuration B"]
pub mod st0_tconfb;
#[doc = "State transition configuration A"]
pub struct ST1_TCONFA {
    register: VolatileCell<u32>,
}
#[doc = "State transition configuration A"]
pub mod st1_tconfa;
#[doc = "State transition configuration B"]
pub struct ST1_TCONFB {
    register: VolatileCell<u32>,
}
#[doc = "State transition configuration B"]
pub mod st1_tconfb;
#[doc = "State transition configuration A"]
pub struct ST2_TCONFA {
    register: VolatileCell<u32>,
}
#[doc = "State transition configuration A"]
pub mod st2_tconfa;
#[doc = "State transition configuration B"]
pub struct ST2_TCONFB {
    register: VolatileCell<u32>,
}
#[doc = "State transition configuration B"]
pub mod st2_tconfb;
#[doc = "State transition configuration A"]
pub struct ST3_TCONFA {
    register: VolatileCell<u32>,
}
#[doc = "State transition configuration A"]
pub mod st3_tconfa;
#[doc = "State transition configuration B"]
pub struct ST3_TCONFB {
    register: VolatileCell<u32>,
}
#[doc = "State transition configuration B"]
pub mod st3_tconfb;
#[doc = "State transition configuration A"]
pub struct ST4_TCONFA {
    register: VolatileCell<u32>,
}
#[doc = "State transition configuration A"]
pub mod st4_tconfa;
#[doc = "State transition configuration B"]
pub struct ST4_TCONFB {
    register: VolatileCell<u32>,
}
#[doc = "State transition configuration B"]
pub mod st4_tconfb;
#[doc = "State transition configuration A"]
pub struct ST5_TCONFA {
    register: VolatileCell<u32>,
}
#[doc = "State transition configuration A"]
pub mod st5_tconfa;
#[doc = "State transition configuration B"]
pub struct ST5_TCONFB {
    register: VolatileCell<u32>,
}
#[doc = "State transition configuration B"]
pub mod st5_tconfb;
#[doc = "State transition configuration A"]
pub struct ST6_TCONFA {
    register: VolatileCell<u32>,
}
#[doc = "State transition configuration A"]
pub mod st6_tconfa;
#[doc = "State transition configuration B"]
pub struct ST6_TCONFB {
    register: VolatileCell<u32>,
}
#[doc = "State transition configuration B"]
pub mod st6_tconfb;
#[doc = "State transition configuration A"]
pub struct ST7_TCONFA {
    register: VolatileCell<u32>,
}
#[doc = "State transition configuration A"]
pub mod st7_tconfa;
#[doc = "State transition configuration B"]
pub struct ST7_TCONFB {
    register: VolatileCell<u32>,
}
#[doc = "State transition configuration B"]
pub mod st7_tconfb;
#[doc = "State transition configuration A"]
pub struct ST8_TCONFA {
    register: VolatileCell<u32>,
}
#[doc = "State transition configuration A"]
pub mod st8_tconfa;
#[doc = "State transition configuration B"]
pub struct ST8_TCONFB {
    register: VolatileCell<u32>,
}
#[doc = "State transition configuration B"]
pub mod st8_tconfb;
#[doc = "State transition configuration A"]
pub struct ST9_TCONFA {
    register: VolatileCell<u32>,
}
#[doc = "State transition configuration A"]
pub mod st9_tconfa;
#[doc = "State transition configuration B"]
pub struct ST9_TCONFB {
    register: VolatileCell<u32>,
}
#[doc = "State transition configuration B"]
pub mod st9_tconfb;
#[doc = "State transition configuration A"]
pub struct ST10_TCONFA {
    register: VolatileCell<u32>,
}
#[doc = "State transition configuration A"]
pub mod st10_tconfa;
#[doc = "State transition configuration B"]
pub struct ST10_TCONFB {
    register: VolatileCell<u32>,
}
#[doc = "State transition configuration B"]
pub mod st10_tconfb;
#[doc = "State transition configuration A"]
pub struct ST11_TCONFA {
    register: VolatileCell<u32>,
}
#[doc = "State transition configuration A"]
pub mod st11_tconfa;
#[doc = "State transition configuration B"]
pub struct ST11_TCONFB {
    register: VolatileCell<u32>,
}
#[doc = "State transition configuration B"]
pub mod st11_tconfb;
#[doc = "State transition configuration A"]
pub struct ST12_TCONFA {
    register: VolatileCell<u32>,
}
#[doc = "State transition configuration A"]
pub mod st12_tconfa;
#[doc = "State transition configuration B"]
pub struct ST12_TCONFB {
    register: VolatileCell<u32>,
}
#[doc = "State transition configuration B"]
pub mod st12_tconfb;
#[doc = "State transition configuration A"]
pub struct ST13_TCONFA {
    register: VolatileCell<u32>,
}
#[doc = "State transition configuration A"]
pub mod st13_tconfa;
#[doc = "State transition configuration B"]
pub struct ST13_TCONFB {
    register: VolatileCell<u32>,
}
#[doc = "State transition configuration B"]
pub mod st13_tconfb;
#[doc = "State transition configuration A"]
pub struct ST14_TCONFA {
    register: VolatileCell<u32>,
}
#[doc = "State transition configuration A"]
pub mod st14_tconfa;
#[doc = "State transition configuration B"]
pub struct ST14_TCONFB {
    register: VolatileCell<u32>,
}
#[doc = "State transition configuration B"]
pub mod st14_tconfb;
#[doc = "State transition configuration A"]
pub struct ST15_TCONFA {
    register: VolatileCell<u32>,
}
#[doc = "State transition configuration A"]
pub mod st15_tconfa;
#[doc = "State transition configuration B"]
pub struct ST15_TCONFB {
    register: VolatileCell<u32>,
}
#[doc = "State transition configuration B"]
pub mod st15_tconfb;
#[doc = "Scan results"]
pub struct BUF0_DATA {
    register: VolatileCell<u32>,
}
#[doc = "Scan results"]
pub mod buf0_data;
#[doc = "Scan results"]
pub struct BUF1_DATA {
    register: VolatileCell<u32>,
}
#[doc = "Scan results"]
pub mod buf1_data;
#[doc = "Scan results"]
pub struct BUF2_DATA {
    register: VolatileCell<u32>,
}
#[doc = "Scan results"]
pub mod buf2_data;
#[doc = "Scan results"]
pub struct BUF3_DATA {
    register: VolatileCell<u32>,
}
#[doc = "Scan results"]
pub mod buf3_data;
#[doc = "Scan results"]
pub struct BUF4_DATA {
    register: VolatileCell<u32>,
}
#[doc = "Scan results"]
pub mod buf4_data;
#[doc = "Scan results"]
pub struct BUF5_DATA {
    register: VolatileCell<u32>,
}
#[doc = "Scan results"]
pub mod buf5_data;
#[doc = "Scan results"]
pub struct BUF6_DATA {
    register: VolatileCell<u32>,
}
#[doc = "Scan results"]
pub mod buf6_data;
#[doc = "Scan results"]
pub struct BUF7_DATA {
    register: VolatileCell<u32>,
}
#[doc = "Scan results"]
pub mod buf7_data;
#[doc = "Scan results"]
pub struct BUF8_DATA {
    register: VolatileCell<u32>,
}
#[doc = "Scan results"]
pub mod buf8_data;
#[doc = "Scan results"]
pub struct BUF9_DATA {
    register: VolatileCell<u32>,
}
#[doc = "Scan results"]
pub mod buf9_data;
#[doc = "Scan results"]
pub struct BUF10_DATA {
    register: VolatileCell<u32>,
}
#[doc = "Scan results"]
pub mod buf10_data;
#[doc = "Scan results"]
pub struct BUF11_DATA {
    register: VolatileCell<u32>,
}
#[doc = "Scan results"]
pub mod buf11_data;
#[doc = "Scan results"]
pub struct BUF12_DATA {
    register: VolatileCell<u32>,
}
#[doc = "Scan results"]
pub mod buf12_data;
#[doc = "Scan results"]
pub struct BUF13_DATA {
    register: VolatileCell<u32>,
}
#[doc = "Scan results"]
pub mod buf13_data;
#[doc = "Scan results"]
pub struct BUF14_DATA {
    register: VolatileCell<u32>,
}
#[doc = "Scan results"]
pub mod buf14_data;
#[doc = "Scan results"]
pub struct BUF15_DATA {
    register: VolatileCell<u32>,
}
#[doc = "Scan results"]
pub mod buf15_data;
#[doc = "Scan configuration"]
pub struct CH0_TIMING {
    register: VolatileCell<u32>,
}
#[doc = "Scan configuration"]
pub mod ch0_timing;
#[doc = "Scan configuration"]
pub struct CH0_INTERACT {
    register: VolatileCell<u32>,
}
#[doc = "Scan configuration"]
pub mod ch0_interact;
#[doc = "Scan configuration"]
pub struct CH0_EVAL {
    register: VolatileCell<u32>,
}
#[doc = "Scan configuration"]
pub mod ch0_eval;
#[doc = "Scan configuration"]
pub struct CH1_TIMING {
    register: VolatileCell<u32>,
}
#[doc = "Scan configuration"]
pub mod ch1_timing;
#[doc = "Scan configuration"]
pub struct CH1_INTERACT {
    register: VolatileCell<u32>,
}
#[doc = "Scan configuration"]
pub mod ch1_interact;
#[doc = "Scan configuration"]
pub struct CH1_EVAL {
    register: VolatileCell<u32>,
}
#[doc = "Scan configuration"]
pub mod ch1_eval;
#[doc = "Scan configuration"]
pub struct CH2_TIMING {
    register: VolatileCell<u32>,
}
#[doc = "Scan configuration"]
pub mod ch2_timing;
#[doc = "Scan configuration"]
pub struct CH2_INTERACT {
    register: VolatileCell<u32>,
}
#[doc = "Scan configuration"]
pub mod ch2_interact;
#[doc = "Scan configuration"]
pub struct CH2_EVAL {
    register: VolatileCell<u32>,
}
#[doc = "Scan configuration"]
pub mod ch2_eval;
#[doc = "Scan configuration"]
pub struct CH3_TIMING {
    register: VolatileCell<u32>,
}
#[doc = "Scan configuration"]
pub mod ch3_timing;
#[doc = "Scan configuration"]
pub struct CH3_INTERACT {
    register: VolatileCell<u32>,
}
#[doc = "Scan configuration"]
pub mod ch3_interact;
#[doc = "Scan configuration"]
pub struct CH3_EVAL {
    register: VolatileCell<u32>,
}
#[doc = "Scan configuration"]
pub mod ch3_eval;
#[doc = "Scan configuration"]
pub struct CH4_TIMING {
    register: VolatileCell<u32>,
}
#[doc = "Scan configuration"]
pub mod ch4_timing;
#[doc = "Scan configuration"]
pub struct CH4_INTERACT {
    register: VolatileCell<u32>,
}
#[doc = "Scan configuration"]
pub mod ch4_interact;
#[doc = "Scan configuration"]
pub struct CH4_EVAL {
    register: VolatileCell<u32>,
}
#[doc = "Scan configuration"]
pub mod ch4_eval;
#[doc = "Scan configuration"]
pub struct CH5_TIMING {
    register: VolatileCell<u32>,
}
#[doc = "Scan configuration"]
pub mod ch5_timing;
#[doc = "Scan configuration"]
pub struct CH5_INTERACT {
    register: VolatileCell<u32>,
}
#[doc = "Scan configuration"]
pub mod ch5_interact;
#[doc = "Scan configuration"]
pub struct CH5_EVAL {
    register: VolatileCell<u32>,
}
#[doc = "Scan configuration"]
pub mod ch5_eval;
#[doc = "Scan configuration"]
pub struct CH6_TIMING {
    register: VolatileCell<u32>,
}
#[doc = "Scan configuration"]
pub mod ch6_timing;
#[doc = "Scan configuration"]
pub struct CH6_INTERACT {
    register: VolatileCell<u32>,
}
#[doc = "Scan configuration"]
pub mod ch6_interact;
#[doc = "Scan configuration"]
pub struct CH6_EVAL {
    register: VolatileCell<u32>,
}
#[doc = "Scan configuration"]
pub mod ch6_eval;
#[doc = "Scan configuration"]
pub struct CH7_TIMING {
    register: VolatileCell<u32>,
}
#[doc = "Scan configuration"]
pub mod ch7_timing;
#[doc = "Scan configuration"]
pub struct CH7_INTERACT {
    register: VolatileCell<u32>,
}
#[doc = "Scan configuration"]
pub mod ch7_interact;
#[doc = "Scan configuration"]
pub struct CH7_EVAL {
    register: VolatileCell<u32>,
}
#[doc = "Scan configuration"]
pub mod ch7_eval;
#[doc = "Scan configuration"]
pub struct CH8_TIMING {
    register: VolatileCell<u32>,
}
#[doc = "Scan configuration"]
pub mod ch8_timing;
#[doc = "Scan configuration"]
pub struct CH8_INTERACT {
    register: VolatileCell<u32>,
}
#[doc = "Scan configuration"]
pub mod ch8_interact;
#[doc = "Scan configuration"]
pub struct CH8_EVAL {
    register: VolatileCell<u32>,
}
#[doc = "Scan configuration"]
pub mod ch8_eval;
#[doc = "Scan configuration"]
pub struct CH9_TIMING {
    register: VolatileCell<u32>,
}
#[doc = "Scan configuration"]
pub mod ch9_timing;
#[doc = "Scan configuration"]
pub struct CH9_INTERACT {
    register: VolatileCell<u32>,
}
#[doc = "Scan configuration"]
pub mod ch9_interact;
#[doc = "Scan configuration"]
pub struct CH9_EVAL {
    register: VolatileCell<u32>,
}
#[doc = "Scan configuration"]
pub mod ch9_eval;
#[doc = "Scan configuration"]
pub struct CH10_TIMING {
    register: VolatileCell<u32>,
}
#[doc = "Scan configuration"]
pub mod ch10_timing;
#[doc = "Scan configuration"]
pub struct CH10_INTERACT {
    register: VolatileCell<u32>,
}
#[doc = "Scan configuration"]
pub mod ch10_interact;
#[doc = "Scan configuration"]
pub struct CH10_EVAL {
    register: VolatileCell<u32>,
}
#[doc = "Scan configuration"]
pub mod ch10_eval;
#[doc = "Scan configuration"]
pub struct CH11_TIMING {
    register: VolatileCell<u32>,
}
#[doc = "Scan configuration"]
pub mod ch11_timing;
#[doc = "Scan configuration"]
pub struct CH11_INTERACT {
    register: VolatileCell<u32>,
}
#[doc = "Scan configuration"]
pub mod ch11_interact;
#[doc = "Scan configuration"]
pub struct CH11_EVAL {
    register: VolatileCell<u32>,
}
#[doc = "Scan configuration"]
pub mod ch11_eval;
#[doc = "Scan configuration"]
pub struct CH12_TIMING {
    register: VolatileCell<u32>,
}
#[doc = "Scan configuration"]
pub mod ch12_timing;
#[doc = "Scan configuration"]
pub struct CH12_INTERACT {
    register: VolatileCell<u32>,
}
#[doc = "Scan configuration"]
pub mod ch12_interact;
#[doc = "Scan configuration"]
pub struct CH12_EVAL {
    register: VolatileCell<u32>,
}
#[doc = "Scan configuration"]
pub mod ch12_eval;
#[doc = "Scan configuration"]
pub struct CH13_TIMING {
    register: VolatileCell<u32>,
}
#[doc = "Scan configuration"]
pub mod ch13_timing;
#[doc = "Scan configuration"]
pub struct CH13_INTERACT {
    register: VolatileCell<u32>,
}
#[doc = "Scan configuration"]
pub mod ch13_interact;
#[doc = "Scan configuration"]
pub struct CH13_EVAL {
    register: VolatileCell<u32>,
}
#[doc = "Scan configuration"]
pub mod ch13_eval;
#[doc = "Scan configuration"]
pub struct CH14_TIMING {
    register: VolatileCell<u32>,
}
#[doc = "Scan configuration"]
pub mod ch14_timing;
#[doc = "Scan configuration"]
pub struct CH14_INTERACT {
    register: VolatileCell<u32>,
}
#[doc = "Scan configuration"]
pub mod ch14_interact;
#[doc = "Scan configuration"]
pub struct CH14_EVAL {
    register: VolatileCell<u32>,
}
#[doc = "Scan configuration"]
pub mod ch14_eval;
#[doc = "Scan configuration"]
pub struct CH15_TIMING {
    register: VolatileCell<u32>,
}
#[doc = "Scan configuration"]
pub mod ch15_timing;
#[doc = "Scan configuration"]
pub struct CH15_INTERACT {
    register: VolatileCell<u32>,
}
#[doc = "Scan configuration"]
pub mod ch15_interact;
#[doc = "Scan configuration"]
pub struct CH15_EVAL {
    register: VolatileCell<u32>,
}
#[doc = "Scan configuration"]
pub mod ch15_eval;

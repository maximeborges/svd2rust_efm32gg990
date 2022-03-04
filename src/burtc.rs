use vcell::VolatileCell;
#[doc = r" Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Control Register"]
    pub ctrl: CTRL,
    #[doc = "0x04 - Low power mode configuration"]
    pub lpmode: LPMODE,
    #[doc = "0x08 - Counter Value Register"]
    pub cnt: CNT,
    #[doc = "0x0c - Counter Compare Value"]
    pub comp0: COMP0,
    #[doc = "0x10 - Backup mode timestamp"]
    pub timestamp: TIMESTAMP,
    #[doc = "0x14 - LFXO"]
    pub lfxofdet: LFXOFDET,
    #[doc = "0x18 - Status Register"]
    pub status: STATUS,
    #[doc = "0x1c - Command Register"]
    pub cmd: CMD,
    #[doc = "0x20 - Retention RAM power-down Register"]
    pub powerdown: POWERDOWN,
    #[doc = "0x24 - Configuration Lock Register"]
    pub lock: LOCK,
    #[doc = "0x28 - Interrupt Flag Register"]
    pub if_: IF,
    #[doc = "0x2c - Interrupt Flag Set Register"]
    pub ifs: IFS,
    #[doc = "0x30 - Interrupt Flag Clear Register"]
    pub ifc: IFC,
    #[doc = "0x34 - Interrupt Enable Register"]
    pub ien: IEN,
    #[doc = "0x38 - Freeze Register"]
    pub freeze: FREEZE,
    #[doc = "0x3c - Synchronization Busy Register"]
    pub syncbusy: SYNCBUSY,
    _reserved0: [u8; 192usize],
    #[doc = "0x100 - Retention Register"]
    pub ret0_reg: RET0_REG,
    #[doc = "0x104 - Retention Register"]
    pub ret1_reg: RET1_REG,
    #[doc = "0x108 - Retention Register"]
    pub ret2_reg: RET2_REG,
    #[doc = "0x10c - Retention Register"]
    pub ret3_reg: RET3_REG,
    #[doc = "0x110 - Retention Register"]
    pub ret4_reg: RET4_REG,
    #[doc = "0x114 - Retention Register"]
    pub ret5_reg: RET5_REG,
    #[doc = "0x118 - Retention Register"]
    pub ret6_reg: RET6_REG,
    #[doc = "0x11c - Retention Register"]
    pub ret7_reg: RET7_REG,
    #[doc = "0x120 - Retention Register"]
    pub ret8_reg: RET8_REG,
    #[doc = "0x124 - Retention Register"]
    pub ret9_reg: RET9_REG,
    #[doc = "0x128 - Retention Register"]
    pub ret10_reg: RET10_REG,
    #[doc = "0x12c - Retention Register"]
    pub ret11_reg: RET11_REG,
    #[doc = "0x130 - Retention Register"]
    pub ret12_reg: RET12_REG,
    #[doc = "0x134 - Retention Register"]
    pub ret13_reg: RET13_REG,
    #[doc = "0x138 - Retention Register"]
    pub ret14_reg: RET14_REG,
    #[doc = "0x13c - Retention Register"]
    pub ret15_reg: RET15_REG,
    #[doc = "0x140 - Retention Register"]
    pub ret16_reg: RET16_REG,
    #[doc = "0x144 - Retention Register"]
    pub ret17_reg: RET17_REG,
    #[doc = "0x148 - Retention Register"]
    pub ret18_reg: RET18_REG,
    #[doc = "0x14c - Retention Register"]
    pub ret19_reg: RET19_REG,
    #[doc = "0x150 - Retention Register"]
    pub ret20_reg: RET20_REG,
    #[doc = "0x154 - Retention Register"]
    pub ret21_reg: RET21_REG,
    #[doc = "0x158 - Retention Register"]
    pub ret22_reg: RET22_REG,
    #[doc = "0x15c - Retention Register"]
    pub ret23_reg: RET23_REG,
    #[doc = "0x160 - Retention Register"]
    pub ret24_reg: RET24_REG,
    #[doc = "0x164 - Retention Register"]
    pub ret25_reg: RET25_REG,
    #[doc = "0x168 - Retention Register"]
    pub ret26_reg: RET26_REG,
    #[doc = "0x16c - Retention Register"]
    pub ret27_reg: RET27_REG,
    #[doc = "0x170 - Retention Register"]
    pub ret28_reg: RET28_REG,
    #[doc = "0x174 - Retention Register"]
    pub ret29_reg: RET29_REG,
    #[doc = "0x178 - Retention Register"]
    pub ret30_reg: RET30_REG,
    #[doc = "0x17c - Retention Register"]
    pub ret31_reg: RET31_REG,
    #[doc = "0x180 - Retention Register"]
    pub ret32_reg: RET32_REG,
    #[doc = "0x184 - Retention Register"]
    pub ret33_reg: RET33_REG,
    #[doc = "0x188 - Retention Register"]
    pub ret34_reg: RET34_REG,
    #[doc = "0x18c - Retention Register"]
    pub ret35_reg: RET35_REG,
    #[doc = "0x190 - Retention Register"]
    pub ret36_reg: RET36_REG,
    #[doc = "0x194 - Retention Register"]
    pub ret37_reg: RET37_REG,
    #[doc = "0x198 - Retention Register"]
    pub ret38_reg: RET38_REG,
    #[doc = "0x19c - Retention Register"]
    pub ret39_reg: RET39_REG,
    #[doc = "0x1a0 - Retention Register"]
    pub ret40_reg: RET40_REG,
    #[doc = "0x1a4 - Retention Register"]
    pub ret41_reg: RET41_REG,
    #[doc = "0x1a8 - Retention Register"]
    pub ret42_reg: RET42_REG,
    #[doc = "0x1ac - Retention Register"]
    pub ret43_reg: RET43_REG,
    #[doc = "0x1b0 - Retention Register"]
    pub ret44_reg: RET44_REG,
    #[doc = "0x1b4 - Retention Register"]
    pub ret45_reg: RET45_REG,
    #[doc = "0x1b8 - Retention Register"]
    pub ret46_reg: RET46_REG,
    #[doc = "0x1bc - Retention Register"]
    pub ret47_reg: RET47_REG,
    #[doc = "0x1c0 - Retention Register"]
    pub ret48_reg: RET48_REG,
    #[doc = "0x1c4 - Retention Register"]
    pub ret49_reg: RET49_REG,
    #[doc = "0x1c8 - Retention Register"]
    pub ret50_reg: RET50_REG,
    #[doc = "0x1cc - Retention Register"]
    pub ret51_reg: RET51_REG,
    #[doc = "0x1d0 - Retention Register"]
    pub ret52_reg: RET52_REG,
    #[doc = "0x1d4 - Retention Register"]
    pub ret53_reg: RET53_REG,
    #[doc = "0x1d8 - Retention Register"]
    pub ret54_reg: RET54_REG,
    #[doc = "0x1dc - Retention Register"]
    pub ret55_reg: RET55_REG,
    #[doc = "0x1e0 - Retention Register"]
    pub ret56_reg: RET56_REG,
    #[doc = "0x1e4 - Retention Register"]
    pub ret57_reg: RET57_REG,
    #[doc = "0x1e8 - Retention Register"]
    pub ret58_reg: RET58_REG,
    #[doc = "0x1ec - Retention Register"]
    pub ret59_reg: RET59_REG,
    #[doc = "0x1f0 - Retention Register"]
    pub ret60_reg: RET60_REG,
    #[doc = "0x1f4 - Retention Register"]
    pub ret61_reg: RET61_REG,
    #[doc = "0x1f8 - Retention Register"]
    pub ret62_reg: RET62_REG,
    #[doc = "0x1fc - Retention Register"]
    pub ret63_reg: RET63_REG,
    #[doc = "0x200 - Retention Register"]
    pub ret64_reg: RET64_REG,
    #[doc = "0x204 - Retention Register"]
    pub ret65_reg: RET65_REG,
    #[doc = "0x208 - Retention Register"]
    pub ret66_reg: RET66_REG,
    #[doc = "0x20c - Retention Register"]
    pub ret67_reg: RET67_REG,
    #[doc = "0x210 - Retention Register"]
    pub ret68_reg: RET68_REG,
    #[doc = "0x214 - Retention Register"]
    pub ret69_reg: RET69_REG,
    #[doc = "0x218 - Retention Register"]
    pub ret70_reg: RET70_REG,
    #[doc = "0x21c - Retention Register"]
    pub ret71_reg: RET71_REG,
    #[doc = "0x220 - Retention Register"]
    pub ret72_reg: RET72_REG,
    #[doc = "0x224 - Retention Register"]
    pub ret73_reg: RET73_REG,
    #[doc = "0x228 - Retention Register"]
    pub ret74_reg: RET74_REG,
    #[doc = "0x22c - Retention Register"]
    pub ret75_reg: RET75_REG,
    #[doc = "0x230 - Retention Register"]
    pub ret76_reg: RET76_REG,
    #[doc = "0x234 - Retention Register"]
    pub ret77_reg: RET77_REG,
    #[doc = "0x238 - Retention Register"]
    pub ret78_reg: RET78_REG,
    #[doc = "0x23c - Retention Register"]
    pub ret79_reg: RET79_REG,
    #[doc = "0x240 - Retention Register"]
    pub ret80_reg: RET80_REG,
    #[doc = "0x244 - Retention Register"]
    pub ret81_reg: RET81_REG,
    #[doc = "0x248 - Retention Register"]
    pub ret82_reg: RET82_REG,
    #[doc = "0x24c - Retention Register"]
    pub ret83_reg: RET83_REG,
    #[doc = "0x250 - Retention Register"]
    pub ret84_reg: RET84_REG,
    #[doc = "0x254 - Retention Register"]
    pub ret85_reg: RET85_REG,
    #[doc = "0x258 - Retention Register"]
    pub ret86_reg: RET86_REG,
    #[doc = "0x25c - Retention Register"]
    pub ret87_reg: RET87_REG,
    #[doc = "0x260 - Retention Register"]
    pub ret88_reg: RET88_REG,
    #[doc = "0x264 - Retention Register"]
    pub ret89_reg: RET89_REG,
    #[doc = "0x268 - Retention Register"]
    pub ret90_reg: RET90_REG,
    #[doc = "0x26c - Retention Register"]
    pub ret91_reg: RET91_REG,
    #[doc = "0x270 - Retention Register"]
    pub ret92_reg: RET92_REG,
    #[doc = "0x274 - Retention Register"]
    pub ret93_reg: RET93_REG,
    #[doc = "0x278 - Retention Register"]
    pub ret94_reg: RET94_REG,
    #[doc = "0x27c - Retention Register"]
    pub ret95_reg: RET95_REG,
    #[doc = "0x280 - Retention Register"]
    pub ret96_reg: RET96_REG,
    #[doc = "0x284 - Retention Register"]
    pub ret97_reg: RET97_REG,
    #[doc = "0x288 - Retention Register"]
    pub ret98_reg: RET98_REG,
    #[doc = "0x28c - Retention Register"]
    pub ret99_reg: RET99_REG,
    #[doc = "0x290 - Retention Register"]
    pub ret100_reg: RET100_REG,
    #[doc = "0x294 - Retention Register"]
    pub ret101_reg: RET101_REG,
    #[doc = "0x298 - Retention Register"]
    pub ret102_reg: RET102_REG,
    #[doc = "0x29c - Retention Register"]
    pub ret103_reg: RET103_REG,
    #[doc = "0x2a0 - Retention Register"]
    pub ret104_reg: RET104_REG,
    #[doc = "0x2a4 - Retention Register"]
    pub ret105_reg: RET105_REG,
    #[doc = "0x2a8 - Retention Register"]
    pub ret106_reg: RET106_REG,
    #[doc = "0x2ac - Retention Register"]
    pub ret107_reg: RET107_REG,
    #[doc = "0x2b0 - Retention Register"]
    pub ret108_reg: RET108_REG,
    #[doc = "0x2b4 - Retention Register"]
    pub ret109_reg: RET109_REG,
    #[doc = "0x2b8 - Retention Register"]
    pub ret110_reg: RET110_REG,
    #[doc = "0x2bc - Retention Register"]
    pub ret111_reg: RET111_REG,
    #[doc = "0x2c0 - Retention Register"]
    pub ret112_reg: RET112_REG,
    #[doc = "0x2c4 - Retention Register"]
    pub ret113_reg: RET113_REG,
    #[doc = "0x2c8 - Retention Register"]
    pub ret114_reg: RET114_REG,
    #[doc = "0x2cc - Retention Register"]
    pub ret115_reg: RET115_REG,
    #[doc = "0x2d0 - Retention Register"]
    pub ret116_reg: RET116_REG,
    #[doc = "0x2d4 - Retention Register"]
    pub ret117_reg: RET117_REG,
    #[doc = "0x2d8 - Retention Register"]
    pub ret118_reg: RET118_REG,
    #[doc = "0x2dc - Retention Register"]
    pub ret119_reg: RET119_REG,
    #[doc = "0x2e0 - Retention Register"]
    pub ret120_reg: RET120_REG,
    #[doc = "0x2e4 - Retention Register"]
    pub ret121_reg: RET121_REG,
    #[doc = "0x2e8 - Retention Register"]
    pub ret122_reg: RET122_REG,
    #[doc = "0x2ec - Retention Register"]
    pub ret123_reg: RET123_REG,
    #[doc = "0x2f0 - Retention Register"]
    pub ret124_reg: RET124_REG,
    #[doc = "0x2f4 - Retention Register"]
    pub ret125_reg: RET125_REG,
    #[doc = "0x2f8 - Retention Register"]
    pub ret126_reg: RET126_REG,
    #[doc = "0x2fc - Retention Register"]
    pub ret127_reg: RET127_REG,
}
#[doc = "Control Register"]
pub struct CTRL {
    register: VolatileCell<u32>,
}
#[doc = "Control Register"]
pub mod ctrl;
#[doc = "Low power mode configuration"]
pub struct LPMODE {
    register: VolatileCell<u32>,
}
#[doc = "Low power mode configuration"]
pub mod lpmode;
#[doc = "Counter Value Register"]
pub struct CNT {
    register: VolatileCell<u32>,
}
#[doc = "Counter Value Register"]
pub mod cnt;
#[doc = "Counter Compare Value"]
pub struct COMP0 {
    register: VolatileCell<u32>,
}
#[doc = "Counter Compare Value"]
pub mod comp0;
#[doc = "Backup mode timestamp"]
pub struct TIMESTAMP {
    register: VolatileCell<u32>,
}
#[doc = "Backup mode timestamp"]
pub mod timestamp;
#[doc = "LFXO"]
pub struct LFXOFDET {
    register: VolatileCell<u32>,
}
#[doc = "LFXO"]
pub mod lfxofdet;
#[doc = "Status Register"]
pub struct STATUS {
    register: VolatileCell<u32>,
}
#[doc = "Status Register"]
pub mod status;
#[doc = "Command Register"]
pub struct CMD {
    register: VolatileCell<u32>,
}
#[doc = "Command Register"]
pub mod cmd;
#[doc = "Retention RAM power-down Register"]
pub struct POWERDOWN {
    register: VolatileCell<u32>,
}
#[doc = "Retention RAM power-down Register"]
pub mod powerdown;
#[doc = "Configuration Lock Register"]
pub struct LOCK {
    register: VolatileCell<u32>,
}
#[doc = "Configuration Lock Register"]
pub mod lock;
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
#[doc = "Retention Register"]
pub struct RET0_REG {
    register: VolatileCell<u32>,
}
#[doc = "Retention Register"]
pub mod ret0_reg;
#[doc = "Retention Register"]
pub struct RET1_REG {
    register: VolatileCell<u32>,
}
#[doc = "Retention Register"]
pub mod ret1_reg;
#[doc = "Retention Register"]
pub struct RET2_REG {
    register: VolatileCell<u32>,
}
#[doc = "Retention Register"]
pub mod ret2_reg;
#[doc = "Retention Register"]
pub struct RET3_REG {
    register: VolatileCell<u32>,
}
#[doc = "Retention Register"]
pub mod ret3_reg;
#[doc = "Retention Register"]
pub struct RET4_REG {
    register: VolatileCell<u32>,
}
#[doc = "Retention Register"]
pub mod ret4_reg;
#[doc = "Retention Register"]
pub struct RET5_REG {
    register: VolatileCell<u32>,
}
#[doc = "Retention Register"]
pub mod ret5_reg;
#[doc = "Retention Register"]
pub struct RET6_REG {
    register: VolatileCell<u32>,
}
#[doc = "Retention Register"]
pub mod ret6_reg;
#[doc = "Retention Register"]
pub struct RET7_REG {
    register: VolatileCell<u32>,
}
#[doc = "Retention Register"]
pub mod ret7_reg;
#[doc = "Retention Register"]
pub struct RET8_REG {
    register: VolatileCell<u32>,
}
#[doc = "Retention Register"]
pub mod ret8_reg;
#[doc = "Retention Register"]
pub struct RET9_REG {
    register: VolatileCell<u32>,
}
#[doc = "Retention Register"]
pub mod ret9_reg;
#[doc = "Retention Register"]
pub struct RET10_REG {
    register: VolatileCell<u32>,
}
#[doc = "Retention Register"]
pub mod ret10_reg;
#[doc = "Retention Register"]
pub struct RET11_REG {
    register: VolatileCell<u32>,
}
#[doc = "Retention Register"]
pub mod ret11_reg;
#[doc = "Retention Register"]
pub struct RET12_REG {
    register: VolatileCell<u32>,
}
#[doc = "Retention Register"]
pub mod ret12_reg;
#[doc = "Retention Register"]
pub struct RET13_REG {
    register: VolatileCell<u32>,
}
#[doc = "Retention Register"]
pub mod ret13_reg;
#[doc = "Retention Register"]
pub struct RET14_REG {
    register: VolatileCell<u32>,
}
#[doc = "Retention Register"]
pub mod ret14_reg;
#[doc = "Retention Register"]
pub struct RET15_REG {
    register: VolatileCell<u32>,
}
#[doc = "Retention Register"]
pub mod ret15_reg;
#[doc = "Retention Register"]
pub struct RET16_REG {
    register: VolatileCell<u32>,
}
#[doc = "Retention Register"]
pub mod ret16_reg;
#[doc = "Retention Register"]
pub struct RET17_REG {
    register: VolatileCell<u32>,
}
#[doc = "Retention Register"]
pub mod ret17_reg;
#[doc = "Retention Register"]
pub struct RET18_REG {
    register: VolatileCell<u32>,
}
#[doc = "Retention Register"]
pub mod ret18_reg;
#[doc = "Retention Register"]
pub struct RET19_REG {
    register: VolatileCell<u32>,
}
#[doc = "Retention Register"]
pub mod ret19_reg;
#[doc = "Retention Register"]
pub struct RET20_REG {
    register: VolatileCell<u32>,
}
#[doc = "Retention Register"]
pub mod ret20_reg;
#[doc = "Retention Register"]
pub struct RET21_REG {
    register: VolatileCell<u32>,
}
#[doc = "Retention Register"]
pub mod ret21_reg;
#[doc = "Retention Register"]
pub struct RET22_REG {
    register: VolatileCell<u32>,
}
#[doc = "Retention Register"]
pub mod ret22_reg;
#[doc = "Retention Register"]
pub struct RET23_REG {
    register: VolatileCell<u32>,
}
#[doc = "Retention Register"]
pub mod ret23_reg;
#[doc = "Retention Register"]
pub struct RET24_REG {
    register: VolatileCell<u32>,
}
#[doc = "Retention Register"]
pub mod ret24_reg;
#[doc = "Retention Register"]
pub struct RET25_REG {
    register: VolatileCell<u32>,
}
#[doc = "Retention Register"]
pub mod ret25_reg;
#[doc = "Retention Register"]
pub struct RET26_REG {
    register: VolatileCell<u32>,
}
#[doc = "Retention Register"]
pub mod ret26_reg;
#[doc = "Retention Register"]
pub struct RET27_REG {
    register: VolatileCell<u32>,
}
#[doc = "Retention Register"]
pub mod ret27_reg;
#[doc = "Retention Register"]
pub struct RET28_REG {
    register: VolatileCell<u32>,
}
#[doc = "Retention Register"]
pub mod ret28_reg;
#[doc = "Retention Register"]
pub struct RET29_REG {
    register: VolatileCell<u32>,
}
#[doc = "Retention Register"]
pub mod ret29_reg;
#[doc = "Retention Register"]
pub struct RET30_REG {
    register: VolatileCell<u32>,
}
#[doc = "Retention Register"]
pub mod ret30_reg;
#[doc = "Retention Register"]
pub struct RET31_REG {
    register: VolatileCell<u32>,
}
#[doc = "Retention Register"]
pub mod ret31_reg;
#[doc = "Retention Register"]
pub struct RET32_REG {
    register: VolatileCell<u32>,
}
#[doc = "Retention Register"]
pub mod ret32_reg;
#[doc = "Retention Register"]
pub struct RET33_REG {
    register: VolatileCell<u32>,
}
#[doc = "Retention Register"]
pub mod ret33_reg;
#[doc = "Retention Register"]
pub struct RET34_REG {
    register: VolatileCell<u32>,
}
#[doc = "Retention Register"]
pub mod ret34_reg;
#[doc = "Retention Register"]
pub struct RET35_REG {
    register: VolatileCell<u32>,
}
#[doc = "Retention Register"]
pub mod ret35_reg;
#[doc = "Retention Register"]
pub struct RET36_REG {
    register: VolatileCell<u32>,
}
#[doc = "Retention Register"]
pub mod ret36_reg;
#[doc = "Retention Register"]
pub struct RET37_REG {
    register: VolatileCell<u32>,
}
#[doc = "Retention Register"]
pub mod ret37_reg;
#[doc = "Retention Register"]
pub struct RET38_REG {
    register: VolatileCell<u32>,
}
#[doc = "Retention Register"]
pub mod ret38_reg;
#[doc = "Retention Register"]
pub struct RET39_REG {
    register: VolatileCell<u32>,
}
#[doc = "Retention Register"]
pub mod ret39_reg;
#[doc = "Retention Register"]
pub struct RET40_REG {
    register: VolatileCell<u32>,
}
#[doc = "Retention Register"]
pub mod ret40_reg;
#[doc = "Retention Register"]
pub struct RET41_REG {
    register: VolatileCell<u32>,
}
#[doc = "Retention Register"]
pub mod ret41_reg;
#[doc = "Retention Register"]
pub struct RET42_REG {
    register: VolatileCell<u32>,
}
#[doc = "Retention Register"]
pub mod ret42_reg;
#[doc = "Retention Register"]
pub struct RET43_REG {
    register: VolatileCell<u32>,
}
#[doc = "Retention Register"]
pub mod ret43_reg;
#[doc = "Retention Register"]
pub struct RET44_REG {
    register: VolatileCell<u32>,
}
#[doc = "Retention Register"]
pub mod ret44_reg;
#[doc = "Retention Register"]
pub struct RET45_REG {
    register: VolatileCell<u32>,
}
#[doc = "Retention Register"]
pub mod ret45_reg;
#[doc = "Retention Register"]
pub struct RET46_REG {
    register: VolatileCell<u32>,
}
#[doc = "Retention Register"]
pub mod ret46_reg;
#[doc = "Retention Register"]
pub struct RET47_REG {
    register: VolatileCell<u32>,
}
#[doc = "Retention Register"]
pub mod ret47_reg;
#[doc = "Retention Register"]
pub struct RET48_REG {
    register: VolatileCell<u32>,
}
#[doc = "Retention Register"]
pub mod ret48_reg;
#[doc = "Retention Register"]
pub struct RET49_REG {
    register: VolatileCell<u32>,
}
#[doc = "Retention Register"]
pub mod ret49_reg;
#[doc = "Retention Register"]
pub struct RET50_REG {
    register: VolatileCell<u32>,
}
#[doc = "Retention Register"]
pub mod ret50_reg;
#[doc = "Retention Register"]
pub struct RET51_REG {
    register: VolatileCell<u32>,
}
#[doc = "Retention Register"]
pub mod ret51_reg;
#[doc = "Retention Register"]
pub struct RET52_REG {
    register: VolatileCell<u32>,
}
#[doc = "Retention Register"]
pub mod ret52_reg;
#[doc = "Retention Register"]
pub struct RET53_REG {
    register: VolatileCell<u32>,
}
#[doc = "Retention Register"]
pub mod ret53_reg;
#[doc = "Retention Register"]
pub struct RET54_REG {
    register: VolatileCell<u32>,
}
#[doc = "Retention Register"]
pub mod ret54_reg;
#[doc = "Retention Register"]
pub struct RET55_REG {
    register: VolatileCell<u32>,
}
#[doc = "Retention Register"]
pub mod ret55_reg;
#[doc = "Retention Register"]
pub struct RET56_REG {
    register: VolatileCell<u32>,
}
#[doc = "Retention Register"]
pub mod ret56_reg;
#[doc = "Retention Register"]
pub struct RET57_REG {
    register: VolatileCell<u32>,
}
#[doc = "Retention Register"]
pub mod ret57_reg;
#[doc = "Retention Register"]
pub struct RET58_REG {
    register: VolatileCell<u32>,
}
#[doc = "Retention Register"]
pub mod ret58_reg;
#[doc = "Retention Register"]
pub struct RET59_REG {
    register: VolatileCell<u32>,
}
#[doc = "Retention Register"]
pub mod ret59_reg;
#[doc = "Retention Register"]
pub struct RET60_REG {
    register: VolatileCell<u32>,
}
#[doc = "Retention Register"]
pub mod ret60_reg;
#[doc = "Retention Register"]
pub struct RET61_REG {
    register: VolatileCell<u32>,
}
#[doc = "Retention Register"]
pub mod ret61_reg;
#[doc = "Retention Register"]
pub struct RET62_REG {
    register: VolatileCell<u32>,
}
#[doc = "Retention Register"]
pub mod ret62_reg;
#[doc = "Retention Register"]
pub struct RET63_REG {
    register: VolatileCell<u32>,
}
#[doc = "Retention Register"]
pub mod ret63_reg;
#[doc = "Retention Register"]
pub struct RET64_REG {
    register: VolatileCell<u32>,
}
#[doc = "Retention Register"]
pub mod ret64_reg;
#[doc = "Retention Register"]
pub struct RET65_REG {
    register: VolatileCell<u32>,
}
#[doc = "Retention Register"]
pub mod ret65_reg;
#[doc = "Retention Register"]
pub struct RET66_REG {
    register: VolatileCell<u32>,
}
#[doc = "Retention Register"]
pub mod ret66_reg;
#[doc = "Retention Register"]
pub struct RET67_REG {
    register: VolatileCell<u32>,
}
#[doc = "Retention Register"]
pub mod ret67_reg;
#[doc = "Retention Register"]
pub struct RET68_REG {
    register: VolatileCell<u32>,
}
#[doc = "Retention Register"]
pub mod ret68_reg;
#[doc = "Retention Register"]
pub struct RET69_REG {
    register: VolatileCell<u32>,
}
#[doc = "Retention Register"]
pub mod ret69_reg;
#[doc = "Retention Register"]
pub struct RET70_REG {
    register: VolatileCell<u32>,
}
#[doc = "Retention Register"]
pub mod ret70_reg;
#[doc = "Retention Register"]
pub struct RET71_REG {
    register: VolatileCell<u32>,
}
#[doc = "Retention Register"]
pub mod ret71_reg;
#[doc = "Retention Register"]
pub struct RET72_REG {
    register: VolatileCell<u32>,
}
#[doc = "Retention Register"]
pub mod ret72_reg;
#[doc = "Retention Register"]
pub struct RET73_REG {
    register: VolatileCell<u32>,
}
#[doc = "Retention Register"]
pub mod ret73_reg;
#[doc = "Retention Register"]
pub struct RET74_REG {
    register: VolatileCell<u32>,
}
#[doc = "Retention Register"]
pub mod ret74_reg;
#[doc = "Retention Register"]
pub struct RET75_REG {
    register: VolatileCell<u32>,
}
#[doc = "Retention Register"]
pub mod ret75_reg;
#[doc = "Retention Register"]
pub struct RET76_REG {
    register: VolatileCell<u32>,
}
#[doc = "Retention Register"]
pub mod ret76_reg;
#[doc = "Retention Register"]
pub struct RET77_REG {
    register: VolatileCell<u32>,
}
#[doc = "Retention Register"]
pub mod ret77_reg;
#[doc = "Retention Register"]
pub struct RET78_REG {
    register: VolatileCell<u32>,
}
#[doc = "Retention Register"]
pub mod ret78_reg;
#[doc = "Retention Register"]
pub struct RET79_REG {
    register: VolatileCell<u32>,
}
#[doc = "Retention Register"]
pub mod ret79_reg;
#[doc = "Retention Register"]
pub struct RET80_REG {
    register: VolatileCell<u32>,
}
#[doc = "Retention Register"]
pub mod ret80_reg;
#[doc = "Retention Register"]
pub struct RET81_REG {
    register: VolatileCell<u32>,
}
#[doc = "Retention Register"]
pub mod ret81_reg;
#[doc = "Retention Register"]
pub struct RET82_REG {
    register: VolatileCell<u32>,
}
#[doc = "Retention Register"]
pub mod ret82_reg;
#[doc = "Retention Register"]
pub struct RET83_REG {
    register: VolatileCell<u32>,
}
#[doc = "Retention Register"]
pub mod ret83_reg;
#[doc = "Retention Register"]
pub struct RET84_REG {
    register: VolatileCell<u32>,
}
#[doc = "Retention Register"]
pub mod ret84_reg;
#[doc = "Retention Register"]
pub struct RET85_REG {
    register: VolatileCell<u32>,
}
#[doc = "Retention Register"]
pub mod ret85_reg;
#[doc = "Retention Register"]
pub struct RET86_REG {
    register: VolatileCell<u32>,
}
#[doc = "Retention Register"]
pub mod ret86_reg;
#[doc = "Retention Register"]
pub struct RET87_REG {
    register: VolatileCell<u32>,
}
#[doc = "Retention Register"]
pub mod ret87_reg;
#[doc = "Retention Register"]
pub struct RET88_REG {
    register: VolatileCell<u32>,
}
#[doc = "Retention Register"]
pub mod ret88_reg;
#[doc = "Retention Register"]
pub struct RET89_REG {
    register: VolatileCell<u32>,
}
#[doc = "Retention Register"]
pub mod ret89_reg;
#[doc = "Retention Register"]
pub struct RET90_REG {
    register: VolatileCell<u32>,
}
#[doc = "Retention Register"]
pub mod ret90_reg;
#[doc = "Retention Register"]
pub struct RET91_REG {
    register: VolatileCell<u32>,
}
#[doc = "Retention Register"]
pub mod ret91_reg;
#[doc = "Retention Register"]
pub struct RET92_REG {
    register: VolatileCell<u32>,
}
#[doc = "Retention Register"]
pub mod ret92_reg;
#[doc = "Retention Register"]
pub struct RET93_REG {
    register: VolatileCell<u32>,
}
#[doc = "Retention Register"]
pub mod ret93_reg;
#[doc = "Retention Register"]
pub struct RET94_REG {
    register: VolatileCell<u32>,
}
#[doc = "Retention Register"]
pub mod ret94_reg;
#[doc = "Retention Register"]
pub struct RET95_REG {
    register: VolatileCell<u32>,
}
#[doc = "Retention Register"]
pub mod ret95_reg;
#[doc = "Retention Register"]
pub struct RET96_REG {
    register: VolatileCell<u32>,
}
#[doc = "Retention Register"]
pub mod ret96_reg;
#[doc = "Retention Register"]
pub struct RET97_REG {
    register: VolatileCell<u32>,
}
#[doc = "Retention Register"]
pub mod ret97_reg;
#[doc = "Retention Register"]
pub struct RET98_REG {
    register: VolatileCell<u32>,
}
#[doc = "Retention Register"]
pub mod ret98_reg;
#[doc = "Retention Register"]
pub struct RET99_REG {
    register: VolatileCell<u32>,
}
#[doc = "Retention Register"]
pub mod ret99_reg;
#[doc = "Retention Register"]
pub struct RET100_REG {
    register: VolatileCell<u32>,
}
#[doc = "Retention Register"]
pub mod ret100_reg;
#[doc = "Retention Register"]
pub struct RET101_REG {
    register: VolatileCell<u32>,
}
#[doc = "Retention Register"]
pub mod ret101_reg;
#[doc = "Retention Register"]
pub struct RET102_REG {
    register: VolatileCell<u32>,
}
#[doc = "Retention Register"]
pub mod ret102_reg;
#[doc = "Retention Register"]
pub struct RET103_REG {
    register: VolatileCell<u32>,
}
#[doc = "Retention Register"]
pub mod ret103_reg;
#[doc = "Retention Register"]
pub struct RET104_REG {
    register: VolatileCell<u32>,
}
#[doc = "Retention Register"]
pub mod ret104_reg;
#[doc = "Retention Register"]
pub struct RET105_REG {
    register: VolatileCell<u32>,
}
#[doc = "Retention Register"]
pub mod ret105_reg;
#[doc = "Retention Register"]
pub struct RET106_REG {
    register: VolatileCell<u32>,
}
#[doc = "Retention Register"]
pub mod ret106_reg;
#[doc = "Retention Register"]
pub struct RET107_REG {
    register: VolatileCell<u32>,
}
#[doc = "Retention Register"]
pub mod ret107_reg;
#[doc = "Retention Register"]
pub struct RET108_REG {
    register: VolatileCell<u32>,
}
#[doc = "Retention Register"]
pub mod ret108_reg;
#[doc = "Retention Register"]
pub struct RET109_REG {
    register: VolatileCell<u32>,
}
#[doc = "Retention Register"]
pub mod ret109_reg;
#[doc = "Retention Register"]
pub struct RET110_REG {
    register: VolatileCell<u32>,
}
#[doc = "Retention Register"]
pub mod ret110_reg;
#[doc = "Retention Register"]
pub struct RET111_REG {
    register: VolatileCell<u32>,
}
#[doc = "Retention Register"]
pub mod ret111_reg;
#[doc = "Retention Register"]
pub struct RET112_REG {
    register: VolatileCell<u32>,
}
#[doc = "Retention Register"]
pub mod ret112_reg;
#[doc = "Retention Register"]
pub struct RET113_REG {
    register: VolatileCell<u32>,
}
#[doc = "Retention Register"]
pub mod ret113_reg;
#[doc = "Retention Register"]
pub struct RET114_REG {
    register: VolatileCell<u32>,
}
#[doc = "Retention Register"]
pub mod ret114_reg;
#[doc = "Retention Register"]
pub struct RET115_REG {
    register: VolatileCell<u32>,
}
#[doc = "Retention Register"]
pub mod ret115_reg;
#[doc = "Retention Register"]
pub struct RET116_REG {
    register: VolatileCell<u32>,
}
#[doc = "Retention Register"]
pub mod ret116_reg;
#[doc = "Retention Register"]
pub struct RET117_REG {
    register: VolatileCell<u32>,
}
#[doc = "Retention Register"]
pub mod ret117_reg;
#[doc = "Retention Register"]
pub struct RET118_REG {
    register: VolatileCell<u32>,
}
#[doc = "Retention Register"]
pub mod ret118_reg;
#[doc = "Retention Register"]
pub struct RET119_REG {
    register: VolatileCell<u32>,
}
#[doc = "Retention Register"]
pub mod ret119_reg;
#[doc = "Retention Register"]
pub struct RET120_REG {
    register: VolatileCell<u32>,
}
#[doc = "Retention Register"]
pub mod ret120_reg;
#[doc = "Retention Register"]
pub struct RET121_REG {
    register: VolatileCell<u32>,
}
#[doc = "Retention Register"]
pub mod ret121_reg;
#[doc = "Retention Register"]
pub struct RET122_REG {
    register: VolatileCell<u32>,
}
#[doc = "Retention Register"]
pub mod ret122_reg;
#[doc = "Retention Register"]
pub struct RET123_REG {
    register: VolatileCell<u32>,
}
#[doc = "Retention Register"]
pub mod ret123_reg;
#[doc = "Retention Register"]
pub struct RET124_REG {
    register: VolatileCell<u32>,
}
#[doc = "Retention Register"]
pub mod ret124_reg;
#[doc = "Retention Register"]
pub struct RET125_REG {
    register: VolatileCell<u32>,
}
#[doc = "Retention Register"]
pub mod ret125_reg;
#[doc = "Retention Register"]
pub struct RET126_REG {
    register: VolatileCell<u32>,
}
#[doc = "Retention Register"]
pub mod ret126_reg;
#[doc = "Retention Register"]
pub struct RET127_REG {
    register: VolatileCell<u32>,
}
#[doc = "Retention Register"]
pub mod ret127_reg;

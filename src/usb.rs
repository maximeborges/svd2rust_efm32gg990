use vcell::VolatileCell;
#[doc = r" Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - System Control Register"]
    pub ctrl: CTRL,
    #[doc = "0x04 - System Status Register"]
    pub status: STATUS,
    #[doc = "0x08 - Interrupt Flag Register"]
    pub if_: IF,
    #[doc = "0x0c - Interrupt Flag Set Register"]
    pub ifs: IFS,
    #[doc = "0x10 - Interrupt Flag Clear Register"]
    pub ifc: IFC,
    #[doc = "0x14 - Interrupt Enable Register"]
    pub ien: IEN,
    #[doc = "0x18 - I/O Routing Register"]
    pub route: ROUTE,
    _reserved0: [u8; 245732usize],
    #[doc = "0x3c000 - OTG Control and Status Register"]
    pub gotgctl: GOTGCTL,
    #[doc = "0x3c004 - OTG Interrupt Register"]
    pub gotgint: GOTGINT,
    #[doc = "0x3c008 - AHB Configuration Register"]
    pub gahbcfg: GAHBCFG,
    #[doc = "0x3c00c - USB Configuration Register"]
    pub gusbcfg: GUSBCFG,
    #[doc = "0x3c010 - Reset Register"]
    pub grstctl: GRSTCTL,
    #[doc = "0x3c014 - Interrupt Register"]
    pub gintsts: GINTSTS,
    #[doc = "0x3c018 - Interrupt Mask Register"]
    pub gintmsk: GINTMSK,
    #[doc = "0x3c01c - Receive Status Debug Read Register"]
    pub grxstsr: GRXSTSR,
    #[doc = "0x3c020 - Receive Status Read and Pop Register"]
    pub grxstsp: GRXSTSP,
    #[doc = "0x3c024 - Receive FIFO Size Register"]
    pub grxfsiz: GRXFSIZ,
    #[doc = "0x3c028 - Non-periodic Transmit FIFO Size Register"]
    pub gnptxfsiz: GNPTXFSIZ,
    #[doc = "0x3c02c - Non-periodic Transmit FIFO/Queue Status Register"]
    pub gnptxsts: GNPTXSTS,
    _reserved1: [u8; 44usize],
    #[doc = "0x3c05c - Global DFIFO Configuration Register"]
    pub gdfifocfg: GDFIFOCFG,
    _reserved2: [u8; 160usize],
    #[doc = "0x3c100 - Host Periodic Transmit FIFO Size Register"]
    pub hptxfsiz: HPTXFSIZ,
    #[doc = "0x3c104 - Device IN Endpoint Transmit FIFO 1 Size Register"]
    pub dieptxf1: DIEPTXF1,
    #[doc = "0x3c108 - Device IN Endpoint Transmit FIFO 2 Size Register"]
    pub dieptxf2: DIEPTXF2,
    #[doc = "0x3c10c - Device IN Endpoint Transmit FIFO 3 Size Register"]
    pub dieptxf3: DIEPTXF3,
    #[doc = "0x3c110 - Device IN Endpoint Transmit FIFO 4 Size Register"]
    pub dieptxf4: DIEPTXF4,
    #[doc = "0x3c114 - Device IN Endpoint Transmit FIFO 5 Size Register"]
    pub dieptxf5: DIEPTXF5,
    #[doc = "0x3c118 - Device IN Endpoint Transmit FIFO 6 Size Register"]
    pub dieptxf6: DIEPTXF6,
    _reserved3: [u8; 740usize],
    #[doc = "0x3c400 - Host Configuration Register"]
    pub hcfg: HCFG,
    #[doc = "0x3c404 - Host Frame Interval Register"]
    pub hfir: HFIR,
    #[doc = "0x3c408 - Host Frame Number/Frame Time Remaining Register"]
    pub hfnum: HFNUM,
    _reserved4: [u8; 4usize],
    #[doc = "0x3c410 - Host Periodic Transmit FIFO/Queue Status Register"]
    pub hptxsts: HPTXSTS,
    #[doc = "0x3c414 - Host All Channels Interrupt Register"]
    pub haint: HAINT,
    #[doc = "0x3c418 - Host All Channels Interrupt Mask Register"]
    pub haintmsk: HAINTMSK,
    _reserved5: [u8; 36usize],
    #[doc = "0x3c440 - Host Port Control and Status Register"]
    pub hprt: HPRT,
    _reserved6: [u8; 188usize],
    #[doc = "0x3c500 - Host Channel x Characteristics Register"]
    pub hc0_char: HC0_CHAR,
    _reserved7: [u8; 4usize],
    #[doc = "0x3c508 - Host Channel x Interrupt Register"]
    pub hc0_int: HC0_INT,
    #[doc = "0x3c50c - Host Channel x Interrupt Mask Register"]
    pub hc0_intmsk: HC0_INTMSK,
    #[doc = "0x3c510 - Host Channel x Transfer Size Register"]
    pub hc0_tsiz: HC0_TSIZ,
    #[doc = "0x3c514 - Host Channel x DMA Address Register"]
    pub hc0_dmaaddr: HC0_DMAADDR,
    _reserved8: [u8; 8usize],
    #[doc = "0x3c520 - Host Channel x Characteristics Register"]
    pub hc1_char: HC1_CHAR,
    _reserved9: [u8; 4usize],
    #[doc = "0x3c528 - Host Channel x Interrupt Register"]
    pub hc1_int: HC1_INT,
    #[doc = "0x3c52c - Host Channel x Interrupt Mask Register"]
    pub hc1_intmsk: HC1_INTMSK,
    #[doc = "0x3c530 - Host Channel x Transfer Size Register"]
    pub hc1_tsiz: HC1_TSIZ,
    #[doc = "0x3c534 - Host Channel x DMA Address Register"]
    pub hc1_dmaaddr: HC1_DMAADDR,
    _reserved10: [u8; 8usize],
    #[doc = "0x3c540 - Host Channel x Characteristics Register"]
    pub hc2_char: HC2_CHAR,
    _reserved11: [u8; 4usize],
    #[doc = "0x3c548 - Host Channel x Interrupt Register"]
    pub hc2_int: HC2_INT,
    #[doc = "0x3c54c - Host Channel x Interrupt Mask Register"]
    pub hc2_intmsk: HC2_INTMSK,
    #[doc = "0x3c550 - Host Channel x Transfer Size Register"]
    pub hc2_tsiz: HC2_TSIZ,
    #[doc = "0x3c554 - Host Channel x DMA Address Register"]
    pub hc2_dmaaddr: HC2_DMAADDR,
    _reserved12: [u8; 8usize],
    #[doc = "0x3c560 - Host Channel x Characteristics Register"]
    pub hc3_char: HC3_CHAR,
    _reserved13: [u8; 4usize],
    #[doc = "0x3c568 - Host Channel x Interrupt Register"]
    pub hc3_int: HC3_INT,
    #[doc = "0x3c56c - Host Channel x Interrupt Mask Register"]
    pub hc3_intmsk: HC3_INTMSK,
    #[doc = "0x3c570 - Host Channel x Transfer Size Register"]
    pub hc3_tsiz: HC3_TSIZ,
    #[doc = "0x3c574 - Host Channel x DMA Address Register"]
    pub hc3_dmaaddr: HC3_DMAADDR,
    _reserved14: [u8; 8usize],
    #[doc = "0x3c580 - Host Channel x Characteristics Register"]
    pub hc4_char: HC4_CHAR,
    _reserved15: [u8; 4usize],
    #[doc = "0x3c588 - Host Channel x Interrupt Register"]
    pub hc4_int: HC4_INT,
    #[doc = "0x3c58c - Host Channel x Interrupt Mask Register"]
    pub hc4_intmsk: HC4_INTMSK,
    #[doc = "0x3c590 - Host Channel x Transfer Size Register"]
    pub hc4_tsiz: HC4_TSIZ,
    #[doc = "0x3c594 - Host Channel x DMA Address Register"]
    pub hc4_dmaaddr: HC4_DMAADDR,
    _reserved16: [u8; 8usize],
    #[doc = "0x3c5a0 - Host Channel x Characteristics Register"]
    pub hc5_char: HC5_CHAR,
    _reserved17: [u8; 4usize],
    #[doc = "0x3c5a8 - Host Channel x Interrupt Register"]
    pub hc5_int: HC5_INT,
    #[doc = "0x3c5ac - Host Channel x Interrupt Mask Register"]
    pub hc5_intmsk: HC5_INTMSK,
    #[doc = "0x3c5b0 - Host Channel x Transfer Size Register"]
    pub hc5_tsiz: HC5_TSIZ,
    #[doc = "0x3c5b4 - Host Channel x DMA Address Register"]
    pub hc5_dmaaddr: HC5_DMAADDR,
    _reserved18: [u8; 8usize],
    #[doc = "0x3c5c0 - Host Channel x Characteristics Register"]
    pub hc6_char: HC6_CHAR,
    _reserved19: [u8; 4usize],
    #[doc = "0x3c5c8 - Host Channel x Interrupt Register"]
    pub hc6_int: HC6_INT,
    #[doc = "0x3c5cc - Host Channel x Interrupt Mask Register"]
    pub hc6_intmsk: HC6_INTMSK,
    #[doc = "0x3c5d0 - Host Channel x Transfer Size Register"]
    pub hc6_tsiz: HC6_TSIZ,
    #[doc = "0x3c5d4 - Host Channel x DMA Address Register"]
    pub hc6_dmaaddr: HC6_DMAADDR,
    _reserved20: [u8; 8usize],
    #[doc = "0x3c5e0 - Host Channel x Characteristics Register"]
    pub hc7_char: HC7_CHAR,
    _reserved21: [u8; 4usize],
    #[doc = "0x3c5e8 - Host Channel x Interrupt Register"]
    pub hc7_int: HC7_INT,
    #[doc = "0x3c5ec - Host Channel x Interrupt Mask Register"]
    pub hc7_intmsk: HC7_INTMSK,
    #[doc = "0x3c5f0 - Host Channel x Transfer Size Register"]
    pub hc7_tsiz: HC7_TSIZ,
    #[doc = "0x3c5f4 - Host Channel x DMA Address Register"]
    pub hc7_dmaaddr: HC7_DMAADDR,
    _reserved22: [u8; 8usize],
    #[doc = "0x3c600 - Host Channel x Characteristics Register"]
    pub hc8_char: HC8_CHAR,
    _reserved23: [u8; 4usize],
    #[doc = "0x3c608 - Host Channel x Interrupt Register"]
    pub hc8_int: HC8_INT,
    #[doc = "0x3c60c - Host Channel x Interrupt Mask Register"]
    pub hc8_intmsk: HC8_INTMSK,
    #[doc = "0x3c610 - Host Channel x Transfer Size Register"]
    pub hc8_tsiz: HC8_TSIZ,
    #[doc = "0x3c614 - Host Channel x DMA Address Register"]
    pub hc8_dmaaddr: HC8_DMAADDR,
    _reserved24: [u8; 8usize],
    #[doc = "0x3c620 - Host Channel x Characteristics Register"]
    pub hc9_char: HC9_CHAR,
    _reserved25: [u8; 4usize],
    #[doc = "0x3c628 - Host Channel x Interrupt Register"]
    pub hc9_int: HC9_INT,
    #[doc = "0x3c62c - Host Channel x Interrupt Mask Register"]
    pub hc9_intmsk: HC9_INTMSK,
    #[doc = "0x3c630 - Host Channel x Transfer Size Register"]
    pub hc9_tsiz: HC9_TSIZ,
    #[doc = "0x3c634 - Host Channel x DMA Address Register"]
    pub hc9_dmaaddr: HC9_DMAADDR,
    _reserved26: [u8; 8usize],
    #[doc = "0x3c640 - Host Channel x Characteristics Register"]
    pub hc10_char: HC10_CHAR,
    _reserved27: [u8; 4usize],
    #[doc = "0x3c648 - Host Channel x Interrupt Register"]
    pub hc10_int: HC10_INT,
    #[doc = "0x3c64c - Host Channel x Interrupt Mask Register"]
    pub hc10_intmsk: HC10_INTMSK,
    #[doc = "0x3c650 - Host Channel x Transfer Size Register"]
    pub hc10_tsiz: HC10_TSIZ,
    #[doc = "0x3c654 - Host Channel x DMA Address Register"]
    pub hc10_dmaaddr: HC10_DMAADDR,
    _reserved28: [u8; 8usize],
    #[doc = "0x3c660 - Host Channel x Characteristics Register"]
    pub hc11_char: HC11_CHAR,
    _reserved29: [u8; 4usize],
    #[doc = "0x3c668 - Host Channel x Interrupt Register"]
    pub hc11_int: HC11_INT,
    #[doc = "0x3c66c - Host Channel x Interrupt Mask Register"]
    pub hc11_intmsk: HC11_INTMSK,
    #[doc = "0x3c670 - Host Channel x Transfer Size Register"]
    pub hc11_tsiz: HC11_TSIZ,
    #[doc = "0x3c674 - Host Channel x DMA Address Register"]
    pub hc11_dmaaddr: HC11_DMAADDR,
    _reserved30: [u8; 8usize],
    #[doc = "0x3c680 - Host Channel x Characteristics Register"]
    pub hc12_char: HC12_CHAR,
    _reserved31: [u8; 4usize],
    #[doc = "0x3c688 - Host Channel x Interrupt Register"]
    pub hc12_int: HC12_INT,
    #[doc = "0x3c68c - Host Channel x Interrupt Mask Register"]
    pub hc12_intmsk: HC12_INTMSK,
    #[doc = "0x3c690 - Host Channel x Transfer Size Register"]
    pub hc12_tsiz: HC12_TSIZ,
    #[doc = "0x3c694 - Host Channel x DMA Address Register"]
    pub hc12_dmaaddr: HC12_DMAADDR,
    _reserved32: [u8; 8usize],
    #[doc = "0x3c6a0 - Host Channel x Characteristics Register"]
    pub hc13_char: HC13_CHAR,
    _reserved33: [u8; 4usize],
    #[doc = "0x3c6a8 - Host Channel x Interrupt Register"]
    pub hc13_int: HC13_INT,
    #[doc = "0x3c6ac - Host Channel x Interrupt Mask Register"]
    pub hc13_intmsk: HC13_INTMSK,
    #[doc = "0x3c6b0 - Host Channel x Transfer Size Register"]
    pub hc13_tsiz: HC13_TSIZ,
    #[doc = "0x3c6b4 - Host Channel x DMA Address Register"]
    pub hc13_dmaaddr: HC13_DMAADDR,
    _reserved34: [u8; 328usize],
    #[doc = "0x3c800 - Device Configuration Register"]
    pub dcfg: DCFG,
    #[doc = "0x3c804 - Device Control Register"]
    pub dctl: DCTL,
    #[doc = "0x3c808 - Device Status Register"]
    pub dsts: DSTS,
    _reserved35: [u8; 4usize],
    #[doc = "0x3c810 - Device IN Endpoint Common Interrupt Mask Register"]
    pub diepmsk: DIEPMSK,
    #[doc = "0x3c814 - Device OUT Endpoint Common Interrupt Mask Register"]
    pub doepmsk: DOEPMSK,
    #[doc = "0x3c818 - Device All Endpoints Interrupt Register"]
    pub daint: DAINT,
    #[doc = "0x3c81c - Device All Endpoints Interrupt Mask Register"]
    pub daintmsk: DAINTMSK,
    _reserved36: [u8; 8usize],
    #[doc = "0x3c828 - Device VBUS Discharge Time Register"]
    pub dvbusdis: DVBUSDIS,
    #[doc = "0x3c82c - Device VBUS Pulsing Time Register"]
    pub dvbuspulse: DVBUSPULSE,
    _reserved37: [u8; 4usize],
    #[doc = "0x3c834 - Device IN Endpoint FIFO Empty Interrupt Mask Register"]
    pub diepempmsk: DIEPEMPMSK,
    _reserved38: [u8; 200usize],
    #[doc = "0x3c900 - Device IN Endpoint 0 Control Register"]
    pub diep0ctl: DIEP0CTL,
    _reserved39: [u8; 4usize],
    #[doc = "0x3c908 - Device IN Endpoint 0 Interrupt Register"]
    pub diep0int: DIEP0INT,
    _reserved40: [u8; 4usize],
    #[doc = "0x3c910 - Device IN Endpoint 0 Transfer Size Register"]
    pub diep0tsiz: DIEP0TSIZ,
    #[doc = "0x3c914 - Device IN Endpoint 0 DMA Address Register"]
    pub diep0dmaaddr: DIEP0DMAADDR,
    #[doc = "0x3c918 - Device IN Endpoint 0 Transmit FIFO Status Register"]
    pub diep0txfsts: DIEP0TXFSTS,
    _reserved41: [u8; 4usize],
    #[doc = "0x3c920 - Device IN Endpoint x+1 Control Register"]
    pub diep0_ctl: DIEP0_CTL,
    _reserved42: [u8; 4usize],
    #[doc = "0x3c928 - Device IN Endpoint x+1 Interrupt Register"]
    pub diep0_int: DIEP0_INT,
    _reserved43: [u8; 4usize],
    #[doc = "0x3c930 - Device IN Endpoint x+1 Transfer Size Register"]
    pub diep0_tsiz: DIEP0_TSIZ,
    #[doc = "0x3c934 - Device IN Endpoint x+1 DMA Address Register"]
    pub diep0_dmaaddr: DIEP0_DMAADDR,
    #[doc = "0x3c938 - Device IN Endpoint x+1 Transmit FIFO Status Register"]
    pub diep0_txfsts: DIEP0_TXFSTS,
    _reserved44: [u8; 4usize],
    #[doc = "0x3c940 - Device IN Endpoint x+1 Control Register"]
    pub diep1_ctl: DIEP1_CTL,
    _reserved45: [u8; 4usize],
    #[doc = "0x3c948 - Device IN Endpoint x+1 Interrupt Register"]
    pub diep1_int: DIEP1_INT,
    _reserved46: [u8; 4usize],
    #[doc = "0x3c950 - Device IN Endpoint x+1 Transfer Size Register"]
    pub diep1_tsiz: DIEP1_TSIZ,
    #[doc = "0x3c954 - Device IN Endpoint x+1 DMA Address Register"]
    pub diep1_dmaaddr: DIEP1_DMAADDR,
    #[doc = "0x3c958 - Device IN Endpoint x+1 Transmit FIFO Status Register"]
    pub diep1_txfsts: DIEP1_TXFSTS,
    _reserved47: [u8; 4usize],
    #[doc = "0x3c960 - Device IN Endpoint x+1 Control Register"]
    pub diep2_ctl: DIEP2_CTL,
    _reserved48: [u8; 4usize],
    #[doc = "0x3c968 - Device IN Endpoint x+1 Interrupt Register"]
    pub diep2_int: DIEP2_INT,
    _reserved49: [u8; 4usize],
    #[doc = "0x3c970 - Device IN Endpoint x+1 Transfer Size Register"]
    pub diep2_tsiz: DIEP2_TSIZ,
    #[doc = "0x3c974 - Device IN Endpoint x+1 DMA Address Register"]
    pub diep2_dmaaddr: DIEP2_DMAADDR,
    #[doc = "0x3c978 - Device IN Endpoint x+1 Transmit FIFO Status Register"]
    pub diep2_txfsts: DIEP2_TXFSTS,
    _reserved50: [u8; 4usize],
    #[doc = "0x3c980 - Device IN Endpoint x+1 Control Register"]
    pub diep3_ctl: DIEP3_CTL,
    _reserved51: [u8; 4usize],
    #[doc = "0x3c988 - Device IN Endpoint x+1 Interrupt Register"]
    pub diep3_int: DIEP3_INT,
    _reserved52: [u8; 4usize],
    #[doc = "0x3c990 - Device IN Endpoint x+1 Transfer Size Register"]
    pub diep3_tsiz: DIEP3_TSIZ,
    #[doc = "0x3c994 - Device IN Endpoint x+1 DMA Address Register"]
    pub diep3_dmaaddr: DIEP3_DMAADDR,
    #[doc = "0x3c998 - Device IN Endpoint x+1 Transmit FIFO Status Register"]
    pub diep3_txfsts: DIEP3_TXFSTS,
    _reserved53: [u8; 4usize],
    #[doc = "0x3c9a0 - Device IN Endpoint x+1 Control Register"]
    pub diep4_ctl: DIEP4_CTL,
    _reserved54: [u8; 4usize],
    #[doc = "0x3c9a8 - Device IN Endpoint x+1 Interrupt Register"]
    pub diep4_int: DIEP4_INT,
    _reserved55: [u8; 4usize],
    #[doc = "0x3c9b0 - Device IN Endpoint x+1 Transfer Size Register"]
    pub diep4_tsiz: DIEP4_TSIZ,
    #[doc = "0x3c9b4 - Device IN Endpoint x+1 DMA Address Register"]
    pub diep4_dmaaddr: DIEP4_DMAADDR,
    #[doc = "0x3c9b8 - Device IN Endpoint x+1 Transmit FIFO Status Register"]
    pub diep4_txfsts: DIEP4_TXFSTS,
    _reserved56: [u8; 4usize],
    #[doc = "0x3c9c0 - Device IN Endpoint x+1 Control Register"]
    pub diep5_ctl: DIEP5_CTL,
    _reserved57: [u8; 4usize],
    #[doc = "0x3c9c8 - Device IN Endpoint x+1 Interrupt Register"]
    pub diep5_int: DIEP5_INT,
    _reserved58: [u8; 4usize],
    #[doc = "0x3c9d0 - Device IN Endpoint x+1 Transfer Size Register"]
    pub diep5_tsiz: DIEP5_TSIZ,
    #[doc = "0x3c9d4 - Device IN Endpoint x+1 DMA Address Register"]
    pub diep5_dmaaddr: DIEP5_DMAADDR,
    #[doc = "0x3c9d8 - Device IN Endpoint x+1 Transmit FIFO Status Register"]
    pub diep5_txfsts: DIEP5_TXFSTS,
    _reserved59: [u8; 292usize],
    #[doc = "0x3cb00 - Device OUT Endpoint 0 Control Register"]
    pub doep0ctl: DOEP0CTL,
    _reserved60: [u8; 4usize],
    #[doc = "0x3cb08 - Device OUT Endpoint 0 Interrupt Register"]
    pub doep0int: DOEP0INT,
    _reserved61: [u8; 4usize],
    #[doc = "0x3cb10 - Device OUT Endpoint 0 Transfer Size Register"]
    pub doep0tsiz: DOEP0TSIZ,
    #[doc = "0x3cb14 - Device OUT Endpoint 0 DMA Address Register"]
    pub doep0dmaaddr: DOEP0DMAADDR,
    _reserved62: [u8; 8usize],
    #[doc = "0x3cb20 - Device OUT Endpoint x+1 Control Register"]
    pub doep0_ctl: DOEP0_CTL,
    _reserved63: [u8; 4usize],
    #[doc = "0x3cb28 - Device OUT Endpoint x+1 Interrupt Register"]
    pub doep0_int: DOEP0_INT,
    _reserved64: [u8; 4usize],
    #[doc = "0x3cb30 - Device OUT Endpoint x+1 Transfer Size Register"]
    pub doep0_tsiz: DOEP0_TSIZ,
    #[doc = "0x3cb34 - Device OUT Endpoint x+1 DMA Address Register"]
    pub doep0_dmaaddr: DOEP0_DMAADDR,
    _reserved65: [u8; 8usize],
    #[doc = "0x3cb40 - Device OUT Endpoint x+1 Control Register"]
    pub doep1_ctl: DOEP1_CTL,
    _reserved66: [u8; 4usize],
    #[doc = "0x3cb48 - Device OUT Endpoint x+1 Interrupt Register"]
    pub doep1_int: DOEP1_INT,
    _reserved67: [u8; 4usize],
    #[doc = "0x3cb50 - Device OUT Endpoint x+1 Transfer Size Register"]
    pub doep1_tsiz: DOEP1_TSIZ,
    #[doc = "0x3cb54 - Device OUT Endpoint x+1 DMA Address Register"]
    pub doep1_dmaaddr: DOEP1_DMAADDR,
    _reserved68: [u8; 8usize],
    #[doc = "0x3cb60 - Device OUT Endpoint x+1 Control Register"]
    pub doep2_ctl: DOEP2_CTL,
    _reserved69: [u8; 4usize],
    #[doc = "0x3cb68 - Device OUT Endpoint x+1 Interrupt Register"]
    pub doep2_int: DOEP2_INT,
    _reserved70: [u8; 4usize],
    #[doc = "0x3cb70 - Device OUT Endpoint x+1 Transfer Size Register"]
    pub doep2_tsiz: DOEP2_TSIZ,
    #[doc = "0x3cb74 - Device OUT Endpoint x+1 DMA Address Register"]
    pub doep2_dmaaddr: DOEP2_DMAADDR,
    _reserved71: [u8; 8usize],
    #[doc = "0x3cb80 - Device OUT Endpoint x+1 Control Register"]
    pub doep3_ctl: DOEP3_CTL,
    _reserved72: [u8; 4usize],
    #[doc = "0x3cb88 - Device OUT Endpoint x+1 Interrupt Register"]
    pub doep3_int: DOEP3_INT,
    _reserved73: [u8; 4usize],
    #[doc = "0x3cb90 - Device OUT Endpoint x+1 Transfer Size Register"]
    pub doep3_tsiz: DOEP3_TSIZ,
    #[doc = "0x3cb94 - Device OUT Endpoint x+1 DMA Address Register"]
    pub doep3_dmaaddr: DOEP3_DMAADDR,
    _reserved74: [u8; 8usize],
    #[doc = "0x3cba0 - Device OUT Endpoint x+1 Control Register"]
    pub doep4_ctl: DOEP4_CTL,
    _reserved75: [u8; 4usize],
    #[doc = "0x3cba8 - Device OUT Endpoint x+1 Interrupt Register"]
    pub doep4_int: DOEP4_INT,
    _reserved76: [u8; 4usize],
    #[doc = "0x3cbb0 - Device OUT Endpoint x+1 Transfer Size Register"]
    pub doep4_tsiz: DOEP4_TSIZ,
    #[doc = "0x3cbb4 - Device OUT Endpoint x+1 DMA Address Register"]
    pub doep4_dmaaddr: DOEP4_DMAADDR,
    _reserved77: [u8; 8usize],
    #[doc = "0x3cbc0 - Device OUT Endpoint x+1 Control Register"]
    pub doep5_ctl: DOEP5_CTL,
    _reserved78: [u8; 4usize],
    #[doc = "0x3cbc8 - Device OUT Endpoint x+1 Interrupt Register"]
    pub doep5_int: DOEP5_INT,
    _reserved79: [u8; 4usize],
    #[doc = "0x3cbd0 - Device OUT Endpoint x+1 Transfer Size Register"]
    pub doep5_tsiz: DOEP5_TSIZ,
    #[doc = "0x3cbd4 - Device OUT Endpoint x+1 DMA Address Register"]
    pub doep5_dmaaddr: DOEP5_DMAADDR,
    _reserved80: [u8; 552usize],
    #[doc = "0x3ce00 - Power and Clock Gating Control Register"]
    pub pcgcctl: PCGCCTL,
}
#[doc = "System Control Register"]
pub struct CTRL {
    register: VolatileCell<u32>,
}
#[doc = "System Control Register"]
pub mod ctrl;
#[doc = "System Status Register"]
pub struct STATUS {
    register: VolatileCell<u32>,
}
#[doc = "System Status Register"]
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
#[doc = "I/O Routing Register"]
pub struct ROUTE {
    register: VolatileCell<u32>,
}
#[doc = "I/O Routing Register"]
pub mod route;
#[doc = "OTG Control and Status Register"]
pub struct GOTGCTL {
    register: VolatileCell<u32>,
}
#[doc = "OTG Control and Status Register"]
pub mod gotgctl;
#[doc = "OTG Interrupt Register"]
pub struct GOTGINT {
    register: VolatileCell<u32>,
}
#[doc = "OTG Interrupt Register"]
pub mod gotgint;
#[doc = "AHB Configuration Register"]
pub struct GAHBCFG {
    register: VolatileCell<u32>,
}
#[doc = "AHB Configuration Register"]
pub mod gahbcfg;
#[doc = "USB Configuration Register"]
pub struct GUSBCFG {
    register: VolatileCell<u32>,
}
#[doc = "USB Configuration Register"]
pub mod gusbcfg;
#[doc = "Reset Register"]
pub struct GRSTCTL {
    register: VolatileCell<u32>,
}
#[doc = "Reset Register"]
pub mod grstctl;
#[doc = "Interrupt Register"]
pub struct GINTSTS {
    register: VolatileCell<u32>,
}
#[doc = "Interrupt Register"]
pub mod gintsts;
#[doc = "Interrupt Mask Register"]
pub struct GINTMSK {
    register: VolatileCell<u32>,
}
#[doc = "Interrupt Mask Register"]
pub mod gintmsk;
#[doc = "Receive Status Debug Read Register"]
pub struct GRXSTSR {
    register: VolatileCell<u32>,
}
#[doc = "Receive Status Debug Read Register"]
pub mod grxstsr;
#[doc = "Receive Status Read and Pop Register"]
pub struct GRXSTSP {
    register: VolatileCell<u32>,
}
#[doc = "Receive Status Read and Pop Register"]
pub mod grxstsp;
#[doc = "Receive FIFO Size Register"]
pub struct GRXFSIZ {
    register: VolatileCell<u32>,
}
#[doc = "Receive FIFO Size Register"]
pub mod grxfsiz;
#[doc = "Non-periodic Transmit FIFO Size Register"]
pub struct GNPTXFSIZ {
    register: VolatileCell<u32>,
}
#[doc = "Non-periodic Transmit FIFO Size Register"]
pub mod gnptxfsiz;
#[doc = "Non-periodic Transmit FIFO/Queue Status Register"]
pub struct GNPTXSTS {
    register: VolatileCell<u32>,
}
#[doc = "Non-periodic Transmit FIFO/Queue Status Register"]
pub mod gnptxsts;
#[doc = "Global DFIFO Configuration Register"]
pub struct GDFIFOCFG {
    register: VolatileCell<u32>,
}
#[doc = "Global DFIFO Configuration Register"]
pub mod gdfifocfg;
#[doc = "Host Periodic Transmit FIFO Size Register"]
pub struct HPTXFSIZ {
    register: VolatileCell<u32>,
}
#[doc = "Host Periodic Transmit FIFO Size Register"]
pub mod hptxfsiz;
#[doc = "Device IN Endpoint Transmit FIFO 1 Size Register"]
pub struct DIEPTXF1 {
    register: VolatileCell<u32>,
}
#[doc = "Device IN Endpoint Transmit FIFO 1 Size Register"]
pub mod dieptxf1;
#[doc = "Device IN Endpoint Transmit FIFO 2 Size Register"]
pub struct DIEPTXF2 {
    register: VolatileCell<u32>,
}
#[doc = "Device IN Endpoint Transmit FIFO 2 Size Register"]
pub mod dieptxf2;
#[doc = "Device IN Endpoint Transmit FIFO 3 Size Register"]
pub struct DIEPTXF3 {
    register: VolatileCell<u32>,
}
#[doc = "Device IN Endpoint Transmit FIFO 3 Size Register"]
pub mod dieptxf3;
#[doc = "Device IN Endpoint Transmit FIFO 4 Size Register"]
pub struct DIEPTXF4 {
    register: VolatileCell<u32>,
}
#[doc = "Device IN Endpoint Transmit FIFO 4 Size Register"]
pub mod dieptxf4;
#[doc = "Device IN Endpoint Transmit FIFO 5 Size Register"]
pub struct DIEPTXF5 {
    register: VolatileCell<u32>,
}
#[doc = "Device IN Endpoint Transmit FIFO 5 Size Register"]
pub mod dieptxf5;
#[doc = "Device IN Endpoint Transmit FIFO 6 Size Register"]
pub struct DIEPTXF6 {
    register: VolatileCell<u32>,
}
#[doc = "Device IN Endpoint Transmit FIFO 6 Size Register"]
pub mod dieptxf6;
#[doc = "Host Configuration Register"]
pub struct HCFG {
    register: VolatileCell<u32>,
}
#[doc = "Host Configuration Register"]
pub mod hcfg;
#[doc = "Host Frame Interval Register"]
pub struct HFIR {
    register: VolatileCell<u32>,
}
#[doc = "Host Frame Interval Register"]
pub mod hfir;
#[doc = "Host Frame Number/Frame Time Remaining Register"]
pub struct HFNUM {
    register: VolatileCell<u32>,
}
#[doc = "Host Frame Number/Frame Time Remaining Register"]
pub mod hfnum;
#[doc = "Host Periodic Transmit FIFO/Queue Status Register"]
pub struct HPTXSTS {
    register: VolatileCell<u32>,
}
#[doc = "Host Periodic Transmit FIFO/Queue Status Register"]
pub mod hptxsts;
#[doc = "Host All Channels Interrupt Register"]
pub struct HAINT {
    register: VolatileCell<u32>,
}
#[doc = "Host All Channels Interrupt Register"]
pub mod haint;
#[doc = "Host All Channels Interrupt Mask Register"]
pub struct HAINTMSK {
    register: VolatileCell<u32>,
}
#[doc = "Host All Channels Interrupt Mask Register"]
pub mod haintmsk;
#[doc = "Host Port Control and Status Register"]
pub struct HPRT {
    register: VolatileCell<u32>,
}
#[doc = "Host Port Control and Status Register"]
pub mod hprt;
#[doc = "Host Channel x Characteristics Register"]
pub struct HC0_CHAR {
    register: VolatileCell<u32>,
}
#[doc = "Host Channel x Characteristics Register"]
pub mod hc0_char;
#[doc = "Host Channel x Interrupt Register"]
pub struct HC0_INT {
    register: VolatileCell<u32>,
}
#[doc = "Host Channel x Interrupt Register"]
pub mod hc0_int;
#[doc = "Host Channel x Interrupt Mask Register"]
pub struct HC0_INTMSK {
    register: VolatileCell<u32>,
}
#[doc = "Host Channel x Interrupt Mask Register"]
pub mod hc0_intmsk;
#[doc = "Host Channel x Transfer Size Register"]
pub struct HC0_TSIZ {
    register: VolatileCell<u32>,
}
#[doc = "Host Channel x Transfer Size Register"]
pub mod hc0_tsiz;
#[doc = "Host Channel x DMA Address Register"]
pub struct HC0_DMAADDR {
    register: VolatileCell<u32>,
}
#[doc = "Host Channel x DMA Address Register"]
pub mod hc0_dmaaddr;
#[doc = "Host Channel x Characteristics Register"]
pub struct HC1_CHAR {
    register: VolatileCell<u32>,
}
#[doc = "Host Channel x Characteristics Register"]
pub mod hc1_char;
#[doc = "Host Channel x Interrupt Register"]
pub struct HC1_INT {
    register: VolatileCell<u32>,
}
#[doc = "Host Channel x Interrupt Register"]
pub mod hc1_int;
#[doc = "Host Channel x Interrupt Mask Register"]
pub struct HC1_INTMSK {
    register: VolatileCell<u32>,
}
#[doc = "Host Channel x Interrupt Mask Register"]
pub mod hc1_intmsk;
#[doc = "Host Channel x Transfer Size Register"]
pub struct HC1_TSIZ {
    register: VolatileCell<u32>,
}
#[doc = "Host Channel x Transfer Size Register"]
pub mod hc1_tsiz;
#[doc = "Host Channel x DMA Address Register"]
pub struct HC1_DMAADDR {
    register: VolatileCell<u32>,
}
#[doc = "Host Channel x DMA Address Register"]
pub mod hc1_dmaaddr;
#[doc = "Host Channel x Characteristics Register"]
pub struct HC2_CHAR {
    register: VolatileCell<u32>,
}
#[doc = "Host Channel x Characteristics Register"]
pub mod hc2_char;
#[doc = "Host Channel x Interrupt Register"]
pub struct HC2_INT {
    register: VolatileCell<u32>,
}
#[doc = "Host Channel x Interrupt Register"]
pub mod hc2_int;
#[doc = "Host Channel x Interrupt Mask Register"]
pub struct HC2_INTMSK {
    register: VolatileCell<u32>,
}
#[doc = "Host Channel x Interrupt Mask Register"]
pub mod hc2_intmsk;
#[doc = "Host Channel x Transfer Size Register"]
pub struct HC2_TSIZ {
    register: VolatileCell<u32>,
}
#[doc = "Host Channel x Transfer Size Register"]
pub mod hc2_tsiz;
#[doc = "Host Channel x DMA Address Register"]
pub struct HC2_DMAADDR {
    register: VolatileCell<u32>,
}
#[doc = "Host Channel x DMA Address Register"]
pub mod hc2_dmaaddr;
#[doc = "Host Channel x Characteristics Register"]
pub struct HC3_CHAR {
    register: VolatileCell<u32>,
}
#[doc = "Host Channel x Characteristics Register"]
pub mod hc3_char;
#[doc = "Host Channel x Interrupt Register"]
pub struct HC3_INT {
    register: VolatileCell<u32>,
}
#[doc = "Host Channel x Interrupt Register"]
pub mod hc3_int;
#[doc = "Host Channel x Interrupt Mask Register"]
pub struct HC3_INTMSK {
    register: VolatileCell<u32>,
}
#[doc = "Host Channel x Interrupt Mask Register"]
pub mod hc3_intmsk;
#[doc = "Host Channel x Transfer Size Register"]
pub struct HC3_TSIZ {
    register: VolatileCell<u32>,
}
#[doc = "Host Channel x Transfer Size Register"]
pub mod hc3_tsiz;
#[doc = "Host Channel x DMA Address Register"]
pub struct HC3_DMAADDR {
    register: VolatileCell<u32>,
}
#[doc = "Host Channel x DMA Address Register"]
pub mod hc3_dmaaddr;
#[doc = "Host Channel x Characteristics Register"]
pub struct HC4_CHAR {
    register: VolatileCell<u32>,
}
#[doc = "Host Channel x Characteristics Register"]
pub mod hc4_char;
#[doc = "Host Channel x Interrupt Register"]
pub struct HC4_INT {
    register: VolatileCell<u32>,
}
#[doc = "Host Channel x Interrupt Register"]
pub mod hc4_int;
#[doc = "Host Channel x Interrupt Mask Register"]
pub struct HC4_INTMSK {
    register: VolatileCell<u32>,
}
#[doc = "Host Channel x Interrupt Mask Register"]
pub mod hc4_intmsk;
#[doc = "Host Channel x Transfer Size Register"]
pub struct HC4_TSIZ {
    register: VolatileCell<u32>,
}
#[doc = "Host Channel x Transfer Size Register"]
pub mod hc4_tsiz;
#[doc = "Host Channel x DMA Address Register"]
pub struct HC4_DMAADDR {
    register: VolatileCell<u32>,
}
#[doc = "Host Channel x DMA Address Register"]
pub mod hc4_dmaaddr;
#[doc = "Host Channel x Characteristics Register"]
pub struct HC5_CHAR {
    register: VolatileCell<u32>,
}
#[doc = "Host Channel x Characteristics Register"]
pub mod hc5_char;
#[doc = "Host Channel x Interrupt Register"]
pub struct HC5_INT {
    register: VolatileCell<u32>,
}
#[doc = "Host Channel x Interrupt Register"]
pub mod hc5_int;
#[doc = "Host Channel x Interrupt Mask Register"]
pub struct HC5_INTMSK {
    register: VolatileCell<u32>,
}
#[doc = "Host Channel x Interrupt Mask Register"]
pub mod hc5_intmsk;
#[doc = "Host Channel x Transfer Size Register"]
pub struct HC5_TSIZ {
    register: VolatileCell<u32>,
}
#[doc = "Host Channel x Transfer Size Register"]
pub mod hc5_tsiz;
#[doc = "Host Channel x DMA Address Register"]
pub struct HC5_DMAADDR {
    register: VolatileCell<u32>,
}
#[doc = "Host Channel x DMA Address Register"]
pub mod hc5_dmaaddr;
#[doc = "Host Channel x Characteristics Register"]
pub struct HC6_CHAR {
    register: VolatileCell<u32>,
}
#[doc = "Host Channel x Characteristics Register"]
pub mod hc6_char;
#[doc = "Host Channel x Interrupt Register"]
pub struct HC6_INT {
    register: VolatileCell<u32>,
}
#[doc = "Host Channel x Interrupt Register"]
pub mod hc6_int;
#[doc = "Host Channel x Interrupt Mask Register"]
pub struct HC6_INTMSK {
    register: VolatileCell<u32>,
}
#[doc = "Host Channel x Interrupt Mask Register"]
pub mod hc6_intmsk;
#[doc = "Host Channel x Transfer Size Register"]
pub struct HC6_TSIZ {
    register: VolatileCell<u32>,
}
#[doc = "Host Channel x Transfer Size Register"]
pub mod hc6_tsiz;
#[doc = "Host Channel x DMA Address Register"]
pub struct HC6_DMAADDR {
    register: VolatileCell<u32>,
}
#[doc = "Host Channel x DMA Address Register"]
pub mod hc6_dmaaddr;
#[doc = "Host Channel x Characteristics Register"]
pub struct HC7_CHAR {
    register: VolatileCell<u32>,
}
#[doc = "Host Channel x Characteristics Register"]
pub mod hc7_char;
#[doc = "Host Channel x Interrupt Register"]
pub struct HC7_INT {
    register: VolatileCell<u32>,
}
#[doc = "Host Channel x Interrupt Register"]
pub mod hc7_int;
#[doc = "Host Channel x Interrupt Mask Register"]
pub struct HC7_INTMSK {
    register: VolatileCell<u32>,
}
#[doc = "Host Channel x Interrupt Mask Register"]
pub mod hc7_intmsk;
#[doc = "Host Channel x Transfer Size Register"]
pub struct HC7_TSIZ {
    register: VolatileCell<u32>,
}
#[doc = "Host Channel x Transfer Size Register"]
pub mod hc7_tsiz;
#[doc = "Host Channel x DMA Address Register"]
pub struct HC7_DMAADDR {
    register: VolatileCell<u32>,
}
#[doc = "Host Channel x DMA Address Register"]
pub mod hc7_dmaaddr;
#[doc = "Host Channel x Characteristics Register"]
pub struct HC8_CHAR {
    register: VolatileCell<u32>,
}
#[doc = "Host Channel x Characteristics Register"]
pub mod hc8_char;
#[doc = "Host Channel x Interrupt Register"]
pub struct HC8_INT {
    register: VolatileCell<u32>,
}
#[doc = "Host Channel x Interrupt Register"]
pub mod hc8_int;
#[doc = "Host Channel x Interrupt Mask Register"]
pub struct HC8_INTMSK {
    register: VolatileCell<u32>,
}
#[doc = "Host Channel x Interrupt Mask Register"]
pub mod hc8_intmsk;
#[doc = "Host Channel x Transfer Size Register"]
pub struct HC8_TSIZ {
    register: VolatileCell<u32>,
}
#[doc = "Host Channel x Transfer Size Register"]
pub mod hc8_tsiz;
#[doc = "Host Channel x DMA Address Register"]
pub struct HC8_DMAADDR {
    register: VolatileCell<u32>,
}
#[doc = "Host Channel x DMA Address Register"]
pub mod hc8_dmaaddr;
#[doc = "Host Channel x Characteristics Register"]
pub struct HC9_CHAR {
    register: VolatileCell<u32>,
}
#[doc = "Host Channel x Characteristics Register"]
pub mod hc9_char;
#[doc = "Host Channel x Interrupt Register"]
pub struct HC9_INT {
    register: VolatileCell<u32>,
}
#[doc = "Host Channel x Interrupt Register"]
pub mod hc9_int;
#[doc = "Host Channel x Interrupt Mask Register"]
pub struct HC9_INTMSK {
    register: VolatileCell<u32>,
}
#[doc = "Host Channel x Interrupt Mask Register"]
pub mod hc9_intmsk;
#[doc = "Host Channel x Transfer Size Register"]
pub struct HC9_TSIZ {
    register: VolatileCell<u32>,
}
#[doc = "Host Channel x Transfer Size Register"]
pub mod hc9_tsiz;
#[doc = "Host Channel x DMA Address Register"]
pub struct HC9_DMAADDR {
    register: VolatileCell<u32>,
}
#[doc = "Host Channel x DMA Address Register"]
pub mod hc9_dmaaddr;
#[doc = "Host Channel x Characteristics Register"]
pub struct HC10_CHAR {
    register: VolatileCell<u32>,
}
#[doc = "Host Channel x Characteristics Register"]
pub mod hc10_char;
#[doc = "Host Channel x Interrupt Register"]
pub struct HC10_INT {
    register: VolatileCell<u32>,
}
#[doc = "Host Channel x Interrupt Register"]
pub mod hc10_int;
#[doc = "Host Channel x Interrupt Mask Register"]
pub struct HC10_INTMSK {
    register: VolatileCell<u32>,
}
#[doc = "Host Channel x Interrupt Mask Register"]
pub mod hc10_intmsk;
#[doc = "Host Channel x Transfer Size Register"]
pub struct HC10_TSIZ {
    register: VolatileCell<u32>,
}
#[doc = "Host Channel x Transfer Size Register"]
pub mod hc10_tsiz;
#[doc = "Host Channel x DMA Address Register"]
pub struct HC10_DMAADDR {
    register: VolatileCell<u32>,
}
#[doc = "Host Channel x DMA Address Register"]
pub mod hc10_dmaaddr;
#[doc = "Host Channel x Characteristics Register"]
pub struct HC11_CHAR {
    register: VolatileCell<u32>,
}
#[doc = "Host Channel x Characteristics Register"]
pub mod hc11_char;
#[doc = "Host Channel x Interrupt Register"]
pub struct HC11_INT {
    register: VolatileCell<u32>,
}
#[doc = "Host Channel x Interrupt Register"]
pub mod hc11_int;
#[doc = "Host Channel x Interrupt Mask Register"]
pub struct HC11_INTMSK {
    register: VolatileCell<u32>,
}
#[doc = "Host Channel x Interrupt Mask Register"]
pub mod hc11_intmsk;
#[doc = "Host Channel x Transfer Size Register"]
pub struct HC11_TSIZ {
    register: VolatileCell<u32>,
}
#[doc = "Host Channel x Transfer Size Register"]
pub mod hc11_tsiz;
#[doc = "Host Channel x DMA Address Register"]
pub struct HC11_DMAADDR {
    register: VolatileCell<u32>,
}
#[doc = "Host Channel x DMA Address Register"]
pub mod hc11_dmaaddr;
#[doc = "Host Channel x Characteristics Register"]
pub struct HC12_CHAR {
    register: VolatileCell<u32>,
}
#[doc = "Host Channel x Characteristics Register"]
pub mod hc12_char;
#[doc = "Host Channel x Interrupt Register"]
pub struct HC12_INT {
    register: VolatileCell<u32>,
}
#[doc = "Host Channel x Interrupt Register"]
pub mod hc12_int;
#[doc = "Host Channel x Interrupt Mask Register"]
pub struct HC12_INTMSK {
    register: VolatileCell<u32>,
}
#[doc = "Host Channel x Interrupt Mask Register"]
pub mod hc12_intmsk;
#[doc = "Host Channel x Transfer Size Register"]
pub struct HC12_TSIZ {
    register: VolatileCell<u32>,
}
#[doc = "Host Channel x Transfer Size Register"]
pub mod hc12_tsiz;
#[doc = "Host Channel x DMA Address Register"]
pub struct HC12_DMAADDR {
    register: VolatileCell<u32>,
}
#[doc = "Host Channel x DMA Address Register"]
pub mod hc12_dmaaddr;
#[doc = "Host Channel x Characteristics Register"]
pub struct HC13_CHAR {
    register: VolatileCell<u32>,
}
#[doc = "Host Channel x Characteristics Register"]
pub mod hc13_char;
#[doc = "Host Channel x Interrupt Register"]
pub struct HC13_INT {
    register: VolatileCell<u32>,
}
#[doc = "Host Channel x Interrupt Register"]
pub mod hc13_int;
#[doc = "Host Channel x Interrupt Mask Register"]
pub struct HC13_INTMSK {
    register: VolatileCell<u32>,
}
#[doc = "Host Channel x Interrupt Mask Register"]
pub mod hc13_intmsk;
#[doc = "Host Channel x Transfer Size Register"]
pub struct HC13_TSIZ {
    register: VolatileCell<u32>,
}
#[doc = "Host Channel x Transfer Size Register"]
pub mod hc13_tsiz;
#[doc = "Host Channel x DMA Address Register"]
pub struct HC13_DMAADDR {
    register: VolatileCell<u32>,
}
#[doc = "Host Channel x DMA Address Register"]
pub mod hc13_dmaaddr;
#[doc = "Device Configuration Register"]
pub struct DCFG {
    register: VolatileCell<u32>,
}
#[doc = "Device Configuration Register"]
pub mod dcfg;
#[doc = "Device Control Register"]
pub struct DCTL {
    register: VolatileCell<u32>,
}
#[doc = "Device Control Register"]
pub mod dctl;
#[doc = "Device Status Register"]
pub struct DSTS {
    register: VolatileCell<u32>,
}
#[doc = "Device Status Register"]
pub mod dsts;
#[doc = "Device IN Endpoint Common Interrupt Mask Register"]
pub struct DIEPMSK {
    register: VolatileCell<u32>,
}
#[doc = "Device IN Endpoint Common Interrupt Mask Register"]
pub mod diepmsk;
#[doc = "Device OUT Endpoint Common Interrupt Mask Register"]
pub struct DOEPMSK {
    register: VolatileCell<u32>,
}
#[doc = "Device OUT Endpoint Common Interrupt Mask Register"]
pub mod doepmsk;
#[doc = "Device All Endpoints Interrupt Register"]
pub struct DAINT {
    register: VolatileCell<u32>,
}
#[doc = "Device All Endpoints Interrupt Register"]
pub mod daint;
#[doc = "Device All Endpoints Interrupt Mask Register"]
pub struct DAINTMSK {
    register: VolatileCell<u32>,
}
#[doc = "Device All Endpoints Interrupt Mask Register"]
pub mod daintmsk;
#[doc = "Device VBUS Discharge Time Register"]
pub struct DVBUSDIS {
    register: VolatileCell<u32>,
}
#[doc = "Device VBUS Discharge Time Register"]
pub mod dvbusdis;
#[doc = "Device VBUS Pulsing Time Register"]
pub struct DVBUSPULSE {
    register: VolatileCell<u32>,
}
#[doc = "Device VBUS Pulsing Time Register"]
pub mod dvbuspulse;
#[doc = "Device IN Endpoint FIFO Empty Interrupt Mask Register"]
pub struct DIEPEMPMSK {
    register: VolatileCell<u32>,
}
#[doc = "Device IN Endpoint FIFO Empty Interrupt Mask Register"]
pub mod diepempmsk;
#[doc = "Device IN Endpoint 0 Control Register"]
pub struct DIEP0CTL {
    register: VolatileCell<u32>,
}
#[doc = "Device IN Endpoint 0 Control Register"]
pub mod diep0ctl;
#[doc = "Device IN Endpoint 0 Interrupt Register"]
pub struct DIEP0INT {
    register: VolatileCell<u32>,
}
#[doc = "Device IN Endpoint 0 Interrupt Register"]
pub mod diep0int;
#[doc = "Device IN Endpoint 0 Transfer Size Register"]
pub struct DIEP0TSIZ {
    register: VolatileCell<u32>,
}
#[doc = "Device IN Endpoint 0 Transfer Size Register"]
pub mod diep0tsiz;
#[doc = "Device IN Endpoint 0 DMA Address Register"]
pub struct DIEP0DMAADDR {
    register: VolatileCell<u32>,
}
#[doc = "Device IN Endpoint 0 DMA Address Register"]
pub mod diep0dmaaddr;
#[doc = "Device IN Endpoint 0 Transmit FIFO Status Register"]
pub struct DIEP0TXFSTS {
    register: VolatileCell<u32>,
}
#[doc = "Device IN Endpoint 0 Transmit FIFO Status Register"]
pub mod diep0txfsts;
#[doc = "Device IN Endpoint x+1 Control Register"]
pub struct DIEP0_CTL {
    register: VolatileCell<u32>,
}
#[doc = "Device IN Endpoint x+1 Control Register"]
pub mod diep0_ctl;
#[doc = "Device IN Endpoint x+1 Interrupt Register"]
pub struct DIEP0_INT {
    register: VolatileCell<u32>,
}
#[doc = "Device IN Endpoint x+1 Interrupt Register"]
pub mod diep0_int;
#[doc = "Device IN Endpoint x+1 Transfer Size Register"]
pub struct DIEP0_TSIZ {
    register: VolatileCell<u32>,
}
#[doc = "Device IN Endpoint x+1 Transfer Size Register"]
pub mod diep0_tsiz;
#[doc = "Device IN Endpoint x+1 DMA Address Register"]
pub struct DIEP0_DMAADDR {
    register: VolatileCell<u32>,
}
#[doc = "Device IN Endpoint x+1 DMA Address Register"]
pub mod diep0_dmaaddr;
#[doc = "Device IN Endpoint x+1 Transmit FIFO Status Register"]
pub struct DIEP0_TXFSTS {
    register: VolatileCell<u32>,
}
#[doc = "Device IN Endpoint x+1 Transmit FIFO Status Register"]
pub mod diep0_txfsts;
#[doc = "Device IN Endpoint x+1 Control Register"]
pub struct DIEP1_CTL {
    register: VolatileCell<u32>,
}
#[doc = "Device IN Endpoint x+1 Control Register"]
pub mod diep1_ctl;
#[doc = "Device IN Endpoint x+1 Interrupt Register"]
pub struct DIEP1_INT {
    register: VolatileCell<u32>,
}
#[doc = "Device IN Endpoint x+1 Interrupt Register"]
pub mod diep1_int;
#[doc = "Device IN Endpoint x+1 Transfer Size Register"]
pub struct DIEP1_TSIZ {
    register: VolatileCell<u32>,
}
#[doc = "Device IN Endpoint x+1 Transfer Size Register"]
pub mod diep1_tsiz;
#[doc = "Device IN Endpoint x+1 DMA Address Register"]
pub struct DIEP1_DMAADDR {
    register: VolatileCell<u32>,
}
#[doc = "Device IN Endpoint x+1 DMA Address Register"]
pub mod diep1_dmaaddr;
#[doc = "Device IN Endpoint x+1 Transmit FIFO Status Register"]
pub struct DIEP1_TXFSTS {
    register: VolatileCell<u32>,
}
#[doc = "Device IN Endpoint x+1 Transmit FIFO Status Register"]
pub mod diep1_txfsts;
#[doc = "Device IN Endpoint x+1 Control Register"]
pub struct DIEP2_CTL {
    register: VolatileCell<u32>,
}
#[doc = "Device IN Endpoint x+1 Control Register"]
pub mod diep2_ctl;
#[doc = "Device IN Endpoint x+1 Interrupt Register"]
pub struct DIEP2_INT {
    register: VolatileCell<u32>,
}
#[doc = "Device IN Endpoint x+1 Interrupt Register"]
pub mod diep2_int;
#[doc = "Device IN Endpoint x+1 Transfer Size Register"]
pub struct DIEP2_TSIZ {
    register: VolatileCell<u32>,
}
#[doc = "Device IN Endpoint x+1 Transfer Size Register"]
pub mod diep2_tsiz;
#[doc = "Device IN Endpoint x+1 DMA Address Register"]
pub struct DIEP2_DMAADDR {
    register: VolatileCell<u32>,
}
#[doc = "Device IN Endpoint x+1 DMA Address Register"]
pub mod diep2_dmaaddr;
#[doc = "Device IN Endpoint x+1 Transmit FIFO Status Register"]
pub struct DIEP2_TXFSTS {
    register: VolatileCell<u32>,
}
#[doc = "Device IN Endpoint x+1 Transmit FIFO Status Register"]
pub mod diep2_txfsts;
#[doc = "Device IN Endpoint x+1 Control Register"]
pub struct DIEP3_CTL {
    register: VolatileCell<u32>,
}
#[doc = "Device IN Endpoint x+1 Control Register"]
pub mod diep3_ctl;
#[doc = "Device IN Endpoint x+1 Interrupt Register"]
pub struct DIEP3_INT {
    register: VolatileCell<u32>,
}
#[doc = "Device IN Endpoint x+1 Interrupt Register"]
pub mod diep3_int;
#[doc = "Device IN Endpoint x+1 Transfer Size Register"]
pub struct DIEP3_TSIZ {
    register: VolatileCell<u32>,
}
#[doc = "Device IN Endpoint x+1 Transfer Size Register"]
pub mod diep3_tsiz;
#[doc = "Device IN Endpoint x+1 DMA Address Register"]
pub struct DIEP3_DMAADDR {
    register: VolatileCell<u32>,
}
#[doc = "Device IN Endpoint x+1 DMA Address Register"]
pub mod diep3_dmaaddr;
#[doc = "Device IN Endpoint x+1 Transmit FIFO Status Register"]
pub struct DIEP3_TXFSTS {
    register: VolatileCell<u32>,
}
#[doc = "Device IN Endpoint x+1 Transmit FIFO Status Register"]
pub mod diep3_txfsts;
#[doc = "Device IN Endpoint x+1 Control Register"]
pub struct DIEP4_CTL {
    register: VolatileCell<u32>,
}
#[doc = "Device IN Endpoint x+1 Control Register"]
pub mod diep4_ctl;
#[doc = "Device IN Endpoint x+1 Interrupt Register"]
pub struct DIEP4_INT {
    register: VolatileCell<u32>,
}
#[doc = "Device IN Endpoint x+1 Interrupt Register"]
pub mod diep4_int;
#[doc = "Device IN Endpoint x+1 Transfer Size Register"]
pub struct DIEP4_TSIZ {
    register: VolatileCell<u32>,
}
#[doc = "Device IN Endpoint x+1 Transfer Size Register"]
pub mod diep4_tsiz;
#[doc = "Device IN Endpoint x+1 DMA Address Register"]
pub struct DIEP4_DMAADDR {
    register: VolatileCell<u32>,
}
#[doc = "Device IN Endpoint x+1 DMA Address Register"]
pub mod diep4_dmaaddr;
#[doc = "Device IN Endpoint x+1 Transmit FIFO Status Register"]
pub struct DIEP4_TXFSTS {
    register: VolatileCell<u32>,
}
#[doc = "Device IN Endpoint x+1 Transmit FIFO Status Register"]
pub mod diep4_txfsts;
#[doc = "Device IN Endpoint x+1 Control Register"]
pub struct DIEP5_CTL {
    register: VolatileCell<u32>,
}
#[doc = "Device IN Endpoint x+1 Control Register"]
pub mod diep5_ctl;
#[doc = "Device IN Endpoint x+1 Interrupt Register"]
pub struct DIEP5_INT {
    register: VolatileCell<u32>,
}
#[doc = "Device IN Endpoint x+1 Interrupt Register"]
pub mod diep5_int;
#[doc = "Device IN Endpoint x+1 Transfer Size Register"]
pub struct DIEP5_TSIZ {
    register: VolatileCell<u32>,
}
#[doc = "Device IN Endpoint x+1 Transfer Size Register"]
pub mod diep5_tsiz;
#[doc = "Device IN Endpoint x+1 DMA Address Register"]
pub struct DIEP5_DMAADDR {
    register: VolatileCell<u32>,
}
#[doc = "Device IN Endpoint x+1 DMA Address Register"]
pub mod diep5_dmaaddr;
#[doc = "Device IN Endpoint x+1 Transmit FIFO Status Register"]
pub struct DIEP5_TXFSTS {
    register: VolatileCell<u32>,
}
#[doc = "Device IN Endpoint x+1 Transmit FIFO Status Register"]
pub mod diep5_txfsts;
#[doc = "Device OUT Endpoint 0 Control Register"]
pub struct DOEP0CTL {
    register: VolatileCell<u32>,
}
#[doc = "Device OUT Endpoint 0 Control Register"]
pub mod doep0ctl;
#[doc = "Device OUT Endpoint 0 Interrupt Register"]
pub struct DOEP0INT {
    register: VolatileCell<u32>,
}
#[doc = "Device OUT Endpoint 0 Interrupt Register"]
pub mod doep0int;
#[doc = "Device OUT Endpoint 0 Transfer Size Register"]
pub struct DOEP0TSIZ {
    register: VolatileCell<u32>,
}
#[doc = "Device OUT Endpoint 0 Transfer Size Register"]
pub mod doep0tsiz;
#[doc = "Device OUT Endpoint 0 DMA Address Register"]
pub struct DOEP0DMAADDR {
    register: VolatileCell<u32>,
}
#[doc = "Device OUT Endpoint 0 DMA Address Register"]
pub mod doep0dmaaddr;
#[doc = "Device OUT Endpoint x+1 Control Register"]
pub struct DOEP0_CTL {
    register: VolatileCell<u32>,
}
#[doc = "Device OUT Endpoint x+1 Control Register"]
pub mod doep0_ctl;
#[doc = "Device OUT Endpoint x+1 Interrupt Register"]
pub struct DOEP0_INT {
    register: VolatileCell<u32>,
}
#[doc = "Device OUT Endpoint x+1 Interrupt Register"]
pub mod doep0_int;
#[doc = "Device OUT Endpoint x+1 Transfer Size Register"]
pub struct DOEP0_TSIZ {
    register: VolatileCell<u32>,
}
#[doc = "Device OUT Endpoint x+1 Transfer Size Register"]
pub mod doep0_tsiz;
#[doc = "Device OUT Endpoint x+1 DMA Address Register"]
pub struct DOEP0_DMAADDR {
    register: VolatileCell<u32>,
}
#[doc = "Device OUT Endpoint x+1 DMA Address Register"]
pub mod doep0_dmaaddr;
#[doc = "Device OUT Endpoint x+1 Control Register"]
pub struct DOEP1_CTL {
    register: VolatileCell<u32>,
}
#[doc = "Device OUT Endpoint x+1 Control Register"]
pub mod doep1_ctl;
#[doc = "Device OUT Endpoint x+1 Interrupt Register"]
pub struct DOEP1_INT {
    register: VolatileCell<u32>,
}
#[doc = "Device OUT Endpoint x+1 Interrupt Register"]
pub mod doep1_int;
#[doc = "Device OUT Endpoint x+1 Transfer Size Register"]
pub struct DOEP1_TSIZ {
    register: VolatileCell<u32>,
}
#[doc = "Device OUT Endpoint x+1 Transfer Size Register"]
pub mod doep1_tsiz;
#[doc = "Device OUT Endpoint x+1 DMA Address Register"]
pub struct DOEP1_DMAADDR {
    register: VolatileCell<u32>,
}
#[doc = "Device OUT Endpoint x+1 DMA Address Register"]
pub mod doep1_dmaaddr;
#[doc = "Device OUT Endpoint x+1 Control Register"]
pub struct DOEP2_CTL {
    register: VolatileCell<u32>,
}
#[doc = "Device OUT Endpoint x+1 Control Register"]
pub mod doep2_ctl;
#[doc = "Device OUT Endpoint x+1 Interrupt Register"]
pub struct DOEP2_INT {
    register: VolatileCell<u32>,
}
#[doc = "Device OUT Endpoint x+1 Interrupt Register"]
pub mod doep2_int;
#[doc = "Device OUT Endpoint x+1 Transfer Size Register"]
pub struct DOEP2_TSIZ {
    register: VolatileCell<u32>,
}
#[doc = "Device OUT Endpoint x+1 Transfer Size Register"]
pub mod doep2_tsiz;
#[doc = "Device OUT Endpoint x+1 DMA Address Register"]
pub struct DOEP2_DMAADDR {
    register: VolatileCell<u32>,
}
#[doc = "Device OUT Endpoint x+1 DMA Address Register"]
pub mod doep2_dmaaddr;
#[doc = "Device OUT Endpoint x+1 Control Register"]
pub struct DOEP3_CTL {
    register: VolatileCell<u32>,
}
#[doc = "Device OUT Endpoint x+1 Control Register"]
pub mod doep3_ctl;
#[doc = "Device OUT Endpoint x+1 Interrupt Register"]
pub struct DOEP3_INT {
    register: VolatileCell<u32>,
}
#[doc = "Device OUT Endpoint x+1 Interrupt Register"]
pub mod doep3_int;
#[doc = "Device OUT Endpoint x+1 Transfer Size Register"]
pub struct DOEP3_TSIZ {
    register: VolatileCell<u32>,
}
#[doc = "Device OUT Endpoint x+1 Transfer Size Register"]
pub mod doep3_tsiz;
#[doc = "Device OUT Endpoint x+1 DMA Address Register"]
pub struct DOEP3_DMAADDR {
    register: VolatileCell<u32>,
}
#[doc = "Device OUT Endpoint x+1 DMA Address Register"]
pub mod doep3_dmaaddr;
#[doc = "Device OUT Endpoint x+1 Control Register"]
pub struct DOEP4_CTL {
    register: VolatileCell<u32>,
}
#[doc = "Device OUT Endpoint x+1 Control Register"]
pub mod doep4_ctl;
#[doc = "Device OUT Endpoint x+1 Interrupt Register"]
pub struct DOEP4_INT {
    register: VolatileCell<u32>,
}
#[doc = "Device OUT Endpoint x+1 Interrupt Register"]
pub mod doep4_int;
#[doc = "Device OUT Endpoint x+1 Transfer Size Register"]
pub struct DOEP4_TSIZ {
    register: VolatileCell<u32>,
}
#[doc = "Device OUT Endpoint x+1 Transfer Size Register"]
pub mod doep4_tsiz;
#[doc = "Device OUT Endpoint x+1 DMA Address Register"]
pub struct DOEP4_DMAADDR {
    register: VolatileCell<u32>,
}
#[doc = "Device OUT Endpoint x+1 DMA Address Register"]
pub mod doep4_dmaaddr;
#[doc = "Device OUT Endpoint x+1 Control Register"]
pub struct DOEP5_CTL {
    register: VolatileCell<u32>,
}
#[doc = "Device OUT Endpoint x+1 Control Register"]
pub mod doep5_ctl;
#[doc = "Device OUT Endpoint x+1 Interrupt Register"]
pub struct DOEP5_INT {
    register: VolatileCell<u32>,
}
#[doc = "Device OUT Endpoint x+1 Interrupt Register"]
pub mod doep5_int;
#[doc = "Device OUT Endpoint x+1 Transfer Size Register"]
pub struct DOEP5_TSIZ {
    register: VolatileCell<u32>,
}
#[doc = "Device OUT Endpoint x+1 Transfer Size Register"]
pub mod doep5_tsiz;
#[doc = "Device OUT Endpoint x+1 DMA Address Register"]
pub struct DOEP5_DMAADDR {
    register: VolatileCell<u32>,
}
#[doc = "Device OUT Endpoint x+1 DMA Address Register"]
pub mod doep5_dmaaddr;
#[doc = "Power and Clock Gating Control Register"]
pub struct PCGCCTL {
    register: VolatileCell<u32>,
}
#[doc = "Power and Clock Gating Control Register"]
pub mod pcgcctl;

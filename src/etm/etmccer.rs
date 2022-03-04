#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
impl super::ETMCCER {
    #[doc = r" Reads the contents of the register"]
    #[inline]
    pub fn read(&self) -> R {
        R {
            bits: self.register.get(),
        }
    }
}
#[doc = r" Value of the field"]
pub struct EXTINPSELR {
    bits: u8,
}
impl EXTINPSELR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct EXTINPBUSR {
    bits: u8,
}
impl EXTINPBUSR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct READREGSR {
    bits: bool,
}
impl READREGSR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bit(&self) -> bool {
        self.bits
    }
    #[doc = r" Returns `true` if the bit is clear (0)"]
    #[inline]
    pub fn bit_is_clear(&self) -> bool {
        !self.bit()
    }
    #[doc = r" Returns `true` if the bit is set (1)"]
    #[inline]
    pub fn bit_is_set(&self) -> bool {
        self.bit()
    }
}
#[doc = r" Value of the field"]
pub struct DADDRCMPR {
    bits: bool,
}
impl DADDRCMPR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bit(&self) -> bool {
        self.bits
    }
    #[doc = r" Returns `true` if the bit is clear (0)"]
    #[inline]
    pub fn bit_is_clear(&self) -> bool {
        !self.bit()
    }
    #[doc = r" Returns `true` if the bit is set (1)"]
    #[inline]
    pub fn bit_is_set(&self) -> bool {
        self.bit()
    }
}
#[doc = r" Value of the field"]
pub struct INSTRESR {
    bits: u8,
}
impl INSTRESR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct EICEWPNTR {
    bits: u8,
}
impl EICEWPNTR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct TEICEWPNTR {
    bits: bool,
}
impl TEICEWPNTR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bit(&self) -> bool {
        self.bits
    }
    #[doc = r" Returns `true` if the bit is clear (0)"]
    #[inline]
    pub fn bit_is_clear(&self) -> bool {
        !self.bit()
    }
    #[doc = r" Returns `true` if the bit is set (1)"]
    #[inline]
    pub fn bit_is_set(&self) -> bool {
        self.bit()
    }
}
#[doc = r" Value of the field"]
pub struct EICEIMPR {
    bits: bool,
}
impl EICEIMPR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bit(&self) -> bool {
        self.bits
    }
    #[doc = r" Returns `true` if the bit is clear (0)"]
    #[inline]
    pub fn bit_is_clear(&self) -> bool {
        !self.bit()
    }
    #[doc = r" Returns `true` if the bit is set (1)"]
    #[inline]
    pub fn bit_is_set(&self) -> bool {
        self.bit()
    }
}
#[doc = r" Value of the field"]
pub struct TIMPR {
    bits: bool,
}
impl TIMPR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bit(&self) -> bool {
        self.bits
    }
    #[doc = r" Returns `true` if the bit is clear (0)"]
    #[inline]
    pub fn bit_is_clear(&self) -> bool {
        !self.bit()
    }
    #[doc = r" Returns `true` if the bit is set (1)"]
    #[inline]
    pub fn bit_is_set(&self) -> bool {
        self.bit()
    }
}
#[doc = r" Value of the field"]
pub struct RFCNTR {
    bits: bool,
}
impl RFCNTR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bit(&self) -> bool {
        self.bits
    }
    #[doc = r" Returns `true` if the bit is clear (0)"]
    #[inline]
    pub fn bit_is_clear(&self) -> bool {
        !self.bit()
    }
    #[doc = r" Returns `true` if the bit is set (1)"]
    #[inline]
    pub fn bit_is_set(&self) -> bool {
        self.bit()
    }
}
#[doc = r" Value of the field"]
pub struct TENCR {
    bits: bool,
}
impl TENCR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bit(&self) -> bool {
        self.bits
    }
    #[doc = r" Returns `true` if the bit is clear (0)"]
    #[inline]
    pub fn bit_is_clear(&self) -> bool {
        !self.bit()
    }
    #[doc = r" Returns `true` if the bit is set (1)"]
    #[inline]
    pub fn bit_is_set(&self) -> bool {
        self.bit()
    }
}
#[doc = r" Value of the field"]
pub struct TSIZER {
    bits: bool,
}
impl TSIZER {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bit(&self) -> bool {
        self.bits
    }
    #[doc = r" Returns `true` if the bit is clear (0)"]
    #[inline]
    pub fn bit_is_clear(&self) -> bool {
        !self.bit()
    }
    #[doc = r" Returns `true` if the bit is set (1)"]
    #[inline]
    pub fn bit_is_set(&self) -> bool {
        self.bit()
    }
}
impl R {
    #[doc = r" Value of the register as raw bits"]
    #[inline]
    pub fn bits(&self) -> u32 {
        self.bits
    }
    #[doc = "Bits 0:1 - Extended External Input Selectors"]
    #[inline]
    pub fn extinpsel(&self) -> EXTINPSELR {
        let bits = {
            const MASK: u8 = 0x03;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        EXTINPSELR { bits }
    }
    #[doc = "Bits 3:10 - Extended External Input Bus"]
    #[inline]
    pub fn extinpbus(&self) -> EXTINPBUSR {
        let bits = {
            const MASK: u8 = 0xff;
            const OFFSET: u8 = 3;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        EXTINPBUSR { bits }
    }
    #[doc = "Bit 11 - Readable Registers"]
    #[inline]
    pub fn readregs(&self) -> READREGSR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 11;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        READREGSR { bits }
    }
    #[doc = "Bit 12 - Data Address comparisons"]
    #[inline]
    pub fn daddrcmp(&self) -> DADDRCMPR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 12;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        DADDRCMPR { bits }
    }
    #[doc = "Bits 13:15 - Instrumentation Resources"]
    #[inline]
    pub fn instres(&self) -> INSTRESR {
        let bits = {
            const MASK: u8 = 0x07;
            const OFFSET: u8 = 13;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        INSTRESR { bits }
    }
    #[doc = "Bits 16:19 - EmbeddedICE watchpoint inputs"]
    #[inline]
    pub fn eicewpnt(&self) -> EICEWPNTR {
        let bits = {
            const MASK: u8 = 0x0f;
            const OFFSET: u8 = 16;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        EICEWPNTR { bits }
    }
    #[doc = "Bit 20 - Trace Sart/Stop Block Uses EmbeddedICE watchpoint inputs"]
    #[inline]
    pub fn teicewpnt(&self) -> TEICEWPNTR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 20;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        TEICEWPNTR { bits }
    }
    #[doc = "Bit 21 - EmbeddedICE Behavior control Implemented"]
    #[inline]
    pub fn eiceimp(&self) -> EICEIMPR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 21;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        EICEIMPR { bits }
    }
    #[doc = "Bit 22 - Timestamping Implemented"]
    #[inline]
    pub fn timp(&self) -> TIMPR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 22;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        TIMPR { bits }
    }
    #[doc = "Bit 27 - Reduced Function Counter"]
    #[inline]
    pub fn rfcnt(&self) -> RFCNTR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 27;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        RFCNTR { bits }
    }
    #[doc = "Bit 28 - Timestamp Encoding"]
    #[inline]
    pub fn tenc(&self) -> TENCR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 28;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        TENCR { bits }
    }
    #[doc = "Bit 29 - Timestamp Size"]
    #[inline]
    pub fn tsize(&self) -> TSIZER {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 29;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        TSIZER { bits }
    }
}

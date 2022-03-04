#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
impl super::ETMIDR {
    #[doc = r" Reads the contents of the register"]
    #[inline]
    pub fn read(&self) -> R {
        R {
            bits: self.register.get(),
        }
    }
}
#[doc = r" Value of the field"]
pub struct IMPVERR {
    bits: u8,
}
impl IMPVERR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct ETMMINVERR {
    bits: u8,
}
impl ETMMINVERR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct ETMMAJVERR {
    bits: u8,
}
impl ETMMAJVERR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct PROCFAMR {
    bits: u8,
}
impl PROCFAMR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct LPCFR {
    bits: bool,
}
impl LPCFR {
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
pub struct THUMBTR {
    bits: bool,
}
impl THUMBTR {
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
pub struct SECEXTR {
    bits: bool,
}
impl SECEXTR {
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
pub struct BPER {
    bits: bool,
}
impl BPER {
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
pub struct IMPCODER {
    bits: u8,
}
impl IMPCODER {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
impl R {
    #[doc = r" Value of the register as raw bits"]
    #[inline]
    pub fn bits(&self) -> u32 {
        self.bits
    }
    #[doc = "Bits 0:3 - Implementation Revision"]
    #[inline]
    pub fn impver(&self) -> IMPVERR {
        let bits = {
            const MASK: u8 = 0x0f;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        IMPVERR { bits }
    }
    #[doc = "Bits 4:7 - Minor ETM Architecture Version"]
    #[inline]
    pub fn etmminver(&self) -> ETMMINVERR {
        let bits = {
            const MASK: u8 = 0x0f;
            const OFFSET: u8 = 4;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        ETMMINVERR { bits }
    }
    #[doc = "Bits 8:11 - Major ETM Architecture Version"]
    #[inline]
    pub fn etmmajver(&self) -> ETMMAJVERR {
        let bits = {
            const MASK: u8 = 0x0f;
            const OFFSET: u8 = 8;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        ETMMAJVERR { bits }
    }
    #[doc = "Bits 12:15 - Implementer Code"]
    #[inline]
    pub fn procfam(&self) -> PROCFAMR {
        let bits = {
            const MASK: u8 = 0x0f;
            const OFFSET: u8 = 12;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        PROCFAMR { bits }
    }
    #[doc = "Bit 16 - Load PC First"]
    #[inline]
    pub fn lpcf(&self) -> LPCFR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 16;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        LPCFR { bits }
    }
    #[doc = "Bit 18 - 32-bit Thumb Instruction Tracing"]
    #[inline]
    pub fn thumbt(&self) -> THUMBTR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 18;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        THUMBTR { bits }
    }
    #[doc = "Bit 19 - Security Extension Support"]
    #[inline]
    pub fn secext(&self) -> SECEXTR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 19;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        SECEXTR { bits }
    }
    #[doc = "Bit 20 - Branch Packet Encoding"]
    #[inline]
    pub fn bpe(&self) -> BPER {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 20;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        BPER { bits }
    }
    #[doc = "Bits 24:31 - Implementer Code"]
    #[inline]
    pub fn impcode(&self) -> IMPCODER {
        let bits = {
            const MASK: u8 = 0xff;
            const OFFSET: u8 = 24;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        IMPCODER { bits }
    }
}

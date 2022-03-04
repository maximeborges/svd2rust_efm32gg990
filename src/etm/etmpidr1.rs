#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
impl super::ETMPIDR1 {
    #[doc = r" Reads the contents of the register"]
    #[inline]
    pub fn read(&self) -> R {
        R {
            bits: self.register.get(),
        }
    }
}
#[doc = r" Value of the field"]
pub struct PARTNUMR {
    bits: u8,
}
impl PARTNUMR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct IDCODER {
    bits: u8,
}
impl IDCODER {
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
    #[doc = "Bits 0:3 - Part Number"]
    #[inline]
    pub fn partnum(&self) -> PARTNUMR {
        let bits = {
            const MASK: u8 = 0x0f;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        PARTNUMR { bits }
    }
    #[doc = "Bits 4:7 - JEP106 Identity Code"]
    #[inline]
    pub fn idcode(&self) -> IDCODER {
        let bits = {
            const MASK: u8 = 0x0f;
            const OFFSET: u8 = 4;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        IDCODER { bits }
    }
}

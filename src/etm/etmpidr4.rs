#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
impl super::ETMPIDR4 {
    #[doc = r" Reads the contents of the register"]
    #[inline]
    pub fn read(&self) -> R {
        R {
            bits: self.register.get(),
        }
    }
}
#[doc = r" Value of the field"]
pub struct CONTCODER {
    bits: u8,
}
impl CONTCODER {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct COUNTR {
    bits: u8,
}
impl COUNTR {
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
    #[doc = "Bits 0:3 - JEP106 Continuation Code"]
    #[inline]
    pub fn contcode(&self) -> CONTCODER {
        let bits = {
            const MASK: u8 = 0x0f;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        CONTCODER { bits }
    }
    #[doc = "Bits 4:7 - 4KB Count"]
    #[inline]
    pub fn count(&self) -> COUNTR {
        let bits = {
            const MASK: u8 = 0x0f;
            const OFFSET: u8 = 4;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        COUNTR { bits }
    }
}

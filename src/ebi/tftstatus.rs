#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
impl super::TFTSTATUS {
    #[doc = r" Reads the contents of the register"]
    #[inline]
    pub fn read(&self) -> R {
        R {
            bits: self.register.get(),
        }
    }
}
#[doc = r" Value of the field"]
pub struct HCNTR {
    bits: u16,
}
impl HCNTR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u16 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct VCNTR {
    bits: u16,
}
impl VCNTR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u16 {
        self.bits
    }
}
impl R {
    #[doc = r" Value of the register as raw bits"]
    #[inline]
    pub fn bits(&self) -> u32 {
        self.bits
    }
    #[doc = "Bits 0:10 - Horizontal Count"]
    #[inline]
    pub fn hcnt(&self) -> HCNTR {
        let bits = {
            const MASK: u16 = 0x07ff;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) as u16
        };
        HCNTR { bits }
    }
    #[doc = "Bits 16:26 - Vertical Count"]
    #[inline]
    pub fn vcnt(&self) -> VCNTR {
        let bits = {
            const MASK: u16 = 0x07ff;
            const OFFSET: u8 = 16;
            ((self.bits >> OFFSET) & MASK as u32) as u16
        };
        VCNTR { bits }
    }
}

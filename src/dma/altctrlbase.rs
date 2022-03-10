#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
impl super::ALTCTRLBASE {
    #[doc = r" Reads the contents of the register"]
    #[inline]
    pub fn read(&self) -> R {
        R {
            bits: self.register.get(),
        }
    }
}
#[doc = r" Value of the field"]
pub struct ALTCTRLBASER {
    bits: u32,
}
impl ALTCTRLBASER {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u32 {
        self.bits
    }
}
impl R {
    #[doc = r" Value of the register as raw bits"]
    #[inline]
    pub fn bits(&self) -> u32 {
        self.bits
    }
    #[doc = "Bits 0:31 - Channel Alternate Control Data Base Pointer"]
    #[inline]
    pub fn altctrlbase(&self) -> ALTCTRLBASER {
        let bits = {
            const MASK: u32 = 0xffff_ffff;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) as u32
        };
        ALTCTRLBASER { bits }
    }
}

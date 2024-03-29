#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
impl super::HAINT {
    #[doc = r" Reads the contents of the register"]
    #[inline]
    pub fn read(&self) -> R {
        R {
            bits: self.register.get(),
        }
    }
}
#[doc = r" Value of the field"]
pub struct HAINTR {
    bits: u16,
}
impl HAINTR {
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
    #[doc = "Bits 0:13 - Channel Interrupt for channel 0 - 13."]
    #[inline]
    pub fn haint(&self) -> HAINTR {
        let bits = {
            const MASK: u16 = 0x3fff;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) as u16
        };
        HAINTR { bits }
    }
}

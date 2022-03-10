#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
impl super::HFNUM {
    #[doc = r" Reads the contents of the register"]
    #[inline]
    pub fn read(&self) -> R {
        R {
            bits: self.register.get(),
        }
    }
}
#[doc = r" Value of the field"]
pub struct FRNUMR {
    bits: u16,
}
impl FRNUMR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u16 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct FRREMR {
    bits: u16,
}
impl FRREMR {
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
    #[doc = "Bits 0:15 - Frame Number"]
    #[inline]
    pub fn frnum(&self) -> FRNUMR {
        let bits = {
            const MASK: u16 = 0xffff;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) as u16
        };
        FRNUMR { bits }
    }
    #[doc = "Bits 16:31 - Frame Time Remaining"]
    #[inline]
    pub fn frrem(&self) -> FRREMR {
        let bits = {
            const MASK: u16 = 0xffff;
            const OFFSET: u8 = 16;
            ((self.bits >> OFFSET) & MASK as u32) as u16
        };
        FRREMR { bits }
    }
}

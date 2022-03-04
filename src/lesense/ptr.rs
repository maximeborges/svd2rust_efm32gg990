#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
impl super::PTR {
    #[doc = r" Reads the contents of the register"]
    #[inline]
    pub fn read(&self) -> R {
        R {
            bits: self.register.get(),
        }
    }
}
#[doc = r" Value of the field"]
pub struct RDR {
    bits: u8,
}
impl RDR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct WRR {
    bits: u8,
}
impl WRR {
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
    #[doc = "Bits 0:3 - Result buffer read pointer."]
    #[inline]
    pub fn rd(&self) -> RDR {
        let bits = {
            const MASK: u8 = 0x0f;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        RDR { bits }
    }
    #[doc = "Bits 5:8 - Result buffer write pointer."]
    #[inline]
    pub fn wr(&self) -> WRR {
        let bits = {
            const MASK: u8 = 0x0f;
            const OFFSET: u8 = 5;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        WRR { bits }
    }
}

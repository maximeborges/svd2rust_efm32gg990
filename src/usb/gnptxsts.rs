#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
impl super::GNPTXSTS {
    #[doc = r" Reads the contents of the register"]
    #[inline]
    pub fn read(&self) -> R {
        R {
            bits: self.register.get(),
        }
    }
}
#[doc = r" Value of the field"]
pub struct NPTXFSPCAVAILR {
    bits: u16,
}
impl NPTXFSPCAVAILR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u16 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct NPTXQSPCAVAILR {
    bits: u8,
}
impl NPTXQSPCAVAILR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct NPTXQTOPR {
    bits: u8,
}
impl NPTXQTOPR {
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
    #[doc = "Bits 0:15 - Non-periodic TxFIFO Space Available"]
    #[inline]
    pub fn nptxfspcavail(&self) -> NPTXFSPCAVAILR {
        let bits = {
            const MASK: u16 = 0xffff;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) as u16
        };
        NPTXFSPCAVAILR { bits }
    }
    #[doc = "Bits 16:23 - Non-periodic Transmit Request Queue Space Available"]
    #[inline]
    pub fn nptxqspcavail(&self) -> NPTXQSPCAVAILR {
        let bits = {
            const MASK: u8 = 0xff;
            const OFFSET: u8 = 16;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        NPTXQSPCAVAILR { bits }
    }
    #[doc = "Bits 24:30 - Top of the Non-periodic Transmit Request Queue"]
    #[inline]
    pub fn nptxqtop(&self) -> NPTXQTOPR {
        let bits = {
            const MASK: u8 = 0x7f;
            const OFFSET: u8 = 24;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        NPTXQTOPR { bits }
    }
}

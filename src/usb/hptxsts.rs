#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
impl super::HPTXSTS {
    #[doc = r" Reads the contents of the register"]
    #[inline]
    pub fn read(&self) -> R {
        R {
            bits: self.register.get(),
        }
    }
}
#[doc = r" Value of the field"]
pub struct PTXFSPCAVAILR {
    bits: u16,
}
impl PTXFSPCAVAILR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u16 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct PTXQSPCAVAILR {
    bits: u8,
}
impl PTXQSPCAVAILR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct PTXQTOPR {
    bits: u8,
}
impl PTXQTOPR {
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
    #[doc = "Bits 0:15 - Periodic Transmit Data FIFO Space Available"]
    #[inline]
    pub fn ptxfspcavail(&self) -> PTXFSPCAVAILR {
        let bits = {
            const MASK: u16 = 0xffff;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) as u16
        };
        PTXFSPCAVAILR { bits }
    }
    #[doc = "Bits 16:23 - Periodic Transmit Request Queue Space Available"]
    #[inline]
    pub fn ptxqspcavail(&self) -> PTXQSPCAVAILR {
        let bits = {
            const MASK: u8 = 0xff;
            const OFFSET: u8 = 16;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        PTXQSPCAVAILR { bits }
    }
    #[doc = "Bits 24:31 - Top of the Periodic Transmit Request Queue"]
    #[inline]
    pub fn ptxqtop(&self) -> PTXQTOPR {
        let bits = {
            const MASK: u8 = 0xff;
            const OFFSET: u8 = 24;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        PTXQTOPR { bits }
    }
}

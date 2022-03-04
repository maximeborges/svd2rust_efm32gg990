#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
impl super::ETMDEVTYPE {
    #[doc = r" Reads the contents of the register"]
    #[inline]
    pub fn read(&self) -> R {
        R {
            bits: self.register.get(),
        }
    }
}
#[doc = r" Value of the field"]
pub struct TRACESRCR {
    bits: u8,
}
impl TRACESRCR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct PROCTRACER {
    bits: u8,
}
impl PROCTRACER {
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
    #[doc = "Bits 0:3 - Trace Source"]
    #[inline]
    pub fn tracesrc(&self) -> TRACESRCR {
        let bits = {
            const MASK: u8 = 0x0f;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        TRACESRCR { bits }
    }
    #[doc = "Bits 4:7 - Processor Trace"]
    #[inline]
    pub fn proctrace(&self) -> PROCTRACER {
        let bits = {
            const MASK: u8 = 0x0f;
            const OFFSET: u8 = 4;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        PROCTRACER { bits }
    }
}

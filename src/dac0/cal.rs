#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::CAL {
    #[doc = r" Modifies the contents of the register"]
    #[inline]
    pub fn modify<F>(&self, f: F)
    where
        for<'w> F: FnOnce(&R, &'w mut W) -> &'w mut W,
    {
        let bits = self.register.get();
        let r = R { bits: bits };
        let mut w = W { bits: bits };
        f(&r, &mut w);
        self.register.set(w.bits);
    }
    #[doc = r" Reads the contents of the register"]
    #[inline]
    pub fn read(&self) -> R {
        R {
            bits: self.register.get(),
        }
    }
    #[doc = r" Writes to the register"]
    #[inline]
    pub fn write<F>(&self, f: F)
    where
        F: FnOnce(&mut W) -> &mut W,
    {
        let mut w = W::reset_value();
        f(&mut w);
        self.register.set(w.bits);
    }
    #[doc = r" Writes the reset value to the register"]
    #[inline]
    pub fn reset(&self) {
        self.write(|w| w)
    }
}
#[doc = r" Value of the field"]
pub struct CH0OFFSETR {
    bits: u8,
}
impl CH0OFFSETR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct CH1OFFSETR {
    bits: u8,
}
impl CH1OFFSETR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct GAINR {
    bits: u8,
}
impl GAINR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Proxy"]
pub struct _CH0OFFSETW<'a> {
    w: &'a mut W,
}
impl<'a> _CH0OFFSETW<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 0x3f;
        const OFFSET: u8 = 0;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _CH1OFFSETW<'a> {
    w: &'a mut W,
}
impl<'a> _CH1OFFSETW<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 0x3f;
        const OFFSET: u8 = 8;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _GAINW<'a> {
    w: &'a mut W,
}
impl<'a> _GAINW<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 0x7f;
        const OFFSET: u8 = 16;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
impl R {
    #[doc = r" Value of the register as raw bits"]
    #[inline]
    pub fn bits(&self) -> u32 {
        self.bits
    }
    #[doc = "Bits 0:5 - Channel 0 Offset Calibration Value"]
    #[inline]
    pub fn ch0offset(&self) -> CH0OFFSETR {
        let bits = {
            const MASK: u8 = 0x3f;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        CH0OFFSETR { bits }
    }
    #[doc = "Bits 8:13 - Channel 1 Offset Calibration Value"]
    #[inline]
    pub fn ch1offset(&self) -> CH1OFFSETR {
        let bits = {
            const MASK: u8 = 0x3f;
            const OFFSET: u8 = 8;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        CH1OFFSETR { bits }
    }
    #[doc = "Bits 16:22 - Gain Calibration Value"]
    #[inline]
    pub fn gain(&self) -> GAINR {
        let bits = {
            const MASK: u8 = 0x7f;
            const OFFSET: u8 = 16;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        GAINR { bits }
    }
}
impl W {
    #[doc = r" Reset value of the register"]
    #[inline]
    pub fn reset_value() -> W {
        W { bits: 0x0040_0000 }
    }
    #[doc = r" Writes raw bits to the register"]
    #[inline]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
    #[doc = "Bits 0:5 - Channel 0 Offset Calibration Value"]
    #[inline]
    pub fn ch0offset(&mut self) -> _CH0OFFSETW {
        _CH0OFFSETW { w: self }
    }
    #[doc = "Bits 8:13 - Channel 1 Offset Calibration Value"]
    #[inline]
    pub fn ch1offset(&mut self) -> _CH1OFFSETW {
        _CH1OFFSETW { w: self }
    }
    #[doc = "Bits 16:22 - Gain Calibration Value"]
    #[inline]
    pub fn gain(&mut self) -> _GAINW {
        _GAINW { w: self }
    }
}

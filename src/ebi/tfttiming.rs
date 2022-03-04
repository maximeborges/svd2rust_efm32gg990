#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::TFTTIMING {
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
pub struct DCLKPERIODR {
    bits: u16,
}
impl DCLKPERIODR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u16 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct TFTSTARTR {
    bits: u16,
}
impl TFTSTARTR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u16 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct TFTSETUPR {
    bits: u8,
}
impl TFTSETUPR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct TFTHOLDR {
    bits: u8,
}
impl TFTHOLDR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Proxy"]
pub struct _DCLKPERIODW<'a> {
    w: &'a mut W,
}
impl<'a> _DCLKPERIODW<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        const MASK: u16 = 0x07ff;
        const OFFSET: u8 = 0;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _TFTSTARTW<'a> {
    w: &'a mut W,
}
impl<'a> _TFTSTARTW<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        const MASK: u16 = 0x07ff;
        const OFFSET: u8 = 12;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _TFTSETUPW<'a> {
    w: &'a mut W,
}
impl<'a> _TFTSETUPW<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 0x03;
        const OFFSET: u8 = 24;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _TFTHOLDW<'a> {
    w: &'a mut W,
}
impl<'a> _TFTHOLDW<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 0x03;
        const OFFSET: u8 = 28;
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
    #[doc = "Bits 0:10 - TFT Direct Drive Transaction (EBI_DCLK) Period"]
    #[inline]
    pub fn dclkperiod(&self) -> DCLKPERIODR {
        let bits = {
            const MASK: u16 = 0x07ff;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) as u16
        };
        DCLKPERIODR { bits }
    }
    #[doc = "Bits 12:22 - TFT Direct Drive Transaction Start"]
    #[inline]
    pub fn tftstart(&self) -> TFTSTARTR {
        let bits = {
            const MASK: u16 = 0x07ff;
            const OFFSET: u8 = 12;
            ((self.bits >> OFFSET) & MASK as u32) as u16
        };
        TFTSTARTR { bits }
    }
    #[doc = "Bits 24:25 - TFT Setup Time"]
    #[inline]
    pub fn tftsetup(&self) -> TFTSETUPR {
        let bits = {
            const MASK: u8 = 0x03;
            const OFFSET: u8 = 24;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        TFTSETUPR { bits }
    }
    #[doc = "Bits 28:29 - TFT Hold Time"]
    #[inline]
    pub fn tfthold(&self) -> TFTHOLDR {
        let bits = {
            const MASK: u8 = 0x03;
            const OFFSET: u8 = 28;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        TFTHOLDR { bits }
    }
}
impl W {
    #[doc = r" Reset value of the register"]
    #[inline]
    pub fn reset_value() -> W {
        W { bits: 0 }
    }
    #[doc = r" Writes raw bits to the register"]
    #[inline]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
    #[doc = "Bits 0:10 - TFT Direct Drive Transaction (EBI_DCLK) Period"]
    #[inline]
    pub fn dclkperiod(&mut self) -> _DCLKPERIODW {
        _DCLKPERIODW { w: self }
    }
    #[doc = "Bits 12:22 - TFT Direct Drive Transaction Start"]
    #[inline]
    pub fn tftstart(&mut self) -> _TFTSTARTW {
        _TFTSTARTW { w: self }
    }
    #[doc = "Bits 24:25 - TFT Setup Time"]
    #[inline]
    pub fn tftsetup(&mut self) -> _TFTSETUPW {
        _TFTSETUPW { w: self }
    }
    #[doc = "Bits 28:29 - TFT Hold Time"]
    #[inline]
    pub fn tfthold(&mut self) -> _TFTHOLDW {
        _TFTHOLDW { w: self }
    }
}

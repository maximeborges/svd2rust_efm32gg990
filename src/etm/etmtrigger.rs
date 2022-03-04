#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::ETMTRIGGER {
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
pub struct RESAR {
    bits: u8,
}
impl RESAR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct RESBR {
    bits: u8,
}
impl RESBR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct ETMFCNR {
    bits: u8,
}
impl ETMFCNR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Proxy"]
pub struct _RESAW<'a> {
    w: &'a mut W,
}
impl<'a> _RESAW<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 0x7f;
        const OFFSET: u8 = 0;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _RESBW<'a> {
    w: &'a mut W,
}
impl<'a> _RESBW<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 0x7f;
        const OFFSET: u8 = 7;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _ETMFCNW<'a> {
    w: &'a mut W,
}
impl<'a> _ETMFCNW<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 0x07;
        const OFFSET: u8 = 14;
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
    #[doc = "Bits 0:6 - ETM Resource A"]
    #[inline]
    pub fn resa(&self) -> RESAR {
        let bits = {
            const MASK: u8 = 0x7f;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        RESAR { bits }
    }
    #[doc = "Bits 7:13 - ETM Resource B"]
    #[inline]
    pub fn resb(&self) -> RESBR {
        let bits = {
            const MASK: u8 = 0x7f;
            const OFFSET: u8 = 7;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        RESBR { bits }
    }
    #[doc = "Bits 14:16 - ETM Function"]
    #[inline]
    pub fn etmfcn(&self) -> ETMFCNR {
        let bits = {
            const MASK: u8 = 0x07;
            const OFFSET: u8 = 14;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        ETMFCNR { bits }
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
    #[doc = "Bits 0:6 - ETM Resource A"]
    #[inline]
    pub fn resa(&mut self) -> _RESAW {
        _RESAW { w: self }
    }
    #[doc = "Bits 7:13 - ETM Resource B"]
    #[inline]
    pub fn resb(&mut self) -> _RESBW {
        _RESBW { w: self }
    }
    #[doc = "Bits 14:16 - ETM Function"]
    #[inline]
    pub fn etmfcn(&mut self) -> _ETMFCNW {
        _ETMFCNW { w: self }
    }
}

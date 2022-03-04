#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::ETMTSEVR {
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
pub struct RESAEVTR {
    bits: u8,
}
impl RESAEVTR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct RESBEVTR {
    bits: u8,
}
impl RESBEVTR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct ETMFCNEVTR {
    bits: u8,
}
impl ETMFCNEVTR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Proxy"]
pub struct _RESAEVTW<'a> {
    w: &'a mut W,
}
impl<'a> _RESAEVTW<'a> {
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
pub struct _RESBEVTW<'a> {
    w: &'a mut W,
}
impl<'a> _RESBEVTW<'a> {
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
pub struct _ETMFCNEVTW<'a> {
    w: &'a mut W,
}
impl<'a> _ETMFCNEVTW<'a> {
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
    #[doc = "Bits 0:6 - ETM Resource A Event"]
    #[inline]
    pub fn resaevt(&self) -> RESAEVTR {
        let bits = {
            const MASK: u8 = 0x7f;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        RESAEVTR { bits }
    }
    #[doc = "Bits 7:13 - ETM Resource B Event"]
    #[inline]
    pub fn resbevt(&self) -> RESBEVTR {
        let bits = {
            const MASK: u8 = 0x7f;
            const OFFSET: u8 = 7;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        RESBEVTR { bits }
    }
    #[doc = "Bits 14:16 - ETM Function Event"]
    #[inline]
    pub fn etmfcnevt(&self) -> ETMFCNEVTR {
        let bits = {
            const MASK: u8 = 0x07;
            const OFFSET: u8 = 14;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        ETMFCNEVTR { bits }
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
    #[doc = "Bits 0:6 - ETM Resource A Event"]
    #[inline]
    pub fn resaevt(&mut self) -> _RESAEVTW {
        _RESAEVTW { w: self }
    }
    #[doc = "Bits 7:13 - ETM Resource B Event"]
    #[inline]
    pub fn resbevt(&mut self) -> _RESBEVTW {
        _RESBEVTW { w: self }
    }
    #[doc = "Bits 14:16 - ETM Function Event"]
    #[inline]
    pub fn etmfcnevt(&mut self) -> _ETMFCNEVTW {
        _ETMFCNEVTW { w: self }
    }
}

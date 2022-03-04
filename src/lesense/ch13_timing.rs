#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::CH13_TIMING {
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
pub struct EXTIMER {
    bits: u8,
}
impl EXTIMER {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct SAMPLEDLYR {
    bits: u8,
}
impl SAMPLEDLYR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct MEASUREDLYR {
    bits: u8,
}
impl MEASUREDLYR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Proxy"]
pub struct _EXTIMEW<'a> {
    w: &'a mut W,
}
impl<'a> _EXTIMEW<'a> {
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
pub struct _SAMPLEDLYW<'a> {
    w: &'a mut W,
}
impl<'a> _SAMPLEDLYW<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 0x7f;
        const OFFSET: u8 = 6;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _MEASUREDLYW<'a> {
    w: &'a mut W,
}
impl<'a> _MEASUREDLYW<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 0x7f;
        const OFFSET: u8 = 13;
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
    #[doc = "Bits 0:5 - Set excitation time"]
    #[inline]
    pub fn extime(&self) -> EXTIMER {
        let bits = {
            const MASK: u8 = 0x3f;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        EXTIMER { bits }
    }
    #[doc = "Bits 6:12 - Set sample delay"]
    #[inline]
    pub fn sampledly(&self) -> SAMPLEDLYR {
        let bits = {
            const MASK: u8 = 0x7f;
            const OFFSET: u8 = 6;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        SAMPLEDLYR { bits }
    }
    #[doc = "Bits 13:19 - Set measure delay"]
    #[inline]
    pub fn measuredly(&self) -> MEASUREDLYR {
        let bits = {
            const MASK: u8 = 0x7f;
            const OFFSET: u8 = 13;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        MEASUREDLYR { bits }
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
    #[doc = "Bits 0:5 - Set excitation time"]
    #[inline]
    pub fn extime(&mut self) -> _EXTIMEW {
        _EXTIMEW { w: self }
    }
    #[doc = "Bits 6:12 - Set sample delay"]
    #[inline]
    pub fn sampledly(&mut self) -> _SAMPLEDLYW {
        _SAMPLEDLYW { w: self }
    }
    #[doc = "Bits 13:19 - Set measure delay"]
    #[inline]
    pub fn measuredly(&mut self) -> _MEASUREDLYW {
        _MEASUREDLYW { w: self }
    }
}

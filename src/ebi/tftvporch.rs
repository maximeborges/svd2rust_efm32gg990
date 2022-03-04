#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::TFTVPORCH {
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
pub struct VSYNCR {
    bits: u8,
}
impl VSYNCR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct VFPORCHR {
    bits: u8,
}
impl VFPORCHR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct VBPORCHR {
    bits: u8,
}
impl VBPORCHR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Proxy"]
pub struct _VSYNCW<'a> {
    w: &'a mut W,
}
impl<'a> _VSYNCW<'a> {
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
pub struct _VFPORCHW<'a> {
    w: &'a mut W,
}
impl<'a> _VFPORCHW<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 0xff;
        const OFFSET: u8 = 8;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _VBPORCHW<'a> {
    w: &'a mut W,
}
impl<'a> _VBPORCHW<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 0xff;
        const OFFSET: u8 = 18;
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
    #[doc = "Bits 0:6 - Vertical Synchronization Pulse Width"]
    #[inline]
    pub fn vsync(&self) -> VSYNCR {
        let bits = {
            const MASK: u8 = 0x7f;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        VSYNCR { bits }
    }
    #[doc = "Bits 8:15 - Vertical Front Porch Size"]
    #[inline]
    pub fn vfporch(&self) -> VFPORCHR {
        let bits = {
            const MASK: u8 = 0xff;
            const OFFSET: u8 = 8;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        VFPORCHR { bits }
    }
    #[doc = "Bits 18:25 - Vertical Back Porch Size"]
    #[inline]
    pub fn vbporch(&self) -> VBPORCHR {
        let bits = {
            const MASK: u8 = 0xff;
            const OFFSET: u8 = 18;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        VBPORCHR { bits }
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
    #[doc = "Bits 0:6 - Vertical Synchronization Pulse Width"]
    #[inline]
    pub fn vsync(&mut self) -> _VSYNCW {
        _VSYNCW { w: self }
    }
    #[doc = "Bits 8:15 - Vertical Front Porch Size"]
    #[inline]
    pub fn vfporch(&mut self) -> _VFPORCHW {
        _VFPORCHW { w: self }
    }
    #[doc = "Bits 18:25 - Vertical Back Porch Size"]
    #[inline]
    pub fn vbporch(&mut self) -> _VBPORCHW {
        _VBPORCHW { w: self }
    }
}

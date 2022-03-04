#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::TFTHPORCH {
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
pub struct HSYNCR {
    bits: u8,
}
impl HSYNCR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct HFPORCHR {
    bits: u8,
}
impl HFPORCHR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct HBPORCHR {
    bits: u8,
}
impl HBPORCHR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct HSYNCSTARTR {
    bits: u8,
}
impl HSYNCSTARTR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Proxy"]
pub struct _HSYNCW<'a> {
    w: &'a mut W,
}
impl<'a> _HSYNCW<'a> {
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
pub struct _HFPORCHW<'a> {
    w: &'a mut W,
}
impl<'a> _HFPORCHW<'a> {
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
pub struct _HBPORCHW<'a> {
    w: &'a mut W,
}
impl<'a> _HBPORCHW<'a> {
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
#[doc = r" Proxy"]
pub struct _HSYNCSTARTW<'a> {
    w: &'a mut W,
}
impl<'a> _HSYNCSTARTW<'a> {
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
    #[doc = "Bits 0:6 - Horizontal Synchronization Pulse Width"]
    #[inline]
    pub fn hsync(&self) -> HSYNCR {
        let bits = {
            const MASK: u8 = 0x7f;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        HSYNCR { bits }
    }
    #[doc = "Bits 8:15 - Horizontal Front Porch Size"]
    #[inline]
    pub fn hfporch(&self) -> HFPORCHR {
        let bits = {
            const MASK: u8 = 0xff;
            const OFFSET: u8 = 8;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        HFPORCHR { bits }
    }
    #[doc = "Bits 18:25 - Horizontal Back Porch Size"]
    #[inline]
    pub fn hbporch(&self) -> HBPORCHR {
        let bits = {
            const MASK: u8 = 0xff;
            const OFFSET: u8 = 18;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        HBPORCHR { bits }
    }
    #[doc = "Bits 28:29 - HSYNC Start Delay"]
    #[inline]
    pub fn hsyncstart(&self) -> HSYNCSTARTR {
        let bits = {
            const MASK: u8 = 0x03;
            const OFFSET: u8 = 28;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        HSYNCSTARTR { bits }
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
    #[doc = "Bits 0:6 - Horizontal Synchronization Pulse Width"]
    #[inline]
    pub fn hsync(&mut self) -> _HSYNCW {
        _HSYNCW { w: self }
    }
    #[doc = "Bits 8:15 - Horizontal Front Porch Size"]
    #[inline]
    pub fn hfporch(&mut self) -> _HFPORCHW {
        _HFPORCHW { w: self }
    }
    #[doc = "Bits 18:25 - Horizontal Back Porch Size"]
    #[inline]
    pub fn hbporch(&mut self) -> _HBPORCHW {
        _HBPORCHW { w: self }
    }
    #[doc = "Bits 28:29 - HSYNC Start Delay"]
    #[inline]
    pub fn hsyncstart(&mut self) -> _HSYNCSTARTW {
        _HSYNCSTARTW { w: self }
    }
}

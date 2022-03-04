#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::RECT0 {
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
pub struct HEIGHTR {
    bits: u16,
}
impl HEIGHTR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u16 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct SRCSTRIDER {
    bits: u16,
}
impl SRCSTRIDER {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u16 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct DSTSTRIDER {
    bits: u16,
}
impl DSTSTRIDER {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u16 {
        self.bits
    }
}
#[doc = r" Proxy"]
pub struct _HEIGHTW<'a> {
    w: &'a mut W,
}
impl<'a> _HEIGHTW<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        const MASK: u16 = 0x03ff;
        const OFFSET: u8 = 0;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _SRCSTRIDEW<'a> {
    w: &'a mut W,
}
impl<'a> _SRCSTRIDEW<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        const MASK: u16 = 0x07ff;
        const OFFSET: u8 = 10;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _DSTSTRIDEW<'a> {
    w: &'a mut W,
}
impl<'a> _DSTSTRIDEW<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        const MASK: u16 = 0x07ff;
        const OFFSET: u8 = 21;
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
    #[doc = "Bits 0:9 - DMA Channel 0 Rectangle Height"]
    #[inline]
    pub fn height(&self) -> HEIGHTR {
        let bits = {
            const MASK: u16 = 0x03ff;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) as u16
        };
        HEIGHTR { bits }
    }
    #[doc = "Bits 10:20 - DMA Channel 0 Source Stride"]
    #[inline]
    pub fn srcstride(&self) -> SRCSTRIDER {
        let bits = {
            const MASK: u16 = 0x07ff;
            const OFFSET: u8 = 10;
            ((self.bits >> OFFSET) & MASK as u32) as u16
        };
        SRCSTRIDER { bits }
    }
    #[doc = "Bits 21:31 - DMA Channel 0 Destination Stride"]
    #[inline]
    pub fn dststride(&self) -> DSTSTRIDER {
        let bits = {
            const MASK: u16 = 0x07ff;
            const OFFSET: u8 = 21;
            ((self.bits >> OFFSET) & MASK as u32) as u16
        };
        DSTSTRIDER { bits }
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
    #[doc = "Bits 0:9 - DMA Channel 0 Rectangle Height"]
    #[inline]
    pub fn height(&mut self) -> _HEIGHTW {
        _HEIGHTW { w: self }
    }
    #[doc = "Bits 10:20 - DMA Channel 0 Source Stride"]
    #[inline]
    pub fn srcstride(&mut self) -> _SRCSTRIDEW {
        _SRCSTRIDEW { w: self }
    }
    #[doc = "Bits 21:31 - DMA Channel 0 Destination Stride"]
    #[inline]
    pub fn dststride(&mut self) -> _DSTSTRIDEW {
        _DSTSTRIDEW { w: self }
    }
}

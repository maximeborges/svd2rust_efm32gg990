#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::DIEPTXF4 {
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
pub struct INEPNTXFSTADDRR {
    bits: u16,
}
impl INEPNTXFSTADDRR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u16 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct INEPNTXFDEPR {
    bits: u16,
}
impl INEPNTXFDEPR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u16 {
        self.bits
    }
}
#[doc = r" Proxy"]
pub struct _INEPNTXFSTADDRW<'a> {
    w: &'a mut W,
}
impl<'a> _INEPNTXFSTADDRW<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        const MASK: u16 = 0x0fff;
        const OFFSET: u8 = 0;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _INEPNTXFDEPW<'a> {
    w: &'a mut W,
}
impl<'a> _INEPNTXFDEPW<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        const MASK: u16 = 0x03ff;
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
    #[doc = "Bits 0:11 - IN Endpoint FIFO 4 Transmit RAM Start Address"]
    #[inline]
    pub fn inepntxfstaddr(&self) -> INEPNTXFSTADDRR {
        let bits = {
            const MASK: u16 = 0x0fff;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) as u16
        };
        INEPNTXFSTADDRR { bits }
    }
    #[doc = "Bits 16:25 - IN Endpoint TxFIFO Depth"]
    #[inline]
    pub fn inepntxfdep(&self) -> INEPNTXFDEPR {
        let bits = {
            const MASK: u16 = 0x03ff;
            const OFFSET: u8 = 16;
            ((self.bits >> OFFSET) & MASK as u32) as u16
        };
        INEPNTXFDEPR { bits }
    }
}
impl W {
    #[doc = r" Reset value of the register"]
    #[inline]
    pub fn reset_value() -> W {
        W { bits: 0x0200_0a00 }
    }
    #[doc = r" Writes raw bits to the register"]
    #[inline]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
    #[doc = "Bits 0:11 - IN Endpoint FIFO 4 Transmit RAM Start Address"]
    #[inline]
    pub fn inepntxfstaddr(&mut self) -> _INEPNTXFSTADDRW {
        _INEPNTXFSTADDRW { w: self }
    }
    #[doc = "Bits 16:25 - IN Endpoint TxFIFO Depth"]
    #[inline]
    pub fn inepntxfdep(&mut self) -> _INEPNTXFDEPW {
        _INEPNTXFDEPW { w: self }
    }
}

#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::LFXOFDET {
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
#[doc = "Possible values of the field `OSC`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum OSCR {
    #[doc = "LFXO failure detection disabled."]
    DISABLE,
    #[doc = "LFRCO used for LFXO failure detection."]
    LFRCO,
    #[doc = "ULFRCO used for LFXO failure detection."]
    ULFRCO,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl OSCR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            OSCR::DISABLE => 0,
            OSCR::LFRCO => 0x01,
            OSCR::ULFRCO => 0x02,
            OSCR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> OSCR {
        match value {
            0 => OSCR::DISABLE,
            1 => OSCR::LFRCO,
            2 => OSCR::ULFRCO,
            i => OSCR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline]
    pub fn is_disable(&self) -> bool {
        *self == OSCR::DISABLE
    }
    #[doc = "Checks if the value of the field is `LFRCO`"]
    #[inline]
    pub fn is_lfrco(&self) -> bool {
        *self == OSCR::LFRCO
    }
    #[doc = "Checks if the value of the field is `ULFRCO`"]
    #[inline]
    pub fn is_ulfrco(&self) -> bool {
        *self == OSCR::ULFRCO
    }
}
#[doc = r" Value of the field"]
pub struct TOPR {
    bits: u8,
}
impl TOPR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = "Values that can be written to the field `OSC`"]
pub enum OSCW {
    #[doc = "LFXO failure detection disabled."]
    DISABLE,
    #[doc = "LFRCO used for LFXO failure detection."]
    LFRCO,
    #[doc = "ULFRCO used for LFXO failure detection."]
    ULFRCO,
}
impl OSCW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            OSCW::DISABLE => 0,
            OSCW::LFRCO => 1,
            OSCW::ULFRCO => 2,
        }
    }
}
#[doc = r" Proxy"]
pub struct _OSCW<'a> {
    w: &'a mut W,
}
impl<'a> _OSCW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: OSCW) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "LFXO failure detection disabled."]
    #[inline]
    pub fn disable(self) -> &'a mut W {
        self.variant(OSCW::DISABLE)
    }
    #[doc = "LFRCO used for LFXO failure detection."]
    #[inline]
    pub fn lfrco(self) -> &'a mut W {
        self.variant(OSCW::LFRCO)
    }
    #[doc = "ULFRCO used for LFXO failure detection."]
    #[inline]
    pub fn ulfrco(self) -> &'a mut W {
        self.variant(OSCW::ULFRCO)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 0x03;
        const OFFSET: u8 = 0;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _TOPW<'a> {
    w: &'a mut W,
}
impl<'a> _TOPW<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 0x1f;
        const OFFSET: u8 = 4;
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
    #[doc = "Bits 0:1 - LFXO failure detection configuration."]
    #[inline]
    pub fn osc(&self) -> OSCR {
        OSCR::_from({
            const MASK: u8 = 0x03;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 4:8 - LFXO failure counter top value."]
    #[inline]
    pub fn top(&self) -> TOPR {
        let bits = {
            const MASK: u8 = 0x1f;
            const OFFSET: u8 = 4;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        TOPR { bits }
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
    #[doc = "Bits 0:1 - LFXO failure detection configuration."]
    #[inline]
    pub fn osc(&mut self) -> _OSCW {
        _OSCW { w: self }
    }
    #[doc = "Bits 4:8 - LFXO failure counter top value."]
    #[inline]
    pub fn top(&mut self) -> _TOPW {
        _TOPW { w: self }
    }
}

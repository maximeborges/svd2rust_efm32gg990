#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::LPMODE {
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
#[doc = "Possible values of the field `LPMODE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LPMODER {
    #[doc = "Low power mode is disabled."]
    DISABLE,
    #[doc = "Low power mode always enabled."]
    ENABLE,
    #[doc = "Low power mode enabled in backup mode."]
    BUEN,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl LPMODER {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            LPMODER::DISABLE => 0,
            LPMODER::ENABLE => 0x01,
            LPMODER::BUEN => 0x02,
            LPMODER::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> LPMODER {
        match value {
            0 => LPMODER::DISABLE,
            1 => LPMODER::ENABLE,
            2 => LPMODER::BUEN,
            i => LPMODER::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline]
    pub fn is_disable(&self) -> bool {
        *self == LPMODER::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline]
    pub fn is_enable(&self) -> bool {
        *self == LPMODER::ENABLE
    }
    #[doc = "Checks if the value of the field is `BUEN`"]
    #[inline]
    pub fn is_buen(&self) -> bool {
        *self == LPMODER::BUEN
    }
}
#[doc = "Values that can be written to the field `LPMODE`"]
pub enum LPMODEW {
    #[doc = "Low power mode is disabled."]
    DISABLE,
    #[doc = "Low power mode always enabled."]
    ENABLE,
    #[doc = "Low power mode enabled in backup mode."]
    BUEN,
}
impl LPMODEW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            LPMODEW::DISABLE => 0,
            LPMODEW::ENABLE => 1,
            LPMODEW::BUEN => 2,
        }
    }
}
#[doc = r" Proxy"]
pub struct _LPMODEW<'a> {
    w: &'a mut W,
}
impl<'a> _LPMODEW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: LPMODEW) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "Low power mode is disabled."]
    #[inline]
    pub fn disable(self) -> &'a mut W {
        self.variant(LPMODEW::DISABLE)
    }
    #[doc = "Low power mode always enabled."]
    #[inline]
    pub fn enable(self) -> &'a mut W {
        self.variant(LPMODEW::ENABLE)
    }
    #[doc = "Low power mode enabled in backup mode."]
    #[inline]
    pub fn buen(self) -> &'a mut W {
        self.variant(LPMODEW::BUEN)
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
impl R {
    #[doc = r" Value of the register as raw bits"]
    #[inline]
    pub fn bits(&self) -> u32 {
        self.bits
    }
    #[doc = "Bits 0:1 - Low power mode configuration."]
    #[inline]
    pub fn lpmode(&self) -> LPMODER {
        LPMODER::_from({
            const MASK: u8 = 0x03;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
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
    #[doc = "Bits 0:1 - Low power mode configuration."]
    #[inline]
    pub fn lpmode(&mut self) -> _LPMODEW {
        _LPMODEW { w: self }
    }
}

#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::BIASCTRL {
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
#[doc = "Possible values of the field `BIASMODE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BIASMODER {
    #[doc = "Bias module duty cycled between low power and high accuracy mode"]
    DUTYCYCLE,
    #[doc = "Bias module always in high accuracy mode"]
    HIGHACC,
    #[doc = "Bias module not affected by LESENSE"]
    DONTTOUCH,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl BIASMODER {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            BIASMODER::DUTYCYCLE => 0,
            BIASMODER::HIGHACC => 0x01,
            BIASMODER::DONTTOUCH => 0x02,
            BIASMODER::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> BIASMODER {
        match value {
            0 => BIASMODER::DUTYCYCLE,
            1 => BIASMODER::HIGHACC,
            2 => BIASMODER::DONTTOUCH,
            i => BIASMODER::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `DUTYCYCLE`"]
    #[inline]
    pub fn is_dutycycle(&self) -> bool {
        *self == BIASMODER::DUTYCYCLE
    }
    #[doc = "Checks if the value of the field is `HIGHACC`"]
    #[inline]
    pub fn is_highacc(&self) -> bool {
        *self == BIASMODER::HIGHACC
    }
    #[doc = "Checks if the value of the field is `DONTTOUCH`"]
    #[inline]
    pub fn is_donttouch(&self) -> bool {
        *self == BIASMODER::DONTTOUCH
    }
}
#[doc = "Values that can be written to the field `BIASMODE`"]
pub enum BIASMODEW {
    #[doc = "Bias module duty cycled between low power and high accuracy mode"]
    DUTYCYCLE,
    #[doc = "Bias module always in high accuracy mode"]
    HIGHACC,
    #[doc = "Bias module not affected by LESENSE"]
    DONTTOUCH,
}
impl BIASMODEW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            BIASMODEW::DUTYCYCLE => 0,
            BIASMODEW::HIGHACC => 1,
            BIASMODEW::DONTTOUCH => 2,
        }
    }
}
#[doc = r" Proxy"]
pub struct _BIASMODEW<'a> {
    w: &'a mut W,
}
impl<'a> _BIASMODEW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: BIASMODEW) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "Bias module duty cycled between low power and high accuracy mode"]
    #[inline]
    pub fn dutycycle(self) -> &'a mut W {
        self.variant(BIASMODEW::DUTYCYCLE)
    }
    #[doc = "Bias module always in high accuracy mode"]
    #[inline]
    pub fn highacc(self) -> &'a mut W {
        self.variant(BIASMODEW::HIGHACC)
    }
    #[doc = "Bias module not affected by LESENSE"]
    #[inline]
    pub fn donttouch(self) -> &'a mut W {
        self.variant(BIASMODEW::DONTTOUCH)
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
    #[doc = "Bits 0:1 - Select bias mode"]
    #[inline]
    pub fn biasmode(&self) -> BIASMODER {
        BIASMODER::_from({
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
    #[doc = "Bits 0:1 - Select bias mode"]
    #[inline]
    pub fn biasmode(&mut self) -> _BIASMODEW {
        _BIASMODEW { w: self }
    }
}

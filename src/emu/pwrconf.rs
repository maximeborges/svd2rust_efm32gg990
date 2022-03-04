#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::PWRCONF {
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
pub struct VOUTWEAKR {
    bits: bool,
}
impl VOUTWEAKR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bit(&self) -> bool {
        self.bits
    }
    #[doc = r" Returns `true` if the bit is clear (0)"]
    #[inline]
    pub fn bit_is_clear(&self) -> bool {
        !self.bit()
    }
    #[doc = r" Returns `true` if the bit is set (1)"]
    #[inline]
    pub fn bit_is_set(&self) -> bool {
        self.bit()
    }
}
#[doc = r" Value of the field"]
pub struct VOUTMEDR {
    bits: bool,
}
impl VOUTMEDR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bit(&self) -> bool {
        self.bits
    }
    #[doc = r" Returns `true` if the bit is clear (0)"]
    #[inline]
    pub fn bit_is_clear(&self) -> bool {
        !self.bit()
    }
    #[doc = r" Returns `true` if the bit is set (1)"]
    #[inline]
    pub fn bit_is_set(&self) -> bool {
        self.bit()
    }
}
#[doc = r" Value of the field"]
pub struct VOUTSTRONGR {
    bits: bool,
}
impl VOUTSTRONGR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bit(&self) -> bool {
        self.bits
    }
    #[doc = r" Returns `true` if the bit is clear (0)"]
    #[inline]
    pub fn bit_is_clear(&self) -> bool {
        !self.bit()
    }
    #[doc = r" Returns `true` if the bit is set (1)"]
    #[inline]
    pub fn bit_is_set(&self) -> bool {
        self.bit()
    }
}
#[doc = "Possible values of the field `PWRRES`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PWRRESR {
    #[doc = "Main power and backup power connected with RES0 series resistance."]
    RES0,
    #[doc = "Main power and backup power connected with RES1 series resistance."]
    RES1,
    #[doc = "Main power and backup power connected with RES2 series resistance."]
    RES2,
    #[doc = "Main power and backup power connected with RES3 series resistance."]
    RES3,
}
impl PWRRESR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            PWRRESR::RES0 => 0,
            PWRRESR::RES1 => 0x01,
            PWRRESR::RES2 => 0x02,
            PWRRESR::RES3 => 0x03,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> PWRRESR {
        match value {
            0 => PWRRESR::RES0,
            1 => PWRRESR::RES1,
            2 => PWRRESR::RES2,
            3 => PWRRESR::RES3,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `RES0`"]
    #[inline]
    pub fn is_res0(&self) -> bool {
        *self == PWRRESR::RES0
    }
    #[doc = "Checks if the value of the field is `RES1`"]
    #[inline]
    pub fn is_res1(&self) -> bool {
        *self == PWRRESR::RES1
    }
    #[doc = "Checks if the value of the field is `RES2`"]
    #[inline]
    pub fn is_res2(&self) -> bool {
        *self == PWRRESR::RES2
    }
    #[doc = "Checks if the value of the field is `RES3`"]
    #[inline]
    pub fn is_res3(&self) -> bool {
        *self == PWRRESR::RES3
    }
}
#[doc = r" Proxy"]
pub struct _VOUTWEAKW<'a> {
    w: &'a mut W,
}
impl<'a> _VOUTWEAKW<'a> {
    #[doc = r" Sets the field bit"]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r" Clears the field bit"]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bit(self, value: bool) -> &'a mut W {
        const MASK: bool = true;
        const OFFSET: u8 = 0;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _VOUTMEDW<'a> {
    w: &'a mut W,
}
impl<'a> _VOUTMEDW<'a> {
    #[doc = r" Sets the field bit"]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r" Clears the field bit"]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bit(self, value: bool) -> &'a mut W {
        const MASK: bool = true;
        const OFFSET: u8 = 1;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _VOUTSTRONGW<'a> {
    w: &'a mut W,
}
impl<'a> _VOUTSTRONGW<'a> {
    #[doc = r" Sets the field bit"]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r" Clears the field bit"]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bit(self, value: bool) -> &'a mut W {
        const MASK: bool = true;
        const OFFSET: u8 = 2;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `PWRRES`"]
pub enum PWRRESW {
    #[doc = "Main power and backup power connected with RES0 series resistance."]
    RES0,
    #[doc = "Main power and backup power connected with RES1 series resistance."]
    RES1,
    #[doc = "Main power and backup power connected with RES2 series resistance."]
    RES2,
    #[doc = "Main power and backup power connected with RES3 series resistance."]
    RES3,
}
impl PWRRESW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            PWRRESW::RES0 => 0,
            PWRRESW::RES1 => 1,
            PWRRESW::RES2 => 2,
            PWRRESW::RES3 => 3,
        }
    }
}
#[doc = r" Proxy"]
pub struct _PWRRESW<'a> {
    w: &'a mut W,
}
impl<'a> _PWRRESW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: PWRRESW) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "Main power and backup power connected with RES0 series resistance."]
    #[inline]
    pub fn res0(self) -> &'a mut W {
        self.variant(PWRRESW::RES0)
    }
    #[doc = "Main power and backup power connected with RES1 series resistance."]
    #[inline]
    pub fn res1(self) -> &'a mut W {
        self.variant(PWRRESW::RES1)
    }
    #[doc = "Main power and backup power connected with RES2 series resistance."]
    #[inline]
    pub fn res2(self) -> &'a mut W {
        self.variant(PWRRESW::RES2)
    }
    #[doc = "Main power and backup power connected with RES3 series resistance."]
    #[inline]
    pub fn res3(self) -> &'a mut W {
        self.variant(PWRRESW::RES3)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 0x03;
        const OFFSET: u8 = 3;
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
    #[doc = "Bit 0 - BU_VOUT weak enable"]
    #[inline]
    pub fn voutweak(&self) -> VOUTWEAKR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        VOUTWEAKR { bits }
    }
    #[doc = "Bit 1 - BU_VOUT medium enable"]
    #[inline]
    pub fn voutmed(&self) -> VOUTMEDR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 1;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        VOUTMEDR { bits }
    }
    #[doc = "Bit 2 - BU_VOUT strong enable"]
    #[inline]
    pub fn voutstrong(&self) -> VOUTSTRONGR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 2;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        VOUTSTRONGR { bits }
    }
    #[doc = "Bits 3:4 - Power domain resistor select"]
    #[inline]
    pub fn pwrres(&self) -> PWRRESR {
        PWRRESR::_from({
            const MASK: u8 = 0x03;
            const OFFSET: u8 = 3;
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
    #[doc = "Bit 0 - BU_VOUT weak enable"]
    #[inline]
    pub fn voutweak(&mut self) -> _VOUTWEAKW {
        _VOUTWEAKW { w: self }
    }
    #[doc = "Bit 1 - BU_VOUT medium enable"]
    #[inline]
    pub fn voutmed(&mut self) -> _VOUTMEDW {
        _VOUTMEDW { w: self }
    }
    #[doc = "Bit 2 - BU_VOUT strong enable"]
    #[inline]
    pub fn voutstrong(&mut self) -> _VOUTSTRONGW {
        _VOUTSTRONGW { w: self }
    }
    #[doc = "Bits 3:4 - Power domain resistor select"]
    #[inline]
    pub fn pwrres(&mut self) -> _PWRRESW {
        _PWRRESW { w: self }
    }
}

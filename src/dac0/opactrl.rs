#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::OPACTRL {
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
pub struct OPA0ENR {
    bits: bool,
}
impl OPA0ENR {
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
pub struct OPA1ENR {
    bits: bool,
}
impl OPA1ENR {
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
pub struct OPA2ENR {
    bits: bool,
}
impl OPA2ENR {
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
pub struct OPA0HCMDISR {
    bits: bool,
}
impl OPA0HCMDISR {
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
pub struct OPA1HCMDISR {
    bits: bool,
}
impl OPA1HCMDISR {
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
pub struct OPA2HCMDISR {
    bits: bool,
}
impl OPA2HCMDISR {
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
#[doc = "Possible values of the field `OPA0LPFDIS`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum OPA0LPFDISR {
    #[doc = "Disables the LPF between positive pad and positive input."]
    PLPFDIS,
    #[doc = "Disables the LPF between negative pad and negative input."]
    NLPFDIS,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl OPA0LPFDISR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            OPA0LPFDISR::PLPFDIS => 0x01,
            OPA0LPFDISR::NLPFDIS => 0x02,
            OPA0LPFDISR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> OPA0LPFDISR {
        match value {
            1 => OPA0LPFDISR::PLPFDIS,
            2 => OPA0LPFDISR::NLPFDIS,
            i => OPA0LPFDISR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `PLPFDIS`"]
    #[inline]
    pub fn is_plpfdis(&self) -> bool {
        *self == OPA0LPFDISR::PLPFDIS
    }
    #[doc = "Checks if the value of the field is `NLPFDIS`"]
    #[inline]
    pub fn is_nlpfdis(&self) -> bool {
        *self == OPA0LPFDISR::NLPFDIS
    }
}
#[doc = "Possible values of the field `OPA1LPFDIS`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum OPA1LPFDISR {
    #[doc = "Disables the LPF between positive pad and positive input."]
    PLPFDIS,
    #[doc = "Disables the LPF between negative pad and negative input."]
    NLPFDIS,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl OPA1LPFDISR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            OPA1LPFDISR::PLPFDIS => 0x01,
            OPA1LPFDISR::NLPFDIS => 0x02,
            OPA1LPFDISR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> OPA1LPFDISR {
        match value {
            1 => OPA1LPFDISR::PLPFDIS,
            2 => OPA1LPFDISR::NLPFDIS,
            i => OPA1LPFDISR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `PLPFDIS`"]
    #[inline]
    pub fn is_plpfdis(&self) -> bool {
        *self == OPA1LPFDISR::PLPFDIS
    }
    #[doc = "Checks if the value of the field is `NLPFDIS`"]
    #[inline]
    pub fn is_nlpfdis(&self) -> bool {
        *self == OPA1LPFDISR::NLPFDIS
    }
}
#[doc = "Possible values of the field `OPA2LPFDIS`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum OPA2LPFDISR {
    #[doc = "Disables the LPF between positive pad and positive input."]
    PLPFDIS,
    #[doc = "Disables the LPF between negative pad and negative input."]
    NLPFDIS,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl OPA2LPFDISR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            OPA2LPFDISR::PLPFDIS => 0x01,
            OPA2LPFDISR::NLPFDIS => 0x02,
            OPA2LPFDISR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> OPA2LPFDISR {
        match value {
            1 => OPA2LPFDISR::PLPFDIS,
            2 => OPA2LPFDISR::NLPFDIS,
            i => OPA2LPFDISR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `PLPFDIS`"]
    #[inline]
    pub fn is_plpfdis(&self) -> bool {
        *self == OPA2LPFDISR::PLPFDIS
    }
    #[doc = "Checks if the value of the field is `NLPFDIS`"]
    #[inline]
    pub fn is_nlpfdis(&self) -> bool {
        *self == OPA2LPFDISR::NLPFDIS
    }
}
#[doc = r" Value of the field"]
pub struct OPA0SHORTR {
    bits: bool,
}
impl OPA0SHORTR {
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
pub struct OPA1SHORTR {
    bits: bool,
}
impl OPA1SHORTR {
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
pub struct OPA2SHORTR {
    bits: bool,
}
impl OPA2SHORTR {
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
#[doc = r" Proxy"]
pub struct _OPA0ENW<'a> {
    w: &'a mut W,
}
impl<'a> _OPA0ENW<'a> {
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
pub struct _OPA1ENW<'a> {
    w: &'a mut W,
}
impl<'a> _OPA1ENW<'a> {
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
pub struct _OPA2ENW<'a> {
    w: &'a mut W,
}
impl<'a> _OPA2ENW<'a> {
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
#[doc = r" Proxy"]
pub struct _OPA0HCMDISW<'a> {
    w: &'a mut W,
}
impl<'a> _OPA0HCMDISW<'a> {
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
        const OFFSET: u8 = 6;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _OPA1HCMDISW<'a> {
    w: &'a mut W,
}
impl<'a> _OPA1HCMDISW<'a> {
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
        const OFFSET: u8 = 7;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _OPA2HCMDISW<'a> {
    w: &'a mut W,
}
impl<'a> _OPA2HCMDISW<'a> {
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
        const OFFSET: u8 = 8;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `OPA0LPFDIS`"]
pub enum OPA0LPFDISW {
    #[doc = "Disables the LPF between positive pad and positive input."]
    PLPFDIS,
    #[doc = "Disables the LPF between negative pad and negative input."]
    NLPFDIS,
}
impl OPA0LPFDISW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            OPA0LPFDISW::PLPFDIS => 1,
            OPA0LPFDISW::NLPFDIS => 2,
        }
    }
}
#[doc = r" Proxy"]
pub struct _OPA0LPFDISW<'a> {
    w: &'a mut W,
}
impl<'a> _OPA0LPFDISW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: OPA0LPFDISW) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "Disables the LPF between positive pad and positive input."]
    #[inline]
    pub fn plpfdis(self) -> &'a mut W {
        self.variant(OPA0LPFDISW::PLPFDIS)
    }
    #[doc = "Disables the LPF between negative pad and negative input."]
    #[inline]
    pub fn nlpfdis(self) -> &'a mut W {
        self.variant(OPA0LPFDISW::NLPFDIS)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 0x03;
        const OFFSET: u8 = 12;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `OPA1LPFDIS`"]
pub enum OPA1LPFDISW {
    #[doc = "Disables the LPF between positive pad and positive input."]
    PLPFDIS,
    #[doc = "Disables the LPF between negative pad and negative input."]
    NLPFDIS,
}
impl OPA1LPFDISW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            OPA1LPFDISW::PLPFDIS => 1,
            OPA1LPFDISW::NLPFDIS => 2,
        }
    }
}
#[doc = r" Proxy"]
pub struct _OPA1LPFDISW<'a> {
    w: &'a mut W,
}
impl<'a> _OPA1LPFDISW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: OPA1LPFDISW) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "Disables the LPF between positive pad and positive input."]
    #[inline]
    pub fn plpfdis(self) -> &'a mut W {
        self.variant(OPA1LPFDISW::PLPFDIS)
    }
    #[doc = "Disables the LPF between negative pad and negative input."]
    #[inline]
    pub fn nlpfdis(self) -> &'a mut W {
        self.variant(OPA1LPFDISW::NLPFDIS)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 0x03;
        const OFFSET: u8 = 14;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `OPA2LPFDIS`"]
pub enum OPA2LPFDISW {
    #[doc = "Disables the LPF between positive pad and positive input."]
    PLPFDIS,
    #[doc = "Disables the LPF between negative pad and negative input."]
    NLPFDIS,
}
impl OPA2LPFDISW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            OPA2LPFDISW::PLPFDIS => 1,
            OPA2LPFDISW::NLPFDIS => 2,
        }
    }
}
#[doc = r" Proxy"]
pub struct _OPA2LPFDISW<'a> {
    w: &'a mut W,
}
impl<'a> _OPA2LPFDISW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: OPA2LPFDISW) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "Disables the LPF between positive pad and positive input."]
    #[inline]
    pub fn plpfdis(self) -> &'a mut W {
        self.variant(OPA2LPFDISW::PLPFDIS)
    }
    #[doc = "Disables the LPF between negative pad and negative input."]
    #[inline]
    pub fn nlpfdis(self) -> &'a mut W {
        self.variant(OPA2LPFDISW::NLPFDIS)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 0x03;
        const OFFSET: u8 = 16;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _OPA0SHORTW<'a> {
    w: &'a mut W,
}
impl<'a> _OPA0SHORTW<'a> {
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
        const OFFSET: u8 = 22;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _OPA1SHORTW<'a> {
    w: &'a mut W,
}
impl<'a> _OPA1SHORTW<'a> {
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
        const OFFSET: u8 = 23;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _OPA2SHORTW<'a> {
    w: &'a mut W,
}
impl<'a> _OPA2SHORTW<'a> {
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
        const OFFSET: u8 = 24;
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
    #[doc = "Bit 0 - OPA0 Enable"]
    #[inline]
    pub fn opa0en(&self) -> OPA0ENR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        OPA0ENR { bits }
    }
    #[doc = "Bit 1 - OPA1 Enable"]
    #[inline]
    pub fn opa1en(&self) -> OPA1ENR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 1;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        OPA1ENR { bits }
    }
    #[doc = "Bit 2 - OPA2 Enable"]
    #[inline]
    pub fn opa2en(&self) -> OPA2ENR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 2;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        OPA2ENR { bits }
    }
    #[doc = "Bit 6 - High Common Mode Disable."]
    #[inline]
    pub fn opa0hcmdis(&self) -> OPA0HCMDISR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 6;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        OPA0HCMDISR { bits }
    }
    #[doc = "Bit 7 - High Common Mode Disable."]
    #[inline]
    pub fn opa1hcmdis(&self) -> OPA1HCMDISR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 7;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        OPA1HCMDISR { bits }
    }
    #[doc = "Bit 8 - High Common Mode Disable."]
    #[inline]
    pub fn opa2hcmdis(&self) -> OPA2HCMDISR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 8;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        OPA2HCMDISR { bits }
    }
    #[doc = "Bits 12:13 - Disables Low Pass Filter."]
    #[inline]
    pub fn opa0lpfdis(&self) -> OPA0LPFDISR {
        OPA0LPFDISR::_from({
            const MASK: u8 = 0x03;
            const OFFSET: u8 = 12;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 14:15 - Disables Low Pass Filter."]
    #[inline]
    pub fn opa1lpfdis(&self) -> OPA1LPFDISR {
        OPA1LPFDISR::_from({
            const MASK: u8 = 0x03;
            const OFFSET: u8 = 14;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 16:17 - Disables Low Pass Filter."]
    #[inline]
    pub fn opa2lpfdis(&self) -> OPA2LPFDISR {
        OPA2LPFDISR::_from({
            const MASK: u8 = 0x03;
            const OFFSET: u8 = 16;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bit 22 - Short the non-inverting and inverting input."]
    #[inline]
    pub fn opa0short(&self) -> OPA0SHORTR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 22;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        OPA0SHORTR { bits }
    }
    #[doc = "Bit 23 - Short the non-inverting and inverting input."]
    #[inline]
    pub fn opa1short(&self) -> OPA1SHORTR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 23;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        OPA1SHORTR { bits }
    }
    #[doc = "Bit 24 - Short the non-inverting and inverting input."]
    #[inline]
    pub fn opa2short(&self) -> OPA2SHORTR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 24;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        OPA2SHORTR { bits }
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
    #[doc = "Bit 0 - OPA0 Enable"]
    #[inline]
    pub fn opa0en(&mut self) -> _OPA0ENW {
        _OPA0ENW { w: self }
    }
    #[doc = "Bit 1 - OPA1 Enable"]
    #[inline]
    pub fn opa1en(&mut self) -> _OPA1ENW {
        _OPA1ENW { w: self }
    }
    #[doc = "Bit 2 - OPA2 Enable"]
    #[inline]
    pub fn opa2en(&mut self) -> _OPA2ENW {
        _OPA2ENW { w: self }
    }
    #[doc = "Bit 6 - High Common Mode Disable."]
    #[inline]
    pub fn opa0hcmdis(&mut self) -> _OPA0HCMDISW {
        _OPA0HCMDISW { w: self }
    }
    #[doc = "Bit 7 - High Common Mode Disable."]
    #[inline]
    pub fn opa1hcmdis(&mut self) -> _OPA1HCMDISW {
        _OPA1HCMDISW { w: self }
    }
    #[doc = "Bit 8 - High Common Mode Disable."]
    #[inline]
    pub fn opa2hcmdis(&mut self) -> _OPA2HCMDISW {
        _OPA2HCMDISW { w: self }
    }
    #[doc = "Bits 12:13 - Disables Low Pass Filter."]
    #[inline]
    pub fn opa0lpfdis(&mut self) -> _OPA0LPFDISW {
        _OPA0LPFDISW { w: self }
    }
    #[doc = "Bits 14:15 - Disables Low Pass Filter."]
    #[inline]
    pub fn opa1lpfdis(&mut self) -> _OPA1LPFDISW {
        _OPA1LPFDISW { w: self }
    }
    #[doc = "Bits 16:17 - Disables Low Pass Filter."]
    #[inline]
    pub fn opa2lpfdis(&mut self) -> _OPA2LPFDISW {
        _OPA2LPFDISW { w: self }
    }
    #[doc = "Bit 22 - Short the non-inverting and inverting input."]
    #[inline]
    pub fn opa0short(&mut self) -> _OPA0SHORTW {
        _OPA0SHORTW { w: self }
    }
    #[doc = "Bit 23 - Short the non-inverting and inverting input."]
    #[inline]
    pub fn opa1short(&mut self) -> _OPA1SHORTW {
        _OPA1SHORTW { w: self }
    }
    #[doc = "Bit 24 - Short the non-inverting and inverting input."]
    #[inline]
    pub fn opa2short(&mut self) -> _OPA2SHORTW {
        _OPA2SHORTW { w: self }
    }
}

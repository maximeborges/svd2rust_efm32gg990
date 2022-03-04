#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::CTRL {
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
#[doc = "Possible values of the field `MODE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MODER {
    #[doc = "The module is disabled."]
    DISABLE,
    #[doc = "Single input LFACLK oversampling mode (available in EM0-EM2)."]
    OVSSINGLE,
    #[doc = "Externally clocked single input counter mode (available in EM0-EM3)."]
    EXTCLKSINGLE,
    #[doc = "Externally clocked quadrature decoder mode (available in EM0-EM3)."]
    EXTCLKQUAD,
}
impl MODER {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            MODER::DISABLE => 0,
            MODER::OVSSINGLE => 0x01,
            MODER::EXTCLKSINGLE => 0x02,
            MODER::EXTCLKQUAD => 0x03,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> MODER {
        match value {
            0 => MODER::DISABLE,
            1 => MODER::OVSSINGLE,
            2 => MODER::EXTCLKSINGLE,
            3 => MODER::EXTCLKQUAD,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline]
    pub fn is_disable(&self) -> bool {
        *self == MODER::DISABLE
    }
    #[doc = "Checks if the value of the field is `OVSSINGLE`"]
    #[inline]
    pub fn is_ovssingle(&self) -> bool {
        *self == MODER::OVSSINGLE
    }
    #[doc = "Checks if the value of the field is `EXTCLKSINGLE`"]
    #[inline]
    pub fn is_extclksingle(&self) -> bool {
        *self == MODER::EXTCLKSINGLE
    }
    #[doc = "Checks if the value of the field is `EXTCLKQUAD`"]
    #[inline]
    pub fn is_extclkquad(&self) -> bool {
        *self == MODER::EXTCLKQUAD
    }
}
#[doc = r" Value of the field"]
pub struct CNTDIRR {
    bits: bool,
}
impl CNTDIRR {
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
pub struct EDGER {
    bits: bool,
}
impl EDGER {
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
pub struct FILTR {
    bits: bool,
}
impl FILTR {
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
pub struct RSTENR {
    bits: bool,
}
impl RSTENR {
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
pub struct HYSTR {
    bits: bool,
}
impl HYSTR {
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
pub struct S1CDIRR {
    bits: bool,
}
impl S1CDIRR {
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
#[doc = "Possible values of the field `CNTEV`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CNTEVR {
    #[doc = "Counts up on up-count and down on down-count events."]
    BOTH,
    #[doc = "Only counts up on up-count events."]
    UP,
    #[doc = "Only counts down on down-count events."]
    DOWN,
    #[doc = "Never counts."]
    NONE,
}
impl CNTEVR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            CNTEVR::BOTH => 0,
            CNTEVR::UP => 0x01,
            CNTEVR::DOWN => 0x02,
            CNTEVR::NONE => 0x03,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> CNTEVR {
        match value {
            0 => CNTEVR::BOTH,
            1 => CNTEVR::UP,
            2 => CNTEVR::DOWN,
            3 => CNTEVR::NONE,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `BOTH`"]
    #[inline]
    pub fn is_both(&self) -> bool {
        *self == CNTEVR::BOTH
    }
    #[doc = "Checks if the value of the field is `UP`"]
    #[inline]
    pub fn is_up(&self) -> bool {
        *self == CNTEVR::UP
    }
    #[doc = "Checks if the value of the field is `DOWN`"]
    #[inline]
    pub fn is_down(&self) -> bool {
        *self == CNTEVR::DOWN
    }
    #[doc = "Checks if the value of the field is `NONE`"]
    #[inline]
    pub fn is_none(&self) -> bool {
        *self == CNTEVR::NONE
    }
}
#[doc = "Possible values of the field `AUXCNTEV`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum AUXCNTEVR {
    #[doc = "Never counts."]
    NONE,
    #[doc = "Counts up on up-count events."]
    UP,
    #[doc = "Counts up on down-count events."]
    DOWN,
    #[doc = "Counts up on both up-count and down-count events."]
    BOTH,
}
impl AUXCNTEVR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            AUXCNTEVR::NONE => 0,
            AUXCNTEVR::UP => 0x01,
            AUXCNTEVR::DOWN => 0x02,
            AUXCNTEVR::BOTH => 0x03,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> AUXCNTEVR {
        match value {
            0 => AUXCNTEVR::NONE,
            1 => AUXCNTEVR::UP,
            2 => AUXCNTEVR::DOWN,
            3 => AUXCNTEVR::BOTH,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `NONE`"]
    #[inline]
    pub fn is_none(&self) -> bool {
        *self == AUXCNTEVR::NONE
    }
    #[doc = "Checks if the value of the field is `UP`"]
    #[inline]
    pub fn is_up(&self) -> bool {
        *self == AUXCNTEVR::UP
    }
    #[doc = "Checks if the value of the field is `DOWN`"]
    #[inline]
    pub fn is_down(&self) -> bool {
        *self == AUXCNTEVR::DOWN
    }
    #[doc = "Checks if the value of the field is `BOTH`"]
    #[inline]
    pub fn is_both(&self) -> bool {
        *self == AUXCNTEVR::BOTH
    }
}
#[doc = "Values that can be written to the field `MODE`"]
pub enum MODEW {
    #[doc = "The module is disabled."]
    DISABLE,
    #[doc = "Single input LFACLK oversampling mode (available in EM0-EM2)."]
    OVSSINGLE,
    #[doc = "Externally clocked single input counter mode (available in EM0-EM3)."]
    EXTCLKSINGLE,
    #[doc = "Externally clocked quadrature decoder mode (available in EM0-EM3)."]
    EXTCLKQUAD,
}
impl MODEW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            MODEW::DISABLE => 0,
            MODEW::OVSSINGLE => 1,
            MODEW::EXTCLKSINGLE => 2,
            MODEW::EXTCLKQUAD => 3,
        }
    }
}
#[doc = r" Proxy"]
pub struct _MODEW<'a> {
    w: &'a mut W,
}
impl<'a> _MODEW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: MODEW) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "The module is disabled."]
    #[inline]
    pub fn disable(self) -> &'a mut W {
        self.variant(MODEW::DISABLE)
    }
    #[doc = "Single input LFACLK oversampling mode (available in EM0-EM2)."]
    #[inline]
    pub fn ovssingle(self) -> &'a mut W {
        self.variant(MODEW::OVSSINGLE)
    }
    #[doc = "Externally clocked single input counter mode (available in EM0-EM3)."]
    #[inline]
    pub fn extclksingle(self) -> &'a mut W {
        self.variant(MODEW::EXTCLKSINGLE)
    }
    #[doc = "Externally clocked quadrature decoder mode (available in EM0-EM3)."]
    #[inline]
    pub fn extclkquad(self) -> &'a mut W {
        self.variant(MODEW::EXTCLKQUAD)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 0x03;
        const OFFSET: u8 = 0;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _CNTDIRW<'a> {
    w: &'a mut W,
}
impl<'a> _CNTDIRW<'a> {
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
pub struct _EDGEW<'a> {
    w: &'a mut W,
}
impl<'a> _EDGEW<'a> {
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
        const OFFSET: u8 = 3;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _FILTW<'a> {
    w: &'a mut W,
}
impl<'a> _FILTW<'a> {
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
        const OFFSET: u8 = 4;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _RSTENW<'a> {
    w: &'a mut W,
}
impl<'a> _RSTENW<'a> {
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
        const OFFSET: u8 = 5;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _HYSTW<'a> {
    w: &'a mut W,
}
impl<'a> _HYSTW<'a> {
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
#[doc = r" Proxy"]
pub struct _S1CDIRW<'a> {
    w: &'a mut W,
}
impl<'a> _S1CDIRW<'a> {
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
        const OFFSET: u8 = 9;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `CNTEV`"]
pub enum CNTEVW {
    #[doc = "Counts up on up-count and down on down-count events."]
    BOTH,
    #[doc = "Only counts up on up-count events."]
    UP,
    #[doc = "Only counts down on down-count events."]
    DOWN,
    #[doc = "Never counts."]
    NONE,
}
impl CNTEVW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            CNTEVW::BOTH => 0,
            CNTEVW::UP => 1,
            CNTEVW::DOWN => 2,
            CNTEVW::NONE => 3,
        }
    }
}
#[doc = r" Proxy"]
pub struct _CNTEVW<'a> {
    w: &'a mut W,
}
impl<'a> _CNTEVW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: CNTEVW) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "Counts up on up-count and down on down-count events."]
    #[inline]
    pub fn both(self) -> &'a mut W {
        self.variant(CNTEVW::BOTH)
    }
    #[doc = "Only counts up on up-count events."]
    #[inline]
    pub fn up(self) -> &'a mut W {
        self.variant(CNTEVW::UP)
    }
    #[doc = "Only counts down on down-count events."]
    #[inline]
    pub fn down(self) -> &'a mut W {
        self.variant(CNTEVW::DOWN)
    }
    #[doc = "Never counts."]
    #[inline]
    pub fn none(self) -> &'a mut W {
        self.variant(CNTEVW::NONE)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 0x03;
        const OFFSET: u8 = 10;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `AUXCNTEV`"]
pub enum AUXCNTEVW {
    #[doc = "Never counts."]
    NONE,
    #[doc = "Counts up on up-count events."]
    UP,
    #[doc = "Counts up on down-count events."]
    DOWN,
    #[doc = "Counts up on both up-count and down-count events."]
    BOTH,
}
impl AUXCNTEVW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            AUXCNTEVW::NONE => 0,
            AUXCNTEVW::UP => 1,
            AUXCNTEVW::DOWN => 2,
            AUXCNTEVW::BOTH => 3,
        }
    }
}
#[doc = r" Proxy"]
pub struct _AUXCNTEVW<'a> {
    w: &'a mut W,
}
impl<'a> _AUXCNTEVW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: AUXCNTEVW) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "Never counts."]
    #[inline]
    pub fn none(self) -> &'a mut W {
        self.variant(AUXCNTEVW::NONE)
    }
    #[doc = "Counts up on up-count events."]
    #[inline]
    pub fn up(self) -> &'a mut W {
        self.variant(AUXCNTEVW::UP)
    }
    #[doc = "Counts up on down-count events."]
    #[inline]
    pub fn down(self) -> &'a mut W {
        self.variant(AUXCNTEVW::DOWN)
    }
    #[doc = "Counts up on both up-count and down-count events."]
    #[inline]
    pub fn both(self) -> &'a mut W {
        self.variant(AUXCNTEVW::BOTH)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 0x03;
        const OFFSET: u8 = 14;
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
    #[doc = "Bits 0:1 - Mode Select"]
    #[inline]
    pub fn mode(&self) -> MODER {
        MODER::_from({
            const MASK: u8 = 0x03;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bit 2 - Non-Quadrature Mode Counter Direction Control"]
    #[inline]
    pub fn cntdir(&self) -> CNTDIRR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 2;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        CNTDIRR { bits }
    }
    #[doc = "Bit 3 - Edge Select"]
    #[inline]
    pub fn edge(&self) -> EDGER {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 3;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        EDGER { bits }
    }
    #[doc = "Bit 4 - Enable Digital Pulse Width Filter"]
    #[inline]
    pub fn filt(&self) -> FILTR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 4;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        FILTR { bits }
    }
    #[doc = "Bit 5 - Enable PCNT Clock Domain Reset"]
    #[inline]
    pub fn rsten(&self) -> RSTENR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 5;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        RSTENR { bits }
    }
    #[doc = "Bit 8 - Enable Hysteresis"]
    #[inline]
    pub fn hyst(&self) -> HYSTR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 8;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        HYSTR { bits }
    }
    #[doc = "Bit 9 - Count direction determined by S1"]
    #[inline]
    pub fn s1cdir(&self) -> S1CDIRR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 9;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        S1CDIRR { bits }
    }
    #[doc = "Bits 10:11 - Controls when the counter counts"]
    #[inline]
    pub fn cntev(&self) -> CNTEVR {
        CNTEVR::_from({
            const MASK: u8 = 0x03;
            const OFFSET: u8 = 10;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 14:15 - Controls when the auxiliary counter counts"]
    #[inline]
    pub fn auxcntev(&self) -> AUXCNTEVR {
        AUXCNTEVR::_from({
            const MASK: u8 = 0x03;
            const OFFSET: u8 = 14;
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
    #[doc = "Bits 0:1 - Mode Select"]
    #[inline]
    pub fn mode(&mut self) -> _MODEW {
        _MODEW { w: self }
    }
    #[doc = "Bit 2 - Non-Quadrature Mode Counter Direction Control"]
    #[inline]
    pub fn cntdir(&mut self) -> _CNTDIRW {
        _CNTDIRW { w: self }
    }
    #[doc = "Bit 3 - Edge Select"]
    #[inline]
    pub fn edge(&mut self) -> _EDGEW {
        _EDGEW { w: self }
    }
    #[doc = "Bit 4 - Enable Digital Pulse Width Filter"]
    #[inline]
    pub fn filt(&mut self) -> _FILTW {
        _FILTW { w: self }
    }
    #[doc = "Bit 5 - Enable PCNT Clock Domain Reset"]
    #[inline]
    pub fn rsten(&mut self) -> _RSTENW {
        _RSTENW { w: self }
    }
    #[doc = "Bit 8 - Enable Hysteresis"]
    #[inline]
    pub fn hyst(&mut self) -> _HYSTW {
        _HYSTW { w: self }
    }
    #[doc = "Bit 9 - Count direction determined by S1"]
    #[inline]
    pub fn s1cdir(&mut self) -> _S1CDIRW {
        _S1CDIRW { w: self }
    }
    #[doc = "Bits 10:11 - Controls when the counter counts"]
    #[inline]
    pub fn cntev(&mut self) -> _CNTEVW {
        _CNTEVW { w: self }
    }
    #[doc = "Bits 14:15 - Controls when the auxiliary counter counts"]
    #[inline]
    pub fn auxcntev(&mut self) -> _AUXCNTEVW {
        _AUXCNTEVW { w: self }
    }
}

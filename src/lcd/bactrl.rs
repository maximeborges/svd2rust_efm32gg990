#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::BACTRL {
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
pub struct BLINKENR {
    bits: bool,
}
impl BLINKENR {
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
pub struct BLANKR {
    bits: bool,
}
impl BLANKR {
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
pub struct AENR {
    bits: bool,
}
impl AENR {
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
#[doc = "Possible values of the field `AREGASC`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum AREGASCR {
    #[doc = "No Shift operation on Animation Register A"]
    NOSHIFT,
    #[doc = "Animation Register A is shifted left"]
    SHIFTLEFT,
    #[doc = "Animation Register A is shifted right"]
    SHIFTRIGHT,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl AREGASCR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            AREGASCR::NOSHIFT => 0,
            AREGASCR::SHIFTLEFT => 0x01,
            AREGASCR::SHIFTRIGHT => 0x02,
            AREGASCR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> AREGASCR {
        match value {
            0 => AREGASCR::NOSHIFT,
            1 => AREGASCR::SHIFTLEFT,
            2 => AREGASCR::SHIFTRIGHT,
            i => AREGASCR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `NOSHIFT`"]
    #[inline]
    pub fn is_noshift(&self) -> bool {
        *self == AREGASCR::NOSHIFT
    }
    #[doc = "Checks if the value of the field is `SHIFTLEFT`"]
    #[inline]
    pub fn is_shiftleft(&self) -> bool {
        *self == AREGASCR::SHIFTLEFT
    }
    #[doc = "Checks if the value of the field is `SHIFTRIGHT`"]
    #[inline]
    pub fn is_shiftright(&self) -> bool {
        *self == AREGASCR::SHIFTRIGHT
    }
}
#[doc = "Possible values of the field `AREGBSC`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum AREGBSCR {
    #[doc = "No Shift operation on Animation Register B"]
    NOSHIFT,
    #[doc = "Animation Register B is shifted left"]
    SHIFTLEFT,
    #[doc = "Animation Register B is shifted right"]
    SHIFTRIGHT,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl AREGBSCR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            AREGBSCR::NOSHIFT => 0,
            AREGBSCR::SHIFTLEFT => 0x01,
            AREGBSCR::SHIFTRIGHT => 0x02,
            AREGBSCR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> AREGBSCR {
        match value {
            0 => AREGBSCR::NOSHIFT,
            1 => AREGBSCR::SHIFTLEFT,
            2 => AREGBSCR::SHIFTRIGHT,
            i => AREGBSCR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `NOSHIFT`"]
    #[inline]
    pub fn is_noshift(&self) -> bool {
        *self == AREGBSCR::NOSHIFT
    }
    #[doc = "Checks if the value of the field is `SHIFTLEFT`"]
    #[inline]
    pub fn is_shiftleft(&self) -> bool {
        *self == AREGBSCR::SHIFTLEFT
    }
    #[doc = "Checks if the value of the field is `SHIFTRIGHT`"]
    #[inline]
    pub fn is_shiftright(&self) -> bool {
        *self == AREGBSCR::SHIFTRIGHT
    }
}
#[doc = r" Value of the field"]
pub struct ALOGSELR {
    bits: bool,
}
impl ALOGSELR {
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
pub struct FCENR {
    bits: bool,
}
impl FCENR {
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
#[doc = "Possible values of the field `FCPRESC`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FCPRESCR {
    #[doc = "CLKFC = CLKFRAME / 1"]
    DIV1,
    #[doc = "CLKFC = CLKFRAME / 2"]
    DIV2,
    #[doc = "CLKFC = CLKFRAME / 4"]
    DIV4,
    #[doc = "CLKFC = CLKFRAME / 8"]
    DIV8,
}
impl FCPRESCR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            FCPRESCR::DIV1 => 0,
            FCPRESCR::DIV2 => 0x01,
            FCPRESCR::DIV4 => 0x02,
            FCPRESCR::DIV8 => 0x03,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> FCPRESCR {
        match value {
            0 => FCPRESCR::DIV1,
            1 => FCPRESCR::DIV2,
            2 => FCPRESCR::DIV4,
            3 => FCPRESCR::DIV8,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `DIV1`"]
    #[inline]
    pub fn is_div1(&self) -> bool {
        *self == FCPRESCR::DIV1
    }
    #[doc = "Checks if the value of the field is `DIV2`"]
    #[inline]
    pub fn is_div2(&self) -> bool {
        *self == FCPRESCR::DIV2
    }
    #[doc = "Checks if the value of the field is `DIV4`"]
    #[inline]
    pub fn is_div4(&self) -> bool {
        *self == FCPRESCR::DIV4
    }
    #[doc = "Checks if the value of the field is `DIV8`"]
    #[inline]
    pub fn is_div8(&self) -> bool {
        *self == FCPRESCR::DIV8
    }
}
#[doc = r" Value of the field"]
pub struct FCTOPR {
    bits: u8,
}
impl FCTOPR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct ALOCR {
    bits: bool,
}
impl ALOCR {
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
pub struct _BLINKENW<'a> {
    w: &'a mut W,
}
impl<'a> _BLINKENW<'a> {
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
pub struct _BLANKW<'a> {
    w: &'a mut W,
}
impl<'a> _BLANKW<'a> {
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
pub struct _AENW<'a> {
    w: &'a mut W,
}
impl<'a> _AENW<'a> {
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
#[doc = "Values that can be written to the field `AREGASC`"]
pub enum AREGASCW {
    #[doc = "No Shift operation on Animation Register A"]
    NOSHIFT,
    #[doc = "Animation Register A is shifted left"]
    SHIFTLEFT,
    #[doc = "Animation Register A is shifted right"]
    SHIFTRIGHT,
}
impl AREGASCW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            AREGASCW::NOSHIFT => 0,
            AREGASCW::SHIFTLEFT => 1,
            AREGASCW::SHIFTRIGHT => 2,
        }
    }
}
#[doc = r" Proxy"]
pub struct _AREGASCW<'a> {
    w: &'a mut W,
}
impl<'a> _AREGASCW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: AREGASCW) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "No Shift operation on Animation Register A"]
    #[inline]
    pub fn noshift(self) -> &'a mut W {
        self.variant(AREGASCW::NOSHIFT)
    }
    #[doc = "Animation Register A is shifted left"]
    #[inline]
    pub fn shiftleft(self) -> &'a mut W {
        self.variant(AREGASCW::SHIFTLEFT)
    }
    #[doc = "Animation Register A is shifted right"]
    #[inline]
    pub fn shiftright(self) -> &'a mut W {
        self.variant(AREGASCW::SHIFTRIGHT)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 0x03;
        const OFFSET: u8 = 3;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `AREGBSC`"]
pub enum AREGBSCW {
    #[doc = "No Shift operation on Animation Register B"]
    NOSHIFT,
    #[doc = "Animation Register B is shifted left"]
    SHIFTLEFT,
    #[doc = "Animation Register B is shifted right"]
    SHIFTRIGHT,
}
impl AREGBSCW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            AREGBSCW::NOSHIFT => 0,
            AREGBSCW::SHIFTLEFT => 1,
            AREGBSCW::SHIFTRIGHT => 2,
        }
    }
}
#[doc = r" Proxy"]
pub struct _AREGBSCW<'a> {
    w: &'a mut W,
}
impl<'a> _AREGBSCW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: AREGBSCW) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "No Shift operation on Animation Register B"]
    #[inline]
    pub fn noshift(self) -> &'a mut W {
        self.variant(AREGBSCW::NOSHIFT)
    }
    #[doc = "Animation Register B is shifted left"]
    #[inline]
    pub fn shiftleft(self) -> &'a mut W {
        self.variant(AREGBSCW::SHIFTLEFT)
    }
    #[doc = "Animation Register B is shifted right"]
    #[inline]
    pub fn shiftright(self) -> &'a mut W {
        self.variant(AREGBSCW::SHIFTRIGHT)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 0x03;
        const OFFSET: u8 = 5;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _ALOGSELW<'a> {
    w: &'a mut W,
}
impl<'a> _ALOGSELW<'a> {
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
pub struct _FCENW<'a> {
    w: &'a mut W,
}
impl<'a> _FCENW<'a> {
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
#[doc = "Values that can be written to the field `FCPRESC`"]
pub enum FCPRESCW {
    #[doc = "CLKFC = CLKFRAME / 1"]
    DIV1,
    #[doc = "CLKFC = CLKFRAME / 2"]
    DIV2,
    #[doc = "CLKFC = CLKFRAME / 4"]
    DIV4,
    #[doc = "CLKFC = CLKFRAME / 8"]
    DIV8,
}
impl FCPRESCW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            FCPRESCW::DIV1 => 0,
            FCPRESCW::DIV2 => 1,
            FCPRESCW::DIV4 => 2,
            FCPRESCW::DIV8 => 3,
        }
    }
}
#[doc = r" Proxy"]
pub struct _FCPRESCW<'a> {
    w: &'a mut W,
}
impl<'a> _FCPRESCW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: FCPRESCW) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "CLKFC = CLKFRAME / 1"]
    #[inline]
    pub fn div1(self) -> &'a mut W {
        self.variant(FCPRESCW::DIV1)
    }
    #[doc = "CLKFC = CLKFRAME / 2"]
    #[inline]
    pub fn div2(self) -> &'a mut W {
        self.variant(FCPRESCW::DIV2)
    }
    #[doc = "CLKFC = CLKFRAME / 4"]
    #[inline]
    pub fn div4(self) -> &'a mut W {
        self.variant(FCPRESCW::DIV4)
    }
    #[doc = "CLKFC = CLKFRAME / 8"]
    #[inline]
    pub fn div8(self) -> &'a mut W {
        self.variant(FCPRESCW::DIV8)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 0x03;
        const OFFSET: u8 = 16;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _FCTOPW<'a> {
    w: &'a mut W,
}
impl<'a> _FCTOPW<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 0x3f;
        const OFFSET: u8 = 18;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _ALOCW<'a> {
    w: &'a mut W,
}
impl<'a> _ALOCW<'a> {
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
    #[doc = "Bit 0 - Blink Enable"]
    #[inline]
    pub fn blinken(&self) -> BLINKENR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        BLINKENR { bits }
    }
    #[doc = "Bit 1 - Blank Display"]
    #[inline]
    pub fn blank(&self) -> BLANKR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 1;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        BLANKR { bits }
    }
    #[doc = "Bit 2 - Animation Enable"]
    #[inline]
    pub fn aen(&self) -> AENR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 2;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        AENR { bits }
    }
    #[doc = "Bits 3:4 - Animate Register A Shift Control"]
    #[inline]
    pub fn aregasc(&self) -> AREGASCR {
        AREGASCR::_from({
            const MASK: u8 = 0x03;
            const OFFSET: u8 = 3;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 5:6 - Animate Register B Shift Control"]
    #[inline]
    pub fn aregbsc(&self) -> AREGBSCR {
        AREGBSCR::_from({
            const MASK: u8 = 0x03;
            const OFFSET: u8 = 5;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bit 7 - Animate Logic Function Select"]
    #[inline]
    pub fn alogsel(&self) -> ALOGSELR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 7;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        ALOGSELR { bits }
    }
    #[doc = "Bit 8 - Frame Counter Enable"]
    #[inline]
    pub fn fcen(&self) -> FCENR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 8;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        FCENR { bits }
    }
    #[doc = "Bits 16:17 - Frame Counter Prescaler"]
    #[inline]
    pub fn fcpresc(&self) -> FCPRESCR {
        FCPRESCR::_from({
            const MASK: u8 = 0x03;
            const OFFSET: u8 = 16;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 18:23 - Frame Counter Top Value"]
    #[inline]
    pub fn fctop(&self) -> FCTOPR {
        let bits = {
            const MASK: u8 = 0x3f;
            const OFFSET: u8 = 18;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        FCTOPR { bits }
    }
    #[doc = "Bit 28 - Animation Location"]
    #[inline]
    pub fn aloc(&self) -> ALOCR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 28;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        ALOCR { bits }
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
    #[doc = "Bit 0 - Blink Enable"]
    #[inline]
    pub fn blinken(&mut self) -> _BLINKENW {
        _BLINKENW { w: self }
    }
    #[doc = "Bit 1 - Blank Display"]
    #[inline]
    pub fn blank(&mut self) -> _BLANKW {
        _BLANKW { w: self }
    }
    #[doc = "Bit 2 - Animation Enable"]
    #[inline]
    pub fn aen(&mut self) -> _AENW {
        _AENW { w: self }
    }
    #[doc = "Bits 3:4 - Animate Register A Shift Control"]
    #[inline]
    pub fn aregasc(&mut self) -> _AREGASCW {
        _AREGASCW { w: self }
    }
    #[doc = "Bits 5:6 - Animate Register B Shift Control"]
    #[inline]
    pub fn aregbsc(&mut self) -> _AREGBSCW {
        _AREGBSCW { w: self }
    }
    #[doc = "Bit 7 - Animate Logic Function Select"]
    #[inline]
    pub fn alogsel(&mut self) -> _ALOGSELW {
        _ALOGSELW { w: self }
    }
    #[doc = "Bit 8 - Frame Counter Enable"]
    #[inline]
    pub fn fcen(&mut self) -> _FCENW {
        _FCENW { w: self }
    }
    #[doc = "Bits 16:17 - Frame Counter Prescaler"]
    #[inline]
    pub fn fcpresc(&mut self) -> _FCPRESCW {
        _FCPRESCW { w: self }
    }
    #[doc = "Bits 18:23 - Frame Counter Top Value"]
    #[inline]
    pub fn fctop(&mut self) -> _FCTOPW {
        _FCTOPW { w: self }
    }
    #[doc = "Bit 28 - Animation Location"]
    #[inline]
    pub fn aloc(&mut self) -> _ALOCW {
        _ALOCW { w: self }
    }
}

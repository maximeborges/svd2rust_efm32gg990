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
#[doc = "Possible values of the field `REPMODE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum REPMODER {
    #[doc = "When started, the LETIMER counts down until it is stopped by software."]
    FREE,
    #[doc = "The counter counts REP0 times. When REP0 reaches zero, the counter stops."]
    ONESHOT,
    #[doc = "The counter counts REP0 times. If REP1 has been written, it is loaded into REP0 when REP0 reaches zero. Else the counter stops"]
    BUFFERED,
    #[doc = "Both REP0 and REP1 are decremented when the LETIMER wraps around. The LETIMER counts until both REP0 and REP1 are zero"]
    DOUBLE,
}
impl REPMODER {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            REPMODER::FREE => 0,
            REPMODER::ONESHOT => 0x01,
            REPMODER::BUFFERED => 0x02,
            REPMODER::DOUBLE => 0x03,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> REPMODER {
        match value {
            0 => REPMODER::FREE,
            1 => REPMODER::ONESHOT,
            2 => REPMODER::BUFFERED,
            3 => REPMODER::DOUBLE,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `FREE`"]
    #[inline]
    pub fn is_free(&self) -> bool {
        *self == REPMODER::FREE
    }
    #[doc = "Checks if the value of the field is `ONESHOT`"]
    #[inline]
    pub fn is_oneshot(&self) -> bool {
        *self == REPMODER::ONESHOT
    }
    #[doc = "Checks if the value of the field is `BUFFERED`"]
    #[inline]
    pub fn is_buffered(&self) -> bool {
        *self == REPMODER::BUFFERED
    }
    #[doc = "Checks if the value of the field is `DOUBLE`"]
    #[inline]
    pub fn is_double(&self) -> bool {
        *self == REPMODER::DOUBLE
    }
}
#[doc = "Possible values of the field `UFOA0`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum UFOA0R {
    #[doc = "LETn_O0 is held at its idle value as defined by OPOL0."]
    NONE,
    #[doc = "LETn_O0 is toggled on CNT underflow."]
    TOGGLE,
    #[doc = "LETn_O0 is held active for one LFACLKLETIMER0 clock cycle on CNT underflow. The output then returns to its idle value as defined by OPOL0."]
    PULSE,
    #[doc = "LETn_O0 is set idle on CNT underflow, and active on compare match with COMP1"]
    PWM,
}
impl UFOA0R {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            UFOA0R::NONE => 0,
            UFOA0R::TOGGLE => 0x01,
            UFOA0R::PULSE => 0x02,
            UFOA0R::PWM => 0x03,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> UFOA0R {
        match value {
            0 => UFOA0R::NONE,
            1 => UFOA0R::TOGGLE,
            2 => UFOA0R::PULSE,
            3 => UFOA0R::PWM,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `NONE`"]
    #[inline]
    pub fn is_none(&self) -> bool {
        *self == UFOA0R::NONE
    }
    #[doc = "Checks if the value of the field is `TOGGLE`"]
    #[inline]
    pub fn is_toggle(&self) -> bool {
        *self == UFOA0R::TOGGLE
    }
    #[doc = "Checks if the value of the field is `PULSE`"]
    #[inline]
    pub fn is_pulse(&self) -> bool {
        *self == UFOA0R::PULSE
    }
    #[doc = "Checks if the value of the field is `PWM`"]
    #[inline]
    pub fn is_pwm(&self) -> bool {
        *self == UFOA0R::PWM
    }
}
#[doc = "Possible values of the field `UFOA1`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum UFOA1R {
    #[doc = "LETn_O1 is held at its idle value as defined by OPOL1."]
    NONE,
    #[doc = "LETn_O1 is toggled on CNT underflow."]
    TOGGLE,
    #[doc = "LETn_O1 is held active for one LFACLKLETIMER0 clock cycle on CNT underflow. The output then returns to its idle value as defined by OPOL1."]
    PULSE,
    #[doc = "LETn_O1 is set idle on CNT underflow, and active on compare match with COMP1"]
    PWM,
}
impl UFOA1R {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            UFOA1R::NONE => 0,
            UFOA1R::TOGGLE => 0x01,
            UFOA1R::PULSE => 0x02,
            UFOA1R::PWM => 0x03,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> UFOA1R {
        match value {
            0 => UFOA1R::NONE,
            1 => UFOA1R::TOGGLE,
            2 => UFOA1R::PULSE,
            3 => UFOA1R::PWM,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `NONE`"]
    #[inline]
    pub fn is_none(&self) -> bool {
        *self == UFOA1R::NONE
    }
    #[doc = "Checks if the value of the field is `TOGGLE`"]
    #[inline]
    pub fn is_toggle(&self) -> bool {
        *self == UFOA1R::TOGGLE
    }
    #[doc = "Checks if the value of the field is `PULSE`"]
    #[inline]
    pub fn is_pulse(&self) -> bool {
        *self == UFOA1R::PULSE
    }
    #[doc = "Checks if the value of the field is `PWM`"]
    #[inline]
    pub fn is_pwm(&self) -> bool {
        *self == UFOA1R::PWM
    }
}
#[doc = r" Value of the field"]
pub struct OPOL0R {
    bits: bool,
}
impl OPOL0R {
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
pub struct OPOL1R {
    bits: bool,
}
impl OPOL1R {
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
pub struct BUFTOPR {
    bits: bool,
}
impl BUFTOPR {
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
pub struct COMP0TOPR {
    bits: bool,
}
impl COMP0TOPR {
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
pub struct RTCC0TENR {
    bits: bool,
}
impl RTCC0TENR {
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
pub struct RTCC1TENR {
    bits: bool,
}
impl RTCC1TENR {
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
pub struct DEBUGRUNR {
    bits: bool,
}
impl DEBUGRUNR {
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
#[doc = "Values that can be written to the field `REPMODE`"]
pub enum REPMODEW {
    #[doc = "When started, the LETIMER counts down until it is stopped by software."]
    FREE,
    #[doc = "The counter counts REP0 times. When REP0 reaches zero, the counter stops."]
    ONESHOT,
    #[doc = "The counter counts REP0 times. If REP1 has been written, it is loaded into REP0 when REP0 reaches zero. Else the counter stops"]
    BUFFERED,
    #[doc = "Both REP0 and REP1 are decremented when the LETIMER wraps around. The LETIMER counts until both REP0 and REP1 are zero"]
    DOUBLE,
}
impl REPMODEW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            REPMODEW::FREE => 0,
            REPMODEW::ONESHOT => 1,
            REPMODEW::BUFFERED => 2,
            REPMODEW::DOUBLE => 3,
        }
    }
}
#[doc = r" Proxy"]
pub struct _REPMODEW<'a> {
    w: &'a mut W,
}
impl<'a> _REPMODEW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: REPMODEW) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "When started, the LETIMER counts down until it is stopped by software."]
    #[inline]
    pub fn free(self) -> &'a mut W {
        self.variant(REPMODEW::FREE)
    }
    #[doc = "The counter counts REP0 times. When REP0 reaches zero, the counter stops."]
    #[inline]
    pub fn oneshot(self) -> &'a mut W {
        self.variant(REPMODEW::ONESHOT)
    }
    #[doc = "The counter counts REP0 times. If REP1 has been written, it is loaded into REP0 when REP0 reaches zero. Else the counter stops"]
    #[inline]
    pub fn buffered(self) -> &'a mut W {
        self.variant(REPMODEW::BUFFERED)
    }
    #[doc = "Both REP0 and REP1 are decremented when the LETIMER wraps around. The LETIMER counts until both REP0 and REP1 are zero"]
    #[inline]
    pub fn double(self) -> &'a mut W {
        self.variant(REPMODEW::DOUBLE)
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
#[doc = "Values that can be written to the field `UFOA0`"]
pub enum UFOA0W {
    #[doc = "LETn_O0 is held at its idle value as defined by OPOL0."]
    NONE,
    #[doc = "LETn_O0 is toggled on CNT underflow."]
    TOGGLE,
    #[doc = "LETn_O0 is held active for one LFACLKLETIMER0 clock cycle on CNT underflow. The output then returns to its idle value as defined by OPOL0."]
    PULSE,
    #[doc = "LETn_O0 is set idle on CNT underflow, and active on compare match with COMP1"]
    PWM,
}
impl UFOA0W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            UFOA0W::NONE => 0,
            UFOA0W::TOGGLE => 1,
            UFOA0W::PULSE => 2,
            UFOA0W::PWM => 3,
        }
    }
}
#[doc = r" Proxy"]
pub struct _UFOA0W<'a> {
    w: &'a mut W,
}
impl<'a> _UFOA0W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: UFOA0W) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "LETn_O0 is held at its idle value as defined by OPOL0."]
    #[inline]
    pub fn none(self) -> &'a mut W {
        self.variant(UFOA0W::NONE)
    }
    #[doc = "LETn_O0 is toggled on CNT underflow."]
    #[inline]
    pub fn toggle(self) -> &'a mut W {
        self.variant(UFOA0W::TOGGLE)
    }
    #[doc = "LETn_O0 is held active for one LFACLKLETIMER0 clock cycle on CNT underflow. The output then returns to its idle value as defined by OPOL0."]
    #[inline]
    pub fn pulse(self) -> &'a mut W {
        self.variant(UFOA0W::PULSE)
    }
    #[doc = "LETn_O0 is set idle on CNT underflow, and active on compare match with COMP1"]
    #[inline]
    pub fn pwm(self) -> &'a mut W {
        self.variant(UFOA0W::PWM)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 0x03;
        const OFFSET: u8 = 2;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `UFOA1`"]
pub enum UFOA1W {
    #[doc = "LETn_O1 is held at its idle value as defined by OPOL1."]
    NONE,
    #[doc = "LETn_O1 is toggled on CNT underflow."]
    TOGGLE,
    #[doc = "LETn_O1 is held active for one LFACLKLETIMER0 clock cycle on CNT underflow. The output then returns to its idle value as defined by OPOL1."]
    PULSE,
    #[doc = "LETn_O1 is set idle on CNT underflow, and active on compare match with COMP1"]
    PWM,
}
impl UFOA1W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            UFOA1W::NONE => 0,
            UFOA1W::TOGGLE => 1,
            UFOA1W::PULSE => 2,
            UFOA1W::PWM => 3,
        }
    }
}
#[doc = r" Proxy"]
pub struct _UFOA1W<'a> {
    w: &'a mut W,
}
impl<'a> _UFOA1W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: UFOA1W) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "LETn_O1 is held at its idle value as defined by OPOL1."]
    #[inline]
    pub fn none(self) -> &'a mut W {
        self.variant(UFOA1W::NONE)
    }
    #[doc = "LETn_O1 is toggled on CNT underflow."]
    #[inline]
    pub fn toggle(self) -> &'a mut W {
        self.variant(UFOA1W::TOGGLE)
    }
    #[doc = "LETn_O1 is held active for one LFACLKLETIMER0 clock cycle on CNT underflow. The output then returns to its idle value as defined by OPOL1."]
    #[inline]
    pub fn pulse(self) -> &'a mut W {
        self.variant(UFOA1W::PULSE)
    }
    #[doc = "LETn_O1 is set idle on CNT underflow, and active on compare match with COMP1"]
    #[inline]
    pub fn pwm(self) -> &'a mut W {
        self.variant(UFOA1W::PWM)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 0x03;
        const OFFSET: u8 = 4;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _OPOL0W<'a> {
    w: &'a mut W,
}
impl<'a> _OPOL0W<'a> {
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
pub struct _OPOL1W<'a> {
    w: &'a mut W,
}
impl<'a> _OPOL1W<'a> {
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
pub struct _BUFTOPW<'a> {
    w: &'a mut W,
}
impl<'a> _BUFTOPW<'a> {
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
pub struct _COMP0TOPW<'a> {
    w: &'a mut W,
}
impl<'a> _COMP0TOPW<'a> {
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
#[doc = r" Proxy"]
pub struct _RTCC0TENW<'a> {
    w: &'a mut W,
}
impl<'a> _RTCC0TENW<'a> {
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
        const OFFSET: u8 = 10;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _RTCC1TENW<'a> {
    w: &'a mut W,
}
impl<'a> _RTCC1TENW<'a> {
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
        const OFFSET: u8 = 11;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _DEBUGRUNW<'a> {
    w: &'a mut W,
}
impl<'a> _DEBUGRUNW<'a> {
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
        const OFFSET: u8 = 12;
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
    #[doc = "Bits 0:1 - Repeat Mode"]
    #[inline]
    pub fn repmode(&self) -> REPMODER {
        REPMODER::_from({
            const MASK: u8 = 0x03;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 2:3 - Underflow Output Action 0"]
    #[inline]
    pub fn ufoa0(&self) -> UFOA0R {
        UFOA0R::_from({
            const MASK: u8 = 0x03;
            const OFFSET: u8 = 2;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 4:5 - Underflow Output Action 1"]
    #[inline]
    pub fn ufoa1(&self) -> UFOA1R {
        UFOA1R::_from({
            const MASK: u8 = 0x03;
            const OFFSET: u8 = 4;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bit 6 - Output 0 Polarity"]
    #[inline]
    pub fn opol0(&self) -> OPOL0R {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 6;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        OPOL0R { bits }
    }
    #[doc = "Bit 7 - Output 1 Polarity"]
    #[inline]
    pub fn opol1(&self) -> OPOL1R {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 7;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        OPOL1R { bits }
    }
    #[doc = "Bit 8 - Buffered Top"]
    #[inline]
    pub fn buftop(&self) -> BUFTOPR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 8;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        BUFTOPR { bits }
    }
    #[doc = "Bit 9 - Compare Value 0 Is Top Value"]
    #[inline]
    pub fn comp0top(&self) -> COMP0TOPR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 9;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        COMP0TOPR { bits }
    }
    #[doc = "Bit 10 - RTC Compare 0 Trigger Enable"]
    #[inline]
    pub fn rtcc0ten(&self) -> RTCC0TENR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 10;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        RTCC0TENR { bits }
    }
    #[doc = "Bit 11 - RTC Compare 1 Trigger Enable"]
    #[inline]
    pub fn rtcc1ten(&self) -> RTCC1TENR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 11;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        RTCC1TENR { bits }
    }
    #[doc = "Bit 12 - Debug Mode Run Enable"]
    #[inline]
    pub fn debugrun(&self) -> DEBUGRUNR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 12;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        DEBUGRUNR { bits }
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
    #[doc = "Bits 0:1 - Repeat Mode"]
    #[inline]
    pub fn repmode(&mut self) -> _REPMODEW {
        _REPMODEW { w: self }
    }
    #[doc = "Bits 2:3 - Underflow Output Action 0"]
    #[inline]
    pub fn ufoa0(&mut self) -> _UFOA0W {
        _UFOA0W { w: self }
    }
    #[doc = "Bits 4:5 - Underflow Output Action 1"]
    #[inline]
    pub fn ufoa1(&mut self) -> _UFOA1W {
        _UFOA1W { w: self }
    }
    #[doc = "Bit 6 - Output 0 Polarity"]
    #[inline]
    pub fn opol0(&mut self) -> _OPOL0W {
        _OPOL0W { w: self }
    }
    #[doc = "Bit 7 - Output 1 Polarity"]
    #[inline]
    pub fn opol1(&mut self) -> _OPOL1W {
        _OPOL1W { w: self }
    }
    #[doc = "Bit 8 - Buffered Top"]
    #[inline]
    pub fn buftop(&mut self) -> _BUFTOPW {
        _BUFTOPW { w: self }
    }
    #[doc = "Bit 9 - Compare Value 0 Is Top Value"]
    #[inline]
    pub fn comp0top(&mut self) -> _COMP0TOPW {
        _COMP0TOPW { w: self }
    }
    #[doc = "Bit 10 - RTC Compare 0 Trigger Enable"]
    #[inline]
    pub fn rtcc0ten(&mut self) -> _RTCC0TENW {
        _RTCC0TENW { w: self }
    }
    #[doc = "Bit 11 - RTC Compare 1 Trigger Enable"]
    #[inline]
    pub fn rtcc1ten(&mut self) -> _RTCC1TENW {
        _RTCC1TENW { w: self }
    }
    #[doc = "Bit 12 - Debug Mode Run Enable"]
    #[inline]
    pub fn debugrun(&mut self) -> _DEBUGRUNW {
        _DEBUGRUNW { w: self }
    }
}

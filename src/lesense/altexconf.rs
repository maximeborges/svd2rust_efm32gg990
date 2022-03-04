#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::ALTEXCONF {
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
#[doc = "Possible values of the field `IDLECONF0`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum IDLECONF0R {
    #[doc = "ALTEX0 output is disabled in idle phase"]
    DISABLE,
    #[doc = "ALTEX0 output is high in idle phase"]
    HIGH,
    #[doc = "ALTEX0 output is low in idle phase"]
    LOW,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl IDLECONF0R {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            IDLECONF0R::DISABLE => 0,
            IDLECONF0R::HIGH => 0x01,
            IDLECONF0R::LOW => 0x02,
            IDLECONF0R::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> IDLECONF0R {
        match value {
            0 => IDLECONF0R::DISABLE,
            1 => IDLECONF0R::HIGH,
            2 => IDLECONF0R::LOW,
            i => IDLECONF0R::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline]
    pub fn is_disable(&self) -> bool {
        *self == IDLECONF0R::DISABLE
    }
    #[doc = "Checks if the value of the field is `HIGH`"]
    #[inline]
    pub fn is_high(&self) -> bool {
        *self == IDLECONF0R::HIGH
    }
    #[doc = "Checks if the value of the field is `LOW`"]
    #[inline]
    pub fn is_low(&self) -> bool {
        *self == IDLECONF0R::LOW
    }
}
#[doc = "Possible values of the field `IDLECONF1`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum IDLECONF1R {
    #[doc = "ALTEX1 output is disabled in idle phase"]
    DISABLE,
    #[doc = "ALTEX1 output is high in idle phase"]
    HIGH,
    #[doc = "ALTEX1 output is low in idle phase"]
    LOW,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl IDLECONF1R {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            IDLECONF1R::DISABLE => 0,
            IDLECONF1R::HIGH => 0x01,
            IDLECONF1R::LOW => 0x02,
            IDLECONF1R::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> IDLECONF1R {
        match value {
            0 => IDLECONF1R::DISABLE,
            1 => IDLECONF1R::HIGH,
            2 => IDLECONF1R::LOW,
            i => IDLECONF1R::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline]
    pub fn is_disable(&self) -> bool {
        *self == IDLECONF1R::DISABLE
    }
    #[doc = "Checks if the value of the field is `HIGH`"]
    #[inline]
    pub fn is_high(&self) -> bool {
        *self == IDLECONF1R::HIGH
    }
    #[doc = "Checks if the value of the field is `LOW`"]
    #[inline]
    pub fn is_low(&self) -> bool {
        *self == IDLECONF1R::LOW
    }
}
#[doc = "Possible values of the field `IDLECONF2`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum IDLECONF2R {
    #[doc = "ALTEX2 output is disabled in idle phase"]
    DISABLE,
    #[doc = "ALTEX2 output is high in idle phase"]
    HIGH,
    #[doc = "ALTEX2 output is low in idle phase"]
    LOW,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl IDLECONF2R {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            IDLECONF2R::DISABLE => 0,
            IDLECONF2R::HIGH => 0x01,
            IDLECONF2R::LOW => 0x02,
            IDLECONF2R::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> IDLECONF2R {
        match value {
            0 => IDLECONF2R::DISABLE,
            1 => IDLECONF2R::HIGH,
            2 => IDLECONF2R::LOW,
            i => IDLECONF2R::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline]
    pub fn is_disable(&self) -> bool {
        *self == IDLECONF2R::DISABLE
    }
    #[doc = "Checks if the value of the field is `HIGH`"]
    #[inline]
    pub fn is_high(&self) -> bool {
        *self == IDLECONF2R::HIGH
    }
    #[doc = "Checks if the value of the field is `LOW`"]
    #[inline]
    pub fn is_low(&self) -> bool {
        *self == IDLECONF2R::LOW
    }
}
#[doc = "Possible values of the field `IDLECONF3`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum IDLECONF3R {
    #[doc = "ALTEX3 output is disabled in idle phase"]
    DISABLE,
    #[doc = "ALTEX3 output is high in idle phase"]
    HIGH,
    #[doc = "ALTEX3 output is low in idle phase"]
    LOW,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl IDLECONF3R {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            IDLECONF3R::DISABLE => 0,
            IDLECONF3R::HIGH => 0x01,
            IDLECONF3R::LOW => 0x02,
            IDLECONF3R::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> IDLECONF3R {
        match value {
            0 => IDLECONF3R::DISABLE,
            1 => IDLECONF3R::HIGH,
            2 => IDLECONF3R::LOW,
            i => IDLECONF3R::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline]
    pub fn is_disable(&self) -> bool {
        *self == IDLECONF3R::DISABLE
    }
    #[doc = "Checks if the value of the field is `HIGH`"]
    #[inline]
    pub fn is_high(&self) -> bool {
        *self == IDLECONF3R::HIGH
    }
    #[doc = "Checks if the value of the field is `LOW`"]
    #[inline]
    pub fn is_low(&self) -> bool {
        *self == IDLECONF3R::LOW
    }
}
#[doc = "Possible values of the field `IDLECONF4`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum IDLECONF4R {
    #[doc = "ALTEX4 output is disabled in idle phase"]
    DISABLE,
    #[doc = "ALTEX4 output is high in idle phase"]
    HIGH,
    #[doc = "ALTEX4 output is low in idle phase"]
    LOW,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl IDLECONF4R {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            IDLECONF4R::DISABLE => 0,
            IDLECONF4R::HIGH => 0x01,
            IDLECONF4R::LOW => 0x02,
            IDLECONF4R::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> IDLECONF4R {
        match value {
            0 => IDLECONF4R::DISABLE,
            1 => IDLECONF4R::HIGH,
            2 => IDLECONF4R::LOW,
            i => IDLECONF4R::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline]
    pub fn is_disable(&self) -> bool {
        *self == IDLECONF4R::DISABLE
    }
    #[doc = "Checks if the value of the field is `HIGH`"]
    #[inline]
    pub fn is_high(&self) -> bool {
        *self == IDLECONF4R::HIGH
    }
    #[doc = "Checks if the value of the field is `LOW`"]
    #[inline]
    pub fn is_low(&self) -> bool {
        *self == IDLECONF4R::LOW
    }
}
#[doc = "Possible values of the field `IDLECONF5`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum IDLECONF5R {
    #[doc = "ALTEX5 output is disabled in idle phase"]
    DISABLE,
    #[doc = "ALTEX5 output is high in idle phase"]
    HIGH,
    #[doc = "ALTEX5 output is low in idle phase"]
    LOW,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl IDLECONF5R {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            IDLECONF5R::DISABLE => 0,
            IDLECONF5R::HIGH => 0x01,
            IDLECONF5R::LOW => 0x02,
            IDLECONF5R::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> IDLECONF5R {
        match value {
            0 => IDLECONF5R::DISABLE,
            1 => IDLECONF5R::HIGH,
            2 => IDLECONF5R::LOW,
            i => IDLECONF5R::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline]
    pub fn is_disable(&self) -> bool {
        *self == IDLECONF5R::DISABLE
    }
    #[doc = "Checks if the value of the field is `HIGH`"]
    #[inline]
    pub fn is_high(&self) -> bool {
        *self == IDLECONF5R::HIGH
    }
    #[doc = "Checks if the value of the field is `LOW`"]
    #[inline]
    pub fn is_low(&self) -> bool {
        *self == IDLECONF5R::LOW
    }
}
#[doc = "Possible values of the field `IDLECONF6`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum IDLECONF6R {
    #[doc = "ALTEX6 output is disabled in idle phase"]
    DISABLE,
    #[doc = "ALTEX6 output is high in idle phase"]
    HIGH,
    #[doc = "ALTEX6 output is low in idle phase"]
    LOW,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl IDLECONF6R {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            IDLECONF6R::DISABLE => 0,
            IDLECONF6R::HIGH => 0x01,
            IDLECONF6R::LOW => 0x02,
            IDLECONF6R::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> IDLECONF6R {
        match value {
            0 => IDLECONF6R::DISABLE,
            1 => IDLECONF6R::HIGH,
            2 => IDLECONF6R::LOW,
            i => IDLECONF6R::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline]
    pub fn is_disable(&self) -> bool {
        *self == IDLECONF6R::DISABLE
    }
    #[doc = "Checks if the value of the field is `HIGH`"]
    #[inline]
    pub fn is_high(&self) -> bool {
        *self == IDLECONF6R::HIGH
    }
    #[doc = "Checks if the value of the field is `LOW`"]
    #[inline]
    pub fn is_low(&self) -> bool {
        *self == IDLECONF6R::LOW
    }
}
#[doc = "Possible values of the field `IDLECONF7`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum IDLECONF7R {
    #[doc = "ALTEX7 output is disabled in idle phase"]
    DISABLE,
    #[doc = "ALTEX7 output is high in idle phase"]
    HIGH,
    #[doc = "ALTEX7 output is low in idle phase"]
    LOW,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl IDLECONF7R {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            IDLECONF7R::DISABLE => 0,
            IDLECONF7R::HIGH => 0x01,
            IDLECONF7R::LOW => 0x02,
            IDLECONF7R::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> IDLECONF7R {
        match value {
            0 => IDLECONF7R::DISABLE,
            1 => IDLECONF7R::HIGH,
            2 => IDLECONF7R::LOW,
            i => IDLECONF7R::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline]
    pub fn is_disable(&self) -> bool {
        *self == IDLECONF7R::DISABLE
    }
    #[doc = "Checks if the value of the field is `HIGH`"]
    #[inline]
    pub fn is_high(&self) -> bool {
        *self == IDLECONF7R::HIGH
    }
    #[doc = "Checks if the value of the field is `LOW`"]
    #[inline]
    pub fn is_low(&self) -> bool {
        *self == IDLECONF7R::LOW
    }
}
#[doc = r" Value of the field"]
pub struct AEX0R {
    bits: bool,
}
impl AEX0R {
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
pub struct AEX1R {
    bits: bool,
}
impl AEX1R {
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
pub struct AEX2R {
    bits: bool,
}
impl AEX2R {
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
pub struct AEX3R {
    bits: bool,
}
impl AEX3R {
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
pub struct AEX4R {
    bits: bool,
}
impl AEX4R {
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
pub struct AEX5R {
    bits: bool,
}
impl AEX5R {
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
pub struct AEX6R {
    bits: bool,
}
impl AEX6R {
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
pub struct AEX7R {
    bits: bool,
}
impl AEX7R {
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
#[doc = "Values that can be written to the field `IDLECONF0`"]
pub enum IDLECONF0W {
    #[doc = "ALTEX0 output is disabled in idle phase"]
    DISABLE,
    #[doc = "ALTEX0 output is high in idle phase"]
    HIGH,
    #[doc = "ALTEX0 output is low in idle phase"]
    LOW,
}
impl IDLECONF0W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            IDLECONF0W::DISABLE => 0,
            IDLECONF0W::HIGH => 1,
            IDLECONF0W::LOW => 2,
        }
    }
}
#[doc = r" Proxy"]
pub struct _IDLECONF0W<'a> {
    w: &'a mut W,
}
impl<'a> _IDLECONF0W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: IDLECONF0W) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "ALTEX0 output is disabled in idle phase"]
    #[inline]
    pub fn disable(self) -> &'a mut W {
        self.variant(IDLECONF0W::DISABLE)
    }
    #[doc = "ALTEX0 output is high in idle phase"]
    #[inline]
    pub fn high(self) -> &'a mut W {
        self.variant(IDLECONF0W::HIGH)
    }
    #[doc = "ALTEX0 output is low in idle phase"]
    #[inline]
    pub fn low(self) -> &'a mut W {
        self.variant(IDLECONF0W::LOW)
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
#[doc = "Values that can be written to the field `IDLECONF1`"]
pub enum IDLECONF1W {
    #[doc = "ALTEX1 output is disabled in idle phase"]
    DISABLE,
    #[doc = "ALTEX1 output is high in idle phase"]
    HIGH,
    #[doc = "ALTEX1 output is low in idle phase"]
    LOW,
}
impl IDLECONF1W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            IDLECONF1W::DISABLE => 0,
            IDLECONF1W::HIGH => 1,
            IDLECONF1W::LOW => 2,
        }
    }
}
#[doc = r" Proxy"]
pub struct _IDLECONF1W<'a> {
    w: &'a mut W,
}
impl<'a> _IDLECONF1W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: IDLECONF1W) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "ALTEX1 output is disabled in idle phase"]
    #[inline]
    pub fn disable(self) -> &'a mut W {
        self.variant(IDLECONF1W::DISABLE)
    }
    #[doc = "ALTEX1 output is high in idle phase"]
    #[inline]
    pub fn high(self) -> &'a mut W {
        self.variant(IDLECONF1W::HIGH)
    }
    #[doc = "ALTEX1 output is low in idle phase"]
    #[inline]
    pub fn low(self) -> &'a mut W {
        self.variant(IDLECONF1W::LOW)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 0x03;
        const OFFSET: u8 = 2;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `IDLECONF2`"]
pub enum IDLECONF2W {
    #[doc = "ALTEX2 output is disabled in idle phase"]
    DISABLE,
    #[doc = "ALTEX2 output is high in idle phase"]
    HIGH,
    #[doc = "ALTEX2 output is low in idle phase"]
    LOW,
}
impl IDLECONF2W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            IDLECONF2W::DISABLE => 0,
            IDLECONF2W::HIGH => 1,
            IDLECONF2W::LOW => 2,
        }
    }
}
#[doc = r" Proxy"]
pub struct _IDLECONF2W<'a> {
    w: &'a mut W,
}
impl<'a> _IDLECONF2W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: IDLECONF2W) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "ALTEX2 output is disabled in idle phase"]
    #[inline]
    pub fn disable(self) -> &'a mut W {
        self.variant(IDLECONF2W::DISABLE)
    }
    #[doc = "ALTEX2 output is high in idle phase"]
    #[inline]
    pub fn high(self) -> &'a mut W {
        self.variant(IDLECONF2W::HIGH)
    }
    #[doc = "ALTEX2 output is low in idle phase"]
    #[inline]
    pub fn low(self) -> &'a mut W {
        self.variant(IDLECONF2W::LOW)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 0x03;
        const OFFSET: u8 = 4;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `IDLECONF3`"]
pub enum IDLECONF3W {
    #[doc = "ALTEX3 output is disabled in idle phase"]
    DISABLE,
    #[doc = "ALTEX3 output is high in idle phase"]
    HIGH,
    #[doc = "ALTEX3 output is low in idle phase"]
    LOW,
}
impl IDLECONF3W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            IDLECONF3W::DISABLE => 0,
            IDLECONF3W::HIGH => 1,
            IDLECONF3W::LOW => 2,
        }
    }
}
#[doc = r" Proxy"]
pub struct _IDLECONF3W<'a> {
    w: &'a mut W,
}
impl<'a> _IDLECONF3W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: IDLECONF3W) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "ALTEX3 output is disabled in idle phase"]
    #[inline]
    pub fn disable(self) -> &'a mut W {
        self.variant(IDLECONF3W::DISABLE)
    }
    #[doc = "ALTEX3 output is high in idle phase"]
    #[inline]
    pub fn high(self) -> &'a mut W {
        self.variant(IDLECONF3W::HIGH)
    }
    #[doc = "ALTEX3 output is low in idle phase"]
    #[inline]
    pub fn low(self) -> &'a mut W {
        self.variant(IDLECONF3W::LOW)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 0x03;
        const OFFSET: u8 = 6;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `IDLECONF4`"]
pub enum IDLECONF4W {
    #[doc = "ALTEX4 output is disabled in idle phase"]
    DISABLE,
    #[doc = "ALTEX4 output is high in idle phase"]
    HIGH,
    #[doc = "ALTEX4 output is low in idle phase"]
    LOW,
}
impl IDLECONF4W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            IDLECONF4W::DISABLE => 0,
            IDLECONF4W::HIGH => 1,
            IDLECONF4W::LOW => 2,
        }
    }
}
#[doc = r" Proxy"]
pub struct _IDLECONF4W<'a> {
    w: &'a mut W,
}
impl<'a> _IDLECONF4W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: IDLECONF4W) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "ALTEX4 output is disabled in idle phase"]
    #[inline]
    pub fn disable(self) -> &'a mut W {
        self.variant(IDLECONF4W::DISABLE)
    }
    #[doc = "ALTEX4 output is high in idle phase"]
    #[inline]
    pub fn high(self) -> &'a mut W {
        self.variant(IDLECONF4W::HIGH)
    }
    #[doc = "ALTEX4 output is low in idle phase"]
    #[inline]
    pub fn low(self) -> &'a mut W {
        self.variant(IDLECONF4W::LOW)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 0x03;
        const OFFSET: u8 = 8;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `IDLECONF5`"]
pub enum IDLECONF5W {
    #[doc = "ALTEX5 output is disabled in idle phase"]
    DISABLE,
    #[doc = "ALTEX5 output is high in idle phase"]
    HIGH,
    #[doc = "ALTEX5 output is low in idle phase"]
    LOW,
}
impl IDLECONF5W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            IDLECONF5W::DISABLE => 0,
            IDLECONF5W::HIGH => 1,
            IDLECONF5W::LOW => 2,
        }
    }
}
#[doc = r" Proxy"]
pub struct _IDLECONF5W<'a> {
    w: &'a mut W,
}
impl<'a> _IDLECONF5W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: IDLECONF5W) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "ALTEX5 output is disabled in idle phase"]
    #[inline]
    pub fn disable(self) -> &'a mut W {
        self.variant(IDLECONF5W::DISABLE)
    }
    #[doc = "ALTEX5 output is high in idle phase"]
    #[inline]
    pub fn high(self) -> &'a mut W {
        self.variant(IDLECONF5W::HIGH)
    }
    #[doc = "ALTEX5 output is low in idle phase"]
    #[inline]
    pub fn low(self) -> &'a mut W {
        self.variant(IDLECONF5W::LOW)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 0x03;
        const OFFSET: u8 = 10;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `IDLECONF6`"]
pub enum IDLECONF6W {
    #[doc = "ALTEX6 output is disabled in idle phase"]
    DISABLE,
    #[doc = "ALTEX6 output is high in idle phase"]
    HIGH,
    #[doc = "ALTEX6 output is low in idle phase"]
    LOW,
}
impl IDLECONF6W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            IDLECONF6W::DISABLE => 0,
            IDLECONF6W::HIGH => 1,
            IDLECONF6W::LOW => 2,
        }
    }
}
#[doc = r" Proxy"]
pub struct _IDLECONF6W<'a> {
    w: &'a mut W,
}
impl<'a> _IDLECONF6W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: IDLECONF6W) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "ALTEX6 output is disabled in idle phase"]
    #[inline]
    pub fn disable(self) -> &'a mut W {
        self.variant(IDLECONF6W::DISABLE)
    }
    #[doc = "ALTEX6 output is high in idle phase"]
    #[inline]
    pub fn high(self) -> &'a mut W {
        self.variant(IDLECONF6W::HIGH)
    }
    #[doc = "ALTEX6 output is low in idle phase"]
    #[inline]
    pub fn low(self) -> &'a mut W {
        self.variant(IDLECONF6W::LOW)
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
#[doc = "Values that can be written to the field `IDLECONF7`"]
pub enum IDLECONF7W {
    #[doc = "ALTEX7 output is disabled in idle phase"]
    DISABLE,
    #[doc = "ALTEX7 output is high in idle phase"]
    HIGH,
    #[doc = "ALTEX7 output is low in idle phase"]
    LOW,
}
impl IDLECONF7W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            IDLECONF7W::DISABLE => 0,
            IDLECONF7W::HIGH => 1,
            IDLECONF7W::LOW => 2,
        }
    }
}
#[doc = r" Proxy"]
pub struct _IDLECONF7W<'a> {
    w: &'a mut W,
}
impl<'a> _IDLECONF7W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: IDLECONF7W) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "ALTEX7 output is disabled in idle phase"]
    #[inline]
    pub fn disable(self) -> &'a mut W {
        self.variant(IDLECONF7W::DISABLE)
    }
    #[doc = "ALTEX7 output is high in idle phase"]
    #[inline]
    pub fn high(self) -> &'a mut W {
        self.variant(IDLECONF7W::HIGH)
    }
    #[doc = "ALTEX7 output is low in idle phase"]
    #[inline]
    pub fn low(self) -> &'a mut W {
        self.variant(IDLECONF7W::LOW)
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
#[doc = r" Proxy"]
pub struct _AEX0W<'a> {
    w: &'a mut W,
}
impl<'a> _AEX0W<'a> {
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
        const OFFSET: u8 = 16;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _AEX1W<'a> {
    w: &'a mut W,
}
impl<'a> _AEX1W<'a> {
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
        const OFFSET: u8 = 17;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _AEX2W<'a> {
    w: &'a mut W,
}
impl<'a> _AEX2W<'a> {
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
        const OFFSET: u8 = 18;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _AEX3W<'a> {
    w: &'a mut W,
}
impl<'a> _AEX3W<'a> {
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
        const OFFSET: u8 = 19;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _AEX4W<'a> {
    w: &'a mut W,
}
impl<'a> _AEX4W<'a> {
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
        const OFFSET: u8 = 20;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _AEX5W<'a> {
    w: &'a mut W,
}
impl<'a> _AEX5W<'a> {
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
        const OFFSET: u8 = 21;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _AEX6W<'a> {
    w: &'a mut W,
}
impl<'a> _AEX6W<'a> {
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
pub struct _AEX7W<'a> {
    w: &'a mut W,
}
impl<'a> _AEX7W<'a> {
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
impl R {
    #[doc = r" Value of the register as raw bits"]
    #[inline]
    pub fn bits(&self) -> u32 {
        self.bits
    }
    #[doc = "Bits 0:1 - ALTEX0 idle phase configuration"]
    #[inline]
    pub fn idleconf0(&self) -> IDLECONF0R {
        IDLECONF0R::_from({
            const MASK: u8 = 0x03;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 2:3 - ALTEX1 idle phase configuration"]
    #[inline]
    pub fn idleconf1(&self) -> IDLECONF1R {
        IDLECONF1R::_from({
            const MASK: u8 = 0x03;
            const OFFSET: u8 = 2;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 4:5 - ALTEX2 idle phase configuration"]
    #[inline]
    pub fn idleconf2(&self) -> IDLECONF2R {
        IDLECONF2R::_from({
            const MASK: u8 = 0x03;
            const OFFSET: u8 = 4;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 6:7 - ALTEX3 idle phase configuration"]
    #[inline]
    pub fn idleconf3(&self) -> IDLECONF3R {
        IDLECONF3R::_from({
            const MASK: u8 = 0x03;
            const OFFSET: u8 = 6;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 8:9 - ALTEX4 idle phase configuration"]
    #[inline]
    pub fn idleconf4(&self) -> IDLECONF4R {
        IDLECONF4R::_from({
            const MASK: u8 = 0x03;
            const OFFSET: u8 = 8;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 10:11 - ALTEX5 idle phase configuration"]
    #[inline]
    pub fn idleconf5(&self) -> IDLECONF5R {
        IDLECONF5R::_from({
            const MASK: u8 = 0x03;
            const OFFSET: u8 = 10;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 12:13 - ALTEX6 idle phase configuration"]
    #[inline]
    pub fn idleconf6(&self) -> IDLECONF6R {
        IDLECONF6R::_from({
            const MASK: u8 = 0x03;
            const OFFSET: u8 = 12;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 14:15 - ALTEX7 idle phase configuration"]
    #[inline]
    pub fn idleconf7(&self) -> IDLECONF7R {
        IDLECONF7R::_from({
            const MASK: u8 = 0x03;
            const OFFSET: u8 = 14;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bit 16 - ALTEX0 always excite enable"]
    #[inline]
    pub fn aex0(&self) -> AEX0R {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 16;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        AEX0R { bits }
    }
    #[doc = "Bit 17 - ALTEX1 always excite enable"]
    #[inline]
    pub fn aex1(&self) -> AEX1R {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 17;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        AEX1R { bits }
    }
    #[doc = "Bit 18 - ALTEX2 always excite enable"]
    #[inline]
    pub fn aex2(&self) -> AEX2R {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 18;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        AEX2R { bits }
    }
    #[doc = "Bit 19 - ALTEX3 always excite enable"]
    #[inline]
    pub fn aex3(&self) -> AEX3R {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 19;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        AEX3R { bits }
    }
    #[doc = "Bit 20 - ALTEX4 always excite enable"]
    #[inline]
    pub fn aex4(&self) -> AEX4R {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 20;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        AEX4R { bits }
    }
    #[doc = "Bit 21 - ALTEX5 always excite enable"]
    #[inline]
    pub fn aex5(&self) -> AEX5R {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 21;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        AEX5R { bits }
    }
    #[doc = "Bit 22 - ALTEX6 always excite enable"]
    #[inline]
    pub fn aex6(&self) -> AEX6R {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 22;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        AEX6R { bits }
    }
    #[doc = "Bit 23 - ALTEX7 always excite enable"]
    #[inline]
    pub fn aex7(&self) -> AEX7R {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 23;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        AEX7R { bits }
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
    #[doc = "Bits 0:1 - ALTEX0 idle phase configuration"]
    #[inline]
    pub fn idleconf0(&mut self) -> _IDLECONF0W {
        _IDLECONF0W { w: self }
    }
    #[doc = "Bits 2:3 - ALTEX1 idle phase configuration"]
    #[inline]
    pub fn idleconf1(&mut self) -> _IDLECONF1W {
        _IDLECONF1W { w: self }
    }
    #[doc = "Bits 4:5 - ALTEX2 idle phase configuration"]
    #[inline]
    pub fn idleconf2(&mut self) -> _IDLECONF2W {
        _IDLECONF2W { w: self }
    }
    #[doc = "Bits 6:7 - ALTEX3 idle phase configuration"]
    #[inline]
    pub fn idleconf3(&mut self) -> _IDLECONF3W {
        _IDLECONF3W { w: self }
    }
    #[doc = "Bits 8:9 - ALTEX4 idle phase configuration"]
    #[inline]
    pub fn idleconf4(&mut self) -> _IDLECONF4W {
        _IDLECONF4W { w: self }
    }
    #[doc = "Bits 10:11 - ALTEX5 idle phase configuration"]
    #[inline]
    pub fn idleconf5(&mut self) -> _IDLECONF5W {
        _IDLECONF5W { w: self }
    }
    #[doc = "Bits 12:13 - ALTEX6 idle phase configuration"]
    #[inline]
    pub fn idleconf6(&mut self) -> _IDLECONF6W {
        _IDLECONF6W { w: self }
    }
    #[doc = "Bits 14:15 - ALTEX7 idle phase configuration"]
    #[inline]
    pub fn idleconf7(&mut self) -> _IDLECONF7W {
        _IDLECONF7W { w: self }
    }
    #[doc = "Bit 16 - ALTEX0 always excite enable"]
    #[inline]
    pub fn aex0(&mut self) -> _AEX0W {
        _AEX0W { w: self }
    }
    #[doc = "Bit 17 - ALTEX1 always excite enable"]
    #[inline]
    pub fn aex1(&mut self) -> _AEX1W {
        _AEX1W { w: self }
    }
    #[doc = "Bit 18 - ALTEX2 always excite enable"]
    #[inline]
    pub fn aex2(&mut self) -> _AEX2W {
        _AEX2W { w: self }
    }
    #[doc = "Bit 19 - ALTEX3 always excite enable"]
    #[inline]
    pub fn aex3(&mut self) -> _AEX3W {
        _AEX3W { w: self }
    }
    #[doc = "Bit 20 - ALTEX4 always excite enable"]
    #[inline]
    pub fn aex4(&mut self) -> _AEX4W {
        _AEX4W { w: self }
    }
    #[doc = "Bit 21 - ALTEX5 always excite enable"]
    #[inline]
    pub fn aex5(&mut self) -> _AEX5W {
        _AEX5W { w: self }
    }
    #[doc = "Bit 22 - ALTEX6 always excite enable"]
    #[inline]
    pub fn aex6(&mut self) -> _AEX6W {
        _AEX6W { w: self }
    }
    #[doc = "Bit 23 - ALTEX7 always excite enable"]
    #[inline]
    pub fn aex7(&mut self) -> _AEX7W {
        _AEX7W { w: self }
    }
}

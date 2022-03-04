#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::ROUTE {
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
pub struct SWCLKPENR {
    bits: bool,
}
impl SWCLKPENR {
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
pub struct SWDIOPENR {
    bits: bool,
}
impl SWDIOPENR {
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
pub struct SWOPENR {
    bits: bool,
}
impl SWOPENR {
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
#[doc = "Possible values of the field `SWLOCATION`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SWLOCATIONR {
    #[doc = "Location 0"]
    LOC0,
    #[doc = "Location 1"]
    LOC1,
    #[doc = "Location 2"]
    LOC2,
    #[doc = "Location 3"]
    LOC3,
}
impl SWLOCATIONR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            SWLOCATIONR::LOC0 => 0,
            SWLOCATIONR::LOC1 => 0x01,
            SWLOCATIONR::LOC2 => 0x02,
            SWLOCATIONR::LOC3 => 0x03,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> SWLOCATIONR {
        match value {
            0 => SWLOCATIONR::LOC0,
            1 => SWLOCATIONR::LOC1,
            2 => SWLOCATIONR::LOC2,
            3 => SWLOCATIONR::LOC3,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `LOC0`"]
    #[inline]
    pub fn is_loc0(&self) -> bool {
        *self == SWLOCATIONR::LOC0
    }
    #[doc = "Checks if the value of the field is `LOC1`"]
    #[inline]
    pub fn is_loc1(&self) -> bool {
        *self == SWLOCATIONR::LOC1
    }
    #[doc = "Checks if the value of the field is `LOC2`"]
    #[inline]
    pub fn is_loc2(&self) -> bool {
        *self == SWLOCATIONR::LOC2
    }
    #[doc = "Checks if the value of the field is `LOC3`"]
    #[inline]
    pub fn is_loc3(&self) -> bool {
        *self == SWLOCATIONR::LOC3
    }
}
#[doc = r" Value of the field"]
pub struct TCLKPENR {
    bits: bool,
}
impl TCLKPENR {
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
pub struct TD0PENR {
    bits: bool,
}
impl TD0PENR {
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
pub struct TD1PENR {
    bits: bool,
}
impl TD1PENR {
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
pub struct TD2PENR {
    bits: bool,
}
impl TD2PENR {
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
pub struct TD3PENR {
    bits: bool,
}
impl TD3PENR {
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
#[doc = "Possible values of the field `ETMLOCATION`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ETMLOCATIONR {
    #[doc = "Location 0"]
    LOC0,
    #[doc = "Location 1"]
    LOC1,
    #[doc = "Location 2"]
    LOC2,
    #[doc = "Location 3"]
    LOC3,
}
impl ETMLOCATIONR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            ETMLOCATIONR::LOC0 => 0,
            ETMLOCATIONR::LOC1 => 0x01,
            ETMLOCATIONR::LOC2 => 0x02,
            ETMLOCATIONR::LOC3 => 0x03,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> ETMLOCATIONR {
        match value {
            0 => ETMLOCATIONR::LOC0,
            1 => ETMLOCATIONR::LOC1,
            2 => ETMLOCATIONR::LOC2,
            3 => ETMLOCATIONR::LOC3,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `LOC0`"]
    #[inline]
    pub fn is_loc0(&self) -> bool {
        *self == ETMLOCATIONR::LOC0
    }
    #[doc = "Checks if the value of the field is `LOC1`"]
    #[inline]
    pub fn is_loc1(&self) -> bool {
        *self == ETMLOCATIONR::LOC1
    }
    #[doc = "Checks if the value of the field is `LOC2`"]
    #[inline]
    pub fn is_loc2(&self) -> bool {
        *self == ETMLOCATIONR::LOC2
    }
    #[doc = "Checks if the value of the field is `LOC3`"]
    #[inline]
    pub fn is_loc3(&self) -> bool {
        *self == ETMLOCATIONR::LOC3
    }
}
#[doc = r" Proxy"]
pub struct _SWCLKPENW<'a> {
    w: &'a mut W,
}
impl<'a> _SWCLKPENW<'a> {
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
pub struct _SWDIOPENW<'a> {
    w: &'a mut W,
}
impl<'a> _SWDIOPENW<'a> {
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
pub struct _SWOPENW<'a> {
    w: &'a mut W,
}
impl<'a> _SWOPENW<'a> {
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
#[doc = "Values that can be written to the field `SWLOCATION`"]
pub enum SWLOCATIONW {
    #[doc = "Location 0"]
    LOC0,
    #[doc = "Location 1"]
    LOC1,
    #[doc = "Location 2"]
    LOC2,
    #[doc = "Location 3"]
    LOC3,
}
impl SWLOCATIONW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            SWLOCATIONW::LOC0 => 0,
            SWLOCATIONW::LOC1 => 1,
            SWLOCATIONW::LOC2 => 2,
            SWLOCATIONW::LOC3 => 3,
        }
    }
}
#[doc = r" Proxy"]
pub struct _SWLOCATIONW<'a> {
    w: &'a mut W,
}
impl<'a> _SWLOCATIONW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: SWLOCATIONW) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "Location 0"]
    #[inline]
    pub fn loc0(self) -> &'a mut W {
        self.variant(SWLOCATIONW::LOC0)
    }
    #[doc = "Location 1"]
    #[inline]
    pub fn loc1(self) -> &'a mut W {
        self.variant(SWLOCATIONW::LOC1)
    }
    #[doc = "Location 2"]
    #[inline]
    pub fn loc2(self) -> &'a mut W {
        self.variant(SWLOCATIONW::LOC2)
    }
    #[doc = "Location 3"]
    #[inline]
    pub fn loc3(self) -> &'a mut W {
        self.variant(SWLOCATIONW::LOC3)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 0x03;
        const OFFSET: u8 = 8;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _TCLKPENW<'a> {
    w: &'a mut W,
}
impl<'a> _TCLKPENW<'a> {
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
#[doc = r" Proxy"]
pub struct _TD0PENW<'a> {
    w: &'a mut W,
}
impl<'a> _TD0PENW<'a> {
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
        const OFFSET: u8 = 13;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _TD1PENW<'a> {
    w: &'a mut W,
}
impl<'a> _TD1PENW<'a> {
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
        const OFFSET: u8 = 14;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _TD2PENW<'a> {
    w: &'a mut W,
}
impl<'a> _TD2PENW<'a> {
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
        const OFFSET: u8 = 15;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _TD3PENW<'a> {
    w: &'a mut W,
}
impl<'a> _TD3PENW<'a> {
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
#[doc = "Values that can be written to the field `ETMLOCATION`"]
pub enum ETMLOCATIONW {
    #[doc = "Location 0"]
    LOC0,
    #[doc = "Location 1"]
    LOC1,
    #[doc = "Location 2"]
    LOC2,
    #[doc = "Location 3"]
    LOC3,
}
impl ETMLOCATIONW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            ETMLOCATIONW::LOC0 => 0,
            ETMLOCATIONW::LOC1 => 1,
            ETMLOCATIONW::LOC2 => 2,
            ETMLOCATIONW::LOC3 => 3,
        }
    }
}
#[doc = r" Proxy"]
pub struct _ETMLOCATIONW<'a> {
    w: &'a mut W,
}
impl<'a> _ETMLOCATIONW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: ETMLOCATIONW) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "Location 0"]
    #[inline]
    pub fn loc0(self) -> &'a mut W {
        self.variant(ETMLOCATIONW::LOC0)
    }
    #[doc = "Location 1"]
    #[inline]
    pub fn loc1(self) -> &'a mut W {
        self.variant(ETMLOCATIONW::LOC1)
    }
    #[doc = "Location 2"]
    #[inline]
    pub fn loc2(self) -> &'a mut W {
        self.variant(ETMLOCATIONW::LOC2)
    }
    #[doc = "Location 3"]
    #[inline]
    pub fn loc3(self) -> &'a mut W {
        self.variant(ETMLOCATIONW::LOC3)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 0x03;
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
    #[doc = "Bit 0 - Serial Wire Clock Pin Enable"]
    #[inline]
    pub fn swclkpen(&self) -> SWCLKPENR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        SWCLKPENR { bits }
    }
    #[doc = "Bit 1 - Serial Wire Data Pin Enable"]
    #[inline]
    pub fn swdiopen(&self) -> SWDIOPENR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 1;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        SWDIOPENR { bits }
    }
    #[doc = "Bit 2 - Serial Wire Viewer Output Pin Enable"]
    #[inline]
    pub fn swopen(&self) -> SWOPENR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 2;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        SWOPENR { bits }
    }
    #[doc = "Bits 8:9 - I/O Location"]
    #[inline]
    pub fn swlocation(&self) -> SWLOCATIONR {
        SWLOCATIONR::_from({
            const MASK: u8 = 0x03;
            const OFFSET: u8 = 8;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bit 12 - ETM Trace Clock Pin Enable"]
    #[inline]
    pub fn tclkpen(&self) -> TCLKPENR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 12;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        TCLKPENR { bits }
    }
    #[doc = "Bit 13 - ETM Trace Data Pin Enable"]
    #[inline]
    pub fn td0pen(&self) -> TD0PENR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 13;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        TD0PENR { bits }
    }
    #[doc = "Bit 14 - ETM Trace Data Pin Enable"]
    #[inline]
    pub fn td1pen(&self) -> TD1PENR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 14;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        TD1PENR { bits }
    }
    #[doc = "Bit 15 - ETM Trace Data Pin Enable"]
    #[inline]
    pub fn td2pen(&self) -> TD2PENR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 15;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        TD2PENR { bits }
    }
    #[doc = "Bit 16 - ETM Trace Data Pin Enable"]
    #[inline]
    pub fn td3pen(&self) -> TD3PENR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 16;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        TD3PENR { bits }
    }
    #[doc = "Bits 24:25 - I/O Location"]
    #[inline]
    pub fn etmlocation(&self) -> ETMLOCATIONR {
        ETMLOCATIONR::_from({
            const MASK: u8 = 0x03;
            const OFFSET: u8 = 24;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
}
impl W {
    #[doc = r" Reset value of the register"]
    #[inline]
    pub fn reset_value() -> W {
        W { bits: 0x03 }
    }
    #[doc = r" Writes raw bits to the register"]
    #[inline]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
    #[doc = "Bit 0 - Serial Wire Clock Pin Enable"]
    #[inline]
    pub fn swclkpen(&mut self) -> _SWCLKPENW {
        _SWCLKPENW { w: self }
    }
    #[doc = "Bit 1 - Serial Wire Data Pin Enable"]
    #[inline]
    pub fn swdiopen(&mut self) -> _SWDIOPENW {
        _SWDIOPENW { w: self }
    }
    #[doc = "Bit 2 - Serial Wire Viewer Output Pin Enable"]
    #[inline]
    pub fn swopen(&mut self) -> _SWOPENW {
        _SWOPENW { w: self }
    }
    #[doc = "Bits 8:9 - I/O Location"]
    #[inline]
    pub fn swlocation(&mut self) -> _SWLOCATIONW {
        _SWLOCATIONW { w: self }
    }
    #[doc = "Bit 12 - ETM Trace Clock Pin Enable"]
    #[inline]
    pub fn tclkpen(&mut self) -> _TCLKPENW {
        _TCLKPENW { w: self }
    }
    #[doc = "Bit 13 - ETM Trace Data Pin Enable"]
    #[inline]
    pub fn td0pen(&mut self) -> _TD0PENW {
        _TD0PENW { w: self }
    }
    #[doc = "Bit 14 - ETM Trace Data Pin Enable"]
    #[inline]
    pub fn td1pen(&mut self) -> _TD1PENW {
        _TD1PENW { w: self }
    }
    #[doc = "Bit 15 - ETM Trace Data Pin Enable"]
    #[inline]
    pub fn td2pen(&mut self) -> _TD2PENW {
        _TD2PENW { w: self }
    }
    #[doc = "Bit 16 - ETM Trace Data Pin Enable"]
    #[inline]
    pub fn td3pen(&mut self) -> _TD3PENW {
        _TD3PENW { w: self }
    }
    #[doc = "Bits 24:25 - I/O Location"]
    #[inline]
    pub fn etmlocation(&mut self) -> _ETMLOCATIONW {
        _ETMLOCATIONW { w: self }
    }
}

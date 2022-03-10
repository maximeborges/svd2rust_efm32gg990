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
pub struct CLKOUT0PENR {
    bits: bool,
}
impl CLKOUT0PENR {
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
pub struct CLKOUT1PENR {
    bits: bool,
}
impl CLKOUT1PENR {
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
#[doc = "Possible values of the field `LOCATION`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LOCATIONR {
    #[doc = "Location 0"]
    LOC0,
    #[doc = "Location 1"]
    LOC1,
    #[doc = "Location 2"]
    LOC2,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl LOCATIONR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            LOCATIONR::LOC0 => 0,
            LOCATIONR::LOC1 => 0x01,
            LOCATIONR::LOC2 => 0x02,
            LOCATIONR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> LOCATIONR {
        match value {
            0 => LOCATIONR::LOC0,
            1 => LOCATIONR::LOC1,
            2 => LOCATIONR::LOC2,
            i => LOCATIONR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `LOC0`"]
    #[inline]
    pub fn is_loc0(&self) -> bool {
        *self == LOCATIONR::LOC0
    }
    #[doc = "Checks if the value of the field is `LOC1`"]
    #[inline]
    pub fn is_loc1(&self) -> bool {
        *self == LOCATIONR::LOC1
    }
    #[doc = "Checks if the value of the field is `LOC2`"]
    #[inline]
    pub fn is_loc2(&self) -> bool {
        *self == LOCATIONR::LOC2
    }
}
#[doc = r" Proxy"]
pub struct _CLKOUT0PENW<'a> {
    w: &'a mut W,
}
impl<'a> _CLKOUT0PENW<'a> {
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
pub struct _CLKOUT1PENW<'a> {
    w: &'a mut W,
}
impl<'a> _CLKOUT1PENW<'a> {
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
#[doc = "Values that can be written to the field `LOCATION`"]
pub enum LOCATIONW {
    #[doc = "Location 0"]
    LOC0,
    #[doc = "Location 1"]
    LOC1,
    #[doc = "Location 2"]
    LOC2,
}
impl LOCATIONW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            LOCATIONW::LOC0 => 0,
            LOCATIONW::LOC1 => 1,
            LOCATIONW::LOC2 => 2,
        }
    }
}
#[doc = r" Proxy"]
pub struct _LOCATIONW<'a> {
    w: &'a mut W,
}
impl<'a> _LOCATIONW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: LOCATIONW) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "Location 0"]
    #[inline]
    pub fn loc0(self) -> &'a mut W {
        self.variant(LOCATIONW::LOC0)
    }
    #[doc = "Location 1"]
    #[inline]
    pub fn loc1(self) -> &'a mut W {
        self.variant(LOCATIONW::LOC1)
    }
    #[doc = "Location 2"]
    #[inline]
    pub fn loc2(self) -> &'a mut W {
        self.variant(LOCATIONW::LOC2)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 0x07;
        const OFFSET: u8 = 2;
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
    #[doc = "Bit 0 - CLKOUT0 Pin Enable"]
    #[inline]
    pub fn clkout0pen(&self) -> CLKOUT0PENR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        CLKOUT0PENR { bits }
    }
    #[doc = "Bit 1 - CLKOUT1 Pin Enable"]
    #[inline]
    pub fn clkout1pen(&self) -> CLKOUT1PENR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 1;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        CLKOUT1PENR { bits }
    }
    #[doc = "Bits 2:4 - I/O Location"]
    #[inline]
    pub fn location(&self) -> LOCATIONR {
        LOCATIONR::_from({
            const MASK: u8 = 0x07;
            const OFFSET: u8 = 2;
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
    #[doc = "Bit 0 - CLKOUT0 Pin Enable"]
    #[inline]
    pub fn clkout0pen(&mut self) -> _CLKOUT0PENW {
        _CLKOUT0PENW { w: self }
    }
    #[doc = "Bit 1 - CLKOUT1 Pin Enable"]
    #[inline]
    pub fn clkout1pen(&mut self) -> _CLKOUT1PENW {
        _CLKOUT1PENW { w: self }
    }
    #[doc = "Bits 2:4 - I/O Location"]
    #[inline]
    pub fn location(&mut self) -> _LOCATIONW {
        _LOCATIONW { w: self }
    }
}

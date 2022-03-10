#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::HFRCOCTRL {
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
pub struct TUNINGR {
    bits: u8,
}
impl TUNINGR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = "Possible values of the field `BAND`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BANDR {
    #[doc = "1 MHz band. NOTE: Also set the TUNING value (bits 7:0) when changing band."]
    _1MHZ,
    #[doc = "7 MHz band. NOTE: Also set the TUNING value (bits 7:0) when changing band."]
    _7MHZ,
    #[doc = "11 MHz band. NOTE: Also set the TUNING value (bits 7:0) when changing band."]
    _11MHZ,
    #[doc = "14 MHz band. NOTE: Also set the TUNING value (bits 7:0) when changing band."]
    _14MHZ,
    #[doc = "21 MHz band. NOTE: Also set the TUNING value (bits 7:0) when changing band."]
    _21MHZ,
    #[doc = "28 MHz band. NOTE: Also set the TUNING value (bits 7:0) when changing band."]
    _28MHZ,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl BANDR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            BANDR::_1MHZ => 0,
            BANDR::_7MHZ => 0x01,
            BANDR::_11MHZ => 0x02,
            BANDR::_14MHZ => 0x03,
            BANDR::_21MHZ => 0x04,
            BANDR::_28MHZ => 0x05,
            BANDR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> BANDR {
        match value {
            0 => BANDR::_1MHZ,
            1 => BANDR::_7MHZ,
            2 => BANDR::_11MHZ,
            3 => BANDR::_14MHZ,
            4 => BANDR::_21MHZ,
            5 => BANDR::_28MHZ,
            i => BANDR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `_1MHZ`"]
    #[inline]
    pub fn is_1mhz(&self) -> bool {
        *self == BANDR::_1MHZ
    }
    #[doc = "Checks if the value of the field is `_7MHZ`"]
    #[inline]
    pub fn is_7mhz(&self) -> bool {
        *self == BANDR::_7MHZ
    }
    #[doc = "Checks if the value of the field is `_11MHZ`"]
    #[inline]
    pub fn is_11mhz(&self) -> bool {
        *self == BANDR::_11MHZ
    }
    #[doc = "Checks if the value of the field is `_14MHZ`"]
    #[inline]
    pub fn is_14mhz(&self) -> bool {
        *self == BANDR::_14MHZ
    }
    #[doc = "Checks if the value of the field is `_21MHZ`"]
    #[inline]
    pub fn is_21mhz(&self) -> bool {
        *self == BANDR::_21MHZ
    }
    #[doc = "Checks if the value of the field is `_28MHZ`"]
    #[inline]
    pub fn is_28mhz(&self) -> bool {
        *self == BANDR::_28MHZ
    }
}
#[doc = r" Value of the field"]
pub struct SUDELAYR {
    bits: u8,
}
impl SUDELAYR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Proxy"]
pub struct _TUNINGW<'a> {
    w: &'a mut W,
}
impl<'a> _TUNINGW<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 0xff;
        const OFFSET: u8 = 0;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `BAND`"]
pub enum BANDW {
    #[doc = "1 MHz band. NOTE: Also set the TUNING value (bits 7:0) when changing band."]
    _1MHZ,
    #[doc = "7 MHz band. NOTE: Also set the TUNING value (bits 7:0) when changing band."]
    _7MHZ,
    #[doc = "11 MHz band. NOTE: Also set the TUNING value (bits 7:0) when changing band."]
    _11MHZ,
    #[doc = "14 MHz band. NOTE: Also set the TUNING value (bits 7:0) when changing band."]
    _14MHZ,
    #[doc = "21 MHz band. NOTE: Also set the TUNING value (bits 7:0) when changing band."]
    _21MHZ,
    #[doc = "28 MHz band. NOTE: Also set the TUNING value (bits 7:0) when changing band."]
    _28MHZ,
}
impl BANDW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            BANDW::_1MHZ => 0,
            BANDW::_7MHZ => 1,
            BANDW::_11MHZ => 2,
            BANDW::_14MHZ => 3,
            BANDW::_21MHZ => 4,
            BANDW::_28MHZ => 5,
        }
    }
}
#[doc = r" Proxy"]
pub struct _BANDW<'a> {
    w: &'a mut W,
}
impl<'a> _BANDW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: BANDW) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "1 MHz band. NOTE: Also set the TUNING value (bits 7:0) when changing band."]
    #[inline]
    pub fn _1mhz(self) -> &'a mut W {
        self.variant(BANDW::_1MHZ)
    }
    #[doc = "7 MHz band. NOTE: Also set the TUNING value (bits 7:0) when changing band."]
    #[inline]
    pub fn _7mhz(self) -> &'a mut W {
        self.variant(BANDW::_7MHZ)
    }
    #[doc = "11 MHz band. NOTE: Also set the TUNING value (bits 7:0) when changing band."]
    #[inline]
    pub fn _11mhz(self) -> &'a mut W {
        self.variant(BANDW::_11MHZ)
    }
    #[doc = "14 MHz band. NOTE: Also set the TUNING value (bits 7:0) when changing band."]
    #[inline]
    pub fn _14mhz(self) -> &'a mut W {
        self.variant(BANDW::_14MHZ)
    }
    #[doc = "21 MHz band. NOTE: Also set the TUNING value (bits 7:0) when changing band."]
    #[inline]
    pub fn _21mhz(self) -> &'a mut W {
        self.variant(BANDW::_21MHZ)
    }
    #[doc = "28 MHz band. NOTE: Also set the TUNING value (bits 7:0) when changing band."]
    #[inline]
    pub fn _28mhz(self) -> &'a mut W {
        self.variant(BANDW::_28MHZ)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 0x07;
        const OFFSET: u8 = 8;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _SUDELAYW<'a> {
    w: &'a mut W,
}
impl<'a> _SUDELAYW<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 0x1f;
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
    #[doc = "Bits 0:7 - HFRCO Tuning Value"]
    #[inline]
    pub fn tuning(&self) -> TUNINGR {
        let bits = {
            const MASK: u8 = 0xff;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        TUNINGR { bits }
    }
    #[doc = "Bits 8:10 - HFRCO Band Select"]
    #[inline]
    pub fn band(&self) -> BANDR {
        BANDR::_from({
            const MASK: u8 = 0x07;
            const OFFSET: u8 = 8;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 12:16 - HFRCO Start-up Delay"]
    #[inline]
    pub fn sudelay(&self) -> SUDELAYR {
        let bits = {
            const MASK: u8 = 0x1f;
            const OFFSET: u8 = 12;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        SUDELAYR { bits }
    }
}
impl W {
    #[doc = r" Reset value of the register"]
    #[inline]
    pub fn reset_value() -> W {
        W { bits: 0x0380 }
    }
    #[doc = r" Writes raw bits to the register"]
    #[inline]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
    #[doc = "Bits 0:7 - HFRCO Tuning Value"]
    #[inline]
    pub fn tuning(&mut self) -> _TUNINGW {
        _TUNINGW { w: self }
    }
    #[doc = "Bits 8:10 - HFRCO Band Select"]
    #[inline]
    pub fn band(&mut self) -> _BANDW {
        _BANDW { w: self }
    }
    #[doc = "Bits 12:16 - HFRCO Start-up Delay"]
    #[inline]
    pub fn sudelay(&mut self) -> _SUDELAYW {
        _SUDELAYW { w: self }
    }
}

#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::LCDCTRL {
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
pub struct FDIVR {
    bits: u8,
}
impl FDIVR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct VBOOSTENR {
    bits: bool,
}
impl VBOOSTENR {
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
#[doc = "Possible values of the field `VBFDIV`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum VBFDIVR {
    #[doc = "Voltage Boost update Frequency = LFACLK."]
    DIV1,
    #[doc = "Voltage Boost update Frequency = LFACLK/2."]
    DIV2,
    #[doc = "Voltage Boost update Frequency = LFACLK/4."]
    DIV4,
    #[doc = "Voltage Boost update Frequency = LFACLK/8."]
    DIV8,
    #[doc = "Voltage Boost update Frequency = LFACLK/16."]
    DIV16,
    #[doc = "Voltage Boost update Frequency = LFACLK/32."]
    DIV32,
    #[doc = "Voltage Boost update Frequency = LFACLK/64."]
    DIV64,
    #[doc = "Voltage Boost update Frequency = LFACLK/128."]
    DIV128,
}
impl VBFDIVR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            VBFDIVR::DIV1 => 0,
            VBFDIVR::DIV2 => 0x01,
            VBFDIVR::DIV4 => 0x02,
            VBFDIVR::DIV8 => 0x03,
            VBFDIVR::DIV16 => 0x04,
            VBFDIVR::DIV32 => 0x05,
            VBFDIVR::DIV64 => 0x06,
            VBFDIVR::DIV128 => 0x07,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> VBFDIVR {
        match value {
            0 => VBFDIVR::DIV1,
            1 => VBFDIVR::DIV2,
            2 => VBFDIVR::DIV4,
            3 => VBFDIVR::DIV8,
            4 => VBFDIVR::DIV16,
            5 => VBFDIVR::DIV32,
            6 => VBFDIVR::DIV64,
            7 => VBFDIVR::DIV128,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `DIV1`"]
    #[inline]
    pub fn is_div1(&self) -> bool {
        *self == VBFDIVR::DIV1
    }
    #[doc = "Checks if the value of the field is `DIV2`"]
    #[inline]
    pub fn is_div2(&self) -> bool {
        *self == VBFDIVR::DIV2
    }
    #[doc = "Checks if the value of the field is `DIV4`"]
    #[inline]
    pub fn is_div4(&self) -> bool {
        *self == VBFDIVR::DIV4
    }
    #[doc = "Checks if the value of the field is `DIV8`"]
    #[inline]
    pub fn is_div8(&self) -> bool {
        *self == VBFDIVR::DIV8
    }
    #[doc = "Checks if the value of the field is `DIV16`"]
    #[inline]
    pub fn is_div16(&self) -> bool {
        *self == VBFDIVR::DIV16
    }
    #[doc = "Checks if the value of the field is `DIV32`"]
    #[inline]
    pub fn is_div32(&self) -> bool {
        *self == VBFDIVR::DIV32
    }
    #[doc = "Checks if the value of the field is `DIV64`"]
    #[inline]
    pub fn is_div64(&self) -> bool {
        *self == VBFDIVR::DIV64
    }
    #[doc = "Checks if the value of the field is `DIV128`"]
    #[inline]
    pub fn is_div128(&self) -> bool {
        *self == VBFDIVR::DIV128
    }
}
#[doc = r" Proxy"]
pub struct _FDIVW<'a> {
    w: &'a mut W,
}
impl<'a> _FDIVW<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 0x07;
        const OFFSET: u8 = 0;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _VBOOSTENW<'a> {
    w: &'a mut W,
}
impl<'a> _VBOOSTENW<'a> {
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
#[doc = "Values that can be written to the field `VBFDIV`"]
pub enum VBFDIVW {
    #[doc = "Voltage Boost update Frequency = LFACLK."]
    DIV1,
    #[doc = "Voltage Boost update Frequency = LFACLK/2."]
    DIV2,
    #[doc = "Voltage Boost update Frequency = LFACLK/4."]
    DIV4,
    #[doc = "Voltage Boost update Frequency = LFACLK/8."]
    DIV8,
    #[doc = "Voltage Boost update Frequency = LFACLK/16."]
    DIV16,
    #[doc = "Voltage Boost update Frequency = LFACLK/32."]
    DIV32,
    #[doc = "Voltage Boost update Frequency = LFACLK/64."]
    DIV64,
    #[doc = "Voltage Boost update Frequency = LFACLK/128."]
    DIV128,
}
impl VBFDIVW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            VBFDIVW::DIV1 => 0,
            VBFDIVW::DIV2 => 1,
            VBFDIVW::DIV4 => 2,
            VBFDIVW::DIV8 => 3,
            VBFDIVW::DIV16 => 4,
            VBFDIVW::DIV32 => 5,
            VBFDIVW::DIV64 => 6,
            VBFDIVW::DIV128 => 7,
        }
    }
}
#[doc = r" Proxy"]
pub struct _VBFDIVW<'a> {
    w: &'a mut W,
}
impl<'a> _VBFDIVW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: VBFDIVW) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "Voltage Boost update Frequency = LFACLK."]
    #[inline]
    pub fn div1(self) -> &'a mut W {
        self.variant(VBFDIVW::DIV1)
    }
    #[doc = "Voltage Boost update Frequency = LFACLK/2."]
    #[inline]
    pub fn div2(self) -> &'a mut W {
        self.variant(VBFDIVW::DIV2)
    }
    #[doc = "Voltage Boost update Frequency = LFACLK/4."]
    #[inline]
    pub fn div4(self) -> &'a mut W {
        self.variant(VBFDIVW::DIV4)
    }
    #[doc = "Voltage Boost update Frequency = LFACLK/8."]
    #[inline]
    pub fn div8(self) -> &'a mut W {
        self.variant(VBFDIVW::DIV8)
    }
    #[doc = "Voltage Boost update Frequency = LFACLK/16."]
    #[inline]
    pub fn div16(self) -> &'a mut W {
        self.variant(VBFDIVW::DIV16)
    }
    #[doc = "Voltage Boost update Frequency = LFACLK/32."]
    #[inline]
    pub fn div32(self) -> &'a mut W {
        self.variant(VBFDIVW::DIV32)
    }
    #[doc = "Voltage Boost update Frequency = LFACLK/64."]
    #[inline]
    pub fn div64(self) -> &'a mut W {
        self.variant(VBFDIVW::DIV64)
    }
    #[doc = "Voltage Boost update Frequency = LFACLK/128."]
    #[inline]
    pub fn div128(self) -> &'a mut W {
        self.variant(VBFDIVW::DIV128)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 0x07;
        const OFFSET: u8 = 4;
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
    #[doc = "Bits 0:2 - Frame Rate Control"]
    #[inline]
    pub fn fdiv(&self) -> FDIVR {
        let bits = {
            const MASK: u8 = 0x07;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        FDIVR { bits }
    }
    #[doc = "Bit 3 - Voltage Boost Enable"]
    #[inline]
    pub fn vboosten(&self) -> VBOOSTENR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 3;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        VBOOSTENR { bits }
    }
    #[doc = "Bits 4:6 - Voltage Boost Frequency Division"]
    #[inline]
    pub fn vbfdiv(&self) -> VBFDIVR {
        VBFDIVR::_from({
            const MASK: u8 = 0x07;
            const OFFSET: u8 = 4;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
}
impl W {
    #[doc = r" Reset value of the register"]
    #[inline]
    pub fn reset_value() -> W {
        W { bits: 0x20 }
    }
    #[doc = r" Writes raw bits to the register"]
    #[inline]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
    #[doc = "Bits 0:2 - Frame Rate Control"]
    #[inline]
    pub fn fdiv(&mut self) -> _FDIVW {
        _FDIVW { w: self }
    }
    #[doc = "Bit 3 - Voltage Boost Enable"]
    #[inline]
    pub fn vboosten(&mut self) -> _VBOOSTENW {
        _VBOOSTENW { w: self }
    }
    #[doc = "Bits 4:6 - Voltage Boost Frequency Division"]
    #[inline]
    pub fn vbfdiv(&mut self) -> _VBFDIVW {
        _VBFDIVW { w: self }
    }
}

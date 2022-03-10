#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::INPUT {
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
#[doc = "Possible values of the field `RXPRSSEL`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RXPRSSELR {
    #[doc = "PRS Channel 0 selected"]
    PRSCH0,
    #[doc = "PRS Channel 1 selected"]
    PRSCH1,
    #[doc = "PRS Channel 2 selected"]
    PRSCH2,
    #[doc = "PRS Channel 3 selected"]
    PRSCH3,
    #[doc = "PRS Channel 4 selected"]
    PRSCH4,
    #[doc = "PRS Channel 5 selected"]
    PRSCH5,
    #[doc = "PRS Channel 6 selected"]
    PRSCH6,
    #[doc = "PRS Channel 7 selected"]
    PRSCH7,
    #[doc = "PRS Channel 8 selected"]
    PRSCH8,
    #[doc = "PRS Channel 9 selected"]
    PRSCH9,
    #[doc = "PRS Channel 10 selected"]
    PRSCH10,
    #[doc = "PRS Channel 11 selected"]
    PRSCH11,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl RXPRSSELR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            RXPRSSELR::PRSCH0 => 0,
            RXPRSSELR::PRSCH1 => 0x01,
            RXPRSSELR::PRSCH2 => 0x02,
            RXPRSSELR::PRSCH3 => 0x03,
            RXPRSSELR::PRSCH4 => 0x04,
            RXPRSSELR::PRSCH5 => 0x05,
            RXPRSSELR::PRSCH6 => 0x06,
            RXPRSSELR::PRSCH7 => 0x07,
            RXPRSSELR::PRSCH8 => 0x08,
            RXPRSSELR::PRSCH9 => 0x09,
            RXPRSSELR::PRSCH10 => 0x0a,
            RXPRSSELR::PRSCH11 => 0x0b,
            RXPRSSELR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> RXPRSSELR {
        match value {
            0 => RXPRSSELR::PRSCH0,
            1 => RXPRSSELR::PRSCH1,
            2 => RXPRSSELR::PRSCH2,
            3 => RXPRSSELR::PRSCH3,
            4 => RXPRSSELR::PRSCH4,
            5 => RXPRSSELR::PRSCH5,
            6 => RXPRSSELR::PRSCH6,
            7 => RXPRSSELR::PRSCH7,
            8 => RXPRSSELR::PRSCH8,
            9 => RXPRSSELR::PRSCH9,
            10 => RXPRSSELR::PRSCH10,
            11 => RXPRSSELR::PRSCH11,
            i => RXPRSSELR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `PRSCH0`"]
    #[inline]
    pub fn is_prsch0(&self) -> bool {
        *self == RXPRSSELR::PRSCH0
    }
    #[doc = "Checks if the value of the field is `PRSCH1`"]
    #[inline]
    pub fn is_prsch1(&self) -> bool {
        *self == RXPRSSELR::PRSCH1
    }
    #[doc = "Checks if the value of the field is `PRSCH2`"]
    #[inline]
    pub fn is_prsch2(&self) -> bool {
        *self == RXPRSSELR::PRSCH2
    }
    #[doc = "Checks if the value of the field is `PRSCH3`"]
    #[inline]
    pub fn is_prsch3(&self) -> bool {
        *self == RXPRSSELR::PRSCH3
    }
    #[doc = "Checks if the value of the field is `PRSCH4`"]
    #[inline]
    pub fn is_prsch4(&self) -> bool {
        *self == RXPRSSELR::PRSCH4
    }
    #[doc = "Checks if the value of the field is `PRSCH5`"]
    #[inline]
    pub fn is_prsch5(&self) -> bool {
        *self == RXPRSSELR::PRSCH5
    }
    #[doc = "Checks if the value of the field is `PRSCH6`"]
    #[inline]
    pub fn is_prsch6(&self) -> bool {
        *self == RXPRSSELR::PRSCH6
    }
    #[doc = "Checks if the value of the field is `PRSCH7`"]
    #[inline]
    pub fn is_prsch7(&self) -> bool {
        *self == RXPRSSELR::PRSCH7
    }
    #[doc = "Checks if the value of the field is `PRSCH8`"]
    #[inline]
    pub fn is_prsch8(&self) -> bool {
        *self == RXPRSSELR::PRSCH8
    }
    #[doc = "Checks if the value of the field is `PRSCH9`"]
    #[inline]
    pub fn is_prsch9(&self) -> bool {
        *self == RXPRSSELR::PRSCH9
    }
    #[doc = "Checks if the value of the field is `PRSCH10`"]
    #[inline]
    pub fn is_prsch10(&self) -> bool {
        *self == RXPRSSELR::PRSCH10
    }
    #[doc = "Checks if the value of the field is `PRSCH11`"]
    #[inline]
    pub fn is_prsch11(&self) -> bool {
        *self == RXPRSSELR::PRSCH11
    }
}
#[doc = r" Value of the field"]
pub struct RXPRSR {
    bits: bool,
}
impl RXPRSR {
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
#[doc = "Values that can be written to the field `RXPRSSEL`"]
pub enum RXPRSSELW {
    #[doc = "PRS Channel 0 selected"]
    PRSCH0,
    #[doc = "PRS Channel 1 selected"]
    PRSCH1,
    #[doc = "PRS Channel 2 selected"]
    PRSCH2,
    #[doc = "PRS Channel 3 selected"]
    PRSCH3,
    #[doc = "PRS Channel 4 selected"]
    PRSCH4,
    #[doc = "PRS Channel 5 selected"]
    PRSCH5,
    #[doc = "PRS Channel 6 selected"]
    PRSCH6,
    #[doc = "PRS Channel 7 selected"]
    PRSCH7,
    #[doc = "PRS Channel 8 selected"]
    PRSCH8,
    #[doc = "PRS Channel 9 selected"]
    PRSCH9,
    #[doc = "PRS Channel 10 selected"]
    PRSCH10,
    #[doc = "PRS Channel 11 selected"]
    PRSCH11,
}
impl RXPRSSELW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            RXPRSSELW::PRSCH0 => 0,
            RXPRSSELW::PRSCH1 => 1,
            RXPRSSELW::PRSCH2 => 2,
            RXPRSSELW::PRSCH3 => 3,
            RXPRSSELW::PRSCH4 => 4,
            RXPRSSELW::PRSCH5 => 5,
            RXPRSSELW::PRSCH6 => 6,
            RXPRSSELW::PRSCH7 => 7,
            RXPRSSELW::PRSCH8 => 8,
            RXPRSSELW::PRSCH9 => 9,
            RXPRSSELW::PRSCH10 => 10,
            RXPRSSELW::PRSCH11 => 11,
        }
    }
}
#[doc = r" Proxy"]
pub struct _RXPRSSELW<'a> {
    w: &'a mut W,
}
impl<'a> _RXPRSSELW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: RXPRSSELW) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "PRS Channel 0 selected"]
    #[inline]
    pub fn prsch0(self) -> &'a mut W {
        self.variant(RXPRSSELW::PRSCH0)
    }
    #[doc = "PRS Channel 1 selected"]
    #[inline]
    pub fn prsch1(self) -> &'a mut W {
        self.variant(RXPRSSELW::PRSCH1)
    }
    #[doc = "PRS Channel 2 selected"]
    #[inline]
    pub fn prsch2(self) -> &'a mut W {
        self.variant(RXPRSSELW::PRSCH2)
    }
    #[doc = "PRS Channel 3 selected"]
    #[inline]
    pub fn prsch3(self) -> &'a mut W {
        self.variant(RXPRSSELW::PRSCH3)
    }
    #[doc = "PRS Channel 4 selected"]
    #[inline]
    pub fn prsch4(self) -> &'a mut W {
        self.variant(RXPRSSELW::PRSCH4)
    }
    #[doc = "PRS Channel 5 selected"]
    #[inline]
    pub fn prsch5(self) -> &'a mut W {
        self.variant(RXPRSSELW::PRSCH5)
    }
    #[doc = "PRS Channel 6 selected"]
    #[inline]
    pub fn prsch6(self) -> &'a mut W {
        self.variant(RXPRSSELW::PRSCH6)
    }
    #[doc = "PRS Channel 7 selected"]
    #[inline]
    pub fn prsch7(self) -> &'a mut W {
        self.variant(RXPRSSELW::PRSCH7)
    }
    #[doc = "PRS Channel 8 selected"]
    #[inline]
    pub fn prsch8(self) -> &'a mut W {
        self.variant(RXPRSSELW::PRSCH8)
    }
    #[doc = "PRS Channel 9 selected"]
    #[inline]
    pub fn prsch9(self) -> &'a mut W {
        self.variant(RXPRSSELW::PRSCH9)
    }
    #[doc = "PRS Channel 10 selected"]
    #[inline]
    pub fn prsch10(self) -> &'a mut W {
        self.variant(RXPRSSELW::PRSCH10)
    }
    #[doc = "PRS Channel 11 selected"]
    #[inline]
    pub fn prsch11(self) -> &'a mut W {
        self.variant(RXPRSSELW::PRSCH11)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 0x0f;
        const OFFSET: u8 = 0;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _RXPRSW<'a> {
    w: &'a mut W,
}
impl<'a> _RXPRSW<'a> {
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
impl R {
    #[doc = r" Value of the register as raw bits"]
    #[inline]
    pub fn bits(&self) -> u32 {
        self.bits
    }
    #[doc = "Bits 0:3 - RX PRS Channel Select"]
    #[inline]
    pub fn rxprssel(&self) -> RXPRSSELR {
        RXPRSSELR::_from({
            const MASK: u8 = 0x0f;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bit 4 - PRS RX Enable"]
    #[inline]
    pub fn rxprs(&self) -> RXPRSR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 4;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        RXPRSR { bits }
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
    #[doc = "Bits 0:3 - RX PRS Channel Select"]
    #[inline]
    pub fn rxprssel(&mut self) -> _RXPRSSELW {
        _RXPRSSELW { w: self }
    }
    #[doc = "Bit 4 - PRS RX Enable"]
    #[inline]
    pub fn rxprs(&mut self) -> _RXPRSW {
        _RXPRSW { w: self }
    }
}

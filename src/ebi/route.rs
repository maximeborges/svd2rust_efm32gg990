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
pub struct EBIPENR {
    bits: bool,
}
impl EBIPENR {
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
pub struct CS0PENR {
    bits: bool,
}
impl CS0PENR {
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
pub struct CS1PENR {
    bits: bool,
}
impl CS1PENR {
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
pub struct CS2PENR {
    bits: bool,
}
impl CS2PENR {
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
pub struct CS3PENR {
    bits: bool,
}
impl CS3PENR {
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
pub struct ALEPENR {
    bits: bool,
}
impl ALEPENR {
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
pub struct ARDYPENR {
    bits: bool,
}
impl ARDYPENR {
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
pub struct BLPENR {
    bits: bool,
}
impl BLPENR {
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
pub struct NANDPENR {
    bits: bool,
}
impl NANDPENR {
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
#[doc = "Possible values of the field `ALB`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ALBR {
    #[doc = "Address lines from EBI_A[0] and upwards can be enabled via APEN."]
    A0,
    #[doc = "Address lines from EBI_A[8] and upwards can be enabled via APEN."]
    A8,
    #[doc = "Address lines from EBI_A[16] and upwards can be enabled via APEN."]
    A16,
    #[doc = "Address lines from EBI_A[24] and upwards can be enabled via APEN."]
    A24,
}
impl ALBR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            ALBR::A0 => 0,
            ALBR::A8 => 0x01,
            ALBR::A16 => 0x02,
            ALBR::A24 => 0x03,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> ALBR {
        match value {
            0 => ALBR::A0,
            1 => ALBR::A8,
            2 => ALBR::A16,
            3 => ALBR::A24,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `A0`"]
    #[inline]
    pub fn is_a0(&self) -> bool {
        *self == ALBR::A0
    }
    #[doc = "Checks if the value of the field is `A8`"]
    #[inline]
    pub fn is_a8(&self) -> bool {
        *self == ALBR::A8
    }
    #[doc = "Checks if the value of the field is `A16`"]
    #[inline]
    pub fn is_a16(&self) -> bool {
        *self == ALBR::A16
    }
    #[doc = "Checks if the value of the field is `A24`"]
    #[inline]
    pub fn is_a24(&self) -> bool {
        *self == ALBR::A24
    }
}
#[doc = "Possible values of the field `APEN`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum APENR {
    #[doc = "All EBI_A pins are disabled."]
    A0,
    #[doc = "EBI_A[4:L] pins enabled."]
    A5,
    #[doc = "EBI_A[5:L] pins enabled."]
    A6,
    #[doc = "EBI_A[6:L] pins enabled."]
    A7,
    #[doc = "EBI_A[7:L] pins enabled."]
    A8,
    #[doc = "EBI_A[8:L] pins enabled."]
    A9,
    #[doc = "EBI_A[9:L] pins enabled."]
    A10,
    #[doc = "EBI_A[10:L] pins enabled."]
    A11,
    #[doc = "EBI_A[11:L] pins enabled."]
    A12,
    #[doc = "EBI_A[12:L] pins enabled."]
    A13,
    #[doc = "EBI_A[13:L] pins enabled."]
    A14,
    #[doc = "EBI_A[14:L] pins enabled."]
    A15,
    #[doc = "EBI_A[15:L] pins enabled."]
    A16,
    #[doc = "EBI_A[16:L] pins enabled."]
    A17,
    #[doc = "EBI_A[17:L] pins enabled."]
    A18,
    #[doc = "EBI_A[18:L] pins enabled."]
    A19,
    #[doc = "EBI_A[19:L] pins enabled."]
    A20,
    #[doc = "EBI_A[20:L] pins enabled."]
    A21,
    #[doc = "EBI_A[21:L] pins enabled."]
    A22,
    #[doc = "EBI_A[22:L] pins enabled."]
    A23,
    #[doc = "EBI_A[23:L] pins enabled."]
    A24,
    #[doc = "EBI_A[24:L] pins enabled."]
    A25,
    #[doc = "EBI_A[25:L] pins enabled."]
    A26,
    #[doc = "EBI_A[26:L] pins enabled."]
    A27,
    #[doc = "EBI_A[27:L] pins enabled."]
    A28,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl APENR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            APENR::A0 => 0,
            APENR::A5 => 0x05,
            APENR::A6 => 0x06,
            APENR::A7 => 0x07,
            APENR::A8 => 0x08,
            APENR::A9 => 0x09,
            APENR::A10 => 0x0a,
            APENR::A11 => 0x0b,
            APENR::A12 => 0x0c,
            APENR::A13 => 0x0d,
            APENR::A14 => 0x0e,
            APENR::A15 => 0x0f,
            APENR::A16 => 0x10,
            APENR::A17 => 0x11,
            APENR::A18 => 0x12,
            APENR::A19 => 0x13,
            APENR::A20 => 0x14,
            APENR::A21 => 0x15,
            APENR::A22 => 0x16,
            APENR::A23 => 0x17,
            APENR::A24 => 0x18,
            APENR::A25 => 0x19,
            APENR::A26 => 0x1a,
            APENR::A27 => 0x1b,
            APENR::A28 => 0x1c,
            APENR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> APENR {
        match value {
            0 => APENR::A0,
            5 => APENR::A5,
            6 => APENR::A6,
            7 => APENR::A7,
            8 => APENR::A8,
            9 => APENR::A9,
            10 => APENR::A10,
            11 => APENR::A11,
            12 => APENR::A12,
            13 => APENR::A13,
            14 => APENR::A14,
            15 => APENR::A15,
            16 => APENR::A16,
            17 => APENR::A17,
            18 => APENR::A18,
            19 => APENR::A19,
            20 => APENR::A20,
            21 => APENR::A21,
            22 => APENR::A22,
            23 => APENR::A23,
            24 => APENR::A24,
            25 => APENR::A25,
            26 => APENR::A26,
            27 => APENR::A27,
            28 => APENR::A28,
            i => APENR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `A0`"]
    #[inline]
    pub fn is_a0(&self) -> bool {
        *self == APENR::A0
    }
    #[doc = "Checks if the value of the field is `A5`"]
    #[inline]
    pub fn is_a5(&self) -> bool {
        *self == APENR::A5
    }
    #[doc = "Checks if the value of the field is `A6`"]
    #[inline]
    pub fn is_a6(&self) -> bool {
        *self == APENR::A6
    }
    #[doc = "Checks if the value of the field is `A7`"]
    #[inline]
    pub fn is_a7(&self) -> bool {
        *self == APENR::A7
    }
    #[doc = "Checks if the value of the field is `A8`"]
    #[inline]
    pub fn is_a8(&self) -> bool {
        *self == APENR::A8
    }
    #[doc = "Checks if the value of the field is `A9`"]
    #[inline]
    pub fn is_a9(&self) -> bool {
        *self == APENR::A9
    }
    #[doc = "Checks if the value of the field is `A10`"]
    #[inline]
    pub fn is_a10(&self) -> bool {
        *self == APENR::A10
    }
    #[doc = "Checks if the value of the field is `A11`"]
    #[inline]
    pub fn is_a11(&self) -> bool {
        *self == APENR::A11
    }
    #[doc = "Checks if the value of the field is `A12`"]
    #[inline]
    pub fn is_a12(&self) -> bool {
        *self == APENR::A12
    }
    #[doc = "Checks if the value of the field is `A13`"]
    #[inline]
    pub fn is_a13(&self) -> bool {
        *self == APENR::A13
    }
    #[doc = "Checks if the value of the field is `A14`"]
    #[inline]
    pub fn is_a14(&self) -> bool {
        *self == APENR::A14
    }
    #[doc = "Checks if the value of the field is `A15`"]
    #[inline]
    pub fn is_a15(&self) -> bool {
        *self == APENR::A15
    }
    #[doc = "Checks if the value of the field is `A16`"]
    #[inline]
    pub fn is_a16(&self) -> bool {
        *self == APENR::A16
    }
    #[doc = "Checks if the value of the field is `A17`"]
    #[inline]
    pub fn is_a17(&self) -> bool {
        *self == APENR::A17
    }
    #[doc = "Checks if the value of the field is `A18`"]
    #[inline]
    pub fn is_a18(&self) -> bool {
        *self == APENR::A18
    }
    #[doc = "Checks if the value of the field is `A19`"]
    #[inline]
    pub fn is_a19(&self) -> bool {
        *self == APENR::A19
    }
    #[doc = "Checks if the value of the field is `A20`"]
    #[inline]
    pub fn is_a20(&self) -> bool {
        *self == APENR::A20
    }
    #[doc = "Checks if the value of the field is `A21`"]
    #[inline]
    pub fn is_a21(&self) -> bool {
        *self == APENR::A21
    }
    #[doc = "Checks if the value of the field is `A22`"]
    #[inline]
    pub fn is_a22(&self) -> bool {
        *self == APENR::A22
    }
    #[doc = "Checks if the value of the field is `A23`"]
    #[inline]
    pub fn is_a23(&self) -> bool {
        *self == APENR::A23
    }
    #[doc = "Checks if the value of the field is `A24`"]
    #[inline]
    pub fn is_a24(&self) -> bool {
        *self == APENR::A24
    }
    #[doc = "Checks if the value of the field is `A25`"]
    #[inline]
    pub fn is_a25(&self) -> bool {
        *self == APENR::A25
    }
    #[doc = "Checks if the value of the field is `A26`"]
    #[inline]
    pub fn is_a26(&self) -> bool {
        *self == APENR::A26
    }
    #[doc = "Checks if the value of the field is `A27`"]
    #[inline]
    pub fn is_a27(&self) -> bool {
        *self == APENR::A27
    }
    #[doc = "Checks if the value of the field is `A28`"]
    #[inline]
    pub fn is_a28(&self) -> bool {
        *self == APENR::A28
    }
}
#[doc = r" Value of the field"]
pub struct TFTPENR {
    bits: bool,
}
impl TFTPENR {
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
pub struct DATAENPENR {
    bits: bool,
}
impl DATAENPENR {
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
pub struct CSTFTPENR {
    bits: bool,
}
impl CSTFTPENR {
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
pub struct _EBIPENW<'a> {
    w: &'a mut W,
}
impl<'a> _EBIPENW<'a> {
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
pub struct _CS0PENW<'a> {
    w: &'a mut W,
}
impl<'a> _CS0PENW<'a> {
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
pub struct _CS1PENW<'a> {
    w: &'a mut W,
}
impl<'a> _CS1PENW<'a> {
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
pub struct _CS2PENW<'a> {
    w: &'a mut W,
}
impl<'a> _CS2PENW<'a> {
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
pub struct _CS3PENW<'a> {
    w: &'a mut W,
}
impl<'a> _CS3PENW<'a> {
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
pub struct _ALEPENW<'a> {
    w: &'a mut W,
}
impl<'a> _ALEPENW<'a> {
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
pub struct _ARDYPENW<'a> {
    w: &'a mut W,
}
impl<'a> _ARDYPENW<'a> {
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
pub struct _BLPENW<'a> {
    w: &'a mut W,
}
impl<'a> _BLPENW<'a> {
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
pub struct _NANDPENW<'a> {
    w: &'a mut W,
}
impl<'a> _NANDPENW<'a> {
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
#[doc = "Values that can be written to the field `ALB`"]
pub enum ALBW {
    #[doc = "Address lines from EBI_A[0] and upwards can be enabled via APEN."]
    A0,
    #[doc = "Address lines from EBI_A[8] and upwards can be enabled via APEN."]
    A8,
    #[doc = "Address lines from EBI_A[16] and upwards can be enabled via APEN."]
    A16,
    #[doc = "Address lines from EBI_A[24] and upwards can be enabled via APEN."]
    A24,
}
impl ALBW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            ALBW::A0 => 0,
            ALBW::A8 => 1,
            ALBW::A16 => 2,
            ALBW::A24 => 3,
        }
    }
}
#[doc = r" Proxy"]
pub struct _ALBW<'a> {
    w: &'a mut W,
}
impl<'a> _ALBW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: ALBW) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "Address lines from EBI_A[0] and upwards can be enabled via APEN."]
    #[inline]
    pub fn a0(self) -> &'a mut W {
        self.variant(ALBW::A0)
    }
    #[doc = "Address lines from EBI_A[8] and upwards can be enabled via APEN."]
    #[inline]
    pub fn a8(self) -> &'a mut W {
        self.variant(ALBW::A8)
    }
    #[doc = "Address lines from EBI_A[16] and upwards can be enabled via APEN."]
    #[inline]
    pub fn a16(self) -> &'a mut W {
        self.variant(ALBW::A16)
    }
    #[doc = "Address lines from EBI_A[24] and upwards can be enabled via APEN."]
    #[inline]
    pub fn a24(self) -> &'a mut W {
        self.variant(ALBW::A24)
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
#[doc = "Values that can be written to the field `APEN`"]
pub enum APENW {
    #[doc = "All EBI_A pins are disabled."]
    A0,
    #[doc = "EBI_A[4:L] pins enabled."]
    A5,
    #[doc = "EBI_A[5:L] pins enabled."]
    A6,
    #[doc = "EBI_A[6:L] pins enabled."]
    A7,
    #[doc = "EBI_A[7:L] pins enabled."]
    A8,
    #[doc = "EBI_A[8:L] pins enabled."]
    A9,
    #[doc = "EBI_A[9:L] pins enabled."]
    A10,
    #[doc = "EBI_A[10:L] pins enabled."]
    A11,
    #[doc = "EBI_A[11:L] pins enabled."]
    A12,
    #[doc = "EBI_A[12:L] pins enabled."]
    A13,
    #[doc = "EBI_A[13:L] pins enabled."]
    A14,
    #[doc = "EBI_A[14:L] pins enabled."]
    A15,
    #[doc = "EBI_A[15:L] pins enabled."]
    A16,
    #[doc = "EBI_A[16:L] pins enabled."]
    A17,
    #[doc = "EBI_A[17:L] pins enabled."]
    A18,
    #[doc = "EBI_A[18:L] pins enabled."]
    A19,
    #[doc = "EBI_A[19:L] pins enabled."]
    A20,
    #[doc = "EBI_A[20:L] pins enabled."]
    A21,
    #[doc = "EBI_A[21:L] pins enabled."]
    A22,
    #[doc = "EBI_A[22:L] pins enabled."]
    A23,
    #[doc = "EBI_A[23:L] pins enabled."]
    A24,
    #[doc = "EBI_A[24:L] pins enabled."]
    A25,
    #[doc = "EBI_A[25:L] pins enabled."]
    A26,
    #[doc = "EBI_A[26:L] pins enabled."]
    A27,
    #[doc = "EBI_A[27:L] pins enabled."]
    A28,
}
impl APENW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            APENW::A0 => 0,
            APENW::A5 => 5,
            APENW::A6 => 6,
            APENW::A7 => 7,
            APENW::A8 => 8,
            APENW::A9 => 9,
            APENW::A10 => 10,
            APENW::A11 => 11,
            APENW::A12 => 12,
            APENW::A13 => 13,
            APENW::A14 => 14,
            APENW::A15 => 15,
            APENW::A16 => 16,
            APENW::A17 => 17,
            APENW::A18 => 18,
            APENW::A19 => 19,
            APENW::A20 => 20,
            APENW::A21 => 21,
            APENW::A22 => 22,
            APENW::A23 => 23,
            APENW::A24 => 24,
            APENW::A25 => 25,
            APENW::A26 => 26,
            APENW::A27 => 27,
            APENW::A28 => 28,
        }
    }
}
#[doc = r" Proxy"]
pub struct _APENW<'a> {
    w: &'a mut W,
}
impl<'a> _APENW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: APENW) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "All EBI_A pins are disabled."]
    #[inline]
    pub fn a0(self) -> &'a mut W {
        self.variant(APENW::A0)
    }
    #[doc = "EBI_A[4:L] pins enabled."]
    #[inline]
    pub fn a5(self) -> &'a mut W {
        self.variant(APENW::A5)
    }
    #[doc = "EBI_A[5:L] pins enabled."]
    #[inline]
    pub fn a6(self) -> &'a mut W {
        self.variant(APENW::A6)
    }
    #[doc = "EBI_A[6:L] pins enabled."]
    #[inline]
    pub fn a7(self) -> &'a mut W {
        self.variant(APENW::A7)
    }
    #[doc = "EBI_A[7:L] pins enabled."]
    #[inline]
    pub fn a8(self) -> &'a mut W {
        self.variant(APENW::A8)
    }
    #[doc = "EBI_A[8:L] pins enabled."]
    #[inline]
    pub fn a9(self) -> &'a mut W {
        self.variant(APENW::A9)
    }
    #[doc = "EBI_A[9:L] pins enabled."]
    #[inline]
    pub fn a10(self) -> &'a mut W {
        self.variant(APENW::A10)
    }
    #[doc = "EBI_A[10:L] pins enabled."]
    #[inline]
    pub fn a11(self) -> &'a mut W {
        self.variant(APENW::A11)
    }
    #[doc = "EBI_A[11:L] pins enabled."]
    #[inline]
    pub fn a12(self) -> &'a mut W {
        self.variant(APENW::A12)
    }
    #[doc = "EBI_A[12:L] pins enabled."]
    #[inline]
    pub fn a13(self) -> &'a mut W {
        self.variant(APENW::A13)
    }
    #[doc = "EBI_A[13:L] pins enabled."]
    #[inline]
    pub fn a14(self) -> &'a mut W {
        self.variant(APENW::A14)
    }
    #[doc = "EBI_A[14:L] pins enabled."]
    #[inline]
    pub fn a15(self) -> &'a mut W {
        self.variant(APENW::A15)
    }
    #[doc = "EBI_A[15:L] pins enabled."]
    #[inline]
    pub fn a16(self) -> &'a mut W {
        self.variant(APENW::A16)
    }
    #[doc = "EBI_A[16:L] pins enabled."]
    #[inline]
    pub fn a17(self) -> &'a mut W {
        self.variant(APENW::A17)
    }
    #[doc = "EBI_A[17:L] pins enabled."]
    #[inline]
    pub fn a18(self) -> &'a mut W {
        self.variant(APENW::A18)
    }
    #[doc = "EBI_A[18:L] pins enabled."]
    #[inline]
    pub fn a19(self) -> &'a mut W {
        self.variant(APENW::A19)
    }
    #[doc = "EBI_A[19:L] pins enabled."]
    #[inline]
    pub fn a20(self) -> &'a mut W {
        self.variant(APENW::A20)
    }
    #[doc = "EBI_A[20:L] pins enabled."]
    #[inline]
    pub fn a21(self) -> &'a mut W {
        self.variant(APENW::A21)
    }
    #[doc = "EBI_A[21:L] pins enabled."]
    #[inline]
    pub fn a22(self) -> &'a mut W {
        self.variant(APENW::A22)
    }
    #[doc = "EBI_A[22:L] pins enabled."]
    #[inline]
    pub fn a23(self) -> &'a mut W {
        self.variant(APENW::A23)
    }
    #[doc = "EBI_A[23:L] pins enabled."]
    #[inline]
    pub fn a24(self) -> &'a mut W {
        self.variant(APENW::A24)
    }
    #[doc = "EBI_A[24:L] pins enabled."]
    #[inline]
    pub fn a25(self) -> &'a mut W {
        self.variant(APENW::A25)
    }
    #[doc = "EBI_A[25:L] pins enabled."]
    #[inline]
    pub fn a26(self) -> &'a mut W {
        self.variant(APENW::A26)
    }
    #[doc = "EBI_A[26:L] pins enabled."]
    #[inline]
    pub fn a27(self) -> &'a mut W {
        self.variant(APENW::A27)
    }
    #[doc = "EBI_A[27:L] pins enabled."]
    #[inline]
    pub fn a28(self) -> &'a mut W {
        self.variant(APENW::A28)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 0x1f;
        const OFFSET: u8 = 18;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _TFTPENW<'a> {
    w: &'a mut W,
}
impl<'a> _TFTPENW<'a> {
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
#[doc = r" Proxy"]
pub struct _DATAENPENW<'a> {
    w: &'a mut W,
}
impl<'a> _DATAENPENW<'a> {
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
        const OFFSET: u8 = 25;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _CSTFTPENW<'a> {
    w: &'a mut W,
}
impl<'a> _CSTFTPENW<'a> {
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
        const OFFSET: u8 = 26;
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
    #[doc = "Bit 0 - EBI Pin Enable"]
    #[inline]
    pub fn ebipen(&self) -> EBIPENR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        EBIPENR { bits }
    }
    #[doc = "Bit 1 - EBI_CS0 Pin Enable"]
    #[inline]
    pub fn cs0pen(&self) -> CS0PENR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 1;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        CS0PENR { bits }
    }
    #[doc = "Bit 2 - EBI_CS1 Pin Enable"]
    #[inline]
    pub fn cs1pen(&self) -> CS1PENR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 2;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        CS1PENR { bits }
    }
    #[doc = "Bit 3 - EBI_CS2 Pin Enable"]
    #[inline]
    pub fn cs2pen(&self) -> CS2PENR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 3;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        CS2PENR { bits }
    }
    #[doc = "Bit 4 - EBI_CS3 Pin Enable"]
    #[inline]
    pub fn cs3pen(&self) -> CS3PENR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 4;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        CS3PENR { bits }
    }
    #[doc = "Bit 5 - EBI_ALE Pin Enable"]
    #[inline]
    pub fn alepen(&self) -> ALEPENR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 5;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        ALEPENR { bits }
    }
    #[doc = "Bit 6 - EBI_ARDY Pin Enable"]
    #[inline]
    pub fn ardypen(&self) -> ARDYPENR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 6;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        ARDYPENR { bits }
    }
    #[doc = "Bit 7 - EBI_BL[1:0] Pin Enable"]
    #[inline]
    pub fn blpen(&self) -> BLPENR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 7;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        BLPENR { bits }
    }
    #[doc = "Bit 12 - NANDRE and NANDWE Pin Enable"]
    #[inline]
    pub fn nandpen(&self) -> NANDPENR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 12;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        NANDPENR { bits }
    }
    #[doc = "Bits 16:17 - Sets the lower bound for EBI_A enabling"]
    #[inline]
    pub fn alb(&self) -> ALBR {
        ALBR::_from({
            const MASK: u8 = 0x03;
            const OFFSET: u8 = 16;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 18:22 - EBI_A Pin Enable"]
    #[inline]
    pub fn apen(&self) -> APENR {
        APENR::_from({
            const MASK: u8 = 0x1f;
            const OFFSET: u8 = 18;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bit 24 - EBI_TFT Pin Enable"]
    #[inline]
    pub fn tftpen(&self) -> TFTPENR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 24;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        TFTPENR { bits }
    }
    #[doc = "Bit 25 - EBI_TFT Pin Enable"]
    #[inline]
    pub fn dataenpen(&self) -> DATAENPENR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 25;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        DATAENPENR { bits }
    }
    #[doc = "Bit 26 - EBI_CSTFT Pin Enable"]
    #[inline]
    pub fn cstftpen(&self) -> CSTFTPENR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 26;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        CSTFTPENR { bits }
    }
    #[doc = "Bits 28:30 - I/O Location"]
    #[inline]
    pub fn location(&self) -> LOCATIONR {
        LOCATIONR::_from({
            const MASK: u8 = 0x07;
            const OFFSET: u8 = 28;
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
    #[doc = "Bit 0 - EBI Pin Enable"]
    #[inline]
    pub fn ebipen(&mut self) -> _EBIPENW {
        _EBIPENW { w: self }
    }
    #[doc = "Bit 1 - EBI_CS0 Pin Enable"]
    #[inline]
    pub fn cs0pen(&mut self) -> _CS0PENW {
        _CS0PENW { w: self }
    }
    #[doc = "Bit 2 - EBI_CS1 Pin Enable"]
    #[inline]
    pub fn cs1pen(&mut self) -> _CS1PENW {
        _CS1PENW { w: self }
    }
    #[doc = "Bit 3 - EBI_CS2 Pin Enable"]
    #[inline]
    pub fn cs2pen(&mut self) -> _CS2PENW {
        _CS2PENW { w: self }
    }
    #[doc = "Bit 4 - EBI_CS3 Pin Enable"]
    #[inline]
    pub fn cs3pen(&mut self) -> _CS3PENW {
        _CS3PENW { w: self }
    }
    #[doc = "Bit 5 - EBI_ALE Pin Enable"]
    #[inline]
    pub fn alepen(&mut self) -> _ALEPENW {
        _ALEPENW { w: self }
    }
    #[doc = "Bit 6 - EBI_ARDY Pin Enable"]
    #[inline]
    pub fn ardypen(&mut self) -> _ARDYPENW {
        _ARDYPENW { w: self }
    }
    #[doc = "Bit 7 - EBI_BL[1:0] Pin Enable"]
    #[inline]
    pub fn blpen(&mut self) -> _BLPENW {
        _BLPENW { w: self }
    }
    #[doc = "Bit 12 - NANDRE and NANDWE Pin Enable"]
    #[inline]
    pub fn nandpen(&mut self) -> _NANDPENW {
        _NANDPENW { w: self }
    }
    #[doc = "Bits 16:17 - Sets the lower bound for EBI_A enabling"]
    #[inline]
    pub fn alb(&mut self) -> _ALBW {
        _ALBW { w: self }
    }
    #[doc = "Bits 18:22 - EBI_A Pin Enable"]
    #[inline]
    pub fn apen(&mut self) -> _APENW {
        _APENW { w: self }
    }
    #[doc = "Bit 24 - EBI_TFT Pin Enable"]
    #[inline]
    pub fn tftpen(&mut self) -> _TFTPENW {
        _TFTPENW { w: self }
    }
    #[doc = "Bit 25 - EBI_TFT Pin Enable"]
    #[inline]
    pub fn dataenpen(&mut self) -> _DATAENPENW {
        _DATAENPENW { w: self }
    }
    #[doc = "Bit 26 - EBI_CSTFT Pin Enable"]
    #[inline]
    pub fn cstftpen(&mut self) -> _CSTFTPENW {
        _CSTFTPENW { w: self }
    }
    #[doc = "Bits 28:30 - I/O Location"]
    #[inline]
    pub fn location(&mut self) -> _LOCATIONW {
        _LOCATIONW { w: self }
    }
}

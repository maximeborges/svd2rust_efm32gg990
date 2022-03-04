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
    #[doc = "EBI_AD drives 8 bit data, 8 bit address, ALE not used. Extended address bits can be enabled on EBI_A in the EBI_ROUTE register."]
    D8A8,
    #[doc = "EBI_AD drives 16 bit data, 16 bit address, ALE is used for address latching. Extended address bits can be enabled on EBI_A in the EBI_ROUTE register."]
    D16A16ALE,
    #[doc = "EBI_AD drives 8 bit data, 24 bit address, ALE is used for address latching. Extended address bits can be enabled on EBI_A in the EBI_ROUTE register."]
    D8A24ALE,
    #[doc = "EBI_AD drives 16 bit data, ALE not used. Extended address bits can be enabled on EBI_A in the EBI_ROUTE register."]
    D16,
}
impl MODER {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            MODER::D8A8 => 0,
            MODER::D16A16ALE => 0x01,
            MODER::D8A24ALE => 0x02,
            MODER::D16 => 0x03,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> MODER {
        match value {
            0 => MODER::D8A8,
            1 => MODER::D16A16ALE,
            2 => MODER::D8A24ALE,
            3 => MODER::D16,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `D8A8`"]
    #[inline]
    pub fn is_d8a8(&self) -> bool {
        *self == MODER::D8A8
    }
    #[doc = "Checks if the value of the field is `D16A16ALE`"]
    #[inline]
    pub fn is_d16a16ale(&self) -> bool {
        *self == MODER::D16A16ALE
    }
    #[doc = "Checks if the value of the field is `D8A24ALE`"]
    #[inline]
    pub fn is_d8a24ale(&self) -> bool {
        *self == MODER::D8A24ALE
    }
    #[doc = "Checks if the value of the field is `D16`"]
    #[inline]
    pub fn is_d16(&self) -> bool {
        *self == MODER::D16
    }
}
#[doc = "Possible values of the field `MODE1`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MODE1R {
    #[doc = "EBI_AD drives 8 bit data, 8 bit address, ALE not used. Extended address bits can be enabled on EBI_A in the EBI_ROUTE register."]
    D8A8,
    #[doc = "EBI_AD drives 16 bit data, 16 bit address, ALE is used for address latching. Extended address bits can be enabled on EBI_A in the EBI_ROUTE register."]
    D16A16ALE,
    #[doc = "EBI_AD drives 8 bit data, 24 bit address, ALE is used for address latching. Extended address bits can be enabled on EBI_A in the EBI_ROUTE register."]
    D8A24ALE,
    #[doc = "EBI_AD drives 16 bit data, ALE not used. Extended address bits can be enabled on EBI_A in the EBI_ROUTE register."]
    D16,
}
impl MODE1R {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            MODE1R::D8A8 => 0,
            MODE1R::D16A16ALE => 0x01,
            MODE1R::D8A24ALE => 0x02,
            MODE1R::D16 => 0x03,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> MODE1R {
        match value {
            0 => MODE1R::D8A8,
            1 => MODE1R::D16A16ALE,
            2 => MODE1R::D8A24ALE,
            3 => MODE1R::D16,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `D8A8`"]
    #[inline]
    pub fn is_d8a8(&self) -> bool {
        *self == MODE1R::D8A8
    }
    #[doc = "Checks if the value of the field is `D16A16ALE`"]
    #[inline]
    pub fn is_d16a16ale(&self) -> bool {
        *self == MODE1R::D16A16ALE
    }
    #[doc = "Checks if the value of the field is `D8A24ALE`"]
    #[inline]
    pub fn is_d8a24ale(&self) -> bool {
        *self == MODE1R::D8A24ALE
    }
    #[doc = "Checks if the value of the field is `D16`"]
    #[inline]
    pub fn is_d16(&self) -> bool {
        *self == MODE1R::D16
    }
}
#[doc = "Possible values of the field `MODE2`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MODE2R {
    #[doc = "EBI_AD drives 8 bit data, 8 bit address, ALE not used. Extended address bits can be enabled on EBI_A in the EBI_ROUTE register."]
    D8A8,
    #[doc = "EBI_AD drives 16 bit data, 16 bit address, ALE is used for address latching. Extended address bits can be enabled on EBI_A in the EBI_ROUTE register."]
    D16A16ALE,
    #[doc = "EBI_AD drives 8 bit data, 24 bit address, ALE is used for address latching. Extended address bits can be enabled on EBI_A in the EBI_ROUTE register."]
    D8A24ALE,
    #[doc = "EBI_AD drives 16 bit data, ALE not used. Extended address bits can be enabled on EBI_A in the EBI_ROUTE register."]
    D16,
}
impl MODE2R {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            MODE2R::D8A8 => 0,
            MODE2R::D16A16ALE => 0x01,
            MODE2R::D8A24ALE => 0x02,
            MODE2R::D16 => 0x03,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> MODE2R {
        match value {
            0 => MODE2R::D8A8,
            1 => MODE2R::D16A16ALE,
            2 => MODE2R::D8A24ALE,
            3 => MODE2R::D16,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `D8A8`"]
    #[inline]
    pub fn is_d8a8(&self) -> bool {
        *self == MODE2R::D8A8
    }
    #[doc = "Checks if the value of the field is `D16A16ALE`"]
    #[inline]
    pub fn is_d16a16ale(&self) -> bool {
        *self == MODE2R::D16A16ALE
    }
    #[doc = "Checks if the value of the field is `D8A24ALE`"]
    #[inline]
    pub fn is_d8a24ale(&self) -> bool {
        *self == MODE2R::D8A24ALE
    }
    #[doc = "Checks if the value of the field is `D16`"]
    #[inline]
    pub fn is_d16(&self) -> bool {
        *self == MODE2R::D16
    }
}
#[doc = "Possible values of the field `MODE3`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MODE3R {
    #[doc = "EBI_AD drives 8 bit data, 8 bit address, ALE not used. Extended address bits can be enabled on EBI_A in the EBI_ROUTE register."]
    D8A8,
    #[doc = "EBI_AD drives 16 bit data, 16 bit address, ALE is used for address latching. Extended address bits can be enabled on EBI_A in the EBI_ROUTE register."]
    D16A16ALE,
    #[doc = "EBI_AD drives 8 bit data, 24 bit address, ALE is used for address latching. Extended address bits can be enabled on EBI_A in the EBI_ROUTE register."]
    D8A24ALE,
    #[doc = "EBI_AD drives 16 bit data, ALE not used. Extended address bits can be enabled on EBI_A in the EBI_ROUTE register."]
    D16,
}
impl MODE3R {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            MODE3R::D8A8 => 0,
            MODE3R::D16A16ALE => 0x01,
            MODE3R::D8A24ALE => 0x02,
            MODE3R::D16 => 0x03,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> MODE3R {
        match value {
            0 => MODE3R::D8A8,
            1 => MODE3R::D16A16ALE,
            2 => MODE3R::D8A24ALE,
            3 => MODE3R::D16,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `D8A8`"]
    #[inline]
    pub fn is_d8a8(&self) -> bool {
        *self == MODE3R::D8A8
    }
    #[doc = "Checks if the value of the field is `D16A16ALE`"]
    #[inline]
    pub fn is_d16a16ale(&self) -> bool {
        *self == MODE3R::D16A16ALE
    }
    #[doc = "Checks if the value of the field is `D8A24ALE`"]
    #[inline]
    pub fn is_d8a24ale(&self) -> bool {
        *self == MODE3R::D8A24ALE
    }
    #[doc = "Checks if the value of the field is `D16`"]
    #[inline]
    pub fn is_d16(&self) -> bool {
        *self == MODE3R::D16
    }
}
#[doc = r" Value of the field"]
pub struct BANK0ENR {
    bits: bool,
}
impl BANK0ENR {
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
pub struct BANK1ENR {
    bits: bool,
}
impl BANK1ENR {
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
pub struct BANK2ENR {
    bits: bool,
}
impl BANK2ENR {
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
pub struct BANK3ENR {
    bits: bool,
}
impl BANK3ENR {
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
pub struct NOIDLER {
    bits: bool,
}
impl NOIDLER {
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
pub struct NOIDLE1R {
    bits: bool,
}
impl NOIDLE1R {
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
pub struct NOIDLE2R {
    bits: bool,
}
impl NOIDLE2R {
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
pub struct NOIDLE3R {
    bits: bool,
}
impl NOIDLE3R {
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
pub struct ARDYENR {
    bits: bool,
}
impl ARDYENR {
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
pub struct ARDYTODISR {
    bits: bool,
}
impl ARDYTODISR {
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
pub struct ARDY1ENR {
    bits: bool,
}
impl ARDY1ENR {
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
pub struct ARDYTO1DISR {
    bits: bool,
}
impl ARDYTO1DISR {
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
pub struct ARDY2ENR {
    bits: bool,
}
impl ARDY2ENR {
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
pub struct ARDYTO2DISR {
    bits: bool,
}
impl ARDYTO2DISR {
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
pub struct ARDY3ENR {
    bits: bool,
}
impl ARDY3ENR {
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
pub struct ARDYTO3DISR {
    bits: bool,
}
impl ARDYTO3DISR {
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
pub struct BLR {
    bits: bool,
}
impl BLR {
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
pub struct BL1R {
    bits: bool,
}
impl BL1R {
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
pub struct BL2R {
    bits: bool,
}
impl BL2R {
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
pub struct BL3R {
    bits: bool,
}
impl BL3R {
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
pub struct ITSR {
    bits: bool,
}
impl ITSR {
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
pub struct ALTMAPR {
    bits: bool,
}
impl ALTMAPR {
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
#[doc = "Values that can be written to the field `MODE`"]
pub enum MODEW {
    #[doc = "EBI_AD drives 8 bit data, 8 bit address, ALE not used. Extended address bits can be enabled on EBI_A in the EBI_ROUTE register."]
    D8A8,
    #[doc = "EBI_AD drives 16 bit data, 16 bit address, ALE is used for address latching. Extended address bits can be enabled on EBI_A in the EBI_ROUTE register."]
    D16A16ALE,
    #[doc = "EBI_AD drives 8 bit data, 24 bit address, ALE is used for address latching. Extended address bits can be enabled on EBI_A in the EBI_ROUTE register."]
    D8A24ALE,
    #[doc = "EBI_AD drives 16 bit data, ALE not used. Extended address bits can be enabled on EBI_A in the EBI_ROUTE register."]
    D16,
}
impl MODEW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            MODEW::D8A8 => 0,
            MODEW::D16A16ALE => 1,
            MODEW::D8A24ALE => 2,
            MODEW::D16 => 3,
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
    #[doc = "EBI_AD drives 8 bit data, 8 bit address, ALE not used. Extended address bits can be enabled on EBI_A in the EBI_ROUTE register."]
    #[inline]
    pub fn d8a8(self) -> &'a mut W {
        self.variant(MODEW::D8A8)
    }
    #[doc = "EBI_AD drives 16 bit data, 16 bit address, ALE is used for address latching. Extended address bits can be enabled on EBI_A in the EBI_ROUTE register."]
    #[inline]
    pub fn d16a16ale(self) -> &'a mut W {
        self.variant(MODEW::D16A16ALE)
    }
    #[doc = "EBI_AD drives 8 bit data, 24 bit address, ALE is used for address latching. Extended address bits can be enabled on EBI_A in the EBI_ROUTE register."]
    #[inline]
    pub fn d8a24ale(self) -> &'a mut W {
        self.variant(MODEW::D8A24ALE)
    }
    #[doc = "EBI_AD drives 16 bit data, ALE not used. Extended address bits can be enabled on EBI_A in the EBI_ROUTE register."]
    #[inline]
    pub fn d16(self) -> &'a mut W {
        self.variant(MODEW::D16)
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
#[doc = "Values that can be written to the field `MODE1`"]
pub enum MODE1W {
    #[doc = "EBI_AD drives 8 bit data, 8 bit address, ALE not used. Extended address bits can be enabled on EBI_A in the EBI_ROUTE register."]
    D8A8,
    #[doc = "EBI_AD drives 16 bit data, 16 bit address, ALE is used for address latching. Extended address bits can be enabled on EBI_A in the EBI_ROUTE register."]
    D16A16ALE,
    #[doc = "EBI_AD drives 8 bit data, 24 bit address, ALE is used for address latching. Extended address bits can be enabled on EBI_A in the EBI_ROUTE register."]
    D8A24ALE,
    #[doc = "EBI_AD drives 16 bit data, ALE not used. Extended address bits can be enabled on EBI_A in the EBI_ROUTE register."]
    D16,
}
impl MODE1W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            MODE1W::D8A8 => 0,
            MODE1W::D16A16ALE => 1,
            MODE1W::D8A24ALE => 2,
            MODE1W::D16 => 3,
        }
    }
}
#[doc = r" Proxy"]
pub struct _MODE1W<'a> {
    w: &'a mut W,
}
impl<'a> _MODE1W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: MODE1W) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "EBI_AD drives 8 bit data, 8 bit address, ALE not used. Extended address bits can be enabled on EBI_A in the EBI_ROUTE register."]
    #[inline]
    pub fn d8a8(self) -> &'a mut W {
        self.variant(MODE1W::D8A8)
    }
    #[doc = "EBI_AD drives 16 bit data, 16 bit address, ALE is used for address latching. Extended address bits can be enabled on EBI_A in the EBI_ROUTE register."]
    #[inline]
    pub fn d16a16ale(self) -> &'a mut W {
        self.variant(MODE1W::D16A16ALE)
    }
    #[doc = "EBI_AD drives 8 bit data, 24 bit address, ALE is used for address latching. Extended address bits can be enabled on EBI_A in the EBI_ROUTE register."]
    #[inline]
    pub fn d8a24ale(self) -> &'a mut W {
        self.variant(MODE1W::D8A24ALE)
    }
    #[doc = "EBI_AD drives 16 bit data, ALE not used. Extended address bits can be enabled on EBI_A in the EBI_ROUTE register."]
    #[inline]
    pub fn d16(self) -> &'a mut W {
        self.variant(MODE1W::D16)
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
#[doc = "Values that can be written to the field `MODE2`"]
pub enum MODE2W {
    #[doc = "EBI_AD drives 8 bit data, 8 bit address, ALE not used. Extended address bits can be enabled on EBI_A in the EBI_ROUTE register."]
    D8A8,
    #[doc = "EBI_AD drives 16 bit data, 16 bit address, ALE is used for address latching. Extended address bits can be enabled on EBI_A in the EBI_ROUTE register."]
    D16A16ALE,
    #[doc = "EBI_AD drives 8 bit data, 24 bit address, ALE is used for address latching. Extended address bits can be enabled on EBI_A in the EBI_ROUTE register."]
    D8A24ALE,
    #[doc = "EBI_AD drives 16 bit data, ALE not used. Extended address bits can be enabled on EBI_A in the EBI_ROUTE register."]
    D16,
}
impl MODE2W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            MODE2W::D8A8 => 0,
            MODE2W::D16A16ALE => 1,
            MODE2W::D8A24ALE => 2,
            MODE2W::D16 => 3,
        }
    }
}
#[doc = r" Proxy"]
pub struct _MODE2W<'a> {
    w: &'a mut W,
}
impl<'a> _MODE2W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: MODE2W) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "EBI_AD drives 8 bit data, 8 bit address, ALE not used. Extended address bits can be enabled on EBI_A in the EBI_ROUTE register."]
    #[inline]
    pub fn d8a8(self) -> &'a mut W {
        self.variant(MODE2W::D8A8)
    }
    #[doc = "EBI_AD drives 16 bit data, 16 bit address, ALE is used for address latching. Extended address bits can be enabled on EBI_A in the EBI_ROUTE register."]
    #[inline]
    pub fn d16a16ale(self) -> &'a mut W {
        self.variant(MODE2W::D16A16ALE)
    }
    #[doc = "EBI_AD drives 8 bit data, 24 bit address, ALE is used for address latching. Extended address bits can be enabled on EBI_A in the EBI_ROUTE register."]
    #[inline]
    pub fn d8a24ale(self) -> &'a mut W {
        self.variant(MODE2W::D8A24ALE)
    }
    #[doc = "EBI_AD drives 16 bit data, ALE not used. Extended address bits can be enabled on EBI_A in the EBI_ROUTE register."]
    #[inline]
    pub fn d16(self) -> &'a mut W {
        self.variant(MODE2W::D16)
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
#[doc = "Values that can be written to the field `MODE3`"]
pub enum MODE3W {
    #[doc = "EBI_AD drives 8 bit data, 8 bit address, ALE not used. Extended address bits can be enabled on EBI_A in the EBI_ROUTE register."]
    D8A8,
    #[doc = "EBI_AD drives 16 bit data, 16 bit address, ALE is used for address latching. Extended address bits can be enabled on EBI_A in the EBI_ROUTE register."]
    D16A16ALE,
    #[doc = "EBI_AD drives 8 bit data, 24 bit address, ALE is used for address latching. Extended address bits can be enabled on EBI_A in the EBI_ROUTE register."]
    D8A24ALE,
    #[doc = "EBI_AD drives 16 bit data, ALE not used. Extended address bits can be enabled on EBI_A in the EBI_ROUTE register."]
    D16,
}
impl MODE3W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            MODE3W::D8A8 => 0,
            MODE3W::D16A16ALE => 1,
            MODE3W::D8A24ALE => 2,
            MODE3W::D16 => 3,
        }
    }
}
#[doc = r" Proxy"]
pub struct _MODE3W<'a> {
    w: &'a mut W,
}
impl<'a> _MODE3W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: MODE3W) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "EBI_AD drives 8 bit data, 8 bit address, ALE not used. Extended address bits can be enabled on EBI_A in the EBI_ROUTE register."]
    #[inline]
    pub fn d8a8(self) -> &'a mut W {
        self.variant(MODE3W::D8A8)
    }
    #[doc = "EBI_AD drives 16 bit data, 16 bit address, ALE is used for address latching. Extended address bits can be enabled on EBI_A in the EBI_ROUTE register."]
    #[inline]
    pub fn d16a16ale(self) -> &'a mut W {
        self.variant(MODE3W::D16A16ALE)
    }
    #[doc = "EBI_AD drives 8 bit data, 24 bit address, ALE is used for address latching. Extended address bits can be enabled on EBI_A in the EBI_ROUTE register."]
    #[inline]
    pub fn d8a24ale(self) -> &'a mut W {
        self.variant(MODE3W::D8A24ALE)
    }
    #[doc = "EBI_AD drives 16 bit data, ALE not used. Extended address bits can be enabled on EBI_A in the EBI_ROUTE register."]
    #[inline]
    pub fn d16(self) -> &'a mut W {
        self.variant(MODE3W::D16)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 0x03;
        const OFFSET: u8 = 6;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _BANK0ENW<'a> {
    w: &'a mut W,
}
impl<'a> _BANK0ENW<'a> {
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
pub struct _BANK1ENW<'a> {
    w: &'a mut W,
}
impl<'a> _BANK1ENW<'a> {
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
pub struct _BANK2ENW<'a> {
    w: &'a mut W,
}
impl<'a> _BANK2ENW<'a> {
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
pub struct _BANK3ENW<'a> {
    w: &'a mut W,
}
impl<'a> _BANK3ENW<'a> {
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
pub struct _NOIDLEW<'a> {
    w: &'a mut W,
}
impl<'a> _NOIDLEW<'a> {
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
pub struct _NOIDLE1W<'a> {
    w: &'a mut W,
}
impl<'a> _NOIDLE1W<'a> {
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
pub struct _NOIDLE2W<'a> {
    w: &'a mut W,
}
impl<'a> _NOIDLE2W<'a> {
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
pub struct _NOIDLE3W<'a> {
    w: &'a mut W,
}
impl<'a> _NOIDLE3W<'a> {
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
pub struct _ARDYENW<'a> {
    w: &'a mut W,
}
impl<'a> _ARDYENW<'a> {
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
pub struct _ARDYTODISW<'a> {
    w: &'a mut W,
}
impl<'a> _ARDYTODISW<'a> {
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
pub struct _ARDY1ENW<'a> {
    w: &'a mut W,
}
impl<'a> _ARDY1ENW<'a> {
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
pub struct _ARDYTO1DISW<'a> {
    w: &'a mut W,
}
impl<'a> _ARDYTO1DISW<'a> {
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
pub struct _ARDY2ENW<'a> {
    w: &'a mut W,
}
impl<'a> _ARDY2ENW<'a> {
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
pub struct _ARDYTO2DISW<'a> {
    w: &'a mut W,
}
impl<'a> _ARDYTO2DISW<'a> {
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
pub struct _ARDY3ENW<'a> {
    w: &'a mut W,
}
impl<'a> _ARDY3ENW<'a> {
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
pub struct _ARDYTO3DISW<'a> {
    w: &'a mut W,
}
impl<'a> _ARDYTO3DISW<'a> {
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
#[doc = r" Proxy"]
pub struct _BLW<'a> {
    w: &'a mut W,
}
impl<'a> _BLW<'a> {
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
pub struct _BL1W<'a> {
    w: &'a mut W,
}
impl<'a> _BL1W<'a> {
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
pub struct _BL2W<'a> {
    w: &'a mut W,
}
impl<'a> _BL2W<'a> {
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
#[doc = r" Proxy"]
pub struct _BL3W<'a> {
    w: &'a mut W,
}
impl<'a> _BL3W<'a> {
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
        const OFFSET: u8 = 27;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _ITSW<'a> {
    w: &'a mut W,
}
impl<'a> _ITSW<'a> {
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
        const OFFSET: u8 = 30;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _ALTMAPW<'a> {
    w: &'a mut W,
}
impl<'a> _ALTMAPW<'a> {
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
        const OFFSET: u8 = 31;
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
    #[doc = "Bits 0:1 - Mode"]
    #[inline]
    pub fn mode(&self) -> MODER {
        MODER::_from({
            const MASK: u8 = 0x03;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 2:3 - Mode 1"]
    #[inline]
    pub fn mode1(&self) -> MODE1R {
        MODE1R::_from({
            const MASK: u8 = 0x03;
            const OFFSET: u8 = 2;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 4:5 - Mode 2"]
    #[inline]
    pub fn mode2(&self) -> MODE2R {
        MODE2R::_from({
            const MASK: u8 = 0x03;
            const OFFSET: u8 = 4;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 6:7 - Mode 3"]
    #[inline]
    pub fn mode3(&self) -> MODE3R {
        MODE3R::_from({
            const MASK: u8 = 0x03;
            const OFFSET: u8 = 6;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bit 8 - Bank 0 Enable"]
    #[inline]
    pub fn bank0en(&self) -> BANK0ENR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 8;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        BANK0ENR { bits }
    }
    #[doc = "Bit 9 - Bank 1 Enable"]
    #[inline]
    pub fn bank1en(&self) -> BANK1ENR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 9;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        BANK1ENR { bits }
    }
    #[doc = "Bit 10 - Bank 2 Enable"]
    #[inline]
    pub fn bank2en(&self) -> BANK2ENR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 10;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        BANK2ENR { bits }
    }
    #[doc = "Bit 11 - Bank 3 Enable"]
    #[inline]
    pub fn bank3en(&self) -> BANK3ENR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 11;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        BANK3ENR { bits }
    }
    #[doc = "Bit 12 - No idle cycle insertion on bank 0."]
    #[inline]
    pub fn noidle(&self) -> NOIDLER {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 12;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        NOIDLER { bits }
    }
    #[doc = "Bit 13 - No idle cycle insertion on bank 1."]
    #[inline]
    pub fn noidle1(&self) -> NOIDLE1R {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 13;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        NOIDLE1R { bits }
    }
    #[doc = "Bit 14 - No idle cycle insertion on bank 2."]
    #[inline]
    pub fn noidle2(&self) -> NOIDLE2R {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 14;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        NOIDLE2R { bits }
    }
    #[doc = "Bit 15 - No idle cycle insertion on bank 3."]
    #[inline]
    pub fn noidle3(&self) -> NOIDLE3R {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 15;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        NOIDLE3R { bits }
    }
    #[doc = "Bit 16 - ARDY Enable"]
    #[inline]
    pub fn ardyen(&self) -> ARDYENR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 16;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        ARDYENR { bits }
    }
    #[doc = "Bit 17 - ARDY Timeout Disable"]
    #[inline]
    pub fn ardytodis(&self) -> ARDYTODISR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 17;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        ARDYTODISR { bits }
    }
    #[doc = "Bit 18 - ARDY Enable for bank 1"]
    #[inline]
    pub fn ardy1en(&self) -> ARDY1ENR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 18;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        ARDY1ENR { bits }
    }
    #[doc = "Bit 19 - ARDY Timeout Disable for bank 1"]
    #[inline]
    pub fn ardyto1dis(&self) -> ARDYTO1DISR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 19;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        ARDYTO1DISR { bits }
    }
    #[doc = "Bit 20 - ARDY Enable for bank 2"]
    #[inline]
    pub fn ardy2en(&self) -> ARDY2ENR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 20;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        ARDY2ENR { bits }
    }
    #[doc = "Bit 21 - ARDY Timeout Disable for bank 2"]
    #[inline]
    pub fn ardyto2dis(&self) -> ARDYTO2DISR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 21;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        ARDYTO2DISR { bits }
    }
    #[doc = "Bit 22 - ARDY Enable for bank 3"]
    #[inline]
    pub fn ardy3en(&self) -> ARDY3ENR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 22;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        ARDY3ENR { bits }
    }
    #[doc = "Bit 23 - ARDY Timeout Disable for bank 3"]
    #[inline]
    pub fn ardyto3dis(&self) -> ARDYTO3DISR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 23;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        ARDYTO3DISR { bits }
    }
    #[doc = "Bit 24 - Byte Lane Enable for bank 0"]
    #[inline]
    pub fn bl(&self) -> BLR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 24;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        BLR { bits }
    }
    #[doc = "Bit 25 - Byte Lane Enable for bank 1"]
    #[inline]
    pub fn bl1(&self) -> BL1R {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 25;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        BL1R { bits }
    }
    #[doc = "Bit 26 - Byte Lane Enable for bank 2"]
    #[inline]
    pub fn bl2(&self) -> BL2R {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 26;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        BL2R { bits }
    }
    #[doc = "Bit 27 - Byte Lane Enable for bank 3"]
    #[inline]
    pub fn bl3(&self) -> BL3R {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 27;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        BL3R { bits }
    }
    #[doc = "Bit 30 - Individual Timing Set, Line Polarity and Mode Definition Enable"]
    #[inline]
    pub fn its(&self) -> ITSR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 30;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        ITSR { bits }
    }
    #[doc = "Bit 31 - Alternative Address Map Enable"]
    #[inline]
    pub fn altmap(&self) -> ALTMAPR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 31;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        ALTMAPR { bits }
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
    #[doc = "Bits 0:1 - Mode"]
    #[inline]
    pub fn mode(&mut self) -> _MODEW {
        _MODEW { w: self }
    }
    #[doc = "Bits 2:3 - Mode 1"]
    #[inline]
    pub fn mode1(&mut self) -> _MODE1W {
        _MODE1W { w: self }
    }
    #[doc = "Bits 4:5 - Mode 2"]
    #[inline]
    pub fn mode2(&mut self) -> _MODE2W {
        _MODE2W { w: self }
    }
    #[doc = "Bits 6:7 - Mode 3"]
    #[inline]
    pub fn mode3(&mut self) -> _MODE3W {
        _MODE3W { w: self }
    }
    #[doc = "Bit 8 - Bank 0 Enable"]
    #[inline]
    pub fn bank0en(&mut self) -> _BANK0ENW {
        _BANK0ENW { w: self }
    }
    #[doc = "Bit 9 - Bank 1 Enable"]
    #[inline]
    pub fn bank1en(&mut self) -> _BANK1ENW {
        _BANK1ENW { w: self }
    }
    #[doc = "Bit 10 - Bank 2 Enable"]
    #[inline]
    pub fn bank2en(&mut self) -> _BANK2ENW {
        _BANK2ENW { w: self }
    }
    #[doc = "Bit 11 - Bank 3 Enable"]
    #[inline]
    pub fn bank3en(&mut self) -> _BANK3ENW {
        _BANK3ENW { w: self }
    }
    #[doc = "Bit 12 - No idle cycle insertion on bank 0."]
    #[inline]
    pub fn noidle(&mut self) -> _NOIDLEW {
        _NOIDLEW { w: self }
    }
    #[doc = "Bit 13 - No idle cycle insertion on bank 1."]
    #[inline]
    pub fn noidle1(&mut self) -> _NOIDLE1W {
        _NOIDLE1W { w: self }
    }
    #[doc = "Bit 14 - No idle cycle insertion on bank 2."]
    #[inline]
    pub fn noidle2(&mut self) -> _NOIDLE2W {
        _NOIDLE2W { w: self }
    }
    #[doc = "Bit 15 - No idle cycle insertion on bank 3."]
    #[inline]
    pub fn noidle3(&mut self) -> _NOIDLE3W {
        _NOIDLE3W { w: self }
    }
    #[doc = "Bit 16 - ARDY Enable"]
    #[inline]
    pub fn ardyen(&mut self) -> _ARDYENW {
        _ARDYENW { w: self }
    }
    #[doc = "Bit 17 - ARDY Timeout Disable"]
    #[inline]
    pub fn ardytodis(&mut self) -> _ARDYTODISW {
        _ARDYTODISW { w: self }
    }
    #[doc = "Bit 18 - ARDY Enable for bank 1"]
    #[inline]
    pub fn ardy1en(&mut self) -> _ARDY1ENW {
        _ARDY1ENW { w: self }
    }
    #[doc = "Bit 19 - ARDY Timeout Disable for bank 1"]
    #[inline]
    pub fn ardyto1dis(&mut self) -> _ARDYTO1DISW {
        _ARDYTO1DISW { w: self }
    }
    #[doc = "Bit 20 - ARDY Enable for bank 2"]
    #[inline]
    pub fn ardy2en(&mut self) -> _ARDY2ENW {
        _ARDY2ENW { w: self }
    }
    #[doc = "Bit 21 - ARDY Timeout Disable for bank 2"]
    #[inline]
    pub fn ardyto2dis(&mut self) -> _ARDYTO2DISW {
        _ARDYTO2DISW { w: self }
    }
    #[doc = "Bit 22 - ARDY Enable for bank 3"]
    #[inline]
    pub fn ardy3en(&mut self) -> _ARDY3ENW {
        _ARDY3ENW { w: self }
    }
    #[doc = "Bit 23 - ARDY Timeout Disable for bank 3"]
    #[inline]
    pub fn ardyto3dis(&mut self) -> _ARDYTO3DISW {
        _ARDYTO3DISW { w: self }
    }
    #[doc = "Bit 24 - Byte Lane Enable for bank 0"]
    #[inline]
    pub fn bl(&mut self) -> _BLW {
        _BLW { w: self }
    }
    #[doc = "Bit 25 - Byte Lane Enable for bank 1"]
    #[inline]
    pub fn bl1(&mut self) -> _BL1W {
        _BL1W { w: self }
    }
    #[doc = "Bit 26 - Byte Lane Enable for bank 2"]
    #[inline]
    pub fn bl2(&mut self) -> _BL2W {
        _BL2W { w: self }
    }
    #[doc = "Bit 27 - Byte Lane Enable for bank 3"]
    #[inline]
    pub fn bl3(&mut self) -> _BL3W {
        _BL3W { w: self }
    }
    #[doc = "Bit 30 - Individual Timing Set, Line Polarity and Mode Definition Enable"]
    #[inline]
    pub fn its(&mut self) -> _ITSW {
        _ITSW { w: self }
    }
    #[doc = "Bit 31 - Alternative Address Map Enable"]
    #[inline]
    pub fn altmap(&mut self) -> _ALTMAPW {
        _ALTMAPW { w: self }
    }
}

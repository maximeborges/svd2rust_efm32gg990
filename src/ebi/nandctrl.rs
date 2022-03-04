#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::NANDCTRL {
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
pub struct ENR {
    bits: bool,
}
impl ENR {
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
#[doc = "Possible values of the field `BANKSEL`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BANKSELR {
    #[doc = "Memory bank 0 is connected to a NAND Flash device."]
    BANK0,
    #[doc = "Memory bank 1 is connected to a NAND Flash device."]
    BANK1,
    #[doc = "Memory bank 2 is connected to a NAND Flash device."]
    BANK2,
    #[doc = "Memory bank 3 is connected to a NAND Flash device."]
    BANK3,
}
impl BANKSELR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            BANKSELR::BANK0 => 0,
            BANKSELR::BANK1 => 0x01,
            BANKSELR::BANK2 => 0x02,
            BANKSELR::BANK3 => 0x03,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> BANKSELR {
        match value {
            0 => BANKSELR::BANK0,
            1 => BANKSELR::BANK1,
            2 => BANKSELR::BANK2,
            3 => BANKSELR::BANK3,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `BANK0`"]
    #[inline]
    pub fn is_bank0(&self) -> bool {
        *self == BANKSELR::BANK0
    }
    #[doc = "Checks if the value of the field is `BANK1`"]
    #[inline]
    pub fn is_bank1(&self) -> bool {
        *self == BANKSELR::BANK1
    }
    #[doc = "Checks if the value of the field is `BANK2`"]
    #[inline]
    pub fn is_bank2(&self) -> bool {
        *self == BANKSELR::BANK2
    }
    #[doc = "Checks if the value of the field is `BANK3`"]
    #[inline]
    pub fn is_bank3(&self) -> bool {
        *self == BANKSELR::BANK3
    }
}
#[doc = r" Proxy"]
pub struct _ENW<'a> {
    w: &'a mut W,
}
impl<'a> _ENW<'a> {
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
#[doc = "Values that can be written to the field `BANKSEL`"]
pub enum BANKSELW {
    #[doc = "Memory bank 0 is connected to a NAND Flash device."]
    BANK0,
    #[doc = "Memory bank 1 is connected to a NAND Flash device."]
    BANK1,
    #[doc = "Memory bank 2 is connected to a NAND Flash device."]
    BANK2,
    #[doc = "Memory bank 3 is connected to a NAND Flash device."]
    BANK3,
}
impl BANKSELW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            BANKSELW::BANK0 => 0,
            BANKSELW::BANK1 => 1,
            BANKSELW::BANK2 => 2,
            BANKSELW::BANK3 => 3,
        }
    }
}
#[doc = r" Proxy"]
pub struct _BANKSELW<'a> {
    w: &'a mut W,
}
impl<'a> _BANKSELW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: BANKSELW) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "Memory bank 0 is connected to a NAND Flash device."]
    #[inline]
    pub fn bank0(self) -> &'a mut W {
        self.variant(BANKSELW::BANK0)
    }
    #[doc = "Memory bank 1 is connected to a NAND Flash device."]
    #[inline]
    pub fn bank1(self) -> &'a mut W {
        self.variant(BANKSELW::BANK1)
    }
    #[doc = "Memory bank 2 is connected to a NAND Flash device."]
    #[inline]
    pub fn bank2(self) -> &'a mut W {
        self.variant(BANKSELW::BANK2)
    }
    #[doc = "Memory bank 3 is connected to a NAND Flash device."]
    #[inline]
    pub fn bank3(self) -> &'a mut W {
        self.variant(BANKSELW::BANK3)
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
impl R {
    #[doc = r" Value of the register as raw bits"]
    #[inline]
    pub fn bits(&self) -> u32 {
        self.bits
    }
    #[doc = "Bit 0 - NAND Flash control enable"]
    #[inline]
    pub fn en(&self) -> ENR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        ENR { bits }
    }
    #[doc = "Bits 4:5 - NAND Flash Bank"]
    #[inline]
    pub fn banksel(&self) -> BANKSELR {
        BANKSELR::_from({
            const MASK: u8 = 0x03;
            const OFFSET: u8 = 4;
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
    #[doc = "Bit 0 - NAND Flash control enable"]
    #[inline]
    pub fn en(&mut self) -> _ENW {
        _ENW { w: self }
    }
    #[doc = "Bits 4:5 - NAND Flash Bank"]
    #[inline]
    pub fn banksel(&mut self) -> _BANKSELW {
        _BANKSELW { w: self }
    }
}

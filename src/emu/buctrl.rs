#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::BUCTRL {
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
#[doc = r" Value of the field"]
pub struct STATENR {
    bits: bool,
}
impl STATENR {
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
pub struct BODCALR {
    bits: bool,
}
impl BODCALR {
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
pub struct BUMODEBODENR {
    bits: bool,
}
impl BUMODEBODENR {
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
#[doc = "Possible values of the field `PROBE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PROBER {
    #[doc = "Disable voltage probe."]
    DISABLE,
    #[doc = "Connect probe to VDD_DREG."]
    VDDDREG,
    #[doc = "Connect probe to BU_IN."]
    BUIN,
    #[doc = "Connect probe to BU_OUT."]
    BUOUT,
}
impl PROBER {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            PROBER::DISABLE => 0,
            PROBER::VDDDREG => 0x01,
            PROBER::BUIN => 0x02,
            PROBER::BUOUT => 0x03,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> PROBER {
        match value {
            0 => PROBER::DISABLE,
            1 => PROBER::VDDDREG,
            2 => PROBER::BUIN,
            3 => PROBER::BUOUT,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline]
    pub fn is_disable(&self) -> bool {
        *self == PROBER::DISABLE
    }
    #[doc = "Checks if the value of the field is `VDDDREG`"]
    #[inline]
    pub fn is_vdddreg(&self) -> bool {
        *self == PROBER::VDDDREG
    }
    #[doc = "Checks if the value of the field is `BUIN`"]
    #[inline]
    pub fn is_buin(&self) -> bool {
        *self == PROBER::BUIN
    }
    #[doc = "Checks if the value of the field is `BUOUT`"]
    #[inline]
    pub fn is_buout(&self) -> bool {
        *self == PROBER::BUOUT
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
#[doc = r" Proxy"]
pub struct _STATENW<'a> {
    w: &'a mut W,
}
impl<'a> _STATENW<'a> {
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
pub struct _BODCALW<'a> {
    w: &'a mut W,
}
impl<'a> _BODCALW<'a> {
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
pub struct _BUMODEBODENW<'a> {
    w: &'a mut W,
}
impl<'a> _BUMODEBODENW<'a> {
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
#[doc = "Values that can be written to the field `PROBE`"]
pub enum PROBEW {
    #[doc = "Disable voltage probe."]
    DISABLE,
    #[doc = "Connect probe to VDD_DREG."]
    VDDDREG,
    #[doc = "Connect probe to BU_IN."]
    BUIN,
    #[doc = "Connect probe to BU_OUT."]
    BUOUT,
}
impl PROBEW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            PROBEW::DISABLE => 0,
            PROBEW::VDDDREG => 1,
            PROBEW::BUIN => 2,
            PROBEW::BUOUT => 3,
        }
    }
}
#[doc = r" Proxy"]
pub struct _PROBEW<'a> {
    w: &'a mut W,
}
impl<'a> _PROBEW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: PROBEW) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "Disable voltage probe."]
    #[inline]
    pub fn disable(self) -> &'a mut W {
        self.variant(PROBEW::DISABLE)
    }
    #[doc = "Connect probe to VDD_DREG."]
    #[inline]
    pub fn vdddreg(self) -> &'a mut W {
        self.variant(PROBEW::VDDDREG)
    }
    #[doc = "Connect probe to BU_IN."]
    #[inline]
    pub fn buin(self) -> &'a mut W {
        self.variant(PROBEW::BUIN)
    }
    #[doc = "Connect probe to BU_OUT."]
    #[inline]
    pub fn buout(self) -> &'a mut W {
        self.variant(PROBEW::BUOUT)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 0x03;
        const OFFSET: u8 = 5;
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
    #[doc = "Bit 0 - Enable backup mode"]
    #[inline]
    pub fn en(&self) -> ENR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        ENR { bits }
    }
    #[doc = "Bit 1 - Enable backup mode status export"]
    #[inline]
    pub fn staten(&self) -> STATENR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 1;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        STATENR { bits }
    }
    #[doc = "Bit 2 - Enable BOD calibration mode"]
    #[inline]
    pub fn bodcal(&self) -> BODCALR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 2;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        BODCALR { bits }
    }
    #[doc = "Bit 3 - Enable brown out detection on BU_VIN when in backup mode"]
    #[inline]
    pub fn bumodeboden(&self) -> BUMODEBODENR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 3;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        BUMODEBODENR { bits }
    }
    #[doc = "Bits 5:6 - Voltage probe select"]
    #[inline]
    pub fn probe(&self) -> PROBER {
        PROBER::_from({
            const MASK: u8 = 0x03;
            const OFFSET: u8 = 5;
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
    #[doc = "Bit 0 - Enable backup mode"]
    #[inline]
    pub fn en(&mut self) -> _ENW {
        _ENW { w: self }
    }
    #[doc = "Bit 1 - Enable backup mode status export"]
    #[inline]
    pub fn staten(&mut self) -> _STATENW {
        _STATENW { w: self }
    }
    #[doc = "Bit 2 - Enable BOD calibration mode"]
    #[inline]
    pub fn bodcal(&mut self) -> _BODCALW {
        _BODCALW { w: self }
    }
    #[doc = "Bit 3 - Enable brown out detection on BU_VIN when in backup mode"]
    #[inline]
    pub fn bumodeboden(&mut self) -> _BUMODEBODENW {
        _BUMODEBODENW { w: self }
    }
    #[doc = "Bits 5:6 - Voltage probe select"]
    #[inline]
    pub fn probe(&mut self) -> _PROBEW {
        _PROBEW { w: self }
    }
}

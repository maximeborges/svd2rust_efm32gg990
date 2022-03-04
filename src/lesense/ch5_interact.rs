#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::CH5_INTERACT {
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
pub struct ACMPTHRESR {
    bits: u16,
}
impl ACMPTHRESR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u16 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct SAMPLER {
    bits: bool,
}
impl SAMPLER {
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
#[doc = "Possible values of the field `SETIF`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SETIFR {
    #[doc = "No interrupt is generated"]
    NONE,
    #[doc = "Set interrupt flag if the sensor triggers."]
    LEVEL,
    #[doc = "Set interrupt flag on positive edge on the sensor state"]
    POSEDGE,
    #[doc = "Set interrupt flag on negative edge on the sensor state"]
    NEGEDGE,
}
impl SETIFR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            SETIFR::NONE => 0,
            SETIFR::LEVEL => 0x01,
            SETIFR::POSEDGE => 0x02,
            SETIFR::NEGEDGE => 0x03,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> SETIFR {
        match value {
            0 => SETIFR::NONE,
            1 => SETIFR::LEVEL,
            2 => SETIFR::POSEDGE,
            3 => SETIFR::NEGEDGE,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `NONE`"]
    #[inline]
    pub fn is_none(&self) -> bool {
        *self == SETIFR::NONE
    }
    #[doc = "Checks if the value of the field is `LEVEL`"]
    #[inline]
    pub fn is_level(&self) -> bool {
        *self == SETIFR::LEVEL
    }
    #[doc = "Checks if the value of the field is `POSEDGE`"]
    #[inline]
    pub fn is_posedge(&self) -> bool {
        *self == SETIFR::POSEDGE
    }
    #[doc = "Checks if the value of the field is `NEGEDGE`"]
    #[inline]
    pub fn is_negedge(&self) -> bool {
        *self == SETIFR::NEGEDGE
    }
}
#[doc = "Possible values of the field `EXMODE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum EXMODER {
    #[doc = "Disabled"]
    DISABLE,
    #[doc = "Push Pull, GPIO is driven high"]
    HIGH,
    #[doc = "Push Pull, GPIO is driven low"]
    LOW,
    #[doc = "DAC output"]
    DACOUT,
}
impl EXMODER {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            EXMODER::DISABLE => 0,
            EXMODER::HIGH => 0x01,
            EXMODER::LOW => 0x02,
            EXMODER::DACOUT => 0x03,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> EXMODER {
        match value {
            0 => EXMODER::DISABLE,
            1 => EXMODER::HIGH,
            2 => EXMODER::LOW,
            3 => EXMODER::DACOUT,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline]
    pub fn is_disable(&self) -> bool {
        *self == EXMODER::DISABLE
    }
    #[doc = "Checks if the value of the field is `HIGH`"]
    #[inline]
    pub fn is_high(&self) -> bool {
        *self == EXMODER::HIGH
    }
    #[doc = "Checks if the value of the field is `LOW`"]
    #[inline]
    pub fn is_low(&self) -> bool {
        *self == EXMODER::LOW
    }
    #[doc = "Checks if the value of the field is `DACOUT`"]
    #[inline]
    pub fn is_dacout(&self) -> bool {
        *self == EXMODER::DACOUT
    }
}
#[doc = r" Value of the field"]
pub struct EXCLKR {
    bits: bool,
}
impl EXCLKR {
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
pub struct SAMPLECLKR {
    bits: bool,
}
impl SAMPLECLKR {
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
pub struct ALTEXR {
    bits: bool,
}
impl ALTEXR {
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
#[doc = r" Proxy"]
pub struct _ACMPTHRESW<'a> {
    w: &'a mut W,
}
impl<'a> _ACMPTHRESW<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        const MASK: u16 = 0x0fff;
        const OFFSET: u8 = 0;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _SAMPLEW<'a> {
    w: &'a mut W,
}
impl<'a> _SAMPLEW<'a> {
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
#[doc = "Values that can be written to the field `SETIF`"]
pub enum SETIFW {
    #[doc = "No interrupt is generated"]
    NONE,
    #[doc = "Set interrupt flag if the sensor triggers."]
    LEVEL,
    #[doc = "Set interrupt flag on positive edge on the sensor state"]
    POSEDGE,
    #[doc = "Set interrupt flag on negative edge on the sensor state"]
    NEGEDGE,
}
impl SETIFW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            SETIFW::NONE => 0,
            SETIFW::LEVEL => 1,
            SETIFW::POSEDGE => 2,
            SETIFW::NEGEDGE => 3,
        }
    }
}
#[doc = r" Proxy"]
pub struct _SETIFW<'a> {
    w: &'a mut W,
}
impl<'a> _SETIFW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: SETIFW) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "No interrupt is generated"]
    #[inline]
    pub fn none(self) -> &'a mut W {
        self.variant(SETIFW::NONE)
    }
    #[doc = "Set interrupt flag if the sensor triggers."]
    #[inline]
    pub fn level(self) -> &'a mut W {
        self.variant(SETIFW::LEVEL)
    }
    #[doc = "Set interrupt flag on positive edge on the sensor state"]
    #[inline]
    pub fn posedge(self) -> &'a mut W {
        self.variant(SETIFW::POSEDGE)
    }
    #[doc = "Set interrupt flag on negative edge on the sensor state"]
    #[inline]
    pub fn negedge(self) -> &'a mut W {
        self.variant(SETIFW::NEGEDGE)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 0x03;
        const OFFSET: u8 = 13;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `EXMODE`"]
pub enum EXMODEW {
    #[doc = "Disabled"]
    DISABLE,
    #[doc = "Push Pull, GPIO is driven high"]
    HIGH,
    #[doc = "Push Pull, GPIO is driven low"]
    LOW,
    #[doc = "DAC output"]
    DACOUT,
}
impl EXMODEW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            EXMODEW::DISABLE => 0,
            EXMODEW::HIGH => 1,
            EXMODEW::LOW => 2,
            EXMODEW::DACOUT => 3,
        }
    }
}
#[doc = r" Proxy"]
pub struct _EXMODEW<'a> {
    w: &'a mut W,
}
impl<'a> _EXMODEW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: EXMODEW) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "Disabled"]
    #[inline]
    pub fn disable(self) -> &'a mut W {
        self.variant(EXMODEW::DISABLE)
    }
    #[doc = "Push Pull, GPIO is driven high"]
    #[inline]
    pub fn high(self) -> &'a mut W {
        self.variant(EXMODEW::HIGH)
    }
    #[doc = "Push Pull, GPIO is driven low"]
    #[inline]
    pub fn low(self) -> &'a mut W {
        self.variant(EXMODEW::LOW)
    }
    #[doc = "DAC output"]
    #[inline]
    pub fn dacout(self) -> &'a mut W {
        self.variant(EXMODEW::DACOUT)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 0x03;
        const OFFSET: u8 = 15;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _EXCLKW<'a> {
    w: &'a mut W,
}
impl<'a> _EXCLKW<'a> {
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
pub struct _SAMPLECLKW<'a> {
    w: &'a mut W,
}
impl<'a> _SAMPLECLKW<'a> {
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
pub struct _ALTEXW<'a> {
    w: &'a mut W,
}
impl<'a> _ALTEXW<'a> {
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
impl R {
    #[doc = r" Value of the register as raw bits"]
    #[inline]
    pub fn bits(&self) -> u32 {
        self.bits
    }
    #[doc = "Bits 0:11 - Set ACMP threshold"]
    #[inline]
    pub fn acmpthres(&self) -> ACMPTHRESR {
        let bits = {
            const MASK: u16 = 0x0fff;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) as u16
        };
        ACMPTHRESR { bits }
    }
    #[doc = "Bit 12 - Select sample mode"]
    #[inline]
    pub fn sample(&self) -> SAMPLER {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 12;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        SAMPLER { bits }
    }
    #[doc = "Bits 13:14 - Enable interrupt generation"]
    #[inline]
    pub fn setif(&self) -> SETIFR {
        SETIFR::_from({
            const MASK: u8 = 0x03;
            const OFFSET: u8 = 13;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 15:16 - Set GPIO mode"]
    #[inline]
    pub fn exmode(&self) -> EXMODER {
        EXMODER::_from({
            const MASK: u8 = 0x03;
            const OFFSET: u8 = 15;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bit 17 - Select clock used for excitation timing"]
    #[inline]
    pub fn exclk(&self) -> EXCLKR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 17;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        EXCLKR { bits }
    }
    #[doc = "Bit 18 - Select clock used for timing of sample delay"]
    #[inline]
    pub fn sampleclk(&self) -> SAMPLECLKR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 18;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        SAMPLECLKR { bits }
    }
    #[doc = "Bit 19 - Use alternative excite pin"]
    #[inline]
    pub fn altex(&self) -> ALTEXR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 19;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        ALTEXR { bits }
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
    #[doc = "Bits 0:11 - Set ACMP threshold"]
    #[inline]
    pub fn acmpthres(&mut self) -> _ACMPTHRESW {
        _ACMPTHRESW { w: self }
    }
    #[doc = "Bit 12 - Select sample mode"]
    #[inline]
    pub fn sample(&mut self) -> _SAMPLEW {
        _SAMPLEW { w: self }
    }
    #[doc = "Bits 13:14 - Enable interrupt generation"]
    #[inline]
    pub fn setif(&mut self) -> _SETIFW {
        _SETIFW { w: self }
    }
    #[doc = "Bits 15:16 - Set GPIO mode"]
    #[inline]
    pub fn exmode(&mut self) -> _EXMODEW {
        _EXMODEW { w: self }
    }
    #[doc = "Bit 17 - Select clock used for excitation timing"]
    #[inline]
    pub fn exclk(&mut self) -> _EXCLKW {
        _EXCLKW { w: self }
    }
    #[doc = "Bit 18 - Select clock used for timing of sample delay"]
    #[inline]
    pub fn sampleclk(&mut self) -> _SAMPLECLKW {
        _SAMPLECLKW { w: self }
    }
    #[doc = "Bit 19 - Use alternative excite pin"]
    #[inline]
    pub fn altex(&mut self) -> _ALTEXW {
        _ALTEXW { w: self }
    }
}

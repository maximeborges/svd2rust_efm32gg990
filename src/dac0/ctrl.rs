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
#[doc = r" Value of the field"]
pub struct DIFFR {
    bits: bool,
}
impl DIFFR {
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
pub struct SINEMODER {
    bits: bool,
}
impl SINEMODER {
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
#[doc = "Possible values of the field `CONVMODE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CONVMODER {
    #[doc = "DAC is set in continuous mode"]
    CONTINUOUS,
    #[doc = "DAC is set in sample/hold mode"]
    SAMPLEHOLD,
    #[doc = "DAC is set in sample/shut off mode"]
    SAMPLEOFF,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl CONVMODER {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            CONVMODER::CONTINUOUS => 0,
            CONVMODER::SAMPLEHOLD => 0x01,
            CONVMODER::SAMPLEOFF => 0x02,
            CONVMODER::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> CONVMODER {
        match value {
            0 => CONVMODER::CONTINUOUS,
            1 => CONVMODER::SAMPLEHOLD,
            2 => CONVMODER::SAMPLEOFF,
            i => CONVMODER::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `CONTINUOUS`"]
    #[inline]
    pub fn is_continuous(&self) -> bool {
        *self == CONVMODER::CONTINUOUS
    }
    #[doc = "Checks if the value of the field is `SAMPLEHOLD`"]
    #[inline]
    pub fn is_samplehold(&self) -> bool {
        *self == CONVMODER::SAMPLEHOLD
    }
    #[doc = "Checks if the value of the field is `SAMPLEOFF`"]
    #[inline]
    pub fn is_sampleoff(&self) -> bool {
        *self == CONVMODER::SAMPLEOFF
    }
}
#[doc = "Possible values of the field `OUTMODE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum OUTMODER {
    #[doc = "DAC output to pin and ADC disabled"]
    DISABLE,
    #[doc = "DAC output to pin enabled. DAC output to ADC and ACMP disabled"]
    PIN,
    #[doc = "DAC output to pin disabled. DAC output to ADC and ACMP enabled"]
    ADC,
    #[doc = "DAC output to pin, ADC, and ACMP enabled"]
    PINADC,
}
impl OUTMODER {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            OUTMODER::DISABLE => 0,
            OUTMODER::PIN => 0x01,
            OUTMODER::ADC => 0x02,
            OUTMODER::PINADC => 0x03,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> OUTMODER {
        match value {
            0 => OUTMODER::DISABLE,
            1 => OUTMODER::PIN,
            2 => OUTMODER::ADC,
            3 => OUTMODER::PINADC,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline]
    pub fn is_disable(&self) -> bool {
        *self == OUTMODER::DISABLE
    }
    #[doc = "Checks if the value of the field is `PIN`"]
    #[inline]
    pub fn is_pin(&self) -> bool {
        *self == OUTMODER::PIN
    }
    #[doc = "Checks if the value of the field is `ADC`"]
    #[inline]
    pub fn is_adc(&self) -> bool {
        *self == OUTMODER::ADC
    }
    #[doc = "Checks if the value of the field is `PINADC`"]
    #[inline]
    pub fn is_pinadc(&self) -> bool {
        *self == OUTMODER::PINADC
    }
}
#[doc = r" Value of the field"]
pub struct OUTENPRSR {
    bits: bool,
}
impl OUTENPRSR {
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
pub struct CH0PRESCRSTR {
    bits: bool,
}
impl CH0PRESCRSTR {
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
#[doc = "Possible values of the field `REFSEL`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum REFSELR {
    #[doc = "Internal 1.25 V bandgap reference"]
    _1V25,
    #[doc = "Internal 2.5 V bandgap reference"]
    _2V5,
    #[doc = "VDD reference"]
    VDD,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl REFSELR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            REFSELR::_1V25 => 0,
            REFSELR::_2V5 => 0x01,
            REFSELR::VDD => 0x02,
            REFSELR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> REFSELR {
        match value {
            0 => REFSELR::_1V25,
            1 => REFSELR::_2V5,
            2 => REFSELR::VDD,
            i => REFSELR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `_1V25`"]
    #[inline]
    pub fn is_1v25(&self) -> bool {
        *self == REFSELR::_1V25
    }
    #[doc = "Checks if the value of the field is `_2V5`"]
    #[inline]
    pub fn is_2v5(&self) -> bool {
        *self == REFSELR::_2V5
    }
    #[doc = "Checks if the value of the field is `VDD`"]
    #[inline]
    pub fn is_vdd(&self) -> bool {
        *self == REFSELR::VDD
    }
}
#[doc = "Possible values of the field `PRESC`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PRESCR {
    #[doc = "\"\""]
    NODIVISION,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl PRESCR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            PRESCR::NODIVISION => 0,
            PRESCR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> PRESCR {
        match value {
            0 => PRESCR::NODIVISION,
            i => PRESCR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `NODIVISION`"]
    #[inline]
    pub fn is_nodivision(&self) -> bool {
        *self == PRESCR::NODIVISION
    }
}
#[doc = "Possible values of the field `REFRSEL`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum REFRSELR {
    #[doc = "All channels with enabled refresh are refreshed every 8 prescaled cycles"]
    _8CYCLES,
    #[doc = "All channels with enabled refresh are refreshed every 16 prescaled cycles"]
    _16CYCLES,
    #[doc = "All channels with enabled refresh are refreshed every 32 prescaled cycles"]
    _32CYCLES,
    #[doc = "All channels with enabled refresh are refreshed every 64 prescaled cycles"]
    _64CYCLES,
}
impl REFRSELR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            REFRSELR::_8CYCLES => 0,
            REFRSELR::_16CYCLES => 0x01,
            REFRSELR::_32CYCLES => 0x02,
            REFRSELR::_64CYCLES => 0x03,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> REFRSELR {
        match value {
            0 => REFRSELR::_8CYCLES,
            1 => REFRSELR::_16CYCLES,
            2 => REFRSELR::_32CYCLES,
            3 => REFRSELR::_64CYCLES,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `_8CYCLES`"]
    #[inline]
    pub fn is_8cycles(&self) -> bool {
        *self == REFRSELR::_8CYCLES
    }
    #[doc = "Checks if the value of the field is `_16CYCLES`"]
    #[inline]
    pub fn is_16cycles(&self) -> bool {
        *self == REFRSELR::_16CYCLES
    }
    #[doc = "Checks if the value of the field is `_32CYCLES`"]
    #[inline]
    pub fn is_32cycles(&self) -> bool {
        *self == REFRSELR::_32CYCLES
    }
    #[doc = "Checks if the value of the field is `_64CYCLES`"]
    #[inline]
    pub fn is_64cycles(&self) -> bool {
        *self == REFRSELR::_64CYCLES
    }
}
#[doc = r" Proxy"]
pub struct _DIFFW<'a> {
    w: &'a mut W,
}
impl<'a> _DIFFW<'a> {
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
pub struct _SINEMODEW<'a> {
    w: &'a mut W,
}
impl<'a> _SINEMODEW<'a> {
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
#[doc = "Values that can be written to the field `CONVMODE`"]
pub enum CONVMODEW {
    #[doc = "DAC is set in continuous mode"]
    CONTINUOUS,
    #[doc = "DAC is set in sample/hold mode"]
    SAMPLEHOLD,
    #[doc = "DAC is set in sample/shut off mode"]
    SAMPLEOFF,
}
impl CONVMODEW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            CONVMODEW::CONTINUOUS => 0,
            CONVMODEW::SAMPLEHOLD => 1,
            CONVMODEW::SAMPLEOFF => 2,
        }
    }
}
#[doc = r" Proxy"]
pub struct _CONVMODEW<'a> {
    w: &'a mut W,
}
impl<'a> _CONVMODEW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: CONVMODEW) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "DAC is set in continuous mode"]
    #[inline]
    pub fn continuous(self) -> &'a mut W {
        self.variant(CONVMODEW::CONTINUOUS)
    }
    #[doc = "DAC is set in sample/hold mode"]
    #[inline]
    pub fn samplehold(self) -> &'a mut W {
        self.variant(CONVMODEW::SAMPLEHOLD)
    }
    #[doc = "DAC is set in sample/shut off mode"]
    #[inline]
    pub fn sampleoff(self) -> &'a mut W {
        self.variant(CONVMODEW::SAMPLEOFF)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 0x03;
        const OFFSET: u8 = 2;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `OUTMODE`"]
pub enum OUTMODEW {
    #[doc = "DAC output to pin and ADC disabled"]
    DISABLE,
    #[doc = "DAC output to pin enabled. DAC output to ADC and ACMP disabled"]
    PIN,
    #[doc = "DAC output to pin disabled. DAC output to ADC and ACMP enabled"]
    ADC,
    #[doc = "DAC output to pin, ADC, and ACMP enabled"]
    PINADC,
}
impl OUTMODEW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            OUTMODEW::DISABLE => 0,
            OUTMODEW::PIN => 1,
            OUTMODEW::ADC => 2,
            OUTMODEW::PINADC => 3,
        }
    }
}
#[doc = r" Proxy"]
pub struct _OUTMODEW<'a> {
    w: &'a mut W,
}
impl<'a> _OUTMODEW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: OUTMODEW) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "DAC output to pin and ADC disabled"]
    #[inline]
    pub fn disable(self) -> &'a mut W {
        self.variant(OUTMODEW::DISABLE)
    }
    #[doc = "DAC output to pin enabled. DAC output to ADC and ACMP disabled"]
    #[inline]
    pub fn pin(self) -> &'a mut W {
        self.variant(OUTMODEW::PIN)
    }
    #[doc = "DAC output to pin disabled. DAC output to ADC and ACMP enabled"]
    #[inline]
    pub fn adc(self) -> &'a mut W {
        self.variant(OUTMODEW::ADC)
    }
    #[doc = "DAC output to pin, ADC, and ACMP enabled"]
    #[inline]
    pub fn pinadc(self) -> &'a mut W {
        self.variant(OUTMODEW::PINADC)
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
#[doc = r" Proxy"]
pub struct _OUTENPRSW<'a> {
    w: &'a mut W,
}
impl<'a> _OUTENPRSW<'a> {
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
pub struct _CH0PRESCRSTW<'a> {
    w: &'a mut W,
}
impl<'a> _CH0PRESCRSTW<'a> {
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
#[doc = "Values that can be written to the field `REFSEL`"]
pub enum REFSELW {
    #[doc = "Internal 1.25 V bandgap reference"]
    _1V25,
    #[doc = "Internal 2.5 V bandgap reference"]
    _2V5,
    #[doc = "VDD reference"]
    VDD,
}
impl REFSELW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            REFSELW::_1V25 => 0,
            REFSELW::_2V5 => 1,
            REFSELW::VDD => 2,
        }
    }
}
#[doc = r" Proxy"]
pub struct _REFSELW<'a> {
    w: &'a mut W,
}
impl<'a> _REFSELW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: REFSELW) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "Internal 1.25 V bandgap reference"]
    #[inline]
    pub fn _1v25(self) -> &'a mut W {
        self.variant(REFSELW::_1V25)
    }
    #[doc = "Internal 2.5 V bandgap reference"]
    #[inline]
    pub fn _2v5(self) -> &'a mut W {
        self.variant(REFSELW::_2V5)
    }
    #[doc = "VDD reference"]
    #[inline]
    pub fn vdd(self) -> &'a mut W {
        self.variant(REFSELW::VDD)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 0x03;
        const OFFSET: u8 = 8;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `PRESC`"]
pub enum PRESCW {
    #[doc = "\"\""]
    NODIVISION,
}
impl PRESCW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            PRESCW::NODIVISION => 0,
        }
    }
}
#[doc = r" Proxy"]
pub struct _PRESCW<'a> {
    w: &'a mut W,
}
impl<'a> _PRESCW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: PRESCW) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "\"\""]
    #[inline]
    pub fn nodivision(self) -> &'a mut W {
        self.variant(PRESCW::NODIVISION)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 0x07;
        const OFFSET: u8 = 16;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `REFRSEL`"]
pub enum REFRSELW {
    #[doc = "All channels with enabled refresh are refreshed every 8 prescaled cycles"]
    _8CYCLES,
    #[doc = "All channels with enabled refresh are refreshed every 16 prescaled cycles"]
    _16CYCLES,
    #[doc = "All channels with enabled refresh are refreshed every 32 prescaled cycles"]
    _32CYCLES,
    #[doc = "All channels with enabled refresh are refreshed every 64 prescaled cycles"]
    _64CYCLES,
}
impl REFRSELW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            REFRSELW::_8CYCLES => 0,
            REFRSELW::_16CYCLES => 1,
            REFRSELW::_32CYCLES => 2,
            REFRSELW::_64CYCLES => 3,
        }
    }
}
#[doc = r" Proxy"]
pub struct _REFRSELW<'a> {
    w: &'a mut W,
}
impl<'a> _REFRSELW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: REFRSELW) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "All channels with enabled refresh are refreshed every 8 prescaled cycles"]
    #[inline]
    pub fn _8cycles(self) -> &'a mut W {
        self.variant(REFRSELW::_8CYCLES)
    }
    #[doc = "All channels with enabled refresh are refreshed every 16 prescaled cycles"]
    #[inline]
    pub fn _16cycles(self) -> &'a mut W {
        self.variant(REFRSELW::_16CYCLES)
    }
    #[doc = "All channels with enabled refresh are refreshed every 32 prescaled cycles"]
    #[inline]
    pub fn _32cycles(self) -> &'a mut W {
        self.variant(REFRSELW::_32CYCLES)
    }
    #[doc = "All channels with enabled refresh are refreshed every 64 prescaled cycles"]
    #[inline]
    pub fn _64cycles(self) -> &'a mut W {
        self.variant(REFRSELW::_64CYCLES)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 0x03;
        const OFFSET: u8 = 20;
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
    #[doc = "Bit 0 - Differential Mode"]
    #[inline]
    pub fn diff(&self) -> DIFFR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        DIFFR { bits }
    }
    #[doc = "Bit 1 - Sine Mode"]
    #[inline]
    pub fn sinemode(&self) -> SINEMODER {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 1;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        SINEMODER { bits }
    }
    #[doc = "Bits 2:3 - Conversion Mode"]
    #[inline]
    pub fn convmode(&self) -> CONVMODER {
        CONVMODER::_from({
            const MASK: u8 = 0x03;
            const OFFSET: u8 = 2;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 4:5 - Output Mode"]
    #[inline]
    pub fn outmode(&self) -> OUTMODER {
        OUTMODER::_from({
            const MASK: u8 = 0x03;
            const OFFSET: u8 = 4;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bit 6 - PRS Controlled Output Enable"]
    #[inline]
    pub fn outenprs(&self) -> OUTENPRSR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 6;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        OUTENPRSR { bits }
    }
    #[doc = "Bit 7 - Channel 0 Start Reset Prescaler"]
    #[inline]
    pub fn ch0prescrst(&self) -> CH0PRESCRSTR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 7;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        CH0PRESCRSTR { bits }
    }
    #[doc = "Bits 8:9 - Reference Selection"]
    #[inline]
    pub fn refsel(&self) -> REFSELR {
        REFSELR::_from({
            const MASK: u8 = 0x03;
            const OFFSET: u8 = 8;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 16:18 - Prescaler Setting"]
    #[inline]
    pub fn presc(&self) -> PRESCR {
        PRESCR::_from({
            const MASK: u8 = 0x07;
            const OFFSET: u8 = 16;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 20:21 - Refresh Interval Select"]
    #[inline]
    pub fn refrsel(&self) -> REFRSELR {
        REFRSELR::_from({
            const MASK: u8 = 0x03;
            const OFFSET: u8 = 20;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
}
impl W {
    #[doc = r" Reset value of the register"]
    #[inline]
    pub fn reset_value() -> W {
        W { bits: 0x10 }
    }
    #[doc = r" Writes raw bits to the register"]
    #[inline]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
    #[doc = "Bit 0 - Differential Mode"]
    #[inline]
    pub fn diff(&mut self) -> _DIFFW {
        _DIFFW { w: self }
    }
    #[doc = "Bit 1 - Sine Mode"]
    #[inline]
    pub fn sinemode(&mut self) -> _SINEMODEW {
        _SINEMODEW { w: self }
    }
    #[doc = "Bits 2:3 - Conversion Mode"]
    #[inline]
    pub fn convmode(&mut self) -> _CONVMODEW {
        _CONVMODEW { w: self }
    }
    #[doc = "Bits 4:5 - Output Mode"]
    #[inline]
    pub fn outmode(&mut self) -> _OUTMODEW {
        _OUTMODEW { w: self }
    }
    #[doc = "Bit 6 - PRS Controlled Output Enable"]
    #[inline]
    pub fn outenprs(&mut self) -> _OUTENPRSW {
        _OUTENPRSW { w: self }
    }
    #[doc = "Bit 7 - Channel 0 Start Reset Prescaler"]
    #[inline]
    pub fn ch0prescrst(&mut self) -> _CH0PRESCRSTW {
        _CH0PRESCRSTW { w: self }
    }
    #[doc = "Bits 8:9 - Reference Selection"]
    #[inline]
    pub fn refsel(&mut self) -> _REFSELW {
        _REFSELW { w: self }
    }
    #[doc = "Bits 16:18 - Prescaler Setting"]
    #[inline]
    pub fn presc(&mut self) -> _PRESCW {
        _PRESCW { w: self }
    }
    #[doc = "Bits 20:21 - Refresh Interval Select"]
    #[inline]
    pub fn refrsel(&mut self) -> _REFRSELW {
        _REFRSELW { w: self }
    }
}

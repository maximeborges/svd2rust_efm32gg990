#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::PERCTRL {
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
pub struct DACCH0DATAR {
    bits: bool,
}
impl DACCH0DATAR {
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
pub struct DACCH1DATAR {
    bits: bool,
}
impl DACCH1DATAR {
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
#[doc = "Possible values of the field `DACCH0CONV`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DACCH0CONVR {
    #[doc = "LESENSE does not control DAC CH0."]
    DISABLE,
    #[doc = "DAC channel 0 is driven in continuous mode."]
    CONTINUOUS,
    #[doc = "DAC channel 0 is driven in sample hold mode."]
    SAMPLEHOLD,
    #[doc = "DAC channel 0 is driven in sample off mode."]
    SAMPLEOFF,
}
impl DACCH0CONVR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            DACCH0CONVR::DISABLE => 0,
            DACCH0CONVR::CONTINUOUS => 0x01,
            DACCH0CONVR::SAMPLEHOLD => 0x02,
            DACCH0CONVR::SAMPLEOFF => 0x03,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> DACCH0CONVR {
        match value {
            0 => DACCH0CONVR::DISABLE,
            1 => DACCH0CONVR::CONTINUOUS,
            2 => DACCH0CONVR::SAMPLEHOLD,
            3 => DACCH0CONVR::SAMPLEOFF,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline]
    pub fn is_disable(&self) -> bool {
        *self == DACCH0CONVR::DISABLE
    }
    #[doc = "Checks if the value of the field is `CONTINUOUS`"]
    #[inline]
    pub fn is_continuous(&self) -> bool {
        *self == DACCH0CONVR::CONTINUOUS
    }
    #[doc = "Checks if the value of the field is `SAMPLEHOLD`"]
    #[inline]
    pub fn is_samplehold(&self) -> bool {
        *self == DACCH0CONVR::SAMPLEHOLD
    }
    #[doc = "Checks if the value of the field is `SAMPLEOFF`"]
    #[inline]
    pub fn is_sampleoff(&self) -> bool {
        *self == DACCH0CONVR::SAMPLEOFF
    }
}
#[doc = "Possible values of the field `DACCH1CONV`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DACCH1CONVR {
    #[doc = "LESENSE does not control DAC CH1."]
    DISABLE,
    #[doc = "DAC channel 1 is driven in continuous mode."]
    CONTINUOUS,
    #[doc = "DAC channel 1 is driven in sample hold mode."]
    SAMPLEHOLD,
    #[doc = "DAC channel 1 is driven in sample off mode."]
    SAMPLEOFF,
}
impl DACCH1CONVR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            DACCH1CONVR::DISABLE => 0,
            DACCH1CONVR::CONTINUOUS => 0x01,
            DACCH1CONVR::SAMPLEHOLD => 0x02,
            DACCH1CONVR::SAMPLEOFF => 0x03,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> DACCH1CONVR {
        match value {
            0 => DACCH1CONVR::DISABLE,
            1 => DACCH1CONVR::CONTINUOUS,
            2 => DACCH1CONVR::SAMPLEHOLD,
            3 => DACCH1CONVR::SAMPLEOFF,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline]
    pub fn is_disable(&self) -> bool {
        *self == DACCH1CONVR::DISABLE
    }
    #[doc = "Checks if the value of the field is `CONTINUOUS`"]
    #[inline]
    pub fn is_continuous(&self) -> bool {
        *self == DACCH1CONVR::CONTINUOUS
    }
    #[doc = "Checks if the value of the field is `SAMPLEHOLD`"]
    #[inline]
    pub fn is_samplehold(&self) -> bool {
        *self == DACCH1CONVR::SAMPLEHOLD
    }
    #[doc = "Checks if the value of the field is `SAMPLEOFF`"]
    #[inline]
    pub fn is_sampleoff(&self) -> bool {
        *self == DACCH1CONVR::SAMPLEOFF
    }
}
#[doc = "Possible values of the field `DACCH0OUT`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DACCH0OUTR {
    #[doc = "DAC CH0 output to pin and ACMP/ADC disabled"]
    DISABLE,
    #[doc = "DAC CH0 output to pin enabled, output to ADC and ACMP disabled"]
    PIN,
    #[doc = "DAC CH0 output to pin disabled, output to ADC and ACMP enabled"]
    ADCACMP,
    #[doc = "DAC CH0 output to pin, ADC, and ACMP enabled."]
    PINADCACMP,
}
impl DACCH0OUTR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            DACCH0OUTR::DISABLE => 0,
            DACCH0OUTR::PIN => 0x01,
            DACCH0OUTR::ADCACMP => 0x02,
            DACCH0OUTR::PINADCACMP => 0x03,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> DACCH0OUTR {
        match value {
            0 => DACCH0OUTR::DISABLE,
            1 => DACCH0OUTR::PIN,
            2 => DACCH0OUTR::ADCACMP,
            3 => DACCH0OUTR::PINADCACMP,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline]
    pub fn is_disable(&self) -> bool {
        *self == DACCH0OUTR::DISABLE
    }
    #[doc = "Checks if the value of the field is `PIN`"]
    #[inline]
    pub fn is_pin(&self) -> bool {
        *self == DACCH0OUTR::PIN
    }
    #[doc = "Checks if the value of the field is `ADCACMP`"]
    #[inline]
    pub fn is_adcacmp(&self) -> bool {
        *self == DACCH0OUTR::ADCACMP
    }
    #[doc = "Checks if the value of the field is `PINADCACMP`"]
    #[inline]
    pub fn is_pinadcacmp(&self) -> bool {
        *self == DACCH0OUTR::PINADCACMP
    }
}
#[doc = "Possible values of the field `DACCH1OUT`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DACCH1OUTR {
    #[doc = "DAC CH1 output to pin and ACMP/ADC disabled"]
    DISABLE,
    #[doc = "DAC CH1 output to pin enabled, output to ADC and ACMP disabled"]
    PIN,
    #[doc = "DAC CH1 output to pin disabled, output to ADC and ACMP enabled"]
    ADCACMP,
    #[doc = "DAC CH1 output to pin, ADC, and ACMP enabled."]
    PINADCACMP,
}
impl DACCH1OUTR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            DACCH1OUTR::DISABLE => 0,
            DACCH1OUTR::PIN => 0x01,
            DACCH1OUTR::ADCACMP => 0x02,
            DACCH1OUTR::PINADCACMP => 0x03,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> DACCH1OUTR {
        match value {
            0 => DACCH1OUTR::DISABLE,
            1 => DACCH1OUTR::PIN,
            2 => DACCH1OUTR::ADCACMP,
            3 => DACCH1OUTR::PINADCACMP,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline]
    pub fn is_disable(&self) -> bool {
        *self == DACCH1OUTR::DISABLE
    }
    #[doc = "Checks if the value of the field is `PIN`"]
    #[inline]
    pub fn is_pin(&self) -> bool {
        *self == DACCH1OUTR::PIN
    }
    #[doc = "Checks if the value of the field is `ADCACMP`"]
    #[inline]
    pub fn is_adcacmp(&self) -> bool {
        *self == DACCH1OUTR::ADCACMP
    }
    #[doc = "Checks if the value of the field is `PINADCACMP`"]
    #[inline]
    pub fn is_pinadcacmp(&self) -> bool {
        *self == DACCH1OUTR::PINADCACMP
    }
}
#[doc = r" Value of the field"]
pub struct DACPRESCR {
    bits: u8,
}
impl DACPRESCR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct DACREFR {
    bits: bool,
}
impl DACREFR {
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
#[doc = "Possible values of the field `ACMP0MODE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ACMP0MODER {
    #[doc = "LESENSE does not control ACMP0"]
    DISABLE,
    #[doc = "LESENSE controls the input mux (POSSEL) of ACMP0"]
    MUX,
    #[doc = "LESENSE controls the input mux (POSSEL) and the threshold value (VDDLEVEL) of ACMP0"]
    MUXTHRES,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl ACMP0MODER {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            ACMP0MODER::DISABLE => 0,
            ACMP0MODER::MUX => 0x01,
            ACMP0MODER::MUXTHRES => 0x02,
            ACMP0MODER::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> ACMP0MODER {
        match value {
            0 => ACMP0MODER::DISABLE,
            1 => ACMP0MODER::MUX,
            2 => ACMP0MODER::MUXTHRES,
            i => ACMP0MODER::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline]
    pub fn is_disable(&self) -> bool {
        *self == ACMP0MODER::DISABLE
    }
    #[doc = "Checks if the value of the field is `MUX`"]
    #[inline]
    pub fn is_mux(&self) -> bool {
        *self == ACMP0MODER::MUX
    }
    #[doc = "Checks if the value of the field is `MUXTHRES`"]
    #[inline]
    pub fn is_muxthres(&self) -> bool {
        *self == ACMP0MODER::MUXTHRES
    }
}
#[doc = "Possible values of the field `ACMP1MODE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ACMP1MODER {
    #[doc = "LESENSE does not control ACMP1"]
    DISABLE,
    #[doc = "LESENSE controls the input mux (POSSEL) of ACMP1"]
    MUX,
    #[doc = "LESENSE controls the input mux and the threshold value (VDDLEVEL) of ACMP1"]
    MUXTHRES,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl ACMP1MODER {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            ACMP1MODER::DISABLE => 0,
            ACMP1MODER::MUX => 0x01,
            ACMP1MODER::MUXTHRES => 0x02,
            ACMP1MODER::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> ACMP1MODER {
        match value {
            0 => ACMP1MODER::DISABLE,
            1 => ACMP1MODER::MUX,
            2 => ACMP1MODER::MUXTHRES,
            i => ACMP1MODER::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline]
    pub fn is_disable(&self) -> bool {
        *self == ACMP1MODER::DISABLE
    }
    #[doc = "Checks if the value of the field is `MUX`"]
    #[inline]
    pub fn is_mux(&self) -> bool {
        *self == ACMP1MODER::MUX
    }
    #[doc = "Checks if the value of the field is `MUXTHRES`"]
    #[inline]
    pub fn is_muxthres(&self) -> bool {
        *self == ACMP1MODER::MUXTHRES
    }
}
#[doc = "Possible values of the field `WARMUPMODE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum WARMUPMODER {
    #[doc = "The analog comparators and DAC are shut down when LESENSE is idle"]
    NORMAL,
    #[doc = "The analog comparators are kept powered up when LESENSE is idle"]
    KEEPACMPWARM,
    #[doc = "The DAC is kept powered up when LESENSE is idle"]
    KEEPDACWARM,
    #[doc = "The analog comparators and DAC are kept powered up when LESENSE is idle"]
    KEEPACMPDACWARM,
}
impl WARMUPMODER {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            WARMUPMODER::NORMAL => 0,
            WARMUPMODER::KEEPACMPWARM => 0x01,
            WARMUPMODER::KEEPDACWARM => 0x02,
            WARMUPMODER::KEEPACMPDACWARM => 0x03,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> WARMUPMODER {
        match value {
            0 => WARMUPMODER::NORMAL,
            1 => WARMUPMODER::KEEPACMPWARM,
            2 => WARMUPMODER::KEEPDACWARM,
            3 => WARMUPMODER::KEEPACMPDACWARM,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `NORMAL`"]
    #[inline]
    pub fn is_normal(&self) -> bool {
        *self == WARMUPMODER::NORMAL
    }
    #[doc = "Checks if the value of the field is `KEEPACMPWARM`"]
    #[inline]
    pub fn is_keepacmpwarm(&self) -> bool {
        *self == WARMUPMODER::KEEPACMPWARM
    }
    #[doc = "Checks if the value of the field is `KEEPDACWARM`"]
    #[inline]
    pub fn is_keepdacwarm(&self) -> bool {
        *self == WARMUPMODER::KEEPDACWARM
    }
    #[doc = "Checks if the value of the field is `KEEPACMPDACWARM`"]
    #[inline]
    pub fn is_keepacmpdacwarm(&self) -> bool {
        *self == WARMUPMODER::KEEPACMPDACWARM
    }
}
#[doc = r" Proxy"]
pub struct _DACCH0DATAW<'a> {
    w: &'a mut W,
}
impl<'a> _DACCH0DATAW<'a> {
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
pub struct _DACCH1DATAW<'a> {
    w: &'a mut W,
}
impl<'a> _DACCH1DATAW<'a> {
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
#[doc = "Values that can be written to the field `DACCH0CONV`"]
pub enum DACCH0CONVW {
    #[doc = "LESENSE does not control DAC CH0."]
    DISABLE,
    #[doc = "DAC channel 0 is driven in continuous mode."]
    CONTINUOUS,
    #[doc = "DAC channel 0 is driven in sample hold mode."]
    SAMPLEHOLD,
    #[doc = "DAC channel 0 is driven in sample off mode."]
    SAMPLEOFF,
}
impl DACCH0CONVW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            DACCH0CONVW::DISABLE => 0,
            DACCH0CONVW::CONTINUOUS => 1,
            DACCH0CONVW::SAMPLEHOLD => 2,
            DACCH0CONVW::SAMPLEOFF => 3,
        }
    }
}
#[doc = r" Proxy"]
pub struct _DACCH0CONVW<'a> {
    w: &'a mut W,
}
impl<'a> _DACCH0CONVW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: DACCH0CONVW) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "LESENSE does not control DAC CH0."]
    #[inline]
    pub fn disable(self) -> &'a mut W {
        self.variant(DACCH0CONVW::DISABLE)
    }
    #[doc = "DAC channel 0 is driven in continuous mode."]
    #[inline]
    pub fn continuous(self) -> &'a mut W {
        self.variant(DACCH0CONVW::CONTINUOUS)
    }
    #[doc = "DAC channel 0 is driven in sample hold mode."]
    #[inline]
    pub fn samplehold(self) -> &'a mut W {
        self.variant(DACCH0CONVW::SAMPLEHOLD)
    }
    #[doc = "DAC channel 0 is driven in sample off mode."]
    #[inline]
    pub fn sampleoff(self) -> &'a mut W {
        self.variant(DACCH0CONVW::SAMPLEOFF)
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
#[doc = "Values that can be written to the field `DACCH1CONV`"]
pub enum DACCH1CONVW {
    #[doc = "LESENSE does not control DAC CH1."]
    DISABLE,
    #[doc = "DAC channel 1 is driven in continuous mode."]
    CONTINUOUS,
    #[doc = "DAC channel 1 is driven in sample hold mode."]
    SAMPLEHOLD,
    #[doc = "DAC channel 1 is driven in sample off mode."]
    SAMPLEOFF,
}
impl DACCH1CONVW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            DACCH1CONVW::DISABLE => 0,
            DACCH1CONVW::CONTINUOUS => 1,
            DACCH1CONVW::SAMPLEHOLD => 2,
            DACCH1CONVW::SAMPLEOFF => 3,
        }
    }
}
#[doc = r" Proxy"]
pub struct _DACCH1CONVW<'a> {
    w: &'a mut W,
}
impl<'a> _DACCH1CONVW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: DACCH1CONVW) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "LESENSE does not control DAC CH1."]
    #[inline]
    pub fn disable(self) -> &'a mut W {
        self.variant(DACCH1CONVW::DISABLE)
    }
    #[doc = "DAC channel 1 is driven in continuous mode."]
    #[inline]
    pub fn continuous(self) -> &'a mut W {
        self.variant(DACCH1CONVW::CONTINUOUS)
    }
    #[doc = "DAC channel 1 is driven in sample hold mode."]
    #[inline]
    pub fn samplehold(self) -> &'a mut W {
        self.variant(DACCH1CONVW::SAMPLEHOLD)
    }
    #[doc = "DAC channel 1 is driven in sample off mode."]
    #[inline]
    pub fn sampleoff(self) -> &'a mut W {
        self.variant(DACCH1CONVW::SAMPLEOFF)
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
#[doc = "Values that can be written to the field `DACCH0OUT`"]
pub enum DACCH0OUTW {
    #[doc = "DAC CH0 output to pin and ACMP/ADC disabled"]
    DISABLE,
    #[doc = "DAC CH0 output to pin enabled, output to ADC and ACMP disabled"]
    PIN,
    #[doc = "DAC CH0 output to pin disabled, output to ADC and ACMP enabled"]
    ADCACMP,
    #[doc = "DAC CH0 output to pin, ADC, and ACMP enabled."]
    PINADCACMP,
}
impl DACCH0OUTW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            DACCH0OUTW::DISABLE => 0,
            DACCH0OUTW::PIN => 1,
            DACCH0OUTW::ADCACMP => 2,
            DACCH0OUTW::PINADCACMP => 3,
        }
    }
}
#[doc = r" Proxy"]
pub struct _DACCH0OUTW<'a> {
    w: &'a mut W,
}
impl<'a> _DACCH0OUTW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: DACCH0OUTW) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "DAC CH0 output to pin and ACMP/ADC disabled"]
    #[inline]
    pub fn disable(self) -> &'a mut W {
        self.variant(DACCH0OUTW::DISABLE)
    }
    #[doc = "DAC CH0 output to pin enabled, output to ADC and ACMP disabled"]
    #[inline]
    pub fn pin(self) -> &'a mut W {
        self.variant(DACCH0OUTW::PIN)
    }
    #[doc = "DAC CH0 output to pin disabled, output to ADC and ACMP enabled"]
    #[inline]
    pub fn adcacmp(self) -> &'a mut W {
        self.variant(DACCH0OUTW::ADCACMP)
    }
    #[doc = "DAC CH0 output to pin, ADC, and ACMP enabled."]
    #[inline]
    pub fn pinadcacmp(self) -> &'a mut W {
        self.variant(DACCH0OUTW::PINADCACMP)
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
#[doc = "Values that can be written to the field `DACCH1OUT`"]
pub enum DACCH1OUTW {
    #[doc = "DAC CH1 output to pin and ACMP/ADC disabled"]
    DISABLE,
    #[doc = "DAC CH1 output to pin enabled, output to ADC and ACMP disabled"]
    PIN,
    #[doc = "DAC CH1 output to pin disabled, output to ADC and ACMP enabled"]
    ADCACMP,
    #[doc = "DAC CH1 output to pin, ADC, and ACMP enabled."]
    PINADCACMP,
}
impl DACCH1OUTW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            DACCH1OUTW::DISABLE => 0,
            DACCH1OUTW::PIN => 1,
            DACCH1OUTW::ADCACMP => 2,
            DACCH1OUTW::PINADCACMP => 3,
        }
    }
}
#[doc = r" Proxy"]
pub struct _DACCH1OUTW<'a> {
    w: &'a mut W,
}
impl<'a> _DACCH1OUTW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: DACCH1OUTW) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "DAC CH1 output to pin and ACMP/ADC disabled"]
    #[inline]
    pub fn disable(self) -> &'a mut W {
        self.variant(DACCH1OUTW::DISABLE)
    }
    #[doc = "DAC CH1 output to pin enabled, output to ADC and ACMP disabled"]
    #[inline]
    pub fn pin(self) -> &'a mut W {
        self.variant(DACCH1OUTW::PIN)
    }
    #[doc = "DAC CH1 output to pin disabled, output to ADC and ACMP enabled"]
    #[inline]
    pub fn adcacmp(self) -> &'a mut W {
        self.variant(DACCH1OUTW::ADCACMP)
    }
    #[doc = "DAC CH1 output to pin, ADC, and ACMP enabled."]
    #[inline]
    pub fn pinadcacmp(self) -> &'a mut W {
        self.variant(DACCH1OUTW::PINADCACMP)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 0x03;
        const OFFSET: u8 = 8;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _DACPRESCW<'a> {
    w: &'a mut W,
}
impl<'a> _DACPRESCW<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 0x1f;
        const OFFSET: u8 = 10;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _DACREFW<'a> {
    w: &'a mut W,
}
impl<'a> _DACREFW<'a> {
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
#[doc = "Values that can be written to the field `ACMP0MODE`"]
pub enum ACMP0MODEW {
    #[doc = "LESENSE does not control ACMP0"]
    DISABLE,
    #[doc = "LESENSE controls the input mux (POSSEL) of ACMP0"]
    MUX,
    #[doc = "LESENSE controls the input mux (POSSEL) and the threshold value (VDDLEVEL) of ACMP0"]
    MUXTHRES,
}
impl ACMP0MODEW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            ACMP0MODEW::DISABLE => 0,
            ACMP0MODEW::MUX => 1,
            ACMP0MODEW::MUXTHRES => 2,
        }
    }
}
#[doc = r" Proxy"]
pub struct _ACMP0MODEW<'a> {
    w: &'a mut W,
}
impl<'a> _ACMP0MODEW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: ACMP0MODEW) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "LESENSE does not control ACMP0"]
    #[inline]
    pub fn disable(self) -> &'a mut W {
        self.variant(ACMP0MODEW::DISABLE)
    }
    #[doc = "LESENSE controls the input mux (POSSEL) of ACMP0"]
    #[inline]
    pub fn mux(self) -> &'a mut W {
        self.variant(ACMP0MODEW::MUX)
    }
    #[doc = "LESENSE controls the input mux (POSSEL) and the threshold value (VDDLEVEL) of ACMP0"]
    #[inline]
    pub fn muxthres(self) -> &'a mut W {
        self.variant(ACMP0MODEW::MUXTHRES)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 0x03;
        const OFFSET: u8 = 20;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `ACMP1MODE`"]
pub enum ACMP1MODEW {
    #[doc = "LESENSE does not control ACMP1"]
    DISABLE,
    #[doc = "LESENSE controls the input mux (POSSEL) of ACMP1"]
    MUX,
    #[doc = "LESENSE controls the input mux and the threshold value (VDDLEVEL) of ACMP1"]
    MUXTHRES,
}
impl ACMP1MODEW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            ACMP1MODEW::DISABLE => 0,
            ACMP1MODEW::MUX => 1,
            ACMP1MODEW::MUXTHRES => 2,
        }
    }
}
#[doc = r" Proxy"]
pub struct _ACMP1MODEW<'a> {
    w: &'a mut W,
}
impl<'a> _ACMP1MODEW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: ACMP1MODEW) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "LESENSE does not control ACMP1"]
    #[inline]
    pub fn disable(self) -> &'a mut W {
        self.variant(ACMP1MODEW::DISABLE)
    }
    #[doc = "LESENSE controls the input mux (POSSEL) of ACMP1"]
    #[inline]
    pub fn mux(self) -> &'a mut W {
        self.variant(ACMP1MODEW::MUX)
    }
    #[doc = "LESENSE controls the input mux and the threshold value (VDDLEVEL) of ACMP1"]
    #[inline]
    pub fn muxthres(self) -> &'a mut W {
        self.variant(ACMP1MODEW::MUXTHRES)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 0x03;
        const OFFSET: u8 = 22;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `WARMUPMODE`"]
pub enum WARMUPMODEW {
    #[doc = "The analog comparators and DAC are shut down when LESENSE is idle"]
    NORMAL,
    #[doc = "The analog comparators are kept powered up when LESENSE is idle"]
    KEEPACMPWARM,
    #[doc = "The DAC is kept powered up when LESENSE is idle"]
    KEEPDACWARM,
    #[doc = "The analog comparators and DAC are kept powered up when LESENSE is idle"]
    KEEPACMPDACWARM,
}
impl WARMUPMODEW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            WARMUPMODEW::NORMAL => 0,
            WARMUPMODEW::KEEPACMPWARM => 1,
            WARMUPMODEW::KEEPDACWARM => 2,
            WARMUPMODEW::KEEPACMPDACWARM => 3,
        }
    }
}
#[doc = r" Proxy"]
pub struct _WARMUPMODEW<'a> {
    w: &'a mut W,
}
impl<'a> _WARMUPMODEW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: WARMUPMODEW) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "The analog comparators and DAC are shut down when LESENSE is idle"]
    #[inline]
    pub fn normal(self) -> &'a mut W {
        self.variant(WARMUPMODEW::NORMAL)
    }
    #[doc = "The analog comparators are kept powered up when LESENSE is idle"]
    #[inline]
    pub fn keepacmpwarm(self) -> &'a mut W {
        self.variant(WARMUPMODEW::KEEPACMPWARM)
    }
    #[doc = "The DAC is kept powered up when LESENSE is idle"]
    #[inline]
    pub fn keepdacwarm(self) -> &'a mut W {
        self.variant(WARMUPMODEW::KEEPDACWARM)
    }
    #[doc = "The analog comparators and DAC are kept powered up when LESENSE is idle"]
    #[inline]
    pub fn keepacmpdacwarm(self) -> &'a mut W {
        self.variant(WARMUPMODEW::KEEPACMPDACWARM)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 0x03;
        const OFFSET: u8 = 26;
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
    #[doc = "Bit 0 - DAC CH0 data selection."]
    #[inline]
    pub fn dacch0data(&self) -> DACCH0DATAR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        DACCH0DATAR { bits }
    }
    #[doc = "Bit 1 - DAC CH1 data selection."]
    #[inline]
    pub fn dacch1data(&self) -> DACCH1DATAR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 1;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        DACCH1DATAR { bits }
    }
    #[doc = "Bits 2:3 - DAC channel 0 conversion mode"]
    #[inline]
    pub fn dacch0conv(&self) -> DACCH0CONVR {
        DACCH0CONVR::_from({
            const MASK: u8 = 0x03;
            const OFFSET: u8 = 2;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 4:5 - DAC channel 1 conversion mode"]
    #[inline]
    pub fn dacch1conv(&self) -> DACCH1CONVR {
        DACCH1CONVR::_from({
            const MASK: u8 = 0x03;
            const OFFSET: u8 = 4;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 6:7 - DAC channel 0 output mode"]
    #[inline]
    pub fn dacch0out(&self) -> DACCH0OUTR {
        DACCH0OUTR::_from({
            const MASK: u8 = 0x03;
            const OFFSET: u8 = 6;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 8:9 - DAC channel 1 output mode"]
    #[inline]
    pub fn dacch1out(&self) -> DACCH1OUTR {
        DACCH1OUTR::_from({
            const MASK: u8 = 0x03;
            const OFFSET: u8 = 8;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 10:14 - DAC prescaler configuration."]
    #[inline]
    pub fn dacpresc(&self) -> DACPRESCR {
        let bits = {
            const MASK: u8 = 0x1f;
            const OFFSET: u8 = 10;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        DACPRESCR { bits }
    }
    #[doc = "Bit 18 - DAC bandgap reference used"]
    #[inline]
    pub fn dacref(&self) -> DACREFR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 18;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        DACREFR { bits }
    }
    #[doc = "Bits 20:21 - ACMP0 mode"]
    #[inline]
    pub fn acmp0mode(&self) -> ACMP0MODER {
        ACMP0MODER::_from({
            const MASK: u8 = 0x03;
            const OFFSET: u8 = 20;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 22:23 - ACMP1 mode"]
    #[inline]
    pub fn acmp1mode(&self) -> ACMP1MODER {
        ACMP1MODER::_from({
            const MASK: u8 = 0x03;
            const OFFSET: u8 = 22;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 26:27 - ACMP and DAC duty cycle mode"]
    #[inline]
    pub fn warmupmode(&self) -> WARMUPMODER {
        WARMUPMODER::_from({
            const MASK: u8 = 0x03;
            const OFFSET: u8 = 26;
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
    #[doc = "Bit 0 - DAC CH0 data selection."]
    #[inline]
    pub fn dacch0data(&mut self) -> _DACCH0DATAW {
        _DACCH0DATAW { w: self }
    }
    #[doc = "Bit 1 - DAC CH1 data selection."]
    #[inline]
    pub fn dacch1data(&mut self) -> _DACCH1DATAW {
        _DACCH1DATAW { w: self }
    }
    #[doc = "Bits 2:3 - DAC channel 0 conversion mode"]
    #[inline]
    pub fn dacch0conv(&mut self) -> _DACCH0CONVW {
        _DACCH0CONVW { w: self }
    }
    #[doc = "Bits 4:5 - DAC channel 1 conversion mode"]
    #[inline]
    pub fn dacch1conv(&mut self) -> _DACCH1CONVW {
        _DACCH1CONVW { w: self }
    }
    #[doc = "Bits 6:7 - DAC channel 0 output mode"]
    #[inline]
    pub fn dacch0out(&mut self) -> _DACCH0OUTW {
        _DACCH0OUTW { w: self }
    }
    #[doc = "Bits 8:9 - DAC channel 1 output mode"]
    #[inline]
    pub fn dacch1out(&mut self) -> _DACCH1OUTW {
        _DACCH1OUTW { w: self }
    }
    #[doc = "Bits 10:14 - DAC prescaler configuration."]
    #[inline]
    pub fn dacpresc(&mut self) -> _DACPRESCW {
        _DACPRESCW { w: self }
    }
    #[doc = "Bit 18 - DAC bandgap reference used"]
    #[inline]
    pub fn dacref(&mut self) -> _DACREFW {
        _DACREFW { w: self }
    }
    #[doc = "Bits 20:21 - ACMP0 mode"]
    #[inline]
    pub fn acmp0mode(&mut self) -> _ACMP0MODEW {
        _ACMP0MODEW { w: self }
    }
    #[doc = "Bits 22:23 - ACMP1 mode"]
    #[inline]
    pub fn acmp1mode(&mut self) -> _ACMP1MODEW {
        _ACMP1MODEW { w: self }
    }
    #[doc = "Bits 26:27 - ACMP and DAC duty cycle mode"]
    #[inline]
    pub fn warmupmode(&mut self) -> _WARMUPMODEW {
        _WARMUPMODEW { w: self }
    }
}

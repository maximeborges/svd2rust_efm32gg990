#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::OPA1MUX {
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
#[doc = "Possible values of the field `POSSEL`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum POSSELR {
    #[doc = "Input disabled"]
    DISABLE,
    #[doc = "DAC as input"]
    DAC,
    #[doc = "POS PAD as input"]
    POSPAD,
    #[doc = "OPA0 as input"]
    OPA0INP,
    #[doc = "OPA 1 Resistor ladder as input"]
    OPATAP,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl POSSELR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            POSSELR::DISABLE => 0,
            POSSELR::DAC => 0x01,
            POSSELR::POSPAD => 0x02,
            POSSELR::OPA0INP => 0x03,
            POSSELR::OPATAP => 0x04,
            POSSELR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> POSSELR {
        match value {
            0 => POSSELR::DISABLE,
            1 => POSSELR::DAC,
            2 => POSSELR::POSPAD,
            3 => POSSELR::OPA0INP,
            4 => POSSELR::OPATAP,
            i => POSSELR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline]
    pub fn is_disable(&self) -> bool {
        *self == POSSELR::DISABLE
    }
    #[doc = "Checks if the value of the field is `DAC`"]
    #[inline]
    pub fn is_dac(&self) -> bool {
        *self == POSSELR::DAC
    }
    #[doc = "Checks if the value of the field is `POSPAD`"]
    #[inline]
    pub fn is_pospad(&self) -> bool {
        *self == POSSELR::POSPAD
    }
    #[doc = "Checks if the value of the field is `OPA0INP`"]
    #[inline]
    pub fn is_opa0inp(&self) -> bool {
        *self == POSSELR::OPA0INP
    }
    #[doc = "Checks if the value of the field is `OPATAP`"]
    #[inline]
    pub fn is_opatap(&self) -> bool {
        *self == POSSELR::OPATAP
    }
}
#[doc = "Possible values of the field `NEGSEL`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum NEGSELR {
    #[doc = "Input disabled"]
    DISABLE,
    #[doc = "Unity Gain feedback path"]
    UG,
    #[doc = "OPA1 Resistor ladder as input"]
    OPATAP,
    #[doc = "Input from NEG PAD"]
    NEGPAD,
}
impl NEGSELR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            NEGSELR::DISABLE => 0,
            NEGSELR::UG => 0x01,
            NEGSELR::OPATAP => 0x02,
            NEGSELR::NEGPAD => 0x03,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> NEGSELR {
        match value {
            0 => NEGSELR::DISABLE,
            1 => NEGSELR::UG,
            2 => NEGSELR::OPATAP,
            3 => NEGSELR::NEGPAD,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline]
    pub fn is_disable(&self) -> bool {
        *self == NEGSELR::DISABLE
    }
    #[doc = "Checks if the value of the field is `UG`"]
    #[inline]
    pub fn is_ug(&self) -> bool {
        *self == NEGSELR::UG
    }
    #[doc = "Checks if the value of the field is `OPATAP`"]
    #[inline]
    pub fn is_opatap(&self) -> bool {
        *self == NEGSELR::OPATAP
    }
    #[doc = "Checks if the value of the field is `NEGPAD`"]
    #[inline]
    pub fn is_negpad(&self) -> bool {
        *self == NEGSELR::NEGPAD
    }
}
#[doc = "Possible values of the field `RESINMUX`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RESINMUXR {
    #[doc = "Set for Unity Gain"]
    DISABLE,
    #[doc = "Set for OPA0 input"]
    OPA0INP,
    #[doc = "NEG PAD connected"]
    NEGPAD,
    #[doc = "POS PAD connected"]
    POSPAD,
    #[doc = "VSS connected"]
    VSS,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl RESINMUXR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            RESINMUXR::DISABLE => 0,
            RESINMUXR::OPA0INP => 0x01,
            RESINMUXR::NEGPAD => 0x02,
            RESINMUXR::POSPAD => 0x03,
            RESINMUXR::VSS => 0x04,
            RESINMUXR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> RESINMUXR {
        match value {
            0 => RESINMUXR::DISABLE,
            1 => RESINMUXR::OPA0INP,
            2 => RESINMUXR::NEGPAD,
            3 => RESINMUXR::POSPAD,
            4 => RESINMUXR::VSS,
            i => RESINMUXR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline]
    pub fn is_disable(&self) -> bool {
        *self == RESINMUXR::DISABLE
    }
    #[doc = "Checks if the value of the field is `OPA0INP`"]
    #[inline]
    pub fn is_opa0inp(&self) -> bool {
        *self == RESINMUXR::OPA0INP
    }
    #[doc = "Checks if the value of the field is `NEGPAD`"]
    #[inline]
    pub fn is_negpad(&self) -> bool {
        *self == RESINMUXR::NEGPAD
    }
    #[doc = "Checks if the value of the field is `POSPAD`"]
    #[inline]
    pub fn is_pospad(&self) -> bool {
        *self == RESINMUXR::POSPAD
    }
    #[doc = "Checks if the value of the field is `VSS`"]
    #[inline]
    pub fn is_vss(&self) -> bool {
        *self == RESINMUXR::VSS
    }
}
#[doc = r" Value of the field"]
pub struct PPENR {
    bits: bool,
}
impl PPENR {
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
pub struct NPENR {
    bits: bool,
}
impl NPENR {
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
#[doc = "Possible values of the field `OUTPEN`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum OUTPENR {
    #[doc = "Alternate Output 0"]
    OUT0,
    #[doc = "Alternate Output 1"]
    OUT1,
    #[doc = "Alternate Output 2"]
    OUT2,
    #[doc = "Alternate Output 3"]
    OUT3,
    #[doc = "Alternate Output 4"]
    OUT4,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl OUTPENR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            OUTPENR::OUT0 => 0x01,
            OUTPENR::OUT1 => 0x02,
            OUTPENR::OUT2 => 0x04,
            OUTPENR::OUT3 => 0x08,
            OUTPENR::OUT4 => 0x10,
            OUTPENR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> OUTPENR {
        match value {
            1 => OUTPENR::OUT0,
            2 => OUTPENR::OUT1,
            4 => OUTPENR::OUT2,
            8 => OUTPENR::OUT3,
            16 => OUTPENR::OUT4,
            i => OUTPENR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `OUT0`"]
    #[inline]
    pub fn is_out0(&self) -> bool {
        *self == OUTPENR::OUT0
    }
    #[doc = "Checks if the value of the field is `OUT1`"]
    #[inline]
    pub fn is_out1(&self) -> bool {
        *self == OUTPENR::OUT1
    }
    #[doc = "Checks if the value of the field is `OUT2`"]
    #[inline]
    pub fn is_out2(&self) -> bool {
        *self == OUTPENR::OUT2
    }
    #[doc = "Checks if the value of the field is `OUT3`"]
    #[inline]
    pub fn is_out3(&self) -> bool {
        *self == OUTPENR::OUT3
    }
    #[doc = "Checks if the value of the field is `OUT4`"]
    #[inline]
    pub fn is_out4(&self) -> bool {
        *self == OUTPENR::OUT4
    }
}
#[doc = "Possible values of the field `OUTMODE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum OUTMODER {
    #[doc = "OPA0 output is disabled"]
    DISABLE,
    #[doc = "Main OPA1 output to pin enabled "]
    MAIN,
    #[doc = "OPA1 alternative output enabled."]
    ALT,
    #[doc = "Main OPA1 output drives both main and alternative outputs."]
    ALL,
}
impl OUTMODER {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            OUTMODER::DISABLE => 0,
            OUTMODER::MAIN => 0x01,
            OUTMODER::ALT => 0x02,
            OUTMODER::ALL => 0x03,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> OUTMODER {
        match value {
            0 => OUTMODER::DISABLE,
            1 => OUTMODER::MAIN,
            2 => OUTMODER::ALT,
            3 => OUTMODER::ALL,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline]
    pub fn is_disable(&self) -> bool {
        *self == OUTMODER::DISABLE
    }
    #[doc = "Checks if the value of the field is `MAIN`"]
    #[inline]
    pub fn is_main(&self) -> bool {
        *self == OUTMODER::MAIN
    }
    #[doc = "Checks if the value of the field is `ALT`"]
    #[inline]
    pub fn is_alt(&self) -> bool {
        *self == OUTMODER::ALT
    }
    #[doc = "Checks if the value of the field is `ALL`"]
    #[inline]
    pub fn is_all(&self) -> bool {
        *self == OUTMODER::ALL
    }
}
#[doc = r" Value of the field"]
pub struct NEXTOUTR {
    bits: bool,
}
impl NEXTOUTR {
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
#[doc = "Possible values of the field `RESSEL`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RESSELR {
    #[doc = "Gain of 1/3"]
    RES0,
    #[doc = "Gain of 1"]
    RES1,
    #[doc = "Gain of 1 2/3"]
    RES2,
    #[doc = "Gain of 2"]
    RES3,
    #[doc = "Gain of 3"]
    RES4,
    #[doc = "Gain of 4 1/3"]
    RES5,
    #[doc = "Gain of 7"]
    RES6,
    #[doc = "Gain of 15"]
    RES7,
}
impl RESSELR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            RESSELR::RES0 => 0,
            RESSELR::RES1 => 0x01,
            RESSELR::RES2 => 0x02,
            RESSELR::RES3 => 0x03,
            RESSELR::RES4 => 0x04,
            RESSELR::RES5 => 0x05,
            RESSELR::RES6 => 0x06,
            RESSELR::RES7 => 0x07,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> RESSELR {
        match value {
            0 => RESSELR::RES0,
            1 => RESSELR::RES1,
            2 => RESSELR::RES2,
            3 => RESSELR::RES3,
            4 => RESSELR::RES4,
            5 => RESSELR::RES5,
            6 => RESSELR::RES6,
            7 => RESSELR::RES7,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `RES0`"]
    #[inline]
    pub fn is_res0(&self) -> bool {
        *self == RESSELR::RES0
    }
    #[doc = "Checks if the value of the field is `RES1`"]
    #[inline]
    pub fn is_res1(&self) -> bool {
        *self == RESSELR::RES1
    }
    #[doc = "Checks if the value of the field is `RES2`"]
    #[inline]
    pub fn is_res2(&self) -> bool {
        *self == RESSELR::RES2
    }
    #[doc = "Checks if the value of the field is `RES3`"]
    #[inline]
    pub fn is_res3(&self) -> bool {
        *self == RESSELR::RES3
    }
    #[doc = "Checks if the value of the field is `RES4`"]
    #[inline]
    pub fn is_res4(&self) -> bool {
        *self == RESSELR::RES4
    }
    #[doc = "Checks if the value of the field is `RES5`"]
    #[inline]
    pub fn is_res5(&self) -> bool {
        *self == RESSELR::RES5
    }
    #[doc = "Checks if the value of the field is `RES6`"]
    #[inline]
    pub fn is_res6(&self) -> bool {
        *self == RESSELR::RES6
    }
    #[doc = "Checks if the value of the field is `RES7`"]
    #[inline]
    pub fn is_res7(&self) -> bool {
        *self == RESSELR::RES7
    }
}
#[doc = "Values that can be written to the field `POSSEL`"]
pub enum POSSELW {
    #[doc = "Input disabled"]
    DISABLE,
    #[doc = "DAC as input"]
    DAC,
    #[doc = "POS PAD as input"]
    POSPAD,
    #[doc = "OPA0 as input"]
    OPA0INP,
    #[doc = "OPA 1 Resistor ladder as input"]
    OPATAP,
}
impl POSSELW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            POSSELW::DISABLE => 0,
            POSSELW::DAC => 1,
            POSSELW::POSPAD => 2,
            POSSELW::OPA0INP => 3,
            POSSELW::OPATAP => 4,
        }
    }
}
#[doc = r" Proxy"]
pub struct _POSSELW<'a> {
    w: &'a mut W,
}
impl<'a> _POSSELW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: POSSELW) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "Input disabled"]
    #[inline]
    pub fn disable(self) -> &'a mut W {
        self.variant(POSSELW::DISABLE)
    }
    #[doc = "DAC as input"]
    #[inline]
    pub fn dac(self) -> &'a mut W {
        self.variant(POSSELW::DAC)
    }
    #[doc = "POS PAD as input"]
    #[inline]
    pub fn pospad(self) -> &'a mut W {
        self.variant(POSSELW::POSPAD)
    }
    #[doc = "OPA0 as input"]
    #[inline]
    pub fn opa0inp(self) -> &'a mut W {
        self.variant(POSSELW::OPA0INP)
    }
    #[doc = "OPA 1 Resistor ladder as input"]
    #[inline]
    pub fn opatap(self) -> &'a mut W {
        self.variant(POSSELW::OPATAP)
    }
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
#[doc = "Values that can be written to the field `NEGSEL`"]
pub enum NEGSELW {
    #[doc = "Input disabled"]
    DISABLE,
    #[doc = "Unity Gain feedback path"]
    UG,
    #[doc = "OPA1 Resistor ladder as input"]
    OPATAP,
    #[doc = "Input from NEG PAD"]
    NEGPAD,
}
impl NEGSELW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            NEGSELW::DISABLE => 0,
            NEGSELW::UG => 1,
            NEGSELW::OPATAP => 2,
            NEGSELW::NEGPAD => 3,
        }
    }
}
#[doc = r" Proxy"]
pub struct _NEGSELW<'a> {
    w: &'a mut W,
}
impl<'a> _NEGSELW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: NEGSELW) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "Input disabled"]
    #[inline]
    pub fn disable(self) -> &'a mut W {
        self.variant(NEGSELW::DISABLE)
    }
    #[doc = "Unity Gain feedback path"]
    #[inline]
    pub fn ug(self) -> &'a mut W {
        self.variant(NEGSELW::UG)
    }
    #[doc = "OPA1 Resistor ladder as input"]
    #[inline]
    pub fn opatap(self) -> &'a mut W {
        self.variant(NEGSELW::OPATAP)
    }
    #[doc = "Input from NEG PAD"]
    #[inline]
    pub fn negpad(self) -> &'a mut W {
        self.variant(NEGSELW::NEGPAD)
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
#[doc = "Values that can be written to the field `RESINMUX`"]
pub enum RESINMUXW {
    #[doc = "Set for Unity Gain"]
    DISABLE,
    #[doc = "Set for OPA0 input"]
    OPA0INP,
    #[doc = "NEG PAD connected"]
    NEGPAD,
    #[doc = "POS PAD connected"]
    POSPAD,
    #[doc = "VSS connected"]
    VSS,
}
impl RESINMUXW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            RESINMUXW::DISABLE => 0,
            RESINMUXW::OPA0INP => 1,
            RESINMUXW::NEGPAD => 2,
            RESINMUXW::POSPAD => 3,
            RESINMUXW::VSS => 4,
        }
    }
}
#[doc = r" Proxy"]
pub struct _RESINMUXW<'a> {
    w: &'a mut W,
}
impl<'a> _RESINMUXW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: RESINMUXW) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "Set for Unity Gain"]
    #[inline]
    pub fn disable(self) -> &'a mut W {
        self.variant(RESINMUXW::DISABLE)
    }
    #[doc = "Set for OPA0 input"]
    #[inline]
    pub fn opa0inp(self) -> &'a mut W {
        self.variant(RESINMUXW::OPA0INP)
    }
    #[doc = "NEG PAD connected"]
    #[inline]
    pub fn negpad(self) -> &'a mut W {
        self.variant(RESINMUXW::NEGPAD)
    }
    #[doc = "POS PAD connected"]
    #[inline]
    pub fn pospad(self) -> &'a mut W {
        self.variant(RESINMUXW::POSPAD)
    }
    #[doc = "VSS connected"]
    #[inline]
    pub fn vss(self) -> &'a mut W {
        self.variant(RESINMUXW::VSS)
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
pub struct _PPENW<'a> {
    w: &'a mut W,
}
impl<'a> _PPENW<'a> {
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
pub struct _NPENW<'a> {
    w: &'a mut W,
}
impl<'a> _NPENW<'a> {
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
#[doc = "Values that can be written to the field `OUTPEN`"]
pub enum OUTPENW {
    #[doc = "Alternate Output 0"]
    OUT0,
    #[doc = "Alternate Output 1"]
    OUT1,
    #[doc = "Alternate Output 2"]
    OUT2,
    #[doc = "Alternate Output 3"]
    OUT3,
    #[doc = "Alternate Output 4"]
    OUT4,
}
impl OUTPENW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            OUTPENW::OUT0 => 1,
            OUTPENW::OUT1 => 2,
            OUTPENW::OUT2 => 4,
            OUTPENW::OUT3 => 8,
            OUTPENW::OUT4 => 16,
        }
    }
}
#[doc = r" Proxy"]
pub struct _OUTPENW<'a> {
    w: &'a mut W,
}
impl<'a> _OUTPENW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: OUTPENW) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "Alternate Output 0"]
    #[inline]
    pub fn out0(self) -> &'a mut W {
        self.variant(OUTPENW::OUT0)
    }
    #[doc = "Alternate Output 1"]
    #[inline]
    pub fn out1(self) -> &'a mut W {
        self.variant(OUTPENW::OUT1)
    }
    #[doc = "Alternate Output 2"]
    #[inline]
    pub fn out2(self) -> &'a mut W {
        self.variant(OUTPENW::OUT2)
    }
    #[doc = "Alternate Output 3"]
    #[inline]
    pub fn out3(self) -> &'a mut W {
        self.variant(OUTPENW::OUT3)
    }
    #[doc = "Alternate Output 4"]
    #[inline]
    pub fn out4(self) -> &'a mut W {
        self.variant(OUTPENW::OUT4)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 0x1f;
        const OFFSET: u8 = 14;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `OUTMODE`"]
pub enum OUTMODEW {
    #[doc = "OPA0 output is disabled"]
    DISABLE,
    #[doc = "Main OPA1 output to pin enabled "]
    MAIN,
    #[doc = "OPA1 alternative output enabled."]
    ALT,
    #[doc = "Main OPA1 output drives both main and alternative outputs."]
    ALL,
}
impl OUTMODEW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            OUTMODEW::DISABLE => 0,
            OUTMODEW::MAIN => 1,
            OUTMODEW::ALT => 2,
            OUTMODEW::ALL => 3,
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
    #[doc = "OPA0 output is disabled"]
    #[inline]
    pub fn disable(self) -> &'a mut W {
        self.variant(OUTMODEW::DISABLE)
    }
    #[doc = "Main OPA1 output to pin enabled"]
    #[inline]
    pub fn main(self) -> &'a mut W {
        self.variant(OUTMODEW::MAIN)
    }
    #[doc = "OPA1 alternative output enabled."]
    #[inline]
    pub fn alt(self) -> &'a mut W {
        self.variant(OUTMODEW::ALT)
    }
    #[doc = "Main OPA1 output drives both main and alternative outputs."]
    #[inline]
    pub fn all(self) -> &'a mut W {
        self.variant(OUTMODEW::ALL)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 0x03;
        const OFFSET: u8 = 22;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _NEXTOUTW<'a> {
    w: &'a mut W,
}
impl<'a> _NEXTOUTW<'a> {
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
#[doc = "Values that can be written to the field `RESSEL`"]
pub enum RESSELW {
    #[doc = "Gain of 1/3"]
    RES0,
    #[doc = "Gain of 1"]
    RES1,
    #[doc = "Gain of 1 2/3"]
    RES2,
    #[doc = "Gain of 2"]
    RES3,
    #[doc = "Gain of 3"]
    RES4,
    #[doc = "Gain of 4 1/3"]
    RES5,
    #[doc = "Gain of 7"]
    RES6,
    #[doc = "Gain of 15"]
    RES7,
}
impl RESSELW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            RESSELW::RES0 => 0,
            RESSELW::RES1 => 1,
            RESSELW::RES2 => 2,
            RESSELW::RES3 => 3,
            RESSELW::RES4 => 4,
            RESSELW::RES5 => 5,
            RESSELW::RES6 => 6,
            RESSELW::RES7 => 7,
        }
    }
}
#[doc = r" Proxy"]
pub struct _RESSELW<'a> {
    w: &'a mut W,
}
impl<'a> _RESSELW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: RESSELW) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "Gain of 1/3"]
    #[inline]
    pub fn res0(self) -> &'a mut W {
        self.variant(RESSELW::RES0)
    }
    #[doc = "Gain of 1"]
    #[inline]
    pub fn res1(self) -> &'a mut W {
        self.variant(RESSELW::RES1)
    }
    #[doc = "Gain of 1 2/3"]
    #[inline]
    pub fn res2(self) -> &'a mut W {
        self.variant(RESSELW::RES2)
    }
    #[doc = "Gain of 2"]
    #[inline]
    pub fn res3(self) -> &'a mut W {
        self.variant(RESSELW::RES3)
    }
    #[doc = "Gain of 3"]
    #[inline]
    pub fn res4(self) -> &'a mut W {
        self.variant(RESSELW::RES4)
    }
    #[doc = "Gain of 4 1/3"]
    #[inline]
    pub fn res5(self) -> &'a mut W {
        self.variant(RESSELW::RES5)
    }
    #[doc = "Gain of 7"]
    #[inline]
    pub fn res6(self) -> &'a mut W {
        self.variant(RESSELW::RES6)
    }
    #[doc = "Gain of 15"]
    #[inline]
    pub fn res7(self) -> &'a mut W {
        self.variant(RESSELW::RES7)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bits(self, value: u8) -> &'a mut W {
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
    #[doc = "Bits 0:2 - OPA1 non-inverting Input Mux"]
    #[inline]
    pub fn possel(&self) -> POSSELR {
        POSSELR::_from({
            const MASK: u8 = 0x07;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 4:5 - OPA1 inverting Input Mux"]
    #[inline]
    pub fn negsel(&self) -> NEGSELR {
        NEGSELR::_from({
            const MASK: u8 = 0x03;
            const OFFSET: u8 = 4;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 8:10 - OPA1 Resistor Ladder Input Mux"]
    #[inline]
    pub fn resinmux(&self) -> RESINMUXR {
        RESINMUXR::_from({
            const MASK: u8 = 0x07;
            const OFFSET: u8 = 8;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bit 12 - OPA1 Positive Pad Input Enable"]
    #[inline]
    pub fn ppen(&self) -> PPENR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 12;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        PPENR { bits }
    }
    #[doc = "Bit 13 - OPA1 Negative Pad Input Enable"]
    #[inline]
    pub fn npen(&self) -> NPENR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 13;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        NPENR { bits }
    }
    #[doc = "Bits 14:18 - OPA1 Output Enable Value"]
    #[inline]
    pub fn outpen(&self) -> OUTPENR {
        OUTPENR::_from({
            const MASK: u8 = 0x1f;
            const OFFSET: u8 = 14;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 22:23 - Output Select"]
    #[inline]
    pub fn outmode(&self) -> OUTMODER {
        OUTMODER::_from({
            const MASK: u8 = 0x03;
            const OFFSET: u8 = 22;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bit 26 - OPA1 Next Enable"]
    #[inline]
    pub fn nextout(&self) -> NEXTOUTR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 26;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        NEXTOUTR { bits }
    }
    #[doc = "Bits 28:30 - OPA1 Resistor Ladder Select"]
    #[inline]
    pub fn ressel(&self) -> RESSELR {
        RESSELR::_from({
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
    #[doc = "Bits 0:2 - OPA1 non-inverting Input Mux"]
    #[inline]
    pub fn possel(&mut self) -> _POSSELW {
        _POSSELW { w: self }
    }
    #[doc = "Bits 4:5 - OPA1 inverting Input Mux"]
    #[inline]
    pub fn negsel(&mut self) -> _NEGSELW {
        _NEGSELW { w: self }
    }
    #[doc = "Bits 8:10 - OPA1 Resistor Ladder Input Mux"]
    #[inline]
    pub fn resinmux(&mut self) -> _RESINMUXW {
        _RESINMUXW { w: self }
    }
    #[doc = "Bit 12 - OPA1 Positive Pad Input Enable"]
    #[inline]
    pub fn ppen(&mut self) -> _PPENW {
        _PPENW { w: self }
    }
    #[doc = "Bit 13 - OPA1 Negative Pad Input Enable"]
    #[inline]
    pub fn npen(&mut self) -> _NPENW {
        _NPENW { w: self }
    }
    #[doc = "Bits 14:18 - OPA1 Output Enable Value"]
    #[inline]
    pub fn outpen(&mut self) -> _OUTPENW {
        _OUTPENW { w: self }
    }
    #[doc = "Bits 22:23 - Output Select"]
    #[inline]
    pub fn outmode(&mut self) -> _OUTMODEW {
        _OUTMODEW { w: self }
    }
    #[doc = "Bit 26 - OPA1 Next Enable"]
    #[inline]
    pub fn nextout(&mut self) -> _NEXTOUTW {
        _NEXTOUTW { w: self }
    }
    #[doc = "Bits 28:30 - OPA1 Resistor Ladder Select"]
    #[inline]
    pub fn ressel(&mut self) -> _RESSELW {
        _RESSELW { w: self }
    }
}

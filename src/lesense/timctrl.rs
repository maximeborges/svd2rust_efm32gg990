#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::TIMCTRL {
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
#[doc = "Possible values of the field `AUXPRESC`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum AUXPRESCR {
    #[doc = "High frequency timer is clocked with AUXHFRCO/1"]
    DIV1,
    #[doc = "High frequency timer is clocked with AUXHFRCO/2"]
    DIV2,
    #[doc = "High frequency timer is clocked with AUXHFRCO/4"]
    DIV4,
    #[doc = "High frequency timer is clocked with AUXHFRCO/8"]
    DIV8,
}
impl AUXPRESCR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            AUXPRESCR::DIV1 => 0,
            AUXPRESCR::DIV2 => 0x01,
            AUXPRESCR::DIV4 => 0x02,
            AUXPRESCR::DIV8 => 0x03,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> AUXPRESCR {
        match value {
            0 => AUXPRESCR::DIV1,
            1 => AUXPRESCR::DIV2,
            2 => AUXPRESCR::DIV4,
            3 => AUXPRESCR::DIV8,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `DIV1`"]
    #[inline]
    pub fn is_div1(&self) -> bool {
        *self == AUXPRESCR::DIV1
    }
    #[doc = "Checks if the value of the field is `DIV2`"]
    #[inline]
    pub fn is_div2(&self) -> bool {
        *self == AUXPRESCR::DIV2
    }
    #[doc = "Checks if the value of the field is `DIV4`"]
    #[inline]
    pub fn is_div4(&self) -> bool {
        *self == AUXPRESCR::DIV4
    }
    #[doc = "Checks if the value of the field is `DIV8`"]
    #[inline]
    pub fn is_div8(&self) -> bool {
        *self == AUXPRESCR::DIV8
    }
}
#[doc = "Possible values of the field `LFPRESC`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LFPRESCR {
    #[doc = "Low frequency timer is clocked with LFACLKLESENSE/1"]
    DIV1,
    #[doc = "Low frequency timer is clocked with LFACLKLESENSE/2"]
    DIV2,
    #[doc = "Low frequency timer is clocked with LFACLKLESENSE/4"]
    DIV4,
    #[doc = "Low frequency timer is clocked with LFACLKLESENSE/8"]
    DIV8,
    #[doc = "Low frequency timer is clocked with LFACLKLESENSE/16"]
    DIV16,
    #[doc = "Low frequency timer is clocked with LFACLKLESENSE/32"]
    DIV32,
    #[doc = "Low frequency timer is clocked with LFACLKLESENSE/64"]
    DIV64,
    #[doc = "Low frequency timer is clocked with LFACLKLESENSE/128"]
    DIV128,
}
impl LFPRESCR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            LFPRESCR::DIV1 => 0,
            LFPRESCR::DIV2 => 0x01,
            LFPRESCR::DIV4 => 0x02,
            LFPRESCR::DIV8 => 0x03,
            LFPRESCR::DIV16 => 0x04,
            LFPRESCR::DIV32 => 0x05,
            LFPRESCR::DIV64 => 0x06,
            LFPRESCR::DIV128 => 0x07,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> LFPRESCR {
        match value {
            0 => LFPRESCR::DIV1,
            1 => LFPRESCR::DIV2,
            2 => LFPRESCR::DIV4,
            3 => LFPRESCR::DIV8,
            4 => LFPRESCR::DIV16,
            5 => LFPRESCR::DIV32,
            6 => LFPRESCR::DIV64,
            7 => LFPRESCR::DIV128,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `DIV1`"]
    #[inline]
    pub fn is_div1(&self) -> bool {
        *self == LFPRESCR::DIV1
    }
    #[doc = "Checks if the value of the field is `DIV2`"]
    #[inline]
    pub fn is_div2(&self) -> bool {
        *self == LFPRESCR::DIV2
    }
    #[doc = "Checks if the value of the field is `DIV4`"]
    #[inline]
    pub fn is_div4(&self) -> bool {
        *self == LFPRESCR::DIV4
    }
    #[doc = "Checks if the value of the field is `DIV8`"]
    #[inline]
    pub fn is_div8(&self) -> bool {
        *self == LFPRESCR::DIV8
    }
    #[doc = "Checks if the value of the field is `DIV16`"]
    #[inline]
    pub fn is_div16(&self) -> bool {
        *self == LFPRESCR::DIV16
    }
    #[doc = "Checks if the value of the field is `DIV32`"]
    #[inline]
    pub fn is_div32(&self) -> bool {
        *self == LFPRESCR::DIV32
    }
    #[doc = "Checks if the value of the field is `DIV64`"]
    #[inline]
    pub fn is_div64(&self) -> bool {
        *self == LFPRESCR::DIV64
    }
    #[doc = "Checks if the value of the field is `DIV128`"]
    #[inline]
    pub fn is_div128(&self) -> bool {
        *self == LFPRESCR::DIV128
    }
}
#[doc = "Possible values of the field `PCPRESC`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PCPRESCR {
    #[doc = "The period counter clock frequency is LFACLKLESENSE/1"]
    DIV1,
    #[doc = "The period counter clock frequency is LFACLKLESENSE/2"]
    DIV2,
    #[doc = "The period counter clock frequency is LFACLKLESENSE/4"]
    DIV4,
    #[doc = "The period counter clock frequency is LFACLKLESENSE/8"]
    DIV8,
    #[doc = "The period counter clock frequency is LFACLKLESENSE/16"]
    DIV16,
    #[doc = "The period counter clock frequency is LFACLKLESENSE/32"]
    DIV32,
    #[doc = "The period counter clock frequency is LFACLKLESENSE/64"]
    DIV64,
    #[doc = "The period counter clock frequency is LFACLKLESENSE/128"]
    DIV128,
}
impl PCPRESCR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            PCPRESCR::DIV1 => 0,
            PCPRESCR::DIV2 => 0x01,
            PCPRESCR::DIV4 => 0x02,
            PCPRESCR::DIV8 => 0x03,
            PCPRESCR::DIV16 => 0x04,
            PCPRESCR::DIV32 => 0x05,
            PCPRESCR::DIV64 => 0x06,
            PCPRESCR::DIV128 => 0x07,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> PCPRESCR {
        match value {
            0 => PCPRESCR::DIV1,
            1 => PCPRESCR::DIV2,
            2 => PCPRESCR::DIV4,
            3 => PCPRESCR::DIV8,
            4 => PCPRESCR::DIV16,
            5 => PCPRESCR::DIV32,
            6 => PCPRESCR::DIV64,
            7 => PCPRESCR::DIV128,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `DIV1`"]
    #[inline]
    pub fn is_div1(&self) -> bool {
        *self == PCPRESCR::DIV1
    }
    #[doc = "Checks if the value of the field is `DIV2`"]
    #[inline]
    pub fn is_div2(&self) -> bool {
        *self == PCPRESCR::DIV2
    }
    #[doc = "Checks if the value of the field is `DIV4`"]
    #[inline]
    pub fn is_div4(&self) -> bool {
        *self == PCPRESCR::DIV4
    }
    #[doc = "Checks if the value of the field is `DIV8`"]
    #[inline]
    pub fn is_div8(&self) -> bool {
        *self == PCPRESCR::DIV8
    }
    #[doc = "Checks if the value of the field is `DIV16`"]
    #[inline]
    pub fn is_div16(&self) -> bool {
        *self == PCPRESCR::DIV16
    }
    #[doc = "Checks if the value of the field is `DIV32`"]
    #[inline]
    pub fn is_div32(&self) -> bool {
        *self == PCPRESCR::DIV32
    }
    #[doc = "Checks if the value of the field is `DIV64`"]
    #[inline]
    pub fn is_div64(&self) -> bool {
        *self == PCPRESCR::DIV64
    }
    #[doc = "Checks if the value of the field is `DIV128`"]
    #[inline]
    pub fn is_div128(&self) -> bool {
        *self == PCPRESCR::DIV128
    }
}
#[doc = r" Value of the field"]
pub struct PCTOPR {
    bits: u8,
}
impl PCTOPR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct STARTDLYR {
    bits: u8,
}
impl STARTDLYR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = "Values that can be written to the field `AUXPRESC`"]
pub enum AUXPRESCW {
    #[doc = "High frequency timer is clocked with AUXHFRCO/1"]
    DIV1,
    #[doc = "High frequency timer is clocked with AUXHFRCO/2"]
    DIV2,
    #[doc = "High frequency timer is clocked with AUXHFRCO/4"]
    DIV4,
    #[doc = "High frequency timer is clocked with AUXHFRCO/8"]
    DIV8,
}
impl AUXPRESCW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            AUXPRESCW::DIV1 => 0,
            AUXPRESCW::DIV2 => 1,
            AUXPRESCW::DIV4 => 2,
            AUXPRESCW::DIV8 => 3,
        }
    }
}
#[doc = r" Proxy"]
pub struct _AUXPRESCW<'a> {
    w: &'a mut W,
}
impl<'a> _AUXPRESCW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: AUXPRESCW) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "High frequency timer is clocked with AUXHFRCO/1"]
    #[inline]
    pub fn div1(self) -> &'a mut W {
        self.variant(AUXPRESCW::DIV1)
    }
    #[doc = "High frequency timer is clocked with AUXHFRCO/2"]
    #[inline]
    pub fn div2(self) -> &'a mut W {
        self.variant(AUXPRESCW::DIV2)
    }
    #[doc = "High frequency timer is clocked with AUXHFRCO/4"]
    #[inline]
    pub fn div4(self) -> &'a mut W {
        self.variant(AUXPRESCW::DIV4)
    }
    #[doc = "High frequency timer is clocked with AUXHFRCO/8"]
    #[inline]
    pub fn div8(self) -> &'a mut W {
        self.variant(AUXPRESCW::DIV8)
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
#[doc = "Values that can be written to the field `LFPRESC`"]
pub enum LFPRESCW {
    #[doc = "Low frequency timer is clocked with LFACLKLESENSE/1"]
    DIV1,
    #[doc = "Low frequency timer is clocked with LFACLKLESENSE/2"]
    DIV2,
    #[doc = "Low frequency timer is clocked with LFACLKLESENSE/4"]
    DIV4,
    #[doc = "Low frequency timer is clocked with LFACLKLESENSE/8"]
    DIV8,
    #[doc = "Low frequency timer is clocked with LFACLKLESENSE/16"]
    DIV16,
    #[doc = "Low frequency timer is clocked with LFACLKLESENSE/32"]
    DIV32,
    #[doc = "Low frequency timer is clocked with LFACLKLESENSE/64"]
    DIV64,
    #[doc = "Low frequency timer is clocked with LFACLKLESENSE/128"]
    DIV128,
}
impl LFPRESCW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            LFPRESCW::DIV1 => 0,
            LFPRESCW::DIV2 => 1,
            LFPRESCW::DIV4 => 2,
            LFPRESCW::DIV8 => 3,
            LFPRESCW::DIV16 => 4,
            LFPRESCW::DIV32 => 5,
            LFPRESCW::DIV64 => 6,
            LFPRESCW::DIV128 => 7,
        }
    }
}
#[doc = r" Proxy"]
pub struct _LFPRESCW<'a> {
    w: &'a mut W,
}
impl<'a> _LFPRESCW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: LFPRESCW) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "Low frequency timer is clocked with LFACLKLESENSE/1"]
    #[inline]
    pub fn div1(self) -> &'a mut W {
        self.variant(LFPRESCW::DIV1)
    }
    #[doc = "Low frequency timer is clocked with LFACLKLESENSE/2"]
    #[inline]
    pub fn div2(self) -> &'a mut W {
        self.variant(LFPRESCW::DIV2)
    }
    #[doc = "Low frequency timer is clocked with LFACLKLESENSE/4"]
    #[inline]
    pub fn div4(self) -> &'a mut W {
        self.variant(LFPRESCW::DIV4)
    }
    #[doc = "Low frequency timer is clocked with LFACLKLESENSE/8"]
    #[inline]
    pub fn div8(self) -> &'a mut W {
        self.variant(LFPRESCW::DIV8)
    }
    #[doc = "Low frequency timer is clocked with LFACLKLESENSE/16"]
    #[inline]
    pub fn div16(self) -> &'a mut W {
        self.variant(LFPRESCW::DIV16)
    }
    #[doc = "Low frequency timer is clocked with LFACLKLESENSE/32"]
    #[inline]
    pub fn div32(self) -> &'a mut W {
        self.variant(LFPRESCW::DIV32)
    }
    #[doc = "Low frequency timer is clocked with LFACLKLESENSE/64"]
    #[inline]
    pub fn div64(self) -> &'a mut W {
        self.variant(LFPRESCW::DIV64)
    }
    #[doc = "Low frequency timer is clocked with LFACLKLESENSE/128"]
    #[inline]
    pub fn div128(self) -> &'a mut W {
        self.variant(LFPRESCW::DIV128)
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
#[doc = "Values that can be written to the field `PCPRESC`"]
pub enum PCPRESCW {
    #[doc = "The period counter clock frequency is LFACLKLESENSE/1"]
    DIV1,
    #[doc = "The period counter clock frequency is LFACLKLESENSE/2"]
    DIV2,
    #[doc = "The period counter clock frequency is LFACLKLESENSE/4"]
    DIV4,
    #[doc = "The period counter clock frequency is LFACLKLESENSE/8"]
    DIV8,
    #[doc = "The period counter clock frequency is LFACLKLESENSE/16"]
    DIV16,
    #[doc = "The period counter clock frequency is LFACLKLESENSE/32"]
    DIV32,
    #[doc = "The period counter clock frequency is LFACLKLESENSE/64"]
    DIV64,
    #[doc = "The period counter clock frequency is LFACLKLESENSE/128"]
    DIV128,
}
impl PCPRESCW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            PCPRESCW::DIV1 => 0,
            PCPRESCW::DIV2 => 1,
            PCPRESCW::DIV4 => 2,
            PCPRESCW::DIV8 => 3,
            PCPRESCW::DIV16 => 4,
            PCPRESCW::DIV32 => 5,
            PCPRESCW::DIV64 => 6,
            PCPRESCW::DIV128 => 7,
        }
    }
}
#[doc = r" Proxy"]
pub struct _PCPRESCW<'a> {
    w: &'a mut W,
}
impl<'a> _PCPRESCW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: PCPRESCW) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "The period counter clock frequency is LFACLKLESENSE/1"]
    #[inline]
    pub fn div1(self) -> &'a mut W {
        self.variant(PCPRESCW::DIV1)
    }
    #[doc = "The period counter clock frequency is LFACLKLESENSE/2"]
    #[inline]
    pub fn div2(self) -> &'a mut W {
        self.variant(PCPRESCW::DIV2)
    }
    #[doc = "The period counter clock frequency is LFACLKLESENSE/4"]
    #[inline]
    pub fn div4(self) -> &'a mut W {
        self.variant(PCPRESCW::DIV4)
    }
    #[doc = "The period counter clock frequency is LFACLKLESENSE/8"]
    #[inline]
    pub fn div8(self) -> &'a mut W {
        self.variant(PCPRESCW::DIV8)
    }
    #[doc = "The period counter clock frequency is LFACLKLESENSE/16"]
    #[inline]
    pub fn div16(self) -> &'a mut W {
        self.variant(PCPRESCW::DIV16)
    }
    #[doc = "The period counter clock frequency is LFACLKLESENSE/32"]
    #[inline]
    pub fn div32(self) -> &'a mut W {
        self.variant(PCPRESCW::DIV32)
    }
    #[doc = "The period counter clock frequency is LFACLKLESENSE/64"]
    #[inline]
    pub fn div64(self) -> &'a mut W {
        self.variant(PCPRESCW::DIV64)
    }
    #[doc = "The period counter clock frequency is LFACLKLESENSE/128"]
    #[inline]
    pub fn div128(self) -> &'a mut W {
        self.variant(PCPRESCW::DIV128)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 0x07;
        const OFFSET: u8 = 8;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _PCTOPW<'a> {
    w: &'a mut W,
}
impl<'a> _PCTOPW<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 0xff;
        const OFFSET: u8 = 12;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _STARTDLYW<'a> {
    w: &'a mut W,
}
impl<'a> _STARTDLYW<'a> {
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
impl R {
    #[doc = r" Value of the register as raw bits"]
    #[inline]
    pub fn bits(&self) -> u32 {
        self.bits
    }
    #[doc = "Bits 0:1 - Prescaling factor for high frequency timer"]
    #[inline]
    pub fn auxpresc(&self) -> AUXPRESCR {
        AUXPRESCR::_from({
            const MASK: u8 = 0x03;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 4:6 - Prescaling factor for low frequency timer"]
    #[inline]
    pub fn lfpresc(&self) -> LFPRESCR {
        LFPRESCR::_from({
            const MASK: u8 = 0x07;
            const OFFSET: u8 = 4;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 8:10 - Period counter prescaling"]
    #[inline]
    pub fn pcpresc(&self) -> PCPRESCR {
        PCPRESCR::_from({
            const MASK: u8 = 0x07;
            const OFFSET: u8 = 8;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 12:19 - Period counter top value"]
    #[inline]
    pub fn pctop(&self) -> PCTOPR {
        let bits = {
            const MASK: u8 = 0xff;
            const OFFSET: u8 = 12;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        PCTOPR { bits }
    }
    #[doc = "Bits 22:23 - Start delay configuration"]
    #[inline]
    pub fn startdly(&self) -> STARTDLYR {
        let bits = {
            const MASK: u8 = 0x03;
            const OFFSET: u8 = 22;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        STARTDLYR { bits }
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
    #[doc = "Bits 0:1 - Prescaling factor for high frequency timer"]
    #[inline]
    pub fn auxpresc(&mut self) -> _AUXPRESCW {
        _AUXPRESCW { w: self }
    }
    #[doc = "Bits 4:6 - Prescaling factor for low frequency timer"]
    #[inline]
    pub fn lfpresc(&mut self) -> _LFPRESCW {
        _LFPRESCW { w: self }
    }
    #[doc = "Bits 8:10 - Period counter prescaling"]
    #[inline]
    pub fn pcpresc(&mut self) -> _PCPRESCW {
        _PCPRESCW { w: self }
    }
    #[doc = "Bits 12:19 - Period counter top value"]
    #[inline]
    pub fn pctop(&mut self) -> _PCTOPW {
        _PCTOPW { w: self }
    }
    #[doc = "Bits 22:23 - Start delay configuration"]
    #[inline]
    pub fn startdly(&mut self) -> _STARTDLYW {
        _STARTDLYW { w: self }
    }
}

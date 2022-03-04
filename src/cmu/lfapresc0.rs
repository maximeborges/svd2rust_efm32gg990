#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::LFAPRESC0 {
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
#[doc = "Possible values of the field `LESENSE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LESENSER {
    #[doc = "LFACLKLESENSE = LFACLK"]
    DIV1,
    #[doc = "LFACLKLESENSE = LFACLK/2"]
    DIV2,
    #[doc = "LFACLKLESENSE = LFACLK/4"]
    DIV4,
    #[doc = "LFACLKLESENSE = LFACLK/8"]
    DIV8,
}
impl LESENSER {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            LESENSER::DIV1 => 0,
            LESENSER::DIV2 => 0x01,
            LESENSER::DIV4 => 0x02,
            LESENSER::DIV8 => 0x03,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> LESENSER {
        match value {
            0 => LESENSER::DIV1,
            1 => LESENSER::DIV2,
            2 => LESENSER::DIV4,
            3 => LESENSER::DIV8,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `DIV1`"]
    #[inline]
    pub fn is_div1(&self) -> bool {
        *self == LESENSER::DIV1
    }
    #[doc = "Checks if the value of the field is `DIV2`"]
    #[inline]
    pub fn is_div2(&self) -> bool {
        *self == LESENSER::DIV2
    }
    #[doc = "Checks if the value of the field is `DIV4`"]
    #[inline]
    pub fn is_div4(&self) -> bool {
        *self == LESENSER::DIV4
    }
    #[doc = "Checks if the value of the field is `DIV8`"]
    #[inline]
    pub fn is_div8(&self) -> bool {
        *self == LESENSER::DIV8
    }
}
#[doc = "Possible values of the field `RTC`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RTCR {
    #[doc = "LFACLKRTC = LFACLK"]
    DIV1,
    #[doc = "LFACLKRTC = LFACLK/2"]
    DIV2,
    #[doc = "LFACLKRTC = LFACLK/4"]
    DIV4,
    #[doc = "LFACLKRTC = LFACLK/8"]
    DIV8,
    #[doc = "LFACLKRTC = LFACLK/16"]
    DIV16,
    #[doc = "LFACLKRTC = LFACLK/32"]
    DIV32,
    #[doc = "LFACLKRTC = LFACLK/64"]
    DIV64,
    #[doc = "LFACLKRTC = LFACLK/128"]
    DIV128,
    #[doc = "LFACLKRTC = LFACLK/256"]
    DIV256,
    #[doc = "LFACLKRTC = LFACLK/512"]
    DIV512,
    #[doc = "LFACLKRTC = LFACLK/1024"]
    DIV1024,
    #[doc = "LFACLKRTC = LFACLK/2048"]
    DIV2048,
    #[doc = "LFACLKRTC = LFACLK/4096"]
    DIV4096,
    #[doc = "LFACLKRTC = LFACLK/8192"]
    DIV8192,
    #[doc = "LFACLKRTC = LFACLK/16384"]
    DIV16384,
    #[doc = "LFACLKRTC = LFACLK/32768"]
    DIV32768,
}
impl RTCR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            RTCR::DIV1 => 0,
            RTCR::DIV2 => 0x01,
            RTCR::DIV4 => 0x02,
            RTCR::DIV8 => 0x03,
            RTCR::DIV16 => 0x04,
            RTCR::DIV32 => 0x05,
            RTCR::DIV64 => 0x06,
            RTCR::DIV128 => 0x07,
            RTCR::DIV256 => 0x08,
            RTCR::DIV512 => 0x09,
            RTCR::DIV1024 => 0x0a,
            RTCR::DIV2048 => 0x0b,
            RTCR::DIV4096 => 0x0c,
            RTCR::DIV8192 => 0x0d,
            RTCR::DIV16384 => 0x0e,
            RTCR::DIV32768 => 0x0f,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> RTCR {
        match value {
            0 => RTCR::DIV1,
            1 => RTCR::DIV2,
            2 => RTCR::DIV4,
            3 => RTCR::DIV8,
            4 => RTCR::DIV16,
            5 => RTCR::DIV32,
            6 => RTCR::DIV64,
            7 => RTCR::DIV128,
            8 => RTCR::DIV256,
            9 => RTCR::DIV512,
            10 => RTCR::DIV1024,
            11 => RTCR::DIV2048,
            12 => RTCR::DIV4096,
            13 => RTCR::DIV8192,
            14 => RTCR::DIV16384,
            15 => RTCR::DIV32768,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `DIV1`"]
    #[inline]
    pub fn is_div1(&self) -> bool {
        *self == RTCR::DIV1
    }
    #[doc = "Checks if the value of the field is `DIV2`"]
    #[inline]
    pub fn is_div2(&self) -> bool {
        *self == RTCR::DIV2
    }
    #[doc = "Checks if the value of the field is `DIV4`"]
    #[inline]
    pub fn is_div4(&self) -> bool {
        *self == RTCR::DIV4
    }
    #[doc = "Checks if the value of the field is `DIV8`"]
    #[inline]
    pub fn is_div8(&self) -> bool {
        *self == RTCR::DIV8
    }
    #[doc = "Checks if the value of the field is `DIV16`"]
    #[inline]
    pub fn is_div16(&self) -> bool {
        *self == RTCR::DIV16
    }
    #[doc = "Checks if the value of the field is `DIV32`"]
    #[inline]
    pub fn is_div32(&self) -> bool {
        *self == RTCR::DIV32
    }
    #[doc = "Checks if the value of the field is `DIV64`"]
    #[inline]
    pub fn is_div64(&self) -> bool {
        *self == RTCR::DIV64
    }
    #[doc = "Checks if the value of the field is `DIV128`"]
    #[inline]
    pub fn is_div128(&self) -> bool {
        *self == RTCR::DIV128
    }
    #[doc = "Checks if the value of the field is `DIV256`"]
    #[inline]
    pub fn is_div256(&self) -> bool {
        *self == RTCR::DIV256
    }
    #[doc = "Checks if the value of the field is `DIV512`"]
    #[inline]
    pub fn is_div512(&self) -> bool {
        *self == RTCR::DIV512
    }
    #[doc = "Checks if the value of the field is `DIV1024`"]
    #[inline]
    pub fn is_div1024(&self) -> bool {
        *self == RTCR::DIV1024
    }
    #[doc = "Checks if the value of the field is `DIV2048`"]
    #[inline]
    pub fn is_div2048(&self) -> bool {
        *self == RTCR::DIV2048
    }
    #[doc = "Checks if the value of the field is `DIV4096`"]
    #[inline]
    pub fn is_div4096(&self) -> bool {
        *self == RTCR::DIV4096
    }
    #[doc = "Checks if the value of the field is `DIV8192`"]
    #[inline]
    pub fn is_div8192(&self) -> bool {
        *self == RTCR::DIV8192
    }
    #[doc = "Checks if the value of the field is `DIV16384`"]
    #[inline]
    pub fn is_div16384(&self) -> bool {
        *self == RTCR::DIV16384
    }
    #[doc = "Checks if the value of the field is `DIV32768`"]
    #[inline]
    pub fn is_div32768(&self) -> bool {
        *self == RTCR::DIV32768
    }
}
#[doc = "Possible values of the field `LETIMER0`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LETIMER0R {
    #[doc = "LFACLKLETIMER0 = LFACLK"]
    DIV1,
    #[doc = "LFACLKLETIMER0 = LFACLK/2"]
    DIV2,
    #[doc = "LFACLKLETIMER0 = LFACLK/4"]
    DIV4,
    #[doc = "LFACLKLETIMER0 = LFACLK/8"]
    DIV8,
    #[doc = "LFACLKLETIMER0 = LFACLK/16"]
    DIV16,
    #[doc = "LFACLKLETIMER0 = LFACLK/32"]
    DIV32,
    #[doc = "LFACLKLETIMER0 = LFACLK/64"]
    DIV64,
    #[doc = "LFACLKLETIMER0 = LFACLK/128"]
    DIV128,
    #[doc = "LFACLKLETIMER0 = LFACLK/256"]
    DIV256,
    #[doc = "LFACLKLETIMER0 = LFACLK/512"]
    DIV512,
    #[doc = "LFACLKLETIMER0 = LFACLK/1024"]
    DIV1024,
    #[doc = "LFACLKLETIMER0 = LFACLK/2048"]
    DIV2048,
    #[doc = "LFACLKLETIMER0 = LFACLK/4096"]
    DIV4096,
    #[doc = "LFACLKLETIMER0 = LFACLK/8192"]
    DIV8192,
    #[doc = "LFACLKLETIMER0 = LFACLK/16384"]
    DIV16384,
    #[doc = "LFACLKLETIMER0 = LFACLK/32768"]
    DIV32768,
}
impl LETIMER0R {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            LETIMER0R::DIV1 => 0,
            LETIMER0R::DIV2 => 0x01,
            LETIMER0R::DIV4 => 0x02,
            LETIMER0R::DIV8 => 0x03,
            LETIMER0R::DIV16 => 0x04,
            LETIMER0R::DIV32 => 0x05,
            LETIMER0R::DIV64 => 0x06,
            LETIMER0R::DIV128 => 0x07,
            LETIMER0R::DIV256 => 0x08,
            LETIMER0R::DIV512 => 0x09,
            LETIMER0R::DIV1024 => 0x0a,
            LETIMER0R::DIV2048 => 0x0b,
            LETIMER0R::DIV4096 => 0x0c,
            LETIMER0R::DIV8192 => 0x0d,
            LETIMER0R::DIV16384 => 0x0e,
            LETIMER0R::DIV32768 => 0x0f,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> LETIMER0R {
        match value {
            0 => LETIMER0R::DIV1,
            1 => LETIMER0R::DIV2,
            2 => LETIMER0R::DIV4,
            3 => LETIMER0R::DIV8,
            4 => LETIMER0R::DIV16,
            5 => LETIMER0R::DIV32,
            6 => LETIMER0R::DIV64,
            7 => LETIMER0R::DIV128,
            8 => LETIMER0R::DIV256,
            9 => LETIMER0R::DIV512,
            10 => LETIMER0R::DIV1024,
            11 => LETIMER0R::DIV2048,
            12 => LETIMER0R::DIV4096,
            13 => LETIMER0R::DIV8192,
            14 => LETIMER0R::DIV16384,
            15 => LETIMER0R::DIV32768,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `DIV1`"]
    #[inline]
    pub fn is_div1(&self) -> bool {
        *self == LETIMER0R::DIV1
    }
    #[doc = "Checks if the value of the field is `DIV2`"]
    #[inline]
    pub fn is_div2(&self) -> bool {
        *self == LETIMER0R::DIV2
    }
    #[doc = "Checks if the value of the field is `DIV4`"]
    #[inline]
    pub fn is_div4(&self) -> bool {
        *self == LETIMER0R::DIV4
    }
    #[doc = "Checks if the value of the field is `DIV8`"]
    #[inline]
    pub fn is_div8(&self) -> bool {
        *self == LETIMER0R::DIV8
    }
    #[doc = "Checks if the value of the field is `DIV16`"]
    #[inline]
    pub fn is_div16(&self) -> bool {
        *self == LETIMER0R::DIV16
    }
    #[doc = "Checks if the value of the field is `DIV32`"]
    #[inline]
    pub fn is_div32(&self) -> bool {
        *self == LETIMER0R::DIV32
    }
    #[doc = "Checks if the value of the field is `DIV64`"]
    #[inline]
    pub fn is_div64(&self) -> bool {
        *self == LETIMER0R::DIV64
    }
    #[doc = "Checks if the value of the field is `DIV128`"]
    #[inline]
    pub fn is_div128(&self) -> bool {
        *self == LETIMER0R::DIV128
    }
    #[doc = "Checks if the value of the field is `DIV256`"]
    #[inline]
    pub fn is_div256(&self) -> bool {
        *self == LETIMER0R::DIV256
    }
    #[doc = "Checks if the value of the field is `DIV512`"]
    #[inline]
    pub fn is_div512(&self) -> bool {
        *self == LETIMER0R::DIV512
    }
    #[doc = "Checks if the value of the field is `DIV1024`"]
    #[inline]
    pub fn is_div1024(&self) -> bool {
        *self == LETIMER0R::DIV1024
    }
    #[doc = "Checks if the value of the field is `DIV2048`"]
    #[inline]
    pub fn is_div2048(&self) -> bool {
        *self == LETIMER0R::DIV2048
    }
    #[doc = "Checks if the value of the field is `DIV4096`"]
    #[inline]
    pub fn is_div4096(&self) -> bool {
        *self == LETIMER0R::DIV4096
    }
    #[doc = "Checks if the value of the field is `DIV8192`"]
    #[inline]
    pub fn is_div8192(&self) -> bool {
        *self == LETIMER0R::DIV8192
    }
    #[doc = "Checks if the value of the field is `DIV16384`"]
    #[inline]
    pub fn is_div16384(&self) -> bool {
        *self == LETIMER0R::DIV16384
    }
    #[doc = "Checks if the value of the field is `DIV32768`"]
    #[inline]
    pub fn is_div32768(&self) -> bool {
        *self == LETIMER0R::DIV32768
    }
}
#[doc = "Possible values of the field `LCD`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LCDR {
    #[doc = "LFACLKLCD = LFACLK/16"]
    DIV16,
    #[doc = "LFACLKLCD = LFACLK/32"]
    DIV32,
    #[doc = "LFACLKLCD = LFACLK/64"]
    DIV64,
    #[doc = "LFACLKLCD = LFACLK/128"]
    DIV128,
}
impl LCDR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            LCDR::DIV16 => 0,
            LCDR::DIV32 => 0x01,
            LCDR::DIV64 => 0x02,
            LCDR::DIV128 => 0x03,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> LCDR {
        match value {
            0 => LCDR::DIV16,
            1 => LCDR::DIV32,
            2 => LCDR::DIV64,
            3 => LCDR::DIV128,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `DIV16`"]
    #[inline]
    pub fn is_div16(&self) -> bool {
        *self == LCDR::DIV16
    }
    #[doc = "Checks if the value of the field is `DIV32`"]
    #[inline]
    pub fn is_div32(&self) -> bool {
        *self == LCDR::DIV32
    }
    #[doc = "Checks if the value of the field is `DIV64`"]
    #[inline]
    pub fn is_div64(&self) -> bool {
        *self == LCDR::DIV64
    }
    #[doc = "Checks if the value of the field is `DIV128`"]
    #[inline]
    pub fn is_div128(&self) -> bool {
        *self == LCDR::DIV128
    }
}
#[doc = "Values that can be written to the field `LESENSE`"]
pub enum LESENSEW {
    #[doc = "LFACLKLESENSE = LFACLK"]
    DIV1,
    #[doc = "LFACLKLESENSE = LFACLK/2"]
    DIV2,
    #[doc = "LFACLKLESENSE = LFACLK/4"]
    DIV4,
    #[doc = "LFACLKLESENSE = LFACLK/8"]
    DIV8,
}
impl LESENSEW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            LESENSEW::DIV1 => 0,
            LESENSEW::DIV2 => 1,
            LESENSEW::DIV4 => 2,
            LESENSEW::DIV8 => 3,
        }
    }
}
#[doc = r" Proxy"]
pub struct _LESENSEW<'a> {
    w: &'a mut W,
}
impl<'a> _LESENSEW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: LESENSEW) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "LFACLKLESENSE = LFACLK"]
    #[inline]
    pub fn div1(self) -> &'a mut W {
        self.variant(LESENSEW::DIV1)
    }
    #[doc = "LFACLKLESENSE = LFACLK/2"]
    #[inline]
    pub fn div2(self) -> &'a mut W {
        self.variant(LESENSEW::DIV2)
    }
    #[doc = "LFACLKLESENSE = LFACLK/4"]
    #[inline]
    pub fn div4(self) -> &'a mut W {
        self.variant(LESENSEW::DIV4)
    }
    #[doc = "LFACLKLESENSE = LFACLK/8"]
    #[inline]
    pub fn div8(self) -> &'a mut W {
        self.variant(LESENSEW::DIV8)
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
#[doc = "Values that can be written to the field `RTC`"]
pub enum RTCW {
    #[doc = "LFACLKRTC = LFACLK"]
    DIV1,
    #[doc = "LFACLKRTC = LFACLK/2"]
    DIV2,
    #[doc = "LFACLKRTC = LFACLK/4"]
    DIV4,
    #[doc = "LFACLKRTC = LFACLK/8"]
    DIV8,
    #[doc = "LFACLKRTC = LFACLK/16"]
    DIV16,
    #[doc = "LFACLKRTC = LFACLK/32"]
    DIV32,
    #[doc = "LFACLKRTC = LFACLK/64"]
    DIV64,
    #[doc = "LFACLKRTC = LFACLK/128"]
    DIV128,
    #[doc = "LFACLKRTC = LFACLK/256"]
    DIV256,
    #[doc = "LFACLKRTC = LFACLK/512"]
    DIV512,
    #[doc = "LFACLKRTC = LFACLK/1024"]
    DIV1024,
    #[doc = "LFACLKRTC = LFACLK/2048"]
    DIV2048,
    #[doc = "LFACLKRTC = LFACLK/4096"]
    DIV4096,
    #[doc = "LFACLKRTC = LFACLK/8192"]
    DIV8192,
    #[doc = "LFACLKRTC = LFACLK/16384"]
    DIV16384,
    #[doc = "LFACLKRTC = LFACLK/32768"]
    DIV32768,
}
impl RTCW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            RTCW::DIV1 => 0,
            RTCW::DIV2 => 1,
            RTCW::DIV4 => 2,
            RTCW::DIV8 => 3,
            RTCW::DIV16 => 4,
            RTCW::DIV32 => 5,
            RTCW::DIV64 => 6,
            RTCW::DIV128 => 7,
            RTCW::DIV256 => 8,
            RTCW::DIV512 => 9,
            RTCW::DIV1024 => 10,
            RTCW::DIV2048 => 11,
            RTCW::DIV4096 => 12,
            RTCW::DIV8192 => 13,
            RTCW::DIV16384 => 14,
            RTCW::DIV32768 => 15,
        }
    }
}
#[doc = r" Proxy"]
pub struct _RTCW<'a> {
    w: &'a mut W,
}
impl<'a> _RTCW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: RTCW) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "LFACLKRTC = LFACLK"]
    #[inline]
    pub fn div1(self) -> &'a mut W {
        self.variant(RTCW::DIV1)
    }
    #[doc = "LFACLKRTC = LFACLK/2"]
    #[inline]
    pub fn div2(self) -> &'a mut W {
        self.variant(RTCW::DIV2)
    }
    #[doc = "LFACLKRTC = LFACLK/4"]
    #[inline]
    pub fn div4(self) -> &'a mut W {
        self.variant(RTCW::DIV4)
    }
    #[doc = "LFACLKRTC = LFACLK/8"]
    #[inline]
    pub fn div8(self) -> &'a mut W {
        self.variant(RTCW::DIV8)
    }
    #[doc = "LFACLKRTC = LFACLK/16"]
    #[inline]
    pub fn div16(self) -> &'a mut W {
        self.variant(RTCW::DIV16)
    }
    #[doc = "LFACLKRTC = LFACLK/32"]
    #[inline]
    pub fn div32(self) -> &'a mut W {
        self.variant(RTCW::DIV32)
    }
    #[doc = "LFACLKRTC = LFACLK/64"]
    #[inline]
    pub fn div64(self) -> &'a mut W {
        self.variant(RTCW::DIV64)
    }
    #[doc = "LFACLKRTC = LFACLK/128"]
    #[inline]
    pub fn div128(self) -> &'a mut W {
        self.variant(RTCW::DIV128)
    }
    #[doc = "LFACLKRTC = LFACLK/256"]
    #[inline]
    pub fn div256(self) -> &'a mut W {
        self.variant(RTCW::DIV256)
    }
    #[doc = "LFACLKRTC = LFACLK/512"]
    #[inline]
    pub fn div512(self) -> &'a mut W {
        self.variant(RTCW::DIV512)
    }
    #[doc = "LFACLKRTC = LFACLK/1024"]
    #[inline]
    pub fn div1024(self) -> &'a mut W {
        self.variant(RTCW::DIV1024)
    }
    #[doc = "LFACLKRTC = LFACLK/2048"]
    #[inline]
    pub fn div2048(self) -> &'a mut W {
        self.variant(RTCW::DIV2048)
    }
    #[doc = "LFACLKRTC = LFACLK/4096"]
    #[inline]
    pub fn div4096(self) -> &'a mut W {
        self.variant(RTCW::DIV4096)
    }
    #[doc = "LFACLKRTC = LFACLK/8192"]
    #[inline]
    pub fn div8192(self) -> &'a mut W {
        self.variant(RTCW::DIV8192)
    }
    #[doc = "LFACLKRTC = LFACLK/16384"]
    #[inline]
    pub fn div16384(self) -> &'a mut W {
        self.variant(RTCW::DIV16384)
    }
    #[doc = "LFACLKRTC = LFACLK/32768"]
    #[inline]
    pub fn div32768(self) -> &'a mut W {
        self.variant(RTCW::DIV32768)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 0x0f;
        const OFFSET: u8 = 4;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `LETIMER0`"]
pub enum LETIMER0W {
    #[doc = "LFACLKLETIMER0 = LFACLK"]
    DIV1,
    #[doc = "LFACLKLETIMER0 = LFACLK/2"]
    DIV2,
    #[doc = "LFACLKLETIMER0 = LFACLK/4"]
    DIV4,
    #[doc = "LFACLKLETIMER0 = LFACLK/8"]
    DIV8,
    #[doc = "LFACLKLETIMER0 = LFACLK/16"]
    DIV16,
    #[doc = "LFACLKLETIMER0 = LFACLK/32"]
    DIV32,
    #[doc = "LFACLKLETIMER0 = LFACLK/64"]
    DIV64,
    #[doc = "LFACLKLETIMER0 = LFACLK/128"]
    DIV128,
    #[doc = "LFACLKLETIMER0 = LFACLK/256"]
    DIV256,
    #[doc = "LFACLKLETIMER0 = LFACLK/512"]
    DIV512,
    #[doc = "LFACLKLETIMER0 = LFACLK/1024"]
    DIV1024,
    #[doc = "LFACLKLETIMER0 = LFACLK/2048"]
    DIV2048,
    #[doc = "LFACLKLETIMER0 = LFACLK/4096"]
    DIV4096,
    #[doc = "LFACLKLETIMER0 = LFACLK/8192"]
    DIV8192,
    #[doc = "LFACLKLETIMER0 = LFACLK/16384"]
    DIV16384,
    #[doc = "LFACLKLETIMER0 = LFACLK/32768"]
    DIV32768,
}
impl LETIMER0W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            LETIMER0W::DIV1 => 0,
            LETIMER0W::DIV2 => 1,
            LETIMER0W::DIV4 => 2,
            LETIMER0W::DIV8 => 3,
            LETIMER0W::DIV16 => 4,
            LETIMER0W::DIV32 => 5,
            LETIMER0W::DIV64 => 6,
            LETIMER0W::DIV128 => 7,
            LETIMER0W::DIV256 => 8,
            LETIMER0W::DIV512 => 9,
            LETIMER0W::DIV1024 => 10,
            LETIMER0W::DIV2048 => 11,
            LETIMER0W::DIV4096 => 12,
            LETIMER0W::DIV8192 => 13,
            LETIMER0W::DIV16384 => 14,
            LETIMER0W::DIV32768 => 15,
        }
    }
}
#[doc = r" Proxy"]
pub struct _LETIMER0W<'a> {
    w: &'a mut W,
}
impl<'a> _LETIMER0W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: LETIMER0W) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "LFACLKLETIMER0 = LFACLK"]
    #[inline]
    pub fn div1(self) -> &'a mut W {
        self.variant(LETIMER0W::DIV1)
    }
    #[doc = "LFACLKLETIMER0 = LFACLK/2"]
    #[inline]
    pub fn div2(self) -> &'a mut W {
        self.variant(LETIMER0W::DIV2)
    }
    #[doc = "LFACLKLETIMER0 = LFACLK/4"]
    #[inline]
    pub fn div4(self) -> &'a mut W {
        self.variant(LETIMER0W::DIV4)
    }
    #[doc = "LFACLKLETIMER0 = LFACLK/8"]
    #[inline]
    pub fn div8(self) -> &'a mut W {
        self.variant(LETIMER0W::DIV8)
    }
    #[doc = "LFACLKLETIMER0 = LFACLK/16"]
    #[inline]
    pub fn div16(self) -> &'a mut W {
        self.variant(LETIMER0W::DIV16)
    }
    #[doc = "LFACLKLETIMER0 = LFACLK/32"]
    #[inline]
    pub fn div32(self) -> &'a mut W {
        self.variant(LETIMER0W::DIV32)
    }
    #[doc = "LFACLKLETIMER0 = LFACLK/64"]
    #[inline]
    pub fn div64(self) -> &'a mut W {
        self.variant(LETIMER0W::DIV64)
    }
    #[doc = "LFACLKLETIMER0 = LFACLK/128"]
    #[inline]
    pub fn div128(self) -> &'a mut W {
        self.variant(LETIMER0W::DIV128)
    }
    #[doc = "LFACLKLETIMER0 = LFACLK/256"]
    #[inline]
    pub fn div256(self) -> &'a mut W {
        self.variant(LETIMER0W::DIV256)
    }
    #[doc = "LFACLKLETIMER0 = LFACLK/512"]
    #[inline]
    pub fn div512(self) -> &'a mut W {
        self.variant(LETIMER0W::DIV512)
    }
    #[doc = "LFACLKLETIMER0 = LFACLK/1024"]
    #[inline]
    pub fn div1024(self) -> &'a mut W {
        self.variant(LETIMER0W::DIV1024)
    }
    #[doc = "LFACLKLETIMER0 = LFACLK/2048"]
    #[inline]
    pub fn div2048(self) -> &'a mut W {
        self.variant(LETIMER0W::DIV2048)
    }
    #[doc = "LFACLKLETIMER0 = LFACLK/4096"]
    #[inline]
    pub fn div4096(self) -> &'a mut W {
        self.variant(LETIMER0W::DIV4096)
    }
    #[doc = "LFACLKLETIMER0 = LFACLK/8192"]
    #[inline]
    pub fn div8192(self) -> &'a mut W {
        self.variant(LETIMER0W::DIV8192)
    }
    #[doc = "LFACLKLETIMER0 = LFACLK/16384"]
    #[inline]
    pub fn div16384(self) -> &'a mut W {
        self.variant(LETIMER0W::DIV16384)
    }
    #[doc = "LFACLKLETIMER0 = LFACLK/32768"]
    #[inline]
    pub fn div32768(self) -> &'a mut W {
        self.variant(LETIMER0W::DIV32768)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 0x0f;
        const OFFSET: u8 = 8;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `LCD`"]
pub enum LCDW {
    #[doc = "LFACLKLCD = LFACLK/16"]
    DIV16,
    #[doc = "LFACLKLCD = LFACLK/32"]
    DIV32,
    #[doc = "LFACLKLCD = LFACLK/64"]
    DIV64,
    #[doc = "LFACLKLCD = LFACLK/128"]
    DIV128,
}
impl LCDW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            LCDW::DIV16 => 0,
            LCDW::DIV32 => 1,
            LCDW::DIV64 => 2,
            LCDW::DIV128 => 3,
        }
    }
}
#[doc = r" Proxy"]
pub struct _LCDW<'a> {
    w: &'a mut W,
}
impl<'a> _LCDW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: LCDW) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "LFACLKLCD = LFACLK/16"]
    #[inline]
    pub fn div16(self) -> &'a mut W {
        self.variant(LCDW::DIV16)
    }
    #[doc = "LFACLKLCD = LFACLK/32"]
    #[inline]
    pub fn div32(self) -> &'a mut W {
        self.variant(LCDW::DIV32)
    }
    #[doc = "LFACLKLCD = LFACLK/64"]
    #[inline]
    pub fn div64(self) -> &'a mut W {
        self.variant(LCDW::DIV64)
    }
    #[doc = "LFACLKLCD = LFACLK/128"]
    #[inline]
    pub fn div128(self) -> &'a mut W {
        self.variant(LCDW::DIV128)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 0x03;
        const OFFSET: u8 = 12;
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
    #[doc = "Bits 0:1 - Low Energy Sensor Interface Prescaler"]
    #[inline]
    pub fn lesense(&self) -> LESENSER {
        LESENSER::_from({
            const MASK: u8 = 0x03;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 4:7 - Real-Time Counter Prescaler"]
    #[inline]
    pub fn rtc(&self) -> RTCR {
        RTCR::_from({
            const MASK: u8 = 0x0f;
            const OFFSET: u8 = 4;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 8:11 - Low Energy Timer 0 Prescaler"]
    #[inline]
    pub fn letimer0(&self) -> LETIMER0R {
        LETIMER0R::_from({
            const MASK: u8 = 0x0f;
            const OFFSET: u8 = 8;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 12:13 - Liquid Crystal Display Controller Prescaler"]
    #[inline]
    pub fn lcd(&self) -> LCDR {
        LCDR::_from({
            const MASK: u8 = 0x03;
            const OFFSET: u8 = 12;
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
    #[doc = "Bits 0:1 - Low Energy Sensor Interface Prescaler"]
    #[inline]
    pub fn lesense(&mut self) -> _LESENSEW {
        _LESENSEW { w: self }
    }
    #[doc = "Bits 4:7 - Real-Time Counter Prescaler"]
    #[inline]
    pub fn rtc(&mut self) -> _RTCW {
        _RTCW { w: self }
    }
    #[doc = "Bits 8:11 - Low Energy Timer 0 Prescaler"]
    #[inline]
    pub fn letimer0(&mut self) -> _LETIMER0W {
        _LETIMER0W { w: self }
    }
    #[doc = "Bits 12:13 - Liquid Crystal Display Controller Prescaler"]
    #[inline]
    pub fn lcd(&mut self) -> _LCDW {
        _LCDW { w: self }
    }
}

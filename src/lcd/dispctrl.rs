#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::DISPCTRL {
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
#[doc = "Possible values of the field `MUX`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MUXR {
    #[doc = "Static"]
    STATIC,
    #[doc = "Duplex"]
    DUPLEX,
    #[doc = "Triplex"]
    TRIPLEX,
    #[doc = "Quadruplex"]
    QUADRUPLEX,
}
impl MUXR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            MUXR::STATIC => 0,
            MUXR::DUPLEX => 0x01,
            MUXR::TRIPLEX => 0x02,
            MUXR::QUADRUPLEX => 0x03,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> MUXR {
        match value {
            0 => MUXR::STATIC,
            1 => MUXR::DUPLEX,
            2 => MUXR::TRIPLEX,
            3 => MUXR::QUADRUPLEX,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `STATIC`"]
    #[inline]
    pub fn is_static_(&self) -> bool {
        *self == MUXR::STATIC
    }
    #[doc = "Checks if the value of the field is `DUPLEX`"]
    #[inline]
    pub fn is_duplex(&self) -> bool {
        *self == MUXR::DUPLEX
    }
    #[doc = "Checks if the value of the field is `TRIPLEX`"]
    #[inline]
    pub fn is_triplex(&self) -> bool {
        *self == MUXR::TRIPLEX
    }
    #[doc = "Checks if the value of the field is `QUADRUPLEX`"]
    #[inline]
    pub fn is_quadruplex(&self) -> bool {
        *self == MUXR::QUADRUPLEX
    }
}
#[doc = "Possible values of the field `BIAS`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BIASR {
    #[doc = "Static"]
    STATIC,
    #[doc = "1/2 Bias"]
    ONEHALF,
    #[doc = "1/3 Bias"]
    ONETHIRD,
    #[doc = "1/4 Bias"]
    ONEFOURTH,
}
impl BIASR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            BIASR::STATIC => 0,
            BIASR::ONEHALF => 0x01,
            BIASR::ONETHIRD => 0x02,
            BIASR::ONEFOURTH => 0x03,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> BIASR {
        match value {
            0 => BIASR::STATIC,
            1 => BIASR::ONEHALF,
            2 => BIASR::ONETHIRD,
            3 => BIASR::ONEFOURTH,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `STATIC`"]
    #[inline]
    pub fn is_static_(&self) -> bool {
        *self == BIASR::STATIC
    }
    #[doc = "Checks if the value of the field is `ONEHALF`"]
    #[inline]
    pub fn is_onehalf(&self) -> bool {
        *self == BIASR::ONEHALF
    }
    #[doc = "Checks if the value of the field is `ONETHIRD`"]
    #[inline]
    pub fn is_onethird(&self) -> bool {
        *self == BIASR::ONETHIRD
    }
    #[doc = "Checks if the value of the field is `ONEFOURTH`"]
    #[inline]
    pub fn is_onefourth(&self) -> bool {
        *self == BIASR::ONEFOURTH
    }
}
#[doc = r" Value of the field"]
pub struct WAVER {
    bits: bool,
}
impl WAVER {
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
#[doc = "Possible values of the field `CONLEV`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CONLEVR {
    #[doc = "Minimum contrast"]
    MIN,
    #[doc = "Maximum contrast"]
    MAX,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl CONLEVR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            CONLEVR::MIN => 0,
            CONLEVR::MAX => 0x1f,
            CONLEVR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> CONLEVR {
        match value {
            0 => CONLEVR::MIN,
            31 => CONLEVR::MAX,
            i => CONLEVR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `MIN`"]
    #[inline]
    pub fn is_min(&self) -> bool {
        *self == CONLEVR::MIN
    }
    #[doc = "Checks if the value of the field is `MAX`"]
    #[inline]
    pub fn is_max(&self) -> bool {
        *self == CONLEVR::MAX
    }
}
#[doc = r" Value of the field"]
pub struct CONCONFR {
    bits: bool,
}
impl CONCONFR {
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
pub struct VLCDSELR {
    bits: bool,
}
impl VLCDSELR {
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
#[doc = "Possible values of the field `VBLEV`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum VBLEVR {
    #[doc = "Minimum boost level"]
    LEVEL0,
    #[doc = "\"\""]
    LEVEL1,
    #[doc = "\"\""]
    LEVEL2,
    #[doc = "\"\""]
    LEVEL3,
    #[doc = "\"\""]
    LEVEL4,
    #[doc = "\"\""]
    LEVEL5,
    #[doc = "\"\""]
    LEVEL6,
    #[doc = "Maximum boost level"]
    LEVEL7,
}
impl VBLEVR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            VBLEVR::LEVEL0 => 0,
            VBLEVR::LEVEL1 => 0x01,
            VBLEVR::LEVEL2 => 0x02,
            VBLEVR::LEVEL3 => 0x03,
            VBLEVR::LEVEL4 => 0x04,
            VBLEVR::LEVEL5 => 0x05,
            VBLEVR::LEVEL6 => 0x06,
            VBLEVR::LEVEL7 => 0x07,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> VBLEVR {
        match value {
            0 => VBLEVR::LEVEL0,
            1 => VBLEVR::LEVEL1,
            2 => VBLEVR::LEVEL2,
            3 => VBLEVR::LEVEL3,
            4 => VBLEVR::LEVEL4,
            5 => VBLEVR::LEVEL5,
            6 => VBLEVR::LEVEL6,
            7 => VBLEVR::LEVEL7,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `LEVEL0`"]
    #[inline]
    pub fn is_level0(&self) -> bool {
        *self == VBLEVR::LEVEL0
    }
    #[doc = "Checks if the value of the field is `LEVEL1`"]
    #[inline]
    pub fn is_level1(&self) -> bool {
        *self == VBLEVR::LEVEL1
    }
    #[doc = "Checks if the value of the field is `LEVEL2`"]
    #[inline]
    pub fn is_level2(&self) -> bool {
        *self == VBLEVR::LEVEL2
    }
    #[doc = "Checks if the value of the field is `LEVEL3`"]
    #[inline]
    pub fn is_level3(&self) -> bool {
        *self == VBLEVR::LEVEL3
    }
    #[doc = "Checks if the value of the field is `LEVEL4`"]
    #[inline]
    pub fn is_level4(&self) -> bool {
        *self == VBLEVR::LEVEL4
    }
    #[doc = "Checks if the value of the field is `LEVEL5`"]
    #[inline]
    pub fn is_level5(&self) -> bool {
        *self == VBLEVR::LEVEL5
    }
    #[doc = "Checks if the value of the field is `LEVEL6`"]
    #[inline]
    pub fn is_level6(&self) -> bool {
        *self == VBLEVR::LEVEL6
    }
    #[doc = "Checks if the value of the field is `LEVEL7`"]
    #[inline]
    pub fn is_level7(&self) -> bool {
        *self == VBLEVR::LEVEL7
    }
}
#[doc = r" Value of the field"]
pub struct MUXER {
    bits: bool,
}
impl MUXER {
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
#[doc = "Values that can be written to the field `MUX`"]
pub enum MUXW {
    #[doc = "Static"]
    STATIC,
    #[doc = "Duplex"]
    DUPLEX,
    #[doc = "Triplex"]
    TRIPLEX,
    #[doc = "Quadruplex"]
    QUADRUPLEX,
}
impl MUXW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            MUXW::STATIC => 0,
            MUXW::DUPLEX => 1,
            MUXW::TRIPLEX => 2,
            MUXW::QUADRUPLEX => 3,
        }
    }
}
#[doc = r" Proxy"]
pub struct _MUXW<'a> {
    w: &'a mut W,
}
impl<'a> _MUXW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: MUXW) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "Static"]
    #[inline]
    pub fn static_(self) -> &'a mut W {
        self.variant(MUXW::STATIC)
    }
    #[doc = "Duplex"]
    #[inline]
    pub fn duplex(self) -> &'a mut W {
        self.variant(MUXW::DUPLEX)
    }
    #[doc = "Triplex"]
    #[inline]
    pub fn triplex(self) -> &'a mut W {
        self.variant(MUXW::TRIPLEX)
    }
    #[doc = "Quadruplex"]
    #[inline]
    pub fn quadruplex(self) -> &'a mut W {
        self.variant(MUXW::QUADRUPLEX)
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
#[doc = "Values that can be written to the field `BIAS`"]
pub enum BIASW {
    #[doc = "Static"]
    STATIC,
    #[doc = "1/2 Bias"]
    ONEHALF,
    #[doc = "1/3 Bias"]
    ONETHIRD,
    #[doc = "1/4 Bias"]
    ONEFOURTH,
}
impl BIASW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            BIASW::STATIC => 0,
            BIASW::ONEHALF => 1,
            BIASW::ONETHIRD => 2,
            BIASW::ONEFOURTH => 3,
        }
    }
}
#[doc = r" Proxy"]
pub struct _BIASW<'a> {
    w: &'a mut W,
}
impl<'a> _BIASW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: BIASW) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "Static"]
    #[inline]
    pub fn static_(self) -> &'a mut W {
        self.variant(BIASW::STATIC)
    }
    #[doc = "1/2 Bias"]
    #[inline]
    pub fn onehalf(self) -> &'a mut W {
        self.variant(BIASW::ONEHALF)
    }
    #[doc = "1/3 Bias"]
    #[inline]
    pub fn onethird(self) -> &'a mut W {
        self.variant(BIASW::ONETHIRD)
    }
    #[doc = "1/4 Bias"]
    #[inline]
    pub fn onefourth(self) -> &'a mut W {
        self.variant(BIASW::ONEFOURTH)
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
#[doc = r" Proxy"]
pub struct _WAVEW<'a> {
    w: &'a mut W,
}
impl<'a> _WAVEW<'a> {
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
#[doc = "Values that can be written to the field `CONLEV`"]
pub enum CONLEVW {
    #[doc = "Minimum contrast"]
    MIN,
    #[doc = "Maximum contrast"]
    MAX,
}
impl CONLEVW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            CONLEVW::MIN => 0,
            CONLEVW::MAX => 31,
        }
    }
}
#[doc = r" Proxy"]
pub struct _CONLEVW<'a> {
    w: &'a mut W,
}
impl<'a> _CONLEVW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: CONLEVW) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "Minimum contrast"]
    #[inline]
    pub fn min(self) -> &'a mut W {
        self.variant(CONLEVW::MIN)
    }
    #[doc = "Maximum contrast"]
    #[inline]
    pub fn max(self) -> &'a mut W {
        self.variant(CONLEVW::MAX)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 0x1f;
        const OFFSET: u8 = 8;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _CONCONFW<'a> {
    w: &'a mut W,
}
impl<'a> _CONCONFW<'a> {
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
pub struct _VLCDSELW<'a> {
    w: &'a mut W,
}
impl<'a> _VLCDSELW<'a> {
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
#[doc = "Values that can be written to the field `VBLEV`"]
pub enum VBLEVW {
    #[doc = "Minimum boost level"]
    LEVEL0,
    #[doc = "\"\""]
    LEVEL1,
    #[doc = "\"\""]
    LEVEL2,
    #[doc = "\"\""]
    LEVEL3,
    #[doc = "\"\""]
    LEVEL4,
    #[doc = "\"\""]
    LEVEL5,
    #[doc = "\"\""]
    LEVEL6,
    #[doc = "Maximum boost level"]
    LEVEL7,
}
impl VBLEVW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            VBLEVW::LEVEL0 => 0,
            VBLEVW::LEVEL1 => 1,
            VBLEVW::LEVEL2 => 2,
            VBLEVW::LEVEL3 => 3,
            VBLEVW::LEVEL4 => 4,
            VBLEVW::LEVEL5 => 5,
            VBLEVW::LEVEL6 => 6,
            VBLEVW::LEVEL7 => 7,
        }
    }
}
#[doc = r" Proxy"]
pub struct _VBLEVW<'a> {
    w: &'a mut W,
}
impl<'a> _VBLEVW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: VBLEVW) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "Minimum boost level"]
    #[inline]
    pub fn level0(self) -> &'a mut W {
        self.variant(VBLEVW::LEVEL0)
    }
    #[doc = "\"\""]
    #[inline]
    pub fn level1(self) -> &'a mut W {
        self.variant(VBLEVW::LEVEL1)
    }
    #[doc = "\"\""]
    #[inline]
    pub fn level2(self) -> &'a mut W {
        self.variant(VBLEVW::LEVEL2)
    }
    #[doc = "\"\""]
    #[inline]
    pub fn level3(self) -> &'a mut W {
        self.variant(VBLEVW::LEVEL3)
    }
    #[doc = "\"\""]
    #[inline]
    pub fn level4(self) -> &'a mut W {
        self.variant(VBLEVW::LEVEL4)
    }
    #[doc = "\"\""]
    #[inline]
    pub fn level5(self) -> &'a mut W {
        self.variant(VBLEVW::LEVEL5)
    }
    #[doc = "\"\""]
    #[inline]
    pub fn level6(self) -> &'a mut W {
        self.variant(VBLEVW::LEVEL6)
    }
    #[doc = "Maximum boost level"]
    #[inline]
    pub fn level7(self) -> &'a mut W {
        self.variant(VBLEVW::LEVEL7)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 0x07;
        const OFFSET: u8 = 18;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _MUXEW<'a> {
    w: &'a mut W,
}
impl<'a> _MUXEW<'a> {
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
impl R {
    #[doc = r" Value of the register as raw bits"]
    #[inline]
    pub fn bits(&self) -> u32 {
        self.bits
    }
    #[doc = "Bits 0:1 - Mux Configuration"]
    #[inline]
    pub fn mux(&self) -> MUXR {
        MUXR::_from({
            const MASK: u8 = 0x03;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 2:3 - Bias Configuration"]
    #[inline]
    pub fn bias(&self) -> BIASR {
        BIASR::_from({
            const MASK: u8 = 0x03;
            const OFFSET: u8 = 2;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bit 4 - Waveform Selection"]
    #[inline]
    pub fn wave(&self) -> WAVER {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 4;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        WAVER { bits }
    }
    #[doc = "Bits 8:12 - Contrast Level"]
    #[inline]
    pub fn conlev(&self) -> CONLEVR {
        CONLEVR::_from({
            const MASK: u8 = 0x1f;
            const OFFSET: u8 = 8;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bit 15 - Contrast Configuration"]
    #[inline]
    pub fn conconf(&self) -> CONCONFR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 15;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        CONCONFR { bits }
    }
    #[doc = "Bit 16 - VLCD Selection"]
    #[inline]
    pub fn vlcdsel(&self) -> VLCDSELR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 16;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        VLCDSELR { bits }
    }
    #[doc = "Bits 18:20 - Voltage Boost Level"]
    #[inline]
    pub fn vblev(&self) -> VBLEVR {
        VBLEVR::_from({
            const MASK: u8 = 0x07;
            const OFFSET: u8 = 18;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bit 22 - Extended Mux Configuration"]
    #[inline]
    pub fn muxe(&self) -> MUXER {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 22;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        MUXER { bits }
    }
}
impl W {
    #[doc = r" Reset value of the register"]
    #[inline]
    pub fn reset_value() -> W {
        W { bits: 0x000c_1f00 }
    }
    #[doc = r" Writes raw bits to the register"]
    #[inline]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
    #[doc = "Bits 0:1 - Mux Configuration"]
    #[inline]
    pub fn mux(&mut self) -> _MUXW {
        _MUXW { w: self }
    }
    #[doc = "Bits 2:3 - Bias Configuration"]
    #[inline]
    pub fn bias(&mut self) -> _BIASW {
        _BIASW { w: self }
    }
    #[doc = "Bit 4 - Waveform Selection"]
    #[inline]
    pub fn wave(&mut self) -> _WAVEW {
        _WAVEW { w: self }
    }
    #[doc = "Bits 8:12 - Contrast Level"]
    #[inline]
    pub fn conlev(&mut self) -> _CONLEVW {
        _CONLEVW { w: self }
    }
    #[doc = "Bit 15 - Contrast Configuration"]
    #[inline]
    pub fn conconf(&mut self) -> _CONCONFW {
        _CONCONFW { w: self }
    }
    #[doc = "Bit 16 - VLCD Selection"]
    #[inline]
    pub fn vlcdsel(&mut self) -> _VLCDSELW {
        _VLCDSELW { w: self }
    }
    #[doc = "Bits 18:20 - Voltage Boost Level"]
    #[inline]
    pub fn vblev(&mut self) -> _VBLEVW {
        _VBLEVW { w: self }
    }
    #[doc = "Bit 22 - Extended Mux Configuration"]
    #[inline]
    pub fn muxe(&mut self) -> _MUXEW {
        _MUXEW { w: self }
    }
}

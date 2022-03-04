#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::TFTCTRL {
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
#[doc = "Possible values of the field `DD`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DDR {
    #[doc = "Direct Drive is disabled."]
    DISABLED,
    #[doc = "Direct Drive from internal memory enabled and started."]
    INTERNAL,
    #[doc = "Direct Drive from external memory enabled and started."]
    EXTERNAL,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl DDR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            DDR::DISABLED => 0,
            DDR::INTERNAL => 0x01,
            DDR::EXTERNAL => 0x02,
            DDR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> DDR {
        match value {
            0 => DDR::DISABLED,
            1 => DDR::INTERNAL,
            2 => DDR::EXTERNAL,
            i => DDR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline]
    pub fn is_disabled(&self) -> bool {
        *self == DDR::DISABLED
    }
    #[doc = "Checks if the value of the field is `INTERNAL`"]
    #[inline]
    pub fn is_internal(&self) -> bool {
        *self == DDR::INTERNAL
    }
    #[doc = "Checks if the value of the field is `EXTERNAL`"]
    #[inline]
    pub fn is_external(&self) -> bool {
        *self == DDR::EXTERNAL
    }
}
#[doc = "Possible values of the field `MASKBLEND`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MASKBLENDR {
    #[doc = "Masking and Blending are disabled."]
    DISABLED,
    #[doc = "Internal Masking is enabled."]
    IMASK,
    #[doc = "Internal Alpha Blending is enabled."]
    IALPHA,
    #[doc = "Internal Masking and Alpha Blending are enabled."]
    IMASKIALPHA,
    #[doc = "External Masking is enabled."]
    EMASK,
    #[doc = "External Alpha Blending is enabled."]
    EALPHA,
    #[doc = "External Masking and Alpha Blending are enabled."]
    EMASKEALPHA,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl MASKBLENDR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            MASKBLENDR::DISABLED => 0,
            MASKBLENDR::IMASK => 0x01,
            MASKBLENDR::IALPHA => 0x02,
            MASKBLENDR::IMASKIALPHA => 0x03,
            MASKBLENDR::EMASK => 0x05,
            MASKBLENDR::EALPHA => 0x06,
            MASKBLENDR::EMASKEALPHA => 0x07,
            MASKBLENDR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> MASKBLENDR {
        match value {
            0 => MASKBLENDR::DISABLED,
            1 => MASKBLENDR::IMASK,
            2 => MASKBLENDR::IALPHA,
            3 => MASKBLENDR::IMASKIALPHA,
            5 => MASKBLENDR::EMASK,
            6 => MASKBLENDR::EALPHA,
            7 => MASKBLENDR::EMASKEALPHA,
            i => MASKBLENDR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline]
    pub fn is_disabled(&self) -> bool {
        *self == MASKBLENDR::DISABLED
    }
    #[doc = "Checks if the value of the field is `IMASK`"]
    #[inline]
    pub fn is_imask(&self) -> bool {
        *self == MASKBLENDR::IMASK
    }
    #[doc = "Checks if the value of the field is `IALPHA`"]
    #[inline]
    pub fn is_ialpha(&self) -> bool {
        *self == MASKBLENDR::IALPHA
    }
    #[doc = "Checks if the value of the field is `IMASKIALPHA`"]
    #[inline]
    pub fn is_imaskialpha(&self) -> bool {
        *self == MASKBLENDR::IMASKIALPHA
    }
    #[doc = "Checks if the value of the field is `EMASK`"]
    #[inline]
    pub fn is_emask(&self) -> bool {
        *self == MASKBLENDR::EMASK
    }
    #[doc = "Checks if the value of the field is `EALPHA`"]
    #[inline]
    pub fn is_ealpha(&self) -> bool {
        *self == MASKBLENDR::EALPHA
    }
    #[doc = "Checks if the value of the field is `EMASKEALPHA`"]
    #[inline]
    pub fn is_emaskealpha(&self) -> bool {
        *self == MASKBLENDR::EMASKEALPHA
    }
}
#[doc = r" Value of the field"]
pub struct SHIFTDCLKENR {
    bits: bool,
}
impl SHIFTDCLKENR {
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
pub struct FBCTRIGR {
    bits: bool,
}
impl FBCTRIGR {
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
#[doc = "Possible values of the field `INTERLEAVE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum INTERLEAVER {
    #[doc = "Allow unlimited interleaved EBI accesses per EBI_DCLK period. This can cause jitter on the EBI_DCLK"]
    UNLIMITED,
    #[doc = "Allow 1 interleaved EBI access per EBI_DCLK period."]
    ONEPERDCLK,
    #[doc = "Only allow EBI accesses during TFT porches."]
    PORCH,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl INTERLEAVER {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            INTERLEAVER::UNLIMITED => 0,
            INTERLEAVER::ONEPERDCLK => 0x01,
            INTERLEAVER::PORCH => 0x02,
            INTERLEAVER::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> INTERLEAVER {
        match value {
            0 => INTERLEAVER::UNLIMITED,
            1 => INTERLEAVER::ONEPERDCLK,
            2 => INTERLEAVER::PORCH,
            i => INTERLEAVER::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `UNLIMITED`"]
    #[inline]
    pub fn is_unlimited(&self) -> bool {
        *self == INTERLEAVER::UNLIMITED
    }
    #[doc = "Checks if the value of the field is `ONEPERDCLK`"]
    #[inline]
    pub fn is_oneperdclk(&self) -> bool {
        *self == INTERLEAVER::ONEPERDCLK
    }
    #[doc = "Checks if the value of the field is `PORCH`"]
    #[inline]
    pub fn is_porch(&self) -> bool {
        *self == INTERLEAVER::PORCH
    }
}
#[doc = r" Value of the field"]
pub struct COLOR1SRCR {
    bits: bool,
}
impl COLOR1SRCR {
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
pub struct WIDTHR {
    bits: bool,
}
impl WIDTHR {
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
    #[doc = "Memory bank 0 is used for Direct Drive, Masking, and Alpha Blending."]
    BANK0,
    #[doc = "Memory bank 1 is used for Direct Drive, Masking, and Alpha Blending."]
    BANK1,
    #[doc = "Memory bank 2 is used for Direct Drive, Masking, and Alpha Blending."]
    BANK2,
    #[doc = "Memory bank 3 is used for Direct Drive, Masking, and Alpha Blending."]
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
#[doc = r" Value of the field"]
pub struct RGBMODER {
    bits: bool,
}
impl RGBMODER {
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
#[doc = "Values that can be written to the field `DD`"]
pub enum DDW {
    #[doc = "Direct Drive is disabled."]
    DISABLED,
    #[doc = "Direct Drive from internal memory enabled and started."]
    INTERNAL,
    #[doc = "Direct Drive from external memory enabled and started."]
    EXTERNAL,
}
impl DDW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            DDW::DISABLED => 0,
            DDW::INTERNAL => 1,
            DDW::EXTERNAL => 2,
        }
    }
}
#[doc = r" Proxy"]
pub struct _DDW<'a> {
    w: &'a mut W,
}
impl<'a> _DDW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: DDW) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "Direct Drive is disabled."]
    #[inline]
    pub fn disabled(self) -> &'a mut W {
        self.variant(DDW::DISABLED)
    }
    #[doc = "Direct Drive from internal memory enabled and started."]
    #[inline]
    pub fn internal(self) -> &'a mut W {
        self.variant(DDW::INTERNAL)
    }
    #[doc = "Direct Drive from external memory enabled and started."]
    #[inline]
    pub fn external(self) -> &'a mut W {
        self.variant(DDW::EXTERNAL)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 0x03;
        const OFFSET: u8 = 0;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `MASKBLEND`"]
pub enum MASKBLENDW {
    #[doc = "Masking and Blending are disabled."]
    DISABLED,
    #[doc = "Internal Masking is enabled."]
    IMASK,
    #[doc = "Internal Alpha Blending is enabled."]
    IALPHA,
    #[doc = "Internal Masking and Alpha Blending are enabled."]
    IMASKIALPHA,
    #[doc = "External Masking is enabled."]
    EMASK,
    #[doc = "External Alpha Blending is enabled."]
    EALPHA,
    #[doc = "External Masking and Alpha Blending are enabled."]
    EMASKEALPHA,
}
impl MASKBLENDW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            MASKBLENDW::DISABLED => 0,
            MASKBLENDW::IMASK => 1,
            MASKBLENDW::IALPHA => 2,
            MASKBLENDW::IMASKIALPHA => 3,
            MASKBLENDW::EMASK => 5,
            MASKBLENDW::EALPHA => 6,
            MASKBLENDW::EMASKEALPHA => 7,
        }
    }
}
#[doc = r" Proxy"]
pub struct _MASKBLENDW<'a> {
    w: &'a mut W,
}
impl<'a> _MASKBLENDW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: MASKBLENDW) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "Masking and Blending are disabled."]
    #[inline]
    pub fn disabled(self) -> &'a mut W {
        self.variant(MASKBLENDW::DISABLED)
    }
    #[doc = "Internal Masking is enabled."]
    #[inline]
    pub fn imask(self) -> &'a mut W {
        self.variant(MASKBLENDW::IMASK)
    }
    #[doc = "Internal Alpha Blending is enabled."]
    #[inline]
    pub fn ialpha(self) -> &'a mut W {
        self.variant(MASKBLENDW::IALPHA)
    }
    #[doc = "Internal Masking and Alpha Blending are enabled."]
    #[inline]
    pub fn imaskialpha(self) -> &'a mut W {
        self.variant(MASKBLENDW::IMASKIALPHA)
    }
    #[doc = "External Masking is enabled."]
    #[inline]
    pub fn emask(self) -> &'a mut W {
        self.variant(MASKBLENDW::EMASK)
    }
    #[doc = "External Alpha Blending is enabled."]
    #[inline]
    pub fn ealpha(self) -> &'a mut W {
        self.variant(MASKBLENDW::EALPHA)
    }
    #[doc = "External Masking and Alpha Blending are enabled."]
    #[inline]
    pub fn emaskealpha(self) -> &'a mut W {
        self.variant(MASKBLENDW::EMASKEALPHA)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 0x07;
        const OFFSET: u8 = 2;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _SHIFTDCLKENW<'a> {
    w: &'a mut W,
}
impl<'a> _SHIFTDCLKENW<'a> {
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
pub struct _FBCTRIGW<'a> {
    w: &'a mut W,
}
impl<'a> _FBCTRIGW<'a> {
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
#[doc = "Values that can be written to the field `INTERLEAVE`"]
pub enum INTERLEAVEW {
    #[doc = "Allow unlimited interleaved EBI accesses per EBI_DCLK period. This can cause jitter on the EBI_DCLK"]
    UNLIMITED,
    #[doc = "Allow 1 interleaved EBI access per EBI_DCLK period."]
    ONEPERDCLK,
    #[doc = "Only allow EBI accesses during TFT porches."]
    PORCH,
}
impl INTERLEAVEW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            INTERLEAVEW::UNLIMITED => 0,
            INTERLEAVEW::ONEPERDCLK => 1,
            INTERLEAVEW::PORCH => 2,
        }
    }
}
#[doc = r" Proxy"]
pub struct _INTERLEAVEW<'a> {
    w: &'a mut W,
}
impl<'a> _INTERLEAVEW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: INTERLEAVEW) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "Allow unlimited interleaved EBI accesses per EBI_DCLK period. This can cause jitter on the EBI_DCLK"]
    #[inline]
    pub fn unlimited(self) -> &'a mut W {
        self.variant(INTERLEAVEW::UNLIMITED)
    }
    #[doc = "Allow 1 interleaved EBI access per EBI_DCLK period."]
    #[inline]
    pub fn oneperdclk(self) -> &'a mut W {
        self.variant(INTERLEAVEW::ONEPERDCLK)
    }
    #[doc = "Only allow EBI accesses during TFT porches."]
    #[inline]
    pub fn porch(self) -> &'a mut W {
        self.variant(INTERLEAVEW::PORCH)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 0x03;
        const OFFSET: u8 = 10;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _COLOR1SRCW<'a> {
    w: &'a mut W,
}
impl<'a> _COLOR1SRCW<'a> {
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
pub struct _WIDTHW<'a> {
    w: &'a mut W,
}
impl<'a> _WIDTHW<'a> {
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
#[doc = "Values that can be written to the field `BANKSEL`"]
pub enum BANKSELW {
    #[doc = "Memory bank 0 is used for Direct Drive, Masking, and Alpha Blending."]
    BANK0,
    #[doc = "Memory bank 1 is used for Direct Drive, Masking, and Alpha Blending."]
    BANK1,
    #[doc = "Memory bank 2 is used for Direct Drive, Masking, and Alpha Blending."]
    BANK2,
    #[doc = "Memory bank 3 is used for Direct Drive, Masking, and Alpha Blending."]
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
    #[doc = "Memory bank 0 is used for Direct Drive, Masking, and Alpha Blending."]
    #[inline]
    pub fn bank0(self) -> &'a mut W {
        self.variant(BANKSELW::BANK0)
    }
    #[doc = "Memory bank 1 is used for Direct Drive, Masking, and Alpha Blending."]
    #[inline]
    pub fn bank1(self) -> &'a mut W {
        self.variant(BANKSELW::BANK1)
    }
    #[doc = "Memory bank 2 is used for Direct Drive, Masking, and Alpha Blending."]
    #[inline]
    pub fn bank2(self) -> &'a mut W {
        self.variant(BANKSELW::BANK2)
    }
    #[doc = "Memory bank 3 is used for Direct Drive, Masking, and Alpha Blending."]
    #[inline]
    pub fn bank3(self) -> &'a mut W {
        self.variant(BANKSELW::BANK3)
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
#[doc = r" Proxy"]
pub struct _RGBMODEW<'a> {
    w: &'a mut W,
}
impl<'a> _RGBMODEW<'a> {
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
impl R {
    #[doc = r" Value of the register as raw bits"]
    #[inline]
    pub fn bits(&self) -> u32 {
        self.bits
    }
    #[doc = "Bits 0:1 - TFT Direct Drive Mode"]
    #[inline]
    pub fn dd(&self) -> DDR {
        DDR::_from({
            const MASK: u8 = 0x03;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 2:4 - TFT Mask and Blend Mode"]
    #[inline]
    pub fn maskblend(&self) -> MASKBLENDR {
        MASKBLENDR::_from({
            const MASK: u8 = 0x07;
            const OFFSET: u8 = 2;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bit 8 - TFT EBI_DCLK Shift Enable"]
    #[inline]
    pub fn shiftdclken(&self) -> SHIFTDCLKENR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 8;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        SHIFTDCLKENR { bits }
    }
    #[doc = "Bit 9 - TFT Frame Base Copy Trigger"]
    #[inline]
    pub fn fbctrig(&self) -> FBCTRIGR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 9;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        FBCTRIGR { bits }
    }
    #[doc = "Bits 10:11 - Interleave Mode"]
    #[inline]
    pub fn interleave(&self) -> INTERLEAVER {
        INTERLEAVER::_from({
            const MASK: u8 = 0x03;
            const OFFSET: u8 = 10;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bit 12 - Masking/Alpha Blending Color1 Source"]
    #[inline]
    pub fn color1src(&self) -> COLOR1SRCR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 12;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        COLOR1SRCR { bits }
    }
    #[doc = "Bit 16 - TFT Transaction Width"]
    #[inline]
    pub fn width(&self) -> WIDTHR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 16;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        WIDTHR { bits }
    }
    #[doc = "Bits 20:21 - Graphics Bank"]
    #[inline]
    pub fn banksel(&self) -> BANKSELR {
        BANKSELR::_from({
            const MASK: u8 = 0x03;
            const OFFSET: u8 = 20;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bit 24 - TFT RGB Mode"]
    #[inline]
    pub fn rgbmode(&self) -> RGBMODER {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 24;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        RGBMODER { bits }
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
    #[doc = "Bits 0:1 - TFT Direct Drive Mode"]
    #[inline]
    pub fn dd(&mut self) -> _DDW {
        _DDW { w: self }
    }
    #[doc = "Bits 2:4 - TFT Mask and Blend Mode"]
    #[inline]
    pub fn maskblend(&mut self) -> _MASKBLENDW {
        _MASKBLENDW { w: self }
    }
    #[doc = "Bit 8 - TFT EBI_DCLK Shift Enable"]
    #[inline]
    pub fn shiftdclken(&mut self) -> _SHIFTDCLKENW {
        _SHIFTDCLKENW { w: self }
    }
    #[doc = "Bit 9 - TFT Frame Base Copy Trigger"]
    #[inline]
    pub fn fbctrig(&mut self) -> _FBCTRIGW {
        _FBCTRIGW { w: self }
    }
    #[doc = "Bits 10:11 - Interleave Mode"]
    #[inline]
    pub fn interleave(&mut self) -> _INTERLEAVEW {
        _INTERLEAVEW { w: self }
    }
    #[doc = "Bit 12 - Masking/Alpha Blending Color1 Source"]
    #[inline]
    pub fn color1src(&mut self) -> _COLOR1SRCW {
        _COLOR1SRCW { w: self }
    }
    #[doc = "Bit 16 - TFT Transaction Width"]
    #[inline]
    pub fn width(&mut self) -> _WIDTHW {
        _WIDTHW { w: self }
    }
    #[doc = "Bits 20:21 - Graphics Bank"]
    #[inline]
    pub fn banksel(&mut self) -> _BANKSELW {
        _BANKSELW { w: self }
    }
    #[doc = "Bit 24 - TFT RGB Mode"]
    #[inline]
    pub fn rgbmode(&mut self) -> _RGBMODEW {
        _RGBMODEW { w: self }
    }
}

#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::HCFG {
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
#[doc = "Possible values of the field `FSLSPCLKSEL`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FSLSPCLKSELR {
    #[doc = "Internal PHY clock is running at 48 MHz (undivided)."]
    DIV1,
    #[doc = "Internal PHY clock is running at 6 MHz (48 MHz divided by 8)."]
    DIV8,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl FSLSPCLKSELR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            FSLSPCLKSELR::DIV1 => 0x01,
            FSLSPCLKSELR::DIV8 => 0x02,
            FSLSPCLKSELR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> FSLSPCLKSELR {
        match value {
            1 => FSLSPCLKSELR::DIV1,
            2 => FSLSPCLKSELR::DIV8,
            i => FSLSPCLKSELR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `DIV1`"]
    #[inline]
    pub fn is_div1(&self) -> bool {
        *self == FSLSPCLKSELR::DIV1
    }
    #[doc = "Checks if the value of the field is `DIV8`"]
    #[inline]
    pub fn is_div8(&self) -> bool {
        *self == FSLSPCLKSELR::DIV8
    }
}
#[doc = r" Value of the field"]
pub struct FSLSSUPPR {
    bits: bool,
}
impl FSLSSUPPR {
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
pub struct ENA32KHZSR {
    bits: bool,
}
impl ENA32KHZSR {
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
pub struct RESVALIDR {
    bits: u8,
}
impl RESVALIDR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct MODECHTIMENR {
    bits: bool,
}
impl MODECHTIMENR {
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
#[doc = "Values that can be written to the field `FSLSPCLKSEL`"]
pub enum FSLSPCLKSELW {
    #[doc = "Internal PHY clock is running at 48 MHz (undivided)."]
    DIV1,
    #[doc = "Internal PHY clock is running at 6 MHz (48 MHz divided by 8)."]
    DIV8,
}
impl FSLSPCLKSELW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            FSLSPCLKSELW::DIV1 => 1,
            FSLSPCLKSELW::DIV8 => 2,
        }
    }
}
#[doc = r" Proxy"]
pub struct _FSLSPCLKSELW<'a> {
    w: &'a mut W,
}
impl<'a> _FSLSPCLKSELW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: FSLSPCLKSELW) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "Internal PHY clock is running at 48 MHz (undivided)."]
    #[inline]
    pub fn div1(self) -> &'a mut W {
        self.variant(FSLSPCLKSELW::DIV1)
    }
    #[doc = "Internal PHY clock is running at 6 MHz (48 MHz divided by 8)."]
    #[inline]
    pub fn div8(self) -> &'a mut W {
        self.variant(FSLSPCLKSELW::DIV8)
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
#[doc = r" Proxy"]
pub struct _FSLSSUPPW<'a> {
    w: &'a mut W,
}
impl<'a> _FSLSSUPPW<'a> {
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
pub struct _ENA32KHZSW<'a> {
    w: &'a mut W,
}
impl<'a> _ENA32KHZSW<'a> {
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
#[doc = r" Proxy"]
pub struct _RESVALIDW<'a> {
    w: &'a mut W,
}
impl<'a> _RESVALIDW<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 0xff;
        const OFFSET: u8 = 8;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _MODECHTIMENW<'a> {
    w: &'a mut W,
}
impl<'a> _MODECHTIMENW<'a> {
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
        const OFFSET: u8 = 31;
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
    #[doc = "Bits 0:1 - FS/LS PHY Clock Select"]
    #[inline]
    pub fn fslspclksel(&self) -> FSLSPCLKSELR {
        FSLSPCLKSELR::_from({
            const MASK: u8 = 0x03;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bit 2 - FS- and LS-Only Support"]
    #[inline]
    pub fn fslssupp(&self) -> FSLSSUPPR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 2;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        FSLSSUPPR { bits }
    }
    #[doc = "Bit 7 - Enable 32 KHz Suspend mode"]
    #[inline]
    pub fn ena32khzs(&self) -> ENA32KHZSR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 7;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        ENA32KHZSR { bits }
    }
    #[doc = "Bits 8:15 - Resume Validation Period"]
    #[inline]
    pub fn resvalid(&self) -> RESVALIDR {
        let bits = {
            const MASK: u8 = 0xff;
            const OFFSET: u8 = 8;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        RESVALIDR { bits }
    }
    #[doc = "Bit 31 - Mode Change Time"]
    #[inline]
    pub fn modechtimen(&self) -> MODECHTIMENR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 31;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        MODECHTIMENR { bits }
    }
}
impl W {
    #[doc = r" Reset value of the register"]
    #[inline]
    pub fn reset_value() -> W {
        W { bits: 0x0020_0000 }
    }
    #[doc = r" Writes raw bits to the register"]
    #[inline]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
    #[doc = "Bits 0:1 - FS/LS PHY Clock Select"]
    #[inline]
    pub fn fslspclksel(&mut self) -> _FSLSPCLKSELW {
        _FSLSPCLKSELW { w: self }
    }
    #[doc = "Bit 2 - FS- and LS-Only Support"]
    #[inline]
    pub fn fslssupp(&mut self) -> _FSLSSUPPW {
        _FSLSSUPPW { w: self }
    }
    #[doc = "Bit 7 - Enable 32 KHz Suspend mode"]
    #[inline]
    pub fn ena32khzs(&mut self) -> _ENA32KHZSW {
        _ENA32KHZSW { w: self }
    }
    #[doc = "Bits 8:15 - Resume Validation Period"]
    #[inline]
    pub fn resvalid(&mut self) -> _RESVALIDW {
        _RESVALIDW { w: self }
    }
    #[doc = "Bit 31 - Mode Change Time"]
    #[inline]
    pub fn modechtimen(&mut self) -> _MODECHTIMENW {
        _MODECHTIMENW { w: self }
    }
}

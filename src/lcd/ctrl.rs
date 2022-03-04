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
#[doc = "Possible values of the field `UDCTRL`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum UDCTRLR {
    #[doc = "The data transfer is controlled by SW. Transfer is performed as soon as possible"]
    REGULAR,
    #[doc = "The data transfer is done at the next event triggered by the Frame Counter"]
    FCEVENT,
    #[doc = "The data transfer is done continuously at every LCD frame start"]
    FRAMESTART,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl UDCTRLR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            UDCTRLR::REGULAR => 0,
            UDCTRLR::FCEVENT => 0x01,
            UDCTRLR::FRAMESTART => 0x02,
            UDCTRLR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> UDCTRLR {
        match value {
            0 => UDCTRLR::REGULAR,
            1 => UDCTRLR::FCEVENT,
            2 => UDCTRLR::FRAMESTART,
            i => UDCTRLR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `REGULAR`"]
    #[inline]
    pub fn is_regular(&self) -> bool {
        *self == UDCTRLR::REGULAR
    }
    #[doc = "Checks if the value of the field is `FCEVENT`"]
    #[inline]
    pub fn is_fcevent(&self) -> bool {
        *self == UDCTRLR::FCEVENT
    }
    #[doc = "Checks if the value of the field is `FRAMESTART`"]
    #[inline]
    pub fn is_framestart(&self) -> bool {
        *self == UDCTRLR::FRAMESTART
    }
}
#[doc = r" Value of the field"]
pub struct DSCR {
    bits: bool,
}
impl DSCR {
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
#[doc = "Values that can be written to the field `UDCTRL`"]
pub enum UDCTRLW {
    #[doc = "The data transfer is controlled by SW. Transfer is performed as soon as possible"]
    REGULAR,
    #[doc = "The data transfer is done at the next event triggered by the Frame Counter"]
    FCEVENT,
    #[doc = "The data transfer is done continuously at every LCD frame start"]
    FRAMESTART,
}
impl UDCTRLW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            UDCTRLW::REGULAR => 0,
            UDCTRLW::FCEVENT => 1,
            UDCTRLW::FRAMESTART => 2,
        }
    }
}
#[doc = r" Proxy"]
pub struct _UDCTRLW<'a> {
    w: &'a mut W,
}
impl<'a> _UDCTRLW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: UDCTRLW) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "The data transfer is controlled by SW. Transfer is performed as soon as possible"]
    #[inline]
    pub fn regular(self) -> &'a mut W {
        self.variant(UDCTRLW::REGULAR)
    }
    #[doc = "The data transfer is done at the next event triggered by the Frame Counter"]
    #[inline]
    pub fn fcevent(self) -> &'a mut W {
        self.variant(UDCTRLW::FCEVENT)
    }
    #[doc = "The data transfer is done continuously at every LCD frame start"]
    #[inline]
    pub fn framestart(self) -> &'a mut W {
        self.variant(UDCTRLW::FRAMESTART)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 0x03;
        const OFFSET: u8 = 1;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _DSCW<'a> {
    w: &'a mut W,
}
impl<'a> _DSCW<'a> {
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
        const OFFSET: u8 = 23;
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
    #[doc = "Bit 0 - LCD Enable"]
    #[inline]
    pub fn en(&self) -> ENR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        ENR { bits }
    }
    #[doc = "Bits 1:2 - Update Data Control"]
    #[inline]
    pub fn udctrl(&self) -> UDCTRLR {
        UDCTRLR::_from({
            const MASK: u8 = 0x03;
            const OFFSET: u8 = 1;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bit 23 - Direct Segment Control"]
    #[inline]
    pub fn dsc(&self) -> DSCR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 23;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        DSCR { bits }
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
    #[doc = "Bit 0 - LCD Enable"]
    #[inline]
    pub fn en(&mut self) -> _ENW {
        _ENW { w: self }
    }
    #[doc = "Bits 1:2 - Update Data Control"]
    #[inline]
    pub fn udctrl(&mut self) -> _UDCTRLW {
        _UDCTRLW { w: self }
    }
    #[doc = "Bit 23 - Direct Segment Control"]
    #[inline]
    pub fn dsc(&mut self) -> _DSCW {
        _DSCW { w: self }
    }
}

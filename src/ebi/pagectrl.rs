#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::PAGECTRL {
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
#[doc = "Possible values of the field `PAGELEN`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PAGELENR {
    #[doc = "4 members in a page."]
    MEMBER4,
    #[doc = "8 members in a page."]
    MEMBER8,
    #[doc = "16 members in a page."]
    MEMBER16,
    #[doc = "32 members in a page."]
    MEMBER32,
}
impl PAGELENR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            PAGELENR::MEMBER4 => 0,
            PAGELENR::MEMBER8 => 0x01,
            PAGELENR::MEMBER16 => 0x02,
            PAGELENR::MEMBER32 => 0x03,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> PAGELENR {
        match value {
            0 => PAGELENR::MEMBER4,
            1 => PAGELENR::MEMBER8,
            2 => PAGELENR::MEMBER16,
            3 => PAGELENR::MEMBER32,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `MEMBER4`"]
    #[inline]
    pub fn is_member4(&self) -> bool {
        *self == PAGELENR::MEMBER4
    }
    #[doc = "Checks if the value of the field is `MEMBER8`"]
    #[inline]
    pub fn is_member8(&self) -> bool {
        *self == PAGELENR::MEMBER8
    }
    #[doc = "Checks if the value of the field is `MEMBER16`"]
    #[inline]
    pub fn is_member16(&self) -> bool {
        *self == PAGELENR::MEMBER16
    }
    #[doc = "Checks if the value of the field is `MEMBER32`"]
    #[inline]
    pub fn is_member32(&self) -> bool {
        *self == PAGELENR::MEMBER32
    }
}
#[doc = r" Value of the field"]
pub struct INCHITR {
    bits: bool,
}
impl INCHITR {
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
pub struct RDPAR {
    bits: u8,
}
impl RDPAR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct KEEPOPENR {
    bits: u8,
}
impl KEEPOPENR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = "Values that can be written to the field `PAGELEN`"]
pub enum PAGELENW {
    #[doc = "4 members in a page."]
    MEMBER4,
    #[doc = "8 members in a page."]
    MEMBER8,
    #[doc = "16 members in a page."]
    MEMBER16,
    #[doc = "32 members in a page."]
    MEMBER32,
}
impl PAGELENW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            PAGELENW::MEMBER4 => 0,
            PAGELENW::MEMBER8 => 1,
            PAGELENW::MEMBER16 => 2,
            PAGELENW::MEMBER32 => 3,
        }
    }
}
#[doc = r" Proxy"]
pub struct _PAGELENW<'a> {
    w: &'a mut W,
}
impl<'a> _PAGELENW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: PAGELENW) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "4 members in a page."]
    #[inline]
    pub fn member4(self) -> &'a mut W {
        self.variant(PAGELENW::MEMBER4)
    }
    #[doc = "8 members in a page."]
    #[inline]
    pub fn member8(self) -> &'a mut W {
        self.variant(PAGELENW::MEMBER8)
    }
    #[doc = "16 members in a page."]
    #[inline]
    pub fn member16(self) -> &'a mut W {
        self.variant(PAGELENW::MEMBER16)
    }
    #[doc = "32 members in a page."]
    #[inline]
    pub fn member32(self) -> &'a mut W {
        self.variant(PAGELENW::MEMBER32)
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
#[doc = r" Proxy"]
pub struct _INCHITW<'a> {
    w: &'a mut W,
}
impl<'a> _INCHITW<'a> {
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
#[doc = r" Proxy"]
pub struct _RDPAW<'a> {
    w: &'a mut W,
}
impl<'a> _RDPAW<'a> {
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
pub struct _KEEPOPENW<'a> {
    w: &'a mut W,
}
impl<'a> _KEEPOPENW<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 0x7f;
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
    #[doc = "Bits 0:1 - Page Length"]
    #[inline]
    pub fn pagelen(&self) -> PAGELENR {
        PAGELENR::_from({
            const MASK: u8 = 0x03;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bit 4 - Intrapage hit only on incremental addresses"]
    #[inline]
    pub fn inchit(&self) -> INCHITR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 4;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        INCHITR { bits }
    }
    #[doc = "Bits 8:10 - Page Read Access Time"]
    #[inline]
    pub fn rdpa(&self) -> RDPAR {
        let bits = {
            const MASK: u8 = 0x07;
            const OFFSET: u8 = 8;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        RDPAR { bits }
    }
    #[doc = "Bits 20:26 - Maximum Page Open Time."]
    #[inline]
    pub fn keepopen(&self) -> KEEPOPENR {
        let bits = {
            const MASK: u8 = 0x7f;
            const OFFSET: u8 = 20;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        KEEPOPENR { bits }
    }
}
impl W {
    #[doc = r" Reset value of the register"]
    #[inline]
    pub fn reset_value() -> W {
        W { bits: 0x0700 }
    }
    #[doc = r" Writes raw bits to the register"]
    #[inline]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
    #[doc = "Bits 0:1 - Page Length"]
    #[inline]
    pub fn pagelen(&mut self) -> _PAGELENW {
        _PAGELENW { w: self }
    }
    #[doc = "Bit 4 - Intrapage hit only on incremental addresses"]
    #[inline]
    pub fn inchit(&mut self) -> _INCHITW {
        _INCHITW { w: self }
    }
    #[doc = "Bits 8:10 - Page Read Access Time"]
    #[inline]
    pub fn rdpa(&mut self) -> _RDPAW {
        _RDPAW { w: self }
    }
    #[doc = "Bits 20:26 - Maximum Page Open Time."]
    #[inline]
    pub fn keepopen(&mut self) -> _KEEPOPENW {
        _KEEPOPENW { w: self }
    }
}

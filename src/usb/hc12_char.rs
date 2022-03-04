#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::HC12_CHAR {
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
pub struct MPSR {
    bits: u16,
}
impl MPSR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u16 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct EPNUMR {
    bits: u8,
}
impl EPNUMR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct EPDIRR {
    bits: bool,
}
impl EPDIRR {
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
pub struct LSPDDEVR {
    bits: bool,
}
impl LSPDDEVR {
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
#[doc = "Possible values of the field `EPTYPE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum EPTYPER {
    #[doc = "Control endpoint."]
    CONTROL,
    #[doc = "Isochronous endpoint."]
    ISO,
    #[doc = "Bulk endpoint."]
    BULK,
    #[doc = "Interrupt endpoint."]
    INT,
}
impl EPTYPER {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            EPTYPER::CONTROL => 0,
            EPTYPER::ISO => 0x01,
            EPTYPER::BULK => 0x02,
            EPTYPER::INT => 0x03,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> EPTYPER {
        match value {
            0 => EPTYPER::CONTROL,
            1 => EPTYPER::ISO,
            2 => EPTYPER::BULK,
            3 => EPTYPER::INT,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `CONTROL`"]
    #[inline]
    pub fn is_control(&self) -> bool {
        *self == EPTYPER::CONTROL
    }
    #[doc = "Checks if the value of the field is `ISO`"]
    #[inline]
    pub fn is_iso(&self) -> bool {
        *self == EPTYPER::ISO
    }
    #[doc = "Checks if the value of the field is `BULK`"]
    #[inline]
    pub fn is_bulk(&self) -> bool {
        *self == EPTYPER::BULK
    }
    #[doc = "Checks if the value of the field is `INT`"]
    #[inline]
    pub fn is_int(&self) -> bool {
        *self == EPTYPER::INT
    }
}
#[doc = r" Value of the field"]
pub struct MCR {
    bits: u8,
}
impl MCR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct DEVADDRR {
    bits: u8,
}
impl DEVADDRR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct ODDFRMR {
    bits: bool,
}
impl ODDFRMR {
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
pub struct CHDISR {
    bits: bool,
}
impl CHDISR {
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
pub struct CHENAR {
    bits: bool,
}
impl CHENAR {
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
pub struct _MPSW<'a> {
    w: &'a mut W,
}
impl<'a> _MPSW<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        const MASK: u16 = 0x07ff;
        const OFFSET: u8 = 0;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _EPNUMW<'a> {
    w: &'a mut W,
}
impl<'a> _EPNUMW<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 0x0f;
        const OFFSET: u8 = 11;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _EPDIRW<'a> {
    w: &'a mut W,
}
impl<'a> _EPDIRW<'a> {
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
pub struct _LSPDDEVW<'a> {
    w: &'a mut W,
}
impl<'a> _LSPDDEVW<'a> {
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
        const OFFSET: u8 = 17;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `EPTYPE`"]
pub enum EPTYPEW {
    #[doc = "Control endpoint."]
    CONTROL,
    #[doc = "Isochronous endpoint."]
    ISO,
    #[doc = "Bulk endpoint."]
    BULK,
    #[doc = "Interrupt endpoint."]
    INT,
}
impl EPTYPEW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            EPTYPEW::CONTROL => 0,
            EPTYPEW::ISO => 1,
            EPTYPEW::BULK => 2,
            EPTYPEW::INT => 3,
        }
    }
}
#[doc = r" Proxy"]
pub struct _EPTYPEW<'a> {
    w: &'a mut W,
}
impl<'a> _EPTYPEW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: EPTYPEW) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "Control endpoint."]
    #[inline]
    pub fn control(self) -> &'a mut W {
        self.variant(EPTYPEW::CONTROL)
    }
    #[doc = "Isochronous endpoint."]
    #[inline]
    pub fn iso(self) -> &'a mut W {
        self.variant(EPTYPEW::ISO)
    }
    #[doc = "Bulk endpoint."]
    #[inline]
    pub fn bulk(self) -> &'a mut W {
        self.variant(EPTYPEW::BULK)
    }
    #[doc = "Interrupt endpoint."]
    #[inline]
    pub fn int(self) -> &'a mut W {
        self.variant(EPTYPEW::INT)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 0x03;
        const OFFSET: u8 = 18;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _MCW<'a> {
    w: &'a mut W,
}
impl<'a> _MCW<'a> {
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
#[doc = r" Proxy"]
pub struct _DEVADDRW<'a> {
    w: &'a mut W,
}
impl<'a> _DEVADDRW<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 0x7f;
        const OFFSET: u8 = 22;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _ODDFRMW<'a> {
    w: &'a mut W,
}
impl<'a> _ODDFRMW<'a> {
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
        const OFFSET: u8 = 29;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _CHDISW<'a> {
    w: &'a mut W,
}
impl<'a> _CHDISW<'a> {
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
        const OFFSET: u8 = 30;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _CHENAW<'a> {
    w: &'a mut W,
}
impl<'a> _CHENAW<'a> {
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
    #[doc = "Bits 0:10 - Maximum Packet Size"]
    #[inline]
    pub fn mps(&self) -> MPSR {
        let bits = {
            const MASK: u16 = 0x07ff;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) as u16
        };
        MPSR { bits }
    }
    #[doc = "Bits 11:14 - Endpoint Number"]
    #[inline]
    pub fn epnum(&self) -> EPNUMR {
        let bits = {
            const MASK: u8 = 0x0f;
            const OFFSET: u8 = 11;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        EPNUMR { bits }
    }
    #[doc = "Bit 15 - Endpoint Direction"]
    #[inline]
    pub fn epdir(&self) -> EPDIRR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 15;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        EPDIRR { bits }
    }
    #[doc = "Bit 17 - Low-Speed Device"]
    #[inline]
    pub fn lspddev(&self) -> LSPDDEVR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 17;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        LSPDDEVR { bits }
    }
    #[doc = "Bits 18:19 - Endpoint Type"]
    #[inline]
    pub fn eptype(&self) -> EPTYPER {
        EPTYPER::_from({
            const MASK: u8 = 0x03;
            const OFFSET: u8 = 18;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 20:21 - Multi Count"]
    #[inline]
    pub fn mc(&self) -> MCR {
        let bits = {
            const MASK: u8 = 0x03;
            const OFFSET: u8 = 20;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        MCR { bits }
    }
    #[doc = "Bits 22:28 - Device Address"]
    #[inline]
    pub fn devaddr(&self) -> DEVADDRR {
        let bits = {
            const MASK: u8 = 0x7f;
            const OFFSET: u8 = 22;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        DEVADDRR { bits }
    }
    #[doc = "Bit 29 - Odd Frame"]
    #[inline]
    pub fn oddfrm(&self) -> ODDFRMR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 29;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        ODDFRMR { bits }
    }
    #[doc = "Bit 30 - Channel Disable"]
    #[inline]
    pub fn chdis(&self) -> CHDISR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 30;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        CHDISR { bits }
    }
    #[doc = "Bit 31 - Channel Enable"]
    #[inline]
    pub fn chena(&self) -> CHENAR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 31;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        CHENAR { bits }
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
    #[doc = "Bits 0:10 - Maximum Packet Size"]
    #[inline]
    pub fn mps(&mut self) -> _MPSW {
        _MPSW { w: self }
    }
    #[doc = "Bits 11:14 - Endpoint Number"]
    #[inline]
    pub fn epnum(&mut self) -> _EPNUMW {
        _EPNUMW { w: self }
    }
    #[doc = "Bit 15 - Endpoint Direction"]
    #[inline]
    pub fn epdir(&mut self) -> _EPDIRW {
        _EPDIRW { w: self }
    }
    #[doc = "Bit 17 - Low-Speed Device"]
    #[inline]
    pub fn lspddev(&mut self) -> _LSPDDEVW {
        _LSPDDEVW { w: self }
    }
    #[doc = "Bits 18:19 - Endpoint Type"]
    #[inline]
    pub fn eptype(&mut self) -> _EPTYPEW {
        _EPTYPEW { w: self }
    }
    #[doc = "Bits 20:21 - Multi Count"]
    #[inline]
    pub fn mc(&mut self) -> _MCW {
        _MCW { w: self }
    }
    #[doc = "Bits 22:28 - Device Address"]
    #[inline]
    pub fn devaddr(&mut self) -> _DEVADDRW {
        _DEVADDRW { w: self }
    }
    #[doc = "Bit 29 - Odd Frame"]
    #[inline]
    pub fn oddfrm(&mut self) -> _ODDFRMW {
        _ODDFRMW { w: self }
    }
    #[doc = "Bit 30 - Channel Disable"]
    #[inline]
    pub fn chdis(&mut self) -> _CHDISW {
        _CHDISW { w: self }
    }
    #[doc = "Bit 31 - Channel Enable"]
    #[inline]
    pub fn chena(&mut self) -> _CHENAW {
        _CHENAW { w: self }
    }
}

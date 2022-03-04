#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::EM4CONF {
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
pub struct VREGENR {
    bits: bool,
}
impl VREGENR {
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
pub struct BURTCWUR {
    bits: bool,
}
impl BURTCWUR {
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
#[doc = "Possible values of the field `OSC`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum OSCR {
    #[doc = "ULFRCO is available."]
    ULFRCO,
    #[doc = "LFRCO is available. Can only be set if LFRCO is running before EM4/backup entry."]
    LFRCO,
    #[doc = "LFXO is available. Can only be set if LFXO is available before EM4/backup entry."]
    LFXO,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl OSCR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            OSCR::ULFRCO => 0,
            OSCR::LFRCO => 0x01,
            OSCR::LFXO => 0x02,
            OSCR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> OSCR {
        match value {
            0 => OSCR::ULFRCO,
            1 => OSCR::LFRCO,
            2 => OSCR::LFXO,
            i => OSCR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `ULFRCO`"]
    #[inline]
    pub fn is_ulfrco(&self) -> bool {
        *self == OSCR::ULFRCO
    }
    #[doc = "Checks if the value of the field is `LFRCO`"]
    #[inline]
    pub fn is_lfrco(&self) -> bool {
        *self == OSCR::LFRCO
    }
    #[doc = "Checks if the value of the field is `LFXO`"]
    #[inline]
    pub fn is_lfxo(&self) -> bool {
        *self == OSCR::LFXO
    }
}
#[doc = r" Value of the field"]
pub struct BUBODRSTDISR {
    bits: bool,
}
impl BUBODRSTDISR {
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
pub struct LOCKCONFR {
    bits: bool,
}
impl LOCKCONFR {
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
pub struct _VREGENW<'a> {
    w: &'a mut W,
}
impl<'a> _VREGENW<'a> {
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
pub struct _BURTCWUW<'a> {
    w: &'a mut W,
}
impl<'a> _BURTCWUW<'a> {
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
#[doc = "Values that can be written to the field `OSC`"]
pub enum OSCW {
    #[doc = "ULFRCO is available."]
    ULFRCO,
    #[doc = "LFRCO is available. Can only be set if LFRCO is running before EM4/backup entry."]
    LFRCO,
    #[doc = "LFXO is available. Can only be set if LFXO is available before EM4/backup entry."]
    LFXO,
}
impl OSCW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            OSCW::ULFRCO => 0,
            OSCW::LFRCO => 1,
            OSCW::LFXO => 2,
        }
    }
}
#[doc = r" Proxy"]
pub struct _OSCW<'a> {
    w: &'a mut W,
}
impl<'a> _OSCW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: OSCW) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "ULFRCO is available."]
    #[inline]
    pub fn ulfrco(self) -> &'a mut W {
        self.variant(OSCW::ULFRCO)
    }
    #[doc = "LFRCO is available. Can only be set if LFRCO is running before EM4/backup entry."]
    #[inline]
    pub fn lfrco(self) -> &'a mut W {
        self.variant(OSCW::LFRCO)
    }
    #[doc = "LFXO is available. Can only be set if LFXO is available before EM4/backup entry."]
    #[inline]
    pub fn lfxo(self) -> &'a mut W {
        self.variant(OSCW::LFXO)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 0x03;
        const OFFSET: u8 = 2;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _BUBODRSTDISW<'a> {
    w: &'a mut W,
}
impl<'a> _BUBODRSTDISW<'a> {
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
pub struct _LOCKCONFW<'a> {
    w: &'a mut W,
}
impl<'a> _LOCKCONFW<'a> {
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
impl R {
    #[doc = r" Value of the register as raw bits"]
    #[inline]
    pub fn bits(&self) -> u32 {
        self.bits
    }
    #[doc = "Bit 0 - EM4 voltage regulator enable"]
    #[inline]
    pub fn vregen(&self) -> VREGENR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        VREGENR { bits }
    }
    #[doc = "Bit 1 - Backup RTC EM4 wakeup enable"]
    #[inline]
    pub fn burtcwu(&self) -> BURTCWUR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 1;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        BURTCWUR { bits }
    }
    #[doc = "Bits 2:3 - Select EM4 duty oscillator"]
    #[inline]
    pub fn osc(&self) -> OSCR {
        OSCR::_from({
            const MASK: u8 = 0x03;
            const OFFSET: u8 = 2;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bit 4 - Disable reset from Backup BOD in EM4"]
    #[inline]
    pub fn bubodrstdis(&self) -> BUBODRSTDISR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 4;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        BUBODRSTDISR { bits }
    }
    #[doc = "Bit 16 - EM4 configuration lock enable"]
    #[inline]
    pub fn lockconf(&self) -> LOCKCONFR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 16;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        LOCKCONFR { bits }
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
    #[doc = "Bit 0 - EM4 voltage regulator enable"]
    #[inline]
    pub fn vregen(&mut self) -> _VREGENW {
        _VREGENW { w: self }
    }
    #[doc = "Bit 1 - Backup RTC EM4 wakeup enable"]
    #[inline]
    pub fn burtcwu(&mut self) -> _BURTCWUW {
        _BURTCWUW { w: self }
    }
    #[doc = "Bits 2:3 - Select EM4 duty oscillator"]
    #[inline]
    pub fn osc(&mut self) -> _OSCW {
        _OSCW { w: self }
    }
    #[doc = "Bit 4 - Disable reset from Backup BOD in EM4"]
    #[inline]
    pub fn bubodrstdis(&mut self) -> _BUBODRSTDISW {
        _BUBODRSTDISW { w: self }
    }
    #[doc = "Bit 16 - EM4 configuration lock enable"]
    #[inline]
    pub fn lockconf(&mut self) -> _LOCKCONFW {
        _LOCKCONFW { w: self }
    }
}

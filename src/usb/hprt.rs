#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::HPRT {
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
pub struct PRTCONNSTSR {
    bits: bool,
}
impl PRTCONNSTSR {
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
pub struct PRTCONNDETR {
    bits: bool,
}
impl PRTCONNDETR {
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
pub struct PRTENAR {
    bits: bool,
}
impl PRTENAR {
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
pub struct PRTENCHNGR {
    bits: bool,
}
impl PRTENCHNGR {
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
pub struct PRTOVRCURRACTR {
    bits: bool,
}
impl PRTOVRCURRACTR {
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
pub struct PRTOVRCURRCHNGR {
    bits: bool,
}
impl PRTOVRCURRCHNGR {
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
pub struct PRTRESR {
    bits: bool,
}
impl PRTRESR {
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
pub struct PRTSUSPR {
    bits: bool,
}
impl PRTSUSPR {
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
pub struct PRTRSTR {
    bits: bool,
}
impl PRTRSTR {
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
pub struct PRTLNSTSR {
    bits: u8,
}
impl PRTLNSTSR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct PRTPWRR {
    bits: bool,
}
impl PRTPWRR {
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
#[doc = "Possible values of the field `PRTTSTCTL`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PRTTSTCTLR {
    #[doc = "Test mode disabled."]
    DISABLE,
    #[doc = "Test_J mode."]
    J,
    #[doc = "Test_K mode."]
    K,
    #[doc = "Test_SE0_NAK mode."]
    SE0NAK,
    #[doc = "Test_Packet mode."]
    PACKET,
    #[doc = "Test_Force_Enable."]
    FORCE,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl PRTTSTCTLR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            PRTTSTCTLR::DISABLE => 0,
            PRTTSTCTLR::J => 0x01,
            PRTTSTCTLR::K => 0x02,
            PRTTSTCTLR::SE0NAK => 0x03,
            PRTTSTCTLR::PACKET => 0x04,
            PRTTSTCTLR::FORCE => 0x05,
            PRTTSTCTLR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> PRTTSTCTLR {
        match value {
            0 => PRTTSTCTLR::DISABLE,
            1 => PRTTSTCTLR::J,
            2 => PRTTSTCTLR::K,
            3 => PRTTSTCTLR::SE0NAK,
            4 => PRTTSTCTLR::PACKET,
            5 => PRTTSTCTLR::FORCE,
            i => PRTTSTCTLR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline]
    pub fn is_disable(&self) -> bool {
        *self == PRTTSTCTLR::DISABLE
    }
    #[doc = "Checks if the value of the field is `J`"]
    #[inline]
    pub fn is_j(&self) -> bool {
        *self == PRTTSTCTLR::J
    }
    #[doc = "Checks if the value of the field is `K`"]
    #[inline]
    pub fn is_k(&self) -> bool {
        *self == PRTTSTCTLR::K
    }
    #[doc = "Checks if the value of the field is `SE0NAK`"]
    #[inline]
    pub fn is_se0nak(&self) -> bool {
        *self == PRTTSTCTLR::SE0NAK
    }
    #[doc = "Checks if the value of the field is `PACKET`"]
    #[inline]
    pub fn is_packet(&self) -> bool {
        *self == PRTTSTCTLR::PACKET
    }
    #[doc = "Checks if the value of the field is `FORCE`"]
    #[inline]
    pub fn is_force(&self) -> bool {
        *self == PRTTSTCTLR::FORCE
    }
}
#[doc = "Possible values of the field `PRTSPD`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PRTSPDR {
    #[doc = "High speed."]
    HS,
    #[doc = "Full speed."]
    FS,
    #[doc = "Low speed."]
    LS,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl PRTSPDR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            PRTSPDR::HS => 0,
            PRTSPDR::FS => 0x01,
            PRTSPDR::LS => 0x02,
            PRTSPDR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> PRTSPDR {
        match value {
            0 => PRTSPDR::HS,
            1 => PRTSPDR::FS,
            2 => PRTSPDR::LS,
            i => PRTSPDR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `HS`"]
    #[inline]
    pub fn is_hs(&self) -> bool {
        *self == PRTSPDR::HS
    }
    #[doc = "Checks if the value of the field is `FS`"]
    #[inline]
    pub fn is_fs(&self) -> bool {
        *self == PRTSPDR::FS
    }
    #[doc = "Checks if the value of the field is `LS`"]
    #[inline]
    pub fn is_ls(&self) -> bool {
        *self == PRTSPDR::LS
    }
}
#[doc = r" Proxy"]
pub struct _PRTCONNDETW<'a> {
    w: &'a mut W,
}
impl<'a> _PRTCONNDETW<'a> {
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
#[doc = r" Proxy"]
pub struct _PRTENAW<'a> {
    w: &'a mut W,
}
impl<'a> _PRTENAW<'a> {
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
pub struct _PRTENCHNGW<'a> {
    w: &'a mut W,
}
impl<'a> _PRTENCHNGW<'a> {
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
        const OFFSET: u8 = 3;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _PRTOVRCURRCHNGW<'a> {
    w: &'a mut W,
}
impl<'a> _PRTOVRCURRCHNGW<'a> {
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
        const OFFSET: u8 = 5;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _PRTRESW<'a> {
    w: &'a mut W,
}
impl<'a> _PRTRESW<'a> {
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
        const OFFSET: u8 = 6;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _PRTSUSPW<'a> {
    w: &'a mut W,
}
impl<'a> _PRTSUSPW<'a> {
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
pub struct _PRTRSTW<'a> {
    w: &'a mut W,
}
impl<'a> _PRTRSTW<'a> {
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
pub struct _PRTPWRW<'a> {
    w: &'a mut W,
}
impl<'a> _PRTPWRW<'a> {
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
#[doc = "Values that can be written to the field `PRTTSTCTL`"]
pub enum PRTTSTCTLW {
    #[doc = "Test mode disabled."]
    DISABLE,
    #[doc = "Test_J mode."]
    J,
    #[doc = "Test_K mode."]
    K,
    #[doc = "Test_SE0_NAK mode."]
    SE0NAK,
    #[doc = "Test_Packet mode."]
    PACKET,
    #[doc = "Test_Force_Enable."]
    FORCE,
}
impl PRTTSTCTLW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            PRTTSTCTLW::DISABLE => 0,
            PRTTSTCTLW::J => 1,
            PRTTSTCTLW::K => 2,
            PRTTSTCTLW::SE0NAK => 3,
            PRTTSTCTLW::PACKET => 4,
            PRTTSTCTLW::FORCE => 5,
        }
    }
}
#[doc = r" Proxy"]
pub struct _PRTTSTCTLW<'a> {
    w: &'a mut W,
}
impl<'a> _PRTTSTCTLW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: PRTTSTCTLW) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "Test mode disabled."]
    #[inline]
    pub fn disable(self) -> &'a mut W {
        self.variant(PRTTSTCTLW::DISABLE)
    }
    #[doc = "Test_J mode."]
    #[inline]
    pub fn j(self) -> &'a mut W {
        self.variant(PRTTSTCTLW::J)
    }
    #[doc = "Test_K mode."]
    #[inline]
    pub fn k(self) -> &'a mut W {
        self.variant(PRTTSTCTLW::K)
    }
    #[doc = "Test_SE0_NAK mode."]
    #[inline]
    pub fn se0nak(self) -> &'a mut W {
        self.variant(PRTTSTCTLW::SE0NAK)
    }
    #[doc = "Test_Packet mode."]
    #[inline]
    pub fn packet(self) -> &'a mut W {
        self.variant(PRTTSTCTLW::PACKET)
    }
    #[doc = "Test_Force_Enable."]
    #[inline]
    pub fn force(self) -> &'a mut W {
        self.variant(PRTTSTCTLW::FORCE)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 0x0f;
        const OFFSET: u8 = 13;
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
    #[doc = "Bit 0 - Port Connect Status"]
    #[inline]
    pub fn prtconnsts(&self) -> PRTCONNSTSR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        PRTCONNSTSR { bits }
    }
    #[doc = "Bit 1 - Port Connect Detected"]
    #[inline]
    pub fn prtconndet(&self) -> PRTCONNDETR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 1;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        PRTCONNDETR { bits }
    }
    #[doc = "Bit 2 - Port Enable"]
    #[inline]
    pub fn prtena(&self) -> PRTENAR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 2;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        PRTENAR { bits }
    }
    #[doc = "Bit 3 - Port Enable/Disable Change"]
    #[inline]
    pub fn prtenchng(&self) -> PRTENCHNGR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 3;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        PRTENCHNGR { bits }
    }
    #[doc = "Bit 4 - Port Overcurrent Active"]
    #[inline]
    pub fn prtovrcurract(&self) -> PRTOVRCURRACTR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 4;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        PRTOVRCURRACTR { bits }
    }
    #[doc = "Bit 5 - Port Overcurrent Change"]
    #[inline]
    pub fn prtovrcurrchng(&self) -> PRTOVRCURRCHNGR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 5;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        PRTOVRCURRCHNGR { bits }
    }
    #[doc = "Bit 6 - Port Resume"]
    #[inline]
    pub fn prtres(&self) -> PRTRESR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 6;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        PRTRESR { bits }
    }
    #[doc = "Bit 7 - Port Suspend"]
    #[inline]
    pub fn prtsusp(&self) -> PRTSUSPR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 7;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        PRTSUSPR { bits }
    }
    #[doc = "Bit 8 - Port Reset"]
    #[inline]
    pub fn prtrst(&self) -> PRTRSTR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 8;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        PRTRSTR { bits }
    }
    #[doc = "Bits 10:11 - Port Line Status"]
    #[inline]
    pub fn prtlnsts(&self) -> PRTLNSTSR {
        let bits = {
            const MASK: u8 = 0x03;
            const OFFSET: u8 = 10;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        PRTLNSTSR { bits }
    }
    #[doc = "Bit 12 - Port Power"]
    #[inline]
    pub fn prtpwr(&self) -> PRTPWRR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 12;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        PRTPWRR { bits }
    }
    #[doc = "Bits 13:16 - Port Test Control"]
    #[inline]
    pub fn prttstctl(&self) -> PRTTSTCTLR {
        PRTTSTCTLR::_from({
            const MASK: u8 = 0x0f;
            const OFFSET: u8 = 13;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 17:18 - Port Speed"]
    #[inline]
    pub fn prtspd(&self) -> PRTSPDR {
        PRTSPDR::_from({
            const MASK: u8 = 0x03;
            const OFFSET: u8 = 17;
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
    #[doc = "Bit 1 - Port Connect Detected"]
    #[inline]
    pub fn prtconndet(&mut self) -> _PRTCONNDETW {
        _PRTCONNDETW { w: self }
    }
    #[doc = "Bit 2 - Port Enable"]
    #[inline]
    pub fn prtena(&mut self) -> _PRTENAW {
        _PRTENAW { w: self }
    }
    #[doc = "Bit 3 - Port Enable/Disable Change"]
    #[inline]
    pub fn prtenchng(&mut self) -> _PRTENCHNGW {
        _PRTENCHNGW { w: self }
    }
    #[doc = "Bit 5 - Port Overcurrent Change"]
    #[inline]
    pub fn prtovrcurrchng(&mut self) -> _PRTOVRCURRCHNGW {
        _PRTOVRCURRCHNGW { w: self }
    }
    #[doc = "Bit 6 - Port Resume"]
    #[inline]
    pub fn prtres(&mut self) -> _PRTRESW {
        _PRTRESW { w: self }
    }
    #[doc = "Bit 7 - Port Suspend"]
    #[inline]
    pub fn prtsusp(&mut self) -> _PRTSUSPW {
        _PRTSUSPW { w: self }
    }
    #[doc = "Bit 8 - Port Reset"]
    #[inline]
    pub fn prtrst(&mut self) -> _PRTRSTW {
        _PRTRSTW { w: self }
    }
    #[doc = "Bit 12 - Port Power"]
    #[inline]
    pub fn prtpwr(&mut self) -> _PRTPWRW {
        _PRTPWRW { w: self }
    }
    #[doc = "Bits 13:16 - Port Test Control"]
    #[inline]
    pub fn prttstctl(&mut self) -> _PRTTSTCTLW {
        _PRTTSTCTLW { w: self }
    }
}

#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::BUACT {
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
pub struct BUEXTHRESR {
    bits: u8,
}
impl BUEXTHRESR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct BUEXRANGER {
    bits: u8,
}
impl BUEXRANGER {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = "Possible values of the field `PWRCON`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PWRCONR {
    #[doc = "No connection."]
    NONE,
    #[doc = "Main power and backup power are connected through a diode, allowing current to flow from backup power source to main power source, but not the other way."]
    BUMAIN,
    #[doc = "Main power and backup power are connected through a diode, allowing current to flow from main power source to backup power source, but not the other way."]
    MAINBU,
    #[doc = "Main power and backup power are connected without diode."]
    NODIODE,
}
impl PWRCONR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            PWRCONR::NONE => 0,
            PWRCONR::BUMAIN => 0x01,
            PWRCONR::MAINBU => 0x02,
            PWRCONR::NODIODE => 0x03,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> PWRCONR {
        match value {
            0 => PWRCONR::NONE,
            1 => PWRCONR::BUMAIN,
            2 => PWRCONR::MAINBU,
            3 => PWRCONR::NODIODE,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `NONE`"]
    #[inline]
    pub fn is_none(&self) -> bool {
        *self == PWRCONR::NONE
    }
    #[doc = "Checks if the value of the field is `BUMAIN`"]
    #[inline]
    pub fn is_bumain(&self) -> bool {
        *self == PWRCONR::BUMAIN
    }
    #[doc = "Checks if the value of the field is `MAINBU`"]
    #[inline]
    pub fn is_mainbu(&self) -> bool {
        *self == PWRCONR::MAINBU
    }
    #[doc = "Checks if the value of the field is `NODIODE`"]
    #[inline]
    pub fn is_nodiode(&self) -> bool {
        *self == PWRCONR::NODIODE
    }
}
#[doc = r" Proxy"]
pub struct _BUEXTHRESW<'a> {
    w: &'a mut W,
}
impl<'a> _BUEXTHRESW<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 0x07;
        const OFFSET: u8 = 0;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _BUEXRANGEW<'a> {
    w: &'a mut W,
}
impl<'a> _BUEXRANGEW<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 0x03;
        const OFFSET: u8 = 3;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `PWRCON`"]
pub enum PWRCONW {
    #[doc = "No connection."]
    NONE,
    #[doc = "Main power and backup power are connected through a diode, allowing current to flow from backup power source to main power source, but not the other way."]
    BUMAIN,
    #[doc = "Main power and backup power are connected through a diode, allowing current to flow from main power source to backup power source, but not the other way."]
    MAINBU,
    #[doc = "Main power and backup power are connected without diode."]
    NODIODE,
}
impl PWRCONW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            PWRCONW::NONE => 0,
            PWRCONW::BUMAIN => 1,
            PWRCONW::MAINBU => 2,
            PWRCONW::NODIODE => 3,
        }
    }
}
#[doc = r" Proxy"]
pub struct _PWRCONW<'a> {
    w: &'a mut W,
}
impl<'a> _PWRCONW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: PWRCONW) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "No connection."]
    #[inline]
    pub fn none(self) -> &'a mut W {
        self.variant(PWRCONW::NONE)
    }
    #[doc = "Main power and backup power are connected through a diode, allowing current to flow from backup power source to main power source, but not the other way."]
    #[inline]
    pub fn bumain(self) -> &'a mut W {
        self.variant(PWRCONW::BUMAIN)
    }
    #[doc = "Main power and backup power are connected through a diode, allowing current to flow from main power source to backup power source, but not the other way."]
    #[inline]
    pub fn mainbu(self) -> &'a mut W {
        self.variant(PWRCONW::MAINBU)
    }
    #[doc = "Main power and backup power are connected without diode."]
    #[inline]
    pub fn nodiode(self) -> &'a mut W {
        self.variant(PWRCONW::NODIODE)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 0x03;
        const OFFSET: u8 = 5;
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
    #[doc = "Bits 0:2 - \"\""]
    #[inline]
    pub fn buexthres(&self) -> BUEXTHRESR {
        let bits = {
            const MASK: u8 = 0x07;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        BUEXTHRESR { bits }
    }
    #[doc = "Bits 3:4 - \"\""]
    #[inline]
    pub fn buexrange(&self) -> BUEXRANGER {
        let bits = {
            const MASK: u8 = 0x03;
            const OFFSET: u8 = 3;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        BUEXRANGER { bits }
    }
    #[doc = "Bits 5:6 - Power connection configuration when in Backup mode"]
    #[inline]
    pub fn pwrcon(&self) -> PWRCONR {
        PWRCONR::_from({
            const MASK: u8 = 0x03;
            const OFFSET: u8 = 5;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
}
impl W {
    #[doc = r" Reset value of the register"]
    #[inline]
    pub fn reset_value() -> W {
        W { bits: 0x0b }
    }
    #[doc = r" Writes raw bits to the register"]
    #[inline]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
    #[doc = "Bits 0:2 - \"\""]
    #[inline]
    pub fn buexthres(&mut self) -> _BUEXTHRESW {
        _BUEXTHRESW { w: self }
    }
    #[doc = "Bits 3:4 - \"\""]
    #[inline]
    pub fn buexrange(&mut self) -> _BUEXRANGEW {
        _BUEXRANGEW { w: self }
    }
    #[doc = "Bits 5:6 - Power connection configuration when in Backup mode"]
    #[inline]
    pub fn pwrcon(&mut self) -> _PWRCONW {
        _PWRCONW { w: self }
    }
}

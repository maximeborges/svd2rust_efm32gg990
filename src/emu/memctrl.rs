#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::MEMCTRL {
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
#[doc = "Possible values of the field `POWERDOWN`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum POWERDOWNR {
    #[doc = "Power down RAM block 3 (address range 0x20018000-0x2001FFFF)."]
    BLK3,
    #[doc = "Power down RAM blocks 2-3 (address range 0x20010000-0x2001FFFF)."]
    BLK23,
    #[doc = "Power down RAM blocks 1-3 (address range 0x20008000-0x2001FFFF)."]
    BLK123,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl POWERDOWNR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            POWERDOWNR::BLK3 => 0x04,
            POWERDOWNR::BLK23 => 0x06,
            POWERDOWNR::BLK123 => 0x07,
            POWERDOWNR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> POWERDOWNR {
        match value {
            4 => POWERDOWNR::BLK3,
            6 => POWERDOWNR::BLK23,
            7 => POWERDOWNR::BLK123,
            i => POWERDOWNR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `BLK3`"]
    #[inline]
    pub fn is_blk3(&self) -> bool {
        *self == POWERDOWNR::BLK3
    }
    #[doc = "Checks if the value of the field is `BLK23`"]
    #[inline]
    pub fn is_blk23(&self) -> bool {
        *self == POWERDOWNR::BLK23
    }
    #[doc = "Checks if the value of the field is `BLK123`"]
    #[inline]
    pub fn is_blk123(&self) -> bool {
        *self == POWERDOWNR::BLK123
    }
}
#[doc = "Values that can be written to the field `POWERDOWN`"]
pub enum POWERDOWNW {
    #[doc = "Power down RAM block 3 (address range 0x20018000-0x2001FFFF)."]
    BLK3,
    #[doc = "Power down RAM blocks 2-3 (address range 0x20010000-0x2001FFFF)."]
    BLK23,
    #[doc = "Power down RAM blocks 1-3 (address range 0x20008000-0x2001FFFF)."]
    BLK123,
}
impl POWERDOWNW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            POWERDOWNW::BLK3 => 4,
            POWERDOWNW::BLK23 => 6,
            POWERDOWNW::BLK123 => 7,
        }
    }
}
#[doc = r" Proxy"]
pub struct _POWERDOWNW<'a> {
    w: &'a mut W,
}
impl<'a> _POWERDOWNW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: POWERDOWNW) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "Power down RAM block 3 (address range 0x20018000-0x2001FFFF)."]
    #[inline]
    pub fn blk3(self) -> &'a mut W {
        self.variant(POWERDOWNW::BLK3)
    }
    #[doc = "Power down RAM blocks 2-3 (address range 0x20010000-0x2001FFFF)."]
    #[inline]
    pub fn blk23(self) -> &'a mut W {
        self.variant(POWERDOWNW::BLK23)
    }
    #[doc = "Power down RAM blocks 1-3 (address range 0x20008000-0x2001FFFF)."]
    #[inline]
    pub fn blk123(self) -> &'a mut W {
        self.variant(POWERDOWNW::BLK123)
    }
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
impl R {
    #[doc = r" Value of the register as raw bits"]
    #[inline]
    pub fn bits(&self) -> u32 {
        self.bits
    }
    #[doc = "Bits 0:2 - RAM block power-down"]
    #[inline]
    pub fn powerdown(&self) -> POWERDOWNR {
        POWERDOWNR::_from({
            const MASK: u8 = 0x07;
            const OFFSET: u8 = 0;
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
    #[doc = "Bits 0:2 - RAM block power-down"]
    #[inline]
    pub fn powerdown(&mut self) -> _POWERDOWNW {
        _POWERDOWNW { w: self }
    }
}

#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::HC9_TSIZ {
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
pub struct XFERSIZER {
    bits: u32,
}
impl XFERSIZER {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u32 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct PKTCNTR {
    bits: u16,
}
impl PKTCNTR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u16 {
        self.bits
    }
}
#[doc = "Possible values of the field `PID`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PIDR {
    #[doc = "DATA0 PID."]
    DATA0,
    #[doc = "DATA2 PID."]
    DATA2,
    #[doc = "DATA1 PID."]
    DATA1,
    #[doc = "MDATA (non-control) / SETUP (control) PID."]
    MDATA,
}
impl PIDR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            PIDR::DATA0 => 0,
            PIDR::DATA2 => 0x01,
            PIDR::DATA1 => 0x02,
            PIDR::MDATA => 0x03,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> PIDR {
        match value {
            0 => PIDR::DATA0,
            1 => PIDR::DATA2,
            2 => PIDR::DATA1,
            3 => PIDR::MDATA,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `DATA0`"]
    #[inline]
    pub fn is_data0(&self) -> bool {
        *self == PIDR::DATA0
    }
    #[doc = "Checks if the value of the field is `DATA2`"]
    #[inline]
    pub fn is_data2(&self) -> bool {
        *self == PIDR::DATA2
    }
    #[doc = "Checks if the value of the field is `DATA1`"]
    #[inline]
    pub fn is_data1(&self) -> bool {
        *self == PIDR::DATA1
    }
    #[doc = "Checks if the value of the field is `MDATA`"]
    #[inline]
    pub fn is_mdata(&self) -> bool {
        *self == PIDR::MDATA
    }
}
#[doc = r" Proxy"]
pub struct _XFERSIZEW<'a> {
    w: &'a mut W,
}
impl<'a> _XFERSIZEW<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        const MASK: u32 = 0x0007_ffff;
        const OFFSET: u8 = 0;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _PKTCNTW<'a> {
    w: &'a mut W,
}
impl<'a> _PKTCNTW<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        const MASK: u16 = 0x03ff;
        const OFFSET: u8 = 19;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `PID`"]
pub enum PIDW {
    #[doc = "DATA0 PID."]
    DATA0,
    #[doc = "DATA2 PID."]
    DATA2,
    #[doc = "DATA1 PID."]
    DATA1,
    #[doc = "MDATA (non-control) / SETUP (control) PID."]
    MDATA,
}
impl PIDW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            PIDW::DATA0 => 0,
            PIDW::DATA2 => 1,
            PIDW::DATA1 => 2,
            PIDW::MDATA => 3,
        }
    }
}
#[doc = r" Proxy"]
pub struct _PIDW<'a> {
    w: &'a mut W,
}
impl<'a> _PIDW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: PIDW) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "DATA0 PID."]
    #[inline]
    pub fn data0(self) -> &'a mut W {
        self.variant(PIDW::DATA0)
    }
    #[doc = "DATA2 PID."]
    #[inline]
    pub fn data2(self) -> &'a mut W {
        self.variant(PIDW::DATA2)
    }
    #[doc = "DATA1 PID."]
    #[inline]
    pub fn data1(self) -> &'a mut W {
        self.variant(PIDW::DATA1)
    }
    #[doc = "MDATA (non-control) / SETUP (control) PID."]
    #[inline]
    pub fn mdata(self) -> &'a mut W {
        self.variant(PIDW::MDATA)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 0x03;
        const OFFSET: u8 = 29;
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
    #[doc = "Bits 0:18 - Transfer Size"]
    #[inline]
    pub fn xfersize(&self) -> XFERSIZER {
        let bits = {
            const MASK: u32 = 0x0007_ffff;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) as u32
        };
        XFERSIZER { bits }
    }
    #[doc = "Bits 19:28 - Packet Count"]
    #[inline]
    pub fn pktcnt(&self) -> PKTCNTR {
        let bits = {
            const MASK: u16 = 0x03ff;
            const OFFSET: u8 = 19;
            ((self.bits >> OFFSET) & MASK as u32) as u16
        };
        PKTCNTR { bits }
    }
    #[doc = "Bits 29:30 - Packet ID"]
    #[inline]
    pub fn pid(&self) -> PIDR {
        PIDR::_from({
            const MASK: u8 = 0x03;
            const OFFSET: u8 = 29;
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
    #[doc = "Bits 0:18 - Transfer Size"]
    #[inline]
    pub fn xfersize(&mut self) -> _XFERSIZEW {
        _XFERSIZEW { w: self }
    }
    #[doc = "Bits 19:28 - Packet Count"]
    #[inline]
    pub fn pktcnt(&mut self) -> _PKTCNTW {
        _PKTCNTW { w: self }
    }
    #[doc = "Bits 29:30 - Packet ID"]
    #[inline]
    pub fn pid(&mut self) -> _PIDW {
        _PIDW { w: self }
    }
}

#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::CHREQMASKC {
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
}
#[doc = r" Proxy"]
pub struct _CH0REQMASKCW<'a> {
    w: &'a mut W,
}
impl<'a> _CH0REQMASKCW<'a> {
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
pub struct _CH1REQMASKCW<'a> {
    w: &'a mut W,
}
impl<'a> _CH1REQMASKCW<'a> {
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
pub struct _CH2REQMASKCW<'a> {
    w: &'a mut W,
}
impl<'a> _CH2REQMASKCW<'a> {
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
pub struct _CH3REQMASKCW<'a> {
    w: &'a mut W,
}
impl<'a> _CH3REQMASKCW<'a> {
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
pub struct _CH4REQMASKCW<'a> {
    w: &'a mut W,
}
impl<'a> _CH4REQMASKCW<'a> {
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
pub struct _CH5REQMASKCW<'a> {
    w: &'a mut W,
}
impl<'a> _CH5REQMASKCW<'a> {
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
pub struct _CH6REQMASKCW<'a> {
    w: &'a mut W,
}
impl<'a> _CH6REQMASKCW<'a> {
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
pub struct _CH7REQMASKCW<'a> {
    w: &'a mut W,
}
impl<'a> _CH7REQMASKCW<'a> {
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
pub struct _CH8REQMASKCW<'a> {
    w: &'a mut W,
}
impl<'a> _CH8REQMASKCW<'a> {
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
pub struct _CH9REQMASKCW<'a> {
    w: &'a mut W,
}
impl<'a> _CH9REQMASKCW<'a> {
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
#[doc = r" Proxy"]
pub struct _CH10REQMASKCW<'a> {
    w: &'a mut W,
}
impl<'a> _CH10REQMASKCW<'a> {
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
        const OFFSET: u8 = 10;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _CH11REQMASKCW<'a> {
    w: &'a mut W,
}
impl<'a> _CH11REQMASKCW<'a> {
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
        const OFFSET: u8 = 11;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
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
    #[doc = "Bit 0 - Channel 0 Request Mask Clear"]
    #[inline]
    pub fn ch0reqmaskc(&mut self) -> _CH0REQMASKCW {
        _CH0REQMASKCW { w: self }
    }
    #[doc = "Bit 1 - Channel 1 Request Mask Clear"]
    #[inline]
    pub fn ch1reqmaskc(&mut self) -> _CH1REQMASKCW {
        _CH1REQMASKCW { w: self }
    }
    #[doc = "Bit 2 - Channel 2 Request Mask Clear"]
    #[inline]
    pub fn ch2reqmaskc(&mut self) -> _CH2REQMASKCW {
        _CH2REQMASKCW { w: self }
    }
    #[doc = "Bit 3 - Channel 3 Request Mask Clear"]
    #[inline]
    pub fn ch3reqmaskc(&mut self) -> _CH3REQMASKCW {
        _CH3REQMASKCW { w: self }
    }
    #[doc = "Bit 4 - Channel 4 Request Mask Clear"]
    #[inline]
    pub fn ch4reqmaskc(&mut self) -> _CH4REQMASKCW {
        _CH4REQMASKCW { w: self }
    }
    #[doc = "Bit 5 - Channel 5 Request Mask Clear"]
    #[inline]
    pub fn ch5reqmaskc(&mut self) -> _CH5REQMASKCW {
        _CH5REQMASKCW { w: self }
    }
    #[doc = "Bit 6 - Channel 6 Request Mask Clear"]
    #[inline]
    pub fn ch6reqmaskc(&mut self) -> _CH6REQMASKCW {
        _CH6REQMASKCW { w: self }
    }
    #[doc = "Bit 7 - Channel 7 Request Mask Clear"]
    #[inline]
    pub fn ch7reqmaskc(&mut self) -> _CH7REQMASKCW {
        _CH7REQMASKCW { w: self }
    }
    #[doc = "Bit 8 - Channel 8 Request Mask Clear"]
    #[inline]
    pub fn ch8reqmaskc(&mut self) -> _CH8REQMASKCW {
        _CH8REQMASKCW { w: self }
    }
    #[doc = "Bit 9 - Channel 9 Request Mask Clear"]
    #[inline]
    pub fn ch9reqmaskc(&mut self) -> _CH9REQMASKCW {
        _CH9REQMASKCW { w: self }
    }
    #[doc = "Bit 10 - Channel 10 Request Mask Clear"]
    #[inline]
    pub fn ch10reqmaskc(&mut self) -> _CH10REQMASKCW {
        _CH10REQMASKCW { w: self }
    }
    #[doc = "Bit 11 - Channel 11 Request Mask Clear"]
    #[inline]
    pub fn ch11reqmaskc(&mut self) -> _CH11REQMASKCW {
        _CH11REQMASKCW { w: self }
    }
}

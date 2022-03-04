#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::CHUSEBURSTC {
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
pub struct _CH0USEBURSTCW<'a> {
    w: &'a mut W,
}
impl<'a> _CH0USEBURSTCW<'a> {
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
pub struct _CH1USEBURSTCW<'a> {
    w: &'a mut W,
}
impl<'a> _CH1USEBURSTCW<'a> {
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
pub struct _CH2USEBURSTCW<'a> {
    w: &'a mut W,
}
impl<'a> _CH2USEBURSTCW<'a> {
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
pub struct _CH3USEBURSTCW<'a> {
    w: &'a mut W,
}
impl<'a> _CH3USEBURSTCW<'a> {
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
pub struct _CH4USEBURSTCW<'a> {
    w: &'a mut W,
}
impl<'a> _CH4USEBURSTCW<'a> {
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
pub struct _CH5USEBURSTCW<'a> {
    w: &'a mut W,
}
impl<'a> _CH5USEBURSTCW<'a> {
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
pub struct _CH6USEBURSTCW<'a> {
    w: &'a mut W,
}
impl<'a> _CH6USEBURSTCW<'a> {
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
pub struct _CH7USEBURSTCW<'a> {
    w: &'a mut W,
}
impl<'a> _CH7USEBURSTCW<'a> {
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
pub struct _CH08USEBURSTCW<'a> {
    w: &'a mut W,
}
impl<'a> _CH08USEBURSTCW<'a> {
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
pub struct _CH9USEBURSTCW<'a> {
    w: &'a mut W,
}
impl<'a> _CH9USEBURSTCW<'a> {
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
pub struct _CH10USEBURSTCW<'a> {
    w: &'a mut W,
}
impl<'a> _CH10USEBURSTCW<'a> {
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
pub struct _CH11USEBURSTCW<'a> {
    w: &'a mut W,
}
impl<'a> _CH11USEBURSTCW<'a> {
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
    #[doc = "Bit 0 - Channel 0 Useburst Clear"]
    #[inline]
    pub fn ch0useburstc(&mut self) -> _CH0USEBURSTCW {
        _CH0USEBURSTCW { w: self }
    }
    #[doc = "Bit 1 - Channel 1 Useburst Clear"]
    #[inline]
    pub fn ch1useburstc(&mut self) -> _CH1USEBURSTCW {
        _CH1USEBURSTCW { w: self }
    }
    #[doc = "Bit 2 - Channel 2 Useburst Clear"]
    #[inline]
    pub fn ch2useburstc(&mut self) -> _CH2USEBURSTCW {
        _CH2USEBURSTCW { w: self }
    }
    #[doc = "Bit 3 - Channel 3 Useburst Clear"]
    #[inline]
    pub fn ch3useburstc(&mut self) -> _CH3USEBURSTCW {
        _CH3USEBURSTCW { w: self }
    }
    #[doc = "Bit 4 - Channel 4 Useburst Clear"]
    #[inline]
    pub fn ch4useburstc(&mut self) -> _CH4USEBURSTCW {
        _CH4USEBURSTCW { w: self }
    }
    #[doc = "Bit 5 - Channel 5 Useburst Clear"]
    #[inline]
    pub fn ch5useburstc(&mut self) -> _CH5USEBURSTCW {
        _CH5USEBURSTCW { w: self }
    }
    #[doc = "Bit 6 - Channel 6 Useburst Clear"]
    #[inline]
    pub fn ch6useburstc(&mut self) -> _CH6USEBURSTCW {
        _CH6USEBURSTCW { w: self }
    }
    #[doc = "Bit 7 - Channel 7 Useburst Clear"]
    #[inline]
    pub fn ch7useburstc(&mut self) -> _CH7USEBURSTCW {
        _CH7USEBURSTCW { w: self }
    }
    #[doc = "Bit 8 - Channel 8 Useburst Clear"]
    #[inline]
    pub fn ch08useburstc(&mut self) -> _CH08USEBURSTCW {
        _CH08USEBURSTCW { w: self }
    }
    #[doc = "Bit 9 - Channel 9 Useburst Clear"]
    #[inline]
    pub fn ch9useburstc(&mut self) -> _CH9USEBURSTCW {
        _CH9USEBURSTCW { w: self }
    }
    #[doc = "Bit 10 - Channel 10 Useburst Clear"]
    #[inline]
    pub fn ch10useburstc(&mut self) -> _CH10USEBURSTCW {
        _CH10USEBURSTCW { w: self }
    }
    #[doc = "Bit 11 - Channel 11 Useburst Clear"]
    #[inline]
    pub fn ch11useburstc(&mut self) -> _CH11USEBURSTCW {
        _CH11USEBURSTCW { w: self }
    }
}

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
#[doc = "Possible values of the field `MODE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MODER {
    #[doc = "The BURTC is disabled."]
    DISABLE,
    #[doc = "The BURTC is in normal operating mode, operating in EM0-EM2. Oscillators must be enabled in CMU for use."]
    EM2EN,
    #[doc = "The BURTC is enabled in EM0-EM3. Will prevent CMU from disabling used oscillators all the way down to EM3."]
    EM3EN,
    #[doc = "The BURTC is enabled in EM0-EM4. Will prevent CMU from disabling used oscillators all the way down to EM4."]
    EM4EN,
}
impl MODER {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            MODER::DISABLE => 0,
            MODER::EM2EN => 0x01,
            MODER::EM3EN => 0x02,
            MODER::EM4EN => 0x03,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> MODER {
        match value {
            0 => MODER::DISABLE,
            1 => MODER::EM2EN,
            2 => MODER::EM3EN,
            3 => MODER::EM4EN,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline]
    pub fn is_disable(&self) -> bool {
        *self == MODER::DISABLE
    }
    #[doc = "Checks if the value of the field is `EM2EN`"]
    #[inline]
    pub fn is_em2en(&self) -> bool {
        *self == MODER::EM2EN
    }
    #[doc = "Checks if the value of the field is `EM3EN`"]
    #[inline]
    pub fn is_em3en(&self) -> bool {
        *self == MODER::EM3EN
    }
    #[doc = "Checks if the value of the field is `EM4EN`"]
    #[inline]
    pub fn is_em4en(&self) -> bool {
        *self == MODER::EM4EN
    }
}
#[doc = r" Value of the field"]
pub struct DEBUGRUNR {
    bits: bool,
}
impl DEBUGRUNR {
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
pub struct RSTENR {
    bits: bool,
}
impl RSTENR {
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
pub struct COMP0TOPR {
    bits: bool,
}
impl COMP0TOPR {
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
#[doc = "Possible values of the field `LPCOMP`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LPCOMPR {
    #[doc = "Do not ignore any bits for compare match evaluation."]
    IGN0LSB,
    #[doc = "The LSB of the counter is ignored for compare match evaluation."]
    IGN1LSB,
    #[doc = "The two LSBs of the counter are ignored for compare match evaluation."]
    IGN2LSB,
    #[doc = "The three LSBs of the counter are ignored for compare match evaluation."]
    IGN3LSB,
    #[doc = "The four LSBs of the counter are ignored for compare match evaluation."]
    IGN4LSB,
    #[doc = "The five LSBs of the counter are ignored for compare match evaluation."]
    IGN5LSB,
    #[doc = "The six LSBs of the counter are ignored for compare match evaluation."]
    IGN6LSB,
    #[doc = "The seven LSBs of the counter are ignored for compare match evaluation."]
    IGN7LSB,
}
impl LPCOMPR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            LPCOMPR::IGN0LSB => 0,
            LPCOMPR::IGN1LSB => 0x01,
            LPCOMPR::IGN2LSB => 0x02,
            LPCOMPR::IGN3LSB => 0x03,
            LPCOMPR::IGN4LSB => 0x04,
            LPCOMPR::IGN5LSB => 0x05,
            LPCOMPR::IGN6LSB => 0x06,
            LPCOMPR::IGN7LSB => 0x07,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> LPCOMPR {
        match value {
            0 => LPCOMPR::IGN0LSB,
            1 => LPCOMPR::IGN1LSB,
            2 => LPCOMPR::IGN2LSB,
            3 => LPCOMPR::IGN3LSB,
            4 => LPCOMPR::IGN4LSB,
            5 => LPCOMPR::IGN5LSB,
            6 => LPCOMPR::IGN6LSB,
            7 => LPCOMPR::IGN7LSB,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `IGN0LSB`"]
    #[inline]
    pub fn is_ign0lsb(&self) -> bool {
        *self == LPCOMPR::IGN0LSB
    }
    #[doc = "Checks if the value of the field is `IGN1LSB`"]
    #[inline]
    pub fn is_ign1lsb(&self) -> bool {
        *self == LPCOMPR::IGN1LSB
    }
    #[doc = "Checks if the value of the field is `IGN2LSB`"]
    #[inline]
    pub fn is_ign2lsb(&self) -> bool {
        *self == LPCOMPR::IGN2LSB
    }
    #[doc = "Checks if the value of the field is `IGN3LSB`"]
    #[inline]
    pub fn is_ign3lsb(&self) -> bool {
        *self == LPCOMPR::IGN3LSB
    }
    #[doc = "Checks if the value of the field is `IGN4LSB`"]
    #[inline]
    pub fn is_ign4lsb(&self) -> bool {
        *self == LPCOMPR::IGN4LSB
    }
    #[doc = "Checks if the value of the field is `IGN5LSB`"]
    #[inline]
    pub fn is_ign5lsb(&self) -> bool {
        *self == LPCOMPR::IGN5LSB
    }
    #[doc = "Checks if the value of the field is `IGN6LSB`"]
    #[inline]
    pub fn is_ign6lsb(&self) -> bool {
        *self == LPCOMPR::IGN6LSB
    }
    #[doc = "Checks if the value of the field is `IGN7LSB`"]
    #[inline]
    pub fn is_ign7lsb(&self) -> bool {
        *self == LPCOMPR::IGN7LSB
    }
}
#[doc = "Possible values of the field `PRESC`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PRESCR {
    #[doc = "No prescaling."]
    DIV1,
    #[doc = "Prescaling factor of 2"]
    DIV2,
    #[doc = "Prescaling factor of 4"]
    DIV4,
    #[doc = "Prescaling factor of 8"]
    DIV8,
    #[doc = "Prescaling factor of 16"]
    DIV16,
    #[doc = "Prescaling factor of 32"]
    DIV32,
    #[doc = "Prescaling factor of 64"]
    DIV64,
    #[doc = "Prescaling factor of 128"]
    DIV128,
}
impl PRESCR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            PRESCR::DIV1 => 0,
            PRESCR::DIV2 => 0x01,
            PRESCR::DIV4 => 0x02,
            PRESCR::DIV8 => 0x03,
            PRESCR::DIV16 => 0x04,
            PRESCR::DIV32 => 0x05,
            PRESCR::DIV64 => 0x06,
            PRESCR::DIV128 => 0x07,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> PRESCR {
        match value {
            0 => PRESCR::DIV1,
            1 => PRESCR::DIV2,
            2 => PRESCR::DIV4,
            3 => PRESCR::DIV8,
            4 => PRESCR::DIV16,
            5 => PRESCR::DIV32,
            6 => PRESCR::DIV64,
            7 => PRESCR::DIV128,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `DIV1`"]
    #[inline]
    pub fn is_div1(&self) -> bool {
        *self == PRESCR::DIV1
    }
    #[doc = "Checks if the value of the field is `DIV2`"]
    #[inline]
    pub fn is_div2(&self) -> bool {
        *self == PRESCR::DIV2
    }
    #[doc = "Checks if the value of the field is `DIV4`"]
    #[inline]
    pub fn is_div4(&self) -> bool {
        *self == PRESCR::DIV4
    }
    #[doc = "Checks if the value of the field is `DIV8`"]
    #[inline]
    pub fn is_div8(&self) -> bool {
        *self == PRESCR::DIV8
    }
    #[doc = "Checks if the value of the field is `DIV16`"]
    #[inline]
    pub fn is_div16(&self) -> bool {
        *self == PRESCR::DIV16
    }
    #[doc = "Checks if the value of the field is `DIV32`"]
    #[inline]
    pub fn is_div32(&self) -> bool {
        *self == PRESCR::DIV32
    }
    #[doc = "Checks if the value of the field is `DIV64`"]
    #[inline]
    pub fn is_div64(&self) -> bool {
        *self == PRESCR::DIV64
    }
    #[doc = "Checks if the value of the field is `DIV128`"]
    #[inline]
    pub fn is_div128(&self) -> bool {
        *self == PRESCR::DIV128
    }
}
#[doc = "Possible values of the field `CLKSEL`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CLKSELR {
    #[doc = "No clock source selected for BURTC."]
    NONE,
    #[doc = "LFRCO selected as BURTC clock source."]
    LFRCO,
    #[doc = "LFXO selected as BURTC clock source."]
    LFXO,
    #[doc = "ULFRCO selected as BURTC clock source."]
    ULFRCO,
}
impl CLKSELR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            CLKSELR::NONE => 0,
            CLKSELR::LFRCO => 0x01,
            CLKSELR::LFXO => 0x02,
            CLKSELR::ULFRCO => 0x03,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> CLKSELR {
        match value {
            0 => CLKSELR::NONE,
            1 => CLKSELR::LFRCO,
            2 => CLKSELR::LFXO,
            3 => CLKSELR::ULFRCO,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `NONE`"]
    #[inline]
    pub fn is_none(&self) -> bool {
        *self == CLKSELR::NONE
    }
    #[doc = "Checks if the value of the field is `LFRCO`"]
    #[inline]
    pub fn is_lfrco(&self) -> bool {
        *self == CLKSELR::LFRCO
    }
    #[doc = "Checks if the value of the field is `LFXO`"]
    #[inline]
    pub fn is_lfxo(&self) -> bool {
        *self == CLKSELR::LFXO
    }
    #[doc = "Checks if the value of the field is `ULFRCO`"]
    #[inline]
    pub fn is_ulfrco(&self) -> bool {
        *self == CLKSELR::ULFRCO
    }
}
#[doc = r" Value of the field"]
pub struct BUMODETSENR {
    bits: bool,
}
impl BUMODETSENR {
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
#[doc = "Values that can be written to the field `MODE`"]
pub enum MODEW {
    #[doc = "The BURTC is disabled."]
    DISABLE,
    #[doc = "The BURTC is in normal operating mode, operating in EM0-EM2. Oscillators must be enabled in CMU for use."]
    EM2EN,
    #[doc = "The BURTC is enabled in EM0-EM3. Will prevent CMU from disabling used oscillators all the way down to EM3."]
    EM3EN,
    #[doc = "The BURTC is enabled in EM0-EM4. Will prevent CMU from disabling used oscillators all the way down to EM4."]
    EM4EN,
}
impl MODEW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            MODEW::DISABLE => 0,
            MODEW::EM2EN => 1,
            MODEW::EM3EN => 2,
            MODEW::EM4EN => 3,
        }
    }
}
#[doc = r" Proxy"]
pub struct _MODEW<'a> {
    w: &'a mut W,
}
impl<'a> _MODEW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: MODEW) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "The BURTC is disabled."]
    #[inline]
    pub fn disable(self) -> &'a mut W {
        self.variant(MODEW::DISABLE)
    }
    #[doc = "The BURTC is in normal operating mode, operating in EM0-EM2. Oscillators must be enabled in CMU for use."]
    #[inline]
    pub fn em2en(self) -> &'a mut W {
        self.variant(MODEW::EM2EN)
    }
    #[doc = "The BURTC is enabled in EM0-EM3. Will prevent CMU from disabling used oscillators all the way down to EM3."]
    #[inline]
    pub fn em3en(self) -> &'a mut W {
        self.variant(MODEW::EM3EN)
    }
    #[doc = "The BURTC is enabled in EM0-EM4. Will prevent CMU from disabling used oscillators all the way down to EM4."]
    #[inline]
    pub fn em4en(self) -> &'a mut W {
        self.variant(MODEW::EM4EN)
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
pub struct _DEBUGRUNW<'a> {
    w: &'a mut W,
}
impl<'a> _DEBUGRUNW<'a> {
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
pub struct _RSTENW<'a> {
    w: &'a mut W,
}
impl<'a> _RSTENW<'a> {
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
pub struct _COMP0TOPW<'a> {
    w: &'a mut W,
}
impl<'a> _COMP0TOPW<'a> {
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
#[doc = "Values that can be written to the field `LPCOMP`"]
pub enum LPCOMPW {
    #[doc = "Do not ignore any bits for compare match evaluation."]
    IGN0LSB,
    #[doc = "The LSB of the counter is ignored for compare match evaluation."]
    IGN1LSB,
    #[doc = "The two LSBs of the counter are ignored for compare match evaluation."]
    IGN2LSB,
    #[doc = "The three LSBs of the counter are ignored for compare match evaluation."]
    IGN3LSB,
    #[doc = "The four LSBs of the counter are ignored for compare match evaluation."]
    IGN4LSB,
    #[doc = "The five LSBs of the counter are ignored for compare match evaluation."]
    IGN5LSB,
    #[doc = "The six LSBs of the counter are ignored for compare match evaluation."]
    IGN6LSB,
    #[doc = "The seven LSBs of the counter are ignored for compare match evaluation."]
    IGN7LSB,
}
impl LPCOMPW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            LPCOMPW::IGN0LSB => 0,
            LPCOMPW::IGN1LSB => 1,
            LPCOMPW::IGN2LSB => 2,
            LPCOMPW::IGN3LSB => 3,
            LPCOMPW::IGN4LSB => 4,
            LPCOMPW::IGN5LSB => 5,
            LPCOMPW::IGN6LSB => 6,
            LPCOMPW::IGN7LSB => 7,
        }
    }
}
#[doc = r" Proxy"]
pub struct _LPCOMPW<'a> {
    w: &'a mut W,
}
impl<'a> _LPCOMPW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: LPCOMPW) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "Do not ignore any bits for compare match evaluation."]
    #[inline]
    pub fn ign0lsb(self) -> &'a mut W {
        self.variant(LPCOMPW::IGN0LSB)
    }
    #[doc = "The LSB of the counter is ignored for compare match evaluation."]
    #[inline]
    pub fn ign1lsb(self) -> &'a mut W {
        self.variant(LPCOMPW::IGN1LSB)
    }
    #[doc = "The two LSBs of the counter are ignored for compare match evaluation."]
    #[inline]
    pub fn ign2lsb(self) -> &'a mut W {
        self.variant(LPCOMPW::IGN2LSB)
    }
    #[doc = "The three LSBs of the counter are ignored for compare match evaluation."]
    #[inline]
    pub fn ign3lsb(self) -> &'a mut W {
        self.variant(LPCOMPW::IGN3LSB)
    }
    #[doc = "The four LSBs of the counter are ignored for compare match evaluation."]
    #[inline]
    pub fn ign4lsb(self) -> &'a mut W {
        self.variant(LPCOMPW::IGN4LSB)
    }
    #[doc = "The five LSBs of the counter are ignored for compare match evaluation."]
    #[inline]
    pub fn ign5lsb(self) -> &'a mut W {
        self.variant(LPCOMPW::IGN5LSB)
    }
    #[doc = "The six LSBs of the counter are ignored for compare match evaluation."]
    #[inline]
    pub fn ign6lsb(self) -> &'a mut W {
        self.variant(LPCOMPW::IGN6LSB)
    }
    #[doc = "The seven LSBs of the counter are ignored for compare match evaluation."]
    #[inline]
    pub fn ign7lsb(self) -> &'a mut W {
        self.variant(LPCOMPW::IGN7LSB)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 0x07;
        const OFFSET: u8 = 5;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `PRESC`"]
pub enum PRESCW {
    #[doc = "No prescaling."]
    DIV1,
    #[doc = "Prescaling factor of 2"]
    DIV2,
    #[doc = "Prescaling factor of 4"]
    DIV4,
    #[doc = "Prescaling factor of 8"]
    DIV8,
    #[doc = "Prescaling factor of 16"]
    DIV16,
    #[doc = "Prescaling factor of 32"]
    DIV32,
    #[doc = "Prescaling factor of 64"]
    DIV64,
    #[doc = "Prescaling factor of 128"]
    DIV128,
}
impl PRESCW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            PRESCW::DIV1 => 0,
            PRESCW::DIV2 => 1,
            PRESCW::DIV4 => 2,
            PRESCW::DIV8 => 3,
            PRESCW::DIV16 => 4,
            PRESCW::DIV32 => 5,
            PRESCW::DIV64 => 6,
            PRESCW::DIV128 => 7,
        }
    }
}
#[doc = r" Proxy"]
pub struct _PRESCW<'a> {
    w: &'a mut W,
}
impl<'a> _PRESCW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: PRESCW) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "No prescaling."]
    #[inline]
    pub fn div1(self) -> &'a mut W {
        self.variant(PRESCW::DIV1)
    }
    #[doc = "Prescaling factor of 2"]
    #[inline]
    pub fn div2(self) -> &'a mut W {
        self.variant(PRESCW::DIV2)
    }
    #[doc = "Prescaling factor of 4"]
    #[inline]
    pub fn div4(self) -> &'a mut W {
        self.variant(PRESCW::DIV4)
    }
    #[doc = "Prescaling factor of 8"]
    #[inline]
    pub fn div8(self) -> &'a mut W {
        self.variant(PRESCW::DIV8)
    }
    #[doc = "Prescaling factor of 16"]
    #[inline]
    pub fn div16(self) -> &'a mut W {
        self.variant(PRESCW::DIV16)
    }
    #[doc = "Prescaling factor of 32"]
    #[inline]
    pub fn div32(self) -> &'a mut W {
        self.variant(PRESCW::DIV32)
    }
    #[doc = "Prescaling factor of 64"]
    #[inline]
    pub fn div64(self) -> &'a mut W {
        self.variant(PRESCW::DIV64)
    }
    #[doc = "Prescaling factor of 128"]
    #[inline]
    pub fn div128(self) -> &'a mut W {
        self.variant(PRESCW::DIV128)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 0x07;
        const OFFSET: u8 = 8;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `CLKSEL`"]
pub enum CLKSELW {
    #[doc = "No clock source selected for BURTC."]
    NONE,
    #[doc = "LFRCO selected as BURTC clock source."]
    LFRCO,
    #[doc = "LFXO selected as BURTC clock source."]
    LFXO,
    #[doc = "ULFRCO selected as BURTC clock source."]
    ULFRCO,
}
impl CLKSELW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            CLKSELW::NONE => 0,
            CLKSELW::LFRCO => 1,
            CLKSELW::LFXO => 2,
            CLKSELW::ULFRCO => 3,
        }
    }
}
#[doc = r" Proxy"]
pub struct _CLKSELW<'a> {
    w: &'a mut W,
}
impl<'a> _CLKSELW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: CLKSELW) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "No clock source selected for BURTC."]
    #[inline]
    pub fn none(self) -> &'a mut W {
        self.variant(CLKSELW::NONE)
    }
    #[doc = "LFRCO selected as BURTC clock source."]
    #[inline]
    pub fn lfrco(self) -> &'a mut W {
        self.variant(CLKSELW::LFRCO)
    }
    #[doc = "LFXO selected as BURTC clock source."]
    #[inline]
    pub fn lfxo(self) -> &'a mut W {
        self.variant(CLKSELW::LFXO)
    }
    #[doc = "ULFRCO selected as BURTC clock source."]
    #[inline]
    pub fn ulfrco(self) -> &'a mut W {
        self.variant(CLKSELW::ULFRCO)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 0x03;
        const OFFSET: u8 = 12;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _BUMODETSENW<'a> {
    w: &'a mut W,
}
impl<'a> _BUMODETSENW<'a> {
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
        const OFFSET: u8 = 14;
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
    #[doc = "Bits 0:1 - BURTC Enable"]
    #[inline]
    pub fn mode(&self) -> MODER {
        MODER::_from({
            const MASK: u8 = 0x03;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bit 2 - Debug Mode Run Enable"]
    #[inline]
    pub fn debugrun(&self) -> DEBUGRUNR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 2;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        DEBUGRUNR { bits }
    }
    #[doc = "Bit 3 - Enable BURTC reset"]
    #[inline]
    pub fn rsten(&self) -> RSTENR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 3;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        RSTENR { bits }
    }
    #[doc = "Bit 4 - Compare clear enable"]
    #[inline]
    pub fn comp0top(&self) -> COMP0TOPR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 4;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        COMP0TOPR { bits }
    }
    #[doc = "Bits 5:7 - Low power mode compare configuration"]
    #[inline]
    pub fn lpcomp(&self) -> LPCOMPR {
        LPCOMPR::_from({
            const MASK: u8 = 0x07;
            const OFFSET: u8 = 5;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 8:10 - Select BURTC prescaler factor"]
    #[inline]
    pub fn presc(&self) -> PRESCR {
        PRESCR::_from({
            const MASK: u8 = 0x07;
            const OFFSET: u8 = 8;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 12:13 - Select BURTC clock source"]
    #[inline]
    pub fn clksel(&self) -> CLKSELR {
        CLKSELR::_from({
            const MASK: u8 = 0x03;
            const OFFSET: u8 = 12;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bit 14 - Backup mode timestamp enable"]
    #[inline]
    pub fn bumodetsen(&self) -> BUMODETSENR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 14;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        BUMODETSENR { bits }
    }
}
impl W {
    #[doc = r" Reset value of the register"]
    #[inline]
    pub fn reset_value() -> W {
        W { bits: 0x08 }
    }
    #[doc = r" Writes raw bits to the register"]
    #[inline]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
    #[doc = "Bits 0:1 - BURTC Enable"]
    #[inline]
    pub fn mode(&mut self) -> _MODEW {
        _MODEW { w: self }
    }
    #[doc = "Bit 2 - Debug Mode Run Enable"]
    #[inline]
    pub fn debugrun(&mut self) -> _DEBUGRUNW {
        _DEBUGRUNW { w: self }
    }
    #[doc = "Bit 3 - Enable BURTC reset"]
    #[inline]
    pub fn rsten(&mut self) -> _RSTENW {
        _RSTENW { w: self }
    }
    #[doc = "Bit 4 - Compare clear enable"]
    #[inline]
    pub fn comp0top(&mut self) -> _COMP0TOPW {
        _COMP0TOPW { w: self }
    }
    #[doc = "Bits 5:7 - Low power mode compare configuration"]
    #[inline]
    pub fn lpcomp(&mut self) -> _LPCOMPW {
        _LPCOMPW { w: self }
    }
    #[doc = "Bits 8:10 - Select BURTC prescaler factor"]
    #[inline]
    pub fn presc(&mut self) -> _PRESCW {
        _PRESCW { w: self }
    }
    #[doc = "Bits 12:13 - Select BURTC clock source"]
    #[inline]
    pub fn clksel(&mut self) -> _CLKSELW {
        _CLKSELW { w: self }
    }
    #[doc = "Bit 14 - Backup mode timestamp enable"]
    #[inline]
    pub fn bumodetsen(&mut self) -> _BUMODETSENW {
        _BUMODETSENW { w: self }
    }
}

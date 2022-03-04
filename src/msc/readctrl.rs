#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::READCTRL {
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
    #[doc = "Zero wait-states inserted in fetch or read transfers."]
    WS0,
    #[doc = "One wait-state inserted for each fetch or read transfer. This mode is required for a core frequency above 16 MHz."]
    WS1,
    #[doc = "Zero wait-states inserted with the Suppressed Conditional Branch Target Prefetch (SCBTP) function enabled. SCBTP saves energy by delaying the Cortex' conditional branch target prefetches until the conditional branch instruction is in the execute stage. When the instruction reaches this stage, the evaluation of the branch condition is completed and the core does not perform a speculative prefetch of both the branch target address and the next sequential address. With the SCBTP function enabled, one instruction fetch is saved for each branch not taken, with a negligible performance penalty."]
    WS0SCBTP,
    #[doc = "One wait-state access with SCBTP enabled."]
    WS1SCBTP,
    #[doc = "Two wait-states inserted for each fetch or read transfer. This mode is required for a core frequency above 32 MHz."]
    WS2,
    #[doc = "Two wait-state access with SCBTP enabled."]
    WS2SCBTP,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl MODER {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            MODER::WS0 => 0,
            MODER::WS1 => 0x01,
            MODER::WS0SCBTP => 0x02,
            MODER::WS1SCBTP => 0x03,
            MODER::WS2 => 0x04,
            MODER::WS2SCBTP => 0x05,
            MODER::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> MODER {
        match value {
            0 => MODER::WS0,
            1 => MODER::WS1,
            2 => MODER::WS0SCBTP,
            3 => MODER::WS1SCBTP,
            4 => MODER::WS2,
            5 => MODER::WS2SCBTP,
            i => MODER::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `WS0`"]
    #[inline]
    pub fn is_ws0(&self) -> bool {
        *self == MODER::WS0
    }
    #[doc = "Checks if the value of the field is `WS1`"]
    #[inline]
    pub fn is_ws1(&self) -> bool {
        *self == MODER::WS1
    }
    #[doc = "Checks if the value of the field is `WS0SCBTP`"]
    #[inline]
    pub fn is_ws0scbtp(&self) -> bool {
        *self == MODER::WS0SCBTP
    }
    #[doc = "Checks if the value of the field is `WS1SCBTP`"]
    #[inline]
    pub fn is_ws1scbtp(&self) -> bool {
        *self == MODER::WS1SCBTP
    }
    #[doc = "Checks if the value of the field is `WS2`"]
    #[inline]
    pub fn is_ws2(&self) -> bool {
        *self == MODER::WS2
    }
    #[doc = "Checks if the value of the field is `WS2SCBTP`"]
    #[inline]
    pub fn is_ws2scbtp(&self) -> bool {
        *self == MODER::WS2SCBTP
    }
}
#[doc = r" Value of the field"]
pub struct IFCDISR {
    bits: bool,
}
impl IFCDISR {
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
pub struct AIDISR {
    bits: bool,
}
impl AIDISR {
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
pub struct ICCDISR {
    bits: bool,
}
impl ICCDISR {
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
pub struct EBICDISR {
    bits: bool,
}
impl EBICDISR {
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
pub struct RAMCENR {
    bits: bool,
}
impl RAMCENR {
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
pub struct PREFETCHR {
    bits: bool,
}
impl PREFETCHR {
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
#[doc = "Possible values of the field `BUSSTRATEGY`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BUSSTRATEGYR {
    #[doc = "\"\""]
    CPU,
    #[doc = "\"\""]
    DMA,
    #[doc = "\"\""]
    DMAEM1,
    #[doc = "\"\""]
    NONE,
}
impl BUSSTRATEGYR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            BUSSTRATEGYR::CPU => 0,
            BUSSTRATEGYR::DMA => 0x01,
            BUSSTRATEGYR::DMAEM1 => 0x02,
            BUSSTRATEGYR::NONE => 0x03,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> BUSSTRATEGYR {
        match value {
            0 => BUSSTRATEGYR::CPU,
            1 => BUSSTRATEGYR::DMA,
            2 => BUSSTRATEGYR::DMAEM1,
            3 => BUSSTRATEGYR::NONE,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `CPU`"]
    #[inline]
    pub fn is_cpu(&self) -> bool {
        *self == BUSSTRATEGYR::CPU
    }
    #[doc = "Checks if the value of the field is `DMA`"]
    #[inline]
    pub fn is_dma(&self) -> bool {
        *self == BUSSTRATEGYR::DMA
    }
    #[doc = "Checks if the value of the field is `DMAEM1`"]
    #[inline]
    pub fn is_dmaem1(&self) -> bool {
        *self == BUSSTRATEGYR::DMAEM1
    }
    #[doc = "Checks if the value of the field is `NONE`"]
    #[inline]
    pub fn is_none(&self) -> bool {
        *self == BUSSTRATEGYR::NONE
    }
}
#[doc = "Values that can be written to the field `MODE`"]
pub enum MODEW {
    #[doc = "Zero wait-states inserted in fetch or read transfers."]
    WS0,
    #[doc = "One wait-state inserted for each fetch or read transfer. This mode is required for a core frequency above 16 MHz."]
    WS1,
    #[doc = "Zero wait-states inserted with the Suppressed Conditional Branch Target Prefetch (SCBTP) function enabled. SCBTP saves energy by delaying the Cortex' conditional branch target prefetches until the conditional branch instruction is in the execute stage. When the instruction reaches this stage, the evaluation of the branch condition is completed and the core does not perform a speculative prefetch of both the branch target address and the next sequential address. With the SCBTP function enabled, one instruction fetch is saved for each branch not taken, with a negligible performance penalty."]
    WS0SCBTP,
    #[doc = "One wait-state access with SCBTP enabled."]
    WS1SCBTP,
    #[doc = "Two wait-states inserted for each fetch or read transfer. This mode is required for a core frequency above 32 MHz."]
    WS2,
    #[doc = "Two wait-state access with SCBTP enabled."]
    WS2SCBTP,
}
impl MODEW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            MODEW::WS0 => 0,
            MODEW::WS1 => 1,
            MODEW::WS0SCBTP => 2,
            MODEW::WS1SCBTP => 3,
            MODEW::WS2 => 4,
            MODEW::WS2SCBTP => 5,
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
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "Zero wait-states inserted in fetch or read transfers."]
    #[inline]
    pub fn ws0(self) -> &'a mut W {
        self.variant(MODEW::WS0)
    }
    #[doc = "One wait-state inserted for each fetch or read transfer. This mode is required for a core frequency above 16 MHz."]
    #[inline]
    pub fn ws1(self) -> &'a mut W {
        self.variant(MODEW::WS1)
    }
    #[doc = "Zero wait-states inserted with the Suppressed Conditional Branch Target Prefetch (SCBTP) function enabled. SCBTP saves energy by delaying the Cortex' conditional branch target prefetches until the conditional branch instruction is in the execute stage. When the instruction reaches this stage, the evaluation of the branch condition is completed and the core does not perform a speculative prefetch of both the branch target address and the next sequential address. With the SCBTP function enabled, one instruction fetch is saved for each branch not taken, with a negligible performance penalty."]
    #[inline]
    pub fn ws0scbtp(self) -> &'a mut W {
        self.variant(MODEW::WS0SCBTP)
    }
    #[doc = "One wait-state access with SCBTP enabled."]
    #[inline]
    pub fn ws1scbtp(self) -> &'a mut W {
        self.variant(MODEW::WS1SCBTP)
    }
    #[doc = "Two wait-states inserted for each fetch or read transfer. This mode is required for a core frequency above 32 MHz."]
    #[inline]
    pub fn ws2(self) -> &'a mut W {
        self.variant(MODEW::WS2)
    }
    #[doc = "Two wait-state access with SCBTP enabled."]
    #[inline]
    pub fn ws2scbtp(self) -> &'a mut W {
        self.variant(MODEW::WS2SCBTP)
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
#[doc = r" Proxy"]
pub struct _IFCDISW<'a> {
    w: &'a mut W,
}
impl<'a> _IFCDISW<'a> {
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
pub struct _AIDISW<'a> {
    w: &'a mut W,
}
impl<'a> _AIDISW<'a> {
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
pub struct _ICCDISW<'a> {
    w: &'a mut W,
}
impl<'a> _ICCDISW<'a> {
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
pub struct _EBICDISW<'a> {
    w: &'a mut W,
}
impl<'a> _EBICDISW<'a> {
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
pub struct _RAMCENW<'a> {
    w: &'a mut W,
}
impl<'a> _RAMCENW<'a> {
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
pub struct _PREFETCHW<'a> {
    w: &'a mut W,
}
impl<'a> _PREFETCHW<'a> {
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
#[doc = "Values that can be written to the field `BUSSTRATEGY`"]
pub enum BUSSTRATEGYW {
    #[doc = "\"\""]
    CPU,
    #[doc = "\"\""]
    DMA,
    #[doc = "\"\""]
    DMAEM1,
    #[doc = "\"\""]
    NONE,
}
impl BUSSTRATEGYW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            BUSSTRATEGYW::CPU => 0,
            BUSSTRATEGYW::DMA => 1,
            BUSSTRATEGYW::DMAEM1 => 2,
            BUSSTRATEGYW::NONE => 3,
        }
    }
}
#[doc = r" Proxy"]
pub struct _BUSSTRATEGYW<'a> {
    w: &'a mut W,
}
impl<'a> _BUSSTRATEGYW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: BUSSTRATEGYW) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "\"\""]
    #[inline]
    pub fn cpu(self) -> &'a mut W {
        self.variant(BUSSTRATEGYW::CPU)
    }
    #[doc = "\"\""]
    #[inline]
    pub fn dma(self) -> &'a mut W {
        self.variant(BUSSTRATEGYW::DMA)
    }
    #[doc = "\"\""]
    #[inline]
    pub fn dmaem1(self) -> &'a mut W {
        self.variant(BUSSTRATEGYW::DMAEM1)
    }
    #[doc = "\"\""]
    #[inline]
    pub fn none(self) -> &'a mut W {
        self.variant(BUSSTRATEGYW::NONE)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 0x03;
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
    #[doc = "Bits 0:2 - Read Mode"]
    #[inline]
    pub fn mode(&self) -> MODER {
        MODER::_from({
            const MASK: u8 = 0x07;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bit 3 - Internal Flash Cache Disable"]
    #[inline]
    pub fn ifcdis(&self) -> IFCDISR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 3;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        IFCDISR { bits }
    }
    #[doc = "Bit 4 - Automatic Invalidate Disable"]
    #[inline]
    pub fn aidis(&self) -> AIDISR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 4;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        AIDISR { bits }
    }
    #[doc = "Bit 5 - Interrupt Context Cache Disable"]
    #[inline]
    pub fn iccdis(&self) -> ICCDISR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 5;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        ICCDISR { bits }
    }
    #[doc = "Bit 6 - External Bus Interface Cache Disable"]
    #[inline]
    pub fn ebicdis(&self) -> EBICDISR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 6;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        EBICDISR { bits }
    }
    #[doc = "Bit 7 - RAM Cache Enable"]
    #[inline]
    pub fn ramcen(&self) -> RAMCENR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 7;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        RAMCENR { bits }
    }
    #[doc = "Bit 8 - Prefetch Mode"]
    #[inline]
    pub fn prefetch(&self) -> PREFETCHR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 8;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        PREFETCHR { bits }
    }
    #[doc = "Bits 16:17 - Strategy for bus matrix"]
    #[inline]
    pub fn busstrategy(&self) -> BUSSTRATEGYR {
        BUSSTRATEGYR::_from({
            const MASK: u8 = 0x03;
            const OFFSET: u8 = 16;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
}
impl W {
    #[doc = r" Reset value of the register"]
    #[inline]
    pub fn reset_value() -> W {
        W { bits: 0x01 }
    }
    #[doc = r" Writes raw bits to the register"]
    #[inline]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
    #[doc = "Bits 0:2 - Read Mode"]
    #[inline]
    pub fn mode(&mut self) -> _MODEW {
        _MODEW { w: self }
    }
    #[doc = "Bit 3 - Internal Flash Cache Disable"]
    #[inline]
    pub fn ifcdis(&mut self) -> _IFCDISW {
        _IFCDISW { w: self }
    }
    #[doc = "Bit 4 - Automatic Invalidate Disable"]
    #[inline]
    pub fn aidis(&mut self) -> _AIDISW {
        _AIDISW { w: self }
    }
    #[doc = "Bit 5 - Interrupt Context Cache Disable"]
    #[inline]
    pub fn iccdis(&mut self) -> _ICCDISW {
        _ICCDISW { w: self }
    }
    #[doc = "Bit 6 - External Bus Interface Cache Disable"]
    #[inline]
    pub fn ebicdis(&mut self) -> _EBICDISW {
        _EBICDISW { w: self }
    }
    #[doc = "Bit 7 - RAM Cache Enable"]
    #[inline]
    pub fn ramcen(&mut self) -> _RAMCENW {
        _RAMCENW { w: self }
    }
    #[doc = "Bit 8 - Prefetch Mode"]
    #[inline]
    pub fn prefetch(&mut self) -> _PREFETCHW {
        _PREFETCHW { w: self }
    }
    #[doc = "Bits 16:17 - Strategy for bus matrix"]
    #[inline]
    pub fn busstrategy(&mut self) -> _BUSSTRATEGYW {
        _BUSSTRATEGYW { w: self }
    }
}

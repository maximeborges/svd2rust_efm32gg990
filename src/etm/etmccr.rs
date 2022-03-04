#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
impl super::ETMCCR {
    #[doc = r" Reads the contents of the register"]
    #[inline]
    pub fn read(&self) -> R {
        R {
            bits: self.register.get(),
        }
    }
}
#[doc = r" Value of the field"]
pub struct ADRCMPPAIRR {
    bits: u8,
}
impl ADRCMPPAIRR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct DATACMPNUMR {
    bits: u8,
}
impl DATACMPNUMR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct MMDECCNTR {
    bits: u8,
}
impl MMDECCNTR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct COUNTNUMR {
    bits: u8,
}
impl COUNTNUMR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct SEQPRESR {
    bits: bool,
}
impl SEQPRESR {
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
#[doc = "Possible values of the field `EXTINPNUM`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum EXTINPNUMR {
    #[doc = "Zero inputs presents"]
    ZERO,
    #[doc = "One inputs presents"]
    ONE,
    #[doc = "Two inputs presents"]
    TWO,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl EXTINPNUMR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            EXTINPNUMR::ZERO => 0,
            EXTINPNUMR::ONE => 0x01,
            EXTINPNUMR::TWO => 0x02,
            EXTINPNUMR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> EXTINPNUMR {
        match value {
            0 => EXTINPNUMR::ZERO,
            1 => EXTINPNUMR::ONE,
            2 => EXTINPNUMR::TWO,
            i => EXTINPNUMR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `ZERO`"]
    #[inline]
    pub fn is_zero(&self) -> bool {
        *self == EXTINPNUMR::ZERO
    }
    #[doc = "Checks if the value of the field is `ONE`"]
    #[inline]
    pub fn is_one(&self) -> bool {
        *self == EXTINPNUMR::ONE
    }
    #[doc = "Checks if the value of the field is `TWO`"]
    #[inline]
    pub fn is_two(&self) -> bool {
        *self == EXTINPNUMR::TWO
    }
}
#[doc = r" Value of the field"]
pub struct EXTOUTNUMR {
    bits: u8,
}
impl EXTOUTNUMR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct FIFOFULLPRESR {
    bits: bool,
}
impl FIFOFULLPRESR {
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
pub struct IDCOMPNUMR {
    bits: u8,
}
impl IDCOMPNUMR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct TRACESSR {
    bits: bool,
}
impl TRACESSR {
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
pub struct MMACCESSR {
    bits: bool,
}
impl MMACCESSR {
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
pub struct ETMIDR {
    bits: bool,
}
impl ETMIDR {
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
impl R {
    #[doc = r" Value of the register as raw bits"]
    #[inline]
    pub fn bits(&self) -> u32 {
        self.bits
    }
    #[doc = "Bits 0:3 - Number of Address Comparator Pairs"]
    #[inline]
    pub fn adrcmppair(&self) -> ADRCMPPAIRR {
        let bits = {
            const MASK: u8 = 0x0f;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        ADRCMPPAIRR { bits }
    }
    #[doc = "Bits 4:7 - Number of Data Value Comparators"]
    #[inline]
    pub fn datacmpnum(&self) -> DATACMPNUMR {
        let bits = {
            const MASK: u8 = 0x0f;
            const OFFSET: u8 = 4;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        DATACMPNUMR { bits }
    }
    #[doc = "Bits 8:12 - Number of Memeory Map Decoders"]
    #[inline]
    pub fn mmdeccnt(&self) -> MMDECCNTR {
        let bits = {
            const MASK: u8 = 0x1f;
            const OFFSET: u8 = 8;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        MMDECCNTR { bits }
    }
    #[doc = "Bits 13:15 - Number of Counters"]
    #[inline]
    pub fn countnum(&self) -> COUNTNUMR {
        let bits = {
            const MASK: u8 = 0x07;
            const OFFSET: u8 = 13;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        COUNTNUMR { bits }
    }
    #[doc = "Bit 16 - Sequencer Present"]
    #[inline]
    pub fn seqpres(&self) -> SEQPRESR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 16;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        SEQPRESR { bits }
    }
    #[doc = "Bits 17:19 - Number of External Inputs"]
    #[inline]
    pub fn extinpnum(&self) -> EXTINPNUMR {
        EXTINPNUMR::_from({
            const MASK: u8 = 0x07;
            const OFFSET: u8 = 17;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 20:22 - Number of External Output"]
    #[inline]
    pub fn extoutnum(&self) -> EXTOUTNUMR {
        let bits = {
            const MASK: u8 = 0x07;
            const OFFSET: u8 = 20;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        EXTOUTNUMR { bits }
    }
    #[doc = "Bit 23 - FIFIO FULL present"]
    #[inline]
    pub fn fifofullpres(&self) -> FIFOFULLPRESR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 23;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        FIFOFULLPRESR { bits }
    }
    #[doc = "Bits 24:25 - Number of context ID Comparators"]
    #[inline]
    pub fn idcompnum(&self) -> IDCOMPNUMR {
        let bits = {
            const MASK: u8 = 0x03;
            const OFFSET: u8 = 24;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        IDCOMPNUMR { bits }
    }
    #[doc = "Bit 26 - Trace Start/Stop Block Present"]
    #[inline]
    pub fn tracess(&self) -> TRACESSR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 26;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        TRACESSR { bits }
    }
    #[doc = "Bit 27 - Coprocessor and Memeory Access"]
    #[inline]
    pub fn mmaccess(&self) -> MMACCESSR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 27;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        MMACCESSR { bits }
    }
    #[doc = "Bit 31 - ETM ID Register Present"]
    #[inline]
    pub fn etmid(&self) -> ETMIDR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 31;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        ETMIDR { bits }
    }
}

#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
impl super::ETMAUTHSTATUS {
    #[doc = r" Reads the contents of the register"]
    #[inline]
    pub fn read(&self) -> R {
        R {
            bits: self.register.get(),
        }
    }
}
#[doc = r" Value of the field"]
pub struct NONSECINVDBGR {
    bits: u8,
}
impl NONSECINVDBGR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = "Possible values of the field `NONSECNONINVDBG`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum NONSECNONINVDBGR {
    #[doc = "Non-secure non-invasive debug disable"]
    DISABLE,
    #[doc = "Non-secure non-invasive debug enable"]
    ENABLE,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl NONSECNONINVDBGR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            NONSECNONINVDBGR::DISABLE => 0x02,
            NONSECNONINVDBGR::ENABLE => 0x03,
            NONSECNONINVDBGR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> NONSECNONINVDBGR {
        match value {
            2 => NONSECNONINVDBGR::DISABLE,
            3 => NONSECNONINVDBGR::ENABLE,
            i => NONSECNONINVDBGR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline]
    pub fn is_disable(&self) -> bool {
        *self == NONSECNONINVDBGR::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline]
    pub fn is_enable(&self) -> bool {
        *self == NONSECNONINVDBGR::ENABLE
    }
}
#[doc = r" Value of the field"]
pub struct SECINVDBGR {
    bits: u8,
}
impl SECINVDBGR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct SECNONINVDBGR {
    bits: u8,
}
impl SECNONINVDBGR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
impl R {
    #[doc = r" Value of the register as raw bits"]
    #[inline]
    pub fn bits(&self) -> u32 {
        self.bits
    }
    #[doc = "Bits 0:1 - Non-secure invasive Debug Status"]
    #[inline]
    pub fn nonsecinvdbg(&self) -> NONSECINVDBGR {
        let bits = {
            const MASK: u8 = 0x03;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        NONSECINVDBGR { bits }
    }
    #[doc = "Bits 2:3 - Non-secure non-invasive Debug Status"]
    #[inline]
    pub fn nonsecnoninvdbg(&self) -> NONSECNONINVDBGR {
        NONSECNONINVDBGR::_from({
            const MASK: u8 = 0x03;
            const OFFSET: u8 = 2;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 4:5 - Secure invasive Debug Status"]
    #[inline]
    pub fn secinvdbg(&self) -> SECINVDBGR {
        let bits = {
            const MASK: u8 = 0x03;
            const OFFSET: u8 = 4;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        SECINVDBGR { bits }
    }
    #[doc = "Bits 6:7 - Secure non-invasive Debug Status"]
    #[inline]
    pub fn secnoninvdbg(&self) -> SECNONINVDBGR {
        let bits = {
            const MASK: u8 = 0x03;
            const OFFSET: u8 = 6;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        SECNONINVDBGR { bits }
    }
}

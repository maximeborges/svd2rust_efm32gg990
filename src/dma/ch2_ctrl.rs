#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::CH2_CTRL {
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
pub struct SIGSELR {
    bits: u8,
}
impl SIGSELR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = "Possible values of the field `SOURCESEL`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SOURCESELR {
    #[doc = "No source selected"]
    NONE,
    #[doc = "Analog to Digital Converter 0"]
    ADC0,
    #[doc = "Digital to Analog Converter 0"]
    DAC0,
    #[doc = "Universal Synchronous/Asynchronous Receiver/Transmitter 0"]
    USART0,
    #[doc = "Universal Synchronous/Asynchronous Receiver/Transmitter 1"]
    USART1,
    #[doc = "Universal Synchronous/Asynchronous Receiver/Transmitter 2"]
    USART2,
    #[doc = "Low Energy UART 0"]
    LEUART0,
    #[doc = "Low Energy UART 1"]
    LEUART1,
    #[doc = "I2C 0"]
    I2C0,
    #[doc = "I2C 1"]
    I2C1,
    #[doc = "Timer 0"]
    TIMER0,
    #[doc = "Timer 1"]
    TIMER1,
    #[doc = "Timer 2"]
    TIMER2,
    #[doc = "Timer 3"]
    TIMER3,
    #[doc = "Universal Asynchronous Receiver/Transmitter 0"]
    UART0,
    #[doc = "Universal Asynchronous Receiver/Transmitter 1"]
    UART1,
    #[doc = "\"\""]
    MSC,
    #[doc = "Advanced Encryption Standard Accelerator"]
    AES,
    #[doc = "Low Energy Sensor Interface"]
    LESENSE,
    #[doc = "External Bus Interface"]
    EBI,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl SOURCESELR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            SOURCESELR::NONE => 0,
            SOURCESELR::ADC0 => 0x08,
            SOURCESELR::DAC0 => 0x0a,
            SOURCESELR::USART0 => 0x0c,
            SOURCESELR::USART1 => 0x0d,
            SOURCESELR::USART2 => 0x0e,
            SOURCESELR::LEUART0 => 0x10,
            SOURCESELR::LEUART1 => 0x11,
            SOURCESELR::I2C0 => 0x14,
            SOURCESELR::I2C1 => 0x15,
            SOURCESELR::TIMER0 => 0x18,
            SOURCESELR::TIMER1 => 0x19,
            SOURCESELR::TIMER2 => 0x1a,
            SOURCESELR::TIMER3 => 0x1b,
            SOURCESELR::UART0 => 0x2c,
            SOURCESELR::UART1 => 0x2d,
            SOURCESELR::MSC => 0x30,
            SOURCESELR::AES => 0x31,
            SOURCESELR::LESENSE => 0x32,
            SOURCESELR::EBI => 0x33,
            SOURCESELR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> SOURCESELR {
        match value {
            0 => SOURCESELR::NONE,
            8 => SOURCESELR::ADC0,
            10 => SOURCESELR::DAC0,
            12 => SOURCESELR::USART0,
            13 => SOURCESELR::USART1,
            14 => SOURCESELR::USART2,
            16 => SOURCESELR::LEUART0,
            17 => SOURCESELR::LEUART1,
            20 => SOURCESELR::I2C0,
            21 => SOURCESELR::I2C1,
            24 => SOURCESELR::TIMER0,
            25 => SOURCESELR::TIMER1,
            26 => SOURCESELR::TIMER2,
            27 => SOURCESELR::TIMER3,
            44 => SOURCESELR::UART0,
            45 => SOURCESELR::UART1,
            48 => SOURCESELR::MSC,
            49 => SOURCESELR::AES,
            50 => SOURCESELR::LESENSE,
            51 => SOURCESELR::EBI,
            i => SOURCESELR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `NONE`"]
    #[inline]
    pub fn is_none(&self) -> bool {
        *self == SOURCESELR::NONE
    }
    #[doc = "Checks if the value of the field is `ADC0`"]
    #[inline]
    pub fn is_adc0(&self) -> bool {
        *self == SOURCESELR::ADC0
    }
    #[doc = "Checks if the value of the field is `DAC0`"]
    #[inline]
    pub fn is_dac0(&self) -> bool {
        *self == SOURCESELR::DAC0
    }
    #[doc = "Checks if the value of the field is `USART0`"]
    #[inline]
    pub fn is_usart0(&self) -> bool {
        *self == SOURCESELR::USART0
    }
    #[doc = "Checks if the value of the field is `USART1`"]
    #[inline]
    pub fn is_usart1(&self) -> bool {
        *self == SOURCESELR::USART1
    }
    #[doc = "Checks if the value of the field is `USART2`"]
    #[inline]
    pub fn is_usart2(&self) -> bool {
        *self == SOURCESELR::USART2
    }
    #[doc = "Checks if the value of the field is `LEUART0`"]
    #[inline]
    pub fn is_leuart0(&self) -> bool {
        *self == SOURCESELR::LEUART0
    }
    #[doc = "Checks if the value of the field is `LEUART1`"]
    #[inline]
    pub fn is_leuart1(&self) -> bool {
        *self == SOURCESELR::LEUART1
    }
    #[doc = "Checks if the value of the field is `I2C0`"]
    #[inline]
    pub fn is_i2c0(&self) -> bool {
        *self == SOURCESELR::I2C0
    }
    #[doc = "Checks if the value of the field is `I2C1`"]
    #[inline]
    pub fn is_i2c1(&self) -> bool {
        *self == SOURCESELR::I2C1
    }
    #[doc = "Checks if the value of the field is `TIMER0`"]
    #[inline]
    pub fn is_timer0(&self) -> bool {
        *self == SOURCESELR::TIMER0
    }
    #[doc = "Checks if the value of the field is `TIMER1`"]
    #[inline]
    pub fn is_timer1(&self) -> bool {
        *self == SOURCESELR::TIMER1
    }
    #[doc = "Checks if the value of the field is `TIMER2`"]
    #[inline]
    pub fn is_timer2(&self) -> bool {
        *self == SOURCESELR::TIMER2
    }
    #[doc = "Checks if the value of the field is `TIMER3`"]
    #[inline]
    pub fn is_timer3(&self) -> bool {
        *self == SOURCESELR::TIMER3
    }
    #[doc = "Checks if the value of the field is `UART0`"]
    #[inline]
    pub fn is_uart0(&self) -> bool {
        *self == SOURCESELR::UART0
    }
    #[doc = "Checks if the value of the field is `UART1`"]
    #[inline]
    pub fn is_uart1(&self) -> bool {
        *self == SOURCESELR::UART1
    }
    #[doc = "Checks if the value of the field is `MSC`"]
    #[inline]
    pub fn is_msc(&self) -> bool {
        *self == SOURCESELR::MSC
    }
    #[doc = "Checks if the value of the field is `AES`"]
    #[inline]
    pub fn is_aes(&self) -> bool {
        *self == SOURCESELR::AES
    }
    #[doc = "Checks if the value of the field is `LESENSE`"]
    #[inline]
    pub fn is_lesense(&self) -> bool {
        *self == SOURCESELR::LESENSE
    }
    #[doc = "Checks if the value of the field is `EBI`"]
    #[inline]
    pub fn is_ebi(&self) -> bool {
        *self == SOURCESELR::EBI
    }
}
#[doc = r" Proxy"]
pub struct _SIGSELW<'a> {
    w: &'a mut W,
}
impl<'a> _SIGSELW<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 0x0f;
        const OFFSET: u8 = 0;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `SOURCESEL`"]
pub enum SOURCESELW {
    #[doc = "No source selected"]
    NONE,
    #[doc = "Analog to Digital Converter 0"]
    ADC0,
    #[doc = "Digital to Analog Converter 0"]
    DAC0,
    #[doc = "Universal Synchronous/Asynchronous Receiver/Transmitter 0"]
    USART0,
    #[doc = "Universal Synchronous/Asynchronous Receiver/Transmitter 1"]
    USART1,
    #[doc = "Universal Synchronous/Asynchronous Receiver/Transmitter 2"]
    USART2,
    #[doc = "Low Energy UART 0"]
    LEUART0,
    #[doc = "Low Energy UART 1"]
    LEUART1,
    #[doc = "I2C 0"]
    I2C0,
    #[doc = "I2C 1"]
    I2C1,
    #[doc = "Timer 0"]
    TIMER0,
    #[doc = "Timer 1"]
    TIMER1,
    #[doc = "Timer 2"]
    TIMER2,
    #[doc = "Timer 3"]
    TIMER3,
    #[doc = "Universal Asynchronous Receiver/Transmitter 0"]
    UART0,
    #[doc = "Universal Asynchronous Receiver/Transmitter 1"]
    UART1,
    #[doc = "\"\""]
    MSC,
    #[doc = "Advanced Encryption Standard Accelerator"]
    AES,
    #[doc = "Low Energy Sensor Interface"]
    LESENSE,
    #[doc = "External Bus Interface"]
    EBI,
}
impl SOURCESELW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            SOURCESELW::NONE => 0,
            SOURCESELW::ADC0 => 8,
            SOURCESELW::DAC0 => 10,
            SOURCESELW::USART0 => 12,
            SOURCESELW::USART1 => 13,
            SOURCESELW::USART2 => 14,
            SOURCESELW::LEUART0 => 16,
            SOURCESELW::LEUART1 => 17,
            SOURCESELW::I2C0 => 20,
            SOURCESELW::I2C1 => 21,
            SOURCESELW::TIMER0 => 24,
            SOURCESELW::TIMER1 => 25,
            SOURCESELW::TIMER2 => 26,
            SOURCESELW::TIMER3 => 27,
            SOURCESELW::UART0 => 44,
            SOURCESELW::UART1 => 45,
            SOURCESELW::MSC => 48,
            SOURCESELW::AES => 49,
            SOURCESELW::LESENSE => 50,
            SOURCESELW::EBI => 51,
        }
    }
}
#[doc = r" Proxy"]
pub struct _SOURCESELW<'a> {
    w: &'a mut W,
}
impl<'a> _SOURCESELW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: SOURCESELW) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "No source selected"]
    #[inline]
    pub fn none(self) -> &'a mut W {
        self.variant(SOURCESELW::NONE)
    }
    #[doc = "Analog to Digital Converter 0"]
    #[inline]
    pub fn adc0(self) -> &'a mut W {
        self.variant(SOURCESELW::ADC0)
    }
    #[doc = "Digital to Analog Converter 0"]
    #[inline]
    pub fn dac0(self) -> &'a mut W {
        self.variant(SOURCESELW::DAC0)
    }
    #[doc = "Universal Synchronous/Asynchronous Receiver/Transmitter 0"]
    #[inline]
    pub fn usart0(self) -> &'a mut W {
        self.variant(SOURCESELW::USART0)
    }
    #[doc = "Universal Synchronous/Asynchronous Receiver/Transmitter 1"]
    #[inline]
    pub fn usart1(self) -> &'a mut W {
        self.variant(SOURCESELW::USART1)
    }
    #[doc = "Universal Synchronous/Asynchronous Receiver/Transmitter 2"]
    #[inline]
    pub fn usart2(self) -> &'a mut W {
        self.variant(SOURCESELW::USART2)
    }
    #[doc = "Low Energy UART 0"]
    #[inline]
    pub fn leuart0(self) -> &'a mut W {
        self.variant(SOURCESELW::LEUART0)
    }
    #[doc = "Low Energy UART 1"]
    #[inline]
    pub fn leuart1(self) -> &'a mut W {
        self.variant(SOURCESELW::LEUART1)
    }
    #[doc = "I2C 0"]
    #[inline]
    pub fn i2c0(self) -> &'a mut W {
        self.variant(SOURCESELW::I2C0)
    }
    #[doc = "I2C 1"]
    #[inline]
    pub fn i2c1(self) -> &'a mut W {
        self.variant(SOURCESELW::I2C1)
    }
    #[doc = "Timer 0"]
    #[inline]
    pub fn timer0(self) -> &'a mut W {
        self.variant(SOURCESELW::TIMER0)
    }
    #[doc = "Timer 1"]
    #[inline]
    pub fn timer1(self) -> &'a mut W {
        self.variant(SOURCESELW::TIMER1)
    }
    #[doc = "Timer 2"]
    #[inline]
    pub fn timer2(self) -> &'a mut W {
        self.variant(SOURCESELW::TIMER2)
    }
    #[doc = "Timer 3"]
    #[inline]
    pub fn timer3(self) -> &'a mut W {
        self.variant(SOURCESELW::TIMER3)
    }
    #[doc = "Universal Asynchronous Receiver/Transmitter 0"]
    #[inline]
    pub fn uart0(self) -> &'a mut W {
        self.variant(SOURCESELW::UART0)
    }
    #[doc = "Universal Asynchronous Receiver/Transmitter 1"]
    #[inline]
    pub fn uart1(self) -> &'a mut W {
        self.variant(SOURCESELW::UART1)
    }
    #[doc = "\"\""]
    #[inline]
    pub fn msc(self) -> &'a mut W {
        self.variant(SOURCESELW::MSC)
    }
    #[doc = "Advanced Encryption Standard Accelerator"]
    #[inline]
    pub fn aes(self) -> &'a mut W {
        self.variant(SOURCESELW::AES)
    }
    #[doc = "Low Energy Sensor Interface"]
    #[inline]
    pub fn lesense(self) -> &'a mut W {
        self.variant(SOURCESELW::LESENSE)
    }
    #[doc = "External Bus Interface"]
    #[inline]
    pub fn ebi(self) -> &'a mut W {
        self.variant(SOURCESELW::EBI)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 0x3f;
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
    #[doc = "Bits 0:3 - Signal Select"]
    #[inline]
    pub fn sigsel(&self) -> SIGSELR {
        let bits = {
            const MASK: u8 = 0x0f;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        SIGSELR { bits }
    }
    #[doc = "Bits 16:21 - Source Select"]
    #[inline]
    pub fn sourcesel(&self) -> SOURCESELR {
        SOURCESELR::_from({
            const MASK: u8 = 0x3f;
            const OFFSET: u8 = 16;
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
    #[doc = "Bits 0:3 - Signal Select"]
    #[inline]
    pub fn sigsel(&mut self) -> _SIGSELW {
        _SIGSELW { w: self }
    }
    #[doc = "Bits 16:21 - Source Select"]
    #[inline]
    pub fn sourcesel(&mut self) -> _SOURCESELW {
        _SOURCESELW { w: self }
    }
}

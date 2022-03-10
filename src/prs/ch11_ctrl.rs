#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::CH11_CTRL {
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
    #[doc = "Voltage Comparator"]
    VCMP,
    #[doc = "Analog Comparator 0"]
    ACMP0,
    #[doc = "Analog Comparator 1"]
    ACMP1,
    #[doc = "Digital to Analog Converter 0"]
    DAC0,
    #[doc = "Analog to Digital Converter 0"]
    ADC0,
    #[doc = "Universal Synchronous/Asynchronous Receiver/Transmitter 0"]
    USART0,
    #[doc = "Universal Synchronous/Asynchronous Receiver/Transmitter 1"]
    USART1,
    #[doc = "Universal Synchronous/Asynchronous Receiver/Transmitter 2"]
    USART2,
    #[doc = "Timer 0"]
    TIMER0,
    #[doc = "Timer 1"]
    TIMER1,
    #[doc = "Timer 2"]
    TIMER2,
    #[doc = "Timer 3"]
    TIMER3,
    #[doc = "Universal Serial Bus Interface"]
    USB,
    #[doc = "Real-Time Counter"]
    RTC,
    #[doc = "Universal Asynchronous Receiver/Transmitter 0"]
    UART0,
    #[doc = "Universal Asynchronous Receiver/Transmitter 1"]
    UART1,
    #[doc = "General purpose Input/Output"]
    GPIOL,
    #[doc = "General purpose Input/Output"]
    GPIOH,
    #[doc = "Low Energy Timer 0"]
    LETIMER0,
    #[doc = "Backup RTC"]
    BURTC,
    #[doc = "Low Energy Sensor Interface"]
    LESENSEL,
    #[doc = "Low Energy Sensor Interface"]
    LESENSEH,
    #[doc = "Low Energy Sensor Interface"]
    LESENSED,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl SOURCESELR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            SOURCESELR::NONE => 0,
            SOURCESELR::VCMP => 0x01,
            SOURCESELR::ACMP0 => 0x02,
            SOURCESELR::ACMP1 => 0x03,
            SOURCESELR::DAC0 => 0x06,
            SOURCESELR::ADC0 => 0x08,
            SOURCESELR::USART0 => 0x10,
            SOURCESELR::USART1 => 0x11,
            SOURCESELR::USART2 => 0x12,
            SOURCESELR::TIMER0 => 0x1c,
            SOURCESELR::TIMER1 => 0x1d,
            SOURCESELR::TIMER2 => 0x1e,
            SOURCESELR::TIMER3 => 0x1f,
            SOURCESELR::USB => 0x24,
            SOURCESELR::RTC => 0x28,
            SOURCESELR::UART0 => 0x29,
            SOURCESELR::UART1 => 0x2a,
            SOURCESELR::GPIOL => 0x30,
            SOURCESELR::GPIOH => 0x31,
            SOURCESELR::LETIMER0 => 0x34,
            SOURCESELR::BURTC => 0x37,
            SOURCESELR::LESENSEL => 0x39,
            SOURCESELR::LESENSEH => 0x3a,
            SOURCESELR::LESENSED => 0x3b,
            SOURCESELR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> SOURCESELR {
        match value {
            0 => SOURCESELR::NONE,
            1 => SOURCESELR::VCMP,
            2 => SOURCESELR::ACMP0,
            3 => SOURCESELR::ACMP1,
            6 => SOURCESELR::DAC0,
            8 => SOURCESELR::ADC0,
            16 => SOURCESELR::USART0,
            17 => SOURCESELR::USART1,
            18 => SOURCESELR::USART2,
            28 => SOURCESELR::TIMER0,
            29 => SOURCESELR::TIMER1,
            30 => SOURCESELR::TIMER2,
            31 => SOURCESELR::TIMER3,
            36 => SOURCESELR::USB,
            40 => SOURCESELR::RTC,
            41 => SOURCESELR::UART0,
            42 => SOURCESELR::UART1,
            48 => SOURCESELR::GPIOL,
            49 => SOURCESELR::GPIOH,
            52 => SOURCESELR::LETIMER0,
            55 => SOURCESELR::BURTC,
            57 => SOURCESELR::LESENSEL,
            58 => SOURCESELR::LESENSEH,
            59 => SOURCESELR::LESENSED,
            i => SOURCESELR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `NONE`"]
    #[inline]
    pub fn is_none(&self) -> bool {
        *self == SOURCESELR::NONE
    }
    #[doc = "Checks if the value of the field is `VCMP`"]
    #[inline]
    pub fn is_vcmp(&self) -> bool {
        *self == SOURCESELR::VCMP
    }
    #[doc = "Checks if the value of the field is `ACMP0`"]
    #[inline]
    pub fn is_acmp0(&self) -> bool {
        *self == SOURCESELR::ACMP0
    }
    #[doc = "Checks if the value of the field is `ACMP1`"]
    #[inline]
    pub fn is_acmp1(&self) -> bool {
        *self == SOURCESELR::ACMP1
    }
    #[doc = "Checks if the value of the field is `DAC0`"]
    #[inline]
    pub fn is_dac0(&self) -> bool {
        *self == SOURCESELR::DAC0
    }
    #[doc = "Checks if the value of the field is `ADC0`"]
    #[inline]
    pub fn is_adc0(&self) -> bool {
        *self == SOURCESELR::ADC0
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
    #[doc = "Checks if the value of the field is `USB`"]
    #[inline]
    pub fn is_usb(&self) -> bool {
        *self == SOURCESELR::USB
    }
    #[doc = "Checks if the value of the field is `RTC`"]
    #[inline]
    pub fn is_rtc(&self) -> bool {
        *self == SOURCESELR::RTC
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
    #[doc = "Checks if the value of the field is `GPIOL`"]
    #[inline]
    pub fn is_gpiol(&self) -> bool {
        *self == SOURCESELR::GPIOL
    }
    #[doc = "Checks if the value of the field is `GPIOH`"]
    #[inline]
    pub fn is_gpioh(&self) -> bool {
        *self == SOURCESELR::GPIOH
    }
    #[doc = "Checks if the value of the field is `LETIMER0`"]
    #[inline]
    pub fn is_letimer0(&self) -> bool {
        *self == SOURCESELR::LETIMER0
    }
    #[doc = "Checks if the value of the field is `BURTC`"]
    #[inline]
    pub fn is_burtc(&self) -> bool {
        *self == SOURCESELR::BURTC
    }
    #[doc = "Checks if the value of the field is `LESENSEL`"]
    #[inline]
    pub fn is_lesensel(&self) -> bool {
        *self == SOURCESELR::LESENSEL
    }
    #[doc = "Checks if the value of the field is `LESENSEH`"]
    #[inline]
    pub fn is_lesenseh(&self) -> bool {
        *self == SOURCESELR::LESENSEH
    }
    #[doc = "Checks if the value of the field is `LESENSED`"]
    #[inline]
    pub fn is_lesensed(&self) -> bool {
        *self == SOURCESELR::LESENSED
    }
}
#[doc = "Possible values of the field `EDSEL`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum EDSELR {
    #[doc = "Signal is left as it is"]
    OFF,
    #[doc = "A one HFPERCLK cycle pulse is generated for every positive edge of the incoming signal"]
    POSEDGE,
    #[doc = "A one HFPERCLK clock cycle pulse is generated for every negative edge of the incoming signal"]
    NEGEDGE,
    #[doc = "A one HFPERCLK clock cycle pulse is generated for every edge of the incoming signal"]
    BOTHEDGES,
}
impl EDSELR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            EDSELR::OFF => 0,
            EDSELR::POSEDGE => 0x01,
            EDSELR::NEGEDGE => 0x02,
            EDSELR::BOTHEDGES => 0x03,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> EDSELR {
        match value {
            0 => EDSELR::OFF,
            1 => EDSELR::POSEDGE,
            2 => EDSELR::NEGEDGE,
            3 => EDSELR::BOTHEDGES,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `OFF`"]
    #[inline]
    pub fn is_off(&self) -> bool {
        *self == EDSELR::OFF
    }
    #[doc = "Checks if the value of the field is `POSEDGE`"]
    #[inline]
    pub fn is_posedge(&self) -> bool {
        *self == EDSELR::POSEDGE
    }
    #[doc = "Checks if the value of the field is `NEGEDGE`"]
    #[inline]
    pub fn is_negedge(&self) -> bool {
        *self == EDSELR::NEGEDGE
    }
    #[doc = "Checks if the value of the field is `BOTHEDGES`"]
    #[inline]
    pub fn is_bothedges(&self) -> bool {
        *self == EDSELR::BOTHEDGES
    }
}
#[doc = r" Value of the field"]
pub struct ASYNCR {
    bits: bool,
}
impl ASYNCR {
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
#[doc = r" Proxy"]
pub struct _SIGSELW<'a> {
    w: &'a mut W,
}
impl<'a> _SIGSELW<'a> {
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
#[doc = "Values that can be written to the field `SOURCESEL`"]
pub enum SOURCESELW {
    #[doc = "No source selected"]
    NONE,
    #[doc = "Voltage Comparator"]
    VCMP,
    #[doc = "Analog Comparator 0"]
    ACMP0,
    #[doc = "Analog Comparator 1"]
    ACMP1,
    #[doc = "Digital to Analog Converter 0"]
    DAC0,
    #[doc = "Analog to Digital Converter 0"]
    ADC0,
    #[doc = "Universal Synchronous/Asynchronous Receiver/Transmitter 0"]
    USART0,
    #[doc = "Universal Synchronous/Asynchronous Receiver/Transmitter 1"]
    USART1,
    #[doc = "Universal Synchronous/Asynchronous Receiver/Transmitter 2"]
    USART2,
    #[doc = "Timer 0"]
    TIMER0,
    #[doc = "Timer 1"]
    TIMER1,
    #[doc = "Timer 2"]
    TIMER2,
    #[doc = "Timer 3"]
    TIMER3,
    #[doc = "Universal Serial Bus Interface"]
    USB,
    #[doc = "Real-Time Counter"]
    RTC,
    #[doc = "Universal Asynchronous Receiver/Transmitter 0"]
    UART0,
    #[doc = "Universal Asynchronous Receiver/Transmitter 1"]
    UART1,
    #[doc = "General purpose Input/Output"]
    GPIOL,
    #[doc = "General purpose Input/Output"]
    GPIOH,
    #[doc = "Low Energy Timer 0"]
    LETIMER0,
    #[doc = "Backup RTC"]
    BURTC,
    #[doc = "Low Energy Sensor Interface"]
    LESENSEL,
    #[doc = "Low Energy Sensor Interface"]
    LESENSEH,
    #[doc = "Low Energy Sensor Interface"]
    LESENSED,
}
impl SOURCESELW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            SOURCESELW::NONE => 0,
            SOURCESELW::VCMP => 1,
            SOURCESELW::ACMP0 => 2,
            SOURCESELW::ACMP1 => 3,
            SOURCESELW::DAC0 => 6,
            SOURCESELW::ADC0 => 8,
            SOURCESELW::USART0 => 16,
            SOURCESELW::USART1 => 17,
            SOURCESELW::USART2 => 18,
            SOURCESELW::TIMER0 => 28,
            SOURCESELW::TIMER1 => 29,
            SOURCESELW::TIMER2 => 30,
            SOURCESELW::TIMER3 => 31,
            SOURCESELW::USB => 36,
            SOURCESELW::RTC => 40,
            SOURCESELW::UART0 => 41,
            SOURCESELW::UART1 => 42,
            SOURCESELW::GPIOL => 48,
            SOURCESELW::GPIOH => 49,
            SOURCESELW::LETIMER0 => 52,
            SOURCESELW::BURTC => 55,
            SOURCESELW::LESENSEL => 57,
            SOURCESELW::LESENSEH => 58,
            SOURCESELW::LESENSED => 59,
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
    #[doc = "Voltage Comparator"]
    #[inline]
    pub fn vcmp(self) -> &'a mut W {
        self.variant(SOURCESELW::VCMP)
    }
    #[doc = "Analog Comparator 0"]
    #[inline]
    pub fn acmp0(self) -> &'a mut W {
        self.variant(SOURCESELW::ACMP0)
    }
    #[doc = "Analog Comparator 1"]
    #[inline]
    pub fn acmp1(self) -> &'a mut W {
        self.variant(SOURCESELW::ACMP1)
    }
    #[doc = "Digital to Analog Converter 0"]
    #[inline]
    pub fn dac0(self) -> &'a mut W {
        self.variant(SOURCESELW::DAC0)
    }
    #[doc = "Analog to Digital Converter 0"]
    #[inline]
    pub fn adc0(self) -> &'a mut W {
        self.variant(SOURCESELW::ADC0)
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
    #[doc = "Universal Serial Bus Interface"]
    #[inline]
    pub fn usb(self) -> &'a mut W {
        self.variant(SOURCESELW::USB)
    }
    #[doc = "Real-Time Counter"]
    #[inline]
    pub fn rtc(self) -> &'a mut W {
        self.variant(SOURCESELW::RTC)
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
    #[doc = "General purpose Input/Output"]
    #[inline]
    pub fn gpiol(self) -> &'a mut W {
        self.variant(SOURCESELW::GPIOL)
    }
    #[doc = "General purpose Input/Output"]
    #[inline]
    pub fn gpioh(self) -> &'a mut W {
        self.variant(SOURCESELW::GPIOH)
    }
    #[doc = "Low Energy Timer 0"]
    #[inline]
    pub fn letimer0(self) -> &'a mut W {
        self.variant(SOURCESELW::LETIMER0)
    }
    #[doc = "Backup RTC"]
    #[inline]
    pub fn burtc(self) -> &'a mut W {
        self.variant(SOURCESELW::BURTC)
    }
    #[doc = "Low Energy Sensor Interface"]
    #[inline]
    pub fn lesensel(self) -> &'a mut W {
        self.variant(SOURCESELW::LESENSEL)
    }
    #[doc = "Low Energy Sensor Interface"]
    #[inline]
    pub fn lesenseh(self) -> &'a mut W {
        self.variant(SOURCESELW::LESENSEH)
    }
    #[doc = "Low Energy Sensor Interface"]
    #[inline]
    pub fn lesensed(self) -> &'a mut W {
        self.variant(SOURCESELW::LESENSED)
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
#[doc = "Values that can be written to the field `EDSEL`"]
pub enum EDSELW {
    #[doc = "Signal is left as it is"]
    OFF,
    #[doc = "A one HFPERCLK cycle pulse is generated for every positive edge of the incoming signal"]
    POSEDGE,
    #[doc = "A one HFPERCLK clock cycle pulse is generated for every negative edge of the incoming signal"]
    NEGEDGE,
    #[doc = "A one HFPERCLK clock cycle pulse is generated for every edge of the incoming signal"]
    BOTHEDGES,
}
impl EDSELW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            EDSELW::OFF => 0,
            EDSELW::POSEDGE => 1,
            EDSELW::NEGEDGE => 2,
            EDSELW::BOTHEDGES => 3,
        }
    }
}
#[doc = r" Proxy"]
pub struct _EDSELW<'a> {
    w: &'a mut W,
}
impl<'a> _EDSELW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: EDSELW) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "Signal is left as it is"]
    #[inline]
    pub fn off(self) -> &'a mut W {
        self.variant(EDSELW::OFF)
    }
    #[doc = "A one HFPERCLK cycle pulse is generated for every positive edge of the incoming signal"]
    #[inline]
    pub fn posedge(self) -> &'a mut W {
        self.variant(EDSELW::POSEDGE)
    }
    #[doc = "A one HFPERCLK clock cycle pulse is generated for every negative edge of the incoming signal"]
    #[inline]
    pub fn negedge(self) -> &'a mut W {
        self.variant(EDSELW::NEGEDGE)
    }
    #[doc = "A one HFPERCLK clock cycle pulse is generated for every edge of the incoming signal"]
    #[inline]
    pub fn bothedges(self) -> &'a mut W {
        self.variant(EDSELW::BOTHEDGES)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 0x03;
        const OFFSET: u8 = 24;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _ASYNCW<'a> {
    w: &'a mut W,
}
impl<'a> _ASYNCW<'a> {
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
        const OFFSET: u8 = 28;
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
    #[doc = "Bits 0:2 - Signal Select"]
    #[inline]
    pub fn sigsel(&self) -> SIGSELR {
        let bits = {
            const MASK: u8 = 0x07;
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
    #[doc = "Bits 24:25 - Edge Detect Select"]
    #[inline]
    pub fn edsel(&self) -> EDSELR {
        EDSELR::_from({
            const MASK: u8 = 0x03;
            const OFFSET: u8 = 24;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bit 28 - Asynchronous reflex"]
    #[inline]
    pub fn async(&self) -> ASYNCR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 28;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        ASYNCR { bits }
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
    #[doc = "Bits 0:2 - Signal Select"]
    #[inline]
    pub fn sigsel(&mut self) -> _SIGSELW {
        _SIGSELW { w: self }
    }
    #[doc = "Bits 16:21 - Source Select"]
    #[inline]
    pub fn sourcesel(&mut self) -> _SOURCESELW {
        _SOURCESELW { w: self }
    }
    #[doc = "Bits 24:25 - Edge Detect Select"]
    #[inline]
    pub fn edsel(&mut self) -> _EDSELW {
        _EDSELW { w: self }
    }
    #[doc = "Bit 28 - Asynchronous reflex"]
    #[inline]
    pub fn async(&mut self) -> _ASYNCW {
        _ASYNCW { w: self }
    }
}

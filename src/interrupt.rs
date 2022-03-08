use bare_metal::Nr;
#[cfg(feature = "rt")]
extern "C" {
    fn DEFAULT_HANDLER();
}
#[cfg(feature = "rt")]
#[allow(non_snake_case)]
#[no_mangle]
pub unsafe extern "C" fn DH_TRAMPOLINE() {
    DEFAULT_HANDLER();
}
#[cfg(feature = "rt")]
global_asm ! ( "\n.weak DMA\nDMA = DH_TRAMPOLINE\n.weak GPIO_EVEN\nGPIO_EVEN = DH_TRAMPOLINE\n.weak TIMER0\nTIMER0 = DH_TRAMPOLINE\n.weak USART0_RX\nUSART0_RX = DH_TRAMPOLINE\n.weak USART0_TX\nUSART0_TX = DH_TRAMPOLINE\n.weak USB\nUSB = DH_TRAMPOLINE\n.weak ACMP0\nACMP0 = DH_TRAMPOLINE\n.weak ADC0\nADC0 = DH_TRAMPOLINE\n.weak DAC0\nDAC0 = DH_TRAMPOLINE\n.weak I2C0\nI2C0 = DH_TRAMPOLINE\n.weak I2C1\nI2C1 = DH_TRAMPOLINE\n.weak GPIO_ODD\nGPIO_ODD = DH_TRAMPOLINE\n.weak TIMER1\nTIMER1 = DH_TRAMPOLINE\n.weak TIMER2\nTIMER2 = DH_TRAMPOLINE\n.weak TIMER3\nTIMER3 = DH_TRAMPOLINE\n.weak USART1_RX\nUSART1_RX = DH_TRAMPOLINE\n.weak USART1_TX\nUSART1_TX = DH_TRAMPOLINE\n.weak LESENSE\nLESENSE = DH_TRAMPOLINE\n.weak USART2_RX\nUSART2_RX = DH_TRAMPOLINE\n.weak USART2_TX\nUSART2_TX = DH_TRAMPOLINE\n.weak UART0_RX\nUART0_RX = DH_TRAMPOLINE\n.weak UART0_TX\nUART0_TX = DH_TRAMPOLINE\n.weak UART1_RX\nUART1_RX = DH_TRAMPOLINE\n.weak UART1_TX\nUART1_TX = DH_TRAMPOLINE\n.weak LEUART0\nLEUART0 = DH_TRAMPOLINE\n.weak LEUART1\nLEUART1 = DH_TRAMPOLINE\n.weak LETIMER0\nLETIMER0 = DH_TRAMPOLINE\n.weak PCNT0\nPCNT0 = DH_TRAMPOLINE\n.weak PCNT1\nPCNT1 = DH_TRAMPOLINE\n.weak PCNT2\nPCNT2 = DH_TRAMPOLINE\n.weak RTC\nRTC = DH_TRAMPOLINE\n.weak BURTC\nBURTC = DH_TRAMPOLINE\n.weak CMU\nCMU = DH_TRAMPOLINE\n.weak VCMP\nVCMP = DH_TRAMPOLINE\n.weak LCD\nLCD = DH_TRAMPOLINE\n.weak MSC\nMSC = DH_TRAMPOLINE\n.weak AES\nAES = DH_TRAMPOLINE\n.weak EBI\nEBI = DH_TRAMPOLINE\n.weak EMU\nEMU = DH_TRAMPOLINE" ) ;
#[cfg(feature = "rt")]
extern "C" {
    fn DMA();
    fn GPIO_EVEN();
    fn TIMER0();
    fn USART0_RX();
    fn USART0_TX();
    fn USB();
    fn ACMP0();
    fn ADC0();
    fn DAC0();
    fn I2C0();
    fn I2C1();
    fn GPIO_ODD();
    fn TIMER1();
    fn TIMER2();
    fn TIMER3();
    fn USART1_RX();
    fn USART1_TX();
    fn LESENSE();
    fn USART2_RX();
    fn USART2_TX();
    fn UART0_RX();
    fn UART0_TX();
    fn UART1_RX();
    fn UART1_TX();
    fn LEUART0();
    fn LEUART1();
    fn LETIMER0();
    fn PCNT0();
    fn PCNT1();
    fn PCNT2();
    fn RTC();
    fn BURTC();
    fn CMU();
    fn VCMP();
    fn LCD();
    fn MSC();
    fn AES();
    fn EBI();
    fn EMU();
}
#[cfg(feature = "rt")]
#[doc(hidden)]
#[link_section = ".vector_table.interrupts"]
#[no_mangle]
#[used]
pub static INTERRUPTS: [Option<unsafe extern "C" fn()>; 39] = [
    Some(DMA),
    Some(GPIO_EVEN),
    Some(TIMER0),
    Some(USART0_RX),
    Some(USART0_TX),
    Some(USB),
    Some(ACMP0),
    Some(ADC0),
    Some(DAC0),
    Some(I2C0),
    Some(I2C1),
    Some(GPIO_ODD),
    Some(TIMER1),
    Some(TIMER2),
    Some(TIMER3),
    Some(USART1_RX),
    Some(USART1_TX),
    Some(LESENSE),
    Some(USART2_RX),
    Some(USART2_TX),
    Some(UART0_RX),
    Some(UART0_TX),
    Some(UART1_RX),
    Some(UART1_TX),
    Some(LEUART0),
    Some(LEUART1),
    Some(LETIMER0),
    Some(PCNT0),
    Some(PCNT1),
    Some(PCNT2),
    Some(RTC),
    Some(BURTC),
    Some(CMU),
    Some(VCMP),
    Some(LCD),
    Some(MSC),
    Some(AES),
    Some(EBI),
    Some(EMU),
];
#[doc = r" Enumeration of all the interrupts"]
pub enum Interrupt {
    #[doc = "0 - DMA"]
    DMA,
    #[doc = "1 - GPIO_EVEN"]
    GPIO_EVEN,
    #[doc = "2 - TIMER0"]
    TIMER0,
    #[doc = "3 - USART0_RX"]
    USART0_RX,
    #[doc = "4 - USART0_TX"]
    USART0_TX,
    #[doc = "5 - USB"]
    USB,
    #[doc = "6 - ACMP0"]
    ACMP0,
    #[doc = "7 - ADC0"]
    ADC0,
    #[doc = "8 - DAC0"]
    DAC0,
    #[doc = "9 - I2C0"]
    I2C0,
    #[doc = "10 - I2C1"]
    I2C1,
    #[doc = "11 - GPIO_ODD"]
    GPIO_ODD,
    #[doc = "12 - TIMER1"]
    TIMER1,
    #[doc = "13 - TIMER2"]
    TIMER2,
    #[doc = "14 - TIMER3"]
    TIMER3,
    #[doc = "15 - USART1_RX"]
    USART1_RX,
    #[doc = "16 - USART1_TX"]
    USART1_TX,
    #[doc = "17 - LESENSE"]
    LESENSE,
    #[doc = "18 - USART2_RX"]
    USART2_RX,
    #[doc = "19 - USART2_TX"]
    USART2_TX,
    #[doc = "20 - UART0_RX"]
    UART0_RX,
    #[doc = "21 - UART0_TX"]
    UART0_TX,
    #[doc = "22 - UART1_RX"]
    UART1_RX,
    #[doc = "23 - UART1_TX"]
    UART1_TX,
    #[doc = "24 - LEUART0"]
    LEUART0,
    #[doc = "25 - LEUART1"]
    LEUART1,
    #[doc = "26 - LETIMER0"]
    LETIMER0,
    #[doc = "27 - PCNT0"]
    PCNT0,
    #[doc = "28 - PCNT1"]
    PCNT1,
    #[doc = "29 - PCNT2"]
    PCNT2,
    #[doc = "30 - RTC"]
    RTC,
    #[doc = "31 - BURTC"]
    BURTC,
    #[doc = "32 - CMU"]
    CMU,
    #[doc = "33 - VCMP"]
    VCMP,
    #[doc = "34 - LCD"]
    LCD,
    #[doc = "35 - MSC"]
    MSC,
    #[doc = "36 - AES"]
    AES,
    #[doc = "37 - EBI"]
    EBI,
    #[doc = "38 - EMU"]
    EMU,
}
unsafe impl Nr for Interrupt {
    #[inline]
    fn nr(&self) -> u8 {
        match *self {
            Interrupt::DMA => 0,
            Interrupt::GPIO_EVEN => 1,
            Interrupt::TIMER0 => 2,
            Interrupt::USART0_RX => 3,
            Interrupt::USART0_TX => 4,
            Interrupt::USB => 5,
            Interrupt::ACMP0 => 6,
            Interrupt::ADC0 => 7,
            Interrupt::DAC0 => 8,
            Interrupt::I2C0 => 9,
            Interrupt::I2C1 => 10,
            Interrupt::GPIO_ODD => 11,
            Interrupt::TIMER1 => 12,
            Interrupt::TIMER2 => 13,
            Interrupt::TIMER3 => 14,
            Interrupt::USART1_RX => 15,
            Interrupt::USART1_TX => 16,
            Interrupt::LESENSE => 17,
            Interrupt::USART2_RX => 18,
            Interrupt::USART2_TX => 19,
            Interrupt::UART0_RX => 20,
            Interrupt::UART0_TX => 21,
            Interrupt::UART1_RX => 22,
            Interrupt::UART1_TX => 23,
            Interrupt::LEUART0 => 24,
            Interrupt::LEUART1 => 25,
            Interrupt::LETIMER0 => 26,
            Interrupt::PCNT0 => 27,
            Interrupt::PCNT1 => 28,
            Interrupt::PCNT2 => 29,
            Interrupt::RTC => 30,
            Interrupt::BURTC => 31,
            Interrupt::CMU => 32,
            Interrupt::VCMP => 33,
            Interrupt::LCD => 34,
            Interrupt::MSC => 35,
            Interrupt::AES => 36,
            Interrupt::EBI => 37,
            Interrupt::EMU => 38,
        }
    }
}
#[cfg(feature = "rt")]
#[macro_export]
macro_rules ! interrupt { ( $ NAME : ident , $ path : path , locals : { $ ( $ lvar : ident : $ lty : ty = $ lval : expr ; ) * } ) => { # [ allow ( non_snake_case ) ] mod $ NAME { pub struct Locals { $ ( pub $ lvar : $ lty , ) * } } # [ allow ( non_snake_case ) ] # [ no_mangle ] pub extern "C" fn $ NAME ( ) { let _ = $ crate :: interrupt :: Interrupt :: $ NAME ; static mut LOCALS : self :: $ NAME :: Locals = self :: $ NAME :: Locals { $ ( $ lvar : $ lval , ) * } ; let f : fn ( & mut self :: $ NAME :: Locals ) = $ path ; f ( unsafe { & mut LOCALS } ) ; } } ; ( $ NAME : ident , $ path : path ) => { # [ allow ( non_snake_case ) ] # [ no_mangle ] pub extern "C" fn $ NAME ( ) { let _ = $ crate :: interrupt :: Interrupt :: $ NAME ; let f : fn ( ) = $ path ; f ( ) ; } } }

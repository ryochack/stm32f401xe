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
global_asm ! ( "\n.weak PVD\nPVD = DH_TRAMPOLINE\n.weak TAMP_STAMP\nTAMP_STAMP = DH_TRAMPOLINE\n.weak RTC_WKUP\nRTC_WKUP = DH_TRAMPOLINE\n.weak FLASH\nFLASH = DH_TRAMPOLINE\n.weak RCC\nRCC = DH_TRAMPOLINE\n.weak EXTI0\nEXTI0 = DH_TRAMPOLINE\n.weak EXTI1\nEXTI1 = DH_TRAMPOLINE\n.weak EXTI2\nEXTI2 = DH_TRAMPOLINE\n.weak EXTI3\nEXTI3 = DH_TRAMPOLINE\n.weak EXTI4\nEXTI4 = DH_TRAMPOLINE\n.weak ADC\nADC = DH_TRAMPOLINE\n.weak EXTI9_5\nEXTI9_5 = DH_TRAMPOLINE\n.weak TIM1_BRK_TIM9\nTIM1_BRK_TIM9 = DH_TRAMPOLINE\n.weak TIM1_UP_TIM10\nTIM1_UP_TIM10 = DH_TRAMPOLINE\n.weak TIM1_TRG_COM_TIM11\nTIM1_TRG_COM_TIM11 = DH_TRAMPOLINE\n.weak TIM1_CC\nTIM1_CC = DH_TRAMPOLINE\n.weak TIM2\nTIM2 = DH_TRAMPOLINE\n.weak TIM3\nTIM3 = DH_TRAMPOLINE\n.weak I2C1_EV\nI2C1_EV = DH_TRAMPOLINE\n.weak I2C1_ER\nI2C1_ER = DH_TRAMPOLINE\n.weak I2C2_EV\nI2C2_EV = DH_TRAMPOLINE\n.weak I2C2_ER\nI2C2_ER = DH_TRAMPOLINE\n.weak SPI1\nSPI1 = DH_TRAMPOLINE\n.weak SPI2\nSPI2 = DH_TRAMPOLINE\n.weak EXTI15_10\nEXTI15_10 = DH_TRAMPOLINE\n.weak RTC_ALARM\nRTC_ALARM = DH_TRAMPOLINE\n.weak OTG_FS_WKUP\nOTG_FS_WKUP = DH_TRAMPOLINE\n.weak SDIO\nSDIO = DH_TRAMPOLINE\n.weak SPI3\nSPI3 = DH_TRAMPOLINE\n.weak OTG_FS\nOTG_FS = DH_TRAMPOLINE\n.weak I2C3_EV\nI2C3_EV = DH_TRAMPOLINE\n.weak I2C3_ER\nI2C3_ER = DH_TRAMPOLINE\n.weak FPU\nFPU = DH_TRAMPOLINE\n.weak SPI4\nSPI4 = DH_TRAMPOLINE" ) ;
#[cfg(feature = "rt")]
extern "C" {
    fn PVD();
    fn TAMP_STAMP();
    fn RTC_WKUP();
    fn FLASH();
    fn RCC();
    fn EXTI0();
    fn EXTI1();
    fn EXTI2();
    fn EXTI3();
    fn EXTI4();
    fn ADC();
    fn EXTI9_5();
    fn TIM1_BRK_TIM9();
    fn TIM1_UP_TIM10();
    fn TIM1_TRG_COM_TIM11();
    fn TIM1_CC();
    fn TIM2();
    fn TIM3();
    fn I2C1_EV();
    fn I2C1_ER();
    fn I2C2_EV();
    fn I2C2_ER();
    fn SPI1();
    fn SPI2();
    fn EXTI15_10();
    fn RTC_ALARM();
    fn OTG_FS_WKUP();
    fn SDIO();
    fn SPI3();
    fn OTG_FS();
    fn I2C3_EV();
    fn I2C3_ER();
    fn FPU();
    fn SPI4();
}
#[allow(private_no_mangle_statics)]
#[cfg(feature = "rt")]
#[doc(hidden)]
#[link_section = ".vector_table.interrupts"]
#[no_mangle]
#[used]
pub static INTERRUPTS: [Option<unsafe extern "C" fn()>; 85] = [
    None,
    Some(PVD),
    Some(TAMP_STAMP),
    Some(RTC_WKUP),
    Some(FLASH),
    Some(RCC),
    Some(EXTI0),
    Some(EXTI1),
    Some(EXTI2),
    Some(EXTI3),
    Some(EXTI4),
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    Some(ADC),
    None,
    None,
    None,
    None,
    Some(EXTI9_5),
    Some(TIM1_BRK_TIM9),
    Some(TIM1_UP_TIM10),
    Some(TIM1_TRG_COM_TIM11),
    Some(TIM1_CC),
    Some(TIM2),
    Some(TIM3),
    None,
    Some(I2C1_EV),
    Some(I2C1_ER),
    Some(I2C2_EV),
    Some(I2C2_ER),
    Some(SPI1),
    Some(SPI2),
    None,
    None,
    None,
    Some(EXTI15_10),
    Some(RTC_ALARM),
    Some(OTG_FS_WKUP),
    None,
    None,
    None,
    None,
    None,
    None,
    Some(SDIO),
    None,
    Some(SPI3),
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    Some(OTG_FS),
    None,
    None,
    None,
    None,
    Some(I2C3_EV),
    Some(I2C3_ER),
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    Some(FPU),
    None,
    None,
    Some(SPI4),
];
#[doc = r" Enumeration of all the interrupts"]
pub enum Interrupt {
    #[doc = "1 - PVD through EXTI line detection interrupt"]
    PVD,
    #[doc = "2 - Tamper and TimeStamp interrupts through the EXTI line"]
    TAMP_STAMP,
    #[doc = "3 - RTC Wakeup interrupt through the EXTI line"]
    RTC_WKUP,
    #[doc = "4 - FLASH global interrupt"]
    FLASH,
    #[doc = "5 - RCC global interrupt"]
    RCC,
    #[doc = "6 - EXTI Line0 interrupt"]
    EXTI0,
    #[doc = "7 - EXTI Line1 interrupt"]
    EXTI1,
    #[doc = "8 - EXTI Line2 interrupt"]
    EXTI2,
    #[doc = "9 - EXTI Line3 interrupt"]
    EXTI3,
    #[doc = "10 - EXTI Line4 interrupt"]
    EXTI4,
    #[doc = "18 - ADC1 global interrupt"]
    ADC,
    #[doc = "23 - EXTI Line[9:5] interrupts"]
    EXTI9_5,
    #[doc = "24 - TIM1 Break interrupt and TIM9 global interrupt"]
    TIM1_BRK_TIM9,
    #[doc = "25 - TIM1 Update interrupt and TIM10 global interrupt"]
    TIM1_UP_TIM10,
    #[doc = "26 - TIM1 Trigger and Commutation interrupts and TIM11 global interrupt"]
    TIM1_TRG_COM_TIM11,
    #[doc = "27 - TIM1 Capture Compare interrupt"]
    TIM1_CC,
    #[doc = "28 - TIM2 global interrupt"]
    TIM2,
    #[doc = "29 - TIM3 global interrupt"]
    TIM3,
    #[doc = "31 - I2C1 event interrupt"]
    I2C1_EV,
    #[doc = "32 - I2C1 error interrupt"]
    I2C1_ER,
    #[doc = "33 - I2C2 event interrupt"]
    I2C2_EV,
    #[doc = "34 - I2C2 error interrupt"]
    I2C2_ER,
    #[doc = "35 - SPI1 global interrupt"]
    SPI1,
    #[doc = "36 - SPI2 global interrupt"]
    SPI2,
    #[doc = "40 - EXTI Line[15:10] interrupts"]
    EXTI15_10,
    #[doc = "41 - RTC Alarms (A and B) through EXTI line interrupt"]
    RTC_ALARM,
    #[doc = "42 - USB On-The-Go FS Wakeup through EXTI line interrupt"]
    OTG_FS_WKUP,
    #[doc = "49 - SDIO global interrupt"]
    SDIO,
    #[doc = "51 - SPI3 global interrupt"]
    SPI3,
    #[doc = "67 - USB On The Go FS global interrupt"]
    OTG_FS,
    #[doc = "72 - I2C3 event interrupt"]
    I2C3_EV,
    #[doc = "73 - I2C3 error interrupt"]
    I2C3_ER,
    #[doc = "81 - FPU interrupt"]
    FPU,
    #[doc = "84 - SPI4 global interrupt"]
    SPI4,
}
unsafe impl Nr for Interrupt {
    #[inline]
    fn nr(&self) -> u8 {
        match *self {
            Interrupt::PVD => 1,
            Interrupt::TAMP_STAMP => 2,
            Interrupt::RTC_WKUP => 3,
            Interrupt::FLASH => 4,
            Interrupt::RCC => 5,
            Interrupt::EXTI0 => 6,
            Interrupt::EXTI1 => 7,
            Interrupt::EXTI2 => 8,
            Interrupt::EXTI3 => 9,
            Interrupt::EXTI4 => 10,
            Interrupt::ADC => 18,
            Interrupt::EXTI9_5 => 23,
            Interrupt::TIM1_BRK_TIM9 => 24,
            Interrupt::TIM1_UP_TIM10 => 25,
            Interrupt::TIM1_TRG_COM_TIM11 => 26,
            Interrupt::TIM1_CC => 27,
            Interrupt::TIM2 => 28,
            Interrupt::TIM3 => 29,
            Interrupt::I2C1_EV => 31,
            Interrupt::I2C1_ER => 32,
            Interrupt::I2C2_EV => 33,
            Interrupt::I2C2_ER => 34,
            Interrupt::SPI1 => 35,
            Interrupt::SPI2 => 36,
            Interrupt::EXTI15_10 => 40,
            Interrupt::RTC_ALARM => 41,
            Interrupt::OTG_FS_WKUP => 42,
            Interrupt::SDIO => 49,
            Interrupt::SPI3 => 51,
            Interrupt::OTG_FS => 67,
            Interrupt::I2C3_EV => 72,
            Interrupt::I2C3_ER => 73,
            Interrupt::FPU => 81,
            Interrupt::SPI4 => 84,
        }
    }
}
#[cfg(feature = "rt")]
#[macro_export]
macro_rules ! interrupt { ( $ NAME : ident , $ path : path , locals : { $ ( $ lvar : ident : $ lty : ty = $ lval : expr ; ) * } ) => { # [ allow ( non_snake_case ) ] mod $ NAME { pub struct Locals { $ ( pub $ lvar : $ lty , ) * } } # [ allow ( non_snake_case ) ] # [ no_mangle ] pub extern "C" fn $ NAME ( ) { let _ = $ crate :: interrupt :: Interrupt :: $ NAME ; static mut LOCALS : self :: $ NAME :: Locals = self :: $ NAME :: Locals { $ ( $ lvar : $ lval , ) * } ; let f : fn ( & mut self :: $ NAME :: Locals ) = $ path ; f ( unsafe { & mut LOCALS } ) ; } } ; ( $ NAME : ident , $ path : path ) => { # [ allow ( non_snake_case ) ] # [ no_mangle ] pub extern "C" fn $ NAME ( ) { let _ = $ crate :: interrupt :: Interrupt :: $ NAME ; let f : fn ( ) = $ path ; f ( ) ; } } }

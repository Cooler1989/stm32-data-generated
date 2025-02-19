#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum Interrupt {
    #[doc = "0 - WWDG"]
    WWDG = 0,
    #[doc = "1 - PVD_AVD"]
    PVD_AVD = 1,
    #[doc = "2 - RTC"]
    RTC = 2,
    #[doc = "3 - RTC_S"]
    RTC_S = 3,
    #[doc = "4 - TAMP"]
    TAMP = 4,
    #[doc = "5 - RAMCFG"]
    RAMCFG = 5,
    #[doc = "6 - FLASH"]
    FLASH = 6,
    #[doc = "7 - FLASH_S"]
    FLASH_S = 7,
    #[doc = "8 - GTZC"]
    GTZC = 8,
    #[doc = "9 - RCC"]
    RCC = 9,
    #[doc = "10 - RCC_S"]
    RCC_S = 10,
    #[doc = "11 - EXTI0"]
    EXTI0 = 11,
    #[doc = "12 - EXTI1"]
    EXTI1 = 12,
    #[doc = "13 - EXTI2"]
    EXTI2 = 13,
    #[doc = "14 - EXTI3"]
    EXTI3 = 14,
    #[doc = "15 - EXTI4"]
    EXTI4 = 15,
    #[doc = "16 - EXTI5"]
    EXTI5 = 16,
    #[doc = "17 - EXTI6"]
    EXTI6 = 17,
    #[doc = "18 - EXTI7"]
    EXTI7 = 18,
    #[doc = "19 - EXTI8"]
    EXTI8 = 19,
    #[doc = "20 - EXTI9"]
    EXTI9 = 20,
    #[doc = "21 - EXTI10"]
    EXTI10 = 21,
    #[doc = "22 - EXTI11"]
    EXTI11 = 22,
    #[doc = "23 - EXTI12"]
    EXTI12 = 23,
    #[doc = "24 - EXTI13"]
    EXTI13 = 24,
    #[doc = "25 - EXTI14"]
    EXTI14 = 25,
    #[doc = "26 - EXTI15"]
    EXTI15 = 26,
    #[doc = "27 - GPDMA1_CHANNEL0"]
    GPDMA1_CHANNEL0 = 27,
    #[doc = "28 - GPDMA1_CHANNEL1"]
    GPDMA1_CHANNEL1 = 28,
    #[doc = "29 - GPDMA1_CHANNEL2"]
    GPDMA1_CHANNEL2 = 29,
    #[doc = "30 - GPDMA1_CHANNEL3"]
    GPDMA1_CHANNEL3 = 30,
    #[doc = "31 - GPDMA1_CHANNEL4"]
    GPDMA1_CHANNEL4 = 31,
    #[doc = "32 - GPDMA1_CHANNEL5"]
    GPDMA1_CHANNEL5 = 32,
    #[doc = "33 - GPDMA1_CHANNEL6"]
    GPDMA1_CHANNEL6 = 33,
    #[doc = "34 - GPDMA1_CHANNEL7"]
    GPDMA1_CHANNEL7 = 34,
    #[doc = "35 - IWDG"]
    IWDG = 35,
    #[doc = "36 - SAES"]
    SAES = 36,
    #[doc = "37 - ADC1"]
    ADC1 = 37,
    #[doc = "38 - DAC1"]
    DAC1 = 38,
    #[doc = "39 - FDCAN1_IT0"]
    FDCAN1_IT0 = 39,
    #[doc = "40 - FDCAN1_IT1"]
    FDCAN1_IT1 = 40,
    #[doc = "41 - TIM1_BRK"]
    TIM1_BRK = 41,
    #[doc = "42 - TIM1_UP"]
    TIM1_UP = 42,
    #[doc = "43 - TIM1_TRG_COM"]
    TIM1_TRG_COM = 43,
    #[doc = "44 - TIM1_CC"]
    TIM1_CC = 44,
    #[doc = "45 - TIM2"]
    TIM2 = 45,
    #[doc = "46 - TIM3"]
    TIM3 = 46,
    #[doc = "47 - TIM4"]
    TIM4 = 47,
    #[doc = "48 - TIM5"]
    TIM5 = 48,
    #[doc = "49 - TIM6"]
    TIM6 = 49,
    #[doc = "50 - TIM7"]
    TIM7 = 50,
    #[doc = "51 - I2C1_EV"]
    I2C1_EV = 51,
    #[doc = "52 - I2C1_ER"]
    I2C1_ER = 52,
    #[doc = "53 - I2C2_EV"]
    I2C2_EV = 53,
    #[doc = "54 - I2C2_ER"]
    I2C2_ER = 54,
    #[doc = "55 - SPI1"]
    SPI1 = 55,
    #[doc = "56 - SPI2"]
    SPI2 = 56,
    #[doc = "57 - SPI3"]
    SPI3 = 57,
    #[doc = "58 - USART1"]
    USART1 = 58,
    #[doc = "59 - USART2"]
    USART2 = 59,
    #[doc = "60 - USART3"]
    USART3 = 60,
    #[doc = "61 - UART4"]
    UART4 = 61,
    #[doc = "62 - UART5"]
    UART5 = 62,
    #[doc = "63 - LPUART1"]
    LPUART1 = 63,
    #[doc = "64 - LPTIM1"]
    LPTIM1 = 64,
    #[doc = "65 - TIM8_BRK"]
    TIM8_BRK = 65,
    #[doc = "66 - TIM8_UP"]
    TIM8_UP = 66,
    #[doc = "67 - TIM8_TRG_COM"]
    TIM8_TRG_COM = 67,
    #[doc = "68 - TIM8_CC"]
    TIM8_CC = 68,
    #[doc = "69 - ADC2"]
    ADC2 = 69,
    #[doc = "70 - LPTIM2"]
    LPTIM2 = 70,
    #[doc = "71 - TIM15"]
    TIM15 = 71,
    #[doc = "72 - TIM16"]
    TIM16 = 72,
    #[doc = "73 - TIM17"]
    TIM17 = 73,
    #[doc = "74 - USB_DRD_FS"]
    USB_DRD_FS = 74,
    #[doc = "75 - CRS"]
    CRS = 75,
    #[doc = "76 - UCPD1"]
    UCPD1 = 76,
    #[doc = "77 - FMC"]
    FMC = 77,
    #[doc = "78 - OCTOSPI1"]
    OCTOSPI1 = 78,
    #[doc = "79 - SDMMC1"]
    SDMMC1 = 79,
    #[doc = "80 - I2C3_EV"]
    I2C3_EV = 80,
    #[doc = "81 - I2C3_ER"]
    I2C3_ER = 81,
    #[doc = "82 - SPI4"]
    SPI4 = 82,
    #[doc = "83 - SPI5"]
    SPI5 = 83,
    #[doc = "84 - SPI6"]
    SPI6 = 84,
    #[doc = "85 - USART6"]
    USART6 = 85,
    #[doc = "86 - USART10"]
    USART10 = 86,
    #[doc = "87 - USART11"]
    USART11 = 87,
    #[doc = "88 - SAI1"]
    SAI1 = 88,
    #[doc = "89 - SAI2"]
    SAI2 = 89,
    #[doc = "90 - GPDMA2_CHANNEL0"]
    GPDMA2_CHANNEL0 = 90,
    #[doc = "91 - GPDMA2_CHANNEL1"]
    GPDMA2_CHANNEL1 = 91,
    #[doc = "92 - GPDMA2_CHANNEL2"]
    GPDMA2_CHANNEL2 = 92,
    #[doc = "93 - GPDMA2_CHANNEL3"]
    GPDMA2_CHANNEL3 = 93,
    #[doc = "94 - GPDMA2_CHANNEL4"]
    GPDMA2_CHANNEL4 = 94,
    #[doc = "95 - GPDMA2_CHANNEL5"]
    GPDMA2_CHANNEL5 = 95,
    #[doc = "96 - GPDMA2_CHANNEL6"]
    GPDMA2_CHANNEL6 = 96,
    #[doc = "97 - GPDMA2_CHANNEL7"]
    GPDMA2_CHANNEL7 = 97,
    #[doc = "98 - UART7"]
    UART7 = 98,
    #[doc = "99 - UART8"]
    UART8 = 99,
    #[doc = "100 - UART9"]
    UART9 = 100,
    #[doc = "101 - UART12"]
    UART12 = 101,
    #[doc = "102 - SDMMC2"]
    SDMMC2 = 102,
    #[doc = "103 - FPU"]
    FPU = 103,
    #[doc = "104 - ICACHE"]
    ICACHE = 104,
    #[doc = "105 - DCACHE1"]
    DCACHE1 = 105,
    #[doc = "106 - ETH"]
    ETH = 106,
    #[doc = "107 - ETH_WKUP"]
    ETH_WKUP = 107,
    #[doc = "108 - DCMI_PSSI"]
    DCMI_PSSI = 108,
    #[doc = "109 - FDCAN2_IT0"]
    FDCAN2_IT0 = 109,
    #[doc = "110 - FDCAN2_IT1"]
    FDCAN2_IT1 = 110,
    #[doc = "111 - CORDIC"]
    CORDIC = 111,
    #[doc = "112 - FMAC"]
    FMAC = 112,
    #[doc = "113 - DTS"]
    DTS = 113,
    #[doc = "114 - RNG"]
    RNG = 114,
    #[doc = "115 - OTFDEC1"]
    OTFDEC1 = 115,
    #[doc = "116 - AES"]
    AES = 116,
    #[doc = "117 - HASH"]
    HASH = 117,
    #[doc = "118 - PKA"]
    PKA = 118,
    #[doc = "119 - CEC"]
    CEC = 119,
    #[doc = "120 - TIM12"]
    TIM12 = 120,
    #[doc = "121 - TIM13"]
    TIM13 = 121,
    #[doc = "122 - TIM14"]
    TIM14 = 122,
    #[doc = "123 - I3C1_EV"]
    I3C1_EV = 123,
    #[doc = "124 - I3C1_ER"]
    I3C1_ER = 124,
    #[doc = "125 - I2C4_EV"]
    I2C4_EV = 125,
    #[doc = "126 - I2C4_ER"]
    I2C4_ER = 126,
    #[doc = "127 - LPTIM3"]
    LPTIM3 = 127,
    #[doc = "128 - LPTIM4"]
    LPTIM4 = 128,
    #[doc = "129 - LPTIM5"]
    LPTIM5 = 129,
    #[doc = "130 - LPTIM6"]
    LPTIM6 = 130,
}
unsafe impl cortex_m::interrupt::InterruptNumber for Interrupt {
    #[inline(always)]
    fn number(self) -> u16 {
        self as u16
    }
}
#[cfg(feature = "rt")]
mod _vectors {
    extern "C" {
        fn WWDG();
        fn PVD_AVD();
        fn RTC();
        fn RTC_S();
        fn TAMP();
        fn RAMCFG();
        fn FLASH();
        fn FLASH_S();
        fn GTZC();
        fn RCC();
        fn RCC_S();
        fn EXTI0();
        fn EXTI1();
        fn EXTI2();
        fn EXTI3();
        fn EXTI4();
        fn EXTI5();
        fn EXTI6();
        fn EXTI7();
        fn EXTI8();
        fn EXTI9();
        fn EXTI10();
        fn EXTI11();
        fn EXTI12();
        fn EXTI13();
        fn EXTI14();
        fn EXTI15();
        fn GPDMA1_CHANNEL0();
        fn GPDMA1_CHANNEL1();
        fn GPDMA1_CHANNEL2();
        fn GPDMA1_CHANNEL3();
        fn GPDMA1_CHANNEL4();
        fn GPDMA1_CHANNEL5();
        fn GPDMA1_CHANNEL6();
        fn GPDMA1_CHANNEL7();
        fn IWDG();
        fn SAES();
        fn ADC1();
        fn DAC1();
        fn FDCAN1_IT0();
        fn FDCAN1_IT1();
        fn TIM1_BRK();
        fn TIM1_UP();
        fn TIM1_TRG_COM();
        fn TIM1_CC();
        fn TIM2();
        fn TIM3();
        fn TIM4();
        fn TIM5();
        fn TIM6();
        fn TIM7();
        fn I2C1_EV();
        fn I2C1_ER();
        fn I2C2_EV();
        fn I2C2_ER();
        fn SPI1();
        fn SPI2();
        fn SPI3();
        fn USART1();
        fn USART2();
        fn USART3();
        fn UART4();
        fn UART5();
        fn LPUART1();
        fn LPTIM1();
        fn TIM8_BRK();
        fn TIM8_UP();
        fn TIM8_TRG_COM();
        fn TIM8_CC();
        fn ADC2();
        fn LPTIM2();
        fn TIM15();
        fn TIM16();
        fn TIM17();
        fn USB_DRD_FS();
        fn CRS();
        fn UCPD1();
        fn FMC();
        fn OCTOSPI1();
        fn SDMMC1();
        fn I2C3_EV();
        fn I2C3_ER();
        fn SPI4();
        fn SPI5();
        fn SPI6();
        fn USART6();
        fn USART10();
        fn USART11();
        fn SAI1();
        fn SAI2();
        fn GPDMA2_CHANNEL0();
        fn GPDMA2_CHANNEL1();
        fn GPDMA2_CHANNEL2();
        fn GPDMA2_CHANNEL3();
        fn GPDMA2_CHANNEL4();
        fn GPDMA2_CHANNEL5();
        fn GPDMA2_CHANNEL6();
        fn GPDMA2_CHANNEL7();
        fn UART7();
        fn UART8();
        fn UART9();
        fn UART12();
        fn SDMMC2();
        fn FPU();
        fn ICACHE();
        fn DCACHE1();
        fn ETH();
        fn ETH_WKUP();
        fn DCMI_PSSI();
        fn FDCAN2_IT0();
        fn FDCAN2_IT1();
        fn CORDIC();
        fn FMAC();
        fn DTS();
        fn RNG();
        fn OTFDEC1();
        fn AES();
        fn HASH();
        fn PKA();
        fn CEC();
        fn TIM12();
        fn TIM13();
        fn TIM14();
        fn I3C1_EV();
        fn I3C1_ER();
        fn I2C4_EV();
        fn I2C4_ER();
        fn LPTIM3();
        fn LPTIM4();
        fn LPTIM5();
        fn LPTIM6();
    }
    pub union Vector {
        _handler: unsafe extern "C" fn(),
        _reserved: u32,
    }
    #[link_section = ".vector_table.interrupts"]
    #[no_mangle]
    pub static __INTERRUPTS: [Vector; 131] = [
        Vector { _handler: WWDG },
        Vector { _handler: PVD_AVD },
        Vector { _handler: RTC },
        Vector { _handler: RTC_S },
        Vector { _handler: TAMP },
        Vector { _handler: RAMCFG },
        Vector { _handler: FLASH },
        Vector { _handler: FLASH_S },
        Vector { _handler: GTZC },
        Vector { _handler: RCC },
        Vector { _handler: RCC_S },
        Vector { _handler: EXTI0 },
        Vector { _handler: EXTI1 },
        Vector { _handler: EXTI2 },
        Vector { _handler: EXTI3 },
        Vector { _handler: EXTI4 },
        Vector { _handler: EXTI5 },
        Vector { _handler: EXTI6 },
        Vector { _handler: EXTI7 },
        Vector { _handler: EXTI8 },
        Vector { _handler: EXTI9 },
        Vector { _handler: EXTI10 },
        Vector { _handler: EXTI11 },
        Vector { _handler: EXTI12 },
        Vector { _handler: EXTI13 },
        Vector { _handler: EXTI14 },
        Vector { _handler: EXTI15 },
        Vector {
            _handler: GPDMA1_CHANNEL0,
        },
        Vector {
            _handler: GPDMA1_CHANNEL1,
        },
        Vector {
            _handler: GPDMA1_CHANNEL2,
        },
        Vector {
            _handler: GPDMA1_CHANNEL3,
        },
        Vector {
            _handler: GPDMA1_CHANNEL4,
        },
        Vector {
            _handler: GPDMA1_CHANNEL5,
        },
        Vector {
            _handler: GPDMA1_CHANNEL6,
        },
        Vector {
            _handler: GPDMA1_CHANNEL7,
        },
        Vector { _handler: IWDG },
        Vector { _handler: SAES },
        Vector { _handler: ADC1 },
        Vector { _handler: DAC1 },
        Vector { _handler: FDCAN1_IT0 },
        Vector { _handler: FDCAN1_IT1 },
        Vector { _handler: TIM1_BRK },
        Vector { _handler: TIM1_UP },
        Vector { _handler: TIM1_TRG_COM },
        Vector { _handler: TIM1_CC },
        Vector { _handler: TIM2 },
        Vector { _handler: TIM3 },
        Vector { _handler: TIM4 },
        Vector { _handler: TIM5 },
        Vector { _handler: TIM6 },
        Vector { _handler: TIM7 },
        Vector { _handler: I2C1_EV },
        Vector { _handler: I2C1_ER },
        Vector { _handler: I2C2_EV },
        Vector { _handler: I2C2_ER },
        Vector { _handler: SPI1 },
        Vector { _handler: SPI2 },
        Vector { _handler: SPI3 },
        Vector { _handler: USART1 },
        Vector { _handler: USART2 },
        Vector { _handler: USART3 },
        Vector { _handler: UART4 },
        Vector { _handler: UART5 },
        Vector { _handler: LPUART1 },
        Vector { _handler: LPTIM1 },
        Vector { _handler: TIM8_BRK },
        Vector { _handler: TIM8_UP },
        Vector { _handler: TIM8_TRG_COM },
        Vector { _handler: TIM8_CC },
        Vector { _handler: ADC2 },
        Vector { _handler: LPTIM2 },
        Vector { _handler: TIM15 },
        Vector { _handler: TIM16 },
        Vector { _handler: TIM17 },
        Vector { _handler: USB_DRD_FS },
        Vector { _handler: CRS },
        Vector { _handler: UCPD1 },
        Vector { _handler: FMC },
        Vector { _handler: OCTOSPI1 },
        Vector { _handler: SDMMC1 },
        Vector { _handler: I2C3_EV },
        Vector { _handler: I2C3_ER },
        Vector { _handler: SPI4 },
        Vector { _handler: SPI5 },
        Vector { _handler: SPI6 },
        Vector { _handler: USART6 },
        Vector { _handler: USART10 },
        Vector { _handler: USART11 },
        Vector { _handler: SAI1 },
        Vector { _handler: SAI2 },
        Vector {
            _handler: GPDMA2_CHANNEL0,
        },
        Vector {
            _handler: GPDMA2_CHANNEL1,
        },
        Vector {
            _handler: GPDMA2_CHANNEL2,
        },
        Vector {
            _handler: GPDMA2_CHANNEL3,
        },
        Vector {
            _handler: GPDMA2_CHANNEL4,
        },
        Vector {
            _handler: GPDMA2_CHANNEL5,
        },
        Vector {
            _handler: GPDMA2_CHANNEL6,
        },
        Vector {
            _handler: GPDMA2_CHANNEL7,
        },
        Vector { _handler: UART7 },
        Vector { _handler: UART8 },
        Vector { _handler: UART9 },
        Vector { _handler: UART12 },
        Vector { _handler: SDMMC2 },
        Vector { _handler: FPU },
        Vector { _handler: ICACHE },
        Vector { _handler: DCACHE1 },
        Vector { _handler: ETH },
        Vector { _handler: ETH_WKUP },
        Vector { _handler: DCMI_PSSI },
        Vector { _handler: FDCAN2_IT0 },
        Vector { _handler: FDCAN2_IT1 },
        Vector { _handler: CORDIC },
        Vector { _handler: FMAC },
        Vector { _handler: DTS },
        Vector { _handler: RNG },
        Vector { _handler: OTFDEC1 },
        Vector { _handler: AES },
        Vector { _handler: HASH },
        Vector { _handler: PKA },
        Vector { _handler: CEC },
        Vector { _handler: TIM12 },
        Vector { _handler: TIM13 },
        Vector { _handler: TIM14 },
        Vector { _handler: I3C1_EV },
        Vector { _handler: I3C1_ER },
        Vector { _handler: I2C4_EV },
        Vector { _handler: I2C4_ER },
        Vector { _handler: LPTIM3 },
        Vector { _handler: LPTIM4 },
        Vector { _handler: LPTIM5 },
        Vector { _handler: LPTIM6 },
    ];
}
pub const UID: uid::Uid = unsafe { uid::Uid::from_ptr(0x08ff_f800 as usize as _) };
pub const TIM2: timer::TimGp32 = unsafe { timer::TimGp32::from_ptr(0x4000_0000 as usize as _) };
pub const TIM3: timer::TimGp16 = unsafe { timer::TimGp16::from_ptr(0x4000_0400 as usize as _) };
pub const TIM4: timer::TimGp16 = unsafe { timer::TimGp16::from_ptr(0x4000_0800 as usize as _) };
pub const TIM5: timer::TimGp32 = unsafe { timer::TimGp32::from_ptr(0x4000_0c00 as usize as _) };
pub const TIM6: timer::TimBasic = unsafe { timer::TimBasic::from_ptr(0x4000_1000 as usize as _) };
pub const TIM7: timer::TimBasic = unsafe { timer::TimBasic::from_ptr(0x4000_1400 as usize as _) };
pub const TIM12: timer::TimGp16 = unsafe { timer::TimGp16::from_ptr(0x4000_1800 as usize as _) };
pub const TIM13: timer::TimGp16 = unsafe { timer::TimGp16::from_ptr(0x4000_1c00 as usize as _) };
pub const TIM14: timer::TimGp16 = unsafe { timer::TimGp16::from_ptr(0x4000_2000 as usize as _) };
pub const WWDG: wwdg::Wwdg = unsafe { wwdg::Wwdg::from_ptr(0x4000_2c00 as usize as _) };
pub const IWDG: *mut () = 0x4000_3000 as usize as _;
pub const SPI2: spi::Spi = unsafe { spi::Spi::from_ptr(0x4000_3800 as usize as _) };
pub const SPI3: spi::Spi = unsafe { spi::Spi::from_ptr(0x4000_3c00 as usize as _) };
pub const USART2: usart::Usart = unsafe { usart::Usart::from_ptr(0x4000_4400 as usize as _) };
pub const USART3: usart::Usart = unsafe { usart::Usart::from_ptr(0x4000_4800 as usize as _) };
pub const UART4: usart::Usart = unsafe { usart::Usart::from_ptr(0x4000_4c00 as usize as _) };
pub const UART5: usart::Usart = unsafe { usart::Usart::from_ptr(0x4000_5000 as usize as _) };
pub const I2C1: i2c::I2c = unsafe { i2c::I2c::from_ptr(0x4000_5400 as usize as _) };
pub const I2C2: i2c::I2c = unsafe { i2c::I2c::from_ptr(0x4000_5800 as usize as _) };
pub const I3C1: *mut () = 0x4000_5c00 as usize as _;
pub const CRS: crs::Crs = unsafe { crs::Crs::from_ptr(0x4000_6000 as usize as _) };
pub const USART6: usart::Usart = unsafe { usart::Usart::from_ptr(0x4000_6400 as usize as _) };
pub const USART10: usart::Usart = unsafe { usart::Usart::from_ptr(0x4000_6800 as usize as _) };
pub const USART11: usart::Usart = unsafe { usart::Usart::from_ptr(0x4000_6c00 as usize as _) };
pub const CEC: *mut () = 0x4000_7000 as usize as _;
pub const UART7: usart::Usart = unsafe { usart::Usart::from_ptr(0x4000_7800 as usize as _) };
pub const UART8: usart::Usart = unsafe { usart::Usart::from_ptr(0x4000_7c00 as usize as _) };
pub const UART9: usart::Usart = unsafe { usart::Usart::from_ptr(0x4000_8000 as usize as _) };
pub const UART12: usart::Usart = unsafe { usart::Usart::from_ptr(0x4000_8400 as usize as _) };
pub const DTS: *mut () = 0x4000_8c00 as usize as _;
pub const LPTIM2: *mut () = 0x4000_9400 as usize as _;
pub const FDCAN1: *mut () = 0x4000_a400 as usize as _;
pub const FDCAN2: *mut () = 0x4000_a800 as usize as _;
pub const UCPD1: ucpd::Ucpd = unsafe { ucpd::Ucpd::from_ptr(0x4000_dc00 as usize as _) };
pub const TIM1: timer::TimAdv = unsafe { timer::TimAdv::from_ptr(0x4001_2c00 as usize as _) };
pub const SPI1: spi::Spi = unsafe { spi::Spi::from_ptr(0x4001_3000 as usize as _) };
pub const TIM8: timer::TimAdv = unsafe { timer::TimAdv::from_ptr(0x4001_3400 as usize as _) };
pub const USART1: usart::Usart = unsafe { usart::Usart::from_ptr(0x4001_3800 as usize as _) };
pub const TIM15: timer::TimGp16 = unsafe { timer::TimGp16::from_ptr(0x4001_4000 as usize as _) };
pub const TIM16: timer::TimGp16 = unsafe { timer::TimGp16::from_ptr(0x4001_4400 as usize as _) };
pub const TIM17: timer::TimGp16 = unsafe { timer::TimGp16::from_ptr(0x4001_4800 as usize as _) };
pub const SPI4: spi::Spi = unsafe { spi::Spi::from_ptr(0x4001_4c00 as usize as _) };
pub const SPI6: spi::Spi = unsafe { spi::Spi::from_ptr(0x4001_5000 as usize as _) };
pub const SAI1: *mut () = 0x4001_5400 as usize as _;
pub const SAI2: *mut () = 0x4001_5800 as usize as _;
pub const USB: usb::Usb = unsafe { usb::Usb::from_ptr(0x4001_6000 as usize as _) };
pub const USBRAM: usbram::Usbram = unsafe { usbram::Usbram::from_ptr(0x4001_6400 as usize as _) };
pub const GPDMA1: gpdma::Gpdma = unsafe { gpdma::Gpdma::from_ptr(0x4002_0000 as usize as _) };
pub const GPDMA2: gpdma::Gpdma = unsafe { gpdma::Gpdma::from_ptr(0x4002_1000 as usize as _) };
pub const FLASH: flash::Flash = unsafe { flash::Flash::from_ptr(0x4002_2000 as usize as _) };
pub const CRC: crc::Crc = unsafe { crc::Crc::from_ptr(0x4002_3000 as usize as _) };
pub const CORDIC: *mut () = 0x4002_3800 as usize as _;
pub const FMAC: fmac::Fmac = unsafe { fmac::Fmac::from_ptr(0x4002_3c00 as usize as _) };
pub const ETH: eth::Eth = unsafe { eth::Eth::from_ptr(0x4002_8000 as usize as _) };
pub const ICACHE: *mut () = 0x4003_0400 as usize as _;
pub const DCACHE1: *mut () = 0x4003_1400 as usize as _;
pub const GPIOA: gpio::Gpio = unsafe { gpio::Gpio::from_ptr(0x4202_0000 as usize as _) };
pub const GPIOB: gpio::Gpio = unsafe { gpio::Gpio::from_ptr(0x4202_0400 as usize as _) };
pub const GPIOC: gpio::Gpio = unsafe { gpio::Gpio::from_ptr(0x4202_0800 as usize as _) };
pub const GPIOD: gpio::Gpio = unsafe { gpio::Gpio::from_ptr(0x4202_0c00 as usize as _) };
pub const GPIOE: gpio::Gpio = unsafe { gpio::Gpio::from_ptr(0x4202_1000 as usize as _) };
pub const GPIOF: gpio::Gpio = unsafe { gpio::Gpio::from_ptr(0x4202_1400 as usize as _) };
pub const GPIOG: gpio::Gpio = unsafe { gpio::Gpio::from_ptr(0x4202_1800 as usize as _) };
pub const GPIOH: gpio::Gpio = unsafe { gpio::Gpio::from_ptr(0x4202_1c00 as usize as _) };
pub const GPIOI: gpio::Gpio = unsafe { gpio::Gpio::from_ptr(0x4202_2000 as usize as _) };
pub const ADC1: *mut () = 0x4202_8000 as usize as _;
pub const ADC2: *mut () = 0x4202_8100 as usize as _;
pub const ADC_COMMON: *mut () = 0x4202_8300 as usize as _;
pub const DAC1: *mut () = 0x4202_8400 as usize as _;
pub const DCMI: dcmi::Dcmi = unsafe { dcmi::Dcmi::from_ptr(0x4202_c000 as usize as _) };
pub const PSSI: *mut () = 0x4202_c400 as usize as _;
pub const AES: *mut () = 0x420c_0000 as usize as _;
pub const HASH: *mut () = 0x420c_0400 as usize as _;
pub const RNG: rng::Rng = unsafe { rng::Rng::from_ptr(0x420c_0800 as usize as _) };
pub const SAES: *mut () = 0x420c_0c00 as usize as _;
pub const PKA: *mut () = 0x420c_2000 as usize as _;
pub const SYSCFG: syscfg::Syscfg = unsafe { syscfg::Syscfg::from_ptr(0x4400_0400 as usize as _) };
pub const SPI5: spi::Spi = unsafe { spi::Spi::from_ptr(0x4400_2000 as usize as _) };
pub const LPUART1: usart::Lpuart = unsafe { usart::Lpuart::from_ptr(0x4400_2400 as usize as _) };
pub const I2C3: i2c::I2c = unsafe { i2c::I2c::from_ptr(0x4400_2800 as usize as _) };
pub const I2C4: i2c::I2c = unsafe { i2c::I2c::from_ptr(0x4400_2c00 as usize as _) };
pub const LPTIM1: *mut () = 0x4400_4400 as usize as _;
pub const LPTIM3: *mut () = 0x4400_4800 as usize as _;
pub const LPTIM4: *mut () = 0x4400_4c00 as usize as _;
pub const LPTIM5: *mut () = 0x4400_5000 as usize as _;
pub const LPTIM6: *mut () = 0x4400_5400 as usize as _;
pub const VREFBUF: *mut () = 0x4400_7400 as usize as _;
pub const RTC: rtc::Rtc = unsafe { rtc::Rtc::from_ptr(0x4400_7800 as usize as _) };
pub const TAMP: *mut () = 0x4400_7c00 as usize as _;
pub const PWR: pwr::Pwr = unsafe { pwr::Pwr::from_ptr(0x4402_0800 as usize as _) };
pub const RCC: rcc::Rcc = unsafe { rcc::Rcc::from_ptr(0x4402_0c00 as usize as _) };
pub const EXTI: exti::Exti = unsafe { exti::Exti::from_ptr(0x4402_2000 as usize as _) };
pub const DBGMCU: *mut () = 0x4402_4000 as usize as _;
pub const OTFDEC1: *mut () = 0x4600_5000 as usize as _;
pub const SDMMC1: sdmmc::Sdmmc = unsafe { sdmmc::Sdmmc::from_ptr(0x4600_8000 as usize as _) };
pub const SDMMC2: sdmmc::Sdmmc = unsafe { sdmmc::Sdmmc::from_ptr(0x4600_8c00 as usize as _) };
pub const FMC: *mut () = 0x6000_0000 as usize as _;
pub const OCTOSPI1: octospi::Octospi = unsafe { octospi::Octospi::from_ptr(0x9000_0000 as usize as _) };
#[doc = r" Number available in the NVIC for configuring priority"]
#[cfg(feature = "rt")]
pub const NVIC_PRIO_BITS: u8 = 4;
#[cfg(feature = "rt")]
pub use cortex_m_rt::interrupt;
#[cfg(feature = "rt")]
pub use Interrupt as interrupt;
pub fn GPIO(n: usize) -> gpio::Gpio {
    unsafe { gpio::Gpio::from_ptr((1107427328 + 1024 * n) as _) }
}
#[path = "../../peripherals/crc_v2.rs"]
pub mod crc;
#[path = "../../peripherals/crs_v1.rs"]
pub mod crs;
#[path = "../../peripherals/dcmi_v1.rs"]
pub mod dcmi;
#[path = "../../peripherals/eth_v2.rs"]
pub mod eth;
#[path = "../../peripherals/exti_h5.rs"]
pub mod exti;
#[path = "../../peripherals/flash_h5.rs"]
pub mod flash;
#[path = "../../peripherals/fmac_v1.rs"]
pub mod fmac;
#[path = "../../peripherals/gpdma_v1.rs"]
pub mod gpdma;
#[path = "../../peripherals/gpio_v2.rs"]
pub mod gpio;
#[path = "../../peripherals/i2c_v2.rs"]
pub mod i2c;
#[path = "../../peripherals/octospi_v2.rs"]
pub mod octospi;
#[path = "../../peripherals/pwr_h5.rs"]
pub mod pwr;
#[path = "../../peripherals/rcc_h5.rs"]
pub mod rcc;
#[path = "../../peripherals/rng_v3.rs"]
pub mod rng;
#[path = "../../peripherals/rtc_v3u5.rs"]
pub mod rtc;
#[path = "../../peripherals/sdmmc_v2.rs"]
pub mod sdmmc;
#[path = "../../peripherals/spi_v4.rs"]
pub mod spi;
#[path = "../../peripherals/syscfg_h5.rs"]
pub mod syscfg;
#[path = "../../peripherals/timer_v1.rs"]
pub mod timer;
#[path = "../../peripherals/ucpd_v1.rs"]
pub mod ucpd;
#[path = "../../peripherals/uid_v1.rs"]
pub mod uid;
#[path = "../../peripherals/usart_v4.rs"]
pub mod usart;
#[path = "../../peripherals/usb_v4.rs"]
pub mod usb;
#[path = "../../peripherals/usbram_32_2048.rs"]
pub mod usbram;
#[path = "../../peripherals/wwdg_v2.rs"]
pub mod wwdg;
pub const CORE_INDEX: usize = 0;
pub const FLASH_BASE: usize = 134217728;
pub const FLASH_SIZE: usize = 2097152;
pub const WRITE_SIZE: usize = 16;

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum Interrupt {
    #[doc = "0 - WWDG"]
    WWDG = 0,
    #[doc = "1 - PVD_PVM"]
    PVD_PVM = 1,
    #[doc = "2 - TAMP_STAMP"]
    TAMP_STAMP = 2,
    #[doc = "3 - RTC_WKUP"]
    RTC_WKUP = 3,
    #[doc = "4 - FLASH"]
    FLASH = 4,
    #[doc = "5 - RCC"]
    RCC = 5,
    #[doc = "6 - EXTI0"]
    EXTI0 = 6,
    #[doc = "7 - EXTI1"]
    EXTI1 = 7,
    #[doc = "8 - EXTI2"]
    EXTI2 = 8,
    #[doc = "9 - EXTI3"]
    EXTI3 = 9,
    #[doc = "10 - EXTI4"]
    EXTI4 = 10,
    #[doc = "11 - DMA1_CHANNEL1"]
    DMA1_CHANNEL1 = 11,
    #[doc = "12 - DMA1_CHANNEL2"]
    DMA1_CHANNEL2 = 12,
    #[doc = "13 - DMA1_CHANNEL3"]
    DMA1_CHANNEL3 = 13,
    #[doc = "14 - DMA1_CHANNEL4"]
    DMA1_CHANNEL4 = 14,
    #[doc = "15 - DMA1_CHANNEL5"]
    DMA1_CHANNEL5 = 15,
    #[doc = "16 - DMA1_CHANNEL6"]
    DMA1_CHANNEL6 = 16,
    #[doc = "17 - DMA1_CHANNEL7"]
    DMA1_CHANNEL7 = 17,
    #[doc = "18 - ADC1"]
    ADC1 = 18,
    #[doc = "19 - CAN1_TX"]
    CAN1_TX = 19,
    #[doc = "20 - CAN1_RX0"]
    CAN1_RX0 = 20,
    #[doc = "21 - CAN1_RX1"]
    CAN1_RX1 = 21,
    #[doc = "22 - CAN1_SCE"]
    CAN1_SCE = 22,
    #[doc = "23 - EXTI9_5"]
    EXTI9_5 = 23,
    #[doc = "24 - TIM1_BRK_TIM15"]
    TIM1_BRK_TIM15 = 24,
    #[doc = "25 - TIM1_UP_TIM16"]
    TIM1_UP_TIM16 = 25,
    #[doc = "26 - TIM1_TRG_COM_TIM17"]
    TIM1_TRG_COM_TIM17 = 26,
    #[doc = "27 - TIM1_CC"]
    TIM1_CC = 27,
    #[doc = "28 - TIM2"]
    TIM2 = 28,
    #[doc = "29 - TIM3"]
    TIM3 = 29,
    #[doc = "30 - TIM4"]
    TIM4 = 30,
    #[doc = "31 - I2C1_EV"]
    I2C1_EV = 31,
    #[doc = "32 - I2C1_ER"]
    I2C1_ER = 32,
    #[doc = "33 - I2C2_EV"]
    I2C2_EV = 33,
    #[doc = "34 - I2C2_ER"]
    I2C2_ER = 34,
    #[doc = "35 - SPI1"]
    SPI1 = 35,
    #[doc = "36 - SPI2"]
    SPI2 = 36,
    #[doc = "37 - USART1"]
    USART1 = 37,
    #[doc = "38 - USART2"]
    USART2 = 38,
    #[doc = "39 - USART3"]
    USART3 = 39,
    #[doc = "40 - EXTI15_10"]
    EXTI15_10 = 40,
    #[doc = "41 - RTC_ALARM"]
    RTC_ALARM = 41,
    #[doc = "42 - DFSDM1_FLT3"]
    DFSDM1_FLT3 = 42,
    #[doc = "43 - TIM8_BRK"]
    TIM8_BRK = 43,
    #[doc = "44 - TIM8_UP"]
    TIM8_UP = 44,
    #[doc = "45 - TIM8_TRG_COM"]
    TIM8_TRG_COM = 45,
    #[doc = "46 - TIM8_CC"]
    TIM8_CC = 46,
    #[doc = "48 - FMC"]
    FMC = 48,
    #[doc = "49 - SDMMC1"]
    SDMMC1 = 49,
    #[doc = "50 - TIM5"]
    TIM5 = 50,
    #[doc = "51 - SPI3"]
    SPI3 = 51,
    #[doc = "52 - UART4"]
    UART4 = 52,
    #[doc = "53 - UART5"]
    UART5 = 53,
    #[doc = "54 - TIM6_DAC"]
    TIM6_DAC = 54,
    #[doc = "55 - TIM7"]
    TIM7 = 55,
    #[doc = "56 - DMA2_CHANNEL1"]
    DMA2_CHANNEL1 = 56,
    #[doc = "57 - DMA2_CHANNEL2"]
    DMA2_CHANNEL2 = 57,
    #[doc = "58 - DMA2_CHANNEL3"]
    DMA2_CHANNEL3 = 58,
    #[doc = "59 - DMA2_CHANNEL4"]
    DMA2_CHANNEL4 = 59,
    #[doc = "60 - DMA2_CHANNEL5"]
    DMA2_CHANNEL5 = 60,
    #[doc = "61 - DFSDM1_FLT0"]
    DFSDM1_FLT0 = 61,
    #[doc = "62 - DFSDM1_FLT1"]
    DFSDM1_FLT1 = 62,
    #[doc = "63 - DFSDM1_FLT2"]
    DFSDM1_FLT2 = 63,
    #[doc = "64 - COMP"]
    COMP = 64,
    #[doc = "65 - LPTIM1"]
    LPTIM1 = 65,
    #[doc = "66 - LPTIM2"]
    LPTIM2 = 66,
    #[doc = "67 - OTG_FS"]
    OTG_FS = 67,
    #[doc = "68 - DMA2_CHANNEL6"]
    DMA2_CHANNEL6 = 68,
    #[doc = "69 - DMA2_CHANNEL7"]
    DMA2_CHANNEL7 = 69,
    #[doc = "70 - LPUART1"]
    LPUART1 = 70,
    #[doc = "71 - OCTOSPI1"]
    OCTOSPI1 = 71,
    #[doc = "72 - I2C3_EV"]
    I2C3_EV = 72,
    #[doc = "73 - I2C3_ER"]
    I2C3_ER = 73,
    #[doc = "74 - SAI1"]
    SAI1 = 74,
    #[doc = "75 - SAI2"]
    SAI2 = 75,
    #[doc = "76 - OCTOSPI2"]
    OCTOSPI2 = 76,
    #[doc = "77 - TSC"]
    TSC = 77,
    #[doc = "78 - DSI"]
    DSI = 78,
    #[doc = "79 - AES"]
    AES = 79,
    #[doc = "80 - RNG"]
    RNG = 80,
    #[doc = "81 - FPU"]
    FPU = 81,
    #[doc = "82 - HASH_CRS"]
    HASH_CRS = 82,
    #[doc = "83 - I2C4_ER"]
    I2C4_ER = 83,
    #[doc = "84 - I2C4_EV"]
    I2C4_EV = 84,
    #[doc = "85 - DCMI"]
    DCMI = 85,
    #[doc = "90 - DMA2D"]
    DMA2D = 90,
    #[doc = "91 - LTDC"]
    LTDC = 91,
    #[doc = "92 - LTDC_ER"]
    LTDC_ER = 92,
    #[doc = "93 - GFXMMU"]
    GFXMMU = 93,
    #[doc = "94 - DMAMUX1_OVR"]
    DMAMUX1_OVR = 94,
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
        fn PVD_PVM();
        fn TAMP_STAMP();
        fn RTC_WKUP();
        fn FLASH();
        fn RCC();
        fn EXTI0();
        fn EXTI1();
        fn EXTI2();
        fn EXTI3();
        fn EXTI4();
        fn DMA1_CHANNEL1();
        fn DMA1_CHANNEL2();
        fn DMA1_CHANNEL3();
        fn DMA1_CHANNEL4();
        fn DMA1_CHANNEL5();
        fn DMA1_CHANNEL6();
        fn DMA1_CHANNEL7();
        fn ADC1();
        fn CAN1_TX();
        fn CAN1_RX0();
        fn CAN1_RX1();
        fn CAN1_SCE();
        fn EXTI9_5();
        fn TIM1_BRK_TIM15();
        fn TIM1_UP_TIM16();
        fn TIM1_TRG_COM_TIM17();
        fn TIM1_CC();
        fn TIM2();
        fn TIM3();
        fn TIM4();
        fn I2C1_EV();
        fn I2C1_ER();
        fn I2C2_EV();
        fn I2C2_ER();
        fn SPI1();
        fn SPI2();
        fn USART1();
        fn USART2();
        fn USART3();
        fn EXTI15_10();
        fn RTC_ALARM();
        fn DFSDM1_FLT3();
        fn TIM8_BRK();
        fn TIM8_UP();
        fn TIM8_TRG_COM();
        fn TIM8_CC();
        fn FMC();
        fn SDMMC1();
        fn TIM5();
        fn SPI3();
        fn UART4();
        fn UART5();
        fn TIM6_DAC();
        fn TIM7();
        fn DMA2_CHANNEL1();
        fn DMA2_CHANNEL2();
        fn DMA2_CHANNEL3();
        fn DMA2_CHANNEL4();
        fn DMA2_CHANNEL5();
        fn DFSDM1_FLT0();
        fn DFSDM1_FLT1();
        fn DFSDM1_FLT2();
        fn COMP();
        fn LPTIM1();
        fn LPTIM2();
        fn OTG_FS();
        fn DMA2_CHANNEL6();
        fn DMA2_CHANNEL7();
        fn LPUART1();
        fn OCTOSPI1();
        fn I2C3_EV();
        fn I2C3_ER();
        fn SAI1();
        fn SAI2();
        fn OCTOSPI2();
        fn TSC();
        fn DSI();
        fn AES();
        fn RNG();
        fn FPU();
        fn HASH_CRS();
        fn I2C4_ER();
        fn I2C4_EV();
        fn DCMI();
        fn DMA2D();
        fn LTDC();
        fn LTDC_ER();
        fn GFXMMU();
        fn DMAMUX1_OVR();
    }
    pub union Vector {
        _handler: unsafe extern "C" fn(),
        _reserved: u32,
    }
    #[link_section = ".vector_table.interrupts"]
    #[no_mangle]
    pub static __INTERRUPTS: [Vector; 95] = [
        Vector { _handler: WWDG },
        Vector { _handler: PVD_PVM },
        Vector { _handler: TAMP_STAMP },
        Vector { _handler: RTC_WKUP },
        Vector { _handler: FLASH },
        Vector { _handler: RCC },
        Vector { _handler: EXTI0 },
        Vector { _handler: EXTI1 },
        Vector { _handler: EXTI2 },
        Vector { _handler: EXTI3 },
        Vector { _handler: EXTI4 },
        Vector {
            _handler: DMA1_CHANNEL1,
        },
        Vector {
            _handler: DMA1_CHANNEL2,
        },
        Vector {
            _handler: DMA1_CHANNEL3,
        },
        Vector {
            _handler: DMA1_CHANNEL4,
        },
        Vector {
            _handler: DMA1_CHANNEL5,
        },
        Vector {
            _handler: DMA1_CHANNEL6,
        },
        Vector {
            _handler: DMA1_CHANNEL7,
        },
        Vector { _handler: ADC1 },
        Vector { _handler: CAN1_TX },
        Vector { _handler: CAN1_RX0 },
        Vector { _handler: CAN1_RX1 },
        Vector { _handler: CAN1_SCE },
        Vector { _handler: EXTI9_5 },
        Vector {
            _handler: TIM1_BRK_TIM15,
        },
        Vector {
            _handler: TIM1_UP_TIM16,
        },
        Vector {
            _handler: TIM1_TRG_COM_TIM17,
        },
        Vector { _handler: TIM1_CC },
        Vector { _handler: TIM2 },
        Vector { _handler: TIM3 },
        Vector { _handler: TIM4 },
        Vector { _handler: I2C1_EV },
        Vector { _handler: I2C1_ER },
        Vector { _handler: I2C2_EV },
        Vector { _handler: I2C2_ER },
        Vector { _handler: SPI1 },
        Vector { _handler: SPI2 },
        Vector { _handler: USART1 },
        Vector { _handler: USART2 },
        Vector { _handler: USART3 },
        Vector { _handler: EXTI15_10 },
        Vector { _handler: RTC_ALARM },
        Vector { _handler: DFSDM1_FLT3 },
        Vector { _handler: TIM8_BRK },
        Vector { _handler: TIM8_UP },
        Vector { _handler: TIM8_TRG_COM },
        Vector { _handler: TIM8_CC },
        Vector { _reserved: 0 },
        Vector { _handler: FMC },
        Vector { _handler: SDMMC1 },
        Vector { _handler: TIM5 },
        Vector { _handler: SPI3 },
        Vector { _handler: UART4 },
        Vector { _handler: UART5 },
        Vector { _handler: TIM6_DAC },
        Vector { _handler: TIM7 },
        Vector {
            _handler: DMA2_CHANNEL1,
        },
        Vector {
            _handler: DMA2_CHANNEL2,
        },
        Vector {
            _handler: DMA2_CHANNEL3,
        },
        Vector {
            _handler: DMA2_CHANNEL4,
        },
        Vector {
            _handler: DMA2_CHANNEL5,
        },
        Vector { _handler: DFSDM1_FLT0 },
        Vector { _handler: DFSDM1_FLT1 },
        Vector { _handler: DFSDM1_FLT2 },
        Vector { _handler: COMP },
        Vector { _handler: LPTIM1 },
        Vector { _handler: LPTIM2 },
        Vector { _handler: OTG_FS },
        Vector {
            _handler: DMA2_CHANNEL6,
        },
        Vector {
            _handler: DMA2_CHANNEL7,
        },
        Vector { _handler: LPUART1 },
        Vector { _handler: OCTOSPI1 },
        Vector { _handler: I2C3_EV },
        Vector { _handler: I2C3_ER },
        Vector { _handler: SAI1 },
        Vector { _handler: SAI2 },
        Vector { _handler: OCTOSPI2 },
        Vector { _handler: TSC },
        Vector { _handler: DSI },
        Vector { _handler: AES },
        Vector { _handler: RNG },
        Vector { _handler: FPU },
        Vector { _handler: HASH_CRS },
        Vector { _handler: I2C4_ER },
        Vector { _handler: I2C4_EV },
        Vector { _handler: DCMI },
        Vector { _reserved: 0 },
        Vector { _reserved: 0 },
        Vector { _reserved: 0 },
        Vector { _reserved: 0 },
        Vector { _handler: DMA2D },
        Vector { _handler: LTDC },
        Vector { _handler: LTDC_ER },
        Vector { _handler: GFXMMU },
        Vector { _handler: DMAMUX1_OVR },
    ];
}
pub const UID: uid::Uid = unsafe { uid::Uid::from_ptr(0x1fff_7590 as usize as _) };
pub const VREFINTCAL: vrefintcal::Vrefintcal = unsafe { vrefintcal::Vrefintcal::from_ptr(0x1fff_75aa as usize as _) };
pub const TIM2: timer::TimGp32 = unsafe { timer::TimGp32::from_ptr(0x4000_0000 as usize as _) };
pub const TIM3: timer::TimGp16 = unsafe { timer::TimGp16::from_ptr(0x4000_0400 as usize as _) };
pub const TIM4: timer::TimGp16 = unsafe { timer::TimGp16::from_ptr(0x4000_0800 as usize as _) };
pub const TIM5: timer::TimGp32 = unsafe { timer::TimGp32::from_ptr(0x4000_0c00 as usize as _) };
pub const TIM6: timer::TimBasic = unsafe { timer::TimBasic::from_ptr(0x4000_1000 as usize as _) };
pub const TIM7: timer::TimBasic = unsafe { timer::TimBasic::from_ptr(0x4000_1400 as usize as _) };
pub const RTC: rtc::Rtc = unsafe { rtc::Rtc::from_ptr(0x4000_2800 as usize as _) };
pub const WWDG: wwdg::Wwdg = unsafe { wwdg::Wwdg::from_ptr(0x4000_2c00 as usize as _) };
pub const IWDG: iwdg::Iwdg = unsafe { iwdg::Iwdg::from_ptr(0x4000_3000 as usize as _) };
pub const SPI2: spi::Spi = unsafe { spi::Spi::from_ptr(0x4000_3800 as usize as _) };
pub const SPI3: spi::Spi = unsafe { spi::Spi::from_ptr(0x4000_3c00 as usize as _) };
pub const USART2: usart::Usart = unsafe { usart::Usart::from_ptr(0x4000_4400 as usize as _) };
pub const USART3: usart::Usart = unsafe { usart::Usart::from_ptr(0x4000_4800 as usize as _) };
pub const UART4: usart::Usart = unsafe { usart::Usart::from_ptr(0x4000_4c00 as usize as _) };
pub const UART5: usart::Usart = unsafe { usart::Usart::from_ptr(0x4000_5000 as usize as _) };
pub const I2C1: i2c::I2c = unsafe { i2c::I2c::from_ptr(0x4000_5400 as usize as _) };
pub const I2C2: i2c::I2c = unsafe { i2c::I2c::from_ptr(0x4000_5800 as usize as _) };
pub const I2C3: i2c::I2c = unsafe { i2c::I2c::from_ptr(0x4000_5c00 as usize as _) };
pub const CRS: *mut () = 0x4000_6000 as usize as _;
pub const CAN1: can::Can = unsafe { can::Can::from_ptr(0x4000_6400 as usize as _) };
pub const PWR: pwr::Pwr = unsafe { pwr::Pwr::from_ptr(0x4000_7000 as usize as _) };
pub const DAC1: dac::Dac = unsafe { dac::Dac::from_ptr(0x4000_7400 as usize as _) };
pub const OPAMP1: *mut () = 0x4000_7800 as usize as _;
pub const OPAMP2: *mut () = 0x4000_7810 as usize as _;
pub const LPTIM1: *mut () = 0x4000_7c00 as usize as _;
pub const LPUART1: usart::Lpuart = unsafe { usart::Lpuart::from_ptr(0x4000_8000 as usize as _) };
pub const I2C4: i2c::I2c = unsafe { i2c::I2c::from_ptr(0x4000_8400 as usize as _) };
pub const LPTIM2: *mut () = 0x4000_9400 as usize as _;
pub const SYSCFG: syscfg::Syscfg = unsafe { syscfg::Syscfg::from_ptr(0x4001_0000 as usize as _) };
pub const COMP1: *mut () = 0x4001_0200 as usize as _;
pub const COMP2: *mut () = 0x4001_0204 as usize as _;
pub const EXTI: exti::Exti = unsafe { exti::Exti::from_ptr(0x4001_0400 as usize as _) };
pub const TIM1: timer::TimAdv = unsafe { timer::TimAdv::from_ptr(0x4001_2c00 as usize as _) };
pub const SPI1: spi::Spi = unsafe { spi::Spi::from_ptr(0x4001_3000 as usize as _) };
pub const TIM8: timer::TimAdv = unsafe { timer::TimAdv::from_ptr(0x4001_3400 as usize as _) };
pub const USART1: usart::Usart = unsafe { usart::Usart::from_ptr(0x4001_3800 as usize as _) };
pub const TIM15: timer::TimGp16 = unsafe { timer::TimGp16::from_ptr(0x4001_4000 as usize as _) };
pub const TIM16: timer::TimGp16 = unsafe { timer::TimGp16::from_ptr(0x4001_4400 as usize as _) };
pub const TIM17: timer::TimGp16 = unsafe { timer::TimGp16::from_ptr(0x4001_4800 as usize as _) };
pub const SAI1: sai::Sai = unsafe { sai::Sai::from_ptr(0x4001_5400 as usize as _) };
pub const SAI2: sai::Sai = unsafe { sai::Sai::from_ptr(0x4001_5800 as usize as _) };
pub const DFSDM1: *mut () = 0x4001_6000 as usize as _;
pub const LTDC: ltdc::Ltdc = unsafe { ltdc::Ltdc::from_ptr(0x4001_6800 as usize as _) };
pub const DMA1: bdma::Dma = unsafe { bdma::Dma::from_ptr(0x4002_0000 as usize as _) };
pub const DMA2: bdma::Dma = unsafe { bdma::Dma::from_ptr(0x4002_0400 as usize as _) };
pub const DMAMUX1: dmamux::Dmamux = unsafe { dmamux::Dmamux::from_ptr(0x4002_0800 as usize as _) };
pub const RCC: rcc::Rcc = unsafe { rcc::Rcc::from_ptr(0x4002_1000 as usize as _) };
pub const FLASH: flash::Flash = unsafe { flash::Flash::from_ptr(0x4002_2000 as usize as _) };
pub const CRC: crc::Crc = unsafe { crc::Crc::from_ptr(0x4002_3000 as usize as _) };
pub const TSC: tsc::Tsc = unsafe { tsc::Tsc::from_ptr(0x4002_4000 as usize as _) };
pub const DMA2D: dma2d::Dma2d = unsafe { dma2d::Dma2d::from_ptr(0x4002_b000 as usize as _) };
pub const GFXMMU: gfxmmu::Gfxmmu = unsafe { gfxmmu::Gfxmmu::from_ptr(0x4002_c000 as usize as _) };
pub const GPIOA: gpio::Gpio = unsafe { gpio::Gpio::from_ptr(0x4800_0000 as usize as _) };
pub const GPIOB: gpio::Gpio = unsafe { gpio::Gpio::from_ptr(0x4800_0400 as usize as _) };
pub const GPIOC: gpio::Gpio = unsafe { gpio::Gpio::from_ptr(0x4800_0800 as usize as _) };
pub const GPIOD: gpio::Gpio = unsafe { gpio::Gpio::from_ptr(0x4800_0c00 as usize as _) };
pub const GPIOE: gpio::Gpio = unsafe { gpio::Gpio::from_ptr(0x4800_1000 as usize as _) };
pub const GPIOF: gpio::Gpio = unsafe { gpio::Gpio::from_ptr(0x4800_1400 as usize as _) };
pub const GPIOG: gpio::Gpio = unsafe { gpio::Gpio::from_ptr(0x4800_1800 as usize as _) };
pub const GPIOH: gpio::Gpio = unsafe { gpio::Gpio::from_ptr(0x4800_1c00 as usize as _) };
pub const GPIOI: gpio::Gpio = unsafe { gpio::Gpio::from_ptr(0x4800_2000 as usize as _) };
pub const USB_OTG_FS: otg::Otg = unsafe { otg::Otg::from_ptr(0x5000_0000 as usize as _) };
pub const ADC1: adc::Adc = unsafe { adc::Adc::from_ptr(0x5004_0000 as usize as _) };
pub const ADC_COMMON: adccommon::AdcCommon = unsafe { adccommon::AdcCommon::from_ptr(0x5004_0300 as usize as _) };
pub const DCMI: dcmi::Dcmi = unsafe { dcmi::Dcmi::from_ptr(0x5005_0000 as usize as _) };
pub const AES: aes::Aes = unsafe { aes::Aes::from_ptr(0x5006_0000 as usize as _) };
pub const HASH: *mut () = 0x5006_0400 as usize as _;
pub const RNG: rng::Rng = unsafe { rng::Rng::from_ptr(0x5006_0800 as usize as _) };
pub const OCTOSPIM: octospim::Octospim = unsafe { octospim::Octospim::from_ptr(0x5006_1c00 as usize as _) };
pub const SDMMC1: *mut () = 0x5006_2400 as usize as _;
pub const FMC: *mut () = 0x6000_0000 as usize as _;
pub const OCTOSPI2: octospi::Octospi = unsafe { octospi::Octospi::from_ptr(0x7000_0000 as usize as _) };
pub const OCTOSPI1: octospi::Octospi = unsafe { octospi::Octospi::from_ptr(0x9000_0000 as usize as _) };
pub const DBGMCU: dbgmcu::Dbgmcu = unsafe { dbgmcu::Dbgmcu::from_ptr(0xe004_2000 as usize as _) };
#[doc = r" Number available in the NVIC for configuring priority"]
#[cfg(feature = "rt")]
pub const NVIC_PRIO_BITS: u8 = 4;
#[cfg(feature = "rt")]
pub use cortex_m_rt::interrupt;
#[cfg(feature = "rt")]
pub use Interrupt as interrupt;
pub fn GPIO(n: usize) -> gpio::Gpio {
    unsafe { gpio::Gpio::from_ptr((1207959552 + 1024 * n) as _) }
}
#[path = "../../peripherals/adc_v3.rs"]
pub mod adc;
#[path = "../../peripherals/adccommon_v3.rs"]
pub mod adccommon;
#[path = "../../peripherals/aes_v1.rs"]
pub mod aes;
#[path = "../../peripherals/bdma_v1.rs"]
pub mod bdma;
#[path = "../../peripherals/can_bxcan.rs"]
pub mod can;
#[path = "../../peripherals/crc_v3.rs"]
pub mod crc;
#[path = "../../peripherals/dac_v2.rs"]
pub mod dac;
#[path = "../../peripherals/dbgmcu_l4.rs"]
pub mod dbgmcu;
#[path = "../../peripherals/dcmi_v1.rs"]
pub mod dcmi;
#[path = "../../peripherals/dma2d_v1.rs"]
pub mod dma2d;
#[path = "../../peripherals/dmamux_v1.rs"]
pub mod dmamux;
#[path = "../../peripherals/exti_v1.rs"]
pub mod exti;
#[path = "../../peripherals/flash_l4.rs"]
pub mod flash;
#[path = "../../peripherals/gfxmmu_v1.rs"]
pub mod gfxmmu;
#[path = "../../peripherals/gpio_v2.rs"]
pub mod gpio;
#[path = "../../peripherals/i2c_v2.rs"]
pub mod i2c;
#[path = "../../peripherals/iwdg_v2.rs"]
pub mod iwdg;
#[path = "../../peripherals/ltdc_v1.rs"]
pub mod ltdc;
#[path = "../../peripherals/octospi_v1.rs"]
pub mod octospi;
#[path = "../../peripherals/octospim_v1.rs"]
pub mod octospim;
#[path = "../../peripherals/otg_v1.rs"]
pub mod otg;
#[path = "../../peripherals/pwr_l4.rs"]
pub mod pwr;
#[path = "../../peripherals/rcc_l4plus.rs"]
pub mod rcc;
#[path = "../../peripherals/rng_v1.rs"]
pub mod rng;
#[path = "../../peripherals/rtc_v2l4.rs"]
pub mod rtc;
#[path = "../../peripherals/sai_v1.rs"]
pub mod sai;
#[path = "../../peripherals/spi_v2.rs"]
pub mod spi;
#[path = "../../peripherals/syscfg_l4.rs"]
pub mod syscfg;
#[path = "../../peripherals/timer_v1.rs"]
pub mod timer;
#[path = "../../peripherals/tsc_v3.rs"]
pub mod tsc;
#[path = "../../peripherals/uid_v1.rs"]
pub mod uid;
#[path = "../../peripherals/usart_v4.rs"]
pub mod usart;
#[path = "../../peripherals/vrefintcal_v1.rs"]
pub mod vrefintcal;
#[path = "../../peripherals/wwdg_v1.rs"]
pub mod wwdg;
pub const CORE_INDEX: usize = 0;
pub const FLASH_BASE: usize = 134217728;
pub const FLASH_SIZE: usize = 2097152;
pub const WRITE_SIZE: usize = 8;


pub(crate) static PERIPHERALS: &'static [Peripheral] = &[
    Peripheral {
        name: "ADC",
        address: 1073816576,
        registers: Some(PeripheralRegisters {
            kind: "adc",
            version: "g0",
            block: "ADC",
            ir: &adc::REGISTERS,
        }),
        rcc: Some(PeripheralRcc {
            clock: "pclk2",
            enable: Some(PeripheralRccRegister {
                register: "apb2enr",
                field: "adcen",
            }),
            reset: Some(PeripheralRccRegister {
                register: "apb2rstr",
                field: "adcrst",
            }),
            mux: Some(PeripheralRccRegister {
                register: "ccipr",
                field: "adcsel",
            }),
            stop_mode: StopMode::Stop1,
        }),
        pins: &[
            PeripheralPin {
                pin: "PA10",
                signal: "IN6",
                af: None,
            },
            PeripheralPin {
                pin: "PA11",
                signal: "IN7",
                af: None,
            },
            PeripheralPin {
                pin: "PA12",
                signal: "IN8",
                af: None,
            },
            PeripheralPin {
                pin: "PA13",
                signal: "IN9",
                af: None,
            },
            PeripheralPin {
                pin: "PA14",
                signal: "IN10",
                af: None,
            },
            PeripheralPin {
                pin: "PA15",
                signal: "IN11",
                af: None,
            },
            PeripheralPin {
                pin: "PB2",
                signal: "IN4",
                af: None,
            },
            PeripheralPin {
                pin: "PB3",
                signal: "IN2",
                af: None,
            },
            PeripheralPin {
                pin: "PB4",
                signal: "IN3",
                af: None,
            },
        ],
        dma_channels: &[PeripheralDmaChannel {
            signal: "ADC",
            channel: None,
            dmamux: Some("DMAMUX1"),
            dma: None,
            request: Some(5),
        }],
        interrupts: &[PeripheralInterrupt {
            signal: "GLOBAL",
            interrupt: "ADC",
        }],
    },
    Peripheral {
        name: "ADC_COMMON",
        address: 1073817352,
        registers: Some(PeripheralRegisters {
            kind: "adccommon",
            version: "v3",
            block: "ADC_COMMON",
            ir: &adccommon::REGISTERS,
        }),
        rcc: None,
        pins: &[],
        dma_channels: &[],
        interrupts: &[],
    },
    Peripheral {
        name: "AES",
        address: 1476401152,
        registers: Some(PeripheralRegisters {
            kind: "aes",
            version: "v2",
            block: "AES",
            ir: &aes::REGISTERS,
        }),
        rcc: Some(PeripheralRcc {
            clock: "hclk3",
            enable: Some(PeripheralRccRegister {
                register: "ahb3enr",
                field: "aesen",
            }),
            reset: Some(PeripheralRccRegister {
                register: "ahb3rstr",
                field: "aesrst",
            }),
            mux: None,
            stop_mode: StopMode::Stop1,
        }),
        pins: &[],
        dma_channels: &[
            PeripheralDmaChannel {
                signal: "IN",
                channel: None,
                dmamux: Some("DMAMUX1"),
                dma: None,
                request: Some(39),
            },
            PeripheralDmaChannel {
                signal: "OUT",
                channel: None,
                dmamux: Some("DMAMUX1"),
                dma: None,
                request: Some(40),
            },
        ],
        interrupts: &[PeripheralInterrupt {
            signal: "GLOBAL",
            interrupt: "AES",
        }],
    },
    Peripheral {
        name: "COMP1",
        address: 1073807872,
        registers: None,
        rcc: None,
        pins: &[
            PeripheralPin {
                pin: "PA0",
                signal: "OUT",
                af: Some(12),
            },
            PeripheralPin {
                pin: "PA10",
                signal: "INM",
                af: None,
            },
            PeripheralPin {
                pin: "PA11",
                signal: "INM",
                af: None,
            },
            PeripheralPin {
                pin: "PA15",
                signal: "INM",
                af: None,
            },
            PeripheralPin {
                pin: "PB0",
                signal: "OUT",
                af: Some(12),
            },
            PeripheralPin {
                pin: "PB2",
                signal: "INP",
                af: None,
            },
            PeripheralPin {
                pin: "PB3",
                signal: "INM",
                af: None,
            },
            PeripheralPin {
                pin: "PB4",
                signal: "INP",
                af: None,
            },
        ],
        dma_channels: &[],
        interrupts: &[PeripheralInterrupt {
            signal: "WKUP",
            interrupt: "COMP",
        }],
    },
    Peripheral {
        name: "COMP2",
        address: 1073807876,
        registers: None,
        rcc: None,
        pins: &[
            PeripheralPin {
                pin: "PA10",
                signal: "INM",
                af: None,
            },
            PeripheralPin {
                pin: "PA11",
                signal: "INM",
                af: None,
            },
            PeripheralPin {
                pin: "PA15",
                signal: "INP",
                af: None,
            },
            PeripheralPin {
                pin: "PA2",
                signal: "OUT",
                af: Some(12),
            },
            PeripheralPin {
                pin: "PA7",
                signal: "OUT",
                af: Some(12),
            },
            PeripheralPin {
                pin: "PB2",
                signal: "INM",
                af: None,
            },
            PeripheralPin {
                pin: "PB3",
                signal: "INM",
                af: None,
            },
            PeripheralPin {
                pin: "PB4",
                signal: "INP",
                af: None,
            },
            PeripheralPin {
                pin: "PB5",
                signal: "OUT",
                af: Some(12),
            },
        ],
        dma_channels: &[],
        interrupts: &[PeripheralInterrupt {
            signal: "WKUP",
            interrupt: "COMP",
        }],
    },
    Peripheral {
        name: "CRC",
        address: 1073885184,
        registers: Some(PeripheralRegisters {
            kind: "crc",
            version: "v2",
            block: "CRC",
            ir: &crc::REGISTERS,
        }),
        rcc: Some(PeripheralRcc {
            clock: "hclk1",
            enable: Some(PeripheralRccRegister {
                register: "ahb1enr",
                field: "crcen",
            }),
            reset: Some(PeripheralRccRegister {
                register: "ahb1rstr",
                field: "crcrst",
            }),
            mux: None,
            stop_mode: StopMode::Stop1,
        }),
        pins: &[],
        dma_channels: &[],
        interrupts: &[],
    },
    Peripheral {
        name: "DAC",
        address: 1073771520,
        registers: None,
        rcc: None,
        pins: &[PeripheralPin {
            pin: "PA10",
            signal: "OUT1",
            af: None,
        }],
        dma_channels: &[PeripheralDmaChannel {
            signal: "OUT1",
            channel: None,
            dmamux: Some("DMAMUX1"),
            dma: None,
            request: Some(6),
        }],
        interrupts: &[PeripheralInterrupt {
            signal: "GLOBAL",
            interrupt: "DAC",
        }],
    },
    Peripheral {
        name: "DBGMCU",
        address: 3758366720,
        registers: Some(PeripheralRegisters {
            kind: "dbgmcu",
            version: "wl",
            block: "DBGMCU",
            ir: &dbgmcu::REGISTERS,
        }),
        rcc: None,
        pins: &[],
        dma_channels: &[],
        interrupts: &[],
    },
    Peripheral {
        name: "DMA1",
        address: 1073872896,
        registers: Some(PeripheralRegisters {
            kind: "bdma",
            version: "v1",
            block: "DMA",
            ir: &bdma::REGISTERS,
        }),
        rcc: Some(PeripheralRcc {
            clock: "hclk1",
            enable: Some(PeripheralRccRegister {
                register: "ahb1enr",
                field: "dma1en",
            }),
            reset: Some(PeripheralRccRegister {
                register: "ahb1rstr",
                field: "dma1rst",
            }),
            mux: None,
            stop_mode: StopMode::Stop1,
        }),
        pins: &[],
        dma_channels: &[],
        interrupts: &[
            PeripheralInterrupt {
                signal: "CH1",
                interrupt: "DMA1_CHANNEL1",
            },
            PeripheralInterrupt {
                signal: "CH2",
                interrupt: "DMA1_CHANNEL2",
            },
            PeripheralInterrupt {
                signal: "CH3",
                interrupt: "DMA1_CHANNEL3",
            },
            PeripheralInterrupt {
                signal: "CH4",
                interrupt: "DMA1_CHANNEL4",
            },
            PeripheralInterrupt {
                signal: "CH5",
                interrupt: "DMA1_CHANNEL5",
            },
            PeripheralInterrupt {
                signal: "CH6",
                interrupt: "DMA1_CHANNEL6",
            },
            PeripheralInterrupt {
                signal: "CH7",
                interrupt: "DMA1_CHANNEL7",
            },
        ],
    },
    Peripheral {
        name: "DMA2",
        address: 1073873920,
        registers: Some(PeripheralRegisters {
            kind: "bdma",
            version: "v1",
            block: "DMA",
            ir: &bdma::REGISTERS,
        }),
        rcc: Some(PeripheralRcc {
            clock: "hclk1",
            enable: Some(PeripheralRccRegister {
                register: "ahb1enr",
                field: "dma2en",
            }),
            reset: Some(PeripheralRccRegister {
                register: "ahb1rstr",
                field: "dma2rst",
            }),
            mux: None,
            stop_mode: StopMode::Stop1,
        }),
        pins: &[],
        dma_channels: &[],
        interrupts: &[
            PeripheralInterrupt {
                signal: "CH1",
                interrupt: "DMA2_CHANNEL1",
            },
            PeripheralInterrupt {
                signal: "CH2",
                interrupt: "DMA2_CHANNEL2",
            },
            PeripheralInterrupt {
                signal: "CH3",
                interrupt: "DMA2_CHANNEL3",
            },
            PeripheralInterrupt {
                signal: "CH4",
                interrupt: "DMA2_CHANNEL4",
            },
            PeripheralInterrupt {
                signal: "CH5",
                interrupt: "DMA2_CHANNEL5",
            },
            PeripheralInterrupt {
                signal: "CH6",
                interrupt: "DMA2_CHANNEL6",
            },
            PeripheralInterrupt {
                signal: "CH7",
                interrupt: "DMA2_CHANNEL7",
            },
        ],
    },
    Peripheral {
        name: "DMAMUX1",
        address: 1073874944,
        registers: Some(PeripheralRegisters {
            kind: "dmamux",
            version: "v1",
            block: "DMAMUX",
            ir: &dmamux::REGISTERS,
        }),
        rcc: Some(PeripheralRcc {
            clock: "hclk1",
            enable: Some(PeripheralRccRegister {
                register: "ahb1enr",
                field: "dmamux1en",
            }),
            reset: Some(PeripheralRccRegister {
                register: "ahb1rstr",
                field: "dmamux1rst",
            }),
            mux: None,
            stop_mode: StopMode::Stop1,
        }),
        pins: &[],
        dma_channels: &[],
        interrupts: &[PeripheralInterrupt {
            signal: "OVR",
            interrupt: "DMAMUX1_OVR",
        }],
    },
    Peripheral {
        name: "EXTI",
        address: 1476397056,
        registers: Some(PeripheralRegisters {
            kind: "exti",
            version: "wle",
            block: "EXTI",
            ir: &exti::REGISTERS,
        }),
        rcc: None,
        pins: &[],
        dma_channels: &[],
        interrupts: &[
            PeripheralInterrupt {
                signal: "EXTI0",
                interrupt: "EXTI0",
            },
            PeripheralInterrupt {
                signal: "EXTI1",
                interrupt: "EXTI1",
            },
            PeripheralInterrupt {
                signal: "EXTI10",
                interrupt: "EXTI15_10",
            },
            PeripheralInterrupt {
                signal: "EXTI11",
                interrupt: "EXTI15_10",
            },
            PeripheralInterrupt {
                signal: "EXTI12",
                interrupt: "EXTI15_10",
            },
            PeripheralInterrupt {
                signal: "EXTI13",
                interrupt: "EXTI15_10",
            },
            PeripheralInterrupt {
                signal: "EXTI14",
                interrupt: "EXTI15_10",
            },
            PeripheralInterrupt {
                signal: "EXTI15",
                interrupt: "EXTI15_10",
            },
            PeripheralInterrupt {
                signal: "EXTI2",
                interrupt: "EXTI2",
            },
            PeripheralInterrupt {
                signal: "EXTI3",
                interrupt: "EXTI3",
            },
            PeripheralInterrupt {
                signal: "EXTI4",
                interrupt: "EXTI4",
            },
            PeripheralInterrupt {
                signal: "EXTI5",
                interrupt: "EXTI9_5",
            },
            PeripheralInterrupt {
                signal: "EXTI6",
                interrupt: "EXTI9_5",
            },
            PeripheralInterrupt {
                signal: "EXTI7",
                interrupt: "EXTI9_5",
            },
            PeripheralInterrupt {
                signal: "EXTI8",
                interrupt: "EXTI9_5",
            },
            PeripheralInterrupt {
                signal: "EXTI9",
                interrupt: "EXTI9_5",
            },
        ],
    },
    Peripheral {
        name: "FLASH",
        address: 1476411392,
        registers: Some(PeripheralRegisters {
            kind: "flash",
            version: "wl",
            block: "FLASH",
            ir: &flash::REGISTERS,
        }),
        rcc: Some(PeripheralRcc {
            clock: "hclk3",
            enable: Some(PeripheralRccRegister {
                register: "ahb3enr",
                field: "flashen",
            }),
            reset: Some(PeripheralRccRegister {
                register: "ahb3rstr",
                field: "flashrst",
            }),
            mux: None,
            stop_mode: StopMode::Stop1,
        }),
        pins: &[],
        dma_channels: &[],
        interrupts: &[PeripheralInterrupt {
            signal: "GLOBAL",
            interrupt: "FLASH",
        }],
    },
    Peripheral {
        name: "GPIOA",
        address: 1207959552,
        registers: Some(PeripheralRegisters {
            kind: "gpio",
            version: "v2",
            block: "GPIO",
            ir: &gpio::REGISTERS,
        }),
        rcc: Some(PeripheralRcc {
            clock: "hclk2",
            enable: Some(PeripheralRccRegister {
                register: "ahb2enr",
                field: "gpioaen",
            }),
            reset: Some(PeripheralRccRegister {
                register: "ahb2rstr",
                field: "gpioarst",
            }),
            mux: None,
            stop_mode: StopMode::Stop1,
        }),
        pins: &[],
        dma_channels: &[],
        interrupts: &[],
    },
    Peripheral {
        name: "GPIOB",
        address: 1207960576,
        registers: Some(PeripheralRegisters {
            kind: "gpio",
            version: "v2",
            block: "GPIO",
            ir: &gpio::REGISTERS,
        }),
        rcc: Some(PeripheralRcc {
            clock: "hclk2",
            enable: Some(PeripheralRccRegister {
                register: "ahb2enr",
                field: "gpioben",
            }),
            reset: Some(PeripheralRccRegister {
                register: "ahb2rstr",
                field: "gpiobrst",
            }),
            mux: None,
            stop_mode: StopMode::Stop1,
        }),
        pins: &[],
        dma_channels: &[],
        interrupts: &[],
    },
    Peripheral {
        name: "GPIOC",
        address: 1207961600,
        registers: Some(PeripheralRegisters {
            kind: "gpio",
            version: "v2",
            block: "GPIO",
            ir: &gpio::REGISTERS,
        }),
        rcc: Some(PeripheralRcc {
            clock: "hclk2",
            enable: Some(PeripheralRccRegister {
                register: "ahb2enr",
                field: "gpiocen",
            }),
            reset: Some(PeripheralRccRegister {
                register: "ahb2rstr",
                field: "gpiocrst",
            }),
            mux: None,
            stop_mode: StopMode::Stop1,
        }),
        pins: &[],
        dma_channels: &[],
        interrupts: &[],
    },
    Peripheral {
        name: "GPIOH",
        address: 1207966720,
        registers: Some(PeripheralRegisters {
            kind: "gpio",
            version: "v2",
            block: "GPIO",
            ir: &gpio::REGISTERS,
        }),
        rcc: Some(PeripheralRcc {
            clock: "hclk2",
            enable: Some(PeripheralRccRegister {
                register: "ahb2enr",
                field: "gpiohen",
            }),
            reset: Some(PeripheralRccRegister {
                register: "ahb2rstr",
                field: "gpiohrst",
            }),
            mux: None,
            stop_mode: StopMode::Stop1,
        }),
        pins: &[],
        dma_channels: &[],
        interrupts: &[],
    },
    Peripheral {
        name: "HSEM",
        address: 1476400128,
        registers: None,
        rcc: Some(PeripheralRcc {
            clock: "hclk3",
            enable: Some(PeripheralRccRegister {
                register: "ahb3enr",
                field: "hsemen",
            }),
            reset: Some(PeripheralRccRegister {
                register: "ahb3rstr",
                field: "hsemrst",
            }),
            mux: None,
            stop_mode: StopMode::Stop1,
        }),
        pins: &[],
        dma_channels: &[],
        interrupts: &[PeripheralInterrupt {
            signal: "GLOBAL",
            interrupt: "HSEM",
        }],
    },
    Peripheral {
        name: "I2C1",
        address: 1073763328,
        registers: Some(PeripheralRegisters {
            kind: "i2c",
            version: "v2",
            block: "I2C",
            ir: &i2c::REGISTERS,
        }),
        rcc: Some(PeripheralRcc {
            clock: "pclk1",
            enable: Some(PeripheralRccRegister {
                register: "apb1enr1",
                field: "i2c1en",
            }),
            reset: Some(PeripheralRccRegister {
                register: "apb1rstr1",
                field: "i2c1rst",
            }),
            mux: Some(PeripheralRccRegister {
                register: "ccipr",
                field: "i2c1sel",
            }),
            stop_mode: StopMode::Stop1,
        }),
        pins: &[
            PeripheralPin {
                pin: "PA1",
                signal: "SMBA",
                af: Some(4),
            },
            PeripheralPin {
                pin: "PA10",
                signal: "SDA",
                af: Some(4),
            },
            PeripheralPin {
                pin: "PA14",
                signal: "SMBA",
                af: Some(4),
            },
            PeripheralPin {
                pin: "PA9",
                signal: "SCL",
                af: Some(4),
            },
            PeripheralPin {
                pin: "PB5",
                signal: "SMBA",
                af: Some(4),
            },
            PeripheralPin {
                pin: "PB6",
                signal: "SCL",
                af: Some(4),
            },
            PeripheralPin {
                pin: "PB7",
                signal: "SDA",
                af: Some(4),
            },
            PeripheralPin {
                pin: "PB8",
                signal: "SCL",
                af: Some(4),
            },
        ],
        dma_channels: &[
            PeripheralDmaChannel {
                signal: "RX",
                channel: None,
                dmamux: Some("DMAMUX1"),
                dma: None,
                request: Some(11),
            },
            PeripheralDmaChannel {
                signal: "TX",
                channel: None,
                dmamux: Some("DMAMUX1"),
                dma: None,
                request: Some(12),
            },
        ],
        interrupts: &[
            PeripheralInterrupt {
                signal: "ER",
                interrupt: "I2C1_ER",
            },
            PeripheralInterrupt {
                signal: "EV",
                interrupt: "I2C1_EV",
            },
        ],
    },
    Peripheral {
        name: "I2C2",
        address: 1073764352,
        registers: Some(PeripheralRegisters {
            kind: "i2c",
            version: "v2",
            block: "I2C",
            ir: &i2c::REGISTERS,
        }),
        rcc: Some(PeripheralRcc {
            clock: "pclk1",
            enable: Some(PeripheralRccRegister {
                register: "apb1enr1",
                field: "i2c2en",
            }),
            reset: Some(PeripheralRccRegister {
                register: "apb1rstr1",
                field: "i2c2rst",
            }),
            mux: Some(PeripheralRccRegister {
                register: "ccipr",
                field: "i2c2sel",
            }),
            stop_mode: StopMode::Stop1,
        }),
        pins: &[
            PeripheralPin {
                pin: "PA11",
                signal: "SDA",
                af: Some(4),
            },
            PeripheralPin {
                pin: "PA12",
                signal: "SCL",
                af: Some(4),
            },
            PeripheralPin {
                pin: "PA13",
                signal: "SMBA",
                af: Some(4),
            },
            PeripheralPin {
                pin: "PA15",
                signal: "SDA",
                af: Some(4),
            },
            PeripheralPin {
                pin: "PA6",
                signal: "SMBA",
                af: Some(4),
            },
        ],
        dma_channels: &[
            PeripheralDmaChannel {
                signal: "RX",
                channel: None,
                dmamux: Some("DMAMUX1"),
                dma: None,
                request: Some(13),
            },
            PeripheralDmaChannel {
                signal: "TX",
                channel: None,
                dmamux: Some("DMAMUX1"),
                dma: None,
                request: Some(14),
            },
        ],
        interrupts: &[
            PeripheralInterrupt {
                signal: "ER",
                interrupt: "I2C2_ER",
            },
            PeripheralInterrupt {
                signal: "EV",
                interrupt: "I2C2_EV",
            },
        ],
    },
    Peripheral {
        name: "I2C3",
        address: 1073765376,
        registers: Some(PeripheralRegisters {
            kind: "i2c",
            version: "v2",
            block: "I2C",
            ir: &i2c::REGISTERS,
        }),
        rcc: Some(PeripheralRcc {
            clock: "pclk1",
            enable: Some(PeripheralRccRegister {
                register: "apb1enr1",
                field: "i2c3en",
            }),
            reset: Some(PeripheralRccRegister {
                register: "apb1rstr1",
                field: "i2c3rst",
            }),
            mux: Some(PeripheralRccRegister {
                register: "ccipr",
                field: "i2c3sel",
            }),
            stop_mode: StopMode::Stop1,
        }),
        pins: &[
            PeripheralPin {
                pin: "PA0",
                signal: "SMBA",
                af: Some(4),
            },
            PeripheralPin {
                pin: "PA7",
                signal: "SCL",
                af: Some(4),
            },
            PeripheralPin {
                pin: "PB12",
                signal: "SMBA",
                af: Some(4),
            },
            PeripheralPin {
                pin: "PB2",
                signal: "SMBA",
                af: Some(4),
            },
            PeripheralPin {
                pin: "PB4",
                signal: "SDA",
                af: Some(4),
            },
        ],
        dma_channels: &[
            PeripheralDmaChannel {
                signal: "RX",
                channel: None,
                dmamux: Some("DMAMUX1"),
                dma: None,
                request: Some(15),
            },
            PeripheralDmaChannel {
                signal: "TX",
                channel: None,
                dmamux: Some("DMAMUX1"),
                dma: None,
                request: Some(16),
            },
        ],
        interrupts: &[
            PeripheralInterrupt {
                signal: "ER",
                interrupt: "I2C3_ER",
            },
            PeripheralInterrupt {
                signal: "EV",
                interrupt: "I2C3_EV",
            },
        ],
    },
    Peripheral {
        name: "IWDG",
        address: 1073754112,
        registers: Some(PeripheralRegisters {
            kind: "iwdg",
            version: "v2",
            block: "IWDG",
            ir: &iwdg::REGISTERS,
        }),
        rcc: None,
        pins: &[],
        dma_channels: &[],
        interrupts: &[],
    },
    Peripheral {
        name: "LPTIM1",
        address: 1073773568,
        registers: None,
        rcc: Some(PeripheralRcc {
            clock: "pclk1",
            enable: Some(PeripheralRccRegister {
                register: "apb1enr1",
                field: "lptim1en",
            }),
            reset: Some(PeripheralRccRegister {
                register: "apb1rstr1",
                field: "lptim1rst",
            }),
            mux: Some(PeripheralRccRegister {
                register: "ccipr",
                field: "lptim1sel",
            }),
            stop_mode: StopMode::Stop2,
        }),
        pins: &[
            PeripheralPin {
                pin: "PA14",
                signal: "OUT",
                af: Some(1),
            },
            PeripheralPin {
                pin: "PA4",
                signal: "OUT",
                af: Some(1),
            },
            PeripheralPin {
                pin: "PB2",
                signal: "OUT",
                af: Some(1),
            },
            PeripheralPin {
                pin: "PB5",
                signal: "IN1",
                af: Some(1),
            },
            PeripheralPin {
                pin: "PB6",
                signal: "ETR",
                af: Some(1),
            },
            PeripheralPin {
                pin: "PB7",
                signal: "IN2",
                af: Some(1),
            },
        ],
        dma_channels: &[],
        interrupts: &[PeripheralInterrupt {
            signal: "GLOBAL",
            interrupt: "LPTIM1",
        }],
    },
    Peripheral {
        name: "LPTIM2",
        address: 1073779712,
        registers: None,
        rcc: Some(PeripheralRcc {
            clock: "pclk1",
            enable: Some(PeripheralRccRegister {
                register: "apb1enr2",
                field: "lptim2en",
            }),
            reset: Some(PeripheralRccRegister {
                register: "apb1rstr2",
                field: "lptim2rst",
            }),
            mux: Some(PeripheralRccRegister {
                register: "ccipr",
                field: "lptim2sel",
            }),
            stop_mode: StopMode::Stop2,
        }),
        pins: &[
            PeripheralPin {
                pin: "PA4",
                signal: "OUT",
                af: Some(14),
            },
            PeripheralPin {
                pin: "PA5",
                signal: "ETR",
                af: Some(14),
            },
            PeripheralPin {
                pin: "PA8",
                signal: "OUT",
                af: Some(14),
            },
        ],
        dma_channels: &[],
        interrupts: &[PeripheralInterrupt {
            signal: "GLOBAL",
            interrupt: "LPTIM2",
        }],
    },
    Peripheral {
        name: "LPTIM3",
        address: 1073780736,
        registers: None,
        rcc: Some(PeripheralRcc {
            clock: "pclk1",
            enable: Some(PeripheralRccRegister {
                register: "apb1enr2",
                field: "lptim3en",
            }),
            reset: Some(PeripheralRccRegister {
                register: "apb1rstr2",
                field: "lptim3rst",
            }),
            mux: Some(PeripheralRccRegister {
                register: "ccipr",
                field: "lptim3sel",
            }),
            stop_mode: StopMode::Stop2,
        }),
        pins: &[
            PeripheralPin {
                pin: "PA1",
                signal: "OUT",
                af: Some(3),
            },
            PeripheralPin {
                pin: "PA11",
                signal: "ETR",
                af: Some(3),
            },
            PeripheralPin {
                pin: "PA12",
                signal: "IN1",
                af: Some(3),
            },
        ],
        dma_channels: &[],
        interrupts: &[PeripheralInterrupt {
            signal: "GLOBAL",
            interrupt: "LPTIM3",
        }],
    },
    Peripheral {
        name: "LPUART1",
        address: 1073774592,
        registers: Some(PeripheralRegisters {
            kind: "usart",
            version: "v4",
            block: "LPUART",
            ir: &usart::REGISTERS,
        }),
        rcc: Some(PeripheralRcc {
            clock: "pclk1",
            enable: Some(PeripheralRccRegister {
                register: "apb1enr2",
                field: "lpuart1en",
            }),
            reset: Some(PeripheralRccRegister {
                register: "apb1rstr2",
                field: "lpuart1rst",
            }),
            mux: Some(PeripheralRccRegister {
                register: "ccipr",
                field: "lpuart1sel",
            }),
            stop_mode: StopMode::Stop2,
        }),
        pins: &[
            PeripheralPin {
                pin: "PA1",
                signal: "RTS",
                af: Some(8),
            },
            PeripheralPin {
                pin: "PA2",
                signal: "TX",
                af: Some(8),
            },
            PeripheralPin {
                pin: "PA3",
                signal: "RX",
                af: Some(8),
            },
            PeripheralPin {
                pin: "PA6",
                signal: "CTS",
                af: Some(8),
            },
            PeripheralPin {
                pin: "PB12",
                signal: "RTS",
                af: Some(8),
            },
        ],
        dma_channels: &[
            PeripheralDmaChannel {
                signal: "RX",
                channel: None,
                dmamux: Some("DMAMUX1"),
                dma: None,
                request: Some(21),
            },
            PeripheralDmaChannel {
                signal: "TX",
                channel: None,
                dmamux: Some("DMAMUX1"),
                dma: None,
                request: Some(22),
            },
        ],
        interrupts: &[PeripheralInterrupt {
            signal: "GLOBAL",
            interrupt: "LPUART1",
        }],
    },
    Peripheral {
        name: "PKA",
        address: 1476403200,
        registers: None,
        rcc: Some(PeripheralRcc {
            clock: "hclk3",
            enable: Some(PeripheralRccRegister {
                register: "ahb3enr",
                field: "pkaen",
            }),
            reset: Some(PeripheralRccRegister {
                register: "ahb3rstr",
                field: "pkarst",
            }),
            mux: None,
            stop_mode: StopMode::Stop1,
        }),
        pins: &[],
        dma_channels: &[],
        interrupts: &[PeripheralInterrupt {
            signal: "GLOBAL",
            interrupt: "PKA",
        }],
    },
    Peripheral {
        name: "PWR",
        address: 1476396032,
        registers: Some(PeripheralRegisters {
            kind: "pwr",
            version: "wl5",
            block: "PWR",
            ir: &pwr::REGISTERS,
        }),
        rcc: None,
        pins: &[
            PeripheralPin {
                pin: "PA0",
                signal: "WKUP1",
                af: None,
            },
            PeripheralPin {
                pin: "PB3",
                signal: "WKUP3",
                af: None,
            },
            PeripheralPin {
                pin: "PB7",
                signal: "PVD_IN",
                af: None,
            },
            PeripheralPin {
                pin: "PC13",
                signal: "WKUP2",
                af: None,
            },
        ],
        dma_channels: &[],
        interrupts: &[],
    },
    Peripheral {
        name: "RCC",
        address: 1476395008,
        registers: Some(PeripheralRegisters {
            kind: "rcc",
            version: "wle",
            block: "RCC",
            ir: &rcc::REGISTERS,
        }),
        rcc: None,
        pins: &[
            PeripheralPin {
                pin: "PA2",
                signal: "LSCO",
                af: Some(0),
            },
            PeripheralPin {
                pin: "PA8",
                signal: "MCO",
                af: Some(0),
            },
            PeripheralPin {
                pin: "PC14",
                signal: "OSC32_IN",
                af: None,
            },
            PeripheralPin {
                pin: "PC15",
                signal: "OSC32_OUT",
                af: None,
            },
        ],
        dma_channels: &[],
        interrupts: &[
            PeripheralInterrupt {
                signal: "GLOBAL",
                interrupt: "RCC",
            },
            PeripheralInterrupt {
                signal: "LSECSS",
                interrupt: "TAMP_STAMP_LSECSS_SSRU",
            },
        ],
    },
    Peripheral {
        name: "RNG",
        address: 1476399104,
        registers: Some(PeripheralRegisters {
            kind: "rng",
            version: "v2",
            block: "RNG",
            ir: &rng::REGISTERS,
        }),
        rcc: Some(PeripheralRcc {
            clock: "hclk3",
            enable: Some(PeripheralRccRegister {
                register: "ahb3enr",
                field: "rngen",
            }),
            reset: Some(PeripheralRccRegister {
                register: "ahb3rstr",
                field: "rngrst",
            }),
            mux: Some(PeripheralRccRegister {
                register: "ccipr",
                field: "rngsel",
            }),
            stop_mode: StopMode::Stop1,
        }),
        pins: &[],
        dma_channels: &[],
        interrupts: &[PeripheralInterrupt {
            signal: "GLOBAL",
            interrupt: "RNG",
        }],
    },
    Peripheral {
        name: "RTC",
        address: 1073752064,
        registers: Some(PeripheralRegisters {
            kind: "rtc",
            version: "v3",
            block: "RTC",
            ir: &rtc::REGISTERS,
        }),
        rcc: Some(PeripheralRcc {
            clock: "pclk1",
            enable: Some(PeripheralRccRegister {
                register: "apb1enr1",
                field: "rtcapben",
            }),
            reset: None,
            mux: None,
            stop_mode: StopMode::Standby,
        }),
        pins: &[
            PeripheralPin {
                pin: "PA0",
                signal: "TAMP_IN2",
                af: None,
            },
            PeripheralPin {
                pin: "PA10",
                signal: "REFIN",
                af: Some(0),
            },
            PeripheralPin {
                pin: "PA4",
                signal: "OUT2",
                af: None,
            },
            PeripheralPin {
                pin: "PB3",
                signal: "TAMP_IN3",
                af: None,
            },
            PeripheralPin {
                pin: "PC13",
                signal: "OUT1",
                af: None,
            },
            PeripheralPin {
                pin: "PC13",
                signal: "TAMP_IN1",
                af: None,
            },
            PeripheralPin {
                pin: "PC13",
                signal: "TS",
                af: None,
            },
        ],
        dma_channels: &[],
        interrupts: &[
            PeripheralInterrupt {
                signal: "ALARM",
                interrupt: "RTC_ALARM",
            },
            PeripheralInterrupt {
                signal: "SSRU",
                interrupt: "TAMP_STAMP_LSECSS_SSRU",
            },
            PeripheralInterrupt {
                signal: "STAMP",
                interrupt: "TAMP_STAMP_LSECSS_SSRU",
            },
            PeripheralInterrupt {
                signal: "TAMP",
                interrupt: "TAMP_STAMP_LSECSS_SSRU",
            },
            PeripheralInterrupt {
                signal: "WKUP",
                interrupt: "RTC_WKUP",
            },
        ],
    },
    Peripheral {
        name: "SPI1",
        address: 1073819648,
        registers: Some(PeripheralRegisters {
            kind: "spi",
            version: "v2",
            block: "SPI",
            ir: &spi::REGISTERS,
        }),
        rcc: Some(PeripheralRcc {
            clock: "pclk2",
            enable: Some(PeripheralRccRegister {
                register: "apb2enr",
                field: "spi1en",
            }),
            reset: Some(PeripheralRccRegister {
                register: "apb2rstr",
                field: "spi1rst",
            }),
            mux: None,
            stop_mode: StopMode::Stop1,
        }),
        pins: &[
            PeripheralPin {
                pin: "PA1",
                signal: "SCK",
                af: Some(5),
            },
            PeripheralPin {
                pin: "PA11",
                signal: "MISO",
                af: Some(5),
            },
            PeripheralPin {
                pin: "PA12",
                signal: "MOSI",
                af: Some(5),
            },
            PeripheralPin {
                pin: "PA15",
                signal: "NSS",
                af: Some(5),
            },
            PeripheralPin {
                pin: "PA4",
                signal: "NSS",
                af: Some(5),
            },
            PeripheralPin {
                pin: "PA5",
                signal: "SCK",
                af: Some(5),
            },
            PeripheralPin {
                pin: "PA6",
                signal: "MISO",
                af: Some(5),
            },
            PeripheralPin {
                pin: "PA7",
                signal: "MOSI",
                af: Some(5),
            },
            PeripheralPin {
                pin: "PB2",
                signal: "NSS",
                af: Some(5),
            },
            PeripheralPin {
                pin: "PB3",
                signal: "SCK",
                af: Some(5),
            },
            PeripheralPin {
                pin: "PB4",
                signal: "MISO",
                af: Some(5),
            },
            PeripheralPin {
                pin: "PB5",
                signal: "MOSI",
                af: Some(5),
            },
        ],
        dma_channels: &[
            PeripheralDmaChannel {
                signal: "RX",
                channel: None,
                dmamux: Some("DMAMUX1"),
                dma: None,
                request: Some(7),
            },
            PeripheralDmaChannel {
                signal: "TX",
                channel: None,
                dmamux: Some("DMAMUX1"),
                dma: None,
                request: Some(8),
            },
        ],
        interrupts: &[PeripheralInterrupt {
            signal: "GLOBAL",
            interrupt: "SPI1",
        }],
    },
    Peripheral {
        name: "SPI2",
        address: 1073756160,
        registers: Some(PeripheralRegisters {
            kind: "spi",
            version: "v2",
            block: "SPI",
            ir: &spi::REGISTERS,
        }),
        rcc: Some(PeripheralRcc {
            clock: "pclk1",
            enable: Some(PeripheralRccRegister {
                register: "apb1enr1",
                field: "spi2en",
            }),
            reset: Some(PeripheralRccRegister {
                register: "apb1rstr1",
                field: "spi2rst",
            }),
            mux: Some(PeripheralRccRegister {
                register: "ccipr",
                field: "spi2sel",
            }),
            stop_mode: StopMode::Stop1,
        }),
        pins: &[
            PeripheralPin {
                pin: "PA10",
                signal: "I2S_SD",
                af: Some(5),
            },
            PeripheralPin {
                pin: "PA10",
                signal: "MOSI",
                af: Some(5),
            },
            PeripheralPin {
                pin: "PA3",
                signal: "I2S_MCK",
                af: Some(5),
            },
            PeripheralPin {
                pin: "PA5",
                signal: "MISO",
                af: Some(3),
            },
            PeripheralPin {
                pin: "PA8",
                signal: "I2S_CK",
                af: Some(5),
            },
            PeripheralPin {
                pin: "PA8",
                signal: "SCK",
                af: Some(5),
            },
            PeripheralPin {
                pin: "PA9",
                signal: "I2S_CK",
                af: Some(5),
            },
            PeripheralPin {
                pin: "PA9",
                signal: "I2S_WS",
                af: Some(3),
            },
            PeripheralPin {
                pin: "PA9",
                signal: "NSS",
                af: Some(3),
            },
            PeripheralPin {
                pin: "PA9",
                signal: "SCK",
                af: Some(5),
            },
            PeripheralPin {
                pin: "PB12",
                signal: "I2S_WS",
                af: Some(5),
            },
            PeripheralPin {
                pin: "PB12",
                signal: "NSS",
                af: Some(5),
            },
        ],
        dma_channels: &[
            PeripheralDmaChannel {
                signal: "RX",
                channel: None,
                dmamux: Some("DMAMUX1"),
                dma: None,
                request: Some(9),
            },
            PeripheralDmaChannel {
                signal: "TX",
                channel: None,
                dmamux: Some("DMAMUX1"),
                dma: None,
                request: Some(10),
            },
        ],
        interrupts: &[PeripheralInterrupt {
            signal: "GLOBAL",
            interrupt: "SPI2",
        }],
    },
    Peripheral {
        name: "SUBGHZSPI",
        address: 1476460544,
        registers: Some(PeripheralRegisters {
            kind: "spi",
            version: "v2",
            block: "SPI",
            ir: &spi::REGISTERS,
        }),
        rcc: Some(PeripheralRcc {
            clock: "pclk3",
            enable: Some(PeripheralRccRegister {
                register: "apb3enr",
                field: "subghzspien",
            }),
            reset: Some(PeripheralRccRegister {
                register: "apb3rstr",
                field: "subghzspirst",
            }),
            mux: None,
            stop_mode: StopMode::Stop1,
        }),
        pins: &[
            PeripheralPin {
                pin: "PA4",
                signal: "NSS",
                af: Some(13),
            },
            PeripheralPin {
                pin: "PA5",
                signal: "SCK",
                af: Some(13),
            },
            PeripheralPin {
                pin: "PA6",
                signal: "MISO",
                af: Some(13),
            },
            PeripheralPin {
                pin: "PA7",
                signal: "MOSI",
                af: Some(13),
            },
        ],
        dma_channels: &[
            PeripheralDmaChannel {
                signal: "RX",
                channel: None,
                dmamux: Some("DMAMUX1"),
                dma: None,
                request: Some(41),
            },
            PeripheralDmaChannel {
                signal: "TX",
                channel: None,
                dmamux: Some("DMAMUX1"),
                dma: None,
                request: Some(42),
            },
        ],
        interrupts: &[],
    },
    Peripheral {
        name: "SYSCFG",
        address: 1073807360,
        registers: Some(PeripheralRegisters {
            kind: "syscfg",
            version: "wle",
            block: "SYSCFG",
            ir: &syscfg::REGISTERS,
        }),
        rcc: None,
        pins: &[],
        dma_channels: &[],
        interrupts: &[],
    },
    Peripheral {
        name: "TAMP",
        address: 1073786880,
        registers: Some(PeripheralRegisters {
            kind: "tamp",
            version: "wl",
            block: "TAMP",
            ir: &tamp::REGISTERS,
        }),
        rcc: None,
        pins: &[],
        dma_channels: &[],
        interrupts: &[],
    },
    Peripheral {
        name: "TIM1",
        address: 1073818624,
        registers: Some(PeripheralRegisters {
            kind: "timer",
            version: "v1",
            block: "TIM_ADV",
            ir: &timer::REGISTERS,
        }),
        rcc: Some(PeripheralRcc {
            clock: "pclk2_tim",
            enable: Some(PeripheralRccRegister {
                register: "apb2enr",
                field: "tim1en",
            }),
            reset: Some(PeripheralRccRegister {
                register: "apb2rstr",
                field: "tim1rst",
            }),
            mux: None,
            stop_mode: StopMode::Stop1,
        }),
        pins: &[
            PeripheralPin {
                pin: "PA10",
                signal: "CH3",
                af: Some(1),
            },
            PeripheralPin {
                pin: "PA11",
                signal: "BKIN2",
                af: Some(12),
            },
            PeripheralPin {
                pin: "PA11",
                signal: "CH4",
                af: Some(1),
            },
            PeripheralPin {
                pin: "PA12",
                signal: "ETR",
                af: Some(1),
            },
            PeripheralPin {
                pin: "PA6",
                signal: "BKIN",
                af: Some(12),
            },
            PeripheralPin {
                pin: "PA7",
                signal: "CH1N",
                af: Some(1),
            },
            PeripheralPin {
                pin: "PA8",
                signal: "CH1",
                af: Some(1),
            },
            PeripheralPin {
                pin: "PA9",
                signal: "CH2",
                af: Some(1),
            },
            PeripheralPin {
                pin: "PB12",
                signal: "BKIN",
                af: Some(3),
            },
            PeripheralPin {
                pin: "PB7",
                signal: "BKIN",
                af: Some(3),
            },
            PeripheralPin {
                pin: "PB8",
                signal: "CH2N",
                af: Some(1),
            },
        ],
        dma_channels: &[
            PeripheralDmaChannel {
                signal: "CH1",
                channel: None,
                dmamux: Some("DMAMUX1"),
                dma: None,
                request: Some(23),
            },
            PeripheralDmaChannel {
                signal: "CH2",
                channel: None,
                dmamux: Some("DMAMUX1"),
                dma: None,
                request: Some(24),
            },
            PeripheralDmaChannel {
                signal: "CH3",
                channel: None,
                dmamux: Some("DMAMUX1"),
                dma: None,
                request: Some(25),
            },
            PeripheralDmaChannel {
                signal: "CH4",
                channel: None,
                dmamux: Some("DMAMUX1"),
                dma: None,
                request: Some(26),
            },
            PeripheralDmaChannel {
                signal: "UP",
                channel: None,
                dmamux: Some("DMAMUX1"),
                dma: None,
                request: Some(27),
            },
            PeripheralDmaChannel {
                signal: "TRIG",
                channel: None,
                dmamux: Some("DMAMUX1"),
                dma: None,
                request: Some(28),
            },
            PeripheralDmaChannel {
                signal: "COM",
                channel: None,
                dmamux: Some("DMAMUX1"),
                dma: None,
                request: Some(29),
            },
        ],
        interrupts: &[
            PeripheralInterrupt {
                signal: "BRK",
                interrupt: "TIM1_BRK",
            },
            PeripheralInterrupt {
                signal: "CC",
                interrupt: "TIM1_CC",
            },
            PeripheralInterrupt {
                signal: "COM",
                interrupt: "TIM1_TRG_COM",
            },
            PeripheralInterrupt {
                signal: "TRG",
                interrupt: "TIM1_TRG_COM",
            },
            PeripheralInterrupt {
                signal: "UP",
                interrupt: "TIM1_UP",
            },
        ],
    },
    Peripheral {
        name: "TIM16",
        address: 1073824768,
        registers: Some(PeripheralRegisters {
            kind: "timer",
            version: "v1",
            block: "TIM_GP16",
            ir: &timer::REGISTERS,
        }),
        rcc: Some(PeripheralRcc {
            clock: "pclk2_tim",
            enable: Some(PeripheralRccRegister {
                register: "apb2enr",
                field: "tim16en",
            }),
            reset: Some(PeripheralRccRegister {
                register: "apb2rstr",
                field: "tim16rst",
            }),
            mux: None,
            stop_mode: StopMode::Stop1,
        }),
        pins: &[
            PeripheralPin {
                pin: "PA6",
                signal: "CH1",
                af: Some(14),
            },
            PeripheralPin {
                pin: "PB5",
                signal: "BKIN",
                af: Some(14),
            },
            PeripheralPin {
                pin: "PB6",
                signal: "CH1N",
                af: Some(14),
            },
            PeripheralPin {
                pin: "PB8",
                signal: "CH1",
                af: Some(14),
            },
        ],
        dma_channels: &[
            PeripheralDmaChannel {
                signal: "CH1",
                channel: None,
                dmamux: Some("DMAMUX1"),
                dma: None,
                request: Some(35),
            },
            PeripheralDmaChannel {
                signal: "UP",
                channel: None,
                dmamux: Some("DMAMUX1"),
                dma: None,
                request: Some(36),
            },
        ],
        interrupts: &[
            PeripheralInterrupt {
                signal: "BRK",
                interrupt: "TIM16",
            },
            PeripheralInterrupt {
                signal: "CC",
                interrupt: "TIM16",
            },
            PeripheralInterrupt {
                signal: "COM",
                interrupt: "TIM16",
            },
            PeripheralInterrupt {
                signal: "TRG",
                interrupt: "TIM16",
            },
            PeripheralInterrupt {
                signal: "UP",
                interrupt: "TIM16",
            },
        ],
    },
    Peripheral {
        name: "TIM17",
        address: 1073825792,
        registers: Some(PeripheralRegisters {
            kind: "timer",
            version: "v1",
            block: "TIM_GP16",
            ir: &timer::REGISTERS,
        }),
        rcc: Some(PeripheralRcc {
            clock: "pclk2_tim",
            enable: Some(PeripheralRccRegister {
                register: "apb2enr",
                field: "tim17en",
            }),
            reset: Some(PeripheralRccRegister {
                register: "apb2rstr",
                field: "tim17rst",
            }),
            mux: None,
            stop_mode: StopMode::Stop1,
        }),
        pins: &[
            PeripheralPin {
                pin: "PA10",
                signal: "BKIN",
                af: Some(14),
            },
            PeripheralPin {
                pin: "PA7",
                signal: "CH1",
                af: Some(14),
            },
            PeripheralPin {
                pin: "PB4",
                signal: "BKIN",
                af: Some(14),
            },
            PeripheralPin {
                pin: "PB7",
                signal: "CH1N",
                af: Some(14),
            },
        ],
        dma_channels: &[
            PeripheralDmaChannel {
                signal: "CH1",
                channel: None,
                dmamux: Some("DMAMUX1"),
                dma: None,
                request: Some(37),
            },
            PeripheralDmaChannel {
                signal: "UP",
                channel: None,
                dmamux: Some("DMAMUX1"),
                dma: None,
                request: Some(38),
            },
        ],
        interrupts: &[
            PeripheralInterrupt {
                signal: "BRK",
                interrupt: "TIM17",
            },
            PeripheralInterrupt {
                signal: "CC",
                interrupt: "TIM17",
            },
            PeripheralInterrupt {
                signal: "COM",
                interrupt: "TIM17",
            },
            PeripheralInterrupt {
                signal: "TRG",
                interrupt: "TIM17",
            },
            PeripheralInterrupt {
                signal: "UP",
                interrupt: "TIM17",
            },
        ],
    },
    Peripheral {
        name: "TIM2",
        address: 1073741824,
        registers: Some(PeripheralRegisters {
            kind: "timer",
            version: "v1",
            block: "TIM_GP32",
            ir: &timer::REGISTERS,
        }),
        rcc: Some(PeripheralRcc {
            clock: "pclk1_tim",
            enable: Some(PeripheralRccRegister {
                register: "apb1enr1",
                field: "tim2en",
            }),
            reset: Some(PeripheralRccRegister {
                register: "apb1rstr1",
                field: "tim2rst",
            }),
            mux: None,
            stop_mode: StopMode::Stop1,
        }),
        pins: &[
            PeripheralPin {
                pin: "PA0",
                signal: "CH1",
                af: Some(1),
            },
            PeripheralPin {
                pin: "PA0",
                signal: "ETR",
                af: Some(14),
            },
            PeripheralPin {
                pin: "PA1",
                signal: "CH2",
                af: Some(1),
            },
            PeripheralPin {
                pin: "PA15",
                signal: "CH1",
                af: Some(1),
            },
            PeripheralPin {
                pin: "PA15",
                signal: "ETR",
                af: Some(2),
            },
            PeripheralPin {
                pin: "PA2",
                signal: "CH3",
                af: Some(1),
            },
            PeripheralPin {
                pin: "PA3",
                signal: "CH4",
                af: Some(1),
            },
            PeripheralPin {
                pin: "PA5",
                signal: "CH1",
                af: Some(1),
            },
            PeripheralPin {
                pin: "PA5",
                signal: "ETR",
                af: Some(2),
            },
            PeripheralPin {
                pin: "PB3",
                signal: "CH2",
                af: Some(1),
            },
        ],
        dma_channels: &[
            PeripheralDmaChannel {
                signal: "CH1",
                channel: None,
                dmamux: Some("DMAMUX1"),
                dma: None,
                request: Some(30),
            },
            PeripheralDmaChannel {
                signal: "CH2",
                channel: None,
                dmamux: Some("DMAMUX1"),
                dma: None,
                request: Some(31),
            },
            PeripheralDmaChannel {
                signal: "CH3",
                channel: None,
                dmamux: Some("DMAMUX1"),
                dma: None,
                request: Some(32),
            },
            PeripheralDmaChannel {
                signal: "CH4",
                channel: None,
                dmamux: Some("DMAMUX1"),
                dma: None,
                request: Some(33),
            },
            PeripheralDmaChannel {
                signal: "UP",
                channel: None,
                dmamux: Some("DMAMUX1"),
                dma: None,
                request: Some(34),
            },
        ],
        interrupts: &[
            PeripheralInterrupt {
                signal: "BRK",
                interrupt: "TIM2",
            },
            PeripheralInterrupt {
                signal: "CC",
                interrupt: "TIM2",
            },
            PeripheralInterrupt {
                signal: "COM",
                interrupt: "TIM2",
            },
            PeripheralInterrupt {
                signal: "TRG",
                interrupt: "TIM2",
            },
            PeripheralInterrupt {
                signal: "UP",
                interrupt: "TIM2",
            },
        ],
    },
    Peripheral {
        name: "UID",
        address: 536835472,
        registers: Some(PeripheralRegisters {
            kind: "uid",
            version: "v1",
            block: "UID",
            ir: &uid::REGISTERS,
        }),
        rcc: None,
        pins: &[],
        dma_channels: &[],
        interrupts: &[],
    },
    Peripheral {
        name: "USART1",
        address: 1073821696,
        registers: Some(PeripheralRegisters {
            kind: "usart",
            version: "v4",
            block: "USART",
            ir: &usart::REGISTERS,
        }),
        rcc: Some(PeripheralRcc {
            clock: "pclk2",
            enable: Some(PeripheralRccRegister {
                register: "apb2enr",
                field: "usart1en",
            }),
            reset: Some(PeripheralRccRegister {
                register: "apb2rstr",
                field: "usart1rst",
            }),
            mux: Some(PeripheralRccRegister {
                register: "ccipr",
                field: "usart1sel",
            }),
            stop_mode: StopMode::Stop1,
        }),
        pins: &[
            PeripheralPin {
                pin: "PA10",
                signal: "RX",
                af: Some(7),
            },
            PeripheralPin {
                pin: "PA11",
                signal: "CTS",
                af: Some(7),
            },
            PeripheralPin {
                pin: "PA11",
                signal: "NSS",
                af: Some(7),
            },
            PeripheralPin {
                pin: "PA12",
                signal: "DE",
                af: Some(7),
            },
            PeripheralPin {
                pin: "PA12",
                signal: "RTS",
                af: Some(7),
            },
            PeripheralPin {
                pin: "PA8",
                signal: "CK",
                af: Some(7),
            },
            PeripheralPin {
                pin: "PA9",
                signal: "TX",
                af: Some(7),
            },
            PeripheralPin {
                pin: "PB3",
                signal: "DE",
                af: Some(7),
            },
            PeripheralPin {
                pin: "PB3",
                signal: "RTS",
                af: Some(7),
            },
            PeripheralPin {
                pin: "PB4",
                signal: "CTS",
                af: Some(7),
            },
            PeripheralPin {
                pin: "PB4",
                signal: "NSS",
                af: Some(7),
            },
            PeripheralPin {
                pin: "PB5",
                signal: "CK",
                af: Some(7),
            },
            PeripheralPin {
                pin: "PB6",
                signal: "TX",
                af: Some(7),
            },
            PeripheralPin {
                pin: "PB7",
                signal: "RX",
                af: Some(7),
            },
        ],
        dma_channels: &[
            PeripheralDmaChannel {
                signal: "RX",
                channel: None,
                dmamux: Some("DMAMUX1"),
                dma: None,
                request: Some(17),
            },
            PeripheralDmaChannel {
                signal: "TX",
                channel: None,
                dmamux: Some("DMAMUX1"),
                dma: None,
                request: Some(18),
            },
        ],
        interrupts: &[PeripheralInterrupt {
            signal: "GLOBAL",
            interrupt: "USART1",
        }],
    },
    Peripheral {
        name: "USART2",
        address: 1073759232,
        registers: Some(PeripheralRegisters {
            kind: "usart",
            version: "v4",
            block: "USART",
            ir: &usart::REGISTERS,
        }),
        rcc: Some(PeripheralRcc {
            clock: "pclk1",
            enable: Some(PeripheralRccRegister {
                register: "apb1enr1",
                field: "usart2en",
            }),
            reset: Some(PeripheralRccRegister {
                register: "apb1rstr1",
                field: "usart2rst",
            }),
            mux: Some(PeripheralRccRegister {
                register: "ccipr",
                field: "usart2sel",
            }),
            stop_mode: StopMode::Stop1,
        }),
        pins: &[
            PeripheralPin {
                pin: "PA0",
                signal: "CTS",
                af: Some(7),
            },
            PeripheralPin {
                pin: "PA0",
                signal: "NSS",
                af: Some(7),
            },
            PeripheralPin {
                pin: "PA1",
                signal: "DE",
                af: Some(7),
            },
            PeripheralPin {
                pin: "PA1",
                signal: "RTS",
                af: Some(7),
            },
            PeripheralPin {
                pin: "PA2",
                signal: "TX",
                af: Some(7),
            },
            PeripheralPin {
                pin: "PA3",
                signal: "RX",
                af: Some(7),
            },
            PeripheralPin {
                pin: "PA4",
                signal: "CK",
                af: Some(7),
            },
        ],
        dma_channels: &[
            PeripheralDmaChannel {
                signal: "RX",
                channel: None,
                dmamux: Some("DMAMUX1"),
                dma: None,
                request: Some(19),
            },
            PeripheralDmaChannel {
                signal: "TX",
                channel: None,
                dmamux: Some("DMAMUX1"),
                dma: None,
                request: Some(20),
            },
        ],
        interrupts: &[PeripheralInterrupt {
            signal: "GLOBAL",
            interrupt: "USART2",
        }],
    },
    Peripheral {
        name: "VREFBUF",
        address: 1073807408,
        registers: None,
        rcc: None,
        pins: &[],
        dma_channels: &[],
        interrupts: &[],
    },
    Peripheral {
        name: "WWDG",
        address: 1073753088,
        registers: Some(PeripheralRegisters {
            kind: "wwdg",
            version: "v2",
            block: "WWDG",
            ir: &wwdg::REGISTERS,
        }),
        rcc: Some(PeripheralRcc {
            clock: "pclk1",
            enable: Some(PeripheralRccRegister {
                register: "apb1enr1",
                field: "wwdgen",
            }),
            reset: None,
            mux: None,
            stop_mode: StopMode::Stop1,
        }),
        pins: &[],
        dma_channels: &[],
        interrupts: &[
            PeripheralInterrupt {
                signal: "GLOBAL",
                interrupt: "WWDG",
            },
            PeripheralInterrupt {
                signal: "RST",
                interrupt: "WWDG",
            },
        ],
    },
];
pub(crate) static INTERRUPTS: &'static [Interrupt] = &[
    Interrupt {
        name: "WWDG",
        number: 0,
    },
    Interrupt {
        name: "PVD_PVM",
        number: 1,
    },
    Interrupt {
        name: "TAMP_STAMP_LSECSS_SSRU",
        number: 2,
    },
    Interrupt {
        name: "RTC_WKUP",
        number: 3,
    },
    Interrupt {
        name: "FLASH",
        number: 4,
    },
    Interrupt { name: "RCC", number: 5 },
    Interrupt {
        name: "EXTI0",
        number: 6,
    },
    Interrupt {
        name: "EXTI1",
        number: 7,
    },
    Interrupt {
        name: "EXTI2",
        number: 8,
    },
    Interrupt {
        name: "EXTI3",
        number: 9,
    },
    Interrupt {
        name: "EXTI4",
        number: 10,
    },
    Interrupt {
        name: "DMA1_CHANNEL1",
        number: 11,
    },
    Interrupt {
        name: "DMA1_CHANNEL2",
        number: 12,
    },
    Interrupt {
        name: "DMA1_CHANNEL3",
        number: 13,
    },
    Interrupt {
        name: "DMA1_CHANNEL4",
        number: 14,
    },
    Interrupt {
        name: "DMA1_CHANNEL5",
        number: 15,
    },
    Interrupt {
        name: "DMA1_CHANNEL6",
        number: 16,
    },
    Interrupt {
        name: "DMA1_CHANNEL7",
        number: 17,
    },
    Interrupt {
        name: "ADC",
        number: 18,
    },
    Interrupt {
        name: "DAC",
        number: 19,
    },
    Interrupt {
        name: "COMP",
        number: 21,
    },
    Interrupt {
        name: "EXTI9_5",
        number: 22,
    },
    Interrupt {
        name: "TIM1_BRK",
        number: 23,
    },
    Interrupt {
        name: "TIM1_UP",
        number: 24,
    },
    Interrupt {
        name: "TIM1_TRG_COM",
        number: 25,
    },
    Interrupt {
        name: "TIM1_CC",
        number: 26,
    },
    Interrupt {
        name: "TIM2",
        number: 27,
    },
    Interrupt {
        name: "TIM16",
        number: 28,
    },
    Interrupt {
        name: "TIM17",
        number: 29,
    },
    Interrupt {
        name: "I2C1_EV",
        number: 30,
    },
    Interrupt {
        name: "I2C1_ER",
        number: 31,
    },
    Interrupt {
        name: "I2C2_EV",
        number: 32,
    },
    Interrupt {
        name: "I2C2_ER",
        number: 33,
    },
    Interrupt {
        name: "SPI1",
        number: 34,
    },
    Interrupt {
        name: "SPI2",
        number: 35,
    },
    Interrupt {
        name: "USART1",
        number: 36,
    },
    Interrupt {
        name: "USART2",
        number: 37,
    },
    Interrupt {
        name: "LPUART1",
        number: 38,
    },
    Interrupt {
        name: "LPTIM1",
        number: 39,
    },
    Interrupt {
        name: "LPTIM2",
        number: 40,
    },
    Interrupt {
        name: "EXTI15_10",
        number: 41,
    },
    Interrupt {
        name: "RTC_ALARM",
        number: 42,
    },
    Interrupt {
        name: "LPTIM3",
        number: 43,
    },
    Interrupt {
        name: "SUBGHZSPI",
        number: 44,
    },
    Interrupt {
        name: "HSEM",
        number: 47,
    },
    Interrupt {
        name: "I2C3_EV",
        number: 48,
    },
    Interrupt {
        name: "I2C3_ER",
        number: 49,
    },
    Interrupt {
        name: "SUBGHZ_RADIO",
        number: 50,
    },
    Interrupt {
        name: "AES",
        number: 51,
    },
    Interrupt {
        name: "RNG",
        number: 52,
    },
    Interrupt {
        name: "PKA",
        number: 53,
    },
    Interrupt {
        name: "DMA2_CHANNEL1",
        number: 54,
    },
    Interrupt {
        name: "DMA2_CHANNEL2",
        number: 55,
    },
    Interrupt {
        name: "DMA2_CHANNEL3",
        number: 56,
    },
    Interrupt {
        name: "DMA2_CHANNEL4",
        number: 57,
    },
    Interrupt {
        name: "DMA2_CHANNEL5",
        number: 58,
    },
    Interrupt {
        name: "DMA2_CHANNEL6",
        number: 59,
    },
    Interrupt {
        name: "DMA2_CHANNEL7",
        number: 60,
    },
    Interrupt {
        name: "DMAMUX1_OVR",
        number: 61,
    },
];
pub(crate) static DMA_CHANNELS: &'static [DmaChannel] = &[
    DmaChannel {
        name: "DMA1_CH1",
        dma: "DMA1",
        channel: 0,
        dmamux: Some("DMAMUX1"),
        dmamux_channel: Some(0),
    },
    DmaChannel {
        name: "DMA1_CH2",
        dma: "DMA1",
        channel: 1,
        dmamux: Some("DMAMUX1"),
        dmamux_channel: Some(1),
    },
    DmaChannel {
        name: "DMA1_CH3",
        dma: "DMA1",
        channel: 2,
        dmamux: Some("DMAMUX1"),
        dmamux_channel: Some(2),
    },
    DmaChannel {
        name: "DMA1_CH4",
        dma: "DMA1",
        channel: 3,
        dmamux: Some("DMAMUX1"),
        dmamux_channel: Some(3),
    },
    DmaChannel {
        name: "DMA1_CH5",
        dma: "DMA1",
        channel: 4,
        dmamux: Some("DMAMUX1"),
        dmamux_channel: Some(4),
    },
    DmaChannel {
        name: "DMA1_CH6",
        dma: "DMA1",
        channel: 5,
        dmamux: Some("DMAMUX1"),
        dmamux_channel: Some(5),
    },
    DmaChannel {
        name: "DMA1_CH7",
        dma: "DMA1",
        channel: 6,
        dmamux: Some("DMAMUX1"),
        dmamux_channel: Some(6),
    },
    DmaChannel {
        name: "DMA2_CH1",
        dma: "DMA2",
        channel: 0,
        dmamux: Some("DMAMUX1"),
        dmamux_channel: Some(7),
    },
    DmaChannel {
        name: "DMA2_CH2",
        dma: "DMA2",
        channel: 1,
        dmamux: Some("DMAMUX1"),
        dmamux_channel: Some(8),
    },
    DmaChannel {
        name: "DMA2_CH3",
        dma: "DMA2",
        channel: 2,
        dmamux: Some("DMAMUX1"),
        dmamux_channel: Some(9),
    },
    DmaChannel {
        name: "DMA2_CH4",
        dma: "DMA2",
        channel: 3,
        dmamux: Some("DMAMUX1"),
        dmamux_channel: Some(10),
    },
    DmaChannel {
        name: "DMA2_CH5",
        dma: "DMA2",
        channel: 4,
        dmamux: Some("DMAMUX1"),
        dmamux_channel: Some(11),
    },
    DmaChannel {
        name: "DMA2_CH6",
        dma: "DMA2",
        channel: 5,
        dmamux: Some("DMAMUX1"),
        dmamux_channel: Some(12),
    },
    DmaChannel {
        name: "DMA2_CH7",
        dma: "DMA2",
        channel: 6,
        dmamux: Some("DMAMUX1"),
        dmamux_channel: Some(13),
    },
];
#[path = "../registers/adc_g0.rs"]
pub mod adc;
#[path = "../registers/adccommon_v3.rs"]
pub mod adccommon;
#[path = "../registers/aes_v2.rs"]
pub mod aes;
#[path = "../registers/bdma_v1.rs"]
pub mod bdma;
#[path = "../registers/crc_v2.rs"]
pub mod crc;
#[path = "../registers/dbgmcu_wl.rs"]
pub mod dbgmcu;
#[path = "../registers/dmamux_v1.rs"]
pub mod dmamux;
#[path = "../registers/exti_wle.rs"]
pub mod exti;
#[path = "../registers/flash_wl.rs"]
pub mod flash;
#[path = "../registers/gpio_v2.rs"]
pub mod gpio;
#[path = "../registers/i2c_v2.rs"]
pub mod i2c;
#[path = "../registers/iwdg_v2.rs"]
pub mod iwdg;
#[path = "../registers/pwr_wl5.rs"]
pub mod pwr;
#[path = "../registers/rcc_wle.rs"]
pub mod rcc;
#[path = "../registers/rng_v2.rs"]
pub mod rng;
#[path = "../registers/rtc_v3.rs"]
pub mod rtc;
#[path = "../registers/spi_v2.rs"]
pub mod spi;
#[path = "../registers/syscfg_wle.rs"]
pub mod syscfg;
#[path = "../registers/tamp_wl.rs"]
pub mod tamp;
#[path = "../registers/timer_v1.rs"]
pub mod timer;
#[path = "../registers/uid_v1.rs"]
pub mod uid;
#[path = "../registers/usart_v4.rs"]
pub mod usart;
#[path = "../registers/wwdg_v2.rs"]
pub mod wwdg;


pub(crate) static PERIPHERALS: &'static [Peripheral] = &[
    Peripheral {
        name: "ADC1",
        address: 1073815552,
        registers: Some(PeripheralRegisters {
            kind: "adc",
            version: "v2",
            block: "ADC",
            ir: &adc::REGISTERS,
        }),
        rcc: Some(PeripheralRcc {
            clock: "pclk2",
            enable: Some(PeripheralRccRegister {
                register: "apb2enr",
                field: "adc1en",
            }),
            reset: None,
            mux: None,
            stop_mode: StopMode::Stop1,
        }),
        pins: &[
            PeripheralPin {
                pin: "PA0",
                signal: "IN0",
                af: None,
            },
            PeripheralPin {
                pin: "PA1",
                signal: "IN1",
                af: None,
            },
            PeripheralPin {
                pin: "PA2",
                signal: "IN2",
                af: None,
            },
            PeripheralPin {
                pin: "PA3",
                signal: "IN3",
                af: None,
            },
            PeripheralPin {
                pin: "PA4",
                signal: "IN4",
                af: None,
            },
            PeripheralPin {
                pin: "PA5",
                signal: "IN5",
                af: None,
            },
            PeripheralPin {
                pin: "PA6",
                signal: "IN6",
                af: None,
            },
            PeripheralPin {
                pin: "PA7",
                signal: "IN7",
                af: None,
            },
            PeripheralPin {
                pin: "PB0",
                signal: "IN8",
                af: None,
            },
            PeripheralPin {
                pin: "PB1",
                signal: "IN9",
                af: None,
            },
            PeripheralPin {
                pin: "PC0",
                signal: "IN10",
                af: None,
            },
            PeripheralPin {
                pin: "PC1",
                signal: "IN11",
                af: None,
            },
            PeripheralPin {
                pin: "PC2",
                signal: "IN12",
                af: None,
            },
            PeripheralPin {
                pin: "PC3",
                signal: "IN13",
                af: None,
            },
            PeripheralPin {
                pin: "PC4",
                signal: "IN14",
                af: None,
            },
            PeripheralPin {
                pin: "PC5",
                signal: "IN15",
                af: None,
            },
        ],
        dma_channels: &[
            PeripheralDmaChannel {
                signal: "ADC1",
                channel: Some("DMA2_CH0"),
                dmamux: None,
                dma: None,
                request: Some(0),
            },
            PeripheralDmaChannel {
                signal: "ADC1",
                channel: Some("DMA2_CH4"),
                dmamux: None,
                dma: None,
                request: Some(0),
            },
        ],
        interrupts: &[PeripheralInterrupt {
            signal: "GLOBAL",
            interrupt: "ADC",
        }],
    },
    Peripheral {
        name: "ADC2",
        address: 1073815808,
        registers: Some(PeripheralRegisters {
            kind: "adc",
            version: "v2",
            block: "ADC",
            ir: &adc::REGISTERS,
        }),
        rcc: Some(PeripheralRcc {
            clock: "pclk2",
            enable: Some(PeripheralRccRegister {
                register: "apb2enr",
                field: "adc2en",
            }),
            reset: None,
            mux: None,
            stop_mode: StopMode::Stop1,
        }),
        pins: &[
            PeripheralPin {
                pin: "PA0",
                signal: "IN0",
                af: None,
            },
            PeripheralPin {
                pin: "PA1",
                signal: "IN1",
                af: None,
            },
            PeripheralPin {
                pin: "PA2",
                signal: "IN2",
                af: None,
            },
            PeripheralPin {
                pin: "PA3",
                signal: "IN3",
                af: None,
            },
            PeripheralPin {
                pin: "PA4",
                signal: "IN4",
                af: None,
            },
            PeripheralPin {
                pin: "PA5",
                signal: "IN5",
                af: None,
            },
            PeripheralPin {
                pin: "PA6",
                signal: "IN6",
                af: None,
            },
            PeripheralPin {
                pin: "PA7",
                signal: "IN7",
                af: None,
            },
            PeripheralPin {
                pin: "PB0",
                signal: "IN8",
                af: None,
            },
            PeripheralPin {
                pin: "PB1",
                signal: "IN9",
                af: None,
            },
            PeripheralPin {
                pin: "PC0",
                signal: "IN10",
                af: None,
            },
            PeripheralPin {
                pin: "PC1",
                signal: "IN11",
                af: None,
            },
            PeripheralPin {
                pin: "PC2",
                signal: "IN12",
                af: None,
            },
            PeripheralPin {
                pin: "PC3",
                signal: "IN13",
                af: None,
            },
            PeripheralPin {
                pin: "PC4",
                signal: "IN14",
                af: None,
            },
            PeripheralPin {
                pin: "PC5",
                signal: "IN15",
                af: None,
            },
        ],
        dma_channels: &[
            PeripheralDmaChannel {
                signal: "ADC2",
                channel: Some("DMA2_CH2"),
                dmamux: None,
                dma: None,
                request: Some(1),
            },
            PeripheralDmaChannel {
                signal: "ADC2",
                channel: Some("DMA2_CH3"),
                dmamux: None,
                dma: None,
                request: Some(1),
            },
        ],
        interrupts: &[PeripheralInterrupt {
            signal: "GLOBAL",
            interrupt: "ADC",
        }],
    },
    Peripheral {
        name: "ADC3",
        address: 1073816064,
        registers: Some(PeripheralRegisters {
            kind: "adc",
            version: "v2",
            block: "ADC",
            ir: &adc::REGISTERS,
        }),
        rcc: Some(PeripheralRcc {
            clock: "pclk2",
            enable: Some(PeripheralRccRegister {
                register: "apb2enr",
                field: "adc3en",
            }),
            reset: None,
            mux: None,
            stop_mode: StopMode::Stop1,
        }),
        pins: &[
            PeripheralPin {
                pin: "PA0",
                signal: "IN0",
                af: None,
            },
            PeripheralPin {
                pin: "PA1",
                signal: "IN1",
                af: None,
            },
            PeripheralPin {
                pin: "PA2",
                signal: "IN2",
                af: None,
            },
            PeripheralPin {
                pin: "PA3",
                signal: "IN3",
                af: None,
            },
            PeripheralPin {
                pin: "PC0",
                signal: "IN10",
                af: None,
            },
            PeripheralPin {
                pin: "PC1",
                signal: "IN11",
                af: None,
            },
            PeripheralPin {
                pin: "PC2",
                signal: "IN12",
                af: None,
            },
            PeripheralPin {
                pin: "PC3",
                signal: "IN13",
                af: None,
            },
        ],
        dma_channels: &[
            PeripheralDmaChannel {
                signal: "ADC3",
                channel: Some("DMA2_CH0"),
                dmamux: None,
                dma: None,
                request: Some(2),
            },
            PeripheralDmaChannel {
                signal: "ADC3",
                channel: Some("DMA2_CH1"),
                dmamux: None,
                dma: None,
                request: Some(2),
            },
        ],
        interrupts: &[PeripheralInterrupt {
            signal: "GLOBAL",
            interrupt: "ADC",
        }],
    },
    Peripheral {
        name: "ADC_COMMON",
        address: 1073816320,
        registers: Some(PeripheralRegisters {
            kind: "adccommon",
            version: "v2",
            block: "ADC_COMMON",
            ir: &adccommon::REGISTERS,
        }),
        rcc: None,
        pins: &[],
        dma_channels: &[],
        interrupts: &[],
    },
    Peripheral {
        name: "CAN1",
        address: 1073767424,
        registers: Some(PeripheralRegisters {
            kind: "can",
            version: "bxcan",
            block: "CAN",
            ir: &can::REGISTERS,
        }),
        rcc: Some(PeripheralRcc {
            clock: "pclk1",
            enable: Some(PeripheralRccRegister {
                register: "apb1enr",
                field: "can1en",
            }),
            reset: Some(PeripheralRccRegister {
                register: "apb1rstr",
                field: "can1rst",
            }),
            mux: None,
            stop_mode: StopMode::Stop1,
        }),
        pins: &[
            PeripheralPin {
                pin: "PA11",
                signal: "RX",
                af: Some(9),
            },
            PeripheralPin {
                pin: "PA12",
                signal: "TX",
                af: Some(9),
            },
            PeripheralPin {
                pin: "PB8",
                signal: "RX",
                af: Some(9),
            },
            PeripheralPin {
                pin: "PB9",
                signal: "TX",
                af: Some(9),
            },
            PeripheralPin {
                pin: "PD0",
                signal: "RX",
                af: Some(9),
            },
            PeripheralPin {
                pin: "PD1",
                signal: "TX",
                af: Some(9),
            },
        ],
        dma_channels: &[],
        interrupts: &[
            PeripheralInterrupt {
                signal: "RX0",
                interrupt: "CAN1_RX0",
            },
            PeripheralInterrupt {
                signal: "RX1",
                interrupt: "CAN1_RX1",
            },
            PeripheralInterrupt {
                signal: "SCE",
                interrupt: "CAN1_SCE",
            },
            PeripheralInterrupt {
                signal: "TX",
                interrupt: "CAN1_TX",
            },
        ],
    },
    Peripheral {
        name: "CAN2",
        address: 1073768448,
        registers: Some(PeripheralRegisters {
            kind: "can",
            version: "bxcan",
            block: "CAN",
            ir: &can::REGISTERS,
        }),
        rcc: Some(PeripheralRcc {
            clock: "pclk1",
            enable: Some(PeripheralRccRegister {
                register: "apb1enr",
                field: "can2en",
            }),
            reset: Some(PeripheralRccRegister {
                register: "apb1rstr",
                field: "can2rst",
            }),
            mux: None,
            stop_mode: StopMode::Stop1,
        }),
        pins: &[
            PeripheralPin {
                pin: "PB12",
                signal: "RX",
                af: Some(9),
            },
            PeripheralPin {
                pin: "PB13",
                signal: "TX",
                af: Some(9),
            },
            PeripheralPin {
                pin: "PB5",
                signal: "RX",
                af: Some(9),
            },
            PeripheralPin {
                pin: "PB6",
                signal: "TX",
                af: Some(9),
            },
        ],
        dma_channels: &[],
        interrupts: &[
            PeripheralInterrupt {
                signal: "RX0",
                interrupt: "CAN2_RX0",
            },
            PeripheralInterrupt {
                signal: "RX1",
                interrupt: "CAN2_RX1",
            },
            PeripheralInterrupt {
                signal: "SCE",
                interrupt: "CAN2_SCE",
            },
            PeripheralInterrupt {
                signal: "TX",
                interrupt: "CAN2_TX",
            },
        ],
    },
    Peripheral {
        name: "CAN3",
        address: 1073755136,
        registers: Some(PeripheralRegisters {
            kind: "can",
            version: "bxcan",
            block: "CAN",
            ir: &can::REGISTERS,
        }),
        rcc: Some(PeripheralRcc {
            clock: "pclk1",
            enable: Some(PeripheralRccRegister {
                register: "apb1enr",
                field: "can3en",
            }),
            reset: Some(PeripheralRccRegister {
                register: "apb1rstr",
                field: "can3rst",
            }),
            mux: None,
            stop_mode: StopMode::Stop1,
        }),
        pins: &[
            PeripheralPin {
                pin: "PA15",
                signal: "TX",
                af: Some(11),
            },
            PeripheralPin {
                pin: "PA8",
                signal: "RX",
                af: Some(11),
            },
            PeripheralPin {
                pin: "PB3",
                signal: "RX",
                af: Some(11),
            },
            PeripheralPin {
                pin: "PB4",
                signal: "TX",
                af: Some(11),
            },
        ],
        dma_channels: &[],
        interrupts: &[
            PeripheralInterrupt {
                signal: "RX0",
                interrupt: "CAN3_RX0",
            },
            PeripheralInterrupt {
                signal: "RX1",
                interrupt: "CAN3_RX1",
            },
            PeripheralInterrupt {
                signal: "SCE",
                interrupt: "CAN3_SCE",
            },
            PeripheralInterrupt {
                signal: "TX",
                interrupt: "CAN3_TX",
            },
        ],
    },
    Peripheral {
        name: "CEC",
        address: 1073769472,
        registers: None,
        rcc: Some(PeripheralRcc {
            clock: "pclk1",
            enable: Some(PeripheralRccRegister {
                register: "apb1enr",
                field: "cecen",
            }),
            reset: Some(PeripheralRccRegister {
                register: "apb1rstr",
                field: "cecrst",
            }),
            mux: None,
            stop_mode: StopMode::Stop1,
        }),
        pins: &[],
        dma_channels: &[],
        interrupts: &[],
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
        name: "CRYP",
        address: 1342570496,
        registers: None,
        rcc: Some(PeripheralRcc {
            clock: "hclk2",
            enable: Some(PeripheralRccRegister {
                register: "ahb2enr",
                field: "crypen",
            }),
            reset: Some(PeripheralRccRegister {
                register: "ahb2rstr",
                field: "cryprst",
            }),
            mux: None,
            stop_mode: StopMode::Stop1,
        }),
        pins: &[],
        dma_channels: &[
            PeripheralDmaChannel {
                signal: "OUT",
                channel: Some("DMA2_CH5"),
                dmamux: None,
                dma: None,
                request: Some(2),
            },
            PeripheralDmaChannel {
                signal: "IN",
                channel: Some("DMA2_CH6"),
                dmamux: None,
                dma: None,
                request: Some(2),
            },
        ],
        interrupts: &[PeripheralInterrupt {
            signal: "GLOBAL",
            interrupt: "CRYP",
        }],
    },
    Peripheral {
        name: "DAC",
        address: 1073771520,
        registers: Some(PeripheralRegisters {
            kind: "dac",
            version: "v1",
            block: "DAC",
            ir: &dac::REGISTERS,
        }),
        rcc: Some(PeripheralRcc {
            clock: "pclk1",
            enable: Some(PeripheralRccRegister {
                register: "apb1enr",
                field: "dacen",
            }),
            reset: Some(PeripheralRccRegister {
                register: "apb1rstr",
                field: "dacrst",
            }),
            mux: None,
            stop_mode: StopMode::Stop1,
        }),
        pins: &[
            PeripheralPin {
                pin: "PA4",
                signal: "OUT1",
                af: None,
            },
            PeripheralPin {
                pin: "PA5",
                signal: "OUT2",
                af: None,
            },
        ],
        dma_channels: &[
            PeripheralDmaChannel {
                signal: "CH1",
                channel: Some("DMA1_CH5"),
                dmamux: None,
                dma: None,
                request: Some(7),
            },
            PeripheralDmaChannel {
                signal: "CH2",
                channel: Some("DMA1_CH6"),
                dmamux: None,
                dma: None,
                request: Some(7),
            },
        ],
        interrupts: &[PeripheralInterrupt {
            signal: "GLOBAL",
            interrupt: "TIM6_DAC",
        }],
    },
    Peripheral {
        name: "DBGMCU",
        address: 3758366720,
        registers: Some(PeripheralRegisters {
            kind: "dbgmcu",
            version: "f7",
            block: "DBGMCU",
            ir: &dbgmcu::REGISTERS,
        }),
        rcc: None,
        pins: &[],
        dma_channels: &[],
        interrupts: &[],
    },
    Peripheral {
        name: "DCMI",
        address: 1342504960,
        registers: Some(PeripheralRegisters {
            kind: "dcmi",
            version: "v1",
            block: "DCMI",
            ir: &dcmi::REGISTERS,
        }),
        rcc: Some(PeripheralRcc {
            clock: "hclk2",
            enable: Some(PeripheralRccRegister {
                register: "ahb2enr",
                field: "dcmien",
            }),
            reset: Some(PeripheralRccRegister {
                register: "ahb2rstr",
                field: "dcmirst",
            }),
            mux: None,
            stop_mode: StopMode::Stop1,
        }),
        pins: &[
            PeripheralPin {
                pin: "PA10",
                signal: "D1",
                af: Some(13),
            },
            PeripheralPin {
                pin: "PA4",
                signal: "HSYNC",
                af: Some(13),
            },
            PeripheralPin {
                pin: "PA6",
                signal: "PIXCLK",
                af: Some(13),
            },
            PeripheralPin {
                pin: "PA9",
                signal: "D0",
                af: Some(13),
            },
            PeripheralPin {
                pin: "PB5",
                signal: "D10",
                af: Some(13),
            },
            PeripheralPin {
                pin: "PB6",
                signal: "D5",
                af: Some(13),
            },
            PeripheralPin {
                pin: "PB7",
                signal: "VSYNC",
                af: Some(13),
            },
            PeripheralPin {
                pin: "PB8",
                signal: "D6",
                af: Some(13),
            },
            PeripheralPin {
                pin: "PB9",
                signal: "D7",
                af: Some(13),
            },
            PeripheralPin {
                pin: "PC10",
                signal: "D8",
                af: Some(13),
            },
            PeripheralPin {
                pin: "PC11",
                signal: "D4",
                af: Some(13),
            },
            PeripheralPin {
                pin: "PC12",
                signal: "D9",
                af: Some(13),
            },
            PeripheralPin {
                pin: "PC6",
                signal: "D0",
                af: Some(13),
            },
            PeripheralPin {
                pin: "PC7",
                signal: "D1",
                af: Some(13),
            },
            PeripheralPin {
                pin: "PC8",
                signal: "D2",
                af: Some(13),
            },
            PeripheralPin {
                pin: "PC9",
                signal: "D3",
                af: Some(13),
            },
            PeripheralPin {
                pin: "PD2",
                signal: "D11",
                af: Some(13),
            },
            PeripheralPin {
                pin: "PD3",
                signal: "D5",
                af: Some(13),
            },
            PeripheralPin {
                pin: "PD6",
                signal: "D10",
                af: Some(13),
            },
            PeripheralPin {
                pin: "PE0",
                signal: "D2",
                af: Some(13),
            },
            PeripheralPin {
                pin: "PE1",
                signal: "D3",
                af: Some(13),
            },
            PeripheralPin {
                pin: "PE4",
                signal: "D4",
                af: Some(13),
            },
            PeripheralPin {
                pin: "PE5",
                signal: "D6",
                af: Some(13),
            },
            PeripheralPin {
                pin: "PE6",
                signal: "D7",
                af: Some(13),
            },
        ],
        dma_channels: &[
            PeripheralDmaChannel {
                signal: "DCMI",
                channel: Some("DMA2_CH1"),
                dmamux: None,
                dma: None,
                request: Some(1),
            },
            PeripheralDmaChannel {
                signal: "DCMI",
                channel: Some("DMA2_CH7"),
                dmamux: None,
                dma: None,
                request: Some(1),
            },
        ],
        interrupts: &[PeripheralInterrupt {
            signal: "GLOBAL",
            interrupt: "DCMI",
        }],
    },
    Peripheral {
        name: "DFSDM1",
        address: 1073837056,
        registers: None,
        rcc: Some(PeripheralRcc {
            clock: "pclk2",
            enable: Some(PeripheralRccRegister {
                register: "apb2enr",
                field: "dfsdm1en",
            }),
            reset: Some(PeripheralRccRegister {
                register: "apb2rstr",
                field: "dfsdm1rst",
            }),
            mux: None,
            stop_mode: StopMode::Stop1,
        }),
        pins: &[
            PeripheralPin {
                pin: "PB0",
                signal: "CKOUT",
                af: Some(6),
            },
            PeripheralPin {
                pin: "PB1",
                signal: "DATIN1",
                af: Some(6),
            },
            PeripheralPin {
                pin: "PB10",
                signal: "DATIN7",
                af: Some(6),
            },
            PeripheralPin {
                pin: "PB11",
                signal: "CKIN7",
                af: Some(6),
            },
            PeripheralPin {
                pin: "PB12",
                signal: "DATIN1",
                af: Some(6),
            },
            PeripheralPin {
                pin: "PB13",
                signal: "CKIN1",
                af: Some(6),
            },
            PeripheralPin {
                pin: "PB14",
                signal: "DATIN2",
                af: Some(6),
            },
            PeripheralPin {
                pin: "PB15",
                signal: "CKIN2",
                af: Some(6),
            },
            PeripheralPin {
                pin: "PB2",
                signal: "CKIN1",
                af: Some(10),
            },
            PeripheralPin {
                pin: "PB6",
                signal: "DATIN5",
                af: Some(6),
            },
            PeripheralPin {
                pin: "PB7",
                signal: "CKIN5",
                af: Some(6),
            },
            PeripheralPin {
                pin: "PB8",
                signal: "CKIN7",
                af: Some(6),
            },
            PeripheralPin {
                pin: "PB9",
                signal: "DATIN7",
                af: Some(6),
            },
            PeripheralPin {
                pin: "PC0",
                signal: "CKIN0",
                af: Some(3),
            },
            PeripheralPin {
                pin: "PC0",
                signal: "DATIN4",
                af: Some(6),
            },
            PeripheralPin {
                pin: "PC1",
                signal: "CKIN4",
                af: Some(10),
            },
            PeripheralPin {
                pin: "PC1",
                signal: "DATIN0",
                af: Some(3),
            },
            PeripheralPin {
                pin: "PC10",
                signal: "CKIN5",
                af: Some(3),
            },
            PeripheralPin {
                pin: "PC11",
                signal: "DATIN5",
                af: Some(3),
            },
            PeripheralPin {
                pin: "PC2",
                signal: "CKIN1",
                af: Some(3),
            },
            PeripheralPin {
                pin: "PC2",
                signal: "CKOUT",
                af: Some(6),
            },
            PeripheralPin {
                pin: "PC3",
                signal: "DATIN1",
                af: Some(3),
            },
            PeripheralPin {
                pin: "PC4",
                signal: "CKIN2",
                af: Some(3),
            },
            PeripheralPin {
                pin: "PC5",
                signal: "DATIN2",
                af: Some(3),
            },
            PeripheralPin {
                pin: "PC6",
                signal: "CKIN3",
                af: Some(7),
            },
            PeripheralPin {
                pin: "PC7",
                signal: "DATIN3",
                af: Some(7),
            },
            PeripheralPin {
                pin: "PD0",
                signal: "CKIN6",
                af: Some(3),
            },
            PeripheralPin {
                pin: "PD0",
                signal: "DATIN7",
                af: Some(6),
            },
            PeripheralPin {
                pin: "PD1",
                signal: "CKIN7",
                af: Some(6),
            },
            PeripheralPin {
                pin: "PD1",
                signal: "DATIN6",
                af: Some(3),
            },
            PeripheralPin {
                pin: "PD10",
                signal: "CKOUT",
                af: Some(3),
            },
            PeripheralPin {
                pin: "PD3",
                signal: "CKOUT",
                af: Some(3),
            },
            PeripheralPin {
                pin: "PD3",
                signal: "DATIN0",
                af: Some(6),
            },
            PeripheralPin {
                pin: "PD4",
                signal: "CKIN0",
                af: Some(6),
            },
            PeripheralPin {
                pin: "PD6",
                signal: "CKIN4",
                af: Some(3),
            },
            PeripheralPin {
                pin: "PD6",
                signal: "DATIN1",
                af: Some(10),
            },
            PeripheralPin {
                pin: "PD7",
                signal: "CKIN1",
                af: Some(6),
            },
            PeripheralPin {
                pin: "PD7",
                signal: "DATIN4",
                af: Some(3),
            },
            PeripheralPin {
                pin: "PD8",
                signal: "CKIN3",
                af: Some(3),
            },
            PeripheralPin {
                pin: "PD9",
                signal: "DATIN3",
                af: Some(3),
            },
            PeripheralPin {
                pin: "PE10",
                signal: "DATIN4",
                af: Some(6),
            },
            PeripheralPin {
                pin: "PE11",
                signal: "CKIN4",
                af: Some(6),
            },
            PeripheralPin {
                pin: "PE12",
                signal: "DATIN5",
                af: Some(6),
            },
            PeripheralPin {
                pin: "PE13",
                signal: "CKIN5",
                af: Some(6),
            },
            PeripheralPin {
                pin: "PE4",
                signal: "DATIN3",
                af: Some(10),
            },
            PeripheralPin {
                pin: "PE5",
                signal: "CKIN3",
                af: Some(10),
            },
            PeripheralPin {
                pin: "PE7",
                signal: "DATIN2",
                af: Some(6),
            },
            PeripheralPin {
                pin: "PE8",
                signal: "CKIN2",
                af: Some(6),
            },
            PeripheralPin {
                pin: "PE9",
                signal: "CKOUT",
                af: Some(6),
            },
        ],
        dma_channels: &[
            PeripheralDmaChannel {
                signal: "FLT0",
                channel: Some("DMA2_CH0"),
                dmamux: None,
                dma: None,
                request: Some(8),
            },
            PeripheralDmaChannel {
                signal: "FLT1",
                channel: Some("DMA2_CH1"),
                dmamux: None,
                dma: None,
                request: Some(8),
            },
            PeripheralDmaChannel {
                signal: "FLT2",
                channel: Some("DMA2_CH2"),
                dmamux: None,
                dma: None,
                request: Some(8),
            },
            PeripheralDmaChannel {
                signal: "FLT3",
                channel: Some("DMA2_CH3"),
                dmamux: None,
                dma: None,
                request: Some(8),
            },
            PeripheralDmaChannel {
                signal: "FLT0",
                channel: Some("DMA2_CH4"),
                dmamux: None,
                dma: None,
                request: Some(8),
            },
            PeripheralDmaChannel {
                signal: "FLT1",
                channel: Some("DMA2_CH5"),
                dmamux: None,
                dma: None,
                request: Some(8),
            },
            PeripheralDmaChannel {
                signal: "FLT2",
                channel: Some("DMA2_CH6"),
                dmamux: None,
                dma: None,
                request: Some(8),
            },
            PeripheralDmaChannel {
                signal: "FLT3",
                channel: Some("DMA2_CH7"),
                dmamux: None,
                dma: None,
                request: Some(8),
            },
        ],
        interrupts: &[],
    },
    Peripheral {
        name: "DMA1",
        address: 1073897472,
        registers: Some(PeripheralRegisters {
            kind: "dma",
            version: "v2",
            block: "DMA",
            ir: &dma::REGISTERS,
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
                signal: "CH0",
                interrupt: "DMA1_STREAM0",
            },
            PeripheralInterrupt {
                signal: "CH1",
                interrupt: "DMA1_STREAM1",
            },
            PeripheralInterrupt {
                signal: "CH2",
                interrupt: "DMA1_STREAM2",
            },
            PeripheralInterrupt {
                signal: "CH3",
                interrupt: "DMA1_STREAM3",
            },
            PeripheralInterrupt {
                signal: "CH4",
                interrupt: "DMA1_STREAM4",
            },
            PeripheralInterrupt {
                signal: "CH5",
                interrupt: "DMA1_STREAM5",
            },
            PeripheralInterrupt {
                signal: "CH6",
                interrupt: "DMA1_STREAM6",
            },
            PeripheralInterrupt {
                signal: "CH7",
                interrupt: "DMA1_STREAM7",
            },
        ],
    },
    Peripheral {
        name: "DMA2",
        address: 1073898496,
        registers: Some(PeripheralRegisters {
            kind: "dma",
            version: "v2",
            block: "DMA",
            ir: &dma::REGISTERS,
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
                signal: "CH0",
                interrupt: "DMA2_STREAM0",
            },
            PeripheralInterrupt {
                signal: "CH1",
                interrupt: "DMA2_STREAM1",
            },
            PeripheralInterrupt {
                signal: "CH2",
                interrupt: "DMA2_STREAM2",
            },
            PeripheralInterrupt {
                signal: "CH3",
                interrupt: "DMA2_STREAM3",
            },
            PeripheralInterrupt {
                signal: "CH4",
                interrupt: "DMA2_STREAM4",
            },
            PeripheralInterrupt {
                signal: "CH5",
                interrupt: "DMA2_STREAM5",
            },
            PeripheralInterrupt {
                signal: "CH6",
                interrupt: "DMA2_STREAM6",
            },
            PeripheralInterrupt {
                signal: "CH7",
                interrupt: "DMA2_STREAM7",
            },
        ],
    },
    Peripheral {
        name: "DMA2D",
        address: 1073917952,
        registers: Some(PeripheralRegisters {
            kind: "dma2d",
            version: "v1",
            block: "DMA2D",
            ir: &dma2d::REGISTERS,
        }),
        rcc: Some(PeripheralRcc {
            clock: "hclk1",
            enable: Some(PeripheralRccRegister {
                register: "ahb1enr",
                field: "dma2den",
            }),
            reset: Some(PeripheralRccRegister {
                register: "ahb1rstr",
                field: "dma2drst",
            }),
            mux: None,
            stop_mode: StopMode::Stop1,
        }),
        pins: &[],
        dma_channels: &[],
        interrupts: &[PeripheralInterrupt {
            signal: "GLOBAL",
            interrupt: "DMA2D",
        }],
    },
    Peripheral {
        name: "ETH",
        address: 1073905664,
        registers: Some(PeripheralRegisters {
            kind: "eth",
            version: "v1c",
            block: "ETH",
            ir: &eth::REGISTERS,
        }),
        rcc: Some(PeripheralRcc {
            clock: "hclk1",
            enable: Some(PeripheralRccRegister {
                register: "ahb1enr",
                field: "ethen",
            }),
            reset: Some(PeripheralRccRegister {
                register: "ahb1rstr",
                field: "ethrst",
            }),
            mux: None,
            stop_mode: StopMode::Stop1,
        }),
        pins: &[
            PeripheralPin {
                pin: "PA0",
                signal: "CRS",
                af: Some(11),
            },
            PeripheralPin {
                pin: "PA1",
                signal: "REF_CLK",
                af: Some(11),
            },
            PeripheralPin {
                pin: "PA1",
                signal: "RX_CLK",
                af: Some(11),
            },
            PeripheralPin {
                pin: "PA2",
                signal: "MDIO",
                af: Some(11),
            },
            PeripheralPin {
                pin: "PA3",
                signal: "COL",
                af: Some(11),
            },
            PeripheralPin {
                pin: "PA7",
                signal: "CRS_DV",
                af: Some(11),
            },
            PeripheralPin {
                pin: "PA7",
                signal: "RX_DV",
                af: Some(11),
            },
            PeripheralPin {
                pin: "PB0",
                signal: "RXD2",
                af: Some(11),
            },
            PeripheralPin {
                pin: "PB1",
                signal: "RXD3",
                af: Some(11),
            },
            PeripheralPin {
                pin: "PB10",
                signal: "RX_ER",
                af: Some(11),
            },
            PeripheralPin {
                pin: "PB11",
                signal: "TX_EN",
                af: Some(11),
            },
            PeripheralPin {
                pin: "PB12",
                signal: "TXD0",
                af: Some(11),
            },
            PeripheralPin {
                pin: "PB13",
                signal: "TXD1",
                af: Some(11),
            },
            PeripheralPin {
                pin: "PB5",
                signal: "PPS_OUT",
                af: Some(11),
            },
            PeripheralPin {
                pin: "PB8",
                signal: "TXD3",
                af: Some(11),
            },
            PeripheralPin {
                pin: "PC1",
                signal: "MDC",
                af: Some(11),
            },
            PeripheralPin {
                pin: "PC2",
                signal: "TXD2",
                af: Some(11),
            },
            PeripheralPin {
                pin: "PC3",
                signal: "TX_CLK",
                af: Some(11),
            },
            PeripheralPin {
                pin: "PC4",
                signal: "RXD0",
                af: Some(11),
            },
            PeripheralPin {
                pin: "PC5",
                signal: "RXD1",
                af: Some(11),
            },
            PeripheralPin {
                pin: "PE2",
                signal: "TXD3",
                af: Some(11),
            },
        ],
        dma_channels: &[],
        interrupts: &[
            PeripheralInterrupt {
                signal: "GLOBAL",
                interrupt: "ETH",
            },
            PeripheralInterrupt {
                signal: "WKUP",
                interrupt: "ETH",
            },
            PeripheralInterrupt {
                signal: "WKUP",
                interrupt: "ETH_WKUP",
            },
        ],
    },
    Peripheral {
        name: "EXTI",
        address: 1073822720,
        registers: Some(PeripheralRegisters {
            kind: "exti",
            version: "v1",
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
        address: 1073888256,
        registers: Some(PeripheralRegisters {
            kind: "flash",
            version: "f7",
            block: "FLASH",
            ir: &flash::REGISTERS,
        }),
        rcc: None,
        pins: &[],
        dma_channels: &[],
        interrupts: &[PeripheralInterrupt {
            signal: "GLOBAL",
            interrupt: "FLASH",
        }],
    },
    Peripheral {
        name: "FMC",
        address: 2684354560,
        registers: Some(PeripheralRegisters {
            kind: "fmc",
            version: "v2x1",
            block: "FMC",
            ir: &fmc::REGISTERS,
        }),
        rcc: Some(PeripheralRcc {
            clock: "hclk3",
            enable: Some(PeripheralRccRegister {
                register: "ahb3enr",
                field: "fmcen",
            }),
            reset: Some(PeripheralRccRegister {
                register: "ahb3rstr",
                field: "fmcrst",
            }),
            mux: None,
            stop_mode: StopMode::Stop1,
        }),
        pins: &[
            PeripheralPin {
                pin: "PA7",
                signal: "SDNWE",
                af: Some(12),
            },
            PeripheralPin {
                pin: "PB5",
                signal: "SDCKE1",
                af: Some(12),
            },
            PeripheralPin {
                pin: "PB6",
                signal: "SDNE1",
                af: Some(12),
            },
            PeripheralPin {
                pin: "PB7",
                signal: "NL",
                af: Some(12),
            },
            PeripheralPin {
                pin: "PC0",
                signal: "SDNWE",
                af: Some(12),
            },
            PeripheralPin {
                pin: "PC2",
                signal: "SDNE0",
                af: Some(12),
            },
            PeripheralPin {
                pin: "PC3",
                signal: "SDCKE0",
                af: Some(12),
            },
            PeripheralPin {
                pin: "PC4",
                signal: "SDNE0",
                af: Some(12),
            },
            PeripheralPin {
                pin: "PC5",
                signal: "SDCKE0",
                af: Some(12),
            },
            PeripheralPin {
                pin: "PC6",
                signal: "NWAIT",
                af: Some(9),
            },
            PeripheralPin {
                pin: "PC7",
                signal: "NE1",
                af: Some(9),
            },
            PeripheralPin {
                pin: "PC8",
                signal: "NCE",
                af: Some(9),
            },
            PeripheralPin {
                pin: "PC8",
                signal: "NE2",
                af: Some(9),
            },
            PeripheralPin {
                pin: "PD0",
                signal: "D2",
                af: Some(12),
            },
            PeripheralPin {
                pin: "PD0",
                signal: "DA2",
                af: Some(12),
            },
            PeripheralPin {
                pin: "PD1",
                signal: "D3",
                af: Some(12),
            },
            PeripheralPin {
                pin: "PD1",
                signal: "DA3",
                af: Some(12),
            },
            PeripheralPin {
                pin: "PD10",
                signal: "D15",
                af: Some(12),
            },
            PeripheralPin {
                pin: "PD10",
                signal: "DA15",
                af: Some(12),
            },
            PeripheralPin {
                pin: "PD11",
                signal: "A16",
                af: Some(12),
            },
            PeripheralPin {
                pin: "PD11",
                signal: "CLE",
                af: Some(12),
            },
            PeripheralPin {
                pin: "PD12",
                signal: "A17",
                af: Some(12),
            },
            PeripheralPin {
                pin: "PD12",
                signal: "ALE",
                af: Some(12),
            },
            PeripheralPin {
                pin: "PD13",
                signal: "A18",
                af: Some(12),
            },
            PeripheralPin {
                pin: "PD14",
                signal: "D0",
                af: Some(12),
            },
            PeripheralPin {
                pin: "PD14",
                signal: "DA0",
                af: Some(12),
            },
            PeripheralPin {
                pin: "PD15",
                signal: "D1",
                af: Some(12),
            },
            PeripheralPin {
                pin: "PD15",
                signal: "DA1",
                af: Some(12),
            },
            PeripheralPin {
                pin: "PD3",
                signal: "CLK",
                af: Some(12),
            },
            PeripheralPin {
                pin: "PD4",
                signal: "NOE",
                af: Some(12),
            },
            PeripheralPin {
                pin: "PD5",
                signal: "NWE",
                af: Some(12),
            },
            PeripheralPin {
                pin: "PD6",
                signal: "NWAIT",
                af: Some(12),
            },
            PeripheralPin {
                pin: "PD7",
                signal: "NE1",
                af: Some(12),
            },
            PeripheralPin {
                pin: "PD8",
                signal: "D13",
                af: Some(12),
            },
            PeripheralPin {
                pin: "PD8",
                signal: "DA13",
                af: Some(12),
            },
            PeripheralPin {
                pin: "PD9",
                signal: "D14",
                af: Some(12),
            },
            PeripheralPin {
                pin: "PD9",
                signal: "DA14",
                af: Some(12),
            },
            PeripheralPin {
                pin: "PE0",
                signal: "NBL0",
                af: Some(12),
            },
            PeripheralPin {
                pin: "PE1",
                signal: "NBL1",
                af: Some(12),
            },
            PeripheralPin {
                pin: "PE10",
                signal: "D7",
                af: Some(12),
            },
            PeripheralPin {
                pin: "PE10",
                signal: "DA7",
                af: Some(12),
            },
            PeripheralPin {
                pin: "PE11",
                signal: "D8",
                af: Some(12),
            },
            PeripheralPin {
                pin: "PE11",
                signal: "DA8",
                af: Some(12),
            },
            PeripheralPin {
                pin: "PE12",
                signal: "D9",
                af: Some(12),
            },
            PeripheralPin {
                pin: "PE12",
                signal: "DA9",
                af: Some(12),
            },
            PeripheralPin {
                pin: "PE13",
                signal: "D10",
                af: Some(12),
            },
            PeripheralPin {
                pin: "PE13",
                signal: "DA10",
                af: Some(12),
            },
            PeripheralPin {
                pin: "PE14",
                signal: "D11",
                af: Some(12),
            },
            PeripheralPin {
                pin: "PE14",
                signal: "DA11",
                af: Some(12),
            },
            PeripheralPin {
                pin: "PE15",
                signal: "D12",
                af: Some(12),
            },
            PeripheralPin {
                pin: "PE15",
                signal: "DA12",
                af: Some(12),
            },
            PeripheralPin {
                pin: "PE2",
                signal: "A23",
                af: Some(12),
            },
            PeripheralPin {
                pin: "PE3",
                signal: "A19",
                af: Some(12),
            },
            PeripheralPin {
                pin: "PE4",
                signal: "A20",
                af: Some(12),
            },
            PeripheralPin {
                pin: "PE5",
                signal: "A21",
                af: Some(12),
            },
            PeripheralPin {
                pin: "PE6",
                signal: "A22",
                af: Some(12),
            },
            PeripheralPin {
                pin: "PE7",
                signal: "D4",
                af: Some(12),
            },
            PeripheralPin {
                pin: "PE7",
                signal: "DA4",
                af: Some(12),
            },
            PeripheralPin {
                pin: "PE8",
                signal: "D5",
                af: Some(12),
            },
            PeripheralPin {
                pin: "PE8",
                signal: "DA5",
                af: Some(12),
            },
            PeripheralPin {
                pin: "PE9",
                signal: "D6",
                af: Some(12),
            },
            PeripheralPin {
                pin: "PE9",
                signal: "DA6",
                af: Some(12),
            },
        ],
        dma_channels: &[],
        interrupts: &[PeripheralInterrupt {
            signal: "GLOBAL",
            interrupt: "FMC",
        }],
    },
    Peripheral {
        name: "GPIOA",
        address: 1073872896,
        registers: Some(PeripheralRegisters {
            kind: "gpio",
            version: "v2",
            block: "GPIO",
            ir: &gpio::REGISTERS,
        }),
        rcc: Some(PeripheralRcc {
            clock: "hclk1",
            enable: Some(PeripheralRccRegister {
                register: "ahb1enr",
                field: "gpioaen",
            }),
            reset: Some(PeripheralRccRegister {
                register: "ahb1rstr",
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
        address: 1073873920,
        registers: Some(PeripheralRegisters {
            kind: "gpio",
            version: "v2",
            block: "GPIO",
            ir: &gpio::REGISTERS,
        }),
        rcc: Some(PeripheralRcc {
            clock: "hclk1",
            enable: Some(PeripheralRccRegister {
                register: "ahb1enr",
                field: "gpioben",
            }),
            reset: Some(PeripheralRccRegister {
                register: "ahb1rstr",
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
        address: 1073874944,
        registers: Some(PeripheralRegisters {
            kind: "gpio",
            version: "v2",
            block: "GPIO",
            ir: &gpio::REGISTERS,
        }),
        rcc: Some(PeripheralRcc {
            clock: "hclk1",
            enable: Some(PeripheralRccRegister {
                register: "ahb1enr",
                field: "gpiocen",
            }),
            reset: Some(PeripheralRccRegister {
                register: "ahb1rstr",
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
        name: "GPIOD",
        address: 1073875968,
        registers: Some(PeripheralRegisters {
            kind: "gpio",
            version: "v2",
            block: "GPIO",
            ir: &gpio::REGISTERS,
        }),
        rcc: Some(PeripheralRcc {
            clock: "hclk1",
            enable: Some(PeripheralRccRegister {
                register: "ahb1enr",
                field: "gpioden",
            }),
            reset: Some(PeripheralRccRegister {
                register: "ahb1rstr",
                field: "gpiodrst",
            }),
            mux: None,
            stop_mode: StopMode::Stop1,
        }),
        pins: &[],
        dma_channels: &[],
        interrupts: &[],
    },
    Peripheral {
        name: "GPIOE",
        address: 1073876992,
        registers: Some(PeripheralRegisters {
            kind: "gpio",
            version: "v2",
            block: "GPIO",
            ir: &gpio::REGISTERS,
        }),
        rcc: Some(PeripheralRcc {
            clock: "hclk1",
            enable: Some(PeripheralRccRegister {
                register: "ahb1enr",
                field: "gpioeen",
            }),
            reset: Some(PeripheralRccRegister {
                register: "ahb1rstr",
                field: "gpioerst",
            }),
            mux: None,
            stop_mode: StopMode::Stop1,
        }),
        pins: &[],
        dma_channels: &[],
        interrupts: &[],
    },
    Peripheral {
        name: "GPIOF",
        address: 1073878016,
        registers: Some(PeripheralRegisters {
            kind: "gpio",
            version: "v2",
            block: "GPIO",
            ir: &gpio::REGISTERS,
        }),
        rcc: Some(PeripheralRcc {
            clock: "hclk1",
            enable: Some(PeripheralRccRegister {
                register: "ahb1enr",
                field: "gpiofen",
            }),
            reset: Some(PeripheralRccRegister {
                register: "ahb1rstr",
                field: "gpiofrst",
            }),
            mux: None,
            stop_mode: StopMode::Stop1,
        }),
        pins: &[],
        dma_channels: &[],
        interrupts: &[],
    },
    Peripheral {
        name: "GPIOG",
        address: 1073879040,
        registers: Some(PeripheralRegisters {
            kind: "gpio",
            version: "v2",
            block: "GPIO",
            ir: &gpio::REGISTERS,
        }),
        rcc: Some(PeripheralRcc {
            clock: "hclk1",
            enable: Some(PeripheralRccRegister {
                register: "ahb1enr",
                field: "gpiogen",
            }),
            reset: Some(PeripheralRccRegister {
                register: "ahb1rstr",
                field: "gpiogrst",
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
        address: 1073880064,
        registers: Some(PeripheralRegisters {
            kind: "gpio",
            version: "v2",
            block: "GPIO",
            ir: &gpio::REGISTERS,
        }),
        rcc: Some(PeripheralRcc {
            clock: "hclk1",
            enable: Some(PeripheralRccRegister {
                register: "ahb1enr",
                field: "gpiohen",
            }),
            reset: Some(PeripheralRccRegister {
                register: "ahb1rstr",
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
        name: "GPIOI",
        address: 1073881088,
        registers: Some(PeripheralRegisters {
            kind: "gpio",
            version: "v2",
            block: "GPIO",
            ir: &gpio::REGISTERS,
        }),
        rcc: Some(PeripheralRcc {
            clock: "hclk1",
            enable: Some(PeripheralRccRegister {
                register: "ahb1enr",
                field: "gpioien",
            }),
            reset: Some(PeripheralRccRegister {
                register: "ahb1rstr",
                field: "gpioirst",
            }),
            mux: None,
            stop_mode: StopMode::Stop1,
        }),
        pins: &[],
        dma_channels: &[],
        interrupts: &[],
    },
    Peripheral {
        name: "GPIOJ",
        address: 1073882112,
        registers: Some(PeripheralRegisters {
            kind: "gpio",
            version: "v2",
            block: "GPIO",
            ir: &gpio::REGISTERS,
        }),
        rcc: Some(PeripheralRcc {
            clock: "hclk1",
            enable: Some(PeripheralRccRegister {
                register: "ahb1enr",
                field: "gpiojen",
            }),
            reset: Some(PeripheralRccRegister {
                register: "ahb1rstr",
                field: "gpiojrst",
            }),
            mux: None,
            stop_mode: StopMode::Stop1,
        }),
        pins: &[],
        dma_channels: &[],
        interrupts: &[],
    },
    Peripheral {
        name: "GPIOK",
        address: 1073883136,
        registers: Some(PeripheralRegisters {
            kind: "gpio",
            version: "v2",
            block: "GPIO",
            ir: &gpio::REGISTERS,
        }),
        rcc: Some(PeripheralRcc {
            clock: "hclk1",
            enable: Some(PeripheralRccRegister {
                register: "ahb1enr",
                field: "gpioken",
            }),
            reset: Some(PeripheralRccRegister {
                register: "ahb1rstr",
                field: "gpiokrst",
            }),
            mux: None,
            stop_mode: StopMode::Stop1,
        }),
        pins: &[],
        dma_channels: &[],
        interrupts: &[],
    },
    Peripheral {
        name: "HASH",
        address: 1342571520,
        registers: None,
        rcc: Some(PeripheralRcc {
            clock: "hclk2",
            enable: Some(PeripheralRccRegister {
                register: "ahb2enr",
                field: "hashen",
            }),
            reset: None,
            mux: None,
            stop_mode: StopMode::Stop1,
        }),
        pins: &[],
        dma_channels: &[PeripheralDmaChannel {
            signal: "IN",
            channel: Some("DMA2_CH7"),
            dmamux: None,
            dma: None,
            request: Some(2),
        }],
        interrupts: &[PeripheralInterrupt {
            signal: "GLOBAL",
            interrupt: "HASH_RNG",
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
                register: "apb1enr",
                field: "i2c1en",
            }),
            reset: Some(PeripheralRccRegister {
                register: "apb1rstr",
                field: "i2c1rst",
            }),
            mux: None,
            stop_mode: StopMode::Stop1,
        }),
        pins: &[
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
            PeripheralPin {
                pin: "PB9",
                signal: "SDA",
                af: Some(4),
            },
        ],
        dma_channels: &[
            PeripheralDmaChannel {
                signal: "RX",
                channel: Some("DMA1_CH0"),
                dmamux: None,
                dma: None,
                request: Some(1),
            },
            PeripheralDmaChannel {
                signal: "RX",
                channel: Some("DMA1_CH5"),
                dmamux: None,
                dma: None,
                request: Some(1),
            },
            PeripheralDmaChannel {
                signal: "TX",
                channel: Some("DMA1_CH6"),
                dmamux: None,
                dma: None,
                request: Some(1),
            },
            PeripheralDmaChannel {
                signal: "TX",
                channel: Some("DMA1_CH7"),
                dmamux: None,
                dma: None,
                request: Some(1),
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
                register: "apb1enr",
                field: "i2c2en",
            }),
            reset: Some(PeripheralRccRegister {
                register: "apb1rstr",
                field: "i2c2rst",
            }),
            mux: None,
            stop_mode: StopMode::Stop1,
        }),
        pins: &[
            PeripheralPin {
                pin: "PB10",
                signal: "SCL",
                af: Some(4),
            },
            PeripheralPin {
                pin: "PB11",
                signal: "SDA",
                af: Some(4),
            },
            PeripheralPin {
                pin: "PB12",
                signal: "SMBA",
                af: Some(4),
            },
        ],
        dma_channels: &[
            PeripheralDmaChannel {
                signal: "RX",
                channel: Some("DMA1_CH2"),
                dmamux: None,
                dma: None,
                request: Some(7),
            },
            PeripheralDmaChannel {
                signal: "RX",
                channel: Some("DMA1_CH3"),
                dmamux: None,
                dma: None,
                request: Some(7),
            },
            PeripheralDmaChannel {
                signal: "TX",
                channel: Some("DMA1_CH4"),
                dmamux: None,
                dma: None,
                request: Some(8),
            },
            PeripheralDmaChannel {
                signal: "TX",
                channel: Some("DMA1_CH7"),
                dmamux: None,
                dma: None,
                request: Some(7),
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
                register: "apb1enr",
                field: "i2c3en",
            }),
            reset: Some(PeripheralRccRegister {
                register: "apb1rstr",
                field: "i2c3rst",
            }),
            mux: None,
            stop_mode: StopMode::Stop1,
        }),
        pins: &[
            PeripheralPin {
                pin: "PA8",
                signal: "SCL",
                af: Some(4),
            },
            PeripheralPin {
                pin: "PA9",
                signal: "SMBA",
                af: Some(4),
            },
            PeripheralPin {
                pin: "PC9",
                signal: "SDA",
                af: Some(4),
            },
        ],
        dma_channels: &[
            PeripheralDmaChannel {
                signal: "TX",
                channel: Some("DMA1_CH0"),
                dmamux: None,
                dma: None,
                request: Some(8),
            },
            PeripheralDmaChannel {
                signal: "RX",
                channel: Some("DMA1_CH1"),
                dmamux: None,
                dma: None,
                request: Some(1),
            },
            PeripheralDmaChannel {
                signal: "RX",
                channel: Some("DMA1_CH2"),
                dmamux: None,
                dma: None,
                request: Some(3),
            },
            PeripheralDmaChannel {
                signal: "TX",
                channel: Some("DMA1_CH4"),
                dmamux: None,
                dma: None,
                request: Some(3),
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
        name: "I2C4",
        address: 1073766400,
        registers: Some(PeripheralRegisters {
            kind: "i2c",
            version: "v2",
            block: "I2C",
            ir: &i2c::REGISTERS,
        }),
        rcc: Some(PeripheralRcc {
            clock: "pclk1",
            enable: Some(PeripheralRccRegister {
                register: "apb1enr",
                field: "i2c4en",
            }),
            reset: Some(PeripheralRccRegister {
                register: "apb1rstr",
                field: "i2c4rst",
            }),
            mux: None,
            stop_mode: StopMode::Stop1,
        }),
        pins: &[
            PeripheralPin {
                pin: "PB6",
                signal: "SCL",
                af: Some(11),
            },
            PeripheralPin {
                pin: "PB7",
                signal: "SDA",
                af: Some(11),
            },
            PeripheralPin {
                pin: "PB8",
                signal: "SCL",
                af: Some(1),
            },
            PeripheralPin {
                pin: "PB9",
                signal: "SDA",
                af: Some(1),
            },
            PeripheralPin {
                pin: "PB9",
                signal: "SMBA",
                af: Some(11),
            },
            PeripheralPin {
                pin: "PD11",
                signal: "SMBA",
                af: Some(4),
            },
            PeripheralPin {
                pin: "PD12",
                signal: "SCL",
                af: Some(4),
            },
            PeripheralPin {
                pin: "PD13",
                signal: "SDA",
                af: Some(4),
            },
        ],
        dma_channels: &[
            PeripheralDmaChannel {
                signal: "RX",
                channel: Some("DMA1_CH1"),
                dmamux: None,
                dma: None,
                request: Some(8),
            },
            PeripheralDmaChannel {
                signal: "RX",
                channel: Some("DMA1_CH2"),
                dmamux: None,
                dma: None,
                request: Some(2),
            },
            PeripheralDmaChannel {
                signal: "TX",
                channel: Some("DMA1_CH5"),
                dmamux: None,
                dma: None,
                request: Some(2),
            },
            PeripheralDmaChannel {
                signal: "TX",
                channel: Some("DMA1_CH6"),
                dmamux: None,
                dma: None,
                request: Some(8),
            },
        ],
        interrupts: &[
            PeripheralInterrupt {
                signal: "ER",
                interrupt: "I2C4_ER",
            },
            PeripheralInterrupt {
                signal: "EV",
                interrupt: "I2C4_EV",
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
        name: "JPEG",
        address: 1342509056,
        registers: Some(PeripheralRegisters {
            kind: "jpeg",
            version: "v1",
            block: "JPEG",
            ir: &jpeg::REGISTERS,
        }),
        rcc: Some(PeripheralRcc {
            clock: "hclk2",
            enable: Some(PeripheralRccRegister {
                register: "ahb2enr",
                field: "jpegen",
            }),
            reset: None,
            mux: None,
            stop_mode: StopMode::Stop1,
        }),
        pins: &[],
        dma_channels: &[
            PeripheralDmaChannel {
                signal: "IN",
                channel: Some("DMA2_CH0"),
                dmamux: None,
                dma: None,
                request: Some(9),
            },
            PeripheralDmaChannel {
                signal: "OUT",
                channel: Some("DMA2_CH1"),
                dmamux: None,
                dma: None,
                request: Some(9),
            },
            PeripheralDmaChannel {
                signal: "IN",
                channel: Some("DMA2_CH3"),
                dmamux: None,
                dma: None,
                request: Some(9),
            },
            PeripheralDmaChannel {
                signal: "OUT",
                channel: Some("DMA2_CH4"),
                dmamux: None,
                dma: None,
                request: Some(9),
            },
        ],
        interrupts: &[PeripheralInterrupt {
            signal: "GLOBAL",
            interrupt: "JPEG",
        }],
    },
    Peripheral {
        name: "LPTIM1",
        address: 1073751040,
        registers: Some(PeripheralRegisters {
            kind: "lptim",
            version: "v1",
            block: "LPTIM",
            ir: &lptim::REGISTERS,
        }),
        rcc: Some(PeripheralRcc {
            clock: "pclk1",
            enable: Some(PeripheralRccRegister {
                register: "apb1enr",
                field: "lptim1en",
            }),
            reset: Some(PeripheralRccRegister {
                register: "apb1rstr",
                field: "lptim1rst",
            }),
            mux: None,
            stop_mode: StopMode::Stop2,
        }),
        pins: &[
            PeripheralPin {
                pin: "PD12",
                signal: "IN1",
                af: Some(3),
            },
            PeripheralPin {
                pin: "PD13",
                signal: "OUT",
                af: Some(3),
            },
            PeripheralPin {
                pin: "PE0",
                signal: "ETR",
                af: Some(3),
            },
            PeripheralPin {
                pin: "PE1",
                signal: "IN2",
                af: Some(3),
            },
        ],
        dma_channels: &[],
        interrupts: &[PeripheralInterrupt {
            signal: "GLOBAL",
            interrupt: "LPTIM1",
        }],
    },
    Peripheral {
        name: "LTDC",
        address: 1073833984,
        registers: Some(PeripheralRegisters {
            kind: "ltdc",
            version: "v1",
            block: "LTDC",
            ir: &ltdc::REGISTERS,
        }),
        rcc: Some(PeripheralRcc {
            clock: "pclk2",
            enable: Some(PeripheralRccRegister {
                register: "apb2enr",
                field: "ltdcen",
            }),
            reset: Some(PeripheralRccRegister {
                register: "apb2rstr",
                field: "ltdcrst",
            }),
            mux: None,
            stop_mode: StopMode::Stop1,
        }),
        pins: &[
            PeripheralPin {
                pin: "PA1",
                signal: "R2",
                af: Some(14),
            },
            PeripheralPin {
                pin: "PA10",
                signal: "B1",
                af: Some(14),
            },
            PeripheralPin {
                pin: "PA10",
                signal: "B4",
                af: Some(9),
            },
            PeripheralPin {
                pin: "PA11",
                signal: "R4",
                af: Some(14),
            },
            PeripheralPin {
                pin: "PA12",
                signal: "R5",
                af: Some(14),
            },
            PeripheralPin {
                pin: "PA2",
                signal: "R1",
                af: Some(14),
            },
            PeripheralPin {
                pin: "PA3",
                signal: "B2",
                af: Some(9),
            },
            PeripheralPin {
                pin: "PA3",
                signal: "B5",
                af: Some(14),
            },
            PeripheralPin {
                pin: "PA4",
                signal: "VSYNC",
                af: Some(14),
            },
            PeripheralPin {
                pin: "PA5",
                signal: "R4",
                af: Some(14),
            },
            PeripheralPin {
                pin: "PA6",
                signal: "G2",
                af: Some(14),
            },
            PeripheralPin {
                pin: "PA8",
                signal: "B3",
                af: Some(13),
            },
            PeripheralPin {
                pin: "PA8",
                signal: "R6",
                af: Some(14),
            },
            PeripheralPin {
                pin: "PA9",
                signal: "R5",
                af: Some(14),
            },
            PeripheralPin {
                pin: "PB0",
                signal: "G1",
                af: Some(14),
            },
            PeripheralPin {
                pin: "PB0",
                signal: "R3",
                af: Some(9),
            },
            PeripheralPin {
                pin: "PB1",
                signal: "G0",
                af: Some(14),
            },
            PeripheralPin {
                pin: "PB1",
                signal: "R6",
                af: Some(9),
            },
            PeripheralPin {
                pin: "PB10",
                signal: "G4",
                af: Some(14),
            },
            PeripheralPin {
                pin: "PB11",
                signal: "G5",
                af: Some(14),
            },
            PeripheralPin {
                pin: "PB5",
                signal: "G7",
                af: Some(14),
            },
            PeripheralPin {
                pin: "PB8",
                signal: "B6",
                af: Some(14),
            },
            PeripheralPin {
                pin: "PB9",
                signal: "B7",
                af: Some(14),
            },
            PeripheralPin {
                pin: "PC0",
                signal: "R5",
                af: Some(14),
            },
            PeripheralPin {
                pin: "PC10",
                signal: "R2",
                af: Some(14),
            },
            PeripheralPin {
                pin: "PC6",
                signal: "HSYNC",
                af: Some(14),
            },
            PeripheralPin {
                pin: "PC7",
                signal: "G6",
                af: Some(14),
            },
            PeripheralPin {
                pin: "PC9",
                signal: "B2",
                af: Some(14),
            },
            PeripheralPin {
                pin: "PC9",
                signal: "G3",
                af: Some(10),
            },
            PeripheralPin {
                pin: "PD10",
                signal: "B3",
                af: Some(14),
            },
            PeripheralPin {
                pin: "PD3",
                signal: "G7",
                af: Some(14),
            },
            PeripheralPin {
                pin: "PD6",
                signal: "B2",
                af: Some(14),
            },
            PeripheralPin {
                pin: "PE11",
                signal: "G3",
                af: Some(14),
            },
            PeripheralPin {
                pin: "PE12",
                signal: "B4",
                af: Some(14),
            },
            PeripheralPin {
                pin: "PE13",
                signal: "DE",
                af: Some(14),
            },
            PeripheralPin {
                pin: "PE14",
                signal: "CLK",
                af: Some(14),
            },
            PeripheralPin {
                pin: "PE15",
                signal: "R7",
                af: Some(14),
            },
            PeripheralPin {
                pin: "PE4",
                signal: "B0",
                af: Some(14),
            },
            PeripheralPin {
                pin: "PE5",
                signal: "G0",
                af: Some(14),
            },
            PeripheralPin {
                pin: "PE6",
                signal: "G1",
                af: Some(14),
            },
        ],
        dma_channels: &[],
        interrupts: &[
            PeripheralInterrupt {
                signal: "ER",
                interrupt: "LTDC",
            },
            PeripheralInterrupt {
                signal: "ER",
                interrupt: "LTDC_ER",
            },
            PeripheralInterrupt {
                signal: "GLOBAL",
                interrupt: "LTDC",
            },
        ],
    },
    Peripheral {
        name: "MDIOS",
        address: 1073838080,
        registers: Some(PeripheralRegisters {
            kind: "mdios",
            version: "v1",
            block: "MDIOS",
            ir: &mdios::REGISTERS,
        }),
        rcc: Some(PeripheralRcc {
            clock: "pclk2",
            enable: Some(PeripheralRccRegister {
                register: "apb2enr",
                field: "mdiosen",
            }),
            reset: Some(PeripheralRccRegister {
                register: "apb2rstr",
                field: "mdiosrst",
            }),
            mux: None,
            stop_mode: StopMode::Stop1,
        }),
        pins: &[
            PeripheralPin {
                pin: "PA10",
                signal: "MDIO",
                af: Some(12),
            },
            PeripheralPin {
                pin: "PA2",
                signal: "MDIO",
                af: Some(12),
            },
            PeripheralPin {
                pin: "PA6",
                signal: "MDC",
                af: Some(12),
            },
            PeripheralPin {
                pin: "PC1",
                signal: "MDC",
                af: Some(12),
            },
        ],
        dma_channels: &[],
        interrupts: &[
            PeripheralInterrupt {
                signal: "GLOBAL",
                interrupt: "MDIOS",
            },
            PeripheralInterrupt {
                signal: "WKUP",
                interrupt: "MDIOS",
            },
        ],
    },
    Peripheral {
        name: "PWR",
        address: 1073770496,
        registers: Some(PeripheralRegisters {
            kind: "pwr",
            version: "f7",
            block: "PWR",
            ir: &pwr::REGISTERS,
        }),
        rcc: Some(PeripheralRcc {
            clock: "pclk1",
            enable: Some(PeripheralRccRegister {
                register: "apb1enr",
                field: "pwren",
            }),
            reset: Some(PeripheralRccRegister {
                register: "apb1rstr",
                field: "pwrrst",
            }),
            mux: None,
            stop_mode: StopMode::Stop1,
        }),
        pins: &[],
        dma_channels: &[],
        interrupts: &[],
    },
    Peripheral {
        name: "QUADSPI",
        address: 2684358656,
        registers: Some(PeripheralRegisters {
            kind: "quadspi",
            version: "v1",
            block: "QUADSPI",
            ir: &quadspi::REGISTERS,
        }),
        rcc: Some(PeripheralRcc {
            clock: "hclk3",
            enable: Some(PeripheralRccRegister {
                register: "ahb3enr",
                field: "quadspien",
            }),
            reset: Some(PeripheralRccRegister {
                register: "ahb3rstr",
                field: "quadspirst",
            }),
            mux: None,
            stop_mode: StopMode::Stop1,
        }),
        pins: &[
            PeripheralPin {
                pin: "PA1",
                signal: "BK1_IO3",
                af: Some(9),
            },
            PeripheralPin {
                pin: "PB10",
                signal: "BK1_NCS",
                af: Some(9),
            },
            PeripheralPin {
                pin: "PB2",
                signal: "CLK",
                af: Some(9),
            },
            PeripheralPin {
                pin: "PB6",
                signal: "BK1_NCS",
                af: Some(10),
            },
            PeripheralPin {
                pin: "PC10",
                signal: "BK1_IO1",
                af: Some(9),
            },
            PeripheralPin {
                pin: "PC11",
                signal: "BK2_NCS",
                af: Some(9),
            },
            PeripheralPin {
                pin: "PC9",
                signal: "BK1_IO0",
                af: Some(9),
            },
            PeripheralPin {
                pin: "PD11",
                signal: "BK1_IO0",
                af: Some(9),
            },
            PeripheralPin {
                pin: "PD12",
                signal: "BK1_IO1",
                af: Some(9),
            },
            PeripheralPin {
                pin: "PD13",
                signal: "BK1_IO3",
                af: Some(9),
            },
            PeripheralPin {
                pin: "PE10",
                signal: "BK2_IO3",
                af: Some(10),
            },
            PeripheralPin {
                pin: "PE2",
                signal: "BK1_IO2",
                af: Some(9),
            },
            PeripheralPin {
                pin: "PE7",
                signal: "BK2_IO0",
                af: Some(10),
            },
            PeripheralPin {
                pin: "PE8",
                signal: "BK2_IO1",
                af: Some(10),
            },
            PeripheralPin {
                pin: "PE9",
                signal: "BK2_IO2",
                af: Some(10),
            },
        ],
        dma_channels: &[
            PeripheralDmaChannel {
                signal: "QUADSPI",
                channel: Some("DMA2_CH2"),
                dmamux: None,
                dma: None,
                request: Some(11),
            },
            PeripheralDmaChannel {
                signal: "QUADSPI",
                channel: Some("DMA2_CH7"),
                dmamux: None,
                dma: None,
                request: Some(3),
            },
        ],
        interrupts: &[PeripheralInterrupt {
            signal: "GLOBAL",
            interrupt: "QUADSPI",
        }],
    },
    Peripheral {
        name: "RCC",
        address: 1073887232,
        registers: Some(PeripheralRegisters {
            kind: "rcc",
            version: "f7",
            block: "RCC",
            ir: &rcc::REGISTERS,
        }),
        rcc: None,
        pins: &[
            PeripheralPin {
                pin: "PA8",
                signal: "MCO_1",
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
            PeripheralPin {
                pin: "PC9",
                signal: "MCO_2",
                af: Some(0),
            },
            PeripheralPin {
                pin: "PH0",
                signal: "OSC_IN",
                af: None,
            },
            PeripheralPin {
                pin: "PH1",
                signal: "OSC_OUT",
                af: None,
            },
        ],
        dma_channels: &[],
        interrupts: &[PeripheralInterrupt {
            signal: "GLOBAL",
            interrupt: "RCC",
        }],
    },
    Peripheral {
        name: "RNG",
        address: 1342572544,
        registers: Some(PeripheralRegisters {
            kind: "rng",
            version: "v1",
            block: "RNG",
            ir: &rng::REGISTERS,
        }),
        rcc: Some(PeripheralRcc {
            clock: "hclk2",
            enable: Some(PeripheralRccRegister {
                register: "ahb2enr",
                field: "rngen",
            }),
            reset: Some(PeripheralRccRegister {
                register: "ahb2rstr",
                field: "rngrst",
            }),
            mux: None,
            stop_mode: StopMode::Stop1,
        }),
        pins: &[],
        dma_channels: &[],
        interrupts: &[PeripheralInterrupt {
            signal: "GLOBAL",
            interrupt: "HASH_RNG",
        }],
    },
    Peripheral {
        name: "RTC",
        address: 1073752064,
        registers: Some(PeripheralRegisters {
            kind: "rtc",
            version: "v2f7",
            block: "RTC",
            ir: &rtc::REGISTERS,
        }),
        rcc: Some(PeripheralRcc {
            clock: "pclk1",
            enable: Some(PeripheralRccRegister {
                register: "apb1enr",
                field: "rtcen",
            }),
            reset: None,
            mux: None,
            stop_mode: StopMode::Standby,
        }),
        pins: &[
            PeripheralPin {
                pin: "PB15",
                signal: "REFIN",
                af: Some(0),
            },
            PeripheralPin {
                pin: "PC1",
                signal: "TAMP3",
                af: None,
            },
            PeripheralPin {
                pin: "PC1",
                signal: "TS",
                af: None,
            },
            PeripheralPin {
                pin: "PC13",
                signal: "OUT",
                af: None,
            },
            PeripheralPin {
                pin: "PC13",
                signal: "TAMP1",
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
                signal: "STAMP",
                interrupt: "TAMP_STAMP",
            },
            PeripheralInterrupt {
                signal: "TAMP",
                interrupt: "TAMP_STAMP",
            },
            PeripheralInterrupt {
                signal: "WKUP",
                interrupt: "RTC_WKUP",
            },
        ],
    },
    Peripheral {
        name: "SAI1",
        address: 1073829888,
        registers: Some(PeripheralRegisters {
            kind: "sai",
            version: "v2",
            block: "SAI",
            ir: &sai::REGISTERS,
        }),
        rcc: Some(PeripheralRcc {
            clock: "pclk2",
            enable: Some(PeripheralRccRegister {
                register: "apb2enr",
                field: "sai1en",
            }),
            reset: Some(PeripheralRccRegister {
                register: "apb2rstr",
                field: "sai1rst",
            }),
            mux: None,
            stop_mode: StopMode::Stop1,
        }),
        pins: &[
            PeripheralPin {
                pin: "PB2",
                signal: "SD_A",
                af: Some(6),
            },
            PeripheralPin {
                pin: "PC1",
                signal: "SD_A",
                af: Some(6),
            },
            PeripheralPin {
                pin: "PD6",
                signal: "SD_A",
                af: Some(6),
            },
            PeripheralPin {
                pin: "PE2",
                signal: "MCLK_A",
                af: Some(6),
            },
            PeripheralPin {
                pin: "PE3",
                signal: "SD_B",
                af: Some(6),
            },
            PeripheralPin {
                pin: "PE4",
                signal: "FS_A",
                af: Some(6),
            },
            PeripheralPin {
                pin: "PE5",
                signal: "SCK_A",
                af: Some(6),
            },
            PeripheralPin {
                pin: "PE6",
                signal: "SD_A",
                af: Some(6),
            },
        ],
        dma_channels: &[
            PeripheralDmaChannel {
                signal: "B",
                channel: Some("DMA2_CH0"),
                dmamux: None,
                dma: None,
                request: Some(10),
            },
            PeripheralDmaChannel {
                signal: "A",
                channel: Some("DMA2_CH1"),
                dmamux: None,
                dma: None,
                request: Some(0),
            },
            PeripheralDmaChannel {
                signal: "A",
                channel: Some("DMA2_CH3"),
                dmamux: None,
                dma: None,
                request: Some(0),
            },
            PeripheralDmaChannel {
                signal: "B",
                channel: Some("DMA2_CH4"),
                dmamux: None,
                dma: None,
                request: Some(1),
            },
            PeripheralDmaChannel {
                signal: "B",
                channel: Some("DMA2_CH5"),
                dmamux: None,
                dma: None,
                request: Some(0),
            },
            PeripheralDmaChannel {
                signal: "A",
                channel: Some("DMA2_CH6"),
                dmamux: None,
                dma: None,
                request: Some(10),
            },
        ],
        interrupts: &[PeripheralInterrupt {
            signal: "GLOBAL",
            interrupt: "SAI1",
        }],
    },
    Peripheral {
        name: "SAI2",
        address: 1073830912,
        registers: Some(PeripheralRegisters {
            kind: "sai",
            version: "v2",
            block: "SAI",
            ir: &sai::REGISTERS,
        }),
        rcc: Some(PeripheralRcc {
            clock: "pclk2",
            enable: Some(PeripheralRccRegister {
                register: "apb2enr",
                field: "sai2en",
            }),
            reset: Some(PeripheralRccRegister {
                register: "apb2rstr",
                field: "sai2rst",
            }),
            mux: None,
            stop_mode: StopMode::Stop1,
        }),
        pins: &[
            PeripheralPin {
                pin: "PA0",
                signal: "SD_B",
                af: Some(10),
            },
            PeripheralPin {
                pin: "PA1",
                signal: "MCLK_B",
                af: Some(10),
            },
            PeripheralPin {
                pin: "PA12",
                signal: "FS_B",
                af: Some(8),
            },
            PeripheralPin {
                pin: "PA2",
                signal: "SCK_B",
                af: Some(8),
            },
            PeripheralPin {
                pin: "PC0",
                signal: "FS_B",
                af: Some(8),
            },
            PeripheralPin {
                pin: "PD11",
                signal: "SD_A",
                af: Some(10),
            },
            PeripheralPin {
                pin: "PD12",
                signal: "FS_A",
                af: Some(10),
            },
            PeripheralPin {
                pin: "PD13",
                signal: "SCK_A",
                af: Some(10),
            },
            PeripheralPin {
                pin: "PE0",
                signal: "MCLK_A",
                af: Some(10),
            },
            PeripheralPin {
                pin: "PE11",
                signal: "SD_B",
                af: Some(10),
            },
            PeripheralPin {
                pin: "PE12",
                signal: "SCK_B",
                af: Some(10),
            },
            PeripheralPin {
                pin: "PE13",
                signal: "FS_B",
                af: Some(10),
            },
            PeripheralPin {
                pin: "PE14",
                signal: "MCLK_B",
                af: Some(10),
            },
            PeripheralPin {
                pin: "PE6",
                signal: "MCLK_B",
                af: Some(10),
            },
        ],
        dma_channels: &[
            PeripheralDmaChannel {
                signal: "B",
                channel: Some("DMA2_CH1"),
                dmamux: None,
                dma: None,
                request: Some(10),
            },
            PeripheralDmaChannel {
                signal: "A",
                channel: Some("DMA2_CH2"),
                dmamux: None,
                dma: None,
                request: Some(10),
            },
            PeripheralDmaChannel {
                signal: "A",
                channel: Some("DMA2_CH4"),
                dmamux: None,
                dma: None,
                request: Some(3),
            },
            PeripheralDmaChannel {
                signal: "B",
                channel: Some("DMA2_CH6"),
                dmamux: None,
                dma: None,
                request: Some(3),
            },
            PeripheralDmaChannel {
                signal: "B",
                channel: Some("DMA2_CH7"),
                dmamux: None,
                dma: None,
                request: Some(0),
            },
        ],
        interrupts: &[PeripheralInterrupt {
            signal: "GLOBAL",
            interrupt: "SAI2",
        }],
    },
    Peripheral {
        name: "SDMMC1",
        address: 1073818624,
        registers: Some(PeripheralRegisters {
            kind: "sdmmc",
            version: "v1",
            block: "SDMMC",
            ir: &sdmmc::REGISTERS,
        }),
        rcc: Some(PeripheralRcc {
            clock: "pclk2",
            enable: Some(PeripheralRccRegister {
                register: "apb2enr",
                field: "sdmmc1en",
            }),
            reset: Some(PeripheralRccRegister {
                register: "apb2rstr",
                field: "sdmmc1rst",
            }),
            mux: None,
            stop_mode: StopMode::Stop1,
        }),
        pins: &[
            PeripheralPin {
                pin: "PB8",
                signal: "D4",
                af: Some(12),
            },
            PeripheralPin {
                pin: "PB9",
                signal: "D5",
                af: Some(12),
            },
            PeripheralPin {
                pin: "PC10",
                signal: "D2",
                af: Some(12),
            },
            PeripheralPin {
                pin: "PC11",
                signal: "D3",
                af: Some(12),
            },
            PeripheralPin {
                pin: "PC12",
                signal: "CK",
                af: Some(12),
            },
            PeripheralPin {
                pin: "PC6",
                signal: "D6",
                af: Some(12),
            },
            PeripheralPin {
                pin: "PC7",
                signal: "D7",
                af: Some(12),
            },
            PeripheralPin {
                pin: "PC8",
                signal: "D0",
                af: Some(12),
            },
            PeripheralPin {
                pin: "PC9",
                signal: "D1",
                af: Some(12),
            },
            PeripheralPin {
                pin: "PD2",
                signal: "CMD",
                af: Some(12),
            },
        ],
        dma_channels: &[
            PeripheralDmaChannel {
                signal: "SDMMC1",
                channel: Some("DMA2_CH3"),
                dmamux: None,
                dma: None,
                request: Some(4),
            },
            PeripheralDmaChannel {
                signal: "RX",
                channel: Some("DMA2_CH3"),
                dmamux: None,
                dma: None,
                request: Some(4),
            },
            PeripheralDmaChannel {
                signal: "TX",
                channel: Some("DMA2_CH3"),
                dmamux: None,
                dma: None,
                request: Some(4),
            },
            PeripheralDmaChannel {
                signal: "SDMMC1",
                channel: Some("DMA2_CH6"),
                dmamux: None,
                dma: None,
                request: Some(4),
            },
            PeripheralDmaChannel {
                signal: "RX",
                channel: Some("DMA2_CH6"),
                dmamux: None,
                dma: None,
                request: Some(4),
            },
            PeripheralDmaChannel {
                signal: "TX",
                channel: Some("DMA2_CH6"),
                dmamux: None,
                dma: None,
                request: Some(4),
            },
        ],
        interrupts: &[PeripheralInterrupt {
            signal: "GLOBAL",
            interrupt: "SDMMC1",
        }],
    },
    Peripheral {
        name: "SDMMC2",
        address: 1073814528,
        registers: Some(PeripheralRegisters {
            kind: "sdmmc",
            version: "v1",
            block: "SDMMC",
            ir: &sdmmc::REGISTERS,
        }),
        rcc: Some(PeripheralRcc {
            clock: "pclk2",
            enable: Some(PeripheralRccRegister {
                register: "apb2enr",
                field: "sdmmc2en",
            }),
            reset: Some(PeripheralRccRegister {
                register: "apb2rstr",
                field: "sdmmc2rst",
            }),
            mux: None,
            stop_mode: StopMode::Stop1,
        }),
        pins: &[
            PeripheralPin {
                pin: "PB14",
                signal: "D0",
                af: Some(10),
            },
            PeripheralPin {
                pin: "PB15",
                signal: "D1",
                af: Some(10),
            },
            PeripheralPin {
                pin: "PB3",
                signal: "D2",
                af: Some(10),
            },
            PeripheralPin {
                pin: "PB4",
                signal: "D3",
                af: Some(10),
            },
            PeripheralPin {
                pin: "PB8",
                signal: "D4",
                af: Some(10),
            },
            PeripheralPin {
                pin: "PB9",
                signal: "D5",
                af: Some(10),
            },
            PeripheralPin {
                pin: "PC6",
                signal: "D6",
                af: Some(10),
            },
            PeripheralPin {
                pin: "PC7",
                signal: "D7",
                af: Some(10),
            },
            PeripheralPin {
                pin: "PD6",
                signal: "CK",
                af: Some(11),
            },
            PeripheralPin {
                pin: "PD7",
                signal: "CMD",
                af: Some(11),
            },
        ],
        dma_channels: &[
            PeripheralDmaChannel {
                signal: "SDMMC2",
                channel: Some("DMA2_CH0"),
                dmamux: None,
                dma: None,
                request: Some(11),
            },
            PeripheralDmaChannel {
                signal: "RX",
                channel: Some("DMA2_CH0"),
                dmamux: None,
                dma: None,
                request: Some(11),
            },
            PeripheralDmaChannel {
                signal: "TX",
                channel: Some("DMA2_CH0"),
                dmamux: None,
                dma: None,
                request: Some(11),
            },
            PeripheralDmaChannel {
                signal: "SDMMC2",
                channel: Some("DMA2_CH5"),
                dmamux: None,
                dma: None,
                request: Some(11),
            },
            PeripheralDmaChannel {
                signal: "RX",
                channel: Some("DMA2_CH5"),
                dmamux: None,
                dma: None,
                request: Some(11),
            },
            PeripheralDmaChannel {
                signal: "TX",
                channel: Some("DMA2_CH5"),
                dmamux: None,
                dma: None,
                request: Some(11),
            },
        ],
        interrupts: &[PeripheralInterrupt {
            signal: "GLOBAL",
            interrupt: "SDMMC2",
        }],
    },
    Peripheral {
        name: "SPDIFRX",
        address: 1073758208,
        registers: Some(PeripheralRegisters {
            kind: "spdifrx",
            version: "v1",
            block: "SPDIFRX",
            ir: &spdifrx::REGISTERS,
        }),
        rcc: Some(PeripheralRcc {
            clock: "pclk1",
            enable: Some(PeripheralRccRegister {
                register: "apb1enr",
                field: "spdifrxen",
            }),
            reset: Some(PeripheralRccRegister {
                register: "apb1rstr",
                field: "spdifrxrst",
            }),
            mux: None,
            stop_mode: StopMode::Stop1,
        }),
        pins: &[
            PeripheralPin {
                pin: "PC4",
                signal: "IN2",
                af: Some(8),
            },
            PeripheralPin {
                pin: "PC5",
                signal: "IN3",
                af: Some(8),
            },
            PeripheralPin {
                pin: "PD7",
                signal: "IN0",
                af: Some(8),
            },
            PeripheralPin {
                pin: "PD8",
                signal: "IN1",
                af: Some(8),
            },
        ],
        dma_channels: &[],
        interrupts: &[PeripheralInterrupt {
            signal: "GLOBAL",
            interrupt: "SPDIF_RX",
        }],
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
                pin: "PA15",
                signal: "I2S_WS",
                af: Some(5),
            },
            PeripheralPin {
                pin: "PA15",
                signal: "NSS",
                af: Some(5),
            },
            PeripheralPin {
                pin: "PA4",
                signal: "I2S_WS",
                af: Some(5),
            },
            PeripheralPin {
                pin: "PA4",
                signal: "NSS",
                af: Some(5),
            },
            PeripheralPin {
                pin: "PA5",
                signal: "I2S_CK",
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
                signal: "I2S_SD",
                af: Some(5),
            },
            PeripheralPin {
                pin: "PA7",
                signal: "MOSI",
                af: Some(5),
            },
            PeripheralPin {
                pin: "PB3",
                signal: "I2S_CK",
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
                signal: "I2S_SD",
                af: Some(5),
            },
            PeripheralPin {
                pin: "PB5",
                signal: "MOSI",
                af: Some(5),
            },
            PeripheralPin {
                pin: "PC4",
                signal: "I2S_MCK",
                af: Some(5),
            },
            PeripheralPin {
                pin: "PD7",
                signal: "I2S_SD",
                af: Some(5),
            },
            PeripheralPin {
                pin: "PD7",
                signal: "MOSI",
                af: Some(5),
            },
        ],
        dma_channels: &[
            PeripheralDmaChannel {
                signal: "RX",
                channel: Some("DMA2_CH0"),
                dmamux: None,
                dma: None,
                request: Some(3),
            },
            PeripheralDmaChannel {
                signal: "RX",
                channel: Some("DMA2_CH2"),
                dmamux: None,
                dma: None,
                request: Some(3),
            },
            PeripheralDmaChannel {
                signal: "TX",
                channel: Some("DMA2_CH3"),
                dmamux: None,
                dma: None,
                request: Some(3),
            },
            PeripheralDmaChannel {
                signal: "TX",
                channel: Some("DMA2_CH5"),
                dmamux: None,
                dma: None,
                request: Some(3),
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
                register: "apb1enr",
                field: "spi2en",
            }),
            reset: Some(PeripheralRccRegister {
                register: "apb1rstr",
                field: "spi2rst",
            }),
            mux: None,
            stop_mode: StopMode::Stop1,
        }),
        pins: &[
            PeripheralPin {
                pin: "PA11",
                signal: "I2S_WS",
                af: Some(5),
            },
            PeripheralPin {
                pin: "PA11",
                signal: "NSS",
                af: Some(5),
            },
            PeripheralPin {
                pin: "PA12",
                signal: "I2S_CK",
                af: Some(5),
            },
            PeripheralPin {
                pin: "PA12",
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
                signal: "SCK",
                af: Some(5),
            },
            PeripheralPin {
                pin: "PB10",
                signal: "I2S_CK",
                af: Some(5),
            },
            PeripheralPin {
                pin: "PB10",
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
            PeripheralPin {
                pin: "PB13",
                signal: "I2S_CK",
                af: Some(5),
            },
            PeripheralPin {
                pin: "PB13",
                signal: "SCK",
                af: Some(5),
            },
            PeripheralPin {
                pin: "PB14",
                signal: "MISO",
                af: Some(5),
            },
            PeripheralPin {
                pin: "PB15",
                signal: "I2S_SD",
                af: Some(5),
            },
            PeripheralPin {
                pin: "PB15",
                signal: "MOSI",
                af: Some(5),
            },
            PeripheralPin {
                pin: "PB4",
                signal: "I2S_WS",
                af: Some(7),
            },
            PeripheralPin {
                pin: "PB4",
                signal: "NSS",
                af: Some(7),
            },
            PeripheralPin {
                pin: "PB9",
                signal: "I2S_WS",
                af: Some(5),
            },
            PeripheralPin {
                pin: "PB9",
                signal: "NSS",
                af: Some(5),
            },
            PeripheralPin {
                pin: "PC1",
                signal: "I2S_SD",
                af: Some(5),
            },
            PeripheralPin {
                pin: "PC1",
                signal: "MOSI",
                af: Some(5),
            },
            PeripheralPin {
                pin: "PC2",
                signal: "MISO",
                af: Some(5),
            },
            PeripheralPin {
                pin: "PC3",
                signal: "I2S_SD",
                af: Some(5),
            },
            PeripheralPin {
                pin: "PC3",
                signal: "MOSI",
                af: Some(5),
            },
            PeripheralPin {
                pin: "PC6",
                signal: "I2S_MCK",
                af: Some(5),
            },
            PeripheralPin {
                pin: "PD3",
                signal: "I2S_CK",
                af: Some(5),
            },
            PeripheralPin {
                pin: "PD3",
                signal: "SCK",
                af: Some(5),
            },
        ],
        dma_channels: &[
            PeripheralDmaChannel {
                signal: "RX",
                channel: Some("DMA1_CH1"),
                dmamux: None,
                dma: None,
                request: Some(9),
            },
            PeripheralDmaChannel {
                signal: "RX",
                channel: Some("DMA1_CH3"),
                dmamux: None,
                dma: None,
                request: Some(0),
            },
            PeripheralDmaChannel {
                signal: "TX",
                channel: Some("DMA1_CH4"),
                dmamux: None,
                dma: None,
                request: Some(0),
            },
            PeripheralDmaChannel {
                signal: "TX",
                channel: Some("DMA1_CH6"),
                dmamux: None,
                dma: None,
                request: Some(9),
            },
        ],
        interrupts: &[PeripheralInterrupt {
            signal: "GLOBAL",
            interrupt: "SPI2",
        }],
    },
    Peripheral {
        name: "SPI3",
        address: 1073757184,
        registers: Some(PeripheralRegisters {
            kind: "spi",
            version: "v2",
            block: "SPI",
            ir: &spi::REGISTERS,
        }),
        rcc: Some(PeripheralRcc {
            clock: "pclk1",
            enable: Some(PeripheralRccRegister {
                register: "apb1enr",
                field: "spi3en",
            }),
            reset: Some(PeripheralRccRegister {
                register: "apb1rstr",
                field: "spi3rst",
            }),
            mux: None,
            stop_mode: StopMode::Stop1,
        }),
        pins: &[
            PeripheralPin {
                pin: "PA15",
                signal: "I2S_WS",
                af: Some(6),
            },
            PeripheralPin {
                pin: "PA15",
                signal: "NSS",
                af: Some(6),
            },
            PeripheralPin {
                pin: "PA4",
                signal: "I2S_WS",
                af: Some(6),
            },
            PeripheralPin {
                pin: "PA4",
                signal: "NSS",
                af: Some(6),
            },
            PeripheralPin {
                pin: "PB2",
                signal: "I2S_SD",
                af: Some(7),
            },
            PeripheralPin {
                pin: "PB2",
                signal: "MOSI",
                af: Some(7),
            },
            PeripheralPin {
                pin: "PB3",
                signal: "I2S_CK",
                af: Some(6),
            },
            PeripheralPin {
                pin: "PB3",
                signal: "SCK",
                af: Some(6),
            },
            PeripheralPin {
                pin: "PB4",
                signal: "MISO",
                af: Some(6),
            },
            PeripheralPin {
                pin: "PB5",
                signal: "I2S_SD",
                af: Some(6),
            },
            PeripheralPin {
                pin: "PB5",
                signal: "MOSI",
                af: Some(6),
            },
            PeripheralPin {
                pin: "PC10",
                signal: "I2S_CK",
                af: Some(6),
            },
            PeripheralPin {
                pin: "PC10",
                signal: "SCK",
                af: Some(6),
            },
            PeripheralPin {
                pin: "PC11",
                signal: "MISO",
                af: Some(6),
            },
            PeripheralPin {
                pin: "PC12",
                signal: "I2S_SD",
                af: Some(6),
            },
            PeripheralPin {
                pin: "PC12",
                signal: "MOSI",
                af: Some(6),
            },
            PeripheralPin {
                pin: "PC7",
                signal: "I2S_MCK",
                af: Some(6),
            },
            PeripheralPin {
                pin: "PD6",
                signal: "I2S_SD",
                af: Some(5),
            },
            PeripheralPin {
                pin: "PD6",
                signal: "MOSI",
                af: Some(5),
            },
        ],
        dma_channels: &[
            PeripheralDmaChannel {
                signal: "RX",
                channel: Some("DMA1_CH0"),
                dmamux: None,
                dma: None,
                request: Some(0),
            },
            PeripheralDmaChannel {
                signal: "RX",
                channel: Some("DMA1_CH2"),
                dmamux: None,
                dma: None,
                request: Some(0),
            },
            PeripheralDmaChannel {
                signal: "TX",
                channel: Some("DMA1_CH5"),
                dmamux: None,
                dma: None,
                request: Some(0),
            },
            PeripheralDmaChannel {
                signal: "TX",
                channel: Some("DMA1_CH7"),
                dmamux: None,
                dma: None,
                request: Some(0),
            },
        ],
        interrupts: &[PeripheralInterrupt {
            signal: "GLOBAL",
            interrupt: "SPI3",
        }],
    },
    Peripheral {
        name: "SPI4",
        address: 1073820672,
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
                field: "spi4en",
            }),
            reset: Some(PeripheralRccRegister {
                register: "apb2rstr",
                field: "spi4rst",
            }),
            mux: None,
            stop_mode: StopMode::Stop1,
        }),
        pins: &[
            PeripheralPin {
                pin: "PE11",
                signal: "NSS",
                af: Some(5),
            },
            PeripheralPin {
                pin: "PE12",
                signal: "SCK",
                af: Some(5),
            },
            PeripheralPin {
                pin: "PE13",
                signal: "MISO",
                af: Some(5),
            },
            PeripheralPin {
                pin: "PE14",
                signal: "MOSI",
                af: Some(5),
            },
            PeripheralPin {
                pin: "PE2",
                signal: "SCK",
                af: Some(5),
            },
            PeripheralPin {
                pin: "PE4",
                signal: "NSS",
                af: Some(5),
            },
            PeripheralPin {
                pin: "PE5",
                signal: "MISO",
                af: Some(5),
            },
            PeripheralPin {
                pin: "PE6",
                signal: "MOSI",
                af: Some(5),
            },
        ],
        dma_channels: &[
            PeripheralDmaChannel {
                signal: "RX",
                channel: Some("DMA2_CH0"),
                dmamux: None,
                dma: None,
                request: Some(4),
            },
            PeripheralDmaChannel {
                signal: "TX",
                channel: Some("DMA2_CH1"),
                dmamux: None,
                dma: None,
                request: Some(4),
            },
            PeripheralDmaChannel {
                signal: "TX",
                channel: Some("DMA2_CH2"),
                dmamux: None,
                dma: None,
                request: Some(9),
            },
            PeripheralDmaChannel {
                signal: "RX",
                channel: Some("DMA2_CH3"),
                dmamux: None,
                dma: None,
                request: Some(5),
            },
            PeripheralDmaChannel {
                signal: "TX",
                channel: Some("DMA2_CH4"),
                dmamux: None,
                dma: None,
                request: Some(5),
            },
        ],
        interrupts: &[PeripheralInterrupt {
            signal: "GLOBAL",
            interrupt: "SPI4",
        }],
    },
    Peripheral {
        name: "SPI6",
        address: 1073828864,
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
                field: "spi6en",
            }),
            reset: Some(PeripheralRccRegister {
                register: "apb2rstr",
                field: "spi6rst",
            }),
            mux: None,
            stop_mode: StopMode::Stop1,
        }),
        pins: &[
            PeripheralPin {
                pin: "PA15",
                signal: "NSS",
                af: Some(7),
            },
            PeripheralPin {
                pin: "PA4",
                signal: "NSS",
                af: Some(8),
            },
            PeripheralPin {
                pin: "PA5",
                signal: "SCK",
                af: Some(8),
            },
            PeripheralPin {
                pin: "PA6",
                signal: "MISO",
                af: Some(8),
            },
            PeripheralPin {
                pin: "PA7",
                signal: "MOSI",
                af: Some(8),
            },
            PeripheralPin {
                pin: "PB3",
                signal: "SCK",
                af: Some(8),
            },
            PeripheralPin {
                pin: "PB4",
                signal: "MISO",
                af: Some(8),
            },
            PeripheralPin {
                pin: "PB5",
                signal: "MOSI",
                af: Some(8),
            },
        ],
        dma_channels: &[
            PeripheralDmaChannel {
                signal: "TX",
                channel: Some("DMA2_CH5"),
                dmamux: None,
                dma: None,
                request: Some(1),
            },
            PeripheralDmaChannel {
                signal: "RX",
                channel: Some("DMA2_CH6"),
                dmamux: None,
                dma: None,
                request: Some(1),
            },
        ],
        interrupts: &[PeripheralInterrupt {
            signal: "GLOBAL",
            interrupt: "SPI6",
        }],
    },
    Peripheral {
        name: "SYSCFG",
        address: 1073821696,
        registers: Some(PeripheralRegisters {
            kind: "syscfg",
            version: "f7",
            block: "SYSCFG",
            ir: &syscfg::REGISTERS,
        }),
        rcc: Some(PeripheralRcc {
            clock: "pclk2",
            enable: Some(PeripheralRccRegister {
                register: "apb2enr",
                field: "syscfgen",
            }),
            reset: Some(PeripheralRccRegister {
                register: "apb2rstr",
                field: "syscfgrst",
            }),
            mux: None,
            stop_mode: StopMode::Stop1,
        }),
        pins: &[],
        dma_channels: &[],
        interrupts: &[],
    },
    Peripheral {
        name: "TIM1",
        address: 1073807360,
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
                af: Some(1),
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
                pin: "PB0",
                signal: "CH2N",
                af: Some(1),
            },
            PeripheralPin {
                pin: "PB1",
                signal: "CH3N",
                af: Some(1),
            },
            PeripheralPin {
                pin: "PB12",
                signal: "BKIN",
                af: Some(1),
            },
            PeripheralPin {
                pin: "PB13",
                signal: "CH1N",
                af: Some(1),
            },
            PeripheralPin {
                pin: "PB14",
                signal: "CH2N",
                af: Some(1),
            },
            PeripheralPin {
                pin: "PB15",
                signal: "CH3N",
                af: Some(1),
            },
            PeripheralPin {
                pin: "PE10",
                signal: "CH2N",
                af: Some(1),
            },
            PeripheralPin {
                pin: "PE11",
                signal: "CH2",
                af: Some(1),
            },
            PeripheralPin {
                pin: "PE12",
                signal: "CH3N",
                af: Some(1),
            },
            PeripheralPin {
                pin: "PE13",
                signal: "CH3",
                af: Some(1),
            },
            PeripheralPin {
                pin: "PE14",
                signal: "CH4",
                af: Some(1),
            },
            PeripheralPin {
                pin: "PE15",
                signal: "BKIN",
                af: Some(1),
            },
            PeripheralPin {
                pin: "PE6",
                signal: "BKIN2",
                af: Some(1),
            },
            PeripheralPin {
                pin: "PE7",
                signal: "ETR",
                af: Some(1),
            },
            PeripheralPin {
                pin: "PE8",
                signal: "CH1N",
                af: Some(1),
            },
            PeripheralPin {
                pin: "PE9",
                signal: "CH1",
                af: Some(1),
            },
        ],
        dma_channels: &[
            PeripheralDmaChannel {
                signal: "TRIG",
                channel: Some("DMA2_CH0"),
                dmamux: None,
                dma: None,
                request: Some(6),
            },
            PeripheralDmaChannel {
                signal: "CH1",
                channel: Some("DMA2_CH1"),
                dmamux: None,
                dma: None,
                request: Some(6),
            },
            PeripheralDmaChannel {
                signal: "CH2",
                channel: Some("DMA2_CH2"),
                dmamux: None,
                dma: None,
                request: Some(6),
            },
            PeripheralDmaChannel {
                signal: "CH1",
                channel: Some("DMA2_CH3"),
                dmamux: None,
                dma: None,
                request: Some(6),
            },
            PeripheralDmaChannel {
                signal: "CH4",
                channel: Some("DMA2_CH4"),
                dmamux: None,
                dma: None,
                request: Some(6),
            },
            PeripheralDmaChannel {
                signal: "TRIG",
                channel: Some("DMA2_CH4"),
                dmamux: None,
                dma: None,
                request: Some(6),
            },
            PeripheralDmaChannel {
                signal: "COM",
                channel: Some("DMA2_CH4"),
                dmamux: None,
                dma: None,
                request: Some(6),
            },
            PeripheralDmaChannel {
                signal: "UP",
                channel: Some("DMA2_CH5"),
                dmamux: None,
                dma: None,
                request: Some(6),
            },
            PeripheralDmaChannel {
                signal: "CH1",
                channel: Some("DMA2_CH6"),
                dmamux: None,
                dma: None,
                request: Some(0),
            },
            PeripheralDmaChannel {
                signal: "CH2",
                channel: Some("DMA2_CH6"),
                dmamux: None,
                dma: None,
                request: Some(0),
            },
            PeripheralDmaChannel {
                signal: "CH3",
                channel: Some("DMA2_CH6"),
                dmamux: None,
                dma: None,
                request: Some(0),
            },
            PeripheralDmaChannel {
                signal: "CH3",
                channel: Some("DMA2_CH6"),
                dmamux: None,
                dma: None,
                request: Some(6),
            },
        ],
        interrupts: &[
            PeripheralInterrupt {
                signal: "BRK",
                interrupt: "TIM1_BRK_TIM9",
            },
            PeripheralInterrupt {
                signal: "CC",
                interrupt: "TIM1_CC",
            },
            PeripheralInterrupt {
                signal: "COM",
                interrupt: "TIM1_TRG_COM_TIM11",
            },
            PeripheralInterrupt {
                signal: "TRG",
                interrupt: "TIM1_TRG_COM_TIM11",
            },
            PeripheralInterrupt {
                signal: "UP",
                interrupt: "TIM1_UP_TIM10",
            },
        ],
    },
    Peripheral {
        name: "TIM10",
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
                field: "tim10en",
            }),
            reset: Some(PeripheralRccRegister {
                register: "apb2rstr",
                field: "tim10rst",
            }),
            mux: None,
            stop_mode: StopMode::Stop1,
        }),
        pins: &[PeripheralPin {
            pin: "PB8",
            signal: "CH1",
            af: Some(3),
        }],
        dma_channels: &[],
        interrupts: &[
            PeripheralInterrupt {
                signal: "BRK",
                interrupt: "TIM1_UP_TIM10",
            },
            PeripheralInterrupt {
                signal: "CC",
                interrupt: "TIM1_UP_TIM10",
            },
            PeripheralInterrupt {
                signal: "COM",
                interrupt: "TIM1_UP_TIM10",
            },
            PeripheralInterrupt {
                signal: "TRG",
                interrupt: "TIM1_UP_TIM10",
            },
            PeripheralInterrupt {
                signal: "UP",
                interrupt: "TIM1_UP_TIM10",
            },
        ],
    },
    Peripheral {
        name: "TIM11",
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
                field: "tim11en",
            }),
            reset: Some(PeripheralRccRegister {
                register: "apb2rstr",
                field: "tim11rst",
            }),
            mux: None,
            stop_mode: StopMode::Stop1,
        }),
        pins: &[PeripheralPin {
            pin: "PB9",
            signal: "CH1",
            af: Some(3),
        }],
        dma_channels: &[],
        interrupts: &[
            PeripheralInterrupt {
                signal: "BRK",
                interrupt: "TIM1_TRG_COM_TIM11",
            },
            PeripheralInterrupt {
                signal: "CC",
                interrupt: "TIM1_TRG_COM_TIM11",
            },
            PeripheralInterrupt {
                signal: "COM",
                interrupt: "TIM1_TRG_COM_TIM11",
            },
            PeripheralInterrupt {
                signal: "TRG",
                interrupt: "TIM1_TRG_COM_TIM11",
            },
            PeripheralInterrupt {
                signal: "UP",
                interrupt: "TIM1_TRG_COM_TIM11",
            },
        ],
    },
    Peripheral {
        name: "TIM12",
        address: 1073747968,
        registers: Some(PeripheralRegisters {
            kind: "timer",
            version: "v1",
            block: "TIM_GP16",
            ir: &timer::REGISTERS,
        }),
        rcc: Some(PeripheralRcc {
            clock: "pclk1_tim",
            enable: Some(PeripheralRccRegister {
                register: "apb1enr",
                field: "tim12en",
            }),
            reset: Some(PeripheralRccRegister {
                register: "apb1rstr",
                field: "tim12rst",
            }),
            mux: None,
            stop_mode: StopMode::Stop1,
        }),
        pins: &[
            PeripheralPin {
                pin: "PB14",
                signal: "CH1",
                af: Some(9),
            },
            PeripheralPin {
                pin: "PB15",
                signal: "CH2",
                af: Some(9),
            },
        ],
        dma_channels: &[],
        interrupts: &[
            PeripheralInterrupt {
                signal: "BRK",
                interrupt: "TIM8_BRK_TIM12",
            },
            PeripheralInterrupt {
                signal: "CC",
                interrupt: "TIM8_BRK_TIM12",
            },
            PeripheralInterrupt {
                signal: "COM",
                interrupt: "TIM8_BRK_TIM12",
            },
            PeripheralInterrupt {
                signal: "TRG",
                interrupt: "TIM8_BRK_TIM12",
            },
            PeripheralInterrupt {
                signal: "UP",
                interrupt: "TIM8_BRK_TIM12",
            },
        ],
    },
    Peripheral {
        name: "TIM13",
        address: 1073748992,
        registers: Some(PeripheralRegisters {
            kind: "timer",
            version: "v1",
            block: "TIM_GP16",
            ir: &timer::REGISTERS,
        }),
        rcc: Some(PeripheralRcc {
            clock: "pclk1_tim",
            enable: Some(PeripheralRccRegister {
                register: "apb1enr",
                field: "tim13en",
            }),
            reset: Some(PeripheralRccRegister {
                register: "apb1rstr",
                field: "tim13rst",
            }),
            mux: None,
            stop_mode: StopMode::Stop1,
        }),
        pins: &[PeripheralPin {
            pin: "PA6",
            signal: "CH1",
            af: Some(9),
        }],
        dma_channels: &[],
        interrupts: &[
            PeripheralInterrupt {
                signal: "BRK",
                interrupt: "TIM8_UP_TIM13",
            },
            PeripheralInterrupt {
                signal: "CC",
                interrupt: "TIM8_UP_TIM13",
            },
            PeripheralInterrupt {
                signal: "COM",
                interrupt: "TIM8_UP_TIM13",
            },
            PeripheralInterrupt {
                signal: "TRG",
                interrupt: "TIM8_UP_TIM13",
            },
            PeripheralInterrupt {
                signal: "UP",
                interrupt: "TIM8_UP_TIM13",
            },
        ],
    },
    Peripheral {
        name: "TIM14",
        address: 1073750016,
        registers: Some(PeripheralRegisters {
            kind: "timer",
            version: "v1",
            block: "TIM_GP16",
            ir: &timer::REGISTERS,
        }),
        rcc: Some(PeripheralRcc {
            clock: "pclk1_tim",
            enable: Some(PeripheralRccRegister {
                register: "apb1enr",
                field: "tim14en",
            }),
            reset: Some(PeripheralRccRegister {
                register: "apb1rstr",
                field: "tim14rst",
            }),
            mux: None,
            stop_mode: StopMode::Stop1,
        }),
        pins: &[PeripheralPin {
            pin: "PA7",
            signal: "CH1",
            af: Some(9),
        }],
        dma_channels: &[],
        interrupts: &[
            PeripheralInterrupt {
                signal: "BRK",
                interrupt: "TIM8_TRG_COM_TIM14",
            },
            PeripheralInterrupt {
                signal: "CC",
                interrupt: "TIM8_TRG_COM_TIM14",
            },
            PeripheralInterrupt {
                signal: "COM",
                interrupt: "TIM8_TRG_COM_TIM14",
            },
            PeripheralInterrupt {
                signal: "TRG",
                interrupt: "TIM8_TRG_COM_TIM14",
            },
            PeripheralInterrupt {
                signal: "UP",
                interrupt: "TIM8_TRG_COM_TIM14",
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
                register: "apb1enr",
                field: "tim2en",
            }),
            reset: Some(PeripheralRccRegister {
                register: "apb1rstr",
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
                af: Some(1),
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
                af: Some(1),
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
                af: Some(1),
            },
            PeripheralPin {
                pin: "PB10",
                signal: "CH3",
                af: Some(1),
            },
            PeripheralPin {
                pin: "PB11",
                signal: "CH4",
                af: Some(1),
            },
            PeripheralPin {
                pin: "PB3",
                signal: "CH2",
                af: Some(1),
            },
        ],
        dma_channels: &[
            PeripheralDmaChannel {
                signal: "UP",
                channel: Some("DMA1_CH1"),
                dmamux: None,
                dma: None,
                request: Some(3),
            },
            PeripheralDmaChannel {
                signal: "CH3",
                channel: Some("DMA1_CH1"),
                dmamux: None,
                dma: None,
                request: Some(3),
            },
            PeripheralDmaChannel {
                signal: "CH1",
                channel: Some("DMA1_CH5"),
                dmamux: None,
                dma: None,
                request: Some(3),
            },
            PeripheralDmaChannel {
                signal: "CH2",
                channel: Some("DMA1_CH6"),
                dmamux: None,
                dma: None,
                request: Some(3),
            },
            PeripheralDmaChannel {
                signal: "CH4",
                channel: Some("DMA1_CH6"),
                dmamux: None,
                dma: None,
                request: Some(3),
            },
            PeripheralDmaChannel {
                signal: "UP",
                channel: Some("DMA1_CH7"),
                dmamux: None,
                dma: None,
                request: Some(3),
            },
            PeripheralDmaChannel {
                signal: "CH4",
                channel: Some("DMA1_CH7"),
                dmamux: None,
                dma: None,
                request: Some(3),
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
        name: "TIM3",
        address: 1073742848,
        registers: Some(PeripheralRegisters {
            kind: "timer",
            version: "v1",
            block: "TIM_GP16",
            ir: &timer::REGISTERS,
        }),
        rcc: Some(PeripheralRcc {
            clock: "pclk1_tim",
            enable: Some(PeripheralRccRegister {
                register: "apb1enr",
                field: "tim3en",
            }),
            reset: Some(PeripheralRccRegister {
                register: "apb1rstr",
                field: "tim3rst",
            }),
            mux: None,
            stop_mode: StopMode::Stop1,
        }),
        pins: &[
            PeripheralPin {
                pin: "PA6",
                signal: "CH1",
                af: Some(2),
            },
            PeripheralPin {
                pin: "PA7",
                signal: "CH2",
                af: Some(2),
            },
            PeripheralPin {
                pin: "PB0",
                signal: "CH3",
                af: Some(2),
            },
            PeripheralPin {
                pin: "PB1",
                signal: "CH4",
                af: Some(2),
            },
            PeripheralPin {
                pin: "PB4",
                signal: "CH1",
                af: Some(2),
            },
            PeripheralPin {
                pin: "PB5",
                signal: "CH2",
                af: Some(2),
            },
            PeripheralPin {
                pin: "PC6",
                signal: "CH1",
                af: Some(2),
            },
            PeripheralPin {
                pin: "PC7",
                signal: "CH2",
                af: Some(2),
            },
            PeripheralPin {
                pin: "PC8",
                signal: "CH3",
                af: Some(2),
            },
            PeripheralPin {
                pin: "PC9",
                signal: "CH4",
                af: Some(2),
            },
            PeripheralPin {
                pin: "PD2",
                signal: "ETR",
                af: Some(2),
            },
        ],
        dma_channels: &[
            PeripheralDmaChannel {
                signal: "CH4",
                channel: Some("DMA1_CH2"),
                dmamux: None,
                dma: None,
                request: Some(5),
            },
            PeripheralDmaChannel {
                signal: "UP",
                channel: Some("DMA1_CH2"),
                dmamux: None,
                dma: None,
                request: Some(5),
            },
            PeripheralDmaChannel {
                signal: "CH1",
                channel: Some("DMA1_CH4"),
                dmamux: None,
                dma: None,
                request: Some(5),
            },
            PeripheralDmaChannel {
                signal: "TRIG",
                channel: Some("DMA1_CH4"),
                dmamux: None,
                dma: None,
                request: Some(5),
            },
            PeripheralDmaChannel {
                signal: "CH2",
                channel: Some("DMA1_CH5"),
                dmamux: None,
                dma: None,
                request: Some(5),
            },
            PeripheralDmaChannel {
                signal: "CH3",
                channel: Some("DMA1_CH7"),
                dmamux: None,
                dma: None,
                request: Some(5),
            },
        ],
        interrupts: &[
            PeripheralInterrupt {
                signal: "BRK",
                interrupt: "TIM3",
            },
            PeripheralInterrupt {
                signal: "CC",
                interrupt: "TIM3",
            },
            PeripheralInterrupt {
                signal: "COM",
                interrupt: "TIM3",
            },
            PeripheralInterrupt {
                signal: "TRG",
                interrupt: "TIM3",
            },
            PeripheralInterrupt {
                signal: "UP",
                interrupt: "TIM3",
            },
        ],
    },
    Peripheral {
        name: "TIM4",
        address: 1073743872,
        registers: Some(PeripheralRegisters {
            kind: "timer",
            version: "v1",
            block: "TIM_GP16",
            ir: &timer::REGISTERS,
        }),
        rcc: Some(PeripheralRcc {
            clock: "pclk1_tim",
            enable: Some(PeripheralRccRegister {
                register: "apb1enr",
                field: "tim4en",
            }),
            reset: Some(PeripheralRccRegister {
                register: "apb1rstr",
                field: "tim4rst",
            }),
            mux: None,
            stop_mode: StopMode::Stop1,
        }),
        pins: &[
            PeripheralPin {
                pin: "PB6",
                signal: "CH1",
                af: Some(2),
            },
            PeripheralPin {
                pin: "PB7",
                signal: "CH2",
                af: Some(2),
            },
            PeripheralPin {
                pin: "PB8",
                signal: "CH3",
                af: Some(2),
            },
            PeripheralPin {
                pin: "PB9",
                signal: "CH4",
                af: Some(2),
            },
            PeripheralPin {
                pin: "PD12",
                signal: "CH1",
                af: Some(2),
            },
            PeripheralPin {
                pin: "PD13",
                signal: "CH2",
                af: Some(2),
            },
            PeripheralPin {
                pin: "PD14",
                signal: "CH3",
                af: Some(2),
            },
            PeripheralPin {
                pin: "PD15",
                signal: "CH4",
                af: Some(2),
            },
            PeripheralPin {
                pin: "PE0",
                signal: "ETR",
                af: Some(2),
            },
        ],
        dma_channels: &[
            PeripheralDmaChannel {
                signal: "CH1",
                channel: Some("DMA1_CH0"),
                dmamux: None,
                dma: None,
                request: Some(2),
            },
            PeripheralDmaChannel {
                signal: "CH2",
                channel: Some("DMA1_CH3"),
                dmamux: None,
                dma: None,
                request: Some(2),
            },
            PeripheralDmaChannel {
                signal: "UP",
                channel: Some("DMA1_CH6"),
                dmamux: None,
                dma: None,
                request: Some(2),
            },
            PeripheralDmaChannel {
                signal: "CH3",
                channel: Some("DMA1_CH7"),
                dmamux: None,
                dma: None,
                request: Some(2),
            },
        ],
        interrupts: &[
            PeripheralInterrupt {
                signal: "BRK",
                interrupt: "TIM4",
            },
            PeripheralInterrupt {
                signal: "CC",
                interrupt: "TIM4",
            },
            PeripheralInterrupt {
                signal: "COM",
                interrupt: "TIM4",
            },
            PeripheralInterrupt {
                signal: "TRG",
                interrupt: "TIM4",
            },
            PeripheralInterrupt {
                signal: "UP",
                interrupt: "TIM4",
            },
        ],
    },
    Peripheral {
        name: "TIM5",
        address: 1073744896,
        registers: Some(PeripheralRegisters {
            kind: "timer",
            version: "v1",
            block: "TIM_GP32",
            ir: &timer::REGISTERS,
        }),
        rcc: Some(PeripheralRcc {
            clock: "pclk1_tim",
            enable: Some(PeripheralRccRegister {
                register: "apb1enr",
                field: "tim5en",
            }),
            reset: Some(PeripheralRccRegister {
                register: "apb1rstr",
                field: "tim5rst",
            }),
            mux: None,
            stop_mode: StopMode::Stop1,
        }),
        pins: &[
            PeripheralPin {
                pin: "PA0",
                signal: "CH1",
                af: Some(2),
            },
            PeripheralPin {
                pin: "PA1",
                signal: "CH2",
                af: Some(2),
            },
            PeripheralPin {
                pin: "PA2",
                signal: "CH3",
                af: Some(2),
            },
            PeripheralPin {
                pin: "PA3",
                signal: "CH4",
                af: Some(2),
            },
        ],
        dma_channels: &[
            PeripheralDmaChannel {
                signal: "CH3",
                channel: Some("DMA1_CH0"),
                dmamux: None,
                dma: None,
                request: Some(6),
            },
            PeripheralDmaChannel {
                signal: "UP",
                channel: Some("DMA1_CH0"),
                dmamux: None,
                dma: None,
                request: Some(6),
            },
            PeripheralDmaChannel {
                signal: "CH4",
                channel: Some("DMA1_CH1"),
                dmamux: None,
                dma: None,
                request: Some(6),
            },
            PeripheralDmaChannel {
                signal: "TRIG",
                channel: Some("DMA1_CH1"),
                dmamux: None,
                dma: None,
                request: Some(6),
            },
            PeripheralDmaChannel {
                signal: "CH1",
                channel: Some("DMA1_CH2"),
                dmamux: None,
                dma: None,
                request: Some(6),
            },
            PeripheralDmaChannel {
                signal: "CH4",
                channel: Some("DMA1_CH3"),
                dmamux: None,
                dma: None,
                request: Some(6),
            },
            PeripheralDmaChannel {
                signal: "TRIG",
                channel: Some("DMA1_CH3"),
                dmamux: None,
                dma: None,
                request: Some(6),
            },
            PeripheralDmaChannel {
                signal: "CH2",
                channel: Some("DMA1_CH4"),
                dmamux: None,
                dma: None,
                request: Some(6),
            },
            PeripheralDmaChannel {
                signal: "UP",
                channel: Some("DMA1_CH6"),
                dmamux: None,
                dma: None,
                request: Some(6),
            },
        ],
        interrupts: &[
            PeripheralInterrupt {
                signal: "BRK",
                interrupt: "TIM5",
            },
            PeripheralInterrupt {
                signal: "CC",
                interrupt: "TIM5",
            },
            PeripheralInterrupt {
                signal: "COM",
                interrupt: "TIM5",
            },
            PeripheralInterrupt {
                signal: "TRG",
                interrupt: "TIM5",
            },
            PeripheralInterrupt {
                signal: "UP",
                interrupt: "TIM5",
            },
        ],
    },
    Peripheral {
        name: "TIM6",
        address: 1073745920,
        registers: Some(PeripheralRegisters {
            kind: "timer",
            version: "v1",
            block: "TIM_BASIC",
            ir: &timer::REGISTERS,
        }),
        rcc: Some(PeripheralRcc {
            clock: "pclk1_tim",
            enable: Some(PeripheralRccRegister {
                register: "apb1enr",
                field: "tim6en",
            }),
            reset: Some(PeripheralRccRegister {
                register: "apb1rstr",
                field: "tim6rst",
            }),
            mux: None,
            stop_mode: StopMode::Stop1,
        }),
        pins: &[],
        dma_channels: &[PeripheralDmaChannel {
            signal: "UP",
            channel: Some("DMA1_CH1"),
            dmamux: None,
            dma: None,
            request: Some(7),
        }],
        interrupts: &[
            PeripheralInterrupt {
                signal: "BRK",
                interrupt: "TIM6_DAC",
            },
            PeripheralInterrupt {
                signal: "CC",
                interrupt: "TIM6_DAC",
            },
            PeripheralInterrupt {
                signal: "COM",
                interrupt: "TIM6_DAC",
            },
            PeripheralInterrupt {
                signal: "TRG",
                interrupt: "TIM6_DAC",
            },
            PeripheralInterrupt {
                signal: "UP",
                interrupt: "TIM6_DAC",
            },
        ],
    },
    Peripheral {
        name: "TIM7",
        address: 1073746944,
        registers: Some(PeripheralRegisters {
            kind: "timer",
            version: "v1",
            block: "TIM_BASIC",
            ir: &timer::REGISTERS,
        }),
        rcc: Some(PeripheralRcc {
            clock: "pclk1_tim",
            enable: Some(PeripheralRccRegister {
                register: "apb1enr",
                field: "tim7en",
            }),
            reset: Some(PeripheralRccRegister {
                register: "apb1rstr",
                field: "tim7rst",
            }),
            mux: None,
            stop_mode: StopMode::Stop1,
        }),
        pins: &[],
        dma_channels: &[
            PeripheralDmaChannel {
                signal: "UP",
                channel: Some("DMA1_CH2"),
                dmamux: None,
                dma: None,
                request: Some(1),
            },
            PeripheralDmaChannel {
                signal: "UP",
                channel: Some("DMA1_CH4"),
                dmamux: None,
                dma: None,
                request: Some(1),
            },
        ],
        interrupts: &[
            PeripheralInterrupt {
                signal: "BRK",
                interrupt: "TIM7",
            },
            PeripheralInterrupt {
                signal: "CC",
                interrupt: "TIM7",
            },
            PeripheralInterrupt {
                signal: "COM",
                interrupt: "TIM7",
            },
            PeripheralInterrupt {
                signal: "TRG",
                interrupt: "TIM7",
            },
            PeripheralInterrupt {
                signal: "UP",
                interrupt: "TIM7",
            },
        ],
    },
    Peripheral {
        name: "TIM8",
        address: 1073808384,
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
                field: "tim8en",
            }),
            reset: Some(PeripheralRccRegister {
                register: "apb2rstr",
                field: "tim8rst",
            }),
            mux: None,
            stop_mode: StopMode::Stop1,
        }),
        pins: &[
            PeripheralPin {
                pin: "PA0",
                signal: "ETR",
                af: Some(3),
            },
            PeripheralPin {
                pin: "PA5",
                signal: "CH1N",
                af: Some(3),
            },
            PeripheralPin {
                pin: "PA6",
                signal: "BKIN",
                af: Some(3),
            },
            PeripheralPin {
                pin: "PA7",
                signal: "CH1N",
                af: Some(3),
            },
            PeripheralPin {
                pin: "PA8",
                signal: "BKIN2",
                af: Some(3),
            },
            PeripheralPin {
                pin: "PB0",
                signal: "CH2N",
                af: Some(3),
            },
            PeripheralPin {
                pin: "PB1",
                signal: "CH3N",
                af: Some(3),
            },
            PeripheralPin {
                pin: "PB14",
                signal: "CH2N",
                af: Some(3),
            },
            PeripheralPin {
                pin: "PB15",
                signal: "CH3N",
                af: Some(3),
            },
            PeripheralPin {
                pin: "PC6",
                signal: "CH1",
                af: Some(3),
            },
            PeripheralPin {
                pin: "PC7",
                signal: "CH2",
                af: Some(3),
            },
            PeripheralPin {
                pin: "PC8",
                signal: "CH3",
                af: Some(3),
            },
            PeripheralPin {
                pin: "PC9",
                signal: "CH4",
                af: Some(3),
            },
        ],
        dma_channels: &[
            PeripheralDmaChannel {
                signal: "UP",
                channel: Some("DMA2_CH1"),
                dmamux: None,
                dma: None,
                request: Some(7),
            },
            PeripheralDmaChannel {
                signal: "CH1",
                channel: Some("DMA2_CH2"),
                dmamux: None,
                dma: None,
                request: Some(0),
            },
            PeripheralDmaChannel {
                signal: "CH2",
                channel: Some("DMA2_CH2"),
                dmamux: None,
                dma: None,
                request: Some(0),
            },
            PeripheralDmaChannel {
                signal: "CH3",
                channel: Some("DMA2_CH2"),
                dmamux: None,
                dma: None,
                request: Some(0),
            },
            PeripheralDmaChannel {
                signal: "CH1",
                channel: Some("DMA2_CH2"),
                dmamux: None,
                dma: None,
                request: Some(7),
            },
            PeripheralDmaChannel {
                signal: "CH2",
                channel: Some("DMA2_CH3"),
                dmamux: None,
                dma: None,
                request: Some(7),
            },
            PeripheralDmaChannel {
                signal: "CH3",
                channel: Some("DMA2_CH4"),
                dmamux: None,
                dma: None,
                request: Some(7),
            },
            PeripheralDmaChannel {
                signal: "CH4",
                channel: Some("DMA2_CH7"),
                dmamux: None,
                dma: None,
                request: Some(7),
            },
            PeripheralDmaChannel {
                signal: "TRIG",
                channel: Some("DMA2_CH7"),
                dmamux: None,
                dma: None,
                request: Some(7),
            },
            PeripheralDmaChannel {
                signal: "COM",
                channel: Some("DMA2_CH7"),
                dmamux: None,
                dma: None,
                request: Some(7),
            },
        ],
        interrupts: &[
            PeripheralInterrupt {
                signal: "BRK",
                interrupt: "TIM8_BRK_TIM12",
            },
            PeripheralInterrupt {
                signal: "CC",
                interrupt: "TIM8_CC",
            },
            PeripheralInterrupt {
                signal: "COM",
                interrupt: "TIM8_TRG_COM_TIM14",
            },
            PeripheralInterrupt {
                signal: "TRG",
                interrupt: "TIM8_TRG_COM_TIM14",
            },
            PeripheralInterrupt {
                signal: "UP",
                interrupt: "TIM8_UP_TIM13",
            },
        ],
    },
    Peripheral {
        name: "TIM9",
        address: 1073823744,
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
                field: "tim9en",
            }),
            reset: Some(PeripheralRccRegister {
                register: "apb2rstr",
                field: "tim9rst",
            }),
            mux: None,
            stop_mode: StopMode::Stop1,
        }),
        pins: &[
            PeripheralPin {
                pin: "PA2",
                signal: "CH1",
                af: Some(3),
            },
            PeripheralPin {
                pin: "PA3",
                signal: "CH2",
                af: Some(3),
            },
            PeripheralPin {
                pin: "PE5",
                signal: "CH1",
                af: Some(3),
            },
            PeripheralPin {
                pin: "PE6",
                signal: "CH2",
                af: Some(3),
            },
        ],
        dma_channels: &[],
        interrupts: &[
            PeripheralInterrupt {
                signal: "BRK",
                interrupt: "TIM1_BRK_TIM9",
            },
            PeripheralInterrupt {
                signal: "CC",
                interrupt: "TIM1_BRK_TIM9",
            },
            PeripheralInterrupt {
                signal: "COM",
                interrupt: "TIM1_BRK_TIM9",
            },
            PeripheralInterrupt {
                signal: "TRG",
                interrupt: "TIM1_BRK_TIM9",
            },
            PeripheralInterrupt {
                signal: "UP",
                interrupt: "TIM1_BRK_TIM9",
            },
        ],
    },
    Peripheral {
        name: "UART4",
        address: 1073761280,
        registers: Some(PeripheralRegisters {
            kind: "usart",
            version: "v3",
            block: "USART",
            ir: &usart::REGISTERS,
        }),
        rcc: Some(PeripheralRcc {
            clock: "pclk1",
            enable: Some(PeripheralRccRegister {
                register: "apb1enr",
                field: "uart4en",
            }),
            reset: Some(PeripheralRccRegister {
                register: "apb1rstr",
                field: "uart4rst",
            }),
            mux: None,
            stop_mode: StopMode::Stop1,
        }),
        pins: &[
            PeripheralPin {
                pin: "PA0",
                signal: "TX",
                af: Some(8),
            },
            PeripheralPin {
                pin: "PA1",
                signal: "RX",
                af: Some(8),
            },
            PeripheralPin {
                pin: "PA11",
                signal: "RX",
                af: Some(6),
            },
            PeripheralPin {
                pin: "PA12",
                signal: "TX",
                af: Some(6),
            },
            PeripheralPin {
                pin: "PA15",
                signal: "DE",
                af: Some(8),
            },
            PeripheralPin {
                pin: "PA15",
                signal: "RTS",
                af: Some(8),
            },
            PeripheralPin {
                pin: "PB0",
                signal: "CTS",
                af: Some(8),
            },
            PeripheralPin {
                pin: "PB14",
                signal: "DE",
                af: Some(8),
            },
            PeripheralPin {
                pin: "PB14",
                signal: "RTS",
                af: Some(8),
            },
            PeripheralPin {
                pin: "PB15",
                signal: "CTS",
                af: Some(8),
            },
            PeripheralPin {
                pin: "PC10",
                signal: "TX",
                af: Some(8),
            },
            PeripheralPin {
                pin: "PC11",
                signal: "RX",
                af: Some(8),
            },
            PeripheralPin {
                pin: "PD0",
                signal: "RX",
                af: Some(8),
            },
            PeripheralPin {
                pin: "PD1",
                signal: "TX",
                af: Some(8),
            },
        ],
        dma_channels: &[
            PeripheralDmaChannel {
                signal: "RX",
                channel: Some("DMA1_CH2"),
                dmamux: None,
                dma: None,
                request: Some(4),
            },
            PeripheralDmaChannel {
                signal: "TX",
                channel: Some("DMA1_CH4"),
                dmamux: None,
                dma: None,
                request: Some(4),
            },
        ],
        interrupts: &[PeripheralInterrupt {
            signal: "GLOBAL",
            interrupt: "UART4",
        }],
    },
    Peripheral {
        name: "UART5",
        address: 1073762304,
        registers: Some(PeripheralRegisters {
            kind: "usart",
            version: "v3",
            block: "USART",
            ir: &usart::REGISTERS,
        }),
        rcc: Some(PeripheralRcc {
            clock: "pclk1",
            enable: Some(PeripheralRccRegister {
                register: "apb1enr",
                field: "uart5en",
            }),
            reset: Some(PeripheralRccRegister {
                register: "apb1rstr",
                field: "uart5rst",
            }),
            mux: None,
            stop_mode: StopMode::Stop1,
        }),
        pins: &[
            PeripheralPin {
                pin: "PB12",
                signal: "RX",
                af: Some(8),
            },
            PeripheralPin {
                pin: "PB13",
                signal: "TX",
                af: Some(8),
            },
            PeripheralPin {
                pin: "PB5",
                signal: "RX",
                af: Some(1),
            },
            PeripheralPin {
                pin: "PB6",
                signal: "TX",
                af: Some(1),
            },
            PeripheralPin {
                pin: "PB8",
                signal: "RX",
                af: Some(7),
            },
            PeripheralPin {
                pin: "PB9",
                signal: "TX",
                af: Some(7),
            },
            PeripheralPin {
                pin: "PC12",
                signal: "TX",
                af: Some(8),
            },
            PeripheralPin {
                pin: "PC8",
                signal: "DE",
                af: Some(7),
            },
            PeripheralPin {
                pin: "PC8",
                signal: "RTS",
                af: Some(7),
            },
            PeripheralPin {
                pin: "PC9",
                signal: "CTS",
                af: Some(7),
            },
            PeripheralPin {
                pin: "PD2",
                signal: "RX",
                af: Some(8),
            },
        ],
        dma_channels: &[
            PeripheralDmaChannel {
                signal: "RX",
                channel: Some("DMA1_CH0"),
                dmamux: None,
                dma: None,
                request: Some(4),
            },
            PeripheralDmaChannel {
                signal: "TX",
                channel: Some("DMA1_CH7"),
                dmamux: None,
                dma: None,
                request: Some(4),
            },
        ],
        interrupts: &[PeripheralInterrupt {
            signal: "GLOBAL",
            interrupt: "UART5",
        }],
    },
    Peripheral {
        name: "UART7",
        address: 1073772544,
        registers: Some(PeripheralRegisters {
            kind: "usart",
            version: "v3",
            block: "USART",
            ir: &usart::REGISTERS,
        }),
        rcc: Some(PeripheralRcc {
            clock: "pclk1",
            enable: Some(PeripheralRccRegister {
                register: "apb1enr",
                field: "uart7en",
            }),
            reset: Some(PeripheralRccRegister {
                register: "apb1rstr",
                field: "uart7rst",
            }),
            mux: None,
            stop_mode: StopMode::Stop1,
        }),
        pins: &[
            PeripheralPin {
                pin: "PA15",
                signal: "TX",
                af: Some(12),
            },
            PeripheralPin {
                pin: "PA8",
                signal: "RX",
                af: Some(12),
            },
            PeripheralPin {
                pin: "PB3",
                signal: "RX",
                af: Some(12),
            },
            PeripheralPin {
                pin: "PB4",
                signal: "TX",
                af: Some(12),
            },
            PeripheralPin {
                pin: "PE10",
                signal: "CTS",
                af: Some(8),
            },
            PeripheralPin {
                pin: "PE7",
                signal: "RX",
                af: Some(8),
            },
            PeripheralPin {
                pin: "PE8",
                signal: "TX",
                af: Some(8),
            },
            PeripheralPin {
                pin: "PE9",
                signal: "DE",
                af: Some(8),
            },
            PeripheralPin {
                pin: "PE9",
                signal: "RTS",
                af: Some(8),
            },
        ],
        dma_channels: &[
            PeripheralDmaChannel {
                signal: "TX",
                channel: Some("DMA1_CH1"),
                dmamux: None,
                dma: None,
                request: Some(5),
            },
            PeripheralDmaChannel {
                signal: "RX",
                channel: Some("DMA1_CH3"),
                dmamux: None,
                dma: None,
                request: Some(5),
            },
        ],
        interrupts: &[PeripheralInterrupt {
            signal: "GLOBAL",
            interrupt: "UART7",
        }],
    },
    Peripheral {
        name: "UART8",
        address: 1073773568,
        registers: Some(PeripheralRegisters {
            kind: "usart",
            version: "v3",
            block: "USART",
            ir: &usart::REGISTERS,
        }),
        rcc: Some(PeripheralRcc {
            clock: "pclk1",
            enable: Some(PeripheralRccRegister {
                register: "apb1enr",
                field: "uart8en",
            }),
            reset: Some(PeripheralRccRegister {
                register: "apb1rstr",
                field: "uart8rst",
            }),
            mux: None,
            stop_mode: StopMode::Stop1,
        }),
        pins: &[
            PeripheralPin {
                pin: "PD14",
                signal: "CTS",
                af: Some(8),
            },
            PeripheralPin {
                pin: "PD15",
                signal: "DE",
                af: Some(8),
            },
            PeripheralPin {
                pin: "PD15",
                signal: "RTS",
                af: Some(8),
            },
            PeripheralPin {
                pin: "PE0",
                signal: "RX",
                af: Some(8),
            },
            PeripheralPin {
                pin: "PE1",
                signal: "TX",
                af: Some(8),
            },
        ],
        dma_channels: &[
            PeripheralDmaChannel {
                signal: "TX",
                channel: Some("DMA1_CH0"),
                dmamux: None,
                dma: None,
                request: Some(5),
            },
            PeripheralDmaChannel {
                signal: "RX",
                channel: Some("DMA1_CH6"),
                dmamux: None,
                dma: None,
                request: Some(5),
            },
        ],
        interrupts: &[PeripheralInterrupt {
            signal: "GLOBAL",
            interrupt: "UART8",
        }],
    },
    Peripheral {
        name: "UID",
        address: 535884832,
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
        address: 1073811456,
        registers: Some(PeripheralRegisters {
            kind: "usart",
            version: "v3",
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
            mux: None,
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
                pin: "PB14",
                signal: "TX",
                af: Some(4),
            },
            PeripheralPin {
                pin: "PB15",
                signal: "RX",
                af: Some(4),
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
                channel: Some("DMA2_CH2"),
                dmamux: None,
                dma: None,
                request: Some(4),
            },
            PeripheralDmaChannel {
                signal: "RX",
                channel: Some("DMA2_CH5"),
                dmamux: None,
                dma: None,
                request: Some(4),
            },
            PeripheralDmaChannel {
                signal: "TX",
                channel: Some("DMA2_CH7"),
                dmamux: None,
                dma: None,
                request: Some(4),
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
            version: "v3",
            block: "USART",
            ir: &usart::REGISTERS,
        }),
        rcc: Some(PeripheralRcc {
            clock: "pclk1",
            enable: Some(PeripheralRccRegister {
                register: "apb1enr",
                field: "usart2en",
            }),
            reset: Some(PeripheralRccRegister {
                register: "apb1rstr",
                field: "usart2rst",
            }),
            mux: None,
            stop_mode: StopMode::Stop1,
        }),
        pins: &[
            PeripheralPin {
                pin: "PA0",
                signal: "CTS",
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
            PeripheralPin {
                pin: "PD3",
                signal: "CTS",
                af: Some(7),
            },
            PeripheralPin {
                pin: "PD4",
                signal: "DE",
                af: Some(7),
            },
            PeripheralPin {
                pin: "PD4",
                signal: "RTS",
                af: Some(7),
            },
            PeripheralPin {
                pin: "PD5",
                signal: "TX",
                af: Some(7),
            },
            PeripheralPin {
                pin: "PD6",
                signal: "RX",
                af: Some(7),
            },
            PeripheralPin {
                pin: "PD7",
                signal: "CK",
                af: Some(7),
            },
        ],
        dma_channels: &[
            PeripheralDmaChannel {
                signal: "RX",
                channel: Some("DMA1_CH5"),
                dmamux: None,
                dma: None,
                request: Some(4),
            },
            PeripheralDmaChannel {
                signal: "TX",
                channel: Some("DMA1_CH6"),
                dmamux: None,
                dma: None,
                request: Some(4),
            },
        ],
        interrupts: &[PeripheralInterrupt {
            signal: "GLOBAL",
            interrupt: "USART2",
        }],
    },
    Peripheral {
        name: "USART3",
        address: 1073760256,
        registers: Some(PeripheralRegisters {
            kind: "usart",
            version: "v3",
            block: "USART",
            ir: &usart::REGISTERS,
        }),
        rcc: Some(PeripheralRcc {
            clock: "pclk1",
            enable: Some(PeripheralRccRegister {
                register: "apb1enr",
                field: "usart3en",
            }),
            reset: Some(PeripheralRccRegister {
                register: "apb1rstr",
                field: "usart3rst",
            }),
            mux: None,
            stop_mode: StopMode::Stop1,
        }),
        pins: &[
            PeripheralPin {
                pin: "PB10",
                signal: "TX",
                af: Some(7),
            },
            PeripheralPin {
                pin: "PB11",
                signal: "RX",
                af: Some(7),
            },
            PeripheralPin {
                pin: "PB12",
                signal: "CK",
                af: Some(7),
            },
            PeripheralPin {
                pin: "PB13",
                signal: "CTS",
                af: Some(7),
            },
            PeripheralPin {
                pin: "PB14",
                signal: "DE",
                af: Some(7),
            },
            PeripheralPin {
                pin: "PB14",
                signal: "RTS",
                af: Some(7),
            },
            PeripheralPin {
                pin: "PC10",
                signal: "TX",
                af: Some(7),
            },
            PeripheralPin {
                pin: "PC11",
                signal: "RX",
                af: Some(7),
            },
            PeripheralPin {
                pin: "PC12",
                signal: "CK",
                af: Some(7),
            },
            PeripheralPin {
                pin: "PD10",
                signal: "CK",
                af: Some(7),
            },
            PeripheralPin {
                pin: "PD11",
                signal: "CTS",
                af: Some(7),
            },
            PeripheralPin {
                pin: "PD12",
                signal: "DE",
                af: Some(7),
            },
            PeripheralPin {
                pin: "PD12",
                signal: "RTS",
                af: Some(7),
            },
            PeripheralPin {
                pin: "PD8",
                signal: "TX",
                af: Some(7),
            },
            PeripheralPin {
                pin: "PD9",
                signal: "RX",
                af: Some(7),
            },
        ],
        dma_channels: &[
            PeripheralDmaChannel {
                signal: "RX",
                channel: Some("DMA1_CH1"),
                dmamux: None,
                dma: None,
                request: Some(4),
            },
            PeripheralDmaChannel {
                signal: "TX",
                channel: Some("DMA1_CH3"),
                dmamux: None,
                dma: None,
                request: Some(4),
            },
            PeripheralDmaChannel {
                signal: "TX",
                channel: Some("DMA1_CH4"),
                dmamux: None,
                dma: None,
                request: Some(7),
            },
        ],
        interrupts: &[PeripheralInterrupt {
            signal: "GLOBAL",
            interrupt: "USART3",
        }],
    },
    Peripheral {
        name: "USART6",
        address: 1073812480,
        registers: Some(PeripheralRegisters {
            kind: "usart",
            version: "v3",
            block: "USART",
            ir: &usart::REGISTERS,
        }),
        rcc: Some(PeripheralRcc {
            clock: "pclk2",
            enable: Some(PeripheralRccRegister {
                register: "apb2enr",
                field: "usart6en",
            }),
            reset: Some(PeripheralRccRegister {
                register: "apb2rstr",
                field: "usart6rst",
            }),
            mux: None,
            stop_mode: StopMode::Stop1,
        }),
        pins: &[
            PeripheralPin {
                pin: "PC6",
                signal: "TX",
                af: Some(8),
            },
            PeripheralPin {
                pin: "PC7",
                signal: "RX",
                af: Some(8),
            },
            PeripheralPin {
                pin: "PC8",
                signal: "CK",
                af: Some(8),
            },
        ],
        dma_channels: &[
            PeripheralDmaChannel {
                signal: "RX",
                channel: Some("DMA2_CH1"),
                dmamux: None,
                dma: None,
                request: Some(5),
            },
            PeripheralDmaChannel {
                signal: "RX",
                channel: Some("DMA2_CH2"),
                dmamux: None,
                dma: None,
                request: Some(5),
            },
            PeripheralDmaChannel {
                signal: "TX",
                channel: Some("DMA2_CH6"),
                dmamux: None,
                dma: None,
                request: Some(5),
            },
            PeripheralDmaChannel {
                signal: "TX",
                channel: Some("DMA2_CH7"),
                dmamux: None,
                dma: None,
                request: Some(5),
            },
        ],
        interrupts: &[PeripheralInterrupt {
            signal: "GLOBAL",
            interrupt: "USART6",
        }],
    },
    Peripheral {
        name: "USB_OTG_FS",
        address: 1342177280,
        registers: Some(PeripheralRegisters {
            kind: "otg",
            version: "v1",
            block: "OTG",
            ir: &otg::REGISTERS,
        }),
        rcc: Some(PeripheralRcc {
            clock: "hclk2",
            enable: Some(PeripheralRccRegister {
                register: "ahb2enr",
                field: "usb_otg_fsen",
            }),
            reset: Some(PeripheralRccRegister {
                register: "ahb2rstr",
                field: "usb_otg_fsrst",
            }),
            mux: None,
            stop_mode: StopMode::Stop1,
        }),
        pins: &[
            PeripheralPin {
                pin: "PA10",
                signal: "ID",
                af: Some(10),
            },
            PeripheralPin {
                pin: "PA11",
                signal: "DM",
                af: Some(10),
            },
            PeripheralPin {
                pin: "PA12",
                signal: "DP",
                af: Some(10),
            },
            PeripheralPin {
                pin: "PA8",
                signal: "SOF",
                af: Some(10),
            },
            PeripheralPin {
                pin: "PA9",
                signal: "VBUS",
                af: None,
            },
        ],
        dma_channels: &[],
        interrupts: &[
            PeripheralInterrupt {
                signal: "EP1_IN",
                interrupt: "OTG_FS",
            },
            PeripheralInterrupt {
                signal: "EP1_OUT",
                interrupt: "OTG_FS",
            },
            PeripheralInterrupt {
                signal: "GLOBAL",
                interrupt: "OTG_FS",
            },
            PeripheralInterrupt {
                signal: "WKUP",
                interrupt: "OTG_FS",
            },
            PeripheralInterrupt {
                signal: "WKUP",
                interrupt: "OTG_FS_WKUP",
            },
        ],
    },
    Peripheral {
        name: "USB_OTG_HS",
        address: 1074003968,
        registers: Some(PeripheralRegisters {
            kind: "otg",
            version: "v1",
            block: "OTG",
            ir: &otg::REGISTERS,
        }),
        rcc: Some(PeripheralRcc {
            clock: "hclk1",
            enable: Some(PeripheralRccRegister {
                register: "ahb1enr",
                field: "usb_otg_hsen",
            }),
            reset: Some(PeripheralRccRegister {
                register: "ahb1rstr",
                field: "usb_otg_hsrst",
            }),
            mux: None,
            stop_mode: StopMode::Stop1,
        }),
        pins: &[
            PeripheralPin {
                pin: "PA3",
                signal: "ULPI_D0",
                af: Some(10),
            },
            PeripheralPin {
                pin: "PA4",
                signal: "SOF",
                af: Some(12),
            },
            PeripheralPin {
                pin: "PA5",
                signal: "ULPI_CK",
                af: Some(10),
            },
            PeripheralPin {
                pin: "PB0",
                signal: "ULPI_D1",
                af: Some(10),
            },
            PeripheralPin {
                pin: "PB1",
                signal: "ULPI_D2",
                af: Some(10),
            },
            PeripheralPin {
                pin: "PB10",
                signal: "ULPI_D3",
                af: Some(10),
            },
            PeripheralPin {
                pin: "PB11",
                signal: "ULPI_D4",
                af: Some(10),
            },
            PeripheralPin {
                pin: "PB12",
                signal: "ID",
                af: Some(12),
            },
            PeripheralPin {
                pin: "PB12",
                signal: "ULPI_D5",
                af: Some(10),
            },
            PeripheralPin {
                pin: "PB13",
                signal: "ULPI_D6",
                af: Some(10),
            },
            PeripheralPin {
                pin: "PB13",
                signal: "VBUS",
                af: None,
            },
            PeripheralPin {
                pin: "PB14",
                signal: "DM",
                af: Some(12),
            },
            PeripheralPin {
                pin: "PB15",
                signal: "DP",
                af: Some(12),
            },
            PeripheralPin {
                pin: "PB5",
                signal: "ULPI_D7",
                af: Some(10),
            },
            PeripheralPin {
                pin: "PC0",
                signal: "ULPI_STP",
                af: Some(10),
            },
            PeripheralPin {
                pin: "PC2",
                signal: "ULPI_DIR",
                af: Some(10),
            },
            PeripheralPin {
                pin: "PC3",
                signal: "ULPI_NXT",
                af: Some(10),
            },
        ],
        dma_channels: &[],
        interrupts: &[
            PeripheralInterrupt {
                signal: "EP1_IN",
                interrupt: "OTG_HS",
            },
            PeripheralInterrupt {
                signal: "EP1_IN",
                interrupt: "OTG_HS_EP1_IN",
            },
            PeripheralInterrupt {
                signal: "EP1_OUT",
                interrupt: "OTG_HS",
            },
            PeripheralInterrupt {
                signal: "EP1_OUT",
                interrupt: "OTG_HS_EP1_OUT",
            },
            PeripheralInterrupt {
                signal: "GLOBAL",
                interrupt: "OTG_HS",
            },
            PeripheralInterrupt {
                signal: "WKUP",
                interrupt: "OTG_HS",
            },
            PeripheralInterrupt {
                signal: "WKUP",
                interrupt: "OTG_HS_WKUP",
            },
        ],
    },
    Peripheral {
        name: "WWDG",
        address: 1073753088,
        registers: Some(PeripheralRegisters {
            kind: "wwdg",
            version: "v1",
            block: "WWDG",
            ir: &wwdg::REGISTERS,
        }),
        rcc: Some(PeripheralRcc {
            clock: "pclk1",
            enable: Some(PeripheralRccRegister {
                register: "apb1enr",
                field: "wwdgen",
            }),
            reset: Some(PeripheralRccRegister {
                register: "apb1rstr",
                field: "wwdgrst",
            }),
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
    Interrupt { name: "PVD", number: 1 },
    Interrupt {
        name: "TAMP_STAMP",
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
        name: "DMA1_STREAM0",
        number: 11,
    },
    Interrupt {
        name: "DMA1_STREAM1",
        number: 12,
    },
    Interrupt {
        name: "DMA1_STREAM2",
        number: 13,
    },
    Interrupt {
        name: "DMA1_STREAM3",
        number: 14,
    },
    Interrupt {
        name: "DMA1_STREAM4",
        number: 15,
    },
    Interrupt {
        name: "DMA1_STREAM5",
        number: 16,
    },
    Interrupt {
        name: "DMA1_STREAM6",
        number: 17,
    },
    Interrupt {
        name: "ADC",
        number: 18,
    },
    Interrupt {
        name: "CAN1_TX",
        number: 19,
    },
    Interrupt {
        name: "CAN1_RX0",
        number: 20,
    },
    Interrupt {
        name: "CAN1_RX1",
        number: 21,
    },
    Interrupt {
        name: "CAN1_SCE",
        number: 22,
    },
    Interrupt {
        name: "EXTI9_5",
        number: 23,
    },
    Interrupt {
        name: "TIM1_BRK_TIM9",
        number: 24,
    },
    Interrupt {
        name: "TIM1_UP_TIM10",
        number: 25,
    },
    Interrupt {
        name: "TIM1_TRG_COM_TIM11",
        number: 26,
    },
    Interrupt {
        name: "TIM1_CC",
        number: 27,
    },
    Interrupt {
        name: "TIM2",
        number: 28,
    },
    Interrupt {
        name: "TIM3",
        number: 29,
    },
    Interrupt {
        name: "TIM4",
        number: 30,
    },
    Interrupt {
        name: "I2C1_EV",
        number: 31,
    },
    Interrupt {
        name: "I2C1_ER",
        number: 32,
    },
    Interrupt {
        name: "I2C2_EV",
        number: 33,
    },
    Interrupt {
        name: "I2C2_ER",
        number: 34,
    },
    Interrupt {
        name: "SPI1",
        number: 35,
    },
    Interrupt {
        name: "SPI2",
        number: 36,
    },
    Interrupt {
        name: "USART1",
        number: 37,
    },
    Interrupt {
        name: "USART2",
        number: 38,
    },
    Interrupt {
        name: "USART3",
        number: 39,
    },
    Interrupt {
        name: "EXTI15_10",
        number: 40,
    },
    Interrupt {
        name: "RTC_ALARM",
        number: 41,
    },
    Interrupt {
        name: "OTG_FS_WKUP",
        number: 42,
    },
    Interrupt {
        name: "TIM8_BRK_TIM12",
        number: 43,
    },
    Interrupt {
        name: "TIM8_UP_TIM13",
        number: 44,
    },
    Interrupt {
        name: "TIM8_TRG_COM_TIM14",
        number: 45,
    },
    Interrupt {
        name: "TIM8_CC",
        number: 46,
    },
    Interrupt {
        name: "DMA1_STREAM7",
        number: 47,
    },
    Interrupt {
        name: "FMC",
        number: 48,
    },
    Interrupt {
        name: "SDMMC1",
        number: 49,
    },
    Interrupt {
        name: "TIM5",
        number: 50,
    },
    Interrupt {
        name: "SPI3",
        number: 51,
    },
    Interrupt {
        name: "UART4",
        number: 52,
    },
    Interrupt {
        name: "UART5",
        number: 53,
    },
    Interrupt {
        name: "TIM6_DAC",
        number: 54,
    },
    Interrupt {
        name: "TIM7",
        number: 55,
    },
    Interrupt {
        name: "DMA2_STREAM0",
        number: 56,
    },
    Interrupt {
        name: "DMA2_STREAM1",
        number: 57,
    },
    Interrupt {
        name: "DMA2_STREAM2",
        number: 58,
    },
    Interrupt {
        name: "DMA2_STREAM3",
        number: 59,
    },
    Interrupt {
        name: "DMA2_STREAM4",
        number: 60,
    },
    Interrupt {
        name: "ETH",
        number: 61,
    },
    Interrupt {
        name: "ETH_WKUP",
        number: 62,
    },
    Interrupt {
        name: "CAN2_TX",
        number: 63,
    },
    Interrupt {
        name: "CAN2_RX0",
        number: 64,
    },
    Interrupt {
        name: "CAN2_RX1",
        number: 65,
    },
    Interrupt {
        name: "CAN2_SCE",
        number: 66,
    },
    Interrupt {
        name: "OTG_FS",
        number: 67,
    },
    Interrupt {
        name: "DMA2_STREAM5",
        number: 68,
    },
    Interrupt {
        name: "DMA2_STREAM6",
        number: 69,
    },
    Interrupt {
        name: "DMA2_STREAM7",
        number: 70,
    },
    Interrupt {
        name: "USART6",
        number: 71,
    },
    Interrupt {
        name: "I2C3_EV",
        number: 72,
    },
    Interrupt {
        name: "I2C3_ER",
        number: 73,
    },
    Interrupt {
        name: "OTG_HS_EP1_OUT",
        number: 74,
    },
    Interrupt {
        name: "OTG_HS_EP1_IN",
        number: 75,
    },
    Interrupt {
        name: "OTG_HS_WKUP",
        number: 76,
    },
    Interrupt {
        name: "OTG_HS",
        number: 77,
    },
    Interrupt {
        name: "DCMI",
        number: 78,
    },
    Interrupt {
        name: "CRYP",
        number: 79,
    },
    Interrupt {
        name: "HASH_RNG",
        number: 80,
    },
    Interrupt {
        name: "FPU",
        number: 81,
    },
    Interrupt {
        name: "UART7",
        number: 82,
    },
    Interrupt {
        name: "UART8",
        number: 83,
    },
    Interrupt {
        name: "SPI4",
        number: 84,
    },
    Interrupt {
        name: "SPI5",
        number: 85,
    },
    Interrupt {
        name: "SPI6",
        number: 86,
    },
    Interrupt {
        name: "SAI1",
        number: 87,
    },
    Interrupt {
        name: "LTDC",
        number: 88,
    },
    Interrupt {
        name: "LTDC_ER",
        number: 89,
    },
    Interrupt {
        name: "DMA2D",
        number: 90,
    },
    Interrupt {
        name: "SAI2",
        number: 91,
    },
    Interrupt {
        name: "QUADSPI",
        number: 92,
    },
    Interrupt {
        name: "LPTIM1",
        number: 93,
    },
    Interrupt {
        name: "CEC",
        number: 94,
    },
    Interrupt {
        name: "I2C4_EV",
        number: 95,
    },
    Interrupt {
        name: "I2C4_ER",
        number: 96,
    },
    Interrupt {
        name: "SPDIF_RX",
        number: 97,
    },
    Interrupt {
        name: "SDMMC2",
        number: 103,
    },
    Interrupt {
        name: "CAN3_TX",
        number: 104,
    },
    Interrupt {
        name: "CAN3_RX0",
        number: 105,
    },
    Interrupt {
        name: "CAN3_RX1",
        number: 106,
    },
    Interrupt {
        name: "CAN3_SCE",
        number: 107,
    },
    Interrupt {
        name: "JPEG",
        number: 108,
    },
    Interrupt {
        name: "MDIOS",
        number: 109,
    },
];
pub(crate) static DMA_CHANNELS: &'static [DmaChannel] = &[
    DmaChannel {
        name: "DMA1_CH0",
        dma: "DMA1",
        channel: 0,
        dmamux: None,
        dmamux_channel: None,
    },
    DmaChannel {
        name: "DMA1_CH1",
        dma: "DMA1",
        channel: 1,
        dmamux: None,
        dmamux_channel: None,
    },
    DmaChannel {
        name: "DMA1_CH2",
        dma: "DMA1",
        channel: 2,
        dmamux: None,
        dmamux_channel: None,
    },
    DmaChannel {
        name: "DMA1_CH3",
        dma: "DMA1",
        channel: 3,
        dmamux: None,
        dmamux_channel: None,
    },
    DmaChannel {
        name: "DMA1_CH4",
        dma: "DMA1",
        channel: 4,
        dmamux: None,
        dmamux_channel: None,
    },
    DmaChannel {
        name: "DMA1_CH5",
        dma: "DMA1",
        channel: 5,
        dmamux: None,
        dmamux_channel: None,
    },
    DmaChannel {
        name: "DMA1_CH6",
        dma: "DMA1",
        channel: 6,
        dmamux: None,
        dmamux_channel: None,
    },
    DmaChannel {
        name: "DMA1_CH7",
        dma: "DMA1",
        channel: 7,
        dmamux: None,
        dmamux_channel: None,
    },
    DmaChannel {
        name: "DMA2_CH0",
        dma: "DMA2",
        channel: 0,
        dmamux: None,
        dmamux_channel: None,
    },
    DmaChannel {
        name: "DMA2_CH1",
        dma: "DMA2",
        channel: 1,
        dmamux: None,
        dmamux_channel: None,
    },
    DmaChannel {
        name: "DMA2_CH2",
        dma: "DMA2",
        channel: 2,
        dmamux: None,
        dmamux_channel: None,
    },
    DmaChannel {
        name: "DMA2_CH3",
        dma: "DMA2",
        channel: 3,
        dmamux: None,
        dmamux_channel: None,
    },
    DmaChannel {
        name: "DMA2_CH4",
        dma: "DMA2",
        channel: 4,
        dmamux: None,
        dmamux_channel: None,
    },
    DmaChannel {
        name: "DMA2_CH5",
        dma: "DMA2",
        channel: 5,
        dmamux: None,
        dmamux_channel: None,
    },
    DmaChannel {
        name: "DMA2_CH6",
        dma: "DMA2",
        channel: 6,
        dmamux: None,
        dmamux_channel: None,
    },
    DmaChannel {
        name: "DMA2_CH7",
        dma: "DMA2",
        channel: 7,
        dmamux: None,
        dmamux_channel: None,
    },
];
#[path = "../registers/adc_v2.rs"]
pub mod adc;
#[path = "../registers/adccommon_v2.rs"]
pub mod adccommon;
#[path = "../registers/can_bxcan.rs"]
pub mod can;
#[path = "../registers/crc_v2.rs"]
pub mod crc;
#[path = "../registers/dac_v1.rs"]
pub mod dac;
#[path = "../registers/dbgmcu_f7.rs"]
pub mod dbgmcu;
#[path = "../registers/dcmi_v1.rs"]
pub mod dcmi;
#[path = "../registers/dma_v2.rs"]
pub mod dma;
#[path = "../registers/dma2d_v1.rs"]
pub mod dma2d;
#[path = "../registers/eth_v1c.rs"]
pub mod eth;
#[path = "../registers/exti_v1.rs"]
pub mod exti;
#[path = "../registers/flash_f7.rs"]
pub mod flash;
#[path = "../registers/fmc_v2x1.rs"]
pub mod fmc;
#[path = "../registers/gpio_v2.rs"]
pub mod gpio;
#[path = "../registers/i2c_v2.rs"]
pub mod i2c;
#[path = "../registers/iwdg_v2.rs"]
pub mod iwdg;
#[path = "../registers/jpeg_v1.rs"]
pub mod jpeg;
#[path = "../registers/lptim_v1.rs"]
pub mod lptim;
#[path = "../registers/ltdc_v1.rs"]
pub mod ltdc;
#[path = "../registers/mdios_v1.rs"]
pub mod mdios;
#[path = "../registers/otg_v1.rs"]
pub mod otg;
#[path = "../registers/pwr_f7.rs"]
pub mod pwr;
#[path = "../registers/quadspi_v1.rs"]
pub mod quadspi;
#[path = "../registers/rcc_f7.rs"]
pub mod rcc;
#[path = "../registers/rng_v1.rs"]
pub mod rng;
#[path = "../registers/rtc_v2f7.rs"]
pub mod rtc;
#[path = "../registers/sai_v2.rs"]
pub mod sai;
#[path = "../registers/sdmmc_v1.rs"]
pub mod sdmmc;
#[path = "../registers/spdifrx_v1.rs"]
pub mod spdifrx;
#[path = "../registers/spi_v2.rs"]
pub mod spi;
#[path = "../registers/syscfg_f7.rs"]
pub mod syscfg;
#[path = "../registers/timer_v1.rs"]
pub mod timer;
#[path = "../registers/uid_v1.rs"]
pub mod uid;
#[path = "../registers/usart_v3.rs"]
pub mod usart;
#[path = "../registers/wwdg_v1.rs"]
pub mod wwdg;

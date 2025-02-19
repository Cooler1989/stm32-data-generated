
use crate::metadata::ir::*;
pub(crate) static REGISTERS: IR = IR {
    blocks: &[Block {
        name: "AdcCommon",
        extends: None,
        description: Some("ADC common registers"),
        items: &[
            BlockItem {
                name: "csr",
                description: Some("ADC Common status register"),
                array: None,
                byte_offset: 0,
                inner: BlockItemInner::Register(Register {
                    access: Access::Read,
                    bit_size: 32,
                    fieldset: Some("Csr"),
                }),
            },
            BlockItem {
                name: "ccr",
                description: Some("ADC common control register"),
                array: None,
                byte_offset: 4,
                inner: BlockItemInner::Register(Register {
                    access: Access::ReadWrite,
                    bit_size: 32,
                    fieldset: Some("Ccr"),
                }),
            },
            BlockItem {
                name: "cdr",
                description: Some("ADC common regular data register for dual and triple modes"),
                array: None,
                byte_offset: 8,
                inner: BlockItemInner::Register(Register {
                    access: Access::Read,
                    bit_size: 32,
                    fieldset: Some("Cdr"),
                }),
            },
        ],
    }],
    fieldsets: &[
        FieldSet {
            name: "Ccr",
            extends: None,
            description: Some("ADC common control register"),
            bit_size: 32,
            fields: &[
                Field {
                    name: "multi",
                    description: Some("Multi ADC mode selection"),
                    bit_offset: 0,
                    bit_size: 5,
                    array: None,
                    enumm: Some("Multi"),
                },
                Field {
                    name: "delay",
                    description: Some("Delay between 2 sampling phases"),
                    bit_offset: 8,
                    bit_size: 4,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "dds",
                    description: Some("DMA disable selection for multi-ADC mode"),
                    bit_offset: 13,
                    bit_size: 1,
                    array: None,
                    enumm: Some("Dds"),
                },
                Field {
                    name: "dma",
                    description: Some("Direct memory access mode for multi ADC mode"),
                    bit_offset: 14,
                    bit_size: 2,
                    array: None,
                    enumm: Some("Dma"),
                },
                Field {
                    name: "adcpre",
                    description: Some("ADC prescaler"),
                    bit_offset: 16,
                    bit_size: 2,
                    array: None,
                    enumm: Some("Adcpre"),
                },
                Field {
                    name: "vbate",
                    description: Some("VBAT enable"),
                    bit_offset: 22,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "tsvrefe",
                    description: Some("Temperature sensor and VREFINT enable"),
                    bit_offset: 23,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Cdr",
            extends: None,
            description: Some("ADC common regular data register for dual and triple modes"),
            bit_size: 32,
            fields: &[Field {
                name: "data",
                description: Some("1st data item of a pair of regular conversions"),
                bit_offset: 0,
                bit_size: 16,
                array: Some(Array::Regular(RegularArray { len: 2, stride: 16 })),
                enumm: None,
            }],
        },
        FieldSet {
            name: "Csr",
            extends: None,
            description: Some("ADC common status register"),
            bit_size: 32,
            fields: &[
                Field {
                    name: "awd",
                    description: Some("Analog watchdog flag of ADC 1"),
                    bit_offset: 0,
                    bit_size: 1,
                    array: Some(Array::Regular(RegularArray { len: 3, stride: 8 })),
                    enumm: Some("Awd"),
                },
                Field {
                    name: "eoc",
                    description: Some("End of conversion of ADC 1"),
                    bit_offset: 1,
                    bit_size: 1,
                    array: Some(Array::Regular(RegularArray { len: 3, stride: 8 })),
                    enumm: Some("Eoc"),
                },
                Field {
                    name: "jeoc",
                    description: Some("Injected channel end of conversion of ADC 1"),
                    bit_offset: 2,
                    bit_size: 1,
                    array: Some(Array::Regular(RegularArray { len: 3, stride: 8 })),
                    enumm: Some("Jeoc"),
                },
                Field {
                    name: "jstrt",
                    description: Some("Injected channel Start flag of ADC 1"),
                    bit_offset: 3,
                    bit_size: 1,
                    array: Some(Array::Regular(RegularArray { len: 3, stride: 8 })),
                    enumm: Some("Jstrt"),
                },
                Field {
                    name: "strt",
                    description: Some("Regular channel Start flag of ADC 1"),
                    bit_offset: 4,
                    bit_size: 1,
                    array: Some(Array::Regular(RegularArray { len: 3, stride: 8 })),
                    enumm: Some("Strt"),
                },
                Field {
                    name: "ovr",
                    description: Some("Overrun flag of ADC 1"),
                    bit_offset: 5,
                    bit_size: 1,
                    array: Some(Array::Regular(RegularArray { len: 3, stride: 8 })),
                    enumm: Some("Ovr"),
                },
            ],
        },
    ],
    enums: &[
        Enum {
            name: "Adcpre",
            description: None,
            bit_size: 2,
            variants: &[
                EnumVariant {
                    name: "DIV2",
                    description: Some("PCLK2 divided by 2"),
                    value: 0,
                },
                EnumVariant {
                    name: "DIV4",
                    description: Some("PCLK2 divided by 4"),
                    value: 1,
                },
                EnumVariant {
                    name: "DIV6",
                    description: Some("PCLK2 divided by 6"),
                    value: 2,
                },
                EnumVariant {
                    name: "DIV8",
                    description: Some("PCLK2 divided by 8"),
                    value: 3,
                },
            ],
        },
        Enum {
            name: "Awd",
            description: None,
            bit_size: 1,
            variants: &[
                EnumVariant {
                    name: "NOEVENT",
                    description: Some("No analog watchdog event occurred"),
                    value: 0,
                },
                EnumVariant {
                    name: "EVENT",
                    description: Some("Analog watchdog event occurred"),
                    value: 1,
                },
            ],
        },
        Enum {
            name: "Dds",
            description: None,
            bit_size: 1,
            variants: &[
                EnumVariant {
                    name: "SINGLE",
                    description: Some("No new DMA request is issued after the last transfer"),
                    value: 0,
                },
                EnumVariant {
                    name: "CONTINUOUS",
                    description: Some("DMA requests are issued as long as data are converted and DMA=01, 10 or 11"),
                    value: 1,
                },
            ],
        },
        Enum {
            name: "Dma",
            description: None,
            bit_size: 2,
            variants: &[
                EnumVariant {
                    name: "DISABLED",
                    description: Some("DMA mode disabled"),
                    value: 0,
                },
                EnumVariant {
                    name: "MODE1",
                    description: Some("DMA mode 1 enabled (2 / 3 half-words one by one - 1 then 2 then 3)"),
                    value: 1,
                },
                EnumVariant {
                    name: "MODE2",
                    description: Some("DMA mode 2 enabled (2 / 3 half-words by pairs - 2&1 then 1&3 then 3&2)"),
                    value: 2,
                },
                EnumVariant {
                    name: "MODE3",
                    description: Some("DMA mode 3 enabled (2 / 3 half-words by pairs - 2&1 then 1&3 then 3&2)"),
                    value: 3,
                },
            ],
        },
        Enum {
            name: "Eoc",
            description: None,
            bit_size: 1,
            variants: &[
                EnumVariant {
                    name: "NOTCOMPLETE",
                    description: Some("Conversion is not complete"),
                    value: 0,
                },
                EnumVariant {
                    name: "COMPLETE",
                    description: Some("Conversion complete"),
                    value: 1,
                },
            ],
        },
        Enum {
            name: "Jeoc",
            description: None,
            bit_size: 1,
            variants: &[
                EnumVariant {
                    name: "NOTCOMPLETE",
                    description: Some("Conversion is not complete"),
                    value: 0,
                },
                EnumVariant {
                    name: "COMPLETE",
                    description: Some("Conversion complete"),
                    value: 1,
                },
            ],
        },
        Enum {
            name: "Jstrt",
            description: None,
            bit_size: 1,
            variants: &[
                EnumVariant {
                    name: "NOTSTARTED",
                    description: Some("No injected channel conversion started"),
                    value: 0,
                },
                EnumVariant {
                    name: "STARTED",
                    description: Some("Injected channel conversion has started"),
                    value: 1,
                },
            ],
        },
        Enum {
            name: "Multi",
            description: None,
            bit_size: 5,
            variants: &[
                EnumVariant {
                    name: "INDEPENDENT",
                    description: Some("All the ADCs independent: independent mode"),
                    value: 0,
                },
                EnumVariant {
                    name: "DUALRJ",
                    description: Some("Dual ADC1 and ADC2, combined regular and injected simultaneous mode"),
                    value: 1,
                },
                EnumVariant {
                    name: "DUALRA",
                    description: Some("Dual ADC1 and ADC2, combined regular and alternate trigger mode"),
                    value: 2,
                },
                EnumVariant {
                    name: "DUALJ",
                    description: Some("Dual ADC1 and ADC2, injected simultaneous mode only"),
                    value: 5,
                },
                EnumVariant {
                    name: "DUALR",
                    description: Some("Dual ADC1 and ADC2, regular simultaneous mode only"),
                    value: 6,
                },
                EnumVariant {
                    name: "DUALI",
                    description: Some("Dual ADC1 and ADC2, interleaved mode only"),
                    value: 7,
                },
                EnumVariant {
                    name: "DUALA",
                    description: Some("Dual ADC1 and ADC2, alternate trigger mode only"),
                    value: 9,
                },
                EnumVariant {
                    name: "TRIPLERJ",
                    description: Some("Triple ADC, regular and injected simultaneous mode"),
                    value: 17,
                },
                EnumVariant {
                    name: "TRIPLERA",
                    description: Some("Triple ADC, regular and alternate trigger mode"),
                    value: 18,
                },
                EnumVariant {
                    name: "TRIPLEJ",
                    description: Some("Triple ADC, injected simultaneous mode only"),
                    value: 21,
                },
                EnumVariant {
                    name: "TRIPLER",
                    description: Some("Triple ADC, regular simultaneous mode only"),
                    value: 22,
                },
                EnumVariant {
                    name: "TRIPLEI",
                    description: Some("Triple ADC, interleaved mode only"),
                    value: 23,
                },
                EnumVariant {
                    name: "TRIPLEA",
                    description: Some("Triple ADC, alternate trigger mode only"),
                    value: 24,
                },
            ],
        },
        Enum {
            name: "Ovr",
            description: None,
            bit_size: 1,
            variants: &[
                EnumVariant {
                    name: "NOOVERRUN",
                    description: Some("No overrun occurred"),
                    value: 0,
                },
                EnumVariant {
                    name: "OVERRUN",
                    description: Some("Overrun occurred"),
                    value: 1,
                },
            ],
        },
        Enum {
            name: "Strt",
            description: None,
            bit_size: 1,
            variants: &[
                EnumVariant {
                    name: "NOTSTARTED",
                    description: Some("No regular channel conversion started"),
                    value: 0,
                },
                EnumVariant {
                    name: "STARTED",
                    description: Some("Regular channel conversion has started"),
                    value: 1,
                },
            ],
        },
    ],
};

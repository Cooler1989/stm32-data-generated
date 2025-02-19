
use crate::metadata::ir::*;
pub(crate) static REGISTERS: IR = IR {
    blocks: &[Block {
        name: "Wwdg",
        extends: None,
        description: Some("Window watchdog"),
        items: &[
            BlockItem {
                name: "cr",
                description: Some("Control register"),
                array: None,
                byte_offset: 0,
                inner: BlockItemInner::Register(Register {
                    access: Access::ReadWrite,
                    bit_size: 32,
                    fieldset: Some("Cr"),
                }),
            },
            BlockItem {
                name: "cfr",
                description: Some("Configuration register"),
                array: None,
                byte_offset: 4,
                inner: BlockItemInner::Register(Register {
                    access: Access::ReadWrite,
                    bit_size: 32,
                    fieldset: Some("Cfr"),
                }),
            },
            BlockItem {
                name: "sr",
                description: Some("Status register"),
                array: None,
                byte_offset: 8,
                inner: BlockItemInner::Register(Register {
                    access: Access::ReadWrite,
                    bit_size: 32,
                    fieldset: Some("Sr"),
                }),
            },
        ],
    }],
    fieldsets: &[
        FieldSet {
            name: "Cfr",
            extends: None,
            description: Some("Configuration register"),
            bit_size: 32,
            fields: &[
                Field {
                    name: "w",
                    description: Some("7-bit window value"),
                    bit_offset: 0,
                    bit_size: 7,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "wdgtb",
                    description: Some("Timer base"),
                    bit_offset: 7,
                    bit_size: 2,
                    array: None,
                    enumm: Some("Wdgtb"),
                },
                Field {
                    name: "ewi",
                    description: Some("Early wakeup interrupt"),
                    bit_offset: 9,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Cr",
            extends: None,
            description: Some("Control register"),
            bit_size: 32,
            fields: &[
                Field {
                    name: "t",
                    description: Some("7-bit counter (MSB to LSB)"),
                    bit_offset: 0,
                    bit_size: 7,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "wdga",
                    description: Some("Activation bit"),
                    bit_offset: 7,
                    bit_size: 1,
                    array: None,
                    enumm: Some("Wdga"),
                },
            ],
        },
        FieldSet {
            name: "Sr",
            extends: None,
            description: Some("Status register"),
            bit_size: 32,
            fields: &[Field {
                name: "ewif",
                description: Some("Early wakeup interrupt flag"),
                bit_offset: 0,
                bit_size: 1,
                array: None,
                enumm: None,
            }],
        },
    ],
    enums: &[
        Enum {
            name: "Wdga",
            description: None,
            bit_size: 1,
            variants: &[
                EnumVariant {
                    name: "DISABLED",
                    description: Some("Watchdog disabled"),
                    value: 0,
                },
                EnumVariant {
                    name: "ENABLED",
                    description: Some("Watchdog enabled"),
                    value: 1,
                },
            ],
        },
        Enum {
            name: "Wdgtb",
            description: None,
            bit_size: 2,
            variants: &[
                EnumVariant {
                    name: "DIV1",
                    description: Some("Counter clock (PCLK1 div 4096) div 1"),
                    value: 0,
                },
                EnumVariant {
                    name: "DIV2",
                    description: Some("Counter clock (PCLK1 div 4096) div 2"),
                    value: 1,
                },
                EnumVariant {
                    name: "DIV4",
                    description: Some("Counter clock (PCLK1 div 4096) div 4"),
                    value: 2,
                },
                EnumVariant {
                    name: "DIV8",
                    description: Some("Counter clock (PCLK1 div 4096) div 8"),
                    value: 3,
                },
            ],
        },
    ],
};

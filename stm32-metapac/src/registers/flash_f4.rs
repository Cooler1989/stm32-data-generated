
use crate::metadata::ir::*;
pub(crate) static REGISTERS: IR = IR {
    blocks: &[Block {
        name: "Flash",
        extends: None,
        description: Some("FLASH"),
        items: &[
            BlockItem {
                name: "acr",
                description: Some("Flash access control register"),
                array: None,
                byte_offset: 0,
                inner: BlockItemInner::Register(Register {
                    access: Access::ReadWrite,
                    bit_size: 32,
                    fieldset: Some("Acr"),
                }),
            },
            BlockItem {
                name: "keyr",
                description: Some("Flash key register"),
                array: None,
                byte_offset: 4,
                inner: BlockItemInner::Register(Register {
                    access: Access::Write,
                    bit_size: 32,
                    fieldset: Some("Keyr"),
                }),
            },
            BlockItem {
                name: "optkeyr",
                description: Some("Flash option key register"),
                array: None,
                byte_offset: 8,
                inner: BlockItemInner::Register(Register {
                    access: Access::Write,
                    bit_size: 32,
                    fieldset: Some("Optkeyr"),
                }),
            },
            BlockItem {
                name: "sr",
                description: Some("Status register"),
                array: None,
                byte_offset: 12,
                inner: BlockItemInner::Register(Register {
                    access: Access::ReadWrite,
                    bit_size: 32,
                    fieldset: Some("Sr"),
                }),
            },
            BlockItem {
                name: "cr",
                description: Some("Control register"),
                array: None,
                byte_offset: 16,
                inner: BlockItemInner::Register(Register {
                    access: Access::ReadWrite,
                    bit_size: 32,
                    fieldset: Some("Cr"),
                }),
            },
            BlockItem {
                name: "optcr",
                description: Some("Flash option control register"),
                array: None,
                byte_offset: 20,
                inner: BlockItemInner::Register(Register {
                    access: Access::ReadWrite,
                    bit_size: 32,
                    fieldset: Some("Optcr"),
                }),
            },
        ],
    }],
    fieldsets: &[
        FieldSet {
            name: "Acr",
            extends: None,
            description: Some("Flash access control register"),
            bit_size: 32,
            fields: &[
                Field {
                    name: "latency",
                    description: Some("Latency"),
                    bit_offset: 0,
                    bit_size: 3,
                    array: None,
                    enumm: Some("Latency"),
                },
                Field {
                    name: "prften",
                    description: Some("Prefetch enable"),
                    bit_offset: 8,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "icen",
                    description: Some("Instruction cache enable"),
                    bit_offset: 9,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "dcen",
                    description: Some("Data cache enable"),
                    bit_offset: 10,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "icrst",
                    description: Some("Instruction cache reset"),
                    bit_offset: 11,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "dcrst",
                    description: Some("Data cache reset"),
                    bit_offset: 12,
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
                    name: "pg",
                    description: Some("Programming"),
                    bit_offset: 0,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "ser",
                    description: Some("Sector Erase"),
                    bit_offset: 1,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "mer",
                    description: Some("Mass Erase"),
                    bit_offset: 2,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "snb",
                    description: Some("Sector number"),
                    bit_offset: 3,
                    bit_size: 5,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "psize",
                    description: Some("Program size"),
                    bit_offset: 8,
                    bit_size: 2,
                    array: None,
                    enumm: Some("Psize"),
                },
                Field {
                    name: "strt",
                    description: Some("Start"),
                    bit_offset: 16,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "eopie",
                    description: Some("End of operation interrupt enable"),
                    bit_offset: 24,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "errie",
                    description: Some("Error interrupt enable"),
                    bit_offset: 25,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "lock",
                    description: Some("Lock"),
                    bit_offset: 31,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Keyr",
            extends: None,
            description: Some("Flash key register"),
            bit_size: 32,
            fields: &[Field {
                name: "key",
                description: Some("FPEC key"),
                bit_offset: 0,
                bit_size: 32,
                array: None,
                enumm: None,
            }],
        },
        FieldSet {
            name: "Optcr",
            extends: None,
            description: Some("Flash option control register"),
            bit_size: 32,
            fields: &[
                Field {
                    name: "optlock",
                    description: Some("Option lock"),
                    bit_offset: 0,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "optstrt",
                    description: Some("Option start"),
                    bit_offset: 1,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "bor_lev",
                    description: Some("BOR reset Level"),
                    bit_offset: 2,
                    bit_size: 2,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "wdg_sw",
                    description: Some("WDG_SW User option bytes"),
                    bit_offset: 5,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "n_rst_stop",
                    description: Some("nRST_STOP User option bytes"),
                    bit_offset: 6,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "n_rst_stdby",
                    description: Some("nRST_STDBY User option bytes"),
                    bit_offset: 7,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "rdp",
                    description: Some("Read protect"),
                    bit_offset: 8,
                    bit_size: 8,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "n_wrp",
                    description: Some("Not write protect"),
                    bit_offset: 16,
                    bit_size: 12,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "db1m",
                    description: Some("Dual-bank enable on 1 Mbyte Flash memory devices"),
                    bit_offset: 30,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "sprmod",
                    description: Some("Selection of protection mode for nWPRi bits"),
                    bit_offset: 31,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Optkeyr",
            extends: None,
            description: Some("Flash option key register"),
            bit_size: 32,
            fields: &[Field {
                name: "optkey",
                description: Some("Option byte key"),
                bit_offset: 0,
                bit_size: 32,
                array: None,
                enumm: None,
            }],
        },
        FieldSet {
            name: "Sr",
            extends: None,
            description: Some("Status register"),
            bit_size: 32,
            fields: &[
                Field {
                    name: "eop",
                    description: Some("End of operation"),
                    bit_offset: 0,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "operr",
                    description: Some("Operation error"),
                    bit_offset: 1,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "wrperr",
                    description: Some("Write protection error"),
                    bit_offset: 4,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "pgaerr",
                    description: Some("Programming alignment error"),
                    bit_offset: 5,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "pgperr",
                    description: Some("Programming parallelism error"),
                    bit_offset: 6,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "pgserr",
                    description: Some("Programming sequence error"),
                    bit_offset: 7,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "bsy",
                    description: Some("Busy"),
                    bit_offset: 16,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
            ],
        },
    ],
    enums: &[
        Enum {
            name: "Latency",
            description: None,
            bit_size: 3,
            variants: &[
                EnumVariant {
                    name: "WS0",
                    description: Some("0 wait states"),
                    value: 0,
                },
                EnumVariant {
                    name: "WS1",
                    description: Some("1 wait states"),
                    value: 1,
                },
                EnumVariant {
                    name: "WS2",
                    description: Some("2 wait states"),
                    value: 2,
                },
                EnumVariant {
                    name: "WS3",
                    description: Some("3 wait states"),
                    value: 3,
                },
                EnumVariant {
                    name: "WS4",
                    description: Some("4 wait states"),
                    value: 4,
                },
                EnumVariant {
                    name: "WS5",
                    description: Some("5 wait states"),
                    value: 5,
                },
                EnumVariant {
                    name: "WS6",
                    description: Some("6 wait states"),
                    value: 6,
                },
                EnumVariant {
                    name: "WS7",
                    description: Some("7 wait states"),
                    value: 7,
                },
                EnumVariant {
                    name: "WS8",
                    description: Some("8 wait states"),
                    value: 8,
                },
                EnumVariant {
                    name: "WS9",
                    description: Some("9 wait states"),
                    value: 9,
                },
                EnumVariant {
                    name: "WS10",
                    description: Some("10 wait states"),
                    value: 10,
                },
                EnumVariant {
                    name: "WS11",
                    description: Some("11 wait states"),
                    value: 11,
                },
                EnumVariant {
                    name: "WS12",
                    description: Some("12 wait states"),
                    value: 12,
                },
                EnumVariant {
                    name: "WS13",
                    description: Some("13 wait states"),
                    value: 13,
                },
                EnumVariant {
                    name: "WS14",
                    description: Some("14 wait states"),
                    value: 14,
                },
                EnumVariant {
                    name: "WS15",
                    description: Some("15 wait states"),
                    value: 15,
                },
            ],
        },
        Enum {
            name: "Psize",
            description: None,
            bit_size: 2,
            variants: &[
                EnumVariant {
                    name: "PSIZE8",
                    description: Some("Program x8"),
                    value: 0,
                },
                EnumVariant {
                    name: "PSIZE16",
                    description: Some("Program x16"),
                    value: 1,
                },
                EnumVariant {
                    name: "PSIZE32",
                    description: Some("Program x32"),
                    value: 2,
                },
                EnumVariant {
                    name: "PSIZE64",
                    description: Some("Program x64"),
                    value: 3,
                },
            ],
        },
    ],
};

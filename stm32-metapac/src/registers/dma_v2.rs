
use crate::metadata::ir::*;
pub(crate) static REGISTERS: IR = IR {
    blocks: &[
        Block {
            name: "Dma",
            extends: None,
            description: Some("DMA controller"),
            items: &[
                BlockItem {
                    name: "isr",
                    description: Some("low interrupt status register"),
                    array: Some(Array::Regular(RegularArray { len: 2, stride: 4 })),
                    byte_offset: 0,
                    inner: BlockItemInner::Register(Register {
                        access: Access::Read,
                        bit_size: 32,
                        fieldset: Some("Ixr"),
                    }),
                },
                BlockItem {
                    name: "ifcr",
                    description: Some("low interrupt flag clear register"),
                    array: Some(Array::Regular(RegularArray { len: 2, stride: 4 })),
                    byte_offset: 8,
                    inner: BlockItemInner::Register(Register {
                        access: Access::Write,
                        bit_size: 32,
                        fieldset: Some("Ixr"),
                    }),
                },
                BlockItem {
                    name: "st",
                    description: Some("Stream cluster: S?CR, S?NDTR, S?M0AR, S?M1AR and S?FCR registers"),
                    array: Some(Array::Regular(RegularArray { len: 8, stride: 24 })),
                    byte_offset: 16,
                    inner: BlockItemInner::Block(BlockItemBlock { block: "St" }),
                },
            ],
        },
        Block {
            name: "St",
            extends: None,
            description: Some("Stream cluster: S?CR, S?NDTR, S?M0AR, S?M1AR and S?FCR registers"),
            items: &[
                BlockItem {
                    name: "cr",
                    description: Some("stream x configuration register"),
                    array: None,
                    byte_offset: 0,
                    inner: BlockItemInner::Register(Register {
                        access: Access::ReadWrite,
                        bit_size: 32,
                        fieldset: Some("Cr"),
                    }),
                },
                BlockItem {
                    name: "ndtr",
                    description: Some("stream x number of data register"),
                    array: None,
                    byte_offset: 4,
                    inner: BlockItemInner::Register(Register {
                        access: Access::ReadWrite,
                        bit_size: 32,
                        fieldset: Some("Ndtr"),
                    }),
                },
                BlockItem {
                    name: "par",
                    description: Some("stream x peripheral address register"),
                    array: None,
                    byte_offset: 8,
                    inner: BlockItemInner::Register(Register {
                        access: Access::ReadWrite,
                        bit_size: 32,
                        fieldset: None,
                    }),
                },
                BlockItem {
                    name: "m0ar",
                    description: Some("stream x memory 0 address register"),
                    array: None,
                    byte_offset: 12,
                    inner: BlockItemInner::Register(Register {
                        access: Access::ReadWrite,
                        bit_size: 32,
                        fieldset: None,
                    }),
                },
                BlockItem {
                    name: "m1ar",
                    description: Some("stream x memory 1 address register"),
                    array: None,
                    byte_offset: 16,
                    inner: BlockItemInner::Register(Register {
                        access: Access::ReadWrite,
                        bit_size: 32,
                        fieldset: None,
                    }),
                },
                BlockItem {
                    name: "fcr",
                    description: Some("stream x FIFO control register"),
                    array: None,
                    byte_offset: 20,
                    inner: BlockItemInner::Register(Register {
                        access: Access::ReadWrite,
                        bit_size: 32,
                        fieldset: Some("Fcr"),
                    }),
                },
            ],
        },
    ],
    fieldsets: &[
        FieldSet {
            name: "Cr",
            extends: None,
            description: Some("stream x configuration register"),
            bit_size: 32,
            fields: &[
                Field {
                    name: "en",
                    description: Some("Stream enable / flag stream ready when read low"),
                    bit_offset: 0,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "dmeie",
                    description: Some("Direct mode error interrupt enable"),
                    bit_offset: 1,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "teie",
                    description: Some("Transfer error interrupt enable"),
                    bit_offset: 2,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "htie",
                    description: Some("Half transfer interrupt enable"),
                    bit_offset: 3,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "tcie",
                    description: Some("Transfer complete interrupt enable"),
                    bit_offset: 4,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "pfctrl",
                    description: Some("Peripheral flow controller"),
                    bit_offset: 5,
                    bit_size: 1,
                    array: None,
                    enumm: Some("Pfctrl"),
                },
                Field {
                    name: "dir",
                    description: Some("Data transfer direction"),
                    bit_offset: 6,
                    bit_size: 2,
                    array: None,
                    enumm: Some("Dir"),
                },
                Field {
                    name: "circ",
                    description: Some("Circular mode"),
                    bit_offset: 8,
                    bit_size: 1,
                    array: None,
                    enumm: Some("Circ"),
                },
                Field {
                    name: "pinc",
                    description: Some("Peripheral increment mode"),
                    bit_offset: 9,
                    bit_size: 1,
                    array: None,
                    enumm: Some("Inc"),
                },
                Field {
                    name: "minc",
                    description: Some("Memory increment mode"),
                    bit_offset: 10,
                    bit_size: 1,
                    array: None,
                    enumm: Some("Inc"),
                },
                Field {
                    name: "psize",
                    description: Some("Peripheral data size"),
                    bit_offset: 11,
                    bit_size: 2,
                    array: None,
                    enumm: Some("Size"),
                },
                Field {
                    name: "msize",
                    description: Some("Memory data size"),
                    bit_offset: 13,
                    bit_size: 2,
                    array: None,
                    enumm: Some("Size"),
                },
                Field {
                    name: "pincos",
                    description: Some("Peripheral increment offset size"),
                    bit_offset: 15,
                    bit_size: 1,
                    array: None,
                    enumm: Some("Pincos"),
                },
                Field {
                    name: "pl",
                    description: Some("Priority level"),
                    bit_offset: 16,
                    bit_size: 2,
                    array: None,
                    enumm: Some("Pl"),
                },
                Field {
                    name: "dbm",
                    description: Some("Double buffer mode"),
                    bit_offset: 18,
                    bit_size: 1,
                    array: None,
                    enumm: Some("Dbm"),
                },
                Field {
                    name: "ct",
                    description: Some("Current target (only in double buffer mode)"),
                    bit_offset: 19,
                    bit_size: 1,
                    array: None,
                    enumm: Some("Ct"),
                },
                Field {
                    name: "pburst",
                    description: Some("Peripheral burst transfer configuration"),
                    bit_offset: 21,
                    bit_size: 2,
                    array: None,
                    enumm: Some("Burst"),
                },
                Field {
                    name: "mburst",
                    description: Some("Memory burst transfer configuration"),
                    bit_offset: 23,
                    bit_size: 2,
                    array: None,
                    enumm: Some("Burst"),
                },
                Field {
                    name: "chsel",
                    description: Some("Channel selection"),
                    bit_offset: 25,
                    bit_size: 4,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Fcr",
            extends: None,
            description: Some("stream x FIFO control register"),
            bit_size: 32,
            fields: &[
                Field {
                    name: "fth",
                    description: Some("FIFO threshold selection"),
                    bit_offset: 0,
                    bit_size: 2,
                    array: None,
                    enumm: Some("Fth"),
                },
                Field {
                    name: "dmdis",
                    description: Some("Direct mode disable"),
                    bit_offset: 2,
                    bit_size: 1,
                    array: None,
                    enumm: Some("Dmdis"),
                },
                Field {
                    name: "fs",
                    description: Some("FIFO status"),
                    bit_offset: 3,
                    bit_size: 3,
                    array: None,
                    enumm: Some("Fs"),
                },
                Field {
                    name: "feie",
                    description: Some("FIFO error interrupt enable"),
                    bit_offset: 7,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Ixr",
            extends: None,
            description: Some("interrupt register"),
            bit_size: 32,
            fields: &[
                Field {
                    name: "feif",
                    description: Some("Stream x FIFO error interrupt flag (x=3..0)"),
                    bit_offset: 0,
                    bit_size: 1,
                    array: Some(Array::Cursed(CursedArray {
                        offsets: &[0, 6, 16, 22],
                    })),
                    enumm: None,
                },
                Field {
                    name: "dmeif",
                    description: Some("Stream x direct mode error interrupt flag (x=3..0)"),
                    bit_offset: 2,
                    bit_size: 1,
                    array: Some(Array::Cursed(CursedArray {
                        offsets: &[0, 6, 16, 22],
                    })),
                    enumm: None,
                },
                Field {
                    name: "teif",
                    description: Some("Stream x transfer error interrupt flag (x=3..0)"),
                    bit_offset: 3,
                    bit_size: 1,
                    array: Some(Array::Cursed(CursedArray {
                        offsets: &[0, 6, 16, 22],
                    })),
                    enumm: None,
                },
                Field {
                    name: "htif",
                    description: Some("Stream x half transfer interrupt flag (x=3..0)"),
                    bit_offset: 4,
                    bit_size: 1,
                    array: Some(Array::Cursed(CursedArray {
                        offsets: &[0, 6, 16, 22],
                    })),
                    enumm: None,
                },
                Field {
                    name: "tcif",
                    description: Some("Stream x transfer complete interrupt flag (x = 3..0)"),
                    bit_offset: 5,
                    bit_size: 1,
                    array: Some(Array::Cursed(CursedArray {
                        offsets: &[0, 6, 16, 22],
                    })),
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Ndtr",
            extends: None,
            description: Some("stream x number of data register"),
            bit_size: 32,
            fields: &[Field {
                name: "ndt",
                description: Some("Number of data items to transfer"),
                bit_offset: 0,
                bit_size: 16,
                array: None,
                enumm: None,
            }],
        },
    ],
    enums: &[
        Enum {
            name: "Burst",
            description: None,
            bit_size: 2,
            variants: &[
                EnumVariant {
                    name: "SINGLE",
                    description: Some("Single transfer"),
                    value: 0,
                },
                EnumVariant {
                    name: "INCR4",
                    description: Some("Incremental burst of 4 beats"),
                    value: 1,
                },
                EnumVariant {
                    name: "INCR8",
                    description: Some("Incremental burst of 8 beats"),
                    value: 2,
                },
                EnumVariant {
                    name: "INCR16",
                    description: Some("Incremental burst of 16 beats"),
                    value: 3,
                },
            ],
        },
        Enum {
            name: "Circ",
            description: None,
            bit_size: 1,
            variants: &[
                EnumVariant {
                    name: "DISABLED",
                    description: Some("Circular mode disabled"),
                    value: 0,
                },
                EnumVariant {
                    name: "ENABLED",
                    description: Some("Circular mode enabled"),
                    value: 1,
                },
            ],
        },
        Enum {
            name: "Ct",
            description: None,
            bit_size: 1,
            variants: &[
                EnumVariant {
                    name: "MEMORY0",
                    description: Some("The current target memory is Memory 0"),
                    value: 0,
                },
                EnumVariant {
                    name: "MEMORY1",
                    description: Some("The current target memory is Memory 1"),
                    value: 1,
                },
            ],
        },
        Enum {
            name: "Dbm",
            description: None,
            bit_size: 1,
            variants: &[
                EnumVariant {
                    name: "DISABLED",
                    description: Some("No buffer switching at the end of transfer"),
                    value: 0,
                },
                EnumVariant {
                    name: "ENABLED",
                    description: Some("Memory target switched at the end of the DMA transfer"),
                    value: 1,
                },
            ],
        },
        Enum {
            name: "Dir",
            description: None,
            bit_size: 2,
            variants: &[
                EnumVariant {
                    name: "PERIPHERALTOMEMORY",
                    description: Some("Peripheral-to-memory"),
                    value: 0,
                },
                EnumVariant {
                    name: "MEMORYTOPERIPHERAL",
                    description: Some("Memory-to-peripheral"),
                    value: 1,
                },
                EnumVariant {
                    name: "MEMORYTOMEMORY",
                    description: Some("Memory-to-memory"),
                    value: 2,
                },
            ],
        },
        Enum {
            name: "Dmdis",
            description: None,
            bit_size: 1,
            variants: &[
                EnumVariant {
                    name: "ENABLED",
                    description: Some("Direct mode is enabled"),
                    value: 0,
                },
                EnumVariant {
                    name: "DISABLED",
                    description: Some("Direct mode is disabled"),
                    value: 1,
                },
            ],
        },
        Enum {
            name: "Fs",
            description: None,
            bit_size: 3,
            variants: &[
                EnumVariant {
                    name: "QUARTER1",
                    description: Some("0 < fifo_level < 1/4"),
                    value: 0,
                },
                EnumVariant {
                    name: "QUARTER2",
                    description: Some("1/4 <= fifo_level < 1/2"),
                    value: 1,
                },
                EnumVariant {
                    name: "QUARTER3",
                    description: Some("1/2 <= fifo_level < 3/4"),
                    value: 2,
                },
                EnumVariant {
                    name: "QUARTER4",
                    description: Some("3/4 <= fifo_level < full"),
                    value: 3,
                },
                EnumVariant {
                    name: "EMPTY",
                    description: Some("FIFO is empty"),
                    value: 4,
                },
                EnumVariant {
                    name: "FULL",
                    description: Some("FIFO is full"),
                    value: 5,
                },
            ],
        },
        Enum {
            name: "Fth",
            description: None,
            bit_size: 2,
            variants: &[
                EnumVariant {
                    name: "QUARTER",
                    description: Some("1/4 full FIFO"),
                    value: 0,
                },
                EnumVariant {
                    name: "HALF",
                    description: Some("1/2 full FIFO"),
                    value: 1,
                },
                EnumVariant {
                    name: "THREEQUARTERS",
                    description: Some("3/4 full FIFO"),
                    value: 2,
                },
                EnumVariant {
                    name: "FULL",
                    description: Some("Full FIFO"),
                    value: 3,
                },
            ],
        },
        Enum {
            name: "Inc",
            description: None,
            bit_size: 1,
            variants: &[
                EnumVariant {
                    name: "FIXED",
                    description: Some("Address pointer is fixed"),
                    value: 0,
                },
                EnumVariant {
                    name: "INCREMENTED",
                    description: Some("Address pointer is incremented after each data transfer"),
                    value: 1,
                },
            ],
        },
        Enum {
            name: "Pfctrl",
            description: None,
            bit_size: 1,
            variants: &[
                EnumVariant {
                    name: "DMA",
                    description: Some("The DMA is the flow controller"),
                    value: 0,
                },
                EnumVariant {
                    name: "PERIPHERAL",
                    description: Some("The peripheral is the flow controller"),
                    value: 1,
                },
            ],
        },
        Enum {
            name: "Pincos",
            description: None,
            bit_size: 1,
            variants: &[
                EnumVariant {
                    name: "PSIZE",
                    description: Some("The offset size for the peripheral address calculation is linked to the PSIZE"),
                    value: 0,
                },
                EnumVariant {
                    name: "FIXED4",
                    description: Some(
                        "The offset size for the peripheral address calculation is fixed to 4 (32-bit alignment)",
                    ),
                    value: 1,
                },
            ],
        },
        Enum {
            name: "Pl",
            description: None,
            bit_size: 2,
            variants: &[
                EnumVariant {
                    name: "LOW",
                    description: Some("Low"),
                    value: 0,
                },
                EnumVariant {
                    name: "MEDIUM",
                    description: Some("Medium"),
                    value: 1,
                },
                EnumVariant {
                    name: "HIGH",
                    description: Some("High"),
                    value: 2,
                },
                EnumVariant {
                    name: "VERYHIGH",
                    description: Some("Very high"),
                    value: 3,
                },
            ],
        },
        Enum {
            name: "Size",
            description: None,
            bit_size: 2,
            variants: &[
                EnumVariant {
                    name: "BITS8",
                    description: Some("Byte (8-bit)"),
                    value: 0,
                },
                EnumVariant {
                    name: "BITS16",
                    description: Some("Half-word (16-bit)"),
                    value: 1,
                },
                EnumVariant {
                    name: "BITS32",
                    description: Some("Word (32-bit)"),
                    value: 2,
                },
            ],
        },
    ],
};

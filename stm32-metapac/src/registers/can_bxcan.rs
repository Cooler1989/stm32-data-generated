
use crate::metadata::ir::*;
pub(crate) static REGISTERS: IR = IR {
    blocks: &[
        Block {
            name: "Can",
            extends: None,
            description: Some(
                "Controller area network",
            ),
            items: &[
                BlockItem {
                    name: "mcr",
                    description: Some(
                        "master control register",
                    ),
                    array: None,
                    byte_offset: 0,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Mcr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "msr",
                    description: Some(
                        "master status register",
                    ),
                    array: None,
                    byte_offset: 4,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Msr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "tsr",
                    description: Some(
                        "transmit status register",
                    ),
                    array: None,
                    byte_offset: 8,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Tsr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "rfr",
                    description: Some(
                        "receive FIFO 0 register",
                    ),
                    array: Some(
                        Array::Regular(
                            RegularArray {
                                len: 2,
                                stride: 4,
                            },
                        ),
                    ),
                    byte_offset: 12,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Rfr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "ier",
                    description: Some(
                        "interrupt enable register",
                    ),
                    array: None,
                    byte_offset: 20,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Ier",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "esr",
                    description: Some(
                        "error status register",
                    ),
                    array: None,
                    byte_offset: 24,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Esr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "btr",
                    description: Some(
                        "bit timing register",
                    ),
                    array: None,
                    byte_offset: 28,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Btr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "tx",
                    description: Some(
                        "CAN Transmit cluster",
                    ),
                    array: Some(
                        Array::Regular(
                            RegularArray {
                                len: 3,
                                stride: 16,
                            },
                        ),
                    ),
                    byte_offset: 384,
                    inner: BlockItemInner::Block(
                        BlockItemBlock {
                            block: "Tx",
                        },
                    ),
                },
                BlockItem {
                    name: "rx",
                    description: Some(
                        "CAN Receive cluster",
                    ),
                    array: Some(
                        Array::Regular(
                            RegularArray {
                                len: 2,
                                stride: 16,
                            },
                        ),
                    ),
                    byte_offset: 432,
                    inner: BlockItemInner::Block(
                        BlockItemBlock {
                            block: "Rx",
                        },
                    ),
                },
                BlockItem {
                    name: "fmr",
                    description: Some(
                        "filter master register",
                    ),
                    array: None,
                    byte_offset: 512,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Fmr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "fm1r",
                    description: Some(
                        "filter mode register",
                    ),
                    array: None,
                    byte_offset: 516,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Fm1r",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "fs1r",
                    description: Some(
                        "filter scale register",
                    ),
                    array: None,
                    byte_offset: 524,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Fs1r",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "ffa1r",
                    description: Some(
                        "filter FIFO assignment register",
                    ),
                    array: None,
                    byte_offset: 532,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Ffa1r",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "fa1r",
                    description: Some(
                        "filter activation register",
                    ),
                    array: None,
                    byte_offset: 540,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Fa1r",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "fb",
                    description: Some(
                        "CAN Filter Bank cluster",
                    ),
                    array: Some(
                        Array::Regular(
                            RegularArray {
                                len: 28,
                                stride: 8,
                            },
                        ),
                    ),
                    byte_offset: 576,
                    inner: BlockItemInner::Block(
                        BlockItemBlock {
                            block: "Fb",
                        },
                    ),
                },
            ],
        },
        Block {
            name: "Fb",
            extends: None,
            description: Some(
                "CAN Filter Bank cluster",
            ),
            items: &[
                BlockItem {
                    name: "fr1",
                    description: Some(
                        "Filter bank 0 register 1",
                    ),
                    array: None,
                    byte_offset: 0,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Fr1",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "fr2",
                    description: Some(
                        "Filter bank 0 register 2",
                    ),
                    array: None,
                    byte_offset: 4,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Fr2",
                            ),
                        },
                    ),
                },
            ],
        },
        Block {
            name: "Rx",
            extends: None,
            description: Some(
                "CAN Receive cluster",
            ),
            items: &[
                BlockItem {
                    name: "rir",
                    description: Some(
                        "receive FIFO mailbox identifier register",
                    ),
                    array: None,
                    byte_offset: 0,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::Read,
                            bit_size: 32,
                            fieldset: Some(
                                "Rir",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "rdtr",
                    description: Some(
                        "mailbox data high register",
                    ),
                    array: None,
                    byte_offset: 4,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::Read,
                            bit_size: 32,
                            fieldset: Some(
                                "Rdtr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "rdlr",
                    description: Some(
                        "mailbox data high register",
                    ),
                    array: None,
                    byte_offset: 8,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::Read,
                            bit_size: 32,
                            fieldset: Some(
                                "Rdlr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "rdhr",
                    description: Some(
                        "receive FIFO mailbox data high register",
                    ),
                    array: None,
                    byte_offset: 12,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::Read,
                            bit_size: 32,
                            fieldset: Some(
                                "Rdhr",
                            ),
                        },
                    ),
                },
            ],
        },
        Block {
            name: "Tx",
            extends: None,
            description: Some(
                "CAN Transmit cluster",
            ),
            items: &[
                BlockItem {
                    name: "tir",
                    description: Some(
                        "TX mailbox identifier register",
                    ),
                    array: None,
                    byte_offset: 0,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Tir",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "tdtr",
                    description: Some(
                        "mailbox data length control and time stamp register",
                    ),
                    array: None,
                    byte_offset: 4,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Tdtr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "tdlr",
                    description: Some(
                        "mailbox data low register",
                    ),
                    array: None,
                    byte_offset: 8,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Tdlr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "tdhr",
                    description: Some(
                        "mailbox data high register",
                    ),
                    array: None,
                    byte_offset: 12,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Tdhr",
                            ),
                        },
                    ),
                },
            ],
        },
    ],
    fieldsets: &[
        FieldSet {
            name: "Btr",
            extends: None,
            description: Some(
                "bit timing register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "brp",
                    description: Some(
                        "BRP",
                    ),
                    bit_offset: 0,
                    bit_size: 10,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "ts",
                    description: Some(
                        "TS1",
                    ),
                    bit_offset: 16,
                    bit_size: 4,
                    array: Some(
                        Array::Regular(
                            RegularArray {
                                len: 2,
                                stride: 4,
                            },
                        ),
                    ),
                    enumm: None,
                },
                Field {
                    name: "sjw",
                    description: Some(
                        "SJW",
                    ),
                    bit_offset: 24,
                    bit_size: 2,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "lbkm",
                    description: Some(
                        "LBKM",
                    ),
                    bit_offset: 30,
                    bit_size: 1,
                    array: None,
                    enumm: Some(
                        "Lbkm",
                    ),
                },
                Field {
                    name: "silm",
                    description: Some(
                        "SILM",
                    ),
                    bit_offset: 31,
                    bit_size: 1,
                    array: None,
                    enumm: Some(
                        "Silm",
                    ),
                },
            ],
        },
        FieldSet {
            name: "Esr",
            extends: None,
            description: Some(
                "interrupt enable register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "ewgf",
                    description: Some(
                        "EWGF",
                    ),
                    bit_offset: 0,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "epvf",
                    description: Some(
                        "EPVF",
                    ),
                    bit_offset: 1,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "boff",
                    description: Some(
                        "BOFF",
                    ),
                    bit_offset: 2,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "lec",
                    description: Some(
                        "LEC",
                    ),
                    bit_offset: 4,
                    bit_size: 3,
                    array: None,
                    enumm: Some(
                        "Lec",
                    ),
                },
                Field {
                    name: "tec",
                    description: Some(
                        "TEC",
                    ),
                    bit_offset: 16,
                    bit_size: 8,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "rec",
                    description: Some(
                        "REC",
                    ),
                    bit_offset: 24,
                    bit_size: 8,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Fa1r",
            extends: None,
            description: Some(
                "filter activation register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "fact",
                    description: Some(
                        "Filter active",
                    ),
                    bit_offset: 0,
                    bit_size: 1,
                    array: Some(
                        Array::Regular(
                            RegularArray {
                                len: 28,
                                stride: 1,
                            },
                        ),
                    ),
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Ffa1r",
            extends: None,
            description: Some(
                "filter FIFO assignment register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "ffa",
                    description: Some(
                        "Filter FIFO assignment for filter 0",
                    ),
                    bit_offset: 0,
                    bit_size: 1,
                    array: Some(
                        Array::Regular(
                            RegularArray {
                                len: 28,
                                stride: 1,
                            },
                        ),
                    ),
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Fm1r",
            extends: None,
            description: Some(
                "filter mode register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "fbm",
                    description: Some(
                        "Filter mode",
                    ),
                    bit_offset: 0,
                    bit_size: 1,
                    array: Some(
                        Array::Regular(
                            RegularArray {
                                len: 28,
                                stride: 1,
                            },
                        ),
                    ),
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Fmr",
            extends: None,
            description: Some(
                "filter master register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "finit",
                    description: Some(
                        "FINIT",
                    ),
                    bit_offset: 0,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "can2sb",
                    description: Some(
                        "CAN2SB",
                    ),
                    bit_offset: 8,
                    bit_size: 6,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Fr1",
            extends: None,
            description: Some(
                "Filter bank 0 register 1",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "fb",
                    description: Some(
                        "Filter bits",
                    ),
                    bit_offset: 0,
                    bit_size: 1,
                    array: Some(
                        Array::Regular(
                            RegularArray {
                                len: 32,
                                stride: 1,
                            },
                        ),
                    ),
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Fr2",
            extends: None,
            description: Some(
                "Filter bank 0 register 2",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "fb",
                    description: Some(
                        "Filter bits",
                    ),
                    bit_offset: 0,
                    bit_size: 1,
                    array: Some(
                        Array::Regular(
                            RegularArray {
                                len: 32,
                                stride: 1,
                            },
                        ),
                    ),
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Fs1r",
            extends: None,
            description: Some(
                "filter scale register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "fsc",
                    description: Some(
                        "Filter scale configuration",
                    ),
                    bit_offset: 0,
                    bit_size: 1,
                    array: Some(
                        Array::Regular(
                            RegularArray {
                                len: 28,
                                stride: 1,
                            },
                        ),
                    ),
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Ier",
            extends: None,
            description: Some(
                "interrupt enable register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "tmeie",
                    description: Some(
                        "TMEIE",
                    ),
                    bit_offset: 0,
                    bit_size: 1,
                    array: None,
                    enumm: Some(
                        "Tmeie",
                    ),
                },
                Field {
                    name: "fmpie",
                    description: Some(
                        "FMPIE0",
                    ),
                    bit_offset: 1,
                    bit_size: 1,
                    array: Some(
                        Array::Regular(
                            RegularArray {
                                len: 2,
                                stride: 3,
                            },
                        ),
                    ),
                    enumm: Some(
                        "Fmpie",
                    ),
                },
                Field {
                    name: "ffie",
                    description: Some(
                        "FFIE0",
                    ),
                    bit_offset: 2,
                    bit_size: 1,
                    array: Some(
                        Array::Regular(
                            RegularArray {
                                len: 2,
                                stride: 3,
                            },
                        ),
                    ),
                    enumm: Some(
                        "Ffie",
                    ),
                },
                Field {
                    name: "fovie",
                    description: Some(
                        "FOVIE0",
                    ),
                    bit_offset: 3,
                    bit_size: 1,
                    array: Some(
                        Array::Regular(
                            RegularArray {
                                len: 2,
                                stride: 3,
                            },
                        ),
                    ),
                    enumm: Some(
                        "Fovie",
                    ),
                },
                Field {
                    name: "ewgie",
                    description: Some(
                        "EWGIE",
                    ),
                    bit_offset: 8,
                    bit_size: 1,
                    array: None,
                    enumm: Some(
                        "Ewgie",
                    ),
                },
                Field {
                    name: "epvie",
                    description: Some(
                        "EPVIE",
                    ),
                    bit_offset: 9,
                    bit_size: 1,
                    array: None,
                    enumm: Some(
                        "Epvie",
                    ),
                },
                Field {
                    name: "bofie",
                    description: Some(
                        "BOFIE",
                    ),
                    bit_offset: 10,
                    bit_size: 1,
                    array: None,
                    enumm: Some(
                        "Bofie",
                    ),
                },
                Field {
                    name: "lecie",
                    description: Some(
                        "LECIE",
                    ),
                    bit_offset: 11,
                    bit_size: 1,
                    array: None,
                    enumm: Some(
                        "Lecie",
                    ),
                },
                Field {
                    name: "errie",
                    description: Some(
                        "ERRIE",
                    ),
                    bit_offset: 15,
                    bit_size: 1,
                    array: None,
                    enumm: Some(
                        "Errie",
                    ),
                },
                Field {
                    name: "wkuie",
                    description: Some(
                        "WKUIE",
                    ),
                    bit_offset: 16,
                    bit_size: 1,
                    array: None,
                    enumm: Some(
                        "Wkuie",
                    ),
                },
                Field {
                    name: "slkie",
                    description: Some(
                        "SLKIE",
                    ),
                    bit_offset: 17,
                    bit_size: 1,
                    array: None,
                    enumm: Some(
                        "Slkie",
                    ),
                },
            ],
        },
        FieldSet {
            name: "Mcr",
            extends: None,
            description: Some(
                "master control register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "inrq",
                    description: Some(
                        "INRQ",
                    ),
                    bit_offset: 0,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "sleep",
                    description: Some(
                        "SLEEP",
                    ),
                    bit_offset: 1,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "txfp",
                    description: Some(
                        "TXFP",
                    ),
                    bit_offset: 2,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "rflm",
                    description: Some(
                        "RFLM",
                    ),
                    bit_offset: 3,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "nart",
                    description: Some(
                        "NART",
                    ),
                    bit_offset: 4,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "awum",
                    description: Some(
                        "AWUM",
                    ),
                    bit_offset: 5,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "abom",
                    description: Some(
                        "ABOM",
                    ),
                    bit_offset: 6,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "ttcm",
                    description: Some(
                        "TTCM",
                    ),
                    bit_offset: 7,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "reset",
                    description: Some(
                        "RESET",
                    ),
                    bit_offset: 15,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "dbf",
                    description: Some(
                        "DBF",
                    ),
                    bit_offset: 16,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Msr",
            extends: None,
            description: Some(
                "master status register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "inak",
                    description: Some(
                        "INAK",
                    ),
                    bit_offset: 0,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "slak",
                    description: Some(
                        "SLAK",
                    ),
                    bit_offset: 1,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "erri",
                    description: Some(
                        "ERRI",
                    ),
                    bit_offset: 2,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "wkui",
                    description: Some(
                        "WKUI",
                    ),
                    bit_offset: 3,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "slaki",
                    description: Some(
                        "SLAKI",
                    ),
                    bit_offset: 4,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "txm",
                    description: Some(
                        "TXM",
                    ),
                    bit_offset: 8,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "rxm",
                    description: Some(
                        "RXM",
                    ),
                    bit_offset: 9,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "samp",
                    description: Some(
                        "SAMP",
                    ),
                    bit_offset: 10,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "rx",
                    description: Some(
                        "RX",
                    ),
                    bit_offset: 11,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Rdhr",
            extends: None,
            description: Some(
                "receive FIFO mailbox data high register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "data",
                    description: Some(
                        "DATA4",
                    ),
                    bit_offset: 0,
                    bit_size: 8,
                    array: Some(
                        Array::Regular(
                            RegularArray {
                                len: 4,
                                stride: 8,
                            },
                        ),
                    ),
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Rdlr",
            extends: None,
            description: Some(
                "mailbox data high register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "data",
                    description: Some(
                        "DATA0",
                    ),
                    bit_offset: 0,
                    bit_size: 8,
                    array: Some(
                        Array::Regular(
                            RegularArray {
                                len: 4,
                                stride: 8,
                            },
                        ),
                    ),
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Rdtr",
            extends: None,
            description: Some(
                "mailbox data high register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "dlc",
                    description: Some(
                        "DLC",
                    ),
                    bit_offset: 0,
                    bit_size: 4,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "fmi",
                    description: Some(
                        "FMI",
                    ),
                    bit_offset: 8,
                    bit_size: 8,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "time",
                    description: Some(
                        "TIME",
                    ),
                    bit_offset: 16,
                    bit_size: 16,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Rfr",
            extends: None,
            description: Some(
                "receive FIFO 0 register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "fmp",
                    description: Some(
                        "FMP0",
                    ),
                    bit_offset: 0,
                    bit_size: 2,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "full",
                    description: Some(
                        "FULL0",
                    ),
                    bit_offset: 3,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "fovr",
                    description: Some(
                        "FOVR0",
                    ),
                    bit_offset: 4,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "rfom",
                    description: Some(
                        "RFOM0",
                    ),
                    bit_offset: 5,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Rir",
            extends: None,
            description: Some(
                "receive FIFO mailbox identifier register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "rtr",
                    description: Some(
                        "RTR",
                    ),
                    bit_offset: 1,
                    bit_size: 1,
                    array: None,
                    enumm: Some(
                        "RirRtr",
                    ),
                },
                Field {
                    name: "ide",
                    description: Some(
                        "IDE",
                    ),
                    bit_offset: 2,
                    bit_size: 1,
                    array: None,
                    enumm: Some(
                        "RirIde",
                    ),
                },
                Field {
                    name: "exid",
                    description: Some(
                        "EXID",
                    ),
                    bit_offset: 3,
                    bit_size: 18,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "stid",
                    description: Some(
                        "STID",
                    ),
                    bit_offset: 21,
                    bit_size: 11,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Tdhr",
            extends: None,
            description: Some(
                "mailbox data high register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "data",
                    description: Some(
                        "DATA4",
                    ),
                    bit_offset: 0,
                    bit_size: 8,
                    array: Some(
                        Array::Regular(
                            RegularArray {
                                len: 4,
                                stride: 8,
                            },
                        ),
                    ),
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Tdlr",
            extends: None,
            description: Some(
                "mailbox data low register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "data",
                    description: Some(
                        "DATA0",
                    ),
                    bit_offset: 0,
                    bit_size: 8,
                    array: Some(
                        Array::Regular(
                            RegularArray {
                                len: 4,
                                stride: 8,
                            },
                        ),
                    ),
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Tdtr",
            extends: None,
            description: Some(
                "mailbox data length control and time stamp register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "dlc",
                    description: Some(
                        "DLC",
                    ),
                    bit_offset: 0,
                    bit_size: 4,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "tgt",
                    description: Some(
                        "TGT",
                    ),
                    bit_offset: 8,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "time",
                    description: Some(
                        "TIME",
                    ),
                    bit_offset: 16,
                    bit_size: 16,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Tir",
            extends: None,
            description: Some(
                "TX mailbox identifier register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "txrq",
                    description: Some(
                        "TXRQ",
                    ),
                    bit_offset: 0,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "rtr",
                    description: Some(
                        "RTR",
                    ),
                    bit_offset: 1,
                    bit_size: 1,
                    array: None,
                    enumm: Some(
                        "TirRtr",
                    ),
                },
                Field {
                    name: "ide",
                    description: Some(
                        "IDE",
                    ),
                    bit_offset: 2,
                    bit_size: 1,
                    array: None,
                    enumm: Some(
                        "TirIde",
                    ),
                },
                Field {
                    name: "exid",
                    description: Some(
                        "EXID",
                    ),
                    bit_offset: 3,
                    bit_size: 18,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "stid",
                    description: Some(
                        "STID",
                    ),
                    bit_offset: 21,
                    bit_size: 11,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Tsr",
            extends: None,
            description: Some(
                "transmit status register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "rqcp",
                    description: Some(
                        "RQCP0",
                    ),
                    bit_offset: 0,
                    bit_size: 1,
                    array: Some(
                        Array::Regular(
                            RegularArray {
                                len: 3,
                                stride: 8,
                            },
                        ),
                    ),
                    enumm: None,
                },
                Field {
                    name: "txok",
                    description: Some(
                        "TXOK0",
                    ),
                    bit_offset: 1,
                    bit_size: 1,
                    array: Some(
                        Array::Regular(
                            RegularArray {
                                len: 3,
                                stride: 8,
                            },
                        ),
                    ),
                    enumm: None,
                },
                Field {
                    name: "alst",
                    description: Some(
                        "ALST0",
                    ),
                    bit_offset: 2,
                    bit_size: 1,
                    array: Some(
                        Array::Regular(
                            RegularArray {
                                len: 3,
                                stride: 8,
                            },
                        ),
                    ),
                    enumm: None,
                },
                Field {
                    name: "terr",
                    description: Some(
                        "TERR0",
                    ),
                    bit_offset: 3,
                    bit_size: 1,
                    array: Some(
                        Array::Regular(
                            RegularArray {
                                len: 3,
                                stride: 8,
                            },
                        ),
                    ),
                    enumm: None,
                },
                Field {
                    name: "abrq",
                    description: Some(
                        "ABRQ0",
                    ),
                    bit_offset: 7,
                    bit_size: 1,
                    array: Some(
                        Array::Regular(
                            RegularArray {
                                len: 3,
                                stride: 8,
                            },
                        ),
                    ),
                    enumm: None,
                },
                Field {
                    name: "code",
                    description: Some(
                        "CODE",
                    ),
                    bit_offset: 24,
                    bit_size: 2,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "tme",
                    description: Some(
                        "Lowest priority flag for mailbox 0",
                    ),
                    bit_offset: 26,
                    bit_size: 1,
                    array: Some(
                        Array::Regular(
                            RegularArray {
                                len: 3,
                                stride: 1,
                            },
                        ),
                    ),
                    enumm: None,
                },
                Field {
                    name: "low",
                    description: Some(
                        "Lowest priority flag for mailbox 0",
                    ),
                    bit_offset: 29,
                    bit_size: 1,
                    array: Some(
                        Array::Regular(
                            RegularArray {
                                len: 3,
                                stride: 1,
                            },
                        ),
                    ),
                    enumm: None,
                },
            ],
        },
    ],
    enums: &[
        Enum {
            name: "Bofie",
            description: None,
            bit_size: 1,
            variants: &[
                EnumVariant {
                    name: "DISABLED",
                    description: Some(
                        "ERRI bit will not be set when BOFF is set",
                    ),
                    value: 0,
                },
                EnumVariant {
                    name: "ENABLED",
                    description: Some(
                        "ERRI bit will be set when BOFF is set",
                    ),
                    value: 1,
                },
            ],
        },
        Enum {
            name: "Epvie",
            description: None,
            bit_size: 1,
            variants: &[
                EnumVariant {
                    name: "DISABLED",
                    description: Some(
                        "ERRI bit will not be set when EPVF is set",
                    ),
                    value: 0,
                },
                EnumVariant {
                    name: "ENABLED",
                    description: Some(
                        "ERRI bit will be set when EPVF is set",
                    ),
                    value: 1,
                },
            ],
        },
        Enum {
            name: "Errie",
            description: None,
            bit_size: 1,
            variants: &[
                EnumVariant {
                    name: "DISABLED",
                    description: Some(
                        "No interrupt will be generated when an error condition is pending in the CAN_ESR",
                    ),
                    value: 0,
                },
                EnumVariant {
                    name: "ENABLED",
                    description: Some(
                        "An interrupt will be generation when an error condition is pending in the CAN_ESR",
                    ),
                    value: 1,
                },
            ],
        },
        Enum {
            name: "Ewgie",
            description: None,
            bit_size: 1,
            variants: &[
                EnumVariant {
                    name: "DISABLED",
                    description: Some(
                        "ERRI bit will not be set when EWGF is set",
                    ),
                    value: 0,
                },
                EnumVariant {
                    name: "ENABLED",
                    description: Some(
                        "ERRI bit will be set when EWGF is set",
                    ),
                    value: 1,
                },
            ],
        },
        Enum {
            name: "Ffie",
            description: None,
            bit_size: 1,
            variants: &[
                EnumVariant {
                    name: "DISABLED",
                    description: Some(
                        "No interrupt when FULL bit is set",
                    ),
                    value: 0,
                },
                EnumVariant {
                    name: "ENABLED",
                    description: Some(
                        "Interrupt generated when FULL bit is set",
                    ),
                    value: 1,
                },
            ],
        },
        Enum {
            name: "Fmpie",
            description: None,
            bit_size: 1,
            variants: &[
                EnumVariant {
                    name: "DISABLED",
                    description: Some(
                        "No interrupt generated when state of FMP[1:0] bits are not 00b",
                    ),
                    value: 0,
                },
                EnumVariant {
                    name: "ENABLED",
                    description: Some(
                        "Interrupt generated when state of FMP[1:0] bits are not 00b",
                    ),
                    value: 1,
                },
            ],
        },
        Enum {
            name: "Fovie",
            description: None,
            bit_size: 1,
            variants: &[
                EnumVariant {
                    name: "DISABLED",
                    description: Some(
                        "No interrupt when FOVR bit is set",
                    ),
                    value: 0,
                },
                EnumVariant {
                    name: "ENABLED",
                    description: Some(
                        "Interrupt generated when FOVR bit is set",
                    ),
                    value: 1,
                },
            ],
        },
        Enum {
            name: "Lbkm",
            description: None,
            bit_size: 1,
            variants: &[
                EnumVariant {
                    name: "DISABLED",
                    description: Some(
                        "Loop Back Mode disabled",
                    ),
                    value: 0,
                },
                EnumVariant {
                    name: "ENABLED",
                    description: Some(
                        "Loop Back Mode enabled",
                    ),
                    value: 1,
                },
            ],
        },
        Enum {
            name: "Lec",
            description: None,
            bit_size: 3,
            variants: &[
                EnumVariant {
                    name: "NOERROR",
                    description: Some(
                        "No Error",
                    ),
                    value: 0,
                },
                EnumVariant {
                    name: "STUFF",
                    description: Some(
                        "Stuff Error",
                    ),
                    value: 1,
                },
                EnumVariant {
                    name: "FORM",
                    description: Some(
                        "Form Error",
                    ),
                    value: 2,
                },
                EnumVariant {
                    name: "ACK",
                    description: Some(
                        "Acknowledgment Error",
                    ),
                    value: 3,
                },
                EnumVariant {
                    name: "BITRECESSIVE",
                    description: Some(
                        "Bit recessive Error",
                    ),
                    value: 4,
                },
                EnumVariant {
                    name: "BITDOMINANT",
                    description: Some(
                        "Bit dominant Error",
                    ),
                    value: 5,
                },
                EnumVariant {
                    name: "CRC",
                    description: Some(
                        "CRC Error",
                    ),
                    value: 6,
                },
                EnumVariant {
                    name: "CUSTOM",
                    description: Some(
                        "Set by software",
                    ),
                    value: 7,
                },
            ],
        },
        Enum {
            name: "Lecie",
            description: None,
            bit_size: 1,
            variants: &[
                EnumVariant {
                    name: "DISABLED",
                    description: Some(
                        "ERRI bit will not be set when the error code in LEC[2:0] is set by hardware on error detection",
                    ),
                    value: 0,
                },
                EnumVariant {
                    name: "ENABLED",
                    description: Some(
                        "ERRI bit will be set when the error code in LEC[2:0] is set by hardware on error detection",
                    ),
                    value: 1,
                },
            ],
        },
        Enum {
            name: "RirIde",
            description: None,
            bit_size: 1,
            variants: &[
                EnumVariant {
                    name: "STANDARD",
                    description: Some(
                        "Standard identifier",
                    ),
                    value: 0,
                },
                EnumVariant {
                    name: "EXTENDED",
                    description: Some(
                        "Extended identifier",
                    ),
                    value: 1,
                },
            ],
        },
        Enum {
            name: "RirRtr",
            description: None,
            bit_size: 1,
            variants: &[
                EnumVariant {
                    name: "DATA",
                    description: Some(
                        "Data frame",
                    ),
                    value: 0,
                },
                EnumVariant {
                    name: "REMOTE",
                    description: Some(
                        "Remote frame",
                    ),
                    value: 1,
                },
            ],
        },
        Enum {
            name: "Silm",
            description: None,
            bit_size: 1,
            variants: &[
                EnumVariant {
                    name: "NORMAL",
                    description: Some(
                        "Normal operation",
                    ),
                    value: 0,
                },
                EnumVariant {
                    name: "SILENT",
                    description: Some(
                        "Silent Mode",
                    ),
                    value: 1,
                },
            ],
        },
        Enum {
            name: "Slkie",
            description: None,
            bit_size: 1,
            variants: &[
                EnumVariant {
                    name: "DISABLED",
                    description: Some(
                        "No interrupt when SLAKI bit is set",
                    ),
                    value: 0,
                },
                EnumVariant {
                    name: "ENABLED",
                    description: Some(
                        "Interrupt generated when SLAKI bit is set",
                    ),
                    value: 1,
                },
            ],
        },
        Enum {
            name: "TirIde",
            description: None,
            bit_size: 1,
            variants: &[
                EnumVariant {
                    name: "STANDARD",
                    description: Some(
                        "Standard identifier",
                    ),
                    value: 0,
                },
                EnumVariant {
                    name: "EXTENDED",
                    description: Some(
                        "Extended identifier",
                    ),
                    value: 1,
                },
            ],
        },
        Enum {
            name: "TirRtr",
            description: None,
            bit_size: 1,
            variants: &[
                EnumVariant {
                    name: "DATA",
                    description: Some(
                        "Data frame",
                    ),
                    value: 0,
                },
                EnumVariant {
                    name: "REMOTE",
                    description: Some(
                        "Remote frame",
                    ),
                    value: 1,
                },
            ],
        },
        Enum {
            name: "Tmeie",
            description: None,
            bit_size: 1,
            variants: &[
                EnumVariant {
                    name: "DISABLED",
                    description: Some(
                        "No interrupt when RQCPx bit is set",
                    ),
                    value: 0,
                },
                EnumVariant {
                    name: "ENABLED",
                    description: Some(
                        "Interrupt generated when RQCPx bit is set",
                    ),
                    value: 1,
                },
            ],
        },
        Enum {
            name: "Wkuie",
            description: None,
            bit_size: 1,
            variants: &[
                EnumVariant {
                    name: "DISABLED",
                    description: Some(
                        "No interrupt when WKUI is set",
                    ),
                    value: 0,
                },
                EnumVariant {
                    name: "ENABLED",
                    description: Some(
                        "Interrupt generated when WKUI bit is set",
                    ),
                    value: 1,
                },
            ],
        },
    ],
};

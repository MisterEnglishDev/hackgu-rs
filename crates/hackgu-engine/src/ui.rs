//! Subsystem: ui
//! Contains 85 decompiled classes.

use crate::CcClass;

/// Decompiled CC2 class: `ListWnd` (VTable RVA: 0x00871ec0, 64 virtual methods, 49 recovered fields)
/// Inherits: MoveAnmPos
#[derive(Debug, Clone, Default)]
pub struct ListWnd {
    pub vtable_rva: u32,
    pub field_0x0000: u64,
    pub field_0x0008: f32,
    pub field_0x000c: f32,
    pub field_0x0018: u64,
    pub field_0x0020: u32,
    pub field_0x0024: u8,
    pub field_0x0026: u8,
    pub field_0x0028: u32,
    pub field_0x002c: u32,
    pub field_0x0030: u64,
    pub field_0x0040: [f32; 4],
    pub field_0x0050: [f32; 4],
    pub field_0x0054: u32,
    pub field_0x0058: u32,
    pub field_0x005c: u32,
    pub field_0x0060: u32,
    pub field_0x0064: u32,
    pub field_0x0068: f32,
    pub field_0x0070: u32,
    pub field_0x0074: u32,
    pub field_0x0078: f32,
    pub field_0x007c: f32,
    pub field_0x0080: [f32; 4],
    pub field_0x008c: u32,
    pub field_0x0090: u32,
    pub field_0x0094: u64,
    pub field_0x0098: u32,
    pub field_0x009c: u64,
    pub field_0x00a4: u32,
    pub field_0x00ac: f32,
    pub field_0x00b0: u32,
    pub field_0x00b4: u32,
    pub field_0x00c0: u32,
    pub field_0x00c8: u64,
    pub field_0x00cc: u32,
    pub field_0x00d0: u64,
    pub field_0x00d4: u32,
    pub field_0x00d8: u64,
    pub field_0x00e0: u8,
    pub field_0x00e1: u8,
    pub field_0x00f0: u64,
    pub field_0x00f8: u32,
    pub field_0x00fc: u32,
    pub field_0x0100: f32,
    pub field_0x010c: u8,
    pub field_0x010d: u8,
    pub field_0x0110: u32,
    pub field_0x0118: u64,
    pub field_0x0180: u64,
}

impl ListWnd {
    pub const CLASS_NAME: &'static str = "ListWnd";
    pub const VTABLE_RVA: u32 = 0x00871ec0;
    pub const METHOD_COUNT: usize = 64;
    pub const BASE_CLASSES: &'static [&'static str] = &["MoveAnmPos"];
    pub const VIRTUAL_METHODS: &'static [u64] = &[0x0000000180109550, 0x0000000180109c20, 0x000000018010a1e0, 0x000000018010a3e0, 0x000000018057b520, 0x000000018057b450, 0x000000018010c440, 0x000000018057b5a0, 0x0000000180103f70, 0x000000018057b440, 0x000000018010cf30, 0x0000000180103f80, 0x0000000180109520, 0x00000001800d3090, 0x000000018010a4e0, 0x000000018010a100, 0x00000001800d3090, 0x00000001800d3090, 0x00000001800d3090, 0x000000018010a520, 0x00000001800fe200, 0x00000001800fe200, 0x000000018010a590, 0x000000018010a670, 0x000000018010a680, 0x000000018010a8d0, 0x000000018010a910, 0x000000018010a690, 0x000000018010cfd0, 0x00000001800d3090, 0x00000001800d3090, 0x00000001800d3090, 0x00000001800d3090, 0x00000001800d3090, 0x00000001800fe200, 0x00000001800fe200, 0x00000001800fe200, 0x00000001800fe200, 0x00000001800fe200, 0x00000001800fe200, 0x00000001800fe200, 0x00000001800fe200, 0x00000001800fe200, 0x00000001800d3090, 0x00000001801094c0, 0x0000000180109650, 0x0000000180109d40, 0x0000000180109e70, 0x0000000180109fa0, 0x000000018010a020, 0x000000018010a090, 0x000000018010a460, 0x000000018010b660, 0x000000018010bb20, 0x000000018010bf60, 0x000000018010c090, 0x000000018010c230, 0x000000018010c230, 0x000000018010c340, 0x000000018010c2d0, 0x000000018010b3b0, 0x000000018010b5b0, 0x000000018010ce40, 0x000000018010c510];

    pub fn new() -> Self {
        Self {
            vtable_rva: Self::VTABLE_RVA,
            field_0x0000: Default::default(),
            field_0x0008: Default::default(),
            field_0x000c: Default::default(),
            field_0x0018: Default::default(),
            field_0x0020: Default::default(),
            field_0x0024: Default::default(),
            field_0x0026: Default::default(),
            field_0x0028: Default::default(),
            field_0x002c: Default::default(),
            field_0x0030: Default::default(),
            field_0x0040: Default::default(),
            field_0x0050: Default::default(),
            field_0x0054: Default::default(),
            field_0x0058: Default::default(),
            field_0x005c: Default::default(),
            field_0x0060: Default::default(),
            field_0x0064: Default::default(),
            field_0x0068: Default::default(),
            field_0x0070: Default::default(),
            field_0x0074: Default::default(),
            field_0x0078: Default::default(),
            field_0x007c: Default::default(),
            field_0x0080: Default::default(),
            field_0x008c: Default::default(),
            field_0x0090: Default::default(),
            field_0x0094: Default::default(),
            field_0x0098: Default::default(),
            field_0x009c: Default::default(),
            field_0x00a4: Default::default(),
            field_0x00ac: Default::default(),
            field_0x00b0: Default::default(),
            field_0x00b4: Default::default(),
            field_0x00c0: Default::default(),
            field_0x00c8: Default::default(),
            field_0x00cc: Default::default(),
            field_0x00d0: Default::default(),
            field_0x00d4: Default::default(),
            field_0x00d8: Default::default(),
            field_0x00e0: Default::default(),
            field_0x00e1: Default::default(),
            field_0x00f0: Default::default(),
            field_0x00f8: Default::default(),
            field_0x00fc: Default::default(),
            field_0x0100: Default::default(),
            field_0x010c: Default::default(),
            field_0x010d: Default::default(),
            field_0x0110: Default::default(),
            field_0x0118: Default::default(),
            field_0x0180: Default::default(),
        }
    }

    /// Decompiled virtual method #0 @ RVA 0x00109550 (46 instructions, 1 basic blocks)
    pub fn method_00(&mut self) {
    }

    /// Decompiled virtual method #1 @ RVA 0x00109c20 (130 instructions, 13 basic blocks)
    pub fn method_01(&mut self) {
        // Writes to field at offset +0x0118
    }

    /// Decompiled virtual method #2 @ RVA 0x0010a1e0 (112 instructions, 22 basic blocks)
    pub fn method_02(&mut self) {
        // Writes to field at offset +0x010c
        // Writes to field at offset +0x0118
    }

    /// Decompiled virtual method #3 @ RVA 0x0010a3e0 (30 instructions, 3 basic blocks)
    pub fn method_03(&mut self) {
        // Writes to field at offset +0x0118
    }

    /// Decompiled virtual method #4 @ RVA 0x0057b520 (30 instructions, 4 basic blocks)
    pub fn method_04(&mut self) {
    }

    /// Decompiled virtual method #5 @ RVA 0x0057b450 (90 instructions, 7 basic blocks)
    pub fn method_05(&mut self) {
    }

    /// Decompiled virtual method #6 @ RVA 0x0010c440 (24 instructions, 3 basic blocks)
    pub fn method_06(&mut self) {
    }

    /// Decompiled virtual method #7 @ RVA 0x0057b5a0 (7 instructions, 1 basic blocks)
    pub fn method_07(&mut self) {
        // Writes to field at offset +0x000c
        // Writes to field at offset +0x0026
    }

    /// Decompiled virtual method #8 @ RVA 0x00103f70 (2 instructions, 1 basic blocks)
    pub fn method_08(&mut self) {
        // Writes to field at offset +0x0020
    }

    /// Decompiled virtual method #9 @ RVA 0x0057b440 (98 instructions, 8 basic blocks)
    pub fn method_09(&mut self) {
        // Writes to field at offset +0x0008
    }

    /// Decompiled virtual method #10 @ RVA 0x0010cf30 (8 instructions, 3 basic blocks)
    pub fn method_10(&mut self) {
    }

    /// Decompiled virtual method #11 @ RVA 0x00103f80 (3 instructions, 1 basic blocks)
    pub fn method_11(&mut self) {
        // Writes to field at offset +0x0024
    }

    /// Decompiled virtual method #12 @ RVA 0x00109520 (7 instructions, 1 basic blocks)
    pub fn method_12(&mut self) {
        // Writes to field at offset +0x0094
        // Writes to field at offset +0x009c
        // Writes to field at offset +0x00a4
        // Writes to field at offset +0x00b4
        // Writes to field at offset +0x00c0
    }

    /// Decompiled virtual method #13 @ RVA 0x000d3090 (1 instructions, 1 basic blocks)
    pub fn method_13(&mut self) {
    }

    /// Decompiled virtual method #14 @ RVA 0x0010a4e0 (13 instructions, 3 basic blocks)
    pub fn method_14(&mut self) {
        // Writes to field at offset +0x010c
    }

    /// Decompiled virtual method #15 @ RVA 0x0010a100 (47 instructions, 10 basic blocks)
    pub fn method_15(&mut self) {
    }

    /// Decompiled virtual method #16 @ RVA 0x000d3090 (1 instructions, 1 basic blocks)
    pub fn method_16(&mut self) {
    }

    /// Decompiled virtual method #17 @ RVA 0x000d3090 (1 instructions, 1 basic blocks)
    pub fn method_17(&mut self) {
    }

    /// Decompiled virtual method #18 @ RVA 0x000d3090 (1 instructions, 1 basic blocks)
    pub fn method_18(&mut self) {
    }

    /// Decompiled virtual method #19 @ RVA 0x0010a520 (26 instructions, 4 basic blocks)
    pub fn method_19(&mut self) {
        // Writes to field at offset +0x010c
    }

    /// Decompiled virtual method #20 @ RVA 0x000fe200 (2 instructions, 1 basic blocks)
    pub fn method_20(&mut self) {
    }

    /// Decompiled virtual method #21 @ RVA 0x000fe200 (2 instructions, 1 basic blocks)
    pub fn method_21(&mut self) {
    }

    /// Decompiled virtual method #22 @ RVA 0x0010a590 (52 instructions, 9 basic blocks)
    pub fn method_22(&mut self) {
        // Writes to field at offset +0x010c
    }

    /// Decompiled virtual method #23 @ RVA 0x0010a670 (113 instructions, 16 basic blocks)
    pub fn method_23(&mut self) {
        // Writes to field at offset +0x010d
    }

    /// Decompiled virtual method #24 @ RVA 0x0010a680 (110 instructions, 16 basic blocks)
    pub fn method_24(&mut self) {
        // Writes to field at offset +0x010d
    }

    /// Decompiled virtual method #25 @ RVA 0x0010a8d0 (117 instructions, 7 basic blocks)
    pub fn method_25(&mut self) {
        // Writes to field at offset +0x0110
    }

    /// Decompiled virtual method #26 @ RVA 0x0010a910 (112 instructions, 9 basic blocks)
    pub fn method_26(&mut self) {
    }

    /// Decompiled virtual method #27 @ RVA 0x0010a690 (107 instructions, 15 basic blocks)
    pub fn method_27(&mut self) {
        // Writes to field at offset +0x010d
    }

    /// Decompiled virtual method #28 @ RVA 0x0010cfd0 (25 instructions, 3 basic blocks)
    pub fn method_28(&mut self) {
        // Writes to field at offset +0x010c
    }

    /// Decompiled virtual method #29 @ RVA 0x000d3090 (1 instructions, 1 basic blocks)
    pub fn method_29(&mut self) {
    }

    /// Decompiled virtual method #30 @ RVA 0x000d3090 (1 instructions, 1 basic blocks)
    pub fn method_30(&mut self) {
    }

    /// Decompiled virtual method #31 @ RVA 0x000d3090 (1 instructions, 1 basic blocks)
    pub fn method_31(&mut self) {
    }

    /// Decompiled virtual method #32 @ RVA 0x000d3090 (1 instructions, 1 basic blocks)
    pub fn method_32(&mut self) {
    }

    /// Decompiled virtual method #33 @ RVA 0x000d3090 (1 instructions, 1 basic blocks)
    pub fn method_33(&mut self) {
    }

    /// Decompiled virtual method #34 @ RVA 0x000fe200 (2 instructions, 1 basic blocks)
    pub fn method_34(&mut self) {
    }

    /// Decompiled virtual method #35 @ RVA 0x000fe200 (2 instructions, 1 basic blocks)
    pub fn method_35(&mut self) {
    }

    /// Decompiled virtual method #36 @ RVA 0x000fe200 (2 instructions, 1 basic blocks)
    pub fn method_36(&mut self) {
    }

    /// Decompiled virtual method #37 @ RVA 0x000fe200 (2 instructions, 1 basic blocks)
    pub fn method_37(&mut self) {
    }

    /// Decompiled virtual method #38 @ RVA 0x000fe200 (2 instructions, 1 basic blocks)
    pub fn method_38(&mut self) {
    }

    /// Decompiled virtual method #39 @ RVA 0x000fe200 (2 instructions, 1 basic blocks)
    pub fn method_39(&mut self) {
    }

    /// Decompiled virtual method #40 @ RVA 0x000fe200 (2 instructions, 1 basic blocks)
    pub fn method_40(&mut self) {
    }

    /// Decompiled virtual method #41 @ RVA 0x000fe200 (2 instructions, 1 basic blocks)
    pub fn method_41(&mut self) {
    }

    /// Decompiled virtual method #42 @ RVA 0x000fe200 (2 instructions, 1 basic blocks)
    pub fn method_42(&mut self) {
    }

    /// Decompiled virtual method #43 @ RVA 0x000d3090 (1 instructions, 1 basic blocks)
    pub fn method_43(&mut self) {
    }

    /// Decompiled virtual method #44 @ RVA 0x001094c0 (24 instructions, 3 basic blocks)
    pub fn method_44(&mut self) {
        // Writes to field at offset +0x0000
    }

    /// Decompiled virtual method #45 @ RVA 0x00109650 (112 instructions, 17 basic blocks)
    pub fn method_45(&mut self) {
        // Writes to field at offset +0x0040
        // Writes to field at offset +0x0050
        // Writes to field at offset +0x0054
        // Writes to field at offset +0x0058
        // Writes to field at offset +0x005c
        // Writes to field at offset +0x0060
        // Writes to field at offset +0x0070
        // Writes to field at offset +0x0080
        // Writes to field at offset +0x0090
        // Writes to field at offset +0x00c8
        // Writes to field at offset +0x00cc
        // Writes to field at offset +0x00d0
        // Writes to field at offset +0x00d4
        // Writes to field at offset +0x00e0
        // Writes to field at offset +0x00e1
        // Writes to field at offset +0x00f8
    }

    /// Decompiled virtual method #46 @ RVA 0x00109d40 (67 instructions, 5 basic blocks)
    pub fn method_46(&mut self) {
        // Writes to field at offset +0x0118
    }

    /// Decompiled virtual method #47 @ RVA 0x00109e70 (64 instructions, 5 basic blocks)
    pub fn method_47(&mut self) {
        // Writes to field at offset +0x00f8
        // Writes to field at offset +0x00fc
        // Writes to field at offset +0x0100
        // Writes to field at offset +0x0110
        // Writes to field at offset +0x0118
    }

    /// Decompiled virtual method #48 @ RVA 0x00109fa0 (101 instructions, 7 basic blocks)
    pub fn method_48(&mut self) {
    }

    /// Decompiled virtual method #49 @ RVA 0x0010a020 (66 instructions, 5 basic blocks)
    pub fn method_49(&mut self) {
    }

    /// Decompiled virtual method #50 @ RVA 0x0010a090 (26 instructions, 3 basic blocks)
    pub fn method_50(&mut self) {
    }

    /// Decompiled virtual method #51 @ RVA 0x0010a460 (30 instructions, 3 basic blocks)
    pub fn method_51(&mut self) {
        // Writes to field at offset +0x0118
    }

    /// Decompiled virtual method #52 @ RVA 0x0010b660 (126 instructions, 20 basic blocks)
    pub fn method_52(&mut self) {
        // Writes to field at offset +0x009c
    }

    /// Decompiled virtual method #53 @ RVA 0x0010bb20 (131 instructions, 17 basic blocks)
    pub fn method_53(&mut self) {
    }

    /// Decompiled virtual method #54 @ RVA 0x0010bf60 (69 instructions, 11 basic blocks)
    pub fn method_54(&mut self) {
    }

    /// Decompiled virtual method #55 @ RVA 0x0010c090 (98 instructions, 13 basic blocks)
    pub fn method_55(&mut self) {
    }

    /// Decompiled virtual method #56 @ RVA 0x0010c230 (26 instructions, 2 basic blocks)
    pub fn method_56(&mut self) {
    }

    /// Decompiled virtual method #57 @ RVA 0x0010c230 (26 instructions, 2 basic blocks)
    pub fn method_57(&mut self) {
    }

    /// Decompiled virtual method #58 @ RVA 0x0010c340 (29 instructions, 3 basic blocks)
    pub fn method_58(&mut self) {
    }

    /// Decompiled virtual method #59 @ RVA 0x0010c2d0 (29 instructions, 3 basic blocks)
    pub fn method_59(&mut self) {
    }

    /// Decompiled virtual method #60 @ RVA 0x0010b3b0 (11 instructions, 1 basic blocks)
    pub fn method_60(&mut self) {
    }

    /// Decompiled virtual method #61 @ RVA 0x0010b5b0 (23 instructions, 2 basic blocks)
    pub fn method_61(&mut self) {
    }

    /// Decompiled virtual method #62 @ RVA 0x0010ce40 (59 instructions, 6 basic blocks)
    pub fn method_62(&mut self) {
    }

    /// Decompiled virtual method #63 @ RVA 0x0010c510 (52 instructions, 6 basic blocks)
    pub fn method_63(&mut self) {
        // Writes to field at offset +0x0030
        // Writes to field at offset +0x00d0
        // Writes to field at offset +0x00d8
        // Writes to field at offset +0x00f0
    }

}

impl CcClass for ListWnd {
    fn class_name(&self) -> &'static str { Self::CLASS_NAME }
    fn vtable_rva(&self) -> u32 { Self::VTABLE_RVA }
    fn virtual_method_count(&self) -> usize { Self::METHOD_COUNT }
    fn base_classes(&self) -> &'static [&'static str] { Self::BASE_CLASSES }
}

/// Decompiled CC2 class: `ccAllotMenu` (VTable RVA: 0x0087a070, 31 virtual methods, 10 recovered fields)
/// Inherits: ccGameMenuMain, ccGameMenuObj
#[derive(Debug, Clone, Default)]
pub struct ccAllotMenu {
    pub vtable_rva: u32,
    pub field_0x0000: u64,
    pub field_0x0008: u32,
    pub field_0x000c: u32,
    pub field_0x0018: u32,
    pub field_0x0028: u8,
    pub field_0x0030: u32,
    pub field_0x0038: u64,
    pub field_0x0040: u8,
    pub field_0x0048: u64,
    pub field_0x0050: u64,
}

impl ccAllotMenu {
    pub const CLASS_NAME: &'static str = "ccAllotMenu";
    pub const VTABLE_RVA: u32 = 0x0087a070;
    pub const METHOD_COUNT: usize = 31;
    pub const BASE_CLASSES: &'static [&'static str] = &["ccGameMenuMain", "ccGameMenuObj"];
    pub const VIRTUAL_METHODS: &'static [u64] = &[0x00000001801b2890, 0x00000001801b2d30, 0x00000001801b2ea0, 0x00000001804525e0, 0x0000000180452730, 0x00000001804527e0, 0x0000000180452890, 0x00000001801b27f0, 0x00000001801035b0, 0x0000000180452bf0, 0x000000018045ba00, 0x00000001800d3090, 0x0000000180452300, 0x0000000180452380, 0x00000001801b2900, 0x00000001801b2a70, 0x00000001801b2b10, 0x00000001801b2c00, 0x00000001801b2ca0, 0x0000000180103e60, 0x0000000180103e60, 0x0000000180103e60, 0x0000000180103e60, 0x0000000180103e60, 0x0000000180103e60, 0x0000000180103e60, 0x0000000180103e60, 0x0000000180103e70, 0x00000001800d3090, 0x0000000180452bd0, 0x00000001801b2800];

    pub fn new() -> Self {
        Self {
            vtable_rva: Self::VTABLE_RVA,
            field_0x0000: Default::default(),
            field_0x0008: Default::default(),
            field_0x000c: Default::default(),
            field_0x0018: Default::default(),
            field_0x0028: Default::default(),
            field_0x0030: Default::default(),
            field_0x0038: Default::default(),
            field_0x0040: Default::default(),
            field_0x0048: Default::default(),
            field_0x0050: Default::default(),
        }
    }

    /// Decompiled virtual method #0 @ RVA 0x001b2890 (28 instructions, 3 basic blocks)
    pub fn method_00(&mut self) {
        // Writes to field at offset +0x0000
    }

    /// Decompiled virtual method #1 @ RVA 0x001b2d30 (12 instructions, 2 basic blocks)
    pub fn method_01(&mut self) {
    }

    /// Decompiled virtual method #2 @ RVA 0x001b2ea0 (25 instructions, 3 basic blocks)
    pub fn method_02(&mut self) {
    }

    /// Decompiled virtual method #3 @ RVA 0x004525e0 (9 instructions, 2 basic blocks)
    pub fn method_03(&mut self) {
        // Writes to field at offset +0x0040
    }

    /// Decompiled virtual method #4 @ RVA 0x00452730 (9 instructions, 2 basic blocks)
    pub fn method_04(&mut self) {
        // Writes to field at offset +0x0040
    }

    /// Decompiled virtual method #5 @ RVA 0x004527e0 (9 instructions, 2 basic blocks)
    pub fn method_05(&mut self) {
        // Writes to field at offset +0x0040
    }

    /// Decompiled virtual method #6 @ RVA 0x00452890 (9 instructions, 2 basic blocks)
    pub fn method_06(&mut self) {
        // Writes to field at offset +0x0040
    }

    /// Decompiled virtual method #7 @ RVA 0x001b27f0 (2 instructions, 1 basic blocks)
    pub fn method_07(&mut self) {
    }

    /// Decompiled virtual method #8 @ RVA 0x001035b0 (5 instructions, 1 basic blocks)
    pub fn method_08(&mut self) {
    }

    /// Decompiled virtual method #9 @ RVA 0x00452bf0 (12 instructions, 2 basic blocks)
    pub fn method_09(&mut self) {
        // Writes to field at offset +0x0038
    }

    /// Decompiled virtual method #10 @ RVA 0x0045ba00 (17 instructions, 2 basic blocks)
    pub fn method_10(&mut self) {
    }

    /// Decompiled virtual method #11 @ RVA 0x000d3090 (1 instructions, 1 basic blocks)
    pub fn method_11(&mut self) {
    }

    /// Decompiled virtual method #12 @ RVA 0x00452300 (9 instructions, 2 basic blocks)
    pub fn method_12(&mut self) {
        // Writes to field at offset +0x0028
    }

    /// Decompiled virtual method #13 @ RVA 0x00452380 (9 instructions, 2 basic blocks)
    pub fn method_13(&mut self) {
        // Writes to field at offset +0x0028
    }

    /// Decompiled virtual method #14 @ RVA 0x001b2900 (68 instructions, 6 basic blocks)
    pub fn method_14(&mut self) {
        // Writes to field at offset +0x000c
    }

    /// Decompiled virtual method #15 @ RVA 0x001b2a70 (42 instructions, 7 basic blocks)
    pub fn method_15(&mut self) {
    }

    /// Decompiled virtual method #16 @ RVA 0x001b2b10 (85 instructions, 9 basic blocks)
    pub fn method_16(&mut self) {
        // Writes to field at offset +0x0048
    }

    /// Decompiled virtual method #17 @ RVA 0x001b2c00 (9 instructions, 2 basic blocks)
    pub fn method_17(&mut self) {
        // Writes to field at offset +0x0048
    }

    /// Decompiled virtual method #18 @ RVA 0x001b2ca0 (12 instructions, 2 basic blocks)
    pub fn method_18(&mut self) {
    }

    /// Decompiled virtual method #19 @ RVA 0x00103e60 (2 instructions, 1 basic blocks)
    pub fn method_19(&mut self) {
    }

    /// Decompiled virtual method #20 @ RVA 0x00103e60 (2 instructions, 1 basic blocks)
    pub fn method_20(&mut self) {
    }

    /// Decompiled virtual method #21 @ RVA 0x00103e60 (2 instructions, 1 basic blocks)
    pub fn method_21(&mut self) {
    }

    /// Decompiled virtual method #22 @ RVA 0x00103e60 (2 instructions, 1 basic blocks)
    pub fn method_22(&mut self) {
    }

    /// Decompiled virtual method #23 @ RVA 0x00103e60 (2 instructions, 1 basic blocks)
    pub fn method_23(&mut self) {
    }

    /// Decompiled virtual method #24 @ RVA 0x00103e60 (2 instructions, 1 basic blocks)
    pub fn method_24(&mut self) {
    }

    /// Decompiled virtual method #25 @ RVA 0x00103e60 (2 instructions, 1 basic blocks)
    pub fn method_25(&mut self) {
    }

    /// Decompiled virtual method #26 @ RVA 0x00103e60 (2 instructions, 1 basic blocks)
    pub fn method_26(&mut self) {
    }

    /// Decompiled virtual method #27 @ RVA 0x00103e70 (2 instructions, 1 basic blocks)
    pub fn method_27(&mut self) {
    }

    /// Decompiled virtual method #28 @ RVA 0x000d3090 (1 instructions, 1 basic blocks)
    pub fn method_28(&mut self) {
    }

    /// Decompiled virtual method #29 @ RVA 0x00452bd0 (5 instructions, 2 basic blocks)
    pub fn method_29(&mut self) {
    }

    /// Decompiled virtual method #30 @ RVA 0x001b2800 (13 instructions, 4 basic blocks)
    pub fn method_30(&mut self) {
        // Writes to field at offset +0x0018
        // Writes to field at offset +0x0028
    }

}

impl CcClass for ccAllotMenu {
    fn class_name(&self) -> &'static str { Self::CLASS_NAME }
    fn vtable_rva(&self) -> u32 { Self::VTABLE_RVA }
    fn virtual_method_count(&self) -> usize { Self::METHOD_COUNT }
    fn base_classes(&self) -> &'static [&'static str] { Self::BASE_CLASSES }
}

/// Decompiled CC2 class: `ccAntiGauge` (VTable RVA: 0x008a4440, 6 virtual methods, 17 recovered fields)
/// Inherits: ccGauge
#[derive(Debug, Clone, Default)]
pub struct ccAntiGauge {
    pub vtable_rva: u32,
    pub field_0x0000: u64,
    pub field_0x0008: f32,
    pub field_0x000c: f32,
    pub field_0x0010: f32,
    pub field_0x0014: f32,
    pub field_0x0018: f32,
    pub field_0x0024: f32,
    pub field_0x0028: f32,
    pub field_0x002c: f32,
    pub field_0x0038: u64,
    pub field_0x0040: u32,
    pub field_0x0044: u32,
    pub field_0x0048: u32,
    pub field_0x004c: u32,
    pub field_0x0050: u32,
    pub field_0x0058: u64,
    pub field_0x2bd8: u32,
}

impl ccAntiGauge {
    pub const CLASS_NAME: &'static str = "ccAntiGauge";
    pub const VTABLE_RVA: u32 = 0x008a4440;
    pub const METHOD_COUNT: usize = 6;
    pub const BASE_CLASSES: &'static [&'static str] = &["ccGauge"];
    pub const VIRTUAL_METHODS: &'static [u64] = &[0x00000001802a47a0, 0x00000001802a47d0, 0x00000001802a49e0, 0x00000001802a4a90, 0x00000001802a4aa0, 0x00000001801b8190];

    pub fn new() -> Self {
        Self {
            vtable_rva: Self::VTABLE_RVA,
            field_0x0000: Default::default(),
            field_0x0008: Default::default(),
            field_0x000c: Default::default(),
            field_0x0010: Default::default(),
            field_0x0014: Default::default(),
            field_0x0018: Default::default(),
            field_0x0024: Default::default(),
            field_0x0028: Default::default(),
            field_0x002c: Default::default(),
            field_0x0038: Default::default(),
            field_0x0040: Default::default(),
            field_0x0044: Default::default(),
            field_0x0048: Default::default(),
            field_0x004c: Default::default(),
            field_0x0050: Default::default(),
            field_0x0058: Default::default(),
            field_0x2bd8: Default::default(),
        }
    }

    /// Decompiled virtual method #0 @ RVA 0x002a47a0 (13 instructions, 3 basic blocks)
    pub fn method_00(&mut self) {
        // Writes to field at offset +0x0000
    }

    /// Decompiled virtual method #1 @ RVA 0x002a47d0 (109 instructions, 12 basic blocks)
    pub fn method_01(&mut self) {
        // Writes to field at offset +0x0038
        // Writes to field at offset +0x0040
        // Writes to field at offset +0x0044
        // Writes to field at offset +0x0048
        // Writes to field at offset +0x004c
        // Writes to field at offset +0x0050
    }

    /// Decompiled virtual method #2 @ RVA 0x002a49e0 (45 instructions, 13 basic blocks)
    pub fn method_02(&mut self) {
    }

    /// Decompiled virtual method #3 @ RVA 0x002a4a90 (119 instructions, 18 basic blocks)
    pub fn method_03(&mut self) {
    }

    /// Decompiled virtual method #4 @ RVA 0x002a4aa0 (110 instructions, 17 basic blocks)
    pub fn method_04(&mut self) {
    }

    /// Decompiled virtual method #5 @ RVA 0x001b8190 (12 instructions, 1 basic blocks)
    pub fn method_05(&mut self) {
        // Writes to field at offset +0x0024
        // Writes to field at offset +0x0028
        // Writes to field at offset +0x002c
    }

}

impl CcClass for ccAntiGauge {
    fn class_name(&self) -> &'static str { Self::CLASS_NAME }
    fn vtable_rva(&self) -> u32 { Self::VTABLE_RVA }
    fn virtual_method_count(&self) -> usize { Self::METHOD_COUNT }
    fn base_classes(&self) -> &'static [&'static str] { Self::BASE_CLASSES }
}

/// Decompiled CC2 class: `ccAvaTrainListWnd` (VTable RVA: 0x0087dad8, 64 virtual methods, 42 recovered fields)
/// Inherits: ListWnd, MoveAnmPos
#[derive(Debug, Clone, Default)]
pub struct ccAvaTrainListWnd {
    pub vtable_rva: u32,
    pub field_0x0000: u64,
    pub field_0x0008: f32,
    pub field_0x000c: f32,
    pub field_0x0018: u64,
    pub field_0x0020: u32,
    pub field_0x0024: u8,
    pub field_0x0026: u8,
    pub field_0x0028: u32,
    pub field_0x002c: u32,
    pub field_0x0030: u64,
    pub field_0x0060: u32,
    pub field_0x0064: u32,
    pub field_0x0068: f32,
    pub field_0x0070: u32,
    pub field_0x0074: u32,
    pub field_0x0078: f32,
    pub field_0x007c: f32,
    pub field_0x008c: u32,
    pub field_0x0094: u64,
    pub field_0x0098: u32,
    pub field_0x009c: u64,
    pub field_0x00a4: u32,
    pub field_0x00ac: f32,
    pub field_0x00b0: u32,
    pub field_0x00b4: u32,
    pub field_0x00c0: u32,
    pub field_0x00c8: u64,
    pub field_0x00d0: u64,
    pub field_0x00d8: u64,
    pub field_0x00f0: u64,
    pub field_0x00f8: u32,
    pub field_0x00fc: u32,
    pub field_0x0100: f32,
    pub field_0x010c: u8,
    pub field_0x010d: u8,
    pub field_0x0110: u32,
    pub field_0x0118: u64,
    pub field_0x0140: u64,
    pub field_0x0150: u64,
    pub field_0x0158: u64,
    pub field_0x0160: u8,
    pub field_0x0180: u64,
}

impl ccAvaTrainListWnd {
    pub const CLASS_NAME: &'static str = "ccAvaTrainListWnd";
    pub const VTABLE_RVA: u32 = 0x0087dad8;
    pub const METHOD_COUNT: usize = 64;
    pub const BASE_CLASSES: &'static [&'static str] = &["ListWnd", "MoveAnmPos"];
    pub const VIRTUAL_METHODS: &'static [u64] = &[0x0000000180109550, 0x0000000180109c20, 0x000000018010a1e0, 0x00000001801edab0, 0x000000018057b520, 0x000000018057b450, 0x000000018010c440, 0x000000018057b5a0, 0x0000000180103f70, 0x000000018057b440, 0x000000018010cf30, 0x0000000180103f80, 0x0000000180109520, 0x00000001800d3090, 0x000000018010a4e0, 0x000000018010a100, 0x00000001801eda40, 0x00000001800d3090, 0x00000001800d3090, 0x000000018010a520, 0x00000001801edcd0, 0x00000001800fe200, 0x000000018010a590, 0x000000018010a670, 0x000000018010a680, 0x000000018010a8d0, 0x000000018010a910, 0x000000018010a690, 0x000000018010cfd0, 0x00000001800d3090, 0x00000001800d3090, 0x00000001800d3090, 0x00000001800d3090, 0x00000001800d3090, 0x00000001800fe200, 0x00000001800fe200, 0x00000001800fe200, 0x00000001800fe200, 0x00000001800fe200, 0x00000001800fe200, 0x00000001800fe200, 0x00000001800fe200, 0x00000001800fe200, 0x00000001800d3090, 0x00000001801ed5c0, 0x00000001801ed860, 0x0000000180109d40, 0x0000000180109e70, 0x0000000180109fa0, 0x000000018010a020, 0x000000018010a090, 0x000000018010a460, 0x000000018010b660, 0x000000018010bb20, 0x000000018010bf60, 0x000000018010c090, 0x000000018010c230, 0x000000018010c230, 0x000000018010c340, 0x000000018010c2d0, 0x000000018010b3b0, 0x000000018010b5b0, 0x000000018010ce40, 0x000000018010c510];

    pub fn new() -> Self {
        Self {
            vtable_rva: Self::VTABLE_RVA,
            field_0x0000: Default::default(),
            field_0x0008: Default::default(),
            field_0x000c: Default::default(),
            field_0x0018: Default::default(),
            field_0x0020: Default::default(),
            field_0x0024: Default::default(),
            field_0x0026: Default::default(),
            field_0x0028: Default::default(),
            field_0x002c: Default::default(),
            field_0x0030: Default::default(),
            field_0x0060: Default::default(),
            field_0x0064: Default::default(),
            field_0x0068: Default::default(),
            field_0x0070: Default::default(),
            field_0x0074: Default::default(),
            field_0x0078: Default::default(),
            field_0x007c: Default::default(),
            field_0x008c: Default::default(),
            field_0x0094: Default::default(),
            field_0x0098: Default::default(),
            field_0x009c: Default::default(),
            field_0x00a4: Default::default(),
            field_0x00ac: Default::default(),
            field_0x00b0: Default::default(),
            field_0x00b4: Default::default(),
            field_0x00c0: Default::default(),
            field_0x00c8: Default::default(),
            field_0x00d0: Default::default(),
            field_0x00d8: Default::default(),
            field_0x00f0: Default::default(),
            field_0x00f8: Default::default(),
            field_0x00fc: Default::default(),
            field_0x0100: Default::default(),
            field_0x010c: Default::default(),
            field_0x010d: Default::default(),
            field_0x0110: Default::default(),
            field_0x0118: Default::default(),
            field_0x0140: Default::default(),
            field_0x0150: Default::default(),
            field_0x0158: Default::default(),
            field_0x0160: Default::default(),
            field_0x0180: Default::default(),
        }
    }

    /// Decompiled virtual method #0 @ RVA 0x00109550 (46 instructions, 1 basic blocks)
    pub fn method_00(&mut self) {
    }

    /// Decompiled virtual method #1 @ RVA 0x00109c20 (130 instructions, 13 basic blocks)
    pub fn method_01(&mut self) {
        // Writes to field at offset +0x0118
    }

    /// Decompiled virtual method #2 @ RVA 0x0010a1e0 (112 instructions, 22 basic blocks)
    pub fn method_02(&mut self) {
        // Writes to field at offset +0x010c
        // Writes to field at offset +0x0118
    }

    /// Decompiled virtual method #3 @ RVA 0x001edab0 (113 instructions, 13 basic blocks)
    pub fn method_03(&mut self) {
        // Writes to field at offset +0x0118
    }

    /// Decompiled virtual method #4 @ RVA 0x0057b520 (30 instructions, 4 basic blocks)
    pub fn method_04(&mut self) {
    }

    /// Decompiled virtual method #5 @ RVA 0x0057b450 (90 instructions, 7 basic blocks)
    pub fn method_05(&mut self) {
    }

    /// Decompiled virtual method #6 @ RVA 0x0010c440 (24 instructions, 3 basic blocks)
    pub fn method_06(&mut self) {
    }

    /// Decompiled virtual method #7 @ RVA 0x0057b5a0 (7 instructions, 1 basic blocks)
    pub fn method_07(&mut self) {
        // Writes to field at offset +0x000c
        // Writes to field at offset +0x0026
    }

    /// Decompiled virtual method #8 @ RVA 0x00103f70 (2 instructions, 1 basic blocks)
    pub fn method_08(&mut self) {
        // Writes to field at offset +0x0020
    }

    /// Decompiled virtual method #9 @ RVA 0x0057b440 (98 instructions, 8 basic blocks)
    pub fn method_09(&mut self) {
        // Writes to field at offset +0x0008
    }

    /// Decompiled virtual method #10 @ RVA 0x0010cf30 (8 instructions, 3 basic blocks)
    pub fn method_10(&mut self) {
    }

    /// Decompiled virtual method #11 @ RVA 0x00103f80 (3 instructions, 1 basic blocks)
    pub fn method_11(&mut self) {
        // Writes to field at offset +0x0024
    }

    /// Decompiled virtual method #12 @ RVA 0x00109520 (7 instructions, 1 basic blocks)
    pub fn method_12(&mut self) {
        // Writes to field at offset +0x0094
        // Writes to field at offset +0x009c
        // Writes to field at offset +0x00a4
        // Writes to field at offset +0x00b4
        // Writes to field at offset +0x00c0
    }

    /// Decompiled virtual method #13 @ RVA 0x000d3090 (1 instructions, 1 basic blocks)
    pub fn method_13(&mut self) {
    }

    /// Decompiled virtual method #14 @ RVA 0x0010a4e0 (13 instructions, 3 basic blocks)
    pub fn method_14(&mut self) {
        // Writes to field at offset +0x010c
    }

    /// Decompiled virtual method #15 @ RVA 0x0010a100 (47 instructions, 10 basic blocks)
    pub fn method_15(&mut self) {
    }

    /// Decompiled virtual method #16 @ RVA 0x001eda40 (123 instructions, 15 basic blocks)
    pub fn method_16(&mut self) {
        // Writes to field at offset +0x0118
    }

    /// Decompiled virtual method #17 @ RVA 0x000d3090 (1 instructions, 1 basic blocks)
    pub fn method_17(&mut self) {
    }

    /// Decompiled virtual method #18 @ RVA 0x000d3090 (1 instructions, 1 basic blocks)
    pub fn method_18(&mut self) {
    }

    /// Decompiled virtual method #19 @ RVA 0x0010a520 (26 instructions, 4 basic blocks)
    pub fn method_19(&mut self) {
        // Writes to field at offset +0x010c
    }

    /// Decompiled virtual method #20 @ RVA 0x001edcd0 (43 instructions, 6 basic blocks)
    pub fn method_20(&mut self) {
        // Writes to field at offset +0x0160
    }

    /// Decompiled virtual method #21 @ RVA 0x000fe200 (2 instructions, 1 basic blocks)
    pub fn method_21(&mut self) {
    }

    /// Decompiled virtual method #22 @ RVA 0x0010a590 (52 instructions, 9 basic blocks)
    pub fn method_22(&mut self) {
        // Writes to field at offset +0x010c
    }

    /// Decompiled virtual method #23 @ RVA 0x0010a670 (113 instructions, 16 basic blocks)
    pub fn method_23(&mut self) {
        // Writes to field at offset +0x010d
    }

    /// Decompiled virtual method #24 @ RVA 0x0010a680 (110 instructions, 16 basic blocks)
    pub fn method_24(&mut self) {
        // Writes to field at offset +0x010d
    }

    /// Decompiled virtual method #25 @ RVA 0x0010a8d0 (117 instructions, 7 basic blocks)
    pub fn method_25(&mut self) {
        // Writes to field at offset +0x0110
    }

    /// Decompiled virtual method #26 @ RVA 0x0010a910 (112 instructions, 9 basic blocks)
    pub fn method_26(&mut self) {
    }

    /// Decompiled virtual method #27 @ RVA 0x0010a690 (107 instructions, 15 basic blocks)
    pub fn method_27(&mut self) {
        // Writes to field at offset +0x010d
    }

    /// Decompiled virtual method #28 @ RVA 0x0010cfd0 (25 instructions, 3 basic blocks)
    pub fn method_28(&mut self) {
        // Writes to field at offset +0x010c
    }

    /// Decompiled virtual method #29 @ RVA 0x000d3090 (1 instructions, 1 basic blocks)
    pub fn method_29(&mut self) {
    }

    /// Decompiled virtual method #30 @ RVA 0x000d3090 (1 instructions, 1 basic blocks)
    pub fn method_30(&mut self) {
    }

    /// Decompiled virtual method #31 @ RVA 0x000d3090 (1 instructions, 1 basic blocks)
    pub fn method_31(&mut self) {
    }

    /// Decompiled virtual method #32 @ RVA 0x000d3090 (1 instructions, 1 basic blocks)
    pub fn method_32(&mut self) {
    }

    /// Decompiled virtual method #33 @ RVA 0x000d3090 (1 instructions, 1 basic blocks)
    pub fn method_33(&mut self) {
    }

    /// Decompiled virtual method #34 @ RVA 0x000fe200 (2 instructions, 1 basic blocks)
    pub fn method_34(&mut self) {
    }

    /// Decompiled virtual method #35 @ RVA 0x000fe200 (2 instructions, 1 basic blocks)
    pub fn method_35(&mut self) {
    }

    /// Decompiled virtual method #36 @ RVA 0x000fe200 (2 instructions, 1 basic blocks)
    pub fn method_36(&mut self) {
    }

    /// Decompiled virtual method #37 @ RVA 0x000fe200 (2 instructions, 1 basic blocks)
    pub fn method_37(&mut self) {
    }

    /// Decompiled virtual method #38 @ RVA 0x000fe200 (2 instructions, 1 basic blocks)
    pub fn method_38(&mut self) {
    }

    /// Decompiled virtual method #39 @ RVA 0x000fe200 (2 instructions, 1 basic blocks)
    pub fn method_39(&mut self) {
    }

    /// Decompiled virtual method #40 @ RVA 0x000fe200 (2 instructions, 1 basic blocks)
    pub fn method_40(&mut self) {
    }

    /// Decompiled virtual method #41 @ RVA 0x000fe200 (2 instructions, 1 basic blocks)
    pub fn method_41(&mut self) {
    }

    /// Decompiled virtual method #42 @ RVA 0x000fe200 (2 instructions, 1 basic blocks)
    pub fn method_42(&mut self) {
    }

    /// Decompiled virtual method #43 @ RVA 0x000d3090 (1 instructions, 1 basic blocks)
    pub fn method_43(&mut self) {
    }

    /// Decompiled virtual method #44 @ RVA 0x001ed5c0 (16 instructions, 3 basic blocks)
    pub fn method_44(&mut self) {
    }

    /// Decompiled virtual method #45 @ RVA 0x001ed860 (47 instructions, 4 basic blocks)
    pub fn method_45(&mut self) {
    }

    /// Decompiled virtual method #46 @ RVA 0x00109d40 (67 instructions, 5 basic blocks)
    pub fn method_46(&mut self) {
        // Writes to field at offset +0x0118
    }

    /// Decompiled virtual method #47 @ RVA 0x00109e70 (64 instructions, 5 basic blocks)
    pub fn method_47(&mut self) {
        // Writes to field at offset +0x00f8
        // Writes to field at offset +0x00fc
        // Writes to field at offset +0x0100
        // Writes to field at offset +0x0110
        // Writes to field at offset +0x0118
    }

    /// Decompiled virtual method #48 @ RVA 0x00109fa0 (101 instructions, 7 basic blocks)
    pub fn method_48(&mut self) {
    }

    /// Decompiled virtual method #49 @ RVA 0x0010a020 (66 instructions, 5 basic blocks)
    pub fn method_49(&mut self) {
    }

    /// Decompiled virtual method #50 @ RVA 0x0010a090 (26 instructions, 3 basic blocks)
    pub fn method_50(&mut self) {
    }

    /// Decompiled virtual method #51 @ RVA 0x0010a460 (30 instructions, 3 basic blocks)
    pub fn method_51(&mut self) {
        // Writes to field at offset +0x0118
    }

    /// Decompiled virtual method #52 @ RVA 0x0010b660 (126 instructions, 20 basic blocks)
    pub fn method_52(&mut self) {
        // Writes to field at offset +0x009c
    }

    /// Decompiled virtual method #53 @ RVA 0x0010bb20 (131 instructions, 17 basic blocks)
    pub fn method_53(&mut self) {
    }

    /// Decompiled virtual method #54 @ RVA 0x0010bf60 (69 instructions, 11 basic blocks)
    pub fn method_54(&mut self) {
    }

    /// Decompiled virtual method #55 @ RVA 0x0010c090 (98 instructions, 13 basic blocks)
    pub fn method_55(&mut self) {
    }

    /// Decompiled virtual method #56 @ RVA 0x0010c230 (26 instructions, 2 basic blocks)
    pub fn method_56(&mut self) {
    }

    /// Decompiled virtual method #57 @ RVA 0x0010c230 (26 instructions, 2 basic blocks)
    pub fn method_57(&mut self) {
    }

    /// Decompiled virtual method #58 @ RVA 0x0010c340 (29 instructions, 3 basic blocks)
    pub fn method_58(&mut self) {
    }

    /// Decompiled virtual method #59 @ RVA 0x0010c2d0 (29 instructions, 3 basic blocks)
    pub fn method_59(&mut self) {
    }

    /// Decompiled virtual method #60 @ RVA 0x0010b3b0 (11 instructions, 1 basic blocks)
    pub fn method_60(&mut self) {
    }

    /// Decompiled virtual method #61 @ RVA 0x0010b5b0 (23 instructions, 2 basic blocks)
    pub fn method_61(&mut self) {
    }

    /// Decompiled virtual method #62 @ RVA 0x0010ce40 (59 instructions, 6 basic blocks)
    pub fn method_62(&mut self) {
    }

    /// Decompiled virtual method #63 @ RVA 0x0010c510 (52 instructions, 6 basic blocks)
    pub fn method_63(&mut self) {
        // Writes to field at offset +0x0030
        // Writes to field at offset +0x00d0
        // Writes to field at offset +0x00d8
        // Writes to field at offset +0x00f0
    }

}

impl CcClass for ccAvaTrainListWnd {
    fn class_name(&self) -> &'static str { Self::CLASS_NAME }
    fn vtable_rva(&self) -> u32 { Self::VTABLE_RVA }
    fn virtual_method_count(&self) -> usize { Self::METHOD_COUNT }
    fn base_classes(&self) -> &'static [&'static str] { Self::BASE_CLASSES }
}

/// Decompiled CC2 class: `ccAvaTrainMenuCtrl` (VTable RVA: 0x0087d9a8, 37 virtual methods, 15 recovered fields)
/// Inherits: ccGameMenuCtrlBase, ccGameMenuObj
#[derive(Debug, Clone, Default)]
pub struct ccAvaTrainMenuCtrl {
    pub vtable_rva: u32,
    pub field_0x0000: u64,
    pub field_0x0004: u32,
    pub field_0x0008: u32,
    pub field_0x000c: u8,
    pub field_0x0010: u32,
    pub field_0x0018: u32,
    pub field_0x0038: u32,
    pub field_0x0048: u64,
    pub field_0x0050: u64,
    pub field_0x0060: u64,
    pub field_0x0068: u64,
    pub field_0x0095: u8,
    pub field_0x009c: u32,
    pub field_0x00e1: u8,
    pub field_0x02c8: u32,
}

impl ccAvaTrainMenuCtrl {
    pub const CLASS_NAME: &'static str = "ccAvaTrainMenuCtrl";
    pub const VTABLE_RVA: u32 = 0x0087d9a8;
    pub const METHOD_COUNT: usize = 37;
    pub const BASE_CLASSES: &'static [&'static str] = &["ccGameMenuCtrlBase", "ccGameMenuObj"];
    pub const VIRTUAL_METHODS: &'static [u64] = &[0x00000001801edea0, 0x00000001804520b0, 0x0000000180452130, 0x0000000180451fb0, 0x0000000180451ff0, 0x0000000180452030, 0x0000000180452070, 0x0000000180103e90, 0x0000000180103ea0, 0x0000000180103e60, 0x000000018045bb30, 0x0000000180451d90, 0x00000001801e9580, 0x00000001801ee070, 0x0000000180103e60, 0x0000000180451dc0, 0x0000000180451e30, 0x0000000180451ea0, 0x0000000180451ee0, 0x0000000180451f40, 0x00000001801ee3a0, 0x00000001801ee840, 0x00000001801ee9b0, 0x0000000180103e60, 0x0000000180103e60, 0x0000000180103e70, 0x0000000180103e70, 0x0000000180103e60, 0x0000000180103e60, 0x0000000180103e60, 0x00000001801ee9f0, 0x00000001801eec30, 0x00000001800d3090, 0x0000000180103e70, 0x0000000180103e70, 0x0000000180103e70, 0x0000000180103e60];

    pub fn new() -> Self {
        Self {
            vtable_rva: Self::VTABLE_RVA,
            field_0x0000: Default::default(),
            field_0x0004: Default::default(),
            field_0x0008: Default::default(),
            field_0x000c: Default::default(),
            field_0x0010: Default::default(),
            field_0x0018: Default::default(),
            field_0x0038: Default::default(),
            field_0x0048: Default::default(),
            field_0x0050: Default::default(),
            field_0x0060: Default::default(),
            field_0x0068: Default::default(),
            field_0x0095: Default::default(),
            field_0x009c: Default::default(),
            field_0x00e1: Default::default(),
            field_0x02c8: Default::default(),
        }
    }

    /// Decompiled virtual method #0 @ RVA 0x001edea0 (16 instructions, 3 basic blocks)
    pub fn method_00(&mut self) {
    }

    /// Decompiled virtual method #1 @ RVA 0x004520b0 (10 instructions, 2 basic blocks)
    pub fn method_01(&mut self) {
    }

    /// Decompiled virtual method #2 @ RVA 0x00452130 (35 instructions, 9 basic blocks)
    pub fn method_02(&mut self) {
    }

    /// Decompiled virtual method #3 @ RVA 0x00451fb0 (118 instructions, 2 basic blocks)
    pub fn method_03(&mut self) {
    }

    /// Decompiled virtual method #4 @ RVA 0x00451ff0 (91 instructions, 2 basic blocks)
    pub fn method_04(&mut self) {
    }

    /// Decompiled virtual method #5 @ RVA 0x00452030 (64 instructions, 2 basic blocks)
    pub fn method_05(&mut self) {
    }

    /// Decompiled virtual method #6 @ RVA 0x00452070 (37 instructions, 2 basic blocks)
    pub fn method_06(&mut self) {
    }

    /// Decompiled virtual method #7 @ RVA 0x00103e90 (2 instructions, 1 basic blocks)
    pub fn method_07(&mut self) {
    }

    /// Decompiled virtual method #8 @ RVA 0x00103ea0 (9 instructions, 1 basic blocks)
    pub fn method_08(&mut self) {
        // Writes to field at offset +0x0008
    }

    /// Decompiled virtual method #9 @ RVA 0x00103e60 (2 instructions, 1 basic blocks)
    pub fn method_09(&mut self) {
    }

    /// Decompiled virtual method #10 @ RVA 0x0045bb30 (16 instructions, 2 basic blocks)
    pub fn method_10(&mut self) {
    }

    /// Decompiled virtual method #11 @ RVA 0x00451d90 (24 instructions, 2 basic blocks)
    pub fn method_11(&mut self) {
    }

    /// Decompiled virtual method #12 @ RVA 0x001e9580 (53 instructions, 12 basic blocks)
    pub fn method_12(&mut self) {
    }

    /// Decompiled virtual method #13 @ RVA 0x001ee070 (107 instructions, 9 basic blocks)
    pub fn method_13(&mut self) {
    }

    /// Decompiled virtual method #14 @ RVA 0x00103e60 (2 instructions, 1 basic blocks)
    pub fn method_14(&mut self) {
    }

    /// Decompiled virtual method #15 @ RVA 0x00451dc0 (10 instructions, 2 basic blocks)
    pub fn method_15(&mut self) {
    }

    /// Decompiled virtual method #16 @ RVA 0x00451e30 (10 instructions, 2 basic blocks)
    pub fn method_16(&mut self) {
    }

    /// Decompiled virtual method #17 @ RVA 0x00451ea0 (31 instructions, 2 basic blocks)
    pub fn method_17(&mut self) {
        // Writes to field at offset +0x0018
    }

    /// Decompiled virtual method #18 @ RVA 0x00451ee0 (10 instructions, 2 basic blocks)
    pub fn method_18(&mut self) {
    }

    /// Decompiled virtual method #19 @ RVA 0x00451f40 (10 instructions, 2 basic blocks)
    pub fn method_19(&mut self) {
    }

    /// Decompiled virtual method #20 @ RVA 0x001ee3a0 (120 instructions, 15 basic blocks)
    pub fn method_20(&mut self) {
        // Writes to field at offset +0x0000
        // Writes to field at offset +0x0004
        // Writes to field at offset +0x0008
        // Writes to field at offset +0x000c
        // Writes to field at offset +0x0038
        // Writes to field at offset +0x0048
        // Writes to field at offset +0x02c8
    }

    /// Decompiled virtual method #21 @ RVA 0x001ee840 (85 instructions, 21 basic blocks)
    pub fn method_21(&mut self) {
        // Writes to field at offset +0x0010
        // Writes to field at offset +0x0018
        // Writes to field at offset +0x0038
        // Writes to field at offset +0x0048
        // Writes to field at offset +0x0095
        // Writes to field at offset +0x00e1
    }

    /// Decompiled virtual method #22 @ RVA 0x001ee9b0 (15 instructions, 1 basic blocks)
    pub fn method_22(&mut self) {
    }

    /// Decompiled virtual method #23 @ RVA 0x00103e60 (2 instructions, 1 basic blocks)
    pub fn method_23(&mut self) {
    }

    /// Decompiled virtual method #24 @ RVA 0x00103e60 (2 instructions, 1 basic blocks)
    pub fn method_24(&mut self) {
    }

    /// Decompiled virtual method #25 @ RVA 0x00103e70 (2 instructions, 1 basic blocks)
    pub fn method_25(&mut self) {
    }

    /// Decompiled virtual method #26 @ RVA 0x00103e70 (2 instructions, 1 basic blocks)
    pub fn method_26(&mut self) {
    }

    /// Decompiled virtual method #27 @ RVA 0x00103e60 (2 instructions, 1 basic blocks)
    pub fn method_27(&mut self) {
    }

    /// Decompiled virtual method #28 @ RVA 0x00103e60 (2 instructions, 1 basic blocks)
    pub fn method_28(&mut self) {
    }

    /// Decompiled virtual method #29 @ RVA 0x00103e60 (2 instructions, 1 basic blocks)
    pub fn method_29(&mut self) {
    }

    /// Decompiled virtual method #30 @ RVA 0x001ee9f0 (105 instructions, 16 basic blocks)
    pub fn method_30(&mut self) {
    }

    /// Decompiled virtual method #31 @ RVA 0x001eec30 (90 instructions, 11 basic blocks)
    pub fn method_31(&mut self) {
        // Writes to field at offset +0x0068
    }

    /// Decompiled virtual method #32 @ RVA 0x000d3090 (1 instructions, 1 basic blocks)
    pub fn method_32(&mut self) {
    }

    /// Decompiled virtual method #33 @ RVA 0x00103e70 (2 instructions, 1 basic blocks)
    pub fn method_33(&mut self) {
    }

    /// Decompiled virtual method #34 @ RVA 0x00103e70 (2 instructions, 1 basic blocks)
    pub fn method_34(&mut self) {
    }

    /// Decompiled virtual method #35 @ RVA 0x00103e70 (2 instructions, 1 basic blocks)
    pub fn method_35(&mut self) {
    }

    /// Decompiled virtual method #36 @ RVA 0x00103e60 (2 instructions, 1 basic blocks)
    pub fn method_36(&mut self) {
    }

}

impl CcClass for ccAvaTrainMenuCtrl {
    fn class_name(&self) -> &'static str { Self::CLASS_NAME }
    fn vtable_rva(&self) -> u32 { Self::VTABLE_RVA }
    fn virtual_method_count(&self) -> usize { Self::METHOD_COUNT }
    fn base_classes(&self) -> &'static [&'static str] { Self::BASE_CLASSES }
}

/// Decompiled CC2 class: `ccAvaTrainMsgMenuCtrl` (VTable RVA: 0x0087d748, 37 virtual methods, 16 recovered fields)
/// Inherits: ccGameMenuCtrlBase, ccGameMenuObj
#[derive(Debug, Clone, Default)]
pub struct ccAvaTrainMsgMenuCtrl {
    pub vtable_rva: u32,
    pub field_0x0000: u64,
    pub field_0x0004: u32,
    pub field_0x0008: u32,
    pub field_0x000c: u8,
    pub field_0x0010: u64,
    pub field_0x0018: u32,
    pub field_0x0038: u32,
    pub field_0x0040: u32,
    pub field_0x0048: u64,
    pub field_0x0050: u64,
    pub field_0x0060: u64,
    pub field_0x0068: u32,
    pub field_0x006c: u16,
    pub field_0x006e: u16,
    pub field_0x0070: u32,
    pub field_0x009c: u32,
}

impl ccAvaTrainMsgMenuCtrl {
    pub const CLASS_NAME: &'static str = "ccAvaTrainMsgMenuCtrl";
    pub const VTABLE_RVA: u32 = 0x0087d748;
    pub const METHOD_COUNT: usize = 37;
    pub const BASE_CLASSES: &'static [&'static str] = &["ccGameMenuCtrlBase", "ccGameMenuObj"];
    pub const VIRTUAL_METHODS: &'static [u64] = &[0x00000001801f05b0, 0x00000001804520b0, 0x0000000180452130, 0x0000000180451fb0, 0x0000000180451ff0, 0x0000000180452030, 0x0000000180452070, 0x0000000180103e90, 0x0000000180103ea0, 0x0000000180103e60, 0x000000018045bb30, 0x0000000180451d90, 0x00000001801e9580, 0x00000001801f06b0, 0x0000000180103e60, 0x0000000180451dc0, 0x0000000180451e30, 0x0000000180451ea0, 0x0000000180451ee0, 0x0000000180451f40, 0x00000001801f07a0, 0x00000001801f0800, 0x0000000180103e70, 0x0000000180103e60, 0x0000000180103e60, 0x0000000180103e70, 0x0000000180103e70, 0x0000000180103e60, 0x0000000180103e60, 0x0000000180103e60, 0x00000001801f08a0, 0x00000001801f0950, 0x00000001800d3090, 0x0000000180103e70, 0x0000000180103e70, 0x0000000180103e70, 0x0000000180103e60];

    pub fn new() -> Self {
        Self {
            vtable_rva: Self::VTABLE_RVA,
            field_0x0000: Default::default(),
            field_0x0004: Default::default(),
            field_0x0008: Default::default(),
            field_0x000c: Default::default(),
            field_0x0010: Default::default(),
            field_0x0018: Default::default(),
            field_0x0038: Default::default(),
            field_0x0040: Default::default(),
            field_0x0048: Default::default(),
            field_0x0050: Default::default(),
            field_0x0060: Default::default(),
            field_0x0068: Default::default(),
            field_0x006c: Default::default(),
            field_0x006e: Default::default(),
            field_0x0070: Default::default(),
            field_0x009c: Default::default(),
        }
    }

    /// Decompiled virtual method #0 @ RVA 0x001f05b0 (16 instructions, 3 basic blocks)
    pub fn method_00(&mut self) {
    }

    /// Decompiled virtual method #1 @ RVA 0x004520b0 (10 instructions, 2 basic blocks)
    pub fn method_01(&mut self) {
    }

    /// Decompiled virtual method #2 @ RVA 0x00452130 (35 instructions, 9 basic blocks)
    pub fn method_02(&mut self) {
    }

    /// Decompiled virtual method #3 @ RVA 0x00451fb0 (118 instructions, 2 basic blocks)
    pub fn method_03(&mut self) {
    }

    /// Decompiled virtual method #4 @ RVA 0x00451ff0 (91 instructions, 2 basic blocks)
    pub fn method_04(&mut self) {
    }

    /// Decompiled virtual method #5 @ RVA 0x00452030 (64 instructions, 2 basic blocks)
    pub fn method_05(&mut self) {
    }

    /// Decompiled virtual method #6 @ RVA 0x00452070 (37 instructions, 2 basic blocks)
    pub fn method_06(&mut self) {
    }

    /// Decompiled virtual method #7 @ RVA 0x00103e90 (2 instructions, 1 basic blocks)
    pub fn method_07(&mut self) {
    }

    /// Decompiled virtual method #8 @ RVA 0x00103ea0 (9 instructions, 1 basic blocks)
    pub fn method_08(&mut self) {
        // Writes to field at offset +0x0008
    }

    /// Decompiled virtual method #9 @ RVA 0x00103e60 (2 instructions, 1 basic blocks)
    pub fn method_09(&mut self) {
    }

    /// Decompiled virtual method #10 @ RVA 0x0045bb30 (16 instructions, 2 basic blocks)
    pub fn method_10(&mut self) {
    }

    /// Decompiled virtual method #11 @ RVA 0x00451d90 (24 instructions, 2 basic blocks)
    pub fn method_11(&mut self) {
    }

    /// Decompiled virtual method #12 @ RVA 0x001e9580 (53 instructions, 12 basic blocks)
    pub fn method_12(&mut self) {
    }

    /// Decompiled virtual method #13 @ RVA 0x001f06b0 (64 instructions, 10 basic blocks)
    pub fn method_13(&mut self) {
        // Writes to field at offset +0x0000
        // Writes to field at offset +0x0004
        // Writes to field at offset +0x0008
        // Writes to field at offset +0x000c
    }

    /// Decompiled virtual method #14 @ RVA 0x00103e60 (2 instructions, 1 basic blocks)
    pub fn method_14(&mut self) {
    }

    /// Decompiled virtual method #15 @ RVA 0x00451dc0 (10 instructions, 2 basic blocks)
    pub fn method_15(&mut self) {
    }

    /// Decompiled virtual method #16 @ RVA 0x00451e30 (10 instructions, 2 basic blocks)
    pub fn method_16(&mut self) {
    }

    /// Decompiled virtual method #17 @ RVA 0x00451ea0 (31 instructions, 2 basic blocks)
    pub fn method_17(&mut self) {
        // Writes to field at offset +0x0018
    }

    /// Decompiled virtual method #18 @ RVA 0x00451ee0 (10 instructions, 2 basic blocks)
    pub fn method_18(&mut self) {
    }

    /// Decompiled virtual method #19 @ RVA 0x00451f40 (10 instructions, 2 basic blocks)
    pub fn method_19(&mut self) {
    }

    /// Decompiled virtual method #20 @ RVA 0x001f07a0 (21 instructions, 8 basic blocks)
    pub fn method_20(&mut self) {
        // Writes to field at offset +0x0040
        // Writes to field at offset +0x0068
        // Writes to field at offset +0x006c
        // Writes to field at offset +0x006e
        // Writes to field at offset +0x0070
    }

    /// Decompiled virtual method #21 @ RVA 0x001f0800 (39 instructions, 14 basic blocks)
    pub fn method_21(&mut self) {
        // Writes to field at offset +0x0038
        // Writes to field at offset +0x0048
    }

    /// Decompiled virtual method #22 @ RVA 0x00103e70 (2 instructions, 1 basic blocks)
    pub fn method_22(&mut self) {
    }

    /// Decompiled virtual method #23 @ RVA 0x00103e60 (2 instructions, 1 basic blocks)
    pub fn method_23(&mut self) {
    }

    /// Decompiled virtual method #24 @ RVA 0x00103e60 (2 instructions, 1 basic blocks)
    pub fn method_24(&mut self) {
    }

    /// Decompiled virtual method #25 @ RVA 0x00103e70 (2 instructions, 1 basic blocks)
    pub fn method_25(&mut self) {
    }

    /// Decompiled virtual method #26 @ RVA 0x00103e70 (2 instructions, 1 basic blocks)
    pub fn method_26(&mut self) {
    }

    /// Decompiled virtual method #27 @ RVA 0x00103e60 (2 instructions, 1 basic blocks)
    pub fn method_27(&mut self) {
    }

    /// Decompiled virtual method #28 @ RVA 0x00103e60 (2 instructions, 1 basic blocks)
    pub fn method_28(&mut self) {
    }

    /// Decompiled virtual method #29 @ RVA 0x00103e60 (2 instructions, 1 basic blocks)
    pub fn method_29(&mut self) {
    }

    /// Decompiled virtual method #30 @ RVA 0x001f08a0 (35 instructions, 5 basic blocks)
    pub fn method_30(&mut self) {
    }

    /// Decompiled virtual method #31 @ RVA 0x001f0950 (113 instructions, 10 basic blocks)
    pub fn method_31(&mut self) {
    }

    /// Decompiled virtual method #32 @ RVA 0x000d3090 (1 instructions, 1 basic blocks)
    pub fn method_32(&mut self) {
    }

    /// Decompiled virtual method #33 @ RVA 0x00103e70 (2 instructions, 1 basic blocks)
    pub fn method_33(&mut self) {
    }

    /// Decompiled virtual method #34 @ RVA 0x00103e70 (2 instructions, 1 basic blocks)
    pub fn method_34(&mut self) {
    }

    /// Decompiled virtual method #35 @ RVA 0x00103e70 (2 instructions, 1 basic blocks)
    pub fn method_35(&mut self) {
    }

    /// Decompiled virtual method #36 @ RVA 0x00103e60 (2 instructions, 1 basic blocks)
    pub fn method_36(&mut self) {
    }

}

impl CcClass for ccAvaTrainMsgMenuCtrl {
    fn class_name(&self) -> &'static str { Self::CLASS_NAME }
    fn vtable_rva(&self) -> u32 { Self::VTABLE_RVA }
    fn virtual_method_count(&self) -> usize { Self::METHOD_COUNT }
    fn base_classes(&self) -> &'static [&'static str] { Self::BASE_CLASSES }
}

/// Decompiled CC2 class: `ccAvatarGauge` (VTable RVA: 0x0087ccc8, 6 virtual methods, 10 recovered fields)
/// Inherits: ccGauge
#[derive(Debug, Clone, Default)]
pub struct ccAvatarGauge {
    pub vtable_rva: u32,
    pub field_0x0000: u64,
    pub field_0x0004: f32,
    pub field_0x0008: u32,
    pub field_0x000c: f32,
    pub field_0x0024: f32,
    pub field_0x0028: f32,
    pub field_0x002c: f32,
    pub field_0x0050: u64,
    pub field_0x0058: u64,
    pub field_0x00a8: u8,
}

impl ccAvatarGauge {
    pub const CLASS_NAME: &'static str = "ccAvatarGauge";
    pub const VTABLE_RVA: u32 = 0x0087ccc8;
    pub const METHOD_COUNT: usize = 6;
    pub const BASE_CLASSES: &'static [&'static str] = &["ccGauge"];
    pub const VIRTUAL_METHODS: &'static [u64] = &[0x00000001801d7d10, 0x00000001800d3090, 0x00000001801cc600, 0x00000001801cc700, 0x00000001801cc780, 0x00000001801b8190];

    pub fn new() -> Self {
        Self {
            vtable_rva: Self::VTABLE_RVA,
            field_0x0000: Default::default(),
            field_0x0004: Default::default(),
            field_0x0008: Default::default(),
            field_0x000c: Default::default(),
            field_0x0024: Default::default(),
            field_0x0028: Default::default(),
            field_0x002c: Default::default(),
            field_0x0050: Default::default(),
            field_0x0058: Default::default(),
            field_0x00a8: Default::default(),
        }
    }

    /// Decompiled virtual method #0 @ RVA 0x001d7d10 (30 instructions, 3 basic blocks)
    pub fn method_00(&mut self) {
        // Writes to field at offset +0x0000
    }

    /// Decompiled virtual method #1 @ RVA 0x000d3090 (1 instructions, 1 basic blocks)
    pub fn method_01(&mut self) {
    }

    /// Decompiled virtual method #2 @ RVA 0x001cc600 (41 instructions, 13 basic blocks)
    pub fn method_02(&mut self) {
    }

    /// Decompiled virtual method #3 @ RVA 0x001cc700 (33 instructions, 11 basic blocks)
    pub fn method_03(&mut self) {
        // Writes to field at offset +0x0050
        // Writes to field at offset +0x00a8
    }

    /// Decompiled virtual method #4 @ RVA 0x001cc780 (112 instructions, 18 basic blocks)
    pub fn method_04(&mut self) {
        // Writes to field at offset +0x0000
        // Writes to field at offset +0x0000
        // Writes to field at offset +0x0004
        // Writes to field at offset +0x0008
        // Writes to field at offset +0x000c
        // Writes to field at offset +0x00a8
    }

    /// Decompiled virtual method #5 @ RVA 0x001b8190 (12 instructions, 1 basic blocks)
    pub fn method_05(&mut self) {
        // Writes to field at offset +0x0024
        // Writes to field at offset +0x0028
        // Writes to field at offset +0x002c
    }

}

impl CcClass for ccAvatarGauge {
    fn class_name(&self) -> &'static str { Self::CLASS_NAME }
    fn vtable_rva(&self) -> u32 { Self::VTABLE_RVA }
    fn virtual_method_count(&self) -> usize { Self::METHOD_COUNT }
    fn base_classes(&self) -> &'static [&'static str] { Self::BASE_CLASSES }
}

/// Decompiled CC2 class: `ccBResBonusDisp` (VTable RVA: 0x008817b0, 14 virtual methods, 17 recovered fields)
/// Inherits: ccDispTarget, ccDispBase
#[derive(Debug, Clone, Default)]
pub struct ccBResBonusDisp {
    pub vtable_rva: u32,
    pub field_0x0000: u64,
    pub field_0x0008: u32,
    pub field_0x000c: u32,
    pub field_0x0010: u32,
    pub field_0x0014: f32,
    pub field_0x0018: f32,
    pub field_0x001c: u32,
    pub field_0x0028: u32,
    pub field_0x002c: u32,
    pub field_0x0030: u32,
    pub field_0x0034: u32,
    pub field_0x0038: u32,
    pub field_0x003c: u32,
    pub field_0x0048: u32,
    pub field_0x0050: u64,
    pub field_0x0058: u64,
    pub field_0x0060: u64,
}

impl ccBResBonusDisp {
    pub const CLASS_NAME: &'static str = "ccBResBonusDisp";
    pub const VTABLE_RVA: u32 = 0x008817b0;
    pub const METHOD_COUNT: usize = 14;
    pub const BASE_CLASSES: &'static [&'static str] = &["ccDispTarget", "ccDispBase"];
    pub const VIRTUAL_METHODS: &'static [u64] = &[0x0000000180233450, 0x00000001802424b0, 0x00000001802426e0, 0x00000001802429b0, 0x00000001802a0e30, 0x00000001800d3090, 0x00000001801b81f0, 0x00000001801b8200, 0x00000001801b8210, 0x00000001800d3090, 0x00000001800d3090, 0x00000001802427f0, 0x00000001800d3090, 0x00000001802428e0];

    pub fn new() -> Self {
        Self {
            vtable_rva: Self::VTABLE_RVA,
            field_0x0000: Default::default(),
            field_0x0008: Default::default(),
            field_0x000c: Default::default(),
            field_0x0010: Default::default(),
            field_0x0014: Default::default(),
            field_0x0018: Default::default(),
            field_0x001c: Default::default(),
            field_0x0028: Default::default(),
            field_0x002c: Default::default(),
            field_0x0030: Default::default(),
            field_0x0034: Default::default(),
            field_0x0038: Default::default(),
            field_0x003c: Default::default(),
            field_0x0048: Default::default(),
            field_0x0050: Default::default(),
            field_0x0058: Default::default(),
            field_0x0060: Default::default(),
        }
    }

    /// Decompiled virtual method #0 @ RVA 0x00233450 (13 instructions, 3 basic blocks)
    pub fn method_00(&mut self) {
        // Writes to field at offset +0x0000
    }

    /// Decompiled virtual method #1 @ RVA 0x002424b0 (122 instructions, 17 basic blocks)
    pub fn method_01(&mut self) {
        // Writes to field at offset +0x0058
    }

    /// Decompiled virtual method #2 @ RVA 0x002426e0 (50 instructions, 13 basic blocks)
    pub fn method_02(&mut self) {
    }

    /// Decompiled virtual method #3 @ RVA 0x002429b0 (107 instructions, 5 basic blocks)
    pub fn method_03(&mut self) {
        // Writes to field at offset +0x0048
    }

    /// Decompiled virtual method #4 @ RVA 0x002a0e30 (48 instructions, 11 basic blocks)
    pub fn method_04(&mut self) {
        // Writes to field at offset +0x0008
    }

    /// Decompiled virtual method #5 @ RVA 0x000d3090 (1 instructions, 1 basic blocks)
    pub fn method_05(&mut self) {
    }

    /// Decompiled virtual method #6 @ RVA 0x001b81f0 (2 instructions, 1 basic blocks)
    pub fn method_06(&mut self) {
        // Writes to field at offset +0x0008
    }

    /// Decompiled virtual method #7 @ RVA 0x001b8200 (5 instructions, 1 basic blocks)
    pub fn method_07(&mut self) {
    }

    /// Decompiled virtual method #8 @ RVA 0x001b8210 (5 instructions, 1 basic blocks)
    pub fn method_08(&mut self) {
    }

    /// Decompiled virtual method #9 @ RVA 0x000d3090 (1 instructions, 1 basic blocks)
    pub fn method_09(&mut self) {
    }

    /// Decompiled virtual method #10 @ RVA 0x000d3090 (1 instructions, 1 basic blocks)
    pub fn method_10(&mut self) {
    }

    /// Decompiled virtual method #11 @ RVA 0x002427f0 (56 instructions, 10 basic blocks)
    pub fn method_11(&mut self) {
        // Writes to field at offset +0x001c
        // Writes to field at offset +0x001c
    }

    /// Decompiled virtual method #12 @ RVA 0x000d3090 (1 instructions, 1 basic blocks)
    pub fn method_12(&mut self) {
    }

    /// Decompiled virtual method #13 @ RVA 0x002428e0 (47 instructions, 6 basic blocks)
    pub fn method_13(&mut self) {
        // Writes to field at offset +0x001c
        // Writes to field at offset +0x001c
    }

}

impl CcClass for ccBResBonusDisp {
    fn class_name(&self) -> &'static str { Self::CLASS_NAME }
    fn vtable_rva(&self) -> u32 { Self::VTABLE_RVA }
    fn virtual_method_count(&self) -> usize { Self::METHOD_COUNT }
    fn base_classes(&self) -> &'static [&'static str] { Self::BASE_CLASSES }
}

/// Decompiled CC2 class: `ccBResCharDisp` (VTable RVA: 0x00880f98, 14 virtual methods, 17 recovered fields)
/// Inherits: ccDispBase
#[derive(Debug, Clone, Default)]
pub struct ccBResCharDisp {
    pub vtable_rva: u32,
    pub field_0x0000: u64,
    pub field_0x0003: u8,
    pub field_0x0008: u32,
    pub field_0x000c: u32,
    pub field_0x0010: u32,
    pub field_0x0014: f32,
    pub field_0x0018: f32,
    pub field_0x001a: u8,
    pub field_0x001c: f32,
    pub field_0x0028: u32,
    pub field_0x002c: u32,
    pub field_0x0030: u32,
    pub field_0x0034: u32,
    pub field_0x0040: u64,
    pub field_0x0048: u64,
    pub field_0x0058: u64,
    pub field_0x0068: u64,
}

impl ccBResCharDisp {
    pub const CLASS_NAME: &'static str = "ccBResCharDisp";
    pub const VTABLE_RVA: u32 = 0x00880f98;
    pub const METHOD_COUNT: usize = 14;
    pub const BASE_CLASSES: &'static [&'static str] = &["ccDispBase"];
    pub const VIRTUAL_METHODS: &'static [u64] = &[0x0000000180244fb0, 0x0000000180245010, 0x0000000180245170, 0x00000001802452e0, 0x00000001802a0e30, 0x00000001800d3090, 0x00000001801b81f0, 0x00000001801b8200, 0x00000001801b8210, 0x0000000180245bd0, 0x00000001800d3090, 0x0000000180232bb0, 0x0000000180245c10, 0x0000000180232c20];

    pub fn new() -> Self {
        Self {
            vtable_rva: Self::VTABLE_RVA,
            field_0x0000: Default::default(),
            field_0x0003: Default::default(),
            field_0x0008: Default::default(),
            field_0x000c: Default::default(),
            field_0x0010: Default::default(),
            field_0x0014: Default::default(),
            field_0x0018: Default::default(),
            field_0x001a: Default::default(),
            field_0x001c: Default::default(),
            field_0x0028: Default::default(),
            field_0x002c: Default::default(),
            field_0x0030: Default::default(),
            field_0x0034: Default::default(),
            field_0x0040: Default::default(),
            field_0x0048: Default::default(),
            field_0x0058: Default::default(),
            field_0x0068: Default::default(),
        }
    }

    /// Decompiled virtual method #0 @ RVA 0x00244fb0 (23 instructions, 3 basic blocks)
    pub fn method_00(&mut self) {
        // Writes to field at offset +0x0000
    }

    /// Decompiled virtual method #1 @ RVA 0x00245010 (87 instructions, 11 basic blocks)
    pub fn method_01(&mut self) {
        // Writes to field at offset +0x0040
    }

    /// Decompiled virtual method #2 @ RVA 0x00245170 (104 instructions, 19 basic blocks)
    pub fn method_02(&mut self) {
    }

    /// Decompiled virtual method #3 @ RVA 0x002452e0 (110 instructions, 9 basic blocks)
    pub fn method_03(&mut self) {
        // Writes to field at offset +0x0000
        // Writes to field at offset +0x0003
        // Writes to field at offset +0x001a
    }

    /// Decompiled virtual method #4 @ RVA 0x002a0e30 (48 instructions, 11 basic blocks)
    pub fn method_04(&mut self) {
        // Writes to field at offset +0x0008
    }

    /// Decompiled virtual method #5 @ RVA 0x000d3090 (1 instructions, 1 basic blocks)
    pub fn method_05(&mut self) {
    }

    /// Decompiled virtual method #6 @ RVA 0x001b81f0 (2 instructions, 1 basic blocks)
    pub fn method_06(&mut self) {
        // Writes to field at offset +0x0008
    }

    /// Decompiled virtual method #7 @ RVA 0x001b8200 (5 instructions, 1 basic blocks)
    pub fn method_07(&mut self) {
    }

    /// Decompiled virtual method #8 @ RVA 0x001b8210 (5 instructions, 1 basic blocks)
    pub fn method_08(&mut self) {
    }

    /// Decompiled virtual method #9 @ RVA 0x00245bd0 (16 instructions, 3 basic blocks)
    pub fn method_09(&mut self) {
    }

    /// Decompiled virtual method #10 @ RVA 0x000d3090 (1 instructions, 1 basic blocks)
    pub fn method_10(&mut self) {
    }

    /// Decompiled virtual method #11 @ RVA 0x00232bb0 (26 instructions, 6 basic blocks)
    pub fn method_11(&mut self) {
        // Writes to field at offset +0x001c
        // Writes to field at offset +0x001c
    }

    /// Decompiled virtual method #12 @ RVA 0x00245c10 (150 instructions, 29 basic blocks)
    pub fn method_12(&mut self) {
    }

    /// Decompiled virtual method #13 @ RVA 0x00232c20 (26 instructions, 6 basic blocks)
    pub fn method_13(&mut self) {
        // Writes to field at offset +0x001c
        // Writes to field at offset +0x001c
    }

}

impl CcClass for ccBResCharDisp {
    fn class_name(&self) -> &'static str { Self::CLASS_NAME }
    fn vtable_rva(&self) -> u32 { Self::VTABLE_RVA }
    fn virtual_method_count(&self) -> usize { Self::METHOD_COUNT }
    fn base_classes(&self) -> &'static [&'static str] { Self::BASE_CLASSES }
}

/// Decompiled CC2 class: `ccBResGpWnd` (VTable RVA: 0x0087ae50, 15 virtual methods, 17 recovered fields)
/// Inherits: ccBResListWnd, ccDispBase
#[derive(Debug, Clone, Default)]
pub struct ccBResGpWnd {
    pub vtable_rva: u32,
    pub field_0x0000: u64,
    pub field_0x0008: u32,
    pub field_0x000c: u32,
    pub field_0x0010: u32,
    pub field_0x0014: f32,
    pub field_0x0018: f32,
    pub field_0x001c: f32,
    pub field_0x0028: u32,
    pub field_0x002c: u32,
    pub field_0x0030: u32,
    pub field_0x0034: u32,
    pub field_0x0038: u64,
    pub field_0x0040: u32,
    pub field_0x0044: u32,
    pub field_0x0048: u64,
    pub field_0x004c: u32,
    pub field_0x0078: u16,
}

impl ccBResGpWnd {
    pub const CLASS_NAME: &'static str = "ccBResGpWnd";
    pub const VTABLE_RVA: u32 = 0x0087ae50;
    pub const METHOD_COUNT: usize = 15;
    pub const BASE_CLASSES: &'static [&'static str] = &["ccBResListWnd", "ccDispBase"];
    pub const VIRTUAL_METHODS: &'static [u64] = &[0x00000001801c76d0, 0x0000000180242dc0, 0x0000000180242f90, 0x0000000180243200, 0x00000001802a0e30, 0x00000001800d3090, 0x00000001801b81f0, 0x00000001801b8200, 0x00000001801b8210, 0x00000001800d3090, 0x0000000180243150, 0x0000000180243060, 0x00000001802430d0, 0x00000001802430e0, 0x00000001801c7910];

    pub fn new() -> Self {
        Self {
            vtable_rva: Self::VTABLE_RVA,
            field_0x0000: Default::default(),
            field_0x0008: Default::default(),
            field_0x000c: Default::default(),
            field_0x0010: Default::default(),
            field_0x0014: Default::default(),
            field_0x0018: Default::default(),
            field_0x001c: Default::default(),
            field_0x0028: Default::default(),
            field_0x002c: Default::default(),
            field_0x0030: Default::default(),
            field_0x0034: Default::default(),
            field_0x0038: Default::default(),
            field_0x0040: Default::default(),
            field_0x0044: Default::default(),
            field_0x0048: Default::default(),
            field_0x004c: Default::default(),
            field_0x0078: Default::default(),
        }
    }

    /// Decompiled virtual method #0 @ RVA 0x001c76d0 (24 instructions, 3 basic blocks)
    pub fn method_00(&mut self) {
        // Writes to field at offset +0x0000
    }

    /// Decompiled virtual method #1 @ RVA 0x00242dc0 (107 instructions, 15 basic blocks)
    pub fn method_01(&mut self) {
        // Writes to field at offset +0x0040
        // Writes to field at offset +0x0044
    }

    /// Decompiled virtual method #2 @ RVA 0x00242f90 (57 instructions, 13 basic blocks)
    pub fn method_02(&mut self) {
    }

    /// Decompiled virtual method #3 @ RVA 0x00243200 (125 instructions, 14 basic blocks)
    pub fn method_03(&mut self) {
        // Writes to field at offset +0x0008
        // Writes to field at offset +0x0048
        // Writes to field at offset +0x004c
    }

    /// Decompiled virtual method #4 @ RVA 0x002a0e30 (48 instructions, 11 basic blocks)
    pub fn method_04(&mut self) {
        // Writes to field at offset +0x0008
    }

    /// Decompiled virtual method #5 @ RVA 0x000d3090 (1 instructions, 1 basic blocks)
    pub fn method_05(&mut self) {
    }

    /// Decompiled virtual method #6 @ RVA 0x001b81f0 (2 instructions, 1 basic blocks)
    pub fn method_06(&mut self) {
        // Writes to field at offset +0x0008
    }

    /// Decompiled virtual method #7 @ RVA 0x001b8200 (5 instructions, 1 basic blocks)
    pub fn method_07(&mut self) {
    }

    /// Decompiled virtual method #8 @ RVA 0x001b8210 (5 instructions, 1 basic blocks)
    pub fn method_08(&mut self) {
    }

    /// Decompiled virtual method #9 @ RVA 0x000d3090 (1 instructions, 1 basic blocks)
    pub fn method_09(&mut self) {
    }

    /// Decompiled virtual method #10 @ RVA 0x00243150 (37 instructions, 8 basic blocks)
    pub fn method_10(&mut self) {
    }

    /// Decompiled virtual method #11 @ RVA 0x00243060 (104 instructions, 21 basic blocks)
    pub fn method_11(&mut self) {
        // Writes to field at offset +0x001c
        // Writes to field at offset +0x001c
    }

    /// Decompiled virtual method #12 @ RVA 0x002430d0 (75 instructions, 15 basic blocks)
    pub fn method_12(&mut self) {
        // Writes to field at offset +0x001c
        // Writes to field at offset +0x001c
    }

    /// Decompiled virtual method #13 @ RVA 0x002430e0 (66 instructions, 14 basic blocks)
    pub fn method_13(&mut self) {
        // Writes to field at offset +0x001c
        // Writes to field at offset +0x001c
    }

    /// Decompiled virtual method #14 @ RVA 0x001c7910 (55 instructions, 1 basic blocks)
    pub fn method_14(&mut self) {
    }

}

impl CcClass for ccBResGpWnd {
    fn class_name(&self) -> &'static str { Self::CLASS_NAME }
    fn vtable_rva(&self) -> u32 { Self::VTABLE_RVA }
    fn virtual_method_count(&self) -> usize { Self::METHOD_COUNT }
    fn base_classes(&self) -> &'static [&'static str] { Self::BASE_CLASSES }
}

/// Decompiled CC2 class: `ccBResItemWnd` (VTable RVA: 0x008816a8, 15 virtual methods, 18 recovered fields)
/// Inherits: ccBResListWnd, ccDispBase
#[derive(Debug, Clone, Default)]
pub struct ccBResItemWnd {
    pub vtable_rva: u32,
    pub field_0x0000: u64,
    pub field_0x0008: u32,
    pub field_0x000c: u32,
    pub field_0x0010: u32,
    pub field_0x0014: f32,
    pub field_0x0018: f32,
    pub field_0x001a: u8,
    pub field_0x001c: f32,
    pub field_0x0028: u32,
    pub field_0x002c: u32,
    pub field_0x0030: u32,
    pub field_0x0034: u32,
    pub field_0x0038: u64,
    pub field_0x0040: u32,
    pub field_0x0044: u32,
    pub field_0x0048: u64,
    pub field_0x004c: u32,
    pub field_0x0078: u16,
}

impl ccBResItemWnd {
    pub const CLASS_NAME: &'static str = "ccBResItemWnd";
    pub const VTABLE_RVA: u32 = 0x008816a8;
    pub const METHOD_COUNT: usize = 15;
    pub const BASE_CLASSES: &'static [&'static str] = &["ccBResListWnd", "ccDispBase"];
    pub const VIRTUAL_METHODS: &'static [u64] = &[0x00000001802334c0, 0x0000000180242dc0, 0x0000000180242f90, 0x0000000180243200, 0x00000001802a0e30, 0x00000001800d3090, 0x00000001801b81f0, 0x00000001801b8200, 0x00000001801b8210, 0x00000001800d3090, 0x0000000180243150, 0x0000000180243060, 0x00000001802430d0, 0x00000001802430e0, 0x0000000180244010];

    pub fn new() -> Self {
        Self {
            vtable_rva: Self::VTABLE_RVA,
            field_0x0000: Default::default(),
            field_0x0008: Default::default(),
            field_0x000c: Default::default(),
            field_0x0010: Default::default(),
            field_0x0014: Default::default(),
            field_0x0018: Default::default(),
            field_0x001a: Default::default(),
            field_0x001c: Default::default(),
            field_0x0028: Default::default(),
            field_0x002c: Default::default(),
            field_0x0030: Default::default(),
            field_0x0034: Default::default(),
            field_0x0038: Default::default(),
            field_0x0040: Default::default(),
            field_0x0044: Default::default(),
            field_0x0048: Default::default(),
            field_0x004c: Default::default(),
            field_0x0078: Default::default(),
        }
    }

    /// Decompiled virtual method #0 @ RVA 0x002334c0 (18 instructions, 3 basic blocks)
    pub fn method_00(&mut self) {
        // Writes to field at offset +0x0000
    }

    /// Decompiled virtual method #1 @ RVA 0x00242dc0 (107 instructions, 15 basic blocks)
    pub fn method_01(&mut self) {
        // Writes to field at offset +0x0040
        // Writes to field at offset +0x0044
    }

    /// Decompiled virtual method #2 @ RVA 0x00242f90 (57 instructions, 13 basic blocks)
    pub fn method_02(&mut self) {
    }

    /// Decompiled virtual method #3 @ RVA 0x00243200 (125 instructions, 14 basic blocks)
    pub fn method_03(&mut self) {
        // Writes to field at offset +0x0008
        // Writes to field at offset +0x0048
        // Writes to field at offset +0x004c
    }

    /// Decompiled virtual method #4 @ RVA 0x002a0e30 (48 instructions, 11 basic blocks)
    pub fn method_04(&mut self) {
        // Writes to field at offset +0x0008
    }

    /// Decompiled virtual method #5 @ RVA 0x000d3090 (1 instructions, 1 basic blocks)
    pub fn method_05(&mut self) {
    }

    /// Decompiled virtual method #6 @ RVA 0x001b81f0 (2 instructions, 1 basic blocks)
    pub fn method_06(&mut self) {
        // Writes to field at offset +0x0008
    }

    /// Decompiled virtual method #7 @ RVA 0x001b8200 (5 instructions, 1 basic blocks)
    pub fn method_07(&mut self) {
    }

    /// Decompiled virtual method #8 @ RVA 0x001b8210 (5 instructions, 1 basic blocks)
    pub fn method_08(&mut self) {
    }

    /// Decompiled virtual method #9 @ RVA 0x000d3090 (1 instructions, 1 basic blocks)
    pub fn method_09(&mut self) {
    }

    /// Decompiled virtual method #10 @ RVA 0x00243150 (37 instructions, 8 basic blocks)
    pub fn method_10(&mut self) {
    }

    /// Decompiled virtual method #11 @ RVA 0x00243060 (104 instructions, 21 basic blocks)
    pub fn method_11(&mut self) {
        // Writes to field at offset +0x001c
        // Writes to field at offset +0x001c
    }

    /// Decompiled virtual method #12 @ RVA 0x002430d0 (75 instructions, 15 basic blocks)
    pub fn method_12(&mut self) {
        // Writes to field at offset +0x001c
        // Writes to field at offset +0x001c
    }

    /// Decompiled virtual method #13 @ RVA 0x002430e0 (66 instructions, 14 basic blocks)
    pub fn method_13(&mut self) {
        // Writes to field at offset +0x001c
        // Writes to field at offset +0x001c
    }

    /// Decompiled virtual method #14 @ RVA 0x00244010 (110 instructions, 6 basic blocks)
    pub fn method_14(&mut self) {
        // Writes to field at offset +0x001a
    }

}

impl CcClass for ccBResItemWnd {
    fn class_name(&self) -> &'static str { Self::CLASS_NAME }
    fn vtable_rva(&self) -> u32 { Self::VTABLE_RVA }
    fn virtual_method_count(&self) -> usize { Self::METHOD_COUNT }
    fn base_classes(&self) -> &'static [&'static str] { Self::BASE_CLASSES }
}

/// Decompiled CC2 class: `ccBResListWnd` (VTable RVA: 0x00881108, 15 virtual methods, 17 recovered fields)
/// Inherits: ccDispBase
#[derive(Debug, Clone, Default)]
pub struct ccBResListWnd {
    pub vtable_rva: u32,
    pub field_0x0000: u64,
    pub field_0x0008: u32,
    pub field_0x000c: u32,
    pub field_0x0010: u32,
    pub field_0x0014: f32,
    pub field_0x0018: f32,
    pub field_0x001c: f32,
    pub field_0x0028: u32,
    pub field_0x002c: u32,
    pub field_0x0030: u32,
    pub field_0x0034: u32,
    pub field_0x0038: u64,
    pub field_0x0040: u32,
    pub field_0x0044: u32,
    pub field_0x0048: u64,
    pub field_0x004c: u32,
    pub field_0x0078: u16,
}

impl ccBResListWnd {
    pub const CLASS_NAME: &'static str = "ccBResListWnd";
    pub const VTABLE_RVA: u32 = 0x00881108;
    pub const METHOD_COUNT: usize = 15;
    pub const BASE_CLASSES: &'static [&'static str] = &["ccDispBase"];
    pub const VIRTUAL_METHODS: &'static [u64] = &[0x0000000180242d20, 0x0000000180242dc0, 0x0000000180242f90, 0x0000000180243200, 0x00000001802a0e30, 0x00000001800d3090, 0x00000001801b81f0, 0x00000001801b8200, 0x00000001801b8210, 0x00000001800d3090, 0x0000000180243150, 0x0000000180243060, 0x00000001802430d0, 0x00000001802430e0, 0x00000001800d3090];

    pub fn new() -> Self {
        Self {
            vtable_rva: Self::VTABLE_RVA,
            field_0x0000: Default::default(),
            field_0x0008: Default::default(),
            field_0x000c: Default::default(),
            field_0x0010: Default::default(),
            field_0x0014: Default::default(),
            field_0x0018: Default::default(),
            field_0x001c: Default::default(),
            field_0x0028: Default::default(),
            field_0x002c: Default::default(),
            field_0x0030: Default::default(),
            field_0x0034: Default::default(),
            field_0x0038: Default::default(),
            field_0x0040: Default::default(),
            field_0x0044: Default::default(),
            field_0x0048: Default::default(),
            field_0x004c: Default::default(),
            field_0x0078: Default::default(),
        }
    }

    /// Decompiled virtual method #0 @ RVA 0x00242d20 (16 instructions, 3 basic blocks)
    pub fn method_00(&mut self) {
    }

    /// Decompiled virtual method #1 @ RVA 0x00242dc0 (107 instructions, 15 basic blocks)
    pub fn method_01(&mut self) {
        // Writes to field at offset +0x0040
        // Writes to field at offset +0x0044
    }

    /// Decompiled virtual method #2 @ RVA 0x00242f90 (57 instructions, 13 basic blocks)
    pub fn method_02(&mut self) {
    }

    /// Decompiled virtual method #3 @ RVA 0x00243200 (125 instructions, 14 basic blocks)
    pub fn method_03(&mut self) {
        // Writes to field at offset +0x0008
        // Writes to field at offset +0x0048
        // Writes to field at offset +0x004c
    }

    /// Decompiled virtual method #4 @ RVA 0x002a0e30 (48 instructions, 11 basic blocks)
    pub fn method_04(&mut self) {
        // Writes to field at offset +0x0008
    }

    /// Decompiled virtual method #5 @ RVA 0x000d3090 (1 instructions, 1 basic blocks)
    pub fn method_05(&mut self) {
    }

    /// Decompiled virtual method #6 @ RVA 0x001b81f0 (2 instructions, 1 basic blocks)
    pub fn method_06(&mut self) {
        // Writes to field at offset +0x0008
    }

    /// Decompiled virtual method #7 @ RVA 0x001b8200 (5 instructions, 1 basic blocks)
    pub fn method_07(&mut self) {
    }

    /// Decompiled virtual method #8 @ RVA 0x001b8210 (5 instructions, 1 basic blocks)
    pub fn method_08(&mut self) {
    }

    /// Decompiled virtual method #9 @ RVA 0x000d3090 (1 instructions, 1 basic blocks)
    pub fn method_09(&mut self) {
    }

    /// Decompiled virtual method #10 @ RVA 0x00243150 (37 instructions, 8 basic blocks)
    pub fn method_10(&mut self) {
    }

    /// Decompiled virtual method #11 @ RVA 0x00243060 (104 instructions, 21 basic blocks)
    pub fn method_11(&mut self) {
        // Writes to field at offset +0x001c
        // Writes to field at offset +0x001c
    }

    /// Decompiled virtual method #12 @ RVA 0x002430d0 (75 instructions, 15 basic blocks)
    pub fn method_12(&mut self) {
        // Writes to field at offset +0x001c
        // Writes to field at offset +0x001c
    }

    /// Decompiled virtual method #13 @ RVA 0x002430e0 (66 instructions, 14 basic blocks)
    pub fn method_13(&mut self) {
        // Writes to field at offset +0x001c
        // Writes to field at offset +0x001c
    }

    /// Decompiled virtual method #14 @ RVA 0x000d3090 (1 instructions, 1 basic blocks)
    pub fn method_14(&mut self) {
    }

}

impl CcClass for ccBResListWnd {
    fn class_name(&self) -> &'static str { Self::CLASS_NAME }
    fn vtable_rva(&self) -> u32 { Self::VTABLE_RVA }
    fn virtual_method_count(&self) -> usize { Self::METHOD_COUNT }
    fn base_classes(&self) -> &'static [&'static str] { Self::BASE_CLASSES }
}

/// Decompiled CC2 class: `ccBResLvUpDisp` (VTable RVA: 0x00881010, 14 virtual methods, 19 recovered fields)
/// Inherits: ccDispTarget, ccDispBase
#[derive(Debug, Clone, Default)]
pub struct ccBResLvUpDisp {
    pub vtable_rva: u32,
    pub field_0x0000: u64,
    pub field_0x0002: u16,
    pub field_0x0008: u32,
    pub field_0x000c: u32,
    pub field_0x0010: u32,
    pub field_0x0014: f32,
    pub field_0x0018: f32,
    pub field_0x001c: f32,
    pub field_0x0020: u64,
    pub field_0x0028: u32,
    pub field_0x002c: u32,
    pub field_0x0030: u32,
    pub field_0x0034: u32,
    pub field_0x0038: u32,
    pub field_0x003c: u32,
    pub field_0x0048: u64,
    pub field_0x0050: u64,
    pub field_0x0060: u64,
    pub field_0x0068: u16,
}

impl ccBResLvUpDisp {
    pub const CLASS_NAME: &'static str = "ccBResLvUpDisp";
    pub const VTABLE_RVA: u32 = 0x00881010;
    pub const METHOD_COUNT: usize = 14;
    pub const BASE_CLASSES: &'static [&'static str] = &["ccDispTarget", "ccDispBase"];
    pub const VIRTUAL_METHODS: &'static [u64] = &[0x000000018023e480, 0x0000000180244920, 0x0000000180244a80, 0x0000000180244b20, 0x00000001802a0e30, 0x00000001800d3090, 0x0000000180244f30, 0x00000001801b8200, 0x00000001801b8210, 0x00000001800d3090, 0x00000001800d3090, 0x0000000180244dd0, 0x0000000180244ee0, 0x0000000180244e60];

    pub fn new() -> Self {
        Self {
            vtable_rva: Self::VTABLE_RVA,
            field_0x0000: Default::default(),
            field_0x0002: Default::default(),
            field_0x0008: Default::default(),
            field_0x000c: Default::default(),
            field_0x0010: Default::default(),
            field_0x0014: Default::default(),
            field_0x0018: Default::default(),
            field_0x001c: Default::default(),
            field_0x0020: Default::default(),
            field_0x0028: Default::default(),
            field_0x002c: Default::default(),
            field_0x0030: Default::default(),
            field_0x0034: Default::default(),
            field_0x0038: Default::default(),
            field_0x003c: Default::default(),
            field_0x0048: Default::default(),
            field_0x0050: Default::default(),
            field_0x0060: Default::default(),
            field_0x0068: Default::default(),
        }
    }

    /// Decompiled virtual method #0 @ RVA 0x0023e480 (13 instructions, 3 basic blocks)
    pub fn method_00(&mut self) {
        // Writes to field at offset +0x0000
    }

    /// Decompiled virtual method #1 @ RVA 0x00244920 (82 instructions, 16 basic blocks)
    pub fn method_01(&mut self) {
        // Writes to field at offset +0x0020
    }

    /// Decompiled virtual method #2 @ RVA 0x00244a80 (43 instructions, 13 basic blocks)
    pub fn method_02(&mut self) {
    }

    /// Decompiled virtual method #3 @ RVA 0x00244b20 (109 instructions, 7 basic blocks)
    pub fn method_03(&mut self) {
        // Writes to field at offset +0x0048
    }

    /// Decompiled virtual method #4 @ RVA 0x002a0e30 (48 instructions, 11 basic blocks)
    pub fn method_04(&mut self) {
        // Writes to field at offset +0x0008
    }

    /// Decompiled virtual method #5 @ RVA 0x000d3090 (1 instructions, 1 basic blocks)
    pub fn method_05(&mut self) {
    }

    /// Decompiled virtual method #6 @ RVA 0x00244f30 (31 instructions, 6 basic blocks)
    pub fn method_06(&mut self) {
        // Writes to field at offset +0x0008
    }

    /// Decompiled virtual method #7 @ RVA 0x001b8200 (5 instructions, 1 basic blocks)
    pub fn method_07(&mut self) {
    }

    /// Decompiled virtual method #8 @ RVA 0x001b8210 (5 instructions, 1 basic blocks)
    pub fn method_08(&mut self) {
    }

    /// Decompiled virtual method #9 @ RVA 0x000d3090 (1 instructions, 1 basic blocks)
    pub fn method_09(&mut self) {
    }

    /// Decompiled virtual method #10 @ RVA 0x000d3090 (1 instructions, 1 basic blocks)
    pub fn method_10(&mut self) {
    }

    /// Decompiled virtual method #11 @ RVA 0x00244dd0 (32 instructions, 6 basic blocks)
    pub fn method_11(&mut self) {
        // Writes to field at offset +0x001c
        // Writes to field at offset +0x001c
    }

    /// Decompiled virtual method #12 @ RVA 0x00244ee0 (18 instructions, 2 basic blocks)
    pub fn method_12(&mut self) {
        // Writes to field at offset +0x0068
    }

    /// Decompiled virtual method #13 @ RVA 0x00244e60 (29 instructions, 6 basic blocks)
    pub fn method_13(&mut self) {
        // Writes to field at offset +0x001c
        // Writes to field at offset +0x001c
    }

}

impl CcClass for ccBResLvUpDisp {
    fn class_name(&self) -> &'static str { Self::CLASS_NAME }
    fn vtable_rva(&self) -> u32 { Self::VTABLE_RVA }
    fn virtual_method_count(&self) -> usize { Self::METHOD_COUNT }
    fn base_classes(&self) -> &'static [&'static str] { Self::BASE_CLASSES }
}

/// Decompiled CC2 class: `ccBResSkillWnd` (VTable RVA: 0x00881828, 15 virtual methods, 20 recovered fields)
/// Inherits: ccBResListWnd, ccDispBase
#[derive(Debug, Clone, Default)]
pub struct ccBResSkillWnd {
    pub vtable_rva: u32,
    pub field_0x0000: u64,
    pub field_0x0008: u32,
    pub field_0x000c: u32,
    pub field_0x0010: u32,
    pub field_0x0014: f32,
    pub field_0x0018: f32,
    pub field_0x001c: f32,
    pub field_0x0028: u32,
    pub field_0x002c: u32,
    pub field_0x0030: u32,
    pub field_0x0034: u32,
    pub field_0x0038: u64,
    pub field_0x0040: u32,
    pub field_0x0044: u32,
    pub field_0x0048: u64,
    pub field_0x004c: u32,
    pub field_0x0060: u32,
    pub field_0x0064: u32,
    pub field_0x0078: u16,
    pub field_0x0088: u64,
}

impl ccBResSkillWnd {
    pub const CLASS_NAME: &'static str = "ccBResSkillWnd";
    pub const VTABLE_RVA: u32 = 0x00881828;
    pub const METHOD_COUNT: usize = 15;
    pub const BASE_CLASSES: &'static [&'static str] = &["ccBResListWnd", "ccDispBase"];
    pub const VIRTUAL_METHODS: &'static [u64] = &[0x0000000180233480, 0x0000000180242dc0, 0x0000000180242f90, 0x0000000180243200, 0x00000001802a0e30, 0x00000001800d3090, 0x00000001801b81f0, 0x00000001801b8200, 0x00000001801b8210, 0x00000001800d3090, 0x0000000180243150, 0x0000000180243060, 0x00000001802430d0, 0x00000001802430e0, 0x0000000180243970];

    pub fn new() -> Self {
        Self {
            vtable_rva: Self::VTABLE_RVA,
            field_0x0000: Default::default(),
            field_0x0008: Default::default(),
            field_0x000c: Default::default(),
            field_0x0010: Default::default(),
            field_0x0014: Default::default(),
            field_0x0018: Default::default(),
            field_0x001c: Default::default(),
            field_0x0028: Default::default(),
            field_0x002c: Default::default(),
            field_0x0030: Default::default(),
            field_0x0034: Default::default(),
            field_0x0038: Default::default(),
            field_0x0040: Default::default(),
            field_0x0044: Default::default(),
            field_0x0048: Default::default(),
            field_0x004c: Default::default(),
            field_0x0060: Default::default(),
            field_0x0064: Default::default(),
            field_0x0078: Default::default(),
            field_0x0088: Default::default(),
        }
    }

    /// Decompiled virtual method #0 @ RVA 0x00233480 (18 instructions, 3 basic blocks)
    pub fn method_00(&mut self) {
        // Writes to field at offset +0x0000
    }

    /// Decompiled virtual method #1 @ RVA 0x00242dc0 (107 instructions, 15 basic blocks)
    pub fn method_01(&mut self) {
        // Writes to field at offset +0x0040
        // Writes to field at offset +0x0044
    }

    /// Decompiled virtual method #2 @ RVA 0x00242f90 (57 instructions, 13 basic blocks)
    pub fn method_02(&mut self) {
    }

    /// Decompiled virtual method #3 @ RVA 0x00243200 (125 instructions, 14 basic blocks)
    pub fn method_03(&mut self) {
        // Writes to field at offset +0x0008
        // Writes to field at offset +0x0048
        // Writes to field at offset +0x004c
    }

    /// Decompiled virtual method #4 @ RVA 0x002a0e30 (48 instructions, 11 basic blocks)
    pub fn method_04(&mut self) {
        // Writes to field at offset +0x0008
    }

    /// Decompiled virtual method #5 @ RVA 0x000d3090 (1 instructions, 1 basic blocks)
    pub fn method_05(&mut self) {
    }

    /// Decompiled virtual method #6 @ RVA 0x001b81f0 (2 instructions, 1 basic blocks)
    pub fn method_06(&mut self) {
        // Writes to field at offset +0x0008
    }

    /// Decompiled virtual method #7 @ RVA 0x001b8200 (5 instructions, 1 basic blocks)
    pub fn method_07(&mut self) {
    }

    /// Decompiled virtual method #8 @ RVA 0x001b8210 (5 instructions, 1 basic blocks)
    pub fn method_08(&mut self) {
    }

    /// Decompiled virtual method #9 @ RVA 0x000d3090 (1 instructions, 1 basic blocks)
    pub fn method_09(&mut self) {
    }

    /// Decompiled virtual method #10 @ RVA 0x00243150 (37 instructions, 8 basic blocks)
    pub fn method_10(&mut self) {
    }

    /// Decompiled virtual method #11 @ RVA 0x00243060 (104 instructions, 21 basic blocks)
    pub fn method_11(&mut self) {
        // Writes to field at offset +0x001c
        // Writes to field at offset +0x001c
    }

    /// Decompiled virtual method #12 @ RVA 0x002430d0 (75 instructions, 15 basic blocks)
    pub fn method_12(&mut self) {
        // Writes to field at offset +0x001c
        // Writes to field at offset +0x001c
    }

    /// Decompiled virtual method #13 @ RVA 0x002430e0 (66 instructions, 14 basic blocks)
    pub fn method_13(&mut self) {
        // Writes to field at offset +0x001c
        // Writes to field at offset +0x001c
    }

    /// Decompiled virtual method #14 @ RVA 0x00243970 (108 instructions, 5 basic blocks)
    pub fn method_14(&mut self) {
        // Writes to field at offset +0x000c
        // Writes to field at offset +0x0010
        // Writes to field at offset +0x001c
        // Writes to field at offset +0x0060
        // Writes to field at offset +0x0064
    }

}

impl CcClass for ccBResSkillWnd {
    fn class_name(&self) -> &'static str { Self::CLASS_NAME }
    fn vtable_rva(&self) -> u32 { Self::VTABLE_RVA }
    fn virtual_method_count(&self) -> usize { Self::METHOD_COUNT }
    fn base_classes(&self) -> &'static [&'static str] { Self::BASE_CLASSES }
}

/// Decompiled CC2 class: `ccBResTrainWnd` (VTable RVA: 0x008c3c90, 15 virtual methods, 19 recovered fields)
/// Inherits: ccBResListWnd, ccDispBase
#[derive(Debug, Clone, Default)]
pub struct ccBResTrainWnd {
    pub vtable_rva: u32,
    pub field_0x0000: u64,
    pub field_0x0008: u32,
    pub field_0x000c: u32,
    pub field_0x0010: u32,
    pub field_0x0014: f32,
    pub field_0x0018: f32,
    pub field_0x001c: f32,
    pub field_0x0028: u32,
    pub field_0x002c: u32,
    pub field_0x0030: u32,
    pub field_0x0034: u32,
    pub field_0x0038: u64,
    pub field_0x0040: u32,
    pub field_0x0044: u32,
    pub field_0x0048: u64,
    pub field_0x004c: u32,
    pub field_0x0060: u32,
    pub field_0x0064: u32,
    pub field_0x0078: u16,
}

impl ccBResTrainWnd {
    pub const CLASS_NAME: &'static str = "ccBResTrainWnd";
    pub const VTABLE_RVA: u32 = 0x008c3c90;
    pub const METHOD_COUNT: usize = 15;
    pub const BASE_CLASSES: &'static [&'static str] = &["ccBResListWnd", "ccDispBase"];
    pub const VIRTUAL_METHODS: &'static [u64] = &[0x000000018054e3e0, 0x0000000180242dc0, 0x0000000180242f90, 0x0000000180243200, 0x00000001802a0e30, 0x00000001800d3090, 0x00000001801b81f0, 0x00000001801b8200, 0x00000001801b8210, 0x00000001800d3090, 0x0000000180243150, 0x0000000180243060, 0x00000001802430d0, 0x00000001802430e0, 0x00000001805504c0];

    pub fn new() -> Self {
        Self {
            vtable_rva: Self::VTABLE_RVA,
            field_0x0000: Default::default(),
            field_0x0008: Default::default(),
            field_0x000c: Default::default(),
            field_0x0010: Default::default(),
            field_0x0014: Default::default(),
            field_0x0018: Default::default(),
            field_0x001c: Default::default(),
            field_0x0028: Default::default(),
            field_0x002c: Default::default(),
            field_0x0030: Default::default(),
            field_0x0034: Default::default(),
            field_0x0038: Default::default(),
            field_0x0040: Default::default(),
            field_0x0044: Default::default(),
            field_0x0048: Default::default(),
            field_0x004c: Default::default(),
            field_0x0060: Default::default(),
            field_0x0064: Default::default(),
            field_0x0078: Default::default(),
        }
    }

    /// Decompiled virtual method #0 @ RVA 0x0054e3e0 (18 instructions, 3 basic blocks)
    pub fn method_00(&mut self) {
        // Writes to field at offset +0x0000
    }

    /// Decompiled virtual method #1 @ RVA 0x00242dc0 (107 instructions, 15 basic blocks)
    pub fn method_01(&mut self) {
        // Writes to field at offset +0x0040
        // Writes to field at offset +0x0044
    }

    /// Decompiled virtual method #2 @ RVA 0x00242f90 (57 instructions, 13 basic blocks)
    pub fn method_02(&mut self) {
    }

    /// Decompiled virtual method #3 @ RVA 0x00243200 (125 instructions, 14 basic blocks)
    pub fn method_03(&mut self) {
        // Writes to field at offset +0x0008
        // Writes to field at offset +0x0048
        // Writes to field at offset +0x004c
    }

    /// Decompiled virtual method #4 @ RVA 0x002a0e30 (48 instructions, 11 basic blocks)
    pub fn method_04(&mut self) {
        // Writes to field at offset +0x0008
    }

    /// Decompiled virtual method #5 @ RVA 0x000d3090 (1 instructions, 1 basic blocks)
    pub fn method_05(&mut self) {
    }

    /// Decompiled virtual method #6 @ RVA 0x001b81f0 (2 instructions, 1 basic blocks)
    pub fn method_06(&mut self) {
        // Writes to field at offset +0x0008
    }

    /// Decompiled virtual method #7 @ RVA 0x001b8200 (5 instructions, 1 basic blocks)
    pub fn method_07(&mut self) {
    }

    /// Decompiled virtual method #8 @ RVA 0x001b8210 (5 instructions, 1 basic blocks)
    pub fn method_08(&mut self) {
    }

    /// Decompiled virtual method #9 @ RVA 0x000d3090 (1 instructions, 1 basic blocks)
    pub fn method_09(&mut self) {
    }

    /// Decompiled virtual method #10 @ RVA 0x00243150 (37 instructions, 8 basic blocks)
    pub fn method_10(&mut self) {
    }

    /// Decompiled virtual method #11 @ RVA 0x00243060 (104 instructions, 21 basic blocks)
    pub fn method_11(&mut self) {
        // Writes to field at offset +0x001c
        // Writes to field at offset +0x001c
    }

    /// Decompiled virtual method #12 @ RVA 0x002430d0 (75 instructions, 15 basic blocks)
    pub fn method_12(&mut self) {
        // Writes to field at offset +0x001c
        // Writes to field at offset +0x001c
    }

    /// Decompiled virtual method #13 @ RVA 0x002430e0 (66 instructions, 14 basic blocks)
    pub fn method_13(&mut self) {
        // Writes to field at offset +0x001c
        // Writes to field at offset +0x001c
    }

    /// Decompiled virtual method #14 @ RVA 0x005504c0 (96 instructions, 5 basic blocks)
    pub fn method_14(&mut self) {
        // Writes to field at offset +0x000c
        // Writes to field at offset +0x0010
        // Writes to field at offset +0x001c
        // Writes to field at offset +0x0060
        // Writes to field at offset +0x0064
        // Writes to field at offset +0x0064
    }

}

impl CcClass for ccBResTrainWnd {
    fn class_name(&self) -> &'static str { Self::CLASS_NAME }
    fn vtable_rva(&self) -> u32 { Self::VTABLE_RVA }
    fn virtual_method_count(&self) -> usize { Self::METHOD_COUNT }
    fn base_classes(&self) -> &'static [&'static str] { Self::BASE_CLASSES }
}

/// Decompiled CC2 class: `ccBResWpWnd` (VTable RVA: 0x00881088, 15 virtual methods, 17 recovered fields)
/// Inherits: ccBResListWnd, ccDispBase
#[derive(Debug, Clone, Default)]
pub struct ccBResWpWnd {
    pub vtable_rva: u32,
    pub field_0x0000: u64,
    pub field_0x0008: u32,
    pub field_0x000c: u32,
    pub field_0x0010: u32,
    pub field_0x0014: f32,
    pub field_0x0018: f32,
    pub field_0x001c: f32,
    pub field_0x0028: u32,
    pub field_0x002c: u32,
    pub field_0x0030: u32,
    pub field_0x0034: u32,
    pub field_0x0038: u64,
    pub field_0x0040: u32,
    pub field_0x0044: u32,
    pub field_0x0048: u64,
    pub field_0x004c: u32,
    pub field_0x0078: u16,
}

impl ccBResWpWnd {
    pub const CLASS_NAME: &'static str = "ccBResWpWnd";
    pub const VTABLE_RVA: u32 = 0x00881088;
    pub const METHOD_COUNT: usize = 15;
    pub const BASE_CLASSES: &'static [&'static str] = &["ccBResListWnd", "ccDispBase"];
    pub const VIRTUAL_METHODS: &'static [u64] = &[0x00000001802445c0, 0x0000000180242dc0, 0x0000000180242f90, 0x0000000180243200, 0x00000001802a0e30, 0x00000001800d3090, 0x00000001801b81f0, 0x00000001801b8200, 0x00000001801b8210, 0x00000001800d3090, 0x0000000180243150, 0x0000000180243060, 0x00000001802430d0, 0x00000001802430e0, 0x0000000180244800];

    pub fn new() -> Self {
        Self {
            vtable_rva: Self::VTABLE_RVA,
            field_0x0000: Default::default(),
            field_0x0008: Default::default(),
            field_0x000c: Default::default(),
            field_0x0010: Default::default(),
            field_0x0014: Default::default(),
            field_0x0018: Default::default(),
            field_0x001c: Default::default(),
            field_0x0028: Default::default(),
            field_0x002c: Default::default(),
            field_0x0030: Default::default(),
            field_0x0034: Default::default(),
            field_0x0038: Default::default(),
            field_0x0040: Default::default(),
            field_0x0044: Default::default(),
            field_0x0048: Default::default(),
            field_0x004c: Default::default(),
            field_0x0078: Default::default(),
        }
    }

    /// Decompiled virtual method #0 @ RVA 0x002445c0 (24 instructions, 3 basic blocks)
    pub fn method_00(&mut self) {
        // Writes to field at offset +0x0000
    }

    /// Decompiled virtual method #1 @ RVA 0x00242dc0 (107 instructions, 15 basic blocks)
    pub fn method_01(&mut self) {
        // Writes to field at offset +0x0040
        // Writes to field at offset +0x0044
    }

    /// Decompiled virtual method #2 @ RVA 0x00242f90 (57 instructions, 13 basic blocks)
    pub fn method_02(&mut self) {
    }

    /// Decompiled virtual method #3 @ RVA 0x00243200 (125 instructions, 14 basic blocks)
    pub fn method_03(&mut self) {
        // Writes to field at offset +0x0008
        // Writes to field at offset +0x0048
        // Writes to field at offset +0x004c
    }

    /// Decompiled virtual method #4 @ RVA 0x002a0e30 (48 instructions, 11 basic blocks)
    pub fn method_04(&mut self) {
        // Writes to field at offset +0x0008
    }

    /// Decompiled virtual method #5 @ RVA 0x000d3090 (1 instructions, 1 basic blocks)
    pub fn method_05(&mut self) {
    }

    /// Decompiled virtual method #6 @ RVA 0x001b81f0 (2 instructions, 1 basic blocks)
    pub fn method_06(&mut self) {
        // Writes to field at offset +0x0008
    }

    /// Decompiled virtual method #7 @ RVA 0x001b8200 (5 instructions, 1 basic blocks)
    pub fn method_07(&mut self) {
    }

    /// Decompiled virtual method #8 @ RVA 0x001b8210 (5 instructions, 1 basic blocks)
    pub fn method_08(&mut self) {
    }

    /// Decompiled virtual method #9 @ RVA 0x000d3090 (1 instructions, 1 basic blocks)
    pub fn method_09(&mut self) {
    }

    /// Decompiled virtual method #10 @ RVA 0x00243150 (37 instructions, 8 basic blocks)
    pub fn method_10(&mut self) {
    }

    /// Decompiled virtual method #11 @ RVA 0x00243060 (104 instructions, 21 basic blocks)
    pub fn method_11(&mut self) {
        // Writes to field at offset +0x001c
        // Writes to field at offset +0x001c
    }

    /// Decompiled virtual method #12 @ RVA 0x002430d0 (75 instructions, 15 basic blocks)
    pub fn method_12(&mut self) {
        // Writes to field at offset +0x001c
        // Writes to field at offset +0x001c
    }

    /// Decompiled virtual method #13 @ RVA 0x002430e0 (66 instructions, 14 basic blocks)
    pub fn method_13(&mut self) {
        // Writes to field at offset +0x001c
        // Writes to field at offset +0x001c
    }

    /// Decompiled virtual method #14 @ RVA 0x00244800 (55 instructions, 1 basic blocks)
    pub fn method_14(&mut self) {
    }

}

impl CcClass for ccBResWpWnd {
    fn class_name(&self) -> &'static str { Self::CLASS_NAME }
    fn vtable_rva(&self) -> u32 { Self::VTABLE_RVA }
    fn virtual_method_count(&self) -> usize { Self::METHOD_COUNT }
    fn base_classes(&self) -> &'static [&'static str] { Self::BASE_CLASSES }
}

/// Decompiled CC2 class: `ccBikeCustomMenu` (VTable RVA: 0x00895280, 37 virtual methods, 9 recovered fields)
/// Inherits: ccGameMenuCtrlBase, ccGameMenuObj
#[derive(Debug, Clone, Default)]
pub struct ccBikeCustomMenu {
    pub vtable_rva: u32,
    pub field_0x0000: u64,
    pub field_0x0008: u32,
    pub field_0x0018: u32,
    pub field_0x001a: u8,
    pub field_0x0038: u32,
    pub field_0x0048: u64,
    pub field_0x0050: u64,
    pub field_0x0064: u32,
    pub field_0x009c: u32,
}

impl ccBikeCustomMenu {
    pub const CLASS_NAME: &'static str = "ccBikeCustomMenu";
    pub const VTABLE_RVA: u32 = 0x00895280;
    pub const METHOD_COUNT: usize = 37;
    pub const BASE_CLASSES: &'static [&'static str] = &["ccGameMenuCtrlBase", "ccGameMenuObj"];
    pub const VIRTUAL_METHODS: &'static [u64] = &[0x0000000180260370, 0x00000001804520b0, 0x0000000180452130, 0x0000000180451fb0, 0x0000000180451ff0, 0x0000000180452030, 0x0000000180452070, 0x0000000180103e90, 0x0000000180103ea0, 0x0000000180103e60, 0x000000018045bb30, 0x00000001802605e0, 0x00000001801e9580, 0x0000000180103e60, 0x0000000180103e60, 0x0000000180451dc0, 0x0000000180451e30, 0x0000000180451ea0, 0x0000000180451ee0, 0x0000000180451f40, 0x0000000180260c60, 0x0000000180260de0, 0x0000000180103e70, 0x0000000180103e60, 0x0000000180103e60, 0x0000000180103e70, 0x0000000180103e70, 0x0000000180103e60, 0x0000000180103e60, 0x0000000180103e60, 0x00000001802608f0, 0x0000000180260b20, 0x00000001800d3090, 0x0000000180103e70, 0x0000000180103e70, 0x0000000180103e80, 0x0000000180103e60];

    pub fn new() -> Self {
        Self {
            vtable_rva: Self::VTABLE_RVA,
            field_0x0000: Default::default(),
            field_0x0008: Default::default(),
            field_0x0018: Default::default(),
            field_0x001a: Default::default(),
            field_0x0038: Default::default(),
            field_0x0048: Default::default(),
            field_0x0050: Default::default(),
            field_0x0064: Default::default(),
            field_0x009c: Default::default(),
        }
    }

    /// Decompiled virtual method #0 @ RVA 0x00260370 (16 instructions, 3 basic blocks)
    pub fn method_00(&mut self) {
    }

    /// Decompiled virtual method #1 @ RVA 0x004520b0 (10 instructions, 2 basic blocks)
    pub fn method_01(&mut self) {
    }

    /// Decompiled virtual method #2 @ RVA 0x00452130 (35 instructions, 9 basic blocks)
    pub fn method_02(&mut self) {
    }

    /// Decompiled virtual method #3 @ RVA 0x00451fb0 (118 instructions, 2 basic blocks)
    pub fn method_03(&mut self) {
    }

    /// Decompiled virtual method #4 @ RVA 0x00451ff0 (91 instructions, 2 basic blocks)
    pub fn method_04(&mut self) {
    }

    /// Decompiled virtual method #5 @ RVA 0x00452030 (64 instructions, 2 basic blocks)
    pub fn method_05(&mut self) {
    }

    /// Decompiled virtual method #6 @ RVA 0x00452070 (37 instructions, 2 basic blocks)
    pub fn method_06(&mut self) {
    }

    /// Decompiled virtual method #7 @ RVA 0x00103e90 (2 instructions, 1 basic blocks)
    pub fn method_07(&mut self) {
    }

    /// Decompiled virtual method #8 @ RVA 0x00103ea0 (9 instructions, 1 basic blocks)
    pub fn method_08(&mut self) {
        // Writes to field at offset +0x0008
    }

    /// Decompiled virtual method #9 @ RVA 0x00103e60 (2 instructions, 1 basic blocks)
    pub fn method_09(&mut self) {
    }

    /// Decompiled virtual method #10 @ RVA 0x0045bb30 (16 instructions, 2 basic blocks)
    pub fn method_10(&mut self) {
    }

    /// Decompiled virtual method #11 @ RVA 0x002605e0 (88 instructions, 10 basic blocks)
    pub fn method_11(&mut self) {
        // Writes to field at offset +0x001a
    }

    /// Decompiled virtual method #12 @ RVA 0x001e9580 (53 instructions, 12 basic blocks)
    pub fn method_12(&mut self) {
    }

    /// Decompiled virtual method #13 @ RVA 0x00103e60 (2 instructions, 1 basic blocks)
    pub fn method_13(&mut self) {
    }

    /// Decompiled virtual method #14 @ RVA 0x00103e60 (2 instructions, 1 basic blocks)
    pub fn method_14(&mut self) {
    }

    /// Decompiled virtual method #15 @ RVA 0x00451dc0 (10 instructions, 2 basic blocks)
    pub fn method_15(&mut self) {
    }

    /// Decompiled virtual method #16 @ RVA 0x00451e30 (10 instructions, 2 basic blocks)
    pub fn method_16(&mut self) {
    }

    /// Decompiled virtual method #17 @ RVA 0x00451ea0 (31 instructions, 2 basic blocks)
    pub fn method_17(&mut self) {
        // Writes to field at offset +0x0018
    }

    /// Decompiled virtual method #18 @ RVA 0x00451ee0 (10 instructions, 2 basic blocks)
    pub fn method_18(&mut self) {
    }

    /// Decompiled virtual method #19 @ RVA 0x00451f40 (10 instructions, 2 basic blocks)
    pub fn method_19(&mut self) {
    }

    /// Decompiled virtual method #20 @ RVA 0x00260c60 (87 instructions, 16 basic blocks)
    pub fn method_20(&mut self) {
    }

    /// Decompiled virtual method #21 @ RVA 0x00260de0 (66 instructions, 6 basic blocks)
    pub fn method_21(&mut self) {
        // Writes to field at offset +0x0038
        // Writes to field at offset +0x0048
    }

    /// Decompiled virtual method #22 @ RVA 0x00103e70 (2 instructions, 1 basic blocks)
    pub fn method_22(&mut self) {
    }

    /// Decompiled virtual method #23 @ RVA 0x00103e60 (2 instructions, 1 basic blocks)
    pub fn method_23(&mut self) {
    }

    /// Decompiled virtual method #24 @ RVA 0x00103e60 (2 instructions, 1 basic blocks)
    pub fn method_24(&mut self) {
    }

    /// Decompiled virtual method #25 @ RVA 0x00103e70 (2 instructions, 1 basic blocks)
    pub fn method_25(&mut self) {
    }

    /// Decompiled virtual method #26 @ RVA 0x00103e70 (2 instructions, 1 basic blocks)
    pub fn method_26(&mut self) {
    }

    /// Decompiled virtual method #27 @ RVA 0x00103e60 (2 instructions, 1 basic blocks)
    pub fn method_27(&mut self) {
    }

    /// Decompiled virtual method #28 @ RVA 0x00103e60 (2 instructions, 1 basic blocks)
    pub fn method_28(&mut self) {
    }

    /// Decompiled virtual method #29 @ RVA 0x00103e60 (2 instructions, 1 basic blocks)
    pub fn method_29(&mut self) {
    }

    /// Decompiled virtual method #30 @ RVA 0x002608f0 (111 instructions, 21 basic blocks)
    pub fn method_30(&mut self) {
    }

    /// Decompiled virtual method #31 @ RVA 0x00260b20 (70 instructions, 7 basic blocks)
    pub fn method_31(&mut self) {
    }

    /// Decompiled virtual method #32 @ RVA 0x000d3090 (1 instructions, 1 basic blocks)
    pub fn method_32(&mut self) {
    }

    /// Decompiled virtual method #33 @ RVA 0x00103e70 (2 instructions, 1 basic blocks)
    pub fn method_33(&mut self) {
    }

    /// Decompiled virtual method #34 @ RVA 0x00103e70 (2 instructions, 1 basic blocks)
    pub fn method_34(&mut self) {
    }

    /// Decompiled virtual method #35 @ RVA 0x00103e80 (2 instructions, 1 basic blocks)
    pub fn method_35(&mut self) {
    }

    /// Decompiled virtual method #36 @ RVA 0x00103e60 (2 instructions, 1 basic blocks)
    pub fn method_36(&mut self) {
    }

}

impl CcClass for ccBikeCustomMenu {
    fn class_name(&self) -> &'static str { Self::CLASS_NAME }
    fn vtable_rva(&self) -> u32 { Self::VTABLE_RVA }
    fn virtual_method_count(&self) -> usize { Self::METHOD_COUNT }
    fn base_classes(&self) -> &'static [&'static str] { Self::BASE_CLASSES }
}

/// Decompiled CC2 class: `ccBikeDisp` (VTable RVA: 0x00895148, 12 virtual methods, 22 recovered fields)
/// Inherits: MoveAnmPos
#[derive(Debug, Clone, Default)]
pub struct ccBikeDisp {
    pub vtable_rva: u32,
    pub field_0x0000: u64,
    pub field_0x0008: f32,
    pub field_0x000c: f32,
    pub field_0x0010: u32,
    pub field_0x0020: u32,
    pub field_0x0024: u8,
    pub field_0x0026: u8,
    pub field_0x0070: f32,
    pub field_0x0074: f32,
    pub field_0x0078: f32,
    pub field_0x007c: f32,
    pub field_0x0080: f32,
    pub field_0x0084: f32,
    pub field_0x0088: u32,
    pub field_0x00a2: u16,
    pub field_0x00c0: u32,
    pub field_0x00c4: f32,
    pub field_0x00e0: u16,
    pub field_0x010e: u8,
    pub field_0x0360: u32,
    pub field_0x0368: u8,
    pub field_0x0398: [f32; 4],
}

impl ccBikeDisp {
    pub const CLASS_NAME: &'static str = "ccBikeDisp";
    pub const VTABLE_RVA: u32 = 0x00895148;
    pub const METHOD_COUNT: usize = 12;
    pub const BASE_CLASSES: &'static [&'static str] = &["MoveAnmPos"];
    pub const VIRTUAL_METHODS: &'static [u64] = &[0x0000000180263970, 0x00000001800d3090, 0x0000000180263f00, 0x0000000180264340, 0x000000018057b520, 0x000000018057b450, 0x000000018025db10, 0x000000018057b5a0, 0x0000000180103f70, 0x000000018057b440, 0x000000018025db20, 0x0000000180103f80];

    pub fn new() -> Self {
        Self {
            vtable_rva: Self::VTABLE_RVA,
            field_0x0000: Default::default(),
            field_0x0008: Default::default(),
            field_0x000c: Default::default(),
            field_0x0010: Default::default(),
            field_0x0020: Default::default(),
            field_0x0024: Default::default(),
            field_0x0026: Default::default(),
            field_0x0070: Default::default(),
            field_0x0074: Default::default(),
            field_0x0078: Default::default(),
            field_0x007c: Default::default(),
            field_0x0080: Default::default(),
            field_0x0084: Default::default(),
            field_0x0088: Default::default(),
            field_0x00a2: Default::default(),
            field_0x00c0: Default::default(),
            field_0x00c4: Default::default(),
            field_0x00e0: Default::default(),
            field_0x010e: Default::default(),
            field_0x0360: Default::default(),
            field_0x0368: Default::default(),
            field_0x0398: Default::default(),
        }
    }

    /// Decompiled virtual method #0 @ RVA 0x00263970 (127 instructions, 23 basic blocks)
    pub fn method_00(&mut self) {
        // Writes to field at offset +0x00e0
        // Writes to field at offset +0x010e
    }

    /// Decompiled virtual method #1 @ RVA 0x000d3090 (1 instructions, 1 basic blocks)
    pub fn method_01(&mut self) {
    }

    /// Decompiled virtual method #2 @ RVA 0x00263f00 (98 instructions, 30 basic blocks)
    pub fn method_02(&mut self) {
        // Writes to field at offset +0x00c0
        // Writes to field at offset +0x00c4
        // Writes to field at offset +0x00c4
    }

    /// Decompiled virtual method #3 @ RVA 0x00264340 (94 instructions, 8 basic blocks)
    pub fn method_03(&mut self) {
        // Writes to field at offset +0x0360
        // Writes to field at offset +0x0368
        // Writes to field at offset +0x0398
    }

    /// Decompiled virtual method #4 @ RVA 0x0057b520 (30 instructions, 4 basic blocks)
    pub fn method_04(&mut self) {
    }

    /// Decompiled virtual method #5 @ RVA 0x0057b450 (90 instructions, 7 basic blocks)
    pub fn method_05(&mut self) {
    }

    /// Decompiled virtual method #6 @ RVA 0x0025db10 (3 instructions, 1 basic blocks)
    pub fn method_06(&mut self) {
        // Writes to field at offset +0x0070
        // Writes to field at offset +0x0074
    }

    /// Decompiled virtual method #7 @ RVA 0x0057b5a0 (7 instructions, 1 basic blocks)
    pub fn method_07(&mut self) {
        // Writes to field at offset +0x000c
        // Writes to field at offset +0x0026
    }

    /// Decompiled virtual method #8 @ RVA 0x00103f70 (2 instructions, 1 basic blocks)
    pub fn method_08(&mut self) {
        // Writes to field at offset +0x0020
    }

    /// Decompiled virtual method #9 @ RVA 0x0057b440 (98 instructions, 8 basic blocks)
    pub fn method_09(&mut self) {
        // Writes to field at offset +0x0008
    }

    /// Decompiled virtual method #10 @ RVA 0x0025db20 (5 instructions, 1 basic blocks)
    pub fn method_10(&mut self) {
    }

    /// Decompiled virtual method #11 @ RVA 0x00103f80 (3 instructions, 1 basic blocks)
    pub fn method_11(&mut self) {
        // Writes to field at offset +0x0024
    }

}

impl CcClass for ccBikeDisp {
    fn class_name(&self) -> &'static str { Self::CLASS_NAME }
    fn vtable_rva(&self) -> u32 { Self::VTABLE_RVA }
    fn virtual_method_count(&self) -> usize { Self::METHOD_COUNT }
    fn base_classes(&self) -> &'static [&'static str] { Self::BASE_CLASSES }
}

/// Decompiled CC2 class: `ccBikePartsBuyMenu` (VTable RVA: 0x008958f0, 45 virtual methods, 15 recovered fields)
/// Inherits: ccBuyMenuBase, ccGameMenuCtrlBase, ccGameMenuObj
#[derive(Debug, Clone, Default)]
pub struct ccBikePartsBuyMenu {
    pub vtable_rva: u32,
    pub field_0x0000: u64,
    pub field_0x0008: u32,
    pub field_0x000c: u32,
    pub field_0x0018: u32,
    pub field_0x0030: u64,
    pub field_0x0048: u64,
    pub field_0x0050: u64,
    pub field_0x0060: u64,
    pub field_0x0090: u64,
    pub field_0x009c: u32,
    pub field_0x00dc: u32,
    pub field_0x00f4: u32,
    pub field_0x00fc: u32,
    pub field_0x0188: u64,
    pub field_0x02b0: u64,
}

impl ccBikePartsBuyMenu {
    pub const CLASS_NAME: &'static str = "ccBikePartsBuyMenu";
    pub const VTABLE_RVA: u32 = 0x008958f0;
    pub const METHOD_COUNT: usize = 45;
    pub const BASE_CLASSES: &'static [&'static str] = &["ccBuyMenuBase", "ccGameMenuCtrlBase", "ccGameMenuObj"];
    pub const VIRTUAL_METHODS: &'static [u64] = &[0x000000018025db30, 0x00000001804520b0, 0x0000000180452130, 0x0000000180451fb0, 0x0000000180451ff0, 0x0000000180452030, 0x0000000180452070, 0x0000000180103e90, 0x0000000180103ea0, 0x0000000180103e60, 0x000000018045bb30, 0x0000000180451d90, 0x00000001801e9580, 0x0000000180265cd0, 0x0000000180103e60, 0x0000000180451dc0, 0x0000000180451e30, 0x0000000180451ea0, 0x0000000180451ee0, 0x0000000180451f40, 0x0000000180265f00, 0x0000000180266120, 0x00000001801ee9b0, 0x0000000180103e60, 0x0000000180103e60, 0x0000000180103e70, 0x0000000180103e70, 0x0000000180103e60, 0x0000000180103e60, 0x0000000180103e60, 0x0000000180266200, 0x0000000180266430, 0x00000001800d3090, 0x0000000180103e70, 0x0000000180103e70, 0x0000000180103e70, 0x0000000180103e60, 0x0000000180266970, 0x0000000180266ae0, 0x0000000180266bf0, 0x0000000180266b30, 0x0000000180266ba0, 0x0000000180266c40, 0x0000000180266c90, 0x0000000180266de0];

    pub fn new() -> Self {
        Self {
            vtable_rva: Self::VTABLE_RVA,
            field_0x0000: Default::default(),
            field_0x0008: Default::default(),
            field_0x000c: Default::default(),
            field_0x0018: Default::default(),
            field_0x0030: Default::default(),
            field_0x0048: Default::default(),
            field_0x0050: Default::default(),
            field_0x0060: Default::default(),
            field_0x0090: Default::default(),
            field_0x009c: Default::default(),
            field_0x00dc: Default::default(),
            field_0x00f4: Default::default(),
            field_0x00fc: Default::default(),
            field_0x0188: Default::default(),
            field_0x02b0: Default::default(),
        }
    }

    /// Decompiled virtual method #0 @ RVA 0x0025db30 (18 instructions, 3 basic blocks)
    pub fn method_00(&mut self) {
        // Writes to field at offset +0x0000
    }

    /// Decompiled virtual method #1 @ RVA 0x004520b0 (10 instructions, 2 basic blocks)
    pub fn method_01(&mut self) {
    }

    /// Decompiled virtual method #2 @ RVA 0x00452130 (35 instructions, 9 basic blocks)
    pub fn method_02(&mut self) {
    }

    /// Decompiled virtual method #3 @ RVA 0x00451fb0 (118 instructions, 2 basic blocks)
    pub fn method_03(&mut self) {
    }

    /// Decompiled virtual method #4 @ RVA 0x00451ff0 (91 instructions, 2 basic blocks)
    pub fn method_04(&mut self) {
    }

    /// Decompiled virtual method #5 @ RVA 0x00452030 (64 instructions, 2 basic blocks)
    pub fn method_05(&mut self) {
    }

    /// Decompiled virtual method #6 @ RVA 0x00452070 (37 instructions, 2 basic blocks)
    pub fn method_06(&mut self) {
    }

    /// Decompiled virtual method #7 @ RVA 0x00103e90 (2 instructions, 1 basic blocks)
    pub fn method_07(&mut self) {
    }

    /// Decompiled virtual method #8 @ RVA 0x00103ea0 (9 instructions, 1 basic blocks)
    pub fn method_08(&mut self) {
        // Writes to field at offset +0x0008
    }

    /// Decompiled virtual method #9 @ RVA 0x00103e60 (2 instructions, 1 basic blocks)
    pub fn method_09(&mut self) {
    }

    /// Decompiled virtual method #10 @ RVA 0x0045bb30 (16 instructions, 2 basic blocks)
    pub fn method_10(&mut self) {
    }

    /// Decompiled virtual method #11 @ RVA 0x00451d90 (24 instructions, 2 basic blocks)
    pub fn method_11(&mut self) {
    }

    /// Decompiled virtual method #12 @ RVA 0x001e9580 (53 instructions, 12 basic blocks)
    pub fn method_12(&mut self) {
    }

    /// Decompiled virtual method #13 @ RVA 0x00265cd0 (108 instructions, 6 basic blocks)
    pub fn method_13(&mut self) {
    }

    /// Decompiled virtual method #14 @ RVA 0x00103e60 (2 instructions, 1 basic blocks)
    pub fn method_14(&mut self) {
    }

    /// Decompiled virtual method #15 @ RVA 0x00451dc0 (10 instructions, 2 basic blocks)
    pub fn method_15(&mut self) {
    }

    /// Decompiled virtual method #16 @ RVA 0x00451e30 (10 instructions, 2 basic blocks)
    pub fn method_16(&mut self) {
    }

    /// Decompiled virtual method #17 @ RVA 0x00451ea0 (31 instructions, 2 basic blocks)
    pub fn method_17(&mut self) {
        // Writes to field at offset +0x0018
    }

    /// Decompiled virtual method #18 @ RVA 0x00451ee0 (10 instructions, 2 basic blocks)
    pub fn method_18(&mut self) {
    }

    /// Decompiled virtual method #19 @ RVA 0x00451f40 (10 instructions, 2 basic blocks)
    pub fn method_19(&mut self) {
    }

    /// Decompiled virtual method #20 @ RVA 0x00265f00 (117 instructions, 12 basic blocks)
    pub fn method_20(&mut self) {
        // Writes to field at offset +0x02b0
    }

    /// Decompiled virtual method #21 @ RVA 0x00266120 (50 instructions, 8 basic blocks)
    pub fn method_21(&mut self) {
        // Writes to field at offset +0x0008
        // Writes to field at offset +0x000c
    }

    /// Decompiled virtual method #22 @ RVA 0x001ee9b0 (15 instructions, 1 basic blocks)
    pub fn method_22(&mut self) {
    }

    /// Decompiled virtual method #23 @ RVA 0x00103e60 (2 instructions, 1 basic blocks)
    pub fn method_23(&mut self) {
    }

    /// Decompiled virtual method #24 @ RVA 0x00103e60 (2 instructions, 1 basic blocks)
    pub fn method_24(&mut self) {
    }

    /// Decompiled virtual method #25 @ RVA 0x00103e70 (2 instructions, 1 basic blocks)
    pub fn method_25(&mut self) {
    }

    /// Decompiled virtual method #26 @ RVA 0x00103e70 (2 instructions, 1 basic blocks)
    pub fn method_26(&mut self) {
    }

    /// Decompiled virtual method #27 @ RVA 0x00103e60 (2 instructions, 1 basic blocks)
    pub fn method_27(&mut self) {
    }

    /// Decompiled virtual method #28 @ RVA 0x00103e60 (2 instructions, 1 basic blocks)
    pub fn method_28(&mut self) {
    }

    /// Decompiled virtual method #29 @ RVA 0x00103e60 (2 instructions, 1 basic blocks)
    pub fn method_29(&mut self) {
    }

    /// Decompiled virtual method #30 @ RVA 0x00266200 (105 instructions, 19 basic blocks)
    pub fn method_30(&mut self) {
    }

    /// Decompiled virtual method #31 @ RVA 0x00266430 (90 instructions, 5 basic blocks)
    pub fn method_31(&mut self) {
    }

    /// Decompiled virtual method #32 @ RVA 0x000d3090 (1 instructions, 1 basic blocks)
    pub fn method_32(&mut self) {
    }

    /// Decompiled virtual method #33 @ RVA 0x00103e70 (2 instructions, 1 basic blocks)
    pub fn method_33(&mut self) {
    }

    /// Decompiled virtual method #34 @ RVA 0x00103e70 (2 instructions, 1 basic blocks)
    pub fn method_34(&mut self) {
    }

    /// Decompiled virtual method #35 @ RVA 0x00103e70 (2 instructions, 1 basic blocks)
    pub fn method_35(&mut self) {
    }

    /// Decompiled virtual method #36 @ RVA 0x00103e60 (2 instructions, 1 basic blocks)
    pub fn method_36(&mut self) {
    }

    /// Decompiled virtual method #37 @ RVA 0x00266970 (103 instructions, 19 basic blocks)
    pub fn method_37(&mut self) {
    }

    /// Decompiled virtual method #38 @ RVA 0x00266ae0 (144 instructions, 22 basic blocks)
    pub fn method_38(&mut self) {
    }

    /// Decompiled virtual method #39 @ RVA 0x00266bf0 (117 instructions, 19 basic blocks)
    pub fn method_39(&mut self) {
    }

    /// Decompiled virtual method #40 @ RVA 0x00266b30 (142 instructions, 18 basic blocks)
    pub fn method_40(&mut self) {
    }

    /// Decompiled virtual method #41 @ RVA 0x00266ba0 (128 instructions, 16 basic blocks)
    pub fn method_41(&mut self) {
    }

    /// Decompiled virtual method #42 @ RVA 0x00266c40 (97 instructions, 14 basic blocks)
    pub fn method_42(&mut self) {
    }

    /// Decompiled virtual method #43 @ RVA 0x00266c90 (77 instructions, 9 basic blocks)
    pub fn method_43(&mut self) {
    }

    /// Decompiled virtual method #44 @ RVA 0x00266de0 (68 instructions, 6 basic blocks)
    pub fn method_44(&mut self) {
    }

}

impl CcClass for ccBikePartsBuyMenu {
    fn class_name(&self) -> &'static str { Self::CLASS_NAME }
    fn vtable_rva(&self) -> u32 { Self::VTABLE_RVA }
    fn virtual_method_count(&self) -> usize { Self::METHOD_COUNT }
    fn base_classes(&self) -> &'static [&'static str] { Self::BASE_CLASSES }
}

/// Decompiled CC2 class: `ccBikePartsCategoryListWnd` (VTable RVA: 0x008955b8, 64 virtual methods, 38 recovered fields)
/// Inherits: ListWnd, MoveAnmPos
#[derive(Debug, Clone, Default)]
pub struct ccBikePartsCategoryListWnd {
    pub vtable_rva: u32,
    pub field_0x0000: u64,
    pub field_0x000c: f32,
    pub field_0x0018: u64,
    pub field_0x0020: u32,
    pub field_0x0024: u8,
    pub field_0x0026: u8,
    pub field_0x0028: u32,
    pub field_0x002c: u32,
    pub field_0x0030: u64,
    pub field_0x0038: u32,
    pub field_0x0048: u32,
    pub field_0x0060: u32,
    pub field_0x0064: u32,
    pub field_0x0068: f32,
    pub field_0x0070: u32,
    pub field_0x0074: u32,
    pub field_0x0078: f32,
    pub field_0x007c: f32,
    pub field_0x008c: u32,
    pub field_0x0094: u64,
    pub field_0x0098: u32,
    pub field_0x009c: u64,
    pub field_0x00a4: u32,
    pub field_0x00ac: f32,
    pub field_0x00b0: u32,
    pub field_0x00b4: u32,
    pub field_0x00c0: u32,
    pub field_0x00c8: u64,
    pub field_0x00d0: u64,
    pub field_0x00d8: u64,
    pub field_0x00f0: u64,
    pub field_0x00f8: u32,
    pub field_0x010c: u8,
    pub field_0x010d: u8,
    pub field_0x0110: u32,
    pub field_0x0118: u64,
    pub field_0x0180: u64,
    pub field_0x2128: u64,
}

impl ccBikePartsCategoryListWnd {
    pub const CLASS_NAME: &'static str = "ccBikePartsCategoryListWnd";
    pub const VTABLE_RVA: u32 = 0x008955b8;
    pub const METHOD_COUNT: usize = 64;
    pub const BASE_CLASSES: &'static [&'static str] = &["ListWnd", "MoveAnmPos"];
    pub const VIRTUAL_METHODS: &'static [u64] = &[0x000000018025ef30, 0x0000000180109c20, 0x000000018025f060, 0x000000018025f1c0, 0x000000018057b520, 0x000000018057b450, 0x000000018010c440, 0x000000018057b5a0, 0x0000000180103f70, 0x000000018025f740, 0x000000018010cf30, 0x0000000180103f80, 0x0000000180109520, 0x00000001800d3090, 0x000000018010a4e0, 0x000000018010a100, 0x00000001800d3090, 0x00000001800d3090, 0x00000001800d3090, 0x000000018010a520, 0x000000018025f1f0, 0x00000001800fe200, 0x000000018010a590, 0x000000018010a670, 0x000000018010a680, 0x000000018010a8d0, 0x000000018010a910, 0x000000018010a690, 0x000000018010cfd0, 0x00000001800d3090, 0x00000001800d3090, 0x00000001800d3090, 0x00000001800d3090, 0x00000001800d3090, 0x00000001800fe200, 0x00000001800fe200, 0x00000001800fe200, 0x00000001800fe200, 0x00000001800fe200, 0x00000001800fe200, 0x00000001800fe200, 0x00000001800fe200, 0x00000001800fe200, 0x00000001800d3090, 0x000000018025ee30, 0x000000018025ef20, 0x000000018025f320, 0x000000018025f440, 0x0000000180109fa0, 0x000000018010a020, 0x000000018010a090, 0x000000018010a460, 0x000000018010b660, 0x000000018010bb20, 0x000000018010bf60, 0x000000018010c090, 0x000000018010c230, 0x000000018010c230, 0x000000018010c340, 0x000000018010c2d0, 0x000000018010b3b0, 0x000000018010b5b0, 0x000000018010ce40, 0x000000018010c510];

    pub fn new() -> Self {
        Self {
            vtable_rva: Self::VTABLE_RVA,
            field_0x0000: Default::default(),
            field_0x000c: Default::default(),
            field_0x0018: Default::default(),
            field_0x0020: Default::default(),
            field_0x0024: Default::default(),
            field_0x0026: Default::default(),
            field_0x0028: Default::default(),
            field_0x002c: Default::default(),
            field_0x0030: Default::default(),
            field_0x0038: Default::default(),
            field_0x0048: Default::default(),
            field_0x0060: Default::default(),
            field_0x0064: Default::default(),
            field_0x0068: Default::default(),
            field_0x0070: Default::default(),
            field_0x0074: Default::default(),
            field_0x0078: Default::default(),
            field_0x007c: Default::default(),
            field_0x008c: Default::default(),
            field_0x0094: Default::default(),
            field_0x0098: Default::default(),
            field_0x009c: Default::default(),
            field_0x00a4: Default::default(),
            field_0x00ac: Default::default(),
            field_0x00b0: Default::default(),
            field_0x00b4: Default::default(),
            field_0x00c0: Default::default(),
            field_0x00c8: Default::default(),
            field_0x00d0: Default::default(),
            field_0x00d8: Default::default(),
            field_0x00f0: Default::default(),
            field_0x00f8: Default::default(),
            field_0x010c: Default::default(),
            field_0x010d: Default::default(),
            field_0x0110: Default::default(),
            field_0x0118: Default::default(),
            field_0x0180: Default::default(),
            field_0x2128: Default::default(),
        }
    }

    /// Decompiled virtual method #0 @ RVA 0x0025ef30 (102 instructions, 10 basic blocks)
    pub fn method_00(&mut self) {
    }

    /// Decompiled virtual method #1 @ RVA 0x00109c20 (130 instructions, 13 basic blocks)
    pub fn method_01(&mut self) {
        // Writes to field at offset +0x0118
    }

    /// Decompiled virtual method #2 @ RVA 0x0025f060 (75 instructions, 7 basic blocks)
    pub fn method_02(&mut self) {
    }

    /// Decompiled virtual method #3 @ RVA 0x0025f1c0 (84 instructions, 6 basic blocks)
    pub fn method_03(&mut self) {
    }

    /// Decompiled virtual method #4 @ RVA 0x0057b520 (30 instructions, 4 basic blocks)
    pub fn method_04(&mut self) {
    }

    /// Decompiled virtual method #5 @ RVA 0x0057b450 (90 instructions, 7 basic blocks)
    pub fn method_05(&mut self) {
    }

    /// Decompiled virtual method #6 @ RVA 0x0010c440 (24 instructions, 3 basic blocks)
    pub fn method_06(&mut self) {
    }

    /// Decompiled virtual method #7 @ RVA 0x0057b5a0 (7 instructions, 1 basic blocks)
    pub fn method_07(&mut self) {
        // Writes to field at offset +0x000c
        // Writes to field at offset +0x0026
    }

    /// Decompiled virtual method #8 @ RVA 0x00103f70 (2 instructions, 1 basic blocks)
    pub fn method_08(&mut self) {
        // Writes to field at offset +0x0020
    }

    /// Decompiled virtual method #9 @ RVA 0x0025f740 (37 instructions, 8 basic blocks)
    pub fn method_09(&mut self) {
    }

    /// Decompiled virtual method #10 @ RVA 0x0010cf30 (8 instructions, 3 basic blocks)
    pub fn method_10(&mut self) {
    }

    /// Decompiled virtual method #11 @ RVA 0x00103f80 (3 instructions, 1 basic blocks)
    pub fn method_11(&mut self) {
        // Writes to field at offset +0x0024
    }

    /// Decompiled virtual method #12 @ RVA 0x00109520 (7 instructions, 1 basic blocks)
    pub fn method_12(&mut self) {
        // Writes to field at offset +0x0094
        // Writes to field at offset +0x009c
        // Writes to field at offset +0x00a4
        // Writes to field at offset +0x00b4
        // Writes to field at offset +0x00c0
    }

    /// Decompiled virtual method #13 @ RVA 0x000d3090 (1 instructions, 1 basic blocks)
    pub fn method_13(&mut self) {
    }

    /// Decompiled virtual method #14 @ RVA 0x0010a4e0 (13 instructions, 3 basic blocks)
    pub fn method_14(&mut self) {
        // Writes to field at offset +0x010c
    }

    /// Decompiled virtual method #15 @ RVA 0x0010a100 (47 instructions, 10 basic blocks)
    pub fn method_15(&mut self) {
    }

    /// Decompiled virtual method #16 @ RVA 0x000d3090 (1 instructions, 1 basic blocks)
    pub fn method_16(&mut self) {
    }

    /// Decompiled virtual method #17 @ RVA 0x000d3090 (1 instructions, 1 basic blocks)
    pub fn method_17(&mut self) {
    }

    /// Decompiled virtual method #18 @ RVA 0x000d3090 (1 instructions, 1 basic blocks)
    pub fn method_18(&mut self) {
    }

    /// Decompiled virtual method #19 @ RVA 0x0010a520 (26 instructions, 4 basic blocks)
    pub fn method_19(&mut self) {
        // Writes to field at offset +0x010c
    }

    /// Decompiled virtual method #20 @ RVA 0x0025f1f0 (63 instructions, 5 basic blocks)
    pub fn method_20(&mut self) {
    }

    /// Decompiled virtual method #21 @ RVA 0x000fe200 (2 instructions, 1 basic blocks)
    pub fn method_21(&mut self) {
    }

    /// Decompiled virtual method #22 @ RVA 0x0010a590 (52 instructions, 9 basic blocks)
    pub fn method_22(&mut self) {
        // Writes to field at offset +0x010c
    }

    /// Decompiled virtual method #23 @ RVA 0x0010a670 (113 instructions, 16 basic blocks)
    pub fn method_23(&mut self) {
        // Writes to field at offset +0x010d
    }

    /// Decompiled virtual method #24 @ RVA 0x0010a680 (110 instructions, 16 basic blocks)
    pub fn method_24(&mut self) {
        // Writes to field at offset +0x010d
    }

    /// Decompiled virtual method #25 @ RVA 0x0010a8d0 (117 instructions, 7 basic blocks)
    pub fn method_25(&mut self) {
        // Writes to field at offset +0x0110
    }

    /// Decompiled virtual method #26 @ RVA 0x0010a910 (112 instructions, 9 basic blocks)
    pub fn method_26(&mut self) {
    }

    /// Decompiled virtual method #27 @ RVA 0x0010a690 (107 instructions, 15 basic blocks)
    pub fn method_27(&mut self) {
        // Writes to field at offset +0x010d
    }

    /// Decompiled virtual method #28 @ RVA 0x0010cfd0 (25 instructions, 3 basic blocks)
    pub fn method_28(&mut self) {
        // Writes to field at offset +0x010c
    }

    /// Decompiled virtual method #29 @ RVA 0x000d3090 (1 instructions, 1 basic blocks)
    pub fn method_29(&mut self) {
    }

    /// Decompiled virtual method #30 @ RVA 0x000d3090 (1 instructions, 1 basic blocks)
    pub fn method_30(&mut self) {
    }

    /// Decompiled virtual method #31 @ RVA 0x000d3090 (1 instructions, 1 basic blocks)
    pub fn method_31(&mut self) {
    }

    /// Decompiled virtual method #32 @ RVA 0x000d3090 (1 instructions, 1 basic blocks)
    pub fn method_32(&mut self) {
    }

    /// Decompiled virtual method #33 @ RVA 0x000d3090 (1 instructions, 1 basic blocks)
    pub fn method_33(&mut self) {
    }

    /// Decompiled virtual method #34 @ RVA 0x000fe200 (2 instructions, 1 basic blocks)
    pub fn method_34(&mut self) {
    }

    /// Decompiled virtual method #35 @ RVA 0x000fe200 (2 instructions, 1 basic blocks)
    pub fn method_35(&mut self) {
    }

    /// Decompiled virtual method #36 @ RVA 0x000fe200 (2 instructions, 1 basic blocks)
    pub fn method_36(&mut self) {
    }

    /// Decompiled virtual method #37 @ RVA 0x000fe200 (2 instructions, 1 basic blocks)
    pub fn method_37(&mut self) {
    }

    /// Decompiled virtual method #38 @ RVA 0x000fe200 (2 instructions, 1 basic blocks)
    pub fn method_38(&mut self) {
    }

    /// Decompiled virtual method #39 @ RVA 0x000fe200 (2 instructions, 1 basic blocks)
    pub fn method_39(&mut self) {
    }

    /// Decompiled virtual method #40 @ RVA 0x000fe200 (2 instructions, 1 basic blocks)
    pub fn method_40(&mut self) {
    }

    /// Decompiled virtual method #41 @ RVA 0x000fe200 (2 instructions, 1 basic blocks)
    pub fn method_41(&mut self) {
    }

    /// Decompiled virtual method #42 @ RVA 0x000fe200 (2 instructions, 1 basic blocks)
    pub fn method_42(&mut self) {
    }

    /// Decompiled virtual method #43 @ RVA 0x000d3090 (1 instructions, 1 basic blocks)
    pub fn method_43(&mut self) {
    }

    /// Decompiled virtual method #44 @ RVA 0x0025ee30 (35 instructions, 5 basic blocks)
    pub fn method_44(&mut self) {
        // Writes to field at offset +0x0000
    }

    /// Decompiled virtual method #45 @ RVA 0x0025ef20 (111 instructions, 11 basic blocks)
    pub fn method_45(&mut self) {
    }

    /// Decompiled virtual method #46 @ RVA 0x0025f320 (129 instructions, 11 basic blocks)
    pub fn method_46(&mut self) {
        // Writes to field at offset +0x0038
        // Writes to field at offset +0x0048
    }

    /// Decompiled virtual method #47 @ RVA 0x0025f440 (126 instructions, 8 basic blocks)
    pub fn method_47(&mut self) {
        // Writes to field at offset +0x0038
        // Writes to field at offset +0x0048
    }

    /// Decompiled virtual method #48 @ RVA 0x00109fa0 (101 instructions, 7 basic blocks)
    pub fn method_48(&mut self) {
    }

    /// Decompiled virtual method #49 @ RVA 0x0010a020 (66 instructions, 5 basic blocks)
    pub fn method_49(&mut self) {
    }

    /// Decompiled virtual method #50 @ RVA 0x0010a090 (26 instructions, 3 basic blocks)
    pub fn method_50(&mut self) {
    }

    /// Decompiled virtual method #51 @ RVA 0x0010a460 (30 instructions, 3 basic blocks)
    pub fn method_51(&mut self) {
        // Writes to field at offset +0x0118
    }

    /// Decompiled virtual method #52 @ RVA 0x0010b660 (126 instructions, 20 basic blocks)
    pub fn method_52(&mut self) {
        // Writes to field at offset +0x009c
    }

    /// Decompiled virtual method #53 @ RVA 0x0010bb20 (131 instructions, 17 basic blocks)
    pub fn method_53(&mut self) {
    }

    /// Decompiled virtual method #54 @ RVA 0x0010bf60 (69 instructions, 11 basic blocks)
    pub fn method_54(&mut self) {
    }

    /// Decompiled virtual method #55 @ RVA 0x0010c090 (98 instructions, 13 basic blocks)
    pub fn method_55(&mut self) {
    }

    /// Decompiled virtual method #56 @ RVA 0x0010c230 (26 instructions, 2 basic blocks)
    pub fn method_56(&mut self) {
    }

    /// Decompiled virtual method #57 @ RVA 0x0010c230 (26 instructions, 2 basic blocks)
    pub fn method_57(&mut self) {
    }

    /// Decompiled virtual method #58 @ RVA 0x0010c340 (29 instructions, 3 basic blocks)
    pub fn method_58(&mut self) {
    }

    /// Decompiled virtual method #59 @ RVA 0x0010c2d0 (29 instructions, 3 basic blocks)
    pub fn method_59(&mut self) {
    }

    /// Decompiled virtual method #60 @ RVA 0x0010b3b0 (11 instructions, 1 basic blocks)
    pub fn method_60(&mut self) {
    }

    /// Decompiled virtual method #61 @ RVA 0x0010b5b0 (23 instructions, 2 basic blocks)
    pub fn method_61(&mut self) {
    }

    /// Decompiled virtual method #62 @ RVA 0x0010ce40 (59 instructions, 6 basic blocks)
    pub fn method_62(&mut self) {
    }

    /// Decompiled virtual method #63 @ RVA 0x0010c510 (52 instructions, 6 basic blocks)
    pub fn method_63(&mut self) {
        // Writes to field at offset +0x0030
        // Writes to field at offset +0x00d0
        // Writes to field at offset +0x00d8
        // Writes to field at offset +0x00f0
    }

}

impl CcClass for ccBikePartsCategoryListWnd {
    fn class_name(&self) -> &'static str { Self::CLASS_NAME }
    fn vtable_rva(&self) -> u32 { Self::VTABLE_RVA }
    fn virtual_method_count(&self) -> usize { Self::METHOD_COUNT }
    fn base_classes(&self) -> &'static [&'static str] { Self::BASE_CLASSES }
}

/// Decompiled CC2 class: `ccBikePartsListWnd` (VTable RVA: 0x008953b0, 64 virtual methods, 44 recovered fields)
/// Inherits: ListWnd, MoveAnmPos
#[derive(Debug, Clone, Default)]
pub struct ccBikePartsListWnd {
    pub vtable_rva: u32,
    pub field_0x0000: u64,
    pub field_0x000c: f32,
    pub field_0x0020: u32,
    pub field_0x0024: u8,
    pub field_0x0026: u8,
    pub field_0x0028: u64,
    pub field_0x0030: u64,
    pub field_0x0040: [f32; 4],
    pub field_0x0050: [f32; 4],
    pub field_0x0054: u32,
    pub field_0x0058: u32,
    pub field_0x005c: u32,
    pub field_0x0060: u32,
    pub field_0x0064: u32,
    pub field_0x0068: f32,
    pub field_0x0070: u32,
    pub field_0x0074: u32,
    pub field_0x0078: f32,
    pub field_0x007c: u32,
    pub field_0x0080: [f32; 4],
    pub field_0x008c: u32,
    pub field_0x0090: u32,
    pub field_0x0094: u64,
    pub field_0x0098: u32,
    pub field_0x009c: u64,
    pub field_0x00a4: u32,
    pub field_0x00ac: f32,
    pub field_0x00b0: u32,
    pub field_0x00b4: u32,
    pub field_0x00c0: u32,
    pub field_0x00c8: u64,
    pub field_0x00cc: u32,
    pub field_0x00d0: u64,
    pub field_0x00d4: u32,
    pub field_0x00d8: u64,
    pub field_0x00e0: u8,
    pub field_0x00e1: u8,
    pub field_0x00f0: u64,
    pub field_0x00f8: u32,
    pub field_0x010c: u8,
    pub field_0x010d: u8,
    pub field_0x0110: u32,
    pub field_0x0118: u64,
    pub field_0x0138: u64,
}

impl ccBikePartsListWnd {
    pub const CLASS_NAME: &'static str = "ccBikePartsListWnd";
    pub const VTABLE_RVA: u32 = 0x008953b0;
    pub const METHOD_COUNT: usize = 64;
    pub const BASE_CLASSES: &'static [&'static str] = &["ListWnd", "MoveAnmPos"];
    pub const VIRTUAL_METHODS: &'static [u64] = &[0x000000018025f940, 0x000000018025fb30, 0x000000018025ff50, 0x00000001802600b0, 0x000000018057b520, 0x000000018057b450, 0x000000018010c440, 0x000000018057b5a0, 0x0000000180103f70, 0x000000018025feb0, 0x000000018010cf30, 0x0000000180103f80, 0x0000000180109520, 0x00000001800d3090, 0x000000018010a4e0, 0x000000018010a100, 0x00000001800d3090, 0x00000001800d3090, 0x00000001800d3090, 0x000000018010a520, 0x00000001800fe200, 0x00000001800fe200, 0x000000018010a590, 0x000000018010a670, 0x000000018010a680, 0x000000018010a8d0, 0x000000018010a910, 0x000000018010a690, 0x000000018010cfd0, 0x00000001800d3090, 0x00000001800d3090, 0x00000001800d3090, 0x00000001800d3090, 0x00000001800d3090, 0x00000001800fe200, 0x00000001800fe200, 0x00000001800fe200, 0x00000001800fe200, 0x00000001800fe200, 0x00000001800fe200, 0x00000001800fe200, 0x00000001800fe200, 0x00000001800fe200, 0x00000001800d3090, 0x000000018025f860, 0x0000000180109650, 0x000000018025fb60, 0x000000018025fd90, 0x0000000180109fa0, 0x000000018010a020, 0x000000018010a090, 0x000000018010a460, 0x000000018010b660, 0x000000018010bb20, 0x000000018010bf60, 0x000000018010c090, 0x000000018010c230, 0x000000018010c230, 0x000000018010c340, 0x000000018010c2d0, 0x000000018010b3b0, 0x000000018010b5b0, 0x000000018010ce40, 0x000000018010c510];

    pub fn new() -> Self {
        Self {
            vtable_rva: Self::VTABLE_RVA,
            field_0x0000: Default::default(),
            field_0x000c: Default::default(),
            field_0x0020: Default::default(),
            field_0x0024: Default::default(),
            field_0x0026: Default::default(),
            field_0x0028: Default::default(),
            field_0x0030: Default::default(),
            field_0x0040: Default::default(),
            field_0x0050: Default::default(),
            field_0x0054: Default::default(),
            field_0x0058: Default::default(),
            field_0x005c: Default::default(),
            field_0x0060: Default::default(),
            field_0x0064: Default::default(),
            field_0x0068: Default::default(),
            field_0x0070: Default::default(),
            field_0x0074: Default::default(),
            field_0x0078: Default::default(),
            field_0x007c: Default::default(),
            field_0x0080: Default::default(),
            field_0x008c: Default::default(),
            field_0x0090: Default::default(),
            field_0x0094: Default::default(),
            field_0x0098: Default::default(),
            field_0x009c: Default::default(),
            field_0x00a4: Default::default(),
            field_0x00ac: Default::default(),
            field_0x00b0: Default::default(),
            field_0x00b4: Default::default(),
            field_0x00c0: Default::default(),
            field_0x00c8: Default::default(),
            field_0x00cc: Default::default(),
            field_0x00d0: Default::default(),
            field_0x00d4: Default::default(),
            field_0x00d8: Default::default(),
            field_0x00e0: Default::default(),
            field_0x00e1: Default::default(),
            field_0x00f0: Default::default(),
            field_0x00f8: Default::default(),
            field_0x010c: Default::default(),
            field_0x010d: Default::default(),
            field_0x0110: Default::default(),
            field_0x0118: Default::default(),
            field_0x0138: Default::default(),
        }
    }

    /// Decompiled virtual method #0 @ RVA 0x0025f940 (97 instructions, 4 basic blocks)
    pub fn method_00(&mut self) {
    }

    /// Decompiled virtual method #1 @ RVA 0x0025fb30 (12 instructions, 3 basic blocks)
    pub fn method_01(&mut self) {
    }

    /// Decompiled virtual method #2 @ RVA 0x0025ff50 (75 instructions, 7 basic blocks)
    pub fn method_02(&mut self) {
    }

    /// Decompiled virtual method #3 @ RVA 0x002600b0 (91 instructions, 13 basic blocks)
    pub fn method_03(&mut self) {
    }

    /// Decompiled virtual method #4 @ RVA 0x0057b520 (30 instructions, 4 basic blocks)
    pub fn method_04(&mut self) {
    }

    /// Decompiled virtual method #5 @ RVA 0x0057b450 (90 instructions, 7 basic blocks)
    pub fn method_05(&mut self) {
    }

    /// Decompiled virtual method #6 @ RVA 0x0010c440 (24 instructions, 3 basic blocks)
    pub fn method_06(&mut self) {
    }

    /// Decompiled virtual method #7 @ RVA 0x0057b5a0 (7 instructions, 1 basic blocks)
    pub fn method_07(&mut self) {
        // Writes to field at offset +0x000c
        // Writes to field at offset +0x0026
    }

    /// Decompiled virtual method #8 @ RVA 0x00103f70 (2 instructions, 1 basic blocks)
    pub fn method_08(&mut self) {
        // Writes to field at offset +0x0020
    }

    /// Decompiled virtual method #9 @ RVA 0x0025feb0 (37 instructions, 8 basic blocks)
    pub fn method_09(&mut self) {
    }

    /// Decompiled virtual method #10 @ RVA 0x0010cf30 (8 instructions, 3 basic blocks)
    pub fn method_10(&mut self) {
    }

    /// Decompiled virtual method #11 @ RVA 0x00103f80 (3 instructions, 1 basic blocks)
    pub fn method_11(&mut self) {
        // Writes to field at offset +0x0024
    }

    /// Decompiled virtual method #12 @ RVA 0x00109520 (7 instructions, 1 basic blocks)
    pub fn method_12(&mut self) {
        // Writes to field at offset +0x0094
        // Writes to field at offset +0x009c
        // Writes to field at offset +0x00a4
        // Writes to field at offset +0x00b4
        // Writes to field at offset +0x00c0
    }

    /// Decompiled virtual method #13 @ RVA 0x000d3090 (1 instructions, 1 basic blocks)
    pub fn method_13(&mut self) {
    }

    /// Decompiled virtual method #14 @ RVA 0x0010a4e0 (13 instructions, 3 basic blocks)
    pub fn method_14(&mut self) {
        // Writes to field at offset +0x010c
    }

    /// Decompiled virtual method #15 @ RVA 0x0010a100 (47 instructions, 10 basic blocks)
    pub fn method_15(&mut self) {
    }

    /// Decompiled virtual method #16 @ RVA 0x000d3090 (1 instructions, 1 basic blocks)
    pub fn method_16(&mut self) {
    }

    /// Decompiled virtual method #17 @ RVA 0x000d3090 (1 instructions, 1 basic blocks)
    pub fn method_17(&mut self) {
    }

    /// Decompiled virtual method #18 @ RVA 0x000d3090 (1 instructions, 1 basic blocks)
    pub fn method_18(&mut self) {
    }

    /// Decompiled virtual method #19 @ RVA 0x0010a520 (26 instructions, 4 basic blocks)
    pub fn method_19(&mut self) {
        // Writes to field at offset +0x010c
    }

    /// Decompiled virtual method #20 @ RVA 0x000fe200 (2 instructions, 1 basic blocks)
    pub fn method_20(&mut self) {
    }

    /// Decompiled virtual method #21 @ RVA 0x000fe200 (2 instructions, 1 basic blocks)
    pub fn method_21(&mut self) {
    }

    /// Decompiled virtual method #22 @ RVA 0x0010a590 (52 instructions, 9 basic blocks)
    pub fn method_22(&mut self) {
        // Writes to field at offset +0x010c
    }

    /// Decompiled virtual method #23 @ RVA 0x0010a670 (113 instructions, 16 basic blocks)
    pub fn method_23(&mut self) {
        // Writes to field at offset +0x010d
    }

    /// Decompiled virtual method #24 @ RVA 0x0010a680 (110 instructions, 16 basic blocks)
    pub fn method_24(&mut self) {
        // Writes to field at offset +0x010d
    }

    /// Decompiled virtual method #25 @ RVA 0x0010a8d0 (117 instructions, 7 basic blocks)
    pub fn method_25(&mut self) {
        // Writes to field at offset +0x0110
    }

    /// Decompiled virtual method #26 @ RVA 0x0010a910 (112 instructions, 9 basic blocks)
    pub fn method_26(&mut self) {
    }

    /// Decompiled virtual method #27 @ RVA 0x0010a690 (107 instructions, 15 basic blocks)
    pub fn method_27(&mut self) {
        // Writes to field at offset +0x010d
    }

    /// Decompiled virtual method #28 @ RVA 0x0010cfd0 (25 instructions, 3 basic blocks)
    pub fn method_28(&mut self) {
        // Writes to field at offset +0x010c
    }

    /// Decompiled virtual method #29 @ RVA 0x000d3090 (1 instructions, 1 basic blocks)
    pub fn method_29(&mut self) {
    }

    /// Decompiled virtual method #30 @ RVA 0x000d3090 (1 instructions, 1 basic blocks)
    pub fn method_30(&mut self) {
    }

    /// Decompiled virtual method #31 @ RVA 0x000d3090 (1 instructions, 1 basic blocks)
    pub fn method_31(&mut self) {
    }

    /// Decompiled virtual method #32 @ RVA 0x000d3090 (1 instructions, 1 basic blocks)
    pub fn method_32(&mut self) {
    }

    /// Decompiled virtual method #33 @ RVA 0x000d3090 (1 instructions, 1 basic blocks)
    pub fn method_33(&mut self) {
    }

    /// Decompiled virtual method #34 @ RVA 0x000fe200 (2 instructions, 1 basic blocks)
    pub fn method_34(&mut self) {
    }

    /// Decompiled virtual method #35 @ RVA 0x000fe200 (2 instructions, 1 basic blocks)
    pub fn method_35(&mut self) {
    }

    /// Decompiled virtual method #36 @ RVA 0x000fe200 (2 instructions, 1 basic blocks)
    pub fn method_36(&mut self) {
    }

    /// Decompiled virtual method #37 @ RVA 0x000fe200 (2 instructions, 1 basic blocks)
    pub fn method_37(&mut self) {
    }

    /// Decompiled virtual method #38 @ RVA 0x000fe200 (2 instructions, 1 basic blocks)
    pub fn method_38(&mut self) {
    }

    /// Decompiled virtual method #39 @ RVA 0x000fe200 (2 instructions, 1 basic blocks)
    pub fn method_39(&mut self) {
    }

    /// Decompiled virtual method #40 @ RVA 0x000fe200 (2 instructions, 1 basic blocks)
    pub fn method_40(&mut self) {
    }

    /// Decompiled virtual method #41 @ RVA 0x000fe200 (2 instructions, 1 basic blocks)
    pub fn method_41(&mut self) {
    }

    /// Decompiled virtual method #42 @ RVA 0x000fe200 (2 instructions, 1 basic blocks)
    pub fn method_42(&mut self) {
    }

    /// Decompiled virtual method #43 @ RVA 0x000d3090 (1 instructions, 1 basic blocks)
    pub fn method_43(&mut self) {
    }

    /// Decompiled virtual method #44 @ RVA 0x0025f860 (16 instructions, 3 basic blocks)
    pub fn method_44(&mut self) {
    }

    /// Decompiled virtual method #45 @ RVA 0x00109650 (112 instructions, 17 basic blocks)
    pub fn method_45(&mut self) {
        // Writes to field at offset +0x0040
        // Writes to field at offset +0x0050
        // Writes to field at offset +0x0054
        // Writes to field at offset +0x0058
        // Writes to field at offset +0x005c
        // Writes to field at offset +0x0060
        // Writes to field at offset +0x0070
        // Writes to field at offset +0x0080
        // Writes to field at offset +0x0090
        // Writes to field at offset +0x00c8
        // Writes to field at offset +0x00cc
        // Writes to field at offset +0x00d0
        // Writes to field at offset +0x00d4
        // Writes to field at offset +0x00e0
        // Writes to field at offset +0x00e1
        // Writes to field at offset +0x00f8
    }

    /// Decompiled virtual method #46 @ RVA 0x0025fb60 (126 instructions, 23 basic blocks)
    pub fn method_46(&mut self) {
    }

    /// Decompiled virtual method #47 @ RVA 0x0025fd90 (116 instructions, 13 basic blocks)
    pub fn method_47(&mut self) {
    }

    /// Decompiled virtual method #48 @ RVA 0x00109fa0 (101 instructions, 7 basic blocks)
    pub fn method_48(&mut self) {
    }

    /// Decompiled virtual method #49 @ RVA 0x0010a020 (66 instructions, 5 basic blocks)
    pub fn method_49(&mut self) {
    }

    /// Decompiled virtual method #50 @ RVA 0x0010a090 (26 instructions, 3 basic blocks)
    pub fn method_50(&mut self) {
    }

    /// Decompiled virtual method #51 @ RVA 0x0010a460 (30 instructions, 3 basic blocks)
    pub fn method_51(&mut self) {
        // Writes to field at offset +0x0118
    }

    /// Decompiled virtual method #52 @ RVA 0x0010b660 (126 instructions, 20 basic blocks)
    pub fn method_52(&mut self) {
        // Writes to field at offset +0x009c
    }

    /// Decompiled virtual method #53 @ RVA 0x0010bb20 (131 instructions, 17 basic blocks)
    pub fn method_53(&mut self) {
    }

    /// Decompiled virtual method #54 @ RVA 0x0010bf60 (69 instructions, 11 basic blocks)
    pub fn method_54(&mut self) {
    }

    /// Decompiled virtual method #55 @ RVA 0x0010c090 (98 instructions, 13 basic blocks)
    pub fn method_55(&mut self) {
    }

    /// Decompiled virtual method #56 @ RVA 0x0010c230 (26 instructions, 2 basic blocks)
    pub fn method_56(&mut self) {
    }

    /// Decompiled virtual method #57 @ RVA 0x0010c230 (26 instructions, 2 basic blocks)
    pub fn method_57(&mut self) {
    }

    /// Decompiled virtual method #58 @ RVA 0x0010c340 (29 instructions, 3 basic blocks)
    pub fn method_58(&mut self) {
    }

    /// Decompiled virtual method #59 @ RVA 0x0010c2d0 (29 instructions, 3 basic blocks)
    pub fn method_59(&mut self) {
    }

    /// Decompiled virtual method #60 @ RVA 0x0010b3b0 (11 instructions, 1 basic blocks)
    pub fn method_60(&mut self) {
    }

    /// Decompiled virtual method #61 @ RVA 0x0010b5b0 (23 instructions, 2 basic blocks)
    pub fn method_61(&mut self) {
    }

    /// Decompiled virtual method #62 @ RVA 0x0010ce40 (59 instructions, 6 basic blocks)
    pub fn method_62(&mut self) {
    }

    /// Decompiled virtual method #63 @ RVA 0x0010c510 (52 instructions, 6 basic blocks)
    pub fn method_63(&mut self) {
        // Writes to field at offset +0x0030
        // Writes to field at offset +0x00d0
        // Writes to field at offset +0x00d8
        // Writes to field at offset +0x00f0
    }

}

impl CcClass for ccBikePartsListWnd {
    fn class_name(&self) -> &'static str { Self::CLASS_NAME }
    fn vtable_rva(&self) -> u32 { Self::VTABLE_RVA }
    fn virtual_method_count(&self) -> usize { Self::METHOD_COUNT }
    fn base_classes(&self) -> &'static [&'static str] { Self::BASE_CLASSES }
}

/// Decompiled CC2 class: `ccBikePartsParamDisp` (VTable RVA: 0x00895218, 12 virtual methods, 14 recovered fields)
/// Inherits: MoveAnmPos
#[derive(Debug, Clone, Default)]
pub struct ccBikePartsParamDisp {
    pub vtable_rva: u32,
    pub field_0x0000: u64,
    pub field_0x0008: f32,
    pub field_0x000c: f32,
    pub field_0x0020: u64,
    pub field_0x0024: u32,
    pub field_0x0026: u8,
    pub field_0x002c: u32,
    pub field_0x0038: u64,
    pub field_0x0040: u64,
    pub field_0x0058: f32,
    pub field_0x005c: f32,
    pub field_0x0068: f32,
    pub field_0x006c: f32,
    pub field_0x0094: f32,
}

impl ccBikePartsParamDisp {
    pub const CLASS_NAME: &'static str = "ccBikePartsParamDisp";
    pub const VTABLE_RVA: u32 = 0x00895218;
    pub const METHOD_COUNT: usize = 12;
    pub const BASE_CLASSES: &'static [&'static str] = &["MoveAnmPos"];
    pub const VIRTUAL_METHODS: &'static [u64] = &[0x00000001802625a0, 0x00000001800d3090, 0x0000000180262890, 0x0000000180262930, 0x0000000180262c40, 0x000000018057b450, 0x0000000180262c30, 0x000000018057b5a0, 0x0000000180103f70, 0x000000018057b440, 0x000000018025dae0, 0x0000000180103f80];

    pub fn new() -> Self {
        Self {
            vtable_rva: Self::VTABLE_RVA,
            field_0x0000: Default::default(),
            field_0x0008: Default::default(),
            field_0x000c: Default::default(),
            field_0x0020: Default::default(),
            field_0x0024: Default::default(),
            field_0x0026: Default::default(),
            field_0x002c: Default::default(),
            field_0x0038: Default::default(),
            field_0x0040: Default::default(),
            field_0x0058: Default::default(),
            field_0x005c: Default::default(),
            field_0x0068: Default::default(),
            field_0x006c: Default::default(),
            field_0x0094: Default::default(),
        }
    }

    /// Decompiled virtual method #0 @ RVA 0x002625a0 (130 instructions, 20 basic blocks)
    pub fn method_00(&mut self) {
    }

    /// Decompiled virtual method #1 @ RVA 0x000d3090 (1 instructions, 1 basic blocks)
    pub fn method_01(&mut self) {
    }

    /// Decompiled virtual method #2 @ RVA 0x00262890 (31 instructions, 5 basic blocks)
    pub fn method_02(&mut self) {
    }

    /// Decompiled virtual method #3 @ RVA 0x00262930 (116 instructions, 13 basic blocks)
    pub fn method_03(&mut self) {
        // Writes to field at offset +0x0020
        // Writes to field at offset +0x0024
        // Writes to field at offset +0x002c
    }

    /// Decompiled virtual method #4 @ RVA 0x00262c40 (4 instructions, 1 basic blocks)
    pub fn method_04(&mut self) {
    }

    /// Decompiled virtual method #5 @ RVA 0x0057b450 (90 instructions, 7 basic blocks)
    pub fn method_05(&mut self) {
    }

    /// Decompiled virtual method #6 @ RVA 0x00262c30 (3 instructions, 1 basic blocks)
    pub fn method_06(&mut self) {
        // Writes to field at offset +0x0068
        // Writes to field at offset +0x006c
    }

    /// Decompiled virtual method #7 @ RVA 0x0057b5a0 (7 instructions, 1 basic blocks)
    pub fn method_07(&mut self) {
        // Writes to field at offset +0x000c
        // Writes to field at offset +0x0026
    }

    /// Decompiled virtual method #8 @ RVA 0x00103f70 (2 instructions, 1 basic blocks)
    pub fn method_08(&mut self) {
        // Writes to field at offset +0x0020
    }

    /// Decompiled virtual method #9 @ RVA 0x0057b440 (98 instructions, 8 basic blocks)
    pub fn method_09(&mut self) {
        // Writes to field at offset +0x0008
    }

    /// Decompiled virtual method #10 @ RVA 0x0025dae0 (5 instructions, 1 basic blocks)
    pub fn method_10(&mut self) {
    }

    /// Decompiled virtual method #11 @ RVA 0x00103f80 (3 instructions, 1 basic blocks)
    pub fn method_11(&mut self) {
        // Writes to field at offset +0x0024
    }

}

impl CcClass for ccBikePartsParamDisp {
    fn class_name(&self) -> &'static str { Self::CLASS_NAME }
    fn vtable_rva(&self) -> u32 { Self::VTABLE_RVA }
    fn virtual_method_count(&self) -> usize { Self::METHOD_COUNT }
    fn base_classes(&self) -> &'static [&'static str] { Self::BASE_CLASSES }
}

/// Decompiled CC2 class: `ccBikePartsParamDispWnd` (VTable RVA: 0x008951b0, 12 virtual methods, 9 recovered fields)
/// Inherits: MoveAnmPos
#[derive(Debug, Clone, Default)]
pub struct ccBikePartsParamDispWnd {
    pub vtable_rva: u32,
    pub field_0x0000: u64,
    pub field_0x0008: f32,
    pub field_0x000c: f32,
    pub field_0x0020: u32,
    pub field_0x0024: u8,
    pub field_0x0026: u8,
    pub field_0x0038: u32,
    pub field_0x0060: f32,
    pub field_0x0064: f32,
}

impl ccBikePartsParamDispWnd {
    pub const CLASS_NAME: &'static str = "ccBikePartsParamDispWnd";
    pub const VTABLE_RVA: u32 = 0x008951b0;
    pub const METHOD_COUNT: usize = 12;
    pub const BASE_CLASSES: &'static [&'static str] = &["MoveAnmPos"];
    pub const VIRTUAL_METHODS: &'static [u64] = &[0x0000000180262c50, 0x00000001800d3090, 0x0000000180262db0, 0x0000000180262e90, 0x000000018057b520, 0x000000018057b450, 0x000000018025daf0, 0x000000018057b5a0, 0x0000000180103f70, 0x000000018057b440, 0x000000018025db00, 0x0000000180103f80];

    pub fn new() -> Self {
        Self {
            vtable_rva: Self::VTABLE_RVA,
            field_0x0000: Default::default(),
            field_0x0008: Default::default(),
            field_0x000c: Default::default(),
            field_0x0020: Default::default(),
            field_0x0024: Default::default(),
            field_0x0026: Default::default(),
            field_0x0038: Default::default(),
            field_0x0060: Default::default(),
            field_0x0064: Default::default(),
        }
    }

    /// Decompiled virtual method #0 @ RVA 0x00262c50 (88 instructions, 6 basic blocks)
    pub fn method_00(&mut self) {
    }

    /// Decompiled virtual method #1 @ RVA 0x000d3090 (1 instructions, 1 basic blocks)
    pub fn method_01(&mut self) {
    }

    /// Decompiled virtual method #2 @ RVA 0x00262db0 (54 instructions, 5 basic blocks)
    pub fn method_02(&mut self) {
    }

    /// Decompiled virtual method #3 @ RVA 0x00262e90 (24 instructions, 3 basic blocks)
    pub fn method_03(&mut self) {
    }

    /// Decompiled virtual method #4 @ RVA 0x0057b520 (30 instructions, 4 basic blocks)
    pub fn method_04(&mut self) {
    }

    /// Decompiled virtual method #5 @ RVA 0x0057b450 (90 instructions, 7 basic blocks)
    pub fn method_05(&mut self) {
    }

    /// Decompiled virtual method #6 @ RVA 0x0025daf0 (3 instructions, 1 basic blocks)
    pub fn method_06(&mut self) {
        // Writes to field at offset +0x0060
        // Writes to field at offset +0x0064
    }

    /// Decompiled virtual method #7 @ RVA 0x0057b5a0 (7 instructions, 1 basic blocks)
    pub fn method_07(&mut self) {
        // Writes to field at offset +0x000c
        // Writes to field at offset +0x0026
    }

    /// Decompiled virtual method #8 @ RVA 0x00103f70 (2 instructions, 1 basic blocks)
    pub fn method_08(&mut self) {
        // Writes to field at offset +0x0020
    }

    /// Decompiled virtual method #9 @ RVA 0x0057b440 (98 instructions, 8 basic blocks)
    pub fn method_09(&mut self) {
        // Writes to field at offset +0x0008
    }

    /// Decompiled virtual method #10 @ RVA 0x0025db00 (5 instructions, 1 basic blocks)
    pub fn method_10(&mut self) {
    }

    /// Decompiled virtual method #11 @ RVA 0x00103f80 (3 instructions, 1 basic blocks)
    pub fn method_11(&mut self) {
        // Writes to field at offset +0x0024
    }

}

impl CcClass for ccBikePartsParamDispWnd {
    fn class_name(&self) -> &'static str { Self::CLASS_NAME }
    fn vtable_rva(&self) -> u32 { Self::VTABLE_RVA }
    fn virtual_method_count(&self) -> usize { Self::METHOD_COUNT }
    fn base_classes(&self) -> &'static [&'static str] { Self::BASE_CLASSES }
}

/// Decompiled CC2 class: `ccBikeShopMenu` (VTable RVA: 0x00894ca0, 37 virtual methods, 18 recovered fields)
/// Inherits: TalkingBaseMenuMain, ccGameMenuMain, ccGameMenuObj
#[derive(Debug, Clone, Default)]
pub struct ccBikeShopMenu {
    pub vtable_rva: u32,
    pub field_0x0000: u16,
    pub field_0x0008: u32,
    pub field_0x000c: u32,
    pub field_0x0010: u64,
    pub field_0x0018: u64,
    pub field_0x0020: u64,
    pub field_0x0028: u8,
    pub field_0x0030: u64,
    pub field_0x0034: f32,
    pub field_0x0038: u64,
    pub field_0x0040: u8,
    pub field_0x0048: u32,
    pub field_0x004c: u32,
    pub field_0x0050: u64,
    pub field_0x009d: u8,
    pub field_0x00a8: u64,
    pub field_0x00b8: u64,
    pub field_0x0588: u64,
}

impl ccBikeShopMenu {
    pub const CLASS_NAME: &'static str = "ccBikeShopMenu";
    pub const VTABLE_RVA: u32 = 0x00894ca0;
    pub const METHOD_COUNT: usize = 37;
    pub const BASE_CLASSES: &'static [&'static str] = &["TalkingBaseMenuMain", "ccGameMenuMain", "ccGameMenuObj"];
    pub const VIRTUAL_METHODS: &'static [u64] = &[0x0000000180266f20, 0x0000000180649280, 0x0000000180649590, 0x00000001804525e0, 0x0000000180452730, 0x00000001804527e0, 0x0000000180452890, 0x00000001801b27f0, 0x00000001801035b0, 0x0000000180452bf0, 0x000000018045ba00, 0x00000001800d3090, 0x0000000180452300, 0x0000000180452380, 0x0000000180266ff0, 0x000000018025e590, 0x0000000180647d90, 0x0000000180648320, 0x0000000180648460, 0x00000001802672b0, 0x000000018025e7a0, 0x00000001802673d0, 0x0000000180649600, 0x0000000180103e60, 0x0000000180103e60, 0x0000000180103e60, 0x0000000180103e60, 0x0000000180103e70, 0x00000001800d3090, 0x0000000180452bd0, 0x00000001801b2800, 0x0000000180649a00, 0x0000000180649bb0, 0x00000001800d3090, 0x00000001800d3090, 0x00000001800d3090, 0x00000001800d3090];

    pub fn new() -> Self {
        Self {
            vtable_rva: Self::VTABLE_RVA,
            field_0x0000: Default::default(),
            field_0x0008: Default::default(),
            field_0x000c: Default::default(),
            field_0x0010: Default::default(),
            field_0x0018: Default::default(),
            field_0x0020: Default::default(),
            field_0x0028: Default::default(),
            field_0x0030: Default::default(),
            field_0x0034: Default::default(),
            field_0x0038: Default::default(),
            field_0x0040: Default::default(),
            field_0x0048: Default::default(),
            field_0x004c: Default::default(),
            field_0x0050: Default::default(),
            field_0x009d: Default::default(),
            field_0x00a8: Default::default(),
            field_0x00b8: Default::default(),
            field_0x0588: Default::default(),
        }
    }

    /// Decompiled virtual method #0 @ RVA 0x00266f20 (16 instructions, 3 basic blocks)
    pub fn method_00(&mut self) {
    }

    /// Decompiled virtual method #1 @ RVA 0x00649280 (10 instructions, 2 basic blocks)
    pub fn method_01(&mut self) {
    }

    /// Decompiled virtual method #2 @ RVA 0x00649590 (23 instructions, 5 basic blocks)
    pub fn method_02(&mut self) {
        // Writes to field at offset +0x009d
    }

    /// Decompiled virtual method #3 @ RVA 0x004525e0 (9 instructions, 2 basic blocks)
    pub fn method_03(&mut self) {
        // Writes to field at offset +0x0040
    }

    /// Decompiled virtual method #4 @ RVA 0x00452730 (9 instructions, 2 basic blocks)
    pub fn method_04(&mut self) {
        // Writes to field at offset +0x0040
    }

    /// Decompiled virtual method #5 @ RVA 0x004527e0 (9 instructions, 2 basic blocks)
    pub fn method_05(&mut self) {
        // Writes to field at offset +0x0040
    }

    /// Decompiled virtual method #6 @ RVA 0x00452890 (9 instructions, 2 basic blocks)
    pub fn method_06(&mut self) {
        // Writes to field at offset +0x0040
    }

    /// Decompiled virtual method #7 @ RVA 0x001b27f0 (2 instructions, 1 basic blocks)
    pub fn method_07(&mut self) {
    }

    /// Decompiled virtual method #8 @ RVA 0x001035b0 (5 instructions, 1 basic blocks)
    pub fn method_08(&mut self) {
    }

    /// Decompiled virtual method #9 @ RVA 0x00452bf0 (12 instructions, 2 basic blocks)
    pub fn method_09(&mut self) {
        // Writes to field at offset +0x0038
    }

    /// Decompiled virtual method #10 @ RVA 0x0045ba00 (17 instructions, 2 basic blocks)
    pub fn method_10(&mut self) {
    }

    /// Decompiled virtual method #11 @ RVA 0x000d3090 (1 instructions, 1 basic blocks)
    pub fn method_11(&mut self) {
    }

    /// Decompiled virtual method #12 @ RVA 0x00452300 (9 instructions, 2 basic blocks)
    pub fn method_12(&mut self) {
        // Writes to field at offset +0x0028
    }

    /// Decompiled virtual method #13 @ RVA 0x00452380 (9 instructions, 2 basic blocks)
    pub fn method_13(&mut self) {
        // Writes to field at offset +0x0028
    }

    /// Decompiled virtual method #14 @ RVA 0x00266ff0 (117 instructions, 11 basic blocks)
    pub fn method_14(&mut self) {
        // Writes to field at offset +0x000c
    }

    /// Decompiled virtual method #15 @ RVA 0x0025e590 (45 instructions, 6 basic blocks)
    pub fn method_15(&mut self) {
    }

    /// Decompiled virtual method #16 @ RVA 0x00647d90 (10 instructions, 2 basic blocks)
    pub fn method_16(&mut self) {
    }

    /// Decompiled virtual method #17 @ RVA 0x00648320 (12 instructions, 2 basic blocks)
    pub fn method_17(&mut self) {
    }

    /// Decompiled virtual method #18 @ RVA 0x00648460 (12 instructions, 2 basic blocks)
    pub fn method_18(&mut self) {
    }

    /// Decompiled virtual method #19 @ RVA 0x002672b0 (71 instructions, 14 basic blocks)
    pub fn method_19(&mut self) {
    }

    /// Decompiled virtual method #20 @ RVA 0x0025e7a0 (40 instructions, 5 basic blocks)
    pub fn method_20(&mut self) {
        // Writes to field at offset +0x0000
        // Writes to field at offset +0x0008
        // Writes to field at offset +0x0010
        // Writes to field at offset +0x0018
        // Writes to field at offset +0x0020
        // Writes to field at offset +0x0028
        // Writes to field at offset +0x0030
        // Writes to field at offset +0x0038
        // Writes to field at offset +0x0038
        // Writes to field at offset +0x0048
    }

    /// Decompiled virtual method #21 @ RVA 0x002673d0 (21 instructions, 1 basic blocks)
    pub fn method_21(&mut self) {
    }

    /// Decompiled virtual method #22 @ RVA 0x00649600 (14 instructions, 2 basic blocks)
    pub fn method_22(&mut self) {
    }

    /// Decompiled virtual method #23 @ RVA 0x00103e60 (2 instructions, 1 basic blocks)
    pub fn method_23(&mut self) {
    }

    /// Decompiled virtual method #24 @ RVA 0x00103e60 (2 instructions, 1 basic blocks)
    pub fn method_24(&mut self) {
    }

    /// Decompiled virtual method #25 @ RVA 0x00103e60 (2 instructions, 1 basic blocks)
    pub fn method_25(&mut self) {
    }

    /// Decompiled virtual method #26 @ RVA 0x00103e60 (2 instructions, 1 basic blocks)
    pub fn method_26(&mut self) {
    }

    /// Decompiled virtual method #27 @ RVA 0x00103e70 (2 instructions, 1 basic blocks)
    pub fn method_27(&mut self) {
    }

    /// Decompiled virtual method #28 @ RVA 0x000d3090 (1 instructions, 1 basic blocks)
    pub fn method_28(&mut self) {
    }

    /// Decompiled virtual method #29 @ RVA 0x00452bd0 (5 instructions, 2 basic blocks)
    pub fn method_29(&mut self) {
    }

    /// Decompiled virtual method #30 @ RVA 0x001b2800 (13 instructions, 4 basic blocks)
    pub fn method_30(&mut self) {
        // Writes to field at offset +0x0018
        // Writes to field at offset +0x0028
    }

    /// Decompiled virtual method #31 @ RVA 0x00649a00 (98 instructions, 10 basic blocks)
    pub fn method_31(&mut self) {
        // Writes to field at offset +0x0048
        // Writes to field at offset +0x0050
    }

    /// Decompiled virtual method #32 @ RVA 0x00649bb0 (103 instructions, 10 basic blocks)
    pub fn method_32(&mut self) {
        // Writes to field at offset +0x004c
    }

    /// Decompiled virtual method #33 @ RVA 0x000d3090 (1 instructions, 1 basic blocks)
    pub fn method_33(&mut self) {
    }

    /// Decompiled virtual method #34 @ RVA 0x000d3090 (1 instructions, 1 basic blocks)
    pub fn method_34(&mut self) {
    }

    /// Decompiled virtual method #35 @ RVA 0x000d3090 (1 instructions, 1 basic blocks)
    pub fn method_35(&mut self) {
    }

    /// Decompiled virtual method #36 @ RVA 0x000d3090 (1 instructions, 1 basic blocks)
    pub fn method_36(&mut self) {
    }

}

impl CcClass for ccBikeShopMenu {
    fn class_name(&self) -> &'static str { Self::CLASS_NAME }
    fn vtable_rva(&self) -> u32 { Self::VTABLE_RVA }
    fn virtual_method_count(&self) -> usize { Self::METHOD_COUNT }
    fn base_classes(&self) -> &'static [&'static str] { Self::BASE_CLASSES }
}

/// Decompiled CC2 class: `ccBikeStudioMenu` (VTable RVA: 0x008957c0, 37 virtual methods, 17 recovered fields)
/// Inherits: TalkingBaseMenuMain, ccGameMenuMain, ccGameMenuObj
#[derive(Debug, Clone, Default)]
pub struct ccBikeStudioMenu {
    pub vtable_rva: u32,
    pub field_0x0000: u16,
    pub field_0x0008: u32,
    pub field_0x0010: u64,
    pub field_0x0018: u64,
    pub field_0x0020: u64,
    pub field_0x0028: u8,
    pub field_0x0030: u64,
    pub field_0x0034: f32,
    pub field_0x0038: u64,
    pub field_0x0040: u8,
    pub field_0x0048: u32,
    pub field_0x004c: u32,
    pub field_0x0050: u64,
    pub field_0x009d: u8,
    pub field_0x00a8: u64,
    pub field_0x00b8: u64,
    pub field_0x0588: u64,
}

impl ccBikeStudioMenu {
    pub const CLASS_NAME: &'static str = "ccBikeStudioMenu";
    pub const VTABLE_RVA: u32 = 0x008957c0;
    pub const METHOD_COUNT: usize = 37;
    pub const BASE_CLASSES: &'static [&'static str] = &["TalkingBaseMenuMain", "ccGameMenuMain", "ccGameMenuObj"];
    pub const VIRTUAL_METHODS: &'static [u64] = &[0x000000018025e000, 0x0000000180649280, 0x0000000180649590, 0x00000001804525e0, 0x0000000180452730, 0x00000001804527e0, 0x0000000180452890, 0x00000001801b27f0, 0x00000001801035b0, 0x0000000180452bf0, 0x000000018045ba00, 0x00000001800d3090, 0x0000000180452300, 0x0000000180452380, 0x000000018025e120, 0x000000018025e590, 0x0000000180647d90, 0x0000000180648320, 0x0000000180648460, 0x000000018025e650, 0x000000018025e7a0, 0x000000018025e850, 0x0000000180649600, 0x0000000180103e60, 0x0000000180103e60, 0x0000000180103e60, 0x0000000180103e60, 0x0000000180103e70, 0x00000001800d3090, 0x0000000180452bd0, 0x00000001801b2800, 0x0000000180649a00, 0x0000000180649bb0, 0x00000001800d3090, 0x00000001800d3090, 0x00000001800d3090, 0x00000001800d3090];

    pub fn new() -> Self {
        Self {
            vtable_rva: Self::VTABLE_RVA,
            field_0x0000: Default::default(),
            field_0x0008: Default::default(),
            field_0x0010: Default::default(),
            field_0x0018: Default::default(),
            field_0x0020: Default::default(),
            field_0x0028: Default::default(),
            field_0x0030: Default::default(),
            field_0x0034: Default::default(),
            field_0x0038: Default::default(),
            field_0x0040: Default::default(),
            field_0x0048: Default::default(),
            field_0x004c: Default::default(),
            field_0x0050: Default::default(),
            field_0x009d: Default::default(),
            field_0x00a8: Default::default(),
            field_0x00b8: Default::default(),
            field_0x0588: Default::default(),
        }
    }

    /// Decompiled virtual method #0 @ RVA 0x0025e000 (16 instructions, 3 basic blocks)
    pub fn method_00(&mut self) {
    }

    /// Decompiled virtual method #1 @ RVA 0x00649280 (10 instructions, 2 basic blocks)
    pub fn method_01(&mut self) {
    }

    /// Decompiled virtual method #2 @ RVA 0x00649590 (23 instructions, 5 basic blocks)
    pub fn method_02(&mut self) {
        // Writes to field at offset +0x009d
    }

    /// Decompiled virtual method #3 @ RVA 0x004525e0 (9 instructions, 2 basic blocks)
    pub fn method_03(&mut self) {
        // Writes to field at offset +0x0040
    }

    /// Decompiled virtual method #4 @ RVA 0x00452730 (9 instructions, 2 basic blocks)
    pub fn method_04(&mut self) {
        // Writes to field at offset +0x0040
    }

    /// Decompiled virtual method #5 @ RVA 0x004527e0 (9 instructions, 2 basic blocks)
    pub fn method_05(&mut self) {
        // Writes to field at offset +0x0040
    }

    /// Decompiled virtual method #6 @ RVA 0x00452890 (9 instructions, 2 basic blocks)
    pub fn method_06(&mut self) {
        // Writes to field at offset +0x0040
    }

    /// Decompiled virtual method #7 @ RVA 0x001b27f0 (2 instructions, 1 basic blocks)
    pub fn method_07(&mut self) {
    }

    /// Decompiled virtual method #8 @ RVA 0x001035b0 (5 instructions, 1 basic blocks)
    pub fn method_08(&mut self) {
    }

    /// Decompiled virtual method #9 @ RVA 0x00452bf0 (12 instructions, 2 basic blocks)
    pub fn method_09(&mut self) {
        // Writes to field at offset +0x0038
    }

    /// Decompiled virtual method #10 @ RVA 0x0045ba00 (17 instructions, 2 basic blocks)
    pub fn method_10(&mut self) {
    }

    /// Decompiled virtual method #11 @ RVA 0x000d3090 (1 instructions, 1 basic blocks)
    pub fn method_11(&mut self) {
    }

    /// Decompiled virtual method #12 @ RVA 0x00452300 (9 instructions, 2 basic blocks)
    pub fn method_12(&mut self) {
        // Writes to field at offset +0x0028
    }

    /// Decompiled virtual method #13 @ RVA 0x00452380 (9 instructions, 2 basic blocks)
    pub fn method_13(&mut self) {
        // Writes to field at offset +0x0028
    }

    /// Decompiled virtual method #14 @ RVA 0x0025e120 (118 instructions, 18 basic blocks)
    pub fn method_14(&mut self) {
    }

    /// Decompiled virtual method #15 @ RVA 0x0025e590 (45 instructions, 6 basic blocks)
    pub fn method_15(&mut self) {
    }

    /// Decompiled virtual method #16 @ RVA 0x00647d90 (10 instructions, 2 basic blocks)
    pub fn method_16(&mut self) {
    }

    /// Decompiled virtual method #17 @ RVA 0x00648320 (12 instructions, 2 basic blocks)
    pub fn method_17(&mut self) {
    }

    /// Decompiled virtual method #18 @ RVA 0x00648460 (12 instructions, 2 basic blocks)
    pub fn method_18(&mut self) {
    }

    /// Decompiled virtual method #19 @ RVA 0x0025e650 (84 instructions, 16 basic blocks)
    pub fn method_19(&mut self) {
    }

    /// Decompiled virtual method #20 @ RVA 0x0025e7a0 (40 instructions, 5 basic blocks)
    pub fn method_20(&mut self) {
        // Writes to field at offset +0x0000
        // Writes to field at offset +0x0008
        // Writes to field at offset +0x0010
        // Writes to field at offset +0x0018
        // Writes to field at offset +0x0020
        // Writes to field at offset +0x0028
        // Writes to field at offset +0x0030
        // Writes to field at offset +0x0038
        // Writes to field at offset +0x0038
        // Writes to field at offset +0x0048
    }

    /// Decompiled virtual method #21 @ RVA 0x0025e850 (25 instructions, 1 basic blocks)
    pub fn method_21(&mut self) {
    }

    /// Decompiled virtual method #22 @ RVA 0x00649600 (14 instructions, 2 basic blocks)
    pub fn method_22(&mut self) {
    }

    /// Decompiled virtual method #23 @ RVA 0x00103e60 (2 instructions, 1 basic blocks)
    pub fn method_23(&mut self) {
    }

    /// Decompiled virtual method #24 @ RVA 0x00103e60 (2 instructions, 1 basic blocks)
    pub fn method_24(&mut self) {
    }

    /// Decompiled virtual method #25 @ RVA 0x00103e60 (2 instructions, 1 basic blocks)
    pub fn method_25(&mut self) {
    }

    /// Decompiled virtual method #26 @ RVA 0x00103e60 (2 instructions, 1 basic blocks)
    pub fn method_26(&mut self) {
    }

    /// Decompiled virtual method #27 @ RVA 0x00103e70 (2 instructions, 1 basic blocks)
    pub fn method_27(&mut self) {
    }

    /// Decompiled virtual method #28 @ RVA 0x000d3090 (1 instructions, 1 basic blocks)
    pub fn method_28(&mut self) {
    }

    /// Decompiled virtual method #29 @ RVA 0x00452bd0 (5 instructions, 2 basic blocks)
    pub fn method_29(&mut self) {
    }

    /// Decompiled virtual method #30 @ RVA 0x001b2800 (13 instructions, 4 basic blocks)
    pub fn method_30(&mut self) {
        // Writes to field at offset +0x0018
        // Writes to field at offset +0x0028
    }

    /// Decompiled virtual method #31 @ RVA 0x00649a00 (98 instructions, 10 basic blocks)
    pub fn method_31(&mut self) {
        // Writes to field at offset +0x0048
        // Writes to field at offset +0x0050
    }

    /// Decompiled virtual method #32 @ RVA 0x00649bb0 (103 instructions, 10 basic blocks)
    pub fn method_32(&mut self) {
        // Writes to field at offset +0x004c
    }

    /// Decompiled virtual method #33 @ RVA 0x000d3090 (1 instructions, 1 basic blocks)
    pub fn method_33(&mut self) {
    }

    /// Decompiled virtual method #34 @ RVA 0x000d3090 (1 instructions, 1 basic blocks)
    pub fn method_34(&mut self) {
    }

    /// Decompiled virtual method #35 @ RVA 0x000d3090 (1 instructions, 1 basic blocks)
    pub fn method_35(&mut self) {
    }

    /// Decompiled virtual method #36 @ RVA 0x000d3090 (1 instructions, 1 basic blocks)
    pub fn method_36(&mut self) {
    }

}

impl CcClass for ccBikeStudioMenu {
    fn class_name(&self) -> &'static str { Self::CLASS_NAME }
    fn vtable_rva(&self) -> u32 { Self::VTABLE_RVA }
    fn virtual_method_count(&self) -> usize { Self::METHOD_COUNT }
    fn base_classes(&self) -> &'static [&'static str] { Self::BASE_CLASSES }
}

/// Decompiled CC2 class: `ccBook2DetailWnd` (VTable RVA: 0x008ea760, 64 virtual methods, 40 recovered fields)
/// Inherits: ListWnd, MoveAnmPos
#[derive(Debug, Clone, Default)]
pub struct ccBook2DetailWnd {
    pub vtable_rva: u32,
    pub field_0x0000: u64,
    pub field_0x0003: u8,
    pub field_0x0008: f32,
    pub field_0x000c: f32,
    pub field_0x0018: u64,
    pub field_0x001a: u8,
    pub field_0x0020: u32,
    pub field_0x0024: u8,
    pub field_0x0026: u8,
    pub field_0x0028: u32,
    pub field_0x002c: u32,
    pub field_0x0030: u64,
    pub field_0x0060: u32,
    pub field_0x0064: u32,
    pub field_0x0068: f32,
    pub field_0x0070: u32,
    pub field_0x0074: u32,
    pub field_0x0078: f32,
    pub field_0x007c: f32,
    pub field_0x008c: u32,
    pub field_0x0094: u64,
    pub field_0x0098: u32,
    pub field_0x009c: u64,
    pub field_0x00a4: u32,
    pub field_0x00ac: f32,
    pub field_0x00b0: u32,
    pub field_0x00b4: u8,
    pub field_0x00c0: u32,
    pub field_0x00c8: u64,
    pub field_0x00d0: u64,
    pub field_0x00d8: u64,
    pub field_0x00f0: u64,
    pub field_0x00f8: u32,
    pub field_0x00fc: u32,
    pub field_0x0100: f32,
    pub field_0x010c: u8,
    pub field_0x010d: u8,
    pub field_0x0110: u32,
    pub field_0x0118: u64,
    pub field_0x0180: u64,
}

impl ccBook2DetailWnd {
    pub const CLASS_NAME: &'static str = "ccBook2DetailWnd";
    pub const VTABLE_RVA: u32 = 0x008ea760;
    pub const METHOD_COUNT: usize = 64;
    pub const BASE_CLASSES: &'static [&'static str] = &["ListWnd", "MoveAnmPos"];
    pub const VIRTUAL_METHODS: &'static [u64] = &[0x0000000180109550, 0x0000000180109c20, 0x0000000180606430, 0x0000000180606440, 0x000000018057b520, 0x000000018057b450, 0x000000018010c440, 0x000000018057b5a0, 0x0000000180103f70, 0x000000018057b440, 0x000000018010cf30, 0x0000000180103f80, 0x0000000180109520, 0x00000001800d3090, 0x000000018010a4e0, 0x000000018010a100, 0x00000001800d3090, 0x00000001800d3090, 0x00000001800d3090, 0x000000018010a520, 0x0000000180606af0, 0x00000001800fe200, 0x000000018010a590, 0x000000018010a670, 0x000000018010a680, 0x000000018010a8d0, 0x000000018010a910, 0x000000018010a690, 0x000000018010cfd0, 0x00000001800d3090, 0x00000001800d3090, 0x00000001800d3090, 0x00000001800d3090, 0x00000001800d3090, 0x00000001800fe200, 0x00000001800fe200, 0x00000001800fe200, 0x00000001800fe200, 0x00000001800fe200, 0x00000001800fe200, 0x00000001800fe200, 0x00000001800fe200, 0x00000001800fe200, 0x00000001800d3090, 0x0000000180606330, 0x00000001806063f0, 0x0000000180109d40, 0x0000000180109e70, 0x0000000180109fa0, 0x000000018010a020, 0x000000018010a090, 0x000000018010a460, 0x000000018010b660, 0x000000018010bb20, 0x000000018010bf60, 0x000000018010c090, 0x000000018010c230, 0x000000018010c230, 0x000000018010c340, 0x000000018010c2d0, 0x0000000180606090, 0x000000018010b5b0, 0x000000018010ce40, 0x000000018010c510];

    pub fn new() -> Self {
        Self {
            vtable_rva: Self::VTABLE_RVA,
            field_0x0000: Default::default(),
            field_0x0003: Default::default(),
            field_0x0008: Default::default(),
            field_0x000c: Default::default(),
            field_0x0018: Default::default(),
            field_0x001a: Default::default(),
            field_0x0020: Default::default(),
            field_0x0024: Default::default(),
            field_0x0026: Default::default(),
            field_0x0028: Default::default(),
            field_0x002c: Default::default(),
            field_0x0030: Default::default(),
            field_0x0060: Default::default(),
            field_0x0064: Default::default(),
            field_0x0068: Default::default(),
            field_0x0070: Default::default(),
            field_0x0074: Default::default(),
            field_0x0078: Default::default(),
            field_0x007c: Default::default(),
            field_0x008c: Default::default(),
            field_0x0094: Default::default(),
            field_0x0098: Default::default(),
            field_0x009c: Default::default(),
            field_0x00a4: Default::default(),
            field_0x00ac: Default::default(),
            field_0x00b0: Default::default(),
            field_0x00b4: Default::default(),
            field_0x00c0: Default::default(),
            field_0x00c8: Default::default(),
            field_0x00d0: Default::default(),
            field_0x00d8: Default::default(),
            field_0x00f0: Default::default(),
            field_0x00f8: Default::default(),
            field_0x00fc: Default::default(),
            field_0x0100: Default::default(),
            field_0x010c: Default::default(),
            field_0x010d: Default::default(),
            field_0x0110: Default::default(),
            field_0x0118: Default::default(),
            field_0x0180: Default::default(),
        }
    }

    /// Decompiled virtual method #0 @ RVA 0x00109550 (46 instructions, 1 basic blocks)
    pub fn method_00(&mut self) {
    }

    /// Decompiled virtual method #1 @ RVA 0x00109c20 (130 instructions, 13 basic blocks)
    pub fn method_01(&mut self) {
        // Writes to field at offset +0x0118
    }

    /// Decompiled virtual method #2 @ RVA 0x00606430 (111 instructions, 6 basic blocks)
    pub fn method_02(&mut self) {
        // Writes to field at offset +0x0000
        // Writes to field at offset +0x0003
        // Writes to field at offset +0x00f8
    }

    /// Decompiled virtual method #3 @ RVA 0x00606440 (104 instructions, 6 basic blocks)
    pub fn method_03(&mut self) {
        // Writes to field at offset +0x0000
        // Writes to field at offset +0x0003
        // Writes to field at offset +0x00f8
    }

    /// Decompiled virtual method #4 @ RVA 0x0057b520 (30 instructions, 4 basic blocks)
    pub fn method_04(&mut self) {
    }

    /// Decompiled virtual method #5 @ RVA 0x0057b450 (90 instructions, 7 basic blocks)
    pub fn method_05(&mut self) {
    }

    /// Decompiled virtual method #6 @ RVA 0x0010c440 (24 instructions, 3 basic blocks)
    pub fn method_06(&mut self) {
    }

    /// Decompiled virtual method #7 @ RVA 0x0057b5a0 (7 instructions, 1 basic blocks)
    pub fn method_07(&mut self) {
        // Writes to field at offset +0x000c
        // Writes to field at offset +0x0026
    }

    /// Decompiled virtual method #8 @ RVA 0x00103f70 (2 instructions, 1 basic blocks)
    pub fn method_08(&mut self) {
        // Writes to field at offset +0x0020
    }

    /// Decompiled virtual method #9 @ RVA 0x0057b440 (98 instructions, 8 basic blocks)
    pub fn method_09(&mut self) {
        // Writes to field at offset +0x0008
    }

    /// Decompiled virtual method #10 @ RVA 0x0010cf30 (8 instructions, 3 basic blocks)
    pub fn method_10(&mut self) {
    }

    /// Decompiled virtual method #11 @ RVA 0x00103f80 (3 instructions, 1 basic blocks)
    pub fn method_11(&mut self) {
        // Writes to field at offset +0x0024
    }

    /// Decompiled virtual method #12 @ RVA 0x00109520 (7 instructions, 1 basic blocks)
    pub fn method_12(&mut self) {
        // Writes to field at offset +0x0094
        // Writes to field at offset +0x009c
        // Writes to field at offset +0x00a4
        // Writes to field at offset +0x00b4
        // Writes to field at offset +0x00c0
    }

    /// Decompiled virtual method #13 @ RVA 0x000d3090 (1 instructions, 1 basic blocks)
    pub fn method_13(&mut self) {
    }

    /// Decompiled virtual method #14 @ RVA 0x0010a4e0 (13 instructions, 3 basic blocks)
    pub fn method_14(&mut self) {
        // Writes to field at offset +0x010c
    }

    /// Decompiled virtual method #15 @ RVA 0x0010a100 (47 instructions, 10 basic blocks)
    pub fn method_15(&mut self) {
    }

    /// Decompiled virtual method #16 @ RVA 0x000d3090 (1 instructions, 1 basic blocks)
    pub fn method_16(&mut self) {
    }

    /// Decompiled virtual method #17 @ RVA 0x000d3090 (1 instructions, 1 basic blocks)
    pub fn method_17(&mut self) {
    }

    /// Decompiled virtual method #18 @ RVA 0x000d3090 (1 instructions, 1 basic blocks)
    pub fn method_18(&mut self) {
    }

    /// Decompiled virtual method #19 @ RVA 0x0010a520 (26 instructions, 4 basic blocks)
    pub fn method_19(&mut self) {
        // Writes to field at offset +0x010c
    }

    /// Decompiled virtual method #20 @ RVA 0x00606af0 (117 instructions, 12 basic blocks)
    pub fn method_20(&mut self) {
        // Writes to field at offset +0x0000
        // Writes to field at offset +0x0003
        // Writes to field at offset +0x001a
    }

    /// Decompiled virtual method #21 @ RVA 0x000fe200 (2 instructions, 1 basic blocks)
    pub fn method_21(&mut self) {
    }

    /// Decompiled virtual method #22 @ RVA 0x0010a590 (52 instructions, 9 basic blocks)
    pub fn method_22(&mut self) {
        // Writes to field at offset +0x010c
    }

    /// Decompiled virtual method #23 @ RVA 0x0010a670 (113 instructions, 16 basic blocks)
    pub fn method_23(&mut self) {
        // Writes to field at offset +0x010d
    }

    /// Decompiled virtual method #24 @ RVA 0x0010a680 (110 instructions, 16 basic blocks)
    pub fn method_24(&mut self) {
        // Writes to field at offset +0x010d
    }

    /// Decompiled virtual method #25 @ RVA 0x0010a8d0 (117 instructions, 7 basic blocks)
    pub fn method_25(&mut self) {
        // Writes to field at offset +0x0110
    }

    /// Decompiled virtual method #26 @ RVA 0x0010a910 (112 instructions, 9 basic blocks)
    pub fn method_26(&mut self) {
    }

    /// Decompiled virtual method #27 @ RVA 0x0010a690 (107 instructions, 15 basic blocks)
    pub fn method_27(&mut self) {
        // Writes to field at offset +0x010d
    }

    /// Decompiled virtual method #28 @ RVA 0x0010cfd0 (25 instructions, 3 basic blocks)
    pub fn method_28(&mut self) {
        // Writes to field at offset +0x010c
    }

    /// Decompiled virtual method #29 @ RVA 0x000d3090 (1 instructions, 1 basic blocks)
    pub fn method_29(&mut self) {
    }

    /// Decompiled virtual method #30 @ RVA 0x000d3090 (1 instructions, 1 basic blocks)
    pub fn method_30(&mut self) {
    }

    /// Decompiled virtual method #31 @ RVA 0x000d3090 (1 instructions, 1 basic blocks)
    pub fn method_31(&mut self) {
    }

    /// Decompiled virtual method #32 @ RVA 0x000d3090 (1 instructions, 1 basic blocks)
    pub fn method_32(&mut self) {
    }

    /// Decompiled virtual method #33 @ RVA 0x000d3090 (1 instructions, 1 basic blocks)
    pub fn method_33(&mut self) {
    }

    /// Decompiled virtual method #34 @ RVA 0x000fe200 (2 instructions, 1 basic blocks)
    pub fn method_34(&mut self) {
    }

    /// Decompiled virtual method #35 @ RVA 0x000fe200 (2 instructions, 1 basic blocks)
    pub fn method_35(&mut self) {
    }

    /// Decompiled virtual method #36 @ RVA 0x000fe200 (2 instructions, 1 basic blocks)
    pub fn method_36(&mut self) {
    }

    /// Decompiled virtual method #37 @ RVA 0x000fe200 (2 instructions, 1 basic blocks)
    pub fn method_37(&mut self) {
    }

    /// Decompiled virtual method #38 @ RVA 0x000fe200 (2 instructions, 1 basic blocks)
    pub fn method_38(&mut self) {
    }

    /// Decompiled virtual method #39 @ RVA 0x000fe200 (2 instructions, 1 basic blocks)
    pub fn method_39(&mut self) {
    }

    /// Decompiled virtual method #40 @ RVA 0x000fe200 (2 instructions, 1 basic blocks)
    pub fn method_40(&mut self) {
    }

    /// Decompiled virtual method #41 @ RVA 0x000fe200 (2 instructions, 1 basic blocks)
    pub fn method_41(&mut self) {
    }

    /// Decompiled virtual method #42 @ RVA 0x000fe200 (2 instructions, 1 basic blocks)
    pub fn method_42(&mut self) {
    }

    /// Decompiled virtual method #43 @ RVA 0x000d3090 (1 instructions, 1 basic blocks)
    pub fn method_43(&mut self) {
    }

    /// Decompiled virtual method #44 @ RVA 0x00606330 (16 instructions, 3 basic blocks)
    pub fn method_44(&mut self) {
    }

    /// Decompiled virtual method #45 @ RVA 0x006063f0 (12 instructions, 1 basic blocks)
    pub fn method_45(&mut self) {
    }

    /// Decompiled virtual method #46 @ RVA 0x00109d40 (67 instructions, 5 basic blocks)
    pub fn method_46(&mut self) {
        // Writes to field at offset +0x0118
    }

    /// Decompiled virtual method #47 @ RVA 0x00109e70 (64 instructions, 5 basic blocks)
    pub fn method_47(&mut self) {
        // Writes to field at offset +0x00f8
        // Writes to field at offset +0x00fc
        // Writes to field at offset +0x0100
        // Writes to field at offset +0x0110
        // Writes to field at offset +0x0118
    }

    /// Decompiled virtual method #48 @ RVA 0x00109fa0 (101 instructions, 7 basic blocks)
    pub fn method_48(&mut self) {
    }

    /// Decompiled virtual method #49 @ RVA 0x0010a020 (66 instructions, 5 basic blocks)
    pub fn method_49(&mut self) {
    }

    /// Decompiled virtual method #50 @ RVA 0x0010a090 (26 instructions, 3 basic blocks)
    pub fn method_50(&mut self) {
    }

    /// Decompiled virtual method #51 @ RVA 0x0010a460 (30 instructions, 3 basic blocks)
    pub fn method_51(&mut self) {
        // Writes to field at offset +0x0118
    }

    /// Decompiled virtual method #52 @ RVA 0x0010b660 (126 instructions, 20 basic blocks)
    pub fn method_52(&mut self) {
        // Writes to field at offset +0x009c
    }

    /// Decompiled virtual method #53 @ RVA 0x0010bb20 (131 instructions, 17 basic blocks)
    pub fn method_53(&mut self) {
    }

    /// Decompiled virtual method #54 @ RVA 0x0010bf60 (69 instructions, 11 basic blocks)
    pub fn method_54(&mut self) {
    }

    /// Decompiled virtual method #55 @ RVA 0x0010c090 (98 instructions, 13 basic blocks)
    pub fn method_55(&mut self) {
    }

    /// Decompiled virtual method #56 @ RVA 0x0010c230 (26 instructions, 2 basic blocks)
    pub fn method_56(&mut self) {
    }

    /// Decompiled virtual method #57 @ RVA 0x0010c230 (26 instructions, 2 basic blocks)
    pub fn method_57(&mut self) {
    }

    /// Decompiled virtual method #58 @ RVA 0x0010c340 (29 instructions, 3 basic blocks)
    pub fn method_58(&mut self) {
    }

    /// Decompiled virtual method #59 @ RVA 0x0010c2d0 (29 instructions, 3 basic blocks)
    pub fn method_59(&mut self) {
    }

    /// Decompiled virtual method #60 @ RVA 0x00606090 (13 instructions, 1 basic blocks)
    pub fn method_60(&mut self) {
    }

    /// Decompiled virtual method #61 @ RVA 0x0010b5b0 (23 instructions, 2 basic blocks)
    pub fn method_61(&mut self) {
    }

    /// Decompiled virtual method #62 @ RVA 0x0010ce40 (59 instructions, 6 basic blocks)
    pub fn method_62(&mut self) {
    }

    /// Decompiled virtual method #63 @ RVA 0x0010c510 (52 instructions, 6 basic blocks)
    pub fn method_63(&mut self) {
        // Writes to field at offset +0x0030
        // Writes to field at offset +0x00d0
        // Writes to field at offset +0x00d8
        // Writes to field at offset +0x00f0
    }

}

impl CcClass for ccBook2DetailWnd {
    fn class_name(&self) -> &'static str { Self::CLASS_NAME }
    fn vtable_rva(&self) -> u32 { Self::VTABLE_RVA }
    fn virtual_method_count(&self) -> usize { Self::METHOD_COUNT }
    fn base_classes(&self) -> &'static [&'static str] { Self::BASE_CLASSES }
}

/// Decompiled CC2 class: `ccBookDetailWnd` (VTable RVA: 0x008ea300, 9 virtual methods, 17 recovered fields)
/// Inherits: ScrollMenuList
#[derive(Debug, Clone, Default)]
pub struct ccBookDetailWnd {
    pub vtable_rva: u32,
    pub field_0x0000: u64,
    pub field_0x0008: u64,
    pub field_0x0010: u64,
    pub field_0x0018: u64,
    pub field_0x001a: u8,
    pub field_0x001c: f32,
    pub field_0x0020: u32,
    pub field_0x0024: u32,
    pub field_0x0028: f32,
    pub field_0x002c: u32,
    pub field_0x0030: u32,
    pub field_0x0038: u32,
    pub field_0x003c: u32,
    pub field_0x0044: u32,
    pub field_0x0048: u32,
    pub field_0x0070: u8,
    pub field_0x0078: u64,
}

impl ccBookDetailWnd {
    pub const CLASS_NAME: &'static str = "ccBookDetailWnd";
    pub const VTABLE_RVA: u32 = 0x008ea300;
    pub const METHOD_COUNT: usize = 9;
    pub const BASE_CLASSES: &'static [&'static str] = &["ScrollMenuList"];
    pub const VIRTUAL_METHODS: &'static [u64] = &[0x0000000180113630, 0x0000000180113640, 0x0000000180113680, 0x0000000180113a20, 0x000000018073e2c0, 0x0000000180608740, 0x0000000180608b80, 0x0000000180608be0, 0x000000018073e2c0];

    pub fn new() -> Self {
        Self {
            vtable_rva: Self::VTABLE_RVA,
            field_0x0000: Default::default(),
            field_0x0008: Default::default(),
            field_0x0010: Default::default(),
            field_0x0018: Default::default(),
            field_0x001a: Default::default(),
            field_0x001c: Default::default(),
            field_0x0020: Default::default(),
            field_0x0024: Default::default(),
            field_0x0028: Default::default(),
            field_0x002c: Default::default(),
            field_0x0030: Default::default(),
            field_0x0038: Default::default(),
            field_0x003c: Default::default(),
            field_0x0044: Default::default(),
            field_0x0048: Default::default(),
            field_0x0070: Default::default(),
            field_0x0078: Default::default(),
        }
    }

    /// Decompiled virtual method #0 @ RVA 0x00113630 (137 instructions, 17 basic blocks)
    pub fn method_00(&mut self) {
        // Writes to field at offset +0x001a
        // Writes to field at offset +0x0044
        // Writes to field at offset +0x0048
    }

    /// Decompiled virtual method #1 @ RVA 0x00113640 (133 instructions, 18 basic blocks)
    pub fn method_01(&mut self) {
        // Writes to field at offset +0x001a
        // Writes to field at offset +0x0044
        // Writes to field at offset +0x0048
    }

    /// Decompiled virtual method #2 @ RVA 0x00113680 (121 instructions, 20 basic blocks)
    pub fn method_02(&mut self) {
        // Writes to field at offset +0x001a
        // Writes to field at offset +0x001c
        // Writes to field at offset +0x0044
        // Writes to field at offset +0x0048
        // Writes to field at offset +0x0070
    }

    /// Decompiled virtual method #3 @ RVA 0x00113a20 (14 instructions, 3 basic blocks)
    pub fn method_03(&mut self) {
    }

    /// Decompiled virtual method #4 @ RVA 0x0073e2c0 (337 instructions, 1 basic blocks)
    pub fn method_04(&mut self) {
    }

    /// Decompiled virtual method #5 @ RVA 0x00608740 (16 instructions, 3 basic blocks)
    pub fn method_05(&mut self) {
    }

    /// Decompiled virtual method #6 @ RVA 0x00608b80 (23 instructions, 4 basic blocks)
    pub fn method_06(&mut self) {
        // Writes to field at offset +0x0020
        // Writes to field at offset +0x0024
        // Writes to field at offset +0x0024
    }

    /// Decompiled virtual method #7 @ RVA 0x00608be0 (28 instructions, 4 basic blocks)
    pub fn method_07(&mut self) {
        // Writes to field at offset +0x0020
        // Writes to field at offset +0x0024
    }

    /// Decompiled virtual method #8 @ RVA 0x0073e2c0 (337 instructions, 1 basic blocks)
    pub fn method_08(&mut self) {
    }

}

impl CcClass for ccBookDetailWnd {
    fn class_name(&self) -> &'static str { Self::CLASS_NAME }
    fn vtable_rva(&self) -> u32 { Self::VTABLE_RVA }
    fn virtual_method_count(&self) -> usize { Self::METHOD_COUNT }
    fn base_classes(&self) -> &'static [&'static str] { Self::BASE_CLASSES }
}

/// Decompiled CC2 class: `ccBookDetailWndBook4` (VTable RVA: 0x008ea278, 9 virtual methods, 18 recovered fields)
/// Inherits: ccBookDetailWnd, ScrollMenuList
#[derive(Debug, Clone, Default)]
pub struct ccBookDetailWndBook4 {
    pub vtable_rva: u32,
    pub field_0x0000: u64,
    pub field_0x0003: u8,
    pub field_0x0008: u64,
    pub field_0x0010: u64,
    pub field_0x0018: u64,
    pub field_0x001a: u8,
    pub field_0x001c: f32,
    pub field_0x0020: u32,
    pub field_0x0024: u32,
    pub field_0x0028: f32,
    pub field_0x002c: u32,
    pub field_0x0030: u32,
    pub field_0x0038: u32,
    pub field_0x003c: u32,
    pub field_0x0044: u32,
    pub field_0x0048: u32,
    pub field_0x0070: u8,
    pub field_0x0078: u64,
}

impl ccBookDetailWndBook4 {
    pub const CLASS_NAME: &'static str = "ccBookDetailWndBook4";
    pub const VTABLE_RVA: u32 = 0x008ea278;
    pub const METHOD_COUNT: usize = 9;
    pub const BASE_CLASSES: &'static [&'static str] = &["ccBookDetailWnd", "ScrollMenuList"];
    pub const VIRTUAL_METHODS: &'static [u64] = &[0x0000000180113630, 0x0000000180113640, 0x0000000180113680, 0x0000000180113a20, 0x0000000180607880, 0x0000000180606f10, 0x0000000180608b80, 0x0000000180608be0, 0x0000000180607440];

    pub fn new() -> Self {
        Self {
            vtable_rva: Self::VTABLE_RVA,
            field_0x0000: Default::default(),
            field_0x0003: Default::default(),
            field_0x0008: Default::default(),
            field_0x0010: Default::default(),
            field_0x0018: Default::default(),
            field_0x001a: Default::default(),
            field_0x001c: Default::default(),
            field_0x0020: Default::default(),
            field_0x0024: Default::default(),
            field_0x0028: Default::default(),
            field_0x002c: Default::default(),
            field_0x0030: Default::default(),
            field_0x0038: Default::default(),
            field_0x003c: Default::default(),
            field_0x0044: Default::default(),
            field_0x0048: Default::default(),
            field_0x0070: Default::default(),
            field_0x0078: Default::default(),
        }
    }

    /// Decompiled virtual method #0 @ RVA 0x00113630 (137 instructions, 17 basic blocks)
    pub fn method_00(&mut self) {
        // Writes to field at offset +0x001a
        // Writes to field at offset +0x0044
        // Writes to field at offset +0x0048
    }

    /// Decompiled virtual method #1 @ RVA 0x00113640 (133 instructions, 18 basic blocks)
    pub fn method_01(&mut self) {
        // Writes to field at offset +0x001a
        // Writes to field at offset +0x0044
        // Writes to field at offset +0x0048
    }

    /// Decompiled virtual method #2 @ RVA 0x00113680 (121 instructions, 20 basic blocks)
    pub fn method_02(&mut self) {
        // Writes to field at offset +0x001a
        // Writes to field at offset +0x001c
        // Writes to field at offset +0x0044
        // Writes to field at offset +0x0048
        // Writes to field at offset +0x0070
    }

    /// Decompiled virtual method #3 @ RVA 0x00113a20 (14 instructions, 3 basic blocks)
    pub fn method_03(&mut self) {
    }

    /// Decompiled virtual method #4 @ RVA 0x00607880 (69 instructions, 1 basic blocks)
    pub fn method_04(&mut self) {
        // Writes to field at offset +0x0000
        // Writes to field at offset +0x0003
    }

    /// Decompiled virtual method #5 @ RVA 0x00606f10 (18 instructions, 3 basic blocks)
    pub fn method_05(&mut self) {
        // Writes to field at offset +0x0000
    }

    /// Decompiled virtual method #6 @ RVA 0x00608b80 (23 instructions, 4 basic blocks)
    pub fn method_06(&mut self) {
        // Writes to field at offset +0x0020
        // Writes to field at offset +0x0024
        // Writes to field at offset +0x0024
    }

    /// Decompiled virtual method #7 @ RVA 0x00608be0 (28 instructions, 4 basic blocks)
    pub fn method_07(&mut self) {
        // Writes to field at offset +0x0020
        // Writes to field at offset +0x0024
    }

    /// Decompiled virtual method #8 @ RVA 0x00607440 (139 instructions, 26 basic blocks)
    pub fn method_08(&mut self) {
        // Writes to field at offset +0x0048
    }

}

impl CcClass for ccBookDetailWndBook4 {
    fn class_name(&self) -> &'static str { Self::CLASS_NAME }
    fn vtable_rva(&self) -> u32 { Self::VTABLE_RVA }
    fn virtual_method_count(&self) -> usize { Self::METHOD_COUNT }
    fn base_classes(&self) -> &'static [&'static str] { Self::BASE_CLASSES }
}

/// Decompiled CC2 class: `ccBookDetailWndBook7` (VTable RVA: 0x008ea228, 9 virtual methods, 17 recovered fields)
/// Inherits: ccBookDetailWnd, ScrollMenuList
#[derive(Debug, Clone, Default)]
pub struct ccBookDetailWndBook7 {
    pub vtable_rva: u32,
    pub field_0x0000: u64,
    pub field_0x0008: u64,
    pub field_0x0010: u64,
    pub field_0x0018: u64,
    pub field_0x001a: u8,
    pub field_0x001c: f32,
    pub field_0x0020: u32,
    pub field_0x0024: u32,
    pub field_0x0028: f32,
    pub field_0x002c: u32,
    pub field_0x0030: u32,
    pub field_0x0038: u32,
    pub field_0x003c: u32,
    pub field_0x0044: u32,
    pub field_0x0048: u32,
    pub field_0x0070: u8,
    pub field_0x0078: u64,
}

impl ccBookDetailWndBook7 {
    pub const CLASS_NAME: &'static str = "ccBookDetailWndBook7";
    pub const VTABLE_RVA: u32 = 0x008ea228;
    pub const METHOD_COUNT: usize = 9;
    pub const BASE_CLASSES: &'static [&'static str] = &["ccBookDetailWnd", "ScrollMenuList"];
    pub const VIRTUAL_METHODS: &'static [u64] = &[0x0000000180113630, 0x0000000180113640, 0x0000000180113680, 0x0000000180113a20, 0x0000000180608350, 0x00000001806079e0, 0x0000000180608b80, 0x0000000180608be0, 0x0000000180607e70];

    pub fn new() -> Self {
        Self {
            vtable_rva: Self::VTABLE_RVA,
            field_0x0000: Default::default(),
            field_0x0008: Default::default(),
            field_0x0010: Default::default(),
            field_0x0018: Default::default(),
            field_0x001a: Default::default(),
            field_0x001c: Default::default(),
            field_0x0020: Default::default(),
            field_0x0024: Default::default(),
            field_0x0028: Default::default(),
            field_0x002c: Default::default(),
            field_0x0030: Default::default(),
            field_0x0038: Default::default(),
            field_0x003c: Default::default(),
            field_0x0044: Default::default(),
            field_0x0048: Default::default(),
            field_0x0070: Default::default(),
            field_0x0078: Default::default(),
        }
    }

    /// Decompiled virtual method #0 @ RVA 0x00113630 (137 instructions, 17 basic blocks)
    pub fn method_00(&mut self) {
        // Writes to field at offset +0x001a
        // Writes to field at offset +0x0044
        // Writes to field at offset +0x0048
    }

    /// Decompiled virtual method #1 @ RVA 0x00113640 (133 instructions, 18 basic blocks)
    pub fn method_01(&mut self) {
        // Writes to field at offset +0x001a
        // Writes to field at offset +0x0044
        // Writes to field at offset +0x0048
    }

    /// Decompiled virtual method #2 @ RVA 0x00113680 (121 instructions, 20 basic blocks)
    pub fn method_02(&mut self) {
        // Writes to field at offset +0x001a
        // Writes to field at offset +0x001c
        // Writes to field at offset +0x0044
        // Writes to field at offset +0x0048
        // Writes to field at offset +0x0070
    }

    /// Decompiled virtual method #3 @ RVA 0x00113a20 (14 instructions, 3 basic blocks)
    pub fn method_03(&mut self) {
    }

    /// Decompiled virtual method #4 @ RVA 0x00608350 (30 instructions, 4 basic blocks)
    pub fn method_04(&mut self) {
        // Writes to field at offset +0x0000
    }

    /// Decompiled virtual method #5 @ RVA 0x006079e0 (18 instructions, 3 basic blocks)
    pub fn method_05(&mut self) {
        // Writes to field at offset +0x0000
    }

    /// Decompiled virtual method #6 @ RVA 0x00608b80 (23 instructions, 4 basic blocks)
    pub fn method_06(&mut self) {
        // Writes to field at offset +0x0020
        // Writes to field at offset +0x0024
        // Writes to field at offset +0x0024
    }

    /// Decompiled virtual method #7 @ RVA 0x00608be0 (28 instructions, 4 basic blocks)
    pub fn method_07(&mut self) {
        // Writes to field at offset +0x0020
        // Writes to field at offset +0x0024
    }

    /// Decompiled virtual method #8 @ RVA 0x00607e70 (134 instructions, 27 basic blocks)
    pub fn method_08(&mut self) {
        // Writes to field at offset +0x0048
    }

}

impl CcClass for ccBookDetailWndBook7 {
    fn class_name(&self) -> &'static str { Self::CLASS_NAME }
    fn vtable_rva(&self) -> u32 { Self::VTABLE_RVA }
    fn virtual_method_count(&self) -> usize { Self::METHOD_COUNT }
    fn base_classes(&self) -> &'static [&'static str] { Self::BASE_CLASSES }
}

/// Decompiled CC2 class: `ccBookDetailWndBook8` (VTable RVA: 0x008ea1d8, 9 virtual methods, 17 recovered fields)
/// Inherits: ccBookDetailWnd, ScrollMenuList
#[derive(Debug, Clone, Default)]
pub struct ccBookDetailWndBook8 {
    pub vtable_rva: u32,
    pub field_0x0000: u64,
    pub field_0x0008: u64,
    pub field_0x0010: u64,
    pub field_0x0018: u64,
    pub field_0x001a: u8,
    pub field_0x001c: f32,
    pub field_0x0020: u32,
    pub field_0x0024: u32,
    pub field_0x0028: f32,
    pub field_0x002c: u32,
    pub field_0x0030: u32,
    pub field_0x0038: u32,
    pub field_0x003c: u32,
    pub field_0x0044: u32,
    pub field_0x0048: u32,
    pub field_0x0070: u8,
    pub field_0x0078: u64,
}

impl ccBookDetailWndBook8 {
    pub const CLASS_NAME: &'static str = "ccBookDetailWndBook8";
    pub const VTABLE_RVA: u32 = 0x008ea1d8;
    pub const METHOD_COUNT: usize = 9;
    pub const BASE_CLASSES: &'static [&'static str] = &["ccBookDetailWnd", "ScrollMenuList"];
    pub const VIRTUAL_METHODS: &'static [u64] = &[0x0000000180113630, 0x0000000180113640, 0x0000000180113680, 0x0000000180113a20, 0x0000000180608350, 0x0000000180608360, 0x0000000180608b80, 0x0000000180608be0, 0x00000001806083a0];

    pub fn new() -> Self {
        Self {
            vtable_rva: Self::VTABLE_RVA,
            field_0x0000: Default::default(),
            field_0x0008: Default::default(),
            field_0x0010: Default::default(),
            field_0x0018: Default::default(),
            field_0x001a: Default::default(),
            field_0x001c: Default::default(),
            field_0x0020: Default::default(),
            field_0x0024: Default::default(),
            field_0x0028: Default::default(),
            field_0x002c: Default::default(),
            field_0x0030: Default::default(),
            field_0x0038: Default::default(),
            field_0x003c: Default::default(),
            field_0x0044: Default::default(),
            field_0x0048: Default::default(),
            field_0x0070: Default::default(),
            field_0x0078: Default::default(),
        }
    }

    /// Decompiled virtual method #0 @ RVA 0x00113630 (137 instructions, 17 basic blocks)
    pub fn method_00(&mut self) {
        // Writes to field at offset +0x001a
        // Writes to field at offset +0x0044
        // Writes to field at offset +0x0048
    }

    /// Decompiled virtual method #1 @ RVA 0x00113640 (133 instructions, 18 basic blocks)
    pub fn method_01(&mut self) {
        // Writes to field at offset +0x001a
        // Writes to field at offset +0x0044
        // Writes to field at offset +0x0048
    }

    /// Decompiled virtual method #2 @ RVA 0x00113680 (121 instructions, 20 basic blocks)
    pub fn method_02(&mut self) {
        // Writes to field at offset +0x001a
        // Writes to field at offset +0x001c
        // Writes to field at offset +0x0044
        // Writes to field at offset +0x0048
        // Writes to field at offset +0x0070
    }

    /// Decompiled virtual method #3 @ RVA 0x00113a20 (14 instructions, 3 basic blocks)
    pub fn method_03(&mut self) {
    }

    /// Decompiled virtual method #4 @ RVA 0x00608350 (30 instructions, 4 basic blocks)
    pub fn method_04(&mut self) {
        // Writes to field at offset +0x0000
    }

    /// Decompiled virtual method #5 @ RVA 0x00608360 (18 instructions, 3 basic blocks)
    pub fn method_05(&mut self) {
        // Writes to field at offset +0x0000
    }

    /// Decompiled virtual method #6 @ RVA 0x00608b80 (23 instructions, 4 basic blocks)
    pub fn method_06(&mut self) {
        // Writes to field at offset +0x0020
        // Writes to field at offset +0x0024
        // Writes to field at offset +0x0024
    }

    /// Decompiled virtual method #7 @ RVA 0x00608be0 (28 instructions, 4 basic blocks)
    pub fn method_07(&mut self) {
        // Writes to field at offset +0x0020
        // Writes to field at offset +0x0024
    }

    /// Decompiled virtual method #8 @ RVA 0x006083a0 (137 instructions, 26 basic blocks)
    pub fn method_08(&mut self) {
        // Writes to field at offset +0x0048
    }

}

impl CcClass for ccBookDetailWndBook8 {
    fn class_name(&self) -> &'static str { Self::CLASS_NAME }
    fn vtable_rva(&self) -> u32 { Self::VTABLE_RVA }
    fn virtual_method_count(&self) -> usize { Self::METHOD_COUNT }
    fn base_classes(&self) -> &'static [&'static str] { Self::BASE_CLASSES }
}

/// Decompiled CC2 class: `ccBookDispWnd` (VTable RVA: 0x008ea350, 64 virtual methods, 40 recovered fields)
/// Inherits: ListWnd, MoveAnmPos
#[derive(Debug, Clone, Default)]
pub struct ccBookDispWnd {
    pub vtable_rva: u32,
    pub field_0x0000: u64,
    pub field_0x0003: u8,
    pub field_0x0008: f32,
    pub field_0x000c: f32,
    pub field_0x0018: u64,
    pub field_0x0020: u32,
    pub field_0x0024: u8,
    pub field_0x0026: u8,
    pub field_0x0028: u32,
    pub field_0x002c: u32,
    pub field_0x0030: u64,
    pub field_0x0060: u8,
    pub field_0x0064: u32,
    pub field_0x0068: f32,
    pub field_0x0070: u32,
    pub field_0x0074: u32,
    pub field_0x0078: f32,
    pub field_0x007c: f32,
    pub field_0x008c: u32,
    pub field_0x0094: u64,
    pub field_0x0098: u32,
    pub field_0x009c: u64,
    pub field_0x00a4: u32,
    pub field_0x00ac: f32,
    pub field_0x00b0: u32,
    pub field_0x00b4: u32,
    pub field_0x00c0: u32,
    pub field_0x00c8: u64,
    pub field_0x00d0: u64,
    pub field_0x00d8: u64,
    pub field_0x00f0: u64,
    pub field_0x00f8: u32,
    pub field_0x00fc: u32,
    pub field_0x0100: f32,
    pub field_0x010c: u8,
    pub field_0x010d: u8,
    pub field_0x0110: u32,
    pub field_0x0118: u64,
    pub field_0x0158: f32,
    pub field_0x0180: u64,
}

impl ccBookDispWnd {
    pub const CLASS_NAME: &'static str = "ccBookDispWnd";
    pub const VTABLE_RVA: u32 = 0x008ea350;
    pub const METHOD_COUNT: usize = 64;
    pub const BASE_CLASSES: &'static [&'static str] = &["ListWnd", "MoveAnmPos"];
    pub const VIRTUAL_METHODS: &'static [u64] = &[0x0000000180109550, 0x0000000180109c20, 0x0000000180604e40, 0x0000000180604f60, 0x000000018057b520, 0x000000018057b450, 0x000000018010c440, 0x000000018057b5a0, 0x0000000180103f70, 0x000000018057b440, 0x000000018010cf30, 0x0000000180103f80, 0x0000000180109520, 0x00000001800d3090, 0x000000018010a4e0, 0x000000018010a100, 0x00000001800d3090, 0x00000001800d3090, 0x00000001800d3090, 0x000000018010a520, 0x00000001806059c0, 0x00000001800fe200, 0x000000018010a590, 0x0000000180605770, 0x0000000180605890, 0x000000018010a8d0, 0x000000018010a910, 0x000000018010a690, 0x000000018010cfd0, 0x00000001800d3090, 0x00000001800d3090, 0x00000001800d3090, 0x00000001800d3090, 0x00000001800d3090, 0x00000001800fe200, 0x00000001800fe200, 0x00000001800fe200, 0x00000001800fe200, 0x00000001800fe200, 0x00000001800fe200, 0x00000001800fe200, 0x00000001800fe200, 0x00000001800fe200, 0x00000001800d3090, 0x0000000180604c10, 0x0000000180604d50, 0x0000000180109d40, 0x0000000180109e70, 0x0000000180109fa0, 0x000000018010a020, 0x000000018010a090, 0x000000018010a460, 0x000000018010b660, 0x000000018010bb20, 0x000000018010bf60, 0x000000018010c090, 0x000000018010c230, 0x000000018010c230, 0x000000018010c340, 0x000000018010c2d0, 0x0000000180606090, 0x000000018010b5b0, 0x000000018010ce40, 0x000000018010c510];

    pub fn new() -> Self {
        Self {
            vtable_rva: Self::VTABLE_RVA,
            field_0x0000: Default::default(),
            field_0x0003: Default::default(),
            field_0x0008: Default::default(),
            field_0x000c: Default::default(),
            field_0x0018: Default::default(),
            field_0x0020: Default::default(),
            field_0x0024: Default::default(),
            field_0x0026: Default::default(),
            field_0x0028: Default::default(),
            field_0x002c: Default::default(),
            field_0x0030: Default::default(),
            field_0x0060: Default::default(),
            field_0x0064: Default::default(),
            field_0x0068: Default::default(),
            field_0x0070: Default::default(),
            field_0x0074: Default::default(),
            field_0x0078: Default::default(),
            field_0x007c: Default::default(),
            field_0x008c: Default::default(),
            field_0x0094: Default::default(),
            field_0x0098: Default::default(),
            field_0x009c: Default::default(),
            field_0x00a4: Default::default(),
            field_0x00ac: Default::default(),
            field_0x00b0: Default::default(),
            field_0x00b4: Default::default(),
            field_0x00c0: Default::default(),
            field_0x00c8: Default::default(),
            field_0x00d0: Default::default(),
            field_0x00d8: Default::default(),
            field_0x00f0: Default::default(),
            field_0x00f8: Default::default(),
            field_0x00fc: Default::default(),
            field_0x0100: Default::default(),
            field_0x010c: Default::default(),
            field_0x010d: Default::default(),
            field_0x0110: Default::default(),
            field_0x0118: Default::default(),
            field_0x0158: Default::default(),
            field_0x0180: Default::default(),
        }
    }

    /// Decompiled virtual method #0 @ RVA 0x00109550 (46 instructions, 1 basic blocks)
    pub fn method_00(&mut self) {
    }

    /// Decompiled virtual method #1 @ RVA 0x00109c20 (130 instructions, 13 basic blocks)
    pub fn method_01(&mut self) {
        // Writes to field at offset +0x0118
    }

    /// Decompiled virtual method #2 @ RVA 0x00604e40 (50 instructions, 9 basic blocks)
    pub fn method_02(&mut self) {
    }

    /// Decompiled virtual method #3 @ RVA 0x00604f60 (101 instructions, 5 basic blocks)
    pub fn method_03(&mut self) {
        // Writes to field at offset +0x00f8
    }

    /// Decompiled virtual method #4 @ RVA 0x0057b520 (30 instructions, 4 basic blocks)
    pub fn method_04(&mut self) {
    }

    /// Decompiled virtual method #5 @ RVA 0x0057b450 (90 instructions, 7 basic blocks)
    pub fn method_05(&mut self) {
    }

    /// Decompiled virtual method #6 @ RVA 0x0010c440 (24 instructions, 3 basic blocks)
    pub fn method_06(&mut self) {
    }

    /// Decompiled virtual method #7 @ RVA 0x0057b5a0 (7 instructions, 1 basic blocks)
    pub fn method_07(&mut self) {
        // Writes to field at offset +0x000c
        // Writes to field at offset +0x0026
    }

    /// Decompiled virtual method #8 @ RVA 0x00103f70 (2 instructions, 1 basic blocks)
    pub fn method_08(&mut self) {
        // Writes to field at offset +0x0020
    }

    /// Decompiled virtual method #9 @ RVA 0x0057b440 (98 instructions, 8 basic blocks)
    pub fn method_09(&mut self) {
        // Writes to field at offset +0x0008
    }

    /// Decompiled virtual method #10 @ RVA 0x0010cf30 (8 instructions, 3 basic blocks)
    pub fn method_10(&mut self) {
    }

    /// Decompiled virtual method #11 @ RVA 0x00103f80 (3 instructions, 1 basic blocks)
    pub fn method_11(&mut self) {
        // Writes to field at offset +0x0024
    }

    /// Decompiled virtual method #12 @ RVA 0x00109520 (7 instructions, 1 basic blocks)
    pub fn method_12(&mut self) {
        // Writes to field at offset +0x0094
        // Writes to field at offset +0x009c
        // Writes to field at offset +0x00a4
        // Writes to field at offset +0x00b4
        // Writes to field at offset +0x00c0
    }

    /// Decompiled virtual method #13 @ RVA 0x000d3090 (1 instructions, 1 basic blocks)
    pub fn method_13(&mut self) {
    }

    /// Decompiled virtual method #14 @ RVA 0x0010a4e0 (13 instructions, 3 basic blocks)
    pub fn method_14(&mut self) {
        // Writes to field at offset +0x010c
    }

    /// Decompiled virtual method #15 @ RVA 0x0010a100 (47 instructions, 10 basic blocks)
    pub fn method_15(&mut self) {
    }

    /// Decompiled virtual method #16 @ RVA 0x000d3090 (1 instructions, 1 basic blocks)
    pub fn method_16(&mut self) {
    }

    /// Decompiled virtual method #17 @ RVA 0x000d3090 (1 instructions, 1 basic blocks)
    pub fn method_17(&mut self) {
    }

    /// Decompiled virtual method #18 @ RVA 0x000d3090 (1 instructions, 1 basic blocks)
    pub fn method_18(&mut self) {
    }

    /// Decompiled virtual method #19 @ RVA 0x0010a520 (26 instructions, 4 basic blocks)
    pub fn method_19(&mut self) {
        // Writes to field at offset +0x010c
    }

    /// Decompiled virtual method #20 @ RVA 0x006059c0 (120 instructions, 13 basic blocks)
    pub fn method_20(&mut self) {
        // Writes to field at offset +0x0000
        // Writes to field at offset +0x0003
        // Writes to field at offset +0x0060
    }

    /// Decompiled virtual method #21 @ RVA 0x000fe200 (2 instructions, 1 basic blocks)
    pub fn method_21(&mut self) {
    }

    /// Decompiled virtual method #22 @ RVA 0x0010a590 (52 instructions, 9 basic blocks)
    pub fn method_22(&mut self) {
        // Writes to field at offset +0x010c
    }

    /// Decompiled virtual method #23 @ RVA 0x00605770 (61 instructions, 4 basic blocks)
    pub fn method_23(&mut self) {
    }

    /// Decompiled virtual method #24 @ RVA 0x00605890 (62 instructions, 4 basic blocks)
    pub fn method_24(&mut self) {
    }

    /// Decompiled virtual method #25 @ RVA 0x0010a8d0 (117 instructions, 7 basic blocks)
    pub fn method_25(&mut self) {
        // Writes to field at offset +0x0110
    }

    /// Decompiled virtual method #26 @ RVA 0x0010a910 (112 instructions, 9 basic blocks)
    pub fn method_26(&mut self) {
    }

    /// Decompiled virtual method #27 @ RVA 0x0010a690 (107 instructions, 15 basic blocks)
    pub fn method_27(&mut self) {
        // Writes to field at offset +0x010d
    }

    /// Decompiled virtual method #28 @ RVA 0x0010cfd0 (25 instructions, 3 basic blocks)
    pub fn method_28(&mut self) {
        // Writes to field at offset +0x010c
    }

    /// Decompiled virtual method #29 @ RVA 0x000d3090 (1 instructions, 1 basic blocks)
    pub fn method_29(&mut self) {
    }

    /// Decompiled virtual method #30 @ RVA 0x000d3090 (1 instructions, 1 basic blocks)
    pub fn method_30(&mut self) {
    }

    /// Decompiled virtual method #31 @ RVA 0x000d3090 (1 instructions, 1 basic blocks)
    pub fn method_31(&mut self) {
    }

    /// Decompiled virtual method #32 @ RVA 0x000d3090 (1 instructions, 1 basic blocks)
    pub fn method_32(&mut self) {
    }

    /// Decompiled virtual method #33 @ RVA 0x000d3090 (1 instructions, 1 basic blocks)
    pub fn method_33(&mut self) {
    }

    /// Decompiled virtual method #34 @ RVA 0x000fe200 (2 instructions, 1 basic blocks)
    pub fn method_34(&mut self) {
    }

    /// Decompiled virtual method #35 @ RVA 0x000fe200 (2 instructions, 1 basic blocks)
    pub fn method_35(&mut self) {
    }

    /// Decompiled virtual method #36 @ RVA 0x000fe200 (2 instructions, 1 basic blocks)
    pub fn method_36(&mut self) {
    }

    /// Decompiled virtual method #37 @ RVA 0x000fe200 (2 instructions, 1 basic blocks)
    pub fn method_37(&mut self) {
    }

    /// Decompiled virtual method #38 @ RVA 0x000fe200 (2 instructions, 1 basic blocks)
    pub fn method_38(&mut self) {
    }

    /// Decompiled virtual method #39 @ RVA 0x000fe200 (2 instructions, 1 basic blocks)
    pub fn method_39(&mut self) {
    }

    /// Decompiled virtual method #40 @ RVA 0x000fe200 (2 instructions, 1 basic blocks)
    pub fn method_40(&mut self) {
    }

    /// Decompiled virtual method #41 @ RVA 0x000fe200 (2 instructions, 1 basic blocks)
    pub fn method_41(&mut self) {
    }

    /// Decompiled virtual method #42 @ RVA 0x000fe200 (2 instructions, 1 basic blocks)
    pub fn method_42(&mut self) {
    }

    /// Decompiled virtual method #43 @ RVA 0x000d3090 (1 instructions, 1 basic blocks)
    pub fn method_43(&mut self) {
    }

    /// Decompiled virtual method #44 @ RVA 0x00604c10 (16 instructions, 3 basic blocks)
    pub fn method_44(&mut self) {
    }

    /// Decompiled virtual method #45 @ RVA 0x00604d50 (48 instructions, 4 basic blocks)
    pub fn method_45(&mut self) {
    }

    /// Decompiled virtual method #46 @ RVA 0x00109d40 (67 instructions, 5 basic blocks)
    pub fn method_46(&mut self) {
        // Writes to field at offset +0x0118
    }

    /// Decompiled virtual method #47 @ RVA 0x00109e70 (64 instructions, 5 basic blocks)
    pub fn method_47(&mut self) {
        // Writes to field at offset +0x00f8
        // Writes to field at offset +0x00fc
        // Writes to field at offset +0x0100
        // Writes to field at offset +0x0110
        // Writes to field at offset +0x0118
    }

    /// Decompiled virtual method #48 @ RVA 0x00109fa0 (101 instructions, 7 basic blocks)
    pub fn method_48(&mut self) {
    }

    /// Decompiled virtual method #49 @ RVA 0x0010a020 (66 instructions, 5 basic blocks)
    pub fn method_49(&mut self) {
    }

    /// Decompiled virtual method #50 @ RVA 0x0010a090 (26 instructions, 3 basic blocks)
    pub fn method_50(&mut self) {
    }

    /// Decompiled virtual method #51 @ RVA 0x0010a460 (30 instructions, 3 basic blocks)
    pub fn method_51(&mut self) {
        // Writes to field at offset +0x0118
    }

    /// Decompiled virtual method #52 @ RVA 0x0010b660 (126 instructions, 20 basic blocks)
    pub fn method_52(&mut self) {
        // Writes to field at offset +0x009c
    }

    /// Decompiled virtual method #53 @ RVA 0x0010bb20 (131 instructions, 17 basic blocks)
    pub fn method_53(&mut self) {
    }

    /// Decompiled virtual method #54 @ RVA 0x0010bf60 (69 instructions, 11 basic blocks)
    pub fn method_54(&mut self) {
    }

    /// Decompiled virtual method #55 @ RVA 0x0010c090 (98 instructions, 13 basic blocks)
    pub fn method_55(&mut self) {
    }

    /// Decompiled virtual method #56 @ RVA 0x0010c230 (26 instructions, 2 basic blocks)
    pub fn method_56(&mut self) {
    }

    /// Decompiled virtual method #57 @ RVA 0x0010c230 (26 instructions, 2 basic blocks)
    pub fn method_57(&mut self) {
    }

    /// Decompiled virtual method #58 @ RVA 0x0010c340 (29 instructions, 3 basic blocks)
    pub fn method_58(&mut self) {
    }

    /// Decompiled virtual method #59 @ RVA 0x0010c2d0 (29 instructions, 3 basic blocks)
    pub fn method_59(&mut self) {
    }

    /// Decompiled virtual method #60 @ RVA 0x00606090 (13 instructions, 1 basic blocks)
    pub fn method_60(&mut self) {
    }

    /// Decompiled virtual method #61 @ RVA 0x0010b5b0 (23 instructions, 2 basic blocks)
    pub fn method_61(&mut self) {
    }

    /// Decompiled virtual method #62 @ RVA 0x0010ce40 (59 instructions, 6 basic blocks)
    pub fn method_62(&mut self) {
    }

    /// Decompiled virtual method #63 @ RVA 0x0010c510 (52 instructions, 6 basic blocks)
    pub fn method_63(&mut self) {
        // Writes to field at offset +0x0030
        // Writes to field at offset +0x00d0
        // Writes to field at offset +0x00d8
        // Writes to field at offset +0x00f0
    }

}

impl CcClass for ccBookDispWnd {
    fn class_name(&self) -> &'static str { Self::CLASS_NAME }
    fn vtable_rva(&self) -> u32 { Self::VTABLE_RVA }
    fn virtual_method_count(&self) -> usize { Self::METHOD_COUNT }
    fn base_classes(&self) -> &'static [&'static str] { Self::BASE_CLASSES }
}

/// Decompiled CC2 class: `ccBookGauge` (VTable RVA: 0x008ea2c8, 6 virtual methods, 10 recovered fields)
/// Inherits: ccGauge
#[derive(Debug, Clone, Default)]
pub struct ccBookGauge {
    pub vtable_rva: u32,
    pub field_0x0000: u64,
    pub field_0x0008: f32,
    pub field_0x000c: f32,
    pub field_0x0010: f32,
    pub field_0x0014: f32,
    pub field_0x0018: f32,
    pub field_0x001c: f32,
    pub field_0x0024: u32,
    pub field_0x0028: f32,
    pub field_0x002c: f32,
}

impl ccBookGauge {
    pub const CLASS_NAME: &'static str = "ccBookGauge";
    pub const VTABLE_RVA: u32 = 0x008ea2c8;
    pub const METHOD_COUNT: usize = 6;
    pub const BASE_CLASSES: &'static [&'static str] = &["ccGauge"];
    pub const VIRTUAL_METHODS: &'static [u64] = &[0x0000000180609260, 0x00000001800d3090, 0x00000001800d3090, 0x0000000180609290, 0x00000001806092d0, 0x00000001801b8190];

    pub fn new() -> Self {
        Self {
            vtable_rva: Self::VTABLE_RVA,
            field_0x0000: Default::default(),
            field_0x0008: Default::default(),
            field_0x000c: Default::default(),
            field_0x0010: Default::default(),
            field_0x0014: Default::default(),
            field_0x0018: Default::default(),
            field_0x001c: Default::default(),
            field_0x0024: Default::default(),
            field_0x0028: Default::default(),
            field_0x002c: Default::default(),
        }
    }

    /// Decompiled virtual method #0 @ RVA 0x00609260 (13 instructions, 3 basic blocks)
    pub fn method_00(&mut self) {
        // Writes to field at offset +0x0000
    }

    /// Decompiled virtual method #1 @ RVA 0x000d3090 (1 instructions, 1 basic blocks)
    pub fn method_01(&mut self) {
    }

    /// Decompiled virtual method #2 @ RVA 0x000d3090 (1 instructions, 1 basic blocks)
    pub fn method_02(&mut self) {
    }

    /// Decompiled virtual method #3 @ RVA 0x00609290 (14 instructions, 3 basic blocks)
    pub fn method_03(&mut self) {
    }

    /// Decompiled virtual method #4 @ RVA 0x006092d0 (112 instructions, 13 basic blocks)
    pub fn method_04(&mut self) {
    }

    /// Decompiled virtual method #5 @ RVA 0x001b8190 (12 instructions, 1 basic blocks)
    pub fn method_05(&mut self) {
        // Writes to field at offset +0x0024
        // Writes to field at offset +0x0028
        // Writes to field at offset +0x002c
    }

}

impl CcClass for ccBookGauge {
    fn class_name(&self) -> &'static str { Self::CLASS_NAME }
    fn vtable_rva(&self) -> u32 { Self::VTABLE_RVA }
    fn virtual_method_count(&self) -> usize { Self::METHOD_COUNT }
    fn base_classes(&self) -> &'static [&'static str] { Self::BASE_CLASSES }
}

/// Decompiled CC2 class: `ccBookSelWnd` (VTable RVA: 0x008ea558, 64 virtual methods, 39 recovered fields)
/// Inherits: ListWnd, MoveAnmPos
#[derive(Debug, Clone, Default)]
pub struct ccBookSelWnd {
    pub vtable_rva: u32,
    pub field_0x0000: u64,
    pub field_0x0004: u32,
    pub field_0x0008: f32,
    pub field_0x000c: f32,
    pub field_0x0010: u64,
    pub field_0x0018: u64,
    pub field_0x0020: u32,
    pub field_0x0024: u8,
    pub field_0x0026: u8,
    pub field_0x0028: u32,
    pub field_0x002c: u32,
    pub field_0x0030: u64,
    pub field_0x0060: u32,
    pub field_0x0064: u32,
    pub field_0x0068: f32,
    pub field_0x0070: u32,
    pub field_0x0074: u32,
    pub field_0x0078: f32,
    pub field_0x007c: f32,
    pub field_0x008c: u32,
    pub field_0x0094: u64,
    pub field_0x0098: u32,
    pub field_0x009c: u64,
    pub field_0x00a4: u32,
    pub field_0x00ac: f32,
    pub field_0x00b0: u32,
    pub field_0x00b4: u32,
    pub field_0x00c0: u32,
    pub field_0x00c8: u64,
    pub field_0x00d0: u64,
    pub field_0x00d8: u64,
    pub field_0x00f0: u64,
    pub field_0x00f8: u32,
    pub field_0x010c: u8,
    pub field_0x010d: u8,
    pub field_0x0110: u32,
    pub field_0x0118: u64,
    pub field_0x0180: u64,
    pub field_0x01cc: u32,
}

impl ccBookSelWnd {
    pub const CLASS_NAME: &'static str = "ccBookSelWnd";
    pub const VTABLE_RVA: u32 = 0x008ea558;
    pub const METHOD_COUNT: usize = 64;
    pub const BASE_CLASSES: &'static [&'static str] = &["ListWnd", "MoveAnmPos"];
    pub const VIRTUAL_METHODS: &'static [u64] = &[0x0000000180109550, 0x0000000180109c20, 0x0000000180603420, 0x0000000180603a90, 0x000000018057b520, 0x000000018057b450, 0x000000018010c440, 0x000000018057b5a0, 0x0000000180103f70, 0x000000018057b440, 0x000000018010cf30, 0x0000000180103f80, 0x0000000180109520, 0x00000001800d3090, 0x000000018010a4e0, 0x000000018010a100, 0x00000001800d3090, 0x00000001800d3090, 0x00000001800d3090, 0x0000000180604a00, 0x0000000180603f00, 0x0000000180604580, 0x000000018010a590, 0x000000018010a670, 0x000000018010a680, 0x000000018010a8d0, 0x000000018010a910, 0x000000018010a690, 0x000000018010cfd0, 0x00000001800d3090, 0x00000001800d3090, 0x00000001800d3090, 0x00000001800d3090, 0x00000001800d3090, 0x00000001800fe200, 0x00000001800fe200, 0x00000001800fe200, 0x00000001800fe200, 0x00000001800fe200, 0x00000001800fe200, 0x00000001800fe200, 0x00000001800fe200, 0x00000001800fe200, 0x00000001800d3090, 0x0000000180603010, 0x00000001806031d0, 0x0000000180603cd0, 0x0000000180603ef0, 0x0000000180109fa0, 0x000000018010a020, 0x000000018010a090, 0x000000018010a460, 0x000000018010b660, 0x000000018010bb20, 0x000000018010bf60, 0x000000018010c090, 0x000000018010c230, 0x000000018010c230, 0x000000018010c340, 0x000000018010c2d0, 0x000000018010b3b0, 0x000000018010b5b0, 0x000000018010ce40, 0x000000018010c510];

    pub fn new() -> Self {
        Self {
            vtable_rva: Self::VTABLE_RVA,
            field_0x0000: Default::default(),
            field_0x0004: Default::default(),
            field_0x0008: Default::default(),
            field_0x000c: Default::default(),
            field_0x0010: Default::default(),
            field_0x0018: Default::default(),
            field_0x0020: Default::default(),
            field_0x0024: Default::default(),
            field_0x0026: Default::default(),
            field_0x0028: Default::default(),
            field_0x002c: Default::default(),
            field_0x0030: Default::default(),
            field_0x0060: Default::default(),
            field_0x0064: Default::default(),
            field_0x0068: Default::default(),
            field_0x0070: Default::default(),
            field_0x0074: Default::default(),
            field_0x0078: Default::default(),
            field_0x007c: Default::default(),
            field_0x008c: Default::default(),
            field_0x0094: Default::default(),
            field_0x0098: Default::default(),
            field_0x009c: Default::default(),
            field_0x00a4: Default::default(),
            field_0x00ac: Default::default(),
            field_0x00b0: Default::default(),
            field_0x00b4: Default::default(),
            field_0x00c0: Default::default(),
            field_0x00c8: Default::default(),
            field_0x00d0: Default::default(),
            field_0x00d8: Default::default(),
            field_0x00f0: Default::default(),
            field_0x00f8: Default::default(),
            field_0x010c: Default::default(),
            field_0x010d: Default::default(),
            field_0x0110: Default::default(),
            field_0x0118: Default::default(),
            field_0x0180: Default::default(),
            field_0x01cc: Default::default(),
        }
    }

    /// Decompiled virtual method #0 @ RVA 0x00109550 (46 instructions, 1 basic blocks)
    pub fn method_00(&mut self) {
    }

    /// Decompiled virtual method #1 @ RVA 0x00109c20 (130 instructions, 13 basic blocks)
    pub fn method_01(&mut self) {
        // Writes to field at offset +0x0118
    }

    /// Decompiled virtual method #2 @ RVA 0x00603420 (126 instructions, 29 basic blocks)
    pub fn method_02(&mut self) {
    }

    /// Decompiled virtual method #3 @ RVA 0x00603a90 (100 instructions, 7 basic blocks)
    pub fn method_03(&mut self) {
        // Writes to field at offset +0x00f8
    }

    /// Decompiled virtual method #4 @ RVA 0x0057b520 (30 instructions, 4 basic blocks)
    pub fn method_04(&mut self) {
    }

    /// Decompiled virtual method #5 @ RVA 0x0057b450 (90 instructions, 7 basic blocks)
    pub fn method_05(&mut self) {
    }

    /// Decompiled virtual method #6 @ RVA 0x0010c440 (24 instructions, 3 basic blocks)
    pub fn method_06(&mut self) {
    }

    /// Decompiled virtual method #7 @ RVA 0x0057b5a0 (7 instructions, 1 basic blocks)
    pub fn method_07(&mut self) {
        // Writes to field at offset +0x000c
        // Writes to field at offset +0x0026
    }

    /// Decompiled virtual method #8 @ RVA 0x00103f70 (2 instructions, 1 basic blocks)
    pub fn method_08(&mut self) {
        // Writes to field at offset +0x0020
    }

    /// Decompiled virtual method #9 @ RVA 0x0057b440 (98 instructions, 8 basic blocks)
    pub fn method_09(&mut self) {
        // Writes to field at offset +0x0008
    }

    /// Decompiled virtual method #10 @ RVA 0x0010cf30 (8 instructions, 3 basic blocks)
    pub fn method_10(&mut self) {
    }

    /// Decompiled virtual method #11 @ RVA 0x00103f80 (3 instructions, 1 basic blocks)
    pub fn method_11(&mut self) {
        // Writes to field at offset +0x0024
    }

    /// Decompiled virtual method #12 @ RVA 0x00109520 (7 instructions, 1 basic blocks)
    pub fn method_12(&mut self) {
        // Writes to field at offset +0x0094
        // Writes to field at offset +0x009c
        // Writes to field at offset +0x00a4
        // Writes to field at offset +0x00b4
        // Writes to field at offset +0x00c0
    }

    /// Decompiled virtual method #13 @ RVA 0x000d3090 (1 instructions, 1 basic blocks)
    pub fn method_13(&mut self) {
    }

    /// Decompiled virtual method #14 @ RVA 0x0010a4e0 (13 instructions, 3 basic blocks)
    pub fn method_14(&mut self) {
        // Writes to field at offset +0x010c
    }

    /// Decompiled virtual method #15 @ RVA 0x0010a100 (47 instructions, 10 basic blocks)
    pub fn method_15(&mut self) {
    }

    /// Decompiled virtual method #16 @ RVA 0x000d3090 (1 instructions, 1 basic blocks)
    pub fn method_16(&mut self) {
    }

    /// Decompiled virtual method #17 @ RVA 0x000d3090 (1 instructions, 1 basic blocks)
    pub fn method_17(&mut self) {
    }

    /// Decompiled virtual method #18 @ RVA 0x000d3090 (1 instructions, 1 basic blocks)
    pub fn method_18(&mut self) {
    }

    /// Decompiled virtual method #19 @ RVA 0x00604a00 (63 instructions, 4 basic blocks)
    pub fn method_19(&mut self) {
        // Writes to field at offset +0x010c
    }

    /// Decompiled virtual method #20 @ RVA 0x00603f00 (112 instructions, 10 basic blocks)
    pub fn method_20(&mut self) {
    }

    /// Decompiled virtual method #21 @ RVA 0x00604580 (35 instructions, 1 basic blocks)
    pub fn method_21(&mut self) {
    }

    /// Decompiled virtual method #22 @ RVA 0x0010a590 (52 instructions, 9 basic blocks)
    pub fn method_22(&mut self) {
        // Writes to field at offset +0x010c
    }

    /// Decompiled virtual method #23 @ RVA 0x0010a670 (113 instructions, 16 basic blocks)
    pub fn method_23(&mut self) {
        // Writes to field at offset +0x010d
    }

    /// Decompiled virtual method #24 @ RVA 0x0010a680 (110 instructions, 16 basic blocks)
    pub fn method_24(&mut self) {
        // Writes to field at offset +0x010d
    }

    /// Decompiled virtual method #25 @ RVA 0x0010a8d0 (117 instructions, 7 basic blocks)
    pub fn method_25(&mut self) {
        // Writes to field at offset +0x0110
    }

    /// Decompiled virtual method #26 @ RVA 0x0010a910 (112 instructions, 9 basic blocks)
    pub fn method_26(&mut self) {
    }

    /// Decompiled virtual method #27 @ RVA 0x0010a690 (107 instructions, 15 basic blocks)
    pub fn method_27(&mut self) {
        // Writes to field at offset +0x010d
    }

    /// Decompiled virtual method #28 @ RVA 0x0010cfd0 (25 instructions, 3 basic blocks)
    pub fn method_28(&mut self) {
        // Writes to field at offset +0x010c
    }

    /// Decompiled virtual method #29 @ RVA 0x000d3090 (1 instructions, 1 basic blocks)
    pub fn method_29(&mut self) {
    }

    /// Decompiled virtual method #30 @ RVA 0x000d3090 (1 instructions, 1 basic blocks)
    pub fn method_30(&mut self) {
    }

    /// Decompiled virtual method #31 @ RVA 0x000d3090 (1 instructions, 1 basic blocks)
    pub fn method_31(&mut self) {
    }

    /// Decompiled virtual method #32 @ RVA 0x000d3090 (1 instructions, 1 basic blocks)
    pub fn method_32(&mut self) {
    }

    /// Decompiled virtual method #33 @ RVA 0x000d3090 (1 instructions, 1 basic blocks)
    pub fn method_33(&mut self) {
    }

    /// Decompiled virtual method #34 @ RVA 0x000fe200 (2 instructions, 1 basic blocks)
    pub fn method_34(&mut self) {
    }

    /// Decompiled virtual method #35 @ RVA 0x000fe200 (2 instructions, 1 basic blocks)
    pub fn method_35(&mut self) {
    }

    /// Decompiled virtual method #36 @ RVA 0x000fe200 (2 instructions, 1 basic blocks)
    pub fn method_36(&mut self) {
    }

    /// Decompiled virtual method #37 @ RVA 0x000fe200 (2 instructions, 1 basic blocks)
    pub fn method_37(&mut self) {
    }

    /// Decompiled virtual method #38 @ RVA 0x000fe200 (2 instructions, 1 basic blocks)
    pub fn method_38(&mut self) {
    }

    /// Decompiled virtual method #39 @ RVA 0x000fe200 (2 instructions, 1 basic blocks)
    pub fn method_39(&mut self) {
    }

    /// Decompiled virtual method #40 @ RVA 0x000fe200 (2 instructions, 1 basic blocks)
    pub fn method_40(&mut self) {
    }

    /// Decompiled virtual method #41 @ RVA 0x000fe200 (2 instructions, 1 basic blocks)
    pub fn method_41(&mut self) {
    }

    /// Decompiled virtual method #42 @ RVA 0x000fe200 (2 instructions, 1 basic blocks)
    pub fn method_42(&mut self) {
    }

    /// Decompiled virtual method #43 @ RVA 0x000d3090 (1 instructions, 1 basic blocks)
    pub fn method_43(&mut self) {
    }

    /// Decompiled virtual method #44 @ RVA 0x00603010 (16 instructions, 3 basic blocks)
    pub fn method_44(&mut self) {
    }

    /// Decompiled virtual method #45 @ RVA 0x006031d0 (114 instructions, 19 basic blocks)
    pub fn method_45(&mut self) {
    }

    /// Decompiled virtual method #46 @ RVA 0x00603cd0 (100 instructions, 1 basic blocks)
    pub fn method_46(&mut self) {
        // Writes to field at offset +0x0000
        // Writes to field at offset +0x0004
        // Writes to field at offset +0x0008
        // Writes to field at offset +0x000c
        // Writes to field at offset +0x0010
        // Writes to field at offset +0x0018
        // Writes to field at offset +0x0020
        // Writes to field at offset +0x0028
    }

    /// Decompiled virtual method #47 @ RVA 0x00603ef0 (121 instructions, 11 basic blocks)
    pub fn method_47(&mut self) {
    }

    /// Decompiled virtual method #48 @ RVA 0x00109fa0 (101 instructions, 7 basic blocks)
    pub fn method_48(&mut self) {
    }

    /// Decompiled virtual method #49 @ RVA 0x0010a020 (66 instructions, 5 basic blocks)
    pub fn method_49(&mut self) {
    }

    /// Decompiled virtual method #50 @ RVA 0x0010a090 (26 instructions, 3 basic blocks)
    pub fn method_50(&mut self) {
    }

    /// Decompiled virtual method #51 @ RVA 0x0010a460 (30 instructions, 3 basic blocks)
    pub fn method_51(&mut self) {
        // Writes to field at offset +0x0118
    }

    /// Decompiled virtual method #52 @ RVA 0x0010b660 (126 instructions, 20 basic blocks)
    pub fn method_52(&mut self) {
        // Writes to field at offset +0x009c
    }

    /// Decompiled virtual method #53 @ RVA 0x0010bb20 (131 instructions, 17 basic blocks)
    pub fn method_53(&mut self) {
    }

    /// Decompiled virtual method #54 @ RVA 0x0010bf60 (69 instructions, 11 basic blocks)
    pub fn method_54(&mut self) {
    }

    /// Decompiled virtual method #55 @ RVA 0x0010c090 (98 instructions, 13 basic blocks)
    pub fn method_55(&mut self) {
    }

    /// Decompiled virtual method #56 @ RVA 0x0010c230 (26 instructions, 2 basic blocks)
    pub fn method_56(&mut self) {
    }

    /// Decompiled virtual method #57 @ RVA 0x0010c230 (26 instructions, 2 basic blocks)
    pub fn method_57(&mut self) {
    }

    /// Decompiled virtual method #58 @ RVA 0x0010c340 (29 instructions, 3 basic blocks)
    pub fn method_58(&mut self) {
    }

    /// Decompiled virtual method #59 @ RVA 0x0010c2d0 (29 instructions, 3 basic blocks)
    pub fn method_59(&mut self) {
    }

    /// Decompiled virtual method #60 @ RVA 0x0010b3b0 (11 instructions, 1 basic blocks)
    pub fn method_60(&mut self) {
    }

    /// Decompiled virtual method #61 @ RVA 0x0010b5b0 (23 instructions, 2 basic blocks)
    pub fn method_61(&mut self) {
    }

    /// Decompiled virtual method #62 @ RVA 0x0010ce40 (59 instructions, 6 basic blocks)
    pub fn method_62(&mut self) {
    }

    /// Decompiled virtual method #63 @ RVA 0x0010c510 (52 instructions, 6 basic blocks)
    pub fn method_63(&mut self) {
        // Writes to field at offset +0x0030
        // Writes to field at offset +0x00d0
        // Writes to field at offset +0x00d8
        // Writes to field at offset +0x00f0
    }

}

impl CcClass for ccBookSelWnd {
    fn class_name(&self) -> &'static str { Self::CLASS_NAME }
    fn vtable_rva(&self) -> u32 { Self::VTABLE_RVA }
    fn virtual_method_count(&self) -> usize { Self::METHOD_COUNT }
    fn base_classes(&self) -> &'static [&'static str] { Self::BASE_CLASSES }
}

/// Decompiled CC2 class: `ccBuyMenuBase` (VTable RVA: 0x00894fd8, 45 virtual methods, 10 recovered fields)
/// Inherits: ccGameMenuCtrlBase, ccGameMenuObj
#[derive(Debug, Clone, Default)]
pub struct ccBuyMenuBase {
    pub vtable_rva: u32,
    pub field_0x0000: u64,
    pub field_0x0008: u32,
    pub field_0x000c: u32,
    pub field_0x0018: u32,
    pub field_0x0030: u64,
    pub field_0x0048: u64,
    pub field_0x0050: u64,
    pub field_0x0060: u64,
    pub field_0x009c: u32,
    pub field_0x02b0: u64,
}

impl ccBuyMenuBase {
    pub const CLASS_NAME: &'static str = "ccBuyMenuBase";
    pub const VTABLE_RVA: u32 = 0x00894fd8;
    pub const METHOD_COUNT: usize = 45;
    pub const BASE_CLASSES: &'static [&'static str] = &["ccGameMenuCtrlBase", "ccGameMenuObj"];
    pub const VIRTUAL_METHODS: &'static [u64] = &[0x0000000180265b60, 0x00000001804520b0, 0x0000000180452130, 0x0000000180451fb0, 0x0000000180451ff0, 0x0000000180452030, 0x0000000180452070, 0x0000000180103e90, 0x0000000180103ea0, 0x0000000180103e60, 0x000000018045bb30, 0x0000000180451d90, 0x00000001801e9580, 0x0000000180265cd0, 0x0000000180103e60, 0x0000000180451dc0, 0x0000000180451e30, 0x0000000180451ea0, 0x0000000180451ee0, 0x0000000180451f40, 0x0000000180265f00, 0x0000000180266120, 0x00000001801ee9b0, 0x0000000180103e60, 0x0000000180103e60, 0x0000000180103e70, 0x0000000180103e70, 0x0000000180103e60, 0x0000000180103e60, 0x0000000180103e60, 0x0000000180266200, 0x0000000180266430, 0x00000001800d3090, 0x0000000180103e70, 0x0000000180103e70, 0x0000000180103e70, 0x0000000180103e60, 0x000000018073e2c0, 0x000000018073e2c0, 0x000000018073e2c0, 0x000000018073e2c0, 0x000000018073e2c0, 0x000000018073e2c0, 0x000000018073e2c0, 0x000000018073e2c0];

    pub fn new() -> Self {
        Self {
            vtable_rva: Self::VTABLE_RVA,
            field_0x0000: Default::default(),
            field_0x0008: Default::default(),
            field_0x000c: Default::default(),
            field_0x0018: Default::default(),
            field_0x0030: Default::default(),
            field_0x0048: Default::default(),
            field_0x0050: Default::default(),
            field_0x0060: Default::default(),
            field_0x009c: Default::default(),
            field_0x02b0: Default::default(),
        }
    }

    /// Decompiled virtual method #0 @ RVA 0x00265b60 (16 instructions, 3 basic blocks)
    pub fn method_00(&mut self) {
    }

    /// Decompiled virtual method #1 @ RVA 0x004520b0 (10 instructions, 2 basic blocks)
    pub fn method_01(&mut self) {
    }

    /// Decompiled virtual method #2 @ RVA 0x00452130 (35 instructions, 9 basic blocks)
    pub fn method_02(&mut self) {
    }

    /// Decompiled virtual method #3 @ RVA 0x00451fb0 (118 instructions, 2 basic blocks)
    pub fn method_03(&mut self) {
    }

    /// Decompiled virtual method #4 @ RVA 0x00451ff0 (91 instructions, 2 basic blocks)
    pub fn method_04(&mut self) {
    }

    /// Decompiled virtual method #5 @ RVA 0x00452030 (64 instructions, 2 basic blocks)
    pub fn method_05(&mut self) {
    }

    /// Decompiled virtual method #6 @ RVA 0x00452070 (37 instructions, 2 basic blocks)
    pub fn method_06(&mut self) {
    }

    /// Decompiled virtual method #7 @ RVA 0x00103e90 (2 instructions, 1 basic blocks)
    pub fn method_07(&mut self) {
    }

    /// Decompiled virtual method #8 @ RVA 0x00103ea0 (9 instructions, 1 basic blocks)
    pub fn method_08(&mut self) {
        // Writes to field at offset +0x0008
    }

    /// Decompiled virtual method #9 @ RVA 0x00103e60 (2 instructions, 1 basic blocks)
    pub fn method_09(&mut self) {
    }

    /// Decompiled virtual method #10 @ RVA 0x0045bb30 (16 instructions, 2 basic blocks)
    pub fn method_10(&mut self) {
    }

    /// Decompiled virtual method #11 @ RVA 0x00451d90 (24 instructions, 2 basic blocks)
    pub fn method_11(&mut self) {
    }

    /// Decompiled virtual method #12 @ RVA 0x001e9580 (53 instructions, 12 basic blocks)
    pub fn method_12(&mut self) {
    }

    /// Decompiled virtual method #13 @ RVA 0x00265cd0 (108 instructions, 6 basic blocks)
    pub fn method_13(&mut self) {
    }

    /// Decompiled virtual method #14 @ RVA 0x00103e60 (2 instructions, 1 basic blocks)
    pub fn method_14(&mut self) {
    }

    /// Decompiled virtual method #15 @ RVA 0x00451dc0 (10 instructions, 2 basic blocks)
    pub fn method_15(&mut self) {
    }

    /// Decompiled virtual method #16 @ RVA 0x00451e30 (10 instructions, 2 basic blocks)
    pub fn method_16(&mut self) {
    }

    /// Decompiled virtual method #17 @ RVA 0x00451ea0 (31 instructions, 2 basic blocks)
    pub fn method_17(&mut self) {
        // Writes to field at offset +0x0018
    }

    /// Decompiled virtual method #18 @ RVA 0x00451ee0 (10 instructions, 2 basic blocks)
    pub fn method_18(&mut self) {
    }

    /// Decompiled virtual method #19 @ RVA 0x00451f40 (10 instructions, 2 basic blocks)
    pub fn method_19(&mut self) {
    }

    /// Decompiled virtual method #20 @ RVA 0x00265f00 (117 instructions, 12 basic blocks)
    pub fn method_20(&mut self) {
        // Writes to field at offset +0x02b0
    }

    /// Decompiled virtual method #21 @ RVA 0x00266120 (50 instructions, 8 basic blocks)
    pub fn method_21(&mut self) {
        // Writes to field at offset +0x0008
        // Writes to field at offset +0x000c
    }

    /// Decompiled virtual method #22 @ RVA 0x001ee9b0 (15 instructions, 1 basic blocks)
    pub fn method_22(&mut self) {
    }

    /// Decompiled virtual method #23 @ RVA 0x00103e60 (2 instructions, 1 basic blocks)
    pub fn method_23(&mut self) {
    }

    /// Decompiled virtual method #24 @ RVA 0x00103e60 (2 instructions, 1 basic blocks)
    pub fn method_24(&mut self) {
    }

    /// Decompiled virtual method #25 @ RVA 0x00103e70 (2 instructions, 1 basic blocks)
    pub fn method_25(&mut self) {
    }

    /// Decompiled virtual method #26 @ RVA 0x00103e70 (2 instructions, 1 basic blocks)
    pub fn method_26(&mut self) {
    }

    /// Decompiled virtual method #27 @ RVA 0x00103e60 (2 instructions, 1 basic blocks)
    pub fn method_27(&mut self) {
    }

    /// Decompiled virtual method #28 @ RVA 0x00103e60 (2 instructions, 1 basic blocks)
    pub fn method_28(&mut self) {
    }

    /// Decompiled virtual method #29 @ RVA 0x00103e60 (2 instructions, 1 basic blocks)
    pub fn method_29(&mut self) {
    }

    /// Decompiled virtual method #30 @ RVA 0x00266200 (105 instructions, 19 basic blocks)
    pub fn method_30(&mut self) {
    }

    /// Decompiled virtual method #31 @ RVA 0x00266430 (90 instructions, 5 basic blocks)
    pub fn method_31(&mut self) {
    }

    /// Decompiled virtual method #32 @ RVA 0x000d3090 (1 instructions, 1 basic blocks)
    pub fn method_32(&mut self) {
    }

    /// Decompiled virtual method #33 @ RVA 0x00103e70 (2 instructions, 1 basic blocks)
    pub fn method_33(&mut self) {
    }

    /// Decompiled virtual method #34 @ RVA 0x00103e70 (2 instructions, 1 basic blocks)
    pub fn method_34(&mut self) {
    }

    /// Decompiled virtual method #35 @ RVA 0x00103e70 (2 instructions, 1 basic blocks)
    pub fn method_35(&mut self) {
    }

    /// Decompiled virtual method #36 @ RVA 0x00103e60 (2 instructions, 1 basic blocks)
    pub fn method_36(&mut self) {
    }

    /// Decompiled virtual method #37 @ RVA 0x0073e2c0 (337 instructions, 1 basic blocks)
    pub fn method_37(&mut self) {
    }

    /// Decompiled virtual method #38 @ RVA 0x0073e2c0 (337 instructions, 1 basic blocks)
    pub fn method_38(&mut self) {
    }

    /// Decompiled virtual method #39 @ RVA 0x0073e2c0 (337 instructions, 1 basic blocks)
    pub fn method_39(&mut self) {
    }

    /// Decompiled virtual method #40 @ RVA 0x0073e2c0 (337 instructions, 1 basic blocks)
    pub fn method_40(&mut self) {
    }

    /// Decompiled virtual method #41 @ RVA 0x0073e2c0 (337 instructions, 1 basic blocks)
    pub fn method_41(&mut self) {
    }

    /// Decompiled virtual method #42 @ RVA 0x0073e2c0 (337 instructions, 1 basic blocks)
    pub fn method_42(&mut self) {
    }

    /// Decompiled virtual method #43 @ RVA 0x0073e2c0 (337 instructions, 1 basic blocks)
    pub fn method_43(&mut self) {
    }

    /// Decompiled virtual method #44 @ RVA 0x0073e2c0 (337 instructions, 1 basic blocks)
    pub fn method_44(&mut self) {
    }

}

impl CcClass for ccBuyMenuBase {
    fn class_name(&self) -> &'static str { Self::CLASS_NAME }
    fn vtable_rva(&self) -> u32 { Self::VTABLE_RVA }
    fn virtual_method_count(&self) -> usize { Self::METHOD_COUNT }
    fn base_classes(&self) -> &'static [&'static str] { Self::BASE_CLASSES }
}

/// Decompiled CC2 class: `ccCmnWnd` (VTable RVA: 0x0087bf78, 9 virtual methods, 43 recovered fields)
/// Inherits: ccWindowBase
#[derive(Debug, Clone, Default)]
pub struct ccCmnWnd {
    pub vtable_rva: u32,
    pub field_0x0000: u64,
    pub field_0x0003: u8,
    pub field_0x0008: u64,
    pub field_0x0010: f32,
    pub field_0x0014: f32,
    pub field_0x001c: u32,
    pub field_0x0020: u32,
    pub field_0x0024: f32,
    pub field_0x0028: f32,
    pub field_0x002c: f32,
    pub field_0x0030: u8,
    pub field_0x0031: u8,
    pub field_0x0032: u8,
    pub field_0x0033: u8,
    pub field_0x0038: f32,
    pub field_0x003c: f32,
    pub field_0x0040: u64,
    pub field_0x0048: u32,
    pub field_0x004c: f32,
    pub field_0x0050: f32,
    pub field_0x0094: f32,
    pub field_0x0098: f32,
    pub field_0x009c: u32,
    pub field_0x00a0: u32,
    pub field_0x00b8: u32,
    pub field_0x00c8: u32,
    pub field_0x00cc: u32,
    pub field_0x00d0: u32,
    pub field_0x00d4: u64,
    pub field_0x00dc: u32,
    pub field_0x00e0: u16,
    pub field_0x0124: f32,
    pub field_0x0128: f32,
    pub field_0x0158: u32,
    pub field_0x0159: u8,
    pub field_0x015a: u8,
    pub field_0x015c: u32,
    pub field_0x0160: u32,
    pub field_0x0164: u32,
    pub field_0x0168: u32,
    pub field_0x016c: u32,
    pub field_0x0170: u32,
    pub field_0x0174: u8,
}

impl ccCmnWnd {
    pub const CLASS_NAME: &'static str = "ccCmnWnd";
    pub const VTABLE_RVA: u32 = 0x0087bf78;
    pub const METHOD_COUNT: usize = 9;
    pub const BASE_CLASSES: &'static [&'static str] = &["ccWindowBase"];
    pub const VIRTUAL_METHODS: &'static [u64] = &[0x00000001801b8100, 0x0000000180670870, 0x000000018057b6a0, 0x0000000180670b40, 0x0000000180670ad0, 0x0000000180671270, 0x0000000180671350, 0x00000001806713e0, 0x0000000180671490];

    pub fn new() -> Self {
        Self {
            vtable_rva: Self::VTABLE_RVA,
            field_0x0000: Default::default(),
            field_0x0003: Default::default(),
            field_0x0008: Default::default(),
            field_0x0010: Default::default(),
            field_0x0014: Default::default(),
            field_0x001c: Default::default(),
            field_0x0020: Default::default(),
            field_0x0024: Default::default(),
            field_0x0028: Default::default(),
            field_0x002c: Default::default(),
            field_0x0030: Default::default(),
            field_0x0031: Default::default(),
            field_0x0032: Default::default(),
            field_0x0033: Default::default(),
            field_0x0038: Default::default(),
            field_0x003c: Default::default(),
            field_0x0040: Default::default(),
            field_0x0048: Default::default(),
            field_0x004c: Default::default(),
            field_0x0050: Default::default(),
            field_0x0094: Default::default(),
            field_0x0098: Default::default(),
            field_0x009c: Default::default(),
            field_0x00a0: Default::default(),
            field_0x00b8: Default::default(),
            field_0x00c8: Default::default(),
            field_0x00cc: Default::default(),
            field_0x00d0: Default::default(),
            field_0x00d4: Default::default(),
            field_0x00dc: Default::default(),
            field_0x00e0: Default::default(),
            field_0x0124: Default::default(),
            field_0x0128: Default::default(),
            field_0x0158: Default::default(),
            field_0x0159: Default::default(),
            field_0x015a: Default::default(),
            field_0x015c: Default::default(),
            field_0x0160: Default::default(),
            field_0x0164: Default::default(),
            field_0x0168: Default::default(),
            field_0x016c: Default::default(),
            field_0x0170: Default::default(),
            field_0x0174: Default::default(),
        }
    }

    /// Decompiled virtual method #0 @ RVA 0x001b8100 (18 instructions, 3 basic blocks)
    pub fn method_00(&mut self) {
        // Writes to field at offset +0x0000
    }

    /// Decompiled virtual method #1 @ RVA 0x00670870 (99 instructions, 8 basic blocks)
    pub fn method_01(&mut self) {
        // Writes to field at offset +0x0040
        // Writes to field at offset +0x0048
        // Writes to field at offset +0x00c8
        // Writes to field at offset +0x00cc
        // Writes to field at offset +0x00d0
        // Writes to field at offset +0x00d4
        // Writes to field at offset +0x00dc
        // Writes to field at offset +0x00e0
        // Writes to field at offset +0x0158
        // Writes to field at offset +0x015c
        // Writes to field at offset +0x0160
        // Writes to field at offset +0x0164
        // Writes to field at offset +0x0168
        // Writes to field at offset +0x016c
        // Writes to field at offset +0x0170
        // Writes to field at offset +0x0174
    }

    /// Decompiled virtual method #2 @ RVA 0x0057b6a0 (7 instructions, 1 basic blocks)
    pub fn method_02(&mut self) {
        // Writes to field at offset +0x0010
        // Writes to field at offset +0x0014
        // Writes to field at offset +0x0094
        // Writes to field at offset +0x0098
        // Writes to field at offset +0x0124
        // Writes to field at offset +0x0128
    }

    /// Decompiled virtual method #3 @ RVA 0x00670b40 (36 instructions, 5 basic blocks)
    pub fn method_03(&mut self) {
        // Writes to field at offset +0x002c
        // Writes to field at offset +0x009c
        // Writes to field at offset +0x00a0
    }

    /// Decompiled virtual method #4 @ RVA 0x00670ad0 (24 instructions, 6 basic blocks)
    pub fn method_04(&mut self) {
        // Writes to field at offset +0x0024
        // Writes to field at offset +0x0028
        // Writes to field at offset +0x004c
        // Writes to field at offset +0x0050
    }

    /// Decompiled virtual method #5 @ RVA 0x00671270 (49 instructions, 1 basic blocks)
    pub fn method_05(&mut self) {
    }

    /// Decompiled virtual method #6 @ RVA 0x00671350 (34 instructions, 1 basic blocks)
    pub fn method_06(&mut self) {
    }

    /// Decompiled virtual method #7 @ RVA 0x006713e0 (37 instructions, 6 basic blocks)
    pub fn method_07(&mut self) {
        // Writes to field at offset +0x0000
        // Writes to field at offset +0x0003
        // Writes to field at offset +0x0038
        // Writes to field at offset +0x00b8
        // Writes to field at offset +0x015a
        // Writes to field at offset +0x0174
    }

    /// Decompiled virtual method #8 @ RVA 0x00671490 (6 instructions, 3 basic blocks)
    pub fn method_08(&mut self) {
        // Writes to field at offset +0x0159
    }

}

impl CcClass for ccCmnWnd {
    fn class_name(&self) -> &'static str { Self::CLASS_NAME }
    fn vtable_rva(&self) -> u32 { Self::VTABLE_RVA }
    fn virtual_method_count(&self) -> usize { Self::METHOD_COUNT }
    fn base_classes(&self) -> &'static [&'static str] { Self::BASE_CLASSES }
}

/// Decompiled CC2 class: `ccComboDispCtrl` (VTable RVA: 0x008815a8, 14 virtual methods, 20 recovered fields)
/// Inherits: ccDispBase
#[derive(Debug, Clone, Default)]
pub struct ccComboDispCtrl {
    pub vtable_rva: u32,
    pub field_0x0000: u64,
    pub field_0x0008: u32,
    pub field_0x000c: u32,
    pub field_0x0010: u32,
    pub field_0x0014: f32,
    pub field_0x0018: f32,
    pub field_0x001c: u32,
    pub field_0x0028: u32,
    pub field_0x002c: u32,
    pub field_0x0030: u32,
    pub field_0x0034: u32,
    pub field_0x0038: u64,
    pub field_0x0040: u64,
    pub field_0x0058: u64,
    pub field_0x0060: u64,
    pub field_0x0068: u32,
    pub field_0x0070: u32,
    pub field_0x00ac: f32,
    pub field_0x02da: u16,
    pub field_0x0314: u16,
}

impl ccComboDispCtrl {
    pub const CLASS_NAME: &'static str = "ccComboDispCtrl";
    pub const VTABLE_RVA: u32 = 0x008815a8;
    pub const METHOD_COUNT: usize = 14;
    pub const BASE_CLASSES: &'static [&'static str] = &["ccDispBase"];
    pub const VIRTUAL_METHODS: &'static [u64] = &[0x00000001802361c0, 0x00000001802361f0, 0x0000000180236560, 0x0000000180236ef0, 0x00000001802a0e30, 0x00000001800d3090, 0x00000001801b81f0, 0x00000001801b8200, 0x00000001801b8210, 0x0000000180236920, 0x00000001802369d0, 0x0000000180236780, 0x00000001800d3090, 0x0000000180236850];

    pub fn new() -> Self {
        Self {
            vtable_rva: Self::VTABLE_RVA,
            field_0x0000: Default::default(),
            field_0x0008: Default::default(),
            field_0x000c: Default::default(),
            field_0x0010: Default::default(),
            field_0x0014: Default::default(),
            field_0x0018: Default::default(),
            field_0x001c: Default::default(),
            field_0x0028: Default::default(),
            field_0x002c: Default::default(),
            field_0x0030: Default::default(),
            field_0x0034: Default::default(),
            field_0x0038: Default::default(),
            field_0x0040: Default::default(),
            field_0x0058: Default::default(),
            field_0x0060: Default::default(),
            field_0x0068: Default::default(),
            field_0x0070: Default::default(),
            field_0x00ac: Default::default(),
            field_0x02da: Default::default(),
            field_0x0314: Default::default(),
        }
    }

    /// Decompiled virtual method #0 @ RVA 0x002361c0 (13 instructions, 3 basic blocks)
    pub fn method_00(&mut self) {
        // Writes to field at offset +0x0000
    }

    /// Decompiled virtual method #1 @ RVA 0x002361f0 (120 instructions, 17 basic blocks)
    pub fn method_01(&mut self) {
        // Writes to field at offset +0x0068
    }

    /// Decompiled virtual method #2 @ RVA 0x00236560 (93 instructions, 25 basic blocks)
    pub fn method_02(&mut self) {
    }

    /// Decompiled virtual method #3 @ RVA 0x00236ef0 (75 instructions, 11 basic blocks)
    pub fn method_03(&mut self) {
        // Writes to field at offset +0x0008
    }

    /// Decompiled virtual method #4 @ RVA 0x002a0e30 (48 instructions, 11 basic blocks)
    pub fn method_04(&mut self) {
        // Writes to field at offset +0x0008
    }

    /// Decompiled virtual method #5 @ RVA 0x000d3090 (1 instructions, 1 basic blocks)
    pub fn method_05(&mut self) {
    }

    /// Decompiled virtual method #6 @ RVA 0x001b81f0 (2 instructions, 1 basic blocks)
    pub fn method_06(&mut self) {
        // Writes to field at offset +0x0008
    }

    /// Decompiled virtual method #7 @ RVA 0x001b8200 (5 instructions, 1 basic blocks)
    pub fn method_07(&mut self) {
    }

    /// Decompiled virtual method #8 @ RVA 0x001b8210 (5 instructions, 1 basic blocks)
    pub fn method_08(&mut self) {
    }

    /// Decompiled virtual method #9 @ RVA 0x00236920 (48 instructions, 15 basic blocks)
    pub fn method_09(&mut self) {
        // Writes to field at offset +0x0070
        // Writes to field at offset +0x02da
        // Writes to field at offset +0x0314
    }

    /// Decompiled virtual method #10 @ RVA 0x002369d0 (108 instructions, 6 basic blocks)
    pub fn method_10(&mut self) {
    }

    /// Decompiled virtual method #11 @ RVA 0x00236780 (45 instructions, 10 basic blocks)
    pub fn method_11(&mut self) {
        // Writes to field at offset +0x001c
        // Writes to field at offset +0x001c
    }

    /// Decompiled virtual method #12 @ RVA 0x000d3090 (1 instructions, 1 basic blocks)
    pub fn method_12(&mut self) {
    }

    /// Decompiled virtual method #13 @ RVA 0x00236850 (46 instructions, 10 basic blocks)
    pub fn method_13(&mut self) {
        // Writes to field at offset +0x001c
        // Writes to field at offset +0x001c
    }

}

impl CcClass for ccComboDispCtrl {
    fn class_name(&self) -> &'static str { Self::CLASS_NAME }
    fn vtable_rva(&self) -> u32 { Self::VTABLE_RVA }
    fn virtual_method_count(&self) -> usize { Self::METHOD_COUNT }
    fn base_classes(&self) -> &'static [&'static str] { Self::BASE_CLASSES }
}

/// Decompiled CC2 class: `ccConsButtonDisp` (VTable RVA: 0x008819b8, 14 virtual methods, 14 recovered fields)
/// Inherits: ccDispW2L, ccDispBase
#[derive(Debug, Clone, Default)]
pub struct ccConsButtonDisp {
    pub vtable_rva: u32,
    pub field_0x0000: u64,
    pub field_0x0008: u32,
    pub field_0x001c: f32,
    pub field_0x0028: u32,
    pub field_0x002c: u32,
    pub field_0x0030: u32,
    pub field_0x0034: u32,
    pub field_0x0068: u64,
    pub field_0x0070: u64,
    pub field_0x0080: f32,
    pub field_0x0084: f32,
    pub field_0x0088: f32,
    pub field_0x0094: u8,
    pub field_0x0108: u64,
}

impl ccConsButtonDisp {
    pub const CLASS_NAME: &'static str = "ccConsButtonDisp";
    pub const VTABLE_RVA: u32 = 0x008819b8;
    pub const METHOD_COUNT: usize = 14;
    pub const BASE_CLASSES: &'static [&'static str] = &["ccDispW2L", "ccDispBase"];
    pub const VIRTUAL_METHODS: &'static [u64] = &[0x0000000180232c90, 0x000000018023aff0, 0x000000018023b230, 0x000000018023b2c0, 0x00000001802a0e30, 0x00000001800d3090, 0x00000001801b81f0, 0x00000001801b8200, 0x00000001801b8210, 0x00000001800d3090, 0x00000001800d3090, 0x0000000180232bb0, 0x000000018023b590, 0x0000000180232c20];

    pub fn new() -> Self {
        Self {
            vtable_rva: Self::VTABLE_RVA,
            field_0x0000: Default::default(),
            field_0x0008: Default::default(),
            field_0x001c: Default::default(),
            field_0x0028: Default::default(),
            field_0x002c: Default::default(),
            field_0x0030: Default::default(),
            field_0x0034: Default::default(),
            field_0x0068: Default::default(),
            field_0x0070: Default::default(),
            field_0x0080: Default::default(),
            field_0x0084: Default::default(),
            field_0x0088: Default::default(),
            field_0x0094: Default::default(),
            field_0x0108: Default::default(),
        }
    }

    /// Decompiled virtual method #0 @ RVA 0x00232c90 (13 instructions, 3 basic blocks)
    pub fn method_00(&mut self) {
        // Writes to field at offset +0x0000
    }

    /// Decompiled virtual method #1 @ RVA 0x0023aff0 (113 instructions, 22 basic blocks)
    pub fn method_01(&mut self) {
        // Writes to field at offset +0x0070
    }

    /// Decompiled virtual method #2 @ RVA 0x0023b230 (35 instructions, 9 basic blocks)
    pub fn method_02(&mut self) {
    }

    /// Decompiled virtual method #3 @ RVA 0x0023b2c0 (91 instructions, 7 basic blocks)
    pub fn method_03(&mut self) {
        // Writes to field at offset +0x0008
    }

    /// Decompiled virtual method #4 @ RVA 0x002a0e30 (48 instructions, 11 basic blocks)
    pub fn method_04(&mut self) {
        // Writes to field at offset +0x0008
    }

    /// Decompiled virtual method #5 @ RVA 0x000d3090 (1 instructions, 1 basic blocks)
    pub fn method_05(&mut self) {
    }

    /// Decompiled virtual method #6 @ RVA 0x001b81f0 (2 instructions, 1 basic blocks)
    pub fn method_06(&mut self) {
        // Writes to field at offset +0x0008
    }

    /// Decompiled virtual method #7 @ RVA 0x001b8200 (5 instructions, 1 basic blocks)
    pub fn method_07(&mut self) {
    }

    /// Decompiled virtual method #8 @ RVA 0x001b8210 (5 instructions, 1 basic blocks)
    pub fn method_08(&mut self) {
    }

    /// Decompiled virtual method #9 @ RVA 0x000d3090 (1 instructions, 1 basic blocks)
    pub fn method_09(&mut self) {
    }

    /// Decompiled virtual method #10 @ RVA 0x000d3090 (1 instructions, 1 basic blocks)
    pub fn method_10(&mut self) {
    }

    /// Decompiled virtual method #11 @ RVA 0x00232bb0 (26 instructions, 6 basic blocks)
    pub fn method_11(&mut self) {
        // Writes to field at offset +0x001c
        // Writes to field at offset +0x001c
    }

    /// Decompiled virtual method #12 @ RVA 0x0023b590 (43 instructions, 7 basic blocks)
    pub fn method_12(&mut self) {
        // Writes to field at offset +0x0008
        // Writes to field at offset +0x0080
        // Writes to field at offset +0x0080
        // Writes to field at offset +0x0084
        // Writes to field at offset +0x0084
        // Writes to field at offset +0x0088
        // Writes to field at offset +0x0088
        // Writes to field at offset +0x0094
    }

    /// Decompiled virtual method #13 @ RVA 0x00232c20 (26 instructions, 6 basic blocks)
    pub fn method_13(&mut self) {
        // Writes to field at offset +0x001c
        // Writes to field at offset +0x001c
    }

}

impl CcClass for ccConsButtonDisp {
    fn class_name(&self) -> &'static str { Self::CLASS_NAME }
    fn vtable_rva(&self) -> u32 { Self::VTABLE_RVA }
    fn virtual_method_count(&self) -> usize { Self::METHOD_COUNT }
    fn base_classes(&self) -> &'static [&'static str] { Self::BASE_CLASSES }
}

/// Decompiled CC2 class: `ccCustomizeWnd` (VTable RVA: 0x008a4d38, 37 virtual methods, 12 recovered fields)
/// Inherits: ccGameMenuCtrlBase, ccGameMenuObj
#[derive(Debug, Clone, Default)]
pub struct ccCustomizeWnd {
    pub vtable_rva: u32,
    pub field_0x0000: u64,
    pub field_0x0004: u32,
    pub field_0x0008: u32,
    pub field_0x000c: u8,
    pub field_0x0010: u64,
    pub field_0x0018: u32,
    pub field_0x0048: u64,
    pub field_0x0050: u64,
    pub field_0x0070: u32,
    pub field_0x0088: u32,
    pub field_0x009c: u32,
    pub field_0x0288: u64,
}

impl ccCustomizeWnd {
    pub const CLASS_NAME: &'static str = "ccCustomizeWnd";
    pub const VTABLE_RVA: u32 = 0x008a4d38;
    pub const METHOD_COUNT: usize = 37;
    pub const BASE_CLASSES: &'static [&'static str] = &["ccGameMenuCtrlBase", "ccGameMenuObj"];
    pub const VIRTUAL_METHODS: &'static [u64] = &[0x00000001802ad080, 0x00000001804520b0, 0x0000000180452130, 0x0000000180451fb0, 0x0000000180451ff0, 0x0000000180452030, 0x0000000180452070, 0x0000000180103e90, 0x0000000180103ea0, 0x0000000180103e60, 0x000000018045bb30, 0x0000000180451d90, 0x00000001801e9580, 0x0000000180103e60, 0x0000000180103e60, 0x0000000180451dc0, 0x0000000180451e30, 0x0000000180451ea0, 0x0000000180451ee0, 0x0000000180451f40, 0x00000001802ad320, 0x00000001802ad8c0, 0x00000001802adb00, 0x0000000180103e60, 0x0000000180103e60, 0x0000000180103e70, 0x00000001802addf0, 0x0000000180103e60, 0x0000000180103e60, 0x0000000180103e60, 0x00000001802adb60, 0x00000001802add00, 0x00000001800d3090, 0x0000000180103e70, 0x0000000180103e70, 0x0000000180103e80, 0x0000000180103e60];

    pub fn new() -> Self {
        Self {
            vtable_rva: Self::VTABLE_RVA,
            field_0x0000: Default::default(),
            field_0x0004: Default::default(),
            field_0x0008: Default::default(),
            field_0x000c: Default::default(),
            field_0x0010: Default::default(),
            field_0x0018: Default::default(),
            field_0x0048: Default::default(),
            field_0x0050: Default::default(),
            field_0x0070: Default::default(),
            field_0x0088: Default::default(),
            field_0x009c: Default::default(),
            field_0x0288: Default::default(),
        }
    }

    /// Decompiled virtual method #0 @ RVA 0x002ad080 (16 instructions, 3 basic blocks)
    pub fn method_00(&mut self) {
    }

    /// Decompiled virtual method #1 @ RVA 0x004520b0 (10 instructions, 2 basic blocks)
    pub fn method_01(&mut self) {
    }

    /// Decompiled virtual method #2 @ RVA 0x00452130 (35 instructions, 9 basic blocks)
    pub fn method_02(&mut self) {
    }

    /// Decompiled virtual method #3 @ RVA 0x00451fb0 (118 instructions, 2 basic blocks)
    pub fn method_03(&mut self) {
    }

    /// Decompiled virtual method #4 @ RVA 0x00451ff0 (91 instructions, 2 basic blocks)
    pub fn method_04(&mut self) {
    }

    /// Decompiled virtual method #5 @ RVA 0x00452030 (64 instructions, 2 basic blocks)
    pub fn method_05(&mut self) {
    }

    /// Decompiled virtual method #6 @ RVA 0x00452070 (37 instructions, 2 basic blocks)
    pub fn method_06(&mut self) {
    }

    /// Decompiled virtual method #7 @ RVA 0x00103e90 (2 instructions, 1 basic blocks)
    pub fn method_07(&mut self) {
    }

    /// Decompiled virtual method #8 @ RVA 0x00103ea0 (9 instructions, 1 basic blocks)
    pub fn method_08(&mut self) {
        // Writes to field at offset +0x0008
    }

    /// Decompiled virtual method #9 @ RVA 0x00103e60 (2 instructions, 1 basic blocks)
    pub fn method_09(&mut self) {
    }

    /// Decompiled virtual method #10 @ RVA 0x0045bb30 (16 instructions, 2 basic blocks)
    pub fn method_10(&mut self) {
    }

    /// Decompiled virtual method #11 @ RVA 0x00451d90 (24 instructions, 2 basic blocks)
    pub fn method_11(&mut self) {
    }

    /// Decompiled virtual method #12 @ RVA 0x001e9580 (53 instructions, 12 basic blocks)
    pub fn method_12(&mut self) {
    }

    /// Decompiled virtual method #13 @ RVA 0x00103e60 (2 instructions, 1 basic blocks)
    pub fn method_13(&mut self) {
    }

    /// Decompiled virtual method #14 @ RVA 0x00103e60 (2 instructions, 1 basic blocks)
    pub fn method_14(&mut self) {
    }

    /// Decompiled virtual method #15 @ RVA 0x00451dc0 (10 instructions, 2 basic blocks)
    pub fn method_15(&mut self) {
    }

    /// Decompiled virtual method #16 @ RVA 0x00451e30 (10 instructions, 2 basic blocks)
    pub fn method_16(&mut self) {
    }

    /// Decompiled virtual method #17 @ RVA 0x00451ea0 (31 instructions, 2 basic blocks)
    pub fn method_17(&mut self) {
        // Writes to field at offset +0x0018
    }

    /// Decompiled virtual method #18 @ RVA 0x00451ee0 (10 instructions, 2 basic blocks)
    pub fn method_18(&mut self) {
    }

    /// Decompiled virtual method #19 @ RVA 0x00451f40 (10 instructions, 2 basic blocks)
    pub fn method_19(&mut self) {
    }

    /// Decompiled virtual method #20 @ RVA 0x002ad320 (116 instructions, 15 basic blocks)
    pub fn method_20(&mut self) {
        // Writes to field at offset +0x0000
        // Writes to field at offset +0x0004
        // Writes to field at offset +0x0008
        // Writes to field at offset +0x000c
        // Writes to field at offset +0x0070
        // Writes to field at offset +0x0088
    }

    /// Decompiled virtual method #21 @ RVA 0x002ad8c0 (122 instructions, 21 basic blocks)
    pub fn method_21(&mut self) {
        // Writes to field at offset +0x0000
        // Writes to field at offset +0x0000
        // Writes to field at offset +0x0008
        // Writes to field at offset +0x0010
        // Writes to field at offset +0x0018
    }

    /// Decompiled virtual method #22 @ RVA 0x002adb00 (21 instructions, 1 basic blocks)
    pub fn method_22(&mut self) {
    }

    /// Decompiled virtual method #23 @ RVA 0x00103e60 (2 instructions, 1 basic blocks)
    pub fn method_23(&mut self) {
    }

    /// Decompiled virtual method #24 @ RVA 0x00103e60 (2 instructions, 1 basic blocks)
    pub fn method_24(&mut self) {
    }

    /// Decompiled virtual method #25 @ RVA 0x00103e70 (2 instructions, 1 basic blocks)
    pub fn method_25(&mut self) {
    }

    /// Decompiled virtual method #26 @ RVA 0x002addf0 (5 instructions, 1 basic blocks)
    pub fn method_26(&mut self) {
    }

    /// Decompiled virtual method #27 @ RVA 0x00103e60 (2 instructions, 1 basic blocks)
    pub fn method_27(&mut self) {
    }

    /// Decompiled virtual method #28 @ RVA 0x00103e60 (2 instructions, 1 basic blocks)
    pub fn method_28(&mut self) {
    }

    /// Decompiled virtual method #29 @ RVA 0x00103e60 (2 instructions, 1 basic blocks)
    pub fn method_29(&mut self) {
    }

    /// Decompiled virtual method #30 @ RVA 0x002adb60 (96 instructions, 16 basic blocks)
    pub fn method_30(&mut self) {
    }

    /// Decompiled virtual method #31 @ RVA 0x002add00 (58 instructions, 5 basic blocks)
    pub fn method_31(&mut self) {
    }

    /// Decompiled virtual method #32 @ RVA 0x000d3090 (1 instructions, 1 basic blocks)
    pub fn method_32(&mut self) {
    }

    /// Decompiled virtual method #33 @ RVA 0x00103e70 (2 instructions, 1 basic blocks)
    pub fn method_33(&mut self) {
    }

    /// Decompiled virtual method #34 @ RVA 0x00103e70 (2 instructions, 1 basic blocks)
    pub fn method_34(&mut self) {
    }

    /// Decompiled virtual method #35 @ RVA 0x00103e80 (2 instructions, 1 basic blocks)
    pub fn method_35(&mut self) {
    }

    /// Decompiled virtual method #36 @ RVA 0x00103e60 (2 instructions, 1 basic blocks)
    pub fn method_36(&mut self) {
    }

}

impl CcClass for ccCustomizeWnd {
    fn class_name(&self) -> &'static str { Self::CLASS_NAME }
    fn vtable_rva(&self) -> u32 { Self::VTABLE_RVA }
    fn virtual_method_count(&self) -> usize { Self::METHOD_COUNT }
    fn base_classes(&self) -> &'static [&'static str] { Self::BASE_CLASSES }
}

/// Decompiled CC2 class: `ccDispAnm` (VTable RVA: 0x0087c958, 5 virtual methods, 1 recovered fields)
#[derive(Debug, Clone, Default)]
pub struct ccDispAnm {
    pub vtable_rva: u32,
    pub field_0x0000: u64,
}

impl ccDispAnm {
    pub const CLASS_NAME: &'static str = "ccDispAnm";
    pub const VTABLE_RVA: u32 = 0x0087c958;
    pub const METHOD_COUNT: usize = 5;
    pub const BASE_CLASSES: &'static [&'static str] = &[];
    pub const VIRTUAL_METHODS: &'static [u64] = &[0x00000001801c9600, 0x00000001800d3090, 0x00000001800d3090, 0x00000001800d3090, 0x00000001800d3090];

    pub fn new() -> Self {
        Self {
            vtable_rva: Self::VTABLE_RVA,
            field_0x0000: Default::default(),
        }
    }

    /// Decompiled virtual method #0 @ RVA 0x001c9600 (21 instructions, 3 basic blocks)
    pub fn method_00(&mut self) {
        // Writes to field at offset +0x0000
    }

    /// Decompiled virtual method #1 @ RVA 0x000d3090 (1 instructions, 1 basic blocks)
    pub fn method_01(&mut self) {
    }

    /// Decompiled virtual method #2 @ RVA 0x000d3090 (1 instructions, 1 basic blocks)
    pub fn method_02(&mut self) {
    }

    /// Decompiled virtual method #3 @ RVA 0x000d3090 (1 instructions, 1 basic blocks)
    pub fn method_03(&mut self) {
    }

    /// Decompiled virtual method #4 @ RVA 0x000d3090 (1 instructions, 1 basic blocks)
    pub fn method_04(&mut self) {
    }

}

impl CcClass for ccDispAnm {
    fn class_name(&self) -> &'static str { Self::CLASS_NAME }
    fn vtable_rva(&self) -> u32 { Self::VTABLE_RVA }
    fn virtual_method_count(&self) -> usize { Self::METHOD_COUNT }
    fn base_classes(&self) -> &'static [&'static str] { Self::BASE_CLASSES }
}

/// Decompiled CC2 class: `ccDispBase` (VTable RVA: 0x00881c68, 14 virtual methods, 6 recovered fields)
#[derive(Debug, Clone, Default)]
pub struct ccDispBase {
    pub vtable_rva: u32,
    pub field_0x0000: u64,
    pub field_0x0008: u32,
    pub field_0x0028: u32,
    pub field_0x002c: u32,
    pub field_0x0030: u32,
    pub field_0x0034: u32,
}

impl ccDispBase {
    pub const CLASS_NAME: &'static str = "ccDispBase";
    pub const VTABLE_RVA: u32 = 0x00881c68;
    pub const METHOD_COUNT: usize = 14;
    pub const BASE_CLASSES: &'static [&'static str] = &[];
    pub const VIRTUAL_METHODS: &'static [u64] = &[0x00000001802324b0, 0x00000001800d3090, 0x00000001800d3090, 0x00000001800d3090, 0x00000001802a0e30, 0x00000001800d3090, 0x00000001801b81f0, 0x00000001801b8200, 0x00000001801b8210, 0x00000001800d3090, 0x00000001800d3090, 0x00000001800d3090, 0x00000001800d3090, 0x00000001800d3090];

    pub fn new() -> Self {
        Self {
            vtable_rva: Self::VTABLE_RVA,
            field_0x0000: Default::default(),
            field_0x0008: Default::default(),
            field_0x0028: Default::default(),
            field_0x002c: Default::default(),
            field_0x0030: Default::default(),
            field_0x0034: Default::default(),
        }
    }

    /// Decompiled virtual method #0 @ RVA 0x002324b0 (13 instructions, 3 basic blocks)
    pub fn method_00(&mut self) {
        // Writes to field at offset +0x0000
    }

    /// Decompiled virtual method #1 @ RVA 0x000d3090 (1 instructions, 1 basic blocks)
    pub fn method_01(&mut self) {
    }

    /// Decompiled virtual method #2 @ RVA 0x000d3090 (1 instructions, 1 basic blocks)
    pub fn method_02(&mut self) {
    }

    /// Decompiled virtual method #3 @ RVA 0x000d3090 (1 instructions, 1 basic blocks)
    pub fn method_03(&mut self) {
    }

    /// Decompiled virtual method #4 @ RVA 0x002a0e30 (48 instructions, 11 basic blocks)
    pub fn method_04(&mut self) {
        // Writes to field at offset +0x0008
    }

    /// Decompiled virtual method #5 @ RVA 0x000d3090 (1 instructions, 1 basic blocks)
    pub fn method_05(&mut self) {
    }

    /// Decompiled virtual method #6 @ RVA 0x001b81f0 (2 instructions, 1 basic blocks)
    pub fn method_06(&mut self) {
        // Writes to field at offset +0x0008
    }

    /// Decompiled virtual method #7 @ RVA 0x001b8200 (5 instructions, 1 basic blocks)
    pub fn method_07(&mut self) {
    }

    /// Decompiled virtual method #8 @ RVA 0x001b8210 (5 instructions, 1 basic blocks)
    pub fn method_08(&mut self) {
    }

    /// Decompiled virtual method #9 @ RVA 0x000d3090 (1 instructions, 1 basic blocks)
    pub fn method_09(&mut self) {
    }

    /// Decompiled virtual method #10 @ RVA 0x000d3090 (1 instructions, 1 basic blocks)
    pub fn method_10(&mut self) {
    }

    /// Decompiled virtual method #11 @ RVA 0x000d3090 (1 instructions, 1 basic blocks)
    pub fn method_11(&mut self) {
    }

    /// Decompiled virtual method #12 @ RVA 0x000d3090 (1 instructions, 1 basic blocks)
    pub fn method_12(&mut self) {
    }

    /// Decompiled virtual method #13 @ RVA 0x000d3090 (1 instructions, 1 basic blocks)
    pub fn method_13(&mut self) {
    }

}

impl CcClass for ccDispBase {
    fn class_name(&self) -> &'static str { Self::CLASS_NAME }
    fn vtable_rva(&self) -> u32 { Self::VTABLE_RVA }
    fn virtual_method_count(&self) -> usize { Self::METHOD_COUNT }
    fn base_classes(&self) -> &'static [&'static str] { Self::BASE_CLASSES }
}

/// Decompiled CC2 class: `ccDispSpline` (VTable RVA: 0x008a44f0, 14 virtual methods, 9 recovered fields)
/// Inherits: ccDispBase
#[derive(Debug, Clone, Default)]
pub struct ccDispSpline {
    pub vtable_rva: u32,
    pub field_0x0000: u64,
    pub field_0x0008: u32,
    pub field_0x0028: u32,
    pub field_0x002c: u32,
    pub field_0x0030: u32,
    pub field_0x0034: u32,
    pub field_0x0078: u16,
    pub field_0x007a: u16,
    pub field_0x007c: u8,
}

impl ccDispSpline {
    pub const CLASS_NAME: &'static str = "ccDispSpline";
    pub const VTABLE_RVA: u32 = 0x008a44f0;
    pub const METHOD_COUNT: usize = 14;
    pub const BASE_CLASSES: &'static [&'static str] = &["ccDispBase"];
    pub const VIRTUAL_METHODS: &'static [u64] = &[0x00000001802a1c10, 0x00000001800d3090, 0x00000001800d3090, 0x00000001800d3090, 0x00000001802a0e30, 0x00000001800d3090, 0x00000001801b81f0, 0x00000001801b8200, 0x00000001801b8210, 0x00000001802a1cf0, 0x00000001800d3090, 0x00000001800d3090, 0x00000001800d3090, 0x00000001800d3090];

    pub fn new() -> Self {
        Self {
            vtable_rva: Self::VTABLE_RVA,
            field_0x0000: Default::default(),
            field_0x0008: Default::default(),
            field_0x0028: Default::default(),
            field_0x002c: Default::default(),
            field_0x0030: Default::default(),
            field_0x0034: Default::default(),
            field_0x0078: Default::default(),
            field_0x007a: Default::default(),
            field_0x007c: Default::default(),
        }
    }

    /// Decompiled virtual method #0 @ RVA 0x002a1c10 (16 instructions, 3 basic blocks)
    pub fn method_00(&mut self) {
    }

    /// Decompiled virtual method #1 @ RVA 0x000d3090 (1 instructions, 1 basic blocks)
    pub fn method_01(&mut self) {
    }

    /// Decompiled virtual method #2 @ RVA 0x000d3090 (1 instructions, 1 basic blocks)
    pub fn method_02(&mut self) {
    }

    /// Decompiled virtual method #3 @ RVA 0x000d3090 (1 instructions, 1 basic blocks)
    pub fn method_03(&mut self) {
    }

    /// Decompiled virtual method #4 @ RVA 0x002a0e30 (48 instructions, 11 basic blocks)
    pub fn method_04(&mut self) {
        // Writes to field at offset +0x0008
    }

    /// Decompiled virtual method #5 @ RVA 0x000d3090 (1 instructions, 1 basic blocks)
    pub fn method_05(&mut self) {
    }

    /// Decompiled virtual method #6 @ RVA 0x001b81f0 (2 instructions, 1 basic blocks)
    pub fn method_06(&mut self) {
        // Writes to field at offset +0x0008
    }

    /// Decompiled virtual method #7 @ RVA 0x001b8200 (5 instructions, 1 basic blocks)
    pub fn method_07(&mut self) {
    }

    /// Decompiled virtual method #8 @ RVA 0x001b8210 (5 instructions, 1 basic blocks)
    pub fn method_08(&mut self) {
    }

    /// Decompiled virtual method #9 @ RVA 0x002a1cf0 (39 instructions, 5 basic blocks)
    pub fn method_09(&mut self) {
        // Writes to field at offset +0x007a
        // Writes to field at offset +0x007c
    }

    /// Decompiled virtual method #10 @ RVA 0x000d3090 (1 instructions, 1 basic blocks)
    pub fn method_10(&mut self) {
    }

    /// Decompiled virtual method #11 @ RVA 0x000d3090 (1 instructions, 1 basic blocks)
    pub fn method_11(&mut self) {
    }

    /// Decompiled virtual method #12 @ RVA 0x000d3090 (1 instructions, 1 basic blocks)
    pub fn method_12(&mut self) {
    }

    /// Decompiled virtual method #13 @ RVA 0x000d3090 (1 instructions, 1 basic blocks)
    pub fn method_13(&mut self) {
    }

}

impl CcClass for ccDispSpline {
    fn class_name(&self) -> &'static str { Self::CLASS_NAME }
    fn vtable_rva(&self) -> u32 { Self::VTABLE_RVA }
    fn virtual_method_count(&self) -> usize { Self::METHOD_COUNT }
    fn base_classes(&self) -> &'static [&'static str] { Self::BASE_CLASSES }
}

/// Decompiled CC2 class: `ccDispStateMenu` (VTable RVA: 0x008b6c58, 4 virtual methods, 6 recovered fields)
/// Inherits: ccEventBase
#[derive(Debug, Clone, Default)]
pub struct ccDispStateMenu {
    pub vtable_rva: u32,
    pub field_0x0000: u64,
    pub field_0x0008: u16,
    pub field_0x000a: u8,
    pub field_0x0010: u64,
    pub field_0x0020: u32,
    pub field_0x0024: u32,
}

impl ccDispStateMenu {
    pub const CLASS_NAME: &'static str = "ccDispStateMenu";
    pub const VTABLE_RVA: u32 = 0x008b6c58;
    pub const METHOD_COUNT: usize = 4;
    pub const BASE_CLASSES: &'static [&'static str] = &["ccEventBase"];
    pub const VIRTUAL_METHODS: &'static [u64] = &[0x0000000180385900, 0x0000000180380cf0, 0x00000001800d3090, 0x000000018038b7b0];

    pub fn new() -> Self {
        Self {
            vtable_rva: Self::VTABLE_RVA,
            field_0x0000: Default::default(),
            field_0x0008: Default::default(),
            field_0x000a: Default::default(),
            field_0x0010: Default::default(),
            field_0x0020: Default::default(),
            field_0x0024: Default::default(),
        }
    }

    /// Decompiled virtual method #0 @ RVA 0x00385900 (18 instructions, 3 basic blocks)
    pub fn method_00(&mut self) {
        // Writes to field at offset +0x0000
    }

    /// Decompiled virtual method #1 @ RVA 0x00380cf0 (13 instructions, 2 basic blocks)
    pub fn method_01(&mut self) {
        // Writes to field at offset +0x0008
        // Writes to field at offset +0x000a
    }

    /// Decompiled virtual method #2 @ RVA 0x000d3090 (1 instructions, 1 basic blocks)
    pub fn method_02(&mut self) {
    }

    /// Decompiled virtual method #3 @ RVA 0x0038b7b0 (28 instructions, 10 basic blocks)
    pub fn method_03(&mut self) {
        // Writes to field at offset +0x000a
    }

}

impl CcClass for ccDispStateMenu {
    fn class_name(&self) -> &'static str { Self::CLASS_NAME }
    fn vtable_rva(&self) -> u32 { Self::VTABLE_RVA }
    fn virtual_method_count(&self) -> usize { Self::METHOD_COUNT }
    fn base_classes(&self) -> &'static [&'static str] { Self::BASE_CLASSES }
}

/// Decompiled CC2 class: `ccEnemyInstDisp` (VTable RVA: 0x00881188, 14 virtual methods, 17 recovered fields)
/// Inherits: ccDispBase
#[derive(Debug, Clone, Default)]
pub struct ccEnemyInstDisp {
    pub vtable_rva: u32,
    pub field_0x0000: u64,
    pub field_0x0008: u32,
    pub field_0x0018: u64,
    pub field_0x001c: f32,
    pub field_0x0024: u16,
    pub field_0x0028: u32,
    pub field_0x002c: u32,
    pub field_0x0030: u32,
    pub field_0x0034: u32,
    pub field_0x0044: u32,
    pub field_0x0048: u64,
    pub field_0x0050: f32,
    pub field_0x0054: f32,
    pub field_0x0058: f32,
    pub field_0x0070: f32,
    pub field_0x0074: f32,
    pub field_0x0087: u8,
}

impl ccEnemyInstDisp {
    pub const CLASS_NAME: &'static str = "ccEnemyInstDisp";
    pub const VTABLE_RVA: u32 = 0x00881188;
    pub const METHOD_COUNT: usize = 14;
    pub const BASE_CLASSES: &'static [&'static str] = &["ccDispBase"];
    pub const VIRTUAL_METHODS: &'static [u64] = &[0x0000000180232930, 0x00000001800d3090, 0x00000001800d3090, 0x0000000180240ae0, 0x00000001802a0e30, 0x00000001800d3090, 0x00000001801b81f0, 0x00000001801b8200, 0x00000001801b8210, 0x00000001800d3090, 0x00000001802411b0, 0x0000000180240fc0, 0x0000000180241030, 0x00000001802410e0];

    pub fn new() -> Self {
        Self {
            vtable_rva: Self::VTABLE_RVA,
            field_0x0000: Default::default(),
            field_0x0008: Default::default(),
            field_0x0018: Default::default(),
            field_0x001c: Default::default(),
            field_0x0024: Default::default(),
            field_0x0028: Default::default(),
            field_0x002c: Default::default(),
            field_0x0030: Default::default(),
            field_0x0034: Default::default(),
            field_0x0044: Default::default(),
            field_0x0048: Default::default(),
            field_0x0050: Default::default(),
            field_0x0054: Default::default(),
            field_0x0058: Default::default(),
            field_0x0070: Default::default(),
            field_0x0074: Default::default(),
            field_0x0087: Default::default(),
        }
    }

    /// Decompiled virtual method #0 @ RVA 0x00232930 (13 instructions, 3 basic blocks)
    pub fn method_00(&mut self) {
        // Writes to field at offset +0x0000
    }

    /// Decompiled virtual method #1 @ RVA 0x000d3090 (1 instructions, 1 basic blocks)
    pub fn method_01(&mut self) {
    }

    /// Decompiled virtual method #2 @ RVA 0x000d3090 (1 instructions, 1 basic blocks)
    pub fn method_02(&mut self) {
    }

    /// Decompiled virtual method #3 @ RVA 0x00240ae0 (101 instructions, 9 basic blocks)
    pub fn method_03(&mut self) {
        // Writes to field at offset +0x0008
        // Writes to field at offset +0x0058
    }

    /// Decompiled virtual method #4 @ RVA 0x002a0e30 (48 instructions, 11 basic blocks)
    pub fn method_04(&mut self) {
        // Writes to field at offset +0x0008
    }

    /// Decompiled virtual method #5 @ RVA 0x000d3090 (1 instructions, 1 basic blocks)
    pub fn method_05(&mut self) {
    }

    /// Decompiled virtual method #6 @ RVA 0x001b81f0 (2 instructions, 1 basic blocks)
    pub fn method_06(&mut self) {
        // Writes to field at offset +0x0008
    }

    /// Decompiled virtual method #7 @ RVA 0x001b8200 (5 instructions, 1 basic blocks)
    pub fn method_07(&mut self) {
    }

    /// Decompiled virtual method #8 @ RVA 0x001b8210 (5 instructions, 1 basic blocks)
    pub fn method_08(&mut self) {
    }

    /// Decompiled virtual method #9 @ RVA 0x000d3090 (1 instructions, 1 basic blocks)
    pub fn method_09(&mut self) {
    }

    /// Decompiled virtual method #10 @ RVA 0x002411b0 (18 instructions, 3 basic blocks)
    pub fn method_10(&mut self) {
        // Writes to field at offset +0x0008
        // Writes to field at offset +0x0087
    }

    /// Decompiled virtual method #11 @ RVA 0x00240fc0 (136 instructions, 35 basic blocks)
    pub fn method_11(&mut self) {
        // Writes to field at offset +0x001c
        // Writes to field at offset +0x001c
        // Writes to field at offset +0x0074
    }

    /// Decompiled virtual method #12 @ RVA 0x00241030 (106 instructions, 29 basic blocks)
    pub fn method_12(&mut self) {
        // Writes to field at offset +0x001c
        // Writes to field at offset +0x001c
        // Writes to field at offset +0x0074
    }

    /// Decompiled virtual method #13 @ RVA 0x002410e0 (49 instructions, 13 basic blocks)
    pub fn method_13(&mut self) {
        // Writes to field at offset +0x001c
        // Writes to field at offset +0x001c
        // Writes to field at offset +0x0074
    }

}

impl CcClass for ccEnemyInstDisp {
    fn class_name(&self) -> &'static str { Self::CLASS_NAME }
    fn vtable_rva(&self) -> u32 { Self::VTABLE_RVA }
    fn virtual_method_count(&self) -> usize { Self::METHOD_COUNT }
    fn base_classes(&self) -> &'static [&'static str] { Self::BASE_CLASSES }
}

/// Decompiled CC2 class: `ccEquipTrainWnd` (VTable RVA: 0x008b3b40, 37 virtual methods, 20 recovered fields)
/// Inherits: ccGameMenuCtrlBase, ccGameMenuObj
#[derive(Debug, Clone, Default)]
pub struct ccEquipTrainWnd {
    pub vtable_rva: u32,
    pub field_0x0000: u64,
    pub field_0x0004: u32,
    pub field_0x0008: u32,
    pub field_0x000c: u8,
    pub field_0x0018: u32,
    pub field_0x0038: u32,
    pub field_0x0048: u64,
    pub field_0x0050: u64,
    pub field_0x0070: u64,
    pub field_0x0080: u64,
    pub field_0x009c: u32,
    pub field_0x00a8: u32,
    pub field_0x00ac: u64,
    pub field_0x00b4: u64,
    pub field_0x00c0: u32,
    pub field_0x00c4: u64,
    pub field_0x00d4: u32,
    pub field_0x00d8: u64,
    pub field_0x0112: u8,
    pub field_0x0114: u32,
}

impl ccEquipTrainWnd {
    pub const CLASS_NAME: &'static str = "ccEquipTrainWnd";
    pub const VTABLE_RVA: u32 = 0x008b3b40;
    pub const METHOD_COUNT: usize = 37;
    pub const BASE_CLASSES: &'static [&'static str] = &["ccGameMenuCtrlBase", "ccGameMenuObj"];
    pub const VIRTUAL_METHODS: &'static [u64] = &[0x0000000180375180, 0x00000001804520b0, 0x0000000180452130, 0x0000000180451fb0, 0x0000000180451ff0, 0x0000000180452030, 0x0000000180452070, 0x0000000180103e90, 0x0000000180103ea0, 0x0000000180103e60, 0x000000018045bb30, 0x0000000180451d90, 0x00000001801e9580, 0x0000000180103e60, 0x0000000180103e60, 0x0000000180451dc0, 0x0000000180451e30, 0x0000000180451ea0, 0x0000000180451ee0, 0x0000000180451f40, 0x0000000180375420, 0x00000001803757f0, 0x0000000180375a90, 0x0000000180103e60, 0x0000000180103e60, 0x0000000180103e70, 0x0000000180375da0, 0x0000000180103e60, 0x0000000180103e60, 0x0000000180103e60, 0x0000000180375ae0, 0x0000000180375c90, 0x00000001800d3090, 0x0000000180103e70, 0x0000000180103e70, 0x0000000180103e80, 0x0000000180103e60];

    pub fn new() -> Self {
        Self {
            vtable_rva: Self::VTABLE_RVA,
            field_0x0000: Default::default(),
            field_0x0004: Default::default(),
            field_0x0008: Default::default(),
            field_0x000c: Default::default(),
            field_0x0018: Default::default(),
            field_0x0038: Default::default(),
            field_0x0048: Default::default(),
            field_0x0050: Default::default(),
            field_0x0070: Default::default(),
            field_0x0080: Default::default(),
            field_0x009c: Default::default(),
            field_0x00a8: Default::default(),
            field_0x00ac: Default::default(),
            field_0x00b4: Default::default(),
            field_0x00c0: Default::default(),
            field_0x00c4: Default::default(),
            field_0x00d4: Default::default(),
            field_0x00d8: Default::default(),
            field_0x0112: Default::default(),
            field_0x0114: Default::default(),
        }
    }

    /// Decompiled virtual method #0 @ RVA 0x00375180 (16 instructions, 3 basic blocks)
    pub fn method_00(&mut self) {
    }

    /// Decompiled virtual method #1 @ RVA 0x004520b0 (10 instructions, 2 basic blocks)
    pub fn method_01(&mut self) {
    }

    /// Decompiled virtual method #2 @ RVA 0x00452130 (35 instructions, 9 basic blocks)
    pub fn method_02(&mut self) {
    }

    /// Decompiled virtual method #3 @ RVA 0x00451fb0 (118 instructions, 2 basic blocks)
    pub fn method_03(&mut self) {
    }

    /// Decompiled virtual method #4 @ RVA 0x00451ff0 (91 instructions, 2 basic blocks)
    pub fn method_04(&mut self) {
    }

    /// Decompiled virtual method #5 @ RVA 0x00452030 (64 instructions, 2 basic blocks)
    pub fn method_05(&mut self) {
    }

    /// Decompiled virtual method #6 @ RVA 0x00452070 (37 instructions, 2 basic blocks)
    pub fn method_06(&mut self) {
    }

    /// Decompiled virtual method #7 @ RVA 0x00103e90 (2 instructions, 1 basic blocks)
    pub fn method_07(&mut self) {
    }

    /// Decompiled virtual method #8 @ RVA 0x00103ea0 (9 instructions, 1 basic blocks)
    pub fn method_08(&mut self) {
        // Writes to field at offset +0x0008
    }

    /// Decompiled virtual method #9 @ RVA 0x00103e60 (2 instructions, 1 basic blocks)
    pub fn method_09(&mut self) {
    }

    /// Decompiled virtual method #10 @ RVA 0x0045bb30 (16 instructions, 2 basic blocks)
    pub fn method_10(&mut self) {
    }

    /// Decompiled virtual method #11 @ RVA 0x00451d90 (24 instructions, 2 basic blocks)
    pub fn method_11(&mut self) {
    }

    /// Decompiled virtual method #12 @ RVA 0x001e9580 (53 instructions, 12 basic blocks)
    pub fn method_12(&mut self) {
    }

    /// Decompiled virtual method #13 @ RVA 0x00103e60 (2 instructions, 1 basic blocks)
    pub fn method_13(&mut self) {
    }

    /// Decompiled virtual method #14 @ RVA 0x00103e60 (2 instructions, 1 basic blocks)
    pub fn method_14(&mut self) {
    }

    /// Decompiled virtual method #15 @ RVA 0x00451dc0 (10 instructions, 2 basic blocks)
    pub fn method_15(&mut self) {
    }

    /// Decompiled virtual method #16 @ RVA 0x00451e30 (10 instructions, 2 basic blocks)
    pub fn method_16(&mut self) {
    }

    /// Decompiled virtual method #17 @ RVA 0x00451ea0 (31 instructions, 2 basic blocks)
    pub fn method_17(&mut self) {
        // Writes to field at offset +0x0018
    }

    /// Decompiled virtual method #18 @ RVA 0x00451ee0 (10 instructions, 2 basic blocks)
    pub fn method_18(&mut self) {
    }

    /// Decompiled virtual method #19 @ RVA 0x00451f40 (10 instructions, 2 basic blocks)
    pub fn method_19(&mut self) {
    }

    /// Decompiled virtual method #20 @ RVA 0x00375420 (118 instructions, 16 basic blocks)
    pub fn method_20(&mut self) {
        // Writes to field at offset +0x0000
        // Writes to field at offset +0x0004
        // Writes to field at offset +0x0008
        // Writes to field at offset +0x000c
        // Writes to field at offset +0x00a8
        // Writes to field at offset +0x00ac
        // Writes to field at offset +0x00b4
        // Writes to field at offset +0x00c0
        // Writes to field at offset +0x00c4
        // Writes to field at offset +0x00d4
        // Writes to field at offset +0x00d8
        // Writes to field at offset +0x0112
        // Writes to field at offset +0x0114
    }

    /// Decompiled virtual method #21 @ RVA 0x003757f0 (121 instructions, 18 basic blocks)
    pub fn method_21(&mut self) {
        // Writes to field at offset +0x0038
        // Writes to field at offset +0x0048
    }

    /// Decompiled virtual method #22 @ RVA 0x00375a90 (22 instructions, 1 basic blocks)
    pub fn method_22(&mut self) {
    }

    /// Decompiled virtual method #23 @ RVA 0x00103e60 (2 instructions, 1 basic blocks)
    pub fn method_23(&mut self) {
    }

    /// Decompiled virtual method #24 @ RVA 0x00103e60 (2 instructions, 1 basic blocks)
    pub fn method_24(&mut self) {
    }

    /// Decompiled virtual method #25 @ RVA 0x00103e70 (2 instructions, 1 basic blocks)
    pub fn method_25(&mut self) {
    }

    /// Decompiled virtual method #26 @ RVA 0x00375da0 (5 instructions, 1 basic blocks)
    pub fn method_26(&mut self) {
    }

    /// Decompiled virtual method #27 @ RVA 0x00103e60 (2 instructions, 1 basic blocks)
    pub fn method_27(&mut self) {
    }

    /// Decompiled virtual method #28 @ RVA 0x00103e60 (2 instructions, 1 basic blocks)
    pub fn method_28(&mut self) {
    }

    /// Decompiled virtual method #29 @ RVA 0x00103e60 (2 instructions, 1 basic blocks)
    pub fn method_29(&mut self) {
    }

    /// Decompiled virtual method #30 @ RVA 0x00375ae0 (99 instructions, 16 basic blocks)
    pub fn method_30(&mut self) {
    }

    /// Decompiled virtual method #31 @ RVA 0x00375c90 (70 instructions, 6 basic blocks)
    pub fn method_31(&mut self) {
    }

    /// Decompiled virtual method #32 @ RVA 0x000d3090 (1 instructions, 1 basic blocks)
    pub fn method_32(&mut self) {
    }

    /// Decompiled virtual method #33 @ RVA 0x00103e70 (2 instructions, 1 basic blocks)
    pub fn method_33(&mut self) {
    }

    /// Decompiled virtual method #34 @ RVA 0x00103e70 (2 instructions, 1 basic blocks)
    pub fn method_34(&mut self) {
    }

    /// Decompiled virtual method #35 @ RVA 0x00103e80 (2 instructions, 1 basic blocks)
    pub fn method_35(&mut self) {
    }

    /// Decompiled virtual method #36 @ RVA 0x00103e60 (2 instructions, 1 basic blocks)
    pub fn method_36(&mut self) {
    }

}

impl CcClass for ccEquipTrainWnd {
    fn class_name(&self) -> &'static str { Self::CLASS_NAME }
    fn vtable_rva(&self) -> u32 { Self::VTABLE_RVA }
    fn virtual_method_count(&self) -> usize { Self::METHOD_COUNT }
    fn base_classes(&self) -> &'static [&'static str] { Self::BASE_CLASSES }
}

/// Decompiled CC2 class: `ccEquipWnd` (VTable RVA: 0x008baea8, 37 virtual methods, 16 recovered fields)
/// Inherits: ccGameMenuCtrlBase, ccGameMenuObj
#[derive(Debug, Clone, Default)]
pub struct ccEquipWnd {
    pub vtable_rva: u32,
    pub field_0x0000: u64,
    pub field_0x0008: u32,
    pub field_0x0010: u64,
    pub field_0x0014: u64,
    pub field_0x0018: u32,
    pub field_0x002c: u32,
    pub field_0x0030: u16,
    pub field_0x0048: u64,
    pub field_0x0050: u64,
    pub field_0x008c: u32,
    pub field_0x0098: u32,
    pub field_0x009c: u32,
    pub field_0x00a0: u32,
    pub field_0x0564: u32,
    pub field_0x0568: u32,
    pub field_0x056c: u32,
}

impl ccEquipWnd {
    pub const CLASS_NAME: &'static str = "ccEquipWnd";
    pub const VTABLE_RVA: u32 = 0x008baea8;
    pub const METHOD_COUNT: usize = 37;
    pub const BASE_CLASSES: &'static [&'static str] = &["ccGameMenuCtrlBase", "ccGameMenuObj"];
    pub const VIRTUAL_METHODS: &'static [u64] = &[0x0000000180450390, 0x00000001804520b0, 0x0000000180452130, 0x0000000180451fb0, 0x0000000180451ff0, 0x0000000180452030, 0x0000000180452070, 0x0000000180103e90, 0x0000000180103ea0, 0x0000000180103e60, 0x000000018045bb30, 0x0000000180451d90, 0x00000001801e9580, 0x0000000180103e60, 0x0000000180103e60, 0x0000000180451dc0, 0x0000000180451e30, 0x0000000180451ea0, 0x0000000180451ee0, 0x0000000180451f40, 0x000000018037bee0, 0x000000018037c1f0, 0x000000018037c470, 0x0000000180103e60, 0x0000000180103e60, 0x0000000180103e70, 0x000000018037c830, 0x0000000180103e60, 0x0000000180103e60, 0x0000000180103e60, 0x000000018037c520, 0x000000018037c790, 0x00000001800d3090, 0x0000000180103e70, 0x0000000180103e70, 0x0000000180103e80, 0x0000000180103e60];

    pub fn new() -> Self {
        Self {
            vtable_rva: Self::VTABLE_RVA,
            field_0x0000: Default::default(),
            field_0x0008: Default::default(),
            field_0x0010: Default::default(),
            field_0x0014: Default::default(),
            field_0x0018: Default::default(),
            field_0x002c: Default::default(),
            field_0x0030: Default::default(),
            field_0x0048: Default::default(),
            field_0x0050: Default::default(),
            field_0x008c: Default::default(),
            field_0x0098: Default::default(),
            field_0x009c: Default::default(),
            field_0x00a0: Default::default(),
            field_0x0564: Default::default(),
            field_0x0568: Default::default(),
            field_0x056c: Default::default(),
        }
    }

    /// Decompiled virtual method #0 @ RVA 0x00450390 (16 instructions, 3 basic blocks)
    pub fn method_00(&mut self) {
    }

    /// Decompiled virtual method #1 @ RVA 0x004520b0 (10 instructions, 2 basic blocks)
    pub fn method_01(&mut self) {
    }

    /// Decompiled virtual method #2 @ RVA 0x00452130 (35 instructions, 9 basic blocks)
    pub fn method_02(&mut self) {
    }

    /// Decompiled virtual method #3 @ RVA 0x00451fb0 (118 instructions, 2 basic blocks)
    pub fn method_03(&mut self) {
    }

    /// Decompiled virtual method #4 @ RVA 0x00451ff0 (91 instructions, 2 basic blocks)
    pub fn method_04(&mut self) {
    }

    /// Decompiled virtual method #5 @ RVA 0x00452030 (64 instructions, 2 basic blocks)
    pub fn method_05(&mut self) {
    }

    /// Decompiled virtual method #6 @ RVA 0x00452070 (37 instructions, 2 basic blocks)
    pub fn method_06(&mut self) {
    }

    /// Decompiled virtual method #7 @ RVA 0x00103e90 (2 instructions, 1 basic blocks)
    pub fn method_07(&mut self) {
    }

    /// Decompiled virtual method #8 @ RVA 0x00103ea0 (9 instructions, 1 basic blocks)
    pub fn method_08(&mut self) {
        // Writes to field at offset +0x0008
    }

    /// Decompiled virtual method #9 @ RVA 0x00103e60 (2 instructions, 1 basic blocks)
    pub fn method_09(&mut self) {
    }

    /// Decompiled virtual method #10 @ RVA 0x0045bb30 (16 instructions, 2 basic blocks)
    pub fn method_10(&mut self) {
    }

    /// Decompiled virtual method #11 @ RVA 0x00451d90 (24 instructions, 2 basic blocks)
    pub fn method_11(&mut self) {
    }

    /// Decompiled virtual method #12 @ RVA 0x001e9580 (53 instructions, 12 basic blocks)
    pub fn method_12(&mut self) {
    }

    /// Decompiled virtual method #13 @ RVA 0x00103e60 (2 instructions, 1 basic blocks)
    pub fn method_13(&mut self) {
    }

    /// Decompiled virtual method #14 @ RVA 0x00103e60 (2 instructions, 1 basic blocks)
    pub fn method_14(&mut self) {
    }

    /// Decompiled virtual method #15 @ RVA 0x00451dc0 (10 instructions, 2 basic blocks)
    pub fn method_15(&mut self) {
    }

    /// Decompiled virtual method #16 @ RVA 0x00451e30 (10 instructions, 2 basic blocks)
    pub fn method_16(&mut self) {
    }

    /// Decompiled virtual method #17 @ RVA 0x00451ea0 (31 instructions, 2 basic blocks)
    pub fn method_17(&mut self) {
        // Writes to field at offset +0x0018
    }

    /// Decompiled virtual method #18 @ RVA 0x00451ee0 (10 instructions, 2 basic blocks)
    pub fn method_18(&mut self) {
    }

    /// Decompiled virtual method #19 @ RVA 0x00451f40 (10 instructions, 2 basic blocks)
    pub fn method_19(&mut self) {
    }

    /// Decompiled virtual method #20 @ RVA 0x0037bee0 (127 instructions, 27 basic blocks)
    pub fn method_20(&mut self) {
        // Writes to field at offset +0x0000
        // Writes to field at offset +0x0000
        // Writes to field at offset +0x0008
        // Writes to field at offset +0x0010
        // Writes to field at offset +0x0018
        // Writes to field at offset +0x002c
        // Writes to field at offset +0x008c
        // Writes to field at offset +0x0098
        // Writes to field at offset +0x00a0
        // Writes to field at offset +0x0564
        // Writes to field at offset +0x0568
        // Writes to field at offset +0x056c
    }

    /// Decompiled virtual method #21 @ RVA 0x0037c1f0 (114 instructions, 22 basic blocks)
    pub fn method_21(&mut self) {
        // Writes to field at offset +0x0000
        // Writes to field at offset +0x0000
        // Writes to field at offset +0x0008
        // Writes to field at offset +0x0010
        // Writes to field at offset +0x0018
        // Writes to field at offset +0x0018
        // Writes to field at offset +0x0564
    }

    /// Decompiled virtual method #22 @ RVA 0x0037c470 (39 instructions, 3 basic blocks)
    pub fn method_22(&mut self) {
    }

    /// Decompiled virtual method #23 @ RVA 0x00103e60 (2 instructions, 1 basic blocks)
    pub fn method_23(&mut self) {
    }

    /// Decompiled virtual method #24 @ RVA 0x00103e60 (2 instructions, 1 basic blocks)
    pub fn method_24(&mut self) {
    }

    /// Decompiled virtual method #25 @ RVA 0x00103e70 (2 instructions, 1 basic blocks)
    pub fn method_25(&mut self) {
    }

    /// Decompiled virtual method #26 @ RVA 0x0037c830 (4 instructions, 1 basic blocks)
    pub fn method_26(&mut self) {
    }

    /// Decompiled virtual method #27 @ RVA 0x00103e60 (2 instructions, 1 basic blocks)
    pub fn method_27(&mut self) {
    }

    /// Decompiled virtual method #28 @ RVA 0x00103e60 (2 instructions, 1 basic blocks)
    pub fn method_28(&mut self) {
    }

    /// Decompiled virtual method #29 @ RVA 0x00103e60 (2 instructions, 1 basic blocks)
    pub fn method_29(&mut self) {
    }

    /// Decompiled virtual method #30 @ RVA 0x0037c520 (103 instructions, 16 basic blocks)
    pub fn method_30(&mut self) {
    }

    /// Decompiled virtual method #31 @ RVA 0x0037c790 (37 instructions, 3 basic blocks)
    pub fn method_31(&mut self) {
    }

    /// Decompiled virtual method #32 @ RVA 0x000d3090 (1 instructions, 1 basic blocks)
    pub fn method_32(&mut self) {
    }

    /// Decompiled virtual method #33 @ RVA 0x00103e70 (2 instructions, 1 basic blocks)
    pub fn method_33(&mut self) {
    }

    /// Decompiled virtual method #34 @ RVA 0x00103e70 (2 instructions, 1 basic blocks)
    pub fn method_34(&mut self) {
    }

    /// Decompiled virtual method #35 @ RVA 0x00103e80 (2 instructions, 1 basic blocks)
    pub fn method_35(&mut self) {
    }

    /// Decompiled virtual method #36 @ RVA 0x00103e60 (2 instructions, 1 basic blocks)
    pub fn method_36(&mut self) {
    }

}

impl CcClass for ccEquipWnd {
    fn class_name(&self) -> &'static str { Self::CLASS_NAME }
    fn vtable_rva(&self) -> u32 { Self::VTABLE_RVA }
    fn virtual_method_count(&self) -> usize { Self::METHOD_COUNT }
    fn base_classes(&self) -> &'static [&'static str] { Self::BASE_CLASSES }
}

/// Decompiled CC2 class: `ccExpGauge` (VTable RVA: 0x008a4408, 6 virtual methods, 15 recovered fields)
/// Inherits: ccGauge
#[derive(Debug, Clone, Default)]
pub struct ccExpGauge {
    pub vtable_rva: u32,
    pub field_0x0000: u64,
    pub field_0x0008: f32,
    pub field_0x000c: f32,
    pub field_0x0010: f32,
    pub field_0x0022: u16,
    pub field_0x0024: f32,
    pub field_0x0028: u32,
    pub field_0x002c: f32,
    pub field_0x0030: u32,
    pub field_0x0032: u16,
    pub field_0x0034: u16,
    pub field_0x0038: u64,
    pub field_0x0040: u64,
    pub field_0x0050: f32,
    pub field_0x2bd8: u32,
}

impl ccExpGauge {
    pub const CLASS_NAME: &'static str = "ccExpGauge";
    pub const VTABLE_RVA: u32 = 0x008a4408;
    pub const METHOD_COUNT: usize = 6;
    pub const BASE_CLASSES: &'static [&'static str] = &["ccGauge"];
    pub const VIRTUAL_METHODS: &'static [u64] = &[0x00000001802a43d0, 0x00000001802a4400, 0x000000018023e560, 0x00000001802a3ef0, 0x00000001802a44e0, 0x00000001801b8190];

    pub fn new() -> Self {
        Self {
            vtable_rva: Self::VTABLE_RVA,
            field_0x0000: Default::default(),
            field_0x0008: Default::default(),
            field_0x000c: Default::default(),
            field_0x0010: Default::default(),
            field_0x0022: Default::default(),
            field_0x0024: Default::default(),
            field_0x0028: Default::default(),
            field_0x002c: Default::default(),
            field_0x0030: Default::default(),
            field_0x0032: Default::default(),
            field_0x0034: Default::default(),
            field_0x0038: Default::default(),
            field_0x0040: Default::default(),
            field_0x0050: Default::default(),
            field_0x2bd8: Default::default(),
        }
    }

    /// Decompiled virtual method #0 @ RVA 0x002a43d0 (13 instructions, 3 basic blocks)
    pub fn method_00(&mut self) {
        // Writes to field at offset +0x0000
    }

    /// Decompiled virtual method #1 @ RVA 0x002a4400 (48 instructions, 4 basic blocks)
    pub fn method_01(&mut self) {
    }

    /// Decompiled virtual method #2 @ RVA 0x0023e560 (16 instructions, 5 basic blocks)
    pub fn method_02(&mut self) {
    }

    /// Decompiled virtual method #3 @ RVA 0x002a3ef0 (58 instructions, 17 basic blocks)
    pub fn method_03(&mut self) {
        // Writes to field at offset +0x0022
        // Writes to field at offset +0x0028
        // Writes to field at offset +0x002c
        // Writes to field at offset +0x0030
    }

    /// Decompiled virtual method #4 @ RVA 0x002a44e0 (100 instructions, 3 basic blocks)
    pub fn method_04(&mut self) {
    }

    /// Decompiled virtual method #5 @ RVA 0x001b8190 (12 instructions, 1 basic blocks)
    pub fn method_05(&mut self) {
        // Writes to field at offset +0x0024
        // Writes to field at offset +0x0028
        // Writes to field at offset +0x002c
    }

}

impl CcClass for ccExpGauge {
    fn class_name(&self) -> &'static str { Self::CLASS_NAME }
    fn vtable_rva(&self) -> u32 { Self::VTABLE_RVA }
    fn virtual_method_count(&self) -> usize { Self::METHOD_COUNT }
    fn base_classes(&self) -> &'static [&'static str] { Self::BASE_CLASSES }
}

/// Decompiled CC2 class: `ccGameMenu` (VTable RVA: 0x008ba288, 31 virtual methods, 17 recovered fields)
/// Inherits: ccGMMMain, ccGameMenuMain, ccGameMenuObj
#[derive(Debug, Clone, Default)]
pub struct ccGameMenu {
    pub vtable_rva: u32,
    pub field_0x0000: u64,
    pub field_0x0002: u16,
    pub field_0x0008: u32,
    pub field_0x0018: u32,
    pub field_0x0020: u64,
    pub field_0x0028: u8,
    pub field_0x0030: u64,
    pub field_0x0034: f32,
    pub field_0x0038: u64,
    pub field_0x0040: u8,
    pub field_0x0048: u64,
    pub field_0x0050: u64,
    pub field_0x0058: u64,
    pub field_0x0068: u64,
    pub field_0x0078: u64,
    pub field_0x0080: u32,
    pub field_0x00e0: u16,
}

impl ccGameMenu {
    pub const CLASS_NAME: &'static str = "ccGameMenu";
    pub const VTABLE_RVA: u32 = 0x008ba288;
    pub const METHOD_COUNT: usize = 31;
    pub const BASE_CLASSES: &'static [&'static str] = &["ccGMMMain", "ccGameMenuMain", "ccGameMenuObj"];
    pub const VIRTUAL_METHODS: &'static [u64] = &[0x00000001804581d0, 0x0000000180452940, 0x0000000180457af0, 0x0000000180103e40, 0x0000000180452730, 0x0000000180457700, 0x0000000180457710, 0x00000001801035d0, 0x00000001801035b0, 0x0000000180457d70, 0x000000018045bbe0, 0x00000001800d3090, 0x0000000180452300, 0x0000000180452380, 0x0000000180458240, 0x0000000180103900, 0x00000001804523f0, 0x0000000180452480, 0x0000000180452520, 0x0000000180456f90, 0x0000000180103bf0, 0x0000000180457240, 0x0000000180457310, 0x00000001804574b0, 0x0000000180457570, 0x0000000180457720, 0x0000000180457720, 0x0000000180457890, 0x00000001800d3090, 0x0000000180452bd0, 0x00000001801b2800];

    pub fn new() -> Self {
        Self {
            vtable_rva: Self::VTABLE_RVA,
            field_0x0000: Default::default(),
            field_0x0002: Default::default(),
            field_0x0008: Default::default(),
            field_0x0018: Default::default(),
            field_0x0020: Default::default(),
            field_0x0028: Default::default(),
            field_0x0030: Default::default(),
            field_0x0034: Default::default(),
            field_0x0038: Default::default(),
            field_0x0040: Default::default(),
            field_0x0048: Default::default(),
            field_0x0050: Default::default(),
            field_0x0058: Default::default(),
            field_0x0068: Default::default(),
            field_0x0078: Default::default(),
            field_0x0080: Default::default(),
            field_0x00e0: Default::default(),
        }
    }

    /// Decompiled virtual method #0 @ RVA 0x004581d0 (28 instructions, 3 basic blocks)
    pub fn method_00(&mut self) {
        // Writes to field at offset +0x0000
    }

    /// Decompiled virtual method #1 @ RVA 0x00452940 (10 instructions, 2 basic blocks)
    pub fn method_01(&mut self) {
    }

    /// Decompiled virtual method #2 @ RVA 0x00457af0 (52 instructions, 7 basic blocks)
    pub fn method_02(&mut self) {
        // Writes to field at offset +0x0068
    }

    /// Decompiled virtual method #3 @ RVA 0x00103e40 (15 instructions, 2 basic blocks)
    pub fn method_03(&mut self) {
        // Writes to field at offset +0x0080
    }

    /// Decompiled virtual method #4 @ RVA 0x00452730 (9 instructions, 2 basic blocks)
    pub fn method_04(&mut self) {
        // Writes to field at offset +0x0040
    }

    /// Decompiled virtual method #5 @ RVA 0x00457700 (55 instructions, 6 basic blocks)
    pub fn method_05(&mut self) {
    }

    /// Decompiled virtual method #6 @ RVA 0x00457710 (43 instructions, 5 basic blocks)
    pub fn method_06(&mut self) {
    }

    /// Decompiled virtual method #7 @ RVA 0x001035d0 (2 instructions, 1 basic blocks)
    pub fn method_07(&mut self) {
    }

    /// Decompiled virtual method #8 @ RVA 0x001035b0 (5 instructions, 1 basic blocks)
    pub fn method_08(&mut self) {
    }

    /// Decompiled virtual method #9 @ RVA 0x00457d70 (79 instructions, 17 basic blocks)
    pub fn method_09(&mut self) {
        // Writes to field at offset +0x0000
    }

    /// Decompiled virtual method #10 @ RVA 0x0045bbe0 (11 instructions, 2 basic blocks)
    pub fn method_10(&mut self) {
    }

    /// Decompiled virtual method #11 @ RVA 0x000d3090 (1 instructions, 1 basic blocks)
    pub fn method_11(&mut self) {
    }

    /// Decompiled virtual method #12 @ RVA 0x00452300 (9 instructions, 2 basic blocks)
    pub fn method_12(&mut self) {
        // Writes to field at offset +0x0028
    }

    /// Decompiled virtual method #13 @ RVA 0x00452380 (9 instructions, 2 basic blocks)
    pub fn method_13(&mut self) {
        // Writes to field at offset +0x0028
    }

    /// Decompiled virtual method #14 @ RVA 0x00458240 (133 instructions, 25 basic blocks)
    pub fn method_14(&mut self) {
    }

    /// Decompiled virtual method #15 @ RVA 0x00103900 (122 instructions, 20 basic blocks)
    pub fn method_15(&mut self) {
    }

    /// Decompiled virtual method #16 @ RVA 0x004523f0 (12 instructions, 2 basic blocks)
    pub fn method_16(&mut self) {
    }

    /// Decompiled virtual method #17 @ RVA 0x00452480 (12 instructions, 2 basic blocks)
    pub fn method_17(&mut self) {
    }

    /// Decompiled virtual method #18 @ RVA 0x00452520 (12 instructions, 2 basic blocks)
    pub fn method_18(&mut self) {
    }

    /// Decompiled virtual method #19 @ RVA 0x00456f90 (119 instructions, 19 basic blocks)
    pub fn method_19(&mut self) {
    }

    /// Decompiled virtual method #20 @ RVA 0x00103bf0 (69 instructions, 10 basic blocks)
    pub fn method_20(&mut self) {
        // Writes to field at offset +0x0038
        // Writes to field at offset +0x0048
    }

    /// Decompiled virtual method #21 @ RVA 0x00457240 (48 instructions, 6 basic blocks)
    pub fn method_21(&mut self) {
    }

    /// Decompiled virtual method #22 @ RVA 0x00457310 (108 instructions, 11 basic blocks)
    pub fn method_22(&mut self) {
        // Writes to field at offset +0x0048
        // Writes to field at offset +0x0050
    }

    /// Decompiled virtual method #23 @ RVA 0x004574b0 (47 instructions, 3 basic blocks)
    pub fn method_23(&mut self) {
        // Writes to field at offset +0x0020
    }

    /// Decompiled virtual method #24 @ RVA 0x00457570 (103 instructions, 9 basic blocks)
    pub fn method_24(&mut self) {
        // Writes to field at offset +0x0048
        // Writes to field at offset +0x0050
    }

    /// Decompiled virtual method #25 @ RVA 0x00457720 (31 instructions, 4 basic blocks)
    pub fn method_25(&mut self) {
    }

    /// Decompiled virtual method #26 @ RVA 0x00457720 (31 instructions, 4 basic blocks)
    pub fn method_26(&mut self) {
    }

    /// Decompiled virtual method #27 @ RVA 0x00457890 (122 instructions, 21 basic blocks)
    pub fn method_27(&mut self) {
        // Writes to field at offset +0x0018
        // Writes to field at offset +0x0050
    }

    /// Decompiled virtual method #28 @ RVA 0x000d3090 (1 instructions, 1 basic blocks)
    pub fn method_28(&mut self) {
    }

    /// Decompiled virtual method #29 @ RVA 0x00452bd0 (5 instructions, 2 basic blocks)
    pub fn method_29(&mut self) {
    }

    /// Decompiled virtual method #30 @ RVA 0x001b2800 (13 instructions, 4 basic blocks)
    pub fn method_30(&mut self) {
        // Writes to field at offset +0x0018
        // Writes to field at offset +0x0028
    }

}

impl CcClass for ccGameMenu {
    fn class_name(&self) -> &'static str { Self::CLASS_NAME }
    fn vtable_rva(&self) -> u32 { Self::VTABLE_RVA }
    fn virtual_method_count(&self) -> usize { Self::METHOD_COUNT }
    fn base_classes(&self) -> &'static [&'static str] { Self::BASE_CLASSES }
}

/// Decompiled CC2 class: `ccGameMenuCtrlBase` (VTable RVA: 0x0087a2d0, 37 virtual methods, 6 recovered fields)
/// Inherits: ccGameMenuObj
#[derive(Debug, Clone, Default)]
pub struct ccGameMenuCtrlBase {
    pub vtable_rva: u32,
    pub field_0x0000: u64,
    pub field_0x0008: u32,
    pub field_0x0018: u32,
    pub field_0x0048: u64,
    pub field_0x0050: u64,
    pub field_0x009c: u32,
}

impl ccGameMenuCtrlBase {
    pub const CLASS_NAME: &'static str = "ccGameMenuCtrlBase";
    pub const VTABLE_RVA: u32 = 0x0087a2d0;
    pub const METHOD_COUNT: usize = 37;
    pub const BASE_CLASSES: &'static [&'static str] = &["ccGameMenuObj"];
    pub const VIRTUAL_METHODS: &'static [u64] = &[0x00000001801b2740, 0x00000001804520b0, 0x0000000180452130, 0x0000000180451fb0, 0x0000000180451ff0, 0x0000000180452030, 0x0000000180452070, 0x0000000180103e90, 0x0000000180103ea0, 0x0000000180103e60, 0x000000018045bb30, 0x0000000180451d90, 0x00000001801e9580, 0x0000000180103e60, 0x0000000180103e60, 0x0000000180451dc0, 0x0000000180451e30, 0x0000000180451ea0, 0x0000000180451ee0, 0x0000000180451f40, 0x0000000180103e60, 0x0000000180103e60, 0x0000000180103e60, 0x0000000180103e60, 0x0000000180103e60, 0x0000000180103e70, 0x0000000180103e70, 0x0000000180103e60, 0x0000000180103e60, 0x0000000180103e60, 0x0000000180103e60, 0x00000001800d3090, 0x00000001800d3090, 0x0000000180103e70, 0x0000000180103e70, 0x0000000180103e80, 0x0000000180103e60];

    pub fn new() -> Self {
        Self {
            vtable_rva: Self::VTABLE_RVA,
            field_0x0000: Default::default(),
            field_0x0008: Default::default(),
            field_0x0018: Default::default(),
            field_0x0048: Default::default(),
            field_0x0050: Default::default(),
            field_0x009c: Default::default(),
        }
    }

    /// Decompiled virtual method #0 @ RVA 0x001b2740 (30 instructions, 3 basic blocks)
    pub fn method_00(&mut self) {
        // Writes to field at offset +0x0000
    }

    /// Decompiled virtual method #1 @ RVA 0x004520b0 (10 instructions, 2 basic blocks)
    pub fn method_01(&mut self) {
    }

    /// Decompiled virtual method #2 @ RVA 0x00452130 (35 instructions, 9 basic blocks)
    pub fn method_02(&mut self) {
    }

    /// Decompiled virtual method #3 @ RVA 0x00451fb0 (118 instructions, 2 basic blocks)
    pub fn method_03(&mut self) {
    }

    /// Decompiled virtual method #4 @ RVA 0x00451ff0 (91 instructions, 2 basic blocks)
    pub fn method_04(&mut self) {
    }

    /// Decompiled virtual method #5 @ RVA 0x00452030 (64 instructions, 2 basic blocks)
    pub fn method_05(&mut self) {
    }

    /// Decompiled virtual method #6 @ RVA 0x00452070 (37 instructions, 2 basic blocks)
    pub fn method_06(&mut self) {
    }

    /// Decompiled virtual method #7 @ RVA 0x00103e90 (2 instructions, 1 basic blocks)
    pub fn method_07(&mut self) {
    }

    /// Decompiled virtual method #8 @ RVA 0x00103ea0 (9 instructions, 1 basic blocks)
    pub fn method_08(&mut self) {
        // Writes to field at offset +0x0008
    }

    /// Decompiled virtual method #9 @ RVA 0x00103e60 (2 instructions, 1 basic blocks)
    pub fn method_09(&mut self) {
    }

    /// Decompiled virtual method #10 @ RVA 0x0045bb30 (16 instructions, 2 basic blocks)
    pub fn method_10(&mut self) {
    }

    /// Decompiled virtual method #11 @ RVA 0x00451d90 (24 instructions, 2 basic blocks)
    pub fn method_11(&mut self) {
    }

    /// Decompiled virtual method #12 @ RVA 0x001e9580 (53 instructions, 12 basic blocks)
    pub fn method_12(&mut self) {
    }

    /// Decompiled virtual method #13 @ RVA 0x00103e60 (2 instructions, 1 basic blocks)
    pub fn method_13(&mut self) {
    }

    /// Decompiled virtual method #14 @ RVA 0x00103e60 (2 instructions, 1 basic blocks)
    pub fn method_14(&mut self) {
    }

    /// Decompiled virtual method #15 @ RVA 0x00451dc0 (10 instructions, 2 basic blocks)
    pub fn method_15(&mut self) {
    }

    /// Decompiled virtual method #16 @ RVA 0x00451e30 (10 instructions, 2 basic blocks)
    pub fn method_16(&mut self) {
    }

    /// Decompiled virtual method #17 @ RVA 0x00451ea0 (31 instructions, 2 basic blocks)
    pub fn method_17(&mut self) {
        // Writes to field at offset +0x0018
    }

    /// Decompiled virtual method #18 @ RVA 0x00451ee0 (10 instructions, 2 basic blocks)
    pub fn method_18(&mut self) {
    }

    /// Decompiled virtual method #19 @ RVA 0x00451f40 (10 instructions, 2 basic blocks)
    pub fn method_19(&mut self) {
    }

    /// Decompiled virtual method #20 @ RVA 0x00103e60 (2 instructions, 1 basic blocks)
    pub fn method_20(&mut self) {
    }

    /// Decompiled virtual method #21 @ RVA 0x00103e60 (2 instructions, 1 basic blocks)
    pub fn method_21(&mut self) {
    }

    /// Decompiled virtual method #22 @ RVA 0x00103e60 (2 instructions, 1 basic blocks)
    pub fn method_22(&mut self) {
    }

    /// Decompiled virtual method #23 @ RVA 0x00103e60 (2 instructions, 1 basic blocks)
    pub fn method_23(&mut self) {
    }

    /// Decompiled virtual method #24 @ RVA 0x00103e60 (2 instructions, 1 basic blocks)
    pub fn method_24(&mut self) {
    }

    /// Decompiled virtual method #25 @ RVA 0x00103e70 (2 instructions, 1 basic blocks)
    pub fn method_25(&mut self) {
    }

    /// Decompiled virtual method #26 @ RVA 0x00103e70 (2 instructions, 1 basic blocks)
    pub fn method_26(&mut self) {
    }

    /// Decompiled virtual method #27 @ RVA 0x00103e60 (2 instructions, 1 basic blocks)
    pub fn method_27(&mut self) {
    }

    /// Decompiled virtual method #28 @ RVA 0x00103e60 (2 instructions, 1 basic blocks)
    pub fn method_28(&mut self) {
    }

    /// Decompiled virtual method #29 @ RVA 0x00103e60 (2 instructions, 1 basic blocks)
    pub fn method_29(&mut self) {
    }

    /// Decompiled virtual method #30 @ RVA 0x00103e60 (2 instructions, 1 basic blocks)
    pub fn method_30(&mut self) {
    }

    /// Decompiled virtual method #31 @ RVA 0x000d3090 (1 instructions, 1 basic blocks)
    pub fn method_31(&mut self) {
    }

    /// Decompiled virtual method #32 @ RVA 0x000d3090 (1 instructions, 1 basic blocks)
    pub fn method_32(&mut self) {
    }

    /// Decompiled virtual method #33 @ RVA 0x00103e70 (2 instructions, 1 basic blocks)
    pub fn method_33(&mut self) {
    }

    /// Decompiled virtual method #34 @ RVA 0x00103e70 (2 instructions, 1 basic blocks)
    pub fn method_34(&mut self) {
    }

    /// Decompiled virtual method #35 @ RVA 0x00103e80 (2 instructions, 1 basic blocks)
    pub fn method_35(&mut self) {
    }

    /// Decompiled virtual method #36 @ RVA 0x00103e60 (2 instructions, 1 basic blocks)
    pub fn method_36(&mut self) {
    }

}

impl CcClass for ccGameMenuCtrlBase {
    fn class_name(&self) -> &'static str { Self::CLASS_NAME }
    fn vtable_rva(&self) -> u32 { Self::VTABLE_RVA }
    fn virtual_method_count(&self) -> usize { Self::METHOD_COUNT }
    fn base_classes(&self) -> &'static [&'static str] { Self::BASE_CLASSES }
}

/// Decompiled CC2 class: `ccGameMenuCtrlBaseOpton` (VTable RVA: 0x008ba818, 37 virtual methods, 14 recovered fields)
/// Inherits: ccGameMenuCtrlBase, ccGameMenuObj
#[derive(Debug, Clone, Default)]
pub struct ccGameMenuCtrlBaseOpton {
    pub vtable_rva: u32,
    pub field_0x0000: u64,
    pub field_0x0008: u32,
    pub field_0x000c: u32,
    pub field_0x0018: u32,
    pub field_0x0019: u8,
    pub field_0x001a: u8,
    pub field_0x0048: u64,
    pub field_0x0050: u64,
    pub field_0x0068: u64,
    pub field_0x0070: u32,
    pub field_0x007c: u32,
    pub field_0x0094: u32,
    pub field_0x009c: u32,
    pub field_0x00a8: u64,
}

impl ccGameMenuCtrlBaseOpton {
    pub const CLASS_NAME: &'static str = "ccGameMenuCtrlBaseOpton";
    pub const VTABLE_RVA: u32 = 0x008ba818;
    pub const METHOD_COUNT: usize = 37;
    pub const BASE_CLASSES: &'static [&'static str] = &["ccGameMenuCtrlBase", "ccGameMenuObj"];
    pub const VIRTUAL_METHODS: &'static [u64] = &[0x00000001804515a0, 0x00000001804520b0, 0x0000000180452130, 0x0000000180451fb0, 0x0000000180451ff0, 0x0000000180452030, 0x0000000180452070, 0x0000000180103e90, 0x0000000180103ea0, 0x0000000180103e60, 0x000000018045bb30, 0x0000000180451d90, 0x00000001801e9580, 0x000000018045d620, 0x000000018045dae0, 0x0000000180451dc0, 0x0000000180451e30, 0x0000000180451ea0, 0x0000000180451ee0, 0x0000000180451f40, 0x000000018045db00, 0x000000018045db70, 0x0000000180103e60, 0x0000000180103e60, 0x0000000180103e60, 0x0000000180103e70, 0x0000000180103e70, 0x0000000180103e60, 0x0000000180103e60, 0x0000000180103e60, 0x000000018045db90, 0x000000018045dd60, 0x00000001800d3090, 0x0000000180103e70, 0x0000000180103e70, 0x0000000180103e80, 0x0000000180103e60];

    pub fn new() -> Self {
        Self {
            vtable_rva: Self::VTABLE_RVA,
            field_0x0000: Default::default(),
            field_0x0008: Default::default(),
            field_0x000c: Default::default(),
            field_0x0018: Default::default(),
            field_0x0019: Default::default(),
            field_0x001a: Default::default(),
            field_0x0048: Default::default(),
            field_0x0050: Default::default(),
            field_0x0068: Default::default(),
            field_0x0070: Default::default(),
            field_0x007c: Default::default(),
            field_0x0094: Default::default(),
            field_0x009c: Default::default(),
            field_0x00a8: Default::default(),
        }
    }

    /// Decompiled virtual method #0 @ RVA 0x004515a0 (30 instructions, 3 basic blocks)
    pub fn method_00(&mut self) {
        // Writes to field at offset +0x0000
    }

    /// Decompiled virtual method #1 @ RVA 0x004520b0 (10 instructions, 2 basic blocks)
    pub fn method_01(&mut self) {
    }

    /// Decompiled virtual method #2 @ RVA 0x00452130 (35 instructions, 9 basic blocks)
    pub fn method_02(&mut self) {
    }

    /// Decompiled virtual method #3 @ RVA 0x00451fb0 (118 instructions, 2 basic blocks)
    pub fn method_03(&mut self) {
    }

    /// Decompiled virtual method #4 @ RVA 0x00451ff0 (91 instructions, 2 basic blocks)
    pub fn method_04(&mut self) {
    }

    /// Decompiled virtual method #5 @ RVA 0x00452030 (64 instructions, 2 basic blocks)
    pub fn method_05(&mut self) {
    }

    /// Decompiled virtual method #6 @ RVA 0x00452070 (37 instructions, 2 basic blocks)
    pub fn method_06(&mut self) {
    }

    /// Decompiled virtual method #7 @ RVA 0x00103e90 (2 instructions, 1 basic blocks)
    pub fn method_07(&mut self) {
    }

    /// Decompiled virtual method #8 @ RVA 0x00103ea0 (9 instructions, 1 basic blocks)
    pub fn method_08(&mut self) {
        // Writes to field at offset +0x0008
    }

    /// Decompiled virtual method #9 @ RVA 0x00103e60 (2 instructions, 1 basic blocks)
    pub fn method_09(&mut self) {
    }

    /// Decompiled virtual method #10 @ RVA 0x0045bb30 (16 instructions, 2 basic blocks)
    pub fn method_10(&mut self) {
    }

    /// Decompiled virtual method #11 @ RVA 0x00451d90 (24 instructions, 2 basic blocks)
    pub fn method_11(&mut self) {
    }

    /// Decompiled virtual method #12 @ RVA 0x001e9580 (53 instructions, 12 basic blocks)
    pub fn method_12(&mut self) {
    }

    /// Decompiled virtual method #13 @ RVA 0x0045d620 (118 instructions, 23 basic blocks)
    pub fn method_13(&mut self) {
    }

    /// Decompiled virtual method #14 @ RVA 0x0045dae0 (6 instructions, 1 basic blocks)
    pub fn method_14(&mut self) {
    }

    /// Decompiled virtual method #15 @ RVA 0x00451dc0 (10 instructions, 2 basic blocks)
    pub fn method_15(&mut self) {
    }

    /// Decompiled virtual method #16 @ RVA 0x00451e30 (10 instructions, 2 basic blocks)
    pub fn method_16(&mut self) {
    }

    /// Decompiled virtual method #17 @ RVA 0x00451ea0 (31 instructions, 2 basic blocks)
    pub fn method_17(&mut self) {
        // Writes to field at offset +0x0018
    }

    /// Decompiled virtual method #18 @ RVA 0x00451ee0 (10 instructions, 2 basic blocks)
    pub fn method_18(&mut self) {
    }

    /// Decompiled virtual method #19 @ RVA 0x00451f40 (10 instructions, 2 basic blocks)
    pub fn method_19(&mut self) {
    }

    /// Decompiled virtual method #20 @ RVA 0x0045db00 (21 instructions, 1 basic blocks)
    pub fn method_20(&mut self) {
    }

    /// Decompiled virtual method #21 @ RVA 0x0045db70 (7 instructions, 1 basic blocks)
    pub fn method_21(&mut self) {
    }

    /// Decompiled virtual method #22 @ RVA 0x00103e60 (2 instructions, 1 basic blocks)
    pub fn method_22(&mut self) {
    }

    /// Decompiled virtual method #23 @ RVA 0x00103e60 (2 instructions, 1 basic blocks)
    pub fn method_23(&mut self) {
    }

    /// Decompiled virtual method #24 @ RVA 0x00103e60 (2 instructions, 1 basic blocks)
    pub fn method_24(&mut self) {
    }

    /// Decompiled virtual method #25 @ RVA 0x00103e70 (2 instructions, 1 basic blocks)
    pub fn method_25(&mut self) {
    }

    /// Decompiled virtual method #26 @ RVA 0x00103e70 (2 instructions, 1 basic blocks)
    pub fn method_26(&mut self) {
    }

    /// Decompiled virtual method #27 @ RVA 0x00103e60 (2 instructions, 1 basic blocks)
    pub fn method_27(&mut self) {
    }

    /// Decompiled virtual method #28 @ RVA 0x00103e60 (2 instructions, 1 basic blocks)
    pub fn method_28(&mut self) {
    }

    /// Decompiled virtual method #29 @ RVA 0x00103e60 (2 instructions, 1 basic blocks)
    pub fn method_29(&mut self) {
    }

    /// Decompiled virtual method #30 @ RVA 0x0045db90 (78 instructions, 15 basic blocks)
    pub fn method_30(&mut self) {
        // Writes to field at offset +0x0019
    }

    /// Decompiled virtual method #31 @ RVA 0x0045dd60 (39 instructions, 2 basic blocks)
    pub fn method_31(&mut self) {
        // Writes to field at offset +0x0008
        // Writes to field at offset +0x000c
        // Writes to field at offset +0x0050
        // Writes to field at offset +0x0068
    }

    /// Decompiled virtual method #32 @ RVA 0x000d3090 (1 instructions, 1 basic blocks)
    pub fn method_32(&mut self) {
    }

    /// Decompiled virtual method #33 @ RVA 0x00103e70 (2 instructions, 1 basic blocks)
    pub fn method_33(&mut self) {
    }

    /// Decompiled virtual method #34 @ RVA 0x00103e70 (2 instructions, 1 basic blocks)
    pub fn method_34(&mut self) {
    }

    /// Decompiled virtual method #35 @ RVA 0x00103e80 (2 instructions, 1 basic blocks)
    pub fn method_35(&mut self) {
    }

    /// Decompiled virtual method #36 @ RVA 0x00103e60 (2 instructions, 1 basic blocks)
    pub fn method_36(&mut self) {
    }

}

impl CcClass for ccGameMenuCtrlBaseOpton {
    fn class_name(&self) -> &'static str { Self::CLASS_NAME }
    fn vtable_rva(&self) -> u32 { Self::VTABLE_RVA }
    fn virtual_method_count(&self) -> usize { Self::METHOD_COUNT }
    fn base_classes(&self) -> &'static [&'static str] { Self::BASE_CLASSES }
}

/// Decompiled CC2 class: `ccGameMenuItem` (VTable RVA: 0x0087bf10, 12 virtual methods, 9 recovered fields)
/// Inherits: ccGameMenuObj
#[derive(Debug, Clone, Default)]
pub struct ccGameMenuItem {
    pub vtable_rva: u32,
    pub field_0x0000: u64,
    pub field_0x0008: u32,
    pub field_0x0010: u32,
    pub field_0x0018: u32,
    pub field_0x001c: u32,
    pub field_0x0020: f32,
    pub field_0x0024: f32,
    pub field_0x0030: u32,
    pub field_0x0040: u64,
}

impl ccGameMenuItem {
    pub const CLASS_NAME: &'static str = "ccGameMenuItem";
    pub const VTABLE_RVA: u32 = 0x0087bf10;
    pub const METHOD_COUNT: usize = 12;
    pub const BASE_CLASSES: &'static [&'static str] = &["ccGameMenuObj"];
    pub const VIRTUAL_METHODS: &'static [u64] = &[0x00000001801b8150, 0x0000000180451c60, 0x0000000180451ca0, 0x0000000180103e60, 0x0000000180103e60, 0x0000000180103e60, 0x0000000180103e60, 0x0000000180103e70, 0x0000000180103ea0, 0x0000000180103e60, 0x0000000180103e60, 0x0000000180103e60];

    pub fn new() -> Self {
        Self {
            vtable_rva: Self::VTABLE_RVA,
            field_0x0000: Default::default(),
            field_0x0008: Default::default(),
            field_0x0010: Default::default(),
            field_0x0018: Default::default(),
            field_0x001c: Default::default(),
            field_0x0020: Default::default(),
            field_0x0024: Default::default(),
            field_0x0030: Default::default(),
            field_0x0040: Default::default(),
        }
    }

    /// Decompiled virtual method #0 @ RVA 0x001b8150 (13 instructions, 3 basic blocks)
    pub fn method_00(&mut self) {
        // Writes to field at offset +0x0000
    }

    /// Decompiled virtual method #1 @ RVA 0x00451c60 (84 instructions, 14 basic blocks)
    pub fn method_01(&mut self) {
        // Writes to field at offset +0x0000
        // Writes to field at offset +0x0010
        // Writes to field at offset +0x0030
    }

    /// Decompiled virtual method #2 @ RVA 0x00451ca0 (65 instructions, 11 basic blocks)
    pub fn method_02(&mut self) {
        // Writes to field at offset +0x0000
    }

    /// Decompiled virtual method #3 @ RVA 0x00103e60 (2 instructions, 1 basic blocks)
    pub fn method_03(&mut self) {
    }

    /// Decompiled virtual method #4 @ RVA 0x00103e60 (2 instructions, 1 basic blocks)
    pub fn method_04(&mut self) {
    }

    /// Decompiled virtual method #5 @ RVA 0x00103e60 (2 instructions, 1 basic blocks)
    pub fn method_05(&mut self) {
    }

    /// Decompiled virtual method #6 @ RVA 0x00103e60 (2 instructions, 1 basic blocks)
    pub fn method_06(&mut self) {
    }

    /// Decompiled virtual method #7 @ RVA 0x00103e70 (2 instructions, 1 basic blocks)
    pub fn method_07(&mut self) {
    }

    /// Decompiled virtual method #8 @ RVA 0x00103ea0 (9 instructions, 1 basic blocks)
    pub fn method_08(&mut self) {
        // Writes to field at offset +0x0008
    }

    /// Decompiled virtual method #9 @ RVA 0x00103e60 (2 instructions, 1 basic blocks)
    pub fn method_09(&mut self) {
    }

    /// Decompiled virtual method #10 @ RVA 0x00103e60 (2 instructions, 1 basic blocks)
    pub fn method_10(&mut self) {
    }

    /// Decompiled virtual method #11 @ RVA 0x00103e60 (2 instructions, 1 basic blocks)
    pub fn method_11(&mut self) {
    }

}

impl CcClass for ccGameMenuItem {
    fn class_name(&self) -> &'static str { Self::CLASS_NAME }
    fn vtable_rva(&self) -> u32 { Self::VTABLE_RVA }
    fn virtual_method_count(&self) -> usize { Self::METHOD_COUNT }
    fn base_classes(&self) -> &'static [&'static str] { Self::BASE_CLASSES }
}

/// Decompiled CC2 class: `ccGameMenuMain` (VTable RVA: 0x0087a170, 31 virtual methods, 8 recovered fields)
/// Inherits: ccGameMenuObj
#[derive(Debug, Clone, Default)]
pub struct ccGameMenuMain {
    pub vtable_rva: u32,
    pub field_0x0000: u64,
    pub field_0x0008: u32,
    pub field_0x0018: u32,
    pub field_0x0028: u8,
    pub field_0x0030: u32,
    pub field_0x0034: f32,
    pub field_0x0038: u64,
    pub field_0x0040: u8,
}

impl ccGameMenuMain {
    pub const CLASS_NAME: &'static str = "ccGameMenuMain";
    pub const VTABLE_RVA: u32 = 0x0087a170;
    pub const METHOD_COUNT: usize = 31;
    pub const BASE_CLASSES: &'static [&'static str] = &["ccGameMenuObj"];
    pub const VIRTUAL_METHODS: &'static [u64] = &[0x00000001801b2830, 0x0000000180452940, 0x0000000180452b60, 0x00000001804525e0, 0x0000000180452730, 0x00000001804527e0, 0x0000000180452890, 0x00000001801b27f0, 0x00000001801035b0, 0x0000000180452bf0, 0x000000018045ba00, 0x00000001800d3090, 0x0000000180452300, 0x0000000180452380, 0x0000000180103e70, 0x0000000180103e70, 0x00000001804523f0, 0x0000000180452480, 0x0000000180452520, 0x0000000180103e60, 0x0000000180103e60, 0x0000000180103e60, 0x0000000180103e60, 0x0000000180103e60, 0x0000000180103e60, 0x0000000180103e60, 0x0000000180103e60, 0x0000000180103e70, 0x00000001800d3090, 0x0000000180452bd0, 0x00000001801b2800];

    pub fn new() -> Self {
        Self {
            vtable_rva: Self::VTABLE_RVA,
            field_0x0000: Default::default(),
            field_0x0008: Default::default(),
            field_0x0018: Default::default(),
            field_0x0028: Default::default(),
            field_0x0030: Default::default(),
            field_0x0034: Default::default(),
            field_0x0038: Default::default(),
            field_0x0040: Default::default(),
        }
    }

    /// Decompiled virtual method #0 @ RVA 0x001b2830 (23 instructions, 3 basic blocks)
    pub fn method_00(&mut self) {
        // Writes to field at offset +0x0000
    }

    /// Decompiled virtual method #1 @ RVA 0x00452940 (10 instructions, 2 basic blocks)
    pub fn method_01(&mut self) {
    }

    /// Decompiled virtual method #2 @ RVA 0x00452b60 (30 instructions, 4 basic blocks)
    pub fn method_02(&mut self) {
    }

    /// Decompiled virtual method #3 @ RVA 0x004525e0 (9 instructions, 2 basic blocks)
    pub fn method_03(&mut self) {
        // Writes to field at offset +0x0040
    }

    /// Decompiled virtual method #4 @ RVA 0x00452730 (9 instructions, 2 basic blocks)
    pub fn method_04(&mut self) {
        // Writes to field at offset +0x0040
    }

    /// Decompiled virtual method #5 @ RVA 0x004527e0 (9 instructions, 2 basic blocks)
    pub fn method_05(&mut self) {
        // Writes to field at offset +0x0040
    }

    /// Decompiled virtual method #6 @ RVA 0x00452890 (9 instructions, 2 basic blocks)
    pub fn method_06(&mut self) {
        // Writes to field at offset +0x0040
    }

    /// Decompiled virtual method #7 @ RVA 0x001b27f0 (2 instructions, 1 basic blocks)
    pub fn method_07(&mut self) {
    }

    /// Decompiled virtual method #8 @ RVA 0x001035b0 (5 instructions, 1 basic blocks)
    pub fn method_08(&mut self) {
    }

    /// Decompiled virtual method #9 @ RVA 0x00452bf0 (12 instructions, 2 basic blocks)
    pub fn method_09(&mut self) {
        // Writes to field at offset +0x0038
    }

    /// Decompiled virtual method #10 @ RVA 0x0045ba00 (17 instructions, 2 basic blocks)
    pub fn method_10(&mut self) {
    }

    /// Decompiled virtual method #11 @ RVA 0x000d3090 (1 instructions, 1 basic blocks)
    pub fn method_11(&mut self) {
    }

    /// Decompiled virtual method #12 @ RVA 0x00452300 (9 instructions, 2 basic blocks)
    pub fn method_12(&mut self) {
        // Writes to field at offset +0x0028
    }

    /// Decompiled virtual method #13 @ RVA 0x00452380 (9 instructions, 2 basic blocks)
    pub fn method_13(&mut self) {
        // Writes to field at offset +0x0028
    }

    /// Decompiled virtual method #14 @ RVA 0x00103e70 (2 instructions, 1 basic blocks)
    pub fn method_14(&mut self) {
    }

    /// Decompiled virtual method #15 @ RVA 0x00103e70 (2 instructions, 1 basic blocks)
    pub fn method_15(&mut self) {
    }

    /// Decompiled virtual method #16 @ RVA 0x004523f0 (12 instructions, 2 basic blocks)
    pub fn method_16(&mut self) {
    }

    /// Decompiled virtual method #17 @ RVA 0x00452480 (12 instructions, 2 basic blocks)
    pub fn method_17(&mut self) {
    }

    /// Decompiled virtual method #18 @ RVA 0x00452520 (12 instructions, 2 basic blocks)
    pub fn method_18(&mut self) {
    }

    /// Decompiled virtual method #19 @ RVA 0x00103e60 (2 instructions, 1 basic blocks)
    pub fn method_19(&mut self) {
    }

    /// Decompiled virtual method #20 @ RVA 0x00103e60 (2 instructions, 1 basic blocks)
    pub fn method_20(&mut self) {
    }

    /// Decompiled virtual method #21 @ RVA 0x00103e60 (2 instructions, 1 basic blocks)
    pub fn method_21(&mut self) {
    }

    /// Decompiled virtual method #22 @ RVA 0x00103e60 (2 instructions, 1 basic blocks)
    pub fn method_22(&mut self) {
    }

    /// Decompiled virtual method #23 @ RVA 0x00103e60 (2 instructions, 1 basic blocks)
    pub fn method_23(&mut self) {
    }

    /// Decompiled virtual method #24 @ RVA 0x00103e60 (2 instructions, 1 basic blocks)
    pub fn method_24(&mut self) {
    }

    /// Decompiled virtual method #25 @ RVA 0x00103e60 (2 instructions, 1 basic blocks)
    pub fn method_25(&mut self) {
    }

    /// Decompiled virtual method #26 @ RVA 0x00103e60 (2 instructions, 1 basic blocks)
    pub fn method_26(&mut self) {
    }

    /// Decompiled virtual method #27 @ RVA 0x00103e70 (2 instructions, 1 basic blocks)
    pub fn method_27(&mut self) {
    }

    /// Decompiled virtual method #28 @ RVA 0x000d3090 (1 instructions, 1 basic blocks)
    pub fn method_28(&mut self) {
    }

    /// Decompiled virtual method #29 @ RVA 0x00452bd0 (5 instructions, 2 basic blocks)
    pub fn method_29(&mut self) {
    }

    /// Decompiled virtual method #30 @ RVA 0x001b2800 (13 instructions, 4 basic blocks)
    pub fn method_30(&mut self) {
        // Writes to field at offset +0x0018
        // Writes to field at offset +0x0028
    }

}

impl CcClass for ccGameMenuMain {
    fn class_name(&self) -> &'static str { Self::CLASS_NAME }
    fn vtable_rva(&self) -> u32 { Self::VTABLE_RVA }
    fn virtual_method_count(&self) -> usize { Self::METHOD_COUNT }
    fn base_classes(&self) -> &'static [&'static str] { Self::BASE_CLASSES }
}

/// Decompiled CC2 class: `ccGameMenuMainOption` (VTable RVA: 0x008ba948, 31 virtual methods, 12 recovered fields)
/// Inherits: ccGameMenuMain, ccGameMenuObj
#[derive(Debug, Clone, Default)]
pub struct ccGameMenuMainOption {
    pub vtable_rva: u32,
    pub field_0x0000: u64,
    pub field_0x0008: u32,
    pub field_0x000c: u32,
    pub field_0x0018: u32,
    pub field_0x0028: u8,
    pub field_0x0030: u32,
    pub field_0x0034: f32,
    pub field_0x0038: u64,
    pub field_0x0040: u8,
    pub field_0x0048: u32,
    pub field_0x0050: u32,
    pub field_0x0068: u64,
}

impl ccGameMenuMainOption {
    pub const CLASS_NAME: &'static str = "ccGameMenuMainOption";
    pub const VTABLE_RVA: u32 = 0x008ba948;
    pub const METHOD_COUNT: usize = 31;
    pub const BASE_CLASSES: &'static [&'static str] = &["ccGameMenuMain", "ccGameMenuObj"];
    pub const VIRTUAL_METHODS: &'static [u64] = &[0x0000000180451780, 0x0000000180452940, 0x000000018045df00, 0x00000001804525e0, 0x0000000180452730, 0x00000001804527e0, 0x0000000180452890, 0x00000001801b27f0, 0x00000001801035b0, 0x0000000180452bf0, 0x000000018045ba00, 0x00000001800d3090, 0x0000000180452300, 0x0000000180452380, 0x000000018045dd70, 0x000000018045ddf0, 0x00000001804523f0, 0x0000000180452480, 0x0000000180452520, 0x000000018045de30, 0x000000018045de70, 0x000000018045deb0, 0x0000000180103e70, 0x0000000180103e70, 0x0000000180103e60, 0x0000000180103e60, 0x0000000180103e60, 0x00000001801c65a0, 0x00000001800d3090, 0x0000000180452bd0, 0x00000001801b2800];

    pub fn new() -> Self {
        Self {
            vtable_rva: Self::VTABLE_RVA,
            field_0x0000: Default::default(),
            field_0x0008: Default::default(),
            field_0x000c: Default::default(),
            field_0x0018: Default::default(),
            field_0x0028: Default::default(),
            field_0x0030: Default::default(),
            field_0x0034: Default::default(),
            field_0x0038: Default::default(),
            field_0x0040: Default::default(),
            field_0x0048: Default::default(),
            field_0x0050: Default::default(),
            field_0x0068: Default::default(),
        }
    }

    /// Decompiled virtual method #0 @ RVA 0x00451780 (16 instructions, 3 basic blocks)
    pub fn method_00(&mut self) {
    }

    /// Decompiled virtual method #1 @ RVA 0x00452940 (10 instructions, 2 basic blocks)
    pub fn method_01(&mut self) {
    }

    /// Decompiled virtual method #2 @ RVA 0x0045df00 (19 instructions, 3 basic blocks)
    pub fn method_02(&mut self) {
    }

    /// Decompiled virtual method #3 @ RVA 0x004525e0 (9 instructions, 2 basic blocks)
    pub fn method_03(&mut self) {
        // Writes to field at offset +0x0040
    }

    /// Decompiled virtual method #4 @ RVA 0x00452730 (9 instructions, 2 basic blocks)
    pub fn method_04(&mut self) {
        // Writes to field at offset +0x0040
    }

    /// Decompiled virtual method #5 @ RVA 0x004527e0 (9 instructions, 2 basic blocks)
    pub fn method_05(&mut self) {
        // Writes to field at offset +0x0040
    }

    /// Decompiled virtual method #6 @ RVA 0x00452890 (9 instructions, 2 basic blocks)
    pub fn method_06(&mut self) {
        // Writes to field at offset +0x0040
    }

    /// Decompiled virtual method #7 @ RVA 0x001b27f0 (2 instructions, 1 basic blocks)
    pub fn method_07(&mut self) {
    }

    /// Decompiled virtual method #8 @ RVA 0x001035b0 (5 instructions, 1 basic blocks)
    pub fn method_08(&mut self) {
    }

    /// Decompiled virtual method #9 @ RVA 0x00452bf0 (12 instructions, 2 basic blocks)
    pub fn method_09(&mut self) {
        // Writes to field at offset +0x0038
    }

    /// Decompiled virtual method #10 @ RVA 0x0045ba00 (17 instructions, 2 basic blocks)
    pub fn method_10(&mut self) {
    }

    /// Decompiled virtual method #11 @ RVA 0x000d3090 (1 instructions, 1 basic blocks)
    pub fn method_11(&mut self) {
    }

    /// Decompiled virtual method #12 @ RVA 0x00452300 (9 instructions, 2 basic blocks)
    pub fn method_12(&mut self) {
        // Writes to field at offset +0x0028
    }

    /// Decompiled virtual method #13 @ RVA 0x00452380 (9 instructions, 2 basic blocks)
    pub fn method_13(&mut self) {
        // Writes to field at offset +0x0028
    }

    /// Decompiled virtual method #14 @ RVA 0x0045dd70 (30 instructions, 1 basic blocks)
    pub fn method_14(&mut self) {
        // Writes to field at offset +0x0008
        // Writes to field at offset +0x000c
        // Writes to field at offset +0x0050
        // Writes to field at offset +0x0068
    }

    /// Decompiled virtual method #15 @ RVA 0x0045ddf0 (15 instructions, 1 basic blocks)
    pub fn method_15(&mut self) {
    }

    /// Decompiled virtual method #16 @ RVA 0x004523f0 (12 instructions, 2 basic blocks)
    pub fn method_16(&mut self) {
    }

    /// Decompiled virtual method #17 @ RVA 0x00452480 (12 instructions, 2 basic blocks)
    pub fn method_17(&mut self) {
    }

    /// Decompiled virtual method #18 @ RVA 0x00452520 (12 instructions, 2 basic blocks)
    pub fn method_18(&mut self) {
    }

    /// Decompiled virtual method #19 @ RVA 0x0045de30 (15 instructions, 1 basic blocks)
    pub fn method_19(&mut self) {
    }

    /// Decompiled virtual method #20 @ RVA 0x0045de70 (15 instructions, 1 basic blocks)
    pub fn method_20(&mut self) {
    }

    /// Decompiled virtual method #21 @ RVA 0x0045deb0 (21 instructions, 1 basic blocks)
    pub fn method_21(&mut self) {
    }

    /// Decompiled virtual method #22 @ RVA 0x00103e70 (2 instructions, 1 basic blocks)
    pub fn method_22(&mut self) {
    }

    /// Decompiled virtual method #23 @ RVA 0x00103e70 (2 instructions, 1 basic blocks)
    pub fn method_23(&mut self) {
    }

    /// Decompiled virtual method #24 @ RVA 0x00103e60 (2 instructions, 1 basic blocks)
    pub fn method_24(&mut self) {
    }

    /// Decompiled virtual method #25 @ RVA 0x00103e60 (2 instructions, 1 basic blocks)
    pub fn method_25(&mut self) {
    }

    /// Decompiled virtual method #26 @ RVA 0x00103e60 (2 instructions, 1 basic blocks)
    pub fn method_26(&mut self) {
    }

    /// Decompiled virtual method #27 @ RVA 0x001c65a0 (11 instructions, 3 basic blocks)
    pub fn method_27(&mut self) {
        // Writes to field at offset +0x0018
    }

    /// Decompiled virtual method #28 @ RVA 0x000d3090 (1 instructions, 1 basic blocks)
    pub fn method_28(&mut self) {
    }

    /// Decompiled virtual method #29 @ RVA 0x00452bd0 (5 instructions, 2 basic blocks)
    pub fn method_29(&mut self) {
    }

    /// Decompiled virtual method #30 @ RVA 0x001b2800 (13 instructions, 4 basic blocks)
    pub fn method_30(&mut self) {
        // Writes to field at offset +0x0018
        // Writes to field at offset +0x0028
    }

}

impl CcClass for ccGameMenuMainOption {
    fn class_name(&self) -> &'static str { Self::CLASS_NAME }
    fn vtable_rva(&self) -> u32 { Self::VTABLE_RVA }
    fn virtual_method_count(&self) -> usize { Self::METHOD_COUNT }
    fn base_classes(&self) -> &'static [&'static str] { Self::BASE_CLASSES }
}

/// Decompiled CC2 class: `ccGameMenuObj` (VTable RVA: 0x0087a270, 11 virtual methods, 1 recovered fields)
#[derive(Debug, Clone, Default)]
pub struct ccGameMenuObj {
    pub vtable_rva: u32,
    pub field_0x0000: u64,
}

impl ccGameMenuObj {
    pub const CLASS_NAME: &'static str = "ccGameMenuObj";
    pub const VTABLE_RVA: u32 = 0x0087a270;
    pub const METHOD_COUNT: usize = 11;
    pub const BASE_CLASSES: &'static [&'static str] = &[];
    pub const VIRTUAL_METHODS: &'static [u64] = &[0x00000001801b26c0, 0x0000000180103e60, 0x00000001800d3090, 0x0000000180103e60, 0x0000000180103e60, 0x0000000180103e60, 0x0000000180103e60, 0x000000018073e2c0, 0x00000001801b26b0, 0x0000000180103e60, 0x0000000180103e60];

    pub fn new() -> Self {
        Self {
            vtable_rva: Self::VTABLE_RVA,
            field_0x0000: Default::default(),
        }
    }

    /// Decompiled virtual method #0 @ RVA 0x001b26c0 (13 instructions, 3 basic blocks)
    pub fn method_00(&mut self) {
        // Writes to field at offset +0x0000
    }

    /// Decompiled virtual method #1 @ RVA 0x00103e60 (2 instructions, 1 basic blocks)
    pub fn method_01(&mut self) {
    }

    /// Decompiled virtual method #2 @ RVA 0x000d3090 (1 instructions, 1 basic blocks)
    pub fn method_02(&mut self) {
    }

    /// Decompiled virtual method #3 @ RVA 0x00103e60 (2 instructions, 1 basic blocks)
    pub fn method_03(&mut self) {
    }

    /// Decompiled virtual method #4 @ RVA 0x00103e60 (2 instructions, 1 basic blocks)
    pub fn method_04(&mut self) {
    }

    /// Decompiled virtual method #5 @ RVA 0x00103e60 (2 instructions, 1 basic blocks)
    pub fn method_05(&mut self) {
    }

    /// Decompiled virtual method #6 @ RVA 0x00103e60 (2 instructions, 1 basic blocks)
    pub fn method_06(&mut self) {
    }

    /// Decompiled virtual method #7 @ RVA 0x0073e2c0 (337 instructions, 1 basic blocks)
    pub fn method_07(&mut self) {
    }

    /// Decompiled virtual method #8 @ RVA 0x001b26b0 (3 instructions, 1 basic blocks)
    pub fn method_08(&mut self) {
    }

    /// Decompiled virtual method #9 @ RVA 0x00103e60 (2 instructions, 1 basic blocks)
    pub fn method_09(&mut self) {
    }

    /// Decompiled virtual method #10 @ RVA 0x00103e60 (2 instructions, 1 basic blocks)
    pub fn method_10(&mut self) {
    }

}

impl CcClass for ccGameMenuObj {
    fn class_name(&self) -> &'static str { Self::CLASS_NAME }
    fn vtable_rva(&self) -> u32 { Self::VTABLE_RVA }
    fn virtual_method_count(&self) -> usize { Self::METHOD_COUNT }
    fn base_classes(&self) -> &'static [&'static str] { Self::BASE_CLASSES }
}

/// Decompiled CC2 class: `ccGauge` (VTable RVA: 0x0087bed8, 6 virtual methods, 6 recovered fields)
#[derive(Debug, Clone, Default)]
pub struct ccGauge {
    pub vtable_rva: u32,
    pub field_0x0000: u64,
    pub field_0x0022: u16,
    pub field_0x0024: f32,
    pub field_0x0028: u32,
    pub field_0x002c: f32,
    pub field_0x0030: u32,
}

impl ccGauge {
    pub const CLASS_NAME: &'static str = "ccGauge";
    pub const VTABLE_RVA: u32 = 0x0087bed8;
    pub const METHOD_COUNT: usize = 6;
    pub const BASE_CLASSES: &'static [&'static str] = &[];
    pub const VIRTUAL_METHODS: &'static [u64] = &[0x00000001801b81c0, 0x00000001800d3090, 0x00000001800d3090, 0x00000001802a3ef0, 0x00000001800d3090, 0x00000001801b8190];

    pub fn new() -> Self {
        Self {
            vtable_rva: Self::VTABLE_RVA,
            field_0x0000: Default::default(),
            field_0x0022: Default::default(),
            field_0x0024: Default::default(),
            field_0x0028: Default::default(),
            field_0x002c: Default::default(),
            field_0x0030: Default::default(),
        }
    }

    /// Decompiled virtual method #0 @ RVA 0x001b81c0 (13 instructions, 3 basic blocks)
    pub fn method_00(&mut self) {
        // Writes to field at offset +0x0000
    }

    /// Decompiled virtual method #1 @ RVA 0x000d3090 (1 instructions, 1 basic blocks)
    pub fn method_01(&mut self) {
    }

    /// Decompiled virtual method #2 @ RVA 0x000d3090 (1 instructions, 1 basic blocks)
    pub fn method_02(&mut self) {
    }

    /// Decompiled virtual method #3 @ RVA 0x002a3ef0 (58 instructions, 17 basic blocks)
    pub fn method_03(&mut self) {
        // Writes to field at offset +0x0022
        // Writes to field at offset +0x0028
        // Writes to field at offset +0x002c
        // Writes to field at offset +0x0030
    }

    /// Decompiled virtual method #4 @ RVA 0x000d3090 (1 instructions, 1 basic blocks)
    pub fn method_04(&mut self) {
    }

    /// Decompiled virtual method #5 @ RVA 0x001b8190 (12 instructions, 1 basic blocks)
    pub fn method_05(&mut self) {
        // Writes to field at offset +0x0024
        // Writes to field at offset +0x0028
        // Writes to field at offset +0x002c
    }

}

impl CcClass for ccGauge {
    fn class_name(&self) -> &'static str { Self::CLASS_NAME }
    fn vtable_rva(&self) -> u32 { Self::VTABLE_RVA }
    fn virtual_method_count(&self) -> usize { Self::METHOD_COUNT }
    fn base_classes(&self) -> &'static [&'static str] { Self::BASE_CLASSES }
}

/// Decompiled CC2 class: `ccGaugeDisp` (VTable RVA: 0x0087bea0, 6 virtual methods, 9 recovered fields)
/// Inherits: ccGauge
#[derive(Debug, Clone, Default)]
pub struct ccGaugeDisp {
    pub vtable_rva: u32,
    pub field_0x001c: u32,
    pub field_0x0020: u16,
    pub field_0x0022: u16,
    pub field_0x0024: f32,
    pub field_0x0028: f32,
    pub field_0x002c: u32,
    pub field_0x0030: u32,
    pub field_0x0040: u16,
    pub field_0x0048: u64,
}

impl ccGaugeDisp {
    pub const CLASS_NAME: &'static str = "ccGaugeDisp";
    pub const VTABLE_RVA: u32 = 0x0087bea0;
    pub const METHOD_COUNT: usize = 6;
    pub const BASE_CLASSES: &'static [&'static str] = &["ccGauge"];
    pub const VIRTUAL_METHODS: &'static [u64] = &[0x00000001801b83b0, 0x00000001800d3090, 0x00000001800d3090, 0x00000001802a4e90, 0x00000001802a4ff0, 0x00000001801b8330];

    pub fn new() -> Self {
        Self {
            vtable_rva: Self::VTABLE_RVA,
            field_0x001c: Default::default(),
            field_0x0020: Default::default(),
            field_0x0022: Default::default(),
            field_0x0024: Default::default(),
            field_0x0028: Default::default(),
            field_0x002c: Default::default(),
            field_0x0030: Default::default(),
            field_0x0040: Default::default(),
            field_0x0048: Default::default(),
        }
    }

    /// Decompiled virtual method #0 @ RVA 0x001b83b0 (16 instructions, 3 basic blocks)
    pub fn method_00(&mut self) {
    }

    /// Decompiled virtual method #1 @ RVA 0x000d3090 (1 instructions, 1 basic blocks)
    pub fn method_01(&mut self) {
    }

    /// Decompiled virtual method #2 @ RVA 0x000d3090 (1 instructions, 1 basic blocks)
    pub fn method_02(&mut self) {
    }

    /// Decompiled virtual method #3 @ RVA 0x002a4e90 (86 instructions, 11 basic blocks)
    pub fn method_03(&mut self) {
        // Writes to field at offset +0x001c
        // Writes to field at offset +0x0022
        // Writes to field at offset +0x0028
        // Writes to field at offset +0x002c
        // Writes to field at offset +0x0030
    }

    /// Decompiled virtual method #4 @ RVA 0x002a4ff0 (38 instructions, 7 basic blocks)
    pub fn method_04(&mut self) {
    }

    /// Decompiled virtual method #5 @ RVA 0x001b8330 (35 instructions, 3 basic blocks)
    pub fn method_05(&mut self) {
        // Writes to field at offset +0x0024
        // Writes to field at offset +0x0028
        // Writes to field at offset +0x002c
    }

}

impl CcClass for ccGaugeDisp {
    fn class_name(&self) -> &'static str { Self::CLASS_NAME }
    fn vtable_rva(&self) -> u32 { Self::VTABLE_RVA }
    fn virtual_method_count(&self) -> usize { Self::METHOD_COUNT }
    fn base_classes(&self) -> &'static [&'static str] { Self::BASE_CLASSES }
}

/// Decompiled CC2 class: `ccGetItemWnd` (VTable RVA: 0x008d80d0, 15 virtual methods, 14 recovered fields)
/// Inherits: ccBResItemWnd, ccBResListWnd, ccDispBase
#[derive(Debug, Clone, Default)]
pub struct ccGetItemWnd {
    pub vtable_rva: u32,
    pub field_0x0000: u64,
    pub field_0x0008: u32,
    pub field_0x001a: u8,
    pub field_0x001c: f32,
    pub field_0x0028: u32,
    pub field_0x002c: u32,
    pub field_0x0030: u32,
    pub field_0x0034: u32,
    pub field_0x0038: u64,
    pub field_0x0040: u32,
    pub field_0x0044: u32,
    pub field_0x0048: u64,
    pub field_0x0068: u64,
    pub field_0x0078: u16,
}

impl ccGetItemWnd {
    pub const CLASS_NAME: &'static str = "ccGetItemWnd";
    pub const VTABLE_RVA: u32 = 0x008d80d0;
    pub const METHOD_COUNT: usize = 15;
    pub const BASE_CLASSES: &'static [&'static str] = &["ccBResItemWnd", "ccBResListWnd", "ccDispBase"];
    pub const VIRTUAL_METHODS: &'static [u64] = &[0x0000000180590770, 0x0000000180242dc0, 0x0000000180242f90, 0x0000000180590a20, 0x00000001802a0e30, 0x00000001800d3090, 0x00000001801b81f0, 0x00000001801b8200, 0x00000001801b8210, 0x00000001800d3090, 0x0000000180243150, 0x0000000180243060, 0x00000001802430d0, 0x00000001802430e0, 0x0000000180244010];

    pub fn new() -> Self {
        Self {
            vtable_rva: Self::VTABLE_RVA,
            field_0x0000: Default::default(),
            field_0x0008: Default::default(),
            field_0x001a: Default::default(),
            field_0x001c: Default::default(),
            field_0x0028: Default::default(),
            field_0x002c: Default::default(),
            field_0x0030: Default::default(),
            field_0x0034: Default::default(),
            field_0x0038: Default::default(),
            field_0x0040: Default::default(),
            field_0x0044: Default::default(),
            field_0x0048: Default::default(),
            field_0x0068: Default::default(),
            field_0x0078: Default::default(),
        }
    }

    /// Decompiled virtual method #0 @ RVA 0x00590770 (28 instructions, 3 basic blocks)
    pub fn method_00(&mut self) {
        // Writes to field at offset +0x0000
    }

    /// Decompiled virtual method #1 @ RVA 0x00242dc0 (107 instructions, 15 basic blocks)
    pub fn method_01(&mut self) {
        // Writes to field at offset +0x0040
        // Writes to field at offset +0x0044
    }

    /// Decompiled virtual method #2 @ RVA 0x00242f90 (57 instructions, 13 basic blocks)
    pub fn method_02(&mut self) {
    }

    /// Decompiled virtual method #3 @ RVA 0x00590a20 (52 instructions, 7 basic blocks)
    pub fn method_03(&mut self) {
        // Writes to field at offset +0x0068
    }

    /// Decompiled virtual method #4 @ RVA 0x002a0e30 (48 instructions, 11 basic blocks)
    pub fn method_04(&mut self) {
        // Writes to field at offset +0x0008
    }

    /// Decompiled virtual method #5 @ RVA 0x000d3090 (1 instructions, 1 basic blocks)
    pub fn method_05(&mut self) {
    }

    /// Decompiled virtual method #6 @ RVA 0x001b81f0 (2 instructions, 1 basic blocks)
    pub fn method_06(&mut self) {
        // Writes to field at offset +0x0008
    }

    /// Decompiled virtual method #7 @ RVA 0x001b8200 (5 instructions, 1 basic blocks)
    pub fn method_07(&mut self) {
    }

    /// Decompiled virtual method #8 @ RVA 0x001b8210 (5 instructions, 1 basic blocks)
    pub fn method_08(&mut self) {
    }

    /// Decompiled virtual method #9 @ RVA 0x000d3090 (1 instructions, 1 basic blocks)
    pub fn method_09(&mut self) {
    }

    /// Decompiled virtual method #10 @ RVA 0x00243150 (37 instructions, 8 basic blocks)
    pub fn method_10(&mut self) {
    }

    /// Decompiled virtual method #11 @ RVA 0x00243060 (104 instructions, 21 basic blocks)
    pub fn method_11(&mut self) {
        // Writes to field at offset +0x001c
        // Writes to field at offset +0x001c
    }

    /// Decompiled virtual method #12 @ RVA 0x002430d0 (75 instructions, 15 basic blocks)
    pub fn method_12(&mut self) {
        // Writes to field at offset +0x001c
        // Writes to field at offset +0x001c
    }

    /// Decompiled virtual method #13 @ RVA 0x002430e0 (66 instructions, 14 basic blocks)
    pub fn method_13(&mut self) {
        // Writes to field at offset +0x001c
        // Writes to field at offset +0x001c
    }

    /// Decompiled virtual method #14 @ RVA 0x00244010 (110 instructions, 6 basic blocks)
    pub fn method_14(&mut self) {
        // Writes to field at offset +0x001a
    }

}

impl CcClass for ccGetItemWnd {
    fn class_name(&self) -> &'static str { Self::CLASS_NAME }
    fn vtable_rva(&self) -> u32 { Self::VTABLE_RVA }
    fn virtual_method_count(&self) -> usize { Self::METHOD_COUNT }
    fn base_classes(&self) -> &'static [&'static str] { Self::BASE_CLASSES }
}

/// Decompiled CC2 class: `ccHpGauge` (VTable RVA: 0x008a4620, 6 virtual methods, 15 recovered fields)
/// Inherits: ccGauge
#[derive(Debug, Clone, Default)]
pub struct ccHpGauge {
    pub vtable_rva: u32,
    pub field_0x0008: f32,
    pub field_0x000c: f32,
    pub field_0x0010: f32,
    pub field_0x0014: f32,
    pub field_0x0018: f32,
    pub field_0x001c: f32,
    pub field_0x0022: u16,
    pub field_0x0024: f32,
    pub field_0x0028: u32,
    pub field_0x002c: f32,
    pub field_0x0030: u32,
    pub field_0x0038: u64,
    pub field_0x0040: u64,
    pub field_0x0048: u64,
    pub field_0x2bd8: u32,
}

impl ccHpGauge {
    pub const CLASS_NAME: &'static str = "ccHpGauge";
    pub const VTABLE_RVA: u32 = 0x008a4620;
    pub const METHOD_COUNT: usize = 6;
    pub const BASE_CLASSES: &'static [&'static str] = &["ccGauge"];
    pub const VIRTUAL_METHODS: &'static [u64] = &[0x000000018029fed0, 0x00000001802a3fe0, 0x00000001802340b0, 0x00000001802a3ef0, 0x00000001802a4090, 0x00000001801b8190];

    pub fn new() -> Self {
        Self {
            vtable_rva: Self::VTABLE_RVA,
            field_0x0008: Default::default(),
            field_0x000c: Default::default(),
            field_0x0010: Default::default(),
            field_0x0014: Default::default(),
            field_0x0018: Default::default(),
            field_0x001c: Default::default(),
            field_0x0022: Default::default(),
            field_0x0024: Default::default(),
            field_0x0028: Default::default(),
            field_0x002c: Default::default(),
            field_0x0030: Default::default(),
            field_0x0038: Default::default(),
            field_0x0040: Default::default(),
            field_0x0048: Default::default(),
            field_0x2bd8: Default::default(),
        }
    }

    /// Decompiled virtual method #0 @ RVA 0x0029fed0 (16 instructions, 3 basic blocks)
    pub fn method_00(&mut self) {
    }

    /// Decompiled virtual method #1 @ RVA 0x002a3fe0 (40 instructions, 3 basic blocks)
    pub fn method_01(&mut self) {
    }

    /// Decompiled virtual method #2 @ RVA 0x002340b0 (21 instructions, 7 basic blocks)
    pub fn method_02(&mut self) {
    }

    /// Decompiled virtual method #3 @ RVA 0x002a3ef0 (58 instructions, 17 basic blocks)
    pub fn method_03(&mut self) {
        // Writes to field at offset +0x0022
        // Writes to field at offset +0x0028
        // Writes to field at offset +0x002c
        // Writes to field at offset +0x0030
    }

    /// Decompiled virtual method #4 @ RVA 0x002a4090 (108 instructions, 14 basic blocks)
    pub fn method_04(&mut self) {
    }

    /// Decompiled virtual method #5 @ RVA 0x001b8190 (12 instructions, 1 basic blocks)
    pub fn method_05(&mut self) {
        // Writes to field at offset +0x0024
        // Writes to field at offset +0x0028
        // Writes to field at offset +0x002c
    }

}

impl CcClass for ccHpGauge {
    fn class_name(&self) -> &'static str { Self::CLASS_NAME }
    fn vtable_rva(&self) -> u32 { Self::VTABLE_RVA }
    fn virtual_method_count(&self) -> usize { Self::METHOD_COUNT }
    fn base_classes(&self) -> &'static [&'static str] { Self::BASE_CLASSES }
}

/// Decompiled CC2 class: `ccItemGiveupMenuCtrl` (VTable RVA: 0x008c3e58, 37 virtual methods, 21 recovered fields)
/// Inherits: ccGameMenuCtrlBase, ccGameMenuObj
#[derive(Debug, Clone, Default)]
pub struct ccItemGiveupMenuCtrl {
    pub vtable_rva: u32,
    pub field_0x0000: u64,
    pub field_0x0004: u32,
    pub field_0x0008: u32,
    pub field_0x000c: u8,
    pub field_0x0010: u32,
    pub field_0x0018: u32,
    pub field_0x0038: u32,
    pub field_0x0048: u64,
    pub field_0x0050: u64,
    pub field_0x0068: u64,
    pub field_0x0070: u64,
    pub field_0x009c: u32,
    pub field_0x0168: u64,
    pub field_0x0170: u32,
    pub field_0x01a0: u8,
    pub field_0x01a1: u8,
    pub field_0x01a2: u8,
    pub field_0x025f: u8,
    pub field_0x3664: u32,
    pub field_0x3668: u16,
    pub field_0x366a: u16,
}

impl ccItemGiveupMenuCtrl {
    pub const CLASS_NAME: &'static str = "ccItemGiveupMenuCtrl";
    pub const VTABLE_RVA: u32 = 0x008c3e58;
    pub const METHOD_COUNT: usize = 37;
    pub const BASE_CLASSES: &'static [&'static str] = &["ccGameMenuCtrlBase", "ccGameMenuObj"];
    pub const VIRTUAL_METHODS: &'static [u64] = &[0x000000018055a070, 0x00000001804520b0, 0x0000000180452130, 0x0000000180451fb0, 0x0000000180451ff0, 0x0000000180452030, 0x0000000180452070, 0x0000000180103e90, 0x0000000180103ea0, 0x0000000180103e60, 0x000000018045bb30, 0x0000000180451d90, 0x00000001801e9580, 0x000000018055a300, 0x0000000180103e60, 0x0000000180451dc0, 0x0000000180451e30, 0x0000000180451ea0, 0x0000000180451ee0, 0x0000000180451f40, 0x000000018055a910, 0x000000018055a9f0, 0x0000000180103e70, 0x0000000180103e60, 0x0000000180103e60, 0x0000000180103e70, 0x0000000180103e70, 0x0000000180103e60, 0x0000000180103e60, 0x0000000180103e60, 0x000000018055aaf0, 0x000000018055ac40, 0x00000001800d3090, 0x0000000180103e70, 0x0000000180103e70, 0x0000000180103e70, 0x0000000180103e60];

    pub fn new() -> Self {
        Self {
            vtable_rva: Self::VTABLE_RVA,
            field_0x0000: Default::default(),
            field_0x0004: Default::default(),
            field_0x0008: Default::default(),
            field_0x000c: Default::default(),
            field_0x0010: Default::default(),
            field_0x0018: Default::default(),
            field_0x0038: Default::default(),
            field_0x0048: Default::default(),
            field_0x0050: Default::default(),
            field_0x0068: Default::default(),
            field_0x0070: Default::default(),
            field_0x009c: Default::default(),
            field_0x0168: Default::default(),
            field_0x0170: Default::default(),
            field_0x01a0: Default::default(),
            field_0x01a1: Default::default(),
            field_0x01a2: Default::default(),
            field_0x025f: Default::default(),
            field_0x3664: Default::default(),
            field_0x3668: Default::default(),
            field_0x366a: Default::default(),
        }
    }

    /// Decompiled virtual method #0 @ RVA 0x0055a070 (16 instructions, 3 basic blocks)
    pub fn method_00(&mut self) {
    }

    /// Decompiled virtual method #1 @ RVA 0x004520b0 (10 instructions, 2 basic blocks)
    pub fn method_01(&mut self) {
    }

    /// Decompiled virtual method #2 @ RVA 0x00452130 (35 instructions, 9 basic blocks)
    pub fn method_02(&mut self) {
    }

    /// Decompiled virtual method #3 @ RVA 0x00451fb0 (118 instructions, 2 basic blocks)
    pub fn method_03(&mut self) {
    }

    /// Decompiled virtual method #4 @ RVA 0x00451ff0 (91 instructions, 2 basic blocks)
    pub fn method_04(&mut self) {
    }

    /// Decompiled virtual method #5 @ RVA 0x00452030 (64 instructions, 2 basic blocks)
    pub fn method_05(&mut self) {
    }

    /// Decompiled virtual method #6 @ RVA 0x00452070 (37 instructions, 2 basic blocks)
    pub fn method_06(&mut self) {
    }

    /// Decompiled virtual method #7 @ RVA 0x00103e90 (2 instructions, 1 basic blocks)
    pub fn method_07(&mut self) {
    }

    /// Decompiled virtual method #8 @ RVA 0x00103ea0 (9 instructions, 1 basic blocks)
    pub fn method_08(&mut self) {
        // Writes to field at offset +0x0008
    }

    /// Decompiled virtual method #9 @ RVA 0x00103e60 (2 instructions, 1 basic blocks)
    pub fn method_09(&mut self) {
    }

    /// Decompiled virtual method #10 @ RVA 0x0045bb30 (16 instructions, 2 basic blocks)
    pub fn method_10(&mut self) {
    }

    /// Decompiled virtual method #11 @ RVA 0x00451d90 (24 instructions, 2 basic blocks)
    pub fn method_11(&mut self) {
    }

    /// Decompiled virtual method #12 @ RVA 0x001e9580 (53 instructions, 12 basic blocks)
    pub fn method_12(&mut self) {
    }

    /// Decompiled virtual method #13 @ RVA 0x0055a300 (126 instructions, 13 basic blocks)
    pub fn method_13(&mut self) {
        // Writes to field at offset +0x025f
    }

    /// Decompiled virtual method #14 @ RVA 0x00103e60 (2 instructions, 1 basic blocks)
    pub fn method_14(&mut self) {
    }

    /// Decompiled virtual method #15 @ RVA 0x00451dc0 (10 instructions, 2 basic blocks)
    pub fn method_15(&mut self) {
    }

    /// Decompiled virtual method #16 @ RVA 0x00451e30 (10 instructions, 2 basic blocks)
    pub fn method_16(&mut self) {
    }

    /// Decompiled virtual method #17 @ RVA 0x00451ea0 (31 instructions, 2 basic blocks)
    pub fn method_17(&mut self) {
        // Writes to field at offset +0x0018
    }

    /// Decompiled virtual method #18 @ RVA 0x00451ee0 (10 instructions, 2 basic blocks)
    pub fn method_18(&mut self) {
    }

    /// Decompiled virtual method #19 @ RVA 0x00451f40 (10 instructions, 2 basic blocks)
    pub fn method_19(&mut self) {
    }

    /// Decompiled virtual method #20 @ RVA 0x0055a910 (47 instructions, 11 basic blocks)
    pub fn method_20(&mut self) {
        // Writes to field at offset +0x0000
        // Writes to field at offset +0x0004
        // Writes to field at offset +0x0008
        // Writes to field at offset +0x000c
        // Writes to field at offset +0x000c
        // Writes to field at offset +0x0010
        // Writes to field at offset +0x0170
        // Writes to field at offset +0x01a0
        // Writes to field at offset +0x01a1
        // Writes to field at offset +0x3664
        // Writes to field at offset +0x3668
        // Writes to field at offset +0x366a
    }

    /// Decompiled virtual method #21 @ RVA 0x0055a9f0 (53 instructions, 18 basic blocks)
    pub fn method_21(&mut self) {
        // Writes to field at offset +0x0038
        // Writes to field at offset +0x0048
        // Writes to field at offset +0x01a0
        // Writes to field at offset +0x01a1
        // Writes to field at offset +0x01a2
    }

    /// Decompiled virtual method #22 @ RVA 0x00103e70 (2 instructions, 1 basic blocks)
    pub fn method_22(&mut self) {
    }

    /// Decompiled virtual method #23 @ RVA 0x00103e60 (2 instructions, 1 basic blocks)
    pub fn method_23(&mut self) {
    }

    /// Decompiled virtual method #24 @ RVA 0x00103e60 (2 instructions, 1 basic blocks)
    pub fn method_24(&mut self) {
    }

    /// Decompiled virtual method #25 @ RVA 0x00103e70 (2 instructions, 1 basic blocks)
    pub fn method_25(&mut self) {
    }

    /// Decompiled virtual method #26 @ RVA 0x00103e70 (2 instructions, 1 basic blocks)
    pub fn method_26(&mut self) {
    }

    /// Decompiled virtual method #27 @ RVA 0x00103e60 (2 instructions, 1 basic blocks)
    pub fn method_27(&mut self) {
    }

    /// Decompiled virtual method #28 @ RVA 0x00103e60 (2 instructions, 1 basic blocks)
    pub fn method_28(&mut self) {
    }

    /// Decompiled virtual method #29 @ RVA 0x00103e60 (2 instructions, 1 basic blocks)
    pub fn method_29(&mut self) {
    }

    /// Decompiled virtual method #30 @ RVA 0x0055aaf0 (75 instructions, 5 basic blocks)
    pub fn method_30(&mut self) {
    }

    /// Decompiled virtual method #31 @ RVA 0x0055ac40 (125 instructions, 28 basic blocks)
    pub fn method_31(&mut self) {
        // Writes to field at offset +0x0068
    }

    /// Decompiled virtual method #32 @ RVA 0x000d3090 (1 instructions, 1 basic blocks)
    pub fn method_32(&mut self) {
    }

    /// Decompiled virtual method #33 @ RVA 0x00103e70 (2 instructions, 1 basic blocks)
    pub fn method_33(&mut self) {
    }

    /// Decompiled virtual method #34 @ RVA 0x00103e70 (2 instructions, 1 basic blocks)
    pub fn method_34(&mut self) {
    }

    /// Decompiled virtual method #35 @ RVA 0x00103e70 (2 instructions, 1 basic blocks)
    pub fn method_35(&mut self) {
    }

    /// Decompiled virtual method #36 @ RVA 0x00103e60 (2 instructions, 1 basic blocks)
    pub fn method_36(&mut self) {
    }

}

impl CcClass for ccItemGiveupMenuCtrl {
    fn class_name(&self) -> &'static str { Self::CLASS_NAME }
    fn vtable_rva(&self) -> u32 { Self::VTABLE_RVA }
    fn virtual_method_count(&self) -> usize { Self::METHOD_COUNT }
    fn base_classes(&self) -> &'static [&'static str] { Self::BASE_CLASSES }
}

/// Decompiled CC2 class: `ccItemMenuShortcut` (VTable RVA: 0x00871508, 31 virtual methods, 17 recovered fields)
/// Inherits: ccGameMenu, ccGMMMain, ccGameMenuMain, ccGameMenuObj
#[derive(Debug, Clone, Default)]
pub struct ccItemMenuShortcut {
    pub vtable_rva: u32,
    pub field_0x0000: u64,
    pub field_0x0002: u16,
    pub field_0x0008: u32,
    pub field_0x000c: u32,
    pub field_0x0018: u32,
    pub field_0x0020: u64,
    pub field_0x0028: u8,
    pub field_0x0034: f32,
    pub field_0x0038: u64,
    pub field_0x0040: u8,
    pub field_0x0048: u32,
    pub field_0x0050: u64,
    pub field_0x0068: u64,
    pub field_0x0078: u64,
    pub field_0x0080: u32,
    pub field_0x0088: u64,
    pub field_0x0090: u32,
}

impl ccItemMenuShortcut {
    pub const CLASS_NAME: &'static str = "ccItemMenuShortcut";
    pub const VTABLE_RVA: u32 = 0x00871508;
    pub const METHOD_COUNT: usize = 31;
    pub const BASE_CLASSES: &'static [&'static str] = &["ccGameMenu", "ccGMMMain", "ccGameMenuMain", "ccGameMenuObj"];
    pub const VIRTUAL_METHODS: &'static [u64] = &[0x0000000180103760, 0x0000000180103d60, 0x0000000180457af0, 0x0000000180103e40, 0x0000000180452730, 0x0000000180457700, 0x0000000180457710, 0x00000001801035d0, 0x00000001801035b0, 0x0000000180457d70, 0x000000018045bbe0, 0x00000001800d3090, 0x0000000180452300, 0x0000000180452380, 0x00000001801037d0, 0x0000000180103900, 0x00000001804523f0, 0x0000000180452480, 0x0000000180452520, 0x0000000180103950, 0x0000000180103bf0, 0x0000000180103d30, 0x0000000180457310, 0x00000001804574b0, 0x0000000180457570, 0x0000000180457720, 0x0000000180457720, 0x0000000180457890, 0x00000001800d3090, 0x0000000180452bd0, 0x0000000180103e50];

    pub fn new() -> Self {
        Self {
            vtable_rva: Self::VTABLE_RVA,
            field_0x0000: Default::default(),
            field_0x0002: Default::default(),
            field_0x0008: Default::default(),
            field_0x000c: Default::default(),
            field_0x0018: Default::default(),
            field_0x0020: Default::default(),
            field_0x0028: Default::default(),
            field_0x0034: Default::default(),
            field_0x0038: Default::default(),
            field_0x0040: Default::default(),
            field_0x0048: Default::default(),
            field_0x0050: Default::default(),
            field_0x0068: Default::default(),
            field_0x0078: Default::default(),
            field_0x0080: Default::default(),
            field_0x0088: Default::default(),
            field_0x0090: Default::default(),
        }
    }

    /// Decompiled virtual method #0 @ RVA 0x00103760 (28 instructions, 3 basic blocks)
    pub fn method_00(&mut self) {
        // Writes to field at offset +0x0000
    }

    /// Decompiled virtual method #1 @ RVA 0x00103d60 (70 instructions, 16 basic blocks)
    pub fn method_01(&mut self) {
        // Writes to field at offset +0x0080
        // Writes to field at offset +0x0090
    }

    /// Decompiled virtual method #2 @ RVA 0x00457af0 (52 instructions, 7 basic blocks)
    pub fn method_02(&mut self) {
        // Writes to field at offset +0x0068
    }

    /// Decompiled virtual method #3 @ RVA 0x00103e40 (15 instructions, 2 basic blocks)
    pub fn method_03(&mut self) {
        // Writes to field at offset +0x0080
    }

    /// Decompiled virtual method #4 @ RVA 0x00452730 (9 instructions, 2 basic blocks)
    pub fn method_04(&mut self) {
        // Writes to field at offset +0x0040
    }

    /// Decompiled virtual method #5 @ RVA 0x00457700 (55 instructions, 6 basic blocks)
    pub fn method_05(&mut self) {
    }

    /// Decompiled virtual method #6 @ RVA 0x00457710 (43 instructions, 5 basic blocks)
    pub fn method_06(&mut self) {
    }

    /// Decompiled virtual method #7 @ RVA 0x001035d0 (2 instructions, 1 basic blocks)
    pub fn method_07(&mut self) {
    }

    /// Decompiled virtual method #8 @ RVA 0x001035b0 (5 instructions, 1 basic blocks)
    pub fn method_08(&mut self) {
    }

    /// Decompiled virtual method #9 @ RVA 0x00457d70 (79 instructions, 17 basic blocks)
    pub fn method_09(&mut self) {
        // Writes to field at offset +0x0000
    }

    /// Decompiled virtual method #10 @ RVA 0x0045bbe0 (11 instructions, 2 basic blocks)
    pub fn method_10(&mut self) {
    }

    /// Decompiled virtual method #11 @ RVA 0x000d3090 (1 instructions, 1 basic blocks)
    pub fn method_11(&mut self) {
    }

    /// Decompiled virtual method #12 @ RVA 0x00452300 (9 instructions, 2 basic blocks)
    pub fn method_12(&mut self) {
        // Writes to field at offset +0x0028
    }

    /// Decompiled virtual method #13 @ RVA 0x00452380 (9 instructions, 2 basic blocks)
    pub fn method_13(&mut self) {
        // Writes to field at offset +0x0028
    }

    /// Decompiled virtual method #14 @ RVA 0x001037d0 (71 instructions, 11 basic blocks)
    pub fn method_14(&mut self) {
        // Writes to field at offset +0x000c
        // Writes to field at offset +0x0038
    }

    /// Decompiled virtual method #15 @ RVA 0x00103900 (122 instructions, 20 basic blocks)
    pub fn method_15(&mut self) {
    }

    /// Decompiled virtual method #16 @ RVA 0x004523f0 (12 instructions, 2 basic blocks)
    pub fn method_16(&mut self) {
    }

    /// Decompiled virtual method #17 @ RVA 0x00452480 (12 instructions, 2 basic blocks)
    pub fn method_17(&mut self) {
    }

    /// Decompiled virtual method #18 @ RVA 0x00452520 (12 instructions, 2 basic blocks)
    pub fn method_18(&mut self) {
    }

    /// Decompiled virtual method #19 @ RVA 0x00103950 (118 instructions, 21 basic blocks)
    pub fn method_19(&mut self) {
    }

    /// Decompiled virtual method #20 @ RVA 0x00103bf0 (69 instructions, 10 basic blocks)
    pub fn method_20(&mut self) {
        // Writes to field at offset +0x0038
        // Writes to field at offset +0x0048
    }

    /// Decompiled virtual method #21 @ RVA 0x00103d30 (9 instructions, 1 basic blocks)
    pub fn method_21(&mut self) {
    }

    /// Decompiled virtual method #22 @ RVA 0x00457310 (108 instructions, 11 basic blocks)
    pub fn method_22(&mut self) {
        // Writes to field at offset +0x0048
        // Writes to field at offset +0x0050
    }

    /// Decompiled virtual method #23 @ RVA 0x004574b0 (47 instructions, 3 basic blocks)
    pub fn method_23(&mut self) {
        // Writes to field at offset +0x0020
    }

    /// Decompiled virtual method #24 @ RVA 0x00457570 (103 instructions, 9 basic blocks)
    pub fn method_24(&mut self) {
        // Writes to field at offset +0x0048
        // Writes to field at offset +0x0050
    }

    /// Decompiled virtual method #25 @ RVA 0x00457720 (31 instructions, 4 basic blocks)
    pub fn method_25(&mut self) {
    }

    /// Decompiled virtual method #26 @ RVA 0x00457720 (31 instructions, 4 basic blocks)
    pub fn method_26(&mut self) {
    }

    /// Decompiled virtual method #27 @ RVA 0x00457890 (122 instructions, 21 basic blocks)
    pub fn method_27(&mut self) {
        // Writes to field at offset +0x0018
        // Writes to field at offset +0x0050
    }

    /// Decompiled virtual method #28 @ RVA 0x000d3090 (1 instructions, 1 basic blocks)
    pub fn method_28(&mut self) {
    }

    /// Decompiled virtual method #29 @ RVA 0x00452bd0 (5 instructions, 2 basic blocks)
    pub fn method_29(&mut self) {
    }

    /// Decompiled virtual method #30 @ RVA 0x00103e50 (3 instructions, 1 basic blocks)
    pub fn method_30(&mut self) {
        // Writes to field at offset +0x0080
    }

}

impl CcClass for ccItemMenuShortcut {
    fn class_name(&self) -> &'static str { Self::CLASS_NAME }
    fn vtable_rva(&self) -> u32 { Self::VTABLE_RVA }
    fn virtual_method_count(&self) -> usize { Self::METHOD_COUNT }
    fn base_classes(&self) -> &'static [&'static str] { Self::BASE_CLASSES }
}

/// Decompiled CC2 class: `ccItemOrbMenuCtrl` (VTable RVA: 0x008c3a30, 37 virtual methods, 10 recovered fields)
/// Inherits: ccGameMenuCtrlBase, ccGameMenuObj
#[derive(Debug, Clone, Default)]
pub struct ccItemOrbMenuCtrl {
    pub vtable_rva: u32,
    pub field_0x0000: u64,
    pub field_0x0008: u32,
    pub field_0x0018: u32,
    pub field_0x0038: u32,
    pub field_0x0048: u64,
    pub field_0x0050: u64,
    pub field_0x0068: u32,
    pub field_0x009c: u32,
    pub field_0x2bf5: u8,
    pub field_0x2bf6: u8,
}

impl ccItemOrbMenuCtrl {
    pub const CLASS_NAME: &'static str = "ccItemOrbMenuCtrl";
    pub const VTABLE_RVA: u32 = 0x008c3a30;
    pub const METHOD_COUNT: usize = 37;
    pub const BASE_CLASSES: &'static [&'static str] = &["ccGameMenuCtrlBase", "ccGameMenuObj"];
    pub const VIRTUAL_METHODS: &'static [u64] = &[0x000000018054e5a0, 0x00000001804520b0, 0x0000000180452130, 0x0000000180451fb0, 0x0000000180451ff0, 0x0000000180452030, 0x0000000180452070, 0x0000000180103e90, 0x0000000180103ea0, 0x0000000180103e60, 0x000000018045bb30, 0x0000000180451d90, 0x00000001801e9580, 0x000000018037b1d0, 0x0000000180103e60, 0x0000000180451dc0, 0x0000000180451e30, 0x0000000180451ea0, 0x0000000180451ee0, 0x0000000180451f40, 0x000000018054e660, 0x000000018054e830, 0x0000000180103e70, 0x0000000180103e60, 0x0000000180103e60, 0x0000000180103e70, 0x0000000180103e70, 0x0000000180103e60, 0x0000000180103e60, 0x0000000180103e60, 0x000000018054e8c0, 0x00000001800d3090, 0x00000001800d3090, 0x0000000180103e70, 0x0000000180103e70, 0x0000000180103e70, 0x0000000180103e60];

    pub fn new() -> Self {
        Self {
            vtable_rva: Self::VTABLE_RVA,
            field_0x0000: Default::default(),
            field_0x0008: Default::default(),
            field_0x0018: Default::default(),
            field_0x0038: Default::default(),
            field_0x0048: Default::default(),
            field_0x0050: Default::default(),
            field_0x0068: Default::default(),
            field_0x009c: Default::default(),
            field_0x2bf5: Default::default(),
            field_0x2bf6: Default::default(),
        }
    }

    /// Decompiled virtual method #0 @ RVA 0x0054e5a0 (16 instructions, 3 basic blocks)
    pub fn method_00(&mut self) {
    }

    /// Decompiled virtual method #1 @ RVA 0x004520b0 (10 instructions, 2 basic blocks)
    pub fn method_01(&mut self) {
    }

    /// Decompiled virtual method #2 @ RVA 0x00452130 (35 instructions, 9 basic blocks)
    pub fn method_02(&mut self) {
    }

    /// Decompiled virtual method #3 @ RVA 0x00451fb0 (118 instructions, 2 basic blocks)
    pub fn method_03(&mut self) {
    }

    /// Decompiled virtual method #4 @ RVA 0x00451ff0 (91 instructions, 2 basic blocks)
    pub fn method_04(&mut self) {
    }

    /// Decompiled virtual method #5 @ RVA 0x00452030 (64 instructions, 2 basic blocks)
    pub fn method_05(&mut self) {
    }

    /// Decompiled virtual method #6 @ RVA 0x00452070 (37 instructions, 2 basic blocks)
    pub fn method_06(&mut self) {
    }

    /// Decompiled virtual method #7 @ RVA 0x00103e90 (2 instructions, 1 basic blocks)
    pub fn method_07(&mut self) {
    }

    /// Decompiled virtual method #8 @ RVA 0x00103ea0 (9 instructions, 1 basic blocks)
    pub fn method_08(&mut self) {
        // Writes to field at offset +0x0008
    }

    /// Decompiled virtual method #9 @ RVA 0x00103e60 (2 instructions, 1 basic blocks)
    pub fn method_09(&mut self) {
    }

    /// Decompiled virtual method #10 @ RVA 0x0045bb30 (16 instructions, 2 basic blocks)
    pub fn method_10(&mut self) {
    }

    /// Decompiled virtual method #11 @ RVA 0x00451d90 (24 instructions, 2 basic blocks)
    pub fn method_11(&mut self) {
    }

    /// Decompiled virtual method #12 @ RVA 0x001e9580 (53 instructions, 12 basic blocks)
    pub fn method_12(&mut self) {
    }

    /// Decompiled virtual method #13 @ RVA 0x0037b1d0 (24 instructions, 4 basic blocks)
    pub fn method_13(&mut self) {
    }

    /// Decompiled virtual method #14 @ RVA 0x00103e60 (2 instructions, 1 basic blocks)
    pub fn method_14(&mut self) {
    }

    /// Decompiled virtual method #15 @ RVA 0x00451dc0 (10 instructions, 2 basic blocks)
    pub fn method_15(&mut self) {
    }

    /// Decompiled virtual method #16 @ RVA 0x00451e30 (10 instructions, 2 basic blocks)
    pub fn method_16(&mut self) {
    }

    /// Decompiled virtual method #17 @ RVA 0x00451ea0 (31 instructions, 2 basic blocks)
    pub fn method_17(&mut self) {
        // Writes to field at offset +0x0018
    }

    /// Decompiled virtual method #18 @ RVA 0x00451ee0 (10 instructions, 2 basic blocks)
    pub fn method_18(&mut self) {
    }

    /// Decompiled virtual method #19 @ RVA 0x00451f40 (10 instructions, 2 basic blocks)
    pub fn method_19(&mut self) {
    }

    /// Decompiled virtual method #20 @ RVA 0x0054e660 (101 instructions, 17 basic blocks)
    pub fn method_20(&mut self) {
        // Writes to field at offset +0x0038
        // Writes to field at offset +0x0048
        // Writes to field at offset +0x0068
    }

    /// Decompiled virtual method #21 @ RVA 0x0054e830 (32 instructions, 8 basic blocks)
    pub fn method_21(&mut self) {
        // Writes to field at offset +0x0038
        // Writes to field at offset +0x0048
        // Writes to field at offset +0x2bf5
        // Writes to field at offset +0x2bf6
    }

    /// Decompiled virtual method #22 @ RVA 0x00103e70 (2 instructions, 1 basic blocks)
    pub fn method_22(&mut self) {
    }

    /// Decompiled virtual method #23 @ RVA 0x00103e60 (2 instructions, 1 basic blocks)
    pub fn method_23(&mut self) {
    }

    /// Decompiled virtual method #24 @ RVA 0x00103e60 (2 instructions, 1 basic blocks)
    pub fn method_24(&mut self) {
    }

    /// Decompiled virtual method #25 @ RVA 0x00103e70 (2 instructions, 1 basic blocks)
    pub fn method_25(&mut self) {
    }

    /// Decompiled virtual method #26 @ RVA 0x00103e70 (2 instructions, 1 basic blocks)
    pub fn method_26(&mut self) {
    }

    /// Decompiled virtual method #27 @ RVA 0x00103e60 (2 instructions, 1 basic blocks)
    pub fn method_27(&mut self) {
    }

    /// Decompiled virtual method #28 @ RVA 0x00103e60 (2 instructions, 1 basic blocks)
    pub fn method_28(&mut self) {
    }

    /// Decompiled virtual method #29 @ RVA 0x00103e60 (2 instructions, 1 basic blocks)
    pub fn method_29(&mut self) {
    }

    /// Decompiled virtual method #30 @ RVA 0x0054e8c0 (39 instructions, 6 basic blocks)
    pub fn method_30(&mut self) {
    }

    /// Decompiled virtual method #31 @ RVA 0x000d3090 (1 instructions, 1 basic blocks)
    pub fn method_31(&mut self) {
    }

    /// Decompiled virtual method #32 @ RVA 0x000d3090 (1 instructions, 1 basic blocks)
    pub fn method_32(&mut self) {
    }

    /// Decompiled virtual method #33 @ RVA 0x00103e70 (2 instructions, 1 basic blocks)
    pub fn method_33(&mut self) {
    }

    /// Decompiled virtual method #34 @ RVA 0x00103e70 (2 instructions, 1 basic blocks)
    pub fn method_34(&mut self) {
    }

    /// Decompiled virtual method #35 @ RVA 0x00103e70 (2 instructions, 1 basic blocks)
    pub fn method_35(&mut self) {
    }

    /// Decompiled virtual method #36 @ RVA 0x00103e60 (2 instructions, 1 basic blocks)
    pub fn method_36(&mut self) {
    }

}

impl CcClass for ccItemOrbMenuCtrl {
    fn class_name(&self) -> &'static str { Self::CLASS_NAME }
    fn vtable_rva(&self) -> u32 { Self::VTABLE_RVA }
    fn virtual_method_count(&self) -> usize { Self::METHOD_COUNT }
    fn base_classes(&self) -> &'static [&'static str] { Self::BASE_CLASSES }
}

/// Decompiled CC2 class: `ccItemWnd` (VTable RVA: 0x008c41e8, 37 virtual methods, 10 recovered fields)
/// Inherits: ccGameMenuCtrlBase, ccGameMenuObj
#[derive(Debug, Clone, Default)]
pub struct ccItemWnd {
    pub vtable_rva: u32,
    pub field_0x0000: u64,
    pub field_0x0008: u32,
    pub field_0x0010: u8,
    pub field_0x0018: u32,
    pub field_0x0048: u64,
    pub field_0x0050: u64,
    pub field_0x0060: u8,
    pub field_0x0078: u32,
    pub field_0x009c: u32,
    pub field_0x00e8: u64,
}

impl ccItemWnd {
    pub const CLASS_NAME: &'static str = "ccItemWnd";
    pub const VTABLE_RVA: u32 = 0x008c41e8;
    pub const METHOD_COUNT: usize = 37;
    pub const BASE_CLASSES: &'static [&'static str] = &["ccGameMenuCtrlBase", "ccGameMenuObj"];
    pub const VIRTUAL_METHODS: &'static [u64] = &[0x0000000180550a50, 0x00000001804520b0, 0x0000000180452130, 0x0000000180451fb0, 0x0000000180451ff0, 0x0000000180452030, 0x0000000180452070, 0x0000000180103e90, 0x0000000180103ea0, 0x0000000180103e60, 0x000000018045bb30, 0x0000000180451d90, 0x00000001801e9580, 0x0000000180103e60, 0x0000000180103e60, 0x0000000180451dc0, 0x0000000180451e30, 0x0000000180451ea0, 0x0000000180451ee0, 0x0000000180451f40, 0x0000000180103f10, 0x0000000180103f40, 0x0000000180550c30, 0x0000000180103e60, 0x0000000180103e60, 0x0000000180103e70, 0x0000000180550cf0, 0x0000000180103e60, 0x0000000180103e60, 0x0000000180103e60, 0x0000000180550c70, 0x0000000180550cb0, 0x00000001800d3090, 0x0000000180103e70, 0x0000000180103e70, 0x0000000180103e80, 0x0000000180103e60];

    pub fn new() -> Self {
        Self {
            vtable_rva: Self::VTABLE_RVA,
            field_0x0000: Default::default(),
            field_0x0008: Default::default(),
            field_0x0010: Default::default(),
            field_0x0018: Default::default(),
            field_0x0048: Default::default(),
            field_0x0050: Default::default(),
            field_0x0060: Default::default(),
            field_0x0078: Default::default(),
            field_0x009c: Default::default(),
            field_0x00e8: Default::default(),
        }
    }

    /// Decompiled virtual method #0 @ RVA 0x00550a50 (16 instructions, 3 basic blocks)
    pub fn method_00(&mut self) {
    }

    /// Decompiled virtual method #1 @ RVA 0x004520b0 (10 instructions, 2 basic blocks)
    pub fn method_01(&mut self) {
    }

    /// Decompiled virtual method #2 @ RVA 0x00452130 (35 instructions, 9 basic blocks)
    pub fn method_02(&mut self) {
    }

    /// Decompiled virtual method #3 @ RVA 0x00451fb0 (118 instructions, 2 basic blocks)
    pub fn method_03(&mut self) {
    }

    /// Decompiled virtual method #4 @ RVA 0x00451ff0 (91 instructions, 2 basic blocks)
    pub fn method_04(&mut self) {
    }

    /// Decompiled virtual method #5 @ RVA 0x00452030 (64 instructions, 2 basic blocks)
    pub fn method_05(&mut self) {
    }

    /// Decompiled virtual method #6 @ RVA 0x00452070 (37 instructions, 2 basic blocks)
    pub fn method_06(&mut self) {
    }

    /// Decompiled virtual method #7 @ RVA 0x00103e90 (2 instructions, 1 basic blocks)
    pub fn method_07(&mut self) {
    }

    /// Decompiled virtual method #8 @ RVA 0x00103ea0 (9 instructions, 1 basic blocks)
    pub fn method_08(&mut self) {
        // Writes to field at offset +0x0008
    }

    /// Decompiled virtual method #9 @ RVA 0x00103e60 (2 instructions, 1 basic blocks)
    pub fn method_09(&mut self) {
    }

    /// Decompiled virtual method #10 @ RVA 0x0045bb30 (16 instructions, 2 basic blocks)
    pub fn method_10(&mut self) {
    }

    /// Decompiled virtual method #11 @ RVA 0x00451d90 (24 instructions, 2 basic blocks)
    pub fn method_11(&mut self) {
    }

    /// Decompiled virtual method #12 @ RVA 0x001e9580 (53 instructions, 12 basic blocks)
    pub fn method_12(&mut self) {
    }

    /// Decompiled virtual method #13 @ RVA 0x00103e60 (2 instructions, 1 basic blocks)
    pub fn method_13(&mut self) {
    }

    /// Decompiled virtual method #14 @ RVA 0x00103e60 (2 instructions, 1 basic blocks)
    pub fn method_14(&mut self) {
    }

    /// Decompiled virtual method #15 @ RVA 0x00451dc0 (10 instructions, 2 basic blocks)
    pub fn method_15(&mut self) {
    }

    /// Decompiled virtual method #16 @ RVA 0x00451e30 (10 instructions, 2 basic blocks)
    pub fn method_16(&mut self) {
    }

    /// Decompiled virtual method #17 @ RVA 0x00451ea0 (31 instructions, 2 basic blocks)
    pub fn method_17(&mut self) {
        // Writes to field at offset +0x0018
    }

    /// Decompiled virtual method #18 @ RVA 0x00451ee0 (10 instructions, 2 basic blocks)
    pub fn method_18(&mut self) {
    }

    /// Decompiled virtual method #19 @ RVA 0x00451f40 (10 instructions, 2 basic blocks)
    pub fn method_19(&mut self) {
    }

    /// Decompiled virtual method #20 @ RVA 0x00103f10 (9 instructions, 1 basic blocks)
    pub fn method_20(&mut self) {
        // Writes to field at offset +0x0078
        // Writes to field at offset +0x00e8
    }

    /// Decompiled virtual method #21 @ RVA 0x00103f40 (10 instructions, 3 basic blocks)
    pub fn method_21(&mut self) {
        // Writes to field at offset +0x0010
    }

    /// Decompiled virtual method #22 @ RVA 0x00550c30 (16 instructions, 1 basic blocks)
    pub fn method_22(&mut self) {
    }

    /// Decompiled virtual method #23 @ RVA 0x00103e60 (2 instructions, 1 basic blocks)
    pub fn method_23(&mut self) {
    }

    /// Decompiled virtual method #24 @ RVA 0x00103e60 (2 instructions, 1 basic blocks)
    pub fn method_24(&mut self) {
    }

    /// Decompiled virtual method #25 @ RVA 0x00103e70 (2 instructions, 1 basic blocks)
    pub fn method_25(&mut self) {
    }

    /// Decompiled virtual method #26 @ RVA 0x00550cf0 (10 instructions, 4 basic blocks)
    pub fn method_26(&mut self) {
        // Writes to field at offset +0x0060
    }

    /// Decompiled virtual method #27 @ RVA 0x00103e60 (2 instructions, 1 basic blocks)
    pub fn method_27(&mut self) {
    }

    /// Decompiled virtual method #28 @ RVA 0x00103e60 (2 instructions, 1 basic blocks)
    pub fn method_28(&mut self) {
    }

    /// Decompiled virtual method #29 @ RVA 0x00103e60 (2 instructions, 1 basic blocks)
    pub fn method_29(&mut self) {
    }

    /// Decompiled virtual method #30 @ RVA 0x00550c70 (15 instructions, 1 basic blocks)
    pub fn method_30(&mut self) {
    }

    /// Decompiled virtual method #31 @ RVA 0x00550cb0 (35 instructions, 4 basic blocks)
    pub fn method_31(&mut self) {
        // Writes to field at offset +0x0060
    }

    /// Decompiled virtual method #32 @ RVA 0x000d3090 (1 instructions, 1 basic blocks)
    pub fn method_32(&mut self) {
    }

    /// Decompiled virtual method #33 @ RVA 0x00103e70 (2 instructions, 1 basic blocks)
    pub fn method_33(&mut self) {
    }

    /// Decompiled virtual method #34 @ RVA 0x00103e70 (2 instructions, 1 basic blocks)
    pub fn method_34(&mut self) {
    }

    /// Decompiled virtual method #35 @ RVA 0x00103e80 (2 instructions, 1 basic blocks)
    pub fn method_35(&mut self) {
    }

    /// Decompiled virtual method #36 @ RVA 0x00103e60 (2 instructions, 1 basic blocks)
    pub fn method_36(&mut self) {
    }

}

impl CcClass for ccItemWnd {
    fn class_name(&self) -> &'static str { Self::CLASS_NAME }
    fn vtable_rva(&self) -> u32 { Self::VTABLE_RVA }
    fn virtual_method_count(&self) -> usize { Self::METHOD_COUNT }
    fn base_classes(&self) -> &'static [&'static str] { Self::BASE_CLASSES }
}

/// Decompiled CC2 class: `ccLogoutWnd` (VTable RVA: 0x008c48d0, 37 virtual methods, 8 recovered fields)
/// Inherits: ccGameMenuCtrlBase, ccGameMenuObj
#[derive(Debug, Clone, Default)]
pub struct ccLogoutWnd {
    pub vtable_rva: u32,
    pub field_0x0000: u64,
    pub field_0x0008: u32,
    pub field_0x0018: u32,
    pub field_0x0048: u64,
    pub field_0x0050: u64,
    pub field_0x0060: u32,
    pub field_0x0068: u64,
    pub field_0x009c: u32,
}

impl ccLogoutWnd {
    pub const CLASS_NAME: &'static str = "ccLogoutWnd";
    pub const VTABLE_RVA: u32 = 0x008c48d0;
    pub const METHOD_COUNT: usize = 37;
    pub const BASE_CLASSES: &'static [&'static str] = &["ccGameMenuCtrlBase", "ccGameMenuObj"];
    pub const VIRTUAL_METHODS: &'static [u64] = &[0x0000000180569a30, 0x00000001804520b0, 0x0000000180452130, 0x0000000180451fb0, 0x0000000180451ff0, 0x0000000180452030, 0x0000000180452070, 0x0000000180103e90, 0x0000000180103ea0, 0x0000000180103e60, 0x000000018045bb30, 0x0000000180451d90, 0x00000001801e9580, 0x0000000180569ad0, 0x0000000180569bb0, 0x0000000180451dc0, 0x0000000180451e30, 0x0000000180451ea0, 0x0000000180451ee0, 0x0000000180451f40, 0x0000000180569c20, 0x0000000180569c90, 0x0000000180569d10, 0x0000000180103e60, 0x0000000180103e60, 0x0000000180103e70, 0x0000000180103e80, 0x0000000180103e60, 0x0000000180103e60, 0x0000000180103e60, 0x0000000180569d50, 0x000000018056a000, 0x00000001800d3090, 0x0000000180103e70, 0x0000000180103e70, 0x0000000180103e80, 0x0000000180103e60];

    pub fn new() -> Self {
        Self {
            vtable_rva: Self::VTABLE_RVA,
            field_0x0000: Default::default(),
            field_0x0008: Default::default(),
            field_0x0018: Default::default(),
            field_0x0048: Default::default(),
            field_0x0050: Default::default(),
            field_0x0060: Default::default(),
            field_0x0068: Default::default(),
            field_0x009c: Default::default(),
        }
    }

    /// Decompiled virtual method #0 @ RVA 0x00569a30 (39 instructions, 3 basic blocks)
    pub fn method_00(&mut self) {
        // Writes to field at offset +0x0000
    }

    /// Decompiled virtual method #1 @ RVA 0x004520b0 (10 instructions, 2 basic blocks)
    pub fn method_01(&mut self) {
    }

    /// Decompiled virtual method #2 @ RVA 0x00452130 (35 instructions, 9 basic blocks)
    pub fn method_02(&mut self) {
    }

    /// Decompiled virtual method #3 @ RVA 0x00451fb0 (118 instructions, 2 basic blocks)
    pub fn method_03(&mut self) {
    }

    /// Decompiled virtual method #4 @ RVA 0x00451ff0 (91 instructions, 2 basic blocks)
    pub fn method_04(&mut self) {
    }

    /// Decompiled virtual method #5 @ RVA 0x00452030 (64 instructions, 2 basic blocks)
    pub fn method_05(&mut self) {
    }

    /// Decompiled virtual method #6 @ RVA 0x00452070 (37 instructions, 2 basic blocks)
    pub fn method_06(&mut self) {
    }

    /// Decompiled virtual method #7 @ RVA 0x00103e90 (2 instructions, 1 basic blocks)
    pub fn method_07(&mut self) {
    }

    /// Decompiled virtual method #8 @ RVA 0x00103ea0 (9 instructions, 1 basic blocks)
    pub fn method_08(&mut self) {
        // Writes to field at offset +0x0008
    }

    /// Decompiled virtual method #9 @ RVA 0x00103e60 (2 instructions, 1 basic blocks)
    pub fn method_09(&mut self) {
    }

    /// Decompiled virtual method #10 @ RVA 0x0045bb30 (16 instructions, 2 basic blocks)
    pub fn method_10(&mut self) {
    }

    /// Decompiled virtual method #11 @ RVA 0x00451d90 (24 instructions, 2 basic blocks)
    pub fn method_11(&mut self) {
    }

    /// Decompiled virtual method #12 @ RVA 0x001e9580 (53 instructions, 12 basic blocks)
    pub fn method_12(&mut self) {
    }

    /// Decompiled virtual method #13 @ RVA 0x00569ad0 (60 instructions, 9 basic blocks)
    pub fn method_13(&mut self) {
    }

    /// Decompiled virtual method #14 @ RVA 0x00569bb0 (22 instructions, 2 basic blocks)
    pub fn method_14(&mut self) {
    }

    /// Decompiled virtual method #15 @ RVA 0x00451dc0 (10 instructions, 2 basic blocks)
    pub fn method_15(&mut self) {
    }

    /// Decompiled virtual method #16 @ RVA 0x00451e30 (10 instructions, 2 basic blocks)
    pub fn method_16(&mut self) {
    }

    /// Decompiled virtual method #17 @ RVA 0x00451ea0 (31 instructions, 2 basic blocks)
    pub fn method_17(&mut self) {
        // Writes to field at offset +0x0018
    }

    /// Decompiled virtual method #18 @ RVA 0x00451ee0 (10 instructions, 2 basic blocks)
    pub fn method_18(&mut self) {
    }

    /// Decompiled virtual method #19 @ RVA 0x00451f40 (10 instructions, 2 basic blocks)
    pub fn method_19(&mut self) {
    }

    /// Decompiled virtual method #20 @ RVA 0x00569c20 (25 instructions, 1 basic blocks)
    pub fn method_20(&mut self) {
        // Writes to field at offset +0x0060
    }

    /// Decompiled virtual method #21 @ RVA 0x00569c90 (33 instructions, 5 basic blocks)
    pub fn method_21(&mut self) {
        // Writes to field at offset +0x0060
    }

    /// Decompiled virtual method #22 @ RVA 0x00569d10 (15 instructions, 1 basic blocks)
    pub fn method_22(&mut self) {
    }

    /// Decompiled virtual method #23 @ RVA 0x00103e60 (2 instructions, 1 basic blocks)
    pub fn method_23(&mut self) {
    }

    /// Decompiled virtual method #24 @ RVA 0x00103e60 (2 instructions, 1 basic blocks)
    pub fn method_24(&mut self) {
    }

    /// Decompiled virtual method #25 @ RVA 0x00103e70 (2 instructions, 1 basic blocks)
    pub fn method_25(&mut self) {
    }

    /// Decompiled virtual method #26 @ RVA 0x00103e80 (2 instructions, 1 basic blocks)
    pub fn method_26(&mut self) {
    }

    /// Decompiled virtual method #27 @ RVA 0x00103e60 (2 instructions, 1 basic blocks)
    pub fn method_27(&mut self) {
    }

    /// Decompiled virtual method #28 @ RVA 0x00103e60 (2 instructions, 1 basic blocks)
    pub fn method_28(&mut self) {
    }

    /// Decompiled virtual method #29 @ RVA 0x00103e60 (2 instructions, 1 basic blocks)
    pub fn method_29(&mut self) {
    }

    /// Decompiled virtual method #30 @ RVA 0x00569d50 (32 instructions, 5 basic blocks)
    pub fn method_30(&mut self) {
    }

    /// Decompiled virtual method #31 @ RVA 0x0056a000 (106 instructions, 14 basic blocks)
    pub fn method_31(&mut self) {
    }

    /// Decompiled virtual method #32 @ RVA 0x000d3090 (1 instructions, 1 basic blocks)
    pub fn method_32(&mut self) {
    }

    /// Decompiled virtual method #33 @ RVA 0x00103e70 (2 instructions, 1 basic blocks)
    pub fn method_33(&mut self) {
    }

    /// Decompiled virtual method #34 @ RVA 0x00103e70 (2 instructions, 1 basic blocks)
    pub fn method_34(&mut self) {
    }

    /// Decompiled virtual method #35 @ RVA 0x00103e80 (2 instructions, 1 basic blocks)
    pub fn method_35(&mut self) {
    }

    /// Decompiled virtual method #36 @ RVA 0x00103e60 (2 instructions, 1 basic blocks)
    pub fn method_36(&mut self) {
    }

}

impl CcClass for ccLogoutWnd {
    fn class_name(&self) -> &'static str { Self::CLASS_NAME }
    fn vtable_rva(&self) -> u32 { Self::VTABLE_RVA }
    fn virtual_method_count(&self) -> usize { Self::METHOD_COUNT }
    fn base_classes(&self) -> &'static [&'static str] { Self::BASE_CLASSES }
}

/// Decompiled CC2 class: `ccMinTensionGauge` (VTable RVA: 0x008a42a8, 6 virtual methods, 6 recovered fields)
/// Inherits: ccGauge
#[derive(Debug, Clone, Default)]
pub struct ccMinTensionGauge {
    pub vtable_rva: u32,
    pub field_0x0000: u64,
    pub field_0x0024: f32,
    pub field_0x0028: f32,
    pub field_0x002c: f32,
    pub field_0x0038: u64,
    pub field_0x0070: u64,
}

impl ccMinTensionGauge {
    pub const CLASS_NAME: &'static str = "ccMinTensionGauge";
    pub const VTABLE_RVA: u32 = 0x008a42a8;
    pub const METHOD_COUNT: usize = 6;
    pub const BASE_CLASSES: &'static [&'static str] = &["ccGauge"];
    pub const VIRTUAL_METHODS: &'static [u64] = &[0x00000001802a9b70, 0x00000001802a9bd0, 0x00000001802aa090, 0x00000001802aa170, 0x00000001802aa2e0, 0x00000001801b8190];

    pub fn new() -> Self {
        Self {
            vtable_rva: Self::VTABLE_RVA,
            field_0x0000: Default::default(),
            field_0x0024: Default::default(),
            field_0x0028: Default::default(),
            field_0x002c: Default::default(),
            field_0x0038: Default::default(),
            field_0x0070: Default::default(),
        }
    }

    /// Decompiled virtual method #0 @ RVA 0x002a9b70 (23 instructions, 3 basic blocks)
    pub fn method_00(&mut self) {
        // Writes to field at offset +0x0000
    }

    /// Decompiled virtual method #1 @ RVA 0x002a9bd0 (124 instructions, 23 basic blocks)
    pub fn method_01(&mut self) {
    }

    /// Decompiled virtual method #2 @ RVA 0x002aa090 (60 instructions, 17 basic blocks)
    pub fn method_02(&mut self) {
    }

    /// Decompiled virtual method #3 @ RVA 0x002aa170 (68 instructions, 14 basic blocks)
    pub fn method_03(&mut self) {
        // Writes to field at offset +0x0038
    }

    /// Decompiled virtual method #4 @ RVA 0x002aa2e0 (102 instructions, 4 basic blocks)
    pub fn method_04(&mut self) {
        // Writes to field at offset +0x0000
        // Writes to field at offset +0x0038
    }

    /// Decompiled virtual method #5 @ RVA 0x001b8190 (12 instructions, 1 basic blocks)
    pub fn method_05(&mut self) {
        // Writes to field at offset +0x0024
        // Writes to field at offset +0x0028
        // Writes to field at offset +0x002c
    }

}

impl CcClass for ccMinTensionGauge {
    fn class_name(&self) -> &'static str { Self::CLASS_NAME }
    fn vtable_rva(&self) -> u32 { Self::VTABLE_RVA }
    fn virtual_method_count(&self) -> usize { Self::METHOD_COUNT }
    fn base_classes(&self) -> &'static [&'static str] { Self::BASE_CLASSES }
}

/// Decompiled CC2 class: `ccNamePriceListWnd` (VTable RVA: 0x00894dd0, 64 virtual methods, 38 recovered fields)
/// Inherits: ListWnd, MoveAnmPos
#[derive(Debug, Clone, Default)]
pub struct ccNamePriceListWnd {
    pub vtable_rva: u32,
    pub field_0x0000: u64,
    pub field_0x0008: f32,
    pub field_0x000c: f32,
    pub field_0x0018: u64,
    pub field_0x0020: u32,
    pub field_0x0024: u8,
    pub field_0x0026: u8,
    pub field_0x0028: u32,
    pub field_0x002c: u32,
    pub field_0x0030: u64,
    pub field_0x0060: u32,
    pub field_0x0064: u32,
    pub field_0x0068: f32,
    pub field_0x0070: u32,
    pub field_0x0074: u32,
    pub field_0x0078: f32,
    pub field_0x007c: f32,
    pub field_0x008c: u32,
    pub field_0x0094: u64,
    pub field_0x0098: u32,
    pub field_0x009c: u64,
    pub field_0x00a4: u32,
    pub field_0x00ac: f32,
    pub field_0x00b0: u32,
    pub field_0x00b4: u32,
    pub field_0x00c0: u32,
    pub field_0x00c8: u64,
    pub field_0x00d0: u64,
    pub field_0x00d8: u64,
    pub field_0x00f0: u64,
    pub field_0x00f8: u32,
    pub field_0x00fc: u32,
    pub field_0x0100: f32,
    pub field_0x010c: u8,
    pub field_0x010d: u8,
    pub field_0x0110: u32,
    pub field_0x0118: u64,
    pub field_0x0180: u64,
}

impl ccNamePriceListWnd {
    pub const CLASS_NAME: &'static str = "ccNamePriceListWnd";
    pub const VTABLE_RVA: u32 = 0x00894dd0;
    pub const METHOD_COUNT: usize = 64;
    pub const BASE_CLASSES: &'static [&'static str] = &["ListWnd", "MoveAnmPos"];
    pub const VIRTUAL_METHODS: &'static [u64] = &[0x0000000180109550, 0x0000000180109c20, 0x000000018010a1e0, 0x000000018010a3e0, 0x000000018057b520, 0x000000018057b450, 0x000000018010c440, 0x000000018057b5a0, 0x0000000180103f70, 0x000000018057b440, 0x000000018010cf30, 0x0000000180103f80, 0x0000000180109520, 0x00000001800d3090, 0x000000018010a4e0, 0x000000018010a100, 0x00000001800d3090, 0x00000001800d3090, 0x00000001800d3090, 0x000000018010a520, 0x0000000180265960, 0x00000001800fe200, 0x000000018010a590, 0x000000018010a670, 0x000000018010a680, 0x000000018010a8d0, 0x000000018010a910, 0x000000018010a690, 0x000000018010cfd0, 0x00000001800d3090, 0x00000001800d3090, 0x00000001800d3090, 0x00000001800d3090, 0x00000001800d3090, 0x00000001800fe200, 0x00000001800fe200, 0x00000001800fe200, 0x00000001800fe200, 0x00000001800fe200, 0x00000001800fe200, 0x00000001800fe200, 0x00000001800fe200, 0x00000001800fe200, 0x00000001800d3090, 0x00000001802656a0, 0x00000001802657c0, 0x0000000180109d40, 0x0000000180109e70, 0x0000000180109fa0, 0x000000018010a020, 0x000000018010a090, 0x000000018010a460, 0x000000018010b660, 0x000000018010bb20, 0x000000018010bf60, 0x000000018010c090, 0x000000018010c230, 0x000000018010c230, 0x000000018010c340, 0x000000018010c2d0, 0x000000018010b3b0, 0x000000018010b5b0, 0x000000018010ce40, 0x000000018010c510];

    pub fn new() -> Self {
        Self {
            vtable_rva: Self::VTABLE_RVA,
            field_0x0000: Default::default(),
            field_0x0008: Default::default(),
            field_0x000c: Default::default(),
            field_0x0018: Default::default(),
            field_0x0020: Default::default(),
            field_0x0024: Default::default(),
            field_0x0026: Default::default(),
            field_0x0028: Default::default(),
            field_0x002c: Default::default(),
            field_0x0030: Default::default(),
            field_0x0060: Default::default(),
            field_0x0064: Default::default(),
            field_0x0068: Default::default(),
            field_0x0070: Default::default(),
            field_0x0074: Default::default(),
            field_0x0078: Default::default(),
            field_0x007c: Default::default(),
            field_0x008c: Default::default(),
            field_0x0094: Default::default(),
            field_0x0098: Default::default(),
            field_0x009c: Default::default(),
            field_0x00a4: Default::default(),
            field_0x00ac: Default::default(),
            field_0x00b0: Default::default(),
            field_0x00b4: Default::default(),
            field_0x00c0: Default::default(),
            field_0x00c8: Default::default(),
            field_0x00d0: Default::default(),
            field_0x00d8: Default::default(),
            field_0x00f0: Default::default(),
            field_0x00f8: Default::default(),
            field_0x00fc: Default::default(),
            field_0x0100: Default::default(),
            field_0x010c: Default::default(),
            field_0x010d: Default::default(),
            field_0x0110: Default::default(),
            field_0x0118: Default::default(),
            field_0x0180: Default::default(),
        }
    }

    /// Decompiled virtual method #0 @ RVA 0x00109550 (46 instructions, 1 basic blocks)
    pub fn method_00(&mut self) {
    }

    /// Decompiled virtual method #1 @ RVA 0x00109c20 (130 instructions, 13 basic blocks)
    pub fn method_01(&mut self) {
        // Writes to field at offset +0x0118
    }

    /// Decompiled virtual method #2 @ RVA 0x0010a1e0 (112 instructions, 22 basic blocks)
    pub fn method_02(&mut self) {
        // Writes to field at offset +0x010c
        // Writes to field at offset +0x0118
    }

    /// Decompiled virtual method #3 @ RVA 0x0010a3e0 (30 instructions, 3 basic blocks)
    pub fn method_03(&mut self) {
        // Writes to field at offset +0x0118
    }

    /// Decompiled virtual method #4 @ RVA 0x0057b520 (30 instructions, 4 basic blocks)
    pub fn method_04(&mut self) {
    }

    /// Decompiled virtual method #5 @ RVA 0x0057b450 (90 instructions, 7 basic blocks)
    pub fn method_05(&mut self) {
    }

    /// Decompiled virtual method #6 @ RVA 0x0010c440 (24 instructions, 3 basic blocks)
    pub fn method_06(&mut self) {
    }

    /// Decompiled virtual method #7 @ RVA 0x0057b5a0 (7 instructions, 1 basic blocks)
    pub fn method_07(&mut self) {
        // Writes to field at offset +0x000c
        // Writes to field at offset +0x0026
    }

    /// Decompiled virtual method #8 @ RVA 0x00103f70 (2 instructions, 1 basic blocks)
    pub fn method_08(&mut self) {
        // Writes to field at offset +0x0020
    }

    /// Decompiled virtual method #9 @ RVA 0x0057b440 (98 instructions, 8 basic blocks)
    pub fn method_09(&mut self) {
        // Writes to field at offset +0x0008
    }

    /// Decompiled virtual method #10 @ RVA 0x0010cf30 (8 instructions, 3 basic blocks)
    pub fn method_10(&mut self) {
    }

    /// Decompiled virtual method #11 @ RVA 0x00103f80 (3 instructions, 1 basic blocks)
    pub fn method_11(&mut self) {
        // Writes to field at offset +0x0024
    }

    /// Decompiled virtual method #12 @ RVA 0x00109520 (7 instructions, 1 basic blocks)
    pub fn method_12(&mut self) {
        // Writes to field at offset +0x0094
        // Writes to field at offset +0x009c
        // Writes to field at offset +0x00a4
        // Writes to field at offset +0x00b4
        // Writes to field at offset +0x00c0
    }

    /// Decompiled virtual method #13 @ RVA 0x000d3090 (1 instructions, 1 basic blocks)
    pub fn method_13(&mut self) {
    }

    /// Decompiled virtual method #14 @ RVA 0x0010a4e0 (13 instructions, 3 basic blocks)
    pub fn method_14(&mut self) {
        // Writes to field at offset +0x010c
    }

    /// Decompiled virtual method #15 @ RVA 0x0010a100 (47 instructions, 10 basic blocks)
    pub fn method_15(&mut self) {
    }

    /// Decompiled virtual method #16 @ RVA 0x000d3090 (1 instructions, 1 basic blocks)
    pub fn method_16(&mut self) {
    }

    /// Decompiled virtual method #17 @ RVA 0x000d3090 (1 instructions, 1 basic blocks)
    pub fn method_17(&mut self) {
    }

    /// Decompiled virtual method #18 @ RVA 0x000d3090 (1 instructions, 1 basic blocks)
    pub fn method_18(&mut self) {
    }

    /// Decompiled virtual method #19 @ RVA 0x0010a520 (26 instructions, 4 basic blocks)
    pub fn method_19(&mut self) {
        // Writes to field at offset +0x010c
    }

    /// Decompiled virtual method #20 @ RVA 0x00265960 (63 instructions, 8 basic blocks)
    pub fn method_20(&mut self) {
        // Writes to field at offset +0x0000
    }

    /// Decompiled virtual method #21 @ RVA 0x000fe200 (2 instructions, 1 basic blocks)
    pub fn method_21(&mut self) {
    }

    /// Decompiled virtual method #22 @ RVA 0x0010a590 (52 instructions, 9 basic blocks)
    pub fn method_22(&mut self) {
        // Writes to field at offset +0x010c
    }

    /// Decompiled virtual method #23 @ RVA 0x0010a670 (113 instructions, 16 basic blocks)
    pub fn method_23(&mut self) {
        // Writes to field at offset +0x010d
    }

    /// Decompiled virtual method #24 @ RVA 0x0010a680 (110 instructions, 16 basic blocks)
    pub fn method_24(&mut self) {
        // Writes to field at offset +0x010d
    }

    /// Decompiled virtual method #25 @ RVA 0x0010a8d0 (117 instructions, 7 basic blocks)
    pub fn method_25(&mut self) {
        // Writes to field at offset +0x0110
    }

    /// Decompiled virtual method #26 @ RVA 0x0010a910 (112 instructions, 9 basic blocks)
    pub fn method_26(&mut self) {
    }

    /// Decompiled virtual method #27 @ RVA 0x0010a690 (107 instructions, 15 basic blocks)
    pub fn method_27(&mut self) {
        // Writes to field at offset +0x010d
    }

    /// Decompiled virtual method #28 @ RVA 0x0010cfd0 (25 instructions, 3 basic blocks)
    pub fn method_28(&mut self) {
        // Writes to field at offset +0x010c
    }

    /// Decompiled virtual method #29 @ RVA 0x000d3090 (1 instructions, 1 basic blocks)
    pub fn method_29(&mut self) {
    }

    /// Decompiled virtual method #30 @ RVA 0x000d3090 (1 instructions, 1 basic blocks)
    pub fn method_30(&mut self) {
    }

    /// Decompiled virtual method #31 @ RVA 0x000d3090 (1 instructions, 1 basic blocks)
    pub fn method_31(&mut self) {
    }

    /// Decompiled virtual method #32 @ RVA 0x000d3090 (1 instructions, 1 basic blocks)
    pub fn method_32(&mut self) {
    }

    /// Decompiled virtual method #33 @ RVA 0x000d3090 (1 instructions, 1 basic blocks)
    pub fn method_33(&mut self) {
    }

    /// Decompiled virtual method #34 @ RVA 0x000fe200 (2 instructions, 1 basic blocks)
    pub fn method_34(&mut self) {
    }

    /// Decompiled virtual method #35 @ RVA 0x000fe200 (2 instructions, 1 basic blocks)
    pub fn method_35(&mut self) {
    }

    /// Decompiled virtual method #36 @ RVA 0x000fe200 (2 instructions, 1 basic blocks)
    pub fn method_36(&mut self) {
    }

    /// Decompiled virtual method #37 @ RVA 0x000fe200 (2 instructions, 1 basic blocks)
    pub fn method_37(&mut self) {
    }

    /// Decompiled virtual method #38 @ RVA 0x000fe200 (2 instructions, 1 basic blocks)
    pub fn method_38(&mut self) {
    }

    /// Decompiled virtual method #39 @ RVA 0x000fe200 (2 instructions, 1 basic blocks)
    pub fn method_39(&mut self) {
    }

    /// Decompiled virtual method #40 @ RVA 0x000fe200 (2 instructions, 1 basic blocks)
    pub fn method_40(&mut self) {
    }

    /// Decompiled virtual method #41 @ RVA 0x000fe200 (2 instructions, 1 basic blocks)
    pub fn method_41(&mut self) {
    }

    /// Decompiled virtual method #42 @ RVA 0x000fe200 (2 instructions, 1 basic blocks)
    pub fn method_42(&mut self) {
    }

    /// Decompiled virtual method #43 @ RVA 0x000d3090 (1 instructions, 1 basic blocks)
    pub fn method_43(&mut self) {
    }

    /// Decompiled virtual method #44 @ RVA 0x002656a0 (16 instructions, 3 basic blocks)
    pub fn method_44(&mut self) {
    }

    /// Decompiled virtual method #45 @ RVA 0x002657c0 (24 instructions, 1 basic blocks)
    pub fn method_45(&mut self) {
    }

    /// Decompiled virtual method #46 @ RVA 0x00109d40 (67 instructions, 5 basic blocks)
    pub fn method_46(&mut self) {
        // Writes to field at offset +0x0118
    }

    /// Decompiled virtual method #47 @ RVA 0x00109e70 (64 instructions, 5 basic blocks)
    pub fn method_47(&mut self) {
        // Writes to field at offset +0x00f8
        // Writes to field at offset +0x00fc
        // Writes to field at offset +0x0100
        // Writes to field at offset +0x0110
        // Writes to field at offset +0x0118
    }

    /// Decompiled virtual method #48 @ RVA 0x00109fa0 (101 instructions, 7 basic blocks)
    pub fn method_48(&mut self) {
    }

    /// Decompiled virtual method #49 @ RVA 0x0010a020 (66 instructions, 5 basic blocks)
    pub fn method_49(&mut self) {
    }

    /// Decompiled virtual method #50 @ RVA 0x0010a090 (26 instructions, 3 basic blocks)
    pub fn method_50(&mut self) {
    }

    /// Decompiled virtual method #51 @ RVA 0x0010a460 (30 instructions, 3 basic blocks)
    pub fn method_51(&mut self) {
        // Writes to field at offset +0x0118
    }

    /// Decompiled virtual method #52 @ RVA 0x0010b660 (126 instructions, 20 basic blocks)
    pub fn method_52(&mut self) {
        // Writes to field at offset +0x009c
    }

    /// Decompiled virtual method #53 @ RVA 0x0010bb20 (131 instructions, 17 basic blocks)
    pub fn method_53(&mut self) {
    }

    /// Decompiled virtual method #54 @ RVA 0x0010bf60 (69 instructions, 11 basic blocks)
    pub fn method_54(&mut self) {
    }

    /// Decompiled virtual method #55 @ RVA 0x0010c090 (98 instructions, 13 basic blocks)
    pub fn method_55(&mut self) {
    }

    /// Decompiled virtual method #56 @ RVA 0x0010c230 (26 instructions, 2 basic blocks)
    pub fn method_56(&mut self) {
    }

    /// Decompiled virtual method #57 @ RVA 0x0010c230 (26 instructions, 2 basic blocks)
    pub fn method_57(&mut self) {
    }

    /// Decompiled virtual method #58 @ RVA 0x0010c340 (29 instructions, 3 basic blocks)
    pub fn method_58(&mut self) {
    }

    /// Decompiled virtual method #59 @ RVA 0x0010c2d0 (29 instructions, 3 basic blocks)
    pub fn method_59(&mut self) {
    }

    /// Decompiled virtual method #60 @ RVA 0x0010b3b0 (11 instructions, 1 basic blocks)
    pub fn method_60(&mut self) {
    }

    /// Decompiled virtual method #61 @ RVA 0x0010b5b0 (23 instructions, 2 basic blocks)
    pub fn method_61(&mut self) {
    }

    /// Decompiled virtual method #62 @ RVA 0x0010ce40 (59 instructions, 6 basic blocks)
    pub fn method_62(&mut self) {
    }

    /// Decompiled virtual method #63 @ RVA 0x0010c510 (52 instructions, 6 basic blocks)
    pub fn method_63(&mut self) {
        // Writes to field at offset +0x0030
        // Writes to field at offset +0x00d0
        // Writes to field at offset +0x00d8
        // Writes to field at offset +0x00f0
    }

}

impl CcClass for ccNamePriceListWnd {
    fn class_name(&self) -> &'static str { Self::CLASS_NAME }
    fn vtable_rva(&self) -> u32 { Self::VTABLE_RVA }
    fn virtual_method_count(&self) -> usize { Self::METHOD_COUNT }
    fn base_classes(&self) -> &'static [&'static str] { Self::BASE_CLASSES }
}

/// Decompiled CC2 class: `ccNumberDisp` (VTable RVA: 0x0087be48, 10 virtual methods, 19 recovered fields)
/// Inherits: ccNumber
#[derive(Debug, Clone, Default)]
pub struct ccNumberDisp {
    pub vtable_rva: u32,
    pub field_0x0000: u64,
    pub field_0x0008: u32,
    pub field_0x000c: f32,
    pub field_0x0010: f32,
    pub field_0x0014: f32,
    pub field_0x0018: f32,
    pub field_0x001c: f32,
    pub field_0x0020: u16,
    pub field_0x0022: u16,
    pub field_0x0024: u32,
    pub field_0x0028: u64,
    pub field_0x0030: u32,
    pub field_0x0034: u16,
    pub field_0x0036: u8,
    pub field_0x0038: u64,
    pub field_0x0040: u32,
    pub field_0x0044: f32,
    pub field_0x0048: u32,
    pub field_0x007c: u16,
}

impl ccNumberDisp {
    pub const CLASS_NAME: &'static str = "ccNumberDisp";
    pub const VTABLE_RVA: u32 = 0x0087be48;
    pub const METHOD_COUNT: usize = 10;
    pub const BASE_CLASSES: &'static [&'static str] = &["ccNumber"];
    pub const VIRTUAL_METHODS: &'static [u64] = &[0x00000001801b83f0, 0x00000001802a5dc0, 0x00000001802a5de0, 0x00000001802a5e40, 0x00000001802a5c70, 0x00000001802a5cf0, 0x00000001802a5d00, 0x0000000180114c60, 0x00000001802a6030, 0x00000001802a5c40];

    pub fn new() -> Self {
        Self {
            vtable_rva: Self::VTABLE_RVA,
            field_0x0000: Default::default(),
            field_0x0008: Default::default(),
            field_0x000c: Default::default(),
            field_0x0010: Default::default(),
            field_0x0014: Default::default(),
            field_0x0018: Default::default(),
            field_0x001c: Default::default(),
            field_0x0020: Default::default(),
            field_0x0022: Default::default(),
            field_0x0024: Default::default(),
            field_0x0028: Default::default(),
            field_0x0030: Default::default(),
            field_0x0034: Default::default(),
            field_0x0036: Default::default(),
            field_0x0038: Default::default(),
            field_0x0040: Default::default(),
            field_0x0044: Default::default(),
            field_0x0048: Default::default(),
            field_0x007c: Default::default(),
        }
    }

    /// Decompiled virtual method #0 @ RVA 0x001b83f0 (39 instructions, 7 basic blocks)
    pub fn method_00(&mut self) {
        // Writes to field at offset +0x0000
    }

    /// Decompiled virtual method #1 @ RVA 0x002a5dc0 (5 instructions, 1 basic blocks)
    pub fn method_01(&mut self) {
        // Writes to field at offset +0x0030
        // Writes to field at offset +0x0034
        // Writes to field at offset +0x0036
        // Writes to field at offset +0x0038
    }

    /// Decompiled virtual method #2 @ RVA 0x002a5de0 (26 instructions, 8 basic blocks)
    pub fn method_02(&mut self) {
    }

    /// Decompiled virtual method #3 @ RVA 0x002a5e40 (128 instructions, 18 basic blocks)
    pub fn method_03(&mut self) {
        // Writes to field at offset +0x0040
        // Writes to field at offset +0x0044
        // Writes to field at offset +0x0048
        // Writes to field at offset +0x007c
    }

    /// Decompiled virtual method #4 @ RVA 0x002a5c70 (36 instructions, 6 basic blocks)
    pub fn method_04(&mut self) {
        // Writes to field at offset +0x0014
        // Writes to field at offset +0x0018
    }

    /// Decompiled virtual method #5 @ RVA 0x002a5cf0 (3 instructions, 1 basic blocks)
    pub fn method_05(&mut self) {
        // Writes to field at offset +0x0014
        // Writes to field at offset +0x0018
    }

    /// Decompiled virtual method #6 @ RVA 0x002a5d00 (56 instructions, 12 basic blocks)
    pub fn method_06(&mut self) {
        // Writes to field at offset +0x001c
        // Writes to field at offset +0x0022
        // Writes to field at offset +0x0024
    }

    /// Decompiled virtual method #7 @ RVA 0x00114c60 (2 instructions, 1 basic blocks)
    pub fn method_07(&mut self) {
        // Writes to field at offset +0x0008
    }

    /// Decompiled virtual method #8 @ RVA 0x002a6030 (6 instructions, 1 basic blocks)
    pub fn method_08(&mut self) {
        // Writes to field at offset +0x000c
        // Writes to field at offset +0x0010
    }

    /// Decompiled virtual method #9 @ RVA 0x002a5c40 (57 instructions, 7 basic blocks)
    pub fn method_09(&mut self) {
        // Writes to field at offset +0x0014
        // Writes to field at offset +0x0018
    }

}

impl CcClass for ccNumberDisp {
    fn class_name(&self) -> &'static str { Self::CLASS_NAME }
    fn vtable_rva(&self) -> u32 { Self::VTABLE_RVA }
    fn virtual_method_count(&self) -> usize { Self::METHOD_COUNT }
    fn base_classes(&self) -> &'static [&'static str] { Self::BASE_CLASSES }
}

/// Decompiled CC2 class: `ccPartyWnd` (VTable RVA: 0x008bad78, 37 virtual methods, 17 recovered fields)
/// Inherits: ccGameMenuCtrlBase, ccGameMenuObj
#[derive(Debug, Clone, Default)]
pub struct ccPartyWnd {
    pub vtable_rva: u32,
    pub field_0x0000: u64,
    pub field_0x0008: u32,
    pub field_0x0018: u32,
    pub field_0x0048: u64,
    pub field_0x0050: u64,
    pub field_0x0060: u32,
    pub field_0x0070: u64,
    pub field_0x007c: u32,
    pub field_0x008c: f32,
    pub field_0x0094: u32,
    pub field_0x009c: u32,
    pub field_0x00f0: u8,
    pub field_0x00f8: u32,
    pub field_0x0100: u32,
    pub field_0x0148: u8,
    pub field_0x018e: u16,
    pub field_0x0d74: u32,
}

impl ccPartyWnd {
    pub const CLASS_NAME: &'static str = "ccPartyWnd";
    pub const VTABLE_RVA: u32 = 0x008bad78;
    pub const METHOD_COUNT: usize = 37;
    pub const BASE_CLASSES: &'static [&'static str] = &["ccGameMenuCtrlBase", "ccGameMenuObj"];
    pub const VIRTUAL_METHODS: &'static [u64] = &[0x0000000180450170, 0x00000001804520b0, 0x0000000180452130, 0x0000000180451fb0, 0x0000000180451ff0, 0x0000000180452030, 0x0000000180452070, 0x0000000180103e90, 0x00000001801035b0, 0x00000001805bde60, 0x000000018045bb30, 0x0000000180451d90, 0x00000001801e9580, 0x0000000180103e60, 0x0000000180103e60, 0x0000000180451dc0, 0x0000000180451e30, 0x0000000180451ea0, 0x0000000180451ee0, 0x0000000180451f40, 0x00000001805bd3d0, 0x00000001805bd6f0, 0x0000000180285e60, 0x0000000180103e60, 0x0000000180103e60, 0x0000000180103e70, 0x00000001805bdca0, 0x0000000180103e60, 0x00000001805bdaa0, 0x00000001805bdba0, 0x00000001805bd7c0, 0x00000001805bda40, 0x00000001800d3090, 0x0000000180103e70, 0x00000001805bdcb0, 0x00000001805bdce0, 0x0000000180103e60];

    pub fn new() -> Self {
        Self {
            vtable_rva: Self::VTABLE_RVA,
            field_0x0000: Default::default(),
            field_0x0008: Default::default(),
            field_0x0018: Default::default(),
            field_0x0048: Default::default(),
            field_0x0050: Default::default(),
            field_0x0060: Default::default(),
            field_0x0070: Default::default(),
            field_0x007c: Default::default(),
            field_0x008c: Default::default(),
            field_0x0094: Default::default(),
            field_0x009c: Default::default(),
            field_0x00f0: Default::default(),
            field_0x00f8: Default::default(),
            field_0x0100: Default::default(),
            field_0x0148: Default::default(),
            field_0x018e: Default::default(),
            field_0x0d74: Default::default(),
        }
    }

    /// Decompiled virtual method #0 @ RVA 0x00450170 (16 instructions, 3 basic blocks)
    pub fn method_00(&mut self) {
    }

    /// Decompiled virtual method #1 @ RVA 0x004520b0 (10 instructions, 2 basic blocks)
    pub fn method_01(&mut self) {
    }

    /// Decompiled virtual method #2 @ RVA 0x00452130 (35 instructions, 9 basic blocks)
    pub fn method_02(&mut self) {
    }

    /// Decompiled virtual method #3 @ RVA 0x00451fb0 (118 instructions, 2 basic blocks)
    pub fn method_03(&mut self) {
    }

    /// Decompiled virtual method #4 @ RVA 0x00451ff0 (91 instructions, 2 basic blocks)
    pub fn method_04(&mut self) {
    }

    /// Decompiled virtual method #5 @ RVA 0x00452030 (64 instructions, 2 basic blocks)
    pub fn method_05(&mut self) {
    }

    /// Decompiled virtual method #6 @ RVA 0x00452070 (37 instructions, 2 basic blocks)
    pub fn method_06(&mut self) {
    }

    /// Decompiled virtual method #7 @ RVA 0x00103e90 (2 instructions, 1 basic blocks)
    pub fn method_07(&mut self) {
    }

    /// Decompiled virtual method #8 @ RVA 0x001035b0 (5 instructions, 1 basic blocks)
    pub fn method_08(&mut self) {
    }

    /// Decompiled virtual method #9 @ RVA 0x005bde60 (72 instructions, 19 basic blocks)
    pub fn method_09(&mut self) {
    }

    /// Decompiled virtual method #10 @ RVA 0x0045bb30 (16 instructions, 2 basic blocks)
    pub fn method_10(&mut self) {
    }

    /// Decompiled virtual method #11 @ RVA 0x00451d90 (24 instructions, 2 basic blocks)
    pub fn method_11(&mut self) {
    }

    /// Decompiled virtual method #12 @ RVA 0x001e9580 (53 instructions, 12 basic blocks)
    pub fn method_12(&mut self) {
    }

    /// Decompiled virtual method #13 @ RVA 0x00103e60 (2 instructions, 1 basic blocks)
    pub fn method_13(&mut self) {
    }

    /// Decompiled virtual method #14 @ RVA 0x00103e60 (2 instructions, 1 basic blocks)
    pub fn method_14(&mut self) {
    }

    /// Decompiled virtual method #15 @ RVA 0x00451dc0 (10 instructions, 2 basic blocks)
    pub fn method_15(&mut self) {
    }

    /// Decompiled virtual method #16 @ RVA 0x00451e30 (10 instructions, 2 basic blocks)
    pub fn method_16(&mut self) {
    }

    /// Decompiled virtual method #17 @ RVA 0x00451ea0 (31 instructions, 2 basic blocks)
    pub fn method_17(&mut self) {
        // Writes to field at offset +0x0018
    }

    /// Decompiled virtual method #18 @ RVA 0x00451ee0 (10 instructions, 2 basic blocks)
    pub fn method_18(&mut self) {
    }

    /// Decompiled virtual method #19 @ RVA 0x00451f40 (10 instructions, 2 basic blocks)
    pub fn method_19(&mut self) {
    }

    /// Decompiled virtual method #20 @ RVA 0x005bd3d0 (109 instructions, 6 basic blocks)
    pub fn method_20(&mut self) {
        // Writes to field at offset +0x0060
        // Writes to field at offset +0x00f0
    }

    /// Decompiled virtual method #21 @ RVA 0x005bd6f0 (40 instructions, 8 basic blocks)
    pub fn method_21(&mut self) {
        // Writes to field at offset +0x0000
    }

    /// Decompiled virtual method #22 @ RVA 0x00285e60 (15 instructions, 1 basic blocks)
    pub fn method_22(&mut self) {
    }

    /// Decompiled virtual method #23 @ RVA 0x00103e60 (2 instructions, 1 basic blocks)
    pub fn method_23(&mut self) {
    }

    /// Decompiled virtual method #24 @ RVA 0x00103e60 (2 instructions, 1 basic blocks)
    pub fn method_24(&mut self) {
    }

    /// Decompiled virtual method #25 @ RVA 0x00103e70 (2 instructions, 1 basic blocks)
    pub fn method_25(&mut self) {
    }

    /// Decompiled virtual method #26 @ RVA 0x005bdca0 (4 instructions, 1 basic blocks)
    pub fn method_26(&mut self) {
    }

    /// Decompiled virtual method #27 @ RVA 0x00103e60 (2 instructions, 1 basic blocks)
    pub fn method_27(&mut self) {
    }

    /// Decompiled virtual method #28 @ RVA 0x005bdaa0 (58 instructions, 4 basic blocks)
    pub fn method_28(&mut self) {
        // Writes to field at offset +0x0148
    }

    /// Decompiled virtual method #29 @ RVA 0x005bdba0 (58 instructions, 4 basic blocks)
    pub fn method_29(&mut self) {
        // Writes to field at offset +0x0148
    }

    /// Decompiled virtual method #30 @ RVA 0x005bd7c0 (118 instructions, 19 basic blocks)
    pub fn method_30(&mut self) {
        // Writes to field at offset +0x0148
    }

    /// Decompiled virtual method #31 @ RVA 0x005bda40 (85 instructions, 5 basic blocks)
    pub fn method_31(&mut self) {
        // Writes to field at offset +0x0148
    }

    /// Decompiled virtual method #32 @ RVA 0x000d3090 (1 instructions, 1 basic blocks)
    pub fn method_32(&mut self) {
    }

    /// Decompiled virtual method #33 @ RVA 0x00103e70 (2 instructions, 1 basic blocks)
    pub fn method_33(&mut self) {
    }

    /// Decompiled virtual method #34 @ RVA 0x005bdcb0 (9 instructions, 1 basic blocks)
    pub fn method_34(&mut self) {
        // Writes to field at offset +0x0100
    }

    /// Decompiled virtual method #35 @ RVA 0x005bdce0 (40 instructions, 6 basic blocks)
    pub fn method_35(&mut self) {
        // Writes to field at offset +0x0018
        // Writes to field at offset +0x0d74
    }

    /// Decompiled virtual method #36 @ RVA 0x00103e60 (2 instructions, 1 basic blocks)
    pub fn method_36(&mut self) {
    }

}

impl CcClass for ccPartyWnd {
    fn class_name(&self) -> &'static str { Self::CLASS_NAME }
    fn vtable_rva(&self) -> u32 { Self::VTABLE_RVA }
    fn virtual_method_count(&self) -> usize { Self::METHOD_COUNT }
    fn base_classes(&self) -> &'static [&'static str] { Self::BASE_CLASSES }
}

/// Decompiled CC2 class: `ccPlatRecoverMenuCtrl` (VTable RVA: 0x008e7200, 37 virtual methods, 8 recovered fields)
/// Inherits: ccGameMenuCtrlBase, ccGameMenuObj
#[derive(Debug, Clone, Default)]
pub struct ccPlatRecoverMenuCtrl {
    pub vtable_rva: u32,
    pub field_0x0000: u64,
    pub field_0x0008: u32,
    pub field_0x0018: u32,
    pub field_0x0048: u64,
    pub field_0x0050: u64,
    pub field_0x0060: u64,
    pub field_0x0064: f32,
    pub field_0x009c: u32,
}

impl ccPlatRecoverMenuCtrl {
    pub const CLASS_NAME: &'static str = "ccPlatRecoverMenuCtrl";
    pub const VTABLE_RVA: u32 = 0x008e7200;
    pub const METHOD_COUNT: usize = 37;
    pub const BASE_CLASSES: &'static [&'static str] = &["ccGameMenuCtrlBase", "ccGameMenuObj"];
    pub const VIRTUAL_METHODS: &'static [u64] = &[0x00000001805dcdd0, 0x00000001804520b0, 0x0000000180452130, 0x0000000180451fb0, 0x0000000180451ff0, 0x0000000180452030, 0x0000000180452070, 0x0000000180103e90, 0x0000000180103ea0, 0x0000000180103e60, 0x000000018045bb30, 0x0000000180451d90, 0x00000001801e9580, 0x00000001805dcea0, 0x0000000180103e60, 0x0000000180451dc0, 0x0000000180451e30, 0x0000000180451ea0, 0x0000000180451ee0, 0x0000000180451f40, 0x00000001805dcf00, 0x00000001805dd1a0, 0x0000000180103e60, 0x0000000180103e60, 0x0000000180103e60, 0x0000000180103e70, 0x0000000180103e70, 0x0000000180103e60, 0x0000000180103e60, 0x0000000180103e60, 0x00000001805dd220, 0x00000001800d3090, 0x00000001800d3090, 0x0000000180103e70, 0x0000000180103e70, 0x0000000180103e70, 0x0000000180103e60];

    pub fn new() -> Self {
        Self {
            vtable_rva: Self::VTABLE_RVA,
            field_0x0000: Default::default(),
            field_0x0008: Default::default(),
            field_0x0018: Default::default(),
            field_0x0048: Default::default(),
            field_0x0050: Default::default(),
            field_0x0060: Default::default(),
            field_0x0064: Default::default(),
            field_0x009c: Default::default(),
        }
    }

    /// Decompiled virtual method #0 @ RVA 0x005dcdd0 (16 instructions, 3 basic blocks)
    pub fn method_00(&mut self) {
    }

    /// Decompiled virtual method #1 @ RVA 0x004520b0 (10 instructions, 2 basic blocks)
    pub fn method_01(&mut self) {
    }

    /// Decompiled virtual method #2 @ RVA 0x00452130 (35 instructions, 9 basic blocks)
    pub fn method_02(&mut self) {
    }

    /// Decompiled virtual method #3 @ RVA 0x00451fb0 (118 instructions, 2 basic blocks)
    pub fn method_03(&mut self) {
    }

    /// Decompiled virtual method #4 @ RVA 0x00451ff0 (91 instructions, 2 basic blocks)
    pub fn method_04(&mut self) {
    }

    /// Decompiled virtual method #5 @ RVA 0x00452030 (64 instructions, 2 basic blocks)
    pub fn method_05(&mut self) {
    }

    /// Decompiled virtual method #6 @ RVA 0x00452070 (37 instructions, 2 basic blocks)
    pub fn method_06(&mut self) {
    }

    /// Decompiled virtual method #7 @ RVA 0x00103e90 (2 instructions, 1 basic blocks)
    pub fn method_07(&mut self) {
    }

    /// Decompiled virtual method #8 @ RVA 0x00103ea0 (9 instructions, 1 basic blocks)
    pub fn method_08(&mut self) {
        // Writes to field at offset +0x0008
    }

    /// Decompiled virtual method #9 @ RVA 0x00103e60 (2 instructions, 1 basic blocks)
    pub fn method_09(&mut self) {
    }

    /// Decompiled virtual method #10 @ RVA 0x0045bb30 (16 instructions, 2 basic blocks)
    pub fn method_10(&mut self) {
    }

    /// Decompiled virtual method #11 @ RVA 0x00451d90 (24 instructions, 2 basic blocks)
    pub fn method_11(&mut self) {
    }

    /// Decompiled virtual method #12 @ RVA 0x001e9580 (53 instructions, 12 basic blocks)
    pub fn method_12(&mut self) {
    }

    /// Decompiled virtual method #13 @ RVA 0x005dcea0 (24 instructions, 4 basic blocks)
    pub fn method_13(&mut self) {
    }

    /// Decompiled virtual method #14 @ RVA 0x00103e60 (2 instructions, 1 basic blocks)
    pub fn method_14(&mut self) {
    }

    /// Decompiled virtual method #15 @ RVA 0x00451dc0 (10 instructions, 2 basic blocks)
    pub fn method_15(&mut self) {
    }

    /// Decompiled virtual method #16 @ RVA 0x00451e30 (10 instructions, 2 basic blocks)
    pub fn method_16(&mut self) {
    }

    /// Decompiled virtual method #17 @ RVA 0x00451ea0 (31 instructions, 2 basic blocks)
    pub fn method_17(&mut self) {
        // Writes to field at offset +0x0018
    }

    /// Decompiled virtual method #18 @ RVA 0x00451ee0 (10 instructions, 2 basic blocks)
    pub fn method_18(&mut self) {
    }

    /// Decompiled virtual method #19 @ RVA 0x00451f40 (10 instructions, 2 basic blocks)
    pub fn method_19(&mut self) {
    }

    /// Decompiled virtual method #20 @ RVA 0x005dcf00 (114 instructions, 17 basic blocks)
    pub fn method_20(&mut self) {
        // Writes to field at offset +0x0060
    }

    /// Decompiled virtual method #21 @ RVA 0x005dd1a0 (29 instructions, 7 basic blocks)
    pub fn method_21(&mut self) {
    }

    /// Decompiled virtual method #22 @ RVA 0x00103e60 (2 instructions, 1 basic blocks)
    pub fn method_22(&mut self) {
    }

    /// Decompiled virtual method #23 @ RVA 0x00103e60 (2 instructions, 1 basic blocks)
    pub fn method_23(&mut self) {
    }

    /// Decompiled virtual method #24 @ RVA 0x00103e60 (2 instructions, 1 basic blocks)
    pub fn method_24(&mut self) {
    }

    /// Decompiled virtual method #25 @ RVA 0x00103e70 (2 instructions, 1 basic blocks)
    pub fn method_25(&mut self) {
    }

    /// Decompiled virtual method #26 @ RVA 0x00103e70 (2 instructions, 1 basic blocks)
    pub fn method_26(&mut self) {
    }

    /// Decompiled virtual method #27 @ RVA 0x00103e60 (2 instructions, 1 basic blocks)
    pub fn method_27(&mut self) {
    }

    /// Decompiled virtual method #28 @ RVA 0x00103e60 (2 instructions, 1 basic blocks)
    pub fn method_28(&mut self) {
    }

    /// Decompiled virtual method #29 @ RVA 0x00103e60 (2 instructions, 1 basic blocks)
    pub fn method_29(&mut self) {
    }

    /// Decompiled virtual method #30 @ RVA 0x005dd220 (18 instructions, 5 basic blocks)
    pub fn method_30(&mut self) {
        // Writes to field at offset +0x0018
        // Writes to field at offset +0x0064
        // Writes to field at offset +0x0064
    }

    /// Decompiled virtual method #31 @ RVA 0x000d3090 (1 instructions, 1 basic blocks)
    pub fn method_31(&mut self) {
    }

    /// Decompiled virtual method #32 @ RVA 0x000d3090 (1 instructions, 1 basic blocks)
    pub fn method_32(&mut self) {
    }

    /// Decompiled virtual method #33 @ RVA 0x00103e70 (2 instructions, 1 basic blocks)
    pub fn method_33(&mut self) {
    }

    /// Decompiled virtual method #34 @ RVA 0x00103e70 (2 instructions, 1 basic blocks)
    pub fn method_34(&mut self) {
    }

    /// Decompiled virtual method #35 @ RVA 0x00103e70 (2 instructions, 1 basic blocks)
    pub fn method_35(&mut self) {
    }

    /// Decompiled virtual method #36 @ RVA 0x00103e60 (2 instructions, 1 basic blocks)
    pub fn method_36(&mut self) {
    }

}

impl CcClass for ccPlatRecoverMenuCtrl {
    fn class_name(&self) -> &'static str { Self::CLASS_NAME }
    fn vtable_rva(&self) -> u32 { Self::VTABLE_RVA }
    fn virtual_method_count(&self) -> usize { Self::METHOD_COUNT }
    fn base_classes(&self) -> &'static [&'static str] { Self::BASE_CLASSES }
}

/// Decompiled CC2 class: `ccPlatTransportMenu` (VTable RVA: 0x008e7330, 37 virtual methods, 8 recovered fields)
/// Inherits: ccGameMenuCtrlBase, ccGameMenuObj
#[derive(Debug, Clone, Default)]
pub struct ccPlatTransportMenu {
    pub vtable_rva: u32,
    pub field_0x0000: u64,
    pub field_0x0008: u32,
    pub field_0x0018: u32,
    pub field_0x0048: u64,
    pub field_0x0050: u64,
    pub field_0x0060: u32,
    pub field_0x009c: u32,
    pub field_0x0298: u64,
}

impl ccPlatTransportMenu {
    pub const CLASS_NAME: &'static str = "ccPlatTransportMenu";
    pub const VTABLE_RVA: u32 = 0x008e7330;
    pub const METHOD_COUNT: usize = 37;
    pub const BASE_CLASSES: &'static [&'static str] = &["ccGameMenuCtrlBase", "ccGameMenuObj"];
    pub const VIRTUAL_METHODS: &'static [u64] = &[0x00000001805dbed0, 0x00000001804520b0, 0x0000000180452130, 0x0000000180451fb0, 0x0000000180451ff0, 0x0000000180452030, 0x0000000180452070, 0x0000000180103e90, 0x0000000180103ea0, 0x0000000180103e60, 0x000000018045bb30, 0x0000000180451d90, 0x00000001801e9580, 0x00000001805dc010, 0x0000000180103e60, 0x0000000180451dc0, 0x0000000180451e30, 0x0000000180451ea0, 0x0000000180451ee0, 0x0000000180451f40, 0x00000001805dc300, 0x00000001805dc750, 0x00000001801ee9b0, 0x0000000180103e60, 0x0000000180103e60, 0x0000000180103e70, 0x0000000180103e70, 0x0000000180103e60, 0x0000000180103e60, 0x0000000180103e60, 0x00000001805dc7f0, 0x00000001805dc9e0, 0x00000001800d3090, 0x0000000180103e70, 0x0000000180103e70, 0x0000000180103e70, 0x0000000180103e60];

    pub fn new() -> Self {
        Self {
            vtable_rva: Self::VTABLE_RVA,
            field_0x0000: Default::default(),
            field_0x0008: Default::default(),
            field_0x0018: Default::default(),
            field_0x0048: Default::default(),
            field_0x0050: Default::default(),
            field_0x0060: Default::default(),
            field_0x009c: Default::default(),
            field_0x0298: Default::default(),
        }
    }

    /// Decompiled virtual method #0 @ RVA 0x005dbed0 (16 instructions, 3 basic blocks)
    pub fn method_00(&mut self) {
    }

    /// Decompiled virtual method #1 @ RVA 0x004520b0 (10 instructions, 2 basic blocks)
    pub fn method_01(&mut self) {
    }

    /// Decompiled virtual method #2 @ RVA 0x00452130 (35 instructions, 9 basic blocks)
    pub fn method_02(&mut self) {
    }

    /// Decompiled virtual method #3 @ RVA 0x00451fb0 (118 instructions, 2 basic blocks)
    pub fn method_03(&mut self) {
    }

    /// Decompiled virtual method #4 @ RVA 0x00451ff0 (91 instructions, 2 basic blocks)
    pub fn method_04(&mut self) {
    }

    /// Decompiled virtual method #5 @ RVA 0x00452030 (64 instructions, 2 basic blocks)
    pub fn method_05(&mut self) {
    }

    /// Decompiled virtual method #6 @ RVA 0x00452070 (37 instructions, 2 basic blocks)
    pub fn method_06(&mut self) {
    }

    /// Decompiled virtual method #7 @ RVA 0x00103e90 (2 instructions, 1 basic blocks)
    pub fn method_07(&mut self) {
    }

    /// Decompiled virtual method #8 @ RVA 0x00103ea0 (9 instructions, 1 basic blocks)
    pub fn method_08(&mut self) {
        // Writes to field at offset +0x0008
    }

    /// Decompiled virtual method #9 @ RVA 0x00103e60 (2 instructions, 1 basic blocks)
    pub fn method_09(&mut self) {
    }

    /// Decompiled virtual method #10 @ RVA 0x0045bb30 (16 instructions, 2 basic blocks)
    pub fn method_10(&mut self) {
    }

    /// Decompiled virtual method #11 @ RVA 0x00451d90 (24 instructions, 2 basic blocks)
    pub fn method_11(&mut self) {
    }

    /// Decompiled virtual method #12 @ RVA 0x001e9580 (53 instructions, 12 basic blocks)
    pub fn method_12(&mut self) {
    }

    /// Decompiled virtual method #13 @ RVA 0x005dc010 (96 instructions, 6 basic blocks)
    pub fn method_13(&mut self) {
    }

    /// Decompiled virtual method #14 @ RVA 0x00103e60 (2 instructions, 1 basic blocks)
    pub fn method_14(&mut self) {
    }

    /// Decompiled virtual method #15 @ RVA 0x00451dc0 (10 instructions, 2 basic blocks)
    pub fn method_15(&mut self) {
    }

    /// Decompiled virtual method #16 @ RVA 0x00451e30 (10 instructions, 2 basic blocks)
    pub fn method_16(&mut self) {
    }

    /// Decompiled virtual method #17 @ RVA 0x00451ea0 (31 instructions, 2 basic blocks)
    pub fn method_17(&mut self) {
        // Writes to field at offset +0x0018
    }

    /// Decompiled virtual method #18 @ RVA 0x00451ee0 (10 instructions, 2 basic blocks)
    pub fn method_18(&mut self) {
    }

    /// Decompiled virtual method #19 @ RVA 0x00451f40 (10 instructions, 2 basic blocks)
    pub fn method_19(&mut self) {
    }

    /// Decompiled virtual method #20 @ RVA 0x005dc300 (118 instructions, 19 basic blocks)
    pub fn method_20(&mut self) {
        // Writes to field at offset +0x0298
    }

    /// Decompiled virtual method #21 @ RVA 0x005dc750 (35 instructions, 1 basic blocks)
    pub fn method_21(&mut self) {
    }

    /// Decompiled virtual method #22 @ RVA 0x001ee9b0 (15 instructions, 1 basic blocks)
    pub fn method_22(&mut self) {
    }

    /// Decompiled virtual method #23 @ RVA 0x00103e60 (2 instructions, 1 basic blocks)
    pub fn method_23(&mut self) {
    }

    /// Decompiled virtual method #24 @ RVA 0x00103e60 (2 instructions, 1 basic blocks)
    pub fn method_24(&mut self) {
    }

    /// Decompiled virtual method #25 @ RVA 0x00103e70 (2 instructions, 1 basic blocks)
    pub fn method_25(&mut self) {
    }

    /// Decompiled virtual method #26 @ RVA 0x00103e70 (2 instructions, 1 basic blocks)
    pub fn method_26(&mut self) {
    }

    /// Decompiled virtual method #27 @ RVA 0x00103e60 (2 instructions, 1 basic blocks)
    pub fn method_27(&mut self) {
    }

    /// Decompiled virtual method #28 @ RVA 0x00103e60 (2 instructions, 1 basic blocks)
    pub fn method_28(&mut self) {
    }

    /// Decompiled virtual method #29 @ RVA 0x00103e60 (2 instructions, 1 basic blocks)
    pub fn method_29(&mut self) {
    }

    /// Decompiled virtual method #30 @ RVA 0x005dc7f0 (94 instructions, 16 basic blocks)
    pub fn method_30(&mut self) {
    }

    /// Decompiled virtual method #31 @ RVA 0x005dc9e0 (76 instructions, 11 basic blocks)
    pub fn method_31(&mut self) {
        // Writes to field at offset +0x0298
    }

    /// Decompiled virtual method #32 @ RVA 0x000d3090 (1 instructions, 1 basic blocks)
    pub fn method_32(&mut self) {
    }

    /// Decompiled virtual method #33 @ RVA 0x00103e70 (2 instructions, 1 basic blocks)
    pub fn method_33(&mut self) {
    }

    /// Decompiled virtual method #34 @ RVA 0x00103e70 (2 instructions, 1 basic blocks)
    pub fn method_34(&mut self) {
    }

    /// Decompiled virtual method #35 @ RVA 0x00103e70 (2 instructions, 1 basic blocks)
    pub fn method_35(&mut self) {
    }

    /// Decompiled virtual method #36 @ RVA 0x00103e60 (2 instructions, 1 basic blocks)
    pub fn method_36(&mut self) {
    }

}

impl CcClass for ccPlatTransportMenu {
    fn class_name(&self) -> &'static str { Self::CLASS_NAME }
    fn vtable_rva(&self) -> u32 { Self::VTABLE_RVA }
    fn virtual_method_count(&self) -> usize { Self::METHOD_COUNT }
    fn base_classes(&self) -> &'static [&'static str] { Self::BASE_CLASSES }
}

/// Decompiled CC2 class: `ccRecollectMenuCtrl` (VTable RVA: 0x008e8d00, 37 virtual methods, 12 recovered fields)
/// Inherits: ccGameMenuCtrlBase, ccGameMenuObj
#[derive(Debug, Clone, Default)]
pub struct ccRecollectMenuCtrl {
    pub vtable_rva: u32,
    pub field_0x0000: u64,
    pub field_0x0004: u32,
    pub field_0x0008: u32,
    pub field_0x000c: u8,
    pub field_0x0010: u64,
    pub field_0x0018: u32,
    pub field_0x0020: u64,
    pub field_0x0048: u64,
    pub field_0x0050: u64,
    pub field_0x0068: u64,
    pub field_0x0070: u64,
    pub field_0x009c: u32,
}

impl ccRecollectMenuCtrl {
    pub const CLASS_NAME: &'static str = "ccRecollectMenuCtrl";
    pub const VTABLE_RVA: u32 = 0x008e8d00;
    pub const METHOD_COUNT: usize = 37;
    pub const BASE_CLASSES: &'static [&'static str] = &["ccGameMenuCtrlBase", "ccGameMenuObj"];
    pub const VIRTUAL_METHODS: &'static [u64] = &[0x00000001805fc7d0, 0x00000001804520b0, 0x0000000180452130, 0x0000000180451fb0, 0x0000000180103e80, 0x0000000180452030, 0x0000000180452070, 0x0000000180103e90, 0x0000000180103ea0, 0x0000000180103e60, 0x000000018045bb30, 0x00000001805fc920, 0x00000001801e9580, 0x0000000180103e60, 0x0000000180103e60, 0x0000000180451dc0, 0x0000000180451e30, 0x0000000180451ea0, 0x0000000180451ee0, 0x0000000180451f40, 0x00000001805fcab0, 0x00000001805fcc80, 0x0000000180103e70, 0x0000000180103e60, 0x0000000180103e60, 0x0000000180103e70, 0x0000000180103e70, 0x0000000180103e60, 0x0000000180103e60, 0x0000000180103e60, 0x00000001805fcdc0, 0x00000001805fcf20, 0x00000001800d3090, 0x0000000180103e70, 0x0000000180103e70, 0x0000000180103e80, 0x0000000180103e60];

    pub fn new() -> Self {
        Self {
            vtable_rva: Self::VTABLE_RVA,
            field_0x0000: Default::default(),
            field_0x0004: Default::default(),
            field_0x0008: Default::default(),
            field_0x000c: Default::default(),
            field_0x0010: Default::default(),
            field_0x0018: Default::default(),
            field_0x0020: Default::default(),
            field_0x0048: Default::default(),
            field_0x0050: Default::default(),
            field_0x0068: Default::default(),
            field_0x0070: Default::default(),
            field_0x009c: Default::default(),
        }
    }

    /// Decompiled virtual method #0 @ RVA 0x005fc7d0 (16 instructions, 3 basic blocks)
    pub fn method_00(&mut self) {
    }

    /// Decompiled virtual method #1 @ RVA 0x004520b0 (10 instructions, 2 basic blocks)
    pub fn method_01(&mut self) {
    }

    /// Decompiled virtual method #2 @ RVA 0x00452130 (35 instructions, 9 basic blocks)
    pub fn method_02(&mut self) {
    }

    /// Decompiled virtual method #3 @ RVA 0x00451fb0 (118 instructions, 2 basic blocks)
    pub fn method_03(&mut self) {
    }

    /// Decompiled virtual method #4 @ RVA 0x00103e80 (2 instructions, 1 basic blocks)
    pub fn method_04(&mut self) {
    }

    /// Decompiled virtual method #5 @ RVA 0x00452030 (64 instructions, 2 basic blocks)
    pub fn method_05(&mut self) {
    }

    /// Decompiled virtual method #6 @ RVA 0x00452070 (37 instructions, 2 basic blocks)
    pub fn method_06(&mut self) {
    }

    /// Decompiled virtual method #7 @ RVA 0x00103e90 (2 instructions, 1 basic blocks)
    pub fn method_07(&mut self) {
    }

    /// Decompiled virtual method #8 @ RVA 0x00103ea0 (9 instructions, 1 basic blocks)
    pub fn method_08(&mut self) {
        // Writes to field at offset +0x0008
    }

    /// Decompiled virtual method #9 @ RVA 0x00103e60 (2 instructions, 1 basic blocks)
    pub fn method_09(&mut self) {
    }

    /// Decompiled virtual method #10 @ RVA 0x0045bb30 (16 instructions, 2 basic blocks)
    pub fn method_10(&mut self) {
    }

    /// Decompiled virtual method #11 @ RVA 0x005fc920 (95 instructions, 12 basic blocks)
    pub fn method_11(&mut self) {
    }

    /// Decompiled virtual method #12 @ RVA 0x001e9580 (53 instructions, 12 basic blocks)
    pub fn method_12(&mut self) {
    }

    /// Decompiled virtual method #13 @ RVA 0x00103e60 (2 instructions, 1 basic blocks)
    pub fn method_13(&mut self) {
    }

    /// Decompiled virtual method #14 @ RVA 0x00103e60 (2 instructions, 1 basic blocks)
    pub fn method_14(&mut self) {
    }

    /// Decompiled virtual method #15 @ RVA 0x00451dc0 (10 instructions, 2 basic blocks)
    pub fn method_15(&mut self) {
    }

    /// Decompiled virtual method #16 @ RVA 0x00451e30 (10 instructions, 2 basic blocks)
    pub fn method_16(&mut self) {
    }

    /// Decompiled virtual method #17 @ RVA 0x00451ea0 (31 instructions, 2 basic blocks)
    pub fn method_17(&mut self) {
        // Writes to field at offset +0x0018
    }

    /// Decompiled virtual method #18 @ RVA 0x00451ee0 (10 instructions, 2 basic blocks)
    pub fn method_18(&mut self) {
    }

    /// Decompiled virtual method #19 @ RVA 0x00451f40 (10 instructions, 2 basic blocks)
    pub fn method_19(&mut self) {
    }

    /// Decompiled virtual method #20 @ RVA 0x005fcab0 (114 instructions, 21 basic blocks)
    pub fn method_20(&mut self) {
        // Writes to field at offset +0x0000
        // Writes to field at offset +0x0000
        // Writes to field at offset +0x0000
        // Writes to field at offset +0x0004
        // Writes to field at offset +0x0008
        // Writes to field at offset +0x000c
        // Writes to field at offset +0x0010
        // Writes to field at offset +0x0018
    }

    /// Decompiled virtual method #21 @ RVA 0x005fcc80 (79 instructions, 15 basic blocks)
    pub fn method_21(&mut self) {
        // Writes to field at offset +0x0000
        // Writes to field at offset +0x0008
        // Writes to field at offset +0x0010
        // Writes to field at offset +0x0018
    }

    /// Decompiled virtual method #22 @ RVA 0x00103e70 (2 instructions, 1 basic blocks)
    pub fn method_22(&mut self) {
    }

    /// Decompiled virtual method #23 @ RVA 0x00103e60 (2 instructions, 1 basic blocks)
    pub fn method_23(&mut self) {
    }

    /// Decompiled virtual method #24 @ RVA 0x00103e60 (2 instructions, 1 basic blocks)
    pub fn method_24(&mut self) {
    }

    /// Decompiled virtual method #25 @ RVA 0x00103e70 (2 instructions, 1 basic blocks)
    pub fn method_25(&mut self) {
    }

    /// Decompiled virtual method #26 @ RVA 0x00103e70 (2 instructions, 1 basic blocks)
    pub fn method_26(&mut self) {
    }

    /// Decompiled virtual method #27 @ RVA 0x00103e60 (2 instructions, 1 basic blocks)
    pub fn method_27(&mut self) {
    }

    /// Decompiled virtual method #28 @ RVA 0x00103e60 (2 instructions, 1 basic blocks)
    pub fn method_28(&mut self) {
    }

    /// Decompiled virtual method #29 @ RVA 0x00103e60 (2 instructions, 1 basic blocks)
    pub fn method_29(&mut self) {
    }

    /// Decompiled virtual method #30 @ RVA 0x005fcdc0 (49 instructions, 5 basic blocks)
    pub fn method_30(&mut self) {
    }

    /// Decompiled virtual method #31 @ RVA 0x005fcf20 (127 instructions, 14 basic blocks)
    pub fn method_31(&mut self) {
    }

    /// Decompiled virtual method #32 @ RVA 0x000d3090 (1 instructions, 1 basic blocks)
    pub fn method_32(&mut self) {
    }

    /// Decompiled virtual method #33 @ RVA 0x00103e70 (2 instructions, 1 basic blocks)
    pub fn method_33(&mut self) {
    }

    /// Decompiled virtual method #34 @ RVA 0x00103e70 (2 instructions, 1 basic blocks)
    pub fn method_34(&mut self) {
    }

    /// Decompiled virtual method #35 @ RVA 0x00103e80 (2 instructions, 1 basic blocks)
    pub fn method_35(&mut self) {
    }

    /// Decompiled virtual method #36 @ RVA 0x00103e60 (2 instructions, 1 basic blocks)
    pub fn method_36(&mut self) {
    }

}

impl CcClass for ccRecollectMenuCtrl {
    fn class_name(&self) -> &'static str { Self::CLASS_NAME }
    fn vtable_rva(&self) -> u32 { Self::VTABLE_RVA }
    fn virtual_method_count(&self) -> usize { Self::METHOD_COUNT }
    fn base_classes(&self) -> &'static [&'static str] { Self::BASE_CLASSES }
}

/// Decompiled CC2 class: `ccRecordBookMenu` (VTable RVA: 0x008ea968, 37 virtual methods, 18 recovered fields)
/// Inherits: ccGameMenuCtrlBase, ccGameMenuObj
#[derive(Debug, Clone, Default)]
pub struct ccRecordBookMenu {
    pub vtable_rva: u32,
    pub field_0x0000: u64,
    pub field_0x0004: f32,
    pub field_0x0008: u32,
    pub field_0x0018: u32,
    pub field_0x0038: u32,
    pub field_0x0048: u64,
    pub field_0x0050: u64,
    pub field_0x0060: u32,
    pub field_0x0064: u32,
    pub field_0x0068: u32,
    pub field_0x006c: u32,
    pub field_0x0070: u8,
    pub field_0x0078: u64,
    pub field_0x009c: u32,
    pub field_0x08c0: u8,
    pub field_0x08c4: u64,
    pub field_0x0c6c: u32,
    pub field_0x0c90: u32,
}

impl ccRecordBookMenu {
    pub const CLASS_NAME: &'static str = "ccRecordBookMenu";
    pub const VTABLE_RVA: u32 = 0x008ea968;
    pub const METHOD_COUNT: usize = 37;
    pub const BASE_CLASSES: &'static [&'static str] = &["ccGameMenuCtrlBase", "ccGameMenuObj"];
    pub const VIRTUAL_METHODS: &'static [u64] = &[0x00000001805fdcf0, 0x00000001804520b0, 0x0000000180452130, 0x0000000180451fb0, 0x0000000180451ff0, 0x0000000180452030, 0x0000000180452070, 0x0000000180103e90, 0x0000000180103ea0, 0x0000000180103e60, 0x000000018045bb30, 0x00000001805fdef0, 0x00000001801e9580, 0x0000000180103e60, 0x0000000180103e60, 0x0000000180451dc0, 0x0000000180451e30, 0x0000000180451ea0, 0x0000000180451ee0, 0x0000000180451f40, 0x00000001805fe560, 0x00000001805fe7e0, 0x00000001805fe8e0, 0x0000000180103e60, 0x0000000180103e60, 0x0000000180103e70, 0x0000000180103e70, 0x0000000180103e60, 0x0000000180103e60, 0x0000000180103e60, 0x00000001805fe960, 0x00000001805fec30, 0x00000001800d3090, 0x0000000180103e70, 0x0000000180103e70, 0x0000000180103e80, 0x0000000180103e60];

    pub fn new() -> Self {
        Self {
            vtable_rva: Self::VTABLE_RVA,
            field_0x0000: Default::default(),
            field_0x0004: Default::default(),
            field_0x0008: Default::default(),
            field_0x0018: Default::default(),
            field_0x0038: Default::default(),
            field_0x0048: Default::default(),
            field_0x0050: Default::default(),
            field_0x0060: Default::default(),
            field_0x0064: Default::default(),
            field_0x0068: Default::default(),
            field_0x006c: Default::default(),
            field_0x0070: Default::default(),
            field_0x0078: Default::default(),
            field_0x009c: Default::default(),
            field_0x08c0: Default::default(),
            field_0x08c4: Default::default(),
            field_0x0c6c: Default::default(),
            field_0x0c90: Default::default(),
        }
    }

    /// Decompiled virtual method #0 @ RVA 0x005fdcf0 (16 instructions, 3 basic blocks)
    pub fn method_00(&mut self) {
    }

    /// Decompiled virtual method #1 @ RVA 0x004520b0 (10 instructions, 2 basic blocks)
    pub fn method_01(&mut self) {
    }

    /// Decompiled virtual method #2 @ RVA 0x00452130 (35 instructions, 9 basic blocks)
    pub fn method_02(&mut self) {
    }

    /// Decompiled virtual method #3 @ RVA 0x00451fb0 (118 instructions, 2 basic blocks)
    pub fn method_03(&mut self) {
    }

    /// Decompiled virtual method #4 @ RVA 0x00451ff0 (91 instructions, 2 basic blocks)
    pub fn method_04(&mut self) {
    }

    /// Decompiled virtual method #5 @ RVA 0x00452030 (64 instructions, 2 basic blocks)
    pub fn method_05(&mut self) {
    }

    /// Decompiled virtual method #6 @ RVA 0x00452070 (37 instructions, 2 basic blocks)
    pub fn method_06(&mut self) {
    }

    /// Decompiled virtual method #7 @ RVA 0x00103e90 (2 instructions, 1 basic blocks)
    pub fn method_07(&mut self) {
    }

    /// Decompiled virtual method #8 @ RVA 0x00103ea0 (9 instructions, 1 basic blocks)
    pub fn method_08(&mut self) {
        // Writes to field at offset +0x0008
    }

    /// Decompiled virtual method #9 @ RVA 0x00103e60 (2 instructions, 1 basic blocks)
    pub fn method_09(&mut self) {
    }

    /// Decompiled virtual method #10 @ RVA 0x0045bb30 (16 instructions, 2 basic blocks)
    pub fn method_10(&mut self) {
    }

    /// Decompiled virtual method #11 @ RVA 0x005fdef0 (90 instructions, 1 basic blocks)
    pub fn method_11(&mut self) {
        // Writes to field at offset +0x0000
    }

    /// Decompiled virtual method #12 @ RVA 0x001e9580 (53 instructions, 12 basic blocks)
    pub fn method_12(&mut self) {
    }

    /// Decompiled virtual method #13 @ RVA 0x00103e60 (2 instructions, 1 basic blocks)
    pub fn method_13(&mut self) {
    }

    /// Decompiled virtual method #14 @ RVA 0x00103e60 (2 instructions, 1 basic blocks)
    pub fn method_14(&mut self) {
    }

    /// Decompiled virtual method #15 @ RVA 0x00451dc0 (10 instructions, 2 basic blocks)
    pub fn method_15(&mut self) {
    }

    /// Decompiled virtual method #16 @ RVA 0x00451e30 (10 instructions, 2 basic blocks)
    pub fn method_16(&mut self) {
    }

    /// Decompiled virtual method #17 @ RVA 0x00451ea0 (31 instructions, 2 basic blocks)
    pub fn method_17(&mut self) {
        // Writes to field at offset +0x0018
    }

    /// Decompiled virtual method #18 @ RVA 0x00451ee0 (10 instructions, 2 basic blocks)
    pub fn method_18(&mut self) {
    }

    /// Decompiled virtual method #19 @ RVA 0x00451f40 (10 instructions, 2 basic blocks)
    pub fn method_19(&mut self) {
    }

    /// Decompiled virtual method #20 @ RVA 0x005fe560 (119 instructions, 16 basic blocks)
    pub fn method_20(&mut self) {
        // Writes to field at offset +0x0038
        // Writes to field at offset +0x0048
        // Writes to field at offset +0x0060
        // Writes to field at offset +0x0064
        // Writes to field at offset +0x0068
        // Writes to field at offset +0x006c
        // Writes to field at offset +0x0070
        // Writes to field at offset +0x08c4
        // Writes to field at offset +0x0c6c
        // Writes to field at offset +0x0c90
    }

    /// Decompiled virtual method #21 @ RVA 0x005fe7e0 (61 instructions, 8 basic blocks)
    pub fn method_21(&mut self) {
    }

    /// Decompiled virtual method #22 @ RVA 0x005fe8e0 (29 instructions, 1 basic blocks)
    pub fn method_22(&mut self) {
    }

    /// Decompiled virtual method #23 @ RVA 0x00103e60 (2 instructions, 1 basic blocks)
    pub fn method_23(&mut self) {
    }

    /// Decompiled virtual method #24 @ RVA 0x00103e60 (2 instructions, 1 basic blocks)
    pub fn method_24(&mut self) {
    }

    /// Decompiled virtual method #25 @ RVA 0x00103e70 (2 instructions, 1 basic blocks)
    pub fn method_25(&mut self) {
    }

    /// Decompiled virtual method #26 @ RVA 0x00103e70 (2 instructions, 1 basic blocks)
    pub fn method_26(&mut self) {
    }

    /// Decompiled virtual method #27 @ RVA 0x00103e60 (2 instructions, 1 basic blocks)
    pub fn method_27(&mut self) {
    }

    /// Decompiled virtual method #28 @ RVA 0x00103e60 (2 instructions, 1 basic blocks)
    pub fn method_28(&mut self) {
    }

    /// Decompiled virtual method #29 @ RVA 0x00103e60 (2 instructions, 1 basic blocks)
    pub fn method_29(&mut self) {
    }

    /// Decompiled virtual method #30 @ RVA 0x005fe960 (105 instructions, 22 basic blocks)
    pub fn method_30(&mut self) {
    }

    /// Decompiled virtual method #31 @ RVA 0x005fec30 (103 instructions, 5 basic blocks)
    pub fn method_31(&mut self) {
        // Writes to field at offset +0x08c0
    }

    /// Decompiled virtual method #32 @ RVA 0x000d3090 (1 instructions, 1 basic blocks)
    pub fn method_32(&mut self) {
    }

    /// Decompiled virtual method #33 @ RVA 0x00103e70 (2 instructions, 1 basic blocks)
    pub fn method_33(&mut self) {
    }

    /// Decompiled virtual method #34 @ RVA 0x00103e70 (2 instructions, 1 basic blocks)
    pub fn method_34(&mut self) {
    }

    /// Decompiled virtual method #35 @ RVA 0x00103e80 (2 instructions, 1 basic blocks)
    pub fn method_35(&mut self) {
    }

    /// Decompiled virtual method #36 @ RVA 0x00103e60 (2 instructions, 1 basic blocks)
    pub fn method_36(&mut self) {
    }

}

impl CcClass for ccRecordBookMenu {
    fn class_name(&self) -> &'static str { Self::CLASS_NAME }
    fn vtable_rva(&self) -> u32 { Self::VTABLE_RVA }
    fn virtual_method_count(&self) -> usize { Self::METHOD_COUNT }
    fn base_classes(&self) -> &'static [&'static str] { Self::BASE_CLASSES }
}

/// Decompiled CC2 class: `ccSaNameDisp` (VTable RVA: 0x00881440, 14 virtual methods, 16 recovered fields)
/// Inherits: ccDispBase
#[derive(Debug, Clone, Default)]
pub struct ccSaNameDisp {
    pub vtable_rva: u32,
    pub field_0x0000: u64,
    pub field_0x0008: u32,
    pub field_0x001a: u8,
    pub field_0x001c: u32,
    pub field_0x0028: u32,
    pub field_0x002c: u32,
    pub field_0x0030: u32,
    pub field_0x0034: u32,
    pub field_0x0038: u32,
    pub field_0x0040: u64,
    pub field_0x0048: u64,
    pub field_0x0050: u64,
    pub field_0x0058: u64,
    pub field_0x00c8: u64,
    pub field_0x00d2: u16,
    pub field_0x00d4: f32,
}

impl ccSaNameDisp {
    pub const CLASS_NAME: &'static str = "ccSaNameDisp";
    pub const VTABLE_RVA: u32 = 0x00881440;
    pub const METHOD_COUNT: usize = 14;
    pub const BASE_CLASSES: &'static [&'static str] = &["ccDispBase"];
    pub const VIRTUAL_METHODS: &'static [u64] = &[0x00000001802398a0, 0x00000001802398d0, 0x0000000180239d00, 0x0000000180239e00, 0x00000001802a0e30, 0x00000001800d3090, 0x00000001801b81f0, 0x00000001801b8200, 0x00000001801b8210, 0x000000018023a190, 0x00000001800d3090, 0x000000018023a250, 0x000000018023a210, 0x000000018023a2d0];

    pub fn new() -> Self {
        Self {
            vtable_rva: Self::VTABLE_RVA,
            field_0x0000: Default::default(),
            field_0x0008: Default::default(),
            field_0x001a: Default::default(),
            field_0x001c: Default::default(),
            field_0x0028: Default::default(),
            field_0x002c: Default::default(),
            field_0x0030: Default::default(),
            field_0x0034: Default::default(),
            field_0x0038: Default::default(),
            field_0x0040: Default::default(),
            field_0x0048: Default::default(),
            field_0x0050: Default::default(),
            field_0x0058: Default::default(),
            field_0x00c8: Default::default(),
            field_0x00d2: Default::default(),
            field_0x00d4: Default::default(),
        }
    }

    /// Decompiled virtual method #0 @ RVA 0x002398a0 (13 instructions, 3 basic blocks)
    pub fn method_00(&mut self) {
        // Writes to field at offset +0x0000
    }

    /// Decompiled virtual method #1 @ RVA 0x002398d0 (116 instructions, 14 basic blocks)
    pub fn method_01(&mut self) {
    }

    /// Decompiled virtual method #2 @ RVA 0x00239d00 (64 instructions, 13 basic blocks)
    pub fn method_02(&mut self) {
    }

    /// Decompiled virtual method #3 @ RVA 0x00239e00 (112 instructions, 12 basic blocks)
    pub fn method_03(&mut self) {
        // Writes to field at offset +0x0008
        // Writes to field at offset +0x001a
        // Writes to field at offset +0x0038
    }

    /// Decompiled virtual method #4 @ RVA 0x002a0e30 (48 instructions, 11 basic blocks)
    pub fn method_04(&mut self) {
        // Writes to field at offset +0x0008
    }

    /// Decompiled virtual method #5 @ RVA 0x000d3090 (1 instructions, 1 basic blocks)
    pub fn method_05(&mut self) {
    }

    /// Decompiled virtual method #6 @ RVA 0x001b81f0 (2 instructions, 1 basic blocks)
    pub fn method_06(&mut self) {
        // Writes to field at offset +0x0008
    }

    /// Decompiled virtual method #7 @ RVA 0x001b8200 (5 instructions, 1 basic blocks)
    pub fn method_07(&mut self) {
    }

    /// Decompiled virtual method #8 @ RVA 0x001b8210 (5 instructions, 1 basic blocks)
    pub fn method_08(&mut self) {
    }

    /// Decompiled virtual method #9 @ RVA 0x0023a190 (32 instructions, 7 basic blocks)
    pub fn method_09(&mut self) {
        // Writes to field at offset +0x0038
    }

    /// Decompiled virtual method #10 @ RVA 0x000d3090 (1 instructions, 1 basic blocks)
    pub fn method_10(&mut self) {
    }

    /// Decompiled virtual method #11 @ RVA 0x0023a250 (35 instructions, 4 basic blocks)
    pub fn method_11(&mut self) {
        // Writes to field at offset +0x0038
    }

    /// Decompiled virtual method #12 @ RVA 0x0023a210 (12 instructions, 5 basic blocks)
    pub fn method_12(&mut self) {
        // Writes to field at offset +0x0038
        // Writes to field at offset +0x00d4
        // Writes to field at offset +0x00d4
    }

    /// Decompiled virtual method #13 @ RVA 0x0023a2d0 (13 instructions, 3 basic blocks)
    pub fn method_13(&mut self) {
        // Writes to field at offset +0x0038
        // Writes to field at offset +0x00d2
    }

}

impl CcClass for ccSaNameDisp {
    fn class_name(&self) -> &'static str { Self::CLASS_NAME }
    fn vtable_rva(&self) -> u32 { Self::VTABLE_RVA }
    fn virtual_method_count(&self) -> usize { Self::METHOD_COUNT }
    fn base_classes(&self) -> &'static [&'static str] { Self::BASE_CLASSES }
}

/// Decompiled CC2 class: `ccShortCutItemWnd` (VTable RVA: 0x00871608, 37 virtual methods, 10 recovered fields)
/// Inherits: ccItemWnd, ccGameMenuCtrlBase, ccGameMenuObj
#[derive(Debug, Clone, Default)]
pub struct ccShortCutItemWnd {
    pub vtable_rva: u32,
    pub field_0x0000: u64,
    pub field_0x0008: u32,
    pub field_0x0010: u8,
    pub field_0x0018: u32,
    pub field_0x0048: u64,
    pub field_0x0050: u64,
    pub field_0x0060: u8,
    pub field_0x0078: u32,
    pub field_0x009c: u32,
    pub field_0x00e8: u64,
}

impl ccShortCutItemWnd {
    pub const CLASS_NAME: &'static str = "ccShortCutItemWnd";
    pub const VTABLE_RVA: u32 = 0x00871608;
    pub const METHOD_COUNT: usize = 37;
    pub const BASE_CLASSES: &'static [&'static str] = &["ccItemWnd", "ccGameMenuCtrlBase", "ccGameMenuObj"];
    pub const VIRTUAL_METHODS: &'static [u64] = &[0x0000000180103ed0, 0x00000001804520b0, 0x0000000180452130, 0x0000000180451fb0, 0x0000000180451ff0, 0x0000000180452030, 0x0000000180452070, 0x0000000180103e90, 0x0000000180103ea0, 0x0000000180103e60, 0x000000018045bb30, 0x0000000180451d90, 0x00000001801e9580, 0x0000000180103e60, 0x0000000180103e60, 0x0000000180451dc0, 0x0000000180451e30, 0x0000000180451ea0, 0x0000000180451ee0, 0x0000000180451f40, 0x0000000180103f10, 0x0000000180103f40, 0x0000000180550c30, 0x0000000180103e60, 0x0000000180103e60, 0x0000000180103e70, 0x0000000180550cf0, 0x0000000180103e60, 0x0000000180103e60, 0x0000000180103e60, 0x0000000180550c70, 0x0000000180550cb0, 0x00000001800d3090, 0x0000000180103e70, 0x0000000180103e70, 0x0000000180103e80, 0x0000000180103e60];

    pub fn new() -> Self {
        Self {
            vtable_rva: Self::VTABLE_RVA,
            field_0x0000: Default::default(),
            field_0x0008: Default::default(),
            field_0x0010: Default::default(),
            field_0x0018: Default::default(),
            field_0x0048: Default::default(),
            field_0x0050: Default::default(),
            field_0x0060: Default::default(),
            field_0x0078: Default::default(),
            field_0x009c: Default::default(),
            field_0x00e8: Default::default(),
        }
    }

    /// Decompiled virtual method #0 @ RVA 0x00103ed0 (18 instructions, 3 basic blocks)
    pub fn method_00(&mut self) {
        // Writes to field at offset +0x0000
    }

    /// Decompiled virtual method #1 @ RVA 0x004520b0 (10 instructions, 2 basic blocks)
    pub fn method_01(&mut self) {
    }

    /// Decompiled virtual method #2 @ RVA 0x00452130 (35 instructions, 9 basic blocks)
    pub fn method_02(&mut self) {
    }

    /// Decompiled virtual method #3 @ RVA 0x00451fb0 (118 instructions, 2 basic blocks)
    pub fn method_03(&mut self) {
    }

    /// Decompiled virtual method #4 @ RVA 0x00451ff0 (91 instructions, 2 basic blocks)
    pub fn method_04(&mut self) {
    }

    /// Decompiled virtual method #5 @ RVA 0x00452030 (64 instructions, 2 basic blocks)
    pub fn method_05(&mut self) {
    }

    /// Decompiled virtual method #6 @ RVA 0x00452070 (37 instructions, 2 basic blocks)
    pub fn method_06(&mut self) {
    }

    /// Decompiled virtual method #7 @ RVA 0x00103e90 (2 instructions, 1 basic blocks)
    pub fn method_07(&mut self) {
    }

    /// Decompiled virtual method #8 @ RVA 0x00103ea0 (9 instructions, 1 basic blocks)
    pub fn method_08(&mut self) {
        // Writes to field at offset +0x0008
    }

    /// Decompiled virtual method #9 @ RVA 0x00103e60 (2 instructions, 1 basic blocks)
    pub fn method_09(&mut self) {
    }

    /// Decompiled virtual method #10 @ RVA 0x0045bb30 (16 instructions, 2 basic blocks)
    pub fn method_10(&mut self) {
    }

    /// Decompiled virtual method #11 @ RVA 0x00451d90 (24 instructions, 2 basic blocks)
    pub fn method_11(&mut self) {
    }

    /// Decompiled virtual method #12 @ RVA 0x001e9580 (53 instructions, 12 basic blocks)
    pub fn method_12(&mut self) {
    }

    /// Decompiled virtual method #13 @ RVA 0x00103e60 (2 instructions, 1 basic blocks)
    pub fn method_13(&mut self) {
    }

    /// Decompiled virtual method #14 @ RVA 0x00103e60 (2 instructions, 1 basic blocks)
    pub fn method_14(&mut self) {
    }

    /// Decompiled virtual method #15 @ RVA 0x00451dc0 (10 instructions, 2 basic blocks)
    pub fn method_15(&mut self) {
    }

    /// Decompiled virtual method #16 @ RVA 0x00451e30 (10 instructions, 2 basic blocks)
    pub fn method_16(&mut self) {
    }

    /// Decompiled virtual method #17 @ RVA 0x00451ea0 (31 instructions, 2 basic blocks)
    pub fn method_17(&mut self) {
        // Writes to field at offset +0x0018
    }

    /// Decompiled virtual method #18 @ RVA 0x00451ee0 (10 instructions, 2 basic blocks)
    pub fn method_18(&mut self) {
    }

    /// Decompiled virtual method #19 @ RVA 0x00451f40 (10 instructions, 2 basic blocks)
    pub fn method_19(&mut self) {
    }

    /// Decompiled virtual method #20 @ RVA 0x00103f10 (9 instructions, 1 basic blocks)
    pub fn method_20(&mut self) {
        // Writes to field at offset +0x0078
        // Writes to field at offset +0x00e8
    }

    /// Decompiled virtual method #21 @ RVA 0x00103f40 (10 instructions, 3 basic blocks)
    pub fn method_21(&mut self) {
        // Writes to field at offset +0x0010
    }

    /// Decompiled virtual method #22 @ RVA 0x00550c30 (16 instructions, 1 basic blocks)
    pub fn method_22(&mut self) {
    }

    /// Decompiled virtual method #23 @ RVA 0x00103e60 (2 instructions, 1 basic blocks)
    pub fn method_23(&mut self) {
    }

    /// Decompiled virtual method #24 @ RVA 0x00103e60 (2 instructions, 1 basic blocks)
    pub fn method_24(&mut self) {
    }

    /// Decompiled virtual method #25 @ RVA 0x00103e70 (2 instructions, 1 basic blocks)
    pub fn method_25(&mut self) {
    }

    /// Decompiled virtual method #26 @ RVA 0x00550cf0 (10 instructions, 4 basic blocks)
    pub fn method_26(&mut self) {
        // Writes to field at offset +0x0060
    }

    /// Decompiled virtual method #27 @ RVA 0x00103e60 (2 instructions, 1 basic blocks)
    pub fn method_27(&mut self) {
    }

    /// Decompiled virtual method #28 @ RVA 0x00103e60 (2 instructions, 1 basic blocks)
    pub fn method_28(&mut self) {
    }

    /// Decompiled virtual method #29 @ RVA 0x00103e60 (2 instructions, 1 basic blocks)
    pub fn method_29(&mut self) {
    }

    /// Decompiled virtual method #30 @ RVA 0x00550c70 (15 instructions, 1 basic blocks)
    pub fn method_30(&mut self) {
    }

    /// Decompiled virtual method #31 @ RVA 0x00550cb0 (35 instructions, 4 basic blocks)
    pub fn method_31(&mut self) {
        // Writes to field at offset +0x0060
    }

    /// Decompiled virtual method #32 @ RVA 0x000d3090 (1 instructions, 1 basic blocks)
    pub fn method_32(&mut self) {
    }

    /// Decompiled virtual method #33 @ RVA 0x00103e70 (2 instructions, 1 basic blocks)
    pub fn method_33(&mut self) {
    }

    /// Decompiled virtual method #34 @ RVA 0x00103e70 (2 instructions, 1 basic blocks)
    pub fn method_34(&mut self) {
    }

    /// Decompiled virtual method #35 @ RVA 0x00103e80 (2 instructions, 1 basic blocks)
    pub fn method_35(&mut self) {
    }

    /// Decompiled virtual method #36 @ RVA 0x00103e60 (2 instructions, 1 basic blocks)
    pub fn method_36(&mut self) {
    }

}

impl CcClass for ccShortCutItemWnd {
    fn class_name(&self) -> &'static str { Self::CLASS_NAME }
    fn vtable_rva(&self) -> u32 { Self::VTABLE_RVA }
    fn virtual_method_count(&self) -> usize { Self::METHOD_COUNT }
    fn base_classes(&self) -> &'static [&'static str] { Self::BASE_CLASSES }
}

/// Decompiled CC2 class: `ccSingleMenuMain` (VTable RVA: 0x0087d648, 31 virtual methods, 11 recovered fields)
/// Inherits: ccGameMenuMain, ccGameMenuObj
#[derive(Debug, Clone, Default)]
pub struct ccSingleMenuMain {
    pub vtable_rva: u32,
    pub field_0x0000: u64,
    pub field_0x0008: u32,
    pub field_0x000c: u32,
    pub field_0x0018: u32,
    pub field_0x0028: u8,
    pub field_0x0030: u32,
    pub field_0x0034: f32,
    pub field_0x0038: u64,
    pub field_0x0040: u8,
    pub field_0x0048: u64,
    pub field_0x0050: u64,
}

impl ccSingleMenuMain {
    pub const CLASS_NAME: &'static str = "ccSingleMenuMain";
    pub const VTABLE_RVA: u32 = 0x0087d648;
    pub const METHOD_COUNT: usize = 31;
    pub const BASE_CLASSES: &'static [&'static str] = &["ccGameMenuMain", "ccGameMenuObj"];
    pub const VIRTUAL_METHODS: &'static [u64] = &[0x00000001801f0ef0, 0x0000000180452940, 0x0000000180452b60, 0x00000001804525e0, 0x0000000180452730, 0x00000001804527e0, 0x0000000180452890, 0x00000001801b27f0, 0x00000001801035b0, 0x0000000180452bf0, 0x000000018045ba00, 0x00000001800d3090, 0x0000000180452300, 0x0000000180452380, 0x00000001801f0fb0, 0x00000001801f1040, 0x00000001804523f0, 0x00000001801f10b0, 0x0000000180452520, 0x0000000180103e70, 0x0000000180103e70, 0x0000000180103e70, 0x0000000180103e60, 0x0000000180103e60, 0x0000000180103e60, 0x0000000180103e60, 0x0000000180103e60, 0x00000001801f10c0, 0x00000001800d3090, 0x0000000180452bd0, 0x00000001801b2800];

    pub fn new() -> Self {
        Self {
            vtable_rva: Self::VTABLE_RVA,
            field_0x0000: Default::default(),
            field_0x0008: Default::default(),
            field_0x000c: Default::default(),
            field_0x0018: Default::default(),
            field_0x0028: Default::default(),
            field_0x0030: Default::default(),
            field_0x0034: Default::default(),
            field_0x0038: Default::default(),
            field_0x0040: Default::default(),
            field_0x0048: Default::default(),
            field_0x0050: Default::default(),
        }
    }

    /// Decompiled virtual method #0 @ RVA 0x001f0ef0 (16 instructions, 3 basic blocks)
    pub fn method_00(&mut self) {
    }

    /// Decompiled virtual method #1 @ RVA 0x00452940 (10 instructions, 2 basic blocks)
    pub fn method_01(&mut self) {
    }

    /// Decompiled virtual method #2 @ RVA 0x00452b60 (30 instructions, 4 basic blocks)
    pub fn method_02(&mut self) {
    }

    /// Decompiled virtual method #3 @ RVA 0x004525e0 (9 instructions, 2 basic blocks)
    pub fn method_03(&mut self) {
        // Writes to field at offset +0x0040
    }

    /// Decompiled virtual method #4 @ RVA 0x00452730 (9 instructions, 2 basic blocks)
    pub fn method_04(&mut self) {
        // Writes to field at offset +0x0040
    }

    /// Decompiled virtual method #5 @ RVA 0x004527e0 (9 instructions, 2 basic blocks)
    pub fn method_05(&mut self) {
        // Writes to field at offset +0x0040
    }

    /// Decompiled virtual method #6 @ RVA 0x00452890 (9 instructions, 2 basic blocks)
    pub fn method_06(&mut self) {
        // Writes to field at offset +0x0040
    }

    /// Decompiled virtual method #7 @ RVA 0x001b27f0 (2 instructions, 1 basic blocks)
    pub fn method_07(&mut self) {
    }

    /// Decompiled virtual method #8 @ RVA 0x001035b0 (5 instructions, 1 basic blocks)
    pub fn method_08(&mut self) {
    }

    /// Decompiled virtual method #9 @ RVA 0x00452bf0 (12 instructions, 2 basic blocks)
    pub fn method_09(&mut self) {
        // Writes to field at offset +0x0038
    }

    /// Decompiled virtual method #10 @ RVA 0x0045ba00 (17 instructions, 2 basic blocks)
    pub fn method_10(&mut self) {
    }

    /// Decompiled virtual method #11 @ RVA 0x000d3090 (1 instructions, 1 basic blocks)
    pub fn method_11(&mut self) {
    }

    /// Decompiled virtual method #12 @ RVA 0x00452300 (9 instructions, 2 basic blocks)
    pub fn method_12(&mut self) {
        // Writes to field at offset +0x0028
    }

    /// Decompiled virtual method #13 @ RVA 0x00452380 (9 instructions, 2 basic blocks)
    pub fn method_13(&mut self) {
        // Writes to field at offset +0x0028
    }

    /// Decompiled virtual method #14 @ RVA 0x001f0fb0 (36 instructions, 1 basic blocks)
    pub fn method_14(&mut self) {
        // Writes to field at offset +0x0000
        // Writes to field at offset +0x000c
    }

    /// Decompiled virtual method #15 @ RVA 0x001f1040 (22 instructions, 2 basic blocks)
    pub fn method_15(&mut self) {
    }

    /// Decompiled virtual method #16 @ RVA 0x004523f0 (12 instructions, 2 basic blocks)
    pub fn method_16(&mut self) {
    }

    /// Decompiled virtual method #17 @ RVA 0x001f10b0 (4 instructions, 1 basic blocks)
    pub fn method_17(&mut self) {
    }

    /// Decompiled virtual method #18 @ RVA 0x00452520 (12 instructions, 2 basic blocks)
    pub fn method_18(&mut self) {
    }

    /// Decompiled virtual method #19 @ RVA 0x00103e70 (2 instructions, 1 basic blocks)
    pub fn method_19(&mut self) {
    }

    /// Decompiled virtual method #20 @ RVA 0x00103e70 (2 instructions, 1 basic blocks)
    pub fn method_20(&mut self) {
    }

    /// Decompiled virtual method #21 @ RVA 0x00103e70 (2 instructions, 1 basic blocks)
    pub fn method_21(&mut self) {
    }

    /// Decompiled virtual method #22 @ RVA 0x00103e60 (2 instructions, 1 basic blocks)
    pub fn method_22(&mut self) {
    }

    /// Decompiled virtual method #23 @ RVA 0x00103e60 (2 instructions, 1 basic blocks)
    pub fn method_23(&mut self) {
    }

    /// Decompiled virtual method #24 @ RVA 0x00103e60 (2 instructions, 1 basic blocks)
    pub fn method_24(&mut self) {
    }

    /// Decompiled virtual method #25 @ RVA 0x00103e60 (2 instructions, 1 basic blocks)
    pub fn method_25(&mut self) {
    }

    /// Decompiled virtual method #26 @ RVA 0x00103e60 (2 instructions, 1 basic blocks)
    pub fn method_26(&mut self) {
    }

    /// Decompiled virtual method #27 @ RVA 0x001f10c0 (22 instructions, 7 basic blocks)
    pub fn method_27(&mut self) {
        // Writes to field at offset +0x0018
        // Writes to field at offset +0x0038
        // Writes to field at offset +0x0048
    }

    /// Decompiled virtual method #28 @ RVA 0x000d3090 (1 instructions, 1 basic blocks)
    pub fn method_28(&mut self) {
    }

    /// Decompiled virtual method #29 @ RVA 0x00452bd0 (5 instructions, 2 basic blocks)
    pub fn method_29(&mut self) {
    }

    /// Decompiled virtual method #30 @ RVA 0x001b2800 (13 instructions, 4 basic blocks)
    pub fn method_30(&mut self) {
        // Writes to field at offset +0x0018
        // Writes to field at offset +0x0028
    }

}

impl CcClass for ccSingleMenuMain {
    fn class_name(&self) -> &'static str { Self::CLASS_NAME }
    fn vtable_rva(&self) -> u32 { Self::VTABLE_RVA }
    fn virtual_method_count(&self) -> usize { Self::METHOD_COUNT }
    fn base_classes(&self) -> &'static [&'static str] { Self::BASE_CLASSES }
}

/// Decompiled CC2 class: `ccSkillNameDisp` (VTable RVA: 0x00881530, 14 virtual methods, 18 recovered fields)
/// Inherits: ccDispBase
#[derive(Debug, Clone, Default)]
pub struct ccSkillNameDisp {
    pub vtable_rva: u32,
    pub field_0x0000: u64,
    pub field_0x0008: u32,
    pub field_0x000c: f32,
    pub field_0x0010: u32,
    pub field_0x0014: u32,
    pub field_0x0018: f32,
    pub field_0x001a: u8,
    pub field_0x001c: f32,
    pub field_0x0028: u32,
    pub field_0x002c: u32,
    pub field_0x0030: u32,
    pub field_0x0034: u32,
    pub field_0x0038: u32,
    pub field_0x0040: u64,
    pub field_0x0070: f32,
    pub field_0x0074: f32,
    pub field_0x0078: f32,
    pub field_0x0086: u16,
}

impl ccSkillNameDisp {
    pub const CLASS_NAME: &'static str = "ccSkillNameDisp";
    pub const VTABLE_RVA: u32 = 0x00881530;
    pub const METHOD_COUNT: usize = 14;
    pub const BASE_CLASSES: &'static [&'static str] = &["ccDispBase"];
    pub const VIRTUAL_METHODS: &'static [u64] = &[0x00000001802374f0, 0x00000001800d3090, 0x0000000180237520, 0x0000000180238000, 0x00000001802a0e30, 0x00000001800d3090, 0x00000001801b81f0, 0x00000001801b8200, 0x00000001801b8210, 0x00000001800d3090, 0x00000001800d3090, 0x0000000180237b60, 0x0000000180237cb0, 0x0000000180237f20];

    pub fn new() -> Self {
        Self {
            vtable_rva: Self::VTABLE_RVA,
            field_0x0000: Default::default(),
            field_0x0008: Default::default(),
            field_0x000c: Default::default(),
            field_0x0010: Default::default(),
            field_0x0014: Default::default(),
            field_0x0018: Default::default(),
            field_0x001a: Default::default(),
            field_0x001c: Default::default(),
            field_0x0028: Default::default(),
            field_0x002c: Default::default(),
            field_0x0030: Default::default(),
            field_0x0034: Default::default(),
            field_0x0038: Default::default(),
            field_0x0040: Default::default(),
            field_0x0070: Default::default(),
            field_0x0074: Default::default(),
            field_0x0078: Default::default(),
            field_0x0086: Default::default(),
        }
    }

    /// Decompiled virtual method #0 @ RVA 0x002374f0 (13 instructions, 3 basic blocks)
    pub fn method_00(&mut self) {
        // Writes to field at offset +0x0000
    }

    /// Decompiled virtual method #1 @ RVA 0x000d3090 (1 instructions, 1 basic blocks)
    pub fn method_01(&mut self) {
    }

    /// Decompiled virtual method #2 @ RVA 0x00237520 (30 instructions, 8 basic blocks)
    pub fn method_02(&mut self) {
    }

    /// Decompiled virtual method #3 @ RVA 0x00238000 (101 instructions, 7 basic blocks)
    pub fn method_03(&mut self) {
        // Writes to field at offset +0x0008
        // Writes to field at offset +0x001a
    }

    /// Decompiled virtual method #4 @ RVA 0x002a0e30 (48 instructions, 11 basic blocks)
    pub fn method_04(&mut self) {
        // Writes to field at offset +0x0008
    }

    /// Decompiled virtual method #5 @ RVA 0x000d3090 (1 instructions, 1 basic blocks)
    pub fn method_05(&mut self) {
    }

    /// Decompiled virtual method #6 @ RVA 0x001b81f0 (2 instructions, 1 basic blocks)
    pub fn method_06(&mut self) {
        // Writes to field at offset +0x0008
    }

    /// Decompiled virtual method #7 @ RVA 0x001b8200 (5 instructions, 1 basic blocks)
    pub fn method_07(&mut self) {
    }

    /// Decompiled virtual method #8 @ RVA 0x001b8210 (5 instructions, 1 basic blocks)
    pub fn method_08(&mut self) {
    }

    /// Decompiled virtual method #9 @ RVA 0x000d3090 (1 instructions, 1 basic blocks)
    pub fn method_09(&mut self) {
    }

    /// Decompiled virtual method #10 @ RVA 0x000d3090 (1 instructions, 1 basic blocks)
    pub fn method_10(&mut self) {
    }

    /// Decompiled virtual method #11 @ RVA 0x00237b60 (68 instructions, 18 basic blocks)
    pub fn method_11(&mut self) {
        // Writes to field at offset +0x001c
        // Writes to field at offset +0x001c
        // Writes to field at offset +0x0038
        // Writes to field at offset +0x0078
        // Writes to field at offset +0x0078
        // Writes to field at offset +0x0086
    }

    /// Decompiled virtual method #12 @ RVA 0x00237cb0 (36 instructions, 10 basic blocks)
    pub fn method_12(&mut self) {
        // Writes to field at offset +0x0038
        // Writes to field at offset +0x0070
        // Writes to field at offset +0x0070
    }

    /// Decompiled virtual method #13 @ RVA 0x00237f20 (58 instructions, 14 basic blocks)
    pub fn method_13(&mut self) {
        // Writes to field at offset +0x001a
        // Writes to field at offset +0x001c
        // Writes to field at offset +0x001c
        // Writes to field at offset +0x0078
        // Writes to field at offset +0x0078
    }

}

impl CcClass for ccSkillNameDisp {
    fn class_name(&self) -> &'static str { Self::CLASS_NAME }
    fn vtable_rva(&self) -> u32 { Self::VTABLE_RVA }
    fn virtual_method_count(&self) -> usize { Self::METHOD_COUNT }
    fn base_classes(&self) -> &'static [&'static str] { Self::BASE_CLASSES }
}

/// Decompiled CC2 class: `ccSkillTrgEditWnd` (VTable RVA: 0x008bb108, 37 virtual methods, 11 recovered fields)
/// Inherits: ccGameMenuCtrlBase, ccGameMenuObj
#[derive(Debug, Clone, Default)]
pub struct ccSkillTrgEditWnd {
    pub vtable_rva: u32,
    pub field_0x0000: u64,
    pub field_0x0002: u32,
    pub field_0x0008: u32,
    pub field_0x0018: u32,
    pub field_0x001a: u8,
    pub field_0x0048: u64,
    pub field_0x0050: u64,
    pub field_0x0060: u32,
    pub field_0x009c: u32,
    pub field_0x00b8: u64,
    pub field_0x00e0: u64,
}

impl ccSkillTrgEditWnd {
    pub const CLASS_NAME: &'static str = "ccSkillTrgEditWnd";
    pub const VTABLE_RVA: u32 = 0x008bb108;
    pub const METHOD_COUNT: usize = 37;
    pub const BASE_CLASSES: &'static [&'static str] = &["ccGameMenuCtrlBase", "ccGameMenuObj"];
    pub const VIRTUAL_METHODS: &'static [u64] = &[0x000000018044ff60, 0x00000001804520b0, 0x0000000180452130, 0x0000000180451fb0, 0x0000000180451ff0, 0x0000000180452030, 0x0000000180452070, 0x0000000180103e90, 0x0000000180103ea0, 0x0000000180103e60, 0x000000018045bb30, 0x0000000180451d90, 0x00000001801e9580, 0x0000000180103e60, 0x0000000180103e60, 0x0000000180451dc0, 0x0000000180451e30, 0x0000000180451ea0, 0x0000000180451ee0, 0x0000000180451f40, 0x0000000180620860, 0x0000000180620bd0, 0x0000000180620d30, 0x0000000180103e60, 0x0000000180103e60, 0x0000000180103e60, 0x0000000180620d50, 0x0000000180103e60, 0x0000000180103e80, 0x0000000180103e80, 0x0000000180620170, 0x0000000180620800, 0x00000001800d3090, 0x0000000180103e70, 0x0000000180103e70, 0x0000000180103e80, 0x0000000180103e60];

    pub fn new() -> Self {
        Self {
            vtable_rva: Self::VTABLE_RVA,
            field_0x0000: Default::default(),
            field_0x0002: Default::default(),
            field_0x0008: Default::default(),
            field_0x0018: Default::default(),
            field_0x001a: Default::default(),
            field_0x0048: Default::default(),
            field_0x0050: Default::default(),
            field_0x0060: Default::default(),
            field_0x009c: Default::default(),
            field_0x00b8: Default::default(),
            field_0x00e0: Default::default(),
        }
    }

    /// Decompiled virtual method #0 @ RVA 0x0044ff60 (34 instructions, 3 basic blocks)
    pub fn method_00(&mut self) {
        // Writes to field at offset +0x0000
    }

    /// Decompiled virtual method #1 @ RVA 0x004520b0 (10 instructions, 2 basic blocks)
    pub fn method_01(&mut self) {
    }

    /// Decompiled virtual method #2 @ RVA 0x00452130 (35 instructions, 9 basic blocks)
    pub fn method_02(&mut self) {
    }

    /// Decompiled virtual method #3 @ RVA 0x00451fb0 (118 instructions, 2 basic blocks)
    pub fn method_03(&mut self) {
    }

    /// Decompiled virtual method #4 @ RVA 0x00451ff0 (91 instructions, 2 basic blocks)
    pub fn method_04(&mut self) {
    }

    /// Decompiled virtual method #5 @ RVA 0x00452030 (64 instructions, 2 basic blocks)
    pub fn method_05(&mut self) {
    }

    /// Decompiled virtual method #6 @ RVA 0x00452070 (37 instructions, 2 basic blocks)
    pub fn method_06(&mut self) {
    }

    /// Decompiled virtual method #7 @ RVA 0x00103e90 (2 instructions, 1 basic blocks)
    pub fn method_07(&mut self) {
    }

    /// Decompiled virtual method #8 @ RVA 0x00103ea0 (9 instructions, 1 basic blocks)
    pub fn method_08(&mut self) {
        // Writes to field at offset +0x0008
    }

    /// Decompiled virtual method #9 @ RVA 0x00103e60 (2 instructions, 1 basic blocks)
    pub fn method_09(&mut self) {
    }

    /// Decompiled virtual method #10 @ RVA 0x0045bb30 (16 instructions, 2 basic blocks)
    pub fn method_10(&mut self) {
    }

    /// Decompiled virtual method #11 @ RVA 0x00451d90 (24 instructions, 2 basic blocks)
    pub fn method_11(&mut self) {
    }

    /// Decompiled virtual method #12 @ RVA 0x001e9580 (53 instructions, 12 basic blocks)
    pub fn method_12(&mut self) {
    }

    /// Decompiled virtual method #13 @ RVA 0x00103e60 (2 instructions, 1 basic blocks)
    pub fn method_13(&mut self) {
    }

    /// Decompiled virtual method #14 @ RVA 0x00103e60 (2 instructions, 1 basic blocks)
    pub fn method_14(&mut self) {
    }

    /// Decompiled virtual method #15 @ RVA 0x00451dc0 (10 instructions, 2 basic blocks)
    pub fn method_15(&mut self) {
    }

    /// Decompiled virtual method #16 @ RVA 0x00451e30 (10 instructions, 2 basic blocks)
    pub fn method_16(&mut self) {
    }

    /// Decompiled virtual method #17 @ RVA 0x00451ea0 (31 instructions, 2 basic blocks)
    pub fn method_17(&mut self) {
        // Writes to field at offset +0x0018
    }

    /// Decompiled virtual method #18 @ RVA 0x00451ee0 (10 instructions, 2 basic blocks)
    pub fn method_18(&mut self) {
    }

    /// Decompiled virtual method #19 @ RVA 0x00451f40 (10 instructions, 2 basic blocks)
    pub fn method_19(&mut self) {
    }

    /// Decompiled virtual method #20 @ RVA 0x00620860 (9 instructions, 2 basic blocks)
    pub fn method_20(&mut self) {
        // Writes to field at offset +0x0018
    }

    /// Decompiled virtual method #21 @ RVA 0x00620bd0 (9 instructions, 2 basic blocks)
    pub fn method_21(&mut self) {
        // Writes to field at offset +0x0018
    }

    /// Decompiled virtual method #22 @ RVA 0x00620d30 (6 instructions, 1 basic blocks)
    pub fn method_22(&mut self) {
    }

    /// Decompiled virtual method #23 @ RVA 0x00103e60 (2 instructions, 1 basic blocks)
    pub fn method_23(&mut self) {
    }

    /// Decompiled virtual method #24 @ RVA 0x00103e60 (2 instructions, 1 basic blocks)
    pub fn method_24(&mut self) {
    }

    /// Decompiled virtual method #25 @ RVA 0x00103e60 (2 instructions, 1 basic blocks)
    pub fn method_25(&mut self) {
    }

    /// Decompiled virtual method #26 @ RVA 0x00620d50 (5 instructions, 1 basic blocks)
    pub fn method_26(&mut self) {
    }

    /// Decompiled virtual method #27 @ RVA 0x00103e60 (2 instructions, 1 basic blocks)
    pub fn method_27(&mut self) {
    }

    /// Decompiled virtual method #28 @ RVA 0x00103e80 (2 instructions, 1 basic blocks)
    pub fn method_28(&mut self) {
    }

    /// Decompiled virtual method #29 @ RVA 0x00103e80 (2 instructions, 1 basic blocks)
    pub fn method_29(&mut self) {
    }

    /// Decompiled virtual method #30 @ RVA 0x00620170 (107 instructions, 18 basic blocks)
    pub fn method_30(&mut self) {
        // Writes to field at offset +0x0018
        // Writes to field at offset +0x001a
    }

    /// Decompiled virtual method #31 @ RVA 0x00620800 (41 instructions, 3 basic blocks)
    pub fn method_31(&mut self) {
        // Writes to field at offset +0x0018
    }

    /// Decompiled virtual method #32 @ RVA 0x000d3090 (1 instructions, 1 basic blocks)
    pub fn method_32(&mut self) {
    }

    /// Decompiled virtual method #33 @ RVA 0x00103e70 (2 instructions, 1 basic blocks)
    pub fn method_33(&mut self) {
    }

    /// Decompiled virtual method #34 @ RVA 0x00103e70 (2 instructions, 1 basic blocks)
    pub fn method_34(&mut self) {
    }

    /// Decompiled virtual method #35 @ RVA 0x00103e80 (2 instructions, 1 basic blocks)
    pub fn method_35(&mut self) {
    }

    /// Decompiled virtual method #36 @ RVA 0x00103e60 (2 instructions, 1 basic blocks)
    pub fn method_36(&mut self) {
    }

}

impl CcClass for ccSkillTrgEditWnd {
    fn class_name(&self) -> &'static str { Self::CLASS_NAME }
    fn vtable_rva(&self) -> u32 { Self::VTABLE_RVA }
    fn virtual_method_count(&self) -> usize { Self::METHOD_COUNT }
    fn base_classes(&self) -> &'static [&'static str] { Self::BASE_CLASSES }
}

/// Decompiled CC2 class: `ccSkillWnd` (VTable RVA: 0x008bafd8, 37 virtual methods, 19 recovered fields)
/// Inherits: ccGameMenuCtrlBase, ccGameMenuObj
#[derive(Debug, Clone, Default)]
pub struct ccSkillWnd {
    pub vtable_rva: u32,
    pub field_0x0000: u64,
    pub field_0x0008: u32,
    pub field_0x0010: u64,
    pub field_0x0014: u64,
    pub field_0x0018: u32,
    pub field_0x0024: u64,
    pub field_0x0028: u32,
    pub field_0x002c: u32,
    pub field_0x0030: u16,
    pub field_0x0038: u32,
    pub field_0x0048: u64,
    pub field_0x0050: u64,
    pub field_0x006c: u64,
    pub field_0x0074: u16,
    pub field_0x008c: u16,
    pub field_0x009c: u32,
    pub field_0x00e0: u64,
    pub field_0x0100: u64,
    pub field_0x0128: u64,
}

impl ccSkillWnd {
    pub const CLASS_NAME: &'static str = "ccSkillWnd";
    pub const VTABLE_RVA: u32 = 0x008bafd8;
    pub const METHOD_COUNT: usize = 37;
    pub const BASE_CLASSES: &'static [&'static str] = &["ccGameMenuCtrlBase", "ccGameMenuObj"];
    pub const VIRTUAL_METHODS: &'static [u64] = &[0x000000018044fe60, 0x00000001804520b0, 0x0000000180452130, 0x0000000180451fb0, 0x0000000180451ff0, 0x0000000180452030, 0x0000000180452070, 0x0000000180103e90, 0x0000000180103ea0, 0x0000000180103e60, 0x000000018045bb30, 0x0000000180451d90, 0x00000001801e9580, 0x0000000180103e60, 0x0000000180103e60, 0x0000000180451dc0, 0x0000000180451e30, 0x0000000180451ea0, 0x0000000180451ee0, 0x0000000180451f40, 0x00000001806272b0, 0x0000000180627800, 0x0000000180627990, 0x0000000180103e60, 0x0000000180103e60, 0x0000000180627a30, 0x0000000180627ce0, 0x0000000180103e60, 0x0000000180627e90, 0x00000001806280f0, 0x0000000180625780, 0x0000000180625d40, 0x00000001800d3090, 0x0000000180103e70, 0x0000000180103e70, 0x0000000180103e70, 0x0000000180103e60];

    pub fn new() -> Self {
        Self {
            vtable_rva: Self::VTABLE_RVA,
            field_0x0000: Default::default(),
            field_0x0008: Default::default(),
            field_0x0010: Default::default(),
            field_0x0014: Default::default(),
            field_0x0018: Default::default(),
            field_0x0024: Default::default(),
            field_0x0028: Default::default(),
            field_0x002c: Default::default(),
            field_0x0030: Default::default(),
            field_0x0038: Default::default(),
            field_0x0048: Default::default(),
            field_0x0050: Default::default(),
            field_0x006c: Default::default(),
            field_0x0074: Default::default(),
            field_0x008c: Default::default(),
            field_0x009c: Default::default(),
            field_0x00e0: Default::default(),
            field_0x0100: Default::default(),
            field_0x0128: Default::default(),
        }
    }

    /// Decompiled virtual method #0 @ RVA 0x0044fe60 (34 instructions, 3 basic blocks)
    pub fn method_00(&mut self) {
        // Writes to field at offset +0x0000
    }

    /// Decompiled virtual method #1 @ RVA 0x004520b0 (10 instructions, 2 basic blocks)
    pub fn method_01(&mut self) {
    }

    /// Decompiled virtual method #2 @ RVA 0x00452130 (35 instructions, 9 basic blocks)
    pub fn method_02(&mut self) {
    }

    /// Decompiled virtual method #3 @ RVA 0x00451fb0 (118 instructions, 2 basic blocks)
    pub fn method_03(&mut self) {
    }

    /// Decompiled virtual method #4 @ RVA 0x00451ff0 (91 instructions, 2 basic blocks)
    pub fn method_04(&mut self) {
    }

    /// Decompiled virtual method #5 @ RVA 0x00452030 (64 instructions, 2 basic blocks)
    pub fn method_05(&mut self) {
    }

    /// Decompiled virtual method #6 @ RVA 0x00452070 (37 instructions, 2 basic blocks)
    pub fn method_06(&mut self) {
    }

    /// Decompiled virtual method #7 @ RVA 0x00103e90 (2 instructions, 1 basic blocks)
    pub fn method_07(&mut self) {
    }

    /// Decompiled virtual method #8 @ RVA 0x00103ea0 (9 instructions, 1 basic blocks)
    pub fn method_08(&mut self) {
        // Writes to field at offset +0x0008
    }

    /// Decompiled virtual method #9 @ RVA 0x00103e60 (2 instructions, 1 basic blocks)
    pub fn method_09(&mut self) {
    }

    /// Decompiled virtual method #10 @ RVA 0x0045bb30 (16 instructions, 2 basic blocks)
    pub fn method_10(&mut self) {
    }

    /// Decompiled virtual method #11 @ RVA 0x00451d90 (24 instructions, 2 basic blocks)
    pub fn method_11(&mut self) {
    }

    /// Decompiled virtual method #12 @ RVA 0x001e9580 (53 instructions, 12 basic blocks)
    pub fn method_12(&mut self) {
    }

    /// Decompiled virtual method #13 @ RVA 0x00103e60 (2 instructions, 1 basic blocks)
    pub fn method_13(&mut self) {
    }

    /// Decompiled virtual method #14 @ RVA 0x00103e60 (2 instructions, 1 basic blocks)
    pub fn method_14(&mut self) {
    }

    /// Decompiled virtual method #15 @ RVA 0x00451dc0 (10 instructions, 2 basic blocks)
    pub fn method_15(&mut self) {
    }

    /// Decompiled virtual method #16 @ RVA 0x00451e30 (10 instructions, 2 basic blocks)
    pub fn method_16(&mut self) {
    }

    /// Decompiled virtual method #17 @ RVA 0x00451ea0 (31 instructions, 2 basic blocks)
    pub fn method_17(&mut self) {
        // Writes to field at offset +0x0018
    }

    /// Decompiled virtual method #18 @ RVA 0x00451ee0 (10 instructions, 2 basic blocks)
    pub fn method_18(&mut self) {
    }

    /// Decompiled virtual method #19 @ RVA 0x00451f40 (10 instructions, 2 basic blocks)
    pub fn method_19(&mut self) {
    }

    /// Decompiled virtual method #20 @ RVA 0x006272b0 (122 instructions, 23 basic blocks)
    pub fn method_20(&mut self) {
        // Writes to field at offset +0x002c
        // Writes to field at offset +0x006c
        // Writes to field at offset +0x0074
        // Writes to field at offset +0x008c
        // Writes to field at offset +0x008c
    }

    /// Decompiled virtual method #21 @ RVA 0x00627800 (96 instructions, 21 basic blocks)
    pub fn method_21(&mut self) {
        // Writes to field at offset +0x0000
        // Writes to field at offset +0x0000
        // Writes to field at offset +0x0008
        // Writes to field at offset +0x0010
        // Writes to field at offset +0x0018
        // Writes to field at offset +0x0038
        // Writes to field at offset +0x0048
    }

    /// Decompiled virtual method #22 @ RVA 0x00627990 (34 instructions, 3 basic blocks)
    pub fn method_22(&mut self) {
    }

    /// Decompiled virtual method #23 @ RVA 0x00103e60 (2 instructions, 1 basic blocks)
    pub fn method_23(&mut self) {
    }

    /// Decompiled virtual method #24 @ RVA 0x00103e60 (2 instructions, 1 basic blocks)
    pub fn method_24(&mut self) {
    }

    /// Decompiled virtual method #25 @ RVA 0x00627a30 (14 instructions, 4 basic blocks)
    pub fn method_25(&mut self) {
        // Writes to field at offset +0x0010
        // Writes to field at offset +0x0018
    }

    /// Decompiled virtual method #26 @ RVA 0x00627ce0 (65 instructions, 9 basic blocks)
    pub fn method_26(&mut self) {
        // Writes to field at offset +0x0018
    }

    /// Decompiled virtual method #27 @ RVA 0x00103e60 (2 instructions, 1 basic blocks)
    pub fn method_27(&mut self) {
    }

    /// Decompiled virtual method #28 @ RVA 0x00627e90 (36 instructions, 3 basic blocks)
    pub fn method_28(&mut self) {
        // Writes to field at offset +0x0010
    }

    /// Decompiled virtual method #29 @ RVA 0x006280f0 (113 instructions, 22 basic blocks)
    pub fn method_29(&mut self) {
        // Writes to field at offset +0x0010
    }

    /// Decompiled virtual method #30 @ RVA 0x00625780 (10 instructions, 2 basic blocks)
    pub fn method_30(&mut self) {
    }

    /// Decompiled virtual method #31 @ RVA 0x00625d40 (97 instructions, 8 basic blocks)
    pub fn method_31(&mut self) {
        // Writes to field at offset +0x0010
        // Writes to field at offset +0x0024
    }

    /// Decompiled virtual method #32 @ RVA 0x000d3090 (1 instructions, 1 basic blocks)
    pub fn method_32(&mut self) {
    }

    /// Decompiled virtual method #33 @ RVA 0x00103e70 (2 instructions, 1 basic blocks)
    pub fn method_33(&mut self) {
    }

    /// Decompiled virtual method #34 @ RVA 0x00103e70 (2 instructions, 1 basic blocks)
    pub fn method_34(&mut self) {
    }

    /// Decompiled virtual method #35 @ RVA 0x00103e70 (2 instructions, 1 basic blocks)
    pub fn method_35(&mut self) {
    }

    /// Decompiled virtual method #36 @ RVA 0x00103e60 (2 instructions, 1 basic blocks)
    pub fn method_36(&mut self) {
    }

}

impl CcClass for ccSkillWnd {
    fn class_name(&self) -> &'static str { Self::CLASS_NAME }
    fn vtable_rva(&self) -> u32 { Self::VTABLE_RVA }
    fn virtual_method_count(&self) -> usize { Self::METHOD_COUNT }
    fn base_classes(&self) -> &'static [&'static str] { Self::BASE_CLASSES }
}

/// Decompiled CC2 class: `ccSpGauge` (VTable RVA: 0x008a45e8, 6 virtual methods, 16 recovered fields)
/// Inherits: ccHpGauge, ccGauge
#[derive(Debug, Clone, Default)]
pub struct ccSpGauge {
    pub vtable_rva: u32,
    pub field_0x0000: u64,
    pub field_0x0008: f32,
    pub field_0x000c: f32,
    pub field_0x0010: f32,
    pub field_0x0014: f32,
    pub field_0x0018: f32,
    pub field_0x001c: f32,
    pub field_0x0022: u16,
    pub field_0x0024: f32,
    pub field_0x0028: u32,
    pub field_0x002c: f32,
    pub field_0x0030: u32,
    pub field_0x0038: u64,
    pub field_0x0040: u64,
    pub field_0x0048: u64,
    pub field_0x2bd8: u32,
}

impl ccSpGauge {
    pub const CLASS_NAME: &'static str = "ccSpGauge";
    pub const VTABLE_RVA: u32 = 0x008a45e8;
    pub const METHOD_COUNT: usize = 6;
    pub const BASE_CLASSES: &'static [&'static str] = &["ccHpGauge", "ccGauge"];
    pub const VIRTUAL_METHODS: &'static [u64] = &[0x000000018029ff10, 0x00000001802a4320, 0x00000001802340b0, 0x00000001802a3ef0, 0x00000001802a4090, 0x00000001801b8190];

    pub fn new() -> Self {
        Self {
            vtable_rva: Self::VTABLE_RVA,
            field_0x0000: Default::default(),
            field_0x0008: Default::default(),
            field_0x000c: Default::default(),
            field_0x0010: Default::default(),
            field_0x0014: Default::default(),
            field_0x0018: Default::default(),
            field_0x001c: Default::default(),
            field_0x0022: Default::default(),
            field_0x0024: Default::default(),
            field_0x0028: Default::default(),
            field_0x002c: Default::default(),
            field_0x0030: Default::default(),
            field_0x0038: Default::default(),
            field_0x0040: Default::default(),
            field_0x0048: Default::default(),
            field_0x2bd8: Default::default(),
        }
    }

    /// Decompiled virtual method #0 @ RVA 0x0029ff10 (18 instructions, 3 basic blocks)
    pub fn method_00(&mut self) {
        // Writes to field at offset +0x0000
    }

    /// Decompiled virtual method #1 @ RVA 0x002a4320 (40 instructions, 3 basic blocks)
    pub fn method_01(&mut self) {
    }

    /// Decompiled virtual method #2 @ RVA 0x002340b0 (21 instructions, 7 basic blocks)
    pub fn method_02(&mut self) {
    }

    /// Decompiled virtual method #3 @ RVA 0x002a3ef0 (58 instructions, 17 basic blocks)
    pub fn method_03(&mut self) {
        // Writes to field at offset +0x0022
        // Writes to field at offset +0x0028
        // Writes to field at offset +0x002c
        // Writes to field at offset +0x0030
    }

    /// Decompiled virtual method #4 @ RVA 0x002a4090 (108 instructions, 14 basic blocks)
    pub fn method_04(&mut self) {
    }

    /// Decompiled virtual method #5 @ RVA 0x001b8190 (12 instructions, 1 basic blocks)
    pub fn method_05(&mut self) {
        // Writes to field at offset +0x0024
        // Writes to field at offset +0x0028
        // Writes to field at offset +0x002c
    }

}

impl CcClass for ccSpGauge {
    fn class_name(&self) -> &'static str { Self::CLASS_NAME }
    fn vtable_rva(&self) -> u32 { Self::VTABLE_RVA }
    fn virtual_method_count(&self) -> usize { Self::METHOD_COUNT }
    fn base_classes(&self) -> &'static [&'static str] { Self::BASE_CLASSES }
}

/// Decompiled CC2 class: `ccStatusPanelDispCtrl` (VTable RVA: 0x008a42e0, 14 virtual methods, 15 recovered fields)
/// Inherits: ccDispBase
#[derive(Debug, Clone, Default)]
pub struct ccStatusPanelDispCtrl {
    pub vtable_rva: u32,
    pub field_0x0000: u64,
    pub field_0x0008: u32,
    pub field_0x000c: u32,
    pub field_0x0010: u32,
    pub field_0x0014: f32,
    pub field_0x0018: f32,
    pub field_0x001a: u8,
    pub field_0x0028: u32,
    pub field_0x002c: f32,
    pub field_0x0030: u32,
    pub field_0x0034: f32,
    pub field_0x0038: u64,
    pub field_0x0040: f32,
    pub field_0x0044: u32,
    pub field_0x0080: u32,
}

impl ccStatusPanelDispCtrl {
    pub const CLASS_NAME: &'static str = "ccStatusPanelDispCtrl";
    pub const VTABLE_RVA: u32 = 0x008a42e0;
    pub const METHOD_COUNT: usize = 14;
    pub const BASE_CLASSES: &'static [&'static str] = &["ccDispBase"];
    pub const VIRTUAL_METHODS: &'static [u64] = &[0x00000001802a87a0, 0x00000001802a87d0, 0x00000001802a8990, 0x00000001802a8b90, 0x00000001802a0e30, 0x00000001800d3090, 0x00000001802a9660, 0x00000001801b8200, 0x00000001801b8210, 0x00000001800d3090, 0x00000001802a9580, 0x00000001802a94a0, 0x00000001800d3090, 0x00000001802a9510];

    pub fn new() -> Self {
        Self {
            vtable_rva: Self::VTABLE_RVA,
            field_0x0000: Default::default(),
            field_0x0008: Default::default(),
            field_0x000c: Default::default(),
            field_0x0010: Default::default(),
            field_0x0014: Default::default(),
            field_0x0018: Default::default(),
            field_0x001a: Default::default(),
            field_0x0028: Default::default(),
            field_0x002c: Default::default(),
            field_0x0030: Default::default(),
            field_0x0034: Default::default(),
            field_0x0038: Default::default(),
            field_0x0040: Default::default(),
            field_0x0044: Default::default(),
            field_0x0080: Default::default(),
        }
    }

    /// Decompiled virtual method #0 @ RVA 0x002a87a0 (13 instructions, 3 basic blocks)
    pub fn method_00(&mut self) {
        // Writes to field at offset +0x0000
    }

    /// Decompiled virtual method #1 @ RVA 0x002a87d0 (100 instructions, 12 basic blocks)
    pub fn method_01(&mut self) {
        // Writes to field at offset +0x001a
    }

    /// Decompiled virtual method #2 @ RVA 0x002a8990 (85 instructions, 22 basic blocks)
    pub fn method_02(&mut self) {
        // Writes to field at offset +0x0038
        // Writes to field at offset +0x0044
    }

    /// Decompiled virtual method #3 @ RVA 0x002a8b90 (119 instructions, 16 basic blocks)
    pub fn method_03(&mut self) {
        // Writes to field at offset +0x000c
        // Writes to field at offset +0x0010
        // Writes to field at offset +0x001a
        // Writes to field at offset +0x0028
        // Writes to field at offset +0x002c
        // Writes to field at offset +0x0030
        // Writes to field at offset +0x0034
        // Writes to field at offset +0x0040
        // Writes to field at offset +0x0040
        // Writes to field at offset +0x0044
    }

    /// Decompiled virtual method #4 @ RVA 0x002a0e30 (48 instructions, 11 basic blocks)
    pub fn method_04(&mut self) {
        // Writes to field at offset +0x0008
    }

    /// Decompiled virtual method #5 @ RVA 0x000d3090 (1 instructions, 1 basic blocks)
    pub fn method_05(&mut self) {
    }

    /// Decompiled virtual method #6 @ RVA 0x002a9660 (92 instructions, 24 basic blocks)
    pub fn method_06(&mut self) {
        // Writes to field at offset +0x0008
        // Writes to field at offset +0x0040
        // Writes to field at offset +0x0080
    }

    /// Decompiled virtual method #7 @ RVA 0x001b8200 (5 instructions, 1 basic blocks)
    pub fn method_07(&mut self) {
    }

    /// Decompiled virtual method #8 @ RVA 0x001b8210 (5 instructions, 1 basic blocks)
    pub fn method_08(&mut self) {
    }

    /// Decompiled virtual method #9 @ RVA 0x000d3090 (1 instructions, 1 basic blocks)
    pub fn method_09(&mut self) {
    }

    /// Decompiled virtual method #10 @ RVA 0x002a9580 (42 instructions, 8 basic blocks)
    pub fn method_10(&mut self) {
        // Writes to field at offset +0x0040
    }

    /// Decompiled virtual method #11 @ RVA 0x002a94a0 (30 instructions, 7 basic blocks)
    pub fn method_11(&mut self) {
    }

    /// Decompiled virtual method #12 @ RVA 0x000d3090 (1 instructions, 1 basic blocks)
    pub fn method_12(&mut self) {
    }

    /// Decompiled virtual method #13 @ RVA 0x002a9510 (30 instructions, 7 basic blocks)
    pub fn method_13(&mut self) {
    }

}

impl CcClass for ccStatusPanelDispCtrl {
    fn class_name(&self) -> &'static str { Self::CLASS_NAME }
    fn vtable_rva(&self) -> u32 { Self::VTABLE_RVA }
    fn virtual_method_count(&self) -> usize { Self::METHOD_COUNT }
    fn base_classes(&self) -> &'static [&'static str] { Self::BASE_CLASSES }
}

/// Decompiled CC2 class: `ccStatusWnd` (VTable RVA: 0x008bb238, 37 virtual methods, 15 recovered fields)
/// Inherits: ccGameMenuCtrlBase, ccGameMenuObj
#[derive(Debug, Clone, Default)]
pub struct ccStatusWnd {
    pub vtable_rva: u32,
    pub field_0x0000: u64,
    pub field_0x0008: u32,
    pub field_0x0010: u64,
    pub field_0x0018: u32,
    pub field_0x0048: u64,
    pub field_0x0050: u64,
    pub field_0x0060: u32,
    pub field_0x0064: u64,
    pub field_0x006c: u32,
    pub field_0x0071: u8,
    pub field_0x0073: u8,
    pub field_0x0078: u64,
    pub field_0x009c: u32,
    pub field_0x00d0: u64,
    pub field_0x018b: u16,
}

impl ccStatusWnd {
    pub const CLASS_NAME: &'static str = "ccStatusWnd";
    pub const VTABLE_RVA: u32 = 0x008bb238;
    pub const METHOD_COUNT: usize = 37;
    pub const BASE_CLASSES: &'static [&'static str] = &["ccGameMenuCtrlBase", "ccGameMenuObj"];
    pub const VIRTUAL_METHODS: &'static [u64] = &[0x000000018044fd70, 0x00000001804520b0, 0x0000000180452130, 0x0000000180451fb0, 0x0000000180451ff0, 0x0000000180452030, 0x0000000180452070, 0x0000000180103e90, 0x0000000180103ea0, 0x0000000180103e60, 0x000000018045bb30, 0x0000000180451d90, 0x00000001801e9580, 0x0000000180103e60, 0x0000000180103e60, 0x0000000180451dc0, 0x0000000180451e30, 0x0000000180451ea0, 0x0000000180451ee0, 0x0000000180451f40, 0x000000018062eb40, 0x000000018062edb0, 0x0000000180103e60, 0x0000000180103e60, 0x0000000180103e60, 0x0000000180103e60, 0x0000000180103e70, 0x0000000180103e70, 0x0000000180103e60, 0x0000000180103e60, 0x000000018062e550, 0x000000018062f8b0, 0x00000001800d3090, 0x0000000180103e70, 0x0000000180103e70, 0x0000000180103e80, 0x0000000180103e60];

    pub fn new() -> Self {
        Self {
            vtable_rva: Self::VTABLE_RVA,
            field_0x0000: Default::default(),
            field_0x0008: Default::default(),
            field_0x0010: Default::default(),
            field_0x0018: Default::default(),
            field_0x0048: Default::default(),
            field_0x0050: Default::default(),
            field_0x0060: Default::default(),
            field_0x0064: Default::default(),
            field_0x006c: Default::default(),
            field_0x0071: Default::default(),
            field_0x0073: Default::default(),
            field_0x0078: Default::default(),
            field_0x009c: Default::default(),
            field_0x00d0: Default::default(),
            field_0x018b: Default::default(),
        }
    }

    /// Decompiled virtual method #0 @ RVA 0x0044fd70 (34 instructions, 3 basic blocks)
    pub fn method_00(&mut self) {
        // Writes to field at offset +0x0000
    }

    /// Decompiled virtual method #1 @ RVA 0x004520b0 (10 instructions, 2 basic blocks)
    pub fn method_01(&mut self) {
    }

    /// Decompiled virtual method #2 @ RVA 0x00452130 (35 instructions, 9 basic blocks)
    pub fn method_02(&mut self) {
    }

    /// Decompiled virtual method #3 @ RVA 0x00451fb0 (118 instructions, 2 basic blocks)
    pub fn method_03(&mut self) {
    }

    /// Decompiled virtual method #4 @ RVA 0x00451ff0 (91 instructions, 2 basic blocks)
    pub fn method_04(&mut self) {
    }

    /// Decompiled virtual method #5 @ RVA 0x00452030 (64 instructions, 2 basic blocks)
    pub fn method_05(&mut self) {
    }

    /// Decompiled virtual method #6 @ RVA 0x00452070 (37 instructions, 2 basic blocks)
    pub fn method_06(&mut self) {
    }

    /// Decompiled virtual method #7 @ RVA 0x00103e90 (2 instructions, 1 basic blocks)
    pub fn method_07(&mut self) {
    }

    /// Decompiled virtual method #8 @ RVA 0x00103ea0 (9 instructions, 1 basic blocks)
    pub fn method_08(&mut self) {
        // Writes to field at offset +0x0008
    }

    /// Decompiled virtual method #9 @ RVA 0x00103e60 (2 instructions, 1 basic blocks)
    pub fn method_09(&mut self) {
    }

    /// Decompiled virtual method #10 @ RVA 0x0045bb30 (16 instructions, 2 basic blocks)
    pub fn method_10(&mut self) {
    }

    /// Decompiled virtual method #11 @ RVA 0x00451d90 (24 instructions, 2 basic blocks)
    pub fn method_11(&mut self) {
    }

    /// Decompiled virtual method #12 @ RVA 0x001e9580 (53 instructions, 12 basic blocks)
    pub fn method_12(&mut self) {
    }

    /// Decompiled virtual method #13 @ RVA 0x00103e60 (2 instructions, 1 basic blocks)
    pub fn method_13(&mut self) {
    }

    /// Decompiled virtual method #14 @ RVA 0x00103e60 (2 instructions, 1 basic blocks)
    pub fn method_14(&mut self) {
    }

    /// Decompiled virtual method #15 @ RVA 0x00451dc0 (10 instructions, 2 basic blocks)
    pub fn method_15(&mut self) {
    }

    /// Decompiled virtual method #16 @ RVA 0x00451e30 (10 instructions, 2 basic blocks)
    pub fn method_16(&mut self) {
    }

    /// Decompiled virtual method #17 @ RVA 0x00451ea0 (31 instructions, 2 basic blocks)
    pub fn method_17(&mut self) {
        // Writes to field at offset +0x0018
    }

    /// Decompiled virtual method #18 @ RVA 0x00451ee0 (10 instructions, 2 basic blocks)
    pub fn method_18(&mut self) {
    }

    /// Decompiled virtual method #19 @ RVA 0x00451f40 (10 instructions, 2 basic blocks)
    pub fn method_19(&mut self) {
    }

    /// Decompiled virtual method #20 @ RVA 0x0062eb40 (34 instructions, 2 basic blocks)
    pub fn method_20(&mut self) {
        // Writes to field at offset +0x0060
        // Writes to field at offset +0x0064
        // Writes to field at offset +0x006c
        // Writes to field at offset +0x0071
        // Writes to field at offset +0x0073
    }

    /// Decompiled virtual method #21 @ RVA 0x0062edb0 (76 instructions, 12 basic blocks)
    pub fn method_21(&mut self) {
        // Writes to field at offset +0x0000
        // Writes to field at offset +0x0000
        // Writes to field at offset +0x0008
        // Writes to field at offset +0x0010
        // Writes to field at offset +0x0018
        // Writes to field at offset +0x0064
        // Writes to field at offset +0x018b
    }

    /// Decompiled virtual method #22 @ RVA 0x00103e60 (2 instructions, 1 basic blocks)
    pub fn method_22(&mut self) {
    }

    /// Decompiled virtual method #23 @ RVA 0x00103e60 (2 instructions, 1 basic blocks)
    pub fn method_23(&mut self) {
    }

    /// Decompiled virtual method #24 @ RVA 0x00103e60 (2 instructions, 1 basic blocks)
    pub fn method_24(&mut self) {
    }

    /// Decompiled virtual method #25 @ RVA 0x00103e60 (2 instructions, 1 basic blocks)
    pub fn method_25(&mut self) {
    }

    /// Decompiled virtual method #26 @ RVA 0x00103e70 (2 instructions, 1 basic blocks)
    pub fn method_26(&mut self) {
    }

    /// Decompiled virtual method #27 @ RVA 0x00103e70 (2 instructions, 1 basic blocks)
    pub fn method_27(&mut self) {
    }

    /// Decompiled virtual method #28 @ RVA 0x00103e60 (2 instructions, 1 basic blocks)
    pub fn method_28(&mut self) {
    }

    /// Decompiled virtual method #29 @ RVA 0x00103e60 (2 instructions, 1 basic blocks)
    pub fn method_29(&mut self) {
    }

    /// Decompiled virtual method #30 @ RVA 0x0062e550 (122 instructions, 25 basic blocks)
    pub fn method_30(&mut self) {
        // Writes to field at offset +0x0018
        // Writes to field at offset +0x018b
    }

    /// Decompiled virtual method #31 @ RVA 0x0062f8b0 (31 instructions, 7 basic blocks)
    pub fn method_31(&mut self) {
    }

    /// Decompiled virtual method #32 @ RVA 0x000d3090 (1 instructions, 1 basic blocks)
    pub fn method_32(&mut self) {
    }

    /// Decompiled virtual method #33 @ RVA 0x00103e70 (2 instructions, 1 basic blocks)
    pub fn method_33(&mut self) {
    }

    /// Decompiled virtual method #34 @ RVA 0x00103e70 (2 instructions, 1 basic blocks)
    pub fn method_34(&mut self) {
    }

    /// Decompiled virtual method #35 @ RVA 0x00103e80 (2 instructions, 1 basic blocks)
    pub fn method_35(&mut self) {
    }

    /// Decompiled virtual method #36 @ RVA 0x00103e60 (2 instructions, 1 basic blocks)
    pub fn method_36(&mut self) {
    }

}

impl CcClass for ccStatusWnd {
    fn class_name(&self) -> &'static str { Self::CLASS_NAME }
    fn vtable_rva(&self) -> u32 { Self::VTABLE_RVA }
    fn virtual_method_count(&self) -> usize { Self::METHOD_COUNT }
    fn base_classes(&self) -> &'static [&'static str] { Self::BASE_CLASSES }
}

/// Decompiled CC2 class: `ccStraightGauge` (VTable RVA: 0x00881408, 6 virtual methods, 13 recovered fields)
/// Inherits: ccGauge
#[derive(Debug, Clone, Default)]
pub struct ccStraightGauge {
    pub vtable_rva: u32,
    pub field_0x0000: u64,
    pub field_0x0024: f32,
    pub field_0x0028: f32,
    pub field_0x002c: f32,
    pub field_0x0038: f32,
    pub field_0x0040: u64,
    pub field_0x0048: u32,
    pub field_0x0058: u64,
    pub field_0x008c: u32,
    pub field_0x0110: u64,
    pub field_0x0360: u32,
    pub field_0x0368: u8,
    pub field_0x0398: [f32; 4],
}

impl ccStraightGauge {
    pub const CLASS_NAME: &'static str = "ccStraightGauge";
    pub const VTABLE_RVA: u32 = 0x00881408;
    pub const METHOD_COUNT: usize = 6;
    pub const BASE_CLASSES: &'static [&'static str] = &["ccGauge"];
    pub const VIRTUAL_METHODS: &'static [u64] = &[0x000000018023a6f0, 0x000000018023a720, 0x000000018023a970, 0x000000018023aa10, 0x000000018023ab80, 0x00000001801b8190];

    pub fn new() -> Self {
        Self {
            vtable_rva: Self::VTABLE_RVA,
            field_0x0000: Default::default(),
            field_0x0024: Default::default(),
            field_0x0028: Default::default(),
            field_0x002c: Default::default(),
            field_0x0038: Default::default(),
            field_0x0040: Default::default(),
            field_0x0048: Default::default(),
            field_0x0058: Default::default(),
            field_0x008c: Default::default(),
            field_0x0110: Default::default(),
            field_0x0360: Default::default(),
            field_0x0368: Default::default(),
            field_0x0398: Default::default(),
        }
    }

    /// Decompiled virtual method #0 @ RVA 0x0023a6f0 (13 instructions, 3 basic blocks)
    pub fn method_00(&mut self) {
        // Writes to field at offset +0x0000
    }

    /// Decompiled virtual method #1 @ RVA 0x0023a720 (113 instructions, 21 basic blocks)
    pub fn method_01(&mut self) {
    }

    /// Decompiled virtual method #2 @ RVA 0x0023a970 (43 instructions, 11 basic blocks)
    pub fn method_02(&mut self) {
    }

    /// Decompiled virtual method #3 @ RVA 0x0023aa10 (74 instructions, 24 basic blocks)
    pub fn method_03(&mut self) {
        // Writes to field at offset +0x008c
    }

    /// Decompiled virtual method #4 @ RVA 0x0023ab80 (94 instructions, 6 basic blocks)
    pub fn method_04(&mut self) {
        // Writes to field at offset +0x0048
        // Writes to field at offset +0x008c
        // Writes to field at offset +0x0360
        // Writes to field at offset +0x0368
        // Writes to field at offset +0x0398
    }

    /// Decompiled virtual method #5 @ RVA 0x001b8190 (12 instructions, 1 basic blocks)
    pub fn method_05(&mut self) {
        // Writes to field at offset +0x0024
        // Writes to field at offset +0x0028
        // Writes to field at offset +0x002c
    }

}

impl CcClass for ccStraightGauge {
    fn class_name(&self) -> &'static str { Self::CLASS_NAME }
    fn vtable_rva(&self) -> u32 { Self::VTABLE_RVA }
    fn virtual_method_count(&self) -> usize { Self::METHOD_COUNT }
    fn base_classes(&self) -> &'static [&'static str] { Self::BASE_CLASSES }
}

/// Decompiled CC2 class: `ccSysMenuItem` (VTable RVA: 0x00877988, 4 virtual methods, 1 recovered fields)
#[derive(Debug, Clone, Default)]
pub struct ccSysMenuItem {
    pub vtable_rva: u32,
    pub field_0x0000: u64,
}

impl ccSysMenuItem {
    pub const CLASS_NAME: &'static str = "ccSysMenuItem";
    pub const VTABLE_RVA: u32 = 0x00877988;
    pub const METHOD_COUNT: usize = 4;
    pub const BASE_CLASSES: &'static [&'static str] = &[];
    pub const VIRTUAL_METHODS: &'static [u64] = &[0x000000018018e0e0, 0x00000001800d3090, 0x00000001800d3090, 0x0000000180193680];

    pub fn new() -> Self {
        Self {
            vtable_rva: Self::VTABLE_RVA,
            field_0x0000: Default::default(),
        }
    }

    /// Decompiled virtual method #0 @ RVA 0x0018e0e0 (13 instructions, 3 basic blocks)
    pub fn method_00(&mut self) {
        // Writes to field at offset +0x0000
    }

    /// Decompiled virtual method #1 @ RVA 0x000d3090 (1 instructions, 1 basic blocks)
    pub fn method_01(&mut self) {
    }

    /// Decompiled virtual method #2 @ RVA 0x000d3090 (1 instructions, 1 basic blocks)
    pub fn method_02(&mut self) {
    }

    /// Decompiled virtual method #3 @ RVA 0x00193680 (5 instructions, 1 basic blocks)
    pub fn method_03(&mut self) {
    }

}

impl CcClass for ccSysMenuItem {
    fn class_name(&self) -> &'static str { Self::CLASS_NAME }
    fn vtable_rva(&self) -> u32 { Self::VTABLE_RVA }
    fn virtual_method_count(&self) -> usize { Self::METHOD_COUNT }
    fn base_classes(&self) -> &'static [&'static str] { Self::BASE_CLASSES }
}

/// Decompiled CC2 class: `ccSysMenuItemOnOff` (VTable RVA: 0x008779b0, 4 virtual methods, 1 recovered fields)
/// Inherits: ccSysMenuItem
#[derive(Debug, Clone, Default)]
pub struct ccSysMenuItemOnOff {
    pub vtable_rva: u32,
    pub field_0x0000: u64,
}

impl ccSysMenuItemOnOff {
    pub const CLASS_NAME: &'static str = "ccSysMenuItemOnOff";
    pub const VTABLE_RVA: u32 = 0x008779b0;
    pub const METHOD_COUNT: usize = 4;
    pub const BASE_CLASSES: &'static [&'static str] = &["ccSysMenuItem"];
    pub const VIRTUAL_METHODS: &'static [u64] = &[0x000000018018e110, 0x00000001800d3090, 0x00000001800d3090, 0x0000000180193690];

    pub fn new() -> Self {
        Self {
            vtable_rva: Self::VTABLE_RVA,
            field_0x0000: Default::default(),
        }
    }

    /// Decompiled virtual method #0 @ RVA 0x0018e110 (13 instructions, 3 basic blocks)
    pub fn method_00(&mut self) {
        // Writes to field at offset +0x0000
    }

    /// Decompiled virtual method #1 @ RVA 0x000d3090 (1 instructions, 1 basic blocks)
    pub fn method_01(&mut self) {
    }

    /// Decompiled virtual method #2 @ RVA 0x000d3090 (1 instructions, 1 basic blocks)
    pub fn method_02(&mut self) {
    }

    /// Decompiled virtual method #3 @ RVA 0x00193690 (95 instructions, 21 basic blocks)
    pub fn method_03(&mut self) {
    }

}

impl CcClass for ccSysMenuItemOnOff {
    fn class_name(&self) -> &'static str { Self::CLASS_NAME }
    fn vtable_rva(&self) -> u32 { Self::VTABLE_RVA }
    fn virtual_method_count(&self) -> usize { Self::METHOD_COUNT }
    fn base_classes(&self) -> &'static [&'static str] { Self::BASE_CLASSES }
}

/// Decompiled CC2 class: `ccSysMsgDispMenuCtrl` (VTable RVA: 0x0087d878, 37 virtual methods, 12 recovered fields)
/// Inherits: ccGameMenuCtrlBase, ccGameMenuObj
#[derive(Debug, Clone, Default)]
pub struct ccSysMsgDispMenuCtrl {
    pub vtable_rva: u32,
    pub field_0x0000: u64,
    pub field_0x0004: u32,
    pub field_0x0008: u32,
    pub field_0x000c: u8,
    pub field_0x0018: u32,
    pub field_0x0038: u32,
    pub field_0x0048: u64,
    pub field_0x0050: u64,
    pub field_0x0060: u64,
    pub field_0x0068: u64,
    pub field_0x0070: u32,
    pub field_0x009c: u32,
}

impl ccSysMsgDispMenuCtrl {
    pub const CLASS_NAME: &'static str = "ccSysMsgDispMenuCtrl";
    pub const VTABLE_RVA: u32 = 0x0087d878;
    pub const METHOD_COUNT: usize = 37;
    pub const BASE_CLASSES: &'static [&'static str] = &["ccGameMenuCtrlBase", "ccGameMenuObj"];
    pub const VIRTUAL_METHODS: &'static [u64] = &[0x00000001801f0af0, 0x00000001804520b0, 0x0000000180452130, 0x0000000180451fb0, 0x0000000180451ff0, 0x0000000180452030, 0x0000000180452070, 0x0000000180103e90, 0x0000000180103ea0, 0x0000000180103e60, 0x000000018045bb30, 0x0000000180451d90, 0x00000001801e9580, 0x00000001801f0be0, 0x0000000180103e60, 0x0000000180451dc0, 0x0000000180451e30, 0x0000000180451ea0, 0x0000000180451ee0, 0x0000000180451f40, 0x00000001801f0cb0, 0x00000001801f0d00, 0x0000000180103e70, 0x0000000180103e60, 0x0000000180103e60, 0x0000000180103e70, 0x0000000180103e70, 0x0000000180103e60, 0x0000000180103e60, 0x0000000180103e60, 0x00000001801f0d90, 0x00000001801f0950, 0x00000001800d3090, 0x0000000180103e70, 0x0000000180103e70, 0x0000000180103e70, 0x0000000180103e60];

    pub fn new() -> Self {
        Self {
            vtable_rva: Self::VTABLE_RVA,
            field_0x0000: Default::default(),
            field_0x0004: Default::default(),
            field_0x0008: Default::default(),
            field_0x000c: Default::default(),
            field_0x0018: Default::default(),
            field_0x0038: Default::default(),
            field_0x0048: Default::default(),
            field_0x0050: Default::default(),
            field_0x0060: Default::default(),
            field_0x0068: Default::default(),
            field_0x0070: Default::default(),
            field_0x009c: Default::default(),
        }
    }

    /// Decompiled virtual method #0 @ RVA 0x001f0af0 (16 instructions, 3 basic blocks)
    pub fn method_00(&mut self) {
    }

    /// Decompiled virtual method #1 @ RVA 0x004520b0 (10 instructions, 2 basic blocks)
    pub fn method_01(&mut self) {
    }

    /// Decompiled virtual method #2 @ RVA 0x00452130 (35 instructions, 9 basic blocks)
    pub fn method_02(&mut self) {
    }

    /// Decompiled virtual method #3 @ RVA 0x00451fb0 (118 instructions, 2 basic blocks)
    pub fn method_03(&mut self) {
    }

    /// Decompiled virtual method #4 @ RVA 0x00451ff0 (91 instructions, 2 basic blocks)
    pub fn method_04(&mut self) {
    }

    /// Decompiled virtual method #5 @ RVA 0x00452030 (64 instructions, 2 basic blocks)
    pub fn method_05(&mut self) {
    }

    /// Decompiled virtual method #6 @ RVA 0x00452070 (37 instructions, 2 basic blocks)
    pub fn method_06(&mut self) {
    }

    /// Decompiled virtual method #7 @ RVA 0x00103e90 (2 instructions, 1 basic blocks)
    pub fn method_07(&mut self) {
    }

    /// Decompiled virtual method #8 @ RVA 0x00103ea0 (9 instructions, 1 basic blocks)
    pub fn method_08(&mut self) {
        // Writes to field at offset +0x0008
    }

    /// Decompiled virtual method #9 @ RVA 0x00103e60 (2 instructions, 1 basic blocks)
    pub fn method_09(&mut self) {
    }

    /// Decompiled virtual method #10 @ RVA 0x0045bb30 (16 instructions, 2 basic blocks)
    pub fn method_10(&mut self) {
    }

    /// Decompiled virtual method #11 @ RVA 0x00451d90 (24 instructions, 2 basic blocks)
    pub fn method_11(&mut self) {
    }

    /// Decompiled virtual method #12 @ RVA 0x001e9580 (53 instructions, 12 basic blocks)
    pub fn method_12(&mut self) {
    }

    /// Decompiled virtual method #13 @ RVA 0x001f0be0 (54 instructions, 7 basic blocks)
    pub fn method_13(&mut self) {
        // Writes to field at offset +0x0000
        // Writes to field at offset +0x0004
        // Writes to field at offset +0x0008
        // Writes to field at offset +0x000c
    }

    /// Decompiled virtual method #14 @ RVA 0x00103e60 (2 instructions, 1 basic blocks)
    pub fn method_14(&mut self) {
    }

    /// Decompiled virtual method #15 @ RVA 0x00451dc0 (10 instructions, 2 basic blocks)
    pub fn method_15(&mut self) {
    }

    /// Decompiled virtual method #16 @ RVA 0x00451e30 (10 instructions, 2 basic blocks)
    pub fn method_16(&mut self) {
    }

    /// Decompiled virtual method #17 @ RVA 0x00451ea0 (31 instructions, 2 basic blocks)
    pub fn method_17(&mut self) {
        // Writes to field at offset +0x0018
    }

    /// Decompiled virtual method #18 @ RVA 0x00451ee0 (10 instructions, 2 basic blocks)
    pub fn method_18(&mut self) {
    }

    /// Decompiled virtual method #19 @ RVA 0x00451f40 (10 instructions, 2 basic blocks)
    pub fn method_19(&mut self) {
    }

    /// Decompiled virtual method #20 @ RVA 0x001f0cb0 (13 instructions, 3 basic blocks)
    pub fn method_20(&mut self) {
        // Writes to field at offset +0x0070
    }

    /// Decompiled virtual method #21 @ RVA 0x001f0d00 (30 instructions, 10 basic blocks)
    pub fn method_21(&mut self) {
        // Writes to field at offset +0x0038
        // Writes to field at offset +0x0048
    }

    /// Decompiled virtual method #22 @ RVA 0x00103e70 (2 instructions, 1 basic blocks)
    pub fn method_22(&mut self) {
    }

    /// Decompiled virtual method #23 @ RVA 0x00103e60 (2 instructions, 1 basic blocks)
    pub fn method_23(&mut self) {
    }

    /// Decompiled virtual method #24 @ RVA 0x00103e60 (2 instructions, 1 basic blocks)
    pub fn method_24(&mut self) {
    }

    /// Decompiled virtual method #25 @ RVA 0x00103e70 (2 instructions, 1 basic blocks)
    pub fn method_25(&mut self) {
    }

    /// Decompiled virtual method #26 @ RVA 0x00103e70 (2 instructions, 1 basic blocks)
    pub fn method_26(&mut self) {
    }

    /// Decompiled virtual method #27 @ RVA 0x00103e60 (2 instructions, 1 basic blocks)
    pub fn method_27(&mut self) {
    }

    /// Decompiled virtual method #28 @ RVA 0x00103e60 (2 instructions, 1 basic blocks)
    pub fn method_28(&mut self) {
    }

    /// Decompiled virtual method #29 @ RVA 0x00103e60 (2 instructions, 1 basic blocks)
    pub fn method_29(&mut self) {
    }

    /// Decompiled virtual method #30 @ RVA 0x001f0d90 (33 instructions, 5 basic blocks)
    pub fn method_30(&mut self) {
    }

    /// Decompiled virtual method #31 @ RVA 0x001f0950 (113 instructions, 10 basic blocks)
    pub fn method_31(&mut self) {
    }

    /// Decompiled virtual method #32 @ RVA 0x000d3090 (1 instructions, 1 basic blocks)
    pub fn method_32(&mut self) {
    }

    /// Decompiled virtual method #33 @ RVA 0x00103e70 (2 instructions, 1 basic blocks)
    pub fn method_33(&mut self) {
    }

    /// Decompiled virtual method #34 @ RVA 0x00103e70 (2 instructions, 1 basic blocks)
    pub fn method_34(&mut self) {
    }

    /// Decompiled virtual method #35 @ RVA 0x00103e70 (2 instructions, 1 basic blocks)
    pub fn method_35(&mut self) {
    }

    /// Decompiled virtual method #36 @ RVA 0x00103e60 (2 instructions, 1 basic blocks)
    pub fn method_36(&mut self) {
    }

}

impl CcClass for ccSysMsgDispMenuCtrl {
    fn class_name(&self) -> &'static str { Self::CLASS_NAME }
    fn vtable_rva(&self) -> u32 { Self::VTABLE_RVA }
    fn virtual_method_count(&self) -> usize { Self::METHOD_COUNT }
    fn base_classes(&self) -> &'static [&'static str] { Self::BASE_CLASSES }
}

/// Decompiled CC2 class: `ccTensionGauge` (VTable RVA: 0x00881398, 6 virtual methods, 8 recovered fields)
/// Inherits: ccGauge
#[derive(Debug, Clone, Default)]
pub struct ccTensionGauge {
    pub vtable_rva: u32,
    pub field_0x0000: u64,
    pub field_0x0024: f32,
    pub field_0x0028: f32,
    pub field_0x002c: f32,
    pub field_0x0038: u32,
    pub field_0x00b0: u64,
    pub field_0x00d8: u64,
    pub field_0x01f0: u8,
}

impl ccTensionGauge {
    pub const CLASS_NAME: &'static str = "ccTensionGauge";
    pub const VTABLE_RVA: u32 = 0x00881398;
    pub const METHOD_COUNT: usize = 6;
    pub const BASE_CLASSES: &'static [&'static str] = &["ccGauge"];
    pub const VIRTUAL_METHODS: &'static [u64] = &[0x000000018023bf50, 0x000000018023bfb0, 0x000000018023c460, 0x000000018023c5c0, 0x000000018023cb10, 0x0000000180232cc0];

    pub fn new() -> Self {
        Self {
            vtable_rva: Self::VTABLE_RVA,
            field_0x0000: Default::default(),
            field_0x0024: Default::default(),
            field_0x0028: Default::default(),
            field_0x002c: Default::default(),
            field_0x0038: Default::default(),
            field_0x00b0: Default::default(),
            field_0x00d8: Default::default(),
            field_0x01f0: Default::default(),
        }
    }

    /// Decompiled virtual method #0 @ RVA 0x0023bf50 (23 instructions, 3 basic blocks)
    pub fn method_00(&mut self) {
        // Writes to field at offset +0x0000
    }

    /// Decompiled virtual method #1 @ RVA 0x0023bfb0 (109 instructions, 19 basic blocks)
    pub fn method_01(&mut self) {
    }

    /// Decompiled virtual method #2 @ RVA 0x0023c460 (80 instructions, 25 basic blocks)
    pub fn method_02(&mut self) {
    }

    /// Decompiled virtual method #3 @ RVA 0x0023c5c0 (104 instructions, 23 basic blocks)
    pub fn method_03(&mut self) {
        // Writes to field at offset +0x0038
        // Writes to field at offset +0x00b0
    }

    /// Decompiled virtual method #4 @ RVA 0x0023cb10 (101 instructions, 3 basic blocks)
    pub fn method_04(&mut self) {
        // Writes to field at offset +0x0038
        // Writes to field at offset +0x00b0
    }

    /// Decompiled virtual method #5 @ RVA 0x00232cc0 (13 instructions, 1 basic blocks)
    pub fn method_05(&mut self) {
        // Writes to field at offset +0x0024
        // Writes to field at offset +0x0028
        // Writes to field at offset +0x002c
        // Writes to field at offset +0x01f0
    }

}

impl CcClass for ccTensionGauge {
    fn class_name(&self) -> &'static str { Self::CLASS_NAME }
    fn vtable_rva(&self) -> u32 { Self::VTABLE_RVA }
    fn virtual_method_count(&self) -> usize { Self::METHOD_COUNT }
    fn base_classes(&self) -> &'static [&'static str] { Self::BASE_CLASSES }
}

/// Decompiled CC2 class: `ccTriggerGauge` (VTable RVA: 0x008813d0, 6 virtual methods, 8 recovered fields)
/// Inherits: ccGauge
#[derive(Debug, Clone, Default)]
pub struct ccTriggerGauge {
    pub vtable_rva: u32,
    pub field_0x0000: u64,
    pub field_0x0024: f32,
    pub field_0x0028: u32,
    pub field_0x002c: f32,
    pub field_0x0038: u32,
    pub field_0x0360: u32,
    pub field_0x0368: u8,
    pub field_0x0398: [f32; 4],
}

impl ccTriggerGauge {
    pub const CLASS_NAME: &'static str = "ccTriggerGauge";
    pub const VTABLE_RVA: u32 = 0x008813d0;
    pub const METHOD_COUNT: usize = 6;
    pub const BASE_CLASSES: &'static [&'static str] = &["ccGauge"];
    pub const VIRTUAL_METHODS: &'static [u64] = &[0x000000018023a420, 0x00000001800d3090, 0x00000001800d3090, 0x000000018023a450, 0x000000018023a4e0, 0x00000001801b8190];

    pub fn new() -> Self {
        Self {
            vtable_rva: Self::VTABLE_RVA,
            field_0x0000: Default::default(),
            field_0x0024: Default::default(),
            field_0x0028: Default::default(),
            field_0x002c: Default::default(),
            field_0x0038: Default::default(),
            field_0x0360: Default::default(),
            field_0x0368: Default::default(),
            field_0x0398: Default::default(),
        }
    }

    /// Decompiled virtual method #0 @ RVA 0x0023a420 (13 instructions, 3 basic blocks)
    pub fn method_00(&mut self) {
        // Writes to field at offset +0x0000
    }

    /// Decompiled virtual method #1 @ RVA 0x000d3090 (1 instructions, 1 basic blocks)
    pub fn method_01(&mut self) {
    }

    /// Decompiled virtual method #2 @ RVA 0x000d3090 (1 instructions, 1 basic blocks)
    pub fn method_02(&mut self) {
    }

    /// Decompiled virtual method #3 @ RVA 0x0023a450 (26 instructions, 7 basic blocks)
    pub fn method_03(&mut self) {
        // Writes to field at offset +0x0038
    }

    /// Decompiled virtual method #4 @ RVA 0x0023a4e0 (79 instructions, 5 basic blocks)
    pub fn method_04(&mut self) {
        // Writes to field at offset +0x0038
        // Writes to field at offset +0x0360
        // Writes to field at offset +0x0368
        // Writes to field at offset +0x0398
    }

    /// Decompiled virtual method #5 @ RVA 0x001b8190 (12 instructions, 1 basic blocks)
    pub fn method_05(&mut self) {
        // Writes to field at offset +0x0024
        // Writes to field at offset +0x0028
        // Writes to field at offset +0x002c
    }

}

impl CcClass for ccTriggerGauge {
    fn class_name(&self) -> &'static str { Self::CLASS_NAME }
    fn vtable_rva(&self) -> u32 { Self::VTABLE_RVA }
    fn virtual_method_count(&self) -> usize { Self::METHOD_COUNT }
    fn base_classes(&self) -> &'static [&'static str] { Self::BASE_CLASSES }
}


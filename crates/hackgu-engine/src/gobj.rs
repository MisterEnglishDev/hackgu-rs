//! Subsystem: gobj
//! Contains 84 decompiled classes.

use crate::CcClass;

/// Decompiled CC2 class: `ccGameObj` (VTable RVA: 0x008bb3d8, 14 virtual methods, 34 recovered fields)
#[derive(Debug, Clone, Default)]
pub struct ccGameObj {
    pub vtable_rva: u32,
    pub field_0x0010: u64,
    pub field_0x0018: u64,
    pub field_0x001c: u8,
    pub field_0x0020: u32,
    pub field_0x0024: u16,
    pub field_0x002c: u16,
    pub field_0x0040: u32,
    pub field_0x0090: u32,
    pub field_0x0094: u32,
    pub field_0x0098: u32,
    pub field_0x009c: u32,
    pub field_0x00a0: u32,
    pub field_0x00a4: u32,
    pub field_0x00a8: u32,
    pub field_0x00ac: u32,
    pub field_0x00b0: u64,
    pub field_0x00b8: u64,
    pub field_0x00c4: u32,
    pub field_0x0120: u32,
    pub field_0x0124: u32,
    pub field_0x0128: u32,
    pub field_0x012c: u32,
    pub field_0x0130: u32,
    pub field_0x0134: u32,
    pub field_0x0138: u32,
    pub field_0x013c: u32,
    pub field_0x0140: u32,
    pub field_0x0144: u32,
    pub field_0x0148: u32,
    pub field_0x014c: u32,
    pub field_0x0150: u32,
    pub field_0x0154: u32,
    pub field_0x0158: u32,
    pub field_0x015c: u32,
}

impl ccGameObj {
    pub const CLASS_NAME: &'static str = "ccGameObj";
    pub const VTABLE_RVA: u32 = 0x008bb3d8;
    pub const METHOD_COUNT: usize = 14;
    pub const BASE_CLASSES: &'static [&'static str] = &[];
    pub const VIRTUAL_METHODS: &'static [u64] = &[0x000000018045e100, 0x00000001800d3090, 0x000000018045e4c0, 0x00000001800d3090, 0x000000018019e550, 0x000000018045ee20, 0x00000001800d3090, 0x00000001800d3090, 0x00000001800d3090, 0x00000001800d3090, 0x000000018045e220, 0x000000018045e2d0, 0x000000018045eec0, 0x000000018045efd0];

    pub fn new() -> Self {
        Self {
            vtable_rva: Self::VTABLE_RVA,
            field_0x0010: Default::default(),
            field_0x0018: Default::default(),
            field_0x001c: Default::default(),
            field_0x0020: Default::default(),
            field_0x0024: Default::default(),
            field_0x002c: Default::default(),
            field_0x0040: Default::default(),
            field_0x0090: Default::default(),
            field_0x0094: Default::default(),
            field_0x0098: Default::default(),
            field_0x009c: Default::default(),
            field_0x00a0: Default::default(),
            field_0x00a4: Default::default(),
            field_0x00a8: Default::default(),
            field_0x00ac: Default::default(),
            field_0x00b0: Default::default(),
            field_0x00b8: Default::default(),
            field_0x00c4: Default::default(),
            field_0x0120: Default::default(),
            field_0x0124: Default::default(),
            field_0x0128: Default::default(),
            field_0x012c: Default::default(),
            field_0x0130: Default::default(),
            field_0x0134: Default::default(),
            field_0x0138: Default::default(),
            field_0x013c: Default::default(),
            field_0x0140: Default::default(),
            field_0x0144: Default::default(),
            field_0x0148: Default::default(),
            field_0x014c: Default::default(),
            field_0x0150: Default::default(),
            field_0x0154: Default::default(),
            field_0x0158: Default::default(),
            field_0x015c: Default::default(),
        }
    }

    /// Decompiled virtual method #0 @ RVA 0x0045e100 (16 instructions, 3 basic blocks)
    pub fn method_00(&mut self) {
    }

    /// Decompiled virtual method #1 @ RVA 0x000d3090 (1 instructions, 1 basic blocks)
    pub fn method_01(&mut self) {
    }

    /// Decompiled virtual method #2 @ RVA 0x0045e4c0 (42 instructions, 4 basic blocks)
    pub fn method_02(&mut self) {
        // Writes to field at offset +0x0090
        // Writes to field at offset +0x0094
        // Writes to field at offset +0x0098
        // Writes to field at offset +0x009c
        // Writes to field at offset +0x00a0
        // Writes to field at offset +0x00a4
        // Writes to field at offset +0x00a8
        // Writes to field at offset +0x00ac
        // Writes to field at offset +0x0140
        // Writes to field at offset +0x0144
        // Writes to field at offset +0x0148
        // Writes to field at offset +0x014c
        // Writes to field at offset +0x0150
        // Writes to field at offset +0x0154
        // Writes to field at offset +0x0158
        // Writes to field at offset +0x015c
    }

    /// Decompiled virtual method #3 @ RVA 0x000d3090 (1 instructions, 1 basic blocks)
    pub fn method_03(&mut self) {
    }

    /// Decompiled virtual method #4 @ RVA 0x0019e550 (2 instructions, 1 basic blocks)
    pub fn method_04(&mut self) {
        // Writes to field at offset +0x00c4
    }

    /// Decompiled virtual method #5 @ RVA 0x0045ee20 (120 instructions, 40 basic blocks)
    pub fn method_05(&mut self) {
        // Writes to field at offset +0x001c
        // Writes to field at offset +0x0024
        // Writes to field at offset +0x0040
        // Writes to field at offset +0x0090
        // Writes to field at offset +0x00b0
    }

    /// Decompiled virtual method #6 @ RVA 0x000d3090 (1 instructions, 1 basic blocks)
    pub fn method_06(&mut self) {
    }

    /// Decompiled virtual method #7 @ RVA 0x000d3090 (1 instructions, 1 basic blocks)
    pub fn method_07(&mut self) {
    }

    /// Decompiled virtual method #8 @ RVA 0x000d3090 (1 instructions, 1 basic blocks)
    pub fn method_08(&mut self) {
    }

    /// Decompiled virtual method #9 @ RVA 0x000d3090 (1 instructions, 1 basic blocks)
    pub fn method_09(&mut self) {
    }

    /// Decompiled virtual method #10 @ RVA 0x0045e220 (31 instructions, 3 basic blocks)
    pub fn method_10(&mut self) {
        // Writes to field at offset +0x0090
        // Writes to field at offset +0x00a0
        // Writes to field at offset +0x00a4
        // Writes to field at offset +0x00a8
        // Writes to field at offset +0x00ac
    }

    /// Decompiled virtual method #11 @ RVA 0x0045e2d0 (33 instructions, 3 basic blocks)
    pub fn method_11(&mut self) {
        // Writes to field at offset +0x0090
        // Writes to field at offset +0x00a0
        // Writes to field at offset +0x00a4
        // Writes to field at offset +0x00a8
        // Writes to field at offset +0x00ac
    }

    /// Decompiled virtual method #12 @ RVA 0x0045eec0 (67 instructions, 22 basic blocks)
    pub fn method_12(&mut self) {
        // Writes to field at offset +0x001c
        // Writes to field at offset +0x0040
        // Writes to field at offset +0x0090
        // Writes to field at offset +0x00b0
    }

    /// Decompiled virtual method #13 @ RVA 0x0045efd0 (27 instructions, 5 basic blocks)
    pub fn method_13(&mut self) {
        // Writes to field at offset +0x0090
        // Writes to field at offset +0x00b0
    }

}

impl CcClass for ccGameObj {
    fn class_name(&self) -> &'static str { Self::CLASS_NAME }
    fn vtable_rva(&self) -> u32 { Self::VTABLE_RVA }
    fn virtual_method_count(&self) -> usize { Self::METHOD_COUNT }
    fn base_classes(&self) -> &'static [&'static str] { Self::BASE_CLASSES }
}

/// Decompiled CC2 class: `ccGameObjGrp` (VTable RVA: 0x008bb3c8, 1 virtual methods, 2 recovered fields)
#[derive(Debug, Clone, Default)]
pub struct ccGameObjGrp {
    pub vtable_rva: u32,
    pub field_0x0000: u64,
    pub field_0x0018: u64,
}

impl ccGameObjGrp {
    pub const CLASS_NAME: &'static str = "ccGameObjGrp";
    pub const VTABLE_RVA: u32 = 0x008bb3c8;
    pub const METHOD_COUNT: usize = 1;
    pub const BASE_CLASSES: &'static [&'static str] = &[];
    pub const VIRTUAL_METHODS: &'static [u64] = &[0x0000000180460530];

    pub fn new() -> Self {
        Self {
            vtable_rva: Self::VTABLE_RVA,
            field_0x0000: Default::default(),
            field_0x0018: Default::default(),
        }
    }

    /// Decompiled virtual method #0 @ RVA 0x00460530 (28 instructions, 5 basic blocks)
    pub fn method_00(&mut self) {
        // Writes to field at offset +0x0000
        // Writes to field at offset +0x0018
    }

}

impl CcClass for ccGameObjGrp {
    fn class_name(&self) -> &'static str { Self::CLASS_NAME }
    fn vtable_rva(&self) -> u32 { Self::VTABLE_RVA }
    fn virtual_method_count(&self) -> usize { Self::METHOD_COUNT }
    fn base_classes(&self) -> &'static [&'static str] { Self::BASE_CLASSES }
}

/// Decompiled CC2 class: `ccGobjAvatarBase` (VTable RVA: 0x008bc3a8, 23 virtual methods, 56 recovered fields)
/// Inherits: ccGameObj
#[derive(Debug, Clone, Default)]
pub struct ccGobjAvatarBase {
    pub vtable_rva: u32,
    pub field_0x0000: u64,
    pub field_0x0010: u64,
    pub field_0x0018: u64,
    pub field_0x001c: u8,
    pub field_0x0020: u32,
    pub field_0x0024: u16,
    pub field_0x002c: u16,
    pub field_0x0030: u32,
    pub field_0x0034: u64,
    pub field_0x003c: u32,
    pub field_0x0040: u32,
    pub field_0x0050: u32,
    pub field_0x0054: u64,
    pub field_0x005c: u64,
    pub field_0x0064: u64,
    pub field_0x006c: u64,
    pub field_0x0074: u64,
    pub field_0x007c: u32,
    pub field_0x0080: u32,
    pub field_0x0084: u32,
    pub field_0x0088: u32,
    pub field_0x008c: u64,
    pub field_0x0090: u32,
    pub field_0x0094: u32,
    pub field_0x0098: u32,
    pub field_0x009c: u32,
    pub field_0x00a0: u32,
    pub field_0x00a4: u32,
    pub field_0x00a8: u32,
    pub field_0x00ac: u32,
    pub field_0x00b0: u64,
    pub field_0x00b8: u64,
    pub field_0x00c0: u64,
    pub field_0x00c4: u32,
    pub field_0x00c8: u64,
    pub field_0x00d0: u64,
    pub field_0x00d8: u64,
    pub field_0x00e0: u32,
    pub field_0x00f0: u16,
    pub field_0x0120: u32,
    pub field_0x0124: u32,
    pub field_0x0128: u32,
    pub field_0x012c: u32,
    pub field_0x0130: u32,
    pub field_0x0134: u32,
    pub field_0x0138: u32,
    pub field_0x013c: u32,
    pub field_0x0140: u32,
    pub field_0x0144: u32,
    pub field_0x0148: u32,
    pub field_0x014c: u32,
    pub field_0x0150: u32,
    pub field_0x0154: u32,
    pub field_0x0158: u32,
    pub field_0x015c: u32,
    pub field_0x0198: u32,
}

impl ccGobjAvatarBase {
    pub const CLASS_NAME: &'static str = "ccGobjAvatarBase";
    pub const VTABLE_RVA: u32 = 0x008bc3a8;
    pub const METHOD_COUNT: usize = 23;
    pub const BASE_CLASSES: &'static [&'static str] = &["ccGameObj"];
    pub const VIRTUAL_METHODS: &'static [u64] = &[0x0000000180478430, 0x00000001800d3090, 0x000000018045e4c0, 0x0000000180478d60, 0x000000018019e550, 0x0000000180477da0, 0x00000001800d3090, 0x00000001800d3090, 0x00000001800d3090, 0x00000001800d3090, 0x00000001800d3090, 0x000000018045e2d0, 0x000000018045eec0, 0x000000018045efd0, 0x00000001804786e0, 0x00000001800d6920, 0x00000001800d6920, 0x00000001800d3090, 0x0000000180478520, 0x00000001800d3090, 0x00000001800d6920, 0x00000001800d6920, 0x00000001800d3090];

    pub fn new() -> Self {
        Self {
            vtable_rva: Self::VTABLE_RVA,
            field_0x0000: Default::default(),
            field_0x0010: Default::default(),
            field_0x0018: Default::default(),
            field_0x001c: Default::default(),
            field_0x0020: Default::default(),
            field_0x0024: Default::default(),
            field_0x002c: Default::default(),
            field_0x0030: Default::default(),
            field_0x0034: Default::default(),
            field_0x003c: Default::default(),
            field_0x0040: Default::default(),
            field_0x0050: Default::default(),
            field_0x0054: Default::default(),
            field_0x005c: Default::default(),
            field_0x0064: Default::default(),
            field_0x006c: Default::default(),
            field_0x0074: Default::default(),
            field_0x007c: Default::default(),
            field_0x0080: Default::default(),
            field_0x0084: Default::default(),
            field_0x0088: Default::default(),
            field_0x008c: Default::default(),
            field_0x0090: Default::default(),
            field_0x0094: Default::default(),
            field_0x0098: Default::default(),
            field_0x009c: Default::default(),
            field_0x00a0: Default::default(),
            field_0x00a4: Default::default(),
            field_0x00a8: Default::default(),
            field_0x00ac: Default::default(),
            field_0x00b0: Default::default(),
            field_0x00b8: Default::default(),
            field_0x00c0: Default::default(),
            field_0x00c4: Default::default(),
            field_0x00c8: Default::default(),
            field_0x00d0: Default::default(),
            field_0x00d8: Default::default(),
            field_0x00e0: Default::default(),
            field_0x00f0: Default::default(),
            field_0x0120: Default::default(),
            field_0x0124: Default::default(),
            field_0x0128: Default::default(),
            field_0x012c: Default::default(),
            field_0x0130: Default::default(),
            field_0x0134: Default::default(),
            field_0x0138: Default::default(),
            field_0x013c: Default::default(),
            field_0x0140: Default::default(),
            field_0x0144: Default::default(),
            field_0x0148: Default::default(),
            field_0x014c: Default::default(),
            field_0x0150: Default::default(),
            field_0x0154: Default::default(),
            field_0x0158: Default::default(),
            field_0x015c: Default::default(),
            field_0x0198: Default::default(),
        }
    }

    /// Decompiled virtual method #0 @ RVA 0x00478430 (16 instructions, 3 basic blocks)
    pub fn method_00(&mut self) {
    }

    /// Decompiled virtual method #1 @ RVA 0x000d3090 (1 instructions, 1 basic blocks)
    pub fn method_01(&mut self) {
    }

    /// Decompiled virtual method #2 @ RVA 0x0045e4c0 (42 instructions, 4 basic blocks)
    pub fn method_02(&mut self) {
        // Writes to field at offset +0x0090
        // Writes to field at offset +0x0094
        // Writes to field at offset +0x0098
        // Writes to field at offset +0x009c
        // Writes to field at offset +0x00a0
        // Writes to field at offset +0x00a4
        // Writes to field at offset +0x00a8
        // Writes to field at offset +0x00ac
        // Writes to field at offset +0x0140
        // Writes to field at offset +0x0144
        // Writes to field at offset +0x0148
        // Writes to field at offset +0x014c
        // Writes to field at offset +0x0150
        // Writes to field at offset +0x0154
        // Writes to field at offset +0x0158
        // Writes to field at offset +0x015c
    }

    /// Decompiled virtual method #3 @ RVA 0x00478d60 (102 instructions, 15 basic blocks)
    pub fn method_03(&mut self) {
        // Writes to field at offset +0x00b0
        // Writes to field at offset +0x0198
    }

    /// Decompiled virtual method #4 @ RVA 0x0019e550 (2 instructions, 1 basic blocks)
    pub fn method_04(&mut self) {
        // Writes to field at offset +0x00c4
    }

    /// Decompiled virtual method #5 @ RVA 0x00477da0 (42 instructions, 2 basic blocks)
    pub fn method_05(&mut self) {
        // Writes to field at offset +0x0010
        // Writes to field at offset +0x0018
        // Writes to field at offset +0x0020
        // Writes to field at offset +0x0030
        // Writes to field at offset +0x0034
        // Writes to field at offset +0x003c
        // Writes to field at offset +0x0050
        // Writes to field at offset +0x0054
        // Writes to field at offset +0x005c
        // Writes to field at offset +0x0064
        // Writes to field at offset +0x006c
        // Writes to field at offset +0x0074
        // Writes to field at offset +0x007c
        // Writes to field at offset +0x0080
        // Writes to field at offset +0x0084
        // Writes to field at offset +0x0088
        // Writes to field at offset +0x008c
        // Writes to field at offset +0x0094
        // Writes to field at offset +0x009c
        // Writes to field at offset +0x00a0
        // Writes to field at offset +0x00a8
        // Writes to field at offset +0x00b0
        // Writes to field at offset +0x00b8
        // Writes to field at offset +0x00c0
        // Writes to field at offset +0x00c8
        // Writes to field at offset +0x00d0
        // Writes to field at offset +0x00d8
        // Writes to field at offset +0x00e0
    }

    /// Decompiled virtual method #6 @ RVA 0x000d3090 (1 instructions, 1 basic blocks)
    pub fn method_06(&mut self) {
    }

    /// Decompiled virtual method #7 @ RVA 0x000d3090 (1 instructions, 1 basic blocks)
    pub fn method_07(&mut self) {
    }

    /// Decompiled virtual method #8 @ RVA 0x000d3090 (1 instructions, 1 basic blocks)
    pub fn method_08(&mut self) {
    }

    /// Decompiled virtual method #9 @ RVA 0x000d3090 (1 instructions, 1 basic blocks)
    pub fn method_09(&mut self) {
    }

    /// Decompiled virtual method #10 @ RVA 0x000d3090 (1 instructions, 1 basic blocks)
    pub fn method_10(&mut self) {
    }

    /// Decompiled virtual method #11 @ RVA 0x0045e2d0 (33 instructions, 3 basic blocks)
    pub fn method_11(&mut self) {
        // Writes to field at offset +0x0090
        // Writes to field at offset +0x00a0
        // Writes to field at offset +0x00a4
        // Writes to field at offset +0x00a8
        // Writes to field at offset +0x00ac
    }

    /// Decompiled virtual method #12 @ RVA 0x0045eec0 (67 instructions, 22 basic blocks)
    pub fn method_12(&mut self) {
        // Writes to field at offset +0x001c
        // Writes to field at offset +0x0040
        // Writes to field at offset +0x0090
        // Writes to field at offset +0x00b0
    }

    /// Decompiled virtual method #13 @ RVA 0x0045efd0 (27 instructions, 5 basic blocks)
    pub fn method_13(&mut self) {
        // Writes to field at offset +0x0090
        // Writes to field at offset +0x00b0
    }

    /// Decompiled virtual method #14 @ RVA 0x004786e0 (45 instructions, 1 basic blocks)
    pub fn method_14(&mut self) {
    }

    /// Decompiled virtual method #15 @ RVA 0x000d6920 (2 instructions, 1 basic blocks)
    pub fn method_15(&mut self) {
    }

    /// Decompiled virtual method #16 @ RVA 0x000d6920 (2 instructions, 1 basic blocks)
    pub fn method_16(&mut self) {
    }

    /// Decompiled virtual method #17 @ RVA 0x000d3090 (1 instructions, 1 basic blocks)
    pub fn method_17(&mut self) {
    }

    /// Decompiled virtual method #18 @ RVA 0x00478520 (106 instructions, 20 basic blocks)
    pub fn method_18(&mut self) {
        // Writes to field at offset +0x00b0
        // Writes to field at offset +0x00c0
        // Writes to field at offset +0x00e0
        // Writes to field at offset +0x00f0
    }

    /// Decompiled virtual method #19 @ RVA 0x000d3090 (1 instructions, 1 basic blocks)
    pub fn method_19(&mut self) {
    }

    /// Decompiled virtual method #20 @ RVA 0x000d6920 (2 instructions, 1 basic blocks)
    pub fn method_20(&mut self) {
    }

    /// Decompiled virtual method #21 @ RVA 0x000d6920 (2 instructions, 1 basic blocks)
    pub fn method_21(&mut self) {
    }

    /// Decompiled virtual method #22 @ RVA 0x000d3090 (1 instructions, 1 basic blocks)
    pub fn method_22(&mut self) {
    }

}

impl CcClass for ccGobjAvatarBase {
    fn class_name(&self) -> &'static str { Self::CLASS_NAME }
    fn vtable_rva(&self) -> u32 { Self::VTABLE_RVA }
    fn virtual_method_count(&self) -> usize { Self::METHOD_COUNT }
    fn base_classes(&self) -> &'static [&'static str] { Self::BASE_CLASSES }
}

/// Decompiled CC2 class: `ccGobjBattle` (VTable RVA: 0x008bc518, 22 virtual methods, 47 recovered fields)
/// Inherits: ccGameObj
#[derive(Debug, Clone, Default)]
pub struct ccGobjBattle {
    pub vtable_rva: u32,
    pub field_0x0000: u32,
    pub field_0x0004: u32,
    pub field_0x0008: u16,
    pub field_0x000a: u16,
    pub field_0x000c: u32,
    pub field_0x0010: u16,
    pub field_0x0018: u64,
    pub field_0x001c: u8,
    pub field_0x0020: [f32; 4],
    pub field_0x0024: u16,
    pub field_0x002c: u16,
    pub field_0x0030: f32,
    pub field_0x0034: f32,
    pub field_0x0038: f32,
    pub field_0x003c: f32,
    pub field_0x0040: u32,
    pub field_0x0090: u32,
    pub field_0x0094: u32,
    pub field_0x0098: u32,
    pub field_0x009c: u32,
    pub field_0x00a0: u32,
    pub field_0x00a4: u32,
    pub field_0x00a8: u32,
    pub field_0x00ac: u32,
    pub field_0x00b0: u64,
    pub field_0x00b8: u64,
    pub field_0x0120: u32,
    pub field_0x0124: u32,
    pub field_0x0128: u32,
    pub field_0x012c: u32,
    pub field_0x0130: u32,
    pub field_0x0134: u32,
    pub field_0x0138: u32,
    pub field_0x013c: u32,
    pub field_0x0140: u32,
    pub field_0x0144: u32,
    pub field_0x0148: u32,
    pub field_0x014c: u32,
    pub field_0x0150: u32,
    pub field_0x0154: u32,
    pub field_0x0158: u32,
    pub field_0x015c: u32,
    pub field_0x01e4: u16,
    pub field_0x0278: u32,
    pub field_0x02da: u16,
    pub field_0x0314: u16,
    pub field_0x03e0: u32,
}

impl ccGobjBattle {
    pub const CLASS_NAME: &'static str = "ccGobjBattle";
    pub const VTABLE_RVA: u32 = 0x008bc518;
    pub const METHOD_COUNT: usize = 22;
    pub const BASE_CLASSES: &'static [&'static str] = &["ccGameObj"];
    pub const VIRTUAL_METHODS: &'static [u64] = &[0x000000018047e9f0, 0x000000018047ed20, 0x000000018045e4c0, 0x00000001804878f0, 0x000000018047ebd0, 0x000000018047f370, 0x00000001800d3090, 0x00000001800d3090, 0x00000001800d3090, 0x000000018047feb0, 0x000000018047f200, 0x000000018047f0e0, 0x000000018045eec0, 0x000000018045efd0, 0x00000001800d3090, 0x00000001800d3090, 0x00000001800d3090, 0x000000018048aff0, 0x00000001800d3090, 0x00000001800d3090, 0x00000001800d3090, 0x0000000180103e70];

    pub fn new() -> Self {
        Self {
            vtable_rva: Self::VTABLE_RVA,
            field_0x0000: Default::default(),
            field_0x0004: Default::default(),
            field_0x0008: Default::default(),
            field_0x000a: Default::default(),
            field_0x000c: Default::default(),
            field_0x0010: Default::default(),
            field_0x0018: Default::default(),
            field_0x001c: Default::default(),
            field_0x0020: Default::default(),
            field_0x0024: Default::default(),
            field_0x002c: Default::default(),
            field_0x0030: Default::default(),
            field_0x0034: Default::default(),
            field_0x0038: Default::default(),
            field_0x003c: Default::default(),
            field_0x0040: Default::default(),
            field_0x0090: Default::default(),
            field_0x0094: Default::default(),
            field_0x0098: Default::default(),
            field_0x009c: Default::default(),
            field_0x00a0: Default::default(),
            field_0x00a4: Default::default(),
            field_0x00a8: Default::default(),
            field_0x00ac: Default::default(),
            field_0x00b0: Default::default(),
            field_0x00b8: Default::default(),
            field_0x0120: Default::default(),
            field_0x0124: Default::default(),
            field_0x0128: Default::default(),
            field_0x012c: Default::default(),
            field_0x0130: Default::default(),
            field_0x0134: Default::default(),
            field_0x0138: Default::default(),
            field_0x013c: Default::default(),
            field_0x0140: Default::default(),
            field_0x0144: Default::default(),
            field_0x0148: Default::default(),
            field_0x014c: Default::default(),
            field_0x0150: Default::default(),
            field_0x0154: Default::default(),
            field_0x0158: Default::default(),
            field_0x015c: Default::default(),
            field_0x01e4: Default::default(),
            field_0x0278: Default::default(),
            field_0x02da: Default::default(),
            field_0x0314: Default::default(),
            field_0x03e0: Default::default(),
        }
    }

    /// Decompiled virtual method #0 @ RVA 0x0047e9f0 (16 instructions, 3 basic blocks)
    pub fn method_00(&mut self) {
    }

    /// Decompiled virtual method #1 @ RVA 0x0047ed20 (122 instructions, 21 basic blocks)
    pub fn method_01(&mut self) {
        // Writes to field at offset +0x0000
        // Writes to field at offset +0x0004
        // Writes to field at offset +0x0008
        // Writes to field at offset +0x000a
    }

    /// Decompiled virtual method #2 @ RVA 0x0045e4c0 (42 instructions, 4 basic blocks)
    pub fn method_02(&mut self) {
        // Writes to field at offset +0x0090
        // Writes to field at offset +0x0094
        // Writes to field at offset +0x0098
        // Writes to field at offset +0x009c
        // Writes to field at offset +0x00a0
        // Writes to field at offset +0x00a4
        // Writes to field at offset +0x00a8
        // Writes to field at offset +0x00ac
        // Writes to field at offset +0x0140
        // Writes to field at offset +0x0144
        // Writes to field at offset +0x0148
        // Writes to field at offset +0x014c
        // Writes to field at offset +0x0150
        // Writes to field at offset +0x0154
        // Writes to field at offset +0x0158
        // Writes to field at offset +0x015c
    }

    /// Decompiled virtual method #3 @ RVA 0x004878f0 (108 instructions, 18 basic blocks)
    pub fn method_03(&mut self) {
        // Writes to field at offset +0x0008
        // Writes to field at offset +0x000c
        // Writes to field at offset +0x00b0
    }

    /// Decompiled virtual method #4 @ RVA 0x0047ebd0 (45 instructions, 12 basic blocks)
    pub fn method_04(&mut self) {
        // Writes to field at offset +0x0004
        // Writes to field at offset +0x02da
        // Writes to field at offset +0x0314
        // Writes to field at offset +0x03e0
    }

    /// Decompiled virtual method #5 @ RVA 0x0047f370 (124 instructions, 30 basic blocks)
    pub fn method_05(&mut self) {
        // Writes to field at offset +0x0040
    }

    /// Decompiled virtual method #6 @ RVA 0x000d3090 (1 instructions, 1 basic blocks)
    pub fn method_06(&mut self) {
    }

    /// Decompiled virtual method #7 @ RVA 0x000d3090 (1 instructions, 1 basic blocks)
    pub fn method_07(&mut self) {
    }

    /// Decompiled virtual method #8 @ RVA 0x000d3090 (1 instructions, 1 basic blocks)
    pub fn method_08(&mut self) {
    }

    /// Decompiled virtual method #9 @ RVA 0x0047feb0 (102 instructions, 24 basic blocks)
    pub fn method_09(&mut self) {
        // Writes to field at offset +0x0008
        // Writes to field at offset +0x000a
        // Writes to field at offset +0x01e4
    }

    /// Decompiled virtual method #10 @ RVA 0x0047f200 (58 instructions, 4 basic blocks)
    pub fn method_10(&mut self) {
    }

    /// Decompiled virtual method #11 @ RVA 0x0047f0e0 (15 instructions, 4 basic blocks)
    pub fn method_11(&mut self) {
    }

    /// Decompiled virtual method #12 @ RVA 0x0045eec0 (67 instructions, 22 basic blocks)
    pub fn method_12(&mut self) {
        // Writes to field at offset +0x001c
        // Writes to field at offset +0x0040
        // Writes to field at offset +0x0090
        // Writes to field at offset +0x00b0
    }

    /// Decompiled virtual method #13 @ RVA 0x0045efd0 (27 instructions, 5 basic blocks)
    pub fn method_13(&mut self) {
        // Writes to field at offset +0x0090
        // Writes to field at offset +0x00b0
    }

    /// Decompiled virtual method #14 @ RVA 0x000d3090 (1 instructions, 1 basic blocks)
    pub fn method_14(&mut self) {
    }

    /// Decompiled virtual method #15 @ RVA 0x000d3090 (1 instructions, 1 basic blocks)
    pub fn method_15(&mut self) {
    }

    /// Decompiled virtual method #16 @ RVA 0x000d3090 (1 instructions, 1 basic blocks)
    pub fn method_16(&mut self) {
    }

    /// Decompiled virtual method #17 @ RVA 0x0048aff0 (9 instructions, 1 basic blocks)
    pub fn method_17(&mut self) {
    }

    /// Decompiled virtual method #18 @ RVA 0x000d3090 (1 instructions, 1 basic blocks)
    pub fn method_18(&mut self) {
    }

    /// Decompiled virtual method #19 @ RVA 0x000d3090 (1 instructions, 1 basic blocks)
    pub fn method_19(&mut self) {
    }

    /// Decompiled virtual method #20 @ RVA 0x000d3090 (1 instructions, 1 basic blocks)
    pub fn method_20(&mut self) {
    }

    /// Decompiled virtual method #21 @ RVA 0x00103e70 (2 instructions, 1 basic blocks)
    pub fn method_21(&mut self) {
    }

}

impl CcClass for ccGobjBattle {
    fn class_name(&self) -> &'static str { Self::CLASS_NAME }
    fn vtable_rva(&self) -> u32 { Self::VTABLE_RVA }
    fn virtual_method_count(&self) -> usize { Self::METHOD_COUNT }
    fn base_classes(&self) -> &'static [&'static str] { Self::BASE_CLASSES }
}

/// Decompiled CC2 class: `ccGobjBattleAI` (VTable RVA: 0x008bc5d0, 9 virtual methods, 7 recovered fields)
#[derive(Debug, Clone, Default)]
pub struct ccGobjBattleAI {
    pub vtable_rva: u32,
    pub field_0x0028: f32,
    pub field_0x0042: u8,
    pub field_0x0080: u64,
    pub field_0x0088: u32,
    pub field_0x008c: u16,
    pub field_0x0090: u64,
    pub field_0x0094: u32,
}

impl ccGobjBattleAI {
    pub const CLASS_NAME: &'static str = "ccGobjBattleAI";
    pub const VTABLE_RVA: u32 = 0x008bc5d0;
    pub const METHOD_COUNT: usize = 9;
    pub const BASE_CLASSES: &'static [&'static str] = &[];
    pub const VIRTUAL_METHODS: &'static [u64] = &[0x00000001800d3090, 0x00000001800d3090, 0x00000001800d3090, 0x00000001800d3090, 0x0000000180492990, 0x00000001800d3090, 0x00000001800d3090, 0x0000000180493070, 0x0000000180470800];

    pub fn new() -> Self {
        Self {
            vtable_rva: Self::VTABLE_RVA,
            field_0x0028: Default::default(),
            field_0x0042: Default::default(),
            field_0x0080: Default::default(),
            field_0x0088: Default::default(),
            field_0x008c: Default::default(),
            field_0x0090: Default::default(),
            field_0x0094: Default::default(),
        }
    }

    /// Decompiled virtual method #0 @ RVA 0x000d3090 (1 instructions, 1 basic blocks)
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

    /// Decompiled virtual method #4 @ RVA 0x00492990 (6 instructions, 1 basic blocks)
    pub fn method_04(&mut self) {
        // Writes to field at offset +0x0080
        // Writes to field at offset +0x0088
        // Writes to field at offset +0x008c
        // Writes to field at offset +0x0090
    }

    /// Decompiled virtual method #5 @ RVA 0x000d3090 (1 instructions, 1 basic blocks)
    pub fn method_05(&mut self) {
    }

    /// Decompiled virtual method #6 @ RVA 0x000d3090 (1 instructions, 1 basic blocks)
    pub fn method_06(&mut self) {
    }

    /// Decompiled virtual method #7 @ RVA 0x00493070 (7 instructions, 3 basic blocks)
    pub fn method_07(&mut self) {
        // Writes to field at offset +0x0042
        // Writes to field at offset +0x0090
        // Writes to field at offset +0x0094
    }

    /// Decompiled virtual method #8 @ RVA 0x00470800 (2 instructions, 1 basic blocks)
    pub fn method_08(&mut self) {
    }

}

impl CcClass for ccGobjBattleAI {
    fn class_name(&self) -> &'static str { Self::CLASS_NAME }
    fn vtable_rva(&self) -> u32 { Self::VTABLE_RVA }
    fn virtual_method_count(&self) -> usize { Self::METHOD_COUNT }
    fn base_classes(&self) -> &'static [&'static str] { Self::BASE_CLASSES }
}

/// Decompiled CC2 class: `ccGobjBattleAIEnemy` (VTable RVA: 0x008bc278, 20 virtual methods, 40 recovered fields)
/// Inherits: ccGobjBattleAI
#[derive(Debug, Clone, Default)]
pub struct ccGobjBattleAIEnemy {
    pub vtable_rva: u32,
    pub field_0x0000: u64,
    pub field_0x0002: u8,
    pub field_0x0004: u16,
    pub field_0x0008: u32,
    pub field_0x000a: u16,
    pub field_0x000c: u32,
    pub field_0x0010: u64,
    pub field_0x0014: u8,
    pub field_0x0016: u8,
    pub field_0x0028: f32,
    pub field_0x002c: u16,
    pub field_0x0040: u32,
    pub field_0x0044: u32,
    pub field_0x0048: u32,
    pub field_0x004c: u32,
    pub field_0x0050: u32,
    pub field_0x0054: u32,
    pub field_0x0066: u32,
    pub field_0x0080: u64,
    pub field_0x0088: u32,
    pub field_0x008c: u16,
    pub field_0x0090: u64,
    pub field_0x0094: u16,
    pub field_0x0096: u16,
    pub field_0x0098: u64,
    pub field_0x009c: u32,
    pub field_0x00d0: u64,
    pub field_0x00d8: u16,
    pub field_0x00da: u16,
    pub field_0x00dc: u16,
    pub field_0x00de: u16,
    pub field_0x0108: u8,
    pub field_0x0198: u32,
    pub field_0x019c: u16,
    pub field_0x019e: u16,
    pub field_0x0268: f32,
    pub field_0x026c: f32,
    pub field_0x0270: f32,
    pub field_0x0314: u16,
    pub field_0x0800: u32,
}

impl ccGobjBattleAIEnemy {
    pub const CLASS_NAME: &'static str = "ccGobjBattleAIEnemy";
    pub const VTABLE_RVA: u32 = 0x008bc278;
    pub const METHOD_COUNT: usize = 20;
    pub const BASE_CLASSES: &'static [&'static str] = &["ccGobjBattleAI"];
    pub const VIRTUAL_METHODS: &'static [u64] = &[0x00000001804708c0, 0x0000000180470810, 0x00000001804709c0, 0x0000000180470b60, 0x0000000180492990, 0x00000001804718f0, 0x00000001800d3090, 0x0000000180470c00, 0x0000000180470800, 0x0000000180470890, 0x00000001800d3090, 0x0000000180470fd0, 0x0000000180471b60, 0x0000000180471ba0, 0x0000000180471e40, 0x0000000180472200, 0x0000000180472390, 0x0000000180472450, 0x0000000180472540, 0x00000001804725e0];

    pub fn new() -> Self {
        Self {
            vtable_rva: Self::VTABLE_RVA,
            field_0x0000: Default::default(),
            field_0x0002: Default::default(),
            field_0x0004: Default::default(),
            field_0x0008: Default::default(),
            field_0x000a: Default::default(),
            field_0x000c: Default::default(),
            field_0x0010: Default::default(),
            field_0x0014: Default::default(),
            field_0x0016: Default::default(),
            field_0x0028: Default::default(),
            field_0x002c: Default::default(),
            field_0x0040: Default::default(),
            field_0x0044: Default::default(),
            field_0x0048: Default::default(),
            field_0x004c: Default::default(),
            field_0x0050: Default::default(),
            field_0x0054: Default::default(),
            field_0x0066: Default::default(),
            field_0x0080: Default::default(),
            field_0x0088: Default::default(),
            field_0x008c: Default::default(),
            field_0x0090: Default::default(),
            field_0x0094: Default::default(),
            field_0x0096: Default::default(),
            field_0x0098: Default::default(),
            field_0x009c: Default::default(),
            field_0x00d0: Default::default(),
            field_0x00d8: Default::default(),
            field_0x00da: Default::default(),
            field_0x00dc: Default::default(),
            field_0x00de: Default::default(),
            field_0x0108: Default::default(),
            field_0x0198: Default::default(),
            field_0x019c: Default::default(),
            field_0x019e: Default::default(),
            field_0x0268: Default::default(),
            field_0x026c: Default::default(),
            field_0x0270: Default::default(),
            field_0x0314: Default::default(),
            field_0x0800: Default::default(),
        }
    }

    /// Decompiled virtual method #0 @ RVA 0x004708c0 (56 instructions, 7 basic blocks)
    pub fn method_00(&mut self) {
        // Writes to field at offset +0x009c
        // Writes to field at offset +0x00d8
        // Writes to field at offset +0x00da
        // Writes to field at offset +0x00dc
        // Writes to field at offset +0x00de
    }

    /// Decompiled virtual method #1 @ RVA 0x00470810 (2 instructions, 1 basic blocks)
    pub fn method_01(&mut self) {
        // Writes to field at offset +0x00d0
    }

    /// Decompiled virtual method #2 @ RVA 0x004709c0 (116 instructions, 9 basic blocks)
    pub fn method_02(&mut self) {
        // Writes to field at offset +0x0044
        // Writes to field at offset +0x0108
        // Writes to field at offset +0x0268
        // Writes to field at offset +0x026c
        // Writes to field at offset +0x0270
    }

    /// Decompiled virtual method #3 @ RVA 0x00470b60 (41 instructions, 7 basic blocks)
    pub fn method_03(&mut self) {
        // Writes to field at offset +0x0044
    }

    /// Decompiled virtual method #4 @ RVA 0x00492990 (6 instructions, 1 basic blocks)
    pub fn method_04(&mut self) {
        // Writes to field at offset +0x0080
        // Writes to field at offset +0x0088
        // Writes to field at offset +0x008c
        // Writes to field at offset +0x0090
    }

    /// Decompiled virtual method #5 @ RVA 0x004718f0 (50 instructions, 10 basic blocks)
    pub fn method_05(&mut self) {
        // Writes to field at offset +0x0314
    }

    /// Decompiled virtual method #6 @ RVA 0x000d3090 (1 instructions, 1 basic blocks)
    pub fn method_06(&mut self) {
    }

    /// Decompiled virtual method #7 @ RVA 0x00470c00 (54 instructions, 14 basic blocks)
    pub fn method_07(&mut self) {
    }

    /// Decompiled virtual method #8 @ RVA 0x00470800 (2 instructions, 1 basic blocks)
    pub fn method_08(&mut self) {
    }

    /// Decompiled virtual method #9 @ RVA 0x00470890 (13 instructions, 3 basic blocks)
    pub fn method_09(&mut self) {
        // Writes to field at offset +0x0000
    }

    /// Decompiled virtual method #10 @ RVA 0x000d3090 (1 instructions, 1 basic blocks)
    pub fn method_10(&mut self) {
    }

    /// Decompiled virtual method #11 @ RVA 0x00470fd0 (124 instructions, 23 basic blocks)
    pub fn method_11(&mut self) {
        // Writes to field at offset +0x0000
        // Writes to field at offset +0x0198
        // Writes to field at offset +0x019c
        // Writes to field at offset +0x019e
    }

    /// Decompiled virtual method #12 @ RVA 0x00471b60 (17 instructions, 3 basic blocks)
    pub fn method_12(&mut self) {
        // Writes to field at offset +0x0096
    }

    /// Decompiled virtual method #13 @ RVA 0x00471ba0 (113 instructions, 12 basic blocks)
    pub fn method_13(&mut self) {
        // Writes to field at offset +0x0000
        // Writes to field at offset +0x0040
        // Writes to field at offset +0x0044
        // Writes to field at offset +0x0048
        // Writes to field at offset +0x004c
        // Writes to field at offset +0x0050
        // Writes to field at offset +0x0054
    }

    /// Decompiled virtual method #14 @ RVA 0x00471e40 (9 instructions, 5 basic blocks)
    pub fn method_14(&mut self) {
    }

    /// Decompiled virtual method #15 @ RVA 0x00472200 (41 instructions, 9 basic blocks)
    pub fn method_15(&mut self) {
        // Writes to field at offset +0x000c
    }

    /// Decompiled virtual method #16 @ RVA 0x00472390 (36 instructions, 8 basic blocks)
    pub fn method_16(&mut self) {
    }

    /// Decompiled virtual method #17 @ RVA 0x00472450 (41 instructions, 9 basic blocks)
    pub fn method_17(&mut self) {
        // Writes to field at offset +0x000c
    }

    /// Decompiled virtual method #18 @ RVA 0x00472540 (39 instructions, 7 basic blocks)
    pub fn method_18(&mut self) {
    }

    /// Decompiled virtual method #19 @ RVA 0x004725e0 (22 instructions, 3 basic blocks)
    pub fn method_19(&mut self) {
        // Writes to field at offset +0x0044
    }

}

impl CcClass for ccGobjBattleAIEnemy {
    fn class_name(&self) -> &'static str { Self::CLASS_NAME }
    fn vtable_rva(&self) -> u32 { Self::VTABLE_RVA }
    fn virtual_method_count(&self) -> usize { Self::METHOD_COUNT }
    fn base_classes(&self) -> &'static [&'static str] { Self::BASE_CLASSES }
}

/// Decompiled CC2 class: `ccGobjBattleAIEnemyExkt20` (VTable RVA: 0x008bdf60, 20 virtual methods, 35 recovered fields)
/// Inherits: ccGobjBattleAIEnemy, ccGobjBattleAI
#[derive(Debug, Clone, Default)]
pub struct ccGobjBattleAIEnemyExkt20 {
    pub vtable_rva: u32,
    pub field_0x0000: u64,
    pub field_0x0002: u8,
    pub field_0x0004: u16,
    pub field_0x000a: u16,
    pub field_0x000c: u32,
    pub field_0x0010: u64,
    pub field_0x0016: u8,
    pub field_0x0028: f32,
    pub field_0x002c: u16,
    pub field_0x0040: u32,
    pub field_0x0044: u32,
    pub field_0x0048: u32,
    pub field_0x004c: u32,
    pub field_0x0050: u32,
    pub field_0x0054: u32,
    pub field_0x0066: u32,
    pub field_0x0080: u64,
    pub field_0x0088: u32,
    pub field_0x008c: u16,
    pub field_0x0090: u64,
    pub field_0x0094: u16,
    pub field_0x0096: u16,
    pub field_0x0098: u64,
    pub field_0x009c: u32,
    pub field_0x00d0: u64,
    pub field_0x00d8: u16,
    pub field_0x00da: u16,
    pub field_0x00dc: u16,
    pub field_0x00de: u16,
    pub field_0x0108: u8,
    pub field_0x0268: f32,
    pub field_0x026c: f32,
    pub field_0x0270: f32,
    pub field_0x0314: u16,
    pub field_0x0800: u32,
}

impl ccGobjBattleAIEnemyExkt20 {
    pub const CLASS_NAME: &'static str = "ccGobjBattleAIEnemyExkt20";
    pub const VTABLE_RVA: u32 = 0x008bdf60;
    pub const METHOD_COUNT: usize = 20;
    pub const BASE_CLASSES: &'static [&'static str] = &["ccGobjBattleAIEnemy", "ccGobjBattleAI"];
    pub const VIRTUAL_METHODS: &'static [u64] = &[0x00000001804708c0, 0x0000000180470810, 0x00000001804709c0, 0x0000000180470b60, 0x0000000180492990, 0x00000001804718f0, 0x00000001800d3090, 0x0000000180470c00, 0x0000000180470800, 0x00000001804bbe40, 0x00000001800d3090, 0x00000001804bf2e0, 0x0000000180471b60, 0x0000000180471ba0, 0x00000001804bfa00, 0x00000001804bf780, 0x0000000180472390, 0x0000000180472450, 0x00000001804bf960, 0x00000001804725e0];

    pub fn new() -> Self {
        Self {
            vtable_rva: Self::VTABLE_RVA,
            field_0x0000: Default::default(),
            field_0x0002: Default::default(),
            field_0x0004: Default::default(),
            field_0x000a: Default::default(),
            field_0x000c: Default::default(),
            field_0x0010: Default::default(),
            field_0x0016: Default::default(),
            field_0x0028: Default::default(),
            field_0x002c: Default::default(),
            field_0x0040: Default::default(),
            field_0x0044: Default::default(),
            field_0x0048: Default::default(),
            field_0x004c: Default::default(),
            field_0x0050: Default::default(),
            field_0x0054: Default::default(),
            field_0x0066: Default::default(),
            field_0x0080: Default::default(),
            field_0x0088: Default::default(),
            field_0x008c: Default::default(),
            field_0x0090: Default::default(),
            field_0x0094: Default::default(),
            field_0x0096: Default::default(),
            field_0x0098: Default::default(),
            field_0x009c: Default::default(),
            field_0x00d0: Default::default(),
            field_0x00d8: Default::default(),
            field_0x00da: Default::default(),
            field_0x00dc: Default::default(),
            field_0x00de: Default::default(),
            field_0x0108: Default::default(),
            field_0x0268: Default::default(),
            field_0x026c: Default::default(),
            field_0x0270: Default::default(),
            field_0x0314: Default::default(),
            field_0x0800: Default::default(),
        }
    }

    /// Decompiled virtual method #0 @ RVA 0x004708c0 (56 instructions, 7 basic blocks)
    pub fn method_00(&mut self) {
        // Writes to field at offset +0x009c
        // Writes to field at offset +0x00d8
        // Writes to field at offset +0x00da
        // Writes to field at offset +0x00dc
        // Writes to field at offset +0x00de
    }

    /// Decompiled virtual method #1 @ RVA 0x00470810 (2 instructions, 1 basic blocks)
    pub fn method_01(&mut self) {
        // Writes to field at offset +0x00d0
    }

    /// Decompiled virtual method #2 @ RVA 0x004709c0 (116 instructions, 9 basic blocks)
    pub fn method_02(&mut self) {
        // Writes to field at offset +0x0044
        // Writes to field at offset +0x0108
        // Writes to field at offset +0x0268
        // Writes to field at offset +0x026c
        // Writes to field at offset +0x0270
    }

    /// Decompiled virtual method #3 @ RVA 0x00470b60 (41 instructions, 7 basic blocks)
    pub fn method_03(&mut self) {
        // Writes to field at offset +0x0044
    }

    /// Decompiled virtual method #4 @ RVA 0x00492990 (6 instructions, 1 basic blocks)
    pub fn method_04(&mut self) {
        // Writes to field at offset +0x0080
        // Writes to field at offset +0x0088
        // Writes to field at offset +0x008c
        // Writes to field at offset +0x0090
    }

    /// Decompiled virtual method #5 @ RVA 0x004718f0 (50 instructions, 10 basic blocks)
    pub fn method_05(&mut self) {
        // Writes to field at offset +0x0314
    }

    /// Decompiled virtual method #6 @ RVA 0x000d3090 (1 instructions, 1 basic blocks)
    pub fn method_06(&mut self) {
    }

    /// Decompiled virtual method #7 @ RVA 0x00470c00 (54 instructions, 14 basic blocks)
    pub fn method_07(&mut self) {
    }

    /// Decompiled virtual method #8 @ RVA 0x00470800 (2 instructions, 1 basic blocks)
    pub fn method_08(&mut self) {
    }

    /// Decompiled virtual method #9 @ RVA 0x004bbe40 (13 instructions, 3 basic blocks)
    pub fn method_09(&mut self) {
        // Writes to field at offset +0x0000
    }

    /// Decompiled virtual method #10 @ RVA 0x000d3090 (1 instructions, 1 basic blocks)
    pub fn method_10(&mut self) {
    }

    /// Decompiled virtual method #11 @ RVA 0x004bf2e0 (87 instructions, 10 basic blocks)
    pub fn method_11(&mut self) {
    }

    /// Decompiled virtual method #12 @ RVA 0x00471b60 (17 instructions, 3 basic blocks)
    pub fn method_12(&mut self) {
        // Writes to field at offset +0x0096
    }

    /// Decompiled virtual method #13 @ RVA 0x00471ba0 (113 instructions, 12 basic blocks)
    pub fn method_13(&mut self) {
        // Writes to field at offset +0x0000
        // Writes to field at offset +0x0040
        // Writes to field at offset +0x0044
        // Writes to field at offset +0x0048
        // Writes to field at offset +0x004c
        // Writes to field at offset +0x0050
        // Writes to field at offset +0x0054
    }

    /// Decompiled virtual method #14 @ RVA 0x004bfa00 (110 instructions, 17 basic blocks)
    pub fn method_14(&mut self) {
    }

    /// Decompiled virtual method #15 @ RVA 0x004bf780 (44 instructions, 13 basic blocks)
    pub fn method_15(&mut self) {
        // Writes to field at offset +0x000c
    }

    /// Decompiled virtual method #16 @ RVA 0x00472390 (36 instructions, 8 basic blocks)
    pub fn method_16(&mut self) {
    }

    /// Decompiled virtual method #17 @ RVA 0x00472450 (41 instructions, 9 basic blocks)
    pub fn method_17(&mut self) {
        // Writes to field at offset +0x000c
    }

    /// Decompiled virtual method #18 @ RVA 0x004bf960 (29 instructions, 5 basic blocks)
    pub fn method_18(&mut self) {
    }

    /// Decompiled virtual method #19 @ RVA 0x004725e0 (22 instructions, 3 basic blocks)
    pub fn method_19(&mut self) {
        // Writes to field at offset +0x0044
    }

}

impl CcClass for ccGobjBattleAIEnemyExkt20 {
    fn class_name(&self) -> &'static str { Self::CLASS_NAME }
    fn vtable_rva(&self) -> u32 { Self::VTABLE_RVA }
    fn virtual_method_count(&self) -> usize { Self::METHOD_COUNT }
    fn base_classes(&self) -> &'static [&'static str] { Self::BASE_CLASSES }
}

/// Decompiled CC2 class: `ccGobjBattleAIEnemyExx10` (VTable RVA: 0x008be0a8, 20 virtual methods, 32 recovered fields)
/// Inherits: ccGobjBattleAIEnemy, ccGobjBattleAI
#[derive(Debug, Clone, Default)]
pub struct ccGobjBattleAIEnemyExx10 {
    pub vtable_rva: u32,
    pub field_0x0000: u64,
    pub field_0x0002: u8,
    pub field_0x0004: u16,
    pub field_0x000a: u16,
    pub field_0x000c: u32,
    pub field_0x0010: u64,
    pub field_0x0016: u8,
    pub field_0x0028: f32,
    pub field_0x002c: u16,
    pub field_0x0044: u32,
    pub field_0x0048: u32,
    pub field_0x005c: u32,
    pub field_0x0066: u32,
    pub field_0x0080: u64,
    pub field_0x0088: u32,
    pub field_0x008c: u16,
    pub field_0x0090: u64,
    pub field_0x0094: u16,
    pub field_0x0096: u16,
    pub field_0x0098: u64,
    pub field_0x009c: u32,
    pub field_0x00d0: u64,
    pub field_0x00d8: u16,
    pub field_0x00da: u16,
    pub field_0x00dc: u16,
    pub field_0x00de: u16,
    pub field_0x0108: u8,
    pub field_0x0128: u32,
    pub field_0x0268: f32,
    pub field_0x026c: f32,
    pub field_0x0270: f32,
    pub field_0x0314: u16,
}

impl ccGobjBattleAIEnemyExx10 {
    pub const CLASS_NAME: &'static str = "ccGobjBattleAIEnemyExx10";
    pub const VTABLE_RVA: u32 = 0x008be0a8;
    pub const METHOD_COUNT: usize = 20;
    pub const BASE_CLASSES: &'static [&'static str] = &["ccGobjBattleAIEnemy", "ccGobjBattleAI"];
    pub const VIRTUAL_METHODS: &'static [u64] = &[0x00000001804708c0, 0x0000000180470810, 0x00000001804709c0, 0x0000000180470b60, 0x0000000180492990, 0x00000001804718f0, 0x00000001800d3090, 0x0000000180470c00, 0x0000000180470800, 0x00000001804bfbe0, 0x00000001804c2e20, 0x00000001804c2d90, 0x0000000180471b60, 0x00000001804c2f60, 0x0000000180471e40, 0x00000001804c32c0, 0x0000000180472390, 0x0000000180472450, 0x0000000180472540, 0x00000001804725e0];

    pub fn new() -> Self {
        Self {
            vtable_rva: Self::VTABLE_RVA,
            field_0x0000: Default::default(),
            field_0x0002: Default::default(),
            field_0x0004: Default::default(),
            field_0x000a: Default::default(),
            field_0x000c: Default::default(),
            field_0x0010: Default::default(),
            field_0x0016: Default::default(),
            field_0x0028: Default::default(),
            field_0x002c: Default::default(),
            field_0x0044: Default::default(),
            field_0x0048: Default::default(),
            field_0x005c: Default::default(),
            field_0x0066: Default::default(),
            field_0x0080: Default::default(),
            field_0x0088: Default::default(),
            field_0x008c: Default::default(),
            field_0x0090: Default::default(),
            field_0x0094: Default::default(),
            field_0x0096: Default::default(),
            field_0x0098: Default::default(),
            field_0x009c: Default::default(),
            field_0x00d0: Default::default(),
            field_0x00d8: Default::default(),
            field_0x00da: Default::default(),
            field_0x00dc: Default::default(),
            field_0x00de: Default::default(),
            field_0x0108: Default::default(),
            field_0x0128: Default::default(),
            field_0x0268: Default::default(),
            field_0x026c: Default::default(),
            field_0x0270: Default::default(),
            field_0x0314: Default::default(),
        }
    }

    /// Decompiled virtual method #0 @ RVA 0x004708c0 (56 instructions, 7 basic blocks)
    pub fn method_00(&mut self) {
        // Writes to field at offset +0x009c
        // Writes to field at offset +0x00d8
        // Writes to field at offset +0x00da
        // Writes to field at offset +0x00dc
        // Writes to field at offset +0x00de
    }

    /// Decompiled virtual method #1 @ RVA 0x00470810 (2 instructions, 1 basic blocks)
    pub fn method_01(&mut self) {
        // Writes to field at offset +0x00d0
    }

    /// Decompiled virtual method #2 @ RVA 0x004709c0 (116 instructions, 9 basic blocks)
    pub fn method_02(&mut self) {
        // Writes to field at offset +0x0044
        // Writes to field at offset +0x0108
        // Writes to field at offset +0x0268
        // Writes to field at offset +0x026c
        // Writes to field at offset +0x0270
    }

    /// Decompiled virtual method #3 @ RVA 0x00470b60 (41 instructions, 7 basic blocks)
    pub fn method_03(&mut self) {
        // Writes to field at offset +0x0044
    }

    /// Decompiled virtual method #4 @ RVA 0x00492990 (6 instructions, 1 basic blocks)
    pub fn method_04(&mut self) {
        // Writes to field at offset +0x0080
        // Writes to field at offset +0x0088
        // Writes to field at offset +0x008c
        // Writes to field at offset +0x0090
    }

    /// Decompiled virtual method #5 @ RVA 0x004718f0 (50 instructions, 10 basic blocks)
    pub fn method_05(&mut self) {
        // Writes to field at offset +0x0314
    }

    /// Decompiled virtual method #6 @ RVA 0x000d3090 (1 instructions, 1 basic blocks)
    pub fn method_06(&mut self) {
    }

    /// Decompiled virtual method #7 @ RVA 0x00470c00 (54 instructions, 14 basic blocks)
    pub fn method_07(&mut self) {
    }

    /// Decompiled virtual method #8 @ RVA 0x00470800 (2 instructions, 1 basic blocks)
    pub fn method_08(&mut self) {
    }

    /// Decompiled virtual method #9 @ RVA 0x004bfbe0 (13 instructions, 3 basic blocks)
    pub fn method_09(&mut self) {
        // Writes to field at offset +0x0000
    }

    /// Decompiled virtual method #10 @ RVA 0x004c2e20 (48 instructions, 17 basic blocks)
    pub fn method_10(&mut self) {
    }

    /// Decompiled virtual method #11 @ RVA 0x004c2d90 (94 instructions, 21 basic blocks)
    pub fn method_11(&mut self) {
    }

    /// Decompiled virtual method #12 @ RVA 0x00471b60 (17 instructions, 3 basic blocks)
    pub fn method_12(&mut self) {
        // Writes to field at offset +0x0096
    }

    /// Decompiled virtual method #13 @ RVA 0x004c2f60 (118 instructions, 25 basic blocks)
    pub fn method_13(&mut self) {
    }

    /// Decompiled virtual method #14 @ RVA 0x00471e40 (9 instructions, 5 basic blocks)
    pub fn method_14(&mut self) {
    }

    /// Decompiled virtual method #15 @ RVA 0x004c32c0 (30 instructions, 8 basic blocks)
    pub fn method_15(&mut self) {
        // Writes to field at offset +0x000c
    }

    /// Decompiled virtual method #16 @ RVA 0x00472390 (36 instructions, 8 basic blocks)
    pub fn method_16(&mut self) {
    }

    /// Decompiled virtual method #17 @ RVA 0x00472450 (41 instructions, 9 basic blocks)
    pub fn method_17(&mut self) {
        // Writes to field at offset +0x000c
    }

    /// Decompiled virtual method #18 @ RVA 0x00472540 (39 instructions, 7 basic blocks)
    pub fn method_18(&mut self) {
    }

    /// Decompiled virtual method #19 @ RVA 0x004725e0 (22 instructions, 3 basic blocks)
    pub fn method_19(&mut self) {
        // Writes to field at offset +0x0044
    }

}

impl CcClass for ccGobjBattleAIEnemyExx10 {
    fn class_name(&self) -> &'static str { Self::CLASS_NAME }
    fn vtable_rva(&self) -> u32 { Self::VTABLE_RVA }
    fn virtual_method_count(&self) -> usize { Self::METHOD_COUNT }
    fn base_classes(&self) -> &'static [&'static str] { Self::BASE_CLASSES }
}

/// Decompiled CC2 class: `ccGobjBattleAISpc` (VTable RVA: 0x008bc320, 16 virtual methods, 48 recovered fields)
/// Inherits: ccGobjBattleAI
#[derive(Debug, Clone, Default)]
pub struct ccGobjBattleAISpc {
    pub vtable_rva: u32,
    pub field_0x0000: u64,
    pub field_0x0010: u64,
    pub field_0x0018: u32,
    pub field_0x001c: u8,
    pub field_0x0020: u32,
    pub field_0x0024: u32,
    pub field_0x0028: u32,
    pub field_0x002c: u32,
    pub field_0x0030: u8,
    pub field_0x0032: u16,
    pub field_0x0034: u8,
    pub field_0x0040: u32,
    pub field_0x0044: u32,
    pub field_0x0048: u32,
    pub field_0x004c: u32,
    pub field_0x0050: u32,
    pub field_0x0054: u32,
    pub field_0x0066: u32,
    pub field_0x0080: u32,
    pub field_0x0088: u32,
    pub field_0x008c: u16,
    pub field_0x0090: u64,
    pub field_0x0098: u64,
    pub field_0x009c: u32,
    pub field_0x00c4: u32,
    pub field_0x00d4: u32,
    pub field_0x00e0: u32,
    pub field_0x00e4: u32,
    pub field_0x00e8: u32,
    pub field_0x00ec: u32,
    pub field_0x01d8: u32,
    pub field_0x01ec: u32,
    pub field_0x01f4: u32,
    pub field_0x01f8: u32,
    pub field_0x01fc: u32,
    pub field_0x0200: u8,
    pub field_0x0201: u8,
    pub field_0x0202: u16,
    pub field_0x0204: u16,
    pub field_0x0206: u16,
    pub field_0x0208: u8,
    pub field_0x020c: u8,
    pub field_0x0219: u8,
    pub field_0x021c: u16,
    pub field_0x021e: u8,
    pub field_0x0314: u16,
    pub field_0x03e0: u32,
    pub field_0x0800: u32,
}

impl ccGobjBattleAISpc {
    pub const CLASS_NAME: &'static str = "ccGobjBattleAISpc";
    pub const VTABLE_RVA: u32 = 0x008bc320;
    pub const METHOD_COUNT: usize = 16;
    pub const BASE_CLASSES: &'static [&'static str] = &["ccGobjBattleAI"];
    pub const VIRTUAL_METHODS: &'static [u64] = &[0x0000000180473190, 0x00000001804733e0, 0x00000001804734b0, 0x0000000180473660, 0x0000000180492990, 0x0000000180474cd0, 0x0000000180473040, 0x0000000180473740, 0x0000000180470800, 0x0000000180473160, 0x00000001800d3090, 0x0000000180474fc0, 0x0000000180473870, 0x0000000180474ca0, 0x00000001800d6920, 0x00000001800d3090];

    pub fn new() -> Self {
        Self {
            vtable_rva: Self::VTABLE_RVA,
            field_0x0000: Default::default(),
            field_0x0010: Default::default(),
            field_0x0018: Default::default(),
            field_0x001c: Default::default(),
            field_0x0020: Default::default(),
            field_0x0024: Default::default(),
            field_0x0028: Default::default(),
            field_0x002c: Default::default(),
            field_0x0030: Default::default(),
            field_0x0032: Default::default(),
            field_0x0034: Default::default(),
            field_0x0040: Default::default(),
            field_0x0044: Default::default(),
            field_0x0048: Default::default(),
            field_0x004c: Default::default(),
            field_0x0050: Default::default(),
            field_0x0054: Default::default(),
            field_0x0066: Default::default(),
            field_0x0080: Default::default(),
            field_0x0088: Default::default(),
            field_0x008c: Default::default(),
            field_0x0090: Default::default(),
            field_0x0098: Default::default(),
            field_0x009c: Default::default(),
            field_0x00c4: Default::default(),
            field_0x00d4: Default::default(),
            field_0x00e0: Default::default(),
            field_0x00e4: Default::default(),
            field_0x00e8: Default::default(),
            field_0x00ec: Default::default(),
            field_0x01d8: Default::default(),
            field_0x01ec: Default::default(),
            field_0x01f4: Default::default(),
            field_0x01f8: Default::default(),
            field_0x01fc: Default::default(),
            field_0x0200: Default::default(),
            field_0x0201: Default::default(),
            field_0x0202: Default::default(),
            field_0x0204: Default::default(),
            field_0x0206: Default::default(),
            field_0x0208: Default::default(),
            field_0x020c: Default::default(),
            field_0x0219: Default::default(),
            field_0x021c: Default::default(),
            field_0x021e: Default::default(),
            field_0x0314: Default::default(),
            field_0x03e0: Default::default(),
            field_0x0800: Default::default(),
        }
    }

    /// Decompiled virtual method #0 @ RVA 0x00473190 (127 instructions, 21 basic blocks)
    pub fn method_00(&mut self) {
        // Writes to field at offset +0x009c
    }

    /// Decompiled virtual method #1 @ RVA 0x004733e0 (15 instructions, 3 basic blocks)
    pub fn method_01(&mut self) {
        // Writes to field at offset +0x001c
        // Writes to field at offset +0x0020
        // Writes to field at offset +0x0201
        // Writes to field at offset +0x0202
        // Writes to field at offset +0x0204
        // Writes to field at offset +0x0208
    }

    /// Decompiled virtual method #2 @ RVA 0x004734b0 (102 instructions, 23 basic blocks)
    pub fn method_02(&mut self) {
        // Writes to field at offset +0x0018
        // Writes to field at offset +0x0024
        // Writes to field at offset +0x0024
        // Writes to field at offset +0x0028
        // Writes to field at offset +0x002c
        // Writes to field at offset +0x0030
        // Writes to field at offset +0x0032
        // Writes to field at offset +0x0034
        // Writes to field at offset +0x0044
        // Writes to field at offset +0x01ec
        // Writes to field at offset +0x01ec
        // Writes to field at offset +0x01f4
        // Writes to field at offset +0x01f8
        // Writes to field at offset +0x01f8
        // Writes to field at offset +0x01f8
        // Writes to field at offset +0x01fc
        // Writes to field at offset +0x01fc
        // Writes to field at offset +0x0200
        // Writes to field at offset +0x0206
        // Writes to field at offset +0x020c
        // Writes to field at offset +0x0219
    }

    /// Decompiled virtual method #3 @ RVA 0x00473660 (53 instructions, 8 basic blocks)
    pub fn method_03(&mut self) {
        // Writes to field at offset +0x0000
        // Writes to field at offset +0x0044
        // Writes to field at offset +0x0219
    }

    /// Decompiled virtual method #4 @ RVA 0x00492990 (6 instructions, 1 basic blocks)
    pub fn method_04(&mut self) {
        // Writes to field at offset +0x0080
        // Writes to field at offset +0x0088
        // Writes to field at offset +0x008c
        // Writes to field at offset +0x0090
    }

    /// Decompiled virtual method #5 @ RVA 0x00474cd0 (63 instructions, 12 basic blocks)
    pub fn method_05(&mut self) {
        // Writes to field at offset +0x00c4
    }

    /// Decompiled virtual method #6 @ RVA 0x00473040 (2 instructions, 1 basic blocks)
    pub fn method_06(&mut self) {
        // Writes to field at offset +0x021e
    }

    /// Decompiled virtual method #7 @ RVA 0x00473740 (68 instructions, 20 basic blocks)
    pub fn method_07(&mut self) {
    }

    /// Decompiled virtual method #8 @ RVA 0x00470800 (2 instructions, 1 basic blocks)
    pub fn method_08(&mut self) {
    }

    /// Decompiled virtual method #9 @ RVA 0x00473160 (13 instructions, 3 basic blocks)
    pub fn method_09(&mut self) {
        // Writes to field at offset +0x0000
    }

    /// Decompiled virtual method #10 @ RVA 0x000d3090 (1 instructions, 1 basic blocks)
    pub fn method_10(&mut self) {
    }

    /// Decompiled virtual method #11 @ RVA 0x00474fc0 (6 instructions, 3 basic blocks)
    pub fn method_11(&mut self) {
        // Writes to field at offset +0x021c
    }

    /// Decompiled virtual method #12 @ RVA 0x00473870 (125 instructions, 17 basic blocks)
    pub fn method_12(&mut self) {
        // Writes to field at offset +0x0000
        // Writes to field at offset +0x0040
        // Writes to field at offset +0x0044
        // Writes to field at offset +0x0048
        // Writes to field at offset +0x004c
        // Writes to field at offset +0x0050
        // Writes to field at offset +0x0054
    }

    /// Decompiled virtual method #13 @ RVA 0x00474ca0 (10 instructions, 4 basic blocks)
    pub fn method_13(&mut self) {
        // Writes to field at offset +0x0314
    }

    /// Decompiled virtual method #14 @ RVA 0x000d6920 (2 instructions, 1 basic blocks)
    pub fn method_14(&mut self) {
    }

    /// Decompiled virtual method #15 @ RVA 0x000d3090 (1 instructions, 1 basic blocks)
    pub fn method_15(&mut self) {
    }

}

impl CcClass for ccGobjBattleAISpc {
    fn class_name(&self) -> &'static str { Self::CLASS_NAME }
    fn vtable_rva(&self) -> u32 { Self::VTABLE_RVA }
    fn virtual_method_count(&self) -> usize { Self::METHOD_COUNT }
    fn base_classes(&self) -> &'static [&'static str] { Self::BASE_CLASSES }
}

/// Decompiled CC2 class: `ccGobjBattleAISpcArena` (VTable RVA: 0x008c0828, 16 virtual methods, 54 recovered fields)
/// Inherits: ccGobjBattleAISpc, ccGobjBattleAI
#[derive(Debug, Clone, Default)]
pub struct ccGobjBattleAISpcArena {
    pub vtable_rva: u32,
    pub field_0x0000: u8,
    pub field_0x0001: u8,
    pub field_0x0004: u32,
    pub field_0x000a: u16,
    pub field_0x0010: u64,
    pub field_0x0028: f32,
    pub field_0x0040: u8,
    pub field_0x0041: u8,
    pub field_0x0042: u8,
    pub field_0x0043: u8,
    pub field_0x0044: u8,
    pub field_0x0066: u32,
    pub field_0x0080: u32,
    pub field_0x0088: u32,
    pub field_0x008c: u16,
    pub field_0x0090: u64,
    pub field_0x0098: u64,
    pub field_0x009c: u32,
    pub field_0x00c4: u32,
    pub field_0x00e0: u32,
    pub field_0x00e4: u32,
    pub field_0x00e8: u32,
    pub field_0x00ec: u32,
    pub field_0x01d8: u32,
    pub field_0x01ec: f32,
    pub field_0x01f4: u32,
    pub field_0x01f8: f32,
    pub field_0x01fc: u32,
    pub field_0x020c: u8,
    pub field_0x020d: u8,
    pub field_0x020e: u8,
    pub field_0x020f: u8,
    pub field_0x0210: u8,
    pub field_0x0212: u16,
    pub field_0x0219: u8,
    pub field_0x021c: u16,
    pub field_0x021e: u8,
    pub field_0x021f: u8,
    pub field_0x0240: u64,
    pub field_0x0248: f32,
    pub field_0x024d: u8,
    pub field_0x024e: u8,
    pub field_0x024f: u8,
    pub field_0x0250: u8,
    pub field_0x0251: u8,
    pub field_0x0254: f32,
    pub field_0x0258: f32,
    pub field_0x025c: f32,
    pub field_0x0260: f32,
    pub field_0x0264: u16,
    pub field_0x0266: u16,
    pub field_0x0268: f32,
    pub field_0x026c: f32,
    pub field_0x03e0: u32,
}

impl ccGobjBattleAISpcArena {
    pub const CLASS_NAME: &'static str = "ccGobjBattleAISpcArena";
    pub const VTABLE_RVA: u32 = 0x008c0828;
    pub const METHOD_COUNT: usize = 16;
    pub const BASE_CLASSES: &'static [&'static str] = &["ccGobjBattleAISpc", "ccGobjBattleAI"];
    pub const VIRTUAL_METHODS: &'static [u64] = &[0x0000000180473190, 0x0000000180476e20, 0x0000000180476e90, 0x0000000180473660, 0x0000000180492990, 0x0000000180474cd0, 0x0000000180473040, 0x0000000180473740, 0x0000000180477880, 0x0000000180508540, 0x0000000180477340, 0x0000000180477410, 0x0000000180477560, 0x0000000180477700, 0x00000001800d6920, 0x00000001800d3090];

    pub fn new() -> Self {
        Self {
            vtable_rva: Self::VTABLE_RVA,
            field_0x0000: Default::default(),
            field_0x0001: Default::default(),
            field_0x0004: Default::default(),
            field_0x000a: Default::default(),
            field_0x0010: Default::default(),
            field_0x0028: Default::default(),
            field_0x0040: Default::default(),
            field_0x0041: Default::default(),
            field_0x0042: Default::default(),
            field_0x0043: Default::default(),
            field_0x0044: Default::default(),
            field_0x0066: Default::default(),
            field_0x0080: Default::default(),
            field_0x0088: Default::default(),
            field_0x008c: Default::default(),
            field_0x0090: Default::default(),
            field_0x0098: Default::default(),
            field_0x009c: Default::default(),
            field_0x00c4: Default::default(),
            field_0x00e0: Default::default(),
            field_0x00e4: Default::default(),
            field_0x00e8: Default::default(),
            field_0x00ec: Default::default(),
            field_0x01d8: Default::default(),
            field_0x01ec: Default::default(),
            field_0x01f4: Default::default(),
            field_0x01f8: Default::default(),
            field_0x01fc: Default::default(),
            field_0x020c: Default::default(),
            field_0x020d: Default::default(),
            field_0x020e: Default::default(),
            field_0x020f: Default::default(),
            field_0x0210: Default::default(),
            field_0x0212: Default::default(),
            field_0x0219: Default::default(),
            field_0x021c: Default::default(),
            field_0x021e: Default::default(),
            field_0x021f: Default::default(),
            field_0x0240: Default::default(),
            field_0x0248: Default::default(),
            field_0x024d: Default::default(),
            field_0x024e: Default::default(),
            field_0x024f: Default::default(),
            field_0x0250: Default::default(),
            field_0x0251: Default::default(),
            field_0x0254: Default::default(),
            field_0x0258: Default::default(),
            field_0x025c: Default::default(),
            field_0x0260: Default::default(),
            field_0x0264: Default::default(),
            field_0x0266: Default::default(),
            field_0x0268: Default::default(),
            field_0x026c: Default::default(),
            field_0x03e0: Default::default(),
        }
    }

    /// Decompiled virtual method #0 @ RVA 0x00473190 (127 instructions, 21 basic blocks)
    pub fn method_00(&mut self) {
        // Writes to field at offset +0x009c
    }

    /// Decompiled virtual method #1 @ RVA 0x00476e20 (5 instructions, 2 basic blocks)
    pub fn method_01(&mut self) {
        // Writes to field at offset +0x0240
    }

    /// Decompiled virtual method #2 @ RVA 0x00476e90 (101 instructions, 2 basic blocks)
    pub fn method_02(&mut self) {
    }

    /// Decompiled virtual method #3 @ RVA 0x00473660 (53 instructions, 8 basic blocks)
    pub fn method_03(&mut self) {
        // Writes to field at offset +0x0000
        // Writes to field at offset +0x0044
        // Writes to field at offset +0x0219
    }

    /// Decompiled virtual method #4 @ RVA 0x00492990 (6 instructions, 1 basic blocks)
    pub fn method_04(&mut self) {
        // Writes to field at offset +0x0080
        // Writes to field at offset +0x0088
        // Writes to field at offset +0x008c
        // Writes to field at offset +0x0090
    }

    /// Decompiled virtual method #5 @ RVA 0x00474cd0 (63 instructions, 12 basic blocks)
    pub fn method_05(&mut self) {
        // Writes to field at offset +0x00c4
    }

    /// Decompiled virtual method #6 @ RVA 0x00473040 (2 instructions, 1 basic blocks)
    pub fn method_06(&mut self) {
        // Writes to field at offset +0x021e
    }

    /// Decompiled virtual method #7 @ RVA 0x00473740 (68 instructions, 20 basic blocks)
    pub fn method_07(&mut self) {
    }

    /// Decompiled virtual method #8 @ RVA 0x00477880 (24 instructions, 4 basic blocks)
    pub fn method_08(&mut self) {
    }

    /// Decompiled virtual method #9 @ RVA 0x00508540 (13 instructions, 3 basic blocks)
    pub fn method_09(&mut self) {
        // Writes to field at offset +0x0000
    }

    /// Decompiled virtual method #10 @ RVA 0x00477340 (26 instructions, 4 basic blocks)
    pub fn method_10(&mut self) {
        // Writes to field at offset +0x01ec
        // Writes to field at offset +0x020c
        // Writes to field at offset +0x020d
        // Writes to field at offset +0x020e
        // Writes to field at offset +0x020f
        // Writes to field at offset +0x0210
        // Writes to field at offset +0x021f
    }

    /// Decompiled virtual method #11 @ RVA 0x00477410 (74 instructions, 14 basic blocks)
    pub fn method_11(&mut self) {
        // Writes to field at offset +0x0028
        // Writes to field at offset +0x0212
        // Writes to field at offset +0x021c
    }

    /// Decompiled virtual method #12 @ RVA 0x00477560 (28 instructions, 9 basic blocks)
    pub fn method_12(&mut self) {
    }

    /// Decompiled virtual method #13 @ RVA 0x00477700 (80 instructions, 20 basic blocks)
    pub fn method_13(&mut self) {
        // Writes to field at offset +0x024d
    }

    /// Decompiled virtual method #14 @ RVA 0x000d6920 (2 instructions, 1 basic blocks)
    pub fn method_14(&mut self) {
    }

    /// Decompiled virtual method #15 @ RVA 0x000d3090 (1 instructions, 1 basic blocks)
    pub fn method_15(&mut self) {
    }

}

impl CcClass for ccGobjBattleAISpcArena {
    fn class_name(&self) -> &'static str { Self::CLASS_NAME }
    fn vtable_rva(&self) -> u32 { Self::VTABLE_RVA }
    fn virtual_method_count(&self) -> usize { Self::METHOD_COUNT }
    fn base_classes(&self) -> &'static [&'static str] { Self::BASE_CLASSES }
}

/// Decompiled CC2 class: `ccGobjBattleAISpcArenaEnd` (VTable RVA: 0x008c0718, 16 virtual methods, 54 recovered fields)
/// Inherits: ccGobjBattleAISpcArena, ccGobjBattleAISpc, ccGobjBattleAI
#[derive(Debug, Clone, Default)]
pub struct ccGobjBattleAISpcArenaEnd {
    pub vtable_rva: u32,
    pub field_0x0000: u8,
    pub field_0x0001: u8,
    pub field_0x0004: u32,
    pub field_0x000a: u16,
    pub field_0x0010: u64,
    pub field_0x0028: f32,
    pub field_0x0040: u8,
    pub field_0x0041: u8,
    pub field_0x0042: u8,
    pub field_0x0043: u8,
    pub field_0x0044: u8,
    pub field_0x0066: u32,
    pub field_0x0080: u32,
    pub field_0x0088: u32,
    pub field_0x008c: u16,
    pub field_0x0090: u64,
    pub field_0x0098: u64,
    pub field_0x009c: u32,
    pub field_0x00c4: u32,
    pub field_0x00e0: u32,
    pub field_0x00e4: u32,
    pub field_0x00e8: u32,
    pub field_0x00ec: u32,
    pub field_0x01d8: u32,
    pub field_0x01ec: f32,
    pub field_0x01f4: u32,
    pub field_0x01f8: f32,
    pub field_0x01fc: u32,
    pub field_0x020c: u8,
    pub field_0x020d: u8,
    pub field_0x020e: u8,
    pub field_0x020f: u8,
    pub field_0x0210: u8,
    pub field_0x0212: u16,
    pub field_0x0219: u8,
    pub field_0x021c: u16,
    pub field_0x021e: u8,
    pub field_0x021f: u8,
    pub field_0x0240: u64,
    pub field_0x0248: f32,
    pub field_0x024d: u8,
    pub field_0x024e: u8,
    pub field_0x024f: u8,
    pub field_0x0250: u8,
    pub field_0x0251: u8,
    pub field_0x0254: f32,
    pub field_0x0258: f32,
    pub field_0x025c: f32,
    pub field_0x0260: f32,
    pub field_0x0264: u16,
    pub field_0x0266: u16,
    pub field_0x0268: f32,
    pub field_0x026c: f32,
    pub field_0x03e0: u32,
}

impl ccGobjBattleAISpcArenaEnd {
    pub const CLASS_NAME: &'static str = "ccGobjBattleAISpcArenaEnd";
    pub const VTABLE_RVA: u32 = 0x008c0718;
    pub const METHOD_COUNT: usize = 16;
    pub const BASE_CLASSES: &'static [&'static str] = &["ccGobjBattleAISpcArena", "ccGobjBattleAISpc", "ccGobjBattleAI"];
    pub const VIRTUAL_METHODS: &'static [u64] = &[0x0000000180473190, 0x0000000180476e20, 0x0000000180476e90, 0x0000000180473660, 0x0000000180492990, 0x0000000180474cd0, 0x0000000180473040, 0x0000000180473740, 0x0000000180477880, 0x0000000180508540, 0x0000000180477340, 0x0000000180477410, 0x0000000180477560, 0x0000000180477700, 0x0000000180477900, 0x00000001800d3090];

    pub fn new() -> Self {
        Self {
            vtable_rva: Self::VTABLE_RVA,
            field_0x0000: Default::default(),
            field_0x0001: Default::default(),
            field_0x0004: Default::default(),
            field_0x000a: Default::default(),
            field_0x0010: Default::default(),
            field_0x0028: Default::default(),
            field_0x0040: Default::default(),
            field_0x0041: Default::default(),
            field_0x0042: Default::default(),
            field_0x0043: Default::default(),
            field_0x0044: Default::default(),
            field_0x0066: Default::default(),
            field_0x0080: Default::default(),
            field_0x0088: Default::default(),
            field_0x008c: Default::default(),
            field_0x0090: Default::default(),
            field_0x0098: Default::default(),
            field_0x009c: Default::default(),
            field_0x00c4: Default::default(),
            field_0x00e0: Default::default(),
            field_0x00e4: Default::default(),
            field_0x00e8: Default::default(),
            field_0x00ec: Default::default(),
            field_0x01d8: Default::default(),
            field_0x01ec: Default::default(),
            field_0x01f4: Default::default(),
            field_0x01f8: Default::default(),
            field_0x01fc: Default::default(),
            field_0x020c: Default::default(),
            field_0x020d: Default::default(),
            field_0x020e: Default::default(),
            field_0x020f: Default::default(),
            field_0x0210: Default::default(),
            field_0x0212: Default::default(),
            field_0x0219: Default::default(),
            field_0x021c: Default::default(),
            field_0x021e: Default::default(),
            field_0x021f: Default::default(),
            field_0x0240: Default::default(),
            field_0x0248: Default::default(),
            field_0x024d: Default::default(),
            field_0x024e: Default::default(),
            field_0x024f: Default::default(),
            field_0x0250: Default::default(),
            field_0x0251: Default::default(),
            field_0x0254: Default::default(),
            field_0x0258: Default::default(),
            field_0x025c: Default::default(),
            field_0x0260: Default::default(),
            field_0x0264: Default::default(),
            field_0x0266: Default::default(),
            field_0x0268: Default::default(),
            field_0x026c: Default::default(),
            field_0x03e0: Default::default(),
        }
    }

    /// Decompiled virtual method #0 @ RVA 0x00473190 (127 instructions, 21 basic blocks)
    pub fn method_00(&mut self) {
        // Writes to field at offset +0x009c
    }

    /// Decompiled virtual method #1 @ RVA 0x00476e20 (5 instructions, 2 basic blocks)
    pub fn method_01(&mut self) {
        // Writes to field at offset +0x0240
    }

    /// Decompiled virtual method #2 @ RVA 0x00476e90 (101 instructions, 2 basic blocks)
    pub fn method_02(&mut self) {
    }

    /// Decompiled virtual method #3 @ RVA 0x00473660 (53 instructions, 8 basic blocks)
    pub fn method_03(&mut self) {
        // Writes to field at offset +0x0000
        // Writes to field at offset +0x0044
        // Writes to field at offset +0x0219
    }

    /// Decompiled virtual method #4 @ RVA 0x00492990 (6 instructions, 1 basic blocks)
    pub fn method_04(&mut self) {
        // Writes to field at offset +0x0080
        // Writes to field at offset +0x0088
        // Writes to field at offset +0x008c
        // Writes to field at offset +0x0090
    }

    /// Decompiled virtual method #5 @ RVA 0x00474cd0 (63 instructions, 12 basic blocks)
    pub fn method_05(&mut self) {
        // Writes to field at offset +0x00c4
    }

    /// Decompiled virtual method #6 @ RVA 0x00473040 (2 instructions, 1 basic blocks)
    pub fn method_06(&mut self) {
        // Writes to field at offset +0x021e
    }

    /// Decompiled virtual method #7 @ RVA 0x00473740 (68 instructions, 20 basic blocks)
    pub fn method_07(&mut self) {
    }

    /// Decompiled virtual method #8 @ RVA 0x00477880 (24 instructions, 4 basic blocks)
    pub fn method_08(&mut self) {
    }

    /// Decompiled virtual method #9 @ RVA 0x00508540 (13 instructions, 3 basic blocks)
    pub fn method_09(&mut self) {
        // Writes to field at offset +0x0000
    }

    /// Decompiled virtual method #10 @ RVA 0x00477340 (26 instructions, 4 basic blocks)
    pub fn method_10(&mut self) {
        // Writes to field at offset +0x01ec
        // Writes to field at offset +0x020c
        // Writes to field at offset +0x020d
        // Writes to field at offset +0x020e
        // Writes to field at offset +0x020f
        // Writes to field at offset +0x0210
        // Writes to field at offset +0x021f
    }

    /// Decompiled virtual method #11 @ RVA 0x00477410 (74 instructions, 14 basic blocks)
    pub fn method_11(&mut self) {
        // Writes to field at offset +0x0028
        // Writes to field at offset +0x0212
        // Writes to field at offset +0x021c
    }

    /// Decompiled virtual method #12 @ RVA 0x00477560 (28 instructions, 9 basic blocks)
    pub fn method_12(&mut self) {
    }

    /// Decompiled virtual method #13 @ RVA 0x00477700 (80 instructions, 20 basic blocks)
    pub fn method_13(&mut self) {
        // Writes to field at offset +0x024d
    }

    /// Decompiled virtual method #14 @ RVA 0x00477900 (61 instructions, 8 basic blocks)
    pub fn method_14(&mut self) {
    }

    /// Decompiled virtual method #15 @ RVA 0x000d3090 (1 instructions, 1 basic blocks)
    pub fn method_15(&mut self) {
    }

}

impl CcClass for ccGobjBattleAISpcArenaEnd {
    fn class_name(&self) -> &'static str { Self::CLASS_NAME }
    fn vtable_rva(&self) -> u32 { Self::VTABLE_RVA }
    fn virtual_method_count(&self) -> usize { Self::METHOD_COUNT }
    fn base_classes(&self) -> &'static [&'static str] { Self::BASE_CLASSES }
}

/// Decompiled CC2 class: `ccGobjBattleAISpcArenaMat` (VTable RVA: 0x008c07a0, 16 virtual methods, 55 recovered fields)
/// Inherits: ccGobjBattleAISpcArena, ccGobjBattleAISpc, ccGobjBattleAI
#[derive(Debug, Clone, Default)]
pub struct ccGobjBattleAISpcArenaMat {
    pub vtable_rva: u32,
    pub field_0x0000: u8,
    pub field_0x0001: u8,
    pub field_0x0004: u32,
    pub field_0x000a: u16,
    pub field_0x0010: u64,
    pub field_0x0028: f32,
    pub field_0x0040: u8,
    pub field_0x0041: u8,
    pub field_0x0042: u8,
    pub field_0x0043: u8,
    pub field_0x0044: u8,
    pub field_0x0066: u32,
    pub field_0x0080: u32,
    pub field_0x0088: u32,
    pub field_0x008c: u16,
    pub field_0x0090: u64,
    pub field_0x0098: u64,
    pub field_0x009c: u32,
    pub field_0x00c4: u32,
    pub field_0x00e0: u32,
    pub field_0x00e4: u32,
    pub field_0x00e8: u32,
    pub field_0x00ec: u32,
    pub field_0x01d8: u32,
    pub field_0x01ec: f32,
    pub field_0x01f4: u32,
    pub field_0x01f8: f32,
    pub field_0x01fc: u32,
    pub field_0x020c: u8,
    pub field_0x020d: u8,
    pub field_0x020e: u8,
    pub field_0x020f: u8,
    pub field_0x0210: u8,
    pub field_0x0212: u16,
    pub field_0x0219: u8,
    pub field_0x021c: u16,
    pub field_0x021e: u8,
    pub field_0x021f: u8,
    pub field_0x0240: u64,
    pub field_0x0248: f32,
    pub field_0x024d: u8,
    pub field_0x024e: u8,
    pub field_0x024f: u8,
    pub field_0x0250: u8,
    pub field_0x0251: u8,
    pub field_0x0254: f32,
    pub field_0x0258: f32,
    pub field_0x025c: f32,
    pub field_0x0260: f32,
    pub field_0x0264: u16,
    pub field_0x0266: u16,
    pub field_0x0268: f32,
    pub field_0x026c: f32,
    pub field_0x0280: u8,
    pub field_0x03e0: u32,
}

impl ccGobjBattleAISpcArenaMat {
    pub const CLASS_NAME: &'static str = "ccGobjBattleAISpcArenaMat";
    pub const VTABLE_RVA: u32 = 0x008c07a0;
    pub const METHOD_COUNT: usize = 16;
    pub const BASE_CLASSES: &'static [&'static str] = &["ccGobjBattleAISpcArena", "ccGobjBattleAISpc", "ccGobjBattleAI"];
    pub const VIRTUAL_METHODS: &'static [u64] = &[0x0000000180473190, 0x0000000180476e20, 0x0000000180476e90, 0x0000000180473660, 0x0000000180492990, 0x0000000180474cd0, 0x0000000180473040, 0x0000000180473740, 0x0000000180477880, 0x0000000180508570, 0x0000000180477340, 0x0000000180477410, 0x0000000180477560, 0x0000000180477700, 0x0000000180477a20, 0x00000001800d3090];

    pub fn new() -> Self {
        Self {
            vtable_rva: Self::VTABLE_RVA,
            field_0x0000: Default::default(),
            field_0x0001: Default::default(),
            field_0x0004: Default::default(),
            field_0x000a: Default::default(),
            field_0x0010: Default::default(),
            field_0x0028: Default::default(),
            field_0x0040: Default::default(),
            field_0x0041: Default::default(),
            field_0x0042: Default::default(),
            field_0x0043: Default::default(),
            field_0x0044: Default::default(),
            field_0x0066: Default::default(),
            field_0x0080: Default::default(),
            field_0x0088: Default::default(),
            field_0x008c: Default::default(),
            field_0x0090: Default::default(),
            field_0x0098: Default::default(),
            field_0x009c: Default::default(),
            field_0x00c4: Default::default(),
            field_0x00e0: Default::default(),
            field_0x00e4: Default::default(),
            field_0x00e8: Default::default(),
            field_0x00ec: Default::default(),
            field_0x01d8: Default::default(),
            field_0x01ec: Default::default(),
            field_0x01f4: Default::default(),
            field_0x01f8: Default::default(),
            field_0x01fc: Default::default(),
            field_0x020c: Default::default(),
            field_0x020d: Default::default(),
            field_0x020e: Default::default(),
            field_0x020f: Default::default(),
            field_0x0210: Default::default(),
            field_0x0212: Default::default(),
            field_0x0219: Default::default(),
            field_0x021c: Default::default(),
            field_0x021e: Default::default(),
            field_0x021f: Default::default(),
            field_0x0240: Default::default(),
            field_0x0248: Default::default(),
            field_0x024d: Default::default(),
            field_0x024e: Default::default(),
            field_0x024f: Default::default(),
            field_0x0250: Default::default(),
            field_0x0251: Default::default(),
            field_0x0254: Default::default(),
            field_0x0258: Default::default(),
            field_0x025c: Default::default(),
            field_0x0260: Default::default(),
            field_0x0264: Default::default(),
            field_0x0266: Default::default(),
            field_0x0268: Default::default(),
            field_0x026c: Default::default(),
            field_0x0280: Default::default(),
            field_0x03e0: Default::default(),
        }
    }

    /// Decompiled virtual method #0 @ RVA 0x00473190 (127 instructions, 21 basic blocks)
    pub fn method_00(&mut self) {
        // Writes to field at offset +0x009c
    }

    /// Decompiled virtual method #1 @ RVA 0x00476e20 (5 instructions, 2 basic blocks)
    pub fn method_01(&mut self) {
        // Writes to field at offset +0x0240
    }

    /// Decompiled virtual method #2 @ RVA 0x00476e90 (101 instructions, 2 basic blocks)
    pub fn method_02(&mut self) {
    }

    /// Decompiled virtual method #3 @ RVA 0x00473660 (53 instructions, 8 basic blocks)
    pub fn method_03(&mut self) {
        // Writes to field at offset +0x0000
        // Writes to field at offset +0x0044
        // Writes to field at offset +0x0219
    }

    /// Decompiled virtual method #4 @ RVA 0x00492990 (6 instructions, 1 basic blocks)
    pub fn method_04(&mut self) {
        // Writes to field at offset +0x0080
        // Writes to field at offset +0x0088
        // Writes to field at offset +0x008c
        // Writes to field at offset +0x0090
    }

    /// Decompiled virtual method #5 @ RVA 0x00474cd0 (63 instructions, 12 basic blocks)
    pub fn method_05(&mut self) {
        // Writes to field at offset +0x00c4
    }

    /// Decompiled virtual method #6 @ RVA 0x00473040 (2 instructions, 1 basic blocks)
    pub fn method_06(&mut self) {
        // Writes to field at offset +0x021e
    }

    /// Decompiled virtual method #7 @ RVA 0x00473740 (68 instructions, 20 basic blocks)
    pub fn method_07(&mut self) {
    }

    /// Decompiled virtual method #8 @ RVA 0x00477880 (24 instructions, 4 basic blocks)
    pub fn method_08(&mut self) {
    }

    /// Decompiled virtual method #9 @ RVA 0x00508570 (13 instructions, 3 basic blocks)
    pub fn method_09(&mut self) {
        // Writes to field at offset +0x0000
    }

    /// Decompiled virtual method #10 @ RVA 0x00477340 (26 instructions, 4 basic blocks)
    pub fn method_10(&mut self) {
        // Writes to field at offset +0x01ec
        // Writes to field at offset +0x020c
        // Writes to field at offset +0x020d
        // Writes to field at offset +0x020e
        // Writes to field at offset +0x020f
        // Writes to field at offset +0x0210
        // Writes to field at offset +0x021f
    }

    /// Decompiled virtual method #11 @ RVA 0x00477410 (74 instructions, 14 basic blocks)
    pub fn method_11(&mut self) {
        // Writes to field at offset +0x0028
        // Writes to field at offset +0x0212
        // Writes to field at offset +0x021c
    }

    /// Decompiled virtual method #12 @ RVA 0x00477560 (28 instructions, 9 basic blocks)
    pub fn method_12(&mut self) {
    }

    /// Decompiled virtual method #13 @ RVA 0x00477700 (80 instructions, 20 basic blocks)
    pub fn method_13(&mut self) {
        // Writes to field at offset +0x024d
    }

    /// Decompiled virtual method #14 @ RVA 0x00477a20 (34 instructions, 5 basic blocks)
    pub fn method_14(&mut self) {
        // Writes to field at offset +0x0280
    }

    /// Decompiled virtual method #15 @ RVA 0x000d3090 (1 instructions, 1 basic blocks)
    pub fn method_15(&mut self) {
    }

}

impl CcClass for ccGobjBattleAISpcArenaMat {
    fn class_name(&self) -> &'static str { Self::CLASS_NAME }
    fn vtable_rva(&self) -> u32 { Self::VTABLE_RVA }
    fn virtual_method_count(&self) -> usize { Self::METHOD_COUNT }
    fn base_classes(&self) -> &'static [&'static str] { Self::BASE_CLASSES }
}

/// Decompiled CC2 class: `ccGobjBike` (VTable RVA: 0x008bc638, 16 virtual methods, 25 recovered fields)
/// Inherits: ccGameObj
#[derive(Debug, Clone, Default)]
pub struct ccGobjBike {
    pub vtable_rva: u32,
    pub field_0x0000: u64,
    pub field_0x0010: u64,
    pub field_0x0018: u64,
    pub field_0x001c: u8,
    pub field_0x0020: u32,
    pub field_0x0024: u16,
    pub field_0x002c: u16,
    pub field_0x0040: u32,
    pub field_0x005e: u8,
    pub field_0x0060: f32,
    pub field_0x006c: u32,
    pub field_0x0070: u32,
    pub field_0x0090: u64,
    pub field_0x0098: u64,
    pub field_0x00a0: u32,
    pub field_0x00a4: u32,
    pub field_0x00a8: u32,
    pub field_0x00ac: u32,
    pub field_0x00b0: u64,
    pub field_0x00b8: u64,
    pub field_0x00c4: u32,
    pub field_0x0168: u64,
    pub field_0x01b0: u32,
    pub field_0x0258: u64,
    pub field_0x0304: u8,
}

impl ccGobjBike {
    pub const CLASS_NAME: &'static str = "ccGobjBike";
    pub const VTABLE_RVA: u32 = 0x008bc638;
    pub const METHOD_COUNT: usize = 16;
    pub const BASE_CLASSES: &'static [&'static str] = &["ccGameObj"];
    pub const VIRTUAL_METHODS: &'static [u64] = &[0x0000000180498670, 0x000000018049b030, 0x000000018049b540, 0x00000001800d3090, 0x000000018019e550, 0x000000018049c580, 0x00000001800d3090, 0x00000001800d3090, 0x000000018049b6b0, 0x000000018049bfe0, 0x000000018049b660, 0x000000018045e2d0, 0x000000018045eec0, 0x000000018045efd0, 0x00000001804986d0, 0x0000000180498ae0];

    pub fn new() -> Self {
        Self {
            vtable_rva: Self::VTABLE_RVA,
            field_0x0000: Default::default(),
            field_0x0010: Default::default(),
            field_0x0018: Default::default(),
            field_0x001c: Default::default(),
            field_0x0020: Default::default(),
            field_0x0024: Default::default(),
            field_0x002c: Default::default(),
            field_0x0040: Default::default(),
            field_0x005e: Default::default(),
            field_0x0060: Default::default(),
            field_0x006c: Default::default(),
            field_0x0070: Default::default(),
            field_0x0090: Default::default(),
            field_0x0098: Default::default(),
            field_0x00a0: Default::default(),
            field_0x00a4: Default::default(),
            field_0x00a8: Default::default(),
            field_0x00ac: Default::default(),
            field_0x00b0: Default::default(),
            field_0x00b8: Default::default(),
            field_0x00c4: Default::default(),
            field_0x0168: Default::default(),
            field_0x01b0: Default::default(),
            field_0x0258: Default::default(),
            field_0x0304: Default::default(),
        }
    }

    /// Decompiled virtual method #0 @ RVA 0x00498670 (24 instructions, 3 basic blocks)
    pub fn method_00(&mut self) {
        // Writes to field at offset +0x0000
    }

    /// Decompiled virtual method #1 @ RVA 0x0049b030 (110 instructions, 27 basic blocks)
    pub fn method_01(&mut self) {
    }

    /// Decompiled virtual method #2 @ RVA 0x0049b540 (56 instructions, 9 basic blocks)
    pub fn method_02(&mut self) {
        // Writes to field at offset +0x0168
    }

    /// Decompiled virtual method #3 @ RVA 0x000d3090 (1 instructions, 1 basic blocks)
    pub fn method_03(&mut self) {
    }

    /// Decompiled virtual method #4 @ RVA 0x0019e550 (2 instructions, 1 basic blocks)
    pub fn method_04(&mut self) {
        // Writes to field at offset +0x00c4
    }

    /// Decompiled virtual method #5 @ RVA 0x0049c580 (100 instructions, 11 basic blocks)
    pub fn method_05(&mut self) {
    }

    /// Decompiled virtual method #6 @ RVA 0x000d3090 (1 instructions, 1 basic blocks)
    pub fn method_06(&mut self) {
    }

    /// Decompiled virtual method #7 @ RVA 0x000d3090 (1 instructions, 1 basic blocks)
    pub fn method_07(&mut self) {
    }

    /// Decompiled virtual method #8 @ RVA 0x0049b6b0 (107 instructions, 25 basic blocks)
    pub fn method_08(&mut self) {
        // Writes to field at offset +0x00c4
    }

    /// Decompiled virtual method #9 @ RVA 0x0049bfe0 (73 instructions, 16 basic blocks)
    pub fn method_09(&mut self) {
    }

    /// Decompiled virtual method #10 @ RVA 0x0049b660 (16 instructions, 1 basic blocks)
    pub fn method_10(&mut self) {
    }

    /// Decompiled virtual method #11 @ RVA 0x0045e2d0 (33 instructions, 3 basic blocks)
    pub fn method_11(&mut self) {
        // Writes to field at offset +0x0090
        // Writes to field at offset +0x00a0
        // Writes to field at offset +0x00a4
        // Writes to field at offset +0x00a8
        // Writes to field at offset +0x00ac
    }

    /// Decompiled virtual method #12 @ RVA 0x0045eec0 (67 instructions, 22 basic blocks)
    pub fn method_12(&mut self) {
        // Writes to field at offset +0x001c
        // Writes to field at offset +0x0040
        // Writes to field at offset +0x0090
        // Writes to field at offset +0x00b0
    }

    /// Decompiled virtual method #13 @ RVA 0x0045efd0 (27 instructions, 5 basic blocks)
    pub fn method_13(&mut self) {
        // Writes to field at offset +0x0090
        // Writes to field at offset +0x00b0
    }

    /// Decompiled virtual method #14 @ RVA 0x004986d0 (128 instructions, 20 basic blocks)
    pub fn method_14(&mut self) {
        // Writes to field at offset +0x01b0
        // Writes to field at offset +0x0304
    }

    /// Decompiled virtual method #15 @ RVA 0x00498ae0 (88 instructions, 24 basic blocks)
    pub fn method_15(&mut self) {
        // Writes to field at offset +0x0000
    }

}

impl CcClass for ccGobjBike {
    fn class_name(&self) -> &'static str { Self::CLASS_NAME }
    fn vtable_rva(&self) -> u32 { Self::VTABLE_RVA }
    fn virtual_method_count(&self) -> usize { Self::METHOD_COUNT }
    fn base_classes(&self) -> &'static [&'static str] { Self::BASE_CLASSES }
}

/// Decompiled CC2 class: `ccGobjBpc` (VTable RVA: 0x008bc6f8, 23 virtual methods, 81 recovered fields)
/// Inherits: ccGobjSpc, ccGobjBattle, ccGameObj
#[derive(Debug, Clone, Default)]
pub struct ccGobjBpc {
    pub vtable_rva: u32,
    pub field_0x0000: u64,
    pub field_0x0004: u32,
    pub field_0x0006: u16,
    pub field_0x0008: u16,
    pub field_0x0009: u8,
    pub field_0x000a: u16,
    pub field_0x000c: u16,
    pub field_0x0010: u64,
    pub field_0x0018: u64,
    pub field_0x001c: u8,
    pub field_0x0020: u32,
    pub field_0x0024: u32,
    pub field_0x0028: u32,
    pub field_0x002c: u16,
    pub field_0x0034: u32,
    pub field_0x003c: [f32; 4],
    pub field_0x0040: u32,
    pub field_0x004c: u16,
    pub field_0x0058: u64,
    pub field_0x0068: u64,
    pub field_0x0070: u32,
    pub field_0x0090: u32,
    pub field_0x0094: u32,
    pub field_0x0098: u32,
    pub field_0x009c: u32,
    pub field_0x00a0: u32,
    pub field_0x00a4: u32,
    pub field_0x00a8: u32,
    pub field_0x00ac: u32,
    pub field_0x00b0: u64,
    pub field_0x00b8: u64,
    pub field_0x00c4: u32,
    pub field_0x00e0: u32,
    pub field_0x00e4: u32,
    pub field_0x00e8: u32,
    pub field_0x00ec: u32,
    pub field_0x00f0: u32,
    pub field_0x00f4: u32,
    pub field_0x00f8: u32,
    pub field_0x0120: u32,
    pub field_0x0124: u32,
    pub field_0x0128: u32,
    pub field_0x012c: u32,
    pub field_0x0130: u32,
    pub field_0x0134: u32,
    pub field_0x0138: u32,
    pub field_0x013c: u32,
    pub field_0x0140: u32,
    pub field_0x0144: u32,
    pub field_0x0148: u32,
    pub field_0x014c: u32,
    pub field_0x0150: u32,
    pub field_0x0154: u32,
    pub field_0x0158: u32,
    pub field_0x015c: u32,
    pub field_0x0190: u8,
    pub field_0x01c8: u32,
    pub field_0x01e4: u16,
    pub field_0x01e6: u16,
    pub field_0x020c: u64,
    pub field_0x0214: u64,
    pub field_0x021c: u64,
    pub field_0x0224: u64,
    pub field_0x022c: u32,
    pub field_0x0230: [f32; 4],
    pub field_0x0234: u64,
    pub field_0x023c: u64,
    pub field_0x0240: u32,
    pub field_0x0244: u64,
    pub field_0x024c: u64,
    pub field_0x0254: u32,
    pub field_0x0278: u32,
    pub field_0x02da: u16,
    pub field_0x02fc: u8,
    pub field_0x0314: u16,
    pub field_0x03e0: u32,
    pub field_0x03f0: u64,
    pub field_0x0400: u64,
    pub field_0x0410: u64,
    pub field_0x0440: u32,
    pub field_0x04f0: u32,
}

impl ccGobjBpc {
    pub const CLASS_NAME: &'static str = "ccGobjBpc";
    pub const VTABLE_RVA: u32 = 0x008bc6f8;
    pub const METHOD_COUNT: usize = 23;
    pub const BASE_CLASSES: &'static [&'static str] = &["ccGobjSpc", "ccGobjBattle", "ccGameObj"];
    pub const VIRTUAL_METHODS: &'static [u64] = &[0x00000001804a2480, 0x0000000180509960, 0x000000018045e4c0, 0x0000000180517180, 0x000000018047ebd0, 0x00000001805094c0, 0x00000001804a2c20, 0x00000001800d3090, 0x000000018050aa50, 0x00000001804a2de0, 0x000000018050a4e0, 0x000000018047f0e0, 0x000000018045eec0, 0x000000018045efd0, 0x000000018051d300, 0x00000001805191c0, 0x0000000180518920, 0x000000018051d220, 0x0000000180518720, 0x00000001805187c0, 0x0000000180518850, 0x000000018051d510, 0x00000001804a24c0];

    pub fn new() -> Self {
        Self {
            vtable_rva: Self::VTABLE_RVA,
            field_0x0000: Default::default(),
            field_0x0004: Default::default(),
            field_0x0006: Default::default(),
            field_0x0008: Default::default(),
            field_0x0009: Default::default(),
            field_0x000a: Default::default(),
            field_0x000c: Default::default(),
            field_0x0010: Default::default(),
            field_0x0018: Default::default(),
            field_0x001c: Default::default(),
            field_0x0020: Default::default(),
            field_0x0024: Default::default(),
            field_0x0028: Default::default(),
            field_0x002c: Default::default(),
            field_0x0034: Default::default(),
            field_0x003c: Default::default(),
            field_0x0040: Default::default(),
            field_0x004c: Default::default(),
            field_0x0058: Default::default(),
            field_0x0068: Default::default(),
            field_0x0070: Default::default(),
            field_0x0090: Default::default(),
            field_0x0094: Default::default(),
            field_0x0098: Default::default(),
            field_0x009c: Default::default(),
            field_0x00a0: Default::default(),
            field_0x00a4: Default::default(),
            field_0x00a8: Default::default(),
            field_0x00ac: Default::default(),
            field_0x00b0: Default::default(),
            field_0x00b8: Default::default(),
            field_0x00c4: Default::default(),
            field_0x00e0: Default::default(),
            field_0x00e4: Default::default(),
            field_0x00e8: Default::default(),
            field_0x00ec: Default::default(),
            field_0x00f0: Default::default(),
            field_0x00f4: Default::default(),
            field_0x00f8: Default::default(),
            field_0x0120: Default::default(),
            field_0x0124: Default::default(),
            field_0x0128: Default::default(),
            field_0x012c: Default::default(),
            field_0x0130: Default::default(),
            field_0x0134: Default::default(),
            field_0x0138: Default::default(),
            field_0x013c: Default::default(),
            field_0x0140: Default::default(),
            field_0x0144: Default::default(),
            field_0x0148: Default::default(),
            field_0x014c: Default::default(),
            field_0x0150: Default::default(),
            field_0x0154: Default::default(),
            field_0x0158: Default::default(),
            field_0x015c: Default::default(),
            field_0x0190: Default::default(),
            field_0x01c8: Default::default(),
            field_0x01e4: Default::default(),
            field_0x01e6: Default::default(),
            field_0x020c: Default::default(),
            field_0x0214: Default::default(),
            field_0x021c: Default::default(),
            field_0x0224: Default::default(),
            field_0x022c: Default::default(),
            field_0x0230: Default::default(),
            field_0x0234: Default::default(),
            field_0x023c: Default::default(),
            field_0x0240: Default::default(),
            field_0x0244: Default::default(),
            field_0x024c: Default::default(),
            field_0x0254: Default::default(),
            field_0x0278: Default::default(),
            field_0x02da: Default::default(),
            field_0x02fc: Default::default(),
            field_0x0314: Default::default(),
            field_0x03e0: Default::default(),
            field_0x03f0: Default::default(),
            field_0x0400: Default::default(),
            field_0x0410: Default::default(),
            field_0x0440: Default::default(),
            field_0x04f0: Default::default(),
        }
    }

    /// Decompiled virtual method #0 @ RVA 0x004a2480 (18 instructions, 3 basic blocks)
    pub fn method_00(&mut self) {
        // Writes to field at offset +0x0000
    }

    /// Decompiled virtual method #1 @ RVA 0x00509960 (19 instructions, 5 basic blocks)
    pub fn method_01(&mut self) {
        // Writes to field at offset +0x00c4
    }

    /// Decompiled virtual method #2 @ RVA 0x0045e4c0 (42 instructions, 4 basic blocks)
    pub fn method_02(&mut self) {
        // Writes to field at offset +0x0090
        // Writes to field at offset +0x0094
        // Writes to field at offset +0x0098
        // Writes to field at offset +0x009c
        // Writes to field at offset +0x00a0
        // Writes to field at offset +0x00a4
        // Writes to field at offset +0x00a8
        // Writes to field at offset +0x00ac
        // Writes to field at offset +0x0140
        // Writes to field at offset +0x0144
        // Writes to field at offset +0x0148
        // Writes to field at offset +0x014c
        // Writes to field at offset +0x0150
        // Writes to field at offset +0x0154
        // Writes to field at offset +0x0158
        // Writes to field at offset +0x015c
    }

    /// Decompiled virtual method #3 @ RVA 0x00517180 (96 instructions, 32 basic blocks)
    pub fn method_03(&mut self) {
        // Writes to field at offset +0x0018
        // Writes to field at offset +0x00b0
    }

    /// Decompiled virtual method #4 @ RVA 0x0047ebd0 (45 instructions, 12 basic blocks)
    pub fn method_04(&mut self) {
        // Writes to field at offset +0x0004
        // Writes to field at offset +0x02da
        // Writes to field at offset +0x0314
        // Writes to field at offset +0x03e0
    }

    /// Decompiled virtual method #5 @ RVA 0x005094c0 (115 instructions, 22 basic blocks)
    pub fn method_05(&mut self) {
        // Writes to field at offset +0x000a
        // Writes to field at offset +0x000c
        // Writes to field at offset +0x0024
    }

    /// Decompiled virtual method #6 @ RVA 0x004a2c20 (25 instructions, 5 basic blocks)
    pub fn method_06(&mut self) {
    }

    /// Decompiled virtual method #7 @ RVA 0x000d3090 (1 instructions, 1 basic blocks)
    pub fn method_07(&mut self) {
    }

    /// Decompiled virtual method #8 @ RVA 0x0050aa50 (30 instructions, 6 basic blocks)
    pub fn method_08(&mut self) {
        // Writes to field at offset +0x00c4
        // Writes to field at offset +0x04f0
    }

    /// Decompiled virtual method #9 @ RVA 0x004a2de0 (112 instructions, 28 basic blocks)
    pub fn method_09(&mut self) {
        // Writes to field at offset +0x0009
        // Writes to field at offset +0x0070
    }

    /// Decompiled virtual method #10 @ RVA 0x0050a4e0 (30 instructions, 12 basic blocks)
    pub fn method_10(&mut self) {
    }

    /// Decompiled virtual method #11 @ RVA 0x0047f0e0 (15 instructions, 4 basic blocks)
    pub fn method_11(&mut self) {
    }

    /// Decompiled virtual method #12 @ RVA 0x0045eec0 (67 instructions, 22 basic blocks)
    pub fn method_12(&mut self) {
        // Writes to field at offset +0x001c
        // Writes to field at offset +0x0040
        // Writes to field at offset +0x0090
        // Writes to field at offset +0x00b0
    }

    /// Decompiled virtual method #13 @ RVA 0x0045efd0 (27 instructions, 5 basic blocks)
    pub fn method_13(&mut self) {
        // Writes to field at offset +0x0090
        // Writes to field at offset +0x00b0
    }

    /// Decompiled virtual method #14 @ RVA 0x0051d300 (107 instructions, 1 basic blocks)
    pub fn method_14(&mut self) {
        // Writes to field at offset +0x0000
        // Writes to field at offset +0x0004
        // Writes to field at offset +0x0008
        // Writes to field at offset +0x000a
        // Writes to field at offset +0x020c
        // Writes to field at offset +0x0214
        // Writes to field at offset +0x021c
        // Writes to field at offset +0x0224
        // Writes to field at offset +0x022c
        // Writes to field at offset +0x0234
        // Writes to field at offset +0x023c
        // Writes to field at offset +0x0244
        // Writes to field at offset +0x024c
        // Writes to field at offset +0x0254
    }

    /// Decompiled virtual method #15 @ RVA 0x005191c0 (107 instructions, 35 basic blocks)
    pub fn method_15(&mut self) {
        // Writes to field at offset +0x0004
        // Writes to field at offset +0x00c4
        // Writes to field at offset +0x01e4
    }

    /// Decompiled virtual method #16 @ RVA 0x00518920 (120 instructions, 25 basic blocks)
    pub fn method_16(&mut self) {
    }

    /// Decompiled virtual method #17 @ RVA 0x0051d220 (52 instructions, 11 basic blocks)
    pub fn method_17(&mut self) {
    }

    /// Decompiled virtual method #18 @ RVA 0x00518720 (40 instructions, 11 basic blocks)
    pub fn method_18(&mut self) {
        // Writes to field at offset +0x0400
    }

    /// Decompiled virtual method #19 @ RVA 0x005187c0 (38 instructions, 11 basic blocks)
    pub fn method_19(&mut self) {
    }

    /// Decompiled virtual method #20 @ RVA 0x00518850 (50 instructions, 15 basic blocks)
    pub fn method_20(&mut self) {
    }

    /// Decompiled virtual method #21 @ RVA 0x0051d510 (9 instructions, 3 basic blocks)
    pub fn method_21(&mut self) {
        // Writes to field at offset +0x0028
        // Writes to field at offset +0x002c
        // Writes to field at offset +0x02fc
    }

    /// Decompiled virtual method #22 @ RVA 0x004a24c0 (114 instructions, 15 basic blocks)
    pub fn method_22(&mut self) {
        // Writes to field at offset +0x0190
    }

}

impl CcClass for ccGobjBpc {
    fn class_name(&self) -> &'static str { Self::CLASS_NAME }
    fn vtable_rva(&self) -> u32 { Self::VTABLE_RVA }
    fn virtual_method_count(&self) -> usize { Self::METHOD_COUNT }
    fn base_classes(&self) -> &'static [&'static str] { Self::BASE_CLASSES }
}

/// Decompiled CC2 class: `ccGobjChimBase` (VTable RVA: 0x008bcb38, 17 virtual methods, 43 recovered fields)
/// Inherits: ccGameObj
#[derive(Debug, Clone, Default)]
pub struct ccGobjChimBase {
    pub vtable_rva: u32,
    pub field_0x0000: u64,
    pub field_0x0009: u8,
    pub field_0x0010: u64,
    pub field_0x0018: u64,
    pub field_0x001c: u8,
    pub field_0x0020: u32,
    pub field_0x0024: u16,
    pub field_0x002c: u16,
    pub field_0x0040: u32,
    pub field_0x0050: u64,
    pub field_0x0058: u64,
    pub field_0x0090: u32,
    pub field_0x0094: u32,
    pub field_0x0098: u32,
    pub field_0x009c: u32,
    pub field_0x00a0: u32,
    pub field_0x00a4: u32,
    pub field_0x00a8: u32,
    pub field_0x00ac: u32,
    pub field_0x00b0: u64,
    pub field_0x00b8: u64,
    pub field_0x00c4: u32,
    pub field_0x00d0: u32,
    pub field_0x0120: u32,
    pub field_0x0124: u32,
    pub field_0x0128: u32,
    pub field_0x012c: u32,
    pub field_0x0130: u32,
    pub field_0x0134: u32,
    pub field_0x0138: u32,
    pub field_0x013c: u32,
    pub field_0x0140: u32,
    pub field_0x0144: u32,
    pub field_0x0148: u32,
    pub field_0x014c: u32,
    pub field_0x0150: u32,
    pub field_0x0154: u32,
    pub field_0x0158: u32,
    pub field_0x015c: u32,
    pub field_0x01b8: u64,
    pub field_0x01c0: u32,
    pub field_0x01c8: u64,
    pub field_0x0236: u16,
}

impl ccGobjChimBase {
    pub const CLASS_NAME: &'static str = "ccGobjChimBase";
    pub const VTABLE_RVA: u32 = 0x008bcb38;
    pub const METHOD_COUNT: usize = 17;
    pub const BASE_CLASSES: &'static [&'static str] = &["ccGameObj"];
    pub const VIRTUAL_METHODS: &'static [u64] = &[0x00000001804a3d00, 0x00000001804a4c00, 0x000000018045e4c0, 0x00000001800d3090, 0x000000018019e550, 0x00000001804a5740, 0x00000001804a6310, 0x00000001800d3090, 0x00000001800d3090, 0x00000001804a68f0, 0x00000001804a5e00, 0x000000018045e2d0, 0x000000018045eec0, 0x000000018045efd0, 0x00000001804a77b0, 0x00000001804a7a50, 0x00000001804a4750];

    pub fn new() -> Self {
        Self {
            vtable_rva: Self::VTABLE_RVA,
            field_0x0000: Default::default(),
            field_0x0009: Default::default(),
            field_0x0010: Default::default(),
            field_0x0018: Default::default(),
            field_0x001c: Default::default(),
            field_0x0020: Default::default(),
            field_0x0024: Default::default(),
            field_0x002c: Default::default(),
            field_0x0040: Default::default(),
            field_0x0050: Default::default(),
            field_0x0058: Default::default(),
            field_0x0090: Default::default(),
            field_0x0094: Default::default(),
            field_0x0098: Default::default(),
            field_0x009c: Default::default(),
            field_0x00a0: Default::default(),
            field_0x00a4: Default::default(),
            field_0x00a8: Default::default(),
            field_0x00ac: Default::default(),
            field_0x00b0: Default::default(),
            field_0x00b8: Default::default(),
            field_0x00c4: Default::default(),
            field_0x00d0: Default::default(),
            field_0x0120: Default::default(),
            field_0x0124: Default::default(),
            field_0x0128: Default::default(),
            field_0x012c: Default::default(),
            field_0x0130: Default::default(),
            field_0x0134: Default::default(),
            field_0x0138: Default::default(),
            field_0x013c: Default::default(),
            field_0x0140: Default::default(),
            field_0x0144: Default::default(),
            field_0x0148: Default::default(),
            field_0x014c: Default::default(),
            field_0x0150: Default::default(),
            field_0x0154: Default::default(),
            field_0x0158: Default::default(),
            field_0x015c: Default::default(),
            field_0x01b8: Default::default(),
            field_0x01c0: Default::default(),
            field_0x01c8: Default::default(),
            field_0x0236: Default::default(),
        }
    }

    /// Decompiled virtual method #0 @ RVA 0x004a3d00 (24 instructions, 3 basic blocks)
    pub fn method_00(&mut self) {
        // Writes to field at offset +0x0000
    }

    /// Decompiled virtual method #1 @ RVA 0x004a4c00 (114 instructions, 25 basic blocks)
    pub fn method_01(&mut self) {
    }

    /// Decompiled virtual method #2 @ RVA 0x0045e4c0 (42 instructions, 4 basic blocks)
    pub fn method_02(&mut self) {
        // Writes to field at offset +0x0090
        // Writes to field at offset +0x0094
        // Writes to field at offset +0x0098
        // Writes to field at offset +0x009c
        // Writes to field at offset +0x00a0
        // Writes to field at offset +0x00a4
        // Writes to field at offset +0x00a8
        // Writes to field at offset +0x00ac
        // Writes to field at offset +0x0140
        // Writes to field at offset +0x0144
        // Writes to field at offset +0x0148
        // Writes to field at offset +0x014c
        // Writes to field at offset +0x0150
        // Writes to field at offset +0x0154
        // Writes to field at offset +0x0158
        // Writes to field at offset +0x015c
    }

    /// Decompiled virtual method #3 @ RVA 0x000d3090 (1 instructions, 1 basic blocks)
    pub fn method_03(&mut self) {
    }

    /// Decompiled virtual method #4 @ RVA 0x0019e550 (2 instructions, 1 basic blocks)
    pub fn method_04(&mut self) {
        // Writes to field at offset +0x00c4
    }

    /// Decompiled virtual method #5 @ RVA 0x004a5740 (134 instructions, 37 basic blocks)
    pub fn method_05(&mut self) {
        // Writes to field at offset +0x0024
    }

    /// Decompiled virtual method #6 @ RVA 0x004a6310 (103 instructions, 17 basic blocks)
    pub fn method_06(&mut self) {
        // Writes to field at offset +0x0009
    }

    /// Decompiled virtual method #7 @ RVA 0x000d3090 (1 instructions, 1 basic blocks)
    pub fn method_07(&mut self) {
    }

    /// Decompiled virtual method #8 @ RVA 0x000d3090 (1 instructions, 1 basic blocks)
    pub fn method_08(&mut self) {
    }

    /// Decompiled virtual method #9 @ RVA 0x004a68f0 (98 instructions, 11 basic blocks)
    pub fn method_09(&mut self) {
        // Writes to field at offset +0x0009
        // Writes to field at offset +0x00a4
        // Writes to field at offset +0x00d0
    }

    /// Decompiled virtual method #10 @ RVA 0x004a5e00 (97 instructions, 16 basic blocks)
    pub fn method_10(&mut self) {
        // Writes to field at offset +0x00c4
    }

    /// Decompiled virtual method #11 @ RVA 0x0045e2d0 (33 instructions, 3 basic blocks)
    pub fn method_11(&mut self) {
        // Writes to field at offset +0x0090
        // Writes to field at offset +0x00a0
        // Writes to field at offset +0x00a4
        // Writes to field at offset +0x00a8
        // Writes to field at offset +0x00ac
    }

    /// Decompiled virtual method #12 @ RVA 0x0045eec0 (67 instructions, 22 basic blocks)
    pub fn method_12(&mut self) {
        // Writes to field at offset +0x001c
        // Writes to field at offset +0x0040
        // Writes to field at offset +0x0090
        // Writes to field at offset +0x00b0
    }

    /// Decompiled virtual method #13 @ RVA 0x0045efd0 (27 instructions, 5 basic blocks)
    pub fn method_13(&mut self) {
        // Writes to field at offset +0x0090
        // Writes to field at offset +0x00b0
    }

    /// Decompiled virtual method #14 @ RVA 0x004a77b0 (91 instructions, 12 basic blocks)
    pub fn method_14(&mut self) {
        // Writes to field at offset +0x00d0
        // Writes to field at offset +0x0236
    }

    /// Decompiled virtual method #15 @ RVA 0x004a7a50 (30 instructions, 8 basic blocks)
    pub fn method_15(&mut self) {
        // Writes to field at offset +0x00d0
    }

    /// Decompiled virtual method #16 @ RVA 0x004a4750 (129 instructions, 21 basic blocks)
    pub fn method_16(&mut self) {
    }

}

impl CcClass for ccGobjChimBase {
    fn class_name(&self) -> &'static str { Self::CLASS_NAME }
    fn vtable_rva(&self) -> u32 { Self::VTABLE_RVA }
    fn virtual_method_count(&self) -> usize { Self::METHOD_COUNT }
    fn base_classes(&self) -> &'static [&'static str] { Self::BASE_CLASSES }
}

/// Decompiled CC2 class: `ccGobjDestroyer` (VTable RVA: 0x008be240, 14 virtual methods, 52 recovered fields)
/// Inherits: ccGobjGimBaseChangeCCS, ccGobjGimBase, ccGameObj
#[derive(Debug, Clone, Default)]
pub struct ccGobjDestroyer {
    pub vtable_rva: u32,
    pub field_0x0000: u64,
    pub field_0x0010: u32,
    pub field_0x0014: u32,
    pub field_0x0018: u32,
    pub field_0x001c: u32,
    pub field_0x0020: u32,
    pub field_0x0024: u16,
    pub field_0x002c: u32,
    pub field_0x0036: u8,
    pub field_0x0038: u8,
    pub field_0x003a: u8,
    pub field_0x0040: u32,
    pub field_0x0090: u32,
    pub field_0x0094: u32,
    pub field_0x0098: u32,
    pub field_0x009c: u32,
    pub field_0x00a0: u32,
    pub field_0x00a4: u32,
    pub field_0x00a8: u32,
    pub field_0x00ac: u32,
    pub field_0x00b0: u64,
    pub field_0x00b8: u64,
    pub field_0x00c4: u32,
    pub field_0x00e0: u32,
    pub field_0x0120: u32,
    pub field_0x0124: u32,
    pub field_0x0128: u32,
    pub field_0x012c: u32,
    pub field_0x0130: u32,
    pub field_0x0134: u32,
    pub field_0x0138: u32,
    pub field_0x013c: u32,
    pub field_0x0140: u32,
    pub field_0x0144: u32,
    pub field_0x0148: u32,
    pub field_0x014c: u32,
    pub field_0x0150: u32,
    pub field_0x0154: u32,
    pub field_0x0158: u32,
    pub field_0x015c: u32,
    pub field_0x0198: u32,
    pub field_0x019c: u16,
    pub field_0x019e: u16,
    pub field_0x01c0: u16,
    pub field_0x01c4: u64,
    pub field_0x01c8: u16,
    pub field_0x01cc: u16,
    pub field_0x01ce: u8,
    pub field_0x01d0: u32,
    pub field_0x01f0: u64,
    pub field_0x01f8: u32,
    pub field_0x01fc: u32,
}

impl ccGobjDestroyer {
    pub const CLASS_NAME: &'static str = "ccGobjDestroyer";
    pub const VTABLE_RVA: u32 = 0x008be240;
    pub const METHOD_COUNT: usize = 14;
    pub const BASE_CLASSES: &'static [&'static str] = &["ccGobjGimBaseChangeCCS", "ccGobjGimBase", "ccGameObj"];
    pub const VIRTUAL_METHODS: &'static [u64] = &[0x00000001804f3da0, 0x00000001804f4850, 0x000000018045e4c0, 0x00000001800d3090, 0x000000018019e550, 0x00000001804f6040, 0x00000001800d3090, 0x00000001800d3090, 0x00000001800d3090, 0x00000001804f5f80, 0x00000001804f4e90, 0x000000018045e2d0, 0x000000018045eec0, 0x000000018045efd0];

    pub fn new() -> Self {
        Self {
            vtable_rva: Self::VTABLE_RVA,
            field_0x0000: Default::default(),
            field_0x0010: Default::default(),
            field_0x0014: Default::default(),
            field_0x0018: Default::default(),
            field_0x001c: Default::default(),
            field_0x0020: Default::default(),
            field_0x0024: Default::default(),
            field_0x002c: Default::default(),
            field_0x0036: Default::default(),
            field_0x0038: Default::default(),
            field_0x003a: Default::default(),
            field_0x0040: Default::default(),
            field_0x0090: Default::default(),
            field_0x0094: Default::default(),
            field_0x0098: Default::default(),
            field_0x009c: Default::default(),
            field_0x00a0: Default::default(),
            field_0x00a4: Default::default(),
            field_0x00a8: Default::default(),
            field_0x00ac: Default::default(),
            field_0x00b0: Default::default(),
            field_0x00b8: Default::default(),
            field_0x00c4: Default::default(),
            field_0x00e0: Default::default(),
            field_0x0120: Default::default(),
            field_0x0124: Default::default(),
            field_0x0128: Default::default(),
            field_0x012c: Default::default(),
            field_0x0130: Default::default(),
            field_0x0134: Default::default(),
            field_0x0138: Default::default(),
            field_0x013c: Default::default(),
            field_0x0140: Default::default(),
            field_0x0144: Default::default(),
            field_0x0148: Default::default(),
            field_0x014c: Default::default(),
            field_0x0150: Default::default(),
            field_0x0154: Default::default(),
            field_0x0158: Default::default(),
            field_0x015c: Default::default(),
            field_0x0198: Default::default(),
            field_0x019c: Default::default(),
            field_0x019e: Default::default(),
            field_0x01c0: Default::default(),
            field_0x01c4: Default::default(),
            field_0x01c8: Default::default(),
            field_0x01cc: Default::default(),
            field_0x01ce: Default::default(),
            field_0x01d0: Default::default(),
            field_0x01f0: Default::default(),
            field_0x01f8: Default::default(),
            field_0x01fc: Default::default(),
        }
    }

    /// Decompiled virtual method #0 @ RVA 0x004f3da0 (20 instructions, 3 basic blocks)
    pub fn method_00(&mut self) {
        // Writes to field at offset +0x0000
    }

    /// Decompiled virtual method #1 @ RVA 0x004f4850 (99 instructions, 2 basic blocks)
    pub fn method_01(&mut self) {
        // Writes to field at offset +0x00c4
    }

    /// Decompiled virtual method #2 @ RVA 0x0045e4c0 (42 instructions, 4 basic blocks)
    pub fn method_02(&mut self) {
        // Writes to field at offset +0x0090
        // Writes to field at offset +0x0094
        // Writes to field at offset +0x0098
        // Writes to field at offset +0x009c
        // Writes to field at offset +0x00a0
        // Writes to field at offset +0x00a4
        // Writes to field at offset +0x00a8
        // Writes to field at offset +0x00ac
        // Writes to field at offset +0x0140
        // Writes to field at offset +0x0144
        // Writes to field at offset +0x0148
        // Writes to field at offset +0x014c
        // Writes to field at offset +0x0150
        // Writes to field at offset +0x0154
        // Writes to field at offset +0x0158
        // Writes to field at offset +0x015c
    }

    /// Decompiled virtual method #3 @ RVA 0x000d3090 (1 instructions, 1 basic blocks)
    pub fn method_03(&mut self) {
    }

    /// Decompiled virtual method #4 @ RVA 0x0019e550 (2 instructions, 1 basic blocks)
    pub fn method_04(&mut self) {
        // Writes to field at offset +0x00c4
    }

    /// Decompiled virtual method #5 @ RVA 0x004f6040 (62 instructions, 17 basic blocks)
    pub fn method_05(&mut self) {
        // Writes to field at offset +0x0198
        // Writes to field at offset +0x019c
        // Writes to field at offset +0x019e
        // Writes to field at offset +0x01c4
        // Writes to field at offset +0x01cc
        // Writes to field at offset +0x01ce
        // Writes to field at offset +0x01f0
        // Writes to field at offset +0x01f8
        // Writes to field at offset +0x01fc
    }

    /// Decompiled virtual method #6 @ RVA 0x000d3090 (1 instructions, 1 basic blocks)
    pub fn method_06(&mut self) {
    }

    /// Decompiled virtual method #7 @ RVA 0x000d3090 (1 instructions, 1 basic blocks)
    pub fn method_07(&mut self) {
    }

    /// Decompiled virtual method #8 @ RVA 0x000d3090 (1 instructions, 1 basic blocks)
    pub fn method_08(&mut self) {
    }

    /// Decompiled virtual method #9 @ RVA 0x004f5f80 (45 instructions, 12 basic blocks)
    pub fn method_09(&mut self) {
    }

    /// Decompiled virtual method #10 @ RVA 0x004f4e90 (77 instructions, 5 basic blocks)
    pub fn method_10(&mut self) {
        // Writes to field at offset +0x002c
        // Writes to field at offset +0x0036
        // Writes to field at offset +0x0038
        // Writes to field at offset +0x003a
        // Writes to field at offset +0x0090
        // Writes to field at offset +0x0094
        // Writes to field at offset +0x0098
        // Writes to field at offset +0x009c
        // Writes to field at offset +0x00a0
        // Writes to field at offset +0x00a4
        // Writes to field at offset +0x00a8
        // Writes to field at offset +0x00ac
    }

    /// Decompiled virtual method #11 @ RVA 0x0045e2d0 (33 instructions, 3 basic blocks)
    pub fn method_11(&mut self) {
        // Writes to field at offset +0x0090
        // Writes to field at offset +0x00a0
        // Writes to field at offset +0x00a4
        // Writes to field at offset +0x00a8
        // Writes to field at offset +0x00ac
    }

    /// Decompiled virtual method #12 @ RVA 0x0045eec0 (67 instructions, 22 basic blocks)
    pub fn method_12(&mut self) {
        // Writes to field at offset +0x001c
        // Writes to field at offset +0x0040
        // Writes to field at offset +0x0090
        // Writes to field at offset +0x00b0
    }

    /// Decompiled virtual method #13 @ RVA 0x0045efd0 (27 instructions, 5 basic blocks)
    pub fn method_13(&mut self) {
        // Writes to field at offset +0x0090
        // Writes to field at offset +0x00b0
    }

}

impl CcClass for ccGobjDestroyer {
    fn class_name(&self) -> &'static str { Self::CLASS_NAME }
    fn vtable_rva(&self) -> u32 { Self::VTABLE_RVA }
    fn virtual_method_count(&self) -> usize { Self::METHOD_COUNT }
    fn base_classes(&self) -> &'static [&'static str] { Self::BASE_CLASSES }
}

/// Decompiled CC2 class: `ccGobjDoppel` (VTable RVA: 0x008bced8, 23 virtual methods, 96 recovered fields)
/// Inherits: ccGobjSpc, ccGobjBattle, ccGameObj
#[derive(Debug, Clone, Default)]
pub struct ccGobjDoppel {
    pub vtable_rva: u32,
    pub field_0x0000: u32,
    pub field_0x0001: u32,
    pub field_0x0002: u16,
    pub field_0x0004: u32,
    pub field_0x0008: u16,
    pub field_0x0009: u8,
    pub field_0x000a: u16,
    pub field_0x000c: u32,
    pub field_0x0010: u64,
    pub field_0x0018: u64,
    pub field_0x001c: u8,
    pub field_0x0020: u32,
    pub field_0x0024: u16,
    pub field_0x0028: u32,
    pub field_0x002c: u32,
    pub field_0x0036: u8,
    pub field_0x0038: u8,
    pub field_0x003a: u8,
    pub field_0x0040: u32,
    pub field_0x0050: u64,
    pub field_0x0058: u64,
    pub field_0x005c: u32,
    pub field_0x0060: u32,
    pub field_0x0064: u32,
    pub field_0x0068: u64,
    pub field_0x0070: u64,
    pub field_0x0078: u32,
    pub field_0x007c: u64,
    pub field_0x0084: u64,
    pub field_0x008c: u32,
    pub field_0x0090: u64,
    pub field_0x0094: u32,
    pub field_0x0098: u32,
    pub field_0x009c: u32,
    pub field_0x00a0: u32,
    pub field_0x00a4: u32,
    pub field_0x00a8: u32,
    pub field_0x00ac: u32,
    pub field_0x00b0: u64,
    pub field_0x00b8: u64,
    pub field_0x00c4: u32,
    pub field_0x00e0: u32,
    pub field_0x00e4: u32,
    pub field_0x00e8: u32,
    pub field_0x00ec: u32,
    pub field_0x00f0: u32,
    pub field_0x00f4: u32,
    pub field_0x00f8: f32,
    pub field_0x0120: u32,
    pub field_0x0124: u32,
    pub field_0x0128: u32,
    pub field_0x012c: u32,
    pub field_0x0130: u32,
    pub field_0x0134: u32,
    pub field_0x0138: u32,
    pub field_0x013c: u32,
    pub field_0x0140: u32,
    pub field_0x0144: u32,
    pub field_0x0148: u32,
    pub field_0x014c: u32,
    pub field_0x0150: u32,
    pub field_0x0154: u32,
    pub field_0x0158: u32,
    pub field_0x015c: u32,
    pub field_0x01c0: u64,
    pub field_0x01c8: u32,
    pub field_0x01e4: u16,
    pub field_0x01e6: u16,
    pub field_0x020c: u64,
    pub field_0x0214: u64,
    pub field_0x021c: u64,
    pub field_0x0224: u64,
    pub field_0x022c: u32,
    pub field_0x0230: [f32; 4],
    pub field_0x0234: u64,
    pub field_0x023c: u64,
    pub field_0x0240: u32,
    pub field_0x0244: u64,
    pub field_0x024c: u64,
    pub field_0x0254: u32,
    pub field_0x0278: u32,
    pub field_0x02da: u16,
    pub field_0x02fc: u8,
    pub field_0x0300: u32,
    pub field_0x0304: u32,
    pub field_0x0308: u32,
    pub field_0x030c: u32,
    pub field_0x0314: u16,
    pub field_0x03e0: u32,
    pub field_0x03f0: u64,
    pub field_0x0400: u64,
    pub field_0x0410: u64,
    pub field_0x0440: u32,
    pub field_0x04f0: u32,
    pub field_0x0660: u32,
    pub field_0x0668: u32,
}

impl ccGobjDoppel {
    pub const CLASS_NAME: &'static str = "ccGobjDoppel";
    pub const VTABLE_RVA: u32 = 0x008bced8;
    pub const METHOD_COUNT: usize = 23;
    pub const BASE_CLASSES: &'static [&'static str] = &["ccGobjSpc", "ccGobjBattle", "ccGameObj"];
    pub const VIRTUAL_METHODS: &'static [u64] = &[0x00000001804acd10, 0x00000001804acf10, 0x000000018045e4c0, 0x0000000180517180, 0x000000018047ebd0, 0x00000001804af140, 0x00000001804ad990, 0x00000001800d3090, 0x000000018050aa50, 0x00000001804ae5f0, 0x00000001804ad500, 0x000000018047f0e0, 0x000000018045eec0, 0x000000018045efd0, 0x000000018051d300, 0x00000001805191c0, 0x0000000180518920, 0x000000018051d220, 0x0000000180518720, 0x00000001805187c0, 0x0000000180518850, 0x000000018051d510, 0x0000000180508900];

    pub fn new() -> Self {
        Self {
            vtable_rva: Self::VTABLE_RVA,
            field_0x0000: Default::default(),
            field_0x0001: Default::default(),
            field_0x0002: Default::default(),
            field_0x0004: Default::default(),
            field_0x0008: Default::default(),
            field_0x0009: Default::default(),
            field_0x000a: Default::default(),
            field_0x000c: Default::default(),
            field_0x0010: Default::default(),
            field_0x0018: Default::default(),
            field_0x001c: Default::default(),
            field_0x0020: Default::default(),
            field_0x0024: Default::default(),
            field_0x0028: Default::default(),
            field_0x002c: Default::default(),
            field_0x0036: Default::default(),
            field_0x0038: Default::default(),
            field_0x003a: Default::default(),
            field_0x0040: Default::default(),
            field_0x0050: Default::default(),
            field_0x0058: Default::default(),
            field_0x005c: Default::default(),
            field_0x0060: Default::default(),
            field_0x0064: Default::default(),
            field_0x0068: Default::default(),
            field_0x0070: Default::default(),
            field_0x0078: Default::default(),
            field_0x007c: Default::default(),
            field_0x0084: Default::default(),
            field_0x008c: Default::default(),
            field_0x0090: Default::default(),
            field_0x0094: Default::default(),
            field_0x0098: Default::default(),
            field_0x009c: Default::default(),
            field_0x00a0: Default::default(),
            field_0x00a4: Default::default(),
            field_0x00a8: Default::default(),
            field_0x00ac: Default::default(),
            field_0x00b0: Default::default(),
            field_0x00b8: Default::default(),
            field_0x00c4: Default::default(),
            field_0x00e0: Default::default(),
            field_0x00e4: Default::default(),
            field_0x00e8: Default::default(),
            field_0x00ec: Default::default(),
            field_0x00f0: Default::default(),
            field_0x00f4: Default::default(),
            field_0x00f8: Default::default(),
            field_0x0120: Default::default(),
            field_0x0124: Default::default(),
            field_0x0128: Default::default(),
            field_0x012c: Default::default(),
            field_0x0130: Default::default(),
            field_0x0134: Default::default(),
            field_0x0138: Default::default(),
            field_0x013c: Default::default(),
            field_0x0140: Default::default(),
            field_0x0144: Default::default(),
            field_0x0148: Default::default(),
            field_0x014c: Default::default(),
            field_0x0150: Default::default(),
            field_0x0154: Default::default(),
            field_0x0158: Default::default(),
            field_0x015c: Default::default(),
            field_0x01c0: Default::default(),
            field_0x01c8: Default::default(),
            field_0x01e4: Default::default(),
            field_0x01e6: Default::default(),
            field_0x020c: Default::default(),
            field_0x0214: Default::default(),
            field_0x021c: Default::default(),
            field_0x0224: Default::default(),
            field_0x022c: Default::default(),
            field_0x0230: Default::default(),
            field_0x0234: Default::default(),
            field_0x023c: Default::default(),
            field_0x0240: Default::default(),
            field_0x0244: Default::default(),
            field_0x024c: Default::default(),
            field_0x0254: Default::default(),
            field_0x0278: Default::default(),
            field_0x02da: Default::default(),
            field_0x02fc: Default::default(),
            field_0x0300: Default::default(),
            field_0x0304: Default::default(),
            field_0x0308: Default::default(),
            field_0x030c: Default::default(),
            field_0x0314: Default::default(),
            field_0x03e0: Default::default(),
            field_0x03f0: Default::default(),
            field_0x0400: Default::default(),
            field_0x0410: Default::default(),
            field_0x0440: Default::default(),
            field_0x04f0: Default::default(),
            field_0x0660: Default::default(),
            field_0x0668: Default::default(),
        }
    }

    /// Decompiled virtual method #0 @ RVA 0x004acd10 (16 instructions, 3 basic blocks)
    pub fn method_00(&mut self) {
    }

    /// Decompiled virtual method #1 @ RVA 0x004acf10 (115 instructions, 20 basic blocks)
    pub fn method_01(&mut self) {
        // Writes to field at offset +0x002c
        // Writes to field at offset +0x0036
        // Writes to field at offset +0x0038
        // Writes to field at offset +0x003a
        // Writes to field at offset +0x00c4
        // Writes to field at offset +0x0660
    }

    /// Decompiled virtual method #2 @ RVA 0x0045e4c0 (42 instructions, 4 basic blocks)
    pub fn method_02(&mut self) {
        // Writes to field at offset +0x0090
        // Writes to field at offset +0x0094
        // Writes to field at offset +0x0098
        // Writes to field at offset +0x009c
        // Writes to field at offset +0x00a0
        // Writes to field at offset +0x00a4
        // Writes to field at offset +0x00a8
        // Writes to field at offset +0x00ac
        // Writes to field at offset +0x0140
        // Writes to field at offset +0x0144
        // Writes to field at offset +0x0148
        // Writes to field at offset +0x014c
        // Writes to field at offset +0x0150
        // Writes to field at offset +0x0154
        // Writes to field at offset +0x0158
        // Writes to field at offset +0x015c
    }

    /// Decompiled virtual method #3 @ RVA 0x00517180 (96 instructions, 32 basic blocks)
    pub fn method_03(&mut self) {
        // Writes to field at offset +0x0018
        // Writes to field at offset +0x00b0
    }

    /// Decompiled virtual method #4 @ RVA 0x0047ebd0 (45 instructions, 12 basic blocks)
    pub fn method_04(&mut self) {
        // Writes to field at offset +0x0004
        // Writes to field at offset +0x02da
        // Writes to field at offset +0x0314
        // Writes to field at offset +0x03e0
    }

    /// Decompiled virtual method #5 @ RVA 0x004af140 (129 instructions, 26 basic blocks)
    pub fn method_05(&mut self) {
    }

    /// Decompiled virtual method #6 @ RVA 0x004ad990 (112 instructions, 16 basic blocks)
    pub fn method_06(&mut self) {
        // Writes to field at offset +0x0009
    }

    /// Decompiled virtual method #7 @ RVA 0x000d3090 (1 instructions, 1 basic blocks)
    pub fn method_07(&mut self) {
    }

    /// Decompiled virtual method #8 @ RVA 0x0050aa50 (30 instructions, 6 basic blocks)
    pub fn method_08(&mut self) {
        // Writes to field at offset +0x00c4
        // Writes to field at offset +0x04f0
    }

    /// Decompiled virtual method #9 @ RVA 0x004ae5f0 (117 instructions, 26 basic blocks)
    pub fn method_09(&mut self) {
        // Writes to field at offset +0x0004
        // Writes to field at offset +0x0008
        // Writes to field at offset +0x0009
    }

    /// Decompiled virtual method #10 @ RVA 0x004ad500 (90 instructions, 6 basic blocks)
    pub fn method_10(&mut self) {
        // Writes to field at offset +0x0050
        // Writes to field at offset +0x0058
        // Writes to field at offset +0x0058
        // Writes to field at offset +0x005c
        // Writes to field at offset +0x0060
        // Writes to field at offset +0x0064
        // Writes to field at offset +0x0068
        // Writes to field at offset +0x0070
        // Writes to field at offset +0x0078
        // Writes to field at offset +0x007c
        // Writes to field at offset +0x0084
        // Writes to field at offset +0x008c
        // Writes to field at offset +0x0090
        // Writes to field at offset +0x0090
        // Writes to field at offset +0x0094
        // Writes to field at offset +0x0098
        // Writes to field at offset +0x009c
        // Writes to field at offset +0x00a0
        // Writes to field at offset +0x00a4
        // Writes to field at offset +0x00a8
        // Writes to field at offset +0x00ac
    }

    /// Decompiled virtual method #11 @ RVA 0x0047f0e0 (15 instructions, 4 basic blocks)
    pub fn method_11(&mut self) {
    }

    /// Decompiled virtual method #12 @ RVA 0x0045eec0 (67 instructions, 22 basic blocks)
    pub fn method_12(&mut self) {
        // Writes to field at offset +0x001c
        // Writes to field at offset +0x0040
        // Writes to field at offset +0x0090
        // Writes to field at offset +0x00b0
    }

    /// Decompiled virtual method #13 @ RVA 0x0045efd0 (27 instructions, 5 basic blocks)
    pub fn method_13(&mut self) {
        // Writes to field at offset +0x0090
        // Writes to field at offset +0x00b0
    }

    /// Decompiled virtual method #14 @ RVA 0x0051d300 (107 instructions, 1 basic blocks)
    pub fn method_14(&mut self) {
        // Writes to field at offset +0x0000
        // Writes to field at offset +0x0004
        // Writes to field at offset +0x0008
        // Writes to field at offset +0x000a
        // Writes to field at offset +0x020c
        // Writes to field at offset +0x0214
        // Writes to field at offset +0x021c
        // Writes to field at offset +0x0224
        // Writes to field at offset +0x022c
        // Writes to field at offset +0x0234
        // Writes to field at offset +0x023c
        // Writes to field at offset +0x0244
        // Writes to field at offset +0x024c
        // Writes to field at offset +0x0254
    }

    /// Decompiled virtual method #15 @ RVA 0x005191c0 (107 instructions, 35 basic blocks)
    pub fn method_15(&mut self) {
        // Writes to field at offset +0x0004
        // Writes to field at offset +0x00c4
        // Writes to field at offset +0x01e4
    }

    /// Decompiled virtual method #16 @ RVA 0x00518920 (120 instructions, 25 basic blocks)
    pub fn method_16(&mut self) {
    }

    /// Decompiled virtual method #17 @ RVA 0x0051d220 (52 instructions, 11 basic blocks)
    pub fn method_17(&mut self) {
    }

    /// Decompiled virtual method #18 @ RVA 0x00518720 (40 instructions, 11 basic blocks)
    pub fn method_18(&mut self) {
        // Writes to field at offset +0x0400
    }

    /// Decompiled virtual method #19 @ RVA 0x005187c0 (38 instructions, 11 basic blocks)
    pub fn method_19(&mut self) {
    }

    /// Decompiled virtual method #20 @ RVA 0x00518850 (50 instructions, 15 basic blocks)
    pub fn method_20(&mut self) {
    }

    /// Decompiled virtual method #21 @ RVA 0x0051d510 (9 instructions, 3 basic blocks)
    pub fn method_21(&mut self) {
        // Writes to field at offset +0x0028
        // Writes to field at offset +0x002c
        // Writes to field at offset +0x02fc
    }

    /// Decompiled virtual method #22 @ RVA 0x00508900 (125 instructions, 21 basic blocks)
    pub fn method_22(&mut self) {
        // Writes to field at offset +0x01c0
    }

}

impl CcClass for ccGobjDoppel {
    fn class_name(&self) -> &'static str { Self::CLASS_NAME }
    fn vtable_rva(&self) -> u32 { Self::VTABLE_RVA }
    fn virtual_method_count(&self) -> usize { Self::METHOD_COUNT }
    fn base_classes(&self) -> &'static [&'static str] { Self::BASE_CLASSES }
}

/// Decompiled CC2 class: `ccGobjDraw` (VTable RVA: 0x008c0928, 1 virtual methods, 0 recovered fields)
#[derive(Debug, Clone, Default)]
pub struct ccGobjDraw {
    pub vtable_rva: u32,
}

impl ccGobjDraw {
    pub const CLASS_NAME: &'static str = "ccGobjDraw";
    pub const VTABLE_RVA: u32 = 0x008c0928;
    pub const METHOD_COUNT: usize = 1;
    pub const BASE_CLASSES: &'static [&'static str] = &[];
    pub const VIRTUAL_METHODS: &'static [u64] = &[0x0000000180522be0];

    pub fn new() -> Self {
        Self {
            vtable_rva: Self::VTABLE_RVA,
        }
    }

    /// Decompiled virtual method #0 @ RVA 0x00522be0 (16 instructions, 3 basic blocks)
    pub fn method_00(&mut self) {
    }

}

impl CcClass for ccGobjDraw {
    fn class_name(&self) -> &'static str { Self::CLASS_NAME }
    fn vtable_rva(&self) -> u32 { Self::VTABLE_RVA }
    fn virtual_method_count(&self) -> usize { Self::METHOD_COUNT }
    fn base_classes(&self) -> &'static [&'static str] { Self::BASE_CLASSES }
}

/// Decompiled CC2 class: `ccGobjDrawOpt` (VTable RVA: 0x008c0908, 3 virtual methods, 8 recovered fields)
#[derive(Debug, Clone, Default)]
pub struct ccGobjDrawOpt {
    pub vtable_rva: u32,
    pub field_0x0020: u64,
    pub field_0x0028: u64,
    pub field_0x0050: u32,
    pub field_0x0058: u64,
    pub field_0x0062: u16,
    pub field_0x0064: u32,
    pub field_0x0078: u32,
    pub field_0x00c1: u8,
}

impl ccGobjDrawOpt {
    pub const CLASS_NAME: &'static str = "ccGobjDrawOpt";
    pub const VTABLE_RVA: u32 = 0x008c0908;
    pub const METHOD_COUNT: usize = 3;
    pub const BASE_CLASSES: &'static [&'static str] = &[];
    pub const VIRTUAL_METHODS: &'static [u64] = &[0x0000000180526330, 0x0000000180526a90, 0x0000000180526b80];

    pub fn new() -> Self {
        Self {
            vtable_rva: Self::VTABLE_RVA,
            field_0x0020: Default::default(),
            field_0x0028: Default::default(),
            field_0x0050: Default::default(),
            field_0x0058: Default::default(),
            field_0x0062: Default::default(),
            field_0x0064: Default::default(),
            field_0x0078: Default::default(),
            field_0x00c1: Default::default(),
        }
    }

    /// Decompiled virtual method #0 @ RVA 0x00526330 (16 instructions, 3 basic blocks)
    pub fn method_00(&mut self) {
    }

    /// Decompiled virtual method #1 @ RVA 0x00526a90 (19 instructions, 4 basic blocks)
    pub fn method_01(&mut self) {
        // Writes to field at offset +0x0058
        // Writes to field at offset +0x0062
        // Writes to field at offset +0x00c1
    }

    /// Decompiled virtual method #2 @ RVA 0x00526b80 (90 instructions, 11 basic blocks)
    pub fn method_02(&mut self) {
        // Writes to field at offset +0x0050
        // Writes to field at offset +0x0058
        // Writes to field at offset +0x0064
        // Writes to field at offset +0x0078
    }

}

impl CcClass for ccGobjDrawOpt {
    fn class_name(&self) -> &'static str { Self::CLASS_NAME }
    fn vtable_rva(&self) -> u32 { Self::VTABLE_RVA }
    fn virtual_method_count(&self) -> usize { Self::METHOD_COUNT }
    fn base_classes(&self) -> &'static [&'static str] { Self::BASE_CLASSES }
}

/// Decompiled CC2 class: `ccGobjEnemy` (VTable RVA: 0x008bdb58, 24 virtual methods, 68 recovered fields)
/// Inherits: ccGobjBattle, ccGameObj
#[derive(Debug, Clone, Default)]
pub struct ccGobjEnemy {
    pub vtable_rva: u32,
    pub field_0x0000: u8,
    pub field_0x0001: u32,
    pub field_0x0004: u8,
    pub field_0x0008: u16,
    pub field_0x000a: u16,
    pub field_0x0010: u64,
    pub field_0x0018: u64,
    pub field_0x001c: u8,
    pub field_0x001e: u16,
    pub field_0x0020: u32,
    pub field_0x0024: u16,
    pub field_0x002c: u16,
    pub field_0x0030: u32,
    pub field_0x0034: f32,
    pub field_0x0038: f32,
    pub field_0x003c: f32,
    pub field_0x0040: u32,
    pub field_0x007e: u16,
    pub field_0x0090: u32,
    pub field_0x0094: u32,
    pub field_0x0098: u64,
    pub field_0x009c: u32,
    pub field_0x00a0: u32,
    pub field_0x00a4: u32,
    pub field_0x00a8: u64,
    pub field_0x00ac: u32,
    pub field_0x00b0: u64,
    pub field_0x00b8: u64,
    pub field_0x00c4: u32,
    pub field_0x00e0: f32,
    pub field_0x00e4: f32,
    pub field_0x0120: u32,
    pub field_0x0124: u32,
    pub field_0x0128: u32,
    pub field_0x012c: u32,
    pub field_0x0130: u32,
    pub field_0x0134: u32,
    pub field_0x0138: u32,
    pub field_0x013c: u32,
    pub field_0x0140: u32,
    pub field_0x0144: u32,
    pub field_0x0148: u32,
    pub field_0x014c: u32,
    pub field_0x0150: u32,
    pub field_0x0154: u32,
    pub field_0x0158: u32,
    pub field_0x015c: u32,
    pub field_0x0170: u32,
    pub field_0x0198: u32,
    pub field_0x019c: u16,
    pub field_0x019e: u16,
    pub field_0x01b0: u64,
    pub field_0x01e4: u16,
    pub field_0x020c: u64,
    pub field_0x0214: u64,
    pub field_0x021c: u64,
    pub field_0x0224: u64,
    pub field_0x022c: u64,
    pub field_0x0234: u64,
    pub field_0x023c: u64,
    pub field_0x0244: u64,
    pub field_0x024c: u64,
    pub field_0x0254: u32,
    pub field_0x0278: u32,
    pub field_0x02da: u16,
    pub field_0x02fc: u8,
    pub field_0x0314: u16,
    pub field_0x03e0: u32,
}

impl ccGobjEnemy {
    pub const CLASS_NAME: &'static str = "ccGobjEnemy";
    pub const VTABLE_RVA: u32 = 0x008bdb58;
    pub const METHOD_COUNT: usize = 24;
    pub const BASE_CLASSES: &'static [&'static str] = &["ccGobjBattle", "ccGameObj"];
    pub const VIRTUAL_METHODS: &'static [u64] = &[0x00000001804b2860, 0x00000001804b3120, 0x000000018045e4c0, 0x00000001804b6a60, 0x000000018047ebd0, 0x000000018047f370, 0x00000001804b4ec0, 0x00000001800d3090, 0x00000001800d3090, 0x00000001804b3290, 0x000000018047f200, 0x000000018047f0e0, 0x000000018045eec0, 0x000000018045efd0, 0x00000001804b2f50, 0x00000001804b66c0, 0x00000001804b6040, 0x00000001804b80d0, 0x00000001800d3090, 0x00000001800d3090, 0x00000001800d3090, 0x0000000180103e70, 0x00000001804b2920, 0x00000001800d3090];

    pub fn new() -> Self {
        Self {
            vtable_rva: Self::VTABLE_RVA,
            field_0x0000: Default::default(),
            field_0x0001: Default::default(),
            field_0x0004: Default::default(),
            field_0x0008: Default::default(),
            field_0x000a: Default::default(),
            field_0x0010: Default::default(),
            field_0x0018: Default::default(),
            field_0x001c: Default::default(),
            field_0x001e: Default::default(),
            field_0x0020: Default::default(),
            field_0x0024: Default::default(),
            field_0x002c: Default::default(),
            field_0x0030: Default::default(),
            field_0x0034: Default::default(),
            field_0x0038: Default::default(),
            field_0x003c: Default::default(),
            field_0x0040: Default::default(),
            field_0x007e: Default::default(),
            field_0x0090: Default::default(),
            field_0x0094: Default::default(),
            field_0x0098: Default::default(),
            field_0x009c: Default::default(),
            field_0x00a0: Default::default(),
            field_0x00a4: Default::default(),
            field_0x00a8: Default::default(),
            field_0x00ac: Default::default(),
            field_0x00b0: Default::default(),
            field_0x00b8: Default::default(),
            field_0x00c4: Default::default(),
            field_0x00e0: Default::default(),
            field_0x00e4: Default::default(),
            field_0x0120: Default::default(),
            field_0x0124: Default::default(),
            field_0x0128: Default::default(),
            field_0x012c: Default::default(),
            field_0x0130: Default::default(),
            field_0x0134: Default::default(),
            field_0x0138: Default::default(),
            field_0x013c: Default::default(),
            field_0x0140: Default::default(),
            field_0x0144: Default::default(),
            field_0x0148: Default::default(),
            field_0x014c: Default::default(),
            field_0x0150: Default::default(),
            field_0x0154: Default::default(),
            field_0x0158: Default::default(),
            field_0x015c: Default::default(),
            field_0x0170: Default::default(),
            field_0x0198: Default::default(),
            field_0x019c: Default::default(),
            field_0x019e: Default::default(),
            field_0x01b0: Default::default(),
            field_0x01e4: Default::default(),
            field_0x020c: Default::default(),
            field_0x0214: Default::default(),
            field_0x021c: Default::default(),
            field_0x0224: Default::default(),
            field_0x022c: Default::default(),
            field_0x0234: Default::default(),
            field_0x023c: Default::default(),
            field_0x0244: Default::default(),
            field_0x024c: Default::default(),
            field_0x0254: Default::default(),
            field_0x0278: Default::default(),
            field_0x02da: Default::default(),
            field_0x02fc: Default::default(),
            field_0x0314: Default::default(),
            field_0x03e0: Default::default(),
        }
    }

    /// Decompiled virtual method #0 @ RVA 0x004b2860 (16 instructions, 3 basic blocks)
    pub fn method_00(&mut self) {
    }

    /// Decompiled virtual method #1 @ RVA 0x004b3120 (112 instructions, 15 basic blocks)
    pub fn method_01(&mut self) {
        // Writes to field at offset +0x0030
        // Writes to field at offset +0x00c4
    }

    /// Decompiled virtual method #2 @ RVA 0x0045e4c0 (42 instructions, 4 basic blocks)
    pub fn method_02(&mut self) {
        // Writes to field at offset +0x0090
        // Writes to field at offset +0x0094
        // Writes to field at offset +0x0098
        // Writes to field at offset +0x009c
        // Writes to field at offset +0x00a0
        // Writes to field at offset +0x00a4
        // Writes to field at offset +0x00a8
        // Writes to field at offset +0x00ac
        // Writes to field at offset +0x0140
        // Writes to field at offset +0x0144
        // Writes to field at offset +0x0148
        // Writes to field at offset +0x014c
        // Writes to field at offset +0x0150
        // Writes to field at offset +0x0154
        // Writes to field at offset +0x0158
        // Writes to field at offset +0x015c
    }

    /// Decompiled virtual method #3 @ RVA 0x004b6a60 (104 instructions, 18 basic blocks)
    pub fn method_03(&mut self) {
        // Writes to field at offset +0x0000
        // Writes to field at offset +0x0001
        // Writes to field at offset +0x0004
        // Writes to field at offset +0x00b0
    }

    /// Decompiled virtual method #4 @ RVA 0x0047ebd0 (45 instructions, 12 basic blocks)
    pub fn method_04(&mut self) {
        // Writes to field at offset +0x0004
        // Writes to field at offset +0x02da
        // Writes to field at offset +0x0314
        // Writes to field at offset +0x03e0
    }

    /// Decompiled virtual method #5 @ RVA 0x0047f370 (124 instructions, 30 basic blocks)
    pub fn method_05(&mut self) {
        // Writes to field at offset +0x0040
    }

    /// Decompiled virtual method #6 @ RVA 0x004b4ec0 (103 instructions, 17 basic blocks)
    pub fn method_06(&mut self) {
    }

    /// Decompiled virtual method #7 @ RVA 0x000d3090 (1 instructions, 1 basic blocks)
    pub fn method_07(&mut self) {
    }

    /// Decompiled virtual method #8 @ RVA 0x000d3090 (1 instructions, 1 basic blocks)
    pub fn method_08(&mut self) {
    }

    /// Decompiled virtual method #9 @ RVA 0x004b3290 (56 instructions, 9 basic blocks)
    pub fn method_09(&mut self) {
    }

    /// Decompiled virtual method #10 @ RVA 0x0047f200 (58 instructions, 4 basic blocks)
    pub fn method_10(&mut self) {
    }

    /// Decompiled virtual method #11 @ RVA 0x0047f0e0 (15 instructions, 4 basic blocks)
    pub fn method_11(&mut self) {
    }

    /// Decompiled virtual method #12 @ RVA 0x0045eec0 (67 instructions, 22 basic blocks)
    pub fn method_12(&mut self) {
        // Writes to field at offset +0x001c
        // Writes to field at offset +0x0040
        // Writes to field at offset +0x0090
        // Writes to field at offset +0x00b0
    }

    /// Decompiled virtual method #13 @ RVA 0x0045efd0 (27 instructions, 5 basic blocks)
    pub fn method_13(&mut self) {
        // Writes to field at offset +0x0090
        // Writes to field at offset +0x00b0
    }

    /// Decompiled virtual method #14 @ RVA 0x004b2f50 (105 instructions, 2 basic blocks)
    pub fn method_14(&mut self) {
        // Writes to field at offset +0x0000
        // Writes to field at offset +0x0004
        // Writes to field at offset +0x0008
        // Writes to field at offset +0x000a
        // Writes to field at offset +0x00c4
        // Writes to field at offset +0x020c
        // Writes to field at offset +0x0214
        // Writes to field at offset +0x021c
        // Writes to field at offset +0x0224
        // Writes to field at offset +0x022c
        // Writes to field at offset +0x0234
        // Writes to field at offset +0x023c
        // Writes to field at offset +0x0244
        // Writes to field at offset +0x024c
        // Writes to field at offset +0x0254
    }

    /// Decompiled virtual method #15 @ RVA 0x004b66c0 (44 instructions, 12 basic blocks)
    pub fn method_15(&mut self) {
        // Writes to field at offset +0x0000
        // Writes to field at offset +0x0004
        // Writes to field at offset +0x0008
        // Writes to field at offset +0x000a
        // Writes to field at offset +0x00c4
        // Writes to field at offset +0x01e4
        // Writes to field at offset +0x02fc
    }

    /// Decompiled virtual method #16 @ RVA 0x004b6040 (54 instructions, 12 basic blocks)
    pub fn method_16(&mut self) {
    }

    /// Decompiled virtual method #17 @ RVA 0x004b80d0 (38 instructions, 8 basic blocks)
    pub fn method_17(&mut self) {
    }

    /// Decompiled virtual method #18 @ RVA 0x000d3090 (1 instructions, 1 basic blocks)
    pub fn method_18(&mut self) {
    }

    /// Decompiled virtual method #19 @ RVA 0x000d3090 (1 instructions, 1 basic blocks)
    pub fn method_19(&mut self) {
    }

    /// Decompiled virtual method #20 @ RVA 0x000d3090 (1 instructions, 1 basic blocks)
    pub fn method_20(&mut self) {
    }

    /// Decompiled virtual method #21 @ RVA 0x00103e70 (2 instructions, 1 basic blocks)
    pub fn method_21(&mut self) {
    }

    /// Decompiled virtual method #22 @ RVA 0x004b2920 (22 instructions, 3 basic blocks)
    pub fn method_22(&mut self) {
        // Writes to field at offset +0x01b0
    }

    /// Decompiled virtual method #23 @ RVA 0x000d3090 (1 instructions, 1 basic blocks)
    pub fn method_23(&mut self) {
    }

}

impl CcClass for ccGobjEnemy {
    fn class_name(&self) -> &'static str { Self::CLASS_NAME }
    fn vtable_rva(&self) -> u32 { Self::VTABLE_RVA }
    fn virtual_method_count(&self) -> usize { Self::METHOD_COUNT }
    fn base_classes(&self) -> &'static [&'static str] { Self::BASE_CLASSES }
}

/// Decompiled CC2 class: `ccGobjEnemyEggx2` (VTable RVA: 0x008bd900, 24 virtual methods, 70 recovered fields)
/// Inherits: ccGobjEnemy, ccGobjBattle, ccGameObj
#[derive(Debug, Clone, Default)]
pub struct ccGobjEnemyEggx2 {
    pub vtable_rva: u32,
    pub field_0x0000: u8,
    pub field_0x0001: u32,
    pub field_0x0004: u8,
    pub field_0x0008: u16,
    pub field_0x000a: u16,
    pub field_0x0010: u64,
    pub field_0x0018: u64,
    pub field_0x001c: u8,
    pub field_0x001e: u16,
    pub field_0x0020: u32,
    pub field_0x0024: u16,
    pub field_0x002c: u16,
    pub field_0x0030: u32,
    pub field_0x0034: f32,
    pub field_0x0038: f32,
    pub field_0x003c: f32,
    pub field_0x0040: u32,
    pub field_0x007e: u16,
    pub field_0x0090: u32,
    pub field_0x0094: u32,
    pub field_0x0098: u64,
    pub field_0x009c: u32,
    pub field_0x00a0: u32,
    pub field_0x00a4: u32,
    pub field_0x00a8: u64,
    pub field_0x00ac: u32,
    pub field_0x00b0: u64,
    pub field_0x00b8: u64,
    pub field_0x00c4: u32,
    pub field_0x00e0: f32,
    pub field_0x00e4: f32,
    pub field_0x010c: u8,
    pub field_0x0120: u32,
    pub field_0x0124: u32,
    pub field_0x0128: u32,
    pub field_0x012c: u32,
    pub field_0x0130: u32,
    pub field_0x0134: u32,
    pub field_0x0138: u32,
    pub field_0x013c: u32,
    pub field_0x0140: u32,
    pub field_0x0144: u32,
    pub field_0x0148: u32,
    pub field_0x014c: u32,
    pub field_0x0150: u32,
    pub field_0x0154: u32,
    pub field_0x0158: u32,
    pub field_0x015c: u32,
    pub field_0x0170: u32,
    pub field_0x0198: u32,
    pub field_0x019c: u16,
    pub field_0x019e: u16,
    pub field_0x01b0: u64,
    pub field_0x01e4: u16,
    pub field_0x020c: u64,
    pub field_0x0214: u64,
    pub field_0x021c: u64,
    pub field_0x0224: u64,
    pub field_0x022c: u64,
    pub field_0x0234: u64,
    pub field_0x023c: u64,
    pub field_0x0244: u64,
    pub field_0x024c: u64,
    pub field_0x0254: u32,
    pub field_0x0278: u32,
    pub field_0x02da: u16,
    pub field_0x02fc: u8,
    pub field_0x0314: u16,
    pub field_0x03e0: u32,
    pub field_0x2cb8: u64,
}

impl ccGobjEnemyEggx2 {
    pub const CLASS_NAME: &'static str = "ccGobjEnemyEggx2";
    pub const VTABLE_RVA: u32 = 0x008bd900;
    pub const METHOD_COUNT: usize = 24;
    pub const BASE_CLASSES: &'static [&'static str] = &["ccGobjEnemy", "ccGobjBattle", "ccGameObj"];
    pub const VIRTUAL_METHODS: &'static [u64] = &[0x00000001804b2860, 0x00000001804b3120, 0x000000018045e4c0, 0x00000001804b6a60, 0x000000018047ebd0, 0x000000018047f370, 0x00000001804b4ec0, 0x00000001800d3090, 0x00000001800d3090, 0x00000001804bbd00, 0x000000018047f200, 0x000000018047f0e0, 0x000000018045eec0, 0x000000018045efd0, 0x00000001804b2f50, 0x00000001804b66c0, 0x00000001804b6040, 0x00000001804b80d0, 0x00000001800d3090, 0x00000001800d3090, 0x00000001800d3090, 0x0000000180103e70, 0x00000001804b2920, 0x00000001800d3090];

    pub fn new() -> Self {
        Self {
            vtable_rva: Self::VTABLE_RVA,
            field_0x0000: Default::default(),
            field_0x0001: Default::default(),
            field_0x0004: Default::default(),
            field_0x0008: Default::default(),
            field_0x000a: Default::default(),
            field_0x0010: Default::default(),
            field_0x0018: Default::default(),
            field_0x001c: Default::default(),
            field_0x001e: Default::default(),
            field_0x0020: Default::default(),
            field_0x0024: Default::default(),
            field_0x002c: Default::default(),
            field_0x0030: Default::default(),
            field_0x0034: Default::default(),
            field_0x0038: Default::default(),
            field_0x003c: Default::default(),
            field_0x0040: Default::default(),
            field_0x007e: Default::default(),
            field_0x0090: Default::default(),
            field_0x0094: Default::default(),
            field_0x0098: Default::default(),
            field_0x009c: Default::default(),
            field_0x00a0: Default::default(),
            field_0x00a4: Default::default(),
            field_0x00a8: Default::default(),
            field_0x00ac: Default::default(),
            field_0x00b0: Default::default(),
            field_0x00b8: Default::default(),
            field_0x00c4: Default::default(),
            field_0x00e0: Default::default(),
            field_0x00e4: Default::default(),
            field_0x010c: Default::default(),
            field_0x0120: Default::default(),
            field_0x0124: Default::default(),
            field_0x0128: Default::default(),
            field_0x012c: Default::default(),
            field_0x0130: Default::default(),
            field_0x0134: Default::default(),
            field_0x0138: Default::default(),
            field_0x013c: Default::default(),
            field_0x0140: Default::default(),
            field_0x0144: Default::default(),
            field_0x0148: Default::default(),
            field_0x014c: Default::default(),
            field_0x0150: Default::default(),
            field_0x0154: Default::default(),
            field_0x0158: Default::default(),
            field_0x015c: Default::default(),
            field_0x0170: Default::default(),
            field_0x0198: Default::default(),
            field_0x019c: Default::default(),
            field_0x019e: Default::default(),
            field_0x01b0: Default::default(),
            field_0x01e4: Default::default(),
            field_0x020c: Default::default(),
            field_0x0214: Default::default(),
            field_0x021c: Default::default(),
            field_0x0224: Default::default(),
            field_0x022c: Default::default(),
            field_0x0234: Default::default(),
            field_0x023c: Default::default(),
            field_0x0244: Default::default(),
            field_0x024c: Default::default(),
            field_0x0254: Default::default(),
            field_0x0278: Default::default(),
            field_0x02da: Default::default(),
            field_0x02fc: Default::default(),
            field_0x0314: Default::default(),
            field_0x03e0: Default::default(),
            field_0x2cb8: Default::default(),
        }
    }

    /// Decompiled virtual method #0 @ RVA 0x004b2860 (16 instructions, 3 basic blocks)
    pub fn method_00(&mut self) {
    }

    /// Decompiled virtual method #1 @ RVA 0x004b3120 (112 instructions, 15 basic blocks)
    pub fn method_01(&mut self) {
        // Writes to field at offset +0x0030
        // Writes to field at offset +0x00c4
    }

    /// Decompiled virtual method #2 @ RVA 0x0045e4c0 (42 instructions, 4 basic blocks)
    pub fn method_02(&mut self) {
        // Writes to field at offset +0x0090
        // Writes to field at offset +0x0094
        // Writes to field at offset +0x0098
        // Writes to field at offset +0x009c
        // Writes to field at offset +0x00a0
        // Writes to field at offset +0x00a4
        // Writes to field at offset +0x00a8
        // Writes to field at offset +0x00ac
        // Writes to field at offset +0x0140
        // Writes to field at offset +0x0144
        // Writes to field at offset +0x0148
        // Writes to field at offset +0x014c
        // Writes to field at offset +0x0150
        // Writes to field at offset +0x0154
        // Writes to field at offset +0x0158
        // Writes to field at offset +0x015c
    }

    /// Decompiled virtual method #3 @ RVA 0x004b6a60 (104 instructions, 18 basic blocks)
    pub fn method_03(&mut self) {
        // Writes to field at offset +0x0000
        // Writes to field at offset +0x0001
        // Writes to field at offset +0x0004
        // Writes to field at offset +0x00b0
    }

    /// Decompiled virtual method #4 @ RVA 0x0047ebd0 (45 instructions, 12 basic blocks)
    pub fn method_04(&mut self) {
        // Writes to field at offset +0x0004
        // Writes to field at offset +0x02da
        // Writes to field at offset +0x0314
        // Writes to field at offset +0x03e0
    }

    /// Decompiled virtual method #5 @ RVA 0x0047f370 (124 instructions, 30 basic blocks)
    pub fn method_05(&mut self) {
        // Writes to field at offset +0x0040
    }

    /// Decompiled virtual method #6 @ RVA 0x004b4ec0 (103 instructions, 17 basic blocks)
    pub fn method_06(&mut self) {
    }

    /// Decompiled virtual method #7 @ RVA 0x000d3090 (1 instructions, 1 basic blocks)
    pub fn method_07(&mut self) {
    }

    /// Decompiled virtual method #8 @ RVA 0x000d3090 (1 instructions, 1 basic blocks)
    pub fn method_08(&mut self) {
    }

    /// Decompiled virtual method #9 @ RVA 0x004bbd00 (105 instructions, 29 basic blocks)
    pub fn method_09(&mut self) {
        // Writes to field at offset +0x0000
        // Writes to field at offset +0x010c
    }

    /// Decompiled virtual method #10 @ RVA 0x0047f200 (58 instructions, 4 basic blocks)
    pub fn method_10(&mut self) {
    }

    /// Decompiled virtual method #11 @ RVA 0x0047f0e0 (15 instructions, 4 basic blocks)
    pub fn method_11(&mut self) {
    }

    /// Decompiled virtual method #12 @ RVA 0x0045eec0 (67 instructions, 22 basic blocks)
    pub fn method_12(&mut self) {
        // Writes to field at offset +0x001c
        // Writes to field at offset +0x0040
        // Writes to field at offset +0x0090
        // Writes to field at offset +0x00b0
    }

    /// Decompiled virtual method #13 @ RVA 0x0045efd0 (27 instructions, 5 basic blocks)
    pub fn method_13(&mut self) {
        // Writes to field at offset +0x0090
        // Writes to field at offset +0x00b0
    }

    /// Decompiled virtual method #14 @ RVA 0x004b2f50 (105 instructions, 2 basic blocks)
    pub fn method_14(&mut self) {
        // Writes to field at offset +0x0000
        // Writes to field at offset +0x0004
        // Writes to field at offset +0x0008
        // Writes to field at offset +0x000a
        // Writes to field at offset +0x00c4
        // Writes to field at offset +0x020c
        // Writes to field at offset +0x0214
        // Writes to field at offset +0x021c
        // Writes to field at offset +0x0224
        // Writes to field at offset +0x022c
        // Writes to field at offset +0x0234
        // Writes to field at offset +0x023c
        // Writes to field at offset +0x0244
        // Writes to field at offset +0x024c
        // Writes to field at offset +0x0254
    }

    /// Decompiled virtual method #15 @ RVA 0x004b66c0 (44 instructions, 12 basic blocks)
    pub fn method_15(&mut self) {
        // Writes to field at offset +0x0000
        // Writes to field at offset +0x0004
        // Writes to field at offset +0x0008
        // Writes to field at offset +0x000a
        // Writes to field at offset +0x00c4
        // Writes to field at offset +0x01e4
        // Writes to field at offset +0x02fc
    }

    /// Decompiled virtual method #16 @ RVA 0x004b6040 (54 instructions, 12 basic blocks)
    pub fn method_16(&mut self) {
    }

    /// Decompiled virtual method #17 @ RVA 0x004b80d0 (38 instructions, 8 basic blocks)
    pub fn method_17(&mut self) {
    }

    /// Decompiled virtual method #18 @ RVA 0x000d3090 (1 instructions, 1 basic blocks)
    pub fn method_18(&mut self) {
    }

    /// Decompiled virtual method #19 @ RVA 0x000d3090 (1 instructions, 1 basic blocks)
    pub fn method_19(&mut self) {
    }

    /// Decompiled virtual method #20 @ RVA 0x000d3090 (1 instructions, 1 basic blocks)
    pub fn method_20(&mut self) {
    }

    /// Decompiled virtual method #21 @ RVA 0x00103e70 (2 instructions, 1 basic blocks)
    pub fn method_21(&mut self) {
    }

    /// Decompiled virtual method #22 @ RVA 0x004b2920 (22 instructions, 3 basic blocks)
    pub fn method_22(&mut self) {
        // Writes to field at offset +0x01b0
    }

    /// Decompiled virtual method #23 @ RVA 0x000d3090 (1 instructions, 1 basic blocks)
    pub fn method_23(&mut self) {
    }

}

impl CcClass for ccGobjEnemyEggx2 {
    fn class_name(&self) -> &'static str { Self::CLASS_NAME }
    fn vtable_rva(&self) -> u32 { Self::VTABLE_RVA }
    fn virtual_method_count(&self) -> usize { Self::METHOD_COUNT }
    fn base_classes(&self) -> &'static [&'static str] { Self::BASE_CLASSES }
}

/// Decompiled CC2 class: `ccGobjEnemyExkt20` (VTable RVA: 0x008bde98, 24 virtual methods, 77 recovered fields)
/// Inherits: ccGobjEnemy, ccGobjBattle, ccGameObj
#[derive(Debug, Clone, Default)]
pub struct ccGobjEnemyExkt20 {
    pub vtable_rva: u32,
    pub field_0x0000: u64,
    pub field_0x0004: u32,
    pub field_0x0008: u16,
    pub field_0x000a: u32,
    pub field_0x0010: u64,
    pub field_0x0018: u64,
    pub field_0x001c: u8,
    pub field_0x0020: u32,
    pub field_0x0024: u16,
    pub field_0x002c: u16,
    pub field_0x0040: u32,
    pub field_0x0050: u64,
    pub field_0x0058: u32,
    pub field_0x005c: u32,
    pub field_0x0060: u32,
    pub field_0x0064: u32,
    pub field_0x0068: u32,
    pub field_0x006c: u32,
    pub field_0x0070: u32,
    pub field_0x0074: u32,
    pub field_0x0078: u32,
    pub field_0x007c: u32,
    pub field_0x007e: u16,
    pub field_0x0080: u32,
    pub field_0x0084: u32,
    pub field_0x0088: u32,
    pub field_0x008c: u32,
    pub field_0x0090: u32,
    pub field_0x0094: u32,
    pub field_0x0098: u32,
    pub field_0x009c: u32,
    pub field_0x00a0: u32,
    pub field_0x00a1: u8,
    pub field_0x00a4: u32,
    pub field_0x00a8: u32,
    pub field_0x00ac: u32,
    pub field_0x00b0: u64,
    pub field_0x00b8: u64,
    pub field_0x00c4: u32,
    pub field_0x0120: u32,
    pub field_0x0124: u32,
    pub field_0x0128: u32,
    pub field_0x012c: u32,
    pub field_0x0130: u32,
    pub field_0x0134: u32,
    pub field_0x0138: u32,
    pub field_0x013c: u32,
    pub field_0x0140: u32,
    pub field_0x0144: u32,
    pub field_0x0148: u32,
    pub field_0x014c: u32,
    pub field_0x0150: u32,
    pub field_0x0154: u32,
    pub field_0x0158: u32,
    pub field_0x015c: u32,
    pub field_0x0170: u32,
    pub field_0x0198: u32,
    pub field_0x019c: u16,
    pub field_0x019e: u16,
    pub field_0x01b0: u64,
    pub field_0x01e4: u16,
    pub field_0x020c: u64,
    pub field_0x0214: u64,
    pub field_0x021c: u64,
    pub field_0x0224: u64,
    pub field_0x022c: u64,
    pub field_0x0234: u64,
    pub field_0x023c: u64,
    pub field_0x0244: u64,
    pub field_0x024c: u64,
    pub field_0x0254: u32,
    pub field_0x0278: u32,
    pub field_0x02a0: u64,
    pub field_0x02da: u16,
    pub field_0x02fc: u8,
    pub field_0x0314: u16,
    pub field_0x03e0: u32,
}

impl ccGobjEnemyExkt20 {
    pub const CLASS_NAME: &'static str = "ccGobjEnemyExkt20";
    pub const VTABLE_RVA: u32 = 0x008bde98;
    pub const METHOD_COUNT: usize = 24;
    pub const BASE_CLASSES: &'static [&'static str] = &["ccGobjEnemy", "ccGobjBattle", "ccGameObj"];
    pub const VIRTUAL_METHODS: &'static [u64] = &[0x00000001804bbe70, 0x00000001804bbfc0, 0x000000018045e4c0, 0x00000001804bea40, 0x000000018047ebd0, 0x000000018047f370, 0x00000001804b4ec0, 0x00000001800d3090, 0x00000001800d3090, 0x00000001804bc2d0, 0x00000001804bc140, 0x000000018047f0e0, 0x000000018045eec0, 0x000000018045efd0, 0x00000001804b2f50, 0x00000001804b66c0, 0x00000001804b6040, 0x00000001804b80d0, 0x00000001800d3090, 0x00000001800d3090, 0x00000001800d3090, 0x00000001804bebe0, 0x00000001804bbf40, 0x00000001804b8820];

    pub fn new() -> Self {
        Self {
            vtable_rva: Self::VTABLE_RVA,
            field_0x0000: Default::default(),
            field_0x0004: Default::default(),
            field_0x0008: Default::default(),
            field_0x000a: Default::default(),
            field_0x0010: Default::default(),
            field_0x0018: Default::default(),
            field_0x001c: Default::default(),
            field_0x0020: Default::default(),
            field_0x0024: Default::default(),
            field_0x002c: Default::default(),
            field_0x0040: Default::default(),
            field_0x0050: Default::default(),
            field_0x0058: Default::default(),
            field_0x005c: Default::default(),
            field_0x0060: Default::default(),
            field_0x0064: Default::default(),
            field_0x0068: Default::default(),
            field_0x006c: Default::default(),
            field_0x0070: Default::default(),
            field_0x0074: Default::default(),
            field_0x0078: Default::default(),
            field_0x007c: Default::default(),
            field_0x007e: Default::default(),
            field_0x0080: Default::default(),
            field_0x0084: Default::default(),
            field_0x0088: Default::default(),
            field_0x008c: Default::default(),
            field_0x0090: Default::default(),
            field_0x0094: Default::default(),
            field_0x0098: Default::default(),
            field_0x009c: Default::default(),
            field_0x00a0: Default::default(),
            field_0x00a1: Default::default(),
            field_0x00a4: Default::default(),
            field_0x00a8: Default::default(),
            field_0x00ac: Default::default(),
            field_0x00b0: Default::default(),
            field_0x00b8: Default::default(),
            field_0x00c4: Default::default(),
            field_0x0120: Default::default(),
            field_0x0124: Default::default(),
            field_0x0128: Default::default(),
            field_0x012c: Default::default(),
            field_0x0130: Default::default(),
            field_0x0134: Default::default(),
            field_0x0138: Default::default(),
            field_0x013c: Default::default(),
            field_0x0140: Default::default(),
            field_0x0144: Default::default(),
            field_0x0148: Default::default(),
            field_0x014c: Default::default(),
            field_0x0150: Default::default(),
            field_0x0154: Default::default(),
            field_0x0158: Default::default(),
            field_0x015c: Default::default(),
            field_0x0170: Default::default(),
            field_0x0198: Default::default(),
            field_0x019c: Default::default(),
            field_0x019e: Default::default(),
            field_0x01b0: Default::default(),
            field_0x01e4: Default::default(),
            field_0x020c: Default::default(),
            field_0x0214: Default::default(),
            field_0x021c: Default::default(),
            field_0x0224: Default::default(),
            field_0x022c: Default::default(),
            field_0x0234: Default::default(),
            field_0x023c: Default::default(),
            field_0x0244: Default::default(),
            field_0x024c: Default::default(),
            field_0x0254: Default::default(),
            field_0x0278: Default::default(),
            field_0x02a0: Default::default(),
            field_0x02da: Default::default(),
            field_0x02fc: Default::default(),
            field_0x0314: Default::default(),
            field_0x03e0: Default::default(),
        }
    }

    /// Decompiled virtual method #0 @ RVA 0x004bbe70 (16 instructions, 3 basic blocks)
    pub fn method_00(&mut self) {
    }

    /// Decompiled virtual method #1 @ RVA 0x004bbfc0 (131 instructions, 25 basic blocks)
    pub fn method_01(&mut self) {
        // Writes to field at offset +0x00a1
    }

    /// Decompiled virtual method #2 @ RVA 0x0045e4c0 (42 instructions, 4 basic blocks)
    pub fn method_02(&mut self) {
        // Writes to field at offset +0x0090
        // Writes to field at offset +0x0094
        // Writes to field at offset +0x0098
        // Writes to field at offset +0x009c
        // Writes to field at offset +0x00a0
        // Writes to field at offset +0x00a4
        // Writes to field at offset +0x00a8
        // Writes to field at offset +0x00ac
        // Writes to field at offset +0x0140
        // Writes to field at offset +0x0144
        // Writes to field at offset +0x0148
        // Writes to field at offset +0x014c
        // Writes to field at offset +0x0150
        // Writes to field at offset +0x0154
        // Writes to field at offset +0x0158
        // Writes to field at offset +0x015c
    }

    /// Decompiled virtual method #3 @ RVA 0x004bea40 (54 instructions, 10 basic blocks)
    pub fn method_03(&mut self) {
        // Writes to field at offset +0x0000
    }

    /// Decompiled virtual method #4 @ RVA 0x0047ebd0 (45 instructions, 12 basic blocks)
    pub fn method_04(&mut self) {
        // Writes to field at offset +0x0004
        // Writes to field at offset +0x02da
        // Writes to field at offset +0x0314
        // Writes to field at offset +0x03e0
    }

    /// Decompiled virtual method #5 @ RVA 0x0047f370 (124 instructions, 30 basic blocks)
    pub fn method_05(&mut self) {
        // Writes to field at offset +0x0040
    }

    /// Decompiled virtual method #6 @ RVA 0x004b4ec0 (103 instructions, 17 basic blocks)
    pub fn method_06(&mut self) {
    }

    /// Decompiled virtual method #7 @ RVA 0x000d3090 (1 instructions, 1 basic blocks)
    pub fn method_07(&mut self) {
    }

    /// Decompiled virtual method #8 @ RVA 0x000d3090 (1 instructions, 1 basic blocks)
    pub fn method_08(&mut self) {
    }

    /// Decompiled virtual method #9 @ RVA 0x004bc2d0 (118 instructions, 29 basic blocks)
    pub fn method_09(&mut self) {
        // Writes to field at offset +0x0000
        // Writes to field at offset +0x0004
        // Writes to field at offset +0x0008
        // Writes to field at offset +0x000a
    }

    /// Decompiled virtual method #10 @ RVA 0x004bc140 (85 instructions, 9 basic blocks)
    pub fn method_10(&mut self) {
        // Writes to field at offset +0x0050
        // Writes to field at offset +0x0058
        // Writes to field at offset +0x0058
        // Writes to field at offset +0x005c
        // Writes to field at offset +0x0060
        // Writes to field at offset +0x0064
        // Writes to field at offset +0x0068
        // Writes to field at offset +0x0068
        // Writes to field at offset +0x006c
        // Writes to field at offset +0x0070
        // Writes to field at offset +0x0070
        // Writes to field at offset +0x0074
        // Writes to field at offset +0x0078
        // Writes to field at offset +0x007c
        // Writes to field at offset +0x007c
        // Writes to field at offset +0x0080
        // Writes to field at offset +0x0084
        // Writes to field at offset +0x0084
        // Writes to field at offset +0x0088
        // Writes to field at offset +0x008c
        // Writes to field at offset +0x00a1
    }

    /// Decompiled virtual method #11 @ RVA 0x0047f0e0 (15 instructions, 4 basic blocks)
    pub fn method_11(&mut self) {
    }

    /// Decompiled virtual method #12 @ RVA 0x0045eec0 (67 instructions, 22 basic blocks)
    pub fn method_12(&mut self) {
        // Writes to field at offset +0x001c
        // Writes to field at offset +0x0040
        // Writes to field at offset +0x0090
        // Writes to field at offset +0x00b0
    }

    /// Decompiled virtual method #13 @ RVA 0x0045efd0 (27 instructions, 5 basic blocks)
    pub fn method_13(&mut self) {
        // Writes to field at offset +0x0090
        // Writes to field at offset +0x00b0
    }

    /// Decompiled virtual method #14 @ RVA 0x004b2f50 (105 instructions, 2 basic blocks)
    pub fn method_14(&mut self) {
        // Writes to field at offset +0x0000
        // Writes to field at offset +0x0004
        // Writes to field at offset +0x0008
        // Writes to field at offset +0x000a
        // Writes to field at offset +0x00c4
        // Writes to field at offset +0x020c
        // Writes to field at offset +0x0214
        // Writes to field at offset +0x021c
        // Writes to field at offset +0x0224
        // Writes to field at offset +0x022c
        // Writes to field at offset +0x0234
        // Writes to field at offset +0x023c
        // Writes to field at offset +0x0244
        // Writes to field at offset +0x024c
        // Writes to field at offset +0x0254
    }

    /// Decompiled virtual method #15 @ RVA 0x004b66c0 (44 instructions, 12 basic blocks)
    pub fn method_15(&mut self) {
        // Writes to field at offset +0x0000
        // Writes to field at offset +0x0004
        // Writes to field at offset +0x0008
        // Writes to field at offset +0x000a
        // Writes to field at offset +0x00c4
        // Writes to field at offset +0x01e4
        // Writes to field at offset +0x02fc
    }

    /// Decompiled virtual method #16 @ RVA 0x004b6040 (54 instructions, 12 basic blocks)
    pub fn method_16(&mut self) {
    }

    /// Decompiled virtual method #17 @ RVA 0x004b80d0 (38 instructions, 8 basic blocks)
    pub fn method_17(&mut self) {
    }

    /// Decompiled virtual method #18 @ RVA 0x000d3090 (1 instructions, 1 basic blocks)
    pub fn method_18(&mut self) {
    }

    /// Decompiled virtual method #19 @ RVA 0x000d3090 (1 instructions, 1 basic blocks)
    pub fn method_19(&mut self) {
    }

    /// Decompiled virtual method #20 @ RVA 0x000d3090 (1 instructions, 1 basic blocks)
    pub fn method_20(&mut self) {
    }

    /// Decompiled virtual method #21 @ RVA 0x004bebe0 (15 instructions, 4 basic blocks)
    pub fn method_21(&mut self) {
        // Writes to field at offset +0x01e4
        // Writes to field at offset +0x02fc
    }

    /// Decompiled virtual method #22 @ RVA 0x004bbf40 (25 instructions, 3 basic blocks)
    pub fn method_22(&mut self) {
        // Writes to field at offset +0x01b0
    }

    /// Decompiled virtual method #23 @ RVA 0x004b8820 (4 instructions, 1 basic blocks)
    pub fn method_23(&mut self) {
        // Writes to field at offset +0x03e0
    }

}

impl CcClass for ccGobjEnemyExkt20 {
    fn class_name(&self) -> &'static str { Self::CLASS_NAME }
    fn vtable_rva(&self) -> u32 { Self::VTABLE_RVA }
    fn virtual_method_count(&self) -> usize { Self::METHOD_COUNT }
    fn base_classes(&self) -> &'static [&'static str] { Self::BASE_CLASSES }
}

/// Decompiled CC2 class: `ccGobjEnemyExx10` (VTable RVA: 0x008bd770, 24 virtual methods, 74 recovered fields)
/// Inherits: ccGobjEnemy, ccGobjBattle, ccGameObj
#[derive(Debug, Clone, Default)]
pub struct ccGobjEnemyExx10 {
    pub vtable_rva: u32,
    pub field_0x0000: u64,
    pub field_0x0004: u32,
    pub field_0x0008: u16,
    pub field_0x000a: u16,
    pub field_0x0010: u64,
    pub field_0x0018: u64,
    pub field_0x001c: u8,
    pub field_0x001e: u16,
    pub field_0x0020: u32,
    pub field_0x0024: u16,
    pub field_0x0028: u64,
    pub field_0x002c: u16,
    pub field_0x0030: u32,
    pub field_0x0040: u32,
    pub field_0x0050: u64,
    pub field_0x0068: u64,
    pub field_0x007e: u16,
    pub field_0x0090: u32,
    pub field_0x0094: u32,
    pub field_0x0098: u64,
    pub field_0x009c: u32,
    pub field_0x00a0: u32,
    pub field_0x00a4: u32,
    pub field_0x00a8: u64,
    pub field_0x00ac: u32,
    pub field_0x00b0: u64,
    pub field_0x00b8: u64,
    pub field_0x00c4: u32,
    pub field_0x00e0: f32,
    pub field_0x00e4: f32,
    pub field_0x0120: u32,
    pub field_0x0124: u32,
    pub field_0x0128: u32,
    pub field_0x012c: u32,
    pub field_0x0130: u32,
    pub field_0x0134: u32,
    pub field_0x0138: u32,
    pub field_0x013c: u32,
    pub field_0x0140: u32,
    pub field_0x0144: u32,
    pub field_0x0148: u32,
    pub field_0x014c: u32,
    pub field_0x0150: u32,
    pub field_0x0154: u32,
    pub field_0x0158: u32,
    pub field_0x015c: u32,
    pub field_0x0170: u32,
    pub field_0x0198: u32,
    pub field_0x019c: u16,
    pub field_0x019e: u16,
    pub field_0x01b0: u64,
    pub field_0x01e4: u16,
    pub field_0x020c: u64,
    pub field_0x0214: u64,
    pub field_0x021c: u64,
    pub field_0x0224: u64,
    pub field_0x022c: u64,
    pub field_0x0234: u64,
    pub field_0x023c: u64,
    pub field_0x0244: u64,
    pub field_0x024c: u64,
    pub field_0x0254: u32,
    pub field_0x0278: u32,
    pub field_0x02da: u16,
    pub field_0x02fc: u8,
    pub field_0x0314: u16,
    pub field_0x03e0: u32,
    pub field_0x0420: u32,
    pub field_0x0428: u64,
    pub field_0x0430: u64,
    pub field_0x0438: u64,
    pub field_0x0440: u64,
    pub field_0x0450: u64,
    pub field_0x0490: u32,
}

impl ccGobjEnemyExx10 {
    pub const CLASS_NAME: &'static str = "ccGobjEnemyExx10";
    pub const VTABLE_RVA: u32 = 0x008bd770;
    pub const METHOD_COUNT: usize = 24;
    pub const BASE_CLASSES: &'static [&'static str] = &["ccGobjEnemy", "ccGobjBattle", "ccGameObj"];
    pub const VIRTUAL_METHODS: &'static [u64] = &[0x00000001804bbc80, 0x00000001804b3120, 0x000000018045e4c0, 0x00000001804bfdf0, 0x000000018047ebd0, 0x00000001804bfd10, 0x00000001804b4ec0, 0x00000001800d3090, 0x00000001800d3090, 0x00000001804c0000, 0x00000001804c3450, 0x000000018047f0e0, 0x000000018045eec0, 0x000000018045efd0, 0x00000001804b2f50, 0x00000001804b66c0, 0x00000001804b6040, 0x00000001804b80d0, 0x00000001800d3090, 0x00000001800d3090, 0x00000001800d3090, 0x0000000180103e70, 0x00000001804bfc10, 0x00000001804bfcd0];

    pub fn new() -> Self {
        Self {
            vtable_rva: Self::VTABLE_RVA,
            field_0x0000: Default::default(),
            field_0x0004: Default::default(),
            field_0x0008: Default::default(),
            field_0x000a: Default::default(),
            field_0x0010: Default::default(),
            field_0x0018: Default::default(),
            field_0x001c: Default::default(),
            field_0x001e: Default::default(),
            field_0x0020: Default::default(),
            field_0x0024: Default::default(),
            field_0x0028: Default::default(),
            field_0x002c: Default::default(),
            field_0x0030: Default::default(),
            field_0x0040: Default::default(),
            field_0x0050: Default::default(),
            field_0x0068: Default::default(),
            field_0x007e: Default::default(),
            field_0x0090: Default::default(),
            field_0x0094: Default::default(),
            field_0x0098: Default::default(),
            field_0x009c: Default::default(),
            field_0x00a0: Default::default(),
            field_0x00a4: Default::default(),
            field_0x00a8: Default::default(),
            field_0x00ac: Default::default(),
            field_0x00b0: Default::default(),
            field_0x00b8: Default::default(),
            field_0x00c4: Default::default(),
            field_0x00e0: Default::default(),
            field_0x00e4: Default::default(),
            field_0x0120: Default::default(),
            field_0x0124: Default::default(),
            field_0x0128: Default::default(),
            field_0x012c: Default::default(),
            field_0x0130: Default::default(),
            field_0x0134: Default::default(),
            field_0x0138: Default::default(),
            field_0x013c: Default::default(),
            field_0x0140: Default::default(),
            field_0x0144: Default::default(),
            field_0x0148: Default::default(),
            field_0x014c: Default::default(),
            field_0x0150: Default::default(),
            field_0x0154: Default::default(),
            field_0x0158: Default::default(),
            field_0x015c: Default::default(),
            field_0x0170: Default::default(),
            field_0x0198: Default::default(),
            field_0x019c: Default::default(),
            field_0x019e: Default::default(),
            field_0x01b0: Default::default(),
            field_0x01e4: Default::default(),
            field_0x020c: Default::default(),
            field_0x0214: Default::default(),
            field_0x021c: Default::default(),
            field_0x0224: Default::default(),
            field_0x022c: Default::default(),
            field_0x0234: Default::default(),
            field_0x023c: Default::default(),
            field_0x0244: Default::default(),
            field_0x024c: Default::default(),
            field_0x0254: Default::default(),
            field_0x0278: Default::default(),
            field_0x02da: Default::default(),
            field_0x02fc: Default::default(),
            field_0x0314: Default::default(),
            field_0x03e0: Default::default(),
            field_0x0420: Default::default(),
            field_0x0428: Default::default(),
            field_0x0430: Default::default(),
            field_0x0438: Default::default(),
            field_0x0440: Default::default(),
            field_0x0450: Default::default(),
            field_0x0490: Default::default(),
        }
    }

    /// Decompiled virtual method #0 @ RVA 0x004bbc80 (16 instructions, 3 basic blocks)
    pub fn method_00(&mut self) {
    }

    /// Decompiled virtual method #1 @ RVA 0x004b3120 (112 instructions, 15 basic blocks)
    pub fn method_01(&mut self) {
        // Writes to field at offset +0x0030
        // Writes to field at offset +0x00c4
    }

    /// Decompiled virtual method #2 @ RVA 0x0045e4c0 (42 instructions, 4 basic blocks)
    pub fn method_02(&mut self) {
        // Writes to field at offset +0x0090
        // Writes to field at offset +0x0094
        // Writes to field at offset +0x0098
        // Writes to field at offset +0x009c
        // Writes to field at offset +0x00a0
        // Writes to field at offset +0x00a4
        // Writes to field at offset +0x00a8
        // Writes to field at offset +0x00ac
        // Writes to field at offset +0x0140
        // Writes to field at offset +0x0144
        // Writes to field at offset +0x0148
        // Writes to field at offset +0x014c
        // Writes to field at offset +0x0150
        // Writes to field at offset +0x0154
        // Writes to field at offset +0x0158
        // Writes to field at offset +0x015c
    }

    /// Decompiled virtual method #3 @ RVA 0x004bfdf0 (105 instructions, 23 basic blocks)
    pub fn method_03(&mut self) {
        // Writes to field at offset +0x0000
    }

    /// Decompiled virtual method #4 @ RVA 0x0047ebd0 (45 instructions, 12 basic blocks)
    pub fn method_04(&mut self) {
        // Writes to field at offset +0x0004
        // Writes to field at offset +0x02da
        // Writes to field at offset +0x0314
        // Writes to field at offset +0x03e0
    }

    /// Decompiled virtual method #5 @ RVA 0x004bfd10 (134 instructions, 39 basic blocks)
    pub fn method_05(&mut self) {
        // Writes to field at offset +0x0000
        // Writes to field at offset +0x0024
        // Writes to field at offset +0x0314
    }

    /// Decompiled virtual method #6 @ RVA 0x004b4ec0 (103 instructions, 17 basic blocks)
    pub fn method_06(&mut self) {
    }

    /// Decompiled virtual method #7 @ RVA 0x000d3090 (1 instructions, 1 basic blocks)
    pub fn method_07(&mut self) {
    }

    /// Decompiled virtual method #8 @ RVA 0x000d3090 (1 instructions, 1 basic blocks)
    pub fn method_08(&mut self) {
    }

    /// Decompiled virtual method #9 @ RVA 0x004c0000 (105 instructions, 16 basic blocks)
    pub fn method_09(&mut self) {
    }

    /// Decompiled virtual method #10 @ RVA 0x004c3450 (64 instructions, 8 basic blocks)
    pub fn method_10(&mut self) {
    }

    /// Decompiled virtual method #11 @ RVA 0x0047f0e0 (15 instructions, 4 basic blocks)
    pub fn method_11(&mut self) {
    }

    /// Decompiled virtual method #12 @ RVA 0x0045eec0 (67 instructions, 22 basic blocks)
    pub fn method_12(&mut self) {
        // Writes to field at offset +0x001c
        // Writes to field at offset +0x0040
        // Writes to field at offset +0x0090
        // Writes to field at offset +0x00b0
    }

    /// Decompiled virtual method #13 @ RVA 0x0045efd0 (27 instructions, 5 basic blocks)
    pub fn method_13(&mut self) {
        // Writes to field at offset +0x0090
        // Writes to field at offset +0x00b0
    }

    /// Decompiled virtual method #14 @ RVA 0x004b2f50 (105 instructions, 2 basic blocks)
    pub fn method_14(&mut self) {
        // Writes to field at offset +0x0000
        // Writes to field at offset +0x0004
        // Writes to field at offset +0x0008
        // Writes to field at offset +0x000a
        // Writes to field at offset +0x00c4
        // Writes to field at offset +0x020c
        // Writes to field at offset +0x0214
        // Writes to field at offset +0x021c
        // Writes to field at offset +0x0224
        // Writes to field at offset +0x022c
        // Writes to field at offset +0x0234
        // Writes to field at offset +0x023c
        // Writes to field at offset +0x0244
        // Writes to field at offset +0x024c
        // Writes to field at offset +0x0254
    }

    /// Decompiled virtual method #15 @ RVA 0x004b66c0 (44 instructions, 12 basic blocks)
    pub fn method_15(&mut self) {
        // Writes to field at offset +0x0000
        // Writes to field at offset +0x0004
        // Writes to field at offset +0x0008
        // Writes to field at offset +0x000a
        // Writes to field at offset +0x00c4
        // Writes to field at offset +0x01e4
        // Writes to field at offset +0x02fc
    }

    /// Decompiled virtual method #16 @ RVA 0x004b6040 (54 instructions, 12 basic blocks)
    pub fn method_16(&mut self) {
    }

    /// Decompiled virtual method #17 @ RVA 0x004b80d0 (38 instructions, 8 basic blocks)
    pub fn method_17(&mut self) {
    }

    /// Decompiled virtual method #18 @ RVA 0x000d3090 (1 instructions, 1 basic blocks)
    pub fn method_18(&mut self) {
    }

    /// Decompiled virtual method #19 @ RVA 0x000d3090 (1 instructions, 1 basic blocks)
    pub fn method_19(&mut self) {
    }

    /// Decompiled virtual method #20 @ RVA 0x000d3090 (1 instructions, 1 basic blocks)
    pub fn method_20(&mut self) {
    }

    /// Decompiled virtual method #21 @ RVA 0x00103e70 (2 instructions, 1 basic blocks)
    pub fn method_21(&mut self) {
    }

    /// Decompiled virtual method #22 @ RVA 0x004bfc10 (32 instructions, 3 basic blocks)
    pub fn method_22(&mut self) {
        // Writes to field at offset +0x01b0
    }

    /// Decompiled virtual method #23 @ RVA 0x004bfcd0 (9 instructions, 1 basic blocks)
    pub fn method_23(&mut self) {
        // Writes to field at offset +0x0420
        // Writes to field at offset +0x0428
        // Writes to field at offset +0x0430
        // Writes to field at offset +0x0438
        // Writes to field at offset +0x0440
        // Writes to field at offset +0x0450
        // Writes to field at offset +0x0490
    }

}

impl CcClass for ccGobjEnemyExx10 {
    fn class_name(&self) -> &'static str { Self::CLASS_NAME }
    fn vtable_rva(&self) -> u32 { Self::VTABLE_RVA }
    fn virtual_method_count(&self) -> usize { Self::METHOD_COUNT }
    fn base_classes(&self) -> &'static [&'static str] { Self::BASE_CLASSES }
}

/// Decompiled CC2 class: `ccGobjEnemyFlyL` (VTable RVA: 0x008bd5e0, 24 virtual methods, 61 recovered fields)
/// Inherits: ccGobjEnemyFly, ccGobjEnemy, ccGobjBattle, ccGameObj
#[derive(Debug, Clone, Default)]
pub struct ccGobjEnemyFlyL {
    pub vtable_rva: u32,
    pub field_0x0000: u8,
    pub field_0x0001: u32,
    pub field_0x0004: u8,
    pub field_0x0008: u16,
    pub field_0x000a: u16,
    pub field_0x000c: u32,
    pub field_0x0010: [f32; 4],
    pub field_0x0018: u64,
    pub field_0x001c: u8,
    pub field_0x001e: u16,
    pub field_0x0020: u32,
    pub field_0x0024: u16,
    pub field_0x002c: u16,
    pub field_0x0030: u32,
    pub field_0x0034: f32,
    pub field_0x0038: f32,
    pub field_0x003c: f32,
    pub field_0x0040: u32,
    pub field_0x007e: u16,
    pub field_0x0090: u32,
    pub field_0x0094: u32,
    pub field_0x0098: u64,
    pub field_0x009c: u32,
    pub field_0x00a0: u32,
    pub field_0x00a4: u32,
    pub field_0x00a8: u64,
    pub field_0x00ac: u32,
    pub field_0x00b0: u64,
    pub field_0x00b8: u64,
    pub field_0x00c4: u32,
    pub field_0x00e0: f32,
    pub field_0x00e4: f32,
    pub field_0x0120: u32,
    pub field_0x0124: u32,
    pub field_0x0128: u32,
    pub field_0x012c: u32,
    pub field_0x0130: u32,
    pub field_0x0134: u32,
    pub field_0x0138: u32,
    pub field_0x013c: u32,
    pub field_0x0140: u32,
    pub field_0x0144: u32,
    pub field_0x0148: u32,
    pub field_0x014c: u32,
    pub field_0x0150: u32,
    pub field_0x0154: u32,
    pub field_0x0158: u32,
    pub field_0x015c: u32,
    pub field_0x0170: u32,
    pub field_0x0198: u32,
    pub field_0x019c: u16,
    pub field_0x019e: u16,
    pub field_0x01b0: u64,
    pub field_0x01e4: u16,
    pub field_0x0278: u32,
    pub field_0x02da: u16,
    pub field_0x02fc: u8,
    pub field_0x0314: u16,
    pub field_0x03e0: u32,
    pub field_0x0420: u64,
    pub field_0x0428: u32,
}

impl ccGobjEnemyFlyL {
    pub const CLASS_NAME: &'static str = "ccGobjEnemyFlyL";
    pub const VTABLE_RVA: u32 = 0x008bd5e0;
    pub const METHOD_COUNT: usize = 24;
    pub const BASE_CLASSES: &'static [&'static str] = &["ccGobjEnemyFly", "ccGobjEnemy", "ccGobjBattle", "ccGameObj"];
    pub const VIRTUAL_METHODS: &'static [u64] = &[0x00000001804bbcc0, 0x00000001804b3120, 0x000000018045e4c0, 0x00000001804b6a60, 0x000000018047ebd0, 0x00000001804b9c40, 0x00000001804b4ec0, 0x00000001800d3090, 0x00000001800d3090, 0x00000001804ba020, 0x000000018047f200, 0x000000018047f0e0, 0x000000018045eec0, 0x000000018045efd0, 0x00000001804b9c00, 0x00000001804b66c0, 0x00000001804b6040, 0x00000001804b80d0, 0x00000001800d3090, 0x00000001800d3090, 0x00000001800d3090, 0x0000000180103e70, 0x00000001804b2920, 0x00000001804b9c20];

    pub fn new() -> Self {
        Self {
            vtable_rva: Self::VTABLE_RVA,
            field_0x0000: Default::default(),
            field_0x0001: Default::default(),
            field_0x0004: Default::default(),
            field_0x0008: Default::default(),
            field_0x000a: Default::default(),
            field_0x000c: Default::default(),
            field_0x0010: Default::default(),
            field_0x0018: Default::default(),
            field_0x001c: Default::default(),
            field_0x001e: Default::default(),
            field_0x0020: Default::default(),
            field_0x0024: Default::default(),
            field_0x002c: Default::default(),
            field_0x0030: Default::default(),
            field_0x0034: Default::default(),
            field_0x0038: Default::default(),
            field_0x003c: Default::default(),
            field_0x0040: Default::default(),
            field_0x007e: Default::default(),
            field_0x0090: Default::default(),
            field_0x0094: Default::default(),
            field_0x0098: Default::default(),
            field_0x009c: Default::default(),
            field_0x00a0: Default::default(),
            field_0x00a4: Default::default(),
            field_0x00a8: Default::default(),
            field_0x00ac: Default::default(),
            field_0x00b0: Default::default(),
            field_0x00b8: Default::default(),
            field_0x00c4: Default::default(),
            field_0x00e0: Default::default(),
            field_0x00e4: Default::default(),
            field_0x0120: Default::default(),
            field_0x0124: Default::default(),
            field_0x0128: Default::default(),
            field_0x012c: Default::default(),
            field_0x0130: Default::default(),
            field_0x0134: Default::default(),
            field_0x0138: Default::default(),
            field_0x013c: Default::default(),
            field_0x0140: Default::default(),
            field_0x0144: Default::default(),
            field_0x0148: Default::default(),
            field_0x014c: Default::default(),
            field_0x0150: Default::default(),
            field_0x0154: Default::default(),
            field_0x0158: Default::default(),
            field_0x015c: Default::default(),
            field_0x0170: Default::default(),
            field_0x0198: Default::default(),
            field_0x019c: Default::default(),
            field_0x019e: Default::default(),
            field_0x01b0: Default::default(),
            field_0x01e4: Default::default(),
            field_0x0278: Default::default(),
            field_0x02da: Default::default(),
            field_0x02fc: Default::default(),
            field_0x0314: Default::default(),
            field_0x03e0: Default::default(),
            field_0x0420: Default::default(),
            field_0x0428: Default::default(),
        }
    }

    /// Decompiled virtual method #0 @ RVA 0x004bbcc0 (16 instructions, 3 basic blocks)
    pub fn method_00(&mut self) {
    }

    /// Decompiled virtual method #1 @ RVA 0x004b3120 (112 instructions, 15 basic blocks)
    pub fn method_01(&mut self) {
        // Writes to field at offset +0x0030
        // Writes to field at offset +0x00c4
    }

    /// Decompiled virtual method #2 @ RVA 0x0045e4c0 (42 instructions, 4 basic blocks)
    pub fn method_02(&mut self) {
        // Writes to field at offset +0x0090
        // Writes to field at offset +0x0094
        // Writes to field at offset +0x0098
        // Writes to field at offset +0x009c
        // Writes to field at offset +0x00a0
        // Writes to field at offset +0x00a4
        // Writes to field at offset +0x00a8
        // Writes to field at offset +0x00ac
        // Writes to field at offset +0x0140
        // Writes to field at offset +0x0144
        // Writes to field at offset +0x0148
        // Writes to field at offset +0x014c
        // Writes to field at offset +0x0150
        // Writes to field at offset +0x0154
        // Writes to field at offset +0x0158
        // Writes to field at offset +0x015c
    }

    /// Decompiled virtual method #3 @ RVA 0x004b6a60 (104 instructions, 18 basic blocks)
    pub fn method_03(&mut self) {
        // Writes to field at offset +0x0000
        // Writes to field at offset +0x0001
        // Writes to field at offset +0x0004
        // Writes to field at offset +0x00b0
    }

    /// Decompiled virtual method #4 @ RVA 0x0047ebd0 (45 instructions, 12 basic blocks)
    pub fn method_04(&mut self) {
        // Writes to field at offset +0x0004
        // Writes to field at offset +0x02da
        // Writes to field at offset +0x0314
        // Writes to field at offset +0x03e0
    }

    /// Decompiled virtual method #5 @ RVA 0x004b9c40 (124 instructions, 20 basic blocks)
    pub fn method_05(&mut self) {
    }

    /// Decompiled virtual method #6 @ RVA 0x004b4ec0 (103 instructions, 17 basic blocks)
    pub fn method_06(&mut self) {
    }

    /// Decompiled virtual method #7 @ RVA 0x000d3090 (1 instructions, 1 basic blocks)
    pub fn method_07(&mut self) {
    }

    /// Decompiled virtual method #8 @ RVA 0x000d3090 (1 instructions, 1 basic blocks)
    pub fn method_08(&mut self) {
    }

    /// Decompiled virtual method #9 @ RVA 0x004ba020 (107 instructions, 25 basic blocks)
    pub fn method_09(&mut self) {
        // Writes to field at offset +0x0000
        // Writes to field at offset +0x0004
        // Writes to field at offset +0x0008
        // Writes to field at offset +0x000a
        // Writes to field at offset +0x000c
        // Writes to field at offset +0x01e4
        // Writes to field at offset +0x0428
    }

    /// Decompiled virtual method #10 @ RVA 0x0047f200 (58 instructions, 4 basic blocks)
    pub fn method_10(&mut self) {
    }

    /// Decompiled virtual method #11 @ RVA 0x0047f0e0 (15 instructions, 4 basic blocks)
    pub fn method_11(&mut self) {
    }

    /// Decompiled virtual method #12 @ RVA 0x0045eec0 (67 instructions, 22 basic blocks)
    pub fn method_12(&mut self) {
        // Writes to field at offset +0x001c
        // Writes to field at offset +0x0040
        // Writes to field at offset +0x0090
        // Writes to field at offset +0x00b0
    }

    /// Decompiled virtual method #13 @ RVA 0x0045efd0 (27 instructions, 5 basic blocks)
    pub fn method_13(&mut self) {
        // Writes to field at offset +0x0090
        // Writes to field at offset +0x00b0
    }

    /// Decompiled virtual method #14 @ RVA 0x004b9c00 (15 instructions, 2 basic blocks)
    pub fn method_14(&mut self) {
        // Writes to field at offset +0x03e0
        // Writes to field at offset +0x0420
        // Writes to field at offset +0x0428
    }

    /// Decompiled virtual method #15 @ RVA 0x004b66c0 (44 instructions, 12 basic blocks)
    pub fn method_15(&mut self) {
        // Writes to field at offset +0x0000
        // Writes to field at offset +0x0004
        // Writes to field at offset +0x0008
        // Writes to field at offset +0x000a
        // Writes to field at offset +0x00c4
        // Writes to field at offset +0x01e4
        // Writes to field at offset +0x02fc
    }

    /// Decompiled virtual method #16 @ RVA 0x004b6040 (54 instructions, 12 basic blocks)
    pub fn method_16(&mut self) {
    }

    /// Decompiled virtual method #17 @ RVA 0x004b80d0 (38 instructions, 8 basic blocks)
    pub fn method_17(&mut self) {
    }

    /// Decompiled virtual method #18 @ RVA 0x000d3090 (1 instructions, 1 basic blocks)
    pub fn method_18(&mut self) {
    }

    /// Decompiled virtual method #19 @ RVA 0x000d3090 (1 instructions, 1 basic blocks)
    pub fn method_19(&mut self) {
    }

    /// Decompiled virtual method #20 @ RVA 0x000d3090 (1 instructions, 1 basic blocks)
    pub fn method_20(&mut self) {
    }

    /// Decompiled virtual method #21 @ RVA 0x00103e70 (2 instructions, 1 basic blocks)
    pub fn method_21(&mut self) {
    }

    /// Decompiled virtual method #22 @ RVA 0x004b2920 (22 instructions, 3 basic blocks)
    pub fn method_22(&mut self) {
        // Writes to field at offset +0x01b0
    }

    /// Decompiled virtual method #23 @ RVA 0x004b9c20 (5 instructions, 1 basic blocks)
    pub fn method_23(&mut self) {
        // Writes to field at offset +0x03e0
        // Writes to field at offset +0x0420
        // Writes to field at offset +0x0428
    }

}

impl CcClass for ccGobjEnemyFlyL {
    fn class_name(&self) -> &'static str { Self::CLASS_NAME }
    fn vtable_rva(&self) -> u32 { Self::VTABLE_RVA }
    fn virtual_method_count(&self) -> usize { Self::METHOD_COUNT }
    fn base_classes(&self) -> &'static [&'static str] { Self::BASE_CLASSES }
}

/// Decompiled CC2 class: `ccGobjEnemyFlyLL` (VTable RVA: 0x008bd6a8, 24 virtual methods, 61 recovered fields)
/// Inherits: ccGobjEnemyFly, ccGobjEnemy, ccGobjBattle, ccGameObj
#[derive(Debug, Clone, Default)]
pub struct ccGobjEnemyFlyLL {
    pub vtable_rva: u32,
    pub field_0x0000: u8,
    pub field_0x0001: u32,
    pub field_0x0004: u8,
    pub field_0x0008: u16,
    pub field_0x000a: u16,
    pub field_0x000c: u32,
    pub field_0x0010: [f32; 4],
    pub field_0x0018: u64,
    pub field_0x001c: u8,
    pub field_0x001e: u16,
    pub field_0x0020: u32,
    pub field_0x0024: u16,
    pub field_0x002c: u16,
    pub field_0x0030: u32,
    pub field_0x0034: f32,
    pub field_0x0038: f32,
    pub field_0x003c: f32,
    pub field_0x0040: u32,
    pub field_0x007e: u16,
    pub field_0x0090: u32,
    pub field_0x0094: u32,
    pub field_0x0098: u64,
    pub field_0x009c: u32,
    pub field_0x00a0: u32,
    pub field_0x00a4: u32,
    pub field_0x00a8: u64,
    pub field_0x00ac: u32,
    pub field_0x00b0: u64,
    pub field_0x00b8: u64,
    pub field_0x00c4: u32,
    pub field_0x00e0: f32,
    pub field_0x00e4: f32,
    pub field_0x0120: u32,
    pub field_0x0124: u32,
    pub field_0x0128: u32,
    pub field_0x012c: u32,
    pub field_0x0130: u32,
    pub field_0x0134: u32,
    pub field_0x0138: u32,
    pub field_0x013c: u32,
    pub field_0x0140: u32,
    pub field_0x0144: u32,
    pub field_0x0148: u32,
    pub field_0x014c: u32,
    pub field_0x0150: u32,
    pub field_0x0154: u32,
    pub field_0x0158: u32,
    pub field_0x015c: u32,
    pub field_0x0170: u32,
    pub field_0x0198: u32,
    pub field_0x019c: u16,
    pub field_0x019e: u16,
    pub field_0x01b0: u64,
    pub field_0x01e4: u16,
    pub field_0x0278: u32,
    pub field_0x02da: u16,
    pub field_0x02fc: u8,
    pub field_0x0314: u16,
    pub field_0x03e0: u32,
    pub field_0x0420: u64,
    pub field_0x0428: u32,
}

impl ccGobjEnemyFlyLL {
    pub const CLASS_NAME: &'static str = "ccGobjEnemyFlyLL";
    pub const VTABLE_RVA: u32 = 0x008bd6a8;
    pub const METHOD_COUNT: usize = 24;
    pub const BASE_CLASSES: &'static [&'static str] = &["ccGobjEnemyFly", "ccGobjEnemy", "ccGobjBattle", "ccGameObj"];
    pub const VIRTUAL_METHODS: &'static [u64] = &[0x00000001804bbcc0, 0x00000001804b3120, 0x000000018045e4c0, 0x00000001804b6a60, 0x000000018047ebd0, 0x00000001804b9c40, 0x00000001804b4ec0, 0x00000001800d3090, 0x00000001800d3090, 0x00000001804ba020, 0x000000018047f200, 0x000000018047f0e0, 0x000000018045eec0, 0x000000018045efd0, 0x00000001804b9c00, 0x00000001804b66c0, 0x00000001804b6040, 0x00000001804b80d0, 0x00000001800d3090, 0x00000001800d3090, 0x00000001800d3090, 0x0000000180103e70, 0x00000001804b2920, 0x00000001804b9c20];

    pub fn new() -> Self {
        Self {
            vtable_rva: Self::VTABLE_RVA,
            field_0x0000: Default::default(),
            field_0x0001: Default::default(),
            field_0x0004: Default::default(),
            field_0x0008: Default::default(),
            field_0x000a: Default::default(),
            field_0x000c: Default::default(),
            field_0x0010: Default::default(),
            field_0x0018: Default::default(),
            field_0x001c: Default::default(),
            field_0x001e: Default::default(),
            field_0x0020: Default::default(),
            field_0x0024: Default::default(),
            field_0x002c: Default::default(),
            field_0x0030: Default::default(),
            field_0x0034: Default::default(),
            field_0x0038: Default::default(),
            field_0x003c: Default::default(),
            field_0x0040: Default::default(),
            field_0x007e: Default::default(),
            field_0x0090: Default::default(),
            field_0x0094: Default::default(),
            field_0x0098: Default::default(),
            field_0x009c: Default::default(),
            field_0x00a0: Default::default(),
            field_0x00a4: Default::default(),
            field_0x00a8: Default::default(),
            field_0x00ac: Default::default(),
            field_0x00b0: Default::default(),
            field_0x00b8: Default::default(),
            field_0x00c4: Default::default(),
            field_0x00e0: Default::default(),
            field_0x00e4: Default::default(),
            field_0x0120: Default::default(),
            field_0x0124: Default::default(),
            field_0x0128: Default::default(),
            field_0x012c: Default::default(),
            field_0x0130: Default::default(),
            field_0x0134: Default::default(),
            field_0x0138: Default::default(),
            field_0x013c: Default::default(),
            field_0x0140: Default::default(),
            field_0x0144: Default::default(),
            field_0x0148: Default::default(),
            field_0x014c: Default::default(),
            field_0x0150: Default::default(),
            field_0x0154: Default::default(),
            field_0x0158: Default::default(),
            field_0x015c: Default::default(),
            field_0x0170: Default::default(),
            field_0x0198: Default::default(),
            field_0x019c: Default::default(),
            field_0x019e: Default::default(),
            field_0x01b0: Default::default(),
            field_0x01e4: Default::default(),
            field_0x0278: Default::default(),
            field_0x02da: Default::default(),
            field_0x02fc: Default::default(),
            field_0x0314: Default::default(),
            field_0x03e0: Default::default(),
            field_0x0420: Default::default(),
            field_0x0428: Default::default(),
        }
    }

    /// Decompiled virtual method #0 @ RVA 0x004bbcc0 (16 instructions, 3 basic blocks)
    pub fn method_00(&mut self) {
    }

    /// Decompiled virtual method #1 @ RVA 0x004b3120 (112 instructions, 15 basic blocks)
    pub fn method_01(&mut self) {
        // Writes to field at offset +0x0030
        // Writes to field at offset +0x00c4
    }

    /// Decompiled virtual method #2 @ RVA 0x0045e4c0 (42 instructions, 4 basic blocks)
    pub fn method_02(&mut self) {
        // Writes to field at offset +0x0090
        // Writes to field at offset +0x0094
        // Writes to field at offset +0x0098
        // Writes to field at offset +0x009c
        // Writes to field at offset +0x00a0
        // Writes to field at offset +0x00a4
        // Writes to field at offset +0x00a8
        // Writes to field at offset +0x00ac
        // Writes to field at offset +0x0140
        // Writes to field at offset +0x0144
        // Writes to field at offset +0x0148
        // Writes to field at offset +0x014c
        // Writes to field at offset +0x0150
        // Writes to field at offset +0x0154
        // Writes to field at offset +0x0158
        // Writes to field at offset +0x015c
    }

    /// Decompiled virtual method #3 @ RVA 0x004b6a60 (104 instructions, 18 basic blocks)
    pub fn method_03(&mut self) {
        // Writes to field at offset +0x0000
        // Writes to field at offset +0x0001
        // Writes to field at offset +0x0004
        // Writes to field at offset +0x00b0
    }

    /// Decompiled virtual method #4 @ RVA 0x0047ebd0 (45 instructions, 12 basic blocks)
    pub fn method_04(&mut self) {
        // Writes to field at offset +0x0004
        // Writes to field at offset +0x02da
        // Writes to field at offset +0x0314
        // Writes to field at offset +0x03e0
    }

    /// Decompiled virtual method #5 @ RVA 0x004b9c40 (124 instructions, 20 basic blocks)
    pub fn method_05(&mut self) {
    }

    /// Decompiled virtual method #6 @ RVA 0x004b4ec0 (103 instructions, 17 basic blocks)
    pub fn method_06(&mut self) {
    }

    /// Decompiled virtual method #7 @ RVA 0x000d3090 (1 instructions, 1 basic blocks)
    pub fn method_07(&mut self) {
    }

    /// Decompiled virtual method #8 @ RVA 0x000d3090 (1 instructions, 1 basic blocks)
    pub fn method_08(&mut self) {
    }

    /// Decompiled virtual method #9 @ RVA 0x004ba020 (107 instructions, 25 basic blocks)
    pub fn method_09(&mut self) {
        // Writes to field at offset +0x0000
        // Writes to field at offset +0x0004
        // Writes to field at offset +0x0008
        // Writes to field at offset +0x000a
        // Writes to field at offset +0x000c
        // Writes to field at offset +0x01e4
        // Writes to field at offset +0x0428
    }

    /// Decompiled virtual method #10 @ RVA 0x0047f200 (58 instructions, 4 basic blocks)
    pub fn method_10(&mut self) {
    }

    /// Decompiled virtual method #11 @ RVA 0x0047f0e0 (15 instructions, 4 basic blocks)
    pub fn method_11(&mut self) {
    }

    /// Decompiled virtual method #12 @ RVA 0x0045eec0 (67 instructions, 22 basic blocks)
    pub fn method_12(&mut self) {
        // Writes to field at offset +0x001c
        // Writes to field at offset +0x0040
        // Writes to field at offset +0x0090
        // Writes to field at offset +0x00b0
    }

    /// Decompiled virtual method #13 @ RVA 0x0045efd0 (27 instructions, 5 basic blocks)
    pub fn method_13(&mut self) {
        // Writes to field at offset +0x0090
        // Writes to field at offset +0x00b0
    }

    /// Decompiled virtual method #14 @ RVA 0x004b9c00 (15 instructions, 2 basic blocks)
    pub fn method_14(&mut self) {
        // Writes to field at offset +0x03e0
        // Writes to field at offset +0x0420
        // Writes to field at offset +0x0428
    }

    /// Decompiled virtual method #15 @ RVA 0x004b66c0 (44 instructions, 12 basic blocks)
    pub fn method_15(&mut self) {
        // Writes to field at offset +0x0000
        // Writes to field at offset +0x0004
        // Writes to field at offset +0x0008
        // Writes to field at offset +0x000a
        // Writes to field at offset +0x00c4
        // Writes to field at offset +0x01e4
        // Writes to field at offset +0x02fc
    }

    /// Decompiled virtual method #16 @ RVA 0x004b6040 (54 instructions, 12 basic blocks)
    pub fn method_16(&mut self) {
    }

    /// Decompiled virtual method #17 @ RVA 0x004b80d0 (38 instructions, 8 basic blocks)
    pub fn method_17(&mut self) {
    }

    /// Decompiled virtual method #18 @ RVA 0x000d3090 (1 instructions, 1 basic blocks)
    pub fn method_18(&mut self) {
    }

    /// Decompiled virtual method #19 @ RVA 0x000d3090 (1 instructions, 1 basic blocks)
    pub fn method_19(&mut self) {
    }

    /// Decompiled virtual method #20 @ RVA 0x000d3090 (1 instructions, 1 basic blocks)
    pub fn method_20(&mut self) {
    }

    /// Decompiled virtual method #21 @ RVA 0x00103e70 (2 instructions, 1 basic blocks)
    pub fn method_21(&mut self) {
    }

    /// Decompiled virtual method #22 @ RVA 0x004b2920 (22 instructions, 3 basic blocks)
    pub fn method_22(&mut self) {
        // Writes to field at offset +0x01b0
    }

    /// Decompiled virtual method #23 @ RVA 0x004b9c20 (5 instructions, 1 basic blocks)
    pub fn method_23(&mut self) {
        // Writes to field at offset +0x03e0
        // Writes to field at offset +0x0420
        // Writes to field at offset +0x0428
    }

}

impl CcClass for ccGobjEnemyFlyLL {
    fn class_name(&self) -> &'static str { Self::CLASS_NAME }
    fn vtable_rva(&self) -> u32 { Self::VTABLE_RVA }
    fn virtual_method_count(&self) -> usize { Self::METHOD_COUNT }
    fn base_classes(&self) -> &'static [&'static str] { Self::BASE_CLASSES }
}

/// Decompiled CC2 class: `ccGobjEnemyFlyM` (VTable RVA: 0x008bd068, 24 virtual methods, 62 recovered fields)
/// Inherits: ccGobjEnemyFly, ccGobjEnemy, ccGobjBattle, ccGameObj
#[derive(Debug, Clone, Default)]
pub struct ccGobjEnemyFlyM {
    pub vtable_rva: u32,
    pub field_0x0000: u8,
    pub field_0x0001: u32,
    pub field_0x0004: u8,
    pub field_0x0008: u16,
    pub field_0x000a: u16,
    pub field_0x000c: u32,
    pub field_0x0010: [f32; 4],
    pub field_0x0018: u64,
    pub field_0x001c: u8,
    pub field_0x001e: u16,
    pub field_0x0020: u32,
    pub field_0x0024: u16,
    pub field_0x002c: u16,
    pub field_0x0030: u32,
    pub field_0x0034: f32,
    pub field_0x0038: f32,
    pub field_0x003c: f32,
    pub field_0x0040: u32,
    pub field_0x007e: u16,
    pub field_0x0090: u32,
    pub field_0x0094: u32,
    pub field_0x0098: u64,
    pub field_0x009c: u32,
    pub field_0x00a0: u32,
    pub field_0x00a4: u32,
    pub field_0x00a8: u64,
    pub field_0x00ac: u32,
    pub field_0x00b0: u64,
    pub field_0x00b8: u64,
    pub field_0x00c4: u32,
    pub field_0x00e0: f32,
    pub field_0x00e4: f32,
    pub field_0x0120: u32,
    pub field_0x0124: u32,
    pub field_0x0128: u32,
    pub field_0x012c: u32,
    pub field_0x0130: u32,
    pub field_0x0134: u32,
    pub field_0x0138: u32,
    pub field_0x013c: u32,
    pub field_0x0140: u32,
    pub field_0x0144: u32,
    pub field_0x0148: u32,
    pub field_0x014c: u32,
    pub field_0x0150: u32,
    pub field_0x0154: u32,
    pub field_0x0158: u32,
    pub field_0x015c: u32,
    pub field_0x0170: u32,
    pub field_0x0174: f32,
    pub field_0x0198: u32,
    pub field_0x019c: u16,
    pub field_0x019e: u16,
    pub field_0x01b0: u64,
    pub field_0x01e4: u16,
    pub field_0x0278: u32,
    pub field_0x02da: u16,
    pub field_0x02fc: u8,
    pub field_0x0314: u16,
    pub field_0x03e0: u32,
    pub field_0x0420: u64,
    pub field_0x0428: u32,
}

impl ccGobjEnemyFlyM {
    pub const CLASS_NAME: &'static str = "ccGobjEnemyFlyM";
    pub const VTABLE_RVA: u32 = 0x008bd068;
    pub const METHOD_COUNT: usize = 24;
    pub const BASE_CLASSES: &'static [&'static str] = &["ccGobjEnemyFly", "ccGobjEnemy", "ccGobjBattle", "ccGameObj"];
    pub const VIRTUAL_METHODS: &'static [u64] = &[0x00000001804bbcc0, 0x00000001804b3120, 0x000000018045e4c0, 0x00000001804b6a60, 0x000000018047ebd0, 0x00000001804b9c40, 0x00000001804b4ec0, 0x00000001800d3090, 0x00000001800d3090, 0x00000001804ba020, 0x000000018047f200, 0x000000018047f0e0, 0x000000018045eec0, 0x000000018045efd0, 0x00000001804b9c00, 0x00000001804b66c0, 0x00000001804b6040, 0x00000001804b80d0, 0x00000001800d3090, 0x00000001800d3090, 0x00000001800d3090, 0x0000000180103e70, 0x00000001804b2920, 0x00000001804bb6a0];

    pub fn new() -> Self {
        Self {
            vtable_rva: Self::VTABLE_RVA,
            field_0x0000: Default::default(),
            field_0x0001: Default::default(),
            field_0x0004: Default::default(),
            field_0x0008: Default::default(),
            field_0x000a: Default::default(),
            field_0x000c: Default::default(),
            field_0x0010: Default::default(),
            field_0x0018: Default::default(),
            field_0x001c: Default::default(),
            field_0x001e: Default::default(),
            field_0x0020: Default::default(),
            field_0x0024: Default::default(),
            field_0x002c: Default::default(),
            field_0x0030: Default::default(),
            field_0x0034: Default::default(),
            field_0x0038: Default::default(),
            field_0x003c: Default::default(),
            field_0x0040: Default::default(),
            field_0x007e: Default::default(),
            field_0x0090: Default::default(),
            field_0x0094: Default::default(),
            field_0x0098: Default::default(),
            field_0x009c: Default::default(),
            field_0x00a0: Default::default(),
            field_0x00a4: Default::default(),
            field_0x00a8: Default::default(),
            field_0x00ac: Default::default(),
            field_0x00b0: Default::default(),
            field_0x00b8: Default::default(),
            field_0x00c4: Default::default(),
            field_0x00e0: Default::default(),
            field_0x00e4: Default::default(),
            field_0x0120: Default::default(),
            field_0x0124: Default::default(),
            field_0x0128: Default::default(),
            field_0x012c: Default::default(),
            field_0x0130: Default::default(),
            field_0x0134: Default::default(),
            field_0x0138: Default::default(),
            field_0x013c: Default::default(),
            field_0x0140: Default::default(),
            field_0x0144: Default::default(),
            field_0x0148: Default::default(),
            field_0x014c: Default::default(),
            field_0x0150: Default::default(),
            field_0x0154: Default::default(),
            field_0x0158: Default::default(),
            field_0x015c: Default::default(),
            field_0x0170: Default::default(),
            field_0x0174: Default::default(),
            field_0x0198: Default::default(),
            field_0x019c: Default::default(),
            field_0x019e: Default::default(),
            field_0x01b0: Default::default(),
            field_0x01e4: Default::default(),
            field_0x0278: Default::default(),
            field_0x02da: Default::default(),
            field_0x02fc: Default::default(),
            field_0x0314: Default::default(),
            field_0x03e0: Default::default(),
            field_0x0420: Default::default(),
            field_0x0428: Default::default(),
        }
    }

    /// Decompiled virtual method #0 @ RVA 0x004bbcc0 (16 instructions, 3 basic blocks)
    pub fn method_00(&mut self) {
    }

    /// Decompiled virtual method #1 @ RVA 0x004b3120 (112 instructions, 15 basic blocks)
    pub fn method_01(&mut self) {
        // Writes to field at offset +0x0030
        // Writes to field at offset +0x00c4
    }

    /// Decompiled virtual method #2 @ RVA 0x0045e4c0 (42 instructions, 4 basic blocks)
    pub fn method_02(&mut self) {
        // Writes to field at offset +0x0090
        // Writes to field at offset +0x0094
        // Writes to field at offset +0x0098
        // Writes to field at offset +0x009c
        // Writes to field at offset +0x00a0
        // Writes to field at offset +0x00a4
        // Writes to field at offset +0x00a8
        // Writes to field at offset +0x00ac
        // Writes to field at offset +0x0140
        // Writes to field at offset +0x0144
        // Writes to field at offset +0x0148
        // Writes to field at offset +0x014c
        // Writes to field at offset +0x0150
        // Writes to field at offset +0x0154
        // Writes to field at offset +0x0158
        // Writes to field at offset +0x015c
    }

    /// Decompiled virtual method #3 @ RVA 0x004b6a60 (104 instructions, 18 basic blocks)
    pub fn method_03(&mut self) {
        // Writes to field at offset +0x0000
        // Writes to field at offset +0x0001
        // Writes to field at offset +0x0004
        // Writes to field at offset +0x00b0
    }

    /// Decompiled virtual method #4 @ RVA 0x0047ebd0 (45 instructions, 12 basic blocks)
    pub fn method_04(&mut self) {
        // Writes to field at offset +0x0004
        // Writes to field at offset +0x02da
        // Writes to field at offset +0x0314
        // Writes to field at offset +0x03e0
    }

    /// Decompiled virtual method #5 @ RVA 0x004b9c40 (124 instructions, 20 basic blocks)
    pub fn method_05(&mut self) {
    }

    /// Decompiled virtual method #6 @ RVA 0x004b4ec0 (103 instructions, 17 basic blocks)
    pub fn method_06(&mut self) {
    }

    /// Decompiled virtual method #7 @ RVA 0x000d3090 (1 instructions, 1 basic blocks)
    pub fn method_07(&mut self) {
    }

    /// Decompiled virtual method #8 @ RVA 0x000d3090 (1 instructions, 1 basic blocks)
    pub fn method_08(&mut self) {
    }

    /// Decompiled virtual method #9 @ RVA 0x004ba020 (107 instructions, 25 basic blocks)
    pub fn method_09(&mut self) {
        // Writes to field at offset +0x0000
        // Writes to field at offset +0x0004
        // Writes to field at offset +0x0008
        // Writes to field at offset +0x000a
        // Writes to field at offset +0x000c
        // Writes to field at offset +0x01e4
        // Writes to field at offset +0x0428
    }

    /// Decompiled virtual method #10 @ RVA 0x0047f200 (58 instructions, 4 basic blocks)
    pub fn method_10(&mut self) {
    }

    /// Decompiled virtual method #11 @ RVA 0x0047f0e0 (15 instructions, 4 basic blocks)
    pub fn method_11(&mut self) {
    }

    /// Decompiled virtual method #12 @ RVA 0x0045eec0 (67 instructions, 22 basic blocks)
    pub fn method_12(&mut self) {
        // Writes to field at offset +0x001c
        // Writes to field at offset +0x0040
        // Writes to field at offset +0x0090
        // Writes to field at offset +0x00b0
    }

    /// Decompiled virtual method #13 @ RVA 0x0045efd0 (27 instructions, 5 basic blocks)
    pub fn method_13(&mut self) {
        // Writes to field at offset +0x0090
        // Writes to field at offset +0x00b0
    }

    /// Decompiled virtual method #14 @ RVA 0x004b9c00 (15 instructions, 2 basic blocks)
    pub fn method_14(&mut self) {
        // Writes to field at offset +0x03e0
        // Writes to field at offset +0x0420
        // Writes to field at offset +0x0428
    }

    /// Decompiled virtual method #15 @ RVA 0x004b66c0 (44 instructions, 12 basic blocks)
    pub fn method_15(&mut self) {
        // Writes to field at offset +0x0000
        // Writes to field at offset +0x0004
        // Writes to field at offset +0x0008
        // Writes to field at offset +0x000a
        // Writes to field at offset +0x00c4
        // Writes to field at offset +0x01e4
        // Writes to field at offset +0x02fc
    }

    /// Decompiled virtual method #16 @ RVA 0x004b6040 (54 instructions, 12 basic blocks)
    pub fn method_16(&mut self) {
    }

    /// Decompiled virtual method #17 @ RVA 0x004b80d0 (38 instructions, 8 basic blocks)
    pub fn method_17(&mut self) {
    }

    /// Decompiled virtual method #18 @ RVA 0x000d3090 (1 instructions, 1 basic blocks)
    pub fn method_18(&mut self) {
    }

    /// Decompiled virtual method #19 @ RVA 0x000d3090 (1 instructions, 1 basic blocks)
    pub fn method_19(&mut self) {
    }

    /// Decompiled virtual method #20 @ RVA 0x000d3090 (1 instructions, 1 basic blocks)
    pub fn method_20(&mut self) {
    }

    /// Decompiled virtual method #21 @ RVA 0x00103e70 (2 instructions, 1 basic blocks)
    pub fn method_21(&mut self) {
    }

    /// Decompiled virtual method #22 @ RVA 0x004b2920 (22 instructions, 3 basic blocks)
    pub fn method_22(&mut self) {
        // Writes to field at offset +0x01b0
    }

    /// Decompiled virtual method #23 @ RVA 0x004bb6a0 (10 instructions, 1 basic blocks)
    pub fn method_23(&mut self) {
        // Writes to field at offset +0x0174
        // Writes to field at offset +0x03e0
        // Writes to field at offset +0x0420
        // Writes to field at offset +0x0428
    }

}

impl CcClass for ccGobjEnemyFlyM {
    fn class_name(&self) -> &'static str { Self::CLASS_NAME }
    fn vtable_rva(&self) -> u32 { Self::VTABLE_RVA }
    fn virtual_method_count(&self) -> usize { Self::METHOD_COUNT }
    fn base_classes(&self) -> &'static [&'static str] { Self::BASE_CLASSES }
}

/// Decompiled CC2 class: `ccGobjEnemyFlyS` (VTable RVA: 0x008bcfa0, 24 virtual methods, 61 recovered fields)
/// Inherits: ccGobjEnemyFly, ccGobjEnemy, ccGobjBattle, ccGameObj
#[derive(Debug, Clone, Default)]
pub struct ccGobjEnemyFlyS {
    pub vtable_rva: u32,
    pub field_0x0000: u8,
    pub field_0x0001: u32,
    pub field_0x0004: u8,
    pub field_0x0008: u16,
    pub field_0x000a: u16,
    pub field_0x000c: u32,
    pub field_0x0010: [f32; 4],
    pub field_0x0018: u64,
    pub field_0x001c: u8,
    pub field_0x001e: u16,
    pub field_0x0020: u32,
    pub field_0x0024: u16,
    pub field_0x002c: u16,
    pub field_0x0030: u32,
    pub field_0x0034: f32,
    pub field_0x0038: f32,
    pub field_0x003c: f32,
    pub field_0x0040: u32,
    pub field_0x007e: u16,
    pub field_0x0090: u32,
    pub field_0x0094: u32,
    pub field_0x0098: u64,
    pub field_0x009c: u32,
    pub field_0x00a0: u32,
    pub field_0x00a4: u32,
    pub field_0x00a8: u64,
    pub field_0x00ac: u32,
    pub field_0x00b0: u64,
    pub field_0x00b8: u64,
    pub field_0x00c4: u32,
    pub field_0x00e0: f32,
    pub field_0x00e4: f32,
    pub field_0x0120: u32,
    pub field_0x0124: u32,
    pub field_0x0128: u32,
    pub field_0x012c: u32,
    pub field_0x0130: u32,
    pub field_0x0134: u32,
    pub field_0x0138: u32,
    pub field_0x013c: u32,
    pub field_0x0140: u32,
    pub field_0x0144: u32,
    pub field_0x0148: u32,
    pub field_0x014c: u32,
    pub field_0x0150: u32,
    pub field_0x0154: u32,
    pub field_0x0158: u32,
    pub field_0x015c: u32,
    pub field_0x0170: u32,
    pub field_0x0198: u32,
    pub field_0x019c: u16,
    pub field_0x019e: u16,
    pub field_0x01b0: u64,
    pub field_0x01e4: u16,
    pub field_0x0278: u32,
    pub field_0x02da: u16,
    pub field_0x02fc: u8,
    pub field_0x0314: u16,
    pub field_0x03e0: u32,
    pub field_0x0420: u64,
    pub field_0x0428: u32,
}

impl ccGobjEnemyFlyS {
    pub const CLASS_NAME: &'static str = "ccGobjEnemyFlyS";
    pub const VTABLE_RVA: u32 = 0x008bcfa0;
    pub const METHOD_COUNT: usize = 24;
    pub const BASE_CLASSES: &'static [&'static str] = &["ccGobjEnemyFly", "ccGobjEnemy", "ccGobjBattle", "ccGameObj"];
    pub const VIRTUAL_METHODS: &'static [u64] = &[0x00000001804bbcc0, 0x00000001804b3120, 0x000000018045e4c0, 0x00000001804b6a60, 0x000000018047ebd0, 0x00000001804b9c40, 0x00000001804b4ec0, 0x00000001800d3090, 0x00000001800d3090, 0x00000001804ba020, 0x000000018047f200, 0x000000018047f0e0, 0x000000018045eec0, 0x000000018045efd0, 0x00000001804b9c00, 0x00000001804b66c0, 0x00000001804b6040, 0x00000001804b80d0, 0x00000001800d3090, 0x00000001800d3090, 0x00000001800d3090, 0x0000000180103e70, 0x00000001804b2920, 0x00000001804b9c20];

    pub fn new() -> Self {
        Self {
            vtable_rva: Self::VTABLE_RVA,
            field_0x0000: Default::default(),
            field_0x0001: Default::default(),
            field_0x0004: Default::default(),
            field_0x0008: Default::default(),
            field_0x000a: Default::default(),
            field_0x000c: Default::default(),
            field_0x0010: Default::default(),
            field_0x0018: Default::default(),
            field_0x001c: Default::default(),
            field_0x001e: Default::default(),
            field_0x0020: Default::default(),
            field_0x0024: Default::default(),
            field_0x002c: Default::default(),
            field_0x0030: Default::default(),
            field_0x0034: Default::default(),
            field_0x0038: Default::default(),
            field_0x003c: Default::default(),
            field_0x0040: Default::default(),
            field_0x007e: Default::default(),
            field_0x0090: Default::default(),
            field_0x0094: Default::default(),
            field_0x0098: Default::default(),
            field_0x009c: Default::default(),
            field_0x00a0: Default::default(),
            field_0x00a4: Default::default(),
            field_0x00a8: Default::default(),
            field_0x00ac: Default::default(),
            field_0x00b0: Default::default(),
            field_0x00b8: Default::default(),
            field_0x00c4: Default::default(),
            field_0x00e0: Default::default(),
            field_0x00e4: Default::default(),
            field_0x0120: Default::default(),
            field_0x0124: Default::default(),
            field_0x0128: Default::default(),
            field_0x012c: Default::default(),
            field_0x0130: Default::default(),
            field_0x0134: Default::default(),
            field_0x0138: Default::default(),
            field_0x013c: Default::default(),
            field_0x0140: Default::default(),
            field_0x0144: Default::default(),
            field_0x0148: Default::default(),
            field_0x014c: Default::default(),
            field_0x0150: Default::default(),
            field_0x0154: Default::default(),
            field_0x0158: Default::default(),
            field_0x015c: Default::default(),
            field_0x0170: Default::default(),
            field_0x0198: Default::default(),
            field_0x019c: Default::default(),
            field_0x019e: Default::default(),
            field_0x01b0: Default::default(),
            field_0x01e4: Default::default(),
            field_0x0278: Default::default(),
            field_0x02da: Default::default(),
            field_0x02fc: Default::default(),
            field_0x0314: Default::default(),
            field_0x03e0: Default::default(),
            field_0x0420: Default::default(),
            field_0x0428: Default::default(),
        }
    }

    /// Decompiled virtual method #0 @ RVA 0x004bbcc0 (16 instructions, 3 basic blocks)
    pub fn method_00(&mut self) {
    }

    /// Decompiled virtual method #1 @ RVA 0x004b3120 (112 instructions, 15 basic blocks)
    pub fn method_01(&mut self) {
        // Writes to field at offset +0x0030
        // Writes to field at offset +0x00c4
    }

    /// Decompiled virtual method #2 @ RVA 0x0045e4c0 (42 instructions, 4 basic blocks)
    pub fn method_02(&mut self) {
        // Writes to field at offset +0x0090
        // Writes to field at offset +0x0094
        // Writes to field at offset +0x0098
        // Writes to field at offset +0x009c
        // Writes to field at offset +0x00a0
        // Writes to field at offset +0x00a4
        // Writes to field at offset +0x00a8
        // Writes to field at offset +0x00ac
        // Writes to field at offset +0x0140
        // Writes to field at offset +0x0144
        // Writes to field at offset +0x0148
        // Writes to field at offset +0x014c
        // Writes to field at offset +0x0150
        // Writes to field at offset +0x0154
        // Writes to field at offset +0x0158
        // Writes to field at offset +0x015c
    }

    /// Decompiled virtual method #3 @ RVA 0x004b6a60 (104 instructions, 18 basic blocks)
    pub fn method_03(&mut self) {
        // Writes to field at offset +0x0000
        // Writes to field at offset +0x0001
        // Writes to field at offset +0x0004
        // Writes to field at offset +0x00b0
    }

    /// Decompiled virtual method #4 @ RVA 0x0047ebd0 (45 instructions, 12 basic blocks)
    pub fn method_04(&mut self) {
        // Writes to field at offset +0x0004
        // Writes to field at offset +0x02da
        // Writes to field at offset +0x0314
        // Writes to field at offset +0x03e0
    }

    /// Decompiled virtual method #5 @ RVA 0x004b9c40 (124 instructions, 20 basic blocks)
    pub fn method_05(&mut self) {
    }

    /// Decompiled virtual method #6 @ RVA 0x004b4ec0 (103 instructions, 17 basic blocks)
    pub fn method_06(&mut self) {
    }

    /// Decompiled virtual method #7 @ RVA 0x000d3090 (1 instructions, 1 basic blocks)
    pub fn method_07(&mut self) {
    }

    /// Decompiled virtual method #8 @ RVA 0x000d3090 (1 instructions, 1 basic blocks)
    pub fn method_08(&mut self) {
    }

    /// Decompiled virtual method #9 @ RVA 0x004ba020 (107 instructions, 25 basic blocks)
    pub fn method_09(&mut self) {
        // Writes to field at offset +0x0000
        // Writes to field at offset +0x0004
        // Writes to field at offset +0x0008
        // Writes to field at offset +0x000a
        // Writes to field at offset +0x000c
        // Writes to field at offset +0x01e4
        // Writes to field at offset +0x0428
    }

    /// Decompiled virtual method #10 @ RVA 0x0047f200 (58 instructions, 4 basic blocks)
    pub fn method_10(&mut self) {
    }

    /// Decompiled virtual method #11 @ RVA 0x0047f0e0 (15 instructions, 4 basic blocks)
    pub fn method_11(&mut self) {
    }

    /// Decompiled virtual method #12 @ RVA 0x0045eec0 (67 instructions, 22 basic blocks)
    pub fn method_12(&mut self) {
        // Writes to field at offset +0x001c
        // Writes to field at offset +0x0040
        // Writes to field at offset +0x0090
        // Writes to field at offset +0x00b0
    }

    /// Decompiled virtual method #13 @ RVA 0x0045efd0 (27 instructions, 5 basic blocks)
    pub fn method_13(&mut self) {
        // Writes to field at offset +0x0090
        // Writes to field at offset +0x00b0
    }

    /// Decompiled virtual method #14 @ RVA 0x004b9c00 (15 instructions, 2 basic blocks)
    pub fn method_14(&mut self) {
        // Writes to field at offset +0x03e0
        // Writes to field at offset +0x0420
        // Writes to field at offset +0x0428
    }

    /// Decompiled virtual method #15 @ RVA 0x004b66c0 (44 instructions, 12 basic blocks)
    pub fn method_15(&mut self) {
        // Writes to field at offset +0x0000
        // Writes to field at offset +0x0004
        // Writes to field at offset +0x0008
        // Writes to field at offset +0x000a
        // Writes to field at offset +0x00c4
        // Writes to field at offset +0x01e4
        // Writes to field at offset +0x02fc
    }

    /// Decompiled virtual method #16 @ RVA 0x004b6040 (54 instructions, 12 basic blocks)
    pub fn method_16(&mut self) {
    }

    /// Decompiled virtual method #17 @ RVA 0x004b80d0 (38 instructions, 8 basic blocks)
    pub fn method_17(&mut self) {
    }

    /// Decompiled virtual method #18 @ RVA 0x000d3090 (1 instructions, 1 basic blocks)
    pub fn method_18(&mut self) {
    }

    /// Decompiled virtual method #19 @ RVA 0x000d3090 (1 instructions, 1 basic blocks)
    pub fn method_19(&mut self) {
    }

    /// Decompiled virtual method #20 @ RVA 0x000d3090 (1 instructions, 1 basic blocks)
    pub fn method_20(&mut self) {
    }

    /// Decompiled virtual method #21 @ RVA 0x00103e70 (2 instructions, 1 basic blocks)
    pub fn method_21(&mut self) {
    }

    /// Decompiled virtual method #22 @ RVA 0x004b2920 (22 instructions, 3 basic blocks)
    pub fn method_22(&mut self) {
        // Writes to field at offset +0x01b0
    }

    /// Decompiled virtual method #23 @ RVA 0x004b9c20 (5 instructions, 1 basic blocks)
    pub fn method_23(&mut self) {
        // Writes to field at offset +0x03e0
        // Writes to field at offset +0x0420
        // Writes to field at offset +0x0428
    }

}

impl CcClass for ccGobjEnemyFlyS {
    fn class_name(&self) -> &'static str { Self::CLASS_NAME }
    fn vtable_rva(&self) -> u32 { Self::VTABLE_RVA }
    fn virtual_method_count(&self) -> usize { Self::METHOD_COUNT }
    fn base_classes(&self) -> &'static [&'static str] { Self::BASE_CLASSES }
}

/// Decompiled CC2 class: `ccGobjEnemyHardshell` (VTable RVA: 0x008bdc20, 24 virtual methods, 59 recovered fields)
/// Inherits: ccGobjEnemy, ccGobjBattle, ccGameObj
#[derive(Debug, Clone, Default)]
pub struct ccGobjEnemyHardshell {
    pub vtable_rva: u32,
    pub field_0x0000: u64,
    pub field_0x0001: u32,
    pub field_0x0004: u8,
    pub field_0x0008: u16,
    pub field_0x000a: u16,
    pub field_0x0010: u64,
    pub field_0x0018: u64,
    pub field_0x001c: u8,
    pub field_0x001e: u16,
    pub field_0x0020: u32,
    pub field_0x0024: u16,
    pub field_0x0028: u8,
    pub field_0x0029: u8,
    pub field_0x002c: f32,
    pub field_0x0030: u32,
    pub field_0x0040: u32,
    pub field_0x007e: u16,
    pub field_0x0090: u32,
    pub field_0x0094: u32,
    pub field_0x0098: u64,
    pub field_0x009c: u32,
    pub field_0x00a0: u32,
    pub field_0x00a4: u32,
    pub field_0x00a8: u64,
    pub field_0x00ac: u32,
    pub field_0x00b0: u64,
    pub field_0x00b8: u64,
    pub field_0x00c4: u32,
    pub field_0x00e0: f32,
    pub field_0x00e4: f32,
    pub field_0x0114: u32,
    pub field_0x0120: u32,
    pub field_0x0124: u32,
    pub field_0x0128: u32,
    pub field_0x012c: u32,
    pub field_0x0130: u32,
    pub field_0x0134: u32,
    pub field_0x0138: u32,
    pub field_0x013c: u32,
    pub field_0x0140: u32,
    pub field_0x0144: u32,
    pub field_0x0148: u32,
    pub field_0x014c: u32,
    pub field_0x0150: u32,
    pub field_0x0154: u32,
    pub field_0x0158: u32,
    pub field_0x015c: u32,
    pub field_0x0170: u32,
    pub field_0x0198: u32,
    pub field_0x019c: u16,
    pub field_0x019e: u16,
    pub field_0x01b0: u64,
    pub field_0x01e4: u16,
    pub field_0x0278: u32,
    pub field_0x02da: u16,
    pub field_0x02fc: u8,
    pub field_0x0314: u16,
    pub field_0x03e0: u32,
    pub field_0x0420: u64,
}

impl ccGobjEnemyHardshell {
    pub const CLASS_NAME: &'static str = "ccGobjEnemyHardshell";
    pub const VTABLE_RVA: u32 = 0x008bdc20;
    pub const METHOD_COUNT: usize = 24;
    pub const BASE_CLASSES: &'static [&'static str] = &["ccGobjEnemy", "ccGobjBattle", "ccGameObj"];
    pub const VIRTUAL_METHODS: &'static [u64] = &[0x00000001804b2780, 0x00000001804b3120, 0x000000018045e4c0, 0x00000001804b6a60, 0x000000018047ebd0, 0x00000001804b8d20, 0x00000001804b4ec0, 0x00000001800d3090, 0x00000001800d3090, 0x00000001804b9340, 0x00000001804b9210, 0x000000018047f0e0, 0x000000018045eec0, 0x000000018045efd0, 0x00000001804b8ce0, 0x00000001804b66c0, 0x00000001804b6040, 0x00000001804b80d0, 0x00000001800d3090, 0x00000001800d3090, 0x00000001800d3090, 0x0000000180103e70, 0x00000001804b2920, 0x00000001804b8cb0];

    pub fn new() -> Self {
        Self {
            vtable_rva: Self::VTABLE_RVA,
            field_0x0000: Default::default(),
            field_0x0001: Default::default(),
            field_0x0004: Default::default(),
            field_0x0008: Default::default(),
            field_0x000a: Default::default(),
            field_0x0010: Default::default(),
            field_0x0018: Default::default(),
            field_0x001c: Default::default(),
            field_0x001e: Default::default(),
            field_0x0020: Default::default(),
            field_0x0024: Default::default(),
            field_0x0028: Default::default(),
            field_0x0029: Default::default(),
            field_0x002c: Default::default(),
            field_0x0030: Default::default(),
            field_0x0040: Default::default(),
            field_0x007e: Default::default(),
            field_0x0090: Default::default(),
            field_0x0094: Default::default(),
            field_0x0098: Default::default(),
            field_0x009c: Default::default(),
            field_0x00a0: Default::default(),
            field_0x00a4: Default::default(),
            field_0x00a8: Default::default(),
            field_0x00ac: Default::default(),
            field_0x00b0: Default::default(),
            field_0x00b8: Default::default(),
            field_0x00c4: Default::default(),
            field_0x00e0: Default::default(),
            field_0x00e4: Default::default(),
            field_0x0114: Default::default(),
            field_0x0120: Default::default(),
            field_0x0124: Default::default(),
            field_0x0128: Default::default(),
            field_0x012c: Default::default(),
            field_0x0130: Default::default(),
            field_0x0134: Default::default(),
            field_0x0138: Default::default(),
            field_0x013c: Default::default(),
            field_0x0140: Default::default(),
            field_0x0144: Default::default(),
            field_0x0148: Default::default(),
            field_0x014c: Default::default(),
            field_0x0150: Default::default(),
            field_0x0154: Default::default(),
            field_0x0158: Default::default(),
            field_0x015c: Default::default(),
            field_0x0170: Default::default(),
            field_0x0198: Default::default(),
            field_0x019c: Default::default(),
            field_0x019e: Default::default(),
            field_0x01b0: Default::default(),
            field_0x01e4: Default::default(),
            field_0x0278: Default::default(),
            field_0x02da: Default::default(),
            field_0x02fc: Default::default(),
            field_0x0314: Default::default(),
            field_0x03e0: Default::default(),
            field_0x0420: Default::default(),
        }
    }

    /// Decompiled virtual method #0 @ RVA 0x004b2780 (25 instructions, 5 basic blocks)
    pub fn method_00(&mut self) {
        // Writes to field at offset +0x0000
    }

    /// Decompiled virtual method #1 @ RVA 0x004b3120 (112 instructions, 15 basic blocks)
    pub fn method_01(&mut self) {
        // Writes to field at offset +0x0030
        // Writes to field at offset +0x00c4
    }

    /// Decompiled virtual method #2 @ RVA 0x0045e4c0 (42 instructions, 4 basic blocks)
    pub fn method_02(&mut self) {
        // Writes to field at offset +0x0090
        // Writes to field at offset +0x0094
        // Writes to field at offset +0x0098
        // Writes to field at offset +0x009c
        // Writes to field at offset +0x00a0
        // Writes to field at offset +0x00a4
        // Writes to field at offset +0x00a8
        // Writes to field at offset +0x00ac
        // Writes to field at offset +0x0140
        // Writes to field at offset +0x0144
        // Writes to field at offset +0x0148
        // Writes to field at offset +0x014c
        // Writes to field at offset +0x0150
        // Writes to field at offset +0x0154
        // Writes to field at offset +0x0158
        // Writes to field at offset +0x015c
    }

    /// Decompiled virtual method #3 @ RVA 0x004b6a60 (104 instructions, 18 basic blocks)
    pub fn method_03(&mut self) {
        // Writes to field at offset +0x0000
        // Writes to field at offset +0x0001
        // Writes to field at offset +0x0004
        // Writes to field at offset +0x00b0
    }

    /// Decompiled virtual method #4 @ RVA 0x0047ebd0 (45 instructions, 12 basic blocks)
    pub fn method_04(&mut self) {
        // Writes to field at offset +0x0004
        // Writes to field at offset +0x02da
        // Writes to field at offset +0x0314
        // Writes to field at offset +0x03e0
    }

    /// Decompiled virtual method #5 @ RVA 0x004b8d20 (104 instructions, 18 basic blocks)
    pub fn method_05(&mut self) {
        // Writes to field at offset +0x0028
    }

    /// Decompiled virtual method #6 @ RVA 0x004b4ec0 (103 instructions, 17 basic blocks)
    pub fn method_06(&mut self) {
    }

    /// Decompiled virtual method #7 @ RVA 0x000d3090 (1 instructions, 1 basic blocks)
    pub fn method_07(&mut self) {
    }

    /// Decompiled virtual method #8 @ RVA 0x000d3090 (1 instructions, 1 basic blocks)
    pub fn method_08(&mut self) {
    }

    /// Decompiled virtual method #9 @ RVA 0x004b9340 (116 instructions, 17 basic blocks)
    pub fn method_09(&mut self) {
        // Writes to field at offset +0x0000
        // Writes to field at offset +0x0000
        // Writes to field at offset +0x0001
        // Writes to field at offset +0x0420
    }

    /// Decompiled virtual method #10 @ RVA 0x004b9210 (63 instructions, 13 basic blocks)
    pub fn method_10(&mut self) {
        // Writes to field at offset +0x0029
        // Writes to field at offset +0x002c
        // Writes to field at offset +0x0030
        // Writes to field at offset +0x0114
    }

    /// Decompiled virtual method #11 @ RVA 0x0047f0e0 (15 instructions, 4 basic blocks)
    pub fn method_11(&mut self) {
    }

    /// Decompiled virtual method #12 @ RVA 0x0045eec0 (67 instructions, 22 basic blocks)
    pub fn method_12(&mut self) {
        // Writes to field at offset +0x001c
        // Writes to field at offset +0x0040
        // Writes to field at offset +0x0090
        // Writes to field at offset +0x00b0
    }

    /// Decompiled virtual method #13 @ RVA 0x0045efd0 (27 instructions, 5 basic blocks)
    pub fn method_13(&mut self) {
        // Writes to field at offset +0x0090
        // Writes to field at offset +0x00b0
    }

    /// Decompiled virtual method #14 @ RVA 0x004b8ce0 (114 instructions, 20 basic blocks)
    pub fn method_14(&mut self) {
        // Writes to field at offset +0x0028
    }

    /// Decompiled virtual method #15 @ RVA 0x004b66c0 (44 instructions, 12 basic blocks)
    pub fn method_15(&mut self) {
        // Writes to field at offset +0x0000
        // Writes to field at offset +0x0004
        // Writes to field at offset +0x0008
        // Writes to field at offset +0x000a
        // Writes to field at offset +0x00c4
        // Writes to field at offset +0x01e4
        // Writes to field at offset +0x02fc
    }

    /// Decompiled virtual method #16 @ RVA 0x004b6040 (54 instructions, 12 basic blocks)
    pub fn method_16(&mut self) {
    }

    /// Decompiled virtual method #17 @ RVA 0x004b80d0 (38 instructions, 8 basic blocks)
    pub fn method_17(&mut self) {
    }

    /// Decompiled virtual method #18 @ RVA 0x000d3090 (1 instructions, 1 basic blocks)
    pub fn method_18(&mut self) {
    }

    /// Decompiled virtual method #19 @ RVA 0x000d3090 (1 instructions, 1 basic blocks)
    pub fn method_19(&mut self) {
    }

    /// Decompiled virtual method #20 @ RVA 0x000d3090 (1 instructions, 1 basic blocks)
    pub fn method_20(&mut self) {
    }

    /// Decompiled virtual method #21 @ RVA 0x00103e70 (2 instructions, 1 basic blocks)
    pub fn method_21(&mut self) {
    }

    /// Decompiled virtual method #22 @ RVA 0x004b2920 (22 instructions, 3 basic blocks)
    pub fn method_22(&mut self) {
        // Writes to field at offset +0x01b0
    }

    /// Decompiled virtual method #23 @ RVA 0x004b8cb0 (6 instructions, 1 basic blocks)
    pub fn method_23(&mut self) {
        // Writes to field at offset +0x03e0
        // Writes to field at offset +0x0420
    }

}

impl CcClass for ccGobjEnemyHardshell {
    fn class_name(&self) -> &'static str { Self::CLASS_NAME }
    fn vtable_rva(&self) -> u32 { Self::VTABLE_RVA }
    fn virtual_method_count(&self) -> usize { Self::METHOD_COUNT }
    fn base_classes(&self) -> &'static [&'static str] { Self::BASE_CLASSES }
}

/// Decompiled CC2 class: `ccGobjEnemyHardshellL` (VTable RVA: 0x008bd2c0, 24 virtual methods, 59 recovered fields)
/// Inherits: ccGobjEnemyHardshell, ccGobjEnemy, ccGobjBattle, ccGameObj
#[derive(Debug, Clone, Default)]
pub struct ccGobjEnemyHardshellL {
    pub vtable_rva: u32,
    pub field_0x0000: u64,
    pub field_0x0001: u32,
    pub field_0x0004: u8,
    pub field_0x0008: u16,
    pub field_0x000a: u16,
    pub field_0x0010: u64,
    pub field_0x0018: u64,
    pub field_0x001c: u8,
    pub field_0x001e: u16,
    pub field_0x0020: u32,
    pub field_0x0024: u16,
    pub field_0x0028: u8,
    pub field_0x0029: u8,
    pub field_0x002c: f32,
    pub field_0x0030: u32,
    pub field_0x0040: u32,
    pub field_0x007e: u16,
    pub field_0x0090: u32,
    pub field_0x0094: u32,
    pub field_0x0098: u64,
    pub field_0x009c: u32,
    pub field_0x00a0: u32,
    pub field_0x00a4: u32,
    pub field_0x00a8: u64,
    pub field_0x00ac: u32,
    pub field_0x00b0: u64,
    pub field_0x00b8: u64,
    pub field_0x00c4: u32,
    pub field_0x00e0: f32,
    pub field_0x00e4: f32,
    pub field_0x0114: u32,
    pub field_0x0120: u32,
    pub field_0x0124: u32,
    pub field_0x0128: u32,
    pub field_0x012c: u32,
    pub field_0x0130: u32,
    pub field_0x0134: u32,
    pub field_0x0138: u32,
    pub field_0x013c: u32,
    pub field_0x0140: u32,
    pub field_0x0144: u32,
    pub field_0x0148: u32,
    pub field_0x014c: u32,
    pub field_0x0150: u32,
    pub field_0x0154: u32,
    pub field_0x0158: u32,
    pub field_0x015c: u32,
    pub field_0x0170: u32,
    pub field_0x0198: u32,
    pub field_0x019c: u16,
    pub field_0x019e: u16,
    pub field_0x01b0: u64,
    pub field_0x01e4: u16,
    pub field_0x0278: u32,
    pub field_0x02da: u16,
    pub field_0x02fc: u8,
    pub field_0x0314: u16,
    pub field_0x03e0: u32,
    pub field_0x0420: u64,
}

impl ccGobjEnemyHardshellL {
    pub const CLASS_NAME: &'static str = "ccGobjEnemyHardshellL";
    pub const VTABLE_RVA: u32 = 0x008bd2c0;
    pub const METHOD_COUNT: usize = 24;
    pub const BASE_CLASSES: &'static [&'static str] = &["ccGobjEnemyHardshell", "ccGobjEnemy", "ccGobjBattle", "ccGameObj"];
    pub const VIRTUAL_METHODS: &'static [u64] = &[0x00000001804b2780, 0x00000001804b3120, 0x000000018045e4c0, 0x00000001804b6a60, 0x000000018047ebd0, 0x00000001804b8d20, 0x00000001804b4ec0, 0x00000001800d3090, 0x00000001800d3090, 0x00000001804b9340, 0x00000001804b9210, 0x000000018047f0e0, 0x000000018045eec0, 0x000000018045efd0, 0x00000001804b8ce0, 0x00000001804b66c0, 0x00000001804b6040, 0x00000001804b80d0, 0x00000001800d3090, 0x00000001800d3090, 0x00000001800d3090, 0x0000000180103e70, 0x00000001804b2920, 0x00000001804b8cb0];

    pub fn new() -> Self {
        Self {
            vtable_rva: Self::VTABLE_RVA,
            field_0x0000: Default::default(),
            field_0x0001: Default::default(),
            field_0x0004: Default::default(),
            field_0x0008: Default::default(),
            field_0x000a: Default::default(),
            field_0x0010: Default::default(),
            field_0x0018: Default::default(),
            field_0x001c: Default::default(),
            field_0x001e: Default::default(),
            field_0x0020: Default::default(),
            field_0x0024: Default::default(),
            field_0x0028: Default::default(),
            field_0x0029: Default::default(),
            field_0x002c: Default::default(),
            field_0x0030: Default::default(),
            field_0x0040: Default::default(),
            field_0x007e: Default::default(),
            field_0x0090: Default::default(),
            field_0x0094: Default::default(),
            field_0x0098: Default::default(),
            field_0x009c: Default::default(),
            field_0x00a0: Default::default(),
            field_0x00a4: Default::default(),
            field_0x00a8: Default::default(),
            field_0x00ac: Default::default(),
            field_0x00b0: Default::default(),
            field_0x00b8: Default::default(),
            field_0x00c4: Default::default(),
            field_0x00e0: Default::default(),
            field_0x00e4: Default::default(),
            field_0x0114: Default::default(),
            field_0x0120: Default::default(),
            field_0x0124: Default::default(),
            field_0x0128: Default::default(),
            field_0x012c: Default::default(),
            field_0x0130: Default::default(),
            field_0x0134: Default::default(),
            field_0x0138: Default::default(),
            field_0x013c: Default::default(),
            field_0x0140: Default::default(),
            field_0x0144: Default::default(),
            field_0x0148: Default::default(),
            field_0x014c: Default::default(),
            field_0x0150: Default::default(),
            field_0x0154: Default::default(),
            field_0x0158: Default::default(),
            field_0x015c: Default::default(),
            field_0x0170: Default::default(),
            field_0x0198: Default::default(),
            field_0x019c: Default::default(),
            field_0x019e: Default::default(),
            field_0x01b0: Default::default(),
            field_0x01e4: Default::default(),
            field_0x0278: Default::default(),
            field_0x02da: Default::default(),
            field_0x02fc: Default::default(),
            field_0x0314: Default::default(),
            field_0x03e0: Default::default(),
            field_0x0420: Default::default(),
        }
    }

    /// Decompiled virtual method #0 @ RVA 0x004b2780 (25 instructions, 5 basic blocks)
    pub fn method_00(&mut self) {
        // Writes to field at offset +0x0000
    }

    /// Decompiled virtual method #1 @ RVA 0x004b3120 (112 instructions, 15 basic blocks)
    pub fn method_01(&mut self) {
        // Writes to field at offset +0x0030
        // Writes to field at offset +0x00c4
    }

    /// Decompiled virtual method #2 @ RVA 0x0045e4c0 (42 instructions, 4 basic blocks)
    pub fn method_02(&mut self) {
        // Writes to field at offset +0x0090
        // Writes to field at offset +0x0094
        // Writes to field at offset +0x0098
        // Writes to field at offset +0x009c
        // Writes to field at offset +0x00a0
        // Writes to field at offset +0x00a4
        // Writes to field at offset +0x00a8
        // Writes to field at offset +0x00ac
        // Writes to field at offset +0x0140
        // Writes to field at offset +0x0144
        // Writes to field at offset +0x0148
        // Writes to field at offset +0x014c
        // Writes to field at offset +0x0150
        // Writes to field at offset +0x0154
        // Writes to field at offset +0x0158
        // Writes to field at offset +0x015c
    }

    /// Decompiled virtual method #3 @ RVA 0x004b6a60 (104 instructions, 18 basic blocks)
    pub fn method_03(&mut self) {
        // Writes to field at offset +0x0000
        // Writes to field at offset +0x0001
        // Writes to field at offset +0x0004
        // Writes to field at offset +0x00b0
    }

    /// Decompiled virtual method #4 @ RVA 0x0047ebd0 (45 instructions, 12 basic blocks)
    pub fn method_04(&mut self) {
        // Writes to field at offset +0x0004
        // Writes to field at offset +0x02da
        // Writes to field at offset +0x0314
        // Writes to field at offset +0x03e0
    }

    /// Decompiled virtual method #5 @ RVA 0x004b8d20 (104 instructions, 18 basic blocks)
    pub fn method_05(&mut self) {
        // Writes to field at offset +0x0028
    }

    /// Decompiled virtual method #6 @ RVA 0x004b4ec0 (103 instructions, 17 basic blocks)
    pub fn method_06(&mut self) {
    }

    /// Decompiled virtual method #7 @ RVA 0x000d3090 (1 instructions, 1 basic blocks)
    pub fn method_07(&mut self) {
    }

    /// Decompiled virtual method #8 @ RVA 0x000d3090 (1 instructions, 1 basic blocks)
    pub fn method_08(&mut self) {
    }

    /// Decompiled virtual method #9 @ RVA 0x004b9340 (116 instructions, 17 basic blocks)
    pub fn method_09(&mut self) {
        // Writes to field at offset +0x0000
        // Writes to field at offset +0x0000
        // Writes to field at offset +0x0001
        // Writes to field at offset +0x0420
    }

    /// Decompiled virtual method #10 @ RVA 0x004b9210 (63 instructions, 13 basic blocks)
    pub fn method_10(&mut self) {
        // Writes to field at offset +0x0029
        // Writes to field at offset +0x002c
        // Writes to field at offset +0x0030
        // Writes to field at offset +0x0114
    }

    /// Decompiled virtual method #11 @ RVA 0x0047f0e0 (15 instructions, 4 basic blocks)
    pub fn method_11(&mut self) {
    }

    /// Decompiled virtual method #12 @ RVA 0x0045eec0 (67 instructions, 22 basic blocks)
    pub fn method_12(&mut self) {
        // Writes to field at offset +0x001c
        // Writes to field at offset +0x0040
        // Writes to field at offset +0x0090
        // Writes to field at offset +0x00b0
    }

    /// Decompiled virtual method #13 @ RVA 0x0045efd0 (27 instructions, 5 basic blocks)
    pub fn method_13(&mut self) {
        // Writes to field at offset +0x0090
        // Writes to field at offset +0x00b0
    }

    /// Decompiled virtual method #14 @ RVA 0x004b8ce0 (114 instructions, 20 basic blocks)
    pub fn method_14(&mut self) {
        // Writes to field at offset +0x0028
    }

    /// Decompiled virtual method #15 @ RVA 0x004b66c0 (44 instructions, 12 basic blocks)
    pub fn method_15(&mut self) {
        // Writes to field at offset +0x0000
        // Writes to field at offset +0x0004
        // Writes to field at offset +0x0008
        // Writes to field at offset +0x000a
        // Writes to field at offset +0x00c4
        // Writes to field at offset +0x01e4
        // Writes to field at offset +0x02fc
    }

    /// Decompiled virtual method #16 @ RVA 0x004b6040 (54 instructions, 12 basic blocks)
    pub fn method_16(&mut self) {
    }

    /// Decompiled virtual method #17 @ RVA 0x004b80d0 (38 instructions, 8 basic blocks)
    pub fn method_17(&mut self) {
    }

    /// Decompiled virtual method #18 @ RVA 0x000d3090 (1 instructions, 1 basic blocks)
    pub fn method_18(&mut self) {
    }

    /// Decompiled virtual method #19 @ RVA 0x000d3090 (1 instructions, 1 basic blocks)
    pub fn method_19(&mut self) {
    }

    /// Decompiled virtual method #20 @ RVA 0x000d3090 (1 instructions, 1 basic blocks)
    pub fn method_20(&mut self) {
    }

    /// Decompiled virtual method #21 @ RVA 0x00103e70 (2 instructions, 1 basic blocks)
    pub fn method_21(&mut self) {
    }

    /// Decompiled virtual method #22 @ RVA 0x004b2920 (22 instructions, 3 basic blocks)
    pub fn method_22(&mut self) {
        // Writes to field at offset +0x01b0
    }

    /// Decompiled virtual method #23 @ RVA 0x004b8cb0 (6 instructions, 1 basic blocks)
    pub fn method_23(&mut self) {
        // Writes to field at offset +0x03e0
        // Writes to field at offset +0x0420
    }

}

impl CcClass for ccGobjEnemyHardshellL {
    fn class_name(&self) -> &'static str { Self::CLASS_NAME }
    fn vtable_rva(&self) -> u32 { Self::VTABLE_RVA }
    fn virtual_method_count(&self) -> usize { Self::METHOD_COUNT }
    fn base_classes(&self) -> &'static [&'static str] { Self::BASE_CLASSES }
}

/// Decompiled CC2 class: `ccGobjEnemyHardshellLL` (VTable RVA: 0x008bd388, 24 virtual methods, 59 recovered fields)
/// Inherits: ccGobjEnemyHardshell, ccGobjEnemy, ccGobjBattle, ccGameObj
#[derive(Debug, Clone, Default)]
pub struct ccGobjEnemyHardshellLL {
    pub vtable_rva: u32,
    pub field_0x0000: u64,
    pub field_0x0001: u32,
    pub field_0x0004: u8,
    pub field_0x0008: u16,
    pub field_0x000a: u16,
    pub field_0x0010: u64,
    pub field_0x0018: u64,
    pub field_0x001c: u8,
    pub field_0x001e: u16,
    pub field_0x0020: u32,
    pub field_0x0024: u16,
    pub field_0x0028: u8,
    pub field_0x0029: u8,
    pub field_0x002c: f32,
    pub field_0x0030: u32,
    pub field_0x0040: u32,
    pub field_0x007e: u16,
    pub field_0x0090: u32,
    pub field_0x0094: u32,
    pub field_0x0098: u64,
    pub field_0x009c: u32,
    pub field_0x00a0: u32,
    pub field_0x00a4: u32,
    pub field_0x00a8: u64,
    pub field_0x00ac: u32,
    pub field_0x00b0: u64,
    pub field_0x00b8: u64,
    pub field_0x00c4: u32,
    pub field_0x00e0: f32,
    pub field_0x00e4: f32,
    pub field_0x0114: u32,
    pub field_0x0120: u32,
    pub field_0x0124: u32,
    pub field_0x0128: u32,
    pub field_0x012c: u32,
    pub field_0x0130: u32,
    pub field_0x0134: u32,
    pub field_0x0138: u32,
    pub field_0x013c: u32,
    pub field_0x0140: u32,
    pub field_0x0144: u32,
    pub field_0x0148: u32,
    pub field_0x014c: u32,
    pub field_0x0150: u32,
    pub field_0x0154: u32,
    pub field_0x0158: u32,
    pub field_0x015c: u32,
    pub field_0x0170: u32,
    pub field_0x0198: u32,
    pub field_0x019c: u16,
    pub field_0x019e: u16,
    pub field_0x01b0: u64,
    pub field_0x01e4: u16,
    pub field_0x0278: u32,
    pub field_0x02da: u16,
    pub field_0x02fc: u8,
    pub field_0x0314: u16,
    pub field_0x03e0: u32,
    pub field_0x0420: u64,
}

impl ccGobjEnemyHardshellLL {
    pub const CLASS_NAME: &'static str = "ccGobjEnemyHardshellLL";
    pub const VTABLE_RVA: u32 = 0x008bd388;
    pub const METHOD_COUNT: usize = 24;
    pub const BASE_CLASSES: &'static [&'static str] = &["ccGobjEnemyHardshell", "ccGobjEnemy", "ccGobjBattle", "ccGameObj"];
    pub const VIRTUAL_METHODS: &'static [u64] = &[0x00000001804b2780, 0x00000001804b3120, 0x000000018045e4c0, 0x00000001804b6a60, 0x000000018047ebd0, 0x00000001804b8d20, 0x00000001804b4ec0, 0x00000001800d3090, 0x00000001800d3090, 0x00000001804b9340, 0x00000001804b9210, 0x000000018047f0e0, 0x000000018045eec0, 0x000000018045efd0, 0x00000001804b8ce0, 0x00000001804b66c0, 0x00000001804b6040, 0x00000001804b80d0, 0x00000001800d3090, 0x00000001800d3090, 0x00000001800d3090, 0x0000000180103e70, 0x00000001804b2920, 0x00000001804b8cb0];

    pub fn new() -> Self {
        Self {
            vtable_rva: Self::VTABLE_RVA,
            field_0x0000: Default::default(),
            field_0x0001: Default::default(),
            field_0x0004: Default::default(),
            field_0x0008: Default::default(),
            field_0x000a: Default::default(),
            field_0x0010: Default::default(),
            field_0x0018: Default::default(),
            field_0x001c: Default::default(),
            field_0x001e: Default::default(),
            field_0x0020: Default::default(),
            field_0x0024: Default::default(),
            field_0x0028: Default::default(),
            field_0x0029: Default::default(),
            field_0x002c: Default::default(),
            field_0x0030: Default::default(),
            field_0x0040: Default::default(),
            field_0x007e: Default::default(),
            field_0x0090: Default::default(),
            field_0x0094: Default::default(),
            field_0x0098: Default::default(),
            field_0x009c: Default::default(),
            field_0x00a0: Default::default(),
            field_0x00a4: Default::default(),
            field_0x00a8: Default::default(),
            field_0x00ac: Default::default(),
            field_0x00b0: Default::default(),
            field_0x00b8: Default::default(),
            field_0x00c4: Default::default(),
            field_0x00e0: Default::default(),
            field_0x00e4: Default::default(),
            field_0x0114: Default::default(),
            field_0x0120: Default::default(),
            field_0x0124: Default::default(),
            field_0x0128: Default::default(),
            field_0x012c: Default::default(),
            field_0x0130: Default::default(),
            field_0x0134: Default::default(),
            field_0x0138: Default::default(),
            field_0x013c: Default::default(),
            field_0x0140: Default::default(),
            field_0x0144: Default::default(),
            field_0x0148: Default::default(),
            field_0x014c: Default::default(),
            field_0x0150: Default::default(),
            field_0x0154: Default::default(),
            field_0x0158: Default::default(),
            field_0x015c: Default::default(),
            field_0x0170: Default::default(),
            field_0x0198: Default::default(),
            field_0x019c: Default::default(),
            field_0x019e: Default::default(),
            field_0x01b0: Default::default(),
            field_0x01e4: Default::default(),
            field_0x0278: Default::default(),
            field_0x02da: Default::default(),
            field_0x02fc: Default::default(),
            field_0x0314: Default::default(),
            field_0x03e0: Default::default(),
            field_0x0420: Default::default(),
        }
    }

    /// Decompiled virtual method #0 @ RVA 0x004b2780 (25 instructions, 5 basic blocks)
    pub fn method_00(&mut self) {
        // Writes to field at offset +0x0000
    }

    /// Decompiled virtual method #1 @ RVA 0x004b3120 (112 instructions, 15 basic blocks)
    pub fn method_01(&mut self) {
        // Writes to field at offset +0x0030
        // Writes to field at offset +0x00c4
    }

    /// Decompiled virtual method #2 @ RVA 0x0045e4c0 (42 instructions, 4 basic blocks)
    pub fn method_02(&mut self) {
        // Writes to field at offset +0x0090
        // Writes to field at offset +0x0094
        // Writes to field at offset +0x0098
        // Writes to field at offset +0x009c
        // Writes to field at offset +0x00a0
        // Writes to field at offset +0x00a4
        // Writes to field at offset +0x00a8
        // Writes to field at offset +0x00ac
        // Writes to field at offset +0x0140
        // Writes to field at offset +0x0144
        // Writes to field at offset +0x0148
        // Writes to field at offset +0x014c
        // Writes to field at offset +0x0150
        // Writes to field at offset +0x0154
        // Writes to field at offset +0x0158
        // Writes to field at offset +0x015c
    }

    /// Decompiled virtual method #3 @ RVA 0x004b6a60 (104 instructions, 18 basic blocks)
    pub fn method_03(&mut self) {
        // Writes to field at offset +0x0000
        // Writes to field at offset +0x0001
        // Writes to field at offset +0x0004
        // Writes to field at offset +0x00b0
    }

    /// Decompiled virtual method #4 @ RVA 0x0047ebd0 (45 instructions, 12 basic blocks)
    pub fn method_04(&mut self) {
        // Writes to field at offset +0x0004
        // Writes to field at offset +0x02da
        // Writes to field at offset +0x0314
        // Writes to field at offset +0x03e0
    }

    /// Decompiled virtual method #5 @ RVA 0x004b8d20 (104 instructions, 18 basic blocks)
    pub fn method_05(&mut self) {
        // Writes to field at offset +0x0028
    }

    /// Decompiled virtual method #6 @ RVA 0x004b4ec0 (103 instructions, 17 basic blocks)
    pub fn method_06(&mut self) {
    }

    /// Decompiled virtual method #7 @ RVA 0x000d3090 (1 instructions, 1 basic blocks)
    pub fn method_07(&mut self) {
    }

    /// Decompiled virtual method #8 @ RVA 0x000d3090 (1 instructions, 1 basic blocks)
    pub fn method_08(&mut self) {
    }

    /// Decompiled virtual method #9 @ RVA 0x004b9340 (116 instructions, 17 basic blocks)
    pub fn method_09(&mut self) {
        // Writes to field at offset +0x0000
        // Writes to field at offset +0x0000
        // Writes to field at offset +0x0001
        // Writes to field at offset +0x0420
    }

    /// Decompiled virtual method #10 @ RVA 0x004b9210 (63 instructions, 13 basic blocks)
    pub fn method_10(&mut self) {
        // Writes to field at offset +0x0029
        // Writes to field at offset +0x002c
        // Writes to field at offset +0x0030
        // Writes to field at offset +0x0114
    }

    /// Decompiled virtual method #11 @ RVA 0x0047f0e0 (15 instructions, 4 basic blocks)
    pub fn method_11(&mut self) {
    }

    /// Decompiled virtual method #12 @ RVA 0x0045eec0 (67 instructions, 22 basic blocks)
    pub fn method_12(&mut self) {
        // Writes to field at offset +0x001c
        // Writes to field at offset +0x0040
        // Writes to field at offset +0x0090
        // Writes to field at offset +0x00b0
    }

    /// Decompiled virtual method #13 @ RVA 0x0045efd0 (27 instructions, 5 basic blocks)
    pub fn method_13(&mut self) {
        // Writes to field at offset +0x0090
        // Writes to field at offset +0x00b0
    }

    /// Decompiled virtual method #14 @ RVA 0x004b8ce0 (114 instructions, 20 basic blocks)
    pub fn method_14(&mut self) {
        // Writes to field at offset +0x0028
    }

    /// Decompiled virtual method #15 @ RVA 0x004b66c0 (44 instructions, 12 basic blocks)
    pub fn method_15(&mut self) {
        // Writes to field at offset +0x0000
        // Writes to field at offset +0x0004
        // Writes to field at offset +0x0008
        // Writes to field at offset +0x000a
        // Writes to field at offset +0x00c4
        // Writes to field at offset +0x01e4
        // Writes to field at offset +0x02fc
    }

    /// Decompiled virtual method #16 @ RVA 0x004b6040 (54 instructions, 12 basic blocks)
    pub fn method_16(&mut self) {
    }

    /// Decompiled virtual method #17 @ RVA 0x004b80d0 (38 instructions, 8 basic blocks)
    pub fn method_17(&mut self) {
    }

    /// Decompiled virtual method #18 @ RVA 0x000d3090 (1 instructions, 1 basic blocks)
    pub fn method_18(&mut self) {
    }

    /// Decompiled virtual method #19 @ RVA 0x000d3090 (1 instructions, 1 basic blocks)
    pub fn method_19(&mut self) {
    }

    /// Decompiled virtual method #20 @ RVA 0x000d3090 (1 instructions, 1 basic blocks)
    pub fn method_20(&mut self) {
    }

    /// Decompiled virtual method #21 @ RVA 0x00103e70 (2 instructions, 1 basic blocks)
    pub fn method_21(&mut self) {
    }

    /// Decompiled virtual method #22 @ RVA 0x004b2920 (22 instructions, 3 basic blocks)
    pub fn method_22(&mut self) {
        // Writes to field at offset +0x01b0
    }

    /// Decompiled virtual method #23 @ RVA 0x004b8cb0 (6 instructions, 1 basic blocks)
    pub fn method_23(&mut self) {
        // Writes to field at offset +0x03e0
        // Writes to field at offset +0x0420
    }

}

impl CcClass for ccGobjEnemyHardshellLL {
    fn class_name(&self) -> &'static str { Self::CLASS_NAME }
    fn vtable_rva(&self) -> u32 { Self::VTABLE_RVA }
    fn virtual_method_count(&self) -> usize { Self::METHOD_COUNT }
    fn base_classes(&self) -> &'static [&'static str] { Self::BASE_CLASSES }
}

/// Decompiled CC2 class: `ccGobjEnemyHardshellM` (VTable RVA: 0x008bd518, 24 virtual methods, 59 recovered fields)
/// Inherits: ccGobjEnemyHardshell, ccGobjEnemy, ccGobjBattle, ccGameObj
#[derive(Debug, Clone, Default)]
pub struct ccGobjEnemyHardshellM {
    pub vtable_rva: u32,
    pub field_0x0000: u64,
    pub field_0x0001: u32,
    pub field_0x0004: u8,
    pub field_0x0008: u16,
    pub field_0x000a: u16,
    pub field_0x0010: u64,
    pub field_0x0018: u64,
    pub field_0x001c: u8,
    pub field_0x001e: u16,
    pub field_0x0020: u32,
    pub field_0x0024: u16,
    pub field_0x0028: u8,
    pub field_0x0029: u8,
    pub field_0x002c: f32,
    pub field_0x0030: u32,
    pub field_0x0040: u32,
    pub field_0x007e: u16,
    pub field_0x0090: u32,
    pub field_0x0094: u32,
    pub field_0x0098: u64,
    pub field_0x009c: u32,
    pub field_0x00a0: u32,
    pub field_0x00a4: u32,
    pub field_0x00a8: u64,
    pub field_0x00ac: u32,
    pub field_0x00b0: u64,
    pub field_0x00b8: u64,
    pub field_0x00c4: u32,
    pub field_0x00e0: f32,
    pub field_0x00e4: f32,
    pub field_0x0114: u32,
    pub field_0x0120: u32,
    pub field_0x0124: u32,
    pub field_0x0128: u32,
    pub field_0x012c: u32,
    pub field_0x0130: u32,
    pub field_0x0134: u32,
    pub field_0x0138: u32,
    pub field_0x013c: u32,
    pub field_0x0140: u32,
    pub field_0x0144: u32,
    pub field_0x0148: u32,
    pub field_0x014c: u32,
    pub field_0x0150: u32,
    pub field_0x0154: u32,
    pub field_0x0158: u32,
    pub field_0x015c: u32,
    pub field_0x0170: u32,
    pub field_0x0198: u32,
    pub field_0x019c: u16,
    pub field_0x019e: u16,
    pub field_0x01b0: u64,
    pub field_0x01e4: u16,
    pub field_0x0278: u32,
    pub field_0x02da: u16,
    pub field_0x02fc: u8,
    pub field_0x0314: u16,
    pub field_0x03e0: u32,
    pub field_0x0420: u64,
}

impl ccGobjEnemyHardshellM {
    pub const CLASS_NAME: &'static str = "ccGobjEnemyHardshellM";
    pub const VTABLE_RVA: u32 = 0x008bd518;
    pub const METHOD_COUNT: usize = 24;
    pub const BASE_CLASSES: &'static [&'static str] = &["ccGobjEnemyHardshell", "ccGobjEnemy", "ccGobjBattle", "ccGameObj"];
    pub const VIRTUAL_METHODS: &'static [u64] = &[0x00000001804b2780, 0x00000001804b3120, 0x000000018045e4c0, 0x00000001804b6a60, 0x000000018047ebd0, 0x00000001804b8d20, 0x00000001804b4ec0, 0x00000001800d3090, 0x00000001800d3090, 0x00000001804b9340, 0x00000001804b9210, 0x000000018047f0e0, 0x000000018045eec0, 0x000000018045efd0, 0x00000001804b8ce0, 0x00000001804b66c0, 0x00000001804b6040, 0x00000001804b80d0, 0x00000001800d3090, 0x00000001800d3090, 0x00000001800d3090, 0x0000000180103e70, 0x00000001804b2920, 0x00000001804b8cb0];

    pub fn new() -> Self {
        Self {
            vtable_rva: Self::VTABLE_RVA,
            field_0x0000: Default::default(),
            field_0x0001: Default::default(),
            field_0x0004: Default::default(),
            field_0x0008: Default::default(),
            field_0x000a: Default::default(),
            field_0x0010: Default::default(),
            field_0x0018: Default::default(),
            field_0x001c: Default::default(),
            field_0x001e: Default::default(),
            field_0x0020: Default::default(),
            field_0x0024: Default::default(),
            field_0x0028: Default::default(),
            field_0x0029: Default::default(),
            field_0x002c: Default::default(),
            field_0x0030: Default::default(),
            field_0x0040: Default::default(),
            field_0x007e: Default::default(),
            field_0x0090: Default::default(),
            field_0x0094: Default::default(),
            field_0x0098: Default::default(),
            field_0x009c: Default::default(),
            field_0x00a0: Default::default(),
            field_0x00a4: Default::default(),
            field_0x00a8: Default::default(),
            field_0x00ac: Default::default(),
            field_0x00b0: Default::default(),
            field_0x00b8: Default::default(),
            field_0x00c4: Default::default(),
            field_0x00e0: Default::default(),
            field_0x00e4: Default::default(),
            field_0x0114: Default::default(),
            field_0x0120: Default::default(),
            field_0x0124: Default::default(),
            field_0x0128: Default::default(),
            field_0x012c: Default::default(),
            field_0x0130: Default::default(),
            field_0x0134: Default::default(),
            field_0x0138: Default::default(),
            field_0x013c: Default::default(),
            field_0x0140: Default::default(),
            field_0x0144: Default::default(),
            field_0x0148: Default::default(),
            field_0x014c: Default::default(),
            field_0x0150: Default::default(),
            field_0x0154: Default::default(),
            field_0x0158: Default::default(),
            field_0x015c: Default::default(),
            field_0x0170: Default::default(),
            field_0x0198: Default::default(),
            field_0x019c: Default::default(),
            field_0x019e: Default::default(),
            field_0x01b0: Default::default(),
            field_0x01e4: Default::default(),
            field_0x0278: Default::default(),
            field_0x02da: Default::default(),
            field_0x02fc: Default::default(),
            field_0x0314: Default::default(),
            field_0x03e0: Default::default(),
            field_0x0420: Default::default(),
        }
    }

    /// Decompiled virtual method #0 @ RVA 0x004b2780 (25 instructions, 5 basic blocks)
    pub fn method_00(&mut self) {
        // Writes to field at offset +0x0000
    }

    /// Decompiled virtual method #1 @ RVA 0x004b3120 (112 instructions, 15 basic blocks)
    pub fn method_01(&mut self) {
        // Writes to field at offset +0x0030
        // Writes to field at offset +0x00c4
    }

    /// Decompiled virtual method #2 @ RVA 0x0045e4c0 (42 instructions, 4 basic blocks)
    pub fn method_02(&mut self) {
        // Writes to field at offset +0x0090
        // Writes to field at offset +0x0094
        // Writes to field at offset +0x0098
        // Writes to field at offset +0x009c
        // Writes to field at offset +0x00a0
        // Writes to field at offset +0x00a4
        // Writes to field at offset +0x00a8
        // Writes to field at offset +0x00ac
        // Writes to field at offset +0x0140
        // Writes to field at offset +0x0144
        // Writes to field at offset +0x0148
        // Writes to field at offset +0x014c
        // Writes to field at offset +0x0150
        // Writes to field at offset +0x0154
        // Writes to field at offset +0x0158
        // Writes to field at offset +0x015c
    }

    /// Decompiled virtual method #3 @ RVA 0x004b6a60 (104 instructions, 18 basic blocks)
    pub fn method_03(&mut self) {
        // Writes to field at offset +0x0000
        // Writes to field at offset +0x0001
        // Writes to field at offset +0x0004
        // Writes to field at offset +0x00b0
    }

    /// Decompiled virtual method #4 @ RVA 0x0047ebd0 (45 instructions, 12 basic blocks)
    pub fn method_04(&mut self) {
        // Writes to field at offset +0x0004
        // Writes to field at offset +0x02da
        // Writes to field at offset +0x0314
        // Writes to field at offset +0x03e0
    }

    /// Decompiled virtual method #5 @ RVA 0x004b8d20 (104 instructions, 18 basic blocks)
    pub fn method_05(&mut self) {
        // Writes to field at offset +0x0028
    }

    /// Decompiled virtual method #6 @ RVA 0x004b4ec0 (103 instructions, 17 basic blocks)
    pub fn method_06(&mut self) {
    }

    /// Decompiled virtual method #7 @ RVA 0x000d3090 (1 instructions, 1 basic blocks)
    pub fn method_07(&mut self) {
    }

    /// Decompiled virtual method #8 @ RVA 0x000d3090 (1 instructions, 1 basic blocks)
    pub fn method_08(&mut self) {
    }

    /// Decompiled virtual method #9 @ RVA 0x004b9340 (116 instructions, 17 basic blocks)
    pub fn method_09(&mut self) {
        // Writes to field at offset +0x0000
        // Writes to field at offset +0x0000
        // Writes to field at offset +0x0001
        // Writes to field at offset +0x0420
    }

    /// Decompiled virtual method #10 @ RVA 0x004b9210 (63 instructions, 13 basic blocks)
    pub fn method_10(&mut self) {
        // Writes to field at offset +0x0029
        // Writes to field at offset +0x002c
        // Writes to field at offset +0x0030
        // Writes to field at offset +0x0114
    }

    /// Decompiled virtual method #11 @ RVA 0x0047f0e0 (15 instructions, 4 basic blocks)
    pub fn method_11(&mut self) {
    }

    /// Decompiled virtual method #12 @ RVA 0x0045eec0 (67 instructions, 22 basic blocks)
    pub fn method_12(&mut self) {
        // Writes to field at offset +0x001c
        // Writes to field at offset +0x0040
        // Writes to field at offset +0x0090
        // Writes to field at offset +0x00b0
    }

    /// Decompiled virtual method #13 @ RVA 0x0045efd0 (27 instructions, 5 basic blocks)
    pub fn method_13(&mut self) {
        // Writes to field at offset +0x0090
        // Writes to field at offset +0x00b0
    }

    /// Decompiled virtual method #14 @ RVA 0x004b8ce0 (114 instructions, 20 basic blocks)
    pub fn method_14(&mut self) {
        // Writes to field at offset +0x0028
    }

    /// Decompiled virtual method #15 @ RVA 0x004b66c0 (44 instructions, 12 basic blocks)
    pub fn method_15(&mut self) {
        // Writes to field at offset +0x0000
        // Writes to field at offset +0x0004
        // Writes to field at offset +0x0008
        // Writes to field at offset +0x000a
        // Writes to field at offset +0x00c4
        // Writes to field at offset +0x01e4
        // Writes to field at offset +0x02fc
    }

    /// Decompiled virtual method #16 @ RVA 0x004b6040 (54 instructions, 12 basic blocks)
    pub fn method_16(&mut self) {
    }

    /// Decompiled virtual method #17 @ RVA 0x004b80d0 (38 instructions, 8 basic blocks)
    pub fn method_17(&mut self) {
    }

    /// Decompiled virtual method #18 @ RVA 0x000d3090 (1 instructions, 1 basic blocks)
    pub fn method_18(&mut self) {
    }

    /// Decompiled virtual method #19 @ RVA 0x000d3090 (1 instructions, 1 basic blocks)
    pub fn method_19(&mut self) {
    }

    /// Decompiled virtual method #20 @ RVA 0x000d3090 (1 instructions, 1 basic blocks)
    pub fn method_20(&mut self) {
    }

    /// Decompiled virtual method #21 @ RVA 0x00103e70 (2 instructions, 1 basic blocks)
    pub fn method_21(&mut self) {
    }

    /// Decompiled virtual method #22 @ RVA 0x004b2920 (22 instructions, 3 basic blocks)
    pub fn method_22(&mut self) {
        // Writes to field at offset +0x01b0
    }

    /// Decompiled virtual method #23 @ RVA 0x004b8cb0 (6 instructions, 1 basic blocks)
    pub fn method_23(&mut self) {
        // Writes to field at offset +0x03e0
        // Writes to field at offset +0x0420
    }

}

impl CcClass for ccGobjEnemyHardshellM {
    fn class_name(&self) -> &'static str { Self::CLASS_NAME }
    fn vtable_rva(&self) -> u32 { Self::VTABLE_RVA }
    fn virtual_method_count(&self) -> usize { Self::METHOD_COUNT }
    fn base_classes(&self) -> &'static [&'static str] { Self::BASE_CLASSES }
}

/// Decompiled CC2 class: `ccGobjEnemyHardshellS` (VTable RVA: 0x008bd450, 24 virtual methods, 59 recovered fields)
/// Inherits: ccGobjEnemyHardshell, ccGobjEnemy, ccGobjBattle, ccGameObj
#[derive(Debug, Clone, Default)]
pub struct ccGobjEnemyHardshellS {
    pub vtable_rva: u32,
    pub field_0x0000: u64,
    pub field_0x0001: u32,
    pub field_0x0004: u8,
    pub field_0x0008: u16,
    pub field_0x000a: u16,
    pub field_0x0010: u64,
    pub field_0x0018: u64,
    pub field_0x001c: u8,
    pub field_0x001e: u16,
    pub field_0x0020: u32,
    pub field_0x0024: u16,
    pub field_0x0028: u8,
    pub field_0x0029: u8,
    pub field_0x002c: f32,
    pub field_0x0030: u32,
    pub field_0x0040: u32,
    pub field_0x007e: u16,
    pub field_0x0090: u32,
    pub field_0x0094: u32,
    pub field_0x0098: u64,
    pub field_0x009c: u32,
    pub field_0x00a0: u32,
    pub field_0x00a4: u32,
    pub field_0x00a8: u64,
    pub field_0x00ac: u32,
    pub field_0x00b0: u64,
    pub field_0x00b8: u64,
    pub field_0x00c4: u32,
    pub field_0x00e0: f32,
    pub field_0x00e4: f32,
    pub field_0x0114: u32,
    pub field_0x0120: u32,
    pub field_0x0124: u32,
    pub field_0x0128: u32,
    pub field_0x012c: u32,
    pub field_0x0130: u32,
    pub field_0x0134: u32,
    pub field_0x0138: u32,
    pub field_0x013c: u32,
    pub field_0x0140: u32,
    pub field_0x0144: u32,
    pub field_0x0148: u32,
    pub field_0x014c: u32,
    pub field_0x0150: u32,
    pub field_0x0154: u32,
    pub field_0x0158: u32,
    pub field_0x015c: u32,
    pub field_0x0170: u32,
    pub field_0x0198: u32,
    pub field_0x019c: u16,
    pub field_0x019e: u16,
    pub field_0x01b0: u64,
    pub field_0x01e4: u16,
    pub field_0x0278: u32,
    pub field_0x02da: u16,
    pub field_0x02fc: u8,
    pub field_0x0314: u16,
    pub field_0x03e0: u32,
    pub field_0x0420: u64,
}

impl ccGobjEnemyHardshellS {
    pub const CLASS_NAME: &'static str = "ccGobjEnemyHardshellS";
    pub const VTABLE_RVA: u32 = 0x008bd450;
    pub const METHOD_COUNT: usize = 24;
    pub const BASE_CLASSES: &'static [&'static str] = &["ccGobjEnemyHardshell", "ccGobjEnemy", "ccGobjBattle", "ccGameObj"];
    pub const VIRTUAL_METHODS: &'static [u64] = &[0x00000001804b2780, 0x00000001804b3120, 0x000000018045e4c0, 0x00000001804b6a60, 0x000000018047ebd0, 0x00000001804b8d20, 0x00000001804b4ec0, 0x00000001800d3090, 0x00000001800d3090, 0x00000001804b9340, 0x00000001804b9210, 0x000000018047f0e0, 0x000000018045eec0, 0x000000018045efd0, 0x00000001804b8ce0, 0x00000001804b66c0, 0x00000001804b6040, 0x00000001804b80d0, 0x00000001800d3090, 0x00000001800d3090, 0x00000001800d3090, 0x0000000180103e70, 0x00000001804b2920, 0x00000001804b8cb0];

    pub fn new() -> Self {
        Self {
            vtable_rva: Self::VTABLE_RVA,
            field_0x0000: Default::default(),
            field_0x0001: Default::default(),
            field_0x0004: Default::default(),
            field_0x0008: Default::default(),
            field_0x000a: Default::default(),
            field_0x0010: Default::default(),
            field_0x0018: Default::default(),
            field_0x001c: Default::default(),
            field_0x001e: Default::default(),
            field_0x0020: Default::default(),
            field_0x0024: Default::default(),
            field_0x0028: Default::default(),
            field_0x0029: Default::default(),
            field_0x002c: Default::default(),
            field_0x0030: Default::default(),
            field_0x0040: Default::default(),
            field_0x007e: Default::default(),
            field_0x0090: Default::default(),
            field_0x0094: Default::default(),
            field_0x0098: Default::default(),
            field_0x009c: Default::default(),
            field_0x00a0: Default::default(),
            field_0x00a4: Default::default(),
            field_0x00a8: Default::default(),
            field_0x00ac: Default::default(),
            field_0x00b0: Default::default(),
            field_0x00b8: Default::default(),
            field_0x00c4: Default::default(),
            field_0x00e0: Default::default(),
            field_0x00e4: Default::default(),
            field_0x0114: Default::default(),
            field_0x0120: Default::default(),
            field_0x0124: Default::default(),
            field_0x0128: Default::default(),
            field_0x012c: Default::default(),
            field_0x0130: Default::default(),
            field_0x0134: Default::default(),
            field_0x0138: Default::default(),
            field_0x013c: Default::default(),
            field_0x0140: Default::default(),
            field_0x0144: Default::default(),
            field_0x0148: Default::default(),
            field_0x014c: Default::default(),
            field_0x0150: Default::default(),
            field_0x0154: Default::default(),
            field_0x0158: Default::default(),
            field_0x015c: Default::default(),
            field_0x0170: Default::default(),
            field_0x0198: Default::default(),
            field_0x019c: Default::default(),
            field_0x019e: Default::default(),
            field_0x01b0: Default::default(),
            field_0x01e4: Default::default(),
            field_0x0278: Default::default(),
            field_0x02da: Default::default(),
            field_0x02fc: Default::default(),
            field_0x0314: Default::default(),
            field_0x03e0: Default::default(),
            field_0x0420: Default::default(),
        }
    }

    /// Decompiled virtual method #0 @ RVA 0x004b2780 (25 instructions, 5 basic blocks)
    pub fn method_00(&mut self) {
        // Writes to field at offset +0x0000
    }

    /// Decompiled virtual method #1 @ RVA 0x004b3120 (112 instructions, 15 basic blocks)
    pub fn method_01(&mut self) {
        // Writes to field at offset +0x0030
        // Writes to field at offset +0x00c4
    }

    /// Decompiled virtual method #2 @ RVA 0x0045e4c0 (42 instructions, 4 basic blocks)
    pub fn method_02(&mut self) {
        // Writes to field at offset +0x0090
        // Writes to field at offset +0x0094
        // Writes to field at offset +0x0098
        // Writes to field at offset +0x009c
        // Writes to field at offset +0x00a0
        // Writes to field at offset +0x00a4
        // Writes to field at offset +0x00a8
        // Writes to field at offset +0x00ac
        // Writes to field at offset +0x0140
        // Writes to field at offset +0x0144
        // Writes to field at offset +0x0148
        // Writes to field at offset +0x014c
        // Writes to field at offset +0x0150
        // Writes to field at offset +0x0154
        // Writes to field at offset +0x0158
        // Writes to field at offset +0x015c
    }

    /// Decompiled virtual method #3 @ RVA 0x004b6a60 (104 instructions, 18 basic blocks)
    pub fn method_03(&mut self) {
        // Writes to field at offset +0x0000
        // Writes to field at offset +0x0001
        // Writes to field at offset +0x0004
        // Writes to field at offset +0x00b0
    }

    /// Decompiled virtual method #4 @ RVA 0x0047ebd0 (45 instructions, 12 basic blocks)
    pub fn method_04(&mut self) {
        // Writes to field at offset +0x0004
        // Writes to field at offset +0x02da
        // Writes to field at offset +0x0314
        // Writes to field at offset +0x03e0
    }

    /// Decompiled virtual method #5 @ RVA 0x004b8d20 (104 instructions, 18 basic blocks)
    pub fn method_05(&mut self) {
        // Writes to field at offset +0x0028
    }

    /// Decompiled virtual method #6 @ RVA 0x004b4ec0 (103 instructions, 17 basic blocks)
    pub fn method_06(&mut self) {
    }

    /// Decompiled virtual method #7 @ RVA 0x000d3090 (1 instructions, 1 basic blocks)
    pub fn method_07(&mut self) {
    }

    /// Decompiled virtual method #8 @ RVA 0x000d3090 (1 instructions, 1 basic blocks)
    pub fn method_08(&mut self) {
    }

    /// Decompiled virtual method #9 @ RVA 0x004b9340 (116 instructions, 17 basic blocks)
    pub fn method_09(&mut self) {
        // Writes to field at offset +0x0000
        // Writes to field at offset +0x0000
        // Writes to field at offset +0x0001
        // Writes to field at offset +0x0420
    }

    /// Decompiled virtual method #10 @ RVA 0x004b9210 (63 instructions, 13 basic blocks)
    pub fn method_10(&mut self) {
        // Writes to field at offset +0x0029
        // Writes to field at offset +0x002c
        // Writes to field at offset +0x0030
        // Writes to field at offset +0x0114
    }

    /// Decompiled virtual method #11 @ RVA 0x0047f0e0 (15 instructions, 4 basic blocks)
    pub fn method_11(&mut self) {
    }

    /// Decompiled virtual method #12 @ RVA 0x0045eec0 (67 instructions, 22 basic blocks)
    pub fn method_12(&mut self) {
        // Writes to field at offset +0x001c
        // Writes to field at offset +0x0040
        // Writes to field at offset +0x0090
        // Writes to field at offset +0x00b0
    }

    /// Decompiled virtual method #13 @ RVA 0x0045efd0 (27 instructions, 5 basic blocks)
    pub fn method_13(&mut self) {
        // Writes to field at offset +0x0090
        // Writes to field at offset +0x00b0
    }

    /// Decompiled virtual method #14 @ RVA 0x004b8ce0 (114 instructions, 20 basic blocks)
    pub fn method_14(&mut self) {
        // Writes to field at offset +0x0028
    }

    /// Decompiled virtual method #15 @ RVA 0x004b66c0 (44 instructions, 12 basic blocks)
    pub fn method_15(&mut self) {
        // Writes to field at offset +0x0000
        // Writes to field at offset +0x0004
        // Writes to field at offset +0x0008
        // Writes to field at offset +0x000a
        // Writes to field at offset +0x00c4
        // Writes to field at offset +0x01e4
        // Writes to field at offset +0x02fc
    }

    /// Decompiled virtual method #16 @ RVA 0x004b6040 (54 instructions, 12 basic blocks)
    pub fn method_16(&mut self) {
    }

    /// Decompiled virtual method #17 @ RVA 0x004b80d0 (38 instructions, 8 basic blocks)
    pub fn method_17(&mut self) {
    }

    /// Decompiled virtual method #18 @ RVA 0x000d3090 (1 instructions, 1 basic blocks)
    pub fn method_18(&mut self) {
    }

    /// Decompiled virtual method #19 @ RVA 0x000d3090 (1 instructions, 1 basic blocks)
    pub fn method_19(&mut self) {
    }

    /// Decompiled virtual method #20 @ RVA 0x000d3090 (1 instructions, 1 basic blocks)
    pub fn method_20(&mut self) {
    }

    /// Decompiled virtual method #21 @ RVA 0x00103e70 (2 instructions, 1 basic blocks)
    pub fn method_21(&mut self) {
    }

    /// Decompiled virtual method #22 @ RVA 0x004b2920 (22 instructions, 3 basic blocks)
    pub fn method_22(&mut self) {
        // Writes to field at offset +0x01b0
    }

    /// Decompiled virtual method #23 @ RVA 0x004b8cb0 (6 instructions, 1 basic blocks)
    pub fn method_23(&mut self) {
        // Writes to field at offset +0x03e0
        // Writes to field at offset +0x0420
    }

}

impl CcClass for ccGobjEnemyHardshellS {
    fn class_name(&self) -> &'static str { Self::CLASS_NAME }
    fn vtable_rva(&self) -> u32 { Self::VTABLE_RVA }
    fn virtual_method_count(&self) -> usize { Self::METHOD_COUNT }
    fn base_classes(&self) -> &'static [&'static str] { Self::BASE_CLASSES }
}

/// Decompiled CC2 class: `ccGobjEnemyNormalL` (VTable RVA: 0x008bda90, 24 virtual methods, 68 recovered fields)
/// Inherits: ccGobjEnemy, ccGobjBattle, ccGameObj
#[derive(Debug, Clone, Default)]
pub struct ccGobjEnemyNormalL {
    pub vtable_rva: u32,
    pub field_0x0000: u8,
    pub field_0x0001: u32,
    pub field_0x0004: u8,
    pub field_0x0008: u16,
    pub field_0x000a: u16,
    pub field_0x0010: u64,
    pub field_0x0018: u64,
    pub field_0x001c: u8,
    pub field_0x001e: u16,
    pub field_0x0020: u32,
    pub field_0x0024: u16,
    pub field_0x002c: u16,
    pub field_0x0030: u32,
    pub field_0x0034: f32,
    pub field_0x0038: f32,
    pub field_0x003c: f32,
    pub field_0x0040: u32,
    pub field_0x007e: u16,
    pub field_0x0090: u32,
    pub field_0x0094: u32,
    pub field_0x0098: u64,
    pub field_0x009c: u32,
    pub field_0x00a0: u32,
    pub field_0x00a4: u32,
    pub field_0x00a8: u64,
    pub field_0x00ac: u32,
    pub field_0x00b0: u64,
    pub field_0x00b8: u64,
    pub field_0x00c4: u32,
    pub field_0x00e0: f32,
    pub field_0x00e4: f32,
    pub field_0x0120: u32,
    pub field_0x0124: u32,
    pub field_0x0128: u32,
    pub field_0x012c: u32,
    pub field_0x0130: u32,
    pub field_0x0134: u32,
    pub field_0x0138: u32,
    pub field_0x013c: u32,
    pub field_0x0140: u32,
    pub field_0x0144: u32,
    pub field_0x0148: u32,
    pub field_0x014c: u32,
    pub field_0x0150: u32,
    pub field_0x0154: u32,
    pub field_0x0158: u32,
    pub field_0x015c: u32,
    pub field_0x0170: u32,
    pub field_0x0198: u32,
    pub field_0x019c: u16,
    pub field_0x019e: u16,
    pub field_0x01b0: u64,
    pub field_0x01e4: u16,
    pub field_0x020c: u64,
    pub field_0x0214: u64,
    pub field_0x021c: u64,
    pub field_0x0224: u64,
    pub field_0x022c: u64,
    pub field_0x0234: u64,
    pub field_0x023c: u64,
    pub field_0x0244: u64,
    pub field_0x024c: u64,
    pub field_0x0254: u32,
    pub field_0x0278: u32,
    pub field_0x02da: u16,
    pub field_0x02fc: u8,
    pub field_0x0314: u16,
    pub field_0x03e0: u32,
}

impl ccGobjEnemyNormalL {
    pub const CLASS_NAME: &'static str = "ccGobjEnemyNormalL";
    pub const VTABLE_RVA: u32 = 0x008bda90;
    pub const METHOD_COUNT: usize = 24;
    pub const BASE_CLASSES: &'static [&'static str] = &["ccGobjEnemy", "ccGobjBattle", "ccGameObj"];
    pub const VIRTUAL_METHODS: &'static [u64] = &[0x00000001804b2860, 0x00000001804b3120, 0x000000018045e4c0, 0x00000001804b6a60, 0x000000018047ebd0, 0x000000018047f370, 0x00000001804b4ec0, 0x00000001800d3090, 0x00000001800d3090, 0x00000001804b85f0, 0x000000018047f200, 0x000000018047f0e0, 0x000000018045eec0, 0x000000018045efd0, 0x00000001804b2f50, 0x00000001804b66c0, 0x00000001804b6040, 0x00000001804b80d0, 0x00000001800d3090, 0x00000001800d3090, 0x00000001800d3090, 0x0000000180103e70, 0x00000001804b2920, 0x00000001804b85e0];

    pub fn new() -> Self {
        Self {
            vtable_rva: Self::VTABLE_RVA,
            field_0x0000: Default::default(),
            field_0x0001: Default::default(),
            field_0x0004: Default::default(),
            field_0x0008: Default::default(),
            field_0x000a: Default::default(),
            field_0x0010: Default::default(),
            field_0x0018: Default::default(),
            field_0x001c: Default::default(),
            field_0x001e: Default::default(),
            field_0x0020: Default::default(),
            field_0x0024: Default::default(),
            field_0x002c: Default::default(),
            field_0x0030: Default::default(),
            field_0x0034: Default::default(),
            field_0x0038: Default::default(),
            field_0x003c: Default::default(),
            field_0x0040: Default::default(),
            field_0x007e: Default::default(),
            field_0x0090: Default::default(),
            field_0x0094: Default::default(),
            field_0x0098: Default::default(),
            field_0x009c: Default::default(),
            field_0x00a0: Default::default(),
            field_0x00a4: Default::default(),
            field_0x00a8: Default::default(),
            field_0x00ac: Default::default(),
            field_0x00b0: Default::default(),
            field_0x00b8: Default::default(),
            field_0x00c4: Default::default(),
            field_0x00e0: Default::default(),
            field_0x00e4: Default::default(),
            field_0x0120: Default::default(),
            field_0x0124: Default::default(),
            field_0x0128: Default::default(),
            field_0x012c: Default::default(),
            field_0x0130: Default::default(),
            field_0x0134: Default::default(),
            field_0x0138: Default::default(),
            field_0x013c: Default::default(),
            field_0x0140: Default::default(),
            field_0x0144: Default::default(),
            field_0x0148: Default::default(),
            field_0x014c: Default::default(),
            field_0x0150: Default::default(),
            field_0x0154: Default::default(),
            field_0x0158: Default::default(),
            field_0x015c: Default::default(),
            field_0x0170: Default::default(),
            field_0x0198: Default::default(),
            field_0x019c: Default::default(),
            field_0x019e: Default::default(),
            field_0x01b0: Default::default(),
            field_0x01e4: Default::default(),
            field_0x020c: Default::default(),
            field_0x0214: Default::default(),
            field_0x021c: Default::default(),
            field_0x0224: Default::default(),
            field_0x022c: Default::default(),
            field_0x0234: Default::default(),
            field_0x023c: Default::default(),
            field_0x0244: Default::default(),
            field_0x024c: Default::default(),
            field_0x0254: Default::default(),
            field_0x0278: Default::default(),
            field_0x02da: Default::default(),
            field_0x02fc: Default::default(),
            field_0x0314: Default::default(),
            field_0x03e0: Default::default(),
        }
    }

    /// Decompiled virtual method #0 @ RVA 0x004b2860 (16 instructions, 3 basic blocks)
    pub fn method_00(&mut self) {
    }

    /// Decompiled virtual method #1 @ RVA 0x004b3120 (112 instructions, 15 basic blocks)
    pub fn method_01(&mut self) {
        // Writes to field at offset +0x0030
        // Writes to field at offset +0x00c4
    }

    /// Decompiled virtual method #2 @ RVA 0x0045e4c0 (42 instructions, 4 basic blocks)
    pub fn method_02(&mut self) {
        // Writes to field at offset +0x0090
        // Writes to field at offset +0x0094
        // Writes to field at offset +0x0098
        // Writes to field at offset +0x009c
        // Writes to field at offset +0x00a0
        // Writes to field at offset +0x00a4
        // Writes to field at offset +0x00a8
        // Writes to field at offset +0x00ac
        // Writes to field at offset +0x0140
        // Writes to field at offset +0x0144
        // Writes to field at offset +0x0148
        // Writes to field at offset +0x014c
        // Writes to field at offset +0x0150
        // Writes to field at offset +0x0154
        // Writes to field at offset +0x0158
        // Writes to field at offset +0x015c
    }

    /// Decompiled virtual method #3 @ RVA 0x004b6a60 (104 instructions, 18 basic blocks)
    pub fn method_03(&mut self) {
        // Writes to field at offset +0x0000
        // Writes to field at offset +0x0001
        // Writes to field at offset +0x0004
        // Writes to field at offset +0x00b0
    }

    /// Decompiled virtual method #4 @ RVA 0x0047ebd0 (45 instructions, 12 basic blocks)
    pub fn method_04(&mut self) {
        // Writes to field at offset +0x0004
        // Writes to field at offset +0x02da
        // Writes to field at offset +0x0314
        // Writes to field at offset +0x03e0
    }

    /// Decompiled virtual method #5 @ RVA 0x0047f370 (124 instructions, 30 basic blocks)
    pub fn method_05(&mut self) {
        // Writes to field at offset +0x0040
    }

    /// Decompiled virtual method #6 @ RVA 0x004b4ec0 (103 instructions, 17 basic blocks)
    pub fn method_06(&mut self) {
    }

    /// Decompiled virtual method #7 @ RVA 0x000d3090 (1 instructions, 1 basic blocks)
    pub fn method_07(&mut self) {
    }

    /// Decompiled virtual method #8 @ RVA 0x000d3090 (1 instructions, 1 basic blocks)
    pub fn method_08(&mut self) {
    }

    /// Decompiled virtual method #9 @ RVA 0x004b85f0 (113 instructions, 28 basic blocks)
    pub fn method_09(&mut self) {
        // Writes to field at offset +0x0000
        // Writes to field at offset +0x0004
        // Writes to field at offset +0x0008
        // Writes to field at offset +0x000a
    }

    /// Decompiled virtual method #10 @ RVA 0x0047f200 (58 instructions, 4 basic blocks)
    pub fn method_10(&mut self) {
    }

    /// Decompiled virtual method #11 @ RVA 0x0047f0e0 (15 instructions, 4 basic blocks)
    pub fn method_11(&mut self) {
    }

    /// Decompiled virtual method #12 @ RVA 0x0045eec0 (67 instructions, 22 basic blocks)
    pub fn method_12(&mut self) {
        // Writes to field at offset +0x001c
        // Writes to field at offset +0x0040
        // Writes to field at offset +0x0090
        // Writes to field at offset +0x00b0
    }

    /// Decompiled virtual method #13 @ RVA 0x0045efd0 (27 instructions, 5 basic blocks)
    pub fn method_13(&mut self) {
        // Writes to field at offset +0x0090
        // Writes to field at offset +0x00b0
    }

    /// Decompiled virtual method #14 @ RVA 0x004b2f50 (105 instructions, 2 basic blocks)
    pub fn method_14(&mut self) {
        // Writes to field at offset +0x0000
        // Writes to field at offset +0x0004
        // Writes to field at offset +0x0008
        // Writes to field at offset +0x000a
        // Writes to field at offset +0x00c4
        // Writes to field at offset +0x020c
        // Writes to field at offset +0x0214
        // Writes to field at offset +0x021c
        // Writes to field at offset +0x0224
        // Writes to field at offset +0x022c
        // Writes to field at offset +0x0234
        // Writes to field at offset +0x023c
        // Writes to field at offset +0x0244
        // Writes to field at offset +0x024c
        // Writes to field at offset +0x0254
    }

    /// Decompiled virtual method #15 @ RVA 0x004b66c0 (44 instructions, 12 basic blocks)
    pub fn method_15(&mut self) {
        // Writes to field at offset +0x0000
        // Writes to field at offset +0x0004
        // Writes to field at offset +0x0008
        // Writes to field at offset +0x000a
        // Writes to field at offset +0x00c4
        // Writes to field at offset +0x01e4
        // Writes to field at offset +0x02fc
    }

    /// Decompiled virtual method #16 @ RVA 0x004b6040 (54 instructions, 12 basic blocks)
    pub fn method_16(&mut self) {
    }

    /// Decompiled virtual method #17 @ RVA 0x004b80d0 (38 instructions, 8 basic blocks)
    pub fn method_17(&mut self) {
    }

    /// Decompiled virtual method #18 @ RVA 0x000d3090 (1 instructions, 1 basic blocks)
    pub fn method_18(&mut self) {
    }

    /// Decompiled virtual method #19 @ RVA 0x000d3090 (1 instructions, 1 basic blocks)
    pub fn method_19(&mut self) {
    }

    /// Decompiled virtual method #20 @ RVA 0x000d3090 (1 instructions, 1 basic blocks)
    pub fn method_20(&mut self) {
    }

    /// Decompiled virtual method #21 @ RVA 0x00103e70 (2 instructions, 1 basic blocks)
    pub fn method_21(&mut self) {
    }

    /// Decompiled virtual method #22 @ RVA 0x004b2920 (22 instructions, 3 basic blocks)
    pub fn method_22(&mut self) {
        // Writes to field at offset +0x01b0
    }

    /// Decompiled virtual method #23 @ RVA 0x004b85e0 (3 instructions, 1 basic blocks)
    pub fn method_23(&mut self) {
    }

}

impl CcClass for ccGobjEnemyNormalL {
    fn class_name(&self) -> &'static str { Self::CLASS_NAME }
    fn vtable_rva(&self) -> u32 { Self::VTABLE_RVA }
    fn virtual_method_count(&self) -> usize { Self::METHOD_COUNT }
    fn base_classes(&self) -> &'static [&'static str] { Self::BASE_CLASSES }
}

/// Decompiled CC2 class: `ccGobjEnemyNormalLL` (VTable RVA: 0x008bd838, 24 virtual methods, 68 recovered fields)
/// Inherits: ccGobjEnemy, ccGobjBattle, ccGameObj
#[derive(Debug, Clone, Default)]
pub struct ccGobjEnemyNormalLL {
    pub vtable_rva: u32,
    pub field_0x0000: u8,
    pub field_0x0001: u32,
    pub field_0x0004: u8,
    pub field_0x0008: u16,
    pub field_0x000a: u16,
    pub field_0x0010: u64,
    pub field_0x0018: u64,
    pub field_0x001c: u8,
    pub field_0x001e: u16,
    pub field_0x0020: u32,
    pub field_0x0024: u16,
    pub field_0x002c: u16,
    pub field_0x0030: u32,
    pub field_0x0034: f32,
    pub field_0x0038: f32,
    pub field_0x003c: f32,
    pub field_0x0040: u32,
    pub field_0x007e: u16,
    pub field_0x0090: u32,
    pub field_0x0094: u32,
    pub field_0x0098: u64,
    pub field_0x009c: u32,
    pub field_0x00a0: u32,
    pub field_0x00a4: u32,
    pub field_0x00a8: u64,
    pub field_0x00ac: u32,
    pub field_0x00b0: u64,
    pub field_0x00b8: u64,
    pub field_0x00c4: u32,
    pub field_0x00e0: f32,
    pub field_0x00e4: f32,
    pub field_0x0120: u32,
    pub field_0x0124: u32,
    pub field_0x0128: u32,
    pub field_0x012c: u32,
    pub field_0x0130: u32,
    pub field_0x0134: u32,
    pub field_0x0138: u32,
    pub field_0x013c: u32,
    pub field_0x0140: u32,
    pub field_0x0144: u32,
    pub field_0x0148: u32,
    pub field_0x014c: u32,
    pub field_0x0150: u32,
    pub field_0x0154: u32,
    pub field_0x0158: u32,
    pub field_0x015c: u32,
    pub field_0x0170: u32,
    pub field_0x0198: u32,
    pub field_0x019c: u16,
    pub field_0x019e: u16,
    pub field_0x01b0: u64,
    pub field_0x01e4: u16,
    pub field_0x020c: u64,
    pub field_0x0214: u64,
    pub field_0x021c: u64,
    pub field_0x0224: u64,
    pub field_0x022c: u64,
    pub field_0x0234: u64,
    pub field_0x023c: u64,
    pub field_0x0244: u64,
    pub field_0x024c: u64,
    pub field_0x0254: u32,
    pub field_0x0278: u32,
    pub field_0x02da: u16,
    pub field_0x02fc: u8,
    pub field_0x0314: u16,
    pub field_0x03e0: u32,
}

impl ccGobjEnemyNormalLL {
    pub const CLASS_NAME: &'static str = "ccGobjEnemyNormalLL";
    pub const VTABLE_RVA: u32 = 0x008bd838;
    pub const METHOD_COUNT: usize = 24;
    pub const BASE_CLASSES: &'static [&'static str] = &["ccGobjEnemy", "ccGobjBattle", "ccGameObj"];
    pub const VIRTUAL_METHODS: &'static [u64] = &[0x00000001804b2860, 0x00000001804b3120, 0x000000018045e4c0, 0x00000001804b6a60, 0x000000018047ebd0, 0x000000018047f370, 0x00000001804b4ec0, 0x00000001800d3090, 0x00000001800d3090, 0x00000001804b3290, 0x000000018047f200, 0x000000018047f0e0, 0x000000018045eec0, 0x000000018045efd0, 0x00000001804b2f50, 0x00000001804b66c0, 0x00000001804b6040, 0x00000001804b80d0, 0x00000001800d3090, 0x00000001800d3090, 0x00000001800d3090, 0x0000000180103e70, 0x00000001804b2920, 0x00000001800d3090];

    pub fn new() -> Self {
        Self {
            vtable_rva: Self::VTABLE_RVA,
            field_0x0000: Default::default(),
            field_0x0001: Default::default(),
            field_0x0004: Default::default(),
            field_0x0008: Default::default(),
            field_0x000a: Default::default(),
            field_0x0010: Default::default(),
            field_0x0018: Default::default(),
            field_0x001c: Default::default(),
            field_0x001e: Default::default(),
            field_0x0020: Default::default(),
            field_0x0024: Default::default(),
            field_0x002c: Default::default(),
            field_0x0030: Default::default(),
            field_0x0034: Default::default(),
            field_0x0038: Default::default(),
            field_0x003c: Default::default(),
            field_0x0040: Default::default(),
            field_0x007e: Default::default(),
            field_0x0090: Default::default(),
            field_0x0094: Default::default(),
            field_0x0098: Default::default(),
            field_0x009c: Default::default(),
            field_0x00a0: Default::default(),
            field_0x00a4: Default::default(),
            field_0x00a8: Default::default(),
            field_0x00ac: Default::default(),
            field_0x00b0: Default::default(),
            field_0x00b8: Default::default(),
            field_0x00c4: Default::default(),
            field_0x00e0: Default::default(),
            field_0x00e4: Default::default(),
            field_0x0120: Default::default(),
            field_0x0124: Default::default(),
            field_0x0128: Default::default(),
            field_0x012c: Default::default(),
            field_0x0130: Default::default(),
            field_0x0134: Default::default(),
            field_0x0138: Default::default(),
            field_0x013c: Default::default(),
            field_0x0140: Default::default(),
            field_0x0144: Default::default(),
            field_0x0148: Default::default(),
            field_0x014c: Default::default(),
            field_0x0150: Default::default(),
            field_0x0154: Default::default(),
            field_0x0158: Default::default(),
            field_0x015c: Default::default(),
            field_0x0170: Default::default(),
            field_0x0198: Default::default(),
            field_0x019c: Default::default(),
            field_0x019e: Default::default(),
            field_0x01b0: Default::default(),
            field_0x01e4: Default::default(),
            field_0x020c: Default::default(),
            field_0x0214: Default::default(),
            field_0x021c: Default::default(),
            field_0x0224: Default::default(),
            field_0x022c: Default::default(),
            field_0x0234: Default::default(),
            field_0x023c: Default::default(),
            field_0x0244: Default::default(),
            field_0x024c: Default::default(),
            field_0x0254: Default::default(),
            field_0x0278: Default::default(),
            field_0x02da: Default::default(),
            field_0x02fc: Default::default(),
            field_0x0314: Default::default(),
            field_0x03e0: Default::default(),
        }
    }

    /// Decompiled virtual method #0 @ RVA 0x004b2860 (16 instructions, 3 basic blocks)
    pub fn method_00(&mut self) {
    }

    /// Decompiled virtual method #1 @ RVA 0x004b3120 (112 instructions, 15 basic blocks)
    pub fn method_01(&mut self) {
        // Writes to field at offset +0x0030
        // Writes to field at offset +0x00c4
    }

    /// Decompiled virtual method #2 @ RVA 0x0045e4c0 (42 instructions, 4 basic blocks)
    pub fn method_02(&mut self) {
        // Writes to field at offset +0x0090
        // Writes to field at offset +0x0094
        // Writes to field at offset +0x0098
        // Writes to field at offset +0x009c
        // Writes to field at offset +0x00a0
        // Writes to field at offset +0x00a4
        // Writes to field at offset +0x00a8
        // Writes to field at offset +0x00ac
        // Writes to field at offset +0x0140
        // Writes to field at offset +0x0144
        // Writes to field at offset +0x0148
        // Writes to field at offset +0x014c
        // Writes to field at offset +0x0150
        // Writes to field at offset +0x0154
        // Writes to field at offset +0x0158
        // Writes to field at offset +0x015c
    }

    /// Decompiled virtual method #3 @ RVA 0x004b6a60 (104 instructions, 18 basic blocks)
    pub fn method_03(&mut self) {
        // Writes to field at offset +0x0000
        // Writes to field at offset +0x0001
        // Writes to field at offset +0x0004
        // Writes to field at offset +0x00b0
    }

    /// Decompiled virtual method #4 @ RVA 0x0047ebd0 (45 instructions, 12 basic blocks)
    pub fn method_04(&mut self) {
        // Writes to field at offset +0x0004
        // Writes to field at offset +0x02da
        // Writes to field at offset +0x0314
        // Writes to field at offset +0x03e0
    }

    /// Decompiled virtual method #5 @ RVA 0x0047f370 (124 instructions, 30 basic blocks)
    pub fn method_05(&mut self) {
        // Writes to field at offset +0x0040
    }

    /// Decompiled virtual method #6 @ RVA 0x004b4ec0 (103 instructions, 17 basic blocks)
    pub fn method_06(&mut self) {
    }

    /// Decompiled virtual method #7 @ RVA 0x000d3090 (1 instructions, 1 basic blocks)
    pub fn method_07(&mut self) {
    }

    /// Decompiled virtual method #8 @ RVA 0x000d3090 (1 instructions, 1 basic blocks)
    pub fn method_08(&mut self) {
    }

    /// Decompiled virtual method #9 @ RVA 0x004b3290 (56 instructions, 9 basic blocks)
    pub fn method_09(&mut self) {
    }

    /// Decompiled virtual method #10 @ RVA 0x0047f200 (58 instructions, 4 basic blocks)
    pub fn method_10(&mut self) {
    }

    /// Decompiled virtual method #11 @ RVA 0x0047f0e0 (15 instructions, 4 basic blocks)
    pub fn method_11(&mut self) {
    }

    /// Decompiled virtual method #12 @ RVA 0x0045eec0 (67 instructions, 22 basic blocks)
    pub fn method_12(&mut self) {
        // Writes to field at offset +0x001c
        // Writes to field at offset +0x0040
        // Writes to field at offset +0x0090
        // Writes to field at offset +0x00b0
    }

    /// Decompiled virtual method #13 @ RVA 0x0045efd0 (27 instructions, 5 basic blocks)
    pub fn method_13(&mut self) {
        // Writes to field at offset +0x0090
        // Writes to field at offset +0x00b0
    }

    /// Decompiled virtual method #14 @ RVA 0x004b2f50 (105 instructions, 2 basic blocks)
    pub fn method_14(&mut self) {
        // Writes to field at offset +0x0000
        // Writes to field at offset +0x0004
        // Writes to field at offset +0x0008
        // Writes to field at offset +0x000a
        // Writes to field at offset +0x00c4
        // Writes to field at offset +0x020c
        // Writes to field at offset +0x0214
        // Writes to field at offset +0x021c
        // Writes to field at offset +0x0224
        // Writes to field at offset +0x022c
        // Writes to field at offset +0x0234
        // Writes to field at offset +0x023c
        // Writes to field at offset +0x0244
        // Writes to field at offset +0x024c
        // Writes to field at offset +0x0254
    }

    /// Decompiled virtual method #15 @ RVA 0x004b66c0 (44 instructions, 12 basic blocks)
    pub fn method_15(&mut self) {
        // Writes to field at offset +0x0000
        // Writes to field at offset +0x0004
        // Writes to field at offset +0x0008
        // Writes to field at offset +0x000a
        // Writes to field at offset +0x00c4
        // Writes to field at offset +0x01e4
        // Writes to field at offset +0x02fc
    }

    /// Decompiled virtual method #16 @ RVA 0x004b6040 (54 instructions, 12 basic blocks)
    pub fn method_16(&mut self) {
    }

    /// Decompiled virtual method #17 @ RVA 0x004b80d0 (38 instructions, 8 basic blocks)
    pub fn method_17(&mut self) {
    }

    /// Decompiled virtual method #18 @ RVA 0x000d3090 (1 instructions, 1 basic blocks)
    pub fn method_18(&mut self) {
    }

    /// Decompiled virtual method #19 @ RVA 0x000d3090 (1 instructions, 1 basic blocks)
    pub fn method_19(&mut self) {
    }

    /// Decompiled virtual method #20 @ RVA 0x000d3090 (1 instructions, 1 basic blocks)
    pub fn method_20(&mut self) {
    }

    /// Decompiled virtual method #21 @ RVA 0x00103e70 (2 instructions, 1 basic blocks)
    pub fn method_21(&mut self) {
    }

    /// Decompiled virtual method #22 @ RVA 0x004b2920 (22 instructions, 3 basic blocks)
    pub fn method_22(&mut self) {
        // Writes to field at offset +0x01b0
    }

    /// Decompiled virtual method #23 @ RVA 0x000d3090 (1 instructions, 1 basic blocks)
    pub fn method_23(&mut self) {
    }

}

impl CcClass for ccGobjEnemyNormalLL {
    fn class_name(&self) -> &'static str { Self::CLASS_NAME }
    fn vtable_rva(&self) -> u32 { Self::VTABLE_RVA }
    fn virtual_method_count(&self) -> usize { Self::METHOD_COUNT }
    fn base_classes(&self) -> &'static [&'static str] { Self::BASE_CLASSES }
}

/// Decompiled CC2 class: `ccGobjEnemyNormalM` (VTable RVA: 0x008bd9c8, 24 virtual methods, 68 recovered fields)
/// Inherits: ccGobjEnemy, ccGobjBattle, ccGameObj
#[derive(Debug, Clone, Default)]
pub struct ccGobjEnemyNormalM {
    pub vtable_rva: u32,
    pub field_0x0000: u8,
    pub field_0x0001: u32,
    pub field_0x0004: u8,
    pub field_0x0008: u16,
    pub field_0x000a: u16,
    pub field_0x0010: u64,
    pub field_0x0018: u64,
    pub field_0x001c: u8,
    pub field_0x001e: u16,
    pub field_0x0020: u32,
    pub field_0x0024: u16,
    pub field_0x002c: u16,
    pub field_0x0030: u32,
    pub field_0x0034: f32,
    pub field_0x0038: f32,
    pub field_0x003c: f32,
    pub field_0x0040: u32,
    pub field_0x007e: u16,
    pub field_0x0090: u32,
    pub field_0x0094: u32,
    pub field_0x0098: u64,
    pub field_0x009c: u32,
    pub field_0x00a0: u32,
    pub field_0x00a4: u32,
    pub field_0x00a8: u64,
    pub field_0x00ac: u32,
    pub field_0x00b0: u64,
    pub field_0x00b8: u64,
    pub field_0x00c4: u32,
    pub field_0x00e0: f32,
    pub field_0x00e4: f32,
    pub field_0x0120: u32,
    pub field_0x0124: u32,
    pub field_0x0128: u32,
    pub field_0x012c: u32,
    pub field_0x0130: u32,
    pub field_0x0134: u32,
    pub field_0x0138: u32,
    pub field_0x013c: u32,
    pub field_0x0140: u32,
    pub field_0x0144: u32,
    pub field_0x0148: u32,
    pub field_0x014c: u32,
    pub field_0x0150: u32,
    pub field_0x0154: u32,
    pub field_0x0158: u32,
    pub field_0x015c: u32,
    pub field_0x0170: u32,
    pub field_0x0198: u32,
    pub field_0x019c: u16,
    pub field_0x019e: u16,
    pub field_0x01b0: u64,
    pub field_0x01e4: u16,
    pub field_0x020c: u64,
    pub field_0x0214: u64,
    pub field_0x021c: u64,
    pub field_0x0224: u64,
    pub field_0x022c: u64,
    pub field_0x0234: u64,
    pub field_0x023c: u64,
    pub field_0x0244: u64,
    pub field_0x024c: u64,
    pub field_0x0254: u32,
    pub field_0x0278: u32,
    pub field_0x02da: u16,
    pub field_0x02fc: u8,
    pub field_0x0314: u16,
    pub field_0x03e0: u32,
}

impl ccGobjEnemyNormalM {
    pub const CLASS_NAME: &'static str = "ccGobjEnemyNormalM";
    pub const VTABLE_RVA: u32 = 0x008bd9c8;
    pub const METHOD_COUNT: usize = 24;
    pub const BASE_CLASSES: &'static [&'static str] = &["ccGobjEnemy", "ccGobjBattle", "ccGameObj"];
    pub const VIRTUAL_METHODS: &'static [u64] = &[0x00000001804b2860, 0x00000001804b3120, 0x000000018045e4c0, 0x00000001804b6a60, 0x000000018047ebd0, 0x000000018047f370, 0x00000001804b4ec0, 0x00000001800d3090, 0x00000001800d3090, 0x00000001804b3290, 0x000000018047f200, 0x000000018047f0e0, 0x000000018045eec0, 0x000000018045efd0, 0x00000001804b2f50, 0x00000001804b66c0, 0x00000001804b6040, 0x00000001804b80d0, 0x00000001800d3090, 0x00000001800d3090, 0x00000001800d3090, 0x0000000180103e70, 0x00000001804b2920, 0x00000001800d3090];

    pub fn new() -> Self {
        Self {
            vtable_rva: Self::VTABLE_RVA,
            field_0x0000: Default::default(),
            field_0x0001: Default::default(),
            field_0x0004: Default::default(),
            field_0x0008: Default::default(),
            field_0x000a: Default::default(),
            field_0x0010: Default::default(),
            field_0x0018: Default::default(),
            field_0x001c: Default::default(),
            field_0x001e: Default::default(),
            field_0x0020: Default::default(),
            field_0x0024: Default::default(),
            field_0x002c: Default::default(),
            field_0x0030: Default::default(),
            field_0x0034: Default::default(),
            field_0x0038: Default::default(),
            field_0x003c: Default::default(),
            field_0x0040: Default::default(),
            field_0x007e: Default::default(),
            field_0x0090: Default::default(),
            field_0x0094: Default::default(),
            field_0x0098: Default::default(),
            field_0x009c: Default::default(),
            field_0x00a0: Default::default(),
            field_0x00a4: Default::default(),
            field_0x00a8: Default::default(),
            field_0x00ac: Default::default(),
            field_0x00b0: Default::default(),
            field_0x00b8: Default::default(),
            field_0x00c4: Default::default(),
            field_0x00e0: Default::default(),
            field_0x00e4: Default::default(),
            field_0x0120: Default::default(),
            field_0x0124: Default::default(),
            field_0x0128: Default::default(),
            field_0x012c: Default::default(),
            field_0x0130: Default::default(),
            field_0x0134: Default::default(),
            field_0x0138: Default::default(),
            field_0x013c: Default::default(),
            field_0x0140: Default::default(),
            field_0x0144: Default::default(),
            field_0x0148: Default::default(),
            field_0x014c: Default::default(),
            field_0x0150: Default::default(),
            field_0x0154: Default::default(),
            field_0x0158: Default::default(),
            field_0x015c: Default::default(),
            field_0x0170: Default::default(),
            field_0x0198: Default::default(),
            field_0x019c: Default::default(),
            field_0x019e: Default::default(),
            field_0x01b0: Default::default(),
            field_0x01e4: Default::default(),
            field_0x020c: Default::default(),
            field_0x0214: Default::default(),
            field_0x021c: Default::default(),
            field_0x0224: Default::default(),
            field_0x022c: Default::default(),
            field_0x0234: Default::default(),
            field_0x023c: Default::default(),
            field_0x0244: Default::default(),
            field_0x024c: Default::default(),
            field_0x0254: Default::default(),
            field_0x0278: Default::default(),
            field_0x02da: Default::default(),
            field_0x02fc: Default::default(),
            field_0x0314: Default::default(),
            field_0x03e0: Default::default(),
        }
    }

    /// Decompiled virtual method #0 @ RVA 0x004b2860 (16 instructions, 3 basic blocks)
    pub fn method_00(&mut self) {
    }

    /// Decompiled virtual method #1 @ RVA 0x004b3120 (112 instructions, 15 basic blocks)
    pub fn method_01(&mut self) {
        // Writes to field at offset +0x0030
        // Writes to field at offset +0x00c4
    }

    /// Decompiled virtual method #2 @ RVA 0x0045e4c0 (42 instructions, 4 basic blocks)
    pub fn method_02(&mut self) {
        // Writes to field at offset +0x0090
        // Writes to field at offset +0x0094
        // Writes to field at offset +0x0098
        // Writes to field at offset +0x009c
        // Writes to field at offset +0x00a0
        // Writes to field at offset +0x00a4
        // Writes to field at offset +0x00a8
        // Writes to field at offset +0x00ac
        // Writes to field at offset +0x0140
        // Writes to field at offset +0x0144
        // Writes to field at offset +0x0148
        // Writes to field at offset +0x014c
        // Writes to field at offset +0x0150
        // Writes to field at offset +0x0154
        // Writes to field at offset +0x0158
        // Writes to field at offset +0x015c
    }

    /// Decompiled virtual method #3 @ RVA 0x004b6a60 (104 instructions, 18 basic blocks)
    pub fn method_03(&mut self) {
        // Writes to field at offset +0x0000
        // Writes to field at offset +0x0001
        // Writes to field at offset +0x0004
        // Writes to field at offset +0x00b0
    }

    /// Decompiled virtual method #4 @ RVA 0x0047ebd0 (45 instructions, 12 basic blocks)
    pub fn method_04(&mut self) {
        // Writes to field at offset +0x0004
        // Writes to field at offset +0x02da
        // Writes to field at offset +0x0314
        // Writes to field at offset +0x03e0
    }

    /// Decompiled virtual method #5 @ RVA 0x0047f370 (124 instructions, 30 basic blocks)
    pub fn method_05(&mut self) {
        // Writes to field at offset +0x0040
    }

    /// Decompiled virtual method #6 @ RVA 0x004b4ec0 (103 instructions, 17 basic blocks)
    pub fn method_06(&mut self) {
    }

    /// Decompiled virtual method #7 @ RVA 0x000d3090 (1 instructions, 1 basic blocks)
    pub fn method_07(&mut self) {
    }

    /// Decompiled virtual method #8 @ RVA 0x000d3090 (1 instructions, 1 basic blocks)
    pub fn method_08(&mut self) {
    }

    /// Decompiled virtual method #9 @ RVA 0x004b3290 (56 instructions, 9 basic blocks)
    pub fn method_09(&mut self) {
    }

    /// Decompiled virtual method #10 @ RVA 0x0047f200 (58 instructions, 4 basic blocks)
    pub fn method_10(&mut self) {
    }

    /// Decompiled virtual method #11 @ RVA 0x0047f0e0 (15 instructions, 4 basic blocks)
    pub fn method_11(&mut self) {
    }

    /// Decompiled virtual method #12 @ RVA 0x0045eec0 (67 instructions, 22 basic blocks)
    pub fn method_12(&mut self) {
        // Writes to field at offset +0x001c
        // Writes to field at offset +0x0040
        // Writes to field at offset +0x0090
        // Writes to field at offset +0x00b0
    }

    /// Decompiled virtual method #13 @ RVA 0x0045efd0 (27 instructions, 5 basic blocks)
    pub fn method_13(&mut self) {
        // Writes to field at offset +0x0090
        // Writes to field at offset +0x00b0
    }

    /// Decompiled virtual method #14 @ RVA 0x004b2f50 (105 instructions, 2 basic blocks)
    pub fn method_14(&mut self) {
        // Writes to field at offset +0x0000
        // Writes to field at offset +0x0004
        // Writes to field at offset +0x0008
        // Writes to field at offset +0x000a
        // Writes to field at offset +0x00c4
        // Writes to field at offset +0x020c
        // Writes to field at offset +0x0214
        // Writes to field at offset +0x021c
        // Writes to field at offset +0x0224
        // Writes to field at offset +0x022c
        // Writes to field at offset +0x0234
        // Writes to field at offset +0x023c
        // Writes to field at offset +0x0244
        // Writes to field at offset +0x024c
        // Writes to field at offset +0x0254
    }

    /// Decompiled virtual method #15 @ RVA 0x004b66c0 (44 instructions, 12 basic blocks)
    pub fn method_15(&mut self) {
        // Writes to field at offset +0x0000
        // Writes to field at offset +0x0004
        // Writes to field at offset +0x0008
        // Writes to field at offset +0x000a
        // Writes to field at offset +0x00c4
        // Writes to field at offset +0x01e4
        // Writes to field at offset +0x02fc
    }

    /// Decompiled virtual method #16 @ RVA 0x004b6040 (54 instructions, 12 basic blocks)
    pub fn method_16(&mut self) {
    }

    /// Decompiled virtual method #17 @ RVA 0x004b80d0 (38 instructions, 8 basic blocks)
    pub fn method_17(&mut self) {
    }

    /// Decompiled virtual method #18 @ RVA 0x000d3090 (1 instructions, 1 basic blocks)
    pub fn method_18(&mut self) {
    }

    /// Decompiled virtual method #19 @ RVA 0x000d3090 (1 instructions, 1 basic blocks)
    pub fn method_19(&mut self) {
    }

    /// Decompiled virtual method #20 @ RVA 0x000d3090 (1 instructions, 1 basic blocks)
    pub fn method_20(&mut self) {
    }

    /// Decompiled virtual method #21 @ RVA 0x00103e70 (2 instructions, 1 basic blocks)
    pub fn method_21(&mut self) {
    }

    /// Decompiled virtual method #22 @ RVA 0x004b2920 (22 instructions, 3 basic blocks)
    pub fn method_22(&mut self) {
        // Writes to field at offset +0x01b0
    }

    /// Decompiled virtual method #23 @ RVA 0x000d3090 (1 instructions, 1 basic blocks)
    pub fn method_23(&mut self) {
    }

}

impl CcClass for ccGobjEnemyNormalM {
    fn class_name(&self) -> &'static str { Self::CLASS_NAME }
    fn vtable_rva(&self) -> u32 { Self::VTABLE_RVA }
    fn virtual_method_count(&self) -> usize { Self::METHOD_COUNT }
    fn base_classes(&self) -> &'static [&'static str] { Self::BASE_CLASSES }
}

/// Decompiled CC2 class: `ccGobjEnemyNormalS` (VTable RVA: 0x008bd1f8, 24 virtual methods, 68 recovered fields)
/// Inherits: ccGobjEnemy, ccGobjBattle, ccGameObj
#[derive(Debug, Clone, Default)]
pub struct ccGobjEnemyNormalS {
    pub vtable_rva: u32,
    pub field_0x0000: u8,
    pub field_0x0001: u32,
    pub field_0x0004: u8,
    pub field_0x0008: u16,
    pub field_0x000a: u16,
    pub field_0x0010: u64,
    pub field_0x0018: u64,
    pub field_0x001c: u8,
    pub field_0x001e: u16,
    pub field_0x0020: u32,
    pub field_0x0024: u16,
    pub field_0x002c: u16,
    pub field_0x0030: u32,
    pub field_0x0034: f32,
    pub field_0x0038: f32,
    pub field_0x003c: f32,
    pub field_0x0040: u32,
    pub field_0x007e: u16,
    pub field_0x0090: u32,
    pub field_0x0094: u32,
    pub field_0x0098: u64,
    pub field_0x009c: u32,
    pub field_0x00a0: u32,
    pub field_0x00a4: u32,
    pub field_0x00a8: u64,
    pub field_0x00ac: u32,
    pub field_0x00b0: u64,
    pub field_0x00b8: u64,
    pub field_0x00c4: u32,
    pub field_0x00e0: f32,
    pub field_0x00e4: f32,
    pub field_0x0120: u32,
    pub field_0x0124: u32,
    pub field_0x0128: u32,
    pub field_0x012c: u32,
    pub field_0x0130: u32,
    pub field_0x0134: u32,
    pub field_0x0138: u32,
    pub field_0x013c: u32,
    pub field_0x0140: u32,
    pub field_0x0144: u32,
    pub field_0x0148: u32,
    pub field_0x014c: u32,
    pub field_0x0150: u32,
    pub field_0x0154: u32,
    pub field_0x0158: u32,
    pub field_0x015c: u32,
    pub field_0x0170: u32,
    pub field_0x0198: u32,
    pub field_0x019c: u16,
    pub field_0x019e: u16,
    pub field_0x01b0: u64,
    pub field_0x01e4: u16,
    pub field_0x020c: u64,
    pub field_0x0214: u64,
    pub field_0x021c: u64,
    pub field_0x0224: u64,
    pub field_0x022c: u64,
    pub field_0x0234: u64,
    pub field_0x023c: u64,
    pub field_0x0244: u64,
    pub field_0x024c: u64,
    pub field_0x0254: u32,
    pub field_0x0278: u32,
    pub field_0x02da: u16,
    pub field_0x02fc: u8,
    pub field_0x0314: u16,
    pub field_0x03e0: u32,
}

impl ccGobjEnemyNormalS {
    pub const CLASS_NAME: &'static str = "ccGobjEnemyNormalS";
    pub const VTABLE_RVA: u32 = 0x008bd1f8;
    pub const METHOD_COUNT: usize = 24;
    pub const BASE_CLASSES: &'static [&'static str] = &["ccGobjEnemy", "ccGobjBattle", "ccGameObj"];
    pub const VIRTUAL_METHODS: &'static [u64] = &[0x00000001804b2860, 0x00000001804b3120, 0x000000018045e4c0, 0x00000001804b6a60, 0x000000018047ebd0, 0x000000018047f370, 0x00000001804b4ec0, 0x00000001800d3090, 0x00000001800d3090, 0x00000001804b3290, 0x000000018047f200, 0x000000018047f0e0, 0x000000018045eec0, 0x000000018045efd0, 0x00000001804b2f50, 0x00000001804b66c0, 0x00000001804b6040, 0x00000001804b80d0, 0x00000001800d3090, 0x00000001800d3090, 0x00000001800d3090, 0x0000000180103e70, 0x00000001804b2920, 0x00000001800d3090];

    pub fn new() -> Self {
        Self {
            vtable_rva: Self::VTABLE_RVA,
            field_0x0000: Default::default(),
            field_0x0001: Default::default(),
            field_0x0004: Default::default(),
            field_0x0008: Default::default(),
            field_0x000a: Default::default(),
            field_0x0010: Default::default(),
            field_0x0018: Default::default(),
            field_0x001c: Default::default(),
            field_0x001e: Default::default(),
            field_0x0020: Default::default(),
            field_0x0024: Default::default(),
            field_0x002c: Default::default(),
            field_0x0030: Default::default(),
            field_0x0034: Default::default(),
            field_0x0038: Default::default(),
            field_0x003c: Default::default(),
            field_0x0040: Default::default(),
            field_0x007e: Default::default(),
            field_0x0090: Default::default(),
            field_0x0094: Default::default(),
            field_0x0098: Default::default(),
            field_0x009c: Default::default(),
            field_0x00a0: Default::default(),
            field_0x00a4: Default::default(),
            field_0x00a8: Default::default(),
            field_0x00ac: Default::default(),
            field_0x00b0: Default::default(),
            field_0x00b8: Default::default(),
            field_0x00c4: Default::default(),
            field_0x00e0: Default::default(),
            field_0x00e4: Default::default(),
            field_0x0120: Default::default(),
            field_0x0124: Default::default(),
            field_0x0128: Default::default(),
            field_0x012c: Default::default(),
            field_0x0130: Default::default(),
            field_0x0134: Default::default(),
            field_0x0138: Default::default(),
            field_0x013c: Default::default(),
            field_0x0140: Default::default(),
            field_0x0144: Default::default(),
            field_0x0148: Default::default(),
            field_0x014c: Default::default(),
            field_0x0150: Default::default(),
            field_0x0154: Default::default(),
            field_0x0158: Default::default(),
            field_0x015c: Default::default(),
            field_0x0170: Default::default(),
            field_0x0198: Default::default(),
            field_0x019c: Default::default(),
            field_0x019e: Default::default(),
            field_0x01b0: Default::default(),
            field_0x01e4: Default::default(),
            field_0x020c: Default::default(),
            field_0x0214: Default::default(),
            field_0x021c: Default::default(),
            field_0x0224: Default::default(),
            field_0x022c: Default::default(),
            field_0x0234: Default::default(),
            field_0x023c: Default::default(),
            field_0x0244: Default::default(),
            field_0x024c: Default::default(),
            field_0x0254: Default::default(),
            field_0x0278: Default::default(),
            field_0x02da: Default::default(),
            field_0x02fc: Default::default(),
            field_0x0314: Default::default(),
            field_0x03e0: Default::default(),
        }
    }

    /// Decompiled virtual method #0 @ RVA 0x004b2860 (16 instructions, 3 basic blocks)
    pub fn method_00(&mut self) {
    }

    /// Decompiled virtual method #1 @ RVA 0x004b3120 (112 instructions, 15 basic blocks)
    pub fn method_01(&mut self) {
        // Writes to field at offset +0x0030
        // Writes to field at offset +0x00c4
    }

    /// Decompiled virtual method #2 @ RVA 0x0045e4c0 (42 instructions, 4 basic blocks)
    pub fn method_02(&mut self) {
        // Writes to field at offset +0x0090
        // Writes to field at offset +0x0094
        // Writes to field at offset +0x0098
        // Writes to field at offset +0x009c
        // Writes to field at offset +0x00a0
        // Writes to field at offset +0x00a4
        // Writes to field at offset +0x00a8
        // Writes to field at offset +0x00ac
        // Writes to field at offset +0x0140
        // Writes to field at offset +0x0144
        // Writes to field at offset +0x0148
        // Writes to field at offset +0x014c
        // Writes to field at offset +0x0150
        // Writes to field at offset +0x0154
        // Writes to field at offset +0x0158
        // Writes to field at offset +0x015c
    }

    /// Decompiled virtual method #3 @ RVA 0x004b6a60 (104 instructions, 18 basic blocks)
    pub fn method_03(&mut self) {
        // Writes to field at offset +0x0000
        // Writes to field at offset +0x0001
        // Writes to field at offset +0x0004
        // Writes to field at offset +0x00b0
    }

    /// Decompiled virtual method #4 @ RVA 0x0047ebd0 (45 instructions, 12 basic blocks)
    pub fn method_04(&mut self) {
        // Writes to field at offset +0x0004
        // Writes to field at offset +0x02da
        // Writes to field at offset +0x0314
        // Writes to field at offset +0x03e0
    }

    /// Decompiled virtual method #5 @ RVA 0x0047f370 (124 instructions, 30 basic blocks)
    pub fn method_05(&mut self) {
        // Writes to field at offset +0x0040
    }

    /// Decompiled virtual method #6 @ RVA 0x004b4ec0 (103 instructions, 17 basic blocks)
    pub fn method_06(&mut self) {
    }

    /// Decompiled virtual method #7 @ RVA 0x000d3090 (1 instructions, 1 basic blocks)
    pub fn method_07(&mut self) {
    }

    /// Decompiled virtual method #8 @ RVA 0x000d3090 (1 instructions, 1 basic blocks)
    pub fn method_08(&mut self) {
    }

    /// Decompiled virtual method #9 @ RVA 0x004b3290 (56 instructions, 9 basic blocks)
    pub fn method_09(&mut self) {
    }

    /// Decompiled virtual method #10 @ RVA 0x0047f200 (58 instructions, 4 basic blocks)
    pub fn method_10(&mut self) {
    }

    /// Decompiled virtual method #11 @ RVA 0x0047f0e0 (15 instructions, 4 basic blocks)
    pub fn method_11(&mut self) {
    }

    /// Decompiled virtual method #12 @ RVA 0x0045eec0 (67 instructions, 22 basic blocks)
    pub fn method_12(&mut self) {
        // Writes to field at offset +0x001c
        // Writes to field at offset +0x0040
        // Writes to field at offset +0x0090
        // Writes to field at offset +0x00b0
    }

    /// Decompiled virtual method #13 @ RVA 0x0045efd0 (27 instructions, 5 basic blocks)
    pub fn method_13(&mut self) {
        // Writes to field at offset +0x0090
        // Writes to field at offset +0x00b0
    }

    /// Decompiled virtual method #14 @ RVA 0x004b2f50 (105 instructions, 2 basic blocks)
    pub fn method_14(&mut self) {
        // Writes to field at offset +0x0000
        // Writes to field at offset +0x0004
        // Writes to field at offset +0x0008
        // Writes to field at offset +0x000a
        // Writes to field at offset +0x00c4
        // Writes to field at offset +0x020c
        // Writes to field at offset +0x0214
        // Writes to field at offset +0x021c
        // Writes to field at offset +0x0224
        // Writes to field at offset +0x022c
        // Writes to field at offset +0x0234
        // Writes to field at offset +0x023c
        // Writes to field at offset +0x0244
        // Writes to field at offset +0x024c
        // Writes to field at offset +0x0254
    }

    /// Decompiled virtual method #15 @ RVA 0x004b66c0 (44 instructions, 12 basic blocks)
    pub fn method_15(&mut self) {
        // Writes to field at offset +0x0000
        // Writes to field at offset +0x0004
        // Writes to field at offset +0x0008
        // Writes to field at offset +0x000a
        // Writes to field at offset +0x00c4
        // Writes to field at offset +0x01e4
        // Writes to field at offset +0x02fc
    }

    /// Decompiled virtual method #16 @ RVA 0x004b6040 (54 instructions, 12 basic blocks)
    pub fn method_16(&mut self) {
    }

    /// Decompiled virtual method #17 @ RVA 0x004b80d0 (38 instructions, 8 basic blocks)
    pub fn method_17(&mut self) {
    }

    /// Decompiled virtual method #18 @ RVA 0x000d3090 (1 instructions, 1 basic blocks)
    pub fn method_18(&mut self) {
    }

    /// Decompiled virtual method #19 @ RVA 0x000d3090 (1 instructions, 1 basic blocks)
    pub fn method_19(&mut self) {
    }

    /// Decompiled virtual method #20 @ RVA 0x000d3090 (1 instructions, 1 basic blocks)
    pub fn method_20(&mut self) {
    }

    /// Decompiled virtual method #21 @ RVA 0x00103e70 (2 instructions, 1 basic blocks)
    pub fn method_21(&mut self) {
    }

    /// Decompiled virtual method #22 @ RVA 0x004b2920 (22 instructions, 3 basic blocks)
    pub fn method_22(&mut self) {
        // Writes to field at offset +0x01b0
    }

    /// Decompiled virtual method #23 @ RVA 0x000d3090 (1 instructions, 1 basic blocks)
    pub fn method_23(&mut self) {
    }

}

impl CcClass for ccGobjEnemyNormalS {
    fn class_name(&self) -> &'static str { Self::CLASS_NAME }
    fn vtable_rva(&self) -> u32 { Self::VTABLE_RVA }
    fn virtual_method_count(&self) -> usize { Self::METHOD_COUNT }
    fn base_classes(&self) -> &'static [&'static str] { Self::BASE_CLASSES }
}

/// Decompiled CC2 class: `ccGobjEnemyShield` (VTable RVA: 0x008bd130, 24 virtual methods, 68 recovered fields)
/// Inherits: ccGobjEnemy, ccGobjBattle, ccGameObj
#[derive(Debug, Clone, Default)]
pub struct ccGobjEnemyShield {
    pub vtable_rva: u32,
    pub field_0x0000: u8,
    pub field_0x0001: u32,
    pub field_0x0004: u8,
    pub field_0x0008: u16,
    pub field_0x000a: u16,
    pub field_0x0010: u64,
    pub field_0x0018: u64,
    pub field_0x001c: u8,
    pub field_0x001e: u16,
    pub field_0x0020: u32,
    pub field_0x0024: u16,
    pub field_0x002c: u16,
    pub field_0x0030: u32,
    pub field_0x0034: f32,
    pub field_0x0038: f32,
    pub field_0x003c: f32,
    pub field_0x0040: u32,
    pub field_0x007e: u16,
    pub field_0x0090: u32,
    pub field_0x0094: u32,
    pub field_0x0098: u64,
    pub field_0x009c: u32,
    pub field_0x00a0: u32,
    pub field_0x00a4: u32,
    pub field_0x00a8: u64,
    pub field_0x00ac: u32,
    pub field_0x00b0: u64,
    pub field_0x00b8: u64,
    pub field_0x00c4: u32,
    pub field_0x00e0: f32,
    pub field_0x00e4: f32,
    pub field_0x0120: u32,
    pub field_0x0124: u32,
    pub field_0x0128: u32,
    pub field_0x012c: u32,
    pub field_0x0130: u32,
    pub field_0x0134: u32,
    pub field_0x0138: u32,
    pub field_0x013c: u32,
    pub field_0x0140: u32,
    pub field_0x0144: u32,
    pub field_0x0148: u32,
    pub field_0x014c: u32,
    pub field_0x0150: u32,
    pub field_0x0154: u32,
    pub field_0x0158: u32,
    pub field_0x015c: u32,
    pub field_0x0170: u32,
    pub field_0x0198: u32,
    pub field_0x019c: u16,
    pub field_0x019e: u16,
    pub field_0x01b0: u64,
    pub field_0x01e4: u16,
    pub field_0x020c: u64,
    pub field_0x0214: u64,
    pub field_0x021c: u64,
    pub field_0x0224: u64,
    pub field_0x022c: u64,
    pub field_0x0234: u64,
    pub field_0x023c: u64,
    pub field_0x0244: u64,
    pub field_0x024c: u64,
    pub field_0x0254: u32,
    pub field_0x0278: u32,
    pub field_0x02da: u16,
    pub field_0x02fc: u8,
    pub field_0x0314: u16,
    pub field_0x03e0: u32,
}

impl ccGobjEnemyShield {
    pub const CLASS_NAME: &'static str = "ccGobjEnemyShield";
    pub const VTABLE_RVA: u32 = 0x008bd130;
    pub const METHOD_COUNT: usize = 24;
    pub const BASE_CLASSES: &'static [&'static str] = &["ccGobjEnemy", "ccGobjBattle", "ccGameObj"];
    pub const VIRTUAL_METHODS: &'static [u64] = &[0x00000001804b2860, 0x00000001804b3120, 0x000000018045e4c0, 0x00000001804b6a60, 0x000000018047ebd0, 0x000000018047f370, 0x00000001804b4ec0, 0x00000001800d3090, 0x00000001800d3090, 0x00000001804b8840, 0x000000018047f200, 0x000000018047f0e0, 0x000000018045eec0, 0x000000018045efd0, 0x00000001804b2f50, 0x00000001804b66c0, 0x00000001804b6040, 0x00000001804b80d0, 0x00000001800d3090, 0x00000001800d3090, 0x00000001800d3090, 0x00000001804b8980, 0x00000001804b2920, 0x00000001804b8820];

    pub fn new() -> Self {
        Self {
            vtable_rva: Self::VTABLE_RVA,
            field_0x0000: Default::default(),
            field_0x0001: Default::default(),
            field_0x0004: Default::default(),
            field_0x0008: Default::default(),
            field_0x000a: Default::default(),
            field_0x0010: Default::default(),
            field_0x0018: Default::default(),
            field_0x001c: Default::default(),
            field_0x001e: Default::default(),
            field_0x0020: Default::default(),
            field_0x0024: Default::default(),
            field_0x002c: Default::default(),
            field_0x0030: Default::default(),
            field_0x0034: Default::default(),
            field_0x0038: Default::default(),
            field_0x003c: Default::default(),
            field_0x0040: Default::default(),
            field_0x007e: Default::default(),
            field_0x0090: Default::default(),
            field_0x0094: Default::default(),
            field_0x0098: Default::default(),
            field_0x009c: Default::default(),
            field_0x00a0: Default::default(),
            field_0x00a4: Default::default(),
            field_0x00a8: Default::default(),
            field_0x00ac: Default::default(),
            field_0x00b0: Default::default(),
            field_0x00b8: Default::default(),
            field_0x00c4: Default::default(),
            field_0x00e0: Default::default(),
            field_0x00e4: Default::default(),
            field_0x0120: Default::default(),
            field_0x0124: Default::default(),
            field_0x0128: Default::default(),
            field_0x012c: Default::default(),
            field_0x0130: Default::default(),
            field_0x0134: Default::default(),
            field_0x0138: Default::default(),
            field_0x013c: Default::default(),
            field_0x0140: Default::default(),
            field_0x0144: Default::default(),
            field_0x0148: Default::default(),
            field_0x014c: Default::default(),
            field_0x0150: Default::default(),
            field_0x0154: Default::default(),
            field_0x0158: Default::default(),
            field_0x015c: Default::default(),
            field_0x0170: Default::default(),
            field_0x0198: Default::default(),
            field_0x019c: Default::default(),
            field_0x019e: Default::default(),
            field_0x01b0: Default::default(),
            field_0x01e4: Default::default(),
            field_0x020c: Default::default(),
            field_0x0214: Default::default(),
            field_0x021c: Default::default(),
            field_0x0224: Default::default(),
            field_0x022c: Default::default(),
            field_0x0234: Default::default(),
            field_0x023c: Default::default(),
            field_0x0244: Default::default(),
            field_0x024c: Default::default(),
            field_0x0254: Default::default(),
            field_0x0278: Default::default(),
            field_0x02da: Default::default(),
            field_0x02fc: Default::default(),
            field_0x0314: Default::default(),
            field_0x03e0: Default::default(),
        }
    }

    /// Decompiled virtual method #0 @ RVA 0x004b2860 (16 instructions, 3 basic blocks)
    pub fn method_00(&mut self) {
    }

    /// Decompiled virtual method #1 @ RVA 0x004b3120 (112 instructions, 15 basic blocks)
    pub fn method_01(&mut self) {
        // Writes to field at offset +0x0030
        // Writes to field at offset +0x00c4
    }

    /// Decompiled virtual method #2 @ RVA 0x0045e4c0 (42 instructions, 4 basic blocks)
    pub fn method_02(&mut self) {
        // Writes to field at offset +0x0090
        // Writes to field at offset +0x0094
        // Writes to field at offset +0x0098
        // Writes to field at offset +0x009c
        // Writes to field at offset +0x00a0
        // Writes to field at offset +0x00a4
        // Writes to field at offset +0x00a8
        // Writes to field at offset +0x00ac
        // Writes to field at offset +0x0140
        // Writes to field at offset +0x0144
        // Writes to field at offset +0x0148
        // Writes to field at offset +0x014c
        // Writes to field at offset +0x0150
        // Writes to field at offset +0x0154
        // Writes to field at offset +0x0158
        // Writes to field at offset +0x015c
    }

    /// Decompiled virtual method #3 @ RVA 0x004b6a60 (104 instructions, 18 basic blocks)
    pub fn method_03(&mut self) {
        // Writes to field at offset +0x0000
        // Writes to field at offset +0x0001
        // Writes to field at offset +0x0004
        // Writes to field at offset +0x00b0
    }

    /// Decompiled virtual method #4 @ RVA 0x0047ebd0 (45 instructions, 12 basic blocks)
    pub fn method_04(&mut self) {
        // Writes to field at offset +0x0004
        // Writes to field at offset +0x02da
        // Writes to field at offset +0x0314
        // Writes to field at offset +0x03e0
    }

    /// Decompiled virtual method #5 @ RVA 0x0047f370 (124 instructions, 30 basic blocks)
    pub fn method_05(&mut self) {
        // Writes to field at offset +0x0040
    }

    /// Decompiled virtual method #6 @ RVA 0x004b4ec0 (103 instructions, 17 basic blocks)
    pub fn method_06(&mut self) {
    }

    /// Decompiled virtual method #7 @ RVA 0x000d3090 (1 instructions, 1 basic blocks)
    pub fn method_07(&mut self) {
    }

    /// Decompiled virtual method #8 @ RVA 0x000d3090 (1 instructions, 1 basic blocks)
    pub fn method_08(&mut self) {
    }

    /// Decompiled virtual method #9 @ RVA 0x004b8840 (23 instructions, 7 basic blocks)
    pub fn method_09(&mut self) {
        // Writes to field at offset +0x0000
        // Writes to field at offset +0x0008
    }

    /// Decompiled virtual method #10 @ RVA 0x0047f200 (58 instructions, 4 basic blocks)
    pub fn method_10(&mut self) {
    }

    /// Decompiled virtual method #11 @ RVA 0x0047f0e0 (15 instructions, 4 basic blocks)
    pub fn method_11(&mut self) {
    }

    /// Decompiled virtual method #12 @ RVA 0x0045eec0 (67 instructions, 22 basic blocks)
    pub fn method_12(&mut self) {
        // Writes to field at offset +0x001c
        // Writes to field at offset +0x0040
        // Writes to field at offset +0x0090
        // Writes to field at offset +0x00b0
    }

    /// Decompiled virtual method #13 @ RVA 0x0045efd0 (27 instructions, 5 basic blocks)
    pub fn method_13(&mut self) {
        // Writes to field at offset +0x0090
        // Writes to field at offset +0x00b0
    }

    /// Decompiled virtual method #14 @ RVA 0x004b2f50 (105 instructions, 2 basic blocks)
    pub fn method_14(&mut self) {
        // Writes to field at offset +0x0000
        // Writes to field at offset +0x0004
        // Writes to field at offset +0x0008
        // Writes to field at offset +0x000a
        // Writes to field at offset +0x00c4
        // Writes to field at offset +0x020c
        // Writes to field at offset +0x0214
        // Writes to field at offset +0x021c
        // Writes to field at offset +0x0224
        // Writes to field at offset +0x022c
        // Writes to field at offset +0x0234
        // Writes to field at offset +0x023c
        // Writes to field at offset +0x0244
        // Writes to field at offset +0x024c
        // Writes to field at offset +0x0254
    }

    /// Decompiled virtual method #15 @ RVA 0x004b66c0 (44 instructions, 12 basic blocks)
    pub fn method_15(&mut self) {
        // Writes to field at offset +0x0000
        // Writes to field at offset +0x0004
        // Writes to field at offset +0x0008
        // Writes to field at offset +0x000a
        // Writes to field at offset +0x00c4
        // Writes to field at offset +0x01e4
        // Writes to field at offset +0x02fc
    }

    /// Decompiled virtual method #16 @ RVA 0x004b6040 (54 instructions, 12 basic blocks)
    pub fn method_16(&mut self) {
    }

    /// Decompiled virtual method #17 @ RVA 0x004b80d0 (38 instructions, 8 basic blocks)
    pub fn method_17(&mut self) {
    }

    /// Decompiled virtual method #18 @ RVA 0x000d3090 (1 instructions, 1 basic blocks)
    pub fn method_18(&mut self) {
    }

    /// Decompiled virtual method #19 @ RVA 0x000d3090 (1 instructions, 1 basic blocks)
    pub fn method_19(&mut self) {
    }

    /// Decompiled virtual method #20 @ RVA 0x000d3090 (1 instructions, 1 basic blocks)
    pub fn method_20(&mut self) {
    }

    /// Decompiled virtual method #21 @ RVA 0x004b8980 (46 instructions, 15 basic blocks)
    pub fn method_21(&mut self) {
        // Writes to field at offset +0x01e4
        // Writes to field at offset +0x02fc
    }

    /// Decompiled virtual method #22 @ RVA 0x004b2920 (22 instructions, 3 basic blocks)
    pub fn method_22(&mut self) {
        // Writes to field at offset +0x01b0
    }

    /// Decompiled virtual method #23 @ RVA 0x004b8820 (4 instructions, 1 basic blocks)
    pub fn method_23(&mut self) {
        // Writes to field at offset +0x03e0
    }

}

impl CcClass for ccGobjEnemyShield {
    fn class_name(&self) -> &'static str { Self::CLASS_NAME }
    fn vtable_rva(&self) -> u32 { Self::VTABLE_RVA }
    fn virtual_method_count(&self) -> usize { Self::METHOD_COUNT }
    fn base_classes(&self) -> &'static [&'static str] { Self::BASE_CLASSES }
}

/// Decompiled CC2 class: `ccGobjGimAnimal` (VTable RVA: 0x008be678, 14 virtual methods, 80 recovered fields)
/// Inherits: ccGameObj
#[derive(Debug, Clone, Default)]
pub struct ccGobjGimAnimal {
    pub vtable_rva: u32,
    pub field_0x0006: u8,
    pub field_0x0010: u64,
    pub field_0x0018: u64,
    pub field_0x001c: u32,
    pub field_0x0020: u32,
    pub field_0x0024: u32,
    pub field_0x0028: u32,
    pub field_0x002c: u32,
    pub field_0x0030: f32,
    pub field_0x0040: u32,
    pub field_0x0050: u64,
    pub field_0x0090: u32,
    pub field_0x0094: u32,
    pub field_0x0098: u32,
    pub field_0x009c: u32,
    pub field_0x00a0: u32,
    pub field_0x00a4: u32,
    pub field_0x00a8: u32,
    pub field_0x00ac: u32,
    pub field_0x00b0: u64,
    pub field_0x00b8: u64,
    pub field_0x00c4: u32,
    pub field_0x0114: u32,
    pub field_0x0120: u32,
    pub field_0x0124: u32,
    pub field_0x0128: u32,
    pub field_0x012c: u32,
    pub field_0x0130: u32,
    pub field_0x0134: u32,
    pub field_0x0138: u32,
    pub field_0x013c: u32,
    pub field_0x0140: u32,
    pub field_0x0144: u32,
    pub field_0x0148: u32,
    pub field_0x014c: u32,
    pub field_0x0150: u32,
    pub field_0x0154: u32,
    pub field_0x0158: u32,
    pub field_0x015c: u32,
    pub field_0x01d0: u64,
    pub field_0x0200: u32,
    pub field_0x0204: u32,
    pub field_0x0208: u32,
    pub field_0x020c: u32,
    pub field_0x0210: u32,
    pub field_0x0214: u32,
    pub field_0x0218: u32,
    pub field_0x021c: u32,
    pub field_0x0220: u32,
    pub field_0x0224: u32,
    pub field_0x0228: u32,
    pub field_0x022c: u32,
    pub field_0x0230: u32,
    pub field_0x0234: u32,
    pub field_0x0238: u32,
    pub field_0x023c: u32,
    pub field_0x0240: u32,
    pub field_0x0244: u32,
    pub field_0x0248: u32,
    pub field_0x024c: u32,
    pub field_0x0250: u32,
    pub field_0x0254: u32,
    pub field_0x0258: u32,
    pub field_0x025c: u32,
    pub field_0x0260: u32,
    pub field_0x0264: u32,
    pub field_0x0268: u32,
    pub field_0x026c: u32,
    pub field_0x0270: u32,
    pub field_0x0274: u32,
    pub field_0x0278: u32,
    pub field_0x027c: u32,
    pub field_0x0280: u32,
    pub field_0x0284: u32,
    pub field_0x0288: u32,
    pub field_0x028c: u32,
    pub field_0x0290: u32,
    pub field_0x0294: u32,
    pub field_0x0298: u32,
    pub field_0x029c: u32,
}

impl ccGobjGimAnimal {
    pub const CLASS_NAME: &'static str = "ccGobjGimAnimal";
    pub const VTABLE_RVA: u32 = 0x008be678;
    pub const METHOD_COUNT: usize = 14;
    pub const BASE_CLASSES: &'static [&'static str] = &["ccGameObj"];
    pub const VIRTUAL_METHODS: &'static [u64] = &[0x00000001804e5e10, 0x00000001804e6000, 0x000000018045e4c0, 0x00000001800d3090, 0x000000018019e550, 0x00000001804e76d0, 0x00000001804e6ec0, 0x00000001800d3090, 0x00000001800d3090, 0x00000001804e6f70, 0x00000001804e6910, 0x000000018045e2d0, 0x000000018045eec0, 0x000000018045efd0];

    pub fn new() -> Self {
        Self {
            vtable_rva: Self::VTABLE_RVA,
            field_0x0006: Default::default(),
            field_0x0010: Default::default(),
            field_0x0018: Default::default(),
            field_0x001c: Default::default(),
            field_0x0020: Default::default(),
            field_0x0024: Default::default(),
            field_0x0028: Default::default(),
            field_0x002c: Default::default(),
            field_0x0030: Default::default(),
            field_0x0040: Default::default(),
            field_0x0050: Default::default(),
            field_0x0090: Default::default(),
            field_0x0094: Default::default(),
            field_0x0098: Default::default(),
            field_0x009c: Default::default(),
            field_0x00a0: Default::default(),
            field_0x00a4: Default::default(),
            field_0x00a8: Default::default(),
            field_0x00ac: Default::default(),
            field_0x00b0: Default::default(),
            field_0x00b8: Default::default(),
            field_0x00c4: Default::default(),
            field_0x0114: Default::default(),
            field_0x0120: Default::default(),
            field_0x0124: Default::default(),
            field_0x0128: Default::default(),
            field_0x012c: Default::default(),
            field_0x0130: Default::default(),
            field_0x0134: Default::default(),
            field_0x0138: Default::default(),
            field_0x013c: Default::default(),
            field_0x0140: Default::default(),
            field_0x0144: Default::default(),
            field_0x0148: Default::default(),
            field_0x014c: Default::default(),
            field_0x0150: Default::default(),
            field_0x0154: Default::default(),
            field_0x0158: Default::default(),
            field_0x015c: Default::default(),
            field_0x01d0: Default::default(),
            field_0x0200: Default::default(),
            field_0x0204: Default::default(),
            field_0x0208: Default::default(),
            field_0x020c: Default::default(),
            field_0x0210: Default::default(),
            field_0x0214: Default::default(),
            field_0x0218: Default::default(),
            field_0x021c: Default::default(),
            field_0x0220: Default::default(),
            field_0x0224: Default::default(),
            field_0x0228: Default::default(),
            field_0x022c: Default::default(),
            field_0x0230: Default::default(),
            field_0x0234: Default::default(),
            field_0x0238: Default::default(),
            field_0x023c: Default::default(),
            field_0x0240: Default::default(),
            field_0x0244: Default::default(),
            field_0x0248: Default::default(),
            field_0x024c: Default::default(),
            field_0x0250: Default::default(),
            field_0x0254: Default::default(),
            field_0x0258: Default::default(),
            field_0x025c: Default::default(),
            field_0x0260: Default::default(),
            field_0x0264: Default::default(),
            field_0x0268: Default::default(),
            field_0x026c: Default::default(),
            field_0x0270: Default::default(),
            field_0x0274: Default::default(),
            field_0x0278: Default::default(),
            field_0x027c: Default::default(),
            field_0x0280: Default::default(),
            field_0x0284: Default::default(),
            field_0x0288: Default::default(),
            field_0x028c: Default::default(),
            field_0x0290: Default::default(),
            field_0x0294: Default::default(),
            field_0x0298: Default::default(),
            field_0x029c: Default::default(),
        }
    }

    /// Decompiled virtual method #0 @ RVA 0x004e5e10 (16 instructions, 3 basic blocks)
    pub fn method_00(&mut self) {
    }

    /// Decompiled virtual method #1 @ RVA 0x004e6000 (108 instructions, 16 basic blocks)
    pub fn method_01(&mut self) {
        // Writes to field at offset +0x0006
    }

    /// Decompiled virtual method #2 @ RVA 0x0045e4c0 (42 instructions, 4 basic blocks)
    pub fn method_02(&mut self) {
        // Writes to field at offset +0x0090
        // Writes to field at offset +0x0094
        // Writes to field at offset +0x0098
        // Writes to field at offset +0x009c
        // Writes to field at offset +0x00a0
        // Writes to field at offset +0x00a4
        // Writes to field at offset +0x00a8
        // Writes to field at offset +0x00ac
        // Writes to field at offset +0x0140
        // Writes to field at offset +0x0144
        // Writes to field at offset +0x0148
        // Writes to field at offset +0x014c
        // Writes to field at offset +0x0150
        // Writes to field at offset +0x0154
        // Writes to field at offset +0x0158
        // Writes to field at offset +0x015c
    }

    /// Decompiled virtual method #3 @ RVA 0x000d3090 (1 instructions, 1 basic blocks)
    pub fn method_03(&mut self) {
    }

    /// Decompiled virtual method #4 @ RVA 0x0019e550 (2 instructions, 1 basic blocks)
    pub fn method_04(&mut self) {
        // Writes to field at offset +0x00c4
    }

    /// Decompiled virtual method #5 @ RVA 0x004e76d0 (106 instructions, 10 basic blocks)
    pub fn method_05(&mut self) {
        // Writes to field at offset +0x0220
        // Writes to field at offset +0x0224
        // Writes to field at offset +0x0228
        // Writes to field at offset +0x022c
        // Writes to field at offset +0x0230
        // Writes to field at offset +0x0234
        // Writes to field at offset +0x0238
        // Writes to field at offset +0x023c
        // Writes to field at offset +0x0240
        // Writes to field at offset +0x0244
        // Writes to field at offset +0x0248
        // Writes to field at offset +0x024c
        // Writes to field at offset +0x0250
        // Writes to field at offset +0x0254
        // Writes to field at offset +0x0258
        // Writes to field at offset +0x025c
        // Writes to field at offset +0x0260
        // Writes to field at offset +0x0264
        // Writes to field at offset +0x0268
        // Writes to field at offset +0x026c
        // Writes to field at offset +0x0270
        // Writes to field at offset +0x0274
        // Writes to field at offset +0x0278
        // Writes to field at offset +0x027c
        // Writes to field at offset +0x0280
        // Writes to field at offset +0x0284
        // Writes to field at offset +0x0288
        // Writes to field at offset +0x028c
        // Writes to field at offset +0x0290
        // Writes to field at offset +0x0294
        // Writes to field at offset +0x0298
        // Writes to field at offset +0x029c
    }

    /// Decompiled virtual method #6 @ RVA 0x004e6ec0 (33 instructions, 3 basic blocks)
    pub fn method_06(&mut self) {
    }

    /// Decompiled virtual method #7 @ RVA 0x000d3090 (1 instructions, 1 basic blocks)
    pub fn method_07(&mut self) {
    }

    /// Decompiled virtual method #8 @ RVA 0x000d3090 (1 instructions, 1 basic blocks)
    pub fn method_08(&mut self) {
    }

    /// Decompiled virtual method #9 @ RVA 0x004e6f70 (115 instructions, 31 basic blocks)
    pub fn method_09(&mut self) {
    }

    /// Decompiled virtual method #10 @ RVA 0x004e6910 (84 instructions, 5 basic blocks)
    pub fn method_10(&mut self) {
        // Writes to field at offset +0x0090
        // Writes to field at offset +0x00a0
        // Writes to field at offset +0x00a4
        // Writes to field at offset +0x00a8
        // Writes to field at offset +0x00ac
        // Writes to field at offset +0x0114
    }

    /// Decompiled virtual method #11 @ RVA 0x0045e2d0 (33 instructions, 3 basic blocks)
    pub fn method_11(&mut self) {
        // Writes to field at offset +0x0090
        // Writes to field at offset +0x00a0
        // Writes to field at offset +0x00a4
        // Writes to field at offset +0x00a8
        // Writes to field at offset +0x00ac
    }

    /// Decompiled virtual method #12 @ RVA 0x0045eec0 (67 instructions, 22 basic blocks)
    pub fn method_12(&mut self) {
        // Writes to field at offset +0x001c
        // Writes to field at offset +0x0040
        // Writes to field at offset +0x0090
        // Writes to field at offset +0x00b0
    }

    /// Decompiled virtual method #13 @ RVA 0x0045efd0 (27 instructions, 5 basic blocks)
    pub fn method_13(&mut self) {
        // Writes to field at offset +0x0090
        // Writes to field at offset +0x00b0
    }

}

impl CcClass for ccGobjGimAnimal {
    fn class_name(&self) -> &'static str { Self::CLASS_NAME }
    fn vtable_rva(&self) -> u32 { Self::VTABLE_RVA }
    fn virtual_method_count(&self) -> usize { Self::METHOD_COUNT }
    fn base_classes(&self) -> &'static [&'static str] { Self::BASE_CLASSES }
}

/// Decompiled CC2 class: `ccGobjGimArenaScreenMenu` (VTable RVA: 0x008be8d0, 14 virtual methods, 34 recovered fields)
/// Inherits: ccGobjGimBase, ccGameObj
#[derive(Debug, Clone, Default)]
pub struct ccGobjGimArenaScreenMenu {
    pub vtable_rva: u32,
    pub field_0x0010: u32,
    pub field_0x0018: u64,
    pub field_0x001c: u8,
    pub field_0x0020: u32,
    pub field_0x0024: u16,
    pub field_0x002c: u16,
    pub field_0x0040: u32,
    pub field_0x0090: u32,
    pub field_0x0094: u32,
    pub field_0x0098: u32,
    pub field_0x009c: u32,
    pub field_0x00a0: u32,
    pub field_0x00a4: u32,
    pub field_0x00a8: u32,
    pub field_0x00ac: u32,
    pub field_0x00b0: u64,
    pub field_0x00b8: u64,
    pub field_0x00c4: u32,
    pub field_0x0120: u32,
    pub field_0x0124: u32,
    pub field_0x0128: u32,
    pub field_0x012c: u32,
    pub field_0x0130: u32,
    pub field_0x0134: u32,
    pub field_0x0138: u32,
    pub field_0x013c: u32,
    pub field_0x0140: u32,
    pub field_0x0144: u32,
    pub field_0x0148: u32,
    pub field_0x014c: u32,
    pub field_0x0150: u32,
    pub field_0x0154: u32,
    pub field_0x0158: u32,
    pub field_0x015c: u32,
}

impl ccGobjGimArenaScreenMenu {
    pub const CLASS_NAME: &'static str = "ccGobjGimArenaScreenMenu";
    pub const VTABLE_RVA: u32 = 0x008be8d0;
    pub const METHOD_COUNT: usize = 14;
    pub const BASE_CLASSES: &'static [&'static str] = &["ccGobjGimBase", "ccGameObj"];
    pub const VIRTUAL_METHODS: &'static [u64] = &[0x00000001804ca230, 0x00000001804d9840, 0x000000018045e4c0, 0x00000001800d3090, 0x000000018019e550, 0x00000001804d99c0, 0x00000001800d3090, 0x00000001800d3090, 0x00000001800d3090, 0x00000001800d3090, 0x00000001800d3090, 0x000000018045e2d0, 0x000000018045eec0, 0x000000018045efd0];

    pub fn new() -> Self {
        Self {
            vtable_rva: Self::VTABLE_RVA,
            field_0x0010: Default::default(),
            field_0x0018: Default::default(),
            field_0x001c: Default::default(),
            field_0x0020: Default::default(),
            field_0x0024: Default::default(),
            field_0x002c: Default::default(),
            field_0x0040: Default::default(),
            field_0x0090: Default::default(),
            field_0x0094: Default::default(),
            field_0x0098: Default::default(),
            field_0x009c: Default::default(),
            field_0x00a0: Default::default(),
            field_0x00a4: Default::default(),
            field_0x00a8: Default::default(),
            field_0x00ac: Default::default(),
            field_0x00b0: Default::default(),
            field_0x00b8: Default::default(),
            field_0x00c4: Default::default(),
            field_0x0120: Default::default(),
            field_0x0124: Default::default(),
            field_0x0128: Default::default(),
            field_0x012c: Default::default(),
            field_0x0130: Default::default(),
            field_0x0134: Default::default(),
            field_0x0138: Default::default(),
            field_0x013c: Default::default(),
            field_0x0140: Default::default(),
            field_0x0144: Default::default(),
            field_0x0148: Default::default(),
            field_0x014c: Default::default(),
            field_0x0150: Default::default(),
            field_0x0154: Default::default(),
            field_0x0158: Default::default(),
            field_0x015c: Default::default(),
        }
    }

    /// Decompiled virtual method #0 @ RVA 0x004ca230 (16 instructions, 3 basic blocks)
    pub fn method_00(&mut self) {
    }

    /// Decompiled virtual method #1 @ RVA 0x004d9840 (13 instructions, 3 basic blocks)
    pub fn method_01(&mut self) {
    }

    /// Decompiled virtual method #2 @ RVA 0x0045e4c0 (42 instructions, 4 basic blocks)
    pub fn method_02(&mut self) {
        // Writes to field at offset +0x0090
        // Writes to field at offset +0x0094
        // Writes to field at offset +0x0098
        // Writes to field at offset +0x009c
        // Writes to field at offset +0x00a0
        // Writes to field at offset +0x00a4
        // Writes to field at offset +0x00a8
        // Writes to field at offset +0x00ac
        // Writes to field at offset +0x0140
        // Writes to field at offset +0x0144
        // Writes to field at offset +0x0148
        // Writes to field at offset +0x014c
        // Writes to field at offset +0x0150
        // Writes to field at offset +0x0154
        // Writes to field at offset +0x0158
        // Writes to field at offset +0x015c
    }

    /// Decompiled virtual method #3 @ RVA 0x000d3090 (1 instructions, 1 basic blocks)
    pub fn method_03(&mut self) {
    }

    /// Decompiled virtual method #4 @ RVA 0x0019e550 (2 instructions, 1 basic blocks)
    pub fn method_04(&mut self) {
        // Writes to field at offset +0x00c4
    }

    /// Decompiled virtual method #5 @ RVA 0x004d99c0 (114 instructions, 26 basic blocks)
    pub fn method_05(&mut self) {
        // Writes to field at offset +0x0024
    }

    /// Decompiled virtual method #6 @ RVA 0x000d3090 (1 instructions, 1 basic blocks)
    pub fn method_06(&mut self) {
    }

    /// Decompiled virtual method #7 @ RVA 0x000d3090 (1 instructions, 1 basic blocks)
    pub fn method_07(&mut self) {
    }

    /// Decompiled virtual method #8 @ RVA 0x000d3090 (1 instructions, 1 basic blocks)
    pub fn method_08(&mut self) {
    }

    /// Decompiled virtual method #9 @ RVA 0x000d3090 (1 instructions, 1 basic blocks)
    pub fn method_09(&mut self) {
    }

    /// Decompiled virtual method #10 @ RVA 0x000d3090 (1 instructions, 1 basic blocks)
    pub fn method_10(&mut self) {
    }

    /// Decompiled virtual method #11 @ RVA 0x0045e2d0 (33 instructions, 3 basic blocks)
    pub fn method_11(&mut self) {
        // Writes to field at offset +0x0090
        // Writes to field at offset +0x00a0
        // Writes to field at offset +0x00a4
        // Writes to field at offset +0x00a8
        // Writes to field at offset +0x00ac
    }

    /// Decompiled virtual method #12 @ RVA 0x0045eec0 (67 instructions, 22 basic blocks)
    pub fn method_12(&mut self) {
        // Writes to field at offset +0x001c
        // Writes to field at offset +0x0040
        // Writes to field at offset +0x0090
        // Writes to field at offset +0x00b0
    }

    /// Decompiled virtual method #13 @ RVA 0x0045efd0 (27 instructions, 5 basic blocks)
    pub fn method_13(&mut self) {
        // Writes to field at offset +0x0090
        // Writes to field at offset +0x00b0
    }

}

impl CcClass for ccGobjGimArenaScreenMenu {
    fn class_name(&self) -> &'static str { Self::CLASS_NAME }
    fn vtable_rva(&self) -> u32 { Self::VTABLE_RVA }
    fn virtual_method_count(&self) -> usize { Self::METHOD_COUNT }
    fn base_classes(&self) -> &'static [&'static str] { Self::BASE_CLASSES }
}

/// Decompiled CC2 class: `ccGobjGimBase` (VTable RVA: 0x008be150, 14 virtual methods, 39 recovered fields)
/// Inherits: ccGameObj
#[derive(Debug, Clone, Default)]
pub struct ccGobjGimBase {
    pub vtable_rva: u32,
    pub field_0x0000: u32,
    pub field_0x0004: u32,
    pub field_0x0008: u16,
    pub field_0x000a: u16,
    pub field_0x0010: u64,
    pub field_0x0018: u64,
    pub field_0x001c: u8,
    pub field_0x0020: u32,
    pub field_0x0024: u16,
    pub field_0x002c: u16,
    pub field_0x0040: u32,
    pub field_0x0090: u32,
    pub field_0x0094: u32,
    pub field_0x0098: u32,
    pub field_0x009c: u32,
    pub field_0x00a0: u32,
    pub field_0x00a4: u32,
    pub field_0x00a8: u32,
    pub field_0x00ac: u32,
    pub field_0x00b0: u64,
    pub field_0x00b8: u64,
    pub field_0x00c0: u16,
    pub field_0x00c4: u32,
    pub field_0x0120: u32,
    pub field_0x0124: u32,
    pub field_0x0128: u32,
    pub field_0x012c: u32,
    pub field_0x0130: u32,
    pub field_0x0134: u32,
    pub field_0x0138: u32,
    pub field_0x013c: u32,
    pub field_0x0140: u32,
    pub field_0x0144: u32,
    pub field_0x0148: u32,
    pub field_0x014c: u32,
    pub field_0x0150: u32,
    pub field_0x0154: u32,
    pub field_0x0158: u32,
    pub field_0x015c: u32,
}

impl ccGobjGimBase {
    pub const CLASS_NAME: &'static str = "ccGobjGimBase";
    pub const VTABLE_RVA: u32 = 0x008be150;
    pub const METHOD_COUNT: usize = 14;
    pub const BASE_CLASSES: &'static [&'static str] = &["ccGameObj"];
    pub const VIRTUAL_METHODS: &'static [u64] = &[0x00000001804ca230, 0x00000001804fd4b0, 0x000000018045e4c0, 0x00000001800d3090, 0x000000018019e550, 0x00000001804fd5c0, 0x00000001800d3090, 0x00000001800d3090, 0x00000001800d3090, 0x00000001800d3090, 0x000000018045e220, 0x000000018045e2d0, 0x000000018045eec0, 0x000000018045efd0];

    pub fn new() -> Self {
        Self {
            vtable_rva: Self::VTABLE_RVA,
            field_0x0000: Default::default(),
            field_0x0004: Default::default(),
            field_0x0008: Default::default(),
            field_0x000a: Default::default(),
            field_0x0010: Default::default(),
            field_0x0018: Default::default(),
            field_0x001c: Default::default(),
            field_0x0020: Default::default(),
            field_0x0024: Default::default(),
            field_0x002c: Default::default(),
            field_0x0040: Default::default(),
            field_0x0090: Default::default(),
            field_0x0094: Default::default(),
            field_0x0098: Default::default(),
            field_0x009c: Default::default(),
            field_0x00a0: Default::default(),
            field_0x00a4: Default::default(),
            field_0x00a8: Default::default(),
            field_0x00ac: Default::default(),
            field_0x00b0: Default::default(),
            field_0x00b8: Default::default(),
            field_0x00c0: Default::default(),
            field_0x00c4: Default::default(),
            field_0x0120: Default::default(),
            field_0x0124: Default::default(),
            field_0x0128: Default::default(),
            field_0x012c: Default::default(),
            field_0x0130: Default::default(),
            field_0x0134: Default::default(),
            field_0x0138: Default::default(),
            field_0x013c: Default::default(),
            field_0x0140: Default::default(),
            field_0x0144: Default::default(),
            field_0x0148: Default::default(),
            field_0x014c: Default::default(),
            field_0x0150: Default::default(),
            field_0x0154: Default::default(),
            field_0x0158: Default::default(),
            field_0x015c: Default::default(),
        }
    }

    /// Decompiled virtual method #0 @ RVA 0x004ca230 (16 instructions, 3 basic blocks)
    pub fn method_00(&mut self) {
    }

    /// Decompiled virtual method #1 @ RVA 0x004fd4b0 (11 instructions, 3 basic blocks)
    pub fn method_01(&mut self) {
    }

    /// Decompiled virtual method #2 @ RVA 0x0045e4c0 (42 instructions, 4 basic blocks)
    pub fn method_02(&mut self) {
        // Writes to field at offset +0x0090
        // Writes to field at offset +0x0094
        // Writes to field at offset +0x0098
        // Writes to field at offset +0x009c
        // Writes to field at offset +0x00a0
        // Writes to field at offset +0x00a4
        // Writes to field at offset +0x00a8
        // Writes to field at offset +0x00ac
        // Writes to field at offset +0x0140
        // Writes to field at offset +0x0144
        // Writes to field at offset +0x0148
        // Writes to field at offset +0x014c
        // Writes to field at offset +0x0150
        // Writes to field at offset +0x0154
        // Writes to field at offset +0x0158
        // Writes to field at offset +0x015c
    }

    /// Decompiled virtual method #3 @ RVA 0x000d3090 (1 instructions, 1 basic blocks)
    pub fn method_03(&mut self) {
    }

    /// Decompiled virtual method #4 @ RVA 0x0019e550 (2 instructions, 1 basic blocks)
    pub fn method_04(&mut self) {
        // Writes to field at offset +0x00c4
    }

    /// Decompiled virtual method #5 @ RVA 0x004fd5c0 (136 instructions, 36 basic blocks)
    pub fn method_05(&mut self) {
        // Writes to field at offset +0x0000
        // Writes to field at offset +0x0004
        // Writes to field at offset +0x0008
        // Writes to field at offset +0x000a
        // Writes to field at offset +0x0024
        // Writes to field at offset +0x00b0
        // Writes to field at offset +0x00c0
    }

    /// Decompiled virtual method #6 @ RVA 0x000d3090 (1 instructions, 1 basic blocks)
    pub fn method_06(&mut self) {
    }

    /// Decompiled virtual method #7 @ RVA 0x000d3090 (1 instructions, 1 basic blocks)
    pub fn method_07(&mut self) {
    }

    /// Decompiled virtual method #8 @ RVA 0x000d3090 (1 instructions, 1 basic blocks)
    pub fn method_08(&mut self) {
    }

    /// Decompiled virtual method #9 @ RVA 0x000d3090 (1 instructions, 1 basic blocks)
    pub fn method_09(&mut self) {
    }

    /// Decompiled virtual method #10 @ RVA 0x0045e220 (31 instructions, 3 basic blocks)
    pub fn method_10(&mut self) {
        // Writes to field at offset +0x0090
        // Writes to field at offset +0x00a0
        // Writes to field at offset +0x00a4
        // Writes to field at offset +0x00a8
        // Writes to field at offset +0x00ac
    }

    /// Decompiled virtual method #11 @ RVA 0x0045e2d0 (33 instructions, 3 basic blocks)
    pub fn method_11(&mut self) {
        // Writes to field at offset +0x0090
        // Writes to field at offset +0x00a0
        // Writes to field at offset +0x00a4
        // Writes to field at offset +0x00a8
        // Writes to field at offset +0x00ac
    }

    /// Decompiled virtual method #12 @ RVA 0x0045eec0 (67 instructions, 22 basic blocks)
    pub fn method_12(&mut self) {
        // Writes to field at offset +0x001c
        // Writes to field at offset +0x0040
        // Writes to field at offset +0x0090
        // Writes to field at offset +0x00b0
    }

    /// Decompiled virtual method #13 @ RVA 0x0045efd0 (27 instructions, 5 basic blocks)
    pub fn method_13(&mut self) {
        // Writes to field at offset +0x0090
        // Writes to field at offset +0x00b0
    }

}

impl CcClass for ccGobjGimBase {
    fn class_name(&self) -> &'static str { Self::CLASS_NAME }
    fn vtable_rva(&self) -> u32 { Self::VTABLE_RVA }
    fn virtual_method_count(&self) -> usize { Self::METHOD_COUNT }
    fn base_classes(&self) -> &'static [&'static str] { Self::BASE_CLASSES }
}

/// Decompiled CC2 class: `ccGobjGimBlueLight` (VTable RVA: 0x008be6f0, 14 virtual methods, 35 recovered fields)
/// Inherits: ccGobjGimBase, ccGameObj
#[derive(Debug, Clone, Default)]
pub struct ccGobjGimBlueLight {
    pub vtable_rva: u32,
    pub field_0x0000: u64,
    pub field_0x0010: u64,
    pub field_0x0018: u64,
    pub field_0x001c: u8,
    pub field_0x0020: u32,
    pub field_0x0024: u16,
    pub field_0x002c: u16,
    pub field_0x0040: u32,
    pub field_0x0090: u32,
    pub field_0x0094: u32,
    pub field_0x0098: u32,
    pub field_0x009c: u32,
    pub field_0x00a0: u32,
    pub field_0x00a4: u32,
    pub field_0x00a8: u32,
    pub field_0x00ac: u32,
    pub field_0x00b0: u64,
    pub field_0x00b8: u64,
    pub field_0x00c4: u32,
    pub field_0x0120: u32,
    pub field_0x0124: u32,
    pub field_0x0128: u32,
    pub field_0x012c: u32,
    pub field_0x0130: u32,
    pub field_0x0134: u32,
    pub field_0x0138: u32,
    pub field_0x013c: u32,
    pub field_0x0140: u32,
    pub field_0x0144: u32,
    pub field_0x0148: u32,
    pub field_0x014c: u32,
    pub field_0x0150: u32,
    pub field_0x0154: u32,
    pub field_0x0158: u32,
    pub field_0x015c: u32,
}

impl ccGobjGimBlueLight {
    pub const CLASS_NAME: &'static str = "ccGobjGimBlueLight";
    pub const VTABLE_RVA: u32 = 0x008be6f0;
    pub const METHOD_COUNT: usize = 14;
    pub const BASE_CLASSES: &'static [&'static str] = &["ccGobjGimBase", "ccGameObj"];
    pub const VIRTUAL_METHODS: &'static [u64] = &[0x00000001804c8e90, 0x00000001804dbcc0, 0x000000018045e4c0, 0x00000001800d3090, 0x000000018019e550, 0x00000001804d6380, 0x00000001800d3090, 0x00000001800d3090, 0x00000001800d3090, 0x00000001800d3090, 0x00000001804dbd90, 0x000000018045e2d0, 0x000000018045eec0, 0x000000018045efd0];

    pub fn new() -> Self {
        Self {
            vtable_rva: Self::VTABLE_RVA,
            field_0x0000: Default::default(),
            field_0x0010: Default::default(),
            field_0x0018: Default::default(),
            field_0x001c: Default::default(),
            field_0x0020: Default::default(),
            field_0x0024: Default::default(),
            field_0x002c: Default::default(),
            field_0x0040: Default::default(),
            field_0x0090: Default::default(),
            field_0x0094: Default::default(),
            field_0x0098: Default::default(),
            field_0x009c: Default::default(),
            field_0x00a0: Default::default(),
            field_0x00a4: Default::default(),
            field_0x00a8: Default::default(),
            field_0x00ac: Default::default(),
            field_0x00b0: Default::default(),
            field_0x00b8: Default::default(),
            field_0x00c4: Default::default(),
            field_0x0120: Default::default(),
            field_0x0124: Default::default(),
            field_0x0128: Default::default(),
            field_0x012c: Default::default(),
            field_0x0130: Default::default(),
            field_0x0134: Default::default(),
            field_0x0138: Default::default(),
            field_0x013c: Default::default(),
            field_0x0140: Default::default(),
            field_0x0144: Default::default(),
            field_0x0148: Default::default(),
            field_0x014c: Default::default(),
            field_0x0150: Default::default(),
            field_0x0154: Default::default(),
            field_0x0158: Default::default(),
            field_0x015c: Default::default(),
        }
    }

    /// Decompiled virtual method #0 @ RVA 0x004c8e90 (16 instructions, 3 basic blocks)
    pub fn method_00(&mut self) {
    }

    /// Decompiled virtual method #1 @ RVA 0x004dbcc0 (11 instructions, 3 basic blocks)
    pub fn method_01(&mut self) {
    }

    /// Decompiled virtual method #2 @ RVA 0x0045e4c0 (42 instructions, 4 basic blocks)
    pub fn method_02(&mut self) {
        // Writes to field at offset +0x0090
        // Writes to field at offset +0x0094
        // Writes to field at offset +0x0098
        // Writes to field at offset +0x009c
        // Writes to field at offset +0x00a0
        // Writes to field at offset +0x00a4
        // Writes to field at offset +0x00a8
        // Writes to field at offset +0x00ac
        // Writes to field at offset +0x0140
        // Writes to field at offset +0x0144
        // Writes to field at offset +0x0148
        // Writes to field at offset +0x014c
        // Writes to field at offset +0x0150
        // Writes to field at offset +0x0154
        // Writes to field at offset +0x0158
        // Writes to field at offset +0x015c
    }

    /// Decompiled virtual method #3 @ RVA 0x000d3090 (1 instructions, 1 basic blocks)
    pub fn method_03(&mut self) {
    }

    /// Decompiled virtual method #4 @ RVA 0x0019e550 (2 instructions, 1 basic blocks)
    pub fn method_04(&mut self) {
        // Writes to field at offset +0x00c4
    }

    /// Decompiled virtual method #5 @ RVA 0x004d6380 (85 instructions, 25 basic blocks)
    pub fn method_05(&mut self) {
        // Writes to field at offset +0x0000
        // Writes to field at offset +0x0024
    }

    /// Decompiled virtual method #6 @ RVA 0x000d3090 (1 instructions, 1 basic blocks)
    pub fn method_06(&mut self) {
    }

    /// Decompiled virtual method #7 @ RVA 0x000d3090 (1 instructions, 1 basic blocks)
    pub fn method_07(&mut self) {
    }

    /// Decompiled virtual method #8 @ RVA 0x000d3090 (1 instructions, 1 basic blocks)
    pub fn method_08(&mut self) {
    }

    /// Decompiled virtual method #9 @ RVA 0x000d3090 (1 instructions, 1 basic blocks)
    pub fn method_09(&mut self) {
    }

    /// Decompiled virtual method #10 @ RVA 0x004dbd90 (102 instructions, 7 basic blocks)
    pub fn method_10(&mut self) {
    }

    /// Decompiled virtual method #11 @ RVA 0x0045e2d0 (33 instructions, 3 basic blocks)
    pub fn method_11(&mut self) {
        // Writes to field at offset +0x0090
        // Writes to field at offset +0x00a0
        // Writes to field at offset +0x00a4
        // Writes to field at offset +0x00a8
        // Writes to field at offset +0x00ac
    }

    /// Decompiled virtual method #12 @ RVA 0x0045eec0 (67 instructions, 22 basic blocks)
    pub fn method_12(&mut self) {
        // Writes to field at offset +0x001c
        // Writes to field at offset +0x0040
        // Writes to field at offset +0x0090
        // Writes to field at offset +0x00b0
    }

    /// Decompiled virtual method #13 @ RVA 0x0045efd0 (27 instructions, 5 basic blocks)
    pub fn method_13(&mut self) {
        // Writes to field at offset +0x0090
        // Writes to field at offset +0x00b0
    }

}

impl CcClass for ccGobjGimBlueLight {
    fn class_name(&self) -> &'static str { Self::CLASS_NAME }
    fn vtable_rva(&self) -> u32 { Self::VTABLE_RVA }
    fn virtual_method_count(&self) -> usize { Self::METHOD_COUNT }
    fn base_classes(&self) -> &'static [&'static str] { Self::BASE_CLASSES }
}

/// Decompiled CC2 class: `ccGobjGimBreakObjD01` (VTable RVA: 0x008bef60, 14 virtual methods, 39 recovered fields)
/// Inherits: ccGobjGimBase, ccGameObj
#[derive(Debug, Clone, Default)]
pub struct ccGobjGimBreakObjD01 {
    pub vtable_rva: u32,
    pub field_0x0000: u32,
    pub field_0x0004: u32,
    pub field_0x0008: u16,
    pub field_0x000a: u16,
    pub field_0x0010: u64,
    pub field_0x0018: u64,
    pub field_0x001c: u8,
    pub field_0x0020: u32,
    pub field_0x0024: u16,
    pub field_0x002c: u16,
    pub field_0x0040: u32,
    pub field_0x0058: u64,
    pub field_0x0090: u32,
    pub field_0x0094: u32,
    pub field_0x0098: u32,
    pub field_0x009c: u32,
    pub field_0x00a0: u32,
    pub field_0x00a4: u32,
    pub field_0x00a8: u32,
    pub field_0x00ac: u32,
    pub field_0x00b0: u64,
    pub field_0x00b8: u64,
    pub field_0x00c4: u32,
    pub field_0x0120: u32,
    pub field_0x0124: u32,
    pub field_0x0128: u32,
    pub field_0x012c: u32,
    pub field_0x0130: u32,
    pub field_0x0134: u32,
    pub field_0x0138: u32,
    pub field_0x013c: u32,
    pub field_0x0140: u32,
    pub field_0x0144: u32,
    pub field_0x0148: u32,
    pub field_0x014c: u32,
    pub field_0x0150: u32,
    pub field_0x0154: u32,
    pub field_0x0158: u32,
    pub field_0x015c: u32,
}

impl ccGobjGimBreakObjD01 {
    pub const CLASS_NAME: &'static str = "ccGobjGimBreakObjD01";
    pub const VTABLE_RVA: u32 = 0x008bef60;
    pub const METHOD_COUNT: usize = 14;
    pub const BASE_CLASSES: &'static [&'static str] = &["ccGobjGimBase", "ccGameObj"];
    pub const VIRTUAL_METHODS: &'static [u64] = &[0x00000001804ca230, 0x00000001804ca270, 0x000000018045e4c0, 0x00000001800d3090, 0x000000018019e550, 0x00000001804ce9f0, 0x00000001800d3090, 0x00000001800d3090, 0x00000001800d3090, 0x00000001804ca460, 0x00000001804cdda0, 0x000000018045e2d0, 0x000000018045eec0, 0x000000018045efd0];

    pub fn new() -> Self {
        Self {
            vtable_rva: Self::VTABLE_RVA,
            field_0x0000: Default::default(),
            field_0x0004: Default::default(),
            field_0x0008: Default::default(),
            field_0x000a: Default::default(),
            field_0x0010: Default::default(),
            field_0x0018: Default::default(),
            field_0x001c: Default::default(),
            field_0x0020: Default::default(),
            field_0x0024: Default::default(),
            field_0x002c: Default::default(),
            field_0x0040: Default::default(),
            field_0x0058: Default::default(),
            field_0x0090: Default::default(),
            field_0x0094: Default::default(),
            field_0x0098: Default::default(),
            field_0x009c: Default::default(),
            field_0x00a0: Default::default(),
            field_0x00a4: Default::default(),
            field_0x00a8: Default::default(),
            field_0x00ac: Default::default(),
            field_0x00b0: Default::default(),
            field_0x00b8: Default::default(),
            field_0x00c4: Default::default(),
            field_0x0120: Default::default(),
            field_0x0124: Default::default(),
            field_0x0128: Default::default(),
            field_0x012c: Default::default(),
            field_0x0130: Default::default(),
            field_0x0134: Default::default(),
            field_0x0138: Default::default(),
            field_0x013c: Default::default(),
            field_0x0140: Default::default(),
            field_0x0144: Default::default(),
            field_0x0148: Default::default(),
            field_0x014c: Default::default(),
            field_0x0150: Default::default(),
            field_0x0154: Default::default(),
            field_0x0158: Default::default(),
            field_0x015c: Default::default(),
        }
    }

    /// Decompiled virtual method #0 @ RVA 0x004ca230 (16 instructions, 3 basic blocks)
    pub fn method_00(&mut self) {
    }

    /// Decompiled virtual method #1 @ RVA 0x004ca270 (13 instructions, 3 basic blocks)
    pub fn method_01(&mut self) {
    }

    /// Decompiled virtual method #2 @ RVA 0x0045e4c0 (42 instructions, 4 basic blocks)
    pub fn method_02(&mut self) {
        // Writes to field at offset +0x0090
        // Writes to field at offset +0x0094
        // Writes to field at offset +0x0098
        // Writes to field at offset +0x009c
        // Writes to field at offset +0x00a0
        // Writes to field at offset +0x00a4
        // Writes to field at offset +0x00a8
        // Writes to field at offset +0x00ac
        // Writes to field at offset +0x0140
        // Writes to field at offset +0x0144
        // Writes to field at offset +0x0148
        // Writes to field at offset +0x014c
        // Writes to field at offset +0x0150
        // Writes to field at offset +0x0154
        // Writes to field at offset +0x0158
        // Writes to field at offset +0x015c
    }

    /// Decompiled virtual method #3 @ RVA 0x000d3090 (1 instructions, 1 basic blocks)
    pub fn method_03(&mut self) {
    }

    /// Decompiled virtual method #4 @ RVA 0x0019e550 (2 instructions, 1 basic blocks)
    pub fn method_04(&mut self) {
        // Writes to field at offset +0x00c4
    }

    /// Decompiled virtual method #5 @ RVA 0x004ce9f0 (136 instructions, 37 basic blocks)
    pub fn method_05(&mut self) {
        // Writes to field at offset +0x0000
        // Writes to field at offset +0x0000
        // Writes to field at offset +0x0004
        // Writes to field at offset +0x0008
        // Writes to field at offset +0x000a
        // Writes to field at offset +0x0040
    }

    /// Decompiled virtual method #6 @ RVA 0x000d3090 (1 instructions, 1 basic blocks)
    pub fn method_06(&mut self) {
    }

    /// Decompiled virtual method #7 @ RVA 0x000d3090 (1 instructions, 1 basic blocks)
    pub fn method_07(&mut self) {
    }

    /// Decompiled virtual method #8 @ RVA 0x000d3090 (1 instructions, 1 basic blocks)
    pub fn method_08(&mut self) {
    }

    /// Decompiled virtual method #9 @ RVA 0x004ca460 (25 instructions, 6 basic blocks)
    pub fn method_09(&mut self) {
    }

    /// Decompiled virtual method #10 @ RVA 0x004cdda0 (33 instructions, 3 basic blocks)
    pub fn method_10(&mut self) {
        // Writes to field at offset +0x0090
        // Writes to field at offset +0x0094
        // Writes to field at offset +0x0098
        // Writes to field at offset +0x009c
        // Writes to field at offset +0x00a0
        // Writes to field at offset +0x00a4
        // Writes to field at offset +0x00a8
        // Writes to field at offset +0x00ac
    }

    /// Decompiled virtual method #11 @ RVA 0x0045e2d0 (33 instructions, 3 basic blocks)
    pub fn method_11(&mut self) {
        // Writes to field at offset +0x0090
        // Writes to field at offset +0x00a0
        // Writes to field at offset +0x00a4
        // Writes to field at offset +0x00a8
        // Writes to field at offset +0x00ac
    }

    /// Decompiled virtual method #12 @ RVA 0x0045eec0 (67 instructions, 22 basic blocks)
    pub fn method_12(&mut self) {
        // Writes to field at offset +0x001c
        // Writes to field at offset +0x0040
        // Writes to field at offset +0x0090
        // Writes to field at offset +0x00b0
    }

    /// Decompiled virtual method #13 @ RVA 0x0045efd0 (27 instructions, 5 basic blocks)
    pub fn method_13(&mut self) {
        // Writes to field at offset +0x0090
        // Writes to field at offset +0x00b0
    }

}

impl CcClass for ccGobjGimBreakObjD01 {
    fn class_name(&self) -> &'static str { Self::CLASS_NAME }
    fn vtable_rva(&self) -> u32 { Self::VTABLE_RVA }
    fn virtual_method_count(&self) -> usize { Self::METHOD_COUNT }
    fn base_classes(&self) -> &'static [&'static str] { Self::BASE_CLASSES }
}

/// Decompiled CC2 class: `ccGobjGimChaosGate` (VTable RVA: 0x008bf410, 14 virtual methods, 36 recovered fields)
/// Inherits: ccGobjGimBase, ccGameObj
#[derive(Debug, Clone, Default)]
pub struct ccGobjGimChaosGate {
    pub vtable_rva: u32,
    pub field_0x0000: u64,
    pub field_0x0010: u64,
    pub field_0x0018: u64,
    pub field_0x001c: u8,
    pub field_0x0020: u32,
    pub field_0x0024: u16,
    pub field_0x002c: u16,
    pub field_0x0040: u32,
    pub field_0x0090: u32,
    pub field_0x0094: u32,
    pub field_0x0098: u32,
    pub field_0x009c: u32,
    pub field_0x00a0: u32,
    pub field_0x00a4: u32,
    pub field_0x00a8: u32,
    pub field_0x00ac: u32,
    pub field_0x00b0: u64,
    pub field_0x00b8: u64,
    pub field_0x00c4: u32,
    pub field_0x0120: u32,
    pub field_0x0124: u32,
    pub field_0x0128: u32,
    pub field_0x012c: u32,
    pub field_0x0130: u32,
    pub field_0x0134: u32,
    pub field_0x0138: u32,
    pub field_0x013c: u32,
    pub field_0x0140: u32,
    pub field_0x0144: u32,
    pub field_0x0148: u32,
    pub field_0x014c: u32,
    pub field_0x0150: u32,
    pub field_0x0154: u32,
    pub field_0x0158: u32,
    pub field_0x015c: u32,
    pub field_0x01c0: u32,
}

impl ccGobjGimChaosGate {
    pub const CLASS_NAME: &'static str = "ccGobjGimChaosGate";
    pub const VTABLE_RVA: u32 = 0x008bf410;
    pub const METHOD_COUNT: usize = 14;
    pub const BASE_CLASSES: &'static [&'static str] = &["ccGobjGimBase", "ccGameObj"];
    pub const VIRTUAL_METHODS: &'static [u64] = &[0x00000001804c8970, 0x00000001804c89d0, 0x000000018045e4c0, 0x00000001800d3090, 0x000000018019e550, 0x00000001804c8c50, 0x00000001800d3090, 0x00000001800d3090, 0x00000001800d3090, 0x00000001804c8c10, 0x000000018045e220, 0x000000018045e2d0, 0x000000018045eec0, 0x000000018045efd0];

    pub fn new() -> Self {
        Self {
            vtable_rva: Self::VTABLE_RVA,
            field_0x0000: Default::default(),
            field_0x0010: Default::default(),
            field_0x0018: Default::default(),
            field_0x001c: Default::default(),
            field_0x0020: Default::default(),
            field_0x0024: Default::default(),
            field_0x002c: Default::default(),
            field_0x0040: Default::default(),
            field_0x0090: Default::default(),
            field_0x0094: Default::default(),
            field_0x0098: Default::default(),
            field_0x009c: Default::default(),
            field_0x00a0: Default::default(),
            field_0x00a4: Default::default(),
            field_0x00a8: Default::default(),
            field_0x00ac: Default::default(),
            field_0x00b0: Default::default(),
            field_0x00b8: Default::default(),
            field_0x00c4: Default::default(),
            field_0x0120: Default::default(),
            field_0x0124: Default::default(),
            field_0x0128: Default::default(),
            field_0x012c: Default::default(),
            field_0x0130: Default::default(),
            field_0x0134: Default::default(),
            field_0x0138: Default::default(),
            field_0x013c: Default::default(),
            field_0x0140: Default::default(),
            field_0x0144: Default::default(),
            field_0x0148: Default::default(),
            field_0x014c: Default::default(),
            field_0x0150: Default::default(),
            field_0x0154: Default::default(),
            field_0x0158: Default::default(),
            field_0x015c: Default::default(),
            field_0x01c0: Default::default(),
        }
    }

    /// Decompiled virtual method #0 @ RVA 0x004c8970 (26 instructions, 6 basic blocks)
    pub fn method_00(&mut self) {
        // Writes to field at offset +0x0000
    }

    /// Decompiled virtual method #1 @ RVA 0x004c89d0 (13 instructions, 3 basic blocks)
    pub fn method_01(&mut self) {
    }

    /// Decompiled virtual method #2 @ RVA 0x0045e4c0 (42 instructions, 4 basic blocks)
    pub fn method_02(&mut self) {
        // Writes to field at offset +0x0090
        // Writes to field at offset +0x0094
        // Writes to field at offset +0x0098
        // Writes to field at offset +0x009c
        // Writes to field at offset +0x00a0
        // Writes to field at offset +0x00a4
        // Writes to field at offset +0x00a8
        // Writes to field at offset +0x00ac
        // Writes to field at offset +0x0140
        // Writes to field at offset +0x0144
        // Writes to field at offset +0x0148
        // Writes to field at offset +0x014c
        // Writes to field at offset +0x0150
        // Writes to field at offset +0x0154
        // Writes to field at offset +0x0158
        // Writes to field at offset +0x015c
    }

    /// Decompiled virtual method #3 @ RVA 0x000d3090 (1 instructions, 1 basic blocks)
    pub fn method_03(&mut self) {
    }

    /// Decompiled virtual method #4 @ RVA 0x0019e550 (2 instructions, 1 basic blocks)
    pub fn method_04(&mut self) {
        // Writes to field at offset +0x00c4
    }

    /// Decompiled virtual method #5 @ RVA 0x004c8c50 (147 instructions, 32 basic blocks)
    pub fn method_05(&mut self) {
        // Writes to field at offset +0x0040
    }

    /// Decompiled virtual method #6 @ RVA 0x000d3090 (1 instructions, 1 basic blocks)
    pub fn method_06(&mut self) {
    }

    /// Decompiled virtual method #7 @ RVA 0x000d3090 (1 instructions, 1 basic blocks)
    pub fn method_07(&mut self) {
    }

    /// Decompiled virtual method #8 @ RVA 0x000d3090 (1 instructions, 1 basic blocks)
    pub fn method_08(&mut self) {
    }

    /// Decompiled virtual method #9 @ RVA 0x004c8c10 (15 instructions, 4 basic blocks)
    pub fn method_09(&mut self) {
    }

    /// Decompiled virtual method #10 @ RVA 0x0045e220 (31 instructions, 3 basic blocks)
    pub fn method_10(&mut self) {
        // Writes to field at offset +0x0090
        // Writes to field at offset +0x00a0
        // Writes to field at offset +0x00a4
        // Writes to field at offset +0x00a8
        // Writes to field at offset +0x00ac
    }

    /// Decompiled virtual method #11 @ RVA 0x0045e2d0 (33 instructions, 3 basic blocks)
    pub fn method_11(&mut self) {
        // Writes to field at offset +0x0090
        // Writes to field at offset +0x00a0
        // Writes to field at offset +0x00a4
        // Writes to field at offset +0x00a8
        // Writes to field at offset +0x00ac
    }

    /// Decompiled virtual method #12 @ RVA 0x0045eec0 (67 instructions, 22 basic blocks)
    pub fn method_12(&mut self) {
        // Writes to field at offset +0x001c
        // Writes to field at offset +0x0040
        // Writes to field at offset +0x0090
        // Writes to field at offset +0x00b0
    }

    /// Decompiled virtual method #13 @ RVA 0x0045efd0 (27 instructions, 5 basic blocks)
    pub fn method_13(&mut self) {
        // Writes to field at offset +0x0090
        // Writes to field at offset +0x00b0
    }

}

impl CcClass for ccGobjGimChaosGate {
    fn class_name(&self) -> &'static str { Self::CLASS_NAME }
    fn vtable_rva(&self) -> u32 { Self::VTABLE_RVA }
    fn virtual_method_count(&self) -> usize { Self::METHOD_COUNT }
    fn base_classes(&self) -> &'static [&'static str] { Self::BASE_CLASSES }
}

/// Decompiled CC2 class: `ccGobjGimChimAss` (VTable RVA: 0x00877ff0, 14 virtual methods, 44 recovered fields)
/// Inherits: ccGameObj
#[derive(Debug, Clone, Default)]
pub struct ccGobjGimChimAss {
    pub vtable_rva: u32,
    pub field_0x0008: u32,
    pub field_0x0009: u8,
    pub field_0x0010: u64,
    pub field_0x0018: u64,
    pub field_0x001c: u8,
    pub field_0x0020: u32,
    pub field_0x0024: u16,
    pub field_0x002c: u16,
    pub field_0x0040: u32,
    pub field_0x0050: u64,
    pub field_0x0058: u64,
    pub field_0x0070: u32,
    pub field_0x0090: u32,
    pub field_0x0094: u32,
    pub field_0x0098: u32,
    pub field_0x009c: u32,
    pub field_0x00a0: u32,
    pub field_0x00a4: u32,
    pub field_0x00a8: u32,
    pub field_0x00ac: u32,
    pub field_0x00b0: u64,
    pub field_0x00b8: u64,
    pub field_0x00c4: u32,
    pub field_0x00e0: u32,
    pub field_0x0120: u32,
    pub field_0x0124: u32,
    pub field_0x0128: u32,
    pub field_0x012c: u32,
    pub field_0x0130: u32,
    pub field_0x0134: u32,
    pub field_0x0138: u32,
    pub field_0x013c: u32,
    pub field_0x0140: u32,
    pub field_0x0144: u32,
    pub field_0x0148: u32,
    pub field_0x014c: u32,
    pub field_0x0150: u32,
    pub field_0x0154: u32,
    pub field_0x0158: u32,
    pub field_0x015c: u32,
    pub field_0x016c: f32,
    pub field_0x0170: u32,
    pub field_0x01b8: u32,
    pub field_0x01c0: u32,
}

impl ccGobjGimChimAss {
    pub const CLASS_NAME: &'static str = "ccGobjGimChimAss";
    pub const VTABLE_RVA: u32 = 0x00877ff0;
    pub const METHOD_COUNT: usize = 14;
    pub const BASE_CLASSES: &'static [&'static str] = &["ccGameObj"];
    pub const VIRTUAL_METHODS: &'static [u64] = &[0x000000018019e8a0, 0x000000018019eca0, 0x000000018045e4c0, 0x00000001800d3090, 0x000000018019e550, 0x000000018019f9e0, 0x000000018019f720, 0x00000001800d3090, 0x00000001800d3090, 0x00000001800d3090, 0x000000018019f2e0, 0x000000018045e2d0, 0x000000018045eec0, 0x000000018045efd0];

    pub fn new() -> Self {
        Self {
            vtable_rva: Self::VTABLE_RVA,
            field_0x0008: Default::default(),
            field_0x0009: Default::default(),
            field_0x0010: Default::default(),
            field_0x0018: Default::default(),
            field_0x001c: Default::default(),
            field_0x0020: Default::default(),
            field_0x0024: Default::default(),
            field_0x002c: Default::default(),
            field_0x0040: Default::default(),
            field_0x0050: Default::default(),
            field_0x0058: Default::default(),
            field_0x0070: Default::default(),
            field_0x0090: Default::default(),
            field_0x0094: Default::default(),
            field_0x0098: Default::default(),
            field_0x009c: Default::default(),
            field_0x00a0: Default::default(),
            field_0x00a4: Default::default(),
            field_0x00a8: Default::default(),
            field_0x00ac: Default::default(),
            field_0x00b0: Default::default(),
            field_0x00b8: Default::default(),
            field_0x00c4: Default::default(),
            field_0x00e0: Default::default(),
            field_0x0120: Default::default(),
            field_0x0124: Default::default(),
            field_0x0128: Default::default(),
            field_0x012c: Default::default(),
            field_0x0130: Default::default(),
            field_0x0134: Default::default(),
            field_0x0138: Default::default(),
            field_0x013c: Default::default(),
            field_0x0140: Default::default(),
            field_0x0144: Default::default(),
            field_0x0148: Default::default(),
            field_0x014c: Default::default(),
            field_0x0150: Default::default(),
            field_0x0154: Default::default(),
            field_0x0158: Default::default(),
            field_0x015c: Default::default(),
            field_0x016c: Default::default(),
            field_0x0170: Default::default(),
            field_0x01b8: Default::default(),
            field_0x01c0: Default::default(),
        }
    }

    /// Decompiled virtual method #0 @ RVA 0x0019e8a0 (16 instructions, 3 basic blocks)
    pub fn method_00(&mut self) {
    }

    /// Decompiled virtual method #1 @ RVA 0x0019eca0 (110 instructions, 21 basic blocks)
    pub fn method_01(&mut self) {
        // Writes to field at offset +0x0070
    }

    /// Decompiled virtual method #2 @ RVA 0x0045e4c0 (42 instructions, 4 basic blocks)
    pub fn method_02(&mut self) {
        // Writes to field at offset +0x0090
        // Writes to field at offset +0x0094
        // Writes to field at offset +0x0098
        // Writes to field at offset +0x009c
        // Writes to field at offset +0x00a0
        // Writes to field at offset +0x00a4
        // Writes to field at offset +0x00a8
        // Writes to field at offset +0x00ac
        // Writes to field at offset +0x0140
        // Writes to field at offset +0x0144
        // Writes to field at offset +0x0148
        // Writes to field at offset +0x014c
        // Writes to field at offset +0x0150
        // Writes to field at offset +0x0154
        // Writes to field at offset +0x0158
        // Writes to field at offset +0x015c
    }

    /// Decompiled virtual method #3 @ RVA 0x000d3090 (1 instructions, 1 basic blocks)
    pub fn method_03(&mut self) {
    }

    /// Decompiled virtual method #4 @ RVA 0x0019e550 (2 instructions, 1 basic blocks)
    pub fn method_04(&mut self) {
        // Writes to field at offset +0x00c4
    }

    /// Decompiled virtual method #5 @ RVA 0x0019f9e0 (114 instructions, 22 basic blocks)
    pub fn method_05(&mut self) {
        // Writes to field at offset +0x0009
        // Writes to field at offset +0x0040
        // Writes to field at offset +0x016c
        // Writes to field at offset +0x01b8
    }

    /// Decompiled virtual method #6 @ RVA 0x0019f720 (126 instructions, 38 basic blocks)
    pub fn method_06(&mut self) {
        // Writes to field at offset +0x0070
        // Writes to field at offset +0x01c0
    }

    /// Decompiled virtual method #7 @ RVA 0x000d3090 (1 instructions, 1 basic blocks)
    pub fn method_07(&mut self) {
    }

    /// Decompiled virtual method #8 @ RVA 0x000d3090 (1 instructions, 1 basic blocks)
    pub fn method_08(&mut self) {
    }

    /// Decompiled virtual method #9 @ RVA 0x000d3090 (1 instructions, 1 basic blocks)
    pub fn method_09(&mut self) {
    }

    /// Decompiled virtual method #10 @ RVA 0x0019f2e0 (84 instructions, 4 basic blocks)
    pub fn method_10(&mut self) {
        // Writes to field at offset +0x0090
        // Writes to field at offset +0x0094
        // Writes to field at offset +0x0098
        // Writes to field at offset +0x009c
        // Writes to field at offset +0x00a0
        // Writes to field at offset +0x00a4
        // Writes to field at offset +0x00a8
        // Writes to field at offset +0x00ac
    }

    /// Decompiled virtual method #11 @ RVA 0x0045e2d0 (33 instructions, 3 basic blocks)
    pub fn method_11(&mut self) {
        // Writes to field at offset +0x0090
        // Writes to field at offset +0x00a0
        // Writes to field at offset +0x00a4
        // Writes to field at offset +0x00a8
        // Writes to field at offset +0x00ac
    }

    /// Decompiled virtual method #12 @ RVA 0x0045eec0 (67 instructions, 22 basic blocks)
    pub fn method_12(&mut self) {
        // Writes to field at offset +0x001c
        // Writes to field at offset +0x0040
        // Writes to field at offset +0x0090
        // Writes to field at offset +0x00b0
    }

    /// Decompiled virtual method #13 @ RVA 0x0045efd0 (27 instructions, 5 basic blocks)
    pub fn method_13(&mut self) {
        // Writes to field at offset +0x0090
        // Writes to field at offset +0x00b0
    }

}

impl CcClass for ccGobjGimChimAss {
    fn class_name(&self) -> &'static str { Self::CLASS_NAME }
    fn vtable_rva(&self) -> u32 { Self::VTABLE_RVA }
    fn virtual_method_count(&self) -> usize { Self::METHOD_COUNT }
    fn base_classes(&self) -> &'static [&'static str] { Self::BASE_CLASSES }
}

/// Decompiled CC2 class: `ccGobjGimChimBall` (VTable RVA: 0x008be9c0, 14 virtual methods, 46 recovered fields)
/// Inherits: ccGobjGimBase, ccGameObj
#[derive(Debug, Clone, Default)]
pub struct ccGobjGimChimBall {
    pub vtable_rva: u32,
    pub field_0x0000: u64,
    pub field_0x0002: u32,
    pub field_0x0004: u32,
    pub field_0x0008: u16,
    pub field_0x0009: u8,
    pub field_0x000a: u16,
    pub field_0x0010: u64,
    pub field_0x0018: u64,
    pub field_0x001c: u8,
    pub field_0x0020: u32,
    pub field_0x0024: u16,
    pub field_0x002c: u16,
    pub field_0x0040: u32,
    pub field_0x0048: u64,
    pub field_0x0058: u64,
    pub field_0x0090: u32,
    pub field_0x0094: u32,
    pub field_0x0098: u32,
    pub field_0x009c: u32,
    pub field_0x00a0: u32,
    pub field_0x00a4: u32,
    pub field_0x00a8: u32,
    pub field_0x00ac: u32,
    pub field_0x00b0: u64,
    pub field_0x00b8: u64,
    pub field_0x00c4: u32,
    pub field_0x00d8: u32,
    pub field_0x0120: u32,
    pub field_0x0124: u32,
    pub field_0x0128: u32,
    pub field_0x012c: u32,
    pub field_0x0130: u32,
    pub field_0x0134: u32,
    pub field_0x0138: u32,
    pub field_0x013c: u32,
    pub field_0x0140: u32,
    pub field_0x0144: u32,
    pub field_0x0148: u32,
    pub field_0x014c: u32,
    pub field_0x0150: u32,
    pub field_0x0154: u32,
    pub field_0x0158: u32,
    pub field_0x015c: u32,
    pub field_0x01d0: u64,
    pub field_0x01d8: u16,
    pub field_0x01da: u16,
}

impl ccGobjGimChimBall {
    pub const CLASS_NAME: &'static str = "ccGobjGimChimBall";
    pub const VTABLE_RVA: u32 = 0x008be9c0;
    pub const METHOD_COUNT: usize = 14;
    pub const BASE_CLASSES: &'static [&'static str] = &["ccGobjGimBase", "ccGameObj"];
    pub const VIRTUAL_METHODS: &'static [u64] = &[0x00000001804d86e0, 0x00000001804d8920, 0x000000018045e4c0, 0x00000001800d3090, 0x000000018019e550, 0x00000001804d8dc0, 0x00000001800d3090, 0x00000001800d3090, 0x00000001800d3090, 0x00000001804d8bb0, 0x00000001804d8b90, 0x000000018045e2d0, 0x000000018045eec0, 0x000000018045efd0];

    pub fn new() -> Self {
        Self {
            vtable_rva: Self::VTABLE_RVA,
            field_0x0000: Default::default(),
            field_0x0002: Default::default(),
            field_0x0004: Default::default(),
            field_0x0008: Default::default(),
            field_0x0009: Default::default(),
            field_0x000a: Default::default(),
            field_0x0010: Default::default(),
            field_0x0018: Default::default(),
            field_0x001c: Default::default(),
            field_0x0020: Default::default(),
            field_0x0024: Default::default(),
            field_0x002c: Default::default(),
            field_0x0040: Default::default(),
            field_0x0048: Default::default(),
            field_0x0058: Default::default(),
            field_0x0090: Default::default(),
            field_0x0094: Default::default(),
            field_0x0098: Default::default(),
            field_0x009c: Default::default(),
            field_0x00a0: Default::default(),
            field_0x00a4: Default::default(),
            field_0x00a8: Default::default(),
            field_0x00ac: Default::default(),
            field_0x00b0: Default::default(),
            field_0x00b8: Default::default(),
            field_0x00c4: Default::default(),
            field_0x00d8: Default::default(),
            field_0x0120: Default::default(),
            field_0x0124: Default::default(),
            field_0x0128: Default::default(),
            field_0x012c: Default::default(),
            field_0x0130: Default::default(),
            field_0x0134: Default::default(),
            field_0x0138: Default::default(),
            field_0x013c: Default::default(),
            field_0x0140: Default::default(),
            field_0x0144: Default::default(),
            field_0x0148: Default::default(),
            field_0x014c: Default::default(),
            field_0x0150: Default::default(),
            field_0x0154: Default::default(),
            field_0x0158: Default::default(),
            field_0x015c: Default::default(),
            field_0x01d0: Default::default(),
            field_0x01d8: Default::default(),
            field_0x01da: Default::default(),
        }
    }

    /// Decompiled virtual method #0 @ RVA 0x004d86e0 (20 instructions, 3 basic blocks)
    pub fn method_00(&mut self) {
        // Writes to field at offset +0x0000
    }

    /// Decompiled virtual method #1 @ RVA 0x004d8920 (128 instructions, 37 basic blocks)
    pub fn method_01(&mut self) {
        // Writes to field at offset +0x0000
        // Writes to field at offset +0x0004
        // Writes to field at offset +0x0008
        // Writes to field at offset +0x0009
        // Writes to field at offset +0x000a
        // Writes to field at offset +0x001c
        // Writes to field at offset +0x0040
    }

    /// Decompiled virtual method #2 @ RVA 0x0045e4c0 (42 instructions, 4 basic blocks)
    pub fn method_02(&mut self) {
        // Writes to field at offset +0x0090
        // Writes to field at offset +0x0094
        // Writes to field at offset +0x0098
        // Writes to field at offset +0x009c
        // Writes to field at offset +0x00a0
        // Writes to field at offset +0x00a4
        // Writes to field at offset +0x00a8
        // Writes to field at offset +0x00ac
        // Writes to field at offset +0x0140
        // Writes to field at offset +0x0144
        // Writes to field at offset +0x0148
        // Writes to field at offset +0x014c
        // Writes to field at offset +0x0150
        // Writes to field at offset +0x0154
        // Writes to field at offset +0x0158
        // Writes to field at offset +0x015c
    }

    /// Decompiled virtual method #3 @ RVA 0x000d3090 (1 instructions, 1 basic blocks)
    pub fn method_03(&mut self) {
    }

    /// Decompiled virtual method #4 @ RVA 0x0019e550 (2 instructions, 1 basic blocks)
    pub fn method_04(&mut self) {
        // Writes to field at offset +0x00c4
    }

    /// Decompiled virtual method #5 @ RVA 0x004d8dc0 (91 instructions, 24 basic blocks)
    pub fn method_05(&mut self) {
        // Writes to field at offset +0x0024
    }

    /// Decompiled virtual method #6 @ RVA 0x000d3090 (1 instructions, 1 basic blocks)
    pub fn method_06(&mut self) {
    }

    /// Decompiled virtual method #7 @ RVA 0x000d3090 (1 instructions, 1 basic blocks)
    pub fn method_07(&mut self) {
    }

    /// Decompiled virtual method #8 @ RVA 0x000d3090 (1 instructions, 1 basic blocks)
    pub fn method_08(&mut self) {
    }

    /// Decompiled virtual method #9 @ RVA 0x004d8bb0 (102 instructions, 31 basic blocks)
    pub fn method_09(&mut self) {
        // Writes to field at offset +0x0010
        // Writes to field at offset +0x009c
        // Writes to field at offset +0x00d8
        // Writes to field at offset +0x01d0
        // Writes to field at offset +0x01d8
        // Writes to field at offset +0x01da
    }

    /// Decompiled virtual method #10 @ RVA 0x004d8b90 (4 instructions, 2 basic blocks)
    pub fn method_10(&mut self) {
    }

    /// Decompiled virtual method #11 @ RVA 0x0045e2d0 (33 instructions, 3 basic blocks)
    pub fn method_11(&mut self) {
        // Writes to field at offset +0x0090
        // Writes to field at offset +0x00a0
        // Writes to field at offset +0x00a4
        // Writes to field at offset +0x00a8
        // Writes to field at offset +0x00ac
    }

    /// Decompiled virtual method #12 @ RVA 0x0045eec0 (67 instructions, 22 basic blocks)
    pub fn method_12(&mut self) {
        // Writes to field at offset +0x001c
        // Writes to field at offset +0x0040
        // Writes to field at offset +0x0090
        // Writes to field at offset +0x00b0
    }

    /// Decompiled virtual method #13 @ RVA 0x0045efd0 (27 instructions, 5 basic blocks)
    pub fn method_13(&mut self) {
        // Writes to field at offset +0x0090
        // Writes to field at offset +0x00b0
    }

}

impl CcClass for ccGobjGimChimBall {
    fn class_name(&self) -> &'static str { Self::CLASS_NAME }
    fn vtable_rva(&self) -> u32 { Self::VTABLE_RVA }
    fn virtual_method_count(&self) -> usize { Self::METHOD_COUNT }
    fn base_classes(&self) -> &'static [&'static str] { Self::BASE_CLASSES }
}

/// Decompiled CC2 class: `ccGobjGimChimBtn` (VTable RVA: 0x008beb28, 14 virtual methods, 43 recovered fields)
/// Inherits: ccGobjGimBaseChangeCCS, ccGobjGimBase, ccGameObj
#[derive(Debug, Clone, Default)]
pub struct ccGobjGimChimBtn {
    pub vtable_rva: u32,
    pub field_0x0000: u64,
    pub field_0x0004: u32,
    pub field_0x0008: u16,
    pub field_0x0009: u8,
    pub field_0x000a: u16,
    pub field_0x0010: u64,
    pub field_0x0018: u64,
    pub field_0x001c: u8,
    pub field_0x0020: u32,
    pub field_0x0024: u16,
    pub field_0x002c: u16,
    pub field_0x0040: u32,
    pub field_0x0048: u64,
    pub field_0x0058: u64,
    pub field_0x0090: u32,
    pub field_0x0094: u32,
    pub field_0x0098: u32,
    pub field_0x009c: u32,
    pub field_0x00a0: u32,
    pub field_0x00a4: u32,
    pub field_0x00a8: u32,
    pub field_0x00ac: u32,
    pub field_0x00b0: u64,
    pub field_0x00b8: u64,
    pub field_0x00c4: u32,
    pub field_0x0120: u32,
    pub field_0x0124: u32,
    pub field_0x0128: u32,
    pub field_0x012c: u32,
    pub field_0x0130: u32,
    pub field_0x0134: u32,
    pub field_0x0138: u32,
    pub field_0x013c: u32,
    pub field_0x0140: u32,
    pub field_0x0144: u32,
    pub field_0x0148: u32,
    pub field_0x014c: u32,
    pub field_0x0150: u32,
    pub field_0x0154: u32,
    pub field_0x0158: u32,
    pub field_0x015c: u32,
    pub field_0x01c0: u32,
    pub field_0x01c8: u32,
}

impl ccGobjGimChimBtn {
    pub const CLASS_NAME: &'static str = "ccGobjGimChimBtn";
    pub const VTABLE_RVA: u32 = 0x008beb28;
    pub const METHOD_COUNT: usize = 14;
    pub const BASE_CLASSES: &'static [&'static str] = &["ccGobjGimBaseChangeCCS", "ccGobjGimBase", "ccGameObj"];
    pub const VIRTUAL_METHODS: &'static [u64] = &[0x00000001804d6a00, 0x00000001804d6a60, 0x000000018045e4c0, 0x00000001800d3090, 0x000000018019e550, 0x00000001804d6e00, 0x00000001800d3090, 0x00000001800d3090, 0x00000001800d3090, 0x00000001804d6d60, 0x000000018045e220, 0x000000018045e2d0, 0x000000018045eec0, 0x000000018045efd0];

    pub fn new() -> Self {
        Self {
            vtable_rva: Self::VTABLE_RVA,
            field_0x0000: Default::default(),
            field_0x0004: Default::default(),
            field_0x0008: Default::default(),
            field_0x0009: Default::default(),
            field_0x000a: Default::default(),
            field_0x0010: Default::default(),
            field_0x0018: Default::default(),
            field_0x001c: Default::default(),
            field_0x0020: Default::default(),
            field_0x0024: Default::default(),
            field_0x002c: Default::default(),
            field_0x0040: Default::default(),
            field_0x0048: Default::default(),
            field_0x0058: Default::default(),
            field_0x0090: Default::default(),
            field_0x0094: Default::default(),
            field_0x0098: Default::default(),
            field_0x009c: Default::default(),
            field_0x00a0: Default::default(),
            field_0x00a4: Default::default(),
            field_0x00a8: Default::default(),
            field_0x00ac: Default::default(),
            field_0x00b0: Default::default(),
            field_0x00b8: Default::default(),
            field_0x00c4: Default::default(),
            field_0x0120: Default::default(),
            field_0x0124: Default::default(),
            field_0x0128: Default::default(),
            field_0x012c: Default::default(),
            field_0x0130: Default::default(),
            field_0x0134: Default::default(),
            field_0x0138: Default::default(),
            field_0x013c: Default::default(),
            field_0x0140: Default::default(),
            field_0x0144: Default::default(),
            field_0x0148: Default::default(),
            field_0x014c: Default::default(),
            field_0x0150: Default::default(),
            field_0x0154: Default::default(),
            field_0x0158: Default::default(),
            field_0x015c: Default::default(),
            field_0x01c0: Default::default(),
            field_0x01c8: Default::default(),
        }
    }

    /// Decompiled virtual method #0 @ RVA 0x004d6a00 (22 instructions, 5 basic blocks)
    pub fn method_00(&mut self) {
        // Writes to field at offset +0x0000
    }

    /// Decompiled virtual method #1 @ RVA 0x004d6a60 (117 instructions, 19 basic blocks)
    pub fn method_01(&mut self) {
        // Writes to field at offset +0x0000
        // Writes to field at offset +0x0004
        // Writes to field at offset +0x0008
        // Writes to field at offset +0x000a
        // Writes to field at offset +0x0040
    }

    /// Decompiled virtual method #2 @ RVA 0x0045e4c0 (42 instructions, 4 basic blocks)
    pub fn method_02(&mut self) {
        // Writes to field at offset +0x0090
        // Writes to field at offset +0x0094
        // Writes to field at offset +0x0098
        // Writes to field at offset +0x009c
        // Writes to field at offset +0x00a0
        // Writes to field at offset +0x00a4
        // Writes to field at offset +0x00a8
        // Writes to field at offset +0x00ac
        // Writes to field at offset +0x0140
        // Writes to field at offset +0x0144
        // Writes to field at offset +0x0148
        // Writes to field at offset +0x014c
        // Writes to field at offset +0x0150
        // Writes to field at offset +0x0154
        // Writes to field at offset +0x0158
        // Writes to field at offset +0x015c
    }

    /// Decompiled virtual method #3 @ RVA 0x000d3090 (1 instructions, 1 basic blocks)
    pub fn method_03(&mut self) {
    }

    /// Decompiled virtual method #4 @ RVA 0x0019e550 (2 instructions, 1 basic blocks)
    pub fn method_04(&mut self) {
        // Writes to field at offset +0x00c4
    }

    /// Decompiled virtual method #5 @ RVA 0x004d6e00 (94 instructions, 11 basic blocks)
    pub fn method_05(&mut self) {
        // Writes to field at offset +0x0009
    }

    /// Decompiled virtual method #6 @ RVA 0x000d3090 (1 instructions, 1 basic blocks)
    pub fn method_06(&mut self) {
    }

    /// Decompiled virtual method #7 @ RVA 0x000d3090 (1 instructions, 1 basic blocks)
    pub fn method_07(&mut self) {
    }

    /// Decompiled virtual method #8 @ RVA 0x000d3090 (1 instructions, 1 basic blocks)
    pub fn method_08(&mut self) {
    }

    /// Decompiled virtual method #9 @ RVA 0x004d6d60 (36 instructions, 12 basic blocks)
    pub fn method_09(&mut self) {
        // Writes to field at offset +0x0009
        // Writes to field at offset +0x01c0
        // Writes to field at offset +0x01c8
    }

    /// Decompiled virtual method #10 @ RVA 0x0045e220 (31 instructions, 3 basic blocks)
    pub fn method_10(&mut self) {
        // Writes to field at offset +0x0090
        // Writes to field at offset +0x00a0
        // Writes to field at offset +0x00a4
        // Writes to field at offset +0x00a8
        // Writes to field at offset +0x00ac
    }

    /// Decompiled virtual method #11 @ RVA 0x0045e2d0 (33 instructions, 3 basic blocks)
    pub fn method_11(&mut self) {
        // Writes to field at offset +0x0090
        // Writes to field at offset +0x00a0
        // Writes to field at offset +0x00a4
        // Writes to field at offset +0x00a8
        // Writes to field at offset +0x00ac
    }

    /// Decompiled virtual method #12 @ RVA 0x0045eec0 (67 instructions, 22 basic blocks)
    pub fn method_12(&mut self) {
        // Writes to field at offset +0x001c
        // Writes to field at offset +0x0040
        // Writes to field at offset +0x0090
        // Writes to field at offset +0x00b0
    }

    /// Decompiled virtual method #13 @ RVA 0x0045efd0 (27 instructions, 5 basic blocks)
    pub fn method_13(&mut self) {
        // Writes to field at offset +0x0090
        // Writes to field at offset +0x00b0
    }

}

impl CcClass for ccGobjGimChimBtn {
    fn class_name(&self) -> &'static str { Self::CLASS_NAME }
    fn vtable_rva(&self) -> u32 { Self::VTABLE_RVA }
    fn virtual_method_count(&self) -> usize { Self::METHOD_COUNT }
    fn base_classes(&self) -> &'static [&'static str] { Self::BASE_CLASSES }
}

/// Decompiled CC2 class: `ccGobjGimChimEat` (VTable RVA: 0x008be1c8, 14 virtual methods, 44 recovered fields)
/// Inherits: ccGobjGimBase, ccGameObj
#[derive(Debug, Clone, Default)]
pub struct ccGobjGimChimEat {
    pub vtable_rva: u32,
    pub field_0x0000: u64,
    pub field_0x0008: u32,
    pub field_0x0009: u8,
    pub field_0x0010: u64,
    pub field_0x0018: u64,
    pub field_0x001c: u8,
    pub field_0x0020: u32,
    pub field_0x0024: u16,
    pub field_0x002c: u16,
    pub field_0x0040: u32,
    pub field_0x0058: u64,
    pub field_0x0070: u32,
    pub field_0x0090: u32,
    pub field_0x0094: u32,
    pub field_0x0098: u32,
    pub field_0x009c: u32,
    pub field_0x00a0: u32,
    pub field_0x00a4: u32,
    pub field_0x00a8: u32,
    pub field_0x00ac: u32,
    pub field_0x00b0: u64,
    pub field_0x00b8: u64,
    pub field_0x00c4: u32,
    pub field_0x00cc: u32,
    pub field_0x00e0: u32,
    pub field_0x0120: u32,
    pub field_0x0124: u32,
    pub field_0x0128: u32,
    pub field_0x012c: u32,
    pub field_0x0130: u32,
    pub field_0x0134: u32,
    pub field_0x0138: u32,
    pub field_0x013c: u32,
    pub field_0x0140: u32,
    pub field_0x0144: u32,
    pub field_0x0148: u32,
    pub field_0x014c: u32,
    pub field_0x0150: u32,
    pub field_0x0154: u32,
    pub field_0x0158: u32,
    pub field_0x015c: u32,
    pub field_0x016c: f32,
    pub field_0x0170: u32,
    pub field_0x01b8: u32,
}

impl ccGobjGimChimEat {
    pub const CLASS_NAME: &'static str = "ccGobjGimChimEat";
    pub const VTABLE_RVA: u32 = 0x008be1c8;
    pub const METHOD_COUNT: usize = 14;
    pub const BASE_CLASSES: &'static [&'static str] = &["ccGobjGimBase", "ccGameObj"];
    pub const VIRTUAL_METHODS: &'static [u64] = &[0x00000001804f86c0, 0x00000001804f88b0, 0x000000018045e4c0, 0x00000001800d3090, 0x000000018019e550, 0x00000001804f9050, 0x00000001804f8da0, 0x00000001800d3090, 0x00000001800d3090, 0x00000001800d3090, 0x00000001804f8cf0, 0x000000018045e2d0, 0x000000018045eec0, 0x000000018045efd0];

    pub fn new() -> Self {
        Self {
            vtable_rva: Self::VTABLE_RVA,
            field_0x0000: Default::default(),
            field_0x0008: Default::default(),
            field_0x0009: Default::default(),
            field_0x0010: Default::default(),
            field_0x0018: Default::default(),
            field_0x001c: Default::default(),
            field_0x0020: Default::default(),
            field_0x0024: Default::default(),
            field_0x002c: Default::default(),
            field_0x0040: Default::default(),
            field_0x0058: Default::default(),
            field_0x0070: Default::default(),
            field_0x0090: Default::default(),
            field_0x0094: Default::default(),
            field_0x0098: Default::default(),
            field_0x009c: Default::default(),
            field_0x00a0: Default::default(),
            field_0x00a4: Default::default(),
            field_0x00a8: Default::default(),
            field_0x00ac: Default::default(),
            field_0x00b0: Default::default(),
            field_0x00b8: Default::default(),
            field_0x00c4: Default::default(),
            field_0x00cc: Default::default(),
            field_0x00e0: Default::default(),
            field_0x0120: Default::default(),
            field_0x0124: Default::default(),
            field_0x0128: Default::default(),
            field_0x012c: Default::default(),
            field_0x0130: Default::default(),
            field_0x0134: Default::default(),
            field_0x0138: Default::default(),
            field_0x013c: Default::default(),
            field_0x0140: Default::default(),
            field_0x0144: Default::default(),
            field_0x0148: Default::default(),
            field_0x014c: Default::default(),
            field_0x0150: Default::default(),
            field_0x0154: Default::default(),
            field_0x0158: Default::default(),
            field_0x015c: Default::default(),
            field_0x016c: Default::default(),
            field_0x0170: Default::default(),
            field_0x01b8: Default::default(),
        }
    }

    /// Decompiled virtual method #0 @ RVA 0x004f86c0 (20 instructions, 3 basic blocks)
    pub fn method_00(&mut self) {
        // Writes to field at offset +0x0000
    }

    /// Decompiled virtual method #1 @ RVA 0x004f88b0 (95 instructions, 14 basic blocks)
    pub fn method_01(&mut self) {
    }

    /// Decompiled virtual method #2 @ RVA 0x0045e4c0 (42 instructions, 4 basic blocks)
    pub fn method_02(&mut self) {
        // Writes to field at offset +0x0090
        // Writes to field at offset +0x0094
        // Writes to field at offset +0x0098
        // Writes to field at offset +0x009c
        // Writes to field at offset +0x00a0
        // Writes to field at offset +0x00a4
        // Writes to field at offset +0x00a8
        // Writes to field at offset +0x00ac
        // Writes to field at offset +0x0140
        // Writes to field at offset +0x0144
        // Writes to field at offset +0x0148
        // Writes to field at offset +0x014c
        // Writes to field at offset +0x0150
        // Writes to field at offset +0x0154
        // Writes to field at offset +0x0158
        // Writes to field at offset +0x015c
    }

    /// Decompiled virtual method #3 @ RVA 0x000d3090 (1 instructions, 1 basic blocks)
    pub fn method_03(&mut self) {
    }

    /// Decompiled virtual method #4 @ RVA 0x0019e550 (2 instructions, 1 basic blocks)
    pub fn method_04(&mut self) {
        // Writes to field at offset +0x00c4
    }

    /// Decompiled virtual method #5 @ RVA 0x004f9050 (114 instructions, 23 basic blocks)
    pub fn method_05(&mut self) {
        // Writes to field at offset +0x0009
        // Writes to field at offset +0x0040
        // Writes to field at offset +0x016c
        // Writes to field at offset +0x01b8
    }

    /// Decompiled virtual method #6 @ RVA 0x004f8da0 (125 instructions, 41 basic blocks)
    pub fn method_06(&mut self) {
        // Writes to field at offset +0x0070
        // Writes to field at offset +0x00cc
    }

    /// Decompiled virtual method #7 @ RVA 0x000d3090 (1 instructions, 1 basic blocks)
    pub fn method_07(&mut self) {
    }

    /// Decompiled virtual method #8 @ RVA 0x000d3090 (1 instructions, 1 basic blocks)
    pub fn method_08(&mut self) {
    }

    /// Decompiled virtual method #9 @ RVA 0x000d3090 (1 instructions, 1 basic blocks)
    pub fn method_09(&mut self) {
    }

    /// Decompiled virtual method #10 @ RVA 0x004f8cf0 (34 instructions, 5 basic blocks)
    pub fn method_10(&mut self) {
        // Writes to field at offset +0x0090
        // Writes to field at offset +0x0094
        // Writes to field at offset +0x0098
        // Writes to field at offset +0x009c
        // Writes to field at offset +0x00a0
        // Writes to field at offset +0x00a4
        // Writes to field at offset +0x00a8
        // Writes to field at offset +0x00ac
    }

    /// Decompiled virtual method #11 @ RVA 0x0045e2d0 (33 instructions, 3 basic blocks)
    pub fn method_11(&mut self) {
        // Writes to field at offset +0x0090
        // Writes to field at offset +0x00a0
        // Writes to field at offset +0x00a4
        // Writes to field at offset +0x00a8
        // Writes to field at offset +0x00ac
    }

    /// Decompiled virtual method #12 @ RVA 0x0045eec0 (67 instructions, 22 basic blocks)
    pub fn method_12(&mut self) {
        // Writes to field at offset +0x001c
        // Writes to field at offset +0x0040
        // Writes to field at offset +0x0090
        // Writes to field at offset +0x00b0
    }

    /// Decompiled virtual method #13 @ RVA 0x0045efd0 (27 instructions, 5 basic blocks)
    pub fn method_13(&mut self) {
        // Writes to field at offset +0x0090
        // Writes to field at offset +0x00b0
    }

}

impl CcClass for ccGobjGimChimEat {
    fn class_name(&self) -> &'static str { Self::CLASS_NAME }
    fn vtable_rva(&self) -> u32 { Self::VTABLE_RVA }
    fn virtual_method_count(&self) -> usize { Self::METHOD_COUNT }
    fn base_classes(&self) -> &'static [&'static str] { Self::BASE_CLASSES }
}

/// Decompiled CC2 class: `ccGobjGimChimHole` (VTable RVA: 0x008beab0, 14 virtual methods, 36 recovered fields)
/// Inherits: ccGobjGimBaseChangeCCS, ccGobjGimBase, ccGameObj
#[derive(Debug, Clone, Default)]
pub struct ccGobjGimChimHole {
    pub vtable_rva: u32,
    pub field_0x0000: u64,
    pub field_0x0010: u64,
    pub field_0x0018: u64,
    pub field_0x001c: u8,
    pub field_0x0020: u32,
    pub field_0x0024: u16,
    pub field_0x002c: u16,
    pub field_0x0040: u32,
    pub field_0x0090: u32,
    pub field_0x0094: u32,
    pub field_0x0098: u32,
    pub field_0x009c: u32,
    pub field_0x00a0: u32,
    pub field_0x00a4: u32,
    pub field_0x00a8: u32,
    pub field_0x00ac: u32,
    pub field_0x00b0: u64,
    pub field_0x00b8: u64,
    pub field_0x00c4: u32,
    pub field_0x0120: u32,
    pub field_0x0124: u32,
    pub field_0x0128: u32,
    pub field_0x012c: u32,
    pub field_0x0130: u32,
    pub field_0x0134: u32,
    pub field_0x0138: u32,
    pub field_0x013c: u32,
    pub field_0x0140: u32,
    pub field_0x0144: u32,
    pub field_0x0148: u32,
    pub field_0x014c: u32,
    pub field_0x0150: u32,
    pub field_0x0154: u32,
    pub field_0x0158: u32,
    pub field_0x015c: u32,
    pub field_0x01b8: u64,
}

impl ccGobjGimChimHole {
    pub const CLASS_NAME: &'static str = "ccGobjGimChimHole";
    pub const VTABLE_RVA: u32 = 0x008beab0;
    pub const METHOD_COUNT: usize = 14;
    pub const BASE_CLASSES: &'static [&'static str] = &["ccGobjGimBaseChangeCCS", "ccGobjGimBase", "ccGameObj"];
    pub const VIRTUAL_METHODS: &'static [u64] = &[0x00000001804d6430, 0x00000001804d6490, 0x000000018045e4c0, 0x00000001800d3090, 0x000000018019e550, 0x00000001804d6560, 0x00000001800d3090, 0x00000001800d3090, 0x00000001800d3090, 0x00000001800d3090, 0x000000018045e220, 0x000000018045e2d0, 0x000000018045eec0, 0x000000018045efd0];

    pub fn new() -> Self {
        Self {
            vtable_rva: Self::VTABLE_RVA,
            field_0x0000: Default::default(),
            field_0x0010: Default::default(),
            field_0x0018: Default::default(),
            field_0x001c: Default::default(),
            field_0x0020: Default::default(),
            field_0x0024: Default::default(),
            field_0x002c: Default::default(),
            field_0x0040: Default::default(),
            field_0x0090: Default::default(),
            field_0x0094: Default::default(),
            field_0x0098: Default::default(),
            field_0x009c: Default::default(),
            field_0x00a0: Default::default(),
            field_0x00a4: Default::default(),
            field_0x00a8: Default::default(),
            field_0x00ac: Default::default(),
            field_0x00b0: Default::default(),
            field_0x00b8: Default::default(),
            field_0x00c4: Default::default(),
            field_0x0120: Default::default(),
            field_0x0124: Default::default(),
            field_0x0128: Default::default(),
            field_0x012c: Default::default(),
            field_0x0130: Default::default(),
            field_0x0134: Default::default(),
            field_0x0138: Default::default(),
            field_0x013c: Default::default(),
            field_0x0140: Default::default(),
            field_0x0144: Default::default(),
            field_0x0148: Default::default(),
            field_0x014c: Default::default(),
            field_0x0150: Default::default(),
            field_0x0154: Default::default(),
            field_0x0158: Default::default(),
            field_0x015c: Default::default(),
            field_0x01b8: Default::default(),
        }
    }

    /// Decompiled virtual method #0 @ RVA 0x004d6430 (22 instructions, 5 basic blocks)
    pub fn method_00(&mut self) {
        // Writes to field at offset +0x0000
    }

    /// Decompiled virtual method #1 @ RVA 0x004d6490 (11 instructions, 3 basic blocks)
    pub fn method_01(&mut self) {
    }

    /// Decompiled virtual method #2 @ RVA 0x0045e4c0 (42 instructions, 4 basic blocks)
    pub fn method_02(&mut self) {
        // Writes to field at offset +0x0090
        // Writes to field at offset +0x0094
        // Writes to field at offset +0x0098
        // Writes to field at offset +0x009c
        // Writes to field at offset +0x00a0
        // Writes to field at offset +0x00a4
        // Writes to field at offset +0x00a8
        // Writes to field at offset +0x00ac
        // Writes to field at offset +0x0140
        // Writes to field at offset +0x0144
        // Writes to field at offset +0x0148
        // Writes to field at offset +0x014c
        // Writes to field at offset +0x0150
        // Writes to field at offset +0x0154
        // Writes to field at offset +0x0158
        // Writes to field at offset +0x015c
    }

    /// Decompiled virtual method #3 @ RVA 0x000d3090 (1 instructions, 1 basic blocks)
    pub fn method_03(&mut self) {
    }

    /// Decompiled virtual method #4 @ RVA 0x0019e550 (2 instructions, 1 basic blocks)
    pub fn method_04(&mut self) {
        // Writes to field at offset +0x00c4
    }

    /// Decompiled virtual method #5 @ RVA 0x004d6560 (113 instructions, 9 basic blocks)
    pub fn method_05(&mut self) {
    }

    /// Decompiled virtual method #6 @ RVA 0x000d3090 (1 instructions, 1 basic blocks)
    pub fn method_06(&mut self) {
    }

    /// Decompiled virtual method #7 @ RVA 0x000d3090 (1 instructions, 1 basic blocks)
    pub fn method_07(&mut self) {
    }

    /// Decompiled virtual method #8 @ RVA 0x000d3090 (1 instructions, 1 basic blocks)
    pub fn method_08(&mut self) {
    }

    /// Decompiled virtual method #9 @ RVA 0x000d3090 (1 instructions, 1 basic blocks)
    pub fn method_09(&mut self) {
    }

    /// Decompiled virtual method #10 @ RVA 0x0045e220 (31 instructions, 3 basic blocks)
    pub fn method_10(&mut self) {
        // Writes to field at offset +0x0090
        // Writes to field at offset +0x00a0
        // Writes to field at offset +0x00a4
        // Writes to field at offset +0x00a8
        // Writes to field at offset +0x00ac
    }

    /// Decompiled virtual method #11 @ RVA 0x0045e2d0 (33 instructions, 3 basic blocks)
    pub fn method_11(&mut self) {
        // Writes to field at offset +0x0090
        // Writes to field at offset +0x00a0
        // Writes to field at offset +0x00a4
        // Writes to field at offset +0x00a8
        // Writes to field at offset +0x00ac
    }

    /// Decompiled virtual method #12 @ RVA 0x0045eec0 (67 instructions, 22 basic blocks)
    pub fn method_12(&mut self) {
        // Writes to field at offset +0x001c
        // Writes to field at offset +0x0040
        // Writes to field at offset +0x0090
        // Writes to field at offset +0x00b0
    }

    /// Decompiled virtual method #13 @ RVA 0x0045efd0 (27 instructions, 5 basic blocks)
    pub fn method_13(&mut self) {
        // Writes to field at offset +0x0090
        // Writes to field at offset +0x00b0
    }

}

impl CcClass for ccGobjGimChimHole {
    fn class_name(&self) -> &'static str { Self::CLASS_NAME }
    fn vtable_rva(&self) -> u32 { Self::VTABLE_RVA }
    fn virtual_method_count(&self) -> usize { Self::METHOD_COUNT }
    fn base_classes(&self) -> &'static [&'static str] { Self::BASE_CLASSES }
}

/// Decompiled CC2 class: `ccGobjGimChimHouse01` (VTable RVA: 0x008bed08, 14 virtual methods, 37 recovered fields)
/// Inherits: ccGobjGimBaseChangeCCS, ccGobjGimBase, ccGameObj
#[derive(Debug, Clone, Default)]
pub struct ccGobjGimChimHouse01 {
    pub vtable_rva: u32,
    pub field_0x0010: u64,
    pub field_0x0018: u64,
    pub field_0x001c: u8,
    pub field_0x0020: u32,
    pub field_0x0024: u16,
    pub field_0x002c: u16,
    pub field_0x0040: u32,
    pub field_0x0048: u64,
    pub field_0x0090: u32,
    pub field_0x0094: u32,
    pub field_0x0098: u32,
    pub field_0x009c: u32,
    pub field_0x00a0: u32,
    pub field_0x00a4: u32,
    pub field_0x00a8: u32,
    pub field_0x00ac: u32,
    pub field_0x00b0: u64,
    pub field_0x00b8: u64,
    pub field_0x00c4: u32,
    pub field_0x0120: u32,
    pub field_0x0124: u32,
    pub field_0x0128: u32,
    pub field_0x012c: u32,
    pub field_0x0130: u32,
    pub field_0x0134: u32,
    pub field_0x0138: u32,
    pub field_0x013c: u32,
    pub field_0x0140: u32,
    pub field_0x0144: u32,
    pub field_0x0148: u32,
    pub field_0x014c: u32,
    pub field_0x0150: u32,
    pub field_0x0154: u32,
    pub field_0x0158: u32,
    pub field_0x015c: u32,
    pub field_0x01e8: u32,
    pub field_0x01ec: u32,
}

impl ccGobjGimChimHouse01 {
    pub const CLASS_NAME: &'static str = "ccGobjGimChimHouse01";
    pub const VTABLE_RVA: u32 = 0x008bed08;
    pub const METHOD_COUNT: usize = 14;
    pub const BASE_CLASSES: &'static [&'static str] = &["ccGobjGimBaseChangeCCS", "ccGobjGimBase", "ccGameObj"];
    pub const VIRTUAL_METHODS: &'static [u64] = &[0x00000001804d40e0, 0x00000001804d4190, 0x000000018045e4c0, 0x00000001800d3090, 0x000000018019e550, 0x00000001804d4990, 0x00000001804d4400, 0x00000001800d3090, 0x00000001800d3090, 0x00000001804d45b0, 0x00000001804d4d60, 0x000000018045e2d0, 0x000000018045eec0, 0x000000018045efd0];

    pub fn new() -> Self {
        Self {
            vtable_rva: Self::VTABLE_RVA,
            field_0x0010: Default::default(),
            field_0x0018: Default::default(),
            field_0x001c: Default::default(),
            field_0x0020: Default::default(),
            field_0x0024: Default::default(),
            field_0x002c: Default::default(),
            field_0x0040: Default::default(),
            field_0x0048: Default::default(),
            field_0x0090: Default::default(),
            field_0x0094: Default::default(),
            field_0x0098: Default::default(),
            field_0x009c: Default::default(),
            field_0x00a0: Default::default(),
            field_0x00a4: Default::default(),
            field_0x00a8: Default::default(),
            field_0x00ac: Default::default(),
            field_0x00b0: Default::default(),
            field_0x00b8: Default::default(),
            field_0x00c4: Default::default(),
            field_0x0120: Default::default(),
            field_0x0124: Default::default(),
            field_0x0128: Default::default(),
            field_0x012c: Default::default(),
            field_0x0130: Default::default(),
            field_0x0134: Default::default(),
            field_0x0138: Default::default(),
            field_0x013c: Default::default(),
            field_0x0140: Default::default(),
            field_0x0144: Default::default(),
            field_0x0148: Default::default(),
            field_0x014c: Default::default(),
            field_0x0150: Default::default(),
            field_0x0154: Default::default(),
            field_0x0158: Default::default(),
            field_0x015c: Default::default(),
            field_0x01e8: Default::default(),
            field_0x01ec: Default::default(),
        }
    }

    /// Decompiled virtual method #0 @ RVA 0x004d40e0 (16 instructions, 3 basic blocks)
    pub fn method_00(&mut self) {
    }

    /// Decompiled virtual method #1 @ RVA 0x004d4190 (13 instructions, 3 basic blocks)
    pub fn method_01(&mut self) {
    }

    /// Decompiled virtual method #2 @ RVA 0x0045e4c0 (42 instructions, 4 basic blocks)
    pub fn method_02(&mut self) {
        // Writes to field at offset +0x0090
        // Writes to field at offset +0x0094
        // Writes to field at offset +0x0098
        // Writes to field at offset +0x009c
        // Writes to field at offset +0x00a0
        // Writes to field at offset +0x00a4
        // Writes to field at offset +0x00a8
        // Writes to field at offset +0x00ac
        // Writes to field at offset +0x0140
        // Writes to field at offset +0x0144
        // Writes to field at offset +0x0148
        // Writes to field at offset +0x014c
        // Writes to field at offset +0x0150
        // Writes to field at offset +0x0154
        // Writes to field at offset +0x0158
        // Writes to field at offset +0x015c
    }

    /// Decompiled virtual method #3 @ RVA 0x000d3090 (1 instructions, 1 basic blocks)
    pub fn method_03(&mut self) {
    }

    /// Decompiled virtual method #4 @ RVA 0x0019e550 (2 instructions, 1 basic blocks)
    pub fn method_04(&mut self) {
        // Writes to field at offset +0x00c4
    }

    /// Decompiled virtual method #5 @ RVA 0x004d4990 (116 instructions, 25 basic blocks)
    pub fn method_05(&mut self) {
        // Writes to field at offset +0x0024
    }

    /// Decompiled virtual method #6 @ RVA 0x004d4400 (21 instructions, 6 basic blocks)
    pub fn method_06(&mut self) {
        // Writes to field at offset +0x01e8
        // Writes to field at offset +0x01ec
    }

    /// Decompiled virtual method #7 @ RVA 0x000d3090 (1 instructions, 1 basic blocks)
    pub fn method_07(&mut self) {
    }

    /// Decompiled virtual method #8 @ RVA 0x000d3090 (1 instructions, 1 basic blocks)
    pub fn method_08(&mut self) {
    }

    /// Decompiled virtual method #9 @ RVA 0x004d45b0 (110 instructions, 28 basic blocks)
    pub fn method_09(&mut self) {
    }

    /// Decompiled virtual method #10 @ RVA 0x004d4d60 (33 instructions, 3 basic blocks)
    pub fn method_10(&mut self) {
        // Writes to field at offset +0x0090
        // Writes to field at offset +0x0094
        // Writes to field at offset +0x0098
        // Writes to field at offset +0x009c
        // Writes to field at offset +0x00a0
        // Writes to field at offset +0x00a4
        // Writes to field at offset +0x00a8
        // Writes to field at offset +0x00ac
    }

    /// Decompiled virtual method #11 @ RVA 0x0045e2d0 (33 instructions, 3 basic blocks)
    pub fn method_11(&mut self) {
        // Writes to field at offset +0x0090
        // Writes to field at offset +0x00a0
        // Writes to field at offset +0x00a4
        // Writes to field at offset +0x00a8
        // Writes to field at offset +0x00ac
    }

    /// Decompiled virtual method #12 @ RVA 0x0045eec0 (67 instructions, 22 basic blocks)
    pub fn method_12(&mut self) {
        // Writes to field at offset +0x001c
        // Writes to field at offset +0x0040
        // Writes to field at offset +0x0090
        // Writes to field at offset +0x00b0
    }

    /// Decompiled virtual method #13 @ RVA 0x0045efd0 (27 instructions, 5 basic blocks)
    pub fn method_13(&mut self) {
        // Writes to field at offset +0x0090
        // Writes to field at offset +0x00b0
    }

}

impl CcClass for ccGobjGimChimHouse01 {
    fn class_name(&self) -> &'static str { Self::CLASS_NAME }
    fn vtable_rva(&self) -> u32 { Self::VTABLE_RVA }
    fn virtual_method_count(&self) -> usize { Self::METHOD_COUNT }
    fn base_classes(&self) -> &'static [&'static str] { Self::BASE_CLASSES }
}

/// Decompiled CC2 class: `ccGobjGimChimRetriver` (VTable RVA: 0x008beba0, 14 virtual methods, 37 recovered fields)
/// Inherits: ccGobjGimBaseChangeCCS, ccGobjGimBase, ccGameObj
#[derive(Debug, Clone, Default)]
pub struct ccGobjGimChimRetriver {
    pub vtable_rva: u32,
    pub field_0x0000: u64,
    pub field_0x0009: u8,
    pub field_0x0010: u64,
    pub field_0x0018: u64,
    pub field_0x001c: u8,
    pub field_0x0020: u32,
    pub field_0x0024: u16,
    pub field_0x002c: u16,
    pub field_0x0040: u32,
    pub field_0x0058: u64,
    pub field_0x0090: u32,
    pub field_0x0094: u32,
    pub field_0x0098: u32,
    pub field_0x009c: u32,
    pub field_0x00a0: u32,
    pub field_0x00a4: u32,
    pub field_0x00a8: u32,
    pub field_0x00ac: u32,
    pub field_0x00b0: u64,
    pub field_0x00b8: u64,
    pub field_0x00c4: u32,
    pub field_0x0120: u32,
    pub field_0x0124: u32,
    pub field_0x0128: u32,
    pub field_0x012c: u32,
    pub field_0x0130: u32,
    pub field_0x0134: u32,
    pub field_0x0138: u32,
    pub field_0x013c: u32,
    pub field_0x0140: u32,
    pub field_0x0144: u32,
    pub field_0x0148: u32,
    pub field_0x014c: u32,
    pub field_0x0150: u32,
    pub field_0x0154: u32,
    pub field_0x0158: u32,
    pub field_0x015c: u32,
}

impl ccGobjGimChimRetriver {
    pub const CLASS_NAME: &'static str = "ccGobjGimChimRetriver";
    pub const VTABLE_RVA: u32 = 0x008beba0;
    pub const METHOD_COUNT: usize = 14;
    pub const BASE_CLASSES: &'static [&'static str] = &["ccGobjGimBaseChangeCCS", "ccGobjGimBase", "ccGameObj"];
    pub const VIRTUAL_METHODS: &'static [u64] = &[0x00000001804ca230, 0x00000001804d74c0, 0x000000018045e4c0, 0x00000001800d3090, 0x000000018019e550, 0x00000001804d7790, 0x00000001800d3090, 0x00000001800d3090, 0x00000001800d3090, 0x00000001800d3090, 0x000000018045e220, 0x000000018045e2d0, 0x000000018045eec0, 0x000000018045efd0];

    pub fn new() -> Self {
        Self {
            vtable_rva: Self::VTABLE_RVA,
            field_0x0000: Default::default(),
            field_0x0009: Default::default(),
            field_0x0010: Default::default(),
            field_0x0018: Default::default(),
            field_0x001c: Default::default(),
            field_0x0020: Default::default(),
            field_0x0024: Default::default(),
            field_0x002c: Default::default(),
            field_0x0040: Default::default(),
            field_0x0058: Default::default(),
            field_0x0090: Default::default(),
            field_0x0094: Default::default(),
            field_0x0098: Default::default(),
            field_0x009c: Default::default(),
            field_0x00a0: Default::default(),
            field_0x00a4: Default::default(),
            field_0x00a8: Default::default(),
            field_0x00ac: Default::default(),
            field_0x00b0: Default::default(),
            field_0x00b8: Default::default(),
            field_0x00c4: Default::default(),
            field_0x0120: Default::default(),
            field_0x0124: Default::default(),
            field_0x0128: Default::default(),
            field_0x012c: Default::default(),
            field_0x0130: Default::default(),
            field_0x0134: Default::default(),
            field_0x0138: Default::default(),
            field_0x013c: Default::default(),
            field_0x0140: Default::default(),
            field_0x0144: Default::default(),
            field_0x0148: Default::default(),
            field_0x014c: Default::default(),
            field_0x0150: Default::default(),
            field_0x0154: Default::default(),
            field_0x0158: Default::default(),
            field_0x015c: Default::default(),
        }
    }

    /// Decompiled virtual method #0 @ RVA 0x004ca230 (16 instructions, 3 basic blocks)
    pub fn method_00(&mut self) {
    }

    /// Decompiled virtual method #1 @ RVA 0x004d74c0 (13 instructions, 3 basic blocks)
    pub fn method_01(&mut self) {
    }

    /// Decompiled virtual method #2 @ RVA 0x0045e4c0 (42 instructions, 4 basic blocks)
    pub fn method_02(&mut self) {
        // Writes to field at offset +0x0090
        // Writes to field at offset +0x0094
        // Writes to field at offset +0x0098
        // Writes to field at offset +0x009c
        // Writes to field at offset +0x00a0
        // Writes to field at offset +0x00a4
        // Writes to field at offset +0x00a8
        // Writes to field at offset +0x00ac
        // Writes to field at offset +0x0140
        // Writes to field at offset +0x0144
        // Writes to field at offset +0x0148
        // Writes to field at offset +0x014c
        // Writes to field at offset +0x0150
        // Writes to field at offset +0x0154
        // Writes to field at offset +0x0158
        // Writes to field at offset +0x015c
    }

    /// Decompiled virtual method #3 @ RVA 0x000d3090 (1 instructions, 1 basic blocks)
    pub fn method_03(&mut self) {
    }

    /// Decompiled virtual method #4 @ RVA 0x0019e550 (2 instructions, 1 basic blocks)
    pub fn method_04(&mut self) {
        // Writes to field at offset +0x00c4
    }

    /// Decompiled virtual method #5 @ RVA 0x004d7790 (119 instructions, 30 basic blocks)
    pub fn method_05(&mut self) {
        // Writes to field at offset +0x0009
        // Writes to field at offset +0x0040
    }

    /// Decompiled virtual method #6 @ RVA 0x000d3090 (1 instructions, 1 basic blocks)
    pub fn method_06(&mut self) {
    }

    /// Decompiled virtual method #7 @ RVA 0x000d3090 (1 instructions, 1 basic blocks)
    pub fn method_07(&mut self) {
    }

    /// Decompiled virtual method #8 @ RVA 0x000d3090 (1 instructions, 1 basic blocks)
    pub fn method_08(&mut self) {
    }

    /// Decompiled virtual method #9 @ RVA 0x000d3090 (1 instructions, 1 basic blocks)
    pub fn method_09(&mut self) {
    }

    /// Decompiled virtual method #10 @ RVA 0x0045e220 (31 instructions, 3 basic blocks)
    pub fn method_10(&mut self) {
        // Writes to field at offset +0x0090
        // Writes to field at offset +0x00a0
        // Writes to field at offset +0x00a4
        // Writes to field at offset +0x00a8
        // Writes to field at offset +0x00ac
    }

    /// Decompiled virtual method #11 @ RVA 0x0045e2d0 (33 instructions, 3 basic blocks)
    pub fn method_11(&mut self) {
        // Writes to field at offset +0x0090
        // Writes to field at offset +0x00a0
        // Writes to field at offset +0x00a4
        // Writes to field at offset +0x00a8
        // Writes to field at offset +0x00ac
    }

    /// Decompiled virtual method #12 @ RVA 0x0045eec0 (67 instructions, 22 basic blocks)
    pub fn method_12(&mut self) {
        // Writes to field at offset +0x001c
        // Writes to field at offset +0x0040
        // Writes to field at offset +0x0090
        // Writes to field at offset +0x00b0
    }

    /// Decompiled virtual method #13 @ RVA 0x0045efd0 (27 instructions, 5 basic blocks)
    pub fn method_13(&mut self) {
        // Writes to field at offset +0x0090
        // Writes to field at offset +0x00b0
    }

}

impl CcClass for ccGobjGimChimRetriver {
    fn class_name(&self) -> &'static str { Self::CLASS_NAME }
    fn vtable_rva(&self) -> u32 { Self::VTABLE_RVA }
    fn virtual_method_count(&self) -> usize { Self::METHOD_COUNT }
    fn base_classes(&self) -> &'static [&'static str] { Self::BASE_CLASSES }
}

/// Decompiled CC2 class: `ccGobjGimChimTruck` (VTable RVA: 0x008be948, 14 virtual methods, 45 recovered fields)
/// Inherits: ccGobjGimBase, ccGameObj
#[derive(Debug, Clone, Default)]
pub struct ccGobjGimChimTruck {
    pub vtable_rva: u32,
    pub field_0x0000: u64,
    pub field_0x0009: u8,
    pub field_0x0010: u64,
    pub field_0x0018: u64,
    pub field_0x001c: u8,
    pub field_0x0020: u8,
    pub field_0x0024: u16,
    pub field_0x002c: u16,
    pub field_0x002e: u32,
    pub field_0x0034: u32,
    pub field_0x0040: u32,
    pub field_0x0058: u64,
    pub field_0x0090: u32,
    pub field_0x0094: u32,
    pub field_0x0098: u32,
    pub field_0x009c: u32,
    pub field_0x00a0: u32,
    pub field_0x00a4: u32,
    pub field_0x00a8: u32,
    pub field_0x00ac: u32,
    pub field_0x00b0: u64,
    pub field_0x00b8: u64,
    pub field_0x00c4: u32,
    pub field_0x0108: u16,
    pub field_0x010a: u8,
    pub field_0x0110: u64,
    pub field_0x0120: u32,
    pub field_0x0124: u32,
    pub field_0x0128: u32,
    pub field_0x012c: u32,
    pub field_0x0130: u32,
    pub field_0x0134: u32,
    pub field_0x0138: u32,
    pub field_0x013c: u32,
    pub field_0x0140: u32,
    pub field_0x0144: u32,
    pub field_0x0148: u32,
    pub field_0x014c: u32,
    pub field_0x0150: u32,
    pub field_0x0154: u32,
    pub field_0x0158: u32,
    pub field_0x015c: u32,
    pub field_0x01c0: u16,
    pub field_0x01c4: u32,
    pub field_0x01c8: u32,
}

impl ccGobjGimChimTruck {
    pub const CLASS_NAME: &'static str = "ccGobjGimChimTruck";
    pub const VTABLE_RVA: u32 = 0x008be948;
    pub const METHOD_COUNT: usize = 14;
    pub const BASE_CLASSES: &'static [&'static str] = &["ccGobjGimBase", "ccGameObj"];
    pub const VIRTUAL_METHODS: &'static [u64] = &[0x00000001804d9c30, 0x00000001804d9cb0, 0x000000018045e4c0, 0x00000001800d3090, 0x000000018019e550, 0x00000001804d6380, 0x00000001804d9e40, 0x00000001800d3090, 0x00000001800d3090, 0x00000001804c8c10, 0x00000001804d4d60, 0x000000018045e2d0, 0x000000018045eec0, 0x000000018045efd0];

    pub fn new() -> Self {
        Self {
            vtable_rva: Self::VTABLE_RVA,
            field_0x0000: Default::default(),
            field_0x0009: Default::default(),
            field_0x0010: Default::default(),
            field_0x0018: Default::default(),
            field_0x001c: Default::default(),
            field_0x0020: Default::default(),
            field_0x0024: Default::default(),
            field_0x002c: Default::default(),
            field_0x002e: Default::default(),
            field_0x0034: Default::default(),
            field_0x0040: Default::default(),
            field_0x0058: Default::default(),
            field_0x0090: Default::default(),
            field_0x0094: Default::default(),
            field_0x0098: Default::default(),
            field_0x009c: Default::default(),
            field_0x00a0: Default::default(),
            field_0x00a4: Default::default(),
            field_0x00a8: Default::default(),
            field_0x00ac: Default::default(),
            field_0x00b0: Default::default(),
            field_0x00b8: Default::default(),
            field_0x00c4: Default::default(),
            field_0x0108: Default::default(),
            field_0x010a: Default::default(),
            field_0x0110: Default::default(),
            field_0x0120: Default::default(),
            field_0x0124: Default::default(),
            field_0x0128: Default::default(),
            field_0x012c: Default::default(),
            field_0x0130: Default::default(),
            field_0x0134: Default::default(),
            field_0x0138: Default::default(),
            field_0x013c: Default::default(),
            field_0x0140: Default::default(),
            field_0x0144: Default::default(),
            field_0x0148: Default::default(),
            field_0x014c: Default::default(),
            field_0x0150: Default::default(),
            field_0x0154: Default::default(),
            field_0x0158: Default::default(),
            field_0x015c: Default::default(),
            field_0x01c0: Default::default(),
            field_0x01c4: Default::default(),
            field_0x01c8: Default::default(),
        }
    }

    /// Decompiled virtual method #0 @ RVA 0x004d9c30 (28 instructions, 6 basic blocks)
    pub fn method_00(&mut self) {
        // Writes to field at offset +0x0000
    }

    /// Decompiled virtual method #1 @ RVA 0x004d9cb0 (14 instructions, 3 basic blocks)
    pub fn method_01(&mut self) {
    }

    /// Decompiled virtual method #2 @ RVA 0x0045e4c0 (42 instructions, 4 basic blocks)
    pub fn method_02(&mut self) {
        // Writes to field at offset +0x0090
        // Writes to field at offset +0x0094
        // Writes to field at offset +0x0098
        // Writes to field at offset +0x009c
        // Writes to field at offset +0x00a0
        // Writes to field at offset +0x00a4
        // Writes to field at offset +0x00a8
        // Writes to field at offset +0x00ac
        // Writes to field at offset +0x0140
        // Writes to field at offset +0x0144
        // Writes to field at offset +0x0148
        // Writes to field at offset +0x014c
        // Writes to field at offset +0x0150
        // Writes to field at offset +0x0154
        // Writes to field at offset +0x0158
        // Writes to field at offset +0x015c
    }

    /// Decompiled virtual method #3 @ RVA 0x000d3090 (1 instructions, 1 basic blocks)
    pub fn method_03(&mut self) {
    }

    /// Decompiled virtual method #4 @ RVA 0x0019e550 (2 instructions, 1 basic blocks)
    pub fn method_04(&mut self) {
        // Writes to field at offset +0x00c4
    }

    /// Decompiled virtual method #5 @ RVA 0x004d6380 (85 instructions, 25 basic blocks)
    pub fn method_05(&mut self) {
        // Writes to field at offset +0x0000
        // Writes to field at offset +0x0024
    }

    /// Decompiled virtual method #6 @ RVA 0x004d9e40 (88 instructions, 16 basic blocks)
    pub fn method_06(&mut self) {
        // Writes to field at offset +0x0009
        // Writes to field at offset +0x0020
        // Writes to field at offset +0x002c
        // Writes to field at offset +0x002e
        // Writes to field at offset +0x0034
        // Writes to field at offset +0x0108
        // Writes to field at offset +0x010a
        // Writes to field at offset +0x0110
        // Writes to field at offset +0x01c8
    }

    /// Decompiled virtual method #7 @ RVA 0x000d3090 (1 instructions, 1 basic blocks)
    pub fn method_07(&mut self) {
    }

    /// Decompiled virtual method #8 @ RVA 0x000d3090 (1 instructions, 1 basic blocks)
    pub fn method_08(&mut self) {
    }

    /// Decompiled virtual method #9 @ RVA 0x004c8c10 (15 instructions, 4 basic blocks)
    pub fn method_09(&mut self) {
    }

    /// Decompiled virtual method #10 @ RVA 0x004d4d60 (33 instructions, 3 basic blocks)
    pub fn method_10(&mut self) {
        // Writes to field at offset +0x0090
        // Writes to field at offset +0x0094
        // Writes to field at offset +0x0098
        // Writes to field at offset +0x009c
        // Writes to field at offset +0x00a0
        // Writes to field at offset +0x00a4
        // Writes to field at offset +0x00a8
        // Writes to field at offset +0x00ac
    }

    /// Decompiled virtual method #11 @ RVA 0x0045e2d0 (33 instructions, 3 basic blocks)
    pub fn method_11(&mut self) {
        // Writes to field at offset +0x0090
        // Writes to field at offset +0x00a0
        // Writes to field at offset +0x00a4
        // Writes to field at offset +0x00a8
        // Writes to field at offset +0x00ac
    }

    /// Decompiled virtual method #12 @ RVA 0x0045eec0 (67 instructions, 22 basic blocks)
    pub fn method_12(&mut self) {
        // Writes to field at offset +0x001c
        // Writes to field at offset +0x0040
        // Writes to field at offset +0x0090
        // Writes to field at offset +0x00b0
    }

    /// Decompiled virtual method #13 @ RVA 0x0045efd0 (27 instructions, 5 basic blocks)
    pub fn method_13(&mut self) {
        // Writes to field at offset +0x0090
        // Writes to field at offset +0x00b0
    }

}

impl CcClass for ccGobjGimChimTruck {
    fn class_name(&self) -> &'static str { Self::CLASS_NAME }
    fn vtable_rva(&self) -> u32 { Self::VTABLE_RVA }
    fn virtual_method_count(&self) -> usize { Self::METHOD_COUNT }
    fn base_classes(&self) -> &'static [&'static str] { Self::BASE_CLASSES }
}

/// Decompiled CC2 class: `ccGobjGimChimWall` (VTable RVA: 0x008bec18, 14 virtual methods, 41 recovered fields)
/// Inherits: ccGobjGimBaseChangeCCS, ccGobjGimBase, ccGameObj
#[derive(Debug, Clone, Default)]
pub struct ccGobjGimChimWall {
    pub vtable_rva: u32,
    pub field_0x0000: u32,
    pub field_0x0004: u32,
    pub field_0x0008: u16,
    pub field_0x0009: u8,
    pub field_0x000a: u16,
    pub field_0x0010: u64,
    pub field_0x0018: u32,
    pub field_0x001c: u8,
    pub field_0x0020: u32,
    pub field_0x0024: u16,
    pub field_0x002c: u16,
    pub field_0x0040: u32,
    pub field_0x0050: u64,
    pub field_0x0058: u64,
    pub field_0x0090: u32,
    pub field_0x0094: u32,
    pub field_0x0098: u32,
    pub field_0x009c: u32,
    pub field_0x00a0: u32,
    pub field_0x00a4: u32,
    pub field_0x00a8: u32,
    pub field_0x00ac: u32,
    pub field_0x00b0: u64,
    pub field_0x00b8: u64,
    pub field_0x00c4: u32,
    pub field_0x0120: u32,
    pub field_0x0124: u32,
    pub field_0x0128: u32,
    pub field_0x012c: u32,
    pub field_0x0130: u32,
    pub field_0x0134: u32,
    pub field_0x0138: u32,
    pub field_0x013c: u32,
    pub field_0x0140: u32,
    pub field_0x0144: u32,
    pub field_0x0148: u32,
    pub field_0x014c: u32,
    pub field_0x0150: u32,
    pub field_0x0154: u32,
    pub field_0x0158: u32,
    pub field_0x015c: u32,
}

impl ccGobjGimChimWall {
    pub const CLASS_NAME: &'static str = "ccGobjGimChimWall";
    pub const VTABLE_RVA: u32 = 0x008bec18;
    pub const METHOD_COUNT: usize = 14;
    pub const BASE_CLASSES: &'static [&'static str] = &["ccGobjGimBaseChangeCCS", "ccGobjGimBase", "ccGameObj"];
    pub const VIRTUAL_METHODS: &'static [u64] = &[0x00000001804c8e90, 0x00000001804d7e70, 0x000000018045e4c0, 0x00000001800d3090, 0x000000018019e550, 0x00000001804d8320, 0x00000001804d80e0, 0x00000001800d3090, 0x00000001800d3090, 0x00000001800d3090, 0x000000018045e220, 0x000000018045e2d0, 0x000000018045eec0, 0x000000018045efd0];

    pub fn new() -> Self {
        Self {
            vtable_rva: Self::VTABLE_RVA,
            field_0x0000: Default::default(),
            field_0x0004: Default::default(),
            field_0x0008: Default::default(),
            field_0x0009: Default::default(),
            field_0x000a: Default::default(),
            field_0x0010: Default::default(),
            field_0x0018: Default::default(),
            field_0x001c: Default::default(),
            field_0x0020: Default::default(),
            field_0x0024: Default::default(),
            field_0x002c: Default::default(),
            field_0x0040: Default::default(),
            field_0x0050: Default::default(),
            field_0x0058: Default::default(),
            field_0x0090: Default::default(),
            field_0x0094: Default::default(),
            field_0x0098: Default::default(),
            field_0x009c: Default::default(),
            field_0x00a0: Default::default(),
            field_0x00a4: Default::default(),
            field_0x00a8: Default::default(),
            field_0x00ac: Default::default(),
            field_0x00b0: Default::default(),
            field_0x00b8: Default::default(),
            field_0x00c4: Default::default(),
            field_0x0120: Default::default(),
            field_0x0124: Default::default(),
            field_0x0128: Default::default(),
            field_0x012c: Default::default(),
            field_0x0130: Default::default(),
            field_0x0134: Default::default(),
            field_0x0138: Default::default(),
            field_0x013c: Default::default(),
            field_0x0140: Default::default(),
            field_0x0144: Default::default(),
            field_0x0148: Default::default(),
            field_0x014c: Default::default(),
            field_0x0150: Default::default(),
            field_0x0154: Default::default(),
            field_0x0158: Default::default(),
            field_0x015c: Default::default(),
        }
    }

    /// Decompiled virtual method #0 @ RVA 0x004c8e90 (16 instructions, 3 basic blocks)
    pub fn method_00(&mut self) {
    }

    /// Decompiled virtual method #1 @ RVA 0x004d7e70 (123 instructions, 27 basic blocks)
    pub fn method_01(&mut self) {
        // Writes to field at offset +0x0000
        // Writes to field at offset +0x0004
        // Writes to field at offset +0x0008
        // Writes to field at offset +0x000a
        // Writes to field at offset +0x0018
        // Writes to field at offset +0x001c
        // Writes to field at offset +0x0040
    }

    /// Decompiled virtual method #2 @ RVA 0x0045e4c0 (42 instructions, 4 basic blocks)
    pub fn method_02(&mut self) {
        // Writes to field at offset +0x0090
        // Writes to field at offset +0x0094
        // Writes to field at offset +0x0098
        // Writes to field at offset +0x009c
        // Writes to field at offset +0x00a0
        // Writes to field at offset +0x00a4
        // Writes to field at offset +0x00a8
        // Writes to field at offset +0x00ac
        // Writes to field at offset +0x0140
        // Writes to field at offset +0x0144
        // Writes to field at offset +0x0148
        // Writes to field at offset +0x014c
        // Writes to field at offset +0x0150
        // Writes to field at offset +0x0154
        // Writes to field at offset +0x0158
        // Writes to field at offset +0x015c
    }

    /// Decompiled virtual method #3 @ RVA 0x000d3090 (1 instructions, 1 basic blocks)
    pub fn method_03(&mut self) {
    }

    /// Decompiled virtual method #4 @ RVA 0x0019e550 (2 instructions, 1 basic blocks)
    pub fn method_04(&mut self) {
        // Writes to field at offset +0x00c4
    }

    /// Decompiled virtual method #5 @ RVA 0x004d8320 (115 instructions, 21 basic blocks)
    pub fn method_05(&mut self) {
        // Writes to field at offset +0x0009
        // Writes to field at offset +0x0040
    }

    /// Decompiled virtual method #6 @ RVA 0x004d80e0 (83 instructions, 9 basic blocks)
    pub fn method_06(&mut self) {
    }

    /// Decompiled virtual method #7 @ RVA 0x000d3090 (1 instructions, 1 basic blocks)
    pub fn method_07(&mut self) {
    }

    /// Decompiled virtual method #8 @ RVA 0x000d3090 (1 instructions, 1 basic blocks)
    pub fn method_08(&mut self) {
    }

    /// Decompiled virtual method #9 @ RVA 0x000d3090 (1 instructions, 1 basic blocks)
    pub fn method_09(&mut self) {
    }

    /// Decompiled virtual method #10 @ RVA 0x0045e220 (31 instructions, 3 basic blocks)
    pub fn method_10(&mut self) {
        // Writes to field at offset +0x0090
        // Writes to field at offset +0x00a0
        // Writes to field at offset +0x00a4
        // Writes to field at offset +0x00a8
        // Writes to field at offset +0x00ac
    }

    /// Decompiled virtual method #11 @ RVA 0x0045e2d0 (33 instructions, 3 basic blocks)
    pub fn method_11(&mut self) {
        // Writes to field at offset +0x0090
        // Writes to field at offset +0x00a0
        // Writes to field at offset +0x00a4
        // Writes to field at offset +0x00a8
        // Writes to field at offset +0x00ac
    }

    /// Decompiled virtual method #12 @ RVA 0x0045eec0 (67 instructions, 22 basic blocks)
    pub fn method_12(&mut self) {
        // Writes to field at offset +0x001c
        // Writes to field at offset +0x0040
        // Writes to field at offset +0x0090
        // Writes to field at offset +0x00b0
    }

    /// Decompiled virtual method #13 @ RVA 0x0045efd0 (27 instructions, 5 basic blocks)
    pub fn method_13(&mut self) {
        // Writes to field at offset +0x0090
        // Writes to field at offset +0x00b0
    }

}

impl CcClass for ccGobjGimChimWall {
    fn class_name(&self) -> &'static str { Self::CLASS_NAME }
    fn vtable_rva(&self) -> u32 { Self::VTABLE_RVA }
    fn virtual_method_count(&self) -> usize { Self::METHOD_COUNT }
    fn base_classes(&self) -> &'static [&'static str] { Self::BASE_CLASSES }
}

/// Decompiled CC2 class: `ccGobjGimCircle` (VTable RVA: 0x008bf578, 14 virtual methods, 49 recovered fields)
/// Inherits: ccGobjGimBase, ccGameObj
#[derive(Debug, Clone, Default)]
pub struct ccGobjGimCircle {
    pub vtable_rva: u32,
    pub field_0x0000: u32,
    pub field_0x0004: u32,
    pub field_0x0008: u16,
    pub field_0x000a: u16,
    pub field_0x0010: u16,
    pub field_0x0018: u64,
    pub field_0x001c: u8,
    pub field_0x0020: u32,
    pub field_0x0024: u16,
    pub field_0x002c: u16,
    pub field_0x0030: u32,
    pub field_0x0040: u32,
    pub field_0x0090: u32,
    pub field_0x0094: u32,
    pub field_0x0098: u32,
    pub field_0x009c: u32,
    pub field_0x00a0: u32,
    pub field_0x00a4: u32,
    pub field_0x00a8: u32,
    pub field_0x00ac: u32,
    pub field_0x00b0: u64,
    pub field_0x00b8: u64,
    pub field_0x00c0: u16,
    pub field_0x00c4: u32,
    pub field_0x00e0: u32,
    pub field_0x00e4: u32,
    pub field_0x0120: u32,
    pub field_0x0124: u32,
    pub field_0x0128: u32,
    pub field_0x012c: u32,
    pub field_0x0130: u32,
    pub field_0x0134: u32,
    pub field_0x0138: u32,
    pub field_0x013c: u32,
    pub field_0x0140: u32,
    pub field_0x0144: u32,
    pub field_0x0148: u32,
    pub field_0x014c: u32,
    pub field_0x0150: u32,
    pub field_0x0154: u32,
    pub field_0x0158: u32,
    pub field_0x015c: u32,
    pub field_0x01f0: u64,
    pub field_0x020c: u32,
    pub field_0x0212: u8,
    pub field_0x0340: u32,
    pub field_0x0344: u32,
    pub field_0x0348: u32,
    pub field_0x034c: u32,
}

impl ccGobjGimCircle {
    pub const CLASS_NAME: &'static str = "ccGobjGimCircle";
    pub const VTABLE_RVA: u32 = 0x008bf578;
    pub const METHOD_COUNT: usize = 14;
    pub const BASE_CLASSES: &'static [&'static str] = &["ccGobjGimBase", "ccGameObj"];
    pub const VIRTUAL_METHODS: &'static [u64] = &[0x00000001804c55e0, 0x00000001804c5770, 0x000000018045e4c0, 0x00000001800d3090, 0x000000018019e550, 0x00000001804fd5c0, 0x00000001804c5950, 0x00000001800d3090, 0x00000001800d3090, 0x00000001804c60c0, 0x00000001804c7c50, 0x000000018045e2d0, 0x000000018045eec0, 0x000000018045efd0];

    pub fn new() -> Self {
        Self {
            vtable_rva: Self::VTABLE_RVA,
            field_0x0000: Default::default(),
            field_0x0004: Default::default(),
            field_0x0008: Default::default(),
            field_0x000a: Default::default(),
            field_0x0010: Default::default(),
            field_0x0018: Default::default(),
            field_0x001c: Default::default(),
            field_0x0020: Default::default(),
            field_0x0024: Default::default(),
            field_0x002c: Default::default(),
            field_0x0030: Default::default(),
            field_0x0040: Default::default(),
            field_0x0090: Default::default(),
            field_0x0094: Default::default(),
            field_0x0098: Default::default(),
            field_0x009c: Default::default(),
            field_0x00a0: Default::default(),
            field_0x00a4: Default::default(),
            field_0x00a8: Default::default(),
            field_0x00ac: Default::default(),
            field_0x00b0: Default::default(),
            field_0x00b8: Default::default(),
            field_0x00c0: Default::default(),
            field_0x00c4: Default::default(),
            field_0x00e0: Default::default(),
            field_0x00e4: Default::default(),
            field_0x0120: Default::default(),
            field_0x0124: Default::default(),
            field_0x0128: Default::default(),
            field_0x012c: Default::default(),
            field_0x0130: Default::default(),
            field_0x0134: Default::default(),
            field_0x0138: Default::default(),
            field_0x013c: Default::default(),
            field_0x0140: Default::default(),
            field_0x0144: Default::default(),
            field_0x0148: Default::default(),
            field_0x014c: Default::default(),
            field_0x0150: Default::default(),
            field_0x0154: Default::default(),
            field_0x0158: Default::default(),
            field_0x015c: Default::default(),
            field_0x01f0: Default::default(),
            field_0x020c: Default::default(),
            field_0x0212: Default::default(),
            field_0x0340: Default::default(),
            field_0x0344: Default::default(),
            field_0x0348: Default::default(),
            field_0x034c: Default::default(),
        }
    }

    /// Decompiled virtual method #0 @ RVA 0x004c55e0 (16 instructions, 3 basic blocks)
    pub fn method_00(&mut self) {
    }

    /// Decompiled virtual method #1 @ RVA 0x004c5770 (96 instructions, 10 basic blocks)
    pub fn method_01(&mut self) {
        // Writes to field at offset +0x0000
        // Writes to field at offset +0x0004
        // Writes to field at offset +0x0008
        // Writes to field at offset +0x000a
        // Writes to field at offset +0x0212
    }

    /// Decompiled virtual method #2 @ RVA 0x0045e4c0 (42 instructions, 4 basic blocks)
    pub fn method_02(&mut self) {
        // Writes to field at offset +0x0090
        // Writes to field at offset +0x0094
        // Writes to field at offset +0x0098
        // Writes to field at offset +0x009c
        // Writes to field at offset +0x00a0
        // Writes to field at offset +0x00a4
        // Writes to field at offset +0x00a8
        // Writes to field at offset +0x00ac
        // Writes to field at offset +0x0140
        // Writes to field at offset +0x0144
        // Writes to field at offset +0x0148
        // Writes to field at offset +0x014c
        // Writes to field at offset +0x0150
        // Writes to field at offset +0x0154
        // Writes to field at offset +0x0158
        // Writes to field at offset +0x015c
    }

    /// Decompiled virtual method #3 @ RVA 0x000d3090 (1 instructions, 1 basic blocks)
    pub fn method_03(&mut self) {
    }

    /// Decompiled virtual method #4 @ RVA 0x0019e550 (2 instructions, 1 basic blocks)
    pub fn method_04(&mut self) {
        // Writes to field at offset +0x00c4
    }

    /// Decompiled virtual method #5 @ RVA 0x004fd5c0 (136 instructions, 36 basic blocks)
    pub fn method_05(&mut self) {
        // Writes to field at offset +0x0000
        // Writes to field at offset +0x0004
        // Writes to field at offset +0x0008
        // Writes to field at offset +0x000a
        // Writes to field at offset +0x0024
        // Writes to field at offset +0x00b0
        // Writes to field at offset +0x00c0
    }

    /// Decompiled virtual method #6 @ RVA 0x004c5950 (108 instructions, 21 basic blocks)
    pub fn method_06(&mut self) {
    }

    /// Decompiled virtual method #7 @ RVA 0x000d3090 (1 instructions, 1 basic blocks)
    pub fn method_07(&mut self) {
    }

    /// Decompiled virtual method #8 @ RVA 0x000d3090 (1 instructions, 1 basic blocks)
    pub fn method_08(&mut self) {
    }

    /// Decompiled virtual method #9 @ RVA 0x004c60c0 (109 instructions, 26 basic blocks)
    pub fn method_09(&mut self) {
        // Writes to field at offset +0x0008
    }

    /// Decompiled virtual method #10 @ RVA 0x004c7c50 (6 instructions, 3 basic blocks)
    pub fn method_10(&mut self) {
        // Writes to field at offset +0x0212
    }

    /// Decompiled virtual method #11 @ RVA 0x0045e2d0 (33 instructions, 3 basic blocks)
    pub fn method_11(&mut self) {
        // Writes to field at offset +0x0090
        // Writes to field at offset +0x00a0
        // Writes to field at offset +0x00a4
        // Writes to field at offset +0x00a8
        // Writes to field at offset +0x00ac
    }

    /// Decompiled virtual method #12 @ RVA 0x0045eec0 (67 instructions, 22 basic blocks)
    pub fn method_12(&mut self) {
        // Writes to field at offset +0x001c
        // Writes to field at offset +0x0040
        // Writes to field at offset +0x0090
        // Writes to field at offset +0x00b0
    }

    /// Decompiled virtual method #13 @ RVA 0x0045efd0 (27 instructions, 5 basic blocks)
    pub fn method_13(&mut self) {
        // Writes to field at offset +0x0090
        // Writes to field at offset +0x00b0
    }

}

impl CcClass for ccGobjGimCircle {
    fn class_name(&self) -> &'static str { Self::CLASS_NAME }
    fn vtable_rva(&self) -> u32 { Self::VTABLE_RVA }
    fn virtual_method_count(&self) -> usize { Self::METHOD_COUNT }
    fn base_classes(&self) -> &'static [&'static str] { Self::BASE_CLASSES }
}

/// Decompiled CC2 class: `ccGobjGimDungeonDoor` (VTable RVA: 0x008bf1b8, 14 virtual methods, 44 recovered fields)
/// Inherits: ccGobjGimBaseChangeCCS, ccGobjGimBase, ccGameObj
#[derive(Debug, Clone, Default)]
pub struct ccGobjGimDungeonDoor {
    pub vtable_rva: u32,
    pub field_0x0000: u32,
    pub field_0x0004: u32,
    pub field_0x0008: u16,
    pub field_0x0009: u8,
    pub field_0x000a: u16,
    pub field_0x0010: u64,
    pub field_0x0018: u64,
    pub field_0x001c: u8,
    pub field_0x0020: u32,
    pub field_0x0024: u16,
    pub field_0x002c: u16,
    pub field_0x0040: u32,
    pub field_0x0050: u64,
    pub field_0x0058: u64,
    pub field_0x0090: u32,
    pub field_0x0094: u32,
    pub field_0x0098: u32,
    pub field_0x009c: u32,
    pub field_0x00a0: u32,
    pub field_0x00a4: u32,
    pub field_0x00a8: u32,
    pub field_0x00ac: u32,
    pub field_0x00b0: u64,
    pub field_0x00b8: u64,
    pub field_0x00c4: u32,
    pub field_0x00cc: u32,
    pub field_0x00d0: u32,
    pub field_0x0120: u32,
    pub field_0x0124: u32,
    pub field_0x0128: u32,
    pub field_0x012c: u32,
    pub field_0x0130: u32,
    pub field_0x0134: u32,
    pub field_0x0138: u32,
    pub field_0x013c: u32,
    pub field_0x0140: u32,
    pub field_0x0144: u32,
    pub field_0x0148: u32,
    pub field_0x014c: u32,
    pub field_0x0150: u32,
    pub field_0x0154: u32,
    pub field_0x0158: u32,
    pub field_0x015c: u32,
    pub field_0x01e0: u32,
}

impl ccGobjGimDungeonDoor {
    pub const CLASS_NAME: &'static str = "ccGobjGimDungeonDoor";
    pub const VTABLE_RVA: u32 = 0x008bf1b8;
    pub const METHOD_COUNT: usize = 14;
    pub const BASE_CLASSES: &'static [&'static str] = &["ccGobjGimBaseChangeCCS", "ccGobjGimBase", "ccGameObj"];
    pub const VIRTUAL_METHODS: &'static [u64] = &[0x00000001804ccf20, 0x00000001804ccf70, 0x000000018045e4c0, 0x00000001800d3090, 0x000000018019e550, 0x00000001804cd3e0, 0x00000001804cd290, 0x00000001800d3090, 0x00000001800d3090, 0x00000001804cd2a0, 0x00000001804cdda0, 0x000000018045e2d0, 0x000000018045eec0, 0x000000018045efd0];

    pub fn new() -> Self {
        Self {
            vtable_rva: Self::VTABLE_RVA,
            field_0x0000: Default::default(),
            field_0x0004: Default::default(),
            field_0x0008: Default::default(),
            field_0x0009: Default::default(),
            field_0x000a: Default::default(),
            field_0x0010: Default::default(),
            field_0x0018: Default::default(),
            field_0x001c: Default::default(),
            field_0x0020: Default::default(),
            field_0x0024: Default::default(),
            field_0x002c: Default::default(),
            field_0x0040: Default::default(),
            field_0x0050: Default::default(),
            field_0x0058: Default::default(),
            field_0x0090: Default::default(),
            field_0x0094: Default::default(),
            field_0x0098: Default::default(),
            field_0x009c: Default::default(),
            field_0x00a0: Default::default(),
            field_0x00a4: Default::default(),
            field_0x00a8: Default::default(),
            field_0x00ac: Default::default(),
            field_0x00b0: Default::default(),
            field_0x00b8: Default::default(),
            field_0x00c4: Default::default(),
            field_0x00cc: Default::default(),
            field_0x00d0: Default::default(),
            field_0x0120: Default::default(),
            field_0x0124: Default::default(),
            field_0x0128: Default::default(),
            field_0x012c: Default::default(),
            field_0x0130: Default::default(),
            field_0x0134: Default::default(),
            field_0x0138: Default::default(),
            field_0x013c: Default::default(),
            field_0x0140: Default::default(),
            field_0x0144: Default::default(),
            field_0x0148: Default::default(),
            field_0x014c: Default::default(),
            field_0x0150: Default::default(),
            field_0x0154: Default::default(),
            field_0x0158: Default::default(),
            field_0x015c: Default::default(),
            field_0x01e0: Default::default(),
        }
    }

    /// Decompiled virtual method #0 @ RVA 0x004ccf20 (16 instructions, 3 basic blocks)
    pub fn method_00(&mut self) {
    }

    /// Decompiled virtual method #1 @ RVA 0x004ccf70 (13 instructions, 3 basic blocks)
    pub fn method_01(&mut self) {
    }

    /// Decompiled virtual method #2 @ RVA 0x0045e4c0 (42 instructions, 4 basic blocks)
    pub fn method_02(&mut self) {
        // Writes to field at offset +0x0090
        // Writes to field at offset +0x0094
        // Writes to field at offset +0x0098
        // Writes to field at offset +0x009c
        // Writes to field at offset +0x00a0
        // Writes to field at offset +0x00a4
        // Writes to field at offset +0x00a8
        // Writes to field at offset +0x00ac
        // Writes to field at offset +0x0140
        // Writes to field at offset +0x0144
        // Writes to field at offset +0x0148
        // Writes to field at offset +0x014c
        // Writes to field at offset +0x0150
        // Writes to field at offset +0x0154
        // Writes to field at offset +0x0158
        // Writes to field at offset +0x015c
    }

    /// Decompiled virtual method #3 @ RVA 0x000d3090 (1 instructions, 1 basic blocks)
    pub fn method_03(&mut self) {
    }

    /// Decompiled virtual method #4 @ RVA 0x0019e550 (2 instructions, 1 basic blocks)
    pub fn method_04(&mut self) {
        // Writes to field at offset +0x00c4
    }

    /// Decompiled virtual method #5 @ RVA 0x004cd3e0 (115 instructions, 28 basic blocks)
    pub fn method_05(&mut self) {
        // Writes to field at offset +0x0009
        // Writes to field at offset +0x0040
        // Writes to field at offset +0x00cc
        // Writes to field at offset +0x00d0
    }

    /// Decompiled virtual method #6 @ RVA 0x004cd290 (3 instructions, 2 basic blocks)
    pub fn method_06(&mut self) {
        // Writes to field at offset +0x01e0
    }

    /// Decompiled virtual method #7 @ RVA 0x000d3090 (1 instructions, 1 basic blocks)
    pub fn method_07(&mut self) {
    }

    /// Decompiled virtual method #8 @ RVA 0x000d3090 (1 instructions, 1 basic blocks)
    pub fn method_08(&mut self) {
    }

    /// Decompiled virtual method #9 @ RVA 0x004cd2a0 (69 instructions, 12 basic blocks)
    pub fn method_09(&mut self) {
        // Writes to field at offset +0x0000
        // Writes to field at offset +0x0004
        // Writes to field at offset +0x0008
        // Writes to field at offset +0x000a
        // Writes to field at offset +0x0010
    }

    /// Decompiled virtual method #10 @ RVA 0x004cdda0 (33 instructions, 3 basic blocks)
    pub fn method_10(&mut self) {
        // Writes to field at offset +0x0090
        // Writes to field at offset +0x0094
        // Writes to field at offset +0x0098
        // Writes to field at offset +0x009c
        // Writes to field at offset +0x00a0
        // Writes to field at offset +0x00a4
        // Writes to field at offset +0x00a8
        // Writes to field at offset +0x00ac
    }

    /// Decompiled virtual method #11 @ RVA 0x0045e2d0 (33 instructions, 3 basic blocks)
    pub fn method_11(&mut self) {
        // Writes to field at offset +0x0090
        // Writes to field at offset +0x00a0
        // Writes to field at offset +0x00a4
        // Writes to field at offset +0x00a8
        // Writes to field at offset +0x00ac
    }

    /// Decompiled virtual method #12 @ RVA 0x0045eec0 (67 instructions, 22 basic blocks)
    pub fn method_12(&mut self) {
        // Writes to field at offset +0x001c
        // Writes to field at offset +0x0040
        // Writes to field at offset +0x0090
        // Writes to field at offset +0x00b0
    }

    /// Decompiled virtual method #13 @ RVA 0x0045efd0 (27 instructions, 5 basic blocks)
    pub fn method_13(&mut self) {
        // Writes to field at offset +0x0090
        // Writes to field at offset +0x00b0
    }

}

impl CcClass for ccGobjGimDungeonDoor {
    fn class_name(&self) -> &'static str { Self::CLASS_NAME }
    fn vtable_rva(&self) -> u32 { Self::VTABLE_RVA }
    fn virtual_method_count(&self) -> usize { Self::METHOD_COUNT }
    fn base_classes(&self) -> &'static [&'static str] { Self::BASE_CLASSES }
}

/// Decompiled CC2 class: `ccGobjGimDungeonLift` (VTable RVA: 0x008be7e0, 14 virtual methods, 45 recovered fields)
/// Inherits: ccGobjGimBase, ccGameObj
#[derive(Debug, Clone, Default)]
pub struct ccGobjGimDungeonLift {
    pub vtable_rva: u32,
    pub field_0x0000: u64,
    pub field_0x0002: u32,
    pub field_0x0004: u32,
    pub field_0x0008: u16,
    pub field_0x000a: u16,
    pub field_0x0010: u64,
    pub field_0x0018: u64,
    pub field_0x001c: u8,
    pub field_0x0020: u32,
    pub field_0x0024: u16,
    pub field_0x002c: u16,
    pub field_0x0040: u32,
    pub field_0x0050: u64,
    pub field_0x0090: u32,
    pub field_0x0094: u32,
    pub field_0x0098: u32,
    pub field_0x009c: u32,
    pub field_0x00a0: u32,
    pub field_0x00a4: u32,
    pub field_0x00a8: u32,
    pub field_0x00ac: u32,
    pub field_0x00b0: u64,
    pub field_0x00b8: u64,
    pub field_0x00c4: u32,
    pub field_0x0120: u32,
    pub field_0x0124: u32,
    pub field_0x0128: u32,
    pub field_0x012c: u32,
    pub field_0x0130: u32,
    pub field_0x0134: u32,
    pub field_0x0138: u32,
    pub field_0x013c: u32,
    pub field_0x0140: u32,
    pub field_0x0144: u32,
    pub field_0x0148: u32,
    pub field_0x014c: u32,
    pub field_0x0150: u32,
    pub field_0x0154: u32,
    pub field_0x0158: u32,
    pub field_0x015c: u32,
    pub field_0x01c0: u32,
    pub field_0x01c4: u32,
    pub field_0x01c8: u32,
    pub field_0x01cc: u8,
    pub field_0x01d0: u64,
}

impl ccGobjGimDungeonLift {
    pub const CLASS_NAME: &'static str = "ccGobjGimDungeonLift";
    pub const VTABLE_RVA: u32 = 0x008be7e0;
    pub const METHOD_COUNT: usize = 14;
    pub const BASE_CLASSES: &'static [&'static str] = &["ccGobjGimBase", "ccGameObj"];
    pub const VIRTUAL_METHODS: &'static [u64] = &[0x00000001804da4c0, 0x00000001804da560, 0x000000018045e4c0, 0x00000001800d3090, 0x000000018019e550, 0x00000001804dadb0, 0x00000001804daad0, 0x00000001800d3090, 0x00000001800d3090, 0x00000001800d3090, 0x00000001804da8d0, 0x000000018045e2d0, 0x000000018045eec0, 0x000000018045efd0];

    pub fn new() -> Self {
        Self {
            vtable_rva: Self::VTABLE_RVA,
            field_0x0000: Default::default(),
            field_0x0002: Default::default(),
            field_0x0004: Default::default(),
            field_0x0008: Default::default(),
            field_0x000a: Default::default(),
            field_0x0010: Default::default(),
            field_0x0018: Default::default(),
            field_0x001c: Default::default(),
            field_0x0020: Default::default(),
            field_0x0024: Default::default(),
            field_0x002c: Default::default(),
            field_0x0040: Default::default(),
            field_0x0050: Default::default(),
            field_0x0090: Default::default(),
            field_0x0094: Default::default(),
            field_0x0098: Default::default(),
            field_0x009c: Default::default(),
            field_0x00a0: Default::default(),
            field_0x00a4: Default::default(),
            field_0x00a8: Default::default(),
            field_0x00ac: Default::default(),
            field_0x00b0: Default::default(),
            field_0x00b8: Default::default(),
            field_0x00c4: Default::default(),
            field_0x0120: Default::default(),
            field_0x0124: Default::default(),
            field_0x0128: Default::default(),
            field_0x012c: Default::default(),
            field_0x0130: Default::default(),
            field_0x0134: Default::default(),
            field_0x0138: Default::default(),
            field_0x013c: Default::default(),
            field_0x0140: Default::default(),
            field_0x0144: Default::default(),
            field_0x0148: Default::default(),
            field_0x014c: Default::default(),
            field_0x0150: Default::default(),
            field_0x0154: Default::default(),
            field_0x0158: Default::default(),
            field_0x015c: Default::default(),
            field_0x01c0: Default::default(),
            field_0x01c4: Default::default(),
            field_0x01c8: Default::default(),
            field_0x01cc: Default::default(),
            field_0x01d0: Default::default(),
        }
    }

    /// Decompiled virtual method #0 @ RVA 0x004da4c0 (39 instructions, 5 basic blocks)
    pub fn method_00(&mut self) {
        // Writes to field at offset +0x0000
    }

    /// Decompiled virtual method #1 @ RVA 0x004da560 (114 instructions, 15 basic blocks)
    pub fn method_01(&mut self) {
        // Writes to field at offset +0x0000
        // Writes to field at offset +0x0004
        // Writes to field at offset +0x0008
        // Writes to field at offset +0x000a
        // Writes to field at offset +0x0040
    }

    /// Decompiled virtual method #2 @ RVA 0x0045e4c0 (42 instructions, 4 basic blocks)
    pub fn method_02(&mut self) {
        // Writes to field at offset +0x0090
        // Writes to field at offset +0x0094
        // Writes to field at offset +0x0098
        // Writes to field at offset +0x009c
        // Writes to field at offset +0x00a0
        // Writes to field at offset +0x00a4
        // Writes to field at offset +0x00a8
        // Writes to field at offset +0x00ac
        // Writes to field at offset +0x0140
        // Writes to field at offset +0x0144
        // Writes to field at offset +0x0148
        // Writes to field at offset +0x014c
        // Writes to field at offset +0x0150
        // Writes to field at offset +0x0154
        // Writes to field at offset +0x0158
        // Writes to field at offset +0x015c
    }

    /// Decompiled virtual method #3 @ RVA 0x000d3090 (1 instructions, 1 basic blocks)
    pub fn method_03(&mut self) {
    }

    /// Decompiled virtual method #4 @ RVA 0x0019e550 (2 instructions, 1 basic blocks)
    pub fn method_04(&mut self) {
        // Writes to field at offset +0x00c4
    }

    /// Decompiled virtual method #5 @ RVA 0x004dadb0 (118 instructions, 33 basic blocks)
    pub fn method_05(&mut self) {
        // Writes to field at offset +0x0024
    }

    /// Decompiled virtual method #6 @ RVA 0x004daad0 (99 instructions, 7 basic blocks)
    pub fn method_06(&mut self) {
        // Writes to field at offset +0x01c8
    }

    /// Decompiled virtual method #7 @ RVA 0x000d3090 (1 instructions, 1 basic blocks)
    pub fn method_07(&mut self) {
    }

    /// Decompiled virtual method #8 @ RVA 0x000d3090 (1 instructions, 1 basic blocks)
    pub fn method_08(&mut self) {
    }

    /// Decompiled virtual method #9 @ RVA 0x000d3090 (1 instructions, 1 basic blocks)
    pub fn method_09(&mut self) {
    }

    /// Decompiled virtual method #10 @ RVA 0x004da8d0 (102 instructions, 11 basic blocks)
    pub fn method_10(&mut self) {
        // Writes to field at offset +0x0090
        // Writes to field at offset +0x0094
        // Writes to field at offset +0x0098
        // Writes to field at offset +0x009c
        // Writes to field at offset +0x00a0
        // Writes to field at offset +0x00a4
        // Writes to field at offset +0x00a8
        // Writes to field at offset +0x00ac
        // Writes to field at offset +0x01cc
    }

    /// Decompiled virtual method #11 @ RVA 0x0045e2d0 (33 instructions, 3 basic blocks)
    pub fn method_11(&mut self) {
        // Writes to field at offset +0x0090
        // Writes to field at offset +0x00a0
        // Writes to field at offset +0x00a4
        // Writes to field at offset +0x00a8
        // Writes to field at offset +0x00ac
    }

    /// Decompiled virtual method #12 @ RVA 0x0045eec0 (67 instructions, 22 basic blocks)
    pub fn method_12(&mut self) {
        // Writes to field at offset +0x001c
        // Writes to field at offset +0x0040
        // Writes to field at offset +0x0090
        // Writes to field at offset +0x00b0
    }

    /// Decompiled virtual method #13 @ RVA 0x0045efd0 (27 instructions, 5 basic blocks)
    pub fn method_13(&mut self) {
        // Writes to field at offset +0x0090
        // Writes to field at offset +0x00b0
    }

}

impl CcClass for ccGobjGimDungeonLift {
    fn class_name(&self) -> &'static str { Self::CLASS_NAME }
    fn vtable_rva(&self) -> u32 { Self::VTABLE_RVA }
    fn virtual_method_count(&self) -> usize { Self::METHOD_COUNT }
    fn base_classes(&self) -> &'static [&'static str] { Self::BASE_CLASSES }
}

/// Decompiled CC2 class: `ccGobjGimEvMachine` (VTable RVA: 0x008be768, 14 virtual methods, 36 recovered fields)
/// Inherits: ccGobjGimBase, ccGameObj
#[derive(Debug, Clone, Default)]
pub struct ccGobjGimEvMachine {
    pub vtable_rva: u32,
    pub field_0x0010: u64,
    pub field_0x0018: u64,
    pub field_0x001c: u8,
    pub field_0x0020: u32,
    pub field_0x0024: u16,
    pub field_0x002c: u16,
    pub field_0x0040: u32,
    pub field_0x0050: u64,
    pub field_0x0090: u32,
    pub field_0x0094: u32,
    pub field_0x0098: u32,
    pub field_0x009c: u32,
    pub field_0x00a0: u32,
    pub field_0x00a4: u32,
    pub field_0x00a8: u32,
    pub field_0x00ac: u32,
    pub field_0x00b0: u64,
    pub field_0x00b8: u64,
    pub field_0x00c4: u32,
    pub field_0x0120: u32,
    pub field_0x0124: u32,
    pub field_0x0128: u32,
    pub field_0x012c: u32,
    pub field_0x0130: u32,
    pub field_0x0134: u32,
    pub field_0x0138: u32,
    pub field_0x013c: u32,
    pub field_0x0140: u32,
    pub field_0x0144: u32,
    pub field_0x0148: u32,
    pub field_0x014c: u32,
    pub field_0x0150: u32,
    pub field_0x0154: u32,
    pub field_0x0158: u32,
    pub field_0x015c: u32,
    pub field_0x0168: u64,
}

impl ccGobjGimEvMachine {
    pub const CLASS_NAME: &'static str = "ccGobjGimEvMachine";
    pub const VTABLE_RVA: u32 = 0x008be768;
    pub const METHOD_COUNT: usize = 14;
    pub const BASE_CLASSES: &'static [&'static str] = &["ccGobjGimBase", "ccGameObj"];
    pub const VIRTUAL_METHODS: &'static [u64] = &[0x00000001804c8e90, 0x00000001804da070, 0x000000018045e4c0, 0x00000001800d3090, 0x000000018019e550, 0x00000001804da260, 0x00000001800d3090, 0x00000001800d3090, 0x00000001800d3090, 0x00000001800d3090, 0x00000001804da1a0, 0x000000018045e2d0, 0x000000018045eec0, 0x000000018045efd0];

    pub fn new() -> Self {
        Self {
            vtable_rva: Self::VTABLE_RVA,
            field_0x0010: Default::default(),
            field_0x0018: Default::default(),
            field_0x001c: Default::default(),
            field_0x0020: Default::default(),
            field_0x0024: Default::default(),
            field_0x002c: Default::default(),
            field_0x0040: Default::default(),
            field_0x0050: Default::default(),
            field_0x0090: Default::default(),
            field_0x0094: Default::default(),
            field_0x0098: Default::default(),
            field_0x009c: Default::default(),
            field_0x00a0: Default::default(),
            field_0x00a4: Default::default(),
            field_0x00a8: Default::default(),
            field_0x00ac: Default::default(),
            field_0x00b0: Default::default(),
            field_0x00b8: Default::default(),
            field_0x00c4: Default::default(),
            field_0x0120: Default::default(),
            field_0x0124: Default::default(),
            field_0x0128: Default::default(),
            field_0x012c: Default::default(),
            field_0x0130: Default::default(),
            field_0x0134: Default::default(),
            field_0x0138: Default::default(),
            field_0x013c: Default::default(),
            field_0x0140: Default::default(),
            field_0x0144: Default::default(),
            field_0x0148: Default::default(),
            field_0x014c: Default::default(),
            field_0x0150: Default::default(),
            field_0x0154: Default::default(),
            field_0x0158: Default::default(),
            field_0x015c: Default::default(),
            field_0x0168: Default::default(),
        }
    }

    /// Decompiled virtual method #0 @ RVA 0x004c8e90 (16 instructions, 3 basic blocks)
    pub fn method_00(&mut self) {
    }

    /// Decompiled virtual method #1 @ RVA 0x004da070 (13 instructions, 3 basic blocks)
    pub fn method_01(&mut self) {
    }

    /// Decompiled virtual method #2 @ RVA 0x0045e4c0 (42 instructions, 4 basic blocks)
    pub fn method_02(&mut self) {
        // Writes to field at offset +0x0090
        // Writes to field at offset +0x0094
        // Writes to field at offset +0x0098
        // Writes to field at offset +0x009c
        // Writes to field at offset +0x00a0
        // Writes to field at offset +0x00a4
        // Writes to field at offset +0x00a8
        // Writes to field at offset +0x00ac
        // Writes to field at offset +0x0140
        // Writes to field at offset +0x0144
        // Writes to field at offset +0x0148
        // Writes to field at offset +0x014c
        // Writes to field at offset +0x0150
        // Writes to field at offset +0x0154
        // Writes to field at offset +0x0158
        // Writes to field at offset +0x015c
    }

    /// Decompiled virtual method #3 @ RVA 0x000d3090 (1 instructions, 1 basic blocks)
    pub fn method_03(&mut self) {
    }

    /// Decompiled virtual method #4 @ RVA 0x0019e550 (2 instructions, 1 basic blocks)
    pub fn method_04(&mut self) {
        // Writes to field at offset +0x00c4
    }

    /// Decompiled virtual method #5 @ RVA 0x004da260 (137 instructions, 37 basic blocks)
    pub fn method_05(&mut self) {
        // Writes to field at offset +0x0024
        // Writes to field at offset +0x0050
        // Writes to field at offset +0x0168
    }

    /// Decompiled virtual method #6 @ RVA 0x000d3090 (1 instructions, 1 basic blocks)
    pub fn method_06(&mut self) {
    }

    /// Decompiled virtual method #7 @ RVA 0x000d3090 (1 instructions, 1 basic blocks)
    pub fn method_07(&mut self) {
    }

    /// Decompiled virtual method #8 @ RVA 0x000d3090 (1 instructions, 1 basic blocks)
    pub fn method_08(&mut self) {
    }

    /// Decompiled virtual method #9 @ RVA 0x000d3090 (1 instructions, 1 basic blocks)
    pub fn method_09(&mut self) {
    }

    /// Decompiled virtual method #10 @ RVA 0x004da1a0 (33 instructions, 3 basic blocks)
    pub fn method_10(&mut self) {
        // Writes to field at offset +0x0090
        // Writes to field at offset +0x0094
        // Writes to field at offset +0x0098
        // Writes to field at offset +0x009c
        // Writes to field at offset +0x00a0
        // Writes to field at offset +0x00a4
        // Writes to field at offset +0x00a8
        // Writes to field at offset +0x00ac
    }

    /// Decompiled virtual method #11 @ RVA 0x0045e2d0 (33 instructions, 3 basic blocks)
    pub fn method_11(&mut self) {
        // Writes to field at offset +0x0090
        // Writes to field at offset +0x00a0
        // Writes to field at offset +0x00a4
        // Writes to field at offset +0x00a8
        // Writes to field at offset +0x00ac
    }

    /// Decompiled virtual method #12 @ RVA 0x0045eec0 (67 instructions, 22 basic blocks)
    pub fn method_12(&mut self) {
        // Writes to field at offset +0x001c
        // Writes to field at offset +0x0040
        // Writes to field at offset +0x0090
        // Writes to field at offset +0x00b0
    }

    /// Decompiled virtual method #13 @ RVA 0x0045efd0 (27 instructions, 5 basic blocks)
    pub fn method_13(&mut self) {
        // Writes to field at offset +0x0090
        // Writes to field at offset +0x00b0
    }

}

impl CcClass for ccGobjGimEvMachine {
    fn class_name(&self) -> &'static str { Self::CLASS_NAME }
    fn vtable_rva(&self) -> u32 { Self::VTABLE_RVA }
    fn virtual_method_count(&self) -> usize { Self::METHOD_COUNT }
    fn base_classes(&self) -> &'static [&'static str] { Self::BASE_CLASSES }
}

/// Decompiled CC2 class: `ccGobjGimEvent` (VTable RVA: 0x008be858, 14 virtual methods, 43 recovered fields)
/// Inherits: ccGobjGimBase, ccGameObj
#[derive(Debug, Clone, Default)]
pub struct ccGobjGimEvent {
    pub vtable_rva: u32,
    pub field_0x0000: u32,
    pub field_0x0004: u32,
    pub field_0x0008: u16,
    pub field_0x000a: u16,
    pub field_0x0010: u16,
    pub field_0x0018: u64,
    pub field_0x001c: u8,
    pub field_0x0020: u32,
    pub field_0x0024: u16,
    pub field_0x002c: u16,
    pub field_0x0040: u32,
    pub field_0x0050: u64,
    pub field_0x0058: u64,
    pub field_0x0090: u32,
    pub field_0x0094: u32,
    pub field_0x0098: u32,
    pub field_0x009c: u32,
    pub field_0x00a0: u32,
    pub field_0x00a4: u32,
    pub field_0x00a8: u32,
    pub field_0x00ac: u32,
    pub field_0x00b0: u64,
    pub field_0x00b8: u64,
    pub field_0x00c4: u32,
    pub field_0x00e8: f32,
    pub field_0x0120: u32,
    pub field_0x0124: u32,
    pub field_0x0128: u32,
    pub field_0x012c: u32,
    pub field_0x0130: u32,
    pub field_0x0134: u32,
    pub field_0x0138: u32,
    pub field_0x013c: u32,
    pub field_0x0140: u32,
    pub field_0x0144: u32,
    pub field_0x0148: u32,
    pub field_0x014c: u32,
    pub field_0x0150: u32,
    pub field_0x0154: u32,
    pub field_0x0158: u32,
    pub field_0x015c: u32,
    pub field_0x01c0: u32,
    pub field_0x01c4: u32,
}

impl ccGobjGimEvent {
    pub const CLASS_NAME: &'static str = "ccGobjGimEvent";
    pub const VTABLE_RVA: u32 = 0x008be858;
    pub const METHOD_COUNT: usize = 14;
    pub const BASE_CLASSES: &'static [&'static str] = &["ccGobjGimBase", "ccGameObj"];
    pub const VIRTUAL_METHODS: &'static [u64] = &[0x00000001804c8e90, 0x00000001804db620, 0x000000018045e4c0, 0x00000001800d3090, 0x000000018019e550, 0x00000001804dba60, 0x00000001800d3090, 0x00000001800d3090, 0x00000001800d3090, 0x00000001804db8c0, 0x00000001804dbbd0, 0x000000018045e2d0, 0x000000018045eec0, 0x000000018045efd0];

    pub fn new() -> Self {
        Self {
            vtable_rva: Self::VTABLE_RVA,
            field_0x0000: Default::default(),
            field_0x0004: Default::default(),
            field_0x0008: Default::default(),
            field_0x000a: Default::default(),
            field_0x0010: Default::default(),
            field_0x0018: Default::default(),
            field_0x001c: Default::default(),
            field_0x0020: Default::default(),
            field_0x0024: Default::default(),
            field_0x002c: Default::default(),
            field_0x0040: Default::default(),
            field_0x0050: Default::default(),
            field_0x0058: Default::default(),
            field_0x0090: Default::default(),
            field_0x0094: Default::default(),
            field_0x0098: Default::default(),
            field_0x009c: Default::default(),
            field_0x00a0: Default::default(),
            field_0x00a4: Default::default(),
            field_0x00a8: Default::default(),
            field_0x00ac: Default::default(),
            field_0x00b0: Default::default(),
            field_0x00b8: Default::default(),
            field_0x00c4: Default::default(),
            field_0x00e8: Default::default(),
            field_0x0120: Default::default(),
            field_0x0124: Default::default(),
            field_0x0128: Default::default(),
            field_0x012c: Default::default(),
            field_0x0130: Default::default(),
            field_0x0134: Default::default(),
            field_0x0138: Default::default(),
            field_0x013c: Default::default(),
            field_0x0140: Default::default(),
            field_0x0144: Default::default(),
            field_0x0148: Default::default(),
            field_0x014c: Default::default(),
            field_0x0150: Default::default(),
            field_0x0154: Default::default(),
            field_0x0158: Default::default(),
            field_0x015c: Default::default(),
            field_0x01c0: Default::default(),
            field_0x01c4: Default::default(),
        }
    }

    /// Decompiled virtual method #0 @ RVA 0x004c8e90 (16 instructions, 3 basic blocks)
    pub fn method_00(&mut self) {
    }

    /// Decompiled virtual method #1 @ RVA 0x004db620 (13 instructions, 3 basic blocks)
    pub fn method_01(&mut self) {
    }

    /// Decompiled virtual method #2 @ RVA 0x0045e4c0 (42 instructions, 4 basic blocks)
    pub fn method_02(&mut self) {
        // Writes to field at offset +0x0090
        // Writes to field at offset +0x0094
        // Writes to field at offset +0x0098
        // Writes to field at offset +0x009c
        // Writes to field at offset +0x00a0
        // Writes to field at offset +0x00a4
        // Writes to field at offset +0x00a8
        // Writes to field at offset +0x00ac
        // Writes to field at offset +0x0140
        // Writes to field at offset +0x0144
        // Writes to field at offset +0x0148
        // Writes to field at offset +0x014c
        // Writes to field at offset +0x0150
        // Writes to field at offset +0x0154
        // Writes to field at offset +0x0158
        // Writes to field at offset +0x015c
    }

    /// Decompiled virtual method #3 @ RVA 0x000d3090 (1 instructions, 1 basic blocks)
    pub fn method_03(&mut self) {
    }

    /// Decompiled virtual method #4 @ RVA 0x0019e550 (2 instructions, 1 basic blocks)
    pub fn method_04(&mut self) {
        // Writes to field at offset +0x00c4
    }

    /// Decompiled virtual method #5 @ RVA 0x004dba60 (122 instructions, 31 basic blocks)
    pub fn method_05(&mut self) {
        // Writes to field at offset +0x0000
        // Writes to field at offset +0x0004
        // Writes to field at offset +0x0008
        // Writes to field at offset +0x000a
        // Writes to field at offset +0x0024
        // Writes to field at offset +0x0050
        // Writes to field at offset +0x0090
        // Writes to field at offset +0x0094
        // Writes to field at offset +0x0098
        // Writes to field at offset +0x009c
    }

    /// Decompiled virtual method #6 @ RVA 0x000d3090 (1 instructions, 1 basic blocks)
    pub fn method_06(&mut self) {
    }

    /// Decompiled virtual method #7 @ RVA 0x000d3090 (1 instructions, 1 basic blocks)
    pub fn method_07(&mut self) {
    }

    /// Decompiled virtual method #8 @ RVA 0x000d3090 (1 instructions, 1 basic blocks)
    pub fn method_08(&mut self) {
    }

    /// Decompiled virtual method #9 @ RVA 0x004db8c0 (100 instructions, 24 basic blocks)
    pub fn method_09(&mut self) {
        // Writes to field at offset +0x0000
        // Writes to field at offset +0x0004
        // Writes to field at offset +0x0008
        // Writes to field at offset +0x000a
        // Writes to field at offset +0x001c
        // Writes to field at offset +0x002c
        // Writes to field at offset +0x00e8
    }

    /// Decompiled virtual method #10 @ RVA 0x004dbbd0 (40 instructions, 4 basic blocks)
    pub fn method_10(&mut self) {
        // Writes to field at offset +0x0050
        // Writes to field at offset +0x0090
        // Writes to field at offset +0x0094
        // Writes to field at offset +0x0098
        // Writes to field at offset +0x009c
        // Writes to field at offset +0x00a0
        // Writes to field at offset +0x00a4
        // Writes to field at offset +0x00a8
        // Writes to field at offset +0x00ac
    }

    /// Decompiled virtual method #11 @ RVA 0x0045e2d0 (33 instructions, 3 basic blocks)
    pub fn method_11(&mut self) {
        // Writes to field at offset +0x0090
        // Writes to field at offset +0x00a0
        // Writes to field at offset +0x00a4
        // Writes to field at offset +0x00a8
        // Writes to field at offset +0x00ac
    }

    /// Decompiled virtual method #12 @ RVA 0x0045eec0 (67 instructions, 22 basic blocks)
    pub fn method_12(&mut self) {
        // Writes to field at offset +0x001c
        // Writes to field at offset +0x0040
        // Writes to field at offset +0x0090
        // Writes to field at offset +0x00b0
    }

    /// Decompiled virtual method #13 @ RVA 0x0045efd0 (27 instructions, 5 basic blocks)
    pub fn method_13(&mut self) {
        // Writes to field at offset +0x0090
        // Writes to field at offset +0x00b0
    }

}

impl CcClass for ccGobjGimEvent {
    fn class_name(&self) -> &'static str { Self::CLASS_NAME }
    fn vtable_rva(&self) -> u32 { Self::VTABLE_RVA }
    fn virtual_method_count(&self) -> usize { Self::METHOD_COUNT }
    fn base_classes(&self) -> &'static [&'static str] { Self::BASE_CLASSES }
}

/// Decompiled CC2 class: `ccGobjGimItemBox` (VTable RVA: 0x008bf140, 14 virtual methods, 41 recovered fields)
/// Inherits: ccGobjGimBase, ccGameObj
#[derive(Debug, Clone, Default)]
pub struct ccGobjGimItemBox {
    pub vtable_rva: u32,
    pub field_0x0000: u64,
    pub field_0x0004: u32,
    pub field_0x0008: u16,
    pub field_0x000a: u16,
    pub field_0x0010: u64,
    pub field_0x0018: u64,
    pub field_0x001c: u8,
    pub field_0x0020: u32,
    pub field_0x0024: u16,
    pub field_0x002c: u16,
    pub field_0x0040: u32,
    pub field_0x0050: u64,
    pub field_0x0058: u64,
    pub field_0x0090: u32,
    pub field_0x0094: u32,
    pub field_0x0098: u32,
    pub field_0x009c: u32,
    pub field_0x00a0: u32,
    pub field_0x00a4: u32,
    pub field_0x00a8: u32,
    pub field_0x00ac: u32,
    pub field_0x00b0: u64,
    pub field_0x00b8: u64,
    pub field_0x00c4: u32,
    pub field_0x0120: u32,
    pub field_0x0124: u32,
    pub field_0x0128: u32,
    pub field_0x012c: u32,
    pub field_0x0130: u32,
    pub field_0x0134: u32,
    pub field_0x0138: u32,
    pub field_0x013c: u32,
    pub field_0x0140: u32,
    pub field_0x0144: u32,
    pub field_0x0148: u32,
    pub field_0x014c: u32,
    pub field_0x0150: u32,
    pub field_0x0154: u32,
    pub field_0x0158: u32,
    pub field_0x015c: u32,
    pub field_0x01c0: u32,
}

impl ccGobjGimItemBox {
    pub const CLASS_NAME: &'static str = "ccGobjGimItemBox";
    pub const VTABLE_RVA: u32 = 0x008bf140;
    pub const METHOD_COUNT: usize = 14;
    pub const BASE_CLASSES: &'static [&'static str] = &["ccGobjGimBase", "ccGameObj"];
    pub const VIRTUAL_METHODS: &'static [u64] = &[0x00000001804cc2b0, 0x00000001804cc310, 0x000000018045e4c0, 0x00000001800d3090, 0x000000018019e550, 0x00000001804ccae0, 0x00000001800d3090, 0x00000001800d3090, 0x00000001800d3090, 0x00000001804cc560, 0x00000001804ccd70, 0x000000018045e2d0, 0x000000018045eec0, 0x000000018045efd0];

    pub fn new() -> Self {
        Self {
            vtable_rva: Self::VTABLE_RVA,
            field_0x0000: Default::default(),
            field_0x0004: Default::default(),
            field_0x0008: Default::default(),
            field_0x000a: Default::default(),
            field_0x0010: Default::default(),
            field_0x0018: Default::default(),
            field_0x001c: Default::default(),
            field_0x0020: Default::default(),
            field_0x0024: Default::default(),
            field_0x002c: Default::default(),
            field_0x0040: Default::default(),
            field_0x0050: Default::default(),
            field_0x0058: Default::default(),
            field_0x0090: Default::default(),
            field_0x0094: Default::default(),
            field_0x0098: Default::default(),
            field_0x009c: Default::default(),
            field_0x00a0: Default::default(),
            field_0x00a4: Default::default(),
            field_0x00a8: Default::default(),
            field_0x00ac: Default::default(),
            field_0x00b0: Default::default(),
            field_0x00b8: Default::default(),
            field_0x00c4: Default::default(),
            field_0x0120: Default::default(),
            field_0x0124: Default::default(),
            field_0x0128: Default::default(),
            field_0x012c: Default::default(),
            field_0x0130: Default::default(),
            field_0x0134: Default::default(),
            field_0x0138: Default::default(),
            field_0x013c: Default::default(),
            field_0x0140: Default::default(),
            field_0x0144: Default::default(),
            field_0x0148: Default::default(),
            field_0x014c: Default::default(),
            field_0x0150: Default::default(),
            field_0x0154: Default::default(),
            field_0x0158: Default::default(),
            field_0x015c: Default::default(),
            field_0x01c0: Default::default(),
        }
    }

    /// Decompiled virtual method #0 @ RVA 0x004cc2b0 (26 instructions, 6 basic blocks)
    pub fn method_00(&mut self) {
        // Writes to field at offset +0x0000
    }

    /// Decompiled virtual method #1 @ RVA 0x004cc310 (104 instructions, 14 basic blocks)
    pub fn method_01(&mut self) {
        // Writes to field at offset +0x0000
        // Writes to field at offset +0x0004
        // Writes to field at offset +0x0008
        // Writes to field at offset +0x000a
        // Writes to field at offset +0x001c
    }

    /// Decompiled virtual method #2 @ RVA 0x0045e4c0 (42 instructions, 4 basic blocks)
    pub fn method_02(&mut self) {
        // Writes to field at offset +0x0090
        // Writes to field at offset +0x0094
        // Writes to field at offset +0x0098
        // Writes to field at offset +0x009c
        // Writes to field at offset +0x00a0
        // Writes to field at offset +0x00a4
        // Writes to field at offset +0x00a8
        // Writes to field at offset +0x00ac
        // Writes to field at offset +0x0140
        // Writes to field at offset +0x0144
        // Writes to field at offset +0x0148
        // Writes to field at offset +0x014c
        // Writes to field at offset +0x0150
        // Writes to field at offset +0x0154
        // Writes to field at offset +0x0158
        // Writes to field at offset +0x015c
    }

    /// Decompiled virtual method #3 @ RVA 0x000d3090 (1 instructions, 1 basic blocks)
    pub fn method_03(&mut self) {
    }

    /// Decompiled virtual method #4 @ RVA 0x0019e550 (2 instructions, 1 basic blocks)
    pub fn method_04(&mut self) {
        // Writes to field at offset +0x00c4
    }

    /// Decompiled virtual method #5 @ RVA 0x004ccae0 (135 instructions, 36 basic blocks)
    pub fn method_05(&mut self) {
        // Writes to field at offset +0x0000
        // Writes to field at offset +0x0000
        // Writes to field at offset +0x0004
        // Writes to field at offset +0x0008
        // Writes to field at offset +0x000a
        // Writes to field at offset +0x0040
    }

    /// Decompiled virtual method #6 @ RVA 0x000d3090 (1 instructions, 1 basic blocks)
    pub fn method_06(&mut self) {
    }

    /// Decompiled virtual method #7 @ RVA 0x000d3090 (1 instructions, 1 basic blocks)
    pub fn method_07(&mut self) {
    }

    /// Decompiled virtual method #8 @ RVA 0x000d3090 (1 instructions, 1 basic blocks)
    pub fn method_08(&mut self) {
    }

    /// Decompiled virtual method #9 @ RVA 0x004cc560 (35 instructions, 9 basic blocks)
    pub fn method_09(&mut self) {
    }

    /// Decompiled virtual method #10 @ RVA 0x004ccd70 (45 instructions, 5 basic blocks)
    pub fn method_10(&mut self) {
        // Writes to field at offset +0x0090
        // Writes to field at offset +0x0094
        // Writes to field at offset +0x0098
        // Writes to field at offset +0x009c
        // Writes to field at offset +0x00a0
        // Writes to field at offset +0x00a4
        // Writes to field at offset +0x00a8
        // Writes to field at offset +0x00ac
    }

    /// Decompiled virtual method #11 @ RVA 0x0045e2d0 (33 instructions, 3 basic blocks)
    pub fn method_11(&mut self) {
        // Writes to field at offset +0x0090
        // Writes to field at offset +0x00a0
        // Writes to field at offset +0x00a4
        // Writes to field at offset +0x00a8
        // Writes to field at offset +0x00ac
    }

    /// Decompiled virtual method #12 @ RVA 0x0045eec0 (67 instructions, 22 basic blocks)
    pub fn method_12(&mut self) {
        // Writes to field at offset +0x001c
        // Writes to field at offset +0x0040
        // Writes to field at offset +0x0090
        // Writes to field at offset +0x00b0
    }

    /// Decompiled virtual method #13 @ RVA 0x0045efd0 (27 instructions, 5 basic blocks)
    pub fn method_13(&mut self) {
        // Writes to field at offset +0x0090
        // Writes to field at offset +0x00b0
    }

}

impl CcClass for ccGobjGimItemBox {
    fn class_name(&self) -> &'static str { Self::CLASS_NAME }
    fn vtable_rva(&self) -> u32 { Self::VTABLE_RVA }
    fn virtual_method_count(&self) -> usize { Self::METHOD_COUNT }
    fn base_classes(&self) -> &'static [&'static str] { Self::BASE_CLASSES }
}

/// Decompiled CC2 class: `ccGobjGimItemRoomDoor` (VTable RVA: 0x008bf320, 14 virtual methods, 38 recovered fields)
/// Inherits: ccGobjGimBase, ccGameObj
#[derive(Debug, Clone, Default)]
pub struct ccGobjGimItemRoomDoor {
    pub vtable_rva: u32,
    pub field_0x0000: u32,
    pub field_0x0004: u32,
    pub field_0x0008: u16,
    pub field_0x000a: u16,
    pub field_0x0010: u64,
    pub field_0x0018: u64,
    pub field_0x001c: u8,
    pub field_0x0020: u32,
    pub field_0x0024: u16,
    pub field_0x002c: u16,
    pub field_0x0040: u32,
    pub field_0x0090: u32,
    pub field_0x0094: u32,
    pub field_0x0098: u32,
    pub field_0x009c: u32,
    pub field_0x00a0: u32,
    pub field_0x00a4: u32,
    pub field_0x00a8: u32,
    pub field_0x00ac: u32,
    pub field_0x00b0: u64,
    pub field_0x00b8: u64,
    pub field_0x00c4: u32,
    pub field_0x0120: u32,
    pub field_0x0124: u32,
    pub field_0x0128: u32,
    pub field_0x012c: u32,
    pub field_0x0130: u32,
    pub field_0x0134: u32,
    pub field_0x0138: u32,
    pub field_0x013c: u32,
    pub field_0x0140: u32,
    pub field_0x0144: u32,
    pub field_0x0148: u32,
    pub field_0x014c: u32,
    pub field_0x0150: u32,
    pub field_0x0154: u32,
    pub field_0x0158: u32,
    pub field_0x015c: u32,
}

impl ccGobjGimItemRoomDoor {
    pub const CLASS_NAME: &'static str = "ccGobjGimItemRoomDoor";
    pub const VTABLE_RVA: u32 = 0x008bf320;
    pub const METHOD_COUNT: usize = 14;
    pub const BASE_CLASSES: &'static [&'static str] = &["ccGobjGimBase", "ccGameObj"];
    pub const VIRTUAL_METHODS: &'static [u64] = &[0x00000001804ca230, 0x00000001804cb0b0, 0x000000018045e4c0, 0x00000001800d3090, 0x000000018019e550, 0x00000001804cb420, 0x00000001800d3090, 0x00000001800d3090, 0x00000001800d3090, 0x00000001804cb2e0, 0x00000001804caf40, 0x000000018045e2d0, 0x000000018045eec0, 0x000000018045efd0];

    pub fn new() -> Self {
        Self {
            vtable_rva: Self::VTABLE_RVA,
            field_0x0000: Default::default(),
            field_0x0004: Default::default(),
            field_0x0008: Default::default(),
            field_0x000a: Default::default(),
            field_0x0010: Default::default(),
            field_0x0018: Default::default(),
            field_0x001c: Default::default(),
            field_0x0020: Default::default(),
            field_0x0024: Default::default(),
            field_0x002c: Default::default(),
            field_0x0040: Default::default(),
            field_0x0090: Default::default(),
            field_0x0094: Default::default(),
            field_0x0098: Default::default(),
            field_0x009c: Default::default(),
            field_0x00a0: Default::default(),
            field_0x00a4: Default::default(),
            field_0x00a8: Default::default(),
            field_0x00ac: Default::default(),
            field_0x00b0: Default::default(),
            field_0x00b8: Default::default(),
            field_0x00c4: Default::default(),
            field_0x0120: Default::default(),
            field_0x0124: Default::default(),
            field_0x0128: Default::default(),
            field_0x012c: Default::default(),
            field_0x0130: Default::default(),
            field_0x0134: Default::default(),
            field_0x0138: Default::default(),
            field_0x013c: Default::default(),
            field_0x0140: Default::default(),
            field_0x0144: Default::default(),
            field_0x0148: Default::default(),
            field_0x014c: Default::default(),
            field_0x0150: Default::default(),
            field_0x0154: Default::default(),
            field_0x0158: Default::default(),
            field_0x015c: Default::default(),
        }
    }

    /// Decompiled virtual method #0 @ RVA 0x004ca230 (16 instructions, 3 basic blocks)
    pub fn method_00(&mut self) {
    }

    /// Decompiled virtual method #1 @ RVA 0x004cb0b0 (13 instructions, 3 basic blocks)
    pub fn method_01(&mut self) {
    }

    /// Decompiled virtual method #2 @ RVA 0x0045e4c0 (42 instructions, 4 basic blocks)
    pub fn method_02(&mut self) {
        // Writes to field at offset +0x0090
        // Writes to field at offset +0x0094
        // Writes to field at offset +0x0098
        // Writes to field at offset +0x009c
        // Writes to field at offset +0x00a0
        // Writes to field at offset +0x00a4
        // Writes to field at offset +0x00a8
        // Writes to field at offset +0x00ac
        // Writes to field at offset +0x0140
        // Writes to field at offset +0x0144
        // Writes to field at offset +0x0148
        // Writes to field at offset +0x014c
        // Writes to field at offset +0x0150
        // Writes to field at offset +0x0154
        // Writes to field at offset +0x0158
        // Writes to field at offset +0x015c
    }

    /// Decompiled virtual method #3 @ RVA 0x000d3090 (1 instructions, 1 basic blocks)
    pub fn method_03(&mut self) {
    }

    /// Decompiled virtual method #4 @ RVA 0x0019e550 (2 instructions, 1 basic blocks)
    pub fn method_04(&mut self) {
        // Writes to field at offset +0x00c4
    }

    /// Decompiled virtual method #5 @ RVA 0x004cb420 (109 instructions, 25 basic blocks)
    pub fn method_05(&mut self) {
        // Writes to field at offset +0x0024
    }

    /// Decompiled virtual method #6 @ RVA 0x000d3090 (1 instructions, 1 basic blocks)
    pub fn method_06(&mut self) {
    }

    /// Decompiled virtual method #7 @ RVA 0x000d3090 (1 instructions, 1 basic blocks)
    pub fn method_07(&mut self) {
    }

    /// Decompiled virtual method #8 @ RVA 0x000d3090 (1 instructions, 1 basic blocks)
    pub fn method_08(&mut self) {
    }

    /// Decompiled virtual method #9 @ RVA 0x004cb2e0 (42 instructions, 7 basic blocks)
    pub fn method_09(&mut self) {
        // Writes to field at offset +0x0000
        // Writes to field at offset +0x0004
        // Writes to field at offset +0x0008
        // Writes to field at offset +0x000a
    }

    /// Decompiled virtual method #10 @ RVA 0x004caf40 (38 instructions, 3 basic blocks)
    pub fn method_10(&mut self) {
        // Writes to field at offset +0x0090
        // Writes to field at offset +0x0094
        // Writes to field at offset +0x0098
        // Writes to field at offset +0x009c
        // Writes to field at offset +0x00a0
        // Writes to field at offset +0x00a4
        // Writes to field at offset +0x00a8
        // Writes to field at offset +0x00ac
    }

    /// Decompiled virtual method #11 @ RVA 0x0045e2d0 (33 instructions, 3 basic blocks)
    pub fn method_11(&mut self) {
        // Writes to field at offset +0x0090
        // Writes to field at offset +0x00a0
        // Writes to field at offset +0x00a4
        // Writes to field at offset +0x00a8
        // Writes to field at offset +0x00ac
    }

    /// Decompiled virtual method #12 @ RVA 0x0045eec0 (67 instructions, 22 basic blocks)
    pub fn method_12(&mut self) {
        // Writes to field at offset +0x001c
        // Writes to field at offset +0x0040
        // Writes to field at offset +0x0090
        // Writes to field at offset +0x00b0
    }

    /// Decompiled virtual method #13 @ RVA 0x0045efd0 (27 instructions, 5 basic blocks)
    pub fn method_13(&mut self) {
        // Writes to field at offset +0x0090
        // Writes to field at offset +0x00b0
    }

}

impl CcClass for ccGobjGimItemRoomDoor {
    fn class_name(&self) -> &'static str { Self::CLASS_NAME }
    fn vtable_rva(&self) -> u32 { Self::VTABLE_RVA }
    fn virtual_method_count(&self) -> usize { Self::METHOD_COUNT }
    fn base_classes(&self) -> &'static [&'static str] { Self::BASE_CLASSES }
}

/// Decompiled CC2 class: `ccGobjGimItemRoomDoor2` (VTable RVA: 0x008bf398, 14 virtual methods, 48 recovered fields)
/// Inherits: ccGobjGimBase, ccGameObj
#[derive(Debug, Clone, Default)]
pub struct ccGobjGimItemRoomDoor2 {
    pub vtable_rva: u32,
    pub field_0x0000: u32,
    pub field_0x0004: u32,
    pub field_0x0008: u16,
    pub field_0x0009: u8,
    pub field_0x000a: u16,
    pub field_0x0010: u64,
    pub field_0x0018: u64,
    pub field_0x001c: u8,
    pub field_0x0020: u32,
    pub field_0x0024: u16,
    pub field_0x002c: u16,
    pub field_0x0040: u32,
    pub field_0x0058: u64,
    pub field_0x0060: u32,
    pub field_0x0070: u32,
    pub field_0x0080: u32,
    pub field_0x0088: u32,
    pub field_0x0090: u32,
    pub field_0x0094: u32,
    pub field_0x0095: u8,
    pub field_0x0098: u32,
    pub field_0x009c: u32,
    pub field_0x00a0: u32,
    pub field_0x00a4: u32,
    pub field_0x00a8: u32,
    pub field_0x00ac: u32,
    pub field_0x00b0: u64,
    pub field_0x00b8: u64,
    pub field_0x00c4: u32,
    pub field_0x00e0: u32,
    pub field_0x00e1: u8,
    pub field_0x00f0: u32,
    pub field_0x0120: u32,
    pub field_0x0124: u32,
    pub field_0x0128: u32,
    pub field_0x012c: u32,
    pub field_0x0130: u32,
    pub field_0x0134: u32,
    pub field_0x0138: u32,
    pub field_0x013c: u32,
    pub field_0x0140: u32,
    pub field_0x0144: u32,
    pub field_0x0148: u32,
    pub field_0x014c: u32,
    pub field_0x0150: u32,
    pub field_0x0154: u32,
    pub field_0x0158: u32,
    pub field_0x015c: u32,
}

impl ccGobjGimItemRoomDoor2 {
    pub const CLASS_NAME: &'static str = "ccGobjGimItemRoomDoor2";
    pub const VTABLE_RVA: u32 = 0x008bf398;
    pub const METHOD_COUNT: usize = 14;
    pub const BASE_CLASSES: &'static [&'static str] = &["ccGobjGimBase", "ccGameObj"];
    pub const VIRTUAL_METHODS: &'static [u64] = &[0x00000001804ca230, 0x00000001804cb5a0, 0x000000018045e4c0, 0x00000001800d3090, 0x000000018019e550, 0x00000001804cbab0, 0x00000001800d3090, 0x00000001800d3090, 0x00000001800d3090, 0x00000001804cb770, 0x00000001804ca030, 0x000000018045e2d0, 0x000000018045eec0, 0x000000018045efd0];

    pub fn new() -> Self {
        Self {
            vtable_rva: Self::VTABLE_RVA,
            field_0x0000: Default::default(),
            field_0x0004: Default::default(),
            field_0x0008: Default::default(),
            field_0x0009: Default::default(),
            field_0x000a: Default::default(),
            field_0x0010: Default::default(),
            field_0x0018: Default::default(),
            field_0x001c: Default::default(),
            field_0x0020: Default::default(),
            field_0x0024: Default::default(),
            field_0x002c: Default::default(),
            field_0x0040: Default::default(),
            field_0x0058: Default::default(),
            field_0x0060: Default::default(),
            field_0x0070: Default::default(),
            field_0x0080: Default::default(),
            field_0x0088: Default::default(),
            field_0x0090: Default::default(),
            field_0x0094: Default::default(),
            field_0x0095: Default::default(),
            field_0x0098: Default::default(),
            field_0x009c: Default::default(),
            field_0x00a0: Default::default(),
            field_0x00a4: Default::default(),
            field_0x00a8: Default::default(),
            field_0x00ac: Default::default(),
            field_0x00b0: Default::default(),
            field_0x00b8: Default::default(),
            field_0x00c4: Default::default(),
            field_0x00e0: Default::default(),
            field_0x00e1: Default::default(),
            field_0x00f0: Default::default(),
            field_0x0120: Default::default(),
            field_0x0124: Default::default(),
            field_0x0128: Default::default(),
            field_0x012c: Default::default(),
            field_0x0130: Default::default(),
            field_0x0134: Default::default(),
            field_0x0138: Default::default(),
            field_0x013c: Default::default(),
            field_0x0140: Default::default(),
            field_0x0144: Default::default(),
            field_0x0148: Default::default(),
            field_0x014c: Default::default(),
            field_0x0150: Default::default(),
            field_0x0154: Default::default(),
            field_0x0158: Default::default(),
            field_0x015c: Default::default(),
        }
    }

    /// Decompiled virtual method #0 @ RVA 0x004ca230 (16 instructions, 3 basic blocks)
    pub fn method_00(&mut self) {
    }

    /// Decompiled virtual method #1 @ RVA 0x004cb5a0 (13 instructions, 3 basic blocks)
    pub fn method_01(&mut self) {
    }

    /// Decompiled virtual method #2 @ RVA 0x0045e4c0 (42 instructions, 4 basic blocks)
    pub fn method_02(&mut self) {
        // Writes to field at offset +0x0090
        // Writes to field at offset +0x0094
        // Writes to field at offset +0x0098
        // Writes to field at offset +0x009c
        // Writes to field at offset +0x00a0
        // Writes to field at offset +0x00a4
        // Writes to field at offset +0x00a8
        // Writes to field at offset +0x00ac
        // Writes to field at offset +0x0140
        // Writes to field at offset +0x0144
        // Writes to field at offset +0x0148
        // Writes to field at offset +0x014c
        // Writes to field at offset +0x0150
        // Writes to field at offset +0x0154
        // Writes to field at offset +0x0158
        // Writes to field at offset +0x015c
    }

    /// Decompiled virtual method #3 @ RVA 0x000d3090 (1 instructions, 1 basic blocks)
    pub fn method_03(&mut self) {
    }

    /// Decompiled virtual method #4 @ RVA 0x0019e550 (2 instructions, 1 basic blocks)
    pub fn method_04(&mut self) {
        // Writes to field at offset +0x00c4
    }

    /// Decompiled virtual method #5 @ RVA 0x004cbab0 (127 instructions, 31 basic blocks)
    pub fn method_05(&mut self) {
        // Writes to field at offset +0x0000
        // Writes to field at offset +0x0000
        // Writes to field at offset +0x0004
        // Writes to field at offset +0x0008
        // Writes to field at offset +0x0009
        // Writes to field at offset +0x000a
        // Writes to field at offset +0x0040
    }

    /// Decompiled virtual method #6 @ RVA 0x000d3090 (1 instructions, 1 basic blocks)
    pub fn method_06(&mut self) {
    }

    /// Decompiled virtual method #7 @ RVA 0x000d3090 (1 instructions, 1 basic blocks)
    pub fn method_07(&mut self) {
    }

    /// Decompiled virtual method #8 @ RVA 0x000d3090 (1 instructions, 1 basic blocks)
    pub fn method_08(&mut self) {
    }

    /// Decompiled virtual method #9 @ RVA 0x004cb770 (94 instructions, 9 basic blocks)
    pub fn method_09(&mut self) {
        // Writes to field at offset +0x0010
        // Writes to field at offset +0x0018
        // Writes to field at offset +0x0095
        // Writes to field at offset +0x00e1
    }

    /// Decompiled virtual method #10 @ RVA 0x004ca030 (38 instructions, 3 basic blocks)
    pub fn method_10(&mut self) {
        // Writes to field at offset +0x0090
        // Writes to field at offset +0x0094
        // Writes to field at offset +0x0098
        // Writes to field at offset +0x009c
        // Writes to field at offset +0x00a0
        // Writes to field at offset +0x00a4
        // Writes to field at offset +0x00a8
        // Writes to field at offset +0x00ac
    }

    /// Decompiled virtual method #11 @ RVA 0x0045e2d0 (33 instructions, 3 basic blocks)
    pub fn method_11(&mut self) {
        // Writes to field at offset +0x0090
        // Writes to field at offset +0x00a0
        // Writes to field at offset +0x00a4
        // Writes to field at offset +0x00a8
        // Writes to field at offset +0x00ac
    }

    /// Decompiled virtual method #12 @ RVA 0x0045eec0 (67 instructions, 22 basic blocks)
    pub fn method_12(&mut self) {
        // Writes to field at offset +0x001c
        // Writes to field at offset +0x0040
        // Writes to field at offset +0x0090
        // Writes to field at offset +0x00b0
    }

    /// Decompiled virtual method #13 @ RVA 0x0045efd0 (27 instructions, 5 basic blocks)
    pub fn method_13(&mut self) {
        // Writes to field at offset +0x0090
        // Writes to field at offset +0x00b0
    }

}

impl CcClass for ccGobjGimItemRoomDoor2 {
    fn class_name(&self) -> &'static str { Self::CLASS_NAME }
    fn vtable_rva(&self) -> u32 { Self::VTABLE_RVA }
    fn virtual_method_count(&self) -> usize { Self::METHOD_COUNT }
    fn base_classes(&self) -> &'static [&'static str] { Self::BASE_CLASSES }
}

/// Decompiled CC2 class: `ccGobjGimItemStatue` (VTable RVA: 0x008bf0c8, 14 virtual methods, 20 recovered fields)
/// Inherits: ccGobjGimBase, ccGameObj
#[derive(Debug, Clone, Default)]
pub struct ccGobjGimItemStatue {
    pub vtable_rva: u32,
    pub field_0x0000: u32,
    pub field_0x0004: u32,
    pub field_0x0008: u16,
    pub field_0x000a: u16,
    pub field_0x0010: u8,
    pub field_0x0018: u64,
    pub field_0x001c: u8,
    pub field_0x0020: u32,
    pub field_0x0024: u16,
    pub field_0x002c: u16,
    pub field_0x0040: u32,
    pub field_0x0090: u64,
    pub field_0x0098: u64,
    pub field_0x00a0: u32,
    pub field_0x00a4: u32,
    pub field_0x00a8: u32,
    pub field_0x00ac: u32,
    pub field_0x00b0: u64,
    pub field_0x00b8: u64,
    pub field_0x00c4: u32,
}

impl ccGobjGimItemStatue {
    pub const CLASS_NAME: &'static str = "ccGobjGimItemStatue";
    pub const VTABLE_RVA: u32 = 0x008bf0c8;
    pub const METHOD_COUNT: usize = 14;
    pub const BASE_CLASSES: &'static [&'static str] = &["ccGobjGimBase", "ccGameObj"];
    pub const VIRTUAL_METHODS: &'static [u64] = &[0x00000001804cbe10, 0x00000001804cbe50, 0x00000001804cbf70, 0x00000001800d3090, 0x000000018019e550, 0x00000001804cc0e0, 0x00000001804cbf30, 0x00000001800d3090, 0x00000001800d3090, 0x00000001804cbfc0, 0x000000018045e220, 0x000000018045e2d0, 0x000000018045eec0, 0x000000018045efd0];

    pub fn new() -> Self {
        Self {
            vtable_rva: Self::VTABLE_RVA,
            field_0x0000: Default::default(),
            field_0x0004: Default::default(),
            field_0x0008: Default::default(),
            field_0x000a: Default::default(),
            field_0x0010: Default::default(),
            field_0x0018: Default::default(),
            field_0x001c: Default::default(),
            field_0x0020: Default::default(),
            field_0x0024: Default::default(),
            field_0x002c: Default::default(),
            field_0x0040: Default::default(),
            field_0x0090: Default::default(),
            field_0x0098: Default::default(),
            field_0x00a0: Default::default(),
            field_0x00a4: Default::default(),
            field_0x00a8: Default::default(),
            field_0x00ac: Default::default(),
            field_0x00b0: Default::default(),
            field_0x00b8: Default::default(),
            field_0x00c4: Default::default(),
        }
    }

    /// Decompiled virtual method #0 @ RVA 0x004cbe10 (16 instructions, 3 basic blocks)
    pub fn method_00(&mut self) {
    }

    /// Decompiled virtual method #1 @ RVA 0x004cbe50 (11 instructions, 3 basic blocks)
    pub fn method_01(&mut self) {
    }

    /// Decompiled virtual method #2 @ RVA 0x004cbf70 (17 instructions, 3 basic blocks)
    pub fn method_02(&mut self) {
        // Writes to field at offset +0x00c4
    }

    /// Decompiled virtual method #3 @ RVA 0x000d3090 (1 instructions, 1 basic blocks)
    pub fn method_03(&mut self) {
    }

    /// Decompiled virtual method #4 @ RVA 0x0019e550 (2 instructions, 1 basic blocks)
    pub fn method_04(&mut self) {
        // Writes to field at offset +0x00c4
    }

    /// Decompiled virtual method #5 @ RVA 0x004cc0e0 (138 instructions, 28 basic blocks)
    pub fn method_05(&mut self) {
        // Writes to field at offset +0x0024
    }

    /// Decompiled virtual method #6 @ RVA 0x004cbf30 (11 instructions, 1 basic blocks)
    pub fn method_06(&mut self) {
    }

    /// Decompiled virtual method #7 @ RVA 0x000d3090 (1 instructions, 1 basic blocks)
    pub fn method_07(&mut self) {
    }

    /// Decompiled virtual method #8 @ RVA 0x000d3090 (1 instructions, 1 basic blocks)
    pub fn method_08(&mut self) {
    }

    /// Decompiled virtual method #9 @ RVA 0x004cbfc0 (77 instructions, 17 basic blocks)
    pub fn method_09(&mut self) {
        // Writes to field at offset +0x0000
        // Writes to field at offset +0x0004
        // Writes to field at offset +0x0008
        // Writes to field at offset +0x000a
        // Writes to field at offset +0x0010
    }

    /// Decompiled virtual method #10 @ RVA 0x0045e220 (31 instructions, 3 basic blocks)
    pub fn method_10(&mut self) {
        // Writes to field at offset +0x0090
        // Writes to field at offset +0x00a0
        // Writes to field at offset +0x00a4
        // Writes to field at offset +0x00a8
        // Writes to field at offset +0x00ac
    }

    /// Decompiled virtual method #11 @ RVA 0x0045e2d0 (33 instructions, 3 basic blocks)
    pub fn method_11(&mut self) {
        // Writes to field at offset +0x0090
        // Writes to field at offset +0x00a0
        // Writes to field at offset +0x00a4
        // Writes to field at offset +0x00a8
        // Writes to field at offset +0x00ac
    }

    /// Decompiled virtual method #12 @ RVA 0x0045eec0 (67 instructions, 22 basic blocks)
    pub fn method_12(&mut self) {
        // Writes to field at offset +0x001c
        // Writes to field at offset +0x0040
        // Writes to field at offset +0x0090
        // Writes to field at offset +0x00b0
    }

    /// Decompiled virtual method #13 @ RVA 0x0045efd0 (27 instructions, 5 basic blocks)
    pub fn method_13(&mut self) {
        // Writes to field at offset +0x0090
        // Writes to field at offset +0x00b0
    }

}

impl CcClass for ccGobjGimItemStatue {
    fn class_name(&self) -> &'static str { Self::CLASS_NAME }
    fn vtable_rva(&self) -> u32 { Self::VTABLE_RVA }
    fn virtual_method_count(&self) -> usize { Self::METHOD_COUNT }
    fn base_classes(&self) -> &'static [&'static str] { Self::BASE_CLASSES }
}

/// Decompiled CC2 class: `ccGobjGimItemStatue02` (VTable RVA: 0x008bed80, 14 virtual methods, 20 recovered fields)
/// Inherits: ccGobjGimBase, ccGameObj
#[derive(Debug, Clone, Default)]
pub struct ccGobjGimItemStatue02 {
    pub vtable_rva: u32,
    pub field_0x0000: u32,
    pub field_0x0004: u32,
    pub field_0x0008: u16,
    pub field_0x000a: u16,
    pub field_0x0010: u8,
    pub field_0x0018: u64,
    pub field_0x001c: u8,
    pub field_0x0020: u32,
    pub field_0x0024: u16,
    pub field_0x002c: u16,
    pub field_0x0040: u32,
    pub field_0x0090: u64,
    pub field_0x0098: u64,
    pub field_0x00a0: u32,
    pub field_0x00a4: u32,
    pub field_0x00a8: u32,
    pub field_0x00ac: u32,
    pub field_0x00b0: u64,
    pub field_0x00b8: u64,
    pub field_0x00c4: u32,
}

impl ccGobjGimItemStatue02 {
    pub const CLASS_NAME: &'static str = "ccGobjGimItemStatue02";
    pub const VTABLE_RVA: u32 = 0x008bed80;
    pub const METHOD_COUNT: usize = 14;
    pub const BASE_CLASSES: &'static [&'static str] = &["ccGobjGimBase", "ccGameObj"];
    pub const VIRTUAL_METHODS: &'static [u64] = &[0x00000001804cbe10, 0x00000001804cbe50, 0x00000001804cbf70, 0x00000001800d3090, 0x000000018019e550, 0x00000001804cc0e0, 0x00000001804cbf30, 0x00000001800d3090, 0x00000001800d3090, 0x00000001804cbfc0, 0x000000018045e220, 0x000000018045e2d0, 0x000000018045eec0, 0x000000018045efd0];

    pub fn new() -> Self {
        Self {
            vtable_rva: Self::VTABLE_RVA,
            field_0x0000: Default::default(),
            field_0x0004: Default::default(),
            field_0x0008: Default::default(),
            field_0x000a: Default::default(),
            field_0x0010: Default::default(),
            field_0x0018: Default::default(),
            field_0x001c: Default::default(),
            field_0x0020: Default::default(),
            field_0x0024: Default::default(),
            field_0x002c: Default::default(),
            field_0x0040: Default::default(),
            field_0x0090: Default::default(),
            field_0x0098: Default::default(),
            field_0x00a0: Default::default(),
            field_0x00a4: Default::default(),
            field_0x00a8: Default::default(),
            field_0x00ac: Default::default(),
            field_0x00b0: Default::default(),
            field_0x00b8: Default::default(),
            field_0x00c4: Default::default(),
        }
    }

    /// Decompiled virtual method #0 @ RVA 0x004cbe10 (16 instructions, 3 basic blocks)
    pub fn method_00(&mut self) {
    }

    /// Decompiled virtual method #1 @ RVA 0x004cbe50 (11 instructions, 3 basic blocks)
    pub fn method_01(&mut self) {
    }

    /// Decompiled virtual method #2 @ RVA 0x004cbf70 (17 instructions, 3 basic blocks)
    pub fn method_02(&mut self) {
        // Writes to field at offset +0x00c4
    }

    /// Decompiled virtual method #3 @ RVA 0x000d3090 (1 instructions, 1 basic blocks)
    pub fn method_03(&mut self) {
    }

    /// Decompiled virtual method #4 @ RVA 0x0019e550 (2 instructions, 1 basic blocks)
    pub fn method_04(&mut self) {
        // Writes to field at offset +0x00c4
    }

    /// Decompiled virtual method #5 @ RVA 0x004cc0e0 (138 instructions, 28 basic blocks)
    pub fn method_05(&mut self) {
        // Writes to field at offset +0x0024
    }

    /// Decompiled virtual method #6 @ RVA 0x004cbf30 (11 instructions, 1 basic blocks)
    pub fn method_06(&mut self) {
    }

    /// Decompiled virtual method #7 @ RVA 0x000d3090 (1 instructions, 1 basic blocks)
    pub fn method_07(&mut self) {
    }

    /// Decompiled virtual method #8 @ RVA 0x000d3090 (1 instructions, 1 basic blocks)
    pub fn method_08(&mut self) {
    }

    /// Decompiled virtual method #9 @ RVA 0x004cbfc0 (77 instructions, 17 basic blocks)
    pub fn method_09(&mut self) {
        // Writes to field at offset +0x0000
        // Writes to field at offset +0x0004
        // Writes to field at offset +0x0008
        // Writes to field at offset +0x000a
        // Writes to field at offset +0x0010
    }

    /// Decompiled virtual method #10 @ RVA 0x0045e220 (31 instructions, 3 basic blocks)
    pub fn method_10(&mut self) {
        // Writes to field at offset +0x0090
        // Writes to field at offset +0x00a0
        // Writes to field at offset +0x00a4
        // Writes to field at offset +0x00a8
        // Writes to field at offset +0x00ac
    }

    /// Decompiled virtual method #11 @ RVA 0x0045e2d0 (33 instructions, 3 basic blocks)
    pub fn method_11(&mut self) {
        // Writes to field at offset +0x0090
        // Writes to field at offset +0x00a0
        // Writes to field at offset +0x00a4
        // Writes to field at offset +0x00a8
        // Writes to field at offset +0x00ac
    }

    /// Decompiled virtual method #12 @ RVA 0x0045eec0 (67 instructions, 22 basic blocks)
    pub fn method_12(&mut self) {
        // Writes to field at offset +0x001c
        // Writes to field at offset +0x0040
        // Writes to field at offset +0x0090
        // Writes to field at offset +0x00b0
    }

    /// Decompiled virtual method #13 @ RVA 0x0045efd0 (27 instructions, 5 basic blocks)
    pub fn method_13(&mut self) {
        // Writes to field at offset +0x0090
        // Writes to field at offset +0x00b0
    }

}

impl CcClass for ccGobjGimItemStatue02 {
    fn class_name(&self) -> &'static str { Self::CLASS_NAME }
    fn vtable_rva(&self) -> u32 { Self::VTABLE_RVA }
    fn virtual_method_count(&self) -> usize { Self::METHOD_COUNT }
    fn base_classes(&self) -> &'static [&'static str] { Self::BASE_CLASSES }
}

/// Decompiled CC2 class: `ccGobjGimMechGranty` (VTable RVA: 0x008bee70, 14 virtual methods, 44 recovered fields)
/// Inherits: ccGobjGimBase, ccGameObj
#[derive(Debug, Clone, Default)]
pub struct ccGobjGimMechGranty {
    pub vtable_rva: u32,
    pub field_0x0000: u32,
    pub field_0x0004: u32,
    pub field_0x0008: u16,
    pub field_0x0009: u8,
    pub field_0x000a: u16,
    pub field_0x000c: u32,
    pub field_0x0010: u64,
    pub field_0x0018: u64,
    pub field_0x001c: u8,
    pub field_0x0020: u32,
    pub field_0x0024: u16,
    pub field_0x002c: u16,
    pub field_0x0040: u32,
    pub field_0x0058: u64,
    pub field_0x0090: u32,
    pub field_0x0094: u32,
    pub field_0x0098: u32,
    pub field_0x009c: u32,
    pub field_0x00a0: u32,
    pub field_0x00a4: u32,
    pub field_0x00a8: u32,
    pub field_0x00ac: u32,
    pub field_0x00b0: u64,
    pub field_0x00b8: u64,
    pub field_0x00c4: u32,
    pub field_0x00cc: u32,
    pub field_0x0120: u32,
    pub field_0x0124: u32,
    pub field_0x0128: u32,
    pub field_0x012c: u32,
    pub field_0x0130: u32,
    pub field_0x0134: u32,
    pub field_0x0138: u32,
    pub field_0x013c: u32,
    pub field_0x0140: u32,
    pub field_0x0144: u32,
    pub field_0x0148: u32,
    pub field_0x014c: u32,
    pub field_0x0150: u32,
    pub field_0x0154: u32,
    pub field_0x0158: u32,
    pub field_0x015c: u32,
    pub field_0x0168: u64,
    pub field_0x01e0: u64,
}

impl ccGobjGimMechGranty {
    pub const CLASS_NAME: &'static str = "ccGobjGimMechGranty";
    pub const VTABLE_RVA: u32 = 0x008bee70;
    pub const METHOD_COUNT: usize = 14;
    pub const BASE_CLASSES: &'static [&'static str] = &["ccGobjGimBase", "ccGameObj"];
    pub const VIRTUAL_METHODS: &'static [u64] = &[0x00000001804d0550, 0x00000001804d0640, 0x000000018045e4c0, 0x00000001800d3090, 0x000000018019e550, 0x00000001804d0fc0, 0x00000001804d0e60, 0x00000001800d3090, 0x00000001800d3090, 0x00000001800d3090, 0x00000001804d0a70, 0x000000018045e2d0, 0x000000018045eec0, 0x000000018045efd0];

    pub fn new() -> Self {
        Self {
            vtable_rva: Self::VTABLE_RVA,
            field_0x0000: Default::default(),
            field_0x0004: Default::default(),
            field_0x0008: Default::default(),
            field_0x0009: Default::default(),
            field_0x000a: Default::default(),
            field_0x000c: Default::default(),
            field_0x0010: Default::default(),
            field_0x0018: Default::default(),
            field_0x001c: Default::default(),
            field_0x0020: Default::default(),
            field_0x0024: Default::default(),
            field_0x002c: Default::default(),
            field_0x0040: Default::default(),
            field_0x0058: Default::default(),
            field_0x0090: Default::default(),
            field_0x0094: Default::default(),
            field_0x0098: Default::default(),
            field_0x009c: Default::default(),
            field_0x00a0: Default::default(),
            field_0x00a4: Default::default(),
            field_0x00a8: Default::default(),
            field_0x00ac: Default::default(),
            field_0x00b0: Default::default(),
            field_0x00b8: Default::default(),
            field_0x00c4: Default::default(),
            field_0x00cc: Default::default(),
            field_0x0120: Default::default(),
            field_0x0124: Default::default(),
            field_0x0128: Default::default(),
            field_0x012c: Default::default(),
            field_0x0130: Default::default(),
            field_0x0134: Default::default(),
            field_0x0138: Default::default(),
            field_0x013c: Default::default(),
            field_0x0140: Default::default(),
            field_0x0144: Default::default(),
            field_0x0148: Default::default(),
            field_0x014c: Default::default(),
            field_0x0150: Default::default(),
            field_0x0154: Default::default(),
            field_0x0158: Default::default(),
            field_0x015c: Default::default(),
            field_0x0168: Default::default(),
            field_0x01e0: Default::default(),
        }
    }

    /// Decompiled virtual method #0 @ RVA 0x004d0550 (16 instructions, 3 basic blocks)
    pub fn method_00(&mut self) {
    }

    /// Decompiled virtual method #1 @ RVA 0x004d0640 (130 instructions, 30 basic blocks)
    pub fn method_01(&mut self) {
        // Writes to field at offset +0x0000
        // Writes to field at offset +0x0000
        // Writes to field at offset +0x0004
        // Writes to field at offset +0x0008
        // Writes to field at offset +0x000a
        // Writes to field at offset +0x0040
    }

    /// Decompiled virtual method #2 @ RVA 0x0045e4c0 (42 instructions, 4 basic blocks)
    pub fn method_02(&mut self) {
        // Writes to field at offset +0x0090
        // Writes to field at offset +0x0094
        // Writes to field at offset +0x0098
        // Writes to field at offset +0x009c
        // Writes to field at offset +0x00a0
        // Writes to field at offset +0x00a4
        // Writes to field at offset +0x00a8
        // Writes to field at offset +0x00ac
        // Writes to field at offset +0x0140
        // Writes to field at offset +0x0144
        // Writes to field at offset +0x0148
        // Writes to field at offset +0x014c
        // Writes to field at offset +0x0150
        // Writes to field at offset +0x0154
        // Writes to field at offset +0x0158
        // Writes to field at offset +0x015c
    }

    /// Decompiled virtual method #3 @ RVA 0x000d3090 (1 instructions, 1 basic blocks)
    pub fn method_03(&mut self) {
    }

    /// Decompiled virtual method #4 @ RVA 0x0019e550 (2 instructions, 1 basic blocks)
    pub fn method_04(&mut self) {
        // Writes to field at offset +0x00c4
    }

    /// Decompiled virtual method #5 @ RVA 0x004d0fc0 (123 instructions, 40 basic blocks)
    pub fn method_05(&mut self) {
        // Writes to field at offset +0x0000
        // Writes to field at offset +0x0009
        // Writes to field at offset +0x0024
        // Writes to field at offset +0x00cc
    }

    /// Decompiled virtual method #6 @ RVA 0x004d0e60 (128 instructions, 26 basic blocks)
    pub fn method_06(&mut self) {
        // Writes to field at offset +0x0000
        // Writes to field at offset +0x0004
        // Writes to field at offset +0x0008
        // Writes to field at offset +0x000c
        // Writes to field at offset +0x0168
    }

    /// Decompiled virtual method #7 @ RVA 0x000d3090 (1 instructions, 1 basic blocks)
    pub fn method_07(&mut self) {
    }

    /// Decompiled virtual method #8 @ RVA 0x000d3090 (1 instructions, 1 basic blocks)
    pub fn method_08(&mut self) {
    }

    /// Decompiled virtual method #9 @ RVA 0x000d3090 (1 instructions, 1 basic blocks)
    pub fn method_09(&mut self) {
    }

    /// Decompiled virtual method #10 @ RVA 0x004d0a70 (89 instructions, 5 basic blocks)
    pub fn method_10(&mut self) {
        // Writes to field at offset +0x0090
        // Writes to field at offset +0x0094
        // Writes to field at offset +0x0098
        // Writes to field at offset +0x009c
        // Writes to field at offset +0x00a0
        // Writes to field at offset +0x00a4
        // Writes to field at offset +0x00a8
        // Writes to field at offset +0x00ac
    }

    /// Decompiled virtual method #11 @ RVA 0x0045e2d0 (33 instructions, 3 basic blocks)
    pub fn method_11(&mut self) {
        // Writes to field at offset +0x0090
        // Writes to field at offset +0x00a0
        // Writes to field at offset +0x00a4
        // Writes to field at offset +0x00a8
        // Writes to field at offset +0x00ac
    }

    /// Decompiled virtual method #12 @ RVA 0x0045eec0 (67 instructions, 22 basic blocks)
    pub fn method_12(&mut self) {
        // Writes to field at offset +0x001c
        // Writes to field at offset +0x0040
        // Writes to field at offset +0x0090
        // Writes to field at offset +0x00b0
    }

    /// Decompiled virtual method #13 @ RVA 0x0045efd0 (27 instructions, 5 basic blocks)
    pub fn method_13(&mut self) {
        // Writes to field at offset +0x0090
        // Writes to field at offset +0x00b0
    }

}

impl CcClass for ccGobjGimMechGranty {
    fn class_name(&self) -> &'static str { Self::CLASS_NAME }
    fn vtable_rva(&self) -> u32 { Self::VTABLE_RVA }
    fn virtual_method_count(&self) -> usize { Self::METHOD_COUNT }
    fn base_classes(&self) -> &'static [&'static str] { Self::BASE_CLASSES }
}

/// Decompiled CC2 class: `ccGobjGimPlatform` (VTable RVA: 0x008bf2a8, 14 virtual methods, 34 recovered fields)
/// Inherits: ccGobjGimBase, ccGameObj
#[derive(Debug, Clone, Default)]
pub struct ccGobjGimPlatform {
    pub vtable_rva: u32,
    pub field_0x0010: u64,
    pub field_0x0018: u64,
    pub field_0x001c: u8,
    pub field_0x0020: u32,
    pub field_0x0024: u16,
    pub field_0x002c: u16,
    pub field_0x0040: u32,
    pub field_0x0090: u32,
    pub field_0x0094: u32,
    pub field_0x0098: u32,
    pub field_0x009c: u32,
    pub field_0x00a0: u32,
    pub field_0x00a4: u32,
    pub field_0x00a8: u32,
    pub field_0x00ac: u32,
    pub field_0x00b0: u64,
    pub field_0x00b8: u64,
    pub field_0x00c4: u32,
    pub field_0x0120: u32,
    pub field_0x0124: u32,
    pub field_0x0128: u32,
    pub field_0x012c: u32,
    pub field_0x0130: u32,
    pub field_0x0134: u32,
    pub field_0x0138: u32,
    pub field_0x013c: u32,
    pub field_0x0140: u32,
    pub field_0x0144: u32,
    pub field_0x0148: u32,
    pub field_0x014c: u32,
    pub field_0x0150: u32,
    pub field_0x0154: u32,
    pub field_0x0158: u32,
    pub field_0x015c: u32,
}

impl ccGobjGimPlatform {
    pub const CLASS_NAME: &'static str = "ccGobjGimPlatform";
    pub const VTABLE_RVA: u32 = 0x008bf2a8;
    pub const METHOD_COUNT: usize = 14;
    pub const BASE_CLASSES: &'static [&'static str] = &["ccGobjGimBase", "ccGameObj"];
    pub const VIRTUAL_METHODS: &'static [u64] = &[0x00000001804c8e90, 0x00000001804cab90, 0x000000018045e4c0, 0x00000001800d3090, 0x000000018019e550, 0x00000001804cae30, 0x00000001800d3090, 0x00000001800d3090, 0x00000001800d3090, 0x00000001800d3090, 0x00000001804caf40, 0x000000018045e2d0, 0x000000018045eec0, 0x000000018045efd0];

    pub fn new() -> Self {
        Self {
            vtable_rva: Self::VTABLE_RVA,
            field_0x0010: Default::default(),
            field_0x0018: Default::default(),
            field_0x001c: Default::default(),
            field_0x0020: Default::default(),
            field_0x0024: Default::default(),
            field_0x002c: Default::default(),
            field_0x0040: Default::default(),
            field_0x0090: Default::default(),
            field_0x0094: Default::default(),
            field_0x0098: Default::default(),
            field_0x009c: Default::default(),
            field_0x00a0: Default::default(),
            field_0x00a4: Default::default(),
            field_0x00a8: Default::default(),
            field_0x00ac: Default::default(),
            field_0x00b0: Default::default(),
            field_0x00b8: Default::default(),
            field_0x00c4: Default::default(),
            field_0x0120: Default::default(),
            field_0x0124: Default::default(),
            field_0x0128: Default::default(),
            field_0x012c: Default::default(),
            field_0x0130: Default::default(),
            field_0x0134: Default::default(),
            field_0x0138: Default::default(),
            field_0x013c: Default::default(),
            field_0x0140: Default::default(),
            field_0x0144: Default::default(),
            field_0x0148: Default::default(),
            field_0x014c: Default::default(),
            field_0x0150: Default::default(),
            field_0x0154: Default::default(),
            field_0x0158: Default::default(),
            field_0x015c: Default::default(),
        }
    }

    /// Decompiled virtual method #0 @ RVA 0x004c8e90 (16 instructions, 3 basic blocks)
    pub fn method_00(&mut self) {
    }

    /// Decompiled virtual method #1 @ RVA 0x004cab90 (13 instructions, 3 basic blocks)
    pub fn method_01(&mut self) {
    }

    /// Decompiled virtual method #2 @ RVA 0x0045e4c0 (42 instructions, 4 basic blocks)
    pub fn method_02(&mut self) {
        // Writes to field at offset +0x0090
        // Writes to field at offset +0x0094
        // Writes to field at offset +0x0098
        // Writes to field at offset +0x009c
        // Writes to field at offset +0x00a0
        // Writes to field at offset +0x00a4
        // Writes to field at offset +0x00a8
        // Writes to field at offset +0x00ac
        // Writes to field at offset +0x0140
        // Writes to field at offset +0x0144
        // Writes to field at offset +0x0148
        // Writes to field at offset +0x014c
        // Writes to field at offset +0x0150
        // Writes to field at offset +0x0154
        // Writes to field at offset +0x0158
        // Writes to field at offset +0x015c
    }

    /// Decompiled virtual method #3 @ RVA 0x000d3090 (1 instructions, 1 basic blocks)
    pub fn method_03(&mut self) {
    }

    /// Decompiled virtual method #4 @ RVA 0x0019e550 (2 instructions, 1 basic blocks)
    pub fn method_04(&mut self) {
        // Writes to field at offset +0x00c4
    }

    /// Decompiled virtual method #5 @ RVA 0x004cae30 (120 instructions, 23 basic blocks)
    pub fn method_05(&mut self) {
        // Writes to field at offset +0x0024
        // Writes to field at offset +0x0090
        // Writes to field at offset +0x0094
        // Writes to field at offset +0x0098
        // Writes to field at offset +0x009c
        // Writes to field at offset +0x00a0
        // Writes to field at offset +0x00a4
        // Writes to field at offset +0x00a8
        // Writes to field at offset +0x00ac
    }

    /// Decompiled virtual method #6 @ RVA 0x000d3090 (1 instructions, 1 basic blocks)
    pub fn method_06(&mut self) {
    }

    /// Decompiled virtual method #7 @ RVA 0x000d3090 (1 instructions, 1 basic blocks)
    pub fn method_07(&mut self) {
    }

    /// Decompiled virtual method #8 @ RVA 0x000d3090 (1 instructions, 1 basic blocks)
    pub fn method_08(&mut self) {
    }

    /// Decompiled virtual method #9 @ RVA 0x000d3090 (1 instructions, 1 basic blocks)
    pub fn method_09(&mut self) {
    }

    /// Decompiled virtual method #10 @ RVA 0x004caf40 (38 instructions, 3 basic blocks)
    pub fn method_10(&mut self) {
        // Writes to field at offset +0x0090
        // Writes to field at offset +0x0094
        // Writes to field at offset +0x0098
        // Writes to field at offset +0x009c
        // Writes to field at offset +0x00a0
        // Writes to field at offset +0x00a4
        // Writes to field at offset +0x00a8
        // Writes to field at offset +0x00ac
    }

    /// Decompiled virtual method #11 @ RVA 0x0045e2d0 (33 instructions, 3 basic blocks)
    pub fn method_11(&mut self) {
        // Writes to field at offset +0x0090
        // Writes to field at offset +0x00a0
        // Writes to field at offset +0x00a4
        // Writes to field at offset +0x00a8
        // Writes to field at offset +0x00ac
    }

    /// Decompiled virtual method #12 @ RVA 0x0045eec0 (67 instructions, 22 basic blocks)
    pub fn method_12(&mut self) {
        // Writes to field at offset +0x001c
        // Writes to field at offset +0x0040
        // Writes to field at offset +0x0090
        // Writes to field at offset +0x00b0
    }

    /// Decompiled virtual method #13 @ RVA 0x0045efd0 (27 instructions, 5 basic blocks)
    pub fn method_13(&mut self) {
        // Writes to field at offset +0x0090
        // Writes to field at offset +0x00b0
    }

}

impl CcClass for ccGobjGimPlatform {
    fn class_name(&self) -> &'static str { Self::CLASS_NAME }
    fn vtable_rva(&self) -> u32 { Self::VTABLE_RVA }
    fn virtual_method_count(&self) -> usize { Self::METHOD_COUNT }
    fn base_classes(&self) -> &'static [&'static str] { Self::BASE_CLASSES }
}

/// Decompiled CC2 class: `ccGobjGimPlatformD` (VTable RVA: 0x008befd8, 14 virtual methods, 34 recovered fields)
/// Inherits: ccGobjGimBase, ccGameObj
#[derive(Debug, Clone, Default)]
pub struct ccGobjGimPlatformD {
    pub vtable_rva: u32,
    pub field_0x0010: u64,
    pub field_0x0018: u64,
    pub field_0x001c: u8,
    pub field_0x0020: u32,
    pub field_0x0024: u16,
    pub field_0x002c: u16,
    pub field_0x0040: u32,
    pub field_0x0090: u32,
    pub field_0x0094: u32,
    pub field_0x0098: u32,
    pub field_0x009c: u32,
    pub field_0x00a0: u32,
    pub field_0x00a4: u32,
    pub field_0x00a8: u32,
    pub field_0x00ac: u32,
    pub field_0x00b0: u64,
    pub field_0x00b8: u64,
    pub field_0x00c4: u32,
    pub field_0x0120: u32,
    pub field_0x0124: u32,
    pub field_0x0128: u32,
    pub field_0x012c: u32,
    pub field_0x0130: u32,
    pub field_0x0134: u32,
    pub field_0x0138: u32,
    pub field_0x013c: u32,
    pub field_0x0140: u32,
    pub field_0x0144: u32,
    pub field_0x0148: u32,
    pub field_0x014c: u32,
    pub field_0x0150: u32,
    pub field_0x0154: u32,
    pub field_0x0158: u32,
    pub field_0x015c: u32,
}

impl ccGobjGimPlatformD {
    pub const CLASS_NAME: &'static str = "ccGobjGimPlatformD";
    pub const VTABLE_RVA: u32 = 0x008befd8;
    pub const METHOD_COUNT: usize = 14;
    pub const BASE_CLASSES: &'static [&'static str] = &["ccGobjGimBase", "ccGameObj"];
    pub const VIRTUAL_METHODS: &'static [u64] = &[0x00000001804ca230, 0x00000001804cedb0, 0x000000018045e4c0, 0x00000001800d3090, 0x000000018019e550, 0x00000001804cae30, 0x00000001800d3090, 0x00000001800d3090, 0x00000001800d3090, 0x00000001800d3090, 0x00000001804caf40, 0x000000018045e2d0, 0x000000018045eec0, 0x000000018045efd0];

    pub fn new() -> Self {
        Self {
            vtable_rva: Self::VTABLE_RVA,
            field_0x0010: Default::default(),
            field_0x0018: Default::default(),
            field_0x001c: Default::default(),
            field_0x0020: Default::default(),
            field_0x0024: Default::default(),
            field_0x002c: Default::default(),
            field_0x0040: Default::default(),
            field_0x0090: Default::default(),
            field_0x0094: Default::default(),
            field_0x0098: Default::default(),
            field_0x009c: Default::default(),
            field_0x00a0: Default::default(),
            field_0x00a4: Default::default(),
            field_0x00a8: Default::default(),
            field_0x00ac: Default::default(),
            field_0x00b0: Default::default(),
            field_0x00b8: Default::default(),
            field_0x00c4: Default::default(),
            field_0x0120: Default::default(),
            field_0x0124: Default::default(),
            field_0x0128: Default::default(),
            field_0x012c: Default::default(),
            field_0x0130: Default::default(),
            field_0x0134: Default::default(),
            field_0x0138: Default::default(),
            field_0x013c: Default::default(),
            field_0x0140: Default::default(),
            field_0x0144: Default::default(),
            field_0x0148: Default::default(),
            field_0x014c: Default::default(),
            field_0x0150: Default::default(),
            field_0x0154: Default::default(),
            field_0x0158: Default::default(),
            field_0x015c: Default::default(),
        }
    }

    /// Decompiled virtual method #0 @ RVA 0x004ca230 (16 instructions, 3 basic blocks)
    pub fn method_00(&mut self) {
    }

    /// Decompiled virtual method #1 @ RVA 0x004cedb0 (13 instructions, 3 basic blocks)
    pub fn method_01(&mut self) {
    }

    /// Decompiled virtual method #2 @ RVA 0x0045e4c0 (42 instructions, 4 basic blocks)
    pub fn method_02(&mut self) {
        // Writes to field at offset +0x0090
        // Writes to field at offset +0x0094
        // Writes to field at offset +0x0098
        // Writes to field at offset +0x009c
        // Writes to field at offset +0x00a0
        // Writes to field at offset +0x00a4
        // Writes to field at offset +0x00a8
        // Writes to field at offset +0x00ac
        // Writes to field at offset +0x0140
        // Writes to field at offset +0x0144
        // Writes to field at offset +0x0148
        // Writes to field at offset +0x014c
        // Writes to field at offset +0x0150
        // Writes to field at offset +0x0154
        // Writes to field at offset +0x0158
        // Writes to field at offset +0x015c
    }

    /// Decompiled virtual method #3 @ RVA 0x000d3090 (1 instructions, 1 basic blocks)
    pub fn method_03(&mut self) {
    }

    /// Decompiled virtual method #4 @ RVA 0x0019e550 (2 instructions, 1 basic blocks)
    pub fn method_04(&mut self) {
        // Writes to field at offset +0x00c4
    }

    /// Decompiled virtual method #5 @ RVA 0x004cae30 (120 instructions, 23 basic blocks)
    pub fn method_05(&mut self) {
        // Writes to field at offset +0x0024
        // Writes to field at offset +0x0090
        // Writes to field at offset +0x0094
        // Writes to field at offset +0x0098
        // Writes to field at offset +0x009c
        // Writes to field at offset +0x00a0
        // Writes to field at offset +0x00a4
        // Writes to field at offset +0x00a8
        // Writes to field at offset +0x00ac
    }

    /// Decompiled virtual method #6 @ RVA 0x000d3090 (1 instructions, 1 basic blocks)
    pub fn method_06(&mut self) {
    }

    /// Decompiled virtual method #7 @ RVA 0x000d3090 (1 instructions, 1 basic blocks)
    pub fn method_07(&mut self) {
    }

    /// Decompiled virtual method #8 @ RVA 0x000d3090 (1 instructions, 1 basic blocks)
    pub fn method_08(&mut self) {
    }

    /// Decompiled virtual method #9 @ RVA 0x000d3090 (1 instructions, 1 basic blocks)
    pub fn method_09(&mut self) {
    }

    /// Decompiled virtual method #10 @ RVA 0x004caf40 (38 instructions, 3 basic blocks)
    pub fn method_10(&mut self) {
        // Writes to field at offset +0x0090
        // Writes to field at offset +0x0094
        // Writes to field at offset +0x0098
        // Writes to field at offset +0x009c
        // Writes to field at offset +0x00a0
        // Writes to field at offset +0x00a4
        // Writes to field at offset +0x00a8
        // Writes to field at offset +0x00ac
    }

    /// Decompiled virtual method #11 @ RVA 0x0045e2d0 (33 instructions, 3 basic blocks)
    pub fn method_11(&mut self) {
        // Writes to field at offset +0x0090
        // Writes to field at offset +0x00a0
        // Writes to field at offset +0x00a4
        // Writes to field at offset +0x00a8
        // Writes to field at offset +0x00ac
    }

    /// Decompiled virtual method #12 @ RVA 0x0045eec0 (67 instructions, 22 basic blocks)
    pub fn method_12(&mut self) {
        // Writes to field at offset +0x001c
        // Writes to field at offset +0x0040
        // Writes to field at offset +0x0090
        // Writes to field at offset +0x00b0
    }

    /// Decompiled virtual method #13 @ RVA 0x0045efd0 (27 instructions, 5 basic blocks)
    pub fn method_13(&mut self) {
        // Writes to field at offset +0x0090
        // Writes to field at offset +0x00b0
    }

}

impl CcClass for ccGobjGimPlatformD {
    fn class_name(&self) -> &'static str { Self::CLASS_NAME }
    fn vtable_rva(&self) -> u32 { Self::VTABLE_RVA }
    fn virtual_method_count(&self) -> usize { Self::METHOD_COUNT }
    fn base_classes(&self) -> &'static [&'static str] { Self::BASE_CLASSES }
}

/// Decompiled CC2 class: `ccGobjGimRock` (VTable RVA: 0x008bf230, 14 virtual methods, 38 recovered fields)
/// Inherits: ccGobjGimBase, ccGameObj
#[derive(Debug, Clone, Default)]
pub struct ccGobjGimRock {
    pub vtable_rva: u32,
    pub field_0x0000: u32,
    pub field_0x0004: u32,
    pub field_0x0008: u16,
    pub field_0x000a: u16,
    pub field_0x0010: u64,
    pub field_0x0018: u64,
    pub field_0x001c: u8,
    pub field_0x0020: u32,
    pub field_0x0024: u16,
    pub field_0x002c: u16,
    pub field_0x0040: u32,
    pub field_0x0090: u32,
    pub field_0x0094: u32,
    pub field_0x0098: u32,
    pub field_0x009c: u32,
    pub field_0x00a0: u32,
    pub field_0x00a4: u32,
    pub field_0x00a8: u32,
    pub field_0x00ac: u32,
    pub field_0x00b0: u64,
    pub field_0x00b8: u64,
    pub field_0x00c4: u32,
    pub field_0x0120: u32,
    pub field_0x0124: u32,
    pub field_0x0128: u32,
    pub field_0x012c: u32,
    pub field_0x0130: u32,
    pub field_0x0134: u32,
    pub field_0x0138: u32,
    pub field_0x013c: u32,
    pub field_0x0140: u32,
    pub field_0x0144: u32,
    pub field_0x0148: u32,
    pub field_0x014c: u32,
    pub field_0x0150: u32,
    pub field_0x0154: u32,
    pub field_0x0158: u32,
    pub field_0x015c: u32,
}

impl ccGobjGimRock {
    pub const CLASS_NAME: &'static str = "ccGobjGimRock";
    pub const VTABLE_RVA: u32 = 0x008bf230;
    pub const METHOD_COUNT: usize = 14;
    pub const BASE_CLASSES: &'static [&'static str] = &["ccGobjGimBase", "ccGameObj"];
    pub const VIRTUAL_METHODS: &'static [u64] = &[0x00000001804ca230, 0x00000001804ca270, 0x000000018045e4c0, 0x00000001800d3090, 0x000000018019e550, 0x00000001804ca7d0, 0x00000001800d3090, 0x00000001800d3090, 0x00000001800d3090, 0x00000001804ca460, 0x00000001804ca030, 0x000000018045e2d0, 0x000000018045eec0, 0x000000018045efd0];

    pub fn new() -> Self {
        Self {
            vtable_rva: Self::VTABLE_RVA,
            field_0x0000: Default::default(),
            field_0x0004: Default::default(),
            field_0x0008: Default::default(),
            field_0x000a: Default::default(),
            field_0x0010: Default::default(),
            field_0x0018: Default::default(),
            field_0x001c: Default::default(),
            field_0x0020: Default::default(),
            field_0x0024: Default::default(),
            field_0x002c: Default::default(),
            field_0x0040: Default::default(),
            field_0x0090: Default::default(),
            field_0x0094: Default::default(),
            field_0x0098: Default::default(),
            field_0x009c: Default::default(),
            field_0x00a0: Default::default(),
            field_0x00a4: Default::default(),
            field_0x00a8: Default::default(),
            field_0x00ac: Default::default(),
            field_0x00b0: Default::default(),
            field_0x00b8: Default::default(),
            field_0x00c4: Default::default(),
            field_0x0120: Default::default(),
            field_0x0124: Default::default(),
            field_0x0128: Default::default(),
            field_0x012c: Default::default(),
            field_0x0130: Default::default(),
            field_0x0134: Default::default(),
            field_0x0138: Default::default(),
            field_0x013c: Default::default(),
            field_0x0140: Default::default(),
            field_0x0144: Default::default(),
            field_0x0148: Default::default(),
            field_0x014c: Default::default(),
            field_0x0150: Default::default(),
            field_0x0154: Default::default(),
            field_0x0158: Default::default(),
            field_0x015c: Default::default(),
        }
    }

    /// Decompiled virtual method #0 @ RVA 0x004ca230 (16 instructions, 3 basic blocks)
    pub fn method_00(&mut self) {
    }

    /// Decompiled virtual method #1 @ RVA 0x004ca270 (13 instructions, 3 basic blocks)
    pub fn method_01(&mut self) {
    }

    /// Decompiled virtual method #2 @ RVA 0x0045e4c0 (42 instructions, 4 basic blocks)
    pub fn method_02(&mut self) {
        // Writes to field at offset +0x0090
        // Writes to field at offset +0x0094
        // Writes to field at offset +0x0098
        // Writes to field at offset +0x009c
        // Writes to field at offset +0x00a0
        // Writes to field at offset +0x00a4
        // Writes to field at offset +0x00a8
        // Writes to field at offset +0x00ac
        // Writes to field at offset +0x0140
        // Writes to field at offset +0x0144
        // Writes to field at offset +0x0148
        // Writes to field at offset +0x014c
        // Writes to field at offset +0x0150
        // Writes to field at offset +0x0154
        // Writes to field at offset +0x0158
        // Writes to field at offset +0x015c
    }

    /// Decompiled virtual method #3 @ RVA 0x000d3090 (1 instructions, 1 basic blocks)
    pub fn method_03(&mut self) {
    }

    /// Decompiled virtual method #4 @ RVA 0x0019e550 (2 instructions, 1 basic blocks)
    pub fn method_04(&mut self) {
        // Writes to field at offset +0x00c4
    }

    /// Decompiled virtual method #5 @ RVA 0x004ca7d0 (133 instructions, 35 basic blocks)
    pub fn method_05(&mut self) {
        // Writes to field at offset +0x0000
        // Writes to field at offset +0x0000
        // Writes to field at offset +0x0004
        // Writes to field at offset +0x0008
        // Writes to field at offset +0x000a
        // Writes to field at offset +0x0040
    }

    /// Decompiled virtual method #6 @ RVA 0x000d3090 (1 instructions, 1 basic blocks)
    pub fn method_06(&mut self) {
    }

    /// Decompiled virtual method #7 @ RVA 0x000d3090 (1 instructions, 1 basic blocks)
    pub fn method_07(&mut self) {
    }

    /// Decompiled virtual method #8 @ RVA 0x000d3090 (1 instructions, 1 basic blocks)
    pub fn method_08(&mut self) {
    }

    /// Decompiled virtual method #9 @ RVA 0x004ca460 (25 instructions, 6 basic blocks)
    pub fn method_09(&mut self) {
    }

    /// Decompiled virtual method #10 @ RVA 0x004ca030 (38 instructions, 3 basic blocks)
    pub fn method_10(&mut self) {
        // Writes to field at offset +0x0090
        // Writes to field at offset +0x0094
        // Writes to field at offset +0x0098
        // Writes to field at offset +0x009c
        // Writes to field at offset +0x00a0
        // Writes to field at offset +0x00a4
        // Writes to field at offset +0x00a8
        // Writes to field at offset +0x00ac
    }

    /// Decompiled virtual method #11 @ RVA 0x0045e2d0 (33 instructions, 3 basic blocks)
    pub fn method_11(&mut self) {
        // Writes to field at offset +0x0090
        // Writes to field at offset +0x00a0
        // Writes to field at offset +0x00a4
        // Writes to field at offset +0x00a8
        // Writes to field at offset +0x00ac
    }

    /// Decompiled virtual method #12 @ RVA 0x0045eec0 (67 instructions, 22 basic blocks)
    pub fn method_12(&mut self) {
        // Writes to field at offset +0x001c
        // Writes to field at offset +0x0040
        // Writes to field at offset +0x0090
        // Writes to field at offset +0x00b0
    }

    /// Decompiled virtual method #13 @ RVA 0x0045efd0 (27 instructions, 5 basic blocks)
    pub fn method_13(&mut self) {
        // Writes to field at offset +0x0090
        // Writes to field at offset +0x00b0
    }

}

impl CcClass for ccGobjGimRock {
    fn class_name(&self) -> &'static str { Self::CLASS_NAME }
    fn vtable_rva(&self) -> u32 { Self::VTABLE_RVA }
    fn virtual_method_count(&self) -> usize { Self::METHOD_COUNT }
    fn base_classes(&self) -> &'static [&'static str] { Self::BASE_CLASSES }
}

/// Decompiled CC2 class: `ccGobjGimSteamTrap` (VTable RVA: 0x008bec90, 14 virtual methods, 36 recovered fields)
/// Inherits: ccGobjGimBaseChangeCCS, ccGobjGimBase, ccGameObj
#[derive(Debug, Clone, Default)]
pub struct ccGobjGimSteamTrap {
    pub vtable_rva: u32,
    pub field_0x0000: u64,
    pub field_0x0010: u64,
    pub field_0x0018: u64,
    pub field_0x001c: u8,
    pub field_0x0020: u32,
    pub field_0x0024: u16,
    pub field_0x002c: u16,
    pub field_0x0040: u32,
    pub field_0x0050: u64,
    pub field_0x0090: u32,
    pub field_0x0094: u32,
    pub field_0x0098: u32,
    pub field_0x009c: u32,
    pub field_0x00a0: u32,
    pub field_0x00a4: u32,
    pub field_0x00a8: u32,
    pub field_0x00ac: u32,
    pub field_0x00b0: u64,
    pub field_0x00b8: u64,
    pub field_0x00c4: u32,
    pub field_0x0120: u32,
    pub field_0x0124: u32,
    pub field_0x0128: u32,
    pub field_0x012c: u32,
    pub field_0x0130: u32,
    pub field_0x0134: u32,
    pub field_0x0138: u32,
    pub field_0x013c: u32,
    pub field_0x0140: u32,
    pub field_0x0144: u32,
    pub field_0x0148: u32,
    pub field_0x014c: u32,
    pub field_0x0150: u32,
    pub field_0x0154: u32,
    pub field_0x0158: u32,
    pub field_0x015c: u32,
}

impl ccGobjGimSteamTrap {
    pub const CLASS_NAME: &'static str = "ccGobjGimSteamTrap";
    pub const VTABLE_RVA: u32 = 0x008bec90;
    pub const METHOD_COUNT: usize = 14;
    pub const BASE_CLASSES: &'static [&'static str] = &["ccGobjGimBaseChangeCCS", "ccGobjGimBase", "ccGameObj"];
    pub const VIRTUAL_METHODS: &'static [u64] = &[0x00000001804d5720, 0x00000001804d5840, 0x000000018045e4c0, 0x00000001800d3090, 0x000000018019e550, 0x00000001804d6380, 0x00000001800d3090, 0x00000001800d3090, 0x00000001800d3090, 0x00000001804d61d0, 0x00000001804d5960, 0x000000018045e2d0, 0x000000018045eec0, 0x000000018045efd0];

    pub fn new() -> Self {
        Self {
            vtable_rva: Self::VTABLE_RVA,
            field_0x0000: Default::default(),
            field_0x0010: Default::default(),
            field_0x0018: Default::default(),
            field_0x001c: Default::default(),
            field_0x0020: Default::default(),
            field_0x0024: Default::default(),
            field_0x002c: Default::default(),
            field_0x0040: Default::default(),
            field_0x0050: Default::default(),
            field_0x0090: Default::default(),
            field_0x0094: Default::default(),
            field_0x0098: Default::default(),
            field_0x009c: Default::default(),
            field_0x00a0: Default::default(),
            field_0x00a4: Default::default(),
            field_0x00a8: Default::default(),
            field_0x00ac: Default::default(),
            field_0x00b0: Default::default(),
            field_0x00b8: Default::default(),
            field_0x00c4: Default::default(),
            field_0x0120: Default::default(),
            field_0x0124: Default::default(),
            field_0x0128: Default::default(),
            field_0x012c: Default::default(),
            field_0x0130: Default::default(),
            field_0x0134: Default::default(),
            field_0x0138: Default::default(),
            field_0x013c: Default::default(),
            field_0x0140: Default::default(),
            field_0x0144: Default::default(),
            field_0x0148: Default::default(),
            field_0x014c: Default::default(),
            field_0x0150: Default::default(),
            field_0x0154: Default::default(),
            field_0x0158: Default::default(),
            field_0x015c: Default::default(),
        }
    }

    /// Decompiled virtual method #0 @ RVA 0x004d5720 (16 instructions, 3 basic blocks)
    pub fn method_00(&mut self) {
    }

    /// Decompiled virtual method #1 @ RVA 0x004d5840 (11 instructions, 3 basic blocks)
    pub fn method_01(&mut self) {
    }

    /// Decompiled virtual method #2 @ RVA 0x0045e4c0 (42 instructions, 4 basic blocks)
    pub fn method_02(&mut self) {
        // Writes to field at offset +0x0090
        // Writes to field at offset +0x0094
        // Writes to field at offset +0x0098
        // Writes to field at offset +0x009c
        // Writes to field at offset +0x00a0
        // Writes to field at offset +0x00a4
        // Writes to field at offset +0x00a8
        // Writes to field at offset +0x00ac
        // Writes to field at offset +0x0140
        // Writes to field at offset +0x0144
        // Writes to field at offset +0x0148
        // Writes to field at offset +0x014c
        // Writes to field at offset +0x0150
        // Writes to field at offset +0x0154
        // Writes to field at offset +0x0158
        // Writes to field at offset +0x015c
    }

    /// Decompiled virtual method #3 @ RVA 0x000d3090 (1 instructions, 1 basic blocks)
    pub fn method_03(&mut self) {
    }

    /// Decompiled virtual method #4 @ RVA 0x0019e550 (2 instructions, 1 basic blocks)
    pub fn method_04(&mut self) {
        // Writes to field at offset +0x00c4
    }

    /// Decompiled virtual method #5 @ RVA 0x004d6380 (85 instructions, 25 basic blocks)
    pub fn method_05(&mut self) {
        // Writes to field at offset +0x0000
        // Writes to field at offset +0x0024
    }

    /// Decompiled virtual method #6 @ RVA 0x000d3090 (1 instructions, 1 basic blocks)
    pub fn method_06(&mut self) {
    }

    /// Decompiled virtual method #7 @ RVA 0x000d3090 (1 instructions, 1 basic blocks)
    pub fn method_07(&mut self) {
    }

    /// Decompiled virtual method #8 @ RVA 0x000d3090 (1 instructions, 1 basic blocks)
    pub fn method_08(&mut self) {
    }

    /// Decompiled virtual method #9 @ RVA 0x004d61d0 (64 instructions, 19 basic blocks)
    pub fn method_09(&mut self) {
    }

    /// Decompiled virtual method #10 @ RVA 0x004d5960 (91 instructions, 8 basic blocks)
    pub fn method_10(&mut self) {
        // Writes to field at offset +0x0090
        // Writes to field at offset +0x0094
        // Writes to field at offset +0x0098
        // Writes to field at offset +0x009c
        // Writes to field at offset +0x00a0
        // Writes to field at offset +0x00a4
        // Writes to field at offset +0x00a8
        // Writes to field at offset +0x00ac
    }

    /// Decompiled virtual method #11 @ RVA 0x0045e2d0 (33 instructions, 3 basic blocks)
    pub fn method_11(&mut self) {
        // Writes to field at offset +0x0090
        // Writes to field at offset +0x00a0
        // Writes to field at offset +0x00a4
        // Writes to field at offset +0x00a8
        // Writes to field at offset +0x00ac
    }

    /// Decompiled virtual method #12 @ RVA 0x0045eec0 (67 instructions, 22 basic blocks)
    pub fn method_12(&mut self) {
        // Writes to field at offset +0x001c
        // Writes to field at offset +0x0040
        // Writes to field at offset +0x0090
        // Writes to field at offset +0x00b0
    }

    /// Decompiled virtual method #13 @ RVA 0x0045efd0 (27 instructions, 5 basic blocks)
    pub fn method_13(&mut self) {
        // Writes to field at offset +0x0090
        // Writes to field at offset +0x00b0
    }

}

impl CcClass for ccGobjGimSteamTrap {
    fn class_name(&self) -> &'static str { Self::CLASS_NAME }
    fn vtable_rva(&self) -> u32 { Self::VTABLE_RVA }
    fn virtual_method_count(&self) -> usize { Self::METHOD_COUNT }
    fn base_classes(&self) -> &'static [&'static str] { Self::BASE_CLASSES }
}

/// Decompiled CC2 class: `ccGobjGimTimBarrel` (VTable RVA: 0x008beee8, 14 virtual methods, 39 recovered fields)
/// Inherits: ccGobjGimBase, ccGameObj
#[derive(Debug, Clone, Default)]
pub struct ccGobjGimTimBarrel {
    pub vtable_rva: u32,
    pub field_0x0000: u32,
    pub field_0x0004: u32,
    pub field_0x0010: u64,
    pub field_0x0018: u64,
    pub field_0x001c: u8,
    pub field_0x0020: u32,
    pub field_0x0024: u16,
    pub field_0x002c: u16,
    pub field_0x0040: u32,
    pub field_0x0090: u32,
    pub field_0x0094: u32,
    pub field_0x0098: u32,
    pub field_0x009c: u32,
    pub field_0x00a0: u32,
    pub field_0x00a4: u32,
    pub field_0x00a8: u32,
    pub field_0x00ac: u32,
    pub field_0x00b0: u64,
    pub field_0x00b8: u64,
    pub field_0x00c4: u32,
    pub field_0x00e0: u32,
    pub field_0x0120: u32,
    pub field_0x0124: u32,
    pub field_0x0128: u32,
    pub field_0x012c: u32,
    pub field_0x0130: u32,
    pub field_0x0134: u32,
    pub field_0x0138: u32,
    pub field_0x013c: u32,
    pub field_0x0140: u32,
    pub field_0x0144: u32,
    pub field_0x0148: u32,
    pub field_0x014c: u32,
    pub field_0x0150: u32,
    pub field_0x0154: u32,
    pub field_0x0158: u32,
    pub field_0x015c: u32,
    pub field_0x01c0: u32,
    pub field_0x01c8: u8,
}

impl ccGobjGimTimBarrel {
    pub const CLASS_NAME: &'static str = "ccGobjGimTimBarrel";
    pub const VTABLE_RVA: u32 = 0x008beee8;
    pub const METHOD_COUNT: usize = 14;
    pub const BASE_CLASSES: &'static [&'static str] = &["ccGobjGimBase", "ccGameObj"];
    pub const VIRTUAL_METHODS: &'static [u64] = &[0x00000001804c8e90, 0x00000001804ce160, 0x000000018045e4c0, 0x00000001800d3090, 0x000000018019e550, 0x00000001804ce570, 0x00000001804ce380, 0x00000001800d3090, 0x00000001800d3090, 0x00000001804ce400, 0x00000001804ce830, 0x000000018045e2d0, 0x000000018045eec0, 0x000000018045efd0];

    pub fn new() -> Self {
        Self {
            vtable_rva: Self::VTABLE_RVA,
            field_0x0000: Default::default(),
            field_0x0004: Default::default(),
            field_0x0010: Default::default(),
            field_0x0018: Default::default(),
            field_0x001c: Default::default(),
            field_0x0020: Default::default(),
            field_0x0024: Default::default(),
            field_0x002c: Default::default(),
            field_0x0040: Default::default(),
            field_0x0090: Default::default(),
            field_0x0094: Default::default(),
            field_0x0098: Default::default(),
            field_0x009c: Default::default(),
            field_0x00a0: Default::default(),
            field_0x00a4: Default::default(),
            field_0x00a8: Default::default(),
            field_0x00ac: Default::default(),
            field_0x00b0: Default::default(),
            field_0x00b8: Default::default(),
            field_0x00c4: Default::default(),
            field_0x00e0: Default::default(),
            field_0x0120: Default::default(),
            field_0x0124: Default::default(),
            field_0x0128: Default::default(),
            field_0x012c: Default::default(),
            field_0x0130: Default::default(),
            field_0x0134: Default::default(),
            field_0x0138: Default::default(),
            field_0x013c: Default::default(),
            field_0x0140: Default::default(),
            field_0x0144: Default::default(),
            field_0x0148: Default::default(),
            field_0x014c: Default::default(),
            field_0x0150: Default::default(),
            field_0x0154: Default::default(),
            field_0x0158: Default::default(),
            field_0x015c: Default::default(),
            field_0x01c0: Default::default(),
            field_0x01c8: Default::default(),
        }
    }

    /// Decompiled virtual method #0 @ RVA 0x004c8e90 (16 instructions, 3 basic blocks)
    pub fn method_00(&mut self) {
    }

    /// Decompiled virtual method #1 @ RVA 0x004ce160 (13 instructions, 3 basic blocks)
    pub fn method_01(&mut self) {
    }

    /// Decompiled virtual method #2 @ RVA 0x0045e4c0 (42 instructions, 4 basic blocks)
    pub fn method_02(&mut self) {
        // Writes to field at offset +0x0090
        // Writes to field at offset +0x0094
        // Writes to field at offset +0x0098
        // Writes to field at offset +0x009c
        // Writes to field at offset +0x00a0
        // Writes to field at offset +0x00a4
        // Writes to field at offset +0x00a8
        // Writes to field at offset +0x00ac
        // Writes to field at offset +0x0140
        // Writes to field at offset +0x0144
        // Writes to field at offset +0x0148
        // Writes to field at offset +0x014c
        // Writes to field at offset +0x0150
        // Writes to field at offset +0x0154
        // Writes to field at offset +0x0158
        // Writes to field at offset +0x015c
    }

    /// Decompiled virtual method #3 @ RVA 0x000d3090 (1 instructions, 1 basic blocks)
    pub fn method_03(&mut self) {
    }

    /// Decompiled virtual method #4 @ RVA 0x0019e550 (2 instructions, 1 basic blocks)
    pub fn method_04(&mut self) {
        // Writes to field at offset +0x00c4
    }

    /// Decompiled virtual method #5 @ RVA 0x004ce570 (118 instructions, 22 basic blocks)
    pub fn method_05(&mut self) {
        // Writes to field at offset +0x0000
        // Writes to field at offset +0x0004
        // Writes to field at offset +0x0040
    }

    /// Decompiled virtual method #6 @ RVA 0x004ce380 (27 instructions, 9 basic blocks)
    pub fn method_06(&mut self) {
        // Writes to field at offset +0x01c0
        // Writes to field at offset +0x01c8
    }

    /// Decompiled virtual method #7 @ RVA 0x000d3090 (1 instructions, 1 basic blocks)
    pub fn method_07(&mut self) {
    }

    /// Decompiled virtual method #8 @ RVA 0x000d3090 (1 instructions, 1 basic blocks)
    pub fn method_08(&mut self) {
    }

    /// Decompiled virtual method #9 @ RVA 0x004ce400 (27 instructions, 6 basic blocks)
    pub fn method_09(&mut self) {
    }

    /// Decompiled virtual method #10 @ RVA 0x004ce830 (30 instructions, 3 basic blocks)
    pub fn method_10(&mut self) {
        // Writes to field at offset +0x0090
        // Writes to field at offset +0x0094
        // Writes to field at offset +0x0098
        // Writes to field at offset +0x009c
        // Writes to field at offset +0x00a0
        // Writes to field at offset +0x00a4
        // Writes to field at offset +0x00a8
        // Writes to field at offset +0x00ac
    }

    /// Decompiled virtual method #11 @ RVA 0x0045e2d0 (33 instructions, 3 basic blocks)
    pub fn method_11(&mut self) {
        // Writes to field at offset +0x0090
        // Writes to field at offset +0x00a0
        // Writes to field at offset +0x00a4
        // Writes to field at offset +0x00a8
        // Writes to field at offset +0x00ac
    }

    /// Decompiled virtual method #12 @ RVA 0x0045eec0 (67 instructions, 22 basic blocks)
    pub fn method_12(&mut self) {
        // Writes to field at offset +0x001c
        // Writes to field at offset +0x0040
        // Writes to field at offset +0x0090
        // Writes to field at offset +0x00b0
    }

    /// Decompiled virtual method #13 @ RVA 0x0045efd0 (27 instructions, 5 basic blocks)
    pub fn method_13(&mut self) {
        // Writes to field at offset +0x0090
        // Writes to field at offset +0x00b0
    }

}

impl CcClass for ccGobjGimTimBarrel {
    fn class_name(&self) -> &'static str { Self::CLASS_NAME }
    fn vtable_rva(&self) -> u32 { Self::VTABLE_RVA }
    fn virtual_method_count(&self) -> usize { Self::METHOD_COUNT }
    fn base_classes(&self) -> &'static [&'static str] { Self::BASE_CLASSES }
}

/// Decompiled CC2 class: `ccGobjGimTimBox` (VTable RVA: 0x008bf488, 14 virtual methods, 38 recovered fields)
/// Inherits: ccGobjGimBase, ccGameObj
#[derive(Debug, Clone, Default)]
pub struct ccGobjGimTimBox {
    pub vtable_rva: u32,
    pub field_0x0000: u32,
    pub field_0x0004: u32,
    pub field_0x0008: u16,
    pub field_0x000a: u16,
    pub field_0x0010: u64,
    pub field_0x0018: u64,
    pub field_0x001c: u8,
    pub field_0x0020: u32,
    pub field_0x0024: u16,
    pub field_0x002c: u16,
    pub field_0x0040: u32,
    pub field_0x0090: u32,
    pub field_0x0094: u32,
    pub field_0x0098: u32,
    pub field_0x009c: u32,
    pub field_0x00a0: u32,
    pub field_0x00a4: u32,
    pub field_0x00a8: u32,
    pub field_0x00ac: u32,
    pub field_0x00b0: u64,
    pub field_0x00b8: u64,
    pub field_0x00c4: u32,
    pub field_0x0120: u32,
    pub field_0x0124: u32,
    pub field_0x0128: u32,
    pub field_0x012c: u32,
    pub field_0x0130: u32,
    pub field_0x0134: u32,
    pub field_0x0138: u32,
    pub field_0x013c: u32,
    pub field_0x0140: u32,
    pub field_0x0144: u32,
    pub field_0x0148: u32,
    pub field_0x014c: u32,
    pub field_0x0150: u32,
    pub field_0x0154: u32,
    pub field_0x0158: u32,
    pub field_0x015c: u32,
}

impl ccGobjGimTimBox {
    pub const CLASS_NAME: &'static str = "ccGobjGimTimBox";
    pub const VTABLE_RVA: u32 = 0x008bf488;
    pub const METHOD_COUNT: usize = 14;
    pub const BASE_CLASSES: &'static [&'static str] = &["ccGobjGimBase", "ccGameObj"];
    pub const VIRTUAL_METHODS: &'static [u64] = &[0x00000001804c8e90, 0x00000001804c8ed0, 0x000000018045e4c0, 0x00000001800d3090, 0x000000018019e550, 0x00000001804c93d0, 0x00000001804c90f0, 0x00000001800d3090, 0x00000001800d3090, 0x00000001804c91a0, 0x00000001804c9670, 0x000000018045e2d0, 0x000000018045eec0, 0x000000018045efd0];

    pub fn new() -> Self {
        Self {
            vtable_rva: Self::VTABLE_RVA,
            field_0x0000: Default::default(),
            field_0x0004: Default::default(),
            field_0x0008: Default::default(),
            field_0x000a: Default::default(),
            field_0x0010: Default::default(),
            field_0x0018: Default::default(),
            field_0x001c: Default::default(),
            field_0x0020: Default::default(),
            field_0x0024: Default::default(),
            field_0x002c: Default::default(),
            field_0x0040: Default::default(),
            field_0x0090: Default::default(),
            field_0x0094: Default::default(),
            field_0x0098: Default::default(),
            field_0x009c: Default::default(),
            field_0x00a0: Default::default(),
            field_0x00a4: Default::default(),
            field_0x00a8: Default::default(),
            field_0x00ac: Default::default(),
            field_0x00b0: Default::default(),
            field_0x00b8: Default::default(),
            field_0x00c4: Default::default(),
            field_0x0120: Default::default(),
            field_0x0124: Default::default(),
            field_0x0128: Default::default(),
            field_0x012c: Default::default(),
            field_0x0130: Default::default(),
            field_0x0134: Default::default(),
            field_0x0138: Default::default(),
            field_0x013c: Default::default(),
            field_0x0140: Default::default(),
            field_0x0144: Default::default(),
            field_0x0148: Default::default(),
            field_0x014c: Default::default(),
            field_0x0150: Default::default(),
            field_0x0154: Default::default(),
            field_0x0158: Default::default(),
            field_0x015c: Default::default(),
        }
    }

    /// Decompiled virtual method #0 @ RVA 0x004c8e90 (16 instructions, 3 basic blocks)
    pub fn method_00(&mut self) {
    }

    /// Decompiled virtual method #1 @ RVA 0x004c8ed0 (13 instructions, 3 basic blocks)
    pub fn method_01(&mut self) {
    }

    /// Decompiled virtual method #2 @ RVA 0x0045e4c0 (42 instructions, 4 basic blocks)
    pub fn method_02(&mut self) {
        // Writes to field at offset +0x0090
        // Writes to field at offset +0x0094
        // Writes to field at offset +0x0098
        // Writes to field at offset +0x009c
        // Writes to field at offset +0x00a0
        // Writes to field at offset +0x00a4
        // Writes to field at offset +0x00a8
        // Writes to field at offset +0x00ac
        // Writes to field at offset +0x0140
        // Writes to field at offset +0x0144
        // Writes to field at offset +0x0148
        // Writes to field at offset +0x014c
        // Writes to field at offset +0x0150
        // Writes to field at offset +0x0154
        // Writes to field at offset +0x0158
        // Writes to field at offset +0x015c
    }

    /// Decompiled virtual method #3 @ RVA 0x000d3090 (1 instructions, 1 basic blocks)
    pub fn method_03(&mut self) {
    }

    /// Decompiled virtual method #4 @ RVA 0x0019e550 (2 instructions, 1 basic blocks)
    pub fn method_04(&mut self) {
        // Writes to field at offset +0x00c4
    }

    /// Decompiled virtual method #5 @ RVA 0x004c93d0 (119 instructions, 20 basic blocks)
    pub fn method_05(&mut self) {
        // Writes to field at offset +0x0000
        // Writes to field at offset +0x0004
        // Writes to field at offset +0x0008
        // Writes to field at offset +0x000a
        // Writes to field at offset +0x0040
    }

    /// Decompiled virtual method #6 @ RVA 0x004c90f0 (35 instructions, 10 basic blocks)
    pub fn method_06(&mut self) {
    }

    /// Decompiled virtual method #7 @ RVA 0x000d3090 (1 instructions, 1 basic blocks)
    pub fn method_07(&mut self) {
    }

    /// Decompiled virtual method #8 @ RVA 0x000d3090 (1 instructions, 1 basic blocks)
    pub fn method_08(&mut self) {
    }

    /// Decompiled virtual method #9 @ RVA 0x004c91a0 (27 instructions, 6 basic blocks)
    pub fn method_09(&mut self) {
    }

    /// Decompiled virtual method #10 @ RVA 0x004c9670 (33 instructions, 4 basic blocks)
    pub fn method_10(&mut self) {
        // Writes to field at offset +0x0090
        // Writes to field at offset +0x0094
        // Writes to field at offset +0x0098
        // Writes to field at offset +0x009c
        // Writes to field at offset +0x00a0
        // Writes to field at offset +0x00a4
        // Writes to field at offset +0x00a8
        // Writes to field at offset +0x00ac
    }

    /// Decompiled virtual method #11 @ RVA 0x0045e2d0 (33 instructions, 3 basic blocks)
    pub fn method_11(&mut self) {
        // Writes to field at offset +0x0090
        // Writes to field at offset +0x00a0
        // Writes to field at offset +0x00a4
        // Writes to field at offset +0x00a8
        // Writes to field at offset +0x00ac
    }

    /// Decompiled virtual method #12 @ RVA 0x0045eec0 (67 instructions, 22 basic blocks)
    pub fn method_12(&mut self) {
        // Writes to field at offset +0x001c
        // Writes to field at offset +0x0040
        // Writes to field at offset +0x0090
        // Writes to field at offset +0x00b0
    }

    /// Decompiled virtual method #13 @ RVA 0x0045efd0 (27 instructions, 5 basic blocks)
    pub fn method_13(&mut self) {
        // Writes to field at offset +0x0090
        // Writes to field at offset +0x00b0
    }

}

impl CcClass for ccGobjGimTimBox {
    fn class_name(&self) -> &'static str { Self::CLASS_NAME }
    fn vtable_rva(&self) -> u32 { Self::VTABLE_RVA }
    fn virtual_method_count(&self) -> usize { Self::METHOD_COUNT }
    fn base_classes(&self) -> &'static [&'static str] { Self::BASE_CLASSES }
}

/// Decompiled CC2 class: `ccGobjGimTimTree` (VTable RVA: 0x008bf500, 14 virtual methods, 38 recovered fields)
/// Inherits: ccGobjGimBase, ccGameObj
#[derive(Debug, Clone, Default)]
pub struct ccGobjGimTimTree {
    pub vtable_rva: u32,
    pub field_0x0000: u32,
    pub field_0x0004: u32,
    pub field_0x0008: u16,
    pub field_0x000a: u16,
    pub field_0x0010: u64,
    pub field_0x0018: u64,
    pub field_0x001c: u8,
    pub field_0x0020: u32,
    pub field_0x0024: u16,
    pub field_0x002c: u16,
    pub field_0x0040: u32,
    pub field_0x0090: u32,
    pub field_0x0094: u32,
    pub field_0x0098: u32,
    pub field_0x009c: u32,
    pub field_0x00a0: u32,
    pub field_0x00a4: u32,
    pub field_0x00a8: u32,
    pub field_0x00ac: u32,
    pub field_0x00b0: u64,
    pub field_0x00b8: u64,
    pub field_0x00c4: u32,
    pub field_0x0120: u32,
    pub field_0x0124: u32,
    pub field_0x0128: u32,
    pub field_0x012c: u32,
    pub field_0x0130: u32,
    pub field_0x0134: u32,
    pub field_0x0138: u32,
    pub field_0x013c: u32,
    pub field_0x0140: u32,
    pub field_0x0144: u32,
    pub field_0x0148: u32,
    pub field_0x014c: u32,
    pub field_0x0150: u32,
    pub field_0x0154: u32,
    pub field_0x0158: u32,
    pub field_0x015c: u32,
}

impl ccGobjGimTimTree {
    pub const CLASS_NAME: &'static str = "ccGobjGimTimTree";
    pub const VTABLE_RVA: u32 = 0x008bf500;
    pub const METHOD_COUNT: usize = 14;
    pub const BASE_CLASSES: &'static [&'static str] = &["ccGobjGimBase", "ccGameObj"];
    pub const VIRTUAL_METHODS: &'static [u64] = &[0x00000001804c8e90, 0x00000001804c9840, 0x000000018045e4c0, 0x00000001800d3090, 0x000000018019e550, 0x00000001804c9c10, 0x00000001804c9a70, 0x00000001800d3090, 0x00000001800d3090, 0x00000001804c9b40, 0x00000001804ca030, 0x000000018045e2d0, 0x000000018045eec0, 0x000000018045efd0];

    pub fn new() -> Self {
        Self {
            vtable_rva: Self::VTABLE_RVA,
            field_0x0000: Default::default(),
            field_0x0004: Default::default(),
            field_0x0008: Default::default(),
            field_0x000a: Default::default(),
            field_0x0010: Default::default(),
            field_0x0018: Default::default(),
            field_0x001c: Default::default(),
            field_0x0020: Default::default(),
            field_0x0024: Default::default(),
            field_0x002c: Default::default(),
            field_0x0040: Default::default(),
            field_0x0090: Default::default(),
            field_0x0094: Default::default(),
            field_0x0098: Default::default(),
            field_0x009c: Default::default(),
            field_0x00a0: Default::default(),
            field_0x00a4: Default::default(),
            field_0x00a8: Default::default(),
            field_0x00ac: Default::default(),
            field_0x00b0: Default::default(),
            field_0x00b8: Default::default(),
            field_0x00c4: Default::default(),
            field_0x0120: Default::default(),
            field_0x0124: Default::default(),
            field_0x0128: Default::default(),
            field_0x012c: Default::default(),
            field_0x0130: Default::default(),
            field_0x0134: Default::default(),
            field_0x0138: Default::default(),
            field_0x013c: Default::default(),
            field_0x0140: Default::default(),
            field_0x0144: Default::default(),
            field_0x0148: Default::default(),
            field_0x014c: Default::default(),
            field_0x0150: Default::default(),
            field_0x0154: Default::default(),
            field_0x0158: Default::default(),
            field_0x015c: Default::default(),
        }
    }

    /// Decompiled virtual method #0 @ RVA 0x004c8e90 (16 instructions, 3 basic blocks)
    pub fn method_00(&mut self) {
    }

    /// Decompiled virtual method #1 @ RVA 0x004c9840 (13 instructions, 3 basic blocks)
    pub fn method_01(&mut self) {
    }

    /// Decompiled virtual method #2 @ RVA 0x0045e4c0 (42 instructions, 4 basic blocks)
    pub fn method_02(&mut self) {
        // Writes to field at offset +0x0090
        // Writes to field at offset +0x0094
        // Writes to field at offset +0x0098
        // Writes to field at offset +0x009c
        // Writes to field at offset +0x00a0
        // Writes to field at offset +0x00a4
        // Writes to field at offset +0x00a8
        // Writes to field at offset +0x00ac
        // Writes to field at offset +0x0140
        // Writes to field at offset +0x0144
        // Writes to field at offset +0x0148
        // Writes to field at offset +0x014c
        // Writes to field at offset +0x0150
        // Writes to field at offset +0x0154
        // Writes to field at offset +0x0158
        // Writes to field at offset +0x015c
    }

    /// Decompiled virtual method #3 @ RVA 0x000d3090 (1 instructions, 1 basic blocks)
    pub fn method_03(&mut self) {
    }

    /// Decompiled virtual method #4 @ RVA 0x0019e550 (2 instructions, 1 basic blocks)
    pub fn method_04(&mut self) {
        // Writes to field at offset +0x00c4
    }

    /// Decompiled virtual method #5 @ RVA 0x004c9c10 (116 instructions, 21 basic blocks)
    pub fn method_05(&mut self) {
        // Writes to field at offset +0x0040
    }

    /// Decompiled virtual method #6 @ RVA 0x004c9a70 (46 instructions, 12 basic blocks)
    pub fn method_06(&mut self) {
        // Writes to field at offset +0x0000
        // Writes to field at offset +0x0004
        // Writes to field at offset +0x0008
        // Writes to field at offset +0x000a
    }

    /// Decompiled virtual method #7 @ RVA 0x000d3090 (1 instructions, 1 basic blocks)
    pub fn method_07(&mut self) {
    }

    /// Decompiled virtual method #8 @ RVA 0x000d3090 (1 instructions, 1 basic blocks)
    pub fn method_08(&mut self) {
    }

    /// Decompiled virtual method #9 @ RVA 0x004c9b40 (29 instructions, 5 basic blocks)
    pub fn method_09(&mut self) {
    }

    /// Decompiled virtual method #10 @ RVA 0x004ca030 (38 instructions, 3 basic blocks)
    pub fn method_10(&mut self) {
        // Writes to field at offset +0x0090
        // Writes to field at offset +0x0094
        // Writes to field at offset +0x0098
        // Writes to field at offset +0x009c
        // Writes to field at offset +0x00a0
        // Writes to field at offset +0x00a4
        // Writes to field at offset +0x00a8
        // Writes to field at offset +0x00ac
    }

    /// Decompiled virtual method #11 @ RVA 0x0045e2d0 (33 instructions, 3 basic blocks)
    pub fn method_11(&mut self) {
        // Writes to field at offset +0x0090
        // Writes to field at offset +0x00a0
        // Writes to field at offset +0x00a4
        // Writes to field at offset +0x00a8
        // Writes to field at offset +0x00ac
    }

    /// Decompiled virtual method #12 @ RVA 0x0045eec0 (67 instructions, 22 basic blocks)
    pub fn method_12(&mut self) {
        // Writes to field at offset +0x001c
        // Writes to field at offset +0x0040
        // Writes to field at offset +0x0090
        // Writes to field at offset +0x00b0
    }

    /// Decompiled virtual method #13 @ RVA 0x0045efd0 (27 instructions, 5 basic blocks)
    pub fn method_13(&mut self) {
        // Writes to field at offset +0x0090
        // Writes to field at offset +0x00b0
    }

}

impl CcClass for ccGobjGimTimTree {
    fn class_name(&self) -> &'static str { Self::CLASS_NAME }
    fn vtable_rva(&self) -> u32 { Self::VTABLE_RVA }
    fn virtual_method_count(&self) -> usize { Self::METHOD_COUNT }
    fn base_classes(&self) -> &'static [&'static str] { Self::BASE_CLASSES }
}

/// Decompiled CC2 class: `ccGobjGimTown01Door` (VTable RVA: 0x008bf050, 14 virtual methods, 40 recovered fields)
/// Inherits: ccGobjGimBase, ccGameObj
#[derive(Debug, Clone, Default)]
pub struct ccGobjGimTown01Door {
    pub vtable_rva: u32,
    pub field_0x0000: u32,
    pub field_0x0004: u32,
    pub field_0x0008: u16,
    pub field_0x000a: u16,
    pub field_0x0010: u64,
    pub field_0x0018: u64,
    pub field_0x001c: u8,
    pub field_0x0020: u32,
    pub field_0x0024: u16,
    pub field_0x002c: u16,
    pub field_0x0040: u32,
    pub field_0x0090: u32,
    pub field_0x0094: u32,
    pub field_0x0098: u32,
    pub field_0x009c: u32,
    pub field_0x00a0: u32,
    pub field_0x00a4: u32,
    pub field_0x00a8: u32,
    pub field_0x00ac: u32,
    pub field_0x00b0: u64,
    pub field_0x00b8: u64,
    pub field_0x00c4: u32,
    pub field_0x00d1: u8,
    pub field_0x0120: u32,
    pub field_0x0124: u32,
    pub field_0x0128: u32,
    pub field_0x012c: u32,
    pub field_0x0130: u32,
    pub field_0x0134: u32,
    pub field_0x0138: u32,
    pub field_0x013c: u32,
    pub field_0x0140: u32,
    pub field_0x0144: u32,
    pub field_0x0148: u32,
    pub field_0x014c: u32,
    pub field_0x0150: u32,
    pub field_0x0154: u32,
    pub field_0x0158: u32,
    pub field_0x015c: u32,
    pub field_0x015d: u8,
}

impl ccGobjGimTown01Door {
    pub const CLASS_NAME: &'static str = "ccGobjGimTown01Door";
    pub const VTABLE_RVA: u32 = 0x008bf050;
    pub const METHOD_COUNT: usize = 14;
    pub const BASE_CLASSES: &'static [&'static str] = &["ccGobjGimBase", "ccGameObj"];
    pub const VIRTUAL_METHODS: &'static [u64] = &[0x00000001804c8e90, 0x00000001804cf050, 0x000000018045e4c0, 0x00000001800d3090, 0x000000018019e550, 0x00000001804cf9b0, 0x00000001800d3090, 0x00000001800d3090, 0x00000001800d3090, 0x00000001804cf5f0, 0x00000001804cfd20, 0x000000018045e2d0, 0x000000018045eec0, 0x000000018045efd0];

    pub fn new() -> Self {
        Self {
            vtable_rva: Self::VTABLE_RVA,
            field_0x0000: Default::default(),
            field_0x0004: Default::default(),
            field_0x0008: Default::default(),
            field_0x000a: Default::default(),
            field_0x0010: Default::default(),
            field_0x0018: Default::default(),
            field_0x001c: Default::default(),
            field_0x0020: Default::default(),
            field_0x0024: Default::default(),
            field_0x002c: Default::default(),
            field_0x0040: Default::default(),
            field_0x0090: Default::default(),
            field_0x0094: Default::default(),
            field_0x0098: Default::default(),
            field_0x009c: Default::default(),
            field_0x00a0: Default::default(),
            field_0x00a4: Default::default(),
            field_0x00a8: Default::default(),
            field_0x00ac: Default::default(),
            field_0x00b0: Default::default(),
            field_0x00b8: Default::default(),
            field_0x00c4: Default::default(),
            field_0x00d1: Default::default(),
            field_0x0120: Default::default(),
            field_0x0124: Default::default(),
            field_0x0128: Default::default(),
            field_0x012c: Default::default(),
            field_0x0130: Default::default(),
            field_0x0134: Default::default(),
            field_0x0138: Default::default(),
            field_0x013c: Default::default(),
            field_0x0140: Default::default(),
            field_0x0144: Default::default(),
            field_0x0148: Default::default(),
            field_0x014c: Default::default(),
            field_0x0150: Default::default(),
            field_0x0154: Default::default(),
            field_0x0158: Default::default(),
            field_0x015c: Default::default(),
            field_0x015d: Default::default(),
        }
    }

    /// Decompiled virtual method #0 @ RVA 0x004c8e90 (16 instructions, 3 basic blocks)
    pub fn method_00(&mut self) {
    }

    /// Decompiled virtual method #1 @ RVA 0x004cf050 (15 instructions, 3 basic blocks)
    pub fn method_01(&mut self) {
    }

    /// Decompiled virtual method #2 @ RVA 0x0045e4c0 (42 instructions, 4 basic blocks)
    pub fn method_02(&mut self) {
        // Writes to field at offset +0x0090
        // Writes to field at offset +0x0094
        // Writes to field at offset +0x0098
        // Writes to field at offset +0x009c
        // Writes to field at offset +0x00a0
        // Writes to field at offset +0x00a4
        // Writes to field at offset +0x00a8
        // Writes to field at offset +0x00ac
        // Writes to field at offset +0x0140
        // Writes to field at offset +0x0144
        // Writes to field at offset +0x0148
        // Writes to field at offset +0x014c
        // Writes to field at offset +0x0150
        // Writes to field at offset +0x0154
        // Writes to field at offset +0x0158
        // Writes to field at offset +0x015c
    }

    /// Decompiled virtual method #3 @ RVA 0x000d3090 (1 instructions, 1 basic blocks)
    pub fn method_03(&mut self) {
    }

    /// Decompiled virtual method #4 @ RVA 0x0019e550 (2 instructions, 1 basic blocks)
    pub fn method_04(&mut self) {
        // Writes to field at offset +0x00c4
    }

    /// Decompiled virtual method #5 @ RVA 0x004cf9b0 (132 instructions, 32 basic blocks)
    pub fn method_05(&mut self) {
        // Writes to field at offset +0x0040
    }

    /// Decompiled virtual method #6 @ RVA 0x000d3090 (1 instructions, 1 basic blocks)
    pub fn method_06(&mut self) {
    }

    /// Decompiled virtual method #7 @ RVA 0x000d3090 (1 instructions, 1 basic blocks)
    pub fn method_07(&mut self) {
    }

    /// Decompiled virtual method #8 @ RVA 0x000d3090 (1 instructions, 1 basic blocks)
    pub fn method_08(&mut self) {
    }

    /// Decompiled virtual method #9 @ RVA 0x004cf5f0 (115 instructions, 23 basic blocks)
    pub fn method_09(&mut self) {
        // Writes to field at offset +0x0000
        // Writes to field at offset +0x0004
        // Writes to field at offset +0x0008
        // Writes to field at offset +0x000a
        // Writes to field at offset +0x0010
        // Writes to field at offset +0x00d1
        // Writes to field at offset +0x015d
    }

    /// Decompiled virtual method #10 @ RVA 0x004cfd20 (16 instructions, 4 basic blocks)
    pub fn method_10(&mut self) {
    }

    /// Decompiled virtual method #11 @ RVA 0x0045e2d0 (33 instructions, 3 basic blocks)
    pub fn method_11(&mut self) {
        // Writes to field at offset +0x0090
        // Writes to field at offset +0x00a0
        // Writes to field at offset +0x00a4
        // Writes to field at offset +0x00a8
        // Writes to field at offset +0x00ac
    }

    /// Decompiled virtual method #12 @ RVA 0x0045eec0 (67 instructions, 22 basic blocks)
    pub fn method_12(&mut self) {
        // Writes to field at offset +0x001c
        // Writes to field at offset +0x0040
        // Writes to field at offset +0x0090
        // Writes to field at offset +0x00b0
    }

    /// Decompiled virtual method #13 @ RVA 0x0045efd0 (27 instructions, 5 basic blocks)
    pub fn method_13(&mut self) {
        // Writes to field at offset +0x0090
        // Writes to field at offset +0x00b0
    }

}

impl CcClass for ccGobjGimTown01Door {
    fn class_name(&self) -> &'static str { Self::CLASS_NAME }
    fn vtable_rva(&self) -> u32 { Self::VTABLE_RVA }
    fn virtual_method_count(&self) -> usize { Self::METHOD_COUNT }
    fn base_classes(&self) -> &'static [&'static str] { Self::BASE_CLASSES }
}

/// Decompiled CC2 class: `ccGobjGimTownWarp` (VTable RVA: 0x008bedf8, 14 virtual methods, 37 recovered fields)
/// Inherits: ccGobjGimBase, ccGameObj
#[derive(Debug, Clone, Default)]
pub struct ccGobjGimTownWarp {
    pub vtable_rva: u32,
    pub field_0x0010: u64,
    pub field_0x0018: u64,
    pub field_0x001b: u32,
    pub field_0x001c: u8,
    pub field_0x0020: u32,
    pub field_0x0024: u16,
    pub field_0x002c: u16,
    pub field_0x0040: u32,
    pub field_0x0090: u32,
    pub field_0x0094: u32,
    pub field_0x0098: u32,
    pub field_0x009c: u32,
    pub field_0x00a0: u32,
    pub field_0x00a4: u32,
    pub field_0x00a8: u32,
    pub field_0x00ac: u32,
    pub field_0x00b0: u64,
    pub field_0x00b8: u64,
    pub field_0x00c4: u32,
    pub field_0x0120: u32,
    pub field_0x0124: u32,
    pub field_0x0128: u32,
    pub field_0x012c: u32,
    pub field_0x0130: u32,
    pub field_0x0134: u32,
    pub field_0x0138: u32,
    pub field_0x013c: u32,
    pub field_0x0140: u32,
    pub field_0x0144: u32,
    pub field_0x0148: u32,
    pub field_0x014c: u32,
    pub field_0x0150: u32,
    pub field_0x0154: u32,
    pub field_0x0158: u32,
    pub field_0x015c: u32,
    pub field_0x01c0: f32,
    pub field_0x0c40: u64,
}

impl ccGobjGimTownWarp {
    pub const CLASS_NAME: &'static str = "ccGobjGimTownWarp";
    pub const VTABLE_RVA: u32 = 0x008bedf8;
    pub const METHOD_COUNT: usize = 14;
    pub const BASE_CLASSES: &'static [&'static str] = &["ccGobjGimBase", "ccGameObj"];
    pub const VIRTUAL_METHODS: &'static [u64] = &[0x00000001804c8e90, 0x00000001804cfeb0, 0x000000018045e4c0, 0x00000001800d3090, 0x000000018019e550, 0x00000001804d0130, 0x00000001800d3090, 0x00000001800d3090, 0x00000001800d3090, 0x00000001804d00c0, 0x00000001804caf40, 0x000000018045e2d0, 0x000000018045eec0, 0x000000018045efd0];

    pub fn new() -> Self {
        Self {
            vtable_rva: Self::VTABLE_RVA,
            field_0x0010: Default::default(),
            field_0x0018: Default::default(),
            field_0x001b: Default::default(),
            field_0x001c: Default::default(),
            field_0x0020: Default::default(),
            field_0x0024: Default::default(),
            field_0x002c: Default::default(),
            field_0x0040: Default::default(),
            field_0x0090: Default::default(),
            field_0x0094: Default::default(),
            field_0x0098: Default::default(),
            field_0x009c: Default::default(),
            field_0x00a0: Default::default(),
            field_0x00a4: Default::default(),
            field_0x00a8: Default::default(),
            field_0x00ac: Default::default(),
            field_0x00b0: Default::default(),
            field_0x00b8: Default::default(),
            field_0x00c4: Default::default(),
            field_0x0120: Default::default(),
            field_0x0124: Default::default(),
            field_0x0128: Default::default(),
            field_0x012c: Default::default(),
            field_0x0130: Default::default(),
            field_0x0134: Default::default(),
            field_0x0138: Default::default(),
            field_0x013c: Default::default(),
            field_0x0140: Default::default(),
            field_0x0144: Default::default(),
            field_0x0148: Default::default(),
            field_0x014c: Default::default(),
            field_0x0150: Default::default(),
            field_0x0154: Default::default(),
            field_0x0158: Default::default(),
            field_0x015c: Default::default(),
            field_0x01c0: Default::default(),
            field_0x0c40: Default::default(),
        }
    }

    /// Decompiled virtual method #0 @ RVA 0x004c8e90 (16 instructions, 3 basic blocks)
    pub fn method_00(&mut self) {
    }

    /// Decompiled virtual method #1 @ RVA 0x004cfeb0 (13 instructions, 3 basic blocks)
    pub fn method_01(&mut self) {
    }

    /// Decompiled virtual method #2 @ RVA 0x0045e4c0 (42 instructions, 4 basic blocks)
    pub fn method_02(&mut self) {
        // Writes to field at offset +0x0090
        // Writes to field at offset +0x0094
        // Writes to field at offset +0x0098
        // Writes to field at offset +0x009c
        // Writes to field at offset +0x00a0
        // Writes to field at offset +0x00a4
        // Writes to field at offset +0x00a8
        // Writes to field at offset +0x00ac
        // Writes to field at offset +0x0140
        // Writes to field at offset +0x0144
        // Writes to field at offset +0x0148
        // Writes to field at offset +0x014c
        // Writes to field at offset +0x0150
        // Writes to field at offset +0x0154
        // Writes to field at offset +0x0158
        // Writes to field at offset +0x015c
    }

    /// Decompiled virtual method #3 @ RVA 0x000d3090 (1 instructions, 1 basic blocks)
    pub fn method_03(&mut self) {
    }

    /// Decompiled virtual method #4 @ RVA 0x0019e550 (2 instructions, 1 basic blocks)
    pub fn method_04(&mut self) {
        // Writes to field at offset +0x00c4
    }

    /// Decompiled virtual method #5 @ RVA 0x004d0130 (120 instructions, 36 basic blocks)
    pub fn method_05(&mut self) {
        // Writes to field at offset +0x0024
    }

    /// Decompiled virtual method #6 @ RVA 0x000d3090 (1 instructions, 1 basic blocks)
    pub fn method_06(&mut self) {
    }

    /// Decompiled virtual method #7 @ RVA 0x000d3090 (1 instructions, 1 basic blocks)
    pub fn method_07(&mut self) {
    }

    /// Decompiled virtual method #8 @ RVA 0x000d3090 (1 instructions, 1 basic blocks)
    pub fn method_08(&mut self) {
    }

    /// Decompiled virtual method #9 @ RVA 0x004d00c0 (23 instructions, 7 basic blocks)
    pub fn method_09(&mut self) {
        // Writes to field at offset +0x01c0
    }

    /// Decompiled virtual method #10 @ RVA 0x004caf40 (38 instructions, 3 basic blocks)
    pub fn method_10(&mut self) {
        // Writes to field at offset +0x0090
        // Writes to field at offset +0x0094
        // Writes to field at offset +0x0098
        // Writes to field at offset +0x009c
        // Writes to field at offset +0x00a0
        // Writes to field at offset +0x00a4
        // Writes to field at offset +0x00a8
        // Writes to field at offset +0x00ac
    }

    /// Decompiled virtual method #11 @ RVA 0x0045e2d0 (33 instructions, 3 basic blocks)
    pub fn method_11(&mut self) {
        // Writes to field at offset +0x0090
        // Writes to field at offset +0x00a0
        // Writes to field at offset +0x00a4
        // Writes to field at offset +0x00a8
        // Writes to field at offset +0x00ac
    }

    /// Decompiled virtual method #12 @ RVA 0x0045eec0 (67 instructions, 22 basic blocks)
    pub fn method_12(&mut self) {
        // Writes to field at offset +0x001c
        // Writes to field at offset +0x0040
        // Writes to field at offset +0x0090
        // Writes to field at offset +0x00b0
    }

    /// Decompiled virtual method #13 @ RVA 0x0045efd0 (27 instructions, 5 basic blocks)
    pub fn method_13(&mut self) {
        // Writes to field at offset +0x0090
        // Writes to field at offset +0x00b0
    }

}

impl CcClass for ccGobjGimTownWarp {
    fn class_name(&self) -> &'static str { Self::CLASS_NAME }
    fn vtable_rva(&self) -> u32 { Self::VTABLE_RVA }
    fn virtual_method_count(&self) -> usize { Self::METHOD_COUNT }
    fn base_classes(&self) -> &'static [&'static str] { Self::BASE_CLASSES }
}

/// Decompiled CC2 class: `ccGobjGimTrapBox` (VTable RVA: 0x008be330, 14 virtual methods, 44 recovered fields)
/// Inherits: ccGobjGimBase, ccGameObj
#[derive(Debug, Clone, Default)]
pub struct ccGobjGimTrapBox {
    pub vtable_rva: u32,
    pub field_0x0000: u64,
    pub field_0x0004: u32,
    pub field_0x0008: u16,
    pub field_0x000a: u16,
    pub field_0x0010: u64,
    pub field_0x0018: u64,
    pub field_0x001c: u8,
    pub field_0x0020: u32,
    pub field_0x0024: u16,
    pub field_0x002c: u16,
    pub field_0x0040: u32,
    pub field_0x0090: u32,
    pub field_0x0094: u32,
    pub field_0x0098: u32,
    pub field_0x009c: u32,
    pub field_0x00a0: u32,
    pub field_0x00a4: u32,
    pub field_0x00a8: u32,
    pub field_0x00ac: u32,
    pub field_0x00b0: u64,
    pub field_0x00b8: u64,
    pub field_0x00c4: u32,
    pub field_0x0114: u32,
    pub field_0x0120: u32,
    pub field_0x0124: u32,
    pub field_0x0128: u32,
    pub field_0x012c: u32,
    pub field_0x0130: u32,
    pub field_0x0134: u32,
    pub field_0x0138: u32,
    pub field_0x013c: u32,
    pub field_0x0140: u32,
    pub field_0x0144: u32,
    pub field_0x0148: u32,
    pub field_0x014c: u32,
    pub field_0x0150: u32,
    pub field_0x0154: u32,
    pub field_0x0158: u32,
    pub field_0x015c: u32,
    pub field_0x01c4: u32,
    pub field_0x01e0: u64,
    pub field_0x01e8: u32,
    pub field_0x01ec: u32,
    pub field_0x01f8: u32,
}

impl ccGobjGimTrapBox {
    pub const CLASS_NAME: &'static str = "ccGobjGimTrapBox";
    pub const VTABLE_RVA: u32 = 0x008be330;
    pub const METHOD_COUNT: usize = 14;
    pub const BASE_CLASSES: &'static [&'static str] = &["ccGobjGimBase", "ccGameObj"];
    pub const VIRTUAL_METHODS: &'static [u64] = &[0x00000001804ec6e0, 0x00000001804ecc20, 0x000000018045e4c0, 0x00000001800d3090, 0x000000018019e550, 0x00000001804ed210, 0x00000001804ed030, 0x00000001800d3090, 0x00000001800d3090, 0x00000001800d3090, 0x00000001804ecf60, 0x000000018045e2d0, 0x000000018045eec0, 0x000000018045efd0];

    pub fn new() -> Self {
        Self {
            vtable_rva: Self::VTABLE_RVA,
            field_0x0000: Default::default(),
            field_0x0004: Default::default(),
            field_0x0008: Default::default(),
            field_0x000a: Default::default(),
            field_0x0010: Default::default(),
            field_0x0018: Default::default(),
            field_0x001c: Default::default(),
            field_0x0020: Default::default(),
            field_0x0024: Default::default(),
            field_0x002c: Default::default(),
            field_0x0040: Default::default(),
            field_0x0090: Default::default(),
            field_0x0094: Default::default(),
            field_0x0098: Default::default(),
            field_0x009c: Default::default(),
            field_0x00a0: Default::default(),
            field_0x00a4: Default::default(),
            field_0x00a8: Default::default(),
            field_0x00ac: Default::default(),
            field_0x00b0: Default::default(),
            field_0x00b8: Default::default(),
            field_0x00c4: Default::default(),
            field_0x0114: Default::default(),
            field_0x0120: Default::default(),
            field_0x0124: Default::default(),
            field_0x0128: Default::default(),
            field_0x012c: Default::default(),
            field_0x0130: Default::default(),
            field_0x0134: Default::default(),
            field_0x0138: Default::default(),
            field_0x013c: Default::default(),
            field_0x0140: Default::default(),
            field_0x0144: Default::default(),
            field_0x0148: Default::default(),
            field_0x014c: Default::default(),
            field_0x0150: Default::default(),
            field_0x0154: Default::default(),
            field_0x0158: Default::default(),
            field_0x015c: Default::default(),
            field_0x01c4: Default::default(),
            field_0x01e0: Default::default(),
            field_0x01e8: Default::default(),
            field_0x01ec: Default::default(),
            field_0x01f8: Default::default(),
        }
    }

    /// Decompiled virtual method #0 @ RVA 0x004ec6e0 (26 instructions, 6 basic blocks)
    pub fn method_00(&mut self) {
        // Writes to field at offset +0x0000
    }

    /// Decompiled virtual method #1 @ RVA 0x004ecc20 (117 instructions, 24 basic blocks)
    pub fn method_01(&mut self) {
        // Writes to field at offset +0x0000
        // Writes to field at offset +0x0004
        // Writes to field at offset +0x0008
        // Writes to field at offset +0x000a
        // Writes to field at offset +0x001c
        // Writes to field at offset +0x0040
        // Writes to field at offset +0x01e0
        // Writes to field at offset +0x01e8
        // Writes to field at offset +0x01ec
    }

    /// Decompiled virtual method #2 @ RVA 0x0045e4c0 (42 instructions, 4 basic blocks)
    pub fn method_02(&mut self) {
        // Writes to field at offset +0x0090
        // Writes to field at offset +0x0094
        // Writes to field at offset +0x0098
        // Writes to field at offset +0x009c
        // Writes to field at offset +0x00a0
        // Writes to field at offset +0x00a4
        // Writes to field at offset +0x00a8
        // Writes to field at offset +0x00ac
        // Writes to field at offset +0x0140
        // Writes to field at offset +0x0144
        // Writes to field at offset +0x0148
        // Writes to field at offset +0x014c
        // Writes to field at offset +0x0150
        // Writes to field at offset +0x0154
        // Writes to field at offset +0x0158
        // Writes to field at offset +0x015c
    }

    /// Decompiled virtual method #3 @ RVA 0x000d3090 (1 instructions, 1 basic blocks)
    pub fn method_03(&mut self) {
    }

    /// Decompiled virtual method #4 @ RVA 0x0019e550 (2 instructions, 1 basic blocks)
    pub fn method_04(&mut self) {
        // Writes to field at offset +0x00c4
    }

    /// Decompiled virtual method #5 @ RVA 0x004ed210 (124 instructions, 26 basic blocks)
    pub fn method_05(&mut self) {
        // Writes to field at offset +0x0000
    }

    /// Decompiled virtual method #6 @ RVA 0x004ed030 (97 instructions, 24 basic blocks)
    pub fn method_06(&mut self) {
        // Writes to field at offset +0x0114
    }

    /// Decompiled virtual method #7 @ RVA 0x000d3090 (1 instructions, 1 basic blocks)
    pub fn method_07(&mut self) {
    }

    /// Decompiled virtual method #8 @ RVA 0x000d3090 (1 instructions, 1 basic blocks)
    pub fn method_08(&mut self) {
    }

    /// Decompiled virtual method #9 @ RVA 0x000d3090 (1 instructions, 1 basic blocks)
    pub fn method_09(&mut self) {
    }

    /// Decompiled virtual method #10 @ RVA 0x004ecf60 (38 instructions, 5 basic blocks)
    pub fn method_10(&mut self) {
        // Writes to field at offset +0x0090
        // Writes to field at offset +0x0094
        // Writes to field at offset +0x0098
        // Writes to field at offset +0x009c
        // Writes to field at offset +0x00a0
        // Writes to field at offset +0x00a4
        // Writes to field at offset +0x00a8
        // Writes to field at offset +0x00ac
    }

    /// Decompiled virtual method #11 @ RVA 0x0045e2d0 (33 instructions, 3 basic blocks)
    pub fn method_11(&mut self) {
        // Writes to field at offset +0x0090
        // Writes to field at offset +0x00a0
        // Writes to field at offset +0x00a4
        // Writes to field at offset +0x00a8
        // Writes to field at offset +0x00ac
    }

    /// Decompiled virtual method #12 @ RVA 0x0045eec0 (67 instructions, 22 basic blocks)
    pub fn method_12(&mut self) {
        // Writes to field at offset +0x001c
        // Writes to field at offset +0x0040
        // Writes to field at offset +0x0090
        // Writes to field at offset +0x00b0
    }

    /// Decompiled virtual method #13 @ RVA 0x0045efd0 (27 instructions, 5 basic blocks)
    pub fn method_13(&mut self) {
        // Writes to field at offset +0x0090
        // Writes to field at offset +0x00b0
    }

}

impl CcClass for ccGobjGimTrapBox {
    fn class_name(&self) -> &'static str { Self::CLASS_NAME }
    fn vtable_rva(&self) -> u32 { Self::VTABLE_RVA }
    fn virtual_method_count(&self) -> usize { Self::METHOD_COUNT }
    fn base_classes(&self) -> &'static [&'static str] { Self::BASE_CLASSES }
}

/// Decompiled CC2 class: `ccGobjGimWPStart` (VTable RVA: 0x008be588, 14 virtual methods, 43 recovered fields)
/// Inherits: ccGobjGimBase, ccGameObj
#[derive(Debug, Clone, Default)]
pub struct ccGobjGimWPStart {
    pub vtable_rva: u32,
    pub field_0x0000: u64,
    pub field_0x0004: u32,
    pub field_0x0008: u16,
    pub field_0x0009: u8,
    pub field_0x000a: u16,
    pub field_0x0010: u64,
    pub field_0x0018: u64,
    pub field_0x001c: u8,
    pub field_0x0020: u32,
    pub field_0x0024: u16,
    pub field_0x002c: u16,
    pub field_0x0040: u32,
    pub field_0x0058: u64,
    pub field_0x0090: u32,
    pub field_0x0094: u32,
    pub field_0x0098: u32,
    pub field_0x009c: u32,
    pub field_0x00a0: u32,
    pub field_0x00a4: u32,
    pub field_0x00a8: u32,
    pub field_0x00ac: u32,
    pub field_0x00b0: u64,
    pub field_0x00b8: u64,
    pub field_0x00c4: u32,
    pub field_0x0120: u32,
    pub field_0x0124: u32,
    pub field_0x0128: u32,
    pub field_0x012c: u32,
    pub field_0x0130: u32,
    pub field_0x0134: u32,
    pub field_0x0138: u32,
    pub field_0x013c: u32,
    pub field_0x0140: u32,
    pub field_0x0144: u32,
    pub field_0x0148: u32,
    pub field_0x014c: u32,
    pub field_0x0150: u32,
    pub field_0x0154: u32,
    pub field_0x0158: u32,
    pub field_0x015c: u32,
    pub field_0x01c0: u64,
    pub field_0x01c8: u16,
    pub field_0x01cc: u32,
}

impl ccGobjGimWPStart {
    pub const CLASS_NAME: &'static str = "ccGobjGimWPStart";
    pub const VTABLE_RVA: u32 = 0x008be588;
    pub const METHOD_COUNT: usize = 14;
    pub const BASE_CLASSES: &'static [&'static str] = &["ccGobjGimBase", "ccGameObj"];
    pub const VIRTUAL_METHODS: &'static [u64] = &[0x00000001804eb880, 0x00000001804eb920, 0x000000018045e4c0, 0x00000001800d3090, 0x000000018019e550, 0x00000001804ebec0, 0x00000001804ebea0, 0x00000001800d3090, 0x00000001800d3090, 0x00000001800d3090, 0x00000001804ebc90, 0x000000018045e2d0, 0x000000018045eec0, 0x000000018045efd0];

    pub fn new() -> Self {
        Self {
            vtable_rva: Self::VTABLE_RVA,
            field_0x0000: Default::default(),
            field_0x0004: Default::default(),
            field_0x0008: Default::default(),
            field_0x0009: Default::default(),
            field_0x000a: Default::default(),
            field_0x0010: Default::default(),
            field_0x0018: Default::default(),
            field_0x001c: Default::default(),
            field_0x0020: Default::default(),
            field_0x0024: Default::default(),
            field_0x002c: Default::default(),
            field_0x0040: Default::default(),
            field_0x0058: Default::default(),
            field_0x0090: Default::default(),
            field_0x0094: Default::default(),
            field_0x0098: Default::default(),
            field_0x009c: Default::default(),
            field_0x00a0: Default::default(),
            field_0x00a4: Default::default(),
            field_0x00a8: Default::default(),
            field_0x00ac: Default::default(),
            field_0x00b0: Default::default(),
            field_0x00b8: Default::default(),
            field_0x00c4: Default::default(),
            field_0x0120: Default::default(),
            field_0x0124: Default::default(),
            field_0x0128: Default::default(),
            field_0x012c: Default::default(),
            field_0x0130: Default::default(),
            field_0x0134: Default::default(),
            field_0x0138: Default::default(),
            field_0x013c: Default::default(),
            field_0x0140: Default::default(),
            field_0x0144: Default::default(),
            field_0x0148: Default::default(),
            field_0x014c: Default::default(),
            field_0x0150: Default::default(),
            field_0x0154: Default::default(),
            field_0x0158: Default::default(),
            field_0x015c: Default::default(),
            field_0x01c0: Default::default(),
            field_0x01c8: Default::default(),
            field_0x01cc: Default::default(),
        }
    }

    /// Decompiled virtual method #0 @ RVA 0x004eb880 (39 instructions, 5 basic blocks)
    pub fn method_00(&mut self) {
        // Writes to field at offset +0x0000
    }

    /// Decompiled virtual method #1 @ RVA 0x004eb920 (116 instructions, 18 basic blocks)
    pub fn method_01(&mut self) {
        // Writes to field at offset +0x0000
        // Writes to field at offset +0x0004
        // Writes to field at offset +0x0008
        // Writes to field at offset +0x000a
        // Writes to field at offset +0x0040
    }

    /// Decompiled virtual method #2 @ RVA 0x0045e4c0 (42 instructions, 4 basic blocks)
    pub fn method_02(&mut self) {
        // Writes to field at offset +0x0090
        // Writes to field at offset +0x0094
        // Writes to field at offset +0x0098
        // Writes to field at offset +0x009c
        // Writes to field at offset +0x00a0
        // Writes to field at offset +0x00a4
        // Writes to field at offset +0x00a8
        // Writes to field at offset +0x00ac
        // Writes to field at offset +0x0140
        // Writes to field at offset +0x0144
        // Writes to field at offset +0x0148
        // Writes to field at offset +0x014c
        // Writes to field at offset +0x0150
        // Writes to field at offset +0x0154
        // Writes to field at offset +0x0158
        // Writes to field at offset +0x015c
    }

    /// Decompiled virtual method #3 @ RVA 0x000d3090 (1 instructions, 1 basic blocks)
    pub fn method_03(&mut self) {
    }

    /// Decompiled virtual method #4 @ RVA 0x0019e550 (2 instructions, 1 basic blocks)
    pub fn method_04(&mut self) {
        // Writes to field at offset +0x00c4
    }

    /// Decompiled virtual method #5 @ RVA 0x004ebec0 (133 instructions, 35 basic blocks)
    pub fn method_05(&mut self) {
        // Writes to field at offset +0x0009
        // Writes to field at offset +0x0024
    }

    /// Decompiled virtual method #6 @ RVA 0x004ebea0 (4 instructions, 2 basic blocks)
    pub fn method_06(&mut self) {
        // Writes to field at offset +0x01c8
        // Writes to field at offset +0x01cc
    }

    /// Decompiled virtual method #7 @ RVA 0x000d3090 (1 instructions, 1 basic blocks)
    pub fn method_07(&mut self) {
    }

    /// Decompiled virtual method #8 @ RVA 0x000d3090 (1 instructions, 1 basic blocks)
    pub fn method_08(&mut self) {
    }

    /// Decompiled virtual method #9 @ RVA 0x000d3090 (1 instructions, 1 basic blocks)
    pub fn method_09(&mut self) {
    }

    /// Decompiled virtual method #10 @ RVA 0x004ebc90 (102 instructions, 9 basic blocks)
    pub fn method_10(&mut self) {
        // Writes to field at offset +0x0090
        // Writes to field at offset +0x0094
        // Writes to field at offset +0x0098
        // Writes to field at offset +0x009c
        // Writes to field at offset +0x00a0
        // Writes to field at offset +0x00a4
        // Writes to field at offset +0x00a8
        // Writes to field at offset +0x00ac
    }

    /// Decompiled virtual method #11 @ RVA 0x0045e2d0 (33 instructions, 3 basic blocks)
    pub fn method_11(&mut self) {
        // Writes to field at offset +0x0090
        // Writes to field at offset +0x00a0
        // Writes to field at offset +0x00a4
        // Writes to field at offset +0x00a8
        // Writes to field at offset +0x00ac
    }

    /// Decompiled virtual method #12 @ RVA 0x0045eec0 (67 instructions, 22 basic blocks)
    pub fn method_12(&mut self) {
        // Writes to field at offset +0x001c
        // Writes to field at offset +0x0040
        // Writes to field at offset +0x0090
        // Writes to field at offset +0x00b0
    }

    /// Decompiled virtual method #13 @ RVA 0x0045efd0 (27 instructions, 5 basic blocks)
    pub fn method_13(&mut self) {
        // Writes to field at offset +0x0090
        // Writes to field at offset +0x00b0
    }

}

impl CcClass for ccGobjGimWPStart {
    fn class_name(&self) -> &'static str { Self::CLASS_NAME }
    fn vtable_rva(&self) -> u32 { Self::VTABLE_RVA }
    fn virtual_method_count(&self) -> usize { Self::METHOD_COUNT }
    fn base_classes(&self) -> &'static [&'static str] { Self::BASE_CLASSES }
}

/// Decompiled CC2 class: `ccGobjGimWarpPoint` (VTable RVA: 0x008be600, 14 virtual methods, 37 recovered fields)
/// Inherits: ccGobjGimBase, ccGameObj
#[derive(Debug, Clone, Default)]
pub struct ccGobjGimWarpPoint {
    pub vtable_rva: u32,
    pub field_0x0000: u64,
    pub field_0x0010: u64,
    pub field_0x0018: u64,
    pub field_0x001c: u8,
    pub field_0x0020: u32,
    pub field_0x0024: u16,
    pub field_0x002c: u16,
    pub field_0x0040: u32,
    pub field_0x0090: u32,
    pub field_0x0094: u32,
    pub field_0x0098: u32,
    pub field_0x009c: u32,
    pub field_0x00a0: u32,
    pub field_0x00a4: u32,
    pub field_0x00a8: u32,
    pub field_0x00ac: u32,
    pub field_0x00b0: u64,
    pub field_0x00b8: u64,
    pub field_0x00c4: u32,
    pub field_0x0120: u32,
    pub field_0x0124: u32,
    pub field_0x0128: u32,
    pub field_0x012c: u32,
    pub field_0x0130: u32,
    pub field_0x0134: u32,
    pub field_0x0138: u32,
    pub field_0x013c: u32,
    pub field_0x0140: u32,
    pub field_0x0144: u32,
    pub field_0x0148: u32,
    pub field_0x014c: u32,
    pub field_0x0150: u32,
    pub field_0x0154: u32,
    pub field_0x0158: u32,
    pub field_0x015c: u32,
    pub field_0x01d0: u16,
    pub field_0x01d4: u32,
}

impl ccGobjGimWarpPoint {
    pub const CLASS_NAME: &'static str = "ccGobjGimWarpPoint";
    pub const VTABLE_RVA: u32 = 0x008be600;
    pub const METHOD_COUNT: usize = 14;
    pub const BASE_CLASSES: &'static [&'static str] = &["ccGobjGimBase", "ccGameObj"];
    pub const VIRTUAL_METHODS: &'static [u64] = &[0x00000001804e9fc0, 0x00000001804ea0b0, 0x000000018045e4c0, 0x00000001800d3090, 0x000000018019e550, 0x00000001804ea940, 0x00000001804ea910, 0x00000001800d3090, 0x00000001800d3090, 0x00000001800d3090, 0x00000001804ea700, 0x000000018045e2d0, 0x000000018045eec0, 0x000000018045efd0];

    pub fn new() -> Self {
        Self {
            vtable_rva: Self::VTABLE_RVA,
            field_0x0000: Default::default(),
            field_0x0010: Default::default(),
            field_0x0018: Default::default(),
            field_0x001c: Default::default(),
            field_0x0020: Default::default(),
            field_0x0024: Default::default(),
            field_0x002c: Default::default(),
            field_0x0040: Default::default(),
            field_0x0090: Default::default(),
            field_0x0094: Default::default(),
            field_0x0098: Default::default(),
            field_0x009c: Default::default(),
            field_0x00a0: Default::default(),
            field_0x00a4: Default::default(),
            field_0x00a8: Default::default(),
            field_0x00ac: Default::default(),
            field_0x00b0: Default::default(),
            field_0x00b8: Default::default(),
            field_0x00c4: Default::default(),
            field_0x0120: Default::default(),
            field_0x0124: Default::default(),
            field_0x0128: Default::default(),
            field_0x012c: Default::default(),
            field_0x0130: Default::default(),
            field_0x0134: Default::default(),
            field_0x0138: Default::default(),
            field_0x013c: Default::default(),
            field_0x0140: Default::default(),
            field_0x0144: Default::default(),
            field_0x0148: Default::default(),
            field_0x014c: Default::default(),
            field_0x0150: Default::default(),
            field_0x0154: Default::default(),
            field_0x0158: Default::default(),
            field_0x015c: Default::default(),
            field_0x01d0: Default::default(),
            field_0x01d4: Default::default(),
        }
    }

    /// Decompiled virtual method #0 @ RVA 0x004e9fc0 (16 instructions, 3 basic blocks)
    pub fn method_00(&mut self) {
    }

    /// Decompiled virtual method #1 @ RVA 0x004ea0b0 (13 instructions, 3 basic blocks)
    pub fn method_01(&mut self) {
    }

    /// Decompiled virtual method #2 @ RVA 0x0045e4c0 (42 instructions, 4 basic blocks)
    pub fn method_02(&mut self) {
        // Writes to field at offset +0x0090
        // Writes to field at offset +0x0094
        // Writes to field at offset +0x0098
        // Writes to field at offset +0x009c
        // Writes to field at offset +0x00a0
        // Writes to field at offset +0x00a4
        // Writes to field at offset +0x00a8
        // Writes to field at offset +0x00ac
        // Writes to field at offset +0x0140
        // Writes to field at offset +0x0144
        // Writes to field at offset +0x0148
        // Writes to field at offset +0x014c
        // Writes to field at offset +0x0150
        // Writes to field at offset +0x0154
        // Writes to field at offset +0x0158
        // Writes to field at offset +0x015c
    }

    /// Decompiled virtual method #3 @ RVA 0x000d3090 (1 instructions, 1 basic blocks)
    pub fn method_03(&mut self) {
    }

    /// Decompiled virtual method #4 @ RVA 0x0019e550 (2 instructions, 1 basic blocks)
    pub fn method_04(&mut self) {
        // Writes to field at offset +0x00c4
    }

    /// Decompiled virtual method #5 @ RVA 0x004ea940 (127 instructions, 35 basic blocks)
    pub fn method_05(&mut self) {
        // Writes to field at offset +0x0000
        // Writes to field at offset +0x0024
    }

    /// Decompiled virtual method #6 @ RVA 0x004ea910 (11 instructions, 6 basic blocks)
    pub fn method_06(&mut self) {
        // Writes to field at offset +0x01d4
    }

    /// Decompiled virtual method #7 @ RVA 0x000d3090 (1 instructions, 1 basic blocks)
    pub fn method_07(&mut self) {
    }

    /// Decompiled virtual method #8 @ RVA 0x000d3090 (1 instructions, 1 basic blocks)
    pub fn method_08(&mut self) {
    }

    /// Decompiled virtual method #9 @ RVA 0x000d3090 (1 instructions, 1 basic blocks)
    pub fn method_09(&mut self) {
    }

    /// Decompiled virtual method #10 @ RVA 0x004ea700 (102 instructions, 9 basic blocks)
    pub fn method_10(&mut self) {
        // Writes to field at offset +0x0090
        // Writes to field at offset +0x0094
        // Writes to field at offset +0x0098
        // Writes to field at offset +0x009c
        // Writes to field at offset +0x00a0
        // Writes to field at offset +0x00a4
        // Writes to field at offset +0x00a8
        // Writes to field at offset +0x00ac
    }

    /// Decompiled virtual method #11 @ RVA 0x0045e2d0 (33 instructions, 3 basic blocks)
    pub fn method_11(&mut self) {
        // Writes to field at offset +0x0090
        // Writes to field at offset +0x00a0
        // Writes to field at offset +0x00a4
        // Writes to field at offset +0x00a8
        // Writes to field at offset +0x00ac
    }

    /// Decompiled virtual method #12 @ RVA 0x0045eec0 (67 instructions, 22 basic blocks)
    pub fn method_12(&mut self) {
        // Writes to field at offset +0x001c
        // Writes to field at offset +0x0040
        // Writes to field at offset +0x0090
        // Writes to field at offset +0x00b0
    }

    /// Decompiled virtual method #13 @ RVA 0x0045efd0 (27 instructions, 5 basic blocks)
    pub fn method_13(&mut self) {
        // Writes to field at offset +0x0090
        // Writes to field at offset +0x00b0
    }

}

impl CcClass for ccGobjGimWarpPoint {
    fn class_name(&self) -> &'static str { Self::CLASS_NAME }
    fn vtable_rva(&self) -> u32 { Self::VTABLE_RVA }
    fn virtual_method_count(&self) -> usize { Self::METHOD_COUNT }
    fn base_classes(&self) -> &'static [&'static str] { Self::BASE_CLASSES }
}

/// Decompiled CC2 class: `ccGobjGimWell` (VTable RVA: 0x008bea38, 14 virtual methods, 34 recovered fields)
/// Inherits: ccGobjGimBase, ccGameObj
#[derive(Debug, Clone, Default)]
pub struct ccGobjGimWell {
    pub vtable_rva: u32,
    pub field_0x0010: u64,
    pub field_0x0018: u64,
    pub field_0x001c: u8,
    pub field_0x0020: u32,
    pub field_0x0024: u16,
    pub field_0x002c: u16,
    pub field_0x0040: u32,
    pub field_0x0090: u32,
    pub field_0x0094: u32,
    pub field_0x0098: u32,
    pub field_0x009c: u32,
    pub field_0x00a0: u32,
    pub field_0x00a4: u32,
    pub field_0x00a8: u32,
    pub field_0x00ac: u32,
    pub field_0x00b0: u64,
    pub field_0x00b8: u64,
    pub field_0x00c4: u32,
    pub field_0x0120: u32,
    pub field_0x0124: u32,
    pub field_0x0128: u32,
    pub field_0x012c: u32,
    pub field_0x0130: u32,
    pub field_0x0134: u32,
    pub field_0x0138: u32,
    pub field_0x013c: u32,
    pub field_0x0140: u32,
    pub field_0x0144: u32,
    pub field_0x0148: u32,
    pub field_0x014c: u32,
    pub field_0x0150: u32,
    pub field_0x0154: u32,
    pub field_0x0158: u32,
    pub field_0x015c: u32,
}

impl ccGobjGimWell {
    pub const CLASS_NAME: &'static str = "ccGobjGimWell";
    pub const VTABLE_RVA: u32 = 0x008bea38;
    pub const METHOD_COUNT: usize = 14;
    pub const BASE_CLASSES: &'static [&'static str] = &["ccGobjGimBase", "ccGameObj"];
    pub const VIRTUAL_METHODS: &'static [u64] = &[0x00000001804c8e90, 0x00000001804d9230, 0x000000018045e4c0, 0x00000001800d3090, 0x000000018019e550, 0x00000001804d93b0, 0x00000001800d3090, 0x00000001800d3090, 0x00000001800d3090, 0x00000001800d3090, 0x00000001800d3090, 0x000000018045e2d0, 0x000000018045eec0, 0x000000018045efd0];

    pub fn new() -> Self {
        Self {
            vtable_rva: Self::VTABLE_RVA,
            field_0x0010: Default::default(),
            field_0x0018: Default::default(),
            field_0x001c: Default::default(),
            field_0x0020: Default::default(),
            field_0x0024: Default::default(),
            field_0x002c: Default::default(),
            field_0x0040: Default::default(),
            field_0x0090: Default::default(),
            field_0x0094: Default::default(),
            field_0x0098: Default::default(),
            field_0x009c: Default::default(),
            field_0x00a0: Default::default(),
            field_0x00a4: Default::default(),
            field_0x00a8: Default::default(),
            field_0x00ac: Default::default(),
            field_0x00b0: Default::default(),
            field_0x00b8: Default::default(),
            field_0x00c4: Default::default(),
            field_0x0120: Default::default(),
            field_0x0124: Default::default(),
            field_0x0128: Default::default(),
            field_0x012c: Default::default(),
            field_0x0130: Default::default(),
            field_0x0134: Default::default(),
            field_0x0138: Default::default(),
            field_0x013c: Default::default(),
            field_0x0140: Default::default(),
            field_0x0144: Default::default(),
            field_0x0148: Default::default(),
            field_0x014c: Default::default(),
            field_0x0150: Default::default(),
            field_0x0154: Default::default(),
            field_0x0158: Default::default(),
            field_0x015c: Default::default(),
        }
    }

    /// Decompiled virtual method #0 @ RVA 0x004c8e90 (16 instructions, 3 basic blocks)
    pub fn method_00(&mut self) {
    }

    /// Decompiled virtual method #1 @ RVA 0x004d9230 (13 instructions, 3 basic blocks)
    pub fn method_01(&mut self) {
    }

    /// Decompiled virtual method #2 @ RVA 0x0045e4c0 (42 instructions, 4 basic blocks)
    pub fn method_02(&mut self) {
        // Writes to field at offset +0x0090
        // Writes to field at offset +0x0094
        // Writes to field at offset +0x0098
        // Writes to field at offset +0x009c
        // Writes to field at offset +0x00a0
        // Writes to field at offset +0x00a4
        // Writes to field at offset +0x00a8
        // Writes to field at offset +0x00ac
        // Writes to field at offset +0x0140
        // Writes to field at offset +0x0144
        // Writes to field at offset +0x0148
        // Writes to field at offset +0x014c
        // Writes to field at offset +0x0150
        // Writes to field at offset +0x0154
        // Writes to field at offset +0x0158
        // Writes to field at offset +0x015c
    }

    /// Decompiled virtual method #3 @ RVA 0x000d3090 (1 instructions, 1 basic blocks)
    pub fn method_03(&mut self) {
    }

    /// Decompiled virtual method #4 @ RVA 0x0019e550 (2 instructions, 1 basic blocks)
    pub fn method_04(&mut self) {
        // Writes to field at offset +0x00c4
    }

    /// Decompiled virtual method #5 @ RVA 0x004d93b0 (105 instructions, 16 basic blocks)
    pub fn method_05(&mut self) {
        // Writes to field at offset +0x0024
    }

    /// Decompiled virtual method #6 @ RVA 0x000d3090 (1 instructions, 1 basic blocks)
    pub fn method_06(&mut self) {
    }

    /// Decompiled virtual method #7 @ RVA 0x000d3090 (1 instructions, 1 basic blocks)
    pub fn method_07(&mut self) {
    }

    /// Decompiled virtual method #8 @ RVA 0x000d3090 (1 instructions, 1 basic blocks)
    pub fn method_08(&mut self) {
    }

    /// Decompiled virtual method #9 @ RVA 0x000d3090 (1 instructions, 1 basic blocks)
    pub fn method_09(&mut self) {
    }

    /// Decompiled virtual method #10 @ RVA 0x000d3090 (1 instructions, 1 basic blocks)
    pub fn method_10(&mut self) {
    }

    /// Decompiled virtual method #11 @ RVA 0x0045e2d0 (33 instructions, 3 basic blocks)
    pub fn method_11(&mut self) {
        // Writes to field at offset +0x0090
        // Writes to field at offset +0x00a0
        // Writes to field at offset +0x00a4
        // Writes to field at offset +0x00a8
        // Writes to field at offset +0x00ac
    }

    /// Decompiled virtual method #12 @ RVA 0x0045eec0 (67 instructions, 22 basic blocks)
    pub fn method_12(&mut self) {
        // Writes to field at offset +0x001c
        // Writes to field at offset +0x0040
        // Writes to field at offset +0x0090
        // Writes to field at offset +0x00b0
    }

    /// Decompiled virtual method #13 @ RVA 0x0045efd0 (27 instructions, 5 basic blocks)
    pub fn method_13(&mut self) {
        // Writes to field at offset +0x0090
        // Writes to field at offset +0x00b0
    }

}

impl CcClass for ccGobjGimWell {
    fn class_name(&self) -> &'static str { Self::CLASS_NAME }
    fn vtable_rva(&self) -> u32 { Self::VTABLE_RVA }
    fn virtual_method_count(&self) -> usize { Self::METHOD_COUNT }
    fn base_classes(&self) -> &'static [&'static str] { Self::BASE_CLASSES }
}

/// Decompiled CC2 class: `ccGobjIslandNpc` (VTable RVA: 0x008c02b0, 14 virtual methods, 44 recovered fields)
/// Inherits: ccGobjNpc, ccGameObj
#[derive(Debug, Clone, Default)]
pub struct ccGobjIslandNpc {
    pub vtable_rva: u32,
    pub field_0x0000: u64,
    pub field_0x0001: u32,
    pub field_0x0004: u32,
    pub field_0x0008: u16,
    pub field_0x000a: u16,
    pub field_0x0010: u64,
    pub field_0x0018: u64,
    pub field_0x001c: u8,
    pub field_0x0020: u32,
    pub field_0x0024: u16,
    pub field_0x002c: u16,
    pub field_0x0034: u32,
    pub field_0x0040: u32,
    pub field_0x0060: u64,
    pub field_0x0090: u32,
    pub field_0x0094: u32,
    pub field_0x0095: u8,
    pub field_0x0098: u32,
    pub field_0x009c: u32,
    pub field_0x00a0: u32,
    pub field_0x00a4: u32,
    pub field_0x00a8: u32,
    pub field_0x00ac: u32,
    pub field_0x00b0: u64,
    pub field_0x00b8: u64,
    pub field_0x00c4: u32,
    pub field_0x00e1: u8,
    pub field_0x0120: u32,
    pub field_0x0124: u32,
    pub field_0x0128: u32,
    pub field_0x012c: u32,
    pub field_0x0130: u32,
    pub field_0x0134: u32,
    pub field_0x0138: u32,
    pub field_0x013c: u32,
    pub field_0x0140: u32,
    pub field_0x0144: u32,
    pub field_0x0148: u32,
    pub field_0x014c: u32,
    pub field_0x0150: u32,
    pub field_0x0154: u32,
    pub field_0x0158: u32,
    pub field_0x015c: u32,
    pub field_0x0204: u8,
}

impl ccGobjIslandNpc {
    pub const CLASS_NAME: &'static str = "ccGobjIslandNpc";
    pub const VTABLE_RVA: u32 = 0x008c02b0;
    pub const METHOD_COUNT: usize = 14;
    pub const BASE_CLASSES: &'static [&'static str] = &["ccGobjNpc", "ccGameObj"];
    pub const VIRTUAL_METHODS: &'static [u64] = &[0x00000001805027c0, 0x00000001804ff880, 0x000000018045e4c0, 0x00000001800d3090, 0x000000018019e550, 0x0000000180500090, 0x00000001800d3090, 0x00000001800d3090, 0x00000001800d3090, 0x00000001805004e0, 0x0000000180500250, 0x000000018045e2d0, 0x000000018045eec0, 0x000000018045efd0];

    pub fn new() -> Self {
        Self {
            vtable_rva: Self::VTABLE_RVA,
            field_0x0000: Default::default(),
            field_0x0001: Default::default(),
            field_0x0004: Default::default(),
            field_0x0008: Default::default(),
            field_0x000a: Default::default(),
            field_0x0010: Default::default(),
            field_0x0018: Default::default(),
            field_0x001c: Default::default(),
            field_0x0020: Default::default(),
            field_0x0024: Default::default(),
            field_0x002c: Default::default(),
            field_0x0034: Default::default(),
            field_0x0040: Default::default(),
            field_0x0060: Default::default(),
            field_0x0090: Default::default(),
            field_0x0094: Default::default(),
            field_0x0095: Default::default(),
            field_0x0098: Default::default(),
            field_0x009c: Default::default(),
            field_0x00a0: Default::default(),
            field_0x00a4: Default::default(),
            field_0x00a8: Default::default(),
            field_0x00ac: Default::default(),
            field_0x00b0: Default::default(),
            field_0x00b8: Default::default(),
            field_0x00c4: Default::default(),
            field_0x00e1: Default::default(),
            field_0x0120: Default::default(),
            field_0x0124: Default::default(),
            field_0x0128: Default::default(),
            field_0x012c: Default::default(),
            field_0x0130: Default::default(),
            field_0x0134: Default::default(),
            field_0x0138: Default::default(),
            field_0x013c: Default::default(),
            field_0x0140: Default::default(),
            field_0x0144: Default::default(),
            field_0x0148: Default::default(),
            field_0x014c: Default::default(),
            field_0x0150: Default::default(),
            field_0x0154: Default::default(),
            field_0x0158: Default::default(),
            field_0x015c: Default::default(),
            field_0x0204: Default::default(),
        }
    }

    /// Decompiled virtual method #0 @ RVA 0x005027c0 (16 instructions, 3 basic blocks)
    pub fn method_00(&mut self) {
    }

    /// Decompiled virtual method #1 @ RVA 0x004ff880 (111 instructions, 25 basic blocks)
    pub fn method_01(&mut self) {
        // Writes to field at offset +0x0060
    }

    /// Decompiled virtual method #2 @ RVA 0x0045e4c0 (42 instructions, 4 basic blocks)
    pub fn method_02(&mut self) {
        // Writes to field at offset +0x0090
        // Writes to field at offset +0x0094
        // Writes to field at offset +0x0098
        // Writes to field at offset +0x009c
        // Writes to field at offset +0x00a0
        // Writes to field at offset +0x00a4
        // Writes to field at offset +0x00a8
        // Writes to field at offset +0x00ac
        // Writes to field at offset +0x0140
        // Writes to field at offset +0x0144
        // Writes to field at offset +0x0148
        // Writes to field at offset +0x014c
        // Writes to field at offset +0x0150
        // Writes to field at offset +0x0154
        // Writes to field at offset +0x0158
        // Writes to field at offset +0x015c
    }

    /// Decompiled virtual method #3 @ RVA 0x000d3090 (1 instructions, 1 basic blocks)
    pub fn method_03(&mut self) {
    }

    /// Decompiled virtual method #4 @ RVA 0x0019e550 (2 instructions, 1 basic blocks)
    pub fn method_04(&mut self) {
        // Writes to field at offset +0x00c4
    }

    /// Decompiled virtual method #5 @ RVA 0x00500090 (128 instructions, 36 basic blocks)
    pub fn method_05(&mut self) {
        // Writes to field at offset +0x0010
        // Writes to field at offset +0x0018
        // Writes to field at offset +0x0040
        // Writes to field at offset +0x0095
        // Writes to field at offset +0x00e1
        // Writes to field at offset +0x0204
    }

    /// Decompiled virtual method #6 @ RVA 0x000d3090 (1 instructions, 1 basic blocks)
    pub fn method_06(&mut self) {
    }

    /// Decompiled virtual method #7 @ RVA 0x000d3090 (1 instructions, 1 basic blocks)
    pub fn method_07(&mut self) {
    }

    /// Decompiled virtual method #8 @ RVA 0x000d3090 (1 instructions, 1 basic blocks)
    pub fn method_08(&mut self) {
    }

    /// Decompiled virtual method #9 @ RVA 0x005004e0 (106 instructions, 17 basic blocks)
    pub fn method_09(&mut self) {
        // Writes to field at offset +0x0000
        // Writes to field at offset +0x0004
        // Writes to field at offset +0x0008
        // Writes to field at offset +0x000a
    }

    /// Decompiled virtual method #10 @ RVA 0x00500250 (17 instructions, 6 basic blocks)
    pub fn method_10(&mut self) {
    }

    /// Decompiled virtual method #11 @ RVA 0x0045e2d0 (33 instructions, 3 basic blocks)
    pub fn method_11(&mut self) {
        // Writes to field at offset +0x0090
        // Writes to field at offset +0x00a0
        // Writes to field at offset +0x00a4
        // Writes to field at offset +0x00a8
        // Writes to field at offset +0x00ac
    }

    /// Decompiled virtual method #12 @ RVA 0x0045eec0 (67 instructions, 22 basic blocks)
    pub fn method_12(&mut self) {
        // Writes to field at offset +0x001c
        // Writes to field at offset +0x0040
        // Writes to field at offset +0x0090
        // Writes to field at offset +0x00b0
    }

    /// Decompiled virtual method #13 @ RVA 0x0045efd0 (27 instructions, 5 basic blocks)
    pub fn method_13(&mut self) {
        // Writes to field at offset +0x0090
        // Writes to field at offset +0x00b0
    }

}

impl CcClass for ccGobjIslandNpc {
    fn class_name(&self) -> &'static str { Self::CLASS_NAME }
    fn vtable_rva(&self) -> u32 { Self::VTABLE_RVA }
    fn virtual_method_count(&self) -> usize { Self::METHOD_COUNT }
    fn base_classes(&self) -> &'static [&'static str] { Self::BASE_CLASSES }
}

/// Decompiled CC2 class: `ccGobjKingChim` (VTable RVA: 0x008bc908, 17 virtual methods, 44 recovered fields)
/// Inherits: ccGobjChimBase, ccGameObj
#[derive(Debug, Clone, Default)]
pub struct ccGobjKingChim {
    pub vtable_rva: u32,
    pub field_0x0000: u64,
    pub field_0x0009: u8,
    pub field_0x0010: u64,
    pub field_0x0018: u64,
    pub field_0x001c: u8,
    pub field_0x0020: u32,
    pub field_0x0024: u16,
    pub field_0x002c: u16,
    pub field_0x0040: u32,
    pub field_0x0058: u64,
    pub field_0x0068: u64,
    pub field_0x0070: u32,
    pub field_0x0090: u32,
    pub field_0x0094: u32,
    pub field_0x0098: u32,
    pub field_0x009c: u32,
    pub field_0x00a0: u32,
    pub field_0x00a4: u32,
    pub field_0x00a8: u32,
    pub field_0x00ac: u32,
    pub field_0x00b0: u64,
    pub field_0x00b8: u64,
    pub field_0x00c4: u32,
    pub field_0x00d0: u32,
    pub field_0x00e8: f32,
    pub field_0x0114: u32,
    pub field_0x0120: u32,
    pub field_0x0124: u32,
    pub field_0x0128: u32,
    pub field_0x012c: u32,
    pub field_0x0130: u32,
    pub field_0x0134: u32,
    pub field_0x0138: u32,
    pub field_0x013c: u32,
    pub field_0x0140: u32,
    pub field_0x0144: u32,
    pub field_0x0148: u32,
    pub field_0x014c: u32,
    pub field_0x0150: u32,
    pub field_0x0154: u32,
    pub field_0x0158: u32,
    pub field_0x015c: u32,
    pub field_0x01b8: u64,
    pub field_0x01c8: u64,
}

impl ccGobjKingChim {
    pub const CLASS_NAME: &'static str = "ccGobjKingChim";
    pub const VTABLE_RVA: u32 = 0x008bc908;
    pub const METHOD_COUNT: usize = 17;
    pub const BASE_CLASSES: &'static [&'static str] = &["ccGobjChimBase", "ccGameObj"];
    pub const VIRTUAL_METHODS: &'static [u64] = &[0x00000001804a4470, 0x00000001804aa3b0, 0x000000018045e4c0, 0x00000001800d3090, 0x000000018019e550, 0x00000001804ab710, 0x00000001804a6310, 0x00000001800d3090, 0x00000001800d3090, 0x00000001804a68f0, 0x00000001804aaf40, 0x000000018045e2d0, 0x000000018045eec0, 0x000000018045efd0, 0x00000001804abf50, 0x00000001804ac690, 0x00000001804aa0a0];

    pub fn new() -> Self {
        Self {
            vtable_rva: Self::VTABLE_RVA,
            field_0x0000: Default::default(),
            field_0x0009: Default::default(),
            field_0x0010: Default::default(),
            field_0x0018: Default::default(),
            field_0x001c: Default::default(),
            field_0x0020: Default::default(),
            field_0x0024: Default::default(),
            field_0x002c: Default::default(),
            field_0x0040: Default::default(),
            field_0x0058: Default::default(),
            field_0x0068: Default::default(),
            field_0x0070: Default::default(),
            field_0x0090: Default::default(),
            field_0x0094: Default::default(),
            field_0x0098: Default::default(),
            field_0x009c: Default::default(),
            field_0x00a0: Default::default(),
            field_0x00a4: Default::default(),
            field_0x00a8: Default::default(),
            field_0x00ac: Default::default(),
            field_0x00b0: Default::default(),
            field_0x00b8: Default::default(),
            field_0x00c4: Default::default(),
            field_0x00d0: Default::default(),
            field_0x00e8: Default::default(),
            field_0x0114: Default::default(),
            field_0x0120: Default::default(),
            field_0x0124: Default::default(),
            field_0x0128: Default::default(),
            field_0x012c: Default::default(),
            field_0x0130: Default::default(),
            field_0x0134: Default::default(),
            field_0x0138: Default::default(),
            field_0x013c: Default::default(),
            field_0x0140: Default::default(),
            field_0x0144: Default::default(),
            field_0x0148: Default::default(),
            field_0x014c: Default::default(),
            field_0x0150: Default::default(),
            field_0x0154: Default::default(),
            field_0x0158: Default::default(),
            field_0x015c: Default::default(),
            field_0x01b8: Default::default(),
            field_0x01c8: Default::default(),
        }
    }

    /// Decompiled virtual method #0 @ RVA 0x004a4470 (24 instructions, 3 basic blocks)
    pub fn method_00(&mut self) {
        // Writes to field at offset +0x0000
    }

    /// Decompiled virtual method #1 @ RVA 0x004aa3b0 (107 instructions, 18 basic blocks)
    pub fn method_01(&mut self) {
        // Writes to field at offset +0x0070
    }

    /// Decompiled virtual method #2 @ RVA 0x0045e4c0 (42 instructions, 4 basic blocks)
    pub fn method_02(&mut self) {
        // Writes to field at offset +0x0090
        // Writes to field at offset +0x0094
        // Writes to field at offset +0x0098
        // Writes to field at offset +0x009c
        // Writes to field at offset +0x00a0
        // Writes to field at offset +0x00a4
        // Writes to field at offset +0x00a8
        // Writes to field at offset +0x00ac
        // Writes to field at offset +0x0140
        // Writes to field at offset +0x0144
        // Writes to field at offset +0x0148
        // Writes to field at offset +0x014c
        // Writes to field at offset +0x0150
        // Writes to field at offset +0x0154
        // Writes to field at offset +0x0158
        // Writes to field at offset +0x015c
    }

    /// Decompiled virtual method #3 @ RVA 0x000d3090 (1 instructions, 1 basic blocks)
    pub fn method_03(&mut self) {
    }

    /// Decompiled virtual method #4 @ RVA 0x0019e550 (2 instructions, 1 basic blocks)
    pub fn method_04(&mut self) {
        // Writes to field at offset +0x00c4
    }

    /// Decompiled virtual method #5 @ RVA 0x004ab710 (102 instructions, 26 basic blocks)
    pub fn method_05(&mut self) {
        // Writes to field at offset +0x0040
    }

    /// Decompiled virtual method #6 @ RVA 0x004a6310 (103 instructions, 17 basic blocks)
    pub fn method_06(&mut self) {
        // Writes to field at offset +0x0009
    }

    /// Decompiled virtual method #7 @ RVA 0x000d3090 (1 instructions, 1 basic blocks)
    pub fn method_07(&mut self) {
    }

    /// Decompiled virtual method #8 @ RVA 0x000d3090 (1 instructions, 1 basic blocks)
    pub fn method_08(&mut self) {
    }

    /// Decompiled virtual method #9 @ RVA 0x004a68f0 (98 instructions, 11 basic blocks)
    pub fn method_09(&mut self) {
        // Writes to field at offset +0x0009
        // Writes to field at offset +0x00a4
        // Writes to field at offset +0x00d0
    }

    /// Decompiled virtual method #10 @ RVA 0x004aaf40 (89 instructions, 17 basic blocks)
    pub fn method_10(&mut self) {
        // Writes to field at offset +0x002c
        // Writes to field at offset +0x0040
    }

    /// Decompiled virtual method #11 @ RVA 0x0045e2d0 (33 instructions, 3 basic blocks)
    pub fn method_11(&mut self) {
        // Writes to field at offset +0x0090
        // Writes to field at offset +0x00a0
        // Writes to field at offset +0x00a4
        // Writes to field at offset +0x00a8
        // Writes to field at offset +0x00ac
    }

    /// Decompiled virtual method #12 @ RVA 0x0045eec0 (67 instructions, 22 basic blocks)
    pub fn method_12(&mut self) {
        // Writes to field at offset +0x001c
        // Writes to field at offset +0x0040
        // Writes to field at offset +0x0090
        // Writes to field at offset +0x00b0
    }

    /// Decompiled virtual method #13 @ RVA 0x0045efd0 (27 instructions, 5 basic blocks)
    pub fn method_13(&mut self) {
        // Writes to field at offset +0x0090
        // Writes to field at offset +0x00b0
    }

    /// Decompiled virtual method #14 @ RVA 0x004abf50 (89 instructions, 10 basic blocks)
    pub fn method_14(&mut self) {
    }

    /// Decompiled virtual method #15 @ RVA 0x004ac690 (8 instructions, 1 basic blocks)
    pub fn method_15(&mut self) {
        // Writes to field at offset +0x0114
    }

    /// Decompiled virtual method #16 @ RVA 0x004aa0a0 (128 instructions, 20 basic blocks)
    pub fn method_16(&mut self) {
    }

}

impl CcClass for ccGobjKingChim {
    fn class_name(&self) -> &'static str { Self::CLASS_NAME }
    fn vtable_rva(&self) -> u32 { Self::VTABLE_RVA }
    fn virtual_method_count(&self) -> usize { Self::METHOD_COUNT }
    fn base_classes(&self) -> &'static [&'static str] { Self::BASE_CLASSES }
}

/// Decompiled CC2 class: `ccGobjNpc` (VTable RVA: 0x008c0328, 14 virtual methods, 41 recovered fields)
/// Inherits: ccGameObj
#[derive(Debug, Clone, Default)]
pub struct ccGobjNpc {
    pub vtable_rva: u32,
    pub field_0x0000: u64,
    pub field_0x0004: u32,
    pub field_0x0008: u16,
    pub field_0x000a: u16,
    pub field_0x0010: u64,
    pub field_0x0018: u64,
    pub field_0x001c: u8,
    pub field_0x0020: u32,
    pub field_0x0024: u16,
    pub field_0x002c: u16,
    pub field_0x0034: u32,
    pub field_0x0040: u32,
    pub field_0x0060: u64,
    pub field_0x0070: u32,
    pub field_0x0090: u32,
    pub field_0x0094: u32,
    pub field_0x0098: u32,
    pub field_0x009c: u32,
    pub field_0x00a0: u32,
    pub field_0x00a4: u32,
    pub field_0x00a8: u32,
    pub field_0x00ac: u32,
    pub field_0x00b0: u64,
    pub field_0x00b8: u64,
    pub field_0x00c4: u32,
    pub field_0x0120: u32,
    pub field_0x0124: u32,
    pub field_0x0128: u32,
    pub field_0x012c: u32,
    pub field_0x0130: u32,
    pub field_0x0134: u32,
    pub field_0x0138: u32,
    pub field_0x013c: u32,
    pub field_0x0140: u32,
    pub field_0x0144: u32,
    pub field_0x0148: u32,
    pub field_0x014c: u32,
    pub field_0x0150: u32,
    pub field_0x0154: u32,
    pub field_0x0158: u32,
    pub field_0x015c: u32,
}

impl ccGobjNpc {
    pub const CLASS_NAME: &'static str = "ccGobjNpc";
    pub const VTABLE_RVA: u32 = 0x008c0328;
    pub const METHOD_COUNT: usize = 14;
    pub const BASE_CLASSES: &'static [&'static str] = &["ccGameObj"];
    pub const VIRTUAL_METHODS: &'static [u64] = &[0x00000001804fea30, 0x00000001804ff880, 0x000000018045e4c0, 0x00000001800d3090, 0x000000018019e550, 0x00000001804ffeb0, 0x00000001800d3090, 0x00000001800d3090, 0x00000001800d3090, 0x00000001804ffca0, 0x000000018045e220, 0x000000018045e2d0, 0x000000018045eec0, 0x000000018045efd0];

    pub fn new() -> Self {
        Self {
            vtable_rva: Self::VTABLE_RVA,
            field_0x0000: Default::default(),
            field_0x0004: Default::default(),
            field_0x0008: Default::default(),
            field_0x000a: Default::default(),
            field_0x0010: Default::default(),
            field_0x0018: Default::default(),
            field_0x001c: Default::default(),
            field_0x0020: Default::default(),
            field_0x0024: Default::default(),
            field_0x002c: Default::default(),
            field_0x0034: Default::default(),
            field_0x0040: Default::default(),
            field_0x0060: Default::default(),
            field_0x0070: Default::default(),
            field_0x0090: Default::default(),
            field_0x0094: Default::default(),
            field_0x0098: Default::default(),
            field_0x009c: Default::default(),
            field_0x00a0: Default::default(),
            field_0x00a4: Default::default(),
            field_0x00a8: Default::default(),
            field_0x00ac: Default::default(),
            field_0x00b0: Default::default(),
            field_0x00b8: Default::default(),
            field_0x00c4: Default::default(),
            field_0x0120: Default::default(),
            field_0x0124: Default::default(),
            field_0x0128: Default::default(),
            field_0x012c: Default::default(),
            field_0x0130: Default::default(),
            field_0x0134: Default::default(),
            field_0x0138: Default::default(),
            field_0x013c: Default::default(),
            field_0x0140: Default::default(),
            field_0x0144: Default::default(),
            field_0x0148: Default::default(),
            field_0x014c: Default::default(),
            field_0x0150: Default::default(),
            field_0x0154: Default::default(),
            field_0x0158: Default::default(),
            field_0x015c: Default::default(),
        }
    }

    /// Decompiled virtual method #0 @ RVA 0x004fea30 (16 instructions, 3 basic blocks)
    pub fn method_00(&mut self) {
    }

    /// Decompiled virtual method #1 @ RVA 0x004ff880 (111 instructions, 25 basic blocks)
    pub fn method_01(&mut self) {
        // Writes to field at offset +0x0060
    }

    /// Decompiled virtual method #2 @ RVA 0x0045e4c0 (42 instructions, 4 basic blocks)
    pub fn method_02(&mut self) {
        // Writes to field at offset +0x0090
        // Writes to field at offset +0x0094
        // Writes to field at offset +0x0098
        // Writes to field at offset +0x009c
        // Writes to field at offset +0x00a0
        // Writes to field at offset +0x00a4
        // Writes to field at offset +0x00a8
        // Writes to field at offset +0x00ac
        // Writes to field at offset +0x0140
        // Writes to field at offset +0x0144
        // Writes to field at offset +0x0148
        // Writes to field at offset +0x014c
        // Writes to field at offset +0x0150
        // Writes to field at offset +0x0154
        // Writes to field at offset +0x0158
        // Writes to field at offset +0x015c
    }

    /// Decompiled virtual method #3 @ RVA 0x000d3090 (1 instructions, 1 basic blocks)
    pub fn method_03(&mut self) {
    }

    /// Decompiled virtual method #4 @ RVA 0x0019e550 (2 instructions, 1 basic blocks)
    pub fn method_04(&mut self) {
        // Writes to field at offset +0x00c4
    }

    /// Decompiled virtual method #5 @ RVA 0x004ffeb0 (153 instructions, 36 basic blocks)
    pub fn method_05(&mut self) {
        // Writes to field at offset +0x0000
        // Writes to field at offset +0x0024
    }

    /// Decompiled virtual method #6 @ RVA 0x000d3090 (1 instructions, 1 basic blocks)
    pub fn method_06(&mut self) {
    }

    /// Decompiled virtual method #7 @ RVA 0x000d3090 (1 instructions, 1 basic blocks)
    pub fn method_07(&mut self) {
    }

    /// Decompiled virtual method #8 @ RVA 0x000d3090 (1 instructions, 1 basic blocks)
    pub fn method_08(&mut self) {
    }

    /// Decompiled virtual method #9 @ RVA 0x004ffca0 (107 instructions, 31 basic blocks)
    pub fn method_09(&mut self) {
        // Writes to field at offset +0x0000
        // Writes to field at offset +0x0004
        // Writes to field at offset +0x0008
        // Writes to field at offset +0x000a
        // Writes to field at offset +0x0070
    }

    /// Decompiled virtual method #10 @ RVA 0x0045e220 (31 instructions, 3 basic blocks)
    pub fn method_10(&mut self) {
        // Writes to field at offset +0x0090
        // Writes to field at offset +0x00a0
        // Writes to field at offset +0x00a4
        // Writes to field at offset +0x00a8
        // Writes to field at offset +0x00ac
    }

    /// Decompiled virtual method #11 @ RVA 0x0045e2d0 (33 instructions, 3 basic blocks)
    pub fn method_11(&mut self) {
        // Writes to field at offset +0x0090
        // Writes to field at offset +0x00a0
        // Writes to field at offset +0x00a4
        // Writes to field at offset +0x00a8
        // Writes to field at offset +0x00ac
    }

    /// Decompiled virtual method #12 @ RVA 0x0045eec0 (67 instructions, 22 basic blocks)
    pub fn method_12(&mut self) {
        // Writes to field at offset +0x001c
        // Writes to field at offset +0x0040
        // Writes to field at offset +0x0090
        // Writes to field at offset +0x00b0
    }

    /// Decompiled virtual method #13 @ RVA 0x0045efd0 (27 instructions, 5 basic blocks)
    pub fn method_13(&mut self) {
        // Writes to field at offset +0x0090
        // Writes to field at offset +0x00b0
    }

}

impl CcClass for ccGobjNpc {
    fn class_name(&self) -> &'static str { Self::CLASS_NAME }
    fn vtable_rva(&self) -> u32 { Self::VTABLE_RVA }
    fn virtual_method_count(&self) -> usize { Self::METHOD_COUNT }
    fn base_classes(&self) -> &'static [&'static str] { Self::BASE_CLASSES }
}

/// Decompiled CC2 class: `ccGobjNpcSpDethLundy` (VTable RVA: 0x008c03a0, 14 virtual methods, 38 recovered fields)
/// Inherits: ccGobjNpc, ccGameObj
#[derive(Debug, Clone, Default)]
pub struct ccGobjNpcSpDethLundy {
    pub vtable_rva: u32,
    pub field_0x0000: u64,
    pub field_0x0010: u64,
    pub field_0x0018: u64,
    pub field_0x001c: u8,
    pub field_0x0020: u32,
    pub field_0x0024: u16,
    pub field_0x002c: u16,
    pub field_0x0040: u32,
    pub field_0x0050: u64,
    pub field_0x0090: u32,
    pub field_0x0094: u32,
    pub field_0x0098: u32,
    pub field_0x009c: u32,
    pub field_0x00a0: u32,
    pub field_0x00a4: u32,
    pub field_0x00a8: u32,
    pub field_0x00ac: u32,
    pub field_0x00b0: u64,
    pub field_0x00b8: u64,
    pub field_0x00c4: u32,
    pub field_0x0120: u32,
    pub field_0x0124: u32,
    pub field_0x0128: u32,
    pub field_0x012c: u32,
    pub field_0x0130: u32,
    pub field_0x0134: u32,
    pub field_0x0138: u32,
    pub field_0x013c: u32,
    pub field_0x0140: u32,
    pub field_0x0144: u32,
    pub field_0x0148: u32,
    pub field_0x014c: u32,
    pub field_0x0150: u32,
    pub field_0x0154: u32,
    pub field_0x0158: u32,
    pub field_0x015c: u32,
    pub field_0x01f0: u32,
    pub field_0x01f8: u32,
}

impl ccGobjNpcSpDethLundy {
    pub const CLASS_NAME: &'static str = "ccGobjNpcSpDethLundy";
    pub const VTABLE_RVA: u32 = 0x008c03a0;
    pub const METHOD_COUNT: usize = 14;
    pub const BASE_CLASSES: &'static [&'static str] = &["ccGobjNpc", "ccGameObj"];
    pub const VIRTUAL_METHODS: &'static [u64] = &[0x00000001804fe700, 0x0000000180500d30, 0x000000018045e4c0, 0x00000001800d3090, 0x000000018019e550, 0x0000000180501170, 0x0000000180501130, 0x00000001800d3090, 0x00000001800d3090, 0x00000001800d3090, 0x0000000180500fe0, 0x000000018045e2d0, 0x000000018045eec0, 0x000000018045efd0];

    pub fn new() -> Self {
        Self {
            vtable_rva: Self::VTABLE_RVA,
            field_0x0000: Default::default(),
            field_0x0010: Default::default(),
            field_0x0018: Default::default(),
            field_0x001c: Default::default(),
            field_0x0020: Default::default(),
            field_0x0024: Default::default(),
            field_0x002c: Default::default(),
            field_0x0040: Default::default(),
            field_0x0050: Default::default(),
            field_0x0090: Default::default(),
            field_0x0094: Default::default(),
            field_0x0098: Default::default(),
            field_0x009c: Default::default(),
            field_0x00a0: Default::default(),
            field_0x00a4: Default::default(),
            field_0x00a8: Default::default(),
            field_0x00ac: Default::default(),
            field_0x00b0: Default::default(),
            field_0x00b8: Default::default(),
            field_0x00c4: Default::default(),
            field_0x0120: Default::default(),
            field_0x0124: Default::default(),
            field_0x0128: Default::default(),
            field_0x012c: Default::default(),
            field_0x0130: Default::default(),
            field_0x0134: Default::default(),
            field_0x0138: Default::default(),
            field_0x013c: Default::default(),
            field_0x0140: Default::default(),
            field_0x0144: Default::default(),
            field_0x0148: Default::default(),
            field_0x014c: Default::default(),
            field_0x0150: Default::default(),
            field_0x0154: Default::default(),
            field_0x0158: Default::default(),
            field_0x015c: Default::default(),
            field_0x01f0: Default::default(),
            field_0x01f8: Default::default(),
        }
    }

    /// Decompiled virtual method #0 @ RVA 0x004fe700 (18 instructions, 3 basic blocks)
    pub fn method_00(&mut self) {
        // Writes to field at offset +0x0000
    }

    /// Decompiled virtual method #1 @ RVA 0x00500d30 (13 instructions, 3 basic blocks)
    pub fn method_01(&mut self) {
    }

    /// Decompiled virtual method #2 @ RVA 0x0045e4c0 (42 instructions, 4 basic blocks)
    pub fn method_02(&mut self) {
        // Writes to field at offset +0x0090
        // Writes to field at offset +0x0094
        // Writes to field at offset +0x0098
        // Writes to field at offset +0x009c
        // Writes to field at offset +0x00a0
        // Writes to field at offset +0x00a4
        // Writes to field at offset +0x00a8
        // Writes to field at offset +0x00ac
        // Writes to field at offset +0x0140
        // Writes to field at offset +0x0144
        // Writes to field at offset +0x0148
        // Writes to field at offset +0x014c
        // Writes to field at offset +0x0150
        // Writes to field at offset +0x0154
        // Writes to field at offset +0x0158
        // Writes to field at offset +0x015c
    }

    /// Decompiled virtual method #3 @ RVA 0x000d3090 (1 instructions, 1 basic blocks)
    pub fn method_03(&mut self) {
    }

    /// Decompiled virtual method #4 @ RVA 0x0019e550 (2 instructions, 1 basic blocks)
    pub fn method_04(&mut self) {
        // Writes to field at offset +0x00c4
    }

    /// Decompiled virtual method #5 @ RVA 0x00501170 (106 instructions, 23 basic blocks)
    pub fn method_05(&mut self) {
        // Writes to field at offset +0x0040
    }

    /// Decompiled virtual method #6 @ RVA 0x00501130 (18 instructions, 11 basic blocks)
    pub fn method_06(&mut self) {
        // Writes to field at offset +0x01f8
    }

    /// Decompiled virtual method #7 @ RVA 0x000d3090 (1 instructions, 1 basic blocks)
    pub fn method_07(&mut self) {
    }

    /// Decompiled virtual method #8 @ RVA 0x000d3090 (1 instructions, 1 basic blocks)
    pub fn method_08(&mut self) {
    }

    /// Decompiled virtual method #9 @ RVA 0x000d3090 (1 instructions, 1 basic blocks)
    pub fn method_09(&mut self) {
    }

    /// Decompiled virtual method #10 @ RVA 0x00500fe0 (56 instructions, 3 basic blocks)
    pub fn method_10(&mut self) {
        // Writes to field at offset +0x00a0
        // Writes to field at offset +0x00a4
        // Writes to field at offset +0x00a8
        // Writes to field at offset +0x00ac
    }

    /// Decompiled virtual method #11 @ RVA 0x0045e2d0 (33 instructions, 3 basic blocks)
    pub fn method_11(&mut self) {
        // Writes to field at offset +0x0090
        // Writes to field at offset +0x00a0
        // Writes to field at offset +0x00a4
        // Writes to field at offset +0x00a8
        // Writes to field at offset +0x00ac
    }

    /// Decompiled virtual method #12 @ RVA 0x0045eec0 (67 instructions, 22 basic blocks)
    pub fn method_12(&mut self) {
        // Writes to field at offset +0x001c
        // Writes to field at offset +0x0040
        // Writes to field at offset +0x0090
        // Writes to field at offset +0x00b0
    }

    /// Decompiled virtual method #13 @ RVA 0x0045efd0 (27 instructions, 5 basic blocks)
    pub fn method_13(&mut self) {
        // Writes to field at offset +0x0090
        // Writes to field at offset +0x00b0
    }

}

impl CcClass for ccGobjNpcSpDethLundy {
    fn class_name(&self) -> &'static str { Self::CLASS_NAME }
    fn vtable_rva(&self) -> u32 { Self::VTABLE_RVA }
    fn virtual_method_count(&self) -> usize { Self::METHOD_COUNT }
    fn base_classes(&self) -> &'static [&'static str] { Self::BASE_CLASSES }
}

/// Decompiled CC2 class: `ccGobjPc` (VTable RVA: 0x008c0438, 14 virtual methods, 45 recovered fields)
/// Inherits: ccGameObj
#[derive(Debug, Clone, Default)]
pub struct ccGobjPc {
    pub vtable_rva: u32,
    pub field_0x0000: u16,
    pub field_0x0002: u32,
    pub field_0x0008: u32,
    pub field_0x0009: u8,
    pub field_0x000a: u16,
    pub field_0x000c: u32,
    pub field_0x0010: u64,
    pub field_0x0018: u64,
    pub field_0x001c: u8,
    pub field_0x0020: u32,
    pub field_0x0024: u16,
    pub field_0x002c: u16,
    pub field_0x0030: u32,
    pub field_0x0034: u32,
    pub field_0x0040: u32,
    pub field_0x0048: u8,
    pub field_0x0058: u64,
    pub field_0x0090: u32,
    pub field_0x0094: u32,
    pub field_0x0098: u32,
    pub field_0x009c: u32,
    pub field_0x00a0: u32,
    pub field_0x00a4: u32,
    pub field_0x00a8: u32,
    pub field_0x00ac: u32,
    pub field_0x00b0: u64,
    pub field_0x00b8: u64,
    pub field_0x00c4: u32,
    pub field_0x0120: u32,
    pub field_0x0124: u32,
    pub field_0x0128: u32,
    pub field_0x012c: u32,
    pub field_0x0130: u32,
    pub field_0x0134: u32,
    pub field_0x0138: u32,
    pub field_0x013c: u32,
    pub field_0x0140: u32,
    pub field_0x0144: u32,
    pub field_0x0148: u32,
    pub field_0x014c: u32,
    pub field_0x0150: u32,
    pub field_0x0154: u32,
    pub field_0x0158: u32,
    pub field_0x015c: u32,
    pub field_0x01b8: u64,
}

impl ccGobjPc {
    pub const CLASS_NAME: &'static str = "ccGobjPc";
    pub const VTABLE_RVA: u32 = 0x008c0438;
    pub const METHOD_COUNT: usize = 14;
    pub const BASE_CLASSES: &'static [&'static str] = &["ccGameObj"];
    pub const VIRTUAL_METHODS: &'static [u64] = &[0x0000000180503e20, 0x0000000180504280, 0x000000018045e4c0, 0x00000001800d3090, 0x000000018019e550, 0x0000000180504940, 0x0000000180504e20, 0x00000001800d3090, 0x00000001800d3090, 0x0000000180504e90, 0x00000001805047d0, 0x000000018045e2d0, 0x000000018045eec0, 0x000000018045efd0];

    pub fn new() -> Self {
        Self {
            vtable_rva: Self::VTABLE_RVA,
            field_0x0000: Default::default(),
            field_0x0002: Default::default(),
            field_0x0008: Default::default(),
            field_0x0009: Default::default(),
            field_0x000a: Default::default(),
            field_0x000c: Default::default(),
            field_0x0010: Default::default(),
            field_0x0018: Default::default(),
            field_0x001c: Default::default(),
            field_0x0020: Default::default(),
            field_0x0024: Default::default(),
            field_0x002c: Default::default(),
            field_0x0030: Default::default(),
            field_0x0034: Default::default(),
            field_0x0040: Default::default(),
            field_0x0048: Default::default(),
            field_0x0058: Default::default(),
            field_0x0090: Default::default(),
            field_0x0094: Default::default(),
            field_0x0098: Default::default(),
            field_0x009c: Default::default(),
            field_0x00a0: Default::default(),
            field_0x00a4: Default::default(),
            field_0x00a8: Default::default(),
            field_0x00ac: Default::default(),
            field_0x00b0: Default::default(),
            field_0x00b8: Default::default(),
            field_0x00c4: Default::default(),
            field_0x0120: Default::default(),
            field_0x0124: Default::default(),
            field_0x0128: Default::default(),
            field_0x012c: Default::default(),
            field_0x0130: Default::default(),
            field_0x0134: Default::default(),
            field_0x0138: Default::default(),
            field_0x013c: Default::default(),
            field_0x0140: Default::default(),
            field_0x0144: Default::default(),
            field_0x0148: Default::default(),
            field_0x014c: Default::default(),
            field_0x0150: Default::default(),
            field_0x0154: Default::default(),
            field_0x0158: Default::default(),
            field_0x015c: Default::default(),
            field_0x01b8: Default::default(),
        }
    }

    /// Decompiled virtual method #0 @ RVA 0x00503e20 (16 instructions, 3 basic blocks)
    pub fn method_00(&mut self) {
    }

    /// Decompiled virtual method #1 @ RVA 0x00504280 (16 instructions, 3 basic blocks)
    pub fn method_01(&mut self) {
    }

    /// Decompiled virtual method #2 @ RVA 0x0045e4c0 (42 instructions, 4 basic blocks)
    pub fn method_02(&mut self) {
        // Writes to field at offset +0x0090
        // Writes to field at offset +0x0094
        // Writes to field at offset +0x0098
        // Writes to field at offset +0x009c
        // Writes to field at offset +0x00a0
        // Writes to field at offset +0x00a4
        // Writes to field at offset +0x00a8
        // Writes to field at offset +0x00ac
        // Writes to field at offset +0x0140
        // Writes to field at offset +0x0144
        // Writes to field at offset +0x0148
        // Writes to field at offset +0x014c
        // Writes to field at offset +0x0150
        // Writes to field at offset +0x0154
        // Writes to field at offset +0x0158
        // Writes to field at offset +0x015c
    }

    /// Decompiled virtual method #3 @ RVA 0x000d3090 (1 instructions, 1 basic blocks)
    pub fn method_03(&mut self) {
    }

    /// Decompiled virtual method #4 @ RVA 0x0019e550 (2 instructions, 1 basic blocks)
    pub fn method_04(&mut self) {
        // Writes to field at offset +0x00c4
    }

    /// Decompiled virtual method #5 @ RVA 0x00504940 (147 instructions, 49 basic blocks)
    pub fn method_05(&mut self) {
        // Writes to field at offset +0x0000
        // Writes to field at offset +0x0002
        // Writes to field at offset +0x0008
        // Writes to field at offset +0x000c
        // Writes to field at offset +0x0040
        // Writes to field at offset +0x0048
    }

    /// Decompiled virtual method #6 @ RVA 0x00504e20 (30 instructions, 10 basic blocks)
    pub fn method_06(&mut self) {
        // Writes to field at offset +0x0034
    }

    /// Decompiled virtual method #7 @ RVA 0x000d3090 (1 instructions, 1 basic blocks)
    pub fn method_07(&mut self) {
    }

    /// Decompiled virtual method #8 @ RVA 0x000d3090 (1 instructions, 1 basic blocks)
    pub fn method_08(&mut self) {
    }

    /// Decompiled virtual method #9 @ RVA 0x00504e90 (118 instructions, 33 basic blocks)
    pub fn method_09(&mut self) {
        // Writes to field at offset +0x0009
        // Writes to field at offset +0x000a
        // Writes to field at offset +0x0030
    }

    /// Decompiled virtual method #10 @ RVA 0x005047d0 (72 instructions, 11 basic blocks)
    pub fn method_10(&mut self) {
        // Writes to field at offset +0x0090
        // Writes to field at offset +0x0094
        // Writes to field at offset +0x0098
        // Writes to field at offset +0x009c
        // Writes to field at offset +0x00a0
        // Writes to field at offset +0x00a4
        // Writes to field at offset +0x00a8
        // Writes to field at offset +0x00ac
        // Writes to field at offset +0x01b8
    }

    /// Decompiled virtual method #11 @ RVA 0x0045e2d0 (33 instructions, 3 basic blocks)
    pub fn method_11(&mut self) {
        // Writes to field at offset +0x0090
        // Writes to field at offset +0x00a0
        // Writes to field at offset +0x00a4
        // Writes to field at offset +0x00a8
        // Writes to field at offset +0x00ac
    }

    /// Decompiled virtual method #12 @ RVA 0x0045eec0 (67 instructions, 22 basic blocks)
    pub fn method_12(&mut self) {
        // Writes to field at offset +0x001c
        // Writes to field at offset +0x0040
        // Writes to field at offset +0x0090
        // Writes to field at offset +0x00b0
    }

    /// Decompiled virtual method #13 @ RVA 0x0045efd0 (27 instructions, 5 basic blocks)
    pub fn method_13(&mut self) {
        // Writes to field at offset +0x0090
        // Writes to field at offset +0x00b0
    }

}

impl CcClass for ccGobjPc {
    fn class_name(&self) -> &'static str { Self::CLASS_NAME }
    fn vtable_rva(&self) -> u32 { Self::VTABLE_RVA }
    fn virtual_method_count(&self) -> usize { Self::METHOD_COUNT }
    fn base_classes(&self) -> &'static [&'static str] { Self::BASE_CLASSES }
}

/// Decompiled CC2 class: `ccGobjRareChim` (VTable RVA: 0x008bc878, 17 virtual methods, 43 recovered fields)
/// Inherits: ccGobjChimBase, ccGameObj
#[derive(Debug, Clone, Default)]
pub struct ccGobjRareChim {
    pub vtable_rva: u32,
    pub field_0x0000: u64,
    pub field_0x0009: u8,
    pub field_0x0010: u64,
    pub field_0x0018: u64,
    pub field_0x001c: u8,
    pub field_0x0020: u32,
    pub field_0x0024: u16,
    pub field_0x002c: u16,
    pub field_0x0040: u32,
    pub field_0x0050: u64,
    pub field_0x0058: u64,
    pub field_0x0090: u32,
    pub field_0x0094: u32,
    pub field_0x0098: u32,
    pub field_0x009c: u32,
    pub field_0x00a0: u32,
    pub field_0x00a4: u32,
    pub field_0x00a8: u32,
    pub field_0x00ac: u32,
    pub field_0x00b0: u64,
    pub field_0x00b8: u64,
    pub field_0x00c4: u32,
    pub field_0x00d0: u32,
    pub field_0x0120: u32,
    pub field_0x0124: u32,
    pub field_0x0128: u32,
    pub field_0x012c: u32,
    pub field_0x0130: u32,
    pub field_0x0134: u32,
    pub field_0x0138: u32,
    pub field_0x013c: u32,
    pub field_0x0140: u32,
    pub field_0x0144: u32,
    pub field_0x0148: u32,
    pub field_0x014c: u32,
    pub field_0x0150: u32,
    pub field_0x0154: u32,
    pub field_0x0158: u32,
    pub field_0x015c: u32,
    pub field_0x01b8: u64,
    pub field_0x01c0: u32,
    pub field_0x01c8: u64,
    pub field_0x0236: u16,
}

impl ccGobjRareChim {
    pub const CLASS_NAME: &'static str = "ccGobjRareChim";
    pub const VTABLE_RVA: u32 = 0x008bc878;
    pub const METHOD_COUNT: usize = 17;
    pub const BASE_CLASSES: &'static [&'static str] = &["ccGobjChimBase", "ccGameObj"];
    pub const VIRTUAL_METHODS: &'static [u64] = &[0x00000001804a3d00, 0x00000001804a4c00, 0x000000018045e4c0, 0x00000001800d3090, 0x000000018019e550, 0x00000001804a5740, 0x00000001804a6310, 0x00000001800d3090, 0x00000001800d3090, 0x00000001804a68f0, 0x00000001804a5e00, 0x000000018045e2d0, 0x000000018045eec0, 0x000000018045efd0, 0x00000001804a77b0, 0x00000001804a7a50, 0x00000001804a9dd0];

    pub fn new() -> Self {
        Self {
            vtable_rva: Self::VTABLE_RVA,
            field_0x0000: Default::default(),
            field_0x0009: Default::default(),
            field_0x0010: Default::default(),
            field_0x0018: Default::default(),
            field_0x001c: Default::default(),
            field_0x0020: Default::default(),
            field_0x0024: Default::default(),
            field_0x002c: Default::default(),
            field_0x0040: Default::default(),
            field_0x0050: Default::default(),
            field_0x0058: Default::default(),
            field_0x0090: Default::default(),
            field_0x0094: Default::default(),
            field_0x0098: Default::default(),
            field_0x009c: Default::default(),
            field_0x00a0: Default::default(),
            field_0x00a4: Default::default(),
            field_0x00a8: Default::default(),
            field_0x00ac: Default::default(),
            field_0x00b0: Default::default(),
            field_0x00b8: Default::default(),
            field_0x00c4: Default::default(),
            field_0x00d0: Default::default(),
            field_0x0120: Default::default(),
            field_0x0124: Default::default(),
            field_0x0128: Default::default(),
            field_0x012c: Default::default(),
            field_0x0130: Default::default(),
            field_0x0134: Default::default(),
            field_0x0138: Default::default(),
            field_0x013c: Default::default(),
            field_0x0140: Default::default(),
            field_0x0144: Default::default(),
            field_0x0148: Default::default(),
            field_0x014c: Default::default(),
            field_0x0150: Default::default(),
            field_0x0154: Default::default(),
            field_0x0158: Default::default(),
            field_0x015c: Default::default(),
            field_0x01b8: Default::default(),
            field_0x01c0: Default::default(),
            field_0x01c8: Default::default(),
            field_0x0236: Default::default(),
        }
    }

    /// Decompiled virtual method #0 @ RVA 0x004a3d00 (24 instructions, 3 basic blocks)
    pub fn method_00(&mut self) {
        // Writes to field at offset +0x0000
    }

    /// Decompiled virtual method #1 @ RVA 0x004a4c00 (114 instructions, 25 basic blocks)
    pub fn method_01(&mut self) {
    }

    /// Decompiled virtual method #2 @ RVA 0x0045e4c0 (42 instructions, 4 basic blocks)
    pub fn method_02(&mut self) {
        // Writes to field at offset +0x0090
        // Writes to field at offset +0x0094
        // Writes to field at offset +0x0098
        // Writes to field at offset +0x009c
        // Writes to field at offset +0x00a0
        // Writes to field at offset +0x00a4
        // Writes to field at offset +0x00a8
        // Writes to field at offset +0x00ac
        // Writes to field at offset +0x0140
        // Writes to field at offset +0x0144
        // Writes to field at offset +0x0148
        // Writes to field at offset +0x014c
        // Writes to field at offset +0x0150
        // Writes to field at offset +0x0154
        // Writes to field at offset +0x0158
        // Writes to field at offset +0x015c
    }

    /// Decompiled virtual method #3 @ RVA 0x000d3090 (1 instructions, 1 basic blocks)
    pub fn method_03(&mut self) {
    }

    /// Decompiled virtual method #4 @ RVA 0x0019e550 (2 instructions, 1 basic blocks)
    pub fn method_04(&mut self) {
        // Writes to field at offset +0x00c4
    }

    /// Decompiled virtual method #5 @ RVA 0x004a5740 (134 instructions, 37 basic blocks)
    pub fn method_05(&mut self) {
        // Writes to field at offset +0x0024
    }

    /// Decompiled virtual method #6 @ RVA 0x004a6310 (103 instructions, 17 basic blocks)
    pub fn method_06(&mut self) {
        // Writes to field at offset +0x0009
    }

    /// Decompiled virtual method #7 @ RVA 0x000d3090 (1 instructions, 1 basic blocks)
    pub fn method_07(&mut self) {
    }

    /// Decompiled virtual method #8 @ RVA 0x000d3090 (1 instructions, 1 basic blocks)
    pub fn method_08(&mut self) {
    }

    /// Decompiled virtual method #9 @ RVA 0x004a68f0 (98 instructions, 11 basic blocks)
    pub fn method_09(&mut self) {
        // Writes to field at offset +0x0009
        // Writes to field at offset +0x00a4
        // Writes to field at offset +0x00d0
    }

    /// Decompiled virtual method #10 @ RVA 0x004a5e00 (97 instructions, 16 basic blocks)
    pub fn method_10(&mut self) {
        // Writes to field at offset +0x00c4
    }

    /// Decompiled virtual method #11 @ RVA 0x0045e2d0 (33 instructions, 3 basic blocks)
    pub fn method_11(&mut self) {
        // Writes to field at offset +0x0090
        // Writes to field at offset +0x00a0
        // Writes to field at offset +0x00a4
        // Writes to field at offset +0x00a8
        // Writes to field at offset +0x00ac
    }

    /// Decompiled virtual method #12 @ RVA 0x0045eec0 (67 instructions, 22 basic blocks)
    pub fn method_12(&mut self) {
        // Writes to field at offset +0x001c
        // Writes to field at offset +0x0040
        // Writes to field at offset +0x0090
        // Writes to field at offset +0x00b0
    }

    /// Decompiled virtual method #13 @ RVA 0x0045efd0 (27 instructions, 5 basic blocks)
    pub fn method_13(&mut self) {
        // Writes to field at offset +0x0090
        // Writes to field at offset +0x00b0
    }

    /// Decompiled virtual method #14 @ RVA 0x004a77b0 (91 instructions, 12 basic blocks)
    pub fn method_14(&mut self) {
        // Writes to field at offset +0x00d0
        // Writes to field at offset +0x0236
    }

    /// Decompiled virtual method #15 @ RVA 0x004a7a50 (30 instructions, 8 basic blocks)
    pub fn method_15(&mut self) {
        // Writes to field at offset +0x00d0
    }

    /// Decompiled virtual method #16 @ RVA 0x004a9dd0 (130 instructions, 21 basic blocks)
    pub fn method_16(&mut self) {
    }

}

impl CcClass for ccGobjRareChim {
    fn class_name(&self) -> &'static str { Self::CLASS_NAME }
    fn vtable_rva(&self) -> u32 { Self::VTABLE_RVA }
    fn virtual_method_count(&self) -> usize { Self::METHOD_COUNT }
    fn base_classes(&self) -> &'static [&'static str] { Self::BASE_CLASSES }
}

/// Decompiled CC2 class: `ccGobjSpc` (VTable RVA: 0x008c0658, 23 virtual methods, 76 recovered fields)
/// Inherits: ccGobjBattle, ccGameObj
#[derive(Debug, Clone, Default)]
pub struct ccGobjSpc {
    pub vtable_rva: u32,
    pub field_0x0000: u32,
    pub field_0x0004: u32,
    pub field_0x0008: u16,
    pub field_0x000a: u16,
    pub field_0x000c: u16,
    pub field_0x0010: u64,
    pub field_0x0018: u64,
    pub field_0x001c: u8,
    pub field_0x0020: u32,
    pub field_0x0024: u32,
    pub field_0x0028: u32,
    pub field_0x002c: u16,
    pub field_0x0040: u32,
    pub field_0x0068: u64,
    pub field_0x0070: u32,
    pub field_0x0090: u32,
    pub field_0x0094: u32,
    pub field_0x0098: u32,
    pub field_0x009c: u32,
    pub field_0x00a0: u32,
    pub field_0x00a4: u32,
    pub field_0x00a8: u32,
    pub field_0x00ac: u32,
    pub field_0x00b0: u64,
    pub field_0x00b8: u64,
    pub field_0x00c4: u32,
    pub field_0x00e0: u32,
    pub field_0x00e4: u32,
    pub field_0x00e8: u32,
    pub field_0x00ec: u32,
    pub field_0x00f0: u32,
    pub field_0x00f4: u32,
    pub field_0x00f8: u32,
    pub field_0x0120: u32,
    pub field_0x0124: u32,
    pub field_0x0128: u32,
    pub field_0x012c: u32,
    pub field_0x0130: u32,
    pub field_0x0134: u32,
    pub field_0x0138: u32,
    pub field_0x013c: u32,
    pub field_0x0140: u32,
    pub field_0x0144: u32,
    pub field_0x0148: u32,
    pub field_0x014c: u32,
    pub field_0x0150: u32,
    pub field_0x0154: u32,
    pub field_0x0158: u32,
    pub field_0x015c: u32,
    pub field_0x01c0: u64,
    pub field_0x01c8: u32,
    pub field_0x01e4: u16,
    pub field_0x01e6: u16,
    pub field_0x020c: u64,
    pub field_0x0214: u64,
    pub field_0x021c: u64,
    pub field_0x0224: u64,
    pub field_0x022c: u32,
    pub field_0x0230: [f32; 4],
    pub field_0x0234: u64,
    pub field_0x023c: u64,
    pub field_0x0240: u32,
    pub field_0x0244: u64,
    pub field_0x024c: u64,
    pub field_0x0254: u32,
    pub field_0x0278: u32,
    pub field_0x02da: u16,
    pub field_0x02fc: u8,
    pub field_0x0314: u16,
    pub field_0x0324: u32,
    pub field_0x03e0: u32,
    pub field_0x03f0: u64,
    pub field_0x0400: u64,
    pub field_0x0410: u64,
    pub field_0x0440: u32,
    pub field_0x04f0: u32,
}

impl ccGobjSpc {
    pub const CLASS_NAME: &'static str = "ccGobjSpc";
    pub const VTABLE_RVA: u32 = 0x008c0658;
    pub const METHOD_COUNT: usize = 23;
    pub const BASE_CLASSES: &'static [&'static str] = &["ccGobjBattle", "ccGameObj"];
    pub const VIRTUAL_METHODS: &'static [u64] = &[0x00000001805086e0, 0x0000000180509960, 0x000000018045e4c0, 0x0000000180517180, 0x000000018047ebd0, 0x00000001805094c0, 0x000000018050a570, 0x00000001800d3090, 0x000000018050aa50, 0x000000018050b920, 0x000000018050a4e0, 0x000000018047f0e0, 0x000000018045eec0, 0x000000018045efd0, 0x000000018051d300, 0x00000001805191c0, 0x0000000180518920, 0x000000018051d220, 0x0000000180518720, 0x00000001805187c0, 0x0000000180518850, 0x000000018051d510, 0x0000000180508900];

    pub fn new() -> Self {
        Self {
            vtable_rva: Self::VTABLE_RVA,
            field_0x0000: Default::default(),
            field_0x0004: Default::default(),
            field_0x0008: Default::default(),
            field_0x000a: Default::default(),
            field_0x000c: Default::default(),
            field_0x0010: Default::default(),
            field_0x0018: Default::default(),
            field_0x001c: Default::default(),
            field_0x0020: Default::default(),
            field_0x0024: Default::default(),
            field_0x0028: Default::default(),
            field_0x002c: Default::default(),
            field_0x0040: Default::default(),
            field_0x0068: Default::default(),
            field_0x0070: Default::default(),
            field_0x0090: Default::default(),
            field_0x0094: Default::default(),
            field_0x0098: Default::default(),
            field_0x009c: Default::default(),
            field_0x00a0: Default::default(),
            field_0x00a4: Default::default(),
            field_0x00a8: Default::default(),
            field_0x00ac: Default::default(),
            field_0x00b0: Default::default(),
            field_0x00b8: Default::default(),
            field_0x00c4: Default::default(),
            field_0x00e0: Default::default(),
            field_0x00e4: Default::default(),
            field_0x00e8: Default::default(),
            field_0x00ec: Default::default(),
            field_0x00f0: Default::default(),
            field_0x00f4: Default::default(),
            field_0x00f8: Default::default(),
            field_0x0120: Default::default(),
            field_0x0124: Default::default(),
            field_0x0128: Default::default(),
            field_0x012c: Default::default(),
            field_0x0130: Default::default(),
            field_0x0134: Default::default(),
            field_0x0138: Default::default(),
            field_0x013c: Default::default(),
            field_0x0140: Default::default(),
            field_0x0144: Default::default(),
            field_0x0148: Default::default(),
            field_0x014c: Default::default(),
            field_0x0150: Default::default(),
            field_0x0154: Default::default(),
            field_0x0158: Default::default(),
            field_0x015c: Default::default(),
            field_0x01c0: Default::default(),
            field_0x01c8: Default::default(),
            field_0x01e4: Default::default(),
            field_0x01e6: Default::default(),
            field_0x020c: Default::default(),
            field_0x0214: Default::default(),
            field_0x021c: Default::default(),
            field_0x0224: Default::default(),
            field_0x022c: Default::default(),
            field_0x0230: Default::default(),
            field_0x0234: Default::default(),
            field_0x023c: Default::default(),
            field_0x0240: Default::default(),
            field_0x0244: Default::default(),
            field_0x024c: Default::default(),
            field_0x0254: Default::default(),
            field_0x0278: Default::default(),
            field_0x02da: Default::default(),
            field_0x02fc: Default::default(),
            field_0x0314: Default::default(),
            field_0x0324: Default::default(),
            field_0x03e0: Default::default(),
            field_0x03f0: Default::default(),
            field_0x0400: Default::default(),
            field_0x0410: Default::default(),
            field_0x0440: Default::default(),
            field_0x04f0: Default::default(),
        }
    }

    /// Decompiled virtual method #0 @ RVA 0x005086e0 (16 instructions, 3 basic blocks)
    pub fn method_00(&mut self) {
    }

    /// Decompiled virtual method #1 @ RVA 0x00509960 (19 instructions, 5 basic blocks)
    pub fn method_01(&mut self) {
        // Writes to field at offset +0x00c4
    }

    /// Decompiled virtual method #2 @ RVA 0x0045e4c0 (42 instructions, 4 basic blocks)
    pub fn method_02(&mut self) {
        // Writes to field at offset +0x0090
        // Writes to field at offset +0x0094
        // Writes to field at offset +0x0098
        // Writes to field at offset +0x009c
        // Writes to field at offset +0x00a0
        // Writes to field at offset +0x00a4
        // Writes to field at offset +0x00a8
        // Writes to field at offset +0x00ac
        // Writes to field at offset +0x0140
        // Writes to field at offset +0x0144
        // Writes to field at offset +0x0148
        // Writes to field at offset +0x014c
        // Writes to field at offset +0x0150
        // Writes to field at offset +0x0154
        // Writes to field at offset +0x0158
        // Writes to field at offset +0x015c
    }

    /// Decompiled virtual method #3 @ RVA 0x00517180 (96 instructions, 32 basic blocks)
    pub fn method_03(&mut self) {
        // Writes to field at offset +0x0018
        // Writes to field at offset +0x00b0
    }

    /// Decompiled virtual method #4 @ RVA 0x0047ebd0 (45 instructions, 12 basic blocks)
    pub fn method_04(&mut self) {
        // Writes to field at offset +0x0004
        // Writes to field at offset +0x02da
        // Writes to field at offset +0x0314
        // Writes to field at offset +0x03e0
    }

    /// Decompiled virtual method #5 @ RVA 0x005094c0 (115 instructions, 22 basic blocks)
    pub fn method_05(&mut self) {
        // Writes to field at offset +0x000a
        // Writes to field at offset +0x000c
        // Writes to field at offset +0x0024
    }

    /// Decompiled virtual method #6 @ RVA 0x0050a570 (121 instructions, 26 basic blocks)
    pub fn method_06(&mut self) {
        // Writes to field at offset +0x000c
        // Writes to field at offset +0x0070
        // Writes to field at offset +0x0324
    }

    /// Decompiled virtual method #7 @ RVA 0x000d3090 (1 instructions, 1 basic blocks)
    pub fn method_07(&mut self) {
    }

    /// Decompiled virtual method #8 @ RVA 0x0050aa50 (30 instructions, 6 basic blocks)
    pub fn method_08(&mut self) {
        // Writes to field at offset +0x00c4
        // Writes to field at offset +0x04f0
    }

    /// Decompiled virtual method #9 @ RVA 0x0050b920 (111 instructions, 27 basic blocks)
    pub fn method_09(&mut self) {
    }

    /// Decompiled virtual method #10 @ RVA 0x0050a4e0 (30 instructions, 12 basic blocks)
    pub fn method_10(&mut self) {
    }

    /// Decompiled virtual method #11 @ RVA 0x0047f0e0 (15 instructions, 4 basic blocks)
    pub fn method_11(&mut self) {
    }

    /// Decompiled virtual method #12 @ RVA 0x0045eec0 (67 instructions, 22 basic blocks)
    pub fn method_12(&mut self) {
        // Writes to field at offset +0x001c
        // Writes to field at offset +0x0040
        // Writes to field at offset +0x0090
        // Writes to field at offset +0x00b0
    }

    /// Decompiled virtual method #13 @ RVA 0x0045efd0 (27 instructions, 5 basic blocks)
    pub fn method_13(&mut self) {
        // Writes to field at offset +0x0090
        // Writes to field at offset +0x00b0
    }

    /// Decompiled virtual method #14 @ RVA 0x0051d300 (107 instructions, 1 basic blocks)
    pub fn method_14(&mut self) {
        // Writes to field at offset +0x0000
        // Writes to field at offset +0x0004
        // Writes to field at offset +0x0008
        // Writes to field at offset +0x000a
        // Writes to field at offset +0x020c
        // Writes to field at offset +0x0214
        // Writes to field at offset +0x021c
        // Writes to field at offset +0x0224
        // Writes to field at offset +0x022c
        // Writes to field at offset +0x0234
        // Writes to field at offset +0x023c
        // Writes to field at offset +0x0244
        // Writes to field at offset +0x024c
        // Writes to field at offset +0x0254
    }

    /// Decompiled virtual method #15 @ RVA 0x005191c0 (107 instructions, 35 basic blocks)
    pub fn method_15(&mut self) {
        // Writes to field at offset +0x0004
        // Writes to field at offset +0x00c4
        // Writes to field at offset +0x01e4
    }

    /// Decompiled virtual method #16 @ RVA 0x00518920 (120 instructions, 25 basic blocks)
    pub fn method_16(&mut self) {
    }

    /// Decompiled virtual method #17 @ RVA 0x0051d220 (52 instructions, 11 basic blocks)
    pub fn method_17(&mut self) {
    }

    /// Decompiled virtual method #18 @ RVA 0x00518720 (40 instructions, 11 basic blocks)
    pub fn method_18(&mut self) {
        // Writes to field at offset +0x0400
    }

    /// Decompiled virtual method #19 @ RVA 0x005187c0 (38 instructions, 11 basic blocks)
    pub fn method_19(&mut self) {
    }

    /// Decompiled virtual method #20 @ RVA 0x00518850 (50 instructions, 15 basic blocks)
    pub fn method_20(&mut self) {
    }

    /// Decompiled virtual method #21 @ RVA 0x0051d510 (9 instructions, 3 basic blocks)
    pub fn method_21(&mut self) {
        // Writes to field at offset +0x0028
        // Writes to field at offset +0x002c
        // Writes to field at offset +0x02fc
    }

    /// Decompiled virtual method #22 @ RVA 0x00508900 (125 instructions, 21 basic blocks)
    pub fn method_22(&mut self) {
        // Writes to field at offset +0x01c0
    }

}

impl CcClass for ccGobjSpc {
    fn class_name(&self) -> &'static str { Self::CLASS_NAME }
    fn vtable_rva(&self) -> u32 { Self::VTABLE_RVA }
    fn virtual_method_count(&self) -> usize { Self::METHOD_COUNT }
    fn base_classes(&self) -> &'static [&'static str] { Self::BASE_CLASSES }
}

/// Decompiled CC2 class: `ccGobjSpcEnd` (VTable RVA: 0x008e5df0, 23 virtual methods, 57 recovered fields)
/// Inherits: ccGobjSpc, ccGobjBattle, ccGameObj
#[derive(Debug, Clone, Default)]
pub struct ccGobjSpcEnd {
    pub vtable_rva: u32,
    pub field_0x0000: u32,
    pub field_0x0004: u32,
    pub field_0x0008: u16,
    pub field_0x000a: u16,
    pub field_0x000c: u16,
    pub field_0x0010: u64,
    pub field_0x0018: u64,
    pub field_0x001c: u8,
    pub field_0x0020: u32,
    pub field_0x0024: u32,
    pub field_0x0028: u32,
    pub field_0x002c: u16,
    pub field_0x0040: u32,
    pub field_0x0068: u64,
    pub field_0x0070: u32,
    pub field_0x0090: u32,
    pub field_0x0094: u32,
    pub field_0x0098: u32,
    pub field_0x009c: u32,
    pub field_0x00a0: u32,
    pub field_0x00a4: u32,
    pub field_0x00a8: u32,
    pub field_0x00ac: u32,
    pub field_0x00b0: u64,
    pub field_0x00b8: u64,
    pub field_0x00c4: u32,
    pub field_0x0120: u32,
    pub field_0x0124: u32,
    pub field_0x0128: u32,
    pub field_0x012c: u32,
    pub field_0x0130: u32,
    pub field_0x0134: u32,
    pub field_0x0138: u32,
    pub field_0x013c: u32,
    pub field_0x0140: u32,
    pub field_0x0144: u32,
    pub field_0x0148: u32,
    pub field_0x014c: u32,
    pub field_0x0150: u32,
    pub field_0x0154: u32,
    pub field_0x0158: u32,
    pub field_0x015c: u32,
    pub field_0x01c0: u64,
    pub field_0x01c8: u32,
    pub field_0x01e4: u16,
    pub field_0x0278: u32,
    pub field_0x02da: u16,
    pub field_0x02fc: u8,
    pub field_0x0314: u16,
    pub field_0x0324: u32,
    pub field_0x03e0: u32,
    pub field_0x03f0: u64,
    pub field_0x0400: u64,
    pub field_0x0410: u64,
    pub field_0x0440: u32,
    pub field_0x04f0: u32,
    pub field_0x0510: u16,
}

impl ccGobjSpcEnd {
    pub const CLASS_NAME: &'static str = "ccGobjSpcEnd";
    pub const VTABLE_RVA: u32 = 0x008e5df0;
    pub const METHOD_COUNT: usize = 23;
    pub const BASE_CLASSES: &'static [&'static str] = &["ccGobjSpc", "ccGobjBattle", "ccGameObj"];
    pub const VIRTUAL_METHODS: &'static [u64] = &[0x00000001805b9420, 0x0000000180509960, 0x000000018045e4c0, 0x0000000180517180, 0x000000018047ebd0, 0x00000001805094c0, 0x000000018050a570, 0x00000001800d3090, 0x000000018050aa50, 0x0000000180521d80, 0x000000018050a4e0, 0x000000018047f0e0, 0x000000018045eec0, 0x000000018045efd0, 0x00000001805226a0, 0x00000001805191c0, 0x0000000180518920, 0x000000018051d220, 0x0000000180518720, 0x00000001805187c0, 0x0000000180518850, 0x000000018051d510, 0x0000000180508900];

    pub fn new() -> Self {
        Self {
            vtable_rva: Self::VTABLE_RVA,
            field_0x0000: Default::default(),
            field_0x0004: Default::default(),
            field_0x0008: Default::default(),
            field_0x000a: Default::default(),
            field_0x000c: Default::default(),
            field_0x0010: Default::default(),
            field_0x0018: Default::default(),
            field_0x001c: Default::default(),
            field_0x0020: Default::default(),
            field_0x0024: Default::default(),
            field_0x0028: Default::default(),
            field_0x002c: Default::default(),
            field_0x0040: Default::default(),
            field_0x0068: Default::default(),
            field_0x0070: Default::default(),
            field_0x0090: Default::default(),
            field_0x0094: Default::default(),
            field_0x0098: Default::default(),
            field_0x009c: Default::default(),
            field_0x00a0: Default::default(),
            field_0x00a4: Default::default(),
            field_0x00a8: Default::default(),
            field_0x00ac: Default::default(),
            field_0x00b0: Default::default(),
            field_0x00b8: Default::default(),
            field_0x00c4: Default::default(),
            field_0x0120: Default::default(),
            field_0x0124: Default::default(),
            field_0x0128: Default::default(),
            field_0x012c: Default::default(),
            field_0x0130: Default::default(),
            field_0x0134: Default::default(),
            field_0x0138: Default::default(),
            field_0x013c: Default::default(),
            field_0x0140: Default::default(),
            field_0x0144: Default::default(),
            field_0x0148: Default::default(),
            field_0x014c: Default::default(),
            field_0x0150: Default::default(),
            field_0x0154: Default::default(),
            field_0x0158: Default::default(),
            field_0x015c: Default::default(),
            field_0x01c0: Default::default(),
            field_0x01c8: Default::default(),
            field_0x01e4: Default::default(),
            field_0x0278: Default::default(),
            field_0x02da: Default::default(),
            field_0x02fc: Default::default(),
            field_0x0314: Default::default(),
            field_0x0324: Default::default(),
            field_0x03e0: Default::default(),
            field_0x03f0: Default::default(),
            field_0x0400: Default::default(),
            field_0x0410: Default::default(),
            field_0x0440: Default::default(),
            field_0x04f0: Default::default(),
            field_0x0510: Default::default(),
        }
    }

    /// Decompiled virtual method #0 @ RVA 0x005b9420 (16 instructions, 3 basic blocks)
    pub fn method_00(&mut self) {
    }

    /// Decompiled virtual method #1 @ RVA 0x00509960 (19 instructions, 5 basic blocks)
    pub fn method_01(&mut self) {
        // Writes to field at offset +0x00c4
    }

    /// Decompiled virtual method #2 @ RVA 0x0045e4c0 (42 instructions, 4 basic blocks)
    pub fn method_02(&mut self) {
        // Writes to field at offset +0x0090
        // Writes to field at offset +0x0094
        // Writes to field at offset +0x0098
        // Writes to field at offset +0x009c
        // Writes to field at offset +0x00a0
        // Writes to field at offset +0x00a4
        // Writes to field at offset +0x00a8
        // Writes to field at offset +0x00ac
        // Writes to field at offset +0x0140
        // Writes to field at offset +0x0144
        // Writes to field at offset +0x0148
        // Writes to field at offset +0x014c
        // Writes to field at offset +0x0150
        // Writes to field at offset +0x0154
        // Writes to field at offset +0x0158
        // Writes to field at offset +0x015c
    }

    /// Decompiled virtual method #3 @ RVA 0x00517180 (96 instructions, 32 basic blocks)
    pub fn method_03(&mut self) {
        // Writes to field at offset +0x0018
        // Writes to field at offset +0x00b0
    }

    /// Decompiled virtual method #4 @ RVA 0x0047ebd0 (45 instructions, 12 basic blocks)
    pub fn method_04(&mut self) {
        // Writes to field at offset +0x0004
        // Writes to field at offset +0x02da
        // Writes to field at offset +0x0314
        // Writes to field at offset +0x03e0
    }

    /// Decompiled virtual method #5 @ RVA 0x005094c0 (115 instructions, 22 basic blocks)
    pub fn method_05(&mut self) {
        // Writes to field at offset +0x000a
        // Writes to field at offset +0x000c
        // Writes to field at offset +0x0024
    }

    /// Decompiled virtual method #6 @ RVA 0x0050a570 (121 instructions, 26 basic blocks)
    pub fn method_06(&mut self) {
        // Writes to field at offset +0x000c
        // Writes to field at offset +0x0070
        // Writes to field at offset +0x0324
    }

    /// Decompiled virtual method #7 @ RVA 0x000d3090 (1 instructions, 1 basic blocks)
    pub fn method_07(&mut self) {
    }

    /// Decompiled virtual method #8 @ RVA 0x0050aa50 (30 instructions, 6 basic blocks)
    pub fn method_08(&mut self) {
        // Writes to field at offset +0x00c4
        // Writes to field at offset +0x04f0
    }

    /// Decompiled virtual method #9 @ RVA 0x00521d80 (113 instructions, 23 basic blocks)
    pub fn method_09(&mut self) {
        // Writes to field at offset +0x0000
        // Writes to field at offset +0x0004
        // Writes to field at offset +0x0008
        // Writes to field at offset +0x000a
        // Writes to field at offset +0x02fc
        // Writes to field at offset +0x0510
    }

    /// Decompiled virtual method #10 @ RVA 0x0050a4e0 (30 instructions, 12 basic blocks)
    pub fn method_10(&mut self) {
    }

    /// Decompiled virtual method #11 @ RVA 0x0047f0e0 (15 instructions, 4 basic blocks)
    pub fn method_11(&mut self) {
    }

    /// Decompiled virtual method #12 @ RVA 0x0045eec0 (67 instructions, 22 basic blocks)
    pub fn method_12(&mut self) {
        // Writes to field at offset +0x001c
        // Writes to field at offset +0x0040
        // Writes to field at offset +0x0090
        // Writes to field at offset +0x00b0
    }

    /// Decompiled virtual method #13 @ RVA 0x0045efd0 (27 instructions, 5 basic blocks)
    pub fn method_13(&mut self) {
        // Writes to field at offset +0x0090
        // Writes to field at offset +0x00b0
    }

    /// Decompiled virtual method #14 @ RVA 0x005226a0 (9 instructions, 1 basic blocks)
    pub fn method_14(&mut self) {
    }

    /// Decompiled virtual method #15 @ RVA 0x005191c0 (107 instructions, 35 basic blocks)
    pub fn method_15(&mut self) {
        // Writes to field at offset +0x0004
        // Writes to field at offset +0x00c4
        // Writes to field at offset +0x01e4
    }

    /// Decompiled virtual method #16 @ RVA 0x00518920 (120 instructions, 25 basic blocks)
    pub fn method_16(&mut self) {
    }

    /// Decompiled virtual method #17 @ RVA 0x0051d220 (52 instructions, 11 basic blocks)
    pub fn method_17(&mut self) {
    }

    /// Decompiled virtual method #18 @ RVA 0x00518720 (40 instructions, 11 basic blocks)
    pub fn method_18(&mut self) {
        // Writes to field at offset +0x0400
    }

    /// Decompiled virtual method #19 @ RVA 0x005187c0 (38 instructions, 11 basic blocks)
    pub fn method_19(&mut self) {
    }

    /// Decompiled virtual method #20 @ RVA 0x00518850 (50 instructions, 15 basic blocks)
    pub fn method_20(&mut self) {
    }

    /// Decompiled virtual method #21 @ RVA 0x0051d510 (9 instructions, 3 basic blocks)
    pub fn method_21(&mut self) {
        // Writes to field at offset +0x0028
        // Writes to field at offset +0x002c
        // Writes to field at offset +0x02fc
    }

    /// Decompiled virtual method #22 @ RVA 0x00508900 (125 instructions, 21 basic blocks)
    pub fn method_22(&mut self) {
        // Writes to field at offset +0x01c0
    }

}

impl CcClass for ccGobjSpcEnd {
    fn class_name(&self) -> &'static str { Self::CLASS_NAME }
    fn vtable_rva(&self) -> u32 { Self::VTABLE_RVA }
    fn virtual_method_count(&self) -> usize { Self::METHOD_COUNT }
    fn base_classes(&self) -> &'static [&'static str] { Self::BASE_CLASSES }
}


//! Subsystem: core
//! Contains 150 decompiled classes.

use crate::CcClass;

/// Decompiled CC2 class: `ccAPKWeb` (VTable RVA: 0x008fb1c8, 11 virtual methods, 17 recovered fields)
/// Inherits: ccWebBase
#[derive(Debug, Clone, Default)]
pub struct ccAPKWeb {
    pub vtable_rva: u32,
    pub field_0x0000: u8,
    pub field_0x0001: u32,
    pub field_0x0008: u64,
    pub field_0x0010: u64,
    pub field_0x0040: u32,
    pub field_0x0044: f32,
    pub field_0x0048: f32,
    pub field_0x0054: f32,
    pub field_0x0058: f32,
    pub field_0x0060: f32,
    pub field_0x0068: u64,
    pub field_0x0070: u64,
    pub field_0x0088: u64,
    pub field_0x0090: u8,
    pub field_0x0360: u32,
    pub field_0x0368: u8,
    pub field_0x0398: [f32; 4],
}

impl ccAPKWeb {
    pub const CLASS_NAME: &'static str = "ccAPKWeb";
    pub const VTABLE_RVA: u32 = 0x008fb1c8;
    pub const METHOD_COUNT: usize = 11;
    pub const BASE_CLASSES: &'static [&'static str] = &["ccWebBase"];
    pub const VIRTUAL_METHODS: &'static [u64] = &[0x000000018066b9d0, 0x000000018066be10, 0x00000001800d3090, 0x000000018066bd20, 0x0000000180667700, 0x00000001800d3090, 0x000000018066b940, 0x000000018066b990, 0x00000001806687a0, 0x0000000180668b10, 0x0000000180668d80];

    pub fn new() -> Self {
        Self {
            vtable_rva: Self::VTABLE_RVA,
            field_0x0000: Default::default(),
            field_0x0001: Default::default(),
            field_0x0008: Default::default(),
            field_0x0010: Default::default(),
            field_0x0040: Default::default(),
            field_0x0044: Default::default(),
            field_0x0048: Default::default(),
            field_0x0054: Default::default(),
            field_0x0058: Default::default(),
            field_0x0060: Default::default(),
            field_0x0068: Default::default(),
            field_0x0070: Default::default(),
            field_0x0088: Default::default(),
            field_0x0090: Default::default(),
            field_0x0360: Default::default(),
            field_0x0368: Default::default(),
            field_0x0398: Default::default(),
        }
    }

    /// Decompiled virtual method #0 @ RVA 0x0066b9d0 (117 instructions, 11 basic blocks)
    pub fn method_00(&mut self) {
    }

    /// Decompiled virtual method #1 @ RVA 0x0066be10 (18 instructions, 1 basic blocks)
    pub fn method_01(&mut self) {
    }

    /// Decompiled virtual method #2 @ RVA 0x000d3090 (1 instructions, 1 basic blocks)
    pub fn method_02(&mut self) {
    }

    /// Decompiled virtual method #3 @ RVA 0x0066bd20 (47 instructions, 4 basic blocks)
    pub fn method_03(&mut self) {
    }

    /// Decompiled virtual method #4 @ RVA 0x00667700 (24 instructions, 3 basic blocks)
    pub fn method_04(&mut self) {
        // Writes to field at offset +0x0000
    }

    /// Decompiled virtual method #5 @ RVA 0x000d3090 (1 instructions, 1 basic blocks)
    pub fn method_05(&mut self) {
    }

    /// Decompiled virtual method #6 @ RVA 0x0066b940 (19 instructions, 3 basic blocks)
    pub fn method_06(&mut self) {
    }

    /// Decompiled virtual method #7 @ RVA 0x0066b990 (15 instructions, 4 basic blocks)
    pub fn method_07(&mut self) {
    }

    /// Decompiled virtual method #8 @ RVA 0x006687a0 (123 instructions, 17 basic blocks)
    pub fn method_08(&mut self) {
        // Writes to field at offset +0x0048
        // Writes to field at offset +0x0090
    }

    /// Decompiled virtual method #9 @ RVA 0x00668b10 (114 instructions, 9 basic blocks)
    pub fn method_09(&mut self) {
        // Writes to field at offset +0x0360
        // Writes to field at offset +0x0368
        // Writes to field at offset +0x0398
    }

    /// Decompiled virtual method #10 @ RVA 0x00668d80 (99 instructions, 5 basic blocks)
    pub fn method_10(&mut self) {
        // Writes to field at offset +0x0360
        // Writes to field at offset +0x0368
        // Writes to field at offset +0x0398
    }

}

impl CcClass for ccAPKWeb {
    fn class_name(&self) -> &'static str { Self::CLASS_NAME }
    fn vtable_rva(&self) -> u32 { Self::VTABLE_RVA }
    fn virtual_method_count(&self) -> usize { Self::METHOD_COUNT }
    fn base_classes(&self) -> &'static [&'static str] { Self::BASE_CLASSES }
}

/// Decompiled CC2 class: `ccAvatarCursor` (VTable RVA: 0x0087c8f0, 12 virtual methods, 13 recovered fields)
#[derive(Debug, Clone, Default)]
pub struct ccAvatarCursor {
    pub vtable_rva: u32,
    pub field_0x0000: u64,
    pub field_0x0010: u32,
    pub field_0x001a: u8,
    pub field_0x0038: u32,
    pub field_0x0040: u32,
    pub field_0x0044: u32,
    pub field_0x0060: u32,
    pub field_0x0094: f32,
    pub field_0x00b4: f32,
    pub field_0x00d0: u64,
    pub field_0x0198: u64,
    pub field_0x01b0: u64,
    pub field_0x01b8: u64,
}

impl ccAvatarCursor {
    pub const CLASS_NAME: &'static str = "ccAvatarCursor";
    pub const VTABLE_RVA: u32 = 0x0087c8f0;
    pub const METHOD_COUNT: usize = 12;
    pub const BASE_CLASSES: &'static [&'static str] = &[];
    pub const VIRTUAL_METHODS: &'static [u64] = &[0x00000001801d2b30, 0x00000001801d2b80, 0x00000001801d2f00, 0x00000001801d2fe0, 0x00000001801d36b0, 0x00000001801d3730, 0x00000001801d45b0, 0x00000001801c9de0, 0x00000001801d3920, 0x00000001801d4460, 0x00000001800d3090, 0x00000001800d3090];

    pub fn new() -> Self {
        Self {
            vtable_rva: Self::VTABLE_RVA,
            field_0x0000: Default::default(),
            field_0x0010: Default::default(),
            field_0x001a: Default::default(),
            field_0x0038: Default::default(),
            field_0x0040: Default::default(),
            field_0x0044: Default::default(),
            field_0x0060: Default::default(),
            field_0x0094: Default::default(),
            field_0x00b4: Default::default(),
            field_0x00d0: Default::default(),
            field_0x0198: Default::default(),
            field_0x01b0: Default::default(),
            field_0x01b8: Default::default(),
        }
    }

    /// Decompiled virtual method #0 @ RVA 0x001d2b30 (21 instructions, 3 basic blocks)
    pub fn method_00(&mut self) {
        // Writes to field at offset +0x0000
    }

    /// Decompiled virtual method #1 @ RVA 0x001d2b80 (111 instructions, 17 basic blocks)
    pub fn method_01(&mut self) {
    }

    /// Decompiled virtual method #2 @ RVA 0x001d2f00 (53 instructions, 17 basic blocks)
    pub fn method_02(&mut self) {
    }

    /// Decompiled virtual method #3 @ RVA 0x001d2fe0 (106 instructions, 18 basic blocks)
    pub fn method_03(&mut self) {
    }

    /// Decompiled virtual method #4 @ RVA 0x001d36b0 (16 instructions, 5 basic blocks)
    pub fn method_04(&mut self) {
        // Writes to field at offset +0x0010
        // Writes to field at offset +0x0094
        // Writes to field at offset +0x0094
    }

    /// Decompiled virtual method #5 @ RVA 0x001d3730 (52 instructions, 11 basic blocks)
    pub fn method_05(&mut self) {
    }

    /// Decompiled virtual method #6 @ RVA 0x001d45b0 (91 instructions, 16 basic blocks)
    pub fn method_06(&mut self) {
        // Writes to field at offset +0x001a
    }

    /// Decompiled virtual method #7 @ RVA 0x001c9de0 (2 instructions, 1 basic blocks)
    pub fn method_07(&mut self) {
    }

    /// Decompiled virtual method #8 @ RVA 0x001d3920 (97 instructions, 14 basic blocks)
    pub fn method_08(&mut self) {
        // Writes to field at offset +0x0040
        // Writes to field at offset +0x0044
        // Writes to field at offset +0x00d0
    }

    /// Decompiled virtual method #9 @ RVA 0x001d4460 (44 instructions, 6 basic blocks)
    pub fn method_09(&mut self) {
    }

    /// Decompiled virtual method #10 @ RVA 0x000d3090 (1 instructions, 1 basic blocks)
    pub fn method_10(&mut self) {
    }

    /// Decompiled virtual method #11 @ RVA 0x000d3090 (1 instructions, 1 basic blocks)
    pub fn method_11(&mut self) {
    }

}

impl CcClass for ccAvatarCursor {
    fn class_name(&self) -> &'static str { Self::CLASS_NAME }
    fn vtable_rva(&self) -> u32 { Self::VTABLE_RVA }
    fn virtual_method_count(&self) -> usize { Self::METHOD_COUNT }
    fn base_classes(&self) -> &'static [&'static str] { Self::BASE_CLASSES }
}

/// Decompiled CC2 class: `ccAvatarCursorSlash` (VTable RVA: 0x0087c888, 12 virtual methods, 18 recovered fields)
/// Inherits: ccAvatarCursor
#[derive(Debug, Clone, Default)]
pub struct ccAvatarCursorSlash {
    pub vtable_rva: u32,
    pub field_0x0000: u64,
    pub field_0x0010: u32,
    pub field_0x0040: u32,
    pub field_0x0044: u32,
    pub field_0x0060: u32,
    pub field_0x0094: f32,
    pub field_0x00b0: u32,
    pub field_0x00b4: u32,
    pub field_0x00d0: u64,
    pub field_0x0120: f32,
    pub field_0x0124: f32,
    pub field_0x0198: u64,
    pub field_0x01a0: u64,
    pub field_0x01a8: u64,
    pub field_0x0360: u32,
    pub field_0x0368: u8,
    pub field_0x0398: [f32; 4],
    pub field_0x0488: u64,
}

impl ccAvatarCursorSlash {
    pub const CLASS_NAME: &'static str = "ccAvatarCursorSlash";
    pub const VTABLE_RVA: u32 = 0x0087c888;
    pub const METHOD_COUNT: usize = 12;
    pub const BASE_CLASSES: &'static [&'static str] = &["ccAvatarCursor"];
    pub const VIRTUAL_METHODS: &'static [u64] = &[0x00000001801d4a60, 0x00000001801d4ab0, 0x00000001801d4df0, 0x00000001801d2fe0, 0x00000001801d4e00, 0x00000001801d4e80, 0x00000001801c9df0, 0x00000001801c9e20, 0x00000001801d3920, 0x00000001801d4460, 0x00000001801c9e40, 0x00000001801c9e50];

    pub fn new() -> Self {
        Self {
            vtable_rva: Self::VTABLE_RVA,
            field_0x0000: Default::default(),
            field_0x0010: Default::default(),
            field_0x0040: Default::default(),
            field_0x0044: Default::default(),
            field_0x0060: Default::default(),
            field_0x0094: Default::default(),
            field_0x00b0: Default::default(),
            field_0x00b4: Default::default(),
            field_0x00d0: Default::default(),
            field_0x0120: Default::default(),
            field_0x0124: Default::default(),
            field_0x0198: Default::default(),
            field_0x01a0: Default::default(),
            field_0x01a8: Default::default(),
            field_0x0360: Default::default(),
            field_0x0368: Default::default(),
            field_0x0398: Default::default(),
            field_0x0488: Default::default(),
        }
    }

    /// Decompiled virtual method #0 @ RVA 0x001d4a60 (21 instructions, 3 basic blocks)
    pub fn method_00(&mut self) {
        // Writes to field at offset +0x0000
    }

    /// Decompiled virtual method #1 @ RVA 0x001d4ab0 (116 instructions, 18 basic blocks)
    pub fn method_01(&mut self) {
    }

    /// Decompiled virtual method #2 @ RVA 0x001d4df0 (28 instructions, 6 basic blocks)
    pub fn method_02(&mut self) {
        // Writes to field at offset +0x0010
        // Writes to field at offset +0x0094
        // Writes to field at offset +0x0094
    }

    /// Decompiled virtual method #3 @ RVA 0x001d2fe0 (106 instructions, 18 basic blocks)
    pub fn method_03(&mut self) {
    }

    /// Decompiled virtual method #4 @ RVA 0x001d4e00 (16 instructions, 5 basic blocks)
    pub fn method_04(&mut self) {
        // Writes to field at offset +0x0010
        // Writes to field at offset +0x0094
        // Writes to field at offset +0x0094
    }

    /// Decompiled virtual method #5 @ RVA 0x001d4e80 (97 instructions, 10 basic blocks)
    pub fn method_05(&mut self) {
        // Writes to field at offset +0x0360
        // Writes to field at offset +0x0368
        // Writes to field at offset +0x0398
    }

    /// Decompiled virtual method #6 @ RVA 0x001c9df0 (24 instructions, 6 basic blocks)
    pub fn method_06(&mut self) {
        // Writes to field at offset +0x0120
        // Writes to field at offset +0x0124
    }

    /// Decompiled virtual method #7 @ RVA 0x001c9e20 (3 instructions, 1 basic blocks)
    pub fn method_07(&mut self) {
    }

    /// Decompiled virtual method #8 @ RVA 0x001d3920 (97 instructions, 14 basic blocks)
    pub fn method_08(&mut self) {
        // Writes to field at offset +0x0040
        // Writes to field at offset +0x0044
        // Writes to field at offset +0x00d0
    }

    /// Decompiled virtual method #9 @ RVA 0x001d4460 (44 instructions, 6 basic blocks)
    pub fn method_09(&mut self) {
    }

    /// Decompiled virtual method #10 @ RVA 0x001c9e40 (2 instructions, 1 basic blocks)
    pub fn method_10(&mut self) {
        // Writes to field at offset +0x00b0
    }

    /// Decompiled virtual method #11 @ RVA 0x001c9e50 (2 instructions, 1 basic blocks)
    pub fn method_11(&mut self) {
        // Writes to field at offset +0x00b0
    }

}

impl CcClass for ccAvatarCursorSlash {
    fn class_name(&self) -> &'static str { Self::CLASS_NAME }
    fn vtable_rva(&self) -> u32 { Self::VTABLE_RVA }
    fn virtual_method_count(&self) -> usize { Self::METHOD_COUNT }
    fn base_classes(&self) -> &'static [&'static str] { Self::BASE_CLASSES }
}

/// Decompiled CC2 class: `ccAvatarStreamPlayKitPlugin` (VTable RVA: 0x0087d5e8, 5 virtual methods, 1 recovered fields)
/// Inherits: ccAvatarStreamPlayPlugin
#[derive(Debug, Clone, Default)]
pub struct ccAvatarStreamPlayKitPlugin {
    pub vtable_rva: u32,
    pub field_0x00a0: u32,
}

impl ccAvatarStreamPlayKitPlugin {
    pub const CLASS_NAME: &'static str = "ccAvatarStreamPlayKitPlugin";
    pub const VTABLE_RVA: u32 = 0x0087d5e8;
    pub const METHOD_COUNT: usize = 5;
    pub const BASE_CLASSES: &'static [&'static str] = &["ccAvatarStreamPlayPlugin"];
    pub const VIRTUAL_METHODS: &'static [u64] = &[0x00000001800d3090, 0x00000001801ece70, 0x00000001801ed160, 0x00000001801ed1c0, 0x00000001800d6920];

    pub fn new() -> Self {
        Self {
            vtable_rva: Self::VTABLE_RVA,
            field_0x00a0: Default::default(),
        }
    }

    /// Decompiled virtual method #0 @ RVA 0x000d3090 (1 instructions, 1 basic blocks)
    pub fn method_00(&mut self) {
    }

    /// Decompiled virtual method #1 @ RVA 0x001ece70 (122 instructions, 17 basic blocks)
    pub fn method_01(&mut self) {
    }

    /// Decompiled virtual method #2 @ RVA 0x001ed160 (25 instructions, 5 basic blocks)
    pub fn method_02(&mut self) {
    }

    /// Decompiled virtual method #3 @ RVA 0x001ed1c0 (43 instructions, 7 basic blocks)
    pub fn method_03(&mut self) {
    }

    /// Decompiled virtual method #4 @ RVA 0x000d6920 (2 instructions, 1 basic blocks)
    pub fn method_04(&mut self) {
    }

}

impl CcClass for ccAvatarStreamPlayKitPlugin {
    fn class_name(&self) -> &'static str { Self::CLASS_NAME }
    fn vtable_rva(&self) -> u32 { Self::VTABLE_RVA }
    fn virtual_method_count(&self) -> usize { Self::METHOD_COUNT }
    fn base_classes(&self) -> &'static [&'static str] { Self::BASE_CLASSES }
}

/// Decompiled CC2 class: `ccAvatarStreamPlayPlugin` (VTable RVA: 0x0087d588, 5 virtual methods, 0 recovered fields)
#[derive(Debug, Clone, Default)]
pub struct ccAvatarStreamPlayPlugin {
    pub vtable_rva: u32,
}

impl ccAvatarStreamPlayPlugin {
    pub const CLASS_NAME: &'static str = "ccAvatarStreamPlayPlugin";
    pub const VTABLE_RVA: u32 = 0x0087d588;
    pub const METHOD_COUNT: usize = 5;
    pub const BASE_CLASSES: &'static [&'static str] = &[];
    pub const VIRTUAL_METHODS: &'static [u64] = &[0x00000001800d3090, 0x00000001801ecb80, 0x00000001800d3090, 0x00000001800d3090, 0x00000001800d6920];

    pub fn new() -> Self {
        Self {
            vtable_rva: Self::VTABLE_RVA,
        }
    }

    /// Decompiled virtual method #0 @ RVA 0x000d3090 (1 instructions, 1 basic blocks)
    pub fn method_00(&mut self) {
    }

    /// Decompiled virtual method #1 @ RVA 0x001ecb80 (18 instructions, 2 basic blocks)
    pub fn method_01(&mut self) {
    }

    /// Decompiled virtual method #2 @ RVA 0x000d3090 (1 instructions, 1 basic blocks)
    pub fn method_02(&mut self) {
    }

    /// Decompiled virtual method #3 @ RVA 0x000d3090 (1 instructions, 1 basic blocks)
    pub fn method_03(&mut self) {
    }

    /// Decompiled virtual method #4 @ RVA 0x000d6920 (2 instructions, 1 basic blocks)
    pub fn method_04(&mut self) {
    }

}

impl CcClass for ccAvatarStreamPlayPlugin {
    fn class_name(&self) -> &'static str { Self::CLASS_NAME }
    fn vtable_rva(&self) -> u32 { Self::VTABLE_RVA }
    fn virtual_method_count(&self) -> usize { Self::METHOD_COUNT }
    fn base_classes(&self) -> &'static [&'static str] { Self::BASE_CLASSES }
}

/// Decompiled CC2 class: `ccAvatarStreamPlaySkePlugin` (VTable RVA: 0x0087d5b8, 5 virtual methods, 0 recovered fields)
/// Inherits: ccAvatarStreamPlayPlugin
#[derive(Debug, Clone, Default)]
pub struct ccAvatarStreamPlaySkePlugin {
    pub vtable_rva: u32,
}

impl ccAvatarStreamPlaySkePlugin {
    pub const CLASS_NAME: &'static str = "ccAvatarStreamPlaySkePlugin";
    pub const VTABLE_RVA: u32 = 0x0087d5b8;
    pub const METHOD_COUNT: usize = 5;
    pub const BASE_CLASSES: &'static [&'static str] = &["ccAvatarStreamPlayPlugin"];
    pub const VIRTUAL_METHODS: &'static [u64] = &[0x00000001800d3090, 0x00000001801ecbf0, 0x00000001800d3090, 0x00000001800d3090, 0x00000001800d6920];

    pub fn new() -> Self {
        Self {
            vtable_rva: Self::VTABLE_RVA,
        }
    }

    /// Decompiled virtual method #0 @ RVA 0x000d3090 (1 instructions, 1 basic blocks)
    pub fn method_00(&mut self) {
    }

    /// Decompiled virtual method #1 @ RVA 0x001ecbf0 (91 instructions, 14 basic blocks)
    pub fn method_01(&mut self) {
    }

    /// Decompiled virtual method #2 @ RVA 0x000d3090 (1 instructions, 1 basic blocks)
    pub fn method_02(&mut self) {
    }

    /// Decompiled virtual method #3 @ RVA 0x000d3090 (1 instructions, 1 basic blocks)
    pub fn method_03(&mut self) {
    }

    /// Decompiled virtual method #4 @ RVA 0x000d6920 (2 instructions, 1 basic blocks)
    pub fn method_04(&mut self) {
    }

}

impl CcClass for ccAvatarStreamPlaySkePlugin {
    fn class_name(&self) -> &'static str { Self::CLASS_NAME }
    fn vtable_rva(&self) -> u32 { Self::VTABLE_RVA }
    fn virtual_method_count(&self) -> usize { Self::METHOD_COUNT }
    fn base_classes(&self) -> &'static [&'static str] { Self::BASE_CLASSES }
}

/// Decompiled CC2 class: `ccAvatarStreamPlayVibPadPlugin` (VTable RVA: 0x0087d618, 5 virtual methods, 2 recovered fields)
/// Inherits: ccAvatarStreamPlayPlugin
#[derive(Debug, Clone, Default)]
pub struct ccAvatarStreamPlayVibPadPlugin {
    pub vtable_rva: u32,
    pub field_0x0008: u32,
    pub field_0x0010: u64,
}

impl ccAvatarStreamPlayVibPadPlugin {
    pub const CLASS_NAME: &'static str = "ccAvatarStreamPlayVibPadPlugin";
    pub const VTABLE_RVA: u32 = 0x0087d618;
    pub const METHOD_COUNT: usize = 5;
    pub const BASE_CLASSES: &'static [&'static str] = &["ccAvatarStreamPlayPlugin"];
    pub const VIRTUAL_METHODS: &'static [u64] = &[0x00000001801ed3a0, 0x00000001801ed3f0, 0x00000001800d3090, 0x00000001801ed430, 0x00000001801ed440];

    pub fn new() -> Self {
        Self {
            vtable_rva: Self::VTABLE_RVA,
            field_0x0008: Default::default(),
            field_0x0010: Default::default(),
        }
    }

    /// Decompiled virtual method #0 @ RVA 0x001ed3a0 (15 instructions, 5 basic blocks)
    pub fn method_00(&mut self) {
    }

    /// Decompiled virtual method #1 @ RVA 0x001ed3f0 (11 instructions, 2 basic blocks)
    pub fn method_01(&mut self) {
    }

    /// Decompiled virtual method #2 @ RVA 0x000d3090 (1 instructions, 1 basic blocks)
    pub fn method_02(&mut self) {
    }

    /// Decompiled virtual method #3 @ RVA 0x001ed430 (3 instructions, 1 basic blocks)
    pub fn method_03(&mut self) {
        // Writes to field at offset +0x0008
        // Writes to field at offset +0x0010
    }

    /// Decompiled virtual method #4 @ RVA 0x001ed440 (4 instructions, 1 basic blocks)
    pub fn method_04(&mut self) {
        // Writes to field at offset +0x0008
    }

}

impl CcClass for ccAvatarStreamPlayVibPadPlugin {
    fn class_name(&self) -> &'static str { Self::CLASS_NAME }
    fn vtable_rva(&self) -> u32 { Self::VTABLE_RVA }
    fn virtual_method_count(&self) -> usize { Self::METHOD_COUNT }
    fn base_classes(&self) -> &'static [&'static str] { Self::BASE_CLASSES }
}

/// Decompiled CC2 class: `ccBikeFrame` (VTable RVA: 0x008bc620, 2 virtual methods, 24 recovered fields)
#[derive(Debug, Clone, Default)]
pub struct ccBikeFrame {
    pub vtable_rva: u32,
    pub field_0x0020: u32,
    pub field_0x0030: u64,
    pub field_0x0038: u32,
    pub field_0x003c: u32,
    pub field_0x0044: u32,
    pub field_0x0048: u32,
    pub field_0x004c: f32,
    pub field_0x0058: f32,
    pub field_0x005c: u32,
    pub field_0x0060: u32,
    pub field_0x0068: u8,
    pub field_0x0070: u32,
    pub field_0x0074: u32,
    pub field_0x0078: u32,
    pub field_0x0080: f32,
    pub field_0x0084: f32,
    pub field_0x0088: f32,
    pub field_0x0090: u32,
    pub field_0x0094: u32,
    pub field_0x00a0: u32,
    pub field_0x00a4: u32,
    pub field_0x00a8: u32,
    pub field_0x00b0: f32,
    pub field_0x00b8: u32,
}

impl ccBikeFrame {
    pub const CLASS_NAME: &'static str = "ccBikeFrame";
    pub const VTABLE_RVA: u32 = 0x008bc620;
    pub const METHOD_COUNT: usize = 2;
    pub const BASE_CLASSES: &'static [&'static str] = &[];
    pub const VIRTUAL_METHODS: &'static [u64] = &[0x00000001804980e0, 0x00000001804983f0];

    pub fn new() -> Self {
        Self {
            vtable_rva: Self::VTABLE_RVA,
            field_0x0020: Default::default(),
            field_0x0030: Default::default(),
            field_0x0038: Default::default(),
            field_0x003c: Default::default(),
            field_0x0044: Default::default(),
            field_0x0048: Default::default(),
            field_0x004c: Default::default(),
            field_0x0058: Default::default(),
            field_0x005c: Default::default(),
            field_0x0060: Default::default(),
            field_0x0068: Default::default(),
            field_0x0070: Default::default(),
            field_0x0074: Default::default(),
            field_0x0078: Default::default(),
            field_0x0080: Default::default(),
            field_0x0084: Default::default(),
            field_0x0088: Default::default(),
            field_0x0090: Default::default(),
            field_0x0094: Default::default(),
            field_0x00a0: Default::default(),
            field_0x00a4: Default::default(),
            field_0x00a8: Default::default(),
            field_0x00b0: Default::default(),
            field_0x00b8: Default::default(),
        }
    }

    /// Decompiled virtual method #0 @ RVA 0x004980e0 (104 instructions, 12 basic blocks)
    pub fn method_00(&mut self) {
        // Writes to field at offset +0x0030
        // Writes to field at offset +0x0038
        // Writes to field at offset +0x003c
        // Writes to field at offset +0x0068
    }

    /// Decompiled virtual method #1 @ RVA 0x004983f0 (57 instructions, 16 basic blocks)
    pub fn method_01(&mut self) {
        // Writes to field at offset +0x0030
        // Writes to field at offset +0x0038
        // Writes to field at offset +0x003c
        // Writes to field at offset +0x0044
        // Writes to field at offset +0x004c
        // Writes to field at offset +0x0058
        // Writes to field at offset +0x005c
        // Writes to field at offset +0x0068
        // Writes to field at offset +0x0080
        // Writes to field at offset +0x0084
        // Writes to field at offset +0x0088
        // Writes to field at offset +0x0090
        // Writes to field at offset +0x0094
    }

}

impl CcClass for ccBikeFrame {
    fn class_name(&self) -> &'static str { Self::CLASS_NAME }
    fn vtable_rva(&self) -> u32 { Self::VTABLE_RVA }
    fn virtual_method_count(&self) -> usize { Self::METHOD_COUNT }
    fn base_classes(&self) -> &'static [&'static str] { Self::BASE_CLASSES }
}

/// Decompiled CC2 class: `ccBtnAPK` (VTable RVA: 0x008fb228, 9 virtual methods, 32 recovered fields)
/// Inherits: ccWebButtonBase
#[derive(Debug, Clone, Default)]
pub struct ccBtnAPK {
    pub vtable_rva: u32,
    pub field_0x0000: u64,
    pub field_0x000c: u32,
    pub field_0x0018: u32,
    pub field_0x001c: u32,
    pub field_0x0020: u32,
    pub field_0x0024: u32,
    pub field_0x0028: u32,
    pub field_0x002c: u32,
    pub field_0x0030: u32,
    pub field_0x0034: u32,
    pub field_0x0038: u32,
    pub field_0x003c: u32,
    pub field_0x0040: u32,
    pub field_0x0044: u32,
    pub field_0x0048: u32,
    pub field_0x004c: u8,
    pub field_0x009c: f32,
    pub field_0x00a0: f32,
    pub field_0x00a4: u32,
    pub field_0x00a8: u32,
    pub field_0x00ac: u32,
    pub field_0x00b0: u32,
    pub field_0x00b4: u32,
    pub field_0x00b8: u32,
    pub field_0x00bc: u32,
    pub field_0x00c0: u32,
    pub field_0x00c4: u32,
    pub field_0x00cc: u32,
    pub field_0x00d0: u32,
    pub field_0x00d4: u32,
    pub field_0x00d8: u8,
    pub field_0x00e0: u64,
}

impl ccBtnAPK {
    pub const CLASS_NAME: &'static str = "ccBtnAPK";
    pub const VTABLE_RVA: u32 = 0x008fb228;
    pub const METHOD_COUNT: usize = 9;
    pub const BASE_CLASSES: &'static [&'static str] = &["ccWebButtonBase"];
    pub const VIRTUAL_METHODS: &'static [u64] = &[0x000000018066b820, 0x00000001800d3090, 0x000000018066b920, 0x00000001806676c0, 0x0000000180667f40, 0x0000000180667f60, 0x0000000180667f20, 0x0000000180667cd0, 0x0000000180667ea0];

    pub fn new() -> Self {
        Self {
            vtable_rva: Self::VTABLE_RVA,
            field_0x0000: Default::default(),
            field_0x000c: Default::default(),
            field_0x0018: Default::default(),
            field_0x001c: Default::default(),
            field_0x0020: Default::default(),
            field_0x0024: Default::default(),
            field_0x0028: Default::default(),
            field_0x002c: Default::default(),
            field_0x0030: Default::default(),
            field_0x0034: Default::default(),
            field_0x0038: Default::default(),
            field_0x003c: Default::default(),
            field_0x0040: Default::default(),
            field_0x0044: Default::default(),
            field_0x0048: Default::default(),
            field_0x004c: Default::default(),
            field_0x009c: Default::default(),
            field_0x00a0: Default::default(),
            field_0x00a4: Default::default(),
            field_0x00a8: Default::default(),
            field_0x00ac: Default::default(),
            field_0x00b0: Default::default(),
            field_0x00b4: Default::default(),
            field_0x00b8: Default::default(),
            field_0x00bc: Default::default(),
            field_0x00c0: Default::default(),
            field_0x00c4: Default::default(),
            field_0x00cc: Default::default(),
            field_0x00d0: Default::default(),
            field_0x00d4: Default::default(),
            field_0x00d8: Default::default(),
            field_0x00e0: Default::default(),
        }
    }

    /// Decompiled virtual method #0 @ RVA 0x0066b820 (59 instructions, 14 basic blocks)
    pub fn method_00(&mut self) {
        // Writes to field at offset +0x000c
        // Writes to field at offset +0x0018
        // Writes to field at offset +0x001c
        // Writes to field at offset +0x0020
        // Writes to field at offset +0x0024
        // Writes to field at offset +0x0028
        // Writes to field at offset +0x002c
        // Writes to field at offset +0x0030
        // Writes to field at offset +0x0034
        // Writes to field at offset +0x004c
        // Writes to field at offset +0x00a4
        // Writes to field at offset +0x00a8
        // Writes to field at offset +0x00ac
        // Writes to field at offset +0x00b0
        // Writes to field at offset +0x00b4
        // Writes to field at offset +0x00b8
        // Writes to field at offset +0x00bc
        // Writes to field at offset +0x00c0
        // Writes to field at offset +0x00d8
    }

    /// Decompiled virtual method #1 @ RVA 0x000d3090 (1 instructions, 1 basic blocks)
    pub fn method_01(&mut self) {
    }

    /// Decompiled virtual method #2 @ RVA 0x0066b920 (5 instructions, 3 basic blocks)
    pub fn method_02(&mut self) {
        // Writes to field at offset +0x00d8
    }

    /// Decompiled virtual method #3 @ RVA 0x006676c0 (16 instructions, 3 basic blocks)
    pub fn method_03(&mut self) {
    }

    /// Decompiled virtual method #4 @ RVA 0x00667f40 (3 instructions, 1 basic blocks)
    pub fn method_04(&mut self) {
        // Writes to field at offset +0x00c4
        // Writes to field at offset +0x00d8
    }

    /// Decompiled virtual method #5 @ RVA 0x00667f60 (2 instructions, 1 basic blocks)
    pub fn method_05(&mut self) {
        // Writes to field at offset +0x00d8
    }

    /// Decompiled virtual method #6 @ RVA 0x00667f20 (5 instructions, 1 basic blocks)
    pub fn method_06(&mut self) {
    }

    /// Decompiled virtual method #7 @ RVA 0x00667cd0 (118 instructions, 18 basic blocks)
    pub fn method_07(&mut self) {
        // Writes to field at offset +0x0040
        // Writes to field at offset +0x0044
        // Writes to field at offset +0x0048
        // Writes to field at offset +0x00c4
        // Writes to field at offset +0x00c4
        // Writes to field at offset +0x00cc
    }

    /// Decompiled virtual method #8 @ RVA 0x00667ea0 (52 instructions, 6 basic blocks)
    pub fn method_08(&mut self) {
    }

}

impl CcClass for ccBtnAPK {
    fn class_name(&self) -> &'static str { Self::CLASS_NAME }
    fn vtable_rva(&self) -> u32 { Self::VTABLE_RVA }
    fn virtual_method_count(&self) -> usize { Self::METHOD_COUNT }
    fn base_classes(&self) -> &'static [&'static str] { Self::BASE_CLASSES }
}

/// Decompiled CC2 class: `ccBtnOffSite` (VTable RVA: 0x008fb178, 9 virtual methods, 17 recovered fields)
/// Inherits: ccWebButtonBase
#[derive(Debug, Clone, Default)]
pub struct ccBtnOffSite {
    pub vtable_rva: u32,
    pub field_0x0000: u64,
    pub field_0x0008: u64,
    pub field_0x0040: u32,
    pub field_0x0044: f32,
    pub field_0x0048: u32,
    pub field_0x00a4: f32,
    pub field_0x00a8: u32,
    pub field_0x00b4: f32,
    pub field_0x00b8: u32,
    pub field_0x00c4: u32,
    pub field_0x00cc: u32,
    pub field_0x00d0: u32,
    pub field_0x00d4: u32,
    pub field_0x00d8: u8,
    pub field_0x00e0: u64,
    pub field_0x00f8: f32,
    pub field_0x0100: u8,
}

impl ccBtnOffSite {
    pub const CLASS_NAME: &'static str = "ccBtnOffSite";
    pub const VTABLE_RVA: u32 = 0x008fb178;
    pub const METHOD_COUNT: usize = 9;
    pub const BASE_CLASSES: &'static [&'static str] = &["ccWebButtonBase"];
    pub const VIRTUAL_METHODS: &'static [u64] = &[0x000000018066d190, 0x00000001800d3090, 0x00000001800d3090, 0x0000000180667810, 0x000000018066d3d0, 0x0000000180667f60, 0x0000000180667f20, 0x0000000180667cd0, 0x000000018066d360];

    pub fn new() -> Self {
        Self {
            vtable_rva: Self::VTABLE_RVA,
            field_0x0000: Default::default(),
            field_0x0008: Default::default(),
            field_0x0040: Default::default(),
            field_0x0044: Default::default(),
            field_0x0048: Default::default(),
            field_0x00a4: Default::default(),
            field_0x00a8: Default::default(),
            field_0x00b4: Default::default(),
            field_0x00b8: Default::default(),
            field_0x00c4: Default::default(),
            field_0x00cc: Default::default(),
            field_0x00d0: Default::default(),
            field_0x00d4: Default::default(),
            field_0x00d8: Default::default(),
            field_0x00e0: Default::default(),
            field_0x00f8: Default::default(),
            field_0x0100: Default::default(),
        }
    }

    /// Decompiled virtual method #0 @ RVA 0x0066d190 (96 instructions, 12 basic blocks)
    pub fn method_00(&mut self) {
        // Writes to field at offset +0x00a4
        // Writes to field at offset +0x00a8
        // Writes to field at offset +0x00b4
        // Writes to field at offset +0x00b8
    }

    /// Decompiled virtual method #1 @ RVA 0x000d3090 (1 instructions, 1 basic blocks)
    pub fn method_01(&mut self) {
    }

    /// Decompiled virtual method #2 @ RVA 0x000d3090 (1 instructions, 1 basic blocks)
    pub fn method_02(&mut self) {
    }

    /// Decompiled virtual method #3 @ RVA 0x00667810 (16 instructions, 3 basic blocks)
    pub fn method_03(&mut self) {
    }

    /// Decompiled virtual method #4 @ RVA 0x0066d3d0 (4 instructions, 1 basic blocks)
    pub fn method_04(&mut self) {
        // Writes to field at offset +0x00c4
        // Writes to field at offset +0x00d8
        // Writes to field at offset +0x0100
    }

    /// Decompiled virtual method #5 @ RVA 0x00667f60 (2 instructions, 1 basic blocks)
    pub fn method_05(&mut self) {
        // Writes to field at offset +0x00d8
    }

    /// Decompiled virtual method #6 @ RVA 0x00667f20 (5 instructions, 1 basic blocks)
    pub fn method_06(&mut self) {
    }

    /// Decompiled virtual method #7 @ RVA 0x00667cd0 (118 instructions, 18 basic blocks)
    pub fn method_07(&mut self) {
        // Writes to field at offset +0x0040
        // Writes to field at offset +0x0044
        // Writes to field at offset +0x0048
        // Writes to field at offset +0x00c4
        // Writes to field at offset +0x00c4
        // Writes to field at offset +0x00cc
    }

    /// Decompiled virtual method #8 @ RVA 0x0066d360 (41 instructions, 6 basic blocks)
    pub fn method_08(&mut self) {
        // Writes to field at offset +0x00c4
        // Writes to field at offset +0x00d8
        // Writes to field at offset +0x0100
    }

}

impl CcClass for ccBtnOffSite {
    fn class_name(&self) -> &'static str { Self::CLASS_NAME }
    fn vtable_rva(&self) -> u32 { Self::VTABLE_RVA }
    fn virtual_method_count(&self) -> usize { Self::METHOD_COUNT }
    fn base_classes(&self) -> &'static [&'static str] { Self::BASE_CLASSES }
}

/// Decompiled CC2 class: `ccBtnYM` (VTable RVA: 0x008fb328, 9 virtual methods, 37 recovered fields)
/// Inherits: ccWebButtonBase
#[derive(Debug, Clone, Default)]
pub struct ccBtnYM {
    pub vtable_rva: u32,
    pub field_0x0000: u64,
    pub field_0x000c: u32,
    pub field_0x0018: u32,
    pub field_0x001c: u32,
    pub field_0x0020: u32,
    pub field_0x0024: u32,
    pub field_0x0028: u32,
    pub field_0x002c: u32,
    pub field_0x0030: u32,
    pub field_0x0034: u32,
    pub field_0x0038: u32,
    pub field_0x003c: u32,
    pub field_0x0040: u32,
    pub field_0x0044: u32,
    pub field_0x0048: u32,
    pub field_0x004c: u8,
    pub field_0x00a4: u32,
    pub field_0x00a8: u32,
    pub field_0x00ac: u32,
    pub field_0x00b0: u32,
    pub field_0x00b4: u32,
    pub field_0x00b8: u32,
    pub field_0x00bc: u32,
    pub field_0x00c0: u32,
    pub field_0x00c4: u32,
    pub field_0x00cc: u32,
    pub field_0x00d0: u32,
    pub field_0x00d4: u32,
    pub field_0x00d8: u8,
    pub field_0x00e0: u64,
    pub field_0x00e8: u64,
    pub field_0x00f0: f32,
    pub field_0x00f4: u32,
    pub field_0x00f8: u32,
    pub field_0x00fc: u32,
    pub field_0x0100: u32,
    pub field_0x0104: f32,
}

impl ccBtnYM {
    pub const CLASS_NAME: &'static str = "ccBtnYM";
    pub const VTABLE_RVA: u32 = 0x008fb328;
    pub const METHOD_COUNT: usize = 9;
    pub const BASE_CLASSES: &'static [&'static str] = &["ccWebButtonBase"];
    pub const VIRTUAL_METHODS: &'static [u64] = &[0x000000018066a190, 0x000000018066a3d0, 0x000000018066a3b0, 0x0000000180667400, 0x0000000180667f40, 0x0000000180667f60, 0x0000000180667f20, 0x0000000180667cd0, 0x0000000180667ea0];

    pub fn new() -> Self {
        Self {
            vtable_rva: Self::VTABLE_RVA,
            field_0x0000: Default::default(),
            field_0x000c: Default::default(),
            field_0x0018: Default::default(),
            field_0x001c: Default::default(),
            field_0x0020: Default::default(),
            field_0x0024: Default::default(),
            field_0x0028: Default::default(),
            field_0x002c: Default::default(),
            field_0x0030: Default::default(),
            field_0x0034: Default::default(),
            field_0x0038: Default::default(),
            field_0x003c: Default::default(),
            field_0x0040: Default::default(),
            field_0x0044: Default::default(),
            field_0x0048: Default::default(),
            field_0x004c: Default::default(),
            field_0x00a4: Default::default(),
            field_0x00a8: Default::default(),
            field_0x00ac: Default::default(),
            field_0x00b0: Default::default(),
            field_0x00b4: Default::default(),
            field_0x00b8: Default::default(),
            field_0x00bc: Default::default(),
            field_0x00c0: Default::default(),
            field_0x00c4: Default::default(),
            field_0x00cc: Default::default(),
            field_0x00d0: Default::default(),
            field_0x00d4: Default::default(),
            field_0x00d8: Default::default(),
            field_0x00e0: Default::default(),
            field_0x00e8: Default::default(),
            field_0x00f0: Default::default(),
            field_0x00f4: Default::default(),
            field_0x00f8: Default::default(),
            field_0x00fc: Default::default(),
            field_0x0100: Default::default(),
            field_0x0104: Default::default(),
        }
    }

    /// Decompiled virtual method #0 @ RVA 0x0066a190 (96 instructions, 15 basic blocks)
    pub fn method_00(&mut self) {
        // Writes to field at offset +0x000c
        // Writes to field at offset +0x0018
        // Writes to field at offset +0x001c
        // Writes to field at offset +0x0020
        // Writes to field at offset +0x0024
        // Writes to field at offset +0x0028
        // Writes to field at offset +0x002c
        // Writes to field at offset +0x0030
        // Writes to field at offset +0x0034
        // Writes to field at offset +0x0040
        // Writes to field at offset +0x0044
        // Writes to field at offset +0x0048
        // Writes to field at offset +0x004c
        // Writes to field at offset +0x00a4
        // Writes to field at offset +0x00a8
        // Writes to field at offset +0x00ac
        // Writes to field at offset +0x00b0
        // Writes to field at offset +0x00b4
        // Writes to field at offset +0x00b8
        // Writes to field at offset +0x00bc
        // Writes to field at offset +0x00c0
        // Writes to field at offset +0x00d8
        // Writes to field at offset +0x00f4
        // Writes to field at offset +0x00f8
        // Writes to field at offset +0x00fc
        // Writes to field at offset +0x0100
        // Writes to field at offset +0x0104
    }

    /// Decompiled virtual method #1 @ RVA 0x0066a3d0 (4 instructions, 2 basic blocks)
    pub fn method_01(&mut self) {
    }

    /// Decompiled virtual method #2 @ RVA 0x0066a3b0 (5 instructions, 3 basic blocks)
    pub fn method_02(&mut self) {
        // Writes to field at offset +0x00d8
    }

    /// Decompiled virtual method #3 @ RVA 0x00667400 (16 instructions, 3 basic blocks)
    pub fn method_03(&mut self) {
    }

    /// Decompiled virtual method #4 @ RVA 0x00667f40 (3 instructions, 1 basic blocks)
    pub fn method_04(&mut self) {
        // Writes to field at offset +0x00c4
        // Writes to field at offset +0x00d8
    }

    /// Decompiled virtual method #5 @ RVA 0x00667f60 (2 instructions, 1 basic blocks)
    pub fn method_05(&mut self) {
        // Writes to field at offset +0x00d8
    }

    /// Decompiled virtual method #6 @ RVA 0x00667f20 (5 instructions, 1 basic blocks)
    pub fn method_06(&mut self) {
    }

    /// Decompiled virtual method #7 @ RVA 0x00667cd0 (118 instructions, 18 basic blocks)
    pub fn method_07(&mut self) {
        // Writes to field at offset +0x0040
        // Writes to field at offset +0x0044
        // Writes to field at offset +0x0048
        // Writes to field at offset +0x00c4
        // Writes to field at offset +0x00c4
        // Writes to field at offset +0x00cc
    }

    /// Decompiled virtual method #8 @ RVA 0x00667ea0 (52 instructions, 6 basic blocks)
    pub fn method_08(&mut self) {
    }

}

impl CcClass for ccBtnYM {
    fn class_name(&self) -> &'static str { Self::CLASS_NAME }
    fn vtable_rva(&self) -> u32 { Self::VTABLE_RVA }
    fn virtual_method_count(&self) -> usize { Self::METHOD_COUNT }
    fn base_classes(&self) -> &'static [&'static str] { Self::BASE_CLASSES }
}

/// Decompiled CC2 class: `ccBtnYM2` (VTable RVA: 0x008fb2d8, 9 virtual methods, 17 recovered fields)
/// Inherits: ccWebButtonBase
#[derive(Debug, Clone, Default)]
pub struct ccBtnYM2 {
    pub vtable_rva: u32,
    pub field_0x0040: u32,
    pub field_0x0044: f32,
    pub field_0x00a4: u32,
    pub field_0x00a8: u32,
    pub field_0x00ac: u32,
    pub field_0x00b0: u32,
    pub field_0x00b4: u32,
    pub field_0x00b8: u32,
    pub field_0x00bc: u32,
    pub field_0x00c0: u32,
    pub field_0x00c4: u32,
    pub field_0x00cc: u32,
    pub field_0x00d0: u32,
    pub field_0x00d4: u32,
    pub field_0x00d8: u8,
    pub field_0x00e0: u64,
    pub field_0x00e8: u32,
}

impl ccBtnYM2 {
    pub const CLASS_NAME: &'static str = "ccBtnYM2";
    pub const VTABLE_RVA: u32 = 0x008fb2d8;
    pub const METHOD_COUNT: usize = 9;
    pub const BASE_CLASSES: &'static [&'static str] = &["ccWebButtonBase"];
    pub const VIRTUAL_METHODS: &'static [u64] = &[0x00000001800d3090, 0x00000001800d3090, 0x00000001800d3090, 0x00000001806674c0, 0x000000018066a8f0, 0x0000000180667f60, 0x0000000180667f20, 0x000000018066a570, 0x000000018066a890];

    pub fn new() -> Self {
        Self {
            vtable_rva: Self::VTABLE_RVA,
            field_0x0040: Default::default(),
            field_0x0044: Default::default(),
            field_0x00a4: Default::default(),
            field_0x00a8: Default::default(),
            field_0x00ac: Default::default(),
            field_0x00b0: Default::default(),
            field_0x00b4: Default::default(),
            field_0x00b8: Default::default(),
            field_0x00bc: Default::default(),
            field_0x00c0: Default::default(),
            field_0x00c4: Default::default(),
            field_0x00cc: Default::default(),
            field_0x00d0: Default::default(),
            field_0x00d4: Default::default(),
            field_0x00d8: Default::default(),
            field_0x00e0: Default::default(),
            field_0x00e8: Default::default(),
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

    /// Decompiled virtual method #3 @ RVA 0x006674c0 (16 instructions, 3 basic blocks)
    pub fn method_03(&mut self) {
    }

    /// Decompiled virtual method #4 @ RVA 0x0066a8f0 (6 instructions, 1 basic blocks)
    pub fn method_04(&mut self) {
        // Writes to field at offset +0x00bc
        // Writes to field at offset +0x00c0
        // Writes to field at offset +0x00c4
        // Writes to field at offset +0x00d8
        // Writes to field at offset +0x00e8
    }

    /// Decompiled virtual method #5 @ RVA 0x00667f60 (2 instructions, 1 basic blocks)
    pub fn method_05(&mut self) {
        // Writes to field at offset +0x00d8
    }

    /// Decompiled virtual method #6 @ RVA 0x00667f20 (5 instructions, 1 basic blocks)
    pub fn method_06(&mut self) {
    }

    /// Decompiled virtual method #7 @ RVA 0x0066a570 (87 instructions, 18 basic blocks)
    pub fn method_07(&mut self) {
        // Writes to field at offset +0x0040
        // Writes to field at offset +0x0044
        // Writes to field at offset +0x00a4
        // Writes to field at offset +0x00a8
        // Writes to field at offset +0x00ac
        // Writes to field at offset +0x00b0
        // Writes to field at offset +0x00b4
        // Writes to field at offset +0x00b8
        // Writes to field at offset +0x00bc
        // Writes to field at offset +0x00c0
        // Writes to field at offset +0x00c4
        // Writes to field at offset +0x00c4
        // Writes to field at offset +0x00cc
        // Writes to field at offset +0x00d8
        // Writes to field at offset +0x00e8
    }

    /// Decompiled virtual method #8 @ RVA 0x0066a890 (24 instructions, 8 basic blocks)
    pub fn method_08(&mut self) {
        // Writes to field at offset +0x00d8
    }

}

impl CcClass for ccBtnYM2 {
    fn class_name(&self) -> &'static str { Self::CLASS_NAME }
    fn vtable_rva(&self) -> u32 { Self::VTABLE_RVA }
    fn virtual_method_count(&self) -> usize { Self::METHOD_COUNT }
    fn base_classes(&self) -> &'static [&'static str] { Self::BASE_CLASSES }
}

/// Decompiled CC2 class: `ccBufferReverce` (VTable RVA: 0x00877af0, 1 virtual methods, 0 recovered fields)
#[derive(Debug, Clone, Default)]
pub struct ccBufferReverce {
    pub vtable_rva: u32,
}

impl ccBufferReverce {
    pub const CLASS_NAME: &'static str = "ccBufferReverce";
    pub const VTABLE_RVA: u32 = 0x00877af0;
    pub const METHOD_COUNT: usize = 1;
    pub const BASE_CLASSES: &'static [&'static str] = &[];
    pub const VIRTUAL_METHODS: &'static [u64] = &[0x0000000180190de0];

    pub fn new() -> Self {
        Self {
            vtable_rva: Self::VTABLE_RVA,
        }
    }

    /// Decompiled virtual method #0 @ RVA 0x00190de0 (16 instructions, 3 basic blocks)
    pub fn method_00(&mut self) {
    }

}

impl CcClass for ccBufferReverce {
    fn class_name(&self) -> &'static str { Self::CLASS_NAME }
    fn vtable_rva(&self) -> u32 { Self::VTABLE_RVA }
    fn virtual_method_count(&self) -> usize { Self::METHOD_COUNT }
    fn base_classes(&self) -> &'static [&'static str] { Self::BASE_CLASSES }
}

/// Decompiled CC2 class: `ccBufferSampling` (VTable RVA: 0x00877b60, 1 virtual methods, 0 recovered fields)
#[derive(Debug, Clone, Default)]
pub struct ccBufferSampling {
    pub vtable_rva: u32,
}

impl ccBufferSampling {
    pub const CLASS_NAME: &'static str = "ccBufferSampling";
    pub const VTABLE_RVA: u32 = 0x00877b60;
    pub const METHOD_COUNT: usize = 1;
    pub const BASE_CLASSES: &'static [&'static str] = &[];
    pub const VIRTUAL_METHODS: &'static [u64] = &[0x0000000180191250];

    pub fn new() -> Self {
        Self {
            vtable_rva: Self::VTABLE_RVA,
        }
    }

    /// Decompiled virtual method #0 @ RVA 0x00191250 (16 instructions, 3 basic blocks)
    pub fn method_00(&mut self) {
    }

}

impl CcClass for ccBufferSampling {
    fn class_name(&self) -> &'static str { Self::CLASS_NAME }
    fn vtable_rva(&self) -> u32 { Self::VTABLE_RVA }
    fn virtual_method_count(&self) -> usize { Self::METHOD_COUNT }
    fn base_classes(&self) -> &'static [&'static str] { Self::BASE_CLASSES }
}

/// Decompiled CC2 class: `ccCharIcon` (VTable RVA: 0x008a4568, 15 virtual methods, 18 recovered fields)
/// Inherits: ccDispBase
#[derive(Debug, Clone, Default)]
pub struct ccCharIcon {
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
    pub field_0x0048: u16,
    pub field_0x004a: u16,
    pub field_0x0050: u64,
    pub field_0x0058: u64,
    pub field_0x0068: u32,
}

impl ccCharIcon {
    pub const CLASS_NAME: &'static str = "ccCharIcon";
    pub const VTABLE_RVA: u32 = 0x008a4568;
    pub const METHOD_COUNT: usize = 15;
    pub const BASE_CLASSES: &'static [&'static str] = &["ccDispBase"];
    pub const VIRTUAL_METHODS: &'static [u64] = &[0x000000018023e480, 0x00000001800d3090, 0x00000001802a2000, 0x00000001802a2060, 0x00000001802a0e30, 0x00000001800d3090, 0x00000001801b81f0, 0x00000001801b8200, 0x00000001801b8210, 0x00000001802a2410, 0x00000001800d3090, 0x00000001800d3090, 0x00000001800d3090, 0x00000001800d3090, 0x00000001802a1f90];

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
            field_0x0048: Default::default(),
            field_0x004a: Default::default(),
            field_0x0050: Default::default(),
            field_0x0058: Default::default(),
            field_0x0068: Default::default(),
        }
    }

    /// Decompiled virtual method #0 @ RVA 0x0023e480 (13 instructions, 3 basic blocks)
    pub fn method_00(&mut self) {
        // Writes to field at offset +0x0000
    }

    /// Decompiled virtual method #1 @ RVA 0x000d3090 (1 instructions, 1 basic blocks)
    pub fn method_01(&mut self) {
    }

    /// Decompiled virtual method #2 @ RVA 0x002a2000 (122 instructions, 14 basic blocks)
    pub fn method_02(&mut self) {
        // Writes to field at offset +0x001c
        // Writes to field at offset +0x0038
        // Writes to field at offset +0x0040
        // Writes to field at offset +0x0050
        // Writes to field at offset +0x0058
        // Writes to field at offset +0x0068
    }

    /// Decompiled virtual method #3 @ RVA 0x002a2060 (103 instructions, 20 basic blocks)
    pub fn method_03(&mut self) {
        // Writes to field at offset +0x0058
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

    /// Decompiled virtual method #9 @ RVA 0x002a2410 (32 instructions, 6 basic blocks)
    pub fn method_09(&mut self) {
        // Writes to field at offset +0x0068
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

    /// Decompiled virtual method #14 @ RVA 0x002a1f90 (134 instructions, 12 basic blocks)
    pub fn method_14(&mut self) {
        // Writes to field at offset +0x001c
        // Writes to field at offset +0x0038
        // Writes to field at offset +0x0040
        // Writes to field at offset +0x0048
        // Writes to field at offset +0x004a
        // Writes to field at offset +0x0050
        // Writes to field at offset +0x0058
        // Writes to field at offset +0x0068
    }

}

impl CcClass for ccCharIcon {
    fn class_name(&self) -> &'static str { Self::CLASS_NAME }
    fn vtable_rva(&self) -> u32 { Self::VTABLE_RVA }
    fn virtual_method_count(&self) -> usize { Self::METHOD_COUNT }
    fn base_classes(&self) -> &'static [&'static str] { Self::BASE_CLASSES }
}

/// Decompiled CC2 class: `ccChimAI` (VTable RVA: 0x008bca58, 23 virtual methods, 21 recovered fields)
#[derive(Debug, Clone, Default)]
pub struct ccChimAI {
    pub vtable_rva: u32,
    pub field_0x0000: u64,
    pub field_0x0001: u32,
    pub field_0x0004: f32,
    pub field_0x0010: u64,
    pub field_0x0020: u32,
    pub field_0x0021: u8,
    pub field_0x0024: u16,
    pub field_0x002c: u16,
    pub field_0x002e: u16,
    pub field_0x0034: u32,
    pub field_0x0040: u32,
    pub field_0x0050: u64,
    pub field_0x0054: u32,
    pub field_0x0090: u64,
    pub field_0x00a0: u32,
    pub field_0x00a4: u32,
    pub field_0x00cc: u32,
    pub field_0x00e0: u32,
    pub field_0x0101: u8,
    pub field_0x0108: u16,
    pub field_0x0160: u8,
}

impl ccChimAI {
    pub const CLASS_NAME: &'static str = "ccChimAI";
    pub const VTABLE_RVA: u32 = 0x008bca58;
    pub const METHOD_COUNT: usize = 23;
    pub const BASE_CLASSES: &'static [&'static str] = &[];
    pub const VIRTUAL_METHODS: &'static [u64] = &[0x0000000180298820, 0x0000000180298c90, 0x0000000180298ce0, 0x00000001802987f0, 0x0000000180299080, 0x0000000180299390, 0x0000000180299680, 0x0000000180299cc0, 0x00000001802998c0, 0x0000000180299f50, 0x000000018029a9c0, 0x000000018029a170, 0x000000018029a680, 0x00000001800d3090, 0x00000001800d3090, 0x00000001800d3090, 0x000000018029a9d0, 0x000000018029abe0, 0x000000018029ac00, 0x000000018029aeb0, 0x000000018029b520, 0x000000018029b5f0, 0x000000018029b870];

    pub fn new() -> Self {
        Self {
            vtable_rva: Self::VTABLE_RVA,
            field_0x0000: Default::default(),
            field_0x0001: Default::default(),
            field_0x0004: Default::default(),
            field_0x0010: Default::default(),
            field_0x0020: Default::default(),
            field_0x0021: Default::default(),
            field_0x0024: Default::default(),
            field_0x002c: Default::default(),
            field_0x002e: Default::default(),
            field_0x0034: Default::default(),
            field_0x0040: Default::default(),
            field_0x0050: Default::default(),
            field_0x0054: Default::default(),
            field_0x0090: Default::default(),
            field_0x00a0: Default::default(),
            field_0x00a4: Default::default(),
            field_0x00cc: Default::default(),
            field_0x00e0: Default::default(),
            field_0x0101: Default::default(),
            field_0x0108: Default::default(),
            field_0x0160: Default::default(),
        }
    }

    /// Decompiled virtual method #0 @ RVA 0x00298820 (124 instructions, 31 basic blocks)
    pub fn method_00(&mut self) {
        // Writes to field at offset +0x0000
    }

    /// Decompiled virtual method #1 @ RVA 0x00298c90 (15 instructions, 5 basic blocks)
    pub fn method_01(&mut self) {
        // Writes to field at offset +0x002e
        // Writes to field at offset +0x00e0
        // Writes to field at offset +0x0160
    }

    /// Decompiled virtual method #2 @ RVA 0x00298ce0 (125 instructions, 30 basic blocks)
    pub fn method_02(&mut self) {
        // Writes to field at offset +0x0000
    }

    /// Decompiled virtual method #3 @ RVA 0x002987f0 (11 instructions, 3 basic blocks)
    pub fn method_03(&mut self) {
        // Writes to field at offset +0x0020
        // Writes to field at offset +0x002c
        // Writes to field at offset +0x002e
        // Writes to field at offset +0x0034
    }

    /// Decompiled virtual method #4 @ RVA 0x00299080 (123 instructions, 23 basic blocks)
    pub fn method_04(&mut self) {
    }

    /// Decompiled virtual method #5 @ RVA 0x00299390 (108 instructions, 20 basic blocks)
    pub fn method_05(&mut self) {
    }

    /// Decompiled virtual method #6 @ RVA 0x00299680 (80 instructions, 9 basic blocks)
    pub fn method_06(&mut self) {
    }

    /// Decompiled virtual method #7 @ RVA 0x00299cc0 (54 instructions, 5 basic blocks)
    pub fn method_07(&mut self) {
    }

    /// Decompiled virtual method #8 @ RVA 0x002998c0 (54 instructions, 5 basic blocks)
    pub fn method_08(&mut self) {
    }

    /// Decompiled virtual method #9 @ RVA 0x00299f50 (60 instructions, 8 basic blocks)
    pub fn method_09(&mut self) {
    }

    /// Decompiled virtual method #10 @ RVA 0x0029a9c0 (2 instructions, 1 basic blocks)
    pub fn method_10(&mut self) {
        // Writes to field at offset +0x0021
    }

    /// Decompiled virtual method #11 @ RVA 0x0029a170 (113 instructions, 25 basic blocks)
    pub fn method_11(&mut self) {
        // Writes to field at offset +0x0000
    }

    /// Decompiled virtual method #12 @ RVA 0x0029a680 (67 instructions, 10 basic blocks)
    pub fn method_12(&mut self) {
    }

    /// Decompiled virtual method #13 @ RVA 0x000d3090 (1 instructions, 1 basic blocks)
    pub fn method_13(&mut self) {
    }

    /// Decompiled virtual method #14 @ RVA 0x000d3090 (1 instructions, 1 basic blocks)
    pub fn method_14(&mut self) {
    }

    /// Decompiled virtual method #15 @ RVA 0x000d3090 (1 instructions, 1 basic blocks)
    pub fn method_15(&mut self) {
    }

    /// Decompiled virtual method #16 @ RVA 0x0029a9d0 (82 instructions, 20 basic blocks)
    pub fn method_16(&mut self) {
        // Writes to field at offset +0x0000
    }

    /// Decompiled virtual method #17 @ RVA 0x0029abe0 (6 instructions, 1 basic blocks)
    pub fn method_17(&mut self) {
        // Writes to field at offset +0x0020
        // Writes to field at offset +0x002c
        // Writes to field at offset +0x002e
        // Writes to field at offset +0x0034
    }

    /// Decompiled virtual method #18 @ RVA 0x0029ac00 (97 instructions, 12 basic blocks)
    pub fn method_18(&mut self) {
        // Writes to field at offset +0x0054
    }

    /// Decompiled virtual method #19 @ RVA 0x0029aeb0 (121 instructions, 28 basic blocks)
    pub fn method_19(&mut self) {
        // Writes to field at offset +0x0000
    }

    /// Decompiled virtual method #20 @ RVA 0x0029b520 (20 instructions, 7 basic blocks)
    pub fn method_20(&mut self) {
        // Writes to field at offset +0x0040
    }

    /// Decompiled virtual method #21 @ RVA 0x0029b5f0 (61 instructions, 9 basic blocks)
    pub fn method_21(&mut self) {
    }

    /// Decompiled virtual method #22 @ RVA 0x0029b870 (109 instructions, 28 basic blocks)
    pub fn method_22(&mut self) {
        // Writes to field at offset +0x0000
        // Writes to field at offset +0x00cc
    }

}

impl CcClass for ccChimAI {
    fn class_name(&self) -> &'static str { Self::CLASS_NAME }
    fn vtable_rva(&self) -> u32 { Self::VTABLE_RVA }
    fn virtual_method_count(&self) -> usize { Self::METHOD_COUNT }
    fn base_classes(&self) -> &'static [&'static str] { Self::BASE_CLASSES }
}

/// Decompiled CC2 class: `ccChimBall` (VTable RVA: 0x008bcb28, 1 virtual methods, 1 recovered fields)
#[derive(Debug, Clone, Default)]
pub struct ccChimBall {
    pub vtable_rva: u32,
    pub field_0x0000: u64,
}

impl ccChimBall {
    pub const CLASS_NAME: &'static str = "ccChimBall";
    pub const VTABLE_RVA: u32 = 0x008bcb28;
    pub const METHOD_COUNT: usize = 1;
    pub const BASE_CLASSES: &'static [&'static str] = &[];
    pub const VIRTUAL_METHODS: &'static [u64] = &[0x00000001804a3bb0];

    pub fn new() -> Self {
        Self {
            vtable_rva: Self::VTABLE_RVA,
            field_0x0000: Default::default(),
        }
    }

    /// Decompiled virtual method #0 @ RVA 0x004a3bb0 (21 instructions, 3 basic blocks)
    pub fn method_00(&mut self) {
        // Writes to field at offset +0x0000
    }

}

impl CcClass for ccChimBall {
    fn class_name(&self) -> &'static str { Self::CLASS_NAME }
    fn vtable_rva(&self) -> u32 { Self::VTABLE_RVA }
    fn virtual_method_count(&self) -> usize { Self::METHOD_COUNT }
    fn base_classes(&self) -> &'static [&'static str] { Self::BASE_CLASSES }
}

/// Decompiled CC2 class: `ccComboFinish` (VTable RVA: 0x00881288, 16 virtual methods, 15 recovered fields)
/// Inherits: ccStateIconBase, ccDispBase
#[derive(Debug, Clone, Default)]
pub struct ccComboFinish {
    pub vtable_rva: u32,
    pub field_0x0000: u64,
    pub field_0x0008: u32,
    pub field_0x001c: f32,
    pub field_0x0028: u32,
    pub field_0x002c: u32,
    pub field_0x0030: u32,
    pub field_0x0034: u32,
    pub field_0x0040: u64,
    pub field_0x0044: f32,
    pub field_0x0048: u32,
    pub field_0x0050: u32,
    pub field_0x0060: f32,
    pub field_0x0064: f32,
    pub field_0x0068: u32,
    pub field_0x006c: u8,
}

impl ccComboFinish {
    pub const CLASS_NAME: &'static str = "ccComboFinish";
    pub const VTABLE_RVA: u32 = 0x00881288;
    pub const METHOD_COUNT: usize = 16;
    pub const BASE_CLASSES: &'static [&'static str] = &["ccStateIconBase", "ccDispBase"];
    pub const VIRTUAL_METHODS: &'static [u64] = &[0x000000018023e480, 0x000000018023e4b0, 0x000000018023e560, 0x000000018023e830, 0x00000001802a0e30, 0x00000001800d3090, 0x00000001801b81f0, 0x00000001801b8200, 0x00000001801b8210, 0x00000001800d3090, 0x00000001800d3090, 0x000000018023e5a0, 0x000000018023e710, 0x0000000180234180, 0x0000000180232e30, 0x000000018023eba0];

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
            field_0x0040: Default::default(),
            field_0x0044: Default::default(),
            field_0x0048: Default::default(),
            field_0x0050: Default::default(),
            field_0x0060: Default::default(),
            field_0x0064: Default::default(),
            field_0x0068: Default::default(),
            field_0x006c: Default::default(),
        }
    }

    /// Decompiled virtual method #0 @ RVA 0x0023e480 (13 instructions, 3 basic blocks)
    pub fn method_00(&mut self) {
        // Writes to field at offset +0x0000
    }

    /// Decompiled virtual method #1 @ RVA 0x0023e4b0 (39 instructions, 5 basic blocks)
    pub fn method_01(&mut self) {
    }

    /// Decompiled virtual method #2 @ RVA 0x0023e560 (16 instructions, 5 basic blocks)
    pub fn method_02(&mut self) {
    }

    /// Decompiled virtual method #3 @ RVA 0x0023e830 (100 instructions, 12 basic blocks)
    pub fn method_03(&mut self) {
        // Writes to field at offset +0x0008
        // Writes to field at offset +0x0040
        // Writes to field at offset +0x0044
        // Writes to field at offset +0x0048
        // Writes to field at offset +0x006c
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

    /// Decompiled virtual method #11 @ RVA 0x0023e5a0 (87 instructions, 22 basic blocks)
    pub fn method_11(&mut self) {
        // Writes to field at offset +0x001c
        // Writes to field at offset +0x001c
    }

    /// Decompiled virtual method #12 @ RVA 0x0023e710 (61 instructions, 23 basic blocks)
    pub fn method_12(&mut self) {
        // Writes to field at offset +0x001c
        // Writes to field at offset +0x001c
        // Writes to field at offset +0x0050
        // Writes to field at offset +0x0060
        // Writes to field at offset +0x0060
        // Writes to field at offset +0x0064
        // Writes to field at offset +0x0068
        // Writes to field at offset +0x006c
    }

    /// Decompiled virtual method #13 @ RVA 0x00234180 (26 instructions, 6 basic blocks)
    pub fn method_13(&mut self) {
        // Writes to field at offset +0x001c
        // Writes to field at offset +0x001c
    }

    /// Decompiled virtual method #14 @ RVA 0x00232e30 (2 instructions, 1 basic blocks)
    pub fn method_14(&mut self) {
    }

    /// Decompiled virtual method #15 @ RVA 0x0023eba0 (31 instructions, 6 basic blocks)
    pub fn method_15(&mut self) {
    }

}

impl CcClass for ccComboFinish {
    fn class_name(&self) -> &'static str { Self::CLASS_NAME }
    fn vtable_rva(&self) -> u32 { Self::VTABLE_RVA }
    fn virtual_method_count(&self) -> usize { Self::METHOD_COUNT }
    fn base_classes(&self) -> &'static [&'static str] { Self::BASE_CLASSES }
}

/// Decompiled CC2 class: `ccCreateModel` (VTable RVA: 0x00876ec0, 1 virtual methods, 0 recovered fields)
/// Inherits: ccCoord
#[derive(Debug, Clone, Default)]
pub struct ccCreateModel {
    pub vtable_rva: u32,
}

impl ccCreateModel {
    pub const CLASS_NAME: &'static str = "ccCreateModel";
    pub const VTABLE_RVA: u32 = 0x00876ec0;
    pub const METHOD_COUNT: usize = 1;
    pub const BASE_CLASSES: &'static [&'static str] = &["ccCoord"];
    pub const VIRTUAL_METHODS: &'static [u64] = &[0x000000018014dbc0];

    pub fn new() -> Self {
        Self {
            vtable_rva: Self::VTABLE_RVA,
        }
    }

    /// Decompiled virtual method #0 @ RVA 0x0014dbc0 (16 instructions, 3 basic blocks)
    pub fn method_00(&mut self) {
    }

}

impl CcClass for ccCreateModel {
    fn class_name(&self) -> &'static str { Self::CLASS_NAME }
    fn vtable_rva(&self) -> u32 { Self::VTABLE_RVA }
    fn virtual_method_count(&self) -> usize { Self::METHOD_COUNT }
    fn base_classes(&self) -> &'static [&'static str] { Self::BASE_CLASSES }
}

/// Decompiled CC2 class: `ccCreateTexChunk` (VTable RVA: 0x00877bd0, 3 virtual methods, 17 recovered fields)
/// Inherits: ccTexChunk, ccChunkBasis
#[derive(Debug, Clone, Default)]
pub struct ccCreateTexChunk {
    pub vtable_rva: u32,
    pub field_0x0008: u64,
    pub field_0x0018: u64,
    pub field_0x0020: u64,
    pub field_0x0028: u64,
    pub field_0x0030: u16,
    pub field_0x0032: u16,
    pub field_0x0034: u16,
    pub field_0x0036: u8,
    pub field_0x0037: u8,
    pub field_0x0038: u8,
    pub field_0x0039: u8,
    pub field_0x003a: u8,
    pub field_0x003d: u8,
    pub field_0x0040: u64,
    pub field_0x0048: u8,
    pub field_0x0049: u8,
    pub field_0x00d8: u32,
}

impl ccCreateTexChunk {
    pub const CLASS_NAME: &'static str = "ccCreateTexChunk";
    pub const VTABLE_RVA: u32 = 0x00877bd0;
    pub const METHOD_COUNT: usize = 3;
    pub const BASE_CLASSES: &'static [&'static str] = &["ccTexChunk", "ccChunkBasis"];
    pub const VIRTUAL_METHODS: &'static [u64] = &[0x00000001801947b0, 0x000000018019adc0, 0x00000001800d3090];

    pub fn new() -> Self {
        Self {
            vtable_rva: Self::VTABLE_RVA,
            field_0x0008: Default::default(),
            field_0x0018: Default::default(),
            field_0x0020: Default::default(),
            field_0x0028: Default::default(),
            field_0x0030: Default::default(),
            field_0x0032: Default::default(),
            field_0x0034: Default::default(),
            field_0x0036: Default::default(),
            field_0x0037: Default::default(),
            field_0x0038: Default::default(),
            field_0x0039: Default::default(),
            field_0x003a: Default::default(),
            field_0x003d: Default::default(),
            field_0x0040: Default::default(),
            field_0x0048: Default::default(),
            field_0x0049: Default::default(),
            field_0x00d8: Default::default(),
        }
    }

    /// Decompiled virtual method #0 @ RVA 0x001947b0 (16 instructions, 3 basic blocks)
    pub fn method_00(&mut self) {
    }

    /// Decompiled virtual method #1 @ RVA 0x0019adc0 (34 instructions, 1 basic blocks)
    pub fn method_01(&mut self) {
        // Writes to field at offset +0x0008
        // Writes to field at offset +0x0018
        // Writes to field at offset +0x0020
        // Writes to field at offset +0x0028
        // Writes to field at offset +0x0030
        // Writes to field at offset +0x0032
        // Writes to field at offset +0x0034
        // Writes to field at offset +0x0036
        // Writes to field at offset +0x0037
        // Writes to field at offset +0x0038
        // Writes to field at offset +0x0039
        // Writes to field at offset +0x003a
        // Writes to field at offset +0x003d
        // Writes to field at offset +0x0040
        // Writes to field at offset +0x0048
        // Writes to field at offset +0x0049
        // Writes to field at offset +0x00d8
    }

    /// Decompiled virtual method #2 @ RVA 0x000d3090 (1 instructions, 1 basic blocks)
    pub fn method_02(&mut self) {
    }

}

impl CcClass for ccCreateTexChunk {
    fn class_name(&self) -> &'static str { Self::CLASS_NAME }
    fn vtable_rva(&self) -> u32 { Self::VTABLE_RVA }
    fn virtual_method_count(&self) -> usize { Self::METHOD_COUNT }
    fn base_classes(&self) -> &'static [&'static str] { Self::BASE_CLASSES }
}

/// Decompiled CC2 class: `ccDamageAll` (VTable RVA: 0x00881620, 16 virtual methods, 21 recovered fields)
/// Inherits: ccDamageNormal, ccDamageBase, ccDispBase
#[derive(Debug, Clone, Default)]
pub struct ccDamageAll {
    pub vtable_rva: u32,
    pub field_0x0000: u64,
    pub field_0x0004: u32,
    pub field_0x0008: u32,
    pub field_0x000c: u32,
    pub field_0x0010: u32,
    pub field_0x0014: f32,
    pub field_0x0018: f32,
    pub field_0x0028: u32,
    pub field_0x002c: u32,
    pub field_0x0030: u32,
    pub field_0x0033: u8,
    pub field_0x0034: u32,
    pub field_0x0038: u32,
    pub field_0x0040: u32,
    pub field_0x0048: u32,
    pub field_0x004c: u32,
    pub field_0x0050: u32,
    pub field_0x0068: u64,
    pub field_0x0070: u32,
    pub field_0x0090: u32,
    pub field_0x0094: u8,
}

impl ccDamageAll {
    pub const CLASS_NAME: &'static str = "ccDamageAll";
    pub const VTABLE_RVA: u32 = 0x00881620;
    pub const METHOD_COUNT: usize = 16;
    pub const BASE_CLASSES: &'static [&'static str] = &["ccDamageNormal", "ccDamageBase", "ccDispBase"];
    pub const VIRTUAL_METHODS: &'static [u64] = &[0x0000000180234af0, 0x0000000180234b50, 0x0000000180234ba0, 0x00000001802352a0, 0x00000001802a0e30, 0x00000001800d3090, 0x00000001801b81f0, 0x00000001801b8200, 0x00000001801b8210, 0x00000001800d3090, 0x00000001800d3090, 0x0000000180234c80, 0x0000000180233970, 0x0000000180234e00, 0x0000000180234f50, 0x0000000180232890];

    pub fn new() -> Self {
        Self {
            vtable_rva: Self::VTABLE_RVA,
            field_0x0000: Default::default(),
            field_0x0004: Default::default(),
            field_0x0008: Default::default(),
            field_0x000c: Default::default(),
            field_0x0010: Default::default(),
            field_0x0014: Default::default(),
            field_0x0018: Default::default(),
            field_0x0028: Default::default(),
            field_0x002c: Default::default(),
            field_0x0030: Default::default(),
            field_0x0033: Default::default(),
            field_0x0034: Default::default(),
            field_0x0038: Default::default(),
            field_0x0040: Default::default(),
            field_0x0048: Default::default(),
            field_0x004c: Default::default(),
            field_0x0050: Default::default(),
            field_0x0068: Default::default(),
            field_0x0070: Default::default(),
            field_0x0090: Default::default(),
            field_0x0094: Default::default(),
        }
    }

    /// Decompiled virtual method #0 @ RVA 0x00234af0 (23 instructions, 3 basic blocks)
    pub fn method_00(&mut self) {
        // Writes to field at offset +0x0000
    }

    /// Decompiled virtual method #1 @ RVA 0x00234b50 (18 instructions, 3 basic blocks)
    pub fn method_01(&mut self) {
    }

    /// Decompiled virtual method #2 @ RVA 0x00234ba0 (61 instructions, 12 basic blocks)
    pub fn method_02(&mut self) {
    }

    /// Decompiled virtual method #3 @ RVA 0x002352a0 (115 instructions, 12 basic blocks)
    pub fn method_03(&mut self) {
        // Writes to field at offset +0x0090
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

    /// Decompiled virtual method #11 @ RVA 0x00234c80 (99 instructions, 20 basic blocks)
    pub fn method_11(&mut self) {
        // Writes to field at offset +0x0000
        // Writes to field at offset +0x0004
        // Writes to field at offset +0x0090
        // Writes to field at offset +0x0094
    }

    /// Decompiled virtual method #12 @ RVA 0x00233970 (33 instructions, 3 basic blocks)
    pub fn method_12(&mut self) {
        // Writes to field at offset +0x0050
    }

    /// Decompiled virtual method #13 @ RVA 0x00234e00 (85 instructions, 15 basic blocks)
    pub fn method_13(&mut self) {
        // Writes to field at offset +0x0000
        // Writes to field at offset +0x0004
        // Writes to field at offset +0x0030
        // Writes to field at offset +0x0033
        // Writes to field at offset +0x0090
    }

    /// Decompiled virtual method #14 @ RVA 0x00234f50 (32 instructions, 3 basic blocks)
    pub fn method_14(&mut self) {
        // Writes to field at offset +0x000c
        // Writes to field at offset +0x0010
        // Writes to field at offset +0x0014
        // Writes to field at offset +0x0038
        // Writes to field at offset +0x0040
        // Writes to field at offset +0x0048
        // Writes to field at offset +0x004c
        // Writes to field at offset +0x0050
    }

    /// Decompiled virtual method #15 @ RVA 0x00232890 (2 instructions, 1 basic blocks)
    pub fn method_15(&mut self) {
    }

}

impl CcClass for ccDamageAll {
    fn class_name(&self) -> &'static str { Self::CLASS_NAME }
    fn vtable_rva(&self) -> u32 { Self::VTABLE_RVA }
    fn virtual_method_count(&self) -> usize { Self::METHOD_COUNT }
    fn base_classes(&self) -> &'static [&'static str] { Self::BASE_CLASSES }
}

/// Decompiled CC2 class: `ccDamageCrittical` (VTable RVA: 0x00881bb8, 16 virtual methods, 20 recovered fields)
/// Inherits: ccDamageNormal, ccDamageBase, ccDispBase
#[derive(Debug, Clone, Default)]
pub struct ccDamageCrittical {
    pub vtable_rva: u32,
    pub field_0x0000: u64,
    pub field_0x0008: u32,
    pub field_0x000c: u32,
    pub field_0x0010: u32,
    pub field_0x0014: f32,
    pub field_0x0018: f32,
    pub field_0x001a: u8,
    pub field_0x0028: u32,
    pub field_0x002c: u32,
    pub field_0x0030: u32,
    pub field_0x0034: u32,
    pub field_0x0038: u32,
    pub field_0x0040: u32,
    pub field_0x0044: f32,
    pub field_0x0048: f32,
    pub field_0x004c: f32,
    pub field_0x0050: u32,
    pub field_0x0070: u64,
    pub field_0x0078: u64,
    pub field_0x0118: u64,
}

impl ccDamageCrittical {
    pub const CLASS_NAME: &'static str = "ccDamageCrittical";
    pub const VTABLE_RVA: u32 = 0x00881bb8;
    pub const METHOD_COUNT: usize = 16;
    pub const BASE_CLASSES: &'static [&'static str] = &["ccDamageNormal", "ccDamageBase", "ccDispBase"];
    pub const VIRTUAL_METHODS: &'static [u64] = &[0x0000000180232a70, 0x0000000180234580, 0x0000000180232960, 0x00000001802346d0, 0x00000001802a0e30, 0x00000001800d3090, 0x0000000180232a00, 0x00000001801b8200, 0x00000001801b8210, 0x00000001800d3090, 0x00000001800d3090, 0x00000001802338c0, 0x0000000180233970, 0x0000000180233a00, 0x0000000180233c10, 0x00000001802329f0];

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
            field_0x0048: Default::default(),
            field_0x004c: Default::default(),
            field_0x0050: Default::default(),
            field_0x0070: Default::default(),
            field_0x0078: Default::default(),
            field_0x0118: Default::default(),
        }
    }

    /// Decompiled virtual method #0 @ RVA 0x00232a70 (23 instructions, 3 basic blocks)
    pub fn method_00(&mut self) {
        // Writes to field at offset +0x0000
    }

    /// Decompiled virtual method #1 @ RVA 0x00234580 (85 instructions, 13 basic blocks)
    pub fn method_01(&mut self) {
        // Writes to field at offset +0x001a
    }

    /// Decompiled virtual method #2 @ RVA 0x00232960 (35 instructions, 7 basic blocks)
    pub fn method_02(&mut self) {
    }

    /// Decompiled virtual method #3 @ RVA 0x002346d0 (124 instructions, 8 basic blocks)
    pub fn method_03(&mut self) {
        // Writes to field at offset +0x001a
    }

    /// Decompiled virtual method #4 @ RVA 0x002a0e30 (48 instructions, 11 basic blocks)
    pub fn method_04(&mut self) {
        // Writes to field at offset +0x0008
    }

    /// Decompiled virtual method #5 @ RVA 0x000d3090 (1 instructions, 1 basic blocks)
    pub fn method_05(&mut self) {
    }

    /// Decompiled virtual method #6 @ RVA 0x00232a00 (32 instructions, 8 basic blocks)
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

    /// Decompiled virtual method #11 @ RVA 0x002338c0 (46 instructions, 14 basic blocks)
    pub fn method_11(&mut self) {
        // Writes to field at offset +0x0040
        // Writes to field at offset +0x0040
        // Writes to field at offset +0x0048
        // Writes to field at offset +0x0048
        // Writes to field at offset +0x004c
        // Writes to field at offset +0x004c
    }

    /// Decompiled virtual method #12 @ RVA 0x00233970 (33 instructions, 3 basic blocks)
    pub fn method_12(&mut self) {
        // Writes to field at offset +0x0050
    }

    /// Decompiled virtual method #13 @ RVA 0x00233a00 (41 instructions, 10 basic blocks)
    pub fn method_13(&mut self) {
        // Writes to field at offset +0x0014
        // Writes to field at offset +0x0018
        // Writes to field at offset +0x0040
        // Writes to field at offset +0x0040
        // Writes to field at offset +0x0048
        // Writes to field at offset +0x004c
    }

    /// Decompiled virtual method #14 @ RVA 0x00233c10 (114 instructions, 13 basic blocks)
    pub fn method_14(&mut self) {
        // Writes to field at offset +0x000c
        // Writes to field at offset +0x0010
        // Writes to field at offset +0x0014
        // Writes to field at offset +0x0038
        // Writes to field at offset +0x0040
        // Writes to field at offset +0x0048
        // Writes to field at offset +0x004c
        // Writes to field at offset +0x0050
    }

    /// Decompiled virtual method #15 @ RVA 0x002329f0 (2 instructions, 1 basic blocks)
    pub fn method_15(&mut self) {
    }

}

impl CcClass for ccDamageCrittical {
    fn class_name(&self) -> &'static str { Self::CLASS_NAME }
    fn vtable_rva(&self) -> u32 { Self::VTABLE_RVA }
    fn virtual_method_count(&self) -> usize { Self::METHOD_COUNT }
    fn base_classes(&self) -> &'static [&'static str] { Self::BASE_CLASSES }
}

/// Decompiled CC2 class: `ccDamageElement` (VTable RVA: 0x008818a8, 16 virtual methods, 29 recovered fields)
/// Inherits: ccDamageNormal, ccDamageBase, ccDispBase
#[derive(Debug, Clone, Default)]
pub struct ccDamageElement {
    pub vtable_rva: u32,
    pub field_0x0000: u64,
    pub field_0x0008: u64,
    pub field_0x000c: u32,
    pub field_0x0010: u32,
    pub field_0x0014: f32,
    pub field_0x0018: f32,
    pub field_0x001a: u8,
    pub field_0x0020: u32,
    pub field_0x0024: u64,
    pub field_0x0028: u32,
    pub field_0x002c: u64,
    pub field_0x0030: u32,
    pub field_0x0034: u64,
    pub field_0x0038: u32,
    pub field_0x0040: u32,
    pub field_0x0044: u32,
    pub field_0x0048: u32,
    pub field_0x004c: u32,
    pub field_0x0050: u32,
    pub field_0x0058: u64,
    pub field_0x0060: u64,
    pub field_0x0068: u64,
    pub field_0x0070: u64,
    pub field_0x0078: u64,
    pub field_0x0080: u64,
    pub field_0x0088: u64,
    pub field_0x0090: u32,
    pub field_0x0094: u16,
    pub field_0x0118: u64,
}

impl ccDamageElement {
    pub const CLASS_NAME: &'static str = "ccDamageElement";
    pub const VTABLE_RVA: u32 = 0x008818a8;
    pub const METHOD_COUNT: usize = 16;
    pub const BASE_CLASSES: &'static [&'static str] = &["ccDamageNormal", "ccDamageBase", "ccDispBase"];
    pub const VIRTUAL_METHODS: &'static [u64] = &[0x0000000180232a70, 0x0000000180234810, 0x0000000180232960, 0x0000000180234920, 0x00000001802a0e30, 0x00000001800d3090, 0x0000000180232a00, 0x00000001801b8200, 0x00000001801b8210, 0x00000001800d3090, 0x00000001800d3090, 0x00000001802338c0, 0x0000000180233970, 0x0000000180233a00, 0x0000000180233c10, 0x00000001802329f0];

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
            field_0x0020: Default::default(),
            field_0x0024: Default::default(),
            field_0x0028: Default::default(),
            field_0x002c: Default::default(),
            field_0x0030: Default::default(),
            field_0x0034: Default::default(),
            field_0x0038: Default::default(),
            field_0x0040: Default::default(),
            field_0x0044: Default::default(),
            field_0x0048: Default::default(),
            field_0x004c: Default::default(),
            field_0x0050: Default::default(),
            field_0x0058: Default::default(),
            field_0x0060: Default::default(),
            field_0x0068: Default::default(),
            field_0x0070: Default::default(),
            field_0x0078: Default::default(),
            field_0x0080: Default::default(),
            field_0x0088: Default::default(),
            field_0x0090: Default::default(),
            field_0x0094: Default::default(),
            field_0x0118: Default::default(),
        }
    }

    /// Decompiled virtual method #0 @ RVA 0x00232a70 (23 instructions, 3 basic blocks)
    pub fn method_00(&mut self) {
        // Writes to field at offset +0x0000
    }

    /// Decompiled virtual method #1 @ RVA 0x00234810 (64 instructions, 10 basic blocks)
    pub fn method_01(&mut self) {
        // Writes to field at offset +0x001a
    }

    /// Decompiled virtual method #2 @ RVA 0x00232960 (35 instructions, 7 basic blocks)
    pub fn method_02(&mut self) {
    }

    /// Decompiled virtual method #3 @ RVA 0x00234920 (101 instructions, 1 basic blocks)
    pub fn method_03(&mut self) {
        // Writes to field at offset +0x0000
        // Writes to field at offset +0x0008
        // Writes to field at offset +0x0010
        // Writes to field at offset +0x0018
        // Writes to field at offset +0x0020
        // Writes to field at offset +0x0024
        // Writes to field at offset +0x002c
        // Writes to field at offset +0x0034
        // Writes to field at offset +0x0040
        // Writes to field at offset +0x0044
        // Writes to field at offset +0x0048
        // Writes to field at offset +0x004c
        // Writes to field at offset +0x0050
        // Writes to field at offset +0x0058
        // Writes to field at offset +0x0060
        // Writes to field at offset +0x0068
        // Writes to field at offset +0x0070
        // Writes to field at offset +0x0078
        // Writes to field at offset +0x0080
        // Writes to field at offset +0x0088
        // Writes to field at offset +0x0090
        // Writes to field at offset +0x0094
    }

    /// Decompiled virtual method #4 @ RVA 0x002a0e30 (48 instructions, 11 basic blocks)
    pub fn method_04(&mut self) {
        // Writes to field at offset +0x0008
    }

    /// Decompiled virtual method #5 @ RVA 0x000d3090 (1 instructions, 1 basic blocks)
    pub fn method_05(&mut self) {
    }

    /// Decompiled virtual method #6 @ RVA 0x00232a00 (32 instructions, 8 basic blocks)
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

    /// Decompiled virtual method #11 @ RVA 0x002338c0 (46 instructions, 14 basic blocks)
    pub fn method_11(&mut self) {
        // Writes to field at offset +0x0040
        // Writes to field at offset +0x0040
        // Writes to field at offset +0x0048
        // Writes to field at offset +0x0048
        // Writes to field at offset +0x004c
        // Writes to field at offset +0x004c
    }

    /// Decompiled virtual method #12 @ RVA 0x00233970 (33 instructions, 3 basic blocks)
    pub fn method_12(&mut self) {
        // Writes to field at offset +0x0050
    }

    /// Decompiled virtual method #13 @ RVA 0x00233a00 (41 instructions, 10 basic blocks)
    pub fn method_13(&mut self) {
        // Writes to field at offset +0x0014
        // Writes to field at offset +0x0018
        // Writes to field at offset +0x0040
        // Writes to field at offset +0x0040
        // Writes to field at offset +0x0048
        // Writes to field at offset +0x004c
    }

    /// Decompiled virtual method #14 @ RVA 0x00233c10 (114 instructions, 13 basic blocks)
    pub fn method_14(&mut self) {
        // Writes to field at offset +0x000c
        // Writes to field at offset +0x0010
        // Writes to field at offset +0x0014
        // Writes to field at offset +0x0038
        // Writes to field at offset +0x0040
        // Writes to field at offset +0x0048
        // Writes to field at offset +0x004c
        // Writes to field at offset +0x0050
    }

    /// Decompiled virtual method #15 @ RVA 0x002329f0 (2 instructions, 1 basic blocks)
    pub fn method_15(&mut self) {
    }

}

impl CcClass for ccDamageElement {
    fn class_name(&self) -> &'static str { Self::CLASS_NAME }
    fn vtable_rva(&self) -> u32 { Self::VTABLE_RVA }
    fn virtual_method_count(&self) -> usize { Self::METHOD_COUNT }
    fn base_classes(&self) -> &'static [&'static str] { Self::BASE_CLASSES }
}

/// Decompiled CC2 class: `ccDamageList` (VTable RVA: 0x00881930, 16 virtual methods, 10 recovered fields)
/// Inherits: ccCharDispList, ccDispW2L, ccDispBase
#[derive(Debug, Clone, Default)]
pub struct ccDamageList {
    pub vtable_rva: u32,
    pub field_0x0000: u64,
    pub field_0x0008: u32,
    pub field_0x001c: u8,
    pub field_0x0028: u32,
    pub field_0x002c: u32,
    pub field_0x0030: u32,
    pub field_0x0034: u32,
    pub field_0x0060: u64,
    pub field_0x0070: u32,
    pub field_0x0074: u32,
}

impl ccDamageList {
    pub const CLASS_NAME: &'static str = "ccDamageList";
    pub const VTABLE_RVA: u32 = 0x00881930;
    pub const METHOD_COUNT: usize = 16;
    pub const BASE_CLASSES: &'static [&'static str] = &["ccCharDispList", "ccDispW2L", "ccDispBase"];
    pub const VIRTUAL_METHODS: &'static [u64] = &[0x0000000180232b80, 0x00000001802a52e0, 0x00000001802a5330, 0x00000001802a5620, 0x00000001802a5380, 0x00000001802a5530, 0x00000001801b81f0, 0x00000001801b8200, 0x00000001801b8210, 0x00000001800d3090, 0x00000001800d3090, 0x00000001800d3090, 0x00000001800d3090, 0x00000001800d3090, 0x00000001800d3090, 0x00000001802a5730];

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
            field_0x0060: Default::default(),
            field_0x0070: Default::default(),
            field_0x0074: Default::default(),
        }
    }

    /// Decompiled virtual method #0 @ RVA 0x00232b80 (13 instructions, 3 basic blocks)
    pub fn method_00(&mut self) {
        // Writes to field at offset +0x0000
    }

    /// Decompiled virtual method #1 @ RVA 0x002a52e0 (18 instructions, 3 basic blocks)
    pub fn method_01(&mut self) {
        // Writes to field at offset +0x0060
    }

    /// Decompiled virtual method #2 @ RVA 0x002a5330 (19 instructions, 3 basic blocks)
    pub fn method_02(&mut self) {
    }

    /// Decompiled virtual method #3 @ RVA 0x002a5620 (59 instructions, 12 basic blocks)
    pub fn method_03(&mut self) {
        // Writes to field at offset +0x0008
    }

    /// Decompiled virtual method #4 @ RVA 0x002a5380 (102 instructions, 22 basic blocks)
    pub fn method_04(&mut self) {
        // Writes to field at offset +0x0008
        // Writes to field at offset +0x001c
    }

    /// Decompiled virtual method #5 @ RVA 0x002a5530 (50 instructions, 15 basic blocks)
    pub fn method_05(&mut self) {
        // Writes to field at offset +0x0008
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

    /// Decompiled virtual method #14 @ RVA 0x000d3090 (1 instructions, 1 basic blocks)
    pub fn method_14(&mut self) {
    }

    /// Decompiled virtual method #15 @ RVA 0x002a5730 (41 instructions, 8 basic blocks)
    pub fn method_15(&mut self) {
    }

}

impl CcClass for ccDamageList {
    fn class_name(&self) -> &'static str { Self::CLASS_NAME }
    fn vtable_rva(&self) -> u32 { Self::VTABLE_RVA }
    fn virtual_method_count(&self) -> usize { Self::METHOD_COUNT }
    fn base_classes(&self) -> &'static [&'static str] { Self::BASE_CLASSES }
}

/// Decompiled CC2 class: `ccDamageNormal` (VTable RVA: 0x00881a30, 16 virtual methods, 20 recovered fields)
/// Inherits: ccDamageBase, ccDispBase
#[derive(Debug, Clone, Default)]
pub struct ccDamageNormal {
    pub vtable_rva: u32,
    pub field_0x0000: u64,
    pub field_0x0008: u32,
    pub field_0x000c: u32,
    pub field_0x0010: u32,
    pub field_0x0014: f32,
    pub field_0x0018: f32,
    pub field_0x0028: u32,
    pub field_0x002c: u32,
    pub field_0x0030: u32,
    pub field_0x0033: u8,
    pub field_0x0034: u32,
    pub field_0x0038: u32,
    pub field_0x0040: u32,
    pub field_0x0044: f32,
    pub field_0x0048: f32,
    pub field_0x004c: u32,
    pub field_0x0050: u32,
    pub field_0x0058: u64,
    pub field_0x0060: u64,
    pub field_0x0118: u64,
}

impl ccDamageNormal {
    pub const CLASS_NAME: &'static str = "ccDamageNormal";
    pub const VTABLE_RVA: u32 = 0x00881a30;
    pub const METHOD_COUNT: usize = 16;
    pub const BASE_CLASSES: &'static [&'static str] = &["ccDamageBase", "ccDispBase"];
    pub const VIRTUAL_METHODS: &'static [u64] = &[0x00000001802328a0, 0x00000001800d3090, 0x0000000180233850, 0x0000000180233aa0, 0x00000001802a0e30, 0x00000001800d3090, 0x00000001801b81f0, 0x00000001801b8200, 0x00000001801b8210, 0x00000001800d3090, 0x00000001800d3090, 0x00000001802338c0, 0x0000000180233970, 0x0000000180233a00, 0x0000000180233c10, 0x0000000180232890];

    pub fn new() -> Self {
        Self {
            vtable_rva: Self::VTABLE_RVA,
            field_0x0000: Default::default(),
            field_0x0008: Default::default(),
            field_0x000c: Default::default(),
            field_0x0010: Default::default(),
            field_0x0014: Default::default(),
            field_0x0018: Default::default(),
            field_0x0028: Default::default(),
            field_0x002c: Default::default(),
            field_0x0030: Default::default(),
            field_0x0033: Default::default(),
            field_0x0034: Default::default(),
            field_0x0038: Default::default(),
            field_0x0040: Default::default(),
            field_0x0044: Default::default(),
            field_0x0048: Default::default(),
            field_0x004c: Default::default(),
            field_0x0050: Default::default(),
            field_0x0058: Default::default(),
            field_0x0060: Default::default(),
            field_0x0118: Default::default(),
        }
    }

    /// Decompiled virtual method #0 @ RVA 0x002328a0 (23 instructions, 3 basic blocks)
    pub fn method_00(&mut self) {
        // Writes to field at offset +0x0000
    }

    /// Decompiled virtual method #1 @ RVA 0x000d3090 (1 instructions, 1 basic blocks)
    pub fn method_01(&mut self) {
    }

    /// Decompiled virtual method #2 @ RVA 0x00233850 (26 instructions, 5 basic blocks)
    pub fn method_02(&mut self) {
    }

    /// Decompiled virtual method #3 @ RVA 0x00233aa0 (128 instructions, 4 basic blocks)
    pub fn method_03(&mut self) {
        // Writes to field at offset +0x000c
        // Writes to field at offset +0x0010
        // Writes to field at offset +0x0014
        // Writes to field at offset +0x0030
        // Writes to field at offset +0x0033
        // Writes to field at offset +0x0038
        // Writes to field at offset +0x0040
        // Writes to field at offset +0x0048
        // Writes to field at offset +0x004c
        // Writes to field at offset +0x0050
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

    /// Decompiled virtual method #11 @ RVA 0x002338c0 (46 instructions, 14 basic blocks)
    pub fn method_11(&mut self) {
        // Writes to field at offset +0x0040
        // Writes to field at offset +0x0040
        // Writes to field at offset +0x0048
        // Writes to field at offset +0x0048
        // Writes to field at offset +0x004c
        // Writes to field at offset +0x004c
    }

    /// Decompiled virtual method #12 @ RVA 0x00233970 (33 instructions, 3 basic blocks)
    pub fn method_12(&mut self) {
        // Writes to field at offset +0x0050
    }

    /// Decompiled virtual method #13 @ RVA 0x00233a00 (41 instructions, 10 basic blocks)
    pub fn method_13(&mut self) {
        // Writes to field at offset +0x0014
        // Writes to field at offset +0x0018
        // Writes to field at offset +0x0040
        // Writes to field at offset +0x0040
        // Writes to field at offset +0x0048
        // Writes to field at offset +0x004c
    }

    /// Decompiled virtual method #14 @ RVA 0x00233c10 (114 instructions, 13 basic blocks)
    pub fn method_14(&mut self) {
        // Writes to field at offset +0x000c
        // Writes to field at offset +0x0010
        // Writes to field at offset +0x0014
        // Writes to field at offset +0x0038
        // Writes to field at offset +0x0040
        // Writes to field at offset +0x0048
        // Writes to field at offset +0x004c
        // Writes to field at offset +0x0050
    }

    /// Decompiled virtual method #15 @ RVA 0x00232890 (2 instructions, 1 basic blocks)
    pub fn method_15(&mut self) {
    }

}

impl CcClass for ccDamageNormal {
    fn class_name(&self) -> &'static str { Self::CLASS_NAME }
    fn vtable_rva(&self) -> u32 { Self::VTABLE_RVA }
    fn virtual_method_count(&self) -> usize { Self::METHOD_COUNT }
    fn base_classes(&self) -> &'static [&'static str] { Self::BASE_CLASSES }
}

/// Decompiled CC2 class: `ccDamageParts` (VTable RVA: 0x00881b40, 14 virtual methods, 9 recovered fields)
/// Inherits: ccDispBase
#[derive(Debug, Clone, Default)]
pub struct ccDamageParts {
    pub vtable_rva: u32,
    pub field_0x0000: u64,
    pub field_0x0008: u32,
    pub field_0x001a: u8,
    pub field_0x001c: f32,
    pub field_0x0028: u32,
    pub field_0x002c: u32,
    pub field_0x0030: u32,
    pub field_0x0034: u32,
    pub field_0x0048: u64,
}

impl ccDamageParts {
    pub const CLASS_NAME: &'static str = "ccDamageParts";
    pub const VTABLE_RVA: u32 = 0x00881b40;
    pub const METHOD_COUNT: usize = 14;
    pub const BASE_CLASSES: &'static [&'static str] = &["ccDispBase"];
    pub const VIRTUAL_METHODS: &'static [u64] = &[0x0000000180232930, 0x00000001800d3090, 0x00000001802340b0, 0x00000001802341f0, 0x00000001802a0e30, 0x00000001800d3090, 0x00000001801b81f0, 0x00000001801b8200, 0x00000001801b8210, 0x00000001800d3090, 0x00000001800d3090, 0x0000000180234110, 0x00000001800d3090, 0x0000000180234180];

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
            field_0x0048: Default::default(),
        }
    }

    /// Decompiled virtual method #0 @ RVA 0x00232930 (13 instructions, 3 basic blocks)
    pub fn method_00(&mut self) {
        // Writes to field at offset +0x0000
    }

    /// Decompiled virtual method #1 @ RVA 0x000d3090 (1 instructions, 1 basic blocks)
    pub fn method_01(&mut self) {
    }

    /// Decompiled virtual method #2 @ RVA 0x002340b0 (21 instructions, 7 basic blocks)
    pub fn method_02(&mut self) {
    }

    /// Decompiled virtual method #3 @ RVA 0x002341f0 (125 instructions, 34 basic blocks)
    pub fn method_03(&mut self) {
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

    /// Decompiled virtual method #11 @ RVA 0x00234110 (26 instructions, 6 basic blocks)
    pub fn method_11(&mut self) {
        // Writes to field at offset +0x001c
        // Writes to field at offset +0x001c
    }

    /// Decompiled virtual method #12 @ RVA 0x000d3090 (1 instructions, 1 basic blocks)
    pub fn method_12(&mut self) {
    }

    /// Decompiled virtual method #13 @ RVA 0x00234180 (26 instructions, 6 basic blocks)
    pub fn method_13(&mut self) {
        // Writes to field at offset +0x001c
        // Writes to field at offset +0x001c
    }

}

impl CcClass for ccDamageParts {
    fn class_name(&self) -> &'static str { Self::CLASS_NAME }
    fn vtable_rva(&self) -> u32 { Self::VTABLE_RVA }
    fn virtual_method_count(&self) -> usize { Self::METHOD_COUNT }
    fn base_classes(&self) -> &'static [&'static str] { Self::BASE_CLASSES }
}

/// Decompiled CC2 class: `ccDestroyerBullet` (VTable RVA: 0x008be2b8, 14 virtual methods, 50 recovered fields)
/// Inherits: ccGobjGimBaseChangeCCS, ccGobjGimBase, ccGameObj
#[derive(Debug, Clone, Default)]
pub struct ccDestroyerBullet {
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
    pub field_0x0198: u32,
    pub field_0x019c: u16,
    pub field_0x019e: u16,
    pub field_0x01c0: u16,
    pub field_0x01c4: u64,
    pub field_0x01c8: u16,
    pub field_0x01cc: u16,
    pub field_0x01ce: u8,
    pub field_0x01f0: u64,
    pub field_0x01f8: u32,
    pub field_0x01fc: u32,
}

impl ccDestroyerBullet {
    pub const CLASS_NAME: &'static str = "ccDestroyerBullet";
    pub const VTABLE_RVA: u32 = 0x008be2b8;
    pub const METHOD_COUNT: usize = 14;
    pub const BASE_CLASSES: &'static [&'static str] = &["ccGobjGimBaseChangeCCS", "ccGobjGimBase", "ccGameObj"];
    pub const VIRTUAL_METHODS: &'static [u64] = &[0x00000001804f3df0, 0x00000001804f7870, 0x000000018045e4c0, 0x00000001800d3090, 0x000000018019e550, 0x00000001804f6040, 0x00000001800d3090, 0x00000001800d3090, 0x00000001800d3090, 0x00000001804f7b50, 0x000000018045e220, 0x000000018045e2d0, 0x000000018045eec0, 0x000000018045efd0];

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
            field_0x0198: Default::default(),
            field_0x019c: Default::default(),
            field_0x019e: Default::default(),
            field_0x01c0: Default::default(),
            field_0x01c4: Default::default(),
            field_0x01c8: Default::default(),
            field_0x01cc: Default::default(),
            field_0x01ce: Default::default(),
            field_0x01f0: Default::default(),
            field_0x01f8: Default::default(),
            field_0x01fc: Default::default(),
        }
    }

    /// Decompiled virtual method #0 @ RVA 0x004f3df0 (18 instructions, 3 basic blocks)
    pub fn method_00(&mut self) {
        // Writes to field at offset +0x0000
    }

    /// Decompiled virtual method #1 @ RVA 0x004f7870 (108 instructions, 16 basic blocks)
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

    /// Decompiled virtual method #9 @ RVA 0x004f7b50 (80 instructions, 7 basic blocks)
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

impl CcClass for ccDestroyerBullet {
    fn class_name(&self) -> &'static str { Self::CLASS_NAME }
    fn vtable_rva(&self) -> u32 { Self::VTABLE_RVA }
    fn virtual_method_count(&self) -> usize { Self::METHOD_COUNT }
    fn base_classes(&self) -> &'static [&'static str] { Self::BASE_CLASSES }
}

/// Decompiled CC2 class: `ccDirectLight` (VTable RVA: 0x00876f38, 2 virtual methods, 5 recovered fields)
/// Inherits: ccLight, ccCoord
#[derive(Debug, Clone, Default)]
pub struct ccDirectLight {
    pub vtable_rva: u32,
    pub field_0x0000: u64,
    pub field_0x00a1: u8,
    pub field_0x00d0: u64,
    pub field_0x0120: u32,
    pub field_0x0170: u32,
}

impl ccDirectLight {
    pub const CLASS_NAME: &'static str = "ccDirectLight";
    pub const VTABLE_RVA: u32 = 0x00876f38;
    pub const METHOD_COUNT: usize = 2;
    pub const BASE_CLASSES: &'static [&'static str] = &["ccLight", "ccCoord"];
    pub const VIRTUAL_METHODS: &'static [u64] = &[0x00000001801588e0, 0x000000018014c790];

    pub fn new() -> Self {
        Self {
            vtable_rva: Self::VTABLE_RVA,
            field_0x0000: Default::default(),
            field_0x00a1: Default::default(),
            field_0x00d0: Default::default(),
            field_0x0120: Default::default(),
            field_0x0170: Default::default(),
        }
    }

    /// Decompiled virtual method #0 @ RVA 0x001588e0 (22 instructions, 5 basic blocks)
    pub fn method_00(&mut self) {
        // Writes to field at offset +0x0000
    }

    /// Decompiled virtual method #1 @ RVA 0x0014c790 (94 instructions, 21 basic blocks)
    pub fn method_01(&mut self) {
        // Writes to field at offset +0x00a1
    }

}

impl CcClass for ccDirectLight {
    fn class_name(&self) -> &'static str { Self::CLASS_NAME }
    fn vtable_rva(&self) -> u32 { Self::VTABLE_RVA }
    fn virtual_method_count(&self) -> usize { Self::METHOD_COUNT }
    fn base_classes(&self) -> &'static [&'static str] { Self::BASE_CLASSES }
}

/// Decompiled CC2 class: `ccDistantLight` (VTable RVA: 0x00876f50, 2 virtual methods, 3 recovered fields)
/// Inherits: ccLight, ccCoord
#[derive(Debug, Clone, Default)]
pub struct ccDistantLight {
    pub vtable_rva: u32,
    pub field_0x0000: u64,
    pub field_0x00d0: u64,
    pub field_0x00e0: u32,
}

impl ccDistantLight {
    pub const CLASS_NAME: &'static str = "ccDistantLight";
    pub const VTABLE_RVA: u32 = 0x00876f50;
    pub const METHOD_COUNT: usize = 2;
    pub const BASE_CLASSES: &'static [&'static str] = &["ccLight", "ccCoord"];
    pub const VIRTUAL_METHODS: &'static [u64] = &[0x0000000180158a10, 0x000000018014c5a0];

    pub fn new() -> Self {
        Self {
            vtable_rva: Self::VTABLE_RVA,
            field_0x0000: Default::default(),
            field_0x00d0: Default::default(),
            field_0x00e0: Default::default(),
        }
    }

    /// Decompiled virtual method #0 @ RVA 0x00158a10 (22 instructions, 5 basic blocks)
    pub fn method_00(&mut self) {
        // Writes to field at offset +0x0000
    }

    /// Decompiled virtual method #1 @ RVA 0x0014c5a0 (13 instructions, 2 basic blocks)
    pub fn method_01(&mut self) {
    }

}

impl CcClass for ccDistantLight {
    fn class_name(&self) -> &'static str { Self::CLASS_NAME }
    fn vtable_rva(&self) -> u32 { Self::VTABLE_RVA }
    fn virtual_method_count(&self) -> usize { Self::METHOD_COUNT }
    fn base_classes(&self) -> &'static [&'static str] { Self::BASE_CLASSES }
}

/// Decompiled CC2 class: `ccDrawEnvCtrl` (VTable RVA: 0x00877260, 2 virtual methods, 1 recovered fields)
#[derive(Debug, Clone, Default)]
pub struct ccDrawEnvCtrl {
    pub vtable_rva: u32,
    pub field_0x0000: u64,
}

impl ccDrawEnvCtrl {
    pub const CLASS_NAME: &'static str = "ccDrawEnvCtrl";
    pub const VTABLE_RVA: u32 = 0x00877260;
    pub const METHOD_COUNT: usize = 2;
    pub const BASE_CLASSES: &'static [&'static str] = &[];
    pub const VIRTUAL_METHODS: &'static [u64] = &[0x00000001800d3090, 0x0000000180168af0];

    pub fn new() -> Self {
        Self {
            vtable_rva: Self::VTABLE_RVA,
            field_0x0000: Default::default(),
        }
    }

    /// Decompiled virtual method #0 @ RVA 0x000d3090 (1 instructions, 1 basic blocks)
    pub fn method_00(&mut self) {
    }

    /// Decompiled virtual method #1 @ RVA 0x00168af0 (13 instructions, 3 basic blocks)
    pub fn method_01(&mut self) {
        // Writes to field at offset +0x0000
    }

}

impl CcClass for ccDrawEnvCtrl {
    fn class_name(&self) -> &'static str { Self::CLASS_NAME }
    fn vtable_rva(&self) -> u32 { Self::VTABLE_RVA }
    fn virtual_method_count(&self) -> usize { Self::METHOD_COUNT }
    fn base_classes(&self) -> &'static [&'static str] { Self::BASE_CLASSES }
}

/// Decompiled CC2 class: `ccDtMcScreen` (VTable RVA: 0x008a5008, 21 virtual methods, 22 recovered fields)
/// Inherits: ccMcScreen
#[derive(Debug, Clone, Default)]
pub struct ccDtMcScreen {
    pub vtable_rva: u32,
    pub field_0x0000: u64,
    pub field_0x0004: u32,
    pub field_0x000c: u32,
    pub field_0x0011: u8,
    pub field_0x0014: u64,
    pub field_0x0016: u32,
    pub field_0x0018: f32,
    pub field_0x001a: u8,
    pub field_0x001c: f32,
    pub field_0x0020: f32,
    pub field_0x0024: f32,
    pub field_0x002c: u32,
    pub field_0x0030: u32,
    pub field_0x0070: u8,
    pub field_0x0218: u64,
    pub field_0x0220: u64,
    pub field_0x0818: f32,
    pub field_0x0820: u32,
    pub field_0x0838: u64,
    pub field_0x0840: u64,
    pub field_0x1ab8: f32,
    pub field_0x1abc: f32,
}

impl ccDtMcScreen {
    pub const CLASS_NAME: &'static str = "ccDtMcScreen";
    pub const VTABLE_RVA: u32 = 0x008a5008;
    pub const METHOD_COUNT: usize = 21;
    pub const BASE_CLASSES: &'static [&'static str] = &["ccMcScreen"];
    pub const VIRTUAL_METHODS: &'static [u64] = &[0x0000000180577910, 0x00000001800d3090, 0x0000000180577b70, 0x0000000180577300, 0x00000001805773d0, 0x0000000180577790, 0x00000001805778c0, 0x00000001805778e0, 0x0000000180578780, 0x0000000180578720, 0x0000000180578550, 0x0000000180578660, 0x00000001805786a0, 0x00000001805786e0, 0x0000000180578740, 0x00000001802b9cf0, 0x00000001802b9d30, 0x00000001802b9d70, 0x00000001805771f0, 0x0000000180577dd0, 0x0000000180577ef0];

    pub fn new() -> Self {
        Self {
            vtable_rva: Self::VTABLE_RVA,
            field_0x0000: Default::default(),
            field_0x0004: Default::default(),
            field_0x000c: Default::default(),
            field_0x0011: Default::default(),
            field_0x0014: Default::default(),
            field_0x0016: Default::default(),
            field_0x0018: Default::default(),
            field_0x001a: Default::default(),
            field_0x001c: Default::default(),
            field_0x0020: Default::default(),
            field_0x0024: Default::default(),
            field_0x002c: Default::default(),
            field_0x0030: Default::default(),
            field_0x0070: Default::default(),
            field_0x0218: Default::default(),
            field_0x0220: Default::default(),
            field_0x0818: Default::default(),
            field_0x0820: Default::default(),
            field_0x0838: Default::default(),
            field_0x0840: Default::default(),
            field_0x1ab8: Default::default(),
            field_0x1abc: Default::default(),
        }
    }

    /// Decompiled virtual method #0 @ RVA 0x00577910 (136 instructions, 35 basic blocks)
    pub fn method_00(&mut self) {
    }

    /// Decompiled virtual method #1 @ RVA 0x000d3090 (1 instructions, 1 basic blocks)
    pub fn method_01(&mut self) {
    }

    /// Decompiled virtual method #2 @ RVA 0x00577b70 (124 instructions, 19 basic blocks)
    pub fn method_02(&mut self) {
    }

    /// Decompiled virtual method #3 @ RVA 0x00577300 (48 instructions, 5 basic blocks)
    pub fn method_03(&mut self) {
        // Writes to field at offset +0x0004
    }

    /// Decompiled virtual method #4 @ RVA 0x005773d0 (146 instructions, 28 basic blocks)
    pub fn method_04(&mut self) {
        // Writes to field at offset +0x0000
        // Writes to field at offset +0x001a
    }

    /// Decompiled virtual method #5 @ RVA 0x00577790 (82 instructions, 11 basic blocks)
    pub fn method_05(&mut self) {
        // Writes to field at offset +0x0018
        // Writes to field at offset +0x001c
        // Writes to field at offset +0x0020
        // Writes to field at offset +0x0024
        // Writes to field at offset +0x0070
        // Writes to field at offset +0x1ab8
        // Writes to field at offset +0x1abc
    }

    /// Decompiled virtual method #6 @ RVA 0x005778c0 (5 instructions, 1 basic blocks)
    pub fn method_06(&mut self) {
        // Writes to field at offset +0x000c
        // Writes to field at offset +0x0011
        // Writes to field at offset +0x0818
        // Writes to field at offset +0x0820
    }

    /// Decompiled virtual method #7 @ RVA 0x005778e0 (6 instructions, 1 basic blocks)
    pub fn method_07(&mut self) {
        // Writes to field at offset +0x000c
        // Writes to field at offset +0x0011
        // Writes to field at offset +0x0818
        // Writes to field at offset +0x0820
    }

    /// Decompiled virtual method #8 @ RVA 0x00578780 (14 instructions, 2 basic blocks)
    pub fn method_08(&mut self) {
    }

    /// Decompiled virtual method #9 @ RVA 0x00578720 (9 instructions, 3 basic blocks)
    pub fn method_09(&mut self) {
    }

    /// Decompiled virtual method #10 @ RVA 0x00578550 (19 instructions, 6 basic blocks)
    pub fn method_10(&mut self) {
        // Writes to field at offset +0x0000
        // Writes to field at offset +0x0014
    }

    /// Decompiled virtual method #11 @ RVA 0x00578660 (15 instructions, 5 basic blocks)
    pub fn method_11(&mut self) {
        // Writes to field at offset +0x0004
        // Writes to field at offset +0x0014
    }

    /// Decompiled virtual method #12 @ RVA 0x005786a0 (12 instructions, 4 basic blocks)
    pub fn method_12(&mut self) {
    }

    /// Decompiled virtual method #13 @ RVA 0x005786e0 (12 instructions, 3 basic blocks)
    pub fn method_13(&mut self) {
    }

    /// Decompiled virtual method #14 @ RVA 0x00578740 (12 instructions, 1 basic blocks)
    pub fn method_14(&mut self) {
    }

    /// Decompiled virtual method #15 @ RVA 0x002b9cf0 (12 instructions, 1 basic blocks)
    pub fn method_15(&mut self) {
    }

    /// Decompiled virtual method #16 @ RVA 0x002b9d30 (12 instructions, 1 basic blocks)
    pub fn method_16(&mut self) {
    }

    /// Decompiled virtual method #17 @ RVA 0x002b9d70 (12 instructions, 1 basic blocks)
    pub fn method_17(&mut self) {
    }

    /// Decompiled virtual method #18 @ RVA 0x005771f0 (25 instructions, 2 basic blocks)
    pub fn method_18(&mut self) {
    }

    /// Decompiled virtual method #19 @ RVA 0x00577dd0 (53 instructions, 5 basic blocks)
    pub fn method_19(&mut self) {
        // Writes to field at offset +0x001a
    }

    /// Decompiled virtual method #20 @ RVA 0x00577ef0 (103 instructions, 8 basic blocks)
    pub fn method_20(&mut self) {
    }

}

impl CcClass for ccDtMcScreen {
    fn class_name(&self) -> &'static str { Self::CLASS_NAME }
    fn vtable_rva(&self) -> u32 { Self::VTABLE_RVA }
    fn virtual_method_count(&self) -> usize { Self::METHOD_COUNT }
    fn base_classes(&self) -> &'static [&'static str] { Self::BASE_CLASSES }
}

/// Decompiled CC2 class: `ccEffObj` (VTable RVA: 0x00876e88, 1 virtual methods, 1 recovered fields)
/// Inherits: ccCoord
#[derive(Debug, Clone, Default)]
pub struct ccEffObj {
    pub vtable_rva: u32,
    pub field_0x0000: u64,
}

impl ccEffObj {
    pub const CLASS_NAME: &'static str = "ccEffObj";
    pub const VTABLE_RVA: u32 = 0x00876e88;
    pub const METHOD_COUNT: usize = 1;
    pub const BASE_CLASSES: &'static [&'static str] = &["ccCoord"];
    pub const VIRTUAL_METHODS: &'static [u64] = &[0x0000000180158310];

    pub fn new() -> Self {
        Self {
            vtable_rva: Self::VTABLE_RVA,
            field_0x0000: Default::default(),
        }
    }

    /// Decompiled virtual method #0 @ RVA 0x00158310 (13 instructions, 3 basic blocks)
    pub fn method_00(&mut self) {
        // Writes to field at offset +0x0000
    }

}

impl CcClass for ccEffObj {
    fn class_name(&self) -> &'static str { Self::CLASS_NAME }
    fn vtable_rva(&self) -> u32 { Self::VTABLE_RVA }
    fn virtual_method_count(&self) -> usize { Self::METHOD_COUNT }
    fn base_classes(&self) -> &'static [&'static str] { Self::BASE_CLASSES }
}

/// Decompiled CC2 class: `ccEffectAnm` (VTable RVA: 0x0087f7e0, 2 virtual methods, 3 recovered fields)
/// Inherits: ccAnm, ccCoord
#[derive(Debug, Clone, Default)]
pub struct ccEffectAnm {
    pub vtable_rva: u32,
    pub field_0x0000: u64,
    pub field_0x009c: f32,
    pub field_0x01b0: u32,
}

impl ccEffectAnm {
    pub const CLASS_NAME: &'static str = "ccEffectAnm";
    pub const VTABLE_RVA: u32 = 0x0087f7e0;
    pub const METHOD_COUNT: usize = 2;
    pub const BASE_CLASSES: &'static [&'static str] = &["ccAnm", "ccCoord"];
    pub const VIRTUAL_METHODS: &'static [u64] = &[0x0000000180202630, 0x0000000180333940];

    pub fn new() -> Self {
        Self {
            vtable_rva: Self::VTABLE_RVA,
            field_0x0000: Default::default(),
            field_0x009c: Default::default(),
            field_0x01b0: Default::default(),
        }
    }

    /// Decompiled virtual method #0 @ RVA 0x00202630 (18 instructions, 3 basic blocks)
    pub fn method_00(&mut self) {
        // Writes to field at offset +0x0000
    }

    /// Decompiled virtual method #1 @ RVA 0x00333940 (40 instructions, 4 basic blocks)
    pub fn method_01(&mut self) {
        // Writes to field at offset +0x009c
    }

}

impl CcClass for ccEffectAnm {
    fn class_name(&self) -> &'static str { Self::CLASS_NAME }
    fn vtable_rva(&self) -> u32 { Self::VTABLE_RVA }
    fn virtual_method_count(&self) -> usize { Self::METHOD_COUNT }
    fn base_classes(&self) -> &'static [&'static str] { Self::BASE_CLASSES }
}

/// Decompiled CC2 class: `ccEffectClump` (VTable RVA: 0x008ae020, 1 virtual methods, 1 recovered fields)
/// Inherits: ccClump, ccCoord
#[derive(Debug, Clone, Default)]
pub struct ccEffectClump {
    pub vtable_rva: u32,
    pub field_0x0000: u64,
}

impl ccEffectClump {
    pub const CLASS_NAME: &'static str = "ccEffectClump";
    pub const VTABLE_RVA: u32 = 0x008ae020;
    pub const METHOD_COUNT: usize = 1;
    pub const BASE_CLASSES: &'static [&'static str] = &["ccClump", "ccCoord"];
    pub const VIRTUAL_METHODS: &'static [u64] = &[0x0000000180333590];

    pub fn new() -> Self {
        Self {
            vtable_rva: Self::VTABLE_RVA,
            field_0x0000: Default::default(),
        }
    }

    /// Decompiled virtual method #0 @ RVA 0x00333590 (18 instructions, 3 basic blocks)
    pub fn method_00(&mut self) {
        // Writes to field at offset +0x0000
    }

}

impl CcClass for ccEffectClump {
    fn class_name(&self) -> &'static str { Self::CLASS_NAME }
    fn vtable_rva(&self) -> u32 { Self::VTABLE_RVA }
    fn virtual_method_count(&self) -> usize { Self::METHOD_COUNT }
    fn base_classes(&self) -> &'static [&'static str] { Self::BASE_CLASSES }
}

/// Decompiled CC2 class: `ccEffectEffObj` (VTable RVA: 0x008808f8, 1 virtual methods, 1 recovered fields)
/// Inherits: ccEffObj, ccCoord
#[derive(Debug, Clone, Default)]
pub struct ccEffectEffObj {
    pub vtable_rva: u32,
    pub field_0x0000: u64,
}

impl ccEffectEffObj {
    pub const CLASS_NAME: &'static str = "ccEffectEffObj";
    pub const VTABLE_RVA: u32 = 0x008808f8;
    pub const METHOD_COUNT: usize = 1;
    pub const BASE_CLASSES: &'static [&'static str] = &["ccEffObj", "ccCoord"];
    pub const VIRTUAL_METHODS: &'static [u64] = &[0x0000000180220630];

    pub fn new() -> Self {
        Self {
            vtable_rva: Self::VTABLE_RVA,
            field_0x0000: Default::default(),
        }
    }

    /// Decompiled virtual method #0 @ RVA 0x00220630 (13 instructions, 3 basic blocks)
    pub fn method_00(&mut self) {
        // Writes to field at offset +0x0000
    }

}

impl CcClass for ccEffectEffObj {
    fn class_name(&self) -> &'static str { Self::CLASS_NAME }
    fn vtable_rva(&self) -> u32 { Self::VTABLE_RVA }
    fn virtual_method_count(&self) -> usize { Self::METHOD_COUNT }
    fn base_classes(&self) -> &'static [&'static str] { Self::BASE_CLASSES }
}

/// Decompiled CC2 class: `ccEffectObj` (VTable RVA: 0x0087f7d0, 1 virtual methods, 1 recovered fields)
/// Inherits: ccObj, ccCoord
#[derive(Debug, Clone, Default)]
pub struct ccEffectObj {
    pub vtable_rva: u32,
    pub field_0x0000: u64,
}

impl ccEffectObj {
    pub const CLASS_NAME: &'static str = "ccEffectObj";
    pub const VTABLE_RVA: u32 = 0x0087f7d0;
    pub const METHOD_COUNT: usize = 1;
    pub const BASE_CLASSES: &'static [&'static str] = &["ccObj", "ccCoord"];
    pub const VIRTUAL_METHODS: &'static [u64] = &[0x00000001802025c0];

    pub fn new() -> Self {
        Self {
            vtable_rva: Self::VTABLE_RVA,
            field_0x0000: Default::default(),
        }
    }

    /// Decompiled virtual method #0 @ RVA 0x002025c0 (18 instructions, 3 basic blocks)
    pub fn method_00(&mut self) {
        // Writes to field at offset +0x0000
    }

}

impl CcClass for ccEffectObj {
    fn class_name(&self) -> &'static str { Self::CLASS_NAME }
    fn vtable_rva(&self) -> u32 { Self::VTABLE_RVA }
    fn virtual_method_count(&self) -> usize { Self::METHOD_COUNT }
    fn base_classes(&self) -> &'static [&'static str] { Self::BASE_CLASSES }
}

/// Decompiled CC2 class: `ccFacePanel` (VTable RVA: 0x008a4e68, 21 virtual methods, 16 recovered fields)
/// Inherits: ccStatusPanelBase, ccDispTarget, ccDispBase
#[derive(Debug, Clone, Default)]
pub struct ccFacePanel {
    pub vtable_rva: u32,
    pub field_0x0000: u64,
    pub field_0x0008: u32,
    pub field_0x001a: u8,
    pub field_0x0028: u32,
    pub field_0x002c: u32,
    pub field_0x0030: u32,
    pub field_0x0034: u32,
    pub field_0x0048: u64,
    pub field_0x0050: u64,
    pub field_0x0054: u32,
    pub field_0x0058: u32,
    pub field_0x005c: u32,
    pub field_0x0060: u64,
    pub field_0x0080: u32,
    pub field_0x0084: u32,
    pub field_0x008c: u16,
}

impl ccFacePanel {
    pub const CLASS_NAME: &'static str = "ccFacePanel";
    pub const VTABLE_RVA: u32 = 0x008a4e68;
    pub const METHOD_COUNT: usize = 21;
    pub const BASE_CLASSES: &'static [&'static str] = &["ccStatusPanelBase", "ccDispTarget", "ccDispBase"];
    pub const VIRTUAL_METHODS: &'static [u64] = &[0x0000000180232c90, 0x00000001802b1d00, 0x000000018029ff50, 0x00000001802b1ef0, 0x00000001802a0e30, 0x00000001800d3090, 0x00000001802a6510, 0x00000001801b8200, 0x00000001801b8210, 0x00000001802a61f0, 0x00000001800d3090, 0x00000001802a63e0, 0x00000001800d3090, 0x00000001802a6470, 0x00000001802b20c0, 0x00000001802a00d0, 0x0000000180103e70, 0x00000001802a0040, 0x00000001802a7640, 0x00000001802a76c0, 0x00000001802a7740];

    pub fn new() -> Self {
        Self {
            vtable_rva: Self::VTABLE_RVA,
            field_0x0000: Default::default(),
            field_0x0008: Default::default(),
            field_0x001a: Default::default(),
            field_0x0028: Default::default(),
            field_0x002c: Default::default(),
            field_0x0030: Default::default(),
            field_0x0034: Default::default(),
            field_0x0048: Default::default(),
            field_0x0050: Default::default(),
            field_0x0054: Default::default(),
            field_0x0058: Default::default(),
            field_0x005c: Default::default(),
            field_0x0060: Default::default(),
            field_0x0080: Default::default(),
            field_0x0084: Default::default(),
            field_0x008c: Default::default(),
        }
    }

    /// Decompiled virtual method #0 @ RVA 0x00232c90 (13 instructions, 3 basic blocks)
    pub fn method_00(&mut self) {
        // Writes to field at offset +0x0000
    }

    /// Decompiled virtual method #1 @ RVA 0x002b1d00 (127 instructions, 11 basic blocks)
    pub fn method_01(&mut self) {
        // Writes to field at offset +0x0008
        // Writes to field at offset +0x0054
        // Writes to field at offset +0x0058
        // Writes to field at offset +0x005c
    }

    /// Decompiled virtual method #2 @ RVA 0x0029ff50 (66 instructions, 11 basic blocks)
    pub fn method_02(&mut self) {
    }

    /// Decompiled virtual method #3 @ RVA 0x002b1ef0 (103 instructions, 13 basic blocks)
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

    /// Decompiled virtual method #6 @ RVA 0x002a6510 (10 instructions, 5 basic blocks)
    pub fn method_06(&mut self) {
        // Writes to field at offset +0x0008
        // Writes to field at offset +0x0084
    }

    /// Decompiled virtual method #7 @ RVA 0x001b8200 (5 instructions, 1 basic blocks)
    pub fn method_07(&mut self) {
    }

    /// Decompiled virtual method #8 @ RVA 0x001b8210 (5 instructions, 1 basic blocks)
    pub fn method_08(&mut self) {
    }

    /// Decompiled virtual method #9 @ RVA 0x002a61f0 (84 instructions, 14 basic blocks)
    pub fn method_09(&mut self) {
    }

    /// Decompiled virtual method #10 @ RVA 0x000d3090 (1 instructions, 1 basic blocks)
    pub fn method_10(&mut self) {
    }

    /// Decompiled virtual method #11 @ RVA 0x002a63e0 (11 instructions, 2 basic blocks)
    pub fn method_11(&mut self) {
        // Writes to field at offset +0x0084
    }

    /// Decompiled virtual method #12 @ RVA 0x000d3090 (1 instructions, 1 basic blocks)
    pub fn method_12(&mut self) {
    }

    /// Decompiled virtual method #13 @ RVA 0x002a6470 (11 instructions, 2 basic blocks)
    pub fn method_13(&mut self) {
        // Writes to field at offset +0x0084
    }

    /// Decompiled virtual method #14 @ RVA 0x002b20c0 (94 instructions, 9 basic blocks)
    pub fn method_14(&mut self) {
        // Writes to field at offset +0x008c
    }

    /// Decompiled virtual method #15 @ RVA 0x002a00d0 (2 instructions, 1 basic blocks)
    pub fn method_15(&mut self) {
    }

    /// Decompiled virtual method #16 @ RVA 0x00103e70 (2 instructions, 1 basic blocks)
    pub fn method_16(&mut self) {
    }

    /// Decompiled virtual method #17 @ RVA 0x002a0040 (3 instructions, 1 basic blocks)
    pub fn method_17(&mut self) {
    }

    /// Decompiled virtual method #18 @ RVA 0x002a7640 (28 instructions, 3 basic blocks)
    pub fn method_18(&mut self) {
        // Writes to field at offset +0x001a
    }

    /// Decompiled virtual method #19 @ RVA 0x002a76c0 (25 instructions, 5 basic blocks)
    pub fn method_19(&mut self) {
    }

    /// Decompiled virtual method #20 @ RVA 0x002a7740 (26 instructions, 5 basic blocks)
    pub fn method_20(&mut self) {
    }

}

impl CcClass for ccFacePanel {
    fn class_name(&self) -> &'static str { Self::CLASS_NAME }
    fn vtable_rva(&self) -> u32 { Self::VTABLE_RVA }
    fn virtual_method_count(&self) -> usize { Self::METHOD_COUNT }
    fn base_classes(&self) -> &'static [&'static str] { Self::BASE_CLASSES }
}

/// Decompiled CC2 class: `ccFade` (VTable RVA: 0x00873bf0, 5 virtual methods, 5 recovered fields)
/// Inherits: ccFadeBasis
#[derive(Debug, Clone, Default)]
pub struct ccFade {
    pub vtable_rva: u32,
    pub field_0x0000: u64,
    pub field_0x001c: u32,
    pub field_0x0020: u32,
    pub field_0x0038: u64,
    pub field_0x003c: u32,
}

impl ccFade {
    pub const CLASS_NAME: &'static str = "ccFade";
    pub const VTABLE_RVA: u32 = 0x00873bf0;
    pub const METHOD_COUNT: usize = 5;
    pub const BASE_CLASSES: &'static [&'static str] = &["ccFadeBasis"];
    pub const VIRTUAL_METHODS: &'static [u64] = &[0x0000000180118250, 0x0000000180118260, 0x0000000180118560, 0x0000000180118500, 0x00000001801185a0];

    pub fn new() -> Self {
        Self {
            vtable_rva: Self::VTABLE_RVA,
            field_0x0000: Default::default(),
            field_0x001c: Default::default(),
            field_0x0020: Default::default(),
            field_0x0038: Default::default(),
            field_0x003c: Default::default(),
        }
    }

    /// Decompiled virtual method #0 @ RVA 0x00118250 (3 instructions, 1 basic blocks)
    pub fn method_00(&mut self) {
        // Writes to field at offset +0x0038
    }

    /// Decompiled virtual method #1 @ RVA 0x00118260 (3 instructions, 1 basic blocks)
    pub fn method_01(&mut self) {
        // Writes to field at offset +0x0038
    }

    /// Decompiled virtual method #2 @ RVA 0x00118560 (16 instructions, 1 basic blocks)
    pub fn method_02(&mut self) {
    }

    /// Decompiled virtual method #3 @ RVA 0x00118500 (26 instructions, 4 basic blocks)
    pub fn method_03(&mut self) {
    }

    /// Decompiled virtual method #4 @ RVA 0x001185a0 (108 instructions, 7 basic blocks)
    pub fn method_04(&mut self) {
    }

}

impl CcClass for ccFade {
    fn class_name(&self) -> &'static str { Self::CLASS_NAME }
    fn vtable_rva(&self) -> u32 { Self::VTABLE_RVA }
    fn virtual_method_count(&self) -> usize { Self::METHOD_COUNT }
    fn base_classes(&self) -> &'static [&'static str] { Self::BASE_CLASSES }
}

/// Decompiled CC2 class: `ccFadeColor` (VTable RVA: 0x00873bc0, 5 virtual methods, 11 recovered fields)
/// Inherits: ccFadeBasis
#[derive(Debug, Clone, Default)]
pub struct ccFadeColor {
    pub vtable_rva: u32,
    pub field_0x0000: u64,
    pub field_0x001c: u32,
    pub field_0x0020: u32,
    pub field_0x0038: u64,
    pub field_0x003c: u32,
    pub field_0x0040: u64,
    pub field_0x0044: u32,
    pub field_0x0048: u64,
    pub field_0x004c: u32,
    pub field_0x0050: u64,
    pub field_0x0054: u32,
}

impl ccFadeColor {
    pub const CLASS_NAME: &'static str = "ccFadeColor";
    pub const VTABLE_RVA: u32 = 0x00873bc0;
    pub const METHOD_COUNT: usize = 5;
    pub const BASE_CLASSES: &'static [&'static str] = &["ccFadeBasis"];
    pub const VIRTUAL_METHODS: &'static [u64] = &[0x00000001801187b0, 0x00000001801187d0, 0x0000000180118560, 0x0000000180118500, 0x00000001801187f0];

    pub fn new() -> Self {
        Self {
            vtable_rva: Self::VTABLE_RVA,
            field_0x0000: Default::default(),
            field_0x001c: Default::default(),
            field_0x0020: Default::default(),
            field_0x0038: Default::default(),
            field_0x003c: Default::default(),
            field_0x0040: Default::default(),
            field_0x0044: Default::default(),
            field_0x0048: Default::default(),
            field_0x004c: Default::default(),
            field_0x0050: Default::default(),
            field_0x0054: Default::default(),
        }
    }

    /// Decompiled virtual method #0 @ RVA 0x001187b0 (6 instructions, 1 basic blocks)
    pub fn method_00(&mut self) {
        // Writes to field at offset +0x0038
        // Writes to field at offset +0x0040
        // Writes to field at offset +0x0048
        // Writes to field at offset +0x0050
    }

    /// Decompiled virtual method #1 @ RVA 0x001187d0 (9 instructions, 1 basic blocks)
    pub fn method_01(&mut self) {
        // Writes to field at offset +0x0038
        // Writes to field at offset +0x003c
        // Writes to field at offset +0x0040
        // Writes to field at offset +0x0044
    }

    /// Decompiled virtual method #2 @ RVA 0x00118560 (16 instructions, 1 basic blocks)
    pub fn method_02(&mut self) {
    }

    /// Decompiled virtual method #3 @ RVA 0x00118500 (26 instructions, 4 basic blocks)
    pub fn method_03(&mut self) {
    }

    /// Decompiled virtual method #4 @ RVA 0x001187f0 (116 instructions, 2 basic blocks)
    pub fn method_04(&mut self) {
    }

}

impl CcClass for ccFadeColor {
    fn class_name(&self) -> &'static str { Self::CLASS_NAME }
    fn vtable_rva(&self) -> u32 { Self::VTABLE_RVA }
    fn virtual_method_count(&self) -> usize { Self::METHOD_COUNT }
    fn base_classes(&self) -> &'static [&'static str] { Self::BASE_CLASSES }
}

/// Decompiled CC2 class: `ccFadeMsg` (VTable RVA: 0x008ba278, 1 virtual methods, 1 recovered fields)
#[derive(Debug, Clone, Default)]
pub struct ccFadeMsg {
    pub vtable_rva: u32,
    pub field_0x0000: u64,
}

impl ccFadeMsg {
    pub const CLASS_NAME: &'static str = "ccFadeMsg";
    pub const VTABLE_RVA: u32 = 0x008ba278;
    pub const METHOD_COUNT: usize = 1;
    pub const BASE_CLASSES: &'static [&'static str] = &[];
    pub const VIRTUAL_METHODS: &'static [u64] = &[0x000000018044f030];

    pub fn new() -> Self {
        Self {
            vtable_rva: Self::VTABLE_RVA,
            field_0x0000: Default::default(),
        }
    }

    /// Decompiled virtual method #0 @ RVA 0x0044f030 (21 instructions, 3 basic blocks)
    pub fn method_00(&mut self) {
        // Writes to field at offset +0x0000
    }

}

impl CcClass for ccFadeMsg {
    fn class_name(&self) -> &'static str { Self::CLASS_NAME }
    fn vtable_rva(&self) -> u32 { Self::VTABLE_RVA }
    fn virtual_method_count(&self) -> usize { Self::METHOD_COUNT }
    fn base_classes(&self) -> &'static [&'static str] { Self::BASE_CLASSES }
}

/// Decompiled CC2 class: `ccFairy` (VTable RVA: 0x008714c8, 4 virtual methods, 0 recovered fields)
#[derive(Debug, Clone, Default)]
pub struct ccFairy {
    pub vtable_rva: u32,
}

impl ccFairy {
    pub const CLASS_NAME: &'static str = "ccFairy";
    pub const VTABLE_RVA: u32 = 0x008714c8;
    pub const METHOD_COUNT: usize = 4;
    pub const BASE_CLASSES: &'static [&'static str] = &[];
    pub const VIRTUAL_METHODS: &'static [u64] = &[0x00000001800d3090, 0x00000001800d3090, 0x00000001800d3090, 0x00000001800d3090];

    pub fn new() -> Self {
        Self {
            vtable_rva: Self::VTABLE_RVA,
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

}

impl CcClass for ccFairy {
    fn class_name(&self) -> &'static str { Self::CLASS_NAME }
    fn vtable_rva(&self) -> u32 { Self::VTABLE_RVA }
    fn virtual_method_count(&self) -> usize { Self::METHOD_COUNT }
    fn base_classes(&self) -> &'static [&'static str] { Self::BASE_CLASSES }
}

/// Decompiled CC2 class: `ccForceField2` (VTable RVA: 0x00865f90, 2 virtual methods, 1 recovered fields)
#[derive(Debug, Clone, Default)]
pub struct ccForceField2 {
    pub vtable_rva: u32,
    pub field_0x0000: u64,
}

impl ccForceField2 {
    pub const CLASS_NAME: &'static str = "ccForceField2";
    pub const VTABLE_RVA: u32 = 0x00865f90;
    pub const METHOD_COUNT: usize = 2;
    pub const BASE_CLASSES: &'static [&'static str] = &[];
    pub const VIRTUAL_METHODS: &'static [u64] = &[0x00000001800eabf0, 0x00000001800d3090];

    pub fn new() -> Self {
        Self {
            vtable_rva: Self::VTABLE_RVA,
            field_0x0000: Default::default(),
        }
    }

    /// Decompiled virtual method #0 @ RVA 0x000eabf0 (13 instructions, 3 basic blocks)
    pub fn method_00(&mut self) {
        // Writes to field at offset +0x0000
    }

    /// Decompiled virtual method #1 @ RVA 0x000d3090 (1 instructions, 1 basic blocks)
    pub fn method_01(&mut self) {
    }

}

impl CcClass for ccForceField2 {
    fn class_name(&self) -> &'static str { Self::CLASS_NAME }
    fn vtable_rva(&self) -> u32 { Self::VTABLE_RVA }
    fn virtual_method_count(&self) -> usize { Self::METHOD_COUNT }
    fn base_classes(&self) -> &'static [&'static str] { Self::BASE_CLASSES }
}

/// Decompiled CC2 class: `ccFragment2D` (VTable RVA: 0x00880f20, 14 virtual methods, 11 recovered fields)
/// Inherits: ccDispBase
#[derive(Debug, Clone, Default)]
pub struct ccFragment2D {
    pub vtable_rva: u32,
    pub field_0x0000: u64,
    pub field_0x0008: u32,
    pub field_0x001c: f32,
    pub field_0x0028: u32,
    pub field_0x002c: u32,
    pub field_0x0030: u32,
    pub field_0x0032: u16,
    pub field_0x0034: u16,
    pub field_0x0038: u32,
    pub field_0x0040: u64,
    pub field_0x0050: u64,
}

impl ccFragment2D {
    pub const CLASS_NAME: &'static str = "ccFragment2D";
    pub const VTABLE_RVA: u32 = 0x00880f20;
    pub const METHOD_COUNT: usize = 14;
    pub const BASE_CLASSES: &'static [&'static str] = &["ccDispBase"];
    pub const VIRTUAL_METHODS: &'static [u64] = &[0x000000018024a310, 0x00000001800d3090, 0x00000001800d3090, 0x0000000180249b20, 0x0000000180249af0, 0x00000001800d3090, 0x00000001801b81f0, 0x00000001801b8200, 0x00000001801b8210, 0x00000001800d3090, 0x00000001800d3090, 0x00000001800d3090, 0x00000001800d3090, 0x00000001800d3090];

    pub fn new() -> Self {
        Self {
            vtable_rva: Self::VTABLE_RVA,
            field_0x0000: Default::default(),
            field_0x0008: Default::default(),
            field_0x001c: Default::default(),
            field_0x0028: Default::default(),
            field_0x002c: Default::default(),
            field_0x0030: Default::default(),
            field_0x0032: Default::default(),
            field_0x0034: Default::default(),
            field_0x0038: Default::default(),
            field_0x0040: Default::default(),
            field_0x0050: Default::default(),
        }
    }

    /// Decompiled virtual method #0 @ RVA 0x0024a310 (36 instructions, 8 basic blocks)
    pub fn method_00(&mut self) {
        // Writes to field at offset +0x0000
    }

    /// Decompiled virtual method #1 @ RVA 0x000d3090 (1 instructions, 1 basic blocks)
    pub fn method_01(&mut self) {
    }

    /// Decompiled virtual method #2 @ RVA 0x000d3090 (1 instructions, 1 basic blocks)
    pub fn method_02(&mut self) {
    }

    /// Decompiled virtual method #3 @ RVA 0x00249b20 (56 instructions, 8 basic blocks)
    pub fn method_03(&mut self) {
        // Writes to field at offset +0x0038
    }

    /// Decompiled virtual method #4 @ RVA 0x00249af0 (12 instructions, 5 basic blocks)
    pub fn method_04(&mut self) {
        // Writes to field at offset +0x0038
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

impl CcClass for ccFragment2D {
    fn class_name(&self) -> &'static str { Self::CLASS_NAME }
    fn vtable_rva(&self) -> u32 { Self::VTABLE_RVA }
    fn virtual_method_count(&self) -> usize { Self::METHOD_COUNT }
    fn base_classes(&self) -> &'static [&'static str] { Self::BASE_CLASSES }
}

/// Decompiled CC2 class: `ccFrameBase` (VTable RVA: 0x00866948, 2 virtual methods, 2 recovered fields)
#[derive(Debug, Clone, Default)]
pub struct ccFrameBase {
    pub vtable_rva: u32,
    pub field_0x0000: u64,
    pub field_0x0058: u64,
}

impl ccFrameBase {
    pub const CLASS_NAME: &'static str = "ccFrameBase";
    pub const VTABLE_RVA: u32 = 0x00866948;
    pub const METHOD_COUNT: usize = 2;
    pub const BASE_CLASSES: &'static [&'static str] = &[];
    pub const VIRTUAL_METHODS: &'static [u64] = &[0x00000001800ff220, 0x00000001800ff250];

    pub fn new() -> Self {
        Self {
            vtable_rva: Self::VTABLE_RVA,
            field_0x0000: Default::default(),
            field_0x0058: Default::default(),
        }
    }

    /// Decompiled virtual method #0 @ RVA 0x000ff220 (13 instructions, 3 basic blocks)
    pub fn method_00(&mut self) {
        // Writes to field at offset +0x0000
    }

    /// Decompiled virtual method #1 @ RVA 0x000ff250 (41 instructions, 5 basic blocks)
    pub fn method_01(&mut self) {
    }

}

impl CcClass for ccFrameBase {
    fn class_name(&self) -> &'static str { Self::CLASS_NAME }
    fn vtable_rva(&self) -> u32 { Self::VTABLE_RVA }
    fn virtual_method_count(&self) -> usize { Self::METHOD_COUNT }
    fn base_classes(&self) -> &'static [&'static str] { Self::BASE_CLASSES }
}

/// Decompiled CC2 class: `ccGACome` (VTable RVA: 0x008c0cb0, 8 virtual methods, 8 recovered fields)
/// Inherits: ccGrtAnmBase
#[derive(Debug, Clone, Default)]
pub struct ccGACome {
    pub vtable_rva: u32,
    pub field_0x0000: u64,
    pub field_0x0008: u64,
    pub field_0x0020: u32,
    pub field_0x0024: f32,
    pub field_0x0028: u32,
    pub field_0x002c: f32,
    pub field_0x0030: u32,
    pub field_0x0070: u32,
}

impl ccGACome {
    pub const CLASS_NAME: &'static str = "ccGACome";
    pub const VTABLE_RVA: u32 = 0x008c0cb0;
    pub const METHOD_COUNT: usize = 8;
    pub const BASE_CLASSES: &'static [&'static str] = &["ccGrtAnmBase"];
    pub const VIRTUAL_METHODS: &'static [u64] = &[0x00000001800d3090, 0x00000001801d8780, 0x000000018052c420, 0x00000001800d3090, 0x000000018052be00, 0x00000001800d3090, 0x0000000180529940, 0x000000018052a4f0];

    pub fn new() -> Self {
        Self {
            vtable_rva: Self::VTABLE_RVA,
            field_0x0000: Default::default(),
            field_0x0008: Default::default(),
            field_0x0020: Default::default(),
            field_0x0024: Default::default(),
            field_0x0028: Default::default(),
            field_0x002c: Default::default(),
            field_0x0030: Default::default(),
            field_0x0070: Default::default(),
        }
    }

    /// Decompiled virtual method #0 @ RVA 0x000d3090 (1 instructions, 1 basic blocks)
    pub fn method_00(&mut self) {
    }

    /// Decompiled virtual method #1 @ RVA 0x001d8780 (21 instructions, 3 basic blocks)
    pub fn method_01(&mut self) {
        // Writes to field at offset +0x0070
    }

    /// Decompiled virtual method #2 @ RVA 0x0052c420 (73 instructions, 12 basic blocks)
    pub fn method_02(&mut self) {
    }

    /// Decompiled virtual method #3 @ RVA 0x000d3090 (1 instructions, 1 basic blocks)
    pub fn method_03(&mut self) {
    }

    /// Decompiled virtual method #4 @ RVA 0x0052be00 (16 instructions, 1 basic blocks)
    pub fn method_04(&mut self) {
    }

    /// Decompiled virtual method #5 @ RVA 0x000d3090 (1 instructions, 1 basic blocks)
    pub fn method_05(&mut self) {
    }

    /// Decompiled virtual method #6 @ RVA 0x00529940 (21 instructions, 3 basic blocks)
    pub fn method_06(&mut self) {
        // Writes to field at offset +0x0000
    }

    /// Decompiled virtual method #7 @ RVA 0x0052a4f0 (27 instructions, 6 basic blocks)
    pub fn method_07(&mut self) {
    }

}

impl CcClass for ccGACome {
    fn class_name(&self) -> &'static str { Self::CLASS_NAME }
    fn vtable_rva(&self) -> u32 { Self::VTABLE_RVA }
    fn virtual_method_count(&self) -> usize { Self::METHOD_COUNT }
    fn base_classes(&self) -> &'static [&'static str] { Self::BASE_CLASSES }
}

/// Decompiled CC2 class: `ccGALikeFood` (VTable RVA: 0x008c0b70, 8 virtual methods, 8 recovered fields)
/// Inherits: ccGrtAnmBase
#[derive(Debug, Clone, Default)]
pub struct ccGALikeFood {
    pub vtable_rva: u32,
    pub field_0x0000: u64,
    pub field_0x0008: u64,
    pub field_0x0020: u32,
    pub field_0x0024: f32,
    pub field_0x0028: u32,
    pub field_0x002c: f32,
    pub field_0x0030: u32,
    pub field_0x0070: u32,
}

impl ccGALikeFood {
    pub const CLASS_NAME: &'static str = "ccGALikeFood";
    pub const VTABLE_RVA: u32 = 0x008c0b70;
    pub const METHOD_COUNT: usize = 8;
    pub const BASE_CLASSES: &'static [&'static str] = &["ccGrtAnmBase"];
    pub const VIRTUAL_METHODS: &'static [u64] = &[0x00000001800d3090, 0x00000001801d8780, 0x000000018052bd70, 0x00000001800d3090, 0x000000018052be00, 0x00000001800d3090, 0x0000000180529940, 0x000000018052a4f0];

    pub fn new() -> Self {
        Self {
            vtable_rva: Self::VTABLE_RVA,
            field_0x0000: Default::default(),
            field_0x0008: Default::default(),
            field_0x0020: Default::default(),
            field_0x0024: Default::default(),
            field_0x0028: Default::default(),
            field_0x002c: Default::default(),
            field_0x0030: Default::default(),
            field_0x0070: Default::default(),
        }
    }

    /// Decompiled virtual method #0 @ RVA 0x000d3090 (1 instructions, 1 basic blocks)
    pub fn method_00(&mut self) {
    }

    /// Decompiled virtual method #1 @ RVA 0x001d8780 (21 instructions, 3 basic blocks)
    pub fn method_01(&mut self) {
        // Writes to field at offset +0x0070
    }

    /// Decompiled virtual method #2 @ RVA 0x0052bd70 (57 instructions, 5 basic blocks)
    pub fn method_02(&mut self) {
    }

    /// Decompiled virtual method #3 @ RVA 0x000d3090 (1 instructions, 1 basic blocks)
    pub fn method_03(&mut self) {
    }

    /// Decompiled virtual method #4 @ RVA 0x0052be00 (16 instructions, 1 basic blocks)
    pub fn method_04(&mut self) {
    }

    /// Decompiled virtual method #5 @ RVA 0x000d3090 (1 instructions, 1 basic blocks)
    pub fn method_05(&mut self) {
    }

    /// Decompiled virtual method #6 @ RVA 0x00529940 (21 instructions, 3 basic blocks)
    pub fn method_06(&mut self) {
        // Writes to field at offset +0x0000
    }

    /// Decompiled virtual method #7 @ RVA 0x0052a4f0 (27 instructions, 6 basic blocks)
    pub fn method_07(&mut self) {
    }

}

impl CcClass for ccGALikeFood {
    fn class_name(&self) -> &'static str { Self::CLASS_NAME }
    fn vtable_rva(&self) -> u32 { Self::VTABLE_RVA }
    fn virtual_method_count(&self) -> usize { Self::METHOD_COUNT }
    fn base_classes(&self) -> &'static [&'static str] { Self::BASE_CLASSES }
}

/// Decompiled CC2 class: `ccGARegard` (VTable RVA: 0x008c0d58, 8 virtual methods, 12 recovered fields)
/// Inherits: ccGrtAnmBase
#[derive(Debug, Clone, Default)]
pub struct ccGARegard {
    pub vtable_rva: u32,
    pub field_0x0000: u64,
    pub field_0x0008: u64,
    pub field_0x000c: u32,
    pub field_0x0010: f32,
    pub field_0x0014: u32,
    pub field_0x0018: f32,
    pub field_0x001c: u32,
    pub field_0x0020: u32,
    pub field_0x0024: f32,
    pub field_0x0028: u32,
    pub field_0x002c: f32,
    pub field_0x0070: u32,
}

impl ccGARegard {
    pub const CLASS_NAME: &'static str = "ccGARegard";
    pub const VTABLE_RVA: u32 = 0x008c0d58;
    pub const METHOD_COUNT: usize = 8;
    pub const BASE_CLASSES: &'static [&'static str] = &["ccGrtAnmBase"];
    pub const VIRTUAL_METHODS: &'static [u64] = &[0x00000001800d3090, 0x00000001801d8780, 0x000000018052b920, 0x00000001800d3090, 0x000000018052ba20, 0x00000001800d3090, 0x0000000180529940, 0x000000018052bab0];

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
            field_0x0024: Default::default(),
            field_0x0028: Default::default(),
            field_0x002c: Default::default(),
            field_0x0070: Default::default(),
        }
    }

    /// Decompiled virtual method #0 @ RVA 0x000d3090 (1 instructions, 1 basic blocks)
    pub fn method_00(&mut self) {
    }

    /// Decompiled virtual method #1 @ RVA 0x001d8780 (21 instructions, 3 basic blocks)
    pub fn method_01(&mut self) {
        // Writes to field at offset +0x0070
    }

    /// Decompiled virtual method #2 @ RVA 0x0052b920 (101 instructions, 8 basic blocks)
    pub fn method_02(&mut self) {
        // Writes to field at offset +0x000c
        // Writes to field at offset +0x0010
        // Writes to field at offset +0x0014
        // Writes to field at offset +0x0018
        // Writes to field at offset +0x001c
    }

    /// Decompiled virtual method #3 @ RVA 0x000d3090 (1 instructions, 1 basic blocks)
    pub fn method_03(&mut self) {
    }

    /// Decompiled virtual method #4 @ RVA 0x0052ba20 (32 instructions, 1 basic blocks)
    pub fn method_04(&mut self) {
        // Writes to field at offset +0x000c
        // Writes to field at offset +0x0010
        // Writes to field at offset +0x0014
        // Writes to field at offset +0x0018
        // Writes to field at offset +0x001c
    }

    /// Decompiled virtual method #5 @ RVA 0x000d3090 (1 instructions, 1 basic blocks)
    pub fn method_05(&mut self) {
    }

    /// Decompiled virtual method #6 @ RVA 0x00529940 (21 instructions, 3 basic blocks)
    pub fn method_06(&mut self) {
        // Writes to field at offset +0x0000
    }

    /// Decompiled virtual method #7 @ RVA 0x0052bab0 (38 instructions, 8 basic blocks)
    pub fn method_07(&mut self) {
    }

}

impl CcClass for ccGARegard {
    fn class_name(&self) -> &'static str { Self::CLASS_NAME }
    fn vtable_rva(&self) -> u32 { Self::VTABLE_RVA }
    fn virtual_method_count(&self) -> usize { Self::METHOD_COUNT }
    fn base_classes(&self) -> &'static [&'static str] { Self::BASE_CLASSES }
}

/// Decompiled CC2 class: `ccGASpirits` (VTable RVA: 0x008c0c10, 8 virtual methods, 11 recovered fields)
/// Inherits: ccGrtAnmBase
#[derive(Debug, Clone, Default)]
pub struct ccGASpirits {
    pub vtable_rva: u32,
    pub field_0x0000: u64,
    pub field_0x0008: u64,
    pub field_0x000c: u32,
    pub field_0x0010: f32,
    pub field_0x0014: u32,
    pub field_0x0018: f32,
    pub field_0x001c: u32,
    pub field_0x0020: u32,
    pub field_0x0024: f32,
    pub field_0x0028: u32,
    pub field_0x002c: f32,
}

impl ccGASpirits {
    pub const CLASS_NAME: &'static str = "ccGASpirits";
    pub const VTABLE_RVA: u32 = 0x008c0c10;
    pub const METHOD_COUNT: usize = 8;
    pub const BASE_CLASSES: &'static [&'static str] = &["ccGrtAnmBase"];
    pub const VIRTUAL_METHODS: &'static [u64] = &[0x00000001800d3090, 0x00000001800d3090, 0x000000018052c060, 0x00000001800d3090, 0x000000018052ba20, 0x00000001800d3090, 0x0000000180529940, 0x000000018052c160];

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
            field_0x0024: Default::default(),
            field_0x0028: Default::default(),
            field_0x002c: Default::default(),
        }
    }

    /// Decompiled virtual method #0 @ RVA 0x000d3090 (1 instructions, 1 basic blocks)
    pub fn method_00(&mut self) {
    }

    /// Decompiled virtual method #1 @ RVA 0x000d3090 (1 instructions, 1 basic blocks)
    pub fn method_01(&mut self) {
    }

    /// Decompiled virtual method #2 @ RVA 0x0052c060 (108 instructions, 15 basic blocks)
    pub fn method_02(&mut self) {
    }

    /// Decompiled virtual method #3 @ RVA 0x000d3090 (1 instructions, 1 basic blocks)
    pub fn method_03(&mut self) {
    }

    /// Decompiled virtual method #4 @ RVA 0x0052ba20 (32 instructions, 1 basic blocks)
    pub fn method_04(&mut self) {
        // Writes to field at offset +0x000c
        // Writes to field at offset +0x0010
        // Writes to field at offset +0x0014
        // Writes to field at offset +0x0018
        // Writes to field at offset +0x001c
    }

    /// Decompiled virtual method #5 @ RVA 0x000d3090 (1 instructions, 1 basic blocks)
    pub fn method_05(&mut self) {
    }

    /// Decompiled virtual method #6 @ RVA 0x00529940 (21 instructions, 3 basic blocks)
    pub fn method_06(&mut self) {
        // Writes to field at offset +0x0000
    }

    /// Decompiled virtual method #7 @ RVA 0x0052c160 (39 instructions, 8 basic blocks)
    pub fn method_07(&mut self) {
    }

}

impl CcClass for ccGASpirits {
    fn class_name(&self) -> &'static str { Self::CLASS_NAME }
    fn vtable_rva(&self) -> u32 { Self::VTABLE_RVA }
    fn virtual_method_count(&self) -> usize { Self::METHOD_COUNT }
    fn base_classes(&self) -> &'static [&'static str] { Self::BASE_CLASSES }
}

/// Decompiled CC2 class: `ccGAThanks` (VTable RVA: 0x008c0ad0, 8 virtual methods, 8 recovered fields)
/// Inherits: ccGrtAnmBase
#[derive(Debug, Clone, Default)]
pub struct ccGAThanks {
    pub vtable_rva: u32,
    pub field_0x0000: u64,
    pub field_0x0008: u64,
    pub field_0x0020: u32,
    pub field_0x0024: f32,
    pub field_0x0028: u32,
    pub field_0x002c: f32,
    pub field_0x0030: u32,
    pub field_0x0070: u32,
}

impl ccGAThanks {
    pub const CLASS_NAME: &'static str = "ccGAThanks";
    pub const VTABLE_RVA: u32 = 0x008c0ad0;
    pub const METHOD_COUNT: usize = 8;
    pub const BASE_CLASSES: &'static [&'static str] = &["ccGrtAnmBase"];
    pub const VIRTUAL_METHODS: &'static [u64] = &[0x00000001800d3090, 0x00000001801d8780, 0x000000018052c6d0, 0x00000001800d3090, 0x000000018052be00, 0x00000001800d3090, 0x0000000180529940, 0x000000018052a4f0];

    pub fn new() -> Self {
        Self {
            vtable_rva: Self::VTABLE_RVA,
            field_0x0000: Default::default(),
            field_0x0008: Default::default(),
            field_0x0020: Default::default(),
            field_0x0024: Default::default(),
            field_0x0028: Default::default(),
            field_0x002c: Default::default(),
            field_0x0030: Default::default(),
            field_0x0070: Default::default(),
        }
    }

    /// Decompiled virtual method #0 @ RVA 0x000d3090 (1 instructions, 1 basic blocks)
    pub fn method_00(&mut self) {
    }

    /// Decompiled virtual method #1 @ RVA 0x001d8780 (21 instructions, 3 basic blocks)
    pub fn method_01(&mut self) {
        // Writes to field at offset +0x0070
    }

    /// Decompiled virtual method #2 @ RVA 0x0052c6d0 (75 instructions, 10 basic blocks)
    pub fn method_02(&mut self) {
    }

    /// Decompiled virtual method #3 @ RVA 0x000d3090 (1 instructions, 1 basic blocks)
    pub fn method_03(&mut self) {
    }

    /// Decompiled virtual method #4 @ RVA 0x0052be00 (16 instructions, 1 basic blocks)
    pub fn method_04(&mut self) {
    }

    /// Decompiled virtual method #5 @ RVA 0x000d3090 (1 instructions, 1 basic blocks)
    pub fn method_05(&mut self) {
    }

    /// Decompiled virtual method #6 @ RVA 0x00529940 (21 instructions, 3 basic blocks)
    pub fn method_06(&mut self) {
        // Writes to field at offset +0x0000
    }

    /// Decompiled virtual method #7 @ RVA 0x0052a4f0 (27 instructions, 6 basic blocks)
    pub fn method_07(&mut self) {
    }

}

impl CcClass for ccGAThanks {
    fn class_name(&self) -> &'static str { Self::CLASS_NAME }
    fn vtable_rva(&self) -> u32 { Self::VTABLE_RVA }
    fn virtual_method_count(&self) -> usize { Self::METHOD_COUNT }
    fn base_classes(&self) -> &'static [&'static str] { Self::BASE_CLASSES }
}

/// Decompiled CC2 class: `ccGCCome` (VTable RVA: 0x008c0c58, 10 virtual methods, 2 recovered fields)
/// Inherits: ccGrtCardBase
#[derive(Debug, Clone, Default)]
pub struct ccGCCome {
    pub vtable_rva: u32,
    pub field_0x0000: u64,
    pub field_0x0088: u64,
}

impl ccGCCome {
    pub const CLASS_NAME: &'static str = "ccGCCome";
    pub const VTABLE_RVA: u32 = 0x008c0c58;
    pub const METHOD_COUNT: usize = 10;
    pub const BASE_CLASSES: &'static [&'static str] = &["ccGrtCardBase"];
    pub const VIRTUAL_METHODS: &'static [u64] = &[0x000000018052c200, 0x000000018052c220, 0x000000018052c290, 0x00000001800d3090, 0x00000001800d3090, 0x000000018052c350, 0x00000001800d3090, 0x00000001800d3090, 0x00000001800d3090, 0x0000000180529990];

    pub fn new() -> Self {
        Self {
            vtable_rva: Self::VTABLE_RVA,
            field_0x0000: Default::default(),
            field_0x0088: Default::default(),
        }
    }

    /// Decompiled virtual method #0 @ RVA 0x0052c200 (32 instructions, 8 basic blocks)
    pub fn method_00(&mut self) {
    }

    /// Decompiled virtual method #1 @ RVA 0x0052c220 (26 instructions, 7 basic blocks)
    pub fn method_01(&mut self) {
    }

    /// Decompiled virtual method #2 @ RVA 0x0052c290 (7 instructions, 1 basic blocks)
    pub fn method_02(&mut self) {
    }

    /// Decompiled virtual method #3 @ RVA 0x000d3090 (1 instructions, 1 basic blocks)
    pub fn method_03(&mut self) {
    }

    /// Decompiled virtual method #4 @ RVA 0x000d3090 (1 instructions, 1 basic blocks)
    pub fn method_04(&mut self) {
    }

    /// Decompiled virtual method #5 @ RVA 0x0052c350 (133 instructions, 16 basic blocks)
    pub fn method_05(&mut self) {
        // Writes to field at offset +0x0088
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

    /// Decompiled virtual method #9 @ RVA 0x00529990 (34 instructions, 5 basic blocks)
    pub fn method_09(&mut self) {
        // Writes to field at offset +0x0000
    }

}

impl CcClass for ccGCCome {
    fn class_name(&self) -> &'static str { Self::CLASS_NAME }
    fn vtable_rva(&self) -> u32 { Self::VTABLE_RVA }
    fn virtual_method_count(&self) -> usize { Self::METHOD_COUNT }
    fn base_classes(&self) -> &'static [&'static str] { Self::BASE_CLASSES }
}

/// Decompiled CC2 class: `ccGCFood` (VTable RVA: 0x008c0b18, 10 virtual methods, 8 recovered fields)
/// Inherits: ccGrtCardBase
#[derive(Debug, Clone, Default)]
pub struct ccGCFood {
    pub vtable_rva: u32,
    pub field_0x0000: u64,
    pub field_0x0008: u64,
    pub field_0x0020: u32,
    pub field_0x0024: f32,
    pub field_0x0028: u32,
    pub field_0x002c: f32,
    pub field_0x0030: u32,
    pub field_0x0088: u64,
}

impl ccGCFood {
    pub const CLASS_NAME: &'static str = "ccGCFood";
    pub const VTABLE_RVA: u32 = 0x008c0b18;
    pub const METHOD_COUNT: usize = 10;
    pub const BASE_CLASSES: &'static [&'static str] = &["ccGrtCardBase"];
    pub const VIRTUAL_METHODS: &'static [u64] = &[0x000000018052bb50, 0x000000018052bb70, 0x000000018052bbe0, 0x00000001800d3090, 0x00000001800d3090, 0x000000018052bca0, 0x00000001800d3090, 0x00000001800d3090, 0x00000001800d3090, 0x0000000180529990];

    pub fn new() -> Self {
        Self {
            vtable_rva: Self::VTABLE_RVA,
            field_0x0000: Default::default(),
            field_0x0008: Default::default(),
            field_0x0020: Default::default(),
            field_0x0024: Default::default(),
            field_0x0028: Default::default(),
            field_0x002c: Default::default(),
            field_0x0030: Default::default(),
            field_0x0088: Default::default(),
        }
    }

    /// Decompiled virtual method #0 @ RVA 0x0052bb50 (32 instructions, 8 basic blocks)
    pub fn method_00(&mut self) {
    }

    /// Decompiled virtual method #1 @ RVA 0x0052bb70 (26 instructions, 7 basic blocks)
    pub fn method_01(&mut self) {
    }

    /// Decompiled virtual method #2 @ RVA 0x0052bbe0 (7 instructions, 1 basic blocks)
    pub fn method_02(&mut self) {
    }

    /// Decompiled virtual method #3 @ RVA 0x000d3090 (1 instructions, 1 basic blocks)
    pub fn method_03(&mut self) {
    }

    /// Decompiled virtual method #4 @ RVA 0x000d3090 (1 instructions, 1 basic blocks)
    pub fn method_04(&mut self) {
    }

    /// Decompiled virtual method #5 @ RVA 0x0052bca0 (117 instructions, 9 basic blocks)
    pub fn method_05(&mut self) {
        // Writes to field at offset +0x0088
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

    /// Decompiled virtual method #9 @ RVA 0x00529990 (34 instructions, 5 basic blocks)
    pub fn method_09(&mut self) {
        // Writes to field at offset +0x0000
    }

}

impl CcClass for ccGCFood {
    fn class_name(&self) -> &'static str { Self::CLASS_NAME }
    fn vtable_rva(&self) -> u32 { Self::VTABLE_RVA }
    fn virtual_method_count(&self) -> usize { Self::METHOD_COUNT }
    fn base_classes(&self) -> &'static [&'static str] { Self::BASE_CLASSES }
}

/// Decompiled CC2 class: `ccGCRegard` (VTable RVA: 0x008c0d00, 10 virtual methods, 7 recovered fields)
/// Inherits: ccGrtCardBase
#[derive(Debug, Clone, Default)]
pub struct ccGCRegard {
    pub vtable_rva: u32,
    pub field_0x0000: u64,
    pub field_0x0008: u64,
    pub field_0x0020: u32,
    pub field_0x0024: f32,
    pub field_0x0028: u32,
    pub field_0x002c: f32,
    pub field_0x0088: u64,
}

impl ccGCRegard {
    pub const CLASS_NAME: &'static str = "ccGCRegard";
    pub const VTABLE_RVA: u32 = 0x008c0d00;
    pub const METHOD_COUNT: usize = 10;
    pub const BASE_CLASSES: &'static [&'static str] = &["ccGrtCardBase"];
    pub const VIRTUAL_METHODS: &'static [u64] = &[0x000000018052b700, 0x000000018052b720, 0x000000018052b790, 0x00000001800d3090, 0x00000001800d3090, 0x000000018052b850, 0x00000001800d3090, 0x00000001800d3090, 0x00000001800d3090, 0x0000000180529990];

    pub fn new() -> Self {
        Self {
            vtable_rva: Self::VTABLE_RVA,
            field_0x0000: Default::default(),
            field_0x0008: Default::default(),
            field_0x0020: Default::default(),
            field_0x0024: Default::default(),
            field_0x0028: Default::default(),
            field_0x002c: Default::default(),
            field_0x0088: Default::default(),
        }
    }

    /// Decompiled virtual method #0 @ RVA 0x0052b700 (32 instructions, 8 basic blocks)
    pub fn method_00(&mut self) {
    }

    /// Decompiled virtual method #1 @ RVA 0x0052b720 (26 instructions, 7 basic blocks)
    pub fn method_01(&mut self) {
    }

    /// Decompiled virtual method #2 @ RVA 0x0052b790 (7 instructions, 1 basic blocks)
    pub fn method_02(&mut self) {
    }

    /// Decompiled virtual method #3 @ RVA 0x000d3090 (1 instructions, 1 basic blocks)
    pub fn method_03(&mut self) {
    }

    /// Decompiled virtual method #4 @ RVA 0x000d3090 (1 instructions, 1 basic blocks)
    pub fn method_04(&mut self) {
    }

    /// Decompiled virtual method #5 @ RVA 0x0052b850 (142 instructions, 12 basic blocks)
    pub fn method_05(&mut self) {
        // Writes to field at offset +0x0088
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

    /// Decompiled virtual method #9 @ RVA 0x00529990 (34 instructions, 5 basic blocks)
    pub fn method_09(&mut self) {
        // Writes to field at offset +0x0000
    }

}

impl CcClass for ccGCRegard {
    fn class_name(&self) -> &'static str { Self::CLASS_NAME }
    fn vtable_rva(&self) -> u32 { Self::VTABLE_RVA }
    fn virtual_method_count(&self) -> usize { Self::METHOD_COUNT }
    fn base_classes(&self) -> &'static [&'static str] { Self::BASE_CLASSES }
}

/// Decompiled CC2 class: `ccGCSpirits` (VTable RVA: 0x008c0bb8, 10 virtual methods, 3 recovered fields)
/// Inherits: ccGrtCardBase
#[derive(Debug, Clone, Default)]
pub struct ccGCSpirits {
    pub vtable_rva: u32,
    pub field_0x0000: u64,
    pub field_0x0008: u64,
    pub field_0x0088: u64,
}

impl ccGCSpirits {
    pub const CLASS_NAME: &'static str = "ccGCSpirits";
    pub const VTABLE_RVA: u32 = 0x008c0bb8;
    pub const METHOD_COUNT: usize = 10;
    pub const BASE_CLASSES: &'static [&'static str] = &["ccGrtCardBase"];
    pub const VIRTUAL_METHODS: &'static [u64] = &[0x000000018052be40, 0x000000018052be60, 0x000000018052bed0, 0x00000001800d3090, 0x00000001800d3090, 0x000000018052bf90, 0x00000001800d3090, 0x00000001800d3090, 0x00000001800d3090, 0x0000000180529990];

    pub fn new() -> Self {
        Self {
            vtable_rva: Self::VTABLE_RVA,
            field_0x0000: Default::default(),
            field_0x0008: Default::default(),
            field_0x0088: Default::default(),
        }
    }

    /// Decompiled virtual method #0 @ RVA 0x0052be40 (32 instructions, 8 basic blocks)
    pub fn method_00(&mut self) {
    }

    /// Decompiled virtual method #1 @ RVA 0x0052be60 (26 instructions, 7 basic blocks)
    pub fn method_01(&mut self) {
    }

    /// Decompiled virtual method #2 @ RVA 0x0052bed0 (7 instructions, 1 basic blocks)
    pub fn method_02(&mut self) {
    }

    /// Decompiled virtual method #3 @ RVA 0x000d3090 (1 instructions, 1 basic blocks)
    pub fn method_03(&mut self) {
    }

    /// Decompiled virtual method #4 @ RVA 0x000d3090 (1 instructions, 1 basic blocks)
    pub fn method_04(&mut self) {
    }

    /// Decompiled virtual method #5 @ RVA 0x0052bf90 (143 instructions, 14 basic blocks)
    pub fn method_05(&mut self) {
        // Writes to field at offset +0x0088
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

    /// Decompiled virtual method #9 @ RVA 0x00529990 (34 instructions, 5 basic blocks)
    pub fn method_09(&mut self) {
        // Writes to field at offset +0x0000
    }

}

impl CcClass for ccGCSpirits {
    fn class_name(&self) -> &'static str { Self::CLASS_NAME }
    fn vtable_rva(&self) -> u32 { Self::VTABLE_RVA }
    fn virtual_method_count(&self) -> usize { Self::METHOD_COUNT }
    fn base_classes(&self) -> &'static [&'static str] { Self::BASE_CLASSES }
}

/// Decompiled CC2 class: `ccGCThanks` (VTable RVA: 0x008c0a78, 10 virtual methods, 2 recovered fields)
/// Inherits: ccGrtCardBase
#[derive(Debug, Clone, Default)]
pub struct ccGCThanks {
    pub vtable_rva: u32,
    pub field_0x0000: u64,
    pub field_0x0088: u64,
}

impl ccGCThanks {
    pub const CLASS_NAME: &'static str = "ccGCThanks";
    pub const VTABLE_RVA: u32 = 0x008c0a78;
    pub const METHOD_COUNT: usize = 10;
    pub const BASE_CLASSES: &'static [&'static str] = &["ccGrtCardBase"];
    pub const VIRTUAL_METHODS: &'static [u64] = &[0x000000018052c4b0, 0x000000018052c4d0, 0x000000018052c540, 0x00000001800d3090, 0x00000001800d3090, 0x000000018052c600, 0x00000001800d3090, 0x00000001800d3090, 0x00000001800d3090, 0x0000000180529990];

    pub fn new() -> Self {
        Self {
            vtable_rva: Self::VTABLE_RVA,
            field_0x0000: Default::default(),
            field_0x0088: Default::default(),
        }
    }

    /// Decompiled virtual method #0 @ RVA 0x0052c4b0 (32 instructions, 8 basic blocks)
    pub fn method_00(&mut self) {
    }

    /// Decompiled virtual method #1 @ RVA 0x0052c4d0 (26 instructions, 7 basic blocks)
    pub fn method_01(&mut self) {
    }

    /// Decompiled virtual method #2 @ RVA 0x0052c540 (7 instructions, 1 basic blocks)
    pub fn method_02(&mut self) {
    }

    /// Decompiled virtual method #3 @ RVA 0x000d3090 (1 instructions, 1 basic blocks)
    pub fn method_03(&mut self) {
    }

    /// Decompiled virtual method #4 @ RVA 0x000d3090 (1 instructions, 1 basic blocks)
    pub fn method_04(&mut self) {
    }

    /// Decompiled virtual method #5 @ RVA 0x0052c600 (135 instructions, 14 basic blocks)
    pub fn method_05(&mut self) {
        // Writes to field at offset +0x0088
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

    /// Decompiled virtual method #9 @ RVA 0x00529990 (34 instructions, 5 basic blocks)
    pub fn method_09(&mut self) {
        // Writes to field at offset +0x0000
    }

}

impl CcClass for ccGCThanks {
    fn class_name(&self) -> &'static str { Self::CLASS_NAME }
    fn vtable_rva(&self) -> u32 { Self::VTABLE_RVA }
    fn virtual_method_count(&self) -> usize { Self::METHOD_COUNT }
    fn base_classes(&self) -> &'static [&'static str] { Self::BASE_CLASSES }
}

/// Decompiled CC2 class: `ccGMMCtrl` (VTable RVA: 0x00897cb8, 42 virtual methods, 19 recovered fields)
/// Inherits: ccGameMenuCtrl, ccGameMenuCtrlBase, ccGameMenuObj
#[derive(Debug, Clone, Default)]
pub struct ccGMMCtrl {
    pub vtable_rva: u32,
    pub field_0x0000: u64,
    pub field_0x0008: u32,
    pub field_0x0018: u32,
    pub field_0x0048: u64,
    pub field_0x0050: u64,
    pub field_0x0060: u32,
    pub field_0x0068: u64,
    pub field_0x0078: u32,
    pub field_0x0080: u32,
    pub field_0x0090: u64,
    pub field_0x0098: u64,
    pub field_0x009c: u32,
    pub field_0x00a6: u16,
    pub field_0x00a8: u16,
    pub field_0x00ac: u32,
    pub field_0x00b0: u32,
    pub field_0x00fd: u8,
    pub field_0x0100: u32,
    pub field_0x0128: u64,
}

impl ccGMMCtrl {
    pub const CLASS_NAME: &'static str = "ccGMMCtrl";
    pub const VTABLE_RVA: u32 = 0x00897cb8;
    pub const METHOD_COUNT: usize = 42;
    pub const BASE_CLASSES: &'static [&'static str] = &["ccGameMenuCtrl", "ccGameMenuCtrlBase", "ccGameMenuObj"];
    pub const VIRTUAL_METHODS: &'static [u64] = &[0x000000018027da60, 0x00000001804520b0, 0x0000000180452130, 0x0000000180451fb0, 0x0000000180451ff0, 0x0000000180452030, 0x0000000180452070, 0x000000018027da50, 0x0000000180103ea0, 0x0000000180452290, 0x000000018045bb30, 0x0000000180451d90, 0x00000001801e9580, 0x0000000180103e60, 0x0000000180454320, 0x0000000180451dc0, 0x0000000180451e30, 0x0000000180451ea0, 0x0000000180451ee0, 0x0000000180451f40, 0x0000000180455be0, 0x0000000180455d30, 0x0000000180455d40, 0x0000000180103e60, 0x0000000180103e60, 0x0000000180455de0, 0x0000000180103e70, 0x0000000180103e60, 0x00000001804560c0, 0x0000000180456430, 0x0000000180455440, 0x0000000180455950, 0x0000000180455910, 0x00000001804521b0, 0x0000000180452250, 0x0000000180452210, 0x0000000180103e60, 0x00000001804521e0, 0x00000001800d3090, 0x0000000180454a10, 0x0000000180454b00, 0x0000000180454b80];

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
            field_0x0078: Default::default(),
            field_0x0080: Default::default(),
            field_0x0090: Default::default(),
            field_0x0098: Default::default(),
            field_0x009c: Default::default(),
            field_0x00a6: Default::default(),
            field_0x00a8: Default::default(),
            field_0x00ac: Default::default(),
            field_0x00b0: Default::default(),
            field_0x00fd: Default::default(),
            field_0x0100: Default::default(),
            field_0x0128: Default::default(),
        }
    }

    /// Decompiled virtual method #0 @ RVA 0x0027da60 (30 instructions, 3 basic blocks)
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

    /// Decompiled virtual method #7 @ RVA 0x0027da50 (2 instructions, 1 basic blocks)
    pub fn method_07(&mut self) {
    }

    /// Decompiled virtual method #8 @ RVA 0x00103ea0 (9 instructions, 1 basic blocks)
    pub fn method_08(&mut self) {
        // Writes to field at offset +0x0008
    }

    /// Decompiled virtual method #9 @ RVA 0x00452290 (9 instructions, 2 basic blocks)
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

    /// Decompiled virtual method #14 @ RVA 0x00454320 (27 instructions, 7 basic blocks)
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

    /// Decompiled virtual method #20 @ RVA 0x00455be0 (60 instructions, 14 basic blocks)
    pub fn method_20(&mut self) {
        // Writes to field at offset +0x0060
        // Writes to field at offset +0x0078
    }

    /// Decompiled virtual method #21 @ RVA 0x00455d30 (3 instructions, 1 basic blocks)
    pub fn method_21(&mut self) {
        // Writes to field at offset +0x0078
    }

    /// Decompiled virtual method #22 @ RVA 0x00455d40 (36 instructions, 7 basic blocks)
    pub fn method_22(&mut self) {
        // Writes to field at offset +0x0078
        // Writes to field at offset +0x0080
    }

    /// Decompiled virtual method #23 @ RVA 0x00103e60 (2 instructions, 1 basic blocks)
    pub fn method_23(&mut self) {
    }

    /// Decompiled virtual method #24 @ RVA 0x00103e60 (2 instructions, 1 basic blocks)
    pub fn method_24(&mut self) {
    }

    /// Decompiled virtual method #25 @ RVA 0x00455de0 (22 instructions, 1 basic blocks)
    pub fn method_25(&mut self) {
    }

    /// Decompiled virtual method #26 @ RVA 0x00103e70 (2 instructions, 1 basic blocks)
    pub fn method_26(&mut self) {
    }

    /// Decompiled virtual method #27 @ RVA 0x00103e60 (2 instructions, 1 basic blocks)
    pub fn method_27(&mut self) {
    }

    /// Decompiled virtual method #28 @ RVA 0x004560c0 (35 instructions, 10 basic blocks)
    pub fn method_28(&mut self) {
        // Writes to field at offset +0x0018
        // Writes to field at offset +0x00fd
    }

    /// Decompiled virtual method #29 @ RVA 0x00456430 (38 instructions, 10 basic blocks)
    pub fn method_29(&mut self) {
        // Writes to field at offset +0x0018
        // Writes to field at offset +0x00fd
    }

    /// Decompiled virtual method #30 @ RVA 0x00455440 (116 instructions, 23 basic blocks)
    pub fn method_30(&mut self) {
    }

    /// Decompiled virtual method #31 @ RVA 0x00455950 (140 instructions, 29 basic blocks)
    pub fn method_31(&mut self) {
    }

    /// Decompiled virtual method #32 @ RVA 0x00455910 (141 instructions, 29 basic blocks)
    pub fn method_32(&mut self) {
        // Writes to field at offset +0x0100
    }

    /// Decompiled virtual method #33 @ RVA 0x004521b0 (11 instructions, 6 basic blocks)
    pub fn method_33(&mut self) {
    }

    /// Decompiled virtual method #34 @ RVA 0x00452250 (34 instructions, 2 basic blocks)
    pub fn method_34(&mut self) {
    }

    /// Decompiled virtual method #35 @ RVA 0x00452210 (16 instructions, 1 basic blocks)
    pub fn method_35(&mut self) {
    }

    /// Decompiled virtual method #36 @ RVA 0x00103e60 (2 instructions, 1 basic blocks)
    pub fn method_36(&mut self) {
    }

    /// Decompiled virtual method #37 @ RVA 0x004521e0 (11 instructions, 6 basic blocks)
    pub fn method_37(&mut self) {
    }

    /// Decompiled virtual method #38 @ RVA 0x000d3090 (1 instructions, 1 basic blocks)
    pub fn method_38(&mut self) {
    }

    /// Decompiled virtual method #39 @ RVA 0x00454a10 (55 instructions, 4 basic blocks)
    pub fn method_39(&mut self) {
        // Writes to field at offset +0x0000
    }

    /// Decompiled virtual method #40 @ RVA 0x00454b00 (26 instructions, 6 basic blocks)
    pub fn method_40(&mut self) {
    }

    /// Decompiled virtual method #41 @ RVA 0x00454b80 (26 instructions, 6 basic blocks)
    pub fn method_41(&mut self) {
    }

}

impl CcClass for ccGMMCtrl {
    fn class_name(&self) -> &'static str { Self::CLASS_NAME }
    fn vtable_rva(&self) -> u32 { Self::VTABLE_RVA }
    fn virtual_method_count(&self) -> usize { Self::METHOD_COUNT }
    fn base_classes(&self) -> &'static [&'static str] { Self::BASE_CLASSES }
}

/// Decompiled CC2 class: `ccGMMItem` (VTable RVA: 0x00897e10, 13 virtual methods, 19 recovered fields)
/// Inherits: ccGameMenuItem, ccGameMenuObj
#[derive(Debug, Clone, Default)]
pub struct ccGMMItem {
    pub vtable_rva: u32,
    pub field_0x0000: u64,
    pub field_0x0008: u32,
    pub field_0x0010: u32,
    pub field_0x0018: u32,
    pub field_0x001c: u32,
    pub field_0x0020: f32,
    pub field_0x0024: f32,
    pub field_0x0028: u64,
    pub field_0x0030: u32,
    pub field_0x0040: u64,
    pub field_0x0050: f32,
    pub field_0x0060: u32,
    pub field_0x01b8: u8,
    pub field_0x01b9: u8,
    pub field_0x01bc: u32,
    pub field_0x01d0: u32,
    pub field_0x0200: u32,
    pub field_0x0210: u8,
    pub field_0x0214: u32,
}

impl ccGMMItem {
    pub const CLASS_NAME: &'static str = "ccGMMItem";
    pub const VTABLE_RVA: u32 = 0x00897e10;
    pub const METHOD_COUNT: usize = 13;
    pub const BASE_CLASSES: &'static [&'static str] = &["ccGameMenuItem", "ccGameMenuObj"];
    pub const VIRTUAL_METHODS: &'static [u64] = &[0x000000018027d990, 0x0000000180451c60, 0x0000000180453c50, 0x0000000180103e60, 0x0000000180103e60, 0x0000000180103e60, 0x0000000180103e60, 0x00000001800d6880, 0x00000001801035b0, 0x0000000180103e60, 0x0000000180103e60, 0x0000000180453430, 0x0000000180453c10];

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
            field_0x0028: Default::default(),
            field_0x0030: Default::default(),
            field_0x0040: Default::default(),
            field_0x0050: Default::default(),
            field_0x0060: Default::default(),
            field_0x01b8: Default::default(),
            field_0x01b9: Default::default(),
            field_0x01bc: Default::default(),
            field_0x01d0: Default::default(),
            field_0x0200: Default::default(),
            field_0x0210: Default::default(),
            field_0x0214: Default::default(),
        }
    }

    /// Decompiled virtual method #0 @ RVA 0x0027d990 (23 instructions, 3 basic blocks)
    pub fn method_00(&mut self) {
        // Writes to field at offset +0x0000
    }

    /// Decompiled virtual method #1 @ RVA 0x00451c60 (84 instructions, 14 basic blocks)
    pub fn method_01(&mut self) {
        // Writes to field at offset +0x0000
        // Writes to field at offset +0x0010
        // Writes to field at offset +0x0030
    }

    /// Decompiled virtual method #2 @ RVA 0x00453c50 (111 instructions, 23 basic blocks)
    pub fn method_02(&mut self) {
        // Writes to field at offset +0x01b8
        // Writes to field at offset +0x01b8
        // Writes to field at offset +0x01b9
        // Writes to field at offset +0x01bc
        // Writes to field at offset +0x01d0
        // Writes to field at offset +0x0210
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

    /// Decompiled virtual method #7 @ RVA 0x000d6880 (2 instructions, 1 basic blocks)
    pub fn method_07(&mut self) {
    }

    /// Decompiled virtual method #8 @ RVA 0x001035b0 (5 instructions, 1 basic blocks)
    pub fn method_08(&mut self) {
    }

    /// Decompiled virtual method #9 @ RVA 0x00103e60 (2 instructions, 1 basic blocks)
    pub fn method_09(&mut self) {
    }

    /// Decompiled virtual method #10 @ RVA 0x00103e60 (2 instructions, 1 basic blocks)
    pub fn method_10(&mut self) {
    }

    /// Decompiled virtual method #11 @ RVA 0x00453430 (9 instructions, 2 basic blocks)
    pub fn method_11(&mut self) {
        // Writes to field at offset +0x0028
    }

    /// Decompiled virtual method #12 @ RVA 0x00453c10 (121 instructions, 19 basic blocks)
    pub fn method_12(&mut self) {
        // Writes to field at offset +0x01b8
        // Writes to field at offset +0x01b8
        // Writes to field at offset +0x01b9
        // Writes to field at offset +0x01bc
        // Writes to field at offset +0x01d0
        // Writes to field at offset +0x0210
        // Writes to field at offset +0x0214
    }

}

impl CcClass for ccGMMItem {
    fn class_name(&self) -> &'static str { Self::CLASS_NAME }
    fn vtable_rva(&self) -> u32 { Self::VTABLE_RVA }
    fn virtual_method_count(&self) -> usize { Self::METHOD_COUNT }
    fn base_classes(&self) -> &'static [&'static str] { Self::BASE_CLASSES }
}

/// Decompiled CC2 class: `ccGeneralLoadMain` (VTable RVA: 0x00865bf8, 4 virtual methods, 3 recovered fields)
/// Inherits: ccLoadBase
#[derive(Debug, Clone, Default)]
pub struct ccGeneralLoadMain {
    pub vtable_rva: u32,
    pub field_0x0000: u16,
    pub field_0x0014: u32,
    pub field_0x001c: u8,
}

impl ccGeneralLoadMain {
    pub const CLASS_NAME: &'static str = "ccGeneralLoadMain";
    pub const VTABLE_RVA: u32 = 0x00865bf8;
    pub const METHOD_COUNT: usize = 4;
    pub const BASE_CLASSES: &'static [&'static str] = &["ccLoadBase"];
    pub const VIRTUAL_METHODS: &'static [u64] = &[0x000000018055fcc0, 0x000000018055fdd0, 0x000000018055ffc0, 0x00000001805612d0];

    pub fn new() -> Self {
        Self {
            vtable_rva: Self::VTABLE_RVA,
            field_0x0000: Default::default(),
            field_0x0014: Default::default(),
            field_0x001c: Default::default(),
        }
    }

    /// Decompiled virtual method #0 @ RVA 0x0055fcc0 (4 instructions, 2 basic blocks)
    pub fn method_00(&mut self) {
        // Writes to field at offset +0x0014
    }

    /// Decompiled virtual method #1 @ RVA 0x0055fdd0 (6 instructions, 2 basic blocks)
    pub fn method_01(&mut self) {
    }

    /// Decompiled virtual method #2 @ RVA 0x0055ffc0 (110 instructions, 28 basic blocks)
    pub fn method_02(&mut self) {
        // Writes to field at offset +0x0000
    }

    /// Decompiled virtual method #3 @ RVA 0x005612d0 (10 instructions, 2 basic blocks)
    pub fn method_03(&mut self) {
        // Writes to field at offset +0x001c
    }

}

impl CcClass for ccGeneralLoadMain {
    fn class_name(&self) -> &'static str { Self::CLASS_NAME }
    fn vtable_rva(&self) -> u32 { Self::VTABLE_RVA }
    fn virtual_method_count(&self) -> usize { Self::METHOD_COUNT }
    fn base_classes(&self) -> &'static [&'static str] { Self::BASE_CLASSES }
}

/// Decompiled CC2 class: `ccGenerator2` (VTable RVA: 0x00866008, 6 virtual methods, 2 recovered fields)
#[derive(Debug, Clone, Default)]
pub struct ccGenerator2 {
    pub vtable_rva: u32,
    pub field_0x0000: u64,
    pub field_0x0018: u64,
}

impl ccGenerator2 {
    pub const CLASS_NAME: &'static str = "ccGenerator2";
    pub const VTABLE_RVA: u32 = 0x00866008;
    pub const METHOD_COUNT: usize = 6;
    pub const BASE_CLASSES: &'static [&'static str] = &[];
    pub const VIRTUAL_METHODS: &'static [u64] = &[0x00000001800eace0, 0x00000001800e4c40, 0x00000001800d3090, 0x00000001800d3090, 0x00000001800d3090, 0x00000001800d3090];

    pub fn new() -> Self {
        Self {
            vtable_rva: Self::VTABLE_RVA,
            field_0x0000: Default::default(),
            field_0x0018: Default::default(),
        }
    }

    /// Decompiled virtual method #0 @ RVA 0x000eace0 (13 instructions, 3 basic blocks)
    pub fn method_00(&mut self) {
        // Writes to field at offset +0x0000
    }

    /// Decompiled virtual method #1 @ RVA 0x000e4c40 (27 instructions, 4 basic blocks)
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

    /// Decompiled virtual method #5 @ RVA 0x000d3090 (1 instructions, 1 basic blocks)
    pub fn method_05(&mut self) {
    }

}

impl CcClass for ccGenerator2 {
    fn class_name(&self) -> &'static str { Self::CLASS_NAME }
    fn vtable_rva(&self) -> u32 { Self::VTABLE_RVA }
    fn virtual_method_count(&self) -> usize { Self::METHOD_COUNT }
    fn base_classes(&self) -> &'static [&'static str] { Self::BASE_CLASSES }
}

/// Decompiled CC2 class: `ccGoldPanel` (VTable RVA: 0x008ba388, 1 virtual methods, 0 recovered fields)
#[derive(Debug, Clone, Default)]
pub struct ccGoldPanel {
    pub vtable_rva: u32,
}

impl ccGoldPanel {
    pub const CLASS_NAME: &'static str = "ccGoldPanel";
    pub const VTABLE_RVA: u32 = 0x008ba388;
    pub const METHOD_COUNT: usize = 1;
    pub const BASE_CLASSES: &'static [&'static str] = &[];
    pub const VIRTUAL_METHODS: &'static [u64] = &[0x0000000180452c90];

    pub fn new() -> Self {
        Self {
            vtable_rva: Self::VTABLE_RVA,
        }
    }

    /// Decompiled virtual method #0 @ RVA 0x00452c90 (16 instructions, 3 basic blocks)
    pub fn method_00(&mut self) {
    }

}

impl CcClass for ccGoldPanel {
    fn class_name(&self) -> &'static str { Self::CLASS_NAME }
    fn vtable_rva(&self) -> u32 { Self::VTABLE_RVA }
    fn virtual_method_count(&self) -> usize { Self::METHOD_COUNT }
    fn base_classes(&self) -> &'static [&'static str] { Self::BASE_CLASSES }
}

/// Decompiled CC2 class: `ccGrtAnmBase` (VTable RVA: 0x008c0da0, 8 virtual methods, 1 recovered fields)
#[derive(Debug, Clone, Default)]
pub struct ccGrtAnmBase {
    pub vtable_rva: u32,
    pub field_0x0000: u64,
}

impl ccGrtAnmBase {
    pub const CLASS_NAME: &'static str = "ccGrtAnmBase";
    pub const VTABLE_RVA: u32 = 0x008c0da0;
    pub const METHOD_COUNT: usize = 8;
    pub const BASE_CLASSES: &'static [&'static str] = &[];
    pub const VIRTUAL_METHODS: &'static [u64] = &[0x00000001800d3090, 0x00000001800d3090, 0x00000001800d3090, 0x00000001800d3090, 0x00000001800d3090, 0x00000001800d3090, 0x0000000180529940, 0x000000018052a4f0];

    pub fn new() -> Self {
        Self {
            vtable_rva: Self::VTABLE_RVA,
            field_0x0000: Default::default(),
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

    /// Decompiled virtual method #4 @ RVA 0x000d3090 (1 instructions, 1 basic blocks)
    pub fn method_04(&mut self) {
    }

    /// Decompiled virtual method #5 @ RVA 0x000d3090 (1 instructions, 1 basic blocks)
    pub fn method_05(&mut self) {
    }

    /// Decompiled virtual method #6 @ RVA 0x00529940 (21 instructions, 3 basic blocks)
    pub fn method_06(&mut self) {
        // Writes to field at offset +0x0000
    }

    /// Decompiled virtual method #7 @ RVA 0x0052a4f0 (27 instructions, 6 basic blocks)
    pub fn method_07(&mut self) {
    }

}

impl CcClass for ccGrtAnmBase {
    fn class_name(&self) -> &'static str { Self::CLASS_NAME }
    fn vtable_rva(&self) -> u32 { Self::VTABLE_RVA }
    fn virtual_method_count(&self) -> usize { Self::METHOD_COUNT }
    fn base_classes(&self) -> &'static [&'static str] { Self::BASE_CLASSES }
}

/// Decompiled CC2 class: `ccGrtCardBase` (VTable RVA: 0x008c0de8, 10 virtual methods, 1 recovered fields)
#[derive(Debug, Clone, Default)]
pub struct ccGrtCardBase {
    pub vtable_rva: u32,
    pub field_0x0000: u64,
}

impl ccGrtCardBase {
    pub const CLASS_NAME: &'static str = "ccGrtCardBase";
    pub const VTABLE_RVA: u32 = 0x008c0de8;
    pub const METHOD_COUNT: usize = 10;
    pub const BASE_CLASSES: &'static [&'static str] = &[];
    pub const VIRTUAL_METHODS: &'static [u64] = &[0x00000001800d3090, 0x00000001800d3090, 0x00000001800fe200, 0x00000001800d3090, 0x00000001800d3090, 0x00000001800d3090, 0x00000001800d3090, 0x00000001800d3090, 0x00000001800d3090, 0x0000000180529990];

    pub fn new() -> Self {
        Self {
            vtable_rva: Self::VTABLE_RVA,
            field_0x0000: Default::default(),
        }
    }

    /// Decompiled virtual method #0 @ RVA 0x000d3090 (1 instructions, 1 basic blocks)
    pub fn method_00(&mut self) {
    }

    /// Decompiled virtual method #1 @ RVA 0x000d3090 (1 instructions, 1 basic blocks)
    pub fn method_01(&mut self) {
    }

    /// Decompiled virtual method #2 @ RVA 0x000fe200 (2 instructions, 1 basic blocks)
    pub fn method_02(&mut self) {
    }

    /// Decompiled virtual method #3 @ RVA 0x000d3090 (1 instructions, 1 basic blocks)
    pub fn method_03(&mut self) {
    }

    /// Decompiled virtual method #4 @ RVA 0x000d3090 (1 instructions, 1 basic blocks)
    pub fn method_04(&mut self) {
    }

    /// Decompiled virtual method #5 @ RVA 0x000d3090 (1 instructions, 1 basic blocks)
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

    /// Decompiled virtual method #9 @ RVA 0x00529990 (34 instructions, 5 basic blocks)
    pub fn method_09(&mut self) {
        // Writes to field at offset +0x0000
    }

}

impl CcClass for ccGrtCardBase {
    fn class_name(&self) -> &'static str { Self::CLASS_NAME }
    fn vtable_rva(&self) -> u32 { Self::VTABLE_RVA }
    fn virtual_method_count(&self) -> usize { Self::METHOD_COUNT }
    fn base_classes(&self) -> &'static [&'static str] { Self::BASE_CLASSES }
}

/// Decompiled CC2 class: `ccGrtMan` (VTable RVA: 0x008c0e40, 1 virtual methods, 1 recovered fields)
#[derive(Debug, Clone, Default)]
pub struct ccGrtMan {
    pub vtable_rva: u32,
    pub field_0x0000: u64,
}

impl ccGrtMan {
    pub const CLASS_NAME: &'static str = "ccGrtMan";
    pub const VTABLE_RVA: u32 = 0x008c0e40;
    pub const METHOD_COUNT: usize = 1;
    pub const BASE_CLASSES: &'static [&'static str] = &[];
    pub const VIRTUAL_METHODS: &'static [u64] = &[0x0000000180529a10];

    pub fn new() -> Self {
        Self {
            vtable_rva: Self::VTABLE_RVA,
            field_0x0000: Default::default(),
        }
    }

    /// Decompiled virtual method #0 @ RVA 0x00529a10 (34 instructions, 5 basic blocks)
    pub fn method_00(&mut self) {
        // Writes to field at offset +0x0000
    }

}

impl CcClass for ccGrtMan {
    fn class_name(&self) -> &'static str { Self::CLASS_NAME }
    fn vtable_rva(&self) -> u32 { Self::VTABLE_RVA }
    fn virtual_method_count(&self) -> usize { Self::METHOD_COUNT }
    fn base_classes(&self) -> &'static [&'static str] { Self::BASE_CLASSES }
}

/// Decompiled CC2 class: `ccHtmlATag` (VTable RVA: 0x008c3468, 14 virtual methods, 6 recovered fields)
/// Inherits: ccHtmlTagBase
#[derive(Debug, Clone, Default)]
pub struct ccHtmlATag {
    pub vtable_rva: u32,
    pub field_0x0000: u64,
    pub field_0x0001: u32,
    pub field_0x0002: u8,
    pub field_0x0008: u16,
    pub field_0x002e: u16,
    pub field_0x0032: u16,
}

impl ccHtmlATag {
    pub const CLASS_NAME: &'static str = "ccHtmlATag";
    pub const VTABLE_RVA: u32 = 0x008c3468;
    pub const METHOD_COUNT: usize = 14;
    pub const BASE_CLASSES: &'static [&'static str] = &["ccHtmlTagBase"];
    pub const VIRTUAL_METHODS: &'static [u64] = &[0x000000018053b3e0, 0x0000000180103e70, 0x0000000180103e70, 0x00000001805411a0, 0x0000000180103e70, 0x0000000180103e70, 0x00000001800d6920, 0x00000001800d6920, 0x00000001800d6920, 0x00000001800fe200, 0x00000001805421e0, 0x0000000180541ff0, 0x0000000180541fb0, 0x0000000180103e80];

    pub fn new() -> Self {
        Self {
            vtable_rva: Self::VTABLE_RVA,
            field_0x0000: Default::default(),
            field_0x0001: Default::default(),
            field_0x0002: Default::default(),
            field_0x0008: Default::default(),
            field_0x002e: Default::default(),
            field_0x0032: Default::default(),
        }
    }

    /// Decompiled virtual method #0 @ RVA 0x0053b3e0 (20 instructions, 3 basic blocks)
    pub fn method_00(&mut self) {
        // Writes to field at offset +0x0000
    }

    /// Decompiled virtual method #1 @ RVA 0x00103e70 (2 instructions, 1 basic blocks)
    pub fn method_01(&mut self) {
    }

    /// Decompiled virtual method #2 @ RVA 0x00103e70 (2 instructions, 1 basic blocks)
    pub fn method_02(&mut self) {
    }

    /// Decompiled virtual method #3 @ RVA 0x005411a0 (139 instructions, 34 basic blocks)
    pub fn method_03(&mut self) {
        // Writes to field at offset +0x0000
    }

    /// Decompiled virtual method #4 @ RVA 0x00103e70 (2 instructions, 1 basic blocks)
    pub fn method_04(&mut self) {
    }

    /// Decompiled virtual method #5 @ RVA 0x00103e70 (2 instructions, 1 basic blocks)
    pub fn method_05(&mut self) {
    }

    /// Decompiled virtual method #6 @ RVA 0x000d6920 (2 instructions, 1 basic blocks)
    pub fn method_06(&mut self) {
    }

    /// Decompiled virtual method #7 @ RVA 0x000d6920 (2 instructions, 1 basic blocks)
    pub fn method_07(&mut self) {
    }

    /// Decompiled virtual method #8 @ RVA 0x000d6920 (2 instructions, 1 basic blocks)
    pub fn method_08(&mut self) {
    }

    /// Decompiled virtual method #9 @ RVA 0x000fe200 (2 instructions, 1 basic blocks)
    pub fn method_09(&mut self) {
    }

    /// Decompiled virtual method #10 @ RVA 0x005421e0 (142 instructions, 38 basic blocks)
    pub fn method_10(&mut self) {
    }

    /// Decompiled virtual method #11 @ RVA 0x00541ff0 (136 instructions, 37 basic blocks)
    pub fn method_11(&mut self) {
        // Writes to field at offset +0x0000
        // Writes to field at offset +0x0001
        // Writes to field at offset +0x0002
    }

    /// Decompiled virtual method #12 @ RVA 0x00541fb0 (11 instructions, 5 basic blocks)
    pub fn method_12(&mut self) {
    }

    /// Decompiled virtual method #13 @ RVA 0x00103e80 (2 instructions, 1 basic blocks)
    pub fn method_13(&mut self) {
    }

}

impl CcClass for ccHtmlATag {
    fn class_name(&self) -> &'static str { Self::CLASS_NAME }
    fn vtable_rva(&self) -> u32 { Self::VTABLE_RVA }
    fn virtual_method_count(&self) -> usize { Self::METHOD_COUNT }
    fn base_classes(&self) -> &'static [&'static str] { Self::BASE_CLASSES }
}

/// Decompiled CC2 class: `ccHtmlAddTag` (VTable RVA: 0x008c2eb0, 14 virtual methods, 4 recovered fields)
/// Inherits: ccHtmlTagBase
#[derive(Debug, Clone, Default)]
pub struct ccHtmlAddTag {
    pub vtable_rva: u32,
    pub field_0x0000: u64,
    pub field_0x0008: u16,
    pub field_0x0028: u32,
    pub field_0x002c: u8,
}

impl ccHtmlAddTag {
    pub const CLASS_NAME: &'static str = "ccHtmlAddTag";
    pub const VTABLE_RVA: u32 = 0x008c2eb0;
    pub const METHOD_COUNT: usize = 14;
    pub const BASE_CLASSES: &'static [&'static str] = &["ccHtmlTagBase"];
    pub const VIRTUAL_METHODS: &'static [u64] = &[0x000000018053b320, 0x0000000180103e70, 0x0000000180103e70, 0x0000000180547f70, 0x0000000180103e70, 0x0000000180103e70, 0x00000001800d6920, 0x00000001800d6920, 0x00000001800d6920, 0x00000001800fe200, 0x0000000180548280, 0x0000000180103e70, 0x00000001805483a0, 0x0000000180103e80];

    pub fn new() -> Self {
        Self {
            vtable_rva: Self::VTABLE_RVA,
            field_0x0000: Default::default(),
            field_0x0008: Default::default(),
            field_0x0028: Default::default(),
            field_0x002c: Default::default(),
        }
    }

    /// Decompiled virtual method #0 @ RVA 0x0053b320 (13 instructions, 3 basic blocks)
    pub fn method_00(&mut self) {
        // Writes to field at offset +0x0000
    }

    /// Decompiled virtual method #1 @ RVA 0x00103e70 (2 instructions, 1 basic blocks)
    pub fn method_01(&mut self) {
    }

    /// Decompiled virtual method #2 @ RVA 0x00103e70 (2 instructions, 1 basic blocks)
    pub fn method_02(&mut self) {
    }

    /// Decompiled virtual method #3 @ RVA 0x00547f70 (17 instructions, 3 basic blocks)
    pub fn method_03(&mut self) {
    }

    /// Decompiled virtual method #4 @ RVA 0x00103e70 (2 instructions, 1 basic blocks)
    pub fn method_04(&mut self) {
    }

    /// Decompiled virtual method #5 @ RVA 0x00103e70 (2 instructions, 1 basic blocks)
    pub fn method_05(&mut self) {
    }

    /// Decompiled virtual method #6 @ RVA 0x000d6920 (2 instructions, 1 basic blocks)
    pub fn method_06(&mut self) {
    }

    /// Decompiled virtual method #7 @ RVA 0x000d6920 (2 instructions, 1 basic blocks)
    pub fn method_07(&mut self) {
    }

    /// Decompiled virtual method #8 @ RVA 0x000d6920 (2 instructions, 1 basic blocks)
    pub fn method_08(&mut self) {
    }

    /// Decompiled virtual method #9 @ RVA 0x000fe200 (2 instructions, 1 basic blocks)
    pub fn method_09(&mut self) {
    }

    /// Decompiled virtual method #10 @ RVA 0x00548280 (78 instructions, 16 basic blocks)
    pub fn method_10(&mut self) {
        // Writes to field at offset +0x0028
        // Writes to field at offset +0x002c
    }

    /// Decompiled virtual method #11 @ RVA 0x00103e70 (2 instructions, 1 basic blocks)
    pub fn method_11(&mut self) {
    }

    /// Decompiled virtual method #12 @ RVA 0x005483a0 (9 instructions, 3 basic blocks)
    pub fn method_12(&mut self) {
        // Writes to field at offset +0x002c
    }

    /// Decompiled virtual method #13 @ RVA 0x00103e80 (2 instructions, 1 basic blocks)
    pub fn method_13(&mut self) {
    }

}

impl CcClass for ccHtmlAddTag {
    fn class_name(&self) -> &'static str { Self::CLASS_NAME }
    fn vtable_rva(&self) -> u32 { Self::VTABLE_RVA }
    fn virtual_method_count(&self) -> usize { Self::METHOD_COUNT }
    fn base_classes(&self) -> &'static [&'static str] { Self::BASE_CLASSES }
}

/// Decompiled CC2 class: `ccHtmlAutoParamTag` (VTable RVA: 0x008c3300, 14 virtual methods, 1 recovered fields)
/// Inherits: ccHtmlTagBase
#[derive(Debug, Clone, Default)]
pub struct ccHtmlAutoParamTag {
    pub vtable_rva: u32,
    pub field_0x0000: u64,
}

impl ccHtmlAutoParamTag {
    pub const CLASS_NAME: &'static str = "ccHtmlAutoParamTag";
    pub const VTABLE_RVA: u32 = 0x008c3300;
    pub const METHOD_COUNT: usize = 14;
    pub const BASE_CLASSES: &'static [&'static str] = &["ccHtmlTagBase"];
    pub const VIRTUAL_METHODS: &'static [u64] = &[0x000000018053b220, 0x0000000180103e70, 0x0000000180103e70, 0x0000000180103e70, 0x0000000180103e70, 0x0000000180103e70, 0x00000001800d6920, 0x00000001800d6920, 0x00000001800d6920, 0x00000001800fe200, 0x00000001800fe200, 0x0000000180103e80, 0x0000000180103e80, 0x0000000180103e80];

    pub fn new() -> Self {
        Self {
            vtable_rva: Self::VTABLE_RVA,
            field_0x0000: Default::default(),
        }
    }

    /// Decompiled virtual method #0 @ RVA 0x0053b220 (13 instructions, 3 basic blocks)
    pub fn method_00(&mut self) {
        // Writes to field at offset +0x0000
    }

    /// Decompiled virtual method #1 @ RVA 0x00103e70 (2 instructions, 1 basic blocks)
    pub fn method_01(&mut self) {
    }

    /// Decompiled virtual method #2 @ RVA 0x00103e70 (2 instructions, 1 basic blocks)
    pub fn method_02(&mut self) {
    }

    /// Decompiled virtual method #3 @ RVA 0x00103e70 (2 instructions, 1 basic blocks)
    pub fn method_03(&mut self) {
    }

    /// Decompiled virtual method #4 @ RVA 0x00103e70 (2 instructions, 1 basic blocks)
    pub fn method_04(&mut self) {
    }

    /// Decompiled virtual method #5 @ RVA 0x00103e70 (2 instructions, 1 basic blocks)
    pub fn method_05(&mut self) {
    }

    /// Decompiled virtual method #6 @ RVA 0x000d6920 (2 instructions, 1 basic blocks)
    pub fn method_06(&mut self) {
    }

    /// Decompiled virtual method #7 @ RVA 0x000d6920 (2 instructions, 1 basic blocks)
    pub fn method_07(&mut self) {
    }

    /// Decompiled virtual method #8 @ RVA 0x000d6920 (2 instructions, 1 basic blocks)
    pub fn method_08(&mut self) {
    }

    /// Decompiled virtual method #9 @ RVA 0x000fe200 (2 instructions, 1 basic blocks)
    pub fn method_09(&mut self) {
    }

    /// Decompiled virtual method #10 @ RVA 0x000fe200 (2 instructions, 1 basic blocks)
    pub fn method_10(&mut self) {
    }

    /// Decompiled virtual method #11 @ RVA 0x00103e80 (2 instructions, 1 basic blocks)
    pub fn method_11(&mut self) {
    }

    /// Decompiled virtual method #12 @ RVA 0x00103e80 (2 instructions, 1 basic blocks)
    pub fn method_12(&mut self) {
    }

    /// Decompiled virtual method #13 @ RVA 0x00103e80 (2 instructions, 1 basic blocks)
    pub fn method_13(&mut self) {
    }

}

impl CcClass for ccHtmlAutoParamTag {
    fn class_name(&self) -> &'static str { Self::CLASS_NAME }
    fn vtable_rva(&self) -> u32 { Self::VTABLE_RVA }
    fn virtual_method_count(&self) -> usize { Self::METHOD_COUNT }
    fn base_classes(&self) -> &'static [&'static str] { Self::BASE_CLASSES }
}

/// Decompiled CC2 class: `ccHtmlBrTag` (VTable RVA: 0x008c38e0, 14 virtual methods, 2 recovered fields)
/// Inherits: ccHtmlTagBase
#[derive(Debug, Clone, Default)]
pub struct ccHtmlBrTag {
    pub vtable_rva: u32,
    pub field_0x0000: u64,
    pub field_0x0008: u16,
}

impl ccHtmlBrTag {
    pub const CLASS_NAME: &'static str = "ccHtmlBrTag";
    pub const VTABLE_RVA: u32 = 0x008c38e0;
    pub const METHOD_COUNT: usize = 14;
    pub const BASE_CLASSES: &'static [&'static str] = &["ccHtmlTagBase"];
    pub const VIRTUAL_METHODS: &'static [u64] = &[0x000000018053b220, 0x0000000180103e70, 0x0000000180103e70, 0x0000000180542450, 0x0000000180103e70, 0x0000000180103e70, 0x00000001800d6920, 0x00000001800d6920, 0x00000001800d6920, 0x00000001800fe200, 0x0000000180542490, 0x0000000180103e80, 0x0000000180103e80, 0x0000000180103e80];

    pub fn new() -> Self {
        Self {
            vtable_rva: Self::VTABLE_RVA,
            field_0x0000: Default::default(),
            field_0x0008: Default::default(),
        }
    }

    /// Decompiled virtual method #0 @ RVA 0x0053b220 (13 instructions, 3 basic blocks)
    pub fn method_00(&mut self) {
        // Writes to field at offset +0x0000
    }

    /// Decompiled virtual method #1 @ RVA 0x00103e70 (2 instructions, 1 basic blocks)
    pub fn method_01(&mut self) {
    }

    /// Decompiled virtual method #2 @ RVA 0x00103e70 (2 instructions, 1 basic blocks)
    pub fn method_02(&mut self) {
    }

    /// Decompiled virtual method #3 @ RVA 0x00542450 (15 instructions, 1 basic blocks)
    pub fn method_03(&mut self) {
    }

    /// Decompiled virtual method #4 @ RVA 0x00103e70 (2 instructions, 1 basic blocks)
    pub fn method_04(&mut self) {
    }

    /// Decompiled virtual method #5 @ RVA 0x00103e70 (2 instructions, 1 basic blocks)
    pub fn method_05(&mut self) {
    }

    /// Decompiled virtual method #6 @ RVA 0x000d6920 (2 instructions, 1 basic blocks)
    pub fn method_06(&mut self) {
    }

    /// Decompiled virtual method #7 @ RVA 0x000d6920 (2 instructions, 1 basic blocks)
    pub fn method_07(&mut self) {
    }

    /// Decompiled virtual method #8 @ RVA 0x000d6920 (2 instructions, 1 basic blocks)
    pub fn method_08(&mut self) {
    }

    /// Decompiled virtual method #9 @ RVA 0x000fe200 (2 instructions, 1 basic blocks)
    pub fn method_09(&mut self) {
    }

    /// Decompiled virtual method #10 @ RVA 0x00542490 (26 instructions, 4 basic blocks)
    pub fn method_10(&mut self) {
    }

    /// Decompiled virtual method #11 @ RVA 0x00103e80 (2 instructions, 1 basic blocks)
    pub fn method_11(&mut self) {
    }

    /// Decompiled virtual method #12 @ RVA 0x00103e80 (2 instructions, 1 basic blocks)
    pub fn method_12(&mut self) {
    }

    /// Decompiled virtual method #13 @ RVA 0x00103e80 (2 instructions, 1 basic blocks)
    pub fn method_13(&mut self) {
    }

}

impl CcClass for ccHtmlBrTag {
    fn class_name(&self) -> &'static str { Self::CLASS_NAME }
    fn vtable_rva(&self) -> u32 { Self::VTABLE_RVA }
    fn virtual_method_count(&self) -> usize { Self::METHOD_COUNT }
    fn base_classes(&self) -> &'static [&'static str] { Self::BASE_CLASSES }
}

/// Decompiled CC2 class: `ccHtmlClumpTag` (VTable RVA: 0x008c3650, 15 virtual methods, 14 recovered fields)
/// Inherits: ccHtmlNestTag, ccHtmlTagBase
#[derive(Debug, Clone, Default)]
pub struct ccHtmlClumpTag {
    pub vtable_rva: u32,
    pub field_0x0000: u64,
    pub field_0x0001: u32,
    pub field_0x0008: u16,
    pub field_0x0010: u64,
    pub field_0x0028: u64,
    pub field_0x0030: u32,
    pub field_0x0034: u32,
    pub field_0x0038: u32,
    pub field_0x003c: u32,
    pub field_0x0040: u32,
    pub field_0x0044: u32,
    pub field_0x0048: u32,
    pub field_0x004c: u32,
    pub field_0x0070: u8,
}

impl ccHtmlClumpTag {
    pub const CLASS_NAME: &'static str = "ccHtmlClumpTag";
    pub const VTABLE_RVA: u32 = 0x008c3650;
    pub const METHOD_COUNT: usize = 15;
    pub const BASE_CLASSES: &'static [&'static str] = &["ccHtmlNestTag", "ccHtmlTagBase"];
    pub const VIRTUAL_METHODS: &'static [u64] = &[0x000000018053b570, 0x0000000180103e70, 0x0000000180103e70, 0x0000000180546800, 0x0000000180103e70, 0x0000000180103e70, 0x00000001800d6920, 0x00000001800fe200, 0x00000001800d6920, 0x00000001800fe200, 0x0000000180546bf0, 0x0000000180546aa0, 0x00000001805469e0, 0x0000000180103e80, 0x0000000180545ad0];

    pub fn new() -> Self {
        Self {
            vtable_rva: Self::VTABLE_RVA,
            field_0x0000: Default::default(),
            field_0x0001: Default::default(),
            field_0x0008: Default::default(),
            field_0x0010: Default::default(),
            field_0x0028: Default::default(),
            field_0x0030: Default::default(),
            field_0x0034: Default::default(),
            field_0x0038: Default::default(),
            field_0x003c: Default::default(),
            field_0x0040: Default::default(),
            field_0x0044: Default::default(),
            field_0x0048: Default::default(),
            field_0x004c: Default::default(),
            field_0x0070: Default::default(),
        }
    }

    /// Decompiled virtual method #0 @ RVA 0x0053b570 (28 instructions, 3 basic blocks)
    pub fn method_00(&mut self) {
        // Writes to field at offset +0x0000
    }

    /// Decompiled virtual method #1 @ RVA 0x00103e70 (2 instructions, 1 basic blocks)
    pub fn method_01(&mut self) {
    }

    /// Decompiled virtual method #2 @ RVA 0x00103e70 (2 instructions, 1 basic blocks)
    pub fn method_02(&mut self) {
    }

    /// Decompiled virtual method #3 @ RVA 0x00546800 (114 instructions, 13 basic blocks)
    pub fn method_03(&mut self) {
    }

    /// Decompiled virtual method #4 @ RVA 0x00103e70 (2 instructions, 1 basic blocks)
    pub fn method_04(&mut self) {
    }

    /// Decompiled virtual method #5 @ RVA 0x00103e70 (2 instructions, 1 basic blocks)
    pub fn method_05(&mut self) {
    }

    /// Decompiled virtual method #6 @ RVA 0x000d6920 (2 instructions, 1 basic blocks)
    pub fn method_06(&mut self) {
    }

    /// Decompiled virtual method #7 @ RVA 0x000fe200 (2 instructions, 1 basic blocks)
    pub fn method_07(&mut self) {
    }

    /// Decompiled virtual method #8 @ RVA 0x000d6920 (2 instructions, 1 basic blocks)
    pub fn method_08(&mut self) {
    }

    /// Decompiled virtual method #9 @ RVA 0x000fe200 (2 instructions, 1 basic blocks)
    pub fn method_09(&mut self) {
    }

    /// Decompiled virtual method #10 @ RVA 0x00546bf0 (135 instructions, 39 basic blocks)
    pub fn method_10(&mut self) {
    }

    /// Decompiled virtual method #11 @ RVA 0x00546aa0 (84 instructions, 15 basic blocks)
    pub fn method_11(&mut self) {
        // Writes to field at offset +0x0000
        // Writes to field at offset +0x0001
        // Writes to field at offset +0x0048
    }

    /// Decompiled virtual method #12 @ RVA 0x005469e0 (38 instructions, 12 basic blocks)
    pub fn method_12(&mut self) {
        // Writes to field at offset +0x0030
        // Writes to field at offset +0x0034
        // Writes to field at offset +0x0038
        // Writes to field at offset +0x003c
        // Writes to field at offset +0x0040
        // Writes to field at offset +0x0044
        // Writes to field at offset +0x004c
        // Writes to field at offset +0x0070
    }

    /// Decompiled virtual method #13 @ RVA 0x00103e80 (2 instructions, 1 basic blocks)
    pub fn method_13(&mut self) {
    }

    /// Decompiled virtual method #14 @ RVA 0x00545ad0 (40 instructions, 10 basic blocks)
    pub fn method_14(&mut self) {
        // Writes to field at offset +0x0000
        // Writes to field at offset +0x0028
    }

}

impl CcClass for ccHtmlClumpTag {
    fn class_name(&self) -> &'static str { Self::CLASS_NAME }
    fn vtable_rva(&self) -> u32 { Self::VTABLE_RVA }
    fn virtual_method_count(&self) -> usize { Self::METHOD_COUNT }
    fn base_classes(&self) -> &'static [&'static str] { Self::BASE_CLASSES }
}

/// Decompiled CC2 class: `ccHtmlColTag` (VTable RVA: 0x008c3120, 14 virtual methods, 1 recovered fields)
/// Inherits: ccHtmlTagBase
#[derive(Debug, Clone, Default)]
pub struct ccHtmlColTag {
    pub vtable_rva: u32,
    pub field_0x0000: u64,
}

impl ccHtmlColTag {
    pub const CLASS_NAME: &'static str = "ccHtmlColTag";
    pub const VTABLE_RVA: u32 = 0x008c3120;
    pub const METHOD_COUNT: usize = 14;
    pub const BASE_CLASSES: &'static [&'static str] = &["ccHtmlTagBase"];
    pub const VIRTUAL_METHODS: &'static [u64] = &[0x000000018053b220, 0x0000000180103e70, 0x0000000180103e70, 0x0000000180103e70, 0x0000000180103e70, 0x0000000180103e70, 0x00000001800d6920, 0x00000001800d6920, 0x00000001800d6920, 0x00000001800fe200, 0x00000001800fe200, 0x0000000180103e80, 0x0000000180103e80, 0x0000000180103e80];

    pub fn new() -> Self {
        Self {
            vtable_rva: Self::VTABLE_RVA,
            field_0x0000: Default::default(),
        }
    }

    /// Decompiled virtual method #0 @ RVA 0x0053b220 (13 instructions, 3 basic blocks)
    pub fn method_00(&mut self) {
        // Writes to field at offset +0x0000
    }

    /// Decompiled virtual method #1 @ RVA 0x00103e70 (2 instructions, 1 basic blocks)
    pub fn method_01(&mut self) {
    }

    /// Decompiled virtual method #2 @ RVA 0x00103e70 (2 instructions, 1 basic blocks)
    pub fn method_02(&mut self) {
    }

    /// Decompiled virtual method #3 @ RVA 0x00103e70 (2 instructions, 1 basic blocks)
    pub fn method_03(&mut self) {
    }

    /// Decompiled virtual method #4 @ RVA 0x00103e70 (2 instructions, 1 basic blocks)
    pub fn method_04(&mut self) {
    }

    /// Decompiled virtual method #5 @ RVA 0x00103e70 (2 instructions, 1 basic blocks)
    pub fn method_05(&mut self) {
    }

    /// Decompiled virtual method #6 @ RVA 0x000d6920 (2 instructions, 1 basic blocks)
    pub fn method_06(&mut self) {
    }

    /// Decompiled virtual method #7 @ RVA 0x000d6920 (2 instructions, 1 basic blocks)
    pub fn method_07(&mut self) {
    }

    /// Decompiled virtual method #8 @ RVA 0x000d6920 (2 instructions, 1 basic blocks)
    pub fn method_08(&mut self) {
    }

    /// Decompiled virtual method #9 @ RVA 0x000fe200 (2 instructions, 1 basic blocks)
    pub fn method_09(&mut self) {
    }

    /// Decompiled virtual method #10 @ RVA 0x000fe200 (2 instructions, 1 basic blocks)
    pub fn method_10(&mut self) {
    }

    /// Decompiled virtual method #11 @ RVA 0x00103e80 (2 instructions, 1 basic blocks)
    pub fn method_11(&mut self) {
    }

    /// Decompiled virtual method #12 @ RVA 0x00103e80 (2 instructions, 1 basic blocks)
    pub fn method_12(&mut self) {
    }

    /// Decompiled virtual method #13 @ RVA 0x00103e80 (2 instructions, 1 basic blocks)
    pub fn method_13(&mut self) {
    }

}

impl CcClass for ccHtmlColTag {
    fn class_name(&self) -> &'static str { Self::CLASS_NAME }
    fn vtable_rva(&self) -> u32 { Self::VTABLE_RVA }
    fn virtual_method_count(&self) -> usize { Self::METHOD_COUNT }
    fn base_classes(&self) -> &'static [&'static str] { Self::BASE_CLASSES }
}

/// Decompiled CC2 class: `ccHtmlColgroupTag` (VTable RVA: 0x008c3198, 14 virtual methods, 1 recovered fields)
/// Inherits: ccHtmlTagBase
#[derive(Debug, Clone, Default)]
pub struct ccHtmlColgroupTag {
    pub vtable_rva: u32,
    pub field_0x0000: u64,
}

impl ccHtmlColgroupTag {
    pub const CLASS_NAME: &'static str = "ccHtmlColgroupTag";
    pub const VTABLE_RVA: u32 = 0x008c3198;
    pub const METHOD_COUNT: usize = 14;
    pub const BASE_CLASSES: &'static [&'static str] = &["ccHtmlTagBase"];
    pub const VIRTUAL_METHODS: &'static [u64] = &[0x000000018053b220, 0x0000000180103e70, 0x0000000180103e70, 0x0000000180103e70, 0x0000000180103e70, 0x0000000180103e70, 0x00000001800d6920, 0x00000001800d6920, 0x00000001800d6920, 0x00000001800fe200, 0x00000001800fe200, 0x0000000180103e80, 0x0000000180103e80, 0x0000000180103e80];

    pub fn new() -> Self {
        Self {
            vtable_rva: Self::VTABLE_RVA,
            field_0x0000: Default::default(),
        }
    }

    /// Decompiled virtual method #0 @ RVA 0x0053b220 (13 instructions, 3 basic blocks)
    pub fn method_00(&mut self) {
        // Writes to field at offset +0x0000
    }

    /// Decompiled virtual method #1 @ RVA 0x00103e70 (2 instructions, 1 basic blocks)
    pub fn method_01(&mut self) {
    }

    /// Decompiled virtual method #2 @ RVA 0x00103e70 (2 instructions, 1 basic blocks)
    pub fn method_02(&mut self) {
    }

    /// Decompiled virtual method #3 @ RVA 0x00103e70 (2 instructions, 1 basic blocks)
    pub fn method_03(&mut self) {
    }

    /// Decompiled virtual method #4 @ RVA 0x00103e70 (2 instructions, 1 basic blocks)
    pub fn method_04(&mut self) {
    }

    /// Decompiled virtual method #5 @ RVA 0x00103e70 (2 instructions, 1 basic blocks)
    pub fn method_05(&mut self) {
    }

    /// Decompiled virtual method #6 @ RVA 0x000d6920 (2 instructions, 1 basic blocks)
    pub fn method_06(&mut self) {
    }

    /// Decompiled virtual method #7 @ RVA 0x000d6920 (2 instructions, 1 basic blocks)
    pub fn method_07(&mut self) {
    }

    /// Decompiled virtual method #8 @ RVA 0x000d6920 (2 instructions, 1 basic blocks)
    pub fn method_08(&mut self) {
    }

    /// Decompiled virtual method #9 @ RVA 0x000fe200 (2 instructions, 1 basic blocks)
    pub fn method_09(&mut self) {
    }

    /// Decompiled virtual method #10 @ RVA 0x000fe200 (2 instructions, 1 basic blocks)
    pub fn method_10(&mut self) {
    }

    /// Decompiled virtual method #11 @ RVA 0x00103e80 (2 instructions, 1 basic blocks)
    pub fn method_11(&mut self) {
    }

    /// Decompiled virtual method #12 @ RVA 0x00103e80 (2 instructions, 1 basic blocks)
    pub fn method_12(&mut self) {
    }

    /// Decompiled virtual method #13 @ RVA 0x00103e80 (2 instructions, 1 basic blocks)
    pub fn method_13(&mut self) {
    }

}

impl CcClass for ccHtmlColgroupTag {
    fn class_name(&self) -> &'static str { Self::CLASS_NAME }
    fn vtable_rva(&self) -> u32 { Self::VTABLE_RVA }
    fn virtual_method_count(&self) -> usize { Self::METHOD_COUNT }
    fn base_classes(&self) -> &'static [&'static str] { Self::BASE_CLASSES }
}

/// Decompiled CC2 class: `ccHtmlEndTag` (VTable RVA: 0x008c37f0, 14 virtual methods, 3 recovered fields)
/// Inherits: ccHtmlTagBase
#[derive(Debug, Clone, Default)]
pub struct ccHtmlEndTag {
    pub vtable_rva: u32,
    pub field_0x0000: u64,
    pub field_0x0008: u16,
    pub field_0x0028: u16,
}

impl ccHtmlEndTag {
    pub const CLASS_NAME: &'static str = "ccHtmlEndTag";
    pub const VTABLE_RVA: u32 = 0x008c37f0;
    pub const METHOD_COUNT: usize = 14;
    pub const BASE_CLASSES: &'static [&'static str] = &["ccHtmlTagBase"];
    pub const VIRTUAL_METHODS: &'static [u64] = &[0x000000018053b260, 0x0000000180103e70, 0x0000000180103e70, 0x00000001805409a0, 0x0000000180103e70, 0x0000000180103e70, 0x00000001800d6920, 0x00000001800d6920, 0x00000001800d6920, 0x000000018053b250, 0x0000000180540a30, 0x0000000180103e80, 0x0000000180103e80, 0x0000000180540a20];

    pub fn new() -> Self {
        Self {
            vtable_rva: Self::VTABLE_RVA,
            field_0x0000: Default::default(),
            field_0x0008: Default::default(),
            field_0x0028: Default::default(),
        }
    }

    /// Decompiled virtual method #0 @ RVA 0x0053b260 (13 instructions, 3 basic blocks)
    pub fn method_00(&mut self) {
        // Writes to field at offset +0x0000
    }

    /// Decompiled virtual method #1 @ RVA 0x00103e70 (2 instructions, 1 basic blocks)
    pub fn method_01(&mut self) {
    }

    /// Decompiled virtual method #2 @ RVA 0x00103e70 (2 instructions, 1 basic blocks)
    pub fn method_02(&mut self) {
    }

    /// Decompiled virtual method #3 @ RVA 0x005409a0 (21 instructions, 4 basic blocks)
    pub fn method_03(&mut self) {
        // Writes to field at offset +0x0000
    }

    /// Decompiled virtual method #4 @ RVA 0x00103e70 (2 instructions, 1 basic blocks)
    pub fn method_04(&mut self) {
    }

    /// Decompiled virtual method #5 @ RVA 0x00103e70 (2 instructions, 1 basic blocks)
    pub fn method_05(&mut self) {
    }

    /// Decompiled virtual method #6 @ RVA 0x000d6920 (2 instructions, 1 basic blocks)
    pub fn method_06(&mut self) {
    }

    /// Decompiled virtual method #7 @ RVA 0x000d6920 (2 instructions, 1 basic blocks)
    pub fn method_07(&mut self) {
    }

    /// Decompiled virtual method #8 @ RVA 0x000d6920 (2 instructions, 1 basic blocks)
    pub fn method_08(&mut self) {
    }

    /// Decompiled virtual method #9 @ RVA 0x0053b250 (3 instructions, 1 basic blocks)
    pub fn method_09(&mut self) {
        // Writes to field at offset +0x0028
    }

    /// Decompiled virtual method #10 @ RVA 0x00540a30 (47 instructions, 7 basic blocks)
    pub fn method_10(&mut self) {
    }

    /// Decompiled virtual method #11 @ RVA 0x00103e80 (2 instructions, 1 basic blocks)
    pub fn method_11(&mut self) {
    }

    /// Decompiled virtual method #12 @ RVA 0x00103e80 (2 instructions, 1 basic blocks)
    pub fn method_12(&mut self) {
    }

    /// Decompiled virtual method #13 @ RVA 0x00540a20 (4 instructions, 1 basic blocks)
    pub fn method_13(&mut self) {
        // Writes to field at offset +0x0028
    }

}

impl CcClass for ccHtmlEndTag {
    fn class_name(&self) -> &'static str { Self::CLASS_NAME }
    fn vtable_rva(&self) -> u32 { Self::VTABLE_RVA }
    fn virtual_method_count(&self) -> usize { Self::METHOD_COUNT }
    fn base_classes(&self) -> &'static [&'static str] { Self::BASE_CLASSES }
}

/// Decompiled CC2 class: `ccHtmlExtIconEmp` (VTable RVA: 0x008c36d0, 6 virtual methods, 1 recovered fields)
#[derive(Debug, Clone, Default)]
pub struct ccHtmlExtIconEmp {
    pub vtable_rva: u32,
    pub field_0x0000: u64,
}

impl ccHtmlExtIconEmp {
    pub const CLASS_NAME: &'static str = "ccHtmlExtIconEmp";
    pub const VTABLE_RVA: u32 = 0x008c36d0;
    pub const METHOD_COUNT: usize = 6;
    pub const BASE_CLASSES: &'static [&'static str] = &[];
    pub const VIRTUAL_METHODS: &'static [u64] = &[0x000000018053aa40, 0x00000001800d3090, 0x00000001800d3090, 0x00000001800d3090, 0x00000001800d3090, 0x000000018053aa20];

    pub fn new() -> Self {
        Self {
            vtable_rva: Self::VTABLE_RVA,
            field_0x0000: Default::default(),
        }
    }

    /// Decompiled virtual method #0 @ RVA 0x0053aa40 (13 instructions, 3 basic blocks)
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

    /// Decompiled virtual method #5 @ RVA 0x0053aa20 (7 instructions, 1 basic blocks)
    pub fn method_05(&mut self) {
    }

}

impl CcClass for ccHtmlExtIconEmp {
    fn class_name(&self) -> &'static str { Self::CLASS_NAME }
    fn vtable_rva(&self) -> u32 { Self::VTABLE_RVA }
    fn virtual_method_count(&self) -> usize { Self::METHOD_COUNT }
    fn base_classes(&self) -> &'static [&'static str] { Self::BASE_CLASSES }
}

/// Decompiled CC2 class: `ccHtmlFontTag` (VTable RVA: 0x008c33f0, 14 virtual methods, 5 recovered fields)
/// Inherits: ccHtmlTagBase
#[derive(Debug, Clone, Default)]
pub struct ccHtmlFontTag {
    pub vtable_rva: u32,
    pub field_0x0000: u64,
    pub field_0x0008: u16,
    pub field_0x0028: u8,
    pub field_0x0029: u8,
    pub field_0x002c: u32,
}

impl ccHtmlFontTag {
    pub const CLASS_NAME: &'static str = "ccHtmlFontTag";
    pub const VTABLE_RVA: u32 = 0x008c33f0;
    pub const METHOD_COUNT: usize = 14;
    pub const BASE_CLASSES: &'static [&'static str] = &["ccHtmlTagBase"];
    pub const VIRTUAL_METHODS: &'static [u64] = &[0x000000018053b260, 0x0000000180103e70, 0x0000000180103e70, 0x0000000180545230, 0x0000000180103e70, 0x0000000180103e70, 0x00000001800d6920, 0x00000001800d6920, 0x00000001800d6920, 0x00000001800fe200, 0x00000001805452b0, 0x0000000180103e80, 0x0000000180545270, 0x0000000180103e80];

    pub fn new() -> Self {
        Self {
            vtable_rva: Self::VTABLE_RVA,
            field_0x0000: Default::default(),
            field_0x0008: Default::default(),
            field_0x0028: Default::default(),
            field_0x0029: Default::default(),
            field_0x002c: Default::default(),
        }
    }

    /// Decompiled virtual method #0 @ RVA 0x0053b260 (13 instructions, 3 basic blocks)
    pub fn method_00(&mut self) {
        // Writes to field at offset +0x0000
    }

    /// Decompiled virtual method #1 @ RVA 0x00103e70 (2 instructions, 1 basic blocks)
    pub fn method_01(&mut self) {
    }

    /// Decompiled virtual method #2 @ RVA 0x00103e70 (2 instructions, 1 basic blocks)
    pub fn method_02(&mut self) {
    }

    /// Decompiled virtual method #3 @ RVA 0x00545230 (14 instructions, 3 basic blocks)
    pub fn method_03(&mut self) {
        // Writes to field at offset +0x0029
    }

    /// Decompiled virtual method #4 @ RVA 0x00103e70 (2 instructions, 1 basic blocks)
    pub fn method_04(&mut self) {
    }

    /// Decompiled virtual method #5 @ RVA 0x00103e70 (2 instructions, 1 basic blocks)
    pub fn method_05(&mut self) {
    }

    /// Decompiled virtual method #6 @ RVA 0x000d6920 (2 instructions, 1 basic blocks)
    pub fn method_06(&mut self) {
    }

    /// Decompiled virtual method #7 @ RVA 0x000d6920 (2 instructions, 1 basic blocks)
    pub fn method_07(&mut self) {
    }

    /// Decompiled virtual method #8 @ RVA 0x000d6920 (2 instructions, 1 basic blocks)
    pub fn method_08(&mut self) {
    }

    /// Decompiled virtual method #9 @ RVA 0x000fe200 (2 instructions, 1 basic blocks)
    pub fn method_09(&mut self) {
    }

    /// Decompiled virtual method #10 @ RVA 0x005452b0 (99 instructions, 22 basic blocks)
    pub fn method_10(&mut self) {
        // Writes to field at offset +0x0028
        // Writes to field at offset +0x0029
        // Writes to field at offset +0x002c
    }

    /// Decompiled virtual method #11 @ RVA 0x00103e80 (2 instructions, 1 basic blocks)
    pub fn method_11(&mut self) {
    }

    /// Decompiled virtual method #12 @ RVA 0x00545270 (11 instructions, 4 basic blocks)
    pub fn method_12(&mut self) {
        // Writes to field at offset +0x002c
    }

    /// Decompiled virtual method #13 @ RVA 0x00103e80 (2 instructions, 1 basic blocks)
    pub fn method_13(&mut self) {
    }

}

impl CcClass for ccHtmlFontTag {
    fn class_name(&self) -> &'static str { Self::CLASS_NAME }
    fn vtable_rva(&self) -> u32 { Self::VTABLE_RVA }
    fn virtual_method_count(&self) -> usize { Self::METHOD_COUNT }
    fn base_classes(&self) -> &'static [&'static str] { Self::BASE_CLASSES }
}

/// Decompiled CC2 class: `ccHtmlFrameSetTag` (VTable RVA: 0x008c3028, 15 virtual methods, 13 recovered fields)
/// Inherits: ccHtmlNestTag, ccHtmlTagBase
#[derive(Debug, Clone, Default)]
pub struct ccHtmlFrameSetTag {
    pub vtable_rva: u32,
    pub field_0x0000: u64,
    pub field_0x0008: u16,
    pub field_0x0028: u64,
    pub field_0x0048: u8,
    pub field_0x0049: u8,
    pub field_0x004a: u8,
    pub field_0x004b: u8,
    pub field_0x004c: u32,
    pub field_0x0054: u32,
    pub field_0x0058: u8,
    pub field_0x0059: u8,
    pub field_0x006c: f32,
    pub field_0x0070: f32,
}

impl ccHtmlFrameSetTag {
    pub const CLASS_NAME: &'static str = "ccHtmlFrameSetTag";
    pub const VTABLE_RVA: u32 = 0x008c3028;
    pub const METHOD_COUNT: usize = 15;
    pub const BASE_CLASSES: &'static [&'static str] = &["ccHtmlNestTag", "ccHtmlTagBase"];
    pub const VIRTUAL_METHODS: &'static [u64] = &[0x000000018053b640, 0x0000000180103e70, 0x0000000180103e70, 0x0000000180545d70, 0x0000000180103e70, 0x0000000180103e70, 0x00000001800d6920, 0x00000001800fe200, 0x00000001800d6920, 0x00000001800fe200, 0x0000000180546320, 0x0000000180103e80, 0x00000001805461a0, 0x0000000180103e80, 0x0000000180545ad0];

    pub fn new() -> Self {
        Self {
            vtable_rva: Self::VTABLE_RVA,
            field_0x0000: Default::default(),
            field_0x0008: Default::default(),
            field_0x0028: Default::default(),
            field_0x0048: Default::default(),
            field_0x0049: Default::default(),
            field_0x004a: Default::default(),
            field_0x004b: Default::default(),
            field_0x004c: Default::default(),
            field_0x0054: Default::default(),
            field_0x0058: Default::default(),
            field_0x0059: Default::default(),
            field_0x006c: Default::default(),
            field_0x0070: Default::default(),
        }
    }

    /// Decompiled virtual method #0 @ RVA 0x0053b640 (20 instructions, 3 basic blocks)
    pub fn method_00(&mut self) {
        // Writes to field at offset +0x0000
    }

    /// Decompiled virtual method #1 @ RVA 0x00103e70 (2 instructions, 1 basic blocks)
    pub fn method_01(&mut self) {
    }

    /// Decompiled virtual method #2 @ RVA 0x00103e70 (2 instructions, 1 basic blocks)
    pub fn method_02(&mut self) {
    }

    /// Decompiled virtual method #3 @ RVA 0x00545d70 (124 instructions, 17 basic blocks)
    pub fn method_03(&mut self) {
    }

    /// Decompiled virtual method #4 @ RVA 0x00103e70 (2 instructions, 1 basic blocks)
    pub fn method_04(&mut self) {
    }

    /// Decompiled virtual method #5 @ RVA 0x00103e70 (2 instructions, 1 basic blocks)
    pub fn method_05(&mut self) {
    }

    /// Decompiled virtual method #6 @ RVA 0x000d6920 (2 instructions, 1 basic blocks)
    pub fn method_06(&mut self) {
    }

    /// Decompiled virtual method #7 @ RVA 0x000fe200 (2 instructions, 1 basic blocks)
    pub fn method_07(&mut self) {
    }

    /// Decompiled virtual method #8 @ RVA 0x000d6920 (2 instructions, 1 basic blocks)
    pub fn method_08(&mut self) {
    }

    /// Decompiled virtual method #9 @ RVA 0x000fe200 (2 instructions, 1 basic blocks)
    pub fn method_09(&mut self) {
    }

    /// Decompiled virtual method #10 @ RVA 0x00546320 (138 instructions, 39 basic blocks)
    pub fn method_10(&mut self) {
        // Writes to field at offset +0x0048
        // Writes to field at offset +0x0049
        // Writes to field at offset +0x004a
        // Writes to field at offset +0x004b
        // Writes to field at offset +0x004c
        // Writes to field at offset +0x0054
    }

    /// Decompiled virtual method #11 @ RVA 0x00103e80 (2 instructions, 1 basic blocks)
    pub fn method_11(&mut self) {
    }

    /// Decompiled virtual method #12 @ RVA 0x005461a0 (14 instructions, 2 basic blocks)
    pub fn method_12(&mut self) {
    }

    /// Decompiled virtual method #13 @ RVA 0x00103e80 (2 instructions, 1 basic blocks)
    pub fn method_13(&mut self) {
    }

    /// Decompiled virtual method #14 @ RVA 0x00545ad0 (40 instructions, 10 basic blocks)
    pub fn method_14(&mut self) {
        // Writes to field at offset +0x0000
        // Writes to field at offset +0x0028
    }

}

impl CcClass for ccHtmlFrameSetTag {
    fn class_name(&self) -> &'static str { Self::CLASS_NAME }
    fn vtable_rva(&self) -> u32 { Self::VTABLE_RVA }
    fn virtual_method_count(&self) -> usize { Self::METHOD_COUNT }
    fn base_classes(&self) -> &'static [&'static str] { Self::BASE_CLASSES }
}

/// Decompiled CC2 class: `ccHtmlFrameTag` (VTable RVA: 0x008c30a8, 14 virtual methods, 7 recovered fields)
/// Inherits: ccHtmlViewTag, ccHtmlTagBase
#[derive(Debug, Clone, Default)]
pub struct ccHtmlFrameTag {
    pub vtable_rva: u32,
    pub field_0x0000: u64,
    pub field_0x0001: u32,
    pub field_0x0008: u16,
    pub field_0x0030: u64,
    pub field_0x0038: u64,
    pub field_0x004d: u8,
    pub field_0x0050: u64,
}

impl ccHtmlFrameTag {
    pub const CLASS_NAME: &'static str = "ccHtmlFrameTag";
    pub const VTABLE_RVA: u32 = 0x008c30a8;
    pub const METHOD_COUNT: usize = 14;
    pub const BASE_CLASSES: &'static [&'static str] = &["ccHtmlViewTag", "ccHtmlTagBase"];
    pub const VIRTUAL_METHODS: &'static [u64] = &[0x000000018053b690, 0x0000000180103e70, 0x0000000180103e70, 0x00000001805454a0, 0x0000000180103e70, 0x0000000180103e70, 0x00000001800d6920, 0x00000001800d6920, 0x00000001800fe200, 0x00000001800fe200, 0x00000001805457d0, 0x0000000180545560, 0x0000000180545500, 0x0000000180103e80];

    pub fn new() -> Self {
        Self {
            vtable_rva: Self::VTABLE_RVA,
            field_0x0000: Default::default(),
            field_0x0001: Default::default(),
            field_0x0008: Default::default(),
            field_0x0030: Default::default(),
            field_0x0038: Default::default(),
            field_0x004d: Default::default(),
            field_0x0050: Default::default(),
        }
    }

    /// Decompiled virtual method #0 @ RVA 0x0053b690 (20 instructions, 3 basic blocks)
    pub fn method_00(&mut self) {
        // Writes to field at offset +0x0000
    }

    /// Decompiled virtual method #1 @ RVA 0x00103e70 (2 instructions, 1 basic blocks)
    pub fn method_01(&mut self) {
    }

    /// Decompiled virtual method #2 @ RVA 0x00103e70 (2 instructions, 1 basic blocks)
    pub fn method_02(&mut self) {
    }

    /// Decompiled virtual method #3 @ RVA 0x005454a0 (22 instructions, 3 basic blocks)
    pub fn method_03(&mut self) {
    }

    /// Decompiled virtual method #4 @ RVA 0x00103e70 (2 instructions, 1 basic blocks)
    pub fn method_04(&mut self) {
    }

    /// Decompiled virtual method #5 @ RVA 0x00103e70 (2 instructions, 1 basic blocks)
    pub fn method_05(&mut self) {
    }

    /// Decompiled virtual method #6 @ RVA 0x000d6920 (2 instructions, 1 basic blocks)
    pub fn method_06(&mut self) {
    }

    /// Decompiled virtual method #7 @ RVA 0x000d6920 (2 instructions, 1 basic blocks)
    pub fn method_07(&mut self) {
    }

    /// Decompiled virtual method #8 @ RVA 0x000fe200 (2 instructions, 1 basic blocks)
    pub fn method_08(&mut self) {
    }

    /// Decompiled virtual method #9 @ RVA 0x000fe200 (2 instructions, 1 basic blocks)
    pub fn method_09(&mut self) {
    }

    /// Decompiled virtual method #10 @ RVA 0x005457d0 (90 instructions, 20 basic blocks)
    pub fn method_10(&mut self) {
        // Writes to field at offset +0x0038
        // Writes to field at offset +0x0050
    }

    /// Decompiled virtual method #11 @ RVA 0x00545560 (134 instructions, 28 basic blocks)
    pub fn method_11(&mut self) {
        // Writes to field at offset +0x0000
    }

    /// Decompiled virtual method #12 @ RVA 0x00545500 (15 instructions, 6 basic blocks)
    pub fn method_12(&mut self) {
        // Writes to field at offset +0x004d
    }

    /// Decompiled virtual method #13 @ RVA 0x00103e80 (2 instructions, 1 basic blocks)
    pub fn method_13(&mut self) {
    }

}

impl CcClass for ccHtmlFrameTag {
    fn class_name(&self) -> &'static str { Self::CLASS_NAME }
    fn vtable_rva(&self) -> u32 { Self::VTABLE_RVA }
    fn virtual_method_count(&self) -> usize { Self::METHOD_COUNT }
    fn base_classes(&self) -> &'static [&'static str] { Self::BASE_CLASSES }
}

/// Decompiled CC2 class: `ccHtmlHrTag` (VTable RVA: 0x008c3958, 14 virtual methods, 6 recovered fields)
/// Inherits: ccHtmlTagBase
#[derive(Debug, Clone, Default)]
pub struct ccHtmlHrTag {
    pub vtable_rva: u32,
    pub field_0x0000: u64,
    pub field_0x0008: u16,
    pub field_0x0028: u8,
    pub field_0x0029: u8,
    pub field_0x002a: u8,
    pub field_0x002c: u32,
}

impl ccHtmlHrTag {
    pub const CLASS_NAME: &'static str = "ccHtmlHrTag";
    pub const VTABLE_RVA: u32 = 0x008c3958;
    pub const METHOD_COUNT: usize = 14;
    pub const BASE_CLASSES: &'static [&'static str] = &["ccHtmlTagBase"];
    pub const VIRTUAL_METHODS: &'static [u64] = &[0x000000018053b320, 0x0000000180103e70, 0x0000000180103e70, 0x0000000180542550, 0x0000000180103e70, 0x0000000180103e70, 0x00000001800d6920, 0x00000001800d6920, 0x00000001800d6920, 0x00000001800fe200, 0x0000000180542680, 0x0000000180103e80, 0x0000000180542500, 0x0000000180103e80];

    pub fn new() -> Self {
        Self {
            vtable_rva: Self::VTABLE_RVA,
            field_0x0000: Default::default(),
            field_0x0008: Default::default(),
            field_0x0028: Default::default(),
            field_0x0029: Default::default(),
            field_0x002a: Default::default(),
            field_0x002c: Default::default(),
        }
    }

    /// Decompiled virtual method #0 @ RVA 0x0053b320 (13 instructions, 3 basic blocks)
    pub fn method_00(&mut self) {
        // Writes to field at offset +0x0000
    }

    /// Decompiled virtual method #1 @ RVA 0x00103e70 (2 instructions, 1 basic blocks)
    pub fn method_01(&mut self) {
    }

    /// Decompiled virtual method #2 @ RVA 0x00103e70 (2 instructions, 1 basic blocks)
    pub fn method_02(&mut self) {
    }

    /// Decompiled virtual method #3 @ RVA 0x00542550 (81 instructions, 12 basic blocks)
    pub fn method_03(&mut self) {
    }

    /// Decompiled virtual method #4 @ RVA 0x00103e70 (2 instructions, 1 basic blocks)
    pub fn method_04(&mut self) {
    }

    /// Decompiled virtual method #5 @ RVA 0x00103e70 (2 instructions, 1 basic blocks)
    pub fn method_05(&mut self) {
    }

    /// Decompiled virtual method #6 @ RVA 0x000d6920 (2 instructions, 1 basic blocks)
    pub fn method_06(&mut self) {
    }

    /// Decompiled virtual method #7 @ RVA 0x000d6920 (2 instructions, 1 basic blocks)
    pub fn method_07(&mut self) {
    }

    /// Decompiled virtual method #8 @ RVA 0x000d6920 (2 instructions, 1 basic blocks)
    pub fn method_08(&mut self) {
    }

    /// Decompiled virtual method #9 @ RVA 0x000fe200 (2 instructions, 1 basic blocks)
    pub fn method_09(&mut self) {
    }

    /// Decompiled virtual method #10 @ RVA 0x00542680 (120 instructions, 28 basic blocks)
    pub fn method_10(&mut self) {
        // Writes to field at offset +0x0028
        // Writes to field at offset +0x0029
        // Writes to field at offset +0x002a
        // Writes to field at offset +0x002c
    }

    /// Decompiled virtual method #11 @ RVA 0x00103e80 (2 instructions, 1 basic blocks)
    pub fn method_11(&mut self) {
    }

    /// Decompiled virtual method #12 @ RVA 0x00542500 (13 instructions, 5 basic blocks)
    pub fn method_12(&mut self) {
        // Writes to field at offset +0x002c
    }

    /// Decompiled virtual method #13 @ RVA 0x00103e80 (2 instructions, 1 basic blocks)
    pub fn method_13(&mut self) {
    }

}

impl CcClass for ccHtmlHrTag {
    fn class_name(&self) -> &'static str { Self::CLASS_NAME }
    fn vtable_rva(&self) -> u32 { Self::VTABLE_RVA }
    fn virtual_method_count(&self) -> usize { Self::METHOD_COUNT }
    fn base_classes(&self) -> &'static [&'static str] { Self::BASE_CLASSES }
}

/// Decompiled CC2 class: `ccHtmlImgTag` (VTable RVA: 0x008c3210, 14 virtual methods, 5 recovered fields)
/// Inherits: ccHtmlTagBase
#[derive(Debug, Clone, Default)]
pub struct ccHtmlImgTag {
    pub vtable_rva: u32,
    pub field_0x0000: u64,
    pub field_0x0008: u16,
    pub field_0x0028: u8,
    pub field_0x0030: u16,
    pub field_0x0032: u16,
}

impl ccHtmlImgTag {
    pub const CLASS_NAME: &'static str = "ccHtmlImgTag";
    pub const VTABLE_RVA: u32 = 0x008c3210;
    pub const METHOD_COUNT: usize = 14;
    pub const BASE_CLASSES: &'static [&'static str] = &["ccHtmlTagBase"];
    pub const VIRTUAL_METHODS: &'static [u64] = &[0x000000018053b6e0, 0x0000000180103e70, 0x0000000180103e70, 0x0000000180547080, 0x0000000180103e70, 0x0000000180103e70, 0x00000001800d6920, 0x00000001800d6920, 0x00000001800d6920, 0x00000001800fe200, 0x0000000180547a30, 0x0000000180547440, 0x0000000180547360, 0x0000000180103e80];

    pub fn new() -> Self {
        Self {
            vtable_rva: Self::VTABLE_RVA,
            field_0x0000: Default::default(),
            field_0x0008: Default::default(),
            field_0x0028: Default::default(),
            field_0x0030: Default::default(),
            field_0x0032: Default::default(),
        }
    }

    /// Decompiled virtual method #0 @ RVA 0x0053b6e0 (23 instructions, 3 basic blocks)
    pub fn method_00(&mut self) {
        // Writes to field at offset +0x0000
    }

    /// Decompiled virtual method #1 @ RVA 0x00103e70 (2 instructions, 1 basic blocks)
    pub fn method_01(&mut self) {
    }

    /// Decompiled virtual method #2 @ RVA 0x00103e70 (2 instructions, 1 basic blocks)
    pub fn method_02(&mut self) {
    }

    /// Decompiled virtual method #3 @ RVA 0x00547080 (125 instructions, 21 basic blocks)
    pub fn method_03(&mut self) {
    }

    /// Decompiled virtual method #4 @ RVA 0x00103e70 (2 instructions, 1 basic blocks)
    pub fn method_04(&mut self) {
    }

    /// Decompiled virtual method #5 @ RVA 0x00103e70 (2 instructions, 1 basic blocks)
    pub fn method_05(&mut self) {
    }

    /// Decompiled virtual method #6 @ RVA 0x000d6920 (2 instructions, 1 basic blocks)
    pub fn method_06(&mut self) {
    }

    /// Decompiled virtual method #7 @ RVA 0x000d6920 (2 instructions, 1 basic blocks)
    pub fn method_07(&mut self) {
    }

    /// Decompiled virtual method #8 @ RVA 0x000d6920 (2 instructions, 1 basic blocks)
    pub fn method_08(&mut self) {
    }

    /// Decompiled virtual method #9 @ RVA 0x000fe200 (2 instructions, 1 basic blocks)
    pub fn method_09(&mut self) {
    }

    /// Decompiled virtual method #10 @ RVA 0x00547a30 (143 instructions, 43 basic blocks)
    pub fn method_10(&mut self) {
    }

    /// Decompiled virtual method #11 @ RVA 0x00547440 (133 instructions, 32 basic blocks)
    pub fn method_11(&mut self) {
    }

    /// Decompiled virtual method #12 @ RVA 0x00547360 (14 instructions, 2 basic blocks)
    pub fn method_12(&mut self) {
        // Writes to field at offset +0x0028
    }

    /// Decompiled virtual method #13 @ RVA 0x00103e80 (2 instructions, 1 basic blocks)
    pub fn method_13(&mut self) {
    }

}

impl CcClass for ccHtmlImgTag {
    fn class_name(&self) -> &'static str { Self::CLASS_NAME }
    fn vtable_rva(&self) -> u32 { Self::VTABLE_RVA }
    fn virtual_method_count(&self) -> usize { Self::METHOD_COUNT }
    fn base_classes(&self) -> &'static [&'static str] { Self::BASE_CLASSES }
}

/// Decompiled CC2 class: `ccHtmlMovThumbnail` (VTable RVA: 0x008c3708, 6 virtual methods, 6 recovered fields)
/// Inherits: ccHtmlExtIconEmp
#[derive(Debug, Clone, Default)]
pub struct ccHtmlMovThumbnail {
    pub vtable_rva: u32,
    pub field_0x0000: u64,
    pub field_0x0018: u64,
    pub field_0x0028: f32,
    pub field_0x002c: f32,
    pub field_0x0032: u16,
    pub field_0x0034: u16,
}

impl ccHtmlMovThumbnail {
    pub const CLASS_NAME: &'static str = "ccHtmlMovThumbnail";
    pub const VTABLE_RVA: u32 = 0x008c3708;
    pub const METHOD_COUNT: usize = 6;
    pub const BASE_CLASSES: &'static [&'static str] = &["ccHtmlExtIconEmp"];
    pub const VIRTUAL_METHODS: &'static [u64] = &[0x000000018053ab80, 0x000000018053aa70, 0x0000000180232280, 0x000000018053aa80, 0x000000018053aab0, 0x000000018053ab40];

    pub fn new() -> Self {
        Self {
            vtable_rva: Self::VTABLE_RVA,
            field_0x0000: Default::default(),
            field_0x0018: Default::default(),
            field_0x0028: Default::default(),
            field_0x002c: Default::default(),
            field_0x0032: Default::default(),
            field_0x0034: Default::default(),
        }
    }

    /// Decompiled virtual method #0 @ RVA 0x0053ab80 (27 instructions, 5 basic blocks)
    pub fn method_00(&mut self) {
        // Writes to field at offset +0x0000
    }

    /// Decompiled virtual method #1 @ RVA 0x0053aa70 (2 instructions, 1 basic blocks)
    pub fn method_01(&mut self) {
        // Writes to field at offset +0x0018
    }

    /// Decompiled virtual method #2 @ RVA 0x00232280 (11 instructions, 3 basic blocks)
    pub fn method_02(&mut self) {
    }

    /// Decompiled virtual method #3 @ RVA 0x0053aa80 (50 instructions, 4 basic blocks)
    pub fn method_03(&mut self) {
        // Writes to field at offset +0x0028
        // Writes to field at offset +0x002c
    }

    /// Decompiled virtual method #4 @ RVA 0x0053aab0 (31 instructions, 3 basic blocks)
    pub fn method_04(&mut self) {
    }

    /// Decompiled virtual method #5 @ RVA 0x0053ab40 (10 instructions, 1 basic blocks)
    pub fn method_05(&mut self) {
    }

}

impl CcClass for ccHtmlMovThumbnail {
    fn class_name(&self) -> &'static str { Self::CLASS_NAME }
    fn vtable_rva(&self) -> u32 { Self::VTABLE_RVA }
    fn virtual_method_count(&self) -> usize { Self::METHOD_COUNT }
    fn base_classes(&self) -> &'static [&'static str] { Self::BASE_CLASSES }
}

/// Decompiled CC2 class: `ccHtmlNestTag` (VTable RVA: 0x008c35d0, 15 virtual methods, 2 recovered fields)
/// Inherits: ccHtmlTagBase
#[derive(Debug, Clone, Default)]
pub struct ccHtmlNestTag {
    pub vtable_rva: u32,
    pub field_0x0000: u64,
    pub field_0x0028: u64,
}

impl ccHtmlNestTag {
    pub const CLASS_NAME: &'static str = "ccHtmlNestTag";
    pub const VTABLE_RVA: u32 = 0x008c35d0;
    pub const METHOD_COUNT: usize = 15;
    pub const BASE_CLASSES: &'static [&'static str] = &["ccHtmlTagBase"];
    pub const VIRTUAL_METHODS: &'static [u64] = &[0x000000018053b520, 0x0000000180103e70, 0x0000000180103e70, 0x0000000180545a00, 0x0000000180103e70, 0x0000000180103e70, 0x00000001800d6920, 0x00000001800fe200, 0x00000001800d6920, 0x00000001800fe200, 0x00000001800fe200, 0x0000000180103e80, 0x0000000180103e80, 0x0000000180103e80, 0x0000000180545ad0];

    pub fn new() -> Self {
        Self {
            vtable_rva: Self::VTABLE_RVA,
            field_0x0000: Default::default(),
            field_0x0028: Default::default(),
        }
    }

    /// Decompiled virtual method #0 @ RVA 0x0053b520 (20 instructions, 3 basic blocks)
    pub fn method_00(&mut self) {
        // Writes to field at offset +0x0000
    }

    /// Decompiled virtual method #1 @ RVA 0x00103e70 (2 instructions, 1 basic blocks)
    pub fn method_01(&mut self) {
    }

    /// Decompiled virtual method #2 @ RVA 0x00103e70 (2 instructions, 1 basic blocks)
    pub fn method_02(&mut self) {
    }

    /// Decompiled virtual method #3 @ RVA 0x00545a00 (51 instructions, 7 basic blocks)
    pub fn method_03(&mut self) {
    }

    /// Decompiled virtual method #4 @ RVA 0x00103e70 (2 instructions, 1 basic blocks)
    pub fn method_04(&mut self) {
    }

    /// Decompiled virtual method #5 @ RVA 0x00103e70 (2 instructions, 1 basic blocks)
    pub fn method_05(&mut self) {
    }

    /// Decompiled virtual method #6 @ RVA 0x000d6920 (2 instructions, 1 basic blocks)
    pub fn method_06(&mut self) {
    }

    /// Decompiled virtual method #7 @ RVA 0x000fe200 (2 instructions, 1 basic blocks)
    pub fn method_07(&mut self) {
    }

    /// Decompiled virtual method #8 @ RVA 0x000d6920 (2 instructions, 1 basic blocks)
    pub fn method_08(&mut self) {
    }

    /// Decompiled virtual method #9 @ RVA 0x000fe200 (2 instructions, 1 basic blocks)
    pub fn method_09(&mut self) {
    }

    /// Decompiled virtual method #10 @ RVA 0x000fe200 (2 instructions, 1 basic blocks)
    pub fn method_10(&mut self) {
    }

    /// Decompiled virtual method #11 @ RVA 0x00103e80 (2 instructions, 1 basic blocks)
    pub fn method_11(&mut self) {
    }

    /// Decompiled virtual method #12 @ RVA 0x00103e80 (2 instructions, 1 basic blocks)
    pub fn method_12(&mut self) {
    }

    /// Decompiled virtual method #13 @ RVA 0x00103e80 (2 instructions, 1 basic blocks)
    pub fn method_13(&mut self) {
    }

    /// Decompiled virtual method #14 @ RVA 0x00545ad0 (40 instructions, 10 basic blocks)
    pub fn method_14(&mut self) {
        // Writes to field at offset +0x0000
        // Writes to field at offset +0x0028
    }

}

impl CcClass for ccHtmlNestTag {
    fn class_name(&self) -> &'static str { Self::CLASS_NAME }
    fn vtable_rva(&self) -> u32 { Self::VTABLE_RVA }
    fn virtual_method_count(&self) -> usize { Self::METHOD_COUNT }
    fn base_classes(&self) -> &'static [&'static str] { Self::BASE_CLASSES }
}

/// Decompiled CC2 class: `ccHtmlObjTag` (VTable RVA: 0x008c3288, 14 virtual methods, 1 recovered fields)
/// Inherits: ccHtmlTagBase
#[derive(Debug, Clone, Default)]
pub struct ccHtmlObjTag {
    pub vtable_rva: u32,
    pub field_0x0000: u64,
}

impl ccHtmlObjTag {
    pub const CLASS_NAME: &'static str = "ccHtmlObjTag";
    pub const VTABLE_RVA: u32 = 0x008c3288;
    pub const METHOD_COUNT: usize = 14;
    pub const BASE_CLASSES: &'static [&'static str] = &["ccHtmlTagBase"];
    pub const VIRTUAL_METHODS: &'static [u64] = &[0x000000018053b220, 0x0000000180103e70, 0x0000000180103e70, 0x0000000180103e70, 0x0000000180103e70, 0x0000000180103e70, 0x00000001800d6920, 0x00000001800d6920, 0x00000001800d6920, 0x00000001800fe200, 0x00000001800fe200, 0x0000000180103e80, 0x0000000180103e80, 0x0000000180103e80];

    pub fn new() -> Self {
        Self {
            vtable_rva: Self::VTABLE_RVA,
            field_0x0000: Default::default(),
        }
    }

    /// Decompiled virtual method #0 @ RVA 0x0053b220 (13 instructions, 3 basic blocks)
    pub fn method_00(&mut self) {
        // Writes to field at offset +0x0000
    }

    /// Decompiled virtual method #1 @ RVA 0x00103e70 (2 instructions, 1 basic blocks)
    pub fn method_01(&mut self) {
    }

    /// Decompiled virtual method #2 @ RVA 0x00103e70 (2 instructions, 1 basic blocks)
    pub fn method_02(&mut self) {
    }

    /// Decompiled virtual method #3 @ RVA 0x00103e70 (2 instructions, 1 basic blocks)
    pub fn method_03(&mut self) {
    }

    /// Decompiled virtual method #4 @ RVA 0x00103e70 (2 instructions, 1 basic blocks)
    pub fn method_04(&mut self) {
    }

    /// Decompiled virtual method #5 @ RVA 0x00103e70 (2 instructions, 1 basic blocks)
    pub fn method_05(&mut self) {
    }

    /// Decompiled virtual method #6 @ RVA 0x000d6920 (2 instructions, 1 basic blocks)
    pub fn method_06(&mut self) {
    }

    /// Decompiled virtual method #7 @ RVA 0x000d6920 (2 instructions, 1 basic blocks)
    pub fn method_07(&mut self) {
    }

    /// Decompiled virtual method #8 @ RVA 0x000d6920 (2 instructions, 1 basic blocks)
    pub fn method_08(&mut self) {
    }

    /// Decompiled virtual method #9 @ RVA 0x000fe200 (2 instructions, 1 basic blocks)
    pub fn method_09(&mut self) {
    }

    /// Decompiled virtual method #10 @ RVA 0x000fe200 (2 instructions, 1 basic blocks)
    pub fn method_10(&mut self) {
    }

    /// Decompiled virtual method #11 @ RVA 0x00103e80 (2 instructions, 1 basic blocks)
    pub fn method_11(&mut self) {
    }

    /// Decompiled virtual method #12 @ RVA 0x00103e80 (2 instructions, 1 basic blocks)
    pub fn method_12(&mut self) {
    }

    /// Decompiled virtual method #13 @ RVA 0x00103e80 (2 instructions, 1 basic blocks)
    pub fn method_13(&mut self) {
    }

}

impl CcClass for ccHtmlObjTag {
    fn class_name(&self) -> &'static str { Self::CLASS_NAME }
    fn vtable_rva(&self) -> u32 { Self::VTABLE_RVA }
    fn virtual_method_count(&self) -> usize { Self::METHOD_COUNT }
    fn base_classes(&self) -> &'static [&'static str] { Self::BASE_CLASSES }
}

/// Decompiled CC2 class: `ccHtmlOjBanner` (VTable RVA: 0x008c3740, 6 virtual methods, 7 recovered fields)
/// Inherits: ccHtmlExtIconEmp
#[derive(Debug, Clone, Default)]
pub struct ccHtmlOjBanner {
    pub vtable_rva: u32,
    pub field_0x0000: u64,
    pub field_0x0008: u64,
    pub field_0x0010: u64,
    pub field_0x0018: u64,
    pub field_0x0020: u64,
    pub field_0x0048: u64,
    pub field_0x0068: u64,
}

impl ccHtmlOjBanner {
    pub const CLASS_NAME: &'static str = "ccHtmlOjBanner";
    pub const VTABLE_RVA: u32 = 0x008c3740;
    pub const METHOD_COUNT: usize = 6;
    pub const BASE_CLASSES: &'static [&'static str] = &["ccHtmlExtIconEmp"];
    pub const VIRTUAL_METHODS: &'static [u64] = &[0x000000018053b050, 0x000000018053abf0, 0x000000018053af00, 0x000000018053e9c0, 0x000000018053afb0, 0x000000018053b020];

    pub fn new() -> Self {
        Self {
            vtable_rva: Self::VTABLE_RVA,
            field_0x0000: Default::default(),
            field_0x0008: Default::default(),
            field_0x0010: Default::default(),
            field_0x0018: Default::default(),
            field_0x0020: Default::default(),
            field_0x0048: Default::default(),
            field_0x0068: Default::default(),
        }
    }

    /// Decompiled virtual method #0 @ RVA 0x0053b050 (23 instructions, 3 basic blocks)
    pub fn method_00(&mut self) {
        // Writes to field at offset +0x0000
    }

    /// Decompiled virtual method #1 @ RVA 0x0053abf0 (130 instructions, 21 basic blocks)
    pub fn method_01(&mut self) {
    }

    /// Decompiled virtual method #2 @ RVA 0x0053af00 (49 instructions, 17 basic blocks)
    pub fn method_02(&mut self) {
    }

    /// Decompiled virtual method #3 @ RVA 0x0053e9c0 (104 instructions, 11 basic blocks)
    pub fn method_03(&mut self) {
    }

    /// Decompiled virtual method #4 @ RVA 0x0053afb0 (46 instructions, 2 basic blocks)
    pub fn method_04(&mut self) {
    }

    /// Decompiled virtual method #5 @ RVA 0x0053b020 (10 instructions, 1 basic blocks)
    pub fn method_05(&mut self) {
    }

}

impl CcClass for ccHtmlOjBanner {
    fn class_name(&self) -> &'static str { Self::CLASS_NAME }
    fn vtable_rva(&self) -> u32 { Self::VTABLE_RVA }
    fn virtual_method_count(&self) -> usize { Self::METHOD_COUNT }
    fn base_classes(&self) -> &'static [&'static str] { Self::BASE_CLASSES }
}

/// Decompiled CC2 class: `ccHtmlTableTag` (VTable RVA: 0x008c2fa8, 15 virtual methods, 12 recovered fields)
/// Inherits: ccHtmlNestTag, ccHtmlTagBase
#[derive(Debug, Clone, Default)]
pub struct ccHtmlTableTag {
    pub vtable_rva: u32,
    pub field_0x0000: u64,
    pub field_0x0008: u16,
    pub field_0x0010: u64,
    pub field_0x0028: u64,
    pub field_0x0030: u16,
    pub field_0x0032: u16,
    pub field_0x0034: u8,
    pub field_0x0035: u8,
    pub field_0x0036: u8,
    pub field_0x0038: u32,
    pub field_0x003c: u32,
    pub field_0x0040: u64,
}

impl ccHtmlTableTag {
    pub const CLASS_NAME: &'static str = "ccHtmlTableTag";
    pub const VTABLE_RVA: u32 = 0x008c2fa8;
    pub const METHOD_COUNT: usize = 15;
    pub const BASE_CLASSES: &'static [&'static str] = &["ccHtmlNestTag", "ccHtmlTagBase"];
    pub const VIRTUAL_METHODS: &'static [u64] = &[0x000000018053b5e0, 0x0000000180103e70, 0x0000000180103e70, 0x0000000180542f80, 0x0000000180542f10, 0x0000000180103e70, 0x00000001800d6920, 0x00000001800fe200, 0x00000001800d6920, 0x00000001800fe200, 0x0000000180543ff0, 0x0000000180543f20, 0x0000000180543e80, 0x0000000180103e80, 0x0000000180542940];

    pub fn new() -> Self {
        Self {
            vtable_rva: Self::VTABLE_RVA,
            field_0x0000: Default::default(),
            field_0x0008: Default::default(),
            field_0x0010: Default::default(),
            field_0x0028: Default::default(),
            field_0x0030: Default::default(),
            field_0x0032: Default::default(),
            field_0x0034: Default::default(),
            field_0x0035: Default::default(),
            field_0x0036: Default::default(),
            field_0x0038: Default::default(),
            field_0x003c: Default::default(),
            field_0x0040: Default::default(),
        }
    }

    /// Decompiled virtual method #0 @ RVA 0x0053b5e0 (24 instructions, 3 basic blocks)
    pub fn method_00(&mut self) {
        // Writes to field at offset +0x0000
    }

    /// Decompiled virtual method #1 @ RVA 0x00103e70 (2 instructions, 1 basic blocks)
    pub fn method_01(&mut self) {
    }

    /// Decompiled virtual method #2 @ RVA 0x00103e70 (2 instructions, 1 basic blocks)
    pub fn method_02(&mut self) {
    }

    /// Decompiled virtual method #3 @ RVA 0x00542f80 (127 instructions, 14 basic blocks)
    pub fn method_03(&mut self) {
        // Writes to field at offset +0x0038
    }

    /// Decompiled virtual method #4 @ RVA 0x00542f10 (32 instructions, 10 basic blocks)
    pub fn method_04(&mut self) {
    }

    /// Decompiled virtual method #5 @ RVA 0x00103e70 (2 instructions, 1 basic blocks)
    pub fn method_05(&mut self) {
    }

    /// Decompiled virtual method #6 @ RVA 0x000d6920 (2 instructions, 1 basic blocks)
    pub fn method_06(&mut self) {
    }

    /// Decompiled virtual method #7 @ RVA 0x000fe200 (2 instructions, 1 basic blocks)
    pub fn method_07(&mut self) {
    }

    /// Decompiled virtual method #8 @ RVA 0x000d6920 (2 instructions, 1 basic blocks)
    pub fn method_08(&mut self) {
    }

    /// Decompiled virtual method #9 @ RVA 0x000fe200 (2 instructions, 1 basic blocks)
    pub fn method_09(&mut self) {
    }

    /// Decompiled virtual method #10 @ RVA 0x00543ff0 (143 instructions, 39 basic blocks)
    pub fn method_10(&mut self) {
        // Writes to field at offset +0x0030
        // Writes to field at offset +0x0032
        // Writes to field at offset +0x0034
        // Writes to field at offset +0x0035
        // Writes to field at offset +0x0036
        // Writes to field at offset +0x0038
        // Writes to field at offset +0x003c
        // Writes to field at offset +0x0040
    }

    /// Decompiled virtual method #11 @ RVA 0x00543f20 (56 instructions, 13 basic blocks)
    pub fn method_11(&mut self) {
    }

    /// Decompiled virtual method #12 @ RVA 0x00543e80 (13 instructions, 2 basic blocks)
    pub fn method_12(&mut self) {
        // Writes to field at offset +0x0030
    }

    /// Decompiled virtual method #13 @ RVA 0x00103e80 (2 instructions, 1 basic blocks)
    pub fn method_13(&mut self) {
    }

    /// Decompiled virtual method #14 @ RVA 0x00542940 (132 instructions, 27 basic blocks)
    pub fn method_14(&mut self) {
        // Writes to field at offset +0x0000
    }

}

impl CcClass for ccHtmlTableTag {
    fn class_name(&self) -> &'static str { Self::CLASS_NAME }
    fn vtable_rva(&self) -> u32 { Self::VTABLE_RVA }
    fn virtual_method_count(&self) -> usize { Self::METHOD_COUNT }
    fn base_classes(&self) -> &'static [&'static str] { Self::BASE_CLASSES }
}

/// Decompiled CC2 class: `ccHtmlTagBase` (VTable RVA: 0x008c3778, 14 virtual methods, 1 recovered fields)
#[derive(Debug, Clone, Default)]
pub struct ccHtmlTagBase {
    pub vtable_rva: u32,
    pub field_0x0000: u64,
}

impl ccHtmlTagBase {
    pub const CLASS_NAME: &'static str = "ccHtmlTagBase";
    pub const VTABLE_RVA: u32 = 0x008c3778;
    pub const METHOD_COUNT: usize = 14;
    pub const BASE_CLASSES: &'static [&'static str] = &[];
    pub const VIRTUAL_METHODS: &'static [u64] = &[0x000000018053b220, 0x0000000180103e70, 0x0000000180103e70, 0x0000000180103e70, 0x0000000180103e70, 0x0000000180103e70, 0x00000001800d6920, 0x00000001800d6920, 0x00000001800d6920, 0x00000001800fe200, 0x00000001800d6920, 0x0000000180103e80, 0x0000000180103e80, 0x0000000180103e80];

    pub fn new() -> Self {
        Self {
            vtable_rva: Self::VTABLE_RVA,
            field_0x0000: Default::default(),
        }
    }

    /// Decompiled virtual method #0 @ RVA 0x0053b220 (13 instructions, 3 basic blocks)
    pub fn method_00(&mut self) {
        // Writes to field at offset +0x0000
    }

    /// Decompiled virtual method #1 @ RVA 0x00103e70 (2 instructions, 1 basic blocks)
    pub fn method_01(&mut self) {
    }

    /// Decompiled virtual method #2 @ RVA 0x00103e70 (2 instructions, 1 basic blocks)
    pub fn method_02(&mut self) {
    }

    /// Decompiled virtual method #3 @ RVA 0x00103e70 (2 instructions, 1 basic blocks)
    pub fn method_03(&mut self) {
    }

    /// Decompiled virtual method #4 @ RVA 0x00103e70 (2 instructions, 1 basic blocks)
    pub fn method_04(&mut self) {
    }

    /// Decompiled virtual method #5 @ RVA 0x00103e70 (2 instructions, 1 basic blocks)
    pub fn method_05(&mut self) {
    }

    /// Decompiled virtual method #6 @ RVA 0x000d6920 (2 instructions, 1 basic blocks)
    pub fn method_06(&mut self) {
    }

    /// Decompiled virtual method #7 @ RVA 0x000d6920 (2 instructions, 1 basic blocks)
    pub fn method_07(&mut self) {
    }

    /// Decompiled virtual method #8 @ RVA 0x000d6920 (2 instructions, 1 basic blocks)
    pub fn method_08(&mut self) {
    }

    /// Decompiled virtual method #9 @ RVA 0x000fe200 (2 instructions, 1 basic blocks)
    pub fn method_09(&mut self) {
    }

    /// Decompiled virtual method #10 @ RVA 0x000d6920 (2 instructions, 1 basic blocks)
    pub fn method_10(&mut self) {
    }

    /// Decompiled virtual method #11 @ RVA 0x00103e80 (2 instructions, 1 basic blocks)
    pub fn method_11(&mut self) {
    }

    /// Decompiled virtual method #12 @ RVA 0x00103e80 (2 instructions, 1 basic blocks)
    pub fn method_12(&mut self) {
    }

    /// Decompiled virtual method #13 @ RVA 0x00103e80 (2 instructions, 1 basic blocks)
    pub fn method_13(&mut self) {
    }

}

impl CcClass for ccHtmlTagBase {
    fn class_name(&self) -> &'static str { Self::CLASS_NAME }
    fn vtable_rva(&self) -> u32 { Self::VTABLE_RVA }
    fn virtual_method_count(&self) -> usize { Self::METHOD_COUNT }
    fn base_classes(&self) -> &'static [&'static str] { Self::BASE_CLASSES }
}

/// Decompiled CC2 class: `ccHtmlTdTag` (VTable RVA: 0x008c3558, 14 virtual methods, 17 recovered fields)
/// Inherits: ccHtmlTagBase
#[derive(Debug, Clone, Default)]
pub struct ccHtmlTdTag {
    pub vtable_rva: u32,
    pub field_0x0000: u8,
    pub field_0x0001: u32,
    pub field_0x0004: u16,
    pub field_0x0006: u8,
    pub field_0x0008: u16,
    pub field_0x001a: u8,
    pub field_0x0028: u8,
    pub field_0x0029: u8,
    pub field_0x002a: u8,
    pub field_0x002b: u8,
    pub field_0x002c: u16,
    pub field_0x002e: u16,
    pub field_0x0038: u64,
    pub field_0x0048: u16,
    pub field_0x0074: u8,
    pub field_0x00f8: u64,
    pub field_0x0100: u64,
}

impl ccHtmlTdTag {
    pub const CLASS_NAME: &'static str = "ccHtmlTdTag";
    pub const VTABLE_RVA: u32 = 0x008c3558;
    pub const METHOD_COUNT: usize = 14;
    pub const BASE_CLASSES: &'static [&'static str] = &["ccHtmlTagBase"];
    pub const VIRTUAL_METHODS: &'static [u64] = &[0x000000018053b4e0, 0x0000000180103e70, 0x0000000180103e70, 0x0000000180544480, 0x0000000180103e70, 0x0000000180103e70, 0x00000001800d6920, 0x00000001800d6920, 0x00000001800d6920, 0x00000001800fe200, 0x0000000180544c50, 0x0000000180544820, 0x0000000180544760, 0x0000000180103e80];

    pub fn new() -> Self {
        Self {
            vtable_rva: Self::VTABLE_RVA,
            field_0x0000: Default::default(),
            field_0x0001: Default::default(),
            field_0x0004: Default::default(),
            field_0x0006: Default::default(),
            field_0x0008: Default::default(),
            field_0x001a: Default::default(),
            field_0x0028: Default::default(),
            field_0x0029: Default::default(),
            field_0x002a: Default::default(),
            field_0x002b: Default::default(),
            field_0x002c: Default::default(),
            field_0x002e: Default::default(),
            field_0x0038: Default::default(),
            field_0x0048: Default::default(),
            field_0x0074: Default::default(),
            field_0x00f8: Default::default(),
            field_0x0100: Default::default(),
        }
    }

    /// Decompiled virtual method #0 @ RVA 0x0053b4e0 (16 instructions, 3 basic blocks)
    pub fn method_00(&mut self) {
    }

    /// Decompiled virtual method #1 @ RVA 0x00103e70 (2 instructions, 1 basic blocks)
    pub fn method_01(&mut self) {
    }

    /// Decompiled virtual method #2 @ RVA 0x00103e70 (2 instructions, 1 basic blocks)
    pub fn method_02(&mut self) {
    }

    /// Decompiled virtual method #3 @ RVA 0x00544480 (126 instructions, 24 basic blocks)
    pub fn method_03(&mut self) {
        // Writes to field at offset +0x001a
    }

    /// Decompiled virtual method #4 @ RVA 0x00103e70 (2 instructions, 1 basic blocks)
    pub fn method_04(&mut self) {
    }

    /// Decompiled virtual method #5 @ RVA 0x00103e70 (2 instructions, 1 basic blocks)
    pub fn method_05(&mut self) {
    }

    /// Decompiled virtual method #6 @ RVA 0x000d6920 (2 instructions, 1 basic blocks)
    pub fn method_06(&mut self) {
    }

    /// Decompiled virtual method #7 @ RVA 0x000d6920 (2 instructions, 1 basic blocks)
    pub fn method_07(&mut self) {
    }

    /// Decompiled virtual method #8 @ RVA 0x000d6920 (2 instructions, 1 basic blocks)
    pub fn method_08(&mut self) {
    }

    /// Decompiled virtual method #9 @ RVA 0x000fe200 (2 instructions, 1 basic blocks)
    pub fn method_09(&mut self) {
    }

    /// Decompiled virtual method #10 @ RVA 0x00544c50 (142 instructions, 39 basic blocks)
    pub fn method_10(&mut self) {
        // Writes to field at offset +0x0028
        // Writes to field at offset +0x0029
        // Writes to field at offset +0x002a
        // Writes to field at offset +0x002b
        // Writes to field at offset +0x002c
        // Writes to field at offset +0x002e
        // Writes to field at offset +0x0038
        // Writes to field at offset +0x0074
    }

    /// Decompiled virtual method #11 @ RVA 0x00544820 (136 instructions, 33 basic blocks)
    pub fn method_11(&mut self) {
        // Writes to field at offset +0x0000
        // Writes to field at offset +0x0000
        // Writes to field at offset +0x0001
        // Writes to field at offset +0x0004
        // Writes to field at offset +0x0006
    }

    /// Decompiled virtual method #12 @ RVA 0x00544760 (13 instructions, 2 basic blocks)
    pub fn method_12(&mut self) {
        // Writes to field at offset +0x0028
    }

    /// Decompiled virtual method #13 @ RVA 0x00103e80 (2 instructions, 1 basic blocks)
    pub fn method_13(&mut self) {
    }

}

impl CcClass for ccHtmlTdTag {
    fn class_name(&self) -> &'static str { Self::CLASS_NAME }
    fn vtable_rva(&self) -> u32 { Self::VTABLE_RVA }
    fn virtual_method_count(&self) -> usize { Self::METHOD_COUNT }
    fn base_classes(&self) -> &'static [&'static str] { Self::BASE_CLASSES }
}

/// Decompiled CC2 class: `ccHtmlText` (VTable RVA: 0x008c3868, 14 virtual methods, 7 recovered fields)
/// Inherits: ccHtmlTagBase
#[derive(Debug, Clone, Default)]
pub struct ccHtmlText {
    pub vtable_rva: u32,
    pub field_0x0000: u64,
    pub field_0x0008: u16,
    pub field_0x000c: u32,
    pub field_0x0010: u64,
    pub field_0x0028: u64,
    pub field_0x0030: u32,
    pub field_0x004c: u8,
}

impl ccHtmlText {
    pub const CLASS_NAME: &'static str = "ccHtmlText";
    pub const VTABLE_RVA: u32 = 0x008c3868;
    pub const METHOD_COUNT: usize = 14;
    pub const BASE_CLASSES: &'static [&'static str] = &["ccHtmlTagBase"];
    pub const VIRTUAL_METHODS: &'static [u64] = &[0x000000018053b290, 0x0000000180103e70, 0x0000000180103e70, 0x0000000180540c20, 0x0000000180103e70, 0x0000000180103e70, 0x00000001800d6920, 0x00000001800d6920, 0x00000001800d6920, 0x00000001800fe200, 0x0000000180540ff0, 0x0000000180103e80, 0x0000000180103e80, 0x0000000180540af0];

    pub fn new() -> Self {
        Self {
            vtable_rva: Self::VTABLE_RVA,
            field_0x0000: Default::default(),
            field_0x0008: Default::default(),
            field_0x000c: Default::default(),
            field_0x0010: Default::default(),
            field_0x0028: Default::default(),
            field_0x0030: Default::default(),
            field_0x004c: Default::default(),
        }
    }

    /// Decompiled virtual method #0 @ RVA 0x0053b290 (37 instructions, 6 basic blocks)
    pub fn method_00(&mut self) {
        // Writes to field at offset +0x0000
    }

    /// Decompiled virtual method #1 @ RVA 0x00103e70 (2 instructions, 1 basic blocks)
    pub fn method_01(&mut self) {
    }

    /// Decompiled virtual method #2 @ RVA 0x00103e70 (2 instructions, 1 basic blocks)
    pub fn method_02(&mut self) {
    }

    /// Decompiled virtual method #3 @ RVA 0x00540c20 (107 instructions, 17 basic blocks)
    pub fn method_03(&mut self) {
        // Writes to field at offset +0x000c
        // Writes to field at offset +0x0010
        // Writes to field at offset +0x004c
    }

    /// Decompiled virtual method #4 @ RVA 0x00103e70 (2 instructions, 1 basic blocks)
    pub fn method_04(&mut self) {
    }

    /// Decompiled virtual method #5 @ RVA 0x00103e70 (2 instructions, 1 basic blocks)
    pub fn method_05(&mut self) {
    }

    /// Decompiled virtual method #6 @ RVA 0x000d6920 (2 instructions, 1 basic blocks)
    pub fn method_06(&mut self) {
    }

    /// Decompiled virtual method #7 @ RVA 0x000d6920 (2 instructions, 1 basic blocks)
    pub fn method_07(&mut self) {
    }

    /// Decompiled virtual method #8 @ RVA 0x000d6920 (2 instructions, 1 basic blocks)
    pub fn method_08(&mut self) {
    }

    /// Decompiled virtual method #9 @ RVA 0x000fe200 (2 instructions, 1 basic blocks)
    pub fn method_09(&mut self) {
    }

    /// Decompiled virtual method #10 @ RVA 0x00540ff0 (64 instructions, 12 basic blocks)
    pub fn method_10(&mut self) {
    }

    /// Decompiled virtual method #11 @ RVA 0x00103e80 (2 instructions, 1 basic blocks)
    pub fn method_11(&mut self) {
    }

    /// Decompiled virtual method #12 @ RVA 0x00103e80 (2 instructions, 1 basic blocks)
    pub fn method_12(&mut self) {
    }

    /// Decompiled virtual method #13 @ RVA 0x00540af0 (29 instructions, 5 basic blocks)
    pub fn method_13(&mut self) {
    }

}

impl CcClass for ccHtmlText {
    fn class_name(&self) -> &'static str { Self::CLASS_NAME }
    fn vtable_rva(&self) -> u32 { Self::VTABLE_RVA }
    fn virtual_method_count(&self) -> usize { Self::METHOD_COUNT }
    fn base_classes(&self) -> &'static [&'static str] { Self::BASE_CLASSES }
}

/// Decompiled CC2 class: `ccHtmlTitleTag` (VTable RVA: 0x008c3378, 14 virtual methods, 3 recovered fields)
/// Inherits: ccHtmlTagBase
#[derive(Debug, Clone, Default)]
pub struct ccHtmlTitleTag {
    pub vtable_rva: u32,
    pub field_0x0000: u64,
    pub field_0x0008: u16,
    pub field_0x0028: u64,
}

impl ccHtmlTitleTag {
    pub const CLASS_NAME: &'static str = "ccHtmlTitleTag";
    pub const VTABLE_RVA: u32 = 0x008c3378;
    pub const METHOD_COUNT: usize = 14;
    pub const BASE_CLASSES: &'static [&'static str] = &["ccHtmlTagBase"];
    pub const VIRTUAL_METHODS: &'static [u64] = &[0x000000018053b350, 0x0000000180103e70, 0x0000000180103e70, 0x0000000180103e70, 0x0000000180103e70, 0x0000000180103e70, 0x00000001800d6920, 0x00000001800d6920, 0x00000001800d6920, 0x00000001800fe200, 0x0000000180542490, 0x0000000180103e80, 0x0000000180103e80, 0x0000000180544fb0];

    pub fn new() -> Self {
        Self {
            vtable_rva: Self::VTABLE_RVA,
            field_0x0000: Default::default(),
            field_0x0008: Default::default(),
            field_0x0028: Default::default(),
        }
    }

    /// Decompiled virtual method #0 @ RVA 0x0053b350 (37 instructions, 6 basic blocks)
    pub fn method_00(&mut self) {
        // Writes to field at offset +0x0000
    }

    /// Decompiled virtual method #1 @ RVA 0x00103e70 (2 instructions, 1 basic blocks)
    pub fn method_01(&mut self) {
    }

    /// Decompiled virtual method #2 @ RVA 0x00103e70 (2 instructions, 1 basic blocks)
    pub fn method_02(&mut self) {
    }

    /// Decompiled virtual method #3 @ RVA 0x00103e70 (2 instructions, 1 basic blocks)
    pub fn method_03(&mut self) {
    }

    /// Decompiled virtual method #4 @ RVA 0x00103e70 (2 instructions, 1 basic blocks)
    pub fn method_04(&mut self) {
    }

    /// Decompiled virtual method #5 @ RVA 0x00103e70 (2 instructions, 1 basic blocks)
    pub fn method_05(&mut self) {
    }

    /// Decompiled virtual method #6 @ RVA 0x000d6920 (2 instructions, 1 basic blocks)
    pub fn method_06(&mut self) {
    }

    /// Decompiled virtual method #7 @ RVA 0x000d6920 (2 instructions, 1 basic blocks)
    pub fn method_07(&mut self) {
    }

    /// Decompiled virtual method #8 @ RVA 0x000d6920 (2 instructions, 1 basic blocks)
    pub fn method_08(&mut self) {
    }

    /// Decompiled virtual method #9 @ RVA 0x000fe200 (2 instructions, 1 basic blocks)
    pub fn method_09(&mut self) {
    }

    /// Decompiled virtual method #10 @ RVA 0x00542490 (26 instructions, 4 basic blocks)
    pub fn method_10(&mut self) {
    }

    /// Decompiled virtual method #11 @ RVA 0x00103e80 (2 instructions, 1 basic blocks)
    pub fn method_11(&mut self) {
    }

    /// Decompiled virtual method #12 @ RVA 0x00103e80 (2 instructions, 1 basic blocks)
    pub fn method_12(&mut self) {
    }

    /// Decompiled virtual method #13 @ RVA 0x00544fb0 (47 instructions, 10 basic blocks)
    pub fn method_13(&mut self) {
    }

}

impl CcClass for ccHtmlTitleTag {
    fn class_name(&self) -> &'static str { Self::CLASS_NAME }
    fn vtable_rva(&self) -> u32 { Self::VTABLE_RVA }
    fn virtual_method_count(&self) -> usize { Self::METHOD_COUNT }
    fn base_classes(&self) -> &'static [&'static str] { Self::BASE_CLASSES }
}

/// Decompiled CC2 class: `ccHtmlTrTag` (VTable RVA: 0x008c34e0, 14 virtual methods, 4 recovered fields)
/// Inherits: ccHtmlTagBase
#[derive(Debug, Clone, Default)]
pub struct ccHtmlTrTag {
    pub vtable_rva: u32,
    pub field_0x0000: u64,
    pub field_0x0008: u16,
    pub field_0x0028: u8,
    pub field_0x0029: u8,
}

impl ccHtmlTrTag {
    pub const CLASS_NAME: &'static str = "ccHtmlTrTag";
    pub const VTABLE_RVA: u32 = 0x008c34e0;
    pub const METHOD_COUNT: usize = 14;
    pub const BASE_CLASSES: &'static [&'static str] = &["ccHtmlTagBase"];
    pub const VIRTUAL_METHODS: &'static [u64] = &[0x000000018053b430, 0x0000000180103e70, 0x0000000180103e70, 0x0000000180103e70, 0x0000000180103e70, 0x0000000180103e70, 0x00000001800d6920, 0x00000001800d6920, 0x00000001800d6920, 0x00000001800fe200, 0x0000000180544350, 0x0000000180103e80, 0x0000000180544330, 0x0000000180103e80];

    pub fn new() -> Self {
        Self {
            vtable_rva: Self::VTABLE_RVA,
            field_0x0000: Default::default(),
            field_0x0008: Default::default(),
            field_0x0028: Default::default(),
            field_0x0029: Default::default(),
        }
    }

    /// Decompiled virtual method #0 @ RVA 0x0053b430 (13 instructions, 3 basic blocks)
    pub fn method_00(&mut self) {
        // Writes to field at offset +0x0000
    }

    /// Decompiled virtual method #1 @ RVA 0x00103e70 (2 instructions, 1 basic blocks)
    pub fn method_01(&mut self) {
    }

    /// Decompiled virtual method #2 @ RVA 0x00103e70 (2 instructions, 1 basic blocks)
    pub fn method_02(&mut self) {
    }

    /// Decompiled virtual method #3 @ RVA 0x00103e70 (2 instructions, 1 basic blocks)
    pub fn method_03(&mut self) {
    }

    /// Decompiled virtual method #4 @ RVA 0x00103e70 (2 instructions, 1 basic blocks)
    pub fn method_04(&mut self) {
    }

    /// Decompiled virtual method #5 @ RVA 0x00103e70 (2 instructions, 1 basic blocks)
    pub fn method_05(&mut self) {
    }

    /// Decompiled virtual method #6 @ RVA 0x000d6920 (2 instructions, 1 basic blocks)
    pub fn method_06(&mut self) {
    }

    /// Decompiled virtual method #7 @ RVA 0x000d6920 (2 instructions, 1 basic blocks)
    pub fn method_07(&mut self) {
    }

    /// Decompiled virtual method #8 @ RVA 0x000d6920 (2 instructions, 1 basic blocks)
    pub fn method_08(&mut self) {
    }

    /// Decompiled virtual method #9 @ RVA 0x000fe200 (2 instructions, 1 basic blocks)
    pub fn method_09(&mut self) {
    }

    /// Decompiled virtual method #10 @ RVA 0x00544350 (79 instructions, 16 basic blocks)
    pub fn method_10(&mut self) {
        // Writes to field at offset +0x0028
        // Writes to field at offset +0x0029
    }

    /// Decompiled virtual method #11 @ RVA 0x00103e80 (2 instructions, 1 basic blocks)
    pub fn method_11(&mut self) {
    }

    /// Decompiled virtual method #12 @ RVA 0x00544330 (9 instructions, 3 basic blocks)
    pub fn method_12(&mut self) {
        // Writes to field at offset +0x0029
    }

    /// Decompiled virtual method #13 @ RVA 0x00103e80 (2 instructions, 1 basic blocks)
    pub fn method_13(&mut self) {
    }

}

impl CcClass for ccHtmlTrTag {
    fn class_name(&self) -> &'static str { Self::CLASS_NAME }
    fn vtable_rva(&self) -> u32 { Self::VTABLE_RVA }
    fn virtual_method_count(&self) -> usize { Self::METHOD_COUNT }
    fn base_classes(&self) -> &'static [&'static str] { Self::BASE_CLASSES }
}

/// Decompiled CC2 class: `ccHtmlUpdateTag` (VTable RVA: 0x008c2f28, 15 virtual methods, 4 recovered fields)
/// Inherits: ccHtmlNestTag, ccHtmlTagBase
#[derive(Debug, Clone, Default)]
pub struct ccHtmlUpdateTag {
    pub vtable_rva: u32,
    pub field_0x0000: u64,
    pub field_0x0028: u64,
    pub field_0x0030: u32,
    pub field_0x0034: u8,
}

impl ccHtmlUpdateTag {
    pub const CLASS_NAME: &'static str = "ccHtmlUpdateTag";
    pub const VTABLE_RVA: u32 = 0x008c2f28;
    pub const METHOD_COUNT: usize = 15;
    pub const BASE_CLASSES: &'static [&'static str] = &["ccHtmlNestTag", "ccHtmlTagBase"];
    pub const VIRTUAL_METHODS: &'static [u64] = &[0x000000018053b740, 0x0000000180103e70, 0x0000000180103e70, 0x0000000180545060, 0x0000000180103e70, 0x0000000180103e70, 0x00000001800d6920, 0x00000001800fe200, 0x00000001800d6920, 0x00000001800fe200, 0x0000000180545130, 0x0000000180103e80, 0x0000000180545200, 0x0000000180103e80, 0x0000000180545ad0];

    pub fn new() -> Self {
        Self {
            vtable_rva: Self::VTABLE_RVA,
            field_0x0000: Default::default(),
            field_0x0028: Default::default(),
            field_0x0030: Default::default(),
            field_0x0034: Default::default(),
        }
    }

    /// Decompiled virtual method #0 @ RVA 0x0053b740 (20 instructions, 3 basic blocks)
    pub fn method_00(&mut self) {
        // Writes to field at offset +0x0000
    }

    /// Decompiled virtual method #1 @ RVA 0x00103e70 (2 instructions, 1 basic blocks)
    pub fn method_01(&mut self) {
    }

    /// Decompiled virtual method #2 @ RVA 0x00103e70 (2 instructions, 1 basic blocks)
    pub fn method_02(&mut self) {
    }

    /// Decompiled virtual method #3 @ RVA 0x00545060 (16 instructions, 3 basic blocks)
    pub fn method_03(&mut self) {
    }

    /// Decompiled virtual method #4 @ RVA 0x00103e70 (2 instructions, 1 basic blocks)
    pub fn method_04(&mut self) {
    }

    /// Decompiled virtual method #5 @ RVA 0x00103e70 (2 instructions, 1 basic blocks)
    pub fn method_05(&mut self) {
    }

    /// Decompiled virtual method #6 @ RVA 0x000d6920 (2 instructions, 1 basic blocks)
    pub fn method_06(&mut self) {
    }

    /// Decompiled virtual method #7 @ RVA 0x000fe200 (2 instructions, 1 basic blocks)
    pub fn method_07(&mut self) {
    }

    /// Decompiled virtual method #8 @ RVA 0x000d6920 (2 instructions, 1 basic blocks)
    pub fn method_08(&mut self) {
    }

    /// Decompiled virtual method #9 @ RVA 0x000fe200 (2 instructions, 1 basic blocks)
    pub fn method_09(&mut self) {
    }

    /// Decompiled virtual method #10 @ RVA 0x00545130 (52 instructions, 9 basic blocks)
    pub fn method_10(&mut self) {
    }

    /// Decompiled virtual method #11 @ RVA 0x00103e80 (2 instructions, 1 basic blocks)
    pub fn method_11(&mut self) {
    }

    /// Decompiled virtual method #12 @ RVA 0x00545200 (9 instructions, 3 basic blocks)
    pub fn method_12(&mut self) {
        // Writes to field at offset +0x0034
    }

    /// Decompiled virtual method #13 @ RVA 0x00103e80 (2 instructions, 1 basic blocks)
    pub fn method_13(&mut self) {
    }

    /// Decompiled virtual method #14 @ RVA 0x00545ad0 (40 instructions, 10 basic blocks)
    pub fn method_14(&mut self) {
        // Writes to field at offset +0x0000
        // Writes to field at offset +0x0028
    }

}

impl CcClass for ccHtmlUpdateTag {
    fn class_name(&self) -> &'static str { Self::CLASS_NAME }
    fn vtable_rva(&self) -> u32 { Self::VTABLE_RVA }
    fn virtual_method_count(&self) -> usize { Self::METHOD_COUNT }
    fn base_classes(&self) -> &'static [&'static str] { Self::BASE_CLASSES }
}

/// Decompiled CC2 class: `ccItemTrainPtsUpCtrl` (VTable RVA: 0x008c3b60, 37 virtual methods, 12 recovered fields)
/// Inherits: ccGameMenuCtrlBase, ccGameMenuObj
#[derive(Debug, Clone, Default)]
pub struct ccItemTrainPtsUpCtrl {
    pub vtable_rva: u32,
    pub field_0x0000: u64,
    pub field_0x0004: u32,
    pub field_0x0008: u32,
    pub field_0x0018: u32,
    pub field_0x0038: u32,
    pub field_0x0048: u64,
    pub field_0x0050: u64,
    pub field_0x0068: u64,
    pub field_0x0094: u32,
    pub field_0x009c: u32,
    pub field_0x2bf5: u8,
    pub field_0x2bf6: u8,
}

impl ccItemTrainPtsUpCtrl {
    pub const CLASS_NAME: &'static str = "ccItemTrainPtsUpCtrl";
    pub const VTABLE_RVA: u32 = 0x008c3b60;
    pub const METHOD_COUNT: usize = 37;
    pub const BASE_CLASSES: &'static [&'static str] = &["ccGameMenuCtrlBase", "ccGameMenuObj"];
    pub const VIRTUAL_METHODS: &'static [u64] = &[0x000000018054ea30, 0x00000001804520b0, 0x0000000180452130, 0x0000000180451fb0, 0x0000000180451ff0, 0x0000000180452030, 0x0000000180452070, 0x0000000180103e90, 0x0000000180103ea0, 0x0000000180103e60, 0x000000018045bb30, 0x0000000180451d90, 0x00000001801e9580, 0x000000018054ebc0, 0x0000000180103e60, 0x0000000180451dc0, 0x0000000180451e30, 0x0000000180451ea0, 0x0000000180451ee0, 0x0000000180451f40, 0x000000018054ec80, 0x000000018054ed60, 0x0000000180103e70, 0x0000000180103e60, 0x0000000180103e60, 0x0000000180103e70, 0x0000000180103e70, 0x0000000180103e60, 0x0000000180103e60, 0x0000000180103e60, 0x000000018054ede0, 0x000000018054ee70, 0x00000001800d3090, 0x0000000180103e70, 0x0000000180103e70, 0x0000000180103e70, 0x0000000180103e60];

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
            field_0x0068: Default::default(),
            field_0x0094: Default::default(),
            field_0x009c: Default::default(),
            field_0x2bf5: Default::default(),
            field_0x2bf6: Default::default(),
        }
    }

    /// Decompiled virtual method #0 @ RVA 0x0054ea30 (16 instructions, 3 basic blocks)
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

    /// Decompiled virtual method #13 @ RVA 0x0054ebc0 (50 instructions, 6 basic blocks)
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

    /// Decompiled virtual method #20 @ RVA 0x0054ec80 (50 instructions, 14 basic blocks)
    pub fn method_20(&mut self) {
        // Writes to field at offset +0x0038
        // Writes to field at offset +0x0048
        // Writes to field at offset +0x0094
    }

    /// Decompiled virtual method #21 @ RVA 0x0054ed60 (27 instructions, 8 basic blocks)
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

    /// Decompiled virtual method #30 @ RVA 0x0054ede0 (18 instructions, 2 basic blocks)
    pub fn method_30(&mut self) {
    }

    /// Decompiled virtual method #31 @ RVA 0x0054ee70 (30 instructions, 5 basic blocks)
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

impl CcClass for ccItemTrainPtsUpCtrl {
    fn class_name(&self) -> &'static str { Self::CLASS_NAME }
    fn vtable_rva(&self) -> u32 { Self::VTABLE_RVA }
    fn virtual_method_count(&self) -> usize { Self::METHOD_COUNT }
    fn base_classes(&self) -> &'static [&'static str] { Self::BASE_CLASSES }
}

/// Decompiled CC2 class: `ccKingChimAI` (VTable RVA: 0x008bc998, 23 virtual methods, 24 recovered fields)
/// Inherits: ccChimAI
#[derive(Debug, Clone, Default)]
pub struct ccKingChimAI {
    pub vtable_rva: u32,
    pub field_0x0000: u64,
    pub field_0x0001: u32,
    pub field_0x0004: f32,
    pub field_0x0010: u64,
    pub field_0x0020: u32,
    pub field_0x0021: u8,
    pub field_0x0024: u16,
    pub field_0x002c: u16,
    pub field_0x002e: u16,
    pub field_0x0034: u32,
    pub field_0x0040: u32,
    pub field_0x0044: f32,
    pub field_0x0048: f32,
    pub field_0x0050: u64,
    pub field_0x0054: u32,
    pub field_0x0090: u64,
    pub field_0x00a0: u32,
    pub field_0x00a4: u32,
    pub field_0x00c0: u64,
    pub field_0x00cc: f32,
    pub field_0x00e0: u32,
    pub field_0x0101: u8,
    pub field_0x0108: u16,
    pub field_0x0160: u8,
}

impl ccKingChimAI {
    pub const CLASS_NAME: &'static str = "ccKingChimAI";
    pub const VTABLE_RVA: u32 = 0x008bc998;
    pub const METHOD_COUNT: usize = 23;
    pub const BASE_CLASSES: &'static [&'static str] = &["ccChimAI"];
    pub const VIRTUAL_METHODS: &'static [u64] = &[0x0000000180298820, 0x0000000180298c90, 0x0000000180298ce0, 0x00000001802987f0, 0x0000000180299080, 0x000000018029cf50, 0x000000018029d310, 0x0000000180299cc0, 0x00000001802998c0, 0x0000000180299f50, 0x000000018029e030, 0x000000018029d640, 0x000000018029a680, 0x000000018029db50, 0x000000018029dbc0, 0x000000018029dd90, 0x000000018029e080, 0x000000018029abe0, 0x000000018029ac00, 0x000000018029aeb0, 0x000000018029b520, 0x000000018029b5f0, 0x000000018029b870];

    pub fn new() -> Self {
        Self {
            vtable_rva: Self::VTABLE_RVA,
            field_0x0000: Default::default(),
            field_0x0001: Default::default(),
            field_0x0004: Default::default(),
            field_0x0010: Default::default(),
            field_0x0020: Default::default(),
            field_0x0021: Default::default(),
            field_0x0024: Default::default(),
            field_0x002c: Default::default(),
            field_0x002e: Default::default(),
            field_0x0034: Default::default(),
            field_0x0040: Default::default(),
            field_0x0044: Default::default(),
            field_0x0048: Default::default(),
            field_0x0050: Default::default(),
            field_0x0054: Default::default(),
            field_0x0090: Default::default(),
            field_0x00a0: Default::default(),
            field_0x00a4: Default::default(),
            field_0x00c0: Default::default(),
            field_0x00cc: Default::default(),
            field_0x00e0: Default::default(),
            field_0x0101: Default::default(),
            field_0x0108: Default::default(),
            field_0x0160: Default::default(),
        }
    }

    /// Decompiled virtual method #0 @ RVA 0x00298820 (124 instructions, 31 basic blocks)
    pub fn method_00(&mut self) {
        // Writes to field at offset +0x0000
    }

    /// Decompiled virtual method #1 @ RVA 0x00298c90 (15 instructions, 5 basic blocks)
    pub fn method_01(&mut self) {
        // Writes to field at offset +0x002e
        // Writes to field at offset +0x00e0
        // Writes to field at offset +0x0160
    }

    /// Decompiled virtual method #2 @ RVA 0x00298ce0 (125 instructions, 30 basic blocks)
    pub fn method_02(&mut self) {
        // Writes to field at offset +0x0000
    }

    /// Decompiled virtual method #3 @ RVA 0x002987f0 (11 instructions, 3 basic blocks)
    pub fn method_03(&mut self) {
        // Writes to field at offset +0x0020
        // Writes to field at offset +0x002c
        // Writes to field at offset +0x002e
        // Writes to field at offset +0x0034
    }

    /// Decompiled virtual method #4 @ RVA 0x00299080 (123 instructions, 23 basic blocks)
    pub fn method_04(&mut self) {
    }

    /// Decompiled virtual method #5 @ RVA 0x0029cf50 (106 instructions, 19 basic blocks)
    pub fn method_05(&mut self) {
    }

    /// Decompiled virtual method #6 @ RVA 0x0029d310 (128 instructions, 16 basic blocks)
    pub fn method_06(&mut self) {
    }

    /// Decompiled virtual method #7 @ RVA 0x00299cc0 (54 instructions, 5 basic blocks)
    pub fn method_07(&mut self) {
    }

    /// Decompiled virtual method #8 @ RVA 0x002998c0 (54 instructions, 5 basic blocks)
    pub fn method_08(&mut self) {
    }

    /// Decompiled virtual method #9 @ RVA 0x00299f50 (60 instructions, 8 basic blocks)
    pub fn method_09(&mut self) {
    }

    /// Decompiled virtual method #10 @ RVA 0x0029e030 (13 instructions, 5 basic blocks)
    pub fn method_10(&mut self) {
        // Writes to field at offset +0x0021
        // Writes to field at offset +0x0044
        // Writes to field at offset +0x0048
    }

    /// Decompiled virtual method #11 @ RVA 0x0029d640 (97 instructions, 18 basic blocks)
    pub fn method_11(&mut self) {
    }

    /// Decompiled virtual method #12 @ RVA 0x0029a680 (67 instructions, 10 basic blocks)
    pub fn method_12(&mut self) {
    }

    /// Decompiled virtual method #13 @ RVA 0x0029db50 (24 instructions, 9 basic blocks)
    pub fn method_13(&mut self) {
    }

    /// Decompiled virtual method #14 @ RVA 0x0029dbc0 (46 instructions, 10 basic blocks)
    pub fn method_14(&mut self) {
    }

    /// Decompiled virtual method #15 @ RVA 0x0029dd90 (32 instructions, 7 basic blocks)
    pub fn method_15(&mut self) {
    }

    /// Decompiled virtual method #16 @ RVA 0x0029e080 (119 instructions, 28 basic blocks)
    pub fn method_16(&mut self) {
        // Writes to field at offset +0x0000
        // Writes to field at offset +0x00cc
    }

    /// Decompiled virtual method #17 @ RVA 0x0029abe0 (6 instructions, 1 basic blocks)
    pub fn method_17(&mut self) {
        // Writes to field at offset +0x0020
        // Writes to field at offset +0x002c
        // Writes to field at offset +0x002e
        // Writes to field at offset +0x0034
    }

    /// Decompiled virtual method #18 @ RVA 0x0029ac00 (97 instructions, 12 basic blocks)
    pub fn method_18(&mut self) {
        // Writes to field at offset +0x0054
    }

    /// Decompiled virtual method #19 @ RVA 0x0029aeb0 (121 instructions, 28 basic blocks)
    pub fn method_19(&mut self) {
        // Writes to field at offset +0x0000
    }

    /// Decompiled virtual method #20 @ RVA 0x0029b520 (20 instructions, 7 basic blocks)
    pub fn method_20(&mut self) {
        // Writes to field at offset +0x0040
    }

    /// Decompiled virtual method #21 @ RVA 0x0029b5f0 (61 instructions, 9 basic blocks)
    pub fn method_21(&mut self) {
    }

    /// Decompiled virtual method #22 @ RVA 0x0029b870 (109 instructions, 28 basic blocks)
    pub fn method_22(&mut self) {
        // Writes to field at offset +0x0000
        // Writes to field at offset +0x00cc
    }

}

impl CcClass for ccKingChimAI {
    fn class_name(&self) -> &'static str { Self::CLASS_NAME }
    fn vtable_rva(&self) -> u32 { Self::VTABLE_RVA }
    fn virtual_method_count(&self) -> usize { Self::METHOD_COUNT }
    fn base_classes(&self) -> &'static [&'static str] { Self::BASE_CLASSES }
}

/// Decompiled CC2 class: `ccLight` (VTable RVA: 0x00876ef0, 2 virtual methods, 2 recovered fields)
/// Inherits: ccCoord
#[derive(Debug, Clone, Default)]
pub struct ccLight {
    pub vtable_rva: u32,
    pub field_0x0000: u64,
    pub field_0x00d0: u64,
}

impl ccLight {
    pub const CLASS_NAME: &'static str = "ccLight";
    pub const VTABLE_RVA: u32 = 0x00876ef0;
    pub const METHOD_COUNT: usize = 2;
    pub const BASE_CLASSES: &'static [&'static str] = &["ccCoord"];
    pub const VIRTUAL_METHODS: &'static [u64] = &[0x000000018014c430, 0x000000018073e2c0];

    pub fn new() -> Self {
        Self {
            vtable_rva: Self::VTABLE_RVA,
            field_0x0000: Default::default(),
            field_0x00d0: Default::default(),
        }
    }

    /// Decompiled virtual method #0 @ RVA 0x0014c430 (22 instructions, 5 basic blocks)
    pub fn method_00(&mut self) {
        // Writes to field at offset +0x0000
    }

    /// Decompiled virtual method #1 @ RVA 0x0073e2c0 (337 instructions, 1 basic blocks)
    pub fn method_01(&mut self) {
    }

}

impl CcClass for ccLight {
    fn class_name(&self) -> &'static str { Self::CLASS_NAME }
    fn vtable_rva(&self) -> u32 { Self::VTABLE_RVA }
    fn virtual_method_count(&self) -> usize { Self::METHOD_COUNT }
    fn base_classes(&self) -> &'static [&'static str] { Self::BASE_CLASSES }
}

/// Decompiled CC2 class: `ccMcGameScreen` (VTable RVA: 0x008eb8f8, 22 virtual methods, 25 recovered fields)
/// Inherits: ccMcScreen
#[derive(Debug, Clone, Default)]
pub struct ccMcGameScreen {
    pub vtable_rva: u32,
    pub field_0x0000: u8,
    pub field_0x0004: u32,
    pub field_0x000c: u32,
    pub field_0x0011: u8,
    pub field_0x0014: u64,
    pub field_0x0016: u32,
    pub field_0x0018: f32,
    pub field_0x001a: u8,
    pub field_0x001c: f32,
    pub field_0x0020: f32,
    pub field_0x0024: f32,
    pub field_0x002c: u32,
    pub field_0x0030: u32,
    pub field_0x0068: u64,
    pub field_0x0070: u8,
    pub field_0x0218: u64,
    pub field_0x0220: u64,
    pub field_0x0818: f32,
    pub field_0x0820: u32,
    pub field_0x0838: u64,
    pub field_0x084c: f32,
    pub field_0x0850: u32,
    pub field_0x0868: u64,
    pub field_0x1ab8: f32,
    pub field_0x1abc: f32,
}

impl ccMcGameScreen {
    pub const CLASS_NAME: &'static str = "ccMcGameScreen";
    pub const VTABLE_RVA: u32 = 0x008eb8f8;
    pub const METHOD_COUNT: usize = 22;
    pub const BASE_CLASSES: &'static [&'static str] = &["ccMcScreen"];
    pub const VIRTUAL_METHODS: &'static [u64] = &[0x0000000180577910, 0x00000001800d3090, 0x0000000180616fe0, 0x0000000180577300, 0x00000001805773d0, 0x0000000180577790, 0x0000000180617ac0, 0x0000000180617b90, 0x0000000180617bf0, 0x0000000180578720, 0x0000000180578550, 0x0000000180578660, 0x0000000180617c10, 0x0000000180617c50, 0x0000000180617070, 0x00000001806170b0, 0x00000001806170f0, 0x00000001800d3090, 0x0000000180617340, 0x0000000180617130, 0x0000000180617220, 0x0000000180616d70];

    pub fn new() -> Self {
        Self {
            vtable_rva: Self::VTABLE_RVA,
            field_0x0000: Default::default(),
            field_0x0004: Default::default(),
            field_0x000c: Default::default(),
            field_0x0011: Default::default(),
            field_0x0014: Default::default(),
            field_0x0016: Default::default(),
            field_0x0018: Default::default(),
            field_0x001a: Default::default(),
            field_0x001c: Default::default(),
            field_0x0020: Default::default(),
            field_0x0024: Default::default(),
            field_0x002c: Default::default(),
            field_0x0030: Default::default(),
            field_0x0068: Default::default(),
            field_0x0070: Default::default(),
            field_0x0218: Default::default(),
            field_0x0220: Default::default(),
            field_0x0818: Default::default(),
            field_0x0820: Default::default(),
            field_0x0838: Default::default(),
            field_0x084c: Default::default(),
            field_0x0850: Default::default(),
            field_0x0868: Default::default(),
            field_0x1ab8: Default::default(),
            field_0x1abc: Default::default(),
        }
    }

    /// Decompiled virtual method #0 @ RVA 0x00577910 (136 instructions, 35 basic blocks)
    pub fn method_00(&mut self) {
    }

    /// Decompiled virtual method #1 @ RVA 0x000d3090 (1 instructions, 1 basic blocks)
    pub fn method_01(&mut self) {
    }

    /// Decompiled virtual method #2 @ RVA 0x00616fe0 (45 instructions, 4 basic blocks)
    pub fn method_02(&mut self) {
    }

    /// Decompiled virtual method #3 @ RVA 0x00577300 (48 instructions, 5 basic blocks)
    pub fn method_03(&mut self) {
        // Writes to field at offset +0x0004
    }

    /// Decompiled virtual method #4 @ RVA 0x005773d0 (146 instructions, 28 basic blocks)
    pub fn method_04(&mut self) {
        // Writes to field at offset +0x0000
        // Writes to field at offset +0x001a
    }

    /// Decompiled virtual method #5 @ RVA 0x00577790 (82 instructions, 11 basic blocks)
    pub fn method_05(&mut self) {
        // Writes to field at offset +0x0018
        // Writes to field at offset +0x001c
        // Writes to field at offset +0x0020
        // Writes to field at offset +0x0024
        // Writes to field at offset +0x0070
        // Writes to field at offset +0x1ab8
        // Writes to field at offset +0x1abc
    }

    /// Decompiled virtual method #6 @ RVA 0x00617ac0 (70 instructions, 6 basic blocks)
    pub fn method_06(&mut self) {
        // Writes to field at offset +0x000c
        // Writes to field at offset +0x0011
        // Writes to field at offset +0x0818
        // Writes to field at offset +0x0820
        // Writes to field at offset +0x084c
        // Writes to field at offset +0x0850
    }

    /// Decompiled virtual method #7 @ RVA 0x00617b90 (16 instructions, 3 basic blocks)
    pub fn method_07(&mut self) {
        // Writes to field at offset +0x000c
        // Writes to field at offset +0x0011
        // Writes to field at offset +0x0818
        // Writes to field at offset +0x0820
        // Writes to field at offset +0x084c
        // Writes to field at offset +0x0850
    }

    /// Decompiled virtual method #8 @ RVA 0x00617bf0 (5 instructions, 2 basic blocks)
    pub fn method_08(&mut self) {
    }

    /// Decompiled virtual method #9 @ RVA 0x00578720 (9 instructions, 3 basic blocks)
    pub fn method_09(&mut self) {
    }

    /// Decompiled virtual method #10 @ RVA 0x00578550 (19 instructions, 6 basic blocks)
    pub fn method_10(&mut self) {
        // Writes to field at offset +0x0000
        // Writes to field at offset +0x0014
    }

    /// Decompiled virtual method #11 @ RVA 0x00578660 (15 instructions, 5 basic blocks)
    pub fn method_11(&mut self) {
        // Writes to field at offset +0x0004
        // Writes to field at offset +0x0014
    }

    /// Decompiled virtual method #12 @ RVA 0x00617c10 (12 instructions, 4 basic blocks)
    pub fn method_12(&mut self) {
    }

    /// Decompiled virtual method #13 @ RVA 0x00617c50 (38 instructions, 7 basic blocks)
    pub fn method_13(&mut self) {
    }

    /// Decompiled virtual method #14 @ RVA 0x00617070 (12 instructions, 1 basic blocks)
    pub fn method_14(&mut self) {
    }

    /// Decompiled virtual method #15 @ RVA 0x006170b0 (12 instructions, 1 basic blocks)
    pub fn method_15(&mut self) {
    }

    /// Decompiled virtual method #16 @ RVA 0x006170f0 (12 instructions, 1 basic blocks)
    pub fn method_16(&mut self) {
    }

    /// Decompiled virtual method #17 @ RVA 0x000d3090 (1 instructions, 1 basic blocks)
    pub fn method_17(&mut self) {
    }

    /// Decompiled virtual method #18 @ RVA 0x00617340 (25 instructions, 2 basic blocks)
    pub fn method_18(&mut self) {
    }

    /// Decompiled virtual method #19 @ RVA 0x00617130 (47 instructions, 6 basic blocks)
    pub fn method_19(&mut self) {
    }

    /// Decompiled virtual method #20 @ RVA 0x00617220 (66 instructions, 9 basic blocks)
    pub fn method_20(&mut self) {
        // Writes to field at offset +0x0068
    }

    /// Decompiled virtual method #21 @ RVA 0x00616d70 (16 instructions, 3 basic blocks)
    pub fn method_21(&mut self) {
    }

}

impl CcClass for ccMcGameScreen {
    fn class_name(&self) -> &'static str { Self::CLASS_NAME }
    fn vtable_rva(&self) -> u32 { Self::VTABLE_RVA }
    fn virtual_method_count(&self) -> usize { Self::METHOD_COUNT }
    fn base_classes(&self) -> &'static [&'static str] { Self::BASE_CLASSES }
}

/// Decompiled CC2 class: `ccMcScreen` (VTable RVA: 0x008a50b8, 21 virtual methods, 20 recovered fields)
#[derive(Debug, Clone, Default)]
pub struct ccMcScreen {
    pub vtable_rva: u32,
    pub field_0x0000: u8,
    pub field_0x0004: u32,
    pub field_0x000c: u32,
    pub field_0x0011: u8,
    pub field_0x0016: u32,
    pub field_0x0018: f32,
    pub field_0x001a: u8,
    pub field_0x001c: f32,
    pub field_0x0020: f32,
    pub field_0x0024: f32,
    pub field_0x002c: u32,
    pub field_0x0030: u32,
    pub field_0x0070: u8,
    pub field_0x0218: u64,
    pub field_0x0220: u64,
    pub field_0x0818: f32,
    pub field_0x081c: f32,
    pub field_0x0820: u32,
    pub field_0x1ab8: f32,
    pub field_0x1abc: f32,
}

impl ccMcScreen {
    pub const CLASS_NAME: &'static str = "ccMcScreen";
    pub const VTABLE_RVA: u32 = 0x008a50b8;
    pub const METHOD_COUNT: usize = 21;
    pub const BASE_CLASSES: &'static [&'static str] = &[];
    pub const VIRTUAL_METHODS: &'static [u64] = &[0x0000000180577910, 0x00000001800d3090, 0x00000001800d3090, 0x0000000180577300, 0x00000001805773d0, 0x0000000180577790, 0x00000001805778c0, 0x00000001805778e0, 0x00000001801499d0, 0x00000001800d6920, 0x00000001800d3090, 0x00000001800d3090, 0x0000000180103e90, 0x00000001800d3090, 0x00000001800d3090, 0x00000001800d3090, 0x00000001800d3090, 0x00000001800d3090, 0x00000001805771f0, 0x0000000180103e90, 0x0000000180576c90];

    pub fn new() -> Self {
        Self {
            vtable_rva: Self::VTABLE_RVA,
            field_0x0000: Default::default(),
            field_0x0004: Default::default(),
            field_0x000c: Default::default(),
            field_0x0011: Default::default(),
            field_0x0016: Default::default(),
            field_0x0018: Default::default(),
            field_0x001a: Default::default(),
            field_0x001c: Default::default(),
            field_0x0020: Default::default(),
            field_0x0024: Default::default(),
            field_0x002c: Default::default(),
            field_0x0030: Default::default(),
            field_0x0070: Default::default(),
            field_0x0218: Default::default(),
            field_0x0220: Default::default(),
            field_0x0818: Default::default(),
            field_0x081c: Default::default(),
            field_0x0820: Default::default(),
            field_0x1ab8: Default::default(),
            field_0x1abc: Default::default(),
        }
    }

    /// Decompiled virtual method #0 @ RVA 0x00577910 (136 instructions, 35 basic blocks)
    pub fn method_00(&mut self) {
    }

    /// Decompiled virtual method #1 @ RVA 0x000d3090 (1 instructions, 1 basic blocks)
    pub fn method_01(&mut self) {
    }

    /// Decompiled virtual method #2 @ RVA 0x000d3090 (1 instructions, 1 basic blocks)
    pub fn method_02(&mut self) {
    }

    /// Decompiled virtual method #3 @ RVA 0x00577300 (48 instructions, 5 basic blocks)
    pub fn method_03(&mut self) {
        // Writes to field at offset +0x0004
    }

    /// Decompiled virtual method #4 @ RVA 0x005773d0 (146 instructions, 28 basic blocks)
    pub fn method_04(&mut self) {
        // Writes to field at offset +0x0000
        // Writes to field at offset +0x001a
    }

    /// Decompiled virtual method #5 @ RVA 0x00577790 (82 instructions, 11 basic blocks)
    pub fn method_05(&mut self) {
        // Writes to field at offset +0x0018
        // Writes to field at offset +0x001c
        // Writes to field at offset +0x0020
        // Writes to field at offset +0x0024
        // Writes to field at offset +0x0070
        // Writes to field at offset +0x1ab8
        // Writes to field at offset +0x1abc
    }

    /// Decompiled virtual method #6 @ RVA 0x005778c0 (5 instructions, 1 basic blocks)
    pub fn method_06(&mut self) {
        // Writes to field at offset +0x000c
        // Writes to field at offset +0x0011
        // Writes to field at offset +0x0818
        // Writes to field at offset +0x0820
    }

    /// Decompiled virtual method #7 @ RVA 0x005778e0 (6 instructions, 1 basic blocks)
    pub fn method_07(&mut self) {
        // Writes to field at offset +0x000c
        // Writes to field at offset +0x0011
        // Writes to field at offset +0x0818
        // Writes to field at offset +0x0820
    }

    /// Decompiled virtual method #8 @ RVA 0x001499d0 (2 instructions, 1 basic blocks)
    pub fn method_08(&mut self) {
    }

    /// Decompiled virtual method #9 @ RVA 0x000d6920 (2 instructions, 1 basic blocks)
    pub fn method_09(&mut self) {
    }

    /// Decompiled virtual method #10 @ RVA 0x000d3090 (1 instructions, 1 basic blocks)
    pub fn method_10(&mut self) {
    }

    /// Decompiled virtual method #11 @ RVA 0x000d3090 (1 instructions, 1 basic blocks)
    pub fn method_11(&mut self) {
    }

    /// Decompiled virtual method #12 @ RVA 0x00103e90 (2 instructions, 1 basic blocks)
    pub fn method_12(&mut self) {
    }

    /// Decompiled virtual method #13 @ RVA 0x000d3090 (1 instructions, 1 basic blocks)
    pub fn method_13(&mut self) {
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

    /// Decompiled virtual method #17 @ RVA 0x000d3090 (1 instructions, 1 basic blocks)
    pub fn method_17(&mut self) {
    }

    /// Decompiled virtual method #18 @ RVA 0x005771f0 (25 instructions, 2 basic blocks)
    pub fn method_18(&mut self) {
    }

    /// Decompiled virtual method #19 @ RVA 0x00103e90 (2 instructions, 1 basic blocks)
    pub fn method_19(&mut self) {
    }

    /// Decompiled virtual method #20 @ RVA 0x00576c90 (119 instructions, 12 basic blocks)
    pub fn method_20(&mut self) {
        // Writes to field at offset +0x0000
    }

}

impl CcClass for ccMcScreen {
    fn class_name(&self) -> &'static str { Self::CLASS_NAME }
    fn vtable_rva(&self) -> u32 { Self::VTABLE_RVA }
    fn virtual_method_count(&self) -> usize { Self::METHOD_COUNT }
    fn base_classes(&self) -> &'static [&'static str] { Self::BASE_CLASSES }
}

/// Decompiled CC2 class: `ccMcScreenTitle` (VTable RVA: 0x008a7208, 22 virtual methods, 23 recovered fields)
/// Inherits: ccMcScreen
#[derive(Debug, Clone, Default)]
pub struct ccMcScreenTitle {
    pub vtable_rva: u32,
    pub field_0x0000: u8,
    pub field_0x0001: u32,
    pub field_0x0004: u32,
    pub field_0x0008: u32,
    pub field_0x000c: u32,
    pub field_0x0011: u8,
    pub field_0x0016: u32,
    pub field_0x0018: f32,
    pub field_0x001a: u8,
    pub field_0x001c: f32,
    pub field_0x0020: f32,
    pub field_0x0024: f32,
    pub field_0x002c: u32,
    pub field_0x0030: u32,
    pub field_0x0070: u8,
    pub field_0x0218: u64,
    pub field_0x0220: u64,
    pub field_0x0818: f32,
    pub field_0x0820: u32,
    pub field_0x082c: u8,
    pub field_0x0840: u64,
    pub field_0x1ab8: f32,
    pub field_0x1abc: f32,
}

impl ccMcScreenTitle {
    pub const CLASS_NAME: &'static str = "ccMcScreenTitle";
    pub const VTABLE_RVA: u32 = 0x008a7208;
    pub const METHOD_COUNT: usize = 22;
    pub const BASE_CLASSES: &'static [&'static str] = &["ccMcScreen"];
    pub const VIRTUAL_METHODS: &'static [u64] = &[0x0000000180577910, 0x00000001800d3090, 0x00000001802b88b0, 0x0000000180577300, 0x00000001805773d0, 0x0000000180577790, 0x00000001805778c0, 0x00000001805778e0, 0x00000001801499d0, 0x00000001800d6920, 0x00000001800d3090, 0x00000001800d3090, 0x0000000180103e90, 0x00000001800d3090, 0x00000001800d3090, 0x00000001802b9cf0, 0x00000001802b9d30, 0x00000001802b9d70, 0x00000001802b89e0, 0x00000001802b89a0, 0x00000001802b8ca0, 0x00000001802b9db0];

    pub fn new() -> Self {
        Self {
            vtable_rva: Self::VTABLE_RVA,
            field_0x0000: Default::default(),
            field_0x0001: Default::default(),
            field_0x0004: Default::default(),
            field_0x0008: Default::default(),
            field_0x000c: Default::default(),
            field_0x0011: Default::default(),
            field_0x0016: Default::default(),
            field_0x0018: Default::default(),
            field_0x001a: Default::default(),
            field_0x001c: Default::default(),
            field_0x0020: Default::default(),
            field_0x0024: Default::default(),
            field_0x002c: Default::default(),
            field_0x0030: Default::default(),
            field_0x0070: Default::default(),
            field_0x0218: Default::default(),
            field_0x0220: Default::default(),
            field_0x0818: Default::default(),
            field_0x0820: Default::default(),
            field_0x082c: Default::default(),
            field_0x0840: Default::default(),
            field_0x1ab8: Default::default(),
            field_0x1abc: Default::default(),
        }
    }

    /// Decompiled virtual method #0 @ RVA 0x00577910 (136 instructions, 35 basic blocks)
    pub fn method_00(&mut self) {
    }

    /// Decompiled virtual method #1 @ RVA 0x000d3090 (1 instructions, 1 basic blocks)
    pub fn method_01(&mut self) {
    }

    /// Decompiled virtual method #2 @ RVA 0x002b88b0 (17 instructions, 2 basic blocks)
    pub fn method_02(&mut self) {
    }

    /// Decompiled virtual method #3 @ RVA 0x00577300 (48 instructions, 5 basic blocks)
    pub fn method_03(&mut self) {
        // Writes to field at offset +0x0004
    }

    /// Decompiled virtual method #4 @ RVA 0x005773d0 (146 instructions, 28 basic blocks)
    pub fn method_04(&mut self) {
        // Writes to field at offset +0x0000
        // Writes to field at offset +0x001a
    }

    /// Decompiled virtual method #5 @ RVA 0x00577790 (82 instructions, 11 basic blocks)
    pub fn method_05(&mut self) {
        // Writes to field at offset +0x0018
        // Writes to field at offset +0x001c
        // Writes to field at offset +0x0020
        // Writes to field at offset +0x0024
        // Writes to field at offset +0x0070
        // Writes to field at offset +0x1ab8
        // Writes to field at offset +0x1abc
    }

    /// Decompiled virtual method #6 @ RVA 0x005778c0 (5 instructions, 1 basic blocks)
    pub fn method_06(&mut self) {
        // Writes to field at offset +0x000c
        // Writes to field at offset +0x0011
        // Writes to field at offset +0x0818
        // Writes to field at offset +0x0820
    }

    /// Decompiled virtual method #7 @ RVA 0x005778e0 (6 instructions, 1 basic blocks)
    pub fn method_07(&mut self) {
        // Writes to field at offset +0x000c
        // Writes to field at offset +0x0011
        // Writes to field at offset +0x0818
        // Writes to field at offset +0x0820
    }

    /// Decompiled virtual method #8 @ RVA 0x001499d0 (2 instructions, 1 basic blocks)
    pub fn method_08(&mut self) {
    }

    /// Decompiled virtual method #9 @ RVA 0x000d6920 (2 instructions, 1 basic blocks)
    pub fn method_09(&mut self) {
    }

    /// Decompiled virtual method #10 @ RVA 0x000d3090 (1 instructions, 1 basic blocks)
    pub fn method_10(&mut self) {
    }

    /// Decompiled virtual method #11 @ RVA 0x000d3090 (1 instructions, 1 basic blocks)
    pub fn method_11(&mut self) {
    }

    /// Decompiled virtual method #12 @ RVA 0x00103e90 (2 instructions, 1 basic blocks)
    pub fn method_12(&mut self) {
    }

    /// Decompiled virtual method #13 @ RVA 0x000d3090 (1 instructions, 1 basic blocks)
    pub fn method_13(&mut self) {
    }

    /// Decompiled virtual method #14 @ RVA 0x000d3090 (1 instructions, 1 basic blocks)
    pub fn method_14(&mut self) {
    }

    /// Decompiled virtual method #15 @ RVA 0x002b9cf0 (12 instructions, 1 basic blocks)
    pub fn method_15(&mut self) {
    }

    /// Decompiled virtual method #16 @ RVA 0x002b9d30 (12 instructions, 1 basic blocks)
    pub fn method_16(&mut self) {
    }

    /// Decompiled virtual method #17 @ RVA 0x002b9d70 (12 instructions, 1 basic blocks)
    pub fn method_17(&mut self) {
    }

    /// Decompiled virtual method #18 @ RVA 0x002b89e0 (25 instructions, 2 basic blocks)
    pub fn method_18(&mut self) {
    }

    /// Decompiled virtual method #19 @ RVA 0x002b89a0 (14 instructions, 3 basic blocks)
    pub fn method_19(&mut self) {
        // Writes to field at offset +0x000c
        // Writes to field at offset +0x082c
    }

    /// Decompiled virtual method #20 @ RVA 0x002b8ca0 (120 instructions, 20 basic blocks)
    pub fn method_20(&mut self) {
    }

    /// Decompiled virtual method #21 @ RVA 0x002b9db0 (91 instructions, 22 basic blocks)
    pub fn method_21(&mut self) {
        // Writes to field at offset +0x0004
        // Writes to field at offset +0x0008
    }

}

impl CcClass for ccMcScreenTitle {
    fn class_name(&self) -> &'static str { Self::CLASS_NAME }
    fn vtable_rva(&self) -> u32 { Self::VTABLE_RVA }
    fn virtual_method_count(&self) -> usize { Self::METHOD_COUNT }
    fn base_classes(&self) -> &'static [&'static str] { Self::BASE_CLASSES }
}

/// Decompiled CC2 class: `ccMessageBox::system::gu` (VTable RVA: 0x00865c68, 5 virtual methods, 6 recovered fields)
/// Inherits: IntrusiveRefCounter::fnd::bl
#[derive(Debug, Clone, Default)]
pub struct ccMessageBox_system_gu {
    pub vtable_rva: u32,
    pub field_0x0000: u64,
    pub field_0x0001: u8,
    pub field_0x0010: u32,
    pub field_0x0078: u64,
    pub field_0x008e: u32,
    pub field_0x0190: u32,
}

impl ccMessageBox_system_gu {
    pub const CLASS_NAME: &'static str = "ccMessageBox::system::gu";
    pub const VTABLE_RVA: u32 = 0x00865c68;
    pub const METHOD_COUNT: usize = 5;
    pub const BASE_CLASSES: &'static [&'static str] = &["IntrusiveRefCounter::fnd::bl"];
    pub const VIRTUAL_METHODS: &'static [u64] = &[0x00000001800e1b60, 0x00000001800e1c80, 0x00000001800e1f60, 0x00000001800e20b0, 0x00000001800d3090];

    pub fn new() -> Self {
        Self {
            vtable_rva: Self::VTABLE_RVA,
            field_0x0000: Default::default(),
            field_0x0001: Default::default(),
            field_0x0010: Default::default(),
            field_0x0078: Default::default(),
            field_0x008e: Default::default(),
            field_0x0190: Default::default(),
        }
    }

    /// Decompiled virtual method #0 @ RVA 0x000e1b60 (13 instructions, 3 basic blocks)
    pub fn method_00(&mut self) {
        // Writes to field at offset +0x0000
    }

    /// Decompiled virtual method #1 @ RVA 0x000e1c80 (55 instructions, 13 basic blocks)
    pub fn method_01(&mut self) {
        // Writes to field at offset +0x0078
        // Writes to field at offset +0x0190
    }

    /// Decompiled virtual method #2 @ RVA 0x000e1f60 (25 instructions, 6 basic blocks)
    pub fn method_02(&mut self) {
    }

    /// Decompiled virtual method #3 @ RVA 0x000e20b0 (107 instructions, 9 basic blocks)
    pub fn method_03(&mut self) {
        // Writes to field at offset +0x0010
    }

    /// Decompiled virtual method #4 @ RVA 0x000d3090 (1 instructions, 1 basic blocks)
    pub fn method_04(&mut self) {
    }

}

impl CcClass for ccMessageBox_system_gu {
    fn class_name(&self) -> &'static str { Self::CLASS_NAME }
    fn vtable_rva(&self) -> u32 { Self::VTABLE_RVA }
    fn virtual_method_count(&self) -> usize { Self::METHOD_COUNT }
    fn base_classes(&self) -> &'static [&'static str] { Self::BASE_CLASSES }
}

/// Decompiled CC2 class: `ccMessageManager::gu` (VTable RVA: 0x00874cd8, 1 virtual methods, 0 recovered fields)
/// Inherits: ?$SingletonPtr::VccMessageManager::gu::::::fnd::bl, Runnable::thread::bl, CommonResourceEventHandler::gu, ?$IEventListener::VCommonResourceEventHandler::gu::::::util::bl
#[derive(Debug, Clone, Default)]
pub struct ccMessageManager_gu {
    pub vtable_rva: u32,
}

impl ccMessageManager_gu {
    pub const CLASS_NAME: &'static str = "ccMessageManager::gu";
    pub const VTABLE_RVA: u32 = 0x00874cd8;
    pub const METHOD_COUNT: usize = 1;
    pub const BASE_CLASSES: &'static [&'static str] = &["?$SingletonPtr::VccMessageManager::gu::::::fnd::bl", "Runnable::thread::bl", "CommonResourceEventHandler::gu", "?$IEventListener::VCommonResourceEventHandler::gu::::::util::bl"];
    pub const VIRTUAL_METHODS: &'static [u64] = &[0x000000018012cde0];

    pub fn new() -> Self {
        Self {
            vtable_rva: Self::VTABLE_RVA,
        }
    }

    /// Decompiled virtual method #0 @ RVA 0x0012cde0 (16 instructions, 3 basic blocks)
    pub fn method_00(&mut self) {
    }

}

impl CcClass for ccMessageManager_gu {
    fn class_name(&self) -> &'static str { Self::CLASS_NAME }
    fn vtable_rva(&self) -> u32 { Self::VTABLE_RVA }
    fn virtual_method_count(&self) -> usize { Self::METHOD_COUNT }
    fn base_classes(&self) -> &'static [&'static str] { Self::BASE_CLASSES }
}

/// Decompiled CC2 class: `ccModifier` (VTable RVA: 0x00876ea8, 2 virtual methods, 1 recovered fields)
#[derive(Debug, Clone, Default)]
pub struct ccModifier {
    pub vtable_rva: u32,
    pub field_0x0000: u64,
}

impl ccModifier {
    pub const CLASS_NAME: &'static str = "ccModifier";
    pub const VTABLE_RVA: u32 = 0x00876ea8;
    pub const METHOD_COUNT: usize = 2;
    pub const BASE_CLASSES: &'static [&'static str] = &[];
    pub const VIRTUAL_METHODS: &'static [u64] = &[0x0000000180158400, 0x00000001800d3090];

    pub fn new() -> Self {
        Self {
            vtable_rva: Self::VTABLE_RVA,
            field_0x0000: Default::default(),
        }
    }

    /// Decompiled virtual method #0 @ RVA 0x00158400 (13 instructions, 3 basic blocks)
    pub fn method_00(&mut self) {
        // Writes to field at offset +0x0000
    }

    /// Decompiled virtual method #1 @ RVA 0x000d3090 (1 instructions, 1 basic blocks)
    pub fn method_01(&mut self) {
    }

}

impl CcClass for ccModifier {
    fn class_name(&self) -> &'static str { Self::CLASS_NAME }
    fn vtable_rva(&self) -> u32 { Self::VTABLE_RVA }
    fn virtual_method_count(&self) -> usize { Self::METHOD_COUNT }
    fn base_classes(&self) -> &'static [&'static str] { Self::BASE_CLASSES }
}

/// Decompiled CC2 class: `ccMorpher` (VTable RVA: 0x00876fa0, 2 virtual methods, 1 recovered fields)
/// Inherits: ccModifier
#[derive(Debug, Clone, Default)]
pub struct ccMorpher {
    pub vtable_rva: u32,
    pub field_0x0000: u64,
}

impl ccMorpher {
    pub const CLASS_NAME: &'static str = "ccMorpher";
    pub const VTABLE_RVA: u32 = 0x00876fa0;
    pub const METHOD_COUNT: usize = 2;
    pub const BASE_CLASSES: &'static [&'static str] = &["ccModifier"];
    pub const VIRTUAL_METHODS: &'static [u64] = &[0x0000000180160810, 0x000000018014eee0];

    pub fn new() -> Self {
        Self {
            vtable_rva: Self::VTABLE_RVA,
            field_0x0000: Default::default(),
        }
    }

    /// Decompiled virtual method #0 @ RVA 0x00160810 (13 instructions, 3 basic blocks)
    pub fn method_00(&mut self) {
        // Writes to field at offset +0x0000
    }

    /// Decompiled virtual method #1 @ RVA 0x0014eee0 (103 instructions, 6 basic blocks)
    pub fn method_01(&mut self) {
    }

}

impl CcClass for ccMorpher {
    fn class_name(&self) -> &'static str { Self::CLASS_NAME }
    fn vtable_rva(&self) -> u32 { Self::VTABLE_RVA }
    fn virtual_method_count(&self) -> usize { Self::METHOD_COUNT }
    fn base_classes(&self) -> &'static [&'static str] { Self::BASE_CLASSES }
}

/// Decompiled CC2 class: `ccMoveLine` (VTable RVA: 0x008fb478, 4 virtual methods, 11 recovered fields)
#[derive(Debug, Clone, Default)]
pub struct ccMoveLine {
    pub vtable_rva: u32,
    pub field_0x0000: u64,
    pub field_0x0008: u64,
    pub field_0x0010: u32,
    pub field_0x0014: u32,
    pub field_0x0018: u64,
    pub field_0x0020: u32,
    pub field_0x0024: u32,
    pub field_0x0028: u32,
    pub field_0x002c: u32,
    pub field_0x0034: u32,
    pub field_0x0038: u32,
}

impl ccMoveLine {
    pub const CLASS_NAME: &'static str = "ccMoveLine";
    pub const VTABLE_RVA: u32 = 0x008fb478;
    pub const METHOD_COUNT: usize = 4;
    pub const BASE_CLASSES: &'static [&'static str] = &[];
    pub const VIRTUAL_METHODS: &'static [u64] = &[0x00000001800d3090, 0x00000001800d3090, 0x0000000180670540, 0x00000001800d3090];

    pub fn new() -> Self {
        Self {
            vtable_rva: Self::VTABLE_RVA,
            field_0x0000: Default::default(),
            field_0x0008: Default::default(),
            field_0x0010: Default::default(),
            field_0x0014: Default::default(),
            field_0x0018: Default::default(),
            field_0x0020: Default::default(),
            field_0x0024: Default::default(),
            field_0x0028: Default::default(),
            field_0x002c: Default::default(),
            field_0x0034: Default::default(),
            field_0x0038: Default::default(),
        }
    }

    /// Decompiled virtual method #0 @ RVA 0x000d3090 (1 instructions, 1 basic blocks)
    pub fn method_00(&mut self) {
    }

    /// Decompiled virtual method #1 @ RVA 0x000d3090 (1 instructions, 1 basic blocks)
    pub fn method_01(&mut self) {
    }

    /// Decompiled virtual method #2 @ RVA 0x00670540 (108 instructions, 9 basic blocks)
    pub fn method_02(&mut self) {
        // Writes to field at offset +0x0000
        // Writes to field at offset +0x0008
        // Writes to field at offset +0x0010
        // Writes to field at offset +0x0014
        // Writes to field at offset +0x0018
        // Writes to field at offset +0x0020
        // Writes to field at offset +0x0024
        // Writes to field at offset +0x0028
        // Writes to field at offset +0x002c
        // Writes to field at offset +0x0034
        // Writes to field at offset +0x0038
    }

    /// Decompiled virtual method #3 @ RVA 0x000d3090 (1 instructions, 1 basic blocks)
    pub fn method_03(&mut self) {
    }

}

impl CcClass for ccMoveLine {
    fn class_name(&self) -> &'static str { Self::CLASS_NAME }
    fn vtable_rva(&self) -> u32 { Self::VTABLE_RVA }
    fn virtual_method_count(&self) -> usize { Self::METHOD_COUNT }
    fn base_classes(&self) -> &'static [&'static str] { Self::BASE_CLASSES }
}

/// Decompiled CC2 class: `ccMultiFaceNamePanel` (VTable RVA: 0x008a4878, 21 virtual methods, 18 recovered fields)
/// Inherits: ccStatusPanelBase, ccDispTarget, ccDispBase
#[derive(Debug, Clone, Default)]
pub struct ccMultiFaceNamePanel {
    pub vtable_rva: u32,
    pub field_0x0000: u64,
    pub field_0x0008: u32,
    pub field_0x001a: u8,
    pub field_0x0020: u32,
    pub field_0x0028: u32,
    pub field_0x002c: u32,
    pub field_0x0030: u32,
    pub field_0x0034: u32,
    pub field_0x0048: u64,
    pub field_0x0054: u32,
    pub field_0x0058: u32,
    pub field_0x005c: u32,
    pub field_0x0060: u64,
    pub field_0x0080: u32,
    pub field_0x0084: u32,
    pub field_0x008c: u16,
    pub field_0x0098: u32,
    pub field_0x00b3: u8,
}

impl ccMultiFaceNamePanel {
    pub const CLASS_NAME: &'static str = "ccMultiFaceNamePanel";
    pub const VTABLE_RVA: u32 = 0x008a4878;
    pub const METHOD_COUNT: usize = 21;
    pub const BASE_CLASSES: &'static [&'static str] = &["ccStatusPanelBase", "ccDispTarget", "ccDispBase"];
    pub const VIRTUAL_METHODS: &'static [u64] = &[0x00000001802b2230, 0x00000001802b2340, 0x000000018029ff50, 0x00000001802b2610, 0x00000001802a0e30, 0x00000001800d3090, 0x00000001802a6510, 0x00000001801b8200, 0x00000001801b8210, 0x00000001802b2b80, 0x00000001800d3090, 0x00000001802a63e0, 0x00000001800d3090, 0x00000001802a6470, 0x00000001802b2b40, 0x00000001802a00d0, 0x0000000180103e70, 0x00000001802a0040, 0x00000001802a7640, 0x00000001802a76c0, 0x00000001802a7740];

    pub fn new() -> Self {
        Self {
            vtable_rva: Self::VTABLE_RVA,
            field_0x0000: Default::default(),
            field_0x0008: Default::default(),
            field_0x001a: Default::default(),
            field_0x0020: Default::default(),
            field_0x0028: Default::default(),
            field_0x002c: Default::default(),
            field_0x0030: Default::default(),
            field_0x0034: Default::default(),
            field_0x0048: Default::default(),
            field_0x0054: Default::default(),
            field_0x0058: Default::default(),
            field_0x005c: Default::default(),
            field_0x0060: Default::default(),
            field_0x0080: Default::default(),
            field_0x0084: Default::default(),
            field_0x008c: Default::default(),
            field_0x0098: Default::default(),
            field_0x00b3: Default::default(),
        }
    }

    /// Decompiled virtual method #0 @ RVA 0x002b2230 (16 instructions, 3 basic blocks)
    pub fn method_00(&mut self) {
    }

    /// Decompiled virtual method #1 @ RVA 0x002b2340 (118 instructions, 13 basic blocks)
    pub fn method_01(&mut self) {
        // Writes to field at offset +0x0054
        // Writes to field at offset +0x0058
        // Writes to field at offset +0x005c
    }

    /// Decompiled virtual method #2 @ RVA 0x0029ff50 (66 instructions, 11 basic blocks)
    pub fn method_02(&mut self) {
    }

    /// Decompiled virtual method #3 @ RVA 0x002b2610 (106 instructions, 15 basic blocks)
    pub fn method_03(&mut self) {
        // Writes to field at offset +0x0008
        // Writes to field at offset +0x0020
        // Writes to field at offset +0x0048
    }

    /// Decompiled virtual method #4 @ RVA 0x002a0e30 (48 instructions, 11 basic blocks)
    pub fn method_04(&mut self) {
        // Writes to field at offset +0x0008
    }

    /// Decompiled virtual method #5 @ RVA 0x000d3090 (1 instructions, 1 basic blocks)
    pub fn method_05(&mut self) {
    }

    /// Decompiled virtual method #6 @ RVA 0x002a6510 (10 instructions, 5 basic blocks)
    pub fn method_06(&mut self) {
        // Writes to field at offset +0x0008
        // Writes to field at offset +0x0084
    }

    /// Decompiled virtual method #7 @ RVA 0x001b8200 (5 instructions, 1 basic blocks)
    pub fn method_07(&mut self) {
    }

    /// Decompiled virtual method #8 @ RVA 0x001b8210 (5 instructions, 1 basic blocks)
    pub fn method_08(&mut self) {
    }

    /// Decompiled virtual method #9 @ RVA 0x002b2b80 (25 instructions, 5 basic blocks)
    pub fn method_09(&mut self) {
    }

    /// Decompiled virtual method #10 @ RVA 0x000d3090 (1 instructions, 1 basic blocks)
    pub fn method_10(&mut self) {
    }

    /// Decompiled virtual method #11 @ RVA 0x002a63e0 (11 instructions, 2 basic blocks)
    pub fn method_11(&mut self) {
        // Writes to field at offset +0x0084
    }

    /// Decompiled virtual method #12 @ RVA 0x000d3090 (1 instructions, 1 basic blocks)
    pub fn method_12(&mut self) {
    }

    /// Decompiled virtual method #13 @ RVA 0x002a6470 (11 instructions, 2 basic blocks)
    pub fn method_13(&mut self) {
        // Writes to field at offset +0x0084
    }

    /// Decompiled virtual method #14 @ RVA 0x002b2b40 (13 instructions, 3 basic blocks)
    pub fn method_14(&mut self) {
        // Writes to field at offset +0x008c
        // Writes to field at offset +0x00b3
    }

    /// Decompiled virtual method #15 @ RVA 0x002a00d0 (2 instructions, 1 basic blocks)
    pub fn method_15(&mut self) {
    }

    /// Decompiled virtual method #16 @ RVA 0x00103e70 (2 instructions, 1 basic blocks)
    pub fn method_16(&mut self) {
    }

    /// Decompiled virtual method #17 @ RVA 0x002a0040 (3 instructions, 1 basic blocks)
    pub fn method_17(&mut self) {
    }

    /// Decompiled virtual method #18 @ RVA 0x002a7640 (28 instructions, 3 basic blocks)
    pub fn method_18(&mut self) {
        // Writes to field at offset +0x001a
    }

    /// Decompiled virtual method #19 @ RVA 0x002a76c0 (25 instructions, 5 basic blocks)
    pub fn method_19(&mut self) {
    }

    /// Decompiled virtual method #20 @ RVA 0x002a7740 (26 instructions, 5 basic blocks)
    pub fn method_20(&mut self) {
    }

}

impl CcClass for ccMultiFaceNamePanel {
    fn class_name(&self) -> &'static str { Self::CLASS_NAME }
    fn vtable_rva(&self) -> u32 { Self::VTABLE_RVA }
    fn virtual_method_count(&self) -> usize { Self::METHOD_COUNT }
    fn base_classes(&self) -> &'static [&'static str] { Self::BASE_CLASSES }
}

/// Decompiled CC2 class: `ccNumber` (VTable RVA: 0x008721f8, 10 virtual methods, 11 recovered fields)
#[derive(Debug, Clone, Default)]
pub struct ccNumber {
    pub vtable_rva: u32,
    pub field_0x0000: u64,
    pub field_0x0008: u32,
    pub field_0x000c: u32,
    pub field_0x0010: u32,
    pub field_0x0014: f32,
    pub field_0x0018: f32,
    pub field_0x001c: f32,
    pub field_0x0020: u16,
    pub field_0x0022: u16,
    pub field_0x0024: u32,
    pub field_0x0028: u64,
}

impl ccNumber {
    pub const CLASS_NAME: &'static str = "ccNumber";
    pub const VTABLE_RVA: u32 = 0x008721f8;
    pub const METHOD_COUNT: usize = 10;
    pub const BASE_CLASSES: &'static [&'static str] = &[];
    pub const VIRTUAL_METHODS: &'static [u64] = &[0x0000000180114c80, 0x00000001802a5a80, 0x00000001802a5b20, 0x00000001802a5b70, 0x00000001802a5c70, 0x00000001802a5cf0, 0x00000001802a5d00, 0x0000000180114c60, 0x0000000180114c70, 0x00000001802a5c40];

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
        }
    }

    /// Decompiled virtual method #0 @ RVA 0x00114c80 (35 instructions, 7 basic blocks)
    pub fn method_00(&mut self) {
        // Writes to field at offset +0x0000
    }

    /// Decompiled virtual method #1 @ RVA 0x002a5a80 (69 instructions, 5 basic blocks)
    pub fn method_01(&mut self) {
        // Writes to field at offset +0x0008
        // Writes to field at offset +0x000c
        // Writes to field at offset +0x0010
        // Writes to field at offset +0x0022
        // Writes to field at offset +0x0024
        // Writes to field at offset +0x0028
    }

    /// Decompiled virtual method #2 @ RVA 0x002a5b20 (19 instructions, 5 basic blocks)
    pub fn method_02(&mut self) {
    }

    /// Decompiled virtual method #3 @ RVA 0x002a5b70 (54 instructions, 8 basic blocks)
    pub fn method_03(&mut self) {
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

    /// Decompiled virtual method #8 @ RVA 0x00114c70 (3 instructions, 1 basic blocks)
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

impl CcClass for ccNumber {
    fn class_name(&self) -> &'static str { Self::CLASS_NAME }
    fn vtable_rva(&self) -> u32 { Self::VTABLE_RVA }
    fn virtual_method_count(&self) -> usize { Self::METHOD_COUNT }
    fn base_classes(&self) -> &'static [&'static str] { Self::BASE_CLASSES }
}

/// Decompiled CC2 class: `ccOJBtn` (VTable RVA: 0x008fb3c8, 9 virtual methods, 18 recovered fields)
/// Inherits: ccWebButtonBase
#[derive(Debug, Clone, Default)]
pub struct ccOJBtn {
    pub vtable_rva: u32,
    pub field_0x0000: u64,
    pub field_0x0008: u64,
    pub field_0x0040: u32,
    pub field_0x0044: f32,
    pub field_0x0048: u32,
    pub field_0x009c: u32,
    pub field_0x00a0: u32,
    pub field_0x00a4: u32,
    pub field_0x00a8: u32,
    pub field_0x00c4: u32,
    pub field_0x00cc: u32,
    pub field_0x00d0: u32,
    pub field_0x00d4: u32,
    pub field_0x00d8: u8,
    pub field_0x00e0: u64,
    pub field_0x0130: u64,
    pub field_0x013c: u32,
    pub field_0x0142: u16,
}

impl ccOJBtn {
    pub const CLASS_NAME: &'static str = "ccOJBtn";
    pub const VTABLE_RVA: u32 = 0x008fb3c8;
    pub const METHOD_COUNT: usize = 9;
    pub const BASE_CLASSES: &'static [&'static str] = &["ccWebButtonBase"];
    pub const VIRTUAL_METHODS: &'static [u64] = &[0x000000018066e8c0, 0x00000001800d3090, 0x00000001800d3090, 0x0000000180667a30, 0x0000000180667f40, 0x0000000180667f60, 0x000000018066f4b0, 0x0000000180667cd0, 0x000000018066ee90];

    pub fn new() -> Self {
        Self {
            vtable_rva: Self::VTABLE_RVA,
            field_0x0000: Default::default(),
            field_0x0008: Default::default(),
            field_0x0040: Default::default(),
            field_0x0044: Default::default(),
            field_0x0048: Default::default(),
            field_0x009c: Default::default(),
            field_0x00a0: Default::default(),
            field_0x00a4: Default::default(),
            field_0x00a8: Default::default(),
            field_0x00c4: Default::default(),
            field_0x00cc: Default::default(),
            field_0x00d0: Default::default(),
            field_0x00d4: Default::default(),
            field_0x00d8: Default::default(),
            field_0x00e0: Default::default(),
            field_0x0130: Default::default(),
            field_0x013c: Default::default(),
            field_0x0142: Default::default(),
        }
    }

    /// Decompiled virtual method #0 @ RVA 0x0066e8c0 (126 instructions, 17 basic blocks)
    pub fn method_00(&mut self) {
        // Writes to field at offset +0x0142
    }

    /// Decompiled virtual method #1 @ RVA 0x000d3090 (1 instructions, 1 basic blocks)
    pub fn method_01(&mut self) {
    }

    /// Decompiled virtual method #2 @ RVA 0x000d3090 (1 instructions, 1 basic blocks)
    pub fn method_02(&mut self) {
    }

    /// Decompiled virtual method #3 @ RVA 0x00667a30 (16 instructions, 3 basic blocks)
    pub fn method_03(&mut self) {
    }

    /// Decompiled virtual method #4 @ RVA 0x00667f40 (3 instructions, 1 basic blocks)
    pub fn method_04(&mut self) {
        // Writes to field at offset +0x00c4
        // Writes to field at offset +0x00d8
    }

    /// Decompiled virtual method #5 @ RVA 0x00667f60 (2 instructions, 1 basic blocks)
    pub fn method_05(&mut self) {
        // Writes to field at offset +0x00d8
    }

    /// Decompiled virtual method #6 @ RVA 0x0066f4b0 (11 instructions, 1 basic blocks)
    pub fn method_06(&mut self) {
    }

    /// Decompiled virtual method #7 @ RVA 0x00667cd0 (118 instructions, 18 basic blocks)
    pub fn method_07(&mut self) {
        // Writes to field at offset +0x0040
        // Writes to field at offset +0x0044
        // Writes to field at offset +0x0048
        // Writes to field at offset +0x00c4
        // Writes to field at offset +0x00c4
        // Writes to field at offset +0x00cc
    }

    /// Decompiled virtual method #8 @ RVA 0x0066ee90 (97 instructions, 3 basic blocks)
    pub fn method_08(&mut self) {
    }

}

impl CcClass for ccOJBtn {
    fn class_name(&self) -> &'static str { Self::CLASS_NAME }
    fn vtable_rva(&self) -> u32 { Self::VTABLE_RVA }
    fn virtual_method_count(&self) -> usize { Self::METHOD_COUNT }
    fn base_classes(&self) -> &'static [&'static str] { Self::BASE_CLASSES }
}

/// Decompiled CC2 class: `ccOJWeb` (VTable RVA: 0x008fb418, 11 virtual methods, 10 recovered fields)
/// Inherits: ccWebBase
#[derive(Debug, Clone, Default)]
pub struct ccOJWeb {
    pub vtable_rva: u32,
    pub field_0x0000: u64,
    pub field_0x0010: u64,
    pub field_0x0068: u64,
    pub field_0x0070: u64,
    pub field_0x0078: u64,
    pub field_0x0098: u32,
    pub field_0x009c: u32,
    pub field_0x0360: u32,
    pub field_0x0368: u8,
    pub field_0x0398: [f32; 4],
}

impl ccOJWeb {
    pub const CLASS_NAME: &'static str = "ccOJWeb";
    pub const VTABLE_RVA: u32 = 0x008fb418;
    pub const METHOD_COUNT: usize = 11;
    pub const BASE_CLASSES: &'static [&'static str] = &["ccWebBase"];
    pub const VIRTUAL_METHODS: &'static [u64] = &[0x000000018066d5d0, 0x0000000180103e90, 0x000000018066e360, 0x00000001800d3090, 0x0000000180667970, 0x00000001800d3090, 0x000000018066d530, 0x000000018066d570, 0x000000018066db00, 0x000000018066e270, 0x0000000180668d80];

    pub fn new() -> Self {
        Self {
            vtable_rva: Self::VTABLE_RVA,
            field_0x0000: Default::default(),
            field_0x0010: Default::default(),
            field_0x0068: Default::default(),
            field_0x0070: Default::default(),
            field_0x0078: Default::default(),
            field_0x0098: Default::default(),
            field_0x009c: Default::default(),
            field_0x0360: Default::default(),
            field_0x0368: Default::default(),
            field_0x0398: Default::default(),
        }
    }

    /// Decompiled virtual method #0 @ RVA 0x0066d5d0 (116 instructions, 13 basic blocks)
    pub fn method_00(&mut self) {
    }

    /// Decompiled virtual method #1 @ RVA 0x00103e90 (2 instructions, 1 basic blocks)
    pub fn method_01(&mut self) {
    }

    /// Decompiled virtual method #2 @ RVA 0x0066e360 (92 instructions, 1 basic blocks)
    pub fn method_02(&mut self) {
    }

    /// Decompiled virtual method #3 @ RVA 0x000d3090 (1 instructions, 1 basic blocks)
    pub fn method_03(&mut self) {
    }

    /// Decompiled virtual method #4 @ RVA 0x00667970 (24 instructions, 3 basic blocks)
    pub fn method_04(&mut self) {
        // Writes to field at offset +0x0000
    }

    /// Decompiled virtual method #5 @ RVA 0x000d3090 (1 instructions, 1 basic blocks)
    pub fn method_05(&mut self) {
    }

    /// Decompiled virtual method #6 @ RVA 0x0066d530 (14 instructions, 3 basic blocks)
    pub fn method_06(&mut self) {
    }

    /// Decompiled virtual method #7 @ RVA 0x0066d570 (17 instructions, 3 basic blocks)
    pub fn method_07(&mut self) {
    }

    /// Decompiled virtual method #8 @ RVA 0x0066db00 (99 instructions, 15 basic blocks)
    pub fn method_08(&mut self) {
        // Writes to field at offset +0x009c
    }

    /// Decompiled virtual method #9 @ RVA 0x0066e270 (53 instructions, 11 basic blocks)
    pub fn method_09(&mut self) {
    }

    /// Decompiled virtual method #10 @ RVA 0x00668d80 (99 instructions, 5 basic blocks)
    pub fn method_10(&mut self) {
        // Writes to field at offset +0x0360
        // Writes to field at offset +0x0368
        // Writes to field at offset +0x0398
    }

}

impl CcClass for ccOJWeb {
    fn class_name(&self) -> &'static str { Self::CLASS_NAME }
    fn vtable_rva(&self) -> u32 { Self::VTABLE_RVA }
    fn virtual_method_count(&self) -> usize { Self::METHOD_COUNT }
    fn base_classes(&self) -> &'static [&'static str] { Self::BASE_CLASSES }
}

/// Decompiled CC2 class: `ccObj` (VTable RVA: 0x00866050, 1 virtual methods, 0 recovered fields)
/// Inherits: ccCoord
#[derive(Debug, Clone, Default)]
pub struct ccObj {
    pub vtable_rva: u32,
}

impl ccObj {
    pub const CLASS_NAME: &'static str = "ccObj";
    pub const VTABLE_RVA: u32 = 0x00866050;
    pub const METHOD_COUNT: usize = 1;
    pub const BASE_CLASSES: &'static [&'static str] = &["ccCoord"];
    pub const VIRTUAL_METHODS: &'static [u64] = &[0x00000001800eb620];

    pub fn new() -> Self {
        Self {
            vtable_rva: Self::VTABLE_RVA,
        }
    }

    /// Decompiled virtual method #0 @ RVA 0x000eb620 (16 instructions, 3 basic blocks)
    pub fn method_00(&mut self) {
    }

}

impl CcClass for ccObj {
    fn class_name(&self) -> &'static str { Self::CLASS_NAME }
    fn vtable_rva(&self) -> u32 { Self::VTABLE_RVA }
    fn virtual_method_count(&self) -> usize { Self::METHOD_COUNT }
    fn base_classes(&self) -> &'static [&'static str] { Self::BASE_CLASSES }
}

/// Decompiled CC2 class: `ccOffSiteWeb` (VTable RVA: 0x008c4810, 11 virtual methods, 11 recovered fields)
/// Inherits: ccWebBase
#[derive(Debug, Clone, Default)]
pub struct ccOffSiteWeb {
    pub vtable_rva: u32,
    pub field_0x0000: u8,
    pub field_0x0001: u32,
    pub field_0x0008: u64,
    pub field_0x0068: u64,
    pub field_0x0070: u64,
    pub field_0x0078: u64,
    pub field_0x008c: u8,
    pub field_0x0108: u8,
    pub field_0x0360: u32,
    pub field_0x0368: u8,
    pub field_0x0398: [f32; 4],
}

impl ccOffSiteWeb {
    pub const CLASS_NAME: &'static str = "ccOffSiteWeb";
    pub const VTABLE_RVA: u32 = 0x008c4810;
    pub const METHOD_COUNT: usize = 11;
    pub const BASE_CLASSES: &'static [&'static str] = &["ccWebBase"];
    pub const VIRTUAL_METHODS: &'static [u64] = &[0x000000018066bfd0, 0x0000000180103e90, 0x000000018066ca20, 0x000000018066cc60, 0x00000001805658b0, 0x00000001800d3090, 0x000000018066be80, 0x000000018066bed0, 0x000000018066c380, 0x0000000180668b10, 0x0000000180668d80];

    pub fn new() -> Self {
        Self {
            vtable_rva: Self::VTABLE_RVA,
            field_0x0000: Default::default(),
            field_0x0001: Default::default(),
            field_0x0008: Default::default(),
            field_0x0068: Default::default(),
            field_0x0070: Default::default(),
            field_0x0078: Default::default(),
            field_0x008c: Default::default(),
            field_0x0108: Default::default(),
            field_0x0360: Default::default(),
            field_0x0368: Default::default(),
            field_0x0398: Default::default(),
        }
    }

    /// Decompiled virtual method #0 @ RVA 0x0066bfd0 (121 instructions, 17 basic blocks)
    pub fn method_00(&mut self) {
    }

    /// Decompiled virtual method #1 @ RVA 0x00103e90 (2 instructions, 1 basic blocks)
    pub fn method_01(&mut self) {
    }

    /// Decompiled virtual method #2 @ RVA 0x0066ca20 (97 instructions, 19 basic blocks)
    pub fn method_02(&mut self) {
        // Writes to field at offset +0x008c
        // Writes to field at offset +0x0108
    }

    /// Decompiled virtual method #3 @ RVA 0x0066cc60 (75 instructions, 6 basic blocks)
    pub fn method_03(&mut self) {
    }

    /// Decompiled virtual method #4 @ RVA 0x005658b0 (24 instructions, 3 basic blocks)
    pub fn method_04(&mut self) {
        // Writes to field at offset +0x0000
    }

    /// Decompiled virtual method #5 @ RVA 0x000d3090 (1 instructions, 1 basic blocks)
    pub fn method_05(&mut self) {
    }

    /// Decompiled virtual method #6 @ RVA 0x0066be80 (29 instructions, 3 basic blocks)
    pub fn method_06(&mut self) {
    }

    /// Decompiled virtual method #7 @ RVA 0x0066bed0 (7 instructions, 2 basic blocks)
    pub fn method_07(&mut self) {
    }

    /// Decompiled virtual method #8 @ RVA 0x0066c380 (114 instructions, 21 basic blocks)
    pub fn method_08(&mut self) {
    }

    /// Decompiled virtual method #9 @ RVA 0x00668b10 (114 instructions, 9 basic blocks)
    pub fn method_09(&mut self) {
        // Writes to field at offset +0x0360
        // Writes to field at offset +0x0368
        // Writes to field at offset +0x0398
    }

    /// Decompiled virtual method #10 @ RVA 0x00668d80 (99 instructions, 5 basic blocks)
    pub fn method_10(&mut self) {
        // Writes to field at offset +0x0360
        // Writes to field at offset +0x0368
        // Writes to field at offset +0x0398
    }

}

impl CcClass for ccOffSiteWeb {
    fn class_name(&self) -> &'static str { Self::CLASS_NAME }
    fn vtable_rva(&self) -> u32 { Self::VTABLE_RVA }
    fn virtual_method_count(&self) -> usize { Self::METHOD_COUNT }
    fn base_classes(&self) -> &'static [&'static str] { Self::BASE_CLASSES }
}

/// Decompiled CC2 class: `ccOfficialBBS` (VTable RVA: 0x008c4760, 10 virtual methods, 4 recovered fields)
/// Inherits: ccOfficialBase
#[derive(Debug, Clone, Default)]
pub struct ccOfficialBBS {
    pub vtable_rva: u32,
    pub field_0x0000: u64,
    pub field_0x0020: u32,
    pub field_0x0030: u32,
    pub field_0x0050: u64,
}

impl ccOfficialBBS {
    pub const CLASS_NAME: &'static str = "ccOfficialBBS";
    pub const VTABLE_RVA: u32 = 0x008c4760;
    pub const METHOD_COUNT: usize = 10;
    pub const BASE_CLASSES: &'static [&'static str] = &["ccOfficialBase"];
    pub const VIRTUAL_METHODS: &'static [u64] = &[0x0000000180569280, 0x0000000180569430, 0x00000001800d3090, 0x00000001800d3090, 0x0000000180569490, 0x00000001800fe200, 0x00000001805694f0, 0x0000000180569360, 0x0000000180569420, 0x00000001805659c0];

    pub fn new() -> Self {
        Self {
            vtable_rva: Self::VTABLE_RVA,
            field_0x0000: Default::default(),
            field_0x0020: Default::default(),
            field_0x0030: Default::default(),
            field_0x0050: Default::default(),
        }
    }

    /// Decompiled virtual method #0 @ RVA 0x00569280 (53 instructions, 7 basic blocks)
    pub fn method_00(&mut self) {
    }

    /// Decompiled virtual method #1 @ RVA 0x00569430 (44 instructions, 5 basic blocks)
    pub fn method_01(&mut self) {
    }

    /// Decompiled virtual method #2 @ RVA 0x000d3090 (1 instructions, 1 basic blocks)
    pub fn method_02(&mut self) {
    }

    /// Decompiled virtual method #3 @ RVA 0x000d3090 (1 instructions, 1 basic blocks)
    pub fn method_03(&mut self) {
    }

    /// Decompiled virtual method #4 @ RVA 0x00569490 (17 instructions, 2 basic blocks)
    pub fn method_04(&mut self) {
    }

    /// Decompiled virtual method #5 @ RVA 0x000fe200 (2 instructions, 1 basic blocks)
    pub fn method_05(&mut self) {
    }

    /// Decompiled virtual method #6 @ RVA 0x005694f0 (6 instructions, 2 basic blocks)
    pub fn method_06(&mut self) {
        // Writes to field at offset +0x0020
    }

    /// Decompiled virtual method #7 @ RVA 0x00569360 (39 instructions, 4 basic blocks)
    pub fn method_07(&mut self) {
    }

    /// Decompiled virtual method #8 @ RVA 0x00569420 (53 instructions, 6 basic blocks)
    pub fn method_08(&mut self) {
    }

    /// Decompiled virtual method #9 @ RVA 0x005659c0 (30 instructions, 5 basic blocks)
    pub fn method_09(&mut self) {
        // Writes to field at offset +0x0000
    }

}

impl CcClass for ccOfficialBBS {
    fn class_name(&self) -> &'static str { Self::CLASS_NAME }
    fn vtable_rva(&self) -> u32 { Self::VTABLE_RVA }
    fn virtual_method_count(&self) -> usize { Self::METHOD_COUNT }
    fn base_classes(&self) -> &'static [&'static str] { Self::BASE_CLASSES }
}

/// Decompiled CC2 class: `ccOfficialBase` (VTable RVA: 0x008c4708, 10 virtual methods, 0 recovered fields)
#[derive(Debug, Clone, Default)]
pub struct ccOfficialBase {
    pub vtable_rva: u32,
}

impl ccOfficialBase {
    pub const CLASS_NAME: &'static str = "ccOfficialBase";
    pub const VTABLE_RVA: u32 = 0x008c4708;
    pub const METHOD_COUNT: usize = 10;
    pub const BASE_CLASSES: &'static [&'static str] = &[];
    pub const VIRTUAL_METHODS: &'static [u64] = &[0x00000001800d3090, 0x00000001800d3090, 0x00000001800d3090, 0x00000001800d3090, 0x00000001800d3090, 0x00000001800fe200, 0x00000001800d6920, 0x0000000180103e90, 0x00000001800d3090, 0x0000000180565980];

    pub fn new() -> Self {
        Self {
            vtable_rva: Self::VTABLE_RVA,
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

    /// Decompiled virtual method #4 @ RVA 0x000d3090 (1 instructions, 1 basic blocks)
    pub fn method_04(&mut self) {
    }

    /// Decompiled virtual method #5 @ RVA 0x000fe200 (2 instructions, 1 basic blocks)
    pub fn method_05(&mut self) {
    }

    /// Decompiled virtual method #6 @ RVA 0x000d6920 (2 instructions, 1 basic blocks)
    pub fn method_06(&mut self) {
    }

    /// Decompiled virtual method #7 @ RVA 0x00103e90 (2 instructions, 1 basic blocks)
    pub fn method_07(&mut self) {
    }

    /// Decompiled virtual method #8 @ RVA 0x000d3090 (1 instructions, 1 basic blocks)
    pub fn method_08(&mut self) {
    }

    /// Decompiled virtual method #9 @ RVA 0x00565980 (16 instructions, 3 basic blocks)
    pub fn method_09(&mut self) {
    }

}

impl CcClass for ccOfficialBase {
    fn class_name(&self) -> &'static str { Self::CLASS_NAME }
    fn vtable_rva(&self) -> u32 { Self::VTABLE_RVA }
    fn virtual_method_count(&self) -> usize { Self::METHOD_COUNT }
    fn base_classes(&self) -> &'static [&'static str] { Self::BASE_CLASSES }
}

/// Decompiled CC2 class: `ccOfficialSITE` (VTable RVA: 0x008c47b8, 10 virtual methods, 5 recovered fields)
/// Inherits: ccOfficialBase
#[derive(Debug, Clone, Default)]
pub struct ccOfficialSITE {
    pub vtable_rva: u32,
    pub field_0x0000: u64,
    pub field_0x0018: u64,
    pub field_0x0030: u32,
    pub field_0x0050: u64,
    pub field_0x008b: u8,
}

impl ccOfficialSITE {
    pub const CLASS_NAME: &'static str = "ccOfficialSITE";
    pub const VTABLE_RVA: u32 = 0x008c47b8;
    pub const METHOD_COUNT: usize = 10;
    pub const BASE_CLASSES: &'static [&'static str] = &["ccOfficialBase"];
    pub const VIRTUAL_METHODS: &'static [u64] = &[0x0000000180569520, 0x0000000180569860, 0x0000000180569920, 0x00000001800d3090, 0x0000000180569940, 0x00000001805699b0, 0x00000001800d6920, 0x0000000180569770, 0x0000000180569820, 0x0000000180565a40];

    pub fn new() -> Self {
        Self {
            vtable_rva: Self::VTABLE_RVA,
            field_0x0000: Default::default(),
            field_0x0018: Default::default(),
            field_0x0030: Default::default(),
            field_0x0050: Default::default(),
            field_0x008b: Default::default(),
        }
    }

    /// Decompiled virtual method #0 @ RVA 0x00569520 (132 instructions, 16 basic blocks)
    pub fn method_00(&mut self) {
    }

    /// Decompiled virtual method #1 @ RVA 0x00569860 (41 instructions, 5 basic blocks)
    pub fn method_01(&mut self) {
        // Writes to field at offset +0x008b
    }

    /// Decompiled virtual method #2 @ RVA 0x00569920 (36 instructions, 6 basic blocks)
    pub fn method_02(&mut self) {
    }

    /// Decompiled virtual method #3 @ RVA 0x000d3090 (1 instructions, 1 basic blocks)
    pub fn method_03(&mut self) {
    }

    /// Decompiled virtual method #4 @ RVA 0x00569940 (27 instructions, 5 basic blocks)
    pub fn method_04(&mut self) {
    }

    /// Decompiled virtual method #5 @ RVA 0x005699b0 (32 instructions, 6 basic blocks)
    pub fn method_05(&mut self) {
    }

    /// Decompiled virtual method #6 @ RVA 0x000d6920 (2 instructions, 1 basic blocks)
    pub fn method_06(&mut self) {
    }

    /// Decompiled virtual method #7 @ RVA 0x00569770 (116 instructions, 7 basic blocks)
    pub fn method_07(&mut self) {
        // Writes to field at offset +0x008b
    }

    /// Decompiled virtual method #8 @ RVA 0x00569820 (70 instructions, 5 basic blocks)
    pub fn method_08(&mut self) {
        // Writes to field at offset +0x008b
    }

    /// Decompiled virtual method #9 @ RVA 0x00565a40 (24 instructions, 3 basic blocks)
    pub fn method_09(&mut self) {
        // Writes to field at offset +0x0000
    }

}

impl CcClass for ccOfficialSITE {
    fn class_name(&self) -> &'static str { Self::CLASS_NAME }
    fn vtable_rva(&self) -> u32 { Self::VTABLE_RVA }
    fn virtual_method_count(&self) -> usize { Self::METHOD_COUNT }
    fn base_classes(&self) -> &'static [&'static str] { Self::BASE_CLASSES }
}

/// Decompiled CC2 class: `ccOmniLight` (VTable RVA: 0x00876f08, 2 virtual methods, 4 recovered fields)
/// Inherits: ccLight, ccCoord
#[derive(Debug, Clone, Default)]
pub struct ccOmniLight {
    pub vtable_rva: u32,
    pub field_0x0000: u64,
    pub field_0x00a1: u8,
    pub field_0x00d0: u64,
    pub field_0x00f0: u32,
}

impl ccOmniLight {
    pub const CLASS_NAME: &'static str = "ccOmniLight";
    pub const VTABLE_RVA: u32 = 0x00876f08;
    pub const METHOD_COUNT: usize = 2;
    pub const BASE_CLASSES: &'static [&'static str] = &["ccLight", "ccCoord"];
    pub const VIRTUAL_METHODS: &'static [u64] = &[0x00000001801587f0, 0x000000018014cb50];

    pub fn new() -> Self {
        Self {
            vtable_rva: Self::VTABLE_RVA,
            field_0x0000: Default::default(),
            field_0x00a1: Default::default(),
            field_0x00d0: Default::default(),
            field_0x00f0: Default::default(),
        }
    }

    /// Decompiled virtual method #0 @ RVA 0x001587f0 (22 instructions, 5 basic blocks)
    pub fn method_00(&mut self) {
        // Writes to field at offset +0x0000
    }

    /// Decompiled virtual method #1 @ RVA 0x0014cb50 (83 instructions, 11 basic blocks)
    pub fn method_01(&mut self) {
        // Writes to field at offset +0x00a1
    }

}

impl CcClass for ccOmniLight {
    fn class_name(&self) -> &'static str { Self::CLASS_NAME }
    fn vtable_rva(&self) -> u32 { Self::VTABLE_RVA }
    fn virtual_method_count(&self) -> usize { Self::METHOD_COUNT }
    fn base_classes(&self) -> &'static [&'static str] { Self::BASE_CLASSES }
}

/// Decompiled CC2 class: `ccPDEffectBlur` (VTable RVA: 0x008b6ec8, 3 virtual methods, 22 recovered fields)
/// Inherits: ccPDEffectBase
#[derive(Debug, Clone, Default)]
pub struct ccPDEffectBlur {
    pub vtable_rva: u32,
    pub field_0x0010: u16,
    pub field_0x0020: u64,
    pub field_0x0028: u32,
    pub field_0x002c: f32,
    pub field_0x0030: f32,
    pub field_0x0034: u32,
    pub field_0x0038: u32,
    pub field_0x003c: u8,
    pub field_0x0040: u32,
    pub field_0x0044: f32,
    pub field_0x004c: u32,
    pub field_0x0050: u8,
    pub field_0x0078: u32,
    pub field_0x007c: f32,
    pub field_0x0080: f32,
    pub field_0x0084: u32,
    pub field_0x0094: f32,
    pub field_0x00a0: f32,
    pub field_0x00a4: u8,
    pub field_0x00a8: u32,
    pub field_0x00ac: u32,
    pub field_0x00b0: u32,
}

impl ccPDEffectBlur {
    pub const CLASS_NAME: &'static str = "ccPDEffectBlur";
    pub const VTABLE_RVA: u32 = 0x008b6ec8;
    pub const METHOD_COUNT: usize = 3;
    pub const BASE_CLASSES: &'static [&'static str] = &["ccPDEffectBase"];
    pub const VIRTUAL_METHODS: &'static [u64] = &[0x0000000180384fd0, 0x0000000180384f00, 0x0000000180384da0];

    pub fn new() -> Self {
        Self {
            vtable_rva: Self::VTABLE_RVA,
            field_0x0010: Default::default(),
            field_0x0020: Default::default(),
            field_0x0028: Default::default(),
            field_0x002c: Default::default(),
            field_0x0030: Default::default(),
            field_0x0034: Default::default(),
            field_0x0038: Default::default(),
            field_0x003c: Default::default(),
            field_0x0040: Default::default(),
            field_0x0044: Default::default(),
            field_0x004c: Default::default(),
            field_0x0050: Default::default(),
            field_0x0078: Default::default(),
            field_0x007c: Default::default(),
            field_0x0080: Default::default(),
            field_0x0084: Default::default(),
            field_0x0094: Default::default(),
            field_0x00a0: Default::default(),
            field_0x00a4: Default::default(),
            field_0x00a8: Default::default(),
            field_0x00ac: Default::default(),
            field_0x00b0: Default::default(),
        }
    }

    /// Decompiled virtual method #0 @ RVA 0x00384fd0 (31 instructions, 7 basic blocks)
    pub fn method_00(&mut self) {
        // Writes to field at offset +0x00a4
    }

    /// Decompiled virtual method #1 @ RVA 0x00384f00 (36 instructions, 3 basic blocks)
    pub fn method_01(&mut self) {
        // Writes to field at offset +0x0010
        // Writes to field at offset +0x0020
        // Writes to field at offset +0x0028
        // Writes to field at offset +0x002c
        // Writes to field at offset +0x0030
        // Writes to field at offset +0x0034
        // Writes to field at offset +0x0034
        // Writes to field at offset +0x0038
        // Writes to field at offset +0x003c
        // Writes to field at offset +0x0040
        // Writes to field at offset +0x0044
        // Writes to field at offset +0x0044
        // Writes to field at offset +0x004c
        // Writes to field at offset +0x0050
        // Writes to field at offset +0x00a8
        // Writes to field at offset +0x00ac
        // Writes to field at offset +0x00b0
    }

    /// Decompiled virtual method #2 @ RVA 0x00384da0 (2 instructions, 1 basic blocks)
    pub fn method_02(&mut self) {
    }

}

impl CcClass for ccPDEffectBlur {
    fn class_name(&self) -> &'static str { Self::CLASS_NAME }
    fn vtable_rva(&self) -> u32 { Self::VTABLE_RVA }
    fn virtual_method_count(&self) -> usize { Self::METHOD_COUNT }
    fn base_classes(&self) -> &'static [&'static str] { Self::BASE_CLASSES }
}

/// Decompiled CC2 class: `ccPDEffectFocusline` (VTable RVA: 0x008e6fe8, 3 virtual methods, 3 recovered fields)
/// Inherits: ccPDEffectBase
#[derive(Debug, Clone, Default)]
pub struct ccPDEffectFocusline {
    pub vtable_rva: u32,
    pub field_0x0000: u32,
    pub field_0x0010: u32,
    pub field_0x0014: u32,
}

impl ccPDEffectFocusline {
    pub const CLASS_NAME: &'static str = "ccPDEffectFocusline";
    pub const VTABLE_RVA: u32 = 0x008e6fe8;
    pub const METHOD_COUNT: usize = 3;
    pub const BASE_CLASSES: &'static [&'static str] = &["ccPDEffectBase"];
    pub const VIRTUAL_METHODS: &'static [u64] = &[0x00000001805d08c0, 0x00000001805d0970, 0x0000000180385130];

    pub fn new() -> Self {
        Self {
            vtable_rva: Self::VTABLE_RVA,
            field_0x0000: Default::default(),
            field_0x0010: Default::default(),
            field_0x0014: Default::default(),
        }
    }

    /// Decompiled virtual method #0 @ RVA 0x005d08c0 (43 instructions, 7 basic blocks)
    pub fn method_00(&mut self) {
    }

    /// Decompiled virtual method #1 @ RVA 0x005d0970 (38 instructions, 6 basic blocks)
    pub fn method_01(&mut self) {
        // Writes to field at offset +0x0000
        // Writes to field at offset +0x0010
        // Writes to field at offset +0x0014
    }

    /// Decompiled virtual method #2 @ RVA 0x00385130 (2 instructions, 1 basic blocks)
    pub fn method_02(&mut self) {
    }

}

impl CcClass for ccPDEffectFocusline {
    fn class_name(&self) -> &'static str { Self::CLASS_NAME }
    fn vtable_rva(&self) -> u32 { Self::VTABLE_RVA }
    fn virtual_method_count(&self) -> usize { Self::METHOD_COUNT }
    fn base_classes(&self) -> &'static [&'static str] { Self::BASE_CLASSES }
}

/// Decompiled CC2 class: `ccPDEffectNoize` (VTable RVA: 0x008b6ea8, 3 virtual methods, 5 recovered fields)
/// Inherits: ccPDEffectBase
#[derive(Debug, Clone, Default)]
pub struct ccPDEffectNoize {
    pub vtable_rva: u32,
    pub field_0x0010: u32,
    pub field_0x0014: u16,
    pub field_0x001a: u8,
    pub field_0x0058: u32,
    pub field_0x01fc: u8,
}

impl ccPDEffectNoize {
    pub const CLASS_NAME: &'static str = "ccPDEffectNoize";
    pub const VTABLE_RVA: u32 = 0x008b6ea8;
    pub const METHOD_COUNT: usize = 3;
    pub const BASE_CLASSES: &'static [&'static str] = &["ccPDEffectBase"];
    pub const VIRTUAL_METHODS: &'static [u64] = &[0x0000000180385220, 0x0000000180385210, 0x0000000180385130];

    pub fn new() -> Self {
        Self {
            vtable_rva: Self::VTABLE_RVA,
            field_0x0010: Default::default(),
            field_0x0014: Default::default(),
            field_0x001a: Default::default(),
            field_0x0058: Default::default(),
            field_0x01fc: Default::default(),
        }
    }

    /// Decompiled virtual method #0 @ RVA 0x00385220 (138 instructions, 15 basic blocks)
    pub fn method_00(&mut self) {
        // Writes to field at offset +0x0010
        // Writes to field at offset +0x001a
    }

    /// Decompiled virtual method #1 @ RVA 0x00385210 (6 instructions, 3 basic blocks)
    pub fn method_01(&mut self) {
        // Writes to field at offset +0x0010
    }

    /// Decompiled virtual method #2 @ RVA 0x00385130 (2 instructions, 1 basic blocks)
    pub fn method_02(&mut self) {
    }

}

impl CcClass for ccPDEffectNoize {
    fn class_name(&self) -> &'static str { Self::CLASS_NAME }
    fn vtable_rva(&self) -> u32 { Self::VTABLE_RVA }
    fn virtual_method_count(&self) -> usize { Self::METHOD_COUNT }
    fn base_classes(&self) -> &'static [&'static str] { Self::BASE_CLASSES }
}

/// Decompiled CC2 class: `ccPDEffectReverse` (VTable RVA: 0x008b6e88, 3 virtual methods, 4 recovered fields)
/// Inherits: ccPDEffectBase
#[derive(Debug, Clone, Default)]
pub struct ccPDEffectReverse {
    pub vtable_rva: u32,
    pub field_0x0010: u32,
    pub field_0x0014: u16,
    pub field_0x0016: u16,
    pub field_0x0018: u8,
}

impl ccPDEffectReverse {
    pub const CLASS_NAME: &'static str = "ccPDEffectReverse";
    pub const VTABLE_RVA: u32 = 0x008b6e88;
    pub const METHOD_COUNT: usize = 3;
    pub const BASE_CLASSES: &'static [&'static str] = &["ccPDEffectBase"];
    pub const VIRTUAL_METHODS: &'static [u64] = &[0x0000000180385180, 0x0000000180385140, 0x0000000180385130];

    pub fn new() -> Self {
        Self {
            vtable_rva: Self::VTABLE_RVA,
            field_0x0010: Default::default(),
            field_0x0014: Default::default(),
            field_0x0016: Default::default(),
            field_0x0018: Default::default(),
        }
    }

    /// Decompiled virtual method #0 @ RVA 0x00385180 (14 instructions, 4 basic blocks)
    pub fn method_00(&mut self) {
        // Writes to field at offset +0x0010
        // Writes to field at offset +0x0018
    }

    /// Decompiled virtual method #1 @ RVA 0x00385140 (17 instructions, 6 basic blocks)
    pub fn method_01(&mut self) {
        // Writes to field at offset +0x0010
        // Writes to field at offset +0x0016
        // Writes to field at offset +0x0018
    }

    /// Decompiled virtual method #2 @ RVA 0x00385130 (2 instructions, 1 basic blocks)
    pub fn method_02(&mut self) {
    }

}

impl CcClass for ccPDEffectReverse {
    fn class_name(&self) -> &'static str { Self::CLASS_NAME }
    fn vtable_rva(&self) -> u32 { Self::VTABLE_RVA }
    fn virtual_method_count(&self) -> usize { Self::METHOD_COUNT }
    fn base_classes(&self) -> &'static [&'static str] { Self::BASE_CLASSES }
}

/// Decompiled CC2 class: `ccPart` (VTable RVA: 0x00876e98, 1 virtual methods, 1 recovered fields)
/// Inherits: ccCoord
#[derive(Debug, Clone, Default)]
pub struct ccPart {
    pub vtable_rva: u32,
    pub field_0x0000: u64,
}

impl ccPart {
    pub const CLASS_NAME: &'static str = "ccPart";
    pub const VTABLE_RVA: u32 = 0x00876e98;
    pub const METHOD_COUNT: usize = 1;
    pub const BASE_CLASSES: &'static [&'static str] = &["ccCoord"];
    pub const VIRTUAL_METHODS: &'static [u64] = &[0x00000001801582e0];

    pub fn new() -> Self {
        Self {
            vtable_rva: Self::VTABLE_RVA,
            field_0x0000: Default::default(),
        }
    }

    /// Decompiled virtual method #0 @ RVA 0x001582e0 (13 instructions, 3 basic blocks)
    pub fn method_00(&mut self) {
        // Writes to field at offset +0x0000
    }

}

impl CcClass for ccPart {
    fn class_name(&self) -> &'static str { Self::CLASS_NAME }
    fn vtable_rva(&self) -> u32 { Self::VTABLE_RVA }
    fn virtual_method_count(&self) -> usize { Self::METHOD_COUNT }
    fn base_classes(&self) -> &'static [&'static str] { Self::BASE_CLASSES }
}

/// Decompiled CC2 class: `ccParticle2` (VTable RVA: 0x00865fb8, 5 virtual methods, 15 recovered fields)
/// Inherits: ccParticleElm
#[derive(Debug, Clone, Default)]
pub struct ccParticle2 {
    pub vtable_rva: u32,
    pub field_0x0000: u64,
    pub field_0x0018: u64,
    pub field_0x0030: f32,
    pub field_0x0034: u32,
    pub field_0x0038: f32,
    pub field_0x003c: f32,
    pub field_0x009c: f32,
    pub field_0x00a0: u8,
    pub field_0x00b0: u8,
    pub field_0x00b4: u32,
    pub field_0x00b8: u32,
    pub field_0x00c0: u32,
    pub field_0x00c4: f32,
    pub field_0x00c8: u32,
    pub field_0x00cc: u32,
}

impl ccParticle2 {
    pub const CLASS_NAME: &'static str = "ccParticle2";
    pub const VTABLE_RVA: u32 = 0x00865fb8;
    pub const METHOD_COUNT: usize = 5;
    pub const BASE_CLASSES: &'static [&'static str] = &["ccParticleElm"];
    pub const VIRTUAL_METHODS: &'static [u64] = &[0x00000001800ead20, 0x00000001800e3a40, 0x00000001800eb100, 0x00000001800ead60, 0x00000001800d3090];

    pub fn new() -> Self {
        Self {
            vtable_rva: Self::VTABLE_RVA,
            field_0x0000: Default::default(),
            field_0x0018: Default::default(),
            field_0x0030: Default::default(),
            field_0x0034: Default::default(),
            field_0x0038: Default::default(),
            field_0x003c: Default::default(),
            field_0x009c: Default::default(),
            field_0x00a0: Default::default(),
            field_0x00b0: Default::default(),
            field_0x00b4: Default::default(),
            field_0x00b8: Default::default(),
            field_0x00c0: Default::default(),
            field_0x00c4: Default::default(),
            field_0x00c8: Default::default(),
            field_0x00cc: Default::default(),
        }
    }

    /// Decompiled virtual method #0 @ RVA 0x000ead20 (32 instructions, 4 basic blocks)
    pub fn method_00(&mut self) {
        // Writes to field at offset +0x0000
    }

    /// Decompiled virtual method #1 @ RVA 0x000e3a40 (68 instructions, 7 basic blocks)
    pub fn method_01(&mut self) {
        // Writes to field at offset +0x0018
        // Writes to field at offset +0x0030
        // Writes to field at offset +0x0034
        // Writes to field at offset +0x0038
        // Writes to field at offset +0x003c
        // Writes to field at offset +0x009c
        // Writes to field at offset +0x00a0
        // Writes to field at offset +0x00b4
        // Writes to field at offset +0x00b8
    }

    /// Decompiled virtual method #2 @ RVA 0x000eb100 (3 instructions, 2 basic blocks)
    pub fn method_02(&mut self) {
        // Writes to field at offset +0x00b0
    }

    /// Decompiled virtual method #3 @ RVA 0x000ead60 (18 instructions, 3 basic blocks)
    pub fn method_03(&mut self) {
        // Writes to field at offset +0x0000
    }

    /// Decompiled virtual method #4 @ RVA 0x000d3090 (1 instructions, 1 basic blocks)
    pub fn method_04(&mut self) {
    }

}

impl CcClass for ccParticle2 {
    fn class_name(&self) -> &'static str { Self::CLASS_NAME }
    fn vtable_rva(&self) -> u32 { Self::VTABLE_RVA }
    fn virtual_method_count(&self) -> usize { Self::METHOD_COUNT }
    fn base_classes(&self) -> &'static [&'static str] { Self::BASE_CLASSES }
}

/// Decompiled CC2 class: `ccParticleElm` (VTable RVA: 0x00865fe8, 3 virtual methods, 10 recovered fields)
#[derive(Debug, Clone, Default)]
pub struct ccParticleElm {
    pub vtable_rva: u32,
    pub field_0x0010: u32,
    pub field_0x0014: f32,
    pub field_0x0018: u64,
    pub field_0x001c: u32,
    pub field_0x003a: u8,
    pub field_0x0090: u32,
    pub field_0x00b0: u64,
    pub field_0x00b8: u64,
    pub field_0x014c: u32,
    pub field_0x0168: u64,
}

impl ccParticleElm {
    pub const CLASS_NAME: &'static str = "ccParticleElm";
    pub const VTABLE_RVA: u32 = 0x00865fe8;
    pub const METHOD_COUNT: usize = 3;
    pub const BASE_CLASSES: &'static [&'static str] = &[];
    pub const VIRTUAL_METHODS: &'static [u64] = &[0x00000001800e3690, 0x00000001800e3700, 0x00000001800e3810];

    pub fn new() -> Self {
        Self {
            vtable_rva: Self::VTABLE_RVA,
            field_0x0010: Default::default(),
            field_0x0014: Default::default(),
            field_0x0018: Default::default(),
            field_0x001c: Default::default(),
            field_0x003a: Default::default(),
            field_0x0090: Default::default(),
            field_0x00b0: Default::default(),
            field_0x00b8: Default::default(),
            field_0x014c: Default::default(),
            field_0x0168: Default::default(),
        }
    }

    /// Decompiled virtual method #0 @ RVA 0x000e3690 (112 instructions, 30 basic blocks)
    pub fn method_00(&mut self) {
        // Writes to field at offset +0x0168
    }

    /// Decompiled virtual method #1 @ RVA 0x000e3700 (78 instructions, 22 basic blocks)
    pub fn method_01(&mut self) {
        // Writes to field at offset +0x0168
    }

    /// Decompiled virtual method #2 @ RVA 0x000e3810 (54 instructions, 15 basic blocks)
    pub fn method_02(&mut self) {
        // Writes to field at offset +0x001c
        // Writes to field at offset +0x003a
    }

}

impl CcClass for ccParticleElm {
    fn class_name(&self) -> &'static str { Self::CLASS_NAME }
    fn vtable_rva(&self) -> u32 { Self::VTABLE_RVA }
    fn virtual_method_count(&self) -> usize { Self::METHOD_COUNT }
    fn base_classes(&self) -> &'static [&'static str] { Self::BASE_CLASSES }
}

/// Decompiled CC2 class: `ccParticleManager` (VTable RVA: 0x00865f80, 1 virtual methods, 3 recovered fields)
#[derive(Debug, Clone, Default)]
pub struct ccParticleManager {
    pub vtable_rva: u32,
    pub field_0x0000: u64,
    pub field_0x0060: u64,
    pub field_0x0090: u64,
}

impl ccParticleManager {
    pub const CLASS_NAME: &'static str = "ccParticleManager";
    pub const VTABLE_RVA: u32 = 0x00865f80;
    pub const METHOD_COUNT: usize = 1;
    pub const BASE_CLASSES: &'static [&'static str] = &[];
    pub const VIRTUAL_METHODS: &'static [u64] = &[0x00000001800eaaa0];

    pub fn new() -> Self {
        Self {
            vtable_rva: Self::VTABLE_RVA,
            field_0x0000: Default::default(),
            field_0x0060: Default::default(),
            field_0x0090: Default::default(),
        }
    }

    /// Decompiled virtual method #0 @ RVA 0x000eaaa0 (30 instructions, 5 basic blocks)
    pub fn method_00(&mut self) {
        // Writes to field at offset +0x0000
    }

}

impl CcClass for ccParticleManager {
    fn class_name(&self) -> &'static str { Self::CLASS_NAME }
    fn vtable_rva(&self) -> u32 { Self::VTABLE_RVA }
    fn virtual_method_count(&self) -> usize { Self::METHOD_COUNT }
    fn base_classes(&self) -> &'static [&'static str] { Self::BASE_CLASSES }
}

/// Decompiled CC2 class: `ccPuppetShowCast` (VTable RVA: 0x008e78e8, 14 virtual methods, 42 recovered fields)
/// Inherits: ccGameObj
#[derive(Debug, Clone, Default)]
pub struct ccPuppetShowCast {
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
    pub field_0x01c8: u32,
    pub field_0x01ee: u16,
    pub field_0x01f0: u32,
}

impl ccPuppetShowCast {
    pub const CLASS_NAME: &'static str = "ccPuppetShowCast";
    pub const VTABLE_RVA: u32 = 0x008e78e8;
    pub const METHOD_COUNT: usize = 14;
    pub const BASE_CLASSES: &'static [&'static str] = &["ccGameObj"];
    pub const VIRTUAL_METHODS: &'static [u64] = &[0x00000001805e6630, 0x00000001805e8a20, 0x000000018045e4c0, 0x00000001805e98c0, 0x000000018019e550, 0x00000001805e8f10, 0x00000001800d3090, 0x00000001800d3090, 0x00000001800d3090, 0x00000001805e9020, 0x00000001805e8ff0, 0x000000018045e2d0, 0x000000018045eec0, 0x000000018045efd0];

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
            field_0x01c8: Default::default(),
            field_0x01ee: Default::default(),
            field_0x01f0: Default::default(),
        }
    }

    /// Decompiled virtual method #0 @ RVA 0x005e6630 (16 instructions, 3 basic blocks)
    pub fn method_00(&mut self) {
    }

    /// Decompiled virtual method #1 @ RVA 0x005e8a20 (16 instructions, 3 basic blocks)
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

    /// Decompiled virtual method #3 @ RVA 0x005e98c0 (18 instructions, 7 basic blocks)
    pub fn method_03(&mut self) {
        // Writes to field at offset +0x00b0
    }

    /// Decompiled virtual method #4 @ RVA 0x0019e550 (2 instructions, 1 basic blocks)
    pub fn method_04(&mut self) {
        // Writes to field at offset +0x00c4
    }

    /// Decompiled virtual method #5 @ RVA 0x005e8f10 (82 instructions, 24 basic blocks)
    pub fn method_05(&mut self) {
        // Writes to field at offset +0x0024
        // Writes to field at offset +0x01c8
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

    /// Decompiled virtual method #9 @ RVA 0x005e9020 (104 instructions, 10 basic blocks)
    pub fn method_09(&mut self) {
        // Writes to field at offset +0x0000
        // Writes to field at offset +0x0004
        // Writes to field at offset +0x0008
        // Writes to field at offset +0x000a
    }

    /// Decompiled virtual method #10 @ RVA 0x005e8ff0 (13 instructions, 3 basic blocks)
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

impl CcClass for ccPuppetShowCast {
    fn class_name(&self) -> &'static str { Self::CLASS_NAME }
    fn vtable_rva(&self) -> u32 { Self::VTABLE_RVA }
    fn virtual_method_count(&self) -> usize { Self::METHOD_COUNT }
    fn base_classes(&self) -> &'static [&'static str] { Self::BASE_CLASSES }
}

/// Decompiled CC2 class: `ccQuestShopOpt` (VTable RVA: 0x008c0418, 3 virtual methods, 9 recovered fields)
/// Inherits: ccGobjDrawOpt
#[derive(Debug, Clone, Default)]
pub struct ccQuestShopOpt {
    pub vtable_rva: u32,
    pub field_0x0000: u64,
    pub field_0x00b8: u64,
    pub field_0x00d0: u64,
    pub field_0x00d8: u32,
    pub field_0x00dc: u32,
    pub field_0x00e0: u64,
    pub field_0x00e8: u64,
    pub field_0x00f0: u64,
    pub field_0x00f8: u32,
}

impl ccQuestShopOpt {
    pub const CLASS_NAME: &'static str = "ccQuestShopOpt";
    pub const VTABLE_RVA: u32 = 0x008c0418;
    pub const METHOD_COUNT: usize = 3;
    pub const BASE_CLASSES: &'static [&'static str] = &["ccGobjDrawOpt"];
    pub const VIRTUAL_METHODS: &'static [u64] = &[0x00000001804fe810, 0x0000000180502410, 0x0000000180502470];

    pub fn new() -> Self {
        Self {
            vtable_rva: Self::VTABLE_RVA,
            field_0x0000: Default::default(),
            field_0x00b8: Default::default(),
            field_0x00d0: Default::default(),
            field_0x00d8: Default::default(),
            field_0x00dc: Default::default(),
            field_0x00e0: Default::default(),
            field_0x00e8: Default::default(),
            field_0x00f0: Default::default(),
            field_0x00f8: Default::default(),
        }
    }

    /// Decompiled virtual method #0 @ RVA 0x004fe810 (26 instructions, 3 basic blocks)
    pub fn method_00(&mut self) {
        // Writes to field at offset +0x0000
        // Writes to field at offset +0x00d0
        // Writes to field at offset +0x00d8
        // Writes to field at offset +0x00dc
        // Writes to field at offset +0x00e0
        // Writes to field at offset +0x00e8
        // Writes to field at offset +0x00f0
        // Writes to field at offset +0x00f8
    }

    /// Decompiled virtual method #1 @ RVA 0x00502410 (12 instructions, 3 basic blocks)
    pub fn method_01(&mut self) {
    }

    /// Decompiled virtual method #2 @ RVA 0x00502470 (46 instructions, 9 basic blocks)
    pub fn method_02(&mut self) {
    }

}

impl CcClass for ccQuestShopOpt {
    fn class_name(&self) -> &'static str { Self::CLASS_NAME }
    fn vtable_rva(&self) -> u32 { Self::VTABLE_RVA }
    fn virtual_method_count(&self) -> usize { Self::METHOD_COUNT }
    fn base_classes(&self) -> &'static [&'static str] { Self::BASE_CLASSES }
}

/// Decompiled CC2 class: `ccRareChimAI` (VTable RVA: 0x008bc7b8, 23 virtual methods, 21 recovered fields)
/// Inherits: ccChimAI
#[derive(Debug, Clone, Default)]
pub struct ccRareChimAI {
    pub vtable_rva: u32,
    pub field_0x0000: u64,
    pub field_0x0001: u32,
    pub field_0x0004: f32,
    pub field_0x0010: u64,
    pub field_0x0020: u32,
    pub field_0x0021: u8,
    pub field_0x0024: u16,
    pub field_0x002c: u16,
    pub field_0x002e: u16,
    pub field_0x0034: u32,
    pub field_0x0040: u32,
    pub field_0x0050: u64,
    pub field_0x0054: u32,
    pub field_0x0090: u64,
    pub field_0x00a0: u32,
    pub field_0x00a4: u32,
    pub field_0x00cc: u32,
    pub field_0x00e0: u32,
    pub field_0x0101: u8,
    pub field_0x0108: u16,
    pub field_0x0160: u8,
}

impl ccRareChimAI {
    pub const CLASS_NAME: &'static str = "ccRareChimAI";
    pub const VTABLE_RVA: u32 = 0x008bc7b8;
    pub const METHOD_COUNT: usize = 23;
    pub const BASE_CLASSES: &'static [&'static str] = &["ccChimAI"];
    pub const VIRTUAL_METHODS: &'static [u64] = &[0x0000000180298820, 0x0000000180298c90, 0x0000000180298ce0, 0x00000001802987f0, 0x000000018029cc60, 0x000000018029c650, 0x0000000180299680, 0x000000018029c980, 0x00000001802998c0, 0x0000000180299f50, 0x000000018029a9c0, 0x000000018029a170, 0x000000018029c390, 0x00000001800d3090, 0x00000001800d3090, 0x00000001800d3090, 0x000000018029a9d0, 0x000000018029abe0, 0x000000018029ac00, 0x000000018029aeb0, 0x000000018029b520, 0x000000018029b5f0, 0x000000018029b870];

    pub fn new() -> Self {
        Self {
            vtable_rva: Self::VTABLE_RVA,
            field_0x0000: Default::default(),
            field_0x0001: Default::default(),
            field_0x0004: Default::default(),
            field_0x0010: Default::default(),
            field_0x0020: Default::default(),
            field_0x0021: Default::default(),
            field_0x0024: Default::default(),
            field_0x002c: Default::default(),
            field_0x002e: Default::default(),
            field_0x0034: Default::default(),
            field_0x0040: Default::default(),
            field_0x0050: Default::default(),
            field_0x0054: Default::default(),
            field_0x0090: Default::default(),
            field_0x00a0: Default::default(),
            field_0x00a4: Default::default(),
            field_0x00cc: Default::default(),
            field_0x00e0: Default::default(),
            field_0x0101: Default::default(),
            field_0x0108: Default::default(),
            field_0x0160: Default::default(),
        }
    }

    /// Decompiled virtual method #0 @ RVA 0x00298820 (124 instructions, 31 basic blocks)
    pub fn method_00(&mut self) {
        // Writes to field at offset +0x0000
    }

    /// Decompiled virtual method #1 @ RVA 0x00298c90 (15 instructions, 5 basic blocks)
    pub fn method_01(&mut self) {
        // Writes to field at offset +0x002e
        // Writes to field at offset +0x00e0
        // Writes to field at offset +0x0160
    }

    /// Decompiled virtual method #2 @ RVA 0x00298ce0 (125 instructions, 30 basic blocks)
    pub fn method_02(&mut self) {
        // Writes to field at offset +0x0000
    }

    /// Decompiled virtual method #3 @ RVA 0x002987f0 (11 instructions, 3 basic blocks)
    pub fn method_03(&mut self) {
        // Writes to field at offset +0x0020
        // Writes to field at offset +0x002c
        // Writes to field at offset +0x002e
        // Writes to field at offset +0x0034
    }

    /// Decompiled virtual method #4 @ RVA 0x0029cc60 (123 instructions, 23 basic blocks)
    pub fn method_04(&mut self) {
    }

    /// Decompiled virtual method #5 @ RVA 0x0029c650 (104 instructions, 19 basic blocks)
    pub fn method_05(&mut self) {
    }

    /// Decompiled virtual method #6 @ RVA 0x00299680 (80 instructions, 9 basic blocks)
    pub fn method_06(&mut self) {
    }

    /// Decompiled virtual method #7 @ RVA 0x0029c980 (54 instructions, 5 basic blocks)
    pub fn method_07(&mut self) {
    }

    /// Decompiled virtual method #8 @ RVA 0x002998c0 (54 instructions, 5 basic blocks)
    pub fn method_08(&mut self) {
    }

    /// Decompiled virtual method #9 @ RVA 0x00299f50 (60 instructions, 8 basic blocks)
    pub fn method_09(&mut self) {
    }

    /// Decompiled virtual method #10 @ RVA 0x0029a9c0 (2 instructions, 1 basic blocks)
    pub fn method_10(&mut self) {
        // Writes to field at offset +0x0021
    }

    /// Decompiled virtual method #11 @ RVA 0x0029a170 (113 instructions, 25 basic blocks)
    pub fn method_11(&mut self) {
        // Writes to field at offset +0x0000
    }

    /// Decompiled virtual method #12 @ RVA 0x0029c390 (68 instructions, 10 basic blocks)
    pub fn method_12(&mut self) {
    }

    /// Decompiled virtual method #13 @ RVA 0x000d3090 (1 instructions, 1 basic blocks)
    pub fn method_13(&mut self) {
    }

    /// Decompiled virtual method #14 @ RVA 0x000d3090 (1 instructions, 1 basic blocks)
    pub fn method_14(&mut self) {
    }

    /// Decompiled virtual method #15 @ RVA 0x000d3090 (1 instructions, 1 basic blocks)
    pub fn method_15(&mut self) {
    }

    /// Decompiled virtual method #16 @ RVA 0x0029a9d0 (82 instructions, 20 basic blocks)
    pub fn method_16(&mut self) {
        // Writes to field at offset +0x0000
    }

    /// Decompiled virtual method #17 @ RVA 0x0029abe0 (6 instructions, 1 basic blocks)
    pub fn method_17(&mut self) {
        // Writes to field at offset +0x0020
        // Writes to field at offset +0x002c
        // Writes to field at offset +0x002e
        // Writes to field at offset +0x0034
    }

    /// Decompiled virtual method #18 @ RVA 0x0029ac00 (97 instructions, 12 basic blocks)
    pub fn method_18(&mut self) {
        // Writes to field at offset +0x0054
    }

    /// Decompiled virtual method #19 @ RVA 0x0029aeb0 (121 instructions, 28 basic blocks)
    pub fn method_19(&mut self) {
        // Writes to field at offset +0x0000
    }

    /// Decompiled virtual method #20 @ RVA 0x0029b520 (20 instructions, 7 basic blocks)
    pub fn method_20(&mut self) {
        // Writes to field at offset +0x0040
    }

    /// Decompiled virtual method #21 @ RVA 0x0029b5f0 (61 instructions, 9 basic blocks)
    pub fn method_21(&mut self) {
    }

    /// Decompiled virtual method #22 @ RVA 0x0029b870 (109 instructions, 28 basic blocks)
    pub fn method_22(&mut self) {
        // Writes to field at offset +0x0000
        // Writes to field at offset +0x00cc
    }

}

impl CcClass for ccRareChimAI {
    fn class_name(&self) -> &'static str { Self::CLASS_NAME }
    fn vtable_rva(&self) -> u32 { Self::VTABLE_RVA }
    fn virtual_method_count(&self) -> usize { Self::METHOD_COUNT }
    fn base_classes(&self) -> &'static [&'static str] { Self::BASE_CLASSES }
}

/// Decompiled CC2 class: `ccRecollectGyroList` (VTable RVA: 0x008e8e30, 19 virtual methods, 15 recovered fields)
/// Inherits: GyroMenuList, CenteringMenuList, MenuList
#[derive(Debug, Clone, Default)]
pub struct ccRecollectGyroList {
    pub vtable_rva: u32,
    pub field_0x0000: u64,
    pub field_0x0010: u64,
    pub field_0x0020: u32,
    pub field_0x0028: u32,
    pub field_0x0030: f32,
    pub field_0x0034: u32,
    pub field_0x003c: u32,
    pub field_0x0040: u32,
    pub field_0x0044: u32,
    pub field_0x004c: u8,
    pub field_0x00e4: u32,
    pub field_0x0100: u8,
    pub field_0x0108: u64,
    pub field_0x0110: u64,
    pub field_0x0120: u64,
}

impl ccRecollectGyroList {
    pub const CLASS_NAME: &'static str = "ccRecollectGyroList";
    pub const VTABLE_RVA: u32 = 0x008e8e30;
    pub const METHOD_COUNT: usize = 19;
    pub const BASE_CLASSES: &'static [&'static str] = &["GyroMenuList", "CenteringMenuList", "MenuList"];
    pub const VIRTUAL_METHODS: &'static [u64] = &[0x00000001800d3090, 0x00000001800d3090, 0x00000001805fbca0, 0x000000018010ff30, 0x00000001805fc310, 0x00000001805fc540, 0x00000001805fc580, 0x00000001800d3090, 0x0000000180112510, 0x00000001801127a0, 0x0000000180108e30, 0x0000000180108f10, 0x00000001805fc600, 0x00000001801105f0, 0x000000018010ff60, 0x00000001805fbd30, 0x0000000180112a50, 0x0000000180112d00, 0x00000001805fc630];

    pub fn new() -> Self {
        Self {
            vtable_rva: Self::VTABLE_RVA,
            field_0x0000: Default::default(),
            field_0x0010: Default::default(),
            field_0x0020: Default::default(),
            field_0x0028: Default::default(),
            field_0x0030: Default::default(),
            field_0x0034: Default::default(),
            field_0x003c: Default::default(),
            field_0x0040: Default::default(),
            field_0x0044: Default::default(),
            field_0x004c: Default::default(),
            field_0x00e4: Default::default(),
            field_0x0100: Default::default(),
            field_0x0108: Default::default(),
            field_0x0110: Default::default(),
            field_0x0120: Default::default(),
        }
    }

    /// Decompiled virtual method #0 @ RVA 0x000d3090 (1 instructions, 1 basic blocks)
    pub fn method_00(&mut self) {
    }

    /// Decompiled virtual method #1 @ RVA 0x000d3090 (1 instructions, 1 basic blocks)
    pub fn method_01(&mut self) {
    }

    /// Decompiled virtual method #2 @ RVA 0x005fbca0 (32 instructions, 5 basic blocks)
    pub fn method_02(&mut self) {
        // Writes to field at offset +0x0000
    }

    /// Decompiled virtual method #3 @ RVA 0x0010ff30 (9 instructions, 1 basic blocks)
    pub fn method_03(&mut self) {
    }

    /// Decompiled virtual method #4 @ RVA 0x005fc310 (21 instructions, 2 basic blocks)
    pub fn method_04(&mut self) {
    }

    /// Decompiled virtual method #5 @ RVA 0x005fc540 (13 instructions, 1 basic blocks)
    pub fn method_05(&mut self) {
    }

    /// Decompiled virtual method #6 @ RVA 0x005fc580 (29 instructions, 6 basic blocks)
    pub fn method_06(&mut self) {
    }

    /// Decompiled virtual method #7 @ RVA 0x000d3090 (1 instructions, 1 basic blocks)
    pub fn method_07(&mut self) {
    }

    /// Decompiled virtual method #8 @ RVA 0x00112510 (9 instructions, 2 basic blocks)
    pub fn method_08(&mut self) {
        // Writes to field at offset +0x00e4
    }

    /// Decompiled virtual method #9 @ RVA 0x001127a0 (9 instructions, 2 basic blocks)
    pub fn method_09(&mut self) {
        // Writes to field at offset +0x00e4
    }

    /// Decompiled virtual method #10 @ RVA 0x00108e30 (65 instructions, 13 basic blocks)
    pub fn method_10(&mut self) {
        // Writes to field at offset +0x0020
        // Writes to field at offset +0x0028
        // Writes to field at offset +0x0034
        // Writes to field at offset +0x004c
    }

    /// Decompiled virtual method #11 @ RVA 0x00108f10 (70 instructions, 13 basic blocks)
    pub fn method_11(&mut self) {
        // Writes to field at offset +0x0028
        // Writes to field at offset +0x0034
    }

    /// Decompiled virtual method #12 @ RVA 0x005fc600 (8 instructions, 3 basic blocks)
    pub fn method_12(&mut self) {
    }

    /// Decompiled virtual method #13 @ RVA 0x001105f0 (13 instructions, 1 basic blocks)
    pub fn method_13(&mut self) {
    }

    /// Decompiled virtual method #14 @ RVA 0x0010ff60 (117 instructions, 12 basic blocks)
    pub fn method_14(&mut self) {
        // Writes to field at offset +0x003c
        // Writes to field at offset +0x0040
        // Writes to field at offset +0x0044
    }

    /// Decompiled virtual method #15 @ RVA 0x005fbd30 (62 instructions, 4 basic blocks)
    pub fn method_15(&mut self) {
        // Writes to field at offset +0x004c
        // Writes to field at offset +0x0100
    }

    /// Decompiled virtual method #16 @ RVA 0x00112a50 (12 instructions, 2 basic blocks)
    pub fn method_16(&mut self) {
        // Writes to field at offset +0x00e4
    }

    /// Decompiled virtual method #17 @ RVA 0x00112d00 (12 instructions, 2 basic blocks)
    pub fn method_17(&mut self) {
        // Writes to field at offset +0x00e4
    }

    /// Decompiled virtual method #18 @ RVA 0x005fc630 (98 instructions, 21 basic blocks)
    pub fn method_18(&mut self) {
        // Writes to field at offset +0x00e4
    }

}

impl CcClass for ccRecollectGyroList {
    fn class_name(&self) -> &'static str { Self::CLASS_NAME }
    fn vtable_rva(&self) -> u32 { Self::VTABLE_RVA }
    fn virtual_method_count(&self) -> usize { Self::METHOD_COUNT }
    fn base_classes(&self) -> &'static [&'static str] { Self::BASE_CLASSES }
}

/// Decompiled CC2 class: `ccRecovHp` (VTable RVA: 0x00881ab8, 16 virtual methods, 18 recovered fields)
/// Inherits: ccDamageNormal, ccDamageBase, ccDispBase
#[derive(Debug, Clone, Default)]
pub struct ccRecovHp {
    pub vtable_rva: u32,
    pub field_0x0000: u64,
    pub field_0x0008: u32,
    pub field_0x000c: u32,
    pub field_0x0010: u32,
    pub field_0x0014: f32,
    pub field_0x0018: f32,
    pub field_0x0028: u32,
    pub field_0x002c: u32,
    pub field_0x0030: u32,
    pub field_0x0033: u8,
    pub field_0x0034: u32,
    pub field_0x0040: u32,
    pub field_0x0044: f32,
    pub field_0x0048: f32,
    pub field_0x004c: f32,
    pub field_0x0050: u32,
    pub field_0x0058: u64,
    pub field_0x0060: u64,
}

impl ccRecovHp {
    pub const CLASS_NAME: &'static str = "ccRecovHp";
    pub const VTABLE_RVA: u32 = 0x00881ab8;
    pub const METHOD_COUNT: usize = 16;
    pub const BASE_CLASSES: &'static [&'static str] = &["ccDamageNormal", "ccDamageBase", "ccDispBase"];
    pub const VIRTUAL_METHODS: &'static [u64] = &[0x00000001802328a0, 0x00000001800d3090, 0x0000000180233850, 0x0000000180233de0, 0x00000001802a0e30, 0x00000001800d3090, 0x00000001801b81f0, 0x00000001801b8200, 0x00000001801b8210, 0x00000001800d3090, 0x00000001800d3090, 0x00000001802338c0, 0x0000000180233970, 0x0000000180233a00, 0x0000000180232900, 0x0000000180232920];

    pub fn new() -> Self {
        Self {
            vtable_rva: Self::VTABLE_RVA,
            field_0x0000: Default::default(),
            field_0x0008: Default::default(),
            field_0x000c: Default::default(),
            field_0x0010: Default::default(),
            field_0x0014: Default::default(),
            field_0x0018: Default::default(),
            field_0x0028: Default::default(),
            field_0x002c: Default::default(),
            field_0x0030: Default::default(),
            field_0x0033: Default::default(),
            field_0x0034: Default::default(),
            field_0x0040: Default::default(),
            field_0x0044: Default::default(),
            field_0x0048: Default::default(),
            field_0x004c: Default::default(),
            field_0x0050: Default::default(),
            field_0x0058: Default::default(),
            field_0x0060: Default::default(),
        }
    }

    /// Decompiled virtual method #0 @ RVA 0x002328a0 (23 instructions, 3 basic blocks)
    pub fn method_00(&mut self) {
        // Writes to field at offset +0x0000
    }

    /// Decompiled virtual method #1 @ RVA 0x000d3090 (1 instructions, 1 basic blocks)
    pub fn method_01(&mut self) {
    }

    /// Decompiled virtual method #2 @ RVA 0x00233850 (26 instructions, 5 basic blocks)
    pub fn method_02(&mut self) {
    }

    /// Decompiled virtual method #3 @ RVA 0x00233de0 (126 instructions, 3 basic blocks)
    pub fn method_03(&mut self) {
        // Writes to field at offset +0x0030
        // Writes to field at offset +0x0033
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

    /// Decompiled virtual method #11 @ RVA 0x002338c0 (46 instructions, 14 basic blocks)
    pub fn method_11(&mut self) {
        // Writes to field at offset +0x0040
        // Writes to field at offset +0x0040
        // Writes to field at offset +0x0048
        // Writes to field at offset +0x0048
        // Writes to field at offset +0x004c
        // Writes to field at offset +0x004c
    }

    /// Decompiled virtual method #12 @ RVA 0x00233970 (33 instructions, 3 basic blocks)
    pub fn method_12(&mut self) {
        // Writes to field at offset +0x0050
    }

    /// Decompiled virtual method #13 @ RVA 0x00233a00 (41 instructions, 10 basic blocks)
    pub fn method_13(&mut self) {
        // Writes to field at offset +0x0014
        // Writes to field at offset +0x0018
        // Writes to field at offset +0x0040
        // Writes to field at offset +0x0040
        // Writes to field at offset +0x0048
        // Writes to field at offset +0x004c
    }

    /// Decompiled virtual method #14 @ RVA 0x00232900 (8 instructions, 1 basic blocks)
    pub fn method_14(&mut self) {
    }

    /// Decompiled virtual method #15 @ RVA 0x00232920 (2 instructions, 1 basic blocks)
    pub fn method_15(&mut self) {
    }

}

impl CcClass for ccRecovHp {
    fn class_name(&self) -> &'static str { Self::CLASS_NAME }
    fn vtable_rva(&self) -> u32 { Self::VTABLE_RVA }
    fn virtual_method_count(&self) -> usize { Self::METHOD_COUNT }
    fn base_classes(&self) -> &'static [&'static str] { Self::BASE_CLASSES }
}

/// Decompiled CC2 class: `ccSaNameStr` (VTable RVA: 0x008814b8, 14 virtual methods, 15 recovered fields)
/// Inherits: ccDispBase
#[derive(Debug, Clone, Default)]
pub struct ccSaNameStr {
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
    pub field_0x0032: u16,
    pub field_0x0034: u32,
    pub field_0x0084: u16,
    pub field_0x0086: u16,
    pub field_0x0088: u32,
}

impl ccSaNameStr {
    pub const CLASS_NAME: &'static str = "ccSaNameStr";
    pub const VTABLE_RVA: u32 = 0x008814b8;
    pub const METHOD_COUNT: usize = 14;
    pub const BASE_CLASSES: &'static [&'static str] = &["ccDispBase"];
    pub const VIRTUAL_METHODS: &'static [u64] = &[0x0000000180238920, 0x00000001800d3090, 0x00000001800d3090, 0x0000000180239100, 0x00000001802a0e30, 0x00000001800d3090, 0x00000001801b81f0, 0x00000001801b8200, 0x00000001801b8210, 0x00000001800d3090, 0x00000001800d3090, 0x00000001802389f0, 0x0000000180238a70, 0x0000000180238fb0];

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
            field_0x0032: Default::default(),
            field_0x0034: Default::default(),
            field_0x0084: Default::default(),
            field_0x0086: Default::default(),
            field_0x0088: Default::default(),
        }
    }

    /// Decompiled virtual method #0 @ RVA 0x00238920 (16 instructions, 3 basic blocks)
    pub fn method_00(&mut self) {
    }

    /// Decompiled virtual method #1 @ RVA 0x000d3090 (1 instructions, 1 basic blocks)
    pub fn method_01(&mut self) {
    }

    /// Decompiled virtual method #2 @ RVA 0x000d3090 (1 instructions, 1 basic blocks)
    pub fn method_02(&mut self) {
    }

    /// Decompiled virtual method #3 @ RVA 0x00239100 (98 instructions, 9 basic blocks)
    pub fn method_03(&mut self) {
        // Writes to field at offset +0x0008
        // Writes to field at offset +0x0088
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

    /// Decompiled virtual method #11 @ RVA 0x002389f0 (24 instructions, 4 basic blocks)
    pub fn method_11(&mut self) {
    }

    /// Decompiled virtual method #12 @ RVA 0x00238a70 (118 instructions, 29 basic blocks)
    pub fn method_12(&mut self) {
    }

    /// Decompiled virtual method #13 @ RVA 0x00238fb0 (7 instructions, 2 basic blocks)
    pub fn method_13(&mut self) {
        // Writes to field at offset +0x001c
        // Writes to field at offset +0x001c
    }

}

impl CcClass for ccSaNameStr {
    fn class_name(&self) -> &'static str { Self::CLASS_NAME }
    fn vtable_rva(&self) -> u32 { Self::VTABLE_RVA }
    fn virtual_method_count(&self) -> usize { Self::METHOD_COUNT }
    fn base_classes(&self) -> &'static [&'static str] { Self::BASE_CLASSES }
}

/// Decompiled CC2 class: `ccSamplingTexChunk` (VTable RVA: 0x00877e00, 3 virtual methods, 19 recovered fields)
/// Inherits: ccTexChunk, ccChunkBasis
#[derive(Debug, Clone, Default)]
pub struct ccSamplingTexChunk {
    pub vtable_rva: u32,
    pub field_0x0000: u64,
    pub field_0x0008: u64,
    pub field_0x0018: u64,
    pub field_0x0020: u64,
    pub field_0x0028: u64,
    pub field_0x0030: u16,
    pub field_0x0032: u16,
    pub field_0x0034: u16,
    pub field_0x0036: u8,
    pub field_0x0037: u8,
    pub field_0x0038: u8,
    pub field_0x0039: u8,
    pub field_0x003a: u8,
    pub field_0x003d: u8,
    pub field_0x0040: u64,
    pub field_0x0048: u8,
    pub field_0x0049: u8,
    pub field_0x00d8: u32,
    pub field_0x00f8: u32,
}

impl ccSamplingTexChunk {
    pub const CLASS_NAME: &'static str = "ccSamplingTexChunk";
    pub const VTABLE_RVA: u32 = 0x00877e00;
    pub const METHOD_COUNT: usize = 3;
    pub const BASE_CLASSES: &'static [&'static str] = &["ccTexChunk", "ccChunkBasis"];
    pub const VIRTUAL_METHODS: &'static [u64] = &[0x000000018019c4d0, 0x000000018019c510, 0x000000018019c5a0];

    pub fn new() -> Self {
        Self {
            vtable_rva: Self::VTABLE_RVA,
            field_0x0000: Default::default(),
            field_0x0008: Default::default(),
            field_0x0018: Default::default(),
            field_0x0020: Default::default(),
            field_0x0028: Default::default(),
            field_0x0030: Default::default(),
            field_0x0032: Default::default(),
            field_0x0034: Default::default(),
            field_0x0036: Default::default(),
            field_0x0037: Default::default(),
            field_0x0038: Default::default(),
            field_0x0039: Default::default(),
            field_0x003a: Default::default(),
            field_0x003d: Default::default(),
            field_0x0040: Default::default(),
            field_0x0048: Default::default(),
            field_0x0049: Default::default(),
            field_0x00d8: Default::default(),
            field_0x00f8: Default::default(),
        }
    }

    /// Decompiled virtual method #0 @ RVA 0x0019c4d0 (18 instructions, 3 basic blocks)
    pub fn method_00(&mut self) {
        // Writes to field at offset +0x0000
    }

    /// Decompiled virtual method #1 @ RVA 0x0019c510 (35 instructions, 1 basic blocks)
    pub fn method_01(&mut self) {
        // Writes to field at offset +0x0008
        // Writes to field at offset +0x0018
        // Writes to field at offset +0x0020
        // Writes to field at offset +0x0028
        // Writes to field at offset +0x0030
        // Writes to field at offset +0x0032
        // Writes to field at offset +0x0034
        // Writes to field at offset +0x0036
        // Writes to field at offset +0x0037
        // Writes to field at offset +0x0038
        // Writes to field at offset +0x0039
        // Writes to field at offset +0x003a
        // Writes to field at offset +0x003d
        // Writes to field at offset +0x0040
        // Writes to field at offset +0x0048
        // Writes to field at offset +0x0049
        // Writes to field at offset +0x00d8
        // Writes to field at offset +0x00f8
    }

    /// Decompiled virtual method #2 @ RVA 0x0019c5a0 (57 instructions, 9 basic blocks)
    pub fn method_02(&mut self) {
        // Writes to field at offset +0x00d8
    }

}

impl CcClass for ccSamplingTexChunk {
    fn class_name(&self) -> &'static str { Self::CLASS_NAME }
    fn vtable_rva(&self) -> u32 { Self::VTABLE_RVA }
    fn virtual_method_count(&self) -> usize { Self::METHOD_COUNT }
    fn base_classes(&self) -> &'static [&'static str] { Self::BASE_CLASSES }
}

/// Decompiled CC2 class: `ccSaveData` (VTable RVA: 0x008cc060, 647 virtual methods, 0 recovered fields)
#[derive(Debug, Clone, Default)]
pub struct ccSaveData {
    pub vtable_rva: u32,
}

impl ccSaveData {
    pub const CLASS_NAME: &'static str = "ccSaveData";
    pub const VTABLE_RVA: u32 = 0x008cc060;
    pub const METHOD_COUNT: usize = 647;
    pub const BASE_CLASSES: &'static [&'static str] = &[];
    pub const VIRTUAL_METHODS: &'static [u64] = &[0x0000000180103e70, 0x0000000180103e70, 0x00000001800d3090, 0x00000001800d3090, 0x00000001800d3090, 0x00000001800d3090, 0x0000000180103e70, 0x00000001800d3090, 0x00000001800d3090, 0x00000001800d3090, 0x00000001800d3090, 0x00000001800d3090, 0x00000001800d3090, 0x00000001800d3090, 0x00000001800d3090, 0x00000001800d3090, 0x00000001800d3090, 0x00000001800d3090, 0x00000001800d3090, 0x00000001800d3090, 0x00000001805844e0, 0x0000000180103e70, 0x00000001800d3090, 0x00000001800d3090, 0x0000000180103e70, 0x00000001805844f0, 0x00000001800d3090, 0x0000000180103e70, 0x00000001800d3090, 0x0000000180103e80, 0x00000001800d3090, 0x0000000180103e80, 0x00000001800d6920, 0x0000000180103e80, 0x00000001800d3090, 0x0000000180103e80, 0x00000001800d3090, 0x00000001800d3090, 0x0000000180103e80, 0x00000001800d3090, 0x0000000180103e70, 0x0000000180103e70, 0x0000000180103e70, 0x0000000180103e70, 0x0000000180103e80, 0x0000000180103e80, 0x0000000180103e80, 0x00000001800d3090, 0x00000001800d3090, 0x0000000180103e80, 0x0000000180103e80, 0x00000001800d3090, 0x00000001800d3090, 0x0000000180103e70, 0x0000000180103e70, 0x0000000180103e70, 0x0000000180103e70, 0x0000000180103e70, 0x0000000180103e70, 0x0000000180103e80, 0x0000000180103e70, 0x0000000180103e70, 0x0000000180103e70, 0x0000000180103e70, 0x00000001800d3090, 0x00000001800d3090, 0x0000000180103e80, 0x0000000180103e80, 0x00000001800d6920, 0x00000001800d6920, 0x00000001800d3090, 0x0000000180103e80, 0x0000000180103e80, 0x00000001800d3090, 0x00000001800d3090, 0x00000001800d3090, 0x0000000180103e80, 0x00000001800d3090, 0x00000001800d3090, 0x00000001800d3090, 0x00000001800d3090, 0x00000001800d3090, 0x00000001800d3090, 0x00000001800d3090, 0x00000001800d3090, 0x0000000180103e80, 0x0000000180103e80, 0x0000000180103e80, 0x0000000180103e80, 0x0000000180103e80, 0x0000000180103e80, 0x0000000180103e70, 0x0000000180103e70, 0x0000000180103e80, 0x00000001800d3090, 0x00000001800d3090, 0x00000001800d3090, 0x0000000180103e80, 0x0000000180103e80, 0x0000000180103e80, 0x0000000180103e80, 0x0000000180103e80, 0x0000000180103e80, 0x0000000180103e80, 0x0000000180103e80, 0x0000000180103e80, 0x0000000180103e80, 0x0000000180103e80, 0x0000000180103e80, 0x00000001800d3090, 0x0000000180103e70, 0x0000000180103e70, 0x00000001800d3090, 0x0000000180103e80, 0x00000001800d3090, 0x0000000180103e80, 0x00000001800d3090, 0x0000000180103e80, 0x00000001800d6920, 0x00000001800d6920, 0x00000001800d3090, 0x00000001800d3090, 0x00000001800d6920, 0x00000001800d3090, 0x00000001800d3090, 0x00000001800d3090, 0x00000001800d3090, 0x00000001800d3090, 0x0000000180103e70, 0x0000000180103e80, 0x00000001800d3090, 0x0000000180103e80, 0x00000001800d6920, 0x0000000180103e80, 0x00000001800d3090, 0x0000000180103e80, 0x00000001800d3090, 0x00000001800d3090, 0x00000001800d3090, 0x0000000180103e80, 0x00000001800d3090, 0x00000001800d3090, 0x00000001800d3090, 0x0000000180103e80, 0x00000001800d3090, 0x00000001800d3090, 0x00000001800d3090, 0x0000000180103e80, 0x00000001800d3090, 0x00000001800d3090, 0x00000001800d3090, 0x0000000180103e80, 0x00000001800d3090, 0x00000001800d3090, 0x00000001800d3090, 0x00000001800d3090, 0x00000001800d6920, 0x00000001800d6920, 0x00000001800d6920, 0x00000001800d6920, 0x00000001800d6920, 0x00000001800d6920, 0x00000001800d6920, 0x0000000180103e80, 0x00000001800d6920, 0x0000000180103e80, 0x0000000180103e80, 0x0000000180103e80, 0x0000000180103e80, 0x00000001800d6920, 0x00000001800d6920, 0x00000001800d6920, 0x00000001800d6920, 0x0000000180103e80, 0x00000001800d3090, 0x00000001800d3090, 0x00000001800d3090, 0x00000001800d6920, 0x00000001800d6920, 0x00000001800d6920, 0x00000001800d6920, 0x00000001800d6920, 0x00000001800d6920, 0x00000001800d6920, 0x00000001800d6920, 0x00000001800d6920, 0x00000001800d6920, 0x00000001800d6920, 0x0000000180103e80, 0x0000000180103e80, 0x0000000180103e80, 0x00000001800d3090, 0x00000001800d3090, 0x00000001800d3090, 0x00000001800d6920, 0x00000001800d3090, 0x00000001800d6920, 0x00000001800d3090, 0x0000000180103e80, 0x0000000180103e80, 0x0000000180103e80, 0x0000000180103e80, 0x00000001800d6920, 0x00000001800d6920, 0x00000001800d6920, 0x00000001800d6920, 0x00000001800d6920, 0x00000001800d6920, 0x0000000180103e80, 0x0000000180103e80, 0x00000001800d6920, 0x0000000180103e80, 0x00000001800d3090, 0x00000001800d3090, 0x00000001800d3090, 0x00000001800d3090, 0x00000001800d3090, 0x0000000180103e70, 0x00000001800d6920, 0x00000001800d6920, 0x00000001800d3090, 0x0000000180103e80, 0x0000000180103e70, 0x00000001800d3090, 0x00000001800d3090, 0x00000001800d3090, 0x0000000180103e80, 0x00000001800d6920, 0x00000001800d6920, 0x00000001800d6920, 0x0000000180103e80, 0x0000000180103e80, 0x0000000180103e80, 0x00000001800d3090, 0x00000001800d3090, 0x00000001800d3090, 0x00000001800d6920, 0x00000001800d3090, 0x00000001800d3090, 0x00000001800d3090, 0x00000001800d6920, 0x00000001800d3090, 0x00000001800d3090, 0x0000000180103e80, 0x00000001800d3090, 0x00000001800d3090, 0x00000001800d3090, 0x00000001800d3090, 0x0000000180103e80, 0x00000001800d6920, 0x00000001800d3090, 0x00000001800d3090, 0x00000001800d3090, 0x00000001800d3090, 0x0000000180103e80, 0x00000001800d3090, 0x00000001800d3090, 0x0000000180103e80, 0x00000001800d3090, 0x0000000180103e80, 0x00000001800d6920, 0x00000001800d6920, 0x00000001800d6920, 0x00000001800d6920, 0x00000001800d6920, 0x00000001800d3090, 0x0000000180103e80, 0x00000001800d6920, 0x00000001800d6920, 0x00000001800d6920, 0x00000001800d6920, 0x00000001800d6920, 0x00000001800d6920, 0x00000001800d6920, 0x00000001800d3090, 0x00000001800d3090, 0x00000001800d3090, 0x00000001800d3090, 0x00000001800d3090, 0x00000001800d6920, 0x00000001801499d0, 0x00000001801499d0, 0x00000001801499d0, 0x00000001801499d0, 0x000000018073e2c0, 0x000000018073e2c0, 0x000000018073e2c0, 0x000000018073e2c0, 0x000000018073e2c0, 0x000000018073e2c0, 0x00000001800d3090, 0x0000000180103e80, 0x00000001800d6920, 0x00000001800d6920, 0x00000001800d6920, 0x00000001800d6920, 0x00000001800d6920, 0x00000001800d6920, 0x00000001800d6920, 0x00000001800d6920, 0x0000000180103e80, 0x0000000180103e80, 0x0000000180103e80, 0x00000001800d6920, 0x00000001800d6920, 0x00000001800d6920, 0x00000001800d3090, 0x00000001800d3090, 0x00000001800d3090, 0x00000001800d3090, 0x0000000180584500, 0x00000001800d3090, 0x0000000180103e80, 0x00000001800d3090, 0x0000000180584500, 0x00000001800d3090, 0x00000001800d3090, 0x00000001800d6920, 0x00000001800d3090, 0x00000001800d3090, 0x00000001800d6920, 0x0000000180103e80, 0x00000001800d3090, 0x00000001800d3090, 0x00000001800d3090, 0x00000001800d3090, 0x0000000180103e80, 0x00000001800d3090, 0x0000000180103e80, 0x00000001800d3090, 0x0000000180103e80, 0x00000001800d3090, 0x0000000180103e80, 0x00000001800d3090, 0x00000001800d3090, 0x00000001800d6920, 0x00000001800d3090, 0x00000001800d3090, 0x00000001800d3090, 0x00000001800d3090, 0x0000000180103e80, 0x0000000180103e80, 0x00000001800d6920, 0x00000001800d6920, 0x00000001800d6920, 0x00000001800d3090, 0x0000000180103e80, 0x00000001800d3090, 0x00000001800d3090, 0x00000001800d6920, 0x00000001800d3090, 0x00000001800d3090, 0x00000001800d6920, 0x00000001800d3090, 0x00000001800d3090, 0x00000001800d6920, 0x00000001800d3090, 0x00000001800d3090, 0x00000001800d6920, 0x00000001800d3090, 0x00000001800d3090, 0x00000001800d6920, 0x00000001800d3090, 0x00000001800d3090, 0x00000001800d6920, 0x0000000180103e80, 0x00000001800d3090, 0x00000001800d3090, 0x0000000180103e80, 0x00000001800d3090, 0x00000001800d3090, 0x00000001800d6920, 0x00000001800d6920, 0x00000001800d3090, 0x00000001800d3090, 0x00000001800d3090, 0x00000001800d3090, 0x00000001800d3090, 0x00000001800d3090, 0x00000001800d3090, 0x00000001800d3090, 0x0000000180103e80, 0x0000000180103e80, 0x0000000180103e80, 0x0000000180103e70, 0x0000000180103e70, 0x0000000180103e70, 0x0000000180103e80, 0x00000001800d3090, 0x0000000180103e70, 0x0000000180103e80, 0x00000001800d3090, 0x0000000180103e70, 0x0000000180103e70, 0x00000001800d6920, 0x00000001800d3090, 0x00000001800d3090, 0x00000001800d3090, 0x00000001800d3090, 0x00000001800d3090, 0x0000000180103e80, 0x00000001800d3090, 0x0000000180103e80, 0x0000000180103e70, 0x00000001800d3090, 0x00000001800d3090, 0x00000001800d6920, 0x0000000180103e70, 0x00000001800d3090, 0x0000000180103e70, 0x0000000180103e80, 0x00000001800d3090, 0x0000000180103e80, 0x00000001800d3090, 0x0000000180103e80, 0x00000001800d3090, 0x00000001800d3090, 0x00000001800d3090, 0x00000001800d3090, 0x00000001800d3090, 0x00000001800d6920, 0x00000001800d6920, 0x00000001800d3090, 0x00000001800d3090, 0x00000001800d3090, 0x00000001800d6920, 0x00000001800d6920, 0x00000001800d3090, 0x00000001800d3090, 0x00000001800d3090, 0x00000001800d6920, 0x00000001800d6920, 0x00000001800d3090, 0x00000001800d3090, 0x00000001800d3090, 0x00000001800d6920, 0x00000001800d6920, 0x00000001800d3090, 0x00000001800d3090, 0x00000001800d6920, 0x00000001800d3090, 0x00000001800d3090, 0x00000001800d6920, 0x00000001800d3090, 0x00000001800d3090, 0x00000001800d6920, 0x00000001800d3090, 0x00000001800d3090, 0x00000001800d3090, 0x00000001800d6920, 0x00000001800d3090, 0x00000001800d3090, 0x00000001800d6920, 0x00000001800d3090, 0x00000001800d3090, 0x00000001800d6920, 0x00000001800d6920, 0x00000001800d3090, 0x00000001800d3090, 0x00000001800d6920, 0x00000001800d3090, 0x00000001800d6920, 0x0000000180103e70, 0x00000001800d3090, 0x0000000180103e80, 0x00000001800d3090, 0x0000000180103e70, 0x00000001800d3090, 0x00000001800d3090, 0x00000001800d3090, 0x00000001800d3090, 0x00000001800d3090, 0x0000000180103e80, 0x00000001800d6920, 0x00000001800d3090, 0x0000000180584510, 0x00000001800d3090, 0x00000001800d3090, 0x00000001800d6920, 0x00000001800d3090, 0x0000000180103e70, 0x00000001800d3090, 0x0000000180103e70, 0x0000000180584520, 0x00000001800d3090, 0x0000000180103e80, 0x0000000180584530, 0x00000001800d3090, 0x00000001800d6920, 0x00000001800d3090, 0x00000001800d6920, 0x00000001800d3090, 0x00000001800d3090, 0x0000000180103e80, 0x00000001800d3090, 0x00000001800d3090, 0x0000000180103e80, 0x00000001800d3090, 0x00000001800d3090, 0x0000000180103e80, 0x0000000180103e80, 0x00000001800d3090, 0x00000001800d3090, 0x00000001800d3090, 0x00000001800d3090, 0x00000001800d3090, 0x00000001800d3090, 0x0000000180103e80, 0x00000001800d3090, 0x00000001800d3090, 0x00000001800d3090, 0x0000000180103e80, 0x0000000180103e80, 0x00000001800d3090, 0x00000001800d3090, 0x00000001800d3090, 0x00000001800d3090, 0x00000001800d3090, 0x00000001800d3090, 0x0000000180103e80, 0x0000000180103e80, 0x0000000180103e80, 0x0000000180103e80, 0x00000001800d3090, 0x00000001800d3090, 0x00000001800d3090, 0x00000001800d3090, 0x00000001800d3090, 0x00000001800d6920, 0x00000001800d3090, 0x00000001800d3090, 0x00000001800d3090, 0x00000001800d6920, 0x00000001800d3090, 0x00000001800d3090, 0x00000001800d3090, 0x00000001800d3090, 0x00000001800d3090, 0x00000001800d3090, 0x00000001800d3090, 0x0000000180103e80, 0x0000000180103e80, 0x00000001800d3090, 0x00000001800d3090, 0x00000001800d3090, 0x00000001800d3090, 0x00000001800d3090, 0x00000001800d6920, 0x00000001800d6920, 0x00000001800d6920, 0x00000001800d3090, 0x00000001800d3090, 0x00000001800d3090, 0x00000001800d3090, 0x00000001800d6920, 0x00000001800d3090, 0x00000001800d3090, 0x00000001800d6920, 0x00000001800d3090, 0x00000001800d6920, 0x00000001800d3090, 0x00000001800d3090, 0x00000001800d6920, 0x00000001800d3090, 0x00000001800d3090, 0x00000001800d6920, 0x00000001800d3090, 0x00000001800d3090, 0x00000001800d6920, 0x00000001800d3090, 0x00000001800d3090, 0x0000000180103e70, 0x00000001800d3090, 0x00000001800d3090, 0x00000001800d3090, 0x0000000180103e70, 0x00000001800d3090, 0x0000000180103e70, 0x00000001800d3090, 0x00000001800d3090, 0x0000000180103e80, 0x00000001800d3090, 0x00000001800d3090, 0x0000000180103e80, 0x00000001800d3090, 0x00000001800d3090, 0x0000000180103e80, 0x0000000180103e70, 0x00000001800d3090, 0x0000000180103e80, 0x00000001800d3090, 0x00000001800d3090, 0x0000000180103e80, 0x00000001800d3090, 0x00000001800d6920, 0x0000000180103e80, 0x00000001800d3090, 0x00000001800d3090, 0x0000000180103e80, 0x00000001800d3090, 0x00000001800d3090, 0x0000000180103e80, 0x0000000180103e80, 0x00000001800d6920, 0x00000001800d6920, 0x0000000180103e80, 0x00000001800d3090, 0x0000000180103e80, 0x0000000180103e80, 0x00000001800d3090, 0x00000001800d3090, 0x0000000180103e80, 0x00000001800d3090, 0x00000001800d3090, 0x0000000180103e80, 0x00000001800d3090, 0x0000000180103e80, 0x00000001800d3090, 0x0000000180103e70, 0x00000001800d6920, 0x00000001800d3090, 0x0000000180103e80, 0x00000001800d3090, 0x00000001800d3090, 0x00000001800d3090, 0x00000001800d3090, 0x0000000180103e80, 0x0000000180103e80, 0x00000001800d3090, 0x0000000180584510, 0x00000001800d3090, 0x0000000180584510, 0x0000000180103e80, 0x00000001800d3090, 0x00000001800d3090, 0x0000000180103e80, 0x00000001800d3090, 0x00000001800d3090, 0x00000001800d3090, 0x0000000180103e70, 0x00000001800d3090, 0x00000001800d6920, 0x00000001800d6920, 0x0000000180103e70, 0x00000001800d3090, 0x0000000180103e70, 0x00000001800d3090];

    pub fn new() -> Self {
        Self {
            vtable_rva: Self::VTABLE_RVA,
        }
    }

    /// Decompiled virtual method #0 @ RVA 0x00103e70 (2 instructions, 1 basic blocks)
    pub fn method_00(&mut self) {
    }

    /// Decompiled virtual method #1 @ RVA 0x00103e70 (2 instructions, 1 basic blocks)
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

    /// Decompiled virtual method #5 @ RVA 0x000d3090 (1 instructions, 1 basic blocks)
    pub fn method_05(&mut self) {
    }

    /// Decompiled virtual method #6 @ RVA 0x00103e70 (2 instructions, 1 basic blocks)
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

    /// Decompiled virtual method #11 @ RVA 0x000d3090 (1 instructions, 1 basic blocks)
    pub fn method_11(&mut self) {
    }

    /// Decompiled virtual method #12 @ RVA 0x000d3090 (1 instructions, 1 basic blocks)
    pub fn method_12(&mut self) {
    }

    /// Decompiled virtual method #13 @ RVA 0x000d3090 (1 instructions, 1 basic blocks)
    pub fn method_13(&mut self) {
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

    /// Decompiled virtual method #17 @ RVA 0x000d3090 (1 instructions, 1 basic blocks)
    pub fn method_17(&mut self) {
    }

    /// Decompiled virtual method #18 @ RVA 0x000d3090 (1 instructions, 1 basic blocks)
    pub fn method_18(&mut self) {
    }

    /// Decompiled virtual method #19 @ RVA 0x000d3090 (1 instructions, 1 basic blocks)
    pub fn method_19(&mut self) {
    }

    /// Decompiled virtual method #20 @ RVA 0x005844e0 (2 instructions, 1 basic blocks)
    pub fn method_20(&mut self) {
    }

    /// Decompiled virtual method #21 @ RVA 0x00103e70 (2 instructions, 1 basic blocks)
    pub fn method_21(&mut self) {
    }

    /// Decompiled virtual method #22 @ RVA 0x000d3090 (1 instructions, 1 basic blocks)
    pub fn method_22(&mut self) {
    }

    /// Decompiled virtual method #23 @ RVA 0x000d3090 (1 instructions, 1 basic blocks)
    pub fn method_23(&mut self) {
    }

    /// Decompiled virtual method #24 @ RVA 0x00103e70 (2 instructions, 1 basic blocks)
    pub fn method_24(&mut self) {
    }

    /// Decompiled virtual method #25 @ RVA 0x005844f0 (2 instructions, 1 basic blocks)
    pub fn method_25(&mut self) {
    }

    /// Decompiled virtual method #26 @ RVA 0x000d3090 (1 instructions, 1 basic blocks)
    pub fn method_26(&mut self) {
    }

    /// Decompiled virtual method #27 @ RVA 0x00103e70 (2 instructions, 1 basic blocks)
    pub fn method_27(&mut self) {
    }

    /// Decompiled virtual method #28 @ RVA 0x000d3090 (1 instructions, 1 basic blocks)
    pub fn method_28(&mut self) {
    }

    /// Decompiled virtual method #29 @ RVA 0x00103e80 (2 instructions, 1 basic blocks)
    pub fn method_29(&mut self) {
    }

    /// Decompiled virtual method #30 @ RVA 0x000d3090 (1 instructions, 1 basic blocks)
    pub fn method_30(&mut self) {
    }

    /// Decompiled virtual method #31 @ RVA 0x00103e80 (2 instructions, 1 basic blocks)
    pub fn method_31(&mut self) {
    }

    /// Decompiled virtual method #32 @ RVA 0x000d6920 (2 instructions, 1 basic blocks)
    pub fn method_32(&mut self) {
    }

    /// Decompiled virtual method #33 @ RVA 0x00103e80 (2 instructions, 1 basic blocks)
    pub fn method_33(&mut self) {
    }

    /// Decompiled virtual method #34 @ RVA 0x000d3090 (1 instructions, 1 basic blocks)
    pub fn method_34(&mut self) {
    }

    /// Decompiled virtual method #35 @ RVA 0x00103e80 (2 instructions, 1 basic blocks)
    pub fn method_35(&mut self) {
    }

    /// Decompiled virtual method #36 @ RVA 0x000d3090 (1 instructions, 1 basic blocks)
    pub fn method_36(&mut self) {
    }

    /// Decompiled virtual method #37 @ RVA 0x000d3090 (1 instructions, 1 basic blocks)
    pub fn method_37(&mut self) {
    }

    /// Decompiled virtual method #38 @ RVA 0x00103e80 (2 instructions, 1 basic blocks)
    pub fn method_38(&mut self) {
    }

    /// Decompiled virtual method #39 @ RVA 0x000d3090 (1 instructions, 1 basic blocks)
    pub fn method_39(&mut self) {
    }

    /// Decompiled virtual method #40 @ RVA 0x00103e70 (2 instructions, 1 basic blocks)
    pub fn method_40(&mut self) {
    }

    /// Decompiled virtual method #41 @ RVA 0x00103e70 (2 instructions, 1 basic blocks)
    pub fn method_41(&mut self) {
    }

    /// Decompiled virtual method #42 @ RVA 0x00103e70 (2 instructions, 1 basic blocks)
    pub fn method_42(&mut self) {
    }

    /// Decompiled virtual method #43 @ RVA 0x00103e70 (2 instructions, 1 basic blocks)
    pub fn method_43(&mut self) {
    }

    /// Decompiled virtual method #44 @ RVA 0x00103e80 (2 instructions, 1 basic blocks)
    pub fn method_44(&mut self) {
    }

    /// Decompiled virtual method #45 @ RVA 0x00103e80 (2 instructions, 1 basic blocks)
    pub fn method_45(&mut self) {
    }

    /// Decompiled virtual method #46 @ RVA 0x00103e80 (2 instructions, 1 basic blocks)
    pub fn method_46(&mut self) {
    }

    /// Decompiled virtual method #47 @ RVA 0x000d3090 (1 instructions, 1 basic blocks)
    pub fn method_47(&mut self) {
    }

    /// Decompiled virtual method #48 @ RVA 0x000d3090 (1 instructions, 1 basic blocks)
    pub fn method_48(&mut self) {
    }

    /// Decompiled virtual method #49 @ RVA 0x00103e80 (2 instructions, 1 basic blocks)
    pub fn method_49(&mut self) {
    }

    /// Decompiled virtual method #50 @ RVA 0x00103e80 (2 instructions, 1 basic blocks)
    pub fn method_50(&mut self) {
    }

    /// Decompiled virtual method #51 @ RVA 0x000d3090 (1 instructions, 1 basic blocks)
    pub fn method_51(&mut self) {
    }

    /// Decompiled virtual method #52 @ RVA 0x000d3090 (1 instructions, 1 basic blocks)
    pub fn method_52(&mut self) {
    }

    /// Decompiled virtual method #53 @ RVA 0x00103e70 (2 instructions, 1 basic blocks)
    pub fn method_53(&mut self) {
    }

    /// Decompiled virtual method #54 @ RVA 0x00103e70 (2 instructions, 1 basic blocks)
    pub fn method_54(&mut self) {
    }

    /// Decompiled virtual method #55 @ RVA 0x00103e70 (2 instructions, 1 basic blocks)
    pub fn method_55(&mut self) {
    }

    /// Decompiled virtual method #56 @ RVA 0x00103e70 (2 instructions, 1 basic blocks)
    pub fn method_56(&mut self) {
    }

    /// Decompiled virtual method #57 @ RVA 0x00103e70 (2 instructions, 1 basic blocks)
    pub fn method_57(&mut self) {
    }

    /// Decompiled virtual method #58 @ RVA 0x00103e70 (2 instructions, 1 basic blocks)
    pub fn method_58(&mut self) {
    }

    /// Decompiled virtual method #59 @ RVA 0x00103e80 (2 instructions, 1 basic blocks)
    pub fn method_59(&mut self) {
    }

    /// Decompiled virtual method #60 @ RVA 0x00103e70 (2 instructions, 1 basic blocks)
    pub fn method_60(&mut self) {
    }

    /// Decompiled virtual method #61 @ RVA 0x00103e70 (2 instructions, 1 basic blocks)
    pub fn method_61(&mut self) {
    }

    /// Decompiled virtual method #62 @ RVA 0x00103e70 (2 instructions, 1 basic blocks)
    pub fn method_62(&mut self) {
    }

    /// Decompiled virtual method #63 @ RVA 0x00103e70 (2 instructions, 1 basic blocks)
    pub fn method_63(&mut self) {
    }

    /// Decompiled virtual method #64 @ RVA 0x000d3090 (1 instructions, 1 basic blocks)
    pub fn method_64(&mut self) {
    }

    /// Decompiled virtual method #65 @ RVA 0x000d3090 (1 instructions, 1 basic blocks)
    pub fn method_65(&mut self) {
    }

    /// Decompiled virtual method #66 @ RVA 0x00103e80 (2 instructions, 1 basic blocks)
    pub fn method_66(&mut self) {
    }

    /// Decompiled virtual method #67 @ RVA 0x00103e80 (2 instructions, 1 basic blocks)
    pub fn method_67(&mut self) {
    }

    /// Decompiled virtual method #68 @ RVA 0x000d6920 (2 instructions, 1 basic blocks)
    pub fn method_68(&mut self) {
    }

    /// Decompiled virtual method #69 @ RVA 0x000d6920 (2 instructions, 1 basic blocks)
    pub fn method_69(&mut self) {
    }

    /// Decompiled virtual method #70 @ RVA 0x000d3090 (1 instructions, 1 basic blocks)
    pub fn method_70(&mut self) {
    }

    /// Decompiled virtual method #71 @ RVA 0x00103e80 (2 instructions, 1 basic blocks)
    pub fn method_71(&mut self) {
    }

    /// Decompiled virtual method #72 @ RVA 0x00103e80 (2 instructions, 1 basic blocks)
    pub fn method_72(&mut self) {
    }

    /// Decompiled virtual method #73 @ RVA 0x000d3090 (1 instructions, 1 basic blocks)
    pub fn method_73(&mut self) {
    }

    /// Decompiled virtual method #74 @ RVA 0x000d3090 (1 instructions, 1 basic blocks)
    pub fn method_74(&mut self) {
    }

    /// Decompiled virtual method #75 @ RVA 0x000d3090 (1 instructions, 1 basic blocks)
    pub fn method_75(&mut self) {
    }

    /// Decompiled virtual method #76 @ RVA 0x00103e80 (2 instructions, 1 basic blocks)
    pub fn method_76(&mut self) {
    }

    /// Decompiled virtual method #77 @ RVA 0x000d3090 (1 instructions, 1 basic blocks)
    pub fn method_77(&mut self) {
    }

    /// Decompiled virtual method #78 @ RVA 0x000d3090 (1 instructions, 1 basic blocks)
    pub fn method_78(&mut self) {
    }

    /// Decompiled virtual method #79 @ RVA 0x000d3090 (1 instructions, 1 basic blocks)
    pub fn method_79(&mut self) {
    }

    /// Decompiled virtual method #80 @ RVA 0x000d3090 (1 instructions, 1 basic blocks)
    pub fn method_80(&mut self) {
    }

    /// Decompiled virtual method #81 @ RVA 0x000d3090 (1 instructions, 1 basic blocks)
    pub fn method_81(&mut self) {
    }

    /// Decompiled virtual method #82 @ RVA 0x000d3090 (1 instructions, 1 basic blocks)
    pub fn method_82(&mut self) {
    }

    /// Decompiled virtual method #83 @ RVA 0x000d3090 (1 instructions, 1 basic blocks)
    pub fn method_83(&mut self) {
    }

    /// Decompiled virtual method #84 @ RVA 0x000d3090 (1 instructions, 1 basic blocks)
    pub fn method_84(&mut self) {
    }

    /// Decompiled virtual method #85 @ RVA 0x00103e80 (2 instructions, 1 basic blocks)
    pub fn method_85(&mut self) {
    }

    /// Decompiled virtual method #86 @ RVA 0x00103e80 (2 instructions, 1 basic blocks)
    pub fn method_86(&mut self) {
    }

    /// Decompiled virtual method #87 @ RVA 0x00103e80 (2 instructions, 1 basic blocks)
    pub fn method_87(&mut self) {
    }

    /// Decompiled virtual method #88 @ RVA 0x00103e80 (2 instructions, 1 basic blocks)
    pub fn method_88(&mut self) {
    }

    /// Decompiled virtual method #89 @ RVA 0x00103e80 (2 instructions, 1 basic blocks)
    pub fn method_89(&mut self) {
    }

    /// Decompiled virtual method #90 @ RVA 0x00103e80 (2 instructions, 1 basic blocks)
    pub fn method_90(&mut self) {
    }

    /// Decompiled virtual method #91 @ RVA 0x00103e70 (2 instructions, 1 basic blocks)
    pub fn method_91(&mut self) {
    }

    /// Decompiled virtual method #92 @ RVA 0x00103e70 (2 instructions, 1 basic blocks)
    pub fn method_92(&mut self) {
    }

    /// Decompiled virtual method #93 @ RVA 0x00103e80 (2 instructions, 1 basic blocks)
    pub fn method_93(&mut self) {
    }

    /// Decompiled virtual method #94 @ RVA 0x000d3090 (1 instructions, 1 basic blocks)
    pub fn method_94(&mut self) {
    }

    /// Decompiled virtual method #95 @ RVA 0x000d3090 (1 instructions, 1 basic blocks)
    pub fn method_95(&mut self) {
    }

    /// Decompiled virtual method #96 @ RVA 0x000d3090 (1 instructions, 1 basic blocks)
    pub fn method_96(&mut self) {
    }

    /// Decompiled virtual method #97 @ RVA 0x00103e80 (2 instructions, 1 basic blocks)
    pub fn method_97(&mut self) {
    }

    /// Decompiled virtual method #98 @ RVA 0x00103e80 (2 instructions, 1 basic blocks)
    pub fn method_98(&mut self) {
    }

    /// Decompiled virtual method #99 @ RVA 0x00103e80 (2 instructions, 1 basic blocks)
    pub fn method_99(&mut self) {
    }

    /// Decompiled virtual method #100 @ RVA 0x00103e80 (2 instructions, 1 basic blocks)
    pub fn method_100(&mut self) {
    }

    /// Decompiled virtual method #101 @ RVA 0x00103e80 (2 instructions, 1 basic blocks)
    pub fn method_101(&mut self) {
    }

    /// Decompiled virtual method #102 @ RVA 0x00103e80 (2 instructions, 1 basic blocks)
    pub fn method_102(&mut self) {
    }

    /// Decompiled virtual method #103 @ RVA 0x00103e80 (2 instructions, 1 basic blocks)
    pub fn method_103(&mut self) {
    }

    /// Decompiled virtual method #104 @ RVA 0x00103e80 (2 instructions, 1 basic blocks)
    pub fn method_104(&mut self) {
    }

    /// Decompiled virtual method #105 @ RVA 0x00103e80 (2 instructions, 1 basic blocks)
    pub fn method_105(&mut self) {
    }

    /// Decompiled virtual method #106 @ RVA 0x00103e80 (2 instructions, 1 basic blocks)
    pub fn method_106(&mut self) {
    }

    /// Decompiled virtual method #107 @ RVA 0x00103e80 (2 instructions, 1 basic blocks)
    pub fn method_107(&mut self) {
    }

    /// Decompiled virtual method #108 @ RVA 0x00103e80 (2 instructions, 1 basic blocks)
    pub fn method_108(&mut self) {
    }

    /// Decompiled virtual method #109 @ RVA 0x000d3090 (1 instructions, 1 basic blocks)
    pub fn method_109(&mut self) {
    }

    /// Decompiled virtual method #110 @ RVA 0x00103e70 (2 instructions, 1 basic blocks)
    pub fn method_110(&mut self) {
    }

    /// Decompiled virtual method #111 @ RVA 0x00103e70 (2 instructions, 1 basic blocks)
    pub fn method_111(&mut self) {
    }

    /// Decompiled virtual method #112 @ RVA 0x000d3090 (1 instructions, 1 basic blocks)
    pub fn method_112(&mut self) {
    }

    /// Decompiled virtual method #113 @ RVA 0x00103e80 (2 instructions, 1 basic blocks)
    pub fn method_113(&mut self) {
    }

    /// Decompiled virtual method #114 @ RVA 0x000d3090 (1 instructions, 1 basic blocks)
    pub fn method_114(&mut self) {
    }

    /// Decompiled virtual method #115 @ RVA 0x00103e80 (2 instructions, 1 basic blocks)
    pub fn method_115(&mut self) {
    }

    /// Decompiled virtual method #116 @ RVA 0x000d3090 (1 instructions, 1 basic blocks)
    pub fn method_116(&mut self) {
    }

    /// Decompiled virtual method #117 @ RVA 0x00103e80 (2 instructions, 1 basic blocks)
    pub fn method_117(&mut self) {
    }

    /// Decompiled virtual method #118 @ RVA 0x000d6920 (2 instructions, 1 basic blocks)
    pub fn method_118(&mut self) {
    }

    /// Decompiled virtual method #119 @ RVA 0x000d6920 (2 instructions, 1 basic blocks)
    pub fn method_119(&mut self) {
    }

    /// Decompiled virtual method #120 @ RVA 0x000d3090 (1 instructions, 1 basic blocks)
    pub fn method_120(&mut self) {
    }

    /// Decompiled virtual method #121 @ RVA 0x000d3090 (1 instructions, 1 basic blocks)
    pub fn method_121(&mut self) {
    }

    /// Decompiled virtual method #122 @ RVA 0x000d6920 (2 instructions, 1 basic blocks)
    pub fn method_122(&mut self) {
    }

    /// Decompiled virtual method #123 @ RVA 0x000d3090 (1 instructions, 1 basic blocks)
    pub fn method_123(&mut self) {
    }

    /// Decompiled virtual method #124 @ RVA 0x000d3090 (1 instructions, 1 basic blocks)
    pub fn method_124(&mut self) {
    }

    /// Decompiled virtual method #125 @ RVA 0x000d3090 (1 instructions, 1 basic blocks)
    pub fn method_125(&mut self) {
    }

    /// Decompiled virtual method #126 @ RVA 0x000d3090 (1 instructions, 1 basic blocks)
    pub fn method_126(&mut self) {
    }

    /// Decompiled virtual method #127 @ RVA 0x000d3090 (1 instructions, 1 basic blocks)
    pub fn method_127(&mut self) {
    }

    /// Decompiled virtual method #128 @ RVA 0x00103e70 (2 instructions, 1 basic blocks)
    pub fn method_128(&mut self) {
    }

    /// Decompiled virtual method #129 @ RVA 0x00103e80 (2 instructions, 1 basic blocks)
    pub fn method_129(&mut self) {
    }

    /// Decompiled virtual method #130 @ RVA 0x000d3090 (1 instructions, 1 basic blocks)
    pub fn method_130(&mut self) {
    }

    /// Decompiled virtual method #131 @ RVA 0x00103e80 (2 instructions, 1 basic blocks)
    pub fn method_131(&mut self) {
    }

    /// Decompiled virtual method #132 @ RVA 0x000d6920 (2 instructions, 1 basic blocks)
    pub fn method_132(&mut self) {
    }

    /// Decompiled virtual method #133 @ RVA 0x00103e80 (2 instructions, 1 basic blocks)
    pub fn method_133(&mut self) {
    }

    /// Decompiled virtual method #134 @ RVA 0x000d3090 (1 instructions, 1 basic blocks)
    pub fn method_134(&mut self) {
    }

    /// Decompiled virtual method #135 @ RVA 0x00103e80 (2 instructions, 1 basic blocks)
    pub fn method_135(&mut self) {
    }

    /// Decompiled virtual method #136 @ RVA 0x000d3090 (1 instructions, 1 basic blocks)
    pub fn method_136(&mut self) {
    }

    /// Decompiled virtual method #137 @ RVA 0x000d3090 (1 instructions, 1 basic blocks)
    pub fn method_137(&mut self) {
    }

    /// Decompiled virtual method #138 @ RVA 0x000d3090 (1 instructions, 1 basic blocks)
    pub fn method_138(&mut self) {
    }

    /// Decompiled virtual method #139 @ RVA 0x00103e80 (2 instructions, 1 basic blocks)
    pub fn method_139(&mut self) {
    }

    /// Decompiled virtual method #140 @ RVA 0x000d3090 (1 instructions, 1 basic blocks)
    pub fn method_140(&mut self) {
    }

    /// Decompiled virtual method #141 @ RVA 0x000d3090 (1 instructions, 1 basic blocks)
    pub fn method_141(&mut self) {
    }

    /// Decompiled virtual method #142 @ RVA 0x000d3090 (1 instructions, 1 basic blocks)
    pub fn method_142(&mut self) {
    }

    /// Decompiled virtual method #143 @ RVA 0x00103e80 (2 instructions, 1 basic blocks)
    pub fn method_143(&mut self) {
    }

    /// Decompiled virtual method #144 @ RVA 0x000d3090 (1 instructions, 1 basic blocks)
    pub fn method_144(&mut self) {
    }

    /// Decompiled virtual method #145 @ RVA 0x000d3090 (1 instructions, 1 basic blocks)
    pub fn method_145(&mut self) {
    }

    /// Decompiled virtual method #146 @ RVA 0x000d3090 (1 instructions, 1 basic blocks)
    pub fn method_146(&mut self) {
    }

    /// Decompiled virtual method #147 @ RVA 0x00103e80 (2 instructions, 1 basic blocks)
    pub fn method_147(&mut self) {
    }

    /// Decompiled virtual method #148 @ RVA 0x000d3090 (1 instructions, 1 basic blocks)
    pub fn method_148(&mut self) {
    }

    /// Decompiled virtual method #149 @ RVA 0x000d3090 (1 instructions, 1 basic blocks)
    pub fn method_149(&mut self) {
    }

    /// Decompiled virtual method #150 @ RVA 0x000d3090 (1 instructions, 1 basic blocks)
    pub fn method_150(&mut self) {
    }

    /// Decompiled virtual method #151 @ RVA 0x00103e80 (2 instructions, 1 basic blocks)
    pub fn method_151(&mut self) {
    }

    /// Decompiled virtual method #152 @ RVA 0x000d3090 (1 instructions, 1 basic blocks)
    pub fn method_152(&mut self) {
    }

    /// Decompiled virtual method #153 @ RVA 0x000d3090 (1 instructions, 1 basic blocks)
    pub fn method_153(&mut self) {
    }

    /// Decompiled virtual method #154 @ RVA 0x000d3090 (1 instructions, 1 basic blocks)
    pub fn method_154(&mut self) {
    }

    /// Decompiled virtual method #155 @ RVA 0x000d3090 (1 instructions, 1 basic blocks)
    pub fn method_155(&mut self) {
    }

    /// Decompiled virtual method #156 @ RVA 0x000d6920 (2 instructions, 1 basic blocks)
    pub fn method_156(&mut self) {
    }

    /// Decompiled virtual method #157 @ RVA 0x000d6920 (2 instructions, 1 basic blocks)
    pub fn method_157(&mut self) {
    }

    /// Decompiled virtual method #158 @ RVA 0x000d6920 (2 instructions, 1 basic blocks)
    pub fn method_158(&mut self) {
    }

    /// Decompiled virtual method #159 @ RVA 0x000d6920 (2 instructions, 1 basic blocks)
    pub fn method_159(&mut self) {
    }

    /// Decompiled virtual method #160 @ RVA 0x000d6920 (2 instructions, 1 basic blocks)
    pub fn method_160(&mut self) {
    }

    /// Decompiled virtual method #161 @ RVA 0x000d6920 (2 instructions, 1 basic blocks)
    pub fn method_161(&mut self) {
    }

    /// Decompiled virtual method #162 @ RVA 0x000d6920 (2 instructions, 1 basic blocks)
    pub fn method_162(&mut self) {
    }

    /// Decompiled virtual method #163 @ RVA 0x00103e80 (2 instructions, 1 basic blocks)
    pub fn method_163(&mut self) {
    }

    /// Decompiled virtual method #164 @ RVA 0x000d6920 (2 instructions, 1 basic blocks)
    pub fn method_164(&mut self) {
    }

    /// Decompiled virtual method #165 @ RVA 0x00103e80 (2 instructions, 1 basic blocks)
    pub fn method_165(&mut self) {
    }

    /// Decompiled virtual method #166 @ RVA 0x00103e80 (2 instructions, 1 basic blocks)
    pub fn method_166(&mut self) {
    }

    /// Decompiled virtual method #167 @ RVA 0x00103e80 (2 instructions, 1 basic blocks)
    pub fn method_167(&mut self) {
    }

    /// Decompiled virtual method #168 @ RVA 0x00103e80 (2 instructions, 1 basic blocks)
    pub fn method_168(&mut self) {
    }

    /// Decompiled virtual method #169 @ RVA 0x000d6920 (2 instructions, 1 basic blocks)
    pub fn method_169(&mut self) {
    }

    /// Decompiled virtual method #170 @ RVA 0x000d6920 (2 instructions, 1 basic blocks)
    pub fn method_170(&mut self) {
    }

    /// Decompiled virtual method #171 @ RVA 0x000d6920 (2 instructions, 1 basic blocks)
    pub fn method_171(&mut self) {
    }

    /// Decompiled virtual method #172 @ RVA 0x000d6920 (2 instructions, 1 basic blocks)
    pub fn method_172(&mut self) {
    }

    /// Decompiled virtual method #173 @ RVA 0x00103e80 (2 instructions, 1 basic blocks)
    pub fn method_173(&mut self) {
    }

    /// Decompiled virtual method #174 @ RVA 0x000d3090 (1 instructions, 1 basic blocks)
    pub fn method_174(&mut self) {
    }

    /// Decompiled virtual method #175 @ RVA 0x000d3090 (1 instructions, 1 basic blocks)
    pub fn method_175(&mut self) {
    }

    /// Decompiled virtual method #176 @ RVA 0x000d3090 (1 instructions, 1 basic blocks)
    pub fn method_176(&mut self) {
    }

    /// Decompiled virtual method #177 @ RVA 0x000d6920 (2 instructions, 1 basic blocks)
    pub fn method_177(&mut self) {
    }

    /// Decompiled virtual method #178 @ RVA 0x000d6920 (2 instructions, 1 basic blocks)
    pub fn method_178(&mut self) {
    }

    /// Decompiled virtual method #179 @ RVA 0x000d6920 (2 instructions, 1 basic blocks)
    pub fn method_179(&mut self) {
    }

    /// Decompiled virtual method #180 @ RVA 0x000d6920 (2 instructions, 1 basic blocks)
    pub fn method_180(&mut self) {
    }

    /// Decompiled virtual method #181 @ RVA 0x000d6920 (2 instructions, 1 basic blocks)
    pub fn method_181(&mut self) {
    }

    /// Decompiled virtual method #182 @ RVA 0x000d6920 (2 instructions, 1 basic blocks)
    pub fn method_182(&mut self) {
    }

    /// Decompiled virtual method #183 @ RVA 0x000d6920 (2 instructions, 1 basic blocks)
    pub fn method_183(&mut self) {
    }

    /// Decompiled virtual method #184 @ RVA 0x000d6920 (2 instructions, 1 basic blocks)
    pub fn method_184(&mut self) {
    }

    /// Decompiled virtual method #185 @ RVA 0x000d6920 (2 instructions, 1 basic blocks)
    pub fn method_185(&mut self) {
    }

    /// Decompiled virtual method #186 @ RVA 0x000d6920 (2 instructions, 1 basic blocks)
    pub fn method_186(&mut self) {
    }

    /// Decompiled virtual method #187 @ RVA 0x000d6920 (2 instructions, 1 basic blocks)
    pub fn method_187(&mut self) {
    }

    /// Decompiled virtual method #188 @ RVA 0x00103e80 (2 instructions, 1 basic blocks)
    pub fn method_188(&mut self) {
    }

    /// Decompiled virtual method #189 @ RVA 0x00103e80 (2 instructions, 1 basic blocks)
    pub fn method_189(&mut self) {
    }

    /// Decompiled virtual method #190 @ RVA 0x00103e80 (2 instructions, 1 basic blocks)
    pub fn method_190(&mut self) {
    }

    /// Decompiled virtual method #191 @ RVA 0x000d3090 (1 instructions, 1 basic blocks)
    pub fn method_191(&mut self) {
    }

    /// Decompiled virtual method #192 @ RVA 0x000d3090 (1 instructions, 1 basic blocks)
    pub fn method_192(&mut self) {
    }

    /// Decompiled virtual method #193 @ RVA 0x000d3090 (1 instructions, 1 basic blocks)
    pub fn method_193(&mut self) {
    }

    /// Decompiled virtual method #194 @ RVA 0x000d6920 (2 instructions, 1 basic blocks)
    pub fn method_194(&mut self) {
    }

    /// Decompiled virtual method #195 @ RVA 0x000d3090 (1 instructions, 1 basic blocks)
    pub fn method_195(&mut self) {
    }

    /// Decompiled virtual method #196 @ RVA 0x000d6920 (2 instructions, 1 basic blocks)
    pub fn method_196(&mut self) {
    }

    /// Decompiled virtual method #197 @ RVA 0x000d3090 (1 instructions, 1 basic blocks)
    pub fn method_197(&mut self) {
    }

    /// Decompiled virtual method #198 @ RVA 0x00103e80 (2 instructions, 1 basic blocks)
    pub fn method_198(&mut self) {
    }

    /// Decompiled virtual method #199 @ RVA 0x00103e80 (2 instructions, 1 basic blocks)
    pub fn method_199(&mut self) {
    }

    /// Decompiled virtual method #200 @ RVA 0x00103e80 (2 instructions, 1 basic blocks)
    pub fn method_200(&mut self) {
    }

    /// Decompiled virtual method #201 @ RVA 0x00103e80 (2 instructions, 1 basic blocks)
    pub fn method_201(&mut self) {
    }

    /// Decompiled virtual method #202 @ RVA 0x000d6920 (2 instructions, 1 basic blocks)
    pub fn method_202(&mut self) {
    }

    /// Decompiled virtual method #203 @ RVA 0x000d6920 (2 instructions, 1 basic blocks)
    pub fn method_203(&mut self) {
    }

    /// Decompiled virtual method #204 @ RVA 0x000d6920 (2 instructions, 1 basic blocks)
    pub fn method_204(&mut self) {
    }

    /// Decompiled virtual method #205 @ RVA 0x000d6920 (2 instructions, 1 basic blocks)
    pub fn method_205(&mut self) {
    }

    /// Decompiled virtual method #206 @ RVA 0x000d6920 (2 instructions, 1 basic blocks)
    pub fn method_206(&mut self) {
    }

    /// Decompiled virtual method #207 @ RVA 0x000d6920 (2 instructions, 1 basic blocks)
    pub fn method_207(&mut self) {
    }

    /// Decompiled virtual method #208 @ RVA 0x00103e80 (2 instructions, 1 basic blocks)
    pub fn method_208(&mut self) {
    }

    /// Decompiled virtual method #209 @ RVA 0x00103e80 (2 instructions, 1 basic blocks)
    pub fn method_209(&mut self) {
    }

    /// Decompiled virtual method #210 @ RVA 0x000d6920 (2 instructions, 1 basic blocks)
    pub fn method_210(&mut self) {
    }

    /// Decompiled virtual method #211 @ RVA 0x00103e80 (2 instructions, 1 basic blocks)
    pub fn method_211(&mut self) {
    }

    /// Decompiled virtual method #212 @ RVA 0x000d3090 (1 instructions, 1 basic blocks)
    pub fn method_212(&mut self) {
    }

    /// Decompiled virtual method #213 @ RVA 0x000d3090 (1 instructions, 1 basic blocks)
    pub fn method_213(&mut self) {
    }

    /// Decompiled virtual method #214 @ RVA 0x000d3090 (1 instructions, 1 basic blocks)
    pub fn method_214(&mut self) {
    }

    /// Decompiled virtual method #215 @ RVA 0x000d3090 (1 instructions, 1 basic blocks)
    pub fn method_215(&mut self) {
    }

    /// Decompiled virtual method #216 @ RVA 0x000d3090 (1 instructions, 1 basic blocks)
    pub fn method_216(&mut self) {
    }

    /// Decompiled virtual method #217 @ RVA 0x00103e70 (2 instructions, 1 basic blocks)
    pub fn method_217(&mut self) {
    }

    /// Decompiled virtual method #218 @ RVA 0x000d6920 (2 instructions, 1 basic blocks)
    pub fn method_218(&mut self) {
    }

    /// Decompiled virtual method #219 @ RVA 0x000d6920 (2 instructions, 1 basic blocks)
    pub fn method_219(&mut self) {
    }

    /// Decompiled virtual method #220 @ RVA 0x000d3090 (1 instructions, 1 basic blocks)
    pub fn method_220(&mut self) {
    }

    /// Decompiled virtual method #221 @ RVA 0x00103e80 (2 instructions, 1 basic blocks)
    pub fn method_221(&mut self) {
    }

    /// Decompiled virtual method #222 @ RVA 0x00103e70 (2 instructions, 1 basic blocks)
    pub fn method_222(&mut self) {
    }

    /// Decompiled virtual method #223 @ RVA 0x000d3090 (1 instructions, 1 basic blocks)
    pub fn method_223(&mut self) {
    }

    /// Decompiled virtual method #224 @ RVA 0x000d3090 (1 instructions, 1 basic blocks)
    pub fn method_224(&mut self) {
    }

    /// Decompiled virtual method #225 @ RVA 0x000d3090 (1 instructions, 1 basic blocks)
    pub fn method_225(&mut self) {
    }

    /// Decompiled virtual method #226 @ RVA 0x00103e80 (2 instructions, 1 basic blocks)
    pub fn method_226(&mut self) {
    }

    /// Decompiled virtual method #227 @ RVA 0x000d6920 (2 instructions, 1 basic blocks)
    pub fn method_227(&mut self) {
    }

    /// Decompiled virtual method #228 @ RVA 0x000d6920 (2 instructions, 1 basic blocks)
    pub fn method_228(&mut self) {
    }

    /// Decompiled virtual method #229 @ RVA 0x000d6920 (2 instructions, 1 basic blocks)
    pub fn method_229(&mut self) {
    }

    /// Decompiled virtual method #230 @ RVA 0x00103e80 (2 instructions, 1 basic blocks)
    pub fn method_230(&mut self) {
    }

    /// Decompiled virtual method #231 @ RVA 0x00103e80 (2 instructions, 1 basic blocks)
    pub fn method_231(&mut self) {
    }

    /// Decompiled virtual method #232 @ RVA 0x00103e80 (2 instructions, 1 basic blocks)
    pub fn method_232(&mut self) {
    }

    /// Decompiled virtual method #233 @ RVA 0x000d3090 (1 instructions, 1 basic blocks)
    pub fn method_233(&mut self) {
    }

    /// Decompiled virtual method #234 @ RVA 0x000d3090 (1 instructions, 1 basic blocks)
    pub fn method_234(&mut self) {
    }

    /// Decompiled virtual method #235 @ RVA 0x000d3090 (1 instructions, 1 basic blocks)
    pub fn method_235(&mut self) {
    }

    /// Decompiled virtual method #236 @ RVA 0x000d6920 (2 instructions, 1 basic blocks)
    pub fn method_236(&mut self) {
    }

    /// Decompiled virtual method #237 @ RVA 0x000d3090 (1 instructions, 1 basic blocks)
    pub fn method_237(&mut self) {
    }

    /// Decompiled virtual method #238 @ RVA 0x000d3090 (1 instructions, 1 basic blocks)
    pub fn method_238(&mut self) {
    }

    /// Decompiled virtual method #239 @ RVA 0x000d3090 (1 instructions, 1 basic blocks)
    pub fn method_239(&mut self) {
    }

    /// Decompiled virtual method #240 @ RVA 0x000d6920 (2 instructions, 1 basic blocks)
    pub fn method_240(&mut self) {
    }

    /// Decompiled virtual method #241 @ RVA 0x000d3090 (1 instructions, 1 basic blocks)
    pub fn method_241(&mut self) {
    }

    /// Decompiled virtual method #242 @ RVA 0x000d3090 (1 instructions, 1 basic blocks)
    pub fn method_242(&mut self) {
    }

    /// Decompiled virtual method #243 @ RVA 0x00103e80 (2 instructions, 1 basic blocks)
    pub fn method_243(&mut self) {
    }

    /// Decompiled virtual method #244 @ RVA 0x000d3090 (1 instructions, 1 basic blocks)
    pub fn method_244(&mut self) {
    }

    /// Decompiled virtual method #245 @ RVA 0x000d3090 (1 instructions, 1 basic blocks)
    pub fn method_245(&mut self) {
    }

    /// Decompiled virtual method #246 @ RVA 0x000d3090 (1 instructions, 1 basic blocks)
    pub fn method_246(&mut self) {
    }

    /// Decompiled virtual method #247 @ RVA 0x000d3090 (1 instructions, 1 basic blocks)
    pub fn method_247(&mut self) {
    }

    /// Decompiled virtual method #248 @ RVA 0x00103e80 (2 instructions, 1 basic blocks)
    pub fn method_248(&mut self) {
    }

    /// Decompiled virtual method #249 @ RVA 0x000d6920 (2 instructions, 1 basic blocks)
    pub fn method_249(&mut self) {
    }

    /// Decompiled virtual method #250 @ RVA 0x000d3090 (1 instructions, 1 basic blocks)
    pub fn method_250(&mut self) {
    }

    /// Decompiled virtual method #251 @ RVA 0x000d3090 (1 instructions, 1 basic blocks)
    pub fn method_251(&mut self) {
    }

    /// Decompiled virtual method #252 @ RVA 0x000d3090 (1 instructions, 1 basic blocks)
    pub fn method_252(&mut self) {
    }

    /// Decompiled virtual method #253 @ RVA 0x000d3090 (1 instructions, 1 basic blocks)
    pub fn method_253(&mut self) {
    }

    /// Decompiled virtual method #254 @ RVA 0x00103e80 (2 instructions, 1 basic blocks)
    pub fn method_254(&mut self) {
    }

    /// Decompiled virtual method #255 @ RVA 0x000d3090 (1 instructions, 1 basic blocks)
    pub fn method_255(&mut self) {
    }

    /// Decompiled virtual method #256 @ RVA 0x000d3090 (1 instructions, 1 basic blocks)
    pub fn method_256(&mut self) {
    }

    /// Decompiled virtual method #257 @ RVA 0x00103e80 (2 instructions, 1 basic blocks)
    pub fn method_257(&mut self) {
    }

    /// Decompiled virtual method #258 @ RVA 0x000d3090 (1 instructions, 1 basic blocks)
    pub fn method_258(&mut self) {
    }

    /// Decompiled virtual method #259 @ RVA 0x00103e80 (2 instructions, 1 basic blocks)
    pub fn method_259(&mut self) {
    }

    /// Decompiled virtual method #260 @ RVA 0x000d6920 (2 instructions, 1 basic blocks)
    pub fn method_260(&mut self) {
    }

    /// Decompiled virtual method #261 @ RVA 0x000d6920 (2 instructions, 1 basic blocks)
    pub fn method_261(&mut self) {
    }

    /// Decompiled virtual method #262 @ RVA 0x000d6920 (2 instructions, 1 basic blocks)
    pub fn method_262(&mut self) {
    }

    /// Decompiled virtual method #263 @ RVA 0x000d6920 (2 instructions, 1 basic blocks)
    pub fn method_263(&mut self) {
    }

    /// Decompiled virtual method #264 @ RVA 0x000d6920 (2 instructions, 1 basic blocks)
    pub fn method_264(&mut self) {
    }

    /// Decompiled virtual method #265 @ RVA 0x000d3090 (1 instructions, 1 basic blocks)
    pub fn method_265(&mut self) {
    }

    /// Decompiled virtual method #266 @ RVA 0x00103e80 (2 instructions, 1 basic blocks)
    pub fn method_266(&mut self) {
    }

    /// Decompiled virtual method #267 @ RVA 0x000d6920 (2 instructions, 1 basic blocks)
    pub fn method_267(&mut self) {
    }

    /// Decompiled virtual method #268 @ RVA 0x000d6920 (2 instructions, 1 basic blocks)
    pub fn method_268(&mut self) {
    }

    /// Decompiled virtual method #269 @ RVA 0x000d6920 (2 instructions, 1 basic blocks)
    pub fn method_269(&mut self) {
    }

    /// Decompiled virtual method #270 @ RVA 0x000d6920 (2 instructions, 1 basic blocks)
    pub fn method_270(&mut self) {
    }

    /// Decompiled virtual method #271 @ RVA 0x000d6920 (2 instructions, 1 basic blocks)
    pub fn method_271(&mut self) {
    }

    /// Decompiled virtual method #272 @ RVA 0x000d6920 (2 instructions, 1 basic blocks)
    pub fn method_272(&mut self) {
    }

    /// Decompiled virtual method #273 @ RVA 0x000d6920 (2 instructions, 1 basic blocks)
    pub fn method_273(&mut self) {
    }

    /// Decompiled virtual method #274 @ RVA 0x000d3090 (1 instructions, 1 basic blocks)
    pub fn method_274(&mut self) {
    }

    /// Decompiled virtual method #275 @ RVA 0x000d3090 (1 instructions, 1 basic blocks)
    pub fn method_275(&mut self) {
    }

    /// Decompiled virtual method #276 @ RVA 0x000d3090 (1 instructions, 1 basic blocks)
    pub fn method_276(&mut self) {
    }

    /// Decompiled virtual method #277 @ RVA 0x000d3090 (1 instructions, 1 basic blocks)
    pub fn method_277(&mut self) {
    }

    /// Decompiled virtual method #278 @ RVA 0x000d3090 (1 instructions, 1 basic blocks)
    pub fn method_278(&mut self) {
    }

    /// Decompiled virtual method #279 @ RVA 0x000d6920 (2 instructions, 1 basic blocks)
    pub fn method_279(&mut self) {
    }

    /// Decompiled virtual method #280 @ RVA 0x001499d0 (2 instructions, 1 basic blocks)
    pub fn method_280(&mut self) {
    }

    /// Decompiled virtual method #281 @ RVA 0x001499d0 (2 instructions, 1 basic blocks)
    pub fn method_281(&mut self) {
    }

    /// Decompiled virtual method #282 @ RVA 0x001499d0 (2 instructions, 1 basic blocks)
    pub fn method_282(&mut self) {
    }

    /// Decompiled virtual method #283 @ RVA 0x001499d0 (2 instructions, 1 basic blocks)
    pub fn method_283(&mut self) {
    }

    /// Decompiled virtual method #284 @ RVA 0x0073e2c0 (337 instructions, 1 basic blocks)
    pub fn method_284(&mut self) {
    }

    /// Decompiled virtual method #285 @ RVA 0x0073e2c0 (337 instructions, 1 basic blocks)
    pub fn method_285(&mut self) {
    }

    /// Decompiled virtual method #286 @ RVA 0x0073e2c0 (337 instructions, 1 basic blocks)
    pub fn method_286(&mut self) {
    }

    /// Decompiled virtual method #287 @ RVA 0x0073e2c0 (337 instructions, 1 basic blocks)
    pub fn method_287(&mut self) {
    }

    /// Decompiled virtual method #288 @ RVA 0x0073e2c0 (337 instructions, 1 basic blocks)
    pub fn method_288(&mut self) {
    }

    /// Decompiled virtual method #289 @ RVA 0x0073e2c0 (337 instructions, 1 basic blocks)
    pub fn method_289(&mut self) {
    }

    /// Decompiled virtual method #290 @ RVA 0x000d3090 (1 instructions, 1 basic blocks)
    pub fn method_290(&mut self) {
    }

    /// Decompiled virtual method #291 @ RVA 0x00103e80 (2 instructions, 1 basic blocks)
    pub fn method_291(&mut self) {
    }

    /// Decompiled virtual method #292 @ RVA 0x000d6920 (2 instructions, 1 basic blocks)
    pub fn method_292(&mut self) {
    }

    /// Decompiled virtual method #293 @ RVA 0x000d6920 (2 instructions, 1 basic blocks)
    pub fn method_293(&mut self) {
    }

    /// Decompiled virtual method #294 @ RVA 0x000d6920 (2 instructions, 1 basic blocks)
    pub fn method_294(&mut self) {
    }

    /// Decompiled virtual method #295 @ RVA 0x000d6920 (2 instructions, 1 basic blocks)
    pub fn method_295(&mut self) {
    }

    /// Decompiled virtual method #296 @ RVA 0x000d6920 (2 instructions, 1 basic blocks)
    pub fn method_296(&mut self) {
    }

    /// Decompiled virtual method #297 @ RVA 0x000d6920 (2 instructions, 1 basic blocks)
    pub fn method_297(&mut self) {
    }

    /// Decompiled virtual method #298 @ RVA 0x000d6920 (2 instructions, 1 basic blocks)
    pub fn method_298(&mut self) {
    }

    /// Decompiled virtual method #299 @ RVA 0x000d6920 (2 instructions, 1 basic blocks)
    pub fn method_299(&mut self) {
    }

    /// Decompiled virtual method #300 @ RVA 0x00103e80 (2 instructions, 1 basic blocks)
    pub fn method_300(&mut self) {
    }

    /// Decompiled virtual method #301 @ RVA 0x00103e80 (2 instructions, 1 basic blocks)
    pub fn method_301(&mut self) {
    }

    /// Decompiled virtual method #302 @ RVA 0x00103e80 (2 instructions, 1 basic blocks)
    pub fn method_302(&mut self) {
    }

    /// Decompiled virtual method #303 @ RVA 0x000d6920 (2 instructions, 1 basic blocks)
    pub fn method_303(&mut self) {
    }

    /// Decompiled virtual method #304 @ RVA 0x000d6920 (2 instructions, 1 basic blocks)
    pub fn method_304(&mut self) {
    }

    /// Decompiled virtual method #305 @ RVA 0x000d6920 (2 instructions, 1 basic blocks)
    pub fn method_305(&mut self) {
    }

    /// Decompiled virtual method #306 @ RVA 0x000d3090 (1 instructions, 1 basic blocks)
    pub fn method_306(&mut self) {
    }

    /// Decompiled virtual method #307 @ RVA 0x000d3090 (1 instructions, 1 basic blocks)
    pub fn method_307(&mut self) {
    }

    /// Decompiled virtual method #308 @ RVA 0x000d3090 (1 instructions, 1 basic blocks)
    pub fn method_308(&mut self) {
    }

    /// Decompiled virtual method #309 @ RVA 0x000d3090 (1 instructions, 1 basic blocks)
    pub fn method_309(&mut self) {
    }

    /// Decompiled virtual method #310 @ RVA 0x00584500 (2 instructions, 1 basic blocks)
    pub fn method_310(&mut self) {
    }

    /// Decompiled virtual method #311 @ RVA 0x000d3090 (1 instructions, 1 basic blocks)
    pub fn method_311(&mut self) {
    }

    /// Decompiled virtual method #312 @ RVA 0x00103e80 (2 instructions, 1 basic blocks)
    pub fn method_312(&mut self) {
    }

    /// Decompiled virtual method #313 @ RVA 0x000d3090 (1 instructions, 1 basic blocks)
    pub fn method_313(&mut self) {
    }

    /// Decompiled virtual method #314 @ RVA 0x00584500 (2 instructions, 1 basic blocks)
    pub fn method_314(&mut self) {
    }

    /// Decompiled virtual method #315 @ RVA 0x000d3090 (1 instructions, 1 basic blocks)
    pub fn method_315(&mut self) {
    }

    /// Decompiled virtual method #316 @ RVA 0x000d3090 (1 instructions, 1 basic blocks)
    pub fn method_316(&mut self) {
    }

    /// Decompiled virtual method #317 @ RVA 0x000d6920 (2 instructions, 1 basic blocks)
    pub fn method_317(&mut self) {
    }

    /// Decompiled virtual method #318 @ RVA 0x000d3090 (1 instructions, 1 basic blocks)
    pub fn method_318(&mut self) {
    }

    /// Decompiled virtual method #319 @ RVA 0x000d3090 (1 instructions, 1 basic blocks)
    pub fn method_319(&mut self) {
    }

    /// Decompiled virtual method #320 @ RVA 0x000d6920 (2 instructions, 1 basic blocks)
    pub fn method_320(&mut self) {
    }

    /// Decompiled virtual method #321 @ RVA 0x00103e80 (2 instructions, 1 basic blocks)
    pub fn method_321(&mut self) {
    }

    /// Decompiled virtual method #322 @ RVA 0x000d3090 (1 instructions, 1 basic blocks)
    pub fn method_322(&mut self) {
    }

    /// Decompiled virtual method #323 @ RVA 0x000d3090 (1 instructions, 1 basic blocks)
    pub fn method_323(&mut self) {
    }

    /// Decompiled virtual method #324 @ RVA 0x000d3090 (1 instructions, 1 basic blocks)
    pub fn method_324(&mut self) {
    }

    /// Decompiled virtual method #325 @ RVA 0x000d3090 (1 instructions, 1 basic blocks)
    pub fn method_325(&mut self) {
    }

    /// Decompiled virtual method #326 @ RVA 0x00103e80 (2 instructions, 1 basic blocks)
    pub fn method_326(&mut self) {
    }

    /// Decompiled virtual method #327 @ RVA 0x000d3090 (1 instructions, 1 basic blocks)
    pub fn method_327(&mut self) {
    }

    /// Decompiled virtual method #328 @ RVA 0x00103e80 (2 instructions, 1 basic blocks)
    pub fn method_328(&mut self) {
    }

    /// Decompiled virtual method #329 @ RVA 0x000d3090 (1 instructions, 1 basic blocks)
    pub fn method_329(&mut self) {
    }

    /// Decompiled virtual method #330 @ RVA 0x00103e80 (2 instructions, 1 basic blocks)
    pub fn method_330(&mut self) {
    }

    /// Decompiled virtual method #331 @ RVA 0x000d3090 (1 instructions, 1 basic blocks)
    pub fn method_331(&mut self) {
    }

    /// Decompiled virtual method #332 @ RVA 0x00103e80 (2 instructions, 1 basic blocks)
    pub fn method_332(&mut self) {
    }

    /// Decompiled virtual method #333 @ RVA 0x000d3090 (1 instructions, 1 basic blocks)
    pub fn method_333(&mut self) {
    }

    /// Decompiled virtual method #334 @ RVA 0x000d3090 (1 instructions, 1 basic blocks)
    pub fn method_334(&mut self) {
    }

    /// Decompiled virtual method #335 @ RVA 0x000d6920 (2 instructions, 1 basic blocks)
    pub fn method_335(&mut self) {
    }

    /// Decompiled virtual method #336 @ RVA 0x000d3090 (1 instructions, 1 basic blocks)
    pub fn method_336(&mut self) {
    }

    /// Decompiled virtual method #337 @ RVA 0x000d3090 (1 instructions, 1 basic blocks)
    pub fn method_337(&mut self) {
    }

    /// Decompiled virtual method #338 @ RVA 0x000d3090 (1 instructions, 1 basic blocks)
    pub fn method_338(&mut self) {
    }

    /// Decompiled virtual method #339 @ RVA 0x000d3090 (1 instructions, 1 basic blocks)
    pub fn method_339(&mut self) {
    }

    /// Decompiled virtual method #340 @ RVA 0x00103e80 (2 instructions, 1 basic blocks)
    pub fn method_340(&mut self) {
    }

    /// Decompiled virtual method #341 @ RVA 0x00103e80 (2 instructions, 1 basic blocks)
    pub fn method_341(&mut self) {
    }

    /// Decompiled virtual method #342 @ RVA 0x000d6920 (2 instructions, 1 basic blocks)
    pub fn method_342(&mut self) {
    }

    /// Decompiled virtual method #343 @ RVA 0x000d6920 (2 instructions, 1 basic blocks)
    pub fn method_343(&mut self) {
    }

    /// Decompiled virtual method #344 @ RVA 0x000d6920 (2 instructions, 1 basic blocks)
    pub fn method_344(&mut self) {
    }

    /// Decompiled virtual method #345 @ RVA 0x000d3090 (1 instructions, 1 basic blocks)
    pub fn method_345(&mut self) {
    }

    /// Decompiled virtual method #346 @ RVA 0x00103e80 (2 instructions, 1 basic blocks)
    pub fn method_346(&mut self) {
    }

    /// Decompiled virtual method #347 @ RVA 0x000d3090 (1 instructions, 1 basic blocks)
    pub fn method_347(&mut self) {
    }

    /// Decompiled virtual method #348 @ RVA 0x000d3090 (1 instructions, 1 basic blocks)
    pub fn method_348(&mut self) {
    }

    /// Decompiled virtual method #349 @ RVA 0x000d6920 (2 instructions, 1 basic blocks)
    pub fn method_349(&mut self) {
    }

    /// Decompiled virtual method #350 @ RVA 0x000d3090 (1 instructions, 1 basic blocks)
    pub fn method_350(&mut self) {
    }

    /// Decompiled virtual method #351 @ RVA 0x000d3090 (1 instructions, 1 basic blocks)
    pub fn method_351(&mut self) {
    }

    /// Decompiled virtual method #352 @ RVA 0x000d6920 (2 instructions, 1 basic blocks)
    pub fn method_352(&mut self) {
    }

    /// Decompiled virtual method #353 @ RVA 0x000d3090 (1 instructions, 1 basic blocks)
    pub fn method_353(&mut self) {
    }

    /// Decompiled virtual method #354 @ RVA 0x000d3090 (1 instructions, 1 basic blocks)
    pub fn method_354(&mut self) {
    }

    /// Decompiled virtual method #355 @ RVA 0x000d6920 (2 instructions, 1 basic blocks)
    pub fn method_355(&mut self) {
    }

    /// Decompiled virtual method #356 @ RVA 0x000d3090 (1 instructions, 1 basic blocks)
    pub fn method_356(&mut self) {
    }

    /// Decompiled virtual method #357 @ RVA 0x000d3090 (1 instructions, 1 basic blocks)
    pub fn method_357(&mut self) {
    }

    /// Decompiled virtual method #358 @ RVA 0x000d6920 (2 instructions, 1 basic blocks)
    pub fn method_358(&mut self) {
    }

    /// Decompiled virtual method #359 @ RVA 0x000d3090 (1 instructions, 1 basic blocks)
    pub fn method_359(&mut self) {
    }

    /// Decompiled virtual method #360 @ RVA 0x000d3090 (1 instructions, 1 basic blocks)
    pub fn method_360(&mut self) {
    }

    /// Decompiled virtual method #361 @ RVA 0x000d6920 (2 instructions, 1 basic blocks)
    pub fn method_361(&mut self) {
    }

    /// Decompiled virtual method #362 @ RVA 0x000d3090 (1 instructions, 1 basic blocks)
    pub fn method_362(&mut self) {
    }

    /// Decompiled virtual method #363 @ RVA 0x000d3090 (1 instructions, 1 basic blocks)
    pub fn method_363(&mut self) {
    }

    /// Decompiled virtual method #364 @ RVA 0x000d6920 (2 instructions, 1 basic blocks)
    pub fn method_364(&mut self) {
    }

    /// Decompiled virtual method #365 @ RVA 0x00103e80 (2 instructions, 1 basic blocks)
    pub fn method_365(&mut self) {
    }

    /// Decompiled virtual method #366 @ RVA 0x000d3090 (1 instructions, 1 basic blocks)
    pub fn method_366(&mut self) {
    }

    /// Decompiled virtual method #367 @ RVA 0x000d3090 (1 instructions, 1 basic blocks)
    pub fn method_367(&mut self) {
    }

    /// Decompiled virtual method #368 @ RVA 0x00103e80 (2 instructions, 1 basic blocks)
    pub fn method_368(&mut self) {
    }

    /// Decompiled virtual method #369 @ RVA 0x000d3090 (1 instructions, 1 basic blocks)
    pub fn method_369(&mut self) {
    }

    /// Decompiled virtual method #370 @ RVA 0x000d3090 (1 instructions, 1 basic blocks)
    pub fn method_370(&mut self) {
    }

    /// Decompiled virtual method #371 @ RVA 0x000d6920 (2 instructions, 1 basic blocks)
    pub fn method_371(&mut self) {
    }

    /// Decompiled virtual method #372 @ RVA 0x000d6920 (2 instructions, 1 basic blocks)
    pub fn method_372(&mut self) {
    }

    /// Decompiled virtual method #373 @ RVA 0x000d3090 (1 instructions, 1 basic blocks)
    pub fn method_373(&mut self) {
    }

    /// Decompiled virtual method #374 @ RVA 0x000d3090 (1 instructions, 1 basic blocks)
    pub fn method_374(&mut self) {
    }

    /// Decompiled virtual method #375 @ RVA 0x000d3090 (1 instructions, 1 basic blocks)
    pub fn method_375(&mut self) {
    }

    /// Decompiled virtual method #376 @ RVA 0x000d3090 (1 instructions, 1 basic blocks)
    pub fn method_376(&mut self) {
    }

    /// Decompiled virtual method #377 @ RVA 0x000d3090 (1 instructions, 1 basic blocks)
    pub fn method_377(&mut self) {
    }

    /// Decompiled virtual method #378 @ RVA 0x000d3090 (1 instructions, 1 basic blocks)
    pub fn method_378(&mut self) {
    }

    /// Decompiled virtual method #379 @ RVA 0x000d3090 (1 instructions, 1 basic blocks)
    pub fn method_379(&mut self) {
    }

    /// Decompiled virtual method #380 @ RVA 0x000d3090 (1 instructions, 1 basic blocks)
    pub fn method_380(&mut self) {
    }

    /// Decompiled virtual method #381 @ RVA 0x00103e80 (2 instructions, 1 basic blocks)
    pub fn method_381(&mut self) {
    }

    /// Decompiled virtual method #382 @ RVA 0x00103e80 (2 instructions, 1 basic blocks)
    pub fn method_382(&mut self) {
    }

    /// Decompiled virtual method #383 @ RVA 0x00103e80 (2 instructions, 1 basic blocks)
    pub fn method_383(&mut self) {
    }

    /// Decompiled virtual method #384 @ RVA 0x00103e70 (2 instructions, 1 basic blocks)
    pub fn method_384(&mut self) {
    }

    /// Decompiled virtual method #385 @ RVA 0x00103e70 (2 instructions, 1 basic blocks)
    pub fn method_385(&mut self) {
    }

    /// Decompiled virtual method #386 @ RVA 0x00103e70 (2 instructions, 1 basic blocks)
    pub fn method_386(&mut self) {
    }

    /// Decompiled virtual method #387 @ RVA 0x00103e80 (2 instructions, 1 basic blocks)
    pub fn method_387(&mut self) {
    }

    /// Decompiled virtual method #388 @ RVA 0x000d3090 (1 instructions, 1 basic blocks)
    pub fn method_388(&mut self) {
    }

    /// Decompiled virtual method #389 @ RVA 0x00103e70 (2 instructions, 1 basic blocks)
    pub fn method_389(&mut self) {
    }

    /// Decompiled virtual method #390 @ RVA 0x00103e80 (2 instructions, 1 basic blocks)
    pub fn method_390(&mut self) {
    }

    /// Decompiled virtual method #391 @ RVA 0x000d3090 (1 instructions, 1 basic blocks)
    pub fn method_391(&mut self) {
    }

    /// Decompiled virtual method #392 @ RVA 0x00103e70 (2 instructions, 1 basic blocks)
    pub fn method_392(&mut self) {
    }

    /// Decompiled virtual method #393 @ RVA 0x00103e70 (2 instructions, 1 basic blocks)
    pub fn method_393(&mut self) {
    }

    /// Decompiled virtual method #394 @ RVA 0x000d6920 (2 instructions, 1 basic blocks)
    pub fn method_394(&mut self) {
    }

    /// Decompiled virtual method #395 @ RVA 0x000d3090 (1 instructions, 1 basic blocks)
    pub fn method_395(&mut self) {
    }

    /// Decompiled virtual method #396 @ RVA 0x000d3090 (1 instructions, 1 basic blocks)
    pub fn method_396(&mut self) {
    }

    /// Decompiled virtual method #397 @ RVA 0x000d3090 (1 instructions, 1 basic blocks)
    pub fn method_397(&mut self) {
    }

    /// Decompiled virtual method #398 @ RVA 0x000d3090 (1 instructions, 1 basic blocks)
    pub fn method_398(&mut self) {
    }

    /// Decompiled virtual method #399 @ RVA 0x000d3090 (1 instructions, 1 basic blocks)
    pub fn method_399(&mut self) {
    }

    /// Decompiled virtual method #400 @ RVA 0x00103e80 (2 instructions, 1 basic blocks)
    pub fn method_400(&mut self) {
    }

    /// Decompiled virtual method #401 @ RVA 0x000d3090 (1 instructions, 1 basic blocks)
    pub fn method_401(&mut self) {
    }

    /// Decompiled virtual method #402 @ RVA 0x00103e80 (2 instructions, 1 basic blocks)
    pub fn method_402(&mut self) {
    }

    /// Decompiled virtual method #403 @ RVA 0x00103e70 (2 instructions, 1 basic blocks)
    pub fn method_403(&mut self) {
    }

    /// Decompiled virtual method #404 @ RVA 0x000d3090 (1 instructions, 1 basic blocks)
    pub fn method_404(&mut self) {
    }

    /// Decompiled virtual method #405 @ RVA 0x000d3090 (1 instructions, 1 basic blocks)
    pub fn method_405(&mut self) {
    }

    /// Decompiled virtual method #406 @ RVA 0x000d6920 (2 instructions, 1 basic blocks)
    pub fn method_406(&mut self) {
    }

    /// Decompiled virtual method #407 @ RVA 0x00103e70 (2 instructions, 1 basic blocks)
    pub fn method_407(&mut self) {
    }

    /// Decompiled virtual method #408 @ RVA 0x000d3090 (1 instructions, 1 basic blocks)
    pub fn method_408(&mut self) {
    }

    /// Decompiled virtual method #409 @ RVA 0x00103e70 (2 instructions, 1 basic blocks)
    pub fn method_409(&mut self) {
    }

    /// Decompiled virtual method #410 @ RVA 0x00103e80 (2 instructions, 1 basic blocks)
    pub fn method_410(&mut self) {
    }

    /// Decompiled virtual method #411 @ RVA 0x000d3090 (1 instructions, 1 basic blocks)
    pub fn method_411(&mut self) {
    }

    /// Decompiled virtual method #412 @ RVA 0x00103e80 (2 instructions, 1 basic blocks)
    pub fn method_412(&mut self) {
    }

    /// Decompiled virtual method #413 @ RVA 0x000d3090 (1 instructions, 1 basic blocks)
    pub fn method_413(&mut self) {
    }

    /// Decompiled virtual method #414 @ RVA 0x00103e80 (2 instructions, 1 basic blocks)
    pub fn method_414(&mut self) {
    }

    /// Decompiled virtual method #415 @ RVA 0x000d3090 (1 instructions, 1 basic blocks)
    pub fn method_415(&mut self) {
    }

    /// Decompiled virtual method #416 @ RVA 0x000d3090 (1 instructions, 1 basic blocks)
    pub fn method_416(&mut self) {
    }

    /// Decompiled virtual method #417 @ RVA 0x000d3090 (1 instructions, 1 basic blocks)
    pub fn method_417(&mut self) {
    }

    /// Decompiled virtual method #418 @ RVA 0x000d3090 (1 instructions, 1 basic blocks)
    pub fn method_418(&mut self) {
    }

    /// Decompiled virtual method #419 @ RVA 0x000d3090 (1 instructions, 1 basic blocks)
    pub fn method_419(&mut self) {
    }

    /// Decompiled virtual method #420 @ RVA 0x000d6920 (2 instructions, 1 basic blocks)
    pub fn method_420(&mut self) {
    }

    /// Decompiled virtual method #421 @ RVA 0x000d6920 (2 instructions, 1 basic blocks)
    pub fn method_421(&mut self) {
    }

    /// Decompiled virtual method #422 @ RVA 0x000d3090 (1 instructions, 1 basic blocks)
    pub fn method_422(&mut self) {
    }

    /// Decompiled virtual method #423 @ RVA 0x000d3090 (1 instructions, 1 basic blocks)
    pub fn method_423(&mut self) {
    }

    /// Decompiled virtual method #424 @ RVA 0x000d3090 (1 instructions, 1 basic blocks)
    pub fn method_424(&mut self) {
    }

    /// Decompiled virtual method #425 @ RVA 0x000d6920 (2 instructions, 1 basic blocks)
    pub fn method_425(&mut self) {
    }

    /// Decompiled virtual method #426 @ RVA 0x000d6920 (2 instructions, 1 basic blocks)
    pub fn method_426(&mut self) {
    }

    /// Decompiled virtual method #427 @ RVA 0x000d3090 (1 instructions, 1 basic blocks)
    pub fn method_427(&mut self) {
    }

    /// Decompiled virtual method #428 @ RVA 0x000d3090 (1 instructions, 1 basic blocks)
    pub fn method_428(&mut self) {
    }

    /// Decompiled virtual method #429 @ RVA 0x000d3090 (1 instructions, 1 basic blocks)
    pub fn method_429(&mut self) {
    }

    /// Decompiled virtual method #430 @ RVA 0x000d6920 (2 instructions, 1 basic blocks)
    pub fn method_430(&mut self) {
    }

    /// Decompiled virtual method #431 @ RVA 0x000d6920 (2 instructions, 1 basic blocks)
    pub fn method_431(&mut self) {
    }

    /// Decompiled virtual method #432 @ RVA 0x000d3090 (1 instructions, 1 basic blocks)
    pub fn method_432(&mut self) {
    }

    /// Decompiled virtual method #433 @ RVA 0x000d3090 (1 instructions, 1 basic blocks)
    pub fn method_433(&mut self) {
    }

    /// Decompiled virtual method #434 @ RVA 0x000d3090 (1 instructions, 1 basic blocks)
    pub fn method_434(&mut self) {
    }

    /// Decompiled virtual method #435 @ RVA 0x000d6920 (2 instructions, 1 basic blocks)
    pub fn method_435(&mut self) {
    }

    /// Decompiled virtual method #436 @ RVA 0x000d6920 (2 instructions, 1 basic blocks)
    pub fn method_436(&mut self) {
    }

    /// Decompiled virtual method #437 @ RVA 0x000d3090 (1 instructions, 1 basic blocks)
    pub fn method_437(&mut self) {
    }

    /// Decompiled virtual method #438 @ RVA 0x000d3090 (1 instructions, 1 basic blocks)
    pub fn method_438(&mut self) {
    }

    /// Decompiled virtual method #439 @ RVA 0x000d6920 (2 instructions, 1 basic blocks)
    pub fn method_439(&mut self) {
    }

    /// Decompiled virtual method #440 @ RVA 0x000d3090 (1 instructions, 1 basic blocks)
    pub fn method_440(&mut self) {
    }

    /// Decompiled virtual method #441 @ RVA 0x000d3090 (1 instructions, 1 basic blocks)
    pub fn method_441(&mut self) {
    }

    /// Decompiled virtual method #442 @ RVA 0x000d6920 (2 instructions, 1 basic blocks)
    pub fn method_442(&mut self) {
    }

    /// Decompiled virtual method #443 @ RVA 0x000d3090 (1 instructions, 1 basic blocks)
    pub fn method_443(&mut self) {
    }

    /// Decompiled virtual method #444 @ RVA 0x000d3090 (1 instructions, 1 basic blocks)
    pub fn method_444(&mut self) {
    }

    /// Decompiled virtual method #445 @ RVA 0x000d6920 (2 instructions, 1 basic blocks)
    pub fn method_445(&mut self) {
    }

    /// Decompiled virtual method #446 @ RVA 0x000d3090 (1 instructions, 1 basic blocks)
    pub fn method_446(&mut self) {
    }

    /// Decompiled virtual method #447 @ RVA 0x000d3090 (1 instructions, 1 basic blocks)
    pub fn method_447(&mut self) {
    }

    /// Decompiled virtual method #448 @ RVA 0x000d3090 (1 instructions, 1 basic blocks)
    pub fn method_448(&mut self) {
    }

    /// Decompiled virtual method #449 @ RVA 0x000d6920 (2 instructions, 1 basic blocks)
    pub fn method_449(&mut self) {
    }

    /// Decompiled virtual method #450 @ RVA 0x000d3090 (1 instructions, 1 basic blocks)
    pub fn method_450(&mut self) {
    }

    /// Decompiled virtual method #451 @ RVA 0x000d3090 (1 instructions, 1 basic blocks)
    pub fn method_451(&mut self) {
    }

    /// Decompiled virtual method #452 @ RVA 0x000d6920 (2 instructions, 1 basic blocks)
    pub fn method_452(&mut self) {
    }

    /// Decompiled virtual method #453 @ RVA 0x000d3090 (1 instructions, 1 basic blocks)
    pub fn method_453(&mut self) {
    }

    /// Decompiled virtual method #454 @ RVA 0x000d3090 (1 instructions, 1 basic blocks)
    pub fn method_454(&mut self) {
    }

    /// Decompiled virtual method #455 @ RVA 0x000d6920 (2 instructions, 1 basic blocks)
    pub fn method_455(&mut self) {
    }

    /// Decompiled virtual method #456 @ RVA 0x000d6920 (2 instructions, 1 basic blocks)
    pub fn method_456(&mut self) {
    }

    /// Decompiled virtual method #457 @ RVA 0x000d3090 (1 instructions, 1 basic blocks)
    pub fn method_457(&mut self) {
    }

    /// Decompiled virtual method #458 @ RVA 0x000d3090 (1 instructions, 1 basic blocks)
    pub fn method_458(&mut self) {
    }

    /// Decompiled virtual method #459 @ RVA 0x000d6920 (2 instructions, 1 basic blocks)
    pub fn method_459(&mut self) {
    }

    /// Decompiled virtual method #460 @ RVA 0x000d3090 (1 instructions, 1 basic blocks)
    pub fn method_460(&mut self) {
    }

    /// Decompiled virtual method #461 @ RVA 0x000d6920 (2 instructions, 1 basic blocks)
    pub fn method_461(&mut self) {
    }

    /// Decompiled virtual method #462 @ RVA 0x00103e70 (2 instructions, 1 basic blocks)
    pub fn method_462(&mut self) {
    }

    /// Decompiled virtual method #463 @ RVA 0x000d3090 (1 instructions, 1 basic blocks)
    pub fn method_463(&mut self) {
    }

    /// Decompiled virtual method #464 @ RVA 0x00103e80 (2 instructions, 1 basic blocks)
    pub fn method_464(&mut self) {
    }

    /// Decompiled virtual method #465 @ RVA 0x000d3090 (1 instructions, 1 basic blocks)
    pub fn method_465(&mut self) {
    }

    /// Decompiled virtual method #466 @ RVA 0x00103e70 (2 instructions, 1 basic blocks)
    pub fn method_466(&mut self) {
    }

    /// Decompiled virtual method #467 @ RVA 0x000d3090 (1 instructions, 1 basic blocks)
    pub fn method_467(&mut self) {
    }

    /// Decompiled virtual method #468 @ RVA 0x000d3090 (1 instructions, 1 basic blocks)
    pub fn method_468(&mut self) {
    }

    /// Decompiled virtual method #469 @ RVA 0x000d3090 (1 instructions, 1 basic blocks)
    pub fn method_469(&mut self) {
    }

    /// Decompiled virtual method #470 @ RVA 0x000d3090 (1 instructions, 1 basic blocks)
    pub fn method_470(&mut self) {
    }

    /// Decompiled virtual method #471 @ RVA 0x000d3090 (1 instructions, 1 basic blocks)
    pub fn method_471(&mut self) {
    }

    /// Decompiled virtual method #472 @ RVA 0x00103e80 (2 instructions, 1 basic blocks)
    pub fn method_472(&mut self) {
    }

    /// Decompiled virtual method #473 @ RVA 0x000d6920 (2 instructions, 1 basic blocks)
    pub fn method_473(&mut self) {
    }

    /// Decompiled virtual method #474 @ RVA 0x000d3090 (1 instructions, 1 basic blocks)
    pub fn method_474(&mut self) {
    }

    /// Decompiled virtual method #475 @ RVA 0x00584510 (2 instructions, 1 basic blocks)
    pub fn method_475(&mut self) {
    }

    /// Decompiled virtual method #476 @ RVA 0x000d3090 (1 instructions, 1 basic blocks)
    pub fn method_476(&mut self) {
    }

    /// Decompiled virtual method #477 @ RVA 0x000d3090 (1 instructions, 1 basic blocks)
    pub fn method_477(&mut self) {
    }

    /// Decompiled virtual method #478 @ RVA 0x000d6920 (2 instructions, 1 basic blocks)
    pub fn method_478(&mut self) {
    }

    /// Decompiled virtual method #479 @ RVA 0x000d3090 (1 instructions, 1 basic blocks)
    pub fn method_479(&mut self) {
    }

    /// Decompiled virtual method #480 @ RVA 0x00103e70 (2 instructions, 1 basic blocks)
    pub fn method_480(&mut self) {
    }

    /// Decompiled virtual method #481 @ RVA 0x000d3090 (1 instructions, 1 basic blocks)
    pub fn method_481(&mut self) {
    }

    /// Decompiled virtual method #482 @ RVA 0x00103e70 (2 instructions, 1 basic blocks)
    pub fn method_482(&mut self) {
    }

    /// Decompiled virtual method #483 @ RVA 0x00584520 (2 instructions, 1 basic blocks)
    pub fn method_483(&mut self) {
    }

    /// Decompiled virtual method #484 @ RVA 0x000d3090 (1 instructions, 1 basic blocks)
    pub fn method_484(&mut self) {
    }

    /// Decompiled virtual method #485 @ RVA 0x00103e80 (2 instructions, 1 basic blocks)
    pub fn method_485(&mut self) {
    }

    /// Decompiled virtual method #486 @ RVA 0x00584530 (2 instructions, 1 basic blocks)
    pub fn method_486(&mut self) {
    }

    /// Decompiled virtual method #487 @ RVA 0x000d3090 (1 instructions, 1 basic blocks)
    pub fn method_487(&mut self) {
    }

    /// Decompiled virtual method #488 @ RVA 0x000d6920 (2 instructions, 1 basic blocks)
    pub fn method_488(&mut self) {
    }

    /// Decompiled virtual method #489 @ RVA 0x000d3090 (1 instructions, 1 basic blocks)
    pub fn method_489(&mut self) {
    }

    /// Decompiled virtual method #490 @ RVA 0x000d6920 (2 instructions, 1 basic blocks)
    pub fn method_490(&mut self) {
    }

    /// Decompiled virtual method #491 @ RVA 0x000d3090 (1 instructions, 1 basic blocks)
    pub fn method_491(&mut self) {
    }

    /// Decompiled virtual method #492 @ RVA 0x000d3090 (1 instructions, 1 basic blocks)
    pub fn method_492(&mut self) {
    }

    /// Decompiled virtual method #493 @ RVA 0x00103e80 (2 instructions, 1 basic blocks)
    pub fn method_493(&mut self) {
    }

    /// Decompiled virtual method #494 @ RVA 0x000d3090 (1 instructions, 1 basic blocks)
    pub fn method_494(&mut self) {
    }

    /// Decompiled virtual method #495 @ RVA 0x000d3090 (1 instructions, 1 basic blocks)
    pub fn method_495(&mut self) {
    }

    /// Decompiled virtual method #496 @ RVA 0x00103e80 (2 instructions, 1 basic blocks)
    pub fn method_496(&mut self) {
    }

    /// Decompiled virtual method #497 @ RVA 0x000d3090 (1 instructions, 1 basic blocks)
    pub fn method_497(&mut self) {
    }

    /// Decompiled virtual method #498 @ RVA 0x000d3090 (1 instructions, 1 basic blocks)
    pub fn method_498(&mut self) {
    }

    /// Decompiled virtual method #499 @ RVA 0x00103e80 (2 instructions, 1 basic blocks)
    pub fn method_499(&mut self) {
    }

    /// Decompiled virtual method #500 @ RVA 0x00103e80 (2 instructions, 1 basic blocks)
    pub fn method_500(&mut self) {
    }

    /// Decompiled virtual method #501 @ RVA 0x000d3090 (1 instructions, 1 basic blocks)
    pub fn method_501(&mut self) {
    }

    /// Decompiled virtual method #502 @ RVA 0x000d3090 (1 instructions, 1 basic blocks)
    pub fn method_502(&mut self) {
    }

    /// Decompiled virtual method #503 @ RVA 0x000d3090 (1 instructions, 1 basic blocks)
    pub fn method_503(&mut self) {
    }

    /// Decompiled virtual method #504 @ RVA 0x000d3090 (1 instructions, 1 basic blocks)
    pub fn method_504(&mut self) {
    }

    /// Decompiled virtual method #505 @ RVA 0x000d3090 (1 instructions, 1 basic blocks)
    pub fn method_505(&mut self) {
    }

    /// Decompiled virtual method #506 @ RVA 0x000d3090 (1 instructions, 1 basic blocks)
    pub fn method_506(&mut self) {
    }

    /// Decompiled virtual method #507 @ RVA 0x00103e80 (2 instructions, 1 basic blocks)
    pub fn method_507(&mut self) {
    }

    /// Decompiled virtual method #508 @ RVA 0x000d3090 (1 instructions, 1 basic blocks)
    pub fn method_508(&mut self) {
    }

    /// Decompiled virtual method #509 @ RVA 0x000d3090 (1 instructions, 1 basic blocks)
    pub fn method_509(&mut self) {
    }

    /// Decompiled virtual method #510 @ RVA 0x000d3090 (1 instructions, 1 basic blocks)
    pub fn method_510(&mut self) {
    }

    /// Decompiled virtual method #511 @ RVA 0x00103e80 (2 instructions, 1 basic blocks)
    pub fn method_511(&mut self) {
    }

    /// Decompiled virtual method #512 @ RVA 0x00103e80 (2 instructions, 1 basic blocks)
    pub fn method_512(&mut self) {
    }

    /// Decompiled virtual method #513 @ RVA 0x000d3090 (1 instructions, 1 basic blocks)
    pub fn method_513(&mut self) {
    }

    /// Decompiled virtual method #514 @ RVA 0x000d3090 (1 instructions, 1 basic blocks)
    pub fn method_514(&mut self) {
    }

    /// Decompiled virtual method #515 @ RVA 0x000d3090 (1 instructions, 1 basic blocks)
    pub fn method_515(&mut self) {
    }

    /// Decompiled virtual method #516 @ RVA 0x000d3090 (1 instructions, 1 basic blocks)
    pub fn method_516(&mut self) {
    }

    /// Decompiled virtual method #517 @ RVA 0x000d3090 (1 instructions, 1 basic blocks)
    pub fn method_517(&mut self) {
    }

    /// Decompiled virtual method #518 @ RVA 0x000d3090 (1 instructions, 1 basic blocks)
    pub fn method_518(&mut self) {
    }

    /// Decompiled virtual method #519 @ RVA 0x00103e80 (2 instructions, 1 basic blocks)
    pub fn method_519(&mut self) {
    }

    /// Decompiled virtual method #520 @ RVA 0x00103e80 (2 instructions, 1 basic blocks)
    pub fn method_520(&mut self) {
    }

    /// Decompiled virtual method #521 @ RVA 0x00103e80 (2 instructions, 1 basic blocks)
    pub fn method_521(&mut self) {
    }

    /// Decompiled virtual method #522 @ RVA 0x00103e80 (2 instructions, 1 basic blocks)
    pub fn method_522(&mut self) {
    }

    /// Decompiled virtual method #523 @ RVA 0x000d3090 (1 instructions, 1 basic blocks)
    pub fn method_523(&mut self) {
    }

    /// Decompiled virtual method #524 @ RVA 0x000d3090 (1 instructions, 1 basic blocks)
    pub fn method_524(&mut self) {
    }

    /// Decompiled virtual method #525 @ RVA 0x000d3090 (1 instructions, 1 basic blocks)
    pub fn method_525(&mut self) {
    }

    /// Decompiled virtual method #526 @ RVA 0x000d3090 (1 instructions, 1 basic blocks)
    pub fn method_526(&mut self) {
    }

    /// Decompiled virtual method #527 @ RVA 0x000d3090 (1 instructions, 1 basic blocks)
    pub fn method_527(&mut self) {
    }

    /// Decompiled virtual method #528 @ RVA 0x000d6920 (2 instructions, 1 basic blocks)
    pub fn method_528(&mut self) {
    }

    /// Decompiled virtual method #529 @ RVA 0x000d3090 (1 instructions, 1 basic blocks)
    pub fn method_529(&mut self) {
    }

    /// Decompiled virtual method #530 @ RVA 0x000d3090 (1 instructions, 1 basic blocks)
    pub fn method_530(&mut self) {
    }

    /// Decompiled virtual method #531 @ RVA 0x000d3090 (1 instructions, 1 basic blocks)
    pub fn method_531(&mut self) {
    }

    /// Decompiled virtual method #532 @ RVA 0x000d6920 (2 instructions, 1 basic blocks)
    pub fn method_532(&mut self) {
    }

    /// Decompiled virtual method #533 @ RVA 0x000d3090 (1 instructions, 1 basic blocks)
    pub fn method_533(&mut self) {
    }

    /// Decompiled virtual method #534 @ RVA 0x000d3090 (1 instructions, 1 basic blocks)
    pub fn method_534(&mut self) {
    }

    /// Decompiled virtual method #535 @ RVA 0x000d3090 (1 instructions, 1 basic blocks)
    pub fn method_535(&mut self) {
    }

    /// Decompiled virtual method #536 @ RVA 0x000d3090 (1 instructions, 1 basic blocks)
    pub fn method_536(&mut self) {
    }

    /// Decompiled virtual method #537 @ RVA 0x000d3090 (1 instructions, 1 basic blocks)
    pub fn method_537(&mut self) {
    }

    /// Decompiled virtual method #538 @ RVA 0x000d3090 (1 instructions, 1 basic blocks)
    pub fn method_538(&mut self) {
    }

    /// Decompiled virtual method #539 @ RVA 0x000d3090 (1 instructions, 1 basic blocks)
    pub fn method_539(&mut self) {
    }

    /// Decompiled virtual method #540 @ RVA 0x00103e80 (2 instructions, 1 basic blocks)
    pub fn method_540(&mut self) {
    }

    /// Decompiled virtual method #541 @ RVA 0x00103e80 (2 instructions, 1 basic blocks)
    pub fn method_541(&mut self) {
    }

    /// Decompiled virtual method #542 @ RVA 0x000d3090 (1 instructions, 1 basic blocks)
    pub fn method_542(&mut self) {
    }

    /// Decompiled virtual method #543 @ RVA 0x000d3090 (1 instructions, 1 basic blocks)
    pub fn method_543(&mut self) {
    }

    /// Decompiled virtual method #544 @ RVA 0x000d3090 (1 instructions, 1 basic blocks)
    pub fn method_544(&mut self) {
    }

    /// Decompiled virtual method #545 @ RVA 0x000d3090 (1 instructions, 1 basic blocks)
    pub fn method_545(&mut self) {
    }

    /// Decompiled virtual method #546 @ RVA 0x000d3090 (1 instructions, 1 basic blocks)
    pub fn method_546(&mut self) {
    }

    /// Decompiled virtual method #547 @ RVA 0x000d6920 (2 instructions, 1 basic blocks)
    pub fn method_547(&mut self) {
    }

    /// Decompiled virtual method #548 @ RVA 0x000d6920 (2 instructions, 1 basic blocks)
    pub fn method_548(&mut self) {
    }

    /// Decompiled virtual method #549 @ RVA 0x000d6920 (2 instructions, 1 basic blocks)
    pub fn method_549(&mut self) {
    }

    /// Decompiled virtual method #550 @ RVA 0x000d3090 (1 instructions, 1 basic blocks)
    pub fn method_550(&mut self) {
    }

    /// Decompiled virtual method #551 @ RVA 0x000d3090 (1 instructions, 1 basic blocks)
    pub fn method_551(&mut self) {
    }

    /// Decompiled virtual method #552 @ RVA 0x000d3090 (1 instructions, 1 basic blocks)
    pub fn method_552(&mut self) {
    }

    /// Decompiled virtual method #553 @ RVA 0x000d3090 (1 instructions, 1 basic blocks)
    pub fn method_553(&mut self) {
    }

    /// Decompiled virtual method #554 @ RVA 0x000d6920 (2 instructions, 1 basic blocks)
    pub fn method_554(&mut self) {
    }

    /// Decompiled virtual method #555 @ RVA 0x000d3090 (1 instructions, 1 basic blocks)
    pub fn method_555(&mut self) {
    }

    /// Decompiled virtual method #556 @ RVA 0x000d3090 (1 instructions, 1 basic blocks)
    pub fn method_556(&mut self) {
    }

    /// Decompiled virtual method #557 @ RVA 0x000d6920 (2 instructions, 1 basic blocks)
    pub fn method_557(&mut self) {
    }

    /// Decompiled virtual method #558 @ RVA 0x000d3090 (1 instructions, 1 basic blocks)
    pub fn method_558(&mut self) {
    }

    /// Decompiled virtual method #559 @ RVA 0x000d6920 (2 instructions, 1 basic blocks)
    pub fn method_559(&mut self) {
    }

    /// Decompiled virtual method #560 @ RVA 0x000d3090 (1 instructions, 1 basic blocks)
    pub fn method_560(&mut self) {
    }

    /// Decompiled virtual method #561 @ RVA 0x000d3090 (1 instructions, 1 basic blocks)
    pub fn method_561(&mut self) {
    }

    /// Decompiled virtual method #562 @ RVA 0x000d6920 (2 instructions, 1 basic blocks)
    pub fn method_562(&mut self) {
    }

    /// Decompiled virtual method #563 @ RVA 0x000d3090 (1 instructions, 1 basic blocks)
    pub fn method_563(&mut self) {
    }

    /// Decompiled virtual method #564 @ RVA 0x000d3090 (1 instructions, 1 basic blocks)
    pub fn method_564(&mut self) {
    }

    /// Decompiled virtual method #565 @ RVA 0x000d6920 (2 instructions, 1 basic blocks)
    pub fn method_565(&mut self) {
    }

    /// Decompiled virtual method #566 @ RVA 0x000d3090 (1 instructions, 1 basic blocks)
    pub fn method_566(&mut self) {
    }

    /// Decompiled virtual method #567 @ RVA 0x000d3090 (1 instructions, 1 basic blocks)
    pub fn method_567(&mut self) {
    }

    /// Decompiled virtual method #568 @ RVA 0x000d6920 (2 instructions, 1 basic blocks)
    pub fn method_568(&mut self) {
    }

    /// Decompiled virtual method #569 @ RVA 0x000d3090 (1 instructions, 1 basic blocks)
    pub fn method_569(&mut self) {
    }

    /// Decompiled virtual method #570 @ RVA 0x000d3090 (1 instructions, 1 basic blocks)
    pub fn method_570(&mut self) {
    }

    /// Decompiled virtual method #571 @ RVA 0x00103e70 (2 instructions, 1 basic blocks)
    pub fn method_571(&mut self) {
    }

    /// Decompiled virtual method #572 @ RVA 0x000d3090 (1 instructions, 1 basic blocks)
    pub fn method_572(&mut self) {
    }

    /// Decompiled virtual method #573 @ RVA 0x000d3090 (1 instructions, 1 basic blocks)
    pub fn method_573(&mut self) {
    }

    /// Decompiled virtual method #574 @ RVA 0x000d3090 (1 instructions, 1 basic blocks)
    pub fn method_574(&mut self) {
    }

    /// Decompiled virtual method #575 @ RVA 0x00103e70 (2 instructions, 1 basic blocks)
    pub fn method_575(&mut self) {
    }

    /// Decompiled virtual method #576 @ RVA 0x000d3090 (1 instructions, 1 basic blocks)
    pub fn method_576(&mut self) {
    }

    /// Decompiled virtual method #577 @ RVA 0x00103e70 (2 instructions, 1 basic blocks)
    pub fn method_577(&mut self) {
    }

    /// Decompiled virtual method #578 @ RVA 0x000d3090 (1 instructions, 1 basic blocks)
    pub fn method_578(&mut self) {
    }

    /// Decompiled virtual method #579 @ RVA 0x000d3090 (1 instructions, 1 basic blocks)
    pub fn method_579(&mut self) {
    }

    /// Decompiled virtual method #580 @ RVA 0x00103e80 (2 instructions, 1 basic blocks)
    pub fn method_580(&mut self) {
    }

    /// Decompiled virtual method #581 @ RVA 0x000d3090 (1 instructions, 1 basic blocks)
    pub fn method_581(&mut self) {
    }

    /// Decompiled virtual method #582 @ RVA 0x000d3090 (1 instructions, 1 basic blocks)
    pub fn method_582(&mut self) {
    }

    /// Decompiled virtual method #583 @ RVA 0x00103e80 (2 instructions, 1 basic blocks)
    pub fn method_583(&mut self) {
    }

    /// Decompiled virtual method #584 @ RVA 0x000d3090 (1 instructions, 1 basic blocks)
    pub fn method_584(&mut self) {
    }

    /// Decompiled virtual method #585 @ RVA 0x000d3090 (1 instructions, 1 basic blocks)
    pub fn method_585(&mut self) {
    }

    /// Decompiled virtual method #586 @ RVA 0x00103e80 (2 instructions, 1 basic blocks)
    pub fn method_586(&mut self) {
    }

    /// Decompiled virtual method #587 @ RVA 0x00103e70 (2 instructions, 1 basic blocks)
    pub fn method_587(&mut self) {
    }

    /// Decompiled virtual method #588 @ RVA 0x000d3090 (1 instructions, 1 basic blocks)
    pub fn method_588(&mut self) {
    }

    /// Decompiled virtual method #589 @ RVA 0x00103e80 (2 instructions, 1 basic blocks)
    pub fn method_589(&mut self) {
    }

    /// Decompiled virtual method #590 @ RVA 0x000d3090 (1 instructions, 1 basic blocks)
    pub fn method_590(&mut self) {
    }

    /// Decompiled virtual method #591 @ RVA 0x000d3090 (1 instructions, 1 basic blocks)
    pub fn method_591(&mut self) {
    }

    /// Decompiled virtual method #592 @ RVA 0x00103e80 (2 instructions, 1 basic blocks)
    pub fn method_592(&mut self) {
    }

    /// Decompiled virtual method #593 @ RVA 0x000d3090 (1 instructions, 1 basic blocks)
    pub fn method_593(&mut self) {
    }

    /// Decompiled virtual method #594 @ RVA 0x000d6920 (2 instructions, 1 basic blocks)
    pub fn method_594(&mut self) {
    }

    /// Decompiled virtual method #595 @ RVA 0x00103e80 (2 instructions, 1 basic blocks)
    pub fn method_595(&mut self) {
    }

    /// Decompiled virtual method #596 @ RVA 0x000d3090 (1 instructions, 1 basic blocks)
    pub fn method_596(&mut self) {
    }

    /// Decompiled virtual method #597 @ RVA 0x000d3090 (1 instructions, 1 basic blocks)
    pub fn method_597(&mut self) {
    }

    /// Decompiled virtual method #598 @ RVA 0x00103e80 (2 instructions, 1 basic blocks)
    pub fn method_598(&mut self) {
    }

    /// Decompiled virtual method #599 @ RVA 0x000d3090 (1 instructions, 1 basic blocks)
    pub fn method_599(&mut self) {
    }

    /// Decompiled virtual method #600 @ RVA 0x000d3090 (1 instructions, 1 basic blocks)
    pub fn method_600(&mut self) {
    }

    /// Decompiled virtual method #601 @ RVA 0x00103e80 (2 instructions, 1 basic blocks)
    pub fn method_601(&mut self) {
    }

    /// Decompiled virtual method #602 @ RVA 0x00103e80 (2 instructions, 1 basic blocks)
    pub fn method_602(&mut self) {
    }

    /// Decompiled virtual method #603 @ RVA 0x000d6920 (2 instructions, 1 basic blocks)
    pub fn method_603(&mut self) {
    }

    /// Decompiled virtual method #604 @ RVA 0x000d6920 (2 instructions, 1 basic blocks)
    pub fn method_604(&mut self) {
    }

    /// Decompiled virtual method #605 @ RVA 0x00103e80 (2 instructions, 1 basic blocks)
    pub fn method_605(&mut self) {
    }

    /// Decompiled virtual method #606 @ RVA 0x000d3090 (1 instructions, 1 basic blocks)
    pub fn method_606(&mut self) {
    }

    /// Decompiled virtual method #607 @ RVA 0x00103e80 (2 instructions, 1 basic blocks)
    pub fn method_607(&mut self) {
    }

    /// Decompiled virtual method #608 @ RVA 0x00103e80 (2 instructions, 1 basic blocks)
    pub fn method_608(&mut self) {
    }

    /// Decompiled virtual method #609 @ RVA 0x000d3090 (1 instructions, 1 basic blocks)
    pub fn method_609(&mut self) {
    }

    /// Decompiled virtual method #610 @ RVA 0x000d3090 (1 instructions, 1 basic blocks)
    pub fn method_610(&mut self) {
    }

    /// Decompiled virtual method #611 @ RVA 0x00103e80 (2 instructions, 1 basic blocks)
    pub fn method_611(&mut self) {
    }

    /// Decompiled virtual method #612 @ RVA 0x000d3090 (1 instructions, 1 basic blocks)
    pub fn method_612(&mut self) {
    }

    /// Decompiled virtual method #613 @ RVA 0x000d3090 (1 instructions, 1 basic blocks)
    pub fn method_613(&mut self) {
    }

    /// Decompiled virtual method #614 @ RVA 0x00103e80 (2 instructions, 1 basic blocks)
    pub fn method_614(&mut self) {
    }

    /// Decompiled virtual method #615 @ RVA 0x000d3090 (1 instructions, 1 basic blocks)
    pub fn method_615(&mut self) {
    }

    /// Decompiled virtual method #616 @ RVA 0x00103e80 (2 instructions, 1 basic blocks)
    pub fn method_616(&mut self) {
    }

    /// Decompiled virtual method #617 @ RVA 0x000d3090 (1 instructions, 1 basic blocks)
    pub fn method_617(&mut self) {
    }

    /// Decompiled virtual method #618 @ RVA 0x00103e70 (2 instructions, 1 basic blocks)
    pub fn method_618(&mut self) {
    }

    /// Decompiled virtual method #619 @ RVA 0x000d6920 (2 instructions, 1 basic blocks)
    pub fn method_619(&mut self) {
    }

    /// Decompiled virtual method #620 @ RVA 0x000d3090 (1 instructions, 1 basic blocks)
    pub fn method_620(&mut self) {
    }

    /// Decompiled virtual method #621 @ RVA 0x00103e80 (2 instructions, 1 basic blocks)
    pub fn method_621(&mut self) {
    }

    /// Decompiled virtual method #622 @ RVA 0x000d3090 (1 instructions, 1 basic blocks)
    pub fn method_622(&mut self) {
    }

    /// Decompiled virtual method #623 @ RVA 0x000d3090 (1 instructions, 1 basic blocks)
    pub fn method_623(&mut self) {
    }

    /// Decompiled virtual method #624 @ RVA 0x000d3090 (1 instructions, 1 basic blocks)
    pub fn method_624(&mut self) {
    }

    /// Decompiled virtual method #625 @ RVA 0x000d3090 (1 instructions, 1 basic blocks)
    pub fn method_625(&mut self) {
    }

    /// Decompiled virtual method #626 @ RVA 0x00103e80 (2 instructions, 1 basic blocks)
    pub fn method_626(&mut self) {
    }

    /// Decompiled virtual method #627 @ RVA 0x00103e80 (2 instructions, 1 basic blocks)
    pub fn method_627(&mut self) {
    }

    /// Decompiled virtual method #628 @ RVA 0x000d3090 (1 instructions, 1 basic blocks)
    pub fn method_628(&mut self) {
    }

    /// Decompiled virtual method #629 @ RVA 0x00584510 (2 instructions, 1 basic blocks)
    pub fn method_629(&mut self) {
    }

    /// Decompiled virtual method #630 @ RVA 0x000d3090 (1 instructions, 1 basic blocks)
    pub fn method_630(&mut self) {
    }

    /// Decompiled virtual method #631 @ RVA 0x00584510 (2 instructions, 1 basic blocks)
    pub fn method_631(&mut self) {
    }

    /// Decompiled virtual method #632 @ RVA 0x00103e80 (2 instructions, 1 basic blocks)
    pub fn method_632(&mut self) {
    }

    /// Decompiled virtual method #633 @ RVA 0x000d3090 (1 instructions, 1 basic blocks)
    pub fn method_633(&mut self) {
    }

    /// Decompiled virtual method #634 @ RVA 0x000d3090 (1 instructions, 1 basic blocks)
    pub fn method_634(&mut self) {
    }

    /// Decompiled virtual method #635 @ RVA 0x00103e80 (2 instructions, 1 basic blocks)
    pub fn method_635(&mut self) {
    }

    /// Decompiled virtual method #636 @ RVA 0x000d3090 (1 instructions, 1 basic blocks)
    pub fn method_636(&mut self) {
    }

    /// Decompiled virtual method #637 @ RVA 0x000d3090 (1 instructions, 1 basic blocks)
    pub fn method_637(&mut self) {
    }

    /// Decompiled virtual method #638 @ RVA 0x000d3090 (1 instructions, 1 basic blocks)
    pub fn method_638(&mut self) {
    }

    /// Decompiled virtual method #639 @ RVA 0x00103e70 (2 instructions, 1 basic blocks)
    pub fn method_639(&mut self) {
    }

    /// Decompiled virtual method #640 @ RVA 0x000d3090 (1 instructions, 1 basic blocks)
    pub fn method_640(&mut self) {
    }

    /// Decompiled virtual method #641 @ RVA 0x000d6920 (2 instructions, 1 basic blocks)
    pub fn method_641(&mut self) {
    }

    /// Decompiled virtual method #642 @ RVA 0x000d6920 (2 instructions, 1 basic blocks)
    pub fn method_642(&mut self) {
    }

    /// Decompiled virtual method #643 @ RVA 0x00103e70 (2 instructions, 1 basic blocks)
    pub fn method_643(&mut self) {
    }

    /// Decompiled virtual method #644 @ RVA 0x000d3090 (1 instructions, 1 basic blocks)
    pub fn method_644(&mut self) {
    }

    /// Decompiled virtual method #645 @ RVA 0x00103e70 (2 instructions, 1 basic blocks)
    pub fn method_645(&mut self) {
    }

    /// Decompiled virtual method #646 @ RVA 0x000d3090 (1 instructions, 1 basic blocks)
    pub fn method_646(&mut self) {
    }

}

impl CcClass for ccSaveData {
    fn class_name(&self) -> &'static str { Self::CLASS_NAME }
    fn vtable_rva(&self) -> u32 { Self::VTABLE_RVA }
    fn virtual_method_count(&self) -> usize { Self::METHOD_COUNT }
    fn base_classes(&self) -> &'static [&'static str] { Self::BASE_CLASSES }
}

/// Decompiled CC2 class: `ccSaveDataSys::system::gu` (VTable RVA: 0x008660c0, 1 virtual methods, 1 recovered fields)
/// Inherits: ?$SingletonPtr::VccSaveDataSys::system::gu::::::fnd::bl
#[derive(Debug, Clone, Default)]
pub struct ccSaveDataSys_system_gu {
    pub vtable_rva: u32,
    pub field_0x0000: u64,
}

impl ccSaveDataSys_system_gu {
    pub const CLASS_NAME: &'static str = "ccSaveDataSys::system::gu";
    pub const VTABLE_RVA: u32 = 0x008660c0;
    pub const METHOD_COUNT: usize = 1;
    pub const BASE_CLASSES: &'static [&'static str] = &["?$SingletonPtr::VccSaveDataSys::system::gu::::::fnd::bl"];
    pub const VIRTUAL_METHODS: &'static [u64] = &[0x00000001800f4460];

    pub fn new() -> Self {
        Self {
            vtable_rva: Self::VTABLE_RVA,
            field_0x0000: Default::default(),
        }
    }

    /// Decompiled virtual method #0 @ RVA 0x000f4460 (19 instructions, 3 basic blocks)
    pub fn method_00(&mut self) {
        // Writes to field at offset +0x0000
    }

}

impl CcClass for ccSaveDataSys_system_gu {
    fn class_name(&self) -> &'static str { Self::CLASS_NAME }
    fn vtable_rva(&self) -> u32 { Self::VTABLE_RVA }
    fn virtual_method_count(&self) -> usize { Self::METHOD_COUNT }
    fn base_classes(&self) -> &'static [&'static str] { Self::BASE_CLASSES }
}

/// Decompiled CC2 class: `ccSaveDataVol1::vol1` (VTable RVA: 0x008cac18, 648 virtual methods, 67 recovered fields)
/// Inherits: ccSaveData
#[derive(Debug, Clone, Default)]
pub struct ccSaveDataVol1_vol1 {
    pub vtable_rva: u32,
    pub field_0x0000: u64,
    pub field_0x0002: u16,
    pub field_0x0004: u16,
    pub field_0x0005: u32,
    pub field_0x0006: u16,
    pub field_0x0008: u32,
    pub field_0x000a: u16,
    pub field_0x000c: u16,
    pub field_0x000d: u8,
    pub field_0x000e: u16,
    pub field_0x0010: u32,
    pub field_0x0012: u16,
    pub field_0x0014: u16,
    pub field_0x0016: u16,
    pub field_0x0018: u32,
    pub field_0x001c: u32,
    pub field_0x0020: u32,
    pub field_0x0024: u16,
    pub field_0x0028: u32,
    pub field_0x002c: u64,
    pub field_0x002e: u16,
    pub field_0x0030: u32,
    pub field_0x0034: u16,
    pub field_0x0040: u32,
    pub field_0x0044: u16,
    pub field_0x0050: u32,
    pub field_0x0054: u16,
    pub field_0x0090: u64,
    pub field_0x020c: u16,
    pub field_0x020e: u16,
    pub field_0x0210: u16,
    pub field_0x0212: u16,
    pub field_0x0214: u16,
    pub field_0x0216: u16,
    pub field_0x0218: u16,
    pub field_0x021a: u16,
    pub field_0x021c: u16,
    pub field_0x021e: u16,
    pub field_0x0220: u16,
    pub field_0x0222: u16,
    pub field_0x0224: u16,
    pub field_0x0228: u32,
    pub field_0x022c: u16,
    pub field_0x022e: u16,
    pub field_0x02b0: u64,
    pub field_0x02b8: u64,
    pub field_0x030c: f32,
    pub field_0x04ee: u32,
    pub field_0x05a6: u16,
    pub field_0x05a8: u16,
    pub field_0x05aa: u8,
    pub field_0x05af: u8,
    pub field_0x05b0: u8,
    pub field_0x05b2: u8,
    pub field_0x05bc: u32,
    pub field_0x05c0: u16,
    pub field_0x0b60: u32,
    pub field_0x0bc0: u64,
    pub field_0x0be0: u64,
    pub field_0x105f: u32,
    pub field_0x1078: u64,
    pub field_0x15b0: u16,
    pub field_0x1600: u32,
    pub field_0x16a0: u32,
    pub field_0x4ee0: u32,
    pub field_0x4ee4: u32,
    pub field_0x4ee6: u32,
}

impl ccSaveDataVol1_vol1 {
    pub const CLASS_NAME: &'static str = "ccSaveDataVol1::vol1";
    pub const VTABLE_RVA: u32 = 0x008cac18;
    pub const METHOD_COUNT: usize = 648;
    pub const BASE_CLASSES: &'static [&'static str] = &["ccSaveData"];
    pub const VIRTUAL_METHODS: &'static [u64] = &[0x0000000180584540, 0x0000000180584550, 0x000000018060b1e0, 0x000000018060c330, 0x000000018060c530, 0x00000001800d3090, 0x0000000180103e70, 0x00000001800d3090, 0x00000001800d3090, 0x00000001800d3090, 0x00000001800d3090, 0x00000001800d3090, 0x00000001800d3090, 0x00000001800d3090, 0x00000001800d3090, 0x00000001800d3090, 0x00000001800d3090, 0x00000001800d3090, 0x000000018060bd80, 0x000000018060ce70, 0x00000001805844e0, 0x0000000180584560, 0x0000000180584570, 0x00000001805845a0, 0x0000000180103e70, 0x00000001805845c0, 0x00000001805845d0, 0x0000000180584600, 0x0000000180584610, 0x0000000180584640, 0x0000000180584650, 0x0000000180584680, 0x0000000180584690, 0x00000001805846a0, 0x00000001805846b0, 0x00000001805846c0, 0x00000001805846d0, 0x00000001805846e0, 0x00000001805846f0, 0x0000000180584700, 0x0000000180584710, 0x0000000180584720, 0x0000000180584740, 0x0000000180584760, 0x0000000180103e80, 0x0000000180584780, 0x0000000180612700, 0x0000000180612740, 0x0000000180612780, 0x0000000180103e80, 0x0000000180103e80, 0x00000001800d3090, 0x00000001800d3090, 0x0000000180584790, 0x000000018060cf10, 0x000000018060d6f0, 0x000000018060d700, 0x00000001805847b0, 0x000000018060d910, 0x000000018060d950, 0x00000001805847d0, 0x0000000180584830, 0x0000000180584890, 0x0000000180103e70, 0x00000001805848f0, 0x000000018060d180, 0x0000000180584910, 0x000000018060cf40, 0x0000000180584980, 0x000000018060d320, 0x000000018060d520, 0x00000001805849f0, 0x000000018060d730, 0x0000000180584a60, 0x000000018060d850, 0x000000018060d8c0, 0x000000018060da00, 0x000000018060e6d0, 0x000000018060e4a0, 0x000000018060ea60, 0x000000018060e8b0, 0x000000018060ec20, 0x0000000180584ad0, 0x0000000180584b30, 0x0000000180584b90, 0x000000018060db80, 0x000000018060dd40, 0x000000018060dfc0, 0x000000018060e100, 0x000000018060e1e0, 0x000000018060e3c0, 0x000000018060eca0, 0x0000000180584bf0, 0x000000018060ece0, 0x000000018060ed20, 0x000000018060ee30, 0x00000001800d3090, 0x0000000180103e80, 0x0000000180103e80, 0x000000018060eec0, 0x000000018060ef20, 0x000000018060f070, 0x000000018060f110, 0x0000000180584c10, 0x0000000180584c60, 0x0000000180584cb0, 0x0000000180103e80, 0x000000018060efa0, 0x000000018060f000, 0x000000018060f1d0, 0x0000000180584d00, 0x0000000180584d20, 0x0000000180584d30, 0x0000000180584d40, 0x0000000180584d50, 0x0000000180584d60, 0x0000000180584d70, 0x0000000180584d80, 0x0000000180615860, 0x0000000180584d90, 0x0000000180615890, 0x0000000180615ab0, 0x0000000180584da0, 0x0000000180584db0, 0x0000000180584dc0, 0x0000000180610200, 0x0000000180584dd0, 0x0000000180584df0, 0x0000000180584e10, 0x0000000180584e20, 0x0000000180584e30, 0x0000000180584e50, 0x0000000180584e60, 0x0000000180584e70, 0x0000000180584e80, 0x0000000180584eb0, 0x0000000180584ec0, 0x0000000180584ed0, 0x0000000180584ee0, 0x0000000180584ef0, 0x0000000180584f00, 0x0000000180584f10, 0x0000000180584f20, 0x0000000180584f30, 0x0000000180584f40, 0x0000000180584f50, 0x0000000180584f60, 0x0000000180584f70, 0x0000000180584f80, 0x0000000180584f90, 0x0000000180584fa0, 0x0000000180584fb0, 0x0000000180584fc0, 0x0000000180584fd0, 0x0000000180584fe0, 0x0000000180584ff0, 0x000000018060f230, 0x000000018060f300, 0x000000018060f3a0, 0x000000018060f460, 0x000000018060f4b0, 0x000000018060f5f0, 0x000000018060f630, 0x000000018060f680, 0x000000018060f6e0, 0x000000018060f730, 0x000000018060f7a0, 0x000000018060f7b0, 0x000000018060f7d0, 0x000000018060f820, 0x000000018060f8a0, 0x000000018060f8d0, 0x000000018060f8f0, 0x000000018060f910, 0x000000018060f930, 0x000000018060f950, 0x000000018060f980, 0x000000018060f9b0, 0x000000018060f9d0, 0x00000001800d6920, 0x000000018060f9f0, 0x000000018060fae0, 0x000000018060fb30, 0x000000018060fb50, 0x000000018060fb90, 0x000000018060fbd0, 0x000000018060fbf0, 0x000000018060fc30, 0x000000018060fc70, 0x000000018060fca0, 0x000000018060fce0, 0x000000018060fd20, 0x000000018060fd40, 0x000000018060fdb0, 0x000000018060fe10, 0x000000018060ff10, 0x000000018060ffc0, 0x0000000180610020, 0x0000000180610080, 0x0000000180610110, 0x00000001806101a0, 0x00000001806101c0, 0x0000000180610230, 0x0000000180610310, 0x0000000180610490, 0x0000000180610580, 0x0000000180610660, 0x0000000180610750, 0x0000000180610840, 0x0000000180610990, 0x0000000180610ae0, 0x0000000180610b50, 0x0000000180610bb0, 0x0000000180610d70, 0x0000000180610df0, 0x0000000180610e50, 0x0000000180610f30, 0x0000000180610fc0, 0x0000000180611030, 0x00000001806110a0, 0x00000001800d3090, 0x0000000180611110, 0x0000000180611170, 0x0000000180611240, 0x00000001806111d0, 0x00000001800d3090, 0x0000000180585090, 0x0000000180611340, 0x0000000180611370, 0x00000001806113a0, 0x00000001806113c0, 0x00000001805850a0, 0x0000000180611400, 0x0000000180611450, 0x00000001806114a0, 0x00000001806114b0, 0x00000001806114e0, 0x0000000180611500, 0x0000000180611510, 0x0000000180611520, 0x0000000180611530, 0x0000000180611540, 0x0000000180611570, 0x00000001806115a0, 0x00000001806115c0, 0x0000000180611610, 0x0000000180611650, 0x0000000180611690, 0x00000001806116b0, 0x00000001806116d0, 0x0000000180611740, 0x0000000180611760, 0x0000000180611790, 0x00000001806117b0, 0x00000001806117f0, 0x0000000180611810, 0x0000000180611830, 0x00000001805850b0, 0x00000001805850c0, 0x00000001805850d0, 0x0000000180611850, 0x0000000180611870, 0x00000001806118e0, 0x0000000180611900, 0x0000000180611920, 0x00000001805850e0, 0x00000001805850f0, 0x0000000180611940, 0x0000000180611960, 0x00000001806119d0, 0x00000001806119f0, 0x0000000180611a10, 0x0000000180611a30, 0x0000000180611a70, 0x0000000180611ac0, 0x0000000180611ad0, 0x0000000180611af0, 0x0000000180611b10, 0x0000000180611b30, 0x0000000180611b50, 0x0000000180611b60, 0x0000000180611b80, 0x0000000180611ba0, 0x0000000180611bc0, 0x0000000180611be0, 0x0000000180611c30, 0x0000000180611cb0, 0x0000000180611d00, 0x0000000180611d50, 0x0000000180103e70, 0x0000000180585100, 0x0000000180585110, 0x0000000180611da0, 0x0000000180611dc0, 0x0000000180611df0, 0x0000000180611e10, 0x0000000180611e30, 0x0000000180611e50, 0x0000000180611ef0, 0x0000000180611f10, 0x0000000180611f30, 0x0000000180611f40, 0x0000000180611f70, 0x0000000180611fc0, 0x0000000180612000, 0x0000000180612040, 0x0000000180612070, 0x00000001806120a0, 0x00000001806120d0, 0x00000001806120f0, 0x0000000180612140, 0x0000000180612160, 0x0000000180612170, 0x0000000180612180, 0x00000001806121a0, 0x00000001806121c0, 0x00000001806121d0, 0x00000001806121e0, 0x00000001806121f0, 0x0000000180612200, 0x0000000180612210, 0x0000000180612220, 0x0000000180612250, 0x0000000180612280, 0x00000001806122c0, 0x0000000180585120, 0x0000000180585130, 0x0000000180612300, 0x0000000180612310, 0x0000000180612320, 0x0000000180612330, 0x0000000180612340, 0x0000000180612350, 0x0000000180612360, 0x0000000180612370, 0x0000000180612380, 0x00000001806124a0, 0x00000001806124c0, 0x00000001806124e0, 0x0000000180612500, 0x0000000180612410, 0x0000000180612520, 0x0000000180612580, 0x00000001806125a0, 0x00000001806125c0, 0x0000000180612390, 0x00000001806123a0, 0x00000001806123b0, 0x00000001806123c0, 0x00000001806123d0, 0x00000001806123e0, 0x00000001806123f0, 0x0000000180612400, 0x00000001800d3090, 0x00000001800d3090, 0x00000001800d6920, 0x00000001806125e0, 0x0000000180612610, 0x0000000180612640, 0x0000000180612670, 0x00000001806126a0, 0x00000001806126d0, 0x00000001800d3090, 0x00000001800d3090, 0x00000001800d6920, 0x0000000180585140, 0x0000000180585150, 0x0000000180585160, 0x0000000180585170, 0x00000001806127a0, 0x0000000180612810, 0x0000000180612c50, 0x0000000180612a40, 0x00000001806134b0, 0x0000000180613a70, 0x00000001806137a0, 0x0000000180612880, 0x0000000180612dd0, 0x00000001806131c0, 0x0000000180613290, 0x0000000180613450, 0x0000000180613c20, 0x0000000180613d40, 0x0000000180613d60, 0x0000000180613dd0, 0x0000000180613e30, 0x0000000180613ec0, 0x0000000180103e80, 0x0000000180613f20, 0x0000000180585180, 0x0000000180103e80, 0x00000001800d3090, 0x0000000180103e70, 0x0000000180103e70, 0x0000000180613fb0, 0x0000000180614190, 0x0000000180614860, 0x0000000180614a60, 0x0000000180614b60, 0x0000000180614c00, 0x0000000180614d40, 0x0000000180614d80, 0x0000000180585190, 0x00000001805851a0, 0x0000000180614da0, 0x0000000180614e20, 0x0000000180614f10, 0x0000000180614f90, 0x0000000180615030, 0x0000000180615090, 0x00000001806150b0, 0x0000000180615120, 0x00000001805851c0, 0x00000001805851d0, 0x00000001805851e0, 0x00000001805851f0, 0x00000001800d3090, 0x0000000180615170, 0x00000001800d3090, 0x0000000180615190, 0x00000001800d6920, 0x00000001806151b0, 0x00000001806151e0, 0x0000000180615210, 0x0000000180615240, 0x0000000180615270, 0x00000001806152a0, 0x00000001806152d0, 0x00000001806152f0, 0x0000000180615310, 0x0000000180615330, 0x0000000180615350, 0x0000000180615370, 0x0000000180615390, 0x00000001806153b0, 0x00000001806153d0, 0x00000001806153f0, 0x0000000180585200, 0x0000000180585210, 0x0000000180585220, 0x0000000180585230, 0x0000000180585240, 0x0000000180585250, 0x0000000180585260, 0x0000000180585270, 0x0000000180585280, 0x0000000180585290, 0x00000001805852a0, 0x00000001805852b0, 0x00000001805852c0, 0x00000001805852e0, 0x00000001805852f0, 0x0000000180585300, 0x0000000180615410, 0x0000000180615440, 0x00000001806154a0, 0x0000000180615470, 0x0000000180615550, 0x0000000180615580, 0x00000001806155b0, 0x00000001806154f0, 0x0000000180615520, 0x0000000180585310, 0x00000001806155e0, 0x0000000180615600, 0x0000000180615620, 0x0000000180585320, 0x0000000180585330, 0x00000001806162b0, 0x00000001806162c0, 0x00000001806162f0, 0x00000001806162d0, 0x0000000180616310, 0x0000000180616370, 0x0000000180615640, 0x00000001806156d0, 0x00000001806156f0, 0x0000000180615710, 0x00000001806157b0, 0x00000001806157d0, 0x0000000180585340, 0x00000001806157f0, 0x0000000180103e70, 0x0000000180615840, 0x00000001800d3090, 0x0000000180103e80, 0x0000000180584540, 0x0000000180615b20, 0x0000000180615b30, 0x0000000180615b40, 0x0000000180615b50, 0x0000000180615b60, 0x0000000180615b80, 0x0000000180615b70, 0x0000000180615ba0, 0x0000000180615bd0, 0x0000000180615bc0, 0x0000000180615ad0, 0x0000000180615ae0, 0x0000000180615b00, 0x0000000180615b10, 0x0000000180615bf0, 0x0000000180615c10, 0x00000001800d3090, 0x00000001800d3090, 0x0000000180615cb0, 0x00000001800d3090, 0x0000000180103e80, 0x0000000180615c30, 0x00000001800d3090, 0x00000001800d3090, 0x0000000180103e80, 0x0000000180103e80, 0x00000001800d3090, 0x0000000180615cd0, 0x0000000180615cf0, 0x0000000180615d10, 0x0000000180615da0, 0x0000000180615dc0, 0x0000000180103e80, 0x0000000180103e80, 0x0000000180103e80, 0x0000000180103e80, 0x0000000180615df0, 0x0000000180615e10, 0x0000000180615e30, 0x0000000180615e50, 0x0000000180615f90, 0x00000001800d6920, 0x0000000180615fe0, 0x0000000180616000, 0x0000000180616020, 0x00000001800d6920, 0x0000000180615e70, 0x0000000180615e90, 0x0000000180615eb0, 0x0000000180615ed0, 0x0000000180615ef0, 0x00000001800d3090, 0x00000001800d3090, 0x0000000180103e80, 0x0000000180103e80, 0x0000000180615f10, 0x0000000180615f50, 0x0000000180615f70, 0x0000000180615f30, 0x00000001800d3090, 0x0000000180616390, 0x00000001806163a0, 0x00000001806163b0, 0x00000001806163d0, 0x00000001806163e0, 0x00000001806163f0, 0x0000000180616420, 0x0000000180616400, 0x0000000180616430, 0x0000000180616460, 0x0000000180616490, 0x0000000180616040, 0x00000001806164c0, 0x0000000180616500, 0x0000000180616540, 0x0000000180616580, 0x00000001806165c0, 0x0000000180616600, 0x0000000180616640, 0x0000000180616680, 0x00000001806166c0, 0x0000000180616700, 0x0000000180616740, 0x0000000180616780, 0x0000000180103e70, 0x00000001800d3090, 0x00000001800d3090, 0x00000001800d3090, 0x0000000180103e70, 0x00000001800d3090, 0x0000000180103e70, 0x00000001800d3090, 0x00000001800d3090, 0x0000000180103e80, 0x00000001800d3090, 0x00000001800d3090, 0x0000000180103e80, 0x00000001800d3090, 0x00000001800d3090, 0x0000000180103e80, 0x0000000180103e70, 0x00000001800d3090, 0x0000000180103e80, 0x00000001800d3090, 0x00000001800d3090, 0x0000000180103e80, 0x00000001800d3090, 0x00000001800d6920, 0x0000000180103e80, 0x00000001800d3090, 0x00000001800d3090, 0x0000000180103e80, 0x00000001800d3090, 0x00000001800d3090, 0x0000000180103e80, 0x0000000180103e80, 0x00000001800d6920, 0x00000001800d6920, 0x0000000180103e80, 0x00000001800d3090, 0x0000000180103e80, 0x0000000180103e80, 0x00000001800d3090, 0x00000001800d3090, 0x0000000180103e80, 0x00000001800d3090, 0x00000001800d3090, 0x0000000180103e80, 0x00000001800d3090, 0x0000000180103e80, 0x00000001800d3090, 0x0000000180103e70, 0x00000001800d6920, 0x00000001800d3090, 0x0000000180103e80, 0x00000001800d3090, 0x00000001800d3090, 0x00000001800d3090, 0x00000001800d3090, 0x0000000180103e80, 0x0000000180103e80, 0x00000001800d3090, 0x0000000180584510, 0x00000001800d3090, 0x0000000180584510, 0x0000000180103e80, 0x00000001800d3090, 0x00000001800d3090, 0x0000000180103e80, 0x00000001800d3090, 0x00000001800d3090, 0x0000000180585350, 0x0000000180585370, 0x00000001800d3090, 0x00000001800d6920, 0x00000001800d6920, 0x0000000180103e70, 0x00000001806167c0, 0x00000001806167d0, 0x00000001800d3090, 0x0000000180585390];

    pub fn new() -> Self {
        Self {
            vtable_rva: Self::VTABLE_RVA,
            field_0x0000: Default::default(),
            field_0x0002: Default::default(),
            field_0x0004: Default::default(),
            field_0x0005: Default::default(),
            field_0x0006: Default::default(),
            field_0x0008: Default::default(),
            field_0x000a: Default::default(),
            field_0x000c: Default::default(),
            field_0x000d: Default::default(),
            field_0x000e: Default::default(),
            field_0x0010: Default::default(),
            field_0x0012: Default::default(),
            field_0x0014: Default::default(),
            field_0x0016: Default::default(),
            field_0x0018: Default::default(),
            field_0x001c: Default::default(),
            field_0x0020: Default::default(),
            field_0x0024: Default::default(),
            field_0x0028: Default::default(),
            field_0x002c: Default::default(),
            field_0x002e: Default::default(),
            field_0x0030: Default::default(),
            field_0x0034: Default::default(),
            field_0x0040: Default::default(),
            field_0x0044: Default::default(),
            field_0x0050: Default::default(),
            field_0x0054: Default::default(),
            field_0x0090: Default::default(),
            field_0x020c: Default::default(),
            field_0x020e: Default::default(),
            field_0x0210: Default::default(),
            field_0x0212: Default::default(),
            field_0x0214: Default::default(),
            field_0x0216: Default::default(),
            field_0x0218: Default::default(),
            field_0x021a: Default::default(),
            field_0x021c: Default::default(),
            field_0x021e: Default::default(),
            field_0x0220: Default::default(),
            field_0x0222: Default::default(),
            field_0x0224: Default::default(),
            field_0x0228: Default::default(),
            field_0x022c: Default::default(),
            field_0x022e: Default::default(),
            field_0x02b0: Default::default(),
            field_0x02b8: Default::default(),
            field_0x030c: Default::default(),
            field_0x04ee: Default::default(),
            field_0x05a6: Default::default(),
            field_0x05a8: Default::default(),
            field_0x05aa: Default::default(),
            field_0x05af: Default::default(),
            field_0x05b0: Default::default(),
            field_0x05b2: Default::default(),
            field_0x05bc: Default::default(),
            field_0x05c0: Default::default(),
            field_0x0b60: Default::default(),
            field_0x0bc0: Default::default(),
            field_0x0be0: Default::default(),
            field_0x105f: Default::default(),
            field_0x1078: Default::default(),
            field_0x15b0: Default::default(),
            field_0x1600: Default::default(),
            field_0x16a0: Default::default(),
            field_0x4ee0: Default::default(),
            field_0x4ee4: Default::default(),
            field_0x4ee6: Default::default(),
        }
    }

    /// Decompiled virtual method #0 @ RVA 0x00584540 (2 instructions, 1 basic blocks)
    pub fn method_00(&mut self) {
    }

    /// Decompiled virtual method #1 @ RVA 0x00584550 (2 instructions, 1 basic blocks)
    pub fn method_01(&mut self) {
    }

    /// Decompiled virtual method #2 @ RVA 0x0060b1e0 (102 instructions, 11 basic blocks)
    pub fn method_02(&mut self) {
    }

    /// Decompiled virtual method #3 @ RVA 0x0060c330 (61 instructions, 11 basic blocks)
    pub fn method_03(&mut self) {
    }

    /// Decompiled virtual method #4 @ RVA 0x0060c530 (120 instructions, 4 basic blocks)
    pub fn method_04(&mut self) {
    }

    /// Decompiled virtual method #5 @ RVA 0x000d3090 (1 instructions, 1 basic blocks)
    pub fn method_05(&mut self) {
    }

    /// Decompiled virtual method #6 @ RVA 0x00103e70 (2 instructions, 1 basic blocks)
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

    /// Decompiled virtual method #11 @ RVA 0x000d3090 (1 instructions, 1 basic blocks)
    pub fn method_11(&mut self) {
    }

    /// Decompiled virtual method #12 @ RVA 0x000d3090 (1 instructions, 1 basic blocks)
    pub fn method_12(&mut self) {
    }

    /// Decompiled virtual method #13 @ RVA 0x000d3090 (1 instructions, 1 basic blocks)
    pub fn method_13(&mut self) {
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

    /// Decompiled virtual method #17 @ RVA 0x000d3090 (1 instructions, 1 basic blocks)
    pub fn method_17(&mut self) {
    }

    /// Decompiled virtual method #18 @ RVA 0x0060bd80 (122 instructions, 22 basic blocks)
    pub fn method_18(&mut self) {
    }

    /// Decompiled virtual method #19 @ RVA 0x0060ce70 (33 instructions, 8 basic blocks)
    pub fn method_19(&mut self) {
        // Writes to field at offset +0x030c
    }

    /// Decompiled virtual method #20 @ RVA 0x005844e0 (2 instructions, 1 basic blocks)
    pub fn method_20(&mut self) {
    }

    /// Decompiled virtual method #21 @ RVA 0x00584560 (2 instructions, 1 basic blocks)
    pub fn method_21(&mut self) {
    }

    /// Decompiled virtual method #22 @ RVA 0x00584570 (9 instructions, 1 basic blocks)
    pub fn method_22(&mut self) {
        // Writes to field at offset +0x05bc
    }

    /// Decompiled virtual method #23 @ RVA 0x005845a0 (5 instructions, 1 basic blocks)
    pub fn method_23(&mut self) {
        // Writes to field at offset +0x05bc
    }

    /// Decompiled virtual method #24 @ RVA 0x00103e70 (2 instructions, 1 basic blocks)
    pub fn method_24(&mut self) {
    }

    /// Decompiled virtual method #25 @ RVA 0x005845c0 (2 instructions, 1 basic blocks)
    pub fn method_25(&mut self) {
    }

    /// Decompiled virtual method #26 @ RVA 0x005845d0 (9 instructions, 3 basic blocks)
    pub fn method_26(&mut self) {
    }

    /// Decompiled virtual method #27 @ RVA 0x00584600 (2 instructions, 1 basic blocks)
    pub fn method_27(&mut self) {
    }

    /// Decompiled virtual method #28 @ RVA 0x00584610 (6 instructions, 2 basic blocks)
    pub fn method_28(&mut self) {
    }

    /// Decompiled virtual method #29 @ RVA 0x00584640 (2 instructions, 1 basic blocks)
    pub fn method_29(&mut self) {
    }

    /// Decompiled virtual method #30 @ RVA 0x00584650 (6 instructions, 2 basic blocks)
    pub fn method_30(&mut self) {
    }

    /// Decompiled virtual method #31 @ RVA 0x00584680 (2 instructions, 1 basic blocks)
    pub fn method_31(&mut self) {
    }

    /// Decompiled virtual method #32 @ RVA 0x00584690 (3 instructions, 1 basic blocks)
    pub fn method_32(&mut self) {
    }

    /// Decompiled virtual method #33 @ RVA 0x005846a0 (2 instructions, 1 basic blocks)
    pub fn method_33(&mut self) {
    }

    /// Decompiled virtual method #34 @ RVA 0x005846b0 (2 instructions, 1 basic blocks)
    pub fn method_34(&mut self) {
    }

    /// Decompiled virtual method #35 @ RVA 0x005846c0 (3 instructions, 1 basic blocks)
    pub fn method_35(&mut self) {
    }

    /// Decompiled virtual method #36 @ RVA 0x005846d0 (3 instructions, 1 basic blocks)
    pub fn method_36(&mut self) {
    }

    /// Decompiled virtual method #37 @ RVA 0x005846e0 (2 instructions, 1 basic blocks)
    pub fn method_37(&mut self) {
    }

    /// Decompiled virtual method #38 @ RVA 0x005846f0 (2 instructions, 1 basic blocks)
    pub fn method_38(&mut self) {
    }

    /// Decompiled virtual method #39 @ RVA 0x00584700 (2 instructions, 1 basic blocks)
    pub fn method_39(&mut self) {
    }

    /// Decompiled virtual method #40 @ RVA 0x00584710 (2 instructions, 1 basic blocks)
    pub fn method_40(&mut self) {
    }

    /// Decompiled virtual method #41 @ RVA 0x00584720 (5 instructions, 1 basic blocks)
    pub fn method_41(&mut self) {
    }

    /// Decompiled virtual method #42 @ RVA 0x00584740 (6 instructions, 1 basic blocks)
    pub fn method_42(&mut self) {
    }

    /// Decompiled virtual method #43 @ RVA 0x00584760 (5 instructions, 1 basic blocks)
    pub fn method_43(&mut self) {
    }

    /// Decompiled virtual method #44 @ RVA 0x00103e80 (2 instructions, 1 basic blocks)
    pub fn method_44(&mut self) {
    }

    /// Decompiled virtual method #45 @ RVA 0x00584780 (3 instructions, 1 basic blocks)
    pub fn method_45(&mut self) {
    }

    /// Decompiled virtual method #46 @ RVA 0x00612700 (9 instructions, 3 basic blocks)
    pub fn method_46(&mut self) {
    }

    /// Decompiled virtual method #47 @ RVA 0x00612740 (16 instructions, 6 basic blocks)
    pub fn method_47(&mut self) {
    }

    /// Decompiled virtual method #48 @ RVA 0x00612780 (8 instructions, 4 basic blocks)
    pub fn method_48(&mut self) {
        // Writes to field at offset +0x15b0
    }

    /// Decompiled virtual method #49 @ RVA 0x00103e80 (2 instructions, 1 basic blocks)
    pub fn method_49(&mut self) {
    }

    /// Decompiled virtual method #50 @ RVA 0x00103e80 (2 instructions, 1 basic blocks)
    pub fn method_50(&mut self) {
    }

    /// Decompiled virtual method #51 @ RVA 0x000d3090 (1 instructions, 1 basic blocks)
    pub fn method_51(&mut self) {
    }

    /// Decompiled virtual method #52 @ RVA 0x000d3090 (1 instructions, 1 basic blocks)
    pub fn method_52(&mut self) {
    }

    /// Decompiled virtual method #53 @ RVA 0x00584790 (7 instructions, 1 basic blocks)
    pub fn method_53(&mut self) {
    }

    /// Decompiled virtual method #54 @ RVA 0x0060cf10 (14 instructions, 5 basic blocks)
    pub fn method_54(&mut self) {
    }

    /// Decompiled virtual method #55 @ RVA 0x0060d6f0 (4 instructions, 1 basic blocks)
    pub fn method_55(&mut self) {
    }

    /// Decompiled virtual method #56 @ RVA 0x0060d700 (12 instructions, 5 basic blocks)
    pub fn method_56(&mut self) {
    }

    /// Decompiled virtual method #57 @ RVA 0x005847b0 (8 instructions, 1 basic blocks)
    pub fn method_57(&mut self) {
    }

    /// Decompiled virtual method #58 @ RVA 0x0060d910 (19 instructions, 8 basic blocks)
    pub fn method_58(&mut self) {
    }

    /// Decompiled virtual method #59 @ RVA 0x0060d950 (42 instructions, 9 basic blocks)
    pub fn method_59(&mut self) {
    }

    /// Decompiled virtual method #60 @ RVA 0x005847d0 (97 instructions, 1 basic blocks)
    pub fn method_60(&mut self) {
        // Writes to field at offset +0x05c0
    }

    /// Decompiled virtual method #61 @ RVA 0x00584830 (64 instructions, 1 basic blocks)
    pub fn method_61(&mut self) {
        // Writes to field at offset +0x05c0
    }

    /// Decompiled virtual method #62 @ RVA 0x00584890 (34 instructions, 1 basic blocks)
    pub fn method_62(&mut self) {
        // Writes to field at offset +0x05c0
    }

    /// Decompiled virtual method #63 @ RVA 0x00103e70 (2 instructions, 1 basic blocks)
    pub fn method_63(&mut self) {
    }

    /// Decompiled virtual method #64 @ RVA 0x005848f0 (4 instructions, 1 basic blocks)
    pub fn method_64(&mut self) {
        // Writes to field at offset +0x05c0
    }

    /// Decompiled virtual method #65 @ RVA 0x0060d180 (113 instructions, 31 basic blocks)
    pub fn method_65(&mut self) {
    }

    /// Decompiled virtual method #66 @ RVA 0x00584910 (27 instructions, 1 basic blocks)
    pub fn method_66(&mut self) {
    }

    /// Decompiled virtual method #67 @ RVA 0x0060cf40 (142 instructions, 35 basic blocks)
    pub fn method_67(&mut self) {
    }

    /// Decompiled virtual method #68 @ RVA 0x00584980 (161 instructions, 1 basic blocks)
    pub fn method_68(&mut self) {
    }

    /// Decompiled virtual method #69 @ RVA 0x0060d320 (146 instructions, 43 basic blocks)
    pub fn method_69(&mut self) {
        // Writes to field at offset +0x0000
        // Writes to field at offset +0x0000
    }

    /// Decompiled virtual method #70 @ RVA 0x0060d520 (126 instructions, 37 basic blocks)
    pub fn method_70(&mut self) {
    }

    /// Decompiled virtual method #71 @ RVA 0x005849f0 (165 instructions, 1 basic blocks)
    pub fn method_71(&mut self) {
    }

    /// Decompiled virtual method #72 @ RVA 0x0060d730 (76 instructions, 24 basic blocks)
    pub fn method_72(&mut self) {
        // Writes to field at offset +0x0000
    }

    /// Decompiled virtual method #73 @ RVA 0x00584a60 (137 instructions, 1 basic blocks)
    pub fn method_73(&mut self) {
    }

    /// Decompiled virtual method #74 @ RVA 0x0060d850 (12 instructions, 4 basic blocks)
    pub fn method_74(&mut self) {
    }

    /// Decompiled virtual method #75 @ RVA 0x0060d8c0 (22 instructions, 4 basic blocks)
    pub fn method_75(&mut self) {
    }

    /// Decompiled virtual method #76 @ RVA 0x0060da00 (107 instructions, 20 basic blocks)
    pub fn method_76(&mut self) {
        // Writes to field at offset +0x0000
    }

    /// Decompiled virtual method #77 @ RVA 0x0060e6d0 (25 instructions, 6 basic blocks)
    pub fn method_77(&mut self) {
    }

    /// Decompiled virtual method #78 @ RVA 0x0060e4a0 (146 instructions, 38 basic blocks)
    pub fn method_78(&mut self) {
        // Writes to field at offset +0x0000
        // Writes to field at offset +0x0004
        // Writes to field at offset +0x0006
        // Writes to field at offset +0x0008
        // Writes to field at offset +0x000a
        // Writes to field at offset +0x000c
    }

    /// Decompiled virtual method #79 @ RVA 0x0060ea60 (19 instructions, 4 basic blocks)
    pub fn method_79(&mut self) {
    }

    /// Decompiled virtual method #80 @ RVA 0x0060e8b0 (118 instructions, 31 basic blocks)
    pub fn method_80(&mut self) {
    }

    /// Decompiled virtual method #81 @ RVA 0x0060ec20 (58 instructions, 8 basic blocks)
    pub fn method_81(&mut self) {
    }

    /// Decompiled virtual method #82 @ RVA 0x00584ad0 (101 instructions, 1 basic blocks)
    pub fn method_82(&mut self) {
    }

    /// Decompiled virtual method #83 @ RVA 0x00584b30 (68 instructions, 1 basic blocks)
    pub fn method_83(&mut self) {
    }

    /// Decompiled virtual method #84 @ RVA 0x00584b90 (38 instructions, 1 basic blocks)
    pub fn method_84(&mut self) {
    }

    /// Decompiled virtual method #85 @ RVA 0x0060db80 (116 instructions, 20 basic blocks)
    pub fn method_85(&mut self) {
        // Writes to field at offset +0x0000
    }

    /// Decompiled virtual method #86 @ RVA 0x0060dd40 (136 instructions, 21 basic blocks)
    pub fn method_86(&mut self) {
    }

    /// Decompiled virtual method #87 @ RVA 0x0060dfc0 (86 instructions, 18 basic blocks)
    pub fn method_87(&mut self) {
        // Writes to field at offset +0x0000
    }

    /// Decompiled virtual method #88 @ RVA 0x0060e100 (64 instructions, 13 basic blocks)
    pub fn method_88(&mut self) {
    }

    /// Decompiled virtual method #89 @ RVA 0x0060e1e0 (132 instructions, 40 basic blocks)
    pub fn method_89(&mut self) {
    }

    /// Decompiled virtual method #90 @ RVA 0x0060e3c0 (62 instructions, 10 basic blocks)
    pub fn method_90(&mut self) {
    }

    /// Decompiled virtual method #91 @ RVA 0x0060eca0 (21 instructions, 8 basic blocks)
    pub fn method_91(&mut self) {
    }

    /// Decompiled virtual method #92 @ RVA 0x00584bf0 (8 instructions, 1 basic blocks)
    pub fn method_92(&mut self) {
    }

    /// Decompiled virtual method #93 @ RVA 0x0060ece0 (14 instructions, 4 basic blocks)
    pub fn method_93(&mut self) {
        // Writes to field at offset +0x0004
    }

    /// Decompiled virtual method #94 @ RVA 0x0060ed20 (73 instructions, 16 basic blocks)
    pub fn method_94(&mut self) {
    }

    /// Decompiled virtual method #95 @ RVA 0x0060ee30 (38 instructions, 6 basic blocks)
    pub fn method_95(&mut self) {
    }

    /// Decompiled virtual method #96 @ RVA 0x000d3090 (1 instructions, 1 basic blocks)
    pub fn method_96(&mut self) {
    }

    /// Decompiled virtual method #97 @ RVA 0x00103e80 (2 instructions, 1 basic blocks)
    pub fn method_97(&mut self) {
    }

    /// Decompiled virtual method #98 @ RVA 0x00103e80 (2 instructions, 1 basic blocks)
    pub fn method_98(&mut self) {
    }

    /// Decompiled virtual method #99 @ RVA 0x0060eec0 (25 instructions, 3 basic blocks)
    pub fn method_99(&mut self) {
    }

    /// Decompiled virtual method #100 @ RVA 0x0060ef20 (31 instructions, 3 basic blocks)
    pub fn method_100(&mut self) {
        // Writes to field at offset +0x0000
    }

    /// Decompiled virtual method #101 @ RVA 0x0060f070 (41 instructions, 10 basic blocks)
    pub fn method_101(&mut self) {
    }

    /// Decompiled virtual method #102 @ RVA 0x0060f110 (54 instructions, 27 basic blocks)
    pub fn method_102(&mut self) {
        // Writes to field at offset +0x0000
        // Writes to field at offset +0x0004
        // Writes to field at offset +0x0010
        // Writes to field at offset +0x0014
        // Writes to field at offset +0x0020
        // Writes to field at offset +0x0024
        // Writes to field at offset +0x0030
        // Writes to field at offset +0x0034
        // Writes to field at offset +0x0040
        // Writes to field at offset +0x0044
        // Writes to field at offset +0x0050
        // Writes to field at offset +0x0054
    }

    /// Decompiled virtual method #103 @ RVA 0x00584c10 (94 instructions, 1 basic blocks)
    pub fn method_103(&mut self) {
    }

    /// Decompiled virtual method #104 @ RVA 0x00584c60 (63 instructions, 1 basic blocks)
    pub fn method_104(&mut self) {
    }

    /// Decompiled virtual method #105 @ RVA 0x00584cb0 (35 instructions, 1 basic blocks)
    pub fn method_105(&mut self) {
    }

    /// Decompiled virtual method #106 @ RVA 0x00103e80 (2 instructions, 1 basic blocks)
    pub fn method_106(&mut self) {
    }

    /// Decompiled virtual method #107 @ RVA 0x0060efa0 (22 instructions, 2 basic blocks)
    pub fn method_107(&mut self) {
    }

    /// Decompiled virtual method #108 @ RVA 0x0060f000 (28 instructions, 3 basic blocks)
    pub fn method_108(&mut self) {
        // Writes to field at offset +0x0000
    }

    /// Decompiled virtual method #109 @ RVA 0x0060f1d0 (21 instructions, 7 basic blocks)
    pub fn method_109(&mut self) {
    }

    /// Decompiled virtual method #110 @ RVA 0x00584d00 (7 instructions, 1 basic blocks)
    pub fn method_110(&mut self) {
    }

    /// Decompiled virtual method #111 @ RVA 0x00584d20 (4 instructions, 1 basic blocks)
    pub fn method_111(&mut self) {
    }

    /// Decompiled virtual method #112 @ RVA 0x00584d30 (3 instructions, 1 basic blocks)
    pub fn method_112(&mut self) {
    }

    /// Decompiled virtual method #113 @ RVA 0x00584d40 (3 instructions, 1 basic blocks)
    pub fn method_113(&mut self) {
    }

    /// Decompiled virtual method #114 @ RVA 0x00584d50 (3 instructions, 1 basic blocks)
    pub fn method_114(&mut self) {
    }

    /// Decompiled virtual method #115 @ RVA 0x00584d60 (3 instructions, 1 basic blocks)
    pub fn method_115(&mut self) {
    }

    /// Decompiled virtual method #116 @ RVA 0x00584d70 (3 instructions, 1 basic blocks)
    pub fn method_116(&mut self) {
    }

    /// Decompiled virtual method #117 @ RVA 0x00584d80 (3 instructions, 1 basic blocks)
    pub fn method_117(&mut self) {
    }

    /// Decompiled virtual method #118 @ RVA 0x00615860 (6 instructions, 2 basic blocks)
    pub fn method_118(&mut self) {
    }

    /// Decompiled virtual method #119 @ RVA 0x00584d90 (4 instructions, 1 basic blocks)
    pub fn method_119(&mut self) {
    }

    /// Decompiled virtual method #120 @ RVA 0x00615890 (134 instructions, 5 basic blocks)
    pub fn method_120(&mut self) {
    }

    /// Decompiled virtual method #121 @ RVA 0x00615ab0 (4 instructions, 2 basic blocks)
    pub fn method_121(&mut self) {
    }

    /// Decompiled virtual method #122 @ RVA 0x00584da0 (4 instructions, 1 basic blocks)
    pub fn method_122(&mut self) {
    }

    /// Decompiled virtual method #123 @ RVA 0x00584db0 (2 instructions, 1 basic blocks)
    pub fn method_123(&mut self) {
    }

    /// Decompiled virtual method #124 @ RVA 0x00584dc0 (2 instructions, 1 basic blocks)
    pub fn method_124(&mut self) {
    }

    /// Decompiled virtual method #125 @ RVA 0x00610200 (6 instructions, 1 basic blocks)
    pub fn method_125(&mut self) {
    }

    /// Decompiled virtual method #126 @ RVA 0x00584dd0 (3 instructions, 1 basic blocks)
    pub fn method_126(&mut self) {
    }

    /// Decompiled virtual method #127 @ RVA 0x00584df0 (6 instructions, 1 basic blocks)
    pub fn method_127(&mut self) {
    }

    /// Decompiled virtual method #128 @ RVA 0x00584e10 (2 instructions, 1 basic blocks)
    pub fn method_128(&mut self) {
    }

    /// Decompiled virtual method #129 @ RVA 0x00584e20 (2 instructions, 1 basic blocks)
    pub fn method_129(&mut self) {
    }

    /// Decompiled virtual method #130 @ RVA 0x00584e30 (7 instructions, 1 basic blocks)
    pub fn method_130(&mut self) {
    }

    /// Decompiled virtual method #131 @ RVA 0x00584e50 (2 instructions, 1 basic blocks)
    pub fn method_131(&mut self) {
    }

    /// Decompiled virtual method #132 @ RVA 0x00584e60 (4 instructions, 1 basic blocks)
    pub fn method_132(&mut self) {
    }

    /// Decompiled virtual method #133 @ RVA 0x00584e70 (2 instructions, 1 basic blocks)
    pub fn method_133(&mut self) {
    }

    /// Decompiled virtual method #134 @ RVA 0x00584e80 (8 instructions, 1 basic blocks)
    pub fn method_134(&mut self) {
    }

    /// Decompiled virtual method #135 @ RVA 0x00584eb0 (2 instructions, 1 basic blocks)
    pub fn method_135(&mut self) {
    }

    /// Decompiled virtual method #136 @ RVA 0x00584ec0 (2 instructions, 1 basic blocks)
    pub fn method_136(&mut self) {
    }

    /// Decompiled virtual method #137 @ RVA 0x00584ed0 (2 instructions, 1 basic blocks)
    pub fn method_137(&mut self) {
    }

    /// Decompiled virtual method #138 @ RVA 0x00584ee0 (3 instructions, 1 basic blocks)
    pub fn method_138(&mut self) {
    }

    /// Decompiled virtual method #139 @ RVA 0x00584ef0 (2 instructions, 1 basic blocks)
    pub fn method_139(&mut self) {
    }

    /// Decompiled virtual method #140 @ RVA 0x00584f00 (2 instructions, 1 basic blocks)
    pub fn method_140(&mut self) {
    }

    /// Decompiled virtual method #141 @ RVA 0x00584f10 (2 instructions, 1 basic blocks)
    pub fn method_141(&mut self) {
    }

    /// Decompiled virtual method #142 @ RVA 0x00584f20 (3 instructions, 1 basic blocks)
    pub fn method_142(&mut self) {
    }

    /// Decompiled virtual method #143 @ RVA 0x00584f30 (2 instructions, 1 basic blocks)
    pub fn method_143(&mut self) {
    }

    /// Decompiled virtual method #144 @ RVA 0x00584f40 (2 instructions, 1 basic blocks)
    pub fn method_144(&mut self) {
    }

    /// Decompiled virtual method #145 @ RVA 0x00584f50 (2 instructions, 1 basic blocks)
    pub fn method_145(&mut self) {
    }

    /// Decompiled virtual method #146 @ RVA 0x00584f60 (3 instructions, 1 basic blocks)
    pub fn method_146(&mut self) {
    }

    /// Decompiled virtual method #147 @ RVA 0x00584f70 (2 instructions, 1 basic blocks)
    pub fn method_147(&mut self) {
    }

    /// Decompiled virtual method #148 @ RVA 0x00584f80 (2 instructions, 1 basic blocks)
    pub fn method_148(&mut self) {
    }

    /// Decompiled virtual method #149 @ RVA 0x00584f90 (2 instructions, 1 basic blocks)
    pub fn method_149(&mut self) {
    }

    /// Decompiled virtual method #150 @ RVA 0x00584fa0 (2 instructions, 1 basic blocks)
    pub fn method_150(&mut self) {
    }

    /// Decompiled virtual method #151 @ RVA 0x00584fb0 (2 instructions, 1 basic blocks)
    pub fn method_151(&mut self) {
    }

    /// Decompiled virtual method #152 @ RVA 0x00584fc0 (2 instructions, 1 basic blocks)
    pub fn method_152(&mut self) {
    }

    /// Decompiled virtual method #153 @ RVA 0x00584fd0 (2 instructions, 1 basic blocks)
    pub fn method_153(&mut self) {
    }

    /// Decompiled virtual method #154 @ RVA 0x00584fe0 (2 instructions, 1 basic blocks)
    pub fn method_154(&mut self) {
    }

    /// Decompiled virtual method #155 @ RVA 0x00584ff0 (53 instructions, 1 basic blocks)
    pub fn method_155(&mut self) {
    }

    /// Decompiled virtual method #156 @ RVA 0x0060f230 (27 instructions, 3 basic blocks)
    pub fn method_156(&mut self) {
    }

    /// Decompiled virtual method #157 @ RVA 0x0060f300 (23 instructions, 3 basic blocks)
    pub fn method_157(&mut self) {
    }

    /// Decompiled virtual method #158 @ RVA 0x0060f3a0 (48 instructions, 6 basic blocks)
    pub fn method_158(&mut self) {
    }

    /// Decompiled virtual method #159 @ RVA 0x0060f460 (20 instructions, 2 basic blocks)
    pub fn method_159(&mut self) {
    }

    /// Decompiled virtual method #160 @ RVA 0x0060f4b0 (91 instructions, 2 basic blocks)
    pub fn method_160(&mut self) {
    }

    /// Decompiled virtual method #161 @ RVA 0x0060f5f0 (17 instructions, 2 basic blocks)
    pub fn method_161(&mut self) {
    }

    /// Decompiled virtual method #162 @ RVA 0x0060f630 (20 instructions, 2 basic blocks)
    pub fn method_162(&mut self) {
    }

    /// Decompiled virtual method #163 @ RVA 0x0060f680 (25 instructions, 7 basic blocks)
    pub fn method_163(&mut self) {
    }

    /// Decompiled virtual method #164 @ RVA 0x0060f6e0 (23 instructions, 3 basic blocks)
    pub fn method_164(&mut self) {
    }

    /// Decompiled virtual method #165 @ RVA 0x0060f730 (29 instructions, 5 basic blocks)
    pub fn method_165(&mut self) {
    }

    /// Decompiled virtual method #166 @ RVA 0x0060f7a0 (2 instructions, 1 basic blocks)
    pub fn method_166(&mut self) {
    }

    /// Decompiled virtual method #167 @ RVA 0x0060f7b0 (7 instructions, 3 basic blocks)
    pub fn method_167(&mut self) {
    }

    /// Decompiled virtual method #168 @ RVA 0x0060f7d0 (18 instructions, 6 basic blocks)
    pub fn method_168(&mut self) {
    }

    /// Decompiled virtual method #169 @ RVA 0x0060f820 (28 instructions, 2 basic blocks)
    pub fn method_169(&mut self) {
    }

    /// Decompiled virtual method #170 @ RVA 0x0060f8a0 (11 instructions, 2 basic blocks)
    pub fn method_170(&mut self) {
    }

    /// Decompiled virtual method #171 @ RVA 0x0060f8d0 (6 instructions, 2 basic blocks)
    pub fn method_171(&mut self) {
    }

    /// Decompiled virtual method #172 @ RVA 0x0060f8f0 (6 instructions, 2 basic blocks)
    pub fn method_172(&mut self) {
    }

    /// Decompiled virtual method #173 @ RVA 0x0060f910 (5 instructions, 2 basic blocks)
    pub fn method_173(&mut self) {
    }

    /// Decompiled virtual method #174 @ RVA 0x0060f930 (5 instructions, 3 basic blocks)
    pub fn method_174(&mut self) {
    }

    /// Decompiled virtual method #175 @ RVA 0x0060f950 (11 instructions, 4 basic blocks)
    pub fn method_175(&mut self) {
    }

    /// Decompiled virtual method #176 @ RVA 0x0060f980 (12 instructions, 4 basic blocks)
    pub fn method_176(&mut self) {
    }

    /// Decompiled virtual method #177 @ RVA 0x0060f9b0 (9 instructions, 3 basic blocks)
    pub fn method_177(&mut self) {
    }

    /// Decompiled virtual method #178 @ RVA 0x0060f9d0 (7 instructions, 2 basic blocks)
    pub fn method_178(&mut self) {
    }

    /// Decompiled virtual method #179 @ RVA 0x000d6920 (2 instructions, 1 basic blocks)
    pub fn method_179(&mut self) {
    }

    /// Decompiled virtual method #180 @ RVA 0x0060f9f0 (34 instructions, 8 basic blocks)
    pub fn method_180(&mut self) {
    }

    /// Decompiled virtual method #181 @ RVA 0x0060fae0 (8 instructions, 2 basic blocks)
    pub fn method_181(&mut self) {
    }

    /// Decompiled virtual method #182 @ RVA 0x0060fb30 (7 instructions, 2 basic blocks)
    pub fn method_182(&mut self) {
    }

    /// Decompiled virtual method #183 @ RVA 0x0060fb50 (6 instructions, 2 basic blocks)
    pub fn method_183(&mut self) {
    }

    /// Decompiled virtual method #184 @ RVA 0x0060fb90 (6 instructions, 2 basic blocks)
    pub fn method_184(&mut self) {
    }

    /// Decompiled virtual method #185 @ RVA 0x0060fbd0 (7 instructions, 2 basic blocks)
    pub fn method_185(&mut self) {
    }

    /// Decompiled virtual method #186 @ RVA 0x0060fbf0 (6 instructions, 2 basic blocks)
    pub fn method_186(&mut self) {
    }

    /// Decompiled virtual method #187 @ RVA 0x0060fc30 (6 instructions, 2 basic blocks)
    pub fn method_187(&mut self) {
    }

    /// Decompiled virtual method #188 @ RVA 0x0060fc70 (14 instructions, 7 basic blocks)
    pub fn method_188(&mut self) {
    }

    /// Decompiled virtual method #189 @ RVA 0x0060fca0 (16 instructions, 3 basic blocks)
    pub fn method_189(&mut self) {
    }

    /// Decompiled virtual method #190 @ RVA 0x0060fce0 (16 instructions, 3 basic blocks)
    pub fn method_190(&mut self) {
    }

    /// Decompiled virtual method #191 @ RVA 0x0060fd20 (4 instructions, 1 basic blocks)
    pub fn method_191(&mut self) {
    }

    /// Decompiled virtual method #192 @ RVA 0x0060fd40 (28 instructions, 4 basic blocks)
    pub fn method_192(&mut self) {
    }

    /// Decompiled virtual method #193 @ RVA 0x0060fdb0 (23 instructions, 4 basic blocks)
    pub fn method_193(&mut self) {
    }

    /// Decompiled virtual method #194 @ RVA 0x0060fe10 (34 instructions, 4 basic blocks)
    pub fn method_194(&mut self) {
    }

    /// Decompiled virtual method #195 @ RVA 0x0060ff10 (44 instructions, 6 basic blocks)
    pub fn method_195(&mut self) {
    }

    /// Decompiled virtual method #196 @ RVA 0x0060ffc0 (23 instructions, 3 basic blocks)
    pub fn method_196(&mut self) {
    }

    /// Decompiled virtual method #197 @ RVA 0x00610020 (26 instructions, 5 basic blocks)
    pub fn method_197(&mut self) {
    }

    /// Decompiled virtual method #198 @ RVA 0x00610080 (35 instructions, 6 basic blocks)
    pub fn method_198(&mut self) {
    }

    /// Decompiled virtual method #199 @ RVA 0x00610110 (35 instructions, 6 basic blocks)
    pub fn method_199(&mut self) {
    }

    /// Decompiled virtual method #200 @ RVA 0x006101a0 (5 instructions, 2 basic blocks)
    pub fn method_200(&mut self) {
    }

    /// Decompiled virtual method #201 @ RVA 0x006101c0 (12 instructions, 4 basic blocks)
    pub fn method_201(&mut self) {
    }

    /// Decompiled virtual method #202 @ RVA 0x00610230 (25 instructions, 7 basic blocks)
    pub fn method_202(&mut self) {
    }

    /// Decompiled virtual method #203 @ RVA 0x00610310 (82 instructions, 16 basic blocks)
    pub fn method_203(&mut self) {
    }

    /// Decompiled virtual method #204 @ RVA 0x00610490 (27 instructions, 7 basic blocks)
    pub fn method_204(&mut self) {
    }

    /// Decompiled virtual method #205 @ RVA 0x00610580 (25 instructions, 7 basic blocks)
    pub fn method_205(&mut self) {
    }

    /// Decompiled virtual method #206 @ RVA 0x00610660 (27 instructions, 7 basic blocks)
    pub fn method_206(&mut self) {
    }

    /// Decompiled virtual method #207 @ RVA 0x00610750 (27 instructions, 7 basic blocks)
    pub fn method_207(&mut self) {
    }

    /// Decompiled virtual method #208 @ RVA 0x00610840 (36 instructions, 8 basic blocks)
    pub fn method_208(&mut self) {
    }

    /// Decompiled virtual method #209 @ RVA 0x00610990 (36 instructions, 8 basic blocks)
    pub fn method_209(&mut self) {
    }

    /// Decompiled virtual method #210 @ RVA 0x00610ae0 (10 instructions, 3 basic blocks)
    pub fn method_210(&mut self) {
    }

    /// Decompiled virtual method #211 @ RVA 0x00610b50 (20 instructions, 8 basic blocks)
    pub fn method_211(&mut self) {
        // Writes to field at offset +0x0000
        // Writes to field at offset +0x0008
        // Writes to field at offset +0x0010
        // Writes to field at offset +0x0018
    }

    /// Decompiled virtual method #212 @ RVA 0x00610bb0 (106 instructions, 10 basic blocks)
    pub fn method_212(&mut self) {
    }

    /// Decompiled virtual method #213 @ RVA 0x00610d70 (35 instructions, 7 basic blocks)
    pub fn method_213(&mut self) {
    }

    /// Decompiled virtual method #214 @ RVA 0x00610df0 (22 instructions, 4 basic blocks)
    pub fn method_214(&mut self) {
    }

    /// Decompiled virtual method #215 @ RVA 0x00610e50 (48 instructions, 8 basic blocks)
    pub fn method_215(&mut self) {
        // Writes to field at offset +0x0000
    }

    /// Decompiled virtual method #216 @ RVA 0x00610f30 (39 instructions, 7 basic blocks)
    pub fn method_216(&mut self) {
    }

    /// Decompiled virtual method #217 @ RVA 0x00610fc0 (12 instructions, 2 basic blocks)
    pub fn method_217(&mut self) {
    }

    /// Decompiled virtual method #218 @ RVA 0x00611030 (12 instructions, 2 basic blocks)
    pub fn method_218(&mut self) {
    }

    /// Decompiled virtual method #219 @ RVA 0x006110a0 (31 instructions, 6 basic blocks)
    pub fn method_219(&mut self) {
    }

    /// Decompiled virtual method #220 @ RVA 0x000d3090 (1 instructions, 1 basic blocks)
    pub fn method_220(&mut self) {
    }

    /// Decompiled virtual method #221 @ RVA 0x00611110 (20 instructions, 8 basic blocks)
    pub fn method_221(&mut self) {
        // Writes to field at offset +0x0000
        // Writes to field at offset +0x0008
        // Writes to field at offset +0x0010
        // Writes to field at offset +0x0018
    }

    /// Decompiled virtual method #222 @ RVA 0x00611170 (12 instructions, 2 basic blocks)
    pub fn method_222(&mut self) {
    }

    /// Decompiled virtual method #223 @ RVA 0x00611240 (57 instructions, 14 basic blocks)
    pub fn method_223(&mut self) {
    }

    /// Decompiled virtual method #224 @ RVA 0x006111d0 (26 instructions, 1 basic blocks)
    pub fn method_224(&mut self) {
    }

    /// Decompiled virtual method #225 @ RVA 0x000d3090 (1 instructions, 1 basic blocks)
    pub fn method_225(&mut self) {
    }

    /// Decompiled virtual method #226 @ RVA 0x00585090 (2 instructions, 1 basic blocks)
    pub fn method_226(&mut self) {
    }

    /// Decompiled virtual method #227 @ RVA 0x00611340 (6 instructions, 2 basic blocks)
    pub fn method_227(&mut self) {
    }

    /// Decompiled virtual method #228 @ RVA 0x00611370 (7 instructions, 2 basic blocks)
    pub fn method_228(&mut self) {
    }

    /// Decompiled virtual method #229 @ RVA 0x006113a0 (5 instructions, 2 basic blocks)
    pub fn method_229(&mut self) {
    }

    /// Decompiled virtual method #230 @ RVA 0x006113c0 (7 instructions, 2 basic blocks)
    pub fn method_230(&mut self) {
    }

    /// Decompiled virtual method #231 @ RVA 0x005850a0 (2 instructions, 1 basic blocks)
    pub fn method_231(&mut self) {
    }

    /// Decompiled virtual method #232 @ RVA 0x00611400 (19 instructions, 3 basic blocks)
    pub fn method_232(&mut self) {
    }

    /// Decompiled virtual method #233 @ RVA 0x00611450 (8 instructions, 3 basic blocks)
    pub fn method_233(&mut self) {
    }

    /// Decompiled virtual method #234 @ RVA 0x006114a0 (2 instructions, 1 basic blocks)
    pub fn method_234(&mut self) {
    }

    /// Decompiled virtual method #235 @ RVA 0x006114b0 (12 instructions, 6 basic blocks)
    pub fn method_235(&mut self) {
    }

    /// Decompiled virtual method #236 @ RVA 0x006114e0 (6 instructions, 1 basic blocks)
    pub fn method_236(&mut self) {
    }

    /// Decompiled virtual method #237 @ RVA 0x00611500 (2 instructions, 1 basic blocks)
    pub fn method_237(&mut self) {
    }

    /// Decompiled virtual method #238 @ RVA 0x00611510 (10 instructions, 2 basic blocks)
    pub fn method_238(&mut self) {
    }

    /// Decompiled virtual method #239 @ RVA 0x00611520 (3 instructions, 1 basic blocks)
    pub fn method_239(&mut self) {
    }

    /// Decompiled virtual method #240 @ RVA 0x00611530 (3 instructions, 1 basic blocks)
    pub fn method_240(&mut self) {
    }

    /// Decompiled virtual method #241 @ RVA 0x00611540 (9 instructions, 4 basic blocks)
    pub fn method_241(&mut self) {
    }

    /// Decompiled virtual method #242 @ RVA 0x00611570 (9 instructions, 4 basic blocks)
    pub fn method_242(&mut self) {
    }

    /// Decompiled virtual method #243 @ RVA 0x006115a0 (5 instructions, 2 basic blocks)
    pub fn method_243(&mut self) {
    }

    /// Decompiled virtual method #244 @ RVA 0x006115c0 (18 instructions, 6 basic blocks)
    pub fn method_244(&mut self) {
    }

    /// Decompiled virtual method #245 @ RVA 0x00611610 (16 instructions, 6 basic blocks)
    pub fn method_245(&mut self) {
    }

    /// Decompiled virtual method #246 @ RVA 0x00611650 (8 instructions, 3 basic blocks)
    pub fn method_246(&mut self) {
    }

    /// Decompiled virtual method #247 @ RVA 0x00611690 (5 instructions, 3 basic blocks)
    pub fn method_247(&mut self) {
    }

    /// Decompiled virtual method #248 @ RVA 0x006116b0 (5 instructions, 2 basic blocks)
    pub fn method_248(&mut self) {
    }

    /// Decompiled virtual method #249 @ RVA 0x006116d0 (26 instructions, 4 basic blocks)
    pub fn method_249(&mut self) {
    }

    /// Decompiled virtual method #250 @ RVA 0x00611740 (6 instructions, 3 basic blocks)
    pub fn method_250(&mut self) {
    }

    /// Decompiled virtual method #251 @ RVA 0x00611760 (9 instructions, 4 basic blocks)
    pub fn method_251(&mut self) {
    }

    /// Decompiled virtual method #252 @ RVA 0x00611790 (8 instructions, 4 basic blocks)
    pub fn method_252(&mut self) {
    }

    /// Decompiled virtual method #253 @ RVA 0x006117b0 (8 instructions, 3 basic blocks)
    pub fn method_253(&mut self) {
    }

    /// Decompiled virtual method #254 @ RVA 0x006117f0 (5 instructions, 2 basic blocks)
    pub fn method_254(&mut self) {
    }

    /// Decompiled virtual method #255 @ RVA 0x00611810 (6 instructions, 3 basic blocks)
    pub fn method_255(&mut self) {
    }

    /// Decompiled virtual method #256 @ RVA 0x00611830 (6 instructions, 3 basic blocks)
    pub fn method_256(&mut self) {
    }

    /// Decompiled virtual method #257 @ RVA 0x005850b0 (2 instructions, 1 basic blocks)
    pub fn method_257(&mut self) {
    }

    /// Decompiled virtual method #258 @ RVA 0x005850c0 (2 instructions, 1 basic blocks)
    pub fn method_258(&mut self) {
    }

    /// Decompiled virtual method #259 @ RVA 0x005850d0 (2 instructions, 1 basic blocks)
    pub fn method_259(&mut self) {
    }

    /// Decompiled virtual method #260 @ RVA 0x00611850 (8 instructions, 2 basic blocks)
    pub fn method_260(&mut self) {
    }

    /// Decompiled virtual method #261 @ RVA 0x00611870 (25 instructions, 3 basic blocks)
    pub fn method_261(&mut self) {
    }

    /// Decompiled virtual method #262 @ RVA 0x006118e0 (6 instructions, 2 basic blocks)
    pub fn method_262(&mut self) {
    }

    /// Decompiled virtual method #263 @ RVA 0x00611900 (6 instructions, 2 basic blocks)
    pub fn method_263(&mut self) {
    }

    /// Decompiled virtual method #264 @ RVA 0x00611920 (6 instructions, 2 basic blocks)
    pub fn method_264(&mut self) {
    }

    /// Decompiled virtual method #265 @ RVA 0x005850e0 (2 instructions, 1 basic blocks)
    pub fn method_265(&mut self) {
    }

    /// Decompiled virtual method #266 @ RVA 0x005850f0 (2 instructions, 1 basic blocks)
    pub fn method_266(&mut self) {
    }

    /// Decompiled virtual method #267 @ RVA 0x00611940 (8 instructions, 2 basic blocks)
    pub fn method_267(&mut self) {
    }

    /// Decompiled virtual method #268 @ RVA 0x00611960 (25 instructions, 3 basic blocks)
    pub fn method_268(&mut self) {
    }

    /// Decompiled virtual method #269 @ RVA 0x006119d0 (6 instructions, 3 basic blocks)
    pub fn method_269(&mut self) {
    }

    /// Decompiled virtual method #270 @ RVA 0x006119f0 (6 instructions, 2 basic blocks)
    pub fn method_270(&mut self) {
    }

    /// Decompiled virtual method #271 @ RVA 0x00611a10 (6 instructions, 2 basic blocks)
    pub fn method_271(&mut self) {
    }

    /// Decompiled virtual method #272 @ RVA 0x00611a30 (43 instructions, 4 basic blocks)
    pub fn method_272(&mut self) {
    }

    /// Decompiled virtual method #273 @ RVA 0x00611a70 (20 instructions, 4 basic blocks)
    pub fn method_273(&mut self) {
    }

    /// Decompiled virtual method #274 @ RVA 0x00611ac0 (4 instructions, 1 basic blocks)
    pub fn method_274(&mut self) {
    }

    /// Decompiled virtual method #275 @ RVA 0x00611ad0 (4 instructions, 1 basic blocks)
    pub fn method_275(&mut self) {
    }

    /// Decompiled virtual method #276 @ RVA 0x00611af0 (4 instructions, 1 basic blocks)
    pub fn method_276(&mut self) {
    }

    /// Decompiled virtual method #277 @ RVA 0x00611b10 (4 instructions, 1 basic blocks)
    pub fn method_277(&mut self) {
    }

    /// Decompiled virtual method #278 @ RVA 0x00611b30 (4 instructions, 1 basic blocks)
    pub fn method_278(&mut self) {
    }

    /// Decompiled virtual method #279 @ RVA 0x00611b50 (3 instructions, 1 basic blocks)
    pub fn method_279(&mut self) {
    }

    /// Decompiled virtual method #280 @ RVA 0x00611b60 (5 instructions, 1 basic blocks)
    pub fn method_280(&mut self) {
    }

    /// Decompiled virtual method #281 @ RVA 0x00611b80 (5 instructions, 1 basic blocks)
    pub fn method_281(&mut self) {
    }

    /// Decompiled virtual method #282 @ RVA 0x00611ba0 (5 instructions, 1 basic blocks)
    pub fn method_282(&mut self) {
    }

    /// Decompiled virtual method #283 @ RVA 0x00611bc0 (5 instructions, 1 basic blocks)
    pub fn method_283(&mut self) {
    }

    /// Decompiled virtual method #284 @ RVA 0x00611be0 (19 instructions, 2 basic blocks)
    pub fn method_284(&mut self) {
    }

    /// Decompiled virtual method #285 @ RVA 0x00611c30 (35 instructions, 5 basic blocks)
    pub fn method_285(&mut self) {
    }

    /// Decompiled virtual method #286 @ RVA 0x00611cb0 (17 instructions, 2 basic blocks)
    pub fn method_286(&mut self) {
    }

    /// Decompiled virtual method #287 @ RVA 0x00611d00 (17 instructions, 2 basic blocks)
    pub fn method_287(&mut self) {
    }

    /// Decompiled virtual method #288 @ RVA 0x00611d50 (17 instructions, 2 basic blocks)
    pub fn method_288(&mut self) {
    }

    /// Decompiled virtual method #289 @ RVA 0x00103e70 (2 instructions, 1 basic blocks)
    pub fn method_289(&mut self) {
    }

    /// Decompiled virtual method #290 @ RVA 0x00585100 (2 instructions, 1 basic blocks)
    pub fn method_290(&mut self) {
    }

    /// Decompiled virtual method #291 @ RVA 0x00585110 (2 instructions, 1 basic blocks)
    pub fn method_291(&mut self) {
    }

    /// Decompiled virtual method #292 @ RVA 0x00611da0 (6 instructions, 2 basic blocks)
    pub fn method_292(&mut self) {
    }

    /// Decompiled virtual method #293 @ RVA 0x00611dc0 (9 instructions, 2 basic blocks)
    pub fn method_293(&mut self) {
    }

    /// Decompiled virtual method #294 @ RVA 0x00611df0 (6 instructions, 2 basic blocks)
    pub fn method_294(&mut self) {
    }

    /// Decompiled virtual method #295 @ RVA 0x00611e10 (6 instructions, 2 basic blocks)
    pub fn method_295(&mut self) {
    }

    /// Decompiled virtual method #296 @ RVA 0x00611e30 (6 instructions, 2 basic blocks)
    pub fn method_296(&mut self) {
    }

    /// Decompiled virtual method #297 @ RVA 0x00611e50 (40 instructions, 7 basic blocks)
    pub fn method_297(&mut self) {
    }

    /// Decompiled virtual method #298 @ RVA 0x00611ef0 (10 instructions, 4 basic blocks)
    pub fn method_298(&mut self) {
    }

    /// Decompiled virtual method #299 @ RVA 0x00611f10 (6 instructions, 2 basic blocks)
    pub fn method_299(&mut self) {
    }

    /// Decompiled virtual method #300 @ RVA 0x00611f30 (2 instructions, 1 basic blocks)
    pub fn method_300(&mut self) {
    }

    /// Decompiled virtual method #301 @ RVA 0x00611f40 (9 instructions, 4 basic blocks)
    pub fn method_301(&mut self) {
    }

    /// Decompiled virtual method #302 @ RVA 0x00611f70 (19 instructions, 6 basic blocks)
    pub fn method_302(&mut self) {
    }

    /// Decompiled virtual method #303 @ RVA 0x00611fc0 (15 instructions, 4 basic blocks)
    pub fn method_303(&mut self) {
    }

    /// Decompiled virtual method #304 @ RVA 0x00612000 (15 instructions, 4 basic blocks)
    pub fn method_304(&mut self) {
    }

    /// Decompiled virtual method #305 @ RVA 0x00612040 (14 instructions, 4 basic blocks)
    pub fn method_305(&mut self) {
    }

    /// Decompiled virtual method #306 @ RVA 0x00612070 (10 instructions, 1 basic blocks)
    pub fn method_306(&mut self) {
    }

    /// Decompiled virtual method #307 @ RVA 0x006120a0 (10 instructions, 1 basic blocks)
    pub fn method_307(&mut self) {
    }

    /// Decompiled virtual method #308 @ RVA 0x006120d0 (7 instructions, 1 basic blocks)
    pub fn method_308(&mut self) {
    }

    /// Decompiled virtual method #309 @ RVA 0x006120f0 (31 instructions, 2 basic blocks)
    pub fn method_309(&mut self) {
    }

    /// Decompiled virtual method #310 @ RVA 0x00612140 (5 instructions, 2 basic blocks)
    pub fn method_310(&mut self) {
    }

    /// Decompiled virtual method #311 @ RVA 0x00612160 (4 instructions, 1 basic blocks)
    pub fn method_311(&mut self) {
    }

    /// Decompiled virtual method #312 @ RVA 0x00612170 (4 instructions, 1 basic blocks)
    pub fn method_312(&mut self) {
    }

    /// Decompiled virtual method #313 @ RVA 0x00612180 (5 instructions, 1 basic blocks)
    pub fn method_313(&mut self) {
        // Writes to field at offset +0x0000
    }

    /// Decompiled virtual method #314 @ RVA 0x006121a0 (5 instructions, 1 basic blocks)
    pub fn method_314(&mut self) {
    }

    /// Decompiled virtual method #315 @ RVA 0x006121c0 (2 instructions, 1 basic blocks)
    pub fn method_315(&mut self) {
    }

    /// Decompiled virtual method #316 @ RVA 0x006121d0 (3 instructions, 1 basic blocks)
    pub fn method_316(&mut self) {
    }

    /// Decompiled virtual method #317 @ RVA 0x006121e0 (3 instructions, 1 basic blocks)
    pub fn method_317(&mut self) {
    }

    /// Decompiled virtual method #318 @ RVA 0x006121f0 (2 instructions, 1 basic blocks)
    pub fn method_318(&mut self) {
    }

    /// Decompiled virtual method #319 @ RVA 0x00612200 (3 instructions, 1 basic blocks)
    pub fn method_319(&mut self) {
    }

    /// Decompiled virtual method #320 @ RVA 0x00612210 (3 instructions, 1 basic blocks)
    pub fn method_320(&mut self) {
    }

    /// Decompiled virtual method #321 @ RVA 0x00612220 (10 instructions, 3 basic blocks)
    pub fn method_321(&mut self) {
    }

    /// Decompiled virtual method #322 @ RVA 0x00612250 (13 instructions, 4 basic blocks)
    pub fn method_322(&mut self) {
        // Writes to field at offset +0x0000
    }

    /// Decompiled virtual method #323 @ RVA 0x00612280 (16 instructions, 4 basic blocks)
    pub fn method_323(&mut self) {
        // Writes to field at offset +0x0000
    }

    /// Decompiled virtual method #324 @ RVA 0x006122c0 (15 instructions, 4 basic blocks)
    pub fn method_324(&mut self) {
        // Writes to field at offset +0x0000
    }

    /// Decompiled virtual method #325 @ RVA 0x00585120 (2 instructions, 1 basic blocks)
    pub fn method_325(&mut self) {
    }

    /// Decompiled virtual method #326 @ RVA 0x00585130 (2 instructions, 1 basic blocks)
    pub fn method_326(&mut self) {
    }

    /// Decompiled virtual method #327 @ RVA 0x00612300 (2 instructions, 1 basic blocks)
    pub fn method_327(&mut self) {
    }

    /// Decompiled virtual method #328 @ RVA 0x00612310 (2 instructions, 1 basic blocks)
    pub fn method_328(&mut self) {
    }

    /// Decompiled virtual method #329 @ RVA 0x00612320 (2 instructions, 1 basic blocks)
    pub fn method_329(&mut self) {
    }

    /// Decompiled virtual method #330 @ RVA 0x00612330 (2 instructions, 1 basic blocks)
    pub fn method_330(&mut self) {
    }

    /// Decompiled virtual method #331 @ RVA 0x00612340 (2 instructions, 1 basic blocks)
    pub fn method_331(&mut self) {
    }

    /// Decompiled virtual method #332 @ RVA 0x00612350 (2 instructions, 1 basic blocks)
    pub fn method_332(&mut self) {
    }

    /// Decompiled virtual method #333 @ RVA 0x00612360 (2 instructions, 1 basic blocks)
    pub fn method_333(&mut self) {
    }

    /// Decompiled virtual method #334 @ RVA 0x00612370 (3 instructions, 1 basic blocks)
    pub fn method_334(&mut self) {
    }

    /// Decompiled virtual method #335 @ RVA 0x00612380 (3 instructions, 1 basic blocks)
    pub fn method_335(&mut self) {
    }

    /// Decompiled virtual method #336 @ RVA 0x006124a0 (5 instructions, 3 basic blocks)
    pub fn method_336(&mut self) {
    }

    /// Decompiled virtual method #337 @ RVA 0x006124c0 (5 instructions, 3 basic blocks)
    pub fn method_337(&mut self) {
    }

    /// Decompiled virtual method #338 @ RVA 0x006124e0 (5 instructions, 3 basic blocks)
    pub fn method_338(&mut self) {
    }

    /// Decompiled virtual method #339 @ RVA 0x00612500 (5 instructions, 3 basic blocks)
    pub fn method_339(&mut self) {
    }

    /// Decompiled virtual method #340 @ RVA 0x00612410 (40 instructions, 19 basic blocks)
    pub fn method_340(&mut self) {
    }

    /// Decompiled virtual method #341 @ RVA 0x00612520 (25 instructions, 3 basic blocks)
    pub fn method_341(&mut self) {
    }

    /// Decompiled virtual method #342 @ RVA 0x00612580 (8 instructions, 2 basic blocks)
    pub fn method_342(&mut self) {
    }

    /// Decompiled virtual method #343 @ RVA 0x006125a0 (6 instructions, 2 basic blocks)
    pub fn method_343(&mut self) {
    }

    /// Decompiled virtual method #344 @ RVA 0x006125c0 (7 instructions, 2 basic blocks)
    pub fn method_344(&mut self) {
    }

    /// Decompiled virtual method #345 @ RVA 0x00612390 (3 instructions, 1 basic blocks)
    pub fn method_345(&mut self) {
    }

    /// Decompiled virtual method #346 @ RVA 0x006123a0 (3 instructions, 1 basic blocks)
    pub fn method_346(&mut self) {
    }

    /// Decompiled virtual method #347 @ RVA 0x006123b0 (3 instructions, 1 basic blocks)
    pub fn method_347(&mut self) {
    }

    /// Decompiled virtual method #348 @ RVA 0x006123c0 (3 instructions, 1 basic blocks)
    pub fn method_348(&mut self) {
    }

    /// Decompiled virtual method #349 @ RVA 0x006123d0 (3 instructions, 1 basic blocks)
    pub fn method_349(&mut self) {
    }

    /// Decompiled virtual method #350 @ RVA 0x006123e0 (2 instructions, 1 basic blocks)
    pub fn method_350(&mut self) {
    }

    /// Decompiled virtual method #351 @ RVA 0x006123f0 (3 instructions, 1 basic blocks)
    pub fn method_351(&mut self) {
    }

    /// Decompiled virtual method #352 @ RVA 0x00612400 (3 instructions, 1 basic blocks)
    pub fn method_352(&mut self) {
    }

    /// Decompiled virtual method #353 @ RVA 0x000d3090 (1 instructions, 1 basic blocks)
    pub fn method_353(&mut self) {
    }

    /// Decompiled virtual method #354 @ RVA 0x000d3090 (1 instructions, 1 basic blocks)
    pub fn method_354(&mut self) {
    }

    /// Decompiled virtual method #355 @ RVA 0x000d6920 (2 instructions, 1 basic blocks)
    pub fn method_355(&mut self) {
    }

    /// Decompiled virtual method #356 @ RVA 0x006125e0 (10 instructions, 1 basic blocks)
    pub fn method_356(&mut self) {
    }

    /// Decompiled virtual method #357 @ RVA 0x00612610 (10 instructions, 1 basic blocks)
    pub fn method_357(&mut self) {
    }

    /// Decompiled virtual method #358 @ RVA 0x00612640 (10 instructions, 1 basic blocks)
    pub fn method_358(&mut self) {
    }

    /// Decompiled virtual method #359 @ RVA 0x00612670 (10 instructions, 1 basic blocks)
    pub fn method_359(&mut self) {
    }

    /// Decompiled virtual method #360 @ RVA 0x006126a0 (10 instructions, 1 basic blocks)
    pub fn method_360(&mut self) {
    }

    /// Decompiled virtual method #361 @ RVA 0x006126d0 (10 instructions, 1 basic blocks)
    pub fn method_361(&mut self) {
    }

    /// Decompiled virtual method #362 @ RVA 0x000d3090 (1 instructions, 1 basic blocks)
    pub fn method_362(&mut self) {
    }

    /// Decompiled virtual method #363 @ RVA 0x000d3090 (1 instructions, 1 basic blocks)
    pub fn method_363(&mut self) {
    }

    /// Decompiled virtual method #364 @ RVA 0x000d6920 (2 instructions, 1 basic blocks)
    pub fn method_364(&mut self) {
    }

    /// Decompiled virtual method #365 @ RVA 0x00585140 (2 instructions, 1 basic blocks)
    pub fn method_365(&mut self) {
    }

    /// Decompiled virtual method #366 @ RVA 0x00585150 (2 instructions, 1 basic blocks)
    pub fn method_366(&mut self) {
    }

    /// Decompiled virtual method #367 @ RVA 0x00585160 (2 instructions, 1 basic blocks)
    pub fn method_367(&mut self) {
    }

    /// Decompiled virtual method #368 @ RVA 0x00585170 (2 instructions, 1 basic blocks)
    pub fn method_368(&mut self) {
    }

    /// Decompiled virtual method #369 @ RVA 0x006127a0 (31 instructions, 8 basic blocks)
    pub fn method_369(&mut self) {
    }

    /// Decompiled virtual method #370 @ RVA 0x00612810 (26 instructions, 6 basic blocks)
    pub fn method_370(&mut self) {
    }

    /// Decompiled virtual method #371 @ RVA 0x00612c50 (108 instructions, 25 basic blocks)
    pub fn method_371(&mut self) {
    }

    /// Decompiled virtual method #372 @ RVA 0x00612a40 (85 instructions, 15 basic blocks)
    pub fn method_372(&mut self) {
    }

    /// Decompiled virtual method #373 @ RVA 0x006134b0 (139 instructions, 40 basic blocks)
    pub fn method_373(&mut self) {
    }

    /// Decompiled virtual method #374 @ RVA 0x00613a70 (122 instructions, 35 basic blocks)
    pub fn method_374(&mut self) {
        // Writes to field at offset +0x0000
        // Writes to field at offset +0x0002
        // Writes to field at offset +0x0004
        // Writes to field at offset +0x0006
        // Writes to field at offset +0x000a
        // Writes to field at offset +0x000e
        // Writes to field at offset +0x0010
        // Writes to field at offset +0x0012
        // Writes to field at offset +0x0014
        // Writes to field at offset +0x0016
    }

    /// Decompiled virtual method #375 @ RVA 0x006137a0 (131 instructions, 26 basic blocks)
    pub fn method_375(&mut self) {
        // Writes to field at offset +0x0000
        // Writes to field at offset +0x0000
        // Writes to field at offset +0x0004
        // Writes to field at offset +0x0006
        // Writes to field at offset +0x0008
        // Writes to field at offset +0x000a
        // Writes to field at offset +0x000c
    }

    /// Decompiled virtual method #376 @ RVA 0x00612880 (124 instructions, 33 basic blocks)
    pub fn method_376(&mut self) {
    }

    /// Decompiled virtual method #377 @ RVA 0x00612dd0 (144 instructions, 33 basic blocks)
    pub fn method_377(&mut self) {
        // Writes to field at offset +0x0000
    }

    /// Decompiled virtual method #378 @ RVA 0x006131c0 (57 instructions, 14 basic blocks)
    pub fn method_378(&mut self) {
    }

    /// Decompiled virtual method #379 @ RVA 0x00613290 (123 instructions, 41 basic blocks)
    pub fn method_379(&mut self) {
    }

    /// Decompiled virtual method #380 @ RVA 0x00613450 (27 instructions, 8 basic blocks)
    pub fn method_380(&mut self) {
    }

    /// Decompiled virtual method #381 @ RVA 0x00613c20 (76 instructions, 10 basic blocks)
    pub fn method_381(&mut self) {
    }

    /// Decompiled virtual method #382 @ RVA 0x00613d40 (6 instructions, 3 basic blocks)
    pub fn method_382(&mut self) {
    }

    /// Decompiled virtual method #383 @ RVA 0x00613d60 (31 instructions, 11 basic blocks)
    pub fn method_383(&mut self) {
        // Writes to field at offset +0x0000
        // Writes to field at offset +0x0004
        // Writes to field at offset +0x0006
        // Writes to field at offset +0x0008
        // Writes to field at offset +0x000a
        // Writes to field at offset +0x000c
    }

    /// Decompiled virtual method #384 @ RVA 0x00613dd0 (28 instructions, 3 basic blocks)
    pub fn method_384(&mut self) {
    }

    /// Decompiled virtual method #385 @ RVA 0x00613e30 (35 instructions, 5 basic blocks)
    pub fn method_385(&mut self) {
    }

    /// Decompiled virtual method #386 @ RVA 0x00613ec0 (29 instructions, 3 basic blocks)
    pub fn method_386(&mut self) {
    }

    /// Decompiled virtual method #387 @ RVA 0x00103e80 (2 instructions, 1 basic blocks)
    pub fn method_387(&mut self) {
    }

    /// Decompiled virtual method #388 @ RVA 0x00613f20 (15 instructions, 4 basic blocks)
    pub fn method_388(&mut self) {
    }

    /// Decompiled virtual method #389 @ RVA 0x00585180 (2 instructions, 1 basic blocks)
    pub fn method_389(&mut self) {
    }

    /// Decompiled virtual method #390 @ RVA 0x00103e80 (2 instructions, 1 basic blocks)
    pub fn method_390(&mut self) {
    }

    /// Decompiled virtual method #391 @ RVA 0x000d3090 (1 instructions, 1 basic blocks)
    pub fn method_391(&mut self) {
    }

    /// Decompiled virtual method #392 @ RVA 0x00103e70 (2 instructions, 1 basic blocks)
    pub fn method_392(&mut self) {
    }

    /// Decompiled virtual method #393 @ RVA 0x00103e70 (2 instructions, 1 basic blocks)
    pub fn method_393(&mut self) {
    }

    /// Decompiled virtual method #394 @ RVA 0x00613fb0 (124 instructions, 17 basic blocks)
    pub fn method_394(&mut self) {
        // Writes to field at offset +0x0000
    }

    /// Decompiled virtual method #395 @ RVA 0x00614190 (126 instructions, 28 basic blocks)
    pub fn method_395(&mut self) {
        // Writes to field at offset +0x0000
        // Writes to field at offset +0x0000
        // Writes to field at offset +0x0010
        // Writes to field at offset +0x0018
    }

    /// Decompiled virtual method #396 @ RVA 0x00614860 (120 instructions, 25 basic blocks)
    pub fn method_396(&mut self) {
        // Writes to field at offset +0x0000
        // Writes to field at offset +0x0000
        // Writes to field at offset +0x0002
        // Writes to field at offset +0x0004
        // Writes to field at offset +0x0006
    }

    /// Decompiled virtual method #397 @ RVA 0x00614a60 (62 instructions, 11 basic blocks)
    pub fn method_397(&mut self) {
    }

    /// Decompiled virtual method #398 @ RVA 0x00614b60 (33 instructions, 6 basic blocks)
    pub fn method_398(&mut self) {
    }

    /// Decompiled virtual method #399 @ RVA 0x00614c00 (78 instructions, 13 basic blocks)
    pub fn method_399(&mut self) {
        // Writes to field at offset +0x0000
        // Writes to field at offset +0x0000
        // Writes to field at offset +0x0010
        // Writes to field at offset +0x0018
    }

    /// Decompiled virtual method #400 @ RVA 0x00614d40 (15 instructions, 4 basic blocks)
    pub fn method_400(&mut self) {
    }

    /// Decompiled virtual method #401 @ RVA 0x00614d80 (6 instructions, 3 basic blocks)
    pub fn method_401(&mut self) {
    }

    /// Decompiled virtual method #402 @ RVA 0x00585190 (2 instructions, 1 basic blocks)
    pub fn method_402(&mut self) {
    }

    /// Decompiled virtual method #403 @ RVA 0x005851a0 (5 instructions, 1 basic blocks)
    pub fn method_403(&mut self) {
    }

    /// Decompiled virtual method #404 @ RVA 0x00614da0 (37 instructions, 8 basic blocks)
    pub fn method_404(&mut self) {
    }

    /// Decompiled virtual method #405 @ RVA 0x00614e20 (46 instructions, 10 basic blocks)
    pub fn method_405(&mut self) {
    }

    /// Decompiled virtual method #406 @ RVA 0x00614f10 (34 instructions, 6 basic blocks)
    pub fn method_406(&mut self) {
    }

    /// Decompiled virtual method #407 @ RVA 0x00614f90 (39 instructions, 6 basic blocks)
    pub fn method_407(&mut self) {
    }

    /// Decompiled virtual method #408 @ RVA 0x00615030 (13 instructions, 4 basic blocks)
    pub fn method_408(&mut self) {
    }

    /// Decompiled virtual method #409 @ RVA 0x00615090 (7 instructions, 2 basic blocks)
    pub fn method_409(&mut self) {
    }

    /// Decompiled virtual method #410 @ RVA 0x006150b0 (28 instructions, 3 basic blocks)
    pub fn method_410(&mut self) {
    }

    /// Decompiled virtual method #411 @ RVA 0x00615120 (17 instructions, 3 basic blocks)
    pub fn method_411(&mut self) {
    }

    /// Decompiled virtual method #412 @ RVA 0x005851c0 (2 instructions, 1 basic blocks)
    pub fn method_412(&mut self) {
    }

    /// Decompiled virtual method #413 @ RVA 0x005851d0 (2 instructions, 1 basic blocks)
    pub fn method_413(&mut self) {
    }

    /// Decompiled virtual method #414 @ RVA 0x005851e0 (2 instructions, 1 basic blocks)
    pub fn method_414(&mut self) {
    }

    /// Decompiled virtual method #415 @ RVA 0x005851f0 (3 instructions, 1 basic blocks)
    pub fn method_415(&mut self) {
    }

    /// Decompiled virtual method #416 @ RVA 0x000d3090 (1 instructions, 1 basic blocks)
    pub fn method_416(&mut self) {
    }

    /// Decompiled virtual method #417 @ RVA 0x00615170 (9 instructions, 3 basic blocks)
    pub fn method_417(&mut self) {
    }

    /// Decompiled virtual method #418 @ RVA 0x000d3090 (1 instructions, 1 basic blocks)
    pub fn method_418(&mut self) {
    }

    /// Decompiled virtual method #419 @ RVA 0x00615190 (9 instructions, 3 basic blocks)
    pub fn method_419(&mut self) {
    }

    /// Decompiled virtual method #420 @ RVA 0x000d6920 (2 instructions, 1 basic blocks)
    pub fn method_420(&mut self) {
    }

    /// Decompiled virtual method #421 @ RVA 0x006151b0 (10 instructions, 2 basic blocks)
    pub fn method_421(&mut self) {
    }

    /// Decompiled virtual method #422 @ RVA 0x006151e0 (6 instructions, 2 basic blocks)
    pub fn method_422(&mut self) {
    }

    /// Decompiled virtual method #423 @ RVA 0x00615210 (6 instructions, 2 basic blocks)
    pub fn method_423(&mut self) {
    }

    /// Decompiled virtual method #424 @ RVA 0x00615240 (6 instructions, 2 basic blocks)
    pub fn method_424(&mut self) {
    }

    /// Decompiled virtual method #425 @ RVA 0x00615270 (7 instructions, 2 basic blocks)
    pub fn method_425(&mut self) {
    }

    /// Decompiled virtual method #426 @ RVA 0x006152a0 (8 instructions, 2 basic blocks)
    pub fn method_426(&mut self) {
    }

    /// Decompiled virtual method #427 @ RVA 0x006152d0 (5 instructions, 3 basic blocks)
    pub fn method_427(&mut self) {
    }

    /// Decompiled virtual method #428 @ RVA 0x006152f0 (5 instructions, 3 basic blocks)
    pub fn method_428(&mut self) {
    }

    /// Decompiled virtual method #429 @ RVA 0x00615310 (5 instructions, 3 basic blocks)
    pub fn method_429(&mut self) {
    }

    /// Decompiled virtual method #430 @ RVA 0x00615330 (6 instructions, 2 basic blocks)
    pub fn method_430(&mut self) {
    }

    /// Decompiled virtual method #431 @ RVA 0x00615350 (7 instructions, 2 basic blocks)
    pub fn method_431(&mut self) {
    }

    /// Decompiled virtual method #432 @ RVA 0x00615370 (5 instructions, 3 basic blocks)
    pub fn method_432(&mut self) {
    }

    /// Decompiled virtual method #433 @ RVA 0x00615390 (5 instructions, 3 basic blocks)
    pub fn method_433(&mut self) {
    }

    /// Decompiled virtual method #434 @ RVA 0x006153b0 (5 instructions, 3 basic blocks)
    pub fn method_434(&mut self) {
    }

    /// Decompiled virtual method #435 @ RVA 0x006153d0 (6 instructions, 2 basic blocks)
    pub fn method_435(&mut self) {
    }

    /// Decompiled virtual method #436 @ RVA 0x006153f0 (7 instructions, 2 basic blocks)
    pub fn method_436(&mut self) {
    }

    /// Decompiled virtual method #437 @ RVA 0x00585200 (2 instructions, 1 basic blocks)
    pub fn method_437(&mut self) {
    }

    /// Decompiled virtual method #438 @ RVA 0x00585210 (2 instructions, 1 basic blocks)
    pub fn method_438(&mut self) {
    }

    /// Decompiled virtual method #439 @ RVA 0x00585220 (4 instructions, 1 basic blocks)
    pub fn method_439(&mut self) {
    }

    /// Decompiled virtual method #440 @ RVA 0x00585230 (2 instructions, 1 basic blocks)
    pub fn method_440(&mut self) {
    }

    /// Decompiled virtual method #441 @ RVA 0x00585240 (2 instructions, 1 basic blocks)
    pub fn method_441(&mut self) {
    }

    /// Decompiled virtual method #442 @ RVA 0x00585250 (4 instructions, 1 basic blocks)
    pub fn method_442(&mut self) {
    }

    /// Decompiled virtual method #443 @ RVA 0x00585260 (2 instructions, 1 basic blocks)
    pub fn method_443(&mut self) {
    }

    /// Decompiled virtual method #444 @ RVA 0x00585270 (3 instructions, 1 basic blocks)
    pub fn method_444(&mut self) {
    }

    /// Decompiled virtual method #445 @ RVA 0x00585280 (3 instructions, 1 basic blocks)
    pub fn method_445(&mut self) {
    }

    /// Decompiled virtual method #446 @ RVA 0x00585290 (3 instructions, 1 basic blocks)
    pub fn method_446(&mut self) {
    }

    /// Decompiled virtual method #447 @ RVA 0x005852a0 (4 instructions, 1 basic blocks)
    pub fn method_447(&mut self) {
    }

    /// Decompiled virtual method #448 @ RVA 0x005852b0 (3 instructions, 1 basic blocks)
    pub fn method_448(&mut self) {
    }

    /// Decompiled virtual method #449 @ RVA 0x005852c0 (5 instructions, 1 basic blocks)
    pub fn method_449(&mut self) {
    }

    /// Decompiled virtual method #450 @ RVA 0x005852e0 (2 instructions, 1 basic blocks)
    pub fn method_450(&mut self) {
    }

    /// Decompiled virtual method #451 @ RVA 0x005852f0 (3 instructions, 1 basic blocks)
    pub fn method_451(&mut self) {
    }

    /// Decompiled virtual method #452 @ RVA 0x00585300 (3 instructions, 1 basic blocks)
    pub fn method_452(&mut self) {
    }

    /// Decompiled virtual method #453 @ RVA 0x00615410 (12 instructions, 3 basic blocks)
    pub fn method_453(&mut self) {
    }

    /// Decompiled virtual method #454 @ RVA 0x00615440 (12 instructions, 3 basic blocks)
    pub fn method_454(&mut self) {
    }

    /// Decompiled virtual method #455 @ RVA 0x006154a0 (45 instructions, 3 basic blocks)
    pub fn method_455(&mut self) {
    }

    /// Decompiled virtual method #456 @ RVA 0x00615470 (12 instructions, 2 basic blocks)
    pub fn method_456(&mut self) {
    }

    /// Decompiled virtual method #457 @ RVA 0x00615550 (12 instructions, 3 basic blocks)
    pub fn method_457(&mut self) {
    }

    /// Decompiled virtual method #458 @ RVA 0x00615580 (12 instructions, 3 basic blocks)
    pub fn method_458(&mut self) {
    }

    /// Decompiled virtual method #459 @ RVA 0x006155b0 (12 instructions, 2 basic blocks)
    pub fn method_459(&mut self) {
    }

    /// Decompiled virtual method #460 @ RVA 0x006154f0 (12 instructions, 3 basic blocks)
    pub fn method_460(&mut self) {
    }

    /// Decompiled virtual method #461 @ RVA 0x00615520 (12 instructions, 2 basic blocks)
    pub fn method_461(&mut self) {
    }

    /// Decompiled virtual method #462 @ RVA 0x00585310 (3 instructions, 1 basic blocks)
    pub fn method_462(&mut self) {
    }

    /// Decompiled virtual method #463 @ RVA 0x006155e0 (7 instructions, 4 basic blocks)
    pub fn method_463(&mut self) {
        // Writes to field at offset +0x1600
    }

    /// Decompiled virtual method #464 @ RVA 0x00615600 (5 instructions, 2 basic blocks)
    pub fn method_464(&mut self) {
    }

    /// Decompiled virtual method #465 @ RVA 0x00615620 (7 instructions, 4 basic blocks)
    pub fn method_465(&mut self) {
    }

    /// Decompiled virtual method #466 @ RVA 0x00585320 (2 instructions, 1 basic blocks)
    pub fn method_466(&mut self) {
    }

    /// Decompiled virtual method #467 @ RVA 0x00585330 (2 instructions, 1 basic blocks)
    pub fn method_467(&mut self) {
    }

    /// Decompiled virtual method #468 @ RVA 0x006162b0 (4 instructions, 1 basic blocks)
    pub fn method_468(&mut self) {
    }

    /// Decompiled virtual method #469 @ RVA 0x006162c0 (4 instructions, 1 basic blocks)
    pub fn method_469(&mut self) {
    }

    /// Decompiled virtual method #470 @ RVA 0x006162f0 (4 instructions, 1 basic blocks)
    pub fn method_470(&mut self) {
    }

    /// Decompiled virtual method #471 @ RVA 0x006162d0 (5 instructions, 1 basic blocks)
    pub fn method_471(&mut self) {
        // Writes to field at offset +0x0000
    }

    /// Decompiled virtual method #472 @ RVA 0x00616310 (29 instructions, 5 basic blocks)
    pub fn method_472(&mut self) {
    }

    /// Decompiled virtual method #473 @ RVA 0x00616370 (5 instructions, 1 basic blocks)
    pub fn method_473(&mut self) {
    }

    /// Decompiled virtual method #474 @ RVA 0x00615640 (38 instructions, 4 basic blocks)
    pub fn method_474(&mut self) {
    }

    /// Decompiled virtual method #475 @ RVA 0x006156d0 (6 instructions, 1 basic blocks)
    pub fn method_475(&mut self) {
    }

    /// Decompiled virtual method #476 @ RVA 0x006156f0 (8 instructions, 1 basic blocks)
    pub fn method_476(&mut self) {
        // Writes to field at offset +0x0000
        // Writes to field at offset +0x0004
    }

    /// Decompiled virtual method #477 @ RVA 0x00615710 (37 instructions, 1 basic blocks)
    pub fn method_477(&mut self) {
    }

    /// Decompiled virtual method #478 @ RVA 0x006157b0 (4 instructions, 2 basic blocks)
    pub fn method_478(&mut self) {
    }

    /// Decompiled virtual method #479 @ RVA 0x006157d0 (5 instructions, 3 basic blocks)
    pub fn method_479(&mut self) {
    }

    /// Decompiled virtual method #480 @ RVA 0x00585340 (2 instructions, 1 basic blocks)
    pub fn method_480(&mut self) {
    }

    /// Decompiled virtual method #481 @ RVA 0x006157f0 (21 instructions, 3 basic blocks)
    pub fn method_481(&mut self) {
    }

    /// Decompiled virtual method #482 @ RVA 0x00103e70 (2 instructions, 1 basic blocks)
    pub fn method_482(&mut self) {
    }

    /// Decompiled virtual method #483 @ RVA 0x00615840 (5 instructions, 1 basic blocks)
    pub fn method_483(&mut self) {
    }

    /// Decompiled virtual method #484 @ RVA 0x000d3090 (1 instructions, 1 basic blocks)
    pub fn method_484(&mut self) {
    }

    /// Decompiled virtual method #485 @ RVA 0x00103e80 (2 instructions, 1 basic blocks)
    pub fn method_485(&mut self) {
    }

    /// Decompiled virtual method #486 @ RVA 0x00584540 (2 instructions, 1 basic blocks)
    pub fn method_486(&mut self) {
    }

    /// Decompiled virtual method #487 @ RVA 0x00615b20 (2 instructions, 1 basic blocks)
    pub fn method_487(&mut self) {
        // Writes to field at offset +0x05aa
    }

    /// Decompiled virtual method #488 @ RVA 0x00615b30 (3 instructions, 1 basic blocks)
    pub fn method_488(&mut self) {
        // Writes to field at offset +0x05aa
    }

    /// Decompiled virtual method #489 @ RVA 0x00615b40 (2 instructions, 1 basic blocks)
    pub fn method_489(&mut self) {
        // Writes to field at offset +0x05aa
    }

    /// Decompiled virtual method #490 @ RVA 0x00615b50 (3 instructions, 1 basic blocks)
    pub fn method_490(&mut self) {
        // Writes to field at offset +0x05aa
    }

    /// Decompiled virtual method #491 @ RVA 0x00615b60 (2 instructions, 1 basic blocks)
    pub fn method_491(&mut self) {
        // Writes to field at offset +0x0008
    }

    /// Decompiled virtual method #492 @ RVA 0x00615b80 (6 instructions, 3 basic blocks)
    pub fn method_492(&mut self) {
        // Writes to field at offset +0x0008
    }

    /// Decompiled virtual method #493 @ RVA 0x00615b70 (2 instructions, 1 basic blocks)
    pub fn method_493(&mut self) {
    }

    /// Decompiled virtual method #494 @ RVA 0x00615ba0 (5 instructions, 1 basic blocks)
    pub fn method_494(&mut self) {
        // Writes to field at offset +0x000a
    }

    /// Decompiled virtual method #495 @ RVA 0x00615bd0 (6 instructions, 3 basic blocks)
    pub fn method_495(&mut self) {
        // Writes to field at offset +0x000a
    }

    /// Decompiled virtual method #496 @ RVA 0x00615bc0 (2 instructions, 1 basic blocks)
    pub fn method_496(&mut self) {
    }

    /// Decompiled virtual method #497 @ RVA 0x00615ad0 (3 instructions, 1 basic blocks)
    pub fn method_497(&mut self) {
        // Writes to field at offset +0x000e
    }

    /// Decompiled virtual method #498 @ RVA 0x00615ae0 (6 instructions, 1 basic blocks)
    pub fn method_498(&mut self) {
        // Writes to field at offset +0x000c
    }

    /// Decompiled virtual method #499 @ RVA 0x00615b00 (3 instructions, 1 basic blocks)
    pub fn method_499(&mut self) {
    }

    /// Decompiled virtual method #500 @ RVA 0x00615b10 (3 instructions, 1 basic blocks)
    pub fn method_500(&mut self) {
    }

    /// Decompiled virtual method #501 @ RVA 0x00615bf0 (6 instructions, 3 basic blocks)
    pub fn method_501(&mut self) {
        // Writes to field at offset +0x020c
    }

    /// Decompiled virtual method #502 @ RVA 0x00615c10 (6 instructions, 3 basic blocks)
    pub fn method_502(&mut self) {
        // Writes to field at offset +0x020e
    }

    /// Decompiled virtual method #503 @ RVA 0x000d3090 (1 instructions, 1 basic blocks)
    pub fn method_503(&mut self) {
    }

    /// Decompiled virtual method #504 @ RVA 0x000d3090 (1 instructions, 1 basic blocks)
    pub fn method_504(&mut self) {
    }

    /// Decompiled virtual method #505 @ RVA 0x00615cb0 (6 instructions, 3 basic blocks)
    pub fn method_505(&mut self) {
        // Writes to field at offset +0x0212
    }

    /// Decompiled virtual method #506 @ RVA 0x000d3090 (1 instructions, 1 basic blocks)
    pub fn method_506(&mut self) {
    }

    /// Decompiled virtual method #507 @ RVA 0x00103e80 (2 instructions, 1 basic blocks)
    pub fn method_507(&mut self) {
    }

    /// Decompiled virtual method #508 @ RVA 0x00615c30 (48 instructions, 5 basic blocks)
    pub fn method_508(&mut self) {
        // Writes to field at offset +0x0210
        // Writes to field at offset +0x0212
    }

    /// Decompiled virtual method #509 @ RVA 0x000d3090 (1 instructions, 1 basic blocks)
    pub fn method_509(&mut self) {
    }

    /// Decompiled virtual method #510 @ RVA 0x000d3090 (1 instructions, 1 basic blocks)
    pub fn method_510(&mut self) {
    }

    /// Decompiled virtual method #511 @ RVA 0x00103e80 (2 instructions, 1 basic blocks)
    pub fn method_511(&mut self) {
    }

    /// Decompiled virtual method #512 @ RVA 0x00103e80 (2 instructions, 1 basic blocks)
    pub fn method_512(&mut self) {
    }

    /// Decompiled virtual method #513 @ RVA 0x000d3090 (1 instructions, 1 basic blocks)
    pub fn method_513(&mut self) {
    }

    /// Decompiled virtual method #514 @ RVA 0x00615cd0 (8 instructions, 3 basic blocks)
    pub fn method_514(&mut self) {
        // Writes to field at offset +0x002c
    }

    /// Decompiled virtual method #515 @ RVA 0x00615cf0 (8 instructions, 3 basic blocks)
    pub fn method_515(&mut self) {
        // Writes to field at offset +0x002e
    }

    /// Decompiled virtual method #516 @ RVA 0x00615d10 (53 instructions, 3 basic blocks)
    pub fn method_516(&mut self) {
        // Writes to field at offset +0x0000
    }

    /// Decompiled virtual method #517 @ RVA 0x00615da0 (9 instructions, 3 basic blocks)
    pub fn method_517(&mut self) {
        // Writes to field at offset +0x0000
    }

    /// Decompiled virtual method #518 @ RVA 0x00615dc0 (9 instructions, 1 basic blocks)
    pub fn method_518(&mut self) {
        // Writes to field at offset +0x0034
    }

    /// Decompiled virtual method #519 @ RVA 0x00103e80 (2 instructions, 1 basic blocks)
    pub fn method_519(&mut self) {
    }

    /// Decompiled virtual method #520 @ RVA 0x00103e80 (2 instructions, 1 basic blocks)
    pub fn method_520(&mut self) {
    }

    /// Decompiled virtual method #521 @ RVA 0x00103e80 (2 instructions, 1 basic blocks)
    pub fn method_521(&mut self) {
    }

    /// Decompiled virtual method #522 @ RVA 0x00103e80 (2 instructions, 1 basic blocks)
    pub fn method_522(&mut self) {
    }

    /// Decompiled virtual method #523 @ RVA 0x00615df0 (6 instructions, 3 basic blocks)
    pub fn method_523(&mut self) {
        // Writes to field at offset +0x0214
    }

    /// Decompiled virtual method #524 @ RVA 0x00615e10 (6 instructions, 3 basic blocks)
    pub fn method_524(&mut self) {
        // Writes to field at offset +0x0216
    }

    /// Decompiled virtual method #525 @ RVA 0x00615e30 (6 instructions, 3 basic blocks)
    pub fn method_525(&mut self) {
        // Writes to field at offset +0x0218
    }

    /// Decompiled virtual method #526 @ RVA 0x00615e50 (6 instructions, 3 basic blocks)
    pub fn method_526(&mut self) {
        // Writes to field at offset +0x021a
    }

    /// Decompiled virtual method #527 @ RVA 0x00615f90 (11 instructions, 5 basic blocks)
    pub fn method_527(&mut self) {
    }

    /// Decompiled virtual method #528 @ RVA 0x000d6920 (2 instructions, 1 basic blocks)
    pub fn method_528(&mut self) {
    }

    /// Decompiled virtual method #529 @ RVA 0x00615fe0 (6 instructions, 3 basic blocks)
    pub fn method_529(&mut self) {
        // Writes to field at offset +0x05a6
    }

    /// Decompiled virtual method #530 @ RVA 0x00616000 (6 instructions, 3 basic blocks)
    pub fn method_530(&mut self) {
        // Writes to field at offset +0x05a8
    }

    /// Decompiled virtual method #531 @ RVA 0x00616020 (5 instructions, 3 basic blocks)
    pub fn method_531(&mut self) {
    }

    /// Decompiled virtual method #532 @ RVA 0x000d6920 (2 instructions, 1 basic blocks)
    pub fn method_532(&mut self) {
    }

    /// Decompiled virtual method #533 @ RVA 0x00615e70 (6 instructions, 3 basic blocks)
    pub fn method_533(&mut self) {
        // Writes to field at offset +0x021c
    }

    /// Decompiled virtual method #534 @ RVA 0x00615e90 (6 instructions, 3 basic blocks)
    pub fn method_534(&mut self) {
        // Writes to field at offset +0x021e
    }

    /// Decompiled virtual method #535 @ RVA 0x00615eb0 (6 instructions, 3 basic blocks)
    pub fn method_535(&mut self) {
        // Writes to field at offset +0x0220
    }

    /// Decompiled virtual method #536 @ RVA 0x00615ed0 (6 instructions, 3 basic blocks)
    pub fn method_536(&mut self) {
        // Writes to field at offset +0x0222
    }

    /// Decompiled virtual method #537 @ RVA 0x00615ef0 (6 instructions, 3 basic blocks)
    pub fn method_537(&mut self) {
        // Writes to field at offset +0x0224
    }

    /// Decompiled virtual method #538 @ RVA 0x000d3090 (1 instructions, 1 basic blocks)
    pub fn method_538(&mut self) {
    }

    /// Decompiled virtual method #539 @ RVA 0x000d3090 (1 instructions, 1 basic blocks)
    pub fn method_539(&mut self) {
    }

    /// Decompiled virtual method #540 @ RVA 0x00103e80 (2 instructions, 1 basic blocks)
    pub fn method_540(&mut self) {
    }

    /// Decompiled virtual method #541 @ RVA 0x00103e80 (2 instructions, 1 basic blocks)
    pub fn method_541(&mut self) {
    }

    /// Decompiled virtual method #542 @ RVA 0x00615f10 (7 instructions, 1 basic blocks)
    pub fn method_542(&mut self) {
        // Writes to field at offset +0x0228
    }

    /// Decompiled virtual method #543 @ RVA 0x00615f50 (6 instructions, 3 basic blocks)
    pub fn method_543(&mut self) {
        // Writes to field at offset +0x022c
    }

    /// Decompiled virtual method #544 @ RVA 0x00615f70 (6 instructions, 3 basic blocks)
    pub fn method_544(&mut self) {
        // Writes to field at offset +0x022e
    }

    /// Decompiled virtual method #545 @ RVA 0x00615f30 (5 instructions, 1 basic blocks)
    pub fn method_545(&mut self) {
        // Writes to field at offset +0x0228
    }

    /// Decompiled virtual method #546 @ RVA 0x000d3090 (1 instructions, 1 basic blocks)
    pub fn method_546(&mut self) {
    }

    /// Decompiled virtual method #547 @ RVA 0x00616390 (4 instructions, 1 basic blocks)
    pub fn method_547(&mut self) {
    }

    /// Decompiled virtual method #548 @ RVA 0x006163a0 (5 instructions, 1 basic blocks)
    pub fn method_548(&mut self) {
    }

    /// Decompiled virtual method #549 @ RVA 0x006163b0 (5 instructions, 1 basic blocks)
    pub fn method_549(&mut self) {
    }

    /// Decompiled virtual method #550 @ RVA 0x006163d0 (3 instructions, 1 basic blocks)
    pub fn method_550(&mut self) {
    }

    /// Decompiled virtual method #551 @ RVA 0x006163e0 (3 instructions, 1 basic blocks)
    pub fn method_551(&mut self) {
    }

    /// Decompiled virtual method #552 @ RVA 0x006163f0 (3 instructions, 1 basic blocks)
    pub fn method_552(&mut self) {
    }

    /// Decompiled virtual method #553 @ RVA 0x00616420 (4 instructions, 1 basic blocks)
    pub fn method_553(&mut self) {
    }

    /// Decompiled virtual method #554 @ RVA 0x00616400 (5 instructions, 1 basic blocks)
    pub fn method_554(&mut self) {
    }

    /// Decompiled virtual method #555 @ RVA 0x00616430 (10 instructions, 1 basic blocks)
    pub fn method_555(&mut self) {
    }

    /// Decompiled virtual method #556 @ RVA 0x00616460 (10 instructions, 1 basic blocks)
    pub fn method_556(&mut self) {
    }

    /// Decompiled virtual method #557 @ RVA 0x00616490 (10 instructions, 1 basic blocks)
    pub fn method_557(&mut self) {
    }

    /// Decompiled virtual method #558 @ RVA 0x00616040 (130 instructions, 2 basic blocks)
    pub fn method_558(&mut self) {
    }

    /// Decompiled virtual method #559 @ RVA 0x006164c0 (16 instructions, 2 basic blocks)
    pub fn method_559(&mut self) {
    }

    /// Decompiled virtual method #560 @ RVA 0x00616500 (18 instructions, 3 basic blocks)
    pub fn method_560(&mut self) {
    }

    /// Decompiled virtual method #561 @ RVA 0x00616540 (19 instructions, 3 basic blocks)
    pub fn method_561(&mut self) {
    }

    /// Decompiled virtual method #562 @ RVA 0x00616580 (16 instructions, 2 basic blocks)
    pub fn method_562(&mut self) {
    }

    /// Decompiled virtual method #563 @ RVA 0x006165c0 (18 instructions, 3 basic blocks)
    pub fn method_563(&mut self) {
    }

    /// Decompiled virtual method #564 @ RVA 0x00616600 (19 instructions, 3 basic blocks)
    pub fn method_564(&mut self) {
    }

    /// Decompiled virtual method #565 @ RVA 0x00616640 (16 instructions, 2 basic blocks)
    pub fn method_565(&mut self) {
    }

    /// Decompiled virtual method #566 @ RVA 0x00616680 (18 instructions, 3 basic blocks)
    pub fn method_566(&mut self) {
    }

    /// Decompiled virtual method #567 @ RVA 0x006166c0 (19 instructions, 3 basic blocks)
    pub fn method_567(&mut self) {
    }

    /// Decompiled virtual method #568 @ RVA 0x00616700 (16 instructions, 2 basic blocks)
    pub fn method_568(&mut self) {
    }

    /// Decompiled virtual method #569 @ RVA 0x00616740 (18 instructions, 3 basic blocks)
    pub fn method_569(&mut self) {
    }

    /// Decompiled virtual method #570 @ RVA 0x00616780 (19 instructions, 3 basic blocks)
    pub fn method_570(&mut self) {
    }

    /// Decompiled virtual method #571 @ RVA 0x00103e70 (2 instructions, 1 basic blocks)
    pub fn method_571(&mut self) {
    }

    /// Decompiled virtual method #572 @ RVA 0x000d3090 (1 instructions, 1 basic blocks)
    pub fn method_572(&mut self) {
    }

    /// Decompiled virtual method #573 @ RVA 0x000d3090 (1 instructions, 1 basic blocks)
    pub fn method_573(&mut self) {
    }

    /// Decompiled virtual method #574 @ RVA 0x000d3090 (1 instructions, 1 basic blocks)
    pub fn method_574(&mut self) {
    }

    /// Decompiled virtual method #575 @ RVA 0x00103e70 (2 instructions, 1 basic blocks)
    pub fn method_575(&mut self) {
    }

    /// Decompiled virtual method #576 @ RVA 0x000d3090 (1 instructions, 1 basic blocks)
    pub fn method_576(&mut self) {
    }

    /// Decompiled virtual method #577 @ RVA 0x00103e70 (2 instructions, 1 basic blocks)
    pub fn method_577(&mut self) {
    }

    /// Decompiled virtual method #578 @ RVA 0x000d3090 (1 instructions, 1 basic blocks)
    pub fn method_578(&mut self) {
    }

    /// Decompiled virtual method #579 @ RVA 0x000d3090 (1 instructions, 1 basic blocks)
    pub fn method_579(&mut self) {
    }

    /// Decompiled virtual method #580 @ RVA 0x00103e80 (2 instructions, 1 basic blocks)
    pub fn method_580(&mut self) {
    }

    /// Decompiled virtual method #581 @ RVA 0x000d3090 (1 instructions, 1 basic blocks)
    pub fn method_581(&mut self) {
    }

    /// Decompiled virtual method #582 @ RVA 0x000d3090 (1 instructions, 1 basic blocks)
    pub fn method_582(&mut self) {
    }

    /// Decompiled virtual method #583 @ RVA 0x00103e80 (2 instructions, 1 basic blocks)
    pub fn method_583(&mut self) {
    }

    /// Decompiled virtual method #584 @ RVA 0x000d3090 (1 instructions, 1 basic blocks)
    pub fn method_584(&mut self) {
    }

    /// Decompiled virtual method #585 @ RVA 0x000d3090 (1 instructions, 1 basic blocks)
    pub fn method_585(&mut self) {
    }

    /// Decompiled virtual method #586 @ RVA 0x00103e80 (2 instructions, 1 basic blocks)
    pub fn method_586(&mut self) {
    }

    /// Decompiled virtual method #587 @ RVA 0x00103e70 (2 instructions, 1 basic blocks)
    pub fn method_587(&mut self) {
    }

    /// Decompiled virtual method #588 @ RVA 0x000d3090 (1 instructions, 1 basic blocks)
    pub fn method_588(&mut self) {
    }

    /// Decompiled virtual method #589 @ RVA 0x00103e80 (2 instructions, 1 basic blocks)
    pub fn method_589(&mut self) {
    }

    /// Decompiled virtual method #590 @ RVA 0x000d3090 (1 instructions, 1 basic blocks)
    pub fn method_590(&mut self) {
    }

    /// Decompiled virtual method #591 @ RVA 0x000d3090 (1 instructions, 1 basic blocks)
    pub fn method_591(&mut self) {
    }

    /// Decompiled virtual method #592 @ RVA 0x00103e80 (2 instructions, 1 basic blocks)
    pub fn method_592(&mut self) {
    }

    /// Decompiled virtual method #593 @ RVA 0x000d3090 (1 instructions, 1 basic blocks)
    pub fn method_593(&mut self) {
    }

    /// Decompiled virtual method #594 @ RVA 0x000d6920 (2 instructions, 1 basic blocks)
    pub fn method_594(&mut self) {
    }

    /// Decompiled virtual method #595 @ RVA 0x00103e80 (2 instructions, 1 basic blocks)
    pub fn method_595(&mut self) {
    }

    /// Decompiled virtual method #596 @ RVA 0x000d3090 (1 instructions, 1 basic blocks)
    pub fn method_596(&mut self) {
    }

    /// Decompiled virtual method #597 @ RVA 0x000d3090 (1 instructions, 1 basic blocks)
    pub fn method_597(&mut self) {
    }

    /// Decompiled virtual method #598 @ RVA 0x00103e80 (2 instructions, 1 basic blocks)
    pub fn method_598(&mut self) {
    }

    /// Decompiled virtual method #599 @ RVA 0x000d3090 (1 instructions, 1 basic blocks)
    pub fn method_599(&mut self) {
    }

    /// Decompiled virtual method #600 @ RVA 0x000d3090 (1 instructions, 1 basic blocks)
    pub fn method_600(&mut self) {
    }

    /// Decompiled virtual method #601 @ RVA 0x00103e80 (2 instructions, 1 basic blocks)
    pub fn method_601(&mut self) {
    }

    /// Decompiled virtual method #602 @ RVA 0x00103e80 (2 instructions, 1 basic blocks)
    pub fn method_602(&mut self) {
    }

    /// Decompiled virtual method #603 @ RVA 0x000d6920 (2 instructions, 1 basic blocks)
    pub fn method_603(&mut self) {
    }

    /// Decompiled virtual method #604 @ RVA 0x000d6920 (2 instructions, 1 basic blocks)
    pub fn method_604(&mut self) {
    }

    /// Decompiled virtual method #605 @ RVA 0x00103e80 (2 instructions, 1 basic blocks)
    pub fn method_605(&mut self) {
    }

    /// Decompiled virtual method #606 @ RVA 0x000d3090 (1 instructions, 1 basic blocks)
    pub fn method_606(&mut self) {
    }

    /// Decompiled virtual method #607 @ RVA 0x00103e80 (2 instructions, 1 basic blocks)
    pub fn method_607(&mut self) {
    }

    /// Decompiled virtual method #608 @ RVA 0x00103e80 (2 instructions, 1 basic blocks)
    pub fn method_608(&mut self) {
    }

    /// Decompiled virtual method #609 @ RVA 0x000d3090 (1 instructions, 1 basic blocks)
    pub fn method_609(&mut self) {
    }

    /// Decompiled virtual method #610 @ RVA 0x000d3090 (1 instructions, 1 basic blocks)
    pub fn method_610(&mut self) {
    }

    /// Decompiled virtual method #611 @ RVA 0x00103e80 (2 instructions, 1 basic blocks)
    pub fn method_611(&mut self) {
    }

    /// Decompiled virtual method #612 @ RVA 0x000d3090 (1 instructions, 1 basic blocks)
    pub fn method_612(&mut self) {
    }

    /// Decompiled virtual method #613 @ RVA 0x000d3090 (1 instructions, 1 basic blocks)
    pub fn method_613(&mut self) {
    }

    /// Decompiled virtual method #614 @ RVA 0x00103e80 (2 instructions, 1 basic blocks)
    pub fn method_614(&mut self) {
    }

    /// Decompiled virtual method #615 @ RVA 0x000d3090 (1 instructions, 1 basic blocks)
    pub fn method_615(&mut self) {
    }

    /// Decompiled virtual method #616 @ RVA 0x00103e80 (2 instructions, 1 basic blocks)
    pub fn method_616(&mut self) {
    }

    /// Decompiled virtual method #617 @ RVA 0x000d3090 (1 instructions, 1 basic blocks)
    pub fn method_617(&mut self) {
    }

    /// Decompiled virtual method #618 @ RVA 0x00103e70 (2 instructions, 1 basic blocks)
    pub fn method_618(&mut self) {
    }

    /// Decompiled virtual method #619 @ RVA 0x000d6920 (2 instructions, 1 basic blocks)
    pub fn method_619(&mut self) {
    }

    /// Decompiled virtual method #620 @ RVA 0x000d3090 (1 instructions, 1 basic blocks)
    pub fn method_620(&mut self) {
    }

    /// Decompiled virtual method #621 @ RVA 0x00103e80 (2 instructions, 1 basic blocks)
    pub fn method_621(&mut self) {
    }

    /// Decompiled virtual method #622 @ RVA 0x000d3090 (1 instructions, 1 basic blocks)
    pub fn method_622(&mut self) {
    }

    /// Decompiled virtual method #623 @ RVA 0x000d3090 (1 instructions, 1 basic blocks)
    pub fn method_623(&mut self) {
    }

    /// Decompiled virtual method #624 @ RVA 0x000d3090 (1 instructions, 1 basic blocks)
    pub fn method_624(&mut self) {
    }

    /// Decompiled virtual method #625 @ RVA 0x000d3090 (1 instructions, 1 basic blocks)
    pub fn method_625(&mut self) {
    }

    /// Decompiled virtual method #626 @ RVA 0x00103e80 (2 instructions, 1 basic blocks)
    pub fn method_626(&mut self) {
    }

    /// Decompiled virtual method #627 @ RVA 0x00103e80 (2 instructions, 1 basic blocks)
    pub fn method_627(&mut self) {
    }

    /// Decompiled virtual method #628 @ RVA 0x000d3090 (1 instructions, 1 basic blocks)
    pub fn method_628(&mut self) {
    }

    /// Decompiled virtual method #629 @ RVA 0x00584510 (2 instructions, 1 basic blocks)
    pub fn method_629(&mut self) {
    }

    /// Decompiled virtual method #630 @ RVA 0x000d3090 (1 instructions, 1 basic blocks)
    pub fn method_630(&mut self) {
    }

    /// Decompiled virtual method #631 @ RVA 0x00584510 (2 instructions, 1 basic blocks)
    pub fn method_631(&mut self) {
    }

    /// Decompiled virtual method #632 @ RVA 0x00103e80 (2 instructions, 1 basic blocks)
    pub fn method_632(&mut self) {
    }

    /// Decompiled virtual method #633 @ RVA 0x000d3090 (1 instructions, 1 basic blocks)
    pub fn method_633(&mut self) {
    }

    /// Decompiled virtual method #634 @ RVA 0x000d3090 (1 instructions, 1 basic blocks)
    pub fn method_634(&mut self) {
    }

    /// Decompiled virtual method #635 @ RVA 0x00103e80 (2 instructions, 1 basic blocks)
    pub fn method_635(&mut self) {
    }

    /// Decompiled virtual method #636 @ RVA 0x000d3090 (1 instructions, 1 basic blocks)
    pub fn method_636(&mut self) {
    }

    /// Decompiled virtual method #637 @ RVA 0x000d3090 (1 instructions, 1 basic blocks)
    pub fn method_637(&mut self) {
    }

    /// Decompiled virtual method #638 @ RVA 0x00585350 (4 instructions, 1 basic blocks)
    pub fn method_638(&mut self) {
    }

    /// Decompiled virtual method #639 @ RVA 0x00585370 (8 instructions, 4 basic blocks)
    pub fn method_639(&mut self) {
    }

    /// Decompiled virtual method #640 @ RVA 0x000d3090 (1 instructions, 1 basic blocks)
    pub fn method_640(&mut self) {
    }

    /// Decompiled virtual method #641 @ RVA 0x000d6920 (2 instructions, 1 basic blocks)
    pub fn method_641(&mut self) {
    }

    /// Decompiled virtual method #642 @ RVA 0x000d6920 (2 instructions, 1 basic blocks)
    pub fn method_642(&mut self) {
    }

    /// Decompiled virtual method #643 @ RVA 0x00103e70 (2 instructions, 1 basic blocks)
    pub fn method_643(&mut self) {
    }

    /// Decompiled virtual method #644 @ RVA 0x006167c0 (2 instructions, 1 basic blocks)
    pub fn method_644(&mut self) {
    }

    /// Decompiled virtual method #645 @ RVA 0x006167d0 (2 instructions, 1 basic blocks)
    pub fn method_645(&mut self) {
    }

    /// Decompiled virtual method #646 @ RVA 0x000d3090 (1 instructions, 1 basic blocks)
    pub fn method_646(&mut self) {
    }

    /// Decompiled virtual method #647 @ RVA 0x00585390 (13 instructions, 3 basic blocks)
    pub fn method_647(&mut self) {
        // Writes to field at offset +0x0000
    }

}

impl CcClass for ccSaveDataVol1_vol1 {
    fn class_name(&self) -> &'static str { Self::CLASS_NAME }
    fn vtable_rva(&self) -> u32 { Self::VTABLE_RVA }
    fn virtual_method_count(&self) -> usize { Self::METHOD_COUNT }
    fn base_classes(&self) -> &'static [&'static str] { Self::BASE_CLASSES }
}

/// Decompiled CC2 class: `ccSkillTriggerOk` (VTable RVA: 0x00881310, 16 virtual methods, 27 recovered fields)
/// Inherits: ccStateIconBase, ccDispBase
#[derive(Debug, Clone, Default)]
pub struct ccSkillTriggerOk {
    pub vtable_rva: u32,
    pub field_0x0000: u64,
    pub field_0x0008: u32,
    pub field_0x000c: u32,
    pub field_0x0010: u32,
    pub field_0x0014: f32,
    pub field_0x0018: f32,
    pub field_0x001c: u32,
    pub field_0x0020: u32,
    pub field_0x0028: u32,
    pub field_0x002c: u32,
    pub field_0x0030: u32,
    pub field_0x0032: u16,
    pub field_0x0034: u16,
    pub field_0x0038: u8,
    pub field_0x003c: u16,
    pub field_0x003e: u16,
    pub field_0x0040: u64,
    pub field_0x0048: u32,
    pub field_0x0058: u64,
    pub field_0x0068: u64,
    pub field_0x0070: u32,
    pub field_0x0074: u8,
    pub field_0x0078: u32,
    pub field_0x008c: f32,
    pub field_0x00a4: u32,
    pub field_0x0148: u64,
    pub field_0x0390: u64,
}

impl ccSkillTriggerOk {
    pub const CLASS_NAME: &'static str = "ccSkillTriggerOk";
    pub const VTABLE_RVA: u32 = 0x00881310;
    pub const METHOD_COUNT: usize = 16;
    pub const BASE_CLASSES: &'static [&'static str] = &["ccStateIconBase", "ccDispBase"];
    pub const VIRTUAL_METHODS: &'static [u64] = &[0x00000001802374f0, 0x000000018023d5f0, 0x000000018023d8b0, 0x000000018023dd30, 0x00000001802a0e30, 0x00000001800d3090, 0x00000001801b81f0, 0x00000001801b8200, 0x00000001801b8210, 0x000000018023da90, 0x00000001800d3090, 0x000000018023da00, 0x000000018023dca0, 0x0000000180234180, 0x0000000180103e90, 0x000000018023e330];

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
            field_0x0028: Default::default(),
            field_0x002c: Default::default(),
            field_0x0030: Default::default(),
            field_0x0032: Default::default(),
            field_0x0034: Default::default(),
            field_0x0038: Default::default(),
            field_0x003c: Default::default(),
            field_0x003e: Default::default(),
            field_0x0040: Default::default(),
            field_0x0048: Default::default(),
            field_0x0058: Default::default(),
            field_0x0068: Default::default(),
            field_0x0070: Default::default(),
            field_0x0074: Default::default(),
            field_0x0078: Default::default(),
            field_0x008c: Default::default(),
            field_0x00a4: Default::default(),
            field_0x0148: Default::default(),
            field_0x0390: Default::default(),
        }
    }

    /// Decompiled virtual method #0 @ RVA 0x002374f0 (13 instructions, 3 basic blocks)
    pub fn method_00(&mut self) {
        // Writes to field at offset +0x0000
    }

    /// Decompiled virtual method #1 @ RVA 0x0023d5f0 (122 instructions, 24 basic blocks)
    pub fn method_01(&mut self) {
        // Writes to field at offset +0x0058
    }

    /// Decompiled virtual method #2 @ RVA 0x0023d8b0 (90 instructions, 23 basic blocks)
    pub fn method_02(&mut self) {
    }

    /// Decompiled virtual method #3 @ RVA 0x0023dd30 (111 instructions, 17 basic blocks)
    pub fn method_03(&mut self) {
        // Writes to field at offset +0x0000
        // Writes to field at offset +0x0008
        // Writes to field at offset +0x0038
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

    /// Decompiled virtual method #9 @ RVA 0x0023da90 (114 instructions, 28 basic blocks)
    pub fn method_09(&mut self) {
        // Writes to field at offset +0x0020
        // Writes to field at offset +0x0070
    }

    /// Decompiled virtual method #10 @ RVA 0x000d3090 (1 instructions, 1 basic blocks)
    pub fn method_10(&mut self) {
    }

    /// Decompiled virtual method #11 @ RVA 0x0023da00 (35 instructions, 9 basic blocks)
    pub fn method_11(&mut self) {
        // Writes to field at offset +0x001c
        // Writes to field at offset +0x001c
        // Writes to field at offset +0x0078
    }

    /// Decompiled virtual method #12 @ RVA 0x0023dca0 (20 instructions, 7 basic blocks)
    pub fn method_12(&mut self) {
        // Writes to field at offset +0x001c
        // Writes to field at offset +0x001c
        // Writes to field at offset +0x0074
    }

    /// Decompiled virtual method #13 @ RVA 0x00234180 (26 instructions, 6 basic blocks)
    pub fn method_13(&mut self) {
        // Writes to field at offset +0x001c
        // Writes to field at offset +0x001c
    }

    /// Decompiled virtual method #14 @ RVA 0x00103e90 (2 instructions, 1 basic blocks)
    pub fn method_14(&mut self) {
    }

    /// Decompiled virtual method #15 @ RVA 0x0023e330 (83 instructions, 14 basic blocks)
    pub fn method_15(&mut self) {
        // Writes to field at offset +0x00a4
    }

}

impl CcClass for ccSkillTriggerOk {
    fn class_name(&self) -> &'static str { Self::CLASS_NAME }
    fn vtable_rva(&self) -> u32 { Self::VTABLE_RVA }
    fn virtual_method_count(&self) -> usize { Self::METHOD_COUNT }
    fn base_classes(&self) -> &'static [&'static str] { Self::BASE_CLASSES }
}

/// Decompiled CC2 class: `ccSpotLight` (VTable RVA: 0x00876f20, 2 virtual methods, 5 recovered fields)
/// Inherits: ccLight, ccCoord
#[derive(Debug, Clone, Default)]
pub struct ccSpotLight {
    pub vtable_rva: u32,
    pub field_0x0000: u64,
    pub field_0x00a1: u8,
    pub field_0x00d0: u64,
    pub field_0x0120: u32,
    pub field_0x0170: u32,
}

impl ccSpotLight {
    pub const CLASS_NAME: &'static str = "ccSpotLight";
    pub const VTABLE_RVA: u32 = 0x00876f20;
    pub const METHOD_COUNT: usize = 2;
    pub const BASE_CLASSES: &'static [&'static str] = &["ccLight", "ccCoord"];
    pub const VIRTUAL_METHODS: &'static [u64] = &[0x00000001801588e0, 0x000000018014c940];

    pub fn new() -> Self {
        Self {
            vtable_rva: Self::VTABLE_RVA,
            field_0x0000: Default::default(),
            field_0x00a1: Default::default(),
            field_0x00d0: Default::default(),
            field_0x0120: Default::default(),
            field_0x0170: Default::default(),
        }
    }

    /// Decompiled virtual method #0 @ RVA 0x001588e0 (22 instructions, 5 basic blocks)
    pub fn method_00(&mut self) {
        // Writes to field at offset +0x0000
    }

    /// Decompiled virtual method #1 @ RVA 0x0014c940 (109 instructions, 21 basic blocks)
    pub fn method_01(&mut self) {
        // Writes to field at offset +0x00a1
    }

}

impl CcClass for ccSpotLight {
    fn class_name(&self) -> &'static str { Self::CLASS_NAME }
    fn vtable_rva(&self) -> u32 { Self::VTABLE_RVA }
    fn virtual_method_count(&self) -> usize { Self::METHOD_COUNT }
    fn base_classes(&self) -> &'static [&'static str] { Self::BASE_CLASSES }
}

/// Decompiled CC2 class: `ccSpr2Anm` (VTable RVA: 0x0087c988, 5 virtual methods, 6 recovered fields)
/// Inherits: ccDispAnm
#[derive(Debug, Clone, Default)]
pub struct ccSpr2Anm {
    pub vtable_rva: u32,
    pub field_0x0000: u64,
    pub field_0x0008: u64,
    pub field_0x0030: u32,
    pub field_0x0038: u8,
    pub field_0x0041: u8,
    pub field_0x0048: u64,
}

impl ccSpr2Anm {
    pub const CLASS_NAME: &'static str = "ccSpr2Anm";
    pub const VTABLE_RVA: u32 = 0x0087c988;
    pub const METHOD_COUNT: usize = 5;
    pub const BASE_CLASSES: &'static [&'static str] = &["ccDispAnm"];
    pub const VIRTUAL_METHODS: &'static [u64] = &[0x00000001801c9650, 0x000000018066ff00, 0x000000018066ff40, 0x000000018066ffa0, 0x0000000180670040];

    pub fn new() -> Self {
        Self {
            vtable_rva: Self::VTABLE_RVA,
            field_0x0000: Default::default(),
            field_0x0008: Default::default(),
            field_0x0030: Default::default(),
            field_0x0038: Default::default(),
            field_0x0041: Default::default(),
            field_0x0048: Default::default(),
        }
    }

    /// Decompiled virtual method #0 @ RVA 0x001c9650 (21 instructions, 3 basic blocks)
    pub fn method_00(&mut self) {
        // Writes to field at offset +0x0000
    }

    /// Decompiled virtual method #1 @ RVA 0x0066ff00 (14 instructions, 1 basic blocks)
    pub fn method_01(&mut self) {
    }

    /// Decompiled virtual method #2 @ RVA 0x0066ff40 (19 instructions, 4 basic blocks)
    pub fn method_02(&mut self) {
    }

    /// Decompiled virtual method #3 @ RVA 0x0066ffa0 (58 instructions, 8 basic blocks)
    pub fn method_03(&mut self) {
        // Writes to field at offset +0x0041
    }

    /// Decompiled virtual method #4 @ RVA 0x00670040 (19 instructions, 3 basic blocks)
    pub fn method_04(&mut self) {
    }

}

impl CcClass for ccSpr2Anm {
    fn class_name(&self) -> &'static str { Self::CLASS_NAME }
    fn vtable_rva(&self) -> u32 { Self::VTABLE_RVA }
    fn virtual_method_count(&self) -> usize { Self::METHOD_COUNT }
    fn base_classes(&self) -> &'static [&'static str] { Self::BASE_CLASSES }
}

/// Decompiled CC2 class: `ccSprObj` (VTable RVA: 0x00881ce0, 12 virtual methods, 4 recovered fields)
#[derive(Debug, Clone, Default)]
pub struct ccSprObj {
    pub vtable_rva: u32,
    pub field_0x0000: u64,
    pub field_0x0010: u32,
    pub field_0x0012: u16,
    pub field_0x0018: u64,
}

impl ccSprObj {
    pub const CLASS_NAME: &'static str = "ccSprObj";
    pub const VTABLE_RVA: u32 = 0x00881ce0;
    pub const METHOD_COUNT: usize = 12;
    pub const BASE_CLASSES: &'static [&'static str] = &[];
    pub const VIRTUAL_METHODS: &'static [u64] = &[0x0000000180232200, 0x0000000180232280, 0x00000001802322b0, 0x00000001802322c0, 0x00000001802322f0, 0x0000000180232360, 0x0000000180232370, 0x0000000180232380, 0x0000000180232390, 0x00000001802323d0, 0x0000000180232410, 0x0000000180232470];

    pub fn new() -> Self {
        Self {
            vtable_rva: Self::VTABLE_RVA,
            field_0x0000: Default::default(),
            field_0x0010: Default::default(),
            field_0x0012: Default::default(),
            field_0x0018: Default::default(),
        }
    }

    /// Decompiled virtual method #0 @ RVA 0x00232200 (15 instructions, 2 basic blocks)
    pub fn method_00(&mut self) {
        // Writes to field at offset +0x0018
    }

    /// Decompiled virtual method #1 @ RVA 0x00232280 (11 instructions, 3 basic blocks)
    pub fn method_01(&mut self) {
    }

    /// Decompiled virtual method #2 @ RVA 0x002322b0 (4 instructions, 1 basic blocks)
    pub fn method_02(&mut self) {
        // Writes to field at offset +0x0010
        // Writes to field at offset +0x0018
    }

    /// Decompiled virtual method #3 @ RVA 0x002322c0 (44 instructions, 11 basic blocks)
    pub fn method_03(&mut self) {
        // Writes to field at offset +0x0012
    }

    /// Decompiled virtual method #4 @ RVA 0x002322f0 (30 instructions, 11 basic blocks)
    pub fn method_04(&mut self) {
        // Writes to field at offset +0x0012
    }

    /// Decompiled virtual method #5 @ RVA 0x00232360 (4 instructions, 1 basic blocks)
    pub fn method_05(&mut self) {
    }

    /// Decompiled virtual method #6 @ RVA 0x00232370 (3 instructions, 1 basic blocks)
    pub fn method_06(&mut self) {
    }

    /// Decompiled virtual method #7 @ RVA 0x00232380 (20 instructions, 2 basic blocks)
    pub fn method_07(&mut self) {
    }

    /// Decompiled virtual method #8 @ RVA 0x00232390 (11 instructions, 1 basic blocks)
    pub fn method_08(&mut self) {
    }

    /// Decompiled virtual method #9 @ RVA 0x002323d0 (11 instructions, 1 basic blocks)
    pub fn method_09(&mut self) {
    }

    /// Decompiled virtual method #10 @ RVA 0x00232410 (18 instructions, 5 basic blocks)
    pub fn method_10(&mut self) {
    }

    /// Decompiled virtual method #11 @ RVA 0x00232470 (9 instructions, 1 basic blocks)
    pub fn method_11(&mut self) {
    }

}

impl CcClass for ccSprObj {
    fn class_name(&self) -> &'static str { Self::CLASS_NAME }
    fn vtable_rva(&self) -> u32 { Self::VTABLE_RVA }
    fn virtual_method_count(&self) -> usize { Self::METHOD_COUNT }
    fn base_classes(&self) -> &'static [&'static str] { Self::BASE_CLASSES }
}

/// Decompiled CC2 class: `ccStateIconList` (VTable RVA: 0x00881728, 16 virtual methods, 12 recovered fields)
/// Inherits: ccCharDispList, ccDispW2L, ccDispBase
#[derive(Debug, Clone, Default)]
pub struct ccStateIconList {
    pub vtable_rva: u32,
    pub field_0x0000: u64,
    pub field_0x0008: u32,
    pub field_0x0018: u64,
    pub field_0x001c: u8,
    pub field_0x0024: u16,
    pub field_0x0028: u32,
    pub field_0x002c: u32,
    pub field_0x0030: u32,
    pub field_0x0034: u32,
    pub field_0x0060: u64,
    pub field_0x0070: u32,
    pub field_0x0074: u32,
}

impl ccStateIconList {
    pub const CLASS_NAME: &'static str = "ccStateIconList";
    pub const VTABLE_RVA: u32 = 0x00881728;
    pub const METHOD_COUNT: usize = 16;
    pub const BASE_CLASSES: &'static [&'static str] = &["ccCharDispList", "ccDispW2L", "ccDispBase"];
    pub const VIRTUAL_METHODS: &'static [u64] = &[0x0000000180232b80, 0x00000001802a52e0, 0x00000001802a5330, 0x00000001802a5620, 0x00000001802a5380, 0x00000001802a5530, 0x00000001801b81f0, 0x00000001801b8200, 0x00000001801b8210, 0x00000001800d3090, 0x00000001800d3090, 0x00000001800d3090, 0x00000001800d3090, 0x00000001800d3090, 0x000000018023f250, 0x00000001802a5730];

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
            field_0x0060: Default::default(),
            field_0x0070: Default::default(),
            field_0x0074: Default::default(),
        }
    }

    /// Decompiled virtual method #0 @ RVA 0x00232b80 (13 instructions, 3 basic blocks)
    pub fn method_00(&mut self) {
        // Writes to field at offset +0x0000
    }

    /// Decompiled virtual method #1 @ RVA 0x002a52e0 (18 instructions, 3 basic blocks)
    pub fn method_01(&mut self) {
        // Writes to field at offset +0x0060
    }

    /// Decompiled virtual method #2 @ RVA 0x002a5330 (19 instructions, 3 basic blocks)
    pub fn method_02(&mut self) {
    }

    /// Decompiled virtual method #3 @ RVA 0x002a5620 (59 instructions, 12 basic blocks)
    pub fn method_03(&mut self) {
        // Writes to field at offset +0x0008
    }

    /// Decompiled virtual method #4 @ RVA 0x002a5380 (102 instructions, 22 basic blocks)
    pub fn method_04(&mut self) {
        // Writes to field at offset +0x0008
        // Writes to field at offset +0x001c
    }

    /// Decompiled virtual method #5 @ RVA 0x002a5530 (50 instructions, 15 basic blocks)
    pub fn method_05(&mut self) {
        // Writes to field at offset +0x0008
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

    /// Decompiled virtual method #14 @ RVA 0x0023f250 (63 instructions, 19 basic blocks)
    pub fn method_14(&mut self) {
    }

    /// Decompiled virtual method #15 @ RVA 0x002a5730 (41 instructions, 8 basic blocks)
    pub fn method_15(&mut self) {
    }

}

impl CcClass for ccStateIconList {
    fn class_name(&self) -> &'static str { Self::CLASS_NAME }
    fn vtable_rva(&self) -> u32 { Self::VTABLE_RVA }
    fn virtual_method_count(&self) -> usize { Self::METHOD_COUNT }
    fn base_classes(&self) -> &'static [&'static str] { Self::BASE_CLASSES }
}

/// Decompiled CC2 class: `ccStatusPanel_Ex` (VTable RVA: 0x008e60b8, 21 virtual methods, 22 recovered fields)
/// Inherits: ccStatusPanelBase, ccDispTarget, ccDispBase
#[derive(Debug, Clone, Default)]
pub struct ccStatusPanel_Ex {
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
    pub field_0x0040: u16,
    pub field_0x0048: u64,
    pub field_0x0050: u64,
    pub field_0x0058: u64,
    pub field_0x0060: u64,
    pub field_0x0070: u32,
    pub field_0x0080: u32,
    pub field_0x0084: u32,
    pub field_0x008c: u16,
}

impl ccStatusPanel_Ex {
    pub const CLASS_NAME: &'static str = "ccStatusPanel_Ex";
    pub const VTABLE_RVA: u32 = 0x008e60b8;
    pub const METHOD_COUNT: usize = 21;
    pub const BASE_CLASSES: &'static [&'static str] = &["ccStatusPanelBase", "ccDispTarget", "ccDispBase"];
    pub const VIRTUAL_METHODS: &'static [u64] = &[0x0000000180232930, 0x00000001802a65a0, 0x000000018029ff50, 0x00000001802a6760, 0x00000001802a0e30, 0x00000001800d3090, 0x00000001802a6510, 0x00000001801b8200, 0x00000001801b8210, 0x00000001802a61f0, 0x00000001800d3090, 0x00000001802a63e0, 0x00000001800d3090, 0x00000001802a6470, 0x00000001802a6ac0, 0x00000001805bb240, 0x0000000180103e80, 0x00000001802a0040, 0x00000001802a6530, 0x00000001802a6560, 0x00000001802a6570];

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
            field_0x0040: Default::default(),
            field_0x0048: Default::default(),
            field_0x0050: Default::default(),
            field_0x0058: Default::default(),
            field_0x0060: Default::default(),
            field_0x0070: Default::default(),
            field_0x0080: Default::default(),
            field_0x0084: Default::default(),
            field_0x008c: Default::default(),
        }
    }

    /// Decompiled virtual method #0 @ RVA 0x00232930 (13 instructions, 3 basic blocks)
    pub fn method_00(&mut self) {
        // Writes to field at offset +0x0000
    }

    /// Decompiled virtual method #1 @ RVA 0x002a65a0 (132 instructions, 24 basic blocks)
    pub fn method_01(&mut self) {
        // Writes to field at offset +0x0008
        // Writes to field at offset +0x0048
        // Writes to field at offset +0x0050
        // Writes to field at offset +0x0070
    }

    /// Decompiled virtual method #2 @ RVA 0x0029ff50 (66 instructions, 11 basic blocks)
    pub fn method_02(&mut self) {
    }

    /// Decompiled virtual method #3 @ RVA 0x002a6760 (124 instructions, 20 basic blocks)
    pub fn method_03(&mut self) {
        // Writes to field at offset +0x0008
        // Writes to field at offset +0x0070
    }

    /// Decompiled virtual method #4 @ RVA 0x002a0e30 (48 instructions, 11 basic blocks)
    pub fn method_04(&mut self) {
        // Writes to field at offset +0x0008
    }

    /// Decompiled virtual method #5 @ RVA 0x000d3090 (1 instructions, 1 basic blocks)
    pub fn method_05(&mut self) {
    }

    /// Decompiled virtual method #6 @ RVA 0x002a6510 (10 instructions, 5 basic blocks)
    pub fn method_06(&mut self) {
        // Writes to field at offset +0x0008
        // Writes to field at offset +0x0084
    }

    /// Decompiled virtual method #7 @ RVA 0x001b8200 (5 instructions, 1 basic blocks)
    pub fn method_07(&mut self) {
    }

    /// Decompiled virtual method #8 @ RVA 0x001b8210 (5 instructions, 1 basic blocks)
    pub fn method_08(&mut self) {
    }

    /// Decompiled virtual method #9 @ RVA 0x002a61f0 (84 instructions, 14 basic blocks)
    pub fn method_09(&mut self) {
    }

    /// Decompiled virtual method #10 @ RVA 0x000d3090 (1 instructions, 1 basic blocks)
    pub fn method_10(&mut self) {
    }

    /// Decompiled virtual method #11 @ RVA 0x002a63e0 (11 instructions, 2 basic blocks)
    pub fn method_11(&mut self) {
        // Writes to field at offset +0x0084
    }

    /// Decompiled virtual method #12 @ RVA 0x000d3090 (1 instructions, 1 basic blocks)
    pub fn method_12(&mut self) {
    }

    /// Decompiled virtual method #13 @ RVA 0x002a6470 (11 instructions, 2 basic blocks)
    pub fn method_13(&mut self) {
        // Writes to field at offset +0x0084
    }

    /// Decompiled virtual method #14 @ RVA 0x002a6ac0 (49 instructions, 7 basic blocks)
    pub fn method_14(&mut self) {
        // Writes to field at offset +0x008c
    }

    /// Decompiled virtual method #15 @ RVA 0x005bb240 (2 instructions, 1 basic blocks)
    pub fn method_15(&mut self) {
    }

    /// Decompiled virtual method #16 @ RVA 0x00103e80 (2 instructions, 1 basic blocks)
    pub fn method_16(&mut self) {
    }

    /// Decompiled virtual method #17 @ RVA 0x002a0040 (3 instructions, 1 basic blocks)
    pub fn method_17(&mut self) {
    }

    /// Decompiled virtual method #18 @ RVA 0x002a6530 (7 instructions, 3 basic blocks)
    pub fn method_18(&mut self) {
    }

    /// Decompiled virtual method #19 @ RVA 0x002a6560 (4 instructions, 2 basic blocks)
    pub fn method_19(&mut self) {
    }

    /// Decompiled virtual method #20 @ RVA 0x002a6570 (8 instructions, 3 basic blocks)
    pub fn method_20(&mut self) {
    }

}

impl CcClass for ccStatusPanel_Ex {
    fn class_name(&self) -> &'static str { Self::CLASS_NAME }
    fn vtable_rva(&self) -> u32 { Self::VTABLE_RVA }
    fn virtual_method_count(&self) -> usize { Self::METHOD_COUNT }
    fn base_classes(&self) -> &'static [&'static str] { Self::BASE_CLASSES }
}

/// Decompiled CC2 class: `ccStatusPanel_M` (VTable RVA: 0x008a4358, 21 virtual methods, 17 recovered fields)
/// Inherits: ccStatusPanelBase, ccDispTarget, ccDispBase
#[derive(Debug, Clone, Default)]
pub struct ccStatusPanel_M {
    pub vtable_rva: u32,
    pub field_0x0000: u64,
    pub field_0x0008: u32,
    pub field_0x0028: u32,
    pub field_0x002c: u32,
    pub field_0x0030: u32,
    pub field_0x0034: u32,
    pub field_0x0048: u64,
    pub field_0x0050: u64,
    pub field_0x0054: u32,
    pub field_0x0058: u32,
    pub field_0x005c: u32,
    pub field_0x0060: u64,
    pub field_0x0080: u32,
    pub field_0x0084: u32,
    pub field_0x008c: u16,
    pub field_0x0090: u64,
    pub field_0x0098: u64,
}

impl ccStatusPanel_M {
    pub const CLASS_NAME: &'static str = "ccStatusPanel_M";
    pub const VTABLE_RVA: u32 = 0x008a4358;
    pub const METHOD_COUNT: usize = 21;
    pub const BASE_CLASSES: &'static [&'static str] = &["ccStatusPanelBase", "ccDispTarget", "ccDispBase"];
    pub const VIRTUAL_METHODS: &'static [u64] = &[0x00000001802a77e0, 0x00000001802a78d0, 0x000000018029ff50, 0x00000001802a7dc0, 0x00000001802a0e30, 0x00000001800d3090, 0x00000001802a6510, 0x00000001801b8200, 0x00000001801b8210, 0x00000001802a61f0, 0x00000001800d3090, 0x00000001802a63e0, 0x00000001800d3090, 0x00000001802a6470, 0x00000001802a8460, 0x00000001802a86f0, 0x0000000180103e90, 0x00000001802a0040, 0x00000001802a6530, 0x00000001802a6560, 0x00000001802a6570];

    pub fn new() -> Self {
        Self {
            vtable_rva: Self::VTABLE_RVA,
            field_0x0000: Default::default(),
            field_0x0008: Default::default(),
            field_0x0028: Default::default(),
            field_0x002c: Default::default(),
            field_0x0030: Default::default(),
            field_0x0034: Default::default(),
            field_0x0048: Default::default(),
            field_0x0050: Default::default(),
            field_0x0054: Default::default(),
            field_0x0058: Default::default(),
            field_0x005c: Default::default(),
            field_0x0060: Default::default(),
            field_0x0080: Default::default(),
            field_0x0084: Default::default(),
            field_0x008c: Default::default(),
            field_0x0090: Default::default(),
            field_0x0098: Default::default(),
        }
    }

    /// Decompiled virtual method #0 @ RVA 0x002a77e0 (33 instructions, 7 basic blocks)
    pub fn method_00(&mut self) {
        // Writes to field at offset +0x0000
    }

    /// Decompiled virtual method #1 @ RVA 0x002a78d0 (120 instructions, 12 basic blocks)
    pub fn method_01(&mut self) {
        // Writes to field at offset +0x0054
        // Writes to field at offset +0x0058
        // Writes to field at offset +0x005c
    }

    /// Decompiled virtual method #2 @ RVA 0x0029ff50 (66 instructions, 11 basic blocks)
    pub fn method_02(&mut self) {
    }

    /// Decompiled virtual method #3 @ RVA 0x002a7dc0 (117 instructions, 12 basic blocks)
    pub fn method_03(&mut self) {
        // Writes to field at offset +0x0008
        // Writes to field at offset +0x0048
    }

    /// Decompiled virtual method #4 @ RVA 0x002a0e30 (48 instructions, 11 basic blocks)
    pub fn method_04(&mut self) {
        // Writes to field at offset +0x0008
    }

    /// Decompiled virtual method #5 @ RVA 0x000d3090 (1 instructions, 1 basic blocks)
    pub fn method_05(&mut self) {
    }

    /// Decompiled virtual method #6 @ RVA 0x002a6510 (10 instructions, 5 basic blocks)
    pub fn method_06(&mut self) {
        // Writes to field at offset +0x0008
        // Writes to field at offset +0x0084
    }

    /// Decompiled virtual method #7 @ RVA 0x001b8200 (5 instructions, 1 basic blocks)
    pub fn method_07(&mut self) {
    }

    /// Decompiled virtual method #8 @ RVA 0x001b8210 (5 instructions, 1 basic blocks)
    pub fn method_08(&mut self) {
    }

    /// Decompiled virtual method #9 @ RVA 0x002a61f0 (84 instructions, 14 basic blocks)
    pub fn method_09(&mut self) {
    }

    /// Decompiled virtual method #10 @ RVA 0x000d3090 (1 instructions, 1 basic blocks)
    pub fn method_10(&mut self) {
    }

    /// Decompiled virtual method #11 @ RVA 0x002a63e0 (11 instructions, 2 basic blocks)
    pub fn method_11(&mut self) {
        // Writes to field at offset +0x0084
    }

    /// Decompiled virtual method #12 @ RVA 0x000d3090 (1 instructions, 1 basic blocks)
    pub fn method_12(&mut self) {
    }

    /// Decompiled virtual method #13 @ RVA 0x002a6470 (11 instructions, 2 basic blocks)
    pub fn method_13(&mut self) {
        // Writes to field at offset +0x0084
    }

    /// Decompiled virtual method #14 @ RVA 0x002a8460 (130 instructions, 12 basic blocks)
    pub fn method_14(&mut self) {
        // Writes to field at offset +0x008c
    }

    /// Decompiled virtual method #15 @ RVA 0x002a86f0 (4 instructions, 2 basic blocks)
    pub fn method_15(&mut self) {
        // Writes to field at offset +0x0090
    }

    /// Decompiled virtual method #16 @ RVA 0x00103e90 (2 instructions, 1 basic blocks)
    pub fn method_16(&mut self) {
    }

    /// Decompiled virtual method #17 @ RVA 0x002a0040 (3 instructions, 1 basic blocks)
    pub fn method_17(&mut self) {
    }

    /// Decompiled virtual method #18 @ RVA 0x002a6530 (7 instructions, 3 basic blocks)
    pub fn method_18(&mut self) {
    }

    /// Decompiled virtual method #19 @ RVA 0x002a6560 (4 instructions, 2 basic blocks)
    pub fn method_19(&mut self) {
    }

    /// Decompiled virtual method #20 @ RVA 0x002a6570 (8 instructions, 3 basic blocks)
    pub fn method_20(&mut self) {
    }

}

impl CcClass for ccStatusPanel_M {
    fn class_name(&self) -> &'static str { Self::CLASS_NAME }
    fn vtable_rva(&self) -> u32 { Self::VTABLE_RVA }
    fn virtual_method_count(&self) -> usize { Self::METHOD_COUNT }
    fn base_classes(&self) -> &'static [&'static str] { Self::BASE_CLASSES }
}

/// Decompiled CC2 class: `ccStatusPanel_S` (VTable RVA: 0x008a4658, 21 virtual methods, 17 recovered fields)
/// Inherits: ccStatusPanelBase, ccDispTarget, ccDispBase
#[derive(Debug, Clone, Default)]
pub struct ccStatusPanel_S {
    pub vtable_rva: u32,
    pub field_0x0000: u32,
    pub field_0x0008: u32,
    pub field_0x001a: u8,
    pub field_0x0028: u32,
    pub field_0x002c: u32,
    pub field_0x0030: u32,
    pub field_0x0034: u32,
    pub field_0x0040: u16,
    pub field_0x0048: u64,
    pub field_0x0050: u64,
    pub field_0x0054: u32,
    pub field_0x0058: u32,
    pub field_0x005c: u32,
    pub field_0x0060: u64,
    pub field_0x0080: u32,
    pub field_0x0084: u32,
    pub field_0x008c: u16,
}

impl ccStatusPanel_S {
    pub const CLASS_NAME: &'static str = "ccStatusPanel_S";
    pub const VTABLE_RVA: u32 = 0x008a4658;
    pub const METHOD_COUNT: usize = 21;
    pub const BASE_CLASSES: &'static [&'static str] = &["ccStatusPanelBase", "ccDispTarget", "ccDispBase"];
    pub const VIRTUAL_METHODS: &'static [u64] = &[0x00000001805bbcf0, 0x00000001802a6bf0, 0x000000018029ff50, 0x00000001802a6f80, 0x00000001802a0e30, 0x00000001800d3090, 0x00000001802a6510, 0x00000001801b8200, 0x00000001801b8210, 0x00000001802a61f0, 0x00000001800d3090, 0x00000001802a63e0, 0x00000001800d3090, 0x00000001802a6470, 0x00000001802a73b0, 0x00000001802a00d0, 0x0000000180103e70, 0x00000001802a0040, 0x00000001802a7640, 0x00000001802a76c0, 0x00000001802a7740];

    pub fn new() -> Self {
        Self {
            vtable_rva: Self::VTABLE_RVA,
            field_0x0000: Default::default(),
            field_0x0008: Default::default(),
            field_0x001a: Default::default(),
            field_0x0028: Default::default(),
            field_0x002c: Default::default(),
            field_0x0030: Default::default(),
            field_0x0034: Default::default(),
            field_0x0040: Default::default(),
            field_0x0048: Default::default(),
            field_0x0050: Default::default(),
            field_0x0054: Default::default(),
            field_0x0058: Default::default(),
            field_0x005c: Default::default(),
            field_0x0060: Default::default(),
            field_0x0080: Default::default(),
            field_0x0084: Default::default(),
            field_0x008c: Default::default(),
        }
    }

    /// Decompiled virtual method #0 @ RVA 0x005bbcf0 (38 instructions, 8 basic blocks)
    pub fn method_00(&mut self) {
        // Writes to field at offset +0x0000
    }

    /// Decompiled virtual method #1 @ RVA 0x002a6bf0 (122 instructions, 12 basic blocks)
    pub fn method_01(&mut self) {
        // Writes to field at offset +0x0054
        // Writes to field at offset +0x0058
        // Writes to field at offset +0x005c
    }

    /// Decompiled virtual method #2 @ RVA 0x0029ff50 (66 instructions, 11 basic blocks)
    pub fn method_02(&mut self) {
    }

    /// Decompiled virtual method #3 @ RVA 0x002a6f80 (125 instructions, 16 basic blocks)
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

    /// Decompiled virtual method #6 @ RVA 0x002a6510 (10 instructions, 5 basic blocks)
    pub fn method_06(&mut self) {
        // Writes to field at offset +0x0008
        // Writes to field at offset +0x0084
    }

    /// Decompiled virtual method #7 @ RVA 0x001b8200 (5 instructions, 1 basic blocks)
    pub fn method_07(&mut self) {
    }

    /// Decompiled virtual method #8 @ RVA 0x001b8210 (5 instructions, 1 basic blocks)
    pub fn method_08(&mut self) {
    }

    /// Decompiled virtual method #9 @ RVA 0x002a61f0 (84 instructions, 14 basic blocks)
    pub fn method_09(&mut self) {
    }

    /// Decompiled virtual method #10 @ RVA 0x000d3090 (1 instructions, 1 basic blocks)
    pub fn method_10(&mut self) {
    }

    /// Decompiled virtual method #11 @ RVA 0x002a63e0 (11 instructions, 2 basic blocks)
    pub fn method_11(&mut self) {
        // Writes to field at offset +0x0084
    }

    /// Decompiled virtual method #12 @ RVA 0x000d3090 (1 instructions, 1 basic blocks)
    pub fn method_12(&mut self) {
    }

    /// Decompiled virtual method #13 @ RVA 0x002a6470 (11 instructions, 2 basic blocks)
    pub fn method_13(&mut self) {
        // Writes to field at offset +0x0084
    }

    /// Decompiled virtual method #14 @ RVA 0x002a73b0 (131 instructions, 14 basic blocks)
    pub fn method_14(&mut self) {
        // Writes to field at offset +0x008c
    }

    /// Decompiled virtual method #15 @ RVA 0x002a00d0 (2 instructions, 1 basic blocks)
    pub fn method_15(&mut self) {
    }

    /// Decompiled virtual method #16 @ RVA 0x00103e70 (2 instructions, 1 basic blocks)
    pub fn method_16(&mut self) {
    }

    /// Decompiled virtual method #17 @ RVA 0x002a0040 (3 instructions, 1 basic blocks)
    pub fn method_17(&mut self) {
    }

    /// Decompiled virtual method #18 @ RVA 0x002a7640 (28 instructions, 3 basic blocks)
    pub fn method_18(&mut self) {
        // Writes to field at offset +0x001a
    }

    /// Decompiled virtual method #19 @ RVA 0x002a76c0 (25 instructions, 5 basic blocks)
    pub fn method_19(&mut self) {
    }

    /// Decompiled virtual method #20 @ RVA 0x002a7740 (26 instructions, 5 basic blocks)
    pub fn method_20(&mut self) {
    }

}

impl CcClass for ccStatusPanel_S {
    fn class_name(&self) -> &'static str { Self::CLASS_NAME }
    fn vtable_rva(&self) -> u32 { Self::VTABLE_RVA }
    fn virtual_method_count(&self) -> usize { Self::METHOD_COUNT }
    fn base_classes(&self) -> &'static [&'static str] { Self::BASE_CLASSES }
}

/// Decompiled CC2 class: `ccStreamCelShadeParam` (VTable RVA: 0x00877248, 2 virtual methods, 3 recovered fields)
/// Inherits: ccDrawEnvCtrl
#[derive(Debug, Clone, Default)]
pub struct ccStreamCelShadeParam {
    pub vtable_rva: u32,
    pub field_0x0000: u64,
    pub field_0x0010: u32,
    pub field_0x0044: f32,
}

impl ccStreamCelShadeParam {
    pub const CLASS_NAME: &'static str = "ccStreamCelShadeParam";
    pub const VTABLE_RVA: u32 = 0x00877248;
    pub const METHOD_COUNT: usize = 2;
    pub const BASE_CLASSES: &'static [&'static str] = &["ccDrawEnvCtrl"];
    pub const VIRTUAL_METHODS: &'static [u64] = &[0x0000000180168a80, 0x0000000180168aa0];

    pub fn new() -> Self {
        Self {
            vtable_rva: Self::VTABLE_RVA,
            field_0x0000: Default::default(),
            field_0x0010: Default::default(),
            field_0x0044: Default::default(),
        }
    }

    /// Decompiled virtual method #0 @ RVA 0x00168a80 (6 instructions, 3 basic blocks)
    pub fn method_00(&mut self) {
    }

    /// Decompiled virtual method #1 @ RVA 0x00168aa0 (18 instructions, 3 basic blocks)
    pub fn method_01(&mut self) {
        // Writes to field at offset +0x0000
    }

}

impl CcClass for ccStreamCelShadeParam {
    fn class_name(&self) -> &'static str { Self::CLASS_NAME }
    fn vtable_rva(&self) -> u32 { Self::VTABLE_RVA }
    fn virtual_method_count(&self) -> usize { Self::METHOD_COUNT }
    fn base_classes(&self) -> &'static [&'static str] { Self::BASE_CLASSES }
}

/// Decompiled CC2 class: `ccStreamFBSBlurParam` (VTable RVA: 0x00877208, 1 virtual methods, 1 recovered fields)
#[derive(Debug, Clone, Default)]
pub struct ccStreamFBSBlurParam {
    pub vtable_rva: u32,
    pub field_0x0000: u64,
}

impl ccStreamFBSBlurParam {
    pub const CLASS_NAME: &'static str = "ccStreamFBSBlurParam";
    pub const VTABLE_RVA: u32 = 0x00877208;
    pub const METHOD_COUNT: usize = 1;
    pub const BASE_CLASSES: &'static [&'static str] = &[];
    pub const VIRTUAL_METHODS: &'static [u64] = &[0x0000000180168960];

    pub fn new() -> Self {
        Self {
            vtable_rva: Self::VTABLE_RVA,
            field_0x0000: Default::default(),
        }
    }

    /// Decompiled virtual method #0 @ RVA 0x00168960 (19 instructions, 3 basic blocks)
    pub fn method_00(&mut self) {
        // Writes to field at offset +0x0000
    }

}

impl CcClass for ccStreamFBSBlurParam {
    fn class_name(&self) -> &'static str { Self::CLASS_NAME }
    fn vtable_rva(&self) -> u32 { Self::VTABLE_RVA }
    fn virtual_method_count(&self) -> usize { Self::METHOD_COUNT }
    fn base_classes(&self) -> &'static [&'static str] { Self::BASE_CLASSES }
}

/// Decompiled CC2 class: `ccStreamOutlineParam` (VTable RVA: 0x00877218, 2 virtual methods, 2 recovered fields)
/// Inherits: ccDrawEnvCtrl
#[derive(Debug, Clone, Default)]
pub struct ccStreamOutlineParam {
    pub vtable_rva: u32,
    pub field_0x0000: u64,
    pub field_0x0010: u32,
}

impl ccStreamOutlineParam {
    pub const CLASS_NAME: &'static str = "ccStreamOutlineParam";
    pub const VTABLE_RVA: u32 = 0x00877218;
    pub const METHOD_COUNT: usize = 2;
    pub const BASE_CLASSES: &'static [&'static str] = &["ccDrawEnvCtrl"];
    pub const VIRTUAL_METHODS: &'static [u64] = &[0x00000001801689b0, 0x00000001801689c0];

    pub fn new() -> Self {
        Self {
            vtable_rva: Self::VTABLE_RVA,
            field_0x0000: Default::default(),
            field_0x0010: Default::default(),
        }
    }

    /// Decompiled virtual method #0 @ RVA 0x001689b0 (3 instructions, 1 basic blocks)
    pub fn method_00(&mut self) {
    }

    /// Decompiled virtual method #1 @ RVA 0x001689c0 (18 instructions, 3 basic blocks)
    pub fn method_01(&mut self) {
        // Writes to field at offset +0x0000
    }

}

impl CcClass for ccStreamOutlineParam {
    fn class_name(&self) -> &'static str { Self::CLASS_NAME }
    fn vtable_rva(&self) -> u32 { Self::VTABLE_RVA }
    fn virtual_method_count(&self) -> usize { Self::METHOD_COUNT }
    fn base_classes(&self) -> &'static [&'static str] { Self::BASE_CLASSES }
}

/// Decompiled CC2 class: `ccStreamToneShadeParam` (VTable RVA: 0x00877230, 2 virtual methods, 3 recovered fields)
/// Inherits: ccDrawEnvCtrl
#[derive(Debug, Clone, Default)]
pub struct ccStreamToneShadeParam {
    pub vtable_rva: u32,
    pub field_0x0000: u64,
    pub field_0x0018: u32,
    pub field_0x0054: f32,
}

impl ccStreamToneShadeParam {
    pub const CLASS_NAME: &'static str = "ccStreamToneShadeParam";
    pub const VTABLE_RVA: u32 = 0x00877230;
    pub const METHOD_COUNT: usize = 2;
    pub const BASE_CLASSES: &'static [&'static str] = &["ccDrawEnvCtrl"];
    pub const VIRTUAL_METHODS: &'static [u64] = &[0x0000000180168a10, 0x0000000180168a30];

    pub fn new() -> Self {
        Self {
            vtable_rva: Self::VTABLE_RVA,
            field_0x0000: Default::default(),
            field_0x0018: Default::default(),
            field_0x0054: Default::default(),
        }
    }

    /// Decompiled virtual method #0 @ RVA 0x00168a10 (6 instructions, 3 basic blocks)
    pub fn method_00(&mut self) {
    }

    /// Decompiled virtual method #1 @ RVA 0x00168a30 (18 instructions, 3 basic blocks)
    pub fn method_01(&mut self) {
        // Writes to field at offset +0x0000
    }

}

impl CcClass for ccStreamToneShadeParam {
    fn class_name(&self) -> &'static str { Self::CLASS_NAME }
    fn vtable_rva(&self) -> u32 { Self::VTABLE_RVA }
    fn virtual_method_count(&self) -> usize { Self::METHOD_COUNT }
    fn base_classes(&self) -> &'static [&'static str] { Self::BASE_CLASSES }
}

/// Decompiled CC2 class: `ccStrgPltLine` (VTable RVA: 0x008ec5e0, 4 virtual methods, 25 recovered fields)
/// Inherits: ccMoveLine
#[derive(Debug, Clone, Default)]
pub struct ccStrgPltLine {
    pub vtable_rva: u32,
    pub field_0x0000: f32,
    pub field_0x0004: f32,
    pub field_0x0008: f32,
    pub field_0x000c: u32,
    pub field_0x0010: u32,
    pub field_0x0014: f32,
    pub field_0x0018: f32,
    pub field_0x001c: f32,
    pub field_0x0020: u32,
    pub field_0x0024: u32,
    pub field_0x0028: f32,
    pub field_0x002c: f32,
    pub field_0x0030: f32,
    pub field_0x0034: u32,
    pub field_0x0038: u32,
    pub field_0x003c: f32,
    pub field_0x0040: f32,
    pub field_0x0044: f32,
    pub field_0x0048: u32,
    pub field_0x004c: u32,
    pub field_0x0050: f32,
    pub field_0x0054: f32,
    pub field_0x0058: f32,
    pub field_0x0110: u64,
    pub field_0x0120: f32,
}

impl ccStrgPltLine {
    pub const CLASS_NAME: &'static str = "ccStrgPltLine";
    pub const VTABLE_RVA: u32 = 0x008ec5e0;
    pub const METHOD_COUNT: usize = 4;
    pub const BASE_CLASSES: &'static [&'static str] = &["ccMoveLine"];
    pub const VIRTUAL_METHODS: &'static [u64] = &[0x00000001806224a0, 0x00000001800d3090, 0x00000001806226e0, 0x0000000180622990];

    pub fn new() -> Self {
        Self {
            vtable_rva: Self::VTABLE_RVA,
            field_0x0000: Default::default(),
            field_0x0004: Default::default(),
            field_0x0008: Default::default(),
            field_0x000c: Default::default(),
            field_0x0010: Default::default(),
            field_0x0014: Default::default(),
            field_0x0018: Default::default(),
            field_0x001c: Default::default(),
            field_0x0020: Default::default(),
            field_0x0024: Default::default(),
            field_0x0028: Default::default(),
            field_0x002c: Default::default(),
            field_0x0030: Default::default(),
            field_0x0034: Default::default(),
            field_0x0038: Default::default(),
            field_0x003c: Default::default(),
            field_0x0040: Default::default(),
            field_0x0044: Default::default(),
            field_0x0048: Default::default(),
            field_0x004c: Default::default(),
            field_0x0050: Default::default(),
            field_0x0054: Default::default(),
            field_0x0058: Default::default(),
            field_0x0110: Default::default(),
            field_0x0120: Default::default(),
        }
    }

    /// Decompiled virtual method #0 @ RVA 0x006224a0 (127 instructions, 33 basic blocks)
    pub fn method_00(&mut self) {
        // Writes to field at offset +0x0000
        // Writes to field at offset +0x0004
        // Writes to field at offset +0x0008
        // Writes to field at offset +0x000c
        // Writes to field at offset +0x0010
        // Writes to field at offset +0x0014
        // Writes to field at offset +0x0018
        // Writes to field at offset +0x001c
        // Writes to field at offset +0x0020
        // Writes to field at offset +0x0024
        // Writes to field at offset +0x0028
        // Writes to field at offset +0x002c
        // Writes to field at offset +0x0030
        // Writes to field at offset +0x0034
        // Writes to field at offset +0x0038
        // Writes to field at offset +0x003c
        // Writes to field at offset +0x0040
        // Writes to field at offset +0x0044
        // Writes to field at offset +0x0048
        // Writes to field at offset +0x004c
        // Writes to field at offset +0x0050
        // Writes to field at offset +0x0054
        // Writes to field at offset +0x0058
    }

    /// Decompiled virtual method #1 @ RVA 0x000d3090 (1 instructions, 1 basic blocks)
    pub fn method_01(&mut self) {
    }

    /// Decompiled virtual method #2 @ RVA 0x006226e0 (104 instructions, 15 basic blocks)
    pub fn method_02(&mut self) {
    }

    /// Decompiled virtual method #3 @ RVA 0x00622990 (55 instructions, 5 basic blocks)
    pub fn method_03(&mut self) {
    }

}

impl CcClass for ccStrgPltLine {
    fn class_name(&self) -> &'static str { Self::CLASS_NAME }
    fn vtable_rva(&self) -> u32 { Self::VTABLE_RVA }
    fn virtual_method_count(&self) -> usize { Self::METHOD_COUNT }
    fn base_classes(&self) -> &'static [&'static str] { Self::BASE_CLASSES }
}

/// Decompiled CC2 class: `ccSystem` (VTable RVA: 0x00877d60, 1 virtual methods, 0 recovered fields)
/// Inherits: ?$SingletonPtr::VccSystem::::::fnd::bl
#[derive(Debug, Clone, Default)]
pub struct ccSystem {
    pub vtable_rva: u32,
}

impl ccSystem {
    pub const CLASS_NAME: &'static str = "ccSystem";
    pub const VTABLE_RVA: u32 = 0x00877d60;
    pub const METHOD_COUNT: usize = 1;
    pub const BASE_CLASSES: &'static [&'static str] = &["?$SingletonPtr::VccSystem::::::fnd::bl"];
    pub const VIRTUAL_METHODS: &'static [u64] = &[0x00000001801978b0];

    pub fn new() -> Self {
        Self {
            vtable_rva: Self::VTABLE_RVA,
        }
    }

    /// Decompiled virtual method #0 @ RVA 0x001978b0 (25 instructions, 3 basic blocks)
    pub fn method_00(&mut self) {
    }

}

impl CcClass for ccSystem {
    fn class_name(&self) -> &'static str { Self::CLASS_NAME }
    fn vtable_rva(&self) -> u32 { Self::VTABLE_RVA }
    fn virtual_method_count(&self) -> usize { Self::METHOD_COUNT }
    fn base_classes(&self) -> &'static [&'static str] { Self::BASE_CLASSES }
}

/// Decompiled CC2 class: `ccTargetCurLine` (VTable RVA: 0x00881c40, 4 virtual methods, 12 recovered fields)
/// Inherits: ccMoveLine
#[derive(Debug, Clone, Default)]
pub struct ccTargetCurLine {
    pub vtable_rva: u32,
    pub field_0x0000: u32,
    pub field_0x0004: u32,
    pub field_0x0008: u32,
    pub field_0x0010: f32,
    pub field_0x001c: f32,
    pub field_0x0020: f32,
    pub field_0x0038: u64,
    pub field_0x0150: u8,
    pub field_0x015c: u32,
    pub field_0x0160: u8,
    pub field_0x0165: u8,
    pub field_0x0168: u64,
}

impl ccTargetCurLine {
    pub const CLASS_NAME: &'static str = "ccTargetCurLine";
    pub const VTABLE_RVA: u32 = 0x00881c40;
    pub const METHOD_COUNT: usize = 4;
    pub const BASE_CLASSES: &'static [&'static str] = &["ccMoveLine"];
    pub const VIRTUAL_METHODS: &'static [u64] = &[0x00000001800d3090, 0x00000001800d3090, 0x00000001802a25e0, 0x00000001800d3090];

    pub fn new() -> Self {
        Self {
            vtable_rva: Self::VTABLE_RVA,
            field_0x0000: Default::default(),
            field_0x0004: Default::default(),
            field_0x0008: Default::default(),
            field_0x0010: Default::default(),
            field_0x001c: Default::default(),
            field_0x0020: Default::default(),
            field_0x0038: Default::default(),
            field_0x0150: Default::default(),
            field_0x015c: Default::default(),
            field_0x0160: Default::default(),
            field_0x0165: Default::default(),
            field_0x0168: Default::default(),
        }
    }

    /// Decompiled virtual method #0 @ RVA 0x000d3090 (1 instructions, 1 basic blocks)
    pub fn method_00(&mut self) {
    }

    /// Decompiled virtual method #1 @ RVA 0x000d3090 (1 instructions, 1 basic blocks)
    pub fn method_01(&mut self) {
    }

    /// Decompiled virtual method #2 @ RVA 0x002a25e0 (112 instructions, 15 basic blocks)
    pub fn method_02(&mut self) {
        // Writes to field at offset +0x0150
        // Writes to field at offset +0x015c
        // Writes to field at offset +0x0160
        // Writes to field at offset +0x0165
        // Writes to field at offset +0x0168
    }

    /// Decompiled virtual method #3 @ RVA 0x000d3090 (1 instructions, 1 basic blocks)
    pub fn method_03(&mut self) {
    }

}

impl CcClass for ccTargetCurLine {
    fn class_name(&self) -> &'static str { Self::CLASS_NAME }
    fn vtable_rva(&self) -> u32 { Self::VTABLE_RVA }
    fn virtual_method_count(&self) -> usize { Self::METHOD_COUNT }
    fn base_classes(&self) -> &'static [&'static str] { Self::BASE_CLASSES }
}

/// Decompiled CC2 class: `ccTargetCursor` (VTable RVA: 0x008a4478, 14 virtual methods, 22 recovered fields)
/// Inherits: ccDispW2L, ccDispBase
#[derive(Debug, Clone, Default)]
pub struct ccTargetCursor {
    pub vtable_rva: u32,
    pub field_0x0000: u64,
    pub field_0x0008: u32,
    pub field_0x0010: f32,
    pub field_0x0014: f32,
    pub field_0x0018: f32,
    pub field_0x001c: u32,
    pub field_0x0020: u32,
    pub field_0x0028: u32,
    pub field_0x002c: u32,
    pub field_0x0030: u32,
    pub field_0x0034: u32,
    pub field_0x0040: f32,
    pub field_0x0044: f32,
    pub field_0x0048: f32,
    pub field_0x004c: f32,
    pub field_0x0068: u64,
    pub field_0x0090: u32,
    pub field_0x0094: u32,
    pub field_0x0098: u32,
    pub field_0x009c: u32,
    pub field_0x00a0: f32,
    pub field_0x00a4: f32,
}

impl ccTargetCursor {
    pub const CLASS_NAME: &'static str = "ccTargetCursor";
    pub const VTABLE_RVA: u32 = 0x008a4478;
    pub const METHOD_COUNT: usize = 14;
    pub const BASE_CLASSES: &'static [&'static str] = &["ccDispW2L", "ccDispBase"];
    pub const VIRTUAL_METHODS: &'static [u64] = &[0x00000001802361c0, 0x00000001802a2ba0, 0x00000001802a31b0, 0x00000001802a3490, 0x00000001802a0e30, 0x00000001800d3090, 0x00000001801b81f0, 0x00000001801b8200, 0x00000001801b8210, 0x00000001800d3090, 0x00000001802a3360, 0x00000001802a3250, 0x00000001800d3090, 0x00000001802a32e0];

    pub fn new() -> Self {
        Self {
            vtable_rva: Self::VTABLE_RVA,
            field_0x0000: Default::default(),
            field_0x0008: Default::default(),
            field_0x0010: Default::default(),
            field_0x0014: Default::default(),
            field_0x0018: Default::default(),
            field_0x001c: Default::default(),
            field_0x0020: Default::default(),
            field_0x0028: Default::default(),
            field_0x002c: Default::default(),
            field_0x0030: Default::default(),
            field_0x0034: Default::default(),
            field_0x0040: Default::default(),
            field_0x0044: Default::default(),
            field_0x0048: Default::default(),
            field_0x004c: Default::default(),
            field_0x0068: Default::default(),
            field_0x0090: Default::default(),
            field_0x0094: Default::default(),
            field_0x0098: Default::default(),
            field_0x009c: Default::default(),
            field_0x00a0: Default::default(),
            field_0x00a4: Default::default(),
        }
    }

    /// Decompiled virtual method #0 @ RVA 0x002361c0 (13 instructions, 3 basic blocks)
    pub fn method_00(&mut self) {
        // Writes to field at offset +0x0000
    }

    /// Decompiled virtual method #1 @ RVA 0x002a2ba0 (124 instructions, 24 basic blocks)
    pub fn method_01(&mut self) {
    }

    /// Decompiled virtual method #2 @ RVA 0x002a31b0 (46 instructions, 7 basic blocks)
    pub fn method_02(&mut self) {
    }

    /// Decompiled virtual method #3 @ RVA 0x002a3490 (91 instructions, 3 basic blocks)
    pub fn method_03(&mut self) {
        // Writes to field at offset +0x0008
        // Writes to field at offset +0x0010
        // Writes to field at offset +0x0014
        // Writes to field at offset +0x0018
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

    /// Decompiled virtual method #10 @ RVA 0x002a3360 (99 instructions, 6 basic blocks)
    pub fn method_10(&mut self) {
        // Writes to field at offset +0x0008
        // Writes to field at offset +0x0040
        // Writes to field at offset +0x0044
        // Writes to field at offset +0x0048
        // Writes to field at offset +0x004c
        // Writes to field at offset +0x00a0
        // Writes to field at offset +0x00a0
    }

    /// Decompiled virtual method #11 @ RVA 0x002a3250 (28 instructions, 5 basic blocks)
    pub fn method_11(&mut self) {
        // Writes to field at offset +0x001c
        // Writes to field at offset +0x001c
    }

    /// Decompiled virtual method #12 @ RVA 0x000d3090 (1 instructions, 1 basic blocks)
    pub fn method_12(&mut self) {
    }

    /// Decompiled virtual method #13 @ RVA 0x002a32e0 (29 instructions, 6 basic blocks)
    pub fn method_13(&mut self) {
        // Writes to field at offset +0x001c
        // Writes to field at offset +0x001c
    }

}

impl CcClass for ccTargetCursor {
    fn class_name(&self) -> &'static str { Self::CLASS_NAME }
    fn vtable_rva(&self) -> u32 { Self::VTABLE_RVA }
    fn virtual_method_count(&self) -> usize { Self::METHOD_COUNT }
    fn base_classes(&self) -> &'static [&'static str] { Self::BASE_CLASSES }
}

/// Decompiled CC2 class: `ccTargetPanel` (VTable RVA: 0x008a4230, 14 virtual methods, 19 recovered fields)
/// Inherits: ccDispBase
#[derive(Debug, Clone, Default)]
pub struct ccTargetPanel {
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
    pub field_0x0038: u32,
    pub field_0x0048: u64,
    pub field_0x0058: u64,
    pub field_0x0074: f32,
    pub field_0x0078: u32,
    pub field_0x0180: u8,
    pub field_0x018c: u32,
    pub field_0x01b7: u8,
}

impl ccTargetPanel {
    pub const CLASS_NAME: &'static str = "ccTargetPanel";
    pub const VTABLE_RVA: u32 = 0x008a4230;
    pub const METHOD_COUNT: usize = 14;
    pub const BASE_CLASSES: &'static [&'static str] = &["ccDispBase"];
    pub const VIRTUAL_METHODS: &'static [u64] = &[0x00000001802aa7d0, 0x00000001802aa800, 0x00000001802aaa00, 0x00000001802ab020, 0x00000001802a0e30, 0x00000001800d3090, 0x00000001802ac3a0, 0x00000001801b8200, 0x00000001801b8210, 0x00000001800d3090, 0x00000001802aacb0, 0x00000001802aab30, 0x00000001800d3090, 0x00000001802aabf0];

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
            field_0x0048: Default::default(),
            field_0x0058: Default::default(),
            field_0x0074: Default::default(),
            field_0x0078: Default::default(),
            field_0x0180: Default::default(),
            field_0x018c: Default::default(),
            field_0x01b7: Default::default(),
        }
    }

    /// Decompiled virtual method #0 @ RVA 0x002aa7d0 (13 instructions, 3 basic blocks)
    pub fn method_00(&mut self) {
        // Writes to field at offset +0x0000
    }

    /// Decompiled virtual method #1 @ RVA 0x002aa800 (129 instructions, 12 basic blocks)
    pub fn method_01(&mut self) {
    }

    /// Decompiled virtual method #2 @ RVA 0x002aaa00 (86 instructions, 15 basic blocks)
    pub fn method_02(&mut self) {
    }

    /// Decompiled virtual method #3 @ RVA 0x002ab020 (106 instructions, 12 basic blocks)
    pub fn method_03(&mut self) {
        // Writes to field at offset +0x0000
        // Writes to field at offset +0x0008
        // Writes to field at offset +0x0074
        // Writes to field at offset +0x0078
        // Writes to field at offset +0x018c
        // Writes to field at offset +0x01b7
    }

    /// Decompiled virtual method #4 @ RVA 0x002a0e30 (48 instructions, 11 basic blocks)
    pub fn method_04(&mut self) {
        // Writes to field at offset +0x0008
    }

    /// Decompiled virtual method #5 @ RVA 0x000d3090 (1 instructions, 1 basic blocks)
    pub fn method_05(&mut self) {
    }

    /// Decompiled virtual method #6 @ RVA 0x002ac3a0 (20 instructions, 8 basic blocks)
    pub fn method_06(&mut self) {
        // Writes to field at offset +0x0008
        // Writes to field at offset +0x0038
        // Writes to field at offset +0x0048
        // Writes to field at offset +0x0180
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

    /// Decompiled virtual method #10 @ RVA 0x002aacb0 (101 instructions, 30 basic blocks)
    pub fn method_10(&mut self) {
    }

    /// Decompiled virtual method #11 @ RVA 0x002aab30 (45 instructions, 10 basic blocks)
    pub fn method_11(&mut self) {
        // Writes to field at offset +0x001c
        // Writes to field at offset +0x001c
    }

    /// Decompiled virtual method #12 @ RVA 0x000d3090 (1 instructions, 1 basic blocks)
    pub fn method_12(&mut self) {
    }

    /// Decompiled virtual method #13 @ RVA 0x002aabf0 (45 instructions, 10 basic blocks)
    pub fn method_13(&mut self) {
        // Writes to field at offset +0x001c
        // Writes to field at offset +0x001c
    }

}

impl CcClass for ccTargetPanel {
    fn class_name(&self) -> &'static str { Self::CLASS_NAME }
    fn vtable_rva(&self) -> u32 { Self::VTABLE_RVA }
    fn virtual_method_count(&self) -> usize { Self::METHOD_COUNT }
    fn base_classes(&self) -> &'static [&'static str] { Self::BASE_CLASSES }
}

/// Decompiled CC2 class: `ccTeamLeader` (VTable RVA: 0x00881200, 16 virtual methods, 16 recovered fields)
/// Inherits: ccStateIconBase, ccDispBase
#[derive(Debug, Clone, Default)]
pub struct ccTeamLeader {
    pub vtable_rva: u32,
    pub field_0x0000: u64,
    pub field_0x0008: u32,
    pub field_0x000c: u32,
    pub field_0x0010: u64,
    pub field_0x0014: f32,
    pub field_0x0018: f32,
    pub field_0x001c: u32,
    pub field_0x0028: u32,
    pub field_0x002c: u32,
    pub field_0x0030: u32,
    pub field_0x0034: u32,
    pub field_0x0038: u64,
    pub field_0x0040: u64,
    pub field_0x0048: u32,
    pub field_0x004c: u8,
    pub field_0x0050: u32,
}

impl ccTeamLeader {
    pub const CLASS_NAME: &'static str = "ccTeamLeader";
    pub const VTABLE_RVA: u32 = 0x00881200;
    pub const METHOD_COUNT: usize = 16;
    pub const BASE_CLASSES: &'static [&'static str] = &["ccStateIconBase", "ccDispBase"];
    pub const VIRTUAL_METHODS: &'static [u64] = &[0x000000018023ec10, 0x000000018023ec40, 0x000000018023e560, 0x000000018023ed20, 0x00000001802a0e30, 0x00000001800d3090, 0x00000001801b81f0, 0x00000001801b8200, 0x00000001801b8210, 0x00000001800d3090, 0x00000001800d3090, 0x000000018023ee90, 0x000000018023eee0, 0x000000018023ef50, 0x00000001801b27f0, 0x000000018023ee40];

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
            field_0x0048: Default::default(),
            field_0x004c: Default::default(),
            field_0x0050: Default::default(),
        }
    }

    /// Decompiled virtual method #0 @ RVA 0x0023ec10 (13 instructions, 3 basic blocks)
    pub fn method_00(&mut self) {
        // Writes to field at offset +0x0000
    }

    /// Decompiled virtual method #1 @ RVA 0x0023ec40 (37 instructions, 5 basic blocks)
    pub fn method_01(&mut self) {
    }

    /// Decompiled virtual method #2 @ RVA 0x0023e560 (16 instructions, 5 basic blocks)
    pub fn method_02(&mut self) {
    }

    /// Decompiled virtual method #3 @ RVA 0x0023ed20 (66 instructions, 7 basic blocks)
    pub fn method_03(&mut self) {
        // Writes to field at offset +0x0008
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

    /// Decompiled virtual method #11 @ RVA 0x0023ee90 (8 instructions, 2 basic blocks)
    pub fn method_11(&mut self) {
        // Writes to field at offset +0x0008
        // Writes to field at offset +0x001c
        // Writes to field at offset +0x001c
    }

    /// Decompiled virtual method #12 @ RVA 0x0023eee0 (13 instructions, 4 basic blocks)
    pub fn method_12(&mut self) {
        // Writes to field at offset +0x0048
        // Writes to field at offset +0x0048
        // Writes to field at offset +0x004c
    }

    /// Decompiled virtual method #13 @ RVA 0x0023ef50 (8 instructions, 2 basic blocks)
    pub fn method_13(&mut self) {
        // Writes to field at offset +0x0008
        // Writes to field at offset +0x001c
        // Writes to field at offset +0x001c
    }

    /// Decompiled virtual method #14 @ RVA 0x001b27f0 (2 instructions, 1 basic blocks)
    pub fn method_14(&mut self) {
    }

    /// Decompiled virtual method #15 @ RVA 0x0023ee40 (23 instructions, 5 basic blocks)
    pub fn method_15(&mut self) {
        // Writes to field at offset +0x0008
    }

}

impl CcClass for ccTeamLeader {
    fn class_name(&self) -> &'static str { Self::CLASS_NAME }
    fn vtable_rva(&self) -> u32 { Self::VTABLE_RVA }
    fn virtual_method_count(&self) -> usize { Self::METHOD_COUNT }
    fn base_classes(&self) -> &'static [&'static str] { Self::BASE_CLASSES }
}

/// Decompiled CC2 class: `ccTensionGem` (VTable RVA: 0x00897e80, 14 virtual methods, 17 recovered fields)
/// Inherits: ccDispSpline, ccDispBase
#[derive(Debug, Clone, Default)]
pub struct ccTensionGem {
    pub vtable_rva: u32,
    pub field_0x0000: u64,
    pub field_0x0008: u32,
    pub field_0x000c: u32,
    pub field_0x0010: u32,
    pub field_0x0014: u32,
    pub field_0x0018: f32,
    pub field_0x0028: u32,
    pub field_0x002c: u32,
    pub field_0x0030: u32,
    pub field_0x0034: u32,
    pub field_0x007c: u8,
    pub field_0x0088: u64,
    pub field_0x0090: f32,
    pub field_0x0094: f32,
    pub field_0x00a0: u32,
    pub field_0x00a6: u8,
    pub field_0x00a8: u64,
}

impl ccTensionGem {
    pub const CLASS_NAME: &'static str = "ccTensionGem";
    pub const VTABLE_RVA: u32 = 0x00897e80;
    pub const METHOD_COUNT: usize = 14;
    pub const BASE_CLASSES: &'static [&'static str] = &["ccDispSpline", "ccDispBase"];
    pub const VIRTUAL_METHODS: &'static [u64] = &[0x0000000180293c90, 0x0000000180293cd0, 0x0000000180293dd0, 0x0000000180293f20, 0x00000001802a0e30, 0x00000001800d3090, 0x00000001801b81f0, 0x00000001801b8200, 0x00000001801b8210, 0x0000000180293e40, 0x0000000180293ed0, 0x0000000180293b60, 0x00000001800d3090, 0x0000000180293be0];

    pub fn new() -> Self {
        Self {
            vtable_rva: Self::VTABLE_RVA,
            field_0x0000: Default::default(),
            field_0x0008: Default::default(),
            field_0x000c: Default::default(),
            field_0x0010: Default::default(),
            field_0x0014: Default::default(),
            field_0x0018: Default::default(),
            field_0x0028: Default::default(),
            field_0x002c: Default::default(),
            field_0x0030: Default::default(),
            field_0x0034: Default::default(),
            field_0x007c: Default::default(),
            field_0x0088: Default::default(),
            field_0x0090: Default::default(),
            field_0x0094: Default::default(),
            field_0x00a0: Default::default(),
            field_0x00a6: Default::default(),
            field_0x00a8: Default::default(),
        }
    }

    /// Decompiled virtual method #0 @ RVA 0x00293c90 (18 instructions, 3 basic blocks)
    pub fn method_00(&mut self) {
        // Writes to field at offset +0x0000
    }

    /// Decompiled virtual method #1 @ RVA 0x00293cd0 (53 instructions, 6 basic blocks)
    pub fn method_01(&mut self) {
        // Writes to field at offset +0x0014
        // Writes to field at offset +0x0088
    }

    /// Decompiled virtual method #2 @ RVA 0x00293dd0 (26 instructions, 7 basic blocks)
    pub fn method_02(&mut self) {
        // Writes to field at offset +0x0000
    }

    /// Decompiled virtual method #3 @ RVA 0x00293f20 (13 instructions, 2 basic blocks)
    pub fn method_03(&mut self) {
        // Writes to field at offset +0x0008
        // Writes to field at offset +0x00a0
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

    /// Decompiled virtual method #9 @ RVA 0x00293e40 (32 instructions, 6 basic blocks)
    pub fn method_09(&mut self) {
    }

    /// Decompiled virtual method #10 @ RVA 0x00293ed0 (42 instructions, 5 basic blocks)
    pub fn method_10(&mut self) {
        // Writes to field at offset +0x0008
        // Writes to field at offset +0x007c
        // Writes to field at offset +0x00a0
    }

    /// Decompiled virtual method #11 @ RVA 0x00293b60 (23 instructions, 2 basic blocks)
    pub fn method_11(&mut self) {
        // Writes to field at offset +0x0090
        // Writes to field at offset +0x0090
    }

    /// Decompiled virtual method #12 @ RVA 0x000d3090 (1 instructions, 1 basic blocks)
    pub fn method_12(&mut self) {
    }

    /// Decompiled virtual method #13 @ RVA 0x00293be0 (30 instructions, 6 basic blocks)
    pub fn method_13(&mut self) {
        // Writes to field at offset +0x0090
        // Writes to field at offset +0x0090
        // Writes to field at offset +0x0094
        // Writes to field at offset +0x00a6
    }

}

impl CcClass for ccTensionGem {
    fn class_name(&self) -> &'static str { Self::CLASS_NAME }
    fn vtable_rva(&self) -> u32 { Self::VTABLE_RVA }
    fn virtual_method_count(&self) -> usize { Self::METHOD_COUNT }
    fn base_classes(&self) -> &'static [&'static str] { Self::BASE_CLASSES }
}

/// Decompiled CC2 class: `ccTexChunk` (VTable RVA: 0x00876ed0, 3 virtual methods, 17 recovered fields)
/// Inherits: ccChunkBasis
#[derive(Debug, Clone, Default)]
pub struct ccTexChunk {
    pub vtable_rva: u32,
    pub field_0x0008: u64,
    pub field_0x0018: u64,
    pub field_0x0020: u64,
    pub field_0x0028: u64,
    pub field_0x0030: u16,
    pub field_0x0032: u16,
    pub field_0x0034: u16,
    pub field_0x0036: u8,
    pub field_0x0037: u8,
    pub field_0x0038: u8,
    pub field_0x0039: u8,
    pub field_0x003a: u8,
    pub field_0x003d: u8,
    pub field_0x0040: u64,
    pub field_0x0048: u8,
    pub field_0x0049: u8,
    pub field_0x00d8: u32,
}

impl ccTexChunk {
    pub const CLASS_NAME: &'static str = "ccTexChunk";
    pub const VTABLE_RVA: u32 = 0x00876ed0;
    pub const METHOD_COUNT: usize = 3;
    pub const BASE_CLASSES: &'static [&'static str] = &["ccChunkBasis"];
    pub const VIRTUAL_METHODS: &'static [u64] = &[0x00000001801584e0, 0x000000018019adc0, 0x00000001800d3090];

    pub fn new() -> Self {
        Self {
            vtable_rva: Self::VTABLE_RVA,
            field_0x0008: Default::default(),
            field_0x0018: Default::default(),
            field_0x0020: Default::default(),
            field_0x0028: Default::default(),
            field_0x0030: Default::default(),
            field_0x0032: Default::default(),
            field_0x0034: Default::default(),
            field_0x0036: Default::default(),
            field_0x0037: Default::default(),
            field_0x0038: Default::default(),
            field_0x0039: Default::default(),
            field_0x003a: Default::default(),
            field_0x003d: Default::default(),
            field_0x0040: Default::default(),
            field_0x0048: Default::default(),
            field_0x0049: Default::default(),
            field_0x00d8: Default::default(),
        }
    }

    /// Decompiled virtual method #0 @ RVA 0x001584e0 (16 instructions, 3 basic blocks)
    pub fn method_00(&mut self) {
    }

    /// Decompiled virtual method #1 @ RVA 0x0019adc0 (34 instructions, 1 basic blocks)
    pub fn method_01(&mut self) {
        // Writes to field at offset +0x0008
        // Writes to field at offset +0x0018
        // Writes to field at offset +0x0020
        // Writes to field at offset +0x0028
        // Writes to field at offset +0x0030
        // Writes to field at offset +0x0032
        // Writes to field at offset +0x0034
        // Writes to field at offset +0x0036
        // Writes to field at offset +0x0037
        // Writes to field at offset +0x0038
        // Writes to field at offset +0x0039
        // Writes to field at offset +0x003a
        // Writes to field at offset +0x003d
        // Writes to field at offset +0x0040
        // Writes to field at offset +0x0048
        // Writes to field at offset +0x0049
        // Writes to field at offset +0x00d8
    }

    /// Decompiled virtual method #2 @ RVA 0x000d3090 (1 instructions, 1 basic blocks)
    pub fn method_02(&mut self) {
    }

}

impl CcClass for ccTexChunk {
    fn class_name(&self) -> &'static str { Self::CLASS_NAME }
    fn vtable_rva(&self) -> u32 { Self::VTABLE_RVA }
    fn virtual_method_count(&self) -> usize { Self::METHOD_COUNT }
    fn base_classes(&self) -> &'static [&'static str] { Self::BASE_CLASSES }
}

/// Decompiled CC2 class: `ccWebBase` (VTable RVA: 0x008c4870, 11 virtual methods, 14 recovered fields)
#[derive(Debug, Clone, Default)]
pub struct ccWebBase {
    pub vtable_rva: u32,
    pub field_0x0000: u64,
    pub field_0x0040: u32,
    pub field_0x0044: f32,
    pub field_0x0048: f32,
    pub field_0x0054: f32,
    pub field_0x0058: f32,
    pub field_0x0060: f32,
    pub field_0x0068: u64,
    pub field_0x0070: u64,
    pub field_0x0088: u64,
    pub field_0x0090: u8,
    pub field_0x0360: u32,
    pub field_0x0368: u8,
    pub field_0x0398: [f32; 4],
}

impl ccWebBase {
    pub const CLASS_NAME: &'static str = "ccWebBase";
    pub const VTABLE_RVA: u32 = 0x008c4870;
    pub const METHOD_COUNT: usize = 11;
    pub const BASE_CLASSES: &'static [&'static str] = &[];
    pub const VIRTUAL_METHODS: &'static [u64] = &[0x00000001800d3090, 0x0000000180103e90, 0x00000001800d3090, 0x00000001800d3090, 0x00000001805657e0, 0x00000001800d3090, 0x00000001800d3090, 0x00000001800fe200, 0x00000001806687a0, 0x0000000180668b10, 0x0000000180668d80];

    pub fn new() -> Self {
        Self {
            vtable_rva: Self::VTABLE_RVA,
            field_0x0000: Default::default(),
            field_0x0040: Default::default(),
            field_0x0044: Default::default(),
            field_0x0048: Default::default(),
            field_0x0054: Default::default(),
            field_0x0058: Default::default(),
            field_0x0060: Default::default(),
            field_0x0068: Default::default(),
            field_0x0070: Default::default(),
            field_0x0088: Default::default(),
            field_0x0090: Default::default(),
            field_0x0360: Default::default(),
            field_0x0368: Default::default(),
            field_0x0398: Default::default(),
        }
    }

    /// Decompiled virtual method #0 @ RVA 0x000d3090 (1 instructions, 1 basic blocks)
    pub fn method_00(&mut self) {
    }

    /// Decompiled virtual method #1 @ RVA 0x00103e90 (2 instructions, 1 basic blocks)
    pub fn method_01(&mut self) {
    }

    /// Decompiled virtual method #2 @ RVA 0x000d3090 (1 instructions, 1 basic blocks)
    pub fn method_02(&mut self) {
    }

    /// Decompiled virtual method #3 @ RVA 0x000d3090 (1 instructions, 1 basic blocks)
    pub fn method_03(&mut self) {
    }

    /// Decompiled virtual method #4 @ RVA 0x005657e0 (16 instructions, 3 basic blocks)
    pub fn method_04(&mut self) {
    }

    /// Decompiled virtual method #5 @ RVA 0x000d3090 (1 instructions, 1 basic blocks)
    pub fn method_05(&mut self) {
    }

    /// Decompiled virtual method #6 @ RVA 0x000d3090 (1 instructions, 1 basic blocks)
    pub fn method_06(&mut self) {
    }

    /// Decompiled virtual method #7 @ RVA 0x000fe200 (2 instructions, 1 basic blocks)
    pub fn method_07(&mut self) {
    }

    /// Decompiled virtual method #8 @ RVA 0x006687a0 (123 instructions, 17 basic blocks)
    pub fn method_08(&mut self) {
        // Writes to field at offset +0x0048
        // Writes to field at offset +0x0090
    }

    /// Decompiled virtual method #9 @ RVA 0x00668b10 (114 instructions, 9 basic blocks)
    pub fn method_09(&mut self) {
        // Writes to field at offset +0x0360
        // Writes to field at offset +0x0368
        // Writes to field at offset +0x0398
    }

    /// Decompiled virtual method #10 @ RVA 0x00668d80 (99 instructions, 5 basic blocks)
    pub fn method_10(&mut self) {
        // Writes to field at offset +0x0360
        // Writes to field at offset +0x0368
        // Writes to field at offset +0x0398
    }

}

impl CcClass for ccWebBase {
    fn class_name(&self) -> &'static str { Self::CLASS_NAME }
    fn vtable_rva(&self) -> u32 { Self::VTABLE_RVA }
    fn virtual_method_count(&self) -> usize { Self::METHOD_COUNT }
    fn base_classes(&self) -> &'static [&'static str] { Self::BASE_CLASSES }
}

/// Decompiled CC2 class: `ccWebButtonBase` (VTable RVA: 0x008fb378, 9 virtual methods, 12 recovered fields)
#[derive(Debug, Clone, Default)]
pub struct ccWebButtonBase {
    pub vtable_rva: u32,
    pub field_0x0000: u64,
    pub field_0x0008: u64,
    pub field_0x0040: u32,
    pub field_0x0044: f32,
    pub field_0x0048: u32,
    pub field_0x00a4: u32,
    pub field_0x00a8: u32,
    pub field_0x00c4: u32,
    pub field_0x00cc: u32,
    pub field_0x00d0: u32,
    pub field_0x00d4: u32,
    pub field_0x00d8: u8,
}

impl ccWebButtonBase {
    pub const CLASS_NAME: &'static str = "ccWebButtonBase";
    pub const VTABLE_RVA: u32 = 0x008fb378;
    pub const METHOD_COUNT: usize = 9;
    pub const BASE_CLASSES: &'static [&'static str] = &[];
    pub const VIRTUAL_METHODS: &'static [u64] = &[0x00000001800d3090, 0x00000001800d3090, 0x00000001800d3090, 0x0000000180667220, 0x0000000180667f40, 0x0000000180667f60, 0x0000000180667f20, 0x0000000180667cd0, 0x0000000180667ea0];

    pub fn new() -> Self {
        Self {
            vtable_rva: Self::VTABLE_RVA,
            field_0x0000: Default::default(),
            field_0x0008: Default::default(),
            field_0x0040: Default::default(),
            field_0x0044: Default::default(),
            field_0x0048: Default::default(),
            field_0x00a4: Default::default(),
            field_0x00a8: Default::default(),
            field_0x00c4: Default::default(),
            field_0x00cc: Default::default(),
            field_0x00d0: Default::default(),
            field_0x00d4: Default::default(),
            field_0x00d8: Default::default(),
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

    /// Decompiled virtual method #3 @ RVA 0x00667220 (31 instructions, 7 basic blocks)
    pub fn method_03(&mut self) {
        // Writes to field at offset +0x0000
    }

    /// Decompiled virtual method #4 @ RVA 0x00667f40 (3 instructions, 1 basic blocks)
    pub fn method_04(&mut self) {
        // Writes to field at offset +0x00c4
        // Writes to field at offset +0x00d8
    }

    /// Decompiled virtual method #5 @ RVA 0x00667f60 (2 instructions, 1 basic blocks)
    pub fn method_05(&mut self) {
        // Writes to field at offset +0x00d8
    }

    /// Decompiled virtual method #6 @ RVA 0x00667f20 (5 instructions, 1 basic blocks)
    pub fn method_06(&mut self) {
    }

    /// Decompiled virtual method #7 @ RVA 0x00667cd0 (118 instructions, 18 basic blocks)
    pub fn method_07(&mut self) {
        // Writes to field at offset +0x0040
        // Writes to field at offset +0x0044
        // Writes to field at offset +0x0048
        // Writes to field at offset +0x00c4
        // Writes to field at offset +0x00c4
        // Writes to field at offset +0x00cc
    }

    /// Decompiled virtual method #8 @ RVA 0x00667ea0 (52 instructions, 6 basic blocks)
    pub fn method_08(&mut self) {
    }

}

impl CcClass for ccWebButtonBase {
    fn class_name(&self) -> &'static str { Self::CLASS_NAME }
    fn vtable_rva(&self) -> u32 { Self::VTABLE_RVA }
    fn virtual_method_count(&self) -> usize { Self::METHOD_COUNT }
    fn base_classes(&self) -> &'static [&'static str] { Self::BASE_CLASSES }
}

/// Decompiled CC2 class: `ccWindowBase` (VTable RVA: 0x00866960, 9 virtual methods, 10 recovered fields)
#[derive(Debug, Clone, Default)]
pub struct ccWindowBase {
    pub vtable_rva: u32,
    pub field_0x0000: u64,
    pub field_0x0010: f32,
    pub field_0x0014: f32,
    pub field_0x0018: u64,
    pub field_0x0020: u32,
    pub field_0x0024: f32,
    pub field_0x0028: f32,
    pub field_0x002c: u32,
    pub field_0x0034: u32,
    pub field_0x0038: u32,
}

impl ccWindowBase {
    pub const CLASS_NAME: &'static str = "ccWindowBase";
    pub const VTABLE_RVA: u32 = 0x00866960;
    pub const METHOD_COUNT: usize = 9;
    pub const BASE_CLASSES: &'static [&'static str] = &[];
    pub const VIRTUAL_METHODS: &'static [u64] = &[0x00000001800ff1e0, 0x00000001801010f0, 0x00000001801012e0, 0x00000001801012d0, 0x00000001801012c0, 0x00000001800d3090, 0x00000001800d3090, 0x00000001800d3090, 0x00000001800d3090];

    pub fn new() -> Self {
        Self {
            vtable_rva: Self::VTABLE_RVA,
            field_0x0000: Default::default(),
            field_0x0010: Default::default(),
            field_0x0014: Default::default(),
            field_0x0018: Default::default(),
            field_0x0020: Default::default(),
            field_0x0024: Default::default(),
            field_0x0028: Default::default(),
            field_0x002c: Default::default(),
            field_0x0034: Default::default(),
            field_0x0038: Default::default(),
        }
    }

    /// Decompiled virtual method #0 @ RVA 0x000ff1e0 (16 instructions, 3 basic blocks)
    pub fn method_00(&mut self) {
    }

    /// Decompiled virtual method #1 @ RVA 0x001010f0 (111 instructions, 11 basic blocks)
    pub fn method_01(&mut self) {
        // Writes to field at offset +0x0010
        // Writes to field at offset +0x0014
        // Writes to field at offset +0x0018
        // Writes to field at offset +0x0020
        // Writes to field at offset +0x0024
        // Writes to field at offset +0x0028
        // Writes to field at offset +0x002c
        // Writes to field at offset +0x0034
        // Writes to field at offset +0x0038
    }

    /// Decompiled virtual method #2 @ RVA 0x001012e0 (3 instructions, 1 basic blocks)
    pub fn method_02(&mut self) {
        // Writes to field at offset +0x0010
        // Writes to field at offset +0x0014
    }

    /// Decompiled virtual method #3 @ RVA 0x001012d0 (2 instructions, 1 basic blocks)
    pub fn method_03(&mut self) {
        // Writes to field at offset +0x002c
    }

    /// Decompiled virtual method #4 @ RVA 0x001012c0 (3 instructions, 1 basic blocks)
    pub fn method_04(&mut self) {
        // Writes to field at offset +0x0024
        // Writes to field at offset +0x0028
    }

    /// Decompiled virtual method #5 @ RVA 0x000d3090 (1 instructions, 1 basic blocks)
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

}

impl CcClass for ccWindowBase {
    fn class_name(&self) -> &'static str { Self::CLASS_NAME }
    fn vtable_rva(&self) -> u32 { Self::VTABLE_RVA }
    fn virtual_method_count(&self) -> usize { Self::METHOD_COUNT }
    fn base_classes(&self) -> &'static [&'static str] { Self::BASE_CLASSES }
}

/// Decompiled CC2 class: `ccYMWeb` (VTable RVA: 0x008fb278, 11 virtual methods, 20 recovered fields)
/// Inherits: ccWebBase
#[derive(Debug, Clone, Default)]
pub struct ccYMWeb {
    pub vtable_rva: u32,
    pub field_0x0000: u64,
    pub field_0x0008: u64,
    pub field_0x0010: u64,
    pub field_0x0040: u32,
    pub field_0x0044: f32,
    pub field_0x0048: f32,
    pub field_0x0054: f32,
    pub field_0x0058: f32,
    pub field_0x0060: f32,
    pub field_0x0068: u64,
    pub field_0x0070: u64,
    pub field_0x0078: u64,
    pub field_0x0088: u64,
    pub field_0x0090: u8,
    pub field_0x00a0: u64,
    pub field_0x00c0: u64,
    pub field_0x00c8: u64,
    pub field_0x0360: u32,
    pub field_0x0368: u8,
    pub field_0x0398: [f32; 4],
}

impl ccYMWeb {
    pub const CLASS_NAME: &'static str = "ccYMWeb";
    pub const VTABLE_RVA: u32 = 0x008fb278;
    pub const METHOD_COUNT: usize = 11;
    pub const BASE_CLASSES: &'static [&'static str] = &["ccWebBase"];
    pub const VIRTUAL_METHODS: &'static [u64] = &[0x000000018066a9b0, 0x000000018066b070, 0x000000018066b370, 0x000000018066b3c0, 0x00000001806675e0, 0x000000018066b680, 0x000000018066a920, 0x000000018066a970, 0x00000001806687a0, 0x0000000180668b10, 0x0000000180668d80];

    pub fn new() -> Self {
        Self {
            vtable_rva: Self::VTABLE_RVA,
            field_0x0000: Default::default(),
            field_0x0008: Default::default(),
            field_0x0010: Default::default(),
            field_0x0040: Default::default(),
            field_0x0044: Default::default(),
            field_0x0048: Default::default(),
            field_0x0054: Default::default(),
            field_0x0058: Default::default(),
            field_0x0060: Default::default(),
            field_0x0068: Default::default(),
            field_0x0070: Default::default(),
            field_0x0078: Default::default(),
            field_0x0088: Default::default(),
            field_0x0090: Default::default(),
            field_0x00a0: Default::default(),
            field_0x00c0: Default::default(),
            field_0x00c8: Default::default(),
            field_0x0360: Default::default(),
            field_0x0368: Default::default(),
            field_0x0398: Default::default(),
        }
    }

    /// Decompiled virtual method #0 @ RVA 0x0066a9b0 (121 instructions, 18 basic blocks)
    pub fn method_00(&mut self) {
    }

    /// Decompiled virtual method #1 @ RVA 0x0066b070 (85 instructions, 7 basic blocks)
    pub fn method_01(&mut self) {
        // Writes to field at offset +0x0090
    }

    /// Decompiled virtual method #2 @ RVA 0x0066b370 (109 instructions, 11 basic blocks)
    pub fn method_02(&mut self) {
    }

    /// Decompiled virtual method #3 @ RVA 0x0066b3c0 (107 instructions, 14 basic blocks)
    pub fn method_03(&mut self) {
    }

    /// Decompiled virtual method #4 @ RVA 0x006675e0 (24 instructions, 3 basic blocks)
    pub fn method_04(&mut self) {
        // Writes to field at offset +0x0000
    }

    /// Decompiled virtual method #5 @ RVA 0x0066b680 (8 instructions, 1 basic blocks)
    pub fn method_05(&mut self) {
    }

    /// Decompiled virtual method #6 @ RVA 0x0066a920 (19 instructions, 3 basic blocks)
    pub fn method_06(&mut self) {
    }

    /// Decompiled virtual method #7 @ RVA 0x0066a970 (15 instructions, 4 basic blocks)
    pub fn method_07(&mut self) {
    }

    /// Decompiled virtual method #8 @ RVA 0x006687a0 (123 instructions, 17 basic blocks)
    pub fn method_08(&mut self) {
        // Writes to field at offset +0x0048
        // Writes to field at offset +0x0090
    }

    /// Decompiled virtual method #9 @ RVA 0x00668b10 (114 instructions, 9 basic blocks)
    pub fn method_09(&mut self) {
        // Writes to field at offset +0x0360
        // Writes to field at offset +0x0368
        // Writes to field at offset +0x0398
    }

    /// Decompiled virtual method #10 @ RVA 0x00668d80 (99 instructions, 5 basic blocks)
    pub fn method_10(&mut self) {
        // Writes to field at offset +0x0360
        // Writes to field at offset +0x0368
        // Writes to field at offset +0x0398
    }

}

impl CcClass for ccYMWeb {
    fn class_name(&self) -> &'static str { Self::CLASS_NAME }
    fn vtable_rva(&self) -> u32 { Self::VTABLE_RVA }
    fn virtual_method_count(&self) -> usize { Self::METHOD_COUNT }
    fn base_classes(&self) -> &'static [&'static str] { Self::BASE_CLASSES }
}


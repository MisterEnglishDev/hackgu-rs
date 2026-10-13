//! Subsystem: battle
//! Contains 4 decompiled classes.

use crate::CcClass;

/// Decompiled CC2 class: `ccArenaBattleProc::?A0x777a9069` (VTable RVA: 0x0087abe0, 5 virtual methods, 1 recovered fields)
/// Inherits: ccGameExtProcess
#[derive(Debug, Clone, Default)]
pub struct ccArenaBattleProc_A0x777a9069 {
    pub vtable_rva: u32,
    pub field_0x0008: u8,
}

impl ccArenaBattleProc_A0x777a9069 {
    pub const CLASS_NAME: &'static str = "ccArenaBattleProc::?A0x777a9069";
    pub const VTABLE_RVA: u32 = 0x0087abe0;
    pub const METHOD_COUNT: usize = 5;
    pub const BASE_CLASSES: &'static [&'static str] = &["ccGameExtProcess"];
    pub const VIRTUAL_METHODS: &'static [u64] = &[0x00000001801b52d0, 0x00000001801b52f0, 0x00000001801b5310, 0x00000001801b5300, 0x00000001800d6920];

    pub fn new() -> Self {
        Self {
            vtable_rva: Self::VTABLE_RVA,
            field_0x0008: Default::default(),
        }
    }

    /// Decompiled virtual method #0 @ RVA 0x001b52d0 (8 instructions, 1 basic blocks)
    pub fn method_00(&mut self) {
    }

    /// Decompiled virtual method #1 @ RVA 0x001b52f0 (2 instructions, 1 basic blocks)
    pub fn method_01(&mut self) {
    }

    /// Decompiled virtual method #2 @ RVA 0x001b5310 (16 instructions, 5 basic blocks)
    pub fn method_02(&mut self) {
    }

    /// Decompiled virtual method #3 @ RVA 0x001b5300 (28 instructions, 6 basic blocks)
    pub fn method_03(&mut self) {
    }

    /// Decompiled virtual method #4 @ RVA 0x000d6920 (2 instructions, 1 basic blocks)
    pub fn method_04(&mut self) {
    }

}

impl CcClass for ccArenaBattleProc_A0x777a9069 {
    fn class_name(&self) -> &'static str { Self::CLASS_NAME }
    fn vtable_rva(&self) -> u32 { Self::VTABLE_RVA }
    fn virtual_method_count(&self) -> usize { Self::METHOD_COUNT }
    fn base_classes(&self) -> &'static [&'static str] { Self::BASE_CLASSES }
}

/// Decompiled CC2 class: `ccAvatarBattle` (VTable RVA: 0x0087cb80, 3 virtual methods, 6 recovered fields)
#[derive(Debug, Clone, Default)]
pub struct ccAvatarBattle {
    pub vtable_rva: u32,
    pub field_0x0000: u64,
    pub field_0x0080: f32,
    pub field_0x0084: f32,
    pub field_0x0088: f32,
    pub field_0x008c: f32,
    pub field_0x0090: f32,
}

impl ccAvatarBattle {
    pub const CLASS_NAME: &'static str = "ccAvatarBattle";
    pub const VTABLE_RVA: u32 = 0x0087cb80;
    pub const METHOD_COUNT: usize = 3;
    pub const BASE_CLASSES: &'static [&'static str] = &[];
    pub const VIRTUAL_METHODS: &'static [u64] = &[0x00000001801d8b90, 0x00000001800d6920, 0x0000000180103e70];

    pub fn new() -> Self {
        Self {
            vtable_rva: Self::VTABLE_RVA,
            field_0x0000: Default::default(),
            field_0x0080: Default::default(),
            field_0x0084: Default::default(),
            field_0x0088: Default::default(),
            field_0x008c: Default::default(),
            field_0x0090: Default::default(),
        }
    }

    /// Decompiled virtual method #0 @ RVA 0x001d8b90 (23 instructions, 3 basic blocks)
    pub fn method_00(&mut self) {
        // Writes to field at offset +0x0000
    }

    /// Decompiled virtual method #1 @ RVA 0x000d6920 (2 instructions, 1 basic blocks)
    pub fn method_01(&mut self) {
    }

    /// Decompiled virtual method #2 @ RVA 0x00103e70 (2 instructions, 1 basic blocks)
    pub fn method_02(&mut self) {
    }

}

impl CcClass for ccAvatarBattle {
    fn class_name(&self) -> &'static str { Self::CLASS_NAME }
    fn vtable_rva(&self) -> u32 { Self::VTABLE_RVA }
    fn virtual_method_count(&self) -> usize { Self::METHOD_COUNT }
    fn base_classes(&self) -> &'static [&'static str] { Self::BASE_CLASSES }
}

/// Decompiled CC2 class: `ccAvatarBattle2` (VTable RVA: 0x0087cb60, 3 virtual methods, 2 recovered fields)
/// Inherits: ccAvatarBattle
#[derive(Debug, Clone, Default)]
pub struct ccAvatarBattle2 {
    pub vtable_rva: u32,
    pub field_0x0060: u64,
    pub field_0x02b8: u64,
}

impl ccAvatarBattle2 {
    pub const CLASS_NAME: &'static str = "ccAvatarBattle2";
    pub const VTABLE_RVA: u32 = 0x0087cb60;
    pub const METHOD_COUNT: usize = 3;
    pub const BASE_CLASSES: &'static [&'static str] = &["ccAvatarBattle"];
    pub const VIRTUAL_METHODS: &'static [u64] = &[0x00000001801d8c70, 0x00000001801d8ec0, 0x00000001801d8ef0];

    pub fn new() -> Self {
        Self {
            vtable_rva: Self::VTABLE_RVA,
            field_0x0060: Default::default(),
            field_0x02b8: Default::default(),
        }
    }

    /// Decompiled virtual method #0 @ RVA 0x001d8c70 (16 instructions, 3 basic blocks)
    pub fn method_00(&mut self) {
    }

    /// Decompiled virtual method #1 @ RVA 0x001d8ec0 (8 instructions, 2 basic blocks)
    pub fn method_01(&mut self) {
    }

    /// Decompiled virtual method #2 @ RVA 0x001d8ef0 (11 instructions, 4 basic blocks)
    pub fn method_02(&mut self) {
    }

}

impl CcClass for ccAvatarBattle2 {
    fn class_name(&self) -> &'static str { Self::CLASS_NAME }
    fn vtable_rva(&self) -> u32 { Self::VTABLE_RVA }
    fn virtual_method_count(&self) -> usize { Self::METHOD_COUNT }
    fn base_classes(&self) -> &'static [&'static str] { Self::BASE_CLASSES }
}

/// Decompiled CC2 class: `ccAvatarBattleProc::?A0x6a4993fb` (VTable RVA: 0x0087cba0, 5 virtual methods, 2 recovered fields)
/// Inherits: ccGameExtProcess
#[derive(Debug, Clone, Default)]
pub struct ccAvatarBattleProc_A0x6a4993fb {
    pub vtable_rva: u32,
    pub field_0x0008: u8,
    pub field_0x0010: u64,
}

impl ccAvatarBattleProc_A0x6a4993fb {
    pub const CLASS_NAME: &'static str = "ccAvatarBattleProc::?A0x6a4993fb";
    pub const VTABLE_RVA: u32 = 0x0087cba0;
    pub const METHOD_COUNT: usize = 5;
    pub const BASE_CLASSES: &'static [&'static str] = &["ccGameExtProcess"];
    pub const VIRTUAL_METHODS: &'static [u64] = &[0x00000001801d8810, 0x00000001801d88a0, 0x00000001801d8920, 0x00000001801d8910, 0x00000001801b52f0];

    pub fn new() -> Self {
        Self {
            vtable_rva: Self::VTABLE_RVA,
            field_0x0008: Default::default(),
            field_0x0010: Default::default(),
        }
    }

    /// Decompiled virtual method #0 @ RVA 0x001d8810 (31 instructions, 7 basic blocks)
    pub fn method_00(&mut self) {
    }

    /// Decompiled virtual method #1 @ RVA 0x001d88a0 (25 instructions, 7 basic blocks)
    pub fn method_01(&mut self) {
    }

    /// Decompiled virtual method #2 @ RVA 0x001d8920 (13 instructions, 3 basic blocks)
    pub fn method_02(&mut self) {
    }

    /// Decompiled virtual method #3 @ RVA 0x001d8910 (25 instructions, 4 basic blocks)
    pub fn method_03(&mut self) {
    }

    /// Decompiled virtual method #4 @ RVA 0x001b52f0 (2 instructions, 1 basic blocks)
    pub fn method_04(&mut self) {
    }

}

impl CcClass for ccAvatarBattleProc_A0x6a4993fb {
    fn class_name(&self) -> &'static str { Self::CLASS_NAME }
    fn vtable_rva(&self) -> u32 { Self::VTABLE_RVA }
    fn virtual_method_count(&self) -> usize { Self::METHOD_COUNT }
    fn base_classes(&self) -> &'static [&'static str] { Self::BASE_CLASSES }
}


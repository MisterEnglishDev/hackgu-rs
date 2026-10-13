//! Subsystem: event
//! Contains 103 decompiled classes.

use crate::CcClass;

/// Decompiled CC2 class: `ccEvFade` (VTable RVA: 0x008b3e08, 1 virtual methods, 0 recovered fields)
#[derive(Debug, Clone, Default)]
pub struct ccEvFade {
    pub vtable_rva: u32,
}

impl ccEvFade {
    pub const CLASS_NAME: &'static str = "ccEvFade";
    pub const VTABLE_RVA: u32 = 0x008b3e08;
    pub const METHOD_COUNT: usize = 1;
    pub const BASE_CLASSES: &'static [&'static str] = &[];
    pub const VIRTUAL_METHODS: &'static [u64] = &[0x00000001803832a0];

    pub fn new() -> Self {
        Self {
            vtable_rva: Self::VTABLE_RVA,
        }
    }

    /// Decompiled virtual method #0 @ RVA 0x003832a0 (16 instructions, 3 basic blocks)
    pub fn method_00(&mut self) {
    }

}

impl CcClass for ccEvFade {
    fn class_name(&self) -> &'static str { Self::CLASS_NAME }
    fn vtable_rva(&self) -> u32 { Self::VTABLE_RVA }
    fn virtual_method_count(&self) -> usize { Self::METHOD_COUNT }
    fn base_classes(&self) -> &'static [&'static str] { Self::BASE_CLASSES }
}

/// Decompiled CC2 class: `ccEventAction` (VTable RVA: 0x008b6ca8, 4 virtual methods, 3 recovered fields)
/// Inherits: ccEventBase
#[derive(Debug, Clone, Default)]
pub struct ccEventAction {
    pub vtable_rva: u32,
    pub field_0x0000: u64,
    pub field_0x0008: u16,
    pub field_0x000a: u8,
}

impl ccEventAction {
    pub const CLASS_NAME: &'static str = "ccEventAction";
    pub const VTABLE_RVA: u32 = 0x008b6ca8;
    pub const METHOD_COUNT: usize = 4;
    pub const BASE_CLASSES: &'static [&'static str] = &["ccEventBase"];
    pub const VIRTUAL_METHODS: &'static [u64] = &[0x0000000180385700, 0x0000000180380cf0, 0x00000001800d3090, 0x000000018038a580];

    pub fn new() -> Self {
        Self {
            vtable_rva: Self::VTABLE_RVA,
            field_0x0000: Default::default(),
            field_0x0008: Default::default(),
            field_0x000a: Default::default(),
        }
    }

    /// Decompiled virtual method #0 @ RVA 0x00385700 (18 instructions, 3 basic blocks)
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

    /// Decompiled virtual method #3 @ RVA 0x0038a580 (12 instructions, 2 basic blocks)
    pub fn method_03(&mut self) {
    }

}

impl CcClass for ccEventAction {
    fn class_name(&self) -> &'static str { Self::CLASS_NAME }
    fn vtable_rva(&self) -> u32 { Self::VTABLE_RVA }
    fn virtual_method_count(&self) -> usize { Self::METHOD_COUNT }
    fn base_classes(&self) -> &'static [&'static str] { Self::BASE_CLASSES }
}

/// Decompiled CC2 class: `ccEventAnimal` (VTable RVA: 0x008b6710, 4 virtual methods, 7 recovered fields)
/// Inherits: ccEventBase
#[derive(Debug, Clone, Default)]
pub struct ccEventAnimal {
    pub vtable_rva: u32,
    pub field_0x0000: u64,
    pub field_0x0004: u16,
    pub field_0x0006: u8,
    pub field_0x0008: u16,
    pub field_0x000a: u8,
    pub field_0x0c30: u64,
    pub field_0x0c48: u64,
}

impl ccEventAnimal {
    pub const CLASS_NAME: &'static str = "ccEventAnimal";
    pub const VTABLE_RVA: u32 = 0x008b6710;
    pub const METHOD_COUNT: usize = 4;
    pub const BASE_CLASSES: &'static [&'static str] = &["ccEventBase"];
    pub const VIRTUAL_METHODS: &'static [u64] = &[0x00000001803861c0, 0x0000000180380cf0, 0x00000001800d3090, 0x000000018038f620];

    pub fn new() -> Self {
        Self {
            vtable_rva: Self::VTABLE_RVA,
            field_0x0000: Default::default(),
            field_0x0004: Default::default(),
            field_0x0006: Default::default(),
            field_0x0008: Default::default(),
            field_0x000a: Default::default(),
            field_0x0c30: Default::default(),
            field_0x0c48: Default::default(),
        }
    }

    /// Decompiled virtual method #0 @ RVA 0x003861c0 (18 instructions, 3 basic blocks)
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

    /// Decompiled virtual method #3 @ RVA 0x0038f620 (101 instructions, 27 basic blocks)
    pub fn method_03(&mut self) {
        // Writes to field at offset +0x0004
        // Writes to field at offset +0x0006
    }

}

impl CcClass for ccEventAnimal {
    fn class_name(&self) -> &'static str { Self::CLASS_NAME }
    fn vtable_rva(&self) -> u32 { Self::VTABLE_RVA }
    fn virtual_method_count(&self) -> usize { Self::METHOD_COUNT }
    fn base_classes(&self) -> &'static [&'static str] { Self::BASE_CLASSES }
}

/// Decompiled CC2 class: `ccEventApplication` (VTable RVA: 0x008b6688, 4 virtual methods, 4 recovered fields)
/// Inherits: ccEventBase
#[derive(Debug, Clone, Default)]
pub struct ccEventApplication {
    pub vtable_rva: u32,
    pub field_0x0000: u64,
    pub field_0x0008: u16,
    pub field_0x000a: u8,
    pub field_0x0020: u32,
}

impl ccEventApplication {
    pub const CLASS_NAME: &'static str = "ccEventApplication";
    pub const VTABLE_RVA: u32 = 0x008b6688;
    pub const METHOD_COUNT: usize = 4;
    pub const BASE_CLASSES: &'static [&'static str] = &["ccEventBase"];
    pub const VIRTUAL_METHODS: &'static [u64] = &[0x00000001803865f0, 0x0000000180380cf0, 0x00000001800d3090, 0x0000000180390c10];

    pub fn new() -> Self {
        Self {
            vtable_rva: Self::VTABLE_RVA,
            field_0x0000: Default::default(),
            field_0x0008: Default::default(),
            field_0x000a: Default::default(),
            field_0x0020: Default::default(),
        }
    }

    /// Decompiled virtual method #0 @ RVA 0x003865f0 (18 instructions, 3 basic blocks)
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

    /// Decompiled virtual method #3 @ RVA 0x00390c10 (21 instructions, 5 basic blocks)
    pub fn method_03(&mut self) {
    }

}

impl CcClass for ccEventApplication {
    fn class_name(&self) -> &'static str { Self::CLASS_NAME }
    fn vtable_rva(&self) -> u32 { Self::VTABLE_RVA }
    fn virtual_method_count(&self) -> usize { Self::METHOD_COUNT }
    fn base_classes(&self) -> &'static [&'static str] { Self::BASE_CLASSES }
}

/// Decompiled CC2 class: `ccEventAreamove` (VTable RVA: 0x008b66b0, 4 virtual methods, 3 recovered fields)
/// Inherits: ccEventBase
#[derive(Debug, Clone, Default)]
pub struct ccEventAreamove {
    pub vtable_rva: u32,
    pub field_0x0000: u64,
    pub field_0x0008: u16,
    pub field_0x000a: u8,
}

impl ccEventAreamove {
    pub const CLASS_NAME: &'static str = "ccEventAreamove";
    pub const VTABLE_RVA: u32 = 0x008b66b0;
    pub const METHOD_COUNT: usize = 4;
    pub const BASE_CLASSES: &'static [&'static str] = &["ccEventBase"];
    pub const VIRTUAL_METHODS: &'static [u64] = &[0x0000000180386630, 0x0000000180380cf0, 0x00000001800d3090, 0x0000000180390cb0];

    pub fn new() -> Self {
        Self {
            vtable_rva: Self::VTABLE_RVA,
            field_0x0000: Default::default(),
            field_0x0008: Default::default(),
            field_0x000a: Default::default(),
        }
    }

    /// Decompiled virtual method #0 @ RVA 0x00386630 (18 instructions, 3 basic blocks)
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

    /// Decompiled virtual method #3 @ RVA 0x00390cb0 (15 instructions, 2 basic blocks)
    pub fn method_03(&mut self) {
    }

}

impl CcClass for ccEventAreamove {
    fn class_name(&self) -> &'static str { Self::CLASS_NAME }
    fn vtable_rva(&self) -> u32 { Self::VTABLE_RVA }
    fn virtual_method_count(&self) -> usize { Self::METHOD_COUNT }
    fn base_classes(&self) -> &'static [&'static str] { Self::BASE_CLASSES }
}

/// Decompiled CC2 class: `ccEventArena` (VTable RVA: 0x008b56c0, 4 virtual methods, 3 recovered fields)
/// Inherits: ccEventBase
#[derive(Debug, Clone, Default)]
pub struct ccEventArena {
    pub vtable_rva: u32,
    pub field_0x0000: u64,
    pub field_0x0008: u16,
    pub field_0x000a: u8,
}

impl ccEventArena {
    pub const CLASS_NAME: &'static str = "ccEventArena";
    pub const VTABLE_RVA: u32 = 0x008b56c0;
    pub const METHOD_COUNT: usize = 4;
    pub const BASE_CLASSES: &'static [&'static str] = &["ccEventBase"];
    pub const VIRTUAL_METHODS: &'static [u64] = &[0x00000001803868b0, 0x0000000180380cf0, 0x00000001800d3090, 0x0000000180391be0];

    pub fn new() -> Self {
        Self {
            vtable_rva: Self::VTABLE_RVA,
            field_0x0000: Default::default(),
            field_0x0008: Default::default(),
            field_0x000a: Default::default(),
        }
    }

    /// Decompiled virtual method #0 @ RVA 0x003868b0 (18 instructions, 3 basic blocks)
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

    /// Decompiled virtual method #3 @ RVA 0x00391be0 (12 instructions, 2 basic blocks)
    pub fn method_03(&mut self) {
    }

}

impl CcClass for ccEventArena {
    fn class_name(&self) -> &'static str { Self::CLASS_NAME }
    fn vtable_rva(&self) -> u32 { Self::VTABLE_RVA }
    fn virtual_method_count(&self) -> usize { Self::METHOD_COUNT }
    fn base_classes(&self) -> &'static [&'static str] { Self::BASE_CLASSES }
}

/// Decompiled CC2 class: `ccEventAvatar` (VTable RVA: 0x008b6938, 4 virtual methods, 3 recovered fields)
/// Inherits: ccEventBase
#[derive(Debug, Clone, Default)]
pub struct ccEventAvatar {
    pub vtable_rva: u32,
    pub field_0x0000: u64,
    pub field_0x0008: u16,
    pub field_0x000a: u8,
}

impl ccEventAvatar {
    pub const CLASS_NAME: &'static str = "ccEventAvatar";
    pub const VTABLE_RVA: u32 = 0x008b6938;
    pub const METHOD_COUNT: usize = 4;
    pub const BASE_CLASSES: &'static [&'static str] = &["ccEventBase"];
    pub const VIRTUAL_METHODS: &'static [u64] = &[0x0000000180385d70, 0x0000000180380cf0, 0x00000001800d3090, 0x000000018038d440];

    pub fn new() -> Self {
        Self {
            vtable_rva: Self::VTABLE_RVA,
            field_0x0000: Default::default(),
            field_0x0008: Default::default(),
            field_0x000a: Default::default(),
        }
    }

    /// Decompiled virtual method #0 @ RVA 0x00385d70 (18 instructions, 3 basic blocks)
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

    /// Decompiled virtual method #3 @ RVA 0x0038d440 (12 instructions, 2 basic blocks)
    pub fn method_03(&mut self) {
    }

}

impl CcClass for ccEventAvatar {
    fn class_name(&self) -> &'static str { Self::CLASS_NAME }
    fn vtable_rva(&self) -> u32 { Self::VTABLE_RVA }
    fn virtual_method_count(&self) -> usize { Self::METHOD_COUNT }
    fn base_classes(&self) -> &'static [&'static str] { Self::BASE_CLASSES }
}

/// Decompiled CC2 class: `ccEventBGDesign` (VTable RVA: 0x008b5648, 4 virtual methods, 3 recovered fields)
/// Inherits: ccEventBase
#[derive(Debug, Clone, Default)]
pub struct ccEventBGDesign {
    pub vtable_rva: u32,
    pub field_0x0000: u64,
    pub field_0x0008: u16,
    pub field_0x000a: u8,
}

impl ccEventBGDesign {
    pub const CLASS_NAME: &'static str = "ccEventBGDesign";
    pub const VTABLE_RVA: u32 = 0x008b5648;
    pub const METHOD_COUNT: usize = 4;
    pub const BASE_CLASSES: &'static [&'static str] = &["ccEventBase"];
    pub const VIRTUAL_METHODS: &'static [u64] = &[0x0000000180386b00, 0x0000000180380cf0, 0x00000001800d3090, 0x00000001803932f0];

    pub fn new() -> Self {
        Self {
            vtable_rva: Self::VTABLE_RVA,
            field_0x0000: Default::default(),
            field_0x0008: Default::default(),
            field_0x000a: Default::default(),
        }
    }

    /// Decompiled virtual method #0 @ RVA 0x00386b00 (18 instructions, 3 basic blocks)
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

    /// Decompiled virtual method #3 @ RVA 0x003932f0 (53 instructions, 15 basic blocks)
    pub fn method_03(&mut self) {
    }

}

impl CcClass for ccEventBGDesign {
    fn class_name(&self) -> &'static str { Self::CLASS_NAME }
    fn vtable_rva(&self) -> u32 { Self::VTABLE_RVA }
    fn virtual_method_count(&self) -> usize { Self::METHOD_COUNT }
    fn base_classes(&self) -> &'static [&'static str] { Self::BASE_CLASSES }
}

/// Decompiled CC2 class: `ccEventBase` (VTable RVA: 0x008b3f08, 4 virtual methods, 2 recovered fields)
#[derive(Debug, Clone, Default)]
pub struct ccEventBase {
    pub vtable_rva: u32,
    pub field_0x0008: u16,
    pub field_0x000a: u8,
}

impl ccEventBase {
    pub const CLASS_NAME: &'static str = "ccEventBase";
    pub const VTABLE_RVA: u32 = 0x008b3f08;
    pub const METHOD_COUNT: usize = 4;
    pub const BASE_CLASSES: &'static [&'static str] = &[];
    pub const VIRTUAL_METHODS: &'static [u64] = &[0x0000000180380ac0, 0x0000000180380cf0, 0x00000001800d3090, 0x000000018073e2c0];

    pub fn new() -> Self {
        Self {
            vtable_rva: Self::VTABLE_RVA,
            field_0x0008: Default::default(),
            field_0x000a: Default::default(),
        }
    }

    /// Decompiled virtual method #0 @ RVA 0x00380ac0 (16 instructions, 3 basic blocks)
    pub fn method_00(&mut self) {
    }

    /// Decompiled virtual method #1 @ RVA 0x00380cf0 (13 instructions, 2 basic blocks)
    pub fn method_01(&mut self) {
        // Writes to field at offset +0x0008
        // Writes to field at offset +0x000a
    }

    /// Decompiled virtual method #2 @ RVA 0x000d3090 (1 instructions, 1 basic blocks)
    pub fn method_02(&mut self) {
    }

    /// Decompiled virtual method #3 @ RVA 0x0073e2c0 (337 instructions, 1 basic blocks)
    pub fn method_03(&mut self) {
    }

}

impl CcClass for ccEventBase {
    fn class_name(&self) -> &'static str { Self::CLASS_NAME }
    fn vtable_rva(&self) -> u32 { Self::VTABLE_RVA }
    fn virtual_method_count(&self) -> usize { Self::METHOD_COUNT }
    fn base_classes(&self) -> &'static [&'static str] { Self::BASE_CLASSES }
}

/// Decompiled CC2 class: `ccEventBattle` (VTable RVA: 0x008b57b0, 4 virtual methods, 3 recovered fields)
/// Inherits: ccEventBase
#[derive(Debug, Clone, Default)]
pub struct ccEventBattle {
    pub vtable_rva: u32,
    pub field_0x0000: u64,
    pub field_0x0008: u16,
    pub field_0x000a: u8,
}

impl ccEventBattle {
    pub const CLASS_NAME: &'static str = "ccEventBattle";
    pub const VTABLE_RVA: u32 = 0x008b57b0;
    pub const METHOD_COUNT: usize = 4;
    pub const BASE_CLASSES: &'static [&'static str] = &["ccEventBase"];
    pub const VIRTUAL_METHODS: &'static [u64] = &[0x0000000180386930, 0x0000000180380cf0, 0x00000001800d3090, 0x0000000180392200];

    pub fn new() -> Self {
        Self {
            vtable_rva: Self::VTABLE_RVA,
            field_0x0000: Default::default(),
            field_0x0008: Default::default(),
            field_0x000a: Default::default(),
        }
    }

    /// Decompiled virtual method #0 @ RVA 0x00386930 (18 instructions, 3 basic blocks)
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

    /// Decompiled virtual method #3 @ RVA 0x00392200 (116 instructions, 21 basic blocks)
    pub fn method_03(&mut self) {
    }

}

impl CcClass for ccEventBattle {
    fn class_name(&self) -> &'static str { Self::CLASS_NAME }
    fn vtable_rva(&self) -> u32 { Self::VTABLE_RVA }
    fn virtual_method_count(&self) -> usize { Self::METHOD_COUNT }
    fn base_classes(&self) -> &'static [&'static str] { Self::BASE_CLASSES }
}

/// Decompiled CC2 class: `ccEventBbs` (VTable RVA: 0x008b6b40, 4 virtual methods, 3 recovered fields)
/// Inherits: ccEventBase
#[derive(Debug, Clone, Default)]
pub struct ccEventBbs {
    pub vtable_rva: u32,
    pub field_0x0000: u64,
    pub field_0x0008: u16,
    pub field_0x000a: u8,
}

impl ccEventBbs {
    pub const CLASS_NAME: &'static str = "ccEventBbs";
    pub const VTABLE_RVA: u32 = 0x008b6b40;
    pub const METHOD_COUNT: usize = 4;
    pub const BASE_CLASSES: &'static [&'static str] = &["ccEventBase"];
    pub const VIRTUAL_METHODS: &'static [u64] = &[0x0000000180385ac0, 0x0000000180380cf0, 0x00000001800d3090, 0x000000018038c6c0];

    pub fn new() -> Self {
        Self {
            vtable_rva: Self::VTABLE_RVA,
            field_0x0000: Default::default(),
            field_0x0008: Default::default(),
            field_0x000a: Default::default(),
        }
    }

    /// Decompiled virtual method #0 @ RVA 0x00385ac0 (18 instructions, 3 basic blocks)
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

    /// Decompiled virtual method #3 @ RVA 0x0038c6c0 (12 instructions, 2 basic blocks)
    pub fn method_03(&mut self) {
    }

}

impl CcClass for ccEventBbs {
    fn class_name(&self) -> &'static str { Self::CLASS_NAME }
    fn vtable_rva(&self) -> u32 { Self::VTABLE_RVA }
    fn virtual_method_count(&self) -> usize { Self::METHOD_COUNT }
    fn base_classes(&self) -> &'static [&'static str] { Self::BASE_CLASSES }
}

/// Decompiled CC2 class: `ccEventBeast` (VTable RVA: 0x008b3e18, 4 virtual methods, 3 recovered fields)
/// Inherits: ccEventBase
#[derive(Debug, Clone, Default)]
pub struct ccEventBeast {
    pub vtable_rva: u32,
    pub field_0x0000: u64,
    pub field_0x0008: u16,
    pub field_0x000a: u8,
}

impl ccEventBeast {
    pub const CLASS_NAME: &'static str = "ccEventBeast";
    pub const VTABLE_RVA: u32 = 0x008b3e18;
    pub const METHOD_COUNT: usize = 4;
    pub const BASE_CLASSES: &'static [&'static str] = &["ccEventBase"];
    pub const VIRTUAL_METHODS: &'static [u64] = &[0x0000000180381fe0, 0x0000000180380cf0, 0x00000001800d3090, 0x0000000180382020];

    pub fn new() -> Self {
        Self {
            vtable_rva: Self::VTABLE_RVA,
            field_0x0000: Default::default(),
            field_0x0008: Default::default(),
            field_0x000a: Default::default(),
        }
    }

    /// Decompiled virtual method #0 @ RVA 0x00381fe0 (18 instructions, 3 basic blocks)
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

    /// Decompiled virtual method #3 @ RVA 0x00382020 (19 instructions, 4 basic blocks)
    pub fn method_03(&mut self) {
    }

}

impl CcClass for ccEventBeast {
    fn class_name(&self) -> &'static str { Self::CLASS_NAME }
    fn vtable_rva(&self) -> u32 { Self::VTABLE_RVA }
    fn virtual_method_count(&self) -> usize { Self::METHOD_COUNT }
    fn base_classes(&self) -> &'static [&'static str] { Self::BASE_CLASSES }
}

/// Decompiled CC2 class: `ccEventBeastBox` (VTable RVA: 0x008b6ac8, 4 virtual methods, 3 recovered fields)
/// Inherits: ccEventBase
#[derive(Debug, Clone, Default)]
pub struct ccEventBeastBox {
    pub vtable_rva: u32,
    pub field_0x0000: u64,
    pub field_0x0008: u16,
    pub field_0x000a: u8,
}

impl ccEventBeastBox {
    pub const CLASS_NAME: &'static str = "ccEventBeastBox";
    pub const VTABLE_RVA: u32 = 0x008b6ac8;
    pub const METHOD_COUNT: usize = 4;
    pub const BASE_CLASSES: &'static [&'static str] = &["ccEventBase"];
    pub const VIRTUAL_METHODS: &'static [u64] = &[0x0000000180385b00, 0x0000000180380cf0, 0x00000001800d3090, 0x000000018038c830];

    pub fn new() -> Self {
        Self {
            vtable_rva: Self::VTABLE_RVA,
            field_0x0000: Default::default(),
            field_0x0008: Default::default(),
            field_0x000a: Default::default(),
        }
    }

    /// Decompiled virtual method #0 @ RVA 0x00385b00 (18 instructions, 3 basic blocks)
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

    /// Decompiled virtual method #3 @ RVA 0x0038c830 (22 instructions, 5 basic blocks)
    pub fn method_03(&mut self) {
    }

}

impl CcClass for ccEventBeastBox {
    fn class_name(&self) -> &'static str { Self::CLASS_NAME }
    fn vtable_rva(&self) -> u32 { Self::VTABLE_RVA }
    fn virtual_method_count(&self) -> usize { Self::METHOD_COUNT }
    fn base_classes(&self) -> &'static [&'static str] { Self::BASE_CLASSES }
}

/// Decompiled CC2 class: `ccEventBike` (VTable RVA: 0x008b67f8, 4 virtual methods, 3 recovered fields)
/// Inherits: ccEventBase
#[derive(Debug, Clone, Default)]
pub struct ccEventBike {
    pub vtable_rva: u32,
    pub field_0x0000: u64,
    pub field_0x0008: u16,
    pub field_0x000a: u8,
}

impl ccEventBike {
    pub const CLASS_NAME: &'static str = "ccEventBike";
    pub const VTABLE_RVA: u32 = 0x008b67f8;
    pub const METHOD_COUNT: usize = 4;
    pub const BASE_CLASSES: &'static [&'static str] = &["ccEventBase"];
    pub const VIRTUAL_METHODS: &'static [u64] = &[0x0000000180386000, 0x0000000180380cf0, 0x00000001800d3090, 0x000000018038e1b0];

    pub fn new() -> Self {
        Self {
            vtable_rva: Self::VTABLE_RVA,
            field_0x0000: Default::default(),
            field_0x0008: Default::default(),
            field_0x000a: Default::default(),
        }
    }

    /// Decompiled virtual method #0 @ RVA 0x00386000 (18 instructions, 3 basic blocks)
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

    /// Decompiled virtual method #3 @ RVA 0x0038e1b0 (12 instructions, 2 basic blocks)
    pub fn method_03(&mut self) {
    }

}

impl CcClass for ccEventBike {
    fn class_name(&self) -> &'static str { Self::CLASS_NAME }
    fn vtable_rva(&self) -> u32 { Self::VTABLE_RVA }
    fn virtual_method_count(&self) -> usize { Self::METHOD_COUNT }
    fn base_classes(&self) -> &'static [&'static str] { Self::BASE_CLASSES }
}

/// Decompiled CC2 class: `ccEventBlock` (VTable RVA: 0x008b68e8, 4 virtual methods, 3 recovered fields)
/// Inherits: ccEventBase
#[derive(Debug, Clone, Default)]
pub struct ccEventBlock {
    pub vtable_rva: u32,
    pub field_0x0000: u64,
    pub field_0x0008: u16,
    pub field_0x000a: u8,
}

impl ccEventBlock {
    pub const CLASS_NAME: &'static str = "ccEventBlock";
    pub const VTABLE_RVA: u32 = 0x008b68e8;
    pub const METHOD_COUNT: usize = 4;
    pub const BASE_CLASSES: &'static [&'static str] = &["ccEventBase"];
    pub const VIRTUAL_METHODS: &'static [u64] = &[0x0000000180385df0, 0x0000000180380cf0, 0x00000001800d3090, 0x000000018038dbe0];

    pub fn new() -> Self {
        Self {
            vtable_rva: Self::VTABLE_RVA,
            field_0x0000: Default::default(),
            field_0x0008: Default::default(),
            field_0x000a: Default::default(),
        }
    }

    /// Decompiled virtual method #0 @ RVA 0x00385df0 (18 instructions, 3 basic blocks)
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

    /// Decompiled virtual method #3 @ RVA 0x0038dbe0 (48 instructions, 13 basic blocks)
    pub fn method_03(&mut self) {
    }

}

impl CcClass for ccEventBlock {
    fn class_name(&self) -> &'static str { Self::CLASS_NAME }
    fn vtable_rva(&self) -> u32 { Self::VTABLE_RVA }
    fn virtual_method_count(&self) -> usize { Self::METHOD_COUNT }
    fn base_classes(&self) -> &'static [&'static str] { Self::BASE_CLASSES }
}

/// Decompiled CC2 class: `ccEventBookmark` (VTable RVA: 0x008b6a50, 4 virtual methods, 3 recovered fields)
/// Inherits: ccEventBase
#[derive(Debug, Clone, Default)]
pub struct ccEventBookmark {
    pub vtable_rva: u32,
    pub field_0x0000: u64,
    pub field_0x0008: u16,
    pub field_0x000a: u8,
}

impl ccEventBookmark {
    pub const CLASS_NAME: &'static str = "ccEventBookmark";
    pub const VTABLE_RVA: u32 = 0x008b6a50;
    pub const METHOD_COUNT: usize = 4;
    pub const BASE_CLASSES: &'static [&'static str] = &["ccEventBase"];
    pub const VIRTUAL_METHODS: &'static [u64] = &[0x0000000180385a00, 0x0000000180380cf0, 0x00000001800d3090, 0x000000018038c110];

    pub fn new() -> Self {
        Self {
            vtable_rva: Self::VTABLE_RVA,
            field_0x0000: Default::default(),
            field_0x0008: Default::default(),
            field_0x000a: Default::default(),
        }
    }

    /// Decompiled virtual method #0 @ RVA 0x00385a00 (18 instructions, 3 basic blocks)
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

    /// Decompiled virtual method #3 @ RVA 0x0038c110 (12 instructions, 2 basic blocks)
    pub fn method_03(&mut self) {
    }

}

impl CcClass for ccEventBookmark {
    fn class_name(&self) -> &'static str { Self::CLASS_NAME }
    fn vtable_rva(&self) -> u32 { Self::VTABLE_RVA }
    fn virtual_method_count(&self) -> usize { Self::METHOD_COUNT }
    fn base_classes(&self) -> &'static [&'static str] { Self::BASE_CLASSES }
}

/// Decompiled CC2 class: `ccEventBoss` (VTable RVA: 0x008b3eb8, 4 virtual methods, 3 recovered fields)
/// Inherits: ccEventBase
#[derive(Debug, Clone, Default)]
pub struct ccEventBoss {
    pub vtable_rva: u32,
    pub field_0x0000: u64,
    pub field_0x0008: u16,
    pub field_0x000a: u8,
}

impl ccEventBoss {
    pub const CLASS_NAME: &'static str = "ccEventBoss";
    pub const VTABLE_RVA: u32 = 0x008b3eb8;
    pub const METHOD_COUNT: usize = 4;
    pub const BASE_CLASSES: &'static [&'static str] = &["ccEventBase"];
    pub const VIRTUAL_METHODS: &'static [u64] = &[0x0000000180381eb0, 0x0000000180380cf0, 0x00000001800d3090, 0x0000000180381ef0];

    pub fn new() -> Self {
        Self {
            vtable_rva: Self::VTABLE_RVA,
            field_0x0000: Default::default(),
            field_0x0008: Default::default(),
            field_0x000a: Default::default(),
        }
    }

    /// Decompiled virtual method #0 @ RVA 0x00381eb0 (18 instructions, 3 basic blocks)
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

    /// Decompiled virtual method #3 @ RVA 0x00381ef0 (28 instructions, 7 basic blocks)
    pub fn method_03(&mut self) {
    }

}

impl CcClass for ccEventBoss {
    fn class_name(&self) -> &'static str { Self::CLASS_NAME }
    fn vtable_rva(&self) -> u32 { Self::VTABLE_RVA }
    fn virtual_method_count(&self) -> usize { Self::METHOD_COUNT }
    fn base_classes(&self) -> &'static [&'static str] { Self::BASE_CLASSES }
}

/// Decompiled CC2 class: `ccEventBtlState` (VTable RVA: 0x008b6b90, 4 virtual methods, 3 recovered fields)
/// Inherits: ccEventBase
#[derive(Debug, Clone, Default)]
pub struct ccEventBtlState {
    pub vtable_rva: u32,
    pub field_0x0000: u64,
    pub field_0x0008: u16,
    pub field_0x000a: u8,
}

impl ccEventBtlState {
    pub const CLASS_NAME: &'static str = "ccEventBtlState";
    pub const VTABLE_RVA: u32 = 0x008b6b90;
    pub const METHOD_COUNT: usize = 4;
    pub const BASE_CLASSES: &'static [&'static str] = &["ccEventBase"];
    pub const VIRTUAL_METHODS: &'static [u64] = &[0x0000000180385880, 0x0000000180380cf0, 0x00000001800d3090, 0x000000018038b3f0];

    pub fn new() -> Self {
        Self {
            vtable_rva: Self::VTABLE_RVA,
            field_0x0000: Default::default(),
            field_0x0008: Default::default(),
            field_0x000a: Default::default(),
        }
    }

    /// Decompiled virtual method #0 @ RVA 0x00385880 (18 instructions, 3 basic blocks)
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

    /// Decompiled virtual method #3 @ RVA 0x0038b3f0 (21 instructions, 7 basic blocks)
    pub fn method_03(&mut self) {
    }

}

impl CcClass for ccEventBtlState {
    fn class_name(&self) -> &'static str { Self::CLASS_NAME }
    fn vtable_rva(&self) -> u32 { Self::VTABLE_RVA }
    fn virtual_method_count(&self) -> usize { Self::METHOD_COUNT }
    fn base_classes(&self) -> &'static [&'static str] { Self::BASE_CLASSES }
}

/// Decompiled CC2 class: `ccEventBtlsklchk` (VTable RVA: 0x008b65c8, 4 virtual methods, 4 recovered fields)
/// Inherits: ccEventBase
#[derive(Debug, Clone, Default)]
pub struct ccEventBtlsklchk {
    pub vtable_rva: u32,
    pub field_0x0000: u64,
    pub field_0x0008: u16,
    pub field_0x000a: u8,
    pub field_0x0020: u32,
}

impl ccEventBtlsklchk {
    pub const CLASS_NAME: &'static str = "ccEventBtlsklchk";
    pub const VTABLE_RVA: u32 = 0x008b65c8;
    pub const METHOD_COUNT: usize = 4;
    pub const BASE_CLASSES: &'static [&'static str] = &["ccEventBase"];
    pub const VIRTUAL_METHODS: &'static [u64] = &[0x00000001803866f0, 0x0000000180380cf0, 0x00000001800d3090, 0x0000000180391400];

    pub fn new() -> Self {
        Self {
            vtable_rva: Self::VTABLE_RVA,
            field_0x0000: Default::default(),
            field_0x0008: Default::default(),
            field_0x000a: Default::default(),
            field_0x0020: Default::default(),
        }
    }

    /// Decompiled virtual method #0 @ RVA 0x003866f0 (18 instructions, 3 basic blocks)
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

    /// Decompiled virtual method #3 @ RVA 0x00391400 (29 instructions, 8 basic blocks)
    pub fn method_03(&mut self) {
    }

}

impl CcClass for ccEventBtlsklchk {
    fn class_name(&self) -> &'static str { Self::CLASS_NAME }
    fn vtable_rva(&self) -> u32 { Self::VTABLE_RVA }
    fn virtual_method_count(&self) -> usize { Self::METHOD_COUNT }
    fn base_classes(&self) -> &'static [&'static str] { Self::BASE_CLASSES }
}

/// Decompiled CC2 class: `ccEventCamera` (VTable RVA: 0x008b6d48, 4 virtual methods, 4 recovered fields)
/// Inherits: ccEventBase
#[derive(Debug, Clone, Default)]
pub struct ccEventCamera {
    pub vtable_rva: u32,
    pub field_0x0000: u64,
    pub field_0x0008: u16,
    pub field_0x000a: u8,
    pub field_0x0068: u64,
}

impl ccEventCamera {
    pub const CLASS_NAME: &'static str = "ccEventCamera";
    pub const VTABLE_RVA: u32 = 0x008b6d48;
    pub const METHOD_COUNT: usize = 4;
    pub const BASE_CLASSES: &'static [&'static str] = &["ccEventBase"];
    pub const VIRTUAL_METHODS: &'static [u64] = &[0x00000001803857c0, 0x0000000180380cf0, 0x00000001800d3090, 0x0000000180389100];

    pub fn new() -> Self {
        Self {
            vtable_rva: Self::VTABLE_RVA,
            field_0x0000: Default::default(),
            field_0x0008: Default::default(),
            field_0x000a: Default::default(),
            field_0x0068: Default::default(),
        }
    }

    /// Decompiled virtual method #0 @ RVA 0x003857c0 (18 instructions, 3 basic blocks)
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

    /// Decompiled virtual method #3 @ RVA 0x00389100 (99 instructions, 21 basic blocks)
    pub fn method_03(&mut self) {
    }

}

impl CcClass for ccEventCamera {
    fn class_name(&self) -> &'static str { Self::CLASS_NAME }
    fn vtable_rva(&self) -> u32 { Self::VTABLE_RVA }
    fn virtual_method_count(&self) -> usize { Self::METHOD_COUNT }
    fn base_classes(&self) -> &'static [&'static str] { Self::BASE_CLASSES }
}

/// Decompiled CC2 class: `ccEventChangemodel` (VTable RVA: 0x008b5738, 4 virtual methods, 3 recovered fields)
/// Inherits: ccEventBase
#[derive(Debug, Clone, Default)]
pub struct ccEventChangemodel {
    pub vtable_rva: u32,
    pub field_0x0000: u64,
    pub field_0x0008: u16,
    pub field_0x000a: u8,
}

impl ccEventChangemodel {
    pub const CLASS_NAME: &'static str = "ccEventChangemodel";
    pub const VTABLE_RVA: u32 = 0x008b5738;
    pub const METHOD_COUNT: usize = 4;
    pub const BASE_CLASSES: &'static [&'static str] = &["ccEventBase"];
    pub const VIRTUAL_METHODS: &'static [u64] = &[0x0000000180386870, 0x0000000180380cf0, 0x00000001800d3090, 0x0000000180391a60];

    pub fn new() -> Self {
        Self {
            vtable_rva: Self::VTABLE_RVA,
            field_0x0000: Default::default(),
            field_0x0008: Default::default(),
            field_0x000a: Default::default(),
        }
    }

    /// Decompiled virtual method #0 @ RVA 0x00386870 (18 instructions, 3 basic blocks)
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

    /// Decompiled virtual method #3 @ RVA 0x00391a60 (21 instructions, 4 basic blocks)
    pub fn method_03(&mut self) {
    }

}

impl CcClass for ccEventChangemodel {
    fn class_name(&self) -> &'static str { Self::CLASS_NAME }
    fn vtable_rva(&self) -> u32 { Self::VTABLE_RVA }
    fn virtual_method_count(&self) -> usize { Self::METHOD_COUNT }
    fn base_classes(&self) -> &'static [&'static str] { Self::BASE_CLASSES }
}

/// Decompiled CC2 class: `ccEventCheckKingChim` (VTable RVA: 0x008b5440, 4 virtual methods, 5 recovered fields)
/// Inherits: ccEventBase
#[derive(Debug, Clone, Default)]
pub struct ccEventCheckKingChim {
    pub vtable_rva: u32,
    pub field_0x0000: u64,
    pub field_0x0008: u16,
    pub field_0x000a: u8,
    pub field_0x0010: u64,
    pub field_0x0020: u32,
}

impl ccEventCheckKingChim {
    pub const CLASS_NAME: &'static str = "ccEventCheckKingChim";
    pub const VTABLE_RVA: u32 = 0x008b5440;
    pub const METHOD_COUNT: usize = 4;
    pub const BASE_CLASSES: &'static [&'static str] = &["ccEventBase"];
    pub const VIRTUAL_METHODS: &'static [u64] = &[0x0000000180386f00, 0x0000000180380cf0, 0x00000001800d3090, 0x0000000180394770];

    pub fn new() -> Self {
        Self {
            vtable_rva: Self::VTABLE_RVA,
            field_0x0000: Default::default(),
            field_0x0008: Default::default(),
            field_0x000a: Default::default(),
            field_0x0010: Default::default(),
            field_0x0020: Default::default(),
        }
    }

    /// Decompiled virtual method #0 @ RVA 0x00386f00 (18 instructions, 3 basic blocks)
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

    /// Decompiled virtual method #3 @ RVA 0x00394770 (26 instructions, 11 basic blocks)
    pub fn method_03(&mut self) {
        // Writes to field at offset +0x000a
    }

}

impl CcClass for ccEventCheckKingChim {
    fn class_name(&self) -> &'static str { Self::CLASS_NAME }
    fn vtable_rva(&self) -> u32 { Self::VTABLE_RVA }
    fn virtual_method_count(&self) -> usize { Self::METHOD_COUNT }
    fn base_classes(&self) -> &'static [&'static str] { Self::BASE_CLASSES }
}

/// Decompiled CC2 class: `ccEventCheckS101` (VTable RVA: 0x008b53a0, 4 virtual methods, 3 recovered fields)
/// Inherits: ccEventBase
#[derive(Debug, Clone, Default)]
pub struct ccEventCheckS101 {
    pub vtable_rva: u32,
    pub field_0x0000: u64,
    pub field_0x0008: u16,
    pub field_0x000a: u8,
}

impl ccEventCheckS101 {
    pub const CLASS_NAME: &'static str = "ccEventCheckS101";
    pub const VTABLE_RVA: u32 = 0x008b53a0;
    pub const METHOD_COUNT: usize = 4;
    pub const BASE_CLASSES: &'static [&'static str] = &["ccEventBase"];
    pub const VIRTUAL_METHODS: &'static [u64] = &[0x0000000180386e40, 0x0000000180380cf0, 0x00000001800d3090, 0x0000000180394350];

    pub fn new() -> Self {
        Self {
            vtable_rva: Self::VTABLE_RVA,
            field_0x0000: Default::default(),
            field_0x0008: Default::default(),
            field_0x000a: Default::default(),
        }
    }

    /// Decompiled virtual method #0 @ RVA 0x00386e40 (18 instructions, 3 basic blocks)
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

    /// Decompiled virtual method #3 @ RVA 0x00394350 (114 instructions, 17 basic blocks)
    pub fn method_03(&mut self) {
    }

}

impl CcClass for ccEventCheckS101 {
    fn class_name(&self) -> &'static str { Self::CLASS_NAME }
    fn vtable_rva(&self) -> u32 { Self::VTABLE_RVA }
    fn virtual_method_count(&self) -> usize { Self::METHOD_COUNT }
    fn base_classes(&self) -> &'static [&'static str] { Self::BASE_CLASSES }
}

/// Decompiled CC2 class: `ccEventChimCrown` (VTable RVA: 0x008b52d8, 4 virtual methods, 3 recovered fields)
/// Inherits: ccEventBase
#[derive(Debug, Clone, Default)]
pub struct ccEventChimCrown {
    pub vtable_rva: u32,
    pub field_0x0000: u64,
    pub field_0x0008: u16,
    pub field_0x000a: u8,
}

impl ccEventChimCrown {
    pub const CLASS_NAME: &'static str = "ccEventChimCrown";
    pub const VTABLE_RVA: u32 = 0x008b52d8;
    pub const METHOD_COUNT: usize = 4;
    pub const BASE_CLASSES: &'static [&'static str] = &["ccEventBase"];
    pub const VIRTUAL_METHODS: &'static [u64] = &[0x0000000180387000, 0x0000000180380cf0, 0x00000001800d3090, 0x0000000180394a90];

    pub fn new() -> Self {
        Self {
            vtable_rva: Self::VTABLE_RVA,
            field_0x0000: Default::default(),
            field_0x0008: Default::default(),
            field_0x000a: Default::default(),
        }
    }

    /// Decompiled virtual method #0 @ RVA 0x00387000 (18 instructions, 3 basic blocks)
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

    /// Decompiled virtual method #3 @ RVA 0x00394a90 (35 instructions, 9 basic blocks)
    pub fn method_03(&mut self) {
    }

}

impl CcClass for ccEventChimCrown {
    fn class_name(&self) -> &'static str { Self::CLASS_NAME }
    fn vtable_rva(&self) -> u32 { Self::VTABLE_RVA }
    fn virtual_method_count(&self) -> usize { Self::METHOD_COUNT }
    fn base_classes(&self) -> &'static [&'static str] { Self::BASE_CLASSES }
}

/// Decompiled CC2 class: `ccEventControl` (VTable RVA: 0x008b56e8, 4 virtual methods, 3 recovered fields)
/// Inherits: ccEventBase
#[derive(Debug, Clone, Default)]
pub struct ccEventControl {
    pub vtable_rva: u32,
    pub field_0x0000: u64,
    pub field_0x0008: u16,
    pub field_0x000a: u8,
}

impl ccEventControl {
    pub const CLASS_NAME: &'static str = "ccEventControl";
    pub const VTABLE_RVA: u32 = 0x008b56e8;
    pub const METHOD_COUNT: usize = 4;
    pub const BASE_CLASSES: &'static [&'static str] = &["ccEventBase"];
    pub const VIRTUAL_METHODS: &'static [u64] = &[0x00000001803868f0, 0x0000000180380cf0, 0x00000001800d3090, 0x00000001803920b0];

    pub fn new() -> Self {
        Self {
            vtable_rva: Self::VTABLE_RVA,
            field_0x0000: Default::default(),
            field_0x0008: Default::default(),
            field_0x000a: Default::default(),
        }
    }

    /// Decompiled virtual method #0 @ RVA 0x003868f0 (18 instructions, 3 basic blocks)
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

    /// Decompiled virtual method #3 @ RVA 0x003920b0 (28 instructions, 7 basic blocks)
    pub fn method_03(&mut self) {
    }

}

impl CcClass for ccEventControl {
    fn class_name(&self) -> &'static str { Self::CLASS_NAME }
    fn vtable_rva(&self) -> u32 { Self::VTABLE_RVA }
    fn virtual_method_count(&self) -> usize { Self::METHOD_COUNT }
    fn base_classes(&self) -> &'static [&'static str] { Self::BASE_CLASSES }
}

/// Decompiled CC2 class: `ccEventDeleteObj` (VTable RVA: 0x008b55f8, 4 virtual methods, 6 recovered fields)
/// Inherits: ccEventBase
#[derive(Debug, Clone, Default)]
pub struct ccEventDeleteObj {
    pub vtable_rva: u32,
    pub field_0x0000: u64,
    pub field_0x0008: u16,
    pub field_0x000a: u8,
    pub field_0x001c: u32,
    pub field_0x0020: u64,
    pub field_0x0800: u32,
}

impl ccEventDeleteObj {
    pub const CLASS_NAME: &'static str = "ccEventDeleteObj";
    pub const VTABLE_RVA: u32 = 0x008b55f8;
    pub const METHOD_COUNT: usize = 4;
    pub const BASE_CLASSES: &'static [&'static str] = &["ccEventBase"];
    pub const VIRTUAL_METHODS: &'static [u64] = &[0x0000000180386c40, 0x0000000180380cf0, 0x00000001800d3090, 0x0000000180393fe0];

    pub fn new() -> Self {
        Self {
            vtable_rva: Self::VTABLE_RVA,
            field_0x0000: Default::default(),
            field_0x0008: Default::default(),
            field_0x000a: Default::default(),
            field_0x001c: Default::default(),
            field_0x0020: Default::default(),
            field_0x0800: Default::default(),
        }
    }

    /// Decompiled virtual method #0 @ RVA 0x00386c40 (18 instructions, 3 basic blocks)
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

    /// Decompiled virtual method #3 @ RVA 0x00393fe0 (77 instructions, 10 basic blocks)
    pub fn method_03(&mut self) {
        // Writes to field at offset +0x0000
        // Writes to field at offset +0x001c
    }

}

impl CcClass for ccEventDeleteObj {
    fn class_name(&self) -> &'static str { Self::CLASS_NAME }
    fn vtable_rva(&self) -> u32 { Self::VTABLE_RVA }
    fn virtual_method_count(&self) -> usize { Self::METHOD_COUNT }
    fn base_classes(&self) -> &'static [&'static str] { Self::BASE_CLASSES }
}

/// Decompiled CC2 class: `ccEventDeleteObjCtgId` (VTable RVA: 0x008b55d0, 4 virtual methods, 4 recovered fields)
/// Inherits: ccEventBase
#[derive(Debug, Clone, Default)]
pub struct ccEventDeleteObjCtgId {
    pub vtable_rva: u32,
    pub field_0x0000: u64,
    pub field_0x0008: u16,
    pub field_0x000a: u8,
    pub field_0x0020: u64,
}

impl ccEventDeleteObjCtgId {
    pub const CLASS_NAME: &'static str = "ccEventDeleteObjCtgId";
    pub const VTABLE_RVA: u32 = 0x008b55d0;
    pub const METHOD_COUNT: usize = 4;
    pub const BASE_CLASSES: &'static [&'static str] = &["ccEventBase"];
    pub const VIRTUAL_METHODS: &'static [u64] = &[0x0000000180386c80, 0x0000000180380cf0, 0x00000001800d3090, 0x0000000180394110];

    pub fn new() -> Self {
        Self {
            vtable_rva: Self::VTABLE_RVA,
            field_0x0000: Default::default(),
            field_0x0008: Default::default(),
            field_0x000a: Default::default(),
            field_0x0020: Default::default(),
        }
    }

    /// Decompiled virtual method #0 @ RVA 0x00386c80 (18 instructions, 3 basic blocks)
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

    /// Decompiled virtual method #3 @ RVA 0x00394110 (38 instructions, 9 basic blocks)
    pub fn method_03(&mut self) {
    }

}

impl CcClass for ccEventDeleteObjCtgId {
    fn class_name(&self) -> &'static str { Self::CLASS_NAME }
    fn vtable_rva(&self) -> u32 { Self::VTABLE_RVA }
    fn virtual_method_count(&self) -> usize { Self::METHOD_COUNT }
    fn base_classes(&self) -> &'static [&'static str] { Self::BASE_CLASSES }
}

/// Decompiled CC2 class: `ccEventDemo` (VTable RVA: 0x008b3dd0, 4 virtual methods, 3 recovered fields)
/// Inherits: ccEventBase, ccEvFade
#[derive(Debug, Clone, Default)]
pub struct ccEventDemo {
    pub vtable_rva: u32,
    pub field_0x0000: u64,
    pub field_0x0008: u16,
    pub field_0x000a: u8,
}

impl ccEventDemo {
    pub const CLASS_NAME: &'static str = "ccEventDemo";
    pub const VTABLE_RVA: u32 = 0x008b3dd0;
    pub const METHOD_COUNT: usize = 4;
    pub const BASE_CLASSES: &'static [&'static str] = &["ccEventBase", "ccEvFade"];
    pub const VIRTUAL_METHODS: &'static [u64] = &[0x00000001803832e0, 0x0000000180380cf0, 0x00000001800d3090, 0x000000018038ff80];

    pub fn new() -> Self {
        Self {
            vtable_rva: Self::VTABLE_RVA,
            field_0x0000: Default::default(),
            field_0x0008: Default::default(),
            field_0x000a: Default::default(),
        }
    }

    /// Decompiled virtual method #0 @ RVA 0x003832e0 (23 instructions, 3 basic blocks)
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

    /// Decompiled virtual method #3 @ RVA 0x0038ff80 (23 instructions, 5 basic blocks)
    pub fn method_03(&mut self) {
    }

}

impl CcClass for ccEventDemo {
    fn class_name(&self) -> &'static str { Self::CLASS_NAME }
    fn vtable_rva(&self) -> u32 { Self::VTABLE_RVA }
    fn virtual_method_count(&self) -> usize { Self::METHOD_COUNT }
    fn base_classes(&self) -> &'static [&'static str] { Self::BASE_CLASSES }
}

/// Decompiled CC2 class: `ccEventDtbbsview` (VTable RVA: 0x008b5828, 4 virtual methods, 3 recovered fields)
/// Inherits: ccEventBase
#[derive(Debug, Clone, Default)]
pub struct ccEventDtbbsview {
    pub vtable_rva: u32,
    pub field_0x0000: u64,
    pub field_0x0008: u16,
    pub field_0x000a: u8,
}

impl ccEventDtbbsview {
    pub const CLASS_NAME: &'static str = "ccEventDtbbsview";
    pub const VTABLE_RVA: u32 = 0x008b5828;
    pub const METHOD_COUNT: usize = 4;
    pub const BASE_CLASSES: &'static [&'static str] = &["ccEventBase"];
    pub const VIRTUAL_METHODS: &'static [u64] = &[0x00000001803867b0, 0x0000000180380cf0, 0x00000001800d3090, 0x0000000180391660];

    pub fn new() -> Self {
        Self {
            vtable_rva: Self::VTABLE_RVA,
            field_0x0000: Default::default(),
            field_0x0008: Default::default(),
            field_0x000a: Default::default(),
        }
    }

    /// Decompiled virtual method #0 @ RVA 0x003867b0 (18 instructions, 3 basic blocks)
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

    /// Decompiled virtual method #3 @ RVA 0x00391660 (24 instructions, 5 basic blocks)
    pub fn method_03(&mut self) {
    }

}

impl CcClass for ccEventDtbbsview {
    fn class_name(&self) -> &'static str { Self::CLASS_NAME }
    fn vtable_rva(&self) -> u32 { Self::VTABLE_RVA }
    fn virtual_method_count(&self) -> usize { Self::METHOD_COUNT }
    fn base_classes(&self) -> &'static [&'static str] { Self::BASE_CLASSES }
}

/// Decompiled CC2 class: `ccEventDtmailer` (VTable RVA: 0x008b5878, 4 virtual methods, 3 recovered fields)
/// Inherits: ccEventBase
#[derive(Debug, Clone, Default)]
pub struct ccEventDtmailer {
    pub vtable_rva: u32,
    pub field_0x0000: u64,
    pub field_0x0008: u16,
    pub field_0x000a: u8,
}

impl ccEventDtmailer {
    pub const CLASS_NAME: &'static str = "ccEventDtmailer";
    pub const VTABLE_RVA: u32 = 0x008b5878;
    pub const METHOD_COUNT: usize = 4;
    pub const BASE_CLASSES: &'static [&'static str] = &["ccEventBase"];
    pub const VIRTUAL_METHODS: &'static [u64] = &[0x0000000180386770, 0x0000000180380cf0, 0x00000001800d3090, 0x00000001803915c0];

    pub fn new() -> Self {
        Self {
            vtable_rva: Self::VTABLE_RVA,
            field_0x0000: Default::default(),
            field_0x0008: Default::default(),
            field_0x000a: Default::default(),
        }
    }

    /// Decompiled virtual method #0 @ RVA 0x00386770 (18 instructions, 3 basic blocks)
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

    /// Decompiled virtual method #3 @ RVA 0x003915c0 (22 instructions, 5 basic blocks)
    pub fn method_03(&mut self) {
    }

}

impl CcClass for ccEventDtmailer {
    fn class_name(&self) -> &'static str { Self::CLASS_NAME }
    fn vtable_rva(&self) -> u32 { Self::VTABLE_RVA }
    fn virtual_method_count(&self) -> usize { Self::METHOD_COUNT }
    fn base_classes(&self) -> &'static [&'static str] { Self::BASE_CLASSES }
}

/// Decompiled CC2 class: `ccEventDtmenuseal` (VTable RVA: 0x008b5850, 4 virtual methods, 3 recovered fields)
/// Inherits: ccEventBase
#[derive(Debug, Clone, Default)]
pub struct ccEventDtmenuseal {
    pub vtable_rva: u32,
    pub field_0x0000: u64,
    pub field_0x0008: u16,
    pub field_0x000a: u8,
}

impl ccEventDtmenuseal {
    pub const CLASS_NAME: &'static str = "ccEventDtmenuseal";
    pub const VTABLE_RVA: u32 = 0x008b5850;
    pub const METHOD_COUNT: usize = 4;
    pub const BASE_CLASSES: &'static [&'static str] = &["ccEventBase"];
    pub const VIRTUAL_METHODS: &'static [u64] = &[0x0000000180386730, 0x0000000180380cf0, 0x00000001800d3090, 0x00000001803914b0];

    pub fn new() -> Self {
        Self {
            vtable_rva: Self::VTABLE_RVA,
            field_0x0000: Default::default(),
            field_0x0008: Default::default(),
            field_0x000a: Default::default(),
        }
    }

    /// Decompiled virtual method #0 @ RVA 0x00386730 (18 instructions, 3 basic blocks)
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

    /// Decompiled virtual method #3 @ RVA 0x003914b0 (31 instructions, 8 basic blocks)
    pub fn method_03(&mut self) {
    }

}

impl CcClass for ccEventDtmenuseal {
    fn class_name(&self) -> &'static str { Self::CLASS_NAME }
    fn vtable_rva(&self) -> u32 { Self::VTABLE_RVA }
    fn virtual_method_count(&self) -> usize { Self::METHOD_COUNT }
    fn base_classes(&self) -> &'static [&'static str] { Self::BASE_CLASSES }
}

/// Decompiled CC2 class: `ccEventDtmsgwindow` (VTable RVA: 0x008b5800, 4 virtual methods, 3 recovered fields)
/// Inherits: ccEventBase
#[derive(Debug, Clone, Default)]
pub struct ccEventDtmsgwindow {
    pub vtable_rva: u32,
    pub field_0x0000: u64,
    pub field_0x0008: u16,
    pub field_0x000a: u8,
}

impl ccEventDtmsgwindow {
    pub const CLASS_NAME: &'static str = "ccEventDtmsgwindow";
    pub const VTABLE_RVA: u32 = 0x008b5800;
    pub const METHOD_COUNT: usize = 4;
    pub const BASE_CLASSES: &'static [&'static str] = &["ccEventBase"];
    pub const VIRTUAL_METHODS: &'static [u64] = &[0x00000001803867f0, 0x0000000180380cf0, 0x00000001800d3090, 0x0000000180391830];

    pub fn new() -> Self {
        Self {
            vtable_rva: Self::VTABLE_RVA,
            field_0x0000: Default::default(),
            field_0x0008: Default::default(),
            field_0x000a: Default::default(),
        }
    }

    /// Decompiled virtual method #0 @ RVA 0x003867f0 (18 instructions, 3 basic blocks)
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

    /// Decompiled virtual method #3 @ RVA 0x00391830 (12 instructions, 2 basic blocks)
    pub fn method_03(&mut self) {
    }

}

impl CcClass for ccEventDtmsgwindow {
    fn class_name(&self) -> &'static str { Self::CLASS_NAME }
    fn vtable_rva(&self) -> u32 { Self::VTABLE_RVA }
    fn virtual_method_count(&self) -> usize { Self::METHOD_COUNT }
    fn base_classes(&self) -> &'static [&'static str] { Self::BASE_CLASSES }
}

/// Decompiled CC2 class: `ccEventEnemy` (VTable RVA: 0x008b6820, 4 virtual methods, 8 recovered fields)
/// Inherits: ccEventBase
#[derive(Debug, Clone, Default)]
pub struct ccEventEnemy {
    pub vtable_rva: u32,
    pub field_0x0000: u64,
    pub field_0x0008: u16,
    pub field_0x000a: u8,
    pub field_0x0010: u64,
    pub field_0x0018: u64,
    pub field_0x001c: u32,
    pub field_0x002c: u16,
    pub field_0x0040: u32,
}

impl ccEventEnemy {
    pub const CLASS_NAME: &'static str = "ccEventEnemy";
    pub const VTABLE_RVA: u32 = 0x008b6820;
    pub const METHOD_COUNT: usize = 4;
    pub const BASE_CLASSES: &'static [&'static str] = &["ccEventBase"];
    pub const VIRTUAL_METHODS: &'static [u64] = &[0x0000000180386140, 0x0000000180380cf0, 0x00000001800d3090, 0x000000018038f050];

    pub fn new() -> Self {
        Self {
            vtable_rva: Self::VTABLE_RVA,
            field_0x0000: Default::default(),
            field_0x0008: Default::default(),
            field_0x000a: Default::default(),
            field_0x0010: Default::default(),
            field_0x0018: Default::default(),
            field_0x001c: Default::default(),
            field_0x002c: Default::default(),
            field_0x0040: Default::default(),
        }
    }

    /// Decompiled virtual method #0 @ RVA 0x00386140 (18 instructions, 3 basic blocks)
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

    /// Decompiled virtual method #3 @ RVA 0x0038f050 (85 instructions, 28 basic blocks)
    pub fn method_03(&mut self) {
        // Writes to field at offset +0x001c
        // Writes to field at offset +0x0040
    }

}

impl CcClass for ccEventEnemy {
    fn class_name(&self) -> &'static str { Self::CLASS_NAME }
    fn vtable_rva(&self) -> u32 { Self::VTABLE_RVA }
    fn virtual_method_count(&self) -> usize { Self::METHOD_COUNT }
    fn base_classes(&self) -> &'static [&'static str] { Self::BASE_CLASSES }
}

/// Decompiled CC2 class: `ccEventEntry` (VTable RVA: 0x008b5670, 4 virtual methods, 8 recovered fields)
/// Inherits: ccEventBase
#[derive(Debug, Clone, Default)]
pub struct ccEventEntry {
    pub vtable_rva: u32,
    pub field_0x0000: u64,
    pub field_0x0004: u16,
    pub field_0x0006: u16,
    pub field_0x0008: u16,
    pub field_0x000a: u8,
    pub field_0x000c: u32,
    pub field_0x0010: u16,
    pub field_0x0014: u32,
}

impl ccEventEntry {
    pub const CLASS_NAME: &'static str = "ccEventEntry";
    pub const VTABLE_RVA: u32 = 0x008b5670;
    pub const METHOD_COUNT: usize = 4;
    pub const BASE_CLASSES: &'static [&'static str] = &["ccEventBase"];
    pub const VIRTUAL_METHODS: &'static [u64] = &[0x0000000180386b40, 0x0000000180380cf0, 0x00000001800d3090, 0x0000000180393520];

    pub fn new() -> Self {
        Self {
            vtable_rva: Self::VTABLE_RVA,
            field_0x0000: Default::default(),
            field_0x0004: Default::default(),
            field_0x0006: Default::default(),
            field_0x0008: Default::default(),
            field_0x000a: Default::default(),
            field_0x000c: Default::default(),
            field_0x0010: Default::default(),
            field_0x0014: Default::default(),
        }
    }

    /// Decompiled virtual method #0 @ RVA 0x00386b40 (18 instructions, 3 basic blocks)
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

    /// Decompiled virtual method #3 @ RVA 0x00393520 (117 instructions, 23 basic blocks)
    pub fn method_03(&mut self) {
        // Writes to field at offset +0x0004
        // Writes to field at offset +0x0006
        // Writes to field at offset +0x0008
        // Writes to field at offset +0x0010
        // Writes to field at offset +0x0014
    }

}

impl CcClass for ccEventEntry {
    fn class_name(&self) -> &'static str { Self::CLASS_NAME }
    fn vtable_rva(&self) -> u32 { Self::VTABLE_RVA }
    fn virtual_method_count(&self) -> usize { Self::METHOD_COUNT }
    fn base_classes(&self) -> &'static [&'static str] { Self::BASE_CLASSES }
}

/// Decompiled CC2 class: `ccEventEquip` (VTable RVA: 0x008b6c08, 4 virtual methods, 3 recovered fields)
/// Inherits: ccEventBase
#[derive(Debug, Clone, Default)]
pub struct ccEventEquip {
    pub vtable_rva: u32,
    pub field_0x0000: u64,
    pub field_0x0008: u16,
    pub field_0x000a: u8,
}

impl ccEventEquip {
    pub const CLASS_NAME: &'static str = "ccEventEquip";
    pub const VTABLE_RVA: u32 = 0x008b6c08;
    pub const METHOD_COUNT: usize = 4;
    pub const BASE_CLASSES: &'static [&'static str] = &["ccEventBase"];
    pub const VIRTUAL_METHODS: &'static [u64] = &[0x0000000180385980, 0x0000000180380cf0, 0x00000001800d3090, 0x000000018038bd20];

    pub fn new() -> Self {
        Self {
            vtable_rva: Self::VTABLE_RVA,
            field_0x0000: Default::default(),
            field_0x0008: Default::default(),
            field_0x000a: Default::default(),
        }
    }

    /// Decompiled virtual method #0 @ RVA 0x00385980 (18 instructions, 3 basic blocks)
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

    /// Decompiled virtual method #3 @ RVA 0x0038bd20 (18 instructions, 2 basic blocks)
    pub fn method_03(&mut self) {
    }

}

impl CcClass for ccEventEquip {
    fn class_name(&self) -> &'static str { Self::CLASS_NAME }
    fn vtable_rva(&self) -> u32 { Self::VTABLE_RVA }
    fn virtual_method_count(&self) -> usize { Self::METHOD_COUNT }
    fn base_classes(&self) -> &'static [&'static str] { Self::BASE_CLASSES }
}

/// Decompiled CC2 class: `ccEventFade` (VTable RVA: 0x008b6738, 4 virtual methods, 3 recovered fields)
/// Inherits: ccEventBase, ccEvFade
#[derive(Debug, Clone, Default)]
pub struct ccEventFade {
    pub vtable_rva: u32,
    pub field_0x0000: u64,
    pub field_0x0008: u16,
    pub field_0x000a: u8,
}

impl ccEventFade {
    pub const CLASS_NAME: &'static str = "ccEventFade";
    pub const VTABLE_RVA: u32 = 0x008b6738;
    pub const METHOD_COUNT: usize = 4;
    pub const BASE_CLASSES: &'static [&'static str] = &["ccEventBase", "ccEvFade"];
    pub const VIRTUAL_METHODS: &'static [u64] = &[0x0000000180386200, 0x0000000180380cf0, 0x00000001800d3090, 0x000000018038fad0];

    pub fn new() -> Self {
        Self {
            vtable_rva: Self::VTABLE_RVA,
            field_0x0000: Default::default(),
            field_0x0008: Default::default(),
            field_0x000a: Default::default(),
        }
    }

    /// Decompiled virtual method #0 @ RVA 0x00386200 (23 instructions, 3 basic blocks)
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

    /// Decompiled virtual method #3 @ RVA 0x0038fad0 (28 instructions, 7 basic blocks)
    pub fn method_03(&mut self) {
    }

}

impl CcClass for ccEventFade {
    fn class_name(&self) -> &'static str { Self::CLASS_NAME }
    fn vtable_rva(&self) -> u32 { Self::VTABLE_RVA }
    fn virtual_method_count(&self) -> usize { Self::METHOD_COUNT }
    fn base_classes(&self) -> &'static [&'static str] { Self::BASE_CLASSES }
}

/// Decompiled CC2 class: `ccEventFlag` (VTable RVA: 0x008b67d0, 4 virtual methods, 3 recovered fields)
/// Inherits: ccEventBase
#[derive(Debug, Clone, Default)]
pub struct ccEventFlag {
    pub vtable_rva: u32,
    pub field_0x0000: u64,
    pub field_0x0008: u16,
    pub field_0x000a: u8,
}

impl ccEventFlag {
    pub const CLASS_NAME: &'static str = "ccEventFlag";
    pub const VTABLE_RVA: u32 = 0x008b67d0;
    pub const METHOD_COUNT: usize = 4;
    pub const BASE_CLASSES: &'static [&'static str] = &["ccEventBase"];
    pub const VIRTUAL_METHODS: &'static [u64] = &[0x0000000180386080, 0x0000000180380cf0, 0x00000001800d3090, 0x000000018038e890];

    pub fn new() -> Self {
        Self {
            vtable_rva: Self::VTABLE_RVA,
            field_0x0000: Default::default(),
            field_0x0008: Default::default(),
            field_0x000a: Default::default(),
        }
    }

    /// Decompiled virtual method #0 @ RVA 0x00386080 (18 instructions, 3 basic blocks)
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

    /// Decompiled virtual method #3 @ RVA 0x0038e890 (12 instructions, 2 basic blocks)
    pub fn method_03(&mut self) {
    }

}

impl CcClass for ccEventFlag {
    fn class_name(&self) -> &'static str { Self::CLASS_NAME }
    fn vtable_rva(&self) -> u32 { Self::VTABLE_RVA }
    fn virtual_method_count(&self) -> usize { Self::METHOD_COUNT }
    fn base_classes(&self) -> &'static [&'static str] { Self::BASE_CLASSES }
}

/// Decompiled CC2 class: `ccEventGenerator` (VTable RVA: 0x008b6d98, 4 virtual methods, 3 recovered fields)
/// Inherits: ccEventBase
#[derive(Debug, Clone, Default)]
pub struct ccEventGenerator {
    pub vtable_rva: u32,
    pub field_0x0000: u64,
    pub field_0x0008: u16,
    pub field_0x000a: u8,
}

impl ccEventGenerator {
    pub const CLASS_NAME: &'static str = "ccEventGenerator";
    pub const VTABLE_RVA: u32 = 0x008b6d98;
    pub const METHOD_COUNT: usize = 4;
    pub const BASE_CLASSES: &'static [&'static str] = &["ccEventBase"];
    pub const VIRTUAL_METHODS: &'static [u64] = &[0x0000000180385500, 0x0000000180380cf0, 0x00000001800d3090, 0x0000000180388000];

    pub fn new() -> Self {
        Self {
            vtable_rva: Self::VTABLE_RVA,
            field_0x0000: Default::default(),
            field_0x0008: Default::default(),
            field_0x000a: Default::default(),
        }
    }

    /// Decompiled virtual method #0 @ RVA 0x00385500 (18 instructions, 3 basic blocks)
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

    /// Decompiled virtual method #3 @ RVA 0x00388000 (23 instructions, 7 basic blocks)
    pub fn method_03(&mut self) {
    }

}

impl CcClass for ccEventGenerator {
    fn class_name(&self) -> &'static str { Self::CLASS_NAME }
    fn vtable_rva(&self) -> u32 { Self::VTABLE_RVA }
    fn virtual_method_count(&self) -> usize { Self::METHOD_COUNT }
    fn base_classes(&self) -> &'static [&'static str] { Self::BASE_CLASSES }
}

/// Decompiled CC2 class: `ccEventGimmick` (VTable RVA: 0x008b6848, 4 virtual methods, 4 recovered fields)
/// Inherits: ccEventBase
#[derive(Debug, Clone, Default)]
pub struct ccEventGimmick {
    pub vtable_rva: u32,
    pub field_0x0000: u64,
    pub field_0x0008: u16,
    pub field_0x000a: u8,
    pub field_0x000c: u32,
}

impl ccEventGimmick {
    pub const CLASS_NAME: &'static str = "ccEventGimmick";
    pub const VTABLE_RVA: u32 = 0x008b6848;
    pub const METHOD_COUNT: usize = 4;
    pub const BASE_CLASSES: &'static [&'static str] = &["ccEventBase"];
    pub const VIRTUAL_METHODS: &'static [u64] = &[0x0000000180386180, 0x0000000180380cf0, 0x00000001800d3090, 0x000000018038f330];

    pub fn new() -> Self {
        Self {
            vtable_rva: Self::VTABLE_RVA,
            field_0x0000: Default::default(),
            field_0x0008: Default::default(),
            field_0x000a: Default::default(),
            field_0x000c: Default::default(),
        }
    }

    /// Decompiled virtual method #0 @ RVA 0x00386180 (18 instructions, 3 basic blocks)
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

    /// Decompiled virtual method #3 @ RVA 0x0038f330 (110 instructions, 22 basic blocks)
    pub fn method_03(&mut self) {
    }

}

impl CcClass for ccEventGimmick {
    fn class_name(&self) -> &'static str { Self::CLASS_NAME }
    fn vtable_rva(&self) -> u32 { Self::VTABLE_RVA }
    fn virtual_method_count(&self) -> usize { Self::METHOD_COUNT }
    fn base_classes(&self) -> &'static [&'static str] { Self::BASE_CLASSES }
}

/// Decompiled CC2 class: `ccEventGuild` (VTable RVA: 0x008b57d8, 4 virtual methods, 3 recovered fields)
/// Inherits: ccEventBase
#[derive(Debug, Clone, Default)]
pub struct ccEventGuild {
    pub vtable_rva: u32,
    pub field_0x0000: u64,
    pub field_0x0008: u16,
    pub field_0x000a: u8,
}

impl ccEventGuild {
    pub const CLASS_NAME: &'static str = "ccEventGuild";
    pub const VTABLE_RVA: u32 = 0x008b57d8;
    pub const METHOD_COUNT: usize = 4;
    pub const BASE_CLASSES: &'static [&'static str] = &["ccEventBase"];
    pub const VIRTUAL_METHODS: &'static [u64] = &[0x0000000180386970, 0x0000000180380cf0, 0x00000001800d3090, 0x0000000180392640];

    pub fn new() -> Self {
        Self {
            vtable_rva: Self::VTABLE_RVA,
            field_0x0000: Default::default(),
            field_0x0008: Default::default(),
            field_0x000a: Default::default(),
        }
    }

    /// Decompiled virtual method #0 @ RVA 0x00386970 (18 instructions, 3 basic blocks)
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

    /// Decompiled virtual method #3 @ RVA 0x00392640 (12 instructions, 2 basic blocks)
    pub fn method_03(&mut self) {
    }

}

impl CcClass for ccEventGuild {
    fn class_name(&self) -> &'static str { Self::CLASS_NAME }
    fn vtable_rva(&self) -> u32 { Self::VTABLE_RVA }
    fn virtual_method_count(&self) -> usize { Self::METHOD_COUNT }
    fn base_classes(&self) -> &'static [&'static str] { Self::BASE_CLASSES }
}

/// Decompiled CC2 class: `ccEventGuildPlant` (VTable RVA: 0x008b5350, 4 virtual methods, 3 recovered fields)
/// Inherits: ccEventBase
#[derive(Debug, Clone, Default)]
pub struct ccEventGuildPlant {
    pub vtable_rva: u32,
    pub field_0x0000: u64,
    pub field_0x0008: u16,
    pub field_0x000a: u8,
}

impl ccEventGuildPlant {
    pub const CLASS_NAME: &'static str = "ccEventGuildPlant";
    pub const VTABLE_RVA: u32 = 0x008b5350;
    pub const METHOD_COUNT: usize = 4;
    pub const BASE_CLASSES: &'static [&'static str] = &["ccEventBase"];
    pub const VIRTUAL_METHODS: &'static [u64] = &[0x0000000180387100, 0x0000000180380cf0, 0x00000001800d3090, 0x0000000180394e70];

    pub fn new() -> Self {
        Self {
            vtable_rva: Self::VTABLE_RVA,
            field_0x0000: Default::default(),
            field_0x0008: Default::default(),
            field_0x000a: Default::default(),
        }
    }

    /// Decompiled virtual method #0 @ RVA 0x00387100 (18 instructions, 3 basic blocks)
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

    /// Decompiled virtual method #3 @ RVA 0x00394e70 (53 instructions, 16 basic blocks)
    pub fn method_03(&mut self) {
    }

}

impl CcClass for ccEventGuildPlant {
    fn class_name(&self) -> &'static str { Self::CLASS_NAME }
    fn vtable_rva(&self) -> u32 { Self::VTABLE_RVA }
    fn virtual_method_count(&self) -> usize { Self::METHOD_COUNT }
    fn base_classes(&self) -> &'static [&'static str] { Self::BASE_CLASSES }
}

/// Decompiled CC2 class: `ccEventHasewoInit` (VTable RVA: 0x008b55a8, 4 virtual methods, 3 recovered fields)
/// Inherits: ccEventBase
#[derive(Debug, Clone, Default)]
pub struct ccEventHasewoInit {
    pub vtable_rva: u32,
    pub field_0x0000: u64,
    pub field_0x0008: u16,
    pub field_0x000a: u8,
}

impl ccEventHasewoInit {
    pub const CLASS_NAME: &'static str = "ccEventHasewoInit";
    pub const VTABLE_RVA: u32 = 0x008b55a8;
    pub const METHOD_COUNT: usize = 4;
    pub const BASE_CLASSES: &'static [&'static str] = &["ccEventBase"];
    pub const VIRTUAL_METHODS: &'static [u64] = &[0x0000000180386bc0, 0x0000000180380cf0, 0x00000001800d3090, 0x0000000180393a90];

    pub fn new() -> Self {
        Self {
            vtable_rva: Self::VTABLE_RVA,
            field_0x0000: Default::default(),
            field_0x0008: Default::default(),
            field_0x000a: Default::default(),
        }
    }

    /// Decompiled virtual method #0 @ RVA 0x00386bc0 (18 instructions, 3 basic blocks)
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

    /// Decompiled virtual method #3 @ RVA 0x00393a90 (37 instructions, 8 basic blocks)
    pub fn method_03(&mut self) {
    }

}

impl CcClass for ccEventHasewoInit {
    fn class_name(&self) -> &'static str { Self::CLASS_NAME }
    fn vtable_rva(&self) -> u32 { Self::VTABLE_RVA }
    fn virtual_method_count(&self) -> usize { Self::METHOD_COUNT }
    fn base_classes(&self) -> &'static [&'static str] { Self::BASE_CLASSES }
}

/// Decompiled CC2 class: `ccEventIsMsg` (VTable RVA: 0x008b6e10, 4 virtual methods, 3 recovered fields)
/// Inherits: ccEventBase
#[derive(Debug, Clone, Default)]
pub struct ccEventIsMsg {
    pub vtable_rva: u32,
    pub field_0x0000: u64,
    pub field_0x0008: u16,
    pub field_0x000a: u8,
}

impl ccEventIsMsg {
    pub const CLASS_NAME: &'static str = "ccEventIsMsg";
    pub const VTABLE_RVA: u32 = 0x008b6e10;
    pub const METHOD_COUNT: usize = 4;
    pub const BASE_CLASSES: &'static [&'static str] = &["ccEventBase"];
    pub const VIRTUAL_METHODS: &'static [u64] = &[0x0000000180385600, 0x0000000180380cf0, 0x00000001800d3090, 0x0000000180388d10];

    pub fn new() -> Self {
        Self {
            vtable_rva: Self::VTABLE_RVA,
            field_0x0000: Default::default(),
            field_0x0008: Default::default(),
            field_0x000a: Default::default(),
        }
    }

    /// Decompiled virtual method #0 @ RVA 0x00385600 (18 instructions, 3 basic blocks)
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

    /// Decompiled virtual method #3 @ RVA 0x00388d10 (21 instructions, 4 basic blocks)
    pub fn method_03(&mut self) {
    }

}

impl CcClass for ccEventIsMsg {
    fn class_name(&self) -> &'static str { Self::CLASS_NAME }
    fn vtable_rva(&self) -> u32 { Self::VTABLE_RVA }
    fn virtual_method_count(&self) -> usize { Self::METHOD_COUNT }
    fn base_classes(&self) -> &'static [&'static str] { Self::BASE_CLASSES }
}

/// Decompiled CC2 class: `ccEventItem` (VTable RVA: 0x008b6be0, 4 virtual methods, 3 recovered fields)
/// Inherits: ccEventBase
#[derive(Debug, Clone, Default)]
pub struct ccEventItem {
    pub vtable_rva: u32,
    pub field_0x0000: u64,
    pub field_0x0008: u16,
    pub field_0x000a: u8,
}

impl ccEventItem {
    pub const CLASS_NAME: &'static str = "ccEventItem";
    pub const VTABLE_RVA: u32 = 0x008b6be0;
    pub const METHOD_COUNT: usize = 4;
    pub const BASE_CLASSES: &'static [&'static str] = &["ccEventBase"];
    pub const VIRTUAL_METHODS: &'static [u64] = &[0x0000000180385940, 0x0000000180380cf0, 0x00000001800d3090, 0x000000018038b820];

    pub fn new() -> Self {
        Self {
            vtable_rva: Self::VTABLE_RVA,
            field_0x0000: Default::default(),
            field_0x0008: Default::default(),
            field_0x000a: Default::default(),
        }
    }

    /// Decompiled virtual method #0 @ RVA 0x00385940 (18 instructions, 3 basic blocks)
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

    /// Decompiled virtual method #3 @ RVA 0x0038b820 (121 instructions, 15 basic blocks)
    pub fn method_03(&mut self) {
    }

}

impl CcClass for ccEventItem {
    fn class_name(&self) -> &'static str { Self::CLASS_NAME }
    fn vtable_rva(&self) -> u32 { Self::VTABLE_RVA }
    fn virtual_method_count(&self) -> usize { Self::METHOD_COUNT }
    fn base_classes(&self) -> &'static [&'static str] { Self::BASE_CLASSES }
}

/// Decompiled CC2 class: `ccEventKey` (VTable RVA: 0x008b6cf8, 4 virtual methods, 3 recovered fields)
/// Inherits: ccEventBase
#[derive(Debug, Clone, Default)]
pub struct ccEventKey {
    pub vtable_rva: u32,
    pub field_0x0000: u64,
    pub field_0x0008: u16,
    pub field_0x000a: u8,
}

impl ccEventKey {
    pub const CLASS_NAME: &'static str = "ccEventKey";
    pub const VTABLE_RVA: u32 = 0x008b6cf8;
    pub const METHOD_COUNT: usize = 4;
    pub const BASE_CLASSES: &'static [&'static str] = &["ccEventBase"];
    pub const VIRTUAL_METHODS: &'static [u64] = &[0x0000000180385680, 0x0000000180380cf0, 0x00000001800d3090, 0x0000000180388f60];

    pub fn new() -> Self {
        Self {
            vtable_rva: Self::VTABLE_RVA,
            field_0x0000: Default::default(),
            field_0x0008: Default::default(),
            field_0x000a: Default::default(),
        }
    }

    /// Decompiled virtual method #0 @ RVA 0x00385680 (18 instructions, 3 basic blocks)
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

    /// Decompiled virtual method #3 @ RVA 0x00388f60 (19 instructions, 4 basic blocks)
    pub fn method_03(&mut self) {
    }

}

impl CcClass for ccEventKey {
    fn class_name(&self) -> &'static str { Self::CLASS_NAME }
    fn vtable_rva(&self) -> u32 { Self::VTABLE_RVA }
    fn virtual_method_count(&self) -> usize { Self::METHOD_COUNT }
    fn base_classes(&self) -> &'static [&'static str] { Self::BASE_CLASSES }
}

/// Decompiled CC2 class: `ccEventLvSetCooPai` (VTable RVA: 0x008b5558, 4 virtual methods, 3 recovered fields)
/// Inherits: ccEventBase
#[derive(Debug, Clone, Default)]
pub struct ccEventLvSetCooPai {
    pub vtable_rva: u32,
    pub field_0x0000: u64,
    pub field_0x0008: u16,
    pub field_0x000a: u8,
}

impl ccEventLvSetCooPai {
    pub const CLASS_NAME: &'static str = "ccEventLvSetCooPai";
    pub const VTABLE_RVA: u32 = 0x008b5558;
    pub const METHOD_COUNT: usize = 4;
    pub const BASE_CLASSES: &'static [&'static str] = &["ccEventBase"];
    pub const VIRTUAL_METHODS: &'static [u64] = &[0x0000000180386c00, 0x0000000180380cf0, 0x00000001800d3090, 0x0000000180393b10];

    pub fn new() -> Self {
        Self {
            vtable_rva: Self::VTABLE_RVA,
            field_0x0000: Default::default(),
            field_0x0008: Default::default(),
            field_0x000a: Default::default(),
        }
    }

    /// Decompiled virtual method #0 @ RVA 0x00386c00 (18 instructions, 3 basic blocks)
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

    /// Decompiled virtual method #3 @ RVA 0x00393b10 (125 instructions, 12 basic blocks)
    pub fn method_03(&mut self) {
    }

}

impl CcClass for ccEventLvSetCooPai {
    fn class_name(&self) -> &'static str { Self::CLASS_NAME }
    fn vtable_rva(&self) -> u32 { Self::VTABLE_RVA }
    fn virtual_method_count(&self) -> usize { Self::METHOD_COUNT }
    fn base_classes(&self) -> &'static [&'static str] { Self::BASE_CLASSES }
}

/// Decompiled CC2 class: `ccEventM133` (VTable RVA: 0x008b5760, 4 virtual methods, 4 recovered fields)
/// Inherits: ccEventBase
#[derive(Debug, Clone, Default)]
pub struct ccEventM133 {
    pub vtable_rva: u32,
    pub field_0x0000: u64,
    pub field_0x0008: u16,
    pub field_0x000a: u8,
    pub field_0x0030: u64,
}

impl ccEventM133 {
    pub const CLASS_NAME: &'static str = "ccEventM133";
    pub const VTABLE_RVA: u32 = 0x008b5760;
    pub const METHOD_COUNT: usize = 4;
    pub const BASE_CLASSES: &'static [&'static str] = &["ccEventBase"];
    pub const VIRTUAL_METHODS: &'static [u64] = &[0x00000001803869b0, 0x0000000180380cf0, 0x00000001800d3090, 0x00000001803927e0];

    pub fn new() -> Self {
        Self {
            vtable_rva: Self::VTABLE_RVA,
            field_0x0000: Default::default(),
            field_0x0008: Default::default(),
            field_0x000a: Default::default(),
            field_0x0030: Default::default(),
        }
    }

    /// Decompiled virtual method #0 @ RVA 0x003869b0 (38 instructions, 6 basic blocks)
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

    /// Decompiled virtual method #3 @ RVA 0x003927e0 (21 instructions, 6 basic blocks)
    pub fn method_03(&mut self) {
    }

}

impl CcClass for ccEventM133 {
    fn class_name(&self) -> &'static str { Self::CLASS_NAME }
    fn vtable_rva(&self) -> u32 { Self::VTABLE_RVA }
    fn virtual_method_count(&self) -> usize { Self::METHOD_COUNT }
    fn base_classes(&self) -> &'static [&'static str] { Self::BASE_CLASSES }
}

/// Decompiled CC2 class: `ccEventM133CastDelete` (VTable RVA: 0x008b5620, 4 virtual methods, 7 recovered fields)
/// Inherits: ccEventBase
#[derive(Debug, Clone, Default)]
pub struct ccEventM133CastDelete {
    pub vtable_rva: u32,
    pub field_0x0000: u64,
    pub field_0x0008: u16,
    pub field_0x000a: u8,
    pub field_0x0010: u16,
    pub field_0x0018: u64,
    pub field_0x0020: u32,
    pub field_0x0040: u32,
}

impl ccEventM133CastDelete {
    pub const CLASS_NAME: &'static str = "ccEventM133CastDelete";
    pub const VTABLE_RVA: u32 = 0x008b5620;
    pub const METHOD_COUNT: usize = 4;
    pub const BASE_CLASSES: &'static [&'static str] = &["ccEventBase"];
    pub const VIRTUAL_METHODS: &'static [u64] = &[0x0000000180386a80, 0x0000000180380cf0, 0x00000001800d3090, 0x0000000180392e20];

    pub fn new() -> Self {
        Self {
            vtable_rva: Self::VTABLE_RVA,
            field_0x0000: Default::default(),
            field_0x0008: Default::default(),
            field_0x000a: Default::default(),
            field_0x0010: Default::default(),
            field_0x0018: Default::default(),
            field_0x0020: Default::default(),
            field_0x0040: Default::default(),
        }
    }

    /// Decompiled virtual method #0 @ RVA 0x00386a80 (18 instructions, 3 basic blocks)
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

    /// Decompiled virtual method #3 @ RVA 0x00392e20 (110 instructions, 38 basic blocks)
    pub fn method_03(&mut self) {
        // Writes to field at offset +0x0000
        // Writes to field at offset +0x0020
        // Writes to field at offset +0x0040
    }

}

impl CcClass for ccEventM133CastDelete {
    fn class_name(&self) -> &'static str { Self::CLASS_NAME }
    fn vtable_rva(&self) -> u32 { Self::VTABLE_RVA }
    fn virtual_method_count(&self) -> usize { Self::METHOD_COUNT }
    fn base_classes(&self) -> &'static [&'static str] { Self::BASE_CLASSES }
}

/// Decompiled CC2 class: `ccEventM133CastEntry` (VTable RVA: 0x008b5788, 4 virtual methods, 4 recovered fields)
/// Inherits: ccEventBase
#[derive(Debug, Clone, Default)]
pub struct ccEventM133CastEntry {
    pub vtable_rva: u32,
    pub field_0x0000: u64,
    pub field_0x0008: u16,
    pub field_0x000a: u8,
    pub field_0x0020: u32,
}

impl ccEventM133CastEntry {
    pub const CLASS_NAME: &'static str = "ccEventM133CastEntry";
    pub const VTABLE_RVA: u32 = 0x008b5788;
    pub const METHOD_COUNT: usize = 4;
    pub const BASE_CLASSES: &'static [&'static str] = &["ccEventBase"];
    pub const VIRTUAL_METHODS: &'static [u64] = &[0x0000000180386a40, 0x0000000180380cf0, 0x00000001800d3090, 0x0000000180392ad0];

    pub fn new() -> Self {
        Self {
            vtable_rva: Self::VTABLE_RVA,
            field_0x0000: Default::default(),
            field_0x0008: Default::default(),
            field_0x000a: Default::default(),
            field_0x0020: Default::default(),
        }
    }

    /// Decompiled virtual method #0 @ RVA 0x00386a40 (18 instructions, 3 basic blocks)
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

    /// Decompiled virtual method #3 @ RVA 0x00392ad0 (53 instructions, 13 basic blocks)
    pub fn method_03(&mut self) {
    }

}

impl CcClass for ccEventM133CastEntry {
    fn class_name(&self) -> &'static str { Self::CLASS_NAME }
    fn vtable_rva(&self) -> u32 { Self::VTABLE_RVA }
    fn virtual_method_count(&self) -> usize { Self::METHOD_COUNT }
    fn base_classes(&self) -> &'static [&'static str] { Self::BASE_CLASSES }
}

/// Decompiled CC2 class: `ccEventMAFlag` (VTable RVA: 0x008b5580, 4 virtual methods, 3 recovered fields)
/// Inherits: ccEventBase
#[derive(Debug, Clone, Default)]
pub struct ccEventMAFlag {
    pub vtable_rva: u32,
    pub field_0x0000: u64,
    pub field_0x0008: u16,
    pub field_0x000a: u8,
}

impl ccEventMAFlag {
    pub const CLASS_NAME: &'static str = "ccEventMAFlag";
    pub const VTABLE_RVA: u32 = 0x008b5580;
    pub const METHOD_COUNT: usize = 4;
    pub const BASE_CLASSES: &'static [&'static str] = &["ccEventBase"];
    pub const VIRTUAL_METHODS: &'static [u64] = &[0x0000000180386b80, 0x0000000180380cf0, 0x00000001800d3090, 0x00000001803933b0];

    pub fn new() -> Self {
        Self {
            vtable_rva: Self::VTABLE_RVA,
            field_0x0000: Default::default(),
            field_0x0008: Default::default(),
            field_0x000a: Default::default(),
        }
    }

    /// Decompiled virtual method #0 @ RVA 0x00386b80 (18 instructions, 3 basic blocks)
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

    /// Decompiled virtual method #3 @ RVA 0x003933b0 (66 instructions, 18 basic blocks)
    pub fn method_03(&mut self) {
    }

}

impl CcClass for ccEventMAFlag {
    fn class_name(&self) -> &'static str { Self::CLASS_NAME }
    fn vtable_rva(&self) -> u32 { Self::VTABLE_RVA }
    fn virtual_method_count(&self) -> usize { Self::METHOD_COUNT }
    fn base_classes(&self) -> &'static [&'static str] { Self::BASE_CLASSES }
}

/// Decompiled CC2 class: `ccEventMail` (VTable RVA: 0x008b6b18, 4 virtual methods, 3 recovered fields)
/// Inherits: ccEventBase
#[derive(Debug, Clone, Default)]
pub struct ccEventMail {
    pub vtable_rva: u32,
    pub field_0x0000: u64,
    pub field_0x0008: u16,
    pub field_0x000a: u8,
}

impl ccEventMail {
    pub const CLASS_NAME: &'static str = "ccEventMail";
    pub const VTABLE_RVA: u32 = 0x008b6b18;
    pub const METHOD_COUNT: usize = 4;
    pub const BASE_CLASSES: &'static [&'static str] = &["ccEventBase"];
    pub const VIRTUAL_METHODS: &'static [u64] = &[0x0000000180385a80, 0x0000000180380cf0, 0x00000001800d3090, 0x000000018038c550];

    pub fn new() -> Self {
        Self {
            vtable_rva: Self::VTABLE_RVA,
            field_0x0000: Default::default(),
            field_0x0008: Default::default(),
            field_0x000a: Default::default(),
        }
    }

    /// Decompiled virtual method #0 @ RVA 0x00385a80 (18 instructions, 3 basic blocks)
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

    /// Decompiled virtual method #3 @ RVA 0x0038c550 (12 instructions, 2 basic blocks)
    pub fn method_03(&mut self) {
    }

}

impl CcClass for ccEventMail {
    fn class_name(&self) -> &'static str { Self::CLASS_NAME }
    fn vtable_rva(&self) -> u32 { Self::VTABLE_RVA }
    fn virtual_method_count(&self) -> usize { Self::METHOD_COUNT }
    fn base_classes(&self) -> &'static [&'static str] { Self::BASE_CLASSES }
}

/// Decompiled CC2 class: `ccEventManipObj` (VTable RVA: 0x008b5468, 4 virtual methods, 3 recovered fields)
/// Inherits: ccEventBase
#[derive(Debug, Clone, Default)]
pub struct ccEventManipObj {
    pub vtable_rva: u32,
    pub field_0x0000: u64,
    pub field_0x0008: u16,
    pub field_0x000a: u8,
}

impl ccEventManipObj {
    pub const CLASS_NAME: &'static str = "ccEventManipObj";
    pub const VTABLE_RVA: u32 = 0x008b5468;
    pub const METHOD_COUNT: usize = 4;
    pub const BASE_CLASSES: &'static [&'static str] = &["ccEventBase"];
    pub const VIRTUAL_METHODS: &'static [u64] = &[0x0000000180386d00, 0x0000000180380cf0, 0x00000001800d3090, 0x0000000180394200];

    pub fn new() -> Self {
        Self {
            vtable_rva: Self::VTABLE_RVA,
            field_0x0000: Default::default(),
            field_0x0008: Default::default(),
            field_0x000a: Default::default(),
        }
    }

    /// Decompiled virtual method #0 @ RVA 0x00386d00 (18 instructions, 3 basic blocks)
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

    /// Decompiled virtual method #3 @ RVA 0x00394200 (35 instructions, 12 basic blocks)
    pub fn method_03(&mut self) {
    }

}

impl CcClass for ccEventManipObj {
    fn class_name(&self) -> &'static str { Self::CLASS_NAME }
    fn vtable_rva(&self) -> u32 { Self::VTABLE_RVA }
    fn virtual_method_count(&self) -> usize { Self::METHOD_COUNT }
    fn base_classes(&self) -> &'static [&'static str] { Self::BASE_CLASSES }
}

/// Decompiled CC2 class: `ccEventMap` (VTable RVA: 0x008b6b68, 4 virtual methods, 3 recovered fields)
/// Inherits: ccEventBase
#[derive(Debug, Clone, Default)]
pub struct ccEventMap {
    pub vtable_rva: u32,
    pub field_0x0000: u64,
    pub field_0x0008: u16,
    pub field_0x000a: u8,
}

impl ccEventMap {
    pub const CLASS_NAME: &'static str = "ccEventMap";
    pub const VTABLE_RVA: u32 = 0x008b6b68;
    pub const METHOD_COUNT: usize = 4;
    pub const BASE_CLASSES: &'static [&'static str] = &["ccEventBase"];
    pub const VIRTUAL_METHODS: &'static [u64] = &[0x0000000180385840, 0x0000000180380cf0, 0x00000001800d3090, 0x000000018038a480];

    pub fn new() -> Self {
        Self {
            vtable_rva: Self::VTABLE_RVA,
            field_0x0000: Default::default(),
            field_0x0008: Default::default(),
            field_0x000a: Default::default(),
        }
    }

    /// Decompiled virtual method #0 @ RVA 0x00385840 (18 instructions, 3 basic blocks)
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

    /// Decompiled virtual method #3 @ RVA 0x0038a480 (37 instructions, 10 basic blocks)
    pub fn method_03(&mut self) {
    }

}

impl CcClass for ccEventMap {
    fn class_name(&self) -> &'static str { Self::CLASS_NAME }
    fn vtable_rva(&self) -> u32 { Self::VTABLE_RVA }
    fn virtual_method_count(&self) -> usize { Self::METHOD_COUNT }
    fn base_classes(&self) -> &'static [&'static str] { Self::BASE_CLASSES }
}

/// Decompiled CC2 class: `ccEventMapnavi` (VTable RVA: 0x008b6610, 4 virtual methods, 3 recovered fields)
/// Inherits: ccEventBase
#[derive(Debug, Clone, Default)]
pub struct ccEventMapnavi {
    pub vtable_rva: u32,
    pub field_0x0000: u64,
    pub field_0x0008: u16,
    pub field_0x000a: u8,
}

impl ccEventMapnavi {
    pub const CLASS_NAME: &'static str = "ccEventMapnavi";
    pub const VTABLE_RVA: u32 = 0x008b6610;
    pub const METHOD_COUNT: usize = 4;
    pub const BASE_CLASSES: &'static [&'static str] = &["ccEventBase"];
    pub const VIRTUAL_METHODS: &'static [u64] = &[0x00000001803865b0, 0x0000000180380cf0, 0x00000001800d3090, 0x0000000180390ab0];

    pub fn new() -> Self {
        Self {
            vtable_rva: Self::VTABLE_RVA,
            field_0x0000: Default::default(),
            field_0x0008: Default::default(),
            field_0x000a: Default::default(),
        }
    }

    /// Decompiled virtual method #0 @ RVA 0x003865b0 (18 instructions, 3 basic blocks)
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

    /// Decompiled virtual method #3 @ RVA 0x00390ab0 (12 instructions, 2 basic blocks)
    pub fn method_03(&mut self) {
    }

}

impl CcClass for ccEventMapnavi {
    fn class_name(&self) -> &'static str { Self::CLASS_NAME }
    fn vtable_rva(&self) -> u32 { Self::VTABLE_RVA }
    fn virtual_method_count(&self) -> usize { Self::METHOD_COUNT }
    fn base_classes(&self) -> &'static [&'static str] { Self::BASE_CLASSES }
}

/// Decompiled CC2 class: `ccEventMemAdrs` (VTable RVA: 0x008b6a78, 4 virtual methods, 3 recovered fields)
/// Inherits: ccEventBase
#[derive(Debug, Clone, Default)]
pub struct ccEventMemAdrs {
    pub vtable_rva: u32,
    pub field_0x0000: u64,
    pub field_0x0008: u16,
    pub field_0x000a: u8,
}

impl ccEventMemAdrs {
    pub const CLASS_NAME: &'static str = "ccEventMemAdrs";
    pub const VTABLE_RVA: u32 = 0x008b6a78;
    pub const METHOD_COUNT: usize = 4;
    pub const BASE_CLASSES: &'static [&'static str] = &["ccEventBase"];
    pub const VIRTUAL_METHODS: &'static [u64] = &[0x0000000180385a40, 0x0000000180380cf0, 0x00000001800d3090, 0x000000018038c240];

    pub fn new() -> Self {
        Self {
            vtable_rva: Self::VTABLE_RVA,
            field_0x0000: Default::default(),
            field_0x0008: Default::default(),
            field_0x000a: Default::default(),
        }
    }

    /// Decompiled virtual method #0 @ RVA 0x00385a40 (18 instructions, 3 basic blocks)
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

    /// Decompiled virtual method #3 @ RVA 0x0038c240 (12 instructions, 2 basic blocks)
    pub fn method_03(&mut self) {
    }

}

impl CcClass for ccEventMemAdrs {
    fn class_name(&self) -> &'static str { Self::CLASS_NAME }
    fn vtable_rva(&self) -> u32 { Self::VTABLE_RVA }
    fn virtual_method_count(&self) -> usize { Self::METHOD_COUNT }
    fn base_classes(&self) -> &'static [&'static str] { Self::BASE_CLASSES }
}

/// Decompiled CC2 class: `ccEventMemory` (VTable RVA: 0x008b68c0, 4 virtual methods, 5 recovered fields)
/// Inherits: ccEventBase
#[derive(Debug, Clone, Default)]
pub struct ccEventMemory {
    pub vtable_rva: u32,
    pub field_0x0000: u64,
    pub field_0x0008: u16,
    pub field_0x000a: u8,
    pub field_0x002c: u8,
    pub field_0x0c48: u64,
}

impl ccEventMemory {
    pub const CLASS_NAME: &'static str = "ccEventMemory";
    pub const VTABLE_RVA: u32 = 0x008b68c0;
    pub const METHOD_COUNT: usize = 4;
    pub const BASE_CLASSES: &'static [&'static str] = &["ccEventBase"];
    pub const VIRTUAL_METHODS: &'static [u64] = &[0x0000000180385db0, 0x0000000180380cf0, 0x00000001800d3090, 0x000000018038d8f0];

    pub fn new() -> Self {
        Self {
            vtable_rva: Self::VTABLE_RVA,
            field_0x0000: Default::default(),
            field_0x0008: Default::default(),
            field_0x000a: Default::default(),
            field_0x002c: Default::default(),
            field_0x0c48: Default::default(),
        }
    }

    /// Decompiled virtual method #0 @ RVA 0x00385db0 (18 instructions, 3 basic blocks)
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

    /// Decompiled virtual method #3 @ RVA 0x0038d8f0 (109 instructions, 31 basic blocks)
    pub fn method_03(&mut self) {
        // Writes to field at offset +0x002c
    }

}

impl CcClass for ccEventMemory {
    fn class_name(&self) -> &'static str { Self::CLASS_NAME }
    fn vtable_rva(&self) -> u32 { Self::VTABLE_RVA }
    fn virtual_method_count(&self) -> usize { Self::METHOD_COUNT }
    fn base_classes(&self) -> &'static [&'static str] { Self::BASE_CLASSES }
}

/// Decompiled CC2 class: `ccEventMenu` (VTable RVA: 0x008b6c30, 4 virtual methods, 3 recovered fields)
/// Inherits: ccEventBase
#[derive(Debug, Clone, Default)]
pub struct ccEventMenu {
    pub vtable_rva: u32,
    pub field_0x0000: u64,
    pub field_0x0008: u16,
    pub field_0x000a: u8,
}

impl ccEventMenu {
    pub const CLASS_NAME: &'static str = "ccEventMenu";
    pub const VTABLE_RVA: u32 = 0x008b6c30;
    pub const METHOD_COUNT: usize = 4;
    pub const BASE_CLASSES: &'static [&'static str] = &["ccEventBase"];
    pub const VIRTUAL_METHODS: &'static [u64] = &[0x00000001803858c0, 0x0000000180380cf0, 0x00000001800d3090, 0x000000018038b440];

    pub fn new() -> Self {
        Self {
            vtable_rva: Self::VTABLE_RVA,
            field_0x0000: Default::default(),
            field_0x0008: Default::default(),
            field_0x000a: Default::default(),
        }
    }

    /// Decompiled virtual method #0 @ RVA 0x003858c0 (18 instructions, 3 basic blocks)
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

    /// Decompiled virtual method #3 @ RVA 0x0038b440 (43 instructions, 13 basic blocks)
    pub fn method_03(&mut self) {
    }

}

impl CcClass for ccEventMenu {
    fn class_name(&self) -> &'static str { Self::CLASS_NAME }
    fn vtable_rva(&self) -> u32 { Self::VTABLE_RVA }
    fn virtual_method_count(&self) -> usize { Self::METHOD_COUNT }
    fn base_classes(&self) -> &'static [&'static str] { Self::BASE_CLASSES }
}

/// Decompiled CC2 class: `ccEventMenuseal` (VTable RVA: 0x008b6638, 4 virtual methods, 3 recovered fields)
/// Inherits: ccEventBase
#[derive(Debug, Clone, Default)]
pub struct ccEventMenuseal {
    pub vtable_rva: u32,
    pub field_0x0000: u64,
    pub field_0x0008: u16,
    pub field_0x000a: u8,
}

impl ccEventMenuseal {
    pub const CLASS_NAME: &'static str = "ccEventMenuseal";
    pub const VTABLE_RVA: u32 = 0x008b6638;
    pub const METHOD_COUNT: usize = 4;
    pub const BASE_CLASSES: &'static [&'static str] = &["ccEventBase"];
    pub const VIRTUAL_METHODS: &'static [u64] = &[0x0000000180386670, 0x0000000180380cf0, 0x00000001800d3090, 0x0000000180391190];

    pub fn new() -> Self {
        Self {
            vtable_rva: Self::VTABLE_RVA,
            field_0x0000: Default::default(),
            field_0x0008: Default::default(),
            field_0x000a: Default::default(),
        }
    }

    /// Decompiled virtual method #0 @ RVA 0x00386670 (18 instructions, 3 basic blocks)
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

    /// Decompiled virtual method #3 @ RVA 0x00391190 (35 instructions, 11 basic blocks)
    pub fn method_03(&mut self) {
    }

}

impl CcClass for ccEventMenuseal {
    fn class_name(&self) -> &'static str { Self::CLASS_NAME }
    fn vtable_rva(&self) -> u32 { Self::VTABLE_RVA }
    fn virtual_method_count(&self) -> usize { Self::METHOD_COUNT }
    fn base_classes(&self) -> &'static [&'static str] { Self::BASE_CLASSES }
}

/// Decompiled CC2 class: `ccEventMissionChk` (VTable RVA: 0x008b69b0, 4 virtual methods, 3 recovered fields)
/// Inherits: ccEventBase
#[derive(Debug, Clone, Default)]
pub struct ccEventMissionChk {
    pub vtable_rva: u32,
    pub field_0x0000: u64,
    pub field_0x0008: u16,
    pub field_0x000a: u8,
}

impl ccEventMissionChk {
    pub const CLASS_NAME: &'static str = "ccEventMissionChk";
    pub const VTABLE_RVA: u32 = 0x008b69b0;
    pub const METHOD_COUNT: usize = 4;
    pub const BASE_CLASSES: &'static [&'static str] = &["ccEventBase"];
    pub const VIRTUAL_METHODS: &'static [u64] = &[0x0000000180385cb0, 0x0000000180380cf0, 0x00000001800d3090, 0x000000018038d250];

    pub fn new() -> Self {
        Self {
            vtable_rva: Self::VTABLE_RVA,
            field_0x0000: Default::default(),
            field_0x0008: Default::default(),
            field_0x000a: Default::default(),
        }
    }

    /// Decompiled virtual method #0 @ RVA 0x00385cb0 (18 instructions, 3 basic blocks)
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

    /// Decompiled virtual method #3 @ RVA 0x0038d250 (67 instructions, 21 basic blocks)
    pub fn method_03(&mut self) {
    }

}

impl CcClass for ccEventMissionChk {
    fn class_name(&self) -> &'static str { Self::CLASS_NAME }
    fn vtable_rva(&self) -> u32 { Self::VTABLE_RVA }
    fn virtual_method_count(&self) -> usize { Self::METHOD_COUNT }
    fn base_classes(&self) -> &'static [&'static str] { Self::BASE_CLASSES }
}

/// Decompiled CC2 class: `ccEventMoney` (VTable RVA: 0x008b6aa0, 4 virtual methods, 3 recovered fields)
/// Inherits: ccEventBase
#[derive(Debug, Clone, Default)]
pub struct ccEventMoney {
    pub vtable_rva: u32,
    pub field_0x0000: u64,
    pub field_0x0008: u16,
    pub field_0x000a: u8,
}

impl ccEventMoney {
    pub const CLASS_NAME: &'static str = "ccEventMoney";
    pub const VTABLE_RVA: u32 = 0x008b6aa0;
    pub const METHOD_COUNT: usize = 4;
    pub const BASE_CLASSES: &'static [&'static str] = &["ccEventBase"];
    pub const VIRTUAL_METHODS: &'static [u64] = &[0x00000001803859c0, 0x0000000180380cf0, 0x00000001800d3090, 0x000000018038bfe0];

    pub fn new() -> Self {
        Self {
            vtable_rva: Self::VTABLE_RVA,
            field_0x0000: Default::default(),
            field_0x0008: Default::default(),
            field_0x000a: Default::default(),
        }
    }

    /// Decompiled virtual method #0 @ RVA 0x003859c0 (18 instructions, 3 basic blocks)
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

    /// Decompiled virtual method #3 @ RVA 0x0038bfe0 (78 instructions, 18 basic blocks)
    pub fn method_03(&mut self) {
    }

}

impl CcClass for ccEventMoney {
    fn class_name(&self) -> &'static str { Self::CLASS_NAME }
    fn vtable_rva(&self) -> u32 { Self::VTABLE_RVA }
    fn virtual_method_count(&self) -> usize { Self::METHOD_COUNT }
    fn base_classes(&self) -> &'static [&'static str] { Self::BASE_CLASSES }
}

/// Decompiled CC2 class: `ccEventMpeg` (VTable RVA: 0x008b6a28, 4 virtual methods, 6 recovered fields)
/// Inherits: ccEventBase
#[derive(Debug, Clone, Default)]
pub struct ccEventMpeg {
    pub vtable_rva: u32,
    pub field_0x0000: u64,
    pub field_0x0004: u32,
    pub field_0x0008: u16,
    pub field_0x000a: u8,
    pub field_0x0c30: u64,
    pub field_0x0c48: u64,
}

impl ccEventMpeg {
    pub const CLASS_NAME: &'static str = "ccEventMpeg";
    pub const VTABLE_RVA: u32 = 0x008b6a28;
    pub const METHOD_COUNT: usize = 4;
    pub const BASE_CLASSES: &'static [&'static str] = &["ccEventBase"];
    pub const VIRTUAL_METHODS: &'static [u64] = &[0x0000000180385c70, 0x0000000180380cf0, 0x00000001800d3090, 0x000000018038ce80];

    pub fn new() -> Self {
        Self {
            vtable_rva: Self::VTABLE_RVA,
            field_0x0000: Default::default(),
            field_0x0004: Default::default(),
            field_0x0008: Default::default(),
            field_0x000a: Default::default(),
            field_0x0c30: Default::default(),
            field_0x0c48: Default::default(),
        }
    }

    /// Decompiled virtual method #0 @ RVA 0x00385c70 (18 instructions, 3 basic blocks)
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

    /// Decompiled virtual method #3 @ RVA 0x0038ce80 (122 instructions, 35 basic blocks)
    pub fn method_03(&mut self) {
        // Writes to field at offset +0x0c30
    }

}

impl CcClass for ccEventMpeg {
    fn class_name(&self) -> &'static str { Self::CLASS_NAME }
    fn vtable_rva(&self) -> u32 { Self::VTABLE_RVA }
    fn virtual_method_count(&self) -> usize { Self::METHOD_COUNT }
    fn base_classes(&self) -> &'static [&'static str] { Self::BASE_CLASSES }
}

/// Decompiled CC2 class: `ccEventMsg` (VTable RVA: 0x008b6de8, 4 virtual methods, 3 recovered fields)
/// Inherits: ccEventBase
#[derive(Debug, Clone, Default)]
pub struct ccEventMsg {
    pub vtable_rva: u32,
    pub field_0x0000: u64,
    pub field_0x0008: u16,
    pub field_0x000a: u8,
}

impl ccEventMsg {
    pub const CLASS_NAME: &'static str = "ccEventMsg";
    pub const VTABLE_RVA: u32 = 0x008b6de8;
    pub const METHOD_COUNT: usize = 4;
    pub const BASE_CLASSES: &'static [&'static str] = &["ccEventBase"];
    pub const VIRTUAL_METHODS: &'static [u64] = &[0x00000001803855c0, 0x0000000180380cf0, 0x00000001800d3090, 0x0000000180388660];

    pub fn new() -> Self {
        Self {
            vtable_rva: Self::VTABLE_RVA,
            field_0x0000: Default::default(),
            field_0x0008: Default::default(),
            field_0x000a: Default::default(),
        }
    }

    /// Decompiled virtual method #0 @ RVA 0x003855c0 (18 instructions, 3 basic blocks)
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

    /// Decompiled virtual method #3 @ RVA 0x00388660 (12 instructions, 2 basic blocks)
    pub fn method_03(&mut self) {
    }

}

impl CcClass for ccEventMsg {
    fn class_name(&self) -> &'static str { Self::CLASS_NAME }
    fn vtable_rva(&self) -> u32 { Self::VTABLE_RVA }
    fn virtual_method_count(&self) -> usize { Self::METHOD_COUNT }
    fn base_classes(&self) -> &'static [&'static str] { Self::BASE_CLASSES }
}

/// Decompiled CC2 class: `ccEventMsgFile` (VTable RVA: 0x008b6cd0, 4 virtual methods, 3 recovered fields)
/// Inherits: ccEventBase
#[derive(Debug, Clone, Default)]
pub struct ccEventMsgFile {
    pub vtable_rva: u32,
    pub field_0x0000: u64,
    pub field_0x0008: u16,
    pub field_0x000a: u8,
}

impl ccEventMsgFile {
    pub const CLASS_NAME: &'static str = "ccEventMsgFile";
    pub const VTABLE_RVA: u32 = 0x008b6cd0;
    pub const METHOD_COUNT: usize = 4;
    pub const BASE_CLASSES: &'static [&'static str] = &["ccEventBase"];
    pub const VIRTUAL_METHODS: &'static [u64] = &[0x0000000180385640, 0x0000000180380cf0, 0x00000001800d3090, 0x0000000180388db0];

    pub fn new() -> Self {
        Self {
            vtable_rva: Self::VTABLE_RVA,
            field_0x0000: Default::default(),
            field_0x0008: Default::default(),
            field_0x000a: Default::default(),
        }
    }

    /// Decompiled virtual method #0 @ RVA 0x00385640 (18 instructions, 3 basic blocks)
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

    /// Decompiled virtual method #3 @ RVA 0x00388db0 (12 instructions, 2 basic blocks)
    pub fn method_03(&mut self) {
    }

}

impl CcClass for ccEventMsgFile {
    fn class_name(&self) -> &'static str { Self::CLASS_NAME }
    fn vtable_rva(&self) -> u32 { Self::VTABLE_RVA }
    fn virtual_method_count(&self) -> usize { Self::METHOD_COUNT }
    fn base_classes(&self) -> &'static [&'static str] { Self::BASE_CLASSES }
}

/// Decompiled CC2 class: `ccEventNavimemo` (VTable RVA: 0x008b6898, 4 virtual methods, 3 recovered fields)
/// Inherits: ccEventBase
#[derive(Debug, Clone, Default)]
pub struct ccEventNavimemo {
    pub vtable_rva: u32,
    pub field_0x0000: u64,
    pub field_0x0008: u16,
    pub field_0x000a: u8,
}

impl ccEventNavimemo {
    pub const CLASS_NAME: &'static str = "ccEventNavimemo";
    pub const VTABLE_RVA: u32 = 0x008b6898;
    pub const METHOD_COUNT: usize = 4;
    pub const BASE_CLASSES: &'static [&'static str] = &["ccEventBase"];
    pub const VIRTUAL_METHODS: &'static [u64] = &[0x0000000180386100, 0x0000000180380cf0, 0x00000001800d3090, 0x000000018038eef0];

    pub fn new() -> Self {
        Self {
            vtable_rva: Self::VTABLE_RVA,
            field_0x0000: Default::default(),
            field_0x0008: Default::default(),
            field_0x000a: Default::default(),
        }
    }

    /// Decompiled virtual method #0 @ RVA 0x00386100 (18 instructions, 3 basic blocks)
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

    /// Decompiled virtual method #3 @ RVA 0x0038eef0 (30 instructions, 8 basic blocks)
    pub fn method_03(&mut self) {
    }

}

impl CcClass for ccEventNavimemo {
    fn class_name(&self) -> &'static str { Self::CLASS_NAME }
    fn vtable_rva(&self) -> u32 { Self::VTABLE_RVA }
    fn virtual_method_count(&self) -> usize { Self::METHOD_COUNT }
    fn base_classes(&self) -> &'static [&'static str] { Self::BASE_CLASSES }
}

/// Decompiled CC2 class: `ccEventNoize` (VTable RVA: 0x008b66e8, 4 virtual methods, 4 recovered fields)
/// Inherits: ccEventBase, ccPDEffectNoize, ccPDEffectBase
#[derive(Debug, Clone, Default)]
pub struct ccEventNoize {
    pub vtable_rva: u32,
    pub field_0x0008: u16,
    pub field_0x000a: u8,
    pub field_0x0020: u64,
    pub field_0x0b88: u32,
}

impl ccEventNoize {
    pub const CLASS_NAME: &'static str = "ccEventNoize";
    pub const VTABLE_RVA: u32 = 0x008b66e8;
    pub const METHOD_COUNT: usize = 4;
    pub const BASE_CLASSES: &'static [&'static str] = &["ccEventBase", "ccPDEffectNoize", "ccPDEffectBase"];
    pub const VIRTUAL_METHODS: &'static [u64] = &[0x0000000180386410, 0x0000000180380cf0, 0x000000018038fd90, 0x000000018038fc00];

    pub fn new() -> Self {
        Self {
            vtable_rva: Self::VTABLE_RVA,
            field_0x0008: Default::default(),
            field_0x000a: Default::default(),
            field_0x0020: Default::default(),
            field_0x0b88: Default::default(),
        }
    }

    /// Decompiled virtual method #0 @ RVA 0x00386410 (16 instructions, 3 basic blocks)
    pub fn method_00(&mut self) {
    }

    /// Decompiled virtual method #1 @ RVA 0x00380cf0 (13 instructions, 2 basic blocks)
    pub fn method_01(&mut self) {
        // Writes to field at offset +0x0008
        // Writes to field at offset +0x000a
    }

    /// Decompiled virtual method #2 @ RVA 0x0038fd90 (19 instructions, 3 basic blocks)
    pub fn method_02(&mut self) {
        // Writes to field at offset +0x0b88
    }

    /// Decompiled virtual method #3 @ RVA 0x0038fc00 (95 instructions, 20 basic blocks)
    pub fn method_03(&mut self) {
    }

}

impl CcClass for ccEventNoize {
    fn class_name(&self) -> &'static str { Self::CLASS_NAME }
    fn vtable_rva(&self) -> u32 { Self::VTABLE_RVA }
    fn virtual_method_count(&self) -> usize { Self::METHOD_COUNT }
    fn base_classes(&self) -> &'static [&'static str] { Self::BASE_CLASSES }
}

/// Decompiled CC2 class: `ccEventOnOffEnableFlag` (VTable RVA: 0x008b53f0, 4 virtual methods, 3 recovered fields)
/// Inherits: ccEventBase
#[derive(Debug, Clone, Default)]
pub struct ccEventOnOffEnableFlag {
    pub vtable_rva: u32,
    pub field_0x0000: u64,
    pub field_0x0008: u16,
    pub field_0x000a: u8,
}

impl ccEventOnOffEnableFlag {
    pub const CLASS_NAME: &'static str = "ccEventOnOffEnableFlag";
    pub const VTABLE_RVA: u32 = 0x008b53f0;
    pub const METHOD_COUNT: usize = 4;
    pub const BASE_CLASSES: &'static [&'static str] = &["ccEventBase"];
    pub const VIRTUAL_METHODS: &'static [u64] = &[0x0000000180386f80, 0x0000000180380cf0, 0x00000001800d3090, 0x0000000180394930];

    pub fn new() -> Self {
        Self {
            vtable_rva: Self::VTABLE_RVA,
            field_0x0000: Default::default(),
            field_0x0008: Default::default(),
            field_0x000a: Default::default(),
        }
    }

    /// Decompiled virtual method #0 @ RVA 0x00386f80 (18 instructions, 3 basic blocks)
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

    /// Decompiled virtual method #3 @ RVA 0x00394930 (40 instructions, 11 basic blocks)
    pub fn method_03(&mut self) {
    }

}

impl CcClass for ccEventOnOffEnableFlag {
    fn class_name(&self) -> &'static str { Self::CLASS_NAME }
    fn vtable_rva(&self) -> u32 { Self::VTABLE_RVA }
    fn virtual_method_count(&self) -> usize { Self::METHOD_COUNT }
    fn base_classes(&self) -> &'static [&'static str] { Self::BASE_CLASSES }
}

/// Decompiled CC2 class: `ccEventOpenL3` (VTable RVA: 0x008b6660, 4 virtual methods, 6 recovered fields)
/// Inherits: ccEventBase
#[derive(Debug, Clone, Default)]
pub struct ccEventOpenL3 {
    pub vtable_rva: u32,
    pub field_0x0000: u64,
    pub field_0x0008: u16,
    pub field_0x000a: u8,
    pub field_0x0010: u64,
    pub field_0x0018: u64,
    pub field_0x0040: u32,
}

impl ccEventOpenL3 {
    pub const CLASS_NAME: &'static str = "ccEventOpenL3";
    pub const VTABLE_RVA: u32 = 0x008b6660;
    pub const METHOD_COUNT: usize = 4;
    pub const BASE_CLASSES: &'static [&'static str] = &["ccEventBase"];
    pub const VIRTUAL_METHODS: &'static [u64] = &[0x00000001803866b0, 0x0000000180380cf0, 0x00000001800d3090, 0x00000001803912f0];

    pub fn new() -> Self {
        Self {
            vtable_rva: Self::VTABLE_RVA,
            field_0x0000: Default::default(),
            field_0x0008: Default::default(),
            field_0x000a: Default::default(),
            field_0x0010: Default::default(),
            field_0x0018: Default::default(),
            field_0x0040: Default::default(),
        }
    }

    /// Decompiled virtual method #0 @ RVA 0x003866b0 (18 instructions, 3 basic blocks)
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

    /// Decompiled virtual method #3 @ RVA 0x003912f0 (62 instructions, 22 basic blocks)
    pub fn method_03(&mut self) {
        // Writes to field at offset +0x0040
    }

}

impl CcClass for ccEventOpenL3 {
    fn class_name(&self) -> &'static str { Self::CLASS_NAME }
    fn vtable_rva(&self) -> u32 { Self::VTABLE_RVA }
    fn virtual_method_count(&self) -> usize { Self::METHOD_COUNT }
    fn base_classes(&self) -> &'static [&'static str] { Self::BASE_CLASSES }
}

/// Decompiled CC2 class: `ccEventPadVibration` (VTable RVA: 0x008b5328, 4 virtual methods, 3 recovered fields)
/// Inherits: ccEventBase
#[derive(Debug, Clone, Default)]
pub struct ccEventPadVibration {
    pub vtable_rva: u32,
    pub field_0x0000: u64,
    pub field_0x0008: u16,
    pub field_0x000a: u8,
}

impl ccEventPadVibration {
    pub const CLASS_NAME: &'static str = "ccEventPadVibration";
    pub const VTABLE_RVA: u32 = 0x008b5328;
    pub const METHOD_COUNT: usize = 4;
    pub const BASE_CLASSES: &'static [&'static str] = &["ccEventBase"];
    pub const VIRTUAL_METHODS: &'static [u64] = &[0x00000001803870c0, 0x0000000180380cf0, 0x00000001800d3090, 0x0000000180394e30];

    pub fn new() -> Self {
        Self {
            vtable_rva: Self::VTABLE_RVA,
            field_0x0000: Default::default(),
            field_0x0008: Default::default(),
            field_0x000a: Default::default(),
        }
    }

    /// Decompiled virtual method #0 @ RVA 0x003870c0 (18 instructions, 3 basic blocks)
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

    /// Decompiled virtual method #3 @ RVA 0x00394e30 (21 instructions, 6 basic blocks)
    pub fn method_03(&mut self) {
    }

}

impl CcClass for ccEventPadVibration {
    fn class_name(&self) -> &'static str { Self::CLASS_NAME }
    fn vtable_rva(&self) -> u32 { Self::VTABLE_RVA }
    fn virtual_method_count(&self) -> usize { Self::METHOD_COUNT }
    fn base_classes(&self) -> &'static [&'static str] { Self::BASE_CLASSES }
}

/// Decompiled CC2 class: `ccEventParty` (VTable RVA: 0x008b6af0, 4 virtual methods, 3 recovered fields)
/// Inherits: ccEventBase
#[derive(Debug, Clone, Default)]
pub struct ccEventParty {
    pub vtable_rva: u32,
    pub field_0x0000: u64,
    pub field_0x0008: u16,
    pub field_0x000a: u8,
}

impl ccEventParty {
    pub const CLASS_NAME: &'static str = "ccEventParty";
    pub const VTABLE_RVA: u32 = 0x008b6af0;
    pub const METHOD_COUNT: usize = 4;
    pub const BASE_CLASSES: &'static [&'static str] = &["ccEventBase"];
    pub const VIRTUAL_METHODS: &'static [u64] = &[0x0000000180385b40, 0x0000000180380cf0, 0x00000001800d3090, 0x000000018038c8a0];

    pub fn new() -> Self {
        Self {
            vtable_rva: Self::VTABLE_RVA,
            field_0x0000: Default::default(),
            field_0x0008: Default::default(),
            field_0x000a: Default::default(),
        }
    }

    /// Decompiled virtual method #0 @ RVA 0x00385b40 (18 instructions, 3 basic blocks)
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

    /// Decompiled virtual method #3 @ RVA 0x0038c8a0 (12 instructions, 2 basic blocks)
    pub fn method_03(&mut self) {
    }

}

impl CcClass for ccEventParty {
    fn class_name(&self) -> &'static str { Self::CLASS_NAME }
    fn vtable_rva(&self) -> u32 { Self::VTABLE_RVA }
    fn virtual_method_count(&self) -> usize { Self::METHOD_COUNT }
    fn base_classes(&self) -> &'static [&'static str] { Self::BASE_CLASSES }
}

/// Decompiled CC2 class: `ccEventPause` (VTable RVA: 0x008b6e38, 4 virtual methods, 3 recovered fields)
/// Inherits: ccEventBase
#[derive(Debug, Clone, Default)]
pub struct ccEventPause {
    pub vtable_rva: u32,
    pub field_0x0000: u64,
    pub field_0x0008: u16,
    pub field_0x000a: u8,
}

impl ccEventPause {
    pub const CLASS_NAME: &'static str = "ccEventPause";
    pub const VTABLE_RVA: u32 = 0x008b6e38;
    pub const METHOD_COUNT: usize = 4;
    pub const BASE_CLASSES: &'static [&'static str] = &["ccEventBase"];
    pub const VIRTUAL_METHODS: &'static [u64] = &[0x0000000180385540, 0x0000000180380cf0, 0x00000001800d3090, 0x0000000180388110];

    pub fn new() -> Self {
        Self {
            vtable_rva: Self::VTABLE_RVA,
            field_0x0000: Default::default(),
            field_0x0008: Default::default(),
            field_0x000a: Default::default(),
        }
    }

    /// Decompiled virtual method #0 @ RVA 0x00385540 (18 instructions, 3 basic blocks)
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

    /// Decompiled virtual method #3 @ RVA 0x00388110 (57 instructions, 16 basic blocks)
    pub fn method_03(&mut self) {
    }

}

impl CcClass for ccEventPause {
    fn class_name(&self) -> &'static str { Self::CLASS_NAME }
    fn vtable_rva(&self) -> u32 { Self::VTABLE_RVA }
    fn virtual_method_count(&self) -> usize { Self::METHOD_COUNT }
    fn base_classes(&self) -> &'static [&'static str] { Self::BASE_CLASSES }
}

/// Decompiled CC2 class: `ccEventPauseObj` (VTable RVA: 0x008b54b8, 4 virtual methods, 3 recovered fields)
/// Inherits: ccEventBase
#[derive(Debug, Clone, Default)]
pub struct ccEventPauseObj {
    pub vtable_rva: u32,
    pub field_0x0000: u64,
    pub field_0x0008: u16,
    pub field_0x000a: u8,
}

impl ccEventPauseObj {
    pub const CLASS_NAME: &'static str = "ccEventPauseObj";
    pub const VTABLE_RVA: u32 = 0x008b54b8;
    pub const METHOD_COUNT: usize = 4;
    pub const BASE_CLASSES: &'static [&'static str] = &["ccEventBase"];
    pub const VIRTUAL_METHODS: &'static [u64] = &[0x0000000180386cc0, 0x0000000180380cf0, 0x00000001800d3090, 0x00000001803941a0];

    pub fn new() -> Self {
        Self {
            vtable_rva: Self::VTABLE_RVA,
            field_0x0000: Default::default(),
            field_0x0008: Default::default(),
            field_0x000a: Default::default(),
        }
    }

    /// Decompiled virtual method #0 @ RVA 0x00386cc0 (18 instructions, 3 basic blocks)
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

    /// Decompiled virtual method #3 @ RVA 0x003941a0 (29 instructions, 8 basic blocks)
    pub fn method_03(&mut self) {
    }

}

impl CcClass for ccEventPauseObj {
    fn class_name(&self) -> &'static str { Self::CLASS_NAME }
    fn vtable_rva(&self) -> u32 { Self::VTABLE_RVA }
    fn virtual_method_count(&self) -> usize { Self::METHOD_COUNT }
    fn base_classes(&self) -> &'static [&'static str] { Self::BASE_CLASSES }
}

/// Decompiled CC2 class: `ccEventPlayer` (VTable RVA: 0x008b6c80, 4 virtual methods, 3 recovered fields)
/// Inherits: ccEventBase
#[derive(Debug, Clone, Default)]
pub struct ccEventPlayer {
    pub vtable_rva: u32,
    pub field_0x0000: u64,
    pub field_0x0008: u16,
    pub field_0x000a: u8,
}

impl ccEventPlayer {
    pub const CLASS_NAME: &'static str = "ccEventPlayer";
    pub const VTABLE_RVA: u32 = 0x008b6c80;
    pub const METHOD_COUNT: usize = 4;
    pub const BASE_CLASSES: &'static [&'static str] = &["ccEventBase"];
    pub const VIRTUAL_METHODS: &'static [u64] = &[0x00000001803856c0, 0x0000000180380cf0, 0x00000001800d3090, 0x0000000180388fe0];

    pub fn new() -> Self {
        Self {
            vtable_rva: Self::VTABLE_RVA,
            field_0x0000: Default::default(),
            field_0x0008: Default::default(),
            field_0x000a: Default::default(),
        }
    }

    /// Decompiled virtual method #0 @ RVA 0x003856c0 (18 instructions, 3 basic blocks)
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

    /// Decompiled virtual method #3 @ RVA 0x00388fe0 (12 instructions, 2 basic blocks)
    pub fn method_03(&mut self) {
    }

}

impl CcClass for ccEventPlayer {
    fn class_name(&self) -> &'static str { Self::CLASS_NAME }
    fn vtable_rva(&self) -> u32 { Self::VTABLE_RVA }
    fn virtual_method_count(&self) -> usize { Self::METHOD_COUNT }
    fn base_classes(&self) -> &'static [&'static str] { Self::BASE_CLASSES }
}

/// Decompiled CC2 class: `ccEventProgress` (VTable RVA: 0x008b67a8, 4 virtual methods, 3 recovered fields)
/// Inherits: ccEventBase
#[derive(Debug, Clone, Default)]
pub struct ccEventProgress {
    pub vtable_rva: u32,
    pub field_0x0000: u64,
    pub field_0x0008: u16,
    pub field_0x000a: u8,
}

impl ccEventProgress {
    pub const CLASS_NAME: &'static str = "ccEventProgress";
    pub const VTABLE_RVA: u32 = 0x008b67a8;
    pub const METHOD_COUNT: usize = 4;
    pub const BASE_CLASSES: &'static [&'static str] = &["ccEventBase"];
    pub const VIRTUAL_METHODS: &'static [u64] = &[0x0000000180386040, 0x0000000180380cf0, 0x00000001800d3090, 0x000000018038e440];

    pub fn new() -> Self {
        Self {
            vtable_rva: Self::VTABLE_RVA,
            field_0x0000: Default::default(),
            field_0x0008: Default::default(),
            field_0x000a: Default::default(),
        }
    }

    /// Decompiled virtual method #0 @ RVA 0x00386040 (18 instructions, 3 basic blocks)
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

    /// Decompiled virtual method #3 @ RVA 0x0038e440 (12 instructions, 2 basic blocks)
    pub fn method_03(&mut self) {
    }

}

impl CcClass for ccEventProgress {
    fn class_name(&self) -> &'static str { Self::CLASS_NAME }
    fn vtable_rva(&self) -> u32 { Self::VTABLE_RVA }
    fn virtual_method_count(&self) -> usize { Self::METHOD_COUNT }
    fn base_classes(&self) -> &'static [&'static str] { Self::BASE_CLASSES }
}

/// Decompiled CC2 class: `ccEventPupFlg` (VTable RVA: 0x008b6a00, 4 virtual methods, 3 recovered fields)
/// Inherits: ccEventBase
#[derive(Debug, Clone, Default)]
pub struct ccEventPupFlg {
    pub vtable_rva: u32,
    pub field_0x0000: u64,
    pub field_0x0008: u16,
    pub field_0x000a: u8,
}

impl ccEventPupFlg {
    pub const CLASS_NAME: &'static str = "ccEventPupFlg";
    pub const VTABLE_RVA: u32 = 0x008b6a00;
    pub const METHOD_COUNT: usize = 4;
    pub const BASE_CLASSES: &'static [&'static str] = &["ccEventBase"];
    pub const VIRTUAL_METHODS: &'static [u64] = &[0x0000000180385c30, 0x0000000180380cf0, 0x00000001800d3090, 0x000000018038ce00];

    pub fn new() -> Self {
        Self {
            vtable_rva: Self::VTABLE_RVA,
            field_0x0000: Default::default(),
            field_0x0008: Default::default(),
            field_0x000a: Default::default(),
        }
    }

    /// Decompiled virtual method #0 @ RVA 0x00385c30 (18 instructions, 3 basic blocks)
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

    /// Decompiled virtual method #3 @ RVA 0x0038ce00 (21 instructions, 5 basic blocks)
    pub fn method_03(&mut self) {
    }

}

impl CcClass for ccEventPupFlg {
    fn class_name(&self) -> &'static str { Self::CLASS_NAME }
    fn vtable_rva(&self) -> u32 { Self::VTABLE_RVA }
    fn virtual_method_count(&self) -> usize { Self::METHOD_COUNT }
    fn base_classes(&self) -> &'static [&'static str] { Self::BASE_CLASSES }
}

/// Decompiled CC2 class: `ccEventPupTrg` (VTable RVA: 0x008b6960, 4 virtual methods, 3 recovered fields)
/// Inherits: ccEventBase
#[derive(Debug, Clone, Default)]
pub struct ccEventPupTrg {
    pub vtable_rva: u32,
    pub field_0x0000: u64,
    pub field_0x0008: u16,
    pub field_0x000a: u8,
}

impl ccEventPupTrg {
    pub const CLASS_NAME: &'static str = "ccEventPupTrg";
    pub const VTABLE_RVA: u32 = 0x008b6960;
    pub const METHOD_COUNT: usize = 4;
    pub const BASE_CLASSES: &'static [&'static str] = &["ccEventBase"];
    pub const VIRTUAL_METHODS: &'static [u64] = &[0x0000000180385bf0, 0x0000000180380cf0, 0x00000001800d3090, 0x000000018038cd70];

    pub fn new() -> Self {
        Self {
            vtable_rva: Self::VTABLE_RVA,
            field_0x0000: Default::default(),
            field_0x0008: Default::default(),
            field_0x000a: Default::default(),
        }
    }

    /// Decompiled virtual method #0 @ RVA 0x00385bf0 (18 instructions, 3 basic blocks)
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

    /// Decompiled virtual method #3 @ RVA 0x0038cd70 (22 instructions, 5 basic blocks)
    pub fn method_03(&mut self) {
    }

}

impl CcClass for ccEventPupTrg {
    fn class_name(&self) -> &'static str { Self::CLASS_NAME }
    fn vtable_rva(&self) -> u32 { Self::VTABLE_RVA }
    fn virtual_method_count(&self) -> usize { Self::METHOD_COUNT }
    fn base_classes(&self) -> &'static [&'static str] { Self::BASE_CLASSES }
}

/// Decompiled CC2 class: `ccEventPuppet` (VTable RVA: 0x008b6988, 4 virtual methods, 4 recovered fields)
/// Inherits: ccEventBase
#[derive(Debug, Clone, Default)]
pub struct ccEventPuppet {
    pub vtable_rva: u32,
    pub field_0x0000: u64,
    pub field_0x0008: u16,
    pub field_0x000a: u8,
    pub field_0x0020: u32,
}

impl ccEventPuppet {
    pub const CLASS_NAME: &'static str = "ccEventPuppet";
    pub const VTABLE_RVA: u32 = 0x008b6988;
    pub const METHOD_COUNT: usize = 4;
    pub const BASE_CLASSES: &'static [&'static str] = &["ccEventBase"];
    pub const VIRTUAL_METHODS: &'static [u64] = &[0x0000000180385b80, 0x0000000180380cf0, 0x00000001800d3090, 0x000000018038cb50];

    pub fn new() -> Self {
        Self {
            vtable_rva: Self::VTABLE_RVA,
            field_0x0000: Default::default(),
            field_0x0008: Default::default(),
            field_0x000a: Default::default(),
            field_0x0020: Default::default(),
        }
    }

    /// Decompiled virtual method #0 @ RVA 0x00385b80 (31 instructions, 7 basic blocks)
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

    /// Decompiled virtual method #3 @ RVA 0x0038cb50 (12 instructions, 2 basic blocks)
    pub fn method_03(&mut self) {
    }

}

impl CcClass for ccEventPuppet {
    fn class_name(&self) -> &'static str { Self::CLASS_NAME }
    fn vtable_rva(&self) -> u32 { Self::VTABLE_RVA }
    fn virtual_method_count(&self) -> usize { Self::METHOD_COUNT }
    fn base_classes(&self) -> &'static [&'static str] { Self::BASE_CLASSES }
}

/// Decompiled CC2 class: `ccEventQuest` (VTable RVA: 0x008b6870, 4 virtual methods, 3 recovered fields)
/// Inherits: ccEventBase
#[derive(Debug, Clone, Default)]
pub struct ccEventQuest {
    pub vtable_rva: u32,
    pub field_0x0000: u64,
    pub field_0x0008: u16,
    pub field_0x000a: u8,
}

impl ccEventQuest {
    pub const CLASS_NAME: &'static str = "ccEventQuest";
    pub const VTABLE_RVA: u32 = 0x008b6870;
    pub const METHOD_COUNT: usize = 4;
    pub const BASE_CLASSES: &'static [&'static str] = &["ccEventBase"];
    pub const VIRTUAL_METHODS: &'static [u64] = &[0x00000001803860c0, 0x0000000180380cf0, 0x00000001800d3090, 0x000000018038ecf0];

    pub fn new() -> Self {
        Self {
            vtable_rva: Self::VTABLE_RVA,
            field_0x0000: Default::default(),
            field_0x0008: Default::default(),
            field_0x000a: Default::default(),
        }
    }

    /// Decompiled virtual method #0 @ RVA 0x003860c0 (18 instructions, 3 basic blocks)
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

    /// Decompiled virtual method #3 @ RVA 0x0038ecf0 (12 instructions, 2 basic blocks)
    pub fn method_03(&mut self) {
    }

}

impl CcClass for ccEventQuest {
    fn class_name(&self) -> &'static str { Self::CLASS_NAME }
    fn vtable_rva(&self) -> u32 { Self::VTABLE_RVA }
    fn virtual_method_count(&self) -> usize { Self::METHOD_COUNT }
    fn base_classes(&self) -> &'static [&'static str] { Self::BASE_CLASSES }
}

/// Decompiled CC2 class: `ccEventRecordbook` (VTable RVA: 0x008b5698, 4 virtual methods, 3 recovered fields)
/// Inherits: ccEventBase
#[derive(Debug, Clone, Default)]
pub struct ccEventRecordbook {
    pub vtable_rva: u32,
    pub field_0x0000: u64,
    pub field_0x0008: u16,
    pub field_0x000a: u8,
}

impl ccEventRecordbook {
    pub const CLASS_NAME: &'static str = "ccEventRecordbook";
    pub const VTABLE_RVA: u32 = 0x008b5698;
    pub const METHOD_COUNT: usize = 4;
    pub const BASE_CLASSES: &'static [&'static str] = &["ccEventBase"];
    pub const VIRTUAL_METHODS: &'static [u64] = &[0x0000000180386ac0, 0x0000000180380cf0, 0x00000001800d3090, 0x0000000180392fa0];

    pub fn new() -> Self {
        Self {
            vtable_rva: Self::VTABLE_RVA,
            field_0x0000: Default::default(),
            field_0x0008: Default::default(),
            field_0x000a: Default::default(),
        }
    }

    /// Decompiled virtual method #0 @ RVA 0x00386ac0 (18 instructions, 3 basic blocks)
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

    /// Decompiled virtual method #3 @ RVA 0x00392fa0 (119 instructions, 19 basic blocks)
    pub fn method_03(&mut self) {
    }

}

impl CcClass for ccEventRecordbook {
    fn class_name(&self) -> &'static str { Self::CLASS_NAME }
    fn vtable_rva(&self) -> u32 { Self::VTABLE_RVA }
    fn virtual_method_count(&self) -> usize { Self::METHOD_COUNT }
    fn base_classes(&self) -> &'static [&'static str] { Self::BASE_CLASSES }
}

/// Decompiled CC2 class: `ccEventReverse` (VTable RVA: 0x008b6760, 4 virtual methods, 5 recovered fields)
/// Inherits: ccEventBase, ccPDEffectReverse, ccPDEffectBase
#[derive(Debug, Clone, Default)]
pub struct ccEventReverse {
    pub vtable_rva: u32,
    pub field_0x0000: u64,
    pub field_0x0008: u16,
    pub field_0x000a: u8,
    pub field_0x0020: u64,
    pub field_0x0068: u64,
}

impl ccEventReverse {
    pub const CLASS_NAME: &'static str = "ccEventReverse";
    pub const VTABLE_RVA: u32 = 0x008b6760;
    pub const METHOD_COUNT: usize = 4;
    pub const BASE_CLASSES: &'static [&'static str] = &["ccEventBase", "ccPDEffectReverse", "ccPDEffectBase"];
    pub const VIRTUAL_METHODS: &'static [u64] = &[0x0000000180386510, 0x0000000180380cf0, 0x000000018038ff40, 0x000000018038fde0];

    pub fn new() -> Self {
        Self {
            vtable_rva: Self::VTABLE_RVA,
            field_0x0000: Default::default(),
            field_0x0008: Default::default(),
            field_0x000a: Default::default(),
            field_0x0020: Default::default(),
            field_0x0068: Default::default(),
        }
    }

    /// Decompiled virtual method #0 @ RVA 0x00386510 (36 instructions, 5 basic blocks)
    pub fn method_00(&mut self) {
        // Writes to field at offset +0x0000
        // Writes to field at offset +0x0020
    }

    /// Decompiled virtual method #1 @ RVA 0x00380cf0 (13 instructions, 2 basic blocks)
    pub fn method_01(&mut self) {
        // Writes to field at offset +0x0008
        // Writes to field at offset +0x000a
    }

    /// Decompiled virtual method #2 @ RVA 0x0038ff40 (41 instructions, 5 basic blocks)
    pub fn method_02(&mut self) {
    }

    /// Decompiled virtual method #3 @ RVA 0x0038fde0 (92 instructions, 18 basic blocks)
    pub fn method_03(&mut self) {
    }

}

impl CcClass for ccEventReverse {
    fn class_name(&self) -> &'static str { Self::CLASS_NAME }
    fn vtable_rva(&self) -> u32 { Self::VTABLE_RVA }
    fn virtual_method_count(&self) -> usize { Self::METHOD_COUNT }
    fn base_classes(&self) -> &'static [&'static str] { Self::BASE_CLASSES }
}

/// Decompiled CC2 class: `ccEventRoot` (VTable RVA: 0x008b3ee0, 4 virtual methods, 5 recovered fields)
/// Inherits: ccEventBase
#[derive(Debug, Clone, Default)]
pub struct ccEventRoot {
    pub vtable_rva: u32,
    pub field_0x0000: u64,
    pub field_0x0004: u32,
    pub field_0x0008: u16,
    pub field_0x000a: u8,
    pub field_0x2c00: u8,
}

impl ccEventRoot {
    pub const CLASS_NAME: &'static str = "ccEventRoot";
    pub const VTABLE_RVA: u32 = 0x008b3ee0;
    pub const METHOD_COUNT: usize = 4;
    pub const BASE_CLASSES: &'static [&'static str] = &["ccEventBase"];
    pub const VIRTUAL_METHODS: &'static [u64] = &[0x0000000180381120, 0x0000000180381560, 0x00000001800d3090, 0x0000000180381220];

    pub fn new() -> Self {
        Self {
            vtable_rva: Self::VTABLE_RVA,
            field_0x0000: Default::default(),
            field_0x0004: Default::default(),
            field_0x0008: Default::default(),
            field_0x000a: Default::default(),
            field_0x2c00: Default::default(),
        }
    }

    /// Decompiled virtual method #0 @ RVA 0x00381120 (18 instructions, 3 basic blocks)
    pub fn method_00(&mut self) {
        // Writes to field at offset +0x0000
    }

    /// Decompiled virtual method #1 @ RVA 0x00381560 (13 instructions, 2 basic blocks)
    pub fn method_01(&mut self) {
        // Writes to field at offset +0x0008
        // Writes to field at offset +0x000a
    }

    /// Decompiled virtual method #2 @ RVA 0x000d3090 (1 instructions, 1 basic blocks)
    pub fn method_02(&mut self) {
    }

    /// Decompiled virtual method #3 @ RVA 0x00381220 (119 instructions, 28 basic blocks)
    pub fn method_03(&mut self) {
        // Writes to field at offset +0x2c00
    }

}

impl CcClass for ccEventRoot {
    fn class_name(&self) -> &'static str { Self::CLASS_NAME }
    fn vtable_rva(&self) -> u32 { Self::VTABLE_RVA }
    fn virtual_method_count(&self) -> usize { Self::METHOD_COUNT }
    fn base_classes(&self) -> &'static [&'static str] { Self::BASE_CLASSES }
}

/// Decompiled CC2 class: `ccEventRotate` (VTable RVA: 0x008b6d70, 4 virtual methods, 3 recovered fields)
/// Inherits: ccEventBase
#[derive(Debug, Clone, Default)]
pub struct ccEventRotate {
    pub vtable_rva: u32,
    pub field_0x0000: u64,
    pub field_0x0008: u16,
    pub field_0x000a: u8,
}

impl ccEventRotate {
    pub const CLASS_NAME: &'static str = "ccEventRotate";
    pub const VTABLE_RVA: u32 = 0x008b6d70;
    pub const METHOD_COUNT: usize = 4;
    pub const BASE_CLASSES: &'static [&'static str] = &["ccEventBase"];
    pub const VIRTUAL_METHODS: &'static [u64] = &[0x0000000180385740, 0x0000000180380cf0, 0x00000001800d3090, 0x000000018038ab80];

    pub fn new() -> Self {
        Self {
            vtable_rva: Self::VTABLE_RVA,
            field_0x0000: Default::default(),
            field_0x0008: Default::default(),
            field_0x000a: Default::default(),
        }
    }

    /// Decompiled virtual method #0 @ RVA 0x00385740 (18 instructions, 3 basic blocks)
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

    /// Decompiled virtual method #3 @ RVA 0x0038ab80 (12 instructions, 2 basic blocks)
    pub fn method_03(&mut self) {
    }

}

impl CcClass for ccEventRotate {
    fn class_name(&self) -> &'static str { Self::CLASS_NAME }
    fn vtable_rva(&self) -> u32 { Self::VTABLE_RVA }
    fn virtual_method_count(&self) -> usize { Self::METHOD_COUNT }
    fn base_classes(&self) -> &'static [&'static str] { Self::BASE_CLASSES }
}

/// Decompiled CC2 class: `ccEventS107OnOffChimCrown` (VTable RVA: 0x008b52b0, 4 virtual methods, 6 recovered fields)
/// Inherits: ccEventBase
#[derive(Debug, Clone, Default)]
pub struct ccEventS107OnOffChimCrown {
    pub vtable_rva: u32,
    pub field_0x0000: u64,
    pub field_0x0008: u16,
    pub field_0x000a: u8,
    pub field_0x02ec: u32,
    pub field_0x02f0: u32,
    pub field_0x02f4: u32,
}

impl ccEventS107OnOffChimCrown {
    pub const CLASS_NAME: &'static str = "ccEventS107OnOffChimCrown";
    pub const VTABLE_RVA: u32 = 0x008b52b0;
    pub const METHOD_COUNT: usize = 4;
    pub const BASE_CLASSES: &'static [&'static str] = &["ccEventBase"];
    pub const VIRTUAL_METHODS: &'static [u64] = &[0x0000000180386fc0, 0x0000000180380cf0, 0x00000001800d3090, 0x00000001803949c0];

    pub fn new() -> Self {
        Self {
            vtable_rva: Self::VTABLE_RVA,
            field_0x0000: Default::default(),
            field_0x0008: Default::default(),
            field_0x000a: Default::default(),
            field_0x02ec: Default::default(),
            field_0x02f0: Default::default(),
            field_0x02f4: Default::default(),
        }
    }

    /// Decompiled virtual method #0 @ RVA 0x00386fc0 (18 instructions, 3 basic blocks)
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

    /// Decompiled virtual method #3 @ RVA 0x003949c0 (49 instructions, 12 basic blocks)
    pub fn method_03(&mut self) {
    }

}

impl CcClass for ccEventS107OnOffChimCrown {
    fn class_name(&self) -> &'static str { Self::CLASS_NAME }
    fn vtable_rva(&self) -> u32 { Self::VTABLE_RVA }
    fn virtual_method_count(&self) -> usize { Self::METHOD_COUNT }
    fn base_classes(&self) -> &'static [&'static str] { Self::BASE_CLASSES }
}

/// Decompiled CC2 class: `ccEventScene` (VTable RVA: 0x008b6bb8, 4 virtual methods, 3 recovered fields)
/// Inherits: ccEventBase
#[derive(Debug, Clone, Default)]
pub struct ccEventScene {
    pub vtable_rva: u32,
    pub field_0x0000: u64,
    pub field_0x0008: u16,
    pub field_0x000a: u8,
}

impl ccEventScene {
    pub const CLASS_NAME: &'static str = "ccEventScene";
    pub const VTABLE_RVA: u32 = 0x008b6bb8;
    pub const METHOD_COUNT: usize = 4;
    pub const BASE_CLASSES: &'static [&'static str] = &["ccEventBase"];
    pub const VIRTUAL_METHODS: &'static [u64] = &[0x0000000180385800, 0x0000000180380cf0, 0x00000001800d3090, 0x000000018038a110];

    pub fn new() -> Self {
        Self {
            vtable_rva: Self::VTABLE_RVA,
            field_0x0000: Default::default(),
            field_0x0008: Default::default(),
            field_0x000a: Default::default(),
        }
    }

    /// Decompiled virtual method #0 @ RVA 0x00385800 (18 instructions, 3 basic blocks)
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

    /// Decompiled virtual method #3 @ RVA 0x0038a110 (28 instructions, 6 basic blocks)
    pub fn method_03(&mut self) {
    }

}

impl CcClass for ccEventScene {
    fn class_name(&self) -> &'static str { Self::CLASS_NAME }
    fn vtable_rva(&self) -> u32 { Self::VTABLE_RVA }
    fn virtual_method_count(&self) -> usize { Self::METHOD_COUNT }
    fn base_classes(&self) -> &'static [&'static str] { Self::BASE_CLASSES }
}

/// Decompiled CC2 class: `ccEventSeOn` (VTable RVA: 0x008b5260, 4 virtual methods, 3 recovered fields)
/// Inherits: ccEventBase
#[derive(Debug, Clone, Default)]
pub struct ccEventSeOn {
    pub vtable_rva: u32,
    pub field_0x0000: u64,
    pub field_0x0008: u16,
    pub field_0x000a: u8,
}

impl ccEventSeOn {
    pub const CLASS_NAME: &'static str = "ccEventSeOn";
    pub const VTABLE_RVA: u32 = 0x008b5260;
    pub const METHOD_COUNT: usize = 4;
    pub const BASE_CLASSES: &'static [&'static str] = &["ccEventBase"];
    pub const VIRTUAL_METHODS: &'static [u64] = &[0x0000000180387040, 0x0000000180380cf0, 0x00000001800d3090, 0x0000000180394b10];

    pub fn new() -> Self {
        Self {
            vtable_rva: Self::VTABLE_RVA,
            field_0x0000: Default::default(),
            field_0x0008: Default::default(),
            field_0x000a: Default::default(),
        }
    }

    /// Decompiled virtual method #0 @ RVA 0x00387040 (18 instructions, 3 basic blocks)
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

    /// Decompiled virtual method #3 @ RVA 0x00394b10 (56 instructions, 18 basic blocks)
    pub fn method_03(&mut self) {
    }

}

impl CcClass for ccEventSeOn {
    fn class_name(&self) -> &'static str { Self::CLASS_NAME }
    fn vtable_rva(&self) -> u32 { Self::VTABLE_RVA }
    fn virtual_method_count(&self) -> usize { Self::METHOD_COUNT }
    fn base_classes(&self) -> &'static [&'static str] { Self::BASE_CLASSES }
}

/// Decompiled CC2 class: `ccEventSearch` (VTable RVA: 0x008b3e90, 4 virtual methods, 3 recovered fields)
/// Inherits: ccEventBase
#[derive(Debug, Clone, Default)]
pub struct ccEventSearch {
    pub vtable_rva: u32,
    pub field_0x0000: u64,
    pub field_0x000a: u8,
    pub field_0x0020: u8,
}

impl ccEventSearch {
    pub const CLASS_NAME: &'static str = "ccEventSearch";
    pub const VTABLE_RVA: u32 = 0x008b3e90;
    pub const METHOD_COUNT: usize = 4;
    pub const BASE_CLASSES: &'static [&'static str] = &["ccEventBase"];
    pub const VIRTUAL_METHODS: &'static [u64] = &[0x0000000180381cf0, 0x0000000180381df0, 0x00000001800d3090, 0x0000000180381d30];

    pub fn new() -> Self {
        Self {
            vtable_rva: Self::VTABLE_RVA,
            field_0x0000: Default::default(),
            field_0x000a: Default::default(),
            field_0x0020: Default::default(),
        }
    }

    /// Decompiled virtual method #0 @ RVA 0x00381cf0 (18 instructions, 3 basic blocks)
    pub fn method_00(&mut self) {
        // Writes to field at offset +0x0000
    }

    /// Decompiled virtual method #1 @ RVA 0x00381df0 (60 instructions, 11 basic blocks)
    pub fn method_01(&mut self) {
        // Writes to field at offset +0x0020
    }

    /// Decompiled virtual method #2 @ RVA 0x000d3090 (1 instructions, 1 basic blocks)
    pub fn method_02(&mut self) {
    }

    /// Decompiled virtual method #3 @ RVA 0x00381d30 (30 instructions, 10 basic blocks)
    pub fn method_03(&mut self) {
    }

}

impl CcClass for ccEventSearch {
    fn class_name(&self) -> &'static str { Self::CLASS_NAME }
    fn vtable_rva(&self) -> u32 { Self::VTABLE_RVA }
    fn virtual_method_count(&self) -> usize { Self::METHOD_COUNT }
    fn base_classes(&self) -> &'static [&'static str] { Self::BASE_CLASSES }
}

/// Decompiled CC2 class: `ccEventSeek` (VTable RVA: 0x008b3e40, 4 virtual methods, 3 recovered fields)
/// Inherits: ccEventBase
#[derive(Debug, Clone, Default)]
pub struct ccEventSeek {
    pub vtable_rva: u32,
    pub field_0x0000: u64,
    pub field_0x0008: u16,
    pub field_0x000a: u8,
}

impl ccEventSeek {
    pub const CLASS_NAME: &'static str = "ccEventSeek";
    pub const VTABLE_RVA: u32 = 0x008b3e40;
    pub const METHOD_COUNT: usize = 4;
    pub const BASE_CLASSES: &'static [&'static str] = &["ccEventBase"];
    pub const VIRTUAL_METHODS: &'static [u64] = &[0x0000000180382090, 0x0000000180380cf0, 0x00000001800d3090, 0x00000001803820d0];

    pub fn new() -> Self {
        Self {
            vtable_rva: Self::VTABLE_RVA,
            field_0x0000: Default::default(),
            field_0x0008: Default::default(),
            field_0x000a: Default::default(),
        }
    }

    /// Decompiled virtual method #0 @ RVA 0x00382090 (18 instructions, 3 basic blocks)
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

    /// Decompiled virtual method #3 @ RVA 0x003820d0 (12 instructions, 2 basic blocks)
    pub fn method_03(&mut self) {
    }

}

impl CcClass for ccEventSeek {
    fn class_name(&self) -> &'static str { Self::CLASS_NAME }
    fn vtable_rva(&self) -> u32 { Self::VTABLE_RVA }
    fn virtual_method_count(&self) -> usize { Self::METHOD_COUNT }
    fn base_classes(&self) -> &'static [&'static str] { Self::BASE_CLASSES }
}

/// Decompiled CC2 class: `ccEventSendMsgKingChim` (VTable RVA: 0x008b53c8, 4 virtual methods, 4 recovered fields)
/// Inherits: ccEventBase
#[derive(Debug, Clone, Default)]
pub struct ccEventSendMsgKingChim {
    pub vtable_rva: u32,
    pub field_0x0000: u64,
    pub field_0x0008: u16,
    pub field_0x000a: u8,
    pub field_0x0800: u32,
}

impl ccEventSendMsgKingChim {
    pub const CLASS_NAME: &'static str = "ccEventSendMsgKingChim";
    pub const VTABLE_RVA: u32 = 0x008b53c8;
    pub const METHOD_COUNT: usize = 4;
    pub const BASE_CLASSES: &'static [&'static str] = &["ccEventBase"];
    pub const VIRTUAL_METHODS: &'static [u64] = &[0x0000000180386f40, 0x0000000180380cf0, 0x00000001800d3090, 0x00000001803947d0];

    pub fn new() -> Self {
        Self {
            vtable_rva: Self::VTABLE_RVA,
            field_0x0000: Default::default(),
            field_0x0008: Default::default(),
            field_0x000a: Default::default(),
            field_0x0800: Default::default(),
        }
    }

    /// Decompiled virtual method #0 @ RVA 0x00386f40 (18 instructions, 3 basic blocks)
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

    /// Decompiled virtual method #3 @ RVA 0x003947d0 (89 instructions, 17 basic blocks)
    pub fn method_03(&mut self) {
        // Writes to field at offset +0x0000
        // Writes to field at offset +0x0000
    }

}

impl CcClass for ccEventSendMsgKingChim {
    fn class_name(&self) -> &'static str { Self::CLASS_NAME }
    fn vtable_rva(&self) -> u32 { Self::VTABLE_RVA }
    fn virtual_method_count(&self) -> usize { Self::METHOD_COUNT }
    fn base_classes(&self) -> &'static [&'static str] { Self::BASE_CLASSES }
}

/// Decompiled CC2 class: `ccEventSetBossParam` (VTable RVA: 0x008b54e0, 4 virtual methods, 6 recovered fields)
/// Inherits: ccEventBase
#[derive(Debug, Clone, Default)]
pub struct ccEventSetBossParam {
    pub vtable_rva: u32,
    pub field_0x0000: u64,
    pub field_0x0008: u16,
    pub field_0x000a: u8,
    pub field_0x0018: u32,
    pub field_0x0028: u32,
    pub field_0x0030: u32,
}

impl ccEventSetBossParam {
    pub const CLASS_NAME: &'static str = "ccEventSetBossParam";
    pub const VTABLE_RVA: u32 = 0x008b54e0;
    pub const METHOD_COUNT: usize = 4;
    pub const BASE_CLASSES: &'static [&'static str] = &["ccEventBase"];
    pub const VIRTUAL_METHODS: &'static [u64] = &[0x0000000180386dc0, 0x0000000180380cf0, 0x00000001800d3090, 0x00000001803942d0];

    pub fn new() -> Self {
        Self {
            vtable_rva: Self::VTABLE_RVA,
            field_0x0000: Default::default(),
            field_0x0008: Default::default(),
            field_0x000a: Default::default(),
            field_0x0018: Default::default(),
            field_0x0028: Default::default(),
            field_0x0030: Default::default(),
        }
    }

    /// Decompiled virtual method #0 @ RVA 0x00386dc0 (18 instructions, 3 basic blocks)
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

    /// Decompiled virtual method #3 @ RVA 0x003942d0 (41 instructions, 17 basic blocks)
    pub fn method_03(&mut self) {
        // Writes to field at offset +0x0018
        // Writes to field at offset +0x0028
        // Writes to field at offset +0x0030
    }

}

impl CcClass for ccEventSetBossParam {
    fn class_name(&self) -> &'static str { Self::CLASS_NAME }
    fn vtable_rva(&self) -> u32 { Self::VTABLE_RVA }
    fn virtual_method_count(&self) -> usize { Self::METHOD_COUNT }
    fn base_classes(&self) -> &'static [&'static str] { Self::BASE_CLASSES }
}

/// Decompiled CC2 class: `ccEventSetSpc0PosRot` (VTable RVA: 0x008b5378, 4 virtual methods, 3 recovered fields)
/// Inherits: ccEventBase
#[derive(Debug, Clone, Default)]
pub struct ccEventSetSpc0PosRot {
    pub vtable_rva: u32,
    pub field_0x0000: u64,
    pub field_0x0008: u16,
    pub field_0x000a: u8,
}

impl ccEventSetSpc0PosRot {
    pub const CLASS_NAME: &'static str = "ccEventSetSpc0PosRot";
    pub const VTABLE_RVA: u32 = 0x008b5378;
    pub const METHOD_COUNT: usize = 4;
    pub const BASE_CLASSES: &'static [&'static str] = &["ccEventBase"];
    pub const VIRTUAL_METHODS: &'static [u64] = &[0x0000000180386e80, 0x0000000180380cf0, 0x00000001800d3090, 0x0000000180394560];

    pub fn new() -> Self {
        Self {
            vtable_rva: Self::VTABLE_RVA,
            field_0x0000: Default::default(),
            field_0x0008: Default::default(),
            field_0x000a: Default::default(),
        }
    }

    /// Decompiled virtual method #0 @ RVA 0x00386e80 (18 instructions, 3 basic blocks)
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

    /// Decompiled virtual method #3 @ RVA 0x00394560 (44 instructions, 9 basic blocks)
    pub fn method_03(&mut self) {
    }

}

impl CcClass for ccEventSetSpc0PosRot {
    fn class_name(&self) -> &'static str { Self::CLASS_NAME }
    fn vtable_rva(&self) -> u32 { Self::VTABLE_RVA }
    fn virtual_method_count(&self) -> usize { Self::METHOD_COUNT }
    fn base_classes(&self) -> &'static [&'static str] { Self::BASE_CLASSES }
}

/// Decompiled CC2 class: `ccEventSetSpcPosRot` (VTable RVA: 0x008b5418, 4 virtual methods, 3 recovered fields)
/// Inherits: ccEventBase
#[derive(Debug, Clone, Default)]
pub struct ccEventSetSpcPosRot {
    pub vtable_rva: u32,
    pub field_0x0000: u64,
    pub field_0x0008: u16,
    pub field_0x000a: u8,
}

impl ccEventSetSpcPosRot {
    pub const CLASS_NAME: &'static str = "ccEventSetSpcPosRot";
    pub const VTABLE_RVA: u32 = 0x008b5418;
    pub const METHOD_COUNT: usize = 4;
    pub const BASE_CLASSES: &'static [&'static str] = &["ccEventBase"];
    pub const VIRTUAL_METHODS: &'static [u64] = &[0x0000000180386ec0, 0x0000000180380cf0, 0x00000001800d3090, 0x0000000180394610];

    pub fn new() -> Self {
        Self {
            vtable_rva: Self::VTABLE_RVA,
            field_0x0000: Default::default(),
            field_0x0008: Default::default(),
            field_0x000a: Default::default(),
        }
    }

    /// Decompiled virtual method #0 @ RVA 0x00386ec0 (18 instructions, 3 basic blocks)
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

    /// Decompiled virtual method #3 @ RVA 0x00394610 (75 instructions, 10 basic blocks)
    pub fn method_03(&mut self) {
    }

}

impl CcClass for ccEventSetSpcPosRot {
    fn class_name(&self) -> &'static str { Self::CLASS_NAME }
    fn vtable_rva(&self) -> u32 { Self::VTABLE_RVA }
    fn virtual_method_count(&self) -> usize { Self::METHOD_COUNT }
    fn base_classes(&self) -> &'static [&'static str] { Self::BASE_CLASSES }
}

/// Decompiled CC2 class: `ccEventSpAbility` (VTable RVA: 0x008b5508, 4 virtual methods, 4 recovered fields)
/// Inherits: ccEventBase
#[derive(Debug, Clone, Default)]
pub struct ccEventSpAbility {
    pub vtable_rva: u32,
    pub field_0x0000: u64,
    pub field_0x0004: u32,
    pub field_0x0008: u16,
    pub field_0x000a: u8,
}

impl ccEventSpAbility {
    pub const CLASS_NAME: &'static str = "ccEventSpAbility";
    pub const VTABLE_RVA: u32 = 0x008b5508;
    pub const METHOD_COUNT: usize = 4;
    pub const BASE_CLASSES: &'static [&'static str] = &["ccEventBase"];
    pub const VIRTUAL_METHODS: &'static [u64] = &[0x0000000180386e00, 0x0000000180380cf0, 0x00000001800d3090, 0x00000001803934b0];

    pub fn new() -> Self {
        Self {
            vtable_rva: Self::VTABLE_RVA,
            field_0x0000: Default::default(),
            field_0x0004: Default::default(),
            field_0x0008: Default::default(),
            field_0x000a: Default::default(),
        }
    }

    /// Decompiled virtual method #0 @ RVA 0x00386e00 (18 instructions, 3 basic blocks)
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

    /// Decompiled virtual method #3 @ RVA 0x003934b0 (32 instructions, 11 basic blocks)
    pub fn method_03(&mut self) {
    }

}

impl CcClass for ccEventSpAbility {
    fn class_name(&self) -> &'static str { Self::CLASS_NAME }
    fn vtable_rva(&self) -> u32 { Self::VTABLE_RVA }
    fn virtual_method_count(&self) -> usize { Self::METHOD_COUNT }
    fn base_classes(&self) -> &'static [&'static str] { Self::BASE_CLASSES }
}

/// Decompiled CC2 class: `ccEventStaffroll` (VTable RVA: 0x008b5288, 4 virtual methods, 4 recovered fields)
/// Inherits: ccEventBase
#[derive(Debug, Clone, Default)]
pub struct ccEventStaffroll {
    pub vtable_rva: u32,
    pub field_0x0000: u64,
    pub field_0x0008: u16,
    pub field_0x000a: u8,
    pub field_0x000c: u32,
}

impl ccEventStaffroll {
    pub const CLASS_NAME: &'static str = "ccEventStaffroll";
    pub const VTABLE_RVA: u32 = 0x008b5288;
    pub const METHOD_COUNT: usize = 4;
    pub const BASE_CLASSES: &'static [&'static str] = &["ccEventBase"];
    pub const VIRTUAL_METHODS: &'static [u64] = &[0x0000000180387080, 0x0000000180380cf0, 0x00000001800d3090, 0x0000000180394d50];

    pub fn new() -> Self {
        Self {
            vtable_rva: Self::VTABLE_RVA,
            field_0x0000: Default::default(),
            field_0x0008: Default::default(),
            field_0x000a: Default::default(),
            field_0x000c: Default::default(),
        }
    }

    /// Decompiled virtual method #0 @ RVA 0x00387080 (18 instructions, 3 basic blocks)
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

    /// Decompiled virtual method #3 @ RVA 0x00394d50 (53 instructions, 10 basic blocks)
    pub fn method_03(&mut self) {
        // Writes to field at offset +0x000c
    }

}

impl CcClass for ccEventStaffroll {
    fn class_name(&self) -> &'static str { Self::CLASS_NAME }
    fn vtable_rva(&self) -> u32 { Self::VTABLE_RVA }
    fn virtual_method_count(&self) -> usize { Self::METHOD_COUNT }
    fn base_classes(&self) -> &'static [&'static str] { Self::BASE_CLASSES }
}

/// Decompiled CC2 class: `ccEventSubRoot` (VTable RVA: 0x008b3f30, 4 virtual methods, 3 recovered fields)
/// Inherits: ccEventBase
#[derive(Debug, Clone, Default)]
pub struct ccEventSubRoot {
    pub vtable_rva: u32,
    pub field_0x0000: u64,
    pub field_0x0008: u16,
    pub field_0x000a: u8,
}

impl ccEventSubRoot {
    pub const CLASS_NAME: &'static str = "ccEventSubRoot";
    pub const VTABLE_RVA: u32 = 0x008b3f30;
    pub const METHOD_COUNT: usize = 4;
    pub const BASE_CLASSES: &'static [&'static str] = &["ccEventBase"];
    pub const VIRTUAL_METHODS: &'static [u64] = &[0x00000001803817b0, 0x00000001803818d0, 0x00000001800d3090, 0x00000001803817f0];

    pub fn new() -> Self {
        Self {
            vtable_rva: Self::VTABLE_RVA,
            field_0x0000: Default::default(),
            field_0x0008: Default::default(),
            field_0x000a: Default::default(),
        }
    }

    /// Decompiled virtual method #0 @ RVA 0x003817b0 (18 instructions, 3 basic blocks)
    pub fn method_00(&mut self) {
        // Writes to field at offset +0x0000
    }

    /// Decompiled virtual method #1 @ RVA 0x003818d0 (13 instructions, 2 basic blocks)
    pub fn method_01(&mut self) {
        // Writes to field at offset +0x0008
        // Writes to field at offset +0x000a
    }

    /// Decompiled virtual method #2 @ RVA 0x000d3090 (1 instructions, 1 basic blocks)
    pub fn method_02(&mut self) {
    }

    /// Decompiled virtual method #3 @ RVA 0x003817f0 (12 instructions, 2 basic blocks)
    pub fn method_03(&mut self) {
    }

}

impl CcClass for ccEventSubRoot {
    fn class_name(&self) -> &'static str { Self::CLASS_NAME }
    fn vtable_rva(&self) -> u32 { Self::VTABLE_RVA }
    fn virtual_method_count(&self) -> usize { Self::METHOD_COUNT }
    fn base_classes(&self) -> &'static [&'static str] { Self::BASE_CLASSES }
}

/// Decompiled CC2 class: `ccEventSubRootPassive` (VTable RVA: 0x008b3e68, 4 virtual methods, 3 recovered fields)
/// Inherits: ccEventBase
#[derive(Debug, Clone, Default)]
pub struct ccEventSubRootPassive {
    pub vtable_rva: u32,
    pub field_0x0000: u64,
    pub field_0x0008: u16,
    pub field_0x000a: u8,
}

impl ccEventSubRootPassive {
    pub const CLASS_NAME: &'static str = "ccEventSubRootPassive";
    pub const VTABLE_RVA: u32 = 0x008b3e68;
    pub const METHOD_COUNT: usize = 4;
    pub const BASE_CLASSES: &'static [&'static str] = &["ccEventBase"];
    pub const VIRTUAL_METHODS: &'static [u64] = &[0x0000000180381a90, 0x00000001803818d0, 0x00000001800d3090, 0x0000000180381ad0];

    pub fn new() -> Self {
        Self {
            vtable_rva: Self::VTABLE_RVA,
            field_0x0000: Default::default(),
            field_0x0008: Default::default(),
            field_0x000a: Default::default(),
        }
    }

    /// Decompiled virtual method #0 @ RVA 0x00381a90 (18 instructions, 3 basic blocks)
    pub fn method_00(&mut self) {
        // Writes to field at offset +0x0000
    }

    /// Decompiled virtual method #1 @ RVA 0x003818d0 (13 instructions, 2 basic blocks)
    pub fn method_01(&mut self) {
        // Writes to field at offset +0x0008
        // Writes to field at offset +0x000a
    }

    /// Decompiled virtual method #2 @ RVA 0x000d3090 (1 instructions, 1 basic blocks)
    pub fn method_02(&mut self) {
    }

    /// Decompiled virtual method #3 @ RVA 0x00381ad0 (12 instructions, 2 basic blocks)
    pub fn method_03(&mut self) {
    }

}

impl CcClass for ccEventSubRootPassive {
    fn class_name(&self) -> &'static str { Self::CLASS_NAME }
    fn vtable_rva(&self) -> u32 { Self::VTABLE_RVA }
    fn virtual_method_count(&self) -> usize { Self::METHOD_COUNT }
    fn base_classes(&self) -> &'static [&'static str] { Self::BASE_CLASSES }
}

/// Decompiled CC2 class: `ccEventTarget` (VTable RVA: 0x008b6e60, 4 virtual methods, 3 recovered fields)
/// Inherits: ccEventBase
#[derive(Debug, Clone, Default)]
pub struct ccEventTarget {
    pub vtable_rva: u32,
    pub field_0x0000: u64,
    pub field_0x0008: u16,
    pub field_0x000a: u8,
}

impl ccEventTarget {
    pub const CLASS_NAME: &'static str = "ccEventTarget";
    pub const VTABLE_RVA: u32 = 0x008b6e60;
    pub const METHOD_COUNT: usize = 4;
    pub const BASE_CLASSES: &'static [&'static str] = &["ccEventBase"];
    pub const VIRTUAL_METHODS: &'static [u64] = &[0x0000000180385580, 0x0000000180380cf0, 0x00000001800d3090, 0x00000001803881e0];

    pub fn new() -> Self {
        Self {
            vtable_rva: Self::VTABLE_RVA,
            field_0x0000: Default::default(),
            field_0x0008: Default::default(),
            field_0x000a: Default::default(),
        }
    }

    /// Decompiled virtual method #0 @ RVA 0x00385580 (18 instructions, 3 basic blocks)
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

    /// Decompiled virtual method #3 @ RVA 0x003881e0 (12 instructions, 2 basic blocks)
    pub fn method_03(&mut self) {
    }

}

impl CcClass for ccEventTarget {
    fn class_name(&self) -> &'static str { Self::CLASS_NAME }
    fn vtable_rva(&self) -> u32 { Self::VTABLE_RVA }
    fn virtual_method_count(&self) -> usize { Self::METHOD_COUNT }
    fn base_classes(&self) -> &'static [&'static str] { Self::BASE_CLASSES }
}

/// Decompiled CC2 class: `ccEventTeach` (VTable RVA: 0x008b6910, 4 virtual methods, 4 recovered fields)
/// Inherits: ccEventBase
#[derive(Debug, Clone, Default)]
pub struct ccEventTeach {
    pub vtable_rva: u32,
    pub field_0x0000: u64,
    pub field_0x0008: u16,
    pub field_0x000a: u8,
    pub field_0x0024: u32,
}

impl ccEventTeach {
    pub const CLASS_NAME: &'static str = "ccEventTeach";
    pub const VTABLE_RVA: u32 = 0x008b6910;
    pub const METHOD_COUNT: usize = 4;
    pub const BASE_CLASSES: &'static [&'static str] = &["ccEventBase"];
    pub const VIRTUAL_METHODS: &'static [u64] = &[0x0000000180385d30, 0x0000000180380cf0, 0x00000001800d3090, 0x000000018038d3b0];

    pub fn new() -> Self {
        Self {
            vtable_rva: Self::VTABLE_RVA,
            field_0x0000: Default::default(),
            field_0x0008: Default::default(),
            field_0x000a: Default::default(),
            field_0x0024: Default::default(),
        }
    }

    /// Decompiled virtual method #0 @ RVA 0x00385d30 (18 instructions, 3 basic blocks)
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

    /// Decompiled virtual method #3 @ RVA 0x0038d3b0 (24 instructions, 6 basic blocks)
    pub fn method_03(&mut self) {
    }

}

impl CcClass for ccEventTeach {
    fn class_name(&self) -> &'static str { Self::CLASS_NAME }
    fn vtable_rva(&self) -> u32 { Self::VTABLE_RVA }
    fn virtual_method_count(&self) -> usize { Self::METHOD_COUNT }
    fn base_classes(&self) -> &'static [&'static str] { Self::BASE_CLASSES }
}

/// Decompiled CC2 class: `ccEventTeachBtl` (VTable RVA: 0x008b69d8, 4 virtual methods, 5 recovered fields)
/// Inherits: ccEventBase
#[derive(Debug, Clone, Default)]
pub struct ccEventTeachBtl {
    pub vtable_rva: u32,
    pub field_0x0000: u64,
    pub field_0x0008: u16,
    pub field_0x000a: u8,
    pub field_0x0010: u64,
    pub field_0x0020: u16,
}

impl ccEventTeachBtl {
    pub const CLASS_NAME: &'static str = "ccEventTeachBtl";
    pub const VTABLE_RVA: u32 = 0x008b69d8;
    pub const METHOD_COUNT: usize = 4;
    pub const BASE_CLASSES: &'static [&'static str] = &["ccEventBase"];
    pub const VIRTUAL_METHODS: &'static [u64] = &[0x0000000180385cf0, 0x0000000180380cf0, 0x00000001800d3090, 0x000000018038d350];

    pub fn new() -> Self {
        Self {
            vtable_rva: Self::VTABLE_RVA,
            field_0x0000: Default::default(),
            field_0x0008: Default::default(),
            field_0x000a: Default::default(),
            field_0x0010: Default::default(),
            field_0x0020: Default::default(),
        }
    }

    /// Decompiled virtual method #0 @ RVA 0x00385cf0 (18 instructions, 3 basic blocks)
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

    /// Decompiled virtual method #3 @ RVA 0x0038d350 (27 instructions, 9 basic blocks)
    pub fn method_03(&mut self) {
        // Writes to field at offset +0x000a
    }

}

impl CcClass for ccEventTeachBtl {
    fn class_name(&self) -> &'static str { Self::CLASS_NAME }
    fn vtable_rva(&self) -> u32 { Self::VTABLE_RVA }
    fn virtual_method_count(&self) -> usize { Self::METHOD_COUNT }
    fn base_classes(&self) -> &'static [&'static str] { Self::BASE_CLASSES }
}

/// Decompiled CC2 class: `ccEventTrans` (VTable RVA: 0x008b6d20, 4 virtual methods, 3 recovered fields)
/// Inherits: ccEventBase
#[derive(Debug, Clone, Default)]
pub struct ccEventTrans {
    pub vtable_rva: u32,
    pub field_0x0000: u64,
    pub field_0x0008: u16,
    pub field_0x000a: u8,
}

impl ccEventTrans {
    pub const CLASS_NAME: &'static str = "ccEventTrans";
    pub const VTABLE_RVA: u32 = 0x008b6d20;
    pub const METHOD_COUNT: usize = 4;
    pub const BASE_CLASSES: &'static [&'static str] = &["ccEventBase"];
    pub const VIRTUAL_METHODS: &'static [u64] = &[0x0000000180385780, 0x0000000180380cf0, 0x00000001800d3090, 0x000000018038aee0];

    pub fn new() -> Self {
        Self {
            vtable_rva: Self::VTABLE_RVA,
            field_0x0000: Default::default(),
            field_0x0008: Default::default(),
            field_0x000a: Default::default(),
        }
    }

    /// Decompiled virtual method #0 @ RVA 0x00385780 (18 instructions, 3 basic blocks)
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

    /// Decompiled virtual method #3 @ RVA 0x0038aee0 (115 instructions, 24 basic blocks)
    pub fn method_03(&mut self) {
    }

}

impl CcClass for ccEventTrans {
    fn class_name(&self) -> &'static str { Self::CLASS_NAME }
    fn vtable_rva(&self) -> u32 { Self::VTABLE_RVA }
    fn virtual_method_count(&self) -> usize { Self::METHOD_COUNT }
    fn base_classes(&self) -> &'static [&'static str] { Self::BASE_CLASSES }
}

/// Decompiled CC2 class: `ccEventWait` (VTable RVA: 0x008b6dc0, 4 virtual methods, 3 recovered fields)
/// Inherits: ccEventBase
#[derive(Debug, Clone, Default)]
pub struct ccEventWait {
    pub vtable_rva: u32,
    pub field_0x0000: u64,
    pub field_0x0008: u16,
    pub field_0x000a: u8,
}

impl ccEventWait {
    pub const CLASS_NAME: &'static str = "ccEventWait";
    pub const VTABLE_RVA: u32 = 0x008b6dc0;
    pub const METHOD_COUNT: usize = 4;
    pub const BASE_CLASSES: &'static [&'static str] = &["ccEventBase"];
    pub const VIRTUAL_METHODS: &'static [u64] = &[0x00000001803854c0, 0x0000000180380cf0, 0x00000001800d3090, 0x0000000180387f70];

    pub fn new() -> Self {
        Self {
            vtable_rva: Self::VTABLE_RVA,
            field_0x0000: Default::default(),
            field_0x0008: Default::default(),
            field_0x000a: Default::default(),
        }
    }

    /// Decompiled virtual method #0 @ RVA 0x003854c0 (18 instructions, 3 basic blocks)
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

    /// Decompiled virtual method #3 @ RVA 0x00387f70 (12 instructions, 2 basic blocks)
    pub fn method_03(&mut self) {
    }

}

impl CcClass for ccEventWait {
    fn class_name(&self) -> &'static str { Self::CLASS_NAME }
    fn vtable_rva(&self) -> u32 { Self::VTABLE_RVA }
    fn virtual_method_count(&self) -> usize { Self::METHOD_COUNT }
    fn base_classes(&self) -> &'static [&'static str] { Self::BASE_CLASSES }
}

/// Decompiled CC2 class: `ccEventWeakenBoss` (VTable RVA: 0x008b5530, 4 virtual methods, 5 recovered fields)
/// Inherits: ccEventBase
#[derive(Debug, Clone, Default)]
pub struct ccEventWeakenBoss {
    pub vtable_rva: u32,
    pub field_0x0000: u64,
    pub field_0x0008: u16,
    pub field_0x000a: u8,
    pub field_0x0010: u64,
    pub field_0x0024: u16,
}

impl ccEventWeakenBoss {
    pub const CLASS_NAME: &'static str = "ccEventWeakenBoss";
    pub const VTABLE_RVA: u32 = 0x008b5530;
    pub const METHOD_COUNT: usize = 4;
    pub const BASE_CLASSES: &'static [&'static str] = &["ccEventBase"];
    pub const VIRTUAL_METHODS: &'static [u64] = &[0x0000000180386d80, 0x0000000180380cf0, 0x00000001800d3090, 0x0000000180394280];

    pub fn new() -> Self {
        Self {
            vtable_rva: Self::VTABLE_RVA,
            field_0x0000: Default::default(),
            field_0x0008: Default::default(),
            field_0x000a: Default::default(),
            field_0x0010: Default::default(),
            field_0x0024: Default::default(),
        }
    }

    /// Decompiled virtual method #0 @ RVA 0x00386d80 (18 instructions, 3 basic blocks)
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

    /// Decompiled virtual method #3 @ RVA 0x00394280 (21 instructions, 7 basic blocks)
    pub fn method_03(&mut self) {
        // Writes to field at offset +0x000a
    }

}

impl CcClass for ccEventWeakenBoss {
    fn class_name(&self) -> &'static str { Self::CLASS_NAME }
    fn vtable_rva(&self) -> u32 { Self::VTABLE_RVA }
    fn virtual_method_count(&self) -> usize { Self::METHOD_COUNT }
    fn base_classes(&self) -> &'static [&'static str] { Self::BASE_CLASSES }
}

/// Decompiled CC2 class: `ccEventWonder` (VTable RVA: 0x008b5300, 4 virtual methods, 4 recovered fields)
/// Inherits: ccEventBase
#[derive(Debug, Clone, Default)]
pub struct ccEventWonder {
    pub vtable_rva: u32,
    pub field_0x0000: u64,
    pub field_0x0008: u16,
    pub field_0x000a: u8,
    pub field_0x04da: u8,
}

impl ccEventWonder {
    pub const CLASS_NAME: &'static str = "ccEventWonder";
    pub const VTABLE_RVA: u32 = 0x008b5300;
    pub const METHOD_COUNT: usize = 4;
    pub const BASE_CLASSES: &'static [&'static str] = &["ccEventBase"];
    pub const VIRTUAL_METHODS: &'static [u64] = &[0x0000000180387140, 0x0000000180380cf0, 0x00000001800d3090, 0x0000000180394f30];

    pub fn new() -> Self {
        Self {
            vtable_rva: Self::VTABLE_RVA,
            field_0x0000: Default::default(),
            field_0x0008: Default::default(),
            field_0x000a: Default::default(),
            field_0x04da: Default::default(),
        }
    }

    /// Decompiled virtual method #0 @ RVA 0x00387140 (18 instructions, 3 basic blocks)
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

    /// Decompiled virtual method #3 @ RVA 0x00394f30 (55 instructions, 21 basic blocks)
    pub fn method_03(&mut self) {
        // Writes to field at offset +0x04da
    }

}

impl CcClass for ccEventWonder {
    fn class_name(&self) -> &'static str { Self::CLASS_NAME }
    fn vtable_rva(&self) -> u32 { Self::VTABLE_RVA }
    fn virtual_method_count(&self) -> usize { Self::METHOD_COUNT }
    fn base_classes(&self) -> &'static [&'static str] { Self::BASE_CLASSES }
}

/// Decompiled CC2 class: `ccEventWorldeff` (VTable RVA: 0x008b5710, 4 virtual methods, 3 recovered fields)
/// Inherits: ccEventBase
#[derive(Debug, Clone, Default)]
pub struct ccEventWorldeff {
    pub vtable_rva: u32,
    pub field_0x0000: u64,
    pub field_0x0008: u16,
    pub field_0x000a: u8,
}

impl ccEventWorldeff {
    pub const CLASS_NAME: &'static str = "ccEventWorldeff";
    pub const VTABLE_RVA: u32 = 0x008b5710;
    pub const METHOD_COUNT: usize = 4;
    pub const BASE_CLASSES: &'static [&'static str] = &["ccEventBase"];
    pub const VIRTUAL_METHODS: &'static [u64] = &[0x0000000180386830, 0x0000000180380cf0, 0x00000001800d3090, 0x00000001803919d0];

    pub fn new() -> Self {
        Self {
            vtable_rva: Self::VTABLE_RVA,
            field_0x0000: Default::default(),
            field_0x0008: Default::default(),
            field_0x000a: Default::default(),
        }
    }

    /// Decompiled virtual method #0 @ RVA 0x00386830 (18 instructions, 3 basic blocks)
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

    /// Decompiled virtual method #3 @ RVA 0x003919d0 (26 instructions, 6 basic blocks)
    pub fn method_03(&mut self) {
    }

}

impl CcClass for ccEventWorldeff {
    fn class_name(&self) -> &'static str { Self::CLASS_NAME }
    fn vtable_rva(&self) -> u32 { Self::VTABLE_RVA }
    fn virtual_method_count(&self) -> usize { Self::METHOD_COUNT }
    fn base_classes(&self) -> &'static [&'static str] { Self::BASE_CLASSES }
}

/// Decompiled CC2 class: `ccEventWorldman` (VTable RVA: 0x008b5490, 4 virtual methods, 5 recovered fields)
/// Inherits: ccEventBase
#[derive(Debug, Clone, Default)]
pub struct ccEventWorldman {
    pub vtable_rva: u32,
    pub field_0x0000: u64,
    pub field_0x0008: u16,
    pub field_0x000a: u8,
    pub field_0x000c: u32,
    pub field_0x0010: u16,
}

impl ccEventWorldman {
    pub const CLASS_NAME: &'static str = "ccEventWorldman";
    pub const VTABLE_RVA: u32 = 0x008b5490;
    pub const METHOD_COUNT: usize = 4;
    pub const BASE_CLASSES: &'static [&'static str] = &["ccEventBase"];
    pub const VIRTUAL_METHODS: &'static [u64] = &[0x0000000180386d40, 0x0000000180380cf0, 0x00000001800d3090, 0x0000000180394c00];

    pub fn new() -> Self {
        Self {
            vtable_rva: Self::VTABLE_RVA,
            field_0x0000: Default::default(),
            field_0x0008: Default::default(),
            field_0x000a: Default::default(),
            field_0x000c: Default::default(),
            field_0x0010: Default::default(),
        }
    }

    /// Decompiled virtual method #0 @ RVA 0x00386d40 (18 instructions, 3 basic blocks)
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

    /// Decompiled virtual method #3 @ RVA 0x00394c00 (69 instructions, 21 basic blocks)
    pub fn method_03(&mut self) {
        // Writes to field at offset +0x000c
        // Writes to field at offset +0x0010
    }

}

impl CcClass for ccEventWorldman {
    fn class_name(&self) -> &'static str { Self::CLASS_NAME }
    fn vtable_rva(&self) -> u32 { Self::VTABLE_RVA }
    fn virtual_method_count(&self) -> usize { Self::METHOD_COUNT }
    fn base_classes(&self) -> &'static [&'static str] { Self::BASE_CLASSES }
}


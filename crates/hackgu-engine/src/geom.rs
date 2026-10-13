//! Subsystem: geom
//! Contains 9 decompiled classes.

use crate::CcClass;

/// Decompiled CC2 class: `ccAnm` (VTable RVA: 0x00866040, 1 virtual methods, 0 recovered fields)
/// Inherits: ccCoord
#[derive(Debug, Clone, Default)]
pub struct ccAnm {
    pub vtable_rva: u32,
}

impl ccAnm {
    pub const CLASS_NAME: &'static str = "ccAnm";
    pub const VTABLE_RVA: u32 = 0x00866040;
    pub const METHOD_COUNT: usize = 1;
    pub const BASE_CLASSES: &'static [&'static str] = &["ccCoord"];
    pub const VIRTUAL_METHODS: &'static [u64] = &[0x00000001802b7850];

    pub fn new() -> Self {
        Self {
            vtable_rva: Self::VTABLE_RVA,
        }
    }

    /// Decompiled virtual method #0 @ RVA 0x002b7850 (37 instructions, 8 basic blocks)
    pub fn method_00(&mut self) {
    }

}

impl CcClass for ccAnm {
    fn class_name(&self) -> &'static str { Self::CLASS_NAME }
    fn vtable_rva(&self) -> u32 { Self::VTABLE_RVA }
    fn virtual_method_count(&self) -> usize { Self::METHOD_COUNT }
    fn base_classes(&self) -> &'static [&'static str] { Self::BASE_CLASSES }
}

/// Decompiled CC2 class: `ccAnmCtrlInterpBase` (VTable RVA: 0x00876fd8, 3 virtual methods, 1 recovered fields)
#[derive(Debug, Clone, Default)]
pub struct ccAnmCtrlInterpBase {
    pub vtable_rva: u32,
    pub field_0x0000: u64,
}

impl ccAnmCtrlInterpBase {
    pub const CLASS_NAME: &'static str = "ccAnmCtrlInterpBase";
    pub const VTABLE_RVA: u32 = 0x00876fd8;
    pub const METHOD_COUNT: usize = 3;
    pub const BASE_CLASSES: &'static [&'static str] = &[];
    pub const VIRTUAL_METHODS: &'static [u64] = &[0x0000000180158af0, 0x000000018073e2c0, 0x000000018073e2c0];

    pub fn new() -> Self {
        Self {
            vtable_rva: Self::VTABLE_RVA,
            field_0x0000: Default::default(),
        }
    }

    /// Decompiled virtual method #0 @ RVA 0x00158af0 (13 instructions, 3 basic blocks)
    pub fn method_00(&mut self) {
        // Writes to field at offset +0x0000
    }

    /// Decompiled virtual method #1 @ RVA 0x0073e2c0 (337 instructions, 1 basic blocks)
    pub fn method_01(&mut self) {
    }

    /// Decompiled virtual method #2 @ RVA 0x0073e2c0 (337 instructions, 1 basic blocks)
    pub fn method_02(&mut self) {
    }

}

impl CcClass for ccAnmCtrlInterpBase {
    fn class_name(&self) -> &'static str { Self::CLASS_NAME }
    fn vtable_rva(&self) -> u32 { Self::VTABLE_RVA }
    fn virtual_method_count(&self) -> usize { Self::METHOD_COUNT }
    fn base_classes(&self) -> &'static [&'static str] { Self::BASE_CLASSES }
}

/// Decompiled CC2 class: `ccAnmCtrlInterpObj` (VTable RVA: 0x00876fb8, 3 virtual methods, 10 recovered fields)
/// Inherits: ccAnmCtrlInterpBase
#[derive(Debug, Clone, Default)]
pub struct ccAnmCtrlInterpObj {
    pub vtable_rva: u32,
    pub field_0x0000: u64,
    pub field_0x0008: u64,
    pub field_0x0010: u32,
    pub field_0x0014: u32,
    pub field_0x0018: u32,
    pub field_0x001c: u32,
    pub field_0x0020: f32,
    pub field_0x0024: f32,
    pub field_0x0028: f32,
    pub field_0x002c: u32,
}

impl ccAnmCtrlInterpObj {
    pub const CLASS_NAME: &'static str = "ccAnmCtrlInterpObj";
    pub const VTABLE_RVA: u32 = 0x00876fb8;
    pub const METHOD_COUNT: usize = 3;
    pub const BASE_CLASSES: &'static [&'static str] = &["ccAnmCtrlInterpBase"];
    pub const VIRTUAL_METHODS: &'static [u64] = &[0x000000018015a730, 0x0000000180120f90, 0x00000001801210d0];

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
            field_0x0024: Default::default(),
            field_0x0028: Default::default(),
            field_0x002c: Default::default(),
        }
    }

    /// Decompiled virtual method #0 @ RVA 0x0015a730 (13 instructions, 3 basic blocks)
    pub fn method_00(&mut self) {
        // Writes to field at offset +0x0000
    }

    /// Decompiled virtual method #1 @ RVA 0x00120f90 (56 instructions, 1 basic blocks)
    pub fn method_01(&mut self) {
        // Writes to field at offset +0x0008
        // Writes to field at offset +0x0010
        // Writes to field at offset +0x0014
        // Writes to field at offset +0x0018
        // Writes to field at offset +0x0020
        // Writes to field at offset +0x0024
        // Writes to field at offset +0x0028
        // Writes to field at offset +0x002c
    }

    /// Decompiled virtual method #2 @ RVA 0x001210d0 (53 instructions, 1 basic blocks)
    pub fn method_02(&mut self) {
    }

}

impl CcClass for ccAnmCtrlInterpObj {
    fn class_name(&self) -> &'static str { Self::CLASS_NAME }
    fn vtable_rva(&self) -> u32 { Self::VTABLE_RVA }
    fn virtual_method_count(&self) -> usize { Self::METHOD_COUNT }
    fn base_classes(&self) -> &'static [&'static str] { Self::BASE_CLASSES }
}

/// Decompiled CC2 class: `ccClump` (VTable RVA: 0x00866070, 1 virtual methods, 0 recovered fields)
/// Inherits: ccCoord
#[derive(Debug, Clone, Default)]
pub struct ccClump {
    pub vtable_rva: u32,
}

impl ccClump {
    pub const CLASS_NAME: &'static str = "ccClump";
    pub const VTABLE_RVA: u32 = 0x00866070;
    pub const METHOD_COUNT: usize = 1;
    pub const BASE_CLASSES: &'static [&'static str] = &["ccCoord"];
    pub const VIRTUAL_METHODS: &'static [u64] = &[0x00000001800eb660];

    pub fn new() -> Self {
        Self {
            vtable_rva: Self::VTABLE_RVA,
        }
    }

    /// Decompiled virtual method #0 @ RVA 0x000eb660 (16 instructions, 3 basic blocks)
    pub fn method_00(&mut self) {
    }

}

impl CcClass for ccClump {
    fn class_name(&self) -> &'static str { Self::CLASS_NAME }
    fn vtable_rva(&self) -> u32 { Self::VTABLE_RVA }
    fn virtual_method_count(&self) -> usize { Self::METHOD_COUNT }
    fn base_classes(&self) -> &'static [&'static str] { Self::BASE_CLASSES }
}

/// Decompiled CC2 class: `ccCoord` (VTable RVA: 0x00866060, 1 virtual methods, 1 recovered fields)
#[derive(Debug, Clone, Default)]
pub struct ccCoord {
    pub vtable_rva: u32,
    pub field_0x0000: u64,
}

impl ccCoord {
    pub const CLASS_NAME: &'static str = "ccCoord";
    pub const VTABLE_RVA: u32 = 0x00866060;
    pub const METHOD_COUNT: usize = 1;
    pub const BASE_CLASSES: &'static [&'static str] = &[];
    pub const VIRTUAL_METHODS: &'static [u64] = &[0x00000001800eb8b0];

    pub fn new() -> Self {
        Self {
            vtable_rva: Self::VTABLE_RVA,
            field_0x0000: Default::default(),
        }
    }

    /// Decompiled virtual method #0 @ RVA 0x000eb8b0 (13 instructions, 3 basic blocks)
    pub fn method_00(&mut self) {
        // Writes to field at offset +0x0000
    }

}

impl CcClass for ccCoord {
    fn class_name(&self) -> &'static str { Self::CLASS_NAME }
    fn vtable_rva(&self) -> u32 { Self::VTABLE_RVA }
    fn virtual_method_count(&self) -> usize { Self::METHOD_COUNT }
    fn base_classes(&self) -> &'static [&'static str] { Self::BASE_CLASSES }
}

/// Decompiled CC2 class: `ccGeom` (VTable RVA: 0x00876a30, 2 virtual methods, 1 recovered fields)
#[derive(Debug, Clone, Default)]
pub struct ccGeom {
    pub vtable_rva: u32,
    pub field_0x0000: u64,
}

impl ccGeom {
    pub const CLASS_NAME: &'static str = "ccGeom";
    pub const VTABLE_RVA: u32 = 0x00876a30;
    pub const METHOD_COUNT: usize = 2;
    pub const BASE_CLASSES: &'static [&'static str] = &[];
    pub const VIRTUAL_METHODS: &'static [u64] = &[0x00000001800d3090, 0x0000000180145620];

    pub fn new() -> Self {
        Self {
            vtable_rva: Self::VTABLE_RVA,
            field_0x0000: Default::default(),
        }
    }

    /// Decompiled virtual method #0 @ RVA 0x000d3090 (1 instructions, 1 basic blocks)
    pub fn method_00(&mut self) {
    }

    /// Decompiled virtual method #1 @ RVA 0x00145620 (13 instructions, 3 basic blocks)
    pub fn method_01(&mut self) {
        // Writes to field at offset +0x0000
    }

}

impl CcClass for ccGeom {
    fn class_name(&self) -> &'static str { Self::CLASS_NAME }
    fn vtable_rva(&self) -> u32 { Self::VTABLE_RVA }
    fn virtual_method_count(&self) -> usize { Self::METHOD_COUNT }
    fn base_classes(&self) -> &'static [&'static str] { Self::BASE_CLASSES }
}

/// Decompiled CC2 class: `ccGeomBone02` (VTable RVA: 0x00876a48, 2 virtual methods, 1 recovered fields)
/// Inherits: ccGeom
#[derive(Debug, Clone, Default)]
pub struct ccGeomBone02 {
    pub vtable_rva: u32,
    pub field_0x0000: u64,
}

impl ccGeomBone02 {
    pub const CLASS_NAME: &'static str = "ccGeomBone02";
    pub const VTABLE_RVA: u32 = 0x00876a48;
    pub const METHOD_COUNT: usize = 2;
    pub const BASE_CLASSES: &'static [&'static str] = &["ccGeom"];
    pub const VIRTUAL_METHODS: &'static [u64] = &[0x00000001800d3090, 0x0000000180145620];

    pub fn new() -> Self {
        Self {
            vtable_rva: Self::VTABLE_RVA,
            field_0x0000: Default::default(),
        }
    }

    /// Decompiled virtual method #0 @ RVA 0x000d3090 (1 instructions, 1 basic blocks)
    pub fn method_00(&mut self) {
    }

    /// Decompiled virtual method #1 @ RVA 0x00145620 (13 instructions, 3 basic blocks)
    pub fn method_01(&mut self) {
        // Writes to field at offset +0x0000
    }

}

impl CcClass for ccGeomBone02 {
    fn class_name(&self) -> &'static str { Self::CLASS_NAME }
    fn vtable_rva(&self) -> u32 { Self::VTABLE_RVA }
    fn virtual_method_count(&self) -> usize { Self::METHOD_COUNT }
    fn base_classes(&self) -> &'static [&'static str] { Self::BASE_CLASSES }
}

/// Decompiled CC2 class: `ccGeomChunk` (VTable RVA: 0x00876a10, 3 virtual methods, 1 recovered fields)
#[derive(Debug, Clone, Default)]
pub struct ccGeomChunk {
    pub vtable_rva: u32,
    pub field_0x0000: u64,
}

impl ccGeomChunk {
    pub const CLASS_NAME: &'static str = "ccGeomChunk";
    pub const VTABLE_RVA: u32 = 0x00876a10;
    pub const METHOD_COUNT: usize = 3;
    pub const BASE_CLASSES: &'static [&'static str] = &[];
    pub const VIRTUAL_METHODS: &'static [u64] = &[0x00000001801455b0, 0x0000000180103e70, 0x0000000180103e70];

    pub fn new() -> Self {
        Self {
            vtable_rva: Self::VTABLE_RVA,
            field_0x0000: Default::default(),
        }
    }

    /// Decompiled virtual method #0 @ RVA 0x001455b0 (13 instructions, 3 basic blocks)
    pub fn method_00(&mut self) {
        // Writes to field at offset +0x0000
    }

    /// Decompiled virtual method #1 @ RVA 0x00103e70 (2 instructions, 1 basic blocks)
    pub fn method_01(&mut self) {
    }

    /// Decompiled virtual method #2 @ RVA 0x00103e70 (2 instructions, 1 basic blocks)
    pub fn method_02(&mut self) {
    }

}

impl CcClass for ccGeomChunk {
    fn class_name(&self) -> &'static str { Self::CLASS_NAME }
    fn vtable_rva(&self) -> u32 { Self::VTABLE_RVA }
    fn virtual_method_count(&self) -> usize { Self::METHOD_COUNT }
    fn base_classes(&self) -> &'static [&'static str] { Self::BASE_CLASSES }
}

/// Decompiled CC2 class: `ccGeomChunkBone02` (VTable RVA: 0x00876a60, 3 virtual methods, 0 recovered fields)
/// Inherits: ccGeomChunk
#[derive(Debug, Clone, Default)]
pub struct ccGeomChunkBone02 {
    pub vtable_rva: u32,
}

impl ccGeomChunkBone02 {
    pub const CLASS_NAME: &'static str = "ccGeomChunkBone02";
    pub const VTABLE_RVA: u32 = 0x00876a60;
    pub const METHOD_COUNT: usize = 3;
    pub const BASE_CLASSES: &'static [&'static str] = &["ccGeomChunk"];
    pub const VIRTUAL_METHODS: &'static [u64] = &[0x00000001801455e0, 0x0000000180103e70, 0x0000000180145650];

    pub fn new() -> Self {
        Self {
            vtable_rva: Self::VTABLE_RVA,
        }
    }

    /// Decompiled virtual method #0 @ RVA 0x001455e0 (16 instructions, 3 basic blocks)
    pub fn method_00(&mut self) {
    }

    /// Decompiled virtual method #1 @ RVA 0x00103e70 (2 instructions, 1 basic blocks)
    pub fn method_01(&mut self) {
    }

    /// Decompiled virtual method #2 @ RVA 0x00145650 (13 instructions, 2 basic blocks)
    pub fn method_02(&mut self) {
    }

}

impl CcClass for ccGeomChunkBone02 {
    fn class_name(&self) -> &'static str { Self::CLASS_NAME }
    fn vtable_rva(&self) -> u32 { Self::VTABLE_RVA }
    fn virtual_method_count(&self) -> usize { Self::METHOD_COUNT }
    fn base_classes(&self) -> &'static [&'static str] { Self::BASE_CLASSES }
}


//! CyberConnect2 CCS Section Types and Object Definitions.

pub const SECTION_HEADER: u32 = 0x0001;
pub const SECTION_INDEX: u32 = 0x0002;
pub const SECTION_SETUP: u32 = 0x0003;
pub const SECTION_STREAM: u32 = 0x0005;

pub const SECTION_OBJECT: u32 = 0x0100;
pub const SECTION_MATERIAL: u32 = 0x0200;
pub const SECTION_TEXTURE: u32 = 0x0300;
pub const SECTION_CLUT: u32 = 0x0400;
pub const SECTION_CAMERA: u32 = 0x0500;
pub const SECTION_LIGHT: u32 = 0x0600;
pub const SECTION_ANIME: u32 = 0x0700;
pub const SECTION_MODEL: u32 = 0x0800;
pub const SECTION_CLUMP: u32 = 0x0900;
pub const SECTION_EXTERNAL: u32 = 0x0a00;
pub const SECTION_HITMESH: u32 = 0x0b00;
pub const SECTION_BBOX: u32 = 0x0c00;
pub const SECTION_PARTICLE: u32 = 0x0d00;
pub const SECTION_EFFECT: u32 = 0x0e00;
pub const SECTION_BLTGROUP: u32 = 0x1000;
pub const SECTION_FBPAGE: u32 = 0x1100;
pub const SECTION_FBRECT: u32 = 0x1200;
pub const SECTION_DUMMYPOS: u32 = 0x1300;
pub const SECTION_DUMMYPOSROT: u32 = 0x1400;
pub const SECTION_LAYER: u32 = 0x1700;
pub const SECTION_SHADOW: u32 = 0x1800;
pub const SECTION_MORPHER: u32 = 0x1900;
pub const SECTION_OBJECT2: u32 = 0x2000;
pub const SECTION_PCM: u32 = 0x2200;
pub const SECTION_BINARYBLOB: u32 = 0x2400;

pub fn section_name(sec_type: u32) -> &'static str {
    match sec_type {
        SECTION_HEADER => "Header",
        SECTION_INDEX => "Index",
        SECTION_SETUP => "Setup",
        SECTION_STREAM => "Stream",
        SECTION_OBJECT => "Object",
        SECTION_MATERIAL => "Material",
        SECTION_TEXTURE => "Texture",
        SECTION_CLUT => "CLUT",
        SECTION_CAMERA => "Camera",
        SECTION_LIGHT => "Light",
        SECTION_ANIME => "Animation",
        SECTION_MODEL => "Model",
        SECTION_CLUMP => "Clump",
        SECTION_EXTERNAL => "External",
        SECTION_HITMESH => "HitMesh",
        SECTION_BBOX => "BoundingBox",
        SECTION_PARTICLE => "Particle",
        SECTION_EFFECT => "Effect",
        SECTION_BLTGROUP => "BlitGroup",
        SECTION_FBPAGE => "FBPage",
        SECTION_FBRECT => "FBRect",
        SECTION_DUMMYPOS => "DummyPos",
        SECTION_DUMMYPOSROT => "DummyPosRot",
        SECTION_LAYER => "Layer",
        SECTION_SHADOW => "Shadow",
        SECTION_MORPHER => "Morpher",
        SECTION_OBJECT2 => "Object2",
        SECTION_PCM => "PCM Audio",
        SECTION_BINARYBLOB => "BinaryBlob",
        _ => "Unknown",
    }
}

/// A parsed sub-object from the setup section of a CCS container.
#[derive(Clone, Debug)]
pub struct CcsRawObject {
    pub object_id: u32,
    pub section_type: u32,
    pub offset: usize,
    pub size: usize,
    pub payload: Vec<u8>,
}

/// A parsed scene graph node / object definition (0x0100).
#[derive(Clone, Debug)]
pub struct CcsObjectNode {
    pub object_id: u32,
    pub parent_obj_id: u32,
    pub model_obj_id: u32,
    pub shadow_obj_id: u32,
    pub flags: u32,
}

impl CcsObjectNode {
    pub fn from_payload(object_id: u32, payload: &[u8]) -> Option<Self> {
        if payload.len() < 16 {
            return None;
        }
        let parent_obj_id = u32::from_le_bytes(payload[0..4].try_into().ok()?);
        let model_obj_id = u32::from_le_bytes(payload[4..8].try_into().ok()?);
        let shadow_obj_id = u32::from_le_bytes(payload[8..12].try_into().ok()?);
        let flags = u32::from_le_bytes(payload[12..16].try_into().ok()?);
        Some(Self {
            object_id,
            parent_obj_id,
            model_obj_id,
            shadow_obj_id,
            flags,
        })
    }
}


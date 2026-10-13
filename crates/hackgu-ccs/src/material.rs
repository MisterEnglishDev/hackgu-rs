//! CyberConnect2 CCS Material parser (0x0200).

/// A parsed CC2 Material object linking submeshes to textures.
#[derive(Clone, Debug)]
pub struct CcsMaterial {
    pub object_id: u32,
    pub texture_obj_id: u32,
    pub float_param: f32,
}

impl CcsMaterial {
    pub fn from_payload(object_id: u32, payload: &[u8]) -> Option<Self> {
        if payload.len() < 8 {
            return None;
        }
        let texture_obj_id = u32::from_le_bytes(payload[0..4].try_into().ok()?);
        let float_param = f32::from_le_bytes(payload[4..8].try_into().ok()?);
        Some(Self {
            object_id,
            texture_obj_id,
            float_param,
        })
    }
}

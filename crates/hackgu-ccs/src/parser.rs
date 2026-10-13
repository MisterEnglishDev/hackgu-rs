//! CyberConnect2 CCS File parser.

use std::fmt;

use crate::header::{CcsHeader, HeaderError};
use crate::index::{IndexError, IndexFileEntry, IndexObjectEntry, IndexSection};
use crate::objects::{CcsRawObject, SECTION_SETUP, SECTION_STREAM, SECTION_TEXTURE};

#[derive(Debug)]
pub enum CcsError {
    Header(HeaderError),
    Index(IndexError),
    TooShort,
    InvalidSetupSection(u32),
    CorruptedObjectHeader(String),
}

impl fmt::Display for CcsError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            CcsError::Header(e) => write!(f, "Header error: {}", e),
            CcsError::Index(e) => write!(f, "Index error: {}", e),
            CcsError::TooShort => write!(f, "Unexpected end of CCS data"),
            CcsError::InvalidSetupSection(t) => write!(f, "Expected setup section (0x0003), found 0x{:X}", t),
            CcsError::CorruptedObjectHeader(s) => write!(f, "Corrupted object header: {}", s),
        }
    }
}

impl std::error::Error for CcsError {}

impl From<HeaderError> for CcsError {
    fn from(e: HeaderError) -> Self {
        CcsError::Header(e)
    }
}

impl From<IndexError> for CcsError {
    fn from(e: IndexError) -> Self {
        CcsError::Index(e)
    }
}

#[derive(Clone, Debug)]
pub struct CcsFile {
    pub header: CcsHeader,
    pub files: Vec<IndexFileEntry>,
    pub objects: Vec<IndexObjectEntry>,
    pub raw_objects: Vec<CcsRawObject>,
}

impl CcsFile {
    /// Parse a complete uncompressed CCS/CCSF byte buffer.
    pub fn parse(data: &[u8]) -> Result<Self, CcsError> {
        let (header, mut pos) = CcsHeader::parse(data)?;
        let (index, next_pos) = IndexSection::parse(data, pos)?;
        pos = next_pos;

        if pos + 8 > data.len() {
            return Err(CcsError::TooShort);
        }

        let setup_type = u32::from_le_bytes(data[pos..pos + 4].try_into().unwrap()) & 0xffff;
        if setup_type != SECTION_SETUP {
            return Err(CcsError::InvalidSetupSection(setup_type));
        }

        pos += 8;

        let mut raw_objects = Vec::new();

        while pos + 12 <= data.len() {
            let section_type = u32::from_le_bytes(data[pos..pos + 4].try_into().unwrap()) & 0xffff;
            if section_type == 0 || (section_type == SECTION_STREAM && u32::from_le_bytes(data[pos + 4..pos + 8].try_into().unwrap()) == 1) {
                break;
            }

            let size_dwords = u32::from_le_bytes(data[pos + 4..pos + 8].try_into().unwrap());
            let object_id = u32::from_le_bytes(data[pos + 8..pos + 12].try_into().unwrap());

            if object_id as usize >= index.objects.len() {
                break;
            }

            let payload_start = pos + 12;

            let rd32 = |o: usize| u32::from_le_bytes(data[o..o + 4].try_into().unwrap());
            let nobj = index.objects.len();
            // A plausible section header: type is a multiple of 0x100 in the known range (or the stream marker),
            // the object id exists, and the declared size fits inside the file.
            let plausible = |o: usize| -> bool {
                if o == data.len() {
                    return true;
                }
                if o + 12 > data.len() {
                    return false;
                }
                let ty = rd32(o) & 0xffff;
                if ty == SECTION_STREAM {
                    return rd32(o + 4) == 1;
                }
                let sz = rd32(o + 4) as usize;
                let fits = sz >= 1 && o + 12 + (sz - 1) * 4 <= data.len() + 4;
                // Legacy textures can declare a section size larger than the data; their pixel-block length
                // (dwords, at payload +20) is what actually fits (e.g. menu/xgutitle_01.ccs).
                let tex_fits = ty == SECTION_TEXTURE && o + 12 + 24 <= data.len() && o + 12 + 24 + rd32(o + 12 + 20) as usize * 4 <= data.len();
                ty >= 0x100 && ty <= 0x3000 && ty % 0x100 == 0 && (rd32(o + 8) as usize) < nobj && (fits || tex_fits)
            };
            // Candidate payload lengths, most trusted first.
            let mut candidates: Vec<usize> = Vec::new();
            if section_type == SECTION_TEXTURE && payload_start + 28 <= data.len() && &data[payload_start + 24..payload_start + 28] == b"btx\0" {
                candidates.push(24 + rd32(payload_start + 20) as usize);
            }
            if size_dwords > 0 {
                candidates.push((size_dwords - 1) as usize * 4);
            }
            if section_type == SECTION_TEXTURE && payload_start + 24 <= data.len() {
                // Legacy textures: the pixel block length (dwords) lives at +20; the declared section size is
                // sometimes too small (4-bit CLUT images) and sometimes too large (some 8-bit ones).
                candidates.push(24 + rd32(payload_start + 20) as usize * 4);
            }
            let mut payload_size = None;
            for &c in &candidates {
                if plausible(payload_start + c) {
                    payload_size = Some(c);
                    break;
                }
            }
            if payload_size.is_none() {
                // The declared size is wrong (observed on models and textures). Re-synchronise on the next
                // header that is itself followed by another plausible header (or the end of the file).
                // Score a candidate by how many consecutive plausible headers follow it (chained through their
                // declared sizes); a header-shaped pattern inside model data rarely chains more than once.
                let chain = |start: usize| -> u32 {
                    let mut o = start;
                    let mut depth = 0;
                    while depth < 4 {
                        if o == data.len() || (o + 8 <= data.len() && rd32(o) & 0xffff == SECTION_STREAM && rd32(o + 4) == 1) {
                            return 4;
                        }
                        if !plausible(o) {
                            return depth;
                        }
                        let sz = rd32(o + 4) as usize;
                        o += 12 + sz.saturating_sub(1) * 4;
                        depth += 1;
                    }
                    depth
                };
                let mut o = payload_start + 4;
                let mut best: Option<(u32, usize)> = None;
                while o + 12 <= data.len() {
                    if plausible(o) {
                        let sc = chain(o);
                        if sc >= 4 {
                            best = Some((sc, o));
                            break;
                        }
                    }
                    o += 4;
                }
                let found = best.map(|(_, o)| o - payload_start);
                payload_size = found;
            }
            // If nothing chains cleanly, keep the declared size (historic behaviour) instead of guessing.
            let payload_size = match payload_size {
                Some(sz) => sz,
                None => candidates.first().copied().unwrap_or(0),
            };
            let payload_end = (payload_start + payload_size).min(data.len());

            raw_objects.push(CcsRawObject {
                object_id,
                section_type,
                offset: pos,
                size: payload_size,
                payload: data[payload_start..payload_end].to_vec(),
            });

            pos += 12 + payload_size;
        }

        Ok(CcsFile {
            header,
            files: index.files,
            objects: index.objects,
            raw_objects,
        })
    }

    /// Retrieve the name of an object by its ID from the object index.
    pub fn object_name(&self, object_id: u32) -> Option<&str> {
        self.objects.get(object_id as usize).map(|o| o.name.as_str())
    }

    /// Retrieve the subfile name of an object by its ID from the file index.
    pub fn object_file_name(&self, object_id: u32) -> Option<&str> {
        let obj = self.objects.get(object_id as usize)?;
        self.files.get(obj.file_id as usize).map(|f| f.name.as_str())
    }

    /// Filter objects by section type.
    pub fn objects_of_type(&self, sec_type: u32) -> Vec<&CcsRawObject> {
        self.raw_objects.iter().filter(|o| o.section_type == sec_type).collect()
    }

    /// Parse and return all scene graph Object nodes (0x0100).
    pub fn scene_nodes(&self) -> std::collections::HashMap<u32, crate::objects::CcsObjectNode> {
        let mut map = std::collections::HashMap::new();
        for obj in self.objects_of_type(crate::objects::SECTION_OBJECT) {
            if let Some(node) = crate::objects::CcsObjectNode::from_payload(obj.object_id, &obj.payload) {
                map.insert(obj.object_id, node);
            }
        }
        map
    }

    /// Parse and return all Clump skeleton objects (0x0900).
    pub fn clumps(&self) -> Vec<crate::clump::CcsClump> {
        let mut list = Vec::new();
        for obj in self.objects_of_type(crate::objects::SECTION_CLUMP) {
            if let Ok(clump) = crate::clump::CcsClump::from_payload(obj.object_id, &obj.payload) {
                list.push(clump);
            }
        }
        list
    }

    /// For every model referenced by an Object node (0x0100) that is a bone of some clump, return
    /// `model_obj_id -> (clump index in clumps(), node object id)`. Rigid models are drawn with that node's
    /// world matrix; this is the structural link (the old `CMP_<suffix>` name match missed most stage parts).
    pub fn model_owner_nodes(&self) -> std::collections::HashMap<u32, (usize, u32)> {
        let nodes = self.scene_nodes();
        let mut out = std::collections::HashMap::new();
        for (ci, cl) in self.clumps().iter().enumerate() {
            for &oid in &cl.bone_obj_ids {
                if let Some(n) = nodes.get(&oid) {
                    if n.model_obj_id != 0 {
                        out.entry(n.model_obj_id).or_insert((ci, oid));
                    }
                }
            }
        }
        out
    }

    /// Parse and return all Material objects (0x0200).
    pub fn materials(&self) -> std::collections::HashMap<u32, crate::material::CcsMaterial> {
        let mut map = std::collections::HashMap::new();
        for obj in self.objects_of_type(crate::objects::SECTION_MATERIAL) {
            if let Some(mat) = crate::material::CcsMaterial::from_payload(obj.object_id, &obj.payload) {
                map.insert(obj.object_id, mat);
            }
        }
        map
    }

    /// Parse and return all HitMesh (0x0B00) collision objects.
    pub fn hit_meshes(&self) -> Vec<crate::hitmesh::HitMesh> {
        self.objects_of_type(crate::objects::SECTION_HITMESH)
            .into_iter()
            .filter_map(|o| crate::hitmesh::HitMesh::from_payload(o.object_id, &o.payload))
            .collect()
    }

    /// Positions (centimetres, as stored) of every DummyPos (0x1300) / DummyPosRot (0x1400) locator, keyed by name.
    /// DummyPos payload = 3 x f32 (x, y, z).
    pub fn locators(&self) -> Vec<(String, [f32; 3])> {
        let mut out = Vec::new();
        for o in &self.raw_objects {
            if (o.section_type == crate::objects::SECTION_DUMMYPOS || o.section_type == crate::objects::SECTION_DUMMYPOSROT) && o.payload.len() >= 12 {
                let f = |i: usize| f32::from_le_bytes(o.payload[i..i + 4].try_into().unwrap());
                out.push((self.object_name(o.object_id).unwrap_or("").to_string(), [f(0), f(4), f(8)]));
            }
        }
        out
    }

    /// Parse and return all Animation objects (0x0700).
    pub fn animations(&self) -> Vec<crate::anim::CcsAnimation> {
        let mut list = Vec::new();
        for obj in self.objects_of_type(crate::objects::SECTION_ANIME) {
            let name = self.object_name(obj.object_id).unwrap_or("animation");
            if let Ok(anim) = crate::anim::CcsAnimation::from_payload(obj.object_id, name, &obj.payload, self) {
                list.push(anim);
            }
        }
        list
    }
}

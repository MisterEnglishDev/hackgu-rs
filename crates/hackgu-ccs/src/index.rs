//! CyberConnect2 CCS Index Section parser.

use std::fmt;
use crate::objects::SECTION_INDEX;

#[derive(Clone, Debug)]
pub struct IndexFileEntry {
    pub id: usize,
    pub name: String,
}

#[derive(Clone, Debug)]
pub struct IndexObjectEntry {
    pub id: usize,
    pub file_id: u16,
    pub name: String,
}

#[derive(Clone, Debug)]
pub struct IndexSection {
    pub files: Vec<IndexFileEntry>,
    pub objects: Vec<IndexObjectEntry>,
}

#[derive(Debug)]
pub enum IndexError {
    TooShort,
    InvalidSectionType(u32),
    InvalidString(String),
}

impl fmt::Display for IndexError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            IndexError::TooShort => write!(f, "Data too short for CCS index section"),
            IndexError::InvalidSectionType(t) => write!(f, "Invalid index section type: 0x{:X}", t),
            IndexError::InvalidString(s) => write!(f, "Failed to decode index string: {}", s),
        }
    }
}

impl std::error::Error for IndexError {}

impl IndexSection {
    pub fn parse(data: &[u8], start: usize) -> Result<(Self, usize), IndexError> {
        if start + 16 > data.len() {
            return Err(IndexError::TooShort);
        }

        let sec_type = u32::from_le_bytes(data[start..start + 4].try_into().unwrap()) & 0xffff;
        if sec_type != SECTION_INDEX {
            return Err(IndexError::InvalidSectionType(sec_type));
        }

        let _sec_len = u32::from_le_bytes(data[start + 4..start + 8].try_into().unwrap());
        let file_count = u32::from_le_bytes(data[start + 8..start + 12].try_into().unwrap()) as usize;
        let obj_count = u32::from_le_bytes(data[start + 12..start + 16].try_into().unwrap()) as usize;

        let mut pos = start + 16;
        let needed = file_count * 32 + obj_count * 32;
        if pos + needed > data.len() {
            return Err(IndexError::TooShort);
        }

        let mut files = Vec::with_capacity(file_count);
        for id in 0..file_count {
            let entry_bytes = &data[pos..pos + 32];
            let end = entry_bytes.iter().position(|&b| b == 0).unwrap_or(32);
            let name = String::from_utf8_lossy(&entry_bytes[..end]).to_string();
            files.push(IndexFileEntry { id, name });
            pos += 32;
        }

        let mut objects = Vec::with_capacity(obj_count);
        for id in 0..obj_count {
            let name_bytes = &data[pos..pos + 30];
            let end = name_bytes.iter().position(|&b| b == 0).unwrap_or(30);
            let name = String::from_utf8_lossy(&name_bytes[..end]).to_string();
            let file_id = u16::from_le_bytes(data[pos + 30..pos + 32].try_into().unwrap());
            objects.push(IndexObjectEntry { id, file_id, name });
            pos += 32;
        }

        Ok((IndexSection { files, objects }, pos))
    }
}

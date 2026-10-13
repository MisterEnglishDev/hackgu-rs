//! CyberConnect2 CCS File Header parser.

use std::fmt;

pub const CCS_MAGIC: &[u8; 4] = b"CCSF";
pub const SECTION_HEADER: u32 = 0x0001;

pub const VERSION_GEN1: u32 = 0x0110;
pub const VERSION_GEN2: u32 = 0x0120;
pub const VERSION_GEN3: u32 = 0x0125;

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum CcsVersion {
    Gen1, // 0x0110 (IMOQ era)
    Gen2, // 0x0120 (.hack//G.U. PS2)
    Gen3, // 0x0125 (.hack//Link / Last Recode)
    Unknown(u32),
}

impl CcsVersion {
    pub fn from_raw(v: u32) -> Self {
        match v {
            VERSION_GEN1 => CcsVersion::Gen1,
            0x0120..=0x0124 => CcsVersion::Gen2,
            VERSION_GEN3 | 0x0130 => CcsVersion::Gen3,
            other => CcsVersion::Unknown(other),
        }
    }

    pub fn as_str(&self) -> &'static str {
        match self {
            CcsVersion::Gen1 => "Gen 1 (0x0110 - IMOQ)",
            CcsVersion::Gen2 => "Gen 2 (0x0120-0x0124 - G.U. PS2)",
            CcsVersion::Gen3 => "Gen 3 (0x0125/0x0130 - G.U. Last Recode / Stages)",
            CcsVersion::Unknown(_) => "Unknown Version",
        }
    }
}

/// Bit set in `CcsHeader::raw_version` for files whose models store V top-down (header bytes 2..4 = 0xCCCC).
pub const UV_TOP_DOWN: u32 = 0x8000_0000;

#[derive(Clone, Debug)]
pub struct CcsHeader {
    pub name: String,
    pub raw_version: u32,
    pub version: CcsVersion,
    pub unk1: u32,
    pub unk2: u32,
    pub unk3: u32,
}

#[derive(Debug)]
pub enum HeaderError {
    TooShort,
    InvalidSectionType(u32),
    InvalidMagic,
    InvalidString(String),
}

impl fmt::Display for HeaderError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            HeaderError::TooShort => write!(f, "Data too short for CCS header"),
            HeaderError::InvalidSectionType(t) => write!(f, "Invalid header section type: 0x{:X}", t),
            HeaderError::InvalidMagic => write!(f, "Invalid CCSF magic signature"),
            HeaderError::InvalidString(s) => write!(f, "Failed to decode CCSF name string: {}", s),
        }
    }
}

impl std::error::Error for HeaderError {}

impl CcsHeader {
    /// Read header section from cursor position. Returns parsed header and next byte offset (typically 60).
    pub fn parse(data: &[u8]) -> Result<(Self, usize), HeaderError> {
        if data.len() < 60 {
            return Err(HeaderError::TooShort);
        }

        let is_header = (u32::from_le_bytes(data[0..4].try_into().unwrap())) & 0xffff;
        if is_header != SECTION_HEADER {
            return Err(HeaderError::InvalidSectionType(is_header));
        }

        let _section_len = u32::from_le_bytes(data[4..8].try_into().unwrap());
        if &data[8..12] != CCS_MAGIC {
            return Err(HeaderError::InvalidMagic);
        }

        // Fixed 32-byte null-terminated string buffer at offset 12..44
        let name_bytes = &data[12..44];
        let name_end = name_bytes.iter().position(|&b| b == 0).unwrap_or(32);
        let name = String::from_utf8_lossy(&name_bytes[..name_end]).to_string();

        let raw_version = u32::from_le_bytes(data[44..48].try_into().unwrap());
        let unk1 = u32::from_le_bytes(data[48..52].try_into().unwrap());
        let unk2 = u32::from_le_bytes(data[52..56].try_into().unwrap());
        let unk3 = u32::from_le_bytes(data[56..60].try_into().unwrap());

        // Bytes 2..4 of the header section read 0xCCCC in a few files (town/xcg.ccs, town/xw.ccs) whose models store
        // V top-down; the model decoder is told through bit 31 of the version it receives (see DecodedModel).
        let flagged = if data[2..4] == [0xCC, 0xCC] { raw_version | UV_TOP_DOWN } else { raw_version };
        Ok((
            CcsHeader {
                name,
                raw_version: flagged,
                version: CcsVersion::from_raw(raw_version),
                unk1,
                unk2,
                unk3,
            },
            60,
        ))
    }
}

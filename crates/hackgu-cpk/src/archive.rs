//! High-level CRI CPK archive parser and file extractor.

use std::fs::{self, File};
use std::io::{self, Read, Seek, SeekFrom};
use std::path::{Path, PathBuf};
use std::fmt;

use crate::layla::{self, LaylaError};
use crate::utf::{self, UtfError, UtfTable};

pub const CPK_MAGIC: &[u8; 4] = b"CPK ";
pub const TOC_MAGIC: &[u8; 4] = b"TOC ";

#[derive(Debug)]
pub enum CpkError {
    Io(io::Error),
    Utf(UtfError),
    Layla(LaylaError),
    InvalidSignature(&'static str),
    MissingHeaderField(&'static str),
    FileNotFound(String),
}

impl fmt::Display for CpkError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            CpkError::Io(e) => write!(f, "I/O error: {}", e),
            CpkError::Utf(e) => write!(f, "UTF error: {}", e),
            CpkError::Layla(e) => write!(f, "CRILAYLA decompression error: {}", e),
            CpkError::InvalidSignature(sig) => write!(f, "Invalid archive signature, expected {}", sig),
            CpkError::MissingHeaderField(field) => write!(f, "Missing expected header field: {}", field),
            CpkError::FileNotFound(path) => write!(f, "File not found in CPK archive: {}", path),
        }
    }
}

impl std::error::Error for CpkError {}

impl From<io::Error> for CpkError {
    fn from(e: io::Error) -> Self {
        CpkError::Io(e)
    }
}

impl From<UtfError> for CpkError {
    fn from(e: UtfError) -> Self {
        CpkError::Utf(e)
    }
}

impl From<LaylaError> for CpkError {
    fn from(e: LaylaError) -> Self {
        CpkError::Layla(e)
    }
}

#[derive(Clone, Debug)]
pub struct CpkEntry {
    pub id: usize,
    pub dir_name: String,
    pub file_name: String,
    pub path: String,
    pub file_size: u64,
    pub extract_size: u64,
    pub file_offset: u64,
    pub is_compressed: bool,
}

#[derive(Clone, Debug)]
pub struct CpkArchive {
    pub toc_offset: u64,
    pub content_offset: u64,
    pub align: u64,
    pub entries: Vec<CpkEntry>,
}

impl CpkArchive {
    /// Open a CPK archive from a file path. Returns the parsed archive metadata and an open File handle.
    pub fn open<P: AsRef<Path>>(path: P) -> Result<(Self, File), CpkError> {
        let mut file = File::open(path)?;
        let archive = Self::read_from(&mut file)?;
        Ok((archive, file))
    }

    /// Read and parse a CPK archive from any reader implementing Read + Seek.
    pub fn read_from<R: Read + Seek>(reader: &mut R) -> Result<Self, CpkError> {
        // Read CPK signature
        reader.seek(SeekFrom::Start(0))?;
        let mut sig = [0u8; 4];
        reader.read_exact(&mut sig)?;
        if &sig != CPK_MAGIC {
            return Err(CpkError::InvalidSignature("CPK "));
        }

        // Offset 8 has header size (u64 LE)
        reader.seek(SeekFrom::Start(8))?;
        let mut size_buf = [0u8; 8];
        reader.read_exact(&mut size_buf)?;
        let header_table_size = u64::from_le_bytes(size_buf) as usize;

        // Read and decrypt CPK header @UTF table (starts at 0x10)
        reader.seek(SeekFrom::Start(0x10))?;
        let mut header_bytes = vec![0u8; header_table_size];
        reader.read_exact(&mut header_bytes)?;
        utf::decrypt_utf_bytes(&mut header_bytes);

        let header_table = UtfTable::parse(&header_bytes)?;
        let header_row = header_table.rows.first()
            .ok_or(CpkError::MissingHeaderField("CpkHeader row 0"))?;

        let toc_offset = header_row.get_u64("TocOffset")
            .ok_or(CpkError::MissingHeaderField("TocOffset"))?;
        let content_offset = header_row.get_u64("ContentOffset").unwrap_or(0);
        let align = header_row.get_u64("Align").unwrap_or(2048);

        // Read TOC table
        reader.seek(SeekFrom::Start(toc_offset))?;
        let mut toc_sig = [0u8; 4];
        reader.read_exact(&mut toc_sig)?;
        if &toc_sig != TOC_MAGIC {
            return Err(CpkError::InvalidSignature("TOC "));
        }

        reader.seek(SeekFrom::Start(toc_offset + 8))?;
        reader.read_exact(&mut size_buf)?;
        let toc_table_size = u64::from_le_bytes(size_buf) as usize;

        reader.seek(SeekFrom::Start(toc_offset + 0x10))?;
        let mut toc_bytes = vec![0u8; toc_table_size];
        reader.read_exact(&mut toc_bytes)?;
        utf::decrypt_utf_bytes(&mut toc_bytes);

        let toc_table = UtfTable::parse(&toc_bytes)?;

        let mut entries = Vec::with_capacity(toc_table.rows.len());

        for (id, row) in toc_table.rows.into_iter().enumerate() {
            let dir_name = row.get_str("DirName").unwrap_or("").to_string();
            let file_name = row.get_str("FileName").unwrap_or("").to_string();
            let file_size = row.get_u64("FileSize").unwrap_or(0);
            let extract_size = row.get_u64("ExtractSize").unwrap_or(file_size);
            let rel_offset = row.get_u64("FileOffset").unwrap_or(0);

            let path = if dir_name.is_empty() {
                file_name.clone()
            } else {
                format!("{}/{}", dir_name, file_name)
            };

            let is_compressed = extract_size > file_size;

            entries.push(CpkEntry {
                id,
                dir_name,
                file_name,
                path,
                file_size,
                extract_size,
                file_offset: rel_offset,
                is_compressed,
            });
        }

        Ok(Self {
            toc_offset,
            content_offset,
            align,
            entries,
        })
    }

    /// Absolute byte offset of a given entry in the CPK file.
    #[inline]
    pub fn entry_offset(&self, entry: &CpkEntry) -> u64 {
        self.toc_offset + entry.file_offset
    }

    /// Extract an entry's raw decompressed contents into memory.
    pub fn extract_entry<R: Read + Seek>(&self, reader: &mut R, entry: &CpkEntry) -> Result<Vec<u8>, CpkError> {
        let abs_offset = self.entry_offset(entry);
        reader.seek(SeekFrom::Start(abs_offset))?;

        let mut raw = vec![0u8; entry.file_size as usize];
        reader.read_exact(&mut raw)?;

        if entry.is_compressed || layla::is_crilayla(&raw) {
            let decompressed = layla::decompress(&raw)?;
            Ok(decompressed)
        } else {
            Ok(raw)
        }
    }

    /// Find an entry by case-insensitive path substring or exact match.
    pub fn find_entry(&self, needle: &str) -> Option<&CpkEntry> {
        let lower = needle.to_ascii_lowercase().replace('\\', "/");
        self.entries.iter().find(|e| {
            e.path.to_ascii_lowercase() == lower || e.file_name.to_ascii_lowercase() == lower
        })
    }

    /// Extract a file by name/path into memory.
    pub fn extract_by_path<R: Read + Seek>(&self, reader: &mut R, path: &str) -> Result<Vec<u8>, CpkError> {
        let entry = self.find_entry(path).ok_or_else(|| CpkError::FileNotFound(path.to_string()))?;
        self.extract_entry(reader, entry)
    }

    /// Extract all files from this archive to an output directory.
    pub fn extract_all<R: Read + Seek>(&self, reader: &mut R, out_dir: &Path) -> Result<usize, CpkError> {
        fs::create_dir_all(out_dir)?;

        let mut count = 0;
        for entry in &self.entries {
            let data = self.extract_entry(reader, entry)?;
            let file_rel = entry.path.replace('\\', "/");
            let mut target_path = PathBuf::from(out_dir);
            for part in file_rel.split('/') {
                if !part.is_empty() {
                    target_path.push(part);
                }
            }

            if let Some(parent) = target_path.parent() {
                fs::create_dir_all(parent)?;
            }

            fs::write(&target_path, data)?;
            count += 1;
        }

        Ok(count)
    }
}

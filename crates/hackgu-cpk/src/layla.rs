//! CRI CRILAYLA decompression implementation.
//!
//! CRILAYLA is an LZSS-based compression algorithm with a variable Fibonacci-like
//! run length encoding, operating backwards from the end of the uncompressed data.

use std::fmt;

pub const CRILAYLA_MAGIC: &[u8; 8] = b"CRILAYLA";
pub const UNCOMPRESSED_HEADER_SIZE: usize = 0x100;

#[derive(Debug)]
pub enum LaylaError {
    InvalidMagic,
    TooShort,
    InvalidHeaderOffset { offset: usize, file_len: usize },
    CorruptStream(String),
    UnexpectedEof,
}

impl fmt::Display for LaylaError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            LaylaError::InvalidMagic => write!(f, "Invalid CRILAYLA signature"),
            LaylaError::TooShort => write!(f, "Data too short for CRILAYLA stream"),
            LaylaError::InvalidHeaderOffset { offset, file_len } => {
                write!(f, "CRILAYLA uncompressed header offset {} exceeds file len {}", offset, file_len)
            }
            LaylaError::CorruptStream(msg) => write!(f, "Corrupted CRILAYLA bitstream: {}", msg),
            LaylaError::UnexpectedEof => write!(f, "Unexpected EOF while reading CRILAYLA bitstream"),
        }
    }
}

impl std::error::Error for LaylaError {}

struct BitReader<'a> {
    data: &'a [u8],
    pos: usize,
    bits_left: usize,
}

impl<'a> BitReader<'a> {
    fn new(data: &'a [u8], start_pos: usize) -> Self {
        Self {
            data,
            pos: start_pos,
            bits_left: 0,
        }
    }

    fn get_bit(&mut self) -> Result<u8, LaylaError> {
        if self.bits_left != 0 {
            let b = self.data[self.pos];
            self.bits_left -= 1;
            Ok((b >> self.bits_left) & 1)
        } else {
            if self.pos == 0 {
                return Err(LaylaError::UnexpectedEof);
            }
            self.pos -= 1;
            let b = self.data[self.pos];
            self.bits_left = 7;
            Ok((b >> 7) & 1)
        }
    }

    fn read_bits(&mut self, n: usize) -> Result<usize, LaylaError> {
        let mut res = 0usize;
        let mut bits_needed = n;
        while bits_needed > 0 {
            if self.bits_left == 0 {
                if self.pos == 0 {
                    return Err(LaylaError::UnexpectedEof);
                }
                self.pos -= 1;
                self.bits_left = 8;
            }
            let take = bits_needed.min(self.bits_left);
            let b = self.data[self.pos] as usize;
            let chunk = (b >> (self.bits_left - take)) & ((1 << take) - 1);
            res = (res << take) | chunk;
            self.bits_left -= take;
            bits_needed -= take;
        }
        Ok(res)
    }
}

/// Check if slice starts with CRILAYLA header.
pub fn is_crilayla(data: &[u8]) -> bool {
    data.len() >= 16 && &data[..8] == CRILAYLA_MAGIC
}

/// Decompress a CRILAYLA compressed stream.
pub fn decompress(compressed: &[u8]) -> Result<Vec<u8>, LaylaError> {
    if compressed.len() < 16 {
        return Err(LaylaError::TooShort);
    }
    if &compressed[..8] != CRILAYLA_MAGIC {
        return Err(LaylaError::InvalidMagic);
    }

    let uncomp_size_comp_data = u32::from_le_bytes(compressed[8..12].try_into().unwrap()) as usize;
    let uncomp_header_offset = u32::from_le_bytes(compressed[12..16].try_into().unwrap()) as usize;

    let uncompressed_size = uncomp_size_comp_data + UNCOMPRESSED_HEADER_SIZE;
    let uncomp_header_ptr = 0x10 + uncomp_header_offset;

    if uncomp_header_ptr + UNCOMPRESSED_HEADER_SIZE > compressed.len() {
        return Err(LaylaError::InvalidHeaderOffset {
            offset: uncomp_header_ptr,
            file_len: compressed.len(),
        });
    }

    let mut out = vec![0u8; uncompressed_size];
    // Copy the uncompressed header (0x100 bytes)
    out[..UNCOMPRESSED_HEADER_SIZE]
        .copy_from_slice(&compressed[uncomp_header_ptr..uncomp_header_ptr + UNCOMPRESSED_HEADER_SIZE]);

    let mut reader = BitReader::new(compressed, uncomp_header_ptr);

    let mut write_ptr = uncompressed_size.checked_sub(1).ok_or(LaylaError::TooShort)?;
    let min_addr = UNCOMPRESSED_HEADER_SIZE;

    while write_ptr >= min_addr {
        if reader.get_bit()? > 0 {
            let offset = reader.read_bits(13)? + 3;
            let mut length = 3;
            let mut level = reader.read_bits(2)?;
            length += level;
            if level == 3 {
                level = reader.read_bits(3)?;
                length += level;
                if level == 7 {
                    level = reader.read_bits(5)?;
                    length += level;
                    if level == 31 {
                        loop {
                            level = reader.read_bits(8)?;
                            length += level;
                            if level != 255 {
                                break;
                            }
                        }
                    }
                }
            }

            for _ in 0..length {
                let src_idx = write_ptr + offset;
                if src_idx >= uncompressed_size {
                    return Err(LaylaError::CorruptStream(format!(
                        "Source offset {} out of bounds (len {})",
                        src_idx, uncompressed_size
                    )));
                }
                out[write_ptr] = out[src_idx];
                if write_ptr == 0 {
                    break;
                }
                write_ptr -= 1;
            }
        } else {
            let b = reader.read_bits(8)? as u8;
            out[write_ptr] = b;
            if write_ptr == 0 {
                break;
            }
            write_ptr -= 1;
        }
    }

    Ok(out)
}

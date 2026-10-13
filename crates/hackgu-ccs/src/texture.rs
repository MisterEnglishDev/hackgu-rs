//! Texture decoder for CyberConnect2 CCS containers.
//!
//! Supports DXT1 (BC1), DXT5 (BC3), 32-bit RGBA, and 4/8-bit indexed (CLUT) textures.
//! Can export textures to DDS or uncompressed 32-bit BMP images with full alpha channels.

use std::fmt;
use std::fs::File;
use std::io::{self, Write};
use std::path::Path;

pub const TEXTURE_RGBA32: u8 = 0x00;
pub const TEXTURE_I8: u8 = 0x13;
pub const TEXTURE_I4: u8 = 0x14;
pub const TEXTURE_DXT1: u8 = 0x87;
pub const TEXTURE_DXT5: u8 = 0x89;

#[derive(Debug)]
pub enum TextureError {
    TooShort,
    UnsupportedFormat(u8),
    InvalidDimensions { width: usize, height: usize },
    Io(io::Error),
}

impl fmt::Display for TextureError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            TextureError::TooShort => write!(f, "Payload too short for texture"),
            TextureError::UnsupportedFormat(fmt) => write!(f, "Unsupported texture format: 0x{:X}", fmt),
            TextureError::InvalidDimensions { width, height } => {
                write!(f, "Invalid texture dimensions: {}x{}", width, height)
            }
            TextureError::Io(e) => write!(f, "I/O error: {}", e),
        }
    }
}

impl std::error::Error for TextureError {}

impl From<io::Error> for TextureError {
    fn from(e: io::Error) -> Self {
        TextureError::Io(e)
    }
}

#[derive(Clone, Debug)]
pub struct DecodedTexture {
    pub name: String,
    pub width: usize,
    pub height: usize,
    pub format: u8,
    pub raw_data: Vec<u8>,
    pub rgba: Vec<u8>, // Width * Height * 4 (RGBA8)
}

impl DecodedTexture {
    /// Parse a Texture object's payload and decode it to RGBA8 pixels.
    pub fn from_payload(name: &str, payload: &[u8]) -> Result<Self, TextureError> {
        Self::from_payload_with_clut(name, payload, None)
    }

    /// Parse a Texture object's payload, optionally passing a raw CLUT palette slice.
    pub fn from_payload_with_clut(
        name: &str,
        payload: &[u8],
        clut_raw: Option<&[u8]>,
    ) -> Result<Self, TextureError> {
        if payload.len() < 24 {
            return Err(TextureError::TooShort);
        }

        let _clut_id = i32::from_le_bytes(payload[0..4].try_into().unwrap());
        let _blit_group = i32::from_le_bytes(payload[4..8].try_into().unwrap());
        let _flags = payload[8];
        let tex_type = payload[9];
        let _mip_count = payload[10];

        let mut width = payload[12] as usize;
        let mut height = payload[13] as usize;

        // Case 1: Modern BTX Direct3D container (G.U. Last Recode HD textures)
        if payload.len() >= 28 && &payload[24..28] == b"btx\0" {
            let btx_len = u32::from_le_bytes(payload[20..24].try_into().unwrap()) as usize;
            if payload.len() < 24 + btx_len {
                return Err(TextureError::TooShort);
            }
            let btx = &payload[24..24 + btx_len];
            let btx_width = u16::from_le_bytes(btx[8..10].try_into().unwrap()) as usize;
            let btx_height = u16::from_le_bytes(btx[10..12].try_into().unwrap()) as usize;
            let header_size = u32::from_le_bytes(btx[24..28].try_into().unwrap()) as usize;
            let raw_slice = &btx[header_size..];

            let rgba = match tex_type {
                TEXTURE_DXT1 => decode_dxt1(raw_slice, btx_width, btx_height)?,
                TEXTURE_DXT5 => decode_dxt5(raw_slice, btx_width, btx_height)?,
                TEXTURE_RGBA32 => decode_rgba32(raw_slice, btx_width, btx_height)?,
                TEXTURE_I8 => {
                    let clut = clut_raw.ok_or(TextureError::UnsupportedFormat(tex_type))?;
                    decode_i8(raw_slice, btx_width, btx_height, clut)?
                }
                TEXTURE_I4 => {
                    let clut = clut_raw.ok_or(TextureError::UnsupportedFormat(tex_type))?;
                    decode_i4(raw_slice, btx_width, btx_height, clut)?
                }
                other => return Err(TextureError::UnsupportedFormat(other)),
            };

            return Ok(DecodedTexture {
                name: name.to_string(),
                width: btx_width,
                height: btx_height,
                format: tex_type,
                raw_data: raw_slice.to_vec(),
                rgba,
            });
        }

        // Case 2: Legacy PS2 CC2 format
        let data_start = if width == 0xff || height == 0xff {
            if payload.len() < 20 {
                return Err(TextureError::TooShort);
            }
            width = u16::from_le_bytes(payload[16..18].try_into().unwrap()) as usize;
            height = u16::from_le_bytes(payload[18..20].try_into().unwrap()) as usize;
            20
        } else {
            width = 1 << width;
            height = 1 << height;
            20
        };

        if data_start + 4 > payload.len() {
            return Err(TextureError::TooShort);
        }

        let data_size_field = u32::from_le_bytes(payload[data_start..data_start + 4].try_into().unwrap()) as usize;
        let pixel_bytes_start = data_start + 4;
        let pixel_bytes_len = data_size_field * 4;

        if pixel_bytes_start + pixel_bytes_len > payload.len() {
            return Err(TextureError::TooShort);
        }

        let raw_slice = &payload[pixel_bytes_start..pixel_bytes_start + pixel_bytes_len];

        let rgba = match tex_type {
            TEXTURE_DXT1 => decode_dxt1(raw_slice, width, height)?,
            TEXTURE_DXT5 => decode_dxt5(raw_slice, width, height)?,
            TEXTURE_RGBA32 => decode_rgba32(raw_slice, width, height)?,
            TEXTURE_I8 => {
                let clut = clut_raw.ok_or(TextureError::UnsupportedFormat(tex_type))?;
                decode_i8(raw_slice, width, height, clut)?
            }
            TEXTURE_I4 => {
                let clut = clut_raw.ok_or(TextureError::UnsupportedFormat(tex_type))?;
                decode_i4(raw_slice, width, height, clut)?
            }
            other => return Err(TextureError::UnsupportedFormat(other)),
        };

        Ok(DecodedTexture {
            name: name.to_string(),
            width,
            height,
            format: tex_type,
            raw_data: raw_slice.to_vec(),
            rgba,
        })
    }

    /// Save the texture as a 32-bit standard BMP file with alpha channel.
    pub fn save_bmp<P: AsRef<Path>>(&self, path: P) -> Result<(), io::Error> {
        let mut file = File::create(path)?;
        let row_size = self.width * 4;
        let image_size = (row_size * self.height) as u32;
        let file_size = 54 + image_size;

        // BMP Header (14 bytes)
        file.write_all(b"BM")?;
        file.write_all(&file_size.to_le_bytes())?;
        file.write_all(&[0u8; 4])?; // Reserved
        file.write_all(&54u32.to_le_bytes())?; // Pixel data offset

        // DIB Header (BITMAPINFOHEADER - 40 bytes)
        file.write_all(&40u32.to_le_bytes())?; // DIB size
        file.write_all(&(self.width as i32).to_le_bytes())?;
        file.write_all(&(-(self.height as i32)).to_le_bytes())?; // Top-down row order
        file.write_all(&1u16.to_le_bytes())?; // Color planes
        file.write_all(&32u16.to_le_bytes())?; // 32 bits per pixel
        file.write_all(&0u32.to_le_bytes())?; // BI_RGB (uncompressed)
        file.write_all(&image_size.to_le_bytes())?;
        file.write_all(&2835u32.to_le_bytes())?; // Horizontal resolution (72 DPI)
        file.write_all(&2835u32.to_le_bytes())?; // Vertical resolution
        file.write_all(&0u32.to_le_bytes())?; // Colors in palette
        file.write_all(&0u32.to_le_bytes())?; // Important colors

        // BMP stores pixels as BGRA
        let mut bgra = Vec::with_capacity(self.rgba.len());
        for chunk in self.rgba.chunks_exact(4) {
            bgra.push(chunk[2]); // B
            bgra.push(chunk[1]); // G
            bgra.push(chunk[0]); // R
            bgra.push(chunk[3]); // A
        }

        file.write_all(&bgra)?;
        Ok(())
    }

    /// Save the texture as a standard Microsoft DirectDraw Surface (DDS) file.
    pub fn save_dds<P: AsRef<Path>>(&self, path: P) -> Result<(), io::Error> {
        let mut file = File::create(path)?;
        let block_size = if self.format == TEXTURE_DXT1 { 8 } else { 16 };
        let pitch = (1.max((self.width + 3) / 4) * block_size) as u32;

        let four_cc = if self.format == TEXTURE_DXT1 {
            *b"DXT1"
        } else {
            *b"DXT5"
        };

        // DDS Magic
        file.write_all(b"DDS ")?;

        // DDS_HEADER (124 bytes)
        file.write_all(&124u32.to_le_bytes())?; // Size
        file.write_all(&0x000A1007u32.to_le_bytes())?; // Flags
        file.write_all(&(self.height as u32).to_le_bytes())?;
        file.write_all(&(self.width as u32).to_le_bytes())?;
        file.write_all(&pitch.to_le_bytes())?;
        file.write_all(&0u32.to_le_bytes())?; // Depth
        file.write_all(&0u32.to_le_bytes())?; // MipMapCount
        file.write_all(&[0u8; 44])?; // Reserved1[11]

        // DDS_PIXELFORMAT (32 bytes)
        file.write_all(&32u32.to_le_bytes())?; // Size
        file.write_all(&4u32.to_le_bytes())?; // DDPF_FOURCC
        file.write_all(&four_cc)?; // FourCC
        file.write_all(&[0u8; 20])?; // RGBBitCount and masks

        // Caps
        file.write_all(&0x00401008u32.to_le_bytes())?;
        file.write_all(&[0u8; 12])?; // Caps2, Caps3, Caps4
        file.write_all(&0u32.to_le_bytes())?; // Reserved2

        // Texture data
        file.write_all(&self.raw_data)?;
        Ok(())
    }
}

fn rgb565_to_rgba(c: u16) -> [u8; 4] {
    let r = (((c >> 11) & 0x1f) * 255 / 31) as u8;
    let g = (((c >> 5) & 0x3f) * 255 / 63) as u8;
    let b = ((c & 0x1f) * 255 / 31) as u8;
    [r, g, b, 255]
}

fn decode_dxt1(data: &[u8], width: usize, height: usize) -> Result<Vec<u8>, TextureError> {
    let mut out = vec![0u8; width * height * 4];
    let blocks_x = (width + 3) / 4;
    let blocks_y = (height + 3) / 4;
    let mut offset = 0;

    for by in 0..blocks_y {
        for bx in 0..blocks_x {
            if offset + 8 > data.len() {
                break;
            }

            let c0 = u16::from_le_bytes(data[offset..offset + 2].try_into().unwrap());
            let c1 = u16::from_le_bytes(data[offset + 2..offset + 4].try_into().unwrap());
            let lookup = u32::from_le_bytes(data[offset + 4..offset + 8].try_into().unwrap());
            offset += 8;

            let col0 = rgb565_to_rgba(c0);
            let col1 = rgb565_to_rgba(c1);

            let (col2, col3) = if c0 > c1 {
                (
                    [
                        ((2 * col0[0] as u16 + col1[0] as u16) / 3) as u8,
                        ((2 * col0[1] as u16 + col1[1] as u16) / 3) as u8,
                        ((2 * col0[2] as u16 + col1[2] as u16) / 3) as u8,
                        255,
                    ],
                    [
                        ((col0[0] as u16 + 2 * col1[0] as u16) / 3) as u8,
                        ((col0[1] as u16 + 2 * col1[1] as u16) / 3) as u8,
                        ((col0[2] as u16 + 2 * col1[2] as u16) / 3) as u8,
                        255,
                    ],
                )
            } else {
                (
                    [
                        ((col0[0] as u16 + col1[0] as u16) / 2) as u8,
                        ((col0[1] as u16 + col1[1] as u16) / 2) as u8,
                        ((col0[2] as u16 + col1[2] as u16) / 2) as u8,
                        255,
                    ],
                    [0, 0, 0, 0],
                )
            };

            let colors = [col0, col1, col2, col3];

            for py in 0..4 {
                for px in 0..4 {
                    let gx = bx * 4 + px;
                    let gy = by * 4 + py;
                    if gx < width && gy < height {
                        let bit_idx = (py * 4 + px) * 2;
                        let code = ((lookup >> bit_idx) & 3) as usize;
                        let out_idx = (gy * width + gx) * 4;
                        out[out_idx..out_idx + 4].copy_from_slice(&colors[code]);
                    }
                }
            }
        }
    }

    Ok(out)
}

fn decode_dxt5(data: &[u8], width: usize, height: usize) -> Result<Vec<u8>, TextureError> {
    let mut out = vec![0u8; width * height * 4];
    let blocks_x = (width + 3) / 4;
    let blocks_y = (height + 3) / 4;
    let mut offset = 0;

    for by in 0..blocks_y {
        for bx in 0..blocks_x {
            if offset + 16 > data.len() {
                break;
            }

            let a0 = data[offset];
            let a1 = data[offset + 1];
            let mut alpha_indices = [0u8; 16];
            let raw_a_bits = (data[offset + 2] as u64)
                | ((data[offset + 3] as u64) << 8)
                | ((data[offset + 4] as u64) << 16)
                | ((data[offset + 5] as u64) << 24)
                | ((data[offset + 6] as u64) << 32)
                | ((data[offset + 7] as u64) << 40);

            for i in 0..16 {
                alpha_indices[i] = ((raw_a_bits >> (i * 3)) & 7) as u8;
            }

            let mut alphas = [0u8; 8];
            alphas[0] = a0;
            alphas[1] = a1;
            if a0 > a1 {
                for i in 1..=6 {
                    alphas[i + 1] = (((7 - i) * a0 as usize + i * a1 as usize) / 7) as u8;
                }
            } else {
                for i in 1..=4 {
                    alphas[i + 1] = (((5 - i) * a0 as usize + i * a1 as usize) / 5) as u8;
                }
                alphas[6] = 0;
                alphas[7] = 255;
            }

            let c0 = u16::from_le_bytes(data[offset + 8..offset + 10].try_into().unwrap());
            let c1 = u16::from_le_bytes(data[offset + 10..offset + 12].try_into().unwrap());
            let lookup = u32::from_le_bytes(data[offset + 12..offset + 16].try_into().unwrap());
            offset += 16;

            let col0 = rgb565_to_rgba(c0);
            let col1 = rgb565_to_rgba(c1);
            let col2 = [
                ((2 * col0[0] as u16 + col1[0] as u16) / 3) as u8,
                ((2 * col0[1] as u16 + col1[1] as u16) / 3) as u8,
                ((2 * col0[2] as u16 + col1[2] as u16) / 3) as u8,
                255,
            ];
            let col3 = [
                ((col0[0] as u16 + 2 * col1[0] as u16) / 3) as u8,
                ((col0[1] as u16 + 2 * col1[1] as u16) / 3) as u8,
                ((col0[2] as u16 + 2 * col1[2] as u16) / 3) as u8,
                255,
            ];
            let colors = [col0, col1, col2, col3];

            for py in 0..4 {
                for px in 0..4 {
                    let gx = bx * 4 + px;
                    let gy = by * 4 + py;
                    if gx < width && gy < height {
                        let p_idx = py * 4 + px;
                        let code = ((lookup >> (p_idx * 2)) & 3) as usize;
                        let alpha = alphas[alpha_indices[p_idx] as usize];
                        let mut c = colors[code];
                        c[3] = alpha;

                        let out_idx = (gy * width + gx) * 4;
                        out[out_idx..out_idx + 4].copy_from_slice(&c);
                    }
                }
            }
        }
    }

    Ok(out)
}

fn decode_rgba32(data: &[u8], width: usize, height: usize) -> Result<Vec<u8>, TextureError> {
    let mut out = vec![0u8; width * height * 4];
    let n = (width * height).min(data.len() / 4);

    for i in 0..n {
        let b = data[i * 4];
        let g = data[i * 4 + 1];
        let r = data[i * 4 + 2];
        let mut a = data[i * 4 + 3];
        if a >= 0x7f {
            a = 0xff;
        } else {
            a = a.saturating_mul(2);
        }

        out[i * 4] = r;
        out[i * 4 + 1] = g;
        out[i * 4 + 2] = b;
        out[i * 4 + 3] = a;
    }

    Ok(out)
}

fn decode_i8(data: &[u8], width: usize, height: usize, clut: &[u8]) -> Result<Vec<u8>, TextureError> {
    let mut out = vec![0u8; width * height * 4];
    let n = (width * height).min(data.len());

    for i in 0..n {
        let idx = data[i] as usize;
        let c_idx = idx * 4;
        if c_idx + 4 <= clut.len() {
            let r = clut[c_idx];
            let g = clut[c_idx + 1];
            let b = clut[c_idx + 2];
            let mut a = clut[c_idx + 3];
            if a >= 0x7f {
                a = 0xff;
            } else {
                a = a.saturating_mul(2);
            }

            out[i * 4] = r;
            out[i * 4 + 1] = g;
            out[i * 4 + 2] = b;
            out[i * 4 + 3] = a;
        }
    }

    Ok(out)
}

fn decode_i4(data: &[u8], width: usize, height: usize, clut: &[u8]) -> Result<Vec<u8>, TextureError> {
    let mut out = vec![0u8; width * height * 4];
    let mut p_idx = 0;

    for &byte in data {
        for nibble in [byte & 0x0f, (byte >> 4) & 0x0f] {
            if p_idx < width * height {
                let c_idx = (nibble as usize) * 4;
                if c_idx + 4 <= clut.len() {
                    let r = clut[c_idx];
                    let g = clut[c_idx + 1];
                    let b = clut[c_idx + 2];
                    let mut a = clut[c_idx + 3];
                    if a >= 0x7f {
                        a = 0xff;
                    } else {
                        a = a.saturating_mul(2);
                    }

                    out[p_idx * 4] = r;
                    out[p_idx * 4 + 1] = g;
                    out[p_idx * 4 + 2] = b;
                    out[p_idx * 4 + 3] = a;
                }
                p_idx += 1;
            }
        }
    }

    Ok(out)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::parser::CcsFile;

    #[test]
    fn test_inspect_weapon_clut() {
        let path = "../../target/vol1/data/data/eq/w2dsw00l.ccs";
        if !std::path::Path::new(path).exists() {
            return;
        }
        let data = std::fs::read(path).unwrap();
        let ccs = CcsFile::parse(&data).unwrap();

        for o in &ccs.raw_objects {
            if o.section_type == crate::objects::SECTION_TEXTURE || o.section_type == crate::objects::SECTION_CLUT {
                let name = ccs.object_name(o.object_id).unwrap_or("");
                println!("Object id={}, sec={:04x}, name='{}', len={}", o.object_id, o.section_type, name, o.payload.len());
                if o.section_type == crate::objects::SECTION_TEXTURE && o.payload.len() >= 24 {
                    let clut_id = i32::from_le_bytes(o.payload[0..4].try_into().unwrap());
                    let clut_obj = ccs.raw_objects.iter().find(|o| o.object_id == clut_id as u32).unwrap();
                    let clut_raw = &clut_obj.payload[16..16 + 1024];

                    let width = 1 << o.payload[12];
                    let height = 1 << o.payload[13];
                    let pixels = &o.payload[24..24 + width * height];

                    // Test 1: Straight CLUT
                    let mut out1 = vec![0u8; width * height * 4];
                    for (i, &idx) in pixels.iter().enumerate() {
                        let c_idx = idx as usize * 4;
                        let r = clut_raw[c_idx];
                        let g = clut_raw[c_idx + 1];
                        let b = clut_raw[c_idx + 2];
                        let a = clut_raw[c_idx + 3].saturating_mul(2);
                        out1[i * 4] = r;
                        out1[i * 4 + 1] = g;
                        out1[i * 4 + 2] = b;
                        out1[i * 4 + 3] = if a >= 128 { 255 } else { a };
                    }
                    let tex1 = DecodedTexture {
                        name: "straight".into(),
                        width,
                        height,
                        format: 0x13,
                        raw_data: pixels.to_vec(),
                        rgba: out1,
                    };
                    tex1.save_bmp("../../target/w2dsw00_straight.bmp").unwrap();

                    // Test 2: PS2 GS unswizzled CLUT
                    let mut out2 = vec![0u8; width * height * 4];
                    for (i, &idx) in pixels.iter().enumerate() {
                        let chunk = (idx as usize / 32) * 32;
                        let sub = idx as usize % 32;
                        let mapped = if (8..16).contains(&sub) {
                            chunk + sub + 8
                        } else if (16..24).contains(&sub) {
                            chunk + sub - 8
                        } else {
                            idx as usize
                        };
                        let c_idx = mapped * 4;
                        let r = clut_raw[c_idx];
                        let g = clut_raw[c_idx + 1];
                        let b = clut_raw[c_idx + 2];
                        let a = clut_raw[c_idx + 3].saturating_mul(2);
                        out2[i * 4] = r;
                        out2[i * 4 + 1] = g;
                        out2[i * 4 + 2] = b;
                        out2[i * 4 + 3] = if a >= 128 { 255 } else { a };
                    }
                    let tex2 = DecodedTexture {
                        name: "unswizzled".into(),
                        width,
                        height,
                        format: 0x13,
                        raw_data: pixels.to_vec(),
                        rgba: out2,
                    };
                    tex2.save_bmp("../../target/w2dsw00_unswizzled.bmp").unwrap();
                    println!("Saved straight and unswizzled BMPs to target/");
                }
            }
        }
    }
}

//! CRI @UTF Universal Table Format parser and decryptor.

use std::collections::HashMap;
use std::fmt;

pub const UTF_MAGIC: &[u8; 4] = b"@UTF";

pub const COLUMN_STORAGE_MASK: u8 = 0xf0;
pub const COLUMN_STORAGE_PER_ROW: u8 = 0x50;
pub const COLUMN_STORAGE_CONSTANT: u8 = 0x30;
pub const COLUMN_STORAGE_ZERO: u8 = 0x10;

pub const COLUMN_TYPE_MASK: u8 = 0x0f;
pub const COLUMN_TYPE_U8: u8 = 0x00;
pub const COLUMN_TYPE_I8: u8 = 0x01;
pub const COLUMN_TYPE_U16: u8 = 0x02;
pub const COLUMN_TYPE_I16: u8 = 0x03;
pub const COLUMN_TYPE_U32: u8 = 0x04;
pub const COLUMN_TYPE_I32: u8 = 0x05;
pub const COLUMN_TYPE_U64: u8 = 0x06;
pub const COLUMN_TYPE_I64: u8 = 0x07;
pub const COLUMN_TYPE_FLOAT: u8 = 0x08;
pub const COLUMN_TYPE_DOUBLE: u8 = 0x09;
pub const COLUMN_TYPE_STRING: u8 = 0x0a;
pub const COLUMN_TYPE_DATA: u8 = 0x0b;

#[derive(Debug)]
pub enum UtfError {
    InvalidMagic,
    TooShort,
    OutOfBounds(String),
    InvalidString(String),
}

impl fmt::Display for UtfError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            UtfError::InvalidMagic => write!(f, "Invalid @UTF signature"),
            UtfError::TooShort => write!(f, "Data too short for @UTF table"),
            UtfError::OutOfBounds(s) => write!(f, "Out of bounds reading @UTF: {}", s),
            UtfError::InvalidString(s) => write!(f, "Invalid UTF string: {}", s),
        }
    }
}

impl std::error::Error for UtfError {}

/// Decrypts CRI @UTF obfuscated data using the standard 0x5f/0x15 keystream.
pub fn decrypt_utf_bytes(data: &mut [u8]) {
    let mut cur = 0x5f_u8;
    let mult = 0x15_u8;
    for b in data.iter_mut() {
        *b ^= cur;
        cur = cur.wrapping_mul(mult);
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum UtfValue {
    U8(u8),
    I8(i8),
    U16(u16),
    I16(i16),
    U32(u32),
    I32(i32),
    U64(u64),
    I64(i64),
    Float(f32),
    Double(f64),
    String(String),
    Data(Vec<u8>),
    Null,
}

impl UtfValue {
    pub fn as_u64(&self) -> Option<u64> {
        match self {
            UtfValue::U8(v) => Some(*v as u64),
            UtfValue::I8(v) => Some(*v as u64),
            UtfValue::U16(v) => Some(*v as u64),
            UtfValue::I16(v) => Some(*v as u64),
            UtfValue::U32(v) => Some(*v as u64),
            UtfValue::I32(v) => Some(*v as u64),
            UtfValue::U64(v) => Some(*v),
            UtfValue::I64(v) => Some(*v as u64),
            _ => None,
        }
    }

    pub fn as_str(&self) -> Option<&str> {
        match self {
            UtfValue::String(s) => Some(s),
            _ => None,
        }
    }

    pub fn as_bytes(&self) -> Option<&[u8]> {
        match self {
            UtfValue::Data(d) => Some(d),
            _ => None,
        }
    }
}

#[derive(Clone, Debug)]
pub struct UtfColumn {
    pub name: String,
    pub storage: u8,
    pub col_type: u8,
    pub constant: Option<UtfValue>,
}

#[derive(Clone, Debug)]
pub struct UtfRow {
    pub values: HashMap<String, UtfValue>,
}

impl UtfRow {
    pub fn get(&self, col_name: &str) -> Option<&UtfValue> {
        self.values.get(col_name)
    }

    pub fn get_u64(&self, col_name: &str) -> Option<u64> {
        self.values.get(col_name).and_then(|v| v.as_u64())
    }

    pub fn get_str(&self, col_name: &str) -> Option<&str> {
        self.values.get(col_name).and_then(|v| v.as_str())
    }
}

#[derive(Clone, Debug)]
pub struct UtfTable {
    pub name: String,
    pub columns: Vec<UtfColumn>,
    pub rows: Vec<UtfRow>,
}

impl UtfTable {
    /// Parse a decrypted @UTF table slice.
    pub fn parse(data: &[u8]) -> Result<Self, UtfError> {
        if data.len() < 32 {
            return Err(UtfError::TooShort);
        }
        if &data[..4] != UTF_MAGIC {
            return Err(UtfError::InvalidMagic);
        }

        let _table_size = u32::from_be_bytes(data[4..8].try_into().unwrap()) as usize;
        let rows_offset = u32::from_be_bytes(data[8..12].try_into().unwrap()) as usize;
        let string_table_offset = u32::from_be_bytes(data[12..16].try_into().unwrap()) as usize;
        let data_offset = u32::from_be_bytes(data[16..20].try_into().unwrap()) as usize;
        let table_name_offset = u32::from_be_bytes(data[20..24].try_into().unwrap()) as usize;
        let num_columns = u16::from_be_bytes(data[24..26].try_into().unwrap()) as usize;
        let row_width = u16::from_be_bytes(data[26..28].try_into().unwrap()) as usize;
        let num_rows = u32::from_be_bytes(data[28..32].try_into().unwrap()) as usize;

        let str_base = 8 + string_table_offset;
        let get_string = |offset: usize| -> Result<String, UtfError> {
            let start = str_base + offset;
            if start >= data.len() {
                return Err(UtfError::OutOfBounds(format!("String offset {} >= len {}", start, data.len())));
            }
            let mut end = start;
            while end < data.len() && data[end] != 0 {
                end += 1;
            }
            let s = std::str::from_utf8(&data[start..end])
                .map_err(|e| UtfError::InvalidString(e.to_string()))?;
            Ok(s.to_string())
        };

        let table_name = get_string(table_name_offset)?;

        let mut columns = Vec::with_capacity(num_columns);
        let mut pos = 32;

        for _ in 0..num_columns {
            if pos >= data.len() {
                return Err(UtfError::OutOfBounds("Column header overflow".into()));
            }
            let col_flag = data[pos];
            pos += 1;
            let col_name_offset = u32::from_be_bytes(data[pos..pos + 4].try_into().unwrap()) as usize;
            pos += 4;
            let col_name = get_string(col_name_offset)?;

            let storage = col_flag & COLUMN_STORAGE_MASK;
            let col_type = col_flag & COLUMN_TYPE_MASK;

            let mut constant = None;
            if storage == COLUMN_STORAGE_CONSTANT {
                let val = match col_type {
                    COLUMN_TYPE_U8 => {
                        let v = data[pos];
                        pos += 1;
                        UtfValue::U8(v)
                    }
                    COLUMN_TYPE_I8 => {
                        let v = data[pos] as i8;
                        pos += 1;
                        UtfValue::I8(v)
                    }
                    COLUMN_TYPE_U16 => {
                        let v = u16::from_be_bytes(data[pos..pos + 2].try_into().unwrap());
                        pos += 2;
                        UtfValue::U16(v)
                    }
                    COLUMN_TYPE_I16 => {
                        let v = i16::from_be_bytes(data[pos..pos + 2].try_into().unwrap());
                        pos += 2;
                        UtfValue::I16(v)
                    }
                    COLUMN_TYPE_U32 => {
                        let v = u32::from_be_bytes(data[pos..pos + 4].try_into().unwrap());
                        pos += 4;
                        UtfValue::U32(v)
                    }
                    COLUMN_TYPE_I32 => {
                        let v = i32::from_be_bytes(data[pos..pos + 4].try_into().unwrap());
                        pos += 4;
                        UtfValue::I32(v)
                    }
                    COLUMN_TYPE_U64 => {
                        let v = u64::from_be_bytes(data[pos..pos + 8].try_into().unwrap());
                        pos += 8;
                        UtfValue::U64(v)
                    }
                    COLUMN_TYPE_I64 => {
                        let v = i64::from_be_bytes(data[pos..pos + 8].try_into().unwrap());
                        pos += 8;
                        UtfValue::I64(v)
                    }
                    COLUMN_TYPE_FLOAT => {
                        let v = f32::from_be_bytes(data[pos..pos + 4].try_into().unwrap());
                        pos += 4;
                        UtfValue::Float(v)
                    }
                    COLUMN_TYPE_DOUBLE => {
                        let v = f64::from_be_bytes(data[pos..pos + 8].try_into().unwrap());
                        pos += 8;
                        UtfValue::Double(v)
                    }
                    COLUMN_TYPE_STRING => {
                        let s_off = u32::from_be_bytes(data[pos..pos + 4].try_into().unwrap()) as usize;
                        pos += 4;
                        UtfValue::String(get_string(s_off)?)
                    }
                    COLUMN_TYPE_DATA => {
                        let d_off = u32::from_be_bytes(data[pos..pos + 4].try_into().unwrap()) as usize;
                        let d_sz = u32::from_be_bytes(data[pos + 4..pos + 8].try_into().unwrap()) as usize;
                        pos += 8;
                        let start = 8 + data_offset + d_off;
                        let slice = data[start..start + d_sz].to_vec();
                        UtfValue::Data(slice)
                    }
                    _ => UtfValue::Null,
                };
                constant = Some(val);
            }

            columns.push(UtfColumn {
                name: col_name,
                storage,
                col_type,
                constant,
            });
        }

        let mut rows = Vec::with_capacity(num_rows);
        let rows_base = 8 + rows_offset;

        for r in 0..num_rows {
            let mut row_pos = rows_base + (r * row_width);
            let mut values = HashMap::with_capacity(num_columns);

            for col in &columns {
                if let Some(ref c) = col.constant {
                    values.insert(col.name.clone(), c.clone());
                    continue;
                }
                if col.storage == COLUMN_STORAGE_ZERO {
                    let zero_val = match col.col_type {
                        COLUMN_TYPE_U8 | COLUMN_TYPE_I8 => UtfValue::U8(0),
                        COLUMN_TYPE_U16 | COLUMN_TYPE_I16 => UtfValue::U16(0),
                        COLUMN_TYPE_U32 | COLUMN_TYPE_I32 => UtfValue::U32(0),
                        COLUMN_TYPE_U64 | COLUMN_TYPE_I64 => UtfValue::U64(0),
                        COLUMN_TYPE_FLOAT => UtfValue::Float(0.0),
                        COLUMN_TYPE_DOUBLE => UtfValue::Double(0.0),
                        COLUMN_TYPE_STRING => UtfValue::String(String::new()),
                        COLUMN_TYPE_DATA => UtfValue::Data(Vec::new()),
                        _ => UtfValue::Null,
                    };
                    values.insert(col.name.clone(), zero_val);
                    continue;
                }

                let val = match col.col_type {
                    COLUMN_TYPE_U8 => {
                        let v = data[row_pos];
                        row_pos += 1;
                        UtfValue::U8(v)
                    }
                    COLUMN_TYPE_I8 => {
                        let v = data[row_pos] as i8;
                        row_pos += 1;
                        UtfValue::I8(v)
                    }
                    COLUMN_TYPE_U16 => {
                        let v = u16::from_be_bytes(data[row_pos..row_pos + 2].try_into().unwrap());
                        row_pos += 2;
                        UtfValue::U16(v)
                    }
                    COLUMN_TYPE_I16 => {
                        let v = i16::from_be_bytes(data[row_pos..row_pos + 2].try_into().unwrap());
                        row_pos += 2;
                        UtfValue::I16(v)
                    }
                    COLUMN_TYPE_U32 => {
                        let v = u32::from_be_bytes(data[row_pos..row_pos + 4].try_into().unwrap());
                        row_pos += 4;
                        UtfValue::U32(v)
                    }
                    COLUMN_TYPE_I32 => {
                        let v = i32::from_be_bytes(data[row_pos..row_pos + 4].try_into().unwrap());
                        row_pos += 4;
                        UtfValue::I32(v)
                    }
                    COLUMN_TYPE_U64 => {
                        let v = u64::from_be_bytes(data[row_pos..row_pos + 8].try_into().unwrap());
                        row_pos += 8;
                        UtfValue::U64(v)
                    }
                    COLUMN_TYPE_I64 => {
                        let v = i64::from_be_bytes(data[row_pos..row_pos + 8].try_into().unwrap());
                        row_pos += 8;
                        UtfValue::I64(v)
                    }
                    COLUMN_TYPE_FLOAT => {
                        let v = f32::from_be_bytes(data[row_pos..row_pos + 4].try_into().unwrap());
                        row_pos += 4;
                        UtfValue::Float(v)
                    }
                    COLUMN_TYPE_DOUBLE => {
                        let v = f64::from_be_bytes(data[row_pos..row_pos + 8].try_into().unwrap());
                        row_pos += 8;
                        UtfValue::Double(v)
                    }
                    COLUMN_TYPE_STRING => {
                        let s_off = u32::from_be_bytes(data[row_pos..row_pos + 4].try_into().unwrap()) as usize;
                        row_pos += 4;
                        UtfValue::String(get_string(s_off)?)
                    }
                    COLUMN_TYPE_DATA => {
                        let d_off = u32::from_be_bytes(data[row_pos..row_pos + 4].try_into().unwrap()) as usize;
                        let d_sz = u32::from_be_bytes(data[row_pos + 4..row_pos + 8].try_into().unwrap()) as usize;
                        row_pos += 8;
                        let start = 8 + data_offset + d_off;
                        let slice = data[start..start + d_sz].to_vec();
                        UtfValue::Data(slice)
                    }
                    _ => UtfValue::Null,
                };
                values.insert(col.name.clone(), val);
            }

            rows.push(UtfRow { values });
        }

        Ok(UtfTable {
            name: table_name,
            columns,
            rows,
        })
    }
}

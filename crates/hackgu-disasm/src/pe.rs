//! PE (Portable Executable) binary parser for x64 Windows game DLLs/EXEs.
//! Extracts section headers, export tables, import tables, and raw code/data sections.

use std::collections::HashMap;

#[derive(Debug, Clone)]
pub struct SectionHeader {
    pub name: String,
    pub virtual_size: u32,
    pub virtual_address: u32,
    pub size_of_raw_data: u32,
    pub pointer_to_raw_data: u32,
    pub characteristics: u32,
}

#[derive(Debug, Clone)]
pub struct ExportSymbol {
    pub ordinal: u16,
    pub name: Option<String>,
    pub rva: u32,
}

#[derive(Debug, Clone)]
pub struct ImportSymbol {
    pub dll_name: String,
    pub function_name: Option<String>,
    pub ordinal: Option<u16>,
    pub rva: u32,
}

pub struct PeFile<'a> {
    pub data: &'a [u8],
    pub image_base: u64,
    pub entry_point: u32,
    pub sections: Vec<SectionHeader>,
    pub exports: Vec<ExportSymbol>,
    pub imports: Vec<ImportSymbol>,
}

impl<'a> PeFile<'a> {
    pub fn parse(data: &'a [u8]) -> Result<Self, String> {
        if data.len() < 0x40 {
            return Err("File too small for DOS header".into());
        }
        if &data[0..2] != b"MZ" {
            return Err("Invalid DOS magic (expected MZ)".into());
        }

        let e_lfanew = u32::from_le_bytes(data[0x3C..0x40].try_into().unwrap()) as usize;
        if data.len() < e_lfanew + 0x18 {
            return Err("Invalid PE header offset".into());
        }
        if &data[e_lfanew..e_lfanew + 4] != b"PE\0\0" {
            return Err("Invalid PE signature".into());
        }

        let coff_start = e_lfanew + 4;
        let _machine = u16::from_le_bytes(data[coff_start..coff_start + 2].try_into().unwrap());
        let num_sections = u16::from_le_bytes(data[coff_start + 2..coff_start + 4].try_into().unwrap()) as usize;
        let opt_hdr_size = u16::from_le_bytes(data[coff_start + 16..coff_start + 18].try_into().unwrap()) as usize;

        let opt_start = coff_start + 20;
        let opt_magic = u16::from_le_bytes(data[opt_start..opt_start + 2].try_into().unwrap());

        let (image_base, entry_point, rva_offsets) = if opt_magic == 0x20b {
            // PE32+ (64-bit)
            let entry = u32::from_le_bytes(data[opt_start + 16..opt_start + 20].try_into().unwrap());
            let base = u64::from_le_bytes(data[opt_start + 24..opt_start + 32].try_into().unwrap());
            let rvas = opt_start + 112; // Start of Data Directories for PE32+
            (base, entry, rvas)
        } else if opt_magic == 0x10b {
            // PE32 (32-bit)
            let entry = u32::from_le_bytes(data[opt_start + 16..opt_start + 20].try_into().unwrap());
            let base = u32::from_le_bytes(data[opt_start + 28..opt_start + 32].try_into().unwrap()) as u64;
            let rvas = opt_start + 96;
            (base, entry, rvas)
        } else {
            return Err(format!("Unknown optional header magic: 0x{:x}", opt_magic));
        };

        // Parse section headers
        let sec_headers_start = opt_start + opt_hdr_size;
        let mut sections = Vec::with_capacity(num_sections);

        for i in 0..num_sections {
            let s_off = sec_headers_start + i * 40;
            if s_off + 40 > data.len() {
                break;
            }
            let name_raw = &data[s_off..s_off + 8];
            let name_len = name_raw.iter().position(|&b| b == 0).unwrap_or(8);
            let name = String::from_utf8_lossy(&name_raw[..name_len]).into_owned();

            let virtual_size = u32::from_le_bytes(data[s_off + 8..s_off + 12].try_into().unwrap());
            let virtual_address = u32::from_le_bytes(data[s_off + 12..s_off + 16].try_into().unwrap());
            let size_of_raw_data = u32::from_le_bytes(data[s_off + 16..s_off + 20].try_into().unwrap());
            let pointer_to_raw_data = u32::from_le_bytes(data[s_off + 20..s_off + 24].try_into().unwrap());
            let characteristics = u32::from_le_bytes(data[s_off + 36..s_off + 40].try_into().unwrap());

            sections.push(SectionHeader {
                name,
                virtual_size,
                virtual_address,
                size_of_raw_data,
                pointer_to_raw_data,
                characteristics,
            });
        }

        let mut pe = PeFile {
            data,
            image_base,
            entry_point,
            sections,
            exports: Vec::new(),
            imports: Vec::new(),
        };

        // Export Directory (Data Directory #0)
        if rva_offsets + 8 <= data.len() {
            let export_rva = u32::from_le_bytes(data[rva_offsets..rva_offsets + 4].try_into().unwrap());
            let export_size = u32::from_le_bytes(data[rva_offsets + 4..rva_offsets + 8].try_into().unwrap());
            if export_rva != 0 && export_size != 0 {
                pe.parse_exports(export_rva)?;
            }
        }

        // Import Directory (Data Directory #1)
        if rva_offsets + 16 <= data.len() {
            let import_rva = u32::from_le_bytes(data[rva_offsets + 8..rva_offsets + 12].try_into().unwrap());
            let import_size = u32::from_le_bytes(data[rva_offsets + 12..rva_offsets + 16].try_into().unwrap());
            if import_rva != 0 && import_size != 0 {
                pe.parse_imports(import_rva, opt_magic == 0x20b)?;
            }
        }

        Ok(pe)
    }

    pub fn rva_to_offset(&self, rva: u32) -> Option<usize> {
        for s in &self.sections {
            let v_start = s.virtual_address;
            let v_end = v_start + s.virtual_size.max(s.size_of_raw_data);
            if rva >= v_start && rva < v_end {
                let delta = rva - v_start;
                if delta < s.size_of_raw_data {
                    return Some((s.pointer_to_raw_data + delta) as usize);
                }
            }
        }
        None
    }

    pub fn get_section<'s>(&'s self, name: &str) -> Option<(&'s SectionHeader, &'a [u8])> {
        for s in &self.sections {
            if s.name == name {
                let start = s.pointer_to_raw_data as usize;
                let end = start + s.size_of_raw_data as usize;
                if end <= self.data.len() {
                    return Some((s, &self.data[start..end]));
                }
            }
        }
        None
    }

    fn read_cstring(&self, offset: usize) -> Option<String> {
        if offset >= self.data.len() {
            return None;
        }
        let end = self.data[offset..].iter().position(|&b| b == 0)?;
        Some(String::from_utf8_lossy(&self.data[offset..offset + end]).into_owned())
    }

    fn parse_exports(&mut self, export_rva: u32) -> Result<(), String> {
        let off = match self.rva_to_offset(export_rva) {
            Some(o) => o,
            None => return Ok(()),
        };
        if off + 40 > self.data.len() {
            return Ok(());
        }

        let num_functions = u32::from_le_bytes(self.data[off + 20..off + 24].try_into().unwrap()) as usize;
        let num_names = u32::from_le_bytes(self.data[off + 24..off + 28].try_into().unwrap()) as usize;
        let funcs_rva = u32::from_le_bytes(self.data[off + 28..off + 32].try_into().unwrap());
        let names_rva = u32::from_le_bytes(self.data[off + 32..off + 36].try_into().unwrap());
        let ords_rva = u32::from_le_bytes(self.data[off + 36..off + 40].try_into().unwrap());
        let ord_base = u16::from_le_bytes(self.data[off + 16..off + 18].try_into().unwrap());

        let funcs_off = self.rva_to_offset(funcs_rva).unwrap_or(0);
        let names_off = self.rva_to_offset(names_rva).unwrap_or(0);
        let ords_off = self.rva_to_offset(ords_rva).unwrap_or(0);

        let mut name_to_ord = HashMap::new();
        for i in 0..num_names {
            if names_off + (i + 1) * 4 <= self.data.len() && ords_off + (i + 1) * 2 <= self.data.len() {
                let name_rva = u32::from_le_bytes(self.data[names_off + i * 4..names_off + (i + 1) * 4].try_into().unwrap());
                let ord = u16::from_le_bytes(self.data[ords_off + i * 2..ords_off + (i + 1) * 2].try_into().unwrap());
                if let Some(n_off) = self.rva_to_offset(name_rva) {
                    if let Some(s) = self.read_cstring(n_off) {
                        name_to_ord.insert(ord, s);
                    }
                }
            }
        }

        for i in 0..num_functions {
            if funcs_off + (i + 1) * 4 <= self.data.len() {
                let fn_rva = u32::from_le_bytes(self.data[funcs_off + i * 4..funcs_off + (i + 1) * 4].try_into().unwrap());
                if fn_rva != 0 {
                    let ord = ord_base + i as u16;
                    let name = name_to_ord.remove(&(i as u16));
                    self.exports.push(ExportSymbol {
                        ordinal: ord,
                        name,
                        rva: fn_rva,
                    });
                }
            }
        }

        Ok(())
    }

    fn parse_imports(&mut self, import_rva: u32, is_64: bool) -> Result<(), String> {
        let mut cur = match self.rva_to_offset(import_rva) {
            Some(o) => o,
            None => return Ok(()),
        };

        while cur + 20 <= self.data.len() {
            let ilt_rva = u32::from_le_bytes(self.data[cur..cur + 4].try_into().unwrap());
            let name_rva = u32::from_le_bytes(self.data[cur + 12..cur + 16].try_into().unwrap());
            let iat_rva = u32::from_le_bytes(self.data[cur + 16..cur + 20].try_into().unwrap());

            if ilt_rva == 0 && name_rva == 0 && iat_rva == 0 {
                break;
            }

            let dll_name = self.rva_to_offset(name_rva)
                .and_then(|o| self.read_cstring(o))
                .unwrap_or_else(|| "UNKNOWN.dll".into());

            let lookup_rva = if ilt_rva != 0 { ilt_rva } else { iat_rva };
            if let Some(mut lookup_off) = self.rva_to_offset(lookup_rva) {
                let mut slot_rva = iat_rva;
                let step = if is_64 { 8 } else { 4 };

                while lookup_off + step <= self.data.len() {
                    let entry = if is_64 {
                        u64::from_le_bytes(self.data[lookup_off..lookup_off + 8].try_into().unwrap())
                    } else {
                        u32::from_le_bytes(self.data[lookup_off..lookup_off + 4].try_into().unwrap()) as u64
                    };

                    if entry == 0 {
                        break;
                    }

                    let is_ordinal = if is_64 {
                        (entry & 0x8000_0000_0000_0000) != 0
                    } else {
                        (entry & 0x8000_0000) != 0
                    };

                    if is_ordinal {
                        self.imports.push(ImportSymbol {
                            dll_name: dll_name.clone(),
                            function_name: None,
                            ordinal: Some((entry & 0xffff) as u16),
                            rva: slot_rva,
                        });
                    } else {
                        let hint_rva = (entry & 0x7fff_ffff) as u32;
                        if let Some(hint_off) = self.rva_to_offset(hint_rva) {
                            if hint_off + 2 < self.data.len() {
                                let fn_name = self.read_cstring(hint_off + 2);
                                self.imports.push(ImportSymbol {
                                    dll_name: dll_name.clone(),
                                    function_name: fn_name,
                                    ordinal: None,
                                    rva: slot_rva,
                                });
                            }
                        }
                    }

                    lookup_off += step;
                    slot_rva += step as u32;
                }
            }

            cur += 20;
        }

        Ok(())
    }
}

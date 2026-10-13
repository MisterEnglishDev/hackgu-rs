//! MSVC x64 RTTI (Run-Time Type Information) and Virtual Method Table (vftable) parser.
//! Reconstructs the complete C++ class hierarchy, base classes, and virtual function tables
//! directly from compiled executable binaries.

use std::collections::{BTreeMap, HashMap};
use crate::pe::PeFile;

#[derive(Debug, Clone)]
pub struct RttiClassInfo {
    pub name: String,
    pub demangled_name: String,
    pub col_rva: u32,
    pub type_desc_rva: u32,
    pub base_classes: Vec<String>,
    pub vftable_rva: Option<u32>,
    pub virtual_methods: Vec<u64>,
}

pub struct RttiParser;

impl RttiParser {
    /// Demangle MSVC class name: `.?AVccGobjPc@@` -> `ccGobjPc`
    pub fn demangle_msvc_name(raw: &str) -> String {
        let trimmed = raw
            .trim_start_matches(".?AV")
            .trim_start_matches(".?AU")
            .trim_end_matches("@@");

        // Convert templates/namespaces if any
        trimmed.replace("@", "::")
    }

    /// Read null-terminated string at given offset in PE data
    fn read_cstring(data: &[u8], offset: usize) -> Option<String> {
        if offset >= data.len() {
            return None;
        }
        let end = data[offset..].iter().position(|&b| b == 0)?;
        String::from_utf8(data[offset..offset + end].to_vec()).ok()
    }

    /// Parse complete RTTI class database from PE
    pub fn parse_classes(pe: &PeFile) -> BTreeMap<String, RttiClassInfo> {
        let mut classes = BTreeMap::new();

        // 1. Find .text section bounds for vtable method validation
        let (text_rva_start, text_rva_end) = if let Some((sec, _)) = pe.get_section(".text") {
            (sec.virtual_address as u64 + pe.image_base, (sec.virtual_address + sec.virtual_size) as u64 + pe.image_base)
        } else {
            (0, 0)
        };

        // 2. Locate .rdata section containing RTTI structures
        let (rdata_sec, rdata) = match pe.get_section(".rdata") {
            Some(s) => s,
            None => return classes,
        };

        let rdata_start_rva = rdata_sec.virtual_address;

        // Map from col_rva -> (vftable_rva, Vec<method_va>)
        // We scan .rdata for 64-bit pointers that point to COL structures
        let mut col_to_vftable: HashMap<u32, (u32, Vec<u64>)> = HashMap::new();

        // 3. Scan for Complete Object Locators in .rdata
        // In MSVC x64:
        // struct RTTICompleteObjectLocator {
        //     uint32_t signature; // 1 for x64
        //     uint32_t offset;    // 0 for main vtable
        //     uint32_t cdOffset;
        //     int32_t  pTypeDescriptor; // RVA
        //     int32_t  pClassDescriptor; // RVA
        //     int32_t  pSelf; // RVA of this COL
        // }
        for offset in (0..rdata.len().saturating_sub(24)).step_by(4) {
            let signature = u32::from_le_bytes(rdata[offset..offset + 4].try_into().unwrap());
            if signature != 1 {
                continue;
            }

            let col_rva = rdata_start_rva + offset as u32;
            let p_self = u32::from_le_bytes(rdata[offset + 20..offset + 24].try_into().unwrap());
            if p_self != col_rva {
                continue;
            }

            let this_offset = u32::from_le_bytes(rdata[offset + 4..offset + 8].try_into().unwrap());
            if this_offset != 0 {
                // Secondary vtable from multiple inheritance, skip to keep primary
                continue;
            }
            let type_desc_rva = u32::from_le_bytes(rdata[offset + 12..offset + 16].try_into().unwrap());
            let class_desc_rva = u32::from_le_bytes(rdata[offset + 16..offset + 20].try_into().unwrap());

            // Read TypeDescriptor name: offset 0x10 in TypeDescriptor
            let type_desc_off = match pe.rva_to_offset(type_desc_rva) {
                Some(off) => off,
                None => continue,
            };

            let raw_name = match Self::read_cstring(&pe.data, type_desc_off + 16) {
                Some(n) if n.starts_with(".?A") => n,
                _ => continue,
            };

            let demangled = Self::demangle_msvc_name(&raw_name);

            // Read Class Hierarchy Descriptor and Base Class Array
            let mut base_classes = Vec::new();
            if let Some(ch_off) = pe.rva_to_offset(class_desc_rva) {
                if ch_off + 16 <= pe.data.len() {
                    let num_bases = u32::from_le_bytes(pe.data[ch_off + 8..ch_off + 12].try_into().unwrap()) as usize;
                    let base_array_rva = u32::from_le_bytes(pe.data[ch_off + 12..ch_off + 16].try_into().unwrap());

                    if let Some(b_array_off) = pe.rva_to_offset(base_array_rva) {
                        for b_idx in 0..num_bases {
                            let entry_off = b_array_off + b_idx * 4;
                            if entry_off + 4 <= pe.data.len() {
                                let base_desc_rva = u32::from_le_bytes(pe.data[entry_off..entry_off + 4].try_into().unwrap());
                                if let Some(bd_off) = pe.rva_to_offset(base_desc_rva) {
                                    if bd_off + 4 <= pe.data.len() {
                                        let base_td_rva = u32::from_le_bytes(pe.data[bd_off..bd_off + 4].try_into().unwrap());
                                        if let Some(base_td_off) = pe.rva_to_offset(base_td_rva) {
                                            if let Some(base_name) = Self::read_cstring(&pe.data, base_td_off + 16) {
                                                let base_demangled = Self::demangle_msvc_name(&base_name);
                                                if base_demangled != demangled {
                                                    base_classes.push(base_demangled);
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }

            // Record class
            classes.entry(demangled.clone()).or_insert_with(|| RttiClassInfo {
                name: raw_name,
                demangled_name: demangled,
                col_rva,
                type_desc_rva,
                base_classes,
                vftable_rva: None,
                virtual_methods: Vec::new(),
            });
        }

        // 4. Scan for vftables in .rdata pointing to COL pointers
        // A vftable is an array of 64-bit function pointers preceded by a 64-bit pointer to COL.
        for offset in (0..rdata.len().saturating_sub(16)).step_by(8) {
            let potential_col_va = u64::from_le_bytes(rdata[offset..offset + 8].try_into().unwrap());
            if potential_col_va >= pe.image_base {
                let potential_col_rva = (potential_col_va - pe.image_base) as u32;
                let vtable_rva = rdata_start_rva + (offset + 8) as u32;

                // Check if following entries are pointers into .text
                let mut methods = Vec::new();
                let mut cur_off = offset + 8;
                while cur_off + 8 <= rdata.len() {
                    let fn_va = u64::from_le_bytes(rdata[cur_off..cur_off + 8].try_into().unwrap());
                    if fn_va >= text_rva_start && fn_va < text_rva_end {
                        methods.push(fn_va);
                        cur_off += 8;
                    } else {
                        break;
                    }
                }

                if !methods.is_empty() {
                    col_to_vftable.insert(potential_col_rva, (vtable_rva, methods));
                }
            }
        }

        // 5. Connect vftables to class info
        for class_info in classes.values_mut() {
            if let Some((vtable_rva, methods)) = col_to_vftable.get(&class_info.col_rva) {
                class_info.vftable_rva = Some(*vtable_rva);
                class_info.virtual_methods = methods.clone();
            }
        }

        classes
    }
}

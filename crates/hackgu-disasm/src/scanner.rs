//! String reference and symbol scanner for binary decompilation.
//! Searches PE data sections for ASCII/UTF-8 strings (class names, function names, error strings, format strings)
//! and identifies code cross-references (xrefs).

use std::collections::BTreeMap;
use crate::pe::PeFile;

#[derive(Debug, Clone)]
pub struct StringRef {
    pub rva: u32,
    pub value: String,
    pub is_symbol_like: bool,
}

pub struct SymbolScanner;

impl SymbolScanner {
    /// Extract all meaningful ASCII / UTF-8 strings from `.rdata`, `.data`, and other sections.
    pub fn scan_strings(pe: &PeFile, min_len: usize) -> Vec<StringRef> {
        let mut results = Vec::new();

        for sec in &pe.sections {
            // Focus on readable data sections
            if sec.name == ".rdata" || sec.name == ".data" || sec.name == ".rodata" {
                let start = sec.pointer_to_raw_data as usize;
                let end = start + sec.size_of_raw_data as usize;
                if end > pe.data.len() {
                    continue;
                }
                let data = &pe.data[start..end];

                let mut cur = 0;
                while cur < data.len() {
                    // Look for printable ASCII sequence
                    if data[cur].is_ascii_graphic() || data[cur] == b' ' {
                        let str_start = cur;
                        while cur < data.len() && (data[cur].is_ascii_graphic() || data[cur] == b' ' || data[cur] == b'\t') {
                            cur += 1;
                        }

                        // Check if null-terminated
                        if cur < data.len() && data[cur] == 0 {
                            let len = cur - str_start;
                            if len >= min_len {
                                let s = String::from_utf8_lossy(&data[str_start..cur]).into_owned();
                                let rva = sec.virtual_address + str_start as u32;

                                let is_symbol_like = s.starts_with("C") 
                                    || s.starts_with("c_")
                                    || s.contains("::")
                                    || s.contains("Get")
                                    || s.contains("Set")
                                    || s.contains("Init")
                                    || s.contains("Update")
                                    || s.contains("Render")
                                    || s.contains("CCS")
                                    || s.contains("Clump")
                                    || s.contains("Anime")
                                    || s.contains("Player")
                                    || s.contains("Camera")
                                    || s.contains("Battle")
                                    || s.contains("Stage")
                                    || s.contains("Collision")
                                    || s.ends_with(".cpp")
                                    || s.ends_with(".h");

                                results.push(StringRef {
                                    rva,
                                    value: s,
                                    is_symbol_like,
                                });
                            }
                        }
                    }
                    cur += 1;
                }
            }
        }

        results
    }

    /// Identify RTTI (Run-Time Type Information) type descriptors (e.g. `.?AVClassName@@`).
    pub fn scan_rtti_classes(pe: &PeFile) -> BTreeMap<u32, String> {
        let mut classes = BTreeMap::new();
        let strings = Self::scan_strings(pe, 4);

        for s in strings {
            if s.value.starts_with(".?AV") || s.value.starts_with(".?AU") {
                // MSVC mangled class name: .?AVClassName@@ -> ClassName
                let name = s.value
                    .trim_start_matches(".?AV")
                    .trim_start_matches(".?AU")
                    .trim_end_matches("@@");
                classes.insert(s.rva, name.to_string());
            }
        }

        classes
    }
}

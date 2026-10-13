use std::collections::{BTreeMap, HashSet};
use std::fs::File;
use std::io::{BufWriter, Write};
use std::panic::{catch_unwind, set_hook, take_hook, AssertUnwindSafe};
use std::path::{Path, PathBuf};

use hackgu_ccs::objects::{SECTION_CLUT, SECTION_MODEL, SECTION_STREAM, SECTION_TEXTURE};
use hackgu_ccs::{CcsFile, DecodedModel, DecodedTexture};

#[derive(Clone, Debug)]
struct AuditRow {
    path: String,
    file_len: u64,
    parse_ok: String,
    parsed_end_offset: usize,
    reached_stream: bool,
    ends_at_stream_marker: bool,
    object_count: usize,
    texture_count: usize,
    texture_decode_fail_count: usize,
    mesh_count: usize,
    meshes_with_missing_texture: usize,
    model_decode_fail_count: usize,
}

fn csv_field(s: &str) -> String {
    if s.contains(',') || s.contains('"') || s.contains('\n') || s.contains('\r') {
        format!("\"{}\"", s.replace('"', "\"\""))
    } else {
        s.to_string()
    }
}

fn collect_ccs_files(dir: &Path, out: &mut Vec<PathBuf>) {
    if dir.is_file() {
        if let Some(ext) = dir.extension().and_then(|e| e.to_str()) {
            if ext.eq_ignore_ascii_case("ccs") {
                out.push(dir.to_path_buf());
            }
        }
        return;
    }

    if let Ok(entries) = std::fs::read_dir(dir) {
        for entry in entries.flatten() {
            let path = entry.path();
            if path.is_dir() {
                collect_ccs_files(&path, out);
            } else if path.is_file() {
                if let Some(ext) = path.extension().and_then(|e| e.to_str()) {
                    if ext.eq_ignore_ascii_case("ccs") {
                        out.push(path);
                    }
                }
            }
        }
    }
}

fn check_stream_end_marker(data: &[u8], parsed_end_offset: usize) -> bool {
    if parsed_end_offset == data.len() {
        return true;
    }
    if parsed_end_offset + 4 > data.len() {
        return false;
    }
    let tag = u32::from_le_bytes(data[parsed_end_offset..parsed_end_offset + 4].try_into().unwrap()) & 0xffff;
    if tag != SECTION_STREAM {
        return false;
    }
    // Check if followed by 24-byte marker
    if parsed_end_offset + 24 <= data.len() {
        // If it reaches EOF exactly at 24 bytes, or has stream marker structure
        if parsed_end_offset + 24 == data.len() {
            return true;
        }
        let sz = u32::from_le_bytes(data[parsed_end_offset + 4..parsed_end_offset + 8].try_into().unwrap());
        if sz == 1 && &data[parsed_end_offset + 20..parsed_end_offset + 24] == &[0xff, 0xff, 0xff, 0xff] {
            // Marker matches standard 24-byte pattern
            return true;
        }
    }
    false
}

fn main() {
    let args: Vec<String> = std::env::args().collect();

    let mut dirs: Vec<PathBuf> = Vec::new();
    let mut csv_path = PathBuf::from("gemini_out/audit_v3.csv");

    let mut i = 1;
    while i < args.len() {
        if args[i] == "--csv" {
            if i + 1 < args.len() {
                csv_path = PathBuf::from(&args[i + 1]);
                i += 2;
            } else {
                eprintln!("Error: --csv requires a path argument");
                std::process::exit(1);
            }
        } else if !args[i].starts_with('-') {
            dirs.push(PathBuf::from(&args[i]));
            i += 1;
        } else {
            i += 1;
        }
    }

    if dirs.is_empty() {
        dirs.push(PathBuf::from(r"target\vol1"));
        dirs.push(PathBuf::from(r"target\cmn"));
        dirs.push(PathBuf::from(r"target\hackGU_vol2_a"));
    }

    let mut ccs_files = Vec::new();
    for d in &dirs {
        collect_ccs_files(d, &mut ccs_files);
    }
    ccs_files.sort();

    // Suppress panic prints during auditing to gracefully handle malformed files
    let prev_hook = take_hook();
    set_hook(Box::new(|_| {}));

    let mut results: Vec<AuditRow> = Vec::with_capacity(ccs_files.len());
    let mut total_distinct_texture_errors: BTreeMap<String, usize> = BTreeMap::new();

    for path in &ccs_files {
        let path_str = path.to_string_lossy().to_string();
        let file_len = match std::fs::metadata(path) {
            Ok(meta) => meta.len(),
            Err(_) => 0,
        };

        let data = match std::fs::read(path) {
            Ok(d) => d,
            Err(e) => {
                results.push(AuditRow {
                    path: path_str,
                    file_len,
                    parse_ok: format!("false (IO error: {})", e),
                    parsed_end_offset: 0,
                    reached_stream: false,
                    ends_at_stream_marker: false,
                    object_count: 0,
                    texture_count: 0,
                    texture_decode_fail_count: 0,
                    mesh_count: 0,
                    meshes_with_missing_texture: 0,
                    model_decode_fail_count: 0,
                });
                continue;
            }
        };

        let parse_result = catch_unwind(AssertUnwindSafe(|| CcsFile::parse(&data)));

        let ccs = match parse_result {
            Ok(Ok(c)) => c,
            Ok(Err(e)) => {
                results.push(AuditRow {
                    path: path_str,
                    file_len,
                    parse_ok: format!("false ({})", e),
                    parsed_end_offset: 0,
                    reached_stream: false,
                    ends_at_stream_marker: false,
                    object_count: 0,
                    texture_count: 0,
                    texture_decode_fail_count: 0,
                    mesh_count: 0,
                    meshes_with_missing_texture: 0,
                    model_decode_fail_count: 0,
                });
                continue;
            }
            Err(_) => {
                results.push(AuditRow {
                    path: path_str,
                    file_len,
                    parse_ok: "false (panic during parse)".to_string(),
                    parsed_end_offset: 0,
                    reached_stream: false,
                    ends_at_stream_marker: false,
                    object_count: 0,
                    texture_count: 0,
                    texture_decode_fail_count: 0,
                    mesh_count: 0,
                    meshes_with_missing_texture: 0,
                    model_decode_fail_count: 0,
                });
                continue;
            }
        };

        let parsed_end_offset = ccs
            .raw_objects
            .last()
            .map(|obj| obj.offset + 12 + obj.size)
            .unwrap_or(0);

        let reached_stream = if parsed_end_offset + 4 <= data.len() {
            let tag = u32::from_le_bytes(data[parsed_end_offset..parsed_end_offset + 4].try_into().unwrap());
            (tag & 0xffff) == SECTION_STREAM
        } else {
            false
        };

        let ends_at_stream_marker = check_stream_end_marker(&data, parsed_end_offset);

        let object_count = ccs.objects.len();

        // 1. Textures
        let mut decoded_textures: HashSet<u32> = HashSet::new();
        let mut texture_count = 0;
        let mut texture_decode_fail_count = 0;

        for tex_obj in ccs.raw_objects.iter().filter(|o| o.section_type == SECTION_TEXTURE) {
            texture_count += 1;
            let name = ccs.object_name(tex_obj.object_id).unwrap_or("texture");
            let clut_raw = if tex_obj.payload.len() >= 4 {
                let clut_id = i32::from_le_bytes(tex_obj.payload[0..4].try_into().unwrap());
                if clut_id > 0 {
                    ccs.raw_objects
                        .iter()
                        .find(|o| o.object_id == clut_id as u32 && o.section_type == SECTION_CLUT)
                        .and_then(|o| if o.payload.len() >= 16 { Some(&o.payload[16..]) } else { None })
                } else {
                    None
                }
            } else {
                None
            };

            let tex_res = catch_unwind(AssertUnwindSafe(|| {
                DecodedTexture::from_payload_with_clut(name, &tex_obj.payload, clut_raw)
            }));

            match tex_res {
                Ok(Ok(_)) => {
                    decoded_textures.insert(tex_obj.object_id);
                }
                Ok(Err(e)) => {
                    texture_decode_fail_count += 1;
                    let err_str = e.to_string();
                    *total_distinct_texture_errors.entry(err_str).or_default() += 1;
                }
                Err(_) => {
                    texture_decode_fail_count += 1;
                    let err_str = "panic during texture decoding".to_string();
                    *total_distinct_texture_errors.entry(err_str).or_default() += 1;
                }
            }
        }

        // 2. Materials
        let materials = ccs.materials();

        // 3. Models
        let mut mesh_count = 0;
        let mut meshes_with_missing_texture = 0;
        let mut model_decode_fail_count = 0;

        for model_obj in ccs.raw_objects.iter().filter(|o| o.section_type == SECTION_MODEL) {
            let name = ccs.object_name(model_obj.object_id).unwrap_or("model");
            let model_res = catch_unwind(AssertUnwindSafe(|| {
                DecodedModel::from_payload(name, &model_obj.payload, ccs.header.raw_version)
            }));

            match model_res {
                Ok(Ok(model)) => {
                    for mesh in &model.meshes {
                        mesh_count += 1;
                        let has_valid_texture = match materials.get(&mesh.material_index) {
                            Some(mat) => decoded_textures.contains(&mat.texture_obj_id),
                            None => false,
                        };

                        if !has_valid_texture {
                            meshes_with_missing_texture += 1;
                        }
                    }
                }
                Ok(Err(_)) => {
                    model_decode_fail_count += 1;
                }
                Err(_) => {
                    model_decode_fail_count += 1;
                }
            }
        }

        results.push(AuditRow {
            path: path_str,
            file_len,
            parse_ok: "true".to_string(),
            parsed_end_offset,
            reached_stream,
            ends_at_stream_marker,
            object_count,
            texture_count,
            texture_decode_fail_count,
            mesh_count,
            meshes_with_missing_texture,
            model_decode_fail_count,
        });
    }

    set_hook(prev_hook);

    if let Some(parent) = csv_path.parent() {
        if !parent.as_os_str().is_empty() {
            let _ = std::fs::create_dir_all(parent);
        }
    }

    let csv_file = match File::create(&csv_path) {
        Ok(f) => f,
        Err(e) => {
            eprintln!("Failed to create CSV file at {}: {}", csv_path.display(), e);
            std::process::exit(1);
        }
    };

    let mut writer = BufWriter::new(csv_file);
    let header = "path,file_len,parse_ok,parsed_end_offset,reached_stream,ends_at_stream_marker,object_count,texture_count,texture_decode_fail_count,mesh_count,meshes_with_missing_texture,model_decode_fail_count\n";
    if let Err(e) = writer.write_all(header.as_bytes()) {
        eprintln!("Failed to write CSV header: {}", e);
        std::process::exit(1);
    }

    for row in &results {
        let line = format!(
            "{},{},{},{},{},{},{},{},{},{},{},{}\n",
            csv_field(&row.path),
            row.file_len,
            csv_field(&row.parse_ok),
            row.parsed_end_offset,
            row.reached_stream,
            row.ends_at_stream_marker,
            row.object_count,
            row.texture_count,
            row.texture_decode_fail_count,
            row.mesh_count,
            row.meshes_with_missing_texture,
            row.model_decode_fail_count,
        );
        let _ = writer.write_all(line.as_bytes());
    }
    let _ = writer.flush();

    let total_files = results.len();
    let reached_stream_false_count = results.iter().filter(|r| !r.reached_stream).count();
    let ends_at_stream_marker_false: Vec<_> = results.iter().filter(|r| !r.ends_at_stream_marker).collect();

    println!("================================================================================");
    println!("TASK 0: AUDIT V3 SUMMARY");
    println!("================================================================================");
    println!("Total files scanned:                   {}", total_files);
    println!("Files with reached_stream == false:    {}", reached_stream_false_count);
    println!("Files with ends_at_stream_marker == false: {}", ends_at_stream_marker_false.len());
    println!("Output written to:                     {}", csv_path.display());
    println!();
    println!("Files where ends_at_stream_marker == false ({} files):", ends_at_stream_marker_false.len());
    for r in &ends_at_stream_marker_false {
        println!("  {} (len: {}, end_offset: {}, reached_stream: {})", r.path, r.file_len, r.parsed_end_offset, r.reached_stream);
    }
    println!("================================================================================");
}

use std::collections::{BTreeMap, BTreeSet, HashSet};
use std::fs::File;
use std::io::{BufWriter, Write};
use std::panic::{catch_unwind, set_hook, take_hook, AssertUnwindSafe};
use std::path::{Path, PathBuf};

use hackgu_ccs::objects::{
    section_name, SECTION_ANIME, SECTION_DUMMYPOS, SECTION_DUMMYPOSROT, SECTION_HITMESH,
    SECTION_MODEL, SECTION_TEXTURE,
};
use hackgu_ccs::{CcsFile, HitMesh};

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

/// Extract locator prefix matching user rules:
/// "the text before the first digit or the second underscore, e.g. DMY_pc, DMY_ev, DMY_town01_, DMY_warp..."
fn get_locator_prefix(name: &str) -> String {
    if name.is_empty() {
        return "unknown".to_string();
    }
    let underscores: Vec<usize> = name.match_indices('_').map(|(i, _)| i).collect();
    if underscores.len() >= 2 {
        let second_us = underscores[1];
        let part1 = &name[underscores[0] + 1..second_us];
        if part1.chars().any(|c| c.is_ascii_digit()) {
            name[..=second_us].to_string()
        } else {
            name[..second_us].to_string()
        }
    } else {
        if let Some(pos) = name.find(|c: char| c.is_ascii_digit()) {
            name[..pos].to_string()
        } else {
            name.to_string()
        }
    }
}

struct LocatorRow {
    file: String,
    object_id: u32,
    section_type_hex: String,
    name: String,
    payload_len: usize,
    x_cm: f32,
    y_cm: f32,
    z_cm: f32,
    rot: Vec<f32>,
}

struct HitMeshRow {
    file: String,
    object_id: u32,
    object_name: String,
    header_unknown: String,
    record_index: usize,
    flags_hex: String,
    vertex_count: usize,
    triangle_count: usize,
    min_x: f32,
    min_y: f32,
    min_z: f32,
    max_x: f32,
    max_y: f32,
    max_z: f32,
    avg_normal_z: f32,
    kind: String,
}

#[derive(Default)]
struct HitMeshFlagStat {
    count: usize,
    triangles: usize,
    floor_count: usize,
    wall_count: usize,
    ceiling_count: usize,
    seen_files: BTreeSet<String>,
}

struct StageRow {
    file: String,
    size_bytes: u64,
    object_counts: String,
    has_hitmesh: bool,
    hitmesh_triangles: usize,
    locator_count: usize,
    texture_count: usize,
    model_count: usize,
    anim_count: usize,
    special_locators: String,
}

#[derive(Default)]
struct UnknownSectionStat {
    count: usize,
    files: BTreeSet<String>,
    min_len: usize,
    max_len: usize,
    sample_hex: String,
}

fn is_sjis_lead(b: u8) -> bool {
    (0x81..=0x9f).contains(&b) || (0xe0..=0xfc).contains(&b)
}

fn is_sjis_trail(b: u8) -> bool {
    (0x40..=0x7e).contains(&b) || (0x80..=0xfc).contains(&b)
}

fn extract_text_runs(payload: &[u8], max_runs: usize) -> Vec<(usize, String)> {
    let mut hits = Vec::new();
    let mut i = 0;
    while i < payload.len() && hits.len() < max_runs {
        let start = i;
        let mut ascii_letters = 0;
        let mut sjis_count = 0;

        while i < payload.len() {
            let b = payload[i];
            if (0x20..=0x7e).contains(&b) || b == b'\t' || b == b'\r' || b == b'\n' {
                if (b'a'..=b'z').contains(&b) || (b'A'..=b'Z').contains(&b) {
                    ascii_letters += 1;
                }
                i += 1;
            } else if is_sjis_lead(b) && i + 1 < payload.len() && is_sjis_trail(payload[i + 1]) {
                sjis_count += 1;
                i += 2;
            } else if (0xa1..=0xdf).contains(&b) {
                sjis_count += 1;
                i += 1;
            } else {
                break;
            }
        }

        let total_bytes = i - start;
        // Require >= 8 consecutive chars or >= 2 SJIS chars, with meaningful text content
        if (total_bytes >= 8 && ascii_letters >= 3) || (sjis_count >= 2) {
            let slice = &payload[start..i];
            let mut decoded = String::new();
            let mut k = 0;
            while k < slice.len() && decoded.chars().count() < 80 {
                let b = slice[k];
                if (0x20..=0x7e).contains(&b) {
                    decoded.push(b as char);
                    k += 1;
                } else if b == b'\n' {
                    decoded.push_str("\\n");
                    k += 1;
                } else if b == b'\r' {
                    decoded.push_str("\\r");
                    k += 1;
                } else if b == b'\t' {
                    decoded.push_str("\\t");
                    k += 1;
                } else if is_sjis_lead(b) && k + 1 < slice.len() && is_sjis_trail(slice[k + 1]) {
                    decoded.push_str(&format!("[SJIS:{:02X}{:02X}]", b, slice[k + 1]));
                    k += 2;
                } else {
                    decoded.push('.');
                    k += 1;
                }
            }

            hits.push((start, decoded));
        }

        if i == start {
            i += 1;
        }
    }
    hits
}

fn is_stage_file(path_str: &str, file_name: &str) -> bool {
    let lower_name = file_name.to_lowercase();
    let lower_path = path_str.to_lowercase();

    lower_path.contains(r"\stage\")
        || lower_path.contains(r"\town\")
        || lower_path.contains(r"\dungeon\")
        || lower_path.contains(r"\field\")
        || lower_path.contains(r"\ev\")
        || lower_name.starts_with("town")
        || lower_name.starts_with("sf")
        || lower_name.starts_with("st")
        || lower_name.starts_with("se")
        || lower_name.starts_with("ev")
        || lower_name.starts_with("dungeon")
        || lower_name.starts_with("field")
        || lower_name.starts_with("ava")
        || lower_name.starts_with("epc")
        || lower_name.starts_with("estr")
        || lower_name.starts_with("m1")
        || lower_name.starts_with("m2")
        || lower_name.starts_with("m3")
}

fn main() {
    let dirs = vec![
        PathBuf::from(r"target\vol1"),
        PathBuf::from(r"target\cmn"),
        PathBuf::from(r"target\hackGU_vol2_a"),
    ];

    let out_dir = PathBuf::from("gemini_out");
    let _ = std::fs::create_dir_all(&out_dir);

    let mut ccs_files = Vec::new();
    for d in &dirs {
        collect_ccs_files(d, &mut ccs_files);
    }
    ccs_files.sort();

    println!("Starting analysis across {} CCS files...", ccs_files.len());

    let prev_hook = take_hook();
    set_hook(Box::new(|_| {}));

    // Setup streaming writer for text_hits.csv
    let text_hits_path = out_dir.join("text_hits.csv");
    let mut text_hits_writer = BufWriter::new(File::create(&text_hits_path).unwrap());
    text_hits_writer
        .write_all(b"file,object_id,section_type_hex,object_name,offset_in_payload,first_80_chars\n")
        .unwrap();
    let mut total_text_hits = 0usize;

    // TASK 1 storage
    let mut locators: Vec<LocatorRow> = Vec::new();
    let mut locator_prefixes: BTreeMap<String, (usize, BTreeSet<String>)> = BTreeMap::new();
    let mut max_rot_count = 0usize;

    // TASK 2 storage
    let mut hitmesh_rows: Vec<HitMeshRow> = Vec::new();
    let mut hitmesh_flags_hist: BTreeMap<String, HitMeshFlagStat> = BTreeMap::new();

    // TASK 3 storage
    let mut stage_rows: Vec<StageRow> = Vec::new();

    // TASK 4 storage
    let known_types: HashSet<u32> = [
        0x100, 0x200, 0x300, 0x400, 0x700, 0x800, 0x900, 0xA00, 0xB00, 0x1300, 0x1400, 0x2000, 0x2400,
    ]
    .iter()
    .cloned()
    .collect();
    let mut unknown_sections: BTreeMap<u32, UnknownSectionStat> = BTreeMap::new();

    let mut parse_failed_files: Vec<(String, String)> = Vec::new();

    for path in &ccs_files {
        let path_str = path.to_string_lossy().to_string();
        let file_name = path.file_stem().and_then(|s| s.to_str()).unwrap_or("");
        let file_len = match std::fs::metadata(path) {
            Ok(m) => m.len(),
            Err(_) => 0,
        };

        let data = match std::fs::read(path) {
            Ok(d) => d,
            Err(e) => {
                parse_failed_files.push((path_str, format!("IO error: {}", e)));
                continue;
            }
        };

        let ccs_res = catch_unwind(AssertUnwindSafe(|| CcsFile::parse(&data)));
        let ccs = match ccs_res {
            Ok(Ok(c)) => c,
            Ok(Err(e)) => {
                parse_failed_files.push((path_str, format!("Parse error: {}", e)));
                continue;
            }
            Err(_) => {
                parse_failed_files.push((path_str, "Panic during parse".to_string()));
                continue;
            }
        };

        let mut counts_by_type: BTreeMap<u32, usize> = BTreeMap::new();
        let mut hitmesh_triangles_in_file = 0usize;
        let mut locators_in_file = 0usize;
        let mut textures_in_file = 0usize;
        let mut models_in_file = 0usize;
        let mut anims_in_file = 0usize;
        let mut has_hitmesh_in_file = false;
        let mut special_dmy_in_file = Vec::new();

        for o in &ccs.raw_objects {
            *counts_by_type.entry(o.section_type).or_default() += 1;

            // TASK 1: Locators (0x1300 DummyPos, 0x1400 DummyPosRot)
            if o.section_type == SECTION_DUMMYPOS || o.section_type == SECTION_DUMMYPOSROT {
                locators_in_file += 1;
                let name = ccs.object_name(o.object_id).unwrap_or("unknown").to_string();
                let f: Vec<f32> = o
                    .payload
                    .chunks_exact(4)
                    .map(|b| f32::from_le_bytes(b.try_into().unwrap()))
                    .collect();

                let x_cm = if f.len() > 0 { f[0] } else { 0.0 };
                let y_cm = if f.len() > 1 { f[1] } else { 0.0 };
                let z_cm = if f.len() > 2 { f[2] } else { 0.0 };

                let rot = if o.section_type == SECTION_DUMMYPOSROT && f.len() > 3 {
                    f[3..].to_vec()
                } else {
                    Vec::new()
                };

                if rot.len() > max_rot_count {
                    max_rot_count = rot.len();
                }

                let prefix = get_locator_prefix(&name);
                let entry = locator_prefixes.entry(prefix).or_insert((0, BTreeSet::new()));
                entry.0 += 1;
                entry.1.insert(path_str.clone());

                let lower_name = name.to_lowercase();
                if lower_name.starts_with("dmy_ev")
                    || lower_name.starts_with("dmy_door")
                    || lower_name.starts_with("dmy_warp")
                    || lower_name.starts_with("dmy_pc")
                    || lower_name.starts_with("dmy_safety")
                    || lower_name.starts_with("dmy_sp")
                    || lower_name.starts_with("dmy_tim")
                    || lower_name.starts_with("dmy_marker")
                {
                    special_dmy_in_file.push(name.clone());
                }

                locators.push(LocatorRow {
                    file: path_str.clone(),
                    object_id: o.object_id,
                    section_type_hex: format!("0x{:04X}", o.section_type),
                    name,
                    payload_len: o.payload.len(),
                    x_cm,
                    y_cm,
                    z_cm,
                    rot,
                });
            }

            // TASK 2: HitMesh (0x0B00)
            if o.section_type == SECTION_HITMESH {
                has_hitmesh_in_file = true;
                let obj_name = ccs.object_name(o.object_id).unwrap_or("unknown").to_string();
                if let Some(hm) = HitMesh::from_payload(o.object_id, &o.payload) {
                    for (rec_idx, rec) in hm.records.iter().enumerate() {
                        let v_count = rec.positions.len();
                        let tri_count = v_count / 3;
                        hitmesh_triangles_in_file += tri_count;

                        let mut min_x = f32::INFINITY;
                        let mut min_y = f32::INFINITY;
                        let mut min_z = f32::INFINITY;
                        let mut max_x = f32::NEG_INFINITY;
                        let mut max_y = f32::NEG_INFINITY;
                        let mut max_z = f32::NEG_INFINITY;

                        for p in &rec.positions {
                            min_x = min_x.min(p[0]);
                            min_y = min_y.min(p[1]);
                            min_z = min_z.min(p[2]);
                            max_x = max_x.max(p[0]);
                            max_y = max_y.max(p[1]);
                            max_z = max_z.max(p[2]);
                        }
                        if min_x.is_infinite() {
                            min_x = 0.0;
                            min_y = 0.0;
                            min_z = 0.0;
                            max_x = 0.0;
                            max_y = 0.0;
                            max_z = 0.0;
                        }

                        let avg_normal_z = if rec.normals.is_empty() {
                            0.0
                        } else {
                            rec.normals.iter().map(|n| n[2]).sum::<f32>() / rec.normals.len() as f32
                        };

                        let kind = if avg_normal_z > 0.7 {
                            "floor".to_string()
                        } else if avg_normal_z < -0.7 {
                            "ceiling".to_string()
                        } else {
                            "wall".to_string()
                        };

                        let flags_hex = format!("0x{:08X}", rec.flags);

                        let flag_stat = hitmesh_flags_hist.entry(flags_hex.clone()).or_default();
                        flag_stat.count += 1;
                        flag_stat.triangles += tri_count;
                        if kind == "floor" {
                            flag_stat.floor_count += 1;
                        } else if kind == "ceiling" {
                            flag_stat.ceiling_count += 1;
                        } else {
                            flag_stat.wall_count += 1;
                        }
                        if flag_stat.seen_files.len() < 5 {
                            flag_stat.seen_files.insert(path_str.clone());
                        }

                        hitmesh_rows.push(HitMeshRow {
                            file: path_str.clone(),
                            object_id: o.object_id,
                            object_name: obj_name.clone(),
                            header_unknown: hm.header_unknown.to_string(),
                            record_index: rec_idx,
                            flags_hex,
                            vertex_count: v_count,
                            triangle_count: tri_count,
                            min_x,
                            min_y,
                            min_z,
                            max_x,
                            max_y,
                            max_z,
                            avg_normal_z,
                            kind,
                        });
                    }
                } else {
                    hitmesh_rows.push(HitMeshRow {
                        file: path_str.clone(),
                        object_id: o.object_id,
                        object_name: obj_name,
                        header_unknown: "unknown".to_string(),
                        record_index: 0,
                        flags_hex: "unknown".to_string(),
                        vertex_count: 0,
                        triangle_count: 0,
                        min_x: 0.0,
                        min_y: 0.0,
                        min_z: 0.0,
                        max_x: 0.0,
                        max_y: 0.0,
                        max_z: 0.0,
                        avg_normal_z: 0.0,
                        kind: "unparsed".to_string(),
                    });
                }
            }

            if o.section_type == SECTION_TEXTURE {
                textures_in_file += 1;
            }
            if o.section_type == SECTION_MODEL {
                models_in_file += 1;
            }
            if o.section_type == SECTION_ANIME {
                anims_in_file += 1;
            }

            // TASK 4: Unparsed section types
            if !known_types.contains(&o.section_type) {
                let unk = unknown_sections.entry(o.section_type).or_default();
                unk.count += 1;
                if unk.files.len() < 5 {
                    unk.files.insert(path_str.clone());
                }
                if unk.min_len == 0 || o.payload.len() < unk.min_len {
                    unk.min_len = o.payload.len();
                }
                if o.payload.len() > unk.max_len {
                    unk.max_len = o.payload.len();
                }
                if unk.sample_hex.is_empty() {
                    let sample_bytes = &o.payload[..o.payload.len().min(64)];
                    unk.sample_hex = sample_bytes
                        .iter()
                        .map(|b| format!("{:02X}", b))
                        .collect::<Vec<_>>()
                        .join(" ");
                }
            }

            // TASK 5: Text / event data hunt (stream directly to file)
            // Especially scan 0x2400 (BinaryBlob); for others limit to 10 runs per object
            let max_runs = if o.section_type == 0x2400 { 50 } else { 10 };
            let runs = extract_text_runs(&o.payload, max_runs);
            if !runs.is_empty() {
                let obj_name = ccs.object_name(o.object_id).unwrap_or("unknown");
                for (off, text) in runs {
                    let line = format!(
                        "{},{},0x{:04X},{},{},{}\n",
                        csv_field(&path_str),
                        o.object_id,
                        o.section_type,
                        csv_field(obj_name),
                        off,
                        csv_field(&text)
                    );
                    let _ = text_hits_writer.write_all(line.as_bytes());
                    total_text_hits += 1;
                }
            }
        }

        // TASK 3: Stage Inventory
        if is_stage_file(&path_str, file_name) {
            let counts_str = counts_by_type
                .iter()
                .map(|(t, c)| format!("0x{:04X}:{}", t, c))
                .collect::<Vec<_>>()
                .join(";");

            stage_rows.push(StageRow {
                file: path_str,
                size_bytes: file_len,
                object_counts: counts_str,
                has_hitmesh: has_hitmesh_in_file,
                hitmesh_triangles: hitmesh_triangles_in_file,
                locator_count: locators_in_file,
                texture_count: textures_in_file,
                model_count: models_in_file,
                anim_count: anims_in_file,
                special_locators: special_dmy_in_file.join(";"),
            });
        }
    }

    set_hook(prev_hook);
    let _ = text_hits_writer.flush();
    println!("Wrote text_hits.csv ({} hits)", total_text_hits);

    // =========================================================================
    // WRITE OUTPUTS
    // =========================================================================

    // 1. locators.csv
    {
        let path = out_dir.join("locators.csv");
        let mut writer = BufWriter::new(File::create(&path).unwrap());
        let mut header = "file,object_id,section_type_hex,name,payload_len,x_cm,y_cm,z_cm".to_string();
        for r in 0..max_rot_count {
            header.push_str(&format!(",rot{}", r));
        }
        header.push('\n');
        writer.write_all(header.as_bytes()).unwrap();

        for l in &locators {
            let mut line = format!(
                "{},{},{},{},{},{:.4},{:.4},{:.4}",
                csv_field(&l.file),
                l.object_id,
                l.section_type_hex,
                csv_field(&l.name),
                l.payload_len,
                l.x_cm,
                l.y_cm,
                l.z_cm
            );
            for r in 0..max_rot_count {
                if let Some(val) = l.rot.get(r) {
                    line.push_str(&format!(",{:.4}", val));
                } else {
                    line.push(',');
                }
            }
            line.push('\n');
            writer.write_all(line.as_bytes()).unwrap();
        }
        writer.flush().unwrap();
        println!("Wrote locators.csv ({} rows)", locators.len());
    }

    // 2. locators_summary.csv
    {
        let path = out_dir.join("locators_summary.csv");
        let mut writer = BufWriter::new(File::create(&path).unwrap());
        writer.write_all(b"prefix,count,files\n").unwrap();

        for (prefix, (count, files)) in &locator_prefixes {
            let file_list = files.iter().cloned().collect::<Vec<_>>().join("; ");
            let line = format!("{},{},{}\n", csv_field(prefix), count, csv_field(&file_list));
            writer.write_all(line.as_bytes()).unwrap();
        }
        writer.flush().unwrap();
        println!("Wrote locators_summary.csv ({} prefixes)", locator_prefixes.len());
    }

    // 3. hitmesh.csv
    {
        let path = out_dir.join("hitmesh.csv");
        let mut writer = BufWriter::new(File::create(&path).unwrap());
        writer
            .write_all(b"file,object_id,object_name,header_unknown,record_index,flags_hex,vertex_count,triangle_count,min_x,min_y,min_z,max_x,max_y,max_z,avg_normal_z,kind\n")
            .unwrap();

        for h in &hitmesh_rows {
            let line = format!(
                "{},{},{},{},{},{},{},{},{:.2},{:.2},{:.2},{:.2},{:.2},{:.2},{:.4},{}\n",
                csv_field(&h.file),
                h.object_id,
                csv_field(&h.object_name),
                csv_field(&h.header_unknown),
                h.record_index,
                h.flags_hex,
                h.vertex_count,
                h.triangle_count,
                h.min_x,
                h.min_y,
                h.min_z,
                h.max_x,
                h.max_y,
                h.max_z,
                h.avg_normal_z,
                h.kind
            );
            writer.write_all(line.as_bytes()).unwrap();
        }
        writer.flush().unwrap();
        println!("Wrote hitmesh.csv ({} records)", hitmesh_rows.len());
    }

    // 4. hitmesh_flags.csv
    {
        let path = out_dir.join("hitmesh_flags.csv");
        let mut writer = BufWriter::new(File::create(&path).unwrap());
        writer
            .write_all(b"flags_hex,record_count,total_triangles,floor_count,wall_count,ceiling_count,seen_in_files\n")
            .unwrap();

        let mut sorted_flags: Vec<_> = hitmesh_flags_hist.iter().collect();
        sorted_flags.sort_by(|a, b| b.1.count.cmp(&a.1.count));

        for (flags_hex, stat) in sorted_flags {
            let seen = stat.seen_files.iter().cloned().collect::<Vec<_>>().join("; ");
            let line = format!(
                "{},{},{},{},{},{},{}\n",
                flags_hex,
                stat.count,
                stat.triangles,
                stat.floor_count,
                stat.wall_count,
                stat.ceiling_count,
                csv_field(&seen)
            );
            writer.write_all(line.as_bytes()).unwrap();
        }
        writer.flush().unwrap();
        println!("Wrote hitmesh_flags.csv ({} unique flag values)", hitmesh_flags_hist.len());
    }

    // 5. stages.csv & stage_naming_inference.txt
    {
        let path = out_dir.join("stages.csv");
        let mut writer = BufWriter::new(File::create(&path).unwrap());
        writer
            .write_all(b"file,size_bytes,object_counts_by_type,has_hitmesh,hitmesh_triangles,locator_count,texture_count,model_count,anim_count,special_dmy_locators\n")
            .unwrap();

        for s in &stage_rows {
            let line = format!(
                "{},{},{},{},{},{},{},{},{},{}\n",
                csv_field(&s.file),
                s.size_bytes,
                csv_field(&s.object_counts),
                s.has_hitmesh,
                s.hitmesh_triangles,
                s.locator_count,
                s.texture_count,
                s.model_count,
                s.anim_count,
                csv_field(&s.special_locators)
            );
            writer.write_all(line.as_bytes()).unwrap();
        }
        writer.flush().unwrap();
        println!("Wrote stages.csv ({} stage files)", stage_rows.len());

        let inf_path = out_dir.join("stage_naming_inference.txt");
        let inference_text = r#"STAGE / FIELD / EVENT NAMING SCHEME INFERENCE (.hack//G.U. Last Recode)
================================================================================
1. Hubs / Towns:
   - Prefix: town01*, town02*, town03*
   - Directory: data/data/town/ or data/data/stage/
   - Description: City stages with detailed environmental geometry, collision hitmeshes,
     NPC spawn markers, warp portals, and audio/effect locators.
     * town01: Mac Anu (root town with districts 00h-09h, maps, and props)
     * town02: Dol Dona
     * town03: Breg Epona

2. Dungeons & Quest Fields:
   - Prefixes: dungeon*, d01*, d02*, d03*, field*
   - Directories: data/data/dungeon/, data/data/field/, data/data/stage/
   - Description: Dungeon block geometry, collision meshes, item boxes, warp points,
     and puzzle door markers.

3. Special Battle Stages & Arenas:
   - Prefix: sf* (Special Field), st* (Stage), se* (Stage Event / Special Environment)
   - Directory: data/data/stage/
   - Description: Combat arenas, tournament rings, boss battle areas, and quest scenes.

4. Avatar Battles:
   - Prefix: ava_* (e.g. ava_sks, ava_ova, ava_hsw)
   - Directory: data/data/ava/ or data/data/ev/estr/
   - Description: High-altitude avatar combat spaces with streaming animation data.

5. Event Scenes & Cutscenes:
   - Directories: data/data/ev/, data/data/ev/epc/, data/data/ev/estr/
   - Prefixes: ev*, m1*, m2*, m3*, epc*, estr*
   - Description: Scripted sequence assets, character event models, camera animations,
     and dialogue trigger points.
"#;
        std::fs::write(&inf_path, inference_text).unwrap();
        println!("Wrote stage_naming_inference.txt");
    }

    // 6. unknown_sections.csv
    {
        let path = out_dir.join("unknown_sections.csv");
        let mut writer = BufWriter::new(File::create(&path).unwrap());
        writer
            .write_all(b"section_type_hex,section_name,count,files_containing,min_payload_len,max_payload_len,sample_hex_64\n")
            .unwrap();

        for (sec_type, stat) in &unknown_sections {
            let files_str = stat.files.iter().cloned().collect::<Vec<_>>().join("; ");
            let line = format!(
                "0x{:04X},{},{},{},{},{},{}\n",
                sec_type,
                section_name(*sec_type),
                stat.count,
                csv_field(&files_str),
                stat.min_len,
                stat.max_len,
                csv_field(&stat.sample_hex)
            );
            writer.write_all(line.as_bytes()).unwrap();
        }
        writer.flush().unwrap();
        println!("Wrote unknown_sections.csv ({} unknown section types)", unknown_sections.len());
    }

    // 7. Non-CCS game files scan (F:\SteamLibrary\steamapps\common\hackGU)
    {
        let game_install = Path::new(r"F:\SteamLibrary\steamapps\common\hackGU");
        let mut non_ccs_entries: BTreeMap<String, (usize, u64)> = BTreeMap::new();

        fn scan_dir(dir: &Path, hist: &mut BTreeMap<String, (usize, u64)>) {
            if let Ok(entries) = std::fs::read_dir(dir) {
                for e in entries.flatten() {
                    let p = e.path();
                    if p.is_dir() {
                        scan_dir(&p, hist);
                    } else if p.is_file() {
                        let ext = p
                            .extension()
                            .and_then(|x| x.to_str())
                            .map(|x| x.to_lowercase())
                            .unwrap_or_else(|| "<no_ext>".to_string());
                        let len = std::fs::metadata(&p).map(|m| m.len()).unwrap_or(0);
                        let entry = hist.entry(ext).or_default();
                        entry.0 += 1;
                        entry.1 += len;
                    }
                }
            }
        }

        if game_install.exists() {
            scan_dir(game_install, &mut non_ccs_entries);
        }

        let path = out_dir.join("non_ccs_files.csv");
        let mut writer = BufWriter::new(File::create(&path).unwrap());
        writer.write_all(b"extension,file_count,total_bytes,human_size\n").unwrap();

        for (ext, (count, bytes)) in &non_ccs_entries {
            let human = if *bytes >= 1024 * 1024 * 1024 {
                format!("{:.2} GiB", *bytes as f64 / (1024.0 * 1024.0 * 1024.0))
            } else if *bytes >= 1024 * 1024 {
                format!("{:.2} MiB", *bytes as f64 / (1024.0 * 1024.0))
            } else if *bytes >= 1024 {
                format!("{:.2} KiB", *bytes as f64 / 1024.0)
            } else {
                format!("{} B", bytes)
            };
            let line = format!("{},{},{},{}\n", ext, count, bytes, human);
            writer.write_all(line.as_bytes()).unwrap();
        }
        writer.flush().unwrap();
        println!("Wrote non_ccs_files.csv ({} unique extensions in game install)", non_ccs_entries.len());
    }

    if !parse_failed_files.is_empty() {
        println!("WARNING: {} files failed to parse:", parse_failed_files.len());
        for (f, err) in &parse_failed_files {
            println!("  {} -> {}", f, err);
        }
    } else {
        println!("SUCCESS: 0 parse failures across all {} files!", ccs_files.len());
    }
}

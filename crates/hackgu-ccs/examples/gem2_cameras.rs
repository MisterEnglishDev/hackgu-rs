use std::collections::{BTreeMap, HashSet};
use std::env;
use std::fs;
use std::io::Write;
use std::path::{Path, PathBuf};

use hackgu_ccs::{section_name, CcsFile};

struct CamRecord {
    file: String,
    object_id: u32,
    object_name: String,
    section_type_hex: String,
    section_name: String,
    payload_len: usize,
    decoded_fields: String,
}

fn find_all_ccs(dir: &Path, list: &mut Vec<PathBuf>) {
    if let Ok(entries) = fs::read_dir(dir) {
        for entry in entries.flatten() {
            let path = entry.path();
            if path.is_dir() {
                find_all_ccs(&path, list);
            } else if path.extension().and_then(|s| s.to_str()) == Some("ccs") {
                list.push(path);
            }
        }
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 3 {
        eprintln!("Usage: {} <target_dir> <out_cameras.csv>", args[0]);
        std::process::exit(1);
    }

    let target_dir = Path::new(&args[1]);
    let out_csv_path = Path::new(&args[2]);

    println!("Scanning {} for .ccs files...", target_dir.display());
    let mut files = Vec::new();
    find_all_ccs(target_dir, &mut files);
    println!("Found {} CCS files.", files.len());

    let mut records: Vec<CamRecord> = Vec::new();
    let mut section_counts: BTreeMap<u32, (usize, HashSet<String>)> = BTreeMap::new();
    let mut files_with_cam = HashSet::new();

    for fpath in &files {
        let fpath_str = fpath.to_string_lossy().to_string();
        let bytes = match fs::read(fpath) {
            Ok(b) => b,
            Err(_) => continue,
        };

        let ccs = match std::panic::catch_unwind(|| CcsFile::parse(&bytes)) {
            Ok(Ok(c)) => c,
            _ => continue,
        };

        for raw in &ccs.raw_objects {
            let id = raw.object_id;
            let name = ccs.object_name(id).unwrap_or("unknown");
            let is_sec_0500 = raw.section_type == 0x0500;
            let name_lower = name.to_lowercase();
            let is_cam_name = name_lower.contains("cam") || name_lower.contains("camera");

            if is_sec_0500 || is_cam_name {
                files_with_cam.insert(fpath_str.clone());
                let entry = section_counts.entry(raw.section_type).or_insert((0, HashSet::new()));
                entry.0 += 1;
                entry.1.insert(fpath_str.clone());

                let sec_name = section_name(raw.section_type);
                let sec_hex = format!("0x{:04X}", raw.section_type);

                let decoded = match raw.section_type {
                    0x0500 => "Camera node in hierarchy (0-byte payload)".to_string(),
                    0x1300 if raw.payload.len() >= 12 => {
                        let x = f32::from_le_bytes(raw.payload[0..4].try_into().unwrap());
                        let y = f32::from_le_bytes(raw.payload[4..8].try_into().unwrap());
                        let z = f32::from_le_bytes(raw.payload[8..12].try_into().unwrap());
                        format!("pos=[{:.2}, {:.2}, {:.2}] cm", x, y, z)
                    }
                    0x1400 if raw.payload.len() >= 24 => {
                        let x = f32::from_le_bytes(raw.payload[0..4].try_into().unwrap());
                        let y = f32::from_le_bytes(raw.payload[4..8].try_into().unwrap());
                        let z = f32::from_le_bytes(raw.payload[8..12].try_into().unwrap());
                        let r0 = f32::from_le_bytes(raw.payload[12..16].try_into().unwrap());
                        let r1 = f32::from_le_bytes(raw.payload[16..20].try_into().unwrap());
                        let r2 = f32::from_le_bytes(raw.payload[20..24].try_into().unwrap());
                        format!("pos=[{:.2}, {:.2}, {:.2}] cm, rot=[{:.2}, {:.2}, {:.2}]", x, y, z, r0, r1, r2)
                    }
                    _ => format!("payload_bytes={}", raw.payload.len()),
                };

                records.push(CamRecord {
                    file: fpath_str.clone(),
                    object_id: id,
                    object_name: name.to_string(),
                    section_type_hex: sec_hex,
                    section_name: sec_name.to_string(),
                    payload_len: raw.payload.len(),
                    decoded_fields: decoded,
                });
            }
        }
    }

    if let Some(parent) = out_csv_path.parent() {
        let _ = fs::create_dir_all(parent);
    }

    println!("Writing {} records to {}...", records.len(), out_csv_path.display());
    let mut file = fs::File::create(out_csv_path).expect("Failed to create CSV");
    writeln!(file, "file,object_id,object_name,section_type_hex,section_name,payload_len,decoded_fields").unwrap();
    for r in &records {
        // Escape CSV strings if needed
        let safe_name = r.object_name.replace('"', "\"\"");
        let safe_dec = r.decoded_fields.replace('"', "\"\"");
        writeln!(
            file,
            "\"{}\",{},\"{}\",{},\"{}\",{},\"{}\"",
            r.file, r.object_id, safe_name, r.section_type_hex, r.section_name, r.payload_len, safe_dec
        ).unwrap();
    }

    println!("\n=== Camera Data Summary ===");
    println!("Total files containing camera objects: {}", files_with_cam.len());
    println!("Total camera records found: {}", records.len());
    println!("\nSection Type Breakdown:");
    for (sec_type, (cnt, files_set)) in &section_counts {
        println!(
            "  0x{:04X} ({:<12}): {:>5} objects in {:>3} files",
            sec_type,
            section_name(*sec_type),
            cnt,
            files_set.len()
        );
    }
}

//! hackgu-export: Automated batch glTF 2.0 (.glb) exporter for .hack//G.U. Last Recode.

use std::fs::{self, File};
use std::path::{Path, PathBuf};

use hackgu_ccs::parser::CcsFile;
use hackgu_ccs::objects;
use hackgu_export::{convert_ccs_to_glb, export_ccs_file};

fn main() {
    let args: Vec<String> = std::env::args().skip(1).collect();

    if args.is_empty() || args.contains(&"--help".to_string()) || args.contains(&"-h".to_string()) {
        print_usage();
        return;
    }

    if args[0] == "batch" {
        run_batch_export(&args[1..]);
    } else {
        run_single_export(&args);
    }
}

fn print_usage() {
    println!("============================================================");
    println!("     hackgu-export: Automated glTF 2.0 (.glb) Exporter      ");
    println!("============================================================");
    println!("Usage:");
    println!("  hackgu-export <file.ccs> -o <output.glb> [--anim <idx>] [--frame <f>]");
    println!("  hackgu-export batch --cpk <path.cpk> --out <export_dir>");
    println!("  hackgu-export batch --dir <ccs_folder> --out <export_dir>");
    println!();
    println!("Options:");
    println!("  --anim <idx>       Evaluate model with animation track index (default: active/idle)");
    println!("  --frame <frame>    Evaluate at specific frame (default: 0.0)");
    println!("  --bind-pose        Export raw T-pose / bind pose (no animation applied)");
    println!("============================================================");
}

fn run_single_export(args: &[String]) {
    let ccs_path = PathBuf::from(&args[0]);
    let mut out_path = None;
    let mut anim_idx = None;
    let mut frame_val = 0.0f32;
    let mut bind_pose = false;

    let mut i = 1;
    while i < args.len() {
        match args[i].as_str() {
            "-o" | "--out" => {
                if i + 1 < args.len() {
                    out_path = Some(PathBuf::from(&args[i + 1]));
                    i += 1;
                }
            }
            "--anim" => {
                if i + 1 < args.len() {
                    anim_idx = args[i + 1].parse::<usize>().ok();
                    i += 1;
                }
            }
            "--frame" => {
                if i + 1 < args.len() {
                    frame_val = args[i + 1].parse::<f32>().unwrap_or(0.0);
                    i += 1;
                }
            }
            "--bind-pose" => {
                bind_pose = true;
            }
            _ => {}
        }
        i += 1;
    }

    let out = out_path.unwrap_or_else(|| {
        let stem = ccs_path.file_stem().unwrap().to_string_lossy();
        PathBuf::from(format!("{}.glb", stem))
    });

    match export_ccs_file(&ccs_path, &out, anim_idx, frame_val, bind_pose) {
        Ok(count) => {
            println!("Successfully exported {} submeshes to {}", count, out.display());
        }
        Err(e) => {
            eprintln!("Error exporting {}: {}", ccs_path.display(), e);
            std::process::exit(1);
        }
    }
}

fn run_batch_export(args: &[String]) {
    let mut cpk_path = None;
    let mut dir_path = None;
    let mut out_dir = PathBuf::from("exports");

    let mut i = 0;
    while i < args.len() {
        match args[i].as_str() {
            "--cpk" => {
                if i + 1 < args.len() {
                    cpk_path = Some(PathBuf::from(&args[i + 1]));
                    i += 1;
                }
            }
            "--dir" => {
                if i + 1 < args.len() {
                    dir_path = Some(PathBuf::from(&args[i + 1]));
                    i += 1;
                }
            }
            "--out" => {
                if i + 1 < args.len() {
                    out_dir = PathBuf::from(&args[i + 1]);
                    i += 1;
                }
            }
            _ => {}
        }
        i += 1;
    }

    if let Some(cpk) = cpk_path {
        println!("Opening CPK archive: {}", cpk.display());
        let mut file = match File::open(&cpk) {
            Ok(f) => f,
            Err(e) => {
                eprintln!("Failed to open CPK: {}", e);
                return;
            }
        };

        let archive = match hackgu_cpk::CpkArchive::read_from(&mut file) {
            Ok(c) => c,
            Err(e) => {
                eprintln!("Failed to parse CPK archive: {}", e);
                return;
            }
        };

        let ccs_entries: Vec<_> = archive
            .entries
            .iter()
            .filter(|f| f.path.ends_with(".ccs"))
            .cloned()
            .collect();

        println!("Found {} CCS container packages in CPK.", ccs_entries.len());
        let mut total_exported = 0;

        for (idx, entry) in ccs_entries.iter().enumerate() {
            if let Ok(data) = archive.extract_entry(&mut file, entry) {
                if let Ok(ccs) = CcsFile::parse(&data) {
                    let model_count = ccs.objects_of_type(objects::SECTION_MODEL).len();
                    if model_count > 0 {
                        let file_stem = Path::new(&entry.path).file_stem().unwrap().to_string_lossy();
                        
                        // Categorize into folders
                        let cat = if entry.path.contains("/eq/") || file_stem.starts_with('w') {
                            "weapons"
                        } else if file_stem.starts_with("town") || file_stem.starts_with("dungeon") || entry.path.contains("/map/") {
                            "stages"
                        } else if file_stem.starts_with("2") || file_stem.starts_with('c') {
                            "characters"
                        } else {
                            "props"
                        };

                        let cat_dir = out_dir.join(cat);
                        let _ = fs::create_dir_all(&cat_dir);
                        let dest_glb = cat_dir.join(format!("{}.glb", file_stem));

                        print!("[{:3}/{:3}] Exporting {} -> {} ... ", idx + 1, ccs_entries.len(), file_stem, dest_glb.display());
                        match convert_ccs_to_glb(&ccs, &file_stem, None, 0.0, false) {
                            Ok((glb_bytes, submesh_count)) => {
                                if submesh_count > 0 {
                                    if fs::write(&dest_glb, &glb_bytes).is_ok() {
                                        println!("OK ({} meshes, {:.1} KiB)", submesh_count, glb_bytes.len() as f32 / 1024.0);
                                        total_exported += 1;
                                    } else {
                                        println!("WRITE ERROR");
                                    }
                                } else {
                                    println!("SKIPPED (no visible geometry)");
                                }
                            }
                            Err(e) => {
                                println!("ERROR: {}", e);
                            }
                        }
                    }
                }
            }
        }

        println!("Batch complete! Exported {} 3D models to {}", total_exported, out_dir.display());
    } else if let Some(dir) = dir_path {
        println!("Scanning directory: {}", dir.display());
        let _ = fs::create_dir_all(&out_dir);

        let mut ccs_files = Vec::new();
        if let Ok(entries) = fs::read_dir(dir) {
            for entry in entries.flatten() {
                let p = entry.path();
                if p.extension().map(|e| e == "ccs").unwrap_or(false) {
                    ccs_files.push(p);
                }
            }
        }

        println!("Found {} CCS files.", ccs_files.len());
        for p in &ccs_files {
            let stem = p.file_stem().unwrap().to_string_lossy();
            let dest = out_dir.join(format!("{}.glb", stem));
            print!("Exporting {} ... ", p.display());
            match export_ccs_file(p, &dest, None, 0.0, false) {
                Ok(count) => println!("OK ({} meshes)", count),
                Err(e) => println!("ERROR: {}", e),
            }
        }
    } else {
        eprintln!("Error: specify either --cpk <path.cpk> or --dir <folder>");
    }
}

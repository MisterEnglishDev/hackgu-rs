//! hackgu-ccs CLI tool: inspect and dump CyberConnect2 CCS/CCSF container files.

use std::collections::BTreeMap;
use std::env;
use std::fs;
use std::path::PathBuf;
use std::process;

use hackgu_ccs::{section_name, CcsFile};

fn usage() {
    eprintln!("hackgu-ccs: inspect CyberConnect2 CCS/CCSF container assets");
    eprintln!();
    eprintln!("Usage:");
    eprintln!("  hackgu-ccs info <file.ccs>");
    eprintln!("  hackgu-ccs list-files <file.ccs>");
    eprintln!("  hackgu-ccs list-objects <file.ccs>");
    eprintln!("  hackgu-ccs dump-objects <file.ccs> --out <output_dir>");
    eprintln!("  hackgu-ccs dump-textures <file.ccs> --out <output_dir> [--dds]");
    eprintln!("  hackgu-ccs dump-models <file.ccs> --out <output_dir>");
}

fn human_size(n: usize) -> String {
    const UNITS: [&str; 5] = ["B", "KiB", "MiB", "GiB", "TiB"];
    let mut v = n as f64;
    let mut i = 0;
    while v >= 1024.0 && i < UNITS.len() - 1 {
        v /= 1024.0;
        i += 1;
    }
    if i == 0 {
        format!("{} B", n)
    } else {
        format!("{:.1} {}", v, UNITS[i])
    }
}

fn main() {
    let args: Vec<String> = env::args().skip(1).collect();
    if args.is_empty() {
        usage();
        process::exit(1);
    }

    match args[0].as_str() {
        "info" => {
            if args.len() < 2 {
                eprintln!("error: missing file.ccs argument");
                usage();
                process::exit(1);
            }
            let ccs_path = &args[1];
            let data = match fs::read(ccs_path) {
                Ok(d) => d,
                Err(e) => {
                    eprintln!("error reading {}: {}", ccs_path, e);
                    process::exit(1);
                }
            };

            let ccs = match CcsFile::parse(&data) {
                Ok(f) => f,
                Err(e) => {
                    eprintln!("error parsing {}: {}", ccs_path, e);
                    process::exit(1);
                }
            };

            println!("CCS Container:   {}", ccs_path);
            println!("CCSF Name:       {}", ccs.header.name);
            println!("Version:         {}", ccs.header.version.as_str());
            println!("Subfiles:        {}", ccs.files.len());
            println!("Object Index:    {}", ccs.objects.len());
            println!("Setup Objects:   {}", ccs.raw_objects.len());

            let mut type_counts: BTreeMap<&'static str, usize> = BTreeMap::new();
            for obj in &ccs.raw_objects {
                *type_counts.entry(section_name(obj.section_type)).or_default() += 1;
            }

            println!("\nObject breakdown by section type:");
            for (name, count) in type_counts {
                println!("  {:20} : {}", name, count);
            }
        }

        "list-files" => {
            if args.len() < 2 {
                eprintln!("error: missing file.ccs argument");
                usage();
                process::exit(1);
            }
            let ccs_path = &args[1];
            let data = match fs::read(ccs_path) {
                Ok(d) => d,
                Err(e) => {
                    eprintln!("error reading {}: {}", ccs_path, e);
                    process::exit(1);
                }
            };

            let ccs = match CcsFile::parse(&data) {
                Ok(f) => f,
                Err(e) => {
                    eprintln!("error parsing {}: {}", ccs_path, e);
                    process::exit(1);
                }
            };

            println!("Subfiles in {}:", ccs.header.name);
            for f in &ccs.files {
                let name = if f.name.is_empty() { "<root>" } else { &f.name };
                println!("  [{:>3}] {}", f.id, name);
            }
        }

        "list-objects" => {
            if args.len() < 2 {
                eprintln!("error: missing file.ccs argument");
                usage();
                process::exit(1);
            }
            let ccs_path = &args[1];
            let data = match fs::read(ccs_path) {
                Ok(d) => d,
                Err(e) => {
                    eprintln!("error reading {}: {}", ccs_path, e);
                    process::exit(1);
                }
            };

            let ccs = match CcsFile::parse(&data) {
                Ok(f) => f,
                Err(e) => {
                    eprintln!("error parsing {}: {}", ccs_path, e);
                    process::exit(1);
                }
            };

            println!(
                "{:>5}  {:>15}  {:>8}  {:>20}  {}",
                "ID", "Type", "Size", "File", "Object Name"
            );
            println!("{:-<75}", "");

            for obj in &ccs.raw_objects {
                let name = ccs.object_name(obj.object_id).unwrap_or("<unnamed>");
                let file_name = ccs.object_file_name(obj.object_id).unwrap_or("<none>");
                println!(
                    "{:>5}  {:>15}  {:>8}  {:>20}  {}",
                    obj.object_id,
                    section_name(obj.section_type),
                    human_size(obj.size),
                    if file_name.is_empty() { "<root>" } else { file_name },
                    name
                );
            }
        }

        "dump-objects" => {
            if args.len() < 2 {
                eprintln!("error: missing file.ccs argument");
                usage();
                process::exit(1);
            }
            let ccs_path = &args[1];
            let out_idx = args.iter().position(|a| a == "--out");
            let out_dir = match out_idx {
                Some(i) if i + 1 < args.len() => PathBuf::from(&args[i + 1]),
                _ => {
                    eprintln!("error: missing required --out <output_dir> argument");
                    process::exit(1);
                }
            };

            let data = match fs::read(ccs_path) {
                Ok(d) => d,
                Err(e) => {
                    eprintln!("error reading {}: {}", ccs_path, e);
                    process::exit(1);
                }
            };

            let ccs = match CcsFile::parse(&data) {
                Ok(f) => f,
                Err(e) => {
                    eprintln!("error parsing {}: {}", ccs_path, e);
                    process::exit(1);
                }
            };

            fs::create_dir_all(&out_dir).unwrap();
            let mut count = 0;
            for obj in &ccs.raw_objects {
                let name = ccs.object_name(obj.object_id).unwrap_or("obj");
                let clean_name: String = name.chars().map(|c| if c.is_alphanumeric() || c == '_' { c } else { '_' }).collect();
                let type_name = section_name(obj.section_type);
                let fname = format!("{:04}_{}_{}.bin", obj.object_id, type_name, clean_name);
                let target = out_dir.join(fname);
                fs::write(&target, &obj.payload).unwrap();
                count += 1;
            }
            println!("Dumped {} objects to {}", count, out_dir.display());
        }

        "dump-textures" => {
            if args.len() < 2 {
                eprintln!("error: missing file.ccs argument");
                usage();
                process::exit(1);
            }
            let ccs_path = &args[1];
            let out_idx = args.iter().position(|a| a == "--out");
            let out_dir = match out_idx {
                Some(i) if i + 1 < args.len() => PathBuf::from(&args[i + 1]),
                _ => {
                    eprintln!("error: missing required --out <output_dir> argument");
                    process::exit(1);
                }
            };
            let export_dds = args.iter().any(|a| a == "--dds");

            let data = match fs::read(ccs_path) {
                Ok(d) => d,
                Err(e) => {
                    eprintln!("error reading {}: {}", ccs_path, e);
                    process::exit(1);
                }
            };

            let ccs = match CcsFile::parse(&data) {
                Ok(f) => f,
                Err(e) => {
                    eprintln!("error parsing {}: {}", ccs_path, e);
                    process::exit(1);
                }
            };

            let tex_objects = ccs.objects_of_type(hackgu_ccs::objects::SECTION_TEXTURE);
            if tex_objects.is_empty() {
                println!("No Texture objects found in {}", ccs_path);
                return;
            }

            fs::create_dir_all(&out_dir).unwrap();
            let mut count = 0;
            for obj in tex_objects {
                let name = ccs.object_name(obj.object_id).unwrap_or("texture");
                let clean_name: String = name.chars().map(|c| if c.is_alphanumeric() || c == '_' { c } else { '_' }).collect();
                match hackgu_ccs::DecodedTexture::from_payload(name, &obj.payload) {
                    Ok(tex) => {
                        let ext = if export_dds { "dds" } else { "bmp" };
                        let fname = format!("{:04}_{}_{}x{}.{}", obj.object_id, clean_name, tex.width, tex.height, ext);
                        let target = out_dir.join(fname);
                        let res = if export_dds {
                            tex.save_dds(&target)
                        } else {
                            tex.save_bmp(&target)
                        };

                        if let Err(e) = res {
                            eprintln!("error saving texture {}: {}", name, e);
                        } else {
                            println!(
                                "Exported [{:>3}] {:25} ({}x{}, fmt 0x{:02X}) -> {}",
                                obj.object_id, name, tex.width, tex.height, tex.format, target.display()
                            );
                            count += 1;
                        }
                    }
                    Err(e) => {
                        eprintln!("Failed to decode texture [{}] {}: {}", obj.object_id, name, e);
                    }
                }
            }
            println!("Successfully exported {} textures to {}.", count, out_dir.display());
        }

        "dump-models" => {
            if args.len() < 2 {
                eprintln!("error: missing file.ccs argument");
                usage();
                process::exit(1);
            }
            let ccs_path = &args[1];
            let mut out_dir = PathBuf::from("./models_out");

            let mut i = 2;
            while i < args.len() {
                if args[i] == "--out" && i + 1 < args.len() {
                    out_dir = PathBuf::from(&args[i + 1]);
                    i += 2;
                } else {
                    i += 1;
                }
            }

            let data = match fs::read(ccs_path) {
                Ok(d) => d,
                Err(e) => {
                    eprintln!("error reading {}: {}", ccs_path, e);
                    process::exit(1);
                }
            };

            let ccs = match CcsFile::parse(&data) {
                Ok(c) => c,
                Err(e) => {
                    eprintln!("error parsing CCS file {}: {}", ccs_path, e);
                    process::exit(1);
                }
            };

            let model_objects: Vec<_> = ccs.raw_objects.iter().filter(|o| o.section_type == 0x0800).collect();
            if model_objects.is_empty() {
                println!("No Model objects found in {}", ccs_path);
                return;
            }

            fs::create_dir_all(&out_dir).unwrap();

            let clumps = ccs.clumps();
            let scene_nodes = ccs.scene_nodes();
            let materials = ccs.materials();
            let world_matrices = if let Some(clump) = clumps.first() {
                println!(
                    "Found skeleton clump [{}] with {} bones. Resolving bind-pose matrices...",
                    clump.object_id, clump.bone_count
                );
                clump.compute_world_matrices(&scene_nodes)
            } else {
                std::collections::HashMap::new()
            };

            // Export textures into the model output directory so OBJ+MTL renders work seamlessly
            let mut tex_id_to_filename = std::collections::HashMap::new();
            for tex_obj in ccs.objects_of_type(hackgu_ccs::objects::SECTION_TEXTURE) {
                let name = ccs.object_name(tex_obj.object_id).unwrap_or("texture");
                let clean_name: String = name.chars().map(|c| if c.is_alphanumeric() || c == '_' { c } else { '_' }).collect();
                if let Ok(tex) = hackgu_ccs::DecodedTexture::from_payload(name, &tex_obj.payload) {
                    let fname = format!("{:04}_{}.bmp", tex_obj.object_id, clean_name);
                    let target = out_dir.join(&fname);
                    let _ = tex.save_bmp(&target);
                    tex_id_to_filename.insert(tex_obj.object_id, fname);
                }
            }

            // Map Material ID to texture filename
            let mut mat_to_tex_name = std::collections::HashMap::new();
            for (mat_id, mat) in &materials {
                if let Some(tex_fname) = tex_id_to_filename.get(&mat.texture_obj_id) {
                    mat_to_tex_name.insert(*mat_id, tex_fname.clone());
                }
            }

            let mut count = 0;
            for obj in model_objects {
                let name = ccs.object_name(obj.object_id).unwrap_or("model");
                let clean_name: String = name.chars().map(|c| if c.is_alphanumeric() || c == '_' { c } else { '_' }).collect();

                match hackgu_ccs::DecodedModel::from_payload(name, &obj.payload, ccs.header.raw_version) {
                    Ok(mut model) => {
                        let total_verts: usize = model.meshes.iter().map(|m| m.vertices.len()).sum();
                        let total_tris: usize = model.meshes.iter().map(|m| m.indices.len() / 3).sum();
                        if total_verts == 0 {
                            // Dummy or empty attach point
                            continue;
                        }

                        // Apply skeletal bind pose if skeleton is present
                        if let Some(clump) = clumps.first() {
                            if !world_matrices.is_empty() {
                                model.apply_skeleton(clump, &world_matrices);
                            }
                        }

                        let fname = format!("{:04}_{}_{}v_{}t.obj", obj.object_id, clean_name, total_verts, total_tris);
                        let target = out_dir.join(fname);
                        if let Err(e) = model.save_obj_with_materials(&target, &mat_to_tex_name) {
                            eprintln!("error saving model {}: {}", name, e);
                        } else {
                            println!(
                                "Exported [{:>3}] {:25} ({} submeshes, {} verts, {} tris) -> {}",
                                obj.object_id, name, model.meshes.len(), total_verts, total_tris, target.display()
                            );
                            count += 1;
                        }
                    }
                    Err(e) => {
                        eprintln!("Failed to decode model [{}] {}: {}", obj.object_id, name, e);
                    }
                }
            }
            println!("Successfully exported {} 3D models to {}.", count, out_dir.display());
        }

        "-h" | "--help" | "help" => {
            usage();
        }

        unknown => {
            eprintln!("unknown command: {}", unknown);
            usage();
            process::exit(1);
        }
    }
}

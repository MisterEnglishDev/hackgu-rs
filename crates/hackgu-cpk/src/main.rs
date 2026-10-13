//! hackgu-cpk CLI tool: list, inspect, and extract files from CRI CPK archives.

use std::env;
use std::fs;
use std::io::{self, Write};
use std::path::PathBuf;
use std::process;

use hackgu_cpk::CpkArchive;

fn usage() {
    eprintln!("hackgu-cpk: inspect, list, and extract CRI CPK archives for .hack//G.U. Last Recode");
    eprintln!();
    eprintln!("Usage:");
    eprintln!("  hackgu-cpk list <archive.cpk> [--filter <pattern>]");
    eprintln!("  hackgu-cpk info <archive.cpk>");
    eprintln!("  hackgu-cpk extract <archive.cpk> --out <output_dir>");
    eprintln!("  hackgu-cpk cat <archive.cpk> <file_name_or_path> [--out <dest_file>]");
    eprintln!();
    eprintln!("Examples:");
    eprintln!("  hackgu-cpk list F:\\SteamLibrary\\steamapps\\common\\hackGU\\cpk\\hackGU_vol1_i_pc.cpk");
    eprintln!("  hackgu-cpk extract F:\\SteamLibrary\\steamapps\\common\\hackGU\\cpk\\hackGU_vol1_i_pc.cpk --out ./extracted");
}

fn human_size(n: u64) -> String {
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
        "list" => {
            if args.len() < 2 {
                eprintln!("error: missing archive path");
                usage();
                process::exit(1);
            }
            let archive_path = &args[1];
            let filter = args.iter().position(|a| a == "--filter").and_then(|i| args.get(i + 1));

            let (archive, _) = match CpkArchive::open(archive_path) {
                Ok(res) => res,
                Err(e) => {
                    eprintln!("error opening {}: {}", archive_path, e);
                    process::exit(1);
                }
            };

            println!(
                "{:>5}  {:>10}  {:>10}  {:>5}  {}",
                "ID", "Size", "Extracted", "Comp", "Path"
            );
            println!("{:-<75}", "");

            let mut count = 0;
            for entry in &archive.entries {
                if let Some(f) = filter {
                    if !entry.path.to_ascii_lowercase().contains(&f.to_ascii_lowercase()) {
                        continue;
                    }
                }
                println!(
                    "{:>5}  {:>10}  {:>10}  {:>5}  {}",
                    entry.id,
                    human_size(entry.file_size),
                    human_size(entry.extract_size),
                    if entry.is_compressed { "YES" } else { "no" },
                    entry.path
                );
                count += 1;
            }
            println!("{:-<75}", "");
            println!("Displayed {} / {} entries.", count, archive.entries.len());
        }

        "info" => {
            if args.len() < 2 {
                eprintln!("error: missing archive path");
                usage();
                process::exit(1);
            }
            let archive_path = &args[1];
            let (archive, _) = match CpkArchive::open(archive_path) {
                Ok(res) => res,
                Err(e) => {
                    eprintln!("error opening {}: {}", archive_path, e);
                    process::exit(1);
                }
            };

            let total_compressed: u64 = archive.entries.iter().map(|e| e.file_size).sum();
            let total_uncompressed: u64 = archive.entries.iter().map(|e| e.extract_size).sum();
            let compressed_files = archive.entries.iter().filter(|e| e.is_compressed).count();

            println!("Archive:          {}", archive_path);
            println!("Files:            {}", archive.entries.len());
            println!("Compressed files: {}", compressed_files);
            println!("TOC Offset:       0x{:X}", archive.toc_offset);
            println!("Content Offset:   0x{:X}", archive.content_offset);
            println!("Alignment:        {} bytes", archive.align);
            println!("Stored Size:      {} ({})", human_size(total_compressed), total_compressed);
            println!("Unpacked Size:    {} ({})", human_size(total_uncompressed), total_uncompressed);
        }

        "extract" => {
            if args.len() < 2 {
                eprintln!("error: missing archive path");
                usage();
                process::exit(1);
            }
            let archive_path = &args[1];
            let out_idx = args.iter().position(|a| a == "--out");
            let out_dir = match out_idx {
                Some(i) if i + 1 < args.len() => PathBuf::from(&args[i + 1]),
                _ => {
                    eprintln!("error: missing required --out <output_dir> argument");
                    process::exit(1);
                }
            };

            println!("Opening {}...", archive_path);
            let (archive, mut file) = match CpkArchive::open(archive_path) {
                Ok(res) => res,
                Err(e) => {
                    eprintln!("error: {}", e);
                    process::exit(1);
                }
            };

            println!("Extracting {} files to {}...", archive.entries.len(), out_dir.display());
            match archive.extract_all(&mut file, &out_dir) {
                Ok(n) => println!("Successfully extracted {} files.", n),
                Err(e) => {
                    eprintln!("error extracting archive: {}", e);
                    process::exit(1);
                }
            }
        }

        "cat" => {
            if args.len() < 3 {
                eprintln!("error: missing archive path or file path");
                usage();
                process::exit(1);
            }
            let archive_path = &args[1];
            let needle = &args[2];
            let dest_out = args.iter().position(|a| a == "--out").and_then(|i| args.get(i + 1));

            let (archive, mut file) = match CpkArchive::open(archive_path) {
                Ok(res) => res,
                Err(e) => {
                    eprintln!("error: {}", e);
                    process::exit(1);
                }
            };

            let data = match archive.extract_by_path(&mut file, needle) {
                Ok(d) => d,
                Err(e) => {
                    eprintln!("error extracting '{}': {}", needle, e);
                    process::exit(1);
                }
            };

            if let Some(out_path) = dest_out {
                if let Err(e) = fs::write(out_path, &data) {
                    eprintln!("error writing {}: {}", out_path, e);
                    process::exit(1);
                }
                println!("Wrote {} bytes to {}", data.len(), out_path);
            } else {
                let _ = io::stdout().write_all(&data);
            }
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

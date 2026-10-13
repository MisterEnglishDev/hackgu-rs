//! hackgu-vfs CLI tool: inspect and query the layered Virtual File System.

use std::env;
use std::fs;
use std::io::{self, Write};
use std::path::PathBuf;
use std::process;

use hackgu_vfs::Vfs;

fn usage() {
    eprintln!("hackgu-vfs: test layered Virtual File System and mod overrides");
    eprintln!();
    eprintln!("Usage:");
    eprintln!("  hackgu-vfs list <game_root> [--vol <1-4>] [--mods <mods_dir>] [--filter <pattern>]");
    eprintln!("  hackgu-vfs resolve <game_root> <file_path> [--vol <1-4>] [--mods <mods_dir>]");
    eprintln!("  hackgu-vfs cat <game_root> <file_path> [--vol <1-4>] [--mods <mods_dir>] [--out <dest_file>]");
    eprintln!();
    eprintln!("Example:");
    eprintln!("  hackgu-vfs resolve F:\\SteamLibrary\\steamapps\\common\\hackGU mapmenu@ENG.ccs --vol 1");
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

    let cmd = &args[0];
    if cmd == "-h" || cmd == "--help" || cmd == "help" {
        usage();
        return;
    }

    if args.len() < 2 {
        eprintln!("error: missing game_root argument");
        usage();
        process::exit(1);
    }

    let game_root = PathBuf::from(&args[1]);
    let volume: u32 = args.iter().position(|a| a == "--vol")
        .and_then(|i| args.get(i + 1))
        .and_then(|v| v.parse().ok())
        .unwrap_or(1);

    let mods_dir: Option<PathBuf> = args.iter().position(|a| a == "--mods")
        .and_then(|i| args.get(i + 1))
        .map(PathBuf::from);

    let filter = args.iter().position(|a| a == "--filter").and_then(|i| args.get(i + 1));
    let out_dest = args.iter().position(|a| a == "--out").and_then(|i| args.get(i + 1));

    let mut vfs = Vfs::new();
    match vfs.mount_game(&game_root, volume, mods_dir.as_deref()) {
        Ok(count) => {
            eprintln!("Mounted {} layers for Volume {} (game: {})...", count, volume, game_root.display());
        }
        Err(e) => {
            eprintln!("error mounting game: {}", e);
            process::exit(1);
        }
    }

    match cmd.as_str() {
        "list" => {
            let files = vfs.list_all_files();
            println!("{:>4}  {:>30}  {}", "Prio", "Source Mount", "Virtual Path");
            println!("{:-<75}", "");
            let mut shown = 0;
            for f in &files {
                if let Some(pat) = filter {
                    if !f.contains(&pat.to_ascii_lowercase()) {
                        continue;
                    }
                }
                let (source, prio) = vfs.resolve_source(f).unwrap_or(("unknown", 0));
                println!("{:>4}  {:>30}  {}", prio, source, f);
                shown += 1;
            }
            println!("{:-<75}", "");
            println!("Displayed {} / {} unique virtual files.", shown, files.len());
        }

        "resolve" => {
            if args.len() < 3 {
                eprintln!("error: missing file_path argument");
                usage();
                process::exit(1);
            }
            let file_path = &args[2];
            match vfs.resolve_source(file_path) {
                Some((source, prio)) => {
                    println!("File:     {}", file_path);
                    println!("Resolved: YES");
                    println!("Source:   {} (Priority {})", source, prio);
                }
                None => {
                    println!("File:     {}", file_path);
                    println!("Resolved: NO (file not found in any mounted layer)");
                    process::exit(1);
                }
            }
        }

        "cat" => {
            if args.len() < 3 {
                eprintln!("error: missing file_path argument");
                usage();
                process::exit(1);
            }
            let file_path = &args[2];
            let data = match vfs.read(file_path) {
                Ok(d) => d,
                Err(e) => {
                    eprintln!("error reading {}: {}", file_path, e);
                    process::exit(1);
                }
            };

            if let Some(dest) = out_dest {
                if let Err(e) = fs::write(dest, &data) {
                    eprintln!("error writing {}: {}", dest, e);
                    process::exit(1);
                }
                eprintln!("Extracted {} ({}) to {}", file_path, human_size(data.len()), dest);
            } else {
                let _ = io::stdout().write_all(&data);
            }
        }

        unknown => {
            eprintln!("unknown command: {}", unknown);
            usage();
            process::exit(1);
        }
    }
}

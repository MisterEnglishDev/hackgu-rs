//! hackgu-survey: inventory a game install folder you own.
//!
//! Reads files only; never modifies anything. Std-only, no dependencies.
//!
//! Usage:
//!   hackgu-survey <game_dir> [--out report.txt] [--csv files.csv]

use std::collections::{BTreeMap, BTreeSet, HashMap};
use std::env;
use std::fmt::Write as _;
use std::fs::{self, File};
use std::io::{self, Read};
use std::path::{Path, PathBuf};

const SAMPLE_BYTES: usize = 64 * 1024;
const MAX_PE_SCAN_BYTES: u64 = 256 * 1024 * 1024;
const TOP_LARGEST: usize = 40;

struct Entry {
    rel: String,
    size: u64,
    ext: String,
    head: [u8; 16],
    head_len: usize,
    kind: &'static str,
    entropy: f64,
}

fn usage() {
    eprintln!("hackgu-survey <game_dir> [--out report.txt] [--csv files.csv]");
    eprintln!();
    eprintln!("Inventories a game folder: file types, magic bytes, entropy, engine hints.");
    eprintln!("Read-only. Run it on a folder you own and share the report, not the game files.");
}

fn walk(dir: &Path, out: &mut Vec<PathBuf>) -> io::Result<()> {
    for e in fs::read_dir(dir)? {
        let e = e?;
        let ft = e.file_type()?;
        let p = e.path();
        if ft.is_dir() {
            if let Err(err) = walk(&p, out) {
                eprintln!("warn: skipping {}: {}", p.display(), err);
            }
        } else if ft.is_file() {
            out.push(p);
        }
    }
    Ok(())
}

fn read_head(path: &Path) -> io::Result<Vec<u8>> {
    let mut buf = Vec::with_capacity(SAMPLE_BYTES);
    File::open(path)?.take(SAMPLE_BYTES as u64).read_to_end(&mut buf)?;
    Ok(buf)
}

fn entropy(data: &[u8]) -> f64 {
    if data.is_empty() {
        return 0.0;
    }
    let mut counts = [0u64; 256];
    for &b in data {
        counts[b as usize] += 1;
    }
    let n = data.len() as f64;
    counts
        .iter()
        .filter(|&&c| c > 0)
        .map(|&c| {
            let p = c as f64 / n;
            -p * p.log2()
        })
        .sum()
}

fn looks_like_text(data: &[u8]) -> bool {
    let n = data.len().min(256);
    n > 0
        && data[..n]
            .iter()
            .all(|&b| b == b'\n' || b == b'\r' || b == b'\t' || (0x20..0x7f).contains(&b))
}

fn classify(h: &[u8]) -> &'static str {
    if h.starts_with(b"MZ") {
        "PE executable/DLL"
    } else if h.starts_with(b"UnityFS") || h.starts_with(b"UnityWeb") || h.starts_with(b"UnityRaw") {
        "Unity asset bundle"
    } else if h.starts_with(b"CPK ") {
        "CRI CPK archive"
    } else if h.starts_with(b"@UTF") {
        "CRI @UTF table (ACB/CPK family)"
    } else if h.starts_with(b"AFS2") {
        "CRI AFS2 archive"
    } else if h.starts_with(b"CRID") {
        "CRI USM video"
    } else if h.len() >= 3 && (h[0] & 0x7f) == b'H' && (h[1] & 0x7f) == b'C' && (h[2] & 0x7f) == b'A' {
        "CRI HCA audio"
    } else if h.starts_with(b"BKHD") {
        "Wwise soundbank"
    } else if h.starts_with(b"FSB5") {
        "FMOD FSB5 bank"
    } else if h.starts_with(b"RIFF") {
        "RIFF (wav/avi/webp)"
    } else if h.starts_with(b"OggS") {
        "Ogg"
    } else if h.starts_with(b"DDS ") {
        "DDS texture"
    } else if h.starts_with(&[0x89, b'P', b'N', b'G']) {
        "PNG"
    } else if h.starts_with(b"BIK") {
        "Bink video"
    } else if h.starts_with(b"PK\x03\x04") {
        "ZIP"
    } else if h.starts_with(&[0x28, 0xB5, 0x2F, 0xFD]) {
        "Zstandard"
    } else if h.starts_with(&[0x1f, 0x8b]) {
        "gzip"
    } else if h.starts_with(&[0x7f, b'E', b'L', b'F']) {
        "ELF"
    } else if looks_like_text(h) {
        "text-like"
    } else {
        "unknown"
    }
}

fn human(n: u64) -> String {
    const U: [&str; 5] = ["B", "KiB", "MiB", "GiB", "TiB"];
    let mut v = n as f64;
    let mut i = 0;
    while v >= 1024.0 && i < U.len() - 1 {
        v /= 1024.0;
        i += 1;
    }
    if i == 0 {
        format!("{} B", n)
    } else {
        format!("{:.1} {}", v, U[i])
    }
}

fn hex(bytes: &[u8]) -> String {
    bytes.iter().map(|b| format!("{:02x}", b)).collect::<Vec<_>>().join(" ")
}

fn ascii(bytes: &[u8]) -> String {
    bytes
        .iter()
        .map(|&b| if (0x20..0x7f).contains(&b) { b as char } else { '.' })
        .collect()
}

fn is_name_byte(b: u8) -> bool {
    b.is_ascii_alphanumeric() || b == b'.' || b == b'_' || b == b'-'
}

/// Pull printable runs ending in ".dll" out of a PE image (import table names live as plain ASCII).
fn dll_names(data: &[u8]) -> BTreeSet<String> {
    let mut set = BTreeSet::new();
    let n = data.len();
    let mut i = 0;
    while i < n {
        if is_name_byte(data[i]) {
            let s = i;
            while i < n && is_name_byte(data[i]) {
                i += 1;
            }
            let run = &data[s..i];
            if (5..=64).contains(&run.len()) {
                if let Ok(t) = std::str::from_utf8(run) {
                    if t.to_ascii_lowercase().ends_with(".dll") {
                        set.insert(t.to_string());
                    }
                }
            }
        } else {
            i += 1;
        }
    }
    set
}

fn engine_hints(entries: &[Entry]) -> Vec<String> {
    let paths: Vec<String> = entries.iter().map(|e| e.rel.to_ascii_lowercase()).collect();
    let has = |needle: &str| paths.iter().any(|p| p.contains(needle));
    let mut hints = Vec::new();
    let mut check = |needle: &str, msg: &str| {
        if has(needle) {
            hints.push(format!("{}  (matched path containing \"{}\")", msg, needle));
        }
    };
    check("unityplayer.dll", "Unity engine");
    check("globalgamemanagers", "Unity data folder");
    check("mono", "Mono runtime present (Unity/.NET)");
    check("il2cpp", "Unity IL2CPP");
    check("ue4", "Unreal Engine 4 markers");
    check("ue5", "Unreal Engine 5 markers");
    check("engine/", "Unreal-style Engine/ folder");
    check(".pak", "PAK archives (Unreal or custom)");
    check(".cpk", "CRI CPK archives (CRIWARE middleware)");
    check("criware", "CRIWARE middleware");
    check("cri_", "CRIWARE-style naming");
    check(".usm", "CRI USM video");
    check(".acb", "CRI ACB audio banks");
    check(".awb", "CRI AWB wave banks");
    check(".hca", "CRI HCA audio");
    check(".bnk", "Wwise soundbanks");
    check("fmod", "FMOD audio");
    check("bink", "Bink video");
    check("steam_api", "Steamworks");
    check("denuvo", "Denuvo DRM markers");
    check("easyanticheat", "EasyAntiCheat");
    check("d3d11", "Direct3D 11 DLLs bundled");
    check("d3d12", "Direct3D 12 DLLs bundled");
    check("vulkan", "Vulkan loader bundled");
    hints
}

fn main() {
    let args: Vec<String> = env::args().skip(1).collect();
    let mut dir: Option<String> = None;
    let mut out_path: Option<String> = None;
    let mut csv_path: Option<String> = None;
    let mut it = args.into_iter();
    while let Some(a) = it.next() {
        match a.as_str() {
            "--out" => out_path = it.next(),
            "--csv" => csv_path = it.next(),
            "-h" | "--help" => {
                usage();
                return;
            }
            _ => dir = Some(a),
        }
    }
    let Some(dir) = dir else {
        usage();
        std::process::exit(2);
    };
    let root = PathBuf::from(&dir);
    if !root.is_dir() {
        eprintln!("error: {} is not a directory", dir);
        std::process::exit(1);
    }

    let mut paths = Vec::new();
    if let Err(e) = walk(&root, &mut paths) {
        eprintln!("error walking {}: {}", dir, e);
        std::process::exit(1);
    }
    paths.sort();

    let mut entries: Vec<Entry> = Vec::with_capacity(paths.len());
    for p in &paths {
        let size = match fs::metadata(p) {
            Ok(m) => m.len(),
            Err(e) => {
                eprintln!("warn: {}: {}", p.display(), e);
                continue;
            }
        };
        let sample = match read_head(p) {
            Ok(b) => b,
            Err(e) => {
                eprintln!("warn: {}: {}", p.display(), e);
                continue;
            }
        };
        let mut head = [0u8; 16];
        let head_len = sample.len().min(16);
        head[..head_len].copy_from_slice(&sample[..head_len]);
        let rel = p
            .strip_prefix(&root)
            .unwrap_or(p)
            .to_string_lossy()
            .replace('\\', "/");
        let ext = p
            .extension()
            .map(|e| e.to_string_lossy().to_ascii_lowercase())
            .unwrap_or_else(|| "(none)".to_string());
        entries.push(Entry {
            rel,
            size,
            ext,
            head,
            head_len,
            kind: classify(&sample),
            entropy: entropy(&sample),
        });
    }

    let total: u64 = entries.iter().map(|e| e.size).sum();
    let mut r = String::new();
    let _ = writeln!(r, "hackgu-survey report");
    let _ = writeln!(r, "root: {}", dir);
    let _ = writeln!(r, "files: {}   total: {}", entries.len(), human(total));
    let _ = writeln!(r);

    // Engine / middleware hints
    let _ = writeln!(r, "== Engine / middleware hints (from file names) ==");
    let hints = engine_hints(&entries);
    if hints.is_empty() {
        let _ = writeln!(r, "(none matched; check the PE imports below)");
    }
    for h in hints {
        let _ = writeln!(r, "- {}", h);
    }
    let _ = writeln!(r);

    // By extension
    let mut by_ext: BTreeMap<&str, (u64, u64)> = BTreeMap::new();
    for e in &entries {
        let s = by_ext.entry(e.ext.as_str()).or_default();
        s.0 += 1;
        s.1 += e.size;
    }
    let mut ext_rows: Vec<_> = by_ext.into_iter().collect();
    ext_rows.sort_by(|a, b| b.1 .1.cmp(&a.1 .1));
    let _ = writeln!(r, "== By extension (largest first) ==");
    let _ = writeln!(r, "{:<12} {:>8} {:>12}", "ext", "files", "size");
    for (ext, (n, sz)) in &ext_rows {
        let _ = writeln!(r, "{:<12} {:>8} {:>12}", ext, n, human(*sz));
    }
    let _ = writeln!(r);

    // By detected kind
    let mut by_kind: BTreeMap<&str, (u64, u64)> = BTreeMap::new();
    for e in &entries {
        let s = by_kind.entry(e.kind).or_default();
        s.0 += 1;
        s.1 += e.size;
    }
    let mut kind_rows: Vec<_> = by_kind.into_iter().collect();
    kind_rows.sort_by(|a, b| b.1 .1.cmp(&a.1 .1));
    let _ = writeln!(r, "== By detected type (magic bytes) ==");
    let _ = writeln!(r, "{:<34} {:>8} {:>12}", "type", "files", "size");
    for (k, (n, sz)) in &kind_rows {
        let _ = writeln!(r, "{:<34} {:>8} {:>12}", k, n, human(*sz));
    }
    let _ = writeln!(r);

    // Largest files
    let mut by_size: Vec<&Entry> = entries.iter().collect();
    by_size.sort_by(|a, b| b.size.cmp(&a.size));
    let _ = writeln!(r, "== {} largest files ==", TOP_LARGEST);
    let _ = writeln!(r, "{:>11}  {:>4}  {:<30} {}", "size", "H", "type", "path");
    for e in by_size.iter().take(TOP_LARGEST) {
        let _ = writeln!(
            r,
            "{:>11}  {:>4.1}  {:<30} {}",
            human(e.size),
            e.entropy,
            e.kind,
            e.rel
        );
    }
    let _ = writeln!(r, "(H = Shannon entropy of the first 64 KiB, 0-8; near 8 means compressed or encrypted)");
    let _ = writeln!(r);

    // Unrecognized formats: the interesting part for reverse engineering
    #[derive(Default)]
    struct Unk {
        count: u64,
        size: u64,
        ent_sum: f64,
        magics: HashMap<Vec<u8>, u64>,
        sample: String,
    }
    let mut unk: BTreeMap<&str, Unk> = BTreeMap::new();
    for e in entries.iter().filter(|e| e.kind == "unknown") {
        let u = unk.entry(e.ext.as_str()).or_default();
        u.count += 1;
        u.size += e.size;
        u.ent_sum += e.entropy;
        if u.sample.is_empty() {
            u.sample = e.rel.clone();
        }
        *u.magics.entry(e.head[..e.head_len.min(4)].to_vec()).or_default() += 1;
    }
    let mut unk_rows: Vec<_> = unk.into_iter().collect();
    unk_rows.sort_by(|a, b| b.1.size.cmp(&a.1.size));
    let _ = writeln!(r, "== Unrecognized formats, grouped by extension ==");
    let _ = writeln!(r, "(the likely custom formats; the common first 4 bytes often identify the container)");
    for (ext, u) in unk_rows.iter().take(40) {
        let mut mags: Vec<_> = u.magics.iter().collect();
        mags.sort_by(|a, b| b.1.cmp(a.1));
        let top: Vec<String> = mags
            .iter()
            .take(3)
            .map(|(m, c)| format!("[{}] \"{}\" x{}", hex(m), ascii(m), c))
            .collect();
        let _ = writeln!(
            r,
            "- .{:<10} {:>6} files {:>11}  avgH {:.1}  magics: {}\n    e.g. {}",
            ext,
            u.count,
            human(u.size),
            u.ent_sum / u.count as f64,
            top.join(", "),
            u.sample
        );
    }
    let _ = writeln!(r);

    // PE imports for exes / DLLs near the top of the tree
    let _ = writeln!(r, "== PE files near the root: referenced DLL names ==");
    let mut pe_count = 0;
    for e in entries
        .iter()
        .filter(|e| e.kind == "PE executable/DLL" && e.rel.matches('/').count() <= 1 && e.size <= MAX_PE_SCAN_BYTES)
    {
        let path = root.join(&e.rel);
        match fs::read(&path) {
            Ok(data) => {
                let names = dll_names(&data);
                if names.is_empty() {
                    continue;
                }
                pe_count += 1;
                let list: Vec<&str> = names.iter().map(|s| s.as_str()).take(40).collect();
                let _ = writeln!(r, "- {} ({}): {}", e.rel, human(e.size), list.join(", "));
            }
            Err(err) => eprintln!("warn: {}: {}", e.rel, err),
        }
    }
    if pe_count == 0 {
        let _ = writeln!(r, "(none found)");
    }

    print!("{}", r);
    if let Some(p) = out_path {
        match fs::write(&p, &r) {
            Ok(()) => eprintln!("wrote {}", p),
            Err(e) => eprintln!("error writing {}: {}", p, e),
        }
    }
    if let Some(p) = csv_path {
        let mut c = String::from("path,size,ext,type,entropy,head_hex\n");
        for e in &entries {
            let _ = writeln!(
                c,
                "\"{}\",{},{},\"{}\",{:.3},\"{}\"",
                e.rel.replace('"', "\"\""),
                e.size,
                e.ext,
                e.kind,
                e.entropy,
                hex(&e.head[..e.head_len])
            );
        }
        match fs::write(&p, c) {
            Ok(()) => eprintln!("wrote {}", p),
            Err(e) => eprintln!("error writing {}: {}", p, e),
        }
    }
}

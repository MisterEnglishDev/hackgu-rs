//! Layered Virtual File System for .hack//G.U. Last Recode.
//!
//! Allows mounting base CPK archives alongside loose mod folders. Files in higher-priority
//! sources automatically shadow base game assets, enabling zero-repack modding and game mixing.

use std::collections::BTreeSet;
use std::fmt;
use std::fs::{self, File};
use std::io;
use std::path::{Path, PathBuf};
use std::sync::Mutex;

use hackgu_cpk::{CpkArchive, CpkError};

#[derive(Debug)]
pub enum VfsError {
    Io(io::Error),
    Cpk(CpkError),
    FileNotFound(String),
}

impl fmt::Display for VfsError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            VfsError::Io(e) => write!(f, "VFS I/O error: {}", e),
            VfsError::Cpk(e) => write!(f, "VFS CPK error: {}", e),
            VfsError::FileNotFound(p) => write!(f, "File not found in any mounted VFS source: {}", p),
        }
    }
}

impl std::error::Error for VfsError {}

impl From<io::Error> for VfsError {
    fn from(e: io::Error) -> Self {
        VfsError::Io(e)
    }
}

impl From<CpkError> for VfsError {
    fn from(e: CpkError) -> Self {
        VfsError::Cpk(e)
    }
}

/// Normalize path for cross-platform VFS lookup (lowercased, forward slashes, no leading slash).
pub fn normalize_vfs_path(p: &str) -> String {
    let clean = p.replace('\\', "/");
    clean.trim_start_matches('/').to_ascii_lowercase()
}

pub trait MountSource: Send + Sync {
    fn name(&self) -> &str;
    fn priority(&self) -> i32;
    fn has_file(&self, path: &str) -> bool;
    fn read_file(&self, path: &str) -> Result<Vec<u8>, VfsError>;
    fn list_files(&self) -> Vec<String>;
}

/// A loose directory on disk (for mods or loose assets).
pub struct DirMount {
    name: String,
    root: PathBuf,
    priority: i32,
}

impl DirMount {
    pub fn new<P: AsRef<Path>>(name: String, root: P, priority: i32) -> Self {
        Self {
            name,
            root: root.as_ref().to_path_buf(),
            priority,
        }
    }

    fn resolve_path(&self, normalized_path: &str) -> Option<PathBuf> {
        // Direct attempt
        let direct = self.root.join(normalized_path);
        if direct.is_file() {
            return Some(direct);
        }

        // Case-insensitive recursive resolution if exact match fails
        let parts: Vec<&str> = normalized_path.split('/').collect();
        let mut cur = self.root.clone();

        for part in parts {
            let mut found = false;
            if let Ok(entries) = fs::read_dir(&cur) {
                for e in entries.flatten() {
                    let fname = e.file_name();
                    if fname.to_string_lossy().eq_ignore_ascii_case(part) {
                        cur = e.path();
                        found = true;
                        break;
                    }
                }
            }
            if !found {
                return None;
            }
        }

        if cur.is_file() {
            Some(cur)
        } else {
            None
        }
    }
}

impl MountSource for DirMount {
    fn name(&self) -> &str {
        &self.name
    }

    fn priority(&self) -> i32 {
        self.priority
    }

    fn has_file(&self, path: &str) -> bool {
        let norm = normalize_vfs_path(path);
        self.resolve_path(&norm).is_some()
    }

    fn read_file(&self, path: &str) -> Result<Vec<u8>, VfsError> {
        let norm = normalize_vfs_path(path);
        let resolved = self.resolve_path(&norm)
            .ok_or_else(|| VfsError::FileNotFound(path.to_string()))?;
        let data = fs::read(resolved)?;
        Ok(data)
    }

    fn list_files(&self) -> Vec<String> {
        let mut list = Vec::new();
        fn walk(dir: &Path, base: &Path, out: &mut Vec<String>) {
            if let Ok(entries) = fs::read_dir(dir) {
                for e in entries.flatten() {
                    let p = e.path();
                    if p.is_dir() {
                        walk(&p, base, out);
                    } else if p.is_file() {
                        if let Ok(rel) = p.strip_prefix(base) {
                            out.push(normalize_vfs_path(&rel.to_string_lossy()));
                        }
                    }
                }
            }
        }
        walk(&self.root, &self.root, &mut list);
        list
    }
}

/// A mounted CRI CPK archive file.
pub struct CpkMount {
    name: String,
    archive: CpkArchive,
    file: Mutex<File>,
    priority: i32,
}

impl CpkMount {
    pub fn open<P: AsRef<Path>>(path: P, priority: i32) -> Result<Self, VfsError> {
        let p = path.as_ref();
        let name = p.file_name().unwrap_or_default().to_string_lossy().to_string();
        let (archive, file) = CpkArchive::open(p)?;
        Ok(Self {
            name,
            archive,
            file: Mutex::new(file),
            priority,
        })
    }
}

impl MountSource for CpkMount {
    fn name(&self) -> &str {
        &self.name
    }

    fn priority(&self) -> i32 {
        self.priority
    }

    fn has_file(&self, path: &str) -> bool {
        let norm = normalize_vfs_path(path);
        self.archive.entries.iter().any(|e| normalize_vfs_path(&e.path) == norm || normalize_vfs_path(&e.file_name) == norm)
    }

    fn read_file(&self, path: &str) -> Result<Vec<u8>, VfsError> {
        let norm = normalize_vfs_path(path);
        let entry = self.archive.entries.iter().find(|e| {
            normalize_vfs_path(&e.path) == norm || normalize_vfs_path(&e.file_name) == norm
        }).ok_or_else(|| VfsError::FileNotFound(path.to_string()))?;

        let mut file_guard = self.file.lock().unwrap();
        let data = self.archive.extract_entry(&mut *file_guard, entry)?;
        Ok(data)
    }

    fn list_files(&self) -> Vec<String> {
        self.archive.entries.iter().map(|e| normalize_vfs_path(&e.path)).collect()
    }
}

/// Virtual File System managing prioritized mounts.
#[derive(Default)]
pub struct Vfs {
    mounts: Vec<Box<dyn MountSource>>,
}

impl Vfs {
    pub fn new() -> Self {
        Self { mounts: Vec::new() }
    }

    /// Add a mount source and sort by descending priority.
    pub fn add_mount(&mut self, source: Box<dyn MountSource>) {
        self.mounts.push(source);
        self.mounts.sort_by(|a, b| b.priority().cmp(&a.priority()));
    }

    /// Mount a loose directory.
    pub fn mount_dir<P: AsRef<Path>>(&mut self, name: &str, dir_path: P, priority: i32) -> Result<(), VfsError> {
        let p = dir_path.as_ref();
        if !p.is_dir() {
            return Err(VfsError::FileNotFound(format!("Directory does not exist: {}", p.display())));
        }
        self.add_mount(Box::new(DirMount::new(name.to_string(), p, priority)));
        Ok(())
    }

    /// Mount a CPK archive.
    pub fn mount_cpk<P: AsRef<Path>>(&mut self, cpk_path: P, priority: i32) -> Result<(), VfsError> {
        let cpk = CpkMount::open(cpk_path, priority)?;
        self.add_mount(Box::new(cpk));
        Ok(())
    }

    /// Mount standard .hack//G.U. game structure:
    /// - Common archives (cmn_a, cmn_a_pc, cmn_i, cmn_i_pc)
    /// - Volume-specific archives (volN_a, volN_a_pc, volN_i, volN_i_pc)
    /// - Optional mods directory (with each mod folder given priority >= 100)
    pub fn mount_game(&mut self, game_root: &Path, volume: u32, mods_dir: Option<&Path>) -> Result<usize, VfsError> {
        let cpk_dir = game_root.join("cpk");
        let mut mounted = 0;

        let pairs = [
            ("hackGU_cmn_a.cpk", 10),
            ("hackGU_cmn_a_pc.cpk", 20),
            ("hackGU_cmn_i.cpk", 30),
            ("hackGU_cmn_i_pc.cpk", 40),
            (&format!("hackGU_vol{}_a.cpk", volume), 50),
            (&format!("hackGU_vol{}_a_pc.cpk", volume), 60),
            (&format!("hackGU_vol{}_i.cpk", volume), 70),
            (&format!("hackGU_vol{}_i_pc.cpk", volume), 80),
        ];

        for (fname, prio) in pairs {
            let p = cpk_dir.join(fname);
            if p.is_file() {
                self.mount_cpk(&p, prio)?;
                mounted += 1;
            }
        }

        // Mount mods directory if present
        if let Some(mdir) = mods_dir {
            if mdir.is_dir() {
                if let Ok(entries) = fs::read_dir(mdir) {
                    let mut mod_prio = 100;
                    for e in entries.flatten() {
                        let path = e.path();
                        let name = e.file_name().to_string_lossy().to_string();
                        if path.is_dir() {
                            let _ = self.mount_dir(&format!("Mod: {}", name), &path, mod_prio);
                            mounted += 1;
                            mod_prio += 10;
                        } else if path.extension().and_then(|ext| ext.to_str()).map_or(false, |ext| ext.eq_ignore_ascii_case("cpk")) {
                            let _ = self.mount_cpk(&path, mod_prio);
                            mounted += 1;
                            mod_prio += 10;
                        }
                    }
                }
            }
        }

        Ok(mounted)
    }

    /// Check if a file exists in any mounted layer.
    pub fn exists(&self, path: &str) -> bool {
        self.mounts.iter().any(|m| m.has_file(path))
    }

    /// Read file content from the highest-priority mounted layer that contains it.
    pub fn read(&self, path: &str) -> Result<Vec<u8>, VfsError> {
        for mount in &self.mounts {
            if mount.has_file(path) {
                return mount.read_file(path);
            }
        }
        Err(VfsError::FileNotFound(path.to_string()))
    }

    /// Return which mount provides this file and its priority.
    pub fn resolve_source(&self, path: &str) -> Option<(&str, i32)> {
        for mount in &self.mounts {
            if mount.has_file(path) {
                return Some((mount.name(), mount.priority()));
            }
        }
        None
    }

    /// Return all unique file paths across all mounted layers.
    pub fn list_all_files(&self) -> Vec<String> {
        let mut set = BTreeSet::new();
        for mount in &self.mounts {
            for f in mount.list_files() {
                set.insert(f);
            }
        }
        set.into_iter().collect()
    }

    /// Count mounted layers.
    pub fn mount_count(&self) -> usize {
        self.mounts.len()
    }
}

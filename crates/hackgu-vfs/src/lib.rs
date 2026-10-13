//! hackgu-vfs: Virtual File System for .hack//G.U. Last Recode.

pub mod vfs;

pub use vfs::{normalize_vfs_path, CpkMount, DirMount, MountSource, Vfs, VfsError};

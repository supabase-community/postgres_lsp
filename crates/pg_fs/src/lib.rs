//! # pg_fs

mod dir;
mod path;
mod fs;
mod interner;

pub use dir::ensure_cache_dir;
pub use path::PgLspPath;
pub use interner::PathInterner;

pub use fs::{
    AutoSearchResult, ErrorEntry, ConfigName, File, FileSystem, FileSystemDiagnostic,
    FileSystemExt, MemoryFileSystem, OpenOptions, OsFileSystem, TraversalContext, TraversalScope,
};

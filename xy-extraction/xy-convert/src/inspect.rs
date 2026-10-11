//! File inspection: sizes, magic hex dumps, hashes, timestamps — the data
//! behind the GUI's metadata panel and the `hex` viewer's header preview.

use std::fs;
use std::io;
use std::path::Path;
use std::time::SystemTime;

use crate::FileKind;
use sha2::{Digest, Sha256};

/// Everything the metadata panel shows for one file.
#[derive(Debug, Clone)]
pub struct FileInfo {
    pub path: std::path::PathBuf,
    pub size: u64,
    /// First 16 bytes as hex (`"42 43 48 00 …"`), shorter for small files.
    pub magic_hex: String,
    pub kind: FileKind,
    pub crc32: u32,
    pub fnv64: u64,
    pub sha256: Option<String>,
    pub modified: Option<SystemTime>,
}

/// Inspect a file. `with_hash` enables the (expensive, whole-file) SHA-256.
/// Kind detection runs on the full image: GameFreak PC/PF/PT containers
/// validate their entry table against the file length, so a short head
/// probe would misclassify them.
pub fn inspect(path: &Path, with_hash: bool) -> io::Result<FileInfo> {
    let mut f = fs::File::open(path)?;
    let meta = f.metadata()?;
    let mut head = vec![0u8; 0x200.min(meta.len() as usize).max(16)];
    let _ = std::io::Read::read(&mut f, &mut head); // short reads fine
    let magic_hex = head
        .iter()
        .take(16)
        .map(|b| format!("{b:02X}"))
        .collect::<Vec<_>>()
        .join(" ");

    let data = fs::read(path)?;
    let kind = crate::detect_kind(&data, path);

    let mut info = FileInfo {
        path: path.to_path_buf(),
        size: meta.len(),
        magic_hex,
        kind,
        crc32: 0,
        fnv64: 0,
        sha256: None,
        modified: meta.modified().ok(),
    };

    info.crc32 = crc32fast::hash(&data);
    info.fnv64 = xy_core::util::fnv1a64(&data);
    if with_hash {
        let mut h = Sha256::new();
        h.update(&data);
        info.sha256 = Some(h.finalize().iter().map(|b| format!("{b:02x}")).collect());
    }
    Ok(info)
}

/// Read a byte window of a file for hex viewing (clamped to EOF).
pub fn read_window(path: &Path, offset: u64, len: usize) -> io::Result<Vec<u8>> {
    use std::io::{Read, Seek, SeekFrom};
    let mut f = fs::File::open(path)?;
    f.seek(SeekFrom::Start(offset))?;
    let mut buf = vec![0u8; len];
    let n = f.read(&mut buf)?;
    buf.truncate(n);
    Ok(buf)
}

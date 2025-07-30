use std::{
    collections::HashSet,
    fs::File,
    io::Read,
    path::{Path, PathBuf},
    sync::LazyLock,
};

use sha2::{Digest, Sha256};

static MALWARE_HASHES: LazyLock<HashSet<String>> = LazyLock::new(|| {
    include_str!("../full-hash-sha256-aa.txt")
        .lines()
        .map(|line| line.to_string())
        .collect()
});

#[derive(Debug)]
pub enum ScanResult {
    Clean(PathBuf),
    Infected(PathBuf),
    Error(PathBuf, String),
}

pub fn scan_file(path: &Path) -> ScanResult {
    let mut file = match File::open(path) {
        Ok(f) => f,
        Err(e) => return ScanResult::Error(path.to_path_buf(), e.to_string()),
    };

    let mut hasher = Sha256::new();
    let mut buffer = [0u8; 8192];

    loop {
        match file.read(&mut buffer) {
            Ok(0) => break,
            Ok(n) => hasher.update(&buffer[..n]),
            Err(e) => return ScanResult::Error(path.to_path_buf(), e.to_string()),
        }
    }

    let hash = format!("{:x}", hasher.finalize());
    if MALWARE_HASHES.contains(&hash) {
        ScanResult::Infected(path.to_path_buf())
    } else {
        ScanResult::Clean(path.to_path_buf())
    }
}

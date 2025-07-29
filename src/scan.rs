use std::{collections::HashSet, fs::File, io::Read, path::Path, sync::LazyLock};

use sha2::{Digest, Sha256};

static MALWARE_HASHES: LazyLock<HashSet<String>> = LazyLock::new(|| {
    include_str!("../full-hash-sha256-aa.txt")
        .lines()
        .map(|line| line.to_string())
        .collect()
});

#[derive(Debug)]
pub enum ScanResult<'a> {
    Clean,
    Infected { file_path: &'a Path },
    Error(String),
}

pub fn scan_file(path: &Path) -> ScanResult {
    let mut file = match File::open(path) {
        Ok(t) => t,
        Err(err) => return ScanResult::Error(format!("Failed to open file: {err}")),
    };

    let mut hasher = Sha256::new();
    let mut buffer = [0u8; 8192]; // 8KB

    loop {
        match file.read(&mut buffer) {
            Ok(0) => break,
            Ok(n) => hasher.update(&buffer[..n]),
            Err(err) => return ScanResult::Error(format!("Failed to read file: {err}")),
        }
    }

    let hash_hex = format!("{:x}", hasher.finalize());
    if MALWARE_HASHES.contains(&hash_hex) {
        return ScanResult::Infected { file_path: path };
    }
    ScanResult::Clean
}

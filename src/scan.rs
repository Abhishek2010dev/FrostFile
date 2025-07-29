use std::{collections::HashSet, sync::LazyLock};

static MALWARE_HASHES: LazyLock<HashSet<String>> = LazyLock::new(|| {
    include_str!("../full-hash-sha256-aa.txt")
        .lines()
        .map(|line| line.to_string())
        .collect()
});

#[derive(Debug)]
pub enum ScanResult {
    Clean,
    Infected(String), // virus or signature name
    Error(String),    // IO or read error
}

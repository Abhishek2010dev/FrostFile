# 🛡️ Zora Antivirus

**Zora Antivirus** is a fast, lightweight, and user-friendly desktop antivirus application written entirely in **Rust** using `egui`. It scans files and directories for known malware using SHA-256 hash-based detection.

[Watch Zora Antivirus Demo](assets/Screencast_20250730_180517.webm)

---

## 🚀 Features

- ✅ Real-time logging during scan
- ✅ Fast parallel directory scanning using `rayon`
- ✅ SHA-256 hash matching against known malware
- ✅ Beautiful native GUI with `eframe` and `egui`
- ✅ Malware signature database powered by `LazyLock`
- ✅ Cross-platform (Windows, Linux, macOS)

---

## 📸 UI Overview

- 📄 **Scan File** — Select a file to scan manually.
- 📁 **Scan Directory** — Scan an entire folder recursively.
- ⚠️ **Infection Alert** — Instant visual feedback if any file is malicious.
- 📜 **Live Logs** — Filterable logs using `egui_logger`.

---

## 🧪 How It Works

1. Each file is hashed using **SHA-256**.
2. The hash is compared against a malware signature list located in [`full-hash-sha256-aa.txt`](./full-hash-sha256-aa.txt).
3. If a match is found, the file is flagged as **infected**.

---

## 🛠️ Requirements

- Rust (latest stable)
- `cargo` build tool
- OS: Windows, Linux, or macOS

---

## 🧰 Installation

### Clone & Run:

```bash
git clone https://github.com/Abhishek2010dev/zora-antivirus.git
cd zora-antivirus
cargo run --release

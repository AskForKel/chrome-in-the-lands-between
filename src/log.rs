//! Our own log file, next to the game's executable. This is the oracle for in-game runs.

use std::fs::OpenOptions;
use std::io::Write;
use std::path::PathBuf;
use std::sync::{Mutex, OnceLock};

static SINK: OnceLock<Mutex<std::fs::File>> = OnceLock::new();

pub fn init() {
    let path = log_path();
    if let Ok(file) = OpenOptions::new().create(true).append(true).open(&path) {
        let _ = SINK.set(Mutex::new(file));
    }
    line("");
    line("=== Chrome in the Lands Between v0.1 ===");
}

pub fn log_path() -> PathBuf {
    let base = std::env::current_exe()
        .ok()
        .and_then(|p| p.parent().map(|d| d.to_path_buf()))
        .unwrap_or_else(|| PathBuf::from("."));
    base.join("ChromeInTheLandsBetween.log")
}

pub fn line(msg: &str) {
    if let Some(sink) = SINK.get() {
        if let Ok(mut file) = sink.lock() {
            let _ = writeln!(file, "{msg}");
            let _ = file.flush();
        }
    }
}

use std::path::{Path, PathBuf};

const DEFAULT_EXPORT_DIR: &str = "~/Download";
const EXPORT_DIR_ENV: &str = "WORKLOGGER_EXPORT_DIR";

pub fn export_dir() -> PathBuf {
    expand_tilde(
        &std::env::var(EXPORT_DIR_ENV).unwrap_or_else(|_| DEFAULT_EXPORT_DIR.to_string()),
    )
}

fn expand_tilde(path: &str) -> PathBuf {
    if let Some(rest) = path.strip_prefix("~/") {
        if let Some(home) = std::env::var_os("HOME") {
            return PathBuf::from(home).join(rest);
        }
    } else if path == "~" {
        if let Some(home) = std::env::var_os("HOME") {
            return PathBuf::from(home);
        }
    }
    PathBuf::from(path)
}

pub fn write_export_file(dir: &Path, filename: &str, bytes: &[u8]) -> std::io::Result<PathBuf> {
    std::fs::create_dir_all(dir)?;
    let path = dir.join(filename);
    std::fs::write(&path, bytes)?;
    Ok(path)
}

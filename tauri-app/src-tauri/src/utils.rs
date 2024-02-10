use std::fs;
use std::path::Path;
use serde::de::Error;
use tauri::InvokeError;

pub fn file_exists(path: &str) -> bool {
    Path::new(path).exists()
}

pub fn is_kml_file(path: &str) -> bool {
    Path::new(path)
        .extension()
        .map_or(false, |ext| ext.eq_ignore_ascii_case("kml"))
}

pub fn ensure_dir_exists(path: &str) -> anyhow::Result<()> {
    let path = Path::new(path);

    if !path.exists() {
        // 创建目录，包括所有必需的父目录
        fs::create_dir_all(path)?;
    }

    Ok(())
}

pub fn to_invoke_err(e: anyhow::Error) -> InvokeError {
    InvokeError::from_anyhow(e)
}

pub fn new_invoke_err(msg: &str) -> InvokeError {
    InvokeError::from_serde_json(serde_json::Error::custom(msg))
}

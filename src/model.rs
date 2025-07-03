use anyhow::Result;
use std::path::{Path, PathBuf};
use std::fs;
use std::io::Write;
use thiserror::Error;
use std::process::Command;

#[derive(Error, Debug)]
pub enum ModelError {
    #[error("I/O error: {0}")]
    IoError(#[from] std::io::Error),
    #[error("Download error: {0}")]
    DownloadError(String),
    #[error("Invalid model path: {0}")]
    InvalidPath(String),
}

pub fn ensure_yolo_model(model_dir: &Path) -> Result<PathBuf, ModelError> {
    fs::create_dir_all(model_dir)?;
    
    let model_path = model_dir.join("yolov8n-face.onnx");
    
    if model_path.exists() {
        println!("YOLO face detection model already exists at: {}", model_path.display());
        return Ok(model_path);
    }
    
    println!("Downloading YOLOv8 face detection model...");
    
    // Create a simple Python script to download the model
    let script_path = model_dir.join("download_model.py");
    let mut script_file = fs::File::create(&script_path)?;
    
    write!(script_file, r#"
import requests
import os
import sys

def download_file(url, local_path):
    with requests.get(url, stream=True) as r:
        r.raise_for_status()
        with open(local_path, 'wb') as f:
            for chunk in r.iter_content(chunk_size=8192):
                f.write(chunk)
    return local_path

# YOLOv8n face detection model (smaller and faster than YOLOv11 but efficient for face detection)
MODEL_URL = "https://github.com/akanametov/yolov8-face/releases/download/v0.0.0/yolov8n-face.onnx"
LOCAL_PATH = os.path.join('{}', 'yolov8n-face.onnx')

try:
    print(f"Downloading model from {{MODEL_URL}}...")
    download_file(MODEL_URL, LOCAL_PATH)
    print(f"Model downloaded successfully to {{LOCAL_PATH}}")
    sys.exit(0)
except Exception as e:
    print(f"Error downloading model: {{e}}")
    sys.exit(1)
"#, model_dir.display())?;

    // Run the Python script to download the model
    let output = Command::new("python")
        .arg(&script_path)
        .output()
        .map_err(|e| ModelError::DownloadError(format!("Failed to run Python: {}", e)))?;
        
    if !output.status.success() {
        let error_message = String::from_utf8_lossy(&output.stderr);
        return Err(ModelError::DownloadError(format!(
            "Model download script failed: {}",
            error_message
        )));
    }

    println!("Model downloaded successfully");
    
    // Clean up the script
    let _ = fs::remove_file(script_path);
    
    if !model_path.exists() {
        return Err(ModelError::InvalidPath(format!(
            "Expected model file not found after download: {}",
            model_path.display()
        )));
    }
    
    Ok(model_path)
}

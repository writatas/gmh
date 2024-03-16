#![allow(dead_code)]

use std::fs;
use std::path::Path;
use std::process::{Command, Stdio};
use anyhow::{Result, Error};

pub fn install_whisper_cpp_model(path_to_dir: &str) -> Result<(), Error> {
    let whisper_path = Path::new(path_to_dir);

    // Check if the Whisper path exists
    if !whisper_path.exists() {
        return Ok(());
    }

    let whisper_dir = whisper_path.join("whisper.cpp");
    // Clone the Whisper.cpp repository
    let whisper_url = "https://github.com/ggerganov/whisper.cpp.git";
    let _ = Command::new("git")
        .args(&["clone", whisper_url, &whisper_dir.display().to_string().as_str()])
        .stdout(Stdio::null())
        .stderr(Stdio::null())
        .status();

    // Build the Whisper.cpp project
    let mut make_command = Command::new("make");
    if std::env::consts::OS == "windows" {
        make_command.arg("Makefile.win");
    }
    let _ = make_command.current_dir(&whisper_dir).status();

    // Create the Whisper directory if it doesn't exist
    let _ = fs::create_dir_all(&whisper_dir);

    // Download the model (Linux-specific)
    if std::env::consts::OS == "linux" {
        let model_path = &whisper_dir.join("models/download-ggml-model.sh").display().to_string();
        let _ = Command::new("bash")
            .args(&[model_path, "base.en"])
            .current_dir(&whisper_path)
            .stdout(Stdio::null())
            .stderr(Stdio::null())
            .status();
    }

    Ok(())
}

use whisper_rs_2;
use std::fs;
use std::path::Path;
use std::io::Write;
use reqwest::blocking::Client;
use anyhow::{Result, Error};

// install whisper to folder
pub fn install_whisper_cpp_model(path_to_dir: &str) -> Result<(), Error> {
    let whisper_path = Path::new(path_to_dir);
    println!("Reads that path exists: {}", &whisper_path.exists());
    //if whisper_path.exists() {
        // Create the installation directory
        fs::create_dir_all(&whisper_path)?;

        // Download whisper.cpp
        let whisper_url = "https://github.com/ggerganov/whisper.cpp/raw/main/whisper.cpp";
        let whisper_content = Client::new().get(whisper_url).send()?.text()?;
        let whisper_file_path = whisper_path.join("whisper.cpp");
        println!("whisper path : {:#?}", &whisper_file_path.display());
        let mut whisper_file = fs::File::create(&whisper_file_path)?;
        whisper_file.write_all(whisper_content.as_bytes())?;

        // Download the model (base.en)
        let model_url = "https://ggml.ggerganov.com/models/base.en";
        let model_content = Client::new().get(model_url).send()?.bytes()?;
        let model_file_path = whisper_path.join("base.en");
        let mut model_file = fs::File::create(&model_file_path)?;
        model_file.write_all(&model_content)?;

        // Additional steps for Linux (if needed)
        if std::env::consts::OS == "linux" {
            // Download any additional files or perform specific setup
            // For example, you can download other models or dependencies.
        }
    //}
    Ok(())
}
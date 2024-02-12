use std::fs::{OpenOptions, File};
use std::io::Write;
use std::path::Path;

// If the file exists append to the existing text, otherwise create a new text file with the given path
pub fn save_txt(path: &str, text: &str) -> std::io::Result<String> {
    let path_check = Path::new(&path).exists();
    if !path_check {
        let mut file = File::create(&path)?;
        file.write_all(text.as_bytes())?;
        return Ok("File saved.".to_string())
    }
    else {
        let mut existing_file = OpenOptions::new()
            .append(true)
            .open(&path.clone())
            .expect("Cannot open file");

        existing_file
            .write(text.as_bytes())
            .expect("Write failed");
        
        return Ok("File saved".to_string())
    }

}
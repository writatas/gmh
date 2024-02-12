use std::fs::remove_file;

pub fn remove_txt(path: &str) -> std::io::Result<String> {
    remove_file(path)?;
    Ok("Deleted  txt file successfully.".to_string())
}
use std::{fs, path::Path};
use anyhow::{Error, Ok, anyhow};

pub fn create_folder_in_root(root_path: &str, name: &str) -> Result<String, Error> {
    let root = Path::new(&root_path);
    let new_folder = Path::new(&name);
    let end_path = root.join(new_folder);
    match root.is_dir() {
        true => {
            let _ = fs::create_dir_all(&end_path);
        },
        false => {
            return Err(anyhow!("Root folder does not exist!"));
        },
    };
    Ok(format!("{} created successfully", end_path.to_str().unwrap()))
}

// Create a new root folder. Root folders are opened as seperate work files essentially
pub fn create_root(root_path: &str) -> Result<String, Error> {
    let root = Path::new(&root_path);
    match root.is_dir() {
        true => return Err(anyhow!("Can only create new directories to define root!")),
        false => {
            let _ = fs::create_dir_all(&root);
        },
    };
    Ok(format!("{} created successfully [ROOT FILE]", root.to_str().unwrap()))
}
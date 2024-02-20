use std::{fs, path::Path};
use anyhow::{Error, Ok, anyhow};

//delete entire contents of folder
pub fn delete_root_folder (target_name: &str) -> Result<String, Error> {
    let folder_path = Path::new(&target_name);
    if !folder_path.is_dir() {
        return Err(anyhow!("Path provided is not a directory or does not exist."))
    }
    else {
        fs::remove_dir_all(&folder_path)?;
    }
    Ok(format!("Successfully deleted root folder {}", &folder_path.to_str().unwrap()))
}

use std::{error, fs, path::Path};
use anyhow::{Error, Ok};

// Root folders are parent folders that will be used to demarcate seperate text entities.
pub fn create_folder_from_root_folder(root_path: &str, name: &str) -> Result<String, Error> {
    let root = Path::new(&root_path);
    let new_folder = Path::new(&name);
    let end_path = root.join(new_folder);


    match root.exists() {
        true => {
            let _ = fs::create_dir_all(&end_path);
        },
        false => {
            return  Err(
                anyhow::Error::new(error)
                    .context(
                        format!("Root file {} does not exist.",
                        &root.to_str()
                            .unwrap())
                )
            );
        },
    };
    Ok(format!("{} created successfully", end_path.to_str().unwrap()))
}
use std::{fs::{self, ReadDir}, io::Read, path::Path};
use anyhow::{Error, Ok, anyhow};

pub fn convert_folder_to_text_file(folder: &str, destination_folder:&str, keep_original_folder: bool) -> Result<String, Error> {
    let folder_path = Path::new(&folder);
    let destination_path = Path::new(&destination_folder);
    let text_paths = get_text_paths(&folder_path.to_str().unwrap())?;
    let concatenated_txt = concatenate_txt(text_paths)?;

    if folder_path.is_dir() && keep_original_folder {
        let new_text_file_path = folder_path.join(Path::new(format!("{}.txt",&folder).as_str()));
        fs::write(new_text_file_path, concatenated_txt)?;
        return Ok(format!("Successfully converted contents of folder {} into text file {}.txt", &folder, &folder))
    }
    else if folder_path.is_dir() && !keep_original_folder && destination_path.is_dir(){
        let new_text_file_path = destination_path.join(Path::new(format!("{}.txt",&destination_folder).as_str()));
        fs::write(new_text_file_path, concatenated_txt)?;
        return Ok(format!("Successfully converted contents of folder {} into text file {}.txt, saved in folder {}", &folder, &folder, &destination_folder))
    }
    else {
        Err(anyhow!("Unable to convert contents of folder into a singular text file. Contents of folder must only contain text files."))
    }
}

//Gathers the text contents of folder.
pub fn get_text_paths(folder_path: &str) -> Result<ReadDir, Error> {
    let folder_path = Path::new(&folder_path);
    if !folder_path.is_dir() {
        return Err(anyhow!(format!("Folder path {} does not exist or isn't a directory", &folder_path.to_str().unwrap())))
    }
    let text_paths = fs::read_dir(&folder_path)?;
    Ok(text_paths)  
}
// Concatenates the texts contents of folders given their paths in a vector.
pub fn concatenate_txt(folder_read: ReadDir) -> Result<String, Error> {
    let mut result: String = String::new();
    for f in folder_read {
        let f_path = f.unwrap().path();
        if f_path.is_file() {
            let mut text = fs::File::open(&f_path)?;
            let mut text_buf = String::new();
            text.read_to_string(&mut text_buf)?;
            result.push_str("\n\n");
            result.push_str(&text_buf);
        }
        else {
            return Err(anyhow!(format!("Path is not a basic file type {}", &f_path.display())))
        }
    }
    Ok(result)
}
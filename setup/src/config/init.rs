use std::{collections::HashMap, env::consts::OS, path::{Path, PathBuf}};
use anyhow::{anyhow, Error, Ok};
use std::env;

// return master directory that will hold everything from the whisper installer, to program files
fn get_gmh_master_dir_path() -> Result<String, Error>{
    let gmh_master_dir = Path::new("'Game Master Helper'");
    if OS == "windows" {
        let path = Path::new("C:\\'Program Files'");
        if path.exists() && path.is_absolute() && path.is_dir(){
            let new_path = path.join(&gmh_master_dir);
            return Ok(new_path.display().to_string())
        }
    }
    else if OS == "linux" {
        let path = Path::new("/home");
        let user = env::var("USER").unwrap();
        let path = path.join(user);
        if path.exists() && path.is_absolute() && path.is_dir(){
            let new_path = path.join(&gmh_master_dir);
            return Ok(new_path.display().to_string())
        }
    }
    else {
        return Err(anyhow!("Unsupported OS!"));
    }
    Err(anyhow!("Could not find program folder. /home for linux or c:\\Program Files for windows"))
}

pub fn get_auxilliary_paths() -> Result<HashMap<&'static str, PathBuf>, Error> {
    let mut dir_paths = HashMap::new();
    let master_dir = Path::new(&get_gmh_master_dir_path()?)
        .to_owned();
    let whisper_path = &master_dir
        .clone()
        .join("whisper".to_string());
    // each folder in this directory gets treated as it's own text entity. any directory that
    //falls under gmh_roots contains text files that represent that own entity. like if I had a
    // character story the folder path would be gmh_folder/character/text.txt
    let root_path = &master_dir
        .clone()
        .join("gmh_roots".to_string());
    let dotfile_path = &master_dir
        .clone()
        .join(".paths".to_string());
    let dnd_api_path = Path::new("https://www.dnd5eapi.co/api/").to_owned();
    let audio_file_path = &master_dir
        .clone()
        .join("audio".to_string());
    let log_file_path = &master_dir
        .clone()
        .join("logfile.txt".to_string());

    dir_paths.insert("WHISPER_PATH",whisper_path.to_owned());
    dir_paths.insert("GMH_ROOTS_PATH",root_path.to_owned());
    dir_paths.insert("DOTFILE_PATH",dotfile_path.to_owned());
    dir_paths.insert("DND5E_API", dnd_api_path.to_owned());
    dir_paths.insert("AUDIO_PATH", audio_file_path.to_owned());
    dir_paths.insert("LOGFILE_PATH", log_file_path.to_owned());

    // if paths already exist, return an error to be handled
    if !master_dir.is_dir() && 
    !whisper_path.is_dir() && 
    !root_path.is_dir() &&
    !dotfile_path.is_file() &&
    !audio_file_path.is_dir() &&
    !log_file_path.is_file() {
        return Ok(dir_paths)
    } 
    else if master_dir.is_dir() && 
    whisper_path.is_dir() && 
    root_path.is_dir() &&
    dotfile_path.is_file() &&
    audio_file_path.is_dir() &&
    log_file_path.is_file(){
        return  Ok(dir_paths)
    }
    else {
        Err(anyhow!("Could not retrieve or create file paths."))
    }

}


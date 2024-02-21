use std::{fs, path::Path, env, env::consts::OS};
use anyhow::{anyhow, Error, Ok};
use core::result::Result::Ok as Ok_val;

pub fn set_configurations () {
    let gmh_files_path = check_os();
    match gmh_files_path {
        Ok_val(p) => println!("{}", p),
        Err(e) => println!("{}", e)
    }
}

// return directories of windows and linux for program files
fn check_os() -> Result<String, Error>{
    if OS == "windows" {
        let path = Path::new("C:\\'Program Files'");
        if path.exists() && path.is_absolute() && path.is_dir(){
            return Ok(path.display().to_string())
        }
    }
    else if OS == "linux" {
        let path = Path::new("/home");
        if path.exists() && path.is_absolute()&& path.is_dir(){
            return Ok(path.display().to_string())
        }
    }
    else {
        return Err(anyhow!("Unsupported OS!"));
    }
    Err(anyhow!("Could not find program folder. /home for linux nor c:\\Program Files for windows"))
}
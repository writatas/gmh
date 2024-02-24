use setup::*;
use folder_management::*;
use anyhow::{anyhow, Error, Ok};
fn main() {
    let setup_folders = folder_setup().expect_err("Error: ");
    println!("{:#?}", setup_folders);

}

// Set up required folders and install of whisper.cpp files
fn folder_setup() -> Result<(), Error> {
    let paths = get_auxilliary_paths()?;
    let gmh_path = match paths.get("GMH_ROOTS_PATH") {
        Some(p) => {
            let p = p.as_path().display().to_string();
            create_root(&p.as_str())?;
            p
        },
        None => "bad path".to_string()
    };
    let whisper_path = match paths.get("WHISPER_PATH") {
        Some(p) => {
            let p = p.as_path().display().to_string();
            create_root(&p.as_str())?;
            p
        },
        None => "bad path".to_string()
    };
    match paths.get("DOTFILE_PATH") {
        Some(p) => {
            let p = p.as_path().display().to_string();
            save_txt(p.as_str(), format!("DOTFILE_PATH={}\n", &p).as_str())?;
            save_txt(p.as_str(), format!("GMH_ROOTS_PATH={}\n", &gmh_path).as_str())?;
            save_txt(p.as_str(), format!("WHISPER_PATH={}\n", &whisper_path).as_str())?;
        },
        None => ()
    };
    
    Ok(())
}

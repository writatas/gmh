use folder_management::*;
use live_audio_transcription::*;
use setup::*;
use anyhow::{Ok, Error};
use std::{collections::HashMap, path::{Path, PathBuf}};
use log::{error, LevelFilter};
use simple_logging::log_to_file;

pub fn setup_logger(path_to_logfile: &str) {
    log_to_file(path_to_logfile, LevelFilter::Info).expect("Failed to initialize logger");
}

pub fn create_folders() -> Result<HashMap<&'static str, PathBuf>, Error> {
    let paths = get_auxilliary_paths()?;
    let gmh_path = match paths.get("GMH_ROOTS_PATH") {
        Some(p) => {
            let p = p.as_path().display().to_string();
            let gmh_path = Path::new(&p);
            if !gmh_path.exists() {
                create_root(&p.as_str())?;
            }
            p
        },
        None => "bad path".to_string()
    };
    let whisper_path = match paths.get("WHISPER_PATH") {
        Some(p) => {
            let p = p.as_path().display().to_string();
            let whisper_path = Path::new(&p);
            if !whisper_path.exists() {
                create_root(&p.as_str())?;
                install_whisper_cpp_model(&p)?;
            }
            p
        },
        None => "bad path".to_string()
    };
    let dnd_api_path = match paths.get("DND5E_API") {
        Some(p) => {
            let p = p.as_path().display().to_string();
            p
        },
        None => "bad path".to_string()
    };
    let audio_path = match paths.get("AUDIO_PATH") {
        Some(p) => {
            let p = p.as_path().display().to_string();
            let audio_path = Path::new(&p);
            if !audio_path.exists() {
                create_root(&p.as_str())?;
            }
            p
        },
        None => "bad path".to_string()
    };
    let logfile_path = match paths.get("LOGFILE_PATH") {
        Some(p) => {
            let p = p.as_path().display().to_string();
            let logfile_path = Path::new(&p);
            if !logfile_path.exists() {
                save_txt(&p.as_str(), "")?;
            }
            p
        },
        None => "bad path".to_string()
    };

    match paths.get("DOTFILE_PATH") {
        Some(p) => {
            let dotfile_path = Path::new(&p);
            if !dotfile_path.exists() {
                let p = p.as_path().display().to_string();
                save_txt(p.as_str(), format!("DOTFILE_PATH={}\n", &p).as_str())?;
                save_txt(p.as_str(), format!("GMH_ROOTS_PATH={}\n", &gmh_path).as_str())?;
                save_txt(p.as_str(), format!("WHISPER_PATH={}\n", &whisper_path).as_str())?;
                save_txt(p.as_str(), format!("DND5E_API={}\n", &dnd_api_path).as_str())?;
                save_txt(p.as_str(), format!("AUDIO_PATH={}\n", &audio_path).as_str())?;
                save_txt(p.as_str(), format!("LOGFILE_PATH={}\n", &logfile_path).as_str())?;
            }
        },
        None => ()
    };

    Ok(paths)
}
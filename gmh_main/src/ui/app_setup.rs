use folder_management::*;
use live_audio_transcription::*;
use setup::*;
use anyhow::{Ok, Error};
use std::{collections::HashMap, path::{Path, PathBuf}};
use log::{error, LevelFilter};
use simple_logging::log_to_file;
use std::sync::{Arc, Mutex};
use eframe::egui;

fn setup_logger(path_to_logfile: &str) {
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


// For ui implementations
pub struct MainWindow {
    created_folders: Vec<String>,
    created_texts: Vec<String>,
    live_texts: HashMap<String, String>,
    text_buffer: String,
    transcribed_text: String,
    whisper_path: String,
    gmh_roots_path: String,
    log_file_path: String,
    audio_path: String,
    transcribed_text_path: String,
    recording: Arc<Mutex<bool>>
}

impl Default for MainWindow {
    fn default() -> Self {
        let created_folders = Vec::new();
        let created_texts = Vec::new();
        let live_texts = HashMap::<String, String>::new();
        let text_buffer = String::from("");
        let transcribed_text = String::from("");

        let get_folders = create_folders().expect("Could not create folders");
        let log_file_path = get_folders.get("LOGFILE_PATH")
            .expect("Could not get logfile path")
            .display()
            .to_string();

        setup_logger(&log_file_path.as_str());
        let whisper_path = get_folders.get("WHISPER_PATH")
            .expect("Could not retrieve path to whisper.ccp model")
            .display()
            .to_string();
        let audio_path = get_folders.get("AUDIO_PATH")
            .expect("Could not retrieve audio folder path")
            .display()
            .to_string();
        let transcribed_text_path = Path::new(&audio_path)
            .join("transcribed.txt")
            .display()
            .to_string();
        let gmh_roots_path = get_folders.get("GMH_ROOTS_PATH")
            .expect("Could not get GMH_ROOTS path")
            .display()
            .to_string();
        let log_file_path = log_file_path.clone().to_string();
        let recording = Arc::new(Mutex::new(false));


        Self {
            created_folders,
            created_texts,
            live_texts,
            text_buffer,
            transcribed_text,
            whisper_path,
            gmh_roots_path,
            log_file_path,
            audio_path,
            transcribed_text_path,
            recording
        }
    }
}

impl eframe::App for MainWindow {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.strong(&self.transcribed_text_path);
        });
    }
}

pub fn start_desktop_app() -> Result<(), eframe::Error>
{
    let options = eframe::NativeOptions::default();
    eframe::run_native(
    "Game Master Helper",
    options,
    Box::new(|_cc| Box::new(MainWindow::default())),
    )
}
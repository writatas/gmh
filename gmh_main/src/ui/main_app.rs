use std::sync::{Arc, Mutex};
use eframe::egui;
use folder_management::*;
use live_audio_transcription::*;
use gmh_main::ui::app_setup::*;


pub struct MainWindow {
    created_folders: Vec<String>,
    created_texts: Vec<String>,
    live_texts: Vec<String>,
    text_buffer: String,
    transcribed_text_path: String,
    transcribed_text: String,
    whisper_path: String,
    gmh_roots_path: String,
    log_file_path: String,
    recording: Arc<Mutex<bool>>
}

impl Default for MainWindow {
    fn default() -> Self {
        // initialize the logger
        setup_logger(log_file_path);
        let get_folders = match create_folders() {
            Err(err) => err,
            Ok(v) => v
        };
    }
}
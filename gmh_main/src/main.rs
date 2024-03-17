use std::sync::{Arc, Mutex};

use folder_management::save_txt;
use gmh_main::create_folders;
use live_audio_transcription::*;
fn main() {
    // Create folders if they do not exist, returns paths if they do already
    let paths = create_folders().unwrap();
    let audio_path = paths.get("AUDIO_PATH").unwrap();
    let whisper_path = paths.get("WHISPER_PATH").unwrap().join("whisper.cpp/models/ggml-base.en.bin");
    let audio_path = audio_path.join("test.wav").display().to_string();
    
    let recording_bool = Arc::new(Mutex::new(true));
    // Wrap save_txt in a closure and pass it as the callback
    let save_txt_closure = |file_name: &str, text: &str| {
        save_txt(file_name, text)
    };
    continuous_recording(
        &audio_path, 
        &whisper_path.display().to_string().as_str(),
        recording_bool,
        save_txt_closure
    ).unwrap();
}




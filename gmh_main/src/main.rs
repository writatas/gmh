use gmh_main::create_folders;
use live_audio_transcription::*;
use std::thread;
fn main() {
    // Create folders if they do not exist, returns paths if they do already
    let paths = create_folders().unwrap();
    let audio_path = paths.get("AUDIO_PATH").unwrap();
    let whisper_path = paths.get("WHISPER_PATH").unwrap().join("whisper.cpp/models/ggml-base.en.bin");
    let audio_path = audio_path.join("test.wav").display().to_string();
    let mut counter = 0;
    let mut result_txt = "".to_string();
    
    loop {
        record_audio(&audio_path).unwrap();
        let transcribed = transcribe_audio_file(&audio_path, whisper_path.display().to_string().as_str());
        result_txt += transcribed.as_str();
        counter += 1;
        println!("Count {}", counter);
        if counter == 5 {
            break;
        }
    }
    println!("RESULT TEXT: \n\n{}",result_txt);
}




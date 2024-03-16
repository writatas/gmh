use gmh_main::create_folders;
use live_audio_transcription::*;
fn main() {
    // Create folders if they do not exist, returns a readable error if they do already
    //let paths = create_folders().unwrap();
    //let whisper_path = paths.get("WHISPER_PATH").unwrap();

    //let r = record(whisper_path.display().to_string().as_str()).unwrap();
    let _ = record_audio().unwrap();

}




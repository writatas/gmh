use gmh_main::create_folders;
use live_audio_transcription::*;
use std::env;
fn main() {
    // Create folders if they do not exist, returns a readable error if they do already
    create_folders().unwrap();
    record(whispercpp_path)
}


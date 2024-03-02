use folder_management::*;
use setup::*;
use anyhow::{Ok, Error};
use live_audio_transcription::*;

fn main() {
    create_folders().unwrap();
    install_whisper_cpp_model("/home/writatas/'Game Master Helper'/whisper").expect("msg");
    //create_folders().unwrap();

}

fn create_folders() -> Result<(), Error> {
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
    println!("{}", whisper_path);
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
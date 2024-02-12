use std::fs::File;
use std::io::prelude::*;

//A simple function to retrieve text when given a path
pub fn get_txt(path: &str) -> std::io::Result<String> {
    let path = path.to_string();
    let mut file = File::open(path)?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    Ok(contents)
}
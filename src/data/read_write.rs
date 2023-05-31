use serde::Serializer;
use std::{fs, path::PathBuf};

use super::CompetitionData;

pub fn save_to_file(file_path: PathBuf, data: &CompetitionData) -> Result<(), String> {
    if let Some(parent) = file_path.parent() {
        match fs::create_dir_all(parent) {
            Ok(_) => (),
            Err(_) => return Err(String::from("Creation of parents dir failed!")),
        };
    }

    let writer = match fs::File::create(file_path) {
        Ok(file) => file,
        Err(_) => return Err(String::from("File could not be opend!")),
    };

    match serde_json::Serializer::new(writer).serialize_some(data) {
        Ok(()) => Ok(()),
        Err(err) => Err(err.to_string()),
    }
}

pub fn read_from_file(path: PathBuf) -> Result<CompetitionData, String> {
    let json_string = match fs::read_to_string(&path) {
        Ok(bytes) => bytes,
        Err(_) => return Err(format!("Error whilst reading file: {}", path.display())),
    };

    dbg!("{}", &json_string);

    match serde_json::from_str(&json_string) {
        Ok(competition_data) => Ok(competition_data),
        Err(_) => Err(String::from("JSON was not well-formatted")),
    }
}

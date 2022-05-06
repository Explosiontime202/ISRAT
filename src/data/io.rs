use serde::Deserialize;

use super::CompetitionData;

pub fn save_to_file(filename: String, data: &mut CompetitionData) {
    
}

pub fn read_from_file(filename: String) -> CompetitionData {
    CompetitionData::empty()
}

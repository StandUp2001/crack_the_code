use std::path::Path;

use crate::crack::Crack;

const FILE: &str = "./info.json";
pub struct CodesFile;

impl CodesFile {
    pub fn file_exists() -> bool {
        Path::new(FILE).exists()
    }

    pub fn create_file() {
        let path = Path::new(FILE);
        let mut contents = "[".to_string();
        for _ in 0..2 {
            contents += "{\"input\": [0,0,0,0....],\"correct_place\": 0,\"wrong_place\": 0},";
        }
        contents.pop();
        contents += "]";
        std::fs::write(path, contents).expect("Could not create file");
    }

    pub fn read_codes() -> Vec<Crack> {
        let path = Path::new(FILE);
        let contents = std::fs::read_to_string(path).expect("Could not read file");
        let codes: Vec<Crack> = serde_json::from_str(&contents).expect("Could not parse file");
        codes
    }
}

use std::fs;

pub struct InputReader;

impl InputReader {
    pub fn get_lines(&self, file_name: &str) -> Vec<String> {
        let contents = fs::read_to_string(file_name)
            .expect("unable to read input file");

        let split = contents.split("\n");
        split.map(|x| x.to_string()).collect()
    }
}
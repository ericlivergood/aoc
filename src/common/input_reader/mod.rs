mod tests;
use std::fs;
use array2d::{Array2D};

pub struct InputReader;

impl InputReader {
    pub fn get_lines(&self, file_name: &str) -> Vec<String> {
        let contents = fs::read_to_string(file_name)
            .expect("unable to read input file");

        let split = contents.split("\n");
        split.map(|x| x.to_string()).collect()
    }

    /*
           X 012
           Y ___
           0|123
           1|456
           2|789

     */
    pub fn to_2d_array(&self, lines: Vec<String>) -> Array2D<char> {
        let mut rows = Vec::new();
        for l in lines {
            rows.push(l.trim().chars().collect())
        }

        Array2D::from_rows(&rows)
            .expect("unable to create 2d array")
    }

    pub fn get_as_2d_array(&self, file_name: &str) -> Array2D<char>{
        let lines = self.get_lines(file_name);
        self.to_2d_array(lines)
    }
}
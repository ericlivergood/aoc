pub mod utils {
    pub fn list_to_ints(l: &str) -> Vec<i32> {
        l.split_whitespace()
            .map(|s| s.parse::<i32>().unwrap())
            .collect()
    }

    pub fn list_to_ints_by_char(l: &str, char: &str) -> Vec<i32> {
        l.split(char)
            .map(|s| s.parse::<i32>().unwrap())
            .collect()
    }

    pub fn list_to_i64_by_char(l: &str, char: &str) -> Vec<i64> {
        l.split(char)
            .map(|s| s.parse::<i64>().unwrap())
            .collect()
    }
}
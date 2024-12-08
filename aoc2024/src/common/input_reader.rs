pub mod input_reader {
    use std::fs;
    use array2d::{Array2D};

    pub fn get_lines(file_name: &str) -> Vec<String> {
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
    pub fn to_2d_array(lines: Vec<String>) -> Array2D<char> {
        let mut rows = Vec::new();
        for l in lines {
            rows.push(l.trim().chars().collect())
        }

        Array2D::from_rows(&rows)
            .expect("unable to create 2d array")
    }

    pub fn get_as_2d_array(file_name: &str) -> Array2D<char>{
        let lines = get_lines(file_name);
        to_2d_array(lines)
    }

    #[cfg(test)]
    mod tests {
        use super::*;
        #[test]
        pub fn builds_2d_array() {
            let mut lines = Vec::new();
            lines.push(String::from("123"));
            lines.push(String::from("456"));
            lines.push(String::from("789"));
            let a = to_2d_array(lines);


            assert_eq!(*a.get(0,0).unwrap(), '1');
            assert_eq!(*a.get(0,1).unwrap(), '2');
            assert_eq!(*a.get(0,2).unwrap(), '3');
            assert_eq!(*a.get(1,0).unwrap(), '4');
            assert_eq!(*a.get(1,1).unwrap(), '5');
            assert_eq!(*a.get(1,2).unwrap(), '6');
            assert_eq!(*a.get(2,0).unwrap(), '7');
            assert_eq!(*a.get(2,1).unwrap(), '8');
            assert_eq!(*a.get(2,2).unwrap(), '9');
        }
    }
}
#[cfg(test)]
mod tests {
    use crate::common::input_reader::InputReader;

    #[test]
    pub fn builds_2d_array() {
        let mut lines = Vec::new();
        lines.push(String::from("123"));
        lines.push(String::from("456"));
        lines.push(String::from("789"));
        let i = InputReader;
        let a = i.to_2d_array(lines);


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
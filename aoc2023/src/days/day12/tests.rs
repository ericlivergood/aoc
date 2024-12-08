#[cfg(test)]
mod tests {
    use crate::days::day12::record::Record;

    #[test]
    fn detects_matches() {
        let r = Record::new("#.#.### 1,1,3".to_string());
        assert_eq!(r.matches("#.#.###".to_string()), true);
        assert_eq!(r.matches("#...#...###".to_string()), true);
        assert_eq!(r.matches("##...###".to_string()), false);
    }

    #[test]
    fn generates_permutations() {
        let r = Record::new("#.#.### 1,1,3".to_string());
        assert_eq!(r.permutations().len(), 1);

        let r = Record::new("#.#.?### 1,1,3".to_string());
        assert_eq!(r.permutations().len(), 2);


        let r = Record::new("#.??#.### 1,1,3".to_string());
        assert_eq!(r.permutations().len(), 4);


        let r = Record::new("#.#.###??? 1,1,3".to_string());
        assert_eq!(r.permutations().len(), 8);
    }
}
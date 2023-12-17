#[cfg(test)]
mod tests {
    use crate::days::day15::hasher::hash;

    #[test]
    fn hashes() {
        assert_eq!(hash("H".to_string()), 200);
        assert_eq!(hash("HA".to_string()), 153);
        assert_eq!(hash("HAS".to_string()), 172);
        assert_eq!(hash("HASH".to_string()), 52);
        assert_eq!(hash("rn=1".to_string()), 30);
        assert_eq!(hash("cm-".to_string()), 253);
        assert_eq!(hash("qp=3".to_string()), 97);
        assert_eq!(hash("cm=2".to_string()), 47);
        assert_eq!(hash("qp-".to_string()), 14);
        assert_eq!(hash("pc=4".to_string()), 180);
        assert_eq!(hash("ot=9".to_string()), 9);
        assert_eq!(hash("ab=5".to_string()), 197);
        assert_eq!(hash("pc-".to_string()), 48);
        assert_eq!(hash("pc=6".to_string()), 214);
        assert_eq!(hash("ot=7".to_string()), 231);
    }
}
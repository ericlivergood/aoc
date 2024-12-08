#[cfg(test)]
mod tests {
    use crate::days::day5::{extract_map_sections, lines_to_map};

    #[test]
    fn parses_hash_maps() {
        let lines = vec![
            "50 98 2".to_string(),
            "52 50 2".to_string(),
        ];
        let map = lines_to_map(lines);
        assert_eq!(map.len(), 2);
        assert_eq!(map[0].start, 98);
        assert_eq!(map[0].end, 100);
        assert_eq!(map[0].offset, -48);
        assert_eq!(map[1].start, 50);
        assert_eq!(map[1].end, 52);
        assert_eq!(map[1].offset, 2);
    }

    #[test]
    fn extracts_sections() {
        let lines = vec![
            "test-1 map:",
            "a",
            "b",
            "test-2 map:",
            "c",
            "d",
            "e",
            "test-3 map:"
        ];

        let sections = extract_map_sections(lines);
        assert_eq!(sections.len(), 3);
        assert_eq!(sections["test-1"].lines.len(), 2);
        assert_eq!(sections["test-2"].lines.len(), 3);
        assert_eq!(sections["test-3"].lines.len(), 0);
    }
}
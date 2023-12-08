#[cfg(test)]
mod tests {
    use crate::common::input_reader::InputReader;
    use crate::days::day3::Schematic;

    #[test]
    fn gets_part_numbers() {
        let mut lines = Vec::new();
        lines.push(String::from("123....456"));
        lines.push(String::from("...78....."));
        let i = InputReader;
        let data = i.to_2d_array(lines);

        let s = Schematic {
            data
        };

        let parts = s.get_part_numbers();
        assert_eq!(parts.len(), 3);
        assert_eq!(parts[0].value, "123");
        assert_eq!(parts[1].value, "456");
        assert_eq!(parts[2].value, "78");
    }

    #[test]
    fn identifies_part_is_adjacent_to_symbol_on_same_line() {
        let mut lines = Vec::new();
        lines.push(String::from("&123..456..789&"));
        let i = InputReader;
        let data = i.to_2d_array(lines);

        let s = Schematic {
            data
        };

        let parts = s.get_part_numbers();
        assert_eq!(parts.len(), 3);
        let part123 = parts.iter().filter(|x| x.value == "123").next().unwrap();
        let part456 = parts.iter().filter(|x| x.value == "456").next().unwrap();
        let part789 = parts.iter().filter(|x| x.value == "789").next().unwrap();
        assert_eq!(s.is_adjacent_to_symbol(&part123), true);
        assert_eq!(s.is_adjacent_to_symbol(&part456), false);
        assert_eq!(s.is_adjacent_to_symbol(&part789), true);
    }

    #[test]
    fn identifies_part_is_adjacent_to_symbol_on_line_above() {
        let mut lines = Vec::new();
        lines.push(String::from("&....&....&....&&..."));
        lines.push(String::from(".12..34..45..67...89"));
        let i = InputReader;
        let data = i.to_2d_array(lines);

        let s = Schematic {
            data
        };

        let parts = s.get_part_numbers();
        let part12 = parts.iter().filter(|x| x.value == "12").next().unwrap();
        let part34 = parts.iter().filter(|x| x.value == "34").next().unwrap();
        let part45 = parts.iter().filter(|x| x.value == "45").next().unwrap();
        let part67 = parts.iter().filter(|x| x.value == "67").next().unwrap();
        let part89 = parts.iter().filter(|x| x.value == "89").next().unwrap();
        assert_eq!(s.is_adjacent_to_symbol(&part12), true);
        assert_eq!(s.is_adjacent_to_symbol(&part34), true);
        assert_eq!(s.is_adjacent_to_symbol(&part45), true);
        assert_eq!(s.is_adjacent_to_symbol(&part67), true);
        assert_eq!(s.is_adjacent_to_symbol(&part89), false);
    }

    #[test]
    fn identifies_part_is_adjacent_to_symbol_on_line_below() {
        let mut lines = Vec::new();
        lines.push(String::from(".12..34..45..67...89"));
        lines.push(String::from("&....&....&....&&..."));
        let i = InputReader;
        let data = i.to_2d_array(lines);

        let s = Schematic {
            data
        };

        let parts = s.get_part_numbers();
        let part12 = parts.iter().filter(|x| x.value == "12").next().unwrap();
        let part34 = parts.iter().filter(|x| x.value == "34").next().unwrap();
        let part45 = parts.iter().filter(|x| x.value == "45").next().unwrap();
        let part67 = parts.iter().filter(|x| x.value == "67").next().unwrap();
        let part89 = parts.iter().filter(|x| x.value == "89").next().unwrap();
        assert_eq!(s.is_adjacent_to_symbol(&part12), true);
        assert_eq!(s.is_adjacent_to_symbol(&part34), true);
        assert_eq!(s.is_adjacent_to_symbol(&part45), true);
        assert_eq!(s.is_adjacent_to_symbol(&part67), true);
        assert_eq!(s.is_adjacent_to_symbol(&part89), false);
    }
    #[test]
    fn identifies_gear() {
        let mut lines = Vec::new();
        lines.push(String::from("1...1...1."));
        lines.push(String::from("*..1*...*."));
        lines.push(String::from("1...1.*..."));
        let i = InputReader;
        let data = i.to_2d_array(lines);

        let s = Schematic {
            data
        };

        assert_eq!(s.is_gear(0,0), false);
        assert_eq!(s.is_gear(1,0), false);
        assert_eq!(s.is_gear(0,1), true);
        assert_eq!(s.is_gear(4,1), false);
        assert_eq!(s.is_gear(8,1), false);
        assert_eq!(s.is_gear(6,2), false);
    }

    #[test]
    fn gets_adjacent_part_numbers() {
        let mut lines = Vec::new();
        lines.push(String::from("1...2...3."));
        lines.push(String::from("*..4*...*."));
        lines.push(String::from("5...6.*..."));
        let i = InputReader;
        let data = i.to_2d_array(lines);

        let s = Schematic {
            data
        };

        let star01adjacents = s.get_part_numbers_adjacent_to(0, 1);
        assert_eq!(star01adjacents.len(), 2);


        let star41adjacents = s.get_part_numbers_adjacent_to(4, 1);
        assert_eq!(star41adjacents.len(), 3);


        let star81adjacents = s.get_part_numbers_adjacent_to(8, 1);
        assert_eq!(star81adjacents.len(), 1);


        let star62adjacents = s.get_part_numbers_adjacent_to(6, 2);
        assert_eq!(star62adjacents.len(), 0);
    }
}
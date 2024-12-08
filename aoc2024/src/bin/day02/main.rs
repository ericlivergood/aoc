use aoc2024::common::input_reader;
use aoc2024::common::utils;

fn is_next_safe(last: i32, cur: i32, next: i32) -> bool {
    if last == cur {
        return false;
    }

    if cur > last && next <= cur { // ascending
        return false;
    }
    if cur < last && next >= cur {
        return false;
    }
    if (next - cur).abs() > 3 || (last - cur).abs() > 3 {
        return false;
    }
    true
}
fn is_line_safe(l: &str) -> bool {
    let numbers = utils::utils::list_to_ints(l);
    is_safe(numbers)
}

fn is_safe(numbers: Vec<i32>) -> bool {
    let mut i = 2;
    while i < numbers.len() {
        let last = numbers[i-2];
        let cur = numbers[i-1];
        let next = numbers[i];
        if !is_next_safe(last, cur, next) {
            return false;
        }
        i +=1;
    }
    true
}



fn is_safe_without_one_value(l: &str) -> bool {
    let numbers = utils::utils::list_to_ints(l);
    if is_safe(numbers.clone()) {
        return true;
    }

    let mut i = 0;
    let len = numbers.len();
    while i < len {
        let mut cloned = numbers.clone();
        cloned.remove(i);
        if(is_safe(cloned)) {
            return true;
        }
        i += 1;
    }
    false
}

fn part1(lines: Vec<String>) {
    let mut cnt = 0;

    for line in lines {
        if is_line_safe(&line) {
            cnt += 1;
        }
    }
    println!("Part 1: {}", cnt);
}

fn part2(lines: Vec<String>) {
    let mut cnt = 0;

    for line in lines {
        if is_safe_without_one_value(&line) {
            cnt += 1;
        }
    }
    println!("Part 2: {}", cnt);
}

fn main() {
    let lines = input_reader::input_reader::get_lines("./data/day02/input");
    part1(lines.clone());
    part2(lines.clone());
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn is_safe_works() {
        assert_eq!(true, is_line_safe("7 6 4 2 1"));
        assert_eq!(false, is_line_safe("1 2 7 8 9"));
        assert_eq!(false, is_line_safe("9 7 6 2 1"));
        assert_eq!(false, is_line_safe("1 3 2 4 5"));
        assert_eq!(false, is_line_safe("8 6 4 4 1"));
        assert_eq!(true, is_line_safe("1 3 6 7 9"));
    }

    #[test]
    fn is_safe_without_one_value_works() {
        assert_eq!(true, is_safe_without_one_value("7 6 4 2 1"));
        assert_eq!(false, is_safe_without_one_value("1 2 7 8 9"));
        assert_eq!(false, is_safe_without_one_value("9 7 6 2 1"));
        assert_eq!(true, is_safe_without_one_value("1 3 2 4 5"));
        assert_eq!(true, is_safe_without_one_value("8 6 4 4 1"));
        assert_eq!(true, is_safe_without_one_value("1 3 6 7 9"));
    }

    #[test]
    fn is_next_safe_works() {
        assert_eq!(true, is_next_safe(1, 2,3));
        assert_eq!(true, is_next_safe(1, 2,4));
        assert_eq!(true, is_next_safe(1, 2,5));
        assert_eq!(false, is_next_safe(1, 2,6));

        assert_eq!(true, is_next_safe(30, 27,26));
        assert_eq!(true, is_next_safe(30, 27,25));
        assert_eq!(true, is_next_safe(30, 27,24));
        assert_eq!(false, is_next_safe(30, 27,23));

        assert_eq!(false, is_next_safe(1, 2,2));
        assert_eq!(false, is_next_safe(2, 2,4));

        assert_eq!(false, is_next_safe(6, 2,3));
        assert_eq!(false, is_next_safe(1, 2,1));
    }
}
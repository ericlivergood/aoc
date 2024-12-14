use aoc2024::common::input_reader;
use crate::machine::Machine;

mod machine;

fn part1(lines: &Vec<String>) -> i64 {
    let machines = Machine::from_lines(lines);
    let mut sum = 0;
    for m in machines {
        let result1 = m.solve();
        let result2 = m.solve_old();

        if result1 != result2 {
            panic!("old did not match new\n {:?}\n old:{:?}, new:{:?}", m, result2, result1);
        }

        match m.solve() {
            Some(n) => {
                sum += n.2
            },
            None => { }
        }
    }
    sum
}

fn part2(lines: &Vec<String>) -> i64 {
    let machines = Machine::from_lines(lines);
    let mut sum = 0;
    for mut m in machines {
        m.adjust_prize_location(10000000000000);
        m.set_press_limit(10000000000000);
        match m.solve() {
            Some(n) => {
                sum += n.2
            },
            None => { }
        }
    }
    sum
}

fn main() {
    let lines = input_reader::input_reader::get_lines("./data/day13/input");
    println!("part 1: {}", part1(&lines));
    println!("part 2: {}", part2(&lines));
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_part1(){
        let lines = input_reader::input_reader::get_lines("./data/day13/test");
        assert_eq!(part1(&lines), 480);
    }

    #[test]
    fn test_part2(){
        let lines = input_reader::input_reader::get_lines("./data/day13/test");
        assert!(part2(&lines) > 0);
    }
}
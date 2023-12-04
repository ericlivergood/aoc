use crate::common::input_reader;

mod tests;
pub struct Day3

impl Day2 {
    pub fn run(&self) {
        self.run_part_two();
    }
    fn run_part_one(&self) {
        let reader = input_reader::InputReader;
        let mut sum = 0;
        for line in reader.get_lines("/git/aoc23/src/days/day2/input") {
            let result = parse_line(line.as_str());

            let mut passed = true;
            for r in result.results {
                if r.red > 12 || r.green > 13 || r.blue > 14 {
                    passed = false;
                    break;
                }
            }

            if passed {
                sum += result.game_number;
            }
            // if result.red_count() <= 12 && result.green_count() <= 13 && result.blue_count() <= 14 {
            //     println!("Game {0}", result.game_number);
            //     sum += result.game_number;
            // }
        }

        println!("{sum}");
    }

    fn run_part_two(&self) {
        let reader = input_reader::InputReader;
        let mut sum = 0;
        for line in reader.get_lines("/git/aoc23/src/days/day2/input") {
            let result = parse_line(line.as_str());
            let max = result.max_green() * result.max_red() * result.max_blue();
            sum += max;
        }

        println!("{sum}");
    }
}
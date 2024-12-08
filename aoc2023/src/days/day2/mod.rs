use crate::common::input_reader;

mod tests;
pub struct Day2;

struct PieceCount {
    blue: u32,
    green: u32,
    red: u32
}

struct GameResult {
    game_number: u32,
    results: Vec<PieceCount>
}

impl GameResult {
    fn max_blue(&self) -> u32 {
        let mut max = 0;
        for r in self.results.iter() {
            if r.blue > max {
                max = r.blue;
            }
        }
        max
    }
    fn max_red(&self) -> u32 {
        let mut max = 0;
        for r in self.results.iter() {
            if r.red > max {
                max = r.red;
            }
        }
        max
    }
    fn max_green(&self) -> u32 {
        let mut max = 0;
        for r in self.results.iter() {
            if r.green > max {
                max = r.green;
            }
        }
        max
    }
}

fn parse_result(result: &str) -> (&str, u32) {
    let parts = result.trim().split(' ').collect::<Vec<&str>>();
    (
        parts[1],
        parts[0].trim().parse::<u32>().unwrap()
    )
}

fn parse_fragment(fragment: &str) -> PieceCount {
    let parts = fragment.split(',');
    let mut red = 0;
    let mut blue = 0;
    let mut green = 0;

    for p in parts {
        let result = parse_result(p);
        match result.0 {
            "blue" => blue = result.1,
            "red" => red = result.1,
            "green" => green = result.1,
            _=> panic!("unknown color")
        }
    }

    PieceCount{
        red,
        blue,
        green
    }
}

fn parse_line(line: &str) -> GameResult {
    let parts = line.split(':').collect::<Vec<&str>>();
    let game_number = parts[0].replace("Game ", "").parse::<u32>().unwrap();
    let mut results = Vec::<PieceCount>::new();

    for result in parts[1].split(';').collect::<Vec<&str>>() {
        results.push(parse_fragment(result))
    }

    GameResult {
        game_number,
        results
    }
}

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
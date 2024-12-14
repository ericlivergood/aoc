use regex::Regex;
    #[derive(Debug)]
    pub struct Machine {
        pub a_x_delta: i64,
        pub b_x_delta: i64,
        pub a_y_delta: i64,
        pub b_y_delta: i64,
        pub prize_x: i64,
        pub prize_y: i64,
        press_limit: i64
    }

pub fn get_deltas(line: &String) -> (i64,i64) {
    let re = Regex::new(r"X\+(\d+), Y\+(\d+)").unwrap();
    let caps = re.captures(line).unwrap();
    let x = caps.get(1).unwrap().as_str().parse::<i64>().unwrap();
    let y = caps.get(2).unwrap().as_str().parse::<i64>().unwrap();
    (x, y)
}

pub fn get_prize_location(line: &String) -> (i64, i64) {
    let re = Regex::new(r"X=(\d+), Y=(\d+)").unwrap();
    let caps = re.captures(line).unwrap();
    let x = caps.get(1).unwrap().as_str().parse::<i64>().unwrap();
    let y = caps.get(2).unwrap().as_str().parse::<i64>().unwrap();
    (x,y)
}

impl Machine {
   pub fn parse(lines: &[String]) -> Machine {
        if lines.len() != 3 {
            panic!("Expected three lines");
        }
        let a_deltas = get_deltas(&lines[0]);
        let b_deltas = get_deltas(&lines[1]);
        let prize = get_prize_location(&lines[2]);
        Machine {
            a_x_delta: a_deltas.0,
            a_y_delta: a_deltas.1,
            b_x_delta: b_deltas.0,
            b_y_delta: b_deltas.1,
            prize_x: prize.0,
            prize_y: prize.1,
            press_limit: 100
        }
    }

    pub fn from_lines(lines: &[String]) -> Vec<Machine> {
        let mut counter = 0;
        let mut machines = Vec::new();
        while counter < lines.len() {
            let l = vec![
                lines[counter].to_string(),
                lines[counter + 1].to_string(),
                lines[counter + 2].to_string(),
            ];
            machines.push(Machine::parse(&l));
            counter += 4;
        }
        machines
    }

    pub fn adjust_prize_location(&mut self, n: i64) {
        self.prize_x = self.prize_x + n;
        self.prize_y = self.prize_y + n;
    }

    pub fn set_press_limit(&mut self, n: i64) {
        self.press_limit = n;
    }

    fn compare(&self, a: i64, b: i64) -> Result<(), bool> {
        let x = a * self.a_x_delta + b * self.b_x_delta;
        let y = a * self.a_y_delta + b * self.b_y_delta;

        if x == self.prize_x && y == self.prize_y {
            return Ok(());
        }
        if x > self.prize_x || y > self.prize_y {
            return Err(true);
        }
        Err(false)
    }


    pub fn solve_old(&self) -> Option<(i64, i64, i64)> {
        let mut solutions = Vec::new();

        for a in 0..self.press_limit {
            for b in 0..self.press_limit {
                match self.compare(a, b) {
                    Ok(()) => solutions.push((a,b)),
                    Err(false) => { }
                    Err(true) => {
                        break;
                    }
                }
            }
        }

        if solutions.is_empty() {
            return None;
        }

        let mut min_x = 9999;
        let mut min_y = 9999;
        let mut min_tokens = 9999;

        for s in solutions {
            let tokens = 3 * s.0 + s.1;
            if tokens < min_tokens {
                min_tokens = tokens;
                min_x = s.0;
                min_y = s.1;
            }
        }

        Some((min_x,min_y,min_tokens))
    }

    pub fn solve(&self) -> Option<(i64, i64, i64)> {
        let det = (self.a_x_delta as f64) * (self.b_y_delta as f64) -
            (self.a_y_delta as f64) * (self.b_x_delta as f64);

        if det.abs() < f64::EPSILON {
            return None; // No unique solution (determinant is zero)
        }

        let inv_det = 1.0 / det;

        let a = (((self.prize_x as f64 * self.b_y_delta as f64 - self.prize_y as f64 * self.b_x_delta as f64) * inv_det)).round() as i64;
        let b = (((self.a_x_delta as f64 * self.prize_y as f64 - self.a_y_delta as f64 * self.prize_x as f64) * inv_det)).round() as i64;

        if(a >= self.press_limit || b >= self.press_limit) {
            return None;
        }
        if(a * self.a_x_delta + b * self.b_x_delta) != self.prize_x {
            return None;
        }
        if(a * self.a_y_delta + b * self.b_y_delta) != self.prize_y {
            return None;
        }
        Some((a, b, 3* a + b))
    }
}

#[cfg(test)]
mod tests {
    use aoc2024::common::point::Point;
    use super::*;
    #[test]
    fn test_solve() {
        let machine = Machine {
            a_x_delta: 94,
            a_y_delta: 34,
            b_x_delta: 22,
            b_y_delta: 67,
            prize_x: 8400,
            prize_y: 5400,
            press_limit: 100
        };
        assert_eq!(machine.solve().unwrap().0, 80);
        assert_eq!(machine.solve().unwrap().1, 40);
        assert_eq!(machine.solve().unwrap().2, 280);

        let machine = Machine {
            a_x_delta: 26,
            a_y_delta: 66,
            b_x_delta: 67,
            b_y_delta: 21,
            prize_x: 12748,
            prize_y: 12176,
            press_limit: 100
        };

        let solution = machine.solve();
        assert_eq!(machine.solve().is_none(), true);

        let machine = Machine {
            a_x_delta: 17,
            a_y_delta: 86,
            b_x_delta: 84,
            b_y_delta: 37,
            prize_x: 7870,
            prize_y: 6450,
            press_limit: 100
        };
        assert_eq!(machine.solve().unwrap().0, 38);
        assert_eq!(machine.solve().unwrap().1, 86);
        assert_eq!(machine.solve().unwrap().2, 200);


        let machine = Machine {
            a_x_delta: 69,
            a_y_delta: 23,
            b_x_delta: 27,
            b_y_delta: 71,
            prize_x: 18641,
            prize_y: 10279,
            press_limit: 100
        };

        assert_eq!(machine.solve().is_none(), true);

        let machine = Machine { a_x_delta: 69, b_x_delta: 15, a_y_delta: 15, b_y_delta: 63, prize_x: 8156, prize_y: 7136, press_limit: 100 };
        assert_eq!(machine.solve().is_none(), true);

        let machine = Machine { a_x_delta: 48, b_x_delta: 85, a_y_delta: 76, b_y_delta: 25, prize_x: 6812, prize_y: 5964, press_limit: 100 };
        let sln = machine.solve();
        assert_eq!(sln.is_some(), true);
        assert_eq!(sln.unwrap().0, 64);
        assert_eq!(sln.unwrap().1, 44);
        assert_eq!(sln.unwrap().2, 236);
    }
    #[test]
    fn test_from_lines() {
        let lines = vec![
            "Button A: X+94, Y+34".to_string(),
            "Button B: X+22, Y+67".to_string(),
            "Prize: X=8400, Y=5400".to_string(),
            "".to_string(),
            "Button A: X+26, Y+66".to_string(),
            "Button B: X+67, Y+21".to_string(),
            "Prize: X=12748, Y=12176".to_string()
        ];

        let machines = Machine::from_lines(&lines);
        assert_eq!(machines.len(), 2);
    }
    #[test]
    fn test_parse() {
        let lines = vec![
            "Button A: X+94, Y+34".to_string(),
            "Button B: X+22, Y+67".to_string(),
            "Prize: X=8400, Y=5400".to_string()
        ];

        let machine = Machine::parse(&lines);
        assert_eq!(machine.a_x_delta, 94);
        assert_eq!(machine.a_y_delta, 34);
        assert_eq!(machine.b_x_delta, 22);
        assert_eq!(machine.b_y_delta, 67);
        assert_eq!(machine.prize_x, 8400);
        assert_eq!(machine.prize_y, 5400);
    }

    #[test]
    fn test_get_deltas() {
        let line = String::from("Button A: X+94, Y+34");
        let deltas = get_deltas(&line);
        assert_eq!(deltas.0, 94);
        assert_eq!(deltas.1, 34);
    }

    #[test]
    fn test_get_prize_location() {
        let line = String::from("Prize: X=8400, Y=5400");
        let prize = get_prize_location(&line);
        assert_eq!(prize.0, 8400);
        assert_eq!(prize.1, 5400);
    }

    #[test]
    fn test_compare() {
        let machine = Machine {
            a_x_delta: 94,
            a_y_delta: 34,
            b_x_delta: 22,
            b_y_delta: 67,
            prize_x: 8400,
            prize_y: 5400,
            press_limit: 100
        };

        assert_eq!(machine.compare(79, 40), Err(false));
        assert_eq!(machine.compare(80, 39), Err(false));
        assert_eq!(machine.compare(80, 40), Ok(()));
        assert_eq!(machine.compare(81, 40), Err(true));
        assert_eq!(machine.compare(80, 41), Err(true));
    }
}
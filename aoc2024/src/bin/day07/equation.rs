pub struct Equation {
    pub result: i64,
    pub numbers: Vec<i64>,
}

fn concat(x: i64, y: i64) -> i64 {
    let mut as_str = x.to_string();
    as_str.push_str(&y.to_string());
    as_str.parse::<i64>().unwrap()
}

impl Equation {
    pub fn from_line(line: &String) -> Equation {
        let parts = line.split(": ").collect::<Vec<_>>();
        let result = parts[0].parse::<i64>().unwrap();
        let numbers = aoc2024::common::utils::utils::list_to_i64_by_char(parts[1], " ");

        Equation { result, numbers }
    }

    pub fn get_results(&self, allow_concat: bool) -> Vec<i64> {
        let mut results = Vec::new();

        results.push(self.numbers[0]);

        for i in 1..self.numbers.len() {
            let mut next = Vec::new();
            for r in &results {
                let next_val = self.numbers[i];
                next.push(r * next_val);
                next.push(r + next_val);
                if allow_concat {
                    next.push(concat(*r, next_val));
                }
            }
            results = next;
        }
        results
    }

    pub fn is_valid(&self, allow_concat: bool) -> bool {
        let values = self.get_results(allow_concat);

        for v in values {
            if v == self.result {
                return true;
            }
        }
        false
    }
}

#[cfg(test)]
mod tests {
    use crate::equation::{concat, Equation};

    #[test]
    fn parses_line(){
        let line = "123: 4 50 600".to_string();
        let equation = Equation::from_line(&line);
        assert_eq!(equation.result, 123);
        assert_eq!(equation.numbers.len(), 3);
        assert_eq!(equation.numbers[0], 4);
        assert_eq!(equation.numbers[1], 50);
        assert_eq!(equation.numbers[2], 600);
    }

    #[test]
    fn gets_results_without_concat(){
        let line = "123: 4 50 600".to_string();
        let equation = Equation::from_line(&line);
        let results = equation.get_results(false);
        assert_eq!(results.len(), 4);
        //4 + 50 + 600 = 654
        assert_eq!(results.contains(&654), true);
        //4 * 50 + 600 = 800
        assert_eq!(results.contains(&800), true);
        //4 + 50 * 600 = 32400
        assert_eq!(results.contains(&32400), true);
        //4 * 50 * 600 = 120000
        assert_eq!(results.contains(&120000), true);
    }

    #[test]
    fn gets_results_with_concat(){
        let line = "123: 4 50".to_string();
        let equation = Equation::from_line(&line);
        let results = equation.get_results(true);
        assert_eq!(results.len(), 3);
        //4 + 50 = 54
        assert_eq!(results.contains(&54), true);
        //4 * 50 = 200
        assert_eq!(results.contains(&200), true);
        //4 || 50 = 450
        assert_eq!(results.contains(&450), true);
    }

    #[test]
    fn concats() {
        assert_eq!(concat(1, 2), 12);
        assert_eq!(concat(10, 2), 102);
        assert_eq!(concat(1, 20), 120);
        assert_eq!(concat(100, 20), 10020);
    }
}
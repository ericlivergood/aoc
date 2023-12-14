
pub struct Record {
    pub conditions: String,
    pub condition_numbers: Vec<i32>,
    pub condition_numbers_str: String
}

pub fn generate_permutations(input: &str, current: &str, index: usize, result: &mut Vec<String>) {
    if index == input.len() {
        result.push(current.to_string());
        return;
    }

    let next_char = input.chars().nth(index).unwrap();
    if next_char == '?' {
        generate_permutations(input, &(current.to_owned() + "#"), index + 1, result);
        generate_permutations(input, &(current.to_owned() + "."), index + 1, result);
    } else {
        generate_permutations(input, &(current.to_owned() + &next_char.to_string()), index + 1, result);
    }
}

impl Record {
    pub fn new(s: String) -> Self {
        let parts = s.split(' ').collect::<Vec<&str>>();
        let conditions = parts[0].to_string();
        let mut condition_numbers = Vec::new();

        let condition_numbers_str = parts[1].trim().to_string();
        for n in parts[1].trim().split(',') {
            condition_numbers.push(n.trim().parse().unwrap());
        }

        Self {
            conditions,
            condition_numbers,
            condition_numbers_str
        }
    }

    pub fn unfolded(s: String) -> Self {
        let parts = s.split(' ').collect::<Vec<&str>>();
        let c = parts[0].to_string();
        let conditions = format!("{}?{}?{}?{}?{}",c,c,c,c,c);
        let mut condition_numbers = Vec::new();

        let n_str = parts[1].trim().to_string();
        let numbers = format!("{},{},{},{},{}", n_str, n_str, n_str, n_str, n_str);
        for n in numbers.trim().split(',').collect::<Vec<&str>>(){
            condition_numbers.push(n.trim().parse().unwrap());
        }

        Self {
            conditions,
            condition_numbers,
            condition_numbers_str: numbers
        }
    }

    pub fn matches(&self, s: String) -> bool {
        let mut n = 0;
        let parts = s.split('.').filter(|p| p.len() > 0).collect::<Vec<&str>>();
        if parts.len() != self.condition_numbers.len() {
            return false;
        }
        for p in parts {
            if p.len() as i32 != self.condition_numbers[n] {
                return false;
            }
            n+=1;
        }
        true
    }

    pub fn permutations(&self) -> Vec<String> {
        let mut permutations = Vec::new();
        generate_permutations(self.conditions.as_str(), "", 0, &mut permutations);
        permutations
    }

    pub fn count_solutions(&self) -> i64 {
        let mut numbers = self.condition_numbers.clone();
        let mut current = numbers.pop().unwrap();
        let mut current_run = 0;
        let mut options = Vec::new();

        for c in self.conditions.chars().rev() {
            match c {
                '.' => {
                    current_run = 0;
                    //current = numbers.pop().unwrap();
                    options.push(1);
                },
                '#' => {
                    current_run += 1;
                    options.push(1);
                },
                '?' => {
                    if current_run < current_run {
                        options.push(1);
                    }
                    else {
                        options.push(2);
                    }
                },
                _=> ()
            }
        }
        let mut solutions = 1;
        for o in options {
            solutions = solutions * o;
        }
        solutions
    }

    pub fn count_solutions_by_force(&self) -> i32 {
        let mut permutations = Vec::new();
        let mut solutions = 0;
        generate_permutations(self.conditions.as_str(), "", 0, &mut permutations);
        println!("checking {0} permutations", permutations.len());
        let mut i = 0;
        for p in permutations {
            if self.matches(p) {
                solutions += 1;
            }
            i += 1;
            if i % 10000 == 0 {
                println!("checked {i}");
            }
        }
        solutions
    }
}
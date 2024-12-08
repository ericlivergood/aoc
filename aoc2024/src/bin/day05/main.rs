use std::collections::HashMap;
use std::fmt;
use aoc2024::common::input_reader;
use aoc2024::common::utils::utils::list_to_ints_by_char;

struct RuleSet {
    rules: HashMap<i32, HashMap<i32, i32>>
}

impl RuleSet {
    pub fn new(lines: &Vec<&String>) -> Self{
        let mut rules = HashMap::new();

        for l in lines {
            let pair = list_to_ints_by_char(l, "|");
            let page = pair[1];
            let must_precede = pair[0];

            let cur = rules.get(&page);

            if !cur.is_some() {
                rules.insert(page, HashMap::new());
            }
            let page_num = page;
            rules.get_mut(&page_num).unwrap().insert(must_precede, must_precede);
        }

        Self {
            rules
        }
    }

    pub fn check_order(&self, order: &PageOrder) -> (bool, i32) {
        for i in 0..order.pages.len()-1 {
            let current = order.pages[i];
            let r = self.rules.get(&current);
            if !r.is_some() {
                continue;
            }
            let i_must_precede = r.unwrap();
            for x in i+1..order.pages.len() {
                let page = order.pages[x];
                if i_must_precede.contains_key(&page) {
                    return (false, i as i32);
                }
            }
        }
        (true, -1)
    }
}

struct PageOrder {
    pub id: i32,
    pub pages: Vec<i32>
}

impl PageOrder {
    pub fn value(&self) -> i32 {
        if self.pages.len() % 2 == 0 {
            panic!("Cant find middle of an even length list, id: {}", self.id);
        }

        self.pages[self.pages.len() / 2]
    }

    pub fn new(id: i32, line: &String) -> Self{
        let pages = list_to_ints_by_char(line, ",");
        Self {
            id,
            pages
        }
    }

    pub fn to_end(&mut self, position: usize) {
        let tmp = self.pages[position];
        self.pages.remove(position);
        self.pages.push(tmp);
    }
}

impl fmt::Display for PageOrder {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let pages: Vec<String> = self.pages.iter().map(|x| x.to_string()).collect();
        write!(f, "{}", pages.join(","))
    }
}

fn parse_input(lines: &Vec<String>) -> Result<(RuleSet, Vec<PageOrder>), String> {
    let mut rule_lines = Vec::new();
    let mut orders = Vec::new();
    let mut saw_empty_line = false;
    let mut order_id = 1;
    for line in lines {
        if line == "" {
            saw_empty_line = true;
            continue;
        }
        if saw_empty_line {
            orders.push(PageOrder::new(order_id, &line));
            order_id += 1;
        }
        else {
            rule_lines.push(line);
        }
    }
    let rule_set = RuleSet::new(&rule_lines);

    Ok((rule_set, orders))
}

fn part1(lines: &Vec<String>) {
    let (rule_set, page_orders) = parse_input(lines).unwrap();
    let mut sum = 0;
    for o in page_orders {
        if rule_set.check_order(&o).0 {
            sum += o.value();
        }
    }

    println!("Part 1: {}", sum);
}

fn ensure_page_order(rules: &RuleSet, order: &mut PageOrder) {
    let (mut is_valid, mut invalid_position) = rules.check_order(order);
    let mut tries = 0;
    while !is_valid {
        order.to_end(invalid_position as usize);
        tries += 1;
        if tries > 1000 { panic!("coudl not find valid order"); }
        (is_valid, invalid_position) = rules.check_order(order);
    }
}

fn part2(lines: &Vec<String>) {
    let (rule_set, mut page_orders) = parse_input(lines).unwrap();
    let mut sum = 0;

    for o in page_orders.iter_mut() {
        if !rule_set.check_order(&o).0 {
            ensure_page_order(&rule_set, o);
            sum += o.value();
        }
    }
    println!("Part 2: {}", sum);
}

fn main() {
    let lines = input_reader::input_reader::get_lines("./data/day05/input");
    part1(&lines);
    part2(&lines);
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test(){

    }
}
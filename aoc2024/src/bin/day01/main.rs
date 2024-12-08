use aoc2024::common::input_reader;
use std::collections::HashMap;

fn part1(lines: Vec<String>) {
    let mut list1 = Vec::new();
    let mut list2 = Vec::new();

    for l in lines  {
        let parts = l.split_whitespace().collect::<Vec<&str>>();
        let n1 = parts[0].parse::<i32>().unwrap();
        let n2 = parts[1].parse::<i32>().unwrap();
        list1.push(n1);
        list2.push(n2);
    }
    list1.sort();
    list2.sort();
    let n = list1.len();
    let mut sum = 0;
    for i in 0..n {
        let delta = (list2[i] - list1[i]).abs();
        sum += delta;
    }

    println!("{}", sum);
}

fn increment_hash_map(h: &mut HashMap<i32, i32>, n: i32) {
    let cur = h.get(&n);
    if cur.is_some() {
        h.insert(n, cur.unwrap()+1);
    }
    else {
        h.insert(n, 1);
    }
}

fn part2(lines: Vec<String>) {
    let mut list1 = HashMap::new();
    let mut list2 = HashMap::new();

    for l in lines {
        let parts = l.split_whitespace().collect::<Vec<&str>>();
        let n1 = parts[0].parse::<i32>().unwrap();
        let n2 = parts[1].parse::<i32>().unwrap();

        increment_hash_map(&mut list1, n1);
        increment_hash_map(&mut list2, n2);
    }
    let mut sum = 0;
    for k in list1.keys() {
        let n = list2.get(k);
        let m = list1.get(k).unwrap();
        if n.is_some() {
            let x = n.unwrap() * k * m;
            sum += x;
        }
    }
    println!("{}", sum);
}

fn main() {
    let lines = input_reader::input_reader::get_lines("./data/day01/input");
    //part1(lines);
    part2(lines);
}

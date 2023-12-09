#![allow(dead_code)]
mod days {
    pub mod day1;
    pub mod day2;
    pub mod day3;
    pub mod day4;
    pub mod day5;
    pub mod day6;
    pub mod day7;
}
mod common {
    pub mod input_reader;
}

fn main() {
    let day = days::day7::Day;
    day.run();
}

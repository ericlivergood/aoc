#![allow(dead_code)]
mod days {
    pub mod day1;
    pub mod day2;
    pub mod day3;
    pub mod day4;
}
mod common {
    pub mod input_reader;
}

fn main() {
    let day = days::day4::Day;
    day.run();
}

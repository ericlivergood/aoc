#![allow(dead_code)]
extern crate core;

mod days {
    pub mod day1;
    pub mod day2;
    pub mod day3;
    pub mod day4;
    pub mod day5;
    pub mod day6;
    pub mod day7;
    pub mod day8;
    pub mod day9;
}
mod common {
    pub mod input_reader;
}

fn main() {
    let day = days::day9::Day;
    day.run();
}

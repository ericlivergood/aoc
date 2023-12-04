#![allow(dead_code)]
mod days {
    pub mod day1;
    pub mod day2;
}
mod common {
    pub mod input_reader;
}

fn main() {
    let day = days::day2::Day2;
    day.run();
}

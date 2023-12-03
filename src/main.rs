mod days {
    pub mod day1;
}
mod common {
    pub mod input_reader;
}

fn main() {
    let day = days::day1::Day1;
    day.run();
}

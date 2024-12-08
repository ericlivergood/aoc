use std::fmt::{Display, Formatter};

#[derive(Clone, Copy)]
pub enum Direction {
    Left,
    Right
}

impl Display for Direction {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Direction::Left => write!(f, "Left"),
            Direction::Right => write!(f, "Right")
        }
    }
}

impl Direction {
    pub fn from(c: char) -> Direction {
        match c {
            'L' => Direction::Left,
            'R' => Direction::Right,
            _ => {
                println!("unknown direction {c}");
                panic!();
            }
        }
    }
}
use aoc2024::common::point::Point;

#[derive(Debug,Clone,Eq,PartialEq,Hash)]
pub enum Direction {
    North,
    South,
    East,
    West
}

pub enum MovementOutcome {
    Moved,
    Blocked,
    OnEdge
}

impl Direction {
    pub fn next(&self) -> Direction {
        match self {
            Direction::North => Direction::East,
            Direction::East => Direction::South,
            Direction::South => Direction::West,
            Direction::West => Direction::North
        }
    }
}

impl std::fmt::Display for Direction {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Direction::North => write!(f, "North"),
            Direction::East => write!(f, "East"),
            Direction::South => write!(f, "South"),
            Direction::West => write!(f, "West")
        }
    }
}
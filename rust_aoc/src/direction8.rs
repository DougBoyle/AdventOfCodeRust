use std::{io::{Error, ErrorKind}, ops::{Add, AddAssign}};
use crate::point::Point;

#[derive(Copy, Clone, Eq, PartialEq, PartialOrd, Ord, Debug, Hash)]
pub enum Direction8 {
    North,
    NorthEast,
    East,
    SouthEast,
    South,
    SouthWest,
    West,
    NorthWest
}

impl Direction8 {
    pub const fn all() -> [Direction8; 8] {
        [Direction8::North, Direction8::NorthEast, Direction8::East, Direction8::SouthEast, 
         Direction8::South, Direction8::SouthWest, Direction8::West, Direction8::NorthWest]
    }

    pub const fn diagonals() -> [Direction8; 4] {
        [Direction8::NorthEast, Direction8::SouthEast, Direction8::SouthWest, Direction8::NorthWest]
    }

    pub const fn opposite(&self) -> Direction8 {
        match &self {
            Direction8::North => Direction8::South,
            Direction8::NorthEast => Direction8::SouthWest,
            Direction8::East => Direction8::West,
            Direction8::SouthEast => Direction8::NorthWest,
            Direction8::South => Direction8::North,
            Direction8::SouthWest => Direction8::NorthEast,
            Direction8::West => Direction8::East,
            Direction8::NorthWest => Direction8::SouthEast,
        }
    }

    pub const fn clockwise_quarter_turn(&self) -> Direction8 {
        match &self {
            Direction8::North => Direction8::East,
            Direction8::NorthEast => Direction8::SouthEast,
            Direction8::East => Direction8::South,
            Direction8::SouthEast => Direction8::SouthWest,
            Direction8::South => Direction8::West,
            Direction8::SouthWest => Direction8::NorthWest,
            Direction8::West => Direction8::North,
            Direction8::NorthWest => Direction8::NorthEast,
        }
    }
}

impl Add<Direction8> for Point {
    type Output = Point;
    
    fn add(self, dir: Direction8) -> Self::Output {
        self + Point::from(dir)
    }
}

impl AddAssign<Direction8> for Point {
    fn add_assign(&mut self, dir: Direction8) {
        *self += Point::from(dir);
    }
}

impl TryFrom<Point> for Direction8 {
    type Error = Error;

    fn try_from(value: Point) -> Result<Self, Self::Error> {
        if let Some(d) = Direction8::all().iter().filter(|d| Point::from(**d) == value).next() {
            Ok(*d)
        } else {
            Err(Error::new(ErrorKind::InvalidInput, format!("{value} does not correspond to a direction")))
        }
    }
}

/// "North" is -ve in the y-axis, since row 0 of an input file is the top row.
impl From<Direction8> for Point {
    fn from(value: Direction8) -> Self {
        match value {
            Direction8::North => Point { x: 0, y: -1 },
            Direction8::NorthEast => Point { x: 1, y: -1 },
            Direction8::East => Point { x: 1, y: 0 },
            Direction8::SouthEast => Point { x: 1, y: 1 },
            Direction8::South => Point { x: 0, y: 1 },
            Direction8::SouthWest => Point { x: -1, y: 1 },
            Direction8::West => Point { x: -1, y: 0 },
            Direction8::NorthWest => Point { x: -1, y: -1 },
        }
    }
}

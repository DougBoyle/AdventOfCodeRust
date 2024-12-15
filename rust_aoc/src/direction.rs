use std::{io::{Error, ErrorKind}, ops::{Add, AddAssign}};
use crate::point::Point;

#[derive(Copy, Clone, Eq, PartialEq, PartialOrd, Ord, Debug, Hash)]
pub enum Direction {
    North,
    East,
    South,
    West
}

impl Direction {
    pub const fn all() -> [Direction; 4] {
        [Direction::North, Direction::South, Direction::East, Direction::West]
    }

    pub const fn opposite(&self) -> Direction {
        match &self {
            Direction::North => Direction::South,
            Direction::South => Direction::North,
            Direction::East => Direction::West,
            Direction::West => Direction::East,
        }
    }

    pub const fn clockwise_quarter_turn(&self) -> Direction {
        match &self {
            Direction::North => Direction::East,
            Direction::East => Direction::South,
            Direction::South => Direction::West,
            Direction::West => Direction::North,
        }
    }

    pub const fn perpendicular(&self) -> [Direction; 2] {
        let rotated = self.clockwise_quarter_turn();
        [rotated, rotated.opposite()]
    }
}

impl Add<Direction> for Point {
    type Output = Point;
    
    fn add(self, dir: Direction) -> Self::Output {
        self + Point::from(dir)
    }
}

impl AddAssign<Direction> for Point {
    fn add_assign(&mut self, dir: Direction) {
        *self += Point::from(dir);
    }
}

impl TryFrom<Point> for Direction {
    type Error = Error;

    fn try_from(value: Point) -> Result<Self, Self::Error> {
        if let Some(d) = Direction::all().iter().filter(|d| Point::from(**d) == value).next() {
            Ok(*d)
        } else {
            Err(Error::new(ErrorKind::InvalidInput, format!("{value} does not correspond to a direction")))
        }
    }
}

impl TryFrom<char> for Direction {
    type Error = Error;

    fn try_from(value: char) -> Result<Self, Self::Error> {
        match value {
            '^' => Ok(Direction::North),
            '>' => Ok(Direction::East),
            'v' => Ok(Direction::South),
            '<' => Ok(Direction::West),
            _ => Err(Error::new(ErrorKind::InvalidInput, format!("'{value}' does not correspond to a direction")))
        }
    }
}

/// "North" is -ve in the y-axis, since row 0 of an input file is the top row.
impl From<Direction> for Point {
    fn from(value: Direction) -> Self {
        match value {
            Direction::North => Point { x: 0, y: -1 },
            Direction::South => Point { x: 0, y: 1 },
            Direction::East => Point { x: 1, y: 0 },
            Direction::West => Point { x: -1, y: 0 },
        }
    }
}

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
    pub fn all() -> [Direction; 4] {
        [Direction::North, Direction::South, Direction::East, Direction::West]
    }

    pub fn opposite(&self) -> Direction {
        match &self {
            Direction::North => Direction::South,
            Direction::South => Direction::North,
            Direction::East => Direction::West,
            Direction::West => Direction::East,
        }
    }
}

impl Add<Direction> for Point {
    type Output = Point;
    
    fn add(self, dir: Direction) -> Self::Output {
        match dir {
            Direction::North => Point { x: self.x, y: self.y - 1 },
            Direction::South => Point { x: self.x, y: self.y + 1 },
            Direction::East => Point { x: self.x + 1, y: self.y },
            Direction::West => Point { x: self.x - 1, y: self.y },
        }
    }
}

impl AddAssign<Direction> for Point {
    fn add_assign(&mut self, dir: Direction) {
        match dir {
            Direction::North => self.y -= 1,
            Direction::South => self.y += 1,
            Direction::East => self.x += 1,
            Direction::West => self.x -= 1,
        }
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

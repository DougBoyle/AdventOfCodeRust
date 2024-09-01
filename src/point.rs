use crate::direction::Direction;

#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug)]
pub struct Point { pub x: i32, pub y: i32 }

impl std::ops::Add for Point {
    type Output = Point;
    
    fn add(self, rhs: Point) -> Point {
        Point { x: self.x + rhs.x, y: self.y + rhs.y }
    }
}

impl std::ops::AddAssign for Point {
    fn add_assign(&mut self, rhs: Point) {
        self.x += rhs.x;
        self.y += rhs.y;
    }
}

impl std::ops::Sub for Point {
    type Output = Point;
    
    fn sub(self, rhs: Point) -> Point {
        Point { x: self.x - rhs.x, y: self.y - rhs.y }
    }
}

impl std::ops::SubAssign for Point {
    fn sub_assign(&mut self, rhs: Point) {
        self.x -= rhs.x;
        self.y -= rhs.y;
    }
}

impl std::fmt::Display for Point {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({}, {})", self.x, self.y)
    }
}

struct PointIterator { start: Point, end: Point, incr: Point, inclusive_end: bool }

impl PointIterator {
    fn new(start: Point, end: Point, inclusive_end: bool) -> PointIterator {
        if start.x != end.x && start.y != end.y {
            panic!("Cannot iterate between {start} and {end}");
        }
        let incr = if start.x < end.x { Point { x:1, y:0 } }
        else if start.x > end.x { Point { x:-1, y:0 } }
        else if start.y < end.y { Point { x:0, y:1 } }
        else { Point {x:0, y:-1 } };
        PointIterator { start, end, incr, inclusive_end }
    }
}

impl Iterator for PointIterator {
    type Item = Point;

    fn next(&mut self) -> Option<Point> {
        if self.start == self.end {
            if self.inclusive_end {
                self.inclusive_end = false;
                Some(self.start)
            } else {
                None
            }
        } else {
            let p = self.start;
            self.start += self.incr;
            Some(p)
        }
    }
}

impl Point {
    pub fn to_inclusive(self, end: Point) -> impl Iterator<Item=Point> {
        PointIterator::new(self, end, true)
    }

    pub fn to_exclusive(self, end: Point) -> impl Iterator<Item=Point> {
        PointIterator::new(self, end, false)
    }

    pub fn orthogonal_neighbours(&self) -> Vec<Point> {
        Direction::all().iter().map(|d| *self + *d).collect()
    }
}
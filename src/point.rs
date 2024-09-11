use crate::direction::Direction;

#[derive(Copy, Clone, Eq, PartialEq, PartialOrd, Ord, Hash, Debug)]
pub struct Point { pub x: i64, pub y: i64 }

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

impl std::ops::Mul<i64> for Point {
    type Output = Point;
    fn mul(self, c: i64) -> Self::Output {
        let Point { x, y } = self;
        Point { x: x*c, y: y*c }
    }
}

struct PointIterator { start: Point, end: Point, incr: Point, inclusive_end: bool }

impl PointIterator {
    fn new(start: Point, end: Point, inclusive_end: bool) -> PointIterator {
        if !start.is_orthogonal_to(&end) {
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

    pub fn orthogonal_distance(&self, p: &Point) -> u64 {
        self.x.abs_diff(p.x) + self.y.abs_diff(p.y)
    }

    pub fn is_orthogonal_to(&self, p: &Point) -> bool {
        self.x == p.x || self.y == p.y
    }
}
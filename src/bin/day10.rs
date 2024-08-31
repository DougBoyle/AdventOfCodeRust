use std::collections::{BTreeMap, BTreeSet, HashMap};
use std::io::{Error, ErrorKind};
use std::ops::Add;
use rust_aoc::point::Point;

type Grid = HashMap<Point, Tile>;

fn main() {
    let mut grid = HashMap::new();
    let mut start = Point {x: -1, y: -1};
    
    rust_aoc::read_input(10).enumerate().for_each(|(y, line)| 
        parse_row(&line, y.try_into().unwrap(), &mut grid, &mut start)
    );

    println!("Grid: {}", grid.len());
    println!("Start: {start}");

    let start_pipes: Vec<_> = point_neighbours(&start).iter()
        .map(|p| grid.get(p)).filter(Option::is_some).map(Option::unwrap)
        .filter(|tile| tile.connects_to(&start)).collect();

    assert!(start_pipes.len() == 2);

    let mut previous_point = start;
    let mut current = *start_pipes[0];
    let end = *start_pipes[1];

    // Populate start properly, kind not made clear from parsing grid
    let start_kind = infer_kind(start, current.point, end.point);
    let start = Tile { kind: start_kind, point: start };
    grid.insert(start.point, start);

    let mut pipes = vec![start];
    while current.point != end.point {
        pipes.push(current);
        let next_point = current.get_connected_points().into_iter().filter(|p| *p != previous_point).next().unwrap();
        previous_point = current.point;
        current = *grid.get(&next_point).unwrap();
    }
    pipes.push(end);

    println!("Loop size: {}", pipes.len());
    println!("Distance to midpoint: {}", pipes.len() / 2); // 6717

    // Strategy: Get bounds of loop, then just scan rows to work out how many tiles are 'inside' the loop
    let mut pipes_by_row: BTreeMap<i32, Vec<Point>> = BTreeMap::new();
    for Tile { point: p @ Point { y, ..}, ..} in pipes {
        pipes_by_row.entry(y).or_default().push(p);
    }
    // Middle row: only bit on right is 'inside', need to track if line present below/above (B/A)
    //                 |----\
    //    /-----\   /--/    |
    //    |     \---/       |
    //() (B)   ()  (B)(BA)  ()
    let contained_tiles: i32 = pipes_by_row.into_values().map(|row| {
        let mut row: Vec<_> = row.iter().collect();
        row.sort_by_key(|p| p.x); // could we have stored sorted to begin with?
        let mut line_below = false;
        let mut line_above = false;
        row.windows(2).map(|start_end| {
            let start = start_end[0];
            let end = start_end[1];
            let start_dirs = grid.get(start).unwrap().kind.directions();
            for dir in start_dirs {
                match dir {
                    Direction::North => { line_above = !line_above; }
                    Direction::South => { line_below = !line_below; }
                    _ => {}
                }
            }
            
            if line_above && line_below {
                end.x - start.x - 1 // end = start + 1 => no gap between them
            } else {
                0
            }
        }).sum::<i32>()
    }).sum();

    println!("Contains {contained_tiles} tiles"); // 381

}

fn parse_row(row: &str, y: i32, grid: &mut Grid, start: &mut Point) {
    let y = y.try_into().unwrap();
    row.chars().enumerate().for_each(|(x, c)| 
        parse(x.try_into().unwrap(), y, c, grid, start)
    )
}

fn parse(x: i32, y: i32, c: char, grid: &mut Grid, start: &mut Point) {
    let point = Point { x, y };
    if c == '.' {
            return
    } else if c == 'S' { 
        *start = point;
    } else {
        let kind = TileKind::try_from(c).unwrap();
        grid.insert(point, Tile { kind, point });
    }
}

fn infer_kind(p: Point, p1: Point, p2: Point) -> TileKind {
    let dir1 = Direction::try_from(p1 - p).unwrap();
    let dir2 = Direction::try_from(p2 - p).unwrap();
    TileKind::try_from((dir1, dir2)).unwrap()
}

fn point_neighbours(p: &Point) -> Vec<Point> {
    Direction::all().iter().map(|d| *p + *d).collect()
}

#[derive(Copy, Clone)]
struct Tile {
    kind: TileKind,
    point: Point,
}

impl Tile {
    fn connects_to(&self, other: &Point) -> bool {
        self.get_connected_points().contains(other)
    }

    fn get_connected_points(&self) -> Vec<Point> {
        self.kind.directions().iter().map(|d| self.point + *d).collect()
    }   
}

#[derive(Copy, Clone)]
enum TileKind {
    NorthSouth,
    EastWest,
    NorthEast,
    NorthWest,
    SouthWest,
    SouthEast
}

impl TileKind {
    fn all() -> [TileKind; 6] {
        [TileKind::NorthSouth, TileKind::EastWest, TileKind::NorthEast, TileKind::NorthWest, TileKind::SouthWest, TileKind::SouthEast]
    }

    fn directions(&self) -> [Direction; 2] {
        match self {
            TileKind::NorthSouth => [Direction::North, Direction::South],
            TileKind::EastWest => [Direction::East, Direction::West],
            TileKind::NorthEast => [Direction::North, Direction::East],
            TileKind::NorthWest => [Direction::North, Direction::West],
            TileKind::SouthEast => [Direction::South, Direction::East],
            TileKind::SouthWest => [Direction::South, Direction::West],
        }
    }
}

impl TryFrom<char> for TileKind {
    type Error = Error;
    fn try_from(value: char) -> Result<Self, Self::Error> {
        match value {
            '|' => Ok(TileKind::NorthSouth),
            '-' => Ok(TileKind::EastWest),
            'L' => Ok(TileKind::NorthEast),
            'J' => Ok(TileKind::NorthWest),
            'F' => Ok(TileKind::SouthEast),
            '7' => Ok(TileKind::SouthWest),
            _ => Err(Error::new(ErrorKind::InvalidInput, format!("Unknown tile: {value}")))
        }
    }
}

impl TryFrom<(Direction, Direction)> for TileKind {
    type Error = Error;

    fn try_from((d1, d2): (Direction, Direction)) -> Result<Self, Self::Error> {
        let mut dirs = vec![d1, d2];
        dirs.sort();

        if let Some(kind) = TileKind::all().iter().filter(|kind| {
            let mut kind_dirs: Vec<_> = kind.directions().into_iter().collect();
            kind_dirs.sort();
            dirs == kind_dirs
        }).next() {
            Ok(*kind)
        } else {
            Err(Error::new(ErrorKind::InvalidInput, format!("({d1:?}, {d2:?}) does not correspond to a tile")))
        }
    }
}

// TODO: Move to library?
#[derive(Copy, Clone, Eq, PartialEq, PartialOrd, Ord, Debug)]
enum Direction {
    North,
    East,
    South,
    West
}

impl Direction {
    fn all() -> [Direction; 4] {
        [Direction::North, Direction::South, Direction::East, Direction::West]
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

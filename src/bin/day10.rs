use std::collections::{BTreeMap, HashMap};
use std::io::{Error, ErrorKind};
use std::ops::Deref;
use rust_aoc::point::Point;
use rust_aoc::direction::Direction;

type Grid = HashMap<Point, Tile>;

fn main() {
    let mut grid = HashMap::new();
    let mut start = Point {x: -1, y: -1};
    
    rust_aoc::process_grid(10, |p, c| { parse(p, c, &mut grid, &mut start) });

    println!("Grid: {}", grid.len());
    println!("Start: {start}");

    let start_pipes = Tile::tiles_connecting_to_point(start, &grid);
    assert!(start_pipes.len() == 2);
    
    let start = Tile::infer_from_points(start, start_pipes[0].point, start_pipes[1].point);
    grid.insert(start.point, start); // fill in 'S' tile of grid

    let loop_pipes = Tile::traverse_loop(start, &grid);

    println!("Loop size: {}", loop_pipes.len());
    println!("Distance to midpoint: {}", loop_pipes.len() / 2); // 6717

    let contained_tiles = count_enclosed_cells(loop_pipes);

    println!("Contains {contained_tiles} tiles"); // 381

}

fn parse(point: Point, c: char, grid: &mut Grid, start: &mut Point) {
    if c == '.' {
            return
    } else if c == 'S' { 
        *start = point;
    } else {
        let kind = TileKind::try_from(c).unwrap();
        grid.insert(point, Tile { kind, point });
    }
}

fn count_enclosed_cells(loop_tiles: Vec<Tile>) -> i32 {
    // Strategy: Scan rows to work out how many tiles are 'inside' the loop
    let mut pipes_by_row: BTreeMap<i32, BTreeMap<i32, Tile>> = BTreeMap::new();
    for tile in loop_tiles {
        pipes_by_row.entry(tile.y).or_default().insert(tile.x, tile);
    }
    // Middle row: only bit on right is 'inside', need to track if line present below/above (B/A)
    //                 |----\
    //    /-----\   /--/    |
    //    |     \---/       |
    //() (B)   ()  (B)(BA)  ()
    pipes_by_row.into_values().map(|row| {
        let mut line_below = false;
        let mut line_above = false;
        row.into_values().collect::<Vec<_>>().windows(2)
            .map(|w| TryInto::<[Tile; 2]>::try_into(w).unwrap())
            .map(|[start, end]| {
            for dir in start.kind.directions() {
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
    }).sum()
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

    fn tiles_connecting_to_point(p: Point, grid: &Grid) -> Vec<Tile> {
        p.orthogonal_neighbours().iter()
            .map(|p| grid.get(p)).filter(Option::is_some).map(Option::unwrap)
            .filter(|tile| tile.connects_to(&p))
            .map(|tile| *tile)
            .collect()
    }

    fn infer_from_points(point: Point, p1: Point, p2: Point) -> Tile {
        let kind = TileKind::infer_from_points(point, p1, p2);
        Tile { kind, point }
    }

    fn traverse_loop(start: Tile, grid: &Grid) -> Vec<Tile> {
        let mut loop_tiles = vec![start];
        let mut previous = start;

        // pick an arbitrary direction to traverse in
        let neighbours = grid.get(&previous).unwrap().get_connected_points();
        assert!(neighbours.len() == 2);
        let mut current = *grid.get(&neighbours[0]).unwrap();

        while current.point != start.point {
            loop_tiles.push(current);
            // Should be exactly 1 choice
            let next_points = current.get_connected_points().into_iter().filter(|p| *p != previous.point);
            let next_point = rust_aoc::assert_single(next_points);

            previous = current;
            current = *grid.get(&next_point).unwrap();
        }

        loop_tiles
    }
}

impl Deref for Tile {
    type Target = Point;

    fn deref(&self) -> &Self::Target {
        &self.point
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

    fn infer_from_points(p: Point, p1: Point, p2: Point) -> TileKind {
        let dir1 = Direction::try_from(p1 - p).unwrap();
        let dir2 = Direction::try_from(p2 - p).unwrap();
        TileKind::try_from((dir1, dir2)).unwrap()
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

use std::{collections::{HashSet, VecDeque}, io::{Error, ErrorKind}, ops::{Index, IndexMut}};

use rust_aoc::{direction::Direction, point::Point};

fn main() {
    let layout: Vec<Vec<TileKind>> = rust_aoc::read_input(16)
        .map(|line| line.chars().map(TileKind::try_from).map(Result::unwrap).collect())
        .collect();

    let start = Point {x: 0, y: 0};
    let start_dir = Direction::East;

    let total = count_energized(start, start_dir, &layout);

    println!("Part 1: Energized cells: {total}"); // 7434

    let width = layout[0].len() as i32;
    let height = layout.len() as i32;

    let maximum: usize = (0..width).map(|x| (Point {x, y: 0}, Direction::South))
        .chain((0..width).map(|x| (Point {x, y: height - 1}, Direction::North)))
        .chain((0..height).map(|y| (Point {x: 0, y}, Direction::East)))
        .chain((0..height).map(|y| (Point {x: width - 1, y}, Direction::West)))
        .map(|(start, start_dir)| count_energized(start, start_dir, &layout))
        .max().unwrap();

    println!("Part 2: Maximum: {maximum}"); // 8183
}

fn count_energized(start: Point, start_dir: Direction, layout: &Vec<Vec<TileKind>>) -> usize {
    let mut grid: Grid = layout.iter()
        .map(|row| row.iter().map(|kind| Cell::new(*kind)).collect())
        .collect();

    let mut to_process = VecDeque::new();
    grid[&start].energized_directions.insert(start_dir);
    to_process.push_back((start, start_dir));

    // TODO: Split out as a function?
    while let Some((p, dir)) = to_process.pop_front() {
        for next_dir in grid[&p].kind.get_next_dirs(dir) {
            let next_point = p + next_dir;
            if grid.is_in_bounds(&next_point) {
                let existing_directions = &mut grid[&next_point].energized_directions;
                if !existing_directions.contains(&next_dir) {
                    existing_directions.insert(next_dir);
                    to_process.push_back((next_point, next_dir));
                }
            }
        }
    }

    grid.iter().filter(|cell| !cell.energized_directions.is_empty()).count()
}

// TODO: Create a class for this
struct Grid {
    cells: Vec<Vec<Cell>>,
    width: i32,
    height: i32,
}

impl Grid {
    fn new(cells: Vec<Vec<Cell>>) -> Grid {
        let width = cells[0].len();
        let height = cells.len();
        Grid { cells, width: width as i32, height: height as i32 }
    }

    fn is_in_bounds(&self, &Point { x, y }: &Point) -> bool {
        x >= 0 && y >= 0 && x < self.width && y < self.height
    }

    fn iter(&self) -> impl Iterator<Item=&Cell> {
        self.cells.iter().flat_map(|row| row.iter())
    }
}

impl Index<&Point> for Grid {
    type Output = Cell;

    fn index(&self, point: &Point) -> &Self::Output {
        &self.cells[point.y as usize][point.x as usize]
    }
}

impl IndexMut<&Point> for Grid {
    fn index_mut(&mut self, point: &Point) -> &mut Self::Output {
        &mut self.cells[point.y as usize][point.x as usize]
    }
}

impl FromIterator<Vec<Cell>> for Grid {
    fn from_iter<T: IntoIterator<Item = Vec<Cell>>>(iter: T) -> Self {
        Grid::new(iter.into_iter().collect())
    }
}

struct Cell {
    kind: TileKind,
    energized_directions: HashSet<Direction>,
}

impl Cell {
    fn new(kind: TileKind) -> Self {
        Cell { kind, energized_directions: HashSet::new() }
    }
}

#[derive(Copy, Clone)]
enum TileKind {
    Empty,
    UpRightMirror,
    UpLeftMirror,
    VerticalSplitter,
    HorizontalSplitter
}

impl TileKind {
    fn get_next_dirs(&self, dir: Direction) -> Vec<Direction> {
        match self {
            TileKind::Empty => vec![dir],
            TileKind::UpRightMirror => vec![TileKind::reflect_up_right(dir)],
            TileKind::UpLeftMirror => vec![TileKind::reflect_up_left(dir)],
            TileKind::VerticalSplitter => TileKind::vertical_split(dir),
            TileKind::HorizontalSplitter => TileKind::horizontal_split(dir),
        }
    }

    fn reflect_up_right(dir: Direction) -> Direction {
        match dir {
            Direction::North => Direction::East,
            Direction::East => Direction::North,
            Direction::South => Direction::West,
            Direction::West => Direction::South,
        }
    }

    fn reflect_up_left(dir: Direction) -> Direction {
        match dir {
            Direction::North => Direction::West,
            Direction::East => Direction::South,
            Direction::South => Direction::East,
            Direction::West => Direction::North,
        }
    }

    fn vertical_split(dir: Direction) -> Vec<Direction> {
        match dir {
            Direction::North | Direction::South => vec![dir],
            Direction::East | Direction::West => vec![Direction::North, Direction::South],
        }
    }

    fn horizontal_split(dir: Direction) -> Vec<Direction> {
        match dir {
            Direction::North | Direction::South => vec![Direction::East, Direction::West],
            Direction::East | Direction::West => vec![dir],
        }
    }
}

impl TryFrom<char> for TileKind {
    type Error = Error;
    fn try_from(value: char) -> Result<Self, Self::Error> {
        match value {
            '.' => Ok(TileKind::Empty),
            '/' => Ok(TileKind::UpRightMirror),
            '\\' => Ok(TileKind::UpLeftMirror),
            '|' => Ok(TileKind::VerticalSplitter),
            '-' => Ok(TileKind::HorizontalSplitter),
            _ => Err(Error::new(ErrorKind::InvalidInput, format!("Unknown tile: {value}")))
        }
    }
}
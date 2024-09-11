use std::{collections::HashSet, io::{Error, ErrorKind}};

use rust_aoc::{direction::Direction, point::Point, BreadthFirstSearch};

fn main() {
    let layout: Vec<Vec<TileKind>> = rust_aoc::read_input(16)
        .map(|line| line.chars().map(TileKind::try_from).map(Result::unwrap).collect())
        .collect();

    let start = Point {x: 0, y: 0};
    let start_dir = Direction::East;

    let total = count_energized(start, start_dir, &layout);

    println!("Part 1: Energized cells: {total}"); // 7434

    let width = layout[0].len() as i64;
    let height = layout.len() as i64;

    let maximum: usize = (0..width).map(|x| (Point {x, y: 0}, Direction::South))
        .chain((0..width).map(|x| (Point {x, y: height - 1}, Direction::North)))
        .chain((0..height).map(|y| (Point {x: 0, y}, Direction::East)))
        .chain((0..height).map(|y| (Point {x: width - 1, y}, Direction::West)))
        .map(|(start, start_dir)| count_energized(start, start_dir, &layout))
        .max().unwrap();

    println!("Part 2: Maximum: {maximum}"); // 8183
}

struct EnergizedSearch<'a> {
    grid: &'a mut Grid,
}

impl rust_aoc::BreadthFirstSearch for EnergizedSearch<'_> {
    type Node = (Point, Direction);

    fn mark(&mut self, (p, dir): &(Point, Direction)) -> bool {
        self.grid[p].energized_directions.insert(*dir)
    }

    fn neighbours(&self, (p, dir): &(Point, Direction)) -> Vec<(Point, Direction)> {
        self.grid[p].kind.get_next_dirs(*dir).into_iter()
            .map(|next_dir| (*p + next_dir, next_dir))
            .filter(|(p, _)| self.grid.is_in_bounds(p))
            .collect()
    }
}

fn count_energized(start: Point, start_dir: Direction, layout: &Vec<Vec<TileKind>>) -> usize {
    let mut grid: Grid = layout.iter()
        .map(|row| row.iter().map(|kind| Cell::new(*kind)).collect())
        .collect();
    let search = EnergizedSearch { grid: &mut grid };
    search.search((start, start_dir));
    grid.iter().filter(|cell| !cell.energized_directions.is_empty()).count()
}

type Grid = rust_aoc::grid::Grid<Cell>;

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
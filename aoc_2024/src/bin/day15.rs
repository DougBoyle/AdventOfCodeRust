use std::{collections::HashSet, io::{Error, ErrorKind}};

use aoc_2024::read_input;
use rust_aoc::{direction::Direction, grid::Grid, point::Point};

fn main() {
    part1();
    part2();
}

fn part1() {
    let (mut grid, steps) = parse_input();
    
    perform_steps(&mut grid, steps);

    let total = score_boxes(&grid);

    println!("Total: {total}"); // 1465523
}

fn part2() {
    let (grid, steps) = parse_input();

    let mut grid = Grid::new(grid.iter_rows().map(|row| row.flat_map(|tile| match tile {
        Tile::Empty => [Tile::Empty, Tile::Empty],
        Tile::Wall => [Tile::Wall, Tile::Wall],
        Tile::Box => [Tile::BoxLeft, Tile::BoxRight],
        Tile::Robot => [Tile::Robot, Tile::Empty],
        _ => panic!("Not expecting {tile:?} in the original input"),
    }).collect()).collect());

    perform_steps(&mut grid, steps);

    let total = score_boxes(&grid);

    println!("Total: {total}"); // 1471049
}

fn perform_steps(grid: &mut Grid<Tile>, steps: Vec<Direction>) {
    let mut robot = grid.enumerate().filter(|(_, &tile)| tile == Tile::Robot).map(|(p, _)| p).next().unwrap();

    for dir in steps {
        if step(robot, dir, grid) {
            robot = robot + dir;
        }
    }
}

fn step(robot: Point, dir: Direction, grid: &mut Grid<Tile>) -> bool {
    // For each tile up/down track the set of tiles that need to move
    let mut initial_row = HashSet::new();
    initial_row.insert(robot);
    let mut to_move = vec![initial_row];

    loop {
        let last_row = to_move.last().unwrap();
        let mut next_row = HashSet::new();
        for p in last_row.iter().map(|&p| p + dir) {
            match grid[&p] {
                Tile::Box => { next_row.insert(p); },
                Tile::BoxLeft => {
                    next_row.insert(p);
                    if dir == Direction::North || dir == Direction::South { next_row.insert(p + Direction::East); }
                },
                Tile::BoxRight => {
                    next_row.insert(p);
                    if dir == Direction::North || dir == Direction::South { next_row.insert(p + Direction::West); }
                },
                Tile::Wall => return false,
                _ => {}
            }
        }
        if next_row.is_empty() { break }
        to_move.push(next_row);
    }

    for row in to_move.into_iter().rev() {
        for p in row {
            grid[&(p + dir)] = grid[&p];
            grid[&p] = Tile::Empty;
        }
    }
    true
}

fn score_boxes(grid: &Grid<Tile>) -> i64 {
    grid.enumerate().filter(|(_, &tile)| tile == Tile::Box || tile == Tile::BoxLeft)
        .map(|(p, _)| p.x + 100*p.y)
        .sum()
}

#[derive(Copy, Clone, Eq, PartialEq, Debug)]
enum Tile {
    Empty,
    Wall,
    Robot,
    // Part 1
    Box,
    // Part 2
    BoxLeft,
    BoxRight,
}

impl TryFrom<char> for Tile {
    type Error = Error;

    fn try_from(value: char) -> Result<Self, Self::Error> {
        match value {
            '.' => Ok(Tile::Empty),
            '#' => Ok(Tile::Wall),
            'O' => Ok(Tile::Box),
            '@' => Ok(Tile::Robot),
            _ => Err(Error::new(ErrorKind::InvalidInput, format!("Unrecognised character {value}")))
        }
    }
}

fn parse_input() -> (Grid<Tile>, Vec<Direction>) {
    let mut it = read_input(15);

    let grid = Grid::parse(it.by_ref().take_while(|s| !s.trim().is_empty()), |c| Tile::try_from(c).unwrap());
    let steps = it.flat_map(|s| s.chars().map(|c| Direction::try_from(c).unwrap()).collect::<Vec<_>>().into_iter()).collect();
    (grid, steps)
}

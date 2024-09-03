use std::{cmp::{max, Ordering}, collections::HashMap, hash::{DefaultHasher, Hash, Hasher}};

use rust_aoc::{direction::Direction, point::Point};

fn main() {
    println!("Part 1");
    Part1::process(); // 108857
    println!("Part 2");
    Part2::process(); // 95273
}

trait Part {
    fn tilt(grid: &mut Grid);

    fn process() {
        let mut grid = Self::parse_grid();

        println!("Grid size: {} x {}, contains {} rocks", grid.width, grid.height, grid.cells.len());

        Self::tilt(&mut grid);

        let load = Self::calculate_load(&grid);

        println!("Load {load}"); 
    }

    fn parse_grid() -> Grid {
        let mut width = 0;
        let mut height = 0;
        let mut cells = Cells::new();
        rust_aoc::process_grid(14, |p, c| {
            width = max(width, p.x+1);
            height = max(height, p.y+1);
            parse(p, c, &mut cells);
        });

        Grid { cells, width, height }
    }

    fn calculate_load(grid: &Grid) -> i32 {
        grid.cells.iter()
            .filter(|(_, r)| **r == Rock::Round)
            .map(|(p, _)| grid.height - p.y)
            .sum()
    }
}


struct Part1;

impl Part for Part1 {
    fn tilt(grid: &mut Grid) {
        tilt(Direction::North, grid);
    }
}

struct Part2;

impl Part for Part2 {
    /// Strategy: Grid is 100 x 100 with ~3k rocks (with ~50% round rocks).
    /// Rather than storing the full state, just track the hash after each cycle.
    /// Once we see the hash repeat, we know the expected loop period.
    /// Take a copy of the grid state. Run through another loop, check the state is the same,
    /// which verifies that this is a genuine loop rather than a key collision.
    /// Then take the remaining cycles modulo the loop period to quickly reach the final state. 
    fn tilt(grid: &mut Grid) {
        let mut hash_cycles = HashMap::new();

        let mut cycles_done = 0;
        loop {
            cycle(grid);
            cycles_done += 1;
            let hash = grid.hash();
            if hash_cycles.contains_key(&hash) { break }
            else { hash_cycles.insert(hash, cycles_done); }
        }

        let hash = grid.hash();
        let loop_offset = *hash_cycles.get(&hash).unwrap();
        let loop_period = cycles_done - loop_offset;

        println!("Hash cycled on iteration {cycles_done}: offset = {loop_offset}, period = {loop_period}");

        let expected_state = grid.get_sorted_rock_positions();

        for _ in 0..loop_period {
            cycle(grid);
        }
        cycles_done += loop_period;

        let final_state = grid.get_sorted_rock_positions();

        let stable = expected_state.iter().zip(final_state.iter()).all(|(p1, p2)| p1 == p2);
        println!("Final state stable: {stable}");

        if !stable { panic!("Expected equal hashes, but after loop (cycle {} to {}), grid state was different", loop_offset + loop_period, cycles_done) }

        let cycles_remaining = (1000000000 - cycles_done) % loop_period;
        for _ in 0..cycles_remaining {
            cycle(grid);
        }
    }
}

fn cycle(grid: &mut Grid) {
    for dir in [Direction::North, Direction::West, Direction::South, Direction::East] {
        tilt(dir, grid);
    }
}

fn tilt(dir: Direction, grid: &mut Grid) {
    let mut rollable = get_rollable_rocks(grid, dir);
    while !rollable.is_empty() {
        for p in rollable {
            let mut new_point = p + dir;
            while can_roll(&new_point, dir, grid) { new_point += dir }
            grid.cells.remove(&p);
            grid.cells.insert(new_point, Rock::Round);
        }
        rollable = get_rollable_rocks(grid, dir);
    }
}

fn get_rollable_rocks(grid: &Grid, dir: Direction) -> Vec<Point> {
    grid.cells.iter()
        .filter(|(_, &rock)| rock == Rock::Round)
        .map(|(&p, _)| p)
        .filter(|p| can_roll(p, dir, grid)).collect()
}

fn can_roll(point: &Point, dir: Direction, grid: &Grid) -> bool {
    let next = *point + dir;
    is_in_bounds(&next, grid) && !grid.cells.contains_key(&next)   
}

fn is_in_bounds(point: &Point, grid: &Grid) -> bool {
    point.x >= 0 && point.y >= 0 && point.x < grid.width && point.y < grid.height
}

fn parse(p: Point, c: char, cells: &mut Cells) {
    match c {
        'O' => { cells.insert(p, Rock::Round); },
        '#' => { cells.insert(p, Rock::Fixed); },
        _ => {}
    }
}

/// Grid is around 100 x 100, and quite densely populated,
/// so a simple array might actually be faster, but --release build still takes <1s.
type Cells = HashMap<Point, Rock>;

struct Grid {
    cells: Cells,
    width: i32,
    height: i32,
}

impl Grid {
    fn get_sorted_rock_positions(&self) -> Vec<Point> {
        let mut rocks: Vec<Point> = self.cells.keys().map(|p| *p).collect();
        rocks.sort_by(|p1, p2| {
            match p1.x.cmp(&p2.x) {
                Ordering::Equal => p1.y.cmp(&p2.y),
                ord => ord
            }
        });
        rocks
    }

    fn hash(&self) -> u64 {
        let mut s = DefaultHasher::new();
        for point in self.cells.keys() {
            point.hash(&mut s);
        }
        s.finish()
    }
}

#[derive(Clone, Copy, Eq, PartialEq)]
enum Rock {
    Round,
    Fixed,
}

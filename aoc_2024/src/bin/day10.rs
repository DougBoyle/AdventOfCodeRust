use std::collections::{HashMap, HashSet};

use aoc_2024::read_input;
use rust_aoc::{direction::Direction, grid::Grid, point::Point};


fn main() {
    part1();
    part2();
}

fn part1() {
    let grid = read_grid();

    let total: usize = grid.enumerate()
        .filter(|(_, x)| **x == 0)
        .map(|(p, _)| score_start(p, &grid))
        .sum();

    println!("Total: {total}"); // 782
}

fn part2() {
    let grid = read_grid();

    let total: usize = grid.enumerate()
        .filter(|(_, x)| **x == 0)
        .map(|(p, _)| rate_start(p, &grid))
        .sum();

    println!("Total: {total}"); // 1694
}

fn score_start(start: Point, grid: &Grid<u32>) -> usize {
    let mut current_level = HashSet::new();
    current_level.insert(start);
    for next_val in 1..=9 {
        current_level = current_level.iter()
            .flat_map(|p| Direction::all().map(|d| *p + d))
            .filter(|p| grid.is_in_bounds(p) && grid[p] == next_val)
            .collect();
    }
    current_level.len()
}

fn rate_start(start: Point, grid: &Grid<u32>) -> usize {
    let mut current_level = HashMap::new();
    current_level.insert(start, 1);
    for next_val in 1..=9 {
        let mut next_level: HashMap<Point, usize> = HashMap::new();
        for (new_point, count) in current_level.iter()
            .flat_map(|(p, count)| Direction::all().map(|d| (*p + d, count)))
            .filter(|(p, _)| grid.is_in_bounds(p) && grid[p] == next_val) {
                next_level.entry(new_point).and_modify(|c| *c += *count).or_insert(*count);
            }
        current_level = next_level;
    }
    current_level.values().sum()
}

fn read_grid() -> Grid<u32> {
    Grid::parse(read_input(10), |c| c.to_digit(10).unwrap())
}
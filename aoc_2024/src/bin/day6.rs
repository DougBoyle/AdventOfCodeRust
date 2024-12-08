use std::collections::{HashMap, HashSet};

use aoc_2024::read_input;
use rust_aoc::{direction::Direction, grid::Grid, point::Point};


fn main() {
    part1();
    part2();
}

fn part1() {
    let Input { grid, start: mut pos } = parse_input();
    let mut visited = HashSet::new();
    loop {
        visited.insert(pos.0);
        let next = pos.0 + pos.1;
        if !grid.is_in_bounds(&next) {
            break;
        } else if grid[&next] {
            pos = (pos.0, pos.1.clockwise_quarter_turn());
        } else {
            pos = (next, pos.1);
        }
    }

    println!("Total: {}", visited.len()); // 5312
}

fn part2() {
    let Input { mut grid, start: mut pos } = parse_input();
    let mut blockers_tried = HashSet::new();
    let mut blockers_found = 0;
    let mut visited = HashMap::new();
    loop {
        let next = pos.0 + pos.1;
        let new_pos = if !grid.is_in_bounds(&next) {
            break;
        } else if grid[&next] {
            (pos.0, pos.1.clockwise_quarter_turn())
        } else {
            // consider putting an obstact in front of our current position
            if blockers_tried.insert(next) {
                grid[&next] = true;
                if will_loop(pos, &visited, &grid) { blockers_found += 1 };
                grid[&next] = false;
            }
            (next, pos.1)
        };
        visited.entry(pos.0).or_insert(HashSet::new()).insert(pos.1);
        pos = new_pos;
    }

    println!("Total: {blockers_found}"); // 1748
}

fn will_loop(mut pos: (Point, Direction), previously_visited: &HashMap<Point, HashSet<Direction>>, grid: &Grid<bool>) -> bool {
    let mut visited = previously_visited.clone();
    loop {
        if !visited.entry(pos.0).or_insert(HashSet::new()).insert(pos.1) { return true };
        let next = pos.0 + pos.1;
        if !grid.is_in_bounds(&next) {
            return false;
        } else if grid[&next] {
            pos = (pos.0, pos.1.clockwise_quarter_turn());
        } else {
            pos = (next, pos.1);
        }
    }
}

struct Input {
    start: (Point, Direction),
    grid: Grid<bool>,
}

fn parse_input() -> Input {
    let mut start = None;
    let grid = Grid::parse(read_input(6), |c| c)
        .map(|p, c| match c {
            '#' => true,
            '>' => { start = Some((p, Direction::East)); false },
            '^' => { start = Some((p, Direction::North)); false },
            'v' => { start = Some((p, Direction::South)); false },
            '<' => { start = Some((p, Direction::West)); false },
            _ => false,
        });
    Input { start: start.unwrap(), grid }
}
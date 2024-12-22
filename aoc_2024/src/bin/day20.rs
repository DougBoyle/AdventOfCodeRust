use std::collections::HashMap;

use aoc_2024::read_input;
use rust_aoc::{count_occurrences, direction::Direction, grid::Grid, point::Point};

fn main() {
    part1();
    part2();
}

fn part1() {
    let Input { start, end, grid } = parse_input();

    let distances = calculate_regular_distances(start, end, &grid);

    let shortcuts = count_shortcut_lengths(&distances, 2);

    let total: usize = shortcuts.iter().filter(|(&saving, _)| saving >= 100).map(|(_, count)| count).sum();
    println!("Shortcuts saving >= 100 time: {total}"); // 1289
}

fn part2() {
    let Input { start, end, grid } = parse_input();

    let distances = calculate_regular_distances(start, end, &grid);

    let shortcuts = count_shortcut_lengths(&distances, 20);

    let total: usize = shortcuts.iter().filter(|(&saving, _)| saving >= 100).map(|(_, count)| count).sum();
    println!("Shortcuts saving >= 100 time: {total}"); // 982425
}

fn calculate_regular_distances(start: Point, end: Point, grid: &Grid<bool>) -> HashMap<Point, i32> {
    let mut distances = HashMap::new();
    distances.insert(start, 0);

    let mut current = start;
    let mut distance = 0;
    while current != end {
        let mut neighbours = Direction::neighbours(current).into_iter()
            .filter(|p| !distances.contains_key(p) && grid.is_in_bounds(p) && !grid[p]);
        let next = neighbours.next().unwrap();
        assert_eq!(None, neighbours.next());
        distance += 1;
        distances.insert(next, distance);
        current = next;
    }

    distances
}

fn count_shortcut_lengths(distances: &HashMap<Point, i32>, cheat_length: i64) -> HashMap<i32, usize> {
    let all_shortcuts = distances.keys().flat_map(|start| get_savings(*start, cheat_length, &distances));
    count_occurrences(all_shortcuts)
}

fn get_savings(start: Point, steps: i64, distances: &HashMap<Point, i32>) -> impl Iterator<Item=i32> + '_ {
    let start_distance = distances[&start];
    get_reachable(start, steps).filter_map(move |end| {
        let end_distance = distances.get(&end)?;
        let offset = start - end;
        let cost = offset.x.abs() + offset.y.abs();
        let saving = end_distance - start_distance - cost as i32;
        if saving > 0 {
            Some(saving)
        } else {
            None
        }
    })
}

fn get_reachable(start: Point, steps: i64) -> impl Iterator<Item=Point> {
    (-steps..=steps).flat_map(move |dx| {
        let remaining = steps - dx.abs();
        (-remaining..=remaining).map(move |dy| start + Point { x: dx, y: dy })
    })
}

struct Input {
    start: Point,
    end: Point,
    grid: Grid<bool>,
}

fn parse_input() -> Input {
    let mut start = None;
    let mut end = None;
    let grid = Grid::from_strings(read_input(20)).map(|p, c| {
        match c {
            'S' => { start = Some(p); false },
            'E' => { end = Some(p); false },
            '.' => false,
            '#' => true,
            _ => panic!("Unexpected character {c}"),
        }
    });
    Input { start: start.unwrap(), end: end.unwrap(), grid }
}

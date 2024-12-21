use std::collections::{hash_map::Entry, HashMap, HashSet};

use aoc_2024::read_input;
use rust_aoc::{direction::Direction, grid::Grid, point::Point, Dijkstra, ExhaustiveDijkstra};

fn main() {
    part1();
    part2();
}

fn part1() {
    let (grid, start, end) = parse_input();
    
    let search = GridSearch { grid, end, min_costs: HashMap::new() };

    let min_cost = search.search(vec![(start, Direction::East)]);

    println!("Min cost: {min_cost}"); // 99448
}

fn part2() {
    let (grid, start, end) = parse_input();
    
    let search = GridSearch { grid, end, min_costs: HashMap::new() };

    let points_on_path: HashSet<_> = search.all_min_paths_nodes(vec![(start, Direction::East)])
        .into_iter()
        .map(|(p, _)| p)
        .collect();

    println!("Total: {}", points_on_path.len()); // 498
}

struct GridSearch {
    grid: Grid<bool>,
    end: Point,
    min_costs: HashMap<(Point, Direction), usize>
}

type State = (Point, Direction);

impl Dijkstra for GridSearch {
    type State = State;

    fn is_end(&self, &(p, _): &Self::State) -> bool {
        p == self.end
    }

    fn neighbours(&self, &(p, dir): &Self::State) -> Vec<(usize, Self::State)> {
        let mut result: Vec<_> = dir.perpendicular().map(|d| (1000, (p, d))).into_iter().collect();
        let next = p + dir;
        if !self.grid[&next] {
            result.push((1, (next, dir)));
        }
        result
    }

   
    fn try_improve(&mut self, state: &Self::State, cost: usize) -> bool {
        match self.min_costs.entry(*state) {
            Entry::Occupied(mut entry) => {
                if cost < *entry.get() {
                    entry.insert(cost);
                    true
                } else {
                    false
                }
            },
            Entry::Vacant(entry) => {
                entry.insert(cost);
                true
            },
        }
    }
}

impl ExhaustiveDijkstra for GridSearch {
    fn try_not_worse(&mut self, state: &Self::State, cost: usize) -> bool {
        match self.min_costs.entry(*state) {
            Entry::Occupied(mut entry) => {
                // <= rather than <, we want to search all equally good paths
                if cost <= *entry.get() {
                    entry.insert(cost);
                    true
                } else {
                    false
                }
            },
            Entry::Vacant(entry) => {
                entry.insert(cost);
                true
            },
        }
    }
}

fn parse_input() -> (Grid<bool>, Point, Point) {
    let mut start = None;
    let mut end = None;
    let grid = Grid::from_strings(read_input(16)).map(|p, c| match c {
        'S' => { start = Some(p); false },
        'E' => { end = Some(p); false },
        '.' => false,
        '#' => true,
        _ => panic!("Unexpected input character '{c}'"),
    });
    (grid, start.unwrap(), end.unwrap())
}

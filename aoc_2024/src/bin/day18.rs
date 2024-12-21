use std::collections::{hash_map::Entry, HashMap, HashSet};

use aoc_2024::read_input;
use rust_aoc::{direction::Direction, grid::Grid, point::Point, split_in_two, Dijkstra};

fn main() {
    part1();
    part2();
}

const LENGTH: i64 = 71;
const START: Point = Point { x: 0, y: 0 };
const END: Point = Point { x: LENGTH - 1, y: LENGTH - 1 };

fn part1() {
    let points = parse_input();
    let mut grid = Grid::generate(LENGTH, LENGTH, &mut |_| false);
    for point in &points[..1024] {
        grid[point] = true;
    }

    let min_cost = Search::new(&grid)
        .search(vec![START])
        .unwrap();

    println!("Min cost: {min_cost}"); // 364
}

fn part2() {
    let points = parse_input();
    let mut grid = Grid::generate(LENGTH, LENGTH, &mut |_| false);
    for point in &points[..1024] {
        grid[point] = true;
    }

    // Initial valid path
    let mut searches = 1;
    let mut search = Search::new(&grid);
    search.search(vec![START]).unwrap();
    let mut current_path: HashSet<_> = search.extract_points_on_path();

    for (i, point) in points[1024..].iter().enumerate() {
        grid[point] = true;
        if current_path.contains(point) {
            // potentially blocked route
            searches += 1;
            let mut search = Search::new(&grid);
            match search.search(vec![START]) {
                Some(_) => current_path = search.extract_points_on_path(),
                None => {
                    println!("First failure after point {i} (took {searches} searches): {point}"); // 52,28
                    break;
                }
            }
        }
    }
}



struct Search<'a> {
    grid: &'a Grid<bool>,
    cheapest: HashMap<Point, usize>,
}

impl<'a> Search<'a> {
    fn new(grid: &'a Grid<bool>) -> Self {
        Search { grid, cheapest: HashMap::new() }
    }

    fn extract_points_on_path(&self) -> HashSet<Point> {
        self.extract_path().into_iter().collect()
    }

    // relies on costs being symmetric
    fn extract_path(&self) -> Vec<Point> {
        let mut current = END;
        let mut rev_path = vec![current];
        while current != START {
            let current_cost = self.cheapest[&current];
            let next = self.neighbours(&current).into_iter()
                .filter(|(cost, next)| self.cheapest.get(next) == Some(&(current_cost - cost)))
                .map(|(_, next)| next)
                .next()
                .unwrap();
            rev_path.push(next);
            current = next;
        }
        rev_path.reverse();
        rev_path
    }
}

impl Dijkstra for Search<'_> {
    type State = Point;

    fn is_end(&self, point: &Self::State) -> bool {
        *point == END
    }

    fn neighbours(&self, point: &Self::State) -> Vec<(usize, Self::State)> {
        Direction::all().map(|d| *point + d).into_iter()
            .filter(|next| self.grid.is_in_bounds(next) && !self.grid[next])
            .map(|next| (1, next))
            .collect()
    }

    fn try_improve(&mut self, state: &Self::State, cost: usize) -> bool {
        match self.cheapest.entry(*state) {
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


fn parse_input() -> Vec<Point> {
    read_input(18).map(|line| {
        let (x, y) = split_in_two(&line, ',');
        Point { x: x.parse().unwrap(), y: y.parse().unwrap() }
    }).collect()
}

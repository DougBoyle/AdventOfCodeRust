use std::collections::HashSet;

use aoc_2024::read_input;
use rust_aoc::{direction::Direction, grid::Grid, point::Point};


fn main() {
    part1();
    part2();
}

fn part1() {
    let grid = read_grid();
    let mut marked = grid.clone().map(|_, _| false);

    let mut total = 0;

    for (start, _) in grid.enumerate() {
        if !marked[&start] {
            total += find_region_price(start, &grid, &mut marked);
        }
    }

    println!("Total: {total}"); // 1488414
}

fn part2() {
    let grid = read_grid();
    let mut marked = grid.clone().map(|_, _| false);

    let mut total = 0;

    for (start, _) in grid.enumerate() {
        if !marked[&start] {
            total += find_region_price2(start, &grid, &mut marked);
        }
    }

    println!("Total: {total}"); // 911750
}

fn find_region_price(start: Point, grid: &Grid<char>, marked: &mut Grid<bool>) -> usize {
    struct Visitor {
        area: usize,
        perimeter: usize,
    }
    impl RegionVisitor for Visitor {
        fn cell(&mut self, _: Point) {
            self.area += 1;
        }
    
        fn edge(&mut self, _: Point, _: Direction) {
            self.perimeter += 1;
        }
    }

    let mut visitor = Visitor { area: 0, perimeter: 0 };
    visit_region(start,grid, marked, &mut visitor);
    visitor.area * visitor.perimeter
}

fn find_region_price2(start: Point, grid: &Grid<char>, marked: &mut Grid<bool>) -> usize {
    struct Visitor<'a> {
        area: usize,
        sides: usize,
        visited: HashSet<Point>,
        grid: &'a Grid<char>,
    }
    impl RegionVisitor for Visitor<'_> {
        fn cell(&mut self, p: Point) {
            self.area += 1;
            self.visited.insert(p);
        }
    
        fn edge(&mut self, p: Point, d: Direction) {
            self.sides += 1;
            let adjacent_borders = d.perpendicular().map(|other_dir| p + other_dir)
                .into_iter()
                // adjacent tiles that are part of the region and already visited
                .filter(|adjacent| is_same_region(&p, &adjacent, self.grid) && self.visited.contains(adjacent))
                // and have the same border
                .filter(|adjacent| !is_same_region(&p, &(*adjacent + d), self.grid))
                .count();
            self.sides -= adjacent_borders; // If there's a border on either side, we double-counted earlier and will now decrement overall
        }
    }

    let mut visitor = Visitor { area: 0, sides: 0, visited: HashSet::new(), grid };
    visit_region(start, grid, marked, &mut visitor);
    visitor.area * visitor.sides
}

trait RegionVisitor {
    fn cell(&mut self, p: Point);
    fn edge(&mut self, p: Point, d: Direction);
}

fn visit_region<Visitor: RegionVisitor>(start: Point, grid: &Grid<char>, marked: &mut Grid<bool>, visitor: &mut Visitor) {
    let mut to_visit = vec![start];
    marked[&start] = true;
    while let Some(p) = to_visit.pop() {
        visitor.cell(p);
        for d in Direction::all() {
            let neighbour = p + d;
            if is_same_region(&p, &neighbour, grid) {
                if !marked[&neighbour] {
                    marked[&neighbour] = true;
                    to_visit.push(neighbour);
                }
            } else {
                visitor.edge(p, d);
            }
        }
    }
}

fn is_same_region(p1: &Point, p2: &Point, grid: &Grid<char>) -> bool {
    grid.is_in_bounds(p1) && grid.is_in_bounds(p2) && grid[p1] == grid[p2]
}

fn read_grid() -> Grid<char> {
    Grid::parse(read_input(12), |c| c)
}
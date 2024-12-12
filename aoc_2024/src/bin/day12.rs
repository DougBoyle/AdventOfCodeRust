use std::collections::HashSet;

use aoc_2024::read_input;
use rust_aoc::{direction::Direction, grid::Grid, point::Point};


fn main() {
    part1::run();
    part2::run();
}

mod part1 {
    use super::*;

    pub fn run() {
        let grid = read_grid();
        let mut factory = Factory { total: 0 };
        visit_all_regions(&mut factory, &grid);
        println!("Total: {}", factory.total); // 1488414
    }

    struct Factory { total: usize }

    impl VisitorFactory for Factory {
        type Visitor = Visitor;
    
        fn new_region(&mut self) -> Visitor {
            Visitor { area: 0, perimeter: 0 }
        }
    
        fn region_complete(&mut self, visitor: Visitor) {
            self.total += visitor.area * visitor.perimeter;
        }
    }
    
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
}

mod part2 {
    use super::*;

    pub fn run() {
        let grid = read_grid();
        let mut factory = Factory { grid: &grid, total: 0 };
        visit_all_regions(&mut factory, &grid);
    
        println!("Total: {}", factory.total); // 911750
    }

    struct Factory<'a> { grid: &'a Grid<char>, total: usize }

    impl<'a> VisitorFactory for Factory<'a> {
        type Visitor = Visitor<'a>;
    
        fn new_region(&mut self) -> Visitor<'a> {
            Visitor { area: 0, sides: 0, visited: HashSet::new(), grid: &self.grid }
        }
    
        fn region_complete(&mut self, visitor: Visitor) {
            self.total += visitor.area * visitor.sides;
        }
    }
    
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
}

trait VisitorFactory {
    type Visitor: RegionVisitor;
    fn new_region(&mut self) -> Self::Visitor;
    fn region_complete(&mut self, visitor: Self::Visitor);
}

trait RegionVisitor {
    fn cell(&mut self, p: Point);
    fn edge(&mut self, p: Point, d: Direction);
}

fn visit_all_regions<Factory: VisitorFactory>(factory: &mut Factory, grid: &Grid<char>) {
    let mut marked = grid.clone().map(|_, _| false);

    for (start, _) in grid.enumerate() {
        if !marked[&start] {
            let mut visitor  = factory.new_region();
            visit_region(start, grid, &mut marked, &mut visitor);
            factory.region_complete(visitor);
        }
    }
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
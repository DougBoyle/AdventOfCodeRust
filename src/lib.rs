
use std::{
    collections::{BinaryHeap, VecDeque}, fs::File, io::{BufRead, BufReader}
};

pub mod point;
pub mod direction;
pub mod grid;

use point::Point;

pub fn read_input(day: u32) -> impl Iterator<Item=String> {
    let filename = format!("resources/day{day}.txt");
    let f = File::open(&filename).expect(format!("Couldn't open {filename}").as_str());
    BufReader::new(f).lines().map(Result::unwrap)
}

/// f(point, character) for each cell of the grid, with the first character in the top left being Point { x: 0, y: 0 }
pub fn process_grid<F: FnMut(Point, char)>(day: u32, mut f: F) {
    read_input(day).enumerate().for_each(|(y, line)| line.chars().enumerate().for_each(|(x, c)|
        f(Point { x: x.try_into().unwrap(), y: y.try_into().unwrap() }, c)
    ));
}

pub fn split_in_two(s: &str, separator: char) -> (&str, &str) {
    let split: Vec<_> = s.split(separator).collect();
    assert!(split.len() == 2);
    (split[0], split[1])
}

pub fn assert_single<T, I: Iterator<Item=T>>(it: I) -> T {
    let items: Vec<_> = it.collect();
    assert!(items.len() == 1);
    items.into_iter().next().unwrap()
}

pub trait BreadthFirstSearch : Sized {
    type Node;

    /// Returns true if state was updated i.e. the node was not already marked
    fn mark(&mut self, node: &Self::Node) -> bool;
    fn neighbours(&self, node: &Self::Node) -> Vec<Self::Node>;

    fn search(mut self, start: Self::Node) {
        let mut to_process = VecDeque::new();

        // begin in a clean state, so 'start' should not already be marked
        assert!(self.mark(&start));
        to_process.push_back(start);

        while let Some(node) = to_process.pop_front() {
            for next in self.neighbours(&node) {
                if self.mark(&next) { to_process.push_back(next) }
            }
        }
    }
}

pub trait Dijkstra: Sized {
    type State;

    fn is_end(&self, state: &Self::State) -> bool;

    fn neighbours(&self, value: &Self::State) -> Vec<(usize, Self::State)>;

    fn try_improve(&mut self, state: &Self::State, cost: usize) -> bool;

    fn search(mut self, starts: Vec<Self::State>) -> usize {
        let mut to_explore: BinaryHeap<DijkstraCost<Self::State>> = BinaryHeap::new();
        for start in starts {
            to_explore.push(DijkstraCost {value: start, cost: 0});
        }

        loop {
            let DijkstraCost { cost, value } = to_explore.pop().unwrap();
            if self.is_end(&value) { return cost }

            for (added_cost, new_value) in self.neighbours(&value) {
                let new_cost = cost + added_cost;
                if self.try_improve(&new_value, new_cost) {
                    to_explore.push(DijkstraCost { cost: new_cost, value: new_value });
                }
            }
        }
    }
}

#[derive(Debug)]
struct DijkstraCost<T> {
    value: T,
    cost: usize
}

/// Reversed ordering so that BinaryHeap results in a min heap, not max heap.
/// DijkstraCost struct only used for BinaryHeap ordering, not public, hence ignores actual value for comparison.
impl<T> Ord for DijkstraCost<T> {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        other.cost.cmp(&self.cost)
    }
}

impl<T> PartialOrd for DijkstraCost<T> {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl<T> PartialEq for DijkstraCost<T> {
    fn eq(&self, other: &Self) -> bool {
        self.cost == other.cost
    }
}

impl<T> Eq for DijkstraCost<T> {}


/// Shoelace formula and Pick's theorem:
/// Shoelace formula (https://en.wikipedia.org/wiki/Shoelace_formula) - For a shape defined by a sequence of corners 
///   (positively oriented i.e. going counter clockwise), the area is the sum of 1/2 (x_i - x_{i+1})(y_i + y_{i+1}).
///   This can be demonstrated by adding/subtracting trapezoids, with the other two points on the x-axis, as the area
///   of the trapezoid [(x1, 0), (x1, y1), (x2, y2), (x2, 0)] is (x2 - x1)(0.5(y1 + y2)) i.e. base * average height.
///   It can also be viewed as many triangles, with the third point at the origin. Either way, it can be simplified to
///   0.5 * Sum { x_i ( y_{i+1} - y_{i-1} ) }   (wrapping around where necessary)
/// Pick's theorem (https://en.wikipedia.org/wiki/Pick%27s_theorem) - The area of a "simple" polygon with integer coordinate vertices is
///   A = i + b/2 - 1, where A=area, i=number of integer coordinates inside the shape, b=integer points along bounary (i.e. edges/corners).
/// 
/// Combining the two:
///   Both problems are described in terms of points that make up the boundary of a shape. If instead we're thinking of each point as a "cell",
///   then the shape [(0,0), (0,1), (1,1), (1,0)] has an area of 4 rather than 1. To solve this discrepancy, think of each point as being the
///   center of that cell:
/// +------+------+ <-- corner of "cell", unrelated to interior/boundary points. We want the area of this outer shape = number of cells = 4
/// |      |      |
/// |  *---|---*  | <-- boundary "point" (1,1) in center of cell, Shoelace formula/Pick's theorem give the area of this inner shape = 1
/// |  |   |   |  |
/// +------+------+
/// |  |   |   |  |
/// |  *---|---*  |
/// |      |      |
/// +------+------+
///   The Shoelace formula and Pick's theorem describe the same area A = the inner area of the shape outlined by the center points,
///   with the Shoelace formula being the easier way of calculating that. The number of cells is really just the number of "interior"
///   and "boundary" points as describe in Pick's theorem i.e. counting the number of cell centers, with all 4 points in the 2x2 example
///   above being boundary points, and the actual borders of the "cells" having nothing to do with interior/boundary points.
/// We want Number of Cells = i + b
/// A = Area of the shape bounded by the cell centers, gotten from the Shoelace formula.
/// b = Boundary points = number of cells on the perimeter, easy to count from the description of the shape's border cells.
/// Pick's theorem: A = i + b/2 - 1      =>      i = A - b/2 + 1
/// Therefore: Number of Cells = i + b = (A - b/2 + 1) + b = A + b/2 + 1
/// Hence to go from Shoelace formula result to number of cells, just add (cells on perimeter / 2 + 1)
fn shoelace_area_from_boundary_points(points: &Vec<Point>) -> i64 {
    let len = points.len();
    let total: i64 = (0..len).map(|i| {
        let p = points[i];
        let previous = points[(i + len - 1) % len];
        let next = points[(i + 1) % len];
        (p.x as i64) * ((next.y - previous.y) as i64)
    }).sum();
    let total = total / 2;
    // rather than enforcing the correct orientation, just flip sign if end result is negative
    if total < 0 { -total } else { total }
}

pub fn shoelace_area_enclosed_cells_including_border(perimeter_cells: &Vec<Point>) -> i64 {
    let len = perimeter_cells.len();
    let perimeter_len: u32 = (0..len).map(|i| {
        let p1 = &perimeter_cells[i];
        let p2 = &perimeter_cells[(i+1) % len];
        if !p1.is_orthogonal_to(p2) { panic!("Invalid cell outline, not grid aligned!") }
        p1.orthogonal_distance(p2)
    }).sum();
    let perimeter_len = perimeter_len as i64;
    shoelace_area_from_boundary_points(perimeter_cells) + (perimeter_len/2) + 1
}

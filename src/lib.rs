
use std::{
    collections::VecDeque, fs::File, io::{BufRead, BufReader}
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
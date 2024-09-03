use std::collections::{BinaryHeap, HashMap};

use rust_aoc::{direction::Direction, grid::Grid, point::Point};

fn main() {
    let tiles: Grid<usize> = Grid::parse(rust_aoc::read_input(17), |c| c.to_digit(10).unwrap() as usize);

    let start_point = Point {x: 0, y: 0};
    let end_point = Point {x: tiles.width - 1, y: tiles.height - 1};
    // 'start' is the only case where steps=0 and direction irrelevant, since we haven't moved yet
    let starts = [Direction::East, Direction::South]
        .map(|last_dir| Crucible { point: start_point, last_dir, steps: 0 })
        .into_iter().collect();    

    let grid_search = GridSearch {
        tiles: &tiles,
        state_upper_bounds: HashMap::new(),
        constraints: Constraints { min_steps: 1, max_steps: 3 }
    };

    let min_cost = grid_search.search(&starts, |Crucible { point, .. }| *point == end_point );
    
    println!("Part 1: Min cost {min_cost}"); // 635

    let grid_search = GridSearch {
        tiles: &tiles,
        state_upper_bounds: HashMap::new(),
        constraints: Constraints { min_steps: 4, max_steps: 10 }
    };

    let min_cost = grid_search.search(&starts, |Crucible { point, steps, .. }| *point == end_point && *steps >= 4 );
    
    println!("Part 2: Min cost {min_cost}"); // 734
}

struct Constraints {
    min_steps: usize,
    max_steps: usize,
}

struct GridSearch<'a> {
    tiles: &'a Grid<usize>,
    // Point + Direction => Steps => Cost
    // A state is an improvement if, for that point, for its number of steps and all below it,
    // none of them have a better cost.
    state_upper_bounds: HashMap<Crucible, usize>,
    constraints: Constraints,
}

// TODO: Turn into a library trait?
impl GridSearch<'_> {
    fn search<IsEnd: Fn(&Crucible) -> bool>(mut self, starts: &Vec<Crucible>, is_end: IsEnd) -> usize {
        let mut to_explore: BinaryHeap<Costed<Crucible>> = BinaryHeap::new();
        for start in starts {
            to_explore.push(Costed {value: *start, cost: 0});
        }

        loop {
            let Costed { cost, value } = to_explore.pop().unwrap();
            if is_end(&value) { return cost }

            for (added_cost, new_value) in self.neighbours(&value) {
                let new_node = Costed { cost: cost + added_cost, value: new_value };
                if self.try_improve(&new_node) {
                    to_explore.push(new_node);
                }
            }
        }
    }

    fn try_improve(&mut self, &Costed { cost, value }: &Costed<Crucible>) -> bool {
        match self.state_upper_bounds.get_mut(&value) {
            Some(existing_cost) => {
                if *existing_cost > cost {
                    *existing_cost = cost;
                    true
                } else {
                    false
                }
            },
            None => {
                self.state_upper_bounds.insert(value, cost);
                true
            }
        }
    }

    fn neighbours(&self, value: &Crucible) -> Vec<(usize, Crucible)> {
        let &Crucible { point, last_dir, steps } = value;
        
        Direction::all().into_iter()
            .filter(|d| *d != last_dir.opposite() && (*d == last_dir || steps >= self.constraints.min_steps))
            .map(|d| Crucible {
                point: point + d,
                 last_dir: d, 
                 steps: if last_dir == d { steps + 1 } else { 1 } 
                })
            .filter(|Crucible { point, steps, .. }| *steps <= self.constraints.max_steps && self.tiles.is_in_bounds(point) )
            .map(|crucible| {
                let cost = self.tiles[&crucible.point];
                (cost, crucible)
            })
            .collect()
    }
}

#[derive(Debug)]
struct Costed<T> {
    value: T,
    cost: usize
}

/// Reversed ordering so that BinaryHeap results in a min heap, not max heap.
/// Costed struct only used for BinaryHeap ordering, hence ignores actual value for comparison.
impl<T> Ord for Costed<T> {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        other.cost.cmp(&self.cost)
    }
}

impl<T> PartialOrd for Costed<T> {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl<T> PartialEq for Costed<T> {
    fn eq(&self, other: &Self) -> bool {
        self.cost == other.cost
    }
}

impl<T> Eq for Costed<T> {}

#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash)]
struct Crucible {
    point: Point,
    last_dir: Direction,
    steps: usize,
}

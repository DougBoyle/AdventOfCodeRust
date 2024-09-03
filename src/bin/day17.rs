use std::collections::{BTreeMap, BinaryHeap, HashMap};

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
    state_upper_bounds: HashMap<(Point, Direction), BTreeMap<usize, usize>>,
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
            let node = to_explore.pop().unwrap();
            if is_end(&node.value) { return node.cost }

            for new_node in node.neighbours(&self.constraints, &self.tiles) {
                if self.try_improve(&new_node) {
                    to_explore.push(new_node);
                }
            }
        }
    }

    fn try_improve(&mut self, &Costed { cost, value: Crucible { point, last_dir, steps }}: &Costed<Crucible>) -> bool {
        let costs_for_steps = self.state_upper_bounds.entry((point, last_dir))
            .or_insert_with(|| BTreeMap::new());
        // TODO: Allowing any shorter step no longer works once lower bound introduced?
 //       for (_, &existing_cost) in costs_for_steps.range(0..=steps) {
        if let Some(&existing_cost) = costs_for_steps.get(&steps) {
            if existing_cost <= cost { return false }
        }
        costs_for_steps.insert(steps, cost);
        true
    }
}

impl Costed<Crucible> {
    fn neighbours(&self, constraints: &Constraints, tiles: &Grid<usize>) -> Vec<Costed<Crucible>> {
        let &Costed { cost, value: Crucible { point, last_dir, steps } } = self;
        
        Direction::all().into_iter()
            .filter(|d| *d != last_dir.opposite() && (*d == last_dir || steps >= constraints.min_steps))
            .map(|d| Crucible {
                point: point + d,
                 last_dir: d, 
                 steps: if last_dir == d { steps + 1 } else { 1 } 
                })
            .filter(|Crucible { point, steps, .. }| *steps <= constraints.max_steps && tiles.is_in_bounds(point) )
            .map(|crucible| {
                let added_cost = tiles[&crucible.point];
                Costed { value: crucible, cost: cost + added_cost }
            })
            .collect()
    }
}

#[derive(PartialEq, Eq, Debug)]
struct Costed<T: Ord> {
    value: T,
    cost: usize
}

/// Reversed ordering so that BinaryHeap results in a min heap, not max heap
impl<T: Ord> Ord for Costed<T> {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        other.cost.cmp(&self.cost).then_with(|| other.value.cmp(&self.value))
    }
}

impl<T: Ord> PartialOrd for Costed<T> {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

#[derive(PartialEq, Eq, Debug, Clone, Copy)]
struct Crucible {
    point: Point,
    last_dir: Direction,
    steps: usize,
}

impl Ord for Crucible {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.steps.cmp(&other.steps)
            .then_with(|| self.point.cmp(&other.point))
            .then_with(|| self.last_dir.cmp(&other.last_dir))
    }
}

impl PartialOrd for Crucible {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}



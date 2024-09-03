use std::collections::HashMap;

use rust_aoc::{direction::Direction, grid::Grid, point::Point, Dijkstra};

fn main() {
    let tiles: Grid<usize> = Grid::parse(rust_aoc::read_input(17), |c| c.to_digit(10).unwrap() as usize);

    let start_point = Point {x: 0, y: 0};
    // 'start' is the only case where steps=0 and direction irrelevant, since we haven't moved yet
    let starts: Vec<_> = [Direction::East, Direction::South]
        .map(|last_dir| Crucible { point: start_point, last_dir, steps: 0 })
        .into_iter().collect();    

    let grid_search = GridSearch {
        tiles: &tiles,
        state_upper_bounds: HashMap::new(),
        constraints: Constraints { min_steps: 1, max_steps: 3 }
    };

    let min_cost = grid_search.search(starts.clone());
    
    println!("Part 1: Min cost {min_cost}"); // 635

    let grid_search = GridSearch {
        tiles: &tiles,
        state_upper_bounds: HashMap::new(),
        constraints: Constraints { min_steps: 4, max_steps: 10 }
    };

    let min_cost = grid_search.search(starts);
    
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

impl Dijkstra for GridSearch<'_> {
    type State = Crucible;

    fn is_end(&self, &Crucible { point, steps, .. }: &Crucible) -> bool {
        let end_point = Point {x: self.tiles.width - 1, y: self.tiles.height - 1};
        point == end_point && steps >= self.constraints.min_steps
    }

    fn neighbours(&self, value: &Self::State) -> Vec<(usize, Self::State)> {
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

    fn try_improve(&mut self, state: &Self::State, cost: usize) -> bool {
        match self.state_upper_bounds.get_mut(&state) {
            Some(existing_cost) => {
                if *existing_cost > cost {
                    *existing_cost = cost;
                    true
                } else {
                    false
                }
            },
            None => {
                self.state_upper_bounds.insert(*state, cost);
                true
            }
        }
    }
}

#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash)]
struct Crucible {
    point: Point,
    last_dir: Direction,
    steps: usize,
}

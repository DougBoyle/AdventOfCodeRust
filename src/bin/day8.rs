use std::{collections::{BTreeSet, HashMap, HashSet}, io::{Error, ErrorKind}, str::FromStr};

fn main() {
    let mut input = rust_aoc::read_input(8);
    let directions: Vec<_> = input.next().unwrap().chars().map(Direction::try_from).map(Result::unwrap).collect();
    let steps_per_cycle = directions.len();
    println!("Steps before cycling: {steps_per_cycle}"); // ~300

    // blank line
    input.next();

    let nodes: HashMap<String, Node> = input.map(|s| s.parse()).map(Result::unwrap).map(|node: Node| (node.name.clone(), node)).collect();

    let total_num_nodes = nodes.len();
    println!("Total number of nodes: {total_num_nodes}"); // ~700
    println!("Worst case loop length: {}", steps_per_cycle * total_num_nodes); // ~200k

    println!("Part 1");
    Part1::process(&directions, &nodes);
    println!("Part 2");
    Part2::process(&directions, &nodes);
}

trait Part: Sized {
    fn initial_names(nodes: &HashMap<String, Node>) -> Vec<&str>;

    fn finished_node(name: &str) -> bool;

    fn process(directions: &Vec<Direction>, nodes: &HashMap<String, Node>) {
        let start_names = Self::initial_names(&nodes);
        let num_start_nodes = start_names.len();
        println!("Number of start nodes: {num_start_nodes}"); // 6
        println!("Worst case complexity: {}", num_start_nodes * directions.len() * nodes.len()); // ~1.2M

        let cycles: Vec<_> = start_names.iter().map(|start| Cycle::new::<Self>(start, &directions, &nodes)).collect();

        let overall_cycle = cycles.into_iter().reduce(Cycle::intersect).unwrap();

       // 1. 22357
       // 2. 10371555451871
        println!("First endpoint: {:#?}", overall_cycle.first_endpoint());
    }
}

struct Part1;

impl Part for Part1 {
    fn initial_names(_: &HashMap<String, Node>) -> Vec<&str> {
        vec!["AAA"]
    }

    fn finished_node(name: &str) -> bool {
        name == "ZZZ"
    }
}

struct Part2;

impl Part for Part2 {
    fn initial_names(nodes: &HashMap<String, Node>) -> Vec<&str> {
        nodes.keys().filter(|key| key.ends_with('A')).map(std::ops::Deref::deref).collect()
    }

    fn finished_node(name: &str) -> bool {
        name.ends_with('Z')
    }
}

impl Part2 {

}

#[derive(Debug)]
struct Cycle {
    initial_offsets: BTreeSet<usize>, // endpoints before the path enters a cycle
    cycle_offset: usize,
    cycle_period: usize,
    cycle_offsets: BTreeSet<usize>, // x -> endpoints at (cycle_offset + x + n*cycle_period) for all n
}

impl Cycle {
    fn new<Rules: Part>(start: &str, dirs: &Vec<Direction>, nodes: &HashMap<String, Node>) -> Cycle {
        let mut visited_step: HashMap<(usize, &str), usize> = HashMap::new();
        let mut endpoints: HashSet<usize> = HashSet::new();

        let mut current_value = start;
        let mut dir_idx = 0;
        let mut steps = 0;

        while !visited_step.contains_key(&(dir_idx, current_value)) {
            visited_step.insert((dir_idx, current_value), steps);
            if Rules::finished_node (current_value) {
                endpoints.insert(steps);
            }
            let dir = dirs[dir_idx];

            dir_idx = (dir_idx + 1) % dirs.len();
            steps += 1;

            current_value = step(current_value, dir, &nodes); 
        }

        let cycle_offset = *visited_step.get(&(dir_idx, current_value)).unwrap();
        let cycle_period = steps - cycle_offset;
        let (initial_offsets, endpoints_in_cycle): (BTreeSet<_>, BTreeSet<_>) = endpoints.iter().partition(|steps| **steps < cycle_offset);
        let cycle_offsets = endpoints_in_cycle.iter().map(|steps| steps - cycle_offset).collect();
        Cycle { initial_offsets, cycle_offset, cycle_period, cycle_offsets }
    }

    fn is_endpoint(&self, n: usize) -> bool {
        if n < self.cycle_offset {
            self.initial_offsets.contains(&n)
        } else {
            let n = n - self.cycle_offset;
            let n = n % self.cycle_period;
            self.cycle_offsets.contains(&n)
        }
    }

    fn first_endpoint(&self) -> Option<usize> {
        self.initial_offsets.first().map(|n| *n).or_else(|| self.cycle_offsets.first().map(|offset| self.cycle_offset + *offset))
    }

    fn intersect(self, other: Self) -> Self {
        let combined_period = num::integer::lcm(self.cycle_period, other.cycle_period);

        // filter initial offsets
        let initial_offsets = self.initial_offsets.into_iter().filter(|n| other.is_endpoint(*n)).collect();
        
        // populate extra cycle offsets for the longer period, filtering ones that don't intersect
        let period_multiplier = combined_period / self.cycle_period;
        let mut cycle_offsets = BTreeSet::new();
        for offset in self.cycle_offsets {
            for iteration in 0..period_multiplier {
                let n = self.cycle_offset + self.cycle_period * iteration + offset;
                if other.is_endpoint(n) {
                    cycle_offsets.insert(n - self.cycle_offset);
                }
            }
        }

        Cycle { initial_offsets, cycle_offset: self.cycle_offset, cycle_period: combined_period, cycle_offsets }
    }
}

#[derive(Eq, PartialEq, Clone, Copy)]
enum Direction {
    Left,
    Right,
}

impl TryFrom<char> for Direction {
    type Error = Error;
    fn try_from(c: char) -> Result<Self, Self::Error> {
        match c {
            'L' => Ok(Direction::Left),
            'R' => Ok(Direction::Right),
            _ => Err(Error::new(ErrorKind::InvalidInput, format!("Unrecognised Direction: {c}")))
        }
    }
}

struct Node {
    name: String,
    left: String,
    right: String,
}

impl FromStr for Node {
    type Err = Error;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        // AAA = (BBB, CCC)
        let name = String::from(&s[0..3]);
        let left = String::from(&s[7..10]);
        let right = String::from(&s[12..15]);
        Ok(Node {name, left, right})
    }
}

fn step<'a>(name: &str, dir: Direction, nodes: &'a HashMap<String, Node>) -> &'a str {
    let node = nodes.get(name).unwrap();
    if dir == Direction::Left { &node.left } else { &node.right }
}



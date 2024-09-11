use std::{collections::{HashMap, HashSet}, io::{Error, ErrorKind}};

use rust_aoc::{direction::Direction, grid::Grid, point::Point};


fn main() {
    let grid: Grid<Cell> = Grid::parse(rust_aoc::read_input(23), |c| c.try_into().unwrap());

    let (start, _) = grid.row(0).iter().enumerate().filter(|(_, &cell)| cell == Cell::Empty).next().unwrap();
    let start = Point { x: start as i32, y: 0 };
    let (end, _) = grid.row((grid.height - 1) as usize).iter().enumerate()
        .filter(|(_, &cell)| cell == Cell::Empty).next().unwrap();
    let end = Point { x: end as i32, y: grid.height - 1 };

    println!("Start {start}, End {end}");

    // Abstract grid into a graph, where each node is a point in the grid that is one of:
    // 1) the start, 2) the end, 3) a non-trivial node with 3 possible next steps (1 of which is going back the same way so ignored)

    let graph = Graph::new(&start, &end, &grid);
    println!("Grid size {} x {} = {}; Graph size {}", grid.width, grid.height, grid.width * grid.height, graph.nodes.len());
    println!("Part 1 Longest path: {}", graph.longest_path()); // 1966

    // Part 2, 'slippy' cells no longer slippy, more options available
    let grid = grid.map(|_, cell| match cell {
        Cell::West | Cell::East | Cell::North | Cell::South => Cell::Empty,
        _ => cell
    });
    let graph = Graph::new(&start, &end, &grid);
    println!("Grid size {} x {} = {}; Graph size {}", grid.width, grid.height, grid.width * grid.height, graph.nodes.len());
    println!("Part 2 Longest path: {}", graph.longest_path()); // 6286
}

#[derive(Debug)]
struct Graph {
    // x -> (y, d) means there is a d-length trivial path from x to y.
    // if there are multiple trivial paths from x to y, only the longest is considered.
    nodes: HashMap<Point, HashMap<Point, usize>>,
    start: Point,
    end: Point,
}

impl Graph {
    fn new(start: &Point, end: &Point, grid: &Grid<Cell>) -> Self {
        let nodes = Graph::find_nodes(start, end, grid);
        let nodes = Graph::node_edges(nodes, grid);
        Graph { nodes, start: *start, end: *end }
    }

    // effectively a depth first search, allowed to revisit a node via different paths
    fn longest_path(&self) -> usize {
        let mut max_dist = 0;
        let mut visited = HashSet::new();
        let mut to_explore = vec![(self.start, 0)];
        while let Some((node, distance)) = to_explore.pop() {
            if node == self.end {
                max_dist = usize::max(max_dist, distance);
            } else if visited.contains(&node) {
                // done exploring, backtrace from this node
                visited.remove(&node);
            } else {
                // mark as visited, put back on stack to 'leave' later, and explore children
                visited.insert(node);
                to_explore.push((node, distance));
                for (neighbour, cost) in self.nodes[&node].iter().filter(|(neighbour, _)| !visited.contains(&neighbour)) {
                    to_explore.push((*neighbour, distance + cost));
                }
            }
        }
        max_dist
    }

    // 1. find the non-trivial nodes
    fn find_nodes(start:&Point, end: &Point, grid: &Grid<Cell>) -> HashSet<Point> {
        // Start, end, and any point surrounded by <2 walls.
        // e.g. In example below, middle tile only has 1 'neighbour' it can go to, but 
        //      several paths lead to it
        //      v
        //     >v<
        //      .
        grid.enumerate().filter(|(_, &cell)| cell != Cell::Wall).map(|(p, _)| p).filter(|p| {
            p == start || p == end || surrounding_walls(p, grid) < 2
        }).collect()
    }

    // 2. Work out the longest paths between each pair of adjacent nodes
    fn node_edges(nodes: HashSet<Point>, grid: &Grid<Cell>) -> HashMap<Point, HashMap<Point, usize>> {
        nodes.iter().map(|node| {
            let mut edges = HashMap::new();
            for immediate_neighbour in neighbours(node, grid) {
                if let Some((next_node, distance)) = Graph::distance_to_next_node(&nodes, grid, *node, immediate_neighbour) {
                    edges.entry(next_node)
                        .and_modify(|d| *d = usize::max(*d, distance)) // greater distance if multiple paths
                        .or_insert(distance);
                }
            }
            (*node, edges)
        }).collect()
    }

    fn distance_to_next_node(nodes: &HashSet<Point>, grid: &Grid<Cell>, node: Point, immediate_neighbour: Point) -> Option<(Point, usize)> {
        let mut steps = 1;
        let mut previous = node;
        let mut current = immediate_neighbour;
        while !nodes.contains(&current) {
            steps += 1;
            let neighbours: Vec<_> = neighbours(&current, grid).into_iter().filter(|&p| p != previous).collect();
            if neighbours.len() == 0 { return None }
            let next = neighbours[0];
            previous = current;
            current = next; 
        }
        Some((current, steps))
    }
}

fn neighbours(p: &Point, grid: &Grid<Cell>) -> Vec<Point> {
    let cell = grid[p];
    let candidates = match cell {
        Cell::Empty => Direction::all().into_iter().collect(),
        Cell::Wall => vec![],
        Cell::West => vec![Direction::West],
        Cell::East => vec![Direction::East],
        Cell::North => vec![Direction::North],
        Cell::South => vec![Direction::South],
    };
    candidates.into_iter().map(|dir| neighbour_in_direction(p, dir, grid))
        .filter(Option::is_some)
        .map(Option::unwrap)
        .collect()
}

fn neighbour_in_direction(p: &Point, dir: Direction, grid: &Grid<Cell>) -> Option<Point> {
    let neighbour = *p + dir;
    let valid = match grid.get(&neighbour) {
        None | Some(Cell::Wall) => false,
        | Some(Cell::Empty) => true,
        // not allowed to step on a tile that will immediately push us back
        | Some(Cell::North) => dir != Direction::South,
        | Some(Cell::East) =>  dir != Direction::West,
        | Some(Cell::South) => dir != Direction::North,
        | Some(Cell::West) =>  dir != Direction::East,
    };
    if valid { Some(neighbour) } else { None }
}

fn surrounding_walls(p: &Point, grid: &Grid<Cell>) -> usize {
    Direction::all().into_iter()
        .map(|d| *p + d)
        .filter(|p| matches!(grid.get(p), Some(Cell::Wall)))
        .count()
}

#[derive(Eq, PartialEq, Copy, Clone, Debug)]
enum Cell {
    Empty,
    Wall,
    West,
    East,
    North,
    South,
}

impl TryFrom<char> for Cell {
    type Error = Error;

    fn try_from(value: char) -> Result<Self, Self::Error> {
        let value = match value {
            '.' => Cell::Empty,
            '#' => Cell::Wall,
            '<' => Cell::West,
            '>' => Cell::East,
            '^' => Cell::North,
            'v' => Cell::South,
            _ => return Err(Error::new(ErrorKind::InvalidInput, format!("Unknown cell {value}")))
        };
        Ok(value)
    }
}



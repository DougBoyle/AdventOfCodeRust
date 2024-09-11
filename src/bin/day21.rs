use std::{collections::HashSet, fmt::Debug};

use rust_aoc::{grid::Grid, point::Point};

fn main() {
    let (start, grid) = load_grid();

    part1(&start, &grid);
    part2(&start, &grid);
}

fn part1(start: &Point, grid: &Grid<Cell>) {
    println!("Visited {}", reachable_cells(start, grid, 64)); // 3666
}

fn part2(start: &Point, grid: &Grid<Cell>) {
    // Verify that there is a trivial shortest path to enter any grid:
    // - there are straight lines up/down/left/right from the start to the border
    // - having reaching the border, you can freely move along the edge to reach the right column/row to enter from
    // Given that, every border tile can be reached in a number of steps equal to its orthogonal distance from the start,
    // which is the minimum possible number of steps.
    let shortest_paths_trivial = (0..grid.width).all(|x| 
            grid[&Point { x, y: start.y }] == Cell::Garden
            && grid[&Point { x, y: 0 }] == Cell::Garden
            && grid[&Point { x, y: grid.height - 1 }] == Cell::Garden
        ) && (0..grid.height).all(|y| 
            grid[&Point { x: start.x, y }] == Cell::Garden
            && grid[&Point { x: 0, y }] == Cell::Garden
            && grid[&Point { x: grid.width - 1, y }] == Cell::Garden
        );
    if shortest_paths_trivial {
        println!("Trivial path from start to entry of any grid");
    } else {
        panic!("Non-trivial path to entry of each grid, can't simplify problem");
    }

    println!("Grid shape is {} by {}, start={}", grid.width, grid.height, start);
    if grid.width != grid.height {
        panic!("Non-square grid, non-trivial to reason about traversing");
    }

    let grid_length = grid.width;
    assert!(grid_length % 2 == 1);
    let midpoint = grid_length / 2;

    let total_steps = 26501365usize;
    let whole_grids = total_steps / (grid_length as usize); // 202300
    let steps_into_last_grid = total_steps % (grid_length as usize); // 65
    println!("Total steps is {total_steps} = {whole_grids} whole grids ({grid_length}) + {steps_into_last_grid} steps from last grid");
    
    println!("Start from {start}, which should be the center");
    assert!(*start == Point { x: midpoint, y: midpoint });

    // 1. (whole_grids - 2) grid steps out from center, leaves us in the center of various grids, 131 + 131 + 65 steps left
    // 2. either:
    //    a. 131 steps to center of next grid, 66 steps to enter grid above/to side (for the 4 points), 
    //       and then explore that grid up to 130 steps.
    //    b. 131 steps to center of next grid, 132 to corner of diagonal grid, 64 steps to explore "outer diagonal grid"
    //    c. 132 seps to corner, 195 steps to explore "inner diagonal grid"
    // inner/outer diagonal grids differ in whether they're exploring the 'corner' (0,0) (0, 0.5), (0, 0.5),
    // or the larger rest of a grid. Approximately means exploring 1/8 in the corner, or 7/8 up to everything but the last corner,
    // because a diagonal line crosses through (0, 0.5) so we get an alternating pattern of both types of tiles.
    
    let point_counts: Vec<usize> = [
        Point {x: midpoint, y: 0},
        Point {x: midpoint, y: grid_length - 1},
        Point {x: 0, y: midpoint},
        Point {x: grid_length - 1, y: midpoint}
    ].iter().map(|start| reachable_cells(start, grid, grid_length - 1)).collect();

    let (outer_corners_positions, inner_corners_positions) = corner_points(grid).iter()
        .map(|start| reachable_cells_at_each_point(start, grid, vec![midpoint - 1, grid_length + midpoint - 1]))
        .map(|results| (results[0], results[1]))
        // effectively x4 for each direction
        .reduce(|(outer1, inner1), (outer2, inner2)| (outer1 + outer2, inner1 + inner2))
        .unwrap();

    // Also need to know the max number of cells we can fill in an empty grid.
    // Determine this by searching for 2*grid size, starting from the center.
    let even_odd_positions = reachable_cells_at_each_point(start, grid, vec![2*grid_length, 2*grid_length + 1]);
    let even_filled_grid_positions = even_odd_positions[0];
    let odd_filled_grid_poitions = even_odd_positions[1];

    // Number of whole grids: 
    //    .
    //    #
    //   ###                     #
    // .#####. -> 1 center + 4 * ##  = 1 + 4 * (1 + 2)
    //   ###
    //    #
    //    .
    // n whole grids + partial -> (n-1) filled grids out from center + partial border -> 1 + 4*(n-1 + n-2 + n-3 + ... 1)
    // But need to split into even vs odd grids.
    assert!(whole_grids % 2 == 0); // simplify, can assume n (whole_grids) even
    assert!(grid_length % 2 == 1); // grid length is odd, so flips between grids
    assert!(total_steps % 2 == 1); // total steps odd, so center grid is odd, and then 1st grid of each quarter even
    // n whole grids + partial -> n-1 filled grids out from center -> center grid + 4 trianges of length (n-1)
    // per quarter:
    //  total grids 1, 2, 3, .., n-1 = n(n-1)/2
    //  odd grids  2, 4, .., n-2 = 2*(1, 2, .., n/2 - 1) = 2*(n/2)(n/2-1)/2 = (n/2)(n/2 - 1)
    //  even grids 1, 3, .., n-1 = total - odd
    let all_filled_grids_per_quarter = whole_grids*(whole_grids - 1)/2;
    let odd_grids_per_quarter = (whole_grids/2)*(whole_grids/2 - 1);
    let even_grids_per_quarter = all_filled_grids_per_quarter - odd_grids_per_quarter;
    let filled_odd_grids = 1 + 4*odd_grids_per_quarter;
    let filled_even_grids = 4*even_grids_per_quarter;
    
    let filled_count = filled_odd_grids * odd_filled_grid_poitions + filled_even_grids * even_filled_grid_positions;

    println!("Count from {filled_odd_grids} odd + {filled_even_grids} even filled grids: {filled_count}");

    // 1 point in each direction
    let point_count: usize = point_counts.iter().sum();
    println!("Points total: {point_count}");

    // for n whole grids, point cells are n diagonal steps apart => n 'outer' corner grids, n-1 'inner' corner grids
    let outer_corner_total = outer_corners_positions * whole_grids;
    let inner_corner_total = inner_corners_positions * (whole_grids - 1);
    println!("Outer corners total {outer_corner_total}. Inner corner total {inner_corner_total}");

    let overall_total = filled_count + point_count + outer_corner_total + inner_corner_total;
    println!("Overall total: {overall_total}"); // 609298746763952
}

fn corner_points(grid: &Grid<Cell>) -> [Point; 4] {
    let grid_length = grid.width;
    [
        Point {x: 0, y: 0},
        Point {x: 0, y: grid_length - 1},
        Point {x: grid_length - 1, y: 0},
        Point {x: grid_length - 1, y: grid_length - 1}
    ]
}

fn reachable_cells(start: &Point, grid: &Grid<Cell>, steps: i64) -> usize {
    let mut positions = HashSet::new();
    positions.insert(*start);
    for _ in 0..steps {
        positions = positions.iter().flat_map(|p| get_neighbours(p, &grid)).collect();
    }
    positions.len()
}

fn reachable_cells_at_each_point(start: &Point, grid: &Grid<Cell>, steps: Vec<i64>) -> Vec<usize> {
    if !steps.windows(2).all(|values| values[0] < values[1]) { panic!("Out of order steps counts {steps:?}"); }

    let mut positions = HashSet::new();
    positions.insert(*start);
    let mut previous = 0;

    steps.into_iter().map(|end| {
        for _ in previous..end {
            positions = positions.iter().flat_map(|p| get_neighbours(p, &grid)).collect();
        }
        previous = end;
        positions.len()
    }).collect()
}

fn load_grid() -> (Point, Grid<Cell>) {
    let mut start = Point { x: -1, y: -1 };
    let grid = Grid::parse(rust_aoc::read_input(21), |c| c).map(|p, c| {
        match c {
            '.' => Cell::Garden,
            '#' => Cell::Rock,
            'S' => {
                start = p;
                Cell::Garden
            },
            _ => panic!("Unexpected cell {c}"),
        }
    });
    (start, grid)
}

fn get_neighbours(p: &Point, grid: &Grid<Cell>) -> Vec<Point> {
    p.orthogonal_neighbours().into_iter().filter(|p| matches!(grid.get(p), Some(Cell::Garden))).collect()
}

#[derive(Debug, Eq, PartialEq)]
enum Cell { Garden, Rock }

use std::collections::{BTreeSet, HashSet};
use rust_aoc::point::Point;

type Cells = HashSet<Point>;

fn main() {
    let mut cells = Cells::new();
    
    rust_aoc::read_input(11).enumerate().for_each(|(y, line)| 
        parse_row(&line, y.try_into().unwrap(), &mut cells)
    );

    let grid = Grid::new(cells);

    println!("Part 1 distance: {}", Part1::get_total_distance(&grid)); // 9233514
    println!("Part 2 distance: {}", Part2::get_total_distance(&grid)); // 363293506944
}

trait Part {
    fn empty_cell_size() -> usize;

    fn get_total_distance(grid: &Grid) -> usize {
        let empty_size_additional_factor = Self::empty_cell_size() - 1;
        let mut total_dist: usize = 0;
        for_each_pair(&grid.cells, |from, to| {
            total_dist += <u32 as TryInto<usize>>::try_into(from.orthogonal_distance(to)).unwrap();

            let (xmin, xmax) = ordered(from.x, to.x); 
            let (ymin, ymax) = ordered(from.y, to.y);
            total_dist += grid.doubled_cols.range(xmin..xmax).count() * empty_size_additional_factor;
            total_dist += grid.doubled_rows.range(ymin..ymax).count() * empty_size_additional_factor;
        });
        total_dist
    }
}

struct Part1;

impl Part for Part1 {
    fn empty_cell_size() -> usize {
        2
    }
}

struct Part2;

impl Part for Part2 {
    fn empty_cell_size() -> usize {
        1000000
    }
}

fn parse_row(row: &str, y: i32, grid: &mut Cells) {
    let y = y.try_into().unwrap();
    row.chars().enumerate().for_each(|(x, c)| 
        parse(x.try_into().unwrap(), y, c, grid)
    )
}

fn parse(x: i32, y: i32, c: char, grid: &mut Cells) {
    if c == '#' {
        let point = Point { x, y };
        grid.insert(point);
    }
}

struct Grid {
    cells: Cells,
    doubled_rows: BTreeSet<i32>,
    doubled_cols: BTreeSet<i32>,
}

impl Grid {
    /// Figure out doubled rows/cols with 3 passes (+ many updates to the corresponding sets),
    /// first get upper bound on x/y, and then go over cells again removing all the populated ones
    fn new(cells: Cells) -> Grid {
        let x_max = cells.iter().map(|p| p.x).max().unwrap();
        let y_max = cells.iter().map(|p| p.y).max().unwrap();
        let mut doubled_rows: BTreeSet<i32> = (0..y_max).collect();
        let mut doubled_cols: BTreeSet<i32> = (0..x_max).collect();

        for p in cells.iter() {
            doubled_cols.remove(&p.x);
            doubled_rows.remove(&p.y);
        }

        Grid { cells, doubled_rows, doubled_cols }
    }
}

fn for_each_pair<T, F: FnMut(&T, &T)>(values: &HashSet<T>, mut f: F) {
    for (i, first) in values.iter().enumerate() {
        for second in values.iter().skip(i + 1) {
            f(first, second);
        }
    }
}

fn ordered(a: i32, b: i32) -> (i32, i32) {
    if a < b {
        (a, b)
    } else {
        (b, a)
    }
}

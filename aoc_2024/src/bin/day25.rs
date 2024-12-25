use aoc_2024::read_input;
use rust_aoc::grid::Grid;
use rust_aoc::point::Point;

fn main() {
    let Input { locks, keys, height } = parse_input();
    
    println!("{} locks and {} keys, height = {height}", locks.len(), keys.len());

    let combinations: usize = locks.iter()
        .map(|lock| keys.iter().filter(|key| test_lock_and_key(lock, key, height)).count())
        .sum();

    println!("Total {combinations}"); // 3090
}

fn test_lock_and_key(lock: &Pins, key: &Pins, height: i64) -> bool {
    (0..lock.0.len()).all(|i| lock.0[i] + key.0[i] <= height)
}

struct Pins(Vec<i64>);

struct Input {
    locks: Vec<Pins>,
    keys: Vec<Pins>,
    height: i64,
}

fn parse_input() -> Input {
    let mut lines = read_input(25).map(|s| s.trim().to_string()).peekable();
    let mut grids = Vec::new();
    while lines.peek().is_some() {
        let grid = Grid::parse(lines.by_ref().take_while(|s| !s.is_empty()), |c| match c {
            '#' => true,
            '.' => false,
            _ => panic!("Unexpected character '{c}'"),
        });
        grids.push(grid);
    }
    
    let height = grids[0].height;
    let width = grids[0].width;
    assert!(grids.iter().all(|grid| grid.height == height && grid.width == width));

    let mut locks = Vec::new();
    let mut keys = Vec::new();

    for grid in grids {
        type GetPinHeight = Box<dyn Fn(&Grid<bool>, i64) -> i64>;
        let is_lock = grid[&(Point { x: 0, y: 0 })];
        let (get_pin_height, collection): (GetPinHeight, _) = if is_lock {
            (Box::new(get_lock_pin_height), &mut locks)
        } else {
            (Box::new(get_key_pin_height), &mut keys)
        };
        collection.push(Pins((0..width).map(|x| get_pin_height(&grid, x)).collect()));
    }

    let height = height - 2; // ignore the header/footer row that just identifies keys vs locks

    Input { locks, keys, height }
}

fn get_lock_pin_height(lock: &Grid<bool>, pin: i64) -> i64 {
    (0..lock.height).filter(|y| lock[&(Point { x: pin, y: *y })]).last().unwrap()
}

fn get_key_pin_height(key: &Grid<bool>, pin: i64) -> i64 {
    (0..key.height).filter(|y| key[&(Point { x: pin, y: key.height - 1 - *y })]).last().unwrap()
}

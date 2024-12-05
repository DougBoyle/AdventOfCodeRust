use rust_aoc::point::Point;


pub fn read_input(day: u32) -> impl Iterator<Item=String> {
    rust_aoc::read_input(env!("CARGO_PKG_NAME"), day)
}

/// f(point, character) for each cell of the grid, with the first character in the top left being Point { x: 0, y: 0 }
pub fn process_grid<F: FnMut(Point, char)>(day: u32, f: F) {
    rust_aoc::process_grid(env!("CARGO_PKG_NAME"), day, f);
}
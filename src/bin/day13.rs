use std::ops::Range;


fn main() {
    let patterns: Vec<Pattern> = rust_aoc::read_input(13).map(|s| s.chars().collect()).collect::<Vec<_>>()
        .split(|row: &Vec<char>| row.is_empty()).map(|pattern| pattern.to_owned()).collect();

    println!("Part 1");
    Part1::process(&patterns); // 35538
    println!("Part 2");
    Part2::process(&patterns); // 30442
}

type Pattern = Vec<Vec<char>>;

trait Part: Sized {
    fn get_required_errors() -> usize;

    fn process(patterns: &Vec<Pattern>) {
        let total: usize = patterns.iter().map(Self::summarise_symmetry).sum();

        println!("Total: {total}");
    }

    fn summarise_symmetry(pattern: &Pattern) -> usize {
        Horizontal::find_symmetry::<Self>(pattern).unwrap_or_else(|| Vertical::find_symmetry::<Self>(pattern).unwrap() * 100)
    }
}

trait Symmetry {
    fn possible_symmetry_lines(pattern: &Pattern) -> Range<usize>;
    fn get_comparison_range(pattern: &Pattern, symmetry_line: usize) -> (usize, usize);
    fn test_cell_symmetry(pattern: &Pattern, symmetry_line: usize, x: usize, y: usize) -> bool;

    fn find_symmetry<Rules: Part>(pattern: &Pattern) -> Option<usize> {
        Self::possible_symmetry_lines(pattern).filter(|line| Self::test_symmetry::<Rules>(pattern, *line)).next()
    }

    fn test_symmetry<Rules: Part>(pattern: &Pattern, symmetry_line: usize) -> bool {
        let (xrange, yrange) = Self::get_comparison_range(pattern, symmetry_line);
        let errors = enumerate(0..xrange, 0..yrange)
            .filter(|(x, y)| !Self::test_cell_symmetry(pattern, symmetry_line, *x, *y))
            .count();
        errors == Rules::get_required_errors()
    }
}

struct Vertical;

impl Symmetry for Vertical {
    fn possible_symmetry_lines(pattern: &Pattern) -> Range<usize> {
        1..pattern.len()
    }

    fn get_comparison_range(pattern: &Pattern, symmetry_line: usize) -> (usize, usize) {
        let width = pattern[0].len();
        let lines_to_compare = usize::min(symmetry_line, pattern.len() - symmetry_line);
        (width, lines_to_compare)
    }

    fn test_cell_symmetry(pattern: &Pattern, symmetry_line: usize, x: usize, y: usize) -> bool {
        pattern[symmetry_line - y - 1][x] == pattern[symmetry_line + y][x]
    }
}

struct Horizontal;

impl Symmetry for Horizontal {
    fn possible_symmetry_lines(pattern: &Pattern) -> Range<usize> {
        let width = pattern[0].len();
        1..width
    }

    fn get_comparison_range(pattern: &Pattern, symmetry_line: usize) -> (usize, usize) {
        let width = pattern[0].len();
        let lines_to_compare = usize::min(symmetry_line, width - symmetry_line);
        (lines_to_compare, pattern.len())
    }

    fn test_cell_symmetry(pattern: &Pattern, symmetry_line: usize, x: usize, y: usize) -> bool {
        pattern[y][symmetry_line - x - 1] == pattern[y][symmetry_line + x]
    }
}

fn enumerate(x_range: Range<usize>, y_range: Range<usize>) -> impl Iterator<Item=(usize, usize)> {
    x_range.flat_map(move |x| y_range.clone().map(move |y| (x, y)))
}

struct Part1;

impl Part for Part1 {
    fn get_required_errors() -> usize {
        0
    }
}

struct Part2;

impl Part for Part2 {
    fn get_required_errors() -> usize {
        1
    }
}

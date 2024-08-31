
fn main() {
    println!("Part 1");
    Part1::process(); // 1819125966
    println!("Part 2");
    Part2::process(); // 1140
}

trait Part {
    fn predict_impl(values: Vec<i64>, next_delta: i64) -> i64;

    fn process() {
        let total: i64 = rust_aoc::read_input(9).map(|s| parse_line(&s)).map(|values| Self::predict(values)).sum();
        println!("Total: {total}");
    }

    fn predict(values: Vec<i64>) -> i64 {
        if values.iter().all(|v| *v == 0) {
            0
        } else {
            let next_delta = Self::predict(Self::get_deltas(&values));
            Self::predict_impl(values, next_delta)
        }
    }

    fn get_deltas(values: &Vec<i64>) -> Vec<i64> {
        values.windows(2).map(|window| window[1] - window[0]).collect()
    }
}

struct Part1;

impl Part for Part1 {
    fn predict_impl(values: Vec<i64>, next_delta: i64) -> i64 {
        values.last().unwrap() + next_delta
    }
}

struct Part2;

impl Part for Part2 {
    fn predict_impl(values: Vec<i64>, next_delta: i64) -> i64 {
        values[0] - next_delta
    }
}

fn parse_line(s: &str) -> Vec<i64> {
    s.split_ascii_whitespace().map(str::parse).map(Result::unwrap).collect()
}
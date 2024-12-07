fn main() {
    part1();
    part2();
}

fn part1() {
    let safe = read_input()
        .into_iter()
        .filter(is_safe)
        .count();

    println!("Total: {safe}"); // 598
}

fn part2() {
    let safe = read_input()
        .into_iter()
        .filter(damped_is_safe)
        .count();

    println!("Total: {safe}"); // 634
}

fn is_safe(report: &Vec<u32>) -> bool {
    let increasing = report[1] > report[0];
    pair_windows(report).all(|(a, b)| pair_safe(increasing, *a, *b))

}

// Lazy - if a test fails comparing index i and i+1, we try removing each of i-1, i and i+1.
// No need to check the whole rest of the list either side, but also not that expensive to do.
fn damped_is_safe(report: &Vec<u32>) -> bool {
    let increasing = report[1] > report[0];
    if let Some((i, _)) = pair_windows(report).enumerate().filter(|(_, (a, b))| !pair_safe(increasing, **a, **b)).next() {
        (i > 0 && is_safe(&without_element(report, i-1)))
        || is_safe(&without_element(report, i))
        || is_safe(&without_element(report, i+1))
    } else {
        true
    }
}

fn pair_safe(increasing: bool, a: u32, b: u32) -> bool {
    let incr = b > a;
    let diff = a.abs_diff(b);
    (incr == increasing) && diff >= 1 && diff <= 3
}

fn pair_windows<T>(it: &[T]) -> impl Iterator<Item=(&T, &T)> + '_ {
    it.windows(2).map(|slice| (&slice[0], &slice[1]))
}

fn without_element(vec: &Vec<u32>, i: usize) -> Vec<u32> {
    let mut clone = vec.clone();
    clone.remove(i);
    clone
}

fn read_input() -> Vec<Vec<u32>> {
    aoc_2024::read_input(2)
    .map(|line| line.split_ascii_whitespace().map(|n| n.parse().unwrap()).collect())
    .inspect(|report: &Vec<_>| assert!(report.len() >= 2))
    .collect()
}

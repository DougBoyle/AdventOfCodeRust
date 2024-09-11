use std::{collections::HashSet, io::Error, ops::Add, str::FromStr};

use rust_aoc::point3::{Axis, Point3};


fn main() {
    let mut bricks: Vec<Brick> = rust_aoc::read_input(22).map(|s| s.parse().unwrap()).collect();
    // sort by (start) z coordinate, then just need to look at earlier bricks to simulate each brick falling, and never revisit
    bricks.sort_by_key(|p| p.start.z);

    let down = -Axis::Z.as_vec();

    println!("Num bricks: {}", bricks.len());

    let mut falling_steps = 0;
    for i in 0..bricks.len() {
        let mut brick = bricks[i];
        while brick.start.z > 1 {
            let new_brick = brick + down;
            let collides = (0..i).rev().any(|j| bricks[j].intersects(&new_brick));
            if collides {
                break;
            } else {
                brick = new_brick;
                falling_steps += 1;
            }
        }
        bricks[i] = brick;
    }

    println!("Complete! {falling_steps} falling steps");

    // For each brick, work out which ones it is supported by
    let supporters_of_bricks: Vec<HashSet<usize>> = (0..bricks.len()).map(|i| {
        let brick = bricks[i];
        let moved_brick = brick + down;
        (0..i).filter(|&j| i != j && bricks[j].intersects(&moved_brick)).collect()
    }).collect();

    let lone_supporters: HashSet<_> = supporters_of_bricks.iter()
        .filter(|set| set.len() == 1)
        .flat_map(|set| set.iter()).collect();

    println!("Of {} bricks, {} are the only ones supporting some other brick, {} could be removed",
        bricks.len(), lone_supporters.len(), bricks.len() - lone_supporters.len()); // 395

    // Already sorted by height, so just build a map of removing X -> set of other bricks that fall.
    // Working backwards (top down),     


    // Working out how many other bricks fall if one brick is removed resembles the problem of Dominators in graph theory.
    // 1. For each brick i track the dominators of i:
    //      Includes i
    //      Includes the intersection of the dominators of each brick supporting i, i.e. bricks that make every supporter fall
    // 2. No need to build reverse mapping, rather than counting 'how many bricks fall when brick i is removed', that's
    //    the same as counting 'how many different bricks can be removed to make brick j fall'
    // Bricks already tracked in height order, so just needs one forward pass to calculate (1)

    let mut dominators: Vec<HashSet<usize>> = vec![];
    for i in 0..bricks.len() {
        let mut dominators_i = HashSet::new(); 
        dominators_i.insert(i);

        // intersection across all of the sets
        let supporter_dominators: Vec<&HashSet<usize>> = supporters_of_bricks[i].iter().map(|j| &dominators[*j]).collect();
        if !supporter_dominators.is_empty() {
            let first_supporter_dominators = supporter_dominators.iter().next().unwrap();
            for dominator in first_supporter_dominators.iter()
                .filter(|dominator| supporter_dominators.iter().all(|dominators| dominators.contains(dominator))) {
                    dominators_i.insert(*dominator);
            }
        }

        dominators.push(dominators_i);
    }

    println!("Calculated dominators");

    // no need to calculate reverse mapping, just look at number of dominator relations, subtracting each brick dominating itself
    let total_strict_dominators: usize = dominators.iter().map(|dominators| dominators.len() - 1).sum();
    println!("Total chain reaction bricks for each brick: {total_strict_dominators}"); // 64714
}



// Always a straight line i.e. only 1 coordinate differs.
// To simplify handling, always sort points lexicographically so that start < end
#[derive(Copy, Clone, Eq, PartialEq, Debug)]
struct Brick {
    start: Point3, // inclusive at both ends
    end: Point3,
}

impl Brick {
    fn project(&self, axis: Axis) -> (i64, i64) {
        (self.start.project(axis), self.end.project(axis))
    }

    // exploiting the fact all bricks are axis-aligned straight lines,
    // we can just check if their start-end overlap in all 3 dimensions.
    fn intersects(&self, brick: &Brick) -> bool {
        Axis::all().into_iter().all(|axis| {
            let (a_start, a_end) = self.project(axis);
            let (b_start, b_end) = brick.project(axis);
            lines_touch(a_start, a_end, b_start, b_end)
        })
    }
}

impl Add<Point3> for Brick {
    type Output = Brick;

    fn add(self, rhs: Point3) -> Self::Output {
        Brick { start: self.start + rhs, end: self.end + rhs }
    }
}

impl FromStr for Brick {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (first, second) = rust_aoc::split_in_two(s, '~');
        let (first, second) = (first.parse().unwrap(), second.parse().unwrap());
        let (start, end) = if first < second { (first, second) } else { (second, first) };
        Ok(Brick { start, end })
    }
}

// inclusive at endpoints i.e. an L shape counts
fn lines_touch(a_start: i64, a_end: i64, b_start: i64, b_end: i64) -> bool {
    !disjoint_lines(a_start, a_end, b_start, b_end)
}

fn disjoint_lines(a_start: i64, a_end: i64, b_start: i64, b_end: i64) -> bool {
    a_start > b_end || b_start > a_end
}

use std::{io::Error, str::FromStr};

use nalgebra::{Matrix2, Vector2};
use num::abs;
use rust_aoc::{point::Point, point3::Point3};


fn main() {
    let stones: Vec<Stone> = rust_aoc::read_input(24).map(|s| s.parse().unwrap()).collect();
    part1(&stones);
}

fn part1(stones: &Vec<Stone>) {
    let stones: Vec<_> = stones.iter().map(|stone| stone.project()).collect();
    let num_stones = stones.len();
    let total: usize = (0..num_stones).map(|i| {
        let stone1 = &stones[i];
        (0..i).filter(|&j| intersect_in_region(stone1, &stones[j])).count()
    }).sum();
    println!("Total intersections: {total}"); // 16939
}

const MIN: f64 = 200000000000000.0;
const MAX: f64 = 400000000000000.0;
//const MIN: f64 = 7.0;
//const MAX: f64 = 27.0;
const EPS: f64 = 1e-9;

/*
Each line is p + t v

p1 + t1 v1 = p2 + t2 v2
=> t1 v1 - t2 v2 = p2 - p1

[ v1_x -v2_x ] ( t1 ) = ( p2_x - p1_x )
[ v1_y -v2_y ] ( t2 ) = ( p2_y - p1_y )

  ^ (v1 - v2)              ^ (p2 - p1)

( t1 ) = [ v1_x -v2_x ]^-1 ( p2_x - p1_x )
( t2 )   [ v1_y -v2_y ]    ( p2_y - p1_y )

Can then check t1 >= 0 and t2 >= 0, and intersection within required region.

Inverse of [ a b ] is [ d -b ] * 1/(ad-bc)
           [ c d ]    [ -c a ]

If determinant -(v1_x)(v2_y) + (v1_y)(v2_x) = 0,
instead need a separate check for whether lines are coincident or distinct,
and if coincident that they both pass through required region.
*/
fn intersect_in_region(s1: &Stone2, s2: &Stone2) -> bool {
    let p1 = as_vec2(&s1.pos);
    let p2 = as_vec2(&s2.pos);
    let v1 = as_vec2(&s1.v);
    let v2 = as_vec2(&s2.v);

    let velocity_matrix = Matrix2::from_columns(&[v1, -v2]);
    let p = p2 - p1;

    match velocity_matrix.try_inverse() {
        Some(inv_velocity_matrix) => {
            let t = inv_velocity_matrix * p;
            let t1 = t[0];
            let t2 = t[1];
            let intersection = p1 + t1 * v1;
            t1 >= 0.0 && t2 >= 0.0 && within_region(intersection)
        },
        None => parallel_lines_intersect_in_region(p1, p2, v1, v2)
    }
}

// Lines are overlapping rather than distinct, and each pass through the desired region with t >= 0
fn parallel_lines_intersect_in_region(p1: Vector2<f64>, p2: Vector2<f64>, v1: Vector2<f64>, v2: Vector2<f64>) -> bool {
    parallel_lines_are_same(p1, p2, v1) && enters_region(p1, v1) && enters_region(p2, v2)


}

// Check if offset between their starting points is co-linear with (a multiple of) their shared direction vector,
// checking for 0s when comparing ratios.
fn parallel_lines_are_same(p1: Vector2<f64>, p2: Vector2<f64>, v: Vector2<f64>) -> bool {
    let p = p2 - p1;
    if !is_zero(v.x) && !is_zero(v.y) {
        is_zero((p.x / v.x) - (p.y / v.y))
    } else if !is_zero(v.x) && is_zero(v.y) {
        is_zero(p.y)
    } else if is_zero(v.x) && !is_zero(v.y) {
        is_zero(p.x)
    } else { // both lines are simply a point in 2D, check they are the same point
        is_zero(p.x) && is_zero(p.y)
    }
}

/*
  MIN <= x + t*v_x <= MAX
  MIN <= y + t*v_y <= MAX
  v_x = 0 => just check x in range.
  v_y = 0 => just check y in range.
  else get bounds for t of (MIN-x)/v_x, (MAX-x)/v_x
  and (MIN-y)/v_y, (MAX-y)/v_y
  - Enters box as long as both ranges have some overlap, and t positive for at least some of it.
 */
fn enters_region(p: Vector2<f64>, v: Vector2<f64>) -> bool {
    // otherwise, can assume point sits outside region for later checks below
    if within_region(p) { return true; }
    
    if !is_zero(v.x) && !is_zero(v.y) {
        let tx0 = (MIN-p.x)/v.x;
        let tx1 = (MAX-p.x)/v.x;
        let (tx_min, tx_max) = if tx0 < tx1 { (tx0, tx1) } else { (tx1, tx0) };

        let ty0 = (MIN-p.y)/v.y;
        let ty1 = (MAX-p.y)/v.y;
        let (ty_min, ty_max) = if ty0 < ty1 { (ty0, ty1) } else { (ty1, ty0) };

        let (tmin, tmax) = (f64::max(tx_min, ty_min), f64::min(tx_max, ty_max));
        tmin <= tmax && tmax >= 0.0
    } else if !is_zero(v.x) && is_zero(v.y) {
        within_range(p.y) && (if p.x < MIN { v.x > 0.0 } else { v.x < 0.0 })
    } else if is_zero(v.x) && !is_zero(v.y) {
        within_range(p.x) && (if p.y < MIN { v.y > 0.0 } else { v.y < 0.0 })
    } else {
        false
    }
}

fn within_region(p: Vector2<f64>) -> bool {
    within_range(p.x) && within_range(p.y)
}

fn within_range(f: f64) -> bool {
    f >= MIN && f <= MAX
}

struct Stone2 {
    pos: Point,
    v: Point,
}

struct Stone {
    pos: Point3,
    v: Point3,
}

impl Stone {
    fn project(&self) -> Stone2 {
        let (pos, v) = (project_point(self.pos), project_point(self.v));
        Stone2 { pos, v } 
    }
}

impl FromStr for Stone {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (pos, v) = rust_aoc::split_in_two(s, '@');
        let (pos, v) = (pos.parse().unwrap(), v.parse().unwrap());
        Ok(Stone { pos, v})
    }
}

fn project_point(p: Point3) -> Point {
    Point { x: p.x, y: p.y }
}

fn as_vec2(p: &Point) -> Vector2<f64> {
    Vector2::new(p.x as f64, p.y as f64)
}

fn is_zero(f: f64) -> bool {
    abs(f) < EPS
}
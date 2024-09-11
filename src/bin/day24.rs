use std::{ io::Error, str::FromStr};

use nalgebra::{Matrix2, Vector2, Vector3};
use num::abs;
use rust_aoc::{point::Point, point3::Point3};


fn main() {
    let stones: Vec<Stone> = rust_aoc::read_input(24).map(|s| s.parse().unwrap()).collect();
    part1(&stones);
    part2(&stones);
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

fn part2(stones: &Vec<Stone>) {
    println!("Part 2");

    let num_stones = stones.len();

    let parallel_pairs: usize = (0..num_stones).map(|i| {
        let stone1 = &stones[i];
        let v1 = as_vec3(&stone1.v);
        (0..i).filter(|&j| {
            let stone2 = &stones[j];
            let v2 = as_vec3(&stone2.v);
            is_parallel(v1, v2)
        }).count()
    }).sum();
    
    println!("Number of parallel pairs: {parallel_pairs}");
    assert_eq!(0, parallel_pairs, "If lines were parallel, we could simplify the problem substantially");

    // Complex calculation, so hard to guard against numeric precision issues.
    // Instead, we just try various triplets of stones until we find a combination that appears to have suitably small error.
    // (we could exploit knowning the intersections only happen at integer times, to do some error correction part way through,
    //  but the exercise doesn't actually say this is guaranteed)
    let (attempt, origin) = (0..num_stones/3)
        .map(|i| part2_find_origin(&stones[3*i], &stones[3*i + 1], &stones[3*i + 2]))
        .enumerate()
        .filter(|(_, p)| is_integer_point(p))
        .next()
        .expect("Didn't find an integer point solution, suggests need to do something about arithmetic errors");
    let attempt = attempt + 1;
    let origin = Vector3::new(origin.x.round(), origin.y.round(), origin.z.round());
    println!("Found origin on attempt {attempt}: {origin}");
    println!("Sum of coordinates: {}", origin.x + origin.y + origin.z); // 931193307668256
}

/*
 Let p_0 and v_0 be position/velocity of the stone, and each intersection at time t_i position q_i.
 p_0, v_0 and t_i values all unknown, need to phrase question in terms of things we do know.

 !! Shift everything to be vs the first stone's reference point i.e. p_i = p_i - p_1, v_i = v_i - v_1.
 Time remains as-is, still t_1, t_2, t_3, etc. (the first stone from the input, not the unknown new stone!)

 Now q_1 is just the origin 0, and we know the intersections form a straight line,
 so (q_2 - q_1) parallel to (q_3 - q_1) => q_2 parallel to q_3.
 Need to frame this as an equation, so take cross product: q_2 x q_3 = 0
 Expanding:
 (p_2 + t_2 v_2) x (p_3 + t_3 v_3) = 0
 p_2 x p_3 + t_2 v_2 x p_3 + t_3 p_2 x v_3 + t_2 t_3 v_2 x v_3 = 0

 3D vectors, so at this point have 3 equations in terms of 2 variables, and could evaluate and try to solve
 each dimension directly. However, can also remove the (t_2 * t_3) term to make it much simpler to solve.

 !! Want to get back to scalar values for t_2 and t_3, so want to take a dot product. Also, for any a and b,
    a x b is perpendicular to a and b, hence (a x b) * a = (a x b) * b = 0

 Dot product with v_2:
 (p_2 x p_3) * v_2 + 0 + t_3 (p_2 x v_3) * v_2 + 0 = 0
 => t_3 = -( (p_2 x p_3) * v_2 ) / ( (p_2 x v_3) * v_2 )

 with v_3:
 (p_2 x p_3) * v_3 + t_2 (v_2 x p_3) * v_3 + 0 + 0 = 0
 t_2 = -( (p_2 x p_3) * v_3 ) / ( (v_2 x p_3) * v_3 )

 With t_2 and t_3, can now go back to real coordinates and calulate the intersection points, giving the actual line.
 */
fn part2_find_origin(s1: &Stone, s2: &Stone, s3: &Stone) -> Vector3<f64> {
    let (p1, v1) = s1.as_vectors();
    let (p2, v2) = s2.as_vectors();
    let (p3, v3) = s3.as_vectors();

    let (t2, t3) = {
        // reframe everyting in terms of the first stone
        let (p2, p3) = (p2 - p1, p3 - p1);
        let (v2, v3) =  (v2 - v1, v3 - v1);
        let t2 = -(p2.cross(&p3)).dot(&v3) / (v2.cross(&p3)).dot(&v3);
        let t3 = -(p2.cross(&p3)).dot(&v2) / (p2.cross(&v3)).dot(&v2);
        (t2, t3)
    };

    // Due to the size of the values involved (coordinates in the trillions),
    // can get numeric stability issues. Rather than trying several combinations of rocks,
    // could check times are "close" to integer values and round here before further
    // arithmetic, but the task doesn't actually say intersections are at integer times. 

    // assert!(abs(t2) % 1.0 < 1e-2);
    // assert!(abs(t3) % 1.0 < 1e-2);
    // let (t2, t3) = (t2.round(), t3.round());
    // println!("Adjusted intersection times: {t2} and {t3}");

    let q2 = p2 + t2 * v2;
    let q3 = p3 + t3 * v3;

    let v = (q3 - q2)/(t3 - t2);
    let p = q3 - t3*v;
    p
}

fn is_integer_point(p: &Vector3<f64>) -> bool {
    is_zero(p.x - p.x.round()) && is_zero(p.y - p.y.round()) && is_zero(p.z - p.z.round())
}

const MIN: f64 = 200000000000000.0;
const MAX: f64 = 400000000000000.0;
//const MIN: f64 = 7.0;
//const MAX: f64 = 27.0;
const EPS: f64 = 1e-9;

fn is_parallel(v1: Vector3<f64>, v2: Vector3<f64>) -> bool {
    let cross = v1.cross(&v2);
    is_zero(cross.x) && is_zero(cross.y) && is_zero(cross.z)
}

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

#[derive(Debug, Copy, Clone)]
struct Stone {
    pos: Point3,
    v: Point3,
}

impl Stone {
    fn project(&self) -> Stone2 {
        let (pos, v) = (project_point(self.pos), project_point(self.v));
        Stone2 { pos, v } 
    }

    fn as_vectors(&self) -> (Vector3<f64>, Vector3<f64>) {
        (as_vec3(&self.pos), as_vec3(&self.v))
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

fn as_vec3(p: &Point3) -> Vector3<f64> {
    Vector3::new(p.x as f64, p.y as f64, p.z as f64)
}

fn is_zero(f: f64) -> bool {
    abs(f) < EPS
}
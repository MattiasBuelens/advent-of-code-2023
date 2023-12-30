use std::ops::Mul;

use aoc_runner_derive::{aoc, aoc_generator};
use num_traits::Zero;
use z3::ast::{Ast, Int};
use z3::*;

type Vector3D = crate::util::Vector3D<i64>;
type FloatVector2D = crate::util::Vector2D<f64>;
type FloatVector3D = crate::util::Vector3D<f64>;

#[derive(Debug, Copy, Clone)]
struct Hailstone {
    pos: Vector3D,
    vel: Vector3D,
}

#[aoc_generator(day24)]
fn parse(input: &str) -> Vec<Hailstone> {
    input
        .lines()
        .map(|s| {
            let (pos, vel) = s.split_once(" @ ").unwrap();
            let pos = Vector3D::from_iter(pos.split(", ").map(|x| x.trim().parse().unwrap()));
            let vel = Vector3D::from_iter(vel.split(", ").map(|x| x.trim().parse().unwrap()));
            Hailstone { pos, vel }
        })
        .collect()
}

impl Hailstone {
    fn intersect_2d(&self, other: &Self) -> Option<FloatVector2D> {
        self.intersect(other, true).map(|pos| pos.into_2d())
    }

    fn intersect_3d(&self, other: &Self) -> Option<FloatVector3D> {
        self.intersect(other, false)
    }

    fn intersect(&self, other: &Self, skip_z: bool) -> Option<FloatVector3D> {
        // Parametric equations for X coordinates of line 1 and 2:
        // x1 = pos1.x + vel1.x * t1
        // x2 = pos2.x + vel2.x * t2
        // X coordinates must be equal for some values of t1 and t2:
        // pos1.x + vel1.x * t1 = pos2.x + vel2.x * t2
        // t1 = ((pos2.x - pos1.x) + (vel2.x * t2)) / vel1.x  (1)
        // Same for Y and Z:
        // t1 = ((pos2.y - pos1.y) + (vel2.y * t2)) / vel1.y  (2)
        // t1 = ((pos2.z - pos1.z) + (vel2.z * t2)) / vel1.z  (3)
        // Setting (1) and (2) equal:
        // t2 = ((vel1.x * (pos1.y - pos2.y) - vel1.y * (pos1.x - pos2.x)) / (vel1.x * vel2.y - vel2.x * vel1.y)
        let determinant_vel = self.vel.x() * other.vel.y() - other.vel.x() * self.vel.y();
        if determinant_vel == 0 {
            // Parallel
            return None;
        }
        let pos1 = self.pos.to_f64();
        let pos2 = other.pos.to_f64();
        let vel1 = self.vel.to_f64();
        let vel2 = other.vel.to_f64();
        let t2 = (vel1.x() * (pos1.y() - pos2.y()) - vel1.y() * (pos1.x() - pos2.x()))
            / (determinant_vel as f64);
        // Substitute in (1) to find t1:
        let t1 = ((pos2.x() - pos1.x()) + (vel2.x() * t2)) / vel1.x();
        let mut intersect1 = pos1 + vel1 * t1;
        let mut intersect2 = pos2 + vel2 * t2;
        // Ignore Z for part 1
        if skip_z {
            *intersect1.z_mut() = 0.0;
            *intersect2.z_mut() = 0.0;
        }
        // Check if it also works for Z
        if !intersect1.relative_eq(&intersect2, 1e-12) {
            return None;
        }
        Some(intersect1)
    }

    fn time_for(&self, pos: FloatVector2D) -> f64 {
        // x = start_pos.x + vel.x * t
        // t = (x - start_pos.x) / vel.x
        (pos.x() - (self.pos.x()) as f64) / (self.vel.x() as f64)
    }
}

fn count_intersections(hailstones: &[Hailstone], min: f64, max: f64) -> usize {
    hailstones
        .iter()
        .enumerate()
        .flat_map(move |(i, left)| {
            hailstones
                .iter()
                .skip(i + 1)
                .map(move |right| (left, right))
        })
        .filter_map(|(left, right)| {
            let intersection = left.intersect_2d(right)?;
            // Must be within target area
            if (min..=max).contains(&intersection.x()) && (min..=max).contains(&intersection.y()) {
                // Must not be in the past
                let left_time = left.time_for(intersection);
                let right_time = right.time_for(intersection);
                if left_time > 0.0 && right_time > 0.0 {
                    return Some(intersection);
                }
            }
            None
        })
        .count()
}

#[aoc(day24, part1)]
fn part1(hailstones: &[Hailstone]) -> usize {
    count_intersections(hailstones, 200_000_000_000_000.0, 400_000_000_000_000.0)
}

#[aoc(day24, part2)]
fn part2(hailstones: &[Hailstone]) -> i64 {
    let ctx = Context::new(&Config::new());
    // The rock's position
    let pos_x = Int::new_const(&ctx, "pos_x");
    let pos_y = Int::new_const(&ctx, "pos_y");
    let pos_z = Int::new_const(&ctx, "pos_z");
    // The rock's velocity
    let vel_x = Int::new_const(&ctx, "vel_x");
    let vel_y = Int::new_const(&ctx, "vel_y");
    let vel_z = Int::new_const(&ctx, "vel_z");
    // The collision time with each hailstone
    let collision_times = hailstones
        .iter()
        .enumerate()
        .map(|(i, _)| Int::new_const(&ctx, format!("time_{i}")))
        .collect::<Vec<_>>();
    // Set up equations
    let solver = Solver::new(&ctx);
    for (i, hailstone) in hailstones.iter().enumerate() {
        let collision_time = &collision_times[i];
        // Collide in all three coordinates
        solver.assert(
            &(collision_time * &vel_x + &pos_x)
                ._eq(&(collision_time * hailstone.vel.x() + hailstone.pos.x())),
        );
        solver.assert(
            &(collision_time * &vel_y + &pos_y)
                ._eq(&(collision_time * hailstone.vel.y() + hailstone.pos.y())),
        );
        solver.assert(
            &(collision_time * &vel_z + &pos_z)
                ._eq(&(collision_time * hailstone.vel.z() + hailstone.pos.z())),
        );
    }
    assert_eq!(solver.check(), SatResult::Sat);
    let model = solver.get_model().unwrap();
    let pos_x = model.eval(&pos_x, true).unwrap().as_i64().unwrap();
    let pos_y = model.eval(&pos_y, true).unwrap().as_i64().unwrap();
    let pos_z = model.eval(&pos_z, true).unwrap().as_i64().unwrap();
    dbg!(pos_x, pos_y, pos_z);
    pos_x + pos_y + pos_z
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "19, 13, 30 @ -2,  1, -2
18, 19, 22 @ -1, -1, -2
20, 25, 34 @ -2, -2, -4
12, 31, 28 @ -1, -2, -1
20, 19, 15 @  1, -5, -3";

    #[test]
    fn part1_example() {
        assert_eq!(count_intersections(&parse(INPUT), 7.0, 27.0), 2);
    }

    #[test]
    fn part2_example() {
        assert_eq!(part2(&parse(INPUT)), 47);
    }
}

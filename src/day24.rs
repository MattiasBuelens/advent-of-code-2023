use aoc_runner_derive::{aoc, aoc_generator};
use num_traits::Zero;

use crate::util::Num;

type Rational = crate::util::Rational<i64>;
type Vector3D = crate::util::Vector3D<i64>;
type RationalVector2D = crate::util::Vector2D<Rational>;

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
    fn intersect_2d(&self, other: &Self) -> Option<RationalVector2D> {
        // m = vy / vx
        let m1 = Rational::new(self.vel.y(), self.vel.x());
        let m2 = Rational::new(other.vel.y(), other.vel.x());
        if m1 == m2 {
            // Parallel
            return None;
        }
        // y = m * x + b => b = y - m * x
        let b1 = Rational::from(self.pos.y()) - m1 * self.pos.x();
        let b2 = Rational::from(other.pos.y()) - m2 * other.pos.x();
        // m1 * x + b1 = m2 * x + b2
        // (m1 - m2) * x = b2 - b1
        // x = (b2 - b1) / (m1 - m2)
        let intersect_x = (b2 - b1) / (m1 - m2);
        let intersect_y = intersect_x * m1 + b1;
        Some(RationalVector2D::new(intersect_x, intersect_y))
    }

    fn time_for(&self, pos: RationalVector2D) -> Rational {
        // x = start_pos.x + vel.x * t
        // t = (x - start_pos.x) / vel.x
        (pos.x() - Rational::from(self.pos.x())) / Rational::from(self.vel.x())
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
            let x = intersection.x().into_decimal();
            let y = intersection.y().into_decimal();
            if (min..=max).contains(&x) && (min..=max).contains(&y) {
                // Must not be in the past
                let left_time = left.time_for(intersection);
                let right_time = right.time_for(intersection);
                if left_time > Rational::zero() && right_time > Rational::zero() {
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
fn part2(hailstones: &[Hailstone]) -> usize {
    todo!()
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
        assert_eq!(part2(&parse(INPUT)), 0);
    }
}

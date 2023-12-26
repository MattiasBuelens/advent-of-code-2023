use aoc_runner_derive::{aoc, aoc_generator};

type Vector3D = crate::util::Vector3D<i64>;
type FloatVector2D = crate::util::Vector2D<f64>;

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
        // m = vy / vx
        let m1 = (self.vel.y() as f64) / (self.vel.x() as f64);
        let m2 = (other.vel.y() as f64) / (other.vel.x() as f64);
        if m1 == m2 {
            // Parallel
            return None;
        }
        // y = m * x + b => b = y - m * x
        let b1 = (self.pos.y() as f64) - m1 * (self.pos.x() as f64);
        let b2 = (other.pos.y() as f64) - m2 * (other.pos.x() as f64);
        // m1 * x + b1 = m2 * x + b2
        // (m1 - m2) * x = b2 - b1
        // x = (b2 - b1) / (m1 - m2)
        let intersect_x = (b2 - b1) / (m1 - m2);
        let intersect_y = intersect_x * m1 + b1;
        Some(FloatVector2D::new(intersect_x, intersect_y))
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

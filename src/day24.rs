use aoc_runner_derive::{aoc, aoc_generator};

type Vector3D = crate::util::Vector3D<i64>;

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

fn count_intersections(hailstones: &[Hailstone], min: f64, max: f64) -> usize {
    dbg!(hailstones);
    todo!()
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

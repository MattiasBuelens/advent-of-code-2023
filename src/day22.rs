use std::collections::HashSet;

use aoc_runner_derive::{aoc, aoc_generator};

use crate::util::Vector3D;

#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
struct Brick {
    start: Vector3D,
    end: Vector3D,
}

#[aoc_generator(day22)]
fn parse(input: &str) -> Vec<Brick> {
    input
        .lines()
        .map(|line| {
            let (start, end) = line.split_once('~').unwrap();
            let start = Vector3D::from_iter(start.split(',').map(|x| x.parse().unwrap()));
            let end = Vector3D::from_iter(end.split(',').map(|x| x.parse().unwrap()));
            assert!(start <= end);
            Brick { start, end }
        })
        .collect()
}

impl Brick {
    fn cubes(&self) -> Vec<Vector3D> {
        let diff = self.end - self.start;
        let length = diff.manhattan_distance() + 1;
        let step = if diff.x() != 0 {
            Vector3D::new(1, 0, 0)
        } else if diff.y() != 0 {
            Vector3D::new(0, 1, 0)
        } else {
            Vector3D::new(0, 0, 1)
        };
        let mut result = Vec::with_capacity(length as usize);
        let mut pos = self.start;
        while pos != self.end {
            result.push(pos);
            pos += step;
        }
        result.push(self.end);
        result
    }

    fn can_drop(&self) -> bool {
        self.start.z() > 1
    }

    fn drop(&self) -> Self {
        debug_assert!(self.can_drop());
        let mut result = *self;
        *result.start.z_mut() -= 1;
        *result.end.z_mut() -= 1;
        result
    }
}

fn try_drop_brick(brick: &Brick, dropped_cubes: &HashSet<Vector3D>) -> Option<Brick> {
    if !brick.can_drop() {
        return None;
    }
    let next_brick = brick.drop();
    if next_brick
        .cubes()
        .iter()
        .any(|cube| dropped_cubes.contains(&cube))
    {
        return None;
    }
    Some(next_brick)
}

#[aoc(day22, part1)]
fn part1(input: &[Brick]) -> usize {
    // Drop bricks, from lowest to highest
    let mut sorted_bricks = input.to_vec();
    sorted_bricks.sort_by_key(|brick| brick.start.z());
    let mut dropped_bricks = vec![];
    let mut dropped_cubes = HashSet::<Vector3D>::new();
    for mut brick in sorted_bricks {
        while let Some(next_brick) = try_drop_brick(&brick, &dropped_cubes) {
            brick = next_brick;
        }
        dropped_bricks.push(brick);
        dropped_cubes.extend(brick.cubes());
    }
    // Check which bricks can be disintegrated
    let mut num_disintegrate = 0;
    for brick in &dropped_bricks {
        let mut can_disintegrate = true;
        for other_brick in &dropped_bricks {
            if other_brick == brick || !other_brick.can_drop() {
                continue;
            }
            // Disintegrate the brick
            let mut new_cubes = dropped_cubes.clone();
            for cube in brick.cubes() {
                new_cubes.remove(&cube);
            }
            // Remove the other brick too, so it doesn't collide with itself
            for cube in other_brick.cubes() {
                new_cubes.remove(&cube);
            }
            // Check if the other brick would fall
            if try_drop_brick(other_brick, &new_cubes).is_some() {
                can_disintegrate = false;
                break;
            }
        }
        if can_disintegrate {
            num_disintegrate += 1;
        }
    }
    num_disintegrate
}

#[aoc(day22, part2)]
fn part2(input: &[Brick]) -> usize {
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "1,0,1~1,2,1
0,0,2~2,0,2
0,2,3~2,2,3
0,0,4~0,2,4
2,0,5~2,2,5
0,1,6~2,1,6
1,1,8~1,1,9";

    #[test]
    fn part1_example() {
        assert_eq!(part1(&parse(INPUT)), 5);
    }

    #[test]
    fn part2_example() {
        assert_eq!(part2(&parse(INPUT)), 0);
    }
}

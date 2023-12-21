use std::collections::HashSet;

use aoc_runner_derive::{aoc, aoc_generator};

use crate::util::Vector2D;

struct Garden {
    width: i32,
    height: i32,
    rocks: HashSet<Vector2D>,
    start: Vector2D,
}

#[aoc_generator(day21)]
fn parse(input: &str) -> Garden {
    let height = input.lines().count() as i32;
    let width = input.lines().next().unwrap().len() as i32;
    let mut rocks = HashSet::new();
    let mut start = None;
    for (y, line) in input.lines().enumerate() {
        for (x, c) in line.chars().enumerate() {
            let pos = Vector2D::new(x as i32, y as i32);
            match c {
                '.' => {}
                '#' => {
                    rocks.insert(pos);
                }
                'S' => {
                    start = Some(pos);
                }
                _ => panic!("invalid tile: {c}"),
            };
        }
    }
    let start = start.unwrap();
    Garden {
        width,
        height,
        rocks,
        start,
    }
}

fn count_reachable(garden: &Garden, steps: usize) -> usize {
    let mut positions = HashSet::new();
    positions.insert(garden.start);
    for _ in 0..steps {
        let mut new_positions = HashSet::with_capacity(positions.capacity());
        for pos in positions {
            for next_pos in pos.neighbours() {
                if (0..garden.width).contains(&next_pos.x())
                    && (0..garden.height).contains(&next_pos.y())
                    && !garden.rocks.contains(&next_pos)
                {
                    new_positions.insert(next_pos);
                }
            }
        }
        positions = new_positions;
    }
    positions.len()
}

#[aoc(day21, part1)]
fn part1(garden: &Garden) -> usize {
    count_reachable(garden, 64)
}

fn count_wrapping_reachable(garden: &Garden, steps: usize) -> usize {
    todo!()
}

#[aoc(day21, part2)]
fn part2(garden: &Garden) -> usize {
    count_wrapping_reachable(garden, 26_501_365)
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "...........
.....###.#.
.###.##..#.
..#.#...#..
....#.#....
.##..S####.
.##..#...#.
.......##..
.##.#.####.
.##..##.##.
...........";

    #[test]
    fn part1_example() {
        assert_eq!(count_reachable(&parse(INPUT), 6), 16);
    }

    #[test]
    fn part2_example() {
        let garden = parse(INPUT);
        assert_eq!(count_wrapping_reachable(&garden, 6), 16);
        assert_eq!(count_wrapping_reachable(&garden, 10), 50);
        assert_eq!(count_wrapping_reachable(&garden, 50), 1594);
        assert_eq!(count_wrapping_reachable(&garden, 100), 6536);
        assert_eq!(count_wrapping_reachable(&garden, 500), 167004);
        assert_eq!(count_wrapping_reachable(&garden, 1000), 668697);
        assert_eq!(count_wrapping_reachable(&garden, 5000), 16733044);
    }
}

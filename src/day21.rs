use std::collections::HashSet;
use std::hash::{Hash, Hasher};

use aoc_runner_derive::{aoc, aoc_generator};
use pathfinding::prelude::bfs_reach;

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

#[derive(Debug, Copy, Clone)]
struct State {
    pos: Vector2D,
    steps: usize,
}

impl PartialEq<Self> for State {
    fn eq(&self, other: &Self) -> bool {
        self.pos == other.pos
    }
}

impl Eq for State {}

impl Hash for State {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.pos.hash(state)
    }
}

fn count_wrapping_reachable(garden: &Garden, steps: usize) -> usize {
    let start = State {
        pos: garden.start,
        steps: 0,
    };
    let reachable = bfs_reach(start, |&state| {
        state.pos.neighbours().filter_map(move |next_pos| {
            let wrapped_pos = Vector2D::new(
                next_pos.x().rem_euclid(garden.width),
                next_pos.y().rem_euclid(garden.height),
            );
            if !garden.rocks.contains(&wrapped_pos) {
                let next_state = State {
                    pos: next_pos,
                    steps: state.steps + 1,
                };
                Some(next_state)
            } else {
                None
            }
        })
    });
    let mut reachable_plots = HashSet::<Vector2D>::new();
    let parity = steps % 2;
    for state in reachable {
        if state.steps > steps {
            break;
        }
        // Must have same parity
        if state.steps % 2 == parity {
            reachable_plots.insert(state.pos);
        }
    }
    reachable_plots.len()
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
        let garden = parse(INPUT);
        assert_eq!(count_reachable(&garden, 0), 1);
        assert_eq!(count_reachable(&garden, 1), 2);
        assert_eq!(count_reachable(&garden, 2), 4);
        assert_eq!(count_reachable(&garden, 3), 6);
        assert_eq!(count_reachable(&garden, 6), 16);
    }

    #[test]
    fn part2_example() {
        let garden = parse(INPUT);
        assert_eq!(count_wrapping_reachable(&garden, 0), 1);
        assert_eq!(count_wrapping_reachable(&garden, 1), 2);
        assert_eq!(count_wrapping_reachable(&garden, 2), 4);
        assert_eq!(count_wrapping_reachable(&garden, 3), 6);
        assert_eq!(count_wrapping_reachable(&garden, 6), 16);
        assert_eq!(count_wrapping_reachable(&garden, 10), 50);
        assert_eq!(count_wrapping_reachable(&garden, 50), 1594);
        assert_eq!(count_wrapping_reachable(&garden, 100), 6536);
        assert_eq!(count_wrapping_reachable(&garden, 500), 167004);
        // assert_eq!(count_wrapping_reachable(&garden, 1000), 668697);
        // assert_eq!(count_wrapping_reachable(&garden, 5000), 16733044);
    }
}

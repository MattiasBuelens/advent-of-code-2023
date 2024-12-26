use std::collections::{HashMap, HashSet};

use aoc_runner_derive::{aoc, aoc_generator};
use derivative::Derivative;
use pathfinding::prelude::bfs_reach;

use crate::util::{Direction, Vector2D};

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

#[derive(Debug, Default, Copy, Clone, Eq, Derivative)]
#[derivative(PartialEq, Hash)]
struct State {
    local_pos: Vector2D,
    grid_pos: Vector2D,
    #[derivative(PartialEq = "ignore")]
    #[derivative(Hash = "ignore")]
    steps: usize,
}

fn find_local_reachable(garden: &Garden, start: State) -> impl Iterator<Item = State> + '_ {
    bfs_reach(start, move |&state| {
        state.local_pos.neighbours().filter_map(move |next_pos| {
            if (0..garden.width).contains(&next_pos.x())
                && (0..garden.height).contains(&next_pos.y())
                && !garden.rocks.contains(&next_pos)
            {
                Some(State {
                    local_pos: next_pos,
                    grid_pos: state.grid_pos,
                    steps: state.steps + 1,
                })
            } else {
                None
            }
        })
    })
}

fn count_reachable(garden: &Garden, start: Vector2D, steps: usize) -> usize {
    find_local_reachable(
        garden,
        State {
            local_pos: start,
            ..Default::default()
        },
    )
    .take_while(|state| state.steps <= steps)
    .filter(|state| {
        // Must have same parity
        &state.steps % 2 == steps % 2
    })
    .count()
}

#[aoc(day21, part1)]
fn part1(garden: &Garden) -> usize {
    count_reachable(garden, garden.start,64)
}

struct ReachableFromEdge {
    // The start position, on the edge
    start: Vector2D,
    // The closest position on all 4 other edges, with their associated cost
    edges: [(Vector2D, usize); Direction::COUNT],
    // The total number of plots reachable with an odd number of steps
    odd: usize,
    // The total number of plots reachable with an even number of steps
    even: usize,
}

impl ReachableFromEdge {
    fn closest_edge(&self, dir: Direction) -> (Vector2D, usize) {
        self.edges[dir.index() as usize]
    }
}

fn find_reachable_from_edge(garden: &Garden, start: Vector2D) -> ReachableFromEdge {
    let mut edges = HashMap::<Direction, (Vector2D, usize)>::new();
    let mut odd = 0;
    let mut even = 0;
    let reachable = find_local_reachable(
        garden,
        State {
            local_pos: garden.start,
            ..Default::default()
        },
    );
    for state in reachable {
        // Update totals
        if state.steps % 2 == 0 {
            even += 1;
        } else {
            odd += 1;
        }
        // Update the closest position along left or right edge
        if state.local_pos.x() == 0 || state.local_pos.x() == garden.width - 1 {
            let dir = if state.local_pos.x() == 0 {
                Direction::W
            } else {
                Direction::E
            };
            let existing = edges.entry(dir).or_insert((state.local_pos, state.steps));
            if state.steps < existing.1 {
                *existing = (state.local_pos, state.steps);
            }
        }
        // Update the closest position along top or bottom edge
        if state.local_pos.y() == 0 || state.local_pos.y() == garden.height - 1 {
            let dir = if state.local_pos.y() == 0 {
                Direction::N
            } else {
                Direction::S
            };
            let existing = edges.entry(dir).or_insert((state.local_pos, state.steps));
            if state.steps < existing.1 {
                *existing = (state.local_pos, state.steps);
            }
        }
    }
    let edges = Direction::all()
        .into_iter()
        .map(|dir| *edges.get(&dir).unwrap())
        .collect::<Vec<_>>()
        .try_into()
        .unwrap();
    ReachableFromEdge {
        start,
        edges,
        odd,
        even,
    }
}

impl Garden {    
    fn is_edge(&self, pos: Vector2D) -> bool {
        pos.x() == 0 || pos.x() == self.width - 1 || pos.y() == 0 || pos.y() == self.height - 1
    }
}

fn find_wrapping_reachable(garden: &Garden, start: State, steps: usize) -> impl Iterator<Item = State> + '_ {}

fn count_wrapping_reachable(garden: &Garden, steps: usize) -> usize {
    // Pre-compute the shortest path from any edge plot to any other plot within the local grid.
    let mut reachable_from_edge = HashMap::<Vector2D, ReachableFromEdge>::new();
    let start = State {
        local_pos: garden.start,
        ..Default::default()
    };
    // Explore the local grid
    let mut count = 0;
    let mut grids_seen = HashSet::<Vector2D>::new();
    let reachable = find_local_reachable(garden, start);
        
    let mut reachable_plots = HashSet::<Vector2D>::new();
    let parity = steps % 2;
    for state in reachable {
        if state.steps > steps {
            break;
        }
        // Must have same parity
        if state.steps % 2 == parity {
            reachable_plots.insert(state.local_pos);
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
        assert_eq!(count_reachable(&garden, garden.start, 0), 1);
        assert_eq!(count_reachable(&garden, garden.start, 1), 2);
        assert_eq!(count_reachable(&garden, garden.start, 2), 4);
        assert_eq!(count_reachable(&garden, garden.start, 3), 6);
        assert_eq!(count_reachable(&garden, garden.start, 6), 16);
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

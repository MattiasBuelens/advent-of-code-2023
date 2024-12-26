use std::collections::{HashMap, HashSet};

use aoc_runner_derive::{aoc, aoc_generator};
use derivative::Derivative;
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

#[derive(Debug, Copy, Clone, Eq, Derivative)]
#[derivative(PartialEq, Hash)]
struct State {
    pos: Vector2D,
    #[derivative(PartialEq = "ignore")]
    #[derivative(Hash = "ignore")]
    steps: usize,
}

fn find_reachable(garden: &Garden, start: Vector2D) -> impl Iterator<Item = State> + '_ {
    bfs_reach(
        State {
            pos: start,
            steps: 0,
        },
        move |&state| {
            state.pos.neighbours().filter_map(move |next_pos| {
                if (0..garden.width).contains(&next_pos.x())
                    && (0..garden.height).contains(&next_pos.y())
                    && !garden.rocks.contains(&next_pos)
                {
                    Some(State {
                        pos: next_pos,
                        steps: state.steps + 1,
                    })
                } else {
                    None
                }
            })
        },
    )
}

fn count_reachable(garden: &Garden, steps: usize) -> usize {
    find_reachable(garden, garden.start)
        .take_while(|state| state.steps <= steps)
        .filter(|state| {
            // Must have same parity
            &state.steps % 2 == steps % 2
        })
        .count()
}

#[aoc(day21, part1)]
fn part1(garden: &Garden) -> usize {
    count_reachable(garden, 64)
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

fn count_wrapping_reachable_optimized(garden: &Garden, steps: usize) -> usize {
    // https://github.com/villuna/aoc23/wiki/A-Geometric-solution-to-advent-of-code-2023,-day-21
    assert_eq!(garden.width, garden.height);
    assert_eq!(garden.width % 2, 1);
    // Distance to nearest edge is the same in all directions.
    // When we travel to the edge, the remaining number of steps is a multiple of the garden's size.
    let distance_to_edge = steps % (garden.width as usize);
    assert_eq!(distance_to_edge, (garden.width as usize) / 2);
    let reachable = find_reachable(garden, garden.start)
        .map(|state| (state.pos, state.steps))
        .collect::<Vec<_>>();
    // Split by parity. (See below.)
    let even_full = reachable
        .iter()
        .filter(|&&(_, steps)| steps % 2 == 0)
        .count();
    let odd_full = reachable
        .iter()
        .filter(|&&(_, steps)| steps % 2 == 1)
        .count();
    let even_corner = reachable
        .iter()
        .filter(|&&(_, steps)| steps % 2 == 0 && steps > distance_to_edge)
        .count();
    let odd_corner = reachable
        .iter()
        .filter(|&&(_, steps)| steps % 2 == 1 && steps > distance_to_edge)
        .count();
    let distances = reachable.into_iter().collect::<HashMap<_, _>>();
    assert_eq!(
        distance_to_edge,
        *distances.get(&Vector2D::new(0, garden.start.y())).unwrap()
    );
    assert_eq!(
        distance_to_edge,
        *distances
            .get(&Vector2D::new(garden.width - 1, garden.start.y()))
            .unwrap()
    );
    assert_eq!(
        distance_to_edge,
        *distances.get(&Vector2D::new(garden.start.x(), 0)).unwrap()
    );
    assert_eq!(
        distance_to_edge,
        *distances
            .get(&Vector2D::new(garden.start.x(), garden.height - 1))
            .unwrap()
    );
    // After traveling to the first square's edge, we'll travel an integer number of squares
    // in all directions. This forms a "diamond" pattern, where the last square in each direction
    // is only partially reachable.
    assert_eq!((steps - distance_to_edge) % (garden.width as usize), 0);
    let radius = (steps - distance_to_edge) / (garden.width as usize);
    // We must perform an odd number of steps.
    // For the first square, we take the number of positions reachable in an odd number of steps.
    // Since the grid size is odd, the parity flips at every edge between two squares.
    // So the square to the right of the first square is even, the square to its right is odd, etc.
    // For a diamond with even radius R, there are (R+1)^2 fully covered even squares and R^2 odd ones.
    let full_odd_squares = (radius + 1) * (radius + 1);
    let full_even_squares = radius * radius;
    // Along each edge of the diamond, there are (R+1) odd corners that need to be cut away
    // and R even corners that need to be added in.
    // Multiply by 4 to get all edges, and these regions correspond to (R+1) odd plots
    // that are further away from the start than the nearest edge, and R even plots.
    let partial_odd_squares = radius + 1;
    let partial_even_squares = radius;
    full_odd_squares * odd_full + full_even_squares * even_full - partial_odd_squares * odd_corner
        + partial_even_squares * even_corner
}

#[aoc(day21, part2)]
fn part2(garden: &Garden) -> usize {
    count_wrapping_reachable_optimized(garden, 26_501_365)
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = "...........
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
    const INPUT: &str = include_str!("../input/2023/day21.txt");

    #[test]
    fn part1_example() {
        let garden = parse(EXAMPLE);
        assert_eq!(count_reachable(&garden, 0), 1);
        assert_eq!(count_reachable(&garden, 1), 2);
        assert_eq!(count_reachable(&garden, 2), 4);
        assert_eq!(count_reachable(&garden, 3), 6);
        assert_eq!(count_reachable(&garden, 6), 16);
    }

    #[test]
    fn part2_example() {
        let garden = parse(EXAMPLE);
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

    #[test]
    fn part2_input() {
        let garden = parse(INPUT);
        assert_eq!(count_wrapping_reachable(&garden, 131 + 65), 34234);
        assert_eq!(count_wrapping_reachable(&garden, 131 * 5 + 65), 459046);
        assert_eq!(count_wrapping_reachable(&garden, 131 * 10 + 65), 1672171);
    }

    #[test]
    fn part2_input_optimized() {
        let garden = parse(INPUT);
        assert_eq!(count_wrapping_reachable_optimized(&garden, 131 + 65), 34234);
        assert_eq!(
            count_wrapping_reachable_optimized(&garden, 131 * 5 + 65),
            459046
        );
        assert_eq!(
            count_wrapping_reachable_optimized(&garden, 131 * 10 + 65),
            1672171
        );
    }
}

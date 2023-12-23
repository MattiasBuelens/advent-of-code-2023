use std::collections::{BTreeSet, HashMap};
use std::hash::Hash;

use aoc_runner_derive::{aoc, aoc_generator};
use pathfinding::prelude::bfs_reach;

use crate::util::{Direction, Vector2D};

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
enum Tile {
    Path,
    Forest,
    Slope(Direction),
}

type Map = HashMap<Vector2D, Tile>;

#[aoc_generator(day23)]
fn parse(input: &str) -> Map {
    input
        .lines()
        .enumerate()
        .flat_map(move |(y, line)| {
            line.chars().enumerate().map(move |(x, c)| {
                let pos = Vector2D::new(x as i32, y as i32);
                let tile = match c {
                    '.' => Tile::Path,
                    '#' => Tile::Forest,
                    '^' => Tile::Slope(Direction::N),
                    '>' => Tile::Slope(Direction::E),
                    'v' => Tile::Slope(Direction::S),
                    '<' => Tile::Slope(Direction::W),
                    _ => panic!("invalid tile {c}"),
                };
                (pos, tile)
            })
        })
        .collect()
}

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
struct State {
    pos: Vector2D,
    cost: u64,
    seen: BTreeSet<Vector2D>,
}

fn successors(state: &State, map: &Map, part2: bool) -> Vec<State> {
    Direction::all()
        .iter()
        .filter_map(|dir| {
            let next_pos = state.pos + dir.step();
            let next_tile = map.get(&next_pos)?;
            let (next_pos, cost) = match next_tile {
                Tile::Path => (next_pos, 1),
                Tile::Slope(_) if part2 => (next_pos, 1),
                Tile::Slope(slope_dir) if slope_dir == dir && !part2 => {
                    // Slide down the slope
                    (next_pos + dir.step(), 2)
                }
                _ => return None,
            };
            if state.seen.contains(&next_pos) {
                return None;
            }
            let mut next_seen = state.seen.clone();
            next_seen.insert(next_pos);
            Some(State {
                pos: next_pos,
                cost: state.cost + cost,
                seen: next_seen,
            })
        })
        .collect()
}

fn solve(map: &Map, part2: bool) -> u64 {
    let max_y = map.keys().map(|pos| pos.y()).max().unwrap();
    let (&start, _) = map
        .iter()
        .find(|(pos, &tile)| pos.y() == 0 && tile == Tile::Path)
        .unwrap();
    let (&goal, _) = map
        .iter()
        .find(|(pos, &tile)| pos.y() == max_y && tile == Tile::Path)
        .unwrap();
    let start_state = State {
        pos: start,
        cost: 0,
        seen: BTreeSet::from([start]),
    };
    let longest = bfs_reach(start_state, |state| successors(state, map, part2))
        .filter(|state| state.pos == goal)
        .max_by_key(|state| state.cost)
        .unwrap();
    longest.cost
}

#[aoc(day23, part1)]
fn part1(map: &Map) -> u64 {
    solve(map, false)
}

#[aoc(day23, part2)]
fn part2(map: &Map) -> u64 {
    solve(map, true)
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "#.#####################
#.......#########...###
#######.#########.#.###
###.....#.>.>.###.#.###
###v#####.#v#.###.#.###
###.>...#.#.#.....#...#
###v###.#.#.#########.#
###...#.#.#.......#...#
#####.#.#.#######.#.###
#.....#.#.#.......#...#
#.#####.#.#.#########v#
#.#...#...#...###...>.#
#.#.#v#######v###.###v#
#...#.>.#...>.>.#.###.#
#####v#.#.###v#.#.###.#
#.....#...#...#.#.#...#
#.#########.###.#.#.###
#...###...#...#...#.###
###.###.#.###v#####v###
#...#...#.#.>.>.#.>.###
#.###.###.#.###.#.#v###
#.....###...###...#...#
#####################.#";

    #[test]
    fn part1_example() {
        assert_eq!(part1(&parse(INPUT)), 94);
    }

    #[test]
    fn part2_example() {
        assert_eq!(part2(&parse(INPUT)), 154);
    }
}

use std::collections::{HashMap, VecDeque};

use aoc_runner_derive::{aoc, aoc_generator};

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

struct Crossing {
    neighbours: HashMap<Direction, (Vector2D, u64)>,
}

type CrossingMap = HashMap<Vector2D, Crossing>;

fn get_neighbour(pos: Vector2D, dir: Direction, map: &Map, part2: bool) -> Option<(Vector2D, u64)> {
    let next_pos = pos + dir.step();
    let next_tile = map.get(&next_pos)?;
    Some(match next_tile {
        Tile::Path => (next_pos, 1),
        Tile::Slope(_) if part2 => (next_pos, 1),
        Tile::Slope(slope_dir) if *slope_dir == dir && !part2 => {
            // Slide down the slope
            (next_pos + dir.step(), 2)
        }
        _ => return None,
    })
}

fn find_next_crossing(
    start_pos: Vector2D,
    start_dir: Direction,
    map: &Map,
    goal: Vector2D,
    part2: bool,
) -> Option<(Vector2D, u64)> {
    let mut prev_pos = start_pos;
    let (mut pos, mut cost) = get_neighbour(start_pos, start_dir, map, part2)?;
    loop {
        let neighbours = Direction::all()
            .iter()
            .filter_map(|&next_dir| get_neighbour(pos, next_dir, map, part2))
            .filter(|(next_pos, _)| next_pos != &prev_pos)
            .collect::<Vec<_>>();
        if neighbours.len() == 1 {
            // Straight path
            let (next_pos, next_cost) = neighbours[0];
            prev_pos = pos;
            pos = next_pos;
            cost += next_cost;
        } else if neighbours.len() > 1 || pos == goal {
            // Found another crossing, or the goal!
            break;
        } else {
            // Dead end
            return None;
        }
    }
    Some((pos, cost))
}

fn find_next_crossings(pos: Vector2D, map: &Map, goal: Vector2D, part2: bool) -> Crossing {
    let neighbours = Direction::all()
        .iter()
        .filter_map(|&dir| Some((dir, find_next_crossing(pos, dir, map, goal, part2)?)))
        .collect::<HashMap<_, _>>();
    Crossing { neighbours }
}

fn reduce_map(map: &Map, start: Vector2D, goal: Vector2D, part2: bool) -> CrossingMap {
    let mut crossings = HashMap::new();
    let mut queue = VecDeque::from([start]);
    while let Some(crossing) = queue.pop_front() {
        let next_crossings = find_next_crossings(crossing, map, goal, part2);
        for &(next_crossing, _cost) in next_crossings.neighbours.values() {
            if !crossings.contains_key(&next_crossing) {
                queue.push_back(next_crossing);
            }
        }
        crossings.insert(crossing, next_crossings);
    }
    crossings
}

impl Crossing {
    fn is_vertical_edge(&self) -> bool {
        !self.neighbours.contains_key(&Direction::W) || !self.neighbours.contains_key(&Direction::E)
    }

    fn is_horizontal_edge(&self) -> bool {
        !self.neighbours.contains_key(&Direction::N) || !self.neighbours.contains_key(&Direction::S)
    }
}

fn dfs_longest(
    start: Vector2D,
    goal: Vector2D,
    seen: Vec<Vector2D>,
    map: &CrossingMap,
) -> Option<u64> {
    let crossing = map.get(&start).unwrap();
    crossing
        .neighbours
        .iter()
        .filter_map(|(&next_dir, &(next_pos, next_cost))| {
            if next_pos == goal {
                return Some(next_cost);
            }
            if seen.contains(&next_pos) {
                return None;
            }
            if crossing.is_vertical_edge() && next_dir == Direction::N {
                return None;
            }
            if crossing.is_horizontal_edge() && next_dir == Direction::W {
                return None;
            }
            let mut next_seen = seen.clone();
            next_seen.push(next_pos);
            let next_longest = next_cost + dfs_longest(next_pos, goal, next_seen, map)?;
            Some(next_longest)
        })
        .max()
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
    let map = reduce_map(map, start, goal, part2);

    dfs_longest(start, goal, vec![start], &map).unwrap()
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

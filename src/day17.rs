use std::collections::HashMap;

use aoc_runner_derive::{aoc, aoc_generator};
use pathfinding::directed::dijkstra::dijkstra;

use crate::util::Vector2D;

struct Map {
    width: i32,
    height: i32,
    blocks: HashMap<Vector2D, u32>,
}

#[aoc_generator(day17)]
fn parse(input: &str) -> Map {
    let height = input.lines().count() as i32;
    let width = input.lines().next().unwrap().len() as i32;
    let mut blocks = HashMap::new();
    for (y, line) in input.lines().enumerate() {
        for (x, c) in line.chars().enumerate() {
            blocks.insert(Vector2D::new(x as i32, y as i32), c.to_digit(10).unwrap());
        }
    }
    Map {
        width,
        height,
        blocks,
    }
}

#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
enum Direction {
    N,
    S,
    W,
    E,
}

impl Direction {
    fn step(self) -> Vector2D {
        match self {
            Direction::N => Vector2D::new(0, -1),
            Direction::S => Vector2D::new(0, 1),
            Direction::W => Vector2D::new(-1, 0),
            Direction::E => Vector2D::new(1, 0),
        }
    }

    fn opposite(self) -> Direction {
        match self {
            Direction::N => Direction::S,
            Direction::S => Direction::N,
            Direction::W => Direction::E,
            Direction::E => Direction::W,
        }
    }

    fn all() -> [Direction; 4] {
        [Direction::N, Direction::S, Direction::W, Direction::E]
    }
}

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
struct State {
    pos: Vector2D,
    dir: Direction,
    straight: usize,
}

impl State {
    fn successors(&self, map: &Map) -> Vec<(State, u32)> {
        Direction::all().into_iter().filter_map(|dir| {
            let pos = self.pos + dir.step();
            let cost = *map.blocks.get(&pos)?;
            // Cannot go backwards (but ignore at start)
            if self.straight > 0 && dir == self.dir.opposite() {
                return None;
            }
            let straight = if dir == self.dir {
                // Must turn after 3 straight steps
                if self.straight >= 3 {
                    return None;
                }
                self.straight + 1
            } else {
                1
            };
            Some((State { pos, dir, straight }, cost))
        }).collect::<Vec<_>>()
    }
}

#[aoc(day17, part1)]
fn part1(map: &Map) -> u32 {
    let start = State {
        pos: Vector2D::new(0, 0),
        dir: Direction::N,
        straight: 0,
    };
    let goal = Vector2D::new(map.width - 1, map.height - 1);
    let (_path, cost) = dijkstra(
        &start,
        |state| state.successors(&map),
        |state| state.pos == goal,
    )
    .unwrap();
    cost
}

#[aoc(day17, part2)]
fn part2(map: &Map) -> u32 {
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "2413432311323
3215453535623
3255245654254
3446585845452
4546657867536
1438598798454
4457876987766
3637877979653
4654967986887
4564679986453
1224686865563
2546548887735
4322674655533";

    #[test]
    fn part1_example() {
        assert_eq!(part1(&parse(INPUT)), 102);
    }

    #[test]
    fn part2_example() {
        assert_eq!(part2(&parse(INPUT)), 0);
    }
}

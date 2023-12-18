use std::collections::HashMap;

use aoc_runner_derive::{aoc, aoc_generator};
use pathfinding::directed::dijkstra::dijkstra;

use crate::util::{Direction, Vector2D};

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

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
struct State {
    pos: Vector2D,
    dir: Direction,
    straight: usize,
}

impl State {
    fn successors(&self, map: &Map, part2: bool) -> Vec<(State, u32)> {
        Direction::all()
            .into_iter()
            .filter_map(|dir| {
                let pos = self.pos + dir.step();
                let cost = *map.blocks.get(&pos)?;
                // Cannot go backwards (but ignore at start)
                if self.straight != 0 && dir == self.dir.opposite() {
                    return None;
                }
                if part2 {
                    // Must NOT turn until at least 4 straight steps (but ignore at start)
                    if dir != self.dir && self.straight < 4 && self.straight != 0 {
                        return None;
                    }
                    // Must turn after 10 straight steps
                    if dir == self.dir && self.straight >= 10 {
                        return None;
                    }
                } else {
                    // Must turn after 3 straight steps
                    if dir == self.dir && self.straight >= 3 {
                        return None;
                    }
                }
                let straight = if dir == self.dir {
                    self.straight + 1
                } else {
                    1
                };
                Some((State { pos, dir, straight }, cost))
            })
            .collect::<Vec<_>>()
    }
}

fn solve(map: &Map, part2: bool) -> u32 {
    let start = State {
        pos: Vector2D::new(0, 0),
        dir: Direction::N,
        straight: 0,
    };
    let goal = Vector2D::new(map.width - 1, map.height - 1);
    let (_path, cost) = dijkstra(
        &start,
        |state| state.successors(map, part2),
        |state| {
            // Cannot stop at end unless at least 4 straight steps
            if part2 && state.straight < 4 {
                return false;
            }
            state.pos == goal
        },
    )
    .unwrap();
    cost
}

#[aoc(day17, part1)]
fn part1(map: &Map) -> u32 {
    solve(map, false)
}

#[aoc(day17, part2)]
fn part2(map: &Map) -> u32 {
    solve(map, true)
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

    const INPUT2: &str = "111111111111
999999999991
999999999991
999999999991
999999999991";

    #[test]
    fn part2_example1() {
        assert_eq!(part2(&parse(INPUT)), 94);
    }

    #[test]
    fn part2_example2() {
        assert_eq!(part2(&parse(INPUT2)), 71);
    }
}

use std::collections::HashMap;

use aoc_runner_derive::{aoc, aoc_generator};

use crate::util::Vector2D;

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
enum Direction {
    N,
    S,
    W,
    E,
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
enum Pipe {
    Vert,
    Hori,
    L,
    J,
    Seven,
    F,
}

type Pipes = HashMap<Vector2D, Pipe>;

#[derive(Debug, Clone)]
struct Input {
    pipes: Pipes,
    start: Vector2D,
}

impl From<char> for Pipe {
    fn from(value: char) -> Self {
        match value {
            '|' => Pipe::Vert,
            '-' => Pipe::Hori,
            'L' => Pipe::L,
            'J' => Pipe::J,
            '7' => Pipe::Seven,
            'F' => Pipe::F,
            c => panic!("invalid pipe {c}"),
        }
    }
}

impl Pipe {
    fn neighbours(self) -> [Direction; 2] {
        match self {
            Pipe::Vert => [Direction::N, Direction::S],
            Pipe::Hori => [Direction::W, Direction::E],
            Pipe::L => [Direction::N, Direction::E],
            Pipe::J => [Direction::N, Direction::W],
            Pipe::Seven => [Direction::S, Direction::W],
            Pipe::F => [Direction::S, Direction::E],
        }
    }

    fn from_neighbours(directions: [Direction; 2]) -> Pipe {
        if directions.contains(&Direction::N) {
            if directions.contains(&Direction::S) {
                return Pipe::Vert;
            }
            if directions.contains(&Direction::W) {
                return Pipe::J;
            }
            if directions.contains(&Direction::E) {
                return Pipe::L;
            }
        } else if directions.contains(&Direction::S) {
            if directions.contains(&Direction::W) {
                return Pipe::Seven;
            }
            if directions.contains(&Direction::E) {
                return Pipe::F;
            }
        } else if directions.contains(&Direction::W) {
            if directions.contains(&Direction::E) {
                return Pipe::Hori;
            }
        }
        panic!("invalid directions: {directions:?}");
    }
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

#[aoc_generator(day10)]
fn parse(input: &str) -> Input {
    let mut pipes = Pipes::new();
    let mut start: Option<Vector2D> = None;
    for (y, line) in input.lines().enumerate() {
        for (x, c) in line.chars().enumerate() {
            let pos = Vector2D::new(x as i32, y as i32);
            if c == '.' {
                // ground, skip
            } else if c == 'S' {
                // start position
                start = Some(pos);
            } else {
                // pipe
                pipes.insert(pos, Pipe::from(c));
            }
        }
    }
    let start = start.unwrap();
    let start_directions = Direction::all()
        .into_iter()
        .filter(|dir| {
            if let Some(neighbour_pipe) = pipes.get(&(start + dir.step())) {
                let neighbour_dirs = neighbour_pipe.neighbours();
                neighbour_dirs.contains(&dir.opposite())
            } else {
                false
            }
        })
        .collect::<Vec<_>>();
    let start_pipe = Pipe::from_neighbours(start_directions.try_into().unwrap());
    pipes.insert(start, start_pipe);
    Input { pipes, start }
}

#[derive(Debug)]
struct State {
    pos: Vector2D,
    dir: Direction,
}

impl State {
    fn step(self, pipes: &Pipes) -> Self {
        let next_pos = self.pos + self.dir.step();
        let next_pipe = pipes.get(&next_pos).unwrap();
        let from_dir = self.dir.opposite();
        let next_dir = *next_pipe
            .neighbours()
            .iter()
            .find(|&&dir| dir != from_dir)
            .unwrap();
        Self {
            pos: next_pos,
            dir: next_dir,
        }
    }
}

#[aoc(day10, part1)]
fn part1(input: &Input) -> u32 {
    let start_neighbours = input.pipes.get(&input.start).unwrap().neighbours();
    let mut states = start_neighbours.map(|dir| State {
        pos: input.start,
        dir,
    });
    let mut steps = 0;
    loop {
        states = states.map(|state| state.step(&input.pipes));
        steps += 1;
        if &states[0].pos == &states[1].pos {
            break;
        }
    }
    steps
}

#[aoc(day10, part2)]
fn part2(input: &Input) -> String {
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;

    const SQUARE_LOOP: &str = "-L|F7
7S-7|
L|7||
-L-J|
L|-JF";
    const COMPLEX_LOOP: &str = "7-F7-
.FJ|7
SJLL7
|F--J
LJ.LJ";

    #[test]
    fn part1_example() {
        assert_eq!(part1(&parse(SQUARE_LOOP)), 4);
        assert_eq!(part1(&parse(COMPLEX_LOOP)), 8);
    }

    #[test]
    fn part2_example() {
        assert_eq!(part2(&parse("<EXAMPLE>")), "<RESULT>");
    }
}

use std::collections::{HashMap, HashSet};

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
    in_dir: Direction,
    out_dir: Direction,
}

impl State {
    fn step(self, pipes: &Pipes) -> Self {
        let next_pos = self.pos + self.out_dir.step();
        let next_pipe = pipes.get(&next_pos).unwrap();
        let in_dir = self.out_dir.opposite();
        let out_dir = *next_pipe
            .neighbours()
            .iter()
            .find(|&&dir| dir != in_dir)
            .unwrap();
        Self {
            pos: next_pos,
            in_dir,
            out_dir,
        }
    }
}

#[aoc(day10, part1)]
fn part1(input: &Input) -> u32 {
    let start_neighbours = input.pipes.get(&input.start).unwrap().neighbours();
    let mut states = [
        State {
            pos: input.start,
            in_dir: start_neighbours[0],
            out_dir: start_neighbours[1],
        },
        State {
            pos: input.start,
            in_dir: start_neighbours[1],
            out_dir: start_neighbours[0],
        },
    ];
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

fn find_main_loop(input: &Input) -> HashSet<Vector2D> {
    let mut main_loop = HashSet::new();
    let start_neighbours = input.pipes.get(&input.start).unwrap().neighbours();
    let mut state = State {
        pos: input.start,
        in_dir: start_neighbours[0],
        out_dir: start_neighbours[1],
    };
    loop {
        main_loop.insert(state.pos);
        state = state.step(&input.pipes);
        if &state.pos == &input.start {
            break;
        }
    }
    main_loop
}

impl Direction {
    fn rotate_left(self) -> Self {
        match self {
            Direction::N => Direction::W,
            Direction::W => Direction::S,
            Direction::S => Direction::E,
            Direction::E => Direction::N,
        }
    }
}

#[aoc(day10, part2)]
fn part2(input: &Input) -> u32 {
    let main_loop = find_main_loop(input);
    let mut in_loop = HashSet::<Vector2D>::new();
    // Start in the top-left corner of the loop.
    let start = *main_loop.iter().min().unwrap();
    assert_eq!(input.pipes.get(&start), Some(&Pipe::F));
    // Follow main loop in clockwise direction.
    let mut state = State {
        pos: start,
        in_dir: Direction::S,
        out_dir: Direction::E,
    };
    loop {
        // Find ground tiles "inside" the loop,
        // starting from this loop pipe and working our way
        // to the other side of the loop.
        let mut inside_dir = state.in_dir.rotate_left();
        while inside_dir != state.out_dir {
            // Mark everything, until we hit the main loop again.
            let step = inside_dir.step();
            let mut inside_pos = state.pos + step;
            while !main_loop.contains(&inside_pos) {
                in_loop.insert(inside_pos);
                inside_pos += step;
            }
            // Rotate left, within the inside of the shape.
            inside_dir = inside_dir.rotate_left();
        }
        // Move along the main loop
        state = state.step(&input.pipes);
        if &state.pos == &start {
            break;
        }
    }
    in_loop.len() as u32
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

    const ENCLOSED1: &str = "..........
.S------7.
.|F----7|.
.||....||.
.||....||.
.|L-7F-J|.
.|..||..|.
.L--JL--J.
..........";
    const ENCLOSED2: &str = ".F----7F7F7F7F-7....
.|F--7||||||||FJ....
.||.FJ||||||||L7....
FJL7L7LJLJ||LJ.L-7..
L--J.L7...LJS7F-7L7.
....F-J..F7FJ|L7L7L7
....L7.F7||L7|.L7L7|
.....|FJLJ|FJ|F7|.LJ
....FJL-7.||.||||...
....L---J.LJ.LJLJ...";
    const ENCLOSED3: &str = "FF7FSF7F7F7F7F7F---7
L|LJ||||||||||||F--J
FL-7LJLJ||||||LJL-77
F--JF--7||LJLJ7F7FJ-
L---JF-JLJ.||-FJLJJ7
|F|F-JF---7F7-L7L|7|
|FFJF7L7F-JF7|JL---7
7-L-JL7||F7|L7F-7F7|
L.L7LFJ|||||FJL7||LJ
L7JLJL-JLJLJL--JLJ.L";

    #[test]
    fn part2_example() {
        assert_eq!(part2(&parse(ENCLOSED1)), 4);
        assert_eq!(part2(&parse(ENCLOSED2)), 8);
        assert_eq!(part2(&parse(ENCLOSED3)), 10);
    }
}

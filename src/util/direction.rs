use super::Vector2D;
use std::hash::{Hash, Hasher};

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum Direction {
    N,
    S,
    W,
    E,
}

impl Direction {
    pub fn step(self) -> Vector2D {
        match self {
            Direction::N => Vector2D::new(0, -1),
            Direction::S => Vector2D::new(0, 1),
            Direction::W => Vector2D::new(-1, 0),
            Direction::E => Vector2D::new(1, 0),
        }
    }

    pub fn opposite(self) -> Direction {
        match self {
            Direction::N => Direction::S,
            Direction::S => Direction::N,
            Direction::W => Direction::E,
            Direction::E => Direction::W,
        }
    }

    pub const COUNT: usize = 4;

    pub const fn all() -> [Direction; Self::COUNT] {
        [Direction::N, Direction::S, Direction::W, Direction::E]
    }

    pub const fn index(self) -> u8 {
        match self {
            Direction::N => 0,
            Direction::S => 1,
            Direction::W => 2,
            Direction::E => 3,
        }
    }
}

impl Hash for Direction {
    fn hash<H: Hasher>(&self, state: &mut H) {
        state.write_u8(self.index());
    }
}

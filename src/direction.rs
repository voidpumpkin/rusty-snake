#[derive(Debug, PartialEq, Clone, Copy)]
pub enum Direction {
    Up,
    Right,
    Down,
    Left,
}

impl Direction {
    pub fn opposite(&self) -> Direction {
        use Direction::{Down, Left, Right, Up};
        match self {
            Up => Down,
            Right => Left,
            Down => Up,
            Left => Right,
        }
    }
}

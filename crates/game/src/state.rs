#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl Direction {
    pub fn opposite(self) -> Self {
        match self {
            Self::Up => Self::Down,
            Self::Down => Self::Up,
            Self::Left => Self::Right,
            Self::Right => Self::Left,
        }
    }
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub struct GridPos {
    pub x: i32,
    pub y: i32,
}

#[derive(Debug, Clone)]
pub struct GameState {
    pub grid_width: i32,
    pub grid_height: i32,
    pub snake: Vec<GridPos>,
    pub direction: Direction,
    pub pending_direction: Direction,
    pub food: GridPos,
    pub score: u32,
    pub alive: bool,
}

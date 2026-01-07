mod state;
mod update;

pub use state::{Direction, GameState, GridPos};
pub use update::{new_game, set_direction, step, StepResult};

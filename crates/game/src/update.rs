use rand::Rng;

use crate::{Direction, GameState, GridPos};

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum StepResult {
    Moved,
    Ate,
    GameOver,
}

pub fn new_game(grid_width: i32, grid_height: i32) -> GameState {
    let start = GridPos {
        x: grid_width / 2,
        y: grid_height / 2,
    };
    let mut state = GameState {
        grid_width,
        grid_height,
        snake: vec![start],
        direction: Direction::Right,
        pending_direction: Direction::Right,
        food: start,
        score: 0,
        alive: true,
    };

    state.food = spawn_food(&state);
    state
}

pub fn set_direction(state: &mut GameState, direction: Direction) {
    if direction != state.pending_direction.opposite() {
        state.pending_direction = direction;
    }
}

pub fn step(state: &mut GameState) -> StepResult {
    if !state.alive {
        return StepResult::GameOver;
    }

    state.direction = state.pending_direction;
    let head = match state.snake.first() {
        Some(head) => *head,
        None => return StepResult::GameOver,
    };

    let next = next_position(head, state.direction);
    if next.x < 0 || next.x >= state.grid_width || next.y < 0 || next.y >= state.grid_height {
        state.alive = false;
        return StepResult::GameOver;
    }

    let tail = state.snake.last().copied();
    let hits_body = state.snake.contains(&next);
    let is_food = next == state.food;
    if hits_body && (is_food || tail != Some(next)) {
        state.alive = false;
        return StepResult::GameOver;
    }

    state.snake.insert(0, next);
    if is_food {
        state.score = state.score.saturating_add(1);
        state.food = spawn_food(state);
        StepResult::Ate
    } else {
        state.snake.pop();
        StepResult::Moved
    }
}

fn next_position(head: GridPos, direction: Direction) -> GridPos {
    match direction {
        Direction::Up => GridPos {
            x: head.x,
            y: head.y + 1,
        },
        Direction::Down => GridPos {
            x: head.x,
            y: head.y - 1,
        },
        Direction::Left => GridPos {
            x: head.x - 1,
            y: head.y,
        },
        Direction::Right => GridPos {
            x: head.x + 1,
            y: head.y,
        },
    }
}

fn spawn_food(state: &GameState) -> GridPos {
    let mut rng = rand::thread_rng();
    let mut candidate = GridPos { x: 0, y: 0 };

    for _ in 0..100 {
        candidate = GridPos {
            x: rng.gen_range(0..state.grid_width),
            y: rng.gen_range(0..state.grid_height),
        };

        if !state.snake.contains(&candidate) {
            return candidate;
        }
    }

    candidate
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn step_moves_snake_and_increments_score_on_food() {
        let mut state = new_game(10, 10);
        let head = state.snake[0];
        let food = GridPos {
            x: head.x + 1,
            y: head.y,
        };
        state.food = food;

        let result = step(&mut state);

        assert_eq!(result, StepResult::Ate);
        assert_eq!(state.score, 1);
        assert_eq!(state.snake.len(), 2);
    }

    #[test]
    fn step_blocks_reverse_direction() {
        let mut state = new_game(10, 10);
        set_direction(&mut state, Direction::Left);
        step(&mut state);

        assert_eq!(state.direction, Direction::Right);
    }

    #[test]
    fn step_allows_moving_into_tail_when_not_growing() {
        let mut state = new_game(10, 10);
        state.snake = vec![
            GridPos { x: 2, y: 2 },
            GridPos { x: 1, y: 2 },
        ];
        state.direction = Direction::Right;
        state.pending_direction = Direction::Right;
        state.food = GridPos { x: 0, y: 0 };

        let result = step(&mut state);

        assert_eq!(result, StepResult::Moved);
        assert!(state.alive);
    }
}

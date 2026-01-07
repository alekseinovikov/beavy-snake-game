use crate::GameState;

pub fn update(state: &mut GameState, dt_seconds: f32) {
    state.ticks = state.ticks.saturating_add(1);
    state.elapsed_seconds += dt_seconds;
}

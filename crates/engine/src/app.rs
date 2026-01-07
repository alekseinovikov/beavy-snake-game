use bevy::diagnostic::FrameTimeDiagnosticsPlugin;
use bevy::prelude::*;

use crate::hud::{cleanup_hud, setup_hud, update_fps_text};
use crate::menu::{cleanup_menu, menu_input, setup_menu};
use crate::state::{AppState, GameResource};

pub fn run() {
    App::new()
        .add_plugins(
            DefaultPlugins.set(WindowPlugin {
                primary_window: Some(Window {
                    title: "Beavy Snake Game".to_string(),
                    resolution: (960, 540).into(),
                    ..default()
                }),
                ..default()
            }),
        )
        .add_plugins(FrameTimeDiagnosticsPlugin::default())
        .init_resource::<GameResource>()
        .init_state::<AppState>()
        .add_systems(Startup, setup_camera)
        .add_systems(OnEnter(AppState::Menu), setup_menu)
        .add_systems(Update, menu_input.run_if(in_state(AppState::Menu)))
        .add_systems(OnExit(AppState::Menu), cleanup_menu)
        .add_systems(OnEnter(AppState::Playing), setup_hud)
        .add_systems(OnExit(AppState::Playing), cleanup_hud)
        .add_systems(
            Update,
            (advance_game, update_fps_text).run_if(in_state(AppState::Playing)),
        )
        .run();
}

fn setup_camera(mut commands: Commands) {
    commands.spawn(Camera2d);
}

fn advance_game(time: Res<Time>, mut state: ResMut<GameResource>) {
    game::update(&mut state.0, time.delta_secs());
}

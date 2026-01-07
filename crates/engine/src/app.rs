use bevy::diagnostic::FrameTimeDiagnosticsPlugin;
use bevy::prelude::*;
use beavy_config as config;

use crate::gameover::{cleanup_game_over, game_over_input, setup_game_over};
use crate::gameplay::{advance_game, cleanup_gameplay, handle_input, setup_gameplay};
use crate::hud::{cleanup_hud, setup_hud, update_fps_text};
use crate::menu::{cleanup_menu, menu_input, setup_menu};
use crate::state::AppState;

pub fn run() {
    App::new()
        .add_plugins(
            DefaultPlugins.set(WindowPlugin {
                primary_window: Some(Window {
                    title: config::window::TITLE.to_string(),
                    resolution: (config::window::WIDTH, config::window::HEIGHT).into(),
                    ..default()
                }),
                ..default()
            }),
        )
        .add_plugins(FrameTimeDiagnosticsPlugin::default())
        .init_state::<AppState>()
        .add_systems(Startup, setup_camera)
        .add_systems(OnEnter(AppState::Menu), setup_menu)
        .add_systems(Update, menu_input.run_if(in_state(AppState::Menu)))
        .add_systems(OnExit(AppState::Menu), cleanup_menu)
        .add_systems(
            OnEnter(AppState::Playing),
            (setup_hud, setup_gameplay),
        )
        .add_systems(
            OnExit(AppState::Playing),
            (cleanup_gameplay, cleanup_hud),
        )
        .add_systems(OnEnter(AppState::GameOver), setup_game_over)
        .add_systems(
            Update,
            game_over_input.run_if(in_state(AppState::GameOver)),
        )
        .add_systems(OnExit(AppState::GameOver), cleanup_game_over)
        .configure_sets(Update, PlayingSet.run_if(in_state(AppState::Playing)))
        .add_systems(
            Update,
            handle_input.in_set(PlayingSet),
        )
        .add_systems(
            Update,
            advance_game.in_set(PlayingSet),
        )
        .add_systems(
            Update,
            update_fps_text.in_set(PlayingSet),
        )
        .run();
}

fn setup_camera(mut commands: Commands) {
    commands.spawn(Camera2d);
}

#[derive(SystemSet, Debug, Hash, PartialEq, Eq, Clone)]
struct PlayingSet;

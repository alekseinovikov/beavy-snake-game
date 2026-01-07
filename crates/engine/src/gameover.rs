use bevy::prelude::*;
use beavy_config as config;

use crate::state::{AppState, GameResource};

#[derive(Component)]
pub(crate) struct GameOverRoot;

#[derive(Component, Copy, Clone)]
pub(crate) enum GameOverAction {
    Restart,
    Exit,
}

pub(crate) fn setup_game_over(mut commands: Commands, state: Res<GameResource>) {
    commands
        .spawn((
            Node {
                width: percent(config::ui::ROOT_PERCENT),
                height: percent(config::ui::ROOT_PERCENT),
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                flex_direction: FlexDirection::Column,
                row_gap: px(config::ui::PANEL_GAP),
                ..default()
            },
            BackgroundColor(color(config::colors::GAME_OVER_BG)),
            GameOverRoot,
        ))
        .with_children(|parent| {
            parent.spawn((
                Text::new(config::text::GAME_OVER_TITLE),
                TextFont {
                    font_size: config::ui::TITLE_FONT_SIZE,
                    ..default()
                },
                TextColor(color(config::colors::WHITE)),
                GameOverRoot,
            ));

            parent.spawn((
                Text::new(format!("{}{}", config::text::SCORE_LABEL, state.0.score)),
                TextFont {
                    font_size: config::ui::SUBTITLE_FONT_SIZE,
                    ..default()
                },
                TextColor(color(config::colors::GAME_OVER_TEXT)),
                GameOverRoot,
            ));

            parent
                .spawn((
                    Button,
                    Node {
                        width: px(config::ui::BUTTON_WIDTH),
                        height: px(config::ui::BUTTON_HEIGHT),
                        justify_content: JustifyContent::Center,
                        align_items: AlignItems::Center,
                        border: UiRect::all(px(config::ui::BUTTON_BORDER)),
                        ..default()
                    },
                    BorderColor::all(color(config::colors::GAME_OVER_BUTTON_BORDER)),
                    BackgroundColor(color(config::colors::GAME_OVER_BUTTON_BG)),
                    GameOverAction::Restart,
                    GameOverRoot,
                ))
                .with_children(|button| {
                    button.spawn((
                        Text::new(config::text::GAME_OVER_RESTART),
                        TextFont {
                            font_size: config::ui::BUTTON_FONT_SIZE,
                            ..default()
                        },
                        TextColor(color(config::colors::WHITE)),
                        GameOverRoot,
                    ));
                });

            parent
                .spawn((
                    Button,
                    Node {
                        width: px(config::ui::BUTTON_WIDTH),
                        height: px(config::ui::BUTTON_HEIGHT),
                        justify_content: JustifyContent::Center,
                        align_items: AlignItems::Center,
                        border: UiRect::all(px(config::ui::BUTTON_BORDER)),
                        ..default()
                    },
                    BorderColor::all(color(config::colors::GAME_OVER_BUTTON_BORDER)),
                    BackgroundColor(color(config::colors::GAME_OVER_EXIT_BG)),
                    GameOverAction::Exit,
                    GameOverRoot,
                ))
                .with_children(|button| {
                    button.spawn((
                        Text::new(config::text::GAME_OVER_EXIT),
                        TextFont {
                            font_size: config::ui::BUTTON_FONT_SIZE,
                            ..default()
                        },
                        TextColor(color(config::colors::WHITE)),
                        GameOverRoot,
                    ));
                });
        });
}

pub(crate) fn game_over_input(
    mut interactions: Query<(&Interaction, &GameOverAction), Changed<Interaction>>,
    mut next_state: ResMut<NextState<AppState>>,
    mut exit: MessageWriter<AppExit>,
) {
    for (interaction, action) in &mut interactions {
        if *interaction == Interaction::Pressed {
            match action {
                GameOverAction::Restart => next_state.set(AppState::Playing),
                GameOverAction::Exit => {
                    let _ = exit.write(AppExit::Success);
                }
            }
        }
    }
}

pub(crate) fn cleanup_game_over(
    mut commands: Commands,
    entities: Query<Entity, (With<GameOverRoot>, Without<ChildOf>)>,
) {
    for entity in &entities {
        commands.entity(entity).despawn();
    }
}

fn color(rgb: (f32, f32, f32)) -> Color {
    Color::srgb(rgb.0, rgb.1, rgb.2)
}

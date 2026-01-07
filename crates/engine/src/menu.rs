use bevy::prelude::*;

use crate::state::AppState;

#[derive(Component)]
pub(crate) struct MenuRoot;

#[derive(Component, Copy, Clone)]
pub(crate) enum MenuButtonAction {
    Start,
    Exit,
}

pub(crate) fn setup_menu(mut commands: Commands) {
    commands
        .spawn((
            Node {
                width: percent(100.0),
                height: percent(100.0),
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                flex_direction: FlexDirection::Column,
                row_gap: px(12.0),
                ..default()
            },
            BackgroundColor(Color::srgb(0.05, 0.06, 0.08)),
            MenuRoot,
        ))
        .with_children(|parent| {
            parent.spawn((
                Text::new("Beavy Snake Game"),
                TextFont {
                    font_size: 42.0,
                    ..default()
                },
                TextColor(Color::WHITE),
                MenuRoot,
            ));

            parent
                .spawn((
                    Button,
                    Node {
                        width: px(220.0),
                        height: px(48.0),
                        justify_content: JustifyContent::Center,
                        align_items: AlignItems::Center,
                        border: UiRect::all(px(1.0)),
                        ..default()
                    },
                    BorderColor::all(Color::srgb(0.18, 0.2, 0.25)),
                    BackgroundColor(Color::srgb(0.1, 0.12, 0.16)),
                    MenuButtonAction::Start,
                    MenuRoot,
                ))
                .with_children(|button| {
                    button.spawn((
                        Text::new("Start Game"),
                        TextFont {
                            font_size: 20.0,
                            ..default()
                        },
                        TextColor(Color::WHITE),
                        MenuRoot,
                    ));
                });

            parent
                .spawn((
                    Button,
                    Node {
                        width: px(220.0),
                        height: px(48.0),
                        justify_content: JustifyContent::Center,
                        align_items: AlignItems::Center,
                        border: UiRect::all(px(1.0)),
                        ..default()
                    },
                    BorderColor::all(Color::srgb(0.18, 0.2, 0.25)),
                    BackgroundColor(Color::srgb(0.1, 0.12, 0.16)),
                    MenuButtonAction::Exit,
                    MenuRoot,
                ))
                .with_children(|button| {
                    button.spawn((
                        Text::new("Exit"),
                        TextFont {
                            font_size: 20.0,
                            ..default()
                        },
                        TextColor(Color::WHITE),
                        MenuRoot,
                    ));
                });
        });
}

pub(crate) fn menu_input(
    mut interactions: Query<(&Interaction, &MenuButtonAction), Changed<Interaction>>,
    mut next_state: ResMut<NextState<AppState>>,
    mut exit: MessageWriter<AppExit>,
) {
    for (interaction, action) in &mut interactions {
        if *interaction == Interaction::Pressed {
            match action {
                MenuButtonAction::Start => next_state.set(AppState::Playing),
                MenuButtonAction::Exit => {
                    exit.write(AppExit::Success);
                }
            }
        }
    }
}

pub(crate) fn cleanup_menu(
    mut commands: Commands,
    menu_entities: Query<Entity, (With<MenuRoot>, Without<ChildOf>)>,
) {
    for entity in &menu_entities {
        commands.entity(entity).despawn();
    }
}

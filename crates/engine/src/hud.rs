use bevy::diagnostic::{DiagnosticsStore, FrameTimeDiagnosticsPlugin};
use bevy::prelude::*;

#[derive(Component)]
pub(crate) struct FpsText;

#[derive(Component)]
pub(crate) struct GameHudRoot;

pub(crate) fn setup_hud(mut commands: Commands) {
    commands
        .spawn((
            Node {
                width: percent(100.0),
                height: percent(100.0),
                ..default()
            },
            GameHudRoot,
        ))
        .with_children(|parent| {
            parent
                .spawn((
                    Text::new("FPS: "),
                    TextFont {
                        font_size: 18.0,
                        ..default()
                    },
                    Node {
                        position_type: PositionType::Absolute,
                        top: px(8.0),
                        left: px(8.0),
                        ..default()
                    },
                    GameHudRoot,
                ))
                .with_child((
                    TextSpan::default(),
                    TextFont {
                        font_size: 18.0,
                        ..default()
                    },
                    TextColor(Color::WHITE),
                    FpsText,
                    GameHudRoot,
                ));
        });
}

pub(crate) fn update_fps_text(
    diagnostics: Res<DiagnosticsStore>,
    mut query: Query<&mut TextSpan, With<FpsText>>,
) {
    for mut span in &mut query {
        if let Some(fps) = diagnostics
            .get(&FrameTimeDiagnosticsPlugin::FPS)
            .and_then(|diag| diag.smoothed())
        {
            **span = format!("{fps:.0}");
        }
    }
}

pub(crate) fn cleanup_hud(
    mut commands: Commands,
    hud_entities: Query<Entity, (With<GameHudRoot>, Without<ChildOf>)>,
) {
    for entity in &hud_entities {
        commands.entity(entity).despawn();
    }
}

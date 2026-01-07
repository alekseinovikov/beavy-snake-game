use bevy::diagnostic::{DiagnosticsStore, FrameTimeDiagnosticsPlugin};
use bevy::prelude::*;
use beavy_config as config;

#[derive(Component)]
pub(crate) struct FpsText;

#[derive(Component)]
pub(crate) struct GameHudRoot;

pub(crate) fn setup_hud(mut commands: Commands) {
    commands
        .spawn((
            Node {
                width: percent(config::ui::ROOT_PERCENT),
                height: percent(config::ui::ROOT_PERCENT),
                ..default()
            },
            GameHudRoot,
        ))
        .with_children(|parent| {
            parent
                .spawn((
                    Text::new(config::text::FPS_LABEL),
                    TextFont {
                        font_size: config::ui::HUD_FONT_SIZE,
                        ..default()
                    },
                    Node {
                        position_type: PositionType::Absolute,
                        top: px(config::ui::FPS_TOP),
                        left: px(config::ui::FPS_LEFT),
                        ..default()
                    },
                    GameHudRoot,
                ))
                .with_child((
                    TextSpan::default(),
                    TextFont {
                        font_size: config::ui::HUD_FONT_SIZE,
                        ..default()
                    },
                    TextColor(color(config::colors::WHITE)),
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

fn color(rgb: (f32, f32, f32)) -> Color {
    Color::srgb(rgb.0, rgb.1, rgb.2)
}

#[cfg(test)]
mod tests {
    use super::*;
    use bevy::prelude::{App, Entity, With};

    #[test]
    fn setup_hud_spawns_entities() {
        let mut app = App::new();
        app.add_systems(Startup, setup_hud);
        app.update();

        let count = {
            let world = app.world_mut();
            let mut query = world.query_filtered::<Entity, With<GameHudRoot>>();
            query.iter(world).count()
        };
        assert!(count > 0);
    }

    #[test]
    fn cleanup_hud_despawns_roots() {
        let mut app = App::new();
        app.add_systems(Startup, setup_hud);
        app.add_systems(Update, cleanup_hud);
        app.update();

        let count = {
            let world = app.world_mut();
            let mut query = world.query_filtered::<Entity, With<GameHudRoot>>();
            query.iter(world).count()
        };
        assert_eq!(count, 0);
    }
}

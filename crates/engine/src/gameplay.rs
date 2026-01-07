use bevy::prelude::*;
use bevy::transform::components::GlobalTransform;
use beavy_config as config;

use crate::state::{AppState, GameResource};

#[derive(Resource)]
pub(crate) struct StepTimer(Timer);

#[derive(Component)]
pub(crate) struct SnakeSegment {
    index: usize,
}

#[derive(Component)]
pub(crate) struct FoodSprite;

#[derive(Component)]
pub(crate) struct ScoreText;

#[derive(Component)]
pub(crate) struct ScoreRoot;

#[derive(Component)]
pub(crate) struct BorderSegment;

type GameplayCleanupQuery =
    Or<(With<SnakeSegment>, With<FoodSprite>, With<ScoreRoot>, With<BorderSegment>)>;
pub(crate) fn setup_gameplay(mut commands: Commands) {
    let state = game::new_game(config::grid::WIDTH, config::grid::HEIGHT);
    let state_snapshot = state.clone();
    commands.insert_resource(GameResource(state));
    commands.insert_resource(StepTimer(Timer::from_seconds(
        config::timing::TICK_SECONDS,
        TimerMode::Repeating,
    )));

    spawn_borders(&mut commands, &state_snapshot);
    spawn_snake(&mut commands, &state_snapshot);
    spawn_food(&mut commands, &state_snapshot);
    spawn_score(&mut commands);
}

pub(crate) fn handle_input(keys: Res<ButtonInput<KeyCode>>, mut state: ResMut<GameResource>) {
    if keys.just_pressed(KeyCode::ArrowUp) || keys.just_pressed(KeyCode::KeyW) {
        game::set_direction(&mut state.0, game::Direction::Up);
    } else if keys.just_pressed(KeyCode::ArrowDown) || keys.just_pressed(KeyCode::KeyS) {
        game::set_direction(&mut state.0, game::Direction::Down);
    } else if keys.just_pressed(KeyCode::ArrowLeft) || keys.just_pressed(KeyCode::KeyA) {
        game::set_direction(&mut state.0, game::Direction::Left);
    } else if keys.just_pressed(KeyCode::ArrowRight) || keys.just_pressed(KeyCode::KeyD) {
        game::set_direction(&mut state.0, game::Direction::Right);
    }
}

#[allow(clippy::type_complexity)]
pub(crate) fn advance_game(
    time: Res<Time>,
    mut timer: ResMut<StepTimer>,
    mut state: ResMut<GameResource>,
    mut next_state: ResMut<NextState<AppState>>,
    mut commands: Commands,
    mut transforms: ParamSet<(
        Query<(Entity, &mut SnakeSegment, &mut Transform)>,
        Query<&mut Transform, With<FoodSprite>>,
    )>,
    mut score_text: Query<&mut TextSpan, With<ScoreText>>,
) {
    if !timer.0.tick(time.delta()).just_finished() {
        return;
    }

    let result = game::step(&mut state.0);
    sync_snake(&mut commands, &state.0, &mut transforms.p0());
    sync_food(&state.0, &mut transforms.p1());
    update_score(&state.0, &mut score_text);

    if result == game::StepResult::GameOver {
        next_state.set(AppState::GameOver);
    }
}

pub(crate) fn cleanup_gameplay(
    mut commands: Commands,
    entities: Query<Entity, GameplayCleanupQuery>,
) {
    for entity in &entities {
        commands.entity(entity).despawn();
    }
}

fn spawn_snake(commands: &mut Commands, state: &game::GameState) {
    for (index, segment) in state.snake.iter().enumerate() {
        commands.spawn((
            Sprite {
                color: color(config::colors::SNAKE),
                custom_size: Some(Vec2::splat(config::grid::SNAKE_SIZE)),
                ..default()
            },
            Transform::from_translation(grid_to_world(
                state.grid_width,
                state.grid_height,
                *segment,
            )),
            GlobalTransform::default(),
            Visibility::Visible,
            InheritedVisibility::default(),
            ViewVisibility::default(),
            SnakeSegment { index },
        ));
    }
}

fn spawn_food(commands: &mut Commands, state: &game::GameState) {
    commands.spawn((
        Sprite {
            color: color(config::colors::FOOD),
            custom_size: Some(Vec2::splat(config::grid::FOOD_SIZE)),
            ..default()
        },
        Transform::from_translation(grid_to_world(
            state.grid_width,
            state.grid_height,
            state.food,
        )),
        GlobalTransform::default(),
        Visibility::Visible,
        InheritedVisibility::default(),
        ViewVisibility::default(),
        FoodSprite,
    ));
}

fn spawn_score(commands: &mut Commands) {
    commands
        .spawn((
            Text::new(config::text::SCORE_LABEL),
            TextFont {
                font_size: config::ui::SCORE_FONT_SIZE,
                ..default()
            },
            Node {
                position_type: PositionType::Absolute,
                top: px(config::ui::SCORE_TOP),
                right: px(config::ui::SCORE_RIGHT),
                ..default()
            },
            ScoreRoot,
        ))
        .with_child((
            TextSpan::new("0"),
            TextFont {
                font_size: config::ui::SCORE_FONT_SIZE,
                ..default()
            },
            TextColor(color(config::colors::WHITE)),
            ScoreText,
        ));
}

fn spawn_borders(commands: &mut Commands, state: &game::GameState) {
    let width = state.grid_width as f32 * config::grid::CELL_SIZE;
    let height = state.grid_height as f32 * config::grid::CELL_SIZE;
    let half_width = width / 2.0;
    let half_height = height / 2.0;
    let thickness = config::grid::BORDER_THICKNESS;
    let color = color(config::colors::BORDER);

    let segments = [
        (
            Vec2::new(width + thickness, thickness),
            Vec3::new(0.0, half_height + thickness / 2.0, config::grid::BORDER_Z),
        ),
        (
            Vec2::new(width + thickness, thickness),
            Vec3::new(0.0, -half_height - thickness / 2.0, config::grid::BORDER_Z),
        ),
        (
            Vec2::new(thickness, height + thickness),
            Vec3::new(-half_width - thickness / 2.0, 0.0, config::grid::BORDER_Z),
        ),
        (
            Vec2::new(thickness, height + thickness),
            Vec3::new(half_width + thickness / 2.0, 0.0, config::grid::BORDER_Z),
        ),
    ];

    for (size, translation) in segments {
        commands.spawn((
            Sprite {
                color,
                custom_size: Some(size),
                ..default()
            },
            Transform::from_translation(translation),
            GlobalTransform::default(),
            Visibility::Visible,
            InheritedVisibility::default(),
            ViewVisibility::default(),
            BorderSegment,
        ));
    }
}

fn sync_snake(
    commands: &mut Commands,
    state: &game::GameState,
    segments: &mut Query<(Entity, &mut SnakeSegment, &mut Transform)>,
) {
    let mut items: Vec<(Entity, usize, Mut<Transform>)> = segments
        .iter_mut()
        .map(|(entity, segment, transform)| (entity, segment.index, transform))
        .collect();
    items.sort_by_key(|(_, index, _)| *index);

    for (index, pos) in state.snake.iter().enumerate() {
        if let Some((_, _, transform)) = items.iter_mut().find(|(_, idx, _)| *idx == index) {
            transform.translation = grid_to_world(state.grid_width, state.grid_height, *pos);
        } else {
            commands.spawn((
                Sprite {
                    color: color(config::colors::SNAKE),
                    custom_size: Some(Vec2::splat(config::grid::SNAKE_SIZE)),
                    ..default()
                },
                Transform::from_translation(grid_to_world(
                    state.grid_width,
                    state.grid_height,
                    *pos,
                )),
                GlobalTransform::default(),
                Visibility::Visible,
                InheritedVisibility::default(),
                ViewVisibility::default(),
                SnakeSegment { index },
            ));
        }
    }

    for (entity, index, _) in items {
        if index >= state.snake.len() {
            commands.entity(entity).despawn();
        }
    }
}

fn sync_food(state: &game::GameState, food: &mut Query<&mut Transform, With<FoodSprite>>) {
    if let Ok(mut transform) = food.single_mut() {
        transform.translation =
            grid_to_world(state.grid_width, state.grid_height, state.food);
    }
}

fn update_score(state: &game::GameState, score_text: &mut Query<&mut TextSpan, With<ScoreText>>) {
    if let Ok(mut span) = score_text.single_mut() {
        **span = state.score.to_string();
    }
}

fn grid_to_world(grid_width: i32, grid_height: i32, pos: game::GridPos) -> Vec3 {
    let offset_x = grid_width as f32 / 2.0 - 0.5;
    let offset_y = grid_height as f32 / 2.0 - 0.5;

    Vec3::new(
        (pos.x as f32 - offset_x) * config::grid::CELL_SIZE,
        (pos.y as f32 - offset_y) * config::grid::CELL_SIZE,
        0.0,
    )
}

fn color(rgb: (f32, f32, f32)) -> Color {
    Color::srgb(rgb.0, rgb.1, rgb.2)
}

use std::time::Duration;

use bevy::{
    app::{App, Startup},
    prelude::*,
    time::{Timer, TimerMode},
    DefaultPlugins,
};
use grid::GridCell;
use rand::Rng;

mod grid;

#[derive(Resource, Clone, Copy, Debug)]
struct GameConfig {
    width: usize,
    height: usize,
    cell_size: f32,
    initial_dencity: f64,
    update_interval_millis: u64,
}

impl GameConfig {
    pub fn window_width(&self) -> f32 {
        self.width as f32 * self.cell_size
    }

    pub fn window_height(&self) -> f32 {
        self.height as f32 * self.cell_size
    }
}

#[derive(Resource)]
struct GridUpdateTimer(Timer);

#[derive(Component)]
struct ResetButton;

fn spawn_grid(commands: &mut Commands, config: &Res<GameConfig>) {
    let offset = Vec3::new(
        -1.0 * (config.window_width() - config.cell_size) / 2.0,
        -1.0 * (config.window_height() - config.cell_size) / 2.0,
        0.0,
    );

    let mut rng = rand::thread_rng();

    for x in 0..config.width {
        for y in 0..config.height {
            let position = Vec3::new(
                x as f32 * config.cell_size,
                y as f32 * config.cell_size,
                0.0,
            ) + offset;

            commands.spawn((
                SpriteBundle {
                    sprite: Sprite {
                        color: Color::BLACK,
                        custom_size: Some(Vec2::new(config.cell_size, config.cell_size)),
                        ..Default::default()
                    },
                    transform: Transform::from_translation(position),
                    ..Default::default()
                },
                GridCell::new(x, y, rng.gen_bool(config.initial_dencity)),
            ));
        }
    }
}

fn reset_game(
    mut commands: Commands,
    query: Query<Entity, With<GridCell>>,
    mut interaction_query: Query<&Interaction, (Changed<Interaction>, With<ResetButton>)>,
    config: Res<GameConfig>,
) {
    for interaction in &mut interaction_query {
        if *interaction == Interaction::Pressed {
            for entity in query.iter() {
                commands.entity(entity).despawn();
            }
            spawn_grid(&mut commands, &config);
        }
    }
    // let mut rng = rand::thread_rng();

    // for mut cell in query.iter_mut() {
    //     cell.is_alive = rng.gen_bool(config.initial_dencity);
    // }
}

fn update_grid_cell(
    // mut commands: Commands,
    mut query: Query<&mut GridCell>,
    time: Res<Time>,
    mut timer: ResMut<GridUpdateTimer>,
    config: Res<GameConfig>,
) {
    if timer.0.tick(time.delta()).just_finished() {
        let mut next_generation_state: Vec<bool> = Vec::new();

        for cell in query.iter() {
            let mut number_of_neighbor_alive = 0;

            for dy in -1..=1 {
                for dx in -1..=1 {
                    if (dx, dy) == (0, 0) {
                        continue;
                    }

                    let nx = (cell.x as isize + dx).rem_euclid(config.width as isize) as usize;
                    let ny = (cell.y as isize + dy).rem_euclid(config.height as isize) as usize;

                    if query.iter().nth(ny * config.width + nx).unwrap().is_alive {
                        number_of_neighbor_alive += 1;
                    }
                }
            }

            match (cell.is_alive, number_of_neighbor_alive) {
                (true, 2) | (true, 3) => next_generation_state.push(true),
                (false, 3) => next_generation_state.push(true),
                _ => next_generation_state.push(false),
            }
        }

        for (i, mut cell) in query.iter_mut().enumerate() {
            cell.is_alive = next_generation_state[i];
        }
    }
}

fn update_grid_visuals(mut query: Query<(&GridCell, &mut Sprite), Changed<GridCell>>) {
    for (cell, mut sprite) in query.iter_mut() {
        sprite.color = if cell.is_alive {
            Color::WHITE
        } else {
            Color::BLACK
        };
    }
}

fn setup(mut commands: Commands, config: Res<GameConfig>) {
    commands.spawn(Camera2dBundle::default());
    println!("{:?}", config);
    spawn_grid(&mut commands, &config);
}

fn setup_ui(mut commands: Commands) {
    commands
        .spawn(NodeBundle {
            style: Style {
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                justify_content: JustifyContent::Center,
                align_items: AlignItems::FlexEnd,
                ..Default::default()
            },
            ..Default::default()
        })
        .with_children(|parent| {
            parent
                .spawn((
                    ButtonBundle {
                        style: Style {
                            width: Val::Px(150.0),
                            height: Val::Px(60.0),
                            margin: UiRect::all(Val::Px(20.0)),
                            justify_content: JustifyContent::Center,
                            align_items: AlignItems::Center,
                            ..default()
                        },
                        background_color: Color::srgb(0.15, 0.15, 0.15).into(),
                        ..Default::default()
                    },
                    ResetButton,
                ))
                .with_children(|parent| {
                    parent.spawn(TextBundle::from("Reset"));
                });
        });
}

fn main() {
    let game_config = GameConfig {
        width: 30,
        height: 30,
        cell_size: 20.0,
        initial_dencity: 0.3,
        update_interval_millis: 100,
    };

    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: String::from("Rust Life Game on Bevy"),
                resolution: (game_config.window_width(), game_config.window_height()).into(),
                ..Default::default()
            }),
            ..Default::default()
        }))
        .insert_resource(game_config)
        .insert_resource(GridUpdateTimer {
            0: Timer::new(
                Duration::from_millis(game_config.update_interval_millis),
                TimerMode::Repeating,
            ),
        })
        .add_systems(Startup, (setup, setup_ui))
        .add_systems(Update, (update_grid_cell, update_grid_visuals, reset_game))
        .run();
}

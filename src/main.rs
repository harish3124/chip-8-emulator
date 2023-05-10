use bevy::prelude::*;
use bevy::window::PrimaryWindow;

use cpu::Cpu;
use input::handle_input;

mod cpu;
mod font;
mod input;

const WINDOW_SIZE: (f32, f32) = (640.0, 320.0);
const TILE_SIZE: f32 = 10.0;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                resolution: WINDOW_SIZE.into(),
                ..default()
            }),
            ..default()
        }))
        .insert_resource(ClearColor(Color::BLACK))
        .init_resource::<Cpu>()
        .add_startup_system(spawn_camera)
        .add_system(remove_pixel)
        .add_system(draw_pixel.after(remove_pixel))
        .add_system(handle_input)
        // .add_system(get_input)
        .run()
}
fn remove_pixel(mut commands: Commands, pixels: Query<(Entity, &Pixel)>, cpu: Res<Cpu>) {
    if cpu.redraw {
        for (entity, pixel_ref) in pixels.iter() {
            if cpu.display[pixel_ref.row][pixel_ref.col] == 0 {
                commands.entity(entity).despawn();
            }
        }
    }
}

fn draw_pixel(mut commands: Commands, mut cpu: ResMut<Cpu>) {
    if cpu.redraw {
        for row in 0..32 {
            for col in 0..64 {
                if cpu.display[row][col] == 1 {
                    commands.spawn((
                        SpriteBundle {
                            sprite: Sprite {
                                // Check if pixel active
                                color: Color::WHITE,
                                custom_size: Some(Vec2::new(TILE_SIZE, TILE_SIZE)),
                                ..default()
                            },
                            // texture: asset_server.load("pixel.png"),
                            transform: Transform::from_xyz(
                                ((col as f32) * TILE_SIZE) + TILE_SIZE / 2.0,
                                ((row as f32) * TILE_SIZE) + TILE_SIZE / 2.0,
                                0.,
                            ),
                            ..default()
                        },
                        Pixel { row: row, col: col },
                    ));
                }
            }
        }
        cpu.redraw = false;
    }
}

#[derive(Component)]
struct Pixel {
    row: usize,
    col: usize,
}

fn spawn_camera(mut commands: Commands, window_query: Query<&Window, With<PrimaryWindow>>) {
    let window = window_query.get_single().unwrap();

    commands.spawn(Camera2dBundle {
        transform: Transform::from_xyz(window.width() / 2.0, window.height() / 2.0, 0.0),
        ..default()
    });
}

// DEBUG
fn get_input(input: Res<Input<KeyCode>>, mut cpu: ResMut<Cpu>) {
    if input.just_pressed(KeyCode::Up) {
        if cpu.display[10][20] == 0 {
            cpu.display[10][20] = 1;
        } else {
            cpu.display[10][20] = 0;
        }
        cpu.redraw = true;
    }
}

use bevy::prelude::*;
use bevy::window::PrimaryWindow;

use cpu::{Cpu, cycle};
use font::load_font;
use graphics::*;
use keymap::handle_input;
use loader::load_rom;
use timers::tick_timer;

mod cpu;
mod font;
mod graphics;
mod keymap;
mod loader;
mod timers;

const WINDOW_SIZE: (f32, f32) = (640.0, 320.0);

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
        .add_startup_system(load_rom)
        .add_startup_system(load_font)
        .add_system(cycle)
        .add_system(remove_pixel)
        .add_system(draw_pixel.after(remove_pixel))
        .add_system(handle_input)
        .add_system(tick_timer)
        .add_system(get_input)
        .run();
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

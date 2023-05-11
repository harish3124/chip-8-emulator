use crate::Cpu;
use bevy::prelude::*;

const TILE_SIZE: f32 = 10.0;

pub fn draw_pixel(mut commands: Commands, mut cpu: ResMut<Cpu>) {
    if cpu.redraw {
        for row in 0..32 {
            for col in 0..64 {
                // (31 - row) to set (0,0) at top left
                if cpu.display[(31 - row)][col] == 1 {
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
                        Pixel {
                            row: (31 - row),
                            col: col,
                        },
                    ));
                }
            }
        }
        cpu.redraw = false;
    }
}

pub fn remove_pixel(mut commands: Commands, pixels: Query<(Entity, &Pixel)>, cpu: Res<Cpu>) {
    if cpu.redraw {
        for (entity, pixel_ref) in pixels.iter() {
            if cpu.display[pixel_ref.row][pixel_ref.col] == 0 {
                commands.entity(entity).despawn();
            }
        }
    }
}

#[derive(Component)]
pub struct Pixel {
    row: usize,
    col: usize,
}

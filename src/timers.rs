use super::Cpu;
use bevy::prelude::{Res, ResMut, Time};

// Decrement at 60Hz (decrement by 1 every 16.666 milliseconds)
const DECREMENT_RATE: u8 = 16;

pub fn tick_timer(mut cpu: ResMut<Cpu>, time: Res<Time>) {
    // time.delta().as_millis()
    let decrement: u8 = (time.delta().as_millis() as u8) / DECREMENT_RATE;
    if cpu.delay_timer > 0 {
        // Decrement timers at 60Hz
        if cpu.delay_timer > decrement {
            cpu.delay_timer -= decrement;
        } else {
            cpu.delay_timer = 0
        }
    }
    if cpu.sound_timer > 0 {
        // TODO: Play Sound

        // Decrement timers at 60Hz
        if cpu.sound_timer > decrement {
            cpu.sound_timer -= decrement;
        } else {
            cpu.sound_timer = 0
        }
    }
}

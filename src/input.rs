use super::cpu::Cpu;
use bevy::prelude::*;

/*            Remap
Keypad                   Keyboard
+-+-+-+-+                +-+-+-+-+
|1|2|3|C|                |1|2|3|4|
+-+-+-+-+                +-+-+-+-+
|4|5|6|D|                |Q|W|E|R|
+-+-+-+-+       =>       +-+-+-+-+
|7|8|9|E|                |A|S|D|F|
+-+-+-+-+                +-+-+-+-+
|A|0|B|F|                |Z|X|C|V|
+-+-+-+-+                +-+-+-+-+
*/

pub fn handle_input(input: Res<Input<KeyCode>>, mut cpu: ResMut<Cpu>) {
    if input.pressed(KeyCode::Key1) {
        cpu.keypad[1] = 1
    } else {
        cpu.keypad[1] = 0
    }

    if input.pressed(KeyCode::Key2) {
        cpu.keypad[2] = 1
    } else {
        cpu.keypad[2] = 0
    }

    if input.pressed(KeyCode::Key3) {
        cpu.keypad[3] = 1
    } else {
        cpu.keypad[3] = 0
    }

    if input.pressed(KeyCode::Key4) {
        cpu.keypad[0x0C] = 1
    } else {
        cpu.keypad[0x0C] = 0
    }

    if input.pressed(KeyCode::Q) {
        cpu.keypad[4] = 1
    } else {
        cpu.keypad[4] = 0
    }

    if input.pressed(KeyCode::W) {
        cpu.keypad[5] = 1
    } else {
        cpu.keypad[5] = 0
    }

    if input.pressed(KeyCode::E) {
        cpu.keypad[6] = 1
    } else {
        cpu.keypad[6] = 0
    }

    if input.pressed(KeyCode::R) {
        cpu.keypad[0x0D] = 1
    } else {
        cpu.keypad[0x0D] = 0
    }

    if input.pressed(KeyCode::A) {
        cpu.keypad[7] = 1
    } else {
        cpu.keypad[7] = 0
    }

    if input.pressed(KeyCode::S) {
        cpu.keypad[8] = 1
    } else {
        cpu.keypad[8] = 0
    }

    if input.pressed(KeyCode::D) {
        cpu.keypad[9] = 1
    } else {
        cpu.keypad[9] = 0
    }

    if input.pressed(KeyCode::F) {
        cpu.keypad[0x0E] = 1
    } else {
        cpu.keypad[0x0E] = 0
    }

    if input.pressed(KeyCode::Z) {
        cpu.keypad[0x0A] = 1
    } else {
        cpu.keypad[0x0A] = 0
    }

    if input.pressed(KeyCode::X) {
        cpu.keypad[0] = 1
    } else {
        cpu.keypad[0] = 0
    }

    if input.pressed(KeyCode::C) {
        cpu.keypad[0x0B] = 1
    } else {
        cpu.keypad[0x0B] = 0
    }

    if input.pressed(KeyCode::V) {
        cpu.keypad[0x0F] = 1
    } else {
        cpu.keypad[0x0F] = 0
    }
}

use bevy::prelude::{ResMut, Resource};

#[derive(Resource)]
pub struct Cpu {
    pub memory: [u8; 4096], // 4 KB Memory
    pub opcode: u16,
    pub V: [u8; 16], // General Registers
    pub I: u16,      // Special Register
    pub pc: u16,     // Program Counter
    pub display: [[u8; 64]; 32],
    pub delay_timer: u8,
    pub sound_timer: u8,
    pub stack: [u16; 16],
    pub sp: u8, // Stack Pointer
    pub keypad: [u8; 16],
    pub redraw: bool,
}

impl Default for Cpu {
    fn default() -> Self {
        Self {
            memory: [0; 4096],
            opcode: 0,
            V: [0; 16],
            I: 0,
            pc: 0x200,
            display: [[0; 64]; 32],
            delay_timer: 0,
            sound_timer: 0,
            stack: [0; 16],
            sp: 0,
            keypad: [0; 16],
            redraw: false,
        }
    }
}

pub fn cycle(mut cpu: ResMut<Cpu>) {
    cpu.opcode =
        u16::from(cpu.memory[cpu.pc as usize]) << 8 | u16::from(cpu.memory[cpu.pc as usize + 1]);

    // TODO execute opcodes
}

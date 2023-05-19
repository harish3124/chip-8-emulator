use bevy::prelude::{ResMut, Resource};
use rand::{prelude::thread_rng, Rng};

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

    // TODO this does not work, problem with matching
    match cpu.opcode & 0xF000 {
        0x0000 => {
            // Clear the display.
            if cpu.opcode == 0x00E0 {
                cpu.display = [[0; 64]; 32];
                cpu.pc += 2;
            }

            // Return from a subroutine.
            if cpu.opcode == 0x00EE {
                cpu.pc = cpu.stack[cpu.sp as usize];
                cpu.sp -= 1;
            }
        }

        // Jump to location nnn.
        0x1000 => {
            cpu.pc = cpu.opcode & 0x0FFF;
        }

        // Call subroutine at nnn.
        0x2000 => {
            cpu.sp += 1;
            let new_sp = cpu.sp as usize;
            cpu.stack[new_sp] = cpu.pc.clone();
            cpu.pc = cpu.opcode & 0x0FFF;
        }

        // Skip next instruction if Vx = kk.
        0x3000 => {
            let x = ((cpu.opcode & 0x0F00) >> 8) as usize;
            let kk = (cpu.opcode & 0x00FF) as u8;
            if cpu.V[x] == kk {
                cpu.pc += 2
            }
            cpu.pc += 2;
        }

        // Skip next instruction if Vx != kk.
        0x4000 => {
            let x = ((cpu.opcode & 0x0F00) >> 8) as usize;
            let kk = (cpu.opcode & 0x00FF) as u8;
            if cpu.V[x] != kk {
                cpu.pc += 2
            }
            cpu.pc += 2;
        }
        // Skip next instruction if Vx = Vy.
        0x5000 => {
            let x = ((cpu.opcode & 0x0F00) >> 8) as usize;
            let y = ((cpu.opcode & 0x00F0) >> 4) as usize;

            if cpu.V[x] != cpu.V[y] {
                cpu.pc += 2
            }
            cpu.pc += 2;
        }
        // Set Vx = kk.
        0x6000 => {
            let x = ((cpu.opcode & 0x0F00) >> 8) as usize;
            let kk = (cpu.opcode & 0x00FF) as u8;

            cpu.V[x] = kk;

            cpu.pc += 2
        }

        // Set Vx = Vx + kk.
        0x7000 => {
            let x = ((cpu.opcode & 0x0F00) >> 8) as usize;
            let kk = (cpu.opcode & 0x00FF) as u8;

            cpu.V[x] = cpu.V[x] + kk;

            cpu.pc += 2
        }

        0x8000 => match cpu.opcode & 0x000F {
            // Set Vx = Vy.
            0x0000 => {
                let x = ((cpu.opcode & 0x0F00) >> 8) as usize;
                let y = ((cpu.opcode & 0x00F0) >> 4) as usize;

                cpu.V[x] = cpu.V[y];

                cpu.pc += 2;
            }

            // Set Vx = Vx OR Vy. (Bitwise OR)
            0x0001 => {
                let x = ((cpu.opcode & 0x0F00) >> 8) as usize;
                let y = ((cpu.opcode & 0x00F0) >> 4) as usize;

                cpu.V[x] = cpu.V[x] | cpu.V[y];

                cpu.pc += 2;
            }

            // Set Vx = Vx AND Vy.
            0x0002 => {
                let x = ((cpu.opcode & 0x0F00) >> 8) as usize;
                let y = ((cpu.opcode & 0x00F0) >> 4) as usize;

                cpu.V[x] = cpu.V[x] & cpu.V[y];

                cpu.pc += 2;
            }

            // Set Vx = Vx XOR Vy.
            0x0003 => {
                let x = ((cpu.opcode & 0x0F00) >> 8) as usize;
                let y = ((cpu.opcode & 0x00F0) >> 4) as usize;

                cpu.V[x] = cpu.V[x] ^ cpu.V[y];

                cpu.pc += 2;
            }

            // Set Vx = Vx + Vy, set VF = carry.
            0x0004 => {
                let x = ((cpu.opcode & 0x0F00) >> 8) as usize;
                let y = ((cpu.opcode & 0x00F0) >> 4) as usize;

                let sum = cpu.V[x] as u16 + cpu.V[y] as u16;
                if sum > 0x00FF {
                    cpu.V[0xF] = 1
                } else {
                    cpu.V[0xF] = 0
                }

                cpu.V[x] = (sum & 0x0FF) as u8;

                cpu.pc += 2;
            }

            // Set Vx = Vx - Vy, set VF = NOT borrow.
            0x0005 => {
                let x = ((cpu.opcode & 0x0F00) >> 8) as usize;
                let y = ((cpu.opcode & 0x00F0) >> 4) as usize;

                if cpu.V[x] > cpu.V[y] {
                    cpu.V[0xF] = 1;
                } else {
                    cpu.V[0xF] = 0
                }
                cpu.V[x] = cpu.V[x] - cpu.V[y];

                cpu.pc += 2;
            }

            // Set Vx = Vx SHR 1.
            0x0006 => {
                let x = ((cpu.opcode & 0x0F00) >> 8) as usize;

                cpu.V[0xF] = cpu.V[x] & 0x000F;
                cpu.V[x] >>= 1;

                cpu.pc += 2;
            }

            // Set Vx = Vy - Vx, set VF = NOT borrow.
            0x0007 => {
                let x = ((cpu.opcode & 0x0F00) >> 8) as usize;
                let y = ((cpu.opcode & 0x00F0) >> 4) as usize;

                if cpu.V[x] < cpu.V[y] {
                    cpu.V[0xF] = 1;
                } else {
                    cpu.V[0xF] = 0
                }
                cpu.V[x] = cpu.V[y] - cpu.V[x];

                cpu.pc += 2;
            }

            // Stores the most significant bit of VX in VF and then shifts VX to the left by 1.
            0x000E => {
                let x = ((cpu.opcode & 0x0F00) >> 8) as usize;

                cpu.V[0xF] = cpu.V[x] & 0x000F;
                cpu.V[x] <<= 1;

                cpu.pc += 2;
            }
            _ => (),
        },

        // Skip next instruction if Vx != Vy.
        0x9000 => {
            let x = ((cpu.opcode & 0x0F00) >> 8) as usize;
            let y = ((cpu.opcode & 0x00F0) >> 4) as usize;

            if cpu.V[x] != cpu.V[y] {
                cpu.pc += 2;
            }

            cpu.pc += 2;
        }

        // Set I = nnn.
        0xA000 => {
            cpu.I = cpu.opcode & 0x0FFF;
            cpu.pc += 2;
        }

        // Jump to location nnn + V0.
        0xB000 => {
            cpu.pc = (cpu.opcode & 0x0FFF) + cpu.V[0] as u16;
        }

        // Set Vx = random byte AND kk.
        0xC000 => {
            let rand_num = thread_rng().gen::<u8>();

            let x = ((cpu.opcode & 0x0F00) >> 8) as usize;
            let kk = (cpu.opcode & 0x00FF) as u8;

            cpu.V[x] = rand_num & kk;
        }

        // TODO Continue With 0xDxyn
        _ => (),
    }

    // println!("{}", cpu.opcode)
}

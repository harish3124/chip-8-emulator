use crate::Cpu;
use bevy::prelude::ResMut;

use std::env;
use std::fs::File;
use std::io::Read;

// TODO check if writing to cpu.memory works.

pub fn load_rom(mut cpu: ResMut<Cpu>) {
    // Read the command-line argument for the file name
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        eprintln!("Please provide the ROM path !");
    }
    let file_name = &args[1];

    // Open the file
    let mut file = File::open(file_name).unwrap();

    // Read the file byte by byte
    let mut buffer = [0u8; 1];

    let mut index: usize = cpu.pc as usize;

    loop {
        match file.read_exact(&mut buffer) {
            Ok(_) => {
                // Print the byte code
                // print!("{:02X} ", buffer[0]);

                cpu.memory[index] = buffer[0];
                index += 1;
            }
            Err(_) => break, // Reached end of file or encountered an error
        }
    }
}

mod emulator;
#[allow(unused_imports)]
#[allow(non_camel_case_types)]
#[allow(unused_parens)]

use std::io::{self, Read};
use std::fs::File;
use std::env;

const MEMORY_SIZE: usize = 1024 * 1024;

fn main() {
    let args = env::args().collect::<Vec<String>>();

    if args.len() != 2 {
        println!("Usage: {} path", args[0]);
    } else if let Ok(mut file) = File::open(&args[1]) {
        let mut emu = emulator::Emulator::new(MEMORY_SIZE, 0x7c00, 0x7c00);
        emu.load(&mut file);

        while emu.eip < (MEMORY_SIZE as u32) {
            let code = emu.get_code8(0);
            println!("EIP = {:X}, Code = {:X}", emu.eip, code);

            if let Some(inst) = emulator::instructions(code) {
                inst(&mut emu);
            } else {
                eprintln!("Not implimented: {:X}", code);
                break;
            }

            if emu.eip == 0x00 {
                println!("\nEnd of program.\n");
                break;
            }
        }
        emu.dump();
    }
}

mod emulator;

#[macro_use]
extern crate clap;

use std::fs::File;
use std::env;
use emulator::*;
use clap::{App, Arg};

const MEMORY_SIZE: usize = 1024 * 1024;

fn main() {
    let app = App::new(crate_name!())
                .version(crate_version!())
                .author(crate_authors!())
                .about(crate_description!())

                .arg(Arg::with_name("verbose")
                    .help("verbose")
                    .long("verbose")
                    .short("v")
                )

                .arg(Arg::with_name("file")
                    .help("binary file")
                    .required(true)
                );
    let matches = app.get_matches();

    if let Some(path) = matches.value_of("file") {
        if let Ok(mut file) = File::open(path) {
            let mut emu = Emulator::new(MEMORY_SIZE, 0x7c00, 0x7c00);
            emu.load(&mut file);

            while emu.eip < (MEMORY_SIZE as u32) {
                let code = emu.get_code8(0);
                println!("EIP = {:X}, Code = {:X}", emu.eip, code);
                
                if matches.is_present("verbose") {
                    let iwn = instruction::instructions_with_name(code);
                    if let Some(inst) = iwn.0 {
                        inst(&mut emu);
                        println!("{}", iwn.1);
                        println!("{}", emu);
                    }
                } else {
                    if let Some(inst) = instruction::instructions(code) {
                        inst(&mut emu);
                    } else {
                        eprintln!("Not implimented: {:X}", code);
                        break;
                    }
                }

                if emu.eip == 0x00 {
                    println!("\nEnd of program.\n");
                    break;
                }
            }
            emu.dump();
        }
    }
}

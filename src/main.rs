mod emulator;

#[macro_use]
extern crate clap;

use std::fs::File;
use std::env;
use emulator::*;
use clap::{App, Arg};

const MEMORY_SIZE: usize = 1024 * 1024;
const ORG: u32 = 0x7C00;

fn main() {
    let matches = App::new(crate_name!())
                .version(crate_version!())
                .author(crate_authors!())
                .about(crate_description!())

                .arg(Arg::with_name("verbose")
                    .help("Run verbose. dump verbose. information will flood.")
                    .long("verbose")
                    .short("v")
                )

                .arg(Arg::with_name("with_name")
                    .help("Run with print each instruction name.")
                    .long("with_name")
                    .short("w")
                )

                .arg(Arg::with_name("quiet")
                    .help("Shut up and explode")
                    .long("quiet")
                    .short("q")
                )

                .arg(Arg::with_name("file")
                    .help("x86 binary file")
                    .required(true)
                ).get_matches();

    if let Some(path) = matches.value_of("file") {
        if let Ok(mut file) = File::open(path) {
            let mut emu = Emulator::new(MEMORY_SIZE, ORG, ORG);
            emu.load(&mut file);
            let flag = RunFlags {
                verbose:    matches.is_present("verbose"),
                with_name:  matches.is_present("with_name"),
                quiet:      matches.is_present("quiet")
            };
            emu.run(flag);
        } else {
            eprintln!("Can't open {}.", path);
        }
    }
}

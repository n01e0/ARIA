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
                    .help("run verbose. dump verbose. information flood")
                    .long("verbose")
                    .short("v")
                )

                .arg(Arg::with_name("quiet")
                    .help("shut up and explode")
                    .long("quiet")
                    .short("q")
                )

                .arg(Arg::with_name("file")
                    .help("x86 binary file")
                    .required(true)
                );
    let matches = app.get_matches();

    if let Some(path) = matches.value_of("file") {
        if let Ok(mut file) = File::open(path) {
            let mut emu = Emulator::new(MEMORY_SIZE, 0x7c00, 0x7c00);
            emu.load(&mut file);
            let flag = RunFlags {
                verbose:    matches.is_present("verbose"),
                quiet:      matches.is_present("quiet")
            };
            emu.run(flag);
        } else {
            eprintln!("Can't open {}.", path);
        }
    }
}

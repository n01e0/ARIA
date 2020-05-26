mod emulator;

#[macro_use]
extern crate clap;

use std::fs::File;
use std::env;
use emulator::*;
use std::io;

const MEMORY_SIZE: usize = 1024 * 1024;
const ORG: u32 = 0x7C00;

fn main() {
    let matches = clap_app!(myapp =>
            (version:   crate_version!())
            (author:    crate_authors!())
            (about:     crate_description!())
            (@arg verbose: -v --verbose "Run verbose. dump verbose. information will flood.")
            (@arg with_name: -w --with_name "Run with print each instruction name.")
            (@arg quiet: -q --quiet "Shut up and explode")
            (@arg file: +required "x86 binary file")
        ).get_matches();

    if let Some(path) = matches.value_of("file") {
        if let Ok(mut file) = File::open(path) {
            let mut emu: Emulator<Vec::<u8>, Vec<u8>> = Emulator::new(MEMORY_SIZE, ORG, ORG, None, None);
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

extern crate aria;

use aria::emulator::{
        *,
        modrm::ModRM,
        modrm::OR::*,
        modrm::Disp::*
};

#[test]
fn test_rm32() {
    let mut emu = Emulator {
        registers: [0, 0, 0, 0, 0, 0, 0, 0], 
        eflags: 0,
        memory: Vec::new(),
        eip: 0,
    };

    let modrm = ModRM {
        mod_byte: 0b11,
        or: RegIndex(0),
        rm: 0b10,
        sib: 0,
        disp: Disp32(0),
    };

    emu.set_rm32(&modrm, 10);
    assert_eq!(emu.get_rm32(&modrm), 10);
}


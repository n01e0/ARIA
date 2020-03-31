extern crate aria;

use aria::emulator::*;

#[test]
fn test_sub_rm32_imm8() {
    let mut emu = Emulator {
        registers: [0, 0, 0, 0, 0xF0, 0, 0, 0], 
        eflags: 0,
        memory: vec![0x83, 0xec, 0x10],
        eip: 0,
    };
    
    instruction::instructions(emu.get_code8(0)).unwrap()(&mut emu);
    assert_eq!(&format!("{:X}", emu.registers[0x4]), "E0");
    assert_eq!(emu.get_register32(0x4), 0xE0);
}


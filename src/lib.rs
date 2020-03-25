pub mod emulator;

use crate::emulator::*;
#[test]
fn test_enum() {
    assert_eq!(Register::EAX as usize, 0);
    assert_eq!(Register::ECX as usize, 1);
    assert_eq!(Register::EDX as usize, 2);
    assert_eq!(Register::EBX as usize, 3);
    assert_eq!(Register::ESP as usize, 4);
    assert_eq!(Register::EBP as usize, 5);
    assert_eq!(Register::ESI as usize, 6);
    assert_eq!(Register::EDI as usize, 7);
    assert_eq!(Register::RegistersCount as usize, 8);

    assert_eq!(format!("{}", Register::EAX), "EAX");
    assert_eq!(format!("{}", Register::ECX), "ECX");
    assert_eq!(format!("{}", Register::EDX), "EDX");
    assert_eq!(format!("{}", Register::EBX), "EBX");
    assert_eq!(format!("{}", Register::ESP), "ESP");
    assert_eq!(format!("{}", Register::EBP), "EBP");
    assert_eq!(format!("{}", Register::ESI), "ESI");
    assert_eq!(format!("{}", Register::EDI), "EDI");
    assert_eq!(format!("{}", Register::RegistersCount), "RegistersCount");
}

#[test]
fn test_emulator_init() {
    let memsiz = 1024 * 1024;
    let emu = Emulator::new(memsiz, 0x0000, 0x7C00);
    assert_eq!(emu.eip, 0);
    assert_eq!(emu.registers[Register::ESP as usize], 0x7c00);
}

#[test]
fn test_get_code8() {
    let emu = Emulator {
        registers: [0, 0, 0, 0, 0, 0, 0, 0],
        eflags: 0,
        memory: vec![0xB8],
        eip: 0,
    };

    assert_eq!(0xB8, emu.get_code8(0));
}

#[test]
fn test_get_sign_code8() {
    let emu = Emulator {
        registers: [0, 0, 0, 0, 0, 0, 0, 0],
        eflags: 0,
        memory: vec![0xFF, 0xFE],
        eip: 0,
    };

    assert_eq!(-2, emu.get_sign_code8(1));
}

#[test]
fn test_get_code32() {
    let emu = Emulator {
        registers: [0, 0, 0, 0, 0, 0, 0, 0],
        eflags: 0,
        memory: vec![0x78, 0x56, 0x34, 0x12],
        eip: 0,
    };

    assert_eq!(0x12345678, emu.get_code32(0));
}

#[test]
fn test_get_memory32() {
    let emu = Emulator {
        registers: [0, 0, 0, 0, 0, 0, 0, 0],
        eflags: 0,
        memory: vec![0x78, 0x56, 0x34, 0x12],
        eip: 0,
    };

    assert_eq!(0x12345678, emu.get_memory32(0));
}

#[test]
fn test_set_memory8() {
    let mut emu = Emulator {
        registers: [0, 0, 0, 0, 0, 0, 0, 0],
        eflags: 0,
        memory: vec![0x00, 0x56, 0x34, 0x12],
        eip: 1,
    };

    emu.set_memory8(0x00, 0x78);
    emu.set_memory8(0x05, 0x01);
    assert_eq!(0x01123456, emu.get_code32(0));
}

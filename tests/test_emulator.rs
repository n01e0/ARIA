extern crate aria;

#[cfg(test)]
mod emulator {
    use aria::emulator::{
                *,
                Register::*
    };
    
    #[test]
    fn emulator_enum() {
        assert_eq!(EAX as usize, 0);
        assert_eq!(ECX as usize, 1);
        assert_eq!(EDX as usize, 2);
        assert_eq!(EBX as usize, 3);
        assert_eq!(ESP as usize, 4);
        assert_eq!(EBP as usize, 5);
        assert_eq!(ESI as usize, 6);
        assert_eq!(EDI as usize, 7);
        assert_eq!(RegistersCount as usize, 8);
    
        assert_eq!(format!("{}", EAX), "EAX");
        assert_eq!(format!("{}", ECX), "ECX");
        assert_eq!(format!("{}", EDX), "EDX");
        assert_eq!(format!("{}", EBX), "EBX");
        assert_eq!(format!("{}", ESP), "ESP");
        assert_eq!(format!("{}", EBP), "EBP");
        assert_eq!(format!("{}", ESI), "ESI");
        assert_eq!(format!("{}", EDI), "EDI");
        assert_eq!(format!("{}", RegistersCount), "RegistersCount");
    }
    
    #[test]
    fn emulator_init() {
        let memsiz = 1024 * 1024;
        let emu = Emulator::new(memsiz, 0x0000, 0x7C00);
        assert_eq!(emu.registers.iter().sum::<u32>(), 0x7c00);
        assert_eq!(emu.registers[Register::ESP as usize], 0x7c00);
        assert_eq!(emu.eflags.raw, 0);
        assert_eq!(emu.memory.capacity(), memsiz + 0x7c00);
        assert_eq!(emu.eip, 0);
    }

    #[test]
    fn emulator_load() {
        let mut memory = vec![0, 1, 2, 3, 4, 5];
        let mut emu = Emulator::new(memory.len(), 0, 0);
        emu.raw_load(&mut memory);
        assert_eq!(emu.memory.iter().sum::<u8>(), 15);
    }
    
    #[test]
    fn emulator_set_memory8() {
        let mut emu = Emulator {
            registers: [0, 0, 0, 0, 0, 0, 0, 0],
            eflags: Eflags{ raw: 0 },
            memory: vec![0x00, 0x56, 0x34, 0x12],
            eip: 0,
        };
    
        emu.set_memory8(0x00, 0xFF78);
        assert_eq!(0x12345678, emu.get_memory32(0));
    }

    #[test]
    fn emulator_get_memory8() {
        let mut emu = Emulator::new(1, 0, 0);
        emu.raw_load(&mut vec![0xFE]);
        assert_eq!(emu.get_memory8(0), 0xFE);
    }

    #[test]
    fn emulator_set_memory32() {
        let mut emu = Emulator::new(4, 0, 0);
        emu.raw_load(&mut vec![0, 0, 0, 0]);
        emu.set_memory32(0, 0x12345678);
        assert_eq!(emu.memory[0], 0x78);
        assert_eq!(emu.memory[1], 0x56);
        assert_eq!(emu.memory[2], 0x34);
        assert_eq!(emu.memory[3], 0x12);
    }

    #[test]
    fn emulator_get_memory32() {
        let mut emu = Emulator::new(4, 0, 0);
        emu.raw_load(&mut vec![0x78, 0x56, 0x34, 0x12]);
        assert_eq!(emu.get_memory32(0), 0x12345678);
    }

    #[test]
    fn emulator_get_code8() {
        let emu = Emulator {
            registers: [0, 0, 0, 0, 0, 0, 0, 0],
            eflags: Eflags{ raw: 0 },
            memory: vec![0xB8],
            eip: 0,
        };
    
        assert_eq!(0xB8, emu.get_code8(0));
    }
    
    #[test]
    fn emulator_get_sign_code8() {
        let emu = Emulator {
            registers: [0, 0, 0, 0, 0, 0, 0, 0],
            eflags: Eflags{ raw: 0 },
            memory: vec![0xFF, 0xFE],
            eip: 0,
        };
    
        assert_eq!(-2, emu.get_sign_code8(1));
    }
    
    #[test]
    fn emulator_get_code32() {
        let emu = Emulator {
            registers: [0, 0, 0, 0, 0, 0, 0, 0],
            eflags: Eflags{ raw: 0 },
            memory: vec![0x78, 0x56, 0x34, 0x12],
            eip: 0,
        };
    
        assert_eq!(0x12345678, emu.get_code32(0));
    }

    #[test]
    fn emulator_get_sign_code32() {
        let mut emu = Emulator::new(4, 0, 0);
        emu.raw_load(&mut vec![0xFF, 0xFF, 0xFF, 0xFF]);
        assert_eq!(-1, emu.get_sign_code32(0));
    }
    
    #[test]
    fn emulator_register32() {
        let mut emu = Emulator {
            registers: [0, 0, 0, 0, 0, 0, 0, 0],
            eflags: Eflags{ raw: 0 },
            memory: vec![0x00, 0x56, 0x34, 0x12],
            eip: 0,
        };
        emu.set_register32(EAX as usize, 0x61) ;
        assert_eq!(emu.registers[EAX as usize], emu.get_register32(EAX as usize));
    }

    #[test]
   fn emulator_stack() {
        let mut emu = Emulator::new(4, 0, 0);
        emu.raw_load(&mut vec![0, 0, 0, 0]);
        emu.registers[ESP as usize] = 4;
        emu.push32(0x12345678);
        assert_eq!(0x12345678, emu.pop32());
   }
}

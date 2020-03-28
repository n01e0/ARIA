pub mod emulator;

#[cfg(test)]
mod tests {
    use crate::emulator::*;
    use crate::emulator::Register::*;
    
    #[test]
    fn test_enum() {
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
            eip: 0,
        };
    
        emu.set_memory8(0x00, 0x78);
        assert_eq!(0x12345678, emu.get_memory32(0));
    }
    
    #[test]
    fn test_register32() {
        let mut emu = Emulator {
            registers: [0, 0, 0, 0, 0, 0, 0, 0],
            eflags: 0,
            memory: vec![0x00, 0x56, 0x34, 0x12],
            eip: 0,
        };
        emu.set_register32(EAX as usize, 0x61) ;
        assert_eq!(emu.registers[EAX as usize], emu.get_register32(EAX as usize));
    }
}

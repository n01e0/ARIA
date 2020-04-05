extern crate aria;

#[cfg(test)]
mod instruction {
    use aria::emulator:: {
        *,
        instruction::*
    };
    
    #[test]
    fn instructions_name() {
        assert_eq!(instructions_with_name(0x01).1, "add_rm32_r32");
        assert_eq!(instructions_with_name(0x50).1, "push_r32");
        assert_eq!(instructions_with_name(0x58).1, "pop_r32");
        assert_eq!(instructions_with_name(0x68).1, "push_imm32");
        assert_eq!(instructions_with_name(0x6A).1, "push_imm8");
        assert_eq!(instructions_with_name(0x83).1, "code_83");
        assert_eq!(instructions_with_name(0x89).1, "mov_rm32_r32");
        assert_eq!(instructions_with_name(0x8B).1, "mov_r32_rm32");
        assert_eq!(instructions_with_name(0xB8).1, "mov_r32_imm32");
        assert_eq!(instructions_with_name(0xC3).1, "ret");
        assert_eq!(instructions_with_name(0xC7).1, "mov_rm32_imm32");
        assert_eq!(instructions_with_name(0xC9).1, "leave");
        assert_eq!(instructions_with_name(0xE8).1, "call_rel32");
        assert_eq!(instructions_with_name(0xE9).1, "near_jump");
        assert_eq!(instructions_with_name(0xEB).1, "short_jump");
        assert_eq!(instructions_with_name(0xFF).1, "code_ff");
    }
    
    #[test]
    fn instruction_mov_r32_imm32() {
        let mut emu = Emulator {
            registers: [0, 0, 0, 0, 0, 0, 0, 0],
            eflags: 0,
            memory: vec![0xB8, 0x00, 0x00, 0x00, 0x00],
            eip: 0,
        };
        
        emu.set_memory32(1, 0x01234567);
        instructions(emu.get_code8(0)).unwrap()(&mut emu);
        assert_eq!(emu.registers[0] as u32, 0x01234567 as u32);
    }
    
    #[test]
    fn instruction_sub_rm32_imm8() {
        let mut emu = Emulator {
            registers: [0, 0, 0, 0, 0xF0, 0, 0, 0], 
            eflags: 0,
            memory: vec![0x83, 0xec, 0x10],
            eip: 0,
        };
        
        instructions(emu.get_code8(0)).unwrap()(&mut emu);
        assert_eq!(&format!("{:X}", emu.registers[0x4]), "E0");
        assert_eq!(emu.get_register32(0x4), 0xE0);
    }
    
}

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
        assert_eq!(instructions_with_name(0x3B).1, "cmp_r32_rm32");
        assert_eq!(instructions_with_name(0x3C).1, "cmp_al_imm8");
        assert_eq!(instructions_with_name(0x3D).1, "cmp_eax_imm32");
        for i in 0x40 ..= 0x47 { 
            assert_eq!(instructions_with_name(i).1, "inc_r32");
        }
        for i in 0x50 ..= 0x57 {
            assert_eq!(instructions_with_name(i).1, "push_r32");
        }
        for i in 0x58 ..= 0x5F {
            assert_eq!(instructions_with_name(0x58).1, "pop_r32");
        }
        assert_eq!(instructions_with_name(0x68).1, "push_imm32");
        assert_eq!(instructions_with_name(0x6A).1, "push_imm8");
        assert_eq!(instructions_with_name(0x70).1, "jump_overflow");
        assert_eq!(instructions_with_name(0x71).1, "jump_not_overflow");
        assert_eq!(instructions_with_name(0x72).1, "jump_carry");
        assert_eq!(instructions_with_name(0x73).1, "jump_not_carry");
        assert_eq!(instructions_with_name(0x74).1, "jump_zero");
        assert_eq!(instructions_with_name(0x75).1, "jump_not_zero");
        assert_eq!(instructions_with_name(0x78).1, "jump_sign");
        assert_eq!(instructions_with_name(0x79).1, "jump_not_sign");
        assert_eq!(instructions_with_name(0x7C).1, "jump_less");
        assert_eq!(instructions_with_name(0x7E).1, "jump_less_or_eq");
        assert_eq!(instructions_with_name(0x83).1, "code_83");
        assert_eq!(instructions_with_name(0x88).1, "mov_rm8_r8");
        assert_eq!(instructions_with_name(0x89).1, "mov_rm32_r32");
        assert_eq!(instructions_with_name(0x8A).1, "mov_r8_rm8");
        assert_eq!(instructions_with_name(0x8B).1, "mov_r32_rm32");
        for i in 0xB0 ..= 0xB7 {
            assert_eq!(instructions_with_name(i).1, "mov_r8_imm8");
        }
        for i in 0xB8 ..= 0xBF {
            assert_eq!(instructions_with_name(0xB8).1, "mov_r32_imm32");
        }
        assert_eq!(instructions_with_name(0xC3).1, "ret");
        assert_eq!(instructions_with_name(0xC7).1, "mov_rm32_imm32");
        assert_eq!(instructions_with_name(0xC9).1, "leave");
        assert_eq!(instructions_with_name(0xE8).1, "call_rel32");
        assert_eq!(instructions_with_name(0xE9).1, "near_jump");
        assert_eq!(instructions_with_name(0xEB).1, "short_jump");
        assert_eq!(instructions_with_name(0xEC).1, "in_al_dx");
        assert_eq!(instructions_with_name(0xEE).1, "out_dx_al");
        assert_eq!(instructions_with_name(0xFF).1, "code_ff");
    }
    
    #[test]
    fn instruction_mov_r32_imm32() {
        let mut emu = Emulator {
            registers: [0, 0, 0, 0, 0, 0, 0, 0],
            eflags: Eflags { raw: 0 },
            memory: vec![0xB8, 0x00, 0x00, 0x00, 0x00],
            eip: 0,
        };
        
        emu.set_memory32(1, 0x01234567);
        instructions(emu.get_code8(0)).unwrap()(&mut emu);
        assert_eq!(emu.registers[0] as u32, 0x01234567 as u32);
    }

    #[test]
    fn instruction_mov_rm32_imm32() {
        let mut emu = Emulator {
            registers: [0, 0, 0, 0, 0, 0, 0, 0],
            eflags: Eflags { raw: 0 },
            memory: vec![0xC7, 0xC0, 0x00, 0x00, 0x00, 0x00],
            eip: 0,
        };

        emu.set_memory32(2, 0x01234567);
        instructions(emu.get_code8(0)).unwrap()(&mut emu);
        assert_eq!(emu.registers[0], 0x01234567);
    }
    
    #[test]
    fn instruction_mov_rm32_r32() {
        let mut emu = Emulator {
            registers: [2, 0, 0, 0, 0, 0, 0, 0],
            eflags: Eflags { raw: 0 },
            memory: vec![0x89, 0x00, 0x00, 0x00, 0x00, 0x00],
            eip: 0,
        };

        instructions(emu.get_code8(0)).unwrap()(&mut emu);
        assert_eq!(emu.registers[0], emu.get_memory32(2));
    }

    #[test]
    fn instruction_mov_r32_rm32() {
        let mut emu = Emulator {
            registers: [0, 2, 0, 0, 0, 0, 0, 0],
            eflags: Eflags { raw: 0 },
            memory: vec![0x8B, 0x11, 0x00, 0x00, 0x00, 0x00],
            eip: 0,
        };

        emu.set_memory32(2, 0x12345678);
        instructions(emu.get_code8(0)).unwrap()(&mut emu);
        assert_eq!(emu.registers[2], emu.get_memory32(2));
    }

    #[test]
    fn instruction_add_rm32_r32() {
        let mut emu = Emulator {
            registers: [0, 0xF0, 0x0F, 0, 0, 0, 0, 0],
            eflags: Eflags { raw: 0 },
            memory: vec![0x01, 0b11010001, 0x00],
            eip: 0,
        };

        instructions(emu.get_code8(0)).unwrap()(&mut emu);
        assert_eq!(emu.registers[1], 0xFF);
    }

    #[test]
    fn instruction_sub_rm32_imm8() {
        let mut emu = Emulator {
            registers: [0, 0, 0, 0, 0xF0, 0, 0, 0], 
            eflags: Eflags { raw: 0 },
            memory: vec![0x83, 0xec, 0x10],
            eip: 0,
        };
        
        instructions(emu.get_code8(0)).unwrap()(&mut emu);
        assert_eq!(emu.get_register32(0x4), 0xE0);
    }
    
    #[test]
    fn instruction_inc_rm32() {
        let mut emu = Emulator {
            registers: [0, 0, 0, 0, 0, 0, 0, 0],
            eflags: Eflags { raw: 0 },
            memory: vec![0xFF, 0b11000111],
            eip: 0,
        };

        instructions(emu.get_code8(0)).unwrap()(&mut emu);
        assert_eq!(emu.registers[7], 1);
    }

    #[test]
    fn instruction_push_r32() {
        let mut emu = Emulator {
            registers: [0, 0xFF, 0, 0, 0x5, 0, 0, 0],
            eflags: Eflags { raw: 0 },
            memory: vec![0x51, 0x00, 0x00, 0x00, 0x00, 0x00],
            eip: 0,
        };

        instructions(emu.get_code8(0)).unwrap()(&mut emu);
        assert_eq!(emu.get_memory32(1), 0xFF);
    }

    #[test]
    fn instruction_push_imm32() {
        let mut emu = Emulator {
            registers: [0, 0, 0, 0, 0x8, 0, 0, 0],
            eflags: Eflags { raw: 0 },
            memory: vec![0x68, 0, 0, 0, 0, 0, 0, 0, 0],
            eip: 0,
        };

        emu.set_memory32(1, 0x12345678);
        instructions(emu.get_code8(0)).unwrap()(&mut emu);
        assert_eq!(emu.get_memory32(4), 0x12345678);
    }

    #[test]
    fn instruction_push_imm8() {
        let mut emu = Emulator {
            registers: [0, 0, 0, 0, 0x6, 0, 0, 0],
            eflags: Eflags { raw: 0 },
            memory: vec![0x6A, 0, 0, 0, 0, 0],
            eip: 0,
        };

        emu.set_memory8(1, 0xFF);
        instructions(emu.get_code8(0)).unwrap()(&mut emu);
        assert_eq!(emu.get_memory32(2), 0xFF);
    }

    #[test]
    fn instruction_pop_r32() {
        let mut emu = Emulator {
            registers: [0, 0, 0, 0, 1, 0, 0, 0],
            eflags: Eflags { raw: 0 },
            memory: vec![0x58, 0, 0, 0, 0],
            eip: 0,
        };

        emu.set_memory32(1, 0x12345678);
        instructions(emu.get_code8(0)).unwrap()(&mut emu);
        assert_eq!(emu.registers[0], 0x12345678);
    }

    #[test]
    fn instruction_short_jump() {
        let mut emu = Emulator {
            registers: [0, 0, 0, 0, 0, 0, 0, 0],
            eflags: Eflags { raw: 0 },
            memory: vec![0xEB, 0xFF],
            eip: 0,
        };

        instructions(emu.get_code8(0)).unwrap()(&mut emu);
        assert_eq!(emu.eip, 1);
    }

    #[test]
    fn instruction_near_jump() {
        let mut emu = Emulator {
            registers: [0, 0, 0, 0, 0, 0, 0, 0],
            eflags: Eflags { raw: 0 },
            memory: vec![0xE9, 0, 0, 0, 0],
            eip: 0,
        };

        emu.set_memory32(1, 0x12345673);
        instructions(emu.get_code8(0)).unwrap()(&mut emu);
        assert_eq!(emu.eip, 0x12345678);
    }

    #[test]
    fn instruction_call_rel32() {
        let mut emu = Emulator {
            registers: [0, 0, 0, 0, 5, 0, 0, 0],
            eflags: Eflags { raw: 0 },
            memory: vec![0xE8, 0, 0, 0, 0],
            eip: 0,
        };

        emu.set_memory32(1, 0x12345673);
        instructions(emu.get_code8(0)).unwrap()(&mut emu);
        assert_eq!(emu.eip, 0x12345678);
        assert_eq!(emu.get_memory32(1), 5);
    }

    #[test]
    fn instruction_ret() {
        let mut emu = Emulator {
            registers: [0, 0, 0, 0, 1, 0, 0, 0],
            eflags: Eflags { raw: 0 },
            memory: vec![0xC3, 0, 0, 0, 0],
            eip: 0,
        };

        emu.set_memory32(1, 0x12345678);
        instructions(emu.get_code8(0)).unwrap()(&mut emu);
        assert_eq!(emu.eip, 0x12345678);
    }

    #[test]
    fn instruction_leave() {
        let mut emu = Emulator {
            registers: [0, 0, 0, 0, 0, 0, 0, 0],
            eflags: Eflags { raw: 0 },
            memory: vec![0xC9, 0, 0, 0, 0],
            eip: 0,
        };

        emu.set_register32(Register::EBP as usize, 1);
        emu.set_memory32(1, 0x12345678);
        instructions(emu.get_code8(0)).unwrap()(&mut emu);
        assert_eq!(emu.registers[Register::EBP as usize], 0x12345678);
    }
}

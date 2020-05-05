use super::*;
use crate::emulator::modrm::*;
#[allow(unused_imports)]
use crate::emulator::bios::*;
#[allow(unused_imports)]
use crate::emulator::RegisterHigh::*;
use crate::emulator::RegisterLow::*;
use crate::emulator::io;

type Instruction = fn(&mut Emulator);

impl Emulator {
    fn mov_r32_imm32(&mut self) {
        let reg = self.get_code8(0) - 0xB8; // 0xB8 == registers[0]
        let value = self.get_code32(1);
        self.registers[reg as usize] = value;
        self.eip += 5;
    }

    fn mov_rm32_imm32(&mut self) {
        self.eip += 1;
        let modrm = self.parse_modrm();
        let value = self.get_code32(0);

        self.eip += 4;
        self.set_rm32(&modrm, value);
    }

    fn mov_rm32_r32(&mut self) {
        self.eip += 1;
        let modrm = self.parse_modrm();
        let r32 = self.get_r32(&modrm);

        self.set_rm32(&modrm, r32);
    }

    fn mov_r32_rm32(&mut self) {
        self.eip += 1;
        let modrm = self.parse_modrm();
        let rm32 = self.get_rm32(&modrm);
        self.set_r32(&modrm, rm32);
    }

    fn mov_r8_imm8(&mut self) {
        let reg = self.get_code8(0) - 0xB0;
        self.set_register8(reg as usize, self.get_code8(1));
        self.eip += 2;
    }

    fn mov_r8_rm8(&mut self) {
        self.eip += 1;
        let modrm = self.parse_modrm();
        let rm8 = self.get_rm8(&modrm);
        self.set_r8(&modrm, rm8);
    }

    fn mov_rm8_r8(&mut self) {
        self.eip += 1;
        let modrm = self.parse_modrm();
        let r8 = self.get_r8(&modrm);
        self.set_rm8(&modrm, r8);
    }

    fn in_al_dx(&mut self) {
        let addr = self.get_register32(EDX as usize) & 0xFFFF;
        let value = io::io_in8(addr as u16);
        self.set_register8(AL as usize, value);
        self.eip += 1;
    }

    fn out_dx_al(&mut self) {
        let addr = self.get_register32(EDX as usize) & 0xFFFF;
        let value = self.get_register8(AL as usize);
        io::io_out8(addr as u16, value);
        self.eip += 1;
    }

    fn add_rm32_r32(&mut self) {
        self.eip += 1;
        let modrm = self.parse_modrm();
        let r32 = self.get_r32(&modrm);
        let rm32 = self.get_rm32(&modrm);
        self.set_rm32(&modrm, rm32 + r32);
    }

    fn add_rm32_imm8(&mut self, modrm: &ModRM) {
        let rm32 = self.get_rm32(&modrm);
        let imm8 = self.get_sign_code8(0) as u32;
        self.eip += 1;
        self.set_rm32(&modrm, rm32 + imm8);
    }

    fn cmp_r32_rm32(&mut self) {
        self.eip += 1;
        let modrm = self.parse_modrm();
        let r32 = self.get_r32(&modrm);
        let rm32 = self.get_rm32(&modrm);
        let result: u64 = (r32 as u64).wrapping_sub(rm32 as u64);
        self.update_eflags_sub(r32, rm32, result);
    }

    fn cmp_rm32_imm8(&mut self, modrm: &ModRM) {
        let rm32 = self.get_rm32(&modrm);
        let imm8 = self.get_sign_code8(0);
        self.eip += 1;
        let result = (rm32 as u64).wrapping_sub(imm8 as u64);
        self.update_eflags_sub(rm32, imm8 as u32, result);
    }

    fn cmp_eax_imm32(&mut self) {
        let value = self.get_code32(1);
        let eax = self.get_register32(EAX as usize);
        let result = (eax as u64).wrapping_sub(value as u64);
        self.update_eflags_sub(eax, value, result);
        self.eip += 5;
    }

    fn cmp_al_imm8(&mut self) {
        let value = self.get_code8(1);
        let al = self.get_register8(AL as usize);
        let result = (al as u64).wrapping_sub(value as u64);
        self.update_eflags_sub(al as u32, value as u32, result);
        self.eip += 2;
    }

    fn sub_rm32_imm8(&mut self, modrm: &ModRM) {
        let rm32 = self.get_rm32(modrm);
        let imm8 = self.get_sign_code8(0) as u32;
        self.eip += 1;
        let result = (rm32 as u64).wrapping_sub(imm8 as u64);
        self.set_rm32(&modrm, rm32 - imm8);
        self.update_eflags_sub(rm32, imm8, result);
    }

    fn code_83(&mut self) {
        self.eip += 1;
        let modrm = self.parse_modrm();

        match modrm.or.unwrap() {
            0 => self.add_rm32_imm8(&modrm),
            5 => self.sub_rm32_imm8(&modrm),
            7 => self.cmp_rm32_imm8(&modrm),
            n => panic!("Not implimented: 83 /{}", n),
        }
    }

    fn inc_r32(&mut self) {
        let reg = self.get_code8(0) - 0x40;
        self.set_register32(reg as usize, self.get_register32(reg as usize) + 1);
        self.eip += 1;
    }

    fn inc_rm32(&mut self, modrm: &ModRM) {
        let value = self.get_rm32(&modrm);
        self.set_rm32(&modrm, value + 1);
    }

    fn code_ff(&mut self) {
        self.eip += 1;
        let modrm = self.parse_modrm();

        match modrm.or.unwrap() {
            0 => self.inc_rm32(&modrm),
            n => panic!("Not implimented: FF /{}", n),
        }
    }

    fn push_r32(&mut self) {
        let reg = self.get_code8(0) - 0x50;
        self.push32(self.get_register32(reg as usize));
        self.eip += 1;
    }

    fn push_imm32(&mut self) {
        let value = self.get_code32(1);
        self.push32(value);
        self.eip += 5;
    }

    fn push_imm8(&mut self) {
        let value = self.get_code8(1);
        self.push32(value as u32);
        self.eip += 2;
    }

    fn pop_r32(&mut self) {
        let reg = self.get_code8(0) - 0x58;
        let value = self.pop32();
        self.set_register32(reg as usize, value);
        self.eip += 1;
    }

    fn short_jump(&mut self) {
        let diff = self.get_sign_code8(1);
        self.eip = ((self.eip as i64) + diff as i64 + 2) as u32;
    }

    fn near_jump(&mut self) {
        let diff = self.get_sign_code32(1);
        self.eip = ((self.eip as i64) + diff as i64 + 5) as u32;
    }

    fn jump_sign(&mut self) {
        let diff = if self.eflags.is_sign() {
            self.get_sign_code8(1)
        } else {
            0
        };
        self.eip += diff as u32 + 2;
    }

    fn jump_not_sign(&mut self) {
        let diff = if self.eflags.is_sign() {
            0
        } else {
            self.get_sign_code8(1)
        };
        self.eip += diff as u32 + 2;
    }

    fn jump_carry(&mut self) {
        let diff = if self.eflags.is_carry() {
            self.get_sign_code8(1)
        } else {
            0
        };
        self.eip += diff as u32 + 2;
    }

    fn jump_not_carry(&mut self) {
        let diff = if self.eflags.is_carry() {
            0
        } else {
            self.get_sign_code8(1)
        };
        self.eip += diff as u32 + 2;
    }

    fn jump_zero(&mut self) {
        let diff = if self.eflags.is_zero() {
            self.get_sign_code8(1)
        } else {
            0
        };
        self.eip += diff as u32 + 2;
    }
    
    fn jump_not_zero(&mut self) {
        let diff = if self.eflags.is_zero() {
            0
        } else {
            self.get_sign_code8(1)
        };
        self.eip += diff as u32 + 2;
    }

    fn jump_overflow(&mut self) {
        let diff = if self.eflags.is_overflow() {
            self.get_sign_code8(1)
        } else {
            0
        };
        self.eip += diff as u32 + 2;
    }
    
    fn jump_not_overflow(&mut self) {
        let diff = if self.eflags.is_overflow() {
            0
        } else {
            self.get_sign_code8(1)
        };
        self.eip += diff as u32 + 2;
    }

    fn jump_less(&mut self) {
        let diff = if self.eflags.is_sign() != self.eflags.is_overflow() {
            self.get_sign_code8(1)
        } else {
            0
        };
        self.eip += diff as u32 + 2;
    }

    fn jump_less_or_eq(&mut self) {
        let diff = if self.eflags.is_zero() 
                        || self.eflags.is_sign() != self.eflags.is_overflow() {
            self.get_sign_code8(1)
        } else {
            0
        };
        self.eip += diff as u32 + 2;
    }
    
    fn call_rel32(&mut self) {
        let diff = self.get_sign_code32(1);
        self.push32(self.eip + 5);
        self.eip += (diff + 5) as u32;
    }

    fn int(&mut self) {
        let int_index = self.get_code8(1);
        self.eip += 2;

        match int_index {
            0x10    => self.bios_video(),
            n       => {eprintln!("unknown interrupt: 0x{}", n)},
        }
    }

    fn ret(&mut self) {
        self.eip = self.pop32();
    }

    fn leave(&mut self) {
        let ebp = self.get_register32(EBP as usize);
        self.set_register32(ESP as usize, ebp);
        let top = self.pop32();
        self.set_register32(EBP as usize, top);
        self.eip += 1;
    }
}

pub fn instructions(code: u8) -> Option<Instruction> {
    match code {
        0x01 => Some(Emulator::add_rm32_r32),
        0x3B => Some(Emulator::cmp_r32_rm32),
        0x3C => Some(Emulator::cmp_al_imm8),
        0x3D => Some(Emulator::cmp_eax_imm32),
        0x40 ..= 0x47 => Some(Emulator::inc_r32),
        0x50 ..= 0x57 => Some(Emulator::push_r32),
        0x58 ..= 0x5F => Some(Emulator::pop_r32),
        0x68 => Some(Emulator::push_imm32),
        0x6A => Some(Emulator::push_imm8), 
        0x70 => Some(Emulator::jump_overflow),
        0x71 => Some(Emulator::jump_not_overflow),
        0x72 => Some(Emulator::jump_carry),
        0x73 => Some(Emulator::jump_not_carry),
        0x74 => Some(Emulator::jump_zero),
        0x75 => Some(Emulator::jump_not_zero),
        0x78 => Some(Emulator::jump_sign),
        0x79 => Some(Emulator::jump_not_sign),
        0x7C => Some(Emulator::jump_less),
        0x7E => Some(Emulator::jump_less_or_eq),
        0x83 => Some(Emulator::code_83),
        0x88 => Some(Emulator::mov_rm8_r8), 
        0x89 => Some(Emulator::mov_rm32_r32),
        0x8A => Some(Emulator::mov_r8_rm8),
        0x8B => Some(Emulator::mov_r32_rm32),
        0xB0 ..= 0xB7 => Some(Emulator::mov_r8_imm8),
        0xB8 ..= 0xBE => Some(Emulator::mov_r32_imm32),
        0xC3 => Some(Emulator::ret),
        0xC7 => Some(Emulator::mov_rm32_imm32),
        0xC9 => Some(Emulator::leave),
        0xCD => Some(Emulator::int),
        0xE8 => Some(Emulator::call_rel32),
        0xE9 => Some(Emulator::near_jump),
        0xEC => Some(Emulator::in_al_dx),
        0xEE => Some(Emulator::out_dx_al),
        0xEB => Some(Emulator::short_jump),
        0xFF => Some(Emulator::code_ff),
        _ => None,
    }
}

pub fn instructions_with_name(code: u8) -> (Option<Instruction>, &'static str) {
    match code {
        0x01 => (Some(Emulator::add_rm32_r32), "add_rm32_r32"),
        0x3B => (Some(Emulator::cmp_r32_rm32), "cmp_r32_rm32"),
        0x3C => (Some(Emulator::cmp_al_imm8), "cmp_al_imm8"),
        0x3D => (Some(Emulator::cmp_eax_imm32), "cmp_eax_imm32"),
        0x40 ..= 0x47 => (Some(Emulator::inc_r32), "inc_r32"),
        0x50 ..= 0x57 => (Some(Emulator::push_r32), "push_r32"),
        0x58 ..= 0x5F => (Some(Emulator::pop_r32), "pop_r32"),
        0x68 => (Some(Emulator::push_imm32), "push_imm32"),
        0x6A => (Some(Emulator::push_imm8), "push_imm8"),
        0x70 => (Some(Emulator::jump_overflow), "jump_overflow"),
        0x71 => (Some(Emulator::jump_not_overflow), "jump_not_overflow"),
        0x72 => (Some(Emulator::jump_carry), "jump_carry"),
        0x73 => (Some(Emulator::jump_not_carry), "jump_not_carry"),
        0x74 => (Some(Emulator::jump_zero), "jump_zero"),
        0x75 => (Some(Emulator::jump_not_zero), "jump_not_zero"),
        0x78 => (Some(Emulator::jump_sign), "jump_sign"),
        0x79 => (Some(Emulator::jump_not_sign), "jump_not_sign"),
        0x7C => (Some(Emulator::jump_less), "jump_less"),
        0x7E => (Some(Emulator::jump_less_or_eq), "jump_less_or_eq"),
        0x83 => (Some(Emulator::code_83), "code_83"),
        0x88 => (Some(Emulator::mov_rm8_r8), "mov_rm8_r8"),
        0x89 => (Some(Emulator::mov_rm32_r32), "mov_rm32_r32"),
        0x8A => (Some(Emulator::mov_r8_rm8), "mov_r8_rm8"),
        0x8B => (Some(Emulator::mov_r32_rm32), "mov_r32_rm32"),
        0xB0 ..= 0xB7 => (Some(Emulator::mov_r8_imm8), "mov_r8_imm8"),
        0xB8 ..= 0xBE => (Some(Emulator::mov_r32_imm32), "mov_r32_imm32"),
        0xC3 => (Some(Emulator::ret), "ret"),
        0xC7 => (Some(Emulator::mov_rm32_imm32), "mov_rm32_imm32"),
        0xC9 => (Some(Emulator::leave), "leave"),
        0xCD => (Some(Emulator::int), "int"),
        0xE8 => (Some(Emulator::call_rel32), "call_rel32"),
        0xE9 => (Some(Emulator::near_jump), "near_jump"),
        0xEB => (Some(Emulator::short_jump), "short_jump"),
        0xEC => (Some(Emulator::in_al_dx), "in_al_dx"),
        0xEE => (Some(Emulator::out_dx_al), "out_dx_al"),
        0xFF => (Some(Emulator::code_ff), "code_ff"),
        _ => (None, "None"),
    }
}

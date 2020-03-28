use super::*;
use crate::emulator::modrm::*;

impl Emulator {
    fn mov_r32_imm32(&mut self) {
        let reg = self.get_code8(0) - 0xB8;
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
        let rm32 = self.get_rm32(&modrm);

        self.set_rm32(&modrm, rm32);
    }

    fn mov_r32_rm32(&mut self) {
        self.eip += 1;
        let modrm = self.parse_modrm();
        let r32 = self.get_r32(&modrm);
        self.set_r32(&modrm, r32);
    }

    fn add_rm32_r32(&mut self) {
        self.eip += 1;
        let modrm = self.parse_modrm();
        let r32 = self.get_r32(&modrm);
        let rm32 = self.get_rm32(&modrm);
        self.set_rm32(&modrm, rm32 + r32);
    }

    fn sub_rm32_imm8(&mut self, modrm: &ModRM) {
        let rm32 = self.get_rm32(modrm);
        let imm8 = self.get_sign_code8(0) as u32;
        self.eip += 1;
        self.set_rm32(&modrm, rm32 - imm8);
    }

    fn code_83(&mut self) {
        self.eip += 1;
        let modrm = self.parse_modrm();

        match modrm.or.unwrap() {
            5 => self.sub_rm32_imm8(&modrm),
            n => panic!("Not implimented: 83 /{}", n),
        }
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

    fn short_jump(&mut self) {
        let diff = self.get_sign_code8(1);
        self.eip = ((self.eip as i64) + diff as i64 + 2) as u32;
    }

    fn near_jump(&mut self) {
        let diff = self.get_sign_code32(1);
        self.eip = ((self.eip as i64) + diff as i64 + 5) as u32;
    }
}

pub fn instructions(code: u8) -> Option<fn(&mut Emulator)> {
    match code {
        0x01 => Some(Emulator::add_rm32_r32),
        0x83 => Some(Emulator::code_83),
        0x89 => Some(Emulator::mov_rm32_r32),
        0x8B => Some(Emulator::mov_r32_rm32),
        0xB8 ..= 0xBE => Some(Emulator::mov_r32_imm32),
        0xC7 => Some(Emulator::mov_rm32_imm32),
        0xE9 => Some(Emulator::near_jump),
        0xEB => Some(Emulator::short_jump),
        0xFF => Some(Emulator::code_ff),
        _ => None,
    }
}

pub fn instructions_with_name(code: u8) -> (Option<fn(&mut Emulator)>, &'static str) {
    match code {
        0x01 => (Some(Emulator::add_rm32_r32), "add_rm32_r32"),
        0x83 => (Some(Emulator::code_83), "code_83"),
        0x89 => (Some(Emulator::mov_rm32_r32), "mov_rm32_r32"),
        0x8B => (Some(Emulator::mov_r32_rm32), "mov_r32_rm32"),
        0xB8 ..= 0xBE => (Some(Emulator::mov_r32_imm32), "mov_r32_imm32"),
        0xC7 => (Some(Emulator::mov_rm32_imm32), "mov_rm32_imm32"),
        0xE9 => (Some(Emulator::near_jump), "near_jump"),
        0xEB => (Some(Emulator::short_jump), "short_jump"),
        0xFF => (Some(Emulator::code_ff), "code_ff"),
        _ => (None, "None"),
    }
}

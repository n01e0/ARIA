use super::*;

#[derive(Debug, Copy, Clone)]
pub enum OR {
    Opecode(u8),
    RegIndex(u8),
}

use crate::emulator::modrm::OR::*;

impl OR {
    pub fn unwrap(self) -> u8 {
        match self {
            Opecode(ret) => ret,
            RegIndex(ret) => ret,
        }
    }
}

#[derive(Debug, Copy, Clone)]
pub enum Disp {
    Disp8(i8),
    Disp32(u32),
}

use crate::emulator::modrm::Disp::*;

impl Disp {
    pub fn byte(self) -> i8 {
        if let Disp8(ret) = self {
            ret
        } else {
            panic!("{:?} is not a byte.", self);
        }
    }

    pub fn dword(self) -> u32 {
        if let Disp32(ret) = self {
            ret
        } else {
            panic!("{:?} is not 4byte.", self);
        }
    }
}

#[derive(Debug, Copy, Clone)]
pub struct ModRM {
    pub mod_byte: u8,
    pub or: OR,     // u8
    pub rm: u8,
    pub sib: u8,
    pub disp: Disp, // u8 or u32
}

impl Emulator {
    pub fn parse_modrm(&mut self) -> ModRM {
        let mut ret = ModRM {
            mod_byte: 0,
            or: Opecode(0),
            rm: 0,
            sib: 0,
            disp: Disp32(0),
        };

        let code = self.get_code8(0);
        /*  code
         *  +--+--+--+--+--+--+--+--+
         *  | 7| 6| 5| 4| 3| 2| 1| 0|
         *  +-----+--------+--------+
         *  | Mod |   REG  |   R/M  |
         *  +-----+--------+--------+
         */
        ret.mod_byte = (code & 0xC0) >> 6;
        /*  code & 
         *  +--+--+--+--+--+--+--+--+
         *  | 7| 6| 5| 4| 3| 2| 1| 0|
         *  +-----+--------+--------+
         *  |*Mod*| ~~~~~delete~~~~ |
         *  +--+--+--+--+--+--+--+--+
         *  | 1| 1| 0| 0| 0| 0| 0| 0|   => 0xC0
         *  +--+--+--+--+--+--+--+--+
         */
        ret.or = RegIndex( (code & 0x38) >> 3);
        /*  code
         *  +--+--+--+--+--+--+--+--+
         *  | 7| 6| 5| 4| 3| 2| 1| 0|
         *  +-----+--------+--------+
         *  |~del~| *REG*  |~delete~|
         *  +--+--+--+--+--+--+--+--+
         *  | 0| 0| 1| 1| 1| 0| 0| 0|   => 0x38
         *  +--+--+--+--+--+--+--+--+
         */
        ret.rm = code & 0x07;
        /*  code
         *  +--+--+--+--+--+--+--+--+
         *  | 7| 6| 5| 4| 3| 2| 1| 0|
         *  +-----+--------+--------+
         *  |~del~|~delete~|  *R/M* |
         *  +--+--+--+--+--+--+--+--+
         *  | 0| 0| 0| 0| 0| 1| 1| 1|   => 0x07
         *  +--+--+--+--+--+--+--+--+
         */

        self.eip += 1;

        if ret.mod_byte != 0b11 
            && ret.rm == 0b100 {
            ret.sib = self.get_code8(0);
            self.eip += 1;
        }

        if (ret.mod_byte == 0b00 && ret.rm == 0b0101) 
            || ret.mod_byte == 0b0010 {
            ret.disp = Disp32(self.get_sign_code32(0) as u32);
            self.eip += 4;
        } else if ret.mod_byte == 0b01 {
            ret.disp = Disp8(self.get_sign_code8(0));
            self.eip += 1;
        }

        ret
    }

    pub fn get_rm8(&mut self, modrm: &ModRM) -> u8 {
        if modrm.mod_byte == 3 {
            self.get_register8(modrm.or.unwrap() as usize)
        } else {
            let addr = self.calc_memory_address(&modrm);
            self.get_memory8(addr)
        }
    }

    pub fn get_rm32(&mut self, modrm: &ModRM) -> u32 {
        if modrm.mod_byte == 0b11 {
            self.get_register32(modrm.rm as usize)
        } else {
            let addr = self.calc_memory_address(&modrm);
            self.get_memory32(addr)
        }
    }

    pub fn get_r8(&mut self, modrm: &ModRM) -> u8 {
        self.get_register8(modrm.or.unwrap() as usize)
    }

    pub fn get_r32(&mut self, modrm: &ModRM) -> u32 {
        self.get_register32(modrm.or.unwrap() as usize)
    }

    pub fn set_r8(&mut self, modrm: &ModRM, value: u8) {
        self.set_register8(modrm.or.unwrap() as usize, value);
    }

    pub fn set_rm8(&mut self, modrm: &ModRM, value: u8) {
        if modrm.mod_byte == 3 {
            self.set_register8(modrm.rm as usize, value);
        } else {
            let addr = self.calc_memory_address(&modrm);
            self.set_memory8(addr, value as u32);
        }
    }

    pub fn set_rm32(&mut self, modrm: &ModRM, value: u32) {
        if modrm.mod_byte == 0b11 {
            self.set_register32(modrm.rm as usize, value);
        } else {
            let addr = self.calc_memory_address(modrm);
            self.set_memory32(addr, value);
        }
    }

    pub fn set_r32(&mut self, modrm: &ModRM, value: u32) {
        self.set_register32(modrm.or.unwrap() as usize, value);
    }
    
    pub fn calc_memory_address(&self, modrm: &ModRM) -> u32 {
        match modrm.mod_byte {
            0 => {
                match modrm.rm {
                    4 => modrm_not_impl(*modrm),
                    5 => modrm.disp.dword(),
                    _ => self.get_register32(modrm.rm as usize),
                }
            },
            1 => {
                if modrm.rm == 4 {
                    modrm_not_impl(*modrm)
                } else {
                    self.get_register32(modrm.rm as usize) 
                        + modrm.disp.byte() as u32
                }
            },
            2 => {
                if modrm.rm == 4 {
                    modrm_not_impl(*modrm)
                } else {
                    self.get_register32(modrm.rm as usize) + modrm.disp.dword()
                }
            },
            _ => modrm_not_impl(*modrm),
        }
    }
}

fn modrm_not_impl(modrm: ModRM) -> ! {
    panic!("Not implemented ModRM mod = {}, rm = {}", modrm.mod_byte, modrm.rm);
}

use std::fmt;
pub mod instruction;
pub mod modrm;
pub mod io;


pub struct RunFlags {
    pub verbose: bool,
    pub quiet: bool
}

#[derive(Debug)] 
pub enum Register {
    EAX,
    ECX,
    EDX,
    EBX,
    ESP,
    EBP,
    ESI,
    EDI,
    RegistersCount,
} 

#[allow(dead_code)]
pub enum RegisterLow {
    AL,
    CL,
    DL,
    BL,
}

#[allow(dead_code)]
pub enum RegisterHigh {
    AH = RegisterLow::AL as isize + 4,
    CH = RegisterLow::CL as isize + 4,
    DH = RegisterLow::DL as isize + 4,
    BL = RegisterLow::BL as isize + 4,
}

use self::Register::*;
impl fmt::Display for Register {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            EAX => write!(f, "EAX"),
            ECX => write!(f, "ECX"),
            EDX => write!(f, "EDX"),
            EBX => write!(f, "EBX"),
            ESP => write!(f, "ESP"),
            EBP => write!(f, "EBP"),
            ESI => write!(f, "ESI"),
            EDI => write!(f, "EDI"),
            RegistersCount => write!(f, "RegistersCount"),
        }
    }
}

#[derive(Debug, Clone)]
pub struct Eflags {
    pub raw: u32
}

impl Eflags {
    pub fn set_carry(&mut self, is_carry: bool) {
        let carry = 1;
        if is_carry {
            self.raw |= carry;
        } else {
            self.raw &= !carry;
        }
    }

    pub fn set_zero(&mut self, is_zero: bool) {
        let zero = 1 << 6;
        if is_zero {
            self.raw |= zero;
        } else {
            self.raw &= !zero;
        }
    }

    pub fn set_sign(&mut self, is_sign: bool) {
        let sign = 1 << 7;
        if is_sign {
            self.raw |= sign;
        } else {
            self.raw &= !sign;
        }
    }

    pub fn set_overflow(&mut self, is_overflow: bool) {
        let overflow = 1 << 11;
        if is_overflow {
            self.raw |= overflow;
        } else {
            self.raw &= !overflow;
        }
    }

    pub fn is_carry(&self) -> bool {
        self.raw & 1 != 0
    }

    pub fn is_zero(&self) -> bool {
        self.raw & (1 << 6) != 0
    }

    pub fn is_sign(&self) -> bool {
        self.raw & (1 << 7) != 0
    }

    pub fn is_overflow(&self) -> bool {
        self.raw & (1 << 11) != 0
    }
}

#[derive(Debug, Clone)]
pub struct Emulator {
    pub registers: [u32; Register::RegistersCount as usize],
    pub eflags: Eflags,
    pub memory: Vec<u8>,
    pub eip: u32,
}

const ORG: usize = 0x7C00;

impl fmt::Display for Emulator {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        use self::Register::*;
        let emu = format!("\
        Emulator\n\
        \tregisters\n\
        \t\tEAX: 0x{EAX:08X}\n\
        \t\tECX: 0x{ECX:08X}\n\
        \t\tEDX: 0x{EDX:08X}\n\
        \t\tEBX: 0x{EBX:08X}\n\
        \t\tESP: 0x{ESP:08X}\n\
        \t\tEBP: 0x{EBP:08X}\n\
        \t\tESI: 0x{ESI:08X}\n\
        \t\tEDI: 0x{EDI:08X}\n\
        \teflags:  0x{eflags:08X}\n\
        \tmemory:  {memory}\n\
        \teip:     0x{eip:08X}\n",

        EAX=self.registers[EAX as usize],
        ECX=self.registers[ECX as usize],
        EDX=self.registers[EDX as usize],
        EBX=self.registers[EBX as usize],
        ESP=self.registers[ESP as usize],
        EBP=self.registers[EBP as usize],
        ESI=self.registers[ESI as usize],
        EDI=self.registers[EDI as usize],
        eflags=self.eflags.raw,
        memory="<Ommited>",
        eip=self.eip);

        write!(f, "{}", emu)
    }
}

impl Emulator {
    pub fn new(size: usize, eip: u32, esp: u32) -> Emulator {
        Emulator {
            registers: [
                /* EAX */ 0,
                /* ECX */ 0,
                /* EDX */ 0,
                /* EBX */ 0,
                /* ESP */ esp,
                /* EBP */ 0,
                /* ESI */ 0,
                /* EDI */ 0
            ],
            eflags: Eflags { raw: 0 },
            memory: Vec::with_capacity(ORG + size),
            eip: eip,
        }
    }

    pub fn load(&mut self, file: &mut std::fs::File) {
        use std::io::{Read};
        let mut bios = [0; ORG].to_vec();
        self.memory.append(&mut bios);

        let mut buf = Vec::new();
        file.read_to_end(&mut buf).expect("Can't read file");
        self.raw_load(&mut buf);
    }

    pub fn raw_load(&mut self, bytes: &mut Vec<u8>) {
        self.memory.append(bytes);
    }

    pub fn run(&self, flag: RunFlags) {
        if flag.quiet {
            self.quiet();
        } else if flag.verbose {
            self.verbose();
        } else {
            self.default();
        }
    }

    fn quiet(&self) {
         let mut emu = self.to_owned();  
         while (emu.eip as usize) < (emu.memory.capacity()) {
             let code = emu.get_code8(0);
             
             if let Some(inst) = instruction::instructions(code) {
                 inst(&mut emu);
             } else {
                 eprintln!("Not implimented: {:X}", code);
                 break;
             }
         
             if emu.eip == 0x00 {
                 break;
             }
         }
    }

    fn verbose(&self) {
        let mut emu = self.to_owned();
        while (emu.eip as usize) < (emu.memory.capacity()) {
            let code = emu.get_code8(0);

            println!("EIP = {:X}, Code = {:X}", emu.eip, code);
            
            let iwn = instruction::instructions_with_name(code);

            if let Some(inst) = iwn.0 {
                inst(&mut emu);
                println!("{}", iwn.1);
                println!("{}", emu);
            } else {
                eprintln!("Not implimented: {:X}", code);
                break;
            }
        
            if emu.eip == 0x00 {
                println!("\nEnd of program.\n");
                break;
            }
        }

        emu.dump_verbose();

    }

    fn default(&self) {
        let mut emu = self.to_owned();  
        while (emu.eip as usize) < (emu.memory.capacity()) {
            let code = emu.get_code8(0);
            println!("EIP = {:X}, Code = {:X}", emu.eip, code);
            
            if let Some(inst) = instruction::instructions(code) {
                inst(&mut emu);
            } else {
                eprintln!("Not implimented: {:X}", code);
                break;
            }
        
            if emu.eip == 0x00 {
                println!("\nEnd of program.\n");
                break;
            }
        }

        emu.dump();
    }
    /*
     * Emulator instructions
     */

    pub fn dump(&self) {
        eprintln!("{}", self);
    }

    pub fn dump_verbose(&self) {
        eprintln!("{:#?}", self);
    }

    pub fn set_memory8(&mut self, addr: u32, value: u32) {
        self.memory[addr as usize] = (value & 0xFF) as u8;
    }
    
    pub fn get_memory8(&self, addr: u32) -> u8 {
        self.memory[addr as usize]
    }
    
    pub fn set_memory32(&mut self, addr: u32, value: u32) {
        for i in 0..4 {
            self.set_memory8(addr + i, value >> (i * 8));
        }
    }
    
    pub fn get_memory32(&self, addr: u32) -> u32 {
        let mut ret: u32 = 0;
        for i in 0..4 {
            ret |= (self.get_memory8(addr + i) as u32) << (i * 8);
        }
    
        ret
    }

    pub fn get_code8(&self, index: u32) -> u8 {
        self.memory[(self.eip + index) as usize] as u8
    }

    pub fn get_sign_code8(&self, index: u32) -> i8 {
        self.memory[(self.eip + index) as usize] as i8
    }

    pub fn get_code32(&self, index: u32) -> u32 {
        let mut ret: u32 = 0;
        for i in 0..4 {
            ret |= (self.get_code8(index + i) as u32) << (i * 8);
        }

        ret
    }

    pub fn get_sign_code32(&self, index: u32) -> i32 {
        self.get_code32(index) as i32
    }

    pub fn set_register8(&mut self, index: usize, value: u8) {
        if index < 4 {
            let r = self.registers[index] & 0xFFFFFF00;
            self.registers[index] = r | value as u32;
        } else {
            let r = self.registers[index - 4] & 0xFFFF00FF;
            self.registers[index - 4] = r | ((value as u32) << 8);
        }
    }

    pub fn set_register32(&mut self, index: usize, value: u32) {
        self.registers[index] = value;
    }

    pub fn get_register8(&self, index: usize) -> u8 {
        if index < 4 {
            (self.registers[index] & 0xFF) as u8
        } else {
            ((self.registers[index - 4] >> 8) & 0xFF) as u8
        }
    }
    
    pub fn get_register32(&self, index: usize) -> u32 {
        self.registers[index]
    }

    pub fn push32(&mut self, value: u32) {
        let addr = self.get_register32(ESP as usize) - 4;
        self.set_register32(ESP as usize, addr);
        self.set_memory32(addr, value);
    }

    pub fn pop32(&mut self) -> u32 {
        let addr = self.get_register32(ESP as usize);
        let ret = self.get_memory32(addr);
        self.set_register32(ESP as usize, addr + 4);
        ret
    }

    pub fn update_eflags_sub(&mut self, v1: u32, v2: u32, result: u64) {
        let sign1 = v1 >> 31;
        let sign2 = v2 >> 31;
        let signr = (result >> 31) as u32 & 1;

        self.eflags.set_carry(result >> 32 != 0);
        self.eflags.set_zero(result == 0);
        self.eflags.set_sign(signr != 0);
        self.eflags.set_overflow(sign1 != sign2 && sign1 != signr);
    }
}


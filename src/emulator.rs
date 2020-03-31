use std::fmt;
pub mod instruction;
pub mod modrm;

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

#[derive(Debug)]
pub struct Emulator {
    pub registers: [u32; Register::RegistersCount as usize],
    pub eflags: u32,
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
        eflags=self.eflags,
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
            eflags: 0,
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
        self.memory.append(&mut buf);
    }

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

    pub fn set_register32(&mut self, index: usize, value: u32) {
        self.registers[index] = value;
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
}


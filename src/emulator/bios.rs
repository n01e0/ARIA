use super::*;
use crate::emulator::io;
use crate::emulator::RegisterHigh::*;
use crate::emulator::RegisterLow::*;

const BIOS_TO_TERMINAL: [i32; 8] = [30, 34, 32, 36, 31, 35, 33, 37];

impl<I: Read + Clone + Copy, O: Write + Clone + Copy> Emulator<I, O> {
    fn bios_video_teletype(&mut self) {
        let color: u8 = self.get_register8(BL as usize) & 0x0F;
        let ch: u8 = self.get_register8(AL as usize);

        let terminal_color = BIOS_TO_TERMINAL[(color & 0x07) as usize];
        let bright = if (color & 0x08) > 0 { 1 } else { 0 };
        let s = format!("\x1b[{};{}m{}\x1b[0m", bright, terminal_color, ch as char);
        s.bytes().for_each(move |c| io::io_out8(self.output, 0x03F8, c));
    }

    pub fn bios_video(&mut self) {
        match self.get_register8(AH as usize) {
            0x0E => self.bios_video_teletype(),
            n => eprintln!("not implemented BIOS video function 0x{:x}", n),
        }
    }
}

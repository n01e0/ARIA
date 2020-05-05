use std::io::{self, Write};

pub fn io_in8(addr: u16) -> u8 {
    match addr {
        0x03F8 => getchar(),
        _ => 0,
    }
}

pub fn io_out8(addr: u16, value: u8) {
    match addr {
        0x03F8 => putchar(value),
        _ => (),
    }
}

fn getchar() -> u8 {
    let mut buf = String::new();
    io::stdin().read_line(&mut buf).expect("Can't read line");
    buf.as_bytes()[0]
}

fn putchar(value: u8) {
    print!("{}", value as char);
    io::stdout().flush().expect("Can't flush stdout");
}

use std::io::{self, Read, Write};

pub fn io_in8<I: Read>(input: Option<I>, addr: u16) -> u8 {
    match addr {
        0x03F8 => getchar(input),
        _ => 0,
    }
}

pub fn io_out8<O: Write>(output: Option<O>, addr: u16, value: u8) {
    match addr {
        0x03F8 => putchar(output, value),
        _ => (),
    }
}

fn getchar<I: Read>(input: Option<I>) -> u8 {
    if let Some(mut mem) = input {
        let mut buf = Vec::new();
        mem.read_to_end(&mut buf).expect("Can't read line");
        buf[0]
    } else {
        let mut buf = String::new();
        io::stdin().read_line(&mut buf).expect("Can't read line");
        buf.as_bytes()[0]
    }
}

fn putchar<O: Write>(output: Option<O>, value: u8) {
    if let Some(mut mem) = output {
        write!(mem, "{}", value as char).expect("Can't write to memory");
    } else {
        print!("{}", value as char);
        io::stdout().flush().expect("Can't flush stdout");
    }
}

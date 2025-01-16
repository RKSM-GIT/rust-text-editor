use crossterm::terminal::{disable_raw_mode, enable_raw_mode};
use std::io::{self, Read};

fn main() {
    enable_raw_mode().unwrap();

    for b in io::stdin().bytes() {
        let b = b.unwrap();
        let c = b as char;

        if c.is_control() {
            println!("BYTES: {0:08b}, ASCII: {0:#03}\r", b);
        } else {
            println!("BYTES: {0:08b}, ASCII: {0:#03}, CHAR: {1:#?}\r", b, c);
        }

        if c == 'q' {
            disable_raw_mode().unwrap();
            break;
        }
    }
}

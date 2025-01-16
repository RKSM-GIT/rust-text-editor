use crossterm::terminal::{disable_raw_mode, enable_raw_mode};
use std::io::{self, Read};

pub struct Editor {}

impl Editor {
    pub fn default() -> Self {
        Editor {}
    }

    pub fn run(&self) {
        enable_raw_mode().unwrap();

        for b in io::stdin().bytes() {
            match b {
                Ok(b) => {
                    let c = b as char;

                    if c.is_control() {
                        println!("BYTES: {0:08b}, ASCII: {0:#03}\r", b);
                    } else {
                        println!("BYTES: {0:08b}, ASCII: {0:#03}, CHAR: {1:#?}\r", b, c);
                    }

                    if c == 'q' {
                        break;
                    }
                }
                Err(e) => {
                    println!("Error occurred: {}", e);
                }
            }
        }

        disable_raw_mode().unwrap();
    }
}

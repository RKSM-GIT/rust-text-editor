use crossterm::{
    event::{read, Event::Key, KeyCode::Char},
    terminal::{disable_raw_mode, enable_raw_mode},
};

pub struct Editor {}

impl Editor {
    pub fn run() {
        enable_raw_mode().unwrap();

        loop {
            match read() {
                Ok(Key(event)) => {
                    println!("{event:?} \r");

                    if let Char(c) = event.code {
                        if c == 'q' {
                            break;
                        }
                    }
                }
                Err(err) => println!("Error: {err}"),
                _ => {}
            }
        }

        disable_raw_mode().unwrap();
    }
}

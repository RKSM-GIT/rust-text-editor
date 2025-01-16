mod terminal;
use crossterm::event::{read, Event, Event::Key, KeyCode::Char, KeyEvent, KeyModifiers};
use std::io::Error;
use terminal::{Position, Size, Terminal};

pub struct Editor {
    should_quit: bool,
}

impl Editor {
    pub const fn default() -> Self {
        Self { should_quit: false }
    }

    pub fn run(&mut self) {
        Terminal::initialize().unwrap();
        let result = self.repl();
        Terminal::terminate().unwrap();
        result.unwrap();
    }

    fn repl(&mut self) -> Result<(), Error> {
        while !self.should_quit {
            let event = read()?;
            self.evaluate_event(&event);
            self.refresh_screen()?;
        }

        Ok(())
    }

    fn evaluate_event(&mut self, event: &Event) {
        if let Key(KeyEvent {
            code, modifiers, ..
        }) = event
        {
            match code {
                Char('q') if *modifiers == KeyModifiers::CONTROL => {
                    self.should_quit = true;
                }
                _ => (),
            }
        }
    }

    fn refresh_screen(&self) -> Result<(), Error> {
        Terminal::hide_cursor()?;

        if self.should_quit {
            Terminal::clear_screen()?;
            Terminal::print("Goodbye.\r\n")?;
        } else {
            Self::draw_rows()?;
            Terminal::move_cursor_to(Position { row: 0, col: 0 })?;
        }

        Terminal::show_cursor()?;
        Terminal::execute()?;
        Ok(())
    }

    pub fn draw_rows() -> Result<(), Error> {
        let Size { height, width } = Terminal::size()?;

        for row in 0..height {
            Terminal::clear_line()?;
            if row == height / 3 {
                Self::display_welocme_message(row, width)?;
            } else {
                Self::display_empty_row(row)?;
            }
        }

        Ok(())
    }

    fn display_empty_row(row: u16) -> Result<(), Error> {
        Terminal::move_cursor_to(Position { row, col: 0 })?;
        Terminal::print("~")
    }

    #[allow(clippy::cast_possible_truncation)]
    fn display_welocme_message(row: u16, width: u16) -> Result<(), Error> {
        let hecto_info = "Hecto v0.1.0";
        let info_len = hecto_info.len() as u16;
        let center_pos = Position {
            row,
            col: width / 2 - (info_len - 1) / 2,
        };
        Terminal::move_cursor_to(center_pos)?;
        Terminal::print(hecto_info)
    }
}

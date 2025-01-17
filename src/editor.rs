mod buffer;
mod terminal;
mod view;

use crossterm::event::{
    read,
    Event::{self, Key},
    KeyCode, KeyEvent, KeyEventKind, KeyModifiers,
};
use std::io::Error;
use terminal::{Position, Size, Terminal};
use view::View;

pub struct Editor {
    should_quit: bool,
    caret_location: Position,
    view: View,
}

impl Editor {
    pub fn default() -> Self {
        Self {
            should_quit: false,
            caret_location: Position { row: 0, col: 0 },
            view: View::default(),
        }
    }

    pub fn run(&mut self) {
        Terminal::initialize().unwrap();
        self.handle_args();
        let result = self.repl();
        Terminal::terminate().unwrap();
        result.unwrap();
    }

    fn handle_args(&mut self) {
        let args: Vec<String> = std::env::args().collect();

        if let Some(file_name) = args.get(1) {
            self.view.load(file_name);
        }
    }

    fn repl(&mut self) -> Result<(), Error> {
        while !self.should_quit {
            let event = read()?;
            self.evaluate_event(&event)?;
            self.refresh_screen()?;
        }

        Ok(())
    }

    fn evaluate_event(&mut self, event: &Event) -> Result<(), Error> {
        if let Key(KeyEvent {
            code,
            modifiers,
            kind,
            ..
        }) = event
        {
            match code {
                KeyCode::Char('q') if *modifiers == KeyModifiers::CONTROL => {
                    self.should_quit = true;
                }
                KeyCode::Up
                | KeyCode::Right
                | KeyCode::Down
                | KeyCode::Left
                | KeyCode::End
                | KeyCode::Home
                | KeyCode::PageDown
                | KeyCode::PageUp
                    if *kind == KeyEventKind::Press =>
                {
                    self.move_caret(*code)?;
                }
                _ => (),
            }
        }

        Ok(())
    }

    fn refresh_screen(&self) -> Result<(), Error> {
        Terminal::hide_caret()?;
        Terminal::move_caret_to(Position::default())?;

        if self.should_quit {
            Terminal::clear_screen()?;
            Terminal::print("Goodbye.\r\n")?;
        } else {
            self.view.render()?;
            Terminal::move_caret_to(self.caret_location)?;
        }

        Terminal::show_caret()?;
        Terminal::execute()?;
        Ok(())
    }

    fn move_caret(&mut self, key_code: KeyCode) -> Result<(), Error> {
        let Size { height, width } = Terminal::size()?;

        match key_code {
            KeyCode::Left => {
                self.caret_location.col = self.caret_location.col.saturating_sub(1);
            }
            KeyCode::Right => {
                self.caret_location.col = self.caret_location.col.saturating_add(1).min(width - 1);
            }
            KeyCode::Up => {
                self.caret_location.row = self.caret_location.row.saturating_sub(1);
            }
            KeyCode::Down => {
                self.caret_location.row = self.caret_location.row.saturating_add(1).min(height - 1);
            }
            KeyCode::Home => {
                self.caret_location.col = 0;
            }
            KeyCode::End => {
                self.caret_location.col = width - 1;
            }
            KeyCode::PageUp => {
                self.caret_location.row = 0;
            }
            KeyCode::PageDown => {
                self.caret_location.row = height - 1;
            }
            _ => {}
        }

        Terminal::move_caret_to(self.caret_location)?;
        Ok(())
    }
}

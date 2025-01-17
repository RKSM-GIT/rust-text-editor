mod buffer;
mod terminal;
mod view;

use crossterm::event::{read, Event, KeyCode, KeyEvent, KeyEventKind, KeyModifiers};
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
            caret_location: Position::default(),
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
            self.evaluate_event(event)?;
            self.refresh_screen()?;
        }

        Ok(())
    }

    fn evaluate_event(&mut self, event: Event) -> Result<(), Error> {
        match event {
            Event::Key(KeyEvent {
                code,
                modifiers,
                kind: KeyEventKind::Press,
                ..
            }) => match (code, modifiers) {
                (KeyCode::Char('q'), KeyModifiers::CONTROL) => {
                    self.should_quit = true;
                }
                (
                    KeyCode::Up
                    | KeyCode::Right
                    | KeyCode::Down
                    | KeyCode::Left
                    | KeyCode::End
                    | KeyCode::Home
                    | KeyCode::PageDown
                    | KeyCode::PageUp,
                    _,
                ) => {
                    self.move_caret(code)?;
                }
                _ => (),
            },
            Event::Resize(width, height) => {
                self.view.resize(Size {
                    height: height as usize,
                    width: width as usize,
                });
            }
            _ => {}
        }

        Ok(())
    }

    fn refresh_screen(&mut self) -> Result<(), Error> {
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
        let Position { mut row, mut col } = self.caret_location;

        match key_code {
            KeyCode::Left => {
                col = col.saturating_sub(1);
            }
            KeyCode::Right => {
                col = col.saturating_add(1).min(width.saturating_sub(1));
            }
            KeyCode::Up => {
                row = row.saturating_sub(1);
            }
            KeyCode::Down => {
                row = row.saturating_add(1).min(height.saturating_sub(1));
            }
            KeyCode::Home => {
                col = 0;
            }
            KeyCode::End => {
                col = width.saturating_sub(1);
            }
            KeyCode::PageUp => {
                row = 0;
            }
            KeyCode::PageDown => {
                row = height.saturating_sub(1);
            }
            _ => {}
        }

        self.caret_location = Position { row, col };
        Terminal::move_caret_to(self.caret_location)?;
        Ok(())
    }
}

mod buffer;
mod terminal;
mod view;

use crossterm::event::{self, Event, KeyCode, KeyEvent, KeyEventKind, KeyModifiers};
use std::io::Error;
use std::panic;
use terminal::{Position, Size, Terminal};
use view::View;

pub struct Editor {
    should_quit: bool,
    caret_location: Position,
    view: View,
}

impl Editor {
    pub fn new() -> Result<Self, Error> {
        Self::set_panic_printing();
        Terminal::initialize()?;

        let mut view = View::default();
        let args: Vec<String> = std::env::args().collect();
        if let Some(file_name) = args.get(1) {
            view.load(file_name);
        }

        Ok(Self {
            should_quit: false,
            caret_location: Position::default(),
            view,
        })
    }

    fn set_panic_printing() {
        //Retrieve the current hook, which by default does some nice printing of the panic
        let current_hook = panic::take_hook();

        // Define a new closure that takes a reference to the PanicInfo.
        // Move any external variables needed within the closure here.
        // Place the closure into a Box and set it as the new panic hook.
        panic::set_hook(Box::new(move |panic_info| {
            let _ = Terminal::terminate();

            // Our custom panic hook logic goes here
            // Execute the original hook to retain default panic output behavior.
            current_hook(panic_info);
        }));
    }

    pub fn run(&mut self) {
        self.repl();
        Terminal::terminate().unwrap();
    }

    fn repl(&mut self) {
        loop {
            self.refresh_screen();
            if self.should_quit {
                break;
            }

            match event::read() {
                Ok(event) => self.evaluate_event(event),
                Err(err) => {
                    #[cfg(debug_assertions)]
                    {
                        panic!("Could not read event: {err:?}");
                    }
                }
            }
        }
    }

    fn move_caret(&mut self, key_code: KeyCode) {
        let Size { height, width } = Terminal::size().unwrap_or_default();
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
    }

    fn evaluate_event(&mut self, event: Event) {
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
                    self.move_caret(code);
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
    }

    fn refresh_screen(&mut self) {
        let _ = Terminal::hide_caret();

        self.view.render();
        let _ = Terminal::move_caret_to(self.caret_location);

        let _ = Terminal::show_caret();
        let _ = Terminal::execute();
    }
}

impl Drop for Editor {
    fn drop(&mut self) {
        let _ = Terminal::terminate();
        if self.should_quit {
            let _ = Terminal::print("Goodbye.\r\n");
        }
    }
}

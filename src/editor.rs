mod editorcommand;
mod position;
mod terminal;
mod view;

use crossterm::event::{self, Event, KeyEvent, KeyEventKind};
use editorcommand::EditorCommand;
use std::io::Error;
use std::panic;
use terminal::Terminal;
use view::View;
use simplelog::{WriteLogger, LevelFilter, Config};
use std::fs::File;

pub struct Editor {
    should_quit: bool,
    view: View,
}

impl Editor {
    pub fn new() -> Result<Self, Error> {
        Self::initialize_logger();
        Self::set_panic_printing();
        Terminal::initialize()?;

        let mut view = View::default();
        let args: Vec<String> = std::env::args().collect();
        if let Some(file_name) = args.get(1) {
            view.load(file_name);
        }

        Ok(Self {
            should_quit: false,
            view,
        })
    }

    fn initialize_logger() {
        WriteLogger::init(
            LevelFilter::Debug,
            Config::default(),
            File::create("editor.log").unwrap(),
        ).unwrap();
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

    pub fn repl(&mut self) {
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

    fn evaluate_event(&mut self, event: Event) {
        let should_process = match event {
            Event::Key(KeyEvent { kind, .. }) => kind == KeyEventKind::Press,
            Event::Resize(_, _) => true,
            _ => false,
        };

        if !should_process {
            return;
        }

        if let Ok(command) = EditorCommand::try_from(event) {
            if matches!(command, EditorCommand::Quit) {
                self.should_quit = true;
            } else {
                self.view.handle_command(command);
            }
        }
    }

    fn refresh_screen(&mut self) {
        let _ = Terminal::hide_caret();
        self.view.render();
        let _ = Terminal::move_caret_to(self.view.caret_position());
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

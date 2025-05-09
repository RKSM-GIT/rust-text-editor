mod editorcommand;
mod position;
mod terminal;
mod view;
mod statusbar;
mod documentstatus;
mod file_info;
mod messagebar;
mod uicomponent;

use crossterm::event::{self, Event, KeyEvent, KeyEventKind};
use editorcommand::EditorCommand;
use messagebar::MessageBar;
use statusbar::StatusBar;
use uicomponent::UiComponent;
use std::io::Error;
use std::panic;
use terminal::{Size, Terminal};
use view::View;
use simplelog::{WriteLogger, LevelFilter, Config};
use std::fs::File;
use log::error;

pub const NAME: &str = env!("CARGO_PKG_NAME");
pub const VERSION: &str = env!("CARGO_PKG_VERSION");

#[derive(Default)]
pub struct Editor {
    should_quit: bool,
    view: View,
    status_bar: StatusBar,
    message_bar: MessageBar,
    terminal_size: Size,
    title: String,
}

impl Editor {
    pub fn new() -> Result<Self, Error> {
        Self::initialize_logger();
        Self::set_panic_printing();
        Terminal::initialize()?;

        let mut editor = Self::default();
        let size = Terminal::size()?;
        editor.resize(size);

        let args: Vec<String> = std::env::args().collect();
        if let Some(file_name) = args.get(1) {
            editor.view.load(file_name);
        }

        editor.message_bar.update_message("HELP: Ctrl-S = save | Ctrl-Q = quit".to_string());
        editor.refresh_status();

        Ok(editor)
    }

    fn resize(&mut self, size: Size) {
        self.terminal_size = size;
        self.view.resize(Size {
            height: size.height.saturating_sub(2),
            width: size.width,
        });

        self.message_bar.resize(Size {
            height: 1,
            width: size.width,
        });

        self.status_bar.resize(Size {
            height: 1,
            width: size.width,
        });
    }

    pub fn refresh_status(&mut self) {
        let status = self.view.get_status();
        let title = format!("{} - {NAME}", status.file_name);
        self.status_bar.update_status(status);

        if title != self.title && matches!(Terminal::set_title(&title), Ok(())) {
            self.title = title;
        }
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
                    error!("Failed to read event: {err:?}");
                    #[cfg(debug_assertions)]
                    {
                        panic!("Could not read event: {err:?}");
                    }
                }
            }

            self.refresh_status();
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
            } else if let EditorCommand::Resize(size) = command {
                self.resize(size);
            } else {
                self.view.handle_command(command);
                if let EditorCommand::Resize(size) = command {
                    self.status_bar.resize(size);
                }
            }
        }
    }

    fn refresh_screen(&mut self) {
        if self.terminal_size.height == 0 || self.terminal_size.width == 0 {
            return;
        }
        let _ = Terminal::hide_caret();

        self.message_bar
            .render(self.terminal_size.height.saturating_sub(1));


        if self.terminal_size.height > 1 {
            self.status_bar
                .render(self.terminal_size.height.saturating_sub(2));
        }

        if self.terminal_size.height > 2 {
            self.view.render(0);
        }


        if let Err(e) = Terminal::move_caret_to(self.view.caret_position()) {
            error!("Failed to move caret: {e:?}");
        }
        if let Err(e) = Terminal::show_caret() {
            error!("Failed to show caret: {e:?}");
        }
        if let Err(e) = Terminal::execute() {
            error!("Failed to execute terminal commands: {e:?}");
        }
    }
}

impl Drop for Editor {
    fn drop(&mut self) {
        if let Err(e) = Terminal::terminate() {
            error!("Failed to terminate terminal: {e:?}");
        }
        if self.should_quit {
            if let Err(e) = Terminal::print("Goodbye.\r\n") {
                error!("Failed to print goodbye message: {e:?}");
            }
        }
    }
}

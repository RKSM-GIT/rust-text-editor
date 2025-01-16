use crossterm::{
    event::{read, Event::Key, KeyCode::Char, KeyEvent, KeyModifiers},
    terminal::{disable_raw_mode, enable_raw_mode},
};

pub struct Editor {
    should_quit: bool,
}

impl Editor {
    pub fn default() -> Self {
        Editor { should_quit: false }
    }

    pub fn run(&mut self) {
        if let Err(err) = self.repl() {
            panic!("Error: {err:?}");
        }

        println!("Goodbye!");
    }

    fn repl(&mut self) -> Result<(), std::io::Error> {
        enable_raw_mode()?;

        while !self.should_quit {
            if let Key(KeyEvent {
                code,
                modifiers,
                kind,
                state,
            }) = read()?
            {
                println!(
                    "Code: {code:?}, Modifiers: {modifiers:?}, Kind: {kind:?}, State: {state:?} \r"
                );

                if let Char('q') = code {
                    if modifiers == KeyModifiers::CONTROL {
                        self.should_quit = true;
                        break;
                    }
                }
            }
        }

        disable_raw_mode()?;
        Ok(())
    }
}

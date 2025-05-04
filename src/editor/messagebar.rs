use std::io::Error;

use super::{terminal::{Size, Terminal}, uicomponent::UiComponent};

struct MessageBar {
    curr_message: String,
    needs_redraw: bool,
}

impl MessageBar {
    pub fn update_message(&mut self, message: String) {
        if self.curr_message != message {
            self.curr_message = message;
            self.mark_redraw(true);
        }
    }
}

impl UiComponent for MessageBar {
    fn mark_redraw(&mut self, value: bool) {
        self.needs_redraw = value;
    }

    fn needs_redraw(&self) -> bool {
        self.needs_redraw
    }

    fn draw(&mut self, origin_y: usize) -> Result<(), Error> {
        Terminal::print_row(origin_y, &self.curr_message)
    }

    fn set_size(&mut self, _: Size) {}
}
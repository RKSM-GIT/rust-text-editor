mod message;

use std::{io::Error, time::Instant};

use message::Message;

use super::{terminal::{Size, Terminal}, uicomponent::UiComponent};

#[derive(Default, Debug)]
pub struct MessageBar {
    curr_message: Message,
    needs_redraw: bool,
    has_cleared_after_expiry: bool
}

impl MessageBar {
    pub fn update_message(&mut self, message: String) {
        self.curr_message.time = Instant::now();
        self.curr_message.text = message;

        self.has_cleared_after_expiry = false;
        self.set_needs_redraw(true);
    }
}

impl UiComponent for MessageBar {
    fn set_needs_redraw(&mut self, value: bool) {
        self.needs_redraw = value;
    }

    fn needs_redraw(&self) -> bool {
        self.needs_redraw || (!self.has_cleared_after_expiry && self.curr_message.is_expired())
    }

    fn draw(&mut self, origin_y: usize) -> Result<(), Error> {
        if self.curr_message.is_expired() {
            self.has_cleared_after_expiry = true;
        }

        let msg = if self.curr_message.is_expired() {
            "" 
        } else {
            &self.curr_message.text
        };

        Terminal::print_row(origin_y, msg)
    }

    fn set_size(&mut self, _: Size) {}
}
use super::{
    documentstatus::DocumentStatus, size::Size, terminal::Terminal, uicomponent::UiComponent
};

#[derive(Default)]
pub struct StatusBar {
    curr_status: DocumentStatus,
    needs_redraw: bool,
    size: Size
}

impl StatusBar {
    pub fn update_status(&mut self, new_status: DocumentStatus) {
        if new_status != self.curr_status {
            self.curr_status = new_status;
            self.set_needs_redraw(true);
        }
    }
}

impl UiComponent for StatusBar {
    fn set_needs_redraw(&mut self, value: bool) {
        self.needs_redraw = value;
    }

    fn needs_redraw(&self) -> bool {
        self.needs_redraw
    }

    fn draw(&mut self, origin_y: usize) -> Result<(), std::io::Error> {
        // Assemble the first part of the status bar
        let line_count = self.curr_status.get_line_count_string();
        let modified_indicator = self.curr_status.get_modified_indicator_string();

        let beginning = format!(
            "{} - {line_count} {modified_indicator}",
            self.curr_status.file_name
        );

        // Assemble the whole status bar, with the position indicator at the back
        let position_indicator = self.curr_status.get_position_indicator_string();
        let remainder_len = self.size.width.saturating_sub(beginning.len());
        let status = format!("{beginning}{position_indicator:>remainder_len$}");

        // Only print out the status if it fits. 
        // Otherwise write out an empty string to ensure the row is cleared.
        let to_print = if status.len() <= self.size.width {
            status
        } else {
            String::new()
        };


        Terminal::print_inverted_row(origin_y, &to_print)?;

        Ok(())
    }

    fn set_size(&mut self, size: Size) {
        self.size = size;
    }
}
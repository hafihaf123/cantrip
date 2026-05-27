use ratatui::buffer::Buffer;
use ratatui::layout::{Constraint, Rect};
use ratatui::widgets::{Block, Clear, Paragraph, Widget};

pub struct ErrorPopup<'a> {
    error_message: &'a str,
}

impl<'a> ErrorPopup<'a> {
    pub fn new(error_message: &'a str) -> Self {
        Self { error_message }
    }
}

impl Widget for ErrorPopup<'_> {
    fn render(self, area: Rect, buf: &mut Buffer)
    where
        Self: Sized,
    {
        let block = Block::bordered().title("ERROR");
        let centered_area = area.centered(Constraint::Percentage(60), Constraint::Percentage(20));
        Clear.render(centered_area, buf);
        Paragraph::new(self.error_message)
            .block(block)
            .render(centered_area, buf);
    }
}

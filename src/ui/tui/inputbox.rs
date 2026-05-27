use ratatui::widgets::{Block, Borders, Paragraph, Widget};
use tui_input::Input;

pub struct InputBox<'a> {
    input: &'a Input,
}

impl<'a> InputBox<'a> {
    pub fn new(input: &'a Input) -> Self {
        Self { input }
    }

    fn scroll(&self, width: u16) -> usize {
        self.input.visual_scroll(width as usize - 2)
    }

    pub fn offset_x(&self, width: u16) -> u16 {
        let scroll = self.scroll(width);
        (self.input.cursor().max(scroll) - scroll) as u16
    }
}

impl Widget for InputBox<'_> {
    fn render(self, area: ratatui::prelude::Rect, buf: &mut ratatui::prelude::Buffer)
    where
        Self: Sized,
    {
        let scroll = self.scroll(area.width);
        let input_widget = Paragraph::new(self.input.value())
            .scroll((0, scroll as u16))
            .block(Block::default().borders(Borders::ALL).title("INPUT"));
        input_widget.render(area, buf);
    }
}

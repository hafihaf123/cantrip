use ratatui::buffer::Buffer;
use ratatui::layout::{Alignment, Rect};
use ratatui::style::{Color, Modifier, Style};
use ratatui::text::{Line, Span};
use ratatui::widgets::{Block, Paragraph, StatefulWidget, Widget, Wrap};

use crate::chat::{LogMessage, MessageType};
use crate::ui::tui::renderer::ScrollState;

pub struct ChatBox<'a> {
    messages: &'a [LogMessage],
}

impl<'a> ChatBox<'a> {
    pub fn new(messages: &'a [LogMessage]) -> Self {
        Self { messages }
    }

    fn generate_message_lines(&self) -> Vec<Line<'_>> {
        let mut text_lines = Vec::new();
        let mut last_author: Option<MessageType> = None;

        for msg in self.messages {
            let is_same_author = last_author.as_ref() == Some(&msg.message_type);

            match &msg.message_type {
                MessageType::User(name) => {
                    if !is_same_author {
                        if !text_lines.is_empty() {
                            text_lines.push(Line::raw(""));
                        }
                        text_lines.push(Line::from(Span::styled(
                            format!("{}:", name),
                            Style::default()
                                .fg(Color::Cyan)
                                .add_modifier(Modifier::BOLD),
                        )));
                    }
                    text_lines.push(Line::from(vec![Span::raw("  "), Span::raw(&msg.content)]));
                }
                MessageType::Me => {
                    if !is_same_author {
                        if !text_lines.is_empty() {
                            text_lines.push(Line::raw(""));
                        }
                        text_lines.push(Line::from(Span::styled(
                            "You:",
                            Style::default()
                                .fg(Color::Blue)
                                .add_modifier(Modifier::BOLD),
                        )));
                    }
                    text_lines.push(Line::from(vec![Span::raw("  "), Span::raw(&msg.content)]));
                }
                MessageType::System => {
                    if !is_same_author && !text_lines.is_empty() {
                        text_lines.push(Line::raw(""));
                    }
                    text_lines.push(
                        Line::from(Span::styled(
                            format!("-- {} --", msg.content),
                            Style::default()
                                .fg(Color::Gray)
                                .add_modifier(Modifier::ITALIC),
                        ))
                        .alignment(Alignment::Center),
                    );
                }
                MessageType::Dice {
                    user,
                    result,
                    rolls,
                    dice,
                } => {
                    if !is_same_author && !text_lines.is_empty() {
                        text_lines.push(Line::raw(""));
                    }
                    text_lines.push(Line::from(vec![
                        Span::raw("  🎲 "),
                        Span::styled(
                            format!("{} rolled {} from {}   {:#?}", user, result, dice, rolls),
                            Style::default().fg(Color::Yellow),
                        ),
                    ]));
                }
            }
            last_author = Some(msg.message_type.clone());
        }
        text_lines
    }

    // fn calculate_wrapped_height(lines: &[Line], width: usize) -> usize {
    //     if width == 0 {
    //         return 0;
    //     }
    //
    //     lines
    //         .iter()
    //         .map(|line| (line.width().saturating_sub(1) / width) + 1)
    //         .sum()
    // }
}

impl StatefulWidget for ChatBox<'_> {
    type State = ScrollState;

    fn render(self, area: Rect, buf: &mut Buffer, state: &mut Self::State)
    where
        Self: Sized,
    {
        // let inner_width = area.width.saturating_sub(2) as usize;
        // let viewport_height = area.height.saturating_sub(2);
        let text_lines = self.generate_message_lines();

        let block = Block::bordered().title(" Chat Room ");
        let inner_area = block.inner(area);

        let messages = Paragraph::new(text_lines).wrap(Wrap { trim: false });

        let messages_total_lines = messages.line_count(inner_area.width);

        let line_difference = messages_total_lines.saturating_sub(state.previous_total_lines);
        if state.lines_from_bottom > 0 && line_difference > 0 {
            state.lines_from_bottom += line_difference as u16;
        }
        state.previous_total_lines = messages_total_lines;

        state.max_scroll = messages_total_lines.saturating_sub(inner_area.height as usize);

        state.lines_from_bottom = state.lines_from_bottom.min(state.max_scroll as u16);
        let render_scroll = state
            .max_scroll
            .saturating_sub(state.lines_from_bottom as usize) as u16;

        messages
            .block(block)
            .scroll((render_scroll, 0))
            .render(area, buf);
    }
}

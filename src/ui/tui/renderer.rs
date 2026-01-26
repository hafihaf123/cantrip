use crate::events::ChatEvent;
use crate::ui::tui::TuiBackendGuard;
use crate::ui::{ChatRenderer, tui::model::TuiModel};
use anyhow::Result;
use ratatui::layout::{Constraint, Layout};
use ratatui::style::{Modifier, Style};
use ratatui::widgets::{Block, Borders, List, Paragraph};
use ratatui::{Terminal, prelude::CrosstermBackend};
use std::io;
use std::{io::Stdout, sync::Arc};
use tokio::sync::RwLock;

pub struct TuiRenderer {
    terminal: Terminal<CrosstermBackend<Stdout>>,
    model: Arc<RwLock<TuiModel>>,
    _tui_backend_guard: TuiBackendGuard,
}

impl TuiRenderer {
    pub(super) fn new(
        model: Arc<RwLock<TuiModel>>,
        _tui_backend_guard: TuiBackendGuard,
    ) -> io::Result<Self> {
        let terminal = Terminal::new(CrosstermBackend::new(io::stdout()))?;
        Ok(Self {
            terminal,
            model,
            _tui_backend_guard,
        })
    }
}

impl ChatRenderer for TuiRenderer {
    async fn render(&mut self, event: ChatEvent) -> Result<()> {
        {
            let mut model = self.model.write().await;
            model.apply_event(&event);
        }

        let model = self.model.read().await;

        self.terminal.draw(|frame| {
            let chunks = Layout::default()
                .constraints([Constraint::Min(1), Constraint::Length(3)])
                .split(frame.area());

            let messages = List::new(model.messages.clone())
                .block(Block::bordered().title("Messages"))
                .highlight_style(Style::new().add_modifier(Modifier::REVERSED));
            {
                let mut state = model.list_state.lock().unwrap();
                frame.render_stateful_widget(messages, chunks[0], &mut state);
            }

            let scroll = model.input.visual_scroll(chunks[1].width as usize - 2);
            let input_widget = Paragraph::new(model.input.value())
                .scroll((0, scroll as u16))
                .block(Block::default().borders(Borders::ALL).title("INPUT"));
            frame.render_widget(input_widget, chunks[1]);

            frame.set_cursor_position((
                chunks[1].x + 1 + (model.input.cursor().max(scroll) - scroll) as u16,
                chunks[1].y + 1,
            ));
        })?;

        Ok(())
    }
}

use crate::chat::AppState;
use crate::ui::tui::error_popup::ErrorPopup;
use crate::ui::tui::{TuiBackendGuard, chatbox::ChatBox, inputbox::InputBox};
use crate::ui::{ChatRenderer, InputEvent};
use anyhow::Result;
use ratatui::layout::{Constraint, Layout};
use ratatui::{Terminal, prelude::CrosstermBackend};
use std::io::{self, Stdout};

pub struct ScrollState {
    pub lines_from_bottom: u16,
    pub max_scroll: usize,
    pub previous_total_lines: usize,
}

impl ScrollState {
    fn new() -> Self {
        Self {
            lines_from_bottom: 0,
            max_scroll: 0,
            previous_total_lines: 0,
        }
    }

    pub fn scroll_up(&mut self) {
        if (self.lines_from_bottom as usize) < self.max_scroll {
            self.lines_from_bottom = self.lines_from_bottom.saturating_add(1);
        }
    }

    pub fn scroll_down(&mut self) {
        self.lines_from_bottom = self.lines_from_bottom.saturating_sub(1);
    }
}

pub struct TuiRenderer {
    terminal: Terminal<CrosstermBackend<Stdout>>,
    scroll_state: ScrollState,
    _tui_backend_guard: TuiBackendGuard,
}

impl TuiRenderer {
    pub(super) fn new(_tui_backend_guard: TuiBackendGuard) -> io::Result<Self> {
        let terminal = Terminal::new(CrosstermBackend::new(io::stdout()))?;
        Ok(Self {
            terminal,
            _tui_backend_guard,
            scroll_state: ScrollState::new(),
        })
    }

    pub fn scroll_up(&mut self) {
        self.scroll_state.scroll_up();
    }

    pub fn scroll_down(&mut self) {
        self.scroll_state.scroll_down();
    }
}

impl ChatRenderer for TuiRenderer {
    async fn draw(&mut self, state: &AppState) -> Result<()> {
        self.terminal.draw(|frame| {
            let chunks = Layout::default()
                .constraints([Constraint::Min(1), Constraint::Length(3)])
                .split(frame.area());

            let chatbox = ChatBox::new(state.messages());

            let input_box = InputBox::new(state.input());

            frame.set_cursor_position((
                chunks[1].x + 1 + input_box.offset_x(chunks[1].width),
                chunks[1].y + 1,
            ));

            frame.render_stateful_widget(chatbox, chunks[0], &mut self.scroll_state);
            frame.render_widget(input_box, chunks[1]);

            if let Some(error_message) = state.error_popup() {
                frame.render_widget(ErrorPopup::new(error_message), frame.area());
            }
        })?;

        Ok(())
    }

    fn handle_ui_event(&mut self, event: &InputEvent) -> bool {
        match event {
            InputEvent::ScrollUp => {
                self.scroll_up();
                true
            }
            InputEvent::ScrollDown => {
                self.scroll_down();
                true
            }
            InputEvent::Submit
            | InputEvent::Terminal(_)
            | InputEvent::Close
            | InputEvent::Redraw => false,
        }
    }
}

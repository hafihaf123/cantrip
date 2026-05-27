mod chatbox;
mod error_popup;
mod input;
mod inputbox;
mod renderer;

use crate::ui::UserInterface;
use crate::ui::tui::{input::TuiInput, renderer::TuiRenderer};
use anyhow::Result;
use ratatui::crossterm::execute;
use ratatui::crossterm::terminal::{
    EnterAlternateScreen, LeaveAlternateScreen, disable_raw_mode, enable_raw_mode,
};
use std::io;

struct TuiBackendGuard;

impl TuiBackendGuard {
    fn new() -> io::Result<Self> {
        enable_raw_mode()?;

        let mut stdout = io::stdout();
        execute!(stdout, EnterAlternateScreen)?;

        Ok(Self)
    }
}

impl Drop for TuiBackendGuard {
    fn drop(&mut self) {
        _ = disable_raw_mode();
        _ = execute!(io::stdout(), LeaveAlternateScreen);
    }
}

pub struct TerminalInterface;

impl UserInterface for TerminalInterface {
    type Renderer = TuiRenderer;

    type Input = TuiInput;

    fn init() -> Result<(Self::Renderer, Self::Input)> {
        let backend_guard = TuiBackendGuard::new()?;

        let input_source = TuiInput::new();
        let renderer = TuiRenderer::new(backend_guard)?;

        Ok((renderer, input_source))
    }
}

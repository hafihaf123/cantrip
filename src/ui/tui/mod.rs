mod input;
mod model;
mod renderer;

use crate::ui::{
    UserInterface,
    tui::{input::TuiInput, model::TuiModel, renderer::TuiRenderer},
};
use anyhow::Result;
use ratatui::crossterm::{
    execute,
    terminal::{EnterAlternateScreen, LeaveAlternateScreen, disable_raw_mode, enable_raw_mode},
};
use std::{io, sync::Arc};
use tokio::sync::RwLock;

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
        let model = Arc::new(RwLock::new(TuiModel::default()));

        let input_source = TuiInput::new(Arc::clone(&model));
        let renderer = TuiRenderer::new(model, backend_guard)?;

        Ok((renderer, input_source))
    }
}

use crate::chat::{AppState, MessageType};
use crate::ui::{ChatRenderer, InputEvent, InputSource, UserInterface};
use anyhow::{Result, anyhow};
use ratatui::crossterm::event::{Event as CrosstermEvent, KeyCode, KeyEvent};
use std::collections::VecDeque;
use std::io::{self, BufRead};

pub struct StdioUI {
    stdin: io::Stdin,
    pending_keystrokes: VecDeque<char>,
}

impl StdioUI {
    fn new() -> Self {
        Self {
            stdin: io::stdin(),
            pending_keystrokes: VecDeque::new(),
        }
    }
}

impl UserInterface for StdioUI {
    type Renderer = StdioUI;

    type Input = StdioUI;

    fn init() -> Result<(Self::Renderer, Self::Input)> {
        Ok((StdioUI::new(), StdioUI::new()))
    }
}

impl ChatRenderer for StdioUI {
    async fn draw(&mut self, state: &AppState) -> Result<()> {
        let mut previous_user = None;
        let mut was_me_previously = false;
        for message in state.messages() {
            match &message.message_type {
                MessageType::User(user) => {
                    if previous_user.is_none_or(|prev_user| prev_user != user) {
                        println!("{}:", user);
                    }
                    println!("   {}", message.content);
                    previous_user = Some(user);
                    was_me_previously = false;
                    continue;
                }
                MessageType::Me => {
                    if !was_me_previously {
                        println!("You:");
                    }
                    println!("   {}", message.content);
                    previous_user = None;
                    was_me_previously = true;
                    continue;
                }
                MessageType::System => {
                    println!(">> {} <<", message.content);
                }
                MessageType::Dice {
                    user,
                    result,
                    rolls,
                    dice,
                } => {
                    println!("🎲 {} rolled {} from {}   {:#?}", user, result, dice, rolls);
                }
            }
            previous_user = None;
            was_me_previously = false;
        }
        Ok(())
    }
}

impl InputSource for StdioUI {
    fn get_input(&mut self) -> Result<InputEvent> {
        if let Some(c) = self.pending_keystrokes.pop_front() {
            if c == '\n' {
                return Ok(InputEvent::Submit);
            }
            return Ok(InputEvent::Terminal(CrosstermEvent::Key(KeyEvent::from(
                KeyCode::Char(c),
            ))));
        }

        let mut line = String::new();
        let mut handle = self.stdin.lock();
        let bytes = handle.read_line(&mut line)?;

        if bytes == 0 {
            return Err(anyhow!(
                "EOF reached during the handling of input from stdin"
            ));
        }

        let trimmed = line.trim_end_matches(['\r', '\n']);
        self.pending_keystrokes = trimmed.chars().collect();
        self.pending_keystrokes.push_back('\n');

        self.get_input()
    }
}

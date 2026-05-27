use crate::ChatClient;
use crate::chat::state::AppState;
use crate::command::InputCommand;
use crate::dice::Dice;
use crate::events::{ChatEvent, NetworkEvent, SystemEvent};
use crate::ui::{ChatRenderer, InputEvent};
use anyhow::Result;
use arboard::Clipboard;
use std::ops::ControlFlow;
use tokio::sync::broadcast;
use tui_input::backend::crossterm::EventHandler;

pub struct ChatApp<UI: ChatRenderer> {
    renderer: UI,
    app_state: AppState,
    client: Option<ChatClient>,
    shutdown_tx: broadcast::Sender<()>,
    _clipboard: Option<Clipboard>,
}

impl<UI: ChatRenderer> ChatApp<UI> {
    pub fn new(renderer: UI, shutdown_tx: broadcast::Sender<()>) -> Self {
        Self {
            renderer,
            app_state: AppState::default(),
            client: None,
            shutdown_tx,
            _clipboard: None,
        }
    }

    pub fn client(&self) -> Option<&ChatClient> {
        self.client.as_ref()
    }

    pub fn set_client(&mut self, client: ChatClient) {
        self.client = Some(client);
    }

    pub fn subscribe_shutdown(&self) -> broadcast::Receiver<()> {
        self.shutdown_tx.subscribe()
    }

    pub fn set_clipboard(&mut self, clipboard: Option<Clipboard>) {
        self._clipboard = clipboard;
    }

    pub async fn render_error(&mut self, message: String) -> Result<()> {
        self.app_state.apply_event(ChatEvent::Error(message));
        self.renderer.draw(&self.app_state).await
    }

    pub async fn handle_user_input(&mut self, input_event: InputEvent) -> Result<ControlFlow<()>> {
        if !self.renderer.handle_ui_event(&input_event) {
            match input_event {
                InputEvent::Submit => {
                    let message = self.app_state.input_mut().value_and_reset();
                    if !message.is_empty() {
                        let command = InputCommand::from(message);
                        return self.handle_command(command).await;
                    }
                }
                InputEvent::Close => {
                    self.app_state.close_error_popup();
                }
                InputEvent::Terminal(event) => {
                    self.app_state.input_mut().handle_event(&event);
                }
                InputEvent::Redraw => {}
                _ => {}
            }
        }
        self.renderer.draw(&self.app_state).await?;
        Ok(ControlFlow::Continue(()))
    }

    async fn handle_command(&mut self, command: InputCommand) -> Result<ControlFlow<()>> {
        let event = if let Some(client) = &self.client {
            match command {
                InputCommand::Quit => {
                    client.broadcast_left().await?;
                    // give some time to the bradcast to succeed
                    tokio::time::sleep(std::time::Duration::from_millis(100)).await;
                    _ = self.shutdown_tx.send(());
                    return Ok(ControlFlow::Break(()));
                }
                InputCommand::Broadcast(message) => {
                    client.broadcast_text(message.clone()).await?;
                    ChatEvent::MessageSent(message)
                }
                InputCommand::ChangeName(name) => {
                    let msg = format!("Changed name to {}", &name);
                    client.broadcast_join(name).await?;
                    ChatEvent::SystemStatus(msg)
                }
                InputCommand::DiceRoll(dice_str) => match dice_str.parse::<Dice>() {
                    Ok(dice) => {
                        let (result, rolls) = dice.roll();
                        client
                            .broadcast_dice_roll(result, dice, rolls.clone())
                            .await?;
                        ChatEvent::DiceRolled {
                            result,
                            rolls,
                            dice,
                            author: None,
                        }
                    }
                    Err(e) => ChatEvent::Error(e.to_string()),
                },
            }
        } else {
            ChatEvent::Error("Wait for connection...".to_string())
        };

        self.app_state.apply_event(event);
        self.renderer.draw(&self.app_state).await?;
        Ok(ControlFlow::Continue(()))
    }

    pub async fn handle_system_event(&mut self, event: SystemEvent) -> Result<()> {
        match event {
            SystemEvent::Ui(ui_event) => {
                self.app_state.apply_event(ui_event);
                self.renderer.draw(&self.app_state).await?
            }
            SystemEvent::Network(network_event) => match network_event {
                NetworkEvent::BroadcastJoin(name) => {
                    if let Some(client) = &self.client {
                        client.broadcast_join(name).await?;
                    } else {
                        self.app_state
                            .apply_event(ChatEvent::Error("Waiting for connection...".to_string()));
                        self.renderer.draw(&self.app_state).await?;
                    }
                }
            },
        }
        Ok(())
    }

    pub async fn handle_ctrl_c(&mut self) -> Result<()> {
        if let Some(client) = &self.client {
            client.broadcast_left().await?;
            // give some time to the bradcast to succeed
            tokio::time::sleep(std::time::Duration::from_millis(100)).await;
        }
        self.app_state.apply_event(ChatEvent::SystemStatus(
            "Detected shutdown sequence. You can also use the '/quit' command to leave".to_string(),
        ));
        self.renderer.draw(&self.app_state).await?;
        _ = self.shutdown_tx.send(());
        Ok(())
    }
}

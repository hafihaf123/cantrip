use crate::events::{ChatEvent, NetworkEvent, SystemEvent};
use crate::{ChatClient, Dice};
use crate::{command::InputCommand, ui::ChatRenderer};
use anyhow::Result;
use std::ops::ControlFlow;
use tokio::sync::broadcast;

pub struct ChatApp<UI: ChatRenderer> {
    renderer: UI,
    client: Option<ChatClient>,
    shutdown_tx: broadcast::Sender<()>,
}

impl<UI: ChatRenderer> ChatApp<UI> {
    pub fn new(renderer: UI, shutdown_tx: broadcast::Sender<()>) -> Self {
        Self {
            renderer,
            client: None,
            shutdown_tx,
        }
    }

    pub fn client(&self) -> Option<&ChatClient> {
        self.client.as_ref()
    }

    pub fn set_client(&mut self, client: ChatClient) {
        self.client = Some(client);
    }

    pub fn ui(&mut self) -> &mut UI {
        &mut self.renderer
    }

    pub fn subscribe_shutdown(&self) -> broadcast::Receiver<()> {
        self.shutdown_tx.subscribe()
    }

    pub async fn handle_user_input(&mut self, line: String) -> Result<ControlFlow<()>> {
        if let Some(client) = &self.client {
            let command = InputCommand::from(line);
            match command {
                InputCommand::Quit => {
                    client.broadcast_left().await?;
                    // give some time to the bradcast to succeed
                    tokio::time::sleep(std::time::Duration::from_millis(100)).await;
                    self.shutdown_tx.send(()).ok();
                    return Ok(ControlFlow::Break(()));
                }
                InputCommand::Broadcast(message) => client.broadcast_text(message).await?,
                InputCommand::ChangeName(name) => {
                    self.renderer
                        .render(ChatEvent::SystemStatus(format!(
                            "Changed name to {}",
                            &name
                        )))
                        .await?;
                    client.broadcast_join(name).await?;
                }
                InputCommand::DiceRoll(dice_str) => match dice_str.parse::<Dice>() {
                    Ok(dice) => {
                        let (result, rolls) = dice.roll();
                        self.renderer
                            .render(ChatEvent::DiceRolled {
                                result,
                                rolls: rolls.clone(),
                                dice,
                                author: None,
                            })
                            .await?;
                        client.broadcast_dice_roll(result, dice, rolls).await?;
                    }
                    Err(e) => {
                        self.renderer
                            .render(ChatEvent::Error(e.to_string()))
                            .await?
                    }
                },
            }
        } else {
            self.renderer
                .render(ChatEvent::Error("Wait for connection...".to_string()))
                .await?;
        }
        Ok(ControlFlow::Continue(()))
    }

    pub async fn handle_system_event(&mut self, event: SystemEvent) -> Result<()> {
        match event {
            SystemEvent::Ui(ui_event) => self.renderer.render(ui_event).await?,
            SystemEvent::Network(network_event) => match network_event {
                NetworkEvent::BroadcastJoin(name) => {
                    if let Some(client) = &self.client {
                        client.broadcast_join(name).await?;
                    } else {
                        self.renderer
                            .render(ChatEvent::Error("Waiting for connection...".to_string()))
                            .await?;
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
        self.renderer
            .render(ChatEvent::SystemStatus(
                "Detected shutdown sequence. You can also use the '/quit' command to leave"
                    .to_string(),
            ))
            .await?;
        self.shutdown_tx.send(()).ok();
        Ok(())
    }
}

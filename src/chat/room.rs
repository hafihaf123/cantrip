use crate::chat::{ChatBackend, ChatClient, ChatConfig};
use crate::events::{ChatEvent, SystemEvent};
use crate::ticket::Ticket;
use anyhow::Result;
use arboard::Clipboard;
use iroh::discovery::{dns::DnsDiscovery, mdns::MdnsDiscovery, pkarr::PkarrPublisher};
use iroh::{Endpoint, RelayMode, protocol::Router};
use iroh_gossip::Gossip;
use tokio::{sync::mpsc::Sender, task::spawn_blocking};

pub struct ChatRoom {}

impl ChatRoom {
    pub async fn connect(
        config: ChatConfig,
        event_tx: Sender<SystemEvent>,
    ) -> Result<(ChatClient, ChatBackend, Option<Clipboard>)> {
        let endpoint = Endpoint::empty_builder(RelayMode::Default)
            .discovery(PkarrPublisher::n0_dns())
            .discovery(DnsDiscovery::n0_dns())
            .discovery(MdnsDiscovery::builder())
            .secret_key(config.secret_key)
            .bind()
            .await?;
        let gossip = Gossip::builder().spawn(endpoint.clone());
        let router = Router::builder(endpoint.clone())
            .accept(iroh_gossip::ALPN, gossip.clone())
            .spawn();

        let clipboard = if config.is_host {
            let ticket = Ticket::new(config.topic, vec![endpoint.addr()]);
            let ticket_str = ticket.to_string();

            let (clipboard_msg, clipboard) = spawn_blocking(|| match Clipboard::new() {
                Ok(mut clipboard) => match clipboard.set_text(ticket_str) {
                    Ok(_) => (" (copied to clipboard)", Some(clipboard)),
                    Err(_) => ("", None),
                },
                Err(_) => ("", None),
            })
            .await
            .unwrap_or(("", None));

            event_tx
                .send(SystemEvent::Ui(ChatEvent::SystemStatus(format!(
                    "ticket to join: {ticket}{clipboard_msg}"
                ))))
                .await?;

            clipboard
        } else {
            None
        };

        let endpoints = config.bootstrap_nodes;
        let endpoint_ids = endpoints.iter().map(|p| p.id).collect();
        if endpoints.is_empty() {
            event_tx
                .send(SystemEvent::Ui(ChatEvent::SystemStatus(
                    "waiting for endpoints to join us...".to_string(),
                )))
                .await?;
        } else {
            event_tx
                .send(SystemEvent::Ui(ChatEvent::SystemStatus(format!(
                    "trying to connect to {} endpoints...",
                    endpoints.len()
                ))))
                .await?;
        };
        let (sender, receiver) = gossip
            .subscribe_and_join(config.topic, endpoint_ids)
            .await?
            .split();
        event_tx
            .send(SystemEvent::Ui(ChatEvent::SystemStatus(
                "connected!".to_string(),
            )))
            .await?;

        let client = ChatClient::new(sender, endpoint.clone(), config.symmetric_key);
        client.broadcast_join(config.username.clone()).await?;

        let backend = ChatBackend::new(
            endpoint,
            config.symmetric_key,
            router,
            receiver,
            event_tx,
            config.username,
        );
        Ok((client, backend, clipboard))
    }
}

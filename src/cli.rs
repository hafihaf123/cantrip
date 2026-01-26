use anyhow::Result;
use anyhow::anyhow;
use clap::Parser;
use dialoguer::Input;

use crate::ticket::TICKET_PREFIX;

#[derive(Parser)]
#[command(version, about, long_about = None)]
struct Args {
    /// The username to use for this session
    ///
    /// The username is tied to a user's identity, so using a new username means also creating a
    /// new identity - with a separate new secret key.
    #[arg(short, long)]
    username: Option<String>,

    /// The room name to create or connect to
    #[arg(short, long)]
    room: Option<String>,

    /// Specifies whether you want to open a new chat room or join an existing one
    #[clap(subcommand)]
    command: Command,
}

#[derive(Parser, Debug)]
pub enum Command {
    /// Open a chat room for a topic and print a ticket for others to join.
    Open,
    /// Join a chat room from a ticket.
    Join {
        /// The ticket, as base32 string.
        ticket: Option<String>,
    },
}

pub struct Cli {
    pub username: String,
    pub room: String,
    pub password: String,
    pub ticket: Option<String>,
}

impl Cli {
    pub fn parse() -> Result<Self> {
        let args = Args::parse();
        args.try_into()
    }
}

impl TryFrom<Args> for Cli {
    type Error = anyhow::Error;

    fn try_from(value: Args) -> std::result::Result<Self, Self::Error> {
        let username = value
            .username
            .map(|username| {
                if username.len() > 3 {
                    Ok(username)
                } else {
                    Err(anyhow!("Username is too short"))
                }
            })
            .unwrap_or_else(|| {
                inquire_argument("Enter username", "Username is too short", |s| s.len() > 3)
            })?;

        let room = value
            .room
            .map(|topic| {
                if topic.len() > 3 {
                    Ok(topic)
                } else {
                    Err(anyhow!("Topic name is too short"))
                }
            })
            .unwrap_or_else(|| {
                inquire_argument("Enter room name", "Room name is too short", |s| s.len() > 3)
            })?;

        let ticket = if let Command::Join { ticket } = value.command {
            Some(
                ticket
                    .map(|topic| {
                        if topic.starts_with(TICKET_PREFIX) {
                            Ok(topic)
                        } else {
                            Err(anyhow!("Invalid ticket"))
                        }
                    })
                    .unwrap_or_else(|| {
                        inquire_argument("Enter ticket to join", "Invalid ticket", |s| {
                            s.strip_prefix(TICKET_PREFIX).is_some()
                        })
                    })?,
            )
        } else {
            None
        };

        let password = loop {
            match dialoguer::Password::new()
                .with_prompt("Enter room password")
                .interact()
            {
                Ok(password) => break password,
                Err(_) => continue,
            }
        };

        Ok(Cli {
            username,
            room,
            password,
            ticket,
        })
    }
}

fn inquire_argument(
    input_prompt: &str,
    err_prompt: &str,
    validator: impl Fn(&String) -> bool,
) -> Result<String> {
    Ok(Input::new()
        .with_prompt(input_prompt)
        .validate_with(|input: &String| {
            if validator(input) {
                Ok(())
            } else {
                Err(err_prompt)
            }
        })
        .interact_text()?)
}

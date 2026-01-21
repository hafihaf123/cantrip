use anyhow::Result;
use anyhow::anyhow;
use clap::Parser;
use dialoguer::Input;

#[derive(Parser)]
#[command(version, about, long_about = None)]
struct Args {
    /// The username to use for this session
    ///
    /// The username is tied to a user's identity, so using a new username means also creating a
    /// new identity - with a separate new secret key.
    #[arg(short, long)]
    username: Option<String>,

    #[arg(short, long)]
    topic: Option<String>,

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
        ticket: String,
    },
}

pub struct Cli {
    pub username: String,
    pub topic: String,
    pub password: String,
    pub command: Command,
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
            .unwrap_or_else(|| inquire_argument("Enter username", "Username is too short"))?;

        let topic = value
            .topic
            .map(|topic| {
                if topic.len() > 3 {
                    Ok(topic)
                } else {
                    Err(anyhow!("Topic name is too short"))
                }
            })
            .unwrap_or_else(|| inquire_argument("Enter room name", "Room name is too short"))?;

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
            topic,
            password,
            command: value.command,
        })
    }
}

fn inquire_argument(input_prompt: &str, err_prompt: &str) -> Result<String> {
    Ok(Input::new()
        .with_prompt(input_prompt)
        .validate_with(|input: &String| {
            if input.len() > 3 {
                Ok(())
            } else {
                Err(err_prompt)
            }
        })
        .interact_text()?)
}

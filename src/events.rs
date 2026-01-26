use crate::dice::Dice;

pub enum SystemEvent {
    Ui(ChatEvent),
    Network(NetworkEvent),
}

pub enum NetworkEvent {
    BroadcastJoin(String),
}

pub enum ChatEvent {
    MessageReceived {
        author: String,
        content: String,
    },
    MessageSent(String),
    PeerJoined(String),
    PeerLeft(String),
    PeerNameChange {
        old: String,
        new: String,
    },
    SystemStatus(String),
    DiceRolled {
        result: u32,
        rolls: Vec<u32>,
        dice: Dice,
        author: Option<String>,
    },
    Error(String),
    Redraw,
}

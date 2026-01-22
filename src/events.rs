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
    PeerJoined(String),
    #[allow(dead_code)]
    PeerLeft(String),
    PeerNameChange {
        old: String,
        new: String,
    },
    SystemStatus(String),
    Error(String),
}

# Cantrip

Cantrip is a secure, peer-to-peer command-line chat application built in Rust. It leverages Iroh for decentralized networking and provides encrypted communication channels.

## Features

- **Peer-to-Peer:** No central server required; connects directly via Iroh.
- **Secure:** Encrypted communication using standard cryptographic primitives.
- **Ephemeral:** Messages are broadcasted in real-time.
- **Ticket-based Invitation:** Easy sharing of room access via encoded tickets.

## Installation

Ensure you have Rust installed. Clone the repository and build:

```
cargo build --release
```

The binary will be available in `./target/release/cantrip`.

## Usage

Cantrip works by either **Opening** a new chat room or **Joining** an existing one using a ticket.

### Common Arguments
You will be prompted for a password interactively for extra security.
- `-u, --username`: Your display name (minimum 4 characters).
- `-t, --topic`: The chat room topic/name (minimum 4 characters).

### 1. Open a Chat Room
To start a new session and generate an invite ticket:

```
cargo run -- -u Alice -t General open
```

*This will display a ticket string that you can share with others.*

### 2. Join a Chat Room
To join an existing session using a ticket provided by a host:

```
cargo run -- -u Bob -t General join --ticket "ticket_string_here"
```

## License

This project is licensed under either of:

 * Apache License, Version 2.0
 * MIT license

at your option.

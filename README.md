# Cantrip: The Decentralized Table

**Cantrip** is a secure, peer-to-peer Virtual Tabletop (VTT) for Dungeons & Dragons and other RPGs, built in Rust. It leverages **Iroh** for decentralized networking to create a "serverless" game table where the Dungeon Master and players connect directly.

Unlike traditional VTTs that rely on central servers or subscription fees, Cantrip is ephemeral, private, and runs entirely in your terminal.

## The Vision

Cantrip aims to bring the "pen and paper" feel to the digital age without the bloat. It combines a secure chat client with a structured game state manager.
-   **No Servers:** Your game exists only while you are playing.
-   **Private:** All rolls, whispers, and maps are encrypted between peers.
-   **Hacker-Friendly:** A text-first interface that feels like a classic MUD but plays like 5th Edition.

## Features

### Core (Implemented)
-   **Peer-to-Peer:** Connects directly via Iroh's gossip protocol.
-   **End-to-End Encryption:** Standard cryptographic primitives (XChaCha20Poly1305) ensure only your party sees the game.
-   **Ticket-based Invites:** Securely share "Table Access" via encoded tickets.

### Roadmap (In Progress)
-   **The Game Board (TUI):** A split-pane terminal interface separating chat, initiative trackers, and player status (using `Ratatui`).
-   **Dice Engine:** Native support for `/roll 1d20+5` with verifiable results broadcast to the party.
-   **DM Authority:** The room creator acts as the host, managing the "Source of Truth" for health and turns.
-   **Private Whispers:** Encrypted direct messaging between DM and players for secret checks.

## Installation

Ensure you have Rust installed. More ways to install might be added later, after the project is stable.

```
cargo install --git https://github.com/hafihaf123/cantrip
```

The binary will be available in the Cargo binary directory. For more information check the [official Cargo documentation](https://doc.rust-lang.org/cargo/commands/cargo-install.html).

## Usage

Cantrip works by either **Hosting** a new table or **Joining** an existing one using a ticket.

### Common Arguments
You will be prompted for a password interactively for extra security.
-   `-u, --username`: Your character name (minimum 4 characters).
-   `-r, --room`: The room name (minimum 4 characters).

### 1. Host a Game Session (DM)
To start a new session and generate an invite ticket for your players:

```
cantrip -u DungeonMaster -r "CurseOfStrahd" open
```

*This will display a ticket string (e.g., `ticket-abc123...`) that you send to your players.*

### 2. Join a Table (Player)
To join an existing session using a ticket provided by the DM:

```
cantrip -u Grog -r "CurseOfStrahd" join --ticket "ticket_string_here"
```

## Development Roadmap

### Phase 1: The Dice & Command Framework
- [ ]  Implement structured command parsing (migrating from simple `/nick` commands).
- [ ]  Add a dice parser (e.g., `1d8+4`) and a `MessageBody::Roll` struct to visually distinguish rolls from chat.

### Phase 2: The Interface
- [ ]  Migrate from scrolling text (`println!`) to a full TUI with `Ratatui`.
- [ ]  Create a "Table State" pane to view connected peers and HP.

### Phase 3: Identity & Roles
- [ ]  Establish the **Dungeon Master** role based on the room creator's public key.
- [ ]  Implement Diffie-Hellman shared secrets for private "Whisper" channels.

### Phase 4: State Consistency
- [ ]  Implement a basic Authority Model where the DM's client broadcasts state snapshots (Health, Initiative) to sync late-joiners.

## License

This project is licensed under either of:

* Apache License, Version 2.0
* MIT license

at your option.

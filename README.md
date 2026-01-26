# Cantrip

[![Crates.io Version](https://img.shields.io/crates/v/cantrip-vtt?style=plastic)](https://crates.io/crates/cantrip-vtt)
[![GitHub Actions Workflow Status](https://img.shields.io/github/actions/workflow/status/hafihaf123/cantrip/rust.yml?style=plastic)](https://github.com/hafihaf123/cantrip/actions)
![Crates.io License](https://img.shields.io/crates/l/cantrip-vtt?style=plastic)
[![Downloads](https://img.shields.io/crates/d/cantrip-vtt?style=plastic)](https://crates.io/crates/cantrip-vtt)

<!--toc:start-->

- [Cantrip](#cantrip)
  - [The Vision](#the-vision)
  - [Features](#features)
  - [Installation](#installation)
  - [Usage](#usage)
    - [Common Arguments](#common-arguments)
    - [1. Host a Game](#1-host-a-game)
    - [2. Join a Table](#2-join-a-table)
    - [Key Commands](#key-commands)
  - [Development Roadmap](#development-roadmap)
    - [Phase 1: The Dice & Command Framework](#phase-1-the-dice-and-command-framework)
    - [Phase 2: The Interface](#phase-2-the-interface)
    - [Phase 3: Identity & Roles](#phase-3-identity-and-roles)
    - [Phase 4: State Consistency](#phase-4-state-consistency)
  - [License](#license)
  <!--toc:end-->

**Cantrip** is a secure, peer-to-peer Virtual Tabletop (VTT) for Dungeons & Dragons
and other RPGs, built in Rust. It leverages **Iroh** for decentralized networking
to create a "serverless" game table where the Dungeon Master and players connect
directly.

Ephemeral. Private. Hacker-Friendly.

![Cantrip TUI Demo](https://github.com/hafihaf123/cantrip/blob/main/vhs.gif)

## The Vision

Cantrip aims to bring the "pen and paper" feel to the digital age without the bloat.
It combines a secure chat client with a structured game state manager.

- **No Servers:** Your game exists only while you are playing.
- **Private:** All rolls, whispers, and maps are encrypted between peers.
- **Terminal-Native:** A text-first interface that feels like a classic MUD but
  plays like 5th Edition.

## Features

- **Peer-to-Peer:** Connects directly via Iroh's gossip protocol.
- **End-to-End Encryption:** Standard cryptographic primitives (XChaCha20Poly1305)
  ensure only your party sees the game.
- **Ticket-based Invites:** Securely share "Table Access" via encoded tickets.
- **The Game Board (TUI):** A split-pane terminal interface separating chat, initiative
  trackers, and player status (using `Ratatui`).
- **Dice Engine:** Native support for `/roll 1d20+5` with verifiable results broadcast
  to the party.

## Installation

Ensure you have Rust installed. More ways to install might be added later, after
the project is stable.

```bash
cargo install cantrip-vtt
```

_Requirements: A terminal with UTF-8 support and Rust installed._

The binary will be available in the Cargo binary directory. For more information
check the [official Cargo documentation](https://doc.rust-lang.org/cargo/commands/cargo-install.html).

## Usage

Cantrip works by either **Hosting** a new table or **Joining** an existing one using
a ticket. You can omit any commandline arguments to get and interactive prompt,
except the `open` or `join` subcommands.

### Common Arguments

- `-u, --username`: Your character name (minimum 4 characters).
- `-r, --room`: The room name (minimum 4 characters).

You will also be prompted for a password interactively for extra security.

### 1. Host a Game

To start a new session and generate an invite ticket for your players:

```bash
cantrip -u DungeonMaster -r "CurseOfStrahd" open
```

_This will display a ticket string (e.g., `ticket-abc123...`) that you send to your
players._

### 2. Join a Table

To join an existing session using a ticket provided by the DM:

```bash
cantrip -u Grog -r "CurseOfStrahd" join "ticket_string_here"
```

### Key Commands

Once inside the TUI, you can use the following commands:

- `/roll 1d20+2` - Roll for initiative or checks.
- `/nick new_name` - Change your displayed name.
- `/quit` - Leave the table.

## Development Roadmap

### Phase 1: The Dice and Command Framework

- [x] Implement structured command parsing (migrating from simple `/nick` commands).
- [x] Add a dice parser (e.g., `1d8+4`) and a `MessageBody::Roll` struct to visually
      distinguish rolls from chat.

### Phase 2: The Interface

- [x] Migrate from scrolling text (`println!`) to a full TUI with `Ratatui`.
- [ ] Create a "Table State" pane to view connected peers and HP.

### Phase 3: Identity and Roles

- [ ] Establish the **Dungeon Master** role based on the room creator's public key.
- [ ] Implement Diffie-Hellman shared secrets for private "Whisper" channels.

### Phase 4: State Consistency

- [ ] Implement a basic Authority Model where the DM's client broadcasts state snapshots
      (Health, Initiative) to sync late-joiners.

## License

This project is licensed under either of:

- Apache License, Version 2.0
- MIT license

at your option.

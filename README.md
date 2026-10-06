<div align="center">

<img width="100%" alt="header" src="https://capsule-render.vercel.app/api?type=waving&height=210&text=Bumpkin%20Legends%20Bot&fontAlign=50&fontAlignY=36&fontSize=56&desc=Daily%20%7C%20VIP%20%7C%20Tasks%20%7C%20Box%20%7C%20Wheel%20%7C%20Quest%20%7C%20Boss%20%7C%20Multi-Account&descAlign=50&descAlignY=58"/>

<img alt="typing" src="https://readme-typing-svg.demolab.com?font=Inter&size=18&duration=3000&pause=650&center=true&vCenter=true&width=900&lines=Auto+Daily+Reward+%7C+Claim+Each+Cycle;Auto+VIP+Claim+%7C+Free+and+VIP+Tier;Auto+Tasks+%7C+Social+%26+Partner+Configurable;Auto+Box+%7C+Open+All+Available+Boxes;Auto+Wheel+%7C+Free+%2F+VIP+%2F+Paid+Spin;Auto+Quest+%7C+Claim+All+Completed;Auto+Boss+%7C+Enter+%2F+World+Boss+%2F+Reset"/>

<p>
  <img alt="rust" src="https://img.shields.io/badge/Rust-2021-f74c00?logo=rust&logoColor=white"/>
  <img alt="platform" src="https://img.shields.io/badge/Platform-Bumpkin%20Legends%20Miniapp-111111"/>
  <img alt="multi-account" src="https://img.shields.io/badge/Multi--Account-Supported-111111"/>
  <img alt="author" src="https://img.shields.io/badge/by-Yuurisandesu-111111"/>
</p>

<p>
  <b>Bumpkin Legends Bot</b> is a full automation bot for the Bumpkin Legends Telegram Miniapp.<br/>
  It connects to the game server via a stateful WebSocket client, authenticates each account with a pinned device identity, and runs a complete feature cycle: daily reward, VIP claim, tasks, box opening, wheel spins, quest claims, and boss fights, all across multiple accounts sequentially with presence hold, proxy support, and a live countdown between cycles.<br/>
  Built and distributed by <b>Yuurisandesu</b>.
</p>

</div>

---

## Table of Contents

- [Requirements](#requirements)
- [Installation](#installation)
- [Configuration](#configuration)
- [Running the Bot](#running-the-bot)
- [Download Prebuilt Binary](#download-prebuilt-binary)
- [Features](#features)
- [File Structure](#file-structure)
- [Disclaimer](#disclaimer)

---

## Requirements

- Rust `1.70+` (includes `cargo`) -- only needed if building from source
- Git

---

## Installation

### Install Rust

**Linux / macOS:**

```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
source $HOME/.cargo/env
```

**Windows:**

Download and run the installer from https://rustup.rs, then restart your terminal.

**Termux (Android):**

```bash
pkg update && pkg install proot-distro
proot-distro install ubuntu
proot-distro login ubuntu
```

Then inside Ubuntu:

```bash
apt update && apt install -y curl git build-essential pkg-config libssl-dev
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
source $HOME/.cargo/env
```

### Clone the Repository

```bash
git clone https://github.com/Yuurisan-N1/Bumpkin-Miniapp.git
cd Bumpkin-Miniapp
```

---

## Configuration

### 1. Accounts (data.txt)

Fill `data.txt` with Telegram WebApp `initData` for each account, one per line. Lines starting with `#` are ignored:

```
user=%7B%22id%22...&hash=abc123
user=%7B%22id%22...&hash=def456
```

> `initData` can be obtained from the browser DevTools when opening Bumpkin Legends on Telegram Web.

### 2. Proxy (proxy.txt)

Fill `proxy.txt` with proxies, one per line (optional, leave empty to run without proxy):

```
host:port
host:port:user:pass
http://user:pass@host:port
socks5://user:pass@host:port
```

Proxies are assigned to accounts by index in round-robin order.

### 3. Bot Settings (config.json)

`config.json` is loaded at startup. Key settings per module:

| Module | Notable Config |
|---|---|
| `loop_cycle` | `enabled`, `sleep_seconds`, `state_sync_seconds`, `max_cycles` (0 = unlimited), `account_gap_seconds` |
| `profile` | `language` |
| `gate` | `verify_max_tries`, `verify_retry_seconds` |
| `daily` | `enabled` |
| `vip` | `enabled`, `claim` |
| `tasks` | `social`, `partner`, `claim` |
| `box` | `enabled`, `open_all` |
| `wheel` | `free`, `vip`, `spin_paid` |
| `quest` | `enabled` |
| `boss` | `enter`, `world_boss`, `reset` |
| `presence` | `enabled` |
| `spend` | `buy_pass`, `wheel_spin`, `market`, `auction`, `topup`, `withdraw` (all false by default) |

---

## Running the Bot

### Using run.sh (Linux / macOS / Termux)

Make it executable first:

```bash
chmod +x run.sh
```

Then choose a mode:

```bash
./run.sh direct    # build and run in foreground (default)
./run.sh nohup     # build and run in background, logs saved to bumpkin-legends-bot.log
./run.sh screen    # build and run in a detached screen session
./run.sh tmux      # build and run in a detached tmux session
./run.sh logs      # tail the log file
./run.sh stop      # stop the running bot process
```

Attach to a background session anytime:

```bash
# screen
screen -r bumpkin-legends-bot

# tmux
tmux attach -t bumpkin-legends-bot
```

### Using make (Linux / macOS)

```bash
make release    # optimized release build
make start      # build release and run immediately
make run        # build debug and run
make check      # check for errors without building
make fmt        # format the code
make clean      # remove all build artifacts
make size       # show release binary size
make package    # build release and package for distribution
```

### Manual cargo build

```bash
cargo build --release
```

Then run:

```bash
# Linux / macOS / Termux
./target/release/bumpkin-legends-bot

# Windows
.\target\release\bumpkin-legends-bot.exe
```

---

## Download Prebuilt Binary

Prebuilt binaries are automatically compiled on every push to main via GitHub Actions across 7 targets.

Download the latest binaries from the Actions page:
https://github.com/Yuurisan-N1/Bumpkin-Miniapp/actions/workflows/build.yml

Open the latest successful run and scroll to the Artifacts section. All binaries are retained for 90 days.

| Artifact | Platform |
|---|---|
| `bumpkin-legends-bot-linux-x86_64` | Linux x86_64 |
| `bumpkin-legends-bot-linux-aarch64` | Linux ARM64 |
| `bumpkin-legends-bot-linux-armv7` | Linux ARMv7 |
| `bumpkin-legends-bot-windows-x86_64` | Windows x86_64 |
| `bumpkin-legends-bot-macos-aarch64` | macOS Apple Silicon |
| `bumpkin-legends-bot-android-aarch64` | Android ARM64 (Termux) |
| `bumpkin-legends-bot-android-armv7` | Android ARMv7 (Termux) |

**Linux / macOS / Termux after downloading:**

```bash
chmod +x bumpkin-legends-bot-linux-x86_64
./bumpkin-legends-bot-linux-x86_64
```

**Windows:**

```bash
.\bumpkin-legends-bot-windows-x86_64.exe
```

---

## Features

### WebSocket Stateful Connection
The bot connects to the game server using a persistent WebSocket session. Each account gets its own connection with a pinned device identity derived from the account data. The connection handles the full protocol handshake, frame encoding, decoding, and graceful shutdown per account cycle.

### Gate Verification
After connecting, the bot runs a gate verification step before executing any game actions. It retries up to `verify_max_tries` times with `verify_retry_seconds` between attempts if the server is slow to confirm the session.

### Daily Reward
The bot claims the daily login reward for each account. If already claimed today, it is skipped.

### VIP Claim
The bot claims the free VIP reward and, if `vip.vip` is enabled, the paid VIP tier reward as well.

### Auto Tasks
The bot fetches the task list and claims all completed tasks. Social tasks and partner tasks are individually toggleable in config. Tasks already claimed are skipped.

### Auto Box
The bot opens all available boxes for the account. If `open_all` is enabled, it continues opening until no boxes remain.

### Wheel Spin
The bot spins the free wheel slot and optionally the VIP slot and paid slot based on config. Each spin result is logged.

### Auto Quest
The bot claims all completed quest rewards available for the account.

### Boss Fight
The bot enters the available boss fight, optionally including world boss battles. If `reset` is enabled, the boss counter is reset after claiming. Each fight result is logged.

### Presence Hold
After the feature cycle, the bot holds the WebSocket connection open for a configurable duration to fulfill any server-side presence requirement before gracefully shutting down the session.

### Multi Account
All accounts in `data.txt` are processed sequentially within every cycle. A configurable `account_gap_seconds` delay is applied between accounts to avoid overlapping server load. Each account uses its own proxy from the pool and maintains an independent WebSocket session.

### Loop Cycle
Set `loop_cycle.enabled` to `true` to keep the bot running indefinitely. A live countdown shows the wait between cycles. Set `max_cycles` to a positive number to stop automatically after that many cycles.

---

## File Structure

```text
Bumpkin-Miniapp/
├── .github/
│   └── workflows/
│       └── build.yml                    # CI: 7-target matrix build, artifacts retained 90 days
├── src/
│   ├── main.rs                          # Entry point, multi-account loop, cycle dispatch
│   ├── domain/                          # Account session, state, economy, world map models
│   ├── net/
│   │   ├── client/                      # HTTP API client, proxy pool, egress
│   │   └── ws/                          # WebSocket session, headers, frame transport
│   ├── ops/
│   │   ├── runner.rs                    # Engine: connect, sync, shutdown, state log
│   │   ├── features/
│   │   │   ├── boss.rs                  # Boss fight and world boss
│   │   │   ├── box.rs                   # Box open
│   │   │   ├── daily.rs                 # Daily reward claim
│   │   │   ├── quest.rs                 # Quest claim
│   │   │   ├── tasks.rs                 # Task claim (social, partner)
│   │   │   ├── vip.rs                   # VIP reward claim
│   │   │   └── wheel.rs                 # Wheel spin (free, VIP, paid)
│   │   └── sys/
│   │       ├── bootstrap.rs             # Session bootstrap and initial sync
│   │       ├── gate.rs                  # Gate verification
│   │       └── presence.rs              # Presence hold
│   ├── protocol/
│   │   ├── inbound/                     # Decoder, frame parser
│   │   └── outbound/                    # Encoder, request builders
│   └── support/                         # Banner, config loader, clock, logger, hash, constants
├── assets/
│   └── icon.ico                         # Windows binary icon
├── scripts/
│   └── package.sh                       # Package script for release distribution
├── Cargo.toml                           # Project manifest and dependencies
├── Cargo.lock
├── build.rs                             # Build script (embeds icon into Windows binary)
├── Makefile                             # make targets: build, run, release, start, clean, size, package
├── run.sh                               # Run helper: direct, nohup, screen, tmux, logs, stop
├── config.json                          # All module settings
├── data.txt                             # Account initData, one per line
└── proxy.txt                            # Proxy list (optional)
```

---

## Disclaimer

This tool is built for educational and technical exploration purposes. Use it wisely and at your own responsibility.

---

<div align="center">
<img width="100%" alt="footer" src="https://capsule-render.vercel.app/api?type=waving&height=120&section=footer"/>
</div>
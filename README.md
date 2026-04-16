# starex

_Experimental project (ZSH Support Only) – many features pending._

A lightweight Rust CLI that extends **Starship** with a real‑time daemon. It can display the current time with an icon, show holidays, and keep a background Unix socket alive for quick queries.

![Preview](https://github.com/JulianMKhoo/starex/tree/main/preview/0.gif)

## Install

```bash
# From source
cargo install --path .
# Or from crates.io (once published)
cargo install starex
```

## Quick start

```bash
# Start the background daemon (creates /tmp/starex.sock and a pid file)
starex start

# Add the init snippet to your .zshrc (only needed once, after starship eval() line)
eval "$(starex init zsh)"
```

The init command prints a command you should place **after** the Starship evaluation line in your `~/.zshrc` (or `~/.bashrc`).

## Commands

- `starex start` – launch the daemon.
- `starex kill` – stop the daemon.
- `starex status` – verify the daemon and init setup.
- `starex time` – print the current time with an appropriate icon.
- `starex holiday [options]` – show holidays:
  - `--now` – holiday today.
  - `--month <n>` – holidays for month _n_ (0 = current month).
  - `--full` – all holidays for the year.
- `starex weather` – placeholder for future weather integration.

## Configuration

User configuration lives at `~/.config/starex.toml`. You can override:

- `time_offset.timezone` – numeric offset from UTC.
- `time_offset.direction` – `east` or `west`.
- Custom icons for holidays via the `holiday_icon` table.

## How it works

- The daemon forks twice, detaches from the terminal, and writes its PID to `/tmp/starex_daemon.pid`.
- It listens on a Unix domain socket (`/tmp/starex.sock`) and replies with the time string every second.
- Holiday data is loaded from the bundled `data/starex.toml` and merged with any user overrides.

## License

MIT© 2026 JulianMKhoo

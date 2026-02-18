# Matrix Rain

A Matrix-style falling character screensaver built in Rust with SDL2. Designed for the Raspberry Pi 500 running Wayland (labwc), using a software renderer for CPU-only operation.

![Rust](https://img.shields.io/badge/Rust-000000?style=flat&logo=rust&logoColor=white)
![License: MIT](https://img.shields.io/badge/License-MIT-green.svg)

## Features

- Half-width Katakana, Latin, digit, and symbol characters
- Randomized drop speeds, trail lengths, and character flickering
- Pre-rendered glyph texture atlas for efficient blitting
- Fullscreen with hidden cursor; exits on any keypress or mouse movement
- 500ms grace period to ignore spurious input on launch
- Integrates with swayidle as an idle screensaver

## Prerequisites

```bash
sudo apt install libsdl2-dev libsdl2-ttf-dev
```

A monospace font is required. The default config expects JetBrains Mono Nerd Font at:

```
~/.local/share/fonts/JetBrainsMonoNerdFontMono-Regular.ttf
```

## Build

```bash
cargo build --release
```

The binary is at `target/release/matrix-rain` (~389K stripped).

## Run

```bash
./target/release/matrix-rain
```

Press any key, click, or move the mouse to exit.

## Swayidle Integration

Add to `~/.config/swayidle/config`:

```
timeout 300 '/path/to/matrix-rain' resume 'pkill -f matrix-rain'
timeout 600 'wlopm --off HDMI-A-1' resume 'wlopm --on HDMI-A-1'
```

This launches the screensaver after 5 minutes idle and turns off the display after 10 minutes.

## Configuration

Tuning constants are in `src/config.rs`:

| Constant | Default | Description |
|---|---|---|
| `FONT_SIZE` | 14 | Character size in pixels |
| `TARGET_FPS` | 30 | Frame rate cap |
| `SPEED_MIN` / `SPEED_MAX` | 0.3 / 1.2 | Drop speed range (rows/frame) |
| `TRAIL_MIN` / `TRAIL_MAX` | 8 / 50 | Trail length range (cells) |
| `SPAWN_CHANCE` | 0.015 | Probability of new drop per column per frame |
| `MUTATE_CHANCE` | 0.03 | Probability of character flicker per cell per frame |

## License

[MIT](LICENSE)

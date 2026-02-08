# waybar-toggle

A lightweight utility that automatically shows/hides [Waybar](https://github.com/Alexays/Waybar) when you move your mouse to the top of the screen.

## Features

- Automatically reveals Waybar when mouse reaches the top edge of the screen
- Hides Waybar when mouse moves away from the configured threshold
- Works with both Hyprland and Sway compositors
- Configurable hide threshold for fine-tuned control
- Optional hide delay to keep Waybar visible while moving into submenus
- Optional show delay to require the mouse to remain at the top edge before showing Waybar
- Minimal resource usage with configurable polling interval (default: 75ms)
- Simple command-line configuration

## Requirements

- Rust (for building)
- Waybar installed on your system
- Either Hyprland or Sway compositor
- Required compositor tools:
  - `hyprctl` (for Hyprland)
  - `swaymsg` (for Sway)

## Installation

```bash
cargo build --release
sudo cp target/release/waybar-toggle /usr/local/bin/
```

## Usage

```bash
waybar-toggle -c <config_path> -s <style_path> [-y <hide_threshold>] [-p <poll_ms>] [-d <hide_delay_ms>] [-e <show_delay_ms>]
```

### Arguments

- `-c, --config` - Path to your Waybar config file
- `-s, --style` - Path to your Waybar CSS style file
- `-y, --y-threshold` - (Optional) Y-coordinate threshold for hiding Waybar (default: 7)
- `-p, --poll-ms` - (Optional) Poll interval in milliseconds (default: 75)
- `-d, --hide-delay-ms` - (Optional) Delay in milliseconds before hiding Waybar (default: 250)
- `-e, --show-delay-ms` - (Optional) Delay in milliseconds before showing Waybar (default: 0)

### Examples

Basic usage:
```bash
waybar-toggle -c ~/.config/waybar/config -s ~/.config/waybar/style.css
```

With custom hide threshold (hides when mouse is more than 20 pixels from top):
```bash
waybar-toggle -c ~/.config/waybar/config -s ~/.config/waybar/style.css -y 20
```

With custom poll interval (poll every 50ms):
```bash
waybar-toggle -c ~/.config/waybar/config -s ~/.config/waybar/style.css -y 20 -p 50
```

With hide delay to allow submenu interaction:
```bash
waybar-toggle -c ~/.config/waybar/config -s ~/.config/waybar/style.css -y 20 -p 50 -d 300
```

With show delay to avoid accidental reveals:
```bash
waybar-toggle -c ~/.config/waybar/config -s ~/.config/waybar/style.css -y 20 -p 50 -d 300 -e 200
```
## Autostart

To start waybar-toggle automatically with your compositor:

### Hyprland

Add to `hyprland.conf`:
```conf
exec-once = waybar-toggle -c ~/.config/waybar/config -s ~/.config/waybar/style.css
```

With custom threshold:
```conf
exec-once = waybar-toggle -c ~/.config/waybar/config -s ~/.config/waybar/style.css -y 20
```

With custom poll interval:
```conf
exec-once = waybar-toggle -c ~/.config/waybar/config -s ~/.config/waybar/style.css -y 20 -p 50
```

With hide delay:
```conf
exec-once = waybar-toggle -c ~/.config/waybar/config -s ~/.config/waybar/style.css -y 20 -p 50 -d 300
```

With show delay:
```conf
exec-once = waybar-toggle -c ~/.config/waybar/config -s ~/.config/waybar/style.css -y 20 -p 50 -d 300 -e 200
```

### Sway

Add to `config`:
```conf
exec waybar-toggle -c ~/.config/waybar/config -s ~/.config/waybar/style.css
```

With custom threshold:
```conf
exec waybar-toggle -c ~/.config/waybar/config -s ~/.config/waybar/style.css -y 20
```

With custom poll interval:
```conf
exec waybar-toggle -c ~/.config/waybar/config -s ~/.config/waybar/style.css -y 20 -p 50
```

With hide delay:
```conf
exec waybar-toggle -c ~/.config/waybar/config -s ~/.config/waybar/style.css -y 20 -p 50 -d 300
```

With show delay:
```conf
exec waybar-toggle -c ~/.config/waybar/config -s ~/.config/waybar/style.css -y 20 -p 50 -d 300 -e 200
```

## How It Works

- On Hyprland: uses `hyprctl cursorpos` to get mouse position
- On Sway: uses `swaymsg -t get_seats` to get mouse position

When the Y-coordinate is ≤1 (top of screen), Waybar can be spawned. If a show delay is configured, the cursor must stay at the top for that duration before Waybar appears. When the cursor moves beyond the configured threshold (default: 7 pixels), Waybar waits for the hide delay (default: 250ms) and is terminated if the cursor is still beyond the threshold.

### Threshold Behavior

- **Show trigger**: Mouse Y-coordinate ≤ 1 pixel (always at screen edge) and stays there for `--show-delay-ms` if configured
- **Hide trigger**: Mouse Y-coordinate > threshold (configurable via `-y` flag)

This allows you to set a "safe zone" - for example, with `-y 20`, Waybar stays visible until your mouse is more than 20 pixels from the top of the screen.

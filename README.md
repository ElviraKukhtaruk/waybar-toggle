# waybar-toggle

A lightweight utility that automatically shows/hides [Waybar](https://github.com/Alexays/Waybar) when you move your mouse to the top of the screen.

## Features

- Automatically reveals Waybar when mouse reaches the top edge of the screen
- Hides Waybar when mouse moves away
- Works with both Hyprland and Sway compositors
- Minimal resource usage with 75ms polling interval
- Simple command-line configuration

## Requirements

- Rust (for building)
- Waybar installed on your system
- Either Hyprland or Sway compositor
- Required compositor tools:
  - `hyprctl` (for Hyprland)
  - `swaymsg` (for Sway)

## 1. Installation

```bash
cargo build --release
sudo cp target/release/waybar-toggle /usr/local/bin/
```

### Usage

```bash
waybar-toggle -c <config_path> -s <style_path>
```

#### Arguments

- `-c, --config` - Path to your Waybar config file
- `-s, --style` - Path to your Waybar CSS style file

### Example

```bash
waybar-toggle -c ~/.config/waybar/config -s ~/.config/waybar/style.css
```

## 2. Autostart

To start waybar-toggle automatically with your compositor:

### Hyprland

Add to `hyprland.conf`:

```conf
exec-once = waybar-toggle -c ~/.config/waybar/config -s ~/.config/waybar/style.css
```

### Sway

Add to `config`:

```conf
exec waybar-toggle -c ~/.config/waybar/config -s ~/.config/waybar/style.css
```

## How It Works
- On Hyprland: uses `hyprctl cursorpos`
- On Sway: uses `swaymsg -t get_seats`

When the Y-coordinate is ≤1 (top of screen), Waybar is spawned. When you move away, the process is terminated.

use std::process::Command;
use std::sync::{Mutex, OnceLock};
use std::thread;
use std::time::{Duration, Instant};
use std::{env, process};

fn main() {
    let (config_path, style_path, y_threshold, poll_ms, hide_delay_ms, show_delay_ms) =
        match parse_args() {
            Some(v) => v,
            None => {
                eprintln!(
                    "Usage: waybar-togle -c <config_path> -s <style_path> [-y <hide_threshold>] [-p <poll_ms>] [-d <hide_delay_ms>]\n\
Example: waybar-togle -c config -s style.css -y 10 -p 75 -d 250"
                );
                process::exit(2);
            }
        };

    println!("Waybar hover detector started");

    let mut mouse_was_at_top = false;
    let mut top_entered_at: Option<Instant> = None;

    loop {
        if let Some(y) = get_mouse_y() {
            if y <= 1 {
                if top_entered_at.is_none() {
                    top_entered_at = Some(Instant::now());
                }

                if !mouse_was_at_top {
                    let should_show = if show_delay_ms == 0 {
                        true
                    } else {
                        top_entered_at
                            .map(|t| t.elapsed() >= Duration::from_millis(show_delay_ms))
                            .unwrap_or(false)
                    };

                    if should_show {
                        show_waybar(&config_path, &style_path);
                        mouse_was_at_top = true;
                    }
                }
            } else {
                top_entered_at = None;

                if y > y_threshold && mouse_was_at_top {
                    if hide_delay_ms > 0 {
                        thread::sleep(Duration::from_millis(hide_delay_ms));
                        if let Some(y_after_delay) = get_mouse_y() {
                            if y_after_delay <= y_threshold {
                                continue;
                            }
                        }
                    }
                    hide_waybar();
                    mouse_was_at_top = false;
                }
            }
        }

        thread::sleep(Duration::from_millis(poll_ms));
    }
}

fn show_waybar(config_path: &str, style_path: &str) {
    let child = Command::new("waybar")
        .args(&["-c", config_path, "-s", style_path])
        .spawn();

    if let Ok(child) = child {
        let pid = child.id();
        if let Ok(mut guard) = waybar_pid().lock() {
            *guard = Some(pid);
        }
    }
}

fn hide_waybar() {
    let pid = if let Ok(mut guard) = waybar_pid().lock() {
        guard.take()
    } else {
        None
    };

    if let Some(pid) = pid {
        Command::new("kill").arg(pid.to_string()).spawn().ok();
    }
}

fn waybar_pid() -> &'static Mutex<Option<u32>> {
    static WAYBAR_PID: OnceLock<Mutex<Option<u32>>> = OnceLock::new();
    WAYBAR_PID.get_or_init(|| Mutex::new(None))
}

fn parse_args() -> Option<(String, String, i32, u64, u64, u64)> {
    let mut config_path: Option<String> = None;
    let mut style_path: Option<String> = None;
    let mut y_threshold: i32 = 7;
    let mut poll_ms: u64 = 75;
    let mut hide_delay_ms: u64 = 250;
    let mut show_delay_ms: u64 = 0;
    let mut args = env::args().skip(1);

    while let Some(arg) = args.next() {
        match arg.as_str() {
            "-c" | "--config" => {
                config_path = args.next();
            }
            "-s" | "--style" => {
                style_path = args.next();
            }
            "-y" | "--y-threshold" => {
                let next = args.next()?;
                y_threshold = next.parse::<i32>().ok()?;
            }
            "-p" | "--poll-ms" => {
                let next = args.next()?;
                poll_ms = next.parse::<u64>().ok()?;
            }
            "-d" | "--hide-delay-ms" => {
                let next = args.next()?;
                hide_delay_ms = next.parse::<u64>().ok()?;
            }
            "-e" | "--show-delay-ms" => {
                let next = args.next()?;
                show_delay_ms = next.parse::<u64>().ok()?;
            }
            _ => {}
        }
    }

    match (config_path, style_path) {
        (Some(c), Some(s)) => Some((c, s, y_threshold, poll_ms, hide_delay_ms, show_delay_ms)),
        _ => None,
    }
}

fn get_mouse_y() -> Option<i32> {
    // Try Hyprland first
    if let Ok(output) = Command::new("hyprctl").args(&["cursorpos"]).output() {
        if let Ok(pos) = String::from_utf8(output.stdout) {
            if let Some(y_str) = pos.split(',').nth(1) {
                if let Ok(y) = y_str.trim().parse::<i32>() {
                    return Some(y);
                }
            }
        }
    }

    // Try Sway
    if let Ok(output) = Command::new("swaymsg").args(&["-t", "get_seats"]).output() {
        if let Ok(json_str) = String::from_utf8(output.stdout) {
            if let Some(idx) = json_str.find("\"cursor_y\":") {
                let rest = &json_str[idx + 11..];
                if let Some(end) = rest.find(|c: char| !c.is_numeric()) {
                    if let Ok(y) = rest[..end].parse::<i32>() {
                        return Some(y);
                    }
                }
            }
        }
    }

    None
}

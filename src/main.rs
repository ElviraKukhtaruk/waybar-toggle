use std::process::Command;
use std::sync::{Mutex, OnceLock};
use std::thread;
use std::time::Duration;
use std::{env, process};

fn main() {
    let (config_path, style_path) = match parse_args() {
        Some(v) => v,
        None => {
            eprintln!(
                "Usage: waybar-togle -c <config_path> -s <style_path>\n\
Example: waybar-togle -c config -s style.css"
            );
            process::exit(2);
        }
    };

    println!("Waybar hover detector started");

    let mut mouse_was_at_top = false;

    loop {
        let mouse_at_top = is_mouse_at_top();

        if mouse_at_top && !mouse_was_at_top {
            show_waybar(&config_path, &style_path);
            mouse_was_at_top = true;
        } else if !mouse_at_top && mouse_was_at_top {
            hide_waybar();
            mouse_was_at_top = false;
        }

        thread::sleep(Duration::from_millis(75));
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

fn parse_args() -> Option<(String, String)> {
    let mut config_path: Option<String> = None;
    let mut style_path: Option<String> = None;
    let mut args = env::args().skip(1);

    while let Some(arg) = args.next() {
        match arg.as_str() {
            "-c" | "--config" => {
                config_path = args.next();
            }
            "-s" | "--style" => {
                style_path = args.next();
            }
            _ => {}
        }
    }

    match (config_path, style_path) {
        (Some(c), Some(s)) => Some((c, s)),
        _ => None,
    }
}

fn is_mouse_at_top() -> bool {
    // Try Hyprland first
    if let Ok(output) = Command::new("hyprctl").args(&["cursorpos"]).output() {
        if let Ok(pos) = String::from_utf8(output.stdout) {
            if let Some(y_str) = pos.split(',').nth(1) {
                if let Ok(y) = y_str.trim().parse::<i32>() {
                    return y <= 1;
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
                        return y <= 1;
                    }
                }
            }
        }
    }

    false
}

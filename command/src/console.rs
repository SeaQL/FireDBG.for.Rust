#![allow(dead_code)]

use anstyle::*;

pub const NOP: Style = Style::new();
pub const HEADER: Style = AnsiColor::Green.on_default().effects(Effects::BOLD);
pub const USAGE: Style = AnsiColor::Green.on_default().effects(Effects::BOLD);
pub const LITERAL: Style = AnsiColor::Cyan.on_default().effects(Effects::BOLD);
pub const PLACEHOLDER: Style = AnsiColor::Cyan.on_default();
pub const ERROR: Style = AnsiColor::Red.on_default().effects(Effects::BOLD);
pub const WARN: Style = AnsiColor::Yellow.on_default().effects(Effects::BOLD);
pub const NOTE: Style = AnsiColor::Cyan.on_default().effects(Effects::BOLD);
pub const GOOD: Style = AnsiColor::Green.on_default().effects(Effects::BOLD);
pub const VALID: Style = AnsiColor::Cyan.on_default().effects(Effects::BOLD);
pub const INVALID: Style = AnsiColor::Yellow.on_default().effects(Effects::BOLD);

pub fn status(status: &str, message: &str) {
    print(status, message, &HEADER, true);
}

pub fn warn(status: &str, message: &str) {
    print(status, message, &WARN, true);
}

pub fn print(status: &str, message: &str, style: &Style, justified: bool) {
    let style = style.render();
    let bold = (Style::new() | Effects::BOLD).render();
    let reset = Reset.render();

    if justified {
        eprint!("{style}{status:>12}{reset}");
    } else {
        eprint!("{style}{status}{reset}{bold}:{reset}");
    }
    eprintln!(" {message}")
}

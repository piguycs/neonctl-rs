#![allow(unused)]

// mimics the default style for the cargo cli
// source: https://crates.io/crates/clap-cargo

use anstyle::{AnsiColor, Effects, Style};

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

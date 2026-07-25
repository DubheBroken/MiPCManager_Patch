//! TUI 主题系统：Tokyo Night Dark 配色 + 全局样式常量。

use ratatui::style::{Color, Modifier, Style};

// ── Core palette ──────────────────────────────────────────────────
pub const BG: Color = Color::Rgb(26, 27, 38);
pub const SURFACE: Color = Color::Rgb(36, 40, 59);
pub const OVERLAY: Color = Color::Rgb(65, 72, 104);
pub const TEXT: Color = Color::Rgb(192, 202, 245);
pub const DIM: Color = Color::Rgb(86, 95, 137);
pub const MUTED: Color = Color::Rgb(169, 177, 214);

pub const ACCENT: Color = Color::Rgb(122, 162, 247);
pub const ACCENT_DIM: Color = Color::Rgb(61, 81, 124);
pub const GREEN: Color = Color::Rgb(158, 206, 106);
pub const RED: Color = Color::Rgb(247, 118, 142);
pub const YELLOW: Color = Color::Rgb(224, 175, 104);
pub const PURPLE: Color = Color::Rgb(187, 154, 247);
pub const CYAN: Color = Color::Rgb(125, 207, 255);

// ── Structural styles ─────────────────────────────────────────────
pub fn header_title() -> Style {
    Style::default().fg(Color::White).bg(ACCENT_DIM).add_modifier(Modifier::BOLD)
}

pub fn header_status_icon(ok: bool) -> Style {
    if ok {
        Style::default().fg(GREEN).bg(ACCENT_DIM).add_modifier(Modifier::BOLD)
    } else {
        Style::default().fg(YELLOW).bg(ACCENT_DIM).add_modifier(Modifier::BOLD)
    }
}

pub fn tab_active(selected: bool) -> Style {
    if selected {
        Style::default()
            .fg(Color::White)
            .bg(ACCENT)
            .add_modifier(Modifier::BOLD)
    } else {
        Style::default().fg(DIM)
    }
}

pub fn border_default() -> Style {
    Style::default().fg(OVERLAY)
}

pub fn border_focused() -> Style {
    Style::default().fg(ACCENT)
}

pub fn item_selected() -> Style {
    Style::default().fg(Color::Black).bg(ACCENT)
}

pub fn item_normal() -> Style {
    Style::default().fg(TEXT)
}

pub fn item_hint() -> Style {
    Style::default().fg(DIM)
}

pub fn button_normal() -> Style {
    Style::default().fg(MUTED)
}

pub fn button_hover() -> Style {
    Style::default()
        .fg(Color::Black)
        .bg(YELLOW)
        .add_modifier(Modifier::BOLD)
}

pub fn shortcut_bar() -> Style {
    Style::default().fg(MUTED).bg(SURFACE)
}

pub fn mini_log_success() -> Style {
    Style::default().fg(GREEN)
}

pub fn mini_log_error() -> Style {
    Style::default().fg(RED)
}

pub fn mini_log_info() -> Style {
    Style::default().fg(MUTED)
}

pub fn overlay_bg() -> Style {
    Style::default().bg(SURFACE).fg(TEXT)
}

pub fn overlay_border() -> Style {
    Style::default().fg(YELLOW)
}

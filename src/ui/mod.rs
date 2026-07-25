//! 前端入口集合。
//!
//! - `tui` — ratatui 终端全屏界面（无参数启动时进入）
//! - `gui/app.rs` — Slint 声明式 GUI 二进制入口（`MiPCM_GUI`）

#[cfg(feature = "cli")]
pub mod tui;

//! ratatui TUI 前端：终端可视化界面。
//!
//! 无参数启动时自动进入 crossterm raw mode + alternate screen。
//! 补丁操作通过 `std::thread::spawn` 异步执行，不阻塞界面响应。
//! Tokyo Night Dark 配色，Tab 式面板切换，丰富的视觉反馈。

mod app;
mod theme;
mod widgets;

use anyhow::Result;
use crossterm::{
    execute,
    terminal::{EnterAlternateScreen, LeaveAlternateScreen, disable_raw_mode, enable_raw_mode},
};
use std::sync::mpsc;

use crate::i18n;

use self::app::App;

pub fn run() -> Result<()> {
    enable_raw_mode()?;
    let mut stdout = std::io::stdout();
    execute!(stdout, EnterAlternateScreen)?;

    let lang = i18n::detect_lang();
    let (tx, rx) = mpsc::channel::<app::LogMessage>();

    let mut app = App::new(tx, lang);

    let backend = ratatui::backend::CrosstermBackend::new(stdout);
    let mut terminal = ratatui::Terminal::new(backend)?;

    let result = app.run_loop(&mut terminal, rx);

    disable_raw_mode()?;
    execute!(terminal.backend_mut(), LeaveAlternateScreen)?;
    terminal.show_cursor()?;

    result
}

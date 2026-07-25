//! TUI 可复用渲染构件。

use ratatui::{
    layout::{Alignment, Constraint, Layout, Rect},
    style::{Color, Modifier, Style},
    text::{Line, Span, Text},
    widgets::{Block, Borders, Paragraph, Tabs},
    Frame,
};

use super::theme;

pub fn draw_tabs(f: &mut Frame, area: Rect, titles: &[&str], active: usize) {
    let items: Vec<Line> = titles
        .iter()
        .enumerate()
        .map(|(i, t)| {
            Line::from(Span::styled(
                format!("  {t}  "),
                theme::tab_active(i == active),
            ))
        })
        .collect();
    f.render_widget(
        Tabs::new(items)
            .block(Block::default().style(Style::default().bg(theme::SURFACE)))
            .divider(Span::from(" "))
            .highlight_style(theme::tab_active(true)),
        area,
    );
}

pub fn draw_content_block<'a>(title: &'a str, focused: bool) -> Block<'a> {
    let style = if focused {
        theme::border_focused()
    } else {
        theme::border_default()
    };
    Block::default()
        .borders(Borders::ALL)
        .border_style(style)
        .title(format!(" {title} "))
        .title_style(if focused {
            Style::default()
                .fg(theme::ACCENT)
                .add_modifier(Modifier::BOLD)
        } else {
            Style::default().fg(theme::DIM)
        })
        .style(Style::default().bg(theme::BG))
}

#[allow(clippy::too_many_arguments)]
pub fn draw_patch_row(
    area: Rect,
    f: &mut Frame,
    name: &str,
    desc: &str,
    is_selected: bool,
    buttons: &[(bool, &str)],
    hovered_btn: usize,
    extra: Option<&[Line]>,
) {
    let bg_style = if is_selected {
        Style::default().bg(theme::SURFACE)
    } else {
        Style::default()
    };
    f.render_widget(Block::default().style(bg_style), area);

    let chunks = Layout::default()
        .direction(ratatui::layout::Direction::Vertical)
        .constraints([Constraint::Length(1), Constraint::Length(1)])
        .split(area);

    let marker = if is_selected { " ▸" } else { "  " };
    let name_span = Span::styled(
        format!("{marker}{name}"),
        if is_selected {
            Style::default().fg(theme::ACCENT).add_modifier(Modifier::BOLD)
        } else {
            Style::default().fg(theme::TEXT)
        },
    );

    let mut row_spans = vec![name_span];
    if !is_selected {
        row_spans.push(Span::styled("  —  ", theme::item_hint()));
        row_spans.push(Span::styled(desc, theme::item_hint()));
    } else {
        for (bi, (_, label)) in buttons.iter().enumerate() {
            row_spans.push(Span::styled("   ", Style::default()));
            let btn_style = if bi == hovered_btn {
                theme::button_hover()
            } else {
                theme::button_normal()
            };
            row_spans.push(Span::styled(format!("[{label}]"), btn_style));
        }
    }

    f.render_widget(Paragraph::new(Line::from(row_spans)), chunks[0]);

    if let Some(lines) = extra {
        let info_area = Layout::default()
            .constraints([Constraint::Min(0)])
            .horizontal_margin(4)
            .split(chunks[1]);
        f.render_widget(Paragraph::new(Text::from(lines)), info_area[0]);
    }
}

pub fn draw_log_panel(f: &mut Frame, area: Rect, log: &[String], scroll: usize, focused: bool) {
    let block = draw_content_block("日志", focused);
    let inner = block.inner(area);
    f.render_widget(block, area);

    let visible = inner.height as usize;
    if log.is_empty() {
        f.render_widget(
            Paragraph::new("— no logs yet —")
                .style(theme::item_hint())
                .alignment(Alignment::Center),
            inner,
        );
        return;
    }

    let total = log.len();
    let start = scroll
        .saturating_sub(visible.saturating_sub(1))
        .min(total.saturating_sub(visible));
    let end = (start + visible).min(total);

    let items: Vec<Line> = log[start..end]
        .iter()
        .enumerate()
        .map(|(i, line)| {
            let idx = start + i;
            let is_cursor = focused && idx == scroll;
            let base = if line.contains("✓") || line.contains("成功") {
                theme::mini_log_success()
            } else if line.contains("✗") || line.contains("失败") || line.contains("Error") {
                theme::mini_log_error()
            } else if line.contains("⚠") {
                Style::default().fg(theme::YELLOW)
            } else if line.starts_with("——") {
                Style::default().fg(theme::CYAN)
            } else {
                theme::mini_log_info()
            };

            let style = if is_cursor {
                Style::default()
                    .fg(Color::Black)
                    .bg(theme::ACCENT)
                    .add_modifier(Modifier::BOLD)
            } else {
                base
            };

            Line::from(Span::styled(
                if line.len() > inner.width as usize {
                    &line[..inner.width as usize]
                } else {
                    line.as_str()
                },
                style,
            ))
        })
        .collect();

    f.render_widget(Paragraph::new(Text::from(items)), inner);

    if total > visible {
        let pct = ((scroll as f32 / total as f32) * 100.0) as usize;
        let indicator = format!("{pct}%");
        f.render_widget(
            Paragraph::new(indicator)
                .style(theme::item_hint())
                .alignment(Alignment::Right),
            Rect::new(
                inner.x,
                inner.y + inner.height.saturating_sub(1),
                inner.width,
                1,
            ),
        );
    }
}

pub fn draw_mini_log(
    f: &mut Frame,
    area: Rect,
    log: &[String],
) {
    let block = Block::default()
        .borders(Borders::TOP)
        .border_style(theme::border_default())
        .style(Style::default().bg(theme::BG));
    let inner = block.inner(area);
    f.render_widget(block, area);

    let visible = inner.height as usize;
    if log.is_empty() {
        f.render_widget(
            Paragraph::new("").style(theme::item_hint()),
            inner,
        );
        return;
    }

    let total = log.len();
    let start = total.saturating_sub(visible);
    let end = total;

    let items: Vec<Line> = log[start..end]
        .iter()
        .map(|line| {
            let base = if line.contains("✓") || line.contains("成功") {
                theme::mini_log_success()
            } else if line.contains("✗") || line.contains("失败") || line.contains("Error") {
                theme::mini_log_error()
            } else if line.contains("⚠") {
                Style::default().fg(theme::YELLOW)
            } else {
                theme::mini_log_info()
            };
            Line::from(Span::styled(
                format!(" {}", if line.len() > inner.width.saturating_sub(2) as usize { &line[..inner.width.saturating_sub(2) as usize] } else { line }),
                base,
            ))
        })
        .collect();

    f.render_widget(Paragraph::new(Text::from(items)), inner);

    if total > 0 {
        let indicator = format!("{}/{}", (total - 1).min(end), total);
        f.render_widget(
            Paragraph::new(indicator)
                .style(theme::item_hint())
                .alignment(Alignment::Right),
            Rect::new(
                inner.x,
                inner.y + inner.height.saturating_sub(1),
                inner.width,
                1,
            ),
        );
    }
}

pub fn draw_confirm_overlay(
    f: &mut Frame,
    parent: Rect,
    title: &str,
    message: &str,
    hint: &str,
) {
    let w = message.lines().map(|l| l.len()).max().unwrap_or(40).min(70) as u16 + 8;
    let h = (message.lines().count() + 4) as u16;
    let x = parent.x + (parent.width.saturating_sub(w)) / 2;
    let y = parent.y + (parent.height.saturating_sub(h)) / 2;
    let popup = Rect::new(x, y, w.min(parent.width), h.min(parent.height));

    let block = Block::default()
        .borders(Borders::ALL)
        .border_style(theme::overlay_border())
        .title(format!(" {title} "))
        .style(theme::overlay_bg());

    f.render_widget(block, popup);
    let inner = popup.inner(ratatui::layout::Margin {
        horizontal: 2,
        vertical: 1,
    });

    let lines: Vec<Line> = message
        .lines()
        .map(|l| Line::from(Span::styled(l, theme::overlay_bg())))
        .chain(std::iter::once(Line::from("")))
        .chain(std::iter::once(Line::from(Span::styled(
            hint,
            Style::default().fg(theme::YELLOW).bg(theme::SURFACE),
        ))))
        .collect();

    f.render_widget(Paragraph::new(Text::from(lines)), inner);
}

pub fn draw_input_overlay(
    f: &mut Frame,
    parent: Rect,
    title: &str,
    buffer: &str,
    hint: &str,
) {
    let w = 52u16;
    let h = 6u16;
    let x = parent.x + (parent.width.saturating_sub(w)) / 2;
    let y = parent.y + (parent.height.saturating_sub(h)) / 2;
    let popup = Rect::new(x, y, w.min(parent.width), h.min(parent.height));

    let block = Block::default()
        .borders(Borders::ALL)
        .border_style(theme::overlay_border())
        .title(format!(" {title} "))
        .style(theme::overlay_bg());

    f.render_widget(block, popup);
    let inner = popup.inner(ratatui::layout::Margin {
        horizontal: 2,
        vertical: 1,
    });

    let display = format!("▸ {buffer}{}", if buffer.is_empty() { "█" } else { "" });
    f.render_widget(
        Paragraph::new(display).style(theme::overlay_bg()),
        Rect::new(inner.x + 1, inner.y, inner.width.saturating_sub(2), 1),
    );

    f.render_widget(
        Paragraph::new(hint)
            .style(theme::item_hint())
            .alignment(Alignment::Center),
        Rect::new(inner.x, inner.y + 2, inner.width, 1),
    );
}

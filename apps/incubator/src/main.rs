// apps/incubator/src/main.rs
use std::{io, time::Duration};

use crossterm::{
    event::{self, Event, KeyCode},
    terminal,
};
use opentui_rust::{Rgba, Renderer, Style};

const IDEAS: &[(&str, &str, &str, u8)] = &[
    ("AI-001", "AI Router", "AI", 30),
    ("AI-002", "LLM TUI", "AI", 29),
    ("DATA-001", "HardwareBench", "Data", 28),
    ("DEV-001", "GitDash", "Development", 28),
    ("OFF-001", "SolarTUI", "OffGrid", 27),
    ("OPS-001", "DockerTUI", "Operations", 27),
    ("SYS-001", "SysPeek", "System", 27),
    ("PROD-001", "TaskForge", "Productivity", 26),
];

fn main() -> io::Result<()> {
    terminal::enable_raw_mode()?;
    let result = run();
    terminal::disable_raw_mode()?;
    result
}

fn run() -> io::Result<()> {
    let (width, height) = terminal::size()?;
    let mut renderer = Renderer::new(width as usize, height as usize)?;
    let mut selected = 0usize;

    loop {
        let (width, height) = terminal::size()?;
        renderer.resize(width as usize, height as usize)?;
        draw(&mut renderer, selected, width as usize, height as usize)?;

        if event::poll(Duration::from_millis(100))? {
            if let Event::Key(key) = event::read()? {
                match key.code {
                    KeyCode::Char('q') | KeyCode::Esc => break,
                    KeyCode::Down | KeyCode::Char('j') => {
                        selected = (selected + 1) % IDEAS.len();
                    }
                    KeyCode::Up | KeyCode::Char('k') => {
                        selected = selected.checked_sub(1).unwrap_or(IDEAS.len() - 1);
                    }
                    _ => {}
                }
            }
        }
    }

    Ok(())
}

fn draw(renderer: &mut Renderer, selected: usize, width: usize, height: usize) -> io::Result<()> {
    let buffer = renderer.buffer();
    buffer.clear(Rgba::from_hex("#0b1020").unwrap_or(Rgba::BLACK));

    let title = " RUST OPENTUI IDEA INCUBATOR ";
    buffer.draw_text(1, 1, title, Style::bold().fg(Rgba::WHITE));
    buffer.draw_text(1, 2, "=".repeat(width.saturating_sub(2)).as_str(), Style::fg(Rgba::from_hex("#667085").unwrap()));

    buffer.draw_text(2, 4, "CATEGORIES", Style::bold().fg(Rgba::WHITE));
    let categories = ["AI", "Data", "Development", "OffGrid", "Operations", "Productivity", "System"];
    for (index, category) in categories.iter().enumerate() {
        let marker = if *category == IDEAS[selected].2 { ">" } else { " " };
        buffer.draw_text(2, 6 + index, &format!("{marker} {category}"), Style::fg(Rgba::WHITE));
    }

    let list_x = 24usize;
    buffer.draw_text(list_x, 4, "IDEAS", Style::bold().fg(Rgba::WHITE));
    for (index, (id, name, category, score)) in IDEAS.iter().enumerate() {
        let marker = if index == selected { ">" } else { " " };
        let line = format!("{marker} {id:<9} {name:<20} {category:<13} {score:>2}/30");
        buffer.draw_text(list_x, 6 + index, &line, Style::fg(Rgba::WHITE));
    }

    let detail_y = 16usize.min(height.saturating_sub(7));
    buffer.draw_text(2, detail_y, "SELECTED IDEA", Style::bold().fg(Rgba::WHITE));
    buffer.draw_text(2, detail_y + 2, IDEAS[selected].0, Style::bold().fg(Rgba::WHITE));
    buffer.draw_text(12, detail_y + 2, IDEAS[selected].1, Style::fg(Rgba::WHITE));
    buffer.draw_text(2, detail_y + 4, "[E] Experiment  [R] Research  [P] Prototype  [S] Score", Style::fg(Rgba::WHITE));

    let footer_y = height.saturating_sub(2);
    buffer.draw_text(1, footer_y, "↑↓/jk Navigate   Enter Open   / Search   ? Help   Q Quit", Style::fg(Rgba::WHITE));
    renderer.present()?;
    Ok(())
}

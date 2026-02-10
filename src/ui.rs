use crate::app::{App, FocusedColumn};
use hlavi_core::TaskStatus;
use ratatui::{
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, List, ListItem, Paragraph},
    Frame,
};

pub fn draw(f: &mut Frame, app: &App) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Length(3), Constraint::Min(0)])
        .split(f.size());

    // Header
    let header = Paragraph::new("Hlavi TUI - Kanban Board")
        .style(
            Style::default()
                .fg(Color::Cyan)
                .add_modifier(Modifier::BOLD),
        )
        .block(Block::default().borders(Borders::ALL));
    f.render_widget(header, chunks[0]);

    // Board columns
    let board_chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Percentage(25),
            Constraint::Percentage(25),
            Constraint::Percentage(25),
            Constraint::Percentage(25),
        ])
        .split(chunks[1]);

    draw_column(
        f,
        board_chunks[0],
        "OPEN",
        app,
        FocusedColumn::Open,
        TaskStatus::Open,
    );
    draw_column(
        f,
        board_chunks[1],
        "IN PROGRESS",
        app,
        FocusedColumn::InProgress,
        TaskStatus::InProgress,
    );
    draw_column(
        f,
        board_chunks[2],
        "REVIEW",
        app,
        FocusedColumn::Review,
        TaskStatus::Review,
    );
    draw_column(
        f,
        board_chunks[3],
        "DONE",
        app,
        FocusedColumn::Done,
        TaskStatus::Done,
    );

    // Help text
    let help_chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Min(0), Constraint::Length(1)])
        .split(f.size());

    let help_text = Paragraph::new("h/l: ← →  j/k: ↑ ↓  r: reload  q/ESC/Ctrl+C: quit")
        .style(Style::default().fg(Color::DarkGray));
    f.render_widget(help_text, help_chunks[1]);
}

fn draw_column(
    f: &mut Frame,
    area: Rect,
    title: &str,
    app: &App,
    column: FocusedColumn,
    status: TaskStatus,
) {
    let is_focused = app.focused_column() == column;
    let tasks = app.tasks_in_column(status);

    let items: Vec<ListItem> = tasks
        .iter()
        .enumerate()
        .map(|(i, task)| {
            let is_selected = is_focused && i == app.selected_index();
            let style = if is_selected {
                Style::default()
                    .fg(Color::Black)
                    .bg(Color::Yellow)
                    .add_modifier(Modifier::BOLD)
            } else {
                Style::default()
            };

            let completed = task
                .acceptance_criteria
                .iter()
                .filter(|ac| ac.completed)
                .count();
            let total = task.acceptance_criteria.len();

            let content = vec![
                Line::from(vec![
                    Span::styled(task.id.to_string(), Style::default().fg(Color::Cyan)),
                    Span::raw(" "),
                    Span::raw(&task.title),
                ]),
                Line::from(vec![Span::styled(
                    format!("  ✓ {}/{}", completed, total),
                    Style::default().fg(Color::Green),
                )]),
            ];

            ListItem::new(content).style(style)
        })
        .collect();

    let border_style = if is_focused {
        Style::default().fg(Color::Yellow)
    } else {
        Style::default().fg(Color::White)
    };

    let list = List::new(items).block(
        Block::default()
            .borders(Borders::ALL)
            .border_style(border_style)
            .title(format!(" {} ({}) ", title, tasks.len())),
    );

    f.render_widget(list, area);
}

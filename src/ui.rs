use crate::app::{App, FocusedColumn};
use hlavi_core::TicketStatus;
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
        .style(Style::default().fg(Color::Cyan).add_modifier(Modifier::BOLD))
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
        TicketStatus::Open,
    );
    draw_column(
        f,
        board_chunks[1],
        "IN PROGRESS",
        app,
        FocusedColumn::InProgress,
        TicketStatus::InProgress,
    );
    draw_column(
        f,
        board_chunks[2],
        "REVIEW",
        app,
        FocusedColumn::Review,
        TicketStatus::Review,
    );
    draw_column(
        f,
        board_chunks[3],
        "DONE",
        app,
        FocusedColumn::Done,
        TicketStatus::Done,
    );

    // Help text
    let help_chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Min(0), Constraint::Length(1)])
        .split(f.size());

    let help_text = Paragraph::new("h/l: ← →  j/k: ↑ ↓  r: reload  q: quit")
        .style(Style::default().fg(Color::DarkGray));
    f.render_widget(help_text, help_chunks[1]);
}

fn draw_column(
    f: &mut Frame,
    area: Rect,
    title: &str,
    app: &App,
    column: FocusedColumn,
    status: TicketStatus,
) {
    let is_focused = app.focused_column() == column;
    let tickets = app.tickets_in_column(status);

    let items: Vec<ListItem> = tickets
        .iter()
        .enumerate()
        .map(|(i, ticket)| {
            let is_selected = is_focused && i == app.selected_index();
            let style = if is_selected {
                Style::default()
                    .fg(Color::Black)
                    .bg(Color::Yellow)
                    .add_modifier(Modifier::BOLD)
            } else {
                Style::default()
            };

            let completed = ticket
                .acceptance_criteria
                .iter()
                .filter(|ac| ac.completed)
                .count();
            let total = ticket.acceptance_criteria.len();

            let content = vec![
                Line::from(vec![
                    Span::styled(ticket.id.to_string(), Style::default().fg(Color::Cyan)),
                    Span::raw(" "),
                    Span::raw(&ticket.title),
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
            .title(format!(" {} ({}) ", title, tickets.len())),
    );

    f.render_widget(list, area);
}

use crossterm::{
    ExecutableCommand,
    event::{self, Event, KeyCode},
    terminal::{EnterAlternateScreen, LeaveAlternateScreen, disable_raw_mode, enable_raw_mode},
};
use humansize::{BINARY, format_size};
use ratatui::{
    prelude::*,
    widgets::{Block, Borders, Paragraph, Row, Table, Wrap},
};
use std::collections::HashMap;
use std::io::{self, stdout};
use sysinfo::{Pid, System};

use crate::app::{App, SortBy};
use crate::process_data::ProcessData;

pub type Tui = Terminal<CrosstermBackend<io::Stdout>>;

pub fn init() -> io::Result<Tui> {
    stdout().execute(EnterAlternateScreen)?;
    enable_raw_mode()?;
    Terminal::new(CrosstermBackend::new(stdout()))
}

pub fn restore() -> io::Result<()> {
    stdout().execute(LeaveAlternateScreen)?;
    disable_raw_mode()?;
    Ok(())
}

pub fn handle_events(app: &mut App) -> io::Result<bool> {
    if event::poll(std::time::Duration::from_millis(50))? {
        if let Event::Key(key) = event::read()? {
            if key.kind == event::KeyEventKind::Press {
                match key.code {
                    KeyCode::Char('q') => return Ok(true),
                    KeyCode::Char('c') => app.set_sort_by(SortBy::Cpu),
                    KeyCode::Char('m') => app.set_sort_by(SortBy::Memory),
                    KeyCode::Char('p') => app.set_sort_by(SortBy::Pid),
                    KeyCode::Char('n') => app.set_sort_by(SortBy::Name),
                    _ => {}
                }
            }
        }
    }
    Ok(false)
}

pub fn draw_ui(
    frame: &mut Frame,
    app: &mut App,
    sys: &System,
    process_map: &HashMap<Pid, ProcessData>,
) {
    let layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints(vec![Constraint::Min(0), Constraint::Length(5)])
        .split(frame.size());

    draw_process_table(frame, app, sys, process_map, layout[0]);
    draw_alerts(frame, app, layout[1]);
}

fn draw_process_table(
    frame: &mut Frame,
    app: &App,
    sys: &System,
    process_map: &HashMap<Pid, ProcessData>,
    area: Rect,
) {
    let header = ["PID", "Name", "CPU %", "Memory", "Avg CPU", "Avg Mem"];
    let header_style = Style::default().fg(Color::Yellow);
    let rows = app.processes.iter().map(|pid| {
        let process = sys.process(*pid).unwrap();
        let (avg_cpu, avg_mem) = match process_map.get(pid) {
            Some(data) => (data.cpu_average(), data.memory_average()),
            None => (0.0, 0),
        };
        Row::new(vec![
            process.pid().to_string(),
            process.name().to_string(),
            format!("{:.2}", process.cpu_usage()),
            format_size(process.memory(), BINARY),
            format!("{:.2}", avg_cpu),
            format_size(avg_mem, BINARY),
        ])
    });

    let table = Table::new(
        rows,
        [
            Constraint::Length(10),
            Constraint::Length(30),
            Constraint::Length(10),
            Constraint::Length(10),
            Constraint::Length(15),
            Constraint::Length(15),
        ],
    )
    .header(Row::new(header).style(header_style))
    .block(
        Block::default()
            .title("Galactic Tamer (c: cpu, m: mem, p: pid, n: name, q: quit)")
            .borders(Borders::ALL),
    )
    .highlight_style(Style::default().add_modifier(Modifier::REVERSED))
    .widths(&[
        Constraint::Percentage(5),
        Constraint::Percentage(35),
        Constraint::Percentage(10),
        Constraint::Percentage(10),
        Constraint::Percentage(15),
        Constraint::Percentage(15),
    ]);

    frame.render_widget(table, area);
}

fn draw_alerts(frame: &mut Frame, app: &App, area: Rect) {
    let alerts_text: Vec<Line> = app
        .alerts
        .iter()
        .map(|alert| Line::from(alert.as_str()))
        .collect();

    let alerts_paragraph = Paragraph::new(alerts_text)
        .block(Block::default().title("Alerts").borders(Borders::ALL))
        .wrap(Wrap { trim: true });

    frame.render_widget(alerts_paragraph, area);
}

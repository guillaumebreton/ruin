use crate::model::{Account, Service, Transaction};
use crossterm::{
    event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use std::{
    error::Error,
    io,
    time::{Duration, Instant},
};
use tui::{
    backend::{Backend, CrosstermBackend},
    layout::{Constraint, Layout},
    style::{Color, Modifier, Style},
    widgets::{Block, Borders, Cell, Row, Table, TableState},
    Frame, Terminal,
};
struct State {
    state: TableState,
    items: Vec<(Transaction, Account)>,
}

impl State {
    pub fn next(&mut self) {
        let i = match self.state.selected() {
            Some(i) => {
                if i >= self.items.len() - 1 {
                    0
                } else {
                    i + 1
                }
            }
            None => 0,
        };
        self.state.select(Some(i));
    }

    pub fn previous(&mut self) {
        let i = match self.state.selected() {
            Some(i) => {
                if i == 0 {
                    self.items.len() - 1
                } else {
                    i - 1
                }
            }
            None => 0,
        };
        self.state.select(Some(i));
    }
}

pub fn run_app<B: Backend>(terminal: &mut Terminal<B>, service: Service) -> io::Result<()> {
    let transactions = service.list_transactions().unwrap();

    let mut state = State {
        state: TableState::default(),
        items: transactions,
    };
    loop {
        terminal.draw(|f| ui(f, &mut state))?;

        if let Event::Key(key) = event::read()? {
            match key.code {
                KeyCode::Char('q') => return Ok(()),
                KeyCode::Down => state.next(),
                KeyCode::Up => state.previous(),
                _ => {}
            }
        }
    }
}

fn ui<B: Backend>(f: &mut Frame<B>, state: &mut State) {
    let rects = Layout::default()
        .constraints([Constraint::Percentage(100)].as_ref())
        .split(f.size());

    let selected_style = Style::default().add_modifier(Modifier::REVERSED);
    let normal_style = Style::default().bg(Color::Blue);
    let header_cells = ["ID", "Date", "Account", "Amount", "Description"]
        .iter()
        .map(|h| Cell::from(*h).style(Style::default().fg(Color::Red)));
    let header = Row::new(header_cells)
        .style(normal_style)
        .height(1)
        .bottom_margin(1);
    let rows = state.items.iter().map(|item| {
        let cells = vec![
            item.0.id.to_string(),
            item.0.date_posted.to_string(),
            item.1.account_number.clone(),
            ((item.0.transaction_amount as f64) / 100.0).to_string(),
            item.0.description.clone(),
        ];
        Row::new(cells).bottom_margin(0)
    });
    let t = Table::new(rows)
        .header(header)
        .block(Block::default().borders(Borders::ALL).title("Transactions"))
        .highlight_style(selected_style)
        .widths(&[
            Constraint::Length(2),
            Constraint::Length(12),
            Constraint::Min(20),
            Constraint::Min(20),
            Constraint::Percentage(100),
        ]);
    f.render_stateful_widget(t, rects[0], &mut state.state);
}

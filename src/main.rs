use crossterm::{
    event::{self, DisableMouseCapture, Event, KeyCode},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen},
};
use std::{error::Error,io};
use tui::{
    backend::{Backend, CrosstermBackend},
    layout::{Constraint, Direction, Layout},
    style::{Color, Modifier, Style},
    text::{Span, Spans, Text},
    widgets::{Block, Borders, List, ListItem, Paragraph, Tabs},
    Frame, Terminal,
};

enum Mode {
    Editing,
    Normal,
}

struct Task {
    name: Vec<String>,
    input: String,
    mode: Mode,
}

impl Default for Task {
    fn default() -> Task {
        Task {
            name: Vec::new(),
            input: String::new(),
            mode: Mode::Normal,
        }
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    enable_raw_mode().expect("Can't enable raw mode");
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen, DisableMouseCapture)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;
    let mut task = Task::default();
    run_app(&mut terminal, task);
    disable_raw_mode()?;
    Ok(())
}

fn ui<B: Backend>(f: &mut Frame<B>, task: &Task) {
    let size = f.size();
    // build a maintain for futures tabs menu
    let tabs = vec!["Home"];
    let menu = tabs
        .iter()
        .map(|m| {
            let (first, second) = m.split_at(1);
            Spans::from(vec![
                Span::styled(first, Style::default().fg(Color::White)),
                Span::styled(second, Style::default().fg(Color::White)),
            ])
        })
        .collect();
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .margin(2)
        .constraints([Constraint::Percentage(50), Constraint::Min(10)].as_ref())
        .split(size);
    let tabs = Tabs::new(menu)
        .select(0)
        .block(
            Block::default()
                .style(Style::default().fg(Color::Cyan))
                .title("Menu")
                .borders(Borders::ALL),
        )
        .highlight_style(Style::default())
        .divider(Span::raw("|"));
    f.render_widget(tabs, size);
    let input_chunks = Layout::default()
        .direction(Direction::Vertical)
        .margin(2)
        .constraints(
            [
                Constraint::Length(1),
                Constraint::Length(3),
                Constraint::Min(1),
            ]
            .as_ref(),
        )
        .split(chunks[0]);
    let (msg, style) = match task.mode {
        Mode::Normal => (
            vec![
                Span::raw("Press "),
                Span::styled("q", Style::default().add_modifier(Modifier::BOLD)),
                Span::raw(" to exit, "),
                Span::styled("a", Style::default().add_modifier(Modifier::BOLD)),
                Span::raw(" to add task"),
            ],
            Style::default(),
        ),
        Mode::Editing => (
            vec![
                Span::raw("Press "),
                Span::styled(" Esc", Style::default().add_modifier(Modifier::BOLD)),
                Span::raw(" to stop adding, "),
                Span::styled("Enter", Style::default().add_modifier(Modifier::BOLD)),
                Span::raw(" to add task in todo list"),
            ],
            Style::default(),
        ),
    };
    let mut text = Text::from(Spans::from(msg));
    text.patch_style(style);
    let help_message = Paragraph::new(text);
    f.render_widget(help_message, chunks[0]);
    let input = Paragraph::new(task.input.as_ref())
        .style(match task.mode {
            Mode::Normal => Style::default(),
            Mode::Editing => Style::default().fg(Color::Yellow),
        })
        .block(Block::default().borders(Borders::ALL).title("Input"));
    f.render_widget(input, input_chunks[1]);
    let messages: Vec<ListItem> = task
        .name
        .iter()
        .enumerate()
        .map(|(i, m)| {
            let content = vec![Spans::from(Span::raw(format!("{}: {}", i, m)))];
            ListItem::new(content)
        })
        .collect();
    let messages = List::new(messages).block(Block::default().borders(Borders::ALL).title("Tasks"));
    f.render_widget(messages, chunks[1])

    // let mut default_menu = Menu::Home;
}

fn run_app<B: Backend>(terminal: &mut Terminal<B>, mut task: Task) -> io::Result<()> {
    loop {
        terminal.draw(|f| ui(f, &task ))?;
        if let Event::Key(key) = event::read()? {
            match task.mode {
                Mode::Normal => match key.code {
                    KeyCode::Char('a') => task.mode = Mode::Editing,
                    KeyCode::Char('q') => {
                        return Ok(());
                    }
                    _ => {}
                },
                Mode::Editing => match key.code {
                    KeyCode::Char('q') => {
                        return Ok(());
                    }
                    KeyCode::Char(c) => task.input.push(c),
                    KeyCode::Enter => {
                        task.name.push(task.input.drain(..).collect());
                    }
                    KeyCode::Esc => task.mode = Mode::Normal,
                    KeyCode::Backspace => {
                        task.input.pop();
                    }
                    _ => {}
                },
            }
        }
    }
}

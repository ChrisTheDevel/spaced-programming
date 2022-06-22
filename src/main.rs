mod database;
mod error;
mod types;

use crossterm::{
    event::{self, DisableMouseCapture, EnableMouseCapture, Event},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use error::AppResult;
use std::io;
use tui::{
    backend::{Backend, CrosstermBackend},
    layout::{Alignment, Constraint, Direction, Layout},
    style::{Color, Modifier, Style},
    text::{Span, Spans},
    widgets::{Block, Borders, Paragraph, Wrap},
    Frame, Terminal,
};
use types::AppConfig;

fn main() -> AppResult<()> {
    // setup terminal
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    // setup configuration
    // - database directory and name
    let app_config = AppConfig {
        db_path: "./database/app_db.db".into(),
    };

    // then run app
    run_app(&mut terminal, app_config)?;

    // restore terminal
    disable_raw_mode()?;
    execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen,
        DisableMouseCapture
    )?;
    terminal.show_cursor()?;

    Ok(())
}

type Term = Terminal<CrosstermBackend<std::io::Stdout>>;

fn run_app(terminal: &mut Term, _app_conf: AppConfig) -> AppResult<()> {
    let mut state = AppState::default();
    let mut should_render = true;

    let start_banner_active = true;
    if start_banner_active {
        // welcome screen

        // banner with calvin S font
        let banner_str = "╔═╗╦═╗╔═╗╔═╗╦═╗╔═╗╔╦╗╔═╗╔╦╗╦╔═╗╔═╗\n\
                          ╠═╝╠╦╝║ ║║ ╦╠╦╝╠═╣║║║╠═╣ ║ ║║  ╠═╣\n\
                          ╩  ╩╚═╚═╝╚═╝╩╚═╩ ╩╩ ╩╩ ╩ ╩ ╩╚═╝╩ ╩";
        let message = "Welcome to programatica, press any button to continue";

        terminal.draw(|f| {
            let chunks = Layout::default()
                .constraints([
                    Constraint::Percentage(40),
                    Constraint::Percentage(20),
                    Constraint::Percentage(40),
                ])
                .direction(Direction::Vertical)
                .split(f.size());
            let banner = Paragraph::new(banner_str).alignment(Alignment::Center);
            let message = Paragraph::new(message).alignment(Alignment::Center);
            f.render_widget(banner, chunks[1]);
            f.render_widget(message, chunks[2]);
        })?;

        // wait until the user presses a button.
        loop {
            if let Event::Key(_key) = event::read()? {
                break;
            }
        }
    }

    terminal.draw(|f| ui(f, &state))?;

    // main app loop
    loop {
        // we only perform the heavy operation of redrawing the screen if we need to

        match event::read()? {
            Event::Key(key) => match key.code {
                event::KeyCode::Enter => {
                    should_render = true;
                    let AppState { counter, .. } = &mut state;
                    *counter += 10;
                }
                event::KeyCode::Char(char) => match char {
                    'q' => break,
                    _ => {}
                },
                _ => {}
            },
            Event::Resize(_, _) => should_render = true,
            _ => {}
        }

        if should_render {
            terminal.draw(|f| ui(f, &state))?;
            should_render = false;
        }
    }
    Ok(())
}

fn ui<B: Backend>(f: &mut Frame<B>, app_state: &AppState) {
    let counter = app_state.counter;

    let size = f.size();
    // we split the screen into two chunks
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Percentage(95), Constraint::Percentage(5)])
        .split(size);

    let text = vec![
        Spans::from(vec![
            Span::raw(format!("Counter: {counter}")),
            Span::styled("line", Style::default().add_modifier(Modifier::ITALIC)),
            Span::raw("."),
        ]),
        Spans::from(Span::styled("Second line", Style::default().fg(Color::Red))),
    ];

    let main_screen = Paragraph::new(text)
        .block(Block::default().title("Paragraph").borders(Borders::ALL))
        .style(Style::default().fg(Color::White).bg(Color::Black))
        .alignment(Alignment::Center)
        .wrap(Wrap { trim: true });
    f.render_widget(main_screen, chunks[0]);

    let info_screen = Block::default().title("Info").borders(Borders::ALL);
    f.render_widget(info_screen, chunks[1]);
}

struct AppState {
    screen: ScreenState,
    counter: u32,
}

enum ScreenState {
    MainMenu,
    Review,
}

impl Default for AppState {
    fn default() -> Self {
        AppState {
            screen: ScreenState::MainMenu,
            counter: 0,
        }
    }
}

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
    backend::CrosstermBackend,
    layout::{Alignment, Constraint, Direction, Layout},
    widgets::Paragraph,
    Frame, Terminal,
};
use types::AppConfig;

const BANNER_STR: &str = "╔═╗╦═╗╔═╗╔═╗╦═╗╔═╗╔╦╗╔═╗╔╦╗╦╔═╗╔═╗\n\
                          ╠═╝╠╦╝║ ║║ ╦╠╦╝╠═╣║║║╠═╣ ║ ║║  ╠═╣\n\
                          ╩  ╩╚═╚═╝╚═╝╩╚═╩ ╩╩ ╩╩ ╩ ╩ ╩╚═╝╩ ╩";

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

type Back = CrosstermBackend<std::io::Stdout>;
type Term = Terminal<Back>;

// the different screenstates
enum ScreenState {
    WelcomeScreen,
    MainScreen,
}

// the different transitions (or lack of them) between screens
enum Action {
    GotoWS,
    GotoMS,
    Quit,
    DoNothing,
    Resize,
    IncrementCounter,
}

fn run_app(terminal: &mut Term, _app_conf: AppConfig) -> AppResult<()> {
    let mut screen_state = ScreenState::WelcomeScreen;
    let mut should_render = false;

    let mut app_state = AppState::default();

    // initial rendering to screen
    ui(terminal, &screen_state, &app_state)?;

    loop {
        use Action::*;
        // first we see what action we should take given the current state, event combo
        let action = match (&screen_state, event::read()?) {
            (ScreenState::WelcomeScreen, Event::Key(key)) => match key.code {
                event::KeyCode::Char(char) => match char {
                    'q' => Quit,
                    _ => GotoMS,
                },
                event::KeyCode::Esc => Quit,
                _ => DoNothing,
            },
            (ScreenState::MainScreen, Event::Key(key)) => match key.code {
                event::KeyCode::Char(char) => match char {
                    'q' => GotoWS,
                    _ => DoNothing,
                },
                event::KeyCode::Esc => GotoWS,
                event::KeyCode::Enter => IncrementCounter,
                _ => DoNothing,
            },
            (_, Event::Resize(_, _)) => Resize,
            (_, Event::Mouse(_)) => DoNothing,
        };

        // the we act on that action
        match action {
            GotoWS => {
                screen_state = ScreenState::WelcomeScreen;
                should_render = true;
            }
            GotoMS => {
                screen_state = ScreenState::MainScreen;
                should_render = true;
            }
            Quit => break,
            DoNothing => {}
            Resize => should_render = true,
            IncrementCounter => {
                app_state.counter += 1;
                should_render = true
            }
        }

        if should_render {
            ui(terminal, &screen_state, &app_state)?;
            should_render = false;
        }
    }

    Ok(())
}

struct AppState {
    pub counter: u32,
}

impl Default for AppState {
    fn default() -> Self {
        AppState { counter: 0 }
    }
}

fn ui(term: &mut Term, state: &ScreenState, app_state: &AppState) -> Result<(), std::io::Error> {
    term.draw(|f| match state {
        ScreenState::WelcomeScreen => welcome_screen(f),
        ScreenState::MainScreen => main_screen(f, app_state),
    })?;
    Ok(())
}

fn welcome_screen(f: &mut Frame<Back>) {
    let message = "Welcome to programatica, press any button to continue";

    let chunks = Layout::default()
        .constraints([
            Constraint::Percentage(40),
            Constraint::Percentage(20),
            Constraint::Percentage(40),
        ])
        .direction(Direction::Vertical)
        .split(f.size());
    let banner = Paragraph::new(BANNER_STR).alignment(Alignment::Center);
    let message = Paragraph::new(message).alignment(Alignment::Center);
    f.render_widget(banner, chunks[1]);
    f.render_widget(message, chunks[2]);
}

fn main_screen(f: &mut Frame<Back>, app_state: &AppState) {
    let chunks = Layout::default()
        .constraints([
            Constraint::Percentage(40),
            Constraint::Percentage(20),
            Constraint::Percentage(40),
        ])
        .direction(Direction::Vertical)
        .split(f.size());
    let counter_str = format!("Counter: {}", app_state.counter);
    let counter = Paragraph::new(counter_str).alignment(Alignment::Center);
    let message = "This is the main screen, press enter to increment the conter";
    let message = Paragraph::new(message).alignment(Alignment::Center);
    f.render_widget(message, chunks[1]);
    f.render_widget(counter, chunks[2]);
}

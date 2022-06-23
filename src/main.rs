mod app_state;
mod constants;
mod database;
mod error;
mod screens;
mod types;

use app_state::AppState;
use crossterm::{
    event::{self, DisableMouseCapture, EnableMouseCapture, Event},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use error::AppResult;
use screens::{ui, Screen};
use std::io;
use tui::{
    backend::CrosstermBackend,
    layout::{Alignment, Constraint, Direction, Layout},
    widgets::Paragraph,
    Frame, Terminal,
};

use types::{AppConfig, Term};

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

// the different transitions (or lack of them) between screens
pub enum Action {
    Goto(Screen),
    Quit,
    DoNothing,
    Resize,
}

fn run_app(terminal: &mut Term, _app_conf: AppConfig) -> AppResult<()> {
    let mut app_state = AppState::default();

    // initial rendering to screen
    ui(terminal, &app_state)?;

    loop {
        // we only render if we have something new to show (this saves many cycles)
        if app_state.should_render {
            ui(terminal, &app_state)?;
            app_state = AppState::set_should_render(app_state, false);
        }

        let AppState { screen_state, .. } = &app_state;

        use Action::*;
        // first we see what action we should take given the current state, event combo
        let action = match (&screen_state, event::read()?) {
            (screens::Screen::WelcomeScreen, Event::Key(key)) => match key.code {
                event::KeyCode::Char(char) => match char {
                    'q' => Quit,
                    _ => Goto(Screen::MainScreen {
                        n_due: 5,
                        n_new: 6,
                        total: 11,
                    }),
                },
                event::KeyCode::Esc => Quit,
                _ => DoNothing,
            },
            (screens::Screen::MainScreen { .. }, Event::Key(key)) => match key.code {
                event::KeyCode::Char(char) => match char {
                    'q' => Goto(Screen::WelcomeScreen),
                    _ => DoNothing,
                },
                event::KeyCode::Esc => Goto(Screen::WelcomeScreen),
                _ => DoNothing,
            },
            (_, Event::Mouse(_)) => DoNothing,
            (_, Event::Resize(_, _)) => Resize,
        };

        // the we act on that action
        app_state = match action {
            Goto(screen) => app_state.goto_screen(screen),
            Quit => break,
            DoNothing => app_state,
            Resize => app_state.set_should_render(true),
        };
    }

    Ok(())
}

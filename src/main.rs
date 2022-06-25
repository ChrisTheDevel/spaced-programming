extern crate sm;
use screens::ui;
use sm::{sm, State};

mod constants;
mod database;
mod error;
mod screens;
mod types;

use crossterm::{
    event::{self, DisableMouseCapture, EnableMouseCapture},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use error::AppResult;
use std::{io, time::Duration};
use tui::{backend::CrosstermBackend, Terminal};
use types::{AppConfig, Term};

// These are our screen states as well as our transitions between them
sm! {
    Screen {
        InitialStates {
            WelcomeScreen
        }

        StartApp {
            WelcomeScreen => MainScreen
        }

        PromoteNewToDue {
            MainScreen => MainScreen
        }

        StartReview {
            MainScreen => ReviewInfoScreen
        }

        Quit {
            MainScreen, WelcomeScreen => GoodbyeScreen
        }

        CancelReview {
            ReviewInfoScreen => MainScreen
        }

        StartTimer {
            ReviewInfoScreen => ReviewTimerScreen
        }

        RestartTimer {
            ReviewEvalScreen => ReviewTimerScreen
        }

        CancelTimer {
            ReviewTimerScreen => ReviewInfoScreen
        }

        StopTimer {
            ReviewTimerScreen => ReviewEvalScreen
        }

        ReviewEasy {
            ReviewEvalScreen => MainScreen
        }
        ReviewNormal {
            ReviewEvalScreen => MainScreen
        }
        ReviewHard {
            ReviewEvalScreen => MainScreen
        }
     }
}

use crate::Screen::{
    Machine,
    Variant::{self as ScreenState, *},
    *,
};

pub struct AppState {
    pub screen_state: ScreenState,
}

impl AppState {
    fn init(conf: AppConfig) -> Self {
        Self {
            screen_state: Machine::new(WelcomeScreen).as_enum(),
        }
    }
}

fn main() -> AppResult<()> {
    // setup terminal
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    let config = AppConfig::default();

    run_app(&mut terminal, config)?;

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

#[allow(unreachable_code)]
fn run_app(term: &mut Term, conf: AppConfig) -> AppResult<()> {
    let mut app_state = AppState::init(conf);

    loop {
        // first perform any action associated with our current state+'how we got here' combo
        match &app_state.screen_state {
            GoodbyeScreenByQuit(_) => break,
            _ => {}
        }

        // then we first render to the screen
        ui(term, &app_state)?;

        // then we act on the state+event combo. This will block until we've created an event
        use event::{Event::*, KeyCode::*};

        // we consume the state
        let AppState { screen_state, .. } = app_state;
        let screen_state_clone = screen_state.clone();
        // TODO I do not know I I can make this any less boilerplaty?
        let screen_state = match (screen_state, event::read()?) {
            // for every state we define what keys take us to other events
            (InitialWelcomeScreen(sm), Key(key)) => match key.code {
                Esc => sm.transition(Quit).as_enum(),
                _ => sm.transition(StartApp).as_enum(),
            },
            // main screen bindings
            (MainScreenByStartApp(sm), Key(key)) => match key.code {
                Char('r') | Char(' ') => sm.transition(StartReview).as_enum(),
                Char('n') => sm.transition(PromoteNewToDue).as_enum(),
                Esc => sm.transition(Quit).as_enum(),
                _ => screen_state_clone,
            },
            (MainScreenByPromoteNewToDue(sm), Key(key)) => match key.code {
                Char('r') | Char(' ') => sm.transition(StartReview).as_enum(),
                Char('n') => sm.transition(PromoteNewToDue).as_enum(),
                Esc => sm.transition(Quit).as_enum(),
                _ => screen_state_clone,
            },

            (MainScreenByReviewEasy(sm), Key(key)) => match key.code {
                Char('r') | Char(' ') => sm.transition(StartReview).as_enum(),
                Char('n') => sm.transition(PromoteNewToDue).as_enum(),
                Esc => sm.transition(Quit).as_enum(),
                _ => screen_state_clone,
            },

            (MainScreenByReviewNormal(sm), Key(key)) => match key.code {
                Char('r') | Char(' ') => sm.transition(StartReview).as_enum(),
                Char('n') => sm.transition(PromoteNewToDue).as_enum(),
                Esc => sm.transition(Quit).as_enum(),
                _ => screen_state_clone,
            },
            (MainScreenByReviewHard(sm), Key(key)) => match key.code {
                Char('r') | Char(' ') => sm.transition(StartReview).as_enum(),
                Char('n') => sm.transition(PromoteNewToDue).as_enum(),
                Esc => sm.transition(Quit).as_enum(),
                _ => screen_state_clone,
            },
            (MainScreenByCancelReview(sm), Key(key)) => match key.code {
                Char('r') | Char(' ') => sm.transition(StartReview).as_enum(),
                Char('n') => sm.transition(PromoteNewToDue).as_enum(),
                Esc => sm.transition(Quit).as_enum(),
                _ => screen_state_clone,
            },
            // Review screen bindings
            (ReviewInfoScreenByCancelTimer(sm), Key(key)) => match key.code {
                Char(' ') => sm.transition(StartTimer).as_enum(),
                Esc => sm.transition(CancelReview).as_enum(),
                _ => screen_state_clone,
            },
            (ReviewInfoScreenByStartReview(sm), Key(key)) => match key.code {
                Char(' ') => sm.transition(StartTimer).as_enum(),
                Esc => sm.transition(CancelReview).as_enum(),
                _ => screen_state_clone,
            },
            (ReviewTimerScreenByStartTimer(sm), Key(key)) => match key.code {
                Char(' ') => sm.transition(StopTimer).as_enum(),
                Esc => sm.transition(CancelTimer).as_enum(),
                _ => screen_state_clone,
            },
            (ReviewTimerScreenByRestartTimer(sm), Key(key)) => match key.code {
                Char(' ') => sm.transition(StopTimer).as_enum(),
                Esc => sm.transition(CancelTimer).as_enum(),
                _ => screen_state_clone,
            },
            (ReviewEvalScreenByStopTimer(sm), Key(key)) => match key.code {
                Char(' ') | Char('2') => sm.transition(ReviewNormal).as_enum(),
                Char('1') => sm.transition(ReviewEasy).as_enum(),
                Char('3') => sm.transition(ReviewHard).as_enum(),
                Esc => sm.transition(RestartTimer).as_enum(),
                _ => screen_state_clone,
            },
            _ => screen_state_clone,
        };
        app_state = AppState { screen_state };
    }

    Ok(())
}

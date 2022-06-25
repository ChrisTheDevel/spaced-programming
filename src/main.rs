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
use spaced_rs::UserReview;
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

use crate::types::Item;
use std::time::Instant;

pub struct AppState {
    pub screen_state: ScreenState,
    pub should_quit: bool,
    pub due_items: Option<Vec<String>>,
    pub due_item: Option<String>,
    pub n_due: Option<u32>,
    pub n_new: Option<u32>,
    pub total: Option<u32>,
    pub time_stamp: Option<Instant>,
    pub duration: Option<Duration>,
    pub review_result: Option<UserReview>,
}

impl AppState {
    fn init(conf: AppConfig) -> Self {
        Self {
            screen_state: Machine::new(WelcomeScreen).as_enum(),
            should_quit: false,
            due_items: None,
            n_due: None,
            n_new: None,
            total: None,
            time_stamp: None,
            duration: None,
            review_result: None,
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

    // initial render
    ui(term, &app_state)?;

    loop {
        // first we see wait on the next event and see if we should change screen
        app_state = update_screen_state(app_state)?;

        // then we perform any action associated with our current state+'how we got here' combo
        app_state = perform_action(app_state);
        if app_state.should_quit {
            break;
        }

        // then we  render to the screen
        ui(term, &app_state)?;
    }

    Ok(())
}

fn perform_action(mut s: AppState) -> AppState {
    match &s.screen_state {
        GoodbyeScreenByQuit(_) => s.should_quit = true,
        // we do nothing
        InitialWelcomeScreen(_) => {}
        // we need to load in the due items from the database
        MainScreenByStartApp(_) => {
            // we need to load in the items
            // Mockitems
            let mut items: Vec<String> = vec![
                "https://open.kattis.com/problems/sequences".into(),
                "https://open.kattis.com/problems/3dprinter".into(),
                "https://open.kattis.com/problems/aa".into(),
            ];
            s.due_items = Some(items);
        }
        ReviewInfoScreenByStartReview(_) => {
            // TODO, how should I handle the case were there are no more due items?
            if s.due_item.is_none() {
                // try pop from due_items
                s.due_items
                    .and_then(|list| list.pop())
                    .and_then(|item| Some(item));
            }
        }
        MainScreenByCancelReview(_) => todo!(),
        MainScreenByPromoteNewToDue(_) => todo!(),
        ReviewTimerScreenByStartTimer(_) => todo!(),
        ReviewTimerScreenByRestartTimer(_) => todo!(),
        ReviewInfoScreenByCancelTimer(_) => todo!(),
        ReviewEvalScreenByStopTimer(_) => todo!(),
        MainScreenByReviewEasy(_) => todo!(),
        MainScreenByReviewNormal(_) => todo!(),
        MainScreenByReviewHard(_) => todo!(),
    }
    s
}

// TODO, Here I have to
fn update_screen_state(mut app_state: AppState) -> AppResult<AppState> {
    use event::Event::*;
    use event::KeyCode::*;
    let state_clone = app_state.screen_state.clone();
    app_state.screen_state = match (app_state.screen_state, event::read()?) {
        (InitialWelcomeScreen(sm), Key(key)) => match key.code {
            Esc => sm.transition(Quit).as_enum(),
            _ => sm.transition(StartApp).as_enum(),
        },
        // main screen bindings
        (MainScreenByStartApp(sm), Key(key)) => match key.code {
            Char('r') | Char(' ') => sm.transition(StartReview).as_enum(),
            Char('n') => sm.transition(PromoteNewToDue).as_enum(),
            Esc => sm.transition(Quit).as_enum(),
            _ => state_clone,
        },
        (MainScreenByPromoteNewToDue(sm), Key(key)) => match key.code {
            Char('r') | Char(' ') => sm.transition(StartReview).as_enum(),
            Char('n') => sm.transition(PromoteNewToDue).as_enum(),
            Esc => sm.transition(Quit).as_enum(),
            _ => state_clone,
        },

        (MainScreenByReviewEasy(sm), Key(key)) => match key.code {
            Char('r') | Char(' ') => sm.transition(StartReview).as_enum(),
            Char('n') => sm.transition(PromoteNewToDue).as_enum(),
            Esc => sm.transition(Quit).as_enum(),
            _ => state_clone,
        },

        (MainScreenByReviewNormal(sm), Key(key)) => match key.code {
            Char('r') | Char(' ') => sm.transition(StartReview).as_enum(),
            Char('n') => sm.transition(PromoteNewToDue).as_enum(),
            Esc => sm.transition(Quit).as_enum(),
            _ => state_clone,
        },
        (MainScreenByReviewHard(sm), Key(key)) => match key.code {
            Char('r') | Char(' ') => sm.transition(StartReview).as_enum(),
            Char('n') => sm.transition(PromoteNewToDue).as_enum(),
            Esc => sm.transition(Quit).as_enum(),
            _ => state_clone,
        },
        (MainScreenByCancelReview(sm), Key(key)) => match key.code {
            Char('r') | Char(' ') => sm.transition(StartReview).as_enum(),
            Char('n') => sm.transition(PromoteNewToDue).as_enum(),
            Esc => sm.transition(Quit).as_enum(),
            _ => state_clone,
        },
        // Review screen bindings
        (ReviewInfoScreenByCancelTimer(sm), Key(key)) => match key.code {
            Char(' ') => sm.transition(StartTimer).as_enum(),
            Esc => sm.transition(CancelReview).as_enum(),
            _ => state_clone,
        },
        (ReviewInfoScreenByStartReview(sm), Key(key)) => match key.code {
            Char(' ') => sm.transition(StartTimer).as_enum(),
            Esc => sm.transition(CancelReview).as_enum(),
            _ => state_clone,
        },
        (ReviewTimerScreenByStartTimer(sm), Key(key)) => match key.code {
            Char(' ') => sm.transition(StopTimer).as_enum(),
            Esc => sm.transition(CancelTimer).as_enum(),
            _ => state_clone,
        },
        (ReviewTimerScreenByRestartTimer(sm), Key(key)) => match key.code {
            Char(' ') => sm.transition(StopTimer).as_enum(),
            Esc => sm.transition(CancelTimer).as_enum(),
            _ => state_clone,
        },
        (ReviewEvalScreenByStopTimer(sm), Key(key)) => match key.code {
            Char(' ') | Char('2') => sm.transition(ReviewNormal).as_enum(),
            Char('1') => sm.transition(ReviewEasy).as_enum(),
            Char('3') => sm.transition(ReviewHard).as_enum(),
            Esc => sm.transition(RestartTimer).as_enum(),
            _ => state_clone,
        },
        _ => state_clone,
    };
    Ok(app_state)
}

use tui::{
    layout::{Alignment, Constraint, Direction, Layout, Rect},
    widgets::{Block, Borders, List, ListItem, Paragraph},
    Frame,
};

use crate::{
    app_state::AppState,
    constants::BANNER_STR,
    types::{Back, Term},
};

#[derive(Clone)]
pub enum Screen {
    WelcomeScreen,
    MainScreen { n_due: u32, n_new: u32, total: u32 },
}

pub fn welcome_screen(f: &mut Frame<Back>) {
    let message = "Welcome to programatica, press any button to continue";

    let chunks = Layout::default()
        .constraints([
            Constraint::Percentage(40),
            Constraint::Percentage(50),
            Constraint::Percentage(10),
        ])
        .direction(Direction::Vertical)
        .split(f.size());
    let banner = Paragraph::new(BANNER_STR).alignment(Alignment::Center);
    let message = Paragraph::new(message).alignment(Alignment::Center);
    f.render_widget(banner, chunks[1]);
    f.render_widget(message, chunks[2]);
}

pub fn main_screen(f: &mut Frame<Back>, n_due: u32, n_new: u32, total: u32) {
    let [main_area, keymap_hint_area] = split_screen_hint(f.size());

    let n_due = format!("n_due: {n_due}");
    let n_new = format!("n_new: {n_new}");
    let total = format!("total: {total}");

    let items = [
        ListItem::new(n_due),
        ListItem::new(n_new),
        ListItem::new(total),
    ];

    let list = List::new(items).block(Block::default().title("List").borders(Borders::ALL));

    f.render_widget(list, main_area);

    let keymap_hint_str = "r: start review session   n: make new item due";
    let keymap_hint_widget = str_to_paragraph(keymap_hint_str);
    f.render_widget(keymap_hint_widget, keymap_hint_area);
}

fn str_to_paragraph(str: &str) -> Paragraph {
    Paragraph::new(str)
        .block(Block::default().borders(Borders::ALL))
        .alignment(Alignment::Center)
}

// splits the screen area into the keymap hint area and the main screen area
fn split_screen_hint(r: Rect) -> [Rect; 2] {
    let split = Layout::default()
        .constraints([Constraint::Percentage(95), Constraint::Percentage(5)])
        .direction(Direction::Vertical)
        .split(r);
    [split[0], split[1]]
}

// divide the screen into n equal slices along the vertical axis
fn split_screen_n(r: Rect, n: u16) -> Vec<Rect> {
    let percentage = 100 / n;
    let constraints: Vec<Constraint> = (0..n)
        .into_iter()
        .map(|_| Constraint::Percentage(percentage))
        .collect();
    let chunks = Layout::default()
        .constraints(constraints)
        .direction(Direction::Vertical)
        .split(r);
    chunks
}

pub fn ui(term: &mut Term, app_state: &AppState) -> Result<(), std::io::Error> {
    term.draw(|f| {
        let [screen_area, keybind_hint_area] = split_screen_hint(f.size());

        match app_state.screen_state {
            Screen::WelcomeScreen => welcome_screen(f),
            Screen::MainScreen {
                n_due,
                n_new,
                total,
            } => main_screen(f, n_due, n_new, total),
        };
    })?;
    Ok(())
}

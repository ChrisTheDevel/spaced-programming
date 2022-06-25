use sm::sm;
use tui::layout::Alignment;
use tui::widgets::{Block, Borders, Paragraph};
use tui::Frame;

use crate::types::{Back, Term};
use crate::AppState;

use crate::Screen::{
    Variant::{self as ScreenState, *},
    *,
};

fn str_to_paragraph(str: &str) -> Paragraph {
    Paragraph::new(str)
        .block(Block::default().borders(Borders::ALL))
        .alignment(Alignment::Center)
}

fn quick_dirty_test_screen(f: &mut Frame<Back>, state: &ScreenState) {
    let paragraph_str = &format!("{:?}", state);
    let paragraph = str_to_paragraph(paragraph_str);
    f.render_widget(paragraph, f.size());
}

pub fn ui(term: &mut Term, state: &AppState) -> std::io::Result<()> {
    let screen_state: &ScreenState = &state.screen_state;
    term.draw(|f| {
        quick_dirty_test_screen(f, screen_state);
    })?;
    Ok(())
}

// modules
pub mod schema;
mod scheduler;
mod error;
mod database;
#[cfg(test)]
mod test_util;

#[macro_use]
extern crate diesel; // this gives us access to compile time validation of our schema.
#[macro_use]
extern crate diesel_migrations; // this gives us access to diesels migrations but built into the binary

// stdlib imports
use std::io;
use std::time::Duration;

const DURATION_SEC: u64 = 5;

// external crate imports
use crossterm::{
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use tui::widgets::{Block, Borders};
use tui::{backend::CrosstermBackend, Terminal};

fn main() -> Result<(), io::Error> {
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    terminal.draw(|f| {
        let size = f.size();
        let block = Block::default().title("Block").borders(Borders::ALL);
        f.render_widget(block, size);
    })?;

    std::thread::sleep(Duration::from_secs(DURATION_SEC));

    disable_raw_mode()?;
    execute!(terminal.backend_mut(), LeaveAlternateScreen)?;
    terminal.show_cursor()?;
    Ok(())
}

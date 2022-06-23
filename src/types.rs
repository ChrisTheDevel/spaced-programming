use std::path::PathBuf;

pub use spaced_rs::SchedulingData;
use tui::{backend::CrosstermBackend, Terminal};

pub type SchemaVersion = usize;
pub type ItemId = u64;

// Item specific types

/// The review item type.
pub struct Item {
    // the first time we insert the item into the item table we want it to provide us with a id
    pub id: ItemId,

    // scheduling data
    pub scheduling_data: SchedulingData,
    // timestamp
    pub due: u64,

    // url to problem
    pub url: String,
    pub tags: Vec<String>,
    pub item_notes: String,
}

/// Struct representing problem resource that has not been made into a review item.
/// Builds a bcklog of problems that we want to review.
pub struct URLItem {
    pub id: u64,
    pub url: String,
}

pub struct AppConfig {
    pub db_path: PathBuf,
}

// Item specific type end here

// typealiases for tui and crossterm specific types
pub type Back = CrosstermBackend<std::io::Stdout>;
pub type Term = Terminal<Back>;

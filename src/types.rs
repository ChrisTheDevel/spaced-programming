// stdlib imports
use std::path::Path;

// external crate imports
pub use spaced_rs::{SchedulingData, UpdateParameters}; // this is my own library! We reexport these types here so i can use them in the rest of the crate

/// Struct representing an item that is in review.
pub struct Item {
    id: u64,
    scheduling_data: SchedulingData,
    url: String,
    tags: Vec<String>,
    notes: String,
}

/// Struct representing url tag pair that has yet to be made into item
pub struct NewItem {
    url: String,
    tags: Vec<String>,
}

/// Struct representing configuration object. Includes default parameters for scheduling algorithm,
/// db-location etc. Might be generated from a config file in the future
pub struct Configuration {
    /// default parameters to determine when the first review event is for a given item
    default_scheduling_data: SchedulingData,
    /// default parameters for how the review interval of an item should be updated.
    default_update_parameters: UpdateParameters,
    db_location: Path,
}

pub type SchemaVersion = usize;

/// Item type
pub struct Item {
    // Unix time in milliseconds
    id: u128,

    // scheduling data
    intervall: i32,
    difficulty: f32,
    memory_strength: f32,
    adjusting_factor: f32,
    times_reviewed: i32,
    times_recalled: i32,

    // url to problem
    url: String,
    tags: Vec<String>,
    item_notes: String,
}

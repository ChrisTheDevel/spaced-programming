pub use spaced_rs::SchedulingData;
pub type SchemaVersion = usize;
pub type ItemId = u64;

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

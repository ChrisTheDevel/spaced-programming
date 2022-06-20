pub use spaced_rs::SchedulingData;
pub type SchemaVersion = usize;
pub type ItemId = u64;

// id field omittet
pub struct Item {
    // the first time we insert the item into the item table we want it to provide us with a id
    id: Option<ItemId>,

    // scheduling data
    scheduling_data: SchedulingData,

    // url to problem
    url: String,
    tags: Vec<String>,
    item_notes: String,
}

/// Metadata struct related to the scheduling of an item.
pub struct ScheduleItem {
    pub id: u64,
    // unix timestamp
    pub due: u64,
    // references the id of the Item
    pub item_id: ItemId,
}

/// Struct representing problem resource that has not been made into a review item.
/// Builds a backlog of problems that we want to review.
pub struct URLItem {
    pub id: u64,
    pub url: String,
}

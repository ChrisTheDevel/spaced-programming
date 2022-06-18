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

pub struct ScheduleItem {
    pub id: u64,
    // unix timestamp
    pub due: u64,
    // references the id of the Item
    pub item_id: ItemId,
}

// new item. When representing an item that we're going to insert into inbox,
// the id field is none (we want sqlite to give us a id)
pub struct NewItem {
    pub id: Option<u64>,
    pub url: String,
}

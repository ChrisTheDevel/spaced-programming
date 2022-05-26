//! types representing items
use crate::schema::*;

#[derive(Queryable, Insertable)]
#[table_name = "items"]
/// Items from the items table
pub struct Item {
    pub id: i32,
    pub intervall: i32,
    pub difficulty: f32,
    pub memory_strength: f32,
    pub adjusting_factor: f32,
    pub times_reviewed: i32,
    pub times_recalled: i32,
    pub url: String,
    pub item_data: String,
}

/// Items from the schedule table
#[derive(Queryable, Insertable)]
#[table_name = "schedule"]
pub struct ReviewItem {
    pub id: i32,
    pub due: i32,
    pub item_id: i32,
}

/// New items from the inbox table
#[derive(Queryable, Insertable)]
#[table_name = "inbox"]
pub struct NewItem {
    pub id: i32,
    pub url: String,
}

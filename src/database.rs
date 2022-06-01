// external crate imports
use diesel::prelude::*;
use diesel::result::QueryResult;
embed_migrations!(); //Inject a module embeded migrations which has all the migrations we want.
use diesel::expression::dsl::now;
// internal crate imports
use crate::types::*;
// stdlib imports
use std::time::{SystemTime, UNIX_EPOCH};

/// database wrapper
pub struct Database {
    connection: SqliteConnection,
}

impl Database {
    pub fn new(connection: SqliteConnection) -> Self {
        Self { connection }
    }

    pub fn get_item(&self, item_id: i32) -> QueryResult<Item> {
        use crate::schema::items::dsl::*;
        items.find(id).first(&self.connection)
    }

    pub fn get_due(&mut self) -> QueryResult<Vec<ReviewItem>> {
        use crate::schema::schedule::dsl::*;
        let unix_time_now = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs();
        schedule.filter(due.le(unix_time_now)).load(&mut self.connection)
    }
}

// external crate imports
use diesel::prelude::*;
use diesel::result::QueryResult;
// internal crate imports
use crate::types::*;

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

    pub fn add_item(&self, item: Item) -> QueryResult<()> {
        use crate::schema::items;
        let _n_rows_affected = diesel::insert_into(items::table)
            .values(&item)
            .execute(&self.connection)?;
        Ok(())
    }

    pub fn get_due(&self) -> Vec<ReviewItem> {
        todo!()
    }
}

//! This module contains all interactions with the database. I here expose sql calls through functions and nothing else.

// stdlib imports
use std::path::Path;
// external imports
use rusqlite::{Connection, Row};
// internal imports
use crate::{
    error::{DatabaseErrorSource, DatabaseResult},
    types::{Item, ItemId, NewItem, ScheduleItem},
};

const SCHEMA_VERSION: usize = 3;

/// creates a connection to the database at location {path} and creates the paths leading up to it if id didn't exist
pub fn open_connection(path: &Path) -> DatabaseResult<Connection> {
    // check if the db already exists
    let conn;
    if path.exists() {
        // if it exists that means we have initialized it before, therefore the schema should be valid.
        // we now need to validate it's schema... before we can return the connnection
        conn = Connection::open(path)?;
        let sv = schema_version(&conn)?;
        if sv != SCHEMA_VERSION {
            // if the schema was invalid we simply err, might have some migration logic in the future
            return Err(DatabaseErrorSource::InvalidSchemaError(sv));
        }
    } else {
        let parent = path
            .parent()
            .expect("could not retrieve parent! The provided path cannot be a valid location");
        std::fs::create_dir_all(parent)?;
        conn = Connection::open(path)?;
        init_schema(&conn)?;
    }
    Ok(conn)
}

pub fn schema_version(conn: &Connection) -> DatabaseResult<usize> {
    let query = "SELECT schema_version FROM pragma_schema_version";

    let mut stmt = conn.prepare(query)?;
    Ok(stmt.query_row([], |row: &Row<'_>| {
        let schema_version: usize = row.get(0)?;
        Ok(schema_version)
    })?)
}

pub fn init_schema(conn: &Connection) -> DatabaseResult<()> {
    // create items table (containing item specific data)
    // create schedule table (used to assign due dates and query items that are due)
    // create inbox table (used to store urls+tags for future items)
    create_items(&conn)?;
    create_schedule(&conn)?;
    create_inbox(&conn)?;
    Ok(())
}

fn create_items(conn: &Connection) -> DatabaseResult<()> {
    // the item table
    // the id column is an alias for the rowid
    let sql_string = "CREATE TABLE items (\
                id INTEGER PRIMARY KEY AUTOINCREMENT,\
                intervall INTEGER NOT NULL,\
                difficulty REAL NOT NULL,\
                memory_strength REAL NOT NULL,\
                adjusting_factor REAL NOT NULL,\
                times_reviewed INTEGER NOT NULL,\
                times_recalled INTEGER NOT NULL,\
                url TEXT NOT NULL UNIQUE,\
                tags TEXT NOT NULL,\
                item_notes TEXT NOT NULL\
            )";
    conn.execute(sql_string, [])?;
    Ok(())
}

fn create_schedule(conn: &Connection) -> DatabaseResult<()> {
    // TODO, do I need the id column?
    let sql_string = "CREATE TABLE schedule (\
                id INTEGER PRIMARY KEY AUTOINCREMENT,\
                due INTEGER NOT NULL,\
                item_id INTEGER NOT NULL UNIQUE,\
                FOREIGN KEY (item_id) REFERENCES items(id) ON DELETE CASCADE\
            )";
    conn.execute(sql_string, [])?;
    Ok(())
}

fn create_inbox(conn: &Connection) -> DatabaseResult<()> {
    let sql_string = "CREATE TABLE inbox (\
                id INTEGER PRIMARY KEY AUTOINCREMENT,\
                url TEXT NOT NULL\
            )";
    conn.execute(sql_string, [])?;
    Ok(())
}

/// adds a new urls to the bottom inbox table. (enqueue)
pub fn add_urls_to_inbox(conn: &Connection, new_items: Vec<String>) -> DatabaseResult<()> {
    let stmt = "INSERT INTO inbox (url) VALUES (?)";
    for url in new_items {
        conn.execute(stmt, [url])?;
    }
    Ok(())
}

/// gets the top n items in the queue
pub fn get_n_urls_from_inbox(conn: &Connection, n_items: usize) -> DatabaseResult<Vec<NewItem>> {
    let query = "SELECT id, url FROM inbox LIMIT ?";
    let mut smts = conn.prepare(query)?;
    let rows = smts
        .query_map([n_items], |row| {
            Ok(NewItem {
                id: row.get(0)?,
                url: row.get(1)?,
            })
        })?
        .into_iter()
        .map(|row| row.expect("Could not create NewItem from query!"))
        .collect();
    Ok(rows)
}

// removes item from the inbox (probably to turn it into a review item)
pub fn remove_new_item(conn: &Connection, id: u64) -> DatabaseResult<()> {
    let stmt = "DELETE FROM inbox WHERE id=?";
    conn.execute(stmt, [id])?;
    Ok(())
}

// gets the item ids whose due date value is less than timestamp
// TODO the id field in the DueItem will be redundant
pub fn get_due_ids(conn: &Connection, timestamp: u64) -> DatabaseResult<Vec<ScheduleItem>> {
    let query = "SELECT id,due,item_id FROM schedule WHERE due <= ?";
    let mut stmt = conn.prepare(query)?;
    let rows = stmt
        .query_map([timestamp], |row| {
            Ok(ScheduleItem {
                id: row.get(0)?,
                due: row.get(1)?,
                item_id: row.get(2)?,
            })
        })?
        .into_iter()
        .map(|row| row.expect("Could not create ScheduleItem from query"))
        .collect();
    Ok(rows)
}

// sets the due value of the item with item_id' == item_id equal to due
pub fn schedule_item(due: u64, item_id: ItemId) -> DatabaseResult<()> {
    todo!()
}

// gets an item from the items table
pub fn get_review_item(item_id: ItemId) -> DatabaseResult<()> {
    todo!()
}

// gets several items from the items table
pub fn get_review_items(item_ids: Vec<ItemId>) -> DatabaseResult<Vec<Item>> {
    todo!()
}

// sets the columns of a given item row to the fields of our Item instance
// this should be used to update an existing item row.
pub fn update_item(item: Item) -> DatabaseResult<()> {
    todo!()
}

// inserts a new item into the items table (when turning a new_item into an item)
// Should return the id as provided by sqlite (we use this id when scheduling the item)
pub fn insert_item(item: Item) -> DatabaseResult<ItemId> {
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;
    use serial_test::serial;
    use std::path::PathBuf;

    /// util function to create temp path for db_instance. Returns path and cleanup function.
    fn create_temp_dir(db_name: &str) -> (PathBuf, Box<dyn FnOnce() -> std::io::Result<()>>) {
        let temp_dir = std::env::temp_dir().join(format!("{db_name}_dir"));
        let full_path = temp_dir.clone().join(format!("{db_name}.db"));
        // cleanup callback
        let clean_up = Box::new(move || std::fs::remove_dir_all(temp_dir));
        (full_path, clean_up)
    }

    #[test]
    #[serial] // annotation that will make sure that this is run sequentially
    fn create_db_once() {
        let (db_path, cleanup) = create_temp_dir("create_db_once");
        let conn = open_connection(&db_path);
        assert!(conn.is_ok());
        assert!(cleanup().is_ok());
    }

    #[test]
    #[serial]
    fn create_db_twice() {
        let (db_path, cleanup) = create_temp_dir("create_db_once");
        {
            let conn = open_connection(&db_path);
            assert!(conn.is_ok());
        }

        let conn = open_connection(&db_path);
        assert!(conn.is_ok());
        // since we successfully created the db the second time we can also be sure that we do infact do a SCHEMA_VERSION number of schema altering changes

        assert!(cleanup().is_ok());
    }

    #[test]
    #[serial]
    fn add_and_get_one_new_items() {
        let (db_path, cleanup) = create_temp_dir("add_and_get_one_new_items");
        let url: String = "https://open.kattis.com/problems/hello".into();
        let urls = vec![url.clone()];
        // this connection should be fine according to other tests
        let conn = open_connection(&db_path).unwrap();
        let res_add = add_urls_to_inbox(&conn, urls);
        // could we successfully add the item?
        assert!(res_add.is_ok());
        // we could!
        // now can we retrieve that same item?
        let res_get = get_n_urls_from_inbox(&conn, 1);
        assert!(res_get.is_ok());
        let res = res_get.unwrap();
        assert!(res.len() == 1);
        assert!(res[0].url == url);
        assert!(cleanup().is_ok());
    }

    #[test]
    #[serial]
    fn add_and_get_several_new_items() {
        let (db_path, cleanup) = create_temp_dir("add_and_get_several_new_items");
        let urls = vec![
            "https://open.kattis.com/problems/hello".into(),
            "https://open.kattis.com/problems/faktor".into(),
            "https://open.kattis.com/problems/autori".into(),
        ];
        // this connection should be fine according to other tests
        let conn = open_connection(&db_path).unwrap();
        let res_add = add_urls_to_inbox(&conn, urls.clone());
        // could we successfully add the items?
        assert!(res_add.is_ok());
        // we could!
        // now can we retrieve that same items in queue order??
        let res_get = get_n_urls_from_inbox(&conn, urls.len());
        assert!(res_get.is_ok());
        let res = res_get.unwrap();
        let res_inner_strings: Vec<String> = res.into_iter().map(|item| item.url).collect();
        assert!(res_inner_strings.len() == urls.len());
        assert!(urls == res_inner_strings);
        assert!(cleanup().is_ok());
    }

    #[test]
    #[serial]
    fn add_several_and_remove_some_new_items() {
        let (db_path, cleanup) = create_temp_dir("add_several_and_remove_some_new_items");
        // add some urls
        let urls = vec![
            "https://open.kattis.com/problems/hello".into(),
            "https://open.kattis.com/problems/faktor".into(),
            "https://open.kattis.com/problems/autori".into(),
        ];
        let conn = open_connection(&db_path).unwrap();
        let _res_add = add_urls_to_inbox(&conn, urls.clone()).unwrap();

        // now we remove some, readd them and see that the order has been changed
        let res_get = get_n_urls_from_inbox(&conn, urls.len()).unwrap();
        let res_inner_strings: Vec<String> = res_get.into_iter().map(|item| item.url).collect();
        assert!(urls == res_inner_strings);

        // can we remove items?
        assert!(remove_new_item(&conn, 1).is_ok());
        // yes we can!
        // can we insert them again and get the items in the expected order?
        assert!(
            add_urls_to_inbox(&conn, vec!["https://open.kattis.com/problems/hello".into()]).is_ok()
        );
        let second_res = get_n_urls_from_inbox(&conn, urls.len()).unwrap();
        let second_inner_strings: Vec<String> =
            second_res.into_iter().map(|item| item.url).collect();
        let expected: Vec<String> = vec![
            "https://open.kattis.com/problems/faktor".into(),
            "https://open.kattis.com/problems/autori".into(),
            "https://open.kattis.com/problems/hello".into(),
        ];
        assert!(second_inner_strings == expected);
        // yes we can
        // can we sing?
        // ooh yeees we caaaan.

        assert!(cleanup().is_ok());
    }
}

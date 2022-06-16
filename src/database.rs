//! This module contains all interactions with the database. I here expose sql calls through functions and nothing else.

// stdlib imports
use std::path::Path;
// external imports
use rusqlite::{Connection, Row};
// internal imports
use crate::error::{DatabaseErrorSource, DatabaseResult};

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
    let sql_string = "CREATE TABLE items (\
                id INTEGER PRIMARY KEY NOT NULL,\
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
    let sql_string = "CREATE TABLE schedule (\
                id INTEGER PRIMARY KEY NOT NULL,\
                due INTEGER NOT NULL,\
                item_id INTEGER NOT NULL UNIQUE,\
                FOREIGN KEY (item_id) REFERENCES items(id) ON DELETE CASCADE\
            )";
    conn.execute(sql_string, [])?;
    Ok(())
}

fn create_inbox(conn: &Connection) -> DatabaseResult<()> {
    let sql_string = "CREATE TABLE inbox (\
                id INTEGER PRIMARY KEY NOT NULL,\
                url TEXT NOT NULL\
            )";
    conn.execute(sql_string, [])?;
    Ok(())
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
}

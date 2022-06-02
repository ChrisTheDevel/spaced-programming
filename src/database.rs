use crate::error::*;
use rusqlite::Connection;
use std::path::Path;

use queries::init_schema;

/// The number of schema altering commands That has been run on the db.
const SCHEMA_VERSION: i32 = 3;

pub struct Database {
    connection: Connection,
}

impl Database {
    /// Returns an instance of a Database. Also creates it and sets up schema if it did not exist.
    /// Expects full path to db (including db file name)
    pub fn init(db_path: &Path) -> DatabaseResult<Database> {
        // check if the db exist, if it does not, create path for it
        if !db_path.exists() {
            std::fs::create_dir_all(db_path)?;
        }
        // open database connection
        let conn = Connection::open(db_path)?;

        // query for pragma version
        let version = queries::schema_version(&conn)?;

        if version == 0 {
            // if the schema version is zero then the database is new and we need to init its schema
            init_schema(&conn)?;
        } else if version != SCHEMA_VERSION {
            // if the schema version is non-zero but different from our SCHEMA_VERSION constant, throw
            // an error. We might handle migration later
            return Err(DatabaseErrorSource::InvalidSchemaError("The Schema was not valid! A migration might have failed or the db was not properly initalized!".into()));
        }
        // if we've gotten this far then it is ok to take the connectiona and return it.
        Ok(Self { connection: conn })
    }
}

mod queries {
    use super::*;
    use rusqlite::{Result as RusqliteResult, Row};

    pub fn init_schema(conn: &Connection) -> RusqliteResult<()> {
        // create items table (containing item specific data)
        // create schedule table (used to assign due dates and query items that are due)
        // create inbox table (used to store urls+tags for future items)
        create_items(&conn)?;
        create_schedule(&conn)?;
        create_inbox(&conn)?;
        Ok(())
    }

    fn create_items(conn: &Connection) -> RusqliteResult<()> {
        let sql_string = "CREATE TABLE items (\
                id INTEGER PRIMARY KEY NOT NULL,\
                intervall INTEGER NOT NULL,\
                difficulty REAL NOT NULL,\
                memory_strength REAL NOT NULL,\
                adjusting_factor REAL NOT NULL,\
                times_reviewed INTEGER NOT NULL,\
                times_recalled INTEGER NOT NULL,\
                url TEXT NOT NULL UNIQUE,\
                item_data TEXT NOT NULL\
            )";
        conn.execute(sql_string, [])?;
        Ok(())
    }

    fn create_schedule(conn: &Connection) -> RusqliteResult<()> {
        let sql_string = "CREATE TABLE schedule (\
                id INTEGER PRIMARY KEY NOT NULL,\
                due INTEGER NOT NULL, -- due date stored in unix time\
                item_id INTEGER NOT NULL UNIQUE,\
                FOREIGN KEY (item_id) REFERENCES items(id) ON DELETE CASCADE\
            )";
        conn.execute(sql_string, [])?;
        Ok(())
    }

    fn create_inbox(conn: &Connection) -> RusqliteResult<()> {
        let sql_string = "CREATE TABLE inbox (\
                id INTEGER PRIMARY KEY NOT NULL,\
                url TEXT NOT NULL\
            )";
        conn.execute(sql_string, [])?;
        Ok(())
    }

    pub fn schema_version(conn: &Connection) -> RusqliteResult<i32> {
        let query = "SELECT schema_version FROM pragma_schema_version";
        let mut stmt = conn.prepare(query)?;
        stmt.query_row([], |row: &Row<'_>| {
            let schema_version: i32 = row.get(0)?;
            Ok(schema_version)
        })
    }
}

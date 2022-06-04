use crate::error::*;
use rusqlite::Connection;
use std::path::Path;

/// The number of schema altering commands That has been run on the db.
const SCHEMA_VERSION: usize = 3;

pub struct Database {
    connection: Connection,
}

impl Database {
    /// Returns an instance of a Database. Also creates it and sets up schema if it did not exist.
    /// Expects full path to db (including db file name)
    pub fn init(db_path: &Path) -> DatabaseResult<Database> {
        // check if the db exist, if it does not, create path for it
        if !db_path.exists() {
            // we only want to create the path up until but not including the db name
            std::fs::create_dir_all(db_path.parent().unwrap())?;
        }
        // open database connection
        let conn = Connection::open(db_path)?;

        // query for pragma version
        let version = queries_and_stmts::schema_version(&conn)?;

        if version == 0 {
            // if the schema version is zero then the database is new and we need to init its schema
            queries_and_stmts::init_schema(&conn)?;
        } else if version != SCHEMA_VERSION {
            // if the schema version is non-zero but different from our SCHEMA_VERSION constant, throw
            // an error. We might handle migration later
            return Err(DatabaseErrorSource::InvalidSchemaError("The Schema was not valid! A migration might have failed or the db was not properly initalized!".into()));
        }
        // if we've gotten this far then it is ok to take the connectiona and return it.
        Ok(Self { connection: conn })
    }

    fn schema_version(&self) -> DatabaseResult<usize> {
        Ok(queries_and_stmts::schema_version(&self.connection)?)
    }
}

/// module containing all queries and statements abstracted as functions. All sql code should be
/// contained in this module
mod queries_and_stmts {
    use super::*;
    use rusqlite::{MappedRows, Result as RusqliteResult, Row, Statement};

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
                due INTEGER NOT NULL,\
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

    pub fn schema_version(conn: &Connection) -> RusqliteResult<usize> {
        let query = "SELECT schema_version FROM pragma_schema_version";
        let mut stmt = conn.prepare(query)?;
        stmt.query_row([], |row: &Row<'_>| {
            let schema_version: usize = row.get(0)?;
            Ok(schema_version)
        })
    }
}

#[cfg(test)]
mod tests {
    use std::path::PathBuf;

    use super::*;

    /// util function to create temp path for db_instance. Returns path and cleanup function.
    fn create_temp_dir(db_name: &str) -> (PathBuf, Box<dyn FnOnce() -> std::io::Result<()>>) {
        let temp_dir = std::env::temp_dir().join(format!("{db_name}_dir"));
        let full_path = temp_dir.clone().join(format!("{db_name}.db"));

        // since the init function in Database already takes care of creating dirs if nonexistent,
        // we do not create them here
        //std::fs::create_dir_all(&temp_dir).unwrap();

        // TODO why do I not need to use move in the closure below? Clearly it outlives temp_dir
        // Probably because the compiler simply infers it?
        let clean_up = Box::new(|| std::fs::remove_dir_all(temp_dir));
        (full_path, clean_up)
    }

    #[test]
    fn create_db_once() {
        let (db_path, cleanup) = create_temp_dir("create_db_once");
        let db_result = Database::init(&db_path);

        //we should always be able to create a new database and initialize its schema
        assert!(db_result.is_ok());
        assert!(cleanup().is_ok());
    }

    // TODO the rusqlite library apparently supports concurrent access! What is it then that it
    // cannot do concurrently?
    // #[test]
    // fn create_two_db_simul() {
    //     let (db_path, cleanup) = create_temp_dir("create_two_db_simul");
    //
    //     let first_db = Database::init(&db_path);
    //     assert!(first_db.is_ok());  // the first instantiation of the db should succeed
    //     let second_db = Database::init(&db_path);
    //     assert!(first_db.is_err()); // the second one should fail!
    // }

    #[test]
    fn create_two_db_seq() {
        let (db_path, cleanup) = create_temp_dir("create_two_db_seq");
        {
            let first = Database::init(&db_path);
            assert!(first.is_ok());
        }

        let second = Database::init(&db_path);
        assert!(second.is_ok());
        assert!(cleanup().is_ok());
    }

    #[test]
    fn create_db_test_schema_version() {
        let (db_path, cleanup) = create_temp_dir("create_db_test_schema_version");

        // create the db.
        let first_result = Database::init(&db_path);
        assert!(first_result.is_ok());
        let first = first_result.unwrap();

        // query its schema version and validate it
        let version_result = first.schema_version();
        assert!(version_result.is_ok());
        let version = version_result.unwrap();
        assert!(version == SCHEMA_VERSION);
        assert!(cleanup().is_ok());
    }
}

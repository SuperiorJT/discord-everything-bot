pub mod queries;
use std::sync::{Arc};

use rusqlite::{Connection, Result};
use tokio::sync::{Mutex};

use self::queries::{poll::SqlPollQueries, reaction_roles::SqlReactionRolesQueries};

pub struct Database(Arc<Mutex<Connection>>);

impl Database {
    pub fn new() -> Result<Self> {
        let conn = Connection::open_in_memory()?;
        Database::setup_tables(&conn)?;
        let db = Self(Arc::new(Mutex::new(conn)));
        Ok(db)
    }

    fn setup_tables(conn: &Connection) -> Result<()> {
        Database::create_poll_tables(conn)?;
        Database::create_reaction_roles_tables(conn)?;
        Ok(())
    }

    fn create_poll_tables(conn: &Connection) -> Result<()> {
        conn.execute(
            "CREATE TABLE poll (
                    id          INTEGER PRIMARY KEY,
                    author      TEXT,
                    question    TEXT,
                    start       DATE,
                    ends        DATE
            )",
            []
        )?;
        conn.execute(
            "CREATE TABLE poll_option (
                    id              INTEGER PRIMARY KEY,
                    positive_id     INTEGER,
                    positive_name   TEXT,
                    negative_id     INTEGER,
                    negative_name   TEXT
            )",
            []
        )?;
        Ok(())
    }

    fn create_reaction_roles_tables(conn: &Connection) -> Result<()> {
        conn.execute(
            "CREATE TABLE reaction_roles (
                    message_id  INTEGER PRIMARY KEY,
                    channel_id  INTEGER,
                    guild_id    INTEGER,
                    content     TEXT,
                    embeds      TEXT,
                    roles       TEXT
            )",
            []
        )?;
        Ok(())
    }

    pub fn poll(&self) -> SqlPollQueries {
        SqlPollQueries(self.0.clone())
    }

    pub fn reaction_roles(&self) -> SqlReactionRolesQueries {
        SqlReactionRolesQueries(self.0.clone())
    }
}

impl Clone for Database {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
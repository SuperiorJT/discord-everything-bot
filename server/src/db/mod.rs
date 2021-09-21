pub mod queries;
use std::sync::Arc;

use rusqlite::{Connection, Result, params};
use sqlx::{Executor, SqlitePool};
use tokio::sync::Mutex;
use twilight_model::id::GuildId;

use self::queries::{guild::GuildPluginState, poll::SqlPollQueries, reaction_roles::SqlReactionRolesQueries, welcome::WelcomeQueries};

pub struct OldDatabase(Arc<Mutex<Connection>>);

const SCHEMA_VERSION: u32 = 1;

impl OldDatabase {
    pub async fn new() -> Result<Self> {
        let conn = Connection::open_in_memory()?;
        OldDatabase::setup_tables(&conn)?;
        OldDatabase::validate_and_upgrade_schema(&conn)?;
        let db = Self(Arc::new(Mutex::new(conn)));
        Ok(db)
    }

    fn setup_tables(conn: &Connection) -> Result<()> {
        OldDatabase::create_version_table(conn)?;
        OldDatabase::create_poll_tables(conn)?;
        OldDatabase::create_welcome_tables(conn)?;
        OldDatabase::create_reaction_roles_tables(conn)?;
        Ok(())
    }

    fn validate_and_upgrade_schema(conn: &Connection) -> Result<()> {
        if let Ok(version) = OldDatabase::get_version(conn) {
            if version < SCHEMA_VERSION {
                // TODO: Implement schema migrations
                conn.execute("INSERT INTO version (version) VALUES (?1)", params![SCHEMA_VERSION])?;
            }
            return Ok(())
        }
        Ok(())
    }

    fn create_version_table(conn: &Connection) -> Result<()> {
        conn.execute(
            "CREATE TABLE IF NOT EXISTS version (version INTEGER)",
            []
        )?;
        Ok(())
    }

    fn create_poll_tables(conn: &Connection) -> Result<()> {
        conn.execute(
            "CREATE TABLE IF NOT EXISTS poll (
                    id          INTEGER PRIMARY KEY,
                    author      TEXT,
                    question    TEXT,
                    start       DATE,
                    ends        DATE
            )",
            []
        )?;
        conn.execute(
            "CREATE TABLE IF NOT EXISTS poll_option (
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

    fn create_welcome_tables(conn: &Connection) -> Result<()> {
        conn.execute(
            "CREATE TABLE IF NOT EXISTS welcome (
                guild_id    INTEGER PRIMARY KEY,
                enabled     BOOLEAN
            )",
            []
        )?;
        conn.execute(
            "CREATE TABLE IF NOT EXISTS welcome_join (
                guild_id        INTEGER PRIMARY KEY,
                enabled         BOOLEAN
                channel_id      INTEGER
                message_type    INTEGER
                content         TEXT
                embed           TEXT
            )",
            []
        )?;
        Ok(())
    }

    fn create_reaction_roles_tables(conn: &Connection) -> Result<()> {
        conn.execute(
            "CREATE TABLE IF NOT EXISTS reaction_roles (
                guild_id    INTEGER PRIMARY KEY,
                enabled     BOOLEAN
            )",
            []
        )?;
        conn.execute(
            "CREATE TABLE IF NOT EXISTS reaction_roles_message (
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

    pub fn get_version(conn: &Connection) -> Result<u32> {
        let version = conn.prepare("SELECT version FROM version LIMIT 1")?.query_map([], |row| row.get(0))?.next();
        version.unwrap_or_else(|| {
            conn.execute("INSERT INTO version (version) VALUES (?1)", params![SCHEMA_VERSION])?;
            Ok(SCHEMA_VERSION)
        })
    }

    pub fn poll(&self) -> SqlPollQueries {
        SqlPollQueries(self.0.clone())
    }

    pub fn reaction_roles(&self) -> SqlReactionRolesQueries {
        SqlReactionRolesQueries(self.0.clone())
    }

    pub async fn get_guild_state(&self, guild_id: GuildId) -> Result<GuildPluginState> {
        let conn = self.0.lock().await;
        let state = conn.prepare("
                SELECT welcome.guild_id AS guild_id, welcome.enabled, reaction_roles.enabled
                FROM welcome
                INNER JOIN reaction_roles ON welcome.guild_id = reaction_roles.guild_id
                WHERE welcome.guild_id = ?
            ")?
            .query_map(params![guild_id.0], |row| {
                Ok(GuildPluginState {
                    welcome: row.get(0).unwrap_or(false),
                    reaction_roles: row.get(1).unwrap_or(false)
                })
            })?.next();

        state.unwrap_or(Ok(GuildPluginState { welcome: false, reaction_roles: false }))
    }

    pub async fn validate_guilds(&self, guild_ids: Vec<GuildId>) -> Result<()> {
        let mut conn = self.0.lock().await;
        let ids_with_states = conn.prepare("
                SELECT welcome.guild_id AS guild_id
                FROM welcome
                INNER JOIN reaction_roles ON welcome.guild_id = reaction_roles.guild_id
            ")?
            .query_map([], |row| -> Result<u64> {
                row.get(0)
            })?.map(|row| row.unwrap()).collect::<Vec<u64>>();

        let ids_to_validate: Vec<u64> = guild_ids.iter().filter(|guild_id| !ids_with_states.contains(&guild_id.0)).map(|guild_id| guild_id.0).collect();
        let tx = conn.transaction()?;
        for guild_id in ids_to_validate.iter() {
            tx.execute("
                INSERT INTO welcome (guild_id, enabled)
                VALUES (?, false)
            ", params![guild_id])?;
            tx.execute("
                INSERT INTO reaction_roles (guild_id, enabled)
                VALUES (?, false)
            ", params![guild_id])?;
        }
        tx.commit()?;
        Ok(())
    }
}

impl Clone for OldDatabase {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}

pub struct Database {
    pub pool: SqlitePool
}

impl Database {
    pub async fn new(path: &str) -> sqlx::Result<Self> {
        let pool = SqlitePool::connect(path).await?;
        let db = Self { pool };
        Ok(db)
    }

    pub fn welcome(&self) -> WelcomeQueries {
        WelcomeQueries::new(self.pool.clone())
    }
}

impl Clone for Database {
    fn clone(&self) -> Self {
        Self {
            pool: self.pool.clone()
        }
    }
}
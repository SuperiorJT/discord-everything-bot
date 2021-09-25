use std::{error::Error, sync::Arc};

use rusqlite::{params, Connection};
use tokio::sync::Mutex;
use twilight_cache_inmemory::InMemoryCache;
use twilight_model::id::{EmojiId, GuildId, UserId};

use crate::modules::poll::Poll;

pub struct SqlPollQueries(pub Arc<Mutex<Connection>>);

impl SqlPollQueries {
    pub async fn create_poll(&self, poll: &mut Poll) -> Result<(), Box<dyn Error>> {
        let conn = self.0.lock().await;
        conn.execute(
            "INSERT INTO poll (author, question, start, ends) VALUES (?1, ?2, ?3, ?4)",
            params![poll.author.id.0, poll.question, poll.start, poll.ends],
        )?;
        poll.id = conn.last_insert_rowid();
        Ok(())
    }

    pub async fn fetch_polls(&self, cache: &InMemoryCache) -> Result<Vec<Poll>, Box<dyn Error>> {
        let conn = self.0.lock().await;
        let polls = conn
            .prepare("SELECT * from poll")?
            .query_map([], |row| {
                Ok(Poll {
                    id: row.get(0)?,
                    author: cache.user(UserId(row.get(1)?)).unwrap(),
                    question: row.get(2)?,
                    start: row.get(3)?,
                    ends: row.get(4)?,
                    options: None,
                })
            })?
            .map(|res| res.unwrap())
            .collect();
        Ok(polls)
    }

    pub async fn delete_poll(&self, id: i64) -> Result<(), Box<dyn Error>> {
        let conn = self.0.lock().await;
        conn.execute("DELETE poll WHERE id = ?", params![id])?;
        Ok(())
    }

    pub async fn fetch_poll_options(
        &self,
        guild_id: GuildId,
    ) -> Result<((EmojiId, String), (EmojiId, String)), Box<dyn Error>> {
        let conn = self.0.lock().await;
        let options = conn.query_row("SELECT positive_id, positive_name, negative_id, negative_name FROM poll_option WHERE id = ?", params![guild_id.0], |row| {
            Ok((
                (EmojiId(row.get(0)?), row.get(1)?),
                (EmojiId(row.get(2)?), row.get(3)?)
            ))
        })?;
        Ok(options)
    }

    pub async fn upsert_poll_options(
        &self,
        guild_id: GuildId,
        positive: (EmojiId, String),
        negative: (EmojiId, String),
    ) -> Result<(), Box<dyn Error>> {
        let conn = self.0.lock().await;
        conn.execute(
            "
            INSERT INTO poll_option (id, positive_id, positive_name, negative_id, negative_name)
            VALUES (?1 , ?2, ?3, ?4, ?5)
            ON CONFLICT(id) DO UPDATE SET
                positive_id=excluded.positive_id,
                positive_name=excluded.positive_name
                negative_id=excluded.negative_id
                negative_name=excluded.negative_name;
            ",
            params![
                guild_id.0,
                positive.0 .0,
                positive.1,
                negative.0 .0,
                negative.1
            ],
        )?;
        Ok(())
    }
}

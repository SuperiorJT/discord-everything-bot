use std::{error::Error, sync::Arc};

use rusqlite::{Connection, params};
use serde_json::json;
use tokio::sync::{Mutex};
use twilight_model::{id::{ChannelId, GuildId, MessageId}};

use crate::modules::reaction_roles::ReactionRolesMsg;

pub struct SqlReactionRolesQueries(pub Arc<Mutex<Connection>>);

impl SqlReactionRolesQueries {
    pub async fn create_message(&self, message: &ReactionRolesMsg) -> Result<(), Box<dyn Error>> {
        if let None = message.message_id {
            return Err("message_id is not set".into());
        }
        let conn = self.0.lock().await;
        conn.execute("
            INSERT INTO reaction_roles (
                message_id
                channel_id
                guild_id
                content
                embeds
                roles
            ) VALUES (
                ?1, ?2, ?3, ?4, ?5, ?6
            )
        ", params![message.message_id.unwrap().0, message.channel_id.0, message.guild_id.0, message.content, json!(message.embeds), json!(message.role_map)])?;
        Ok(())
    }

    pub async fn fetch_messages_for_guild(&self, guild_id: GuildId) -> Result<Vec<ReactionRolesMsg>, Box<dyn Error>> {
        let conn = self.0.lock().await;
        let messages = conn.prepare("SELECT * FROM reaction_roles WHERE guild_id = ?")?.query_map(params![guild_id.0], |row| {
            Ok(ReactionRolesMsg {
                message_id: Some(MessageId(row.get(0)?)),
                channel_id: ChannelId(row.get(1)?),
                guild_id,
                content: row.get(3)?,
                embeds: serde_json::from_value(row.get(4)?).expect("Could not deserialize embeds"),
                role_map: serde_json::from_value(row.get(5)?).expect("Could not deserialize role map"),
            })
        })?.map(|res| res.unwrap()).collect();
        
        Ok(messages)
    }
}
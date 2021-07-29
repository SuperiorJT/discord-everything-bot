use std::{collections::HashMap, error::Error};

use serde::{Serialize, Deserialize};

use twilight_http::{Client, request::{AuditLogReason, prelude::RequestReactionType}};
use twilight_model::{
    channel::{embed::Embed, ReactionType},
    gateway::payload::{ReactionAdd, ReactionRemove},
    id::{ChannelId, EmojiId, GuildId, MessageId, RoleId},
};

use crate::bot::event_handler::EventHandler;

#[derive(Serialize, Deserialize, Debug)]
pub struct ReactionRolesMsg {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message_id: Option<MessageId>,
    pub channel_id: ChannelId,
    pub guild_id: GuildId,
    pub content: String,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub embeds: Vec<Embed>,
    #[serde(rename = "roles")]
    pub role_map: HashMap<EmojiId, RoleId>,
}

impl ReactionRolesMsg {
    pub async fn handle_reaction_add(
        &self,
        reaction_add: ReactionAdd,
        event_handler: &EventHandler<'_>,
    ) -> Result<(), Box<dyn Error + Send + Sync>> {
        let emoji_id = match &reaction_add.emoji {
            ReactionType::Custom { animated: _, id, name: _ } => id,
            ReactionType::Unicode { name: _ } => return Err("Invalid emoji reaction".into()),
        };
        event_handler
            .bot
            .http
            .add_guild_member_role(
                reaction_add.guild_id.unwrap(),
                reaction_add.user_id,
                self.role_map.get(emoji_id).unwrap().to_owned(),
            )
            .reason("Reaction Role")?
            .await?;

        Ok(())
    }

    pub async fn handle_reaction_remove(
        &self,
        reaction_remove: ReactionRemove,
        event_handler: &EventHandler<'_>,
    ) -> Result<(), Box<dyn Error + Send + Sync>> {
        let emoji_id = match &reaction_remove.emoji {
            ReactionType::Custom { animated: _, id, name: _ } => id,
            ReactionType::Unicode { name: _ } => return Err("Invalid emoji reaction".into()),
        };
        event_handler
            .bot
            .http
            .remove_guild_member_role(
                reaction_remove.guild_id.unwrap(),
                reaction_remove.user_id,
                self.role_map.get(emoji_id).unwrap().to_owned(),
            )
            .reason("Reaction Role")?
            .await?;

        Ok(())
    }

    /// Post the reaction role message to its designated channel.
    /// This will update the message id to the value returned from create_message.
    ///
    /// Will Err if the message is already posted.
    pub async fn post(&mut self, http: &Client) -> Result<(), Box<dyn Error + Send + Sync>> {
        if let Some(_) = self.message_id {
            return Err("Reaction Role already posted: Message ID is not empty".into());
        }

        let message = http.create_message(self.channel_id).content(self.content.clone())?.embeds(self.embeds.clone())?.await?;

        self.message_id = Some(message.id);

        // react with all of the emoji in the role map
        for key in self.role_map.keys() {
            http.create_reaction(self.channel_id, message.id, RequestReactionType::Custom {
                id: *key,
                name: None,
            }).await?;
        }

        Ok(())
    }
}

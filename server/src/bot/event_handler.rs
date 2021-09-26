use std::error::Error;

use twilight_gateway::Event;
use twilight_model::{
    application::{
        callback::{CallbackData, InteractionResponse},
        interaction::{ApplicationCommand, Interaction},
    },
    channel::embed::Embed,
};

use crate::modules::{slash_commands, welcome::handle_member_add};

use super::DiscordBot;

/// Utility struct to store state and services used for event handling.
///
/// Call the [`EventHandler::handle_event()`] function to handle a specific event.
pub struct EventHandler<'a> {
    pub bot: &'a DiscordBot,
    pub shard_id: u64,
}

impl EventHandler<'_> {
    pub async fn handle_event(
        bot: &DiscordBot,
        shard_id: u64,
        event: Event,
    ) -> Result<(), Box<dyn Error + Send + Sync>> {
        let handler = EventHandler { bot, shard_id };

        println!("HANDLING EVENT: {:?}\n", event);

        match event {
            Event::MemberAdd(member_add) => handle_member_add(member_add, &handler).await?,
            Event::InteractionCreate(interaction) => match interaction.0 {
                Interaction::ApplicationCommand(command) => {
                    slash_commands::process(&command, &handler).await?
                }
                _ => {}
            },
            Event::ReactionAdd(_reaction) => {}
            Event::ReactionRemove(_reaction) => {}
            _ => {}
        }

        Ok(())
    }

    pub async fn simple_interaction_reply(
        &self,
        command: &Box<ApplicationCommand>,
        message: &str,
    ) -> Result<(), Box<dyn Error + Send + Sync>> {
        self.bot
            .http
            .interaction_callback(
                command.id,
                &command.token,
                &InteractionResponse::ChannelMessageWithSource(CallbackData {
                    allowed_mentions: None,
                    components: None,
                    content: Some(message.into()),
                    embeds: vec![],
                    flags: None,
                    tts: None,
                }),
            )
            .exec()
            .await?;

        Ok(())
    }

    pub async fn embed_interaction_reply(
        &self,
        command: &Box<ApplicationCommand>,
        embeds: Vec<Embed>,
    ) -> Result<(), Box<dyn Error + Send + Sync>> {
        self.bot
            .http
            .interaction_callback(
                command.id,
                &command.token,
                &InteractionResponse::ChannelMessageWithSource(CallbackData {
                    allowed_mentions: None,
                    components: None,
                    content: None,
                    embeds,
                    flags: None,
                    tts: None,
                }),
            )
            .exec()
            .await?;

        Ok(())
    }
}

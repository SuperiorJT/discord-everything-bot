use std::{error::Error, fmt::Display};

use twilight_model::application::interaction::ApplicationCommand;

use crate::bot::event_handler::EventHandler;

use super::poll::slash_commands::{PollCommand, PollEmojiCommand};

#[derive(Debug)]
pub enum SlashCommandError {
    CannotProcessUnknownCommand,
}

impl Display for SlashCommandError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            SlashCommandError::CannotProcessUnknownCommand => {
                f.write_str("Cannot process unknown command")
            }
        }
    }
}

impl Error for SlashCommandError {}

pub async fn process(
    command: &Box<ApplicationCommand>,
    event_handler: &EventHandler<'_>,
) -> Result<(), Box<dyn Error + Send + Sync>> {
    match command.data.name.as_str() {
        "ping" => PingCommand(command).process_command(event_handler).await,
        "poll" => PollCommand(command).process_command(event_handler).await,
        "poll_emoji" => {
            PollEmojiCommand(command)
                .process_command(event_handler)
                .await
        }
        _ => Err(Box::new(SlashCommandError::CannotProcessUnknownCommand)),
    }
}
pub struct PingCommand<'a>(pub &'a Box<ApplicationCommand>);

impl<'a> PingCommand<'a> {
    pub async fn process_command(
        &self,
        event_handler: &EventHandler<'a>,
    ) -> Result<(), Box<dyn Error + Send + Sync>> {
        event_handler
            .simple_interaction_reply(self.0, "Pong!")
            .await?;
        Ok(())
    }
}

pub mod event_handler;

use std::error::Error;

use twilight_cache_inmemory::InMemoryCache;
use twilight_gateway::{Cluster, Event};
use twilight_http::Client;
use twilight_model::{
    application::command::{ChoiceCommandOptionData, Command, CommandOption, CommandType},
    id::GuildId,
};

use crate::db::Database;

use self::event_handler::EventHandler;

#[derive(Clone)]
pub struct DiscordBot {
    pub cluster: Cluster,
    pub db: Database,
    pub discord_cache: InMemoryCache,
    pub http: Client,
}

impl DiscordBot {
    pub fn new(cluster: Cluster, db: Database, discord_cache: InMemoryCache, http: Client) -> Self {
        Self {
            cluster,
            db,
            discord_cache,
            http,
        }
    }

    pub async fn start(&self) -> Result<(), Box<dyn Error + Send + Sync>> {
        self.cluster.up().await;
        let _guild_ids = self
            .http
            .current_user_guilds()
            .exec()
            .await?
            .models()
            .await?
            .iter()
            .map(|g| g.id)
            .collect::<Vec<GuildId>>();
        // self.db.validate_guilds(guild_ids).await?;
        self.set_up_global_commands().await?;

        Ok(())
    }

    async fn set_up_global_commands(&self) -> Result<(), Box<dyn Error + Send + Sync>> {
        let commands = vec![
            Command {
                application_id: Some(self.http.application_id().unwrap()),
                guild_id: None,
                name: "ping".into(),
                default_permission: None,
                description: "Ping Bingus".into(),
                id: None,
                kind: CommandType::ChatInput,
                options: vec![],
            },
            Command {
                application_id: Some(self.http.application_id().unwrap()),
                guild_id: None,
                name: "poll".into(),
                default_permission: None,
                description: "Create a poll".into(),
                id: None,
                kind: CommandType::ChatInput,
                options: vec![
                    CommandOption::String(ChoiceCommandOptionData {
                        choices: vec![],
                        description: "The question you are polling".into(),
                        name: "question".into(),
                        required: true,
                    }),
                    CommandOption::String(ChoiceCommandOptionData {
                        choices: vec![],
                        description: "When do you want to stop polling?".into(),
                        name: "ends".into(),
                        required: false,
                    }),
                ],
            },
        ];
        // http.set_global_commands(commands.clone()).unwrap();
        let id = dotenv::var("DEFAULT_GUILD_ID")
            .map(|id| {
                id.parse::<u64>()
                    .expect("Failed to parse env var DEFAULT_GUILD_ID")
            })
            .expect("env file is missing DEFAULT_GUILD_ID");
        self.http
            .set_guild_commands(GuildId(id), &commands)?
            .exec()
            .await?;
        Ok(())
    }

    pub async fn handle_event(
        &self,
        shard_id: u64,
        event: Event,
    ) -> Result<(), Box<dyn Error + Send + Sync>> {
        EventHandler::handle_event(self, shard_id, event).await?;

        Ok(())
    }
}

mod api;
mod bot;
mod event_runner;
mod modules;

use std::error::Error;
use twilight_cache_inmemory::{InMemoryCache, ResourceType};
use twilight_gateway::{cluster::ShardScheme, Cluster};
use twilight_http::{Client as HttpClient};
use twilight_model::{
    gateway::Intents,
    id::{ApplicationId}
};

use crate::{bot::{DiscordBot, db::Database}, event_runner::{run, EventRunner}};

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error + Send + Sync>> {
    let token = dotenv::var("DISCORD_TOKEN")?;
    let application_id = ApplicationId(dotenv::var("DISCORD_APP_ID").unwrap().parse().unwrap());

    let scheme = ShardScheme::Auto;

    let http = HttpClient::new(&token);
    http.set_application_id(application_id);

    let cache = InMemoryCache::builder()
        .resource_types(ResourceType::MESSAGE)
        .resource_types(ResourceType::USER_CURRENT)
        .build();

    let (cluster, events) = Cluster::builder(&token, Intents::GUILD_MESSAGES)
        .shard_scheme(scheme)
        .build()
        .await?;

    let db = Database::new()?;

    let routes = api::routes(db.clone(), http.clone());

    let serve = warp::serve(routes).run(([127, 0, 0, 1], 3030));

    let bot = DiscordBot::new(cluster, db.clone(), cache.clone(), http.clone());
    bot.start().await?;

    let event_runner = EventRunner::new(bot, cache.clone());

    tokio::join!(serve, run(event_runner, events));

    Ok(())
}

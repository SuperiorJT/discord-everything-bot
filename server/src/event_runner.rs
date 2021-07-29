use std::error::Error;

use futures::{stream::StreamExt, Stream};
use twilight_cache_inmemory::InMemoryCache;
use twilight_gateway::Event;

use crate::bot::DiscordBot;

#[derive(Clone)]
pub struct EventRunner {
    bot: DiscordBot,
    cache: InMemoryCache,
}

impl EventRunner {
    pub fn new(bot: DiscordBot, cache: InMemoryCache) -> Self {
        Self { bot, cache }
    }
}

pub async fn run(event_runner: EventRunner, mut events: impl Stream<Item = (u64, Event)> + Send + Sync + Unpin) {
    while let Some((shard_id, event)) = events.next().await {
        event_runner.cache.update(&event);
        tokio::spawn(handle_event(event_runner.clone(), shard_id, event));
    }
}

async fn handle_event(event_runner: EventRunner, shard_id: u64, event: Event) -> Result<(), Box<dyn Error + Send + Sync>> {
    event_runner.bot.handle_event(shard_id, event).await?;

    Ok(())
}

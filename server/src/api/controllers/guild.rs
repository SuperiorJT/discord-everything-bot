use std::convert::Infallible;

use twilight_cache_inmemory::InMemoryCache;
use twilight_http::Client;
use twilight_model::{channel::GuildChannel, id::GuildId};
use warp::{hyper::StatusCode, Reply};

use crate::api::util;

pub async fn get_guild_emojis(
    guild_id: u64,
    client: Client,
) -> Result<impl warp::Reply, Infallible> {
    match client.emojis(GuildId(guild_id)).exec().await {
        Ok(emojis) => {
            if let Ok(emojis) = emojis.models().await {
                Ok(warp::reply::json(&emojis).into_response())
            } else {
                Ok(util::create_error_response(
                    StatusCode::INTERNAL_SERVER_ERROR,
                    format!("Failed to deserialize emojis"),
                ))
            }
        }
        Err(_) => Ok(util::create_error_response(
            StatusCode::INTERNAL_SERVER_ERROR,
            format!("Failed to get guild emojis"),
        )),
    }
}

pub async fn get_guild_roles(
    guild_id: u64,
    client: Client,
) -> Result<impl warp::Reply, Infallible> {
    match client.roles(GuildId(guild_id)).exec().await {
        Ok(roles) => {
            if let Ok(roles) = roles.models().await {
                Ok(warp::reply::json(&roles).into_response())
            } else {
                Ok(util::create_error_response(
                    StatusCode::INTERNAL_SERVER_ERROR,
                    format!("Failed to deserialize guild roles"),
                ))
            }
        }
        Err(_) => Ok(util::create_error_response(
            StatusCode::INTERNAL_SERVER_ERROR,
            format!("Failed to get guild roles"),
        )),
    }
}

pub async fn get_guild_channels(
    guild_id: u64,
    client: Client,
    cache: InMemoryCache,
) -> Result<impl warp::Reply, Infallible> {
    if let Some(channels) = cache.guild_channels(GuildId(guild_id)).and_then(|ids| {
        let mut cached_channels = ids.iter().map(|id| cache.guild_channel(*id));
        match cached_channels.all(|opt| opt.is_some()) {
            true => Some(
                cached_channels
                    .map(|opt| opt.unwrap())
                    .collect::<Vec<GuildChannel>>(),
            ),
            false => None,
        }
    }) {
        return Ok(warp::reply::json(&channels).into_response());
    }
    match client.guild_channels(GuildId(guild_id)).exec().await {
        Ok(channels) => {
            if let Ok(channels) = channels.models().await {
                Ok(warp::reply::json(&channels).into_response())
            } else {
                Ok(util::create_error_response(
                    StatusCode::INTERNAL_SERVER_ERROR,
                    format!("Failed to deserialize guild channels"),
                ))
            }
        }
        Err(_) => Ok(util::create_error_response(
            StatusCode::INTERNAL_SERVER_ERROR,
            format!("Failed to get guild channels"),
        )),
    }
}

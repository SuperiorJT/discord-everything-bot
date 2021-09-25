use twilight_cache_inmemory::InMemoryCache;
use twilight_http::Client;
use warp::Filter;

use crate::api::{
    controllers::guild::{get_guild_channels, get_guild_emojis, get_guild_roles},
    with_cache, with_client,
};

pub fn guild_routes(
    client: Client,
    cache: InMemoryCache,
) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    emoji(client.clone()).or(roles(client.clone()).or(channels(client, cache)))
}

fn emoji(
    client: Client,
) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    warp::path!(u64 / "emotes")
        .and(with_client(client))
        .and_then(get_guild_emojis)
}

fn roles(
    client: Client,
) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    warp::path!(u64 / "roles")
        .and(with_client(client))
        .and_then(get_guild_roles)
}

fn channels(
    client: Client,
    cache: InMemoryCache,
) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    warp::path!(u64 / "channels")
        .and(with_client(client))
        .and(with_cache(cache))
        .and_then(get_guild_channels)
}

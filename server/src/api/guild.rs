use std::convert::Infallible;

use twilight_model::id::GuildId;
use warp::Filter;
use warp::Reply;
use warp::hyper::StatusCode;

use super::Client;
use super::ErrorMessage;
use super::with_client;

pub fn guild_routes(client: Client) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    emoji(client.clone())
        .or(roles(client.clone()))
}

fn emoji(client: Client) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    warp::path!(u64 / "emotes")
        .and(with_client(client))
        .and_then(get_guild_emojis)
}

fn roles(client: Client) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    warp::path!(u64 / "roles")
        .and(with_client(client))
        .and_then(get_guild_roles)
}

async fn get_guild_emojis(guild_id: u64, client: Client) -> Result<impl warp::Reply, Infallible> {
    match client.emojis(GuildId(guild_id)).await {
        Ok(emojis) => Ok(warp::reply::json(&emojis).into_response()),
        Err(_) => {
            let json = warp::reply::json(&ErrorMessage {
                code: StatusCode::INTERNAL_SERVER_ERROR.as_u16(),
                message: "Failed to get guild emojis".to_string(),
            });
            Ok(warp::reply::with_status(json, StatusCode::INTERNAL_SERVER_ERROR).into_response())
        },
    }
}

async fn get_guild_roles(guild_id: u64, client: Client) -> Result<impl warp::Reply, Infallible> {
    match client.roles(GuildId(guild_id)).await {
        Ok(roles) => Ok(warp::reply::json(&roles).into_response()),
        Err(_) => {
            let json = warp::reply::json(&ErrorMessage {
                code: StatusCode::INTERNAL_SERVER_ERROR.as_u16(),
                message: "Failed to get guild roles".to_string(),
            });
            Ok(warp::reply::with_status(json, StatusCode::INTERNAL_SERVER_ERROR).into_response())
        },
    }
}
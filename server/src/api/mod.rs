mod controllers;
mod models;
mod recover;
mod routes;
mod util;

use twilight_cache_inmemory::InMemoryCache;
use twilight_http::Client;
use warp::{hyper::Method, Filter};

use crate::db::Database;

use serde::Serialize;

use self::routes::{guild::guild_routes, welcome::welcome_routes};

#[derive(Serialize)]
struct ErrorMessage {
    code: u16,
    message: String,
}

fn with_db(
    db: Database,
) -> impl Filter<Extract = (Database,), Error = std::convert::Infallible> + Clone {
    warp::any().map(move || db.clone())
}

fn with_client(
    client: Client,
) -> impl Filter<Extract = (Client,), Error = std::convert::Infallible> + Clone {
    warp::any().map(move || client.clone())
}

fn with_cache(
    cache: InMemoryCache,
) -> impl Filter<Extract = (InMemoryCache,), Error = std::convert::Infallible> + Clone {
    warp::any().map(move || cache.clone())
}

pub fn routes(
    db: Database,
    client: Client,
    cache: InMemoryCache,
) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    let cors = warp::cors()
        .allow_origin("http://localhost:3000")
        .allow_header("content-type")
        .allow_methods(&[Method::GET, Method::POST, Method::DELETE]);

    guild_routes(client, cache)
        .or(welcome_routes(db))
        .recover(recover::handle_rejection)
        .with(cors)
}

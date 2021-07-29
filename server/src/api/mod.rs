mod guild;
use warp::{Filter};
use twilight_http::Client;

use crate::bot::db::Database;

use serde::{Serialize};

use self::guild::guild_routes;

#[derive(Serialize)]
struct ErrorMessage {
    code: u16,
    message: String
}

fn with_db(db: Database) -> impl Filter<Extract = (Database,), Error = std::convert::Infallible> + Clone {
    warp::any().map(move || db.clone())
}

fn with_client(client: Client) -> impl Filter<Extract = (Client,), Error = std::convert::Infallible> + Clone {
    warp::any().map(move || client.clone())
}

pub fn routes(db: Database, client: Client) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    guild_routes(client.clone())
}
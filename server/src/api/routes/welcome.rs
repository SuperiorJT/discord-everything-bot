use warp::Filter;

use crate::{api::{controllers::welcome::{fetch_module_for_guild, update_module_for_guild, upload_images_for_guild}, with_db}, db::Database};

pub fn welcome_routes(db: Database) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    fetch(db.clone())
        .or(update(db))
        .or(image_upload())
}

fn fetch(db: Database) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    warp::path!(u64 / "welcome")
        .and(warp::get())
        .and(with_db(db))
        .and_then(fetch_module_for_guild)
}

fn update(db: Database) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    warp::path!(u64 / "welcome")
        .and(warp::post())
        .and(warp::body::json())
        .and(with_db(db))
        .and_then(update_module_for_guild)
}

fn image_upload() -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    warp::path!(u64 / "welcome" / "images")
        .and(warp::post())
        .and(warp::multipart::form())
        .and_then(upload_images_for_guild)
}
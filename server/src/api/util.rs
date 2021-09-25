use crate::api::ErrorMessage;
use warp::Reply;

pub fn create_error_response(
    code: warp::http::StatusCode,
    message: String,
) -> warp::reply::Response {
    let json = warp::reply::json(&ErrorMessage {
        code: code.as_u16(),
        message: message,
    });
    return warp::reply::with_status(json, code).into_response();
}

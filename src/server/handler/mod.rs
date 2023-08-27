pub mod redirect_map;
pub mod webfinger;

pub fn bad_request() -> Box<dyn warp::Reply> {
    Box::new(warp::reply::with_status(
        "Bad request",
        warp::http::StatusCode::BAD_REQUEST,
    ))
}

pub fn not_found() -> Box<dyn warp::Reply> {
    Box::new(warp::reply::with_status(
        "Not found",
        warp::http::StatusCode::NOT_FOUND,
    ))
}

pub fn internal_server_error() -> Box<dyn warp::Reply> {
    Box::new(warp::reply::with_status(
        "Internal Server Error",
        warp::http::StatusCode::INTERNAL_SERVER_ERROR,
    ))
}

pub fn gone() -> Box<dyn warp::Reply> {
    Box::new(warp::reply::with_status(
        "Gone",
        warp::http::StatusCode::GONE,
    ))
}

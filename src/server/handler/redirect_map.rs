use std::sync::Arc;

use archivedon::redirect_map::RedirectMap;
use log::error;
use warp::filters::path::FullPath;

use crate::server::env::Env;
use crate::server::handler;

pub async fn handle(
    env: Arc<Env>,
    host: String,
    path: FullPath,
    accept: Option<String>,
) -> Result<Box<dyn warp::Reply>, warp::Rejection> {
    let resource_path = env.resource_path.redirect_map_path(&host, path.as_str());
    match tokio::fs::try_exists(&resource_path).await {
        Ok(false) => return Ok(handler::not_found()),
        Err(err) => {
            error!(
                "Failed to access resource path: path={}, err={}",
                &resource_path.display(),
                err
            );
            return Ok(handler::internal_server_error());
        }
        Ok(true) => {
            // do nothing
        }
    }

    let resource = match tokio::fs::read(&resource_path).await {
        Ok(x) => x,
        Err(err) => {
            error!(
                "Failed to access resource path: path={}, err={}",
                &resource_path.display(),
                err
            );
            return Ok(handler::internal_server_error());
        }
    };
    let resource: RedirectMap = match serde_json::from_slice(&resource) {
        Ok(x) => x,
        Err(err) => {
            error!(
                "Failed to deserialize resource: path={}, err={}",
                &resource_path.display(),
                err
            );
            return Ok(handler::bad_request());
        }
    };

    let mut redirect_url_opt = None;
    // TODO: full support accept header syntax
    if let Some(accept) = accept {
        if let Some(url) = resource.get_entry(&accept) {
            redirect_url_opt = Some(url)
        }
    }

    if redirect_url_opt.is_none() {
        if let Some(url) = resource.get_entry("*/*") {
            redirect_url_opt = Some(url)
        }
    }

    let redirect_url = match redirect_url_opt {
        None => return Err(warp::reject()),
        Some(x) => x,
    };

    let reply = warp::reply::reply();
    let reply = warp::reply::with_status(reply, warp::http::StatusCode::MOVED_PERMANENTLY);
    let reply = warp::reply::with_header(reply, "Access-Control-Allow-Origin", "*");
    let reply = warp::reply::with_header(reply, "Location", redirect_url.to_string());

    Ok(Box::new(reply))
}

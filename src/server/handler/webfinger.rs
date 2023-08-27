use archivedon::webfinger;
use log::error;
use std::sync::Arc;

use crate::server::env::Env;
use crate::server::handler;

pub async fn handle(
    env: Arc<Env>,
    params: Vec<(String, String)>,
) -> Result<Box<dyn warp::Reply>, warp::Rejection> {
    let params = QueryParams::parse(params)?;
    handle_resource(env, &params.resource, params.rel).await
}

#[derive(Debug)]
struct QueryParams {
    resource: String,
    rel: Vec<String>,
}

impl QueryParams {
    fn parse(params: Vec<(String, String)>) -> Result<Self, warp::Rejection> {
        let mut resource: Option<String> = None;
        let mut rel: Vec<String> = vec![];

        for (key, value) in params {
            match key.as_str() {
                "resource" => {
                    resource = Some(value);
                }
                "rel" => {
                    rel.push(value);
                }
                _ => {
                    // do nothing
                }
            }
        }

        match resource {
            None => Err(warp::reject()),
            Some(resource) => Ok(Self { resource, rel }),
        }
    }
}

async fn handle_resource(
    env: Arc<Env>,
    resource: &str,
    rel: Vec<String>,
) -> Result<Box<dyn warp::Reply>, warp::Rejection> {
    let resource_path = env.resource_path.webfinger_path(resource);

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
    let mut resource: webfinger::resource::Resource = match serde_json::from_slice(&resource) {
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
    if !rel.is_empty() {
        resource.links = match resource.links {
            None => None,
            Some(links) => {
                let mut dest = vec![];
                for link in links {
                    if rel.iter().any(|x| x == &link.rel) {
                        dest.push(link);
                    }
                }
                Some(dest)
            }
        }
    }

    let reply = warp::reply::json(&resource);
    let reply = warp::reply::with_header(reply, "Access-Control-Allow-Origin", "*");
    let reply = warp::reply::with_header(reply, "Content-Type", "application/jrd+json");

    Ok(Box::new(reply))
}

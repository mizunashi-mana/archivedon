use crate::webfinger;
use crate::cmd::serve;

pub async fn handle(
    params: Vec<(String, String)>,
) -> Result<Box<dyn warp::Reply>, warp::Rejection> {
    let params = QueryParams::parse(params)?;

    if let Some(resource) = params.resource.strip_prefix("acct:") {
        match resource.split_once('@') {
            None => Ok(serve::bad_request()),
            Some((username, domain)) => {
                handle_account(username, domain, params.rel).await
            }
        }
    } else {
        Ok(serve::bad_request())
    }
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
            None => {
                Err(warp::reject())
            }
            Some(resource) => {
                Ok(Self {
                    resource,
                    rel,
                })
            }
        }
    }
}

async fn handle_account(username: &str, domain: &str, rel: Vec<String>) -> Result<Box<dyn warp::Reply>, warp::Rejection> {
    if domain != "localhost" {
        return Ok(serve::not_found());
    }

    if username != "sample" {
        return Ok(serve::not_found());
    }

    if !rel.is_empty() {
        return Ok(serve::bad_request());
    }

    let result = webfinger::resource::Resource {
        subject: format!("acct:{username}@{domain}"),
        aliases: None,
        properties: None,
        links: None,
    };
    let reply = warp::reply::json(&result);
    let reply = warp::reply::with_header(reply, "Access-Control-Allow-Origin", "*");
    let reply = warp::reply::with_header(reply, "Content-Type", "application/jrd+json");

    Ok(Box::new(reply))
}

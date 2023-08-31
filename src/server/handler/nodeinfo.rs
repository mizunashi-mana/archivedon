use std::sync::Arc;

use archivedon::nodeinfo::{
    Discovery, DiscoveryItem, MetadataItems, NodeInfo, ServicesItems, SoftwareItems, UsageItems,
    UsersItems,
};

use crate::server::env::{self, Env};

pub async fn handle_discovery(env: Arc<Env>) -> Result<Box<dyn warp::Reply>, warp::Rejection> {
    let nodeinfo_url = match env.expose_url_base.join("/archivedon/nodeinfo/2.1.json") {
        Ok(x) => x,
        Err(err) => panic!("unreachable: Given URL is valid: {err}"),
    };

    let reply = warp::reply::json(&Discovery {
        links: vec![DiscoveryItem {
            rel: "http://nodeinfo.diaspora.software/ns/schema/2.1".to_string(),
            href: nodeinfo_url.to_string(),
        }],
    });
    let reply = warp::reply::with_header(reply, "Access-Control-Allow-Origin", "*");
    let reply = warp::reply::with_header(reply, "Content-Type", "application/jrd+json");

    Ok(Box::new(reply))
}

pub async fn handle_resource(_: String) -> Result<Box<dyn warp::Reply>, warp::Rejection> {
    let reply = warp::reply::json(&NodeInfo {
        version: "2.1".to_string(),
        software: SoftwareItems {
            name: env::PROG_NAME.to_string(),
            version: env::PROG_VERSION.to_string(),
            repository: env::PROG_REPOSITORY.map(|x| x.to_string()),
            homepage: env::PROG_HOMEPAGE.map(|x| x.to_string()),
        },
        protocols: vec!["activitypub".to_string()],
        services: ServicesItems {
            inbound: vec![],
            outbound: vec![],
        },
        open_registrations: false,
        usage: UsageItems {
            users: UsersItems {
                total: None,
                active_halfyear: None,
                active_month: None,
            },
            local_posts: None,
            local_comments: None,
        },
        metadata: MetadataItems {
            node_name: None,
            node_description: None,
            maintainer: None,
        },
    });
    let reply = warp::reply::with_header(reply, "Access-Control-Allow-Origin", "*");
    let reply =
        warp::reply::with_header(reply, "Content-Type", "application/jrd+json; charset=utf-8");

    Ok(Box::new(reply))
}

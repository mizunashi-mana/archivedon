use std::{
    error::Error,
    net::{IpAddr, SocketAddr},
    path::Path,
};
use url::Url;
use warp::Filter;

use crate::server::env::Env;
use crate::server::handler;

pub async fn run(
    addr_opt: &Option<String>,
    port: u16,
    resource_dir: &str,
    expose_url_base: &str,
) -> Result<(), Box<dyn Error>> {
    let expose_url_base = Url::parse(expose_url_base)?;
    let env = Env::load(Path::new(resource_dir), expose_url_base);

    let addr = match addr_opt {
        Some(addr_raw) => addr_raw.parse()?,
        None => IpAddr::from([0, 0, 0, 0]),
    };
    let sock_addr = SocketAddr::new(addr, port);

    let static_dir = env.resource_path.static_root_dir.clone();
    let index_html_path = env.resource_path.index_html_path.clone();
    let with_env = move || env.clone();

    let top = warp::get()
        .and(warp::path::end())
        .and(warp::fs::file(index_html_path));

    let webfinger = warp::get()
        .and(warp::path!(".well-known" / "webfinger"))
        .map(with_env.clone())
        .and(warp::query::<Vec<(String, String)>>())
        .and_then(handler::webfinger::handle);

    let nodeinfo_discovery = warp::get()
        .and(warp::path!(".well-known" / "nodeinfo"))
        .map(with_env.clone())
        .and_then(handler::nodeinfo::handle_discovery);

    let nodeinfo_resource = warp::get()
        .and(warp::path!("archivedon" / "nodeinfo" / String))
        .and_then(handler::nodeinfo::handle_resource);

    let static_resource = warp::path("static").and(warp::fs::dir(static_dir));

    let redirect_map = warp::get()
        .map(with_env.clone())
        .and(warp::header("host"))
        .and(warp::filters::path::full())
        .and(warp::header::optional("accept"))
        .and_then(handler::redirect_map::handle);

    let gone_get = warp::get().map(|| handler::gone());

    let gone_post = warp::post().map(|| handler::gone());

    let service = top
        .or(webfinger)
        .or(nodeinfo_discovery)
        .or(nodeinfo_resource)
        .or(static_resource)
        .or(redirect_map)
        .or(gone_get)
        .or(gone_post);

    let (_, server) = warp::serve(service).try_bind_ephemeral(sock_addr)?;
    server.await;

    Ok(())
}

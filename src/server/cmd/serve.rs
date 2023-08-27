use std::{
    error::Error,
    net::{IpAddr, SocketAddr},
    path::Path,
};
use warp::Filter;

use crate::server::handler::webfinger;
use crate::server::{env::Env, handler::redirect_map};

pub async fn run(
    addr_opt: &Option<String>,
    port: u16,
    resource_dir: &str,
) -> Result<(), Box<dyn Error>> {
    let env = Env::load(Path::new(resource_dir));

    let addr = match addr_opt {
        Some(addr_raw) => addr_raw.parse()?,
        None => IpAddr::from([0, 0, 0, 0]),
    };
    let sock_addr = SocketAddr::new(addr, port);

    let static_dir = env.resource_path.static_root_dir.clone();
    let with_env = move || env.clone();

    let webfinger = warp::get()
        .and(warp::path!(".well-known" / "webfinger"))
        .map(with_env.clone())
        .and(warp::query::<Vec<(String, String)>>())
        .and_then(webfinger::handle);

    let static_resource = warp::path("static").and(warp::fs::dir(static_dir));

    let redirect_map = warp::get()
        .map(with_env.clone())
        .and(warp::header("host"))
        .and(warp::filters::path::full())
        .and(warp::header::optional("accept"))
        .and_then(redirect_map::handle);

    let service = webfinger.or(static_resource).or(redirect_map);

    let (_, server) = warp::serve(service).try_bind_ephemeral(sock_addr)?;
    server.await;

    Ok(())
}

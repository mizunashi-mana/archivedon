use std::{
    error::Error,
    net::{IpAddr, SocketAddr},
};
use warp::Filter;

use crate::server::env;
use crate::server::handler::webfinger;

pub async fn run(addr_opt: &Option<String>, port: u16) -> Result<(), Box<dyn Error>> {
    let env = env::env(vec!["localhost".to_string()]);

    let addr = match addr_opt {
        Some(addr_raw) => addr_raw.parse()?,
        None => IpAddr::from([0, 0, 0, 0]),
    };
    let sock_addr = SocketAddr::new(addr, port);

    let webfinger = warp::get()
        .and(warp::path!(".well-known" / "webfinger"))
        .map(move || env.clone())
        .and(warp::query::<Vec<(String, String)>>())
        .and_then(webfinger::handle);

    let user = warp::get()
        .and(warp::path!("users" / String))
        .and(warp::header::exact("accept", "application/activity+json"))
        .and_then(handle_user);

    let user_redirect = warp::get()
        .and(warp::path!("users" / String))
        .and_then(handle_user_redirect);

    let profile = warp::get()
        .and(warp::path!(String))
        .and_then(handle_profile);

    let service = webfinger.or(user).or(user_redirect).or(profile);

    let (_, server) = warp::serve(service).try_bind_ephemeral(sock_addr)?;
    server.await;

    Ok(())
}

async fn handle_user(name: String) -> Result<Box<dyn warp::Reply>, warp::Rejection> {
    Ok(Box::new(format!("name={}", name)))
}

async fn handle_user_redirect(name: String) -> Result<Box<dyn warp::Reply>, warp::Rejection> {
    Ok(Box::new(format!("name={}", name)))
}

async fn handle_profile(name: String) -> Result<Box<dyn warp::Reply>, warp::Rejection> {
    Ok(Box::new(format!("name={}", name)))
}

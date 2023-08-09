use std::{net::{IpAddr, SocketAddr}, error::Error};
use warp::Filter;

pub async fn run(addr_opt: &Option<String>, port: u16) -> Result<(), Box<dyn Error>> {
    let addr = match addr_opt {
        Some(addr_raw) => addr_raw.parse()?,
        None => IpAddr::from([0, 0, 0, 0]),
    };
    let sock_addr = SocketAddr::new(addr, port);

    let hello = warp::path!("hello" / String)
        .and_then(handle_hello);

    let (_, server) = warp::serve(hello)
        .try_bind_ephemeral(sock_addr)?;
    server.await;

    Ok(())
}

async fn handle_hello(name: String) -> Result<Box<dyn warp::Reply>, warp::Rejection> {
    Ok(Box::new(format!("Hello, {}!", name)))
}

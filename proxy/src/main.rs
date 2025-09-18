use crate::config::PROXY_CONFIG;
use crate::error::Error;
use std::net::SocketAddr;
use tokio::net::TcpListener;
use tracing::{debug, error};

mod config;
mod error;
mod log;

fn main() {
    let _ = log::init().expect("failed to init logger");
    println!("starting proxy with config: {:#?}", PROXY_CONFIG);
    let runtime = tokio::runtime::Builder::new_multi_thread()
        .worker_threads(PROXY_CONFIG.worker_threads())
        .enable_all()
        .build()
        .expect("failed to build tokio runtime");
    
    runtime.block_on(async move {
        let listener = TcpListener::bind(SocketAddr::from(([0, 0, 0, 0], PROXY_CONFIG.port())))
            .await
            .expect("failed to bind proxy server to port");
        loop {
            if let Err(e) = handle_listener(&listener).await {
                error!("failed to handle listener: {}", e);
            }
        }
    })
}

async fn handle_listener(listener: &TcpListener) -> Result<(), Error> {
    let (agent_tcp_stream, agent_remote_address) = listener.accept().await?;
    debug!("accepted connection from agent: {:?}", agent_remote_address);
    tokio::spawn(async move {});
    Ok(())
}

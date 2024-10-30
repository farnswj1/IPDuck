use std::net::SocketAddr;

use axum::{
    extract::ConnectInfo,
    http::{header::USER_AGENT, HeaderMap},
    response::Redirect
};
use axum_client_ip::InsecureClientIp;
use tracing::info;

use crate::templates::IndexTemplate;

pub async fn root(
    headers: HeaderMap,
    InsecureClientIp(client_ip): InsecureClientIp,
    ConnectInfo(socket): ConnectInfo<SocketAddr>
) -> IndexTemplate {
    let remote_port = match headers.get("X-Forwarded-Port") {
        Some(port) => port.to_str().unwrap().to_string(),
        None => socket.port().to_string()
    };

    let browser = match headers.get(USER_AGENT) {
        Some(user_agent) => user_agent.to_str().unwrap().to_string(),
        None => "N/A".to_string()
    };

    info!("{client_ip}:{remote_port} - {browser}");
    IndexTemplate { client_ip, remote_port, browser }
}

pub async fn not_found() -> Redirect {
    Redirect::permanent("/")
}

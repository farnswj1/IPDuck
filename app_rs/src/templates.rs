use std::net::IpAddr;

use askama_axum::Template;

#[derive(Template)]
#[template(path = "index.html")]
pub struct IndexTemplate {
    pub client_ip: IpAddr,
    pub remote_port: String,
    pub browser: String
}

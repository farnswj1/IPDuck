use actix_web::{
    dev::PeerAddr,
    get,
    http::header::USER_AGENT,
    web::{Html, Path, Redirect},
    HttpRequest,
    Responder
};
use actix_web_rust_embed_responder::{Compress, IntoResponse};
use askama::Template;
use rust_embed_for_web::RustEmbed;

use crate::{staticfiles::StaticAssets, templates::IndexTemplate};

#[get("/")]
pub async fn index(request: HttpRequest, addr: PeerAddr) -> impl Responder {
    let headers = request.headers();
    let client_ip = request.connection_info().realip_remote_addr().unwrap().to_string();

    let remote_port = match headers.get("X-Forwarded-Port") {
        Some(port) => port.to_str().unwrap().to_string(),
        None => addr.0.port().to_string()
    };

    let browser = match headers.get(USER_AGENT) {
        Some(user_agent) => user_agent.to_str().unwrap().to_string(),
        None => "N/A".to_string()
    };

    let template = IndexTemplate{ client_ip, remote_port, browser };
    Html::new(template.render().expect("Render index.html"))
}

#[get("/static/{path:.*}")]
async fn serve_static(path: Path<String>) -> impl Responder {
    StaticAssets::get(&path).into_response().use_compression(Compress::IfWellKnown)
}

pub async fn redirect() -> impl Responder {
    Redirect::to("/").permanent()
}

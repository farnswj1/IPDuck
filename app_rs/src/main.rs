mod routes;
mod staticfiles;
mod templates;

use std::io::Result;
use actix_web::{middleware::{Logger, NormalizePath}, web::to, App, HttpServer};
use routes::{index, serve_static, redirect};
use tracing::Level;

#[actix_web::main]
async fn main() -> Result<()> {
    tracing_subscriber::fmt().with_max_level(Level::INFO).init();

    HttpServer::new(|| {
        App::new()
            .wrap(Logger::new("%{r}a \"%r\" %s %b \"%{Referer}i\" \"%{User-Agent}i\" %Ts"))
            .wrap(NormalizePath::trim())
            .service(index)
            .service(serve_static)
            .default_service(to(redirect))
    })
        .bind(("0.0.0.0", 8000))?
        .run()
        .await
}

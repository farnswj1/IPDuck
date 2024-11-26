mod config;
mod routes;
mod templates;

use std::io::Result;

use actix_cors::Cors;
use actix_files::Files;
use actix_web::{middleware::{Logger, NormalizePath}, web::to, App, HttpServer};
use config::Config;
use dotenvy::dotenv;
use routes::{index, redirect};
use tracing::Level;

#[actix_web::main]
async fn main() -> Result<()> {
    dotenv().ok();
    tracing_subscriber::fmt().with_max_level(Level::INFO).init();

    HttpServer::new(|| {
        let config = envy::from_env::<Config>().unwrap();
        let cors = Cors::default()
            .allowed_origin(&config.cors_allowed_origins)
            .allowed_methods(vec!["GET"]);

        App::new()
            .wrap(cors)
            .wrap(Logger::new("%{r}a \"%r\" %s %b \"%{Referer}i\" \"%{User-Agent}i\" %Ts"))
            .wrap(NormalizePath::trim())
            .service(index)
            .service(Files::new("/static", "./static"))
            .default_service(to(redirect))
    })
        .bind(("0.0.0.0", 8000))?
        .run()
        .await
}

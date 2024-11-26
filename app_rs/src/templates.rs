use askama_actix::Template;

#[derive(Template)]
#[template(path = "index.html")]
pub struct IndexTemplate {
    pub client_ip: String,
    pub remote_port: String,
    pub browser: String
}

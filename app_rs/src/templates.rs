use {askama::Template, askama_web::WebTemplate};

#[derive(Template, WebTemplate)]
#[template(path = "index.html")]
pub struct IndexTemplate {
    pub client_ip: String,
    pub remote_port: String,
    pub browser: String
}

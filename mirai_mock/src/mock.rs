use actix_web::{get, App, HttpServer, Responder, HttpRequest};
use mirai_test::data::session::About;

pub struct MockServer {
    host: String,
    port: u16,
}

#[get("/about")]
async fn about(_req: HttpRequest) -> impl Responder {
    serde_json::to_string(&About::response()).unwrap()
}

impl MockServer {
    pub fn new<S: AsRef<str>>(host: S, port: u16) -> MockServer {
        MockServer {
            host: String::from(host.as_ref()),
            port,
        }
    }

    pub async fn start(self) -> std::io::Result<()> {
        HttpServer::new(|| {
            App::new()
                .service(about)
        })
            .bind(format!("{}:{}", self.host, self.port))?
            .run()
            .await
    }
}
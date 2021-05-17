use actix::Actor;
use actix_web::{App, HttpServer};

mod app;
use app::{lobby::Lobby};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let chat_server = Lobby::default().start();

    HttpServer::new(move || {
        App::new()
            .service(app::config) 
            .data(chat_server.clone())
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
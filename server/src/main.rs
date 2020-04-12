use actix::{Actor, Addr};
use actix_files as fs;
use actix_web::{middleware, web, App, Error, HttpRequest, HttpResponse, HttpServer};
use actix_web_actors::ws;

mod engine;
mod game;

async fn ws_index(
    r: HttpRequest,
    stream: web::Payload,
    data: web::Data<Addr<engine::Server>>,
) -> Result<HttpResponse, Error> {
    ws::start(engine::WebSocket::new(data.get_ref().clone()), &r, stream)
}

fn env_or_default(env_name: &str, default: &str) -> String {
    if let Ok(env_value) = std::env::var(env_name) {
        env_value
    } else {
        default.to_string()
    }
}

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    std::env::set_var("RUST_LOG", "debug,actix_server=info,actix_web=info");
    let addr = env_or_default("LISTEN_ADDR", "127.0.0.1");
    let port = env_or_default("PORT", "8080");
    env_logger::init();

    let server = engine::Server::default();
    let server_addr = server.start();

    HttpServer::new(move || {
        App::new()
            // enable logger
            .wrap(middleware::Logger::default())
            // provide server address
            .data(server_addr.clone())
            // websocket route
            .service(web::resource("/ws").route(web::get().to(ws_index)))
            // static files
            .service(fs::Files::new("/", "static/").index_file("index.html"))
    })
    .bind(format!("{}:{}", addr, port))?
    .run()
    .await
}

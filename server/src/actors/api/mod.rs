use actix::Addr;

use super::server::Server;

mod inbound_message;
pub mod message;
mod websocket;

use websocket::WebSocket;
pub use websocket::WebSocket as ApiClient;

pub fn create_ws_api(server: Addr<Server>) -> ApiClient {
    WebSocket::new(server)
}

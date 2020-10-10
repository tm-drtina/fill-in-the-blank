use actix::Addr;

use super::server::Server;

mod inbound_message;
mod websocket;

pub mod outbound_message;
pub use crate::api::websocket::WebSocket as ApiClient;

pub fn create_ws_api(server: Addr<Server>) -> ApiClient {
    return websocket::WebSocket::new(server);
}

use actix::{AsyncContext, Handler, Message};
use log::debug;

use super::super::super::server::message as server_msg;
use super::super::WebSocket;

#[derive(Message)]
#[rtype(result = "()")]
pub struct PlayerConnect {
    pub username: String,
}

impl Handler<PlayerConnect> for WebSocket {
    type Result = ();

    fn handle(&mut self, msg: PlayerConnect, ctx: &mut Self::Context) -> Self::Result {
        debug!(
            "Handling PlayerConnect message with username: {:?}",
            msg.username
        );
        self.server.do_send(server_msg::PlayerConnect {
            username: msg.username,
            api_client: ctx.address(),
        });
    }
}

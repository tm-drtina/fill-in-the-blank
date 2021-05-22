use actix::{AsyncContext, Handler, Message};
use log::debug;
use uuid::Uuid;

use super::super::super::server::message as server_msg;
use super::super::WebSocket;

#[derive(Message)]
#[rtype(result = "()")]
pub struct PlayerReconnect {
    pub session_id: Uuid,
}

impl Handler<PlayerReconnect> for WebSocket {
    type Result = ();

    fn handle(&mut self, msg: PlayerReconnect, ctx: &mut Self::Context) -> Self::Result {
        debug!(
            "Handling PlayerReconnect message with session id: {:?}",
            msg.session_id
        );
        self.server.do_send(server_msg::PlayerReconnect {
            api_client: ctx.address(),
            session_id: msg.session_id,
        })
    }
}

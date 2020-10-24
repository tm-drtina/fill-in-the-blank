use actix::{AsyncContext, Handler, Message};
use log::debug;
use uuid::Uuid;

use super::super::super::server::message as server_msg;
use super::super::WebSocket;

#[derive(Message)]
#[rtype(result = "()")]
pub struct Reconnect {
    pub session_id: Uuid,
}

impl Handler<Reconnect> for WebSocket {
    type Result = ();

    fn handle(&mut self, msg: Reconnect, ctx: &mut Self::Context) -> Self::Result {
        debug!(
            "Handling Reconnect message with session id: {:?}",
            msg.session_id
        );
        self.server.do_send(server_msg::Reconnect {
            api_client: ctx.address(),
            session_id: msg.session_id,
        })
    }
}

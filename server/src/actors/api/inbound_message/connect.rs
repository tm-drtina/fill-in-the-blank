use actix::{AsyncContext, Handler, Message};
use log::debug;

use super::super::super::server::message as server_msg;
use super::super::WebSocket;

#[derive(Message)]
#[rtype(result = "()")]
pub struct Connect {
    pub username: String,
}

impl Handler<Connect> for WebSocket {
    type Result = ();

    fn handle(&mut self, msg: Connect, ctx: &mut Self::Context) -> Self::Result {
        debug!("Handling Connect message with username: {:?}", msg.username);
        self.server.do_send(server_msg::Connect {
            username: msg.username,
            api_client: ctx.address(),
        });
    }
}

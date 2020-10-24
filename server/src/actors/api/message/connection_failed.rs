use actix::{Handler, Message};
use serde::Serialize;

use super::super::WebSocket;

#[derive(Message, Serialize)]
#[serde(tag = "type")]
#[rtype(result = "()")]
pub struct ConnectionFailed {
    pub reason: String,
}

impl Handler<ConnectionFailed> for WebSocket {
    type Result = ();

    fn handle(&mut self, msg: ConnectionFailed, ctx: &mut Self::Context) -> Self::Result {
        ctx.text(serde_json::to_string(&msg).unwrap());
    }
}

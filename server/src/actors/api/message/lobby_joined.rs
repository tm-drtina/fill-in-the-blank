use actix::{Handler, Message};
use serde::Serialize;
use uuid::Uuid;

use super::super::WebSocket;

#[derive(Message, Serialize)]
#[serde(tag = "type")]
#[rtype(result = "()")]
pub struct LobbyJoined {
    pub lobby_id: Uuid,
}

impl Handler<LobbyJoined> for WebSocket {
    type Result = ();

    fn handle(&mut self, msg: LobbyJoined, ctx: &mut Self::Context) -> Self::Result {
        ctx.text(serde_json::to_string(&msg).unwrap());
    }
}

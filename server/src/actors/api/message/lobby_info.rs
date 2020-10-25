use actix::{Handler, Message};
use serde::Serialize;
use uuid::Uuid;

use super::super::WebSocket;

#[derive(Message, Serialize)]
#[serde(tag = "type")]
#[rtype(result = "()")]
pub struct LobbyInfo {
    pub lobby_id: Uuid,
    pub name: String,
    pub players: Vec<String>,
}

impl Handler<LobbyInfo> for WebSocket {
    type Result = ();

    fn handle(&mut self, msg: LobbyInfo, ctx: &mut Self::Context) -> Self::Result {
        ctx.text(serde_json::to_string(&msg).unwrap());
    }
}

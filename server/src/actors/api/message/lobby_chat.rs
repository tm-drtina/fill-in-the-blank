use actix::{Handler, Message};
use chrono::{DateTime, Utc};
use serde::Serialize;

use super::super::WebSocket;

#[derive(Message, Serialize)]
#[serde(tag = "type")]
#[rtype(result = "()")]
pub struct LobbyChat {
    pub timestamp: DateTime<Utc>,
    pub system_msg: bool,
    pub username: String,
    pub message: String,
}

impl Handler<LobbyChat> for WebSocket {
    type Result = ();

    fn handle(&mut self, msg: LobbyChat, ctx: &mut Self::Context) -> Self::Result {
        ctx.text(serde_json::to_string(&msg).unwrap());
    }
}

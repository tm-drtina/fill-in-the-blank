use actix::{Handler, Message};
use serde::Serialize;

use super::super::super::messages::LobbyOverview;
use super::super::WebSocket;

#[derive(Message, Serialize)]
#[serde(tag = "type")]
#[rtype(result = "()")]
pub struct LobbyList {
    pub lobbies: Vec<LobbyOverview>,
}

impl Handler<LobbyList> for WebSocket {
    type Result = ();

    fn handle(&mut self, msg: LobbyList, ctx: &mut Self::Context) -> Self::Result {
        ctx.text(serde_json::to_string(&msg).unwrap());
    }
}

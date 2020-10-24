use actix::{Addr, Handler, Message};
use serde::Serialize;
use uuid::Uuid;

use super::super::super::player::Player;
use super::super::WebSocket;

#[derive(Message, Serialize)]
#[rtype(result = "()")]
#[serde(tag = "type")]
pub struct UserConnected {
    pub session_id: Uuid,
    pub username: String,
    #[serde(skip)]
    pub player: Addr<Player>,
}

impl Handler<UserConnected> for WebSocket {
    type Result = ();

    fn handle(&mut self, msg: UserConnected, ctx: &mut Self::Context) -> Self::Result {
        self.player = Some(msg.player.clone());
        ctx.text(serde_json::to_string(&msg).unwrap());
    }
}

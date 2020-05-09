use actix::{Addr, Handler, Message};
use chrono::{DateTime, Utc};
use serde::Serialize;
use uuid::Uuid;

use super::super::player::Player;
use super::websocket::WebSocket;

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

#[derive(Message, Serialize)]
#[serde(tag = "type")]
#[rtype(result = "()")]
pub struct GlobalChat {
    pub timestamp: DateTime<Utc>,
    pub system_msg: bool,
    pub username: String,
    pub message: String,
}

impl Handler<GlobalChat> for WebSocket {
    type Result = ();

    fn handle(&mut self, msg: GlobalChat, ctx: &mut Self::Context) -> Self::Result {
        ctx.text(serde_json::to_string(&msg).unwrap());
    }
}

#[derive(Message, Serialize)]
#[serde(tag = "type")]
#[rtype(result = "()")]
pub struct Error {
    pub timestamp: DateTime<Utc>,
    pub text: String,
}

impl Handler<Error> for WebSocket {
    type Result = ();

    fn handle(&mut self, msg: Error, ctx: &mut Self::Context) -> Self::Result {
        ctx.text(serde_json::to_string(&msg).unwrap());
    }
}

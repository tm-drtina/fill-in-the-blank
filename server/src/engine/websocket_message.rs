use actix::{Addr, Handler, Message};
use actix_web_actors::ws;
use chrono::{DateTime, Utc};
use serde::Serialize;
use uuid::Uuid;

use crate::engine::WebSocket;
use crate::game::Player;

#[derive(Message, Serialize)]
#[serde(tag = "type")]
#[rtype(result = "()")]
pub struct ConnectionFailed {
    pub reason: String,
}

impl Handler<ConnectionFailed> for WebSocket {
    type Result = ();

    fn handle(
        &mut self,
        msg: ConnectionFailed,
        ctx: &mut ws::WebsocketContext<Self>,
    ) -> Self::Result {
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
    pub player_addr: Addr<Player>,
}

impl Handler<UserConnected> for WebSocket {
    type Result = ();

    fn handle(&mut self, msg: UserConnected, ctx: &mut ws::WebsocketContext<Self>) -> Self::Result {
        self.player_addr = Some(msg.player_addr.clone());
        ctx.text(serde_json::to_string(&msg).unwrap());
    }
}

#[derive(Message, Serialize)]
#[serde(tag = "type")]
#[rtype(result = "()")]
pub struct GlobalChat {
    pub timestamp: DateTime<Utc>,
    pub username: String,
    pub message: String,
}

impl Handler<GlobalChat> for WebSocket {
    type Result = ();

    fn handle(&mut self, msg: GlobalChat, ctx: &mut ws::WebsocketContext<Self>) -> Self::Result {
        ctx.text(serde_json::to_string(&msg).unwrap());
    }
}

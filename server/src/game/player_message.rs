use actix::{Addr, AsyncContext, Context, Handler, Message};
use chrono::{DateTime, Utc};
use uuid::Uuid;

use crate::engine::{websocket_message, WebSocket};
use crate::game::Player;

#[derive(Message)]
#[rtype(result = "()")]
pub struct Connected {
    pub websocket_addr: Addr<WebSocket>,
    pub session_id: Uuid,
}

impl Handler<Connected> for Player {
    type Result = ();

    fn handle(&mut self, msg: Connected, ctx: &mut Context<Self>) -> Self::Result {
        if let None = self.websocket_addr {
            self.websocket_addr = Some(msg.websocket_addr.clone());
            msg.websocket_addr
                .do_send(websocket_message::UserConnected {
                    session_id: msg.session_id,
                    username: self.username.clone(),
                    player_addr: ctx.address(),
                });
        } else {
            msg.websocket_addr
                .do_send(websocket_message::ConnectionFailed {
                    reason: "User still connected.".to_string(),
                });
        }
    }
}

#[derive(Message)]
#[rtype(result = "()")]
pub struct Disconnected {}

impl Handler<Disconnected> for Player {
    type Result = ();

    fn handle(&mut self, _msg: Disconnected, _ctx: &mut Context<Self>) -> Self::Result {
        self.websocket_addr = None;
        // TODO: wait, then skip in game, later drop from game
    }
}

#[derive(Message)]
#[rtype(result = "()")]
pub struct GlobalChat {
    pub message: String,
}

impl Handler<GlobalChat> for Player {
    type Result = ();

    fn handle(&mut self, msg: GlobalChat, _ctx: &mut Context<Self>) -> Self::Result {
        self.server_addr.do_send(GlobalChatBroadcast {
            timestamp: Utc::now(),
            username: self.username.clone(),
            message: msg.message,
        })
    }
}

#[derive(Message, Clone)]
#[rtype(result = "()")]
pub struct GlobalChatBroadcast {
    timestamp: DateTime<Utc>,
    username: String,
    message: String,
}

impl Handler<GlobalChatBroadcast> for Player {
    type Result = ();

    fn handle(&mut self, msg: GlobalChatBroadcast, _ctx: &mut Context<Self>) -> Self::Result {
        if let Some(websocket_addr) = &self.websocket_addr {
            websocket_addr.do_send(websocket_message::GlobalChat {
                timestamp: msg.timestamp,
                username: msg.username,
                message: msg.message,
            })
        }
    }
}

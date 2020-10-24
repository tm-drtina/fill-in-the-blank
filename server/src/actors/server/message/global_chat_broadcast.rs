use actix::{Context, Handler, Message};

use super::super::super::player;
use super::super::Server;
use chrono::{DateTime, Utc};

#[derive(Message, Clone)]
#[rtype(result = "()")]
pub struct GlobalChatBroadcast {
    timestamp: DateTime<Utc>,
    system_msg: bool,
    username: String,
    message: String,
}

impl GlobalChatBroadcast {
    pub fn system_message(text: String) -> Self {
        Self {
            timestamp: Utc::now(),
            system_msg: true,
            username: "System".to_string(),
            message: text,
        }
    }
    pub fn user_message(username: String, text: String) -> Self {
        Self {
            timestamp: Utc::now(),
            system_msg: false,
            username,
            message: text,
        }
    }
}

impl Handler<GlobalChatBroadcast> for Server {
    type Result = ();

    fn handle(&mut self, msg: GlobalChatBroadcast, _ctx: &mut Context<Self>) -> Self::Result {
        for player in self.players.values() {
            player.addr.do_send(player::message::ReceiveGlobalChat {
                timestamp: msg.timestamp,
                system_msg: msg.system_msg,
                username: msg.username.clone(),
                message: msg.message.clone(),
            })
        }
    }
}

use actix::{Addr, AsyncContext, Context, Handler, Message};
use chrono::{DateTime, Utc};
use uuid::Uuid;

use super::super::api;
use super::super::api::ApiClient;
use super::super::server;
use super::Player;

#[derive(Message)]
#[rtype(result = "()")]
pub struct Connected {
    pub api_client: Addr<ApiClient>,
    pub session_id: Uuid,
}

impl Handler<Connected> for Player {
    type Result = ();

    fn handle(&mut self, msg: Connected, ctx: &mut Context<Self>) -> Self::Result {
        if let None = self.api_client {
            self.api_client = Some(msg.api_client.clone());
            msg.api_client
                .do_send(api::outbound_message::UserConnected {
                    session_id: msg.session_id,
                    username: self.username.clone(),
                    player_addr: ctx.address(),
                });
        } else {
            msg.api_client
                .do_send(api::outbound_message::ConnectionFailed {
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
        self.api_client = None;
        // TODO: wait, then skip in game, later drop from game
    }
}

#[derive(Message)]
#[rtype(result = "()")]
pub struct SendGlobalChat {
    pub message: String,
}

impl Handler<SendGlobalChat> for Player {
    type Result = ();

    fn handle(&mut self, msg: SendGlobalChat, _ctx: &mut Context<Self>) -> Self::Result {
        self.server_addr
            .do_send(server::message::GlobalChatBroadcast {
                timestamp: Utc::now(),
                system_msg: false,
                username: self.username.clone(),
                message: msg.message,
            })
    }
}

#[derive(Message)]
#[rtype(result = "()")]
pub struct ReceiveGlobalChat {
    pub timestamp: DateTime<Utc>,
    pub system_msg: bool,
    pub username: String,
    pub message: String,
}

impl Handler<ReceiveGlobalChat> for Player {
    type Result = ();

    fn handle(&mut self, msg: ReceiveGlobalChat, _ctx: &mut Context<Self>) -> Self::Result {
        if let Some(api_client) = &self.api_client {
            api_client.do_send(api::outbound_message::GlobalChat {
                timestamp: Utc::now(),
                system_msg: msg.system_msg,
                username: self.username.clone(),
                message: msg.message,
            })
        }
    }
}

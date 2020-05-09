use actix::{Addr, AsyncContext, Context, Handler, Message};
use chrono::{DateTime, Utc};
use std::time::Instant;

use super::super::api;
use super::super::api::ApiClient;
use super::super::server;
use super::{Player, PlayerStatus};

#[derive(Message)]
#[rtype(result = "()")]
pub struct Connected {
    pub api_client: Addr<ApiClient>,
}

impl Handler<Connected> for Player {
    type Result = ();

    fn handle(&mut self, msg: Connected, ctx: &mut Context<Self>) -> Self::Result {
        match self.status {
            PlayerStatus::Connecting => {
                self.status = PlayerStatus::Connected {
                    api_client: msg.api_client.clone(),
                };
                msg.api_client
                    .do_send(api::outbound_message::UserConnected {
                        session_id: self.session_id.clone(),
                        username: self.username.clone(),
                        player: ctx.address(),
                    });
                self.server
                    .do_send(server::message::GlobalChatBroadcast::system_message(
                        format!("User '{}' connected.", self.username),
                    ));
            }
            PlayerStatus::LostConnection { .. } => {
                self.status = PlayerStatus::Connected {
                    api_client: msg.api_client.clone(),
                };
                msg.api_client
                    .do_send(api::outbound_message::UserConnected {
                        session_id: self.session_id.clone(),
                        username: self.username.clone(),
                        player: ctx.address(),
                    });
                self.server
                    .do_send(server::message::GlobalChatBroadcast::system_message(
                        format!("User '{}' reconnected.", self.username),
                    ));
            }
            PlayerStatus::Connected { .. } => {
                msg.api_client
                    .do_send(api::outbound_message::ConnectionFailed {
                        reason: "User still connected.".to_string(),
                    })
            }
        }
    }
}

#[derive(Message)]
#[rtype(result = "()")]
pub struct Disconnected {
    pub reason: String,
}

impl Handler<Disconnected> for Player {
    type Result = ();

    fn handle(&mut self, msg: Disconnected, _ctx: &mut Context<Self>) -> Self::Result {
        self.status = PlayerStatus::LostConnection {
            last_seen: Instant::now(),
        };
        self.server
            .do_send(server::message::GlobalChatBroadcast::system_message(
                format!(
                    "User '{}' disconnected. Reason: {}",
                    self.username, msg.reason
                ),
            ));
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
        self.server
            .do_send(server::message::GlobalChatBroadcast::user_message(
                self.username.clone(),
                msg.message,
            ))
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
        if let PlayerStatus::Connected { api_client } = &self.status {
            api_client.do_send(api::outbound_message::GlobalChat {
                timestamp: msg.timestamp,
                system_msg: msg.system_msg,
                username: msg.username,
                message: msg.message,
            })
        }
        //TODO: else add message to buffer?
    }
}

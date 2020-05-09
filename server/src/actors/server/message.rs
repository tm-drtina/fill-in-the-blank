use actix::{Actor, Addr, AsyncContext, Context, Handler, Message};
use log::debug;
use uuid::Uuid;

use super::super::api::{outbound_message, ApiClient};
use super::super::player;
use super::super::player::Player;
use super::{PlayerInfo, Server};
use chrono::{DateTime, Utc};
use std::collections::hash_map::{Entry, OccupiedEntry};

#[derive(Message)]
#[rtype(result = "()")]
pub struct Connect {
    pub username: String,
    pub api_client: Addr<ApiClient>,
}

impl Handler<Connect> for Server {
    type Result = ();

    fn handle(&mut self, msg: Connect, ctx: &mut Context<Self>) -> Self::Result {
        let session_id = Uuid::new_v4();
        let player = Player::new(ctx.address(), session_id, msg.username.clone()).start();

        player.do_send(player::message::Connected {
            api_client: msg.api_client,
        });
        self.players.insert(
            session_id,
            PlayerInfo {
                addr: player,
                username: msg.username,
            },
        );
    }
}

#[derive(Message)]
#[rtype(result = "()")]
pub struct Reconnect {
    pub session_id: Uuid,
    pub api_client: Addr<ApiClient>,
}

impl Handler<Reconnect> for Server {
    type Result = ();

    fn handle(&mut self, msg: Reconnect, _ctx: &mut Context<Self>) -> Self::Result {
        let player_info = self.players.get(&msg.session_id);
        match player_info {
            Some(player) => player.addr.do_send(player::message::Connected {
                api_client: msg.api_client,
            }),
            None => msg.api_client.do_send(outbound_message::ConnectionFailed {
                reason: "Session not found".to_string(),
            }),
        }
    }
}

#[derive(Message)]
#[rtype(result = "()")]
pub struct DestroySession {
    pub session_id: Uuid,
}

impl Handler<DestroySession> for Server {
    type Result = ();

    fn handle(&mut self, msg: DestroySession, _ctx: &mut Context<Self>) -> Self::Result {
        if let Entry::Occupied(player_info_entry) = self.players.entry(msg.session_id) {
            let player_info = player_info_entry.remove();
            debug!(
                "Destroying session {}. Username: {}",
                msg.session_id, player_info.username,
            );
        } else {
            warn!(
                "Trying to destroy session, that doesn't exist. Session ID: {}",
                msg.session_id
            )
        }
    }
}

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

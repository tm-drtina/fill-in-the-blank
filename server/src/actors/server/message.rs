use actix::{Actor, Addr, AsyncContext, Context, Handler, Message};
use uuid::Uuid;

use super::super::api::{outbound_message, ApiClient};
use super::super::player;
use super::super::player::Player;
use super::super::server::Server;
use chrono::{DateTime, Utc};

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
        let player = Player::new(ctx.address(), msg.username);
        let player_addr = player.start();

        player_addr.do_send(player::message::Connected {
            api_client: msg.api_client,
            session_id,
        });
        self.players.insert(session_id, player_addr);
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
            Some(player_addr) => {
                player_addr.do_send(player::message::Connected {
                    api_client: msg.api_client,
                    session_id: msg.session_id,
                });
            }
            None => {
                msg.api_client.do_send(outbound_message::ConnectionFailed {
                    reason: "Session not found".to_string(),
                });
            }
        }
    }
}

#[derive(Message, Clone)]
#[rtype(result = "()")]
pub struct GlobalChatBroadcast {
    pub timestamp: DateTime<Utc>,
    pub system_msg: bool,
    pub username: String,
    pub message: String,
}

impl Handler<GlobalChatBroadcast> for Server {
    type Result = ();

    fn handle(&mut self, msg: GlobalChatBroadcast, _ctx: &mut Context<Self>) -> Self::Result {
        for player_addr in self.players.values() {
            player_addr.do_send(player::message::ReceiveGlobalChat {
                timestamp: msg.timestamp,
                system_msg: msg.system_msg,
                username: msg.username.clone(),
                message: msg.message.clone(),
            })
        }
    }
}

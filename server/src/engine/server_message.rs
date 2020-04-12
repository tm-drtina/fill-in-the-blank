use actix::{Actor, Addr, AsyncContext, Context, Handler, Message};
use uuid::Uuid;

use crate::engine::{websocket_message, Server, WebSocket};
use crate::game::{player_message, Player};

#[derive(Message)]
#[rtype(result = "()")]
pub struct Connect {
    pub username: String,
    pub websocket_addr: Addr<WebSocket>,
}

impl Handler<Connect> for Server {
    type Result = ();

    fn handle(&mut self, msg: Connect, ctx: &mut Context<Self>) -> Self::Result {
        let session_id = Uuid::new_v4();
        let player = Player::new(ctx.address(), msg.username);
        let player_addr = player.start();

        player_addr.do_send(player_message::Connected {
            websocket_addr: msg.websocket_addr,
            session_id,
        });
        self.players.insert(session_id, player_addr);
    }
}

#[derive(Message)]
#[rtype(result = "()")]
pub struct Reconnect {
    pub session_id: Uuid,
    pub websocket_addr: Addr<WebSocket>,
}

impl Handler<Reconnect> for Server {
    type Result = ();

    fn handle(&mut self, msg: Reconnect, _ctx: &mut Context<Self>) -> Self::Result {
        let player_info = self.players.get(&msg.session_id);
        match player_info {
            Some(player_addr) => {
                player_addr.do_send(player_message::Connected {
                    websocket_addr: msg.websocket_addr,
                    session_id: msg.session_id,
                });
            }
            None => {
                msg.websocket_addr
                    .do_send(websocket_message::ConnectionFailed {
                        reason: "Session not found".to_string(),
                    });
            }
        }
    }
}

impl Handler<player_message::GlobalChatBroadcast> for Server {
    type Result = ();

    fn handle(
        &mut self,
        msg: player_message::GlobalChatBroadcast,
        _ctx: &mut Context<Self>,
    ) -> Self::Result {
        for player_addr in self.players.values() {
            player_addr.do_send(msg.clone())
        }
    }
}

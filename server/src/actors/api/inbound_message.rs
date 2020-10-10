use actix::{AsyncContext, Handler, Message};
use log::{debug, error};
use uuid::Uuid;

use super::super::player;
use super::super::server;
use super::websocket::WebSocket;

#[derive(Message)]
#[rtype(result = "()")]
pub struct Connect {
    pub username: String,
}
impl Handler<Connect> for WebSocket {
    type Result = ();

    fn handle(&mut self, msg: Connect, ctx: &mut Self::Context) -> Self::Result {
        debug!("Handling Connect message with username: {:?}", msg.username);
        self.server.do_send(server::message::Connect {
            username: msg.username,
            api_client: ctx.address(),
        });
    }
}

#[derive(Message)]
#[rtype(result = "()")]
pub struct Reconnect {
    pub session_id: Uuid,
}
impl Handler<Reconnect> for WebSocket {
    type Result = ();

    fn handle(&mut self, msg: Reconnect, ctx: &mut Self::Context) -> Self::Result {
        debug!(
            "Handling Reconnect message with session id: {:?}",
            msg.session_id
        );
        self.server.do_send(server::message::Reconnect {
            api_client: ctx.address(),
            session_id: msg.session_id,
        })
    }
}

#[derive(Message)]
#[rtype(result = "()")]
pub struct GlobalChat {
    pub message: String,
}
impl Handler<GlobalChat> for WebSocket {
    type Result = ();

    fn handle(&mut self, msg: GlobalChat, _ctx: &mut Self::Context) -> Self::Result {
        debug!("Handling GlobalChat message: {}", msg.message);
        if let Some(player) = &self.player {
            player.do_send(player::message::SendGlobalChat {
                message: msg.message,
            });
        } else {
            error!("Got global chat message, but user is not logged in!")
            // TODO: Handle error, when user is not logged in
        }
    }
}

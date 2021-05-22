use actix::{AsyncContext, Handler, Message};
use log::{debug, error};
use uuid::Uuid;

use super::super::super::messages;
use super::super::super::player::message as player_msg;
use super::super::WebSocket;

#[derive(Message)]
#[rtype(result = "()")]
pub struct LobbyJoin {
    pub lobby_id: Uuid,
}

impl Handler<LobbyJoin> for WebSocket {
    type Result = ();

    fn handle(&mut self, msg: LobbyJoin, ctx: &mut Self::Context) -> Self::Result {
        debug!("Handling LobbyJoin message: {}", msg.lobby_id);
        if let Some(player) = &self.player {
            player.do_send(player_msg::LobbyJoin {
                lobby_id: msg.lobby_id,
            });
        } else {
            error!("Got LobbyJoin message, but user is not logged in!");
            ctx.address().do_send(messages::Error::new(
                messages::ErrorType::Disconnected,
                "Got LobbyJoin message, but user is not logged in!",
            ));
        }
    }
}

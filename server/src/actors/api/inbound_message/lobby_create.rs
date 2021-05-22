use actix::{AsyncContext, Handler, Message};
use log::{debug, error};

use super::super::super::messages;
use super::super::super::player::message as player_msg;
use super::super::WebSocket;

#[derive(Message)]
#[rtype(result = "()")]
pub struct LobbyCreate {
    pub name: String,
}

impl Handler<LobbyCreate> for WebSocket {
    type Result = ();

    fn handle(&mut self, msg: LobbyCreate, ctx: &mut Self::Context) -> Self::Result {
        debug!("Handling LobbyCreate message: {}", msg.name);
        if let Some(player) = &self.player {
            player.do_send(player_msg::LobbyCreate { name: msg.name });
        } else {
            error!("Got LobbyCreate message, but user is not logged in!");
            ctx.address().do_send(messages::Error::new(
                messages::ErrorType::Disconnected,
                "Got LobbyCreate message, but user is not logged in!",
            ));
        }
    }
}

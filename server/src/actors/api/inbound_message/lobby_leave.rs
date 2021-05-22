use actix::{AsyncContext, Handler, Message};
use log::{debug, error};

use super::super::super::messages;
use super::super::super::player::message as player_msg;
use super::super::WebSocket;

#[derive(Message)]
#[rtype(result = "()")]
pub struct LobbyLeave;

impl Handler<LobbyLeave> for WebSocket {
    type Result = ();

    fn handle(&mut self, _msg: LobbyLeave, ctx: &mut Self::Context) -> Self::Result {
        debug!("Handling LobbyLeave message");
        if let Some(player) = &self.player {
            player.do_send(player_msg::LobbyLeave);
        } else {
            error!("Got LobbyLeave message, but user is not logged in!");
            ctx.address().do_send(messages::Error::new(
                messages::ErrorType::Disconnected,
                "Got LobbyLeave message, but user is not logged in!",
            ));
        }
    }
}

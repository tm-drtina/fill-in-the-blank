use actix::{AsyncContext, Handler, Message};
use log::{debug, error};

use super::super::super::messages;
use super::super::super::player::message as player_msg;
use super::super::WebSocket;

#[derive(Message)]
#[rtype(result = "()")]
pub struct LobbyList;

impl Handler<LobbyList> for WebSocket {
    type Result = ();

    fn handle(&mut self, _msg: LobbyList, ctx: &mut Self::Context) -> Self::Result {
        debug!("Handling LobbyList message");
        if let Some(player) = &self.player {
            player.do_send(player_msg::LobbyListReq);
        } else {
            error!("Got LobbyList message, but user is not logged in!");
            ctx.address().do_send(messages::Error::new(
                messages::ErrorType::Disconnected,
                "Got LobbyList message, but user is not logged in!",
            ));
        }
    }
}

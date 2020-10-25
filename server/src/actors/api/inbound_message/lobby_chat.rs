use actix::{Handler, Message};
use log::{debug, error};

use super::super::super::player::message as player_msg;
use super::super::WebSocket;

#[derive(Message)]
#[rtype(result = "()")]
pub struct LobbyChat {
    pub message: String,
}

impl Handler<LobbyChat> for WebSocket {
    type Result = ();

    fn handle(&mut self, msg: LobbyChat, _ctx: &mut Self::Context) -> Self::Result {
        debug!("Handling LobbyChat message: {}", msg.message);
        if let Some(player) = &self.player {
            player.do_send(player_msg::SendLobbyChat {
                message: msg.message,
            });
        } else {
            error!("Got lobby chat message, but user is not logged in!")
            // TODO: Handle error, when user is not logged in
        }
    }
}

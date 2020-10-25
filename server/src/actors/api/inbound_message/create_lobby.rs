use actix::{Handler, Message};
use log::{debug, error};

use super::super::super::player::message as player_msg;
use super::super::WebSocket;

#[derive(Message)]
#[rtype(result = "()")]
pub struct CreateLobby {
    pub name: String,
}

impl Handler<CreateLobby> for WebSocket {
    type Result = ();

    fn handle(&mut self, msg: CreateLobby, _ctx: &mut Self::Context) -> Self::Result {
        debug!("Handling CreateLobby message: {}", msg.name);
        if let Some(player) = &self.player {
            player.do_send(player_msg::CreateLobby { name: msg.name });
        } else {
            error!("Got CreateLobby message, but user is not logged in!")
            // TODO: Handle error, when user is not logged in
        }
    }
}

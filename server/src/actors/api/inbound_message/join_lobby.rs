use actix::{Handler, Message};
use log::{debug, error};
use uuid::Uuid;

use super::super::super::player::message as player_msg;
use super::super::WebSocket;

#[derive(Message)]
#[rtype(result = "()")]
pub struct JoinLobby {
    pub lobby_id: Uuid,
}

impl Handler<JoinLobby> for WebSocket {
    type Result = ();

    fn handle(&mut self, msg: JoinLobby, _ctx: &mut Self::Context) -> Self::Result {
        debug!("Handling JoinLobby message: {}", msg.lobby_id);
        if let Some(player) = &self.player {
            player.do_send(player_msg::JoinLobby {
                lobby_id: msg.lobby_id,
            });
        } else {
            error!("Got JoinLobby message, but user is not logged in!")
            // TODO: Handle error, when user is not logged in
        }
    }
}

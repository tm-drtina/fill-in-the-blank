use actix::{Handler, Message};
use uuid::Uuid;

use super::super::super::server::message as server_msg;
use super::super::Player;

#[derive(Message)]
#[rtype(result = "()")]
pub struct JoinLobby {
    pub lobby_id: Uuid,
}

impl Handler<JoinLobby> for Player {
    type Result = ();

    fn handle(&mut self, msg: JoinLobby, _ctx: &mut Self::Context) -> Self::Result {
        self.server.do_send(server_msg::JoinLobby {
            lobby_id: msg.lobby_id,
            player_id: self.session_id,
        });
    }
}

use actix::{Handler, Message};

use super::super::super::server::message as server_msg;
use super::super::Player;

#[derive(Message)]
#[rtype(result = "()")]
pub struct CreateLobby {
    pub name: String,
}

impl Handler<CreateLobby> for Player {
    type Result = ();

    fn handle(&mut self, msg: CreateLobby, _ctx: &mut Self::Context) -> Self::Result {
        self.server.do_send(server_msg::CreateLobby {
            name: msg.name,
            player_id: self.session_id,
        });
    }
}

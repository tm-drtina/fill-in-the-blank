use actix::{Handler, Message};

use super::super::super::server::message as server_msg;
use super::super::Player;

#[derive(Message)]
#[rtype(result = "()")]
pub struct LobbyCreate {
    pub name: String,
}

impl Handler<LobbyCreate> for Player {
    type Result = ();

    fn handle(&mut self, msg: LobbyCreate, _ctx: &mut Self::Context) -> Self::Result {
        self.server.do_send(server_msg::LobbyCreate {
            name: msg.name,
            player_id: self.session_id,
        });
    }
}

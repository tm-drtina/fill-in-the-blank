use actix::{Handler, Message};

use super::super::super::server::message as server_msg;
use super::super::Player;

#[derive(Message)]
#[rtype(result = "()")]
pub struct LobbyLeave;

impl Handler<LobbyLeave> for Player {
    type Result = ();

    fn handle(&mut self, _msg: LobbyLeave, _ctx: &mut Self::Context) -> Self::Result {
        self.server.do_send(server_msg::LobbyLeave {
            player_id: self.session_id,
        });
    }
}

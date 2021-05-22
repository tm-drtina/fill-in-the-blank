use actix::{Handler, Message};

use super::super::super::server::message as server_msg;
use super::super::Player;

#[derive(Message)]
#[rtype(result = "()")]
pub struct LobbyListReq;

impl Handler<LobbyListReq> for Player {
    type Result = ();

    fn handle(&mut self, _msg: LobbyListReq, _ctx: &mut Self::Context) -> Self::Result {
        self.server.do_send(server_msg::LobbyList {
            player_id: self.session_id,
        });
    }
}

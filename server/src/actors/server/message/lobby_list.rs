use actix::{Handler, Message};
use log::error;
use uuid::Uuid;

use super::super::super::player::message as player_msg;
use super::super::Server;

#[derive(Message)]
#[rtype(result = "()")]
pub struct LobbyList {
    pub player_id: Uuid,
}

impl Handler<LobbyList> for Server {
    type Result = ();

    fn handle(&mut self, msg: LobbyList, _ctx: &mut Self::Context) -> Self::Result {
        let player_info = match self.players.get(&msg.player_id) {
            Some(player_info) => player_info,
            None => {
                error!("Player session '{}' not found", msg.player_id);
                return;
            }
        };

        player_info.addr.do_send(player_msg::LobbyListRes {
            lobbies: self.get_lobby_overviews(),
        })
    }
}

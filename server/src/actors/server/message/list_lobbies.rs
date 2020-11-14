use actix::{Handler, Message};
use log::error;
use uuid::Uuid;

use super::super::super::player::message as player_msg;
use super::super::Server;

#[derive(Message)]
#[rtype(result = "()")]
pub struct ListLobbies {
    pub player_id: Uuid,
}

impl Handler<ListLobbies> for Server {
    type Result = ();

    fn handle(&mut self, msg: ListLobbies, _ctx: &mut Self::Context) -> Self::Result {
        let player_info = match self.players.get(&msg.player_id) {
            Some(player_info) => player_info,
            None => {
                error!("Player session '{}' not found", msg.player_id);
                return;
            }
        };

        player_info.addr.do_send(player_msg::LobbyList {
            lobbies: self.get_lobby_overviews(),
        })
    }
}

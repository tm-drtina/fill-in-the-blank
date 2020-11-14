use actix::{AsyncContext, Handler, Message};
use log::error;
use uuid::Uuid;

use super::super::{message as server_msg, LobbyInfo, Server};

#[derive(Message)]
#[rtype(result = "()")]
pub struct CreateLobby {
    pub name: String,
    pub player_id: Uuid,
}

impl Handler<CreateLobby> for Server {
    type Result = ();

    fn handle(&mut self, msg: CreateLobby, ctx: &mut Self::Context) -> Self::Result {
        let player_info = match self.players.get_mut(&msg.player_id) {
            Some(player_info) => player_info,
            None => {
                error!("Player session '{}' not found", msg.player_id);
                return;
            }
        };
        if player_info.current_lobby.is_some() {
            error!("Player '{}' is already in a lobby", player_info.username);
            return;
        };

        let lobby_id = Uuid::new_v4();
        self.lobbies
            .insert(lobby_id, LobbyInfo::new(msg.name.clone()));
        ctx.address().do_send(server_msg::JoinLobby {
            lobby_id,
            player_id: msg.player_id,
        });
    }
}

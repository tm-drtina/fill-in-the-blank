use actix::{Handler, Message};
use log::error;
use uuid::Uuid;

use super::super::super::player::message as player_msg;
use super::super::Server;

#[derive(Message)]
#[rtype(result = "()")]
pub struct JoinLobby {
    pub lobby_id: Uuid,
    pub player_id: Uuid,
}

impl Handler<JoinLobby> for Server {
    type Result = ();

    fn handle(&mut self, msg: JoinLobby, _ctx: &mut Self::Context) -> Self::Result {
        let player_info = match self.players.get_mut(&msg.player_id) {
            Some(player_info) => player_info,
            None => {
                error!("Player session '{}' not found", msg.player_id);
                return;
            }
        };
        let lobby_info = match self.lobbies.get_mut(&msg.lobby_id) {
            Some(info) => info,
            None => {
                error!("Lobby '{}' not found", msg.lobby_id);
                return;
            }
        };
        if player_info.current_lobby.is_some() {
            error!("Player '{}' is already in a lobby", player_info.username);
            return;
        };

        player_info.current_lobby = Some(msg.lobby_id);
        lobby_info.players.push(msg.player_id);

        player_info.addr.do_send(player_msg::LobbyJoined {
            lobby_id: msg.lobby_id,
        });
        let lobby_message = format!("User '{}' joined the lobby.", player_info.username);

        self.broadcast_lobby_info(msg.lobby_id);
        self.broadcast_lobby_system_message(msg.lobby_id, lobby_message);
    }
}

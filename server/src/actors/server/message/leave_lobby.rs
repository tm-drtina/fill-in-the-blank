use actix::{Handler, Message};
use log::{error, info};
use uuid::Uuid;

use super::super::super::player::message as player_msg;
use super::super::Server;

#[derive(Message)]
#[rtype(result = "()")]
pub struct LeaveLobby {
    pub player_id: Uuid,
}

impl Handler<LeaveLobby> for Server {
    type Result = ();

    fn handle(&mut self, msg: LeaveLobby, _ctx: &mut Self::Context) -> Self::Result {
        let player_info = match self.players.get_mut(&msg.player_id) {
            Some(player_info) => player_info,
            None => {
                error!("Player session '{}' not found", msg.player_id);
                return;
            }
        };
        if player_info.current_lobby.is_none() {
            info!(
                "Player '{}' is not in a lobby. Skipping LeaveLobby message",
                player_info.username
            );
            return;
        };
        let lobby_id = player_info.current_lobby.unwrap();
        let lobby_info = self.lobbies.get_mut(&lobby_id).unwrap();

        player_info.current_lobby = None;
        if lobby_info.players.len() > 1 {
            lobby_info.players = lobby_info
                .players
                .iter()
                .filter(|player| **player != msg.player_id)
                .map(|player| *player)
                .collect();

            let lobby_message = format!("User '{}' left the lobby.", player_info.username);
            let lobby_info_msg = self.get_lobby_info_msg(lobby_id).unwrap();
            self.broadcast_message_to_lobby_players(lobby_id, lobby_info_msg);
            self.broadcast_lobby_system_message(lobby_id, lobby_message);
        } else {
            self.lobbies.remove(&lobby_id);
        }

        let lobby_list_msg = player_msg::LobbyList {
            lobbies: self.get_lobby_overviews(),
        };
        self.broadcast_message_to_non_lobby_players(lobby_list_msg);
    }
}

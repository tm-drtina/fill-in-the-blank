use actix::{AsyncContext, Handler, Message};
use log::{error, info};
use uuid::Uuid;

use super::super::super::messages;
use super::super::super::player::message as player_msg;
use super::super::{message as server_msg, Server};

#[derive(Message)]
#[rtype(result = "()")]
pub struct JoinLobby {
    pub lobby_id: Uuid,
    pub player_id: Uuid,
}

impl Handler<JoinLobby> for Server {
    type Result = ();

    fn handle(&mut self, msg: JoinLobby, ctx: &mut Self::Context) -> Self::Result {
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
                player_info.addr.do_send(messages::Error::new(
                    messages::ErrorType::LobbyLeft,
                    "Selected lobby was not found.",
                ));
                info!("Lobby '{}' not found", msg.lobby_id);
                return;
            }
        };
        if player_info.current_lobby.is_some() {
            if player_info.current_lobby.unwrap() == msg.lobby_id {
                info!("Player is trying to join lobby, he/she is currently in. Skipping...");
            } else {
                ctx.address().do_send(server_msg::LeaveLobby {
                    player_id: msg.player_id,
                });
                ctx.address().do_send(server_msg::JoinLobby {
                    player_id: msg.player_id,
                    lobby_id: msg.lobby_id,
                });
            }
            return;
        };

        player_info.current_lobby = Some(msg.lobby_id);
        lobby_info.players.push(msg.player_id);

        let lobby_message = format!("User '{}' joined the lobby.", player_info.username);
        let lobby_info_msg = self.get_lobby_info_msg(msg.lobby_id).unwrap();
        self.broadcast_message_to_lobby_players(msg.lobby_id, &lobby_info_msg);
        self.broadcast_lobby_system_message(msg.lobby_id, lobby_message);

        let lobbies = self.get_lobby_overviews();
        for player_info in self.players.values() {
            if player_info.current_lobby.is_none() {
                player_info.addr.do_send(player_msg::LobbyList {
                    lobbies: lobbies.clone(),
                });
            }
        }
    }
}

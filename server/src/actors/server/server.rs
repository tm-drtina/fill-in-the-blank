use actix::{Actor, Addr, Context};
use chrono::Utc;
use log::error;
use std::collections::HashMap;
use uuid::Uuid;

use super::super::player::{message as player_msg, Player};

pub(super) struct LobbyInfo {
    pub name: String,
    pub players: Vec<Uuid>,
}

pub(super) struct PlayerInfo {
    pub addr: Addr<Player>,
    pub username: String,
    pub current_lobby: Option<Uuid>,
}

#[derive(Default)]
pub struct Server {
    pub(super) players: HashMap<Uuid, PlayerInfo>,
    pub(super) lobbies: HashMap<Uuid, LobbyInfo>,
}

impl Server {
    pub(super) fn broadcast_lobby_info(&self, lobby_id: Uuid) {
        let lobby_info = match self.lobbies.get(&lobby_id) {
            Some(info) => info,
            None => {
                error!("Lobby '{}' not found", lobby_id);
                return;
            }
        };
        let mut players: Vec<&PlayerInfo> = Vec::new();

        for player_id in &lobby_info.players {
            match self.players.get(player_id) {
                Some(player_info) => {
                    players.push(&player_info);
                }
                None => {
                    error!("Player session '{}' not found", player_id);
                }
            };
        }

        let lobby_info_msg = player_msg::LobbyInfo {
            lobby_id,
            name: lobby_info.name.clone(),
            players: (&players)
                .into_iter()
                .map(|player| player.username.clone())
                .collect(),
        };

        for player in players {
            player.addr.do_send(lobby_info_msg.clone());
        }
    }

    fn broadcast_lobby_message(&self, lobby_id: Uuid, player_msg: player_msg::ReceiveLobbyChat) {
        let lobby_info = match self.lobbies.get(&lobby_id) {
            Some(info) => info,
            None => {
                error!("Lobby '{}' not found", lobby_id);
                return;
            }
        };

        for player_id in &lobby_info.players {
            match self.players.get(player_id) {
                Some(player_info) => {
                    player_info.addr.do_send(player_msg.clone());
                }
                None => {
                    error!("Player session '{}' not found", player_id);
                }
            };
        }
    }

    pub(super) fn broadcast_lobby_system_message(&self, lobby_id: Uuid, message: String) {
        self.broadcast_lobby_message(
            lobby_id,
            player_msg::ReceiveLobbyChat {
                timestamp: Utc::now(),
                system_msg: true,
                username: "system".to_string(),
                message: message.clone(),
            },
        );
    }

    pub(super) fn broadcast_lobby_user_message(
        &self,
        player_id: Uuid,
        message: String,
    ) {
        let player_info = match self.players.get(&player_id) {
            Some(player_info) => player_info,
            None => {
                error!("Player session '{}' not found", player_id);
                return
            }
        };
        if player_info.current_lobby.is_none() {
            error!("Player '{}' is not in lobby. Cannot send lobby chat message", player_id);
            return
        }
        self.broadcast_lobby_message(
            player_info.current_lobby.unwrap(),
            player_msg::ReceiveLobbyChat {
                timestamp: Utc::now(),
                system_msg: false,
                username: player_info.username.clone(),
                message: message.clone(),
            },
        );
    }
}

impl Actor for Server {
    type Context = Context<Self>;
}

impl PlayerInfo {
    pub fn new(addr: Addr<Player>, username: String) -> Self {
        Self {
            addr,
            username,
            current_lobby: None,
        }
    }
}

impl LobbyInfo {
    pub fn new(name: String) -> Self {
        Self {
            name,
            players: Vec::new(),
        }
    }
}

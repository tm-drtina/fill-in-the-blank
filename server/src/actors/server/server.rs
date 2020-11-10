use actix::{Actor, Addr, Context, Handler, Message};
use chrono::Utc;
use log::error;
use std::collections::HashMap;
use uuid::Uuid;

use super::super::player::{message as player_msg, Player};
use crate::actors::messages::LobbyOverview;

#[derive(Debug)]
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
    pub(super) fn get_lobby_info_msg(&self, lobby_id: Uuid) -> Option<player_msg::LobbyInfo> {
        let lobby_info = match self.lobbies.get(&lobby_id) {
            Some(info) => info,
            None => {
                return None;
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

        Some(player_msg::LobbyInfo {
            lobby_id,
            name: lobby_info.name.clone(),
            players: players
                .into_iter()
                .map(|player| player.username.clone())
                .collect(),
        })
    }

    pub(super) fn get_lobby_overviews(&self) -> Vec<LobbyOverview> {
        self.lobbies
            .iter()
            .map(|(lobby_id, lobby_info)| LobbyOverview {
                lobby_id: *lobby_id,
                name: lobby_info.name.clone(),
                player_count: lobby_info.players.len() as u8,
            })
            .collect()
    }

    pub(super) fn broadcast_message_to_lobby_players<M>(&self, lobby_id: Uuid, msg: &M)
    where
        M: Message + Send + Clone + 'static,
        M::Result: Send,
        Player: Handler<M>,
    {
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
                    player_info.addr.do_send(msg.clone());
                }
                None => {
                    error!("Player session '{}' not found", player_id);
                }
            };
        }
    }

    pub(super) fn broadcast_message_to_non_lobby_players<M>(&self, msg: &M)
    where
        M: Message + Send + Clone + 'static,
        M::Result: Send,
        Player: Handler<M>,
    {
        for player_info in self.players.values() {
            if player_info.current_lobby.is_none() {
                player_info.addr.do_send(msg.clone());
            }
        }
    }

    pub(super) fn broadcast_lobby_system_message(&self, lobby_id: Uuid, message: String) {
        self.broadcast_message_to_lobby_players(
            lobby_id,
            &player_msg::ReceiveLobbyChat {
                timestamp: Utc::now(),
                system_msg: true,
                username: "system".to_string(),
                message,
            },
        );
    }

    pub(super) fn broadcast_lobby_user_message(&self, player_id: Uuid, message: String) {
        let player_info = match self.players.get(&player_id) {
            Some(player_info) => player_info,
            None => {
                error!("Player session '{}' not found", player_id);
                return;
            }
        };
        if player_info.current_lobby.is_none() {
            error!(
                "Player '{}' is not in lobby. Cannot send lobby chat message",
                player_id
            );
            return;
        }
        self.broadcast_message_to_lobby_players(
            player_info.current_lobby.unwrap(),
            &player_msg::ReceiveLobbyChat {
                timestamp: Utc::now(),
                system_msg: false,
                username: player_info.username.clone(),
                message,
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

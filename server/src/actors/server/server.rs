use actix::{Actor, Addr, Context};
use std::collections::HashMap;
use uuid::Uuid;

use super::super::player::Player;

// pub struct GameInfo {
//     addr: Addr<Game>,
// }

pub(super) struct PlayerInfo {
    pub addr: Addr<Player>,
    pub username: String,
}

#[derive(Default)]
pub struct Server {
    pub(super) players: HashMap<Uuid, PlayerInfo>,
    // pub games: HashMap<String, GameInfo>,
}

impl Actor for Server {
    type Context = Context<Self>;
}

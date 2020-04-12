use actix::{Actor, Addr, Context};
use std::collections::HashMap;
use uuid::Uuid;

use crate::game::Player;

// pub struct GameInfo {
//     addr: Addr<Game>,
// }

#[derive(Default)]
pub struct Server {
    pub players: HashMap<Uuid, Addr<Player>>,
    // pub games: HashMap<String, GameInfo>,
}

impl Actor for Server {
    type Context = Context<Self>;
}

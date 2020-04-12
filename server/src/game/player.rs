use actix::{Actor, Addr, Context};
use log::info;

use crate::engine::{Server, WebSocket};

pub struct Player {
    pub server_addr: Addr<Server>,
    pub websocket_addr: Option<Addr<WebSocket>>,
    pub username: String,
}

impl Actor for Player {
    type Context = Context<Self>;

    fn started(&mut self, _ctx: &mut Context<Self>) {
        info!("Player {} joined.", self.username);
    }

    fn stopped(&mut self, _ctx: &mut Context<Self>) {
        info!("Player {} left.", self.username);
    }
}

impl Player {
    pub fn new(server_addr: Addr<Server>, username: String) -> Self {
        Player {
            server_addr,
            websocket_addr: None,
            username,
        }
    }
}

use actix::{Actor, Addr, Context};
use log::info;

use super::super::api::ApiClient;
use super::super::server::Server;

pub struct Player {
    pub server: Addr<Server>,
    pub api_client: Option<Addr<ApiClient>>,
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
    pub fn new(server: Addr<Server>, username: String) -> Self {
        Player {
            server,
            api_client: None,
            username,
        }
    }
}

use actix::{Handler, Message};
use serde::Serialize;

use super::super::super::api::message as api_msg;
use super::super::super::messages::LobbyOverview;
use super::super::{Player, PlayerStatus};

#[derive(Message, Serialize, Clone)]
#[serde(tag = "type")]
#[rtype(result = "()")]
pub struct LobbyListRes {
    pub lobbies: Vec<LobbyOverview>,
}

impl Handler<LobbyListRes> for Player {
    type Result = ();

    fn handle(&mut self, msg: LobbyListRes, _ctx: &mut Self::Context) -> Self::Result {
        if let PlayerStatus::Connected { api_client } = &self.status {
            api_client.do_send(api_msg::LobbyList {
                lobbies: msg.lobbies,
            });
        }
    }
}

use actix::{Handler, Message};
use uuid::Uuid;

use super::super::super::api::message as api_msg;
use super::super::{Player, PlayerStatus};

#[derive(Message, Clone)]
#[rtype(result = "()")]
pub struct LobbyInfo {
    pub lobby_id: Uuid,
    pub name: String,
    pub players: Vec<String>,
}

impl Handler<LobbyInfo> for Player {
    type Result = ();

    fn handle(&mut self, msg: LobbyInfo, _ctx: &mut Self::Context) -> Self::Result {
        if let PlayerStatus::Connected { api_client } = &self.status {
            api_client.do_send(api_msg::LobbyInfo {
                lobby_id: msg.lobby_id,
                name: msg.name,
                players: msg.players,
            });
        }
    }
}

use actix::{Handler, Message};
use uuid::Uuid;

use super::super::super::api::message as api_msg;
use super::super::{Player, PlayerStatus};

#[derive(Message)]
#[rtype(result = "()")]
pub struct LobbyJoined {
    pub lobby_id: Uuid,
}

impl Handler<LobbyJoined> for Player {
    type Result = ();

    fn handle(&mut self, msg: LobbyJoined, _ctx: &mut Self::Context) -> Self::Result {
        if let PlayerStatus::Connected { api_client } = &self.status {
            api_client.do_send(api_msg::LobbyJoined {
                lobby_id: msg.lobby_id,
            });
        }
    }
}

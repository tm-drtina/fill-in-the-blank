use actix::{Addr, Context, Handler, Message};
use uuid::Uuid;

use super::super::super::api::ApiClient;
use super::super::super::messages;
use super::super::super::player::message as player_msg;
use super::super::Server;

#[derive(Message)]
#[rtype(result = "()")]
pub struct Reconnect {
    pub session_id: Uuid,
    pub api_client: Addr<ApiClient>,
}

impl Handler<Reconnect> for Server {
    type Result = ();

    fn handle(&mut self, msg: Reconnect, _ctx: &mut Context<Self>) -> Self::Result {
        let player_info = self.players.get(&msg.session_id);
        match player_info {
            Some(player) => player.addr.do_send(player_msg::Connected {
                api_client: msg.api_client,
            }),
            None => msg.api_client.do_send(messages::Error::new(
                messages::ErrorType::SessionNotFound,
                "Previous session not found",
            )),
        }
    }
}

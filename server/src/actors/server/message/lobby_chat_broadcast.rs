use actix::{Context, Handler, Message};
use uuid::Uuid;

use super::super::Server;

#[derive(Message, Clone)]
#[rtype(result = "()")]
pub struct LobbyChatBroadcast {
    pub player_id: Uuid,
    pub message: String,
}

impl Handler<LobbyChatBroadcast> for Server {
    type Result = ();

    fn handle(&mut self, msg: LobbyChatBroadcast, _ctx: &mut Context<Self>) -> Self::Result {
        self.broadcast_lobby_user_message(msg.player_id, msg.message);
    }
}

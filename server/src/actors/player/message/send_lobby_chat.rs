use actix::{Context, Handler, Message};

use super::super::super::server::message as server_msg;
use super::super::Player;

#[derive(Message)]
#[rtype(result = "()")]
pub struct SendLobbyChat {
    pub message: String,
}

impl Handler<SendLobbyChat> for Player {
    type Result = ();

    fn handle(&mut self, msg: SendLobbyChat, _ctx: &mut Context<Self>) -> Self::Result {
        self.server.do_send(server_msg::LobbyChatBroadcast {
            player_id: self.session_id,
            message: msg.message,
        });
    }
}

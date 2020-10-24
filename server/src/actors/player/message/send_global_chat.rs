use actix::{Context, Handler, Message};

use super::super::super::server::message as server_msg;
use super::super::Player;

#[derive(Message)]
#[rtype(result = "()")]
pub struct SendGlobalChat {
    pub message: String,
}

impl Handler<SendGlobalChat> for Player {
    type Result = ();

    fn handle(&mut self, msg: SendGlobalChat, _ctx: &mut Context<Self>) -> Self::Result {
        self.server
            .do_send(server_msg::GlobalChatBroadcast::user_message(
                self.username.clone(),
                msg.message,
            ));
    }
}

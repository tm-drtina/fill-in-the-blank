use actix::{Handler, Message};
use log::{debug, error};

use super::super::super::player;
use super::super::WebSocket;

#[derive(Message)]
#[rtype(result = "()")]
pub struct GlobalChat {
    pub message: String,
}

impl Handler<GlobalChat> for WebSocket {
    type Result = ();

    fn handle(&mut self, msg: GlobalChat, _ctx: &mut Self::Context) -> Self::Result {
        debug!("Handling GlobalChat message: {}", msg.message);
        if let Some(player) = &self.player {
            player.do_send(player::message::SendGlobalChat {
                message: msg.message,
            });
        } else {
            error!("Got global chat message, but user is not logged in!")
            // TODO: Handle error, when user is not logged in
        }
    }
}

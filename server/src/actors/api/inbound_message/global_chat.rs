use actix::{AsyncContext, Handler, Message};
use log::{debug, error};

use super::super::super::messages;
use super::super::super::player::message as player_msg;
use super::super::WebSocket;

#[derive(Message)]
#[rtype(result = "()")]
pub struct GlobalChat {
    pub message: String,
}

impl Handler<GlobalChat> for WebSocket {
    type Result = ();

    fn handle(&mut self, msg: GlobalChat, ctx: &mut Self::Context) -> Self::Result {
        debug!("Handling GlobalChat message: {}", msg.message);
        if let Some(player) = &self.player {
            player.do_send(player_msg::GlobalChatSend {
                message: msg.message,
            });
        } else {
            error!("Got GlobalChat message, but user is not logged in!");
            ctx.address().do_send(messages::Error::new(
                messages::ErrorType::Disconnected,
                "Got GlobalChat message, but user is not logged in!",
            ));
        }
    }
}

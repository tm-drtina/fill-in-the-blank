use actix::{ActorContext, Handler, Message};

use super::super::super::server::message as server_msg;
use super::super::Player;

#[derive(Message)]
#[rtype(result = "()")]
pub struct Logout;

impl Handler<Logout> for Player {
    type Result = ();

    fn handle(&mut self, _msg: Logout, ctx: &mut Self::Context) -> Self::Result {
        self.server.do_send(server_msg::DestroySession {
            session_id: self.session_id,
        });
        self.server
            .do_send(server_msg::GlobalChatBroadcast::system_message(format!(
                "User '{}' disconnected. Reason: Logged out.",
                self.username
            )));
        ctx.stop();
    }
}

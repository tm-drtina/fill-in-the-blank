use actix::{Handler, Message};
use log::{debug, warn};

use super::super::super::player::message as player_msg;
use super::super::WebSocket;

#[derive(Message)]
#[rtype(result = "()")]
pub struct Logout;

impl Handler<Logout> for WebSocket {
    type Result = ();

    fn handle(&mut self, _msg: Logout, _ctx: &mut Self::Context) -> Self::Result {
        debug!("Handling Logout message");
        if let Some(player) = &self.player {
            player.do_send(player_msg::Logout {});
            self.player = None;
        } else {
            warn!("Got Logout message, but user is not logged in. Skipping...");
        }
    }
}

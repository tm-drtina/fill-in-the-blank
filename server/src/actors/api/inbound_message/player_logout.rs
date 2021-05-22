use actix::{Handler, Message};
use log::{debug, warn};

use super::super::super::player::message as player_msg;
use super::super::WebSocket;

#[derive(Message)]
#[rtype(result = "()")]
pub struct PlayerLogout;

impl Handler<PlayerLogout> for WebSocket {
    type Result = ();

    fn handle(&mut self, _msg: PlayerLogout, _ctx: &mut Self::Context) -> Self::Result {
        debug!("Handling PlayerLogout message");
        if let Some(player) = &self.player {
            player.do_send(player_msg::PlayerLogout);
            self.player = None;
        } else {
            warn!("Got PlayerLogout message, but user is not logged in. Skipping...");
        }
    }
}

use actix::{Context, Handler, Message};
use std::time::Instant;

use super::super::super::server::message as server_msg;
use super::super::{Player, PlayerStatus};

#[derive(Message)]
#[rtype(result = "()")]
pub struct PlayerDisconnected {
    pub reason: String,
}

impl Handler<PlayerDisconnected> for Player {
    type Result = ();

    fn handle(&mut self, msg: PlayerDisconnected, _ctx: &mut Context<Self>) -> Self::Result {
        self.status = PlayerStatus::LostConnection {
            last_seen: Instant::now(),
        };
        self.server
            .do_send(server_msg::GlobalChatBroadcast::system_message(format!(
                "Player '{}' disconnected. Reason: {}",
                self.username, msg.reason
            )));
        self.server.do_send(server_msg::LobbyLeave {
            player_id: self.session_id,
        });
        // TODO: wait, then skip in game, later drop from game
    }
}

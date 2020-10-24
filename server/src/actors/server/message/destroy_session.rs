use actix::{Context, Handler, Message};
use log::{debug, warn};
use uuid::Uuid;

use super::super::Server;
use std::collections::hash_map::Entry;

#[derive(Message)]
#[rtype(result = "()")]
pub struct DestroySession {
    pub session_id: Uuid,
}

impl Handler<DestroySession> for Server {
    type Result = ();

    fn handle(&mut self, msg: DestroySession, _ctx: &mut Context<Self>) -> Self::Result {
        if let Entry::Occupied(player_info_entry) = self.players.entry(msg.session_id) {
            let player_info = player_info_entry.remove();
            debug!(
                "Destroying session {}. Username: {}",
                msg.session_id, player_info.username,
            );
        } else {
            warn!(
                "Trying to destroy session, that doesn't exist. Session ID: {}",
                msg.session_id
            );
        }
    }
}

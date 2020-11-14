use actix::{AsyncContext, Context, Handler, Message};
use log::{debug, warn};
use uuid::Uuid;

use super::super::{message as server_msg, Server};
use std::collections::hash_map::Entry;

#[derive(Message)]
#[rtype(result = "()")]
pub struct DestroySession {
    pub session_id: Uuid,
}

impl Handler<DestroySession> for Server {
    type Result = ();

    fn handle(&mut self, msg: DestroySession, ctx: &mut Context<Self>) -> Self::Result {
        if let Entry::Occupied(player_info_entry) = self.players.entry(msg.session_id) {
            if player_info_entry.get().current_lobby.is_some() {
                ctx.address().do_send(server_msg::LeaveLobby {
                    player_id: msg.session_id,
                });
                ctx.address().do_send(msg);
                return;
            }

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

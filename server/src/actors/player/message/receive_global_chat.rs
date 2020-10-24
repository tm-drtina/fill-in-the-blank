use actix::{Context, Handler, Message};
use chrono::{DateTime, Utc};

use super::super::super::api::message as api_msg;
use super::super::{Player, PlayerStatus};

#[derive(Message)]
#[rtype(result = "()")]
pub struct ReceiveGlobalChat {
    pub timestamp: DateTime<Utc>,
    pub system_msg: bool,
    pub username: String,
    pub message: String,
}

impl Handler<ReceiveGlobalChat> for Player {
    type Result = ();

    fn handle(&mut self, msg: ReceiveGlobalChat, _ctx: &mut Context<Self>) -> Self::Result {
        if let PlayerStatus::Connected { api_client } = &self.status {
            api_client.do_send(api_msg::GlobalChat {
                timestamp: msg.timestamp,
                system_msg: msg.system_msg,
                username: msg.username,
                message: msg.message,
            });
        }
        //TODO: else add message to buffer?
    }
}

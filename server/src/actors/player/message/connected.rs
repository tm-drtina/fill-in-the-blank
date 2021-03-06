use actix::{Addr, AsyncContext, Context, Handler, Message};

use super::super::super::api::{message as api_msg, ApiClient};
use super::super::super::messages;
use super::super::super::server::message as server_msg;
use super::super::{Player, PlayerStatus};

#[derive(Message)]
#[rtype(result = "()")]
pub struct Connected {
    pub api_client: Addr<ApiClient>,
}

impl Handler<Connected> for Player {
    type Result = ();

    fn handle(&mut self, msg: Connected, ctx: &mut Context<Self>) -> Self::Result {
        match self.status {
            PlayerStatus::Connecting => {
                self.status = PlayerStatus::Connected {
                    api_client: msg.api_client.clone(),
                };
                msg.api_client.do_send(api_msg::UserConnected {
                    session_id: self.session_id,
                    username: self.username.clone(),
                    player: ctx.address(),
                });
                self.server
                    .do_send(server_msg::GlobalChatBroadcast::system_message(format!(
                        "User '{}' connected.",
                        self.username
                    )));
            }
            PlayerStatus::LostConnection { .. } => {
                self.status = PlayerStatus::Connected {
                    api_client: msg.api_client.clone(),
                };
                msg.api_client.do_send(api_msg::UserConnected {
                    session_id: self.session_id,
                    username: self.username.clone(),
                    player: ctx.address(),
                });
                self.server
                    .do_send(server_msg::GlobalChatBroadcast::system_message(format!(
                        "User '{}' reconnected.",
                        self.username
                    )));
            }
            PlayerStatus::Connected { .. } => {
                msg.api_client.do_send(messages::Error::new(
                    messages::ErrorType::ReconnectFailed,
                    "User still connected.",
                ));
            }
        }
    }
}

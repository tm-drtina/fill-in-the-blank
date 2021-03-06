use actix::{Actor, Addr, AsyncContext, Context, Handler, Message};
use uuid::Uuid;

use super::super::super::api::ApiClient;
use super::super::super::player;
use super::super::super::player::Player;
use super::super::{PlayerInfo, Server};

#[derive(Message)]
#[rtype(result = "()")]
pub struct Connect {
    pub username: String,
    pub api_client: Addr<ApiClient>,
}

impl Handler<Connect> for Server {
    type Result = ();

    fn handle(&mut self, msg: Connect, ctx: &mut Context<Self>) -> Self::Result {
        let session_id = Uuid::new_v4();
        let player = Player::new(ctx.address(), session_id, msg.username.clone()).start();

        player.do_send(player::message::Connected {
            api_client: msg.api_client,
        });
        self.players
            .insert(session_id, PlayerInfo::new(player, msg.username));
    }
}

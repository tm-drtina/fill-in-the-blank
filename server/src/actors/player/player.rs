use actix::{Actor, ActorContext, Addr, AsyncContext, Context};
use log::info;
use std::time::{Duration, Instant};
use uuid::Uuid;

use super::super::api::ApiClient;
use super::super::server;
use super::super::server::Server;

const PLAYER_TIMEOUT_CHECK_INTERVAL: Duration = Duration::from_secs(30);
const PLAYER_TIMEOUT: Duration = Duration::from_secs(5 * 60);

pub enum PlayerStatus {
    Connecting,
    Connected { api_client: Addr<ApiClient> },
    LostConnection { last_seen: Instant },
}

pub struct Player {
    pub server: Addr<Server>,
    pub session_id: Uuid,
    pub username: String,
    pub status: PlayerStatus,
}

impl Actor for Player {
    type Context = Context<Self>;

    fn started(&mut self, ctx: &mut Context<Self>) {
        self.timeout(ctx);
    }
}

impl Player {
    pub fn new(server: Addr<Server>, session_id: Uuid, username: String) -> Self {
        Player {
            server,
            session_id,
            username,
            status: PlayerStatus::Connecting,
        }
    }

    fn timeout(&self, ctx: &mut <Self as Actor>::Context) {
        ctx.run_interval(PLAYER_TIMEOUT_CHECK_INTERVAL, |act, ctx| {
            if let PlayerStatus::LostConnection { last_seen } = act.status {
                if Instant::now().duration_since(last_seen) > PLAYER_TIMEOUT {
                    info!("Player {} timed out.", act.username);
                    act.server.do_send(server::message::DestroySession {
                        session_id: act.session_id,
                    });
                    ctx.stop();
                }
            }
        });
    }
}

use actix::{Actor, ActorContext, Addr, AsyncContext, StreamHandler};
use actix_web_actors::ws;
use log::{debug, info, warn};
use serde::Deserialize;
use std::time::{Duration, Instant};
use uuid::Uuid;

use crate::engine::{server_message, Server};
use crate::game::{player_message, Player};

/// How often heartbeat pings are sent
const HEARTBEAT_INTERVAL: Duration = Duration::from_secs(5);
/// How long before lack of client response causes a timeout
const CLIENT_TIMEOUT: Duration = Duration::from_secs(10);

#[derive(Deserialize, Debug)]
#[serde(tag = "action")]
enum Message {
    Reconnect { session_id: Uuid },
    Connect { username: String },
    GlobalChat { message: String },
}

pub struct WebSocket {
    /// Client must send ping at least once per 10 seconds (CLIENT_TIMEOUT),
    /// otherwise we drop connection.
    hb: Instant,

    server_addr: Addr<Server>,
    pub player_addr: Option<Addr<Player>>,
}

impl Actor for WebSocket {
    type Context = ws::WebsocketContext<Self>;

    /// Method is called on actor start. We start the heartbeat process here.
    fn started(&mut self, ctx: &mut Self::Context) {
        self.hb(ctx);
    }

    fn stopped(&mut self, _ctx: &mut Self::Context) {
        if let Some(player_addr) = &self.player_addr {
            player_addr.do_send(player_message::Disconnected {});
        }
    }
}

/// Handler for `ws::Message`
impl StreamHandler<Result<ws::Message, ws::ProtocolError>> for WebSocket {
    fn handle(&mut self, msg: Result<ws::Message, ws::ProtocolError>, ctx: &mut Self::Context) {
        match msg {
            Ok(ws::Message::Ping(msg)) => {
                self.hb = Instant::now();
                ctx.pong(&msg);
            }
            Ok(ws::Message::Pong(_)) => {
                self.hb = Instant::now();
            }
            Ok(ws::Message::Text(text)) => match serde_json::from_str(text.as_str()) {
                Ok(msg) => self.handle_message(ctx, msg),
                Err(_) => warn!("Unprocessable input '{}'", text),
            },
            Ok(ws::Message::Binary(_bin)) => warn!("Received binary data"),
            Ok(ws::Message::Close(_)) => ctx.stop(),
            _ => ctx.stop(),
        }
    }
}

impl WebSocket {
    pub fn new(server_addr: Addr<Server>) -> Self {
        Self {
            hb: Instant::now(),
            server_addr,
            player_addr: None,
        }
    }

    /// helper method that sends ping to client every second.
    /// also this method checks heartbeats from client
    fn hb(&self, ctx: &mut <Self as Actor>::Context) {
        ctx.run_interval(HEARTBEAT_INTERVAL, |act, ctx| {
            // check client heartbeats
            if Instant::now().duration_since(act.hb) > CLIENT_TIMEOUT {
                info!("Websocket Client heartbeat failed, disconnecting!");
                ctx.stop();
                return;
            }

            ctx.ping(b"");
        });
    }

    fn handle_message(&mut self, ctx: &mut <Self as Actor>::Context, message: Message) {
        debug!("Handling WS message: {:?}", message);
        match message {
            Message::Reconnect { session_id: id } => self.handle_reconnect(ctx, id),
            Message::Connect { username } => self.handle_connect(ctx, username),
            Message::GlobalChat { message } => self.handle_global_chat(ctx, message),
        }
    }

    fn handle_reconnect(&mut self, ctx: &mut <Self as Actor>::Context, session_id: Uuid) {
        debug!(
            "Handling Reconnect message with session id: {:?}",
            session_id
        );

        self.server_addr.do_send(server_message::Reconnect {
            websocket_addr: ctx.address(),
            session_id,
        })
    }

    fn handle_connect(&mut self, ctx: &mut <Self as Actor>::Context, username: String) {
        debug!("Handling Connect message with username: {:?}", username);

        self.server_addr.do_send(server_message::Connect {
            username: username.clone(),
            websocket_addr: ctx.address(),
        });
    }

    fn handle_global_chat(&self, _ctx: &mut <Self as Actor>::Context, message: String) {
        debug!("Handling GlobalChat message: {}", message);
        if let Some(player_addr) = &self.player_addr {
            player_addr.do_send(player_message::GlobalChat { message });
        }
    }
}

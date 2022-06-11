use actix::{Actor, ActorContext, Addr, AsyncContext, StreamHandler};
use actix_web_actors::ws;
use log::{debug, info, warn};
use serde::Deserialize;
use std::time::{Duration, Instant};
use uuid::Uuid;

use super::super::player::{message as player_msg, Player};
use super::super::server::Server;
use super::inbound_message;

/// How often heartbeat pings are sent
const HEARTBEAT_INTERVAL: Duration = Duration::from_secs(5);
/// How long before lack of client response causes a timeout
const CLIENT_TIMEOUT: Duration = Duration::from_secs(30);

#[derive(Deserialize, Debug)]
#[serde(tag = "action")]
enum Message {
    Reconnect { session_id: Uuid },
    Connect { username: String },
    Logout,
    GlobalChat { message: String },
    CreateLobby { name: String },
    JoinLobby { lobby_id: Uuid },
    LeaveLobby,
    ListLobbies,
    LobbyChat { message: String },
}

pub struct WebSocket {
    /// Client must send ping at least once per 30 seconds (CLIENT_TIMEOUT),
    /// otherwise we drop connection.
    hb: Instant,

    pub(super) server: Addr<Server>,
    pub(super) player: Option<Addr<Player>>,

    pub(super) disconnect_reason: String,
}

impl Actor for WebSocket {
    type Context = ws::WebsocketContext<Self>;

    /// Method is called on actor start. We start the heartbeat process here.
    fn started(&mut self, ctx: &mut Self::Context) {
        self.hb(ctx);
    }

    fn stopped(&mut self, _ctx: &mut Self::Context) {
        if let Some(player) = &self.player {
            player.do_send(player_msg::Disconnected {
                reason: (&self.disconnect_reason).clone(),
            });
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
            Ok(ws::Message::Text(text)) => match serde_json::from_slice(text.as_bytes()) {
                Ok(msg) => self.handle_message(ctx, msg),
                Err(_) => warn!("Unprocessable input '{}'", text),
            },
            Ok(ws::Message::Binary(_bin)) => warn!("Received binary data"),
            Ok(ws::Message::Close(reason)) => {
                if let Some(ws::CloseReason { code, description }) = reason {
                    if let Some(description) = description {
                        self.disconnect_reason = description
                    } else if code == ws::CloseCode::Normal || code == ws::CloseCode::Away {
                        self.disconnect_reason = "Disconnected.".to_string()
                    }
                }
                ctx.stop();
            }
            _ => ctx.stop(),
        }
    }
}

impl WebSocket {
    pub(super) fn new(server: Addr<Server>) -> Self {
        Self {
            hb: Instant::now(),
            server,
            player: None,
            disconnect_reason: "Unknown".to_string(),
        }
    }

    /// Helper method that sends ping to client every `HEARTBEAT_INTERVAL`.
    /// Also this method checks heartbeats from client and disconnects unresponsive clients.
    fn hb(&self, ctx: &mut <Self as Actor>::Context) {
        ctx.run_interval(HEARTBEAT_INTERVAL, |act, ctx| {
            // check client heartbeats
            if Instant::now().duration_since(act.hb) > CLIENT_TIMEOUT {
                info!("Websocket client heartbeat failed, disconnecting!");
                ctx.close(Some(ws::CloseReason {
                    code: ws::CloseCode::Abnormal,
                    description: Some("Timed out".to_string()),
                }));
                return;
            }

            ctx.ping(b"");
        });
    }

    /// Converts websocket messages to standard actix message.
    fn handle_message(&mut self, ctx: &mut <Self as Actor>::Context, message: Message) {
        debug!("Handling WS message: {:?}", message);
        match message {
            Message::Connect { username } => {
                ctx.address().do_send(inbound_message::Connect { username })
            }
            Message::Reconnect { session_id } => ctx
                .address()
                .do_send(inbound_message::Reconnect { session_id }),
            Message::Logout => ctx.address().do_send(inbound_message::Logout),
            Message::GlobalChat { message } => ctx
                .address()
                .do_send(inbound_message::GlobalChat { message }),
            Message::CreateLobby { name } => {
                ctx.address().do_send(inbound_message::CreateLobby { name })
            }
            Message::JoinLobby { lobby_id } => ctx
                .address()
                .do_send(inbound_message::JoinLobby { lobby_id }),
            Message::LobbyChat { message } => ctx
                .address()
                .do_send(inbound_message::LobbyChat { message }),
            Message::LeaveLobby => ctx.address().do_send(inbound_message::LeaveLobby),
            Message::ListLobbies => ctx.address().do_send(inbound_message::ListLobbies),
        }
    }
}

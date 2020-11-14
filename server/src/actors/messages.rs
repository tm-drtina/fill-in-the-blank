use actix::Message;
use chrono::{DateTime, Utc};
use serde::Serialize;
use uuid::Uuid;

#[derive(Serialize, Clone)]
pub struct LobbyOverview {
    pub lobby_id: Uuid,
    pub name: String,
    pub player_count: u8,
}

#[derive(Serialize)]
pub enum ErrorType {
    Disconnected,
    SessionNotFound,
    ReconnectFailed,
    LobbyLeft,
}

#[derive(Message, Serialize)]
#[serde(tag = "type")]
#[rtype(result = "()")]
pub struct Error {
    pub timestamp: DateTime<Utc>,
    pub error_type: ErrorType,
    pub text: String,
}

impl Error {
    pub fn new(error_type: ErrorType, reason: &str) -> Self {
        Self {
            timestamp: Utc::now(),
            error_type,
            text: reason.to_string(),
        }
    }
}

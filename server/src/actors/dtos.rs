use serde::Serialize;
use uuid::Uuid;

#[derive(Serialize)]
pub struct LobbyOverview {
    pub lobby_id: Uuid,
    pub name: String,
    pub player_count: u8,
}

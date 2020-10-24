use actix::{Handler, Message};
use chrono::{DateTime, Utc};
use serde::Serialize;

use super::super::WebSocket;

#[derive(Message, Serialize)]
#[serde(tag = "type")]
#[rtype(result = "()")]
pub struct Error {
    pub timestamp: DateTime<Utc>,
    pub text: String,
}

impl Handler<Error> for WebSocket {
    type Result = ();

    fn handle(&mut self, msg: Error, ctx: &mut Self::Context) -> Self::Result {
        ctx.text(serde_json::to_string(&msg).unwrap());
    }
}

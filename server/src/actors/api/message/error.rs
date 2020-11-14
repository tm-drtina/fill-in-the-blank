use actix::Handler;

use super::super::super::messages::Error;
use super::super::WebSocket;

impl Handler<Error> for WebSocket {
    type Result = ();

    fn handle(&mut self, msg: Error, ctx: &mut Self::Context) -> Self::Result {
        ctx.text(serde_json::to_string(&msg).unwrap());
    }
}

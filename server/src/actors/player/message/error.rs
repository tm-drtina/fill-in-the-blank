use actix::Handler;

use super::super::super::messages::Error;
use super::super::{Player, PlayerStatus};

impl Handler<Error> for Player {
    type Result = ();

    fn handle(&mut self, msg: Error, _ctx: &mut Self::Context) -> Self::Result {
        if let PlayerStatus::Connected { api_client } = &self.status {
            api_client.do_send(msg);
        }
    }
}

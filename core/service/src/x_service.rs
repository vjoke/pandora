use actix::prelude::*;
use actix::{Actor, Addr, Arbiter, Context, System, Message};
use actix::dev::{MessageResponse, ResponseChannel};
use std::io::Error;
use log::{trace, info, debug};

/// XService setups network, client, and extrinsic pool.
/// manages communication between them.
struct XService {

}

#[derive(Debug, PartialEq, Message)]
#[rtype(result="Result<XResponse, Error>")]
enum XMessage {
    FromClient,
    FromNetwork,
    FromTxPool,
}

#[derive(MessageResponse)]
enum XResponse {
    Ok,
    Err,
}

impl Actor for XService {
    type Context = Context<Self>;

    fn started(&mut self, ctx: &mut Self::Context) {
        trace!("main service started");
        
    }
}

/// Handler handles the message from all of sub-services
/// 
impl Handler<XMessage> for XService {
    type Result = <XMessage as Message>::Result;

    fn handle(&mut self, msg: XMessage, ctx: &mut Context<Self>) -> Self::Result {
        match msg {
            XMessage::FromClient => {
                debug!("got message from client");
                Ok(XResponse::Ok)
            },
            XMessage::FromNetwork => {
                debug!("got message from network");
                Ok(XResponse::Ok)
            },
            XMessage::FromTxPool => {
                debug!("got message from tx pool");
                Ok(XResponse::Ok)
            }
        }
    }
}


#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn launch_should_be_ok() {
        let system = System::new("main_x_service");


    }
}
use actix::dev::{MessageResponse, ResponseChannel};
use actix::prelude::*;
use actix::{Actor, Addr, Arbiter, Context, Message, System};

/// PoolMessage is sent to the subscribers when tx is imported
#[derive(Debug, PartialEq, Message)]
pub struct PoolMessage(());

#[derive(Message)]
pub struct Subscribe(pub Recipient<PoolMessage>);

#[derive(Message)]
pub struct Unsubscribe(pub Recipient<PoolMessage>);

#[derive(Message)]
pub struct Publish(pub String);

pub type PoolStubAddr = Addr<PoolStub>;

pub struct PoolStub {
    subscribers: Vec<Recipient<PoolMessage>>,
}

impl Actor for PoolStub {
    type Context = Context<Self>;
}

impl PoolStub {
    fn tx_imported(&mut self) {
        for subscr in &self.subscribers {
            subscr.do_send(PoolMessage(()));
        }
    }
}

impl Default for PoolStub {
    fn default() -> Self {
        PoolStub {
            subscribers: Vec::new(),
        }
    }
}

impl Handler<Publish> for PoolStub {
    type Result = ();

    fn handle(&mut self, msg: Publish, _: &mut Self::Context) {
        println!("got publish message :{} ", msg.0);
        self.tx_imported();
    }
}

impl Handler<Subscribe> for PoolStub {
    type Result = ();

    fn handle(&mut self, msg: Subscribe, _: &mut Self::Context) {
        println!("got subscribe message");
        self.subscribers.push(msg.0);
    }
}

impl Handler<Unsubscribe> for PoolStub {
    type Result = ();

    fn handle(&mut self, msg: Unsubscribe, _: &mut Self::Context) {
        println!("got unsubscribe message");
    }
}

#[cfg(test)]
mod tests {

}

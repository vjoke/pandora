use actix::prelude::*;
use actix::{Actor, Addr, Arbiter, Context, System, Message};
use actix::dev::{MessageResponse, ResponseChannel};
use std::io::Error;
use log::{trace, info, debug};

/// TODO:
#[derive(Debug, PartialEq, Message)]
struct ClientMessage(u64);

#[derive(Debug, PartialEq, Message)]
struct NetworkMessage(u32);

struct Informant {
}

impl Actor for Informant {
    type Context = Context<Self>;
}

impl Handler<ClientMessage> for Informant {
    type Result = ();

    fn handle(&mut self, msg: ClientMessage, _: &mut Self::Context) {
        println!("received client message: {:#?}",msg);
    }
}

impl Handler<NetworkMessage> for Informant {
   type Result = ();

    fn handle(&mut self, msg: NetworkMessage, _: &mut Self::Context) {
        info!("received network message: {:#?}", msg);
    } 
}

#[cfg(test)]
mod tests {
    use super::*;

    // #[derive(Mesage)]
    // struct BlockImported(u64);

    #[derive(Message)]
    struct Subscribe(pub Recipient<ClientMessage>);

    #[derive(Message)]
    struct Publish(pub u64);

    struct Client {
        subscribers: Vec<Recipient<ClientMessage>>,
    }

    impl Actor for Client {
        type Context = Context<Self>;
    }

    impl Client {
        pub fn block_imported(&mut self, block: u64) {
            for subscr in &self.subscribers {
                subscr.do_send(ClientMessage(block));
            }
        }
    }

    impl Default for Client {
        fn default() -> Self {
            Client {
                subscribers: Vec::new(),
            }
        }
    }

    impl Handler<Publish> for Client {
        type Result = ();

        fn handle(&mut self, msg: Publish, _: &mut Self::Context) {
            println!("got publish message :{} ", msg.0);
            self.block_imported(msg.0);
        }
    }

    impl Handler<Subscribe> for Client {
        type Result = ();

        fn handle(&mut self, msg: Subscribe, _: &mut Self::Context) {
            println!("got subscribe message");
            self.subscribers.push(msg.0);
        }
    }

    #[test]
    fn should_handle_different_message() {
        // let _ = simple_logger::init();
        
        let sys = System::new("test");

        let info_addr = Informant{}.start();
        let mut client = Client::default();
        let client_addr = client.start();

        // Send a client message
        let result = client_addr.send(Subscribe(info_addr.recipient()));

        Arbiter::spawn(
            result.and_then(move |resp|{
                assert_eq!((), resp); 
                client_addr.send(Publish(1))
            })
            .map(|resp| {
                assert_eq!((), resp);
                System::current().stop();
            })
            .map_err(|_| {
                panic!("should not happen");
            })
            
        );

        sys.run();
    }
}
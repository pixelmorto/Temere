use actix::{
    fut, Actor, ActorContext, ActorFuture, Addr, AsyncContext, ContextFutureSpawner, Handler,
    Running, StreamHandler, WrapFuture,
};
use actix_web_actors::ws::{self, Message::Text};
use serde::{Deserialize, Serialize};
use std::time::{Duration, Instant};
use uuid::Uuid;

use crate::app::{Events};

use super::lobby::Lobby;
use super::{ClientActorMessage, Connect, Disconnect, WsMessage};

const HEARTBEAT_INTERVAL: Duration = Duration::from_secs(5);
const CLIENT_TIMEOUT: Duration = Duration::from_secs(10);
#[derive(Serialize, Deserialize)]
struct Request {
    event: String,
    data: String,
    metadata: String,
}

pub struct WsConn {
    pub id: Uuid,
    pub heart_beat: Instant,
    pub lobby_addr: Addr<Lobby>,
}

impl WsConn {
    pub fn new(lobby: Addr<Lobby>) -> Self {
        Self {
            id: Uuid::new_v4(),
            heart_beat: Instant::now(),
            lobby_addr: lobby,
        }
    }
}

impl Actor for WsConn {
    type Context = ws::WebsocketContext<Self>;

    fn started(&mut self, ctx: &mut Self::Context) {
        self.heart_beat(ctx);

        let addr = ctx.address();

        self.lobby_addr
            .send(Connect {
                self_id: self.id,
                addr: addr.recipient(),
            })
            .into_actor(self)
            .then(|res, _, ctx| {
                match res {
                    Ok(_res) => (),
                    _ => ctx.stop(),
                }
                fut::ready(())
            })
            .wait(ctx)
    }

    fn stopping(&mut self, ctx: &mut Self::Context) -> Running {
        self.lobby_addr.do_send(Disconnect { id: self.id });
        Running::Stop
    }
}

impl StreamHandler<Result<ws::Message, ws::ProtocolError>> for WsConn {
    fn handle(&mut self, msg: Result<ws::Message, ws::ProtocolError>, ctx: &mut Self::Context) {
        match msg {
            Ok(ws::Message::Ping(msg)) => {
                self.heart_beat = Instant::now();
                ctx.pong(&msg);
            }
            Ok(ws::Message::Pong(_)) => {
                self.heart_beat = Instant::now();
            }
            Ok(ws::Message::Binary(bin)) => ctx.binary(bin),
            Ok(ws::Message::Close(reason)) => {
                ctx.close(reason);
                ctx.stop();
            }
            Ok(ws::Message::Continuation(_)) => {
                ctx.stop();
            }
            Ok(ws::Message::Nop) => (),
            Ok(Text(s)) => {
                let request_validation: Result<Request, serde_json::Error> = serde_json::from_str(s.as_str());

                if let Ok(request) = &request_validation {
                    match request.event.as_str() {
                        "message" => {
                            self.lobby_addr.do_send(ClientActorMessage{event:Events::Message(s), id: self.id})
                        },
                        "command" => {
                            self.lobby_addr.do_send(ClientActorMessage{event: Events::Command(s), id: self.id})
                        }
                        _ => ()
                    }
                };

                if let Err(_) =  &request_validation { 
                    ctx.text(
                        serde_json::to_string(&Request {
                            event: "error".to_string(),
                            data: "400".to_string(),
                            metadata: "".to_string(),
                        })
                        .unwrap(),
                    )
                }

                // self.lobby_addr.do_send(ClientActorMessage {
                // event: event_type,
                // id: self.id,
                // msg: s,})
            }
            Err(e) => panic!("{}", e),
        }
    }
}

impl Handler<WsMessage> for WsConn {
    type Result = ();

    fn handle(&mut self, msg: WsMessage, ctx: &mut Self::Context) {
        ctx.text(&msg.0);
        println!("{:?}", msg.0);
    }
}

impl WsConn {
    fn heart_beat(&self, ctx: &mut ws::WebsocketContext<Self>) {
        ctx.run_interval(HEARTBEAT_INTERVAL, |act, ctx| {
            if Instant::now().duration_since(act.heart_beat) > CLIENT_TIMEOUT {
                println!("Disconnecting failed heartbeat");
                act.lobby_addr.do_send(Disconnect { id: act.id });
                ctx.stop();
                return;
            }

            ctx.ping(b"PING");
        });
    }
}

use actix::prelude::{Message, Recipient};
use actix::Addr;
use actix_web::{get, web::Data, web::Path, web::Payload, Error, HttpResponse, HttpRequest};
use actix_web_actors::ws;
use uuid::Uuid;

pub mod websocket;
pub mod lobby;

use lobby::Lobby;
use websocket::WsConn;


#[derive(Message)]
#[rtype(result = "()")]
pub struct WsMessage(pub String);

#[derive(Message)]
#[rtype(result = "()")]
pub struct Connect {
    pub addr: Recipient<WsMessage>,
    pub self_id: Uuid,
}

#[derive(Message)]
#[rtype(result = "()")]
pub struct Disconnect {
    pub id: Uuid,
}

#[derive(Message, Debug)]
#[rtype(result = "()")]
pub struct ClientActorMessage<T> {
    pub event: T,
    pub id: Uuid
}


#[derive(Debug)]
pub enum Events {
    Message(String),
    Command(String),
    Jump,
    Start,
    None
}

#[get("/")]
pub async fn config(
    req: HttpRequest,
    stream: Payload,
    srv: Data<Addr<Lobby>>,
) -> Result<HttpResponse, Error> {

    let ws = WsConn::new(
        srv.get_ref().clone(),
    );

    let resp = ws::start(ws, &req, stream)?;
    Ok(resp)
}
use actix::{
    prelude::{Actor, Context, Recipient},
    Handler,
};
use actix_web::web::service;
use serde_json::json;
use std::collections::{HashMap, HashSet};
use uuid::Uuid;

// Types
use super::{ClientActorMessage, Connect, Disconnect, WsMessage};

type Socket = Recipient<WsMessage>;

pub struct Lobby {
    sessions: HashMap<Uuid, Socket>,
    rooms: HashMap<Uuid, HashSet<Uuid>>,
    waiting_room: Vec<Uuid>,
}

impl Actor for Lobby {
    type Context = Context<Self>;
}

impl Default for Lobby {
    fn default() -> Lobby {
        Lobby {
            sessions: HashMap::new(),
            rooms: HashMap::new(),
            waiting_room: Vec::new(),
        }
    }
}

impl Lobby {
    fn send_message(&self, message: &str, id_to: &Uuid) {
        if let Some(socket_recipient) = self.sessions.get(id_to) {
            let _ = socket_recipient.do_send(WsMessage(message.to_owned()));
        } else {
            println!("attempting to send message but couldn't find user id.");
        }
    }
}

impl Handler<Disconnect> for Lobby {
    type Result = ();

    fn handle(&mut self, msg: Disconnect, ctx: &mut Self::Context) -> Self::Result {
        // Percorrer todas as salas
        for (room_id, members_list) in self.rooms.iter() {
            // Percorrer os membros da sala
            if members_list.get(&msg.id).is_some() {
                if self.sessions.remove(&msg.id).is_some() {
                    self.rooms
                        .get(&room_id)
                        .unwrap()
                        .iter()
                        .filter(|conn_id| *conn_id.to_owned() != msg.id)
                        .for_each(|user_id| {
                            self.send_message(&format!("{} disconnected.", &msg.id), user_id);
                            //self.waiting_room.push(*user_id);
                            //self.rooms.remove(room_id);
                        });
                }
            }
        }
    }
}

impl Handler<Connect> for Lobby {
    type Result = ();

    fn handle(&mut self, msg: Connect, ctx: &mut Self::Context) -> Self::Result {
        self.waiting_room.push(msg.self_id);

        self.sessions.insert(msg.self_id, msg.addr);

        self.send_message(
            &json!({"status": "connected", "id": msg.self_id.to_string()}).to_string(),
            &msg.self_id,
        );
    }
}

impl Handler<ClientActorMessage> for Lobby {
    type Result = ();

    fn handle(&mut self, request: ClientActorMessage, _: &mut Context<Self>) -> Self::Result {
        // Em caso de comandos
        if request.msg.starts_with("/") {
            println!("Commando: {}", request.msg);
            match request.msg.as_str() {
                "/new" => {
                    let mut room_id = Uuid::new_v4();

                    for (id, member_list) in self.rooms.iter() {
                        if member_list.len() == 1 {
                            room_id = *id;
                        }
                    }

                    // Create a room if not exists and add user
                    self.rooms
                        .entry(room_id)
                        .or_insert_with(HashSet::new)
                        .insert(request.id);

                    self.rooms
                        .get(&room_id)
                        .unwrap()
                        .iter()
                        .filter(|conn_id| *conn_id.to_owned() != request.id)
                        .for_each(|conn_id| {
                            self.send_message(&json!({"joined": true}).to_string(), conn_id)
                        });
                    
                    self.send_message(&json!({"connected": room_id.to_string()}).to_string(), &request.id);
                }
                _ => self.send_message(
                    &json!({"inbox": {"id": "shuffle", "message": "Command not found"}})
                        .to_string(),
                    &request.id,
                ),
            }
        }
        // Em caso de mensagens
        else {
            for user in self.waiting_room.iter() {
                if user == &request.id {
                    self.send_message(&json!({"code" : 400, "message": "you cannot send messages in the waiting room"}).to_string(), &request.id);
                }
            }

            for (room_id, members_list) in self.rooms.iter() {
                if members_list.get(&request.id).is_some() {
                    self.rooms.get(room_id).unwrap().iter().for_each(|user| {
                        self.send_message(&json!({"inbox": {"id": request.id.to_string(), "message": request.msg.as_str()}}).to_string(), user);
                    })
                }
            }
        }
    }
}

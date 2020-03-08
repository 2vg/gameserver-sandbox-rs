use actix::prelude::*;
use serde::*;
use serde_json::{json, Result, Value};

use crate::domain::models;
use crate::domain::repositories::Repository;

use std::collections::HashMap;

#[derive(Message)]
#[rtype(result = "()")]
pub struct Message(pub String);

#[derive(Message)]
#[rtype(u32)]
pub struct Connect {
    pub addr: Recipient<Message>,
}

#[derive(Message)]
#[rtype(result = "()")]
pub struct Disconnect {
    pub id: u32,
}

#[derive(Message)]
#[rtype(result = "()")]
pub struct ClientMessage {
    pub id: u32,
    pub msg: String,
}

pub struct GameServer<R: std::marker::Unpin + std::marker::Send + 'static + Repository + Clone> {
    repo: R,
    sessions: HashMap<u32, Recipient<Message>>,
}

impl<R: std::marker::Unpin + std::marker::Send + 'static + Repository + Clone> GameServer<R> {
    pub fn new(repo: R) -> GameServer<R> {
        GameServer {
            repo: repo,
            sessions: HashMap::new(),
        }
    }

    fn send_message(&self, my_id: u32, message: &str) {
        for (id, addr) in &self.sessions {
            // no need send new pos to ownself client
            if *id == my_id {
                continue;
            };
            let _ = addr.do_send(Message(message.to_owned()));
        }
    }
}

impl<R: std::marker::Unpin + std::marker::Send + 'static + Repository + Clone> Actor
    for GameServer<R>
{
    type Context = Context<Self>;
}

impl<R: std::marker::Unpin + std::marker::Send + 'static + Repository + Clone> Handler<Connect>
    for GameServer<R>
{
    type Result = u32;

    fn handle(&mut self, msg: Connect, _: &mut Context<Self>) -> Self::Result {
        let ent = models::entities::Entity::new_with_empty();
        if let Ok(new_ent) = self.repo.create_entity(ent) {
            let j = format!(
                "{{\"id\":{}, \"x\":{},\"y\"{}}}",
                new_ent.id, new_ent.pos.0, new_ent.pos.1
            );
            // need to know the unique ID, will send only the id.
            let _ = msg.addr.do_send(Message(new_ent.id.to_string()));

            // If have a client other than self when connect, need the Entity data.
            for (id, _) in &self.sessions {
                if let Ok(ent) = self.repo.select_entity(*id) {
                    let j = format!(
                        "{{\"id\":{}, \"x\":{},\"y\"{}}}",
                        ent.id, ent.pos.0, ent.pos.1
                    );
                    let _ = msg.addr.do_send(Message(j));
                }
            }
            self.sessions.insert(new_ent.id, msg.addr);
            self.send_message(new_ent.id, &j);
            new_ent.id
        } else {
            0
        }
    }
}

impl<R: std::marker::Unpin + std::marker::Send + 'static + Repository + Clone> Handler<Disconnect>
    for GameServer<R>
{
    type Result = ();

    fn handle(&mut self, msg: Disconnect, _: &mut Context<Self>) {
        let _ = self.repo.delete_entity(msg.id);
        // when delete entity, send {id: ent.id, x: -1, y: -1} to all client
        // then client can delete entity from screen with this data
        let j = format!("{{\"id\":{}, \"x\":{},\"y\"{}}}", msg.id, -1, -1);
        self.send_message(msg.id, &j);
        self.sessions.remove(&msg.id);
    }
}

impl<R: std::marker::Unpin + std::marker::Send + 'static + Repository + Clone>
    Handler<ClientMessage> for GameServer<R>
{
    type Result = ();

    fn handle(&mut self, msg: ClientMessage, _: &mut Context<Self>) {
        match serde_json::from_str::<Value>(&msg.msg) {
            Ok(value) => {
                // first, need old
                if let Ok(mut ent) = self.repo.select_entity(msg.id) {
                    // if found, update ent.pos
                    let x = &ent.pos.0 + (*&value["x"].as_i64().unwrap() as i32);
                    let y = &ent.pos.1 + (*&value["y"].as_i64().unwrap() as i32);
                    ent.pos.0 = x;
                    ent.pos.1 = y;

                    if let Ok(ent) = self.repo.update_entity(ent) {
                        let j = format!(
                            "{{\"id\":{}, \"x\":{},\"y\"{}}}",
                            ent.id, ent.pos.0, ent.pos.1
                        );
                        self.send_message(msg.id, &j);
                    };
                };
            }
            Err(_) => {}
        };
    }
}

use actix::prelude::*;

use crate::domain::models;
use crate::domain::repositories::Repository;

use std::collections::{HashMap, HashSet};

#[derive(Message)]
#[rtype(result = "()")]
pub struct Message(pub String);

#[derive(Message)]
#[rtype(u32)]
pub struct Connect<'r> {
    pub repo: &'r Repository,
    pub addr: Recipient<Message>,
}

#[derive(Message)]
#[rtype(result = "()")]
pub struct Disconnect<'r> {
    pub id: u32,
    pub repo: &'r Repository, 
}

#[derive(Message)]
#[rtype(result = "()")]
pub struct ClientMessage<'r> {
    pub id: u32,
    pub msg: String,
    pub repo: &'r Repository, 
}

pub struct GameServer {
    sessions: HashMap<u32, Recipient<Message>>
}

impl Default for GameServer {
    fn default() -> GameServer {
        GameServer {
            sessions: HashMap::new()
        }
    }
}

impl GameServer {
    fn send_message(&self, repo: &Repository, my_id: u32, message: &str) {
        for (id, addr) in &self.sessions {
            if *id == my_id { continue }
            let _ = addr.do_send(Message(message.to_owned()));
        }
    }
}

impl Actor for GameServer {
    type Context = Context<Self>;
}

impl<'r> Handler<Connect<'r>> for GameServer {
    type Result = u32;

    fn handle(&mut self, msg: Connect, _: &mut Context<Self>) -> Self::Result {
        let result = models::entities::Entity::new_with_empty();
        //let result = msg.repo.create_entity(ent);
        self.sessions.insert(result.id, msg.addr);
        result.id
    }
}

impl<'r> Handler<Disconnect<'r>> for GameServer {
    type Result = ();

    fn handle(&mut self, msg: Disconnect, _: &mut Context<Self>) {
        self.sessions.remove(&msg.id);
    }
}

impl<'r> Handler<ClientMessage<'r>> for GameServer {
    type Result = ();

    fn handle(&mut self, msg: ClientMessage, _: &mut Context<Self>) {
        self.send_message(msg.repo, msg.id, msg.msg.as_str());
    }
}

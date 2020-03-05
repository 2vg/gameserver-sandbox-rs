use actix::prelude::*;

use crate::domain::models;
use crate::domain::repositories::Repository;

use std::collections::HashMap;

#[derive(Message)]
#[rtype(result = "()")]
pub struct Message(pub String);

#[derive(Message)]
#[rtype(u32)]
pub struct Connect{
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
    sessions: HashMap<u32, Recipient<Message>>
}

impl<R: std::marker::Unpin + std::marker::Send + 'static + Repository + Clone> GameServer<R> {
    pub fn new(repo: R) -> GameServer<R> {
        GameServer {
            repo: repo,
            sessions: HashMap::new()
        }
    }

    fn send_message(&self, my_id: u32, message: &str) {
        for (id, addr) in &self.sessions {
            if *id == my_id { continue }
            let _ = addr.do_send(Message(message.to_owned()));
        }
    }
}

impl<R: std::marker::Unpin + std::marker::Send + 'static + Repository + Clone> Actor for GameServer<R> {
    type Context = Context<Self>;
}

impl<R: std::marker::Unpin + std::marker::Send + 'static + Repository + Clone> Handler<Connect> for GameServer<R> {
    type Result = u32;

    fn handle(&mut self, msg: Connect, _: &mut Context<Self>) -> Self::Result {
        let ent = models::entities::Entity::new_with_empty();
        if let Ok(result) = self.repo.create_entity(ent) {
            self.sessions.insert(result.id, msg.addr);
            result.id
        }
        else {
            0
        }
    }
}

impl<R: std::marker::Unpin + std::marker::Send + 'static + Repository + Clone> Handler<Disconnect> for GameServer<R> {
    type Result = ();

    fn handle(&mut self, msg: Disconnect, _: &mut Context<Self>) {
        self.sessions.remove(&msg.id);
    }
}

impl<R: std::marker::Unpin + std::marker::Send + 'static + Repository + Clone> Handler<ClientMessage> for GameServer<R> {
    type Result = ();

    fn handle(&mut self, msg: ClientMessage, _: &mut Context<Self>) {
        self.send_message(msg.id, msg.msg.as_str());
    }
}

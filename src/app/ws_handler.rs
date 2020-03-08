use actix::prelude::*;
use actix_web::{web, Error, HttpRequest, HttpResponse};
use actix_web_actors::ws;

use std::time::{Duration, Instant};

use crate::app::game_server_actor as GameServerActor;
use crate::domain::repositories::Repository;

const HEARTBEAT_INTERVAL: Duration = Duration::from_secs(5);
const CLIENT_TIMEOUT: Duration = Duration::from_secs(10);

pub async fn ws_route<R: std::marker::Unpin + std::marker::Send + 'static + Repository + Clone>(
    req: HttpRequest,
    stream: web::Payload,
    srv: web::Data<Addr<GameServerActor::GameServer<R>>>,
) -> Result<HttpResponse, Error> {
    ws::start(
        WsSession {
            id: 0,
            hb: Instant::now(),
            addr: srv.get_ref().clone(),
        },
        &req,
        stream,
    )
}

struct WsSession<R: std::marker::Unpin + std::marker::Send + 'static + Repository + Clone> {
    id: u32,
    hb: Instant,
    /// Game server
    addr: Addr<GameServerActor::GameServer<R>>,
}

impl<R: std::marker::Unpin + std::marker::Send + 'static + Repository + Clone> Actor
    for WsSession<R>
{
    type Context = ws::WebsocketContext<Self>;

    // when client connected
    fn started(&mut self, ctx: &mut Self::Context) {
        println!("connected");
        self.hb(ctx);

        let addr = ctx.address();
        self.addr
            .send(GameServerActor::Connect {
                addr: addr.recipient(),
            })
            .into_actor(self)
            .then(|res, act, ctx| {
                match res {
                    Ok(res) => act.id = res,
                    _ => ctx.stop(),
                }
                fut::ready(())
            })
            .wait(ctx);
    }

    // when client disconnected
    fn stopping(&mut self, _: &mut Self::Context) -> Running {
        self.addr
            .do_send(GameServerActor::Disconnect { id: self.id });
        Running::Stop
    }
}

impl<R: std::marker::Unpin + std::marker::Send + 'static + Repository + Clone>
    Handler<GameServerActor::Message> for WsSession<R>
{
    type Result = ();

    fn handle(&mut self, msg: GameServerActor::Message, ctx: &mut Self::Context) {
        ctx.text(msg.0);
    }
}

impl<R: std::marker::Unpin + std::marker::Send + 'static + Repository + Clone>
    StreamHandler<Result<ws::Message, ws::ProtocolError>> for WsSession<R>
{
    fn handle(&mut self, msg: Result<ws::Message, ws::ProtocolError>, ctx: &mut Self::Context) {
        let msg = match msg {
            Err(_) => {
                ctx.stop();
                return;
            }
            Ok(msg) => msg,
        };

        match msg {
            ws::Message::Ping(msg) => {
                self.hb = Instant::now();
                ctx.pong(&msg);
            }
            ws::Message::Pong(_) => {
                self.hb = Instant::now();
            }
            ws::Message::Text(text) => {
                println!("recv msg");
                let m = text.trim();

                self.addr.do_send(GameServerActor::ClientMessage {
                    id: self.id,
                    msg: m.to_string(),
                })
            }
            ws::Message::Binary(_) => println!("Unexpected binary"),
            ws::Message::Close(_) => {
                ctx.stop();
            }
            ws::Message::Continuation(_) => {
                ctx.stop();
            }
            ws::Message::Nop => (),
        }
    }
}

impl<R: std::marker::Unpin + std::marker::Send + 'static + Repository + Clone> WsSession<R> {
    fn hb(&self, ctx: &mut ws::WebsocketContext<Self>) {
        ctx.run_interval(HEARTBEAT_INTERVAL, |act, ctx| {
            if Instant::now().duration_since(act.hb) > CLIENT_TIMEOUT {
                println!("Websocket Client heartbeat failed, disconnecting!");
                act.addr.do_send(GameServerActor::Disconnect { id: act.id });
                ctx.stop();
                return;
            }
            ctx.ping(b"");
        });
    }
}

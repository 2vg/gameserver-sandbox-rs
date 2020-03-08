use actix::prelude::*;
use actix_web::{web, App, HttpServer};

use crate::app::ws_handler;
use crate::domain::repositories::Repository;

pub async fn start_server<
    R: std::marker::Unpin + std::marker::Send + 'static + Repository + Clone,
>(
    repository: R,
) -> std::io::Result<()> {
    let server = crate::app::game_server_actor::GameServer::new(repository).start();
    HttpServer::new(move || {
        App::new()
            .data(server.clone())
            // websocket
            .service(web::resource("/ws/").to(ws_handler::ws_route::<R>))
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}

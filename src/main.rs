use actix_web::{
    web::{get, post, scope, Data},
    App, HttpServer,
};
use rusqlite::Connection;

mod db;
mod entries_repo;
mod entries_service;
mod feeds_repo;
mod feeds_service;
mod model;

pub struct State {
    conn: Connection,
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    db::init().expect("Failed to initialize database");

    HttpServer::new(|| {
        App::new()
            .app_data(Data::new(State {
                conn: db::connect().expect("Failed to setup database connection"),
            }))
            .service(
                scope("/feeds")
                    .route("", post().to(feeds_service::post))
                    .route("", get().to(feeds_service::get_all))
                    .route("/{id}", get().to(feeds_service::get_by_id)),
            )
            .service(
                scope("/entries")
                    .route("", get().to(entries_service::get_all))
                    .route("/{id}", get().to(entries_service::get_by_id)),
            )
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}

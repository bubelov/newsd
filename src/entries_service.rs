use actix_web::{
    web::{Data, Path},
    HttpResponse, Responder,
};

use crate::{entries_repo, State};

pub async fn get_all(state: Data<State>) -> impl Responder {
    HttpResponse::Ok().json(entries_repo::select_all(&state.conn).unwrap())
}

pub async fn get_by_id(state: Data<State>, path: Path<String>) -> impl Responder {
    HttpResponse::Ok().json(entries_repo::select_by_id(&state.conn, path.into_inner()).unwrap())
}

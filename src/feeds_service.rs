use actix_web::{
    web::{Data, Json, Path},
    HttpResponse, Responder,
};
use chrono::Utc;
use reqwest::Url;
use serde::{Deserialize, Serialize};

use crate::model::{Entry, Feed};
use crate::{entries_repo, feeds_repo, State};

#[derive(Serialize, Deserialize)]
pub struct PostArgs {
    url: String,
}

pub async fn post(state: Data<State>, args: Json<PostArgs>) -> impl Responder {
    let resp = reqwest::get(Url::parse(args.url.as_str()).unwrap())
        .await
        .unwrap()
        .text()
        .await
        .unwrap();

    let feed = feed_rs::parser::parse(resp.as_bytes()).unwrap();

    let title: String = match feed.title {
        Some(title) => title.content,
        None => String::new(),
    };

    feeds_repo::insert(
        &state.conn,
        Feed {
            id: feed.id,
            links: feed.links.iter().map(|it| it.into()).collect(),
            title,
        },
    )
    .unwrap();

    for entry in feed.entries {
        entries_repo::insert(
            &state.conn,
            Entry {
                content: entry.content.map_or(None, |it| it.body),
                id: entry.id,
                links: entry.links.iter().map(|it| it.into()).collect(),
                published: entry
                    .published
                    .map_or(None, |it| Some(it.format("%+").to_string())),
                summary: entry.summary.map_or(None, |it| Some(it.content)),
                title: entry.title.map_or("".to_string(), |it| it.content),
                updated: entry
                    .updated
                    .map_or(Utc::now().format("%+").to_string(), |it| {
                        it.format("%+").to_string()
                    }),
            },
        );
    }

    HttpResponse::Created()
}

pub async fn get_all(state: Data<State>) -> impl Responder {
    HttpResponse::Ok().json(feeds_repo::select_all(&state.conn).unwrap())
}

pub async fn get_by_id(state: Data<State>, path: Path<String>) -> impl Responder {
    HttpResponse::Ok().json(feeds_repo::select_by_id(&state.conn, path.into_inner()).unwrap())
}

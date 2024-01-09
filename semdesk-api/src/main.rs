mod actor;

use core::panic;

use actix::{Addr, SyncArbiter};
use actix_web::middleware::Logger;
use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder};
use actor::{indexer, IndexActor };
use env_logger::Env;
use rand::RngCore;
use serde::{Deserialize, Serialize};

use crate::actor::IndexResponse;

#[get("/")]
async fn hello() -> impl Responder {
    HttpResponse::Ok().body("Hello world!")
}

#[derive(Deserialize)]
struct Source {
    name: String,
    url: String,
}

#[derive(Deserialize)]
struct Ingest {
    title: String,
    content: String,
    source: Source,
}

#[derive(Serialize)]
pub struct SearchResult {
    key: u64,
    distance: f32,
}

#[derive(Deserialize)]
struct Search {
    query: String,
}

#[derive(Serialize)]
struct SearchResults {
    results: Vec<SearchResult>,
}

struct AppState {
    indexer: Addr<IndexActor>,
}

#[post("/ingest")]
async fn ingest(ingest: web::Json<Ingest>, data: web::Data<AppState>) -> impl Responder {
    let mut rng = rand::thread_rng();
    let key = rng.next_u64();
    let response = data
        .indexer
        .send(actor::IndexMessage::Index {
            key: key,
            text: ingest.content.clone(),
        })
        .await
        .unwrap();

    format!("ingested {} to {}", ingest.source.url, key)
}

#[get("/search")]
async fn search(search: web::Query<Search>, data: web::Data<AppState>) -> impl Responder {
    let response = data
        .indexer
        .send(actor::IndexMessage::Search {
            query: search.query.clone(),
        })
        .await
        .unwrap();

    match response {
        IndexResponse::SearchResult { results } => web::Json(SearchResults {
            results: results.iter().map(|r| SearchResult {
                key: r.key,
                distance: r.distance,
            }).collect::<Vec<_>>()
        }),
        _ => panic!()
    }

}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env_logger::init_from_env(Env::default().default_filter_or("info"));

    let indexer = SyncArbiter::start(1, || indexer());
    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(AppState {
                indexer: indexer.clone(),
            }))
            .service(hello)
            .service(ingest)
            .service(search)
            .wrap(Logger::default())
            .wrap(Logger::new("%a %{User-Agent}i"))
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}

mod indexer;
mod searcher;

use core::panic;

use actix::{Addr, SyncArbiter};
use actix_web::middleware::Logger;
use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder};
use env_logger::Env;
use indexer::{indexer, IndexerActor};
use rand::RngCore;
use searcher::{searcher, SearcherActor};
use serde::{Deserialize, Serialize};

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
    searcher: Addr<SearcherActor>,
    indexer: Addr<IndexerActor>,
}

#[post("/ingest")]
async fn ingest(ingest: web::Json<Ingest>, data: web::Data<AppState>) -> impl Responder {
    let mut rng = rand::thread_rng();
    let key = rng.next_u64();
    data.indexer
        .send(indexer::IndexMessage::Index {
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
        .searcher
        .send(searcher::SearchMessage::Search {
            query: search.query.clone(),
        })
        .await
        .unwrap();

    match response {
        searcher::SearchResponse::SearchResult { results } => web::Json(SearchResults {
            results: results
                .iter()
                .map(|r| SearchResult {
                    key: r.key,
                    distance: r.distance,
                })
                .collect::<Vec<_>>(),
        }),
        _ => panic!(),
    }
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env_logger::init_from_env(Env::default().default_filter_or("info"));

    let searcher = SyncArbiter::start(1, || searcher());
    let s1_addr = searcher.clone();
    let indexer = SyncArbiter::start(1, move || indexer(s1_addr.clone()));

    HttpServer::new(move || {
        let searcher = searcher.clone();
        App::new()
            .app_data(web::Data::new(AppState {
                indexer: indexer.clone(),
                searcher: searcher.clone(),
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

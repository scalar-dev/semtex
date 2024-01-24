mod indexer;
mod searcher;

use core::panic;
use actix_cors::Cors;
use std::collections::HashMap;
use std::fmt::Debug;
use std::time::Instant;

use actix::{Addr, SyncArbiter};
use actix_web::middleware::Logger;
use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder};
use chrono::Utc;
use entity::content;
use env_logger::Env;
use indexer::{indexer, IndexerActor};
use migration::{Migrator, MigratorTrait};
use rand::RngCore;
use sea_orm::{
    ActiveModelTrait, ActiveValue, ColumnTrait, DatabaseConnection, EntityTrait, QueryFilter,
};
use searcher::{searcher, SearcherActor};
use semdesk_vector::jina_candle::{self, JinaCandle};
use semdesk_vector::minilm::MiniLM;
use serde::{Deserialize, Serialize};

#[derive(Deserialize)]
struct Source {
    name: String,
    url: Option<String>,
}

#[derive(Deserialize)]
struct Ingest {
    items: Vec<IngestItem>,
}

#[derive(Deserialize)]
struct IngestItem {
    title: String,
    content: String,
    source: Source,
}

#[derive(Serialize)]
pub struct SearchResult {
    key: i32,
    title: String,
    text: String,
    url: Option<String>,
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
    db: DatabaseConnection,
}

#[derive(Clone)]
pub struct Models {
    // jina_candle: JinaCandle,
}

#[get("/")]
async fn root() -> impl Responder {
    HttpResponse::Ok().body("semdesk")
}

#[post("/ingest")]
async fn ingest(ingest: web::Json<Ingest>, data: web::Data<AppState>) -> impl Responder {
    for item in &ingest.items {
        let record = content::ActiveModel {
            id: ActiveValue::NotSet,
            created_at: ActiveValue::Set(Utc::now().to_rfc3339()),
            title: ActiveValue::Set(item.title.to_owned()),
            text: ActiveValue::Set(item.content.to_owned()),
            source: ActiveValue::Set(item.source.name.to_owned()),
            url: ActiveValue::Set(item.source.url.to_owned()),
        };

        let result = record.insert(&data.db).await;
        let key = result.unwrap().id;

        data.indexer
            .send(indexer::IndexMessage::Index {
                key: key as u64,
                text: item.content.clone(),
            })
            .await
            .unwrap();
    }

    format!("ingested {}", ingest.items.len())
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
        searcher::SearchResponse::SearchResult { results } => {
            let ids = results.iter().map(|r| r.key as i32).collect::<Vec<_>>();
            let records = content::Entity::find()
                .filter(content::Column::Id.is_in(ids))
                .all(&data.db)
                .await
                .unwrap();

            let distance_by_id = results
                .iter()
                .map(|r| (r.key as i32, r.distance))
                .collect::<HashMap<_, _>>();

            let mut results = records
                .iter()
                .map(|r| SearchResult {
                    key: r.id,
                    distance: distance_by_id[&r.id],
                    title: r.title.to_owned(),
                    text: r.text.to_owned(),
                    url: r.url.to_owned(),
                })
                .collect::<Vec<_>>();

            results.sort_by(|a, b| a.distance.partial_cmp(&b.distance).unwrap());

            web::Json(SearchResults { results })
        }
        _ => panic!(),
    }
}

pub async fn run_server() -> std::io::Result<()> {
    env_logger::init_from_env(Env::default().default_filter_or("info"));

    let models = Models {
        // jina_candle: jina_candle::JinaCandle::new().unwrap(),
    };

    let searcher_models = models.clone();
    let searcher = SyncArbiter::start(1, move || searcher(&searcher_models));

    let searher_addr = searcher.clone();
    let indexer_models = models.clone();
    let indexer = SyncArbiter::start(1, move || indexer(&indexer_models, &searher_addr));

    let connection = sea_orm::Database::connect("sqlite://db.sqlite?mode=rwc")
        .await
        .unwrap();
    Migrator::up(&connection, None).await.unwrap();


    HttpServer::new(move || {
        let cors = Cors::permissive();
        App::new()
            .app_data(web::Data::new(AppState {
                indexer: indexer.clone(),
                searcher: searcher.clone(),
                db: connection.clone(),
            }))
            .service(root)
            .service(ingest)
            .service(search)
            .wrap(Logger::default())
            .wrap(Logger::new("%a %{User-Agent}i"))
            .wrap(cors)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}

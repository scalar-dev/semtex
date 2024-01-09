use actix::dev::{MessageResponse, OneshotSender};
use actix::prelude::*;
use serde::Serialize;
use usearch::{new_index, Index};

use log::info;
use semdesk_vector::{to_vec, LoadedModel, build_model_and_tokenizer_jina};
use usearch::ffi::{IndexOptions, MetricKind, ScalarKind};

#[derive(Message)]
#[rtype(result = "IndexResponse")]
pub enum IndexMessage {
    Index { key: u64, text: String },
    Search { query: String },
}

pub struct SearchResult {
    pub key: u64,
    pub distance: f32,
}

pub enum IndexResponse {
    IndexResult,
    SearchResult { results: Vec<SearchResult> },
}

impl<A, M> MessageResponse<A, M> for IndexResponse
where
    A: Actor,
    M: Message<Result = IndexResponse>,
{
    fn handle(self, ctx: &mut A::Context, tx: Option<OneshotSender<M::Result>>) {
        if let Some(tx) = tx {
            tx.send(self);
        }
    }
}

pub struct IndexActor {
    index: Index,
    model: LoadedModel,
}

impl Actor for IndexActor {
    type Context = SyncContext<Self>;

    fn started(&mut self, _ctx: &mut SyncContext<Self>) {
        println!("Actor is alive");
    }

    fn stopped(&mut self, _ctx: &mut SyncContext<Self>) {
        println!("Actor is stopped");
    }
}

pub fn indexer() -> IndexActor {
    let options = IndexOptions {
        multi: false,
        dimensions: 512, //768,
        metric: MetricKind::Cos,
        quantization: ScalarKind::F32,
        connectivity: 0,
        expansion_add: 0,
        expansion_search: 0,
    };

    let index = new_index(&options).unwrap();
    
    match index.load("index.usearch") {
        Err(_) => {
            index.save("index.usearch").unwrap();
        }
        Ok(_) => info!("All good")
    }
    index.reserve(10).unwrap();

    let model = build_model_and_tokenizer_jina().unwrap();

    IndexActor {
        index: index,
        model: model,
    }
}

impl Handler<IndexMessage> for IndexActor {
    type Result = IndexResponse;

    fn handle(&mut self, msg: IndexMessage, _ctx: &mut SyncContext<Self>) -> Self::Result {
        match msg {
            IndexMessage::Index { key, text } => {
                let v = to_vec(&mut self.model, &[&text]);
                info!("Got vector of length {}", v[0].len());
                self.index.add(key, &v[0]).unwrap();
                self.index.save("index.usearch").unwrap();
                return IndexResponse::IndexResult;
            }
            IndexMessage::Search { query } => {
                let v = to_vec(&mut self.model, &[&query]);
                let results = self.index.search(&v[0], 10).unwrap();

                return IndexResponse::SearchResult {
                    results: results
                        .keys
                        .iter()
                        .zip(results.distances)
                        .map(|(k, d)| SearchResult {
                            key: *k,
                            distance: d,
                        })
                        .collect::<Vec<_>>(),
                };
            }
        }
    }
}

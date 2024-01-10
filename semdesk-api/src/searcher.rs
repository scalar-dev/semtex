use std::cmp::{min, max};

use actix::dev::{MessageResponse, OneshotSender};
use actix::prelude::*;
use semdesk_vector::embed;
use semdesk_vector::jina_candle::JinaCandle;
use semdesk_vector::minilm::MiniLM;
use usearch::{new_index, Index};

use usearch::ffi::{IndexOptions, MetricKind, ScalarKind};

use crate::Models;

#[derive(Message)]
#[rtype(result = "SearchResponse")]
pub enum SearchMessage {
    Search { query: String },
    Index { key: u64, vector: Vec<f32> },
}

#[derive(Debug)]
pub struct SearchResult {
    pub key: u64,
    pub distance: f32,
}

#[derive(Debug)]
pub enum SearchResponse {
    SearchResult { results: Vec<SearchResult> },
    IndexResult,
}

impl<A, M> MessageResponse<A, M> for SearchResponse
where
    A: Actor,
    M: Message<Result = SearchResponse>,
{
    fn handle(self, _ctx: &mut A::Context, tx: Option<OneshotSender<M::Result>>) {
        if let Some(tx) = tx {
            tx.send(self).unwrap();
        }
    }
}

pub struct SearcherActor {
    models: Models,
    minilm: MiniLM,
    index: Index,
}

impl Actor for SearcherActor {
    type Context = SyncContext<Self>;

    fn started(&mut self, _ctx: &mut SyncContext<Self>) {
        println!("Actor is alive");
    }

    fn stopped(&mut self, _ctx: &mut SyncContext<Self>) {
        println!("Actor is stopped");
    }
}


pub fn searcher(models: &Models) -> SearcherActor {
    let options = IndexOptions {
        multi: false,
        dimensions: 384, //512, //768,
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
        Ok(_) => ()
    }

    SearcherActor {
        models: models.clone(),
        minilm: MiniLM::new(),
        index: index,
    }
}


impl Handler<SearchMessage> for SearcherActor {
    type Result = SearchResponse;

    fn handle(&mut self, msg: SearchMessage, _ctx: &mut SyncContext<Self>) -> Self::Result {
        match msg {
            SearchMessage::Search { query } => {
                let v = embed(&mut self.minilm, &[&query]);
                println!("Vector {}", v.len());
                println!("Vector {}", v[0].len());
                let results = self.index.search(&v[0], 10).unwrap();

                return SearchResponse::SearchResult {
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
            SearchMessage::Index { key, vector } => {
                if self.index.capacity() <= self.index.size() {
                    self.index.reserve(max(100, self.index.capacity() * 2)).unwrap();
                }

                self.index.add(key, vector.as_slice()).unwrap();
                self.index.save("index.usearch").unwrap();
                SearchResponse::IndexResult
            }
        }
    }
}

use actix::dev::{MessageResponse, OneshotSender};
use actix::prelude::*;
use semdesk_vector::embed;
use semdesk_vector::jina_candle::JinaCandle;

use crate::searcher::SearcherActor;

#[derive(Message)]
#[rtype(result = "IndexResponse")]
pub enum IndexMessage {
    Index { key: u64, text: String },
}

#[derive(Debug)]
pub enum IndexResponse {
    IndexResult,
}

impl<A, M> MessageResponse<A, M> for IndexResponse
where
    A: Actor,
    M: Message<Result = IndexResponse>,
{
    fn handle(self, _ctx: &mut A::Context, tx: Option<OneshotSender<M::Result>>) {
        if let Some(tx) = tx {
            tx.send(self).unwrap();
        }
    }
}

pub struct IndexerActor {
    model: JinaCandle,
    searcher: Addr<SearcherActor>,
}

impl Actor for IndexerActor {
    type Context = SyncContext<Self>;

    fn started(&mut self, _ctx: &mut SyncContext<Self>) {
        println!("Actor is alive");
    }

    fn stopped(&mut self, _ctx: &mut SyncContext<Self>) {
        println!("Actor is stopped");
    }
}

pub fn indexer(searcher: Addr<SearcherActor>) -> IndexerActor {
    let model = JinaCandle::new().unwrap();

    IndexerActor {
        model: model,
        searcher: searcher,
    }
}

impl Handler<IndexMessage> for IndexerActor {
    type Result = IndexResponse;

    fn handle(&mut self, msg: IndexMessage, _ctx: &mut SyncContext<Self>) -> Self::Result {
        match msg {
            IndexMessage::Index { key, text } => {
                let v = embed(&mut self.model, &[&text]);

                let _ = self.searcher.send(crate::searcher::SearchMessage::Index {
                    key,
                    vector: v[0].clone(),
                });

                return IndexResponse::IndexResult;
            }
        }
    }
}

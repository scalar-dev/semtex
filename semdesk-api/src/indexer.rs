use actix::dev::{MessageResponse, OneshotSender};
use actix::prelude::*;
use actix_web::rt::Runtime;
use semdesk_vector::embed;
use semdesk_vector::minilm::MiniLM;

use crate::Models;
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
    models: Models,
    searcher: Addr<SearcherActor>,
    minilm: MiniLM,
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

pub fn indexer(models: &Models, searcher: &Addr<SearcherActor>) -> IndexerActor {
    IndexerActor {
        models: models.clone(),
        searcher: searcher.clone(),
        minilm: MiniLM::new(),
    }
}

impl Handler<IndexMessage> for IndexerActor {
    type Result = IndexResponse;

    fn handle(&mut self, msg: IndexMessage, _ctx: &mut SyncContext<Self>) -> Self::Result {
        let rt = Runtime::new().unwrap();
        match msg {
            IndexMessage::Index { key, text } => {
                let v = embed(&mut self.minilm, &[&text]);

                rt.block_on(self.searcher.send(crate::searcher::SearchMessage::Index {
                    key,
                    vector: v[0].clone(),
                })).unwrap();

                return IndexResponse::IndexResult;
            }
        }
    }
}

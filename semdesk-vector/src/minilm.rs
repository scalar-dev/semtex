use std::time::Instant;

use candle_core::{DType, Error, Module, Result, Tensor};
use candle_nn::VarBuilder;
use candle_transformers::models::bert::BertModel;
use hf_hub::{api::sync::Api, Repo, RepoType};
use rust_bert::pipelines::sentence_embeddings::{
    SentenceEmbeddingsBuilder, SentenceEmbeddingsModel, SentenceEmbeddingsModelType,
};
use tokenizers::Tokenizer;

use crate::{
    embedding::{EmbeddingModel, TokenizedOutput},
    util::{device, hub_load_safetensors_files},
};

pub struct MiniLM {
    model: SentenceEmbeddingsModel,
}

pub struct NullTokens {
    tokens: String,
}

impl TokenizedOutput for NullTokens {}

impl MiniLM {
    pub fn new() -> MiniLM {
        let model = SentenceEmbeddingsBuilder::remote(SentenceEmbeddingsModelType::AllMiniLmL12V2)
            .create_model()
            .unwrap();

        MiniLM { model }
    }
}

impl EmbeddingModel for MiniLM {
    type TokenizedOutput = NullTokens;

    fn tokenize(self: &Self, text: &[&str]) -> Vec<NullTokens> {
        text.iter()
            .map(|s| NullTokens {
                tokens: s.to_string(),
            })
            .collect::<Vec<_>>()
    }

    fn embed(self: &Self, tokenized_output: Vec<NullTokens>) -> Vec<Vec<f32>> {
        self.model
            .encode(
                &tokenized_output
                    .iter()
                    .map(|s| s.tokens.to_owned())
                    .collect::<Vec<_>>(),
            )
            .unwrap()
    }
}

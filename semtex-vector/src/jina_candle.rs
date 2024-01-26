use std::time::Instant;

use candle_core::{DType, Error, Module, Result, Tensor};
use candle_nn::VarBuilder;
use candle_transformers::models::jina_bert::BertModel as JinaModel;
use hf_hub::{api::sync::Api, Repo, RepoType};
use tokenizers::Tokenizer;

use crate::{
    embedding::EmbeddingModel,
    util::{device, hub_load_safetensors_files, token_ids, BertTokens},
};

#[derive(Clone)]
pub struct JinaCandle {
    model: JinaModel,
    tokenizer: Tokenizer,
}

impl JinaCandle {
    pub fn new() -> Result<JinaCandle> {
        let device = device(true)?;
        let model_id = "jinaai/jina-embeddings-v2-small-en".to_string();

        let api = Api::new().map_err(Error::wrap)?;
        let repo = api.repo(Repo::new(model_id, RepoType::Model));

        let tokenizer = Api::new()
            .unwrap()
            .repo(Repo::new(
                "sentence-transformers/all-MiniLM-L6-v2".to_string(),
                RepoType::Model,
            ))
            .get("tokenizer.json")
            .unwrap();
        let tokenizer = Tokenizer::from_file(tokenizer).unwrap();

        let filenames = hub_load_safetensors_files(&repo, &["model.safetensors"]).unwrap();
        let vb = unsafe {
            VarBuilder::from_mmaped_safetensors(&filenames, DType::F32, &device).unwrap()
        };

        let config_filename = repo.get("config.json").unwrap();
        let config = std::fs::read_to_string(config_filename)?;
        let config = serde_json::from_str(&config).unwrap();

        let model = JinaModel::new(vb, &config).unwrap();
        Ok(JinaCandle {
            model: model,
            tokenizer: tokenizer,
        })
    }
}

impl EmbeddingModel for JinaCandle {
    type TokenizedOutput = BertTokens;

    fn tokenize(self: &Self, text: &[&str]) -> Vec<BertTokens> {
        let mut tokenizer = self.tokenizer.clone();

        let tokenizer = tokenizer.with_padding(None).with_truncation(None).unwrap();

        let tokens = tokenizer.encode_batch(text.to_vec(), true).unwrap();

        return tokens
            .iter()
            .map(|encoding| BertTokens {
                encoding: encoding.clone(),
            })
            .collect::<Vec<_>>();
    }

    fn embed(self: &Self, tokenized_output: Vec<BertTokens>) -> Vec<Vec<f32>> {
        let device = device(true).unwrap();
        let embeddings = self
            .model
            .forward(&token_ids(
                &tokenized_output,
                &device, /*, &token_type_ids*/
            ))
            .unwrap();
        let (_n_sentence, n_tokens, _hidden_size) = embeddings.dims3().unwrap();

        let embeddings = (embeddings.sum(1).unwrap() / (n_tokens as f64)).unwrap();
        normalize_l2(&embeddings).unwrap();

        embeddings.to_vec2().unwrap()
    }
}

fn normalize_l2(v: &Tensor) -> candle_core::Result<Tensor> {
    v.broadcast_div(&v.sqr()?.sum_keepdim(1)?.sqrt()?)
}

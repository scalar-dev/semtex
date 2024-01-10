pub mod embedding;
pub mod jina_candle;
pub mod minilm;
mod util;

use embedding::{EmbeddingModel, TokenizedOutput};
use jina_candle::JinaCandle;

// pub fn build_model_and_tokenizer() -> candle_core::Result<LoadedModel> {
//     let device = device(true)?;
//     let model_id = "BAAI/bge-base-en-v1.5".to_string();
//     let revision = "main".to_string();

//     let api = Api::new().map_err(Error::wrap)?;
//     let repo = api.repo(Repo::with_revision(model_id, RepoType::Model, revision));

//     let config_filename = repo.get("config.json").unwrap();
//     let tokenizer = repo.get("tokenizer.json").unwrap();
//     let tokenizer = Tokenizer::from_file(tokenizer).unwrap();

//     let filenames = hub_load_safetensors(&repo, "model.safetensors.index.json").unwrap();
//     filenames
//         .iter()
//         .for_each(|fname| println!("File: {:?}", fname.to_str()));
//     let vb =
//         unsafe { VarBuilder::from_mmaped_safetensors(&filenames, DType::F32, &device).unwrap() };
//     let config = std::fs::read_to_string(config_filename)?;
//     let config = serde_json::from_str(&config).unwrap();

//     let model = BertModel::load(vb, &config).unwrap();
//     Ok(LoadedModel {
//         model: model,
//         tokenizer: tokenizer,
//     })
// }



pub fn embed<T: TokenizedOutput>(
    model: &mut dyn EmbeddingModel<TokenizedOutput = T>,
    text: &[&str],
) -> Vec<Vec<f32>> {
    let tokenized_output = model.tokenize(text);
    return model.embed(tokenized_output);
}
